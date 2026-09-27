//! Real TCP/UDP exchanges against a natively booted QEMU guest.
use bastion_core::net::MAX_PAYLOAD;
use std::{
    error::Error,
    fs,
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream, UdpSocket},
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
        Err("interrupted; network guest stopped".into())
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
fn main() {
    if let Err(e) = execute() {
        eprintln!("FAIL: {e}");
        std::process::exit(1);
    }
}
fn execute() -> Result<()> {
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
    let (mut firmware, mut vars) = (None, None);
    let mut hold = 0u64;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let value = args.next().ok_or("option needs value")?;
        match arg.as_str() {
            "--iso" => iso = PathBuf::from(value),
            "--uefi" => firmware = Some(value),
            "--uefi-vars" => vars = Some(value),
            "--hold-seconds" => hold = value.parse()?,
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
    let logs = root.join("target/bastion/network-test");
    fs::create_dir_all(&logs)?;
    // QEMU has to bind the forwards itself; a competing bind fails the test.
    let udp_port = UdpSocket::bind("127.0.0.1:0")?.local_addr()?.port();
    let tcp_port = TcpListener::bind("127.0.0.1:0")?.local_addr()?.port();
    let log = logs.join("serial.log");
    // A previous run must not satisfy readiness before this QEMU opens its log.
    fs::write(&log, [])?;
    let mut command = Command::new("qemu-system-x86_64");
    command.args(["-machine", "pc", "-accel", "tcg", "-cpu", "max", "-m",
        if firmware.is_some() { "128M" } else { "64M" }, "-smp", "1", "-cdrom"])
        .arg(iso).args(["-boot", "d", "-display", "none", "-monitor", "none", "-no-reboot", "-serial"])
        .arg(format!("file:{}", log.display())).arg("-netdev")
        .arg(format!("user,id=n,hostfwd=udp:127.0.0.1:{udp_port}-:9000,hostfwd=tcp:127.0.0.1:{tcp_port}-:9000"))
        .args(["-device", "virtio-net-pci,netdev=n,disable-modern=on"])
        .arg("-object").arg(format!("filter-dump,id=capture,netdev=n,file={}", logs.join("network.pcap").display()))
        .stdin(Stdio::null()).stdout(Stdio::null()).stderr(fs::File::create(logs.join("qemu.log"))?);
    if let (Some(code), Some(template)) = (&firmware, &vars) {
        let writable = temporary.path().join("vars.fd");
        fs::copy(template, &writable)?;
        command
            .arg("-drive")
            .arg(format!(
                "if=pflash,format=raw,unit=0,readonly=on,file={}",
                code.replace(',', ",,")
            ))
            .arg("-drive")
            .arg(format!(
                "if=pflash,format=raw,unit=1,file={}",
                writable.to_string_lossy().replace(',', ",,")
            ));
    }
    let mut guest = Guest(command.spawn()?);
    let deadline = Instant::now() + Duration::from_secs(40);
    loop {
        cancellation()?;
        if let Some(status) = guest.0.try_wait()? {
            return Err(format!("QEMU exited {status}; see {}", logs.display()).into());
        }
        let serial = fs::read_to_string(&log).unwrap_or_default();
        if serial.contains("FAIL") || serial.contains("PANIC") {
            return Err(serial.into());
        }
        if serial.contains("ALL BOOT CHECKS PASSED") && serial.contains("TCP / UDP READY") {
            break;
        }
        if Instant::now() > deadline {
            return Err(format!("boot timed out: {serial}").into());
        }
        thread::sleep(Duration::from_millis(50));
    }
    let udp = UdpSocket::bind("127.0.0.1:0")?;
    udp.connect(("127.0.0.1", udp_port))?;
    udp.set_read_timeout(Some(Duration::from_secs(2)))?;
    let mut buffer = [0; MAX_PAYLOAD + 1];
    for i in 0..525 {
        cancellation()?;
        let n = if i < 5 { [0, 1, 17, 512, 1200][i] } else { 32 };
        let request = vec![(i % 251) as u8; n];
        udp.send(&request)
            .map_err(|e| format!("UDP exchange {i}, send: {e}"))?;
        let got = udp
            .recv(&mut buffer)
            .map_err(|e| format!("UDP exchange {i}, receive: {e}"))?;
        if buffer[..got] != request {
            return Err("UDP response mismatch".into());
        }
    }
    udp.send(&[0; MAX_PAYLOAD + 1])?;
    match udp.recv(&mut buffer) {
        Err(e)
            if matches!(
                e.kind(),
                std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
            ) => {}
        other => return Err(format!("oversized UDP should be silent: {other:?}").into()),
    }
    println!("PASS UDP: empty/max datagrams, oversize rejection, 525 exchanges");
    for session in 0..3 {
        cancellation()?;
        let mut tcp =
            TcpStream::connect_timeout(&([127, 0, 0, 1], tcp_port).into(), Duration::from_secs(5))
                .map_err(|e| format!("TCP session {session}, connect: {e}"))?;
        tcp.set_read_timeout(Some(Duration::from_secs(5)))?;
        tcp.set_write_timeout(Some(Duration::from_secs(5)))?;
        tcp.set_nodelay(true)?;
        for n in [1, 17, 1200, 8192, 65536] {
            let request: Vec<_> = (0..n).map(|i| ((i + session) % 251) as u8).collect();
            // Chunked exchange avoids requiring the service to buffer the whole stream.
            for chunk in request.chunks(997) {
                tcp.write_all(chunk)?;
                let mut response = vec![0; chunk.len()];
                tcp.read_exact(&mut response)?;
                if response != chunk {
                    return Err("TCP response mismatch".into());
                }
            }
        }
        tcp.write_all(b"final bytes before FIN")?;
        tcp.shutdown(Shutdown::Write)?;
        let mut tail = Vec::new();
        tcp.read_to_end(&mut tail)?;
        if tail != b"final bytes before FIN" {
            return Err("TCP half-close lost data".into());
        }
        thread::sleep(Duration::from_millis(100));
    }
    println!("PASS TCP: connect, multi-buffer streams, half-close/EOF, three connections");
    let until = Instant::now() + Duration::from_secs(hold);
    while Instant::now() < until {
        cancellation()?;
        thread::sleep(Duration::from_millis(100));
    }
    println!("PASS network test; logs: {}", logs.display());
    Ok(())
}
