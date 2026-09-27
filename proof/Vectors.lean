import Bastion

-- Executable Lean specification produces deterministic Rust conformance cases.
def main : IO Unit := do
  IO.println "operation,a,b,c,result"
  for limit in [0:9] do
    for used in [0:11] do
      for requested in [0:11] do
        let result := match Bastion.reserve used requested limit with
          | some n => toString n
          | none => "denied"
        IO.println s!"reserve,{used},{requested},{limit},{result}"
  for remaining in [0:11] do
    for elapsed in [0:15] do
      IO.println s!"charge,{remaining},{elapsed},0,{Bastion.charge remaining elapsed}"
