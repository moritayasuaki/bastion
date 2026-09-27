//! Interactive terminal launch and bounded, real-UART console regression checks.
use crate::{Options, Result, run};
use std::{
    fs,
    io::{Read, Write},
    path::Path,
    process::{Child, Command, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
struct Guest(Child);
impl Drop for Guest {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

pub fn console(root: &Path, options: &Options, test: bool) -> Result<()> {
    let iso = options
        .path("--iso")
        .unwrap_or_else(|| root.join("dist/bastion.iso"));
    if !iso.is_file() {
        return Err("Build a release image with cargo xtask build first".into());
    }
    let code = options.path("--uefi");
    let vars = options.path("--uefi-vars");
    if code.is_some() != vars.is_some() {
        return Err("Supply both --uefi and --uefi-vars".into());
    }
    let temp = tempfile::tempdir()?;
    let mut command = Command::new("qemu-system-x86_64");
    command
        .args([
            "-machine",
            "pc",
            "-accel",
            "tcg",
            "-cpu",
            "max",
            "-m",
            if code.is_some() { "128M" } else { "64M" },
            "-smp",
            "1",
            "-cdrom",
        ])
        .arg(iso)
        .args([
            "-boot",
            "d",
            "-display",
            "none",
            "-monitor",
            "none",
            "-no-reboot",
        ]);
    if options.flag("--no-network") {
        command.args(["-net", "none"]);
    } else {
        command.args([
            "-netdev",
            "user,id=n",
            "-device",
            "virtio-net-pci,netdev=n,disable-modern=on",
        ]);
    }
    if let (Some(code), Some(vars)) = (code, vars) {
        let writable = temp.path().join("vars.fd");
        fs::copy(vars, &writable)?;
        let escape = |p: &Path| p.to_string_lossy().replace(',', ",,");
        command
            .arg("-drive")
            .arg(format!(
                "if=pflash,format=raw,unit=0,readonly=on,file={}",
                escape(&code)
            ))
            .arg("-drive")
            .arg(format!(
                "if=pflash,format=raw,unit=1,file={}",
                escape(&writable)
            ));
    }
    if !test {
        println!("Bastion serial console: type help. Exit QEMU with Ctrl-A then X.");
        command.args([
            "-chardev",
            "stdio,id=console,mux=on,signal=off",
            "-serial",
            "chardev:console",
        ]);
        return run(&mut command);
    }
    check_guest(root, &mut command, options.flag("--no-network"), false)
}
pub fn check_guest(root: &Path, command: &mut Command, no_network: bool, port: bool) -> Result<()> {
    let log_dir = root.join(if port {
        "target/bastion/port-test"
    } else {
        "target/bastion/console-test"
    });

    fs::create_dir_all(&log_dir)?;
    command
        .args(["-serial", "stdio"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(fs::File::create(log_dir.join("qemu.log"))?);
    let mut guest = Guest(command.spawn()?);
    let mut input = guest.0.stdin.take().ok_or("missing input pipe")?;
    let mut output = guest.0.stdout.take().ok_or("missing output pipe")?;
    let (send, recv) = mpsc::sync_channel(64);
    let reader = thread::spawn(move || {
        let mut buf = [0; 1024];
        let mut total = 0;
        while let Ok(n) = output.read(&mut buf) {
            if n == 0 || total + n > 1_048_576 {
                break;
            }
            total += n;
            if send.send(buf[..n].to_vec()).is_err() {
                break;
            }
        }
    });
    let mut serial = Serial {
        recv,
        saved: Vec::new(),
        log: fs::File::create(log_dir.join("serial.log"))?,
    };
    let boot = serial.until_prompt(Duration::from_secs(45))?;
    if !boot.contains(if port {
        "ALL PORT CHECKS PASSED"
    } else {
        "ALL BOOT CHECKS PASSED"
    }) || !boot.contains("PASS USER ABI: BAD POINTERS AND FORGED HANDLES DENIED")
    {
        return Err("console started before successful boot checks".into());
    }
    let help = serial.request(&mut input, b"help\r\n")?;
    require(
        &help,
        &["process status", "Ctrl-C cancels", "Userspace shell"],
    )?;
    require(
        &serial.request(&mut input, b"ps\r")?,
        &[
            "PID STATE PAGES CAPS CPU_REMAINING",
            "1 ",
            "2 ",
            "3 exited",
            "4 exited",
            "5 exited",
        ],
    )?;
    require(
        &serial.request(&mut input, b"limits\n")?,
        &[
            if port {
                "reserved=42 capacity=64"
            } else {
                "reserved=48 capacity=80"
            },
            "pages=6/6",
            "caps=0/2",
        ],
    )?;
    let network = serial.request(&mut input, b"net status\r")?;
    if no_network {
        require(&network, &["NETWORK DOWN"])?;
    } else {
        require(
            &network,
            &["NETWORK UP", "UDP echo :9000", "TCP echo :9000"],
        )?;
    }
    require(
        &serial.request(&mut input, b"uptime\r")?,
        &["UPTIME ticks="],
    )?;
    require(
        &serial.request(&mut input, b"version\n")?,
        &["Bastion 0.1.0", "policy ABI 1"],
    )?;
    require(
        &serial.request(&mut input, b"hex\x08lp\r")?,
        &["process status"],
    )?;
    require(
        &serial.request(&mut input, b"discard\x15help\r")?,
        &["process status"],
    )?;
    require(&serial.request(&mut input, b"discard\x03")?, &["^C"])?;
    require(
        &serial.request(&mut input, b"no-such-command\r")?,
        &["Unknown command"],
    )?;
    let mut oversized = b"help".to_vec();
    oversized.extend_from_slice(&[b' '; 61]);
    oversized.push(b'\r');
    let rejected = serial.request(&mut input, &oversized)?;
    require(&rejected, &["ERROR: invalid or overlong line"])?;
    if rejected.contains("process status") {
        return Err("truncated command executed".into());
    }
    require(
        &serial.request(&mut input, b"he\xfflp\r")?,
        &["ERROR: invalid or overlong line"],
    )?;
    for _ in 0..20 {
        require(&serial.request(&mut input, b"help\r")?, &["process status"])?;
    }
    let before = ticks(&serial.request(&mut input, b"uptime\r")?)?;
    thread::sleep(Duration::from_millis(50));
    let after = ticks(&serial.request(&mut input, b"uptime\r")?)?;
    if after <= before {
        return Err("timer stopped while console active".into());
    }
    guest.0.kill()?;
    guest.0.wait()?;
    drop(serial);
    let _ = reader.join();
    println!(
        "PASS serial console: commands, editing, CRLF, cancel, overflow, invalid bytes, queue reuse, timer progress"
    );
    Ok(())
}
fn require(output: &str, expected: &[&str]) -> Result<()> {
    if expected.iter().all(|s| output.contains(s)) {
        Ok(())
    } else {
        Err(format!("Unexpected console response: {output:?}; expected {expected:?}").into())
    }
}
fn ticks(output: &str) -> Result<u64> {
    let value = output
        .split("UPTIME ticks=")
        .nth(1)
        .and_then(|s| s.split_whitespace().next())
        .ok_or("missing uptime")?;
    Ok(value.parse()?)
}
struct Serial {
    recv: mpsc::Receiver<Vec<u8>>,
    saved: Vec<u8>,
    log: fs::File,
}
impl Serial {
    fn until_prompt(&mut self, timeout: Duration) -> Result<String> {
        let until = Instant::now() + timeout;
        loop {
            if self.saved.ends_with(b"bastion> ") {
                let result = String::from_utf8_lossy(&self.saved).into_owned();
                self.saved.clear();
                if result.contains("FAIL:") || result.contains("KERNEL FAULT") {
                    return Err(result.into());
                }
                return Ok(result);
            }
            if self.saved.len() > 65536 {
                return Err("console response exceeds test limit".into());
            }
            let wait = until.saturating_duration_since(Instant::now());
            if wait.is_zero() {
                return Err(
                    format!("Console timeout: {}", String::from_utf8_lossy(&self.saved)).into(),
                );
            }
            let bytes = self.recv.recv_timeout(wait).map_err(|e| {
                format!(
                    "Console stopped/timed out: {e}; {}",
                    String::from_utf8_lossy(&self.saved)
                )
            })?;
            self.log.write_all(&bytes)?;
            self.saved.extend_from_slice(&bytes);
        }
    }
    fn request(&mut self, input: &mut impl Write, bytes: &[u8]) -> Result<String> {
        // Pace input below the 16-byte UART FIFO to exercise editing deterministically.
        for byte in bytes {
            input.write_all(&[*byte])?;
            input.flush()?;
            thread::sleep(Duration::from_millis(2));
        }
        self.until_prompt(Duration::from_secs(5))
    }
}
