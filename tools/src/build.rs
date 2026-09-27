use crate::{Options, Result, audit, capture, extract, run};
use std::{fs, path::Path, process::Command};

pub const LIMINE_REV: &str = "aad3edd370955449717a334f0289dee10e2c5f01";

pub fn build(root: &Path, options: &Options) -> Result<()> {
    let work = options
        .path("--work")
        .unwrap_or_else(|| root.join("target/bastion"));
    fs::create_dir_all(&work)?;
    let work = work.canonicalize()?;
    run(Command::new("lake")
        .arg("build")
        .current_dir(root.join("proof")))?;
    extract::extract(root, true)?;
    let limine = options
        .path("--limine-dir")
        .unwrap_or_else(|| work.join("limine"));
    if !limine.exists() {
        run(Command::new("git").arg("init").arg(&limine))?;
        run(Command::new("git").arg("-C").arg(&limine).args([
            "remote",
            "add",
            "origin",
            "https://github.com/limine-bootloader/limine.git",
        ]))?;
        run(Command::new("git")
            .arg("-C")
            .arg(&limine)
            .args(["fetch", "--depth", "1", "origin", LIMINE_REV]))?;
        run(Command::new("git").arg("-C").arg(&limine).args([
            "checkout",
            "--detach",
            "FETCH_HEAD",
        ]))?;
    }
    let limine = limine.canonicalize()?;
    let revision = capture(
        Command::new("git")
            .arg("-C")
            .arg(&limine)
            .args(["rev-parse", "HEAD"]),
    )?;
    if revision.trim() != LIMINE_REV {
        return Err(format!(
            "Limine revision mismatch: expected {LIMINE_REV}, got {}",
            revision.trim()
        )
        .into());
    }
    run(Command::new("make").arg("-C").arg(&limine))?;
    let test = options.flag("--test");
    let mut command = Command::new("cargo");
    command.args(["build", "--locked", "--release"]);
    if test {
        command.args(["--features", "qemu-test"]);
    }
    run(command
        .current_dir(root.join("boot"))
        .env("CARGO_TARGET_DIR", work.join("target")))?;
    let target = work.join("target/x86_64-unknown-none/release");
    let kernel = target.join("bastion-boot");
    let mut objects = 0;
    for entry in fs::read_dir(target.join("build"))? {
        let entry = entry?;
        let path = entry.path().join("out/policy.o");
        if entry
            .file_name()
            .to_string_lossy()
            .starts_with("bastion-policy-")
            && path.is_file()
        {
            audit::object(root, &path)?;
            objects += 1;
        }
    }
    if objects == 0 {
        return Err("Missing compiled Lean policy object".into());
    }
    audit::kernel(&kernel)?;
    let staging = work.join(if test { "iso-test" } else { "iso-release" });
    let boot = staging.join("boot");
    let config = boot.join("limine");
    let efi = staging.join("EFI/BOOT");
    fs::create_dir_all(&config)?;
    fs::create_dir_all(&efi)?;
    fs::copy(&kernel, boot.join("bastion-boot"))?;
    fs::copy(root.join("boot/limine.conf"), config.join("limine.conf"))?;
    for name in [
        "limine-bios.sys",
        "limine-bios-cd.bin",
        "limine-uefi-cd.bin",
    ] {
        fs::copy(limine.join(name), config.join(name))?;
    }
    fs::copy(limine.join("BOOTX64.EFI"), efi.join("BOOTX64.EFI"))?;
    fs::copy(root.join("THIRD_PARTY.md"), staging.join("THIRD_PARTY.txt"))?;
    fs::copy(root.join("LICENSE"), staging.join("LICENSE"))?;
    fs::copy(
        root.join("policy/generated/LEAN-LICENSE"),
        staging.join("LEAN-LICENSE"),
    )?;
    fs::create_dir_all(staging.join("licenses"))?;
    for entry in fs::read_dir(root.join("docs/licenses"))? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            fs::copy(
                entry.path(),
                staging.join("licenses").join(entry.file_name()),
            )?;
        }
    }
    let output = options.path("--out").unwrap_or_else(|| {
        root.join("dist").join(if test {
            "bastion-test.iso"
        } else {
            "bastion.iso"
        })
    });
    if let Some(parent) = output.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }
    run(Command::new("xorriso")
        .args([
            "-as",
            "mkisofs",
            "-b",
            "boot/limine/limine-bios-cd.bin",
            "-no-emul-boot",
            "-boot-load-size",
            "4",
            "-boot-info-table",
            "--efi-boot",
            "boot/limine/limine-uefi-cd.bin",
            "-efi-boot-part",
            "--efi-boot-image",
            "--protective-msdos-label",
        ])
        .arg(&staging)
        .arg("-o")
        .arg(&output))?;
    run(Command::new(limine.join("limine"))
        .arg("bios-install")
        .arg(&output))?;
    println!("Built: {}", output.display());
    Ok(())
}

pub fn check(root: &Path) -> Result<()> {
    let proof = root.join("proof");
    run(Command::new("lake").arg("build").current_dir(&proof))?;
    extract::extract(root, true)?;
    for (source, saved) in [
        ("Vectors.lean", "policy-vectors.csv"),
        ("RuntimeVectors.lean", "runtime-vectors.csv"),
    ] {
        let vectors = capture(
            Command::new("lake")
                .args(["env", "lean", "--run", source])
                .current_dir(&proof),
        )?;
        if vectors.as_bytes() != fs::read(root.join("tests/data").join(saved))? {
            return Err(format!(
                "Lean vectors differ; regenerate tests/data/{saved} and review the change."
            )
            .into());
        }
    }
    run(Command::new("cargo")
        .args(["fmt", "--all", "--check"])
        .current_dir(root))?;
    run(Command::new("cargo")
        .args(["test", "--locked", "--workspace"])
        .current_dir(root))?;
    run(Command::new("cargo")
        .args([
            "clippy",
            "--locked",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ])
        .current_dir(root))?;
    run(Command::new("cargo")
        .args(["fmt", "--check"])
        .current_dir(root.join("boot")))?;
    run(Command::new("cargo")
        .args([
            "clippy",
            "--locked",
            "--release",
            "--features",
            "qemu-test",
            "--",
            "-D",
            "warnings",
        ])
        .current_dir(root.join("boot")))?;
    println!("All proof, conformance, unit-test, and static checks passed.");
    Ok(())
}
