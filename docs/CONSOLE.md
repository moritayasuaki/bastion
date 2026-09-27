# Serial console

A **serial console** is a text input/output connection to the operating system.
The OS writes characters to a serial port and receives the characters you type.
Historically that port connected a physical terminal with a cable. A hypervisor
can emulate the port and connect it to a terminal window or a provider's console.
The guest does not need an IP address, SSH, or a functioning network stack.

The serial port is the transport. The prompt and commands are the program using
that transport. Bastion's first implementation is a read-only **kernel monitor**;
it is not a POSIX shell or a userspace process.

## Start it

```sh
cd bastion
cargo xtask build
cargo xtask console
```

After the boot checks, the terminal shows:

```text
Bastion serial console. Type help.
bastion> help
```

Type a command and press Enter. Backspace/Delete erases one character; Ctrl-U clears
the line; Ctrl-C cancels it. Exit QEMU with **Ctrl-A, then X**. Commands are lowercase.
Both CR and LF are accepted, and CRLF submits only once. Lines are limited to 64
ASCII bytes; overlong lines, unsupported control/non-ASCII bytes and UART receive
errors reject the entire line. Ctrl-U, Ctrl-C or Enter recovers input. Arrow-key
history, tab completion and escape sequences are not implemented.

| Command | Result |
|---|---|
| `help` or `?` | Commands and editing keys |
| `ps` | Live process usage and exited processes |
| `limits` | Reserved pages/global capacity and each live process's limits |
| `net status` | NIC availability, static address/gateway, UDP port and TCP state |
| `uptime` | PIT timer ticks and approximate elapsed seconds |
| `version` | Kernel version and Lean policy ABI version |

CPU accounting values are TSC ticks, not milliseconds. Uptime uses delivered PIT
interrupts at approximately 1000 Hz; it is not a wall-clock accuracy guarantee.
`ps` distinguishes runnable, throttled and exited tasks. It does not label a task
as running while the monitor itself is executing in the kernel.

`--no-network` demonstrates that the console works without a NIC. `--iso PATH`
selects a release ISO; `--uefi CODE --uefi-vars VARS` selects firmware. The boot-test
ISO exits QEMU automatically and cannot host an interactive session. For example:

```sh
cargo xtask console --no-network
cargo xtask console --uefi /opt/homebrew/share/qemu/edk2-x86_64-code.fd \
  --uefi-vars /opt/homebrew/share/qemu/edk2-i386-vars.fd
```

The launcher connects COM1 to QEMU stdio and uses its character multiplexer for the
exit key. It does not open a public console listener. BIOS uses 64 MB; UEFI 128 MB.
Equivalent essential QEMU options are:

```text
-display none -monitor none \
-chardev stdio,id=console,mux=on,signal=off -serial chardev:console
```

See the [QEMU character-device documentation](https://www.qemu.org/docs/master/system/invocation.html#character-device-options).

## Implementation and boundary

Bastion uses the PC-compatible COM1 UART at I/O address `0x3f8`, configured for
115200 baud, eight data bits, no parity and one stop bit (115200 8N1). The existing
framebuffer still displays boot diagnostics. Interactive commands use serial only;
a graphical VNC keyboard is a different input device and is not implemented.
A VPS must expose a compatible UART and its serial connection. No VPS has been tested.

The monitor runs on the boot CPU with interrupts masked and exposes only status.
There is no login/authentication protocol inside Bastion; access relies on control
of the local terminal or the provider's console. It adds no user syscalls, process
creation or direct access to kernel memory. A userspace shell requires the loader
and capability-controlled console API described in [USERSPACE.md](USERSPACE.md).

The safe core owns a 64-byte line buffer and a fixed output queue. Lean-generated C
chooses input actions and limits each timer poll to 16 input bytes. Four new proofs
cover append bounds, printable input, rejection state and poll bounds. Rust performs
buffer updates, command matching and UART I/O; the full editor/driver is not proved.

After boot the UART output queue holds 4096 bytes. The driver makes at most 64
nonwaiting output attempts per tick; input pauses unless 2048 output bytes remain
available, enough for one bounded response. At most one submitted command executes
per tick. A full queue refuses new bytes without overwriting existing output; CRLF
pairs enqueue atomically. A stalled console can lose incoming bytes when the UART
FIFO overruns; that marks the line invalid. UART errors observed during output
polling are retained until the editor consumes them. Panic output uses a separate
bounded synchronous path so it does not depend on timer interrupts.

Console work has no separate service quota. Current elapsed-TSC accounting includes
interrupt overhead, so console activity can affect effective process CPU budgets.
These bounds do not establish a complete real-time or DoS guarantee.

## Tests

```sh
cargo xtask check
cargo xtask build
cargo xtask console-test
cargo xtask console-test --no-network
cargo xtask console-test --uefi CODE --uefi-vars VARS
```

Host tests exercise editing, cancellation, CRLF, boundary-length lines, overflow,
invalid bytes, UART error signaling, sustained input and atomic output backpressure.
QEMU tests send real serial input and verify each command, editing, error recovery,
repeated output, and timer progress. Input is paced below the UART FIFO limit;
this does not claim lossless arbitrary-rate pasted input. Logs are ignored under
`target/bastion/console-test/`.
