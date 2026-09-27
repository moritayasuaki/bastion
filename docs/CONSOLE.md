# Serial console

A **serial console** is a text connection to an operating system. The OS writes
characters to a serial port and receives the characters you type. A hypervisor
emulates that port and connects it to a terminal or provider console. This works
without a guest IP address or SSH.

The serial port is the transport; the shell is the program using it. Bastion's
commands now run in a separate **userspace Rust ELF application**. The kernel owns
the UART and validates console/status syscalls. The shell has administrative status
access, but cannot directly read kernel memory. There is no authentication protocol
on the serial connection; access to the terminal grants access to the shell.

## Start it

```sh
cargo xtask build
cargo xtask console
# Other native CPU ports:
cargo xtask build-port --arch aarch64
cargo xtask console-port --arch aarch64
# Substitute riscv64 for the RISC-V port.
```

After boot checks and user-ABI rejection probes:

```text
Bastion serial console. Type help.
bastion> help
```

Exit QEMU with **Ctrl-A, then X**. Backspace/Delete edits, Ctrl-U clears the line,
and Ctrl-C cancels. Both CR and LF work; CRLF submits once. Lines are limited to
64 ASCII bytes. Invalid/overlong input or UART errors reject the whole line, so a
truncated prefix cannot execute. There is no history, tab completion, escape parser,
filesystem, program-launch command or POSIX shell syntax.

| Command | Result |
|---|---|
| `help` / `?` | Commands and editing keys |
| `ps` | Live process usage and exited processes |
| `limits` | Page reservations and process limits |
| `net status` | Network availability and TCP/UDP status; down on ARM/RV |
| `uptime` | Delivered timer ticks and approximate seconds |
| `version` | Architecture, kernel version, policy and userspace ABI versions |

These are built-in commands in one userspace application, not separate binaries.
CPU units are hardware counter ticks; uptime uses timer interrupts at roughly 1 kHz,
not a wall-clock guarantee. `ps` reports runnable, throttled or exited status.
The kernel's process-control capability count excludes the two fixed service grants.

On x86, `--no-network`, `--iso PATH`, and `--uefi CODE --uefi-vars VARS` are supported
by the console launcher. A boot-test ISO exits automatically and is not interactive.
The launcher connects serial to stdio and opens no public listener. It uses QEMU's
[character-device multiplexer](https://www.qemu.org/docs/master/system/invocation.html#character-device-options).

## Implementation boundary

x86 uses COM1 at `0x3f8` with 115200 8N1. ARM uses PL011; RV uses the virt 16550 UART.
Interactive input is serial-only. The x86 framebuffer displays boot diagnostics;
a VNC keyboard is a different, unsupported device. A VPS must expose a compatible
serial connection. No VPS has been tested.

The kernel polls at most 16 input bytes per timer tick into a 256-byte input queue.
UART/queue overrun discards pending input and reports an error to the editor. The
4096-byte output queue accepts a complete bounded write or returns `Again`, without
overwriting earlier bytes. LF expands to CRLF atomically. A tick makes at most 64
nonwaiting output attempts. The application yields and retries when output is full.
Boot/panic diagnostics use a separate bounded synchronous path.

The user process owns its 64-byte editor and command parsing. Lean-generated C
chooses edit actions; kernel Lean checks validate buffer windows and launch authority.
Polling bounds and the full UART/editor implementation have different proof scopes:
only the specified scalar policy theorems are formally checked. Console service work
has no separate quota, and interrupt overhead affects process CPU accounting.

See [USERSPACE.md](USERSPACE.md) for the executable and syscall contract.

## Verification

```sh
cargo xtask check
cargo xtask build
cargo xtask console-test
cargo xtask console-test --no-network
cargo xtask console-test --uefi CODE --uefi-vars VARS
cargo xtask test-port --arch aarch64
cargo xtask test-port --arch riscv64
```

Tests send real UART input and cover commands, editing, invalid bytes, overflow,
repeated output and timer progress. Booted init tests invalid pointers, oversized
copies, code-page destinations and forged handles before showing its prompt. Host
tests cover queue backpressure and overrun signaling. Input is paced for repeatable
results; arbitrary-rate pasted input is not guaranteed lossless. Logs stay under
ignored `target/bastion/`.
