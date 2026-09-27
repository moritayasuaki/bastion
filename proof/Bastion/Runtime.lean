/-! Executable kernel decisions. The scalar-only interface is deliberate:
UInt64 and Bool exports compile to an allocation-free C ABI. No Nat, Array,
String, closure, IO, or boxed object crosses the kernel interface. -/
namespace Bastion.Runtime

@[export bastion_abi_version]
def abiVersion (_ : UInt64) : UInt64 := 1

@[export bastion_valid_config]
def validConfig (period quantum : UInt64) : Bool :=
  period > 0 && quantum > 0 && quantum ≤ period

@[export bastion_valid_limits]
def validLimits (pageLimit initialPages capLimit cpuBudget period : UInt64) : Bool :=
  capLimit ≤ 8 && cpuBudget > 0 && cpuBudget ≤ period && initialPages ≤ pageLimit

@[export bastion_can_reserve]
def canReserve (used requested limit : UInt64) : Bool :=
  used ≤ limit && requested ≤ limit - used

-- Status codes preserve the precise reason for rejecting an atomic reservation.
-- 0 = success, 1 = process quota, 2 = global quota.
@[export bastion_reservation_status]
def reservationStatus (owned total requested processLimit globalLimit : UInt64) : UInt64 :=
  if !canReserve owned requested processLimit then 1
  else if !canReserve total requested globalLimit then 2 else 0

@[export bastion_reserve_value]
def reserveValue (used requested limit : UInt64) : UInt64 :=
  if canReserve used requested limit then used + requested else used

@[export bastion_can_release]
def canRelease (used amount : UInt64) : Bool := amount ≤ used

@[export bastion_release_value]
def releaseValue (used amount : UInt64) : UInt64 :=
  if canRelease used amount then used - amount else used

@[export bastion_next_identifier]
def nextIdentifier (current : UInt64) : UInt64 :=
  if current == 0 || current == 18446744073709551615 then 0 else current + 1

@[export bastion_same_identity]
def sameIdentity (stored requested : UInt64) : Bool := stored != 0 && stored == requested

@[export bastion_can_grant]
def canGrant (count limit : UInt64) : Bool := count < limit && limit ≤ 8

@[export bastion_rights_allow]
def rightsAllow (held needed : UInt64) : Bool :=
  held ≤ 3 && needed ≤ 3 && (held &&& needed) == needed

@[export bastion_authorized]
def authorized (caller owner targetLive held needed : UInt64) : Bool :=
  sameIdentity caller owner && targetLive == 1 && rightsAllow held needed

-- 4 is an invalid rights value and means restriction was rejected.
@[export bastion_restrict_rights]
def restrictRights (held requested : UInt64) : UInt64 :=
  if rightsAllow held requested then requested else 4

@[export bastion_revoke_target]
def revokeTarget (capTarget removed : UInt64) : Bool := sameIdentity capTarget removed

@[export bastion_charge]
def charge (remaining elapsed : UInt64) : UInt64 :=
  if elapsed ≤ remaining then remaining - elapsed else 0

@[export bastion_clock_valid]
def clockValid (old now : UInt64) : Bool := old ≤ now

@[export bastion_account]
def account (remaining budget old now period active : UInt64) : UInt64 :=
  if period == 0 || !clockValid old now then 0
  else
    let available := min remaining budget
    if old / period == now / period then
      if active == 1 then charge available (now - old) else available
    else if active == 1 then charge budget (now % period) else budget

@[export bastion_runnable]
def runnable (alive remaining : UInt64) : Bool := alive == 1 && remaining > 0

@[inline] def eligible (mask index : UInt64) : Bool :=
  (mask &&& (1 <<< index)) != 0

-- Explicitly bounded scan: no allocation, recursion, or unbounded traversal.
-- 8 means idle / no valid slot. Bits above bit 7 confer no runnable slot.
@[export bastion_next_slot]
def nextSlot (mask cursor : UInt64) : UInt64 :=
  if cursor ≥ 8 then 8 else
  let i0 := cursor
  let i1 := (cursor + 1) % 8
  let i2 := (cursor + 2) % 8
  let i3 := (cursor + 3) % 8
  let i4 := (cursor + 4) % 8
  let i5 := (cursor + 5) % 8
  let i6 := (cursor + 6) % 8
  let i7 := (cursor + 7) % 8
  if eligible mask i0 then i0 else
  if eligible mask i1 then i1 else
  if eligible mask i2 then i2 else
  if eligible mask i3 then i3 else
  if eligible mask i4 then i4 else
  if eligible mask i5 then i5 else
  if eligible mask i6 then i6 else
  if eligible mask i7 then i7 else 8

@[export bastion_next_cursor]
def nextCursor (selected : UInt64) : UInt64 :=
  if selected < 8 then (selected + 1) % 8 else 0

@[export bastion_deadline]
def deadline (now period quantum remaining active : UInt64) : UInt64 :=
  if period == 0 then now else
  let toPeriod := period - now % period
  let delta := if active == 1 then min quantum (min toPeriod remaining) else toPeriod
  if delta ≤ 18446744073709551615 - now then now + delta else 18446744073709551615

-- Validate user return state before IRET executes in ring 0.
@[export bastion_valid_user_return]
def validUserReturn (rip rsp : UInt64) : Bool :=
  rip < 0x0000800000000000 && rsp < 0x0000800000000000

@[export bastion_user_flags]
def userFlags (flags : UInt64) : UInt64 := (flags &&& 0xcd5) ||| 0x202

-- Preserve only supervisor access in inherited upper-half PML4 entries.
@[export bastion_supervisor_entry]
def supervisorEntry (entry : UInt64) : UInt64 := entry &&& 0xfffffffffffffffb

-- 0 = invalid/unmapped, 1 = RX user code, 2 = RW/NX user data.
-- Reject misaligned, out-of-range physical frames and unsupported permissions.
@[export bastion_user_page_entry]
def userPageEntry (physical kind : UInt64) : UInt64 :=
  if (physical &&& 0xfff) != 0 || physical ≥ 0x0010000000000000 then 0
  else if kind == 1 then physical ||| 5
  else if kind == 2 then physical ||| 0x8000000000000007 else 0

-- Registers are untrusted. 255 is an invalid opcode; 5 is a bounded demo report.
@[export bastion_syscall_opcode]
def syscallOpcode (raw : UInt64) : UInt64 := if raw ≤ 5 then raw else 255

-- Bounded network admission; this first profile accepts IPv4 without options or fragments.
@[export bastion_net_frame_len]
def netFrameLen (size : UInt64) : Bool := 14 ≤ size && size ≤ 1514

@[export bastion_net_ipv4]
def netIpv4 (size available fragment ttl protocol version : UInt64) : Bool :=
  version == 0x45 && (protocol == 17 || protocol == 6) && ttl > 0 && ttl ≤ 255 &&
  (fragment == 0 || fragment == 0x4000) && 20 ≤ size && size ≤ available && size ≤ 1500

@[export bastion_net_udp]
def netUdp (size available destination bound : UInt64) : Bool :=
  8 ≤ size && size ≤ 1208 && size == available && bound > 0 && bound ≤ 65535 &&
  destination == bound

@[export bastion_net_budget]
def netBudget (processed : UInt64) : Bool := processed < 8

@[export bastion_net_tcp]
def netTcp (size header : UInt64) : Bool :=
  20 ≤ header && header ≤ 60 && header ≤ size && size ≤ 1480

-- Trusted socket setup and datagram sends share the same bounded policy.
@[export bastion_net_port]
def netPort (port : UInt64) : Bool := port > 0 && port ≤ 65535

@[export bastion_net_payload]
def netPayload (size : UInt64) : Bool := size ≤ 1200

-- Serial input actions: 0 ignore, 1 append, 2 erase, 3 submit, 4 cancel,
-- 5 reject the whole line, 6 clear. Invalid/overlong lines cannot be truncated
-- into executable commands. Ctrl-C/Ctrl-U or a line ending recover input.
@[export bastion_console_action]
def consoleAction (byte length discarding : UInt64) : UInt64 :=
  if byte == 13 || byte == 10 then 3
  else if byte == 3 then 4
  else if byte == 21 then 6
  else if discarding != 0 then 0
  else if byte == 8 || byte == 127 then (if length > 0 && length ≤ 64 then 2 else 0)
  else if 32 ≤ byte && byte ≤ 126 && length < 64 then 1
  else 5

@[export bastion_console_budget]
def consoleBudget (processed : UInt64) : Bool := processed < 16

-- User copy bounds use subtraction after ordering, never wrapping pointer+length.
@[export bastion_user_buffer]
def userBuffer (pointer length base size : UInt64) : Bool :=
  length ≤ 256 && base ≤ pointer && pointer - base ≤ size &&
  length ≤ size - (pointer - base) && size ≤ 65536

-- ELF profile: RX code (including read-only constants), RW/NX data, fixed windows.
-- The upper 32 KiB of data is reserved for the initial stack.
@[export bastion_elf_segment]
def elfSegment (address fileSize memorySize flags : UInt64) : Bool :=
  memorySize > 0 && fileSize ≤ memorySize &&
  ((flags == 5 && 0x400000 ≤ address && address < 0x410000 && memorySize ≤ 0x410000 - address) ||
   (flags == 6 && 0x500000 ≤ address && address < 0x508000 && memorySize ≤ 0x508000 - address))

-- Hardware-specific leaf encodings share the same RX / RW-NX policy.
@[export bastion_arm_page]
def armPage (physical kind : UInt64) : UInt64 :=
  if (physical &&& 0xfff) != 0 || physical ≥ 0x1000000000000 then 0
  else if kind == 1 then physical ||| 0x200000000007c7
  else if kind == 2 then physical ||| 0x60000000000747 else 0

@[export bastion_riscv_page]
def riscvPage (physical kind : UInt64) : UInt64 :=
  if (physical &&& 0xfff) != 0 || physical ≥ 0x100000000000000 then 0
  else if kind == 1 then (physical >>> 2) ||| 0x5b
  else if kind == 2 then (physical >>> 2) ||| 0xd7 else 0

end Bastion.Runtime
