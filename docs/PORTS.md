# CPU and platform support

Bastion supports three native 64-bit execution targets. This is a specific tested
machine matrix, not a claim of compatibility with every board or VPS using that CPU.
All targets are single-CPU, use the same Lean → C policy and Rust core, and load the
same userspace shell source as an architecture-specific static ELF64 executable.

| Target | Boot and hardware | Current support |
|---|---|---|
| x86-64 | Limine BIOS/UEFI, PC PIC/PIT, COM1 | Private ring-3 processes, console, legacy virtio PCI TCP/UDP |
| AArch64 | QEMU `virt`, Cortex-A53, EL1, GICv2, PL011 | Private EL0 processes, architectural timer, console |
| RV64 IMAC | QEMU `virt`, default OpenSBI, S-mode, Sv39, 16550 UART | Private U-mode processes, SBI TIME, console |

ARM32 and RV32 are not supported. ARM/RV ports do not yet implement NICs, PCI/virtio
transport, storage, UEFI integration, DTB/ACPI discovery or provider networking.
Their device addresses, RAM window and timer setup match the supplied QEMU profile.
RV64 assumes the QEMU virt 10 MHz timebase; AArch64 reads CNTFRQ. Do not use these
images as generic board or provider images without implementing platform discovery.
No cloud VPS has been validated. AArch64 VPS deployment needs additional firmware,
interrupt-controller, NIC and device discovery work.

## Build and test

Install Rust targets and QEMU machines:

```sh
rustup target add x86_64-unknown-none aarch64-unknown-none-softfloat riscv64imac-unknown-none-elf
# Ubuntu:
sudo apt-get install clang qemu-system-x86 qemu-system-arm qemu-system-misc
# macOS: Apple's Clang does not include a RISC-V backend.
brew install llvm qemu
export BASTION_CC="$(brew --prefix llvm)/bin/clang"
```

From the project root:

```sh
cargo xtask build-port --arch aarch64
cargo xtask test-port --arch aarch64
cargo xtask console-port --arch aarch64

cargo xtask build-port --arch riscv64
cargo xtask test-port --arch riscv64
cargo xtask console-port --arch riscv64
```

Exit the interactive launcher with Ctrl-A then X. Outputs are
`dist/bastion-aarch64.elf` and `dist/bastion-riscv64.elf`; the launcher loads them with
QEMU's `-kernel`. RV64 uses QEMU's default OpenSBI firmware. This runs Bastion itself
as the guest OS, with no Linux guest beneath it. See [README](../README.md) for x86
ISO commands. `cargo xtask check` checks all target code; `BASTION_CC` must support
all three targets when running the complete checks.

## Boundary and validation

Each port saves all general-purpose registers on a kernel-owned trap stack. ARM
uses four-level 4 KiB translations; RV uses Sv39. Kernel/device mappings have no
user permission. User code is RX and data/stack RW/NX. Leaf encodings come from
Lean-generated C. Floating-point/SIMD and TLS are not supported. Kernel RAM itself
uses broad supervisor mappings in these initial ports, including writable executable
kernel pages; this is user-process isolation, not kernel self-protection.

The core charges elapsed hardware-counter ticks, uses a 100 ms replenishment period
on ARM/RV, and preempts on roughly 1 kHz timer interrupts. Init gets one third of the
period and the busy probe one fifth. The delivered-tick uptime is approximate;
hypervisor delays and interrupt overhead affect measured budgets. Fixed backing
storage and kernel stacks are not a general physical allocator, and no SMP safety
is claimed.

`test-port` runs real user-mode attempts to read kernel memory, write code, and
execute stack/data; each must fault while the shell and busy process survive.
It verifies timer preemption, syscall pointer/handle rejection, every serial command,
editing, overflow recovery, repeated output and continued timer progress. The host
suite separately tests ELF malformed/truncated inputs, overlapping segments, CPU
mismatch, bounds, BSS zeroing and stack initialization. These are integration checks,
not complete architecture or kernel verification.

Primary platform references: [QEMU Arm virt](https://www.qemu.org/docs/master/system/arm/virt.html),
[QEMU RISC-V virt](https://www.qemu.org/docs/master/system/riscv/virt.html).
