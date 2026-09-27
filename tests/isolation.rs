use bastion_core::*;

fn limits() -> Limits {
    Limits {
        pages: 8,
        capabilities: 4,
        cpu_budget: 3,
    }
}
fn kernel() -> Kernel {
    Kernel::new(16, 10, 1).unwrap()
}

#[test]
fn caller_cannot_use_another_process_handle() {
    let mut k = kernel();
    let a = k.spawn(limits(), 2).unwrap();
    let b = k.spawn(limits(), 2).unwrap();
    let h = k.grant(a, b, Rights::ALL).unwrap();
    assert_eq!(k.schedule(), Some(a));
    assert!(matches!(k.syscall(Call::Inspect(h)), Ok(Reply::Info(_))));
    assert_eq!(k.schedule(), Some(b));
    assert_eq!(k.syscall(Call::Terminate(h)), Err(Error::InvalidHandle));
    assert!(k.snapshot(a).is_ok());
    assert!(k.snapshot(b).is_ok());
}

#[test]
fn forged_handles_and_missing_rights_are_denied() {
    let mut k = kernel();
    let a = k.spawn(limits(), 2).unwrap();
    let b = k.spawn(limits(), 2).unwrap();
    let h = k.grant(a, b, Rights::INSPECT).unwrap();
    k.schedule();
    assert_eq!(k.syscall(Call::Terminate(h)), Err(Error::PermissionDenied));
    for raw in [0, a.raw(), b.raw(), u64::MAX] {
        assert_eq!(
            k.syscall(Call::Inspect(Handle::from_raw(raw))),
            Err(Error::InvalidHandle)
        );
    }
}

#[test]
fn restriction_cannot_amplify_authority() {
    let mut k = kernel();
    let a = k.spawn(limits(), 0).unwrap();
    let h = k.grant(a, a, Rights::ALL).unwrap();
    k.schedule();
    assert_eq!(
        k.syscall(Call::Restrict {
            handle: h,
            rights: Rights::INSPECT
        }),
        Ok(Reply::Done)
    );
    assert_eq!(
        k.syscall(Call::Restrict {
            handle: h,
            rights: Rights::ALL
        }),
        Err(Error::PermissionDenied)
    );
    assert_eq!(k.syscall(Call::Terminate(h)), Err(Error::PermissionDenied));
}

#[test]
fn process_exit_revokes_all_caps_and_reclaims_owned_pages() {
    let mut k = kernel();
    let a = k.spawn(limits(), 2).unwrap();
    let b = k.spawn(limits(), 3).unwrap();
    let h = k.grant(a, b, Rights::ALL).unwrap();
    k.schedule();
    k.syscall(Call::Terminate(h)).unwrap();
    assert_eq!(k.used_pages(), 2);
    assert_eq!(k.snapshot(a).unwrap().usage.capabilities, 0);
    assert_eq!(k.snapshot(b), Err(Error::InvalidProcess));
    let replacement = k.spawn(limits(), 1).unwrap();
    assert_ne!(b, replacement);
    assert_eq!(k.syscall(Call::Terminate(h)), Err(Error::InvalidHandle));
    assert!(k.snapshot(replacement).is_ok());
}

#[test]
fn closed_handles_never_refer_to_replacement_capabilities() {
    let mut k = kernel();
    let a = k.spawn(limits(), 0).unwrap();
    let old = k.grant(a, a, Rights::ALL).unwrap();
    k.schedule();
    k.syscall(Call::Close(old)).unwrap();
    let new = k.grant(a, a, Rights::ALL).unwrap();
    assert_ne!(old, new);
    assert_eq!(k.syscall(Call::Terminate(old)), Err(Error::InvalidHandle));
}

#[test]
fn rejected_reservations_leave_accounting_unchanged() {
    let mut k = kernel();
    let a = k.spawn(limits(), 2).unwrap();
    let before = k.snapshot(a).unwrap();
    for amount in [7, u64::MAX] {
        assert_eq!(k.reserve_pages(a, amount), Err(Error::PageLimit));
        assert_eq!(k.snapshot(a).unwrap(), before);
        assert_eq!(k.used_pages(), 2);
    }
    assert_eq!(k.release_pages(a, 3), Err(Error::InsufficientPages));
    assert_eq!(k.used_pages(), 2);
}

#[test]
fn global_limit_and_capability_limit_are_enforced() {
    let mut k = Kernel::new(3, 10, 1).unwrap();
    let a = k.spawn(limits(), 2).unwrap();
    assert_eq!(k.reserve_pages(a, 2), Err(Error::GlobalPageLimit));
    assert_eq!(k.snapshot(a).unwrap().usage.pages, 2);
    assert_eq!(k.spawn(limits(), 2), Err(Error::GlobalPageLimit));
    for _ in 0..4 {
        k.grant(a, a, Rights::INSPECT).unwrap();
    }
    assert_eq!(k.grant(a, a, Rights::ALL), Err(Error::CapabilityLimit));
}

#[test]
fn yields_do_not_restore_budget_and_idle_waits_for_replenishment() {
    let mut k = kernel();
    let a = k.spawn(limits(), 1).unwrap();
    for now in 0..3 {
        assert_eq!(k.schedule(), Some(a));
        assert_eq!(k.deadline(), now + 1);
        k.account_until(now + 1).unwrap();
        k.syscall(Call::Yield).unwrap();
    }
    assert_eq!(k.schedule(), None);
    assert_eq!(k.deadline(), 10);
    k.account_until(9).unwrap();
    assert_eq!(k.schedule(), None);
    k.account_until(10).unwrap();
    assert_eq!(k.schedule(), Some(a));
    assert_eq!(k.snapshot(a).unwrap().usage.cpu_remaining, 3);
}

#[test]
fn round_robin_prevents_a_busy_process_from_owning_every_quantum() {
    let mut k = kernel();
    let a = k.spawn(limits(), 0).unwrap();
    let b = k.spawn(limits(), 0).unwrap();
    for (now, expected) in [a, b, a, b, a, b].into_iter().enumerate() {
        assert_eq!(k.schedule(), Some(expected));
        k.account_until(now as u64 + 1).unwrap();
    }
    assert_eq!(k.schedule(), None);
}

#[test]
fn clock_regression_is_rejected_without_changing_state() {
    let mut k = kernel();
    let a = k.spawn(limits(), 0).unwrap();
    k.schedule();
    k.account_until(2).unwrap();
    let before = k.snapshot(a).unwrap();
    assert_eq!(k.account_until(1), Err(Error::ClockWentBackwards));
    assert_eq!(k.now(), 2);
    assert_eq!(k.snapshot(a).unwrap(), before);
}

#[test]
fn late_timer_does_not_bank_credit_from_skipped_periods() {
    let mut k = kernel();
    let a = k.spawn(limits(), 0).unwrap();
    k.schedule();
    k.account_until(102).unwrap();
    assert_eq!(k.snapshot(a).unwrap().usage.cpu_remaining, 1);
}

#[test]
fn fault_kills_only_the_current_process() {
    let mut k = kernel();
    let a = k.spawn(limits(), 2).unwrap();
    let b = k.spawn(limits(), 3).unwrap();
    k.schedule();
    assert_eq!(k.fault_current(), Ok(a));
    assert_eq!(k.current(), None);
    assert_eq!(k.used_pages(), 3);
    assert_eq!(k.schedule(), Some(b));
}

#[test]
fn no_untrusted_syscall_can_run_without_a_scheduled_caller() {
    let mut k = kernel();
    assert_eq!(k.syscall(Call::SelfInfo), Err(Error::NoCurrentProcess));
}

#[test]
fn full_process_table_and_bad_limits_fail_cleanly() {
    assert!(matches!(Kernel::new(1, 0, 1), Err(Error::InvalidLimit)));
    let mut k = kernel();
    assert_eq!(
        k.spawn(
            Limits {
                cpu_budget: 11,
                ..limits()
            },
            0
        ),
        Err(Error::InvalidLimit)
    );
    for _ in 0..MAX_PROCESSES {
        k.spawn(limits(), 0).unwrap();
    }
    assert_eq!(k.spawn(limits(), 0), Err(Error::ProcessTableFull));
}

#[test]
fn resource_arithmetic_handles_machine_word_boundaries() {
    assert_eq!(policy::reserve(u64::MAX - 1, 1, u64::MAX), Some(u64::MAX));
    assert_eq!(policy::reserve(u64::MAX, 1, u64::MAX), None);
    assert_eq!(policy::reserve(u64::MAX, u64::MAX, u64::MAX), None);
    assert_eq!(policy::charge(2, u64::MAX), 0);
}

#[test]
fn rust_matches_executable_lean_specification() {
    let cases = include_str!("data/policy-vectors.csv");
    let mut count = 0;
    for line in cases.lines().skip(1) {
        let fields: Vec<_> = line.split(',').collect();
        let a = fields[1].parse().unwrap();
        let b = fields[2].parse().unwrap();
        let c = fields[3].parse().unwrap();
        match fields[0] {
            "reserve" => {
                let expected = if fields[4] == "denied" {
                    None
                } else {
                    Some(fields[4].parse().unwrap())
                };
                assert_eq!(policy::reserve(a, b, c), expected, "{line}");
            }
            "charge" => assert_eq!(policy::charge(a, b), fields[4].parse().unwrap(), "{line}"),
            _ => panic!("unknown vector: {line}"),
        }
        count += 1;
    }
    assert_eq!(count, 1254);
}
