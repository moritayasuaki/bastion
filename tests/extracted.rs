//! Compare actual linked C against execution of the Lean source, including
//! invalid inputs, word boundaries, all 256 runnable masks, and period changes.
use bastion_core::decisions as p;
use std::collections::BTreeSet;

#[test]
fn extracted_c_matches_lean_across_every_export() {
    let mut covered = BTreeSet::new();
    let mut count = 0;
    for line in include_str!("data/runtime-vectors.csv").lines().skip(1) {
        let fields: Vec<_> = line.split(',').collect();
        assert_eq!(fields.len(), 8);
        let v: Vec<u64> = fields[1..].iter().map(|x| x.parse().unwrap()).collect();
        let (a, b, c, d, e, f) = (v[0], v[1], v[2], v[3], v[4], v[5]);
        let actual = match fields[0] {
            "abi_version" => p::abi_version(a),
            "valid_config" => p::valid_config(a, b).into(),
            "valid_limits" => p::valid_limits(a, b, c, d, e).into(),
            "can_reserve" => p::can_reserve(a, b, c).into(),
            "reservation_status" => p::reservation_status(a, b, c, d, e),
            "reserve_value" => p::reserve_value(a, b, c),
            "can_release" => p::can_release(a, b).into(),
            "release_value" => p::release_value(a, b),
            "next_identifier" => p::next_identifier(a),
            "same_identity" => p::same_identity(a, b).into(),
            "can_grant" => p::can_grant(a, b).into(),
            "rights_allow" => p::rights_allow(a, b).into(),
            "authorized" => p::authorized(a, b, c, d, e).into(),
            "restrict_rights" => p::restrict_rights(a, b),
            "revoke_target" => p::revoke_target(a, b).into(),
            "charge" => p::charge(a, b),
            "clock_valid" => p::clock_valid(a, b).into(),
            "account" => p::account(a, b, c, d, e, f),
            "runnable" => p::runnable(a, b).into(),
            "next_slot" => p::next_slot(a, b),
            "next_cursor" => p::next_cursor(a),
            "deadline" => p::deadline(a, b, c, d, e),
            "valid_user_return" => p::valid_user_return(a, b).into(),
            "user_flags" => p::user_flags(a),
            "supervisor_entry" => p::supervisor_entry(a),
            "user_page_entry" => p::user_page_entry(a, b),
            "syscall_opcode" => p::syscall_opcode(a),
            "net_frame_len" => p::net_frame_len(a).into(),
            "net_ipv4" => p::net_ipv4(a, b, c, d, e, f).into(),
            "net_udp" => p::net_udp(a, b, c, d).into(),
            "net_budget" => p::net_budget(a).into(),
            "relay_ingress" => p::relay_ingress(a, b, c, d, e, f).into(),
            "field_share" => p::field_share(a, b, c, d, e),
            "field_reconstruct" => p::field_reconstruct(a, b, c, d, e),
            "unique_step" => p::unique_step(a, b, c),
            other => panic!("unknown exported operation {other}"),
        };
        assert_eq!(actual, v[6], "{line}");
        covered.insert(fields[0]);
        count += 1;
    }
    assert_eq!(covered.len(), 35);
    assert!(count > 10_000, "truncated conformance corpus: {count}");
}

#[test]
fn selected_slot_is_first_eligible_slot_for_every_runnable_mask() {
    for mask in 0..256 {
        for cursor in 0..8 {
            let eligible: Vec<_> = (0..8)
                .map(|offset| (cursor + offset) % 8)
                .filter(|slot| mask & (1 << slot) != 0)
                .collect();
            assert_eq!(
                p::next_slot(mask, cursor),
                eligible.first().copied().unwrap_or(8)
            );
        }
    }
}

#[test]
fn invalid_flags_and_addresses_cannot_create_privileged_user_returns() {
    assert_eq!(p::user_flags(u64::MAX), 0xed7);
    assert_eq!(p::valid_user_return(0x800000000000, 0x501000), 0);
    assert_eq!(p::valid_user_return(0x400000, u64::MAX), 0);
    assert_eq!(p::supervisor_entry(u64::MAX) & 4, 0);
    assert_eq!(p::user_page_entry(0x1001, 1), 0);
    assert_eq!(p::user_page_entry(0x0010000000000000, 1), 0);
    assert_eq!(p::user_page_entry(0x1000, 3), 0);
    assert_eq!(p::user_page_entry(0x1000, 1), 0x1005);
    assert_eq!(p::user_page_entry(0x1000, 2), 0x8000000000001007);
}

#[test]
fn malformed_abi_inputs_fail_closed() {
    assert_eq!(p::account(10, 10, 5, 4, 100, 1), 0);
    assert_eq!(p::account(10, 10, 5, 6, 0, 1), 0);
    assert_eq!(p::next_identifier(0), 0);
    assert_eq!(p::next_identifier(u64::MAX), 0);
    assert_eq!(p::next_slot(255, u64::MAX), 8);
    assert_eq!(p::rights_allow(u64::MAX, 1), 0);
    assert_eq!(p::authorized(1, 1, 2, 3, 1), 0);
    assert_eq!(p::syscall_opcode(u64::MAX), 255);
}
