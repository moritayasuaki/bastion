/* Hand-written portable C companion to PSIV/Model.lean -- NOT Lean-generated.
   State layout follows the authors' public-domain reference implementation:
   https://github.com/MichielVerbauwhede/ChaCha20-Poly1305-PSIV
   This implementation is experimental. See docs/SAFETY.md for actual evidence.
   Fixed-limb Poly1305; no secret-indexed lookup tables or secret-dependent loop
   bounds are intended. This is not a compiled-code constant-time proof. */
#include "psiv.h"
#define MASK26 UINT64_C(0x3ffffff)
#define CTX_MAGIC UINT32_C(0x50534956)
static void copy_bytes(uint8_t *d, const uint8_t *s, size_t n) {
    for(size_t i=0;i<n;i++) d[i]=s[i];
}
void psiv_wipe(void *p, size_t n) {
    volatile uint8_t *q=(volatile uint8_t *)p;
    if(q) while(n--) *q++=0;
}
static uint32_t load32(const uint8_t *p) {
    return (uint32_t)p[0] | (uint32_t)p[1]<<8 | (uint32_t)p[2]<<16 | (uint32_t)p[3]<<24;
}
static uint64_t load64(const uint8_t *p) {
    return (uint64_t)load32(p) | (uint64_t)load32(p+4)<<32;
}
static void store32(uint8_t *p, uint32_t v) {
    for(unsigned i=0;i<4;i++) p[i]=(uint8_t)(v>>(8*i));
}
static void store64(uint8_t *p, uint64_t v) {
    for(unsigned i=0;i<8;i++) p[i]=(uint8_t)(v>>(8*i));
}
static uint32_t rotl(uint32_t x, unsigned n) { return (x<<n)|(x>>(32-n)); }
#define QR(a,b,c,d) do { \
 x[a]+=x[b]; x[d]=rotl(x[d]^x[a],16); \
 x[c]+=x[d]; x[b]=rotl(x[b]^x[c],12); \
 x[a]+=x[b]; x[d]=rotl(x[d]^x[a],8); \
 x[c]+=x[d]; x[b]=rotl(x[b]^x[c],7); \
} while(0)
static void chacha_core(uint8_t out[64], const uint8_t in[64]) {
    uint32_t x[16], initial[16];
    for(unsigned i=0;i<16;i++) x[i]=initial[i]=load32(in+4*i);
    for(unsigned round=0;round<10;round++) {
        QR(0,4,8,12); QR(1,5,9,13); QR(2,6,10,14); QR(3,7,11,15);
        QR(0,5,10,15); QR(1,6,11,12); QR(2,7,8,13); QR(3,4,9,14);
    }
    for(unsigned i=0;i<16;i++) store32(out+4*i,x[i]+initial[i]);
    psiv_wipe(x,sizeof x); psiv_wipe(initial,sizeof initial);
}
#undef QR
/* First row deliberately mixes key bytes and domain bytes; do not replace it
   with RFC 8439 constants. Remaining key bytes occupy offsets 16 through 35. */
static void domain_key(uint8_t out[36], const uint8_t key[32], unsigned domain) {
    static const uint8_t domains[3][4]={{3,12,48,192},{5,10,80,160},{6,9,96,144}};
    for(unsigned i=0;i<3;i++) {
        for(unsigned j=0;j<3;j++) out[4*i+j]=key[4*i+j];
        out[4*i+3]=domains[domain][i];
    }
    out[12]=key[3]; out[13]=key[7]; out[14]=key[11]; out[15]=domains[domain][3];
    copy_bytes(out+16,key+12,20);
}
static void psiv_block(uint8_t out[64], const uint8_t key[36],
                       const uint8_t nonce[12], uint64_t ctr, const uint8_t rest[8]) {
    uint8_t state[64];
    copy_bytes(state,key,36); copy_bytes(state+36,nonce,12);
    store64(state+48,ctr); copy_bytes(state+56,rest,8);
    chacha_core(out,state); psiv_wipe(state,sizeof state);
}
static void poly_key(psiv_ctx *ctx, const uint8_t key[32]) {
    ctx->r[0]= load32(key) & UINT64_C(0x3ffffff);
    ctx->r[1]=(load32(key+3)>>2) & UINT64_C(0x3ffff03);
    ctx->r[2]=(load32(key+6)>>4) & UINT64_C(0x3ffc0ff);
    ctx->r[3]=(load32(key+9)>>6) & UINT64_C(0x3f03fff);
    ctx->r[4]=(load32(key+12)>>8)& UINT64_C(0x00fffff);
    for(unsigned i=0;i<4;i++) ctx->pad[i]=load32(key+16+4*i);
}
/* Explicit verification boundary: the arithmetic is unchanged from v0.2.
   The five convolution coefficients are normalized modulo 2^130-5. */
static void poly_carry(uint64_t h[5], uint64_t d0, uint64_t d1,
                       uint64_t d2, uint64_t d3, uint64_t d4) {
    uint64_t c;
    c=d0>>26; h[0]=d0&MASK26; d1+=c;
    c=d1>>26; h[1]=d1&MASK26; d2+=c;
    c=d2>>26; h[2]=d2&MASK26; d3+=c;
    c=d3>>26; h[3]=d3&MASK26; d4+=c;
    c=d4>>26; h[4]=d4&MASK26; h[0]+=5*c;
    c=h[0]>>26; h[0]&=MASK26; h[1]+=c;
}
/* Byte decoding is kept separate so its value/bounds contract checks the
   actual C operations, rather than a copied decoding expression. */
static void poly_add_block(uint64_t h[5], const uint8_t b[16], uint64_t hibit) {
    h[0]+= load32(b)&MASK26;
    h[1]+=(load32(b+3)>>2)&MASK26;
    h[2]+=(load32(b+6)>>4)&MASK26;
    h[3]+=(load32(b+9)>>6)&MASK26;
    h[4]+=(load32(b+12)>>8)|hibit;
}
/* AEAD hashes zero-padded full 16-byte blocks, each with its implicit 2^128.
   hibit=0 is used only in the raw Poly1305 known-answer self-test. */
static void poly_block(uint64_t h[5], const uint64_t r[5], const uint8_t b[16], uint64_t hibit) {
    uint64_t d0,d1,d2,d3,d4;
    poly_add_block(h,b,hibit);
    d0=h[0]*r[0]+h[1]*(5*r[4])+h[2]*(5*r[3])+h[3]*(5*r[2])+h[4]*(5*r[1]);
    d1=h[0]*r[1]+h[1]*r[0]+h[2]*(5*r[4])+h[3]*(5*r[3])+h[4]*(5*r[2]);
    d2=h[0]*r[2]+h[1]*r[1]+h[2]*r[0]+h[3]*(5*r[4])+h[4]*(5*r[3]);
    d3=h[0]*r[3]+h[1]*r[2]+h[2]*r[1]+h[3]*r[0]+h[4]*(5*r[4]);
    d4=h[0]*r[4]+h[1]*r[3]+h[2]*r[2]+h[3]*r[1]+h[4]*r[0];
    poly_carry(h,d0,d1,d2,d3,d4);
}
static void poly_padded(uint64_t h[5], const uint64_t r[5], const uint8_t *s, size_t n) {
    while(n>=16) { poly_block(h,r,s,UINT64_C(1)<<24); s+=16; n-=16; }
    if(n) {
        uint8_t b[16]={0}; copy_bytes(b,s,n);
        poly_block(h,r,b,UINT64_C(1)<<24); psiv_wipe(b,sizeof b);
    }
}
static void poly_finish(uint8_t out[16], uint64_t h[5], const uint32_t pad[4]) {
    uint64_t c,g[5],mask,f;
    c=h[1]>>26; h[1]&=MASK26; h[2]+=c;
    c=h[2]>>26; h[2]&=MASK26; h[3]+=c;
    c=h[3]>>26; h[3]&=MASK26; h[4]+=c;
    c=h[4]>>26; h[4]&=MASK26; h[0]+=5*c;
    c=h[0]>>26; h[0]&=MASK26; h[1]+=c;
    g[0]=h[0]+5; c=g[0]>>26; g[0]&=MASK26;
    for(unsigned i=1;i<4;i++) { g[i]=h[i]+c; c=g[i]>>26; g[i]&=MASK26; }
    g[4]=h[4]+c-(UINT64_C(1)<<26);
    mask=(g[4]>>63)-1; g[4]&=MASK26;
    for(unsigned i=0;i<5;i++) h[i]=(h[i]&~mask)|(g[i]&mask);
    f=((h[0]|h[1]<<26)&UINT64_C(0xffffffff))+pad[0]; store32(out,(uint32_t)f);
    f=((h[1]>>6|h[2]<<20)&UINT64_C(0xffffffff))+pad[1]+(f>>32); store32(out+4,(uint32_t)f);
    f=((h[2]>>12|h[3]<<14)&UINT64_C(0xffffffff))+pad[2]+(f>>32); store32(out+8,(uint32_t)f);
    f=((h[3]>>18|h[4]<<8)&UINT64_C(0xffffffff))+pad[3]+(f>>32); store32(out+12,(uint32_t)f);
    psiv_wipe(g,sizeof g);
}
static void finish_tag(uint8_t tag[16], const psiv_ctx *ctx, const uint8_t nonce[12],
                       uint64_t h[5], size_t ad_len, size_t msg_len) {
    uint8_t lengths[16],digest[16],b[64];
    store64(lengths,(uint64_t)ad_len); store64(lengths+8,(uint64_t)msg_len);
    poly_block(h,ctx->r,lengths,UINT64_C(1)<<24);
    poly_finish(digest,h,ctx->pad);
    psiv_block(b,ctx->tag_key,nonce,load64(digest),digest+8);
    copy_bytes(tag,b,16); psiv_wipe(digest,sizeof digest); psiv_wipe(b,sizeof b);
    psiv_wipe(h,5*sizeof(uint64_t));
}
static void stream_xor(uint8_t *out, const uint8_t *in, size_t n, const psiv_ctx *ctx,
                       const uint8_t nonce[12], const uint8_t tag[16]) {
    uint8_t b[64]; uint64_t ctr=load64(tag);
    while(n) {
        size_t take=n<64?n:64; psiv_block(b,ctx->enc_key,nonce,ctr,tag+8);
        for(size_t i=0;i<take;i++) out[i]=in[i]^b[i];
        ctr++; /* Defined uint64 wrap; matches the authors' wrapping_add. */
        out+=take; in+=take; n-=take;
    }
    psiv_wipe(b,sizeof b);
}
/* Exactly 16 bytes are read regardless of the first differing tag position.
   Compiled-code side-channel verification is outstanding. */
static unsigned tag_diff(const uint8_t a[16], const uint8_t b[16]) {
    uint32_t v=0; for(unsigned i=0;i<16;i++) v|=(uint32_t)(a[i]^b[i]); return v;
}
size_t psiv_ctx_size(void) { return sizeof(psiv_ctx); }
size_t psiv_ctx_align(void) { return _Alignof(psiv_ctx); }
static int span_ok(const void *p, size_t n) {
    return n==0 || (p!=0 && n<=UINTPTR_MAX-(uintptr_t)p);
}
/* Flat address-space ABI contract (x86-64, AArch64, wasm32). Avoid relational
   pointer comparison/subtraction between unrelated C objects. */
static int overlap(const void *a, size_t an, const void *b, size_t bn) {
    if(!an||!bn) return 0;
    uintptr_t x=(uintptr_t)a,y=(uintptr_t)b;
    return x<=y ? y-x<an : x-y<bn;
}
int psiv_init(psiv_ctx *ctx, const uint8_t *key, size_t key_len) {
    if(!ctx || (uintptr_t)ctx%_Alignof(psiv_ctx) || key_len!=32 || !span_ok(key,key_len)) return PSIV_INVALID;
    if(overlap(ctx,sizeof *ctx,key,32)) return PSIV_OVERLAP;
    uint8_t input[64]={0},b[64];
    psiv_wipe(ctx,sizeof *ctx);
    domain_key(input,key,0); chacha_core(b,input); poly_key(ctx,b);
    domain_key(ctx->tag_key,key,1); domain_key(ctx->enc_key,key,2); ctx->magic=CTX_MAGIC;
    psiv_wipe(input,sizeof input); psiv_wipe(b,sizeof b); return PSIV_OK;
}
void psiv_clear(psiv_ctx *ctx) { if(ctx) psiv_wipe(ctx,sizeof *ctx); }
static int validate(const psiv_ctx *ctx, const uint8_t *nonce,size_t nonce_len,
                    const uint8_t *ad,size_t ad_len,const uint8_t *input,size_t in_len,
                    uint8_t *out,size_t capacity,size_t needed) {
    if(!ctx || (uintptr_t)ctx%_Alignof(psiv_ctx)) return PSIV_INVALID;
    if(ctx->magic!=CTX_MAGIC || nonce_len!=12 || !span_ok(nonce,12) ||
       !span_ok(ad,ad_len) || !span_ok(input,in_len) || !span_ok(out,needed)) return PSIV_INVALID;
    if(ad_len>PSIV_MAX_AD) return PSIV_LIMIT;
    if(capacity<needed) return PSIV_CAPACITY;
    if(overlap(out,needed,ctx,sizeof *ctx)||overlap(out,needed,nonce,12)||overlap(out,needed,ad,ad_len)||
       (out!=input && overlap(out,needed,input,in_len))) return PSIV_OVERLAP;
    return PSIV_OK;
}
int psiv_seal(const psiv_ctx *ctx,const uint8_t *nonce,size_t nonce_len,
              const uint8_t *ad,size_t ad_len,const uint8_t *msg,size_t msg_len,
              uint8_t *out,size_t out_capacity) {
    if(msg_len>PSIV_MAX_MESSAGE) return PSIV_LIMIT;
    int rc=validate(ctx,nonce,nonce_len,ad,ad_len,msg,msg_len,out,out_capacity,msg_len+16);
    if(rc) return rc;
    uint64_t h[5]={0}; uint8_t tag[16];
    poly_padded(h,ctx->r,ad,ad_len); poly_padded(h,ctx->r,msg,msg_len);
    finish_tag(tag,ctx,nonce,h,ad_len,msg_len);
    stream_xor(out,msg,msg_len,ctx,nonce,tag); copy_bytes(out+msg_len,tag,16);
    psiv_wipe(tag,sizeof tag); return PSIV_OK;
}
int psiv_open(const psiv_ctx *ctx,const uint8_t *nonce,size_t nonce_len,
              const uint8_t *ad,size_t ad_len,const uint8_t *record,size_t record_len,
              uint8_t *out,size_t out_capacity) {
    if(record_len<16) return PSIV_INVALID;
    if(record_len-16>PSIV_MAX_MESSAGE) return PSIV_LIMIT;
    size_t msg_len=record_len-16;
    int rc=validate(ctx,nonce,nonce_len,ad,ad_len,record,record_len,out,out_capacity,msg_len);
    if(rc) return rc;
    /* First pass: authenticate candidates in private 64-byte stack storage.
       The output is never touched unless the full tag matches. Inputs must remain
       stable during this synchronous call (ordinary C data-race rules apply). */
    uint64_t h[5]={0},ctr=load64(record+msg_len);
    uint8_t b[64],tag[16],expected[16];
    copy_bytes(tag,record+msg_len,16); poly_padded(h,ctx->r,ad,ad_len);
    size_t off=0;
    while(off<msg_len) {
        size_t take=msg_len-off<64?msg_len-off:64;
        psiv_block(b,ctx->enc_key,nonce,ctr,tag+8);
        for(size_t i=0;i<take;i++) b[i]^=record[off+i];
        poly_padded(h,ctx->r,b,take); off+=take; ctr++;
    }
    finish_tag(expected,ctx,nonce,h,ad_len,msg_len);
    unsigned diff=tag_diff(tag,expected);
    psiv_wipe(b,sizeof b); psiv_wipe(expected,sizeof expected);
    if(diff) { psiv_wipe(tag,sizeof tag); return PSIV_AUTH; }
    stream_xor(out,record,msg_len,ctx,nonce,tag);
    psiv_wipe(tag,sizeof tag); return PSIV_OK;
}
