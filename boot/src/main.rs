#![no_std]
#![no_main]
// This architecture module runs on one CPU with interrupts masked throughout
// boot and trap handling. User mode and idle are the only interruptible states.
#![allow(unsafe_op_in_unsafe_fn, static_mut_refs)]

use bastion_core::{Call, Error, Handle, Kernel, Limits, ProcessId, Reply, decisions};
use core::{
    arch::{asm, global_asm},
    mem::MaybeUninit,
    panic::PanicInfo,
};
mod console;
mod network;
global_asm!(include_str!("entry.S"));

macro_rules! log { ($($arg:tt)*) => { console::print(format_args!($($arg)*)); }; }

#[repr(C)]
struct Request {
    id: [u64; 4],
    revision: u64,
    response: *const u64,
}
impl Request {
    const fn new(a: u64, b: u64) -> Self {
        Self {
            id: [0xc7b1dd30df4c8b88, 0x0a82e883a194f07b, a, b],
            revision: 0,
            response: core::ptr::null(),
        }
    }
}
#[used]
#[unsafe(link_section = ".requests_start")]
static START: [u64; 4] = [
    0xf6b8f4b39de7d1ae,
    0xfab91a6940fcb9cf,
    0x785c6ed015d3e316,
    0x181e920a7852b9d9,
];
#[used]
#[unsafe(link_section = ".requests")]
static mut REVISION: [u64; 3] = [0xf9562b2d5c95a6c8, 0x6a7b384944536bdc, 3];
#[used]
#[unsafe(link_section = ".requests")]
static mut HHDM: Request = Request::new(0x48dcf1cb8ad2b852, 0x63984e959a98244b);
#[used]
#[unsafe(link_section = ".requests")]
static mut ADDRESS: Request = Request::new(0x71ba76863cc55f63, 0xb2644a48c516a487);
#[used]
#[unsafe(link_section = ".requests")]
static mut FRAMEBUFFER: Request = Request::new(0x9d5827dcd881dd75, 0xa3148604f6fab11b);
#[used]
#[unsafe(link_section = ".requests_end")]
static END: [u64; 2] = [0xadc0e0531bb10d03, 0x9572709f31764c62];

#[repr(C, align(16))]
struct Stack([u8; 32768]);
#[unsafe(no_mangle)]
static mut BOOT_STACK: Stack = Stack([0; 32768]);
static mut TRAP_STACK: Stack = Stack([0; 32768]);
static mut IDLE_STACK: Stack = Stack([0; 32768]);
static mut DOUBLE_FAULT_STACK: Stack = Stack([0; 32768]);

#[repr(C, packed)]
struct Descriptor {
    limit: u16,
    base: u64,
}
#[repr(C, packed)]
struct Tss {
    reserved: u32,
    rsp: [u64; 3],
    reserved2: u64,
    ist: [u64; 7],
    reserved3: u64,
    reserved4: u16,
    iomap: u16,
}
static mut TSS: Tss = Tss {
    reserved: 0,
    rsp: [0; 3],
    reserved2: 0,
    ist: [0; 7],
    reserved3: 0,
    reserved4: 0,
    iomap: 104,
};
static mut GDT: [u64; 7] = [
    0,
    0x00af9a000000ffff,
    0x00cf92000000ffff,
    0x00cff2000000ffff,
    0x00affa000000ffff,
    0,
    0,
];
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct Gate {
    low: u16,
    selector: u16,
    ist: u8,
    flags: u8,
    mid: u16,
    high: u32,
    zero: u32,
}
impl Gate {
    const EMPTY: Self = Self {
        low: 0,
        selector: 0,
        ist: 0,
        flags: 0,
        mid: 0,
        high: 0,
        zero: 0,
    };
    fn new(address: u64, flags: u8, ist: u8) -> Self {
        Self {
            low: address as u16,
            selector: 8,
            ist,
            flags,
            mid: (address >> 16) as u16,
            high: (address >> 32) as u32,
            zero: 0,
        }
    }
}
static mut IDT: [Gate; 256] = [Gate::EMPTY; 256];

// Must match entry.S. Each saved context includes every general purpose register.
#[repr(C)]
#[derive(Clone, Copy)]
struct Frame {
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rsi: u64,
    rdi: u64,
    rbp: u64,
    rdx: u64,
    rcx: u64,
    rbx: u64,
    rax: u64,
    vector: u64,
    error: u64,
    rip: u64,
    cs: u64,
    flags: u64,
    rsp: u64,
    ss: u64,
}
const _: () = assert!(core::mem::size_of::<Frame>() == 176);
const _: () = assert!(core::mem::size_of::<Tss>() == 104);
impl Frame {
    const ZERO: Self = Self {
        r15: 0,
        r14: 0,
        r13: 0,
        r12: 0,
        r11: 0,
        r10: 0,
        r9: 0,
        r8: 0,
        rsi: 0,
        rdi: 0,
        rbp: 0,
        rdx: 0,
        rcx: 0,
        rbx: 0,
        rax: 0,
        vector: 0,
        error: 0,
        rip: 0,
        cs: 0,
        flags: 0,
        rsp: 0,
        ss: 0,
    };
}
#[repr(C, align(4096))]
#[derive(Clone, Copy)]
struct Page([u8; 4096]);
#[repr(C, align(4096))]
#[derive(Clone, Copy)]
struct Table([u64; 512]);
#[repr(C)]
#[derive(Clone, Copy)]
struct Space {
    root: Table,
    pdpt: Table,
    pd: Table,
    pt: Table,
    code: Page,
    data: Page,
}
impl Space {
    const ZERO: Self = Self {
        root: Table([0; 512]),
        pdpt: Table([0; 512]),
        pd: Table([0; 512]),
        pt: Table([0; 512]),
        code: Page([0; 4096]),
        data: Page([0; 4096]),
    };
}
const N: usize = 5;
static mut SPACES: [Space; N] = [Space::ZERO; N];
static mut PHYS_BASE: u64 = 0;
static mut VIRT_BASE: u64 = 0;
static mut STATE: MaybeUninit<State> = MaybeUninit::uninit();
struct State {
    core: Kernel,
    pids: [ProcessId; N],
    frames: [Frame; N],
    roots: [u64; N],
    idle: Frame,
    initial_root: u64,
    origin: u64,
    ticks: u64,
    hits: [u64; N],
    post_fault_ticks: u64,
    post_fault_peer_checks: u64,
    faults: u8,
    peer_ok: bool,
    cap_ok: bool,
    throttled: bool,
    passed: bool,
}
unsafe extern "C" {
    fn load_segments();
    fn restore(frame: *const Frame) -> !;
    fn idle();
    static isr_table: [u64; 34];
    static worker_start: u8;
    static worker_end: u8;
    static peer_start: u8;
    static peer_end: u8;
    static attack_start: u8;
    static attack_end: u8;
    static write_start: u8;
    static write_end: u8;
    static nx_start: u8;
    static nx_end: u8;
}

/// # Safety
/// Caller must own the port/device and run at a privilege allowing port I/O.
pub unsafe fn out(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port,in("al") value,options(nomem,nostack,preserves_flags));
}
/// # Safety
/// Caller must own the port/device and run at a privilege allowing port I/O.
pub unsafe fn input(port: u16) -> u8 {
    let x;
    asm!("in al, dx",in("dx") port,out("al") x,options(nomem,nostack,preserves_flags));
    x
}
fn clock() -> u64 {
    let lo: u32;
    let hi: u32;
    // LFENCE orders entry before the timestamp. This MVP requires SSE2-capable x86-64.
    unsafe {
        asm!("lfence", "rdtsc",out("eax") lo,out("edx") hi,options(nomem,nostack));
    }
    (hi as u64) << 32 | lo as u64
}
unsafe fn physical(virtual_address: u64) -> u64 {
    virtual_address - VIRT_BASE + PHYS_BASE
}
unsafe fn set_root(root: u64) {
    asm!("mov cr3, {}",in(reg) root,options(nostack));
}

unsafe fn interrupts() {
    TSS.rsp[0] = (&raw const TRAP_STACK) as u64 + 32768;
    TSS.ist[0] = (&raw const DOUBLE_FAULT_STACK) as u64 + 32768;
    let base = (&raw const TSS) as u64;
    GDT[5] = 103 | ((base & 0xffffff) << 16) | (0x89 << 40) | (((base >> 24) & 0xff) << 56);
    GDT[6] = base >> 32;
    let gdtr = Descriptor {
        limit: 55,
        base: (&raw const GDT) as u64,
    };
    asm!("lgdt [{}]",in(reg) &gdtr,options(readonly,nostack));
    load_segments();
    for (i, address) in isr_table[..33].iter().enumerate() {
        IDT[i] = Gate::new(*address, 0x8e, if i == 8 { 1 } else { 0 });
    }
    IDT[128] = Gate::new(isr_table[33], 0xee, 0); // DPL3 syscall gate; all others DPL0.
    let idtr = Descriptor {
        limit: 4095,
        base: (&raw const IDT) as u64,
    };
    asm!("lidt [{}]",in(reg) &idtr,options(readonly,nostack));
    // Remap legacy PIC, mask every device except PIT IRQ0. No user I/O access.
    for (port, value) in [
        (0x20, 0x11),
        (0xa0, 0x11),
        (0x21, 0x20),
        (0xa1, 0x28),
        (0x21, 4),
        (0xa1, 2),
        (0x21, 1),
        (0xa1, 1),
        (0x21, 0xfe),
        (0xa1, 0xff),
    ] {
        out(port, value);
        out(0x80, 0);
    }
    out(0x43, 0x36);
    out(0x40, (1193 & 255) as u8);
    out(0x40, (1193 >> 8) as u8);
}

unsafe fn address_space(
    i: usize,
    old_root: u64,
    hhdm: u64,
    start: *const u8,
    end: *const u8,
) -> u64 {
    let space = &mut SPACES[i];
    let old = &*((old_root & 0x000f_ffff_ffff_f000).wrapping_add(hhdm) as *const Table);
    // Only the upper supervisor half is inherited; no ambient user mappings.
    for j in 256..512 {
        space.root.0[j] = decisions::supervisor_entry(old.0[j]);
    }
    space.root.0[0] = physical((&raw const space.pdpt) as u64) | 7;
    space.pdpt.0[0] = physical((&raw const space.pd) as u64) | 7;
    space.pd.0[2] = physical((&raw const space.pt) as u64) | 7;
    space.pt.0[0] = decisions::user_page_entry(physical((&raw const space.code) as u64), 1);
    space.pt.0[256] = decisions::user_page_entry(physical((&raw const space.data) as u64), 2);
    assert!(
        space.pt.0[0] != 0 && space.pt.0[256] != 0,
        "Lean rejected user mapping"
    );
    let len = end as usize - start as usize;
    assert!(len <= 4096);
    core::ptr::copy_nonoverlapping(start, space.code.0.as_mut_ptr(), len);
    physical((&raw const space.root) as u64)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn kernel_main() -> ! {
    console::init(FRAMEBUFFER.response);
    log!("BASTION / LEAN POLICY + RUST\nX86-64 NATIVE KERNEL\nLEAN-GENERATED C POLICY ACTIVE\n");
    assert_eq!(
        core::ptr::read_volatile(&raw const REVISION[2]),
        0,
        "Limine revision"
    );
    assert!(!HHDM.response.is_null() && !ADDRESS.response.is_null());
    let hhdm = *HHDM.response.add(1);
    PHYS_BASE = *ADDRESS.response.add(1);
    VIRT_BASE = *ADDRESS.response.add(2);
    let root: u64;
    asm!("mov {}, cr3",out(reg) root);
    let cr4: u64;
    asm!("mov {}, cr4",out(reg) cr4);
    assert_eq!(cr4 & (1 << 12), 0, "5-level paging not supported");
    // TLS bases are not part of the supported user ABI yet. Prevent users from
    // changing FS/GS bases, and discard any bootloader values.
    asm!("mov cr4, {}", in(reg) cr4 & !(1 << 16));
    for msr in [0xc0000100u32, 0xc0000101, 0xc0000102] {
        asm!("wrmsr", in("ecx") msr, in("eax") 0u32, in("edx") 0u32);
    }
    // Enable NX and supervisor write protection. Disable x87/SIMD until its
    // context-switch support is implemented; attempts fault the user process.
    let mut lo: u32;
    let hi: u32;
    asm!("rdmsr",in("ecx") 0xc0000080u32,out("eax") lo,out("edx") hi);
    // INT 0x80 is the only syscall ABI. Disable alternate ring-transition paths.
    lo = (lo | (1 << 11)) & !1;
    asm!("wrmsr",in("ecx") 0xc0000080u32,in("eax") lo,in("edx") hi);
    asm!("wrmsr",in("ecx") 0x174u32,in("eax") 0u32,in("edx") 0u32);
    let mut cr0: u64;
    asm!("mov {}, cr0",out(reg) cr0);
    cr0 |= (1 << 16) | (1 << 2) | (1 << 3);
    asm!("mov cr0, {}",in(reg) cr0);
    interrupts();
    network::init();
    let mut core = Kernel::new(48, 100_000_000, 1_000_000).unwrap();
    let limits = Limits {
        pages: 6,
        capabilities: 2,
        cpu_budget: 20_000_000,
    };
    let pids = core::array::from_fn(|_| core.spawn(limits, 6).unwrap());
    assert_eq!(core.reserve_pages(pids[0], 1), Err(Error::PageLimit));
    log!("PASS MEMORY QUOTA: 6 PAGES PER PROCESS\n");
    let programs = [
        (&raw const worker_start, &raw const worker_end),
        (&raw const peer_start, &raw const peer_end),
        (&raw const attack_start, &raw const attack_end),
        (&raw const write_start, &raw const write_end),
        (&raw const nx_start, &raw const nx_end),
    ];
    let mut roots = [0; N];
    let mut frames = [Frame::ZERO; N];
    for i in 0..N {
        roots[i] = address_space(i, root, hhdm, programs[i].0, programs[i].1);
        frames[i] = Frame {
            rip: 0x400000,
            cs: 0x23,
            ss: 0x1b,
            rsp: 0x501000,
            flags: 0x202,
            r12: (&raw const STATE) as u64,
            ..Frame::ZERO
        };
    }
    log!("5 USER PROCESSES / PRIVATE PAGE TABLES\nTIMER PREEMPTION ENABLED\n");
    let idle_frame = Frame {
        rip: idle as *const () as u64,
        cs: 8,
        ss: 16,
        flags: 0x202,
        rsp: (&raw const IDLE_STACK) as u64 + 32768,
        ..Frame::ZERO
    };
    STATE.write(State {
        core,
        pids,
        frames,
        roots,
        idle: idle_frame,
        initial_root: root,
        origin: clock(),
        ticks: 0,
        hits: [0; N],
        post_fault_ticks: 0,
        post_fault_peer_checks: 0,
        faults: 0,
        peer_ok: false,
        cap_ok: false,
        throttled: false,
        passed: false,
    });
    let state = STATE.assume_init_mut();
    let next = select(state);
    restore(next)
}

unsafe fn select(s: &mut State) -> *const Frame {
    loop {
        match s.core.schedule() {
            Some(pid) => {
                let i = s.pids.iter().position(|p| *p == pid).unwrap();
                // Invalid return state must kill the task, not fault IRET in ring 0.
                if decisions::valid_user_return(s.frames[i].rip, s.frames[i].rsp) == 0 {
                    s.core.fault_current().unwrap();
                    continue;
                }
                s.frames[i].cs = 0x23;
                s.frames[i].ss = 0x1b;
                s.frames[i].flags = decisions::user_flags(s.frames[i].flags);
                set_root(s.roots[i]);
                return &s.frames[i];
            }
            None => {
                set_root(s.initial_root);
                return &s.idle;
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn trap(frame: &Frame) -> *const Frame {
    let s = STATE.assume_init_mut();
    let current = s.core.current();
    let index = current.map(|pid| s.pids.iter().position(|p| *p == pid).unwrap());
    if frame.cs & 3 == 0 && frame.vector != 32 {
        panic!(
            "KERNEL FAULT {} RIP {:X} ERROR {:X}",
            frame.vector, frame.rip, frame.error
        );
    }
    let now = clock().checked_sub(s.origin).expect("TSC regressed");
    s.core.account_until(now).expect("TSC regressed");
    if let Some(i) = index {
        s.frames[i] = *frame;
        if s.core.snapshot(s.pids[i]).unwrap().usage.cpu_remaining == 0 && !s.throttled {
            s.throttled = true;
            log!("PASS CPU BUDGET: PROCESS THROTTLED\n");
        }
    }
    match frame.vector {
        32 => {
            s.ticks += 1;
            if let Some(i) = index {
                s.hits[i] += 1;
                if i == 0 && s.faults == 7 {
                    s.post_fault_ticks += 1;
                }
            }
            out(0x20, 0x20);
            network::poll(s.ticks);
        }
        128 => {
            let i = index.expect("syscall without user process");
            assert_eq!(frame.cs & 3, 3);
            let opcode = decisions::syscall_opcode(frame.rax);
            let call = match opcode {
                0 => Some(Call::SelfInfo),
                1 => Some(Call::Inspect(Handle::from_raw(frame.rdi))),
                2 => Some(Call::Terminate(Handle::from_raw(frame.rdi))),
                3 => Some(Call::Yield),
                4 => Some(Call::Exit),
                _ => None,
            };
            s.frames[i].rax = if let Some(call) = call {
                match s.core.syscall(call) {
                    Ok(Reply::Done) => 0,
                    Ok(Reply::Info(info)) => info.id.raw(),
                    Err(_) => u64::MAX,
                }
            } else if opcode == 5 {
                // Bounded demo reporting; no user pointer dereference.
                match (i, frame.rdi) {
                    (1, 0x21) if !s.peer_ok => {
                        s.peer_ok = true;
                        log!("PASS PRIVATE MEMORY: ZEROED PEER PAGE\n");
                    }
                    (2, 0x22) if !s.cap_ok => {
                        s.cap_ok = true;
                        log!("PASS FORGED CAPABILITY DENIED\n");
                    }
                    (1, 0x23) if s.faults == 7 => {
                        s.post_fault_peer_checks += 1;
                    }
                    (_, 0xe0..=0xef) => {
                        log!("FAIL USER TEST {:X}\n", frame.rdi);
                        s.core.fault_current().unwrap();
                    }
                    _ => {}
                }
                0
            } else {
                u64::MAX
            };
        }
        _ => {
            let i = index.expect("fault without user process");
            let addr: u64;
            asm!("mov {}, cr2",out(reg) addr);
            let expected = frame.vector == 14
                && match i {
                    2 => addr == (&raw const STATE) as u64 && frame.error & 7 == 5,
                    3 => addr == 0x400000 && frame.error & 7 == 7,
                    4 => addr == 0x500000 && frame.error & 0x15 == 0x15,
                    _ => false,
                };
            s.core.fault_current().unwrap();
            // Backing pages are static in this MVP and are never reused. The dead
            // root is never scheduled again. Future reuse must unmap and scrub.
            if expected {
                s.faults |= 1 << (i - 2);
                log!(
                    "PASS FAULT CONTAINED: {}\n",
                    match i {
                        2 => "KERNEL MEMORY",
                        3 => "CODE WRITE",
                        _ => "STACK EXECUTE",
                    }
                );
            } else {
                log!("USER FAULT CONTAINED: TASK {} VECTOR {}\n", i, frame.vector);
            }
        }
    }
    if s.ticks >= 200 && !s.passed {
        let ok = s.post_fault_ticks >= 2
            && s.post_fault_peer_checks >= 2
            && s.peer_ok
            && s.cap_ok
            && s.faults == 7
            && s.hits[0] >= 2
            && s.throttled
            && s.core.snapshot(s.pids[0]).is_ok()
            && s.core.snapshot(s.pids[1]).is_ok()
            && s.core.used_pages() == 12;
        s.passed = true;
        if ok {
            log!(
                "PASS BUSY PROCESS PREEMPTED\nPASS SURVIVORS RUN AFTER FAULTS\nALL BOOT CHECKS PASSED\n"
            );
        } else {
            log!("FAIL: INCOMPLETE BOOT TESTS\n");
        }
        #[cfg(feature = "qemu-test")]
        asm!("out dx, eax",in("dx") 0xf4u16,in("eax") if ok {0x10u32} else {0x11u32},options(nomem,nostack));
    }
    select(s)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        asm!("cli", options(nomem, nostack));
    }
    log!("FAIL: {}\n", info);
    #[cfg(feature = "qemu-test")]
    unsafe {
        asm!("out dx, eax",in("dx") 0xf4u16,in("eax") 0x11u32,options(nomem,nostack));
    }
    loop {
        unsafe {
            asm!("hlt", options(nomem, nostack));
        }
    }
}
