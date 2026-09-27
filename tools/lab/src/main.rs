//! Local eight-VM Octave/PSIV laboratory. No production keys or public listeners.
use bastion_core::{
    net::MAX_PAYLOAD,
    relay::{self, BODY, Config, PACKET},
};
use bastion_crypto::{Session, derive, nonce};
use std::{
    error::Error,
    fs,
    io::Read,
    net::{SocketAddr, UdpSocket},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::{Duration, Instant},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
static STOP: AtomicBool = AtomicBool::new(false);
extern "C" fn interrupted(_: libc::c_int) {
    STOP.store(true, Ordering::Relaxed);
}
fn cancellation() -> Result<()> {
    if STOP.load(Ordering::Relaxed) {
        Err("interrupted; local relay guests stopped".into())
    } else {
        Ok(())
    }
}
struct Guest(Child);
impl Drop for Guest {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
fn run(c: &mut Command) -> Result<()> {
    let output = c.output()?;
    if !output.status.success() {
        return Err(format!("{c:?}: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    Ok(())
}
fn random<const N: usize>() -> Result<[u8; N]> {
    let mut b = [0; N];
    fs::File::open("/dev/urandom")?.read_exact(&mut b)?;
    Ok(b)
}
fn port() -> Result<u16> {
    Ok(UdpSocket::bind("127.0.0.1:0")?.local_addr()?.port())
}
fn transact(socket: &UdpSocket, addr: SocketAddr, packet: &[u8]) -> Result<Vec<u8>> {
    cancellation()?;
    socket.send_to(packet, addr)?;
    let mut out = [0; MAX_PAYLOAD + 1];
    let (n, source) = socket.recv_from(&mut out)?;
    if source != addr {
        return Err("unexpected response endpoint".into());
    }
    Ok(out[..n].to_vec())
}
fn expect_silence(socket: &UdpSocket, addr: SocketAddr, packet: &[u8]) -> Result<()> {
    cancellation()?;
    socket.send_to(packet, addr)?;
    let mut out = [0; MAX_PAYLOAD + 1];
    match socket.recv_from(&mut out) {
        Err(e)
            if matches!(
                e.kind(),
                std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
            ) =>
        {
            Ok(())
        }
        Err(e) => Err(e.into()),
        Ok(_) => Err("rejected request produced a response".into()),
    }
}
#[allow(clippy::too_many_arguments)]
fn exchange(
    config: &Config,
    socket: &UdpSocket,
    port: u16,
    role: u8,
    op: u8,
    sid: [u8; 16],
    sequence: &mut u64,
    body: &[u8],
) -> Result<Vec<u8>> {
    let mut packet = [0; PACKET];
    let mut plaintext = [0; BODY];
    for _ in 0..8 {
        cancellation()?;
        *sequence += 1;
        let n = relay::encode(config, role, op, sid, *sequence, body, &mut packet)
            .ok_or("encode failed")?;
        match transact(socket, ([127, 0, 0, 1], port).into(), &packet[..n]) {
            Ok(response) => {
                let n = relay::decode_response(
                    config,
                    role,
                    if op == 1 { 129 } else { 130 },
                    sid,
                    *sequence,
                    &response,
                    &mut plaintext,
                )
                .ok_or("invalid relay response")?;
                return Ok(plaintext[..n].to_vec());
            }
            Err(_) => continue,
        }
    }
    Err(format!("relay {} timed out", config.id).into())
}
fn main() {
    if let Err(e) = execute() {
        eprintln!("FAIL: {e}");
        std::process::exit(1);
    }
}
fn execute() -> Result<()> {
    // Handlers only set a lock-free flag. Normal stack unwinding owns child cleanup.
    unsafe {
        if libc::signal(libc::SIGINT, interrupted as *const () as libc::sighandler_t)
            == libc::SIG_ERR
            || libc::signal(
                libc::SIGTERM,
                interrupted as *const () as libc::sighandler_t,
            ) == libc::SIG_ERR
        {
            return Err("could not install cancellation handlers".into());
        }
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let mut iso = root.join("dist/bastion.iso");
    let mut firmware = None;
    let mut vars = None;
    let mut hold = 0u64;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let v = args.next().ok_or("option needs value")?;
        match arg.as_str() {
            "--iso" => iso = PathBuf::from(v),
            "--uefi" => firmware = Some(v),
            "--uefi-vars" => vars = Some(v),
            "--hold-seconds" => hold = v.parse()?,
            _ => return Err(format!("unknown option {arg}").into()),
        }
    }
    if firmware.is_some() != vars.is_some() {
        return Err("supply both UEFI firmware and variables".into());
    }
    if !iso.is_file() {
        return Err("Build the normal release ISO with cargo xtask build first".into());
    }
    let temporary = tempfile::tempdir()?;
    let logs = root.join("target/bastion/relay-lab");
    fs::create_dir_all(&logs)?;
    let epoch = random::<16>()?;
    let configs: Vec<_> = (1..=8)
        .map(|id| {
            Ok(Config {
                id,
                epoch,
                alice: random()?,
                bob: random()?,
            })
        })
        .collect::<Result<_>>()?;
    let mut guests = Vec::new();
    let mut ports = Vec::new();
    let mut echoes = Vec::new();
    for c in &configs {
        cancellation()?;
        let dir = temporary.path().join(c.id.to_string());
        fs::create_dir(&dir)?;
        let config = dir.join("relay.bin");
        fs::write(&config, c.encode())?;
        let limine = dir.join("limine.conf");
        fs::write(
            &limine,
            format!(
                "{}\n    module_path: boot():/boot/relay.bin\n",
                fs::read_to_string(root.join("boot/limine.conf"))?
            ),
        )?;
        let image = dir.join("relay.iso");
        run(Command::new("xorriso")
            .arg("-indev")
            .arg(&iso)
            .arg("-outdev")
            .arg(&image)
            .args(["-boot_image", "any", "replay", "-map"])
            .arg(config)
            .arg("/boot/relay.bin")
            .arg("-map")
            .arg(limine)
            .arg("/boot/limine/limine.conf"))?;
        let p = loop {
            let candidate = port()?;
            if !ports.contains(&candidate) && !echoes.contains(&candidate) {
                break candidate;
            }
        };
        let mut e = port()?;
        while e == p || ports.contains(&e) || echoes.contains(&e) {
            e = port()?;
        }
        ports.push(p);
        echoes.push(e);
        let log = logs.join(format!("relay-{}.log", c.id));
        let stderr = fs::File::create(logs.join(format!("qemu-{}.log", c.id)))?;
        let mut cmd = Command::new("qemu-system-x86_64");
        let memory = if firmware.is_some() { "128M" } else { "64M" };
        cmd.args([
            "-machine", "pc", "-accel", "tcg", "-cpu", "max", "-m", memory, "-smp", "1", "-cdrom",
        ])
        .arg(image)
        .args([
            "-boot",
            "d",
            "-display",
            "none",
            "-monitor",
            "none",
            "-no-reboot",
            "-serial",
        ])
        .arg(format!("file:{}", log.display()))
        .arg("-netdev")
        .arg(format!(
            "user,id=n,hostfwd=udp:127.0.0.1:{p}-:9001,hostfwd=udp:127.0.0.1:{e}-:9000"
        ))
        .args(["-device", "virtio-net-pci,netdev=n,disable-modern=on"])
        .arg("-object")
        .arg(format!(
            "filter-dump,id=capture,netdev=n,file={}",
            logs.join(format!("relay-{}.pcap", c.id)).display()
        ))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(stderr);
        if let (Some(code), Some(template)) = (&firmware, &vars) {
            let writable = dir.join("vars.fd");
            fs::copy(template, &writable)?;
            let code = code.replace(',', ",,");
            let writable = writable.to_string_lossy().replace(',', ",,");
            cmd.arg("-drive")
                .arg(format!(
                    "if=pflash,format=raw,unit=0,readonly=on,file={code}"
                ))
                .arg("-drive")
                .arg(format!("if=pflash,format=raw,unit=1,file={}", writable));
        }
        guests.push(Guest(cmd.spawn()?));
    }
    let deadline = Instant::now() + Duration::from_secs(40);
    loop {
        cancellation()?;
        let mut ready = true;
        for (i, guest) in guests.iter_mut().enumerate() {
            if let Some(status) = guest.0.try_wait()? {
                return Err(format!(
                    "QEMU relay {} exited: {status}; see {}",
                    i + 1,
                    logs.display()
                )
                .into());
            }
            let log =
                fs::read_to_string(logs.join(format!("relay-{}.log", i + 1))).unwrap_or_default();
            if log.contains("FAIL") {
                return Err(format!("relay {} kernel failure: {log}", i + 1).into());
            }
            ready &= log.contains("ALL BOOT CHECKS PASSED")
                && log.contains(&format!("OCTAVE RELAY {}", i + 1));
        }
        if ready {
            break;
        }
        if Instant::now() > deadline {
            return Err(format!("relay boot timeout; see {}", logs.display()).into());
        }
        thread::sleep(Duration::from_millis(100));
    }
    println!("PASS eight Bastion guests: isolated processes and authenticated relay modules ready");
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.set_read_timeout(Some(Duration::from_millis(500)))?;
    for &e in &echoes {
        for size in [0, 1, 17, 512, 1200] {
            let payload = vec![size as u8; size];
            let reply = transact(&socket, ([127, 0, 0, 1], e).into(), &payload)
                .map_err(|err| format!("echo port {e}, payload {size}: {err}"))?;
            if reply != payload {
                return Err("UDP echo mismatch".into());
            }
        }
    }
    println!("PASS UDP echo: 0, 1, 17, 512, and 1200 bytes on every relay");
    // Reuse every DMA descriptor and cross the maximum queue's ring boundary twice.
    for sequence in 0..520u16 {
        let payload = sequence.to_le_bytes();
        let reply = transact(&socket, ([127, 0, 0, 1], echoes[0]).into(), &payload)?;
        if reply != payload {
            return Err("virtio ring wrap echo mismatch".into());
        }
    }
    println!("PASS virtio receive/transmit ring reuse across 520 consecutive exchanges");
    let mut alice_seq = [0; 8];
    let mut bob_seq = [0; 8];
    let mut last_session = [0; 16];
    for faults in [false, true] {
        if faults {
            guests[7].0.kill()?;
            guests[7].0.wait()?;
        }
        let sid = random::<16>()?;
        last_session = sid;
        let t = relay::transcript(epoch, sid, random()?);
        let root_key = random::<32>()?;
        let mut coins = [[0u16; 3]; 32];
        for row in &mut coins {
            for value in row {
                loop {
                    let n = u16::from_le_bytes(random()?);
                    if n < 65535 {
                        *value = n % 257;
                        break;
                    }
                }
            }
        }
        // Alice creates immutable confirmation evidence before Bob reconstructs any candidate.
        let proof = relay::confirmation(&root_key, &t, false);
        let mut received: [Option<[u16; 32]>; 8] = [None; 8];
        for i in 0..8 {
            if faults && i == 7 {
                continue;
            }
            let mut body = [0; BODY];
            body[..96].copy_from_slice(&t);
            body[96..160].copy_from_slice(
                &relay::share(&root_key, &coins, (i + 1) as u8).ok_or("sharing failed")?,
            );
            body[160..].copy_from_slice(&proof);
            if faults && i < 3 {
                for j in 0..32 {
                    let offset = 96 + 2 * j;
                    let v = u16::from_le_bytes(body[offset..offset + 2].try_into()?);
                    body[offset..offset + 2].copy_from_slice(&((v + 1) % 257).to_le_bytes());
                }
            }
            exchange(
                &configs[i],
                &socket,
                ports[i],
                1,
                1,
                sid,
                &mut alice_seq[i],
                &body,
            )?;
            let reply = exchange(
                &configs[i],
                &socket,
                ports[i],
                2,
                2,
                sid,
                &mut bob_seq[i],
                &[],
            )?;
            if reply.len() != BODY || reply[..96] != t || reply[160..] != proof {
                return Err("transcript/evidence changed".into());
            }
            received[i] = relay::decode_share(&reply[96..160]);
        }
        let recovered = relay::recover(&received, |candidate| {
            relay::confirms(candidate, &t, &proof, false)
        })
        .ok_or("no unique confirmed root")?;
        let reverse = relay::confirmation(&recovered, &t, true);
        let bob = UdpSocket::bind("127.0.0.1:0")?;
        bob.set_read_timeout(Some(Duration::from_secs(1)))?;
        bob.send_to(&reverse, socket.local_addr()?)?;
        let mut confirmation = [0; 17];
        let (n, source) = socket.recv_from(&mut confirmation)?;
        if n != 16
            || source != bob.local_addr()?
            || !relay::confirms(&root_key, &t, confirmation[..16].try_into()?, true)
        {
            return Err("mutual confirmation failed".into());
        }
        // End-to-end traffic between two real host UDP endpoints, after mutual confirmation.
        let message = b"Octave over Bastion: authenticated UDP traffic";
        let mut sealed = [0; 128];
        let n = Session::new(&derive(&root_key, &t, b"trafficAB"))
            .unwrap()
            .seal(&nonce(1), &t, message, &mut sealed)
            .unwrap();
        socket.send_to(&sealed[..n], bob.local_addr()?)?;
        let mut wire = [0; 128];
        let (n, source) = bob.recv_from(&mut wire)?;
        if source != socket.local_addr()? {
            return Err("unexpected Alice endpoint".into());
        }
        let mut plain = [0; 128];
        let n = Session::new(&derive(&recovered, &t, b"trafficAB"))
            .unwrap()
            .open(&nonce(1), &t, &wire[..n], &mut plain)
            .map_err(|_| "traffic auth failed")?;
        if &plain[..n] != message {
            return Err("traffic mismatch".into());
        }
        let n = Session::new(&derive(&recovered, &t, b"trafficBA"))
            .unwrap()
            .seal(&nonce(1), &t, message, &mut sealed)
            .unwrap();
        bob.send_to(&sealed[..n], socket.local_addr()?)?;
        let (n, source) = socket.recv_from(&mut wire)?;
        if source != bob.local_addr()? {
            return Err("unexpected Bob endpoint".into());
        }
        let n = Session::new(&derive(&root_key, &t, b"trafficBA"))
            .unwrap()
            .open(&nonce(1), &t, &wire[..n], &mut plain)
            .map_err(|_| "reverse traffic auth failed")?;
        if &plain[..n] != message {
            return Err("reverse traffic mismatch".into());
        }
        println!(
            "PASS Octave 4-of-8 / {} / unique confirmation / mutual PSIV confirmation / UDP traffic",
            if faults {
                "3 altered shares + 1 unavailable"
            } else {
                "all 8 honest"
            }
        );
    }
    // Use a known live slot, so an unrelated missing-session check cannot mask failures.
    let mut packet = [0; PACKET];
    let addr = ([127, 0, 0, 1], ports[0]).into();
    let n = relay::encode(
        &configs[0],
        2,
        2,
        last_session,
        bob_seq[0],
        &[],
        &mut packet,
    )
    .unwrap();
    expect_silence(&socket, addr, &packet[..n])?;
    let next = bob_seq[0] + 1;
    let n = relay::encode(&configs[0], 2, 2, last_session, next, &[], &mut packet).unwrap();
    packet[n - 1] ^= 1;
    expect_silence(&socket, addr, &packet[..n])?;
    packet[n - 1] ^= 1;
    let response = transact(&socket, addr, &packet[..n])?;
    let mut plaintext = [0; BODY];
    if relay::decode_response(
        &configs[0],
        2,
        130,
        last_session,
        next,
        &response,
        &mut plaintext,
    ) != Some(BODY)
    {
        return Err("forgery advanced the replay counter or valid retry failed".into());
    }
    println!("PASS replay and authentication rejection; no plaintext or key material logged");
    fs::write(
        logs.join("result.txt"),
        "PASS: 8 Bastion guests; UDP payload boundaries; Octave honest and 3-corrupt/1-offline scenarios; mutual PSIV confirmation; encrypted UDP traffic; replay/forgery rejection.\n",
    )?;
    if hold > 0 {
        println!("Keeping the local relay VMs running for {hold} seconds.");
        let until = Instant::now()
            .checked_add(Duration::from_secs(hold))
            .ok_or("hold too long")?;
        while Instant::now() < until {
            cancellation()?;
            thread::sleep(Duration::from_millis(100));
        }
    }
    Ok(())
}
