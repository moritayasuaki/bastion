import Bastion.Runtime

open Bastion.Runtime

def emit (name : String) (args : List UInt64) (result : UInt64) : IO Unit := do
  let padded := args ++ List.replicate (6 - args.length) 0
  IO.println (String.intercalate "," (name :: (padded.map toString) ++ [toString result]))

def truth (b : Bool) : UInt64 := if b then 1 else 0

def main : IO Unit := do
  IO.println "operation,a,b,c,d,e,f,result"
  let values : List UInt64 := [0,1,2,3,7,8,9,99,100,101,0xfff,0x1000,
    0x7fffffffffff,0x800000000000,0xfffffffffffff,0x10000000000000,
    18446744073709551614,18446744073709551615]
  for a in values do
    emit "abi_version" [a] (abiVersion a)
    emit "next_identifier" [a] (nextIdentifier a)
    emit "next_cursor" [a] (nextCursor a)
    emit "user_flags" [a] (userFlags a)
    emit "supervisor_entry" [a] (supervisorEntry a)
    emit "syscall_opcode" [a] (syscallOpcode a)
    for k in [0,1,2,3] do emit "user_page_entry" [a,k] (userPageEntry a k)
    for b in values do
      emit "valid_config" [a,b] (truth (validConfig a b))
      emit "can_release" [a,b] (truth (canRelease a b))
      emit "release_value" [a,b] (releaseValue a b)
      emit "same_identity" [a,b] (truth (sameIdentity a b))
      emit "can_grant" [a,b] (truth (canGrant a b))
      emit "rights_allow" [a,b] (truth (rightsAllow a b))
      emit "restrict_rights" [a,b] (restrictRights a b)
      emit "revoke_target" [a,b] (truth (revokeTarget a b))
      emit "charge" [a,b] (charge a b)
      emit "clock_valid" [a,b] (truth (clockValid a b))
      emit "runnable" [a,b] (truth (runnable a b))
      emit "valid_user_return" [a,b] (truth (validUserReturn a b))
      for c in [0,8,18446744073709551615] do
        emit "can_reserve" [a,b,c] (truth (canReserve a b c))
        emit "reserve_value" [a,b,c] (reserveValue a b c)
      for caps in [0,8,9] do
        emit "valid_limits" [a,b,caps,3,10] (truth (validLimits a b caps 3 10))
  for mask in [0:256] do
    for cursor in [0:9] do
      emit "next_slot" [mask.toUInt64,cursor.toUInt64] (nextSlot mask.toUInt64 cursor.toUInt64)
  for cursor in values do
    emit "next_slot" [18446744073709551615,cursor] (nextSlot 18446744073709551615 cursor)
  for owned in [0:10] do
    for total in [0:18] do
      for request in [0:10] do
        let a := owned.toUInt64; let b := total.toUInt64; let c := request.toUInt64
        emit "reservation_status" [a,b,c,8,16] (reservationStatus a b c 8 16)
  for held in [0:6] do
    for needed in [0:6] do
      for (caller, owner) in [(1,1),(1,2),(0,0),(18446744073709551615,18446744073709551615)] do
        for live in [0,1,2] do
          emit "authorized" [caller,owner,live,held.toUInt64,needed.toUInt64]
            (truth (authorized caller owner live held.toUInt64 needed.toUInt64))
  for remaining in [0,1,2,100,18446744073709551615] do
    for budget in [0,1,3,100,18446744073709551615] do
      for (old, now) in [(0,0),(0,2),(2,1),(3,10),(3,29),(99,100),
          (18446744073709551614,18446744073709551615),(0,18446744073709551615)] do
        for period in [0,1,10,100,18446744073709551615] do
          for active in [0,1] do
            emit "account" [remaining,budget,old,now,period,active]
              (account remaining budget old now period active)
            emit "deadline" [now,period,budget,remaining,active]
              (deadline now period budget remaining active)
  for a in [0,1,7,8,13,14,27,28,255,256,257,1207,1208,1209,1228,1514,1515,65535,18446744073709551615] do
    emit "net_frame_len" [a] (truth (netFrameLen a))
    emit "net_budget" [a] (truth (netBudget a))
    for b in [0,8,1208,1514,18446744073709551615] do
      emit "net_udp" [a,b,9000,9000] (truth (netUdp a b 9000 9000))
      emit "net_udp" [a,b,9001,9000] (truth (netUdp a b 9001 9000))
      for f in [0,0x4000,0x2000,1,0x8000] do
        emit "net_ipv4" [a,b,f,64,17,0x45] (truth (netIpv4 a b f 64 17 0x45))
    emit "net_port" [a] (truth (netPort a))
    emit "net_payload" [a] (truth (netPayload a))
    for header in [0,19,20,24,60,61,1480,18446744073709551615] do
      emit "net_tcp" [a,header] (truth (netTcp a header))
    for proto in [0,1,6,17,255] do
      for version in [0,0x45,0x46] do
        emit "net_ipv4" [a,1500,0,64,proto,version] (truth (netIpv4 a 1500 0 64 proto version))
  for byte in [0:257] do
    for length in [0,1,63,64,18446744073709551615] do
      for discarding in [0,1,2] do
        emit "console_action" [byte.toUInt64,length,discarding]
          (consoleAction byte.toUInt64 length discarding)
  for n in [0,1,15,16,17,18446744073709551615] do
    emit "console_budget" [n] (truth (consoleBudget n))
