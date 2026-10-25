// Adapted from projectfenrir recursion/stagenet-strawman/recl9.
// Compiled on the pinned stock graph; installed-key equality is checked separately.
//! Off-circuit reference for the residue-witness pairing check.
//!
//! Tower (the same one blst and midnight-curves use):
//!   Fp2  = Fp[u]  / (u^2 + 1)
//!   Fp6  = Fp2[v] / (v^3 - xi),  xi = u + 1
//!   Fp12 = Fp6[w] / (w^2 - v)
//!
//! The G2 twist is the M-twist E': y^2 = x^3 + 4 xi, untwisted by
//! (x', y') -> (x' / w^2, y' / w^3).
//!
//! Line convention. For a line of slope `lambda` (on the twist) through a twist
//! point T = (xT, yT), the line function evaluated at P = (xP, yP) in E(Fp),
//! multiplied by w^3, is
//!
//!   l(P) = (lambda * xT - yT)  +  (-lambda * xP) v  +  yP (v w)
//!
//! i.e. the Fp12 element  c0 = (C, -lambda*xP, 0),  c1 = (0, yP, 0)  with
//! C = lambda*xT - yT.  The factor w^3 and the omitted vertical lines lie in
//! subgroups killed by the final exponentiation (see NOTES.md), so the product
//! F of these line values satisfies F^h = prod e(P_i, Q_i)^(+-1),
//! h = (p^12 - 1) / r.
//!
//! The Miller loop runs over |x| = 0xd201000000010000 (x is negative); the
//! sign does not matter for a "product equals one" check.

use ff::Field;
use group::prime::PrimeCurveAffine;
use midnight_circuits::CircuitField;
use midnight_curves::{
    Fp, Fq, G1Affine, G2Affine,
    bls12_381::{Fp2, Fp6, Fp12},
};
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{One, Zero};

/// |x| for BLS12-381 (x = -0xd201000000010000).
pub const X_ABS: u64 = 0xd201_0000_0001_0000;

/// Number of Miller-loop iterations (bits 62..0 of |x|).
pub const NB_STEPS: usize = 63;

/// Bit `i` of |x|.
pub fn x_bit(i: usize) -> bool {
    (X_ABS >> i) & 1 == 1
}

/// The modulus p of Fp as a BigUint.
pub fn p_big() -> BigUint {
    Fp::modulus()
}

/// The order r of G1/G2/GT as a BigUint.
pub fn r_big() -> BigUint {
    Fq::modulus()
}

/// h = (p^12 - 1) / r.
pub fn final_exp_big() -> BigUint {
    let p = p_big();
    let r = r_big();
    let n = p.pow(12) - BigUint::one();
    let (q, rem) = n.div_rem(&r);
    assert!(rem.is_zero());
    q
}

/// lambda = p + |x| = p - x, a multiple of r.
pub fn lambda_big() -> BigUint {
    p_big() + BigUint::from(X_ABS)
}

/// Fp12 exponentiation by a BigUint.
pub fn pow12(f: &Fp12, e: &BigUint) -> Fp12 {
    f.pow_vartime(e.to_u64_digits())
}

/// Fp2 exponentiation by a BigUint.
pub fn pow2(f: &Fp2, e: &BigUint) -> Fp2 {
    f.pow_vartime(e.to_u64_digits())
}

/// Naive final exponentiation f^h.
pub fn final_exp(f: &Fp12) -> Fp12 {
    pow12(f, &final_exp_big())
}

pub fn fp2(c0: Fp, c1: Fp) -> Fp2 {
    Fp2::new(c0, c1)
}

pub fn fp2_zero() -> Fp2 {
    Fp2::ZERO
}

/// xi = u + 1.
pub fn xi() -> Fp2 {
    Fp2::new(Fp::ONE, Fp::ONE)
}

/// Coefficients of a line: C = lambda*xT - yT and lambda.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Line {
    pub c: Fp2,
    pub lambda: Fp2,
}

/// The lines used in one Miller-loop iteration for one G2 point.
#[derive(Clone, Debug)]
pub struct StepLines {
    pub double: Line,
    pub add: Option<Line>,
}

/// Affine point on the twist (never the identity in the loop).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TwistPoint {
    pub x: Fp2,
    pub y: Fp2,
}

impl TwistPoint {
    pub fn from_affine(q: &G2Affine) -> Self {
        TwistPoint { x: q.x(), y: q.y() }
    }
}

/// Affine doubling on the twist; returns 2T and the tangent line at T.
pub fn twist_double(t: &TwistPoint) -> (TwistPoint, Line) {
    let three = Fp2::from(3u64);
    let two = Fp2::from(2u64);
    let lambda = three * t.x.square() * (two * t.y).invert().unwrap();
    let x3 = lambda.square() - t.x - t.x;
    let y3 = lambda * (t.x - x3) - t.y;
    let c = lambda * t.x - t.y;
    (TwistPoint { x: x3, y: y3 }, Line { c, lambda })
}

/// Affine addition on the twist; returns T + Q and the line through T and Q.
pub fn twist_add(t: &TwistPoint, q: &TwistPoint) -> (TwistPoint, Line) {
    let lambda = (q.y - t.y) * (q.x - t.x).invert().unwrap();
    let x3 = lambda.square() - t.x - q.x;
    let y3 = lambda * (t.x - x3) - t.y;
    let c = lambda * t.x - t.y;
    (TwistPoint { x: x3, y: y3 }, Line { c, lambda })
}

/// All Miller-loop lines for a fixed G2 point, in loop order (bit 62 first).
pub fn g2_lines(q: &G2Affine) -> Vec<StepLines> {
    let q = TwistPoint::from_affine(q);
    let mut t = q;
    let mut out = Vec::with_capacity(NB_STEPS);
    for i in (0..NB_STEPS).rev() {
        let (t2, ld) = twist_double(&t);
        t = t2;
        let add = if x_bit(i) {
            let (t3, la) = twist_add(&t, &q);
            t = t3;
            Some(la)
        } else {
            None
        };
        out.push(StepLines { double: ld, add });
    }
    out
}

/// Evaluate a line at P, in the divided form l(P) / yP:
///   (C * yinv, lambda * xny, 0) + (0, 1, 0) w,   yinv = 1/yP, xny = -xP/yP.
pub fn line_eval(l: &Line, yinv: Fp, xny: Fp) -> Fp12 {
    let c0 = Fp6::new(
        l.c * Fp2::new(yinv, Fp::ZERO),
        l.lambda * Fp2::new(xny, Fp::ZERO),
        Fp2::ZERO,
    );
    let c1 = Fp6::new(Fp2::ZERO, Fp2::ONE, Fp2::ZERO);
    Fp12::new(c0, c1)
}

/// (1/yP, -xP/yP) of a G1 point, with the identity mapped to (0, 0). With
/// (0, 0) every line value is the constant v w (= w^3), which the final
/// exponentiation kills, so the pair contributes 1. yP is never 0 for an
/// affine point of E(Fp): the G1 cofactor is odd, so E(Fp) has no 2-torsion.
pub fn g1_xy(p: &G1Affine) -> (Fp, Fp) {
    if bool::from(p.is_identity()) {
        (Fp::ZERO, Fp::ZERO)
    } else {
        let yinv = p.y().invert().unwrap();
        (yinv, -(p.x() * yinv))
    }
}

/// Multi-Miller loop F = prod_i f_{|x|, Q_i}(P_i) with our line convention,
/// starting from `init` (1 for the plain loop). When `c_inv` is given, the
/// accumulator is also multiplied by `c_inv` at every 1-bit, so that with
/// init = c_inv the result is F * c_inv^{|x|}.
pub fn multi_miller_loop_with(
    pairs: &[((Fp, Fp), &[StepLines])],
    init: Fp12,
    c_inv: Option<Fp12>,
) -> Fp12 {
    let mut f = init;
    for (step, i) in (0..NB_STEPS).rev().enumerate() {
        f = f.square();
        if x_bit(i)
            && let Some(ci) = c_inv
        {
            f *= ci;
        }
        for ((xp, yp), lines) in pairs {
            f *= line_eval(&lines[step].double, *xp, *yp);
        }
        for ((xp, yp), lines) in pairs {
            if let Some(la) = &lines[step].add {
                f *= line_eval(la, *xp, *yp);
            }
        }
    }
    f
}

/// Plain multi-Miller loop.
pub fn multi_miller_loop(pairs: &[(G1Affine, G2Affine)]) -> Fp12 {
    let lines: Vec<Vec<StepLines>> = pairs.iter().map(|(_, q)| g2_lines(q)).collect();
    let inputs: Vec<((Fp, Fp), &[StepLines])> = pairs
        .iter()
        .zip(lines.iter())
        .map(|((p, _), l)| (g1_xy(p), l.as_slice()))
        .collect();
    multi_miller_loop_with(&inputs, Fp12::ONE, None)
}

/// Frobenius coefficients gamma_j = xi^{j (p - 1) / 6}, j = 0..5, so that
/// (a w^j)^p = conj(a) gamma_j w^j for a in Fp2.
pub fn frobenius_gammas() -> [Fp2; 6] {
    let e = (p_big() - BigUint::one()) / BigUint::from(6u32);
    let g1 = pow2(&xi(), &e);
    let mut out = [Fp2::ONE; 6];
    for j in 1..6 {
        out[j] = out[j - 1] * g1;
    }
    out
}

/// Residue witness for a Miller-loop output F with F^h = 1 (Novakovic-Eagen,
/// with gnark's choice of the scaling factor). Returns (c, s) with
/// F * s = c^lambda, lambda = p + |x|, and s in Fp6 (c1 component zero).
/// Returns None if F^h != 1 (no witness exists).
pub fn residue_witness(f: &Fp12) -> Option<(Fp12, Fp12)> {
    let h = final_exp_big();
    if pow12(f, &h) != Fp12::ONE {
        return None;
    }
    // poly_factor = (1 - x) / 3 = (1 + |x|) / 3
    let poly_factor = (BigUint::from(X_ABS) + BigUint::one()) / BigUint::from(3u32);
    let twenty_seven = BigUint::from(27u32);
    let (fef, rem) = h.div_rem(&(&twenty_seven * &poly_factor));
    assert!(rem.is_zero(), "27 * (1-x)/3 must divide h");

    // 1. component of order dividing poly_factor
    let e = &fef * &twenty_seven;
    let root = pow12(f, &e);
    let root_pth_inv = if root == Fp12::ONE {
        Fp12::ONE
    } else {
        let inv = mod_inverse(&(&e % &poly_factor), &poly_factor);
        let exp = (&poly_factor - inv) % &poly_factor;
        pow12(&root, &exp)
    };
    // 2. component of order dividing 27
    let e2 = &poly_factor * &fef;
    let root = pow12(f, &e2);
    let root_27_inv = if root == Fp12::ONE {
        Fp12::ONE
    } else {
        let inv = mod_inverse(&(&e2 % &twenty_seven), &twenty_seven);
        let exp = (&twenty_seven - inv) % &twenty_seven;
        pow12(&root, &exp)
    };
    let s = root_pth_inv * root_27_inv;
    let fs = *f * s;
    let lambda = lambda_big();
    let lambda_inv = mod_inverse(&(&lambda % &fef), &fef);
    let c = pow12(&fs, &lambda_inv);
    debug_assert_eq!(pow12(&c, &lambda), fs);
    Some((c, s))
}

/// Modular inverse via extended Euclid (a and m coprime).
pub fn mod_inverse(a: &BigUint, m: &BigUint) -> BigUint {
    use num_bigint::BigInt;
    let (mut old_r, mut r) = (BigInt::from(a.clone()), BigInt::from(m.clone()));
    let (mut old_s, mut s) = (BigInt::one(), BigInt::zero());
    while !r.is_zero() {
        let q = &old_r / &r;
        let tmp = &old_r - &q * &r;
        old_r = std::mem::replace(&mut r, tmp);
        let tmp = &old_s - &q * &s;
        old_s = std::mem::replace(&mut s, tmp);
    }
    assert!(old_r.is_one(), "not invertible");
    let m_i = BigInt::from(m.clone());
    let res = ((old_s % &m_i) + &m_i) % &m_i;
    res.to_biguint().unwrap()
}

/// Everything the in-circuit residue check consumes, computed off-circuit.
#[derive(Clone, Debug)]
pub struct ResidueHint {
    /// The Miller-loop output F (for tests).
    pub f: Fp12,
    /// c^{-1}.
    pub c_inv: Fp12,
    /// Scaling factor s, an element of Fp6.
    pub s: Fp6,
}

/// Compute the hint for a pairing-product check. If the product is not one, a
/// dummy (all-one) hint is returned; the circuit then rejects.
pub fn residue_hint(pairs: &[((Fp, Fp), &[StepLines])]) -> ResidueHint {
    let f = multi_miller_loop_with(pairs, Fp12::ONE, None);
    match residue_witness(&f) {
        Some((c, s)) => {
            assert_eq!(s.c1(), Fp6::ZERO, "scaling factor must lie in Fp6");
            ResidueHint {
                f,
                c_inv: c.invert().unwrap(),
                s: s.c0(),
            }
        }
        None => ResidueHint {
            f,
            c_inv: Fp12::ONE,
            s: Fp6::ONE,
        },
    }
}

/// The full residue check, off-circuit, exactly as the circuit computes it:
/// g = Miller loop started at c_inv with c_inv folded in at the 1-bits, then
/// g * s * frobenius(c_inv) == 1.
pub fn residue_check(pairs: &[((Fp, Fp), &[StepLines])], hint: &ResidueHint) -> bool {
    let g = multi_miller_loop_with(pairs, hint.c_inv, Some(hint.c_inv));
    let mut fc = hint.c_inv;
    fc.frobenius_map(1);
    g * Fp12::new(hint.s, Fp6::ZERO) * fc == Fp12::ONE
}
