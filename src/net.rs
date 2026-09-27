//! Allocation-free IPv4 TCP/UDP transport for trusted kernel services.
//! These handles are NOT process capabilities; a userspace ABI must wrap them.
use crate::decisions;
use smoltcp::{
    iface::{Config, Interface, PollIngressSingleResult, SocketSet},
    socket::Socket,
    wire::{EthernetAddress, IpCidr},
};
pub use smoltcp::{
    iface::{SocketHandle, SocketStorage},
    phy::{Device, DeviceCapabilities, Medium, RxToken, TxToken},
    socket::{tcp, udp},
    time::{Duration, Instant},
    wire::{IpAddress, IpEndpoint, Ipv4Address},
};

pub const MAX_PAYLOAD: usize = 1200;
pub const MAX_FRAME: usize = 1514;
pub const ECHO_PORT: u16 = 9000;
pub const MAX_SOCKETS: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidAddress,
    InvalidHandle,
    InvalidState,
    Full,
    WouldBlock,
    MessageTooLarge,
}

/// Lean admission runs before the Rust protocol parser. Checksums, TCP sequence
/// handling, ARP validation and destination matching are performed by smoltcp.
pub fn admit(frame: &[u8]) -> bool {
    if decisions::net_frame_len(frame.len() as u64) == 0 {
        return false;
    }
    match word(frame, 12) {
        0x0806 => frame.len() >= 42,
        0x0800 if frame.len() >= 34 => {
            let ip = &frame[14..];
            let size = word(ip, 2) as usize;
            if decisions::net_ipv4(
                size as u64,
                ip.len() as u64,
                word(ip, 6).into(),
                ip[8].into(),
                ip[9].into(),
                ip[0].into(),
            ) == 0
            {
                return false;
            }
            let segment = &ip[20..size];
            match ip[9] {
                17 if segment.len() >= 8 => {
                    decisions::net_udp(
                        word(segment, 4).into(),
                        segment.len() as u64,
                        word(segment, 2).into(),
                        word(segment, 2).into(),
                    ) != 0
                        && word(segment, 0) != 0
                }
                6 if segment.len() >= 20 => {
                    decisions::net_tcp(segment.len() as u64, u64::from(segment[12] >> 4) * 4) != 0
                }
                _ => false,
            }
        }
        _ => false,
    }
}
fn word(bytes: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([bytes[offset], bytes[offset + 1]])
}

fn unicast(ip: Ipv4Address) -> bool {
    ip.octets()[0] != 0 && ip.octets()[0] != 127 && ip.octets()[0] < 224
}

// Apply the same admission policy to every adapter, including host-side tests.
struct Admission<'a, D>(&'a mut D);
struct Admitted<R>(R);
impl<R: RxToken> RxToken for Admitted<R> {
    fn consume<T, F: FnOnce(&[u8]) -> T>(self, f: F) -> T {
        self.0
            .consume(|bytes| f(if admit(bytes) { bytes } else { &[] }))
    }
}
impl<D: Device> Device for Admission<'_, D> {
    type RxToken<'a>
        = Admitted<D::RxToken<'a>>
    where
        Self: 'a;
    type TxToken<'a>
        = D::TxToken<'a>
    where
        Self: 'a;
    fn receive(&mut self, now: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        self.0.receive(now).map(|(rx, tx)| (Admitted(rx), tx))
    }
    fn transmit(&mut self, now: Instant) -> Option<Self::TxToken<'_>> {
        self.0.transmit(now)
    }
    fn capabilities(&self) -> DeviceCapabilities {
        self.0.capabilities()
    }
}

/// Caller supplies all socket and packet storage. No allocator is enabled.
/// Socket handles are valid only in this stack.
pub struct Stack<'a> {
    interface: Interface,
    sockets: SocketSet<'a>,
    capacity: usize,
}
impl<'a> Stack<'a> {
    pub fn new(
        device: &mut impl Device,
        mac: [u8; 6],
        ip: [u8; 4],
        seed: u64,
        storage: &'a mut [SocketStorage<'a>],
    ) -> Result<Self, Error> {
        let address = Ipv4Address::from(ip);
        if mac == [0; 6]
            || mac[0] & 1 != 0
            || !unicast(address)
            || storage.is_empty()
            || storage.len() > MAX_SOCKETS
        {
            return Err(Error::InvalidAddress);
        }
        let mut config = Config::new(EthernetAddress(mac).into());
        config.random_seed = seed;
        let mut interface = Interface::new(config, device, Instant::from_millis(0));
        interface.update_ip_addrs(|addrs| {
            addrs.push(IpCidr::new(address.into(), 24)).unwrap();
        });
        Ok(Self {
            interface,
            capacity: storage.len(),
            sockets: SocketSet::new(storage),
        })
    }
    pub fn gateway(&mut self, address: [u8; 4]) -> Result<(), Error> {
        let address = Ipv4Address::from(address);
        if !unicast(address) {
            return Err(Error::InvalidAddress);
        }
        self.interface
            .routes_mut()
            .add_default_ipv4_route(address)
            .map_err(|_| Error::Full)?;
        Ok(())
    }
    pub fn add_udp(
        &mut self,
        rx: udp::PacketBuffer<'a>,
        tx: udp::PacketBuffer<'a>,
    ) -> Result<SocketHandle, Error> {
        if self.sockets.iter().count() == self.capacity {
            return Err(Error::Full);
        }
        Ok(self.sockets.add(udp::Socket::new(rx, tx)))
    }
    pub fn add_tcp(
        &mut self,
        rx: tcp::SocketBuffer<'a>,
        tx: tcp::SocketBuffer<'a>,
    ) -> Result<SocketHandle, Error> {
        if self.sockets.iter().count() == self.capacity {
            return Err(Error::Full);
        }
        let mut socket = tcp::Socket::new(rx, tx);
        socket.set_timeout(Some(Duration::from_secs(30)));
        Ok(self.sockets.add(socket))
    }
    fn udp(&mut self, handle: SocketHandle) -> Result<&mut udp::Socket<'a>, Error> {
        self.sockets
            .iter_mut()
            .find_map(|(h, socket)| match socket {
                Socket::Udp(s) if h == handle => Some(s),
                _ => None,
            })
            .ok_or(Error::InvalidHandle)
    }
    /// Trusted access to stream state, readiness and timeout configuration.
    pub fn tcp(&mut self, handle: SocketHandle) -> Result<&mut tcp::Socket<'a>, Error> {
        self.sockets
            .iter_mut()
            .find_map(|(h, socket)| match socket {
                Socket::Tcp(s) if h == handle => Some(s),
                _ => None,
            })
            .ok_or(Error::InvalidHandle)
    }
    pub fn udp_bind(&mut self, handle: SocketHandle, port: u16) -> Result<(), Error> {
        if decisions::net_port(port.into()) == 0 {
            return Err(Error::InvalidAddress);
        }
        if self
            .sockets
            .iter()
            .any(|(_, s)| matches!(s, Socket::Udp(s) if s.endpoint().port == port))
        {
            return Err(Error::InvalidState);
        }
        self.udp(handle)?
            .bind(port)
            .map_err(|_| Error::InvalidState)
    }
    pub fn udp_close(&mut self, handle: SocketHandle) -> Result<(), Error> {
        self.udp(handle)?.close();
        Ok(())
    }
    pub fn udp_send_to(
        &mut self,
        handle: SocketHandle,
        data: &[u8],
        remote: IpEndpoint,
    ) -> Result<(), Error> {
        if decisions::net_payload(data.len() as u64) == 0 {
            return Err(Error::MessageTooLarge);
        }
        if decisions::net_port(remote.port.into()) == 0 {
            return Err(Error::InvalidAddress);
        }
        self.udp(handle)?
            .send_slice(data, remote)
            .map_err(|e| match e {
                udp::SendError::BufferFull => Error::WouldBlock,
                udp::SendError::Unaddressable => Error::InvalidAddress,
            })
    }
    /// An undersized receive buffer discards the datagram, returning MessageTooLarge.
    pub fn udp_recv_from(
        &mut self,
        handle: SocketHandle,
        data: &mut [u8],
    ) -> Result<(usize, IpEndpoint), Error> {
        self.udp(handle)?
            .recv_slice(data)
            .map(|(n, meta)| (n, meta.endpoint))
            .map_err(|e| match e {
                udp::RecvError::Exhausted => Error::WouldBlock,
                udp::RecvError::Truncated => Error::MessageTooLarge,
            })
    }
    pub fn tcp_listen(&mut self, handle: SocketHandle, port: u16) -> Result<(), Error> {
        if decisions::net_port(port.into()) == 0 {
            return Err(Error::InvalidAddress);
        }
        self.tcp(handle)?
            .listen(port)
            .map_err(|_| Error::InvalidState)
    }
    pub fn tcp_connect(
        &mut self,
        handle: SocketHandle,
        remote: IpEndpoint,
        local: u16,
    ) -> Result<(), Error> {
        if decisions::net_port(remote.port.into()) == 0 || decisions::net_port(local.into()) == 0 {
            return Err(Error::InvalidAddress);
        }
        let context = self.interface.context();
        let socket = self
            .sockets
            .iter_mut()
            .find_map(|(h, s)| match s {
                Socket::Tcp(s) if h == handle => Some(s),
                _ => None,
            })
            .ok_or(Error::InvalidHandle)?;
        socket.connect(context, remote, local).map_err(|e| match e {
            tcp::ConnectError::InvalidState => Error::InvalidState,
            tcp::ConnectError::Unaddressable => Error::InvalidAddress,
        })
    }
    /// Partial writes are normal; zero progress on nonempty input means WouldBlock.
    pub fn tcp_send(&mut self, handle: SocketHandle, data: &[u8]) -> Result<usize, Error> {
        let n = self
            .tcp(handle)?
            .send_slice(data)
            .map_err(|_| Error::InvalidState)?;
        if n == 0 && !data.is_empty() {
            Err(Error::WouldBlock)
        } else {
            Ok(n)
        }
    }
    /// Returns zero at orderly EOF; WouldBlock means retry after polling.
    pub fn tcp_recv(&mut self, handle: SocketHandle, data: &mut [u8]) -> Result<usize, Error> {
        match self.tcp(handle)?.recv_slice(data) {
            Ok(0) if !data.is_empty() => Err(Error::WouldBlock),
            Ok(n) => Ok(n),
            Err(tcp::RecvError::Finished) => Ok(0),
            Err(tcp::RecvError::InvalidState) => Err(Error::InvalidState),
        }
    }
    pub fn tcp_close(&mut self, handle: SocketHandle) -> Result<(), Error> {
        self.tcp(handle)?.close();
        Ok(())
    }
    /// At most eight ingress attempts and one bounded egress pass per call.
    /// Device implementations must bound their own DMA work.
    pub fn poll(&mut self, now_ms: i64, device: &mut impl Device) {
        let mut device = Admission(device);
        let now = Instant::from_millis(now_ms);
        self.interface.poll_maintenance(now);
        let mut n = 0;
        while decisions::net_budget(n) != 0 {
            n += 1;
            if self
                .interface
                .poll_ingress_single(now, &mut device, &mut self.sockets)
                == PollIngressSingleResult::None
            {
                break;
            }
        }
        self.interface
            .poll_egress(now, &mut device, &mut self.sockets);
    }
}
