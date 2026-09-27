#![no_std]
#![no_main]
use bastion_core::{
    abi::*,
    console::{Command, Event, LineEditor},
};
use core::{
    arch::{asm, global_asm},
    fmt::{self, Write},
    panic::PanicInfo,
};
#[cfg(target_arch = "x86_64")]
global_asm!(".section .text.entry\n.global _start\n_start:\n xor rbp,rbp\n call user_main\n ud2");
#[cfg(target_arch = "aarch64")]
global_asm!(".section .text.entry\n.global _start\n_start:\n bl user_main\n brk #0");
#[cfg(target_arch = "riscv64")]
global_asm!(".section .text.entry\n.global _start\n_start:\n call user_main\n unimp");
fn syscall(op: u64, a: u64, b: u64, c: u64, d: u64) -> u64 {
    let result;
    unsafe {
        #[cfg(target_arch = "x86_64")]
        asm!("int 0x80", inlateout("rax") op => result, in("rdi") a, in("rsi") b, in("rdx") c, in("r10") d, options(nostack));
        #[cfg(target_arch = "aarch64")]
        asm!("svc #0", in("x8") op, inlateout("x0") a => result, in("x1") b, in("x2") c, in("x3") d, options(nostack));
        #[cfg(target_arch = "riscv64")]
        asm!("ecall", in("a7") op, inlateout("a0") a => result, in("a1") b, in("a2") c, in("a3") d, options(nostack));
    }
    result
}
fn yield_now() {
    syscall(3, 0, 0, 0, 0);
}
struct Console;
impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Constants are copied into private RW memory before crossing the ABI.
        let mut buffer = [0u8; MAX_COPY];
        for chunk in s.as_bytes().chunks(MAX_COPY) {
            buffer[..chunk.len()].copy_from_slice(chunk);
            loop {
                let n = syscall(
                    WRITE,
                    CONSOLE,
                    buffer.as_ptr() as u64,
                    chunk.len() as u64,
                    0,
                );
                if n == AGAIN {
                    yield_now();
                    continue;
                }
                if n != chunk.len() as u64 {
                    return Err(fmt::Error);
                }
                break;
            }
        }
        Ok(())
    }
}
macro_rules! print { ($($arg:tt)*) => { let _ = Console.write_fmt(format_args!($($arg)*)); }; }
fn query(kind: u64, index: u64) -> [u64; WORDS] {
    let mut record = [0; WORDS];
    if syscall(QUERY, STATUS, kind, index, record.as_mut_ptr() as u64) != 0 {
        record.fill(0);
    }
    record
}
fn tcp_state(state: u64) -> &'static str {
    match state {
        0 => "CLOSED",
        1 => "LISTEN",
        2 => "SYN-SENT",
        3 => "SYN-RECEIVED",
        4 => "ESTABLISHED",
        5 => "FIN-WAIT-1",
        6 => "FIN-WAIT-2",
        7 => "CLOSE-WAIT",
        8 => "CLOSING",
        9 => "LAST-ACK",
        10 => "TIME-WAIT",
        _ => "UNKNOWN",
    }
}
fn execute(command: Command) {
    match command {
        Command::Help => {
            print!(
                "help / ?   show commands\nps         process status\nlimits     resource limits\nnet status TCP/UDP status\nuptime     elapsed timer ticks\nversion    kernel and policy version\nBackspace edits; Ctrl-U clears; Ctrl-C cancels. Userspace shell.\n"
            );
        }
        Command::Processes => {
            print!("PID STATE PAGES CAPS CPU_REMAINING\n");
            for i in 0..query(0, 0)[4] {
                let r = query(1, i);
                if r[1] == 0 {
                    print!("{} exited\n", r[0]);
                } else {
                    print!(
                        "{} {} {} {} {}\n",
                        r[0],
                        if r[4] == 0 { "throttled" } else { "runnable" },
                        r[2],
                        r[3],
                        r[4]
                    );
                }
            }
        }
        Command::Limits => {
            let r = query(0, 0);
            print!(
                "PAGES reserved={} capacity={} (4096 bytes/page)\nCPU unit=hardware ticks; console/network work is kernel work\n",
                r[2], r[3]
            );
            for i in 0..r[4] {
                let s = query(1, i);
                if s[1] != 0 {
                    print!(
                        "PID {} pages={}/{} caps={}/{} cpu={}/{}\n",
                        s[0], s[2], s[5], s[3], s[6], s[4], s[7]
                    );
                }
            }
        }
        Command::Network => {
            let r = query(2, 0);
            if r[0] == 0 {
                print!("NETWORK DOWN (unsupported, unavailable, or disabled NIC)\n");
            } else {
                print!(
                    "NETWORK UP 10.0.2.15/24 gateway=10.0.2.2\nUDP echo :9000 max=1200 bytes\nTCP echo :9000 state={} rx={} tx={} bytes\n",
                    tcp_state(r[1]),
                    r[2],
                    r[3]
                );
            }
        }
        Command::Uptime => {
            let r = query(0, 0);
            print!(
                "UPTIME ticks={} approximate_seconds={} (timer ~1000 Hz)\n",
                r[0],
                r[0] / 1000
            );
        }
        Command::Version => {
            let r = query(0, 0);
            print!(
                "Bastion 0.1.0 {} / Lean-generated C policy ABI {} / userspace ABI {}\n",
                match r[5] {
                    183 => "aarch64",
                    243 => "riscv64",
                    _ => "x86-64",
                },
                r[1],
                VERSION
            );
        }
        Command::Empty => {}
        Command::Unknown => {
            print!("Unknown command. Type help.\n");
        }
    }
}
#[unsafe(no_mangle)]
extern "C" fn user_main() -> ! {
    if syscall(ABI, 0, 0, 0, 0) != VERSION {
        loop {
            yield_now();
        }
    }
    while query(0, 0)[0] < 250 {
        yield_now();
    }
    let bad = syscall(WRITE, CONSOLE, u64::MAX, 1, 0) == ADDRESS
        && syscall(WRITE, 999, DATA, 1, 0) == DENIED
        && syscall(READ, CONSOLE, STACK_TOP - 1, 2, 0) == ADDRESS
        && syscall(WRITE, CONSOLE, DATA, 257, 0) == ADDRESS
        && syscall(QUERY, STATUS, 0, 0, CODE) == ADDRESS;
    if !bad {
        print!("FAIL USER ABI CHECKS\n");
        loop {
            yield_now();
        }
    }
    print!(
        "PASS USER ABI: BAD POINTERS AND FORGED HANDLES DENIED\nBastion serial console. Type help.\nbastion> "
    );
    let mut editor = LineEditor::new();
    loop {
        let mut byte = 0u8;
        let n = syscall(READ, CONSOLE, &mut byte as *mut u8 as u64, 1, 0);
        if n == INPUT_ERROR {
            editor.reject();
            continue;
        }
        if n != 1 {
            yield_now();
            continue;
        }
        match editor.feed(byte) {
            Event::None => {}
            Event::Echo(b) => {
                print!("{}", char::from(b));
            }
            Event::Erase => {
                print!("\x08 \x08");
            }
            Event::Clear(n) => {
                for _ in 0..n {
                    print!("\x08 \x08");
                }
            }
            Event::Cancel => {
                print!("^C\nbastion> ");
            }
            Event::Rejected => {
                print!("\nERROR: invalid or overlong line (max 64 bytes).\nbastion> ");
            }
            Event::Submit(c) => {
                print!("\n");
                execute(c);
                print!("bastion> ");
            }
        }
    }
}
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    syscall(4, 0, 0, 0, 0);
    loop {
        yield_now();
    }
}
