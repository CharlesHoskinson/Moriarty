//! Task 4.1 (M1', SRC-013) row probes. Modeling only: `IrSource::model()`
//! (the stdlib `cost_model`), no SRS, no keygen, no proof.
//!
//! * `m1_lanes_and_typical_fragment_rows`: lanes per side of the C6 step
//!   statement at the dispatch qualification config, and rows and k of three
//!   typical fragments, raw (harness statement) and as digest leaves
//!   (`c6_child_relation`).
//! * `k_u_universal_leaf_row_probe`: the S=1 universal gated leaf over every
//!   C6 fragment that exists today: the dispatch facts once, every supported
//!   rule's selected bit `sel_r = enable * path_r`, `sum(sel_r over the rules
//!   with a fragment) == enable`, every fragment gated by its `sel_r` over one
//!   shared pre/post statement (the post META/PROG/CTX lanes are the shared
//!   prefix), wrapped as a digest leaf. This is a probe relation, not the 4.2
//!   leaf: post-lane writes are each fragment's own gated constraints (no
//!   candidate/select multiplexer), and there is no `rules_d`/`steps`.
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::time::Instant;

use midnight_transient_crypto::curve::Fr;
use midnight_zkir_v3::ir::{IrMinorVersion, Operand};
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets;
use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostConfig, C6HostLimits, CTX_LANES};
use mori_core::sherlock::helper_machine::complete_observation::{HostSnapshot, capture};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::{OwnerEngine, RuleIdC, StepResult};
use mori_core::sherlock_compile_artifact::draft_execution::draft_genesis;
use serde_json::{Value, json};

use super::c6_dispatch_step::partition::emit_selector;
use super::c6_dispatch_step::{C6DispatchInputs, supported_rules};
use super::c6_push_step::C6PushStatementBindingMode::LocalPublic;
use super::c6_step_harness::{C6StepWitness, step_witness_from_host_pair};
use crate::sherlock_c6_child::{DigestSections, StateOutcome, c6_child_relation};

const CONFIG: &str = include_str!("testdata/dispatch/config.json");
const FACT: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } }\n  entry fact(3);\n}\n";

fn limits() -> C6HostLimits {
    C6HostLimits {
        max_total_lanes: 1 << 20,
        max_state_lanes: 1 << 20,
        max_metadata_bytes: 8192,
        max_string_bytes: 512,
        max_width_entries: 64,
    }
}

fn first_step() -> C6StepWitness {
    let g = draft_genesis(FACT, CONFIG)
        .unwrap_or_else(|f| panic!("genesis: {}", serde_json::to_string(&f).unwrap()));
    let cfg = C6HostConfig::from_retained_layout(
        &g.layout,
        C6CodecBudgets {
            type_expr_nodes: 128,
            template_bytes: 4096,
        },
    );
    let engine = host_owner_engine();
    let mut image = g.image;
    let pre: HostSnapshot = capture(&g.admitted, &image).expect("capture");
    match engine.step(&g.admitted, &mut image).expect("owner step") {
        StepResult::Terminal => panic!("no step"),
        StepResult::Event { .. } => {}
    }
    let post = capture(&g.admitted, &image).expect("capture");
    step_witness_from_host_pair(true, &pre, &post, &cfg, &limits()).expect("witness")
}

type Build = Box<dyn Fn(&C6StepWitness, &str) -> Result<IrSource, String>>;

/// Every C6 fragment that exists today, one per rule.
fn fragments() -> Vec<(RuleIdC, &'static str, Build)> {
    use super::*;
    let mut out: Vec<(RuleIdC, &'static str, Build)> = Vec::new();
    macro_rules! kit {
        ($rules:expr, $file:literal, $build:path) => {
            for rule in $rules {
                out.push((
                    rule,
                    $file,
                    Box::new(move |w: &C6StepWitness, scope: &str| {
                        $build(
                            rule,
                            w.source_params.clone(),
                            w.host_prefix.meta.len(),
                            w.host_prefix.prog.len(),
                            w.host_prefix.meta_bounds_frames_offset,
                            scope,
                        )
                        .map(|c| c.ir().clone())
                    }),
                ));
            }
        };
    }
    macro_rules! single {
        ($rule:expr, $file:literal, $build:path) => {
            out.push((
                $rule,
                $file,
                Box::new(move |w: &C6StepWitness, scope: &str| {
                    $build(
                        w.source_params.clone(),
                        w.host_prefix.meta.len(),
                        w.host_prefix.prog.len(),
                        w.host_prefix.meta_bounds_frames_offset,
                        LocalPublic,
                        scope,
                    )
                    .map(|c| c.ir().clone())
                    .map_err(|e| e.to_string())
                }),
            ));
        };
    }
    kit!(
        c6_admin_publish_step::ADMIN_PUBLISH_RULES,
        "c6_admin_publish",
        c6_admin_publish_consumer::build_c6_admin_publish_circuit
    );
    kit!(
        c6_eval_schedule_step::EVAL_SCHEDULE_RULES,
        "c6_eval_schedule",
        c6_eval_schedule_consumer::build_c6_eval_schedule_circuit
    );
    kit!(
        c6_legacy_control_step::LEGACY_CONTROL_RULES,
        "c6_legacy_control",
        c6_legacy_control_consumer::build_c6_legacy_control_circuit
    );
    kit!(
        c6_call_bind_step::CALL_BIND_RULES,
        "c6_call_bind",
        c6_call_bind_consumer::build_c6_call_bind_circuit
    );
    kit!(
        c6_call_helper_step::CALL_HELPER_RULES,
        "c6_call_helper",
        c6_call_helper_consumer::build_c6_call_helper_circuit
    );
    kit!(
        c6_bind_helper_step::BIND_HELPER_RULES,
        "c6_bind_helper",
        c6_bind_helper_consumer::build_c6_bind_helper_circuit
    );
    kit!(
        c6_append_helper_step::APPEND_HELPER_RULES,
        "c6_append_helper",
        c6_append_helper_consumer::build_c6_append_helper_circuit
    );
    kit!(
        c6_validate_helper_step::VALIDATE_HELPER_RULES,
        "c6_validate_helper",
        c6_validate_helper_consumer::build_c6_validate_helper_circuit
    );
    kit!(
        c6_equal_helper_step::EQUAL_HELPER_RULES,
        "c6_equal_helper",
        c6_equal_helper_consumer::build_c6_equal_helper_circuit
    );
    kit!(
        c6_match_helper_step::MATCH_HELPER_RULES,
        "c6_match_helper",
        c6_match_helper_consumer::build_c6_match_helper_circuit
    );
    kit!(
        c6_scalar_construct_step::SCALAR_CONSTRUCT_RULES,
        "c6_scalar_construct",
        c6_scalar_construct_consumer::build_c6_scalar_construct_circuit
    );
    single!(
        RuleIdC::BHelperScalarEntry,
        "c6_scalar_entry",
        c6_scalar_entry_consumer::build_c6_scalar_entry_step_circuit
    );
    single!(
        RuleIdC::BHelperScalarTag,
        "c6_scalar_tag",
        c6_scalar_tag_consumer::build_c6_scalar_tag_step_circuit
    );
    single!(
        RuleIdC::BHelperScalarArithmetic,
        "c6_scalar_arithmetic",
        c6_scalar_arithmetic_consumer::build_c6_scalar_arithmetic_step_circuit
    );
    single!(
        RuleIdC::BHelperScalarExit,
        "c6_scalar_exit",
        c6_scalar_exit_consumer::build_c6_scalar_exit_step_circuit
    );
    single!(
        RuleIdC::BHelperScalarFailure,
        "c6_scalar_failure",
        c6_scalar_failure_step::build_c6_scalar_failure_step_circuit
    );
    single!(
        RuleIdC::BHelperEqualEmpty,
        "c6_equal_empty",
        c6_equal_empty_step::build_c6_equal_empty_step_circuit
    );
    single!(
        RuleIdC::BHelperValidateField,
        "c6_validate_field",
        c6_validate_field_consumer::build_c6_validate_field_step_circuit
    );
    single!(
        RuleIdC::KMachineReturnLeft,
        "c6_return_left",
        c6_return_left_consumer::build_c6_return_left_step_circuit
    );
    single!(
        RuleIdC::KMachineReturnRight,
        "c6_return_right",
        c6_return_right_consumer::build_c6_return_right_step_circuit
    );
    single!(
        RuleIdC::KMachineReturnScope,
        "c6_return_scope",
        c6_return_scope_consumer::build_c6_return_scope_step_circuit
    );
    single!(
        RuleIdC::KMachineReturnCallValid,
        "c6_return_call",
        c6_return_call_consumer::build_c6_return_call_step_circuit
    );
    single!(
        RuleIdC::XAdtEvalConstructNullary,
        "c6_construct_nullary",
        c6_construct_nullary_consumer::build_c6_construct_nullary_step_circuit
    );
    single!(
        RuleIdC::KMachinePushAccept,
        "c6_push",
        c6_push_step::build_c6_push_step_circuit
    );
    // One fragment per rule: the first listed wins.
    let mut seen = Vec::new();
    out.retain(|(r, _, _)| {
        let k = format!("{r:?}");
        if seen.contains(&k) {
            false
        } else {
            seen.push(k);
            true
        }
    });
    out
}

/// Statement geometry of the harness layout.
struct Geo {
    prefix: usize,
    state: usize,
    mpc: usize,
}

fn geo(w: &C6StepWitness) -> Geo {
    let mpc = w.host_prefix.meta.len() + w.host_prefix.prog.len() + CTX_LANES;
    Geo {
        prefix: 3 + mpc,
        state: w.pre_image.len() + w.pre_data.len() + w.pre_helper.len(),
        mpc,
    }
}

fn load(ins: Vec<Instruction>) -> IrSource {
    let ir0 = IrSource {
        version: IrMinorVersion::V0,
        inputs: Vec::new(),
        outputs: Vec::new(),
        do_communications_commitment: false,
        instructions: Arc::new(ins),
    };
    let mut disk = serde_json::to_value(&ir0).unwrap();
    disk["version"] = json!({ "major": 3, "minor": 0 });
    IrSource::load(serde_json::to_string(&disk).unwrap().as_bytes()).expect("load")
}

fn sections(g: &Geo) -> DigestSections {
    DigestSections {
        prog_lanes: g.prefix,
        pre_lanes: g.state,
        post_lanes: g.state,
    }
}

/// Re-express a harness-shaped fragment on the shared digest-leaf statement:
/// harness position j maps to a shared operand (position 0 to `enable`), the
/// post META/PROG/CTX lanes to the shared prefix, every other identifier is
/// renamed under `%frag{n}.`.
fn rebase(
    ir: &IrSource,
    g: &Geo,
    shared: &[Identifier],
    enable: &Identifier,
    n: usize,
) -> Vec<Instruction> {
    let mut publics = Vec::new();
    let mut body = Vec::new();
    let mut impacts = 0;
    for insn in ir.instructions.iter() {
        match insn {
            Instruction::PublicInput {
                guard: None,
                output,
                ..
            } => publics.push(output.clone()),
            Instruction::PublicInput { .. } => panic!("guarded public input"),
            Instruction::Impact { .. } => impacts += 1,
            Instruction::PrivateInput { .. } => panic!("private input in a harness fragment"),
            other => body.push(other.clone()),
        }
    }
    assert_eq!(impacts, 1, "one publication");
    let harness = g.prefix + 2 * g.state + g.mpc;
    assert_eq!(publics.len(), harness, "harness statement length");
    let post_prefix = g.prefix + g.state;
    let post_state = post_prefix + g.mpc;
    let mut map: HashMap<String, String> = HashMap::new();
    for (j, id) in publics.iter().enumerate() {
        let to = if j == 0 {
            enable.clone()
        } else if j < post_prefix {
            shared[j].clone()
        } else if j < post_state {
            shared[3 + (j - post_prefix)].clone()
        } else {
            shared[post_prefix + (j - post_state)].clone()
        };
        map.insert(id.0.clone(), to.0);
    }
    fn walk(v: &mut Value, map: &HashMap<String, String>, n: usize) {
        match v {
            Value::String(s) if s.starts_with('%') => {
                *s = match map.get(s.as_str()) {
                    Some(t) => t.clone(),
                    None => format!("%frag{n}.{}", &s[1..]),
                };
            }
            Value::Array(a) => a.iter_mut().for_each(|x| walk(x, map, n)),
            Value::Object(o) => o.values_mut().for_each(|x| walk(x, map, n)),
            _ => {}
        }
    }
    body.into_iter()
        .map(|insn| {
            let mut v = serde_json::to_value(&insn).unwrap();
            walk(&mut v, &map, n);
            serde_json::from_value(v).expect("instruction round trip")
        })
        .collect()
}

fn model(ir: &IrSource) -> (u8, usize, f64) {
    let t = Instant::now();
    let m = ir.model();
    (m.k(), m.rows(), t.elapsed().as_secs_f64())
}

fn digest_leaf(ir: &IrSource, g: &Geo) -> IrSource {
    c6_child_relation(
        ir,
        sections(g),
        StateOutcome::RUNNING,
        StateOutcome::RUNNING,
    )
    .expect("digest relation")
}

/// A harness fragment on the digest-leaf statement (shared prefix).
fn single_leaf(ir: &IrSource, g: &Geo) -> IrSource {
    let n = g.prefix + 2 * g.state;
    let shared: Vec<Identifier> = (0..n)
        .map(|j| Identifier(format!("%u.public.{j}")))
        .collect();
    let mut ins: Vec<Instruction> = shared
        .iter()
        .map(|id| Instruction::PublicInput {
            guard: None,
            val_t: IrType::Native,
            output: id.clone(),
        })
        .collect();
    ins.extend(rebase(ir, g, &shared, &shared[0], 0));
    ins.push(Instruction::Impact {
        guard: Operand::Immediate(Fr::from(1u64)),
        inputs: shared
            .iter()
            .map(|id| Operand::Variable(id.clone()))
            .collect(),
    });
    digest_leaf(&load(ins), g)
}

#[test]
#[ignore = "task 4.1 row probe (cost_model, no SRS); run under doctorwatson heavy"]
fn m1_lanes_and_typical_fragment_rows() {
    let w = first_step();
    let g = geo(&w);
    let lanes = json!({
        "config": "crates/mori-backend/src/sherlock_machine/testdata/dispatch/config.json",
        "prefix_lanes": g.prefix,
        "meta_lanes": w.host_prefix.meta.len(),
        "prog_lanes": w.host_prefix.prog.len(),
        "ctx_lanes": CTX_LANES,
        "per_side": {
            "image": w.pre_image.len(),
            "data": w.pre_data.len(),
            "helper": w.pre_helper.len(),
            "total": g.state,
        },
        "harness_statement_lanes": w.to_statement_frs().len(),
        "digest_leaf_private_lanes": g.prefix + 2 * g.state,
    });
    let mut rows = BTreeMap::new();
    let frags = fragments();
    for want in [
        RuleIdC::KMachineEvalBinary,
        RuleIdC::KMachineReturnRight,
        RuleIdC::BHelperMatchArm,
    ] {
        let (rule, file, build) = frags.iter().find(|(r, _, _)| *r == want).expect("fragment");
        let ir = build(&w, &format!("typ{}", format!("{rule:?}").to_lowercase())).expect("build");
        let (k, r, s) = model(&ir);
        let (lk, lr, ls) = model(&single_leaf(&ir, &g));
        rows.insert(
            format!("{rule:?}"),
            json!({"file": file, "instructions": ir.instructions.len(), "raw": {"k": k, "rows": r, "seconds": s}, "digest_leaf": {"k": lk, "rows": lr, "seconds": ls}}),
        );
    }
    println!("M1-PRIME {}", json!({"lanes": lanes, "fragments": rows}));
}

/// The probe leaf: shared statement, the dispatch selector once,
/// `sel_r = enable * path_r`, `sum(sel_r over sum_rules) == enable`, and the
/// fragments `frags` gated by their `sel_r`, as a digest leaf.
fn universal(
    w: &C6StepWitness,
    g: &Geo,
    frags: &[(RuleIdC, &'static str, Build)],
    sum_rules: &[RuleIdC],
) -> (IrSource, Value) {
    let n = g.prefix + 2 * g.state;
    let shared: Vec<Identifier> = (0..n)
        .map(|j| Identifier(format!("%u.public.{j}")))
        .collect();
    let op = |j: usize| Operand::Variable(shared[j].clone());
    let mut ins: Vec<Instruction> = shared
        .iter()
        .map(|id| Instruction::PublicInput {
            guard: None,
            val_t: IrType::Native,
            output: id.clone(),
        })
        .collect();
    let meta = w.host_prefix.meta.len();
    let prog = w.host_prefix.prog.len();
    let (img, data) = (w.pre_image.len(), w.pre_data.len());
    let inputs = C6DispatchInputs {
        enable: op(0),
        caps: w.source_params.geometry().caps,
        pre_prog: (3 + meta..3 + meta + prog).map(op).collect(),
        pre_image: (g.prefix..g.prefix + img).map(op).collect(),
        pre_data: (g.prefix + img..g.prefix + img + data).map(op).collect(),
        pre_helper: (g.prefix + img + data..g.prefix + g.state)
            .map(op)
            .collect(),
    };
    let sel = emit_selector(&inputs, "udispatch").expect("selector");
    let selector_instructions = sel.ins.len();
    ins.extend(sel.ins.iter().cloned());
    let mut sel_of: HashMap<String, Identifier> = HashMap::new();
    for (rule, p) in &sel.paths {
        let id = Identifier(format!("%u.sel.{rule:?}"));
        ins.push(Instruction::Mul {
            a: op(0),
            b: p.clone(),
            output: id.clone(),
        });
        sel_of.insert(format!("{rule:?}"), id);
    }
    let mut acc = Operand::Immediate(Fr::from(0u64));
    for (i, rule) in sum_rules.iter().enumerate() {
        let id = Identifier(format!("%u.selsum.{i}"));
        ins.push(Instruction::Add {
            a: acc,
            b: Operand::Variable(sel_of[&format!("{rule:?}")].clone()),
            output: id.clone(),
        });
        acc = Operand::Variable(id);
    }
    ins.push(Instruction::ConstrainEq { a: acc, b: op(0) });
    let mut per_fragment = BTreeMap::new();
    let mut fragment_instructions = 0usize;
    for (i, (rule, file, build)) in frags.iter().enumerate() {
        let ir = build(w, &format!("uf{i}")).unwrap_or_else(|e| panic!("{rule:?}: {e}"));
        fragment_instructions += ir.instructions.len();
        per_fragment.insert(
            format!("{rule:?}"),
            json!({"file": file, "instructions": ir.instructions.len()}),
        );
        ins.extend(rebase(&ir, g, &shared, &sel_of[&format!("{rule:?}")], i));
    }
    ins.push(Instruction::Impact {
        guard: Operand::Immediate(Fr::from(1u64)),
        inputs: shared
            .iter()
            .map(|id| Operand::Variable(id.clone()))
            .collect(),
    });
    let stats = json!({
        "selector_instructions": selector_instructions,
        "fragment_instructions": fragment_instructions,
        "total_instructions": ins.len(),
        "statement_private_lanes": n,
        "per_fragment": per_fragment,
    });
    (load(ins), stats)
}

/// S under gate G-A' as task 4.10 words it, and the bare inequality.
fn s_reading(k_u: u8) -> Value {
    let bare = (0..=4u32)
        .rev()
        .find(|e| u32::from(k_u) + e <= 20)
        .map(|e| 1u32 << e);
    let rule = match k_u {
        0..=17 => "S=8 (k_U <= 17); S=16 needs k_U <= 16",
        18 => "S=4 (k_U = 18)",
        _ => "k_U > 17 at S=1: option F is on the table and fixes S=1 (SRC-013)",
    };
    json!({
        "k_u_plus_log2_s_le_20_max_s": bare,
        "task_4_10_reading": rule,
        "node_s_max_16_needs_k_u_le": 16,
    })
}

#[test]
#[ignore = "task 4.1 k_U probe (cost_model, no SRS); run alone under doctorwatson heavy"]
fn k_u_universal_leaf_row_probe() {
    let w = first_step();
    let g = geo(&w);
    let frags = fragments();
    let all = supported_rules();
    // Selector-only leaf (no fragment), then the full probe.
    let sel_leaf = digest_leaf(&universal(&w, &g, &[], &all).0, &g);
    let (sk, sr, ss) = model(&sel_leaf);
    drop(sel_leaf);
    let mut per_leaf = BTreeMap::new();
    if std::env::var_os("K_U_PER_FRAGMENT").is_some() {
        for (i, (rule, file, build)) in frags.iter().enumerate() {
            let ir = build(&w, &format!("pf{i}")).expect("build");
            let (k, r, s) = model(&single_leaf(&ir, &g));
            per_leaf.insert(
                format!("{rule:?}"),
                json!({"file": file, "k": k, "rows": r, "seconds": s}),
            );
        }
    }
    // Two-sided bound without modelling the full leaf: the leaf contains the
    // selector-only leaf (lower bound on rows), and its instruction count is
    // the selector plus every fragment (rows estimated from instructions).
    if std::env::var_os("K_U_COUNT_ONLY").is_some() {
        let mut counts = BTreeMap::new();
        let mut total = 0usize;
        for (i, (rule, file, build)) in frags.iter().enumerate() {
            let n = build(&w, &format!("cf{i}"))
                .expect("build")
                .instructions
                .len();
            total += n;
            counts.insert(
                format!("{rule:?}"),
                json!({"file": file, "instructions": n}),
            );
        }
        println!(
            "K-U-COUNT {}",
            json!({
                "selector_only_leaf": {"k": sk, "rows": sr, "seconds": ss},
                "fragments": frags.len(),
                "fragment_instructions": total,
                "per_fragment": counts,
                "per_fragment_leaf": per_leaf,
            })
        );
        return;
    }
    let with: Vec<RuleIdC> = frags.iter().map(|(r, _, _)| *r).collect();
    let (raw, stats) = universal(&w, &g, &frags, &with);
    let leaf = digest_leaf(&raw, &g);
    drop(raw);
    let (k_u, rows, secs) = model(&leaf);
    let missing: Vec<String> = all
        .iter()
        .filter(|r| !with.contains(r))
        .map(|r| format!("{r:?}"))
        .collect();
    println!(
        "K-U {}",
        json!({
            "relation": "probe: S=1 universal gated digest leaf over every existing C6 fragment (not the 4.2 relation; each fragment keeps its own prefix/canonicality checks, so shared checks are duplicated)",
            "k_u": k_u,
            "rows": rows,
            "model_seconds": secs,
            "selector_only_leaf": {"k": sk, "rows": sr, "seconds": ss},
            "fragments": frags.len(),
            "supported_rules": all.len(),
            "rules_without_fragment": missing,
            "s": s_reading(k_u),
            "stats": stats,
            "per_fragment_leaf": per_leaf,
        })
    );
}

/// Full `cost_model` record of one relation (task 4.1a): k, rows, table rows,
/// columns and the stdlib chip set (`Relation::used_chips`).
fn model_full(ir: &IrSource) -> Value {
    use midnight_zk_stdlib::Relation;
    let t = Instant::now();
    let m = midnight_zk_stdlib::cost_model(ir, None);
    json!({
        "k": m.k,
        "rows": m.rows,
        "table_rows": m.table_rows,
        "nb_unusable_rows": m.nb_unusable_rows,
        "max_deg": m.max_deg,
        "advice_columns": m.advice_columns,
        "fixed_columns": m.fixed_columns,
        "lookups": m.lookups,
        "permutations": m.permutations,
        "arch": format!("{:?}", ir.used_chips()),
        "instructions": ir.instructions.len(),
        "seconds": t.elapsed().as_secs_f64(),
    })
}

/// Task 4.1a diagnostic: `cost_model` of every per-rule C6 circuit that
/// exists today (raw, on its own harness statement) and of the dispatch block
/// alone (shared statement, dispatch facts and every `sel_r`, `sum == enable`),
/// at the committed qualification config `testdata/dispatch/config.json` and
/// the fact(3) first step. One `ROW-PROBE` line per circuit, printed as it is
/// measured. `ROW_PROBE_FILTER=<file substring>` limits the circuits;
/// `ROW_PROBE_NO_DISPATCH` skips the dispatch block. A projection, not a bound.
#[test]
#[ignore = "task 4.1a row probe (cost_model, no SRS); hours; run alone under doctorwatson heavy"]
fn row_probe_every_c6_circuit() {
    use std::io::Write as _;
    let w = first_step();
    let g = geo(&w);
    let emit = |v: Value| {
        println!("ROW-PROBE {v}");
        std::io::stdout().flush().ok();
    };
    if std::env::var_os("ROW_PROBE_NO_DISPATCH").is_none() {
        let raw = universal(&w, &g, &[], &supported_rules()).0;
        emit(
            json!({"circuit": "dispatch_block", "file": "c6_dispatch_step", "model": model_full(&raw)}),
        );
    }
    let filter = std::env::var("ROW_PROBE_FILTER").unwrap_or_default();
    for (i, (rule, file, build)) in fragments().iter().enumerate() {
        if !file.contains(filter.as_str()) {
            continue;
        }
        let ir = build(&w, &format!("rp{i}")).unwrap_or_else(|e| panic!("{rule:?}: {e}"));
        emit(json!({"circuit": format!("{rule:?}"), "file": file, "model": model_full(&ir)}));
    }
}
