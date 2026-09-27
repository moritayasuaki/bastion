/* Experimental ChaCha20-Poly1305-PSIV. Not audited or formally verified. */
#ifndef PSIV_H
#define PSIV_H
#include <stddef.h>
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif
#define PSIV_KEY_BYTES 32u
#define PSIV_NONCE_BYTES 12u
#define PSIV_TAG_BYTES 16u
#define PSIV_MAX_MESSAGE 65536u
#define PSIV_MAX_AD 65536u
#define PSIV_VERSION "0.4.0-experimental"
enum { PSIV_OK=0, PSIV_INVALID=-1, PSIV_AUTH=-2, PSIV_LIMIT=-3,
       PSIV_CAPACITY=-4, PSIV_OVERLAP=-5 };
/* Secret, non-serializable, immutable between init and clear. Do not modify fields.
   Concurrent readers are allowed; initialization/destruction need exclusive access.
   An initialized context must not be copied into persistent storage. */
typedef struct {
    uint64_t r[5];
    uint32_t pad[4];
    uint8_t tag_key[36], enc_key[36];
    uint32_t magic;
} psiv_ctx;
size_t psiv_ctx_size(void);
size_t psiv_ctx_align(void);
int psiv_init(psiv_ctx *ctx, const uint8_t *key, size_t key_len);
void psiv_clear(psiv_ctx *ctx);
void psiv_wipe(void *p, size_t n);
/* All pointer/length pairs must describe real, accessible C objects. Raw pointer
   validity cannot be verified by this API. Null is allowed only for zero lengths.
   Outputs must not overlap ctx, nonce, or AD. Exact message/output aliasing is
   supported; other message/output overlap is rejected. Context must be initialized.
   seal writes ciphertext || 16-byte tag. open writes ONLY authenticated plaintext.
   On every error the output is unchanged. Exact in-place open leaves the old tag
   after the plaintext; this tail is not part of the result. No nonce is prepended.
   Return values are status codes, not lengths. No allocation, I/O, or global state.
   Record limit is a local engineering policy, NOT a proven per-key usage bound. */
int psiv_seal(const psiv_ctx *ctx, const uint8_t *nonce, size_t nonce_len,
              const uint8_t *ad, size_t ad_len, const uint8_t *msg, size_t msg_len,
              uint8_t *out, size_t out_capacity);
int psiv_open(const psiv_ctx *ctx, const uint8_t *nonce, size_t nonce_len,
              const uint8_t *ad, size_t ad_len, const uint8_t *record, size_t record_len,
              uint8_t *out, size_t out_capacity);
int psiv_selftest(void);
#ifdef __cplusplus
}
#endif
#endif
