//! The `poly_common` helpers of stark-backend v2.1.0 over `Alg` (`EF = F`, so every value is
//! one Fr cell). Constants that depend only on the key (`omega`, `1/2`, inverse factorials)
//! are host values.

use ff::Field;
use midnight_proofs::plonk::Error;

use crate::{alg::Alg, native::F};

/// `prod_i (1 - y_i - x_i + 2 x_i y_i)`.
pub fn eval_eq_mle<A: Alg>(a: &mut A, x: &[A::V], y: &[A::V]) -> Result<A::V, Error> {
    assert_eq!(x.len(), y.len());
    let mut acc = a.cst(F::ONE)?;
    for (xi, yi) in x.iter().zip(y) {
        let xy = a.mul(xi, yi)?;
        let t = a.lin(&[(-F::ONE, yi.clone()), (-F::ONE, xi.clone()), (F::from(2u64), xy)], F::ONE)?;
        acc = a.mul(&acc, &t)?;
    }
    Ok(acc)
}

/// `eq` with a constant boolean `y`: `prod_i (y_i ? x_i : 1 - x_i)` (no multiplication by y).
pub fn eval_eq_mle_bits<A: Alg>(a: &mut A, x: &[A::V], bits: &[bool]) -> Result<A::V, Error> {
    assert_eq!(x.len(), bits.len());
    let mut acc: Option<A::V> = None;
    for (xi, &b) in x.iter().zip(bits) {
        let t = if b { xi.clone() } else { a.lin(&[(-F::ONE, xi.clone())], F::ONE)? };
        acc = Some(match acc {
            None => t,
            Some(p) => a.mul(&p, &t)?,
        });
    }
    match acc {
        Some(v) => Ok(v),
        None => a.cst(F::ONE),
    }
}

/// `p(x)` for `p` through `(0, e0), (1, e1)`.
pub fn interpolate_linear_at_01<A: Alg>(a: &mut A, e: &[A::V; 2], x: &A::V) -> Result<A::V, Error> {
    let d = a.sub(&e[1], &e[0])?;
    let dx = a.mul(&d, x)?;
    a.add(&dx, &e[0])
}

/// `p(x)` for the quadratic through `(0, e0), (1, e1), (2, e2)`.
pub fn interpolate_quadratic_at_012<A: Alg>(a: &mut A, e: &[A::V; 3], x: &A::V) -> Result<A::V, Error> {
    let half = F::from(2u64).invert().unwrap();
    // p = (e2 - 2 e1 + e0) / 2, q = (e1 - e0) - p
    let p = a.lin(&[(half, e[2].clone()), (-F::ONE, e[1].clone()), (half, e[0].clone())], F::ZERO)?;
    let q = a.lin(&[(F::ONE, e[1].clone()), (-F::ONE, e[0].clone()), (-F::ONE, p.clone())], F::ZERO)?;
    let px = a.mul(&p, x)?;
    let t = a.add(&px, &q)?;
    let tx = a.mul(&t, x)?;
    a.add(&tx, &e[0])
}

/// `p(x)` for the cubic through `(i, e_i)`, `i = 0..3`.
pub fn interpolate_cubic_at_0123<A: Alg>(a: &mut A, e: &[A::V; 4], x: &A::V) -> Result<A::V, Error> {
    let inv6 = F::from(6u64).invert().unwrap();
    let half = F::from(2u64).invert().unwrap();
    // s_i = e_i - e_0; d3 = s3 - 3 (s2 - s1) = e3 - 3 e2 + 3 e1 - e0
    let d3 = a.lin(
        &[(F::ONE, e[3].clone()), (-F::from(3u64), e[2].clone()), (F::from(3u64), e[1].clone()), (-F::ONE, e[0].clone())],
        F::ZERO,
    )?;
    let p = a.mul_c(&d3, inv6)?;
    // q = (s2 - d3) / 2 - s1 = e2/2 - e0/2 - d3/2 - e1 + e0
    let q = a.lin(
        &[(half, e[2].clone()), (half, e[0].clone()), (-half, d3), (-F::ONE, e[1].clone())],
        F::ZERO,
    )?;
    // r = s1 - p - q
    let r = a.lin(&[(F::ONE, e[1].clone()), (-F::ONE, e[0].clone()), (-F::ONE, p.clone()), (-F::ONE, q.clone())], F::ZERO)?;
    let mut acc = a.mul(&p, x)?;
    acc = a.add(&acc, &q)?;
    acc = a.mul(&acc, x)?;
    acc = a.add(&acc, &r)?;
    acc = a.mul(&acc, x)?;
    a.add(&acc, &e[0])
}

/// Horner: `sum c_i x^i`.
pub fn horner<A: Alg>(a: &mut A, coeffs: &[A::V], x: &A::V) -> Result<A::V, Error> {
    let mut acc = a.cst(F::ZERO)?;
    for c in coeffs.iter().rev() {
        let t = a.mul(&acc, x)?;
        acc = a.add(&t, c)?;
    }
    Ok(acc)
}

/// `x^(2^i)` for `i = 0..n`.
pub fn exp_powers_of_2<A: Alg>(a: &mut A, x: &A::V, n: usize) -> Result<Vec<A::V>, Error> {
    let mut v = vec![x.clone()];
    for _ in 1..n {
        let l = v.last().unwrap().clone();
        v.push(a.mul(&l, &l)?);
    }
    Ok(v)
}

/// `1, x, x^2, ..` (`n` values).
pub fn powers<A: Alg>(a: &mut A, x: &A::V, n: usize) -> Result<Vec<A::V>, Error> {
    let mut v = Vec::with_capacity(n);
    if n == 0 {
        return Ok(v);
    }
    v.push(a.cst(F::ONE)?);
    for i in 1..n {
        v.push(if i == 1 { x.clone() } else { a.mul(&v[i - 1], x)? });
    }
    Ok(v)
}
