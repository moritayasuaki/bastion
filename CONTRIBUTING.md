# Contributing

Keep changes small enough to review and explain the behavior being changed.
Include a relevant regression test for changes to isolation, parsing, or authentication.

Run `cargo xtask check`. For kernel/driver changes, also build and run the boot tests;
for network changes, run the relay demo. See [validation](docs/VALIDATION.md).

After changing executable Lean policy:

```sh
cargo xtask extract
(cd proof && lake build && lake env lean --run RuntimeVectors.lean > ../tests/data/runtime-vectors.csv)
cargo xtask check
```

Review Lean source, generated C/bindings, provenance, and vectors together. Do not
edit generated policy by hand or introduce admitted proofs/custom axioms. Keep the
freestanding scalar ABI explicit; unsupported runtime dependencies must fail extraction.

Preserve Octave's fixed k=4, n=8, t=3, f=1 profile, canonical field encoding, and
unique-value acceptance. Keep PSIV's 32-byte key, 12-byte nonce, and 16-byte tag.
Do not present conditional security assumptions or test agreement as a complete proof.

Never commit credentials, private relay modules, captures, build caches, or local
reports. Third-party source changes must retain licensing and update provenance.
Contributions to original project code are under the repository's MIT license.
