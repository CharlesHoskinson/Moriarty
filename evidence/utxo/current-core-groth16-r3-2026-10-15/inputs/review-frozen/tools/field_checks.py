#!/usr/bin/env python3
"""Reviewer-owned integer re-derivation of the field-size facts the gadgets rely on
and of the author's A11 wrap-probe constants. Pure integer arithmetic; no crypto."""
import resource, json
resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
R = 0x73EDA753299D7D483339D80809A1D80553BDA402FFFE5BFEFFFFFFFF00000001  # BLS12-381 scalar modulus
U = (1 << 128) - 1
out = {}
out["r_decimal_matches_author"] = R == 52435875175126190479447740508185965837690552500527637822603658699938581184513
out["r_bits"] = R.bit_length()
b = -(-R // U)
out["wrap_b"] = str(b)
out["wrap_b_matches_author"] = b == 154095187621958656428822154526901524486
out["wrap_b_below_2_128"] = b < (1 << 128)
p = U * b
out["integer_product_overflows_u128"] = p > U
out["product_mod_r"] = str(p % R)
out["product_mod_r_matches_author"] = (p % R) == 74876584989711204157371212294527002617
out["product_mod_r_below_2_128"] = (p % R) < (1 << 128)
# Bounds used by the gadgets (must be < r so field equations are integer equations).
out["add_bound_2_129_lt_r"] = (1 << 129) < R
out["mul_rhs_bound_lt_r"] = (1 << 128) + (1 << 193) < R
out["lt_bound_2_130_lt_r"] = (1 << 130) < R
out["wide_bound_2_144_lt_r"] = (1 << 144) < R
out["min_modulus_bits_198_implies_r_ge_2_197"] = True
out["lt_wrong_flag_needs_t_ge_r_minus_2_129_gt_2_128"] = R - (1 << 129) > (1 << 128)
# Lt gadget exhaustive check on a toy: for all a,b in small range, f,t unique with t in [0, 2^k)
k = 4
ok = True
for a in range(1 << k):
    for c in range(1 << k):
        d = a - c
        sols = [(f, t) for f in (0, 1) for t in range(1 << k) if (f * (2 * d + 1) - (d - t)) % R == 0]
        ok &= sols == [(1 if a < c else 0, (c - a - 1) if a < c else (a - c))]
out["lt_gadget_toy_unique_k4"] = ok
# Mul gadget toy with 2-bit limbs (base 4): exhaustively check satisfiable iff a*b < 16
ok = True
for a in range(16):
    for c2 in range(16):
        a0, a1, b0, b1 = a % 4, a // 4, c2 % 4, c2 // 4
        sat = (a1 * b1 == 0) and (a0 * b0 + 4 * (a0 * b1 + a1 * b0) < 16)
        ok &= sat == (a * c2 < 16)
out["mul_gadget_toy_iff_k2"] = ok
print(json.dumps(out, indent=1))
