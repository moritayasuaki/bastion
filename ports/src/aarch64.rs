use super::*;
use bastion_core::elf::Machine;
use core::arch::global_asm;
global_asm!(include_str!("aarch64.S"));
pub const NAME: &str = "AARCH64";
pub const MACHINE: Machine = Machine::Aarch64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Frame {
    x: [u64; 31],
    sp: u64,
    pc: u64,
    status: u64,
    vector: u64,
    reserved: u64,
}
impl Frame {
    pub const fn zero() -> Self {
        Self {
            x: [0; 31],
            sp: 0,
            pc: 0,
            status: 0,
            vector: 0,
            reserved: 0,
        }
    }
}
unsafe extern "C" {
    pub fn restore(f: *const Frame) -> !;
    fn vectors();
    fn idle();
}
unsafe fn read(at: usize) -> u32 {
    core::ptr::read_volatile(at as *const u32)
}
unsafe fn write(at: usize, value: u32) {
    core::ptr::write_volatile(at as *mut u32, value);
}
pub fn writable() -> bool {
    unsafe { read(0x09000018) & 32 == 0 }
}
pub fn put(b: u8) {
    unsafe {
        for _ in 0..10000 {
            if read(0x09000018) & 32 == 0 {
                write(0x09000000, b as u32);
                return;
            }
        }
    }
}
pub fn get() -> Option<Result<u8, ()>> {
    unsafe {
        if read(0x09000018) & 16 != 0 {
            None
        } else {
            let b = read(0x09000000);
            if b & 0xf00 != 0 {
                write(0x09000004, 0);
                Some(Err(()))
            } else {
                Some(Ok(b as u8))
            }
        }
    }
}
pub fn clock() -> u64 {
    let n;
    unsafe {
        asm!("mrs {}, cntpct_el0",out(reg)n,options(nomem,nostack));
    }
    n
}
pub fn frequency() -> u64 {
    let n;
    unsafe {
        asm!("mrs {}, cntfrq_el0",out(reg)n,options(nomem,nostack));
    }
    n
}
pub unsafe fn init() {
    let level: u64;
    asm!("mrs {}, CurrentEL",out(reg)level);
    assert_eq!(level, 4, "EL1 required");
    asm!("msr vbar_el1, {}",in(reg)vectors as *const () as u64);
    write(0x0900002c, (3 << 5) | (1 << 4));
    write(0x09000030, 0x301);
    // SIMD/FP is disabled; all programs use the soft-float target.
    asm!("msr cpacr_el1, xzr");
    write(0x08000000, 1); // GICv2 distributor
    write(0x08000100, 1 << 30);
    write(0x08010004, 0xff);
    write(0x08010000, 1);
}
pub unsafe fn start_timer() {
    let delta = frequency() / 1000;
    asm!("msr cntp_tval_el0, {}","msr cntp_ctl_el0, {}",in(reg)delta,in(reg)1u64);
}
pub unsafe fn timer(frame: &Frame) -> bool {
    if frame.vector % 4 != 1 {
        return false;
    }
    let id = read(0x0801000c);
    if id & 1023 == 30 {
        start_timer();
        write(0x08010010, id);
        true
    } else {
        if id & 1023 != 1023 {
            write(0x08010010, id);
        }
        false
    }
}
const AF: u64 = 1 << 10;
const UXN: u64 = 1 << 54;
const PXN: u64 = 1 << 53;
pub unsafe fn map(s: &mut Space, pages: usize) {
    let t = &mut s.tables;
    t[0].0[0] = (&raw const t[1]) as u64 | 3;
    t[1].0[0] = (&raw const t[2]) as u64 | 3;
    t[1].0[1] = 0x40000000 | 1 | AF | (3 << 8) | (1 << 2) | UXN;
    t[2].0[64] = 0x08000000 | 1 | AF | UXN | PXN;
    t[2].0[72] = 0x09000000 | 1 | AF | UXN | PXN;
    t[2].0[2] = (&raw const t[3]) as u64 | 3;
    for n in 0..pages {
        t[3].0[n] = decisions::arm_page((&raw const s.code) as u64 + (n * 4096) as u64, 1);
        t[3].0[256 + n] = decisions::arm_page((&raw const s.data) as u64 + (n * 4096) as u64, 2);
    }
}
pub unsafe fn enable(s: &Space) {
    // 4 KiB, 48-bit lower VA, normal WB memory in MAIR slot 1, device slot 0.
    let tcr = 16u64 | (1 << 8) | (1 << 10) | (3 << 12) | (1 << 23) | (2 << 32);
    asm!("msr mair_el1, {}","msr tcr_el1, {}",in(reg)0xff00u64,in(reg)tcr);
    switch(s);
    let mut control: u64;
    asm!("mrs {}, sctlr_el1",out(reg)control);
    control |= 1; // MMU; keep caches disabled for this first platform profile.
    asm!("msr sctlr_el1, {}","isb",in(reg)control);
}
pub unsafe fn switch(s: &Space) {
    asm!("dsb sy","msr ttbr0_el1, {}","isb","tlbi vmalle1","dsb sy","isb",in(reg)(&raw const s.tables[0]) as u64);
}
pub fn user(pc: u64, sp: u64, kernel: u64) -> Frame {
    let mut f = Frame {
        pc,
        sp,
        ..Frame::zero()
    };
    f.x[12] = kernel;
    f.x[13] = CODE;
    f.x[14] = DATA;
    f
}
pub fn idle_frame() -> Frame {
    Frame {
        pc: idle as *const () as u64,
        status: 5,
        ..Frame::zero()
    }
}
pub fn is_user(f: &Frame) -> bool {
    f.status & 15 == 0
}
fn esr() -> u64 {
    let n;
    unsafe {
        asm!("mrs {}, esr_el1",out(reg)n,options(nomem,nostack));
    }
    n
}
pub fn is_syscall() -> bool {
    esr() >> 26 == 0x15
}
pub fn syscall(f: &Frame) -> (u64, [u64; 4]) {
    (f.x[8], [f.x[0], f.x[1], f.x[2], f.x[3]])
}
pub fn advance(_: &mut Frame) {} // SVC stores the following instruction in ELR.
pub fn result(f: &mut Frame, value: u64) {
    f.x[0] = value;
}
pub fn probe(i: usize) -> u32 {
    match i {
        1 => 0x14000000,
        2 => 0xf9400180,
        3 => 0xf90001a0,
        _ => 0xd61f01c0,
    }
}
pub fn expected_fault(i: usize, kernel: u64) -> bool {
    let address: u64;
    unsafe {
        asm!("mrs {}, far_el1",out(reg)address,options(nomem,nostack));
    }
    address
        == match i {
            2 => kernel,
            3 => CODE,
            _ => DATA,
        }
        && esr() >> 26 == if i == 4 { 0x20 } else { 0x24 }
}
