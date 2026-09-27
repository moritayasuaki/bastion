# TCP and UDP

Bastion has an allocation-free IPv4 transport built on pinned `smoltcp` 0.14.0.
Lean compiles admission and bounds decisions to C. Rust implements packet handling,
TCP state, retransmission, buffering, and the virtio driver. These are trusted
kernel components; the protocol implementation is not formally verified.

## Run the network checks

```sh
cargo xtask build
cargo xtask network-test
```

The test boots one native Bastion guest, waits for its process-isolation checks,
then exchanges UDP datagrams and TCP streams through loopback-only QEMU forwards.
It tests zero/max-size UDP, oversized rejection, 525 exchanges to reuse descriptor
rings, TCP data larger than the buffers, FIN with pending data, EOF and reconnection.
The guest stops on completion, failure, SIGINT or SIGTERM. SIGKILL cannot run cleanup.
Logs stay under ignored `target/bastion/network-test/`.

UEFI example on macOS (firmware paths differ on Linux):

```sh
cargo xtask network-test \
  --uefi /opt/homebrew/share/qemu/edk2-x86_64-code.fd \
  --uefi-vars /opt/homebrew/share/qemu/edk2-i386-vars.fd
```

`--iso PATH` selects a release ISO; `--hold-seconds N` keeps the guest alive after
successful tests. The test ISO intentionally exits QEMU and cannot serve traffic.
BIOS uses 64 MB; UEFI uses 128 MB.

Manual boot:

```sh
qemu-system-x86_64 -machine pc -accel tcg -cpu max -m 64M -smp 1 \
  -cdrom dist/bastion.iso -boot d -display none -monitor none -serial stdio \
  -netdev user,id=n,hostfwd=udp:127.0.0.1:19000-:9000,hostfwd=tcp:127.0.0.1:19000-:9000 \
  -device virtio-net-pci,netdev=n,disable-modern=on -no-reboot
```

Both protocols expose a kernel echo service on guest port 9000. TCP handles one
connection at a time and listens again after it closes; this is a transport demo,
not a concurrent application server. There is no service on port 9001.

## Kernel API

`bastion_core::net::Stack` takes caller-owned socket storage and a NIC implementing
`Device`. It supports up to eight sockets, with no heap. The caller supplies buffers
when adding sockets; the boot demo uses one TCP and one UDP socket.

| Operation | Function / behavior |
|---|---|
| Configure | `Stack::new` (IPv4 /24), `gateway` |
| Allocate | `add_udp`, `add_tcp`; returns `Full` when socket storage is exhausted |
| UDP | `udp_bind`, `udp_send_to`, `udp_recv_from`, `udp_close` |
| TCP | `tcp_listen`, `tcp_connect`, `tcp_send`, `tcp_recv`, `tcp_close` |
| Readiness | Trusted `tcp(handle)` access to `can_recv`, `can_send`, connection state |
| Progress | `poll` advances input, output, retransmissions and timeouts |

All calls are nonblocking. TCP writes can be partial; `WouldBlock` means poll and
retry. A TCP read returns zero at orderly EOF. Connect starts the handshake; inspect
state before sending. `tcp_close` sends FIN after queued output. UDP preserves
message boundaries, including empty datagrams. An undersized receive buffer discards
the message and returns `MessageTooLarge`. A full transmit buffer returns
`WouldBlock`. Unknown/wrong-type handles return `InvalidHandle`.

Handles belong to a specific stack and are trusted kernel identifiers. They must
not be exposed as process capabilities. No socket removal/reallocation API or
userspace socket syscall exists yet. `tcp()` exposes the underlying trusted socket
for configuration; this is not an untrusted API. See [USERSPACE.md](USERSPACE.md) for
the proposed process boundary.

## Bounds and supported profile

| Item | Current implementation |
|---|---|
| NIC | Legacy/transitional PCI virtio-net; MAC feature only, no offloads |
| Guest address | `10.0.2.15/24`, gateway `10.0.2.2` |
| Network | Ethernet, ARP neighbor discovery/cache, IPv4, TCP, UDP |
| TCP | Active/passive open, checksums, sequence/ACK handling, retransmission, flow control, FIN/RST |
| UDP payload | 0–1200 bytes; nonzero checksums verified, IPv4 checksum-zero accepted |
| Ethernet frame | At most 1514 bytes; no VLAN or jumbo frame support |
| IP ingress | No options or fragmentation; only TCP/UDP admitted |
| TCP ingress | Header 20–60 bytes within a segment of at most 1480 bytes |
| Boot socket buffers | TCP 4096 bytes each direction; UDP four records and 4800 bytes each direction |
| Polling | At most 8 DMA receive completions and 8 transmit-token reservations per timer tick |
| TCP timeout | 30-second smoltcp socket timeout; trusted callers can configure it |
| DMA | Static supervisor memory; descriptor counts up to 256; invalid completions disable NIC |
| Randomness | Boot requires CPU RDRAND for a fresh stack seed; no constant fallback |

`Stack::poll` applies Lean admission to every adapter and takes at most eight
single-ingress steps plus one bounded egress pass. The boot driver shares its work
budget across the two polls around service processing. Queues are fixed and
backpressure/drops never allocate. This limits work, but is not a complete network
DoS defense. TCP input validation and state transitions remain smoltcp's responsibility.

The kernel services consume static kernel memory and interrupt time, not process
network quotas. DHCP, DNS, IPv6, TLS, SSH, modern-only virtio, configurable provider
networking and a userspace ABI are absent. Ordinary TCP/UDP does not authenticate or
encrypt peers. A working QEMU test does not establish VPS compatibility.

References: [smoltcp](https://docs.rs/smoltcp/0.14.0/smoltcp/),
[TCP RFC 9293](https://www.rfc-editor.org/rfc/rfc9293),
[UDP RFC 768](https://www.rfc-editor.org/rfc/rfc768), and
[virtio 1.2](https://docs.oasis-open.org/virtio/virtio/v1.2/virtio-v1.2.html).
