# Native userspace and executable format

Bastion now builds `user/` as a separate Rust `no_std` application, validates its
ELF file, loads private pages, and starts it in x86 ring 3, AArch64 EL0 or RISC-V
U-mode. Line editing, command matching and response formatting run in that process.
The kernel provides serial I/O and fixed-width status records through checked
system calls. Commands are built into this one application; they are not separate
executables yet. The initial image is embedded at build time. There is no filesystem,
interactive program launcher, arbitrary download-and-execute interface or Linux ABI.

## Why ELF64

Use **little-endian ELF64** for native applications on all three architectures.
It already describes loadable segments, permissions, entry points, zero-initialized
memory and architecture identification, and is supported by Rust/LLVM tooling.
The format is shared; each CPU needs a separately compiled binary.

The first profile accepts **static `ET_EXEC`**, without an interpreter, dynamic
linker, relocations or TLS. This keeps loading small enough to audit. A later profile
should add **static PIE (`ET_DYN`)**, a restricted relocation set, randomized load
addresses and an authenticated source of randomness. ASLR is not implemented now.
A newer format alone would not provide a stronger process boundary.

PE/COFF is useful for UEFI firmware entry and does not need to be the application
format. WebAssembly could become a portable application runtime later, but would
add an interpreter/compiler and its own resource controls. It does not replace
native exception handling, address spaces or kernel access checks.

Sources: [ELF program loading](https://gabi.xinuos.com/elf/07-pheader.html),
[Arm ELF64 ABI](https://github.com/ARM-software/abi-aa/blob/main/aaelf64/aaelf64.rst),
[RISC-V ELF ABI](https://riscv-non-isa.github.io/riscv-elf-psabi-doc/).

## Loader profile

`src/elf.rs` validates the whole file before changing any backing pages. It accepts
at most eight program headers, the expected CPU, ELF class/version/endianness and
OSABI zero, and only the supported header types. It rejects writable executable
segments, overlapping page ranges, unsupported flags, file-range overflow, invalid
alignment, file size exceeding memory size, and entry points outside file-backed
executable code. Dynamic/interpreter/TLS segments and executable GNU stacks fail.
RISC-V uses the integer LP64 ABI; only the compressed-instruction flag is optional.

| Region | Address | Access |
|---|---|---|
| Code and constants | `0x400000..0x410000` | User RX |
| Initialized data and BSS | `0x500000..0x508000` | User RW, NX |
| Initial stack | `0x508000..0x510000` | User RW, NX, grows down |

All 32 user pages are mapped; unused bytes and the stack are zeroed. Four additional
page-table pages are reserved, for a 36-page init charge. RV64 currently uses three
of those four table pages. The stack has no internal guard page; access beyond the
mapped region faults. Static backing storage is not recycled after termination.
The trusted loader supplies the initial application and its fixed service grants.
ELF parsing does not establish executable authenticity or Linux compatibility.

Lean-generated C decides segment/window admission, bounded user copies, service
owner authorization, and RX/RW-NX leaf entries for each CPU. Rust parses the file,
places bytes and installs tables. The parser and hardware setup are tested but not
formally verified end-to-end.

## System-call ABI v1

Arguments and results are 64-bit integers; records contain little-endian `u64`
words. The kernel derives caller identity from its scheduler. Every general-purpose
register other than the result is preserved across a syscall. There is no floating
point, SIMD, TLS, signal ABI, `argc`/`argv`, environment or auxiliary vector yet.

| CPU | Trap | Number | Arguments 0–3 | Result |
|---|---|---|---|---|
| x86-64 | `int 0x80` | RAX | RDI, RSI, RDX, R10 | RAX |
| AArch64 | `svc #0` | X8 | X0–X3 | X0 |
| RV64 | `ecall` | A7 | A0–A3 | A0 |

| Number | Call | Arguments and result |
|---|---|---|
| 0 | self ID | Returns current process ID |
| 3 | yield | Returns zero when scheduled again |
| 4 | exit | Terminates caller |
| 64 | ABI version | Returns 1; no authority needed |
| 65 | console read | Grant, buffer, length; returns bytes read or error |
| 66 | console write | Grant, buffer, length; returns bytes accepted or error |
| 67 | status query | Grant, selector, index, destination; returns zero or error |

The legacy x86 demonstration calls 1/2/5 remain available only on x86. They are not
part of the portable v1 interface. Constants and layouts are in `src/abi.rs`.
The small syscall wrapper and formatter are in `user/src/main.rs`.

Init alone receives two fixed launch grants: handle 1 for console and handle 2 for
system status. These are separate from the core's process-control capability table
and its reported capability usage. They cannot be transferred, minted, or reused by
other processes; the kernel checks both the handle and its scheduled owner on every
call. They are bounded boot-time authority, not a general service-capability system.
The shell's status grant deliberately permits inspection of all boot processes.

Copies accept only the caller's actually mapped RW data/stack window, at most 256
bytes per call. A query writes exactly 128 bytes. Code addresses, kernel addresses,
wrapping/out-of-range spans and oversized requests fail before copying. The kernel
uses its own mapping of the backing storage and never retains a user pointer.
Read-only constants must first be copied to the application's stack/data buffer.

Reads are nonblocking and currently return at most one byte. Empty read/write
requests return zero. Writes are accepted atomically into a bounded queue, with LF
converted to CRLF. A full queue returns `Again`; the app yields and retries. UART or
input queue errors invalidate the editor's current line. The kernel does not parse
commands. There is no serial login; terminal access is administrative authority.

Errors are unsigned values: `Denied = MAX`, `BadAddress = MAX-1`, `Again = MAX-2`,
`Invalid = MAX-3`, `InputError = MAX-4`. Unsupported requests return an error; unknown
operations can be denied during grant checking before opcode validation.

Status is a zero-initialized 16-word record, with unused words reserved zero:

| Selector | Fields, starting at word 0 |
|---|---|
| 0, index ignored | Delivered timer ticks, policy ABI, reserved pages, page capacity, process slot count, ELF machine ID |
| 1, process slot index | PID, live flag, pages, process-control capability count, CPU remaining, page limit, capability limit, CPU budget |
| 2, index ignored | Network available, TCP state, RX queued bytes, TX queued bytes |

TCP state numbers are 0 closed, 1 listening, 2 SYN sent, 3 SYN received, 4 established,
5 FIN wait 1, 6 FIN wait 2, 7 close wait, 8 closing, 9 last ACK, 10 time wait.
On the ARM/RV ports the network record is zero because no NIC adapter is present.

## Next interfaces

The next stage is process-owned socket handles with endpoint rights and socket/buffer
quotas; nonblocking TCP/UDP calls; readiness/wait; runtime process launch and a small
Rust application library. A filesystem and a dynamic linker can follow independently.
TCP accept needs a bounded connection pool; the current kernel echo demonstration
has only one TCP socket. Exit/fault cleanup must reclaim socket ownership and scrub
buffers before reuse. SSH additionally needs authenticated encryption and key
provisioning; TCP alone does not provide remote administration.
