//! Legacy/transitional virtio-net PCI driver. Single CPU, polling at PIT ticks.
//! All DMA storage is static, supervisor-owned and physically contiguous.
use crate::{Request, physical};
use bastion_core::{
    decisions,
    net::{ECHO_PORT, MAX_FRAME, Stack},
    relay::{Config, Relay},
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
static mut DEVICE: Option<Device> = None;
#[used]
#[unsafe(link_section = ".requests")]
static mut MODULES: Request = Request::new(0x3e7e279702be32af, 0xca1c4f3bd1280cee);
struct Queue {
    mem: *mut u8,
    buffers: *mut u8,
    n: usize,
    used_offset: usize,
    seen: u16,
    avail: u16,
    posted: [bool; MAXQ],
}
struct Device {
    io: u16,
    rx: Queue,
    tx: Queue,
    stack: Stack,
    relay: Option<Relay>,
    packets: u64,
    drops: u64,
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
unsafe fn config() -> Option<Config> {
    let response = MODULES.response;
    if response.is_null() || *response.add(1) != 1 {
        return None;
    }
    let files = *response.add(2) as *const *const u64;
    if files.is_null() {
        return None;
    }
    let file = *files;
    if file.is_null() {
        return None;
    }
    let size = *file.add(2);
    if size != bastion_core::relay::CONFIG_SIZE as u64 {
        return None;
    }
    let data = *file.add(1) as *const u8;
    if data.is_null() {
        return None;
    }
    Config::decode(core::slice::from_raw_parts(data, size as usize))
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
    let configuration = config();
    if let Some(c) = &configuration {
        crate::console::print(format_args!(
            "OCTAVE RELAY {} / AUTHENTICATED PSIV LINKS\n",
            c.id
        ));
    }
    DEVICE = Some(Device {
        io,
        rx,
        tx,
        stack: Stack {
            mac,
            ip: [10, 0, 2, 15],
        },
        relay: configuration.map(Relay::new),
        packets: 0,
        drops: 0,
    });
    crate::out(io + 18, 7);
    out16(io + 16, 0);
    crate::console::print(format_args!("UDP READY 10.0.2.15:9000 / RELAY PORT 9001\n"));
}
pub unsafe fn poll(now: u64) {
    let Some(device) = DEVICE.as_mut() else {
        return;
    };
    if device.poll(now).is_err() {
        crate::out(device.io + 18, 0);
        DEVICE = None;
        crate::console::print(format_args!(
            "NETWORK DISABLED: INVALID DEVICE COMPLETION\n"
        ));
    }
}
impl Device {
    unsafe fn poll(&mut self, now: u64) -> Result<(), ()> {
        for _ in 0..self.tx.n {
            if self.tx.take()?.is_none() {
                break;
            }
        }
        let mut consumed = 0;
        while decisions::net_budget(consumed) != 0 {
            let Some((id, len)) = self.rx.take()? else {
                break;
            };
            consumed += 1;
            let buffer = self.rx.buffers.add(id * BUFFER);
            let mut frame = [0; MAX_FRAME];
            let mut output = [0; MAX_FRAME];
            if (24..=MAX_FRAME + 10).contains(&len) && read(buffer) == 0 && read(buffer.add(1)) == 0
            {
                for (i, b) in frame[..len - 10].iter_mut().enumerate() {
                    *b = read(buffer.add(10 + i));
                }
                let relay = &mut self.relay;
                let response =
                    self.stack
                        .receive(&frame[..len - 10], &mut output, |port, input, out| {
                            if port == ECHO_PORT {
                                out[..input.len()].copy_from_slice(input);
                                Some(input.len())
                            } else {
                                relay.as_mut()?.handle(input, now, out)
                            }
                        });
                if let Some(n) = response {
                    if let Some(tx) = self.tx.posted[..self.tx.n].iter().position(|busy| !*busy) {
                        let target = self.tx.buffers.add(tx * BUFFER);
                        for i in 0..10 {
                            write(target.add(i), 0);
                        }
                        for (i, b) in output[..n].iter().enumerate() {
                            write(target.add(10 + i), *b);
                        }
                        self.tx.post(tx, n + 10, false);
                        fence(Ordering::SeqCst);
                        out16(self.io + 16, 1);
                        if output[12..14] == [8, 0] {
                            self.packets = self.packets.saturating_add(1);
                        }
                        if self.packets == 1 && output[12..14] == [8, 0] {
                            crate::console::print(format_args!("PASS UDP REQUEST / RESPONSE\n"));
                        }
                    } else {
                        self.drops = self.drops.saturating_add(1);
                    }
                }
            }
            self.rx.post(id, BUFFER, true);
        }
        if consumed > 0 {
            fence(Ordering::SeqCst);
            out16(self.io + 16, 0);
        }
        let _ = crate::input(self.io + 19);
        Ok(())
    }
}
