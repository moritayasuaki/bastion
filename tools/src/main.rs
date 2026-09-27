//! Host tools for the Lean/C/Rust kernel. Invoke with cargo xtask.
mod audit;
mod build;
mod extract;
mod smoke;

use bastion_integrity::Result;
use std::{
    collections::BTreeMap,
    env,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

fn run(command: &mut Command) -> Result<()> {
    let status = command.status()?;
    if !status.success() {
        return Err(format!("Command failed ({status}): {command:?}").into());
    }
    Ok(())
}

fn capture(command: &mut Command) -> Result<String> {
    let output = command.stderr(Stdio::inherit()).output()?;
    if !output.status.success() {
        return Err(format!("Command failed ({}): {command:?}", output.status).into());
    }
    Ok(String::from_utf8(output.stdout)?)
}

struct Options(BTreeMap<String, Option<String>>);
impl Options {
    fn parse(args: impl Iterator<Item = String>, flags: &[&str], values: &[&str]) -> Result<Self> {
        let mut parsed = BTreeMap::new();
        let mut args = args.peekable();
        while let Some(name) = args.next() {
            if parsed.contains_key(&name) {
                return Err(format!("Duplicate option: {name}").into());
            }
            let value = if flags.contains(&name.as_str()) {
                None
            } else if values.contains(&name.as_str()) {
                if args.peek().is_none_or(|value| value.starts_with("--")) {
                    return Err(format!("Missing value for {name}").into());
                }
                args.next()
            } else {
                return Err(format!("Unknown option: {name}").into());
            };
            parsed.insert(name, value);
        }
        Ok(Self(parsed))
    }
    fn flag(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }
    fn value(&self, name: &str) -> Option<&str> {
        self.0.get(name)?.as_deref()
    }
    fn path(&self, name: &str) -> Option<PathBuf> {
        self.value(name).map(PathBuf::from)
    }
}

fn main() {
    if let Err(error) = dispatch() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn dispatch() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut args = env::args().skip(1);
    let action = args.next().unwrap_or_else(|| "help".into());
    match action.as_str() {
        "extract" => {
            let options = Options::parse(args, &["--check", "--verify"], &[])?;
            if options.flag("--check") && options.flag("--verify") {
                return Err("Choose either --check or --verify".into());
            }
            if options.flag("--verify") {
                bastion_integrity::verify(root)?;
                println!("Generated policy matches its Lean source and artifact hashes.");
            } else {
                extract::extract(root, options.flag("--check"))?;
            }
        }
        "check" => {
            Options::parse(args, &[], &[])?;
            build::check(root)?;
        }
        "network-test" => {
            run(Command::new("cargo")
                .args(["run", "--locked", "--package", "bastion-lab", "--"])
                .args(args)
                .current_dir(root))?;
        }
        "build" => build::build(
            root,
            &Options::parse(args, &["--test"], &["--work", "--limine-dir", "--out"])?,
        )?,
        "smoke" => smoke::smoke(
            root,
            &Options::parse(
                args,
                &[],
                &[
                    "--iso",
                    "--machine",
                    "--memory",
                    "--uefi",
                    "--uefi-vars",
                    "--log",
                ],
            )?,
        )?,
        "audit" => {
            let options = Options::parse(args, &[], &["--object", "--kernel"])?;
            if options.0.is_empty() {
                return Err("Provide --object and/or --kernel".into());
            }
            if let Some(path) = options.path("--object") {
                audit::object(root, &path)?;
            }
            if let Some(path) = options.path("--kernel") {
                audit::kernel(&path)?;
            }
        }
        "help" | "--help" | "-h" => println!(
            "cargo xtask extract [--check | --verify]\n\
             cargo xtask check\n\
             cargo xtask build [--test] [--work DIR] [--limine-dir DIR] [--out ISO]\n\
             cargo xtask smoke [--iso ISO] [--machine pc|q35] [--memory 128M]\n\
             [--uefi CODE --uefi-vars VARS] [--log FILE]\n\
             cargo xtask audit [--object ELF] [--kernel ELF]\n\
             cargo xtask network-test [--iso ISO] [--uefi CODE --uefi-vars VARS] [--hold-seconds N]"
        ),
        _ => return Err(format!("Unknown command: {action}; use cargo xtask help").into()),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn integrity_rejects_changed_source_artifacts_and_incomplete_manifest() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let temporary = tempfile::tempdir().unwrap();
        let copy = temporary.path();
        fs::create_dir_all(copy.join("proof/Bastion")).unwrap();
        fs::create_dir_all(copy.join("policy/generated")).unwrap();
        let source = "proof/Bastion/Runtime.lean";
        fs::copy(root.join(source), copy.join(source)).unwrap();
        for name in bastion_integrity::FILES
            .into_iter()
            .chain(["manifest.json"])
        {
            let relative = Path::new("policy/generated").join(name);
            fs::copy(root.join(&relative), copy.join(relative)).unwrap();
        }
        bastion_integrity::verify(copy).unwrap();
        for relative in std::iter::once(PathBuf::from(source)).chain(
            bastion_integrity::FILES
                .iter()
                .map(|name| Path::new("policy/generated").join(name)),
        ) {
            let path = copy.join(relative);
            let original = fs::read(&path).unwrap();
            let mut changed = original.clone();
            changed.push(b'\n');
            fs::write(&path, changed).unwrap();
            assert!(bastion_integrity::verify(copy).is_err());
            fs::write(&path, original).unwrap();
        }
        let mut saved = bastion_integrity::manifest(copy).unwrap();
        saved.files_sha256.remove("policy.c");
        fs::write(
            copy.join("policy/generated/manifest.json"),
            serde_json::to_vec(&saved).unwrap(),
        )
        .unwrap();
        assert!(bastion_integrity::verify(copy).is_err());
    }
}
