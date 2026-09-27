//! Allocation-free Ethernet/ARP/IPv4/UDP responder. No fragments, IP options, or offload.
use crate::decisions as p;
pub const MAX_PAYLOAD: usize = 1200;
pub const MAX_FRAME: usize = 1514;
pub const ECHO_PORT: u16 = 9000;
pub const RELAY_PORT: u16 = 9001;
fn word(b: &[u8], i: usize) -> u16 {
    u16::from_be_bytes([b[i], b[i + 1]])
}
fn put(b: &mut [u8], i: usize, n: u16) {
    b[i..i + 2].copy_from_slice(&n.to_be_bytes());
}
fn sum(bytes: &[u8]) -> u32 {
    let mut result = 0u32;
    for pair in bytes.chunks(2) {
        result += ((pair[0] as u32) << 8) | pair.get(1).copied().unwrap_or(0) as u32;
    }
    result
}
fn finish(mut n: u32) -> u16 {
    while n >> 16 != 0 {
        n = (n & 65535) + (n >> 16);
    }
    !(n as u16)
}
pub fn checksum(bytes: &[u8]) -> u16 {
    finish(sum(bytes))
}
fn udp_checksum(ip: &[u8], udp: &[u8]) -> u16 {
    finish(sum(&ip[12..20]) + 17 + udp.len() as u32 + sum(udp))
}
fn unicast(ip: &[u8]) -> bool {
    ip[0] != 0 && ip[0] != 127 && ip[0] < 224 && ip != [255; 4]
}
#[derive(Clone, Copy)]
pub struct Stack {
    pub mac: [u8; 6],
    pub ip: [u8; 4],
}
impl Stack {
    pub fn receive(
        &self,
        frame: &[u8],
        out: &mut [u8; MAX_FRAME],
        mut service: impl FnMut(u16, &[u8], &mut [u8]) -> Option<usize>,
    ) -> Option<usize> {
        if p::net_frame_len(frame.len() as u64) == 0
            || (frame[..6] != self.mac && frame[..6] != [255; 6])
        {
            return None;
        }
        if frame[6] & 1 != 0 || frame[6..12] == [0; 6] {
            return None;
        }
        match word(frame, 12) {
            0x0806 => {
                if frame.len() < 42 {
                    return None;
                }
                let a = &frame[14..42];
                if a[..8] != [0, 1, 8, 0, 6, 4, 0, 1]
                    || a[24..28] != self.ip
                    || a[8..14] != frame[6..12]
                {
                    return None;
                }
                out[..42].fill(0);
                out[..6].copy_from_slice(&frame[6..12]);
                out[6..12].copy_from_slice(&self.mac);
                put(out, 12, 0x0806);
                out[14..22].copy_from_slice(&[0, 1, 8, 0, 6, 4, 0, 2]);
                out[22..28].copy_from_slice(&self.mac);
                out[28..32].copy_from_slice(&self.ip);
                out[32..38].copy_from_slice(&a[8..14]);
                out[38..42].copy_from_slice(&a[14..18]);
                Some(42)
            }
            0x0800 => {
                if frame.len() < 42 || frame[..6] != self.mac {
                    return None;
                }
                let ip = &frame[14..];
                let total = word(ip, 2) as usize;
                if p::net_ipv4(
                    total as u64,
                    ip.len() as u64,
                    word(ip, 6) as u64,
                    ip[8] as u64,
                    ip[9] as u64,
                    ip[0] as u64,
                ) == 0
                    || ip[16..20] != self.ip
                    || !unicast(&ip[12..16])
                    || checksum(&ip[..20]) != 0
                {
                    return None;
                }
                let udp = &ip[20..total];
                let port = word(udp, 2);
                let bound = match port {
                    ECHO_PORT => ECHO_PORT,
                    RELAY_PORT => RELAY_PORT,
                    _ => return None,
                };
                if p::net_udp(
                    word(udp, 4) as u64,
                    udp.len() as u64,
                    port as u64,
                    bound as u64,
                ) == 0
                    || word(udp, 0) == 0
                    || (word(udp, 6) != 0 && udp_checksum(ip, udp) != 0)
                {
                    return None;
                }
                let n = service(port, &udp[8..], &mut out[42..42 + MAX_PAYLOAD])?;
                if n > MAX_PAYLOAD {
                    return None;
                }
                out[..42].fill(0);
                out[..6].copy_from_slice(&frame[6..12]);
                out[6..12].copy_from_slice(&self.mac);
                put(out, 12, 0x0800);
                out[14] = 0x45;
                put(out, 16, (28 + n) as u16);
                put(out, 20, 0x4000);
                out[22] = 64;
                out[23] = 17;
                out[26..30].copy_from_slice(&self.ip);
                out[30..34].copy_from_slice(&ip[12..16]);
                let check = checksum(&out[14..34]);
                put(out, 24, check);
                put(out, 34, port);
                put(out, 36, word(udp, 0));
                put(out, 38, (n + 8) as u16);
                let check = udp_checksum(&out[14..34], &out[34..42 + n]);
                put(out, 40, if check == 0 { 65535 } else { check });
                Some(42 + n)
            }
            _ => None,
        }
    }
}
