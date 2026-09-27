import Std

/-! Abstract policy specification. These theorems cover the functions below,
not the Rust compiler, its output, trap assembly, or physical memory protection.
Natural numbers model nonnegative resources; reserve_word_bound connects a
successful reservation to the Rust u64 bound when its limit fits in u64. -/
namespace Bastion

def reserve (used requested limit : Nat) : Option Nat :=
  if used ≤ limit ∧ requested ≤ limit - used then some (used + requested) else none

theorem reserve_safe (h : reserve used requested limit = some result) :
    result ≤ limit := by
  unfold reserve at h
  split at h <;> simp_all <;> omega

theorem reserve_exact (h : reserve used requested limit = some result) :
    result = used + requested := by
  unfold reserve at h
  split at h <;> simp_all

theorem reserve_word_bound (hl : limit ≤ 2^64 - 1)
    (h : reserve used requested limit = some result) : result ≤ 2^64 - 1 := by
  have := reserve_safe h
  omega

theorem reserve_rejects_excess (h : limit < used + requested) :
    reserve used requested limit = none := by
  unfold reserve
  split <;> simp_all <;> omega

def release (used amount : Nat) : Option Nat :=
  if amount ≤ used then some (used - amount) else none

theorem release_conserves (h : release used amount = some result) :
    result + amount = used := by
  unfold release at h
  split at h <;> simp_all <;> omega

def charge (remaining elapsed : Nat) : Nat := remaining - elapsed

theorem charge_never_increases : charge remaining elapsed ≤ remaining := by
  unfold charge
  omega

-- Splitting execution across yields or syscalls gives no extra CPU credit.
theorem charge_split : charge (charge remaining a) b = charge remaining (a + b) := by
  unfold charge
  omega

def account (remaining budget old now period : Nat) : Nat :=
  if old / period = now / period then charge remaining (now - old)
  else charge budget (now % period)

theorem account_bounded (h : remaining ≤ budget) :
    account remaining budget old now period ≤ budget := by
  unfold account charge
  split <;> omega

structure Rights where
  inspect : Bool
  terminate : Bool
  deriving DecidableEq, Repr

def Allows (held requested : Rights) : Prop :=
  (requested.inspect = true → held.inspect = true) ∧
  (requested.terminate = true → held.terminate = true)

instance (held requested : Rights) : Decidable (Allows held requested) :=
  inferInstanceAs (Decidable ((_ → _) ∧ (_ → _)))

def restrict (held requested : Rights) : Option Rights :=
  if Allows held requested then some requested else none

theorem restrict_no_amplification (h : restrict held requested = some result) :
    Allows held result := by
  unfold restrict at h
  split at h <;> simp_all

def authorized (caller owner : Nat) (live : Bool) (held needed : Rights) : Prop :=
  caller = owner ∧ live = true ∧ Allows held needed

theorem other_owner_denied (h : caller ≠ owner) :
    ¬ authorized caller owner live held needed := by
  intro ha
  exact h ha.1

theorem dead_target_denied : ¬ authorized caller owner false held needed := by
  simp [authorized]

end Bastion
