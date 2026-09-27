/* Extracted verbatim from Lean 4.32.1 include/lean/lean.h.
 * Copyright (c) 2019 Microsoft Corporation. All rights reserved.
 * Apache-2.0; see LEAN-LICENSE. Original author: Leonardo de Moura. */
#pragma once
#include <stdint.h>
_Static_assert(sizeof(uint64_t) == 8, "64-bit scalar ABI required");
static inline uint64_t lean_uint64_add(uint64_t a1, uint64_t a2) { return a1+a2; }
static inline uint8_t lean_uint64_dec_eq(uint64_t a1, uint64_t a2) { return a1 == a2; }
static inline uint8_t lean_uint64_dec_le(uint64_t a1, uint64_t a2) { return a1 <= a2; }
static inline uint8_t lean_uint64_dec_lt(uint64_t a1, uint64_t a2) { return a1 < a2; }
static inline uint64_t lean_uint64_div(uint64_t a1, uint64_t a2) { return a2 == 0 ? 0  : a1/a2; }
static inline uint64_t lean_uint64_land(uint64_t a, uint64_t b) { return a & b; }
static inline uint64_t lean_uint64_lor(uint64_t a, uint64_t b) { return a | b; }
static inline uint64_t lean_uint64_mod(uint64_t a1, uint64_t a2) { return a2 == 0 ? a1 : a1%a2; }
static inline uint64_t lean_uint64_shift_left(uint64_t a, uint64_t b) { return a << (b % 64); }
static inline uint64_t lean_uint64_sub(uint64_t a1, uint64_t a2) { return a1-a2; }
