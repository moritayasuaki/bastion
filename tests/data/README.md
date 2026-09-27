# Reference vectors

These committed CSVs allow `cargo xtask check` to run without external source projects.
Each case is evaluated against the linked Lean-generated C during Rust tests.

- `policy-vectors.csv`: 1,254 abstract cases from `proof/Vectors.lean`.
- `runtime-vectors.csv`: 18,046 scalar-policy cases from `proof/RuntimeVectors.lean`.
- `octave-vectors.csv`: 18,944 cases from `tests/OctaveVectors.lean`, importing the
  independent Octave `Relay.Core` reference. It covers every relay label and every
  four-subset, both polynomial shares and arbitrary field inputs.

Generate the first two from the Lean project:

```sh
(cd proof && lake env lean --run Vectors.lean > ../tests/data/policy-vectors.csv)
(cd proof && lake env lean --run RuntimeVectors.lean > ../tests/data/runtime-vectors.csv)
```

The optional Octave source check requires a compatible reference checkout with
`lean/lakefile.toml`, `lean/Relay/Core.lean`, and its Lean/Mathlib dependencies built:

```sh
cargo xtask octave-check --root /path/to/octave
```

It compares freshly generated output with the committed corpus and fails on a mismatch.
The reference `lean/Relay/Core.lean` used for this corpus has SHA-256
`44e1b8530d75a00c378a06def5666fe99f4eb93619ac92fa915949d49cba94f9`.
The external source is not a runtime dependency. Corpus agreement is not a general
refinement proof between the reference and Bastion's scalar implementation.
