# Contributing

Keep changes small enough to review and explain the behavior being changed.
Include a relevant regression test for changes to isolation, parsing, or transport.

Run `cargo xtask check`. For kernel/driver changes, also build and run the boot tests;
for userspace/port changes, run the console and both `test-port` suites;
for network changes, run `cargo xtask network-test`. See [validation](docs/VALIDATION.md).

After changing executable Lean policy:

```sh
cargo xtask extract
(cd proof && lake build && lake env lean --run RuntimeVectors.lean > ../tests/data/runtime-vectors.csv)
cargo xtask check
```

Review Lean source, generated C/bindings, provenance, and vectors together. Do not
edit generated policy by hand or introduce admitted proofs/custom axioms. Keep the
freestanding scalar ABI explicit; unsupported runtime dependencies must fail extraction.

Keep network buffers and work bounded, preserve TCP stream bytes under backpressure,
and test malformed inputs and retransmission. Do not expose kernel socket handles
to userspace. Keep policy proofs distinct from protocol and hardware test coverage.

Never commit credentials, captures, build caches, or local
reports. Third-party source changes must retain licensing and update provenance.
Contributions to original project code are under the repository's MIT license.
