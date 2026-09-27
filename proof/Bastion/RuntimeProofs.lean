import Bastion.Policy
import Bastion.Runtime
import Std.Tactic.BVDecide

namespace Bastion.Runtime

-- These statements concern the actual definitions sent to Lean's C backend.
theorem charge_refines_nat (remaining elapsed : UInt64) :
    (charge remaining elapsed).toNat = Bastion.charge remaining.toNat elapsed.toNat := by
  unfold charge Bastion.charge
  split
  next h => exact UInt64.toNat_sub_of_le _ _ h
  next h => simp only [UInt64.toNat_zero]; simp only [UInt64.le_iff_toNat_le] at h; omega

theorem charge_bounded (remaining elapsed : UInt64) :
    (charge remaining elapsed).toNat ≤ remaining.toNat := by
  rw [charge_refines_nat]
  exact Bastion.charge_never_increases

theorem reservation_refines_nat (used requested limit : UInt64) :
    canReserve used requested limit = true ↔ used.toNat + requested.toNat ≤ limit.toNat := by
  simp only [canReserve, Bool.and_eq_true, decide_eq_true_eq]
  constructor
  · intro h
    have hs := UInt64.toNat_sub_of_le limit used h.1
    simp only [UInt64.le_iff_toNat_le] at h
    omega
  · intro h
    have hu : used ≤ limit := by simp only [UInt64.le_iff_toNat_le]; omega
    have hs := UInt64.toNat_sub_of_le limit used hu
    constructor
    · exact hu
    · simp only [UInt64.le_iff_toNat_le]; omega

theorem reservation_value_exact (h : canReserve used requested limit = true) :
    (reserveValue used requested limit).toNat = used.toNat + requested.toNat := by
  have bound := (reservation_refines_nat used requested limit).mp h
  have upper := limit.toNat_lt_size
  change limit.toNat < 18446744073709551616 at upper
  simp only [reserveValue, h, ↓reduceIte, UInt64.toNat_add]
  apply Nat.mod_eq_of_lt
  omega

theorem reservation_value_bounded (h : canReserve used requested limit = true) :
    (reserveValue used requested limit).toNat ≤ limit.toNat := by
  rw [reservation_value_exact h]
  exact (reservation_refines_nat used requested limit).mp h

theorem reservation_rejection_preserves (h : canReserve used requested limit = false) :
    reserveValue used requested limit = used := by simp [reserveValue, h]

theorem release_conserves_word (h : canRelease used amount = true) :
    (releaseValue used amount).toNat + amount.toNat = used.toNat := by
  have le : amount ≤ used := by simpa [canRelease] using h
  simp only [releaseValue, h, ↓reduceIte, UInt64.toNat_sub_of_le _ _ le]
  simp only [UInt64.le_iff_toNat_le] at le
  omega

theorem identifier_exhaustion : nextIdentifier 18446744073709551615 = 0 := by decide

theorem rights_no_amplification (held requested : UInt64)
    (h : restrictRights held requested != 4) :
    rightsAllow held (restrictRights held requested) = true := by
  simp only [restrictRights] at *
  split at h <;> simp_all

theorem wrong_owner_denied (caller owner live held needed : UInt64) (h : caller ≠ owner) :
    authorized caller owner live held needed = false := by
  simp [authorized, sameIdentity, h]

theorem dead_target_denied_word (caller owner held needed : UInt64) :
    authorized caller owner 0 held needed = false := by simp [authorized]

theorem next_slot_bounded (mask cursor : UInt64) : nextSlot mask cursor ≤ 8 := by
  unfold nextSlot eligible
  bv_decide

theorem empty_mask_idles (cursor : UInt64) : nextSlot 0 cursor = 8 := by
  unfold nextSlot eligible bne
  bv_decide

theorem user_flags_no_privilege (flags : UInt64) :
    (userFlags flags &&& 0x37000) = 0 := by
  unfold userFlags
  bv_decide

theorem user_flags_interrupts_enabled (flags : UInt64) :
    (userFlags flags &&& 0x202) = 0x202 := by
  unfold userFlags
  bv_decide

theorem inherited_kernel_pages_supervisor (entry : UInt64) :
    (supervisorEntry entry &&& 4) = 0 := by
  unfold supervisorEntry
  bv_decide

theorem code_pages_readonly (physical : UInt64) :
    (userPageEntry physical 1 &&& 2) = 0 := by
  unfold userPageEntry bne
  bv_decide

theorem data_pages_nonexecutable (physical : UInt64)
    (h : userPageEntry physical 2 ≠ 0) :
    (userPageEntry physical 2 &&& 0x8000000000000000) ≠ 0 := by
  unfold userPageEntry at *
  bv_decide

theorem udp_payload_bounded (a b c d : UInt64) (h : netUdp a b c d = true) :
    a ≤ 1208 := by
  unfold netUdp at h
  bv_decide

theorem fragmented_packet_rejected (a b ttl proto ver : UInt64)
    (h : fragment ≠ 0) (h' : fragment ≠ 0x4000) :
    netIpv4 a b fragment ttl proto ver = false := by
  simp [netIpv4, h, h']

theorem bounded_poll (n : UInt64) (h : netBudget n = true) : n < 8 := by
  simpa [netBudget] using h

theorem tcp_header_within_segment (size header : UInt64)
    (h : netTcp size header = true) : 20 ≤ header ∧ header ≤ size := by
  unfold netTcp at h
  bv_decide

theorem zero_port_rejected : netPort 0 = false := by decide

theorem outbound_datagram_bounded (size : UInt64) (h : netPayload size = true) :
    size ≤ 1200 := by simpa [netPayload] using h

theorem console_append_bounded (byte length discarding : UInt64)
    (h : consoleAction byte length discarding = 1) : length < 64 := by
  unfold consoleAction bne at h
  bv_decide

theorem console_append_printable (byte length discarding : UInt64)
    (h : consoleAction byte length discarding = 1) : 32 ≤ byte ∧ byte ≤ 126 := by
  unfold consoleAction bne at h
  bv_decide

theorem rejected_line_cannot_append (byte length discarding : UInt64)
    (h : discarding ≠ 0) : consoleAction byte length discarding ≠ 1 := by
  unfold consoleAction bne
  bv_decide

theorem console_poll_bounded (n : UInt64) (h : consoleBudget n = true) : n < 16 := by
  simpa [consoleBudget] using h

theorem user_copy_bounded (p n b s : UInt64) (h : userBuffer p n b s = true) :
    n ≤ 256 ∧ b ≤ p ∧ p - b ≤ s ∧ n ≤ s - (p - b) := by
  unfold userBuffer at h
  bv_decide

theorem elf_no_writable_executable (a f m flags : UInt64)
    (h : elfSegment a f m flags = true) : flags = 5 ∨ flags = 6 := by
  unfold elfSegment at h
  bv_decide

theorem elf_file_within_memory (a f m flags : UInt64)
    (h : elfSegment a f m flags = true) : f ≤ m := by
  unfold elfSegment at h
  bv_decide

theorem arm_code_readonly (p : UInt64) (h : armPage p 1 ≠ 0) :
    (armPage p 1 &&& 0x80) = 0x80 := by
  unfold armPage bne at *
  bv_decide

theorem arm_data_nonexecutable (p : UInt64) (h : armPage p 2 ≠ 0) :
    (armPage p 2 &&& 0x40000000000000) ≠ 0 := by
  unfold armPage bne at *
  bv_decide

theorem riscv_code_readonly (p : UInt64) : (riscvPage p 1 &&& 4) = 0 := by
  unfold riscvPage bne
  bv_decide

theorem riscv_data_nonexecutable (p : UInt64) : (riscvPage p 2 &&& 8) = 0 := by
  unfold riscvPage bne
  bv_decide

end Bastion.Runtime
