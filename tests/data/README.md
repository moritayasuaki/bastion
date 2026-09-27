# Policy conformance fixtures

- `policy-vectors.csv`: 1,254 cases from the abstract Lean policy model.
- `runtime-vectors.csv`: 22,309 cases across every executable Lean policy export,
  including invalid inputs, word boundaries, scheduler masks and network bounds.

The tests compare these results with actual linked C execution. `cargo xtask check`
regenerates both corpora and verifies exact agreement. To update after reviewing a
policy change:

```sh
cargo xtask extract
(cd proof && lake build && lake env lean --run Vectors.lean > ../tests/data/policy-vectors.csv)
(cd proof && lake env lean --run RuntimeVectors.lean > ../tests/data/runtime-vectors.csv)
cargo xtask check
```

Fixture agreement is not a proof of compiler or whole-kernel correctness.
