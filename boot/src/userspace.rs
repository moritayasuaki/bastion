//! The kernel owns mappings and devices; all command parsing runs in init.
use crate::{Frame, SPACES, State, console, network, physical};
use bastion_core::{
    Limits,
    abi::*,
    decisions,
    elf::{Image, Machine},
};
static mut INPUT: bastion_core::console::InputQueue<256> = bastion_core::console::InputQueue::new();
pub unsafe fn poll() {
    for n in 0..16 {
        if decisions::console_budget(n) == 0 {
            break;
        }
        if let Some(b) = console::read_byte() {
            INPUT.push(b);
        } else {
            break;
        }
    }
    console::drain();
}
static INIT: &[u8] = include_bytes!(env!("BASTION_INIT"));
pub unsafe fn launch(s: &mut State) {
    let image = Image::parse(INIT, Machine::X86_64).expect("invalid init ELF");
    let i = 5;
    let pid = s
        .core
        .spawn(
            Limits {
                pages: 36,
                capabilities: 2,
                cpu_budget: 20_000_000,
            },
            36,
        )
        .expect("init reservation");
    let space = &mut SPACES[i];
    let parent = &SPACES[0];
    space.root = parent.root;
    space.root.0[0] = physical((&raw const space.pdpt) as u64) | 7;
    space.pdpt.0[0] = physical((&raw const space.pd) as u64) | 7;
    space.pd.0[2] = physical((&raw const space.pt) as u64) | 7;
    let code = &mut *((&raw mut space.code) as *mut [u8; REGION_SIZE]);
    let data = &mut *((&raw mut space.data) as *mut [u8; REGION_SIZE]);
    image.load(code, data);
    for n in 0..16 {
        space.pt.0[n] = decisions::user_page_entry(physical((&raw const space.code[n]) as u64), 1);
        space.pt.0[256 + n] =
            decisions::user_page_entry(physical((&raw const space.data[n]) as u64), 2);
    }
    s.pids[i] = pid;
    s.roots[i] = physical((&raw const space.root) as u64);
    s.frames[i] = Frame {
        rip: image.entry,
        rsp: STACK_TOP,
        cs: 0x23,
        ss: 0x1b,
        flags: 0x202,
        ..Frame::ZERO
    };
    s.init_started = true;
    console::begin_interactive();
    console::print(format_args!(
        "PASS ELF INIT: RING 3 / RX CODE / RW NX DATA\n"
    ));
}
pub unsafe fn syscall(s: &State, i: usize, f: &Frame) -> u64 {
    if f.rax == ABI {
        return VERSION;
    }
    let owner = if s.init_started { s.pids[5].raw() } else { 0 };
    let expected = if f.rax == QUERY { STATUS } else { CONSOLE };
    if f.rdi != expected || decisions::authorized(s.pids[i].raw(), owner, 1, 1, 1) == 0 {
        return DENIED;
    }
    let space = &mut SPACES[i];
    let data = core::slice::from_raw_parts_mut((&raw mut space.data) as *mut u8, REGION_SIZE);
    match f.rax {
        READ | WRITE => {
            let Some(at) = buffer_offset(f.rsi, f.rdx, REGION_SIZE as u64) else {
                return ADDRESS;
            };
            let len = f.rdx as usize;
            if len == 0 {
                return 0;
            }
            if f.rax == READ {
                match INPUT.pop() {
                    Some(Ok(b)) => {
                        data[at] = b;
                        1
                    }
                    Some(Err(())) => INPUT_ERROR,
                    None => AGAIN,
                }
            } else {
                if !console::write_bytes(&data[at..at + len]) {
                    return AGAIN;
                }
                len as u64
            }
        }
        QUERY => {
            let Some(at) = buffer_offset(f.r10, RECORD_BYTES as u64, REGION_SIZE as u64) else {
                return ADDRESS;
            };
            let mut r = [0u64; WORDS];
            match f.rsi {
                0 => {
                    r[0] = s.ticks;
                    r[1] = decisions::abi_version(0);
                    r[2] = s.core.used_pages();
                    r[3] = s.core.page_capacity();
                    r[4] = 6;
                    r[5] = 62;
                }
                1 => {
                    let Some(&pid) = s.pids.get(f.rdx as usize) else {
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
                2 => {
                    if let Some(n) = network::status() {
                        r[0] = 1;
                        r[1] = n.tcp as u64;
                        r[2] = n.received_bytes as u64;
                        r[3] = n.queued_bytes as u64;
                    }
                }
                _ => return INVALID,
            }
            for (bytes, value) in data[at..at + RECORD_BYTES]
                .as_chunks_mut::<8>()
                .0
                .iter_mut()
                .zip(r)
            {
                bytes.copy_from_slice(&value.to_le_bytes());
            }
            0
        }
        _ => INVALID,
    }
}
