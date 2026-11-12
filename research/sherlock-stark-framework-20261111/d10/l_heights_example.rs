//! D10: trace heights of the Sherlock last layer L by tracegen over one real node proof.
//!
//! Usage: l_heights <child_set> <arity> <w1_log_rows> <prove_l>
//!   child_set : "internal_ud" (OD-19 internal set: UD, blowup 3, queries 97/82/81, fold PoW 0,
//!               mu PoW 9, query PoW 21, LogUp PoW 18, l_skip 2, n_stack 17, w_stack 512)
//!               or "leaf" (stock leaf params, the bench node).
//!   arity     : 1 or 2 (VerifierSubCircuit::<arity> over W1 proofs at app params).
//!   prove_l   : 0 or 1. 1 proves and verifies L_nocached and L_cached with the BN254 root engine
//!               at the OD-19 L set (UD, blowup 3, queries 98/83/82, fold PoW 0, mu PoW 6,
//!               query PoW 20, LogUp PoW 18, l_skip 2, n_stack 18) with w_stack 48.
//!
//! Chain: W1 proofs (CPU, app params) -> node proof (CPU, child_set params, verified) ->
//! L = VerifierSubCircuit::<1> over the node vk: tracegen with the BabyBear engine and with the
//! BN254 root engine, both VerifierConfig variants (has_cached true and false). Per-AIR heights
//! and widths, the stacked width per commit by StackedLayout::new (as proof_shape.rs:540-573),
//! the total interactions and n_logup (proof_shape.rs:378-384), and the BN254 keygen dump.
//! Prints one JSON object on stdout.

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
use openvm_stark_sdk::config::{
    app_params_with_100_bits_security,
    baby_bear_bn254_poseidon2::{BabyBearBn254Poseidon2Config, BabyBearBn254Poseidon2CpuEngine},
    baby_bear_poseidon2::{
        default_duplex_sponge_recorder, BabyBearPoseidon2Config, BabyBearPoseidon2CpuEngine,
        DuplexSponge, DuplexSpongeRecorder,
    },
    leaf_params_with_100_bits_security, params_with_100_bits_security, MAX_APP_LOG_STACKED_HEIGHT,
};
use p3_baby_bear::BabyBear;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};

type F = BabyBear;
type SC = BabyBearPoseidon2Config;
type RootSC = BabyBearBn254Poseidon2Config;
type Eng = BabyBearPoseidon2CpuEngine<DuplexSponge>;
type RootEng = BabyBearBn254Poseidon2CpuEngine;

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

/// L tracegen (and optionally prove + verify) for one engine and one VerifierConfig.
fn l_run<E, C>(
    engine: &E,
    node_vk: &Arc<MultiStarkVerifyingKey<SC>>,
    node_proof: &Proof<SC>,
    has_cached: bool,
    prove: bool,
) -> (Value, MultiStarkProvingKey<C>)
where
    C: StarkProtocolConfig<F = F>,
    E: StarkEngine<SC = C, PB = CpuBackend<C>>,
    (): From<openvm_stark_backend::EngineDeviceCtx<E>>,
    VerifierSubCircuit<1>: VerifierTraceGen<CpuBackend<C>, C, ()>,
{
    let cfg = VerifierConfig { has_cached, ..Default::default() };
    let l = <VerifierSubCircuit<1> as VerifierTraceGen<CpuBackend<C>, C, ()>>::new(
        node_vk.clone(),
        cfg,
    );
    let airs = AggregationSubCircuit::airs::<C>(&l);
    let (pk, vk) = engine.keygen(&airs);
    let cached = if has_cached {
        CachedTraceCtx::PcsData(l.commit_child_vk(engine, node_vk))
    } else {
        CachedTraceCtx::Records(
            <VerifierSubCircuit<1> as VerifierTraceGen<CpuBackend<C>, C, ()>>::cached_trace_record(
                &l, node_vk,
            ),
        )
    };
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
    let mut out = json!({"has_cached": has_cached, "tracegen_s": tracegen_s,
                         "shape": shape(&pk, &ctxs, p.l_skip, p.n_stack)});
    if prove {
        let d_pk = engine.device().transport_pk_to_device(&pk);
        let t = Instant::now();
        let proof = engine
            .prove(&d_pk, ProvingContext::new(ctxs.into_iter().enumerate().collect()))
            .expect("L prove");
        let prove_s = t.elapsed().as_secs_f64();
        let ok = engine.verify(&vk, &proof);
        let lh: Vec<Option<usize>> =
            proof.trace_vdata.iter().map(|v| v.as_ref().map(|v| v.log_height)).collect();
        out["prove"] = json!({"prove_s": prove_s, "verify_ok": ok.is_ok(),
                              "verify_err": ok.err().map(|e| format!("{e:?}")),
                              "proof_log_heights": lh, "params": params_json(&p)});
    }
    (out, pk)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let set = a[1].as_str();
    let arity: usize = a[2].parse().unwrap();
    let log_rows: usize = a[3].parse().unwrap();
    let prove_l = a[4] == "1";
    let t0 = Instant::now();
    let child_params = match set {
        "internal_ud" => internal_ud(),
        "leaf" => leaf_params_with_100_bits_security(),
        _ => panic!("child_set"),
    };
    let (w1_vk, proofs) = w1_proofs(arity, log_rows);
    eprintln!("w1 proofs done {:.1}s", t0.elapsed().as_secs_f64());
    let (node_vk, node_pf, node_json) = match arity {
        1 => node_proof::<1>(child_params, &w1_vk, &proofs),
        2 => node_proof::<2>(child_params, &w1_vk, &proofs),
        _ => panic!("arity"),
    };
    eprintln!("node proof done {:.1}s", t0.elapsed().as_secs_f64());
    let node_vk = Arc::new(node_vk);
    let mut out = json!({"child_set": set, "arity": arity, "w1_log_rows": log_rows, "node": node_json});

    // BabyBear engine at the stock root params (as L-STRUCTURE) and BN254 root engine at the
    // OD-19 L set. Heights of L do not depend on L's own params; both are dumped to show it.
    let bb = Eng::new(openvm_stark_sdk::config::root_params_with_100_bits_security());
    let root = RootEng::new(l_ud(48));
    for has_cached in [true, false] {
        let tag = if has_cached { "L_cached" } else { "L_nocached" };
        let (bb_out, _) = l_run(&bb, &node_vk, &node_pf, has_cached, false);
        out[format!("{tag}_babybear_rootparams")] = bb_out;
        let (bn_out, bn_pk) = l_run(&root, &node_vk, &node_pf, has_cached, prove_l);
        out[format!("{tag}_bn254_ud")] = bn_out;
        out[format!("{tag}_bn254_ud_keygen")] = dump(&bn_pk);
        eprintln!("{tag} done {:.1}s", t0.elapsed().as_secs_f64());
    }
    out["wall_s"] = json!(t0.elapsed().as_secs_f64());
    println!("{}", serde_json::to_string(&out).unwrap());
}
