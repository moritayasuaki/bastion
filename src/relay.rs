//! Octave v0.2 transport adapter: one authenticated slot per relay, bounded storage.
use crate::decisions as p;
pub use bastion_crypto;
use bastion_crypto::{Session, derive, nonce};
pub const BODY: usize = 176;
pub const HEADER: usize = 32;
pub const PACKET: usize = HEADER + BODY + 16;
pub const CONFIG_SIZE: usize = 88;
pub const SESSION_TICKS: u64 = 60_000;
#[derive(Clone)]
pub struct Config {
    pub id: u8,
    pub epoch: [u8; 16],
    pub alice: [u8; 32],
    pub bob: [u8; 32],
}
impl Config {
    pub fn decode(b: &[u8]) -> Option<Self> {
        if b.len() != CONFIG_SIZE
            || &b[..4] != b"BRL1"
            || !(1..=8).contains(&b[4])
            || b[5..8] != [0; 3]
        {
            return None;
        }
        Some(Self {
            id: b[4],
            epoch: b[8..24].try_into().ok()?,
            alice: b[24..56].try_into().ok()?,
            bob: b[56..88].try_into().ok()?,
        })
    }
    pub fn encode(&self) -> [u8; CONFIG_SIZE] {
        let mut b = [0; CONFIG_SIZE];
        b[..4].copy_from_slice(b"BRL1");
        b[4] = self.id;
        b[8..24].copy_from_slice(&self.epoch);
        b[24..56].copy_from_slice(&self.alice);
        b[56..88].copy_from_slice(&self.bob);
        b
    }
    pub fn link(&self, role: u8, response: bool) -> Session {
        let mut context = [0; 18];
        context[..16].copy_from_slice(&self.epoch);
        context[16] = self.id;
        context[17] = role;
        let key = if role == 1 { &self.alice } else { &self.bob };
        Session::new(&derive(
            key,
            &context,
            if response {
                b"relay-to-client"
            } else {
                b"client-to-relay"
            },
        ))
        .expect("fixed PSIV key")
    }
}
pub fn transcript(epoch: [u8; 16], session: [u8; 16], challenge: [u8; 16]) -> [u8; 96] {
    let mut t = [0; 96];
    t[..8].copy_from_slice(b"OCTVPS01");
    t[8..12].copy_from_slice(&[4, 8, 3, 1]);
    t[12] = 1;
    t[16..32].copy_from_slice(&epoch);
    t[32..48].copy_from_slice(&session);
    t[48..64].copy_from_slice(&challenge);
    t[64] = 1;
    t[72] = 2;
    t[80..88].copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
    t
}
pub fn valid_body(body: &[u8], config: &Config, sid: &[u8]) -> bool {
    if body.len() != BODY || &body[32..48] != sid {
        return false;
    }
    let expected = transcript(
        config.epoch,
        body[32..48].try_into().unwrap(),
        body[48..64].try_into().unwrap(),
    );
    body[..96] == expected && decode_share(&body[96..160]).is_some()
}
pub fn header(id: u8, role: u8, op: u8, sid: [u8; 16], seq: u64) -> [u8; 32] {
    let mut h = [0; 32];
    h[..4].copy_from_slice(b"OCTV");
    h[4] = 1;
    h[5] = op;
    h[6] = id;
    h[7] = role;
    h[8..24].copy_from_slice(&sid);
    h[24..32].copy_from_slice(&seq.to_le_bytes());
    h
}
pub fn encode(
    config: &Config,
    role: u8,
    op: u8,
    sid: [u8; 16],
    seq: u64,
    body: &[u8],
    out: &mut [u8],
) -> Option<usize> {
    if out.len() < HEADER + body.len() + 16 {
        return None;
    }
    let h = header(config.id, role, op, sid, seq);
    out[..HEADER].copy_from_slice(&h);
    let n = config
        .link(role, op >= 128)
        .seal(&nonce(seq), &h, body, &mut out[HEADER..])
        .ok()?;
    Some(HEADER + n)
}
pub fn decode_response(
    config: &Config,
    role: u8,
    op: u8,
    sid: [u8; 16],
    seq: u64,
    packet: &[u8],
    out: &mut [u8],
) -> Option<usize> {
    let h = header(config.id, role, op, sid, seq);
    if packet.len() < 48 || packet[..HEADER] != h {
        return None;
    }
    config
        .link(role, true)
        .open(&nonce(seq), &h, &packet[HEADER..], out)
        .ok()
}
#[derive(Clone, Copy)]
struct Slot {
    live: bool,
    poisoned: bool,
    born: u64,
    body: [u8; BODY],
}
impl Slot {
    const EMPTY: Self = Self {
        live: false,
        poisoned: false,
        born: 0,
        body: [0; BODY],
    };
}
pub struct Relay {
    config: Config,
    previous: [u64; 2],
    slots: [Slot; 4],
}
impl Relay {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            previous: [0; 2],
            slots: [Slot::EMPTY; 4],
        }
    }
    pub fn handle(&mut self, packet: &[u8], now: u64, out: &mut [u8]) -> Option<usize> {
        if packet.len() < 48
            || packet.len() > PACKET
            || &packet[..4] != b"OCTV"
            || packet[4] != 1
            || packet[6] != self.config.id
        {
            return None;
        }
        let op = packet[5];
        let role = packet[7];
        if !(1..=2).contains(&role) {
            return None;
        }
        let seq = u64::from_le_bytes(packet[24..32].try_into().ok()?);
        let sid: [u8; 16] = packet[8..24].try_into().ok()?;
        if p::relay_ingress(
            self.config.id as u64,
            role as u64,
            op as u64,
            seq,
            self.previous[(role - 1) as usize],
            (packet.len() - 48) as u64,
        ) == 0
        {
            return None;
        }
        let mut plaintext = [0; BODY];
        let n = self
            .config
            .link(role, false)
            .open(
                &nonce(seq),
                &packet[..HEADER],
                &packet[HEADER..],
                &mut plaintext,
            )
            .ok()?;
        self.previous[(role - 1) as usize] = seq;
        // Expiry does not reset link replay counters. Reboot requires fresh provisioning keys.
        for slot in &mut self.slots {
            if slot.live && (now < slot.born || now - slot.born > SESSION_TICKS) {
                *slot = Slot::EMPTY;
            }
        }
        if op == 1 {
            if n != BODY || !valid_body(&plaintext, &self.config, &sid) {
                return None;
            }
            if let Some(slot) = self
                .slots
                .iter_mut()
                .find(|s| s.live && s.body[32..48] == sid)
            {
                if slot.poisoned {
                    return None;
                }
                if slot.body != plaintext {
                    slot.poisoned = true;
                    return None;
                }
            } else {
                let slot = self.slots.iter_mut().find(|s| !s.live)?;
                *slot = Slot {
                    live: true,
                    poisoned: false,
                    born: now,
                    body: plaintext,
                };
            }
            encode(&self.config, role, 129, sid, seq, &[], out)
        } else {
            let slot = self
                .slots
                .iter()
                .find(|s| s.live && !s.poisoned && s.body[32..48] == sid)?;
            encode(&self.config, role, 130, sid, seq, &slot.body, out)
        }
    }
}
pub fn decode_share(bytes: &[u8]) -> Option<[u16; 32]> {
    if bytes.len() != 64 {
        return None;
    }
    let mut s = [0; 32];
    for (i, x) in s.iter_mut().enumerate() {
        *x = u16::from_le_bytes([bytes[2 * i], bytes[2 * i + 1]]);
        if *x > 256 {
            return None;
        }
    }
    Some(s)
}
pub fn share(root: &[u8; 32], coins: &[[u16; 3]; 32], id: u8) -> Option<[u8; 64]> {
    let mut out = [0; 64];
    for i in 0..32 {
        let c = coins[i];
        let v = p::field_share(
            root[i] as u64,
            c[0] as u64,
            c[1] as u64,
            c[2] as u64,
            id as u64,
        );
        if v > 256 {
            return None;
        }
        out[2 * i..2 * i + 2].copy_from_slice(&(v as u16).to_le_bytes());
    }
    Some(out)
}
/// Freeze evidence before this call. Every available four-subset is examined;
/// candidate equality is over the 32-byte value, never a vote count or subset ID.
pub fn recover(
    slots: &[Option<[u16; 32]>; 8],
    mut confirms: impl FnMut(&[u8; 32]) -> bool,
) -> Option<[u8; 32]> {
    let mut chosen = [0; 32];
    let mut state = 0;
    for a in 0..5 {
        for b in a + 1..6 {
            for c in b + 1..7 {
                for d in c + 1..8 {
                    let indices = [a, b, c, d];
                    if indices.iter().any(|i| slots[*i].is_none()) {
                        continue;
                    }
                    let points =
                        ((a + 1) | ((b + 1) << 8) | ((c + 1) << 16) | ((d + 1) << 24)) as u64;
                    let mut candidate = [0; 32];
                    let mut valid = true;
                    for (i, byte) in candidate.iter_mut().enumerate() {
                        let v = p::field_reconstruct(
                            points,
                            slots[a].as_ref().unwrap()[i] as u64,
                            slots[b].as_ref().unwrap()[i] as u64,
                            slots[c].as_ref().unwrap()[i] as u64,
                            slots[d].as_ref().unwrap()[i] as u64,
                        );
                        if v > 255 {
                            valid = false;
                            break;
                        }
                        *byte = v as u8;
                    }
                    if !valid {
                        continue;
                    }
                    let confirmed = confirms(&candidate);
                    let next =
                        p::unique_step(state, (candidate == chosen) as u64, confirmed as u64);
                    if state == 0 && next == 1 {
                        chosen = candidate;
                    }
                    state = next;
                }
            }
        }
    }
    if state == 1 { Some(chosen) } else { None }
}
pub fn confirmation(root: &[u8; 32], t: &[u8; 96], reverse: bool) -> [u8; 16] {
    let key = derive(root, t, if reverse { b"confirmBA" } else { b"confirmAB" });
    let mut record = [0; 16];
    Session::new(&key)
        .unwrap()
        .seal(&nonce(1), t, &[], &mut record)
        .unwrap();
    record
}
pub fn confirms(root: &[u8; 32], t: &[u8; 96], record: &[u8; 16], reverse: bool) -> bool {
    let key = derive(root, t, if reverse { b"confirmBA" } else { b"confirmAB" });
    Session::new(&key)
        .unwrap()
        .open(&nonce(1), t, record, &mut [])
        .is_ok()
}
