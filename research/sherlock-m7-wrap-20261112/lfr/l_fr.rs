//! M7: one real proof of the Sherlock last layer L with Midnight Poseidon over BLS12-381 Fr as
//! its Merkle and transcript hash (PROPOSAL 3.6, u = 1, 2, 3), at the OD-19 L set.
//!
//! Copy of the D10 program `l_heights.rs` with the BabyBear chain unchanged: two W1 proofs (CPU,
//! app params, 2^20 rows) -> node VerifierSubCircuit::<2> at the OD-19 internal set (UD, blowup
//! 3, queries 97/82/81, fold PoW 0, mu PoW 9, query PoW 21, LogUp PoW 18, l_skip 2, n_stack 17,
//! w_stack 512), proved and verified -> L = VerifierSubCircuit::<1> over the node vk, `L_cached`
//! (has_cached = true, CachedTraceCtx::PcsData), proved with BabyBearBls12381PoseidonCpuEngine
//! at the OD-19 L set (UD, blowup 3, queries 98/83/82, fold PoW 0, mu PoW 6, query PoW 20,
//! LogUp PoW 18, l_skip 2, n_stack 18, w_stack 48 bound) and verified with the stock verifier
//! (`StarkEngine::verify` -> stark-backend `verifier::verify`). Prover randomness is stock.
//!
//! Usage: l_fr. Prints one JSON object on stdout.

use std::{cmp::Reverse, collections::BTreeMap, sync::Arc, time::Instant};

use openvm_cpu_backend::CpuBackend;
use openvm_poseidon2_air::{Poseidon2Config, Poseidon2SubChip};
use openvm_recursion_circuit::system::{
    AggregationSubCircuit, CachedTraceCtx, VerifierConfig, VerifierSubCircuit, VerifierTraceGen,
};
use openvm_stark_backend::{
    air_builders::symbolic::{symbolic_variable::Entry, SymbolicExpressionNode},
    calculate_n_logup,
    keygen::types::{MultiStarkProvingKey, MultiStarkVerifyingKey},
    p3_field::PrimeCharacteristicRing,
    p3_matrix::Matrix,
    proof::Proof,
    prover::{
        stacked_pcs::StackedLayout, AirProvingContext, ColMajorMatrix, DeviceDataTransporter,
        ProvingContext,
    },
    AirRef, StarkEngine, StarkProtocolConfig, SystemParams, WhirProximityStrategy,
};
use openvm_stark_backend::codec::{Decode, Encode};
use openvm_stark_sdk::config::{
    app_params_with_100_bits_security,
    baby_bear_bls12381_poseidon::{
        BabyBearBls12381PoseidonConfig, BabyBearBls12381PoseidonCpuEngine, Bls12381Fr,
    },
    baby_bear_poseidon2::{
        default_duplex_sponge_recorder, BabyBearPoseidon2Config, BabyBearPoseidon2CpuEngine,
        DuplexSponge, DuplexSpongeRecorder,
    },
    params_with_100_bits_security, MAX_APP_LOG_STACKED_HEIGHT,
};
use p3_baby_bear::BabyBear;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};

type F = BabyBear;
type SC = BabyBearPoseidon2Config;
type RootSC = BabyBearBls12381PoseidonConfig;
type Eng = BabyBearPoseidon2CpuEngine<DuplexSponge>;
type RootEng = BabyBearBls12381PoseidonCpuEngine;

#[allow(clippy::too_many_arguments)]
fn ud_params(
    b: usize,
    l_skip: usize,
    n_stack: usize,
    w_stack: usize,
    mu: usize,
    qpow: usize,
    queries: &[usize],
    logup: usize,
) -> SystemParams {
    let mut p = params_with_100_bits_security(
        b, l_skip, n_stack, w_stack, 0, mu, WhirProximityStrategy::UniqueDecoding, 4, qpow, 4,
    );
    assert_eq!(p.whir.rounds.len(), queries.len(), "WHIR round count");
    for (r, &q) in p.whir.rounds.iter_mut().zip(queries) {
        r.num_queries = q;
    }
    p.logup.pow_bits = logup;
    p
}

/// OD-19 internal set (UD-SIZING.md 5.1).
fn internal_ud() -> SystemParams {
    ud_params(3, 2, 17, 512, 9, 21, &[97, 82, 81], 18)
}
/// OD-19 L set (UD-SIZING.md 5.1) at the w_stack bound 48.
fn l_ud(w_stack: usize) -> SystemParams {
    ud_params(3, 2, 18, w_stack, 6, 20, &[98, 83, 82], 18)
}

fn params_json(p: &SystemParams) -> Value {
    json!({
        "l_skip": p.l_skip, "n_stack": p.n_stack, "w_stack": p.w_stack, "log_blowup": p.log_blowup,
        "k_whir": p.k_whir(), "proximity": format!("{:?}", p.whir.proximity),
        "queries": p.whir.rounds.iter().map(|r| r.num_queries).collect::<Vec<_>>(),
        "folding_pow_bits": p.whir.folding_pow_bits, "mu_pow_bits": p.whir.mu_pow_bits,
        "query_phase_pow_bits": p.whir.query_phase_pow_bits, "logup_pow_bits": p.logup.pow_bits,
    })
}

fn log2_exact(h: usize) -> usize {
    assert!(h.is_power_of_two(), "height {h} not a power of two");
    h.trailing_zeros() as usize
}

/// Shape of one L tracegen: per-AIR heights, stacked widths, interactions, n_logup.
fn shape<C: StarkProtocolConfig<F = F>>(
    pk: &MultiStarkProvingKey<C>,
    ctxs: &[AirProvingContext<CpuBackend<C>>],
    l_skip: usize,
    n_stack: usize,
) -> Value {
    assert_eq!(ctxs.len(), pk.per_air.len(), "one ctx per AIR");
    let mut airs = vec![];
    // (air_idx, common width, cached widths, log_height, num_interactions)
    let mut per: Vec<(usize, usize, Vec<usize>, usize, usize)> = vec![];
    let mut cells = 0usize;
    for (i, (apk, ctx)) in pk.per_air.iter().zip(ctxs).enumerate() {
        let h = Matrix::height(&ctx.common_main);
        let w = Matrix::width(&ctx.common_main);
        assert_eq!(w, apk.vk.params.width.common_main, "width of AIR {i}");
        let cached: Vec<(usize, usize)> =
            ctx.cached_mains.iter().map(|c| (Matrix::height(&c.trace), Matrix::width(&c.trace))).collect();
        for (ch, _) in &cached {
            assert_eq!(*ch, h, "cached height of AIR {i}");
        }
        let ni = apk.vk.symbolic_constraints.interactions.len();
        let cw: Vec<usize> = cached.iter().map(|c| c.1).collect();
        cells += h * (w + cw.iter().sum::<usize>());
        airs.push(json!({"idx": i, "name": apk.air_name, "height": h, "log_height": log2_exact(h),
                         "width_common": w, "width_cached": cw, "num_interactions": ni,
                         "need_rot": apk.vk.params.need_rot}));
        per.push((i, w, cw, log2_exact(h), ni));
    }
    let mut sorted = per.clone();
    sorted.sort_by_key(|t| Reverse(t.3)); // stable, as sorted_by_key in proof_shape.rs:375
    let lsh = n_stack + l_skip;
    let common = StackedLayout::new(l_skip, lsh, sorted.iter().map(|t| (t.1, t.3)).collect())
        .expect("common layout")
        .width();
    let other: Vec<usize> = sorted
        .iter()
        .flat_map(|t| t.2.iter().map(move |&cw| (cw, t.3)))
        .map(|(cw, lh)| StackedLayout::new(l_skip, lsh, vec![(cw, lh)]).expect("layout").width())
        .collect();
    let total_int: u64 = per.iter().map(|t| (t.4 as u64) << t.3.max(l_skip)).sum();
    let n_logup = calculate_n_logup(l_skip, total_int);
    let mut hist: BTreeMap<usize, usize> = BTreeMap::new();
    for t in &per {
        *hist.entry(t.3).or_default() += 1;
    }
    json!({
        "airs": airs, "main_cells": cells,
        "stacked_width_common": common, "stacked_width_other": other,
        "w_stack_needed": common + other.iter().sum::<usize>(),
        "total_interactions_lifted": total_int, "n_logup": n_logup,
        "gkr_rounds_T": if total_int == 0 { 0 } else { l_skip + n_logup },
        "distinct_log_heights": hist.keys().collect::<Vec<_>>(), "log_height_histogram": hist,
        "max_log_height": per.iter().map(|t| t.3).max(),
    })
}

#[allow(dead_code)]
fn dump<C: StarkProtocolConfig<F = F>>(pk: &MultiStarkProvingKey<C>) -> Value {
    let mut airs = vec![];
    for apk in pk.per_air.iter() {
        let vk = &apk.vk;
        let w = &vk.params.width;
        let dag = &vk.symbolic_constraints.constraints;
        let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
        for n in &dag.nodes {
            let k = match n {
                SymbolicExpressionNode::Variable(v) => match v.entry {
                    Entry::Preprocessed { .. } => "var_pre",
                    Entry::Main { .. } => "var_main",
                    Entry::Public => "var_public",
                    Entry::Challenge => "var_challenge",
                },
                SymbolicExpressionNode::IsFirstRow => "is_first_row",
                SymbolicExpressionNode::IsLastRow => "is_last_row",
                SymbolicExpressionNode::IsTransition => "is_transition",
                SymbolicExpressionNode::Constant(_) => "constant",
                SymbolicExpressionNode::Add { .. } => "add",
                SymbolicExpressionNode::Sub { .. } => "sub",
                SymbolicExpressionNode::Neg { .. } => "neg",
                SymbolicExpressionNode::Mul { .. } => "mul",
            };
            *kinds.entry(k).or_default() += 1;
        }
        let ints = &vk.symbolic_constraints.interactions;
        airs.push(json!({
            "name": apk.air_name, "width_preprocessed": w.preprocessed.unwrap_or(0),
            "width_cached_mains": w.cached_mains, "width_common_main": w.common_main,
            "need_rot": vk.params.need_rot, "num_public_values": vk.params.num_public_values,
            "max_constraint_degree": vk.max_constraint_degree,
            "num_constraints": dag.constraint_idx.len(), "dag_nodes": dag.nodes.len(),
            "dag_node_kinds": kinds, "num_interactions": ints.len(),
            "interaction_msg_len_sum": ints.iter().map(|it| it.message.len()).sum::<usize>(),
        }));
    }
    json!({"num_airs": pk.per_air.len(), "params": params_json(&pk.params),
           "max_constraint_degree": pk.max_constraint_degree, "airs": airs})
}

fn w1_proofs(n: usize, log_rows: usize) -> (MultiStarkVerifyingKey<SC>, Vec<Proof<SC>>) {
    let mut rng = StdRng::seed_from_u64(0x5e7_10c);
    let chip = Poseidon2SubChip::<F, 1>::new(Poseidon2Config::<F>::default().constants);
    let airs: Vec<AirRef<SC>> = vec![chip.air.clone()];
    let engine = Eng::new(app_params_with_100_bits_security(MAX_APP_LOG_STACKED_HEIGHT));
    let (pk, vk) = engine.keygen(&airs);
    let d_pk = engine.device().transport_pk_to_device(&pk);
    let mut proofs = vec![];
    for _ in 0..n {
        let inputs: Vec<[F; 16]> = (0..1usize << log_rows)
            .map(|_| std::array::from_fn(|_| F::from_u32(rng.random::<u32>() % (1 << 30))))
            .collect();
        let trace = chip.generate_trace(inputs);
        let cm = ColMajorMatrix::from_row_major(&trace);
        let ctx = ProvingContext::new(vec![(
            0,
            AirProvingContext::simple_no_pis(engine.device().transport_matrix_to_device(&cm)),
        )]);
        let proof = engine.prove(&d_pk, ctx).expect("w1 prove");
        engine.verify(&vk, &proof).expect("w1 verify");
        proofs.push(proof);
    }
    (vk, proofs)
}

fn node_proof<const N: usize>(
    params: SystemParams,
    child_vk: &MultiStarkVerifyingKey<SC>,
    proofs: &[Proof<SC>],
) -> (MultiStarkVerifyingKey<SC>, Proof<SC>, Value)
where
    VerifierSubCircuit<N>: VerifierTraceGen<CpuBackend<SC>, SC, ()> + AggregationSubCircuit,
{
    let engine = Eng::new(params.clone());
    let circuit = <VerifierSubCircuit<N> as VerifierTraceGen<CpuBackend<SC>, SC, ()>>::new(
        Arc::new(child_vk.clone()),
        Default::default(),
    );
    let child_pcs = circuit.commit_child_vk(&engine, child_vk);
    let airs = circuit.airs::<SC>();
    let (pk, vk) = engine.keygen(&airs);
    let d_pk = engine.device().transport_pk_to_device(&pk);
    let ctxs = circuit.generate_proving_ctxs_base::<DuplexSpongeRecorder>(
        child_vk,
        CachedTraceCtx::PcsData(child_pcs),
        proofs,
        &(),
        default_duplex_sponge_recorder(),
    );
    let sh = shape(&pk, &ctxs, params.l_skip, params.n_stack);
    let t = Instant::now();
    let proof = engine.prove(&d_pk, ProvingContext::new(ctxs.into_iter().enumerate().collect()))
        .expect("node prove");
    let prove_s = t.elapsed().as_secs_f64();
    engine.verify(&vk, &proof).expect("node verify");
    let q: usize = params.whir.rounds.iter().map(|r| r.num_queries).sum();
    (vk, proof, json!({"arity": N, "params": params_json(&params), "total_queries": q,
                       "prove_s": prove_s, "verified": true, "shape": sh}))
}

/// (VmHWM, VmRSS) of this process in kB, from /proc/self/status.
fn mem_kb() -> (u64, u64) {
    let s = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let get = |k: &str| {
        s.lines()
            .find(|l| l.starts_with(k))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0)
    };
    (get("VmHWM:"), get("VmRSS:"))
}

/// Reset VmHWM to the current RSS (writing 5 to /proc/self/clear_refs).
fn reset_hwm() -> bool {
    std::fs::write("/proc/self/clear_refs", "5").is_ok()
}

/// D10 `L_cached` heights at the OD-19 sets (run internal_ud 2 20 1, proof_log_heights).
const D10_L_LOG_HEIGHTS: [usize; 39] = [
    15, 6, 4, 7, 5, 14, 2, 2, 2, 7, 12, 12, 3, 12, 15, 14, 6, 0, 8, 0, 5, 8, 0, 11, 3, 5, 9, 2, 11,
    2, 4, 9, 14, 12, 12, 8, 16, 5, 14,
];

/// L_cached: keygen, tracegen, prove, verify, negative controls, with the BLS12-381 Fr engine.
fn l_fr_run(
    engine: &RootEng,
    node_vk: &Arc<MultiStarkVerifyingKey<SC>>,
    node_proof: &Proof<SC>,
) -> Value {
    type C = RootSC;
    let (hwm_before_reset_kb, rss_before_kb) = mem_kb();
    let reset_ok = reset_hwm();
    let cfg = VerifierConfig { has_cached: true, ..Default::default() };
    let l = <VerifierSubCircuit<1> as VerifierTraceGen<CpuBackend<C>, C, ()>>::new(
        node_vk.clone(),
        cfg,
    );
    let airs = AggregationSubCircuit::airs::<C>(&l);
    let t = Instant::now();
    let (pk, vk) = engine.keygen(&airs);
    let keygen_s = t.elapsed().as_secs_f64();
    let cached = CachedTraceCtx::PcsData(l.commit_child_vk(engine, node_vk));
    let t = Instant::now();
    let ctxs = l.generate_proving_ctxs_base::<DuplexSpongeRecorder>(
        node_vk,
        cached,
        std::slice::from_ref(node_proof),
        &(),
        default_duplex_sponge_recorder(),
    );
    let tracegen_s = t.elapsed().as_secs_f64();
    let p = engine.params().clone();
    let sh = shape(&pk, &ctxs, p.l_skip, p.n_stack);
    let d_pk = engine.device().transport_pk_to_device(&pk);
    let (hwm_pre_prove_kb, rss_pre_prove_kb) = mem_kb();
    let t = Instant::now();
    let proof = engine
        .prove(&d_pk, ProvingContext::new(ctxs.into_iter().enumerate().collect()))
        .expect("L prove");
    let prove_s = t.elapsed().as_secs_f64();
    let (hwm_after_prove_kb, rss_after_prove_kb) = mem_kb();
    eprintln!("L prove done {prove_s:.1}s");

    let t = Instant::now();
    let ok = engine.verify(&vk, &proof);
    let verify_s = t.elapsed().as_secs_f64();
    eprintln!("L verify {:?} {verify_s:.3}s", ok.as_ref().map(|_| ()));
    let lh: Vec<Option<usize>> =
        proof.trace_vdata.iter().map(|v| v.as_ref().map(|v| v.log_height)).collect();
    let tracegen_lh: Vec<usize> =
        sh["airs"].as_array().unwrap().iter().map(|a| a["log_height"].as_u64().unwrap() as usize).collect();
    let proof_matches_tracegen =
        lh.iter().zip(&tracegen_lh).all(|(a, b)| *a == Some(*b)) && lh.len() == tracegen_lh.len();
    let matches_d10 = tracegen_lh == D10_L_LOG_HEIGHTS;

    // Proof bytes (stark-backend codec) and a decode round trip.
    let bytes = proof.encode_to_vec().expect("encode");
    let decoded = Proof::<C>::decode_from_bytes(&bytes).expect("decode");
    let t = Instant::now();
    let ok_dec = engine.verify(&vk, &decoded);
    let verify_decoded_s = t.elapsed().as_secs_f64();

    // Negative control 1: one flipped bit in one byte of the encoded proof, at several positions.
    let mut flips = vec![];
    for pos in [40usize, bytes.len() / 3, bytes.len() / 2, bytes.len() - 64] {
        let mut b = bytes.clone();
        b[pos] ^= 0x01;
        let (decode_ok, verify_res) = match Proof::<C>::decode_from_bytes(&b) {
            Ok(pf) => (true, Some(engine.verify(&vk, &pf))),
            Err(_) => (false, None),
        };
        let rejected = !decode_ok || verify_res.as_ref().is_some_and(|r| r.is_err());
        flips.push(json!({
            "byte_pos": pos, "decode_ok": decode_ok,
            "verify_err": verify_res.and_then(|r| r.err()).map(|e| format!("{e:?}")),
            "rejected": rejected,
        }));
    }
    // Negative control 2: one public value changed in memory.
    let mut pv_ctrl = json!(null);
    if let Some(ai) = proof.public_values.iter().position(|v| !v.is_empty()) {
        let mut bad = proof.clone();
        bad.public_values[ai][0] += F::ONE;
        let r = engine.verify(&vk, &bad);
        pv_ctrl = json!({"air_idx": ai, "air_name": pk.per_air[ai].air_name,
                         "rejected": r.is_err(), "verify_err": r.err().map(|e| format!("{e:?}"))});
    }

    let q: usize = p.whir.rounds.iter().map(|r| r.num_queries).sum();
    json!({
        "variant": "L_cached", "has_cached": true,
        "engine": "BabyBearBls12381PoseidonCpuEngine (Midnight Poseidon, BLS12-381 Fr, PROPOSAL 3.6 u=1,2,3)",
        "params": params_json(&p), "total_queries": q,
        "keygen_s": keygen_s, "tracegen_s": tracegen_s, "shape": sh,
        "prove_s": prove_s, "verify_s": verify_s,
        "verify_ok": ok.is_ok(), "verify_err": ok.err().map(|e| format!("{e:?}")),
        "verify_decoded_ok": ok_dec.is_ok(), "verify_decoded_s": verify_decoded_s,
        "proof_bytes": bytes.len(),
        "proof_log_heights": lh, "proof_heights_match_tracegen": proof_matches_tracegen,
        "heights_match_d10": matches_d10,
        "mem": {
            "hwm_whole_process_before_reset_kb": hwm_before_reset_kb,
            "rss_before_reset_kb": rss_before_kb, "hwm_reset_ok": reset_ok,
            "hwm_keygen_tracegen_kb": hwm_pre_prove_kb, "rss_pre_prove_kb": rss_pre_prove_kb,
            "hwm_l_keygen_tracegen_prove_kb": hwm_after_prove_kb,
            "rss_after_prove_kb": rss_after_prove_kb,
        },
        "neg_byte_flips": flips, "neg_public_value": pv_ctrl,
        "commit_digest_bytes": 32, "digest_example": format!("{:?}", proof.common_main_commit[0]),
    })
}

fn main() {
    let t0 = Instant::now();
    let (w1_vk, proofs) = w1_proofs(2, 20);
    eprintln!("w1 proofs done {:.1}s", t0.elapsed().as_secs_f64());
    let (node_vk, node_pf, node_json) = node_proof::<2>(internal_ud(), &w1_vk, &proofs);
    drop(proofs);
    eprintln!("node proof done {:.1}s", t0.elapsed().as_secs_f64());
    let node_vk = Arc::new(node_vk);
    let mut out = json!({"child_set": "internal_ud", "arity": 2, "w1_log_rows": 20,
                         "node": node_json,
                         "rayon_threads": std::env::var("RAYON_NUM_THREADS").ok()});
    let root = RootEng::new(l_ud(48));
    out["L_cached_bls12381_ud"] = l_fr_run(&root, &node_vk, &node_pf);
    out["wall_s"] = json!(t0.elapsed().as_secs_f64());
    let _ = std::marker::PhantomData::<Bls12381Fr>;
    println!("{}", serde_json::to_string(&out).unwrap());
}
