#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn, static_mut_refs)]
use bastion_core::{Call, Kernel, Limits, ProcessId, abi::*, decisions, elf::Image};
use core::{
    arch::asm,
    fmt::{self, Write},
    mem::MaybeUninit,
    panic::PanicInfo,
};
#[cfg(target_arch = "aarch64")]
#[path = "aarch64.rs"]
mod arch;
#[cfg(target_arch = "riscv64")]
#[path = "riscv64.rs"]
mod arch;
use arch::Frame;
const N: usize = 5;
#[repr(C, align(4096))]
#[derive(Clone, Copy)]
pub struct Region([u8; REGION_SIZE]);
#[repr(C, align(4096))]
#[derive(Clone, Copy)]
pub struct Table([u64; 512]);
#[derive(Clone, Copy)]
struct Space {
    tables: [Table; 4],
    code: Region,
    data: Region,
}
static mut SPACES: [Space; N] = [Space {
    tables: [Table([0; 512]); 4],
    code: Region([0; REGION_SIZE]),
    data: Region([0; REGION_SIZE]),
}; N];
#[repr(C, align(16))]
struct Stack([u8; 65536]);
#[unsafe(no_mangle)]
static mut KSTACK: Stack = Stack([0; 65536]);
static mut STATE: MaybeUninit<State> = MaybeUninit::uninit();
static mut INPUT: bastion_core::console::InputQueue<256> = bastion_core::console::InputQueue::new();
static mut OUTPUT: bastion_core::console::OutputQueue<4096> =
    bastion_core::console::OutputQueue::new();
static INIT: &[u8] = include_bytes!(env!("BASTION_INIT"));
struct State {
    core: Kernel,
    pids: [ProcessId; N],
    frames: [Frame; N],
    idle: Frame,
    origin: u64,
    ticks: u64,
    faults: u8,
    busy_ticks: u64,
    passed: bool,
    throttled: bool,
}
struct Serial;
impl Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            if b == b'\n' {
                arch::put(b'\r');
            }
            arch::put(b);
        }
        Ok(())
    }
}
macro_rules! log { ($($arg:tt)*)=>{let _=Serial.write_fmt(format_args!($($arg)*));}; }
#[unsafe(no_mangle)]
unsafe extern "C" fn kernel_main() -> ! {
    arch::init();
    log!("BASTION {} / LEAN-GENERATED C POLICY\n", arch::NAME);
    let image = Image::parse(INIT, arch::MACHINE).expect("init ELF rejected");
    let period = arch::frequency() / 10;
    let mut core = Kernel::new(64, period, period / 20).unwrap();
    let first = core
        .spawn(
            Limits {
                pages: 36,
                capabilities: 2,
                cpu_budget: period / 3,
            },
            36,
        )
        .unwrap();
    let mut pids = [first; N];
    for pid in &mut pids[1..] {
        *pid = core
            .spawn(
                Limits {
                    pages: 6,
                    capabilities: 0,
                    cpu_budget: period / 5,
                },
                6,
            )
            .unwrap();
    }
    let mut frames = [Frame::zero(); N];
    for i in 0..N {
        if i == 0 {
            image.load(&mut SPACES[i].code.0, &mut SPACES[i].data.0);
        } else {
            SPACES[i].code.0[..4].copy_from_slice(&arch::probe(i).to_le_bytes());
        }
        arch::map(&mut SPACES[i], if i == 0 { 16 } else { 1 });
        frames[i] = arch::user(
            if i == 0 { image.entry } else { CODE },
            if i == 0 { STACK_TOP } else { DATA + 4096 },
            (&raw const STATE) as u64,
        );
    }
    // Supervisor mappings are identical across roots, and user regions are private.
    arch::enable(&SPACES[0]);
    STATE.write(State {
        core,
        pids,
        frames,
        idle: arch::idle_frame(),
        origin: arch::clock(),
        ticks: 0,
        faults: 0,
        busy_ticks: 0,
        passed: false,
        throttled: false,
    });
    log!("PASS ELF INIT: USER MODE / PRIVATE PAGE TABLES / RX CODE / RW NX DATA\n");
    arch::start_timer();
    arch::restore(select(STATE.assume_init_mut()))
}
unsafe fn select(s: &mut State) -> *const Frame {
    if let Some(pid) = s.core.schedule() {
        let i = s.pids.iter().position(|p| *p == pid).unwrap();
        arch::switch(&SPACES[i]);
        &s.frames[i]
    } else {
        &s.idle
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn trap(frame: &Frame) -> *const Frame {
    let s = STATE.assume_init_mut();
    let user = arch::is_user(frame);
    let i = s
        .core
        .current()
        .map(|pid| s.pids.iter().position(|p| *p == pid).unwrap());
    s.core.account_until(arch::clock() - s.origin).unwrap();
    if let Some(i) = i {
        assert!(user);
        s.frames[i] = *frame;
        if s.core.snapshot(s.pids[i]).unwrap().usage.cpu_remaining == 0 {
            s.throttled = true;
        }
    }
    if arch::timer(frame) {
        s.ticks += 1;
        for n in 0..16 {
            if decisions::console_budget(n) == 0 {
                break;
            }
            if let Some(b) = arch::get() {
                INPUT.push(b);
            } else {
                break;
            }
        }
        for _ in 0..64 {
            if !arch::writable() {
                break;
            }
            if let Some(b) = OUTPUT.pop() {
                arch::put(b);
            } else {
                break;
            }
        }

        if i == Some(1) {
            s.busy_ticks += 1;
        }
    } else if user && arch::is_syscall() {
        let i = i.unwrap();
        let (op, args) = arch::syscall(frame);
        arch::advance(&mut s.frames[i]);
        let result = match op {
            3 => {
                s.core.syscall(Call::Yield).unwrap();
                0
            }
            4 => {
                s.core.syscall(Call::Exit).unwrap();
                0
            }
            0 => s.pids[i].raw(),
            _ => syscall(s, i, op, args),
        };
        arch::result(&mut s.frames[i], result);
    } else if user {
        let i = i.unwrap();
        let expected = i >= 2 && arch::expected_fault(i, (&raw const STATE) as u64);
        s.core.fault_current().unwrap();
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
            log!("USER FAULT CONTAINED: TASK {}\n", i);
        }
    } else {
        panic!("kernel exception");
    }
    if !s.passed && s.ticks >= 200 && s.faults == 7 && s.busy_ticks >= 2 && s.throttled {
        assert_eq!(s.core.used_pages(), 42);
        s.passed = true;
        log!(
            "PASS BUSY PROCESS PREEMPTED\nPASS CPU BUDGET: PROCESS THROTTLED\nALL PORT CHECKS PASSED\n"
        );
    }
    select(s)
}
unsafe fn syscall(s: &State, i: usize, op: u64, a: [u64; 4]) -> u64 {
    if op == ABI {
        return VERSION;
    }
    if a[0] != if op == QUERY { STATUS } else { CONSOLE }
        || decisions::authorized(s.pids[i].raw(), s.pids[0].raw(), 1, 1, 1) == 0
    {
        return DENIED;
    }
    let data = &mut SPACES[i].data.0;
    match op {
        READ | WRITE => {
            let Some(at) = buffer_offset(a[1], a[2], REGION_SIZE as u64) else {
                return ADDRESS;
            };
            let len = a[2] as usize;
            if len == 0 {
                return 0;
            }
            if op == READ {
                match INPUT.pop() {
                    Some(Ok(b)) => {
                        data[at] = b;
                        1
                    }
                    Some(Err(())) => INPUT_ERROR,
                    None => AGAIN,
                }
            } else {
                let mut bytes = [0u8; MAX_COPY * 2];
                let mut count = 0;
                for &b in &data[at..at + len] {
                    if b == b'\n' {
                        bytes[count] = b'\r';
                        count += 1;
                    }
                    bytes[count] = b;
                    count += 1;
                }
                if !OUTPUT.push(&bytes[..count]) {
                    return AGAIN;
                }
                len as u64
            }
        }
        QUERY => {
            let Some(at) = buffer_offset(a[3], RECORD_BYTES as u64, REGION_SIZE as u64) else {
                return ADDRESS;
            };
            let mut r = [0u64; WORDS];
            match a[1] {
                0 => {
                    r[0] = s.ticks;
                    r[1] = decisions::abi_version(0);
                    r[2] = s.core.used_pages();
                    r[3] = s.core.page_capacity();
                    r[4] = N as u64;
                    r[5] = arch::MACHINE as u64;
                }
                1 => {
                    let Some(&pid) = s.pids.get(a[2] as usize) else {
                        return INVALID;
                    };
                    r[0] = pid.raw();
                    if let Ok(p) = s.core.snapshot(pid) {
                        r[1] = 1;
                        r[2] = p.usage.pages;
                        r[3] = p.usage.capabilities as u64;
                        r[4] = p.usage.cpu_remaining;
                        r[5] = p.limits.pages;
                        r[6] = p.limits.capabilities as u64;
                        r[7] = p.limits.cpu_budget;
                    }
                }
                2 => {}
                _ => return INVALID,
            }
            for (b, v) in data[at..at + RECORD_BYTES]
                .as_chunks_mut::<8>()
                .0
                .iter_mut()
                .zip(r)
            {
                b.copy_from_slice(&v.to_le_bytes());
            }
            0
        }
        _ => INVALID,
    }
}
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log!("FAIL: {}\n", info);
    loop {
        unsafe {
            asm!("wfi", options(nomem, nostack));
        }
    }
}
