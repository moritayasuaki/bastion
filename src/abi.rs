//! Bastion native ABI v1. All records contain little-endian u64 words.
pub const VERSION: u64 = 1;
pub const ABI: u64 = 64;
pub const READ: u64 = 65;
pub const WRITE: u64 = 66;
pub const QUERY: u64 = 67;
pub const CONSOLE: u64 = 1;
pub const STATUS: u64 = 2;
pub const DENIED: u64 = u64::MAX;
pub const ADDRESS: u64 = u64::MAX - 1;
pub const AGAIN: u64 = u64::MAX - 2;
pub const INVALID: u64 = u64::MAX - 3;
pub const INPUT_ERROR: u64 = u64::MAX - 4;
pub const MAX_COPY: usize = 256;
pub const WORDS: usize = 16;
pub const RECORD_BYTES: usize = WORDS * 8;
pub const CODE: u64 = 0x400000;
pub const DATA: u64 = 0x500000;
pub const REGION_SIZE: usize = 65536;
pub const STACK_TOP: u64 = DATA + REGION_SIZE as u64;

/// Translate a checked buffer into an offset in the caller's private backing store.
/// This profile maps a contiguous RW/NX data region, including the stack.
pub fn buffer_offset(pointer: u64, length: u64, mapped_size: u64) -> Option<usize> {
    (crate::decisions::user_buffer(pointer, length, DATA, mapped_size) != 0)
        .then(|| (pointer - DATA) as usize)
}
