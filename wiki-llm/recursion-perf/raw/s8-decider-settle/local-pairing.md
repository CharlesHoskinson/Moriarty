# Local mori-recursion pairing.rs
> Source: file:///home/charl/code/Moriarty-perf/crates/mori-recursion/src/pairing.rs
> Collected: 2026-10-26
> Published: Unknown

// Adapted from projectfenrir recursion/stagenet-strawman/recl9.
// Compiled on the pinned stock graph; installed-key equality is checked separately.
//! Residue-witness pairing-product check.
//!
//! Checks prod_i e(P_i, Q_i) = 1 without a final exponentiation:
//!
//!   1. The prover supplies c_inv (= c^{-1}) in Fp12 and s in Fp6 such that
//!      F * s = c^lambda, F the multi-Miller-loop output, lambda = p + |x|
//!      (a multiple of r because p = x mod r).
//!   2. The circuit runs the Miller loop with the accumulator initialised to
//!      c_inv and multiplied by c_inv at every 1-bit of |x|, which yields
//!      g = F * c_inv^{|x|} at no extra squarings.
//!   3. The circuit asserts g * s * frobenius(c_inv) = 1.
//!
//! See NOTES.md for the soundness argument (why s in Fp6 is harmless and why
//! c_inv != 0 is implied).

use ff::Field;
use group::prime::PrimeCurveAffine;
use midnight_circuits::{
    instructions::{ArithInstructions, EccInstructions, ZeroInstructions},
    types::{AssignedBit, AssignedForeignPoint, InnerValue},
};
use midnight_curves::{
    Fp, G1Affine, G1Projective, G2Affine,
    bls12_381::{Fp2, Fp12},
};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};

use crate::{
    reference::{self, Line, NB_STEPS, StepLines, frobenius_gammas, x_bit},
    tower::{AFp, AFp2, AFp6, AFp12, F, Tower},
};

/// G1 point as the zk-stdlib's BLS12-381 chip assigns it.
pub type AG1 = AssignedForeignPoint<F, G1Projective, G1Projective>;

/// A G1 input prepared for the divided line form: yinv = 1/y, xny = -x/y,
/// both 0 for the identity.
#[derive(Clone, Debug)]
pub struct PreparedG1 {
    pub yinv: AFp,
    pub xny: AFp,
}

/// Assigned G2 point (affine, never the identity).
#[derive(Clone, Debug)]
pub struct AG2 {
    pub x: AFp2,
    pub y: AFp2,
}

/// One line with in-circuit coefficients.
#[derive(Clone, Debug)]
pub struct VarLine {
    pub c: AFp2,
    pub lambda: AFp2,
}

#[derive(Clone, Debug)]
pub struct VarStepLines {
    pub double: VarLine,
    pub add: Option<VarLine>,
}

/// Where the lines of a pair come from.
#[derive(Clone, Debug)]
pub enum G2Lines {
    /// Fixed G2 point: every line coefficient is a circuit constant.
    Fixed(Vec<StepLines>),
    /// Variable G2 point: lines computed in-circuit.
    Variable(Vec<VarStepLines>),
}

impl G2Lines {
    fn value_lines(&self) -> Value<Vec<StepLines>> {
        match self {
            G2Lines::Fixed(v) => Value::known(v.clone()),
            G2Lines::Variable(v) => {
                let vals: Vec<Value<StepLines>> = v
                    .iter()
                    .map(|s| {
                        let d = s.double.c.value().zip(s.double.lambda.value());
                        let a = match &s.add {
                            None => Value::known(None),
                            Some(a) => a.c.value().zip(a.lambda.value()).map(Some),
                        };
                        d.zip(a).map(|((c, lambda), add)| StepLines {
                            double: Line { c, lambda },
                            add: add.map(|(c, lambda)| Line { c, lambda }),
                        })
                    })
                    .collect();
                Value::from_iter(vals)
            }
        }
    }
}

/// Cost-attribution hooks: the caller can wrap phases to measure them.
pub struct PairingGadget<'a> {
    pub tower: Tower<'a>,
    pub gammas: [Fp2; 6],
}

impl<'a> PairingGadget<'a> {
    pub fn new(tower: Tower<'a>) -> Self {
        PairingGadget {
            tower,
            gammas: frobenius_gammas(),
        }
    }

    /// Prepare a G1 point (x, y, is_id) for the Miller loop.
    pub fn prepare_g1(
        &self,
        l: &mut impl Layouter<F>,
        x: &AFp,
        y: &AFp,
        is_id: &AssignedBit<F>,
    ) -> Result<PreparedG1, Error> {
        let t = &self.tower;
        let one = t.fp_fixed(l, Fp::ONE)?;
        let zero = t.fp_fixed(l, Fp::ZERO)?;
        let y1 = t.fp_select(l, is_id, &one, y)?;
        // inv asserts y1 * yinv = 1 (unsatisfiable for y1 = 0).
        let yinv = t.fp.inv(l, &y1)?;
        let yinv = t.fp_select(l, is_id, &zero, &yinv)?;
        let xy = t.fp_mul(l, x, &yinv)?;
        let xny = t.fp.neg(l, &xy)?;
        Ok(PreparedG1 { yinv, xny })
    }

    /// Prepare a G1 point assigned by the zk-stdlib BLS12-381 chip.
    pub fn prepare_assigned_g1<E>(
        &self,
        l: &mut impl Layouter<F>,
        ecc: &E,
        p: &AG1,
    ) -> Result<PreparedG1, Error>
    where
        E: EccInstructions<F, G1Projective, Point = AG1, Coordinate = AFp>
            + ZeroInstructions<F, AG1>,
    {
        let is_id = ecc.is_zero(l, p)?;
        let x = ecc.x_coordinate(p);
        let y = ecc.y_coordinate(p);
        self.prepare_g1(l, &x, &y, &is_id)
    }

    fn eval_line(
        &self,
        l: &mut impl Layouter<F>,
        g1: &PreparedG1,
        lines: &G2Lines,
        step: usize,
        add: bool,
    ) -> Result<Option<(AFp2, AFp2)>, Error> {
        let t = &self.tower;
        match lines {
            G2Lines::Fixed(v) => {
                let line = if add {
                    v[step].add
                } else {
                    Some(v[step].double)
                };
                match line {
                    None => Ok(None),
                    Some(line) => {
                        let b0 = t.fp2_const_times_fp(l, line.c, &g1.yinv)?;
                        let b1 = t.fp2_const_times_fp(l, line.lambda, &g1.xny)?;
                        Ok(Some((b0, b1)))
                    }
                }
            }
            G2Lines::Variable(v) => {
                let line = if add {
                    v[step].add.clone()
                } else {
                    Some(v[step].double.clone())
                };
                match line {
                    None => Ok(None),
                    Some(line) => {
                        let b0 = t.fp2_mul_by_fp(l, &line.c, &g1.yinv)?;
                        let b1 = t.fp2_mul_by_fp(l, &line.lambda, &g1.xny)?;
                        Ok(Some((b0, b1)))
                    }
                }
            }
        }
    }

    /// One Miller-loop iteration (index `step`, bit `i = 62 - step` of |x|):
    /// square, fold c_inv on a 1-bit, multiply in the doubling lines, then the
    /// addition lines.
    pub fn miller_step(
        &self,
        l: &mut impl Layouter<F>,
        f: &AFp12,
        pairs: &[(PreparedG1, G2Lines)],
        step: usize,
        c_inv: Option<&AFp12>,
    ) -> Result<AFp12, Error> {
        let t = &self.tower;
        let i = NB_STEPS - 1 - step;
        let mut f = t.fp12_square(l, f)?;
        if x_bit(i)
            && let Some(ci) = c_inv
        {
            f = t.fp12_mul(l, &f, ci)?;
        }
        for add in [false, true] {
            for (g1, lines) in pairs {
                if let Some((b0, b1)) = self.eval_line(l, g1, lines, step, add)? {
                    f = t.fp12_mul_by_line(l, &f, &b0, &b1)?;
                }
            }
        }
        Ok(f)
    }

    /// The residue hint, computed from the assigned values (prover side).
    fn hint(&self, pairs: &[(PreparedG1, G2Lines)]) -> Value<reference::ResidueHint> {
        type MillerPairValue = ((Fp, Fp), Vec<StepLines>);
        let vals: Vec<Value<MillerPairValue>> = pairs
            .iter()
            .map(|(g1, lines)| g1.yinv.value().zip(g1.xny.value()).zip(lines.value_lines()))
            .collect();
        Value::from_iter(vals).map(|v: Vec<((Fp, Fp), Vec<StepLines>)>| {
            let inputs: Vec<((Fp, Fp), &[StepLines])> =
                v.iter().map(|(p, ls)| (*p, ls.as_slice())).collect();
            reference::residue_hint(&inputs)
        })
    }

    /// Assert prod_i e(P_i, Q_i) = 1 with the residue method.
    /// Returns the operation counts of the phases (Miller loop, residue
    /// check) through the tower counters.
    pub fn assert_pairing_product_is_one(
        &self,
        l: &mut impl Layouter<F>,
        pairs: &[(PreparedG1, G2Lines)],
    ) -> Result<(), Error> {
        let t = &self.tower;
        let hint = self.hint(pairs);
        let c_inv = t.fp12_assign(l, hint.as_ref().map(|h| h.c_inv))?;
        let s = t.fp6_assign(l, hint.as_ref().map(|h| h.s))?;

        let mut g = c_inv.clone();
        for step in 0..NB_STEPS {
            g = self.miller_step(l, &g, pairs, step, Some(&c_inv))?;
        }
        self.residue_finish(l, &g, &c_inv, &s)
    }

    /// g * s * frobenius(c_inv) == 1.
    pub fn residue_finish(
        &self,
        l: &mut impl Layouter<F>,
        g: &AFp12,
        c_inv: &AFp12,
        s: &AFp6,
    ) -> Result<(), Error> {
        let t = &self.tower;
        let gs = t.fp12_mul_by_fp6(l, g, s)?;
        let fc = t.fp12_frobenius(l, c_inv, &self.gammas)?;
        let z = t.fp12_mul(l, &gs, &fc)?;
        t.fp12_assert_equal_to_fixed(l, &z, Fp12::ONE)
    }

    // ------------------------------------------------------- variable G2

    /// Assign a G2 point (affine coordinates) and check it is on the twist.
    pub fn assign_g2(&self, l: &mut impl Layouter<F>, q: Value<G2Affine>) -> Result<AG2, Error> {
        let t = &self.tower;
        let x = t.fp2_assign(l, q.map(|q| q.x()))?;
        let y = t.fp2_assign(l, q.map(|q| q.y()))?;
        let q = AG2 { x, y };
        self.assert_on_twist(l, &q)?;
        Ok(q)
    }

    /// Assign a G2 point given by raw coordinates and an identity flag. When
    /// the flag is set, the coordinates are replaced by the generator's (so
    /// every later check passes) and the caller must make the pair contribute
    /// 1 (by passing the flag to `prepare_g1`). Checks the point is on the
    /// twist.
    pub fn assign_g2_raw(
        &self,
        l: &mut impl Layouter<F>,
        x: Value<Fp2>,
        y: Value<Fp2>,
        is_id: &AssignedBit<F>,
    ) -> Result<AG2, Error> {
        let t = &self.tower;
        let g = G2Affine::generator();
        let xa = t.fp2_assign(l, x)?;
        let ya = t.fp2_assign(l, y)?;
        let gx = t.fp2_fixed(l, g.x())?;
        let gy = t.fp2_fixed(l, g.y())?;
        let q = AG2 {
            x: t.fp2_select(l, is_id, &gx, &xa)?,
            y: t.fp2_select(l, is_id, &gy, &ya)?,
        };
        self.assert_on_twist(l, &q)?;
        Ok(q)
    }

    /// y^2 = x^3 + 4 (u + 1).
    pub fn assert_on_twist(&self, l: &mut impl Layouter<F>, q: &AG2) -> Result<(), Error> {
        let t = &self.tower;
        let y2 = t.fp2_square(l, &q.y)?;
        let x2 = t.fp2_square(l, &q.x)?;
        let x3 = t.fp2_mul(l, &x2, &q.x)?;
        let b = Fp2::new(Fp::from(4u64), Fp::from(4u64));
        let rhs = AFp2 {
            c0: t.fp.add_constant(l, &x3.c0, b.c0())?,
            c1: t.fp.add_constant(l, &x3.c1, b.c1())?,
        };
        t.fp2_assert_equal(l, &y2, &rhs)
    }

    /// Compute all Miller-loop lines of an assigned G2 point in-circuit and,
    /// from the same double-and-add chain, assert that Q lies in G2:
    /// [|x|]Q = -psi(Q) (equivalently psi(Q) = [x]Q, x = -|x|).
    pub fn g2_lines_and_subgroup_check(
        &self,
        l: &mut impl Layouter<F>,
        q: &AG2,
    ) -> Result<G2Lines, Error> {
        let t = &self.tower;
        let mut tx = q.x.clone();
        let mut ty = q.y.clone();
        let mut out = Vec::with_capacity(NB_STEPS);
        for step in 0..NB_STEPS {
            let i = NB_STEPS - 1 - step;
            // doubling: lambda = 3 x^2 / (2 y)
            let x2 = t.fp2_square(l, &tx)?;
            let n = AFp2 {
                c0: t.fp.mul_by_constant(l, &x2.c0, Fp::from(3u64))?,
                c1: t.fp.mul_by_constant(l, &x2.c1, Fp::from(3u64))?,
            };
            let d = t.fp2_double(l, &ty)?;
            let dinv = t.fp2_inv(l, &d)?;
            let lambda = t.fp2_mul(l, &n, &dinv)?;
            let (nx, ny, c) = self.line_and_next(l, &lambda, &tx, &ty, &tx)?;
            let double = VarLine { c, lambda };
            tx = nx;
            ty = ny;
            let add = if x_bit(i) {
                // lambda = (yQ - yT) / (xQ - xT)
                let n = t.fp2_sub(l, &q.y, &ty)?;
                let d = t.fp2_sub(l, &q.x, &tx)?;
                let dinv = t.fp2_inv(l, &d)?;
                let lambda = t.fp2_mul(l, &n, &dinv)?;
                let (nx, ny, c) = self.line_and_next(l, &lambda, &tx, &ty, &q.x)?;
                tx = nx;
                ty = ny;
                Some(VarLine { c, lambda })
            } else {
                None
            };
            out.push(VarStepLines { double, add });
        }
        // subgroup check: T = [|x|]Q must equal -psi(Q),
        // psi(x, y) = (conj(x) / gamma_2, conj(y) / gamma_3).
        let g2inv = self.gammas[2].invert().unwrap();
        let g3inv = self.gammas[3].invert().unwrap();
        let cx = t.fp2_conj(l, &q.x)?;
        let cy = t.fp2_conj(l, &q.y)?;
        let psi_x = t.fp2_mul_const(l, &cx, g2inv)?;
        let psi_y = t.fp2_mul_const(l, &cy, -g3inv)?; // already negated
        t.fp2_assert_equal(l, &tx, &psi_x)?;
        t.fp2_assert_equal(l, &ty, &psi_y)?;
        Ok(G2Lines::Variable(out))
    }

    /// Given the slope through T (and a second point with x-coordinate
    /// `x_other`; for doubling x_other = xT), return (x3, y3, C) with
    /// x3 = lambda^2 - xT - x_other, y3 = lambda (xT - x3) - yT,
    /// C = lambda xT - yT.
    fn line_and_next(
        &self,
        l: &mut impl Layouter<F>,
        lambda: &AFp2,
        tx: &AFp2,
        ty: &AFp2,
        x_other: &AFp2,
    ) -> Result<(AFp2, AFp2, AFp2), Error> {
        let t = &self.tower;
        let l2 = t.fp2_square(l, lambda)?;
        let x3 = t.fp2_sum(l, &[&l2], &[tx, x_other])?;
        let lx = t.fp2_mul(l, lambda, tx)?;
        let c = t.fp2_sub(l, &lx, ty)?;
        let lx3 = t.fp2_mul(l, lambda, &x3)?;
        let y3 = t.fp2_sub(l, &c, &lx3)?;
        Ok((x3, y3, c))
    }
}

/// Lines of a fixed G2 point, after checking that no line constant would make
/// a line value degenerate.
pub fn fixed_lines(q: &G2Affine) -> G2Lines {
    let lines = reference::g2_lines(q);
    G2Lines::Fixed(lines)
}

/// The two fixed G2 points of the KZG decider: [tau]_2 and -[1]_2.
pub fn kzg_decider_lines(s_g2: &G2Affine) -> (G2Lines, G2Lines) {
    let neg_g2: G2Affine = (-midnight_curves::G2Projective::from(G2Affine::generator())).into();
    (fixed_lines(s_g2), fixed_lines(&neg_g2))
}

/// Off-circuit: does e(A, s_g2) * e(B, -g2) = 1 hold (the DualMSM check)?
pub fn kzg_decider_reference(a: &G1Affine, b: &G1Affine, s_g2: &G2Affine) -> bool {
    let neg_g2: G2Affine = (-midnight_curves::G2Projective::from(G2Affine::generator())).into();
    let f = reference::multi_miller_loop(&[(*a, *s_g2), (*b, neg_g2)]);
    reference::final_exp(&f) == Fp12::ONE
}
