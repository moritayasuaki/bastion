# Third-party software

The boot ISO includes unmodified Limine bootloader binaries from
https://github.com/limine-bootloader/limine at commit
`aad3edd370955449717a334f0289dee10e2c5f01`. The build helper compiles
Limine’s host-side ISO installer from the same pinned checkout.
The following notice accompanies distribution of these files:

```text
Copyright (C) 2019-2025 mintsuki and contributors.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

## Lean scalar runtime primitives

The generated `policy/generated/scalars.h` retains selected, unmodified scalar functions from Lean 4.32.1 `include/lean/lean.h`. Copyright (c) 2019 Microsoft Corporation. All rights reserved. Original author: Leonardo de Moura. These functions are licensed under Apache-2.0. The full license is in `policy/generated/LEAN-LICENSE` and is also included at the root of each ISO as `LEAN-LICENSE`. The Lean compiler itself runs only on the build machine.

## PSIV and target cryptography

The kernel includes the unmodified portable C implementation from the [PSIV project](https://github.com/moritayasuaki/psiv), portable backend snapshot 0.4.0. Copyright (c) 2026 PSIV-Lean contributors, MIT licensed. Its license and source hashes are under `crypto/vendor/psiv/`; a copy of the license is in `docs/licenses/PSIV-MIT.txt`. This is handwritten C, distinct from Bastion's Lean-generated policy.

The target also includes RustCrypto `sha2` 0.10.9, `digest` 0.10.7, `block-buffer` 0.10.4, `crypto-common` 0.1.7, `generic-array` 0.14.7, `typenum` 1.20.1, `cfg-if` 1.0.5, and the target dependency `cpufeatures` 0.2.17. Scalar SHA-256 is forced for the kernel. MIT notices are copied under `docs/licenses/`; both ISO variants include that directory as `/licenses/`. The Cargo lockfiles record all dependencies.

Octave's protocol parameters and executable reference follow the Octave project, SPEC v0.2. The optional validation bridge imports an explicitly supplied checkout; the Mathlib-dependent Octave project is not copied into the kernel. The scalar Lean arithmetic is a separate implementation checked against its vectors.

## Rust host tooling

The Rust developer tools use `regex`, `serde`, `serde_json`, `sha2`, `tempfile`, and `libc`, with transitive dependencies recorded in Cargo lockfiles. Cargo obtains these packages under their respective licenses. Host tool binaries and their other dependencies are not included in the boot image.
