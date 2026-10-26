# Local mori-recursion tower.rs
> Source: file:///home/charl/code/Moriarty-perf/crates/mori-recursion/src/tower.rs
> Collected: 2026-10-26
> Published: Unknown

// Adapted from projectfenrir recursion/stagenet-strawman/recl9.
// Compiled on the pinned stock graph; installed-key equality is checked separately.
//! Fp2 / Fp6 / Fp12 tower arithmetic in-circuit, built only on the public API
//! of midnight-circuits 7.2.4's emulated field chip `FieldChip<Fq, Fp, ..>`
//! (the BLS12-381 base field emulated over the BLS12-381 scalar field).
//!
//! Tower: Fp2 = Fp[u]/(u^2+1), Fp6 = Fp2[v]/(v^3 - (u+1)), Fp12 = Fp6[w]/(w^2 - v).

use std::cell::RefCell;

use ff::Field;
use midnight_circuits::{
    field::{
        NativeChip, NativeGadget,
        decomposition::chip::P2RDecompositionChip,
        foreign::{AssignedField, FieldChip},
    },
    instructions::{
        ArithInstructions, AssertionInstructions, AssignmentInstructions, ControlFlowInstructions,
    },
    types::{AssignedBit, InnerValue},
};
use midnight_curves::{
    Fp,
    bls12_381::{Fp2, Fp6, Fp12},
};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};

/// Native (circuit) field: the BLS12-381 scalar field.
pub type F = midnight_curves::Fq;
/// Native gadget used by the zk-stdlib.
pub type NG = NativeGadget<F, P2RDecompositionChip<F>, NativeChip<F>>;
/// Emulation parameters the zk-stdlib uses for the BLS12-381 base field
/// (the blanket curve impl, which resolves to 7 limbs of 56 bits).
pub type EP = midnight_curves::G1Projective;
/// The emulated base-field chip.
pub type FpChip = FieldChip<F, Fp, EP, NG>;
/// An emulated Fp element.
pub type AFp = AssignedField<F, Fp, EP>;

#[derive(Clone, Debug)]
pub struct AFp2 {
    pub c0: AFp,
    pub c1: AFp,
}

#[derive(Clone, Debug)]
pub struct AFp6 {
    pub c0: AFp2,
    pub c1: AFp2,
    pub c2: AFp2,
}

#[derive(Clone, Debug)]
pub struct AFp12 {
    pub c0: AFp6,
    pub c1: AFp6,
}

impl AFp2 {
    pub fn value(&self) -> Value<Fp2> {
        self.c0
            .value()
            .zip(self.c1.value())
            .map(|(a, b)| Fp2::new(a, b))
    }
}

impl AFp6 {
    pub fn value(&self) -> Value<Fp6> {
        self.c0
            .value()
            .zip(self.c1.value())
            .zip(self.c2.value())
            .map(|((a, b), c)| Fp6::new(a, b, c))
    }
}

impl AFp12 {
    pub fn value(&self) -> Value<Fp12> {
        self.c0
            .value()
            .zip(self.c1.value())
            .map(|(a, b)| Fp12::new(a, b))
    }

    /// Coefficients a_j (j = 0..5) of f = sum_j a_j w^j, as references.
    pub fn w_coeffs(&self) -> [&AFp2; 6] {
        [
            &self.c0.c0,
            &self.c1.c0,
            &self.c0.c1,
            &self.c1.c1,
            &self.c0.c2,
            &self.c1.c2,
        ]
    }

    pub fn from_w_coeffs(a: [AFp2; 6]) -> Self {
        let [a0, a1, a2, a3, a4, a5] = a;
        AFp12 {
            c0: AFp6 {
                c0: a0,
                c1: a2,
                c2: a4,
            },
            c1: AFp6 {
                c0: a1,
                c1: a3,
                c2: a5,
            },
        }
    }
}

/// Operation counters (number of calls of each tower operation), used for the
/// cost breakdown.
#[derive(Clone, Debug, Default)]
pub struct OpCounts {
    pub fp_mul: usize,
    pub fp_const_mul: usize,
    pub fp2_mul: usize,
    pub fp2_sqr: usize,
    pub fp12_mul: usize,
    pub fp12_sqr: usize,
    pub fp12_sparse: usize,
    pub fp_nf: usize,
}

/// Strategy knobs (measured against each other in RESULTS.md).
#[derive(Clone, Copy, Debug)]
pub struct TowerOpts {
    /// Karatsuba (3 Fp muls + 2 sums) instead of schoolbook (4 Fp muls) for
    /// Fp2 multiplication.
    pub karatsuba_fp2: bool,
    /// Normalize once: re-witness a non-well-formed value as a fresh
    /// range-checked element tied to it by `assert_equal`, before it is used
    /// in several multiplications (the stock chip normalizes at every use).
    pub normalize_once: bool,
    /// Flat (schoolbook-over-Fp) Fp12 products, one sum per output.
    pub flat: bool,
}

impl Default for TowerOpts {
    fn default() -> Self {
        TowerOpts {
            karatsuba_fp2: false,
            normalize_once: true,
            flat: true,
        }
    }
}

/// Tower arithmetic over a borrowed emulated-Fp chip.
pub struct Tower<'a> {
    pub fp: &'a FpChip,
    pub opts: TowerOpts,
    pub counts: RefCell<OpCounts>,
}

macro_rules! assign_fp {
    ($s:expr, $l:expr, $v:expr) => {
        AssignmentInstructions::<F, AFp>::assign($s.fp, $l, $v)
    };
}
macro_rules! fixed_fp {
    ($s:expr, $l:expr, $v:expr) => {
        AssignmentInstructions::<F, AFp>::assign_fixed($s.fp, $l, $v)
    };
}

impl<'a> Tower<'a> {
    pub fn new(fp: &'a FpChip, opts: TowerOpts) -> Self {
        Tower {
            fp,
            opts,
            counts: RefCell::new(OpCounts::default()),
        }
    }

    // ------------------------------------------------------------------ Fp

    pub fn fp_assign(&self, l: &mut impl Layouter<F>, v: Value<Fp>) -> Result<AFp, Error> {
        assign_fp!(self, l, v)
    }

    pub fn fp_fixed(&self, l: &mut impl Layouter<F>, v: Fp) -> Result<AFp, Error> {
        fixed_fp!(self, l, v)
    }

    /// Well-formed copy of `x` (see `TowerOpts::normalize_once`).
    pub fn fp_nf(&self, l: &mut impl Layouter<F>, x: &AFp) -> Result<AFp, Error> {
        if !self.opts.normalize_once || x.is_well_formed() {
            return Ok(x.clone());
        }
        self.counts.borrow_mut().fp_nf += 1;
        let z = assign_fp!(self, l, x.value())?;
        AssertionInstructions::<F, AFp>::assert_equal(self.fp, l, &z, x)?;
        Ok(z)
    }

    pub fn fp2_nf(&self, l: &mut impl Layouter<F>, a: &AFp2) -> Result<AFp2, Error> {
        Ok(AFp2 {
            c0: self.fp_nf(l, &a.c0)?,
            c1: self.fp_nf(l, &a.c1)?,
        })
    }

    pub fn fp6_nf(&self, l: &mut impl Layouter<F>, a: &AFp6) -> Result<AFp6, Error> {
        Ok(AFp6 {
            c0: self.fp2_nf(l, &a.c0)?,
            c1: self.fp2_nf(l, &a.c1)?,
            c2: self.fp2_nf(l, &a.c2)?,
        })
    }

    pub fn fp12_nf(&self, l: &mut impl Layouter<F>, a: &AFp12) -> Result<AFp12, Error> {
        Ok(AFp12 {
            c0: self.fp6_nf(l, &a.c0)?,
            c1: self.fp6_nf(l, &a.c1)?,
        })
    }

    pub fn fp_mul(&self, l: &mut impl Layouter<F>, a: &AFp, b: &AFp) -> Result<AFp, Error> {
        self.counts.borrow_mut().fp_mul += 1;
        self.fp.mul(l, a, b, None)
    }

    pub fn fp_mul_const(&self, l: &mut impl Layouter<F>, a: &AFp, k: Fp) -> Result<AFp, Error> {
        if k != Fp::ZERO && k != Fp::ONE && k != -Fp::ONE {
            self.counts.borrow_mut().fp_const_mul += 1;
        }
        self.fp.mul_by_constant(l, a, k)
    }

    pub fn fp_select(
        &self,
        l: &mut impl Layouter<F>,
        cond: &AssignedBit<F>,
        a: &AFp,
        b: &AFp,
    ) -> Result<AFp, Error> {
        self.fp.select(l, cond, a, b)
    }

    // ----------------------------------------------------------------- Fp2

    pub fn fp2_assign(&self, l: &mut impl Layouter<F>, v: Value<Fp2>) -> Result<AFp2, Error> {
        Ok(AFp2 {
            c0: assign_fp!(self, l, v.map(|x| x.c0()))?,
            c1: assign_fp!(self, l, v.map(|x| x.c1()))?,
        })
    }

    pub fn fp2_fixed(&self, l: &mut impl Layouter<F>, v: Fp2) -> Result<AFp2, Error> {
        Ok(AFp2 {
            c0: fixed_fp!(self, l, v.c0())?,
            c1: fixed_fp!(self, l, v.c1())?,
        })
    }

    pub fn fp2_from_fp(&self, l: &mut impl Layouter<F>, a: &AFp) -> Result<AFp2, Error> {
        Ok(AFp2 {
            c0: a.clone(),
            c1: fixed_fp!(self, l, Fp::ZERO)?,
        })
    }

    pub fn fp2_add(&self, l: &mut impl Layouter<F>, a: &AFp2, b: &AFp2) -> Result<AFp2, Error> {
        Ok(AFp2 {
            c0: self.fp.add(l, &a.c0, &b.c0)?,
            c1: self.fp.add(l, &a.c1, &b.c1)?,
        })
    }

    pub fn fp2_sub(&self, l: &mut impl Layouter<F>, a: &AFp2, b: &AFp2) -> Result<AFp2, Error> {
        Ok(AFp2 {
            c0: self.fp.sub(l, &a.c0, &b.c0)?,
            c1: self.fp.sub(l, &a.c1, &b.c1)?,
        })
    }

    pub fn fp2_neg(&self, l: &mut impl Layouter<F>, a: &AFp2) -> Result<AFp2, Error> {
        Ok(AFp2 {
            c0: self.fp.neg(l, &a.c0)?,
            c1: self.fp.neg(l, &a.c1)?,
        })
    }

    pub fn fp2_double(&self, l: &mut impl Layouter<F>, a: &AFp2) -> Result<AFp2, Error> {
        self.fp2_add(l, a, a)
    }

    pub fn fp2_conj(&self, l: &mut impl Layouter<F>, a: &AFp2) -> Result<AFp2, Error> {
        Ok(AFp2 {
            c0: a.c0.clone(),
            c1: self.fp.neg(l, &a.c1)?,
        })
    }

    /// Sum of positive and negative Fp2 terms (one linear combination per
    /// coordinate).
    pub fn fp2_sum(
        &self,
        l: &mut impl Layouter<F>,
        pos: &[&AFp2],
        neg: &[&AFp2],
    ) -> Result<AFp2, Error> {
        let p0: Vec<AFp> = pos.iter().map(|x| x.c0.clone()).collect();
        let n0: Vec<AFp> = neg.iter().map(|x| x.c0.clone()).collect();
        let p1: Vec<AFp> = pos.iter().map(|x| x.c1.clone()).collect();
        let n1: Vec<AFp> = neg.iter().map(|x| x.c1.clone()).collect();
        Ok(AFp2 {
            c0: self.fp.sum(l, &p0, &n0, Fp::ZERO)?,
            c1: self.fp.sum(l, &p1, &n1, Fp::ZERO)?,
        })
    }

    /// a * xi = (a0 - a1) + (a0 + a1) u.
    pub fn fp2_mul_by_xi(&self, l: &mut impl Layouter<F>, a: &AFp2) -> Result<AFp2, Error> {
        Ok(AFp2 {
            c0: self.fp.sub(l, &a.c0, &a.c1)?,
            c1: self.fp.add(l, &a.c0, &a.c1)?,
        })
    }

    pub fn fp2_mul(&self, l: &mut impl Layouter<F>, a: &AFp2, b: &AFp2) -> Result<AFp2, Error> {
        self.counts.borrow_mut().fp2_mul += 1;
        let a = &self.fp2_nf(l, a)?;
        let b = &self.fp2_nf(l, b)?;
        if self.opts.karatsuba_fp2 {
            let t0 = self.fp_mul(l, &a.c0, &b.c0)?;
            let t1 = self.fp_mul(l, &a.c1, &b.c1)?;
            let sa = self.fp.add(l, &a.c0, &a.c1)?;
            let sb = self.fp.add(l, &b.c0, &b.c1)?;
            let t2 = self.fp_mul(l, &sa, &sb)?;
            let c0 = self.fp.sub(l, &t0, &t1)?;
            let c1 = self.fp.sum(l, &[t2], &[t0, t1], Fp::ZERO)?;
            Ok(AFp2 { c0, c1 })
        } else {
            let t00 = self.fp_mul(l, &a.c0, &b.c0)?;
            let t11 = self.fp_mul(l, &a.c1, &b.c1)?;
            let t01 = self.fp_mul(l, &a.c0, &b.c1)?;
            let t10 = self.fp_mul(l, &a.c1, &b.c0)?;
            let c0 = self.fp.sub(l, &t00, &t11)?;
            let c1 = self.fp.add(l, &t01, &t10)?;
            Ok(AFp2 { c0, c1 })
        }
    }

    pub fn fp2_square(&self, l: &mut impl Layouter<F>, a: &AFp2) -> Result<AFp2, Error> {
        self.counts.borrow_mut().fp2_sqr += 1;
        // (a0 + a1)(a0 - a1) + 2 a0 a1 u
        let s = self.fp.add(l, &a.c0, &a.c1)?;
        let d = self.fp.sub(l, &a.c0, &a.c1)?;
        let c0 = self.fp_mul(l, &s, &d)?;
        let t = self.fp_mul(l, &a.c0, &a.c1)?;
        let c1 = self.fp.add(l, &t, &t)?;
        Ok(AFp2 { c0, c1 })
    }

    pub fn fp2_mul_by_fp(
        &self,
        l: &mut impl Layouter<F>,
        a: &AFp2,
        b: &AFp,
    ) -> Result<AFp2, Error> {
        Ok(AFp2 {
            c0: self.fp_mul(l, &a.c0, b)?,
            c1: self.fp_mul(l, &a.c1, b)?,
        })
    }

    /// Constant Fp2 times a variable Fp (2 constant multiplications, fewer if
    /// a coordinate of the constant is 0 or +-1).
    pub fn fp2_const_times_fp(
        &self,
        l: &mut impl Layouter<F>,
        k: Fp2,
        b: &AFp,
    ) -> Result<AFp2, Error> {
        Ok(AFp2 {
            c0: self.fp_mul_const(l, b, k.c0())?,
            c1: self.fp_mul_const(l, b, k.c1())?,
        })
    }

    /// Variable Fp2 times a constant Fp2.
    pub fn fp2_mul_const(&self, l: &mut impl Layouter<F>, a: &AFp2, k: Fp2) -> Result<AFp2, Error> {
        let (k0, k1) = (k.c0(), k.c1());
        if k1 == Fp::ZERO {
            return Ok(AFp2 {
                c0: self.fp_mul_const(l, &a.c0, k0)?,
                c1: self.fp_mul_const(l, &a.c1, k0)?,
            });
        }
        if k0 == Fp::ZERO {
            // (a0 + a1 u) k1 u = -k1 a1 + k1 a0 u
            return Ok(AFp2 {
                c0: self.fp_mul_const(l, &a.c1, -k1)?,
                c1: self.fp_mul_const(l, &a.c0, k1)?,
            });
        }
        let a0k0 = self.fp_mul_const(l, &a.c0, k0)?;
        let a1k1 = self.fp_mul_const(l, &a.c1, k1)?;
        let a0k1 = self.fp_mul_const(l, &a.c0, k1)?;
        let a1k0 = self.fp_mul_const(l, &a.c1, k0)?;
        Ok(AFp2 {
            c0: self.fp.sub(l, &a0k0, &a1k1)?,
            c1: self.fp.add(l, &a0k1, &a1k0)?,
        })
    }

    pub fn fp2_select(
        &self,
        l: &mut impl Layouter<F>,
        cond: &AssignedBit<F>,
        a: &AFp2,
        b: &AFp2,
    ) -> Result<AFp2, Error> {
        Ok(AFp2 {
            c0: self.fp.select(l, cond, &a.c0, &b.c0)?,
            c1: self.fp.select(l, cond, &a.c1, &b.c1)?,
        })
    }

    pub fn fp2_assert_equal(
        &self,
        l: &mut impl Layouter<F>,
        a: &AFp2,
        b: &AFp2,
    ) -> Result<(), Error> {
        AssertionInstructions::<F, AFp>::assert_equal(self.fp, l, &a.c0, &b.c0)?;
        AssertionInstructions::<F, AFp>::assert_equal(self.fp, l, &a.c1, &b.c1)
    }

    pub fn fp2_assert_equal_to_fixed(
        &self,
        l: &mut impl Layouter<F>,
        a: &AFp2,
        k: Fp2,
    ) -> Result<(), Error> {
        AssertionInstructions::<F, AFp>::assert_equal_to_fixed(self.fp, l, &a.c0, k.c0())?;
        AssertionInstructions::<F, AFp>::assert_equal_to_fixed(self.fp, l, &a.c1, k.c1())
    }

    /// 1 / a, with the constraint a * a^{-1} = 1 (unsatisfiable if a = 0).
    pub fn fp2_inv(&self, l: &mut impl Layouter<F>, a: &AFp2) -> Result<AFp2, Error> {
        let inv_v = a.value().map(|x| x.invert().unwrap_or(Fp2::ZERO));
        let inv = self.fp2_assign(l, inv_v)?;
        let prod = self.fp2_mul(l, a, &inv)?;
        self.fp2_assert_equal_to_fixed(l, &prod, Fp2::ONE)?;
        Ok(inv)
    }

    // ----------------------------------------------------------------- Fp6

    pub fn fp6_assign(&self, l: &mut impl Layouter<F>, v: Value<Fp6>) -> Result<AFp6, Error> {
        Ok(AFp6 {
            c0: self.fp2_assign(l, v.map(|x| x.c0()))?,
            c1: self.fp2_assign(l, v.map(|x| x.c1()))?,
            c2: self.fp2_assign(l, v.map(|x| x.c2()))?,
        })
    }

    pub fn fp6_fixed(&self, l: &mut impl Layouter<F>, v: Fp6) -> Result<AFp6, Error> {
        Ok(AFp6 {
            c0: self.fp2_fixed(l, v.c0())?,
            c1: self.fp2_fixed(l, v.c1())?,
            c2: self.fp2_fixed(l, v.c2())?,
        })
    }

    pub fn fp6_add(&self, l: &mut impl Layouter<F>, a: &AFp6, b: &AFp6) -> Result<AFp6, Error> {
        Ok(AFp6 {
            c0: self.fp2_add(l, &a.c0, &b.c0)?,
            c1: self.fp2_add(l, &a.c1, &b.c1)?,
            c2: self.fp2_add(l, &a.c2, &b.c2)?,
        })
    }

    pub fn fp6_sub(&self, l: &mut impl Layouter<F>, a: &AFp6, b: &AFp6) -> Result<AFp6, Error> {
        Ok(AFp6 {
            c0: self.fp2_sub(l, &a.c0, &b.c0)?,
            c1: self.fp2_sub(l, &a.c1, &b.c1)?,
            c2: self.fp2_sub(l, &a.c2, &b.c2)?,
        })
    }

    /// a * v = (xi a2, a0, a1).
    pub fn fp6_mul_by_v(&self, l: &mut impl Layouter<F>, a: &AFp6) -> Result<AFp6, Error> {
        Ok(AFp6 {
            c0: self.fp2_mul_by_xi(l, &a.c2)?,
            c1: a.c0.clone(),
            c2: a.c1.clone(),
        })
    }

    /// Karatsuba Fp6 multiplication (6 Fp2 multiplications).
    pub fn fp6_mul(&self, l: &mut impl Layouter<F>, a: &AFp6, b: &AFp6) -> Result<AFp6, Error> {
        let t0 = self.fp2_mul(l, &a.c0, &b.c0)?;
        let t1 = self.fp2_mul(l, &a.c1, &b.c1)?;
        let t2 = self.fp2_mul(l, &a.c2, &b.c2)?;
        // c0 = xi((a1+a2)(b1+b2) - t1 - t2) + t0
        let a12 = self.fp2_add(l, &a.c1, &a.c2)?;
        let b12 = self.fp2_add(l, &b.c1, &b.c2)?;
        let m = self.fp2_mul(l, &a12, &b12)?;
        let m = self.fp2_sum(l, &[&m], &[&t1, &t2])?;
        let m = self.fp2_mul_by_xi(l, &m)?;
        let c0 = self.fp2_add(l, &m, &t0)?;
        // c1 = (a0+a1)(b0+b1) - t0 - t1 + xi t2
        let a01 = self.fp2_add(l, &a.c0, &a.c1)?;
        let b01 = self.fp2_add(l, &b.c0, &b.c1)?;
        let m = self.fp2_mul(l, &a01, &b01)?;
        let xt2 = self.fp2_mul_by_xi(l, &t2)?;
        let c1 = self.fp2_sum(l, &[&m, &xt2], &[&t0, &t1])?;
        // c2 = (a0+a2)(b0+b2) - t0 + t1 - t2
        let a02 = self.fp2_add(l, &a.c0, &a.c2)?;
        let b02 = self.fp2_add(l, &b.c0, &b.c2)?;
        let m = self.fp2_mul(l, &a02, &b02)?;
        let c2 = self.fp2_sum(l, &[&m, &t1], &[&t0, &t2])?;
        Ok(AFp6 { c0, c1, c2 })
    }

    /// a * (b0 + b1 v) (5 Fp2 multiplications).
    pub fn fp6_mul_by_01(
        &self,
        l: &mut impl Layouter<F>,
        a: &AFp6,
        b0: &AFp2,
        b1: &AFp2,
    ) -> Result<AFp6, Error> {
        let t0 = self.fp2_mul(l, &a.c0, b0)?;
        let t1 = self.fp2_mul(l, &a.c1, b1)?;
        // c0 = t0 + xi a2 b1
        let m = self.fp2_mul(l, &a.c2, b1)?;
        let m = self.fp2_mul_by_xi(l, &m)?;
        let c0 = self.fp2_add(l, &t0, &m)?;
        // c1 = (a0 + a1)(b0 + b1) - t0 - t1
        let a01 = self.fp2_add(l, &a.c0, &a.c1)?;
        let b01 = self.fp2_add(l, b0, b1)?;
        let m = self.fp2_mul(l, &a01, &b01)?;
        let c1 = self.fp2_sum(l, &[&m], &[&t0, &t1])?;
        // c2 = t1 + a2 b0
        let m = self.fp2_mul(l, &a.c2, b0)?;
        let c2 = self.fp2_add(l, &t1, &m)?;
        Ok(AFp6 { c0, c1, c2 })
    }

    // ---------------------------------------------------------------- Fp12

    pub fn fp12_assign(&self, l: &mut impl Layouter<F>, v: Value<Fp12>) -> Result<AFp12, Error> {
        Ok(AFp12 {
            c0: self.fp6_assign(l, v.map(|x| x.c0()))?,
            c1: self.fp6_assign(l, v.map(|x| x.c1()))?,
        })
    }

    pub fn fp12_fixed(&self, l: &mut impl Layouter<F>, v: Fp12) -> Result<AFp12, Error> {
        Ok(AFp12 {
            c0: self.fp6_fixed(l, v.c0())?,
            c1: self.fp6_fixed(l, v.c1())?,
        })
    }

    /// Karatsuba Fp12 multiplication (3 Fp6 = 18 Fp2 multiplications).
    pub fn fp12_mul(&self, l: &mut impl Layouter<F>, a: &AFp12, b: &AFp12) -> Result<AFp12, Error> {
        if self.opts.flat {
            return self.flat_fp12_mul(l, a, b);
        }
        self.counts.borrow_mut().fp12_mul += 1;
        let t0 = self.fp6_mul(l, &a.c0, &b.c0)?;
        let t1 = self.fp6_mul(l, &a.c1, &b.c1)?;
        let vt1 = self.fp6_mul_by_v(l, &t1)?;
        let c0 = self.fp6_add(l, &t0, &vt1)?;
        let sa = self.fp6_add(l, &a.c0, &a.c1)?;
        let sb = self.fp6_add(l, &b.c0, &b.c1)?;
        let m = self.fp6_mul(l, &sa, &sb)?;
        let m = self.fp6_sub(l, &m, &t0)?;
        let c1 = self.fp6_sub(l, &m, &t1)?;
        self.fp12_nf(l, &AFp12 { c0, c1 })
    }

    /// Complex squaring (2 Fp6 multiplications).
    pub fn fp12_square(&self, l: &mut impl Layouter<F>, a: &AFp12) -> Result<AFp12, Error> {
        if self.opts.flat {
            return self.flat_fp12_square(l, a);
        }
        self.counts.borrow_mut().fp12_sqr += 1;
        // t = a0 a1; c0 = (a0 + a1)(a0 + v a1) - t - v t; c1 = 2t
        let t = self.fp6_mul(l, &a.c0, &a.c1)?;
        let s = self.fp6_add(l, &a.c0, &a.c1)?;
        let va1 = self.fp6_mul_by_v(l, &a.c1)?;
        let s2 = self.fp6_add(l, &a.c0, &va1)?;
        let m = self.fp6_mul(l, &s, &s2)?;
        let vt = self.fp6_mul_by_v(l, &t)?;
        let m = self.fp6_sub(l, &m, &t)?;
        let c0 = self.fp6_sub(l, &m, &vt)?;
        let c1 = self.fp6_add(l, &t, &t)?;
        self.fp12_nf(l, &AFp12 { c0, c1 })
    }

    /// f * l for a line value l = (b0 + b1 v) + v w (the divided line form);
    /// 10 Fp2 multiplications.
    ///   f l = (f0 A + v^2 f1) + (v f0 + f1 A) w,  A = b0 + b1 v.
    pub fn fp12_mul_by_line(
        &self,
        l: &mut impl Layouter<F>,
        f: &AFp12,
        b0: &AFp2,
        b1: &AFp2,
    ) -> Result<AFp12, Error> {
        if self.opts.flat {
            return self.flat_fp12_mul_by_line(l, f, b0, b1);
        }
        self.counts.borrow_mut().fp12_sparse += 1;
        let f0a = self.fp6_mul_by_01(l, &f.c0, b0, b1)?;
        let f1a = self.fp6_mul_by_01(l, &f.c1, b0, b1)?;
        // v^2 f1 = (xi f11, xi f12, f10)
        let v2f1 = AFp6 {
            c0: self.fp2_mul_by_xi(l, &f.c1.c1)?,
            c1: self.fp2_mul_by_xi(l, &f.c1.c2)?,
            c2: f.c1.c0.clone(),
        };
        let vf0 = self.fp6_mul_by_v(l, &f.c0)?;
        let c0 = self.fp6_add(l, &f0a, &v2f1)?;
        let c1 = self.fp6_add(l, &vf0, &f1a)?;
        self.fp12_nf(l, &AFp12 { c0, c1 })
    }

    /// f * s for s in Fp6 (2 Fp6 multiplications).
    pub fn fp12_mul_by_fp6(
        &self,
        l: &mut impl Layouter<F>,
        f: &AFp12,
        s: &AFp6,
    ) -> Result<AFp12, Error> {
        if self.opts.flat {
            return self.flat_fp12_mul_by_fp6(l, f, s);
        }
        Ok(AFp12 {
            c0: self.fp6_mul(l, &f.c0, s)?,
            c1: self.fp6_mul(l, &f.c1, s)?,
        })
    }

    /// f^p: (a_j w^j)^p = conj(a_j) gamma_j w^j.
    pub fn fp12_frobenius(
        &self,
        l: &mut impl Layouter<F>,
        f: &AFp12,
        gammas: &[Fp2; 6],
    ) -> Result<AFp12, Error> {
        let a = f.w_coeffs();
        let mut out = Vec::with_capacity(6);
        for j in 0..6 {
            let c = self.fp2_conj(l, a[j])?;
            let c = if j == 0 {
                c
            } else {
                self.fp2_mul_const(l, &c, gammas[j])?
            };
            out.push(c);
        }
        let arr: [AFp2; 6] = out.try_into().unwrap();
        Ok(AFp12::from_w_coeffs(arr))
    }

    pub fn fp12_assert_equal_to_fixed(
        &self,
        l: &mut impl Layouter<F>,
        a: &AFp12,
        k: Fp12,
    ) -> Result<(), Error> {
        let (k0, k1) = (k.c0(), k.c1());
        for (x, kx) in [
            (&a.c0.c0, k0.c0()),
            (&a.c0.c1, k0.c1()),
            (&a.c0.c2, k0.c2()),
            (&a.c1.c0, k1.c0()),
            (&a.c1.c1, k1.c1()),
            (&a.c1.c2, k1.c2()),
        ] {
            self.fp2_assert_equal_to_fixed(l, x, kx)?;
        }
        Ok(())
    }

    pub fn fp12_assert_equal(
        &self,
        l: &mut impl Layouter<F>,
        a: &AFp12,
        b: &AFp12,
    ) -> Result<(), Error> {
        for (x, y) in [
            (&a.c0.c0, &b.c0.c0),
            (&a.c0.c1, &b.c0.c1),
            (&a.c0.c2, &b.c0.c2),
            (&a.c1.c0, &b.c1.c0),
            (&a.c1.c1, &b.c1.c1),
            (&a.c1.c2, &b.c1.c2),
        ] {
            self.fp2_assert_equal(l, x, y)?;
        }
        Ok(())
    }
}
