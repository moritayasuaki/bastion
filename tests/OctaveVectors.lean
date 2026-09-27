import Relay.Core
open Relay

def row (name : String) (xs : List Nat) : IO Unit :=
  IO.println (String.intercalate "," (name :: xs.map toString))

def main : IO Unit := do
  IO.println "operation,a,b,c,d,e,result"
  for seed in [0:4] do
    let root : RootBytes := fun b => ⟨(seed * 71 + b.val * 17) % 256, Nat.mod_lt _ (by decide)⟩
    let coins : Randomness := fun b j => ((seed*83 + b.val*31 + j.val*113) % 257 : Nat)
    let shares := share (encodeRoot root) coins
    for i in List.finRange 8 do
      for b in List.finRange 32 do
        row "share" [(root b).val,(coins b 0).val,(coins b 1).val,(coins b 2).val,i.val+1,(shares i b).val]
    for variant in [0:2] do
      let received : Shares := if variant == 0 then shares else
        fun i b => ((seed*73 + i.val*17 + b.val*31) % 257 : Nat)
      for subset in allFourSubsets do
        let ids := (List.finRange 8).filter (fun i => decide (i ∈ subset))
        let packed := ids.foldl (fun (state : Nat × Nat) i =>
          (state.1 + (i.val+1)*256^state.2,state.2+1)) (0,0)
        let candidate := reconstruct subset received
        for b in List.finRange 32 do
          row "reconstruct" ([packed.1] ++ (ids.map (fun i => (received i b).val)) ++ [(candidate b).val])
