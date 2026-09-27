use crate::{Result, run};
use std::{
    path::{Path, PathBuf},
    process::Command,
};
pub fn user(root: &Path, target: &str) -> Result<PathBuf> {
    let work = root.join("target/bastion/user");
    run(Command::new("cargo")
        .args(["build", "--locked", "--release", "--target", target])
        .current_dir(root.join("user"))
        .env("CARGO_TARGET_DIR", &work))?;
    Ok(work.join(target).join("release/bastion-init"))
}
pub fn dispatch(root: &Path, options: &crate::Options, action: &str) -> Result<()> {
    let arch = options
        .value("--arch")
        .ok_or("Use --arch aarch64 or --arch riscv64")?;
    let target = match arch {
        "aarch64" => "aarch64-unknown-none-softfloat",
        "riscv64" => "riscv64imac-unknown-none-elf",
        _ => return Err("Supported ports: aarch64, riscv64".into()),
    };
    let kernel = root.join("dist").join(format!("bastion-{arch}.elf"));
    if action == "build-port" {
        let init = user(root, target)?;
        let work = root.join("target/bastion/ports");
        run(Command::new("cargo")
            .args(["build", "--locked", "--release", "--target", target])
            .current_dir(root.join("ports"))
            .env("BASTION_INIT", init)
            .env("CARGO_TARGET_DIR", &work))?;
        let built = work.join(target).join("release");
        for entry in std::fs::read_dir(built.join("build"))? {
            let object = entry?.path().join("out/policy.o");
            if object.is_file() {
                crate::audit::object(root, &object)?;
            }
        }
        crate::audit::native_image(
            &built.join("bastion-virt"),
            if arch == "aarch64" {
                "bastion_arm_page"
            } else {
                "bastion_riscv_page"
            },
        )?;
        std::fs::create_dir_all(root.join("dist"))?;

        std::fs::copy(work.join(target).join("release/bastion-virt"), &kernel)?;
        println!("Built {}", kernel.display());
        return Ok(());
    }
    if !kernel.is_file() {
        return Err("Run build-port for this architecture first".into());
    }
    let mut qemu = Command::new(format!("qemu-system-{arch}"));
    qemu.args([
        "-machine",
        if arch == "aarch64" {
            "virt,gic-version=2,virtualization=off"
        } else {
            "virt"
        },
        "-cpu",
        if arch == "aarch64" {
            "cortex-a53"
        } else {
            "rv64"
        },
        "-m",
        "128M",
        "-smp",
        "1",
        "-display",
        "none",
        "-monitor",
        "none",
        "-no-reboot",
        "-net",
        "none",
        "-kernel",
    ])
    .arg(kernel);
    if action == "test-port" {
        crate::console::check_guest(root, &mut qemu, true, true)
    } else {
        qemu.args([
            "-chardev",
            "stdio,id=console,mux=on,signal=off",
            "-serial",
            "chardev:console",
        ]);
        run(&mut qemu)
    }
}

pub fn check(root: &Path) -> Result<()> {
    for directory in ["user", "ports"] {
        run(Command::new("cargo")
            .args(["fmt", "--check"])
            .current_dir(root.join(directory)))?;
    }
    for target in [
        "x86_64-unknown-none",
        "aarch64-unknown-none-softfloat",
        "riscv64imac-unknown-none-elf",
    ] {
        let init = user(root, target)?;
        run(Command::new("cargo")
            .args([
                "clippy",
                "--locked",
                "--release",
                "--target",
                target,
                "--",
                "-D",
                "warnings",
            ])
            .current_dir(root.join("user"))
            .env("CARGO_TARGET_DIR", root.join("target/bastion/user")))?;
        if target != "x86_64-unknown-none" {
            run(Command::new("cargo")
                .args([
                    "clippy",
                    "--locked",
                    "--release",
                    "--target",
                    target,
                    "--",
                    "-D",
                    "warnings",
                ])
                .current_dir(root.join("ports"))
                .env("BASTION_INIT", init)
                .env("CARGO_TARGET_DIR", root.join("target/bastion/ports")))?;
        }
    }
    Ok(())
}
