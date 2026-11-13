//! I5: WHIR (`verifier/whir.rs` of stark-backend v2.1.0) over `Alg`, `EF = F`. The round
//! structure (k, rounds, queries, PoW bits, domain sizes) is pinned from the key; the proof
//! values are witnesses. Query positions come from the transcript sample's canonical low bits:
//! the Merkle directions and `omega^index` are both taken from those bit cells.

use ff::Field;
use midnight_proofs::{circuit::Value, plonk::Error};

use crate::{
    alg::Alg,
    merkle::{hash_slice, merkle_verify, tree_compress},
    native::{F, two_adic_generator},
    poly::{eval_eq_mle, exp_powers_of_2, horner, interpolate_quadratic_at_012, powers},
    transcript::Transcript,
    wrapin::{WrapIn, digest, fr, frs, us},
};

/// The pinned WHIR shape.
#[derive(Clone, Debug)]
pub struct WhirShape {
    pub k: usize,
    pub log_blowup: usize,
    pub m: usize,
    pub num_queries: Vec<usize>,
    pub n_sumcheck: usize,
    pub log_final_poly_len: usize,
    pub mu_pow_bits: usize,
    pub folding_pow_bits: usize,
    pub query_pow_bits: usize,
    pub widths: Vec<usize>,
    pub u_leaf: u128,
    pub u_node: u128,
}

impl WhirShape {
    pub fn from_wrapin(w: &WrapIn, widths: &[usize]) -> Self {
        let p = &w.doc["vk"]["inner"]["params"];
        let wh = &p["whir"];
        WhirShape {
            k: us(&wh["k"]),
            log_blowup: us(&p["log_blowup"]),
            m: us(&p["l_skip"]) + us(&p["n_stack"]),
            num_queries: wh["rounds"].as_array().unwrap().iter().map(|r| us(&r["num_queries"])).collect(),
            n_sumcheck: w.derived("num_whir_sumcheck_rounds"),
            log_final_poly_len: w.derived("log_final_poly_len"),
            mu_pow_bits: us(&wh["mu_pow_bits"]),
            folding_pow_bits: us(&wh["folding_pow_bits"]),
            query_pow_bits: us(&wh["query_phase_pow_bits"]),
            widths: widths.to_vec(),
            u_leaf: w.u_leaf(),
            u_node: w.u_node(),
        }
    }
}

/// The WHIR proof values.
#[derive(Clone, Debug, Default)]
pub struct WhirData {
    pub mu_pow_witness: F,
    pub sumcheck_polys: Vec<[F; 2]>,
    pub codeword_commits: Vec<F>,
    pub ood_values: Vec<F>,
    /// Per commitment, per query, per row (2^k), the row values.
    pub initial_rows: Vec<Vec<Vec<Vec<F>>>>,
    /// Per commitment, per query, siblings.
    pub initial_proofs: Vec<Vec<Vec<F>>>,
    /// Per round >= 1, per query, 2^k values.
    pub codeword_values: Vec<Vec<Vec<F>>>,
    pub codeword_proofs: Vec<Vec<Vec<F>>>,
    pub folding_pow: Vec<F>,
    pub query_pow: Vec<F>,
    pub final_poly: Vec<F>,
}

fn digests(v: &serde_json::Value) -> Vec<F> {
    v.as_array().unwrap().iter().map(digest).collect()
}

impl WhirData {
    pub fn from_wrapin(w: &WrapIn) -> Self {
        let p = &w.proof()["whir_proof"];
        let arr = |k: &str| p[k].as_array().unwrap().clone();
        WhirData {
            mu_pow_witness: fr(&p["mu_pow_witness"]),
            sumcheck_polys: arr("whir_sumcheck_polys").iter().map(|e| [fr(&e[0]), fr(&e[1])]).collect(),
            codeword_commits: digests(&p["codeword_commits"]),
            ood_values: frs(&p["ood_values"]),
            initial_rows: arr("initial_round_opened_rows")
                .iter()
                .map(|c| c.as_array().unwrap().iter().map(|q| q.as_array().unwrap().iter().map(frs).collect()).collect())
                .collect(),
            initial_proofs: arr("initial_round_merkle_proofs")
                .iter()
                .map(|c| c.as_array().unwrap().iter().map(digests).collect())
                .collect(),
            codeword_values: arr("codeword_opened_values")
                .iter()
                .map(|r| r.as_array().unwrap().iter().map(frs).collect())
                .collect(),
            codeword_proofs: arr("codeword_merkle_proofs")
                .iter()
                .map(|r| r.as_array().unwrap().iter().map(digests).collect())
                .collect(),
            folding_pow: frs(&p["folding_pow_witnesses"]),
            query_pow: frs(&p["query_phase_pow_witnesses"]),
            final_poly: frs(&p["final_poly"]),
        }
    }
}

/// `prod_j (b_j ? c^(2^j) : 1)` for constant `c`: `c^index` from the index bits.
fn pow_from_bits<A: Alg>(a: &mut A, bits: &[A::B], c: F, one: &A::V) -> Result<A::V, Error> {
    let mut acc = one.clone();
    let mut cp = c;
    for b in bits {
        let k = a.cst(cp)?;
        let f = a.select(b, &k, one)?;
        acc = a.mul(&acc, &f)?;
        cp = cp.square();
    }
    Ok(acc)
}

/// `binary_k_fold` with `x = zi_root` and `x^-1 = zi_inv` given.
fn k_fold<A: Alg>(a: &mut A, mut v: Vec<A::V>, alphas: &[A::V], x: &A::V, x_inv: &A::V) -> Result<A::V, Error> {
    let k = alphas.len();
    let n = v.len();
    assert_eq!(n, 1 << k);
    let wk = two_adic_generator(k);
    let wk_inv = wk.invert().unwrap();
    let half = F::from(2u64).invert().unwrap();
    let xp = exp_powers_of_2(a, x, k.max(1))?;
    let xip = exp_powers_of_2(a, x_inv, k.max(1))?;
    for (j, alpha) in alphas.iter().enumerate() {
        let m = n >> (j + 1);
        for i in 0..m {
            let tw = wk.pow_vartime([(i << j) as u64]);
            let itw = wk_inv.pow_vartime([(i << j) as u64]);
            let t = a.mul_c(&xp[j], tw)?;
            let at = a.sub(alpha, &t)?;
            let d = a.sub(&v[i], &v[i + m])?;
            let ti = a.mul_c(&xip[j], itw * half)?;
            let p = a.mul(&at, &d)?;
            v[i] = a.mul_add(&p, &ti, &v[i])?;
        }
    }
    Ok(v[0].clone())
}

/// WHIR. `q` are the stacking openings (observed in `part3`), `commits` the round-0 roots in
/// commitment order, `u` the stacked-reduction point (length `n_stack + 1`), `l_skip`.
#[allow(clippy::too_many_arguments)]
pub fn part4<A: Alg>(
    a: &mut A,
    t: &mut Transcript<A::V>,
    ws: &WhirShape,
    pv: &Value<&WhirData>,
    q: &[Vec<A::V>],
    commits: &[A::V],
    u: &[A::V],
    l_skip: usize,
) -> Result<(), Error> {
    let wit = |a: &mut A, f: &dyn Fn(&WhirData) -> F| a.wit(pv.map(f));
    let one = a.cst(F::ONE)?;
    let k = ws.k;
    let rounds = ws.num_queries.len();
    let w_mu = wit(a, &|p: &WhirData| p.mu_pow_witness)?;
    t.check_witness(a, ws.mu_pow_bits, &w_mu)?;
    let mu = t.sample(a)?;
    let total: usize = ws.widths.iter().sum();
    let mu_pows = powers(a, &mu, total)?;
    let mut claim = a.cst(F::ZERO)?;
    for (x, m) in q.iter().flatten().zip(&mu_pows) {
        claim = a.mul_add(x, m, &claim)?;
    }
    let mut log_rs = ws.m + ws.log_blowup;
    let mut sc = 0usize;
    let mut alphas: Vec<A::V> = vec![];
    let mut gammas = vec![];
    let mut z0s = vec![];
    let mut zs: Vec<Vec<A::V>> = vec![];
    let mut final_poly = vec![];
    let mut cw_roots: Vec<A::V> = vec![];
    for r in 0..rounds {
        let mut ar = vec![];
        for _ in 0..k {
            if sc < ws.n_sumcheck {
                let i = sc;
                let e1 = wit(a, &|p: &WhirData| p.sumcheck_polys[i][0])?;
                let e2 = wit(a, &|p: &WhirData| p.sumcheck_polys[i][1])?;
                t.observe(a, &e1)?;
                t.observe(a, &e2)?;
                let fw = wit(a, &|p: &WhirData| p.folding_pow[i])?;
                t.check_witness(a, ws.folding_pow_bits, &fw)?;
                let al = t.sample(a)?;
                let e0 = a.sub(&claim, &e1)?;
                claim = interpolate_quadratic_at_012(a, &[e0, e1, e2], &al)?;
                ar.push(al);
                sc += 1;
            }
        }
        let is_final = r == rounds - 1;
        let y0 = if is_final {
            for i in 0..1usize << ws.log_final_poly_len {
                let c = wit(a, &|p: &WhirData| p.final_poly[i])?;
                t.observe(a, &c)?;
                final_poly.push(c);
            }
            None
        } else {
            let c = wit(a, &|p: &WhirData| p.codeword_commits[r])?;
            t.observe(a, &c)?;
            cw_roots.push(c);
            z0s.push(t.sample(a)?);
            let y0 = wit(a, &|p: &WhirData| p.ood_values[r])?;
            t.observe(a, &y0)?;
            Some(y0)
        };
        let qw = wit(a, &|p: &WhirData| p.query_pow[r])?;
        t.check_witness(a, ws.query_pow_bits, &qw)?;
        let nbits = log_rs - k;
        let omega = two_adic_generator(log_rs);
        let omega_inv = omega.invert().unwrap();
        let mut ys = vec![];
        let mut zr = vec![];
        for qi in 0..ws.num_queries[r] {
            let (_, bits) = t.sample_bits(a, nbits)?;
            let zi_root = pow_from_bits(a, &bits, omega, &one)?;
            let zi_inv = pow_from_bits(a, &bits, omega_inv, &one)?;
            let zi = pow_from_bits(a, &bits, omega.pow_vartime([1u64 << k]), &one)?;
            let yi = if r == 0 {
                let mut vals = vec![a.cst(F::ZERO)?; 1 << k];
                let mut mi = 0usize;
                for (ci, &wd) in ws.widths.iter().enumerate() {
                    let mut rows = vec![];
                    let mut leaves = vec![];
                    for j in 0..1usize << k {
                        let row: Vec<A::V> = (0..wd)
                            .map(|c| a.wit(pv.map(|p| p.initial_rows[ci][qi][j][c])))
                            .collect::<Result<_, _>>()?;
                        leaves.push(hash_slice(a, ws.u_leaf, &row)?);
                        rows.push(row);
                    }
                    let dg = tree_compress(a, ws.u_node, leaves)?;
                    let sib: Vec<A::V> = (0..nbits)
                        .map(|s| a.wit(pv.map(|p| p.initial_proofs[ci][qi][s])))
                        .collect::<Result<_, _>>()?;
                    merkle_verify(a, ws.u_node, &commits[ci], &bits, &dg, &sib)?;
                    for c in 0..wd {
                        for j in 0..1usize << k {
                            vals[j] = a.mul_add(&mu_pows[mi], &rows[j][c], &vals[j])?;
                        }
                        mi += 1;
                    }
                }
                k_fold(a, vals, &ar, &zi_root, &zi_inv)?
            } else {
                let vals: Vec<A::V> = (0..1usize << k)
                    .map(|j| a.wit(pv.map(|p| p.codeword_values[r - 1][qi][j])))
                    .collect::<Result<_, _>>()?;
                let leaves: Vec<A::V> =
                    vals.iter().map(|v| hash_slice(a, ws.u_leaf, std::slice::from_ref(v))).collect::<Result<_, _>>()?;
                let dg = tree_compress(a, ws.u_node, leaves)?;
                let sib: Vec<A::V> = (0..nbits)
                    .map(|s| a.wit(pv.map(|p| p.codeword_proofs[r - 1][qi][s])))
                    .collect::<Result<_, _>>()?;
                // the root is the commitment cell observed in round r - 1
                merkle_verify(a, ws.u_node, &cw_roots[r - 1], &bits, &dg, &sib)?;
                k_fold(a, vals, &ar, &zi_root, &zi_inv)?
            };
            ys.push(yi);
            zr.push(zi);
        }
        let gamma = t.sample(a)?;
        let gp = powers(a, &gamma, ws.num_queries[r] + 2)?;
        if let Some(y0) = y0 {
            claim = a.mul_add(&y0, &gamma, &claim)?;
        }
        for (yi, g) in ys.iter().zip(&gp[2..]) {
            claim = a.mul_add(yi, g, &claim)?;
        }
        gammas.push(gp);
        zs.push(zr);
        alphas.extend(ar);
        log_rs -= 1;
    }

    // Final check.
    let mut u_cube = exp_powers_of_2(a, &u[0], l_skip.max(1))?;
    u_cube.truncate(l_skip);
    u_cube.extend(u[1..].iter().cloned());
    let tt = k * rounds;
    let mut prefix = one.clone();
    for (ui, ai) in u_cube[..tt].iter().zip(&alphas[..tt]) {
        // (1 - 2u)(1 - a) + u a = 1 - 2u - a + 3ua
        let ua = a.mul(ui, ai)?;
        let f = a.lin(&[(-F::from(2u64), ui.clone()), (-F::ONE, ai.clone()), (F::from(3u64), ua)], F::ONE)?;
        prefix = a.mul(&prefix, &f)?;
    }
    let mut ev = final_poly.clone();
    let mut len = ev.len();
    for xj in u_cube[tt..].iter().rev() {
        len >>= 1;
        for i in 0..len {
            let d = a.sub(&ev[i + len], &ev[i])?;
            ev[i] = a.mul_add(xj, &d, &ev[i])?;
        }
    }
    assert_eq!(len, 1, "final poly length");
    let mut acc = a.mul(&prefix, &ev[0])?;
    let mut j = k;
    for i in 0..rounds {
        let slc = &alphas[j..tt];
        let n = tt - j + 1;
        if i != rounds - 1 {
            let zp = exp_powers_of_2(a, &z0s[i], n)?;
            let e = eval_eq_mle(a, slc, &zp[..n - 1])?;
            let h = horner(a, &final_poly, &zp[n - 1])?;
            let x = a.mul(&gammas[i][1], &e)?;
            acc = a.mul_add(&x, &h, &acc)?;
        }
        for (zi, g) in zs[i].iter().zip(&gammas[i][2..]) {
            let zp = exp_powers_of_2(a, zi, n)?;
            let e = eval_eq_mle(a, slc, &zp[..n - 1])?;
            let h = horner(a, &final_poly, &zp[n - 1])?;
            let x = a.mul(g, &e)?;
            acc = a.mul_add(&x, &h, &acc)?;
        }
        j += k;
    }
    a.assert_eq(&acc, &claim, "WHIR final")
}
