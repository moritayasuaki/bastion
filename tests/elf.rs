use bastion_core::{
    abi::{self, *},
    elf::{Image, Machine},
};
fn set16(b: &mut [u8], n: usize, v: u16) {
    b[n..n + 2].copy_from_slice(&v.to_le_bytes());
}
fn set32(b: &mut [u8], n: usize, v: u32) {
    b[n..n + 4].copy_from_slice(&v.to_le_bytes());
}
fn set64(b: &mut [u8], n: usize, v: u64) {
    b[n..n + 8].copy_from_slice(&v.to_le_bytes());
}
fn executable(machine: Machine) -> Vec<u8> {
    let mut b = vec![0u8; 260];
    b[..9].copy_from_slice(b"\x7fELF\x02\x01\x01\0\0");
    set16(&mut b, 16, 2);
    set16(&mut b, 18, machine as u16);
    set32(&mut b, 20, 1);
    set64(&mut b, 24, CODE);
    set64(&mut b, 32, 64);
    set16(&mut b, 52, 64);
    set16(&mut b, 54, 56);
    set16(&mut b, 56, 2);
    for (p, addr, flags, offset) in [(64, CODE, 5, 256), (120, DATA, 6, 258)] {
        set32(&mut b, p, 1);
        set32(&mut b, p + 4, flags);
        set64(&mut b, p + 8, offset);
        set64(&mut b, p + 16, addr);
        set64(&mut b, p + 32, 2);
        set64(&mut b, p + 40, 4096);
        set64(&mut b, p + 48, 1);
    }
    b[256..].copy_from_slice(&[0xaa, 0xbb, 0xcc, 0xdd]);
    b
}
#[test]
fn all_machines_load_private_regions_and_zero_bss_and_stack() {
    for m in [Machine::X86_64, Machine::Aarch64, Machine::Riscv64] {
        let b = executable(m);
        let image = Image::parse(&b, m).unwrap();
        let mut code = Box::new([0xff; REGION_SIZE]);
        let mut data = Box::new([0xff; REGION_SIZE]);
        image.load(&mut code, &mut data);
        assert_eq!(&code[..2], &[0xaa, 0xbb]);
        assert!(code[2..].iter().all(|b| *b == 0));
        assert_eq!(&data[..2], &[0xcc, 0xdd]);
        assert!(data[2..].iter().all(|b| *b == 0));
        assert_eq!(image.entry, CODE);
    }
}
#[test]
fn malformed_and_unsupported_executables_fail_before_loading() {
    let good = executable(Machine::X86_64);
    let reject = |b: &[u8]| assert!(Image::parse(b, Machine::X86_64).is_err());
    for (at, v) in [(16, 3), (18, 183), (52, 63), (54, 0), (56, 0), (56, 9)] {
        let mut b = good.clone();
        set16(&mut b, at, v);
        reject(&b);
    }
    for (at, v) in [
        (64, 2),
        (64, 3),
        (64, 7),
        (68, 7),
        (68, 6),
        (124, 5),
        (124, 4),
    ] {
        let mut b = good.clone();
        set32(&mut b, at, v);
        reject(&b);
    }
    for (at, v) in [
        (24, DATA),
        (24, CODE + 2),
        (32, u64::MAX),
        (72, u64::MAX),
        (80, 0),
        (80, CODE + REGION_SIZE as u64),
        (96, 4097),
        (104, u64::MAX),
        (112, 3),
        (112, 4096),
        (136, DATA + 0x8000),
        (152, 4097),
    ] {
        let mut b = good.clone();
        set64(&mut b, at, v);
        reject(&b);
    }
    let mut overlap = good.clone();
    set32(&mut overlap, 124, 5);
    set64(&mut overlap, 136, CODE + 4095);
    reject(&overlap);
}
#[test]
fn truncations_and_mutations_never_panic() {
    let b = executable(Machine::Riscv64);
    for n in 0..b.len() {
        let _ = Image::parse(&b[..n], Machine::Riscv64);
    }
    for n in 0..b.len() {
        for value in [0, 1, 0x7f, 0xff] {
            let mut changed = b.clone();
            changed[n] = value;
            let _ = Image::parse(&changed, Machine::Riscv64);
        }
    }
}
#[test]
fn copy_bounds_include_exact_end_but_reject_wraparound_code_and_kernel() {
    assert_eq!(abi::buffer_offset(DATA, 256, 65536), Some(0));
    assert_eq!(abi::buffer_offset(STACK_TOP, 0, 65536), Some(65536));
    for (p, n, size) in [
        (DATA, 257, 65536),
        (DATA - 1, 1, 65536),
        (CODE, 1, 65536),
        (u64::MAX, 2, 65536),
        (STACK_TOP - 1, 2, 65536),
        (DATA, 1, 0),
        (DATA, 1, u64::MAX),
    ] {
        assert_eq!(abi::buffer_offset(p, n, size), None);
    }
}
