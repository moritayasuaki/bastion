//! Vendored PSIV portable C backend, plus a contextual HKDF-SHA256 adapter.
#![no_std]
use sha2::{Digest, Sha256};

#[repr(C)]
struct Context {
    r: [u64; 5],
    pad: [u32; 4],
    tag_key: [u8; 36],
    enc_key: [u8; 36],
    magic: u32,
}
unsafe extern "C" {
    fn psiv_init(ctx: *mut Context, key: *const u8, length: usize) -> i32;
    fn psiv_clear(ctx: *mut Context);
    fn psiv_seal(
        ctx: *const Context,
        nonce: *const u8,
        nonce_len: usize,
        ad: *const u8,
        ad_len: usize,
        input: *const u8,
        input_len: usize,
        out: *mut u8,
        capacity: usize,
    ) -> i32;
    fn psiv_open(
        ctx: *const Context,
        nonce: *const u8,
        nonce_len: usize,
        ad: *const u8,
        ad_len: usize,
        input: *const u8,
        input_len: usize,
        out: *mut u8,
        capacity: usize,
    ) -> i32;
}
pub struct Session(Context);
#[derive(Debug, PartialEq, Eq)]
pub struct Error(pub i32);
impl Session {
    pub fn new(key: &[u8; 32]) -> Result<Self, Error> {
        let mut ctx = Context {
            r: [0; 5],
            pad: [0; 4],
            tag_key: [0; 36],
            enc_key: [0; 36],
            magic: 0,
        };
        // SAFETY: exact repr(C) layout, aligned exclusive context, valid disjoint key.
        let status = unsafe { psiv_init(&mut ctx, key.as_ptr(), key.len()) };
        if status != 0 {
            return Err(Error(status));
        }
        Ok(Self(ctx))
    }
    pub fn seal(
        &self,
        nonce: &[u8; 12],
        ad: &[u8],
        input: &[u8],
        out: &mut [u8],
    ) -> Result<usize, Error> {
        // SAFETY: Rust references guarantee accessible, correctly sized, nonoverlapping buffers.
        let status = unsafe {
            psiv_seal(
                &self.0,
                nonce.as_ptr(),
                12,
                ad.as_ptr(),
                ad.len(),
                input.as_ptr(),
                input.len(),
                out.as_mut_ptr(),
                out.len(),
            )
        };
        if status == 0 {
            Ok(input.len() + 16)
        } else {
            Err(Error(status))
        }
    }
    pub fn open(
        &self,
        nonce: &[u8; 12],
        ad: &[u8],
        input: &[u8],
        out: &mut [u8],
    ) -> Result<usize, Error> {
        // SAFETY: same buffer contract as seal; C only releases authenticated plaintext.
        let status = unsafe {
            psiv_open(
                &self.0,
                nonce.as_ptr(),
                12,
                ad.as_ptr(),
                ad.len(),
                input.as_ptr(),
                input.len(),
                out.as_mut_ptr(),
                out.len(),
            )
        };
        if status == 0 {
            Ok(input.len() - 16)
        } else {
            Err(Error(status))
        }
    }
}
impl Drop for Session {
    fn drop(&mut self) {
        unsafe { psiv_clear(&mut self.0) }
    }
}

// HMAC-SHA256, used only with keys of at most 64 bytes. SHA256 is RustCrypto's
// no_std scalar implementation. HKDF expands one 32-byte block (RFC 5869).
fn hmac(key: &[u8], parts: &[&[u8]]) -> [u8; 32] {
    assert!(key.len() <= 64);
    let mut inner = [0x36; 64];
    let mut outer = [0x5c; 64];
    for (i, b) in key.iter().enumerate() {
        inner[i] ^= b;
        outer[i] ^= b;
    }
    let mut h = Sha256::new();
    h.update(inner);
    for part in parts {
        h.update(part);
    }
    let digest = h.finalize();
    let mut h = Sha256::new();
    h.update(outer);
    h.update(digest);
    h.finalize().into()
}
pub fn derive(root: &[u8; 32], context: &[u8], label: &[u8]) -> [u8; 32] {
    assert!(label.len() < 256);
    let prk = hmac(b"Bastion Octave PSIV adapter v1", &[root]);
    hmac(&prk, &[context, &[label.len() as u8], label, &[1]])
}
pub fn nonce(sequence: u64) -> [u8; 12] {
    let mut n = [0; 12];
    n[4..].copy_from_slice(&sequence.to_le_bytes());
    n
}
#[cfg(test)]
mod tests {
    use super::*;
    unsafe extern "C" {
        fn psiv_ctx_size() -> usize;
        fn psiv_ctx_align() -> usize;
    }
    #[test]
    fn authentication_preserves_output_and_binds_context() {
        // Check the ABI before passing Rust storage into the C implementation.
        unsafe {
            assert_eq!(psiv_ctx_size(), core::mem::size_of::<Context>());
            assert_eq!(psiv_ctx_align(), core::mem::align_of::<Context>());
        }
        let s = Session::new(&[7; 32]).unwrap();
        let mut record = [0; 21];
        s.seal(&nonce(1), b"context", b"hello", &mut record)
            .unwrap();
        let mut out = [0xaa; 5];
        assert!(s.open(&nonce(1), b"other", &record, &mut out).is_err());
        assert_eq!(out, [0xaa; 5]);
        s.open(&nonce(1), b"context", &record, &mut out).unwrap();
        assert_eq!(&out, b"hello");
        record[0] ^= 1;
        assert!(s.open(&nonce(1), b"context", &record, &mut out).is_err());
    }
    #[test]
    fn hmac_known_answer_and_direction_separation() {
        assert_eq!(
            hmac(&[0x0b; 20], &[b"Hi There"]),
            [
                0xb0, 0x34, 0x4c, 0x61, 0xd8, 0xdb, 0x38, 0x53, 0x5c, 0xa8, 0xaf, 0xce, 0xaf, 0x0b,
                0xf1, 0x2b, 0x88, 0x1d, 0xc2, 0x00, 0xc9, 0x83, 0x3d, 0xa7, 0x26, 0xe9, 0x37, 0x6c,
                0x2e, 0x32, 0xcf, 0xf7
            ]
        );
        assert_ne!(
            derive(&[1; 32], b"transcript", b"confirmAB"),
            derive(&[1; 32], b"transcript", b"confirmBA")
        );
    }
}
