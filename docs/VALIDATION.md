# Validation

Run the full source checks with:

```sh
cargo xtask check
```

This checks 35 Lean theorems, compares freshly generated C with the committed
artifacts, executes 36 Rust tests, and checks formatting and Clippy for the host
workspace and bare-metal kernel. The policy object audit requires all 35 scalar
exports, no undefined symbols, and no Lean heap runtime. The kernel audit requires
critical process/network policy and PSIV symbols.

## Reference corpora

| Corpus | Cases | Checked against |
|---|---:|---|
| Executable policy | 18,046 | Lean runtime definitions across all 35 exports |
| Abstract policy | 1,254 | Reservation and CPU-charge model |
| Octave arithmetic | 18,944 | Independent Lean reference, all 70 four-subsets |

The reference files live in `tests/data/`; they are required test inputs, not runtime
logs. [Their provenance](../tests/data/README.md) describes regeneration. The external
Octave checkout is needed only to regenerate its corpus, not for ordinary builds/tests.

## Boot and network checks

```sh
cargo xtask build --test
cargo xtask smoke --memory 64M
cargo xtask smoke --machine q35
cargo xtask build
cargo xtask relay-demo
```

For UEFI, supply matching OVMF code/variable files through `--uefi PATH` and
`--uefi-vars PATH` to `smoke` or `relay-demo`. Paths vary by host; the network guide
includes an example. GitHub CI runs the Linux build, source checks, BIOS/UEFI boot
checks, and the relay laboratory.

The local test matrix passed BIOS `pc` at 64 MB, BIOS `q35` at 128 MB, and UEFI `q35`
at 128 MB. Eight-guest network checks passed BIOS at 64 MB and UEFI at 128 MB per
guest. A 64 MB UEFI relay boot exhausted the bootloader allocator, so the harness
uses 128 MB in that configuration. The recorded development toolchain was Rust
1.98.1, Lean 4.32.1, QEMU 11.1.1/TCG, and xorriso 1.5.8.pl02.

The boot checks exercise page quotas, private backing pages, forged capabilities,
kernel-memory access, code writes, NX data, CPU exhaustion, and timer preemption.
Two unaffected processes must continue after the attacking processes terminate.

Network checks cover payload sizes 0–1200, 520 consecutive descriptor-ring reuses,
all-eight relay delivery, three altered shares plus one stopped VM, mutual PSIV
confirmation, encrypted traffic in both directions, replay rejection, and a valid
retry after a forged tag. The three-share fault is injected by the host harness,
not by a complete Byzantine relay implementation. SIGTERM cleanup was also tested.

Fresh relay logs/captures are written to `target/bastion/relay-lab/`. They and private
key-bearing temporary images are not distributed as repository content.

These are targeted tests. No cloud-provider, physical-hardware, SMP, whole-kernel,
constant-time, Rust/Lean equivalence, or concrete cryptographic-reduction claim is made.
