//! Legacy/transitional virtio-net PCI driver. Single CPU, polling at PIT ticks.
//! All DMA storage is static, supervisor-owned and physically contiguous.
use crate::physical;
use bastion_core::{
    decisions,
    net::{
        self, DeviceCapabilities, ECHO_PORT, Instant, MAX_FRAME, MAX_PAYLOAD, Medium, RxToken,
        SocketHandle, SocketStorage, Stack, TxToken, tcp, udp,
    },
};
use core::{
    arch::asm,
    ptr::{read_volatile as read, write_volatile as write},
    sync::atomic::{Ordering, fence},
};
const MAXQ: usize = 256;
const BUFFER: usize = 1536;
#[repr(C, align(4096))]
struct QueueMem([u8; 12288]);
#[repr(C, align(4096))]
struct Buffers([[u8; BUFFER]; MAXQ]);
static mut RXQ: QueueMem = QueueMem([0; 12288]);
static mut TXQ: QueueMem = QueueMem([0; 12288]);
static mut RX: Buffers = Buffers([[0; BUFFER]; MAXQ]);
static mut TX: Buffers = Buffers([[0; BUFFER]; MAXQ]);
static mut DEVICE: Option<Driver> = None;
static mut STACK: Option<Stack<'static>> = None;
static mut HANDLES: Option<(SocketHandle, SocketHandle)> = None;
static mut SOCKETS: [SocketStorage<'static>; 2] = [const { SocketStorage::EMPTY }; 2];
static mut UDP_RX_META: [udp::PacketMetadata; 4] = [udp::PacketMetadata::EMPTY; 4];
static mut UDP_TX_META: [udp::PacketMetadata; 4] = [udp::PacketMetadata::EMPTY; 4];
static mut UDP_RX: [u8; MAX_PAYLOAD * 4] = [0; MAX_PAYLOAD * 4];
static mut UDP_TX: [u8; MAX_PAYLOAD * 4] = [0; MAX_PAYLOAD * 4];
static mut TCP_RX: [u8; 4096] = [0; 4096];
static mut TCP_TX: [u8; 4096] = [0; 4096];

struct Queue {
    mem: *mut u8,
    buffers: *mut u8,
    n: usize,
    used_offset: usize,
    seen: u16,
    avail: u16,
    posted: [bool; MAXQ],
}
struct Driver {
    io: u16,
    rx: Queue,
    tx: Queue,
    received: u64,
    transmitted: u64,
    failed: bool,
}

unsafe fn out16(port: u16, value: u16) {
    asm!("out dx, ax",in("dx")port,in("ax")value,options(nomem,nostack));
}
unsafe fn out32(port: u16, value: u32) {
    asm!("out dx, eax",in("dx")port,in("eax")value,options(nomem,nostack));
}
unsafe fn in16(port: u16) -> u16 {
    let v;
    asm!("in ax, dx",in("dx")port,out("ax")v,options(nomem,nostack));
    v
}
unsafe fn in32(port: u16) -> u32 {
    let v;
    asm!("in eax, dx",in("dx")port,out("eax")v,options(nomem,nostack));
    v
}
unsafe fn pci(addr: u32, offset: u32) -> u32 {
    out32(0xcf8, addr | offset);
    in32(0xcfc)
}
unsafe fn find() -> Option<u16> {
    for bus in 0..256u32 {
        for slot in 0..32u32 {
            let base = 0x80000000 | (bus << 16) | (slot << 11);
            if pci(base, 0) & 65535 == 65535 {
                continue;
            }
            let functions = if pci(base, 12) & 0x00800000 != 0 {
                8
            } else {
                1
            };
            for function in 0..functions {
                let addr = base | (function << 8);
                if pci(addr, 0) != 0x10001af4 || pci(addr, 8) & 255 != 0 || pci(addr, 44) >> 16 != 1
                {
                    continue;
                }
                let bar = pci(addr, 16);
                if bar & 1 == 0 || bar & !3 > 0xffc0 || bar & !3 == 0 {
                    continue;
                }
                let command = pci(addr, 4) as u16;
                out32(0xcf8, addr | 4);
                out16(0xcfc, command | 0x405); // I/O, bus master; disable PCI INTx.
                return Some((bar & !3) as u16);
            }
        }
    }
    None
}
impl Queue {
    unsafe fn new(io: u16, index: u16, mem: *mut u8, buffers: *mut u8) -> Option<Self> {
        out16(io + 14, index);
        let n = in16(io + 12) as usize;
        if n == 0 || n > MAXQ || !n.is_power_of_two() || in32(io + 8) != 0 {
            return None;
        }
        let used_offset = (16 * n + 6 + 2 * n + 4095) & !4095;
        let address = physical(mem as u64);
        if address & 4095 != 0 || address >> 12 > u32::MAX as u64 {
            return None;
        }
        write(mem.add(16 * n).cast::<u16>(), 1); // suppress receive/transmit completion IRQs
        out32(io + 8, (address >> 12) as u32);
        Some(Self {
            mem,
            buffers,
            n,
            used_offset,
            seen: 0,
            avail: 0,
            posted: [false; MAXQ],
        })
    }
    unsafe fn post(&mut self, id: usize, length: usize, writable: bool) {
        let desc = self.mem.add(16 * id);
        write(
            desc.cast::<u64>(),
            physical(self.buffers.add(id * BUFFER) as u64),
        );
        write(desc.add(8).cast::<u32>(), length as u32);
        write(desc.add(12).cast::<u16>(), if writable { 2 } else { 0 });
        write(desc.add(14).cast::<u16>(), 0);
        write(
            self.mem
                .add(16 * self.n + 4 + 2 * (self.avail as usize % self.n))
                .cast::<u16>(),
            id as u16,
        );
        self.posted[id] = true;
        self.avail = self.avail.wrapping_add(1);
        fence(Ordering::SeqCst);
        write(self.mem.add(16 * self.n + 2).cast::<u16>(), self.avail);
    }
    unsafe fn take(&mut self) -> Result<Option<(usize, usize)>, ()> {
        let next = read(self.mem.add(self.used_offset + 2).cast::<u16>());
        if next.wrapping_sub(self.seen) as usize > self.n {
            return Err(());
        }
        if next == self.seen {
            return Ok(None);
        }
        fence(Ordering::SeqCst);
        let entry = self
            .mem
            .add(self.used_offset + 4 + 8 * (self.seen as usize % self.n));
        let id = read(entry.cast::<u32>()) as usize;
        let len = read(entry.add(4).cast::<u32>()) as usize;
        if id >= self.n || !self.posted[id] || len > BUFFER {
            return Err(());
        }
        self.posted[id] = false;
        self.seen = self.seen.wrapping_add(1);
        Ok(Some((id, len)))
    }
}
// TCP initial sequence numbers need a fresh seed. Refuse networking if the
// virtual CPU cannot provide hardware randomness; never fall back to a constant.
unsafe fn random_seed() -> Option<u64> {
    if core::arch::x86_64::__cpuid(1).ecx & (1 << 30) == 0 {
        return None;
    }
    for _ in 0..32 {
        let value: u64;
        let ok: u8;
        asm!("rdrand {}", "setc {}", out(reg) value, out(reg_byte) ok, options(nomem, nostack));
        if ok != 0 {
            return Some(value);
        }
    }
    None
}
pub unsafe fn init() {
    let Some(io) = find() else {
        crate::console::print(format_args!("NETWORK: NO SUPPORTED VIRTIO NIC\n"));
        return;
    };
    crate::out(io + 18, 0);
    let mut reset = false;
    for _ in 0..1000 {
        if crate::input(io + 18) == 0 {
            reset = true;
            break;
        }
    }
    if !reset {
        return;
    }
    crate::out(io + 18, 1);
    crate::out(io + 18, 3);
    if in32(io) & (1 << 5) == 0 {
        crate::out(io + 18, 0x83);
        return;
    }
    out32(io + 4, 1 << 5); // MAC only: no checksum/segmentation/mergeable-buffer offloads.
    let rx = Queue::new(io, 0, (&raw mut RXQ.0).cast(), (&raw mut RX.0).cast());
    let tx = Queue::new(io, 1, (&raw mut TXQ.0).cast(), (&raw mut TX.0).cast());
    let (Some(mut rx), Some(tx)) = (rx, tx) else {
        crate::out(io + 18, 0x83);
        return;
    };
    let mut mac = [0; 6];
    for (i, b) in mac.iter_mut().enumerate() {
        *b = crate::input(io + 20 + i as u16);
    }
    if mac[0] & 1 != 0 || mac == [0; 6] {
        crate::out(io + 18, 0x83);
        return;
    }
    for id in 0..rx.n {
        rx.post(id, BUFFER, true);
    }
    let Some(seed) = random_seed() else {
        crate::out(io + 18, 0x83);
        crate::console::print(format_args!("NETWORK: RANDOM SEED UNAVAILABLE\n"));
        return;
    };
    let mut driver = Driver {
        io,
        rx,
        tx,
        received: 0,
        transmitted: 0,
        failed: false,
    };
    let mut stack = Stack::new(&mut driver, mac, [10, 0, 2, 15], seed, &mut SOCKETS[..]).unwrap();
    stack.gateway([10, 0, 2, 2]).unwrap();
    let udp = stack
        .add_udp(
            udp::PacketBuffer::new(&mut UDP_RX_META[..], &mut UDP_RX[..]),
            udp::PacketBuffer::new(&mut UDP_TX_META[..], &mut UDP_TX[..]),
        )
        .unwrap();
    let tcp = stack
        .add_tcp(
            tcp::SocketBuffer::new(&mut TCP_RX[..]),
            tcp::SocketBuffer::new(&mut TCP_TX[..]),
        )
        .unwrap();
    stack.udp_bind(udp, ECHO_PORT).unwrap();
    stack.tcp_listen(tcp, ECHO_PORT).unwrap();
    HANDLES = Some((udp, tcp));
    STACK = Some(stack);
    DEVICE = Some(driver);
    crate::out(io + 18, 7);
    out16(io + 16, 0);
    crate::console::print(format_args!("TCP / UDP READY 10.0.2.15:9000\n"));
}
pub unsafe fn poll(now: u64) {
    let (Some(driver), Some(stack), Some((udp, tcp))) = (DEVICE.as_mut(), STACK.as_mut(), HANDLES)
    else {
        return;
    };
    driver.received = 0;
    driver.transmitted = 0;
    for _ in 0..driver.tx.n {
        match driver.tx.take() {
            Ok(Some(_)) => {}
            Ok(None) => break,
            Err(()) => {
                driver.failed = true;
                break;
            }
        }
    }
    let now = i64::try_from(now).unwrap_or(i64::MAX);
    stack.poll(now, driver);
    // Trusted echo services exercise the transport; no ring-3 socket ABI yet.
    let mut buffer = [0; MAX_PAYLOAD];
    for _ in 0..4 {
        let Ok((n, remote)) = stack.udp_recv_from(udp, &mut buffer) else {
            break;
        };
        let _ = stack.udp_send_to(udp, &buffer[..n], remote);
    }
    let socket = stack.tcp(tcp).unwrap();
    if !socket.is_open() {
        stack.tcp_listen(tcp, ECHO_PORT).unwrap();
    } else {
        // Consume no more bytes than can be queued for echo, preserving stream
        // bytes under backpressure. FIN follows all buffered replies.
        let space = socket.send_capacity() - socket.send_queue();
        let n = buffer.len().min(space);
        if socket.can_recv()
            && n > 0
            && socket.may_send()
            && let Ok(n) = socket.recv_slice(&mut buffer[..n])
        {
            assert_eq!(socket.send_slice(&buffer[..n]), Ok(n));
        }
        if socket.state() == tcp::State::CloseWait && !socket.can_recv() {
            socket.close();
        }
    }
    stack.poll(now, driver);
    if driver.received > 0 {
        out16(driver.io + 16, 0);
    }
    let _ = crate::input(driver.io + 19);
    if driver.failed {
        crate::out(driver.io + 18, 0);
        DEVICE = None;
        crate::console::print(format_args!(
            "NETWORK DISABLED: INVALID DEVICE COMPLETION\n"
        ));
    }
}
struct Receive {
    bytes: [u8; MAX_FRAME],
    len: usize,
}
impl RxToken for Receive {
    fn consume<R, F: FnOnce(&[u8]) -> R>(self, f: F) -> R {
        f(&self.bytes[..self.len])
    }
}
struct Transmit<'a> {
    driver: &'a mut Driver,
    id: usize,
}
impl TxToken for Transmit<'_> {
    fn consume<R, F: FnOnce(&mut [u8]) -> R>(self, len: usize, f: F) -> R {
        let mut frame = [0; MAX_FRAME];
        let result = f(&mut frame[..len]);
        unsafe {
            let target = self.driver.tx.buffers.add(self.id * BUFFER);
            for i in 0..10 {
                write(target.add(i), 0);
            }
            for (i, b) in frame[..len].iter().enumerate() {
                write(target.add(10 + i), *b);
            }
            self.driver.tx.post(self.id, len + 10, false);
            fence(Ordering::SeqCst);
            out16(self.driver.io + 16, 1);
        }
        result
    }
}
impl net::Device for Driver {
    type RxToken<'a> = Receive;
    type TxToken<'a> = Transmit<'a>;
    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.medium = Medium::Ethernet;
        caps.max_transmission_unit = MAX_FRAME;
        caps.max_burst_size = Some(8);
        caps
    }
    fn receive(&mut self, _: Instant) -> Option<(Receive, Transmit<'_>)> {
        if self.failed || decisions::net_budget(self.transmitted) == 0 {
            return None;
        }
        let tx = self.tx.posted[..self.tx.n].iter().position(|busy| !*busy)?;
        while decisions::net_budget(self.received) != 0 {
            let completion = unsafe { self.rx.take() };
            let (id, len) = match completion {
                Ok(Some(v)) => v,
                Ok(None) => return None,
                Err(()) => {
                    self.failed = true;
                    return None;
                }
            };
            self.received += 1;
            let mut packet = Receive {
                bytes: [0; MAX_FRAME],
                len: 0,
            };
            unsafe {
                let source = self.rx.buffers.add(id * BUFFER);
                if (24..=MAX_FRAME + 10).contains(&len)
                    && read(source) == 0
                    && read(source.add(1)) == 0
                {
                    packet.len = len - 10;
                    for (i, b) in packet.bytes[..packet.len].iter_mut().enumerate() {
                        *b = read(source.add(10 + i));
                    }
                }
                self.rx.post(id, BUFFER, true);
            }
            if packet.len > 0 {
                self.transmitted += 1;
                return Some((
                    packet,
                    Transmit {
                        driver: self,
                        id: tx,
                    },
                ));
            }
        }
        None
    }
    fn transmit(&mut self, _: Instant) -> Option<Transmit<'_>> {
        if self.failed || decisions::net_budget(self.transmitted) == 0 {
            return None;
        }
        let id = self.tx.posted[..self.tx.n].iter().position(|busy| !*busy)?;
        self.transmitted += 1;
        Some(Transmit { driver: self, id })
    }
}

pub struct Status {
    pub tcp: tcp::State,
    pub received_bytes: usize,
    pub queued_bytes: usize,
}
/// Called by the console on the boot CPU, with interrupts masked.
pub unsafe fn status() -> Option<Status> {
    DEVICE.as_ref()?;
    let (_, handle) = HANDLES?;
    let socket = STACK.as_mut()?.tcp(handle).ok()?;
    Some(Status {
        tcp: socket.state(),
        received_bytes: socket.recv_queue(),
        queued_bytes: socket.send_queue(),
    })
}
