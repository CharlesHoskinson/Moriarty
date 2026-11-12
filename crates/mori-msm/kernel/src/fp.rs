// SPDX-License-Identifier: Apache-2.0
//! BLS12-381 base field, 6 x u64 Montgomery limbs (R = 2^384), the blst_fp layout.
//! Every modular reduction is branch free (borrow mask and select), ZKProphet IV-C3.

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
/// R mod p (Montgomery one).
pub const ONE: Fp = Fp([
    0x7609_0000_0002_fffd,
    0xebf4_000b_c40c_0002,
    0x5f48_9857_53c7_58ba,
    0x77ce_5853_7052_5745,
    0x5c07_1a97_a256_ec6d,
    0x15f6_5ec3_fa80_e493,
]);
pub const ZERO: Fp = Fp([0; 6]);

#[inline(always)]
fn mac(t: u64, a: u64, b: u64, c: u64) -> (u64, u64) {
    let x = t as u128 + (a as u128) * (b as u128) + c as u128;
    (x as u64, (x >> 64) as u64)
}

#[inline(always)]
fn adc(a: u64, b: u64, c: u64) -> (u64, u64) {
    let x = a as u128 + b as u128 + c as u128;
    (x as u64, (x >> 64) as u64)
}

#[inline(always)]
fn sbb(a: u64, b: u64, borrow: u64) -> (u64, u64) {
    let x = (a as u128).wrapping_sub(b as u128 + borrow as u128);
    (x as u64, ((x >> 64) as u64) & 1)
}

impl Fp {
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        (self.0[0] | self.0[1] | self.0[2] | self.0[3] | self.0[4] | self.0[5]) == 0
    }

    #[inline(always)]
    pub fn eq(&self, o: &Fp) -> bool {
        let mut d = 0u64;
        for j in 0..6 {
            d |= self.0[j] ^ o.0[j];
        }
        d == 0
    }

    /// Subtracts p if the value is at least p (input below 2p).
    #[inline(always)]
    pub fn reduce_once(self) -> Fp {
        let mut d = [0u64; 6];
        let mut b = 0u64;
        for j in 0..6 {
            (d[j], b) = sbb(self.0[j], P[j], b);
        }
        let keep = b.wrapping_neg(); // all ones when self < p
        let mut r = [0u64; 6];
        for j in 0..6 {
            r[j] = (self.0[j] & keep) | (d[j] & !keep);
        }
        Fp(r)
    }

    #[inline(always)]
    pub fn add(&self, o: &Fp) -> Fp {
        let mut r = [0u64; 6];
        let mut c = 0u64;
        for j in 0..6 {
            (r[j], c) = adc(self.0[j], o.0[j], c);
        }
        Fp(r).reduce_once()
    }

    #[inline(always)]
    pub fn dbl(&self) -> Fp {
        self.add(self)
    }

    #[inline(always)]
    pub fn sub(&self, o: &Fp) -> Fp {
        let mut r = [0u64; 6];
        let mut b = 0u64;
        for j in 0..6 {
            (r[j], b) = sbb(self.0[j], o.0[j], b);
        }
        let m = b.wrapping_neg(); // all ones when a borrow occurred: add p back
        let mut c = 0u64;
        for j in 0..6 {
            (r[j], c) = adc(r[j], P[j] & m, c);
        }
        Fp(r)
    }

    #[inline(always)]
    pub fn neg(&self) -> Fp {
        ZERO.sub(self)
    }

    /// CIOS Montgomery multiplication; inputs and output in [0, p). The outer loop is
    /// unrolled by a macro so every limb stays in a register (no local memory).
    #[inline(always)]
    pub fn mul(&self, b: &Fp) -> Fp {
        let a = &self.0;
        let b = &b.0;
        let mut t = [0u64; 8];
        macro_rules! row {
            ($i:expr) => {{
                let bi = b[$i];
                let mut c = 0u64;
                (t[0], c) = mac(t[0], a[0], bi, c);
                (t[1], c) = mac(t[1], a[1], bi, c);
                (t[2], c) = mac(t[2], a[2], bi, c);
                (t[3], c) = mac(t[3], a[3], bi, c);
                (t[4], c) = mac(t[4], a[4], bi, c);
                (t[5], c) = mac(t[5], a[5], bi, c);
                let x = t[6] as u128 + c as u128;
                t[6] = x as u64;
                t[7] = (x >> 64) as u64;
                let m = t[0].wrapping_mul(INV);
                let (_, mut c) = mac(t[0], m, P[0], 0);
                (t[0], c) = mac(t[1], m, P[1], c);
                (t[1], c) = mac(t[2], m, P[2], c);
                (t[2], c) = mac(t[3], m, P[3], c);
                (t[3], c) = mac(t[4], m, P[4], c);
                (t[4], c) = mac(t[5], m, P[5], c);
                let x = t[6] as u128 + c as u128;
                t[5] = x as u64;
                t[6] = t[7] + (x >> 64) as u64;
            }};
        }
        row!(0);
        row!(1);
        row!(2);
        row!(3);
        row!(4);
        row!(5);
        Fp([t[0], t[1], t[2], t[3], t[4], t[5]]).reduce_once()
    }

    #[inline(always)]
    pub fn sqr(&self) -> Fp {
        self.mul(self)
    }

    /// a^(p-2) (Fermat). Zero maps to zero.
    pub fn inv(&self) -> Fp {
        let e = [P[0] - 2, P[1], P[2], P[3], P[4], P[5]];
        let mut r = ONE;
        let mut i = 6;
        while i > 0 {
            i -= 1;
            let mut bit = 64;
            while bit > 0 {
                bit -= 1;
                r = r.sqr();
                if (e[i] >> bit) & 1 == 1 {
                    r = r.mul(self);
                }
            }
        }
        r
    }
}
