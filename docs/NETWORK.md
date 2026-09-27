# UDP and the Octave/PSIV relay laboratory

Bastion implements an Octave v0.2 transport adapter using the vendored PSIV portable C backend. Eight separate QEMU guests boot Bastion directly. Two host Rust clients establish a shared root through those relays, confirm it with PSIV, and exchange protected UDP traffic. All machines are on one development host; this does not provide eight independently administered security domains. No cloud deployment has been performed.

## Run

```sh
cd bastion
cargo xtask build
cargo xtask relay-demo
```

The harness provisions fresh keys into eight temporary boot modules, builds private ISO variants, starts eight single-CPU guests (64 MB for BIOS, 128 MB for UEFI), and checks the existing process-isolation assertions before sending traffic. It uses OS randomness and rejection sampling for unbiased field coefficients. Ports are chosen dynamically and forwarded only on `127.0.0.1`; packet captures and serial logs are saved in `target/bastion/relay-lab/`. It stops all remaining guests and removes temporary images on completion or error. SIGINT/SIGTERM set a cancellation flag and run cleanup; SIGKILL cannot run cleanup. Temporary image deletion is not a secure-erasure guarantee.

For UEFI on the tested macOS host:

```sh
cargo xtask relay-demo \
  --uefi /opt/homebrew/share/qemu/edk2-x86_64-code.fd \
  --uefi-vars /opt/homebrew/share/qemu/edk2-i386-vars.fd
```

`--iso PATH` selects a normal release image. `--hold-seconds N` retains the seven surviving guests after the fault test, until the timer or cancellation. The demo runs two sessions: all eight relays, then three altered shares and an actually stopped eighth guest. Share corruption is injected by the harness before Alice's authenticated PUT; it exercises reconstruction under adversarial values, not a complete malicious-relay implementation or exhaustive network adversary.

To boot one unprovisioned UDP echo guest:

```sh
qemu-system-x86_64 -machine pc -accel tcg -cpu max -m 64M -smp 1 \
  -cdrom dist/bastion.iso -boot d -display none -monitor none -serial stdio \
  -netdev user,id=n,hostfwd=udp:127.0.0.1:19000-:9000 \
  -device virtio-net-pci,netdev=n,disable-modern=on -no-reboot
```

The normal image contains no relay keys and drops traffic on port 9001. Standard QEMU user networking is used: its `restrict=on` setting caused valid UDP replies to be discarded in the tested setup. There are no public listeners in this lab.

## Kernel transport and limits

| Item | Implemented boundary |
|---|---|
| NIC | Legacy/transitional PCI virtio-net, I/O BAR, MAC feature only |
| Address | Static IPv4 `10.0.2.15`, matching QEMU user networking |
| Protocols | Ethernet, ARP replies, IPv4, UDP; checksummed replies |
| Services | UDP echo 9000; provisioned authenticated relay 9001 |
| Payload | 0–1200 bytes; exact UDP/IP length relationship |
| Receive work | At most 8 frames per PIT tick, fixed 1514-byte frame buffers |
| DMA queues | Power-of-two queue sizes up to 256; bounded descriptor validation |
| Relay state | 4 live session slots; fixed 176-byte body per slot |
| Expiry | More than 60,000 PIT ticks since slot creation, approximately 60 seconds |
| Replay | Strictly increasing sequence per authenticated role/link; values 1–1,048,575 |

IP options, fragmentation, TCP, ICMP, IPv6, DHCP, DNS, routed initiation, modern-only virtio, and user-visible socket syscalls are absent. IPv4 UDP checksum zero is accepted as allowed by the protocol; nonzero checksums are verified. The kernel replies to the source MAC/IP and does not maintain an outbound ARP cache. Frames outside the fixed bounds and packets to unbound ports are dropped. Invalid DMA completions disable the NIC. Full transmit queues drop replies, with no unbounded allocation or retry.

The service is trusted kernel code, separate from the five embedded ring-3 test processes. It uses kernel-wide static memory and interrupt-time CPU work; it does not yet enforce separate process quotas on network services. Neither eight frames per tick nor the four-slot cap is a complete denial-of-service defense. Echo is intended for the local lab.

## Provisioning

Limine must load exactly one 88-byte module, with `module_path: boot():/boot/relay.bin`:

| Bytes | Value |
|---|---|
| 0–3 | ASCII `BRL1` |
| 4 | External relay label 1–8 |
| 5–7 | Zero |
| 8–23 | Deployment epoch, 16 random bytes |
| 24–55 | Alice-to-relay root link key, 32 random bytes |
| 56–87 | Bob-to-relay root link key, 32 random bytes |

Every relay has different Alice and Bob link keys. All eight share the deployment epoch. Client-to-relay and relay-to-client keys are separately derived; responses cannot be reflected as requests. The selected role is accepted only after verifying PSIV with that role's configured link key. Endpoint IP addresses and claimed participant IDs alone confer no authority.

**Provision fresh keys and epoch after every reboot.** Replay counters exist only in RAM. Reusing a module across reboot permits old authenticated requests to replay. The lab automatically regenerates everything per run. Sequence exhaustion also requires reprovisioning. Slot expiry never resets link counters. Images containing these keys are private credentials and must not be published as downloadable cloud images. Durable provisioning, crash recovery, and secure erasure remain future work.

## Wire format (Bastion adapter v1)

Each UDP relay record is a 32-byte authenticated header followed by PSIV ciphertext and its 16-byte tag:

| Offset | Size | Field |
|---|---|---|
| 0 | 4 | ASCII `OCTV` |
| 4 | 1 | Adapter version 1 |
| 5 | 1 | PUT=1, GET=2, ACK=129, SHARE=130 |
| 6 | 1 | Relay label 1–8 |
| 7 | 1 | Role Alice=1 or Bob=2 |
| 8 | 16 | Session identifier |
| 24 | 8 | Sequence, little-endian |

The full header is PSIV associated data. Nonce is four zero bytes followed by the 64-bit little-endian sequence. Alice may PUT a 176-byte body; Bob may GET with an empty body. ACK has an empty body; SHARE returns exactly the stored body. Total records are 48 or 224 bytes. An identical PUT at a fresh sequence is idempotent. A different authenticated body for the same live session poisons that slot until expiry. Authentication failure cannot advance the sequence counter. Valid authenticated but semantically rejected requests consume their sequence.

The body consists of a canonical 96-byte transcript, a 64-byte share, and 16 bytes of Alice's confirmation evidence. Transcript bytes are:

| Offset | Size | Value |
|---|---|---|
| 0 | 8 | ASCII `OCTVPS01` (version/profile domain) |
| 8 | 4 | `[4,8,3,1]` threshold/relay/corruption/failure profile |
| 12 | 4 | Suite 1, little-endian |
| 16 | 16 | Deployment epoch |
| 32 | 16 | Session identifier |
| 48 | 16 | Fresh Bob challenge |
| 64 | 8 | Alice identity 1, little-endian |
| 72 | 8 | Bob identity 2, little-endian |
| 80 | 8 | Ordered external relay labels `[1,2,3,4,5,6,7,8]` |
| 88 | 8 | Reserved zero bytes |

The adapter currently has one fixed Alice/Bob pair. The epoch selects its provisioned roster/keys. The host orchestrator supplies their fresh session/challenge context; peer discovery, independent endpoint applications, and a network challenge-handshake protocol are not implemented. Direction is bound by distinct KDF labels and authenticated record roles/opcodes. This is a new concrete adapter to Octave's abstract link/confirmation interfaces, not a claim that another existing transport understands this wire format.

## Octave arithmetic and confirmation

Parameters follow Octave SPEC v0.2: `k=4`, `n=8`, `t=3`, `f=1`. Each of 32 root bytes is the constant term of an independently sampled cubic over GF(257). A share contains 32 little-endian 16-bit field elements. Values above 256 are rejected, never reduced modulo 257. Reconstruction tests all available four-subsets, at most 70. A reconstructed coordinate equal to 256 is rejected as a noncanonical root byte. Acceptance requires a unique confirmed **32-byte value**, not a subset count or early first match. Distinct confirmed values cause failure.

Sharing, interpolation, and the uniqueness-state transition execute through Lean-generated C. `tests/OctaveVectors.lean` imports the original `Relay.Core`; `cargo xtask octave-check --root /path/to/octave` compares 18,944 cases covering every label, every four-subset, normal shares, and arbitrary field inputs. This is executable agreement on those cases, not a general refinement theorem between the two projects.

HKDF-SHA256 uses salt `Bastion Octave PSIV adapter v1`, a 32-byte input root, and one expand block with info `context || byte(label_length) || label`. Labels `confirmAB`, `confirmBA`, `trafficAB`, and `trafficBA` bind the canonical transcript; link keys use context `epoch || relay_label || role` and labels `client-to-relay` / `relay-to-client`. PSIV has a 32-byte key, 12-byte nonce, and 16-byte tag. Confirmation is the tag for an empty message with the transcript as associated data and sequence 1. Evidence is fixed before examining reconstruction candidates. The demo's reverse confirmation and both traffic directions use real host UDP sockets; traffic does not traverse the relays after key establishment. This is a one-record traffic demonstration, not a general secure datagram session API with replay state.

## Trust and research status

The PSIV C backend is vendored unchanged from the PSIV project’s portable C backend and wrapped by a small safe Rust API. It is the existing handwritten portable implementation, not a newly extracted Lean cipher. RustCrypto's scalar SHA-256 supports the KDF without requiring SIMD context support. PSIV authenticates before releasing plaintext. The local source provenance and license are in `crypto/vendor/psiv/`.

Octave's confidentiality and acceptance claims retain their explicit assumptions: fresh independent randomness, authenticated links, enough honest delivered shares, and sound confirmation. The computational security of the PSIV/HKDF composition and the adapter is not proved here. The source PSIV package is experimental; no production-security or complete Lean/C equivalence claim is made. No whole-kernel, network-parser, DMA, constant-time, or end-to-end cryptographic proof is implied by the 35 policy theorems. Tests cover the stated scenarios only.

Protocol references: [UDP RFC 768](https://www.rfc-editor.org/rfc/rfc768), [IPv4 RFC 791](https://www.rfc-editor.org/rfc/rfc791), [ARP RFC 826](https://www.rfc-editor.org/rfc/rfc826), [virtio 1.2 legacy interface](https://docs.oasis-open.org/virtio/virtio/v1.2/virtio-v1.2.html), and [QEMU networking](https://www.qemu.org/docs/master/system/devices/net.html).
