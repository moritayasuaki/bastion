use crate::{Options, Result};
use std::{
    fs,
    path::Path,
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

const REQUIRED: [&str; 11] = [
    "LEAN-GENERATED C POLICY ACTIVE",
    "PASS MEMORY QUOTA",
    "PASS PRIVATE MEMORY",
    "PASS FORGED CAPABILITY",
    "PASS FAULT CONTAINED: KERNEL MEMORY",
    "PASS FAULT CONTAINED: CODE WRITE",
    "PASS FAULT CONTAINED: STACK EXECUTE",
    "PASS CPU BUDGET",
    "PASS BUSY PROCESS PREEMPTED",
    "PASS SURVIVORS RUN AFTER FAULTS",
    "ALL BOOT CHECKS PASSED",
];

struct ChildGuard(Child);
impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn valid_output(status: Option<i32>, output: &str) -> bool {
    status == Some(33)
        && REQUIRED.iter().all(|marker| output.contains(marker))
        && !output.contains("FAIL")
}

pub fn smoke(root: &Path, options: &Options) -> Result<()> {
    let machine = options.value("--machine").unwrap_or("pc");
    if !["pc", "q35"].contains(&machine) {
        return Err("--machine must be pc or q35".into());
    }
    let memory = options.value("--memory").unwrap_or("128M");
    let iso = options
        .path("--iso")
        .unwrap_or_else(|| root.join("dist/bastion-test.iso"));
    let code = options.path("--uefi");
    let vars = options.path("--uefi-vars");
    if code.is_some() != vars.is_some() {
        return Err("--uefi and --uefi-vars must be supplied together".into());
    }
    let temporary = tempfile::tempdir()?;
    let log_path = temporary.path().join("serial.log");
    let log = fs::File::create(&log_path)?;
    let mut command = Command::new("qemu-system-x86_64");
    command
        .args([
            "-machine", machine, "-accel", "tcg", "-cpu", "max", "-m", memory, "-smp", "1",
        ])
        .arg("-cdrom")
        .arg(iso)
        .args([
            "-boot",
            "d",
            "-display",
            "none",
            "-serial",
            "stdio",
            "-monitor",
            "none",
            "-no-reboot",
            "-net",
            "none",
            "-device",
            "isa-debug-exit,iobase=0xf4,iosize=0x04",
        ]);
    if let (Some(code), Some(vars)) = (&code, vars) {
        let variables = temporary.path().join("vars.fd");
        fs::copy(vars, &variables)?;
        // QEMU drive options use comma as a separator.
        let escape = |path: &Path| path.to_string_lossy().replace(',', ",,");
        command.arg("-drive").arg(format!(
            "if=pflash,format=raw,unit=0,readonly=on,file={}",
            escape(code)
        ));
        command.arg("-drive").arg(format!(
            "if=pflash,format=raw,unit=1,file={}",
            escape(&variables)
        ));
    }
    command
        .stdin(Stdio::null())
        .stdout(log.try_clone()?)
        .stderr(log);
    let mut child = ChildGuard(command.spawn()?);
    let started = Instant::now();
    let status = loop {
        if let Some(status) = child.0.try_wait()? {
            break Some(status);
        }
        if started.elapsed() >= Duration::from_secs(45) {
            child.0.kill()?;
            child.0.wait()?;
            break None;
        }
        thread::sleep(Duration::from_millis(100));
    };
    let output = String::from_utf8_lossy(&fs::read(&log_path)?).into_owned();
    print!("{output}");
    if let Some(path) = options.path("--log") {
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, &output)?;
    }
    let status = status.ok_or("FAIL: boot timed out")?.code();
    if !valid_output(status, &output) {
        return Err(format!("FAIL: incomplete boot checks, QEMU status {status:?}").into());
    }
    println!(
        "PASS: {machine}, {memory}, {}",
        if code.is_some() { "UEFI" } else { "BIOS" }
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot_success_requires_status_and_every_assertion() {
        let output = REQUIRED.join("\n");
        assert!(valid_output(Some(33), &output));
        assert!(!valid_output(Some(0), &output));
        assert!(!valid_output(None, &output));
        assert!(!valid_output(Some(33), &(output.clone() + "\nFAIL")));
        for missing in REQUIRED {
            assert!(!valid_output(Some(33), &output.replace(missing, "")));
        }
    }
}
