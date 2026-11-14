//! The `L_Fr` verifier over `Alg`, following stark-backend v2.1.0 `verifier/*.rs` with
//! `EF = F`. The shape (params, presence, heights, trace order, per-AIR data) is pinned from
//! the key; the proof values are witnesses.
//!
//! I3: preamble (`verifier/mod.rs`), LogUp PoW, LogUp GKR (`fractional_sumcheck_gkr.rs`) and
//! the zerocheck rounds of `batch_constraints.rs` steps 1 to 6.
//! I4: `batch_constraints.rs` steps 7 to 9 (opening observation, DAG evaluation, final claim).

use ff::Field;
use midnight_proofs::{circuit::Value, plonk::Error};

use crate::{
    alg::Alg,
    native::F,
    native::two_adic_generator,
    poly::{eval_eq_mle, eval_eq_mle_bits, exp_powers_of_2, horner, interpolate_cubic_at_0123, interpolate_linear_at_01},
    transcript::Transcript,
    wrapin::{WrapIn, digest, fr, frs, us},
};

/// A node of the symbolic constraint DAG (stark-backend `SymbolicExpressionNode`).
#[derive(Clone, Debug)]
pub enum Node {
    /// part: `None` preprocessed, `Some(p)` main part `p`; `offset` 0 or 1.
    Var { part: Option<usize>, index: usize, offset: usize },
    Public(usize),
    First,
    Last,
    Transition,
    Const(F),
    Add(usize, usize),
    Sub(usize, usize),
    Neg(usize),
    Mul(usize, usize),
}

/// One interaction: message node ids, count node id, bus index.
#[derive(Clone, Debug)]
pub struct Interaction {
    pub message: Vec<usize>,
    pub count: usize,
    pub bus: u64,
}

fn parse_node(n: &serde_json::Value) -> Node {
    if let Some(s) = n.as_str() {
        return match s {
            "IsFirstRow" => Node::First,
            "IsLastRow" => Node::Last,
            "IsTransition" => Node::Transition,
            k => panic!("node {k}"),
        };
    }
    let (k, v) = n.as_object().unwrap().iter().next().unwrap();
    match k.as_str() {
        "Variable" => {
            let index = us(&v["index"]);
            let e = &v["entry"];
            if e.as_str() == Some("Public") {
                return Node::Public(index);
            }
            if let Some(p) = e.get("Preprocessed") {
                Node::Var { part: None, index, offset: us(&p["offset"]) }
            } else if let Some(m) = e.get("Main") {
                Node::Var { part: Some(us(&m["part_index"])), index, offset: us(&m["offset"]) }
            } else {
                panic!("entry {e}")
            }
        }
        "Constant" => Node::Const(fr(v)),
        "Add" => Node::Add(us(&v["left_idx"]), us(&v["right_idx"])),
        "Sub" => Node::Sub(us(&v["left_idx"]), us(&v["right_idx"])),
        "Mul" => Node::Mul(us(&v["left_idx"]), us(&v["right_idx"])),
        "Neg" => Node::Neg(us(&v["idx"])),
        k => panic!("node {k}"),
    }
}

/// Per-AIR key data.
#[derive(Clone, Debug)]
pub struct AirShape {
    pub is_required: bool,
    pub preprocessed_commit: Option<F>,
    pub need_rot: bool,
    pub num_public_values: usize,
    pub num_interactions: usize,
    /// `Some(log_height)` when present (the shape pin).
    pub log_height: Option<usize>,
    pub num_cached: usize,
    pub nodes: Vec<Node>,
    pub constraint_idx: Vec<usize>,
    pub interactions: Vec<Interaction>,
}

/// The pinned shape: key constants of the wrap.
#[derive(Clone, Debug)]
pub struct Shape {
    pub l_skip: usize,
    pub n_stack: usize,
    pub logup_pow_bits: usize,
    pub max_constraint_degree: usize,
    pub vk_max_constraint_degree: usize,
    pub pre_hash: F,
    pub airs: Vec<AirShape>,
    /// Present AIRs sorted by (descending log height, AIR id).
    pub trace_to_air: Vec<usize>,
    pub n_per_trace: Vec<isize>,
    /// Per present trace, per part: number of opened values (key widths, times 2 with rotation).
    pub opening_lens: Vec<Vec<usize>>,
    pub u_transcript: u128,
}

impl Shape {
    pub fn from_wrapin(w: &WrapIn) -> Self {
        let inner = &w.doc["vk"]["inner"];
        let p = &inner["params"];
        let l_skip = us(&p["l_skip"]);
        let tv = w.proof()["trace_vdata"].as_array().unwrap();
        let mut airs = vec![];
        let mut vk_deg = 0;
        for (i, a) in inner["per_air"].as_array().unwrap().iter().enumerate() {
            vk_deg = vk_deg.max(us(&a["max_constraint_degree"]));
            let t = &tv[i];
            // T3: as the stock proof-shape check (RequiredAirNoVData), a required AIR is present.
            assert!(
                !(a["is_required"].as_bool().unwrap() && t.is_null()),
                "wrap shape: required AIR {i} has no trace"
            );
            airs.push(AirShape {
                is_required: a["is_required"].as_bool().unwrap(),
                preprocessed_commit: (!a["preprocessed_data"].is_null())
                    .then(|| digest(&a["preprocessed_data"]["commit"])),
                need_rot: a["params"]["need_rot"].as_bool().unwrap(),
                num_public_values: us(&a["params"]["num_public_values"]),
                num_interactions: a["symbolic_constraints"]["interactions"].as_array().unwrap().len(),
                log_height: (!t.is_null()).then(|| us(&t["log_height"])),
                num_cached: if t.is_null() { 0 } else { t["cached_commitments"].as_array().unwrap().len() },
                nodes: a["symbolic_constraints"]["constraints"]["nodes"].as_array().unwrap().iter().map(parse_node).collect(),
                constraint_idx: a["symbolic_constraints"]["constraints"]["constraint_idx"].as_array().unwrap().iter().map(us).collect(),
                interactions: a["symbolic_constraints"]["interactions"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|i| Interaction {
                        message: i["message"].as_array().unwrap().iter().map(us).collect(),
                        count: us(&i["count"]),
                        bus: i["bus_index"].as_u64().unwrap(),
                    })
                    .collect(),
            });
        }
        let mut order: Vec<usize> = (0..airs.len()).filter(|&i| airs[i].log_height.is_some()).collect();
        order.sort_by_key(|&i| (std::cmp::Reverse(airs[i].log_height.unwrap()), i));
        let n_per_trace = order.iter().map(|&i| airs[i].log_height.unwrap() as isize - l_skip as isize).collect();
        Shape {
            l_skip,
            n_stack: us(&p["n_stack"]),
            logup_pow_bits: us(&p["logup"]["pow_bits"]),
            max_constraint_degree: us(&p["max_constraint_degree"]),
            vk_max_constraint_degree: vk_deg,
            pre_hash: digest(&w.doc["vk"]["pre_hash"]),
            airs,
            trace_to_air: order.clone(),
            opening_lens: order
                .iter()
                .map(|&i| {
                    let a = &inner["per_air"][i];
                    let wd = &a["params"]["width"];
                    let m = if a["params"]["need_rot"].as_bool().unwrap() { 2 } else { 1 };
                    let mut v = vec![m * us(&wd["common_main"])];
                    if !wd["preprocessed"].is_null() {
                        v.push(m * us(&wd["preprocessed"]));
                    }
                    v.extend(wd["cached_mains"].as_array().unwrap().iter().map(|c| m * us(c)));
                    v
                })
                .collect(),
            n_per_trace,
            u_transcript: w.u_transcript(),
        }
    }

    pub fn n_logup(&self) -> usize {
        let total: u64 = self
            .trace_to_air
            .iter()
            .zip(&self.n_per_trace)
            .map(|(&i, &n)| (self.airs[i].num_interactions as u64) << (self.l_skip + n.max(0) as usize))
            .sum();
        if total == 0 { 0 } else { (u64::BITS - total.leading_zeros()) as usize - self.l_skip }
    }

    pub fn has_interactions(&self) -> bool {
        self.trace_to_air.iter().any(|&i| self.airs[i].num_interactions > 0)
    }
}

/// The proof values used by I3 (plain data; wrapped in `Value` as the witness).
#[derive(Clone, Debug, Default)]
pub struct ProofData {
    pub common_main_commit: F,
    /// Per AIR (vk order): cached commitments and public values.
    pub cached: Vec<Vec<F>>,
    pub public_values: Vec<Vec<F>>,
    pub logup_pow_witness: F,
    pub q0_claim: F,
    /// Per layer: `[p_xi_0, q_xi_0, p_xi_1, q_xi_1]` (observe order).
    pub layer_claims: Vec<[F; 4]>,
    pub gkr_sumcheck: Vec<Vec<[F; 3]>>,
    pub numerator_term_per_air: Vec<F>,
    pub denominator_term_per_air: Vec<F>,
    pub univariate_round_coeffs: Vec<F>,
    pub sumcheck_round_polys: Vec<Vec<F>>,
    /// Per present trace (sorted order), per part (common main, preprocessed, cached), values.
    pub column_openings: Vec<Vec<Vec<F>>>,
    /// The stacking proof (I4b).
    pub stack: crate::stacked::StackData,
    /// The WHIR proof (I5).
    pub whir: crate::whir::WhirData,
    /// The wrap-private `PATH` siblings of the decoded log, one list per `PATH` entry in log
    /// order (T9; `wrapin` field `path_sibs`, hex Fr; empty when the log has no `PATH`).
    pub path_sibs: Vec<Vec<F>>,
}

impl ProofData {
    pub fn from_wrapin(w: &WrapIn) -> Self {
        let p = w.proof();
        let g = &p["gkr_proof"];
        let b = &p["batch_constraint_proof"];
        let tv = p["trace_vdata"].as_array().unwrap();
        ProofData {
            common_main_commit: digest(&p["common_main_commit"]),
            cached: tv
                .iter()
                .map(|t| if t.is_null() { vec![] } else { t["cached_commitments"].as_array().unwrap().iter().map(digest).collect() })
                .collect(),
            public_values: p["public_values"].as_array().unwrap().iter().map(frs).collect(),
            logup_pow_witness: fr(&g["logup_pow_witness"]),
            q0_claim: fr(&g["q0_claim"]),
            layer_claims: g["claims_per_layer"]
                .as_array()
                .unwrap()
                .iter()
                .map(|c| [fr(&c["p_xi_0"]), fr(&c["q_xi_0"]), fr(&c["p_xi_1"]), fr(&c["q_xi_1"])])
                .collect(),
            gkr_sumcheck: g["sumcheck_polys"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| r.as_array().unwrap().iter().map(|e| {
                    let v = frs(e);
                    [v[0], v[1], v[2]]
                }).collect())
                .collect(),
            numerator_term_per_air: frs(&b["numerator_term_per_air"]),
            denominator_term_per_air: frs(&b["denominator_term_per_air"]),
            univariate_round_coeffs: frs(&b["univariate_round_coeffs"]),
            sumcheck_round_polys: b["sumcheck_round_polys"].as_array().unwrap().iter().map(frs).collect(),
            column_openings: b["column_openings"]
                .as_array()
                .unwrap()
                .iter()
                .map(|t| t.as_array().unwrap().iter().map(frs).collect())
                .collect(),
            stack: crate::stacked::StackData::from_wrapin(w),
            whir: crate::whir::WhirData::from_wrapin(w),
            path_sibs: w
                .doc
                .get("path_sibs")
                .and_then(|v| v.as_array())
                .map(|ls| {
                    ls.iter()
                        .map(|l| l.as_array().unwrap().iter().map(|h| crate::native::f_from_hex(h.as_str().unwrap()).unwrap()).collect())
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// The state after the zerocheck rounds (inputs of I4).
#[derive(Clone, Debug)]
pub struct Part1<V> {
    pub alpha: V,
    pub beta: V,
    pub xi: Vec<V>,
    pub lambda: V,
    pub mu: V,
    pub rs: Vec<V>,
    pub cur_sum: V,
    pub public_values: Vec<Vec<V>>,
    pub numerator_term_per_air: Vec<V>,
    pub denominator_term_per_air: Vec<V>,
    /// The observed commitment cells: common main, and per AIR (vk order) the cached ones.
    pub common_main_commit: V,
    pub cached: Vec<Vec<V>>,
}

fn wits<A: Alg>(a: &mut A, pv: &Value<&ProofData>, n: usize, f: impl Fn(&ProofData, usize) -> F) -> Result<Vec<A::V>, Error> {
    (0..n).map(|i| a.wit(pv.map(|p| f(p, i)))).collect()
}

/// Preamble, LogUp PoW, GKR and the zerocheck rounds.
pub fn part1<A: Alg>(
    a: &mut A,
    t: &mut Transcript<A::V>,
    s: &Shape,
    pv: &Value<&ProofData>,
) -> Result<Part1<A::V>, Error> {
    let one = F::ONE;
    // Preamble (verifier/mod.rs).
    let pre = a.cst(s.pre_hash)?;
    t.observe(a, &pre)?;
    let cmc = a.wit(pv.map(|p| p.common_main_commit))?;
    t.observe(a, &cmc)?;
    let mut pvals = vec![];
    let mut cached_cells = vec![];
    for (i, air) in s.airs.iter().enumerate() {
        if !air.is_required {
            let b = a.cst(if air.log_height.is_some() { one } else { F::ZERO })?;
            t.observe(a, &b)?;
        }
        let mut vals = vec![];
        if let Some(h) = air.log_height {
            match air.preprocessed_commit {
                Some(c) => {
                    let c = a.cst(c)?;
                    t.observe(a, &c)?;
                }
                None => {
                    let h = a.cst(F::from(h as u64))?;
                    t.observe(a, &h)?;
                }
            }
            let cached = wits(a, pv, air.num_cached, |p, j| p.cached[i][j])?;
            t.observe_all(a, &cached)?;
            cached_cells.push(cached);
            vals = wits(a, pv, air.num_public_values, |p, j| p.public_values[i][j])?;
            t.observe_all(a, &vals)?;
        } else {
            cached_cells.push(vec![]);
        }
        pvals.push(vals);
    }

    // batch_constraints.rs step 1 and 2.
    let w = a.wit(pv.map(|p| p.logup_pow_witness))?;
    t.check_witness(a, s.logup_pow_bits, &w)?;
    let alpha = t.sample(a)?;
    let beta = t.sample(a)?;
    let n_logup = s.n_logup();
    let l_skip = s.l_skip;
    let (mut p_claim, mut q_claim, mut xi) = if s.has_interactions() {
        gkr(a, t, pv, l_skip + n_logup)?
    } else {
        let q0 = a.wit(pv.map(|p| p.q0_claim))?;
        a.assert_eq_c(&q0, one, "q0 = 1 without interactions")?;
        (a.cst(F::ZERO)?, alpha.clone(), vec![])
    };
    let n_max = s.n_per_trace.iter().copied().max().unwrap().max(0) as usize;
    let n_global = n_max.max(n_logup);
    while xi.len() != l_skip + n_global {
        xi.push(t.sample(a)?);
    }
    let lambda = t.sample(a)?;

    // Step 3.
    let nt = s.trace_to_air.len();
    let num = wits(a, pv, nt, |p, i| p.numerator_term_per_air[i])?;
    let den = wits(a, pv, nt, |p, i| p.denominator_term_per_air[i])?;
    for (n, d) in num.iter().zip(&den) {
        p_claim = a.sub(&p_claim, n)?;
        q_claim = a.sub(&q_claim, d)?;
        t.observe(a, n)?;
        t.observe(a, d)?;
    }
    a.assert_eq_c(&p_claim, F::ZERO, "GKR numerator claim")?;
    a.assert_eq(&q_claim, &alpha, "GKR denominator claim")?;

    // Step 4.
    let mu = t.sample(a)?;
    let mut sum_claim = a.cst(F::ZERO)?;
    let mut mu_pow = a.cst(one)?;
    for (n, d) in num.iter().zip(&den) {
        let x = a.mul(n, &mu_pow)?;
        sum_claim = a.add(&sum_claim, &x)?;
        mu_pow = a.mul(&mu_pow, &mu)?;
        let x = a.mul(d, &mu_pow)?;
        sum_claim = a.add(&sum_claim, &x)?;
        mu_pow = a.mul(&mu_pow, &mu)?;
    }

    // Step 5: univariate round.
    let nuni = (s.max_constraint_degree + 1) * ((1 << l_skip) - 1) + 1;
    let coeffs = wits(a, pv, nuni, |p, i| p.univariate_round_coeffs[i])?;
    t.observe_all(a, &coeffs)?;
    let r0 = t.sample(a)?;
    let d = F::from(1u64 << l_skip);
    let terms: Vec<(F, A::V)> = coeffs.iter().step_by(1 << l_skip).map(|c| (d, c.clone())).collect();
    let dom_sum = a.lin(&terms, F::ZERO)?;
    a.assert_eq(&sum_claim, &dom_sum, "zerocheck univariate sum")?;
    let mut cur = horner(a, &coeffs, &r0)?;
    let mut rs = vec![r0];

    // Step 6: multilinear rounds, Lagrange at 0..s_deg with constant inverse factorials.
    let s_deg = s.max_constraint_degree + 1;
    let mut fact = vec![one; s_deg + 1];
    for i in 1..=s_deg {
        fact[i] = fact[i - 1] * F::from(i as u64);
    }
    let invfact: Vec<F> = fact.iter().map(|f| f.invert().unwrap()).collect();
    for round in 0..n_max {
        let ev = wits(a, pv, s_deg, |p, i| p.sumcheck_round_polys[round][i])?;
        t.observe_all(a, &ev)?;
        let s0 = a.sub(&cur, &ev[0])?;
        let r = t.sample(a)?;
        let mut evals = vec![s0];
        evals.extend(ev);
        let mut pref = vec![a.cst(one)?];
        let mut suf = vec![a.cst(one)?];
        for i in 0..s_deg {
            let x = a.add_c(&r, -F::from(i as u64))?;
            pref.push(a.mul(&pref[i], &x)?);
            let y = a.lin(&[(-one, r.clone())], F::from((s_deg - i) as u64))?;
            suf.push(a.mul(&suf[i], &y)?);
        }
        let mut acc = a.cst(F::ZERO)?;
        for i in 0..=s_deg {
            let ps = a.mul(&pref[i], &suf[s_deg - i])?;
            let ps = a.mul_c(&ps, invfact[i] * invfact[s_deg - i])?;
            let x = a.mul(&evals[i], &ps)?;
            acc = a.add(&acc, &x)?;
        }
        cur = acc;
        rs.push(r);
    }
    Ok(Part1 {
        alpha,
        beta,
        xi,
        lambda,
        mu,
        rs,
        cur_sum: cur,
        public_values: pvals,
        numerator_term_per_air: num,
        denominator_term_per_air: den,
        common_main_commit: cmc,
        cached: cached_cells,
    })
}

/// The round-0 WHIR roots in commitment order: common main, then per trace (sorted order)
/// the preprocessed commitment (a key constant) and the cached ones (observed cells).
pub fn commit_cells<A: Alg>(a: &mut A, s: &Shape, p1: &Part1<A::V>) -> Result<Vec<A::V>, Error> {
    let mut v = vec![p1.common_main_commit.clone()];
    for &air in &s.trace_to_air {
        if let Some(c) = s.airs[air].preprocessed_commit {
            v.push(a.cst(c)?);
        }
        v.extend(p1.cached[air].iter().cloned());
    }
    Ok(v)
}

/// LogUp GKR (`verify_gkr`): returns `(p(xi), q(xi), xi)`.
fn gkr<A: Alg>(
    a: &mut A,
    t: &mut Transcript<A::V>,
    pv: &Value<&ProofData>,
    total_rounds: usize,
) -> Result<(A::V, A::V, Vec<A::V>), Error> {
    let q0 = a.wit(pv.map(|p| p.q0_claim))?;
    t.observe(a, &q0)?;
    let claims = |a: &mut A, r: usize| -> Result<[A::V; 4], Error> {
        Ok([
            a.wit(pv.map(|p| p.layer_claims[r][0]))?,
            a.wit(pv.map(|p| p.layer_claims[r][1]))?,
            a.wit(pv.map(|p| p.layer_claims[r][2]))?,
            a.wit(pv.map(|p| p.layer_claims[r][3]))?,
        ])
    };
    // [p0, q0, p1, q1]; cross terms p0 q1 + p1 q0 and q0 q1.
    let cross = |a: &mut A, c: &[A::V; 4]| -> Result<(A::V, A::V), Error> {
        let x = a.mul(&c[0], &c[3])?;
        let y = a.mul(&c[2], &c[1])?;
        Ok((a.add(&x, &y)?, a.mul(&c[1], &c[3])?))
    };
    let c = claims(a, 0)?;
    t.observe_all(a, &c)?;
    let (pc, qc) = cross(a, &c)?;
    a.assert_eq_c(&pc, F::ZERO, "GKR root numerator")?;
    a.assert_eq(&qc, &q0, "GKR root denominator")?;
    let mu = t.sample(a)?;
    let mut numer = interpolate_linear_at_01(a, &[c[0].clone(), c[2].clone()], &mu)?;
    let mut denom = interpolate_linear_at_01(a, &[c[1].clone(), c[3].clone()], &mu)?;
    let mut gkr_r = vec![mu];
    for round in 1..total_rounds {
        let lambda = t.sample(a)?;
        let ld = a.mul(&lambda, &denom)?;
        let mut claim = a.add(&numer, &ld)?;
        let mut round_r = vec![];
        let mut eq = a.cst(F::ONE)?;
        for sub in 0..round {
            let ev: Vec<A::V> = (0..3)
                .map(|k| a.wit(pv.map(|p| p.gkr_sumcheck[round - 1][sub][k])))
                .collect::<Result<_, _>>()?;
            t.observe_all(a, &ev)?;
            let ri = t.sample(a)?;
            let e0 = a.sub(&claim, &ev[0])?;
            claim = interpolate_cubic_at_0123(a, &[e0, ev[0].clone(), ev[1].clone(), ev[2].clone()], &ri)?;
            let e = eval_eq_mle(a, &[gkr_r[sub].clone()], &[ri.clone()])?;
            eq = a.mul(&eq, &e)?;
            round_r.push(ri);
        }
        let c = claims(a, round)?;
        t.observe_all(a, &c)?;
        let (pc, qc) = cross(a, &c)?;
        let lq = a.mul(&lambda, &qc)?;
        let s = a.add(&pc, &lq)?;
        let expected = a.mul(&s, &eq)?;
        a.assert_eq(&expected, &claim, "GKR layer consistency")?;
        let mu = t.sample(a)?;
        numer = interpolate_linear_at_01(a, &[c[0].clone(), c[2].clone()], &mu)?;
        denom = interpolate_linear_at_01(a, &[c[1].clone(), c[3].clone()], &mu)?;
        gkr_r = std::iter::once(mu).chain(round_r).collect();
    }
    Ok((numer, denom, gkr_r))
}

/// Openings per present trace (sorted order), per part (common main, preprocessed, cached),
/// as `(claim, claim_rot)` pairs; `claim_rot` is the constant 0 without rotation.
pub type Openings<V> = Vec<Vec<Vec<(V, V)>>>;

/// `eq_D(x, y)` of `poly_common::eval_eq_uni`.
pub fn eq_uni<A: Alg>(a: &mut A, l: usize, x: &A::V, y: &A::V) -> Result<A::V, Error> {
    let xp = exp_powers_of_2(a, x, l.max(1))?;
    let yp = exp_powers_of_2(a, y, l.max(1))?;
    let mut res = a.cst(F::ONE)?;
    for i in 0..l {
        let s = a.add(&xp[i], &yp[i])?;
        let u = a.mul(&s, &res)?;
        let x1 = a.add_c(&xp[i], -F::ONE)?;
        let y1 = a.add_c(&yp[i], -F::ONE)?;
        let v = a.mul(&x1, &y1)?;
        res = a.add(&u, &v)?;
    }
    let half_l = F::from(2u64).invert().unwrap().pow_vartime([l as u64]);
    a.mul_c(&res, half_l)
}

/// `eq_D(x, 1)` of `poly_common::eval_eq_uni_at_one`: `prod_i (x^(2^i) + 1) / 2^l`.
pub fn eq_uni_at_one<A: Alg>(a: &mut A, l: usize, x: &A::V) -> Result<A::V, Error> {
    let xp = exp_powers_of_2(a, x, l.max(1))?;
    let mut res = a.cst(F::ONE)?;
    for x in xp.iter().take(l) {
        let t = a.add_c(x, F::ONE)?;
        res = a.mul(&res, &t)?;
    }
    let half_l = F::from(2u64).invert().unwrap().pow_vartime([l as u64]);
    a.mul_c(&res, half_l)
}

/// `eq_D(x, c)` with a constant `c`, given `x^(2^i)`.
fn eq_uni_c<A: Alg>(a: &mut A, l: usize, xp: &[A::V], c: F) -> Result<A::V, Error> {
    let mut res = a.cst(F::ONE)?;
    let mut cp = c;
    for x in xp.iter().take(l) {
        // (x + c) res + (x - 1)(c - 1)
        let u = a.mul(x, &res)?;
        res = a.lin(&[(F::ONE, u), (cp, res.clone()), (cp - F::ONE, x.clone())], -(cp - F::ONE))?;
        cp = cp.square();
    }
    let half_l = F::from(2u64).invert().unwrap().pow_vartime([l as u64]);
    a.mul_c(&res, half_l)
}

/// `1 + m + .. + m^(2^l - 1)` (`evaluator.rs::progression_exp_2`).
fn progression_exp_2<A: Alg>(a: &mut A, m: &A::V, l: usize) -> Result<A::V, Error> {
    let mut pow = m.clone();
    let mut sum = a.cst(F::ONE)?;
    for _ in 0..l {
        let t = a.add_c(&pow, F::ONE)?;
        sum = a.mul(&sum, &t)?;
        pow = a.mul(&pow, &pow)?;
    }
    Ok(sum)
}

/// `batch_constraints.rs` steps 7 to 9: observe the column openings, evaluate every AIR's
/// constraint DAG and interactions at `rs`, and check the batched claim against `cur_sum`.
pub fn part2<A: Alg>(
    a: &mut A,
    t: &mut Transcript<A::V>,
    s: &Shape,
    pv: &Value<&ProofData>,
    p1: &Part1<A::V>,
) -> Result<Openings<A::V>, Error> {
    let l_skip = s.l_skip;
    let n_logup = s.n_logup();
    let n_max = s.n_per_trace.iter().copied().max().unwrap().max(0) as usize;
    let xi = &p1.xi;
    let rs = &p1.rs;
    let zero = a.cst(F::ZERO)?;

    // Openings as witnesses (shape from the key: widths from the proof shape check).
    let mut op: Openings<A::V> = vec![];
    for (ti, &air) in s.trace_to_air.iter().enumerate() {
        let need_rot = s.airs[air].need_rot;
        let mut parts = vec![];
        for (pi, len) in s.opening_lens[ti].iter().enumerate() {
            let v = wits(a, pv, *len, |p, j| p.column_openings[ti][pi][j])?;
            parts.push(if need_rot {
                v.chunks_exact(2).map(|c| (c[0].clone(), c[1].clone())).collect()
            } else {
                v.into_iter().map(|c| (c, zero.clone())).collect()
            });
        }
        op.push(parts);
    }

    // Step 7: eq_3b per trace and interaction.
    let mut stacked_idx = 0usize;
    let mut eq_3b: Vec<Vec<A::V>> = vec![];
    for (ti, &n) in s.n_per_trace.iter().enumerate() {
        let ni = s.airs[s.trace_to_air[ti]].interactions.len();
        let n_lift = n.max(0) as usize;
        let mut v = vec![];
        for _ in 0..ni {
            let mut b = stacked_idx >> (l_skip + n_lift);
            let bits: Vec<bool> = (0..n_logup - n_lift)
                .map(|_| {
                    let x = b & 1 == 1;
                    b >>= 1;
                    x
                })
                .collect();
            stacked_idx += 1 << (l_skip + n_lift);
            v.push(eval_eq_mle_bits(a, &xi[l_skip + n_lift..l_skip + n_logup], &bits)?);
        }
        eq_3b.push(v);
    }

    // Step 8: eq_ns and eq_sharp_ns.
    let r0 = &rs[0];
    let mut eq_ns = vec![eq_uni(a, l_skip, &xi[0], r0)?];
    let r0p = exp_powers_of_2(a, r0, l_skip.max(1))?;
    let omega = two_adic_generator(l_skip);
    let mut eq_xi = vec![a.cst(F::ONE)?];
    for x in &xi[..l_skip] {
        // evals_eq_hypercube_serial order: index bit i set means factor x_i
        let mut next = vec![zero.clone(); eq_xi.len() * 2];
        let h = eq_xi.len();
        for (k, e) in eq_xi.iter().enumerate() {
            next[k + h] = a.mul(e, x)?;
            next[k] = a.sub(e, &next[k + h])?;
        }
        eq_xi = next;
    }
    let mut sharp = a.cst(F::ZERO)?;
    let mut w = F::ONE;
    for e in &eq_xi {
        let u = eq_uni_c(a, l_skip, &r0p, w)?;
        let x = a.mul(&u, e)?;
        sharp = a.add(&sharp, &x)?;
        w *= omega;
    }
    let mut eq_sharp_ns = vec![sharp];
    for i in 1..=n_max {
        let m = eval_eq_mle(a, &[xi[l_skip + i - 1].clone()], &[rs[i].clone()])?;
        let e = a.mul(&eq_ns[i - 1], &m)?;
        eq_ns.push(e);
        let e = a.mul(&eq_sharp_ns[i - 1], &m)?;
        eq_sharp_ns.push(e);
    }
    let mut r_rev = rs[n_max].clone();
    for i in (0..n_max).rev() {
        eq_ns[i] = a.mul(&eq_ns[i], &r_rev)?;
        eq_sharp_ns[i] = a.mul(&eq_sharp_ns[i], &r_rev)?;
        r_rev = a.mul(&r_rev, &rs[i])?;
    }

    // Step 9: observe common main of every trace, then the other parts per trace.
    for parts in &op {
        for (c, r) in &parts[0] {
            t.observe(a, c)?;
            t.observe(a, r)?;
        }
    }
    let lambda_pows_n = s.trace_to_air.iter().map(|&i| s.airs[i].constraint_idx.len()).max().unwrap_or(0);
    let lambda_pows = crate::poly::powers(a, &p1.lambda, lambda_pows_n)?;
    let max_msg = s.trace_to_air.iter().flat_map(|&i| s.airs[i].interactions.iter().map(|x| x.message.len())).max().unwrap_or(0);
    let beta_pows = crate::poly::powers(a, &p1.beta, max_msg + 1)?;
    let mut inter_evals = vec![];
    let mut cons_evals = vec![];
    for (ti, &air) in s.trace_to_air.iter().enumerate() {
        let ash = &s.airs[air];
        let n = s.n_per_trace[ti];
        let n_lift = n.max(0) as usize;
        for part in &op[ti][1..] {
            for (c, r) in part {
                t.observe(a, c)?;
                t.observe(a, r)?;
            }
        }
        let has_pre = ash.preprocessed_commit.is_some();
        let pre = has_pre.then(|| &op[ti][1]);
        let cached_idx = 1 + has_pre as usize;
        let mut main: Vec<&Vec<(A::V, A::V)>> = op[ti][cached_idx..].iter().collect();
        main.push(&op[ti][0]);
        let (l, rs_n, norm) = if n < 0 {
            let k = (-n) as usize;
            let pw = exp_powers_of_2(a, r0, k + 1)?;
            (l_skip - k, vec![pw[k].clone()], F::from(1u64 << k).invert().unwrap())
        } else {
            (l_skip, rs[..=n as usize].to_vec(), F::ONE)
        };
        let inv = F::from(1u64 << l).invert().unwrap();
        let pf = progression_exp_2(a, &rs_n[0], l)?;
        let rw = a.mul_c(&rs_n[0], two_adic_generator(l))?;
        let pl = progression_exp_2(a, &rw, l)?;
        let mut first = a.mul_c(&pf, inv)?;
        let mut last = a.mul_c(&pl, inv)?;
        for r in &rs_n[1..] {
            let one_m = a.lin(&[(-F::ONE, r.clone())], F::ONE)?;
            first = a.mul(&first, &one_m)?;
            last = a.mul(&last, r)?;
        }
        let trans = a.lin(&[(-F::ONE, last.clone())], F::ONE)?;
        let mut nodes: Vec<A::V> = Vec::with_capacity(ash.nodes.len());
        for nd in &ash.nodes {
            let v = match nd {
                Node::Var { part, index, offset } => {
                    let pair = match part {
                        None => &pre.expect("preprocessed")[*index],
                        Some(p) => &main[*p][*index],
                    };
                    if *offset == 0 { pair.0.clone() } else { pair.1.clone() }
                }
                Node::Public(i) => p1.public_values[air][*i].clone(),
                Node::First => first.clone(),
                Node::Last => last.clone(),
                Node::Transition => trans.clone(),
                Node::Const(c) => a.cst(*c)?,
                Node::Add(x, y) => a.add(&nodes[*x], &nodes[*y])?,
                Node::Sub(x, y) => a.sub(&nodes[*x], &nodes[*y])?,
                Node::Neg(x) => a.lin(&[(-F::ONE, nodes[*x].clone())], F::ZERO)?,
                Node::Mul(x, y) => a.mul(&nodes[*x], &nodes[*y])?,
            };
            nodes.push(v);
        }
        let mut expr = a.cst(F::ZERO)?;
        for (k, &idx) in ash.constraint_idx.iter().enumerate() {
            expr = a.mul_add(&nodes[idx], &lambda_pows[k], &expr)?;
        }
        cons_evals.push(a.mul(&eq_ns[n_lift], &expr)?);
        let mut num = a.cst(F::ZERO)?;
        let mut den = a.cst(F::ZERO)?;
        for (j, it) in ash.interactions.iter().enumerate() {
            let mut d = a.cst(F::ZERO)?;
            for (m, &idx) in it.message.iter().enumerate() {
                d = a.mul_add(&nodes[idx], &beta_pows[m], &d)?;
            }
            let x = a.mul_c(&beta_pows[it.message.len()], F::from(it.bus + 1))?;
            d = a.add(&d, &x)?;
            num = a.mul_add(&eq_3b[ti][j], &nodes[it.count], &num)?;
            den = a.mul_add(&eq_3b[ti][j], &d, &den)?;
        }
        let x = a.mul(&num, &eq_sharp_ns[n_lift])?;
        inter_evals.push(a.mul_c(&x, norm)?);
        inter_evals.push(a.mul(&den, &eq_sharp_ns[n_lift])?);
    }
    let mut acc = a.cst(F::ZERO)?;
    let mut mp = a.cst(F::ONE)?;
    for e in inter_evals.iter().chain(&cons_evals) {
        acc = a.mul_add(e, &mp, &acc)?;
        mp = a.mul(&mp, &p1.mu)?;
    }
    a.assert_eq(&acc, &p1.cur_sum, "batch constraint final claim")?;
    Ok(op)
}
