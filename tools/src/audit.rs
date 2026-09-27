//! Bounds-checked ELF64 symbol inspection; no platform-specific symbol utility.
use crate::Result;
use std::{collections::BTreeSet, fs, path::Path};

struct Elf<'a>(&'a [u8]);
impl<'a> Elf<'a> {
    fn bytes(&self, offset: usize, size: usize) -> Result<&'a [u8]> {
        let end = offset.checked_add(size).ok_or("ELF offset overflow")?;
        self.0
            .get(offset..end)
            .ok_or_else(|| "Truncated ELF data".into())
    }
    fn u16(&self, offset: usize) -> Result<u16> {
        Ok(u16::from_le_bytes(self.bytes(offset, 2)?.try_into()?))
    }
    fn u32(&self, offset: usize) -> Result<u32> {
        Ok(u32::from_le_bytes(self.bytes(offset, 4)?.try_into()?))
    }
    fn size(&self, offset: usize) -> Result<usize> {
        Ok(usize::try_from(u64::from_le_bytes(
            self.bytes(offset, 8)?.try_into()?,
        ))?)
    }
}

fn symbols(data: &[u8]) -> Result<(BTreeSet<String>, BTreeSet<String>)> {
    let elf = Elf(data);
    if elf.bytes(0, 6)? != b"\x7fELF\x02\x01" || elf.u16(18)? != 62 {
        return Err("Expected little-endian x86-64 ELF64".into());
    }
    let start = elf.size(40)?;
    let stride = usize::from(elf.u16(58)?);
    let count = usize::from(elf.u16(60)?);
    if stride != 64 || count == 0 {
        return Err("Unsupported or missing ELF section table".into());
    }
    elf.bytes(start, stride.checked_mul(count).ok_or("ELF size overflow")?)?;
    let mut defined = BTreeSet::new();
    let mut undefined = BTreeSet::new();
    for i in 0..count {
        let section = start + i * stride;
        if elf.u32(section + 4)? != 2 {
            continue;
        }
        let linked = usize::try_from(elf.u32(section + 40)?)?;
        if linked >= count {
            return Err("Invalid ELF string table index".into());
        }
        let strings = start + linked * stride;
        let string_data = elf.bytes(elf.size(strings + 24)?, elf.size(strings + 32)?)?;
        let entries = elf.bytes(elf.size(section + 24)?, elf.size(section + 32)?)?;
        if elf.size(section + 56)? != 24 || entries.len() % 24 != 0 {
            return Err("Invalid ELF symbol table shape".into());
        }
        for entry in entries.as_chunks::<24>().0 {
            let entry = Elf(entry);
            let name = usize::try_from(entry.u32(0)?)?;
            if name == 0 {
                continue;
            }
            let remaining = string_data
                .get(name..)
                .ok_or("Invalid ELF symbol name offset")?;
            let end = remaining
                .iter()
                .position(|b| *b == 0)
                .ok_or("Unterminated ELF symbol name")?;
            let name = std::str::from_utf8(&remaining[..end])?.to_owned();
            if entry.u16(6)? != 0 {
                defined.insert(name);
            } else if entry.bytes(4, 1)?[0] >> 4 != 0 {
                undefined.insert(name);
            }
        }
    }
    if defined.is_empty() {
        return Err("No symbols found; audit an unstripped artifact".into());
    }
    Ok((defined, undefined))
}

fn forbidden(name: &str, kernel: bool) -> bool {
    [
        "lean_alloc",
        "lean_box",
        "lean_unbox",
        "lean_inc",
        "lean_dec",
    ]
    .iter()
    .any(|prefix| name.starts_with(prefix))
        || name.starts_with(if kernel {
            "initialize_Bastion"
        } else {
            "initialize_"
        })
        || ["malloc", "free"].contains(&name)
}

pub fn object(root: &Path, path: &Path) -> Result<()> {
    let (defined, undefined) = symbols(&fs::read(path)?)?;
    let exports: BTreeSet<_> = bastion_integrity::manifest(root)?
        .exports
        .into_iter()
        .collect();
    let missing: Vec<_> = exports.difference(&defined).collect();
    if exports.is_empty() || !undefined.is_empty() || !missing.is_empty() {
        return Err(format!(
            "Policy object audit failed: undefined={undefined:?}, missing={missing:?}"
        )
        .into());
    }
    let disallowed: Vec<_> = defined.iter().filter(|s| forbidden(s, false)).collect();
    if !disallowed.is_empty() {
        return Err(format!("Heap/initializer runtime in policy: {disallowed:?}").into());
    }
    println!(
        "PASS policy object: {} exports, zero undefined symbols, no heap runtime",
        exports.len()
    );
    Ok(())
}

pub fn kernel(path: &Path) -> Result<()> {
    let (defined, undefined) = symbols(&fs::read(path)?)?;
    let required = [
        "bastion_account",
        "bastion_next_slot",
        "bastion_authorized",
        "bastion_user_flags",
        "bastion_user_page_entry",
        "bastion_valid_user_return",
        "bastion_reservation_status",
        "bastion_supervisor_entry",
        "bastion_net_frame_len",
        "bastion_net_ipv4",
        "bastion_net_udp",
        "bastion_net_budget",
        "bastion_console_action",
        "bastion_console_budget",
        "bastion_net_tcp",
        "bastion_net_port",
        "bastion_net_payload",
    ];
    let missing: Vec<_> = required.iter().filter(|s| !defined.contains(**s)).collect();
    if !missing.is_empty() || !undefined.is_empty() {
        return Err(format!(
            "Kernel linkage audit failed: missing={missing:?}, undefined={undefined:?}"
        )
        .into());
    }
    let disallowed: Vec<_> = defined.iter().filter(|s| forbidden(s, true)).collect();
    if !disallowed.is_empty() {
        return Err(format!("Unexpected Lean heap runtime in kernel: {disallowed:?}").into());
    }
    println!(
        "PASS kernel: {} linked policy symbols",
        defined.iter().filter(|s| s.starts_with("bastion_")).count()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn malformed_elf_is_rejected_without_panicking() {
        assert!(symbols(b"not an elf").is_err());
        let mut data = vec![0; 64];
        data[..6].copy_from_slice(b"\x7fELF\x02\x01");
        data[18] = 62;
        data[40..48].copy_from_slice(&u64::MAX.to_le_bytes());
        data[58] = 64;
        data[60] = 2;
        assert!(symbols(&data).is_err());
    }
}
