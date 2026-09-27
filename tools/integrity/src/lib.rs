//! Shared host-side provenance checks for the build script and developer tools.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub const FILES: [&str; 6] = [
    "Runtime.full.c",
    "policy.c",
    "scalars.h",
    "policy.h",
    "bindings.rs",
    "LEAN-LICENSE",
];

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub lean_version: String,
    pub source_sha256: String,
    pub full_c_sha256: String,
    pub freestanding_c_sha256: String,
    pub lean_header_sha256: String,
    pub exports: Vec<String>,
    pub reachable_functions: Vec<String>,
    pub scalar_primitives: Vec<String>,
    pub removed: String,
    pub runtime: String,
    pub files_sha256: BTreeMap<String, String>,
}

pub fn digest(data: &[u8]) -> String {
    format!("{:x}", Sha256::digest(data))
}

pub fn manifest(root: &Path) -> Result<Manifest> {
    Ok(serde_json::from_slice(&fs::read(
        root.join("policy/generated/manifest.json"),
    )?)?)
}

pub fn verify(root: &Path) -> Result<()> {
    let saved = manifest(root)?;
    if saved.lean_version != "4.32.1" {
        return Err("Unsupported generated Lean version".into());
    }
    if digest(&fs::read(root.join("proof/Bastion/Runtime.lean"))?) != saved.source_sha256 {
        return Err("Lean source changed. Run cargo xtask extract, then cargo xtask check.".into());
    }
    // Require the complete, fixed artifact set; a shortened manifest must not skip checks.
    if saved.files_sha256.len() != FILES.len()
        || FILES
            .iter()
            .any(|name| !saved.files_sha256.contains_key(*name))
    {
        return Err("Generated manifest has an unexpected artifact set".into());
    }
    for name in FILES {
        if digest(&fs::read(root.join("policy/generated").join(name))?) != saved.files_sha256[name]
        {
            return Err(
                format!("Generated artifact changed: {name}. Regenerate from Lean.").into(),
            );
        }
    }
    if saved.full_c_sha256 != saved.files_sha256["Runtime.full.c"]
        || saved.freestanding_c_sha256 != saved.files_sha256["policy.c"]
    {
        return Err("Inconsistent generated C hashes".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saved_policy_integrity() {
        verify(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .as_path(),
        )
        .unwrap();
    }

    #[test]
    fn standard_digest_vector() {
        assert_eq!(
            digest(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
