//! Allocation-free ELF64 static executable validation. No mappings are changed
//! until the entire image is valid. Only the explicit Bastion v1 profile is accepted.
use crate::{abi, decisions};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Machine {
    X86_64 = 62,
    Aarch64 = 183,
    Riscv64 = 243,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Header,
    Machine,
    Bounds,
    Segment,
    Overlap,
    Entry,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Segment {
    pub address: u64,
    pub offset: usize,
    pub file_size: usize,
    pub memory_size: usize,
    pub writable: bool,
}
pub struct Image<'a> {
    bytes: &'a [u8],
    pub entry: u64,
    segments: [Segment; 8],
    count: usize,
}
fn word<const N: usize>(b: &[u8], at: usize) -> Result<[u8; N], Error> {
    b.get(at..at.checked_add(N).ok_or(Error::Bounds)?)
        .ok_or(Error::Bounds)?
        .try_into()
        .map_err(|_| Error::Bounds)
}
fn u16le(b: &[u8], at: usize) -> Result<u16, Error> {
    Ok(u16::from_le_bytes(word(b, at)?))
}
fn u32le(b: &[u8], at: usize) -> Result<u32, Error> {
    Ok(u32::from_le_bytes(word(b, at)?))
}
fn u64le(b: &[u8], at: usize) -> Result<u64, Error> {
    Ok(u64::from_le_bytes(word(b, at)?))
}
impl<'a> Image<'a> {
    pub fn parse(bytes: &'a [u8], machine: Machine) -> Result<Self, Error> {
        if bytes.get(..9) != Some(b"\x7fELF\x02\x01\x01\x00\x00")
            || u16le(bytes, 16)? != 2
            || u32le(bytes, 20)? != 1
            || u16le(bytes, 52)? != 64
            || u16le(bytes, 54)? != 56
        {
            return Err(Error::Header);
        }
        if u16le(bytes, 18)? != machine as u16 {
            return Err(Error::Machine);
        }
        let flags = u32le(bytes, 48)?;
        if flags != 0 && !(machine == Machine::Riscv64 && flags == 1) {
            return Err(Error::Header);
        }
        let start = usize::try_from(u64le(bytes, 32)?).map_err(|_| Error::Bounds)?;
        let count = usize::from(u16le(bytes, 56)?);
        if !(1..=8).contains(&count)
            || start < 64
            || start
                .checked_add(count * 56)
                .is_none_or(|end| end > bytes.len())
        {
            return Err(Error::Bounds);
        }
        let mut image = Self {
            bytes,
            entry: u64le(bytes, 24)?,
            segments: [Segment::default(); 8],
            count: 0,
        };
        for n in 0..count {
            let p = start + n * 56;
            match u32le(bytes, p)? {
                0 | 4 | 0x70000003 => continue, // NULL, NOTE, RISC-V attributes
                0x6474e551 if u32le(bytes, p + 4)? & 1 == 0 => continue, // non-executable GNU stack
                1 => {}
                _ => return Err(Error::Segment), // includes INTERP, DYNAMIC and TLS
            }
            let flags = u32le(bytes, p + 4)?;
            let offset = u64le(bytes, p + 8)?;
            let address = u64le(bytes, p + 16)?;
            let filesz = u64le(bytes, p + 32)?;
            let memsz = u64le(bytes, p + 40)?;
            let align = u64le(bytes, p + 48)?;
            if decisions::elf_segment(address, filesz, memsz, flags.into()) == 0
                || offset
                    .checked_add(filesz)
                    .is_none_or(|end| end > bytes.len() as u64)
                || (align > 1 && (!align.is_power_of_two() || address % align != offset % align))
            {
                return Err(Error::Segment);
            }
            // Reject page aliases as well as byte overlaps, preserving W^X.
            for previous in image.segments() {
                let a = previous.address & !4095;
                let end = (previous.address + previous.memory_size as u64 + 4095) & !4095;
                if address & !4095 < end && a < (address + memsz + 4095) & !4095 {
                    return Err(Error::Overlap);
                }
            }
            image.segments[image.count] = Segment {
                address,
                offset: offset as usize,
                file_size: filesz as usize,
                memory_size: memsz as usize,
                writable: flags == 6,
            };
            image.count += 1;
        }
        if !image.segments().iter().any(|s| {
            !s.writable && image.entry >= s.address && image.entry < s.address + s.file_size as u64
        }) {
            return Err(Error::Entry);
        }
        Ok(image)
    }
    pub fn segments(&self) -> &[Segment] {
        &self.segments[..self.count]
    }
    /// Both regions are private, unpublished zeroed pages supplied by the loader.
    pub fn load(&self, code: &mut [u8; abi::REGION_SIZE], data: &mut [u8; abi::REGION_SIZE]) {
        code.fill(0);
        data.fill(0);
        for s in self.segments() {
            let (region, base) = if s.writable {
                (&mut *data, abi::DATA)
            } else {
                (&mut *code, abi::CODE)
            };
            let start = (s.address - base) as usize;
            region[start..start + s.file_size]
                .copy_from_slice(&self.bytes[s.offset..s.offset + s.file_size]);
        }
    }
}
