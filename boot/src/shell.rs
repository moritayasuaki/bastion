//! Read-only kernel monitor for a trusted operator on COM1, not a userspace shell.
use crate::{State, console, network};
use bastion_core::{
    console::{Command, Event, LineEditor},
    decisions,
};
static mut EDITOR: LineEditor = LineEditor::new();
static mut STARTED: bool = false;
fn prompt() {
    console::print(format_args!("bastion> "));
}

pub unsafe fn poll(state: &State) {
    if !STARTED {
        STARTED = true;
        console::begin_interactive();
        console::print(format_args!("\nBastion serial console. Type help.\n"));
        prompt();
    }
    console::drain();
    // Reserve enough output capacity for the largest response before reading.
    // Backpressure leaves bytes in the UART, whose overrun rejects the whole line.
    if console::output_available() < 2048 {
        return;
    }
    let mut processed = 0;
    while decisions::console_budget(processed) != 0 {
        let Some(byte) = console::read_byte() else {
            break;
        };
        processed += 1;
        let Ok(byte) = byte else {
            EDITOR.reject();
            break;
        };
        match EDITOR.feed(byte) {
            Event::None => {}
            Event::Echo(b) => console::print(format_args!("{}", char::from(b))),
            Event::Erase => console::print(format_args!("\x08 \x08")),
            Event::Clear(n) => {
                for _ in 0..n {
                    console::print(format_args!("\x08 \x08"));
                }
            }
            Event::Cancel => {
                console::print(format_args!("^C\n"));
                prompt();
                break;
            }
            Event::Rejected => {
                console::print(format_args!(
                    "\nERROR: invalid or overlong line (max 64 bytes).\n"
                ));
                prompt();
                break;
            }
            Event::Submit(command) => {
                console::print(format_args!("\n"));
                execute(command, state);
                prompt();
                break;
            }
        }
    }
}
unsafe fn execute(command: Command, state: &State) {
    match command {
        Command::Help => console::print(format_args!(
            "help / ?   show commands\nps         process status\nlimits     resource limits\nnet status TCP/UDP status\nuptime     elapsed timer ticks\nversion    kernel and policy version\nBackspace edits; Ctrl-U clears; Ctrl-C cancels. Read-only monitor.\n"
        )),
        Command::Processes => {
            console::print(format_args!("PID STATE PAGES CAPS CPU_REMAINING\n"));
            for &pid in &state.pids {
                if let Ok(s) = state.core.snapshot(pid) {
                    let status = if s.usage.cpu_remaining == 0 {
                        "throttled"
                    } else {
                        "runnable"
                    };
                    console::print(format_args!(
                        "{} {} {} {} {}\n",
                        pid.raw(),
                        status,
                        s.usage.pages,
                        s.usage.capabilities,
                        s.usage.cpu_remaining
                    ));
                } else {
                    console::print(format_args!("{} exited\n", pid.raw()));
                }
            }
        }
        Command::Limits => {
            console::print(format_args!(
                "PAGES reserved={} capacity={} (4096 bytes/page)\nCPU unit=TSC ticks; console/network work is kernel work\n",
                state.core.used_pages(),
                state.core.page_capacity()
            ));
            for &pid in &state.pids {
                if let Ok(s) = state.core.snapshot(pid) {
                    console::print(format_args!(
                        "PID {} pages={}/{} caps={}/{} cpu={}/{}\n",
                        pid.raw(),
                        s.usage.pages,
                        s.limits.pages,
                        s.usage.capabilities,
                        s.limits.capabilities,
                        s.usage.cpu_remaining,
                        s.limits.cpu_budget
                    ));
                }
            }
        }
        Command::Network => match network::status() {
            Some(n) => console::print(format_args!(
                "NETWORK UP 10.0.2.15/24 gateway=10.0.2.2\nUDP echo :9000 max=1200 bytes\nTCP echo :9000 state={:?} rx={} tx={} bytes\n",
                n.tcp, n.received_bytes, n.queued_bytes
            )),
            None => console::print(format_args!(
                "NETWORK DOWN (unsupported, unavailable, or disabled NIC)\n"
            )),
        },
        Command::Uptime => console::print(format_args!(
            "UPTIME ticks={} approximate_seconds={} (PIT ~1000 Hz)\n",
            state.ticks,
            state.ticks / 1000
        )),
        Command::Version => console::print(format_args!(
            "Bastion 0.1.0 x86-64 / Lean-generated C policy ABI {}\n",
            decisions::abi_version(0)
        )),
        Command::Empty => {}
        Command::Unknown => console::print(format_args!("Unknown command. Type help.\n")),
    }
}
