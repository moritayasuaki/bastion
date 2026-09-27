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

## Rust network stack

The kernel links `smoltcp` 0.14.0 and `managed` 0.8.0 (0BSD), plus `heapless`
0.9.3, `hash32` 0.3.1, `byteorder` 1.5.0, `stable_deref_trait` 1.2.1 and
`bitflags` 1.3.2 (MIT option). Their license texts are copied under `docs/licenses/`
and included in boot images at `/licenses/`. Default features, heap allocation,
asynchronous runtimes and host networking are disabled for smoltcp. Cargo lockfiles
record the versions and registry checksums. Lean proves selected admission policies;
the network protocol stack remains part of the trusted Rust implementation.

## Rust host tooling

The Rust developer tools use `regex`, `serde`, `serde_json`, `sha2`, `tempfile`, and `libc`, with transitive dependencies recorded in Cargo lockfiles. Cargo obtains these packages under their respective licenses. Host tool binaries and their other dependencies are not included in the boot image.
