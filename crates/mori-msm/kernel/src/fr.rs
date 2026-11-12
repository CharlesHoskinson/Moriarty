// SPDX-License-Identifier: Apache-2.0
//! BLS12-381 scalar field: Montgomery form (R = 2^256, blst_fr) to canonical limbs.

const R: [u64; 4] = [
    0xffff_ffff_0000_0001,
    0x53bd_a402_fffe_5bfe,
    0x3339_d808_09a1_d805,
    0x73ed_a753_299d_7d48,
];
const INV: u64 = 0xffff_fffe_ffff_ffff;

/// Montgomery reduction of a (a * R^-1 mod r): the canonical value of a blst_fr.
#[inline(always)]
pub fn from_mont(a: [u64; 4]) -> [u64; 4] {
    let mut t = [a[0], a[1], a[2], a[3], 0];
    for _ in 0..4 {
        let m = t[0].wrapping_mul(INV);
        let x = t[0] as u128 + (m as u128) * (R[0] as u128);
        let mut c = (x >> 64) as u64;
        for j in 1..4 {
            let x = t[j] as u128 + (m as u128) * (R[j] as u128) + c as u128;
            t[j - 1] = x as u64;
            c = (x >> 64) as u64;
        }
        let x = t[4] as u128 + c as u128;
        t[3] = x as u64;
        t[4] = (x >> 64) as u64;
    }
    // t < 2r; subtract r once if needed.
    let mut d = [0u64; 4];
    let mut b = 0u64;
    for j in 0..4 {
        let x = (t[j] as u128).wrapping_sub(R[j] as u128 + b as u128);
        d[j] = x as u64;
        b = ((x >> 64) as u64) & 1;
    }
    let keep = (b & (t[4] == 0) as u64).wrapping_neg();
    let mut r = [0u64; 4];
    for j in 0..4 {
        r[j] = (t[j] & keep) | (d[j] & !keep);
    }
    r
}
