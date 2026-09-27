use bastion_core::net::*;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Default)]
struct Link {
    frames: VecDeque<Vec<u8>>,
    drop_data: bool,
    lost: usize,
}
struct Wire {
    rx: Rc<RefCell<Link>>,
    tx: Rc<RefCell<Link>>,
}
struct Received(Vec<u8>);
struct Sent(Rc<RefCell<Link>>);
impl RxToken for Received {
    fn consume<R, F: FnOnce(&[u8]) -> R>(self, f: F) -> R {
        f(&self.0)
    }
}
impl TxToken for Sent {
    fn consume<R, F: FnOnce(&mut [u8]) -> R>(self, n: usize, f: F) -> R {
        let mut bytes = vec![0; n];
        let result = f(&mut bytes);
        let mut link = self.0.borrow_mut();
        let tcp_data = bytes.len() >= 54 && bytes[23] == 6 && bytes[47] & 8 != 0;
        if link.drop_data && tcp_data {
            link.drop_data = false;
            link.lost += 1;
        } else {
            link.frames.push_back(bytes);
        }
        result
    }
}
impl Device for Wire {
    type RxToken<'a> = Received;
    type TxToken<'a> = Sent;
    fn capabilities(&self) -> DeviceCapabilities {
        let mut c = DeviceCapabilities::default();
        c.medium = Medium::Ethernet;
        c.max_transmission_unit = MAX_FRAME;
        c
    }
    fn receive(&mut self, _: Instant) -> Option<(Received, Sent)> {
        // A rejected frame ends this ingress pass, so malformed input is bounded.
        let bytes = self.rx.borrow_mut().frames.pop_front()?;
        Some((Received(bytes), Sent(self.tx.clone())))
    }
    fn transmit(&mut self, _: Instant) -> Option<Sent> {
        Some(Sent(self.tx.clone()))
    }
}
fn wires() -> (Wire, Wire) {
    let a = Rc::new(RefCell::new(Link::default()));
    let b = Rc::new(RefCell::new(Link::default()));
    (
        Wire {
            rx: a.clone(),
            tx: b.clone(),
        },
        Wire { rx: b, tx: a },
    )
}
struct Buffers {
    ur: [u8; 2400],
    ut: [u8; 2400],
    mr: [udp::PacketMetadata; 2],
    mt: [udp::PacketMetadata; 2],
    tr: [u8; 256],
    tt: [u8; 256],
}
impl Buffers {
    fn new() -> Self {
        Self {
            ur: [0; 2400],
            ut: [0; 2400],
            mr: [udp::PacketMetadata::EMPTY; 2],
            mt: [udp::PacketMetadata::EMPTY; 2],
            tr: [0; 256],
            tt: [0; 256],
        }
    }
    fn stack<'a>(
        &'a mut self,
        wire: &mut Wire,
        n: u8,
        storage: &'a mut [SocketStorage<'a>],
    ) -> (Stack<'a>, SocketHandle, SocketHandle) {
        let mut s = Stack::new(wire, [2, 0, 0, 0, 0, n], [10, 0, 2, n], n.into(), storage).unwrap();
        let u = s
            .add_udp(
                udp::PacketBuffer::new(&mut self.mr[..], &mut self.ur[..]),
                udp::PacketBuffer::new(&mut self.mt[..], &mut self.ut[..]),
            )
            .unwrap();
        let t = s
            .add_tcp(
                tcp::SocketBuffer::new(&mut self.tr[..]),
                tcp::SocketBuffer::new(&mut self.tt[..]),
            )
            .unwrap();
        (s, u, t)
    }
}
fn endpoint(n: u8) -> IpEndpoint {
    (Ipv4Address::new(10, 0, 2, n), ECHO_PORT).into()
}

#[test]
fn udp_bidirectional_limits_backpressure_and_truncation() {
    let (mut a, mut b) = wires();
    let (mut ab, mut bb) = (Buffers::new(), Buffers::new());
    let mut xs = [const { SocketStorage::EMPTY }; 2];
    let mut ys = [const { SocketStorage::EMPTY }; 2];
    let (mut x, u, t) = ab.stack(&mut a, 15, &mut xs);
    let (mut y, v, _) = bb.stack(&mut b, 16, &mut ys);
    assert_eq!(x.udp_bind(t, ECHO_PORT), Err(Error::InvalidHandle));
    assert_eq!(x.udp_bind(u, 0), Err(Error::InvalidAddress));
    x.udp_bind(u, ECHO_PORT).unwrap();
    y.udp_bind(v, ECHO_PORT).unwrap();
    assert_eq!(
        x.udp_send_to(u, &[0; 1201], endpoint(16)),
        Err(Error::MessageTooLarge)
    );
    x.udp_send_to(u, &[1; 1200], endpoint(16)).unwrap();
    x.udp_send_to(u, &[], endpoint(16)).unwrap();
    assert_eq!(x.udp_send_to(u, &[2], endpoint(16)), Err(Error::WouldBlock));
    for now in 0..10 {
        x.poll(now, &mut a);
        y.poll(now, &mut b);
    }
    let mut buf = [0; 1200];
    assert_eq!(y.udp_recv_from(v, &mut buf), Ok((1200, endpoint(15))));
    assert_eq!(buf, [1; 1200]);
    assert_eq!(y.udp_recv_from(v, &mut buf), Ok((0, endpoint(15))));
    assert_eq!(y.udp_recv_from(v, &mut buf), Err(Error::WouldBlock));
    y.udp_send_to(v, b"response", endpoint(15)).unwrap();
    for now in 10..20 {
        x.poll(now, &mut a);
        y.poll(now, &mut b);
    }
    assert_eq!(x.udp_recv_from(u, &mut [0; 2]), Err(Error::MessageTooLarge));
    assert_eq!(x.udp_recv_from(u, &mut buf), Err(Error::WouldBlock));
    x.udp_close(u).unwrap();
    assert_eq!(
        x.udp_send_to(u, b"closed", endpoint(16)),
        Err(Error::InvalidAddress)
    );
    x.udp_bind(u, ECHO_PORT).unwrap();
}

#[test]
fn tcp_connect_retransmit_flow_control_half_close_and_relisten() {
    let (mut a, mut b) = wires();
    let (mut ab, mut bb) = (Buffers::new(), Buffers::new());
    let mut xs = [const { SocketStorage::EMPTY }; 2];
    let mut ys = [const { SocketStorage::EMPTY }; 2];
    let (mut x, _, t) = ab.stack(&mut a, 15, &mut xs);
    let (mut y, _, v) = bb.stack(&mut b, 16, &mut ys);
    y.tcp_listen(v, ECHO_PORT).unwrap();
    x.tcp_connect(t, endpoint(16), 50000).unwrap();
    for now in 0..20 {
        x.poll(now, &mut a);
        y.poll(now, &mut b);
    }
    assert_eq!(x.tcp(t).unwrap().state(), tcp::State::Established);
    assert_eq!(y.tcp(v).unwrap().state(), tcp::State::Established);
    assert_eq!(x.tcp_recv(t, &mut [0; 1]), Err(Error::WouldBlock));
    a.tx.borrow_mut().drop_data = true;
    let input: Vec<u8> = (0..2048).map(|n| n as u8).collect();
    let mut sent = 0;
    let mut received = Vec::new();
    for now in 20..10_000 {
        if sent < input.len() {
            match x.tcp_send(t, &input[sent..]) {
                Ok(n) => sent += n,
                Err(Error::WouldBlock) => {}
                other => panic!("{other:?}"),
            }
        }
        x.poll(now, &mut a);
        y.poll(now, &mut b);
        // Hold the receive window closed long enough to exercise backpressure.
        if now > 1500 {
            let mut buf = [0; 73];
            if let Ok(n) = y.tcp_recv(v, &mut buf) {
                received.extend_from_slice(&buf[..n]);
            }
        }
        if received.len() == input.len() {
            break;
        }
    }
    assert_eq!(a.tx.borrow().lost, 1);
    assert_eq!(received, input);
    x.tcp_close(t).unwrap();
    for now in 10_000..10_100 {
        x.poll(now, &mut a);
        y.poll(now, &mut b);
    }
    assert_eq!(y.tcp_recv(v, &mut [0; 16]), Ok(0));
    y.tcp_send(v, b"after FIN").unwrap();
    y.tcp_close(v).unwrap();
    for now in 10_100..10_200 {
        x.poll(now, &mut a);
        y.poll(now, &mut b);
    }
    let mut reply = [0; 16];
    assert_eq!(x.tcp_recv(t, &mut reply), Ok(9));
    assert_eq!(&reply[..9], b"after FIN");
    assert_eq!(x.tcp_recv(t, &mut reply), Ok(0));
    for now in 10_200..40_500 {
        x.poll(now, &mut a);
        y.poll(now, &mut b);
    }
    assert_eq!(y.tcp(v).unwrap().state(), tcp::State::Closed);
    y.tcp_listen(v, ECHO_PORT).unwrap();
    x.tcp_connect(t, endpoint(16), 50001).unwrap();
    for now in 40_500..40_520 {
        x.poll(now, &mut a);
        y.poll(now, &mut b);
    }
    assert_eq!(x.tcp(t).unwrap().state(), tcp::State::Established);
}

#[test]
fn malformed_headers_fragments_and_all_truncations_are_rejected() {
    let mut f = [0; 54];
    f[12..14].copy_from_slice(&0x0800u16.to_be_bytes());
    f[14] = 0x45;
    f[16..18].copy_from_slice(&40u16.to_be_bytes());
    f[22] = 64;
    f[23] = 6;
    f[46] = 0x50;
    assert!(admit(&f));
    for n in 0..f.len() {
        assert!(!admit(&f[..n]), "truncated at {n}");
    }
    for header in [0, 4, 16, 24, 60] {
        f[46] = header << 2;
        assert!(!admit(&f));
    }
    f[46] = 0x50;
    for frag in [1u16, 0x2000, 0x8000] {
        f[20..22].copy_from_slice(&frag.to_be_bytes());
        assert!(!admit(&f));
    }
    assert!(!admit(&[0; MAX_FRAME + 1]));
}

#[test]
fn checksum_corruption_and_unbound_udp_port_do_not_deliver_data() {
    let (mut a, mut b) = wires();
    let (mut ab, mut bb) = (Buffers::new(), Buffers::new());
    let mut xs = [const { SocketStorage::EMPTY }; 2];
    let mut ys = [const { SocketStorage::EMPTY }; 2];
    let (mut x, u, _) = ab.stack(&mut a, 15, &mut xs);
    let (mut y, v, _) = bb.stack(&mut b, 16, &mut ys);
    x.udp_bind(u, ECHO_PORT).unwrap();
    y.udp_bind(v, ECHO_PORT).unwrap();
    x.udp_send_to(u, b"warmup", endpoint(16)).unwrap();
    for now in 0..10 {
        x.poll(now, &mut a);
        y.poll(now, &mut b);
    }
    assert!(y.udp_recv_from(v, &mut [0; 16]).is_ok());
    for offset in [24, 40, 42] {
        // IPv4 checksum, UDP checksum, payload
        x.udp_send_to(u, b"damaged", endpoint(16)).unwrap();
        x.poll(10, &mut a);
        let mut link = b.rx.borrow_mut();
        let frame = link.frames.back_mut().unwrap();
        assert_eq!(frame[23], 17);
        frame[offset] ^= 1;
        drop(link);
        y.poll(10, &mut b);
        assert_eq!(y.udp_recv_from(v, &mut [0; 16]), Err(Error::WouldBlock));
    }
    let mut unbound = endpoint(16);
    unbound.port = 9001;
    x.udp_send_to(u, b"unbound", unbound).unwrap();
    for now in 11..20 {
        x.poll(now, &mut a);
        y.poll(now, &mut b);
    }
    assert_eq!(y.udp_recv_from(v, &mut [0; 16]), Err(Error::WouldBlock));
}

#[test]
fn fixed_socket_capacity_returns_error_without_allocating() {
    let (mut a, _) = wires();
    let mut ab = Buffers::new();
    let mut storage = [const { SocketStorage::EMPTY }; 2];
    let (mut x, _, _) = ab.stack(&mut a, 15, &mut storage);
    let extra = x.add_tcp(
        tcp::SocketBuffer::new(&mut [][..]),
        tcp::SocketBuffer::new(&mut [][..]),
    );
    assert_eq!(extra, Err(Error::Full));
}
