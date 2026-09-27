# Validation

Run the complete source checks:

```sh
cargo xtask check
```

This checks 35 Lean theorems, regenerates and compares the committed C artifacts,
executes 32 Rust tests, and checks formatting and Clippy for host and bare-metal
code. The policy object audit requires all 34 scalar exports and no undefined
symbols or Lean heap runtime. The boot image links 32 policy symbols.

| Corpus | Cases | Reference |
|---|---:|---|
| Executable policy | 16,716 | Lean runtime definitions across all 34 exports |
| Abstract policy | 1,254 | Natural-number resource/capability model |

The five network tests cover two virtual Ethernet peers: UDP boundaries, truncation,
full queues, checksums, unbound ports, TCP active/passive open, a dropped data segment
and retransmission, a held receive window, partial streams, half-close and reconnect,
malformed headers, every truncation of a valid minimum TCP frame, and socket capacity.
These are integration tests, not exhaustive protocol verification.

## Native boot and traffic

```sh
cargo xtask build --test
cargo xtask smoke --memory 64M
cargo xtask smoke --machine q35
cargo xtask build
cargo xtask network-test
```

Add `--uefi CODE --uefi-vars VARS` to `smoke` or `network-test` for UEFI.
See [NETWORK.md](NETWORK.md) for firmware examples. CI runs the same source checks,
BIOS/UEFI isolation tests and TCP/UDP traffic tests on Ubuntu 24.04.

Boot tests verify private process pages, forbidden kernel access, read-only code,
non-executable data, forged capability rejection, page quotas, CPU throttling,
preemption and surviving processes after faults. Network checks use real host
TCP/UDP sockets against the native guest. They cover 525 UDP exchanges, payloads
0–1200 bytes and oversize rejection, three TCP connections with streams up to
65,536 bytes, and FIN with pending data followed by EOF.

Use 64 MB for BIOS and 128 MB for UEFI. The recorded development toolchain is Rust
1.98.1, Lean 4.32.1, QEMU 11.1.1 and xorriso 1.5.8.pl02 on macOS. No public VPS has
been validated. Test logs stay in ignored `target/bastion/`; generated images and
local validation evidence are not public source artifacts.
