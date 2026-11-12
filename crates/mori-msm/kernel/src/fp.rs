// SPDX-License-Identifier: Apache-2.0
//! BLS12-381 base field, 6 x u64 Montgomery limbs (R = 2^384), the blst_fp layout.

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Fp(pub [u64; 6]);

pub const P: [u64; 6] = [
    0xb9fe_ffff_ffff_aaab,
    0x1eab_fffe_b153_ffff,
    0x6730_d2a0_f6b0_f624,
    0x6477_4b84_f385_12bf,
    0x4b1b_a7b6_434b_acd7,
    0x1a01_11ea_397f_e69a,
];
/// -p^-1 mod 2^64.
pub const INV: u64 = 0x89f3_fffc_fffc_fffd;

#[inline(always)]
fn mac(t: u64, a: u64, b: u64, c: u64) -> (u64, u64) {
    let x = t as u128 + (a as u128) * (b as u128) + c as u128;
    (x as u64, (x >> 64) as u64)
}

impl Fp {
    /// CIOS Montgomery multiplication; inputs and output in [0, p).
    #[inline(always)]
    pub fn mul(&self, b: &Fp) -> Fp {
        let a = &self.0;
        let b = &b.0;
        let mut t = [0u64; 8];
        for i in 0..6 {
            let mut c = 0u64;
            for j in 0..6 {
                (t[j], c) = mac(t[j], a[j], b[i], c);
            }
            let x = t[6] as u128 + c as u128;
            t[6] = x as u64;
            t[7] = (x >> 64) as u64;
            let m = t[0].wrapping_mul(INV);
            let (_, mut c) = mac(t[0], m, P[0], 0);
            for j in 1..6 {
                (t[j - 1], c) = mac(t[j], m, P[j], c);
            }
            let x = t[6] as u128 + c as u128;
            t[5] = x as u64;
            t[6] = t[7] + (x >> 64) as u64;
        }
        let mut r = [0u64; 6];
        r.copy_from_slice(&t[..6]);
        Fp(r).reduce_once()
    }

    /// Subtracts p if the value is at least p (branch free).
    #[inline(always)]
    pub fn reduce_once(self) -> Fp {
        let mut d = [0u64; 6];
        let mut borrow = 0u64;
        for j in 0..6 {
            let (x, b1) = self.0[j].overflowing_sub(P[j]);
            let (x, b2) = x.overflowing_sub(borrow);
            d[j] = x;
            borrow = (b1 | b2) as u64;
        }
        // borrow = 1 means self < p: keep self.
        let mask = borrow.wrapping_neg();
        let mut r = [0u64; 6];
        for j in 0..6 {
            r[j] = (self.0[j] & mask) | (d[j] & !mask);
        }
        Fp(r)
    }
}
