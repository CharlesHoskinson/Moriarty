//! X-EXP (Sherlock freeze experiments X10 and X7 baseline): an OpenVM internal node at the OD-19
//! internal set, with and without the seal-time state header opening as one more AIR of the node
//! proof, and a node over node children.
//!
//! Usage:
//!   x_exp_node x10 <leaf_log_rows> <arity>
//!     Leaf: W1 (Poseidon2 AIR) at app params. Node: VerifierSubCircuit<arity> at the internal set,
//!     once as is and once with `HeaderOpenAir` appended. Keygen, tracegen and shape for both;
//!     prove and verify the node with the opening; one mutant (a forged header word under the
//!     honest post_d) must not verify; parity of the AIR digest with the stark-sdk sponge.
//!   x_exp_node nn <leaf_log_rows> <inner_arity> <outer_arities>
//!     Leaf proofs, then two node proofs of arity <inner_arity> at the internal set, then the
//!     shape of a node over 1 and 2 of those node proofs (internal-set children), each proved and
//!     verified when its stacked layout fits.
//!   x_exp_node x14 <leaf_log_rows> <ops> <log_heights> <prove_log_height>
//!     X14: BLS12-381 operation AIRs (fr_mul, fp_mul, g1_add, g1_double; OpenVM mod-builder,
//!     8-bit limbs, one operation per row): width, constraints, interactions; then a node of
//!     arity 1 over an internal-set node proof with one op AIR at each height appended (the FV
//!     work unit beside one child verification), its stacked layout, and one proof.
//!
//! `HeaderOpenAir` is the FP-5 opening: state_d = P2(SHK_SD2, outcome_tag, outcome_payload,
//! input_d, deps_d, view_root, body_d), 42 BabyBear words (1 domain word, 1 tag word, five 8-word
//! digests), absorbed by the stark-sdk `PaddingFreeSponge<Poseidon2, 16, 8, 8>` in 6 permutations.
//! One permutation per row (8 rows: 6 used, 2 padding), a one-hot row selector, the capacity
//! chained row to row, the partial last block in overwrite mode, the domain word fixed, and the
//! last used row's rate output equal to the 8 public values (the authenticated post_d).
//! Prints one JSON object per line on stdout.

use std::{array::from_fn, borrow::Borrow, cmp::Reverse, collections::BTreeMap, io::Write, sync::Arc, time::Instant};

use openvm_cpu_backend::CpuBackend;
use openvm_poseidon2_air::{Poseidon2Config, Poseidon2SubAir, Poseidon2SubChip, Poseidon2SubCols, POSEIDON2_WIDTH};
use openvm_recursion_circuit::system::{
    AggregationSubCircuit, CachedTraceCtx, VerifierSubCircuit, VerifierTraceGen,
};
use openvm_stark_backend::{
    air_builders::sub::SubAirBuilder,
    calculate_n_logup,
    keygen::types::{MultiStarkProvingKey, MultiStarkVerifyingKey},
    p3_air::{Air, AirBuilder, AirBuilderWithPublicValues, BaseAir},
    p3_field::{PrimeCharacteristicRing, PrimeField32},
    p3_matrix::{dense::RowMajorMatrix, Matrix},
    proof::Proof,
    prover::{stacked_pcs::StackedLayout, AirProvingContext, ColMajorMatrix, DeviceDataTransporter, ProvingContext},
    AirRef, BaseAirWithPublicValues, PartitionedBaseAir, StarkEngine, SystemParams, WhirProximityStrategy,
};
use openvm_stark_sdk::config::{
    app_params_with_100_bits_security,
    baby_bear_poseidon2::{
        default_duplex_sponge_recorder, poseidon2_perm, BabyBearPoseidon2Config, BabyBearPoseidon2CpuEngine,
        DuplexSponge, DuplexSpongeRecorder,
    },
    params_with_100_bits_security, MAX_APP_LOG_STACKED_HEIGHT,
};
use openvm_poseidon2_air::p3_symmetric::{CryptographicHasher, PaddingFreeSponge};
use p3_baby_bear::BabyBear;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};

type F = BabyBear;
type SC = BabyBearPoseidon2Config;
type Eng = BabyBearPoseidon2CpuEngine<DuplexSponge>;

const SBOX: usize = 1;
const RATE: usize = 8;
const DIG: usize = 8;
const HW: usize = 42;
const NPERM: usize = 6;
const NROW: usize = 8;
/// Stand-in value of the domain word SHK_SD2 (ENC-1 does not write the BabyBear value; any fixed
/// constant gives the same shape).
const SHK_SD2: u32 = 0x5348_4b32 % 0x7800_0001;

// ------------------------------------------------------------------ the opening AIR
struct HeaderOpenAir {
    p2: Arc<Poseidon2SubAir<F, SBOX>>,
}

impl HeaderOpenAir {
    fn p2w(&self) -> usize {
        self.p2.width()
    }
}
impl<T> BaseAir<T> for HeaderOpenAir {
    fn width(&self) -> usize {
        BaseAir::<F>::width(self.p2.as_ref()) + NROW
    }
}
impl<T: openvm_stark_backend::p3_field::Field> BaseAirWithPublicValues<T> for HeaderOpenAir {
    fn num_public_values(&self) -> usize {
        DIG
    }
}
impl<T: openvm_stark_backend::p3_field::Field> PartitionedBaseAir<T> for HeaderOpenAir {}

impl<AB: AirBuilder<F = F> + AirBuilderWithPublicValues> Air<AB> for HeaderOpenAir {
    fn eval(&self, builder: &mut AB) {
        let pw = self.p2w();
        let main = builder.main();
        let local: Vec<AB::Var> = main.row_slice(0).expect("row").to_vec();
        let next: Vec<AB::Var> = main.row_slice(1).expect("next").to_vec();
        drop(main);
        let pvs: Vec<AB::Expr> = builder.public_values().iter().map(|&x| x.into()).collect();
        {
            let mut sub = SubAirBuilder::<AB, Poseidon2SubAir<F, SBOX>, F>::new(builder, 0..pw);
            self.p2.eval(&mut sub);
        }
        let lp: &Poseidon2SubCols<AB::Var, SBOX> = local[..pw].borrow();
        let np: &Poseidon2SubCols<AB::Var, SBOX> = next[..pw].borrow();
        let ls = &local[pw..];
        let ns = &next[pw..];
        let lpost = lp.ending_full_rounds.last().unwrap().post.clone();
        // one-hot row selector, s_0 on the first row, shifted by one each row
        let mut sum = AB::Expr::ZERO;
        for s in ls {
            builder.assert_bool(s.clone());
            sum += s.clone().into();
        }
        builder.assert_one(sum);
        builder.when_first_row().assert_one(ls[0].clone());
        for i in 0..NROW - 1 {
            builder.when_transition().assert_eq(ns[i + 1].clone(), ls[i].clone());
        }
        // row 0: domain word, zero capacity
        builder.when(ls[0].clone()).assert_eq(lp.inputs[0].clone(), AB::Expr::from_u32(SHK_SD2));
        for k in RATE..POSEIDON2_WIDTH {
            builder.when(ls[0].clone()).assert_zero(lp.inputs[k].clone());
        }
        // rows 1..5: capacity chained from the previous permutation
        for i in 0..NPERM - 1 {
            for k in RATE..POSEIDON2_WIDTH {
                builder.when_transition().when(ls[i].clone()).assert_eq(np.inputs[k].clone(), lpost[k].clone());
            }
        }
        // row 5: partial block of 42 - 40 = 2 words, overwrite mode keeps rate words 2..8
        let tail = HW - (NPERM - 1) * RATE;
        for k in tail..RATE {
            builder.when_transition().when(ls[NPERM - 2].clone()).assert_eq(np.inputs[k].clone(), lpost[k].clone());
        }
        // row 5 output: the rate half equals the authenticated post_d (8 public values)
        for k in 0..DIG {
            builder.when(ls[NPERM - 1].clone()).assert_eq(lpost[k].clone(), pvs[k].clone());
        }
    }
}

fn header_air() -> HeaderOpenAir {
    let chip = Poseidon2SubChip::<F, SBOX>::new(Poseidon2Config::<F>::default().constants);
    HeaderOpenAir { p2: chip.air.clone() }
}

/// Header words: [SHK_SD2, outcome_tag, payload(8), input_d(8), deps_d(8), view_root(8), body_d(8)].
fn header_words(rng: &mut StdRng, zero_payload: bool) -> [F; HW] {
    let mut w = [F::ZERO; HW];
    w[0] = F::from_u32(SHK_SD2);
    w[1] = F::from_u32(2); // terminal kind (Halted), a tag word
    for (i, x) in w.iter_mut().enumerate().skip(2) {
        let in_payload = i < 2 + DIG;
        if !(zero_payload && in_payload) {
            *x = F::from_u32(rng.random::<u32>() % 0x7800_0001);
        }
    }
    w
}

/// Trace of the opening for `w`, and the AIR's digest (the rate half after permutation 6).
fn header_trace(w: &[F; HW]) -> (RowMajorMatrix<F>, [F; DIG]) {
    let chip = Poseidon2SubChip::<F, SBOX>::new(Poseidon2Config::<F>::default().constants);
    let mut inputs: Vec<[F; POSEIDON2_WIDTH]> = vec![];
    let mut state = [F::ZERO; POSEIDON2_WIDTH];
    for (j, blk) in w.chunks(RATE).enumerate() {
        assert!(j < NPERM);
        state[..blk.len()].copy_from_slice(blk);
        inputs.push(state);
        state = chip.permute(state);
    }
    assert_eq!(inputs.len(), NPERM);
    let digest: [F; DIG] = from_fn(|k| state[k]);
    while inputs.len() < NROW {
        inputs.push([F::ZERO; POSEIDON2_WIDTH]);
    }
    let p = chip.generate_trace(inputs);
    let pw = p.width();
    let mut v = Vec::with_capacity(NROW * (pw + NROW));
    for r in 0..NROW {
        v.extend_from_slice(&p.values[r * pw..(r + 1) * pw]);
        for s in 0..NROW {
            v.push(if s == r { F::ONE } else { F::ZERO });
        }
    }
    (RowMajorMatrix::new(v, pw + NROW), digest)
}

/// stark-sdk sponge on the same words (the hasher of the Merkle trees of every layer).
fn sdk_digest(w: &[F]) -> [F; DIG] {
    let h = PaddingFreeSponge::<_, 16, 8, 8>::new(poseidon2_perm().clone());
    h.hash_iter(w.iter().copied())
}

// ------------------------------------------------------------------ params and shape (as s10x)
fn internal_ud() -> SystemParams {
    let mut p = params_with_100_bits_security(3, 2, 17, 512, 0, 9, WhirProximityStrategy::UniqueDecoding, 4, 21, 4);
    assert_eq!(p.whir.rounds.len(), 3, "WHIR round count");
    for (r, q) in p.whir.rounds.iter_mut().zip([97usize, 82, 81]) {
        r.num_queries = q;
    }
    p.logup.pow_bits = 18;
    p
}

fn log2_exact(h: usize) -> usize {
    assert!(h.is_power_of_two(), "height {h} not a power of two");
    h.trailing_zeros() as usize
}

fn nonzero_rows<M: Matrix<F>>(m: &M) -> usize {
    let (h, w) = (m.height(), m.width());
    for r in (0..h).rev() {
        for c in 0..w {
            if m.get(r, c).unwrap().as_canonical_u32() != 0 {
                return r + 1;
            }
        }
    }
    0
}

fn shape(pk: &MultiStarkProvingKey<SC>, ctxs: &[AirProvingContext<CpuBackend<SC>>], l_skip: usize, n_stack: usize) -> Value {
    assert_eq!(ctxs.len(), pk.per_air.len(), "one ctx per AIR");
    let mut airs = vec![];
    let mut per: Vec<(usize, usize, Vec<usize>, usize, usize)> = vec![];
    let mut cells = 0usize;
    let (mut max_w, mut max_c, mut max_i, mut sum_w) = (0usize, 0usize, 0usize, 0usize);
    for (i, (apk, ctx)) in pk.per_air.iter().zip(ctxs).enumerate() {
        let h = Matrix::height(&ctx.common_main);
        let w = Matrix::width(&ctx.common_main);
        let cw: Vec<usize> = ctx.cached_mains.iter().map(|c| Matrix::width(&c.trace)).collect();
        let pre = apk.vk.params.width.preprocessed.unwrap_or(0);
        let ni = apk.vk.symbolic_constraints.interactions.len();
        let nc = apk.vk.symbolic_constraints.constraints.constraint_idx.len();
        let tw = w + cw.iter().sum::<usize>() + pre;
        max_w = max_w.max(tw);
        sum_w += tw;
        max_c = max_c.max(nc);
        max_i = max_i.max(ni);
        cells += h * (w + cw.iter().sum::<usize>());
        let nz = nonzero_rows(&ctx.common_main);
        airs.push(json!({"idx": i, "name": apk.air_name, "height": h, "log_height": log2_exact(h),
                         "nonzero_rows": nz, "width_common": w, "width_cached": cw,
                         "width_preprocessed": pre, "num_interactions": ni, "num_constraints": nc,
                         "max_constraint_degree": apk.vk.max_constraint_degree}));
        per.push((i, w, cw, log2_exact(h), ni));
    }
    let mut sorted = per.clone();
    sorted.sort_by_key(|t| Reverse(t.3));
    let lsh = n_stack + l_skip;
    let common = StackedLayout::new(l_skip, lsh, sorted.iter().map(|t| (t.1, t.3)).collect())
        .map(|l| l.width())
        .map_err(|e| format!("{e:?}"));
    let other: Vec<Result<usize, String>> = sorted
        .iter()
        .flat_map(|t| t.2.iter().map(move |&cw| (cw, t.3)))
        .map(|(cw, lh)| StackedLayout::new(l_skip, lsh, vec![(cw, lh)]).map(|l| l.width()).map_err(|e| format!("{e:?}")))
        .collect();
    let layout_ok = common.is_ok() && other.iter().all(|o| o.is_ok());
    let w_stack = if layout_ok {
        Some(common.clone().unwrap() + other.iter().map(|o| *o.as_ref().unwrap()).sum::<usize>())
    } else {
        None
    };
    let total_int: u64 = per.iter().map(|t| (t.4 as u64) << t.3.max(l_skip)).sum();
    let n_logup = calculate_n_logup(l_skip, total_int);
    let mut hist: BTreeMap<usize, usize> = BTreeMap::new();
    for t in &per {
        *hist.entry(t.3).or_default() += 1;
    }
    let (imax, maxlh) = per.iter().map(|t| (t.0, t.3)).max_by_key(|x| x.1).unwrap();
    json!({
        "airs": airs, "num_airs": per.len(), "main_cells": cells,
        "max_air_width": max_w, "sum_air_width": sum_w, "max_air_constraints": max_c, "max_air_interactions": max_i,
        "log_stacked_height": lsh, "layout_ok": layout_ok,
        "layout_err": common.err().or_else(|| other.iter().find_map(|o| o.clone().err())),
        "w_stack_needed": w_stack, "total_interactions_lifted": total_int, "n_logup": n_logup,
        "gkr_rounds_T": if total_int == 0 { 0 } else { l_skip + n_logup },
        "log_height_histogram": hist, "max_log_height": maxlh,
        "max_air": pk.per_air[imax].air_name, "max_height": 1usize << maxlh,
    })
}

fn leaf_proofs(n: usize, log_rows: usize) -> (MultiStarkVerifyingKey<SC>, Vec<Proof<SC>>, Value) {
    let mut rng = StdRng::seed_from_u64(0x5e7_10c);
    let chip = Poseidon2SubChip::<F, 1>::new(Poseidon2Config::<F>::default().constants);
    let airs: Vec<AirRef<SC>> = vec![chip.air.clone()];
    let params = app_params_with_100_bits_security(MAX_APP_LOG_STACKED_HEIGHT);
    let engine = Eng::new(params.clone());
    let (pk, vk) = engine.keygen(&airs);
    let d_pk = engine.device().transport_pk_to_device(&pk);
    let mut proofs = vec![];
    let mut leaf_shape = Value::Null;
    for i in 0..n {
        let inputs: Vec<[F; 16]> = (0..1usize << log_rows)
            .map(|_| from_fn(|_| F::from_u32(rng.random::<u32>() % (1 << 30))))
            .collect();
        let t = chip.generate_trace(inputs);
        let ctxs = vec![AirProvingContext::simple_no_pis(engine.device().transport_matrix_to_device(&ColMajorMatrix::from_row_major(&t)))];
        if i == 0 {
            leaf_shape = shape(&pk, &ctxs, params.l_skip, params.n_stack);
        }
        let proof = engine.prove(&d_pk, ProvingContext::new(ctxs.into_iter().enumerate().collect())).expect("leaf prove");
        engine.verify(&vk, &proof).expect("leaf verify");
        proofs.push(proof);
        eprintln!("leaf {i} proved+verified");
    }
    (vk, proofs, json!({"kind": "leaf", "log_rows": log_rows, "n": n, "shape": leaf_shape}))
}

/// Node of arity N at the internal set; `extra`: append HeaderOpenAir with (trace, pvs).
/// Returns (vk, proof if proved and verified, json).
fn node<const N: usize>(
    params: SystemParams,
    child_vk: &MultiStarkVerifyingKey<SC>,
    proofs: &[Proof<SC>],
    extra: Option<(RowMajorMatrix<F>, Vec<F>)>,
    prove: bool,
) -> (MultiStarkVerifyingKey<SC>, Option<Proof<SC>>, Value)
where
    VerifierSubCircuit<N>: VerifierTraceGen<CpuBackend<SC>, SC, ()> + AggregationSubCircuit,
{
    assert_eq!(proofs.len(), N);
    let engine = Eng::new(params.clone());
    let circuit = <VerifierSubCircuit<N> as VerifierTraceGen<CpuBackend<SC>, SC, ()>>::new(Arc::new(child_vk.clone()), Default::default());
    let child_pcs = circuit.commit_child_vk(&engine, child_vk);
    let mut airs = circuit.airs::<SC>();
    if extra.is_some() {
        airs.push(Arc::new(header_air()));
    }
    let t = Instant::now();
    let (pk, vk) = engine.keygen(&airs);
    let keygen_s = t.elapsed().as_secs_f64();
    let t = Instant::now();
    let mut ctxs = circuit.generate_proving_ctxs_base::<DuplexSpongeRecorder>(
        child_vk,
        CachedTraceCtx::PcsData(child_pcs),
        proofs,
        &(),
        default_duplex_sponge_recorder(),
    );
    if let Some((tr, pvs)) = extra {
        ctxs.push(AirProvingContext::simple(engine.device().transport_matrix_to_device(&ColMajorMatrix::from_row_major(&tr)), pvs));
    }
    let tracegen_s = t.elapsed().as_secs_f64();
    let sh = shape(&pk, &ctxs, params.l_skip, params.n_stack);
    let layout_ok = sh["layout_ok"].as_bool() == Some(true);
    let mut out = json!({"kind": "node", "arity": N, "keygen_s": keygen_s, "tracegen_s": tracegen_s, "shape": sh});
    let mut proof_out = None;
    if prove && layout_ok {
        let d_pk = engine.device().transport_pk_to_device(&pk);
        let t = Instant::now();
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            engine.prove(&d_pk, ProvingContext::new(ctxs.into_iter().enumerate().collect()))
        }));
        let prove_s = t.elapsed().as_secs_f64();
        match res {
            Ok(Ok(proof)) => {
                let ok = engine.verify(&vk, &proof);
                out["prove"] = json!({"prove_s": prove_s, "verify_ok": ok.is_ok(), "verify_err": ok.as_ref().err().map(|e| format!("{e:?}"))});
                if ok.is_ok() {
                    proof_out = Some(proof);
                }
            }
            Ok(Err(e)) => out["prove"] = json!({"prove_s": prove_s, "prove_err": format!("{e:?}")}),
            Err(_) => out["prove"] = json!({"prove_s": prove_s, "prove_panic": true}),
        }
    } else if prove {
        out["prove"] = json!({"skipped": "stacked layout exceeds the internal set"});
    }
    (vk, proof_out, out)
}

macro_rules! node_n {
    ($n:expr, $($args:expr),*) => {
        match $n {
            1 => node::<1>($($args),*),
            2 => node::<2>($($args),*),
            3 => node::<3>($($args),*),
            4 => node::<4>($($args),*),
            _ => panic!("arity not instantiated"),
        }
    };
}

fn emit(v: &Value) {
    let mut so = std::io::stdout();
    writeln!(so, "{}", serde_json::to_string(v).unwrap()).unwrap();
    so.flush().unwrap();
}

fn x10(log_rows: usize, arity: usize) {
    let mut rng = StdRng::seed_from_u64(0x10_0e);
    // parity vectors: random header, zero payload (Running and code outcomes), all-zero body words
    let mut parity = vec![];
    for (name, zp) in [("random", false), ("zero_payload", true)] {
        let w = header_words(&mut rng, zp);
        let (_, d_air) = header_trace(&w);
        let d_sdk = sdk_digest(&w);
        parity.push(json!({"vector": name, "equal": d_air == d_sdk,
                           "words": w.iter().map(|x| x.as_canonical_u32()).collect::<Vec<_>>(),
                           "digest": d_sdk.iter().map(|x| x.as_canonical_u32()).collect::<Vec<_>>()}));
    }
    let mut w0 = [F::ZERO; HW];
    w0[0] = F::from_u32(SHK_SD2);
    let (_, d_air) = header_trace(&w0);
    parity.push(json!({"vector": "all_zero_after_domain", "equal": d_air == sdk_digest(&w0),
                       "digest": sdk_digest(&w0).iter().map(|x| x.as_canonical_u32()).collect::<Vec<_>>()}));
    let air = header_air();
    emit(&json!({"kind": "header_air", "width": BaseAir::<F>::width(&air), "poseidon2_width": air.p2w(),
                 "selector_width": NROW, "rows": NROW, "permutations": NPERM, "header_words": HW,
                 "public_values": DIG, "parity": parity}));
    let (vk, proofs, leaf) = leaf_proofs(arity, log_rows);
    emit(&leaf);
    let p = internal_ud();
    let (_, _, base) = node_n!(arity, p.clone(), &vk, &proofs, None, true);
    emit(&json!({"case": "node_base", "node": base}));
    let w = header_words(&mut rng, false);
    let (tr, d) = header_trace(&w);
    let (_, pr, with) = node_n!(arity, p.clone(), &vk, &proofs, Some((tr, d.to_vec())), true);
    emit(&json!({"case": "node_with_header_open", "proved_verified": pr.is_some(), "node": with}));
    // mutant: view_root word forged (word 2 + 4*8 = 34), pvs keep the honest post_d
    let mut wf = w;
    wf[2 + 4 * DIG] += F::ONE;
    let (trf, _) = header_trace(&wf);
    let (_, prf, mutant) = node_n!(arity, p, &vk, &proofs, Some((trf, d.to_vec())), true);
    emit(&json!({"case": "mutant_seal_view_root_forged", "refused": prf.is_none(), "node": mutant}));
}

fn nn(log_rows: usize, inner: usize, outer: &[usize]) {
    let nmax = *outer.iter().max().unwrap();
    let (vk, proofs, leaf) = leaf_proofs(inner, log_rows);
    emit(&leaf);
    let p = internal_ud();
    let mut node_vk = None;
    let mut node_proofs = vec![];
    for i in 0..nmax {
        let (nvk, pr, out) = node_n!(inner, p.clone(), &vk, &proofs, None, true);
        if i == 0 {
            emit(&json!({"case": "inner_node", "node": out}));
        }
        node_proofs.push(pr.expect("inner node proof"));
        node_vk = Some(nvk);
    }
    let nvk = node_vk.unwrap();
    for &a in outer {
        let (_, pr, out) = node_n!(a, p.clone(), &nvk, &node_proofs[..a], None, true);
        emit(&json!({"case": "node_over_node", "outer_arity": a, "inner_arity": inner,
                     "proved_verified": pr.is_some(), "node": out}));
    }
}

// ------------------------------------------------------------------ X14: BLS12-381 operations as AIRs
const P381: &[u8] = b"1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab";
const R255: &[u8] = b"73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001";

/// One FieldExpr AIR per operation kind, one operation per row: (name, AIR, range checker, inputs per row).
fn op_air(kind: &str) -> (openvm_mod_circuit_builder::FieldExpr, Arc<openvm_circuit_primitives::var_range::VariableRangeCheckerChip>, usize) {
    use num_bigint::BigUint;
    use openvm_circuit_primitives::var_range::{VariableRangeCheckerBus, VariableRangeCheckerChip};
    use openvm_mod_circuit_builder::{ExprBuilder, ExprBuilderConfig, FieldExpr};
    let (m, nl) = match kind {
        "fr_mul" => (R255, 32),
        _ => (P381, 48),
    };
    let modulus = BigUint::parse_bytes(m, 16).unwrap();
    let lb: usize = std::env::var("X14_LIMB_BITS").ok().and_then(|v| v.parse().ok()).unwrap_or(8);
    let rb: usize = std::env::var("X14_RANGE_BITS").ok().and_then(|v| v.parse().ok()).unwrap_or(17);
    let rc = Arc::new(VariableRangeCheckerChip::new(VariableRangeCheckerBus::new(RANGE_BUS, rb)));
    let b = ExprBuilder::new(ExprBuilderConfig { modulus, limb_bits: lb, num_limbs: (nl * 8usize).div_ceil(lb) }, rc.range_max_bits());
    let b = std::rc::Rc::new(std::cell::RefCell::new(b));
    let n_in = match kind {
        "fr_mul" | "fp_mul" => {
            let mut x1 = ExprBuilder::new_input(b.clone());
            let mut x2 = ExprBuilder::new_input(b.clone());
            let mut x3 = &mut x1 * &mut x2;
            x3.save();
            2
        }
        "g1_add" => {
            // affine add, x1 != x2: lambda = (y2 - y1) / (x2 - x1); x3 = lambda^2 - x1 - x2;
            // y3 = lambda (x1 - x3) - y1 (the form of the OpenVM ecc extension)
            let mut x1 = ExprBuilder::new_input(b.clone());
            let mut y1 = ExprBuilder::new_input(b.clone());
            let mut x2 = ExprBuilder::new_input(b.clone());
            let mut y2 = ExprBuilder::new_input(b.clone());
            let mut lam = (&mut y2 - &mut y1) / (&mut x2 - &mut x1);
            let mut x3 = lam.square() - x1.clone() - x2.clone();
            x3.save();
            let mut y3 = lam.clone() * (&mut x1 - &mut x3) - y1.clone();
            y3.save();
            4
        }
        "g1_double" => {
            // affine double, y != 0: lambda = 3 x^2 / (2 y); x3 = lambda^2 - 2 x; y3 = lambda (x - x3) - y
            let mut x1 = ExprBuilder::new_input(b.clone());
            let mut y1 = ExprBuilder::new_input(b.clone());
            let mut lam = x1.square().int_mul(3) / y1.int_mul(2);
            let mut x3 = lam.square() - x1.int_mul(2);
            x3.save();
            let mut y3 = lam.clone() * (&mut x1 - &mut x3) - y1.clone();
            y3.save();
            2
        }
        k => panic!("op {k}"),
    };
    let bb = std::cell::RefCell::borrow(&b).clone();
    (FieldExpr::new(bb, rc.bus(), false), rc, n_in)
}

/// Trace of `n` rows of `kind` on valid random inputs (points on BLS12-381 G1 for g1_*).
fn op_trace(kind: &str, log_rows: usize) -> (Vec<AirRef<SC>>, Vec<RowMajorMatrix<F>>) {
    use num_bigint::BigUint;
    use openvm_circuit_primitives::TraceSubRowGenerator;
    let (expr, rc, _) = op_air(kind);
    let width = BaseAir::<F>::width(&expr);
    let n = 1usize << log_rows;
    let p = BigUint::parse_bytes(P381, 16).unwrap();
    let r = BigUint::parse_bytes(R255, 16).unwrap();
    // G1 generator and its multiples by repeated doubling (host), for valid point inputs
    let gx = BigUint::parse_bytes(b"17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb", 16).unwrap();
    let gy = BigUint::parse_bytes(b"08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1", 16).unwrap();
    let inv = |a: &BigUint| a.modpow(&(&p - 2u32), &p);
    let dbl = |x: &BigUint, y: &BigUint| {
        let lam = (BigUint::from(3u32) * x * x % &p) * inv(&(BigUint::from(2u32) * y % &p)) % &p;
        let x3 = (&lam * &lam + &p * 2u32 - x * 2u32) % &p;
        let y3 = (&lam * ((x + &p - &x3) % &p) + &p - y) % &p;
        (x3, y3)
    };
    let mut rng = StdRng::seed_from_u64(14);
    let mut pts = vec![(gx.clone(), gy.clone())];
    for _ in 0..4 {
        let (x, y) = pts.last().unwrap().clone();
        pts.push(dbl(&x, &y));
    }
    let mut vals = F::zero_vec(n * width);
    for (i, row) in vals.chunks_mut(width).enumerate() {
        let ins: Vec<BigUint> = match kind {
            "fr_mul" => (0..2).map(|_| BigUint::new((0..8).map(|_| rng.random::<u32>()).collect()) % &r).collect(),
            "fp_mul" => (0..2).map(|_| BigUint::new((0..12).map(|_| rng.random::<u32>()).collect()) % &p).collect(),
            "g1_add" => {
                let (a, b) = (&pts[i % 2], &pts[2 + i % 2]);
                vec![a.0.clone(), a.1.clone(), b.0.clone(), b.1.clone()]
            }
            _ => {
                let a = &pts[i % 4];
                vec![a.0.clone(), a.1.clone()]
            }
        };
        expr.generate_subrow((rc.as_ref(), ins, vec![]), row);
    }
    let rt = rc.generate_trace::<F>();
    (vec![Arc::new(expr), Arc::new(rc.air)], vec![RowMajorMatrix::new(vals, width), rt])
}

/// Node of arity 1 over an internal-set child, plus op AIRs at height 2^h. Like `node` with a
/// list of extra AIRs.
fn node_extra(
    params: SystemParams,
    child_vk: &MultiStarkVerifyingKey<SC>,
    proofs: &[Proof<SC>],
    extra: (Vec<AirRef<SC>>, Vec<RowMajorMatrix<F>>),
    prove: bool,
) -> (MultiStarkVerifyingKey<SC>, Option<Proof<SC>>, Value) {
    let engine = Eng::new(params.clone());
    let circuit = <VerifierSubCircuit<1> as VerifierTraceGen<CpuBackend<SC>, SC, ()>>::new(Arc::new(child_vk.clone()), Default::default());
    let child_pcs = circuit.commit_child_vk(&engine, child_vk);
    let mut airs = circuit.airs::<SC>();
    airs.extend(extra.0);
    let (pk, vk) = engine.keygen(&airs);
    let mut ctxs = circuit.generate_proving_ctxs_base::<DuplexSpongeRecorder>(
        child_vk, CachedTraceCtx::PcsData(child_pcs), proofs, &(), default_duplex_sponge_recorder());
    for t in extra.1 {
        ctxs.push(AirProvingContext::simple_no_pis(engine.device().transport_matrix_to_device(&ColMajorMatrix::from_row_major(&t))));
    }
    let sh = shape(&pk, &ctxs, params.l_skip, params.n_stack);
    let layout_ok = sh["layout_ok"].as_bool() == Some(true);
    let mut out = json!({"shape": sh});
    let mut proof_out = None;
    if prove && layout_ok {
        let d_pk = engine.device().transport_pk_to_device(&pk);
        let t = Instant::now();
        let res = engine.prove(&d_pk, ProvingContext::new(ctxs.into_iter().enumerate().collect()));
        let prove_s = t.elapsed().as_secs_f64();
        out["prove"] = match res {
            Ok(proof) => {
                let ok = engine.verify(&vk, &proof);
                let j = json!({"prove_s": prove_s, "verify_ok": ok.is_ok(), "verify_err": ok.as_ref().err().map(|e| format!("{e:?}"))});
                if ok.is_ok() {
                    proof_out = Some(proof);
                }
                j
            }
            Err(e) => json!({"prove_s": prove_s, "prove_err": format!("{e:?}")}),
        };
    }
    (vk, proof_out, out)
}

fn x14(log_rows: usize, kinds: &[String], heights: &[usize], prove_h: usize) {
    for k in kinds {
        let (expr, _, n_in) = op_air(k);
        let (airs, traces) = op_trace(k, 4);
        // standalone shape at the app set: width, constraints, interactions, degree
        let params = app_params_with_100_bits_security(MAX_APP_LOG_STACKED_HEIGHT);
        let engine = Eng::new(params.clone());
        let (pk, _) = engine.keygen(&airs);
        let ctxs: Vec<AirProvingContext<CpuBackend<SC>>> = traces.iter()
            .map(|t| AirProvingContext::simple_no_pis(engine.device().transport_matrix_to_device(&ColMajorMatrix::from_row_major(t))))
            .collect();
        let sh = shape(&pk, &ctxs, params.l_skip, params.n_stack);
        emit(&json!({"kind": "op_air", "op": k, "inputs": n_in, "width": BaseAir::<F>::width(&expr), "air": sh["airs"][0].clone(), "range_air": sh["airs"][1].clone()}));
    }
    // child: an internal-set node proof over one W1 leaf (an FV child stands in as a node proof)
    let (lvk, lproofs, leaf) = leaf_proofs(1, log_rows);
    emit(&leaf);
    let p = internal_ud();
    let (nvk, npr, nout) = node::<1>(p.clone(), &lvk, &lproofs, None, true);
    emit(&json!({"case": "child_node", "node": nout}));
    let child = vec![npr.expect("child node proof")];
    let base = node_extra(p.clone(), &nvk, &child, (vec![], vec![]), false).2;
    emit(&json!({"case": "unit_node_base", "node": base}));
    for k in kinds {
        for &h in heights {
            let ex = op_trace(k, h);
            let out = node_extra(p.clone(), &nvk, &child, ex, h == prove_h).2;
            let ok = out["shape"]["layout_ok"].as_bool() == Some(true);
            emit(&json!({"case": "unit_node_with_ops", "op": k, "log_rows": h, "layout_ok": ok, "node": out}));
        }
    }
}


// ------------------------------------------------------------------ X14 (ruling 2026-11-12): G1 as fp_mul-class rows
/// Bus indices of the X14 work AIRs, far above the verifier sub-circuit's buses (0, 1, ...).
const RANGE_BUS: u16 = 1000;

#[derive(Clone, Copy, Debug)]
enum Ref {
    Id,
    In(usize),
    Var(usize),
}
#[derive(Clone, Debug)]
struct Msg {
    bus: u16,
    send: bool,
    refs: Vec<Ref>,
}

/// One FieldExpr row kind (one modular constraint, fp_mul class) plus an `id` column and bus
/// messages that carry its inputs and outputs (at most 2 field values of 48 limbs per message:
/// the LogUp message length bound is 2^7 = 128 fields).
struct ChainAir {
    expr: openvm_mod_circuit_builder::FieldExpr,
    ew: usize,
    msgs: Vec<Msg>,
}
impl<T> BaseAir<T> for ChainAir {
    fn width(&self) -> usize {
        self.ew + 1
    }
}
impl<T: openvm_stark_backend::p3_field::Field> BaseAirWithPublicValues<T> for ChainAir {}
impl<T: openvm_stark_backend::p3_field::Field> PartitionedBaseAir<T> for ChainAir {}
impl<AB: openvm_stark_backend::interaction::InteractionBuilder> Air<AB> for ChainAir {
    fn eval(&self, builder: &mut AB) {
        use openvm_circuit_primitives::SubAir;
        let main = builder.main();
        let local: Vec<AB::Var> = main.row_slice(0).expect("row").to_vec();
        drop(main);
        SubAir::eval(&self.expr, builder, &local[..self.ew]);
        let cols = self.expr.load_vars(&local[..self.ew]);
        let id = local[self.ew].clone();
        for m in &self.msgs {
            let mut f: Vec<AB::Expr> = vec![];
            for r in &m.refs {
                match r {
                    Ref::Id => f.push(id.clone().into()),
                    Ref::In(i) => f.extend(cols.inputs[*i].iter().map(|v| v.clone().into())),
                    Ref::Var(j) => f.extend(cols.vars[*j].iter().map(|v| v.clone().into())),
                }
            }
            let bus = openvm_stark_backend::interaction::PermutationCheckBus::new(m.bus);
            if m.send {
                bus.send(builder, f, cols.is_valid.clone());
            } else {
                bus.receive(builder, f, cols.is_valid.clone());
            }
        }
    }
}

/// Test harness only: stands for the producer of the input points and the consumer of the
/// output point (the MSM accumulator in a real verifier). Columns: id, then `nv` values of 48 limbs.
struct SourceAir {
    nv: usize,
    nl: usize,
    msgs: Vec<(u16, bool, Vec<Option<usize>>)>,
}
impl<T> BaseAir<T> for SourceAir {
    fn width(&self) -> usize {
        1 + self.nv * self.nl
    }
}
impl<T: openvm_stark_backend::p3_field::Field> BaseAirWithPublicValues<T> for SourceAir {}
impl<T: openvm_stark_backend::p3_field::Field> PartitionedBaseAir<T> for SourceAir {}
impl<AB: openvm_stark_backend::interaction::InteractionBuilder> Air<AB> for SourceAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local: Vec<AB::Var> = main.row_slice(0).expect("row").to_vec();
        drop(main);
        for (bus, send, refs) in &self.msgs {
            let mut f: Vec<AB::Expr> = vec![];
            for r in refs {
                match r {
                    None => f.push(local[0].clone().into()),
                    Some(k) => f.extend(local[1 + k * self.nl..1 + (k + 1) * self.nl].iter().map(|v| v.clone().into())),
                }
            }
            let b = openvm_stark_backend::interaction::PermutationCheckBus::new(*bus);
            if *send {
                b.send(builder, f, AB::Expr::ONE);
            } else {
                b.receive(builder, f, AB::Expr::ONE);
            }
        }
    }
}

fn limbs_of(x: &num_bigint::BigUint, nl: usize) -> Vec<F> {
    let mut b = x.to_bytes_le();
    b.resize(nl, 0);
    b.iter().map(|&v| F::from_u32(v as u32)).collect()
}

/// The G1 chain of one operation kind ("add" or "dbl") over BLS12-381 Fp, 8-bit limbs:
/// add: L_add (lambda = (y2 - y1) / (x2 - x1)), X3 (x3 = lambda^2 - x1 - x2), Y3 (y3 = lambda (x1 - x3) - y1);
/// dbl: L_dbl (lambda = 3 x^2 / (2 y)), X3 (x2 = x1), Y3. Returns the AIRs [L, X3, Y3, range, source]
/// and their traces of 2^log_rows rows (one G1 operation per row of each AIR).
fn g1_chain(kind: &str, log_rows: usize) -> (Vec<AirRef<SC>>, Vec<RowMajorMatrix<F>>, Vec<&'static str>) {
    use num_bigint::BigUint;
    use openvm_circuit_primitives::{var_range::{VariableRangeCheckerBus, VariableRangeCheckerChip}, TraceSubRowGenerator};
    use openvm_mod_circuit_builder::{ExprBuilder, ExprBuilderConfig, FieldExpr};
    let nl = 48;
    let p = BigUint::parse_bytes(P381, 16).unwrap();
    let rb: usize = std::env::var("X14_RANGE_BITS").ok().and_then(|v| v.parse().ok()).unwrap_or(17);
    let rc = Arc::new(VariableRangeCheckerChip::new(VariableRangeCheckerBus::new(RANGE_BUS, rb)));
    let mk = |f: &dyn Fn(Rc<RefCell<ExprBuilder>>)| {
        let b = ExprBuilder::new(ExprBuilderConfig { modulus: p.clone(), limb_bits: 8, num_limbs: nl }, rc.range_max_bits());
        let b = Rc::new(RefCell::new(b));
        f(b.clone());
        let bb = RefCell::borrow(&b).clone();
        FieldExpr::new(bb, rc.bus(), false)
    };
    use std::{cell::RefCell, rc::Rc};
    let l_expr = if kind == "add" {
        mk(&|b| {
            let mut x1 = ExprBuilder::new_input(b.clone());
            let mut y1 = ExprBuilder::new_input(b.clone());
            let mut x2 = ExprBuilder::new_input(b.clone());
            let mut y2 = ExprBuilder::new_input(b.clone());
            let mut l = (&mut y2 - &mut y1) / (&mut x2 - &mut x1);
            l.save();
        })
    } else {
        // lambda = 3 s / (2 y) with s = x^2 from the SQ row (one constraint per row kind)
        mk(&|b| {
            let mut s = ExprBuilder::new_input(b.clone());
            let _x = ExprBuilder::new_input(b.clone());
            let mut y = ExprBuilder::new_input(b.clone());
            let mut l = s.int_mul(3) / y.int_mul(2);
            l.save();
        })
    };
    let sq_expr = mk(&|b| {
        let mut x = ExprBuilder::new_input(b.clone());
        let mut s = x.square();
        s.save();
    });
    let x_expr = mk(&|b| {
        let mut l = ExprBuilder::new_input(b.clone());
        let x1 = ExprBuilder::new_input(b.clone());
        let x2 = ExprBuilder::new_input(b.clone());
        let mut x3 = l.square() - x1.clone() - x2.clone();
        x3.save();
    });
    let y_expr = mk(&|b| {
        let mut l = ExprBuilder::new_input(b.clone());
        let mut x1 = ExprBuilder::new_input(b.clone());
        let mut x3 = ExprBuilder::new_input(b.clone());
        let y1 = ExprBuilder::new_input(b.clone());
        let mut y3 = l * (&mut x1 - &mut x3) - y1.clone();
        y3.save();
    });
    for e in [&l_expr, &x_expr, &y_expr, &sq_expr] {
        assert_eq!(e.num_vars(), 1, "one modular constraint per row kind");
    }
    let (id, i, v) = (Ref::Id, Ref::In, Ref::Var);
    let m = |bus: u16, send: bool, refs: Vec<Ref>| Msg { bus, send, refs };
    let o = if kind == "add" { 1100u16 } else { 1110 };
    // buses: o+0 P1, o+1 P2 (add only), o+2 (lambda, x1) to X3, o+3 x2 to X3, o+4 (lambda, x1) to Y3,
    // o+5 y1 to Y3, o+6 x3 to Y3, o+7 (x3, y3) out
    let (l_msgs, src_msgs): (Vec<Msg>, Vec<(u16, bool, Vec<Option<usize>>)>) = if kind == "add" {
        (
            vec![m(o, false, vec![id, i(0), i(1)]), m(o + 1, false, vec![id, i(2), i(3)]),
                 m(o + 2, true, vec![id, v(0), i(0)]), m(o + 3, true, vec![id, i(2)]),
                 m(o + 4, true, vec![id, v(0), i(0)]), m(o + 5, true, vec![id, i(1)])],
            vec![(o, true, vec![None, Some(0), Some(1)]), (o + 1, true, vec![None, Some(2), Some(3)]),
                 (o + 7, false, vec![None, Some(4), Some(5)])],
        )
    } else {
        // L_dbl inputs (s, x, y): receive (id, s, x) from SQ on o+9 and (id, y) on o
        (
            vec![m(o + 9, false, vec![id, i(0), i(1)]), m(o, false, vec![id, i(2)]),
                 m(o + 2, true, vec![id, v(0), i(1)]), m(o + 3, true, vec![id, i(1)]),
                 m(o + 4, true, vec![id, v(0), i(1)]), m(o + 5, true, vec![id, i(2)])],
            vec![(o + 8, true, vec![None, Some(0)]), (o, true, vec![None, Some(1)]), (o + 7, false, vec![None, Some(4), Some(5)])],
        )
    };
    let sq_msgs = vec![m(o + 8, false, vec![id, i(0)]), m(o + 9, true, vec![id, v(0), i(0)])];
    let x_msgs = vec![m(o + 2, false, vec![id, i(0), i(1)]), m(o + 3, false, vec![id, i(2)]), m(o + 6, true, vec![id, v(0)])];
    let y_msgs = vec![m(o + 4, false, vec![id, i(0), i(1)]), m(o + 5, false, vec![id, i(3)]), m(o + 6, false, vec![id, i(2)]),
                      m(o + 7, true, vec![id, i(2), v(0)])];
    // host values
    let gx = BigUint::parse_bytes(b"17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb", 16).unwrap();
    let gy = BigUint::parse_bytes(b"08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1", 16).unwrap();
    let inv = |a: &BigUint| a.modpow(&(&p - 2u32), &p);
    let sub = |a: &BigUint, b: &BigUint| (a + &p - b % &p) % &p;
    let dbl = |x: &BigUint, y: &BigUint| {
        let lam = (BigUint::from(3u32) * x * x % &p) * inv(&(BigUint::from(2u32) * y % &p)) % &p;
        let x3 = sub(&sub(&(&lam * &lam % &p), x), x);
        let y3 = sub(&(&lam * sub(x, &x3) % &p), y);
        (lam, x3, y3)
    };
    let mut pts = vec![(gx.clone(), gy.clone())];
    for _ in 0..4 {
        let (x, y) = pts.last().unwrap().clone();
        let (_, x3, y3) = dbl(&x, &y);
        pts.push((x3, y3));
    }
    let n = 1usize << log_rows;
    let widths = [BaseAir::<F>::width(&l_expr), BaseAir::<F>::width(&x_expr), BaseAir::<F>::width(&y_expr), BaseAir::<F>::width(&sq_expr)];
    let mut tq = F::zero_vec(n * (widths[3] + 1));
    let mut tl = F::zero_vec(n * (widths[0] + 1));
    let mut tx = F::zero_vec(n * (widths[1] + 1));
    let mut ty = F::zero_vec(n * (widths[2] + 1));
    let src_nv = 6;
    let mut ts = F::zero_vec(n * (1 + src_nv * nl));
    for r in 0..n {
        let (x1, y1, x2, y2) = if kind == "add" {
            let (a, b) = (&pts[r % 2], &pts[2 + r % 2]);
            (a.0.clone(), a.1.clone(), b.0.clone(), b.1.clone())
        } else {
            let a = &pts[r % 4];
            (a.0.clone(), a.1.clone(), a.0.clone(), a.1.clone())
        };
        let lam = if kind == "add" { sub(&y2, &y1) * inv(&sub(&x2, &x1)) % &p } else { dbl(&x1, &y1).0 };
        let x3 = sub(&sub(&(&lam * &lam % &p), &x1), &x2);
        let y3 = sub(&(&lam * sub(&x1, &x3) % &p), &y1);
        let idf = F::from_u32(r as u32);
        let li = if kind == "add" {
            vec![x1.clone(), y1.clone(), x2.clone(), y2.clone()]
        } else {
            vec![&x1 * &x1 % &p, x1.clone(), y1.clone()]
        };
        if kind != "add" {
            let row = &mut tq[r * (widths[3] + 1)..(r + 1) * (widths[3] + 1)];
            sq_expr.generate_subrow((rc.as_ref(), vec![x1.clone()], vec![]), &mut row[..widths[3]]);
            row[widths[3]] = idf;
        }
        let row = &mut tl[r * (widths[0] + 1)..(r + 1) * (widths[0] + 1)];
        l_expr.generate_subrow((rc.as_ref(), li, vec![]), &mut row[..widths[0]]);
        row[widths[0]] = idf;
        let row = &mut tx[r * (widths[1] + 1)..(r + 1) * (widths[1] + 1)];
        x_expr.generate_subrow((rc.as_ref(), vec![lam.clone(), x1.clone(), x2.clone()], vec![]), &mut row[..widths[1]]);
        row[widths[1]] = idf;
        let row = &mut ty[r * (widths[2] + 1)..(r + 1) * (widths[2] + 1)];
        y_expr.generate_subrow((rc.as_ref(), vec![lam.clone(), x1.clone(), x3.clone(), y1.clone()], vec![]), &mut row[..widths[2]]);
        row[widths[2]] = idf;
        let row = &mut ts[r * (1 + src_nv * nl)..(r + 1) * (1 + src_nv * nl)];
        row[0] = idf;
        for (k, val) in [&x1, &y1, &x2, &y2, &x3, &y3].iter().enumerate() {
            row[1 + k * nl..1 + (k + 1) * nl].copy_from_slice(&limbs_of(val, nl));
        }
    }
    let rt = rc.generate_trace::<F>();
    let mut airs: Vec<AirRef<SC>> = vec![
        Arc::new(ChainAir { ew: widths[0], expr: l_expr, msgs: l_msgs }),
        Arc::new(ChainAir { ew: widths[1], expr: x_expr, msgs: x_msgs }),
        Arc::new(ChainAir { ew: widths[2], expr: y_expr, msgs: y_msgs }),
        Arc::new(rc.air),
        Arc::new(SourceAir { nv: src_nv, nl, msgs: src_msgs }),
    ];
    let mut traces = vec![
        RowMajorMatrix::new(tl, widths[0] + 1),
        RowMajorMatrix::new(tx, widths[1] + 1),
        RowMajorMatrix::new(ty, widths[2] + 1),
        rt,
        RowMajorMatrix::new(ts, 1 + src_nv * nl),
    ];
    let mut names = vec![if kind == "add" { "L_add" } else { "L_dbl" }, "X3", "Y3", "range", "source (harness)"];
    if kind != "add" {
        airs.push(Arc::new(ChainAir { ew: widths[3], expr: sq_expr, msgs: sq_msgs }));
        traces.push(RowMajorMatrix::new(tq, widths[3] + 1));
        names.push("SQ");
    }
    (airs, traces, names)
}

/// Child node for the unit node: an internal-set node proof over one W1 leaf.
fn unit_child(log_rows: usize) -> (MultiStarkVerifyingKey<SC>, Vec<Proof<SC>>) {
    let (lvk, lproofs, leaf) = leaf_proofs(1, log_rows);
    emit(&leaf);
    let (nvk, npr, nout) = node::<1>(internal_ud(), &lvk, &lproofs, None, true);
    emit(&json!({"case": "child_node", "node": nout}));
    (nvk, vec![npr.expect("child node proof")])
}

fn g1c(log_rows: usize, kinds: &[String], heights: &[usize], prove_h: usize) {
    for k in kinds {
        let (airs, traces, names) = g1_chain(k, 4);
        let params = app_params_with_100_bits_security(MAX_APP_LOG_STACKED_HEIGHT);
        let engine = Eng::new(params.clone());
        let (pk, vk) = engine.keygen(&airs);
        let ctxs: Vec<AirProvingContext<CpuBackend<SC>>> = traces.iter()
            .map(|t| AirProvingContext::simple_no_pis(engine.device().transport_matrix_to_device(&ColMajorMatrix::from_row_major(t))))
            .collect();
        let sh = shape(&pk, &ctxs, params.l_skip, params.n_stack);
        let d_pk = engine.device().transport_pk_to_device(&pk);
        let ok = engine.prove(&d_pk, ProvingContext::new(ctxs.into_iter().enumerate().collect())).map(|pr| engine.verify(&vk, &pr).map_err(|e| format!("{e:?}")));
        emit(&json!({"kind": "g1_chain", "op": k, "names": names, "airs": sh["airs"].clone(), "prove_verify_2p4": format!("{ok:?}")}));
    }
    let (nvk, child) = unit_child(log_rows);
    let p = internal_ud();
    for k in kinds {
        for &h in heights {
            let (airs, traces, _) = g1_chain(k, h);
            let (_, _, out) = node_extra(p.clone(), &nvk, &child, (airs, traces), h == prove_h);
            let ok = out["shape"]["layout_ok"].as_bool() == Some(true);
            emit(&json!({"case": "unit_node_with_g1_chain", "op": k, "log_rows": h, "layout_ok": ok, "node": out}));
        }
    }
}

/// (b) of the ruling: the parent node's cost of verifying a unit node whose op AIR has 193
/// interactions (fp_mul at 2^h), at arity 1 and 2, against the parent over a plain node.
fn parent(log_rows: usize, op: &str, h: usize) {
    let (nvk, child) = unit_child(log_rows);
    let p = internal_ud();
    let (uvk, upr, uout) = node_extra(p.clone(), &nvk, &child, op_trace(op, h), true);
    emit(&json!({"case": "unit_node", "op": op, "log_rows": h, "node": uout}));
    let upr = upr.expect("unit node proof");
    for a in [1usize, 2] {
        let proofs: Vec<Proof<SC>> = (0..a).map(|_| upr.clone()).collect();
        let (_, pr, out) = node_n!(a, p.clone(), &uvk, &proofs, None, true);
        emit(&json!({"case": "parent_over_unit", "arity": a, "op": op, "unit_log_rows": h, "proved_verified": pr.is_some(), "node": out}));
    }
    // baseline: parent over the plain child node (no op AIR)
    for a in [1usize, 2] {
        let proofs: Vec<Proof<SC>> = (0..a).map(|_| child[0].clone()).collect();
        let (_, pr, out) = node_n!(a, p.clone(), &nvk, &proofs, None, true);
        emit(&json!({"case": "parent_over_plain_node", "arity": a, "proved_verified": pr.is_some(), "node": out}));
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let t0 = Instant::now();
    match a[1].as_str() {
        "x10" => x10(a[2].parse().unwrap(), a[3].parse().unwrap()),
        "nn" => {
            let outer: Vec<usize> = a[4].split(',').map(|s| s.parse().unwrap()).collect();
            nn(a[2].parse().unwrap(), a[3].parse().unwrap(), &outer)
        }
        "x14" => {
            let kinds: Vec<String> = a[3].split(',').map(String::from).collect();
            let hs: Vec<usize> = a[4].split(',').map(|s| s.parse().unwrap()).collect();
            x14(a[2].parse().unwrap(), &kinds, &hs, a[5].parse().unwrap())
        }
        "ops" => {
            // op AIR shapes only, under X14_LIMB_BITS / X14_RANGE_BITS
            for k in a[2].split(',') {
                let (expr, rc, n_in) = op_air(k);
                let (airs, traces) = op_trace(k, 4);
                let params = app_params_with_100_bits_security(MAX_APP_LOG_STACKED_HEIGHT);
                let engine = Eng::new(params.clone());
                let (pk, vk) = engine.keygen(&airs);
                let ctxs: Vec<AirProvingContext<CpuBackend<SC>>> = traces.iter()
                    .map(|t| AirProvingContext::simple_no_pis(engine.device().transport_matrix_to_device(&ColMajorMatrix::from_row_major(t))))
                    .collect();
                let sh = shape(&pk, &ctxs, params.l_skip, params.n_stack);
                let d_pk = engine.device().transport_pk_to_device(&pk);
                let ok = engine.prove(&d_pk, ProvingContext::new(ctxs.into_iter().enumerate().collect())).map(|p| engine.verify(&vk, &p).is_ok());
                emit(&json!({"kind": "op_air", "op": k, "inputs": n_in, "range_bits": rc.range_max_bits(), "width": BaseAir::<F>::width(&expr),
                             "air": sh["airs"][0].clone(), "range_air": sh["airs"][1].clone(), "prove_verify_2p4": format!("{ok:?}")}));
            }
        }
        "g1c" => {
            let kinds: Vec<String> = a[3].split(',').map(String::from).collect();
            let hs: Vec<usize> = a[4].split(',').map(|s| s.parse().unwrap()).collect();
            g1c(a[2].parse().unwrap(), &kinds, &hs, a[5].parse().unwrap())
        }
        "parent" => parent(a[2].parse().unwrap(), &a[3], a[4].parse().unwrap()),
        m => panic!("mode {m}"),
    }
    eprintln!("done {:.1}s", t0.elapsed().as_secs_f64());
}
