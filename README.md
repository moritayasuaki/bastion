# Bastion

An experimental x86-64 kernel for process resource limits and security boundaries.
**Lean defines executable policy, compiles it to C, and Rust connects it to hardware.**

Bastion boots directly under QEMU or a compatible hypervisor. It includes isolated
ring-3 processes, timer preemption, capability checks, memory/CPU quotas, a bounded
virtio UDP stack, and an eight-VM Octave/PSIV relay laboratory.

**Research prototype:** no SSH, filesystem, dynamic executable loader, persistent
state, or provider-specific IP configuration. The policy proofs do not constitute
whole-kernel or cryptographic verification. See [security scope](docs/ARCHITECTURE.md).

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
cargo xtask relay-demo    # starts, tests, and stops eight local relay VMs
```

The first build fetches a pinned Limine revision. The relay demo creates fresh
private link keys and forwards UDP ports only on loopback. Alice and Bob are host
clients; relay services run inside Bastion. It checks key recovery with three
altered shares and one stopped relay, encrypted traffic, and replay/forgery rejection.
BIOS guests use 64 MB each; UEFI guests use 128 MB. See the [network guide](docs/NETWORK.md).

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
| `crypto/` | Vendored PSIV portable C backend and Rust crypto adapter |
| `tests/` | Integration tests, reference vectors, and optional Octave Lean bridge |
| `tools/` | Build/check commands, integrity checks, and relay harness |
| `docs/` | Architecture, extraction, networking, deployment, and licenses |

Generated policy and reference vectors are included so a normal build does not
require another project checkout. Build caches, private relay modules, packet captures,
local reports, and boot images are excluded from Git.

## Verification and licensing

The project checks 35 Lean policy theorems, 36 Rust tests, and conformance corpora
covering 18,046 executable-policy cases, 1,254 abstract-model cases, and 18,944 Octave
cases. Field arithmetic executes through Lean-generated C. The PSIV cipher is the
existing handwritten portable C implementation; its integration is not formally proved.

See [validation and reproduction](docs/VALIDATION.md), [Lean extraction](docs/EXTRACTION.md),
and [contributing](CONTRIBUTING.md). Bastion's original code is [MIT licensed](LICENSE).
Third-party components retain their own licenses, listed in [THIRD_PARTY.md](THIRD_PARTY.md).
