//! I5: the stacked reduction (`verifier/stacked_reduction.rs` of stark-backend v2.1.0) over
//! `Alg`. The layouts are pinned from the key shape (the exporter writes the stock
//! `StackedLayout`s); the proof values are witnesses.

use std::collections::HashMap;

use ff::Field;
use midnight_proofs::{circuit::Value, plonk::Error};

use crate::{
    alg::Alg,
    native::{F, two_adic_generator},
    poly::{eval_eq_mle, eval_eq_mle_bits, exp_powers_of_2, horner, interpolate_quadratic_at_012, powers},
    transcript::Transcript,
    verifier::{Openings, Shape, eq_uni, eq_uni_at_one},
    wrapin::{WrapIn, frs, us},
};

/// One stacked column slice: `(mat_idx, col_idx in the stacked matrix, log_height, row_idx)`.
#[derive(Clone, Debug)]
pub struct Slice {
    pub mat: usize,
    pub col_idx: usize,
    pub log_height: usize,
    pub row_idx: usize,
}

/// The pinned stacking shape.
#[derive(Clone, Debug)]
pub struct StackShape {
    pub n_stack: usize,
    /// Per commitment: the sorted columns.
    pub layouts: Vec<Vec<Slice>>,
    /// Per commitment: the stacked width (number of opened stacked columns).
    pub widths: Vec<usize>,
    /// Number of univariate round coefficients.
    pub n_uni: usize,
}

impl StackShape {
    pub fn from_wrapin(w: &WrapIn, s: &Shape) -> Self {
        let layouts: Vec<Vec<Slice>> = w.doc["layouts"]
            .as_array()
            .unwrap()
            .iter()
            .map(|l| {
                l["sorted_cols"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|c| Slice {
                        mat: us(&c[0]),
                        col_idx: us(&c[2]["col_idx"]),
                        log_height: us(&c[2]["log_height"]),
                        row_idx: us(&c[2]["row_idx"]),
                    })
                    .collect()
            })
            .collect();
        let widths = layouts.iter().map(|l| l.iter().map(|c| c.col_idx + 1).max().unwrap_or(0)).collect();
        // s_0 has degree 2 (2^l_skip - 1): eq times the (rotated) column.
        StackShape { n_stack: s.n_stack, layouts, widths, n_uni: 2 * ((1 << s.l_skip) - 1) + 1 }
    }
}

/// The stacking proof values.
#[derive(Clone, Debug, Default)]
pub struct StackData {
    pub univariate_round_coeffs: Vec<F>,
    pub sumcheck_round_polys: Vec<Vec<F>>,
    pub stacking_openings: Vec<Vec<F>>,
}

impl StackData {
    pub fn from_wrapin(w: &WrapIn) -> Self {
        let p = &w.proof()["stacking_proof"];
        StackData {
            univariate_round_coeffs: frs(&p["univariate_round_coeffs"]),
            sumcheck_round_polys: p["sumcheck_round_polys"].as_array().unwrap().iter().map(frs).collect(),
            stacking_openings: p["stacking_openings"].as_array().unwrap().iter().map(frs).collect(),
        }
    }
}

/// `eval_eq_rot_cube`: `(eq(x, y), rot(x, y))`.
fn eq_rot_cube<A: Alg>(a: &mut A, x: &[A::V], y: &[A::V]) -> Result<(A::V, A::V), Error> {
    let mut rot = a.cst(F::ONE)?;
    let mut eq = a.cst(F::ONE)?;
    for i in (0..x.len()).rev() {
        let xy = a.mul(&x[i], &y[i])?;
        // x (1 - y) = x - xy ; (1 - x) y = y - xy
        let p = a.sub(&x[i], &xy)?;
        let q = a.sub(&y[i], &xy)?;
        let pe = a.mul(&p, &eq)?;
        let qr = a.mul(&q, &rot)?;
        rot = a.add(&pe, &qr)?;
        let e = a.lin(&[(F::from(2u64), xy), (-F::ONE, x[i].clone()), (-F::ONE, y[i].clone())], F::ONE)?;
        eq = a.mul(&eq, &e)?;
    }
    Ok((eq, rot))
}

/// The stacked reduction; returns `u` (length `n_stack + 1`) and the stacking openings.
#[allow(clippy::type_complexity)]
pub fn part3<A: Alg>(
    a: &mut A,
    t: &mut Transcript<A::V>,
    s: &Shape,
    ss: &StackShape,
    pv: &Value<&StackData>,
    rs: &[A::V],
    op: &Openings<A::V>,
) -> Result<(Vec<A::V>, Vec<Vec<A::V>>), Error> {
    let l_skip = s.l_skip;
    let wit = |a: &mut A, f: &dyn Fn(&StackData) -> F| a.wit(pv.map(f));

    // need_rot per commitment and matrix.
    let mut need_rot: Vec<Vec<bool>> = vec![s.trace_to_air.iter().map(|&i| s.airs[i].need_rot).collect()];
    for (ti, &air) in s.trace_to_air.iter().enumerate() {
        for _ in 1..op[ti].len() {
            need_rot.push(vec![s.airs[air].need_rot]);
        }
    }
    assert_eq!(need_rot.len(), ss.layouts.len());

    // t_claims: common main of every trace, then the other parts per trace.
    let mut tc: Vec<(A::V, A::V)> = vec![];
    for parts in op {
        tc.extend(parts[0].iter().cloned());
    }
    for parts in op {
        for p in &parts[1..] {
            tc.extend(p.iter().cloned());
        }
    }
    let n_claims: usize = ss.layouts.iter().map(|l| l.len()).sum();
    assert_eq!(tc.len(), n_claims, "t_claims length");

    let lambda = t.sample(a)?;
    let l2 = a.mul(&lambda, &lambda)?;
    let lp = powers(a, &l2, n_claims)?;
    let mut s0 = a.cst(F::ZERO)?;
    for ((c, r), p) in tc.iter().zip(&lp) {
        let x = a.mul(r, &lambda)?;
        let x = a.add(c, &x)?;
        let x = a.mul(&x, p)?;
        s0 = a.add(&s0, &x)?;
    }
    let coeffs: Vec<A::V> =
        (0..ss.n_uni).map(|i| wit(a, &|p: &StackData| p.univariate_round_coeffs[i])).collect::<Result<_, _>>()?;
    let d = F::from(1u64 << l_skip);
    let terms: Vec<(F, A::V)> = coeffs.iter().step_by(1 << l_skip).map(|c| (d, c.clone())).collect();
    let dom = a.lin(&terms, F::ZERO)?;
    a.assert_eq(&s0, &dom, "stacked s_0")?;
    t.observe_all(a, &coeffs)?;
    let mut u = vec![t.sample(a)?];
    let mut claim = horner(a, &coeffs, &u[0])?;
    for j in 1..=ss.n_stack {
        let e1 = wit(a, &|p: &StackData| p.sumcheck_round_polys[j - 1][0])?;
        let e2 = wit(a, &|p: &StackData| p.sumcheck_round_polys[j - 1][1])?;
        t.observe(a, &e1)?;
        t.observe(a, &e2)?;
        let uj = t.sample(a)?;
        let e0 = a.sub(&claim, &e1)?;
        claim = interpolate_quadratic_at_012(a, &[e0, e1, e2], &uj)?;
        u.push(uj);
    }

    // Per distinct n: (eq_prism, rot_kernel_prism or None, ind). Depends on n and need_rot only.
    let mut cache: HashMap<(isize, bool), (A::V, Option<A::V>, A::V)> = HashMap::new();
    let mut prism = |a: &mut A, n: isize, rot: bool| -> Result<(A::V, Option<A::V>, A::V), Error> {
        if let Some(v) = cache.get(&(n, rot)) {
            return Ok(v.clone());
        }
        let n_lift = n.max(0) as usize;
        let (l, rs_n) = if n < 0 {
            let k = (-n) as usize;
            let pw = exp_powers_of_2(a, &rs[0], k + 1)?;
            (l_skip - k, vec![pw[k].clone()])
        } else {
            (l_skip, rs[..=n_lift].to_vec())
        };
        let ux = &u[..=n_lift];
        let e0 = eq_uni(a, l, &ux[0], &rs_n[0])?;
        let em = eval_eq_mle(a, &ux[1..], &rs_n[1..])?;
        let eq_prism = a.mul(&e0, &em)?;
        let rk = if rot {
            let w = two_adic_generator(l);
            let yw = a.mul_c(&rs_n[0], w)?;
            let (eqc, rotc) = eq_rot_cube(a, &ux[1..], &rs_n[1..])?;
            let x = eq_uni(a, l, &ux[0], &yw)?;
            let x = a.mul(&x, &eqc)?;
            let p = eq_uni_at_one(a, l, &ux[0])?;
            let q = eq_uni_at_one(a, l, &yw)?;
            let pq = a.mul(&p, &q)?;
            let dlt = a.sub(&rotc, &eqc)?;
            let y = a.mul(&pq, &dlt)?;
            Some(a.add(&x, &y)?)
        } else {
            None
        };
        let ind = if n < 0 {
            let k = (-n) as usize;
            let pw = exp_powers_of_2(a, &u[0], l_skip - k + 1)?;
            eq_uni_at_one(a, k, &pw[l_skip - k])?
        } else {
            a.cst(F::ONE)?
        };
        let v = (eq_prism, rk, ind);
        cache.insert((n, rot), v.clone());
        Ok(v)
    };

    // Observe the stacking openings (commit order), then sum q_j * coefficient.
    let mut q: Vec<Vec<A::V>> = vec![];
    for (ci, w) in ss.widths.iter().enumerate() {
        let mut v = vec![];
        for k in 0..*w {
            let x = wit(a, &|p: &StackData| p.stacking_openings[ci][k])?;
            t.observe(a, &x)?;
            v.push(x);
        }
        q.push(v);
    }
    let mut final_sum = a.cst(F::ZERO)?;
    let mut li = 0usize;
    for (ci, layout) in ss.layouts.iter().enumerate() {
        for sl in layout {
            let rot = need_rot[ci][sl.mat];
            let n = sl.log_height as isize - l_skip as isize;
            let n_lift = n.max(0) as usize;
            let bits: Vec<bool> = (l_skip + n_lift..l_skip + ss.n_stack).map(|j| (sl.row_idx >> j) & 1 == 1).collect();
            let eqb = eval_eq_mle_bits(a, &u[n_lift + 1..], &bits)?;
            let (ep, rk, ind) = prism(a, n, rot)?;
            let mut batched = ep;
            if let Some(rk) = rk {
                let x = a.mul(&lambda, &rk)?;
                batched = a.add(&batched, &x)?;
            }
            let x = a.mul(&batched, &lp[li])?;
            let x = a.mul(&x, &eqb)?;
            let x = a.mul(&x, &ind)?;
            let x = a.mul(&x, &q[ci][sl.col_idx])?;
            final_sum = a.add(&final_sum, &x)?;
            li += 1;
        }
    }
    a.assert_eq(&claim, &final_sum, "stacked final sum")?;
    Ok((u, q))
}
