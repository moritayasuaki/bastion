//! Safe scalar wrappers around the actual Lean-generated C implementation.
//! The kernel core remains safe Rust; this crate owns the narrow FFI boundary.
#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/generated/bindings.rs"
));
