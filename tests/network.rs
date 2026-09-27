use bastion_core::{
    decisions as p,
    net::{self, MAX_FRAME, Stack},
    relay::{self, BODY, Config, PACKET, Relay, SESSION_TICKS},
};
const GUEST: Stack = Stack {
    mac: [2, 0, 0, 0, 0, 1],
    ip: [10, 0, 2, 15],
};
fn fix_ip(frame: &mut [u8]) {
    frame[24..26].fill(0);
    let c = net::checksum(&frame[14..34]);
    frame[24..26].copy_from_slice(&c.to_be_bytes());
}
fn frame(payload: &[u8]) -> Vec<u8> {
    let mut f = vec![0; 42 + payload.len()];
    f[..6].copy_from_slice(&GUEST.mac);
    f[6..12].copy_from_slice(&[2, 0, 0, 0, 0, 2]);
    f[12..14].copy_from_slice(&[8, 0]);
    f[14] = 0x45;
    f[16..18].copy_from_slice(&((28 + payload.len()) as u16).to_be_bytes());
    f[22] = 64;
    f[23] = 17;
    f[26..30].copy_from_slice(&[10, 0, 2, 2]);
    f[30..34].copy_from_slice(&GUEST.ip);
    f[34..36].copy_from_slice(&40000u16.to_be_bytes());
    f[36..38].copy_from_slice(&9000u16.to_be_bytes());
    f[38..40].copy_from_slice(&((8 + payload.len()) as u16).to_be_bytes());
    f[42..].copy_from_slice(payload);
    fix_ip(&mut f);
    f
}
fn echo(f: &[u8]) -> Option<Vec<u8>> {
    let mut out = [0; MAX_FRAME];
    let n = GUEST.receive(f, &mut out, |_, p, o| {
        o[..p.len()].copy_from_slice(p);
        Some(p.len())
    })?;
    Some(out[..n].to_vec())
}
#[test]
fn udp_roundtrip_lengths_checksums_and_all_truncations() {
    for n in [0, 1, 2, 31, 512, 1200] {
        let payload = vec![0x81; n];
        let mut f = frame(&payload);
        f[34..36].copy_from_slice(&9000u16.to_be_bytes());
        let reply = echo(&f).unwrap();
        assert_eq!(&reply[42..], payload);
        assert_eq!(net::checksum(&reply[14..34]), 0);
        for end in 0..f.len() {
            assert!(echo(&f[..end]).is_none());
        }
        // Validate the emitted nonzero UDP checksum through an independent peer instance.
        let peer = Stack {
            mac: f[6..12].try_into().unwrap(),
            ip: [10, 0, 2, 2],
        };
        let mut out = [0; MAX_FRAME];
        assert_eq!(
            peer.receive(&reply, &mut out, |_, p, o| {
                assert_eq!(p, payload);
                o[..p.len()].copy_from_slice(p);
                Some(p.len())
            }),
            Some(reply.len())
        );
        let mut incoming = reply.clone();
        incoming[40] ^= 1;
        assert!(
            peer.receive(&incoming, &mut out, |_, _, _| Some(0))
                .is_none()
        );
    }
    assert!(echo(&frame(&[0; 1201])).is_none());
}
#[test]
fn malformed_network_headers_never_reach_service() {
    let base = frame(b"hello");
    for (offset, value) in [
        (14, 0x46),
        (14, 0x65),
        (20, 0x20),
        (20, 0x80),
        (21, 1),
        (22, 0),
        (23, 6),
        (30, 11),
        (26, 224),
        (38, 0xff),
        (39, 7),
        (36, 1),
    ] {
        let mut f = base.clone();
        f[offset] = value;
        fix_ip(&mut f);
        assert!(echo(&f).is_none(), "offset {offset}");
    }
    let mut f = base.clone();
    f[24] ^= 1;
    assert!(echo(&f).is_none());
    f = base;
    f[40..42].copy_from_slice(&1u16.to_be_bytes());
    assert!(echo(&f).is_none());
}
#[test]
fn arp_answers_only_the_local_address() {
    let mut f = [0; 42];
    f[..6].fill(255);
    f[6..12].copy_from_slice(&[2, 0, 0, 0, 0, 2]);
    f[12..22].copy_from_slice(&[8, 6, 0, 1, 8, 0, 6, 4, 0, 1]);
    f[22..28].copy_from_slice(&[2, 0, 0, 0, 0, 2]);
    f[28..32].copy_from_slice(&[10, 0, 2, 2]);
    f[38..42].copy_from_slice(&GUEST.ip);
    let reply = echo(&f).unwrap();
    assert_eq!(&reply[20..22], &[0, 2]);
    assert_eq!(&reply[22..28], &GUEST.mac);
    f[41] = 16;
    assert!(echo(&f).is_none());
}
fn config() -> Config {
    Config {
        id: 1,
        epoch: [9; 16],
        alice: [1; 32],
        bob: [2; 32],
    }
}
fn body(c: &Config, sid: [u8; 16]) -> [u8; BODY] {
    let mut b = [0; BODY];
    b[..96].copy_from_slice(&relay::transcript(c.epoch, sid, [8; 16]));
    b
}
#[allow(clippy::too_many_arguments)]
fn request(
    r: &mut Relay,
    c: &Config,
    role: u8,
    op: u8,
    sid: [u8; 16],
    seq: u64,
    b: &[u8],
    now: u64,
) -> Option<Vec<u8>> {
    let mut wire = [0; PACKET];
    let n = relay::encode(c, role, op, sid, seq, b, &mut wire)?;
    let mut out = [0; PACKET];
    let n = r.handle(&wire[..n], now, &mut out)?;
    Some(out[..n].to_vec())
}
#[test]
fn relay_authentication_identity_replay_and_conflicts() {
    let c = config();
    let mut r = Relay::new(c.clone());
    let sid = [3; 16];
    let b = body(&c, sid);
    assert!(request(&mut r, &c, 1, 1, sid, 1, &b, 0).is_some());
    let reply = request(&mut r, &c, 2, 2, sid, 1, &[], 1).unwrap();
    let mut plain = [0; BODY];
    assert_eq!(
        relay::decode_response(&c, 2, 130, sid, 1, &reply, &mut plain),
        Some(BODY)
    );
    assert_eq!(plain, b);
    assert!(request(&mut r, &c, 2, 2, sid, 1, &[], 2).is_none());
    let mut wire = [0; PACKET];
    let n = relay::encode(&c, 2, 2, sid, 2, &[], &mut wire).unwrap();
    wire[n - 1] ^= 1;
    assert!(r.handle(&wire[..n], 2, &mut plain).is_none());
    assert!(request(&mut r, &c, 2, 2, sid, 2, &[], 2).is_some()); // forgery did not advance replay state
    let mut wrong = c.clone();
    wrong.id = 2;
    assert!(request(&mut r, &wrong, 2, 2, sid, 3, &[], 3).is_none());
    assert!(request(&mut r, &c, 2, 1, sid, 3, &b, 3).is_none()); // Bob cannot write Alice's slot
    let mut changed = b;
    changed[96] = 1;
    assert!(request(&mut r, &c, 1, 1, sid, 2, &changed, 4).is_none());
    assert!(request(&mut r, &c, 2, 2, sid, 3, &[], 5).is_none()); // conflicting slot is poisoned
}
#[test]
fn relay_storage_expiry_and_canonical_field_encodings() {
    let c = config();
    let mut r = Relay::new(c.clone());
    for n in 1..=4 {
        let sid = [n; 16];
        assert!(request(&mut r, &c, 1, 1, sid, n as u64, &body(&c, sid), 0).is_some());
    }
    assert!(request(&mut r, &c, 1, 1, [5; 16], 5, &body(&c, [5; 16]), 1).is_none());
    assert!(
        request(
            &mut r,
            &c,
            1,
            1,
            [6; 16],
            6,
            &body(&c, [6; 16]),
            SESSION_TICKS + 1
        )
        .is_some()
    );
    assert!(request(&mut r, &c, 2, 2, [1; 16], 1, &[], SESSION_TICKS + 2).is_none());
    assert!(relay::decode_share(&[0; 63]).is_none());
    let mut bad = [0; 64];
    bad[..2].copy_from_slice(&257u16.to_le_bytes());
    assert!(relay::decode_share(&bad).is_none());
    assert!(
        request(
            &mut r,
            &c,
            1,
            1,
            [7; 16],
            5,
            &body(&c, [7; 16]),
            SESSION_TICKS + 3
        )
        .is_none()
    );
}
#[test]
fn octave_scalar_adapter_matches_canonical_project() {
    let mut count = 0;
    for row in include_str!("data/octave-vectors.csv").lines().skip(1) {
        let f: Vec<_> = row.split(',').collect();
        assert_eq!(f.len(), 7);
        let v: Vec<u64> = f[1..].iter().map(|s| s.parse().unwrap()).collect();
        let got = match f[0] {
            "share" => p::field_share(v[0], v[1], v[2], v[3], v[4]),
            "reconstruct" => p::field_reconstruct(v[0], v[1], v[2], v[3], v[4]),
            _ => panic!("bad vector"),
        };
        assert_eq!(got, v[5], "{row}");
        count += 1;
    }
    assert_eq!(count, 18944);
}
#[test]
fn octave_fault_tolerance_ambiguity_and_no_byte_truncation() {
    let root = [42; 32];
    let coins = [[1, 256, 99]; 32];
    let mut slots: [Option<[u16; 32]>; 8] = [None; 8];
    for (i, slot) in slots.iter_mut().enumerate() {
        *slot = relay::decode_share(&relay::share(&root, &coins, (i + 1) as u8).unwrap());
    }
    let t = relay::transcript([1; 16], [2; 16], [3; 16]);
    let proof = relay::confirmation(&root, &t, false);
    assert_eq!(
        relay::recover(&slots, |r| relay::confirms(r, &t, &proof, false)),
        Some(root)
    );
    assert_eq!(relay::recover(&slots, |_| true), Some(root)); // repeated same value is not ambiguity
    for slot in &mut slots[..3] {
        for v in slot.as_mut().unwrap() {
            *v = (*v + 1) % 257;
        }
    }
    slots[7] = None;
    assert_eq!(
        relay::recover(&slots, |r| relay::confirms(r, &t, &proof, false)),
        Some(root)
    );
    assert!(relay::recover(&slots, |_| true).is_none());
    let nonbyte = [Some([256; 32]); 8];
    let mut called = false;
    assert!(
        relay::recover(&nonbyte, |_| {
            called = true;
            true
        })
        .is_none()
    );
    assert!(!called);
    slots[3..].fill(None);
    assert!(relay::recover(&slots, |_| true).is_none());
}
