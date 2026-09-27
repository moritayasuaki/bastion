# Interface for user programs and administrators

The TCP/UDP transport currently serves trusted kernel code. Embedded ring-3 programs
still use the existing `int 0x80` demonstration ABI (operations 0–5). They cannot call
sockets, load executables, read a console, or access a filesystem. This document is
an implementation plan, not a claim that the following interfaces are available.

## First usable userspace boundary

Build a small versioned syscall ABI with a `no_std` Rust userspace library. Keep
Lean responsible for authorization, bounds, resource reservation and transitions;
keep Rust responsible for copying bytes, driving devices and executing approved
changes. Keep the scalar Lean → C interface so none of these policies require a
Lean heap in the kernel.

| Area | Required interface |
|---|---|
| Processes | `self_info`, `yield`, `exit`, capability-authorized inspect/terminate |
| Console | Bounded `console_read` / `console_write`, initially a serial terminal |
| Memory | Charged page mapping/unmapping, RX vs RW/NX, validated user-buffer copies |
| Sockets | `socket`, `bind`, `listen`, `accept`, `connect`, `send`, `recv`, `send_to`, `recv_from`, `shutdown`, `close` |
| Events | `poll` / wait with readiness flags and a monotonic timeout |
| Execution | Validated static ELF loader, initial process, stack and launch capabilities |

Start with nonblocking sockets and a small fixed event set. TCP accept should return
a new connection handle while retaining a listener, backed by a bounded connection
pool. The current single TCP demo socket does not implement that accept model.
Readiness must distinguish readable data, writable capacity, successful/failed
connect, peer shutdown and errors. On exit or fault, close all owned sockets, cancel
waits and release reservations. A loader can initially use boot modules, so a disk
filesystem is not necessary for the first application.

## Security contract

1. Derive the caller from the scheduler. Never accept a PID as proof of ownership.
2. Allocate fresh process-local handles, with a generation or monotonic identity so
   a stale handle cannot refer to a new socket. Never pass through a smoltcp handle.
3. Grant explicit network rights and allowed local/remote endpoints at launch.
   A port number or IP address is a name, not authority. Deny raw NIC access by default.
4. Reserve socket slots, receive/transmit bytes, listeners, connections and work
   budget before creating state. Charge descendants/services to explicit limits.
   Keep a global cap as well as each process's cap; roll back failed setup atomically.
5. Check pointer+length without overflow against the caller's actual mappings and
   permissions. Copy through bounded kernel buffers; never retain user pointers in
   asynchronous operations. The current private data page is only 4096 bytes.
6. On rejected requests preserve ownership and quota state. Scrub released buffers
   before reuse by another process. Do not amplify rights through duplication/transfer.
7. Specify EOF, partial writes, datagram truncation, cancellation and timeout behavior
   in the ABI. Provide stable errors such as `BadHandle`, `Denied`, `BadAddress`,
   `QuotaExceeded`, `WouldBlock`, `MessageTooLarge`, `ConnectionReset`, and `TimedOut`.

Use fixed-width integer layouts for ABI request/response records, explicit lengths,
reserved-zero fields, and a version query. IPv4 addresses should use four network-order
bytes and ports two network-order bytes; integer result codes can follow the x86-64
ABI. Do not freeze syscall numbers until buffer-copy and capability contracts have
regression tests. The kernel currently returns all-ones for unsupported calls; an
expanded ABI needs explicit, documented errors.

Useful Lean obligations include: successful copies stay within the supplied mapped
range; caller mismatch always denies; socket reservations conserve global and process
usage; failure preserves state; release cannot underflow; restricted capabilities
cannot gain rights; stale handles never authorize a replacement socket.

## Human interface and practical order

The first administration interface should be a serial console with a small userspace
shell: `help`, process/limit inspection, `net status`, and program launch. This works
through a VPS provider's console before remote access is available. The kernel has
console output today; input and an interactive shell still need implementation.

Implement in this order: validated user copies and stable ABI → console and loader →
process-owned socket handles and quotas → readiness/wait → a small network application.
Then add provider address/gateway configuration and required NIC support. SSH requires
an authenticated, encrypted userspace service and secure key provisioning; adding TCP
alone does not provide remote administration. A GUI is unnecessary for this VPS goal.
