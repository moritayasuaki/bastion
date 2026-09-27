# Bastion

An experimental x86-64 kernel for process resource limits and security boundaries.
**Lean defines executable policy, compiles it to C, and Rust connects it to hardware.**

Bastion boots directly under QEMU or a compatible hypervisor. It includes isolated
ring-3 processes, timer preemption, capability checks, memory/CPU quotas, a bounded
virtio TCP/UDP stack with fixed socket buffers, and an interactive serial console.

**Research prototype:** no SSH, filesystem, dynamic executable loader, persistent
state, or provider-specific IP configuration. The policy proofs do not constitute
whole-kernel verification. See [security scope](docs/ARCHITECTURE.md).

## Build and run

Install [Rust](https://rust-lang.github.io/rustup/installation/index.html) with
`rustfmt` and `clippy`, [elan](https://github.com/leanprover/elan#installation),
Clang, Git, Make, xorriso, and QEMU. Lean 4.32.1 is selected by
`proof/lean-toolchain`; Rust 1.98.1 is the tested compiler.

On Ubuntu, the native packages are:

```sh
sudo apt-get update
sudo apt-get install build-essential clang git xorriso qemu-system-x86 ovmf
rustup target add x86_64-unknown-none
```

On macOS, install QEMU and xorriso with `brew install qemu xorriso`; Clang is supplied
by Xcode Command Line Tools. Add the same Rust target shown above.

```sh
git clone https://github.com/moritayasuaki/bastion.git
cd bastion
cargo xtask check         # proofs, conformance, Rust tests, formatting and lints
cargo xtask build         # creates dist/bastion.iso
cargo xtask console       # interactive text console; type help
cargo xtask network-test  # boots one VM and checks TCP and UDP
```

The first build fetches a pinned Limine revision. Network checks use loopback-only
port forwards, 64 MB for BIOS and 128 MB for UEFI. The guest serves TCP and UDP echo
on port 9000. See the [network guide](docs/NETWORK.md) for kernel API and limits,
and the [userspace interface plan](docs/USERSPACE.md) for process-owned sockets,
resource quotas, console and loader work. Socket syscalls are not implemented yet.

The [serial console guide](docs/CONSOLE.md) explains what a serial console is,
the commands, and how to connect. Exit QEMU with Ctrl-A then X.

For the embedded process-isolation boot tests:

```sh
cargo xtask build --test
cargo xtask smoke
```

`cargo xtask help` lists image, firmware, and output options. Custom-ISO boot is
supported in principle; [cloud hardware and networking remain untested](docs/DEPLOYMENT.md).

## Source layout

| Path | Purpose |
|---|---|
| `src/`, `boot/` | Safe process/network core and x86-64 hardware adapter |
| `proof/`, `policy/` | Lean policy and proofs, generated C, FFI, and provenance |
| `tests/` | Integration tests and Lean reference vectors |
| `tools/` | Build/check commands, integrity checks, and TCP/UDP harness |
| `docs/` | Architecture, extraction, networking, deployment, and licenses |

Generated policy and reference vectors are included so a normal build does not
require another project checkout. Build caches, packet captures,
local reports, and boot images are excluded from Git.

## Verification and licensing

The project checks 39 Lean policy theorems, 37 Rust tests, and conformance corpora
covering 20,577 executable-policy cases and 1,254 abstract-model cases. Network
policy executes through Lean-generated C; TCP state and packet parsing use Rust.

See [validation and reproduction](docs/VALIDATION.md), [Lean extraction](docs/EXTRACTION.md),
and [contributing](CONTRIBUTING.md). Bastion's original code is [MIT licensed](LICENSE).
Third-party components retain their own licenses, listed in [THIRD_PARTY.md](THIRD_PARTY.md).
