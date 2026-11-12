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
/// -p^-1 mod 2^32 and p in 32-bit limbs, for the 32-bit multiplication.
const INV32: u32 = 0xfffc_fffd;
const P32: [u32; 12] = [
    0xffff_aaab,
    0xb9fe_ffff,
    0xb153_ffff,
    0x1eab_fffe,
    0xf6b0_f624,
    0x6730_d2a0,
    0xf385_12bf,
    0x6477_4b84,
    0x434b_acd7,
    0x4b1b_a7b6,
    0x397f_e69a,
    0x1a01_11ea,
];
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

    #[inline(always)]
    fn limbs32(&self) -> [u32; 12] {
        let mut r = [0u32; 12];
        for j in 0..6 {
            r[2 * j] = self.0[j] as u32;
            r[2 * j + 1] = (self.0[j] >> 32) as u32;
        }
        r
    }

    #[inline(always)]
    fn from_limbs32(r: [u32; 12]) -> Fp {
        let mut o = [0u64; 6];
        for j in 0..6 {
            o[j] = r[2 * j] as u64 | ((r[2 * j + 1] as u64) << 32);
        }
        Fp(o)
    }

    /// Montgomery multiplication, inputs and output in [0, p): the 32-bit even/odd
    /// form by default, the 6 x 64-bit CIOS with kernel feature `fp64`.
    #[inline(always)]
    pub fn mul(&self, b: &Fp) -> Fp {
        if cfg!(feature = "fp64") {
            self.mul64(b)
        } else {
            self.mul32(b)
        }
    }

    /// CIOS Montgomery multiplication on 12 x 32-bit limbs in the even/odd form
    /// (ZKProphet IV-B2, the sppark mont_t scheme): x takes the products of the even
    /// limbs of a and p, y the odd ones one limb up, so the lo and hi halves of each
    /// product land in adjacent limbs of one carry chain and ptxas fuses each pair into
    /// one IMAD.WIDE.X. Each row folds y[0] into x[0]; that carry enters the odd chain.
    #[inline(always)]
    pub fn mul32(&self, o: &Fp) -> Fp {
        let a = self.limbs32();
        let bb = o.limbs32();
        let mut x = [0u32; 13];
        let mut y = [0u32; 14];
        macro_rules! row {
            ($bi:expr) => {{
                let bi = $bi;
                unsafe {
                    core::arch::asm!(
                        "add.cc.u32 {x0}, {x0}, {y0};",
                        "madc.lo.cc.u32 {y1}, {s0}, {m}, {y1};",
                        "madc.hi.cc.u32 {y2}, {s0}, {m}, {y2};",
                        "madc.lo.cc.u32 {y3}, {s1}, {m}, {y3};",
                        "madc.hi.cc.u32 {y4}, {s1}, {m}, {y4};",
                        "madc.lo.cc.u32 {y5}, {s2}, {m}, {y5};",
                        "madc.hi.cc.u32 {y6}, {s2}, {m}, {y6};",
                        "madc.lo.cc.u32 {y7}, {s3}, {m}, {y7};",
                        "madc.hi.cc.u32 {y8}, {s3}, {m}, {y8};",
                        "madc.lo.cc.u32 {y9}, {s4}, {m}, {y9};",
                        "madc.hi.cc.u32 {y10}, {s4}, {m}, {y10};",
                        "madc.lo.cc.u32 {y11}, {s5}, {m}, {y11};",
                        "madc.hi.cc.u32 {y12}, {s5}, {m}, {y12};",
                        "addc.u32 {y13}, {y13}, 0;",
                        x0 = inout(reg32) x[0],
                        y1 = inout(reg32) y[1],
                        y2 = inout(reg32) y[2],
                        y3 = inout(reg32) y[3],
                        y4 = inout(reg32) y[4],
                        y5 = inout(reg32) y[5],
                        y6 = inout(reg32) y[6],
                        y7 = inout(reg32) y[7],
                        y8 = inout(reg32) y[8],
                        y9 = inout(reg32) y[9],
                        y10 = inout(reg32) y[10],
                        y11 = inout(reg32) y[11],
                        y12 = inout(reg32) y[12],
                        y13 = inout(reg32) y[13],
                        y0 = in(reg32) y[0],
                        s0 = in(reg32) a[1],
                        s1 = in(reg32) a[3],
                        s2 = in(reg32) a[5],
                        s3 = in(reg32) a[7],
                        s4 = in(reg32) a[9],
                        s5 = in(reg32) a[11],
                        m = in(reg32) bi,
                        options(pure, nomem, nostack),
                    );
                    core::arch::asm!(
                        "mad.lo.cc.u32 {x0}, {s0}, {m}, {x0};",
                        "madc.hi.cc.u32 {x1}, {s0}, {m}, {x1};",
                        "madc.lo.cc.u32 {x2}, {s1}, {m}, {x2};",
                        "madc.hi.cc.u32 {x3}, {s1}, {m}, {x3};",
                        "madc.lo.cc.u32 {x4}, {s2}, {m}, {x4};",
                        "madc.hi.cc.u32 {x5}, {s2}, {m}, {x5};",
                        "madc.lo.cc.u32 {x6}, {s3}, {m}, {x6};",
                        "madc.hi.cc.u32 {x7}, {s3}, {m}, {x7};",
                        "madc.lo.cc.u32 {x8}, {s4}, {m}, {x8};",
                        "madc.hi.cc.u32 {x9}, {s4}, {m}, {x9};",
                        "madc.lo.cc.u32 {x10}, {s5}, {m}, {x10};",
                        "madc.hi.cc.u32 {x11}, {s5}, {m}, {x11};",
                        "addc.u32 {x12}, {x12}, 0;",
                        x0 = inout(reg32) x[0],
                        x1 = inout(reg32) x[1],
                        x2 = inout(reg32) x[2],
                        x3 = inout(reg32) x[3],
                        x4 = inout(reg32) x[4],
                        x5 = inout(reg32) x[5],
                        x6 = inout(reg32) x[6],
                        x7 = inout(reg32) x[7],
                        x8 = inout(reg32) x[8],
                        x9 = inout(reg32) x[9],
                        x10 = inout(reg32) x[10],
                        x11 = inout(reg32) x[11],
                        x12 = inout(reg32) x[12],
                        s0 = in(reg32) a[0],
                        s1 = in(reg32) a[2],
                        s2 = in(reg32) a[4],
                        s3 = in(reg32) a[6],
                        s4 = in(reg32) a[8],
                        s5 = in(reg32) a[10],
                        m = in(reg32) bi,
                        options(pure, nomem, nostack),
                    );
                }
                let m = x[0].wrapping_mul(INV32);
                unsafe {
                    core::arch::asm!(
                        "mad.lo.cc.u32 {x0}, {s0}, {m}, {x0};",
                        "madc.hi.cc.u32 {x1}, {s0}, {m}, {x1};",
                        "madc.lo.cc.u32 {x2}, {s1}, {m}, {x2};",
                        "madc.hi.cc.u32 {x3}, {s1}, {m}, {x3};",
                        "madc.lo.cc.u32 {x4}, {s2}, {m}, {x4};",
                        "madc.hi.cc.u32 {x5}, {s2}, {m}, {x5};",
                        "madc.lo.cc.u32 {x6}, {s3}, {m}, {x6};",
                        "madc.hi.cc.u32 {x7}, {s3}, {m}, {x7};",
                        "madc.lo.cc.u32 {x8}, {s4}, {m}, {x8};",
                        "madc.hi.cc.u32 {x9}, {s4}, {m}, {x9};",
                        "madc.lo.cc.u32 {x10}, {s5}, {m}, {x10};",
                        "madc.hi.cc.u32 {x11}, {s5}, {m}, {x11};",
                        "addc.u32 {x12}, {x12}, 0;",
                        x0 = inout(reg32) x[0],
                        x1 = inout(reg32) x[1],
                        x2 = inout(reg32) x[2],
                        x3 = inout(reg32) x[3],
                        x4 = inout(reg32) x[4],
                        x5 = inout(reg32) x[5],
                        x6 = inout(reg32) x[6],
                        x7 = inout(reg32) x[7],
                        x8 = inout(reg32) x[8],
                        x9 = inout(reg32) x[9],
                        x10 = inout(reg32) x[10],
                        x11 = inout(reg32) x[11],
                        x12 = inout(reg32) x[12],
                        s0 = in(reg32) P32[0],
                        s1 = in(reg32) P32[2],
                        s2 = in(reg32) P32[4],
                        s3 = in(reg32) P32[6],
                        s4 = in(reg32) P32[8],
                        s5 = in(reg32) P32[10],
                        m = in(reg32) m,
                        options(pure, nomem, nostack),
                    );
                    core::arch::asm!(
                        "mad.lo.cc.u32 {y1}, {s0}, {m}, {y1};",
                        "madc.hi.cc.u32 {y2}, {s0}, {m}, {y2};",
                        "madc.lo.cc.u32 {y3}, {s1}, {m}, {y3};",
                        "madc.hi.cc.u32 {y4}, {s1}, {m}, {y4};",
                        "madc.lo.cc.u32 {y5}, {s2}, {m}, {y5};",
                        "madc.hi.cc.u32 {y6}, {s2}, {m}, {y6};",
                        "madc.lo.cc.u32 {y7}, {s3}, {m}, {y7};",
                        "madc.hi.cc.u32 {y8}, {s3}, {m}, {y8};",
                        "madc.lo.cc.u32 {y9}, {s4}, {m}, {y9};",
                        "madc.hi.cc.u32 {y10}, {s4}, {m}, {y10};",
                        "madc.lo.cc.u32 {y11}, {s5}, {m}, {y11};",
                        "madc.hi.cc.u32 {y12}, {s5}, {m}, {y12};",
                        "addc.u32 {y13}, {y13}, 0;",
                        y1 = inout(reg32) y[1],
                        y2 = inout(reg32) y[2],
                        y3 = inout(reg32) y[3],
                        y4 = inout(reg32) y[4],
                        y5 = inout(reg32) y[5],
                        y6 = inout(reg32) y[6],
                        y7 = inout(reg32) y[7],
                        y8 = inout(reg32) y[8],
                        y9 = inout(reg32) y[9],
                        y10 = inout(reg32) y[10],
                        y11 = inout(reg32) y[11],
                        y12 = inout(reg32) y[12],
                        y13 = inout(reg32) y[13],
                        s0 = in(reg32) P32[1],
                        s1 = in(reg32) P32[3],
                        s2 = in(reg32) P32[5],
                        s3 = in(reg32) P32[7],
                        s4 = in(reg32) P32[9],
                        s5 = in(reg32) P32[11],
                        m = in(reg32) m,
                        options(pure, nomem, nostack),
                    );
                }
                // x[0] = 0: shift by one limb. x' = y[1..14], y' = x[1..13].
                let mut nx = [0u32; 13];
                nx.copy_from_slice(&y[1..14]);
                let mut ny = [0u32; 14];
                ny[..12].copy_from_slice(&x[1..13]);
                x = nx;
                y = ny;
            }};
        }
        row!(bb[0]);
        row!(bb[1]);
        row!(bb[2]);
        row!(bb[3]);
        row!(bb[4]);
        row!(bb[5]);
        row!(bb[6]);
        row!(bb[7]);
        row!(bb[8]);
        row!(bb[9]);
        row!(bb[10]);
        row!(bb[11]);
        // T = x + y < 2p fits 12 limbs.
        let mut r = [0u32; 12];
        unsafe {
            core::arch::asm!(
                "add.cc.u32 {r0}, {x0}, {y0};",
                "addc.cc.u32 {r1}, {x1}, {y1};",
                "addc.cc.u32 {r2}, {x2}, {y2};",
                "addc.cc.u32 {r3}, {x3}, {y3};",
                "addc.cc.u32 {r4}, {x4}, {y4};",
                "addc.cc.u32 {r5}, {x5}, {y5};",
                "addc.cc.u32 {r6}, {x6}, {y6};",
                "addc.cc.u32 {r7}, {x7}, {y7};",
                "addc.cc.u32 {r8}, {x8}, {y8};",
                "addc.cc.u32 {r9}, {x9}, {y9};",
                "addc.cc.u32 {r10}, {x10}, {y10};",
                "addc.u32 {r11}, {x11}, {y11};",
                r0 = out(reg32) r[0],
                r1 = out(reg32) r[1],
                r2 = out(reg32) r[2],
                r3 = out(reg32) r[3],
                r4 = out(reg32) r[4],
                r5 = out(reg32) r[5],
                r6 = out(reg32) r[6],
                r7 = out(reg32) r[7],
                r8 = out(reg32) r[8],
                r9 = out(reg32) r[9],
                r10 = out(reg32) r[10],
                r11 = out(reg32) r[11],
                x0 = in(reg32) x[0],
                x1 = in(reg32) x[1],
                x2 = in(reg32) x[2],
                x3 = in(reg32) x[3],
                x4 = in(reg32) x[4],
                x5 = in(reg32) x[5],
                x6 = in(reg32) x[6],
                x7 = in(reg32) x[7],
                x8 = in(reg32) x[8],
                x9 = in(reg32) x[9],
                x10 = in(reg32) x[10],
                x11 = in(reg32) x[11],
                y0 = in(reg32) y[0],
                y1 = in(reg32) y[1],
                y2 = in(reg32) y[2],
                y3 = in(reg32) y[3],
                y4 = in(reg32) y[4],
                y5 = in(reg32) y[5],
                y6 = in(reg32) y[6],
                y7 = in(reg32) y[7],
                y8 = in(reg32) y[8],
                y9 = in(reg32) y[9],
                y10 = in(reg32) y[10],
                y11 = in(reg32) y[11],
                options(pure, nomem, nostack),
            );
        }
        Fp::from_limbs32(r).reduce_once()
    }

    /// CIOS Montgomery multiplication on 6 x 64-bit limbs. The outer loop is unrolled
    /// by a macro so every limb stays in a register (no local memory).
    #[inline(always)]
    pub fn mul64(&self, b: &Fp) -> Fp {
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
