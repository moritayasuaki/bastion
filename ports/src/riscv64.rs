use super::*;
use bastion_core::elf::Machine;
use core::arch::global_asm;
global_asm!(include_str!("riscv64.S"));
pub const NAME: &str = "RISCV64";
pub const MACHINE: Machine = Machine::Riscv64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Frame {
    x: [u64; 32],
    pc: u64,
    status: u64,
}
impl Frame {
    pub const fn zero() -> Self {
        Self {
            x: [0; 32],
            pc: 0,
            status: 0,
        }
    }
}
unsafe extern "C" {
    pub fn restore(f: *const Frame) -> !;
    fn exception();
    fn idle();
}
unsafe fn read(at: usize) -> u8 {
    core::ptr::read_volatile(at as *const u8)
}
unsafe fn write(at: usize, b: u8) {
    core::ptr::write_volatile(at as *mut u8, b);
}
static mut RX_ERROR: bool = false;
unsafe fn line_status() -> u8 {
    let status = read(0x10000005);
    if status & 0x1e != 0 {
        RX_ERROR = true;
    }
    status
}
pub fn writable() -> bool {
    unsafe { line_status() & 32 != 0 }
}
pub fn put(b: u8) {
    unsafe {
        for _ in 0..10000 {
            if line_status() & 32 != 0 {
                write(0x10000000, b);
                return;
            }
        }
    }
}
pub fn get() -> Option<Result<u8, ()>> {
    unsafe {
        let s = line_status();
        if RX_ERROR {
            RX_ERROR = false;
            if s & 1 != 0 {
                read(0x10000000);
            }
            Some(Err(()))
        } else if s & 1 != 0 {
            Some(Ok(read(0x10000000)))
        } else {
            None
        }
    }
}
pub fn clock() -> u64 {
    let n;
    unsafe {
        asm!("rdtime {}",out(reg)n,options(nomem,nostack));
    }
    n
}
pub fn frequency() -> u64 {
    10_000_000
} // QEMU virt's documented timebase; fixed machine profile.
pub unsafe fn init() {
    asm!("csrw stvec, {}","csrw sie, {}",in(reg)exception as *const () as u64,in(reg)32u64);
    write(0x10000001, 0);
    write(0x10000002, 7);
}
pub unsafe fn start_timer() {
    let error: i64;
    asm!("ecall",inlateout("a0") clock()+frequency()/1000=>error,inlateout("a1") 0u64=>_,in("a6")0u64,in("a7")0x54494d45u64,options(nostack));
    assert_eq!(error, 0, "SBI TIME extension required");
}
fn cause() -> u64 {
    let n;
    unsafe {
        asm!("csrr {}, scause",out(reg)n,options(nomem,nostack));
    }
    n
}
pub unsafe fn timer(_: &Frame) -> bool {
    if cause() == (1 << 63) | 5 {
        start_timer();
        true
    } else {
        false
    }
}
fn pointer(t: &Table) -> u64 {
    (t as *const Table as u64 >> 12) << 10 | 1
}
pub unsafe fn map(s: &mut Space, pages: usize) {
    s.tables[0].0[0] = pointer(&s.tables[1]);
    s.tables[0].0[2] = (0x80000000 >> 12) << 10 | 0xcf; // supervisor RAM, R/W/X/A/D
    s.tables[1].0[128] = (0x10000000 >> 12) << 10 | 0xc7; // UART, supervisor RW/NX
    s.tables[1].0[2] = pointer(&s.tables[2]);
    for n in 0..pages {
        s.tables[2].0[n] = decisions::riscv_page((&raw const s.code) as u64 + (n * 4096) as u64, 1);
        s.tables[2].0[256 + n] =
            decisions::riscv_page((&raw const s.data) as u64 + (n * 4096) as u64, 2);
    }
}
pub unsafe fn switch(s: &Space) {
    let satp = (8u64 << 60) | ((&raw const s.tables[0]) as u64 >> 12);
    asm!("csrw satp, {}","sfence.vma zero,zero","fence.i",in(reg)satp);
}
pub unsafe fn enable(s: &Space) {
    switch(s);
}
pub fn user(pc: u64, sp: u64, kernel: u64) -> Frame {
    let mut f = Frame {
        pc,
        status: 32,
        ..Frame::zero()
    };
    f.x[2] = sp;
    f.x[12] = kernel;
    f.x[13] = CODE;
    f.x[14] = DATA;
    f
}
pub fn idle_frame() -> Frame {
    let mut f = Frame {
        pc: idle as *const () as u64,
        status: 0x120,
        ..Frame::zero()
    };
    f.x[2] = (&raw const KSTACK) as u64 + 65536;
    f
}
pub fn is_user(f: &Frame) -> bool {
    f.status & 256 == 0
}
pub fn is_syscall() -> bool {
    cause() == 8
}
pub fn syscall(f: &Frame) -> (u64, [u64; 4]) {
    (f.x[17], [f.x[10], f.x[11], f.x[12], f.x[13]])
}
pub fn advance(f: &mut Frame) {
    f.pc += 4;
}
pub fn result(f: &mut Frame, n: u64) {
    f.x[10] = n;
}
pub fn probe(i: usize) -> u32 {
    match i {
        1 => 0x0000006f,
        2 => 0x00063503,
        3 => 0x0006b023,
        _ => 0x00070067,
    }
}
pub fn expected_fault(i: usize, kernel: u64) -> bool {
    let at: u64;
    unsafe {
        asm!("csrr {}, stval",out(reg)at,options(nomem,nostack));
    }
    at == match i {
        2 => kernel,
        3 => CODE,
        _ => DATA,
    } && cause()
        == match i {
            2 => 13,
            3 => 15,
            _ => 12,
        }
}
