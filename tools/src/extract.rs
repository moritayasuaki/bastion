//! Retain the scalar closure from the real Lean C backend, with bodies unchanged.
use crate::{Result, capture};
use bastion_integrity::{Manifest, digest};
use regex::Regex;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::Command,
};

const PRIMITIVES: [&str; 12] = [
    "lean_uint64_mul",
    "lean_uint64_shift_right",
    "lean_uint64_add",
    "lean_uint64_sub",
    "lean_uint64_div",
    "lean_uint64_mod",
    "lean_uint64_land",
    "lean_uint64_lor",
    "lean_uint64_shift_left",
    "lean_uint64_dec_eq",
    "lean_uint64_dec_lt",
    "lean_uint64_dec_le",
];

#[derive(Debug)]
struct Function {
    result: String,
    name: String,
    args: String,
    body: String,
}

fn functions(source: &str) -> Result<Vec<Function>> {
    let pattern = Regex::new(
        r"(?m)^(?:LEAN_EXPORT |static inline )?(uint64_t|uint8_t) (\w+)\(([^;\n]*)\)\s*\{",
    )?;
    let mut names = BTreeSet::new();
    let mut found = Vec::new();
    for matched in pattern.captures_iter(source) {
        let whole = matched.get(0).unwrap();
        let mut depth = 1;
        let mut end = whole.end();
        while depth != 0 && end < source.len() {
            match source.as_bytes()[end] {
                b'{' => depth += 1,
                b'}' => depth -= 1,
                _ => {}
            }
            end += 1;
        }
        if depth != 0 {
            return Err("Unbalanced generated function".into());
        }
        if !names.insert(matched[2].to_owned()) {
            return Err(format!("Duplicate generated function: {}", &matched[2]).into());
        }
        found.push(Function {
            result: matched[1].to_owned(),
            name: matched[2].to_owned(),
            args: matched[3].to_owned(),
            body: source[whole.start()..end].to_owned(),
        });
    }
    Ok(found)
}

fn calls(body: &str) -> BTreeSet<String> {
    Regex::new(r"\b([A-Za-z_]\w*)\s*\(")
        .unwrap()
        .captures_iter(body)
        .map(|m| m[1].to_owned())
        .collect()
}

fn select<'a>(
    defs: &'a [Function],
    exports: &[String],
) -> Result<(Vec<&'a Function>, BTreeSet<String>)> {
    let mut pending = exports.to_vec();
    let mut reachable = BTreeSet::new();
    let mut primitives = BTreeSet::new();
    let abi = Regex::new(r"^uint(?:64|8)_t \w+$")?;
    while let Some(name) = pending.pop() {
        if reachable.contains(&name) {
            continue;
        }
        let function = defs
            .iter()
            .find(|f| f.name == name)
            .ok_or_else(|| format!("Non-scalar or missing generated definition: {name}"))?;
        if function.args.split(',').any(|a| !abi.is_match(a.trim())) {
            return Err(format!("Unsupported ABI in {name}: {}", function.args).into());
        }
        for target in calls(&function.body) {
            if target == name || ["if", "while", "switch", "sizeof"].contains(&target.as_str()) {
                continue;
            }
            if defs.iter().any(|f| f.name == target) {
                pending.push(target);
            } else if PRIMITIVES.contains(&target.as_str()) {
                primitives.insert(target);
            } else {
                return Err(format!("Unsupported call from {name}: {target}").into());
            }
        }
        if ["lean_object", "___closed", "___boxed"]
            .iter()
            .any(|s| function.body.contains(s))
        {
            return Err(format!("Boxed/global runtime dependency in {name}").into());
        }
        reachable.insert(name);
    }
    Ok((
        defs.iter()
            .filter(|f| reachable.contains(&f.name))
            .collect(),
        primitives,
    ))
}

fn rust_type(c: &str) -> &str {
    match c {
        "uint64_t" => "u64",
        "uint8_t" => "u8",
        _ => unreachable!("ABI checked"),
    }
}

pub fn generate(root: &Path) -> Result<BTreeMap<String, Vec<u8>>> {
    let proof = root.join("proof");
    let version = capture(
        Command::new("lean")
            .arg("--short-version")
            .current_dir(&proof),
    )?;
    let version = version.trim();
    if version != "4.32.1" {
        return Err(format!(
            "Expected Lean 4.32.1; got {version}. Review extraction before upgrading."
        )
        .into());
    }
    let prefix = capture(
        Command::new("lean")
            .arg("--print-prefix")
            .current_dir(&proof),
    )?;
    let prefix = Path::new(prefix.trim());
    let temporary = tempfile::tempdir()?;
    let output = temporary.path().join("Runtime.full.c");
    crate::run(
        Command::new("lean")
            .args(["-DwarningAsError=true", "-c"])
            .arg(&output)
            .arg("Bastion/Runtime.lean")
            .current_dir(&proof),
    )?;
    let raw = fs::read_to_string(output)?;
    let defs = functions(&raw)?;
    let lean_source = fs::read(root.join("proof/Bastion/Runtime.lean"))?;
    let exports: Vec<String> = Regex::new(r"@\[export (bastion_\w+)\]")?
        .captures_iter(std::str::from_utf8(&lean_source)?)
        .map(|m| m[1].to_owned())
        .collect();
    if exports.is_empty() || exports.iter().collect::<BTreeSet<_>>().len() != exports.len() {
        return Err("Empty or duplicate Lean exports".into());
    }
    let (selected, primitives) = select(&defs, &exports)?;
    let declaration = |f: &Function| format!("{} {}({});", f.result, f.name, f.args);
    let c = format!(
        "/* GENERATED by cargo xtask extract from Lean C output. DO NOT EDIT. */\n\
         #include \"scalars.h\"\n#define LEAN_EXPORT\n\
         #pragma clang diagnostic ignored \"-Wunused-label\"\n\
         #pragma clang diagnostic ignored \"-Wunused-parameter\"\n{}\n\n{}\n",
        selected
            .iter()
            .map(|f| declaration(f))
            .collect::<Vec<_>>()
            .join("\n"),
        selected
            .iter()
            .map(|f| f.body.as_str())
            .collect::<Vec<_>>()
            .join("\n\n")
    );
    let runtime_header = fs::read(prefix.join("include/lean/lean.h"))?;
    let helpers = functions(std::str::from_utf8(&runtime_header)?)?;
    let mut scalar_functions = Vec::new();
    for name in &primitives {
        let function = helpers
            .iter()
            .find(|f| &f.name == name)
            .ok_or_else(|| format!("Missing runtime primitive: {name}"))?;
        if calls(&function.body).iter().any(|call| call != name) {
            return Err(format!("Non-leaf scalar primitive: {name}").into());
        }
        scalar_functions.push(function.body.as_str());
    }
    let scalar_header = format!(
        "/* Extracted verbatim from Lean 4.32.1 include/lean/lean.h.\n\
         {}* Copyright (c) 2019 Microsoft Corporation. All rights reserved.\n\
         {}* Apache-2.0; see LEAN-LICENSE. Original author: Leonardo de Moura. */\n\
         #pragma once\n#include <stdint.h>\n\
         _Static_assert(sizeof(uint64_t) == 8, \"64-bit scalar ABI required\");\n{}\n",
        " ",
        " ",
        scalar_functions.join("\n")
    );
    let mut bindings = vec![
        "// GENERATED by cargo xtask extract. Integer-only, total, allocation-free ABI.".to_owned(),
        "mod ffi {".into(),
        "    unsafe extern \"C\" {".into(),
    ];
    let mut wrappers = Vec::new();
    let mut declarations = Vec::new();
    for name in &exports {
        let f = defs.iter().find(|f| &f.name == name).unwrap();
        let types: Vec<_> = f
            .args
            .split(',')
            .map(|a| rust_type(a.split_whitespace().next().unwrap()))
            .collect();
        let params = types
            .iter()
            .enumerate()
            .map(|(i, t)| format!("a{i}: {t}"))
            .collect::<Vec<_>>()
            .join(", ");
        let values = (0..types.len())
            .map(|i| format!("a{i}"))
            .collect::<Vec<_>>()
            .join(", ");
        let short = name.strip_prefix("bastion_").unwrap();
        let result = rust_type(&f.result);
        bindings.push(format!("        pub fn {name}({params}) -> {result};"));
        wrappers.push(format!(
            "#[inline]\npub fn {short}({params}) -> {result} {{\n\
             {}// SAFETY: the generated ABI accepts all values of these integer types.\n\
             {}unsafe {{ ffi::{name}({values}) }}\n}}",
            "    ", "    "
        ));
        declarations.push(declaration(f));
    }
    bindings.extend(["    }".into(), "}".into(), wrappers.join("\n")]);
    let header = format!(
        "/* GENERATED scalar C API. */\n#pragma once\n#include <stdint.h>\n{}\n",
        declarations.join("\n")
    );
    let mut artifacts: BTreeMap<String, Vec<u8>> = BTreeMap::from([
        ("Runtime.full.c".into(), raw.as_bytes().to_vec()),
        ("policy.c".into(), c.as_bytes().to_vec()),
        ("scalars.h".into(), scalar_header.into_bytes()),
        ("policy.h".into(), header.into_bytes()),
        (
            "bindings.rs".into(),
            format!("{}\n", bindings.join("\n")).into_bytes(),
        ),
        ("LEAN-LICENSE".into(), fs::read(prefix.join("LICENSE"))?),
    ]);
    let manifest = Manifest {
        lean_version: version.into(),
        source_sha256: digest(&lean_source),
        full_c_sha256: digest(raw.as_bytes()),
        freestanding_c_sha256: digest(c.as_bytes()),
        lean_header_sha256: digest(&runtime_header),
        exports,
        reachable_functions: selected.iter().map(|f| f.name.clone()).collect(),
        scalar_primitives: primitives.into_iter().collect(),
        removed:
            "Unreachable boxed wrappers and module initializers; retained bodies are unchanged."
                .into(),
        runtime:
            "Scalar primitives only; no allocation, reference counting, or module initialization."
                .into(),
        files_sha256: artifacts
            .iter()
            .map(|(name, data)| (name.clone(), digest(data)))
            .collect(),
    };
    artifacts.insert(
        "manifest.json".into(),
        format!("{}\n", serde_json::to_string_pretty(&manifest)?).into_bytes(),
    );
    Ok(artifacts)
}

pub fn extract(root: &Path, check: bool) -> Result<()> {
    let artifacts = generate(root)?;
    let dest = root.join("policy/generated");
    if check {
        let changed: Vec<_> = artifacts
            .iter()
            .filter(|(name, data)| fs::read(dest.join(name)).ok().as_ref() != Some(data))
            .map(|(name, _)| name.as_str())
            .collect();
        if !changed.is_empty() {
            return Err(format!("Stale generated artifacts: {}", changed.join(", ")).into());
        }
        println!("Lean-generated C, scalar primitives, bindings, and provenance are current.");
    } else {
        fs::create_dir_all(&dest)?;
        for (name, data) in artifacts {
            fs::write(dest.join(name), data)?;
        }
        println!("Extracted scalar Lean exports into {}", dest.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserve_transitive_scalar_bodies() {
        let source = "uint64_t helper(uint64_t x) { return lean_uint64_add(x, 1); }\n\
                      LEAN_EXPORT uint64_t bastion_test(uint64_t x) { return helper(x); }\n\
                      uint64_t unused(uint64_t x) { return forbidden(x); }\n";
        let defs = functions(source).unwrap();
        let (selected, primitives) = select(&defs, &["bastion_test".into()]).unwrap();
        assert_eq!(selected.len(), 2);
        assert!(selected.iter().all(|f| source.contains(&f.body)));
        assert_eq!(primitives, BTreeSet::from(["lean_uint64_add".to_owned()]));
    }

    #[test]
    fn reject_unsafe_subset_dependencies() {
        for source in [
            "uint64_t bastion_test(uint64_t *x) { return 0; }",
            "uint64_t bastion_test(uint64_t x) { return lean_alloc(x); }",
            "uint64_t bastion_test(uint64_t x) { return x___closed; }",
            "uint64_t bastion_test(uint64_t x) { return helper(x); }",
        ] {
            assert!(select(&functions(source).unwrap(), &["bastion_test".into()]).is_err());
        }
        assert!(functions("uint64_t f(uint64_t x) {").is_err());
        assert!(
            functions("uint64_t f(uint64_t x) { return x; }\nuint64_t f(uint64_t x) { return x; }")
                .is_err()
        );
    }
}
