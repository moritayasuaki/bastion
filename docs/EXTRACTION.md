# Lean → C → Rust → native kernel

The executable policy source is `proof/Bastion/Runtime.lean`. Its 36 `@[export ...]` definitions provide stable C entry points. Rust invokes those functions through generated scalar bindings; resource and authorization decisions are no longer duplicated as handwritten Rust predicates.

## Areas implemented in Lean

| Area | Lean decisions |
|---|---|
| Admission | Kernel configuration, process limits, per-process/global memory reservation status |
| Resource updates | Accepted reserve/release values, rejection preserving state, capability-table admission |
| Identity/lifecycle | Fresh identifier calculation, identity lookup predicate, target revocation predicate |
| Authority | Rights subsets, caller/owner/liveness authorization, rights restriction |
| CPU | Clock validation, charging, per-period accounting, runnable status, next slot/cursor, deadlines |
| x86 protection | User return addresses, safe flags, supervisor-only mappings, RX/RW-NX page entries |
| Console | Input action, printable-character/line bounds, polling budget |
| Entry filtering | Supported syscall opcode and policy ABI version |
| Network | Frame/IP/UDP/TCP bounds, fragment rejection, poll budget, ports, outbound datagram limit |

Rust still owns arrays, exclusive access, capability lookup, transaction ordering, physical frame placement, and register/interrupt operations. The complete kernel state machine is not compiled from Lean in this milestone. Extending Lean to own arrays or higher-order state would require a broader runtime strategy or another explicitly checked representation.

## Compiler and subset adapter

`tools/src/extract.rs` invokes the pinned Lean 4.32.1 compiler:

```sh
cd proof
lean -DwarningAsError=true -c /path/to/Runtime.full.c Bastion/Runtime.lean
```

This is Lean's actual C backend. The complete result is retained as `policy/generated/Runtime.full.c`. General Lean code uses runtime object representation, allocation, reference counting, and module initialization. This module deliberately restricts the kernel ABI to `UInt64` and `Bool` and uses no heap-backed values or initialization state. See the official [compilation documentation](https://lean-lang.org/doc/reference/latest/Elaboration-and-Compilation/) and [runtime documentation](https://lean-lang.org/doc/reference/latest/Run-Time-Code/).

The generated full module includes unused boxed wrappers and an initializer. The subset adapter finds the scalar exports and their transitive dependencies, checks every call against defined scalar functions or an explicit primitive allowlist, and copies the reachable function bodies **verbatim** into `policy.c`. It fails on an unsupported ABI, missing definition, object/global dependency, or unknown runtime call. It does not translate the decision logic a second time.

The required ten scalar primitives are copied verbatim from that same Lean toolchain's `include/lean/lean.h` into `scalars.h`, preserving Lean's unsigned overflow, division-by-zero, and shift semantics. Their original license accompanies the source and ISO. No replacement allocator, fake boxed objects, or stubbed runtime initializers are linked.

`policy/generated/manifest.json` records the Lean version, source and compiler-output hashes, all exported/reachable functions, primitive allowlist, and generated-file hashes. `cargo xtask extract --check` regenerates all artifacts and compares them byte-for-byte. Every Cargo build also checks saved source/artifact hashes and rejects stale output.

The extraction, build, audit, and smoke-test tools are implemented in Rust. `policy/build.rs` calls the shared `bastion-integrity` Rust library directly, without launching an interpreter. The verifier requires the complete expected artifact set and rejects changed files or inconsistent hashes. These hashes check freshness, not authenticity against someone able to rewrite both sources and manifests.

## Files to inspect or reuse

| File | Contents |
|---|---|
| `policy/generated/Runtime.full.c` | Unmodified full Lean compiler output, including unused runtime wrappers |
| `policy/generated/policy.c` | Reachable freestanding function bodies used by the kernel |
| `policy/generated/scalars.h` | Required scalar primitive implementations from Lean |
| `policy/generated/policy.h` | C API declarations |
| `policy/generated/bindings.rs` | Typed Rust FFI plus safe integer-only wrappers |
| `policy/generated/manifest.json` | Provenance and freshness hashes |
| `policy/generated/LEAN-LICENSE` | License for extracted runtime primitives |

To use the policy from another C program, compile `policy.c` and include `policy.h`; no Lean runtime initialization is needed for these selected exports. The full untrimmed module has different linkage requirements and is retained for audit, not passed directly to the freestanding compiler.

## Compilation and linkage

The `bastion-policy` crate uses `policy/build.rs` to invoke Clang. For `x86_64-unknown-none`, C is compiled as freestanding x86-64 with no red zone, SSE/MMX, stack protector, or libc builtins. Lean's bundled `llvm-ar` writes a GNU-format archive for the bare-metal target and a Darwin-format archive for macOS host tests. `BASTION_CC` and `BASTION_AR` can override Clang and llvm-ar.

The Rust kernel links that static archive. `tools/src/audit.rs` reads the resulting ELF symbol tables and requires all 36 exports in the C object, no undefined object symbols, no boxed/heap runtime, and the critical policy symbols in the final kernel. The linker discards unused exports; 35 policy symbols survive in the current boot image.

The C functions operate only on passed scalar values and do not touch global state. Rust wrappers can expose them safely for all representable integer arguments. The storage/interrupt synchronization rules remain those documented in `ARCHITECTURE.md`.

## Validation and proof scope

`RuntimeProofs.lean` checks properties of the definitions actually sent to the compiler. Reservation and charging are connected to the natural-number model; other proofs cover rights, bounded scheduler output, user flags, and page protection bits. `RuntimeVectors.lean` executes all exports in Lean and emits 20,577 cases, which the Rust integration tests compare to the linked C implementation. The earlier 1,254 abstract model cases remain as an independent model check.

The Lean compiler, C compiler, subset adapter, scalar runtime primitives, FFI ABI, Rust state updates, and hardware layer remain in the trusted computing base. There is no proof that the subset adapter or machine-code compiler preserves all semantics, nor a whole-kernel verification claim. The fresh-generation checks, symbol audits, cross-language cases, and BIOS/UEFI attack tests provide integration evidence with that explicit scope.

Six executable network-policy theorems cover UDP bounds, fragment rejection, polling
limits, TCP header bounds, zero-port rejection, and outbound datagram limits. TCP
state transitions, retransmission and packet parsing are trusted Rust code from
smoltcp; these are not compiled from Lean or covered by the policy proofs.

## Editing workflow

1. Change `Bastion/Runtime.lean` and its proofs; keep exported ABI values scalar.
2. Run `cargo xtask extract` from the project root.
3. Regenerate runtime vectors with `lake env lean --run RuntimeVectors.lean` from `proof/`.
4. Run `cargo xtask check`, then build and smoke-test the image.
5. Review changes to the Lean source, compiler output, primitive set, generated ABI, and provenance together.

An extraction failure is a request to review the representation/runtime boundary. Do not replace rejected Lean logic with a handwritten C implementation merely to make the build pass.
