//! Chiplet study (`research/sherlock-processor-analogy-20261110/study.md` section 6),
//! experiments E0 and E1. Ignored; run each alone under `doctorwatson heavy`.
//!
//! * `chip_e0_census`: (a) instructions of the dispatch block by source, from the
//!   real emitters: the dispatch PROG walk, the three checked value reads (each split
//!   into its own PROG walk, DATA canonicality, lane range checks and selection), the
//!   facts' lane range checks, and the rule logic (sites, guards, path) per rule; the
//!   fragment IR of every kit rule split by instruction kind and emitter scope.
//!   (b, c) native owner runs of fact(3), fold and list123: per step the fired rule,
//!   the lanes it changes mapped to the chip design's registers and memory tables,
//!   and the PROG rows the dispatch reads. Native only; no circuit is built for (b, c).
//! * `chip_e1_shared_walk`: the dispatch relation with one PROG walk shared by the
//!   facts and the value reads (`emit_c6_dispatch_block_dedup`): instructions for all
//!   supported rules, `cost_model` rows and k for two rules, and acceptance parity
//!   with the unchanged circuit on sampled real states (fired rule accepts, other
//!   rules reject), including Validate- and Equal-gated reads.
//!
//! Output: one JSON line per record, prefixed `CHIP-E0 ` / `CHIP-E1 `, and the same
//! records in `$CHIP_OUT/<name>.json` when `CHIP_OUT` is set.
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_core::sherlock::helper_machine::c6_phys::{
    C6CodecBudgets, FRAME_LANES, IMAGE_SECTION_HEADER_LANES, L_CONTROL, L_CONTROL_ORIGIN,
    L_FRAME_COUNT,
};
use mori_core::sherlock::helper_machine::c6_phys_data::{
    DATA_HEADER_LANES, DATA_ROW_LANES, ENV_ROW_LANES, REF_LANES,
};
use mori_core::sherlock::helper_machine::c6_phys_helper::{
    L_HELPER_RECORD, L_RESULT_RECORD, L_TASKS, TASK_LANES,
};
use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostConfig, C6HostLimits};
use mori_core::sherlock::helper_machine::complete_observation::{HostSnapshot, capture};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::{OwnerEngine, RuleIdC, StepResult};
use mori_core::sherlock_compile_artifact::draft_execution::draft_genesis;
use serde_json::{Value, json};

use super::c6_checked_value_read::{C6CheckedValueReadEmitter, C6CheckedValueReadInputs};
use super::c6_data_canonical::{C6DataCanonicalEmitter, C6DataCanonicalInputs};
use super::c6_dispatch_consumer::{
    C6DispatchCircuit, build_c6_dispatch_circuit, build_c6_dispatch_circuit_dedup,
    check_c6_dispatch_circuit, witness_from_host_snapshot,
};
use super::c6_dispatch_step::{
    C6DispatchInputs, emit_c6_dispatch_block, emit_c6_dispatch_block_dedup, supported_rules,
};
use super::c6_program_context_canonical::{
    C6ProgramContextCanonicalEmitter, C6ProgramContextConstCaptureInputs,
    C6ProgramContextDispatchCaptureInputs,
};
use super::c6_step_harness::{C6StepWitness, step_witness_from_host_pair};

const CONFIG: &str = include_str!("testdata/dispatch/config.json");
const FACT: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } }\n  entry fact(3);\n}\n";
const FOLD: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fold(t: Tree): UInt { match t { leaf(x) => x, branch(l, r) => fold(l) + fold(r) } }\n  entry fold(branch(leaf(4), leaf(5)));\n}\n";
const LIST123: &str = "language \"moriarty-sherlock-draft/1\";\nprogram List123 {\n  fn sum(xs: List): UInt { match xs { nil => 0, cons(h, t) => h + sum(t) } }\n  entry sum(cons(1, cons(2, cons(3, nil))));\n}\n";

fn limits() -> C6HostLimits {
    C6HostLimits {
        max_total_lanes: 1 << 20,
        max_state_lanes: 1 << 20,
        max_metadata_bytes: 8192,
        max_string_bytes: 512,
        max_width_entries: 64,
    }
}

fn emit_record(tag: &str, name: &str, v: &Value) {
    println!("{tag} {}", json!({ "name": name, "record": v }));
    if let Some(dir) = std::env::var_os("CHIP_OUT") {
        let dir = std::path::PathBuf::from(dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join(format!("{name}.json")),
            serde_json::to_vec_pretty(v).unwrap(),
        )
        .unwrap();
    }
}

struct Run {
    cfg: C6HostConfig,
    /// (pre, post, fired rule)
    steps: Vec<(HostSnapshot, HostSnapshot, RuleIdC)>,
}

fn run(source: &str, config: &str) -> Result<Run, String> {
    let g = draft_genesis(source, config)
        .map_err(|f| format!("genesis: {}", serde_json::to_string(&f).unwrap()))?;
    let cfg = C6HostConfig::from_retained_layout(
        &g.layout,
        C6CodecBudgets {
            type_expr_nodes: 128,
            template_bytes: 4096,
        },
    );
    let engine = host_owner_engine();
    let mut image = g.image;
    let mut steps = Vec::new();
    loop {
        let pre = capture(&g.admitted, &image).map_err(|e| format!("capture: {e:?}"))?;
        match engine
            .step(&g.admitted, &mut image)
            .map_err(|e| format!("step: {e:?}"))?
        {
            StepResult::Terminal => break,
            StepResult::Event { event } => {
                let post = capture(&g.admitted, &image).map_err(|e| format!("capture: {e:?}"))?;
                steps.push((pre, post, event.rule_id));
            }
        }
    }
    Ok(Run { cfg, steps })
}

/// Instruction variant name.
fn kind(ins: &Instruction) -> String {
    let d = format!("{ins:?}");
    d.split(|c: char| c == ' ' || c == '{' || c == '(')
        .next()
        .unwrap_or("")
        .to_string()
}

/// Counts by instruction kind, with the two kinds the study separates.
fn kinds(ins: &[Instruction]) -> Value {
    let mut by: BTreeMap<String, usize> = BTreeMap::new();
    for i in ins {
        *by.entry(kind(i)).or_default() += 1;
    }
    let range = by.get("ConstrainBits").copied().unwrap_or(0);
    let hash: usize = by
        .iter()
        .filter(|(k, _)| k.to_lowercase().contains("hash"))
        .map(|(_, v)| *v)
        .sum();
    json!({ "total": ins.len(), "constrain_bits": range, "hash": hash, "by_kind": by })
}

fn pubs(label: &str, n: usize) -> Vec<Operand> {
    (0..n)
        .map(|i| Operand::Variable(Identifier(format!("%e0.public.{label}.{i}"))))
        .collect()
}

fn dispatch_inputs(w: &C6StepWitness) -> C6DispatchInputs {
    let geometry = w.source_params.geometry();
    C6DispatchInputs {
        enable: pubs("enable", 1).remove(0),
        caps: geometry.caps,
        pre_prog: pubs("prog", w.host_prefix.prog.len()),
        pre_image: pubs("image", geometry.image_total_lanes),
        pre_data: pubs("data", geometry.data_total_lanes),
        pre_helper: pubs("helper", geometry.helper_total_lanes),
    }
}

fn first_step(source: &str) -> (Run, C6StepWitness) {
    let r = run(source, CONFIG).expect("run");
    let (pre, post, _) = &r.steps[0];
    let w = step_witness_from_host_pair(true, pre, post, &r.cfg, &limits()).expect("witness");
    (r, w)
}

/// E0(a): the dispatch block of every rule, and its shared components standalone.
fn dispatch_census(w: &C6StepWitness) -> Value {
    let inputs = dispatch_inputs(w);
    let caps = inputs.caps;
    let gate = pubs("gate", 1).remove(0);
    let idx = pubs("idx", 1).remove(0);
    let walk = C6ProgramContextCanonicalEmitter::new("e0walk")
        .unwrap()
        .emit_prog_dispatch_capture(&C6ProgramContextDispatchCaptureInputs {
            gate: gate.clone(),
            pre_prog: inputs.pre_prog.clone(),
            caps,
            first_index: idx.clone(),
            second_index: idx.clone(),
            child_index: idx.clone(),
        })
        .unwrap()
        .instructions;
    let const_walk = C6ProgramContextCanonicalEmitter::new("e0cwalk")
        .unwrap()
        .emit_prog_const_capture(&C6ProgramContextConstCaptureInputs {
            gate: gate.clone(),
            pre_prog: inputs.pre_prog.clone(),
            caps,
            selected_index: idx.clone(),
        })
        .unwrap()
        .instructions;
    let data = C6DataCanonicalEmitter::new("e0data")
        .unwrap()
        .emit(&C6DataCanonicalInputs {
            gate: gate.clone(),
            data: inputs.pre_data.clone(),
            caps,
        })
        .unwrap()
        .instructions;
    let read = C6CheckedValueReadEmitter::new("e0read")
        .unwrap()
        .emit(&C6CheckedValueReadInputs {
            gate: gate.clone(),
            ref_kind_is_data: pubs("kind", 1).remove(0),
            ref_index: idx.clone(),
            pre_prog: inputs.pre_prog.clone(),
            pre_data: inputs.pre_data.clone(),
            caps,
        })
        .unwrap()
        .instructions;
    let lanes_prog = inputs.pre_prog.len();
    let lanes_data = inputs.pre_data.len();
    let lanes_image = inputs.pre_image.len();
    let lanes_helper = inputs.pre_helper.len();
    // Per rule: the whole block, and the E1 block.
    let mut per_rule = BTreeMap::new();
    let mut per_rule_dedup = BTreeMap::new();
    let t = Instant::now();
    for rule in supported_rules() {
        let b = emit_c6_dispatch_block(rule, &inputs, "e0d").unwrap();
        per_rule.insert(format!("{rule:?}"), b.instructions.len());
        let d = emit_c6_dispatch_block_dedup(rule, &inputs, "e0d").unwrap();
        per_rule_dedup.insert(format!("{rule:?}"), d.instructions.len());
    }
    let emit_seconds = t.elapsed().as_secs_f64();
    let any = RuleIdC::KMachinePushAccept;
    let whole = emit_c6_dispatch_block(any, &inputs, "e0d")
        .unwrap()
        .instructions;
    let dedup = emit_c6_dispatch_block_dedup(any, &inputs, "e0d")
        .unwrap()
        .instructions;
    let read_n = read.len();
    let read_walk = const_walk.len();
    let read_data = data.len();
    let read_lane_range = lanes_prog + lanes_data;
    let read_select = read_n as i64 - (read_walk + read_data + read_lane_range) as i64;
    let facts_lane_range = lanes_image + lanes_helper;
    let shared = walk.len() + 3 * read_n + facts_lane_range;
    let logic: BTreeMap<String, i64> = per_rule
        .iter()
        .map(|(k, v)| (k.clone(), *v as i64 - shared as i64))
        .collect();
    json!({
        "witness": "fact(3) first step, qualification config testdata/dispatch/config.json",
        "lanes": {"prog": lanes_prog, "image": lanes_image, "data": lanes_data, "helper": lanes_helper},
        "components": {
            "dispatch_prog_walk": kinds(&walk),
            "value_read_whole": kinds(&read),
            "value_read_prog_walk": kinds(&const_walk),
            "value_read_data_canonical": kinds(&data),
            "value_read_lane_range_checks": read_lane_range,
            "value_read_selection_rest": read_select,
            "facts_lane_range_checks": facts_lane_range,
        },
        "sources_per_step": {
            "prog_walks": walk.len() + 3 * read_walk,
            "prog_walk_count": 4,
            "value_reads_excluding_walks": 3 * (read_n as i64 - read_walk as i64),
            "of_which_data_canonical": 3 * read_data,
            "of_which_lane_range": 3 * read_lane_range,
            "facts_lane_range": facts_lane_range,
            "rule_logic_per_rule": logic,
        },
        "whole_block_kinds_KMachinePushAccept": kinds(&whole),
        "dedup_block_kinds_KMachinePushAccept": kinds(&dedup),
        "instructions_per_rule": per_rule,
        "instructions_per_rule_dedup": per_rule_dedup,
        "emit_seconds": emit_seconds,
    })
}

type KitBuild = fn(
    RuleIdC,
    super::c6_push_step::C6PushFamilyParams,
    usize,
    usize,
    usize,
    &str,
) -> Result<super::c6_step_harness::C6StepCircuit, String>;

/// Every kit family (one `C6StepCircuit` per rule) with its rules.
fn kits() -> Vec<(&'static str, Vec<RuleIdC>, KitBuild)> {
    use super::*;
    macro_rules! kit {
        ($rules:expr, $file:literal, $build:path) => {
            (
                $file,
                $rules.to_vec(),
                (|r: RuleIdC,
                  p: c6_push_step::C6PushFamilyParams,
                  m: usize,
                  g: usize,
                  o: usize,
                  s: &str| $build(r, p, m, g, o, s).map_err(|e| e.to_string())) as KitBuild,
            )
        };
    }
    vec![
        kit!(
            c6_admin_publish_step::ADMIN_PUBLISH_RULES,
            "c6_admin_publish",
            c6_admin_publish_consumer::build_c6_admin_publish_circuit
        ),
        kit!(
            c6_eval_schedule_step::EVAL_SCHEDULE_RULES,
            "c6_eval_schedule",
            c6_eval_schedule_consumer::build_c6_eval_schedule_circuit
        ),
        kit!(
            c6_legacy_control_step::LEGACY_CONTROL_RULES,
            "c6_legacy_control",
            c6_legacy_control_consumer::build_c6_legacy_control_circuit
        ),
        kit!(
            c6_call_bind_step::CALL_BIND_RULES,
            "c6_call_bind",
            c6_call_bind_consumer::build_c6_call_bind_circuit
        ),
        kit!(
            c6_call_helper_step::CALL_HELPER_RULES,
            "c6_call_helper",
            c6_call_helper_consumer::build_c6_call_helper_circuit
        ),
        kit!(
            c6_bind_helper_step::BIND_HELPER_RULES,
            "c6_bind_helper",
            c6_bind_helper_consumer::build_c6_bind_helper_circuit
        ),
        kit!(
            c6_append_helper_step::APPEND_HELPER_RULES,
            "c6_append_helper",
            c6_append_helper_consumer::build_c6_append_helper_circuit
        ),
        kit!(
            c6_validate_helper_step::VALIDATE_HELPER_RULES,
            "c6_validate_helper",
            c6_validate_helper_consumer::build_c6_validate_helper_circuit
        ),
        kit!(
            c6_equal_helper_step::EQUAL_HELPER_RULES,
            "c6_equal_helper",
            c6_equal_helper_consumer::build_c6_equal_helper_circuit
        ),
        kit!(
            c6_match_helper_step::MATCH_HELPER_RULES,
            "c6_match_helper",
            c6_match_helper_consumer::build_c6_match_helper_circuit
        ),
        kit!(
            c6_scalar_construct_step::SCALAR_CONSTRUCT_RULES,
            "c6_scalar_construct",
            c6_scalar_construct_consumer::build_c6_scalar_construct_circuit
        ),
    ]
}

/// The emitter scope chain of an identifier below the circuit scope, with numeric
/// and temp segments dropped (`%k.public.x.3` -> `public`).
fn scope_key(ins: &Instruction, scope: &str) -> String {
    let d = format!("{ins:?}");
    let head = format!("%{scope}.");
    let Some(at) = d.find(&head) else {
        return "-".into();
    };
    let rest = &d[at + head.len()..];
    let id: String = rest
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '.' || *c == '_')
        .collect();
    let segs: Vec<&str> = id
        .split('.')
        .filter(|s| !s.is_empty() && !s.chars().all(|c| c.is_ascii_digit()))
        .take_while(|s| *s != "temp" && *s != "local")
        .take(3)
        .collect();
    if segs.is_empty() {
        "-".into()
    } else {
        segs.join(".")
    }
}

/// E0(a): fragment IR of one rule per kit by kind and scope; totals for every rule.
fn fragment_census(w: &C6StepWitness) -> Value {
    let mut out = BTreeMap::new();
    for (file, rules, build) in kits() {
        let mut per_rule = BTreeMap::new();
        let mut detail = Value::Null;
        for (n, rule) in rules.iter().enumerate() {
            let scope = format!("e0f{n}");
            let t = Instant::now();
            let c = match build(
                *rule,
                w.source_params.clone(),
                w.host_prefix.meta.len(),
                w.host_prefix.prog.len(),
                w.host_prefix.meta_bounds_frames_offset,
                &scope,
            ) {
                Ok(c) => c,
                Err(e) => {
                    per_rule.insert(format!("{rule:?}"), json!({"error": e}));
                    continue;
                }
            };
            let ins = c.ir().instructions.as_ref();
            per_rule.insert(
                format!("{rule:?}"),
                json!({"instructions": ins.len(), "seconds": t.elapsed().as_secs_f64()}),
            );
            if n == 0 {
                let mut by_scope: BTreeMap<String, usize> = BTreeMap::new();
                for i in ins {
                    *by_scope.entry(scope_key(i, &scope)).or_default() += 1;
                }
                detail =
                    json!({"rule": format!("{rule:?}"), "kinds": kinds(ins), "by_scope": by_scope});
            }
        }
        out.insert(
            file.to_string(),
            json!({"rules": per_rule, "first_rule_split": detail}),
        );
    }
    json!(out)
}

/// Lanes of `pre` and `post` that differ, as index ranges `[a, b)` into the section.
fn changed(pre: &[u128], post: &[u128], a: usize, b: usize) -> bool {
    (a..b.min(pre.len())).any(|i| pre[i] != post[i])
}

fn all_zero(v: &[u128], a: usize, b: usize) -> bool {
    (a..b.min(v.len())).all(|i| v[i] == 0)
}

/// A table of fixed-width rows: (appended, overwritten, cleared) row counts.
fn table_delta(pre: &[u128], post: &[u128], base: usize, width: usize, rows: usize) -> [usize; 3] {
    let mut d = [0; 3];
    for r in 0..rows {
        let (a, b) = (base + r * width, base + (r + 1) * width);
        if !changed(pre, post, a, b) {
            continue;
        }
        if all_zero(pre, a, b) {
            d[0] += 1;
        } else if all_zero(post, a, b) {
            d[2] += 1;
        } else {
            d[1] += 1;
        }
    }
    d
}

/// E0(b, c): one owner step mapped onto the chip design's registers and memory.
fn step_record(i: usize, rule: RuleIdC, w: &C6StepWitness) -> Value {
    let caps = w.source_params.geometry().caps;
    let (pi, qi) = (&w.pre_image, &w.post_image);
    let (ph, qh) = (&w.pre_helper, &w.post_helper);
    let (pd, qd) = (&w.pre_data, &w.post_data);
    let mut regs = BTreeSet::new();
    let hdr: [(&str, usize, usize); 9] = [
        ("ctl", L_CONTROL, L_CONTROL_ORIGIN + 1),
        ("env", L_CONTROL_ORIGIN + 1, L_CONTROL_ORIGIN + 2),
        ("st.lane21", 21, 22),
        ("st.stack_limit", 22, 23),
        ("st.outcome", 23, 26),
        ("st.fuel_counters", 26, 32),
        ("len.alloc_active", 32, 39),
        ("len.segments", 39, L_FRAME_COUNT),
        ("st.frame_count", L_FRAME_COUNT, L_FRAME_COUNT + 1),
    ];
    for (name, a, b) in hdr {
        if changed(pi, qi, a, b) {
            regs.insert(name.to_string());
        }
    }
    let depth = |v: &[u128]| v[L_FRAME_COUNT] as usize;
    let (d0, d1) = (depth(pi), depth(qi));
    let mut frames = json!({});
    let (mut push, mut top_rw, mut clear, mut below) = (0, 0, 0, 0);
    for f in 0..caps.phys.max_frames {
        let a = IMAGE_SECTION_HEADER_LANES + f * FRAME_LANES;
        if !changed(pi, qi, a, a + FRAME_LANES) {
            continue;
        }
        if f + 1 == d1 && d1 > d0 {
            push += 1;
        } else if f + 1 == d1 {
            top_rw += 1;
        } else if f + 1 > d1 {
            clear += 1;
        } else {
            below += 1;
        }
    }
    frames["push"] = json!(push);
    frames["rewrite_top"] = json!(top_rw);
    frames["clear_above_top"] = json!(clear);
    frames["write_below_top"] = json!(below);
    // Helper: header lanes 0..4 (lane 3 = live tasks), helper record, result, tasks.
    if changed(ph, qh, 0, L_HELPER_RECORD) {
        regs.insert("tp.helper_header".into());
    }
    if changed(ph, qh, L_HELPER_RECORD, L_RESULT_RECORD) {
        regs.insert("h".into());
    }
    if changed(ph, qh, L_RESULT_RECORD, L_TASKS) {
        regs.insert("r".into());
    }
    let (l0, l1) = (ph[3] as usize, qh[3] as usize);
    let (mut tpush, mut ttop, mut tclear, mut tbelow) = (0, 0, 0, 0);
    for t in 0..caps.max_helper_tasks {
        let a = L_TASKS + t * TASK_LANES;
        if !changed(ph, qh, a, a + TASK_LANES) {
            continue;
        }
        if t + 1 == l1 && l1 > l0 {
            tpush += 1;
        } else if t + 1 == l1 {
            ttop += 1;
        } else if t + 1 > l1 {
            tclear += 1;
        } else {
            tbelow += 1;
        }
    }
    if push + top_rw > 0 {
        regs.insert("top".into());
    }
    if tpush + ttop > 0 {
        regs.insert("task".into());
    }
    let geom = super::c6_data_canonical::c6_data_geometry(&caps).unwrap();
    let data_hdr = changed(pd, qd, 0, DATA_HEADER_LANES);
    let tables = json!({
        "data_rows": table_delta(pd, qd, geom.data_base, DATA_ROW_LANES, caps.phys.max_data_nodes),
        "data_fields": table_delta(pd, qd, geom.fields_base, REF_LANES, caps.max_data_field_slots),
        "env_cells": table_delta(pd, qd, geom.env_base, ENV_ROW_LANES, caps.phys.max_env_cells),
        "value_slots": table_delta(pd, qd, geom.values_base, REF_LANES, caps.phys.max_value_slots),
    });
    if data_hdr {
        regs.insert("len.data_header".into());
    }
    let top = |v: &[u128], d: usize| -> u128 {
        if d == 0 {
            0
        } else {
            v[IMAGE_SECTION_HEADER_LANES + (d - 1) * FRAME_LANES]
        }
    };
    json!({
        "i": i,
        "rule": format!("{rule:?}"),
        "registers_written": regs,
        "frames": frames,
        "tasks": {"push": tpush, "rewrite_top": ttop, "clear_above_top": tclear, "write_below_top": tbelow},
        "tables": tables,
        "depth": [d0, d1],
        "live_tasks": [l0, l1],
        "ct": pi[L_CONTROL],
        "k1": pi[L_CONTROL + 1],
        "control_origin": pi[L_CONTROL_ORIGIN],
        "top_tag": top(pi, d0),
        "helper_tag": ph[L_HELPER_RECORD],
        "result_tag": ph[L_RESULT_RECORD],
    })
}

fn owner_runs() -> Value {
    let mut out = BTreeMap::new();
    for (name, src) in [("fact3", FACT), ("fold", FOLD), ("list123", LIST123)] {
        let r = match run(src, CONFIG) {
            Ok(r) => r,
            Err(e) => {
                out.insert(name.to_string(), json!({"error": e}));
                continue;
            }
        };
        let mut steps = Vec::new();
        for (i, (pre, post, rule)) in r.steps.iter().enumerate() {
            match step_witness_from_host_pair(true, pre, post, &r.cfg, &limits()) {
                Ok(w) => steps.push(step_record(i, *rule, &w)),
                Err(e) => steps.push(json!({"i": i, "rule": format!("{rule:?}"), "error": e})),
            }
        }
        out.insert(
            name.to_string(),
            json!({"steps": steps.len(), "records": steps}),
        );
    }
    json!(out)
}

#[test]
#[ignore = "chiplet study E0 census: emits 108 dispatch blocks twice and every kit fragment; run alone under doctorwatson heavy"]
fn chip_e0_census() {
    let t = Instant::now();
    let runs = owner_runs();
    emit_record("CHIP-E0", "e0_owner_runs", &runs);
    let (_, w) = first_step(FACT);
    let d = dispatch_census(&w);
    emit_record("CHIP-E0", "e0_dispatch", &d);
    if std::env::var_os("CHIP_E0_SKIP_FRAGMENTS").is_none() {
        let f = fragment_census(&w);
        emit_record("CHIP-E0", "e0_fragments", &f);
    }
    println!("CHIP-E0 done in {:.1} s", t.elapsed().as_secs_f64());
}

/// One dispatch circuit per (rule, variant), at most eight held at once.
fn circuit<'a>(
    cache: &'a mut BTreeMap<String, C6DispatchCircuit>,
    rule: RuleIdC,
    dedup: bool,
    params: &super::c6_push_step::C6PushFamilyParams,
    (meta_len, prog_len): (usize, usize),
) -> &'a C6DispatchCircuit {
    // Keyed by the witness shape too: programs differ in PROG and META length.
    let key = format!("{rule:?}/{dedup}/{meta_len}/{prog_len}/{params:?}");
    if !cache.contains_key(&key) {
        if cache.len() >= 8 {
            cache.clear();
        }
        let build = if dedup {
            build_c6_dispatch_circuit_dedup
        } else {
            build_c6_dispatch_circuit
        };
        let scope = format!("p{}{}", format!("{rule:?}").to_lowercase(), u8::from(dedup));
        let c = build(rule, params.clone(), meta_len, prog_len, &scope).unwrap();
        cache.insert(key.clone(), c);
    }
    &cache[&key]
}

fn model(ir: &IrSource) -> Value {
    let t = Instant::now();
    let m = ir.model();
    json!({"k": m.k(), "rows": m.rows(), "instructions": ir.instructions.len(), "seconds": t.elapsed().as_secs_f64()})
}

#[test]
#[ignore = "chiplet study E1: cost_model of two dispatch circuits twice and acceptance parity; run alone under doctorwatson heavy"]
fn chip_e1_shared_walk() {
    let fact = run(FACT, CONFIG).expect("fact run");
    let fold = run(FOLD, CONFIG).expect("fold run");
    let (pre0, _, fired0) = &fact.steps[0];
    let w0 = witness_from_host_snapshot(true, pre0, &fact.cfg, &limits()).unwrap();
    let meta_len = {
        let (pre, post, _) = &fact.steps[0];
        let sw = step_witness_from_host_pair(true, pre, post, &fact.cfg, &limits()).unwrap();
        (sw.host_prefix.meta.len(), sw.host_prefix.prog.len())
    };
    // Rows: the first fired rule and BHelperMatchArm (the largest measured C6 circuit).
    // `CHIP_E1_MODELS=0` skips the models; `CHIP_E1_RUNS` lists the parity runs.
    let do_models = std::env::var("CHIP_E1_MODELS").map_or(true, |v| v != "0");
    let runs = std::env::var("CHIP_E1_RUNS").unwrap_or_else(|_| "fact3,fold".into());
    let mut models = BTreeMap::new();
    for rule in [*fired0, RuleIdC::BHelperMatchArm]
        .into_iter()
        .filter(|_| do_models)
    {
        for (label, dedup) in [("unchanged", false), ("shared_walk", true)] {
            let scope = format!("e1{}", if dedup { "s" } else { "u" });
            let build = if dedup {
                build_c6_dispatch_circuit_dedup
            } else {
                build_c6_dispatch_circuit
            };
            let c = build(
                rule,
                w0.source_params().clone(),
                meta_len.0,
                meta_len.1,
                &scope,
            )
            .unwrap();
            let m = model(c.ir());
            println!("CHIP-E1 model {rule:?} {label} {m}");
            models.insert(format!("{rule:?}/{label}"), m);
        }
    }
    // Parity on sampled real states: fact(3) every 8th step and every step whose
    // helper is a pending Validate (7) or Equal (8) read, fold every 8th step.
    let mut parity = Vec::new();
    let mut circuits: BTreeMap<String, C6DispatchCircuit> = BTreeMap::new();
    let mut checked = 0usize;
    let mut mismatches = Vec::new();
    let selected = [("fact3", &fact), ("fold", &fold)]
        .into_iter()
        .filter(|(label, _)| runs.split(',').any(|r| r == *label));
    for (label, r) in selected {
        for (i, (pre, _, fired)) in r.steps.iter().enumerate() {
            let w = witness_from_host_snapshot(true, pre, &r.cfg, &limits()).unwrap();
            let (sw_tag, shape) = {
                let (p, q, _) = &r.steps[i];
                let sw = step_witness_from_host_pair(true, p, q, &r.cfg, &limits()).unwrap();
                (
                    sw.pre_helper[L_HELPER_RECORD],
                    (sw.host_prefix.meta.len(), sw.host_prefix.prog.len()),
                )
            };
            let read_gated = label == "fact3" && (sw_tag == 7 || sw_tag == 8);
            if i % 8 != 0 && !read_gated {
                continue;
            }
            let other = if *fired == RuleIdC::KAdminPublishExhausted {
                RuleIdC::KAdminPublishValue
            } else {
                RuleIdC::KAdminPublishExhausted
            };
            for (rule, expect) in [(*fired, true), (other, false)] {
                // Each circuit is built for this witness's own family and lengths.
                let mut res = [false; 2];
                let mut errs = [String::new(), String::new()];
                for (j, dedup) in [false, true].into_iter().enumerate() {
                    let c = circuit(&mut circuits, rule, dedup, w.source_params(), shape);
                    match check_c6_dispatch_circuit(c, &w) {
                        Ok(_) => res[j] = true,
                        Err(e) => errs[j] = format!("{e:?}").chars().take(160).collect(),
                    }
                }
                checked += 1;
                let rec = json!({"run": label, "i": i, "rule": format!("{rule:?}"), "helper_tag": sw_tag, "expect": expect, "unchanged": res[0], "shared_walk": res[1], "shape": [shape.0, shape.1], "errors": errs});
                if res[0] != expect || res[1] != expect {
                    mismatches.push(rec.clone());
                }
                parity.push(rec);
            }
        }
    }
    let out = json!({
        "witness": "fact(3) and fold, qualification config testdata/dispatch/config.json",
        "models": models,
        "parity_checks": checked,
        "parity_mismatches": mismatches,
        "parity": parity,
    });
    emit_record("CHIP-E1", "e1_shared_walk", &out);
    assert!(
        out["parity_mismatches"].as_array().unwrap().is_empty(),
        "parity mismatch"
    );
}
