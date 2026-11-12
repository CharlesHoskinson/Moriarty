//! Keygen-only dump of the structure of the Sherlock last layer L (no tracegen, no proving).
//!
//! Chain: W1 (Poseidon2 AIR) keygen at app params -> node = VerifierSubCircuit::<2> over W1 vk
//! at leaf params (the bench node) -> L = VerifierSubCircuit::<1> over the node vk at root params.
//! L is keyed at root params (BabyBear engine; see note in main) and, for the C check, with both
//! VerifierConfig defaults (has_cached = true) and the RootProver setting (has_cached = false).
//!
//! Prints one JSON object on stdout: per-AIR structure for the node and for each L variant.

use std::{collections::BTreeMap, sync::Arc};

use openvm_poseidon2_air::{Poseidon2Config, Poseidon2SubChip};
use openvm_recursion_circuit::system::{AggregationSubCircuit, VerifierConfig, VerifierSubCircuit};
use openvm_stark_backend::{
    air_builders::symbolic::{symbolic_variable::Entry, SymbolicExpressionNode},
    keygen::types::MultiStarkProvingKey,
    AirRef, StarkEngine, StarkProtocolConfig, SystemParams,
};
use openvm_stark_sdk::config::{
    app_params_with_100_bits_security,
    baby_bear_poseidon2::{BabyBearPoseidon2Config, BabyBearPoseidon2CpuEngine, DuplexSponge},
    leaf_params_with_100_bits_security, root_params_with_100_bits_security,
    MAX_APP_LOG_STACKED_HEIGHT,
};
use p3_baby_bear::BabyBear;
use serde_json::{json, Value};

type F = BabyBear;
type SC = BabyBearPoseidon2Config;

fn params_json(p: &SystemParams) -> Value {
    json!({
        "l_skip": p.l_skip,
        "n_stack": p.n_stack,
        "w_stack": p.w_stack,
        "log_blowup": p.log_blowup,
        "k_whir": p.k_whir(),
        "num_whir_rounds": p.num_whir_rounds(),
        "whir_queries_per_round": p.whir.rounds.iter().map(|r| r.num_queries).collect::<Vec<_>>(),
        "log_final_poly_len": p.log_final_poly_len(),
        "final_poly_len": 1usize << p.log_final_poly_len(),
        "mu_pow_bits": p.whir.mu_pow_bits,
        "folding_pow_bits": p.whir.folding_pow_bits,
        "query_phase_pow_bits": p.whir.query_phase_pow_bits,
        "logup_pow_bits": p.logup.pow_bits,
        "max_constraint_degree": p.max_constraint_degree,
        "debug": format!("{:?}", p),
    })
}

fn dump<C: StarkProtocolConfig<F = F>>(pk: &MultiStarkProvingKey<C>) -> Value {
    let mut airs = vec![];
    for (i, apk) in pk.per_air.iter().enumerate() {
        let vk = &apk.vk;
        let w = &vk.params.width;
        let dag = &vk.symbolic_constraints.constraints;
        let nodes = &dag.nodes;
        let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
        let mut var_kinds: BTreeMap<&str, usize> = BTreeMap::new();
        let is_const = |j: usize| matches!(nodes[j], SymbolicExpressionNode::Constant(_));
        let mut mul_const = 0usize;
        let mut max_node_deg = 0usize;
        for n in nodes {
            let k = match n {
                SymbolicExpressionNode::Variable(v) => {
                    let vk = match v.entry {
                        Entry::Preprocessed { .. } => "preprocessed",
                        Entry::Main { .. } => "main",
                        Entry::Public => "public",
                        Entry::Challenge => "challenge",
                    };
                    *var_kinds.entry(vk).or_default() += 1;
                    "variable"
                }
                SymbolicExpressionNode::IsFirstRow => "is_first_row",
                SymbolicExpressionNode::IsLastRow => "is_last_row",
                SymbolicExpressionNode::IsTransition => "is_transition",
                SymbolicExpressionNode::Constant(_) => "constant",
                SymbolicExpressionNode::Add { degree_multiple, .. } => {
                    max_node_deg = max_node_deg.max(*degree_multiple);
                    "add"
                }
                SymbolicExpressionNode::Sub { degree_multiple, .. } => {
                    max_node_deg = max_node_deg.max(*degree_multiple);
                    "sub"
                }
                SymbolicExpressionNode::Neg { degree_multiple, .. } => {
                    max_node_deg = max_node_deg.max(*degree_multiple);
                    "neg"
                }
                SymbolicExpressionNode::Mul { left_idx, right_idx, degree_multiple } => {
                    max_node_deg = max_node_deg.max(*degree_multiple);
                    if is_const(*left_idx) || is_const(*right_idx) {
                        mul_const += 1;
                    }
                    "mul"
                }
            };
            *kinds.entry(k).or_default() += 1;
        }
        let ints = &vk.symbolic_constraints.interactions;
        let mut buses: Vec<u16> = ints.iter().map(|it| it.bus_index as u16).collect();
        buses.sort();
        buses.dedup();
        let msg_lens: Vec<usize> = ints.iter().map(|it| it.message.len()).collect();
        airs.push(json!({
            "idx": i,
            "name": apk.air_name,
            "width_preprocessed": w.preprocessed.unwrap_or(0),
            "width_cached_mains": w.cached_mains,
            "width_common_main": w.common_main,
            "width_main_total": w.main_width(),
            "width_after_challenge": 0,
            "has_preprocessed_data": vk.preprocessed_data.is_some(),
            "num_public_values": vk.params.num_public_values,
            "need_rot": vk.params.need_rot,
            "is_required": vk.is_required,
            "max_constraint_degree": vk.max_constraint_degree,
            "max_dag_node_degree": max_node_deg,
            "num_constraints": dag.constraint_idx.len(),
            "dag_nodes": nodes.len(),
            "dag_node_kinds": kinds,
            "dag_variable_kinds": var_kinds,
            "dag_mul_by_constant": mul_const,
            "num_interactions": ints.len(),
            "interaction_msg_lens": msg_lens,
            "interaction_msg_len_sum": msg_lens.iter().sum::<usize>(),
            "bus_ids": buses,
        }));
    }
    json!({
        "num_airs": pk.per_air.len(),
        "params": params_json(&pk.params),
        "max_constraint_degree": pk.max_constraint_degree,
        "num_trace_height_constraints": pk.trace_height_constraints.len(),
        "airs": airs,
    })
}

fn main() {
    // W1 child: Poseidon2 width-16 AIR, app params (the bench leaf).
    let chip = Poseidon2SubChip::<F, 1>::new(Poseidon2Config::<F>::default().constants);
    let w1_airs: Vec<AirRef<SC>> = vec![chip.air.clone()];
    let app = BabyBearPoseidon2CpuEngine::<DuplexSponge>::new(app_params_with_100_bits_security(
        MAX_APP_LOG_STACKED_HEIGHT,
    ));
    let (_, w1_vk) = app.keygen(&w1_airs);

    // Bench node: VerifierSubCircuit::<2> over W1 at leaf params, default config.
    let node = VerifierSubCircuit::<2>::new(Arc::new(w1_vk));
    let node_airs = node.airs::<SC>();
    let leaf = BabyBearPoseidon2CpuEngine::<DuplexSponge>::new(leaf_params_with_100_bits_security());
    let (node_pk, node_vk) = leaf.keygen(&node_airs);
    let node_json = dump(&node_pk);
    eprintln!("node: {} AIRs", node_pk.per_air.len());

    let node_vk = Arc::new(node_vk);
    let mut out = json!({ "node": node_json });
    for (tag, has_cached) in [("L_cached", true), ("L_nocached", false)] {
        let cfg = VerifierConfig { has_cached, ..Default::default() };
        let l = VerifierSubCircuit::<1>::new_with_options(node_vk.clone(), cfg);
        // Root params. The BN254 root config needs the stark-sdk feature baby-bear-bn254-poseidon2,
        // which the bench crate does not enable; the AIRs of L do not depend on L's own hash
        // (airs::<SC>() only fixes the config type), so the BabyBear engine keys the same structure.
        let root = BabyBearPoseidon2CpuEngine::<DuplexSponge>::new(root_params_with_100_bits_security());
        let l_airs = l.airs::<SC>();
        let (l_pk, _) = root.keygen(&l_airs);
        eprintln!("{tag}: {} AIRs", l_pk.per_air.len());
        out[tag] = dump(&l_pk);
        out[tag]["verifier_config"] =
            json!({"has_cached": has_cached, "continuations_enabled": false,
                   "final_state_bus_enabled": false, "engine": "BabyBearPoseidon2CpuEngine (root params; BN254 config not built)"});
    }
    println!("{}", serde_json::to_string(&out).unwrap());
}
