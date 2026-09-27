use std::{env, path::PathBuf, process::Command};

fn run(command: &mut Command) {
    let status = command.status().expect("C toolchain could not be started");
    assert!(status.success(), "C toolchain failed: {command:?}");
}

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    let cc = env::var("BASTION_CC").unwrap_or_else(|_| "clang".into());
    let ar = env::var("BASTION_AR").unwrap_or_else(|_| {
        let prefix = Command::new("lean")
            .arg("--print-prefix")
            .current_dir(root.parent().unwrap().join("proof"))
            .output()
            .expect("Lean is needed to locate its portable llvm-ar");
        assert!(prefix.status.success(), "Could not locate Lean's llvm-ar");
        PathBuf::from(String::from_utf8(prefix.stdout).unwrap().trim())
            .join("bin/llvm-ar")
            .to_string_lossy()
            .into_owned()
    });
    let object = out.join("policy.o");
    let mut compiler = Command::new(cc);
    compiler.args([
        "-std=c11",
        "-O2",
        "-ffreestanding",
        "-fno-builtin",
        "-fno-stack-protector",
        "-ffunction-sections",
        "-fdata-sections",
        "-Wall",
        "-Wextra",
        "-Werror",
        "-c",
    ]);
    if target == "x86_64-unknown-none" {
        compiler.args([
            "--target=x86_64-unknown-none-elf",
            "-mno-red-zone",
            "-mno-sse",
            "-mno-sse2",
            "-mno-mmx",
            "-mcmodel=kernel",
            "-fno-pic",
        ]);
    } else {
        compiler.arg(format!("--target={target}"));
    }
    run(compiler
        .arg(root.join("vendor/psiv/psiv.c"))
        .arg("-o")
        .arg(&object));
    run(Command::new(ar)
        .arg(if target.contains("apple") {
            "--format=darwin"
        } else {
            "--format=gnu"
        })
        .arg("crs")
        .arg(out.join("libbastion_psiv.a"))
        .arg(object));
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=bastion_psiv");
    println!("cargo:rerun-if-changed=vendor/psiv/psiv.c");
    println!("cargo:rerun-if-changed=vendor/psiv/psiv.h");
    println!("cargo:rerun-if-env-changed=BASTION_CC");
    println!("cargo:rerun-if-env-changed=BASTION_AR");
}
