//! Dispatch owner tests, split along task 3.3 (SAS-013, SVM-003).
//!
//! * T1 `dispatch_partition_is_exclusive_and_committed`: the partition derived
//!   mechanically from the dispatch code (`c6_dispatch_partition`) selects at
//!   most one supported rule in every class, gives every supported rule a
//!   class, and equals the committed `testdata/dispatch/partition.json`.
//! * T1 `dispatch_selector_selects_the_fired_rule_on_every_step`: on every
//!   pre-state of fact(3), list123, tree fold and `entry 7;` the dispatch IR,
//!   evaluated concretely, selects exactly the rule the owner fired, and the
//!   state's partition class names that rule.
//! * T1 `dispatch_directed_cases_select_their_class_rule`: every class no
//!   program reaches is realized, where a coordinated raw-lane edit of a real
//!   state can reach it, and the dispatch IR selects exactly the class's rule.
//! * T2 `dispatch_fired_rule_accepted_on_every_step` (ignored): the SDK
//!   `ProofPreimage::check` accepts the fired rule's dispatch circuit on every
//!   step and the disabled identity.
//! * T2 `dispatch_directed_cases_sdk` (ignored; one budget key per class group
//!   via `DISPATCH_GROUP`): per rule one directed case; the rule's circuit
//!   accepts it, every other rule of its group and the publication rules
//!   reject it under the SDK check.
//!
//! The concrete evaluator (`partition::Compiled`) is an in-repo interpreter
//! of the dispatch IR; the T2 SDK runs are what tie it to `ProofPreimage::check`.
//! * T1 `dispatch_rejects_mutated_pre_lanes`: raw single-lane mutations.
//! * Diagnostic `dispatch_diagnostic_one_state_one_rule` (U023a, SAS-014):
//!   reduced coverage, never an acceptance receipt.
//!
//! Expected rules come from the owner event stream, never from the emitter.
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

use super::super::c6_dispatch_step::partition::{
    Compiled, Dom, Ord3, Partition, Selector, derive_partition, emit_selector, group_label,
};
use super::super::c6_dispatch_step::{
    C6DispatchInputs, dispatch_supported, emit_c6_dispatch_block, supported_rules,
};
use super::*;
use midnight_transient_crypto::curve::Fr;
use midnight_zkir_v3::Identifier;
use midnight_zkir_v3::ir::Operand;
use mori_core::sherlock::helper_machine::c6_phys::{
    C6CodecBudgets, FRAME_LANES, IMAGE_SECTION_HEADER_LANES, L_CONTROL, L_FRAME_COUNT, L_OUTCOME,
    L_REMAINING_WORK, L_STACK_LIMIT,
};
use mori_core::sherlock::helper_machine::c6_phys_helper::{
    L_HELPER_RECORD, L_RESULT_RECORD, L_TASKS, TASK_LANES,
};
use mori_core::sherlock::helper_machine::complete_observation::capture;
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::{OwnerEngine, StepResult};
use mori_core::sherlock_compile_artifact::draft_execution::draft_genesis;
use serde_json::json;
use sha2::{Digest, Sha256};

/// Small admitted bounds (16 frames, 48 data/env rows, 8 tasks) that fit every run.
const CONFIG: &str = include_str!("testdata/dispatch/config.json");
const PARTITION_PATH: &str = "src/sherlock_machine/testdata/dispatch/partition.json";
const DISPATCH_SOURCE: &str = include_str!("c6_dispatch_step.rs");
const FACT: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } }\n  entry fact(3);\n}\n";
const FOLD: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fold(t: Tree): UInt { match t { leaf(x) => x, branch(l, r) => fold(l) + fold(r) } }\n  entry fold(branch(leaf(4), leaf(5)));\n}\n";
const LIST123: &str = "language \"moriarty-sherlock-draft/1\";\nprogram List123 {\n  fn sum(xs: List): UInt { match xs { nil => 0, cons(h, t) => h + sum(t) } }\n  entry sum(cons(1, cons(2, cons(3, nil))));\n}\n";
const LITERAL: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  entry 7;\n}\n";
/// Zero-argument call, `let`, a fields constructor and a nullary constructor.
const MK: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn mk(): List { let x = 1 in cons(x, nil) }\n  entry mk();\n}\n";
const PROGRAMS: [(&str, &str); 5] = [
    ("fact(3)", FACT),
    ("list123", LIST123),
    ("fold", FOLD),
    ("entry 7", LITERAL),
    ("mk", MK),
];

fn limits() -> C6HostLimits {
    C6HostLimits {
        max_total_lanes: 1 << 20,
        max_state_lanes: 1 << 20,
        max_metadata_bytes: 8192,
        max_string_bytes: 512,
        max_width_entries: 64,
    }
}

struct Run {
    cfg: C6HostConfig,
    /// (pre-state, rule the owner fired from it)
    steps: Vec<(HostSnapshot, RuleIdC)>,
}

fn run(source: &str) -> Run {
    let g = draft_genesis(source, CONFIG)
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
    let mut steps = Vec::new();
    loop {
        let pre = capture(&g.admitted, &image).expect("capture");
        match engine.step(&g.admitted, &mut image).expect("owner step") {
            StepResult::Terminal => break,
            StepResult::Event { event } => steps.push((pre, event.rule_id)),
        }
    }
    Run { cfg, steps }
}

struct Circuits {
    by_rule: BTreeMap<String, C6DispatchCircuit>,
}

impl Circuits {
    fn new() -> Self {
        Self {
            by_rule: BTreeMap::new(),
        }
    }
    fn get(&mut self, rule: RuleIdC, w: &C6DispatchWitness) -> &C6DispatchCircuit {
        let key = format!("{rule:?}/{}", w.host_prefix.prog.len());
        self.by_rule.entry(key.clone()).or_insert_with(|| {
            build_c6_dispatch_circuit(
                rule,
                w.source_params().clone(),
                w.host_prefix.meta.len(),
                w.host_prefix.prog.len(),
                &format!("dispatch{}", format!("{rule:?}").to_lowercase()),
            )
            .unwrap_or_else(|e| panic!("build {key}: {e}"))
        })
    }
}

fn accepts(c: &mut Circuits, rule: RuleIdC, w: &C6DispatchWitness) -> bool {
    check_c6_dispatch_circuit(c.get(rule, w), w).is_ok()
}

fn var(label: &str, i: usize) -> Identifier {
    Identifier(format!("%probe.{label}.{i}"))
}

/// Dispatch inputs over named variables shaped like the witness.
fn sym_inputs(w: &C6DispatchWitness) -> C6DispatchInputs {
    let g = w.source_params.geometry();
    let v = |label: &str, n: usize| {
        (0..n)
            .map(|i| Operand::Variable(var(label, i)))
            .collect::<Vec<_>>()
    };
    C6DispatchInputs {
        enable: Operand::Variable(var("enable", 0)),
        caps: g.caps,
        pre_prog: v("prog", w.host_prefix.prog.len()),
        pre_image: v("image", g.image_total_lanes),
        pre_data: v("data", g.data_total_lanes),
        pre_helper: v("helper", g.helper_total_lanes),
    }
}

/// The selector compiled for concrete evaluation of many witnesses.
struct Probe {
    sel: Selector,
    c: Compiled,
    roots: Vec<usize>,
    paths: Vec<(RuleIdC, Result<usize, Fr>)>,
}

fn input_ids(w: &C6DispatchWitness) -> Vec<Identifier> {
    let g = w.source_params.geometry();
    let mut ids = vec![var("enable", 0)];
    for (label, n) in [
        ("prog", w.host_prefix.prog.len()),
        ("image", g.image_total_lanes),
        ("data", g.data_total_lanes),
        ("helper", g.helper_total_lanes),
    ] {
        ids.extend((0..n).map(|i| var(label, i)));
    }
    ids
}

fn input_values(w: &C6DispatchWitness) -> Vec<Fr> {
    std::iter::once(Fr::from(w.enable))
        .chain(
            w.host_prefix
                .prog
                .iter()
                .chain(&w.pre_image)
                .chain(&w.pre_data)
                .chain(&w.pre_helper)
                .map(|l| Fr::from(*l)),
        )
        .collect()
}

fn probe(w: &C6DispatchWitness) -> Probe {
    let sel = emit_selector(&sym_inputs(w), "partition").expect("selector");
    let c = Compiled::new(&sel.ins, &input_ids(w)).expect("compile");
    let roots = sel.roots.iter().map(|(_, id)| c.slot(id)).collect();
    let paths = sel
        .paths
        .iter()
        .map(|(r, p)| {
            (
                *r,
                match p {
                    Operand::Variable(id) => Ok(c.slot(id)),
                    Operand::Immediate(f) => Err(*f),
                },
            )
        })
        .collect();
    Probe {
        sel,
        c,
        roots,
        paths,
    }
}

/// Concrete root values and selected rules of one witness, or the first
/// violated dispatch constraint.
fn evaluate(p: &Probe, w: &C6DispatchWitness) -> Result<(Vec<u128>, Vec<RuleIdC>), String> {
    let m = p.c.run(&input_values(w))?;
    let values = p
        .roots
        .iter()
        .zip(&p.sel.roots)
        .map(|(s, (n, _))| u128::try_from(m[*s]).unwrap_or_else(|_| panic!("root {n} > u128")))
        .collect();
    let selected = p
        .paths
        .iter()
        .filter(|(_, x)| {
            let v = match x {
                Ok(s) => m[*s],
                Err(f) => *f,
            };
            v == Fr::from(1u64)
        })
        .map(|(r, _)| *r)
        .collect();
    Ok((values, selected))
}

fn witness(pre: &HostSnapshot, r: &Run) -> C6DispatchWitness {
    witness_from_host_snapshot(true, pre, &r.cfg, &limits()).expect("witness")
}

fn partition_for(w: &C6DispatchWitness) -> (Selector, Partition) {
    let sel = emit_selector(&sym_inputs(w), "partition").expect("selector");
    let p = derive_partition(&sel);
    (sel, p)
}

/// Pretty header, one compact line per class.
fn partition_text(mut doc: serde_json::Value) -> String {
    let classes = doc
        .as_object_mut()
        .and_then(|o| o.remove("classes"))
        .expect("classes");
    let head = serde_json::to_string_pretty(&doc).unwrap();
    let head = head.strip_suffix("\n}").expect("object");
    let lines: Vec<String> = classes
        .as_array()
        .expect("array")
        .iter()
        .map(|c| format!("    {}", serde_json::to_string(c).unwrap()))
        .collect();
    format!(
        "{head},\n  \"classes\": [\n{}\n  ]\n}}\n",
        lines.join(",\n")
    )
}

fn source_sha256() -> String {
    Sha256::digest(DISPATCH_SOURCE.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

#[test]
fn dispatch_partition_is_exclusive_and_committed() {
    let r = run(LITERAL);
    let w = witness(&r.steps[0].0, &r);
    let (_, p) = partition_for(&w);
    let overlaps = p.overlaps();
    assert!(
        overlaps.is_empty(),
        "classes selecting more than one rule: {:?}",
        overlaps
            .iter()
            .map(|i| (p.atoms_json(*i), p.classes[*i].selected.clone()))
            .collect::<Vec<_>>()
    );
    let with_class: BTreeSet<String> = p
        .classes
        .iter()
        .filter_map(|c| c.selected.first().map(|r| format!("{r:?}")))
        .collect();
    let dead: Vec<RuleIdC> = supported_rules()
        .into_iter()
        .filter(|x| !with_class.contains(&format!("{x:?}")))
        .collect();
    assert!(
        dead.is_empty(),
        "supported rules no class selects: {dead:?}"
    );
    let doc = p.to_json(&source_sha256());
    let text = partition_text(doc);
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(PARTITION_PATH);
    if std::env::var_os("MORIARTY_BLESS_DISPATCH_PARTITION").is_some() {
        std::fs::write(&path, &text).expect("write partition");
    }
    let committed = std::fs::read_to_string(&path).unwrap_or_default();
    assert!(
        committed == text,
        "committed partition differs from the derived one; rerun with MORIARTY_BLESS_DISPATCH_PARTITION=1 ({} classes)",
        p.classes.len()
    );
    println!(
        "DISPATCH-PARTITION {}",
        json!({"classes": p.classes.len(), "slice_instructions": p.slice_len, "selector_instructions_rules": supported_rules().len()})
    );
}

/// Per program: fired-rule selection on every step and the classes reached.
fn reached_classes(p: &Partition) -> BTreeMap<usize, Vec<String>> {
    let mut reached: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    for (label, source) in PROGRAMS {
        let r = run(source);
        let sel = probe(&witness(&r.steps[0].0, &r));
        for (i, (pre, fired)) in r.steps.iter().enumerate() {
            let w = witness(pre, &r);
            let (values, selected) =
                evaluate(&sel, &w).unwrap_or_else(|e| panic!("{label} step {i}: refused: {e}"));
            assert!(
                dispatch_supported(*fired),
                "{label} step {i}: fired {fired:?} is not supported by the dispatch owner"
            );
            assert_eq!(
                selected,
                vec![*fired],
                "{label} step {i}: the dispatch IR selects {selected:?}, the owner fired {fired:?}"
            );
            let c = p.classify(&values);
            assert_eq!(
                p.class_rule(c),
                Some(*fired),
                "{label} step {i}: class {c} names {:?}",
                p.class_rule(c)
            );
            reached.entry(c).or_default().push(format!("{label}#{i}"));
        }
    }
    reached
}

#[test]
fn dispatch_selector_selects_the_fired_rule_on_every_step() {
    let r = run(LITERAL);
    let (_, p) = partition_for(&witness(&r.steps[0].0, &r));
    let reached = reached_classes(&p);
    let rules: BTreeSet<String> = reached
        .keys()
        .filter_map(|c| p.class_rule(*c).map(|r| format!("{r:?}")))
        .collect();
    println!(
        "DISPATCH-COVERAGE {}",
        json!({
            "classes": p.classes.len(),
            "reached": reached.len(),
            "rules_fired": rules.len(),
            "rules_supported": supported_rules().len(),
            "reached_ids": reached.keys().collect::<Vec<_>>(),
        })
    );
}

/// SDK checks grouped by circuit: each rule's circuit is built once, checked
/// against its witnesses and dropped (one circuit holds about 1.3M
/// instructions; caching all of them is what cost the old test 47 GB).
fn sdk_checks(
    by_rule: BTreeMap<String, (RuleIdC, Vec<(String, C6DispatchWitness, bool)>)>,
) -> usize {
    let mut n = 0;
    for (_, (rule, cases)) in by_rule {
        let mut c = Circuits::new();
        for (label, w, expect) in &cases {
            assert_eq!(
                accepts(&mut c, rule, w),
                *expect,
                "{label}: {rule:?} circuit, expected accept = {expect}"
            );
            n += 1;
        }
    }
    n
}

#[test]
#[ignore = "T2 SDK fired-rule acceptance on every step (about 800 ProofPreimage checks at about 3.6 s, one circuit at a time); run under doctorwatson heavy"]
fn dispatch_fired_rule_accepted_on_every_step() {
    let mut by_rule: BTreeMap<String, (RuleIdC, Vec<(String, C6DispatchWitness, bool)>)> =
        BTreeMap::new();
    for (label, source) in PROGRAMS {
        let r = run(source);
        for (i, (pre, fired)) in r.steps.iter().enumerate() {
            by_rule
                .entry(format!("{fired:?}"))
                .or_insert((*fired, Vec::new()))
                .1
                .push((format!("{label}#{i}"), witness(pre, &r), true));
        }
        // Disabled: enable = 0 accepts any rule's circuit (identity).
        let off =
            witness_from_host_snapshot(false, &r.steps[0].0, &r.cfg, &limits()).expect("witness");
        by_rule
            .entry(format!("{:?}", RuleIdC::KAdminPublishError))
            .or_insert((RuleIdC::KAdminPublishError, Vec::new()))
            .1
            .push((format!("{label} disabled"), off, true));
    }
    let circuits = by_rule.len();
    let checks = sdk_checks(by_rule);
    println!(
        "DISPATCH-FIRED {}",
        json!({"checks": checks, "circuits": circuits})
    );
}

/// Lanes a root name maps to, or None for PROG/DATA-derived roots.
fn lane_of(name: &str, w: &C6DispatchWitness) -> Option<(bool, usize)> {
    let image = |i: usize| Some((true, i));
    let helper = |i: usize| Some((false, i));
    let num = |prefix: &str| {
        name.strip_prefix(prefix)
            .and_then(|s| s.parse::<usize>().ok())
    };
    match name {
        "outcome" => image(L_OUTCOME),
        "remaining_work" => image(L_REMAINING_WORK),
        "live" => helper(3),
        "ct" => image(L_CONTROL),
        "depth" => image(L_FRAME_COUNT),
        "stack_limit" => image(L_STACK_LIMIT),
        "htag" => helper(L_HELPER_RECORD),
        "rt" => helper(L_RESULT_RECORD),
        "r1" => helper(L_RESULT_RECORD + 1),
        _ => {
            if let Some(i) = num("k") {
                return image(L_CONTROL + i);
            }
            if let Some(i) = num("top") {
                let d = w.pre_image[L_FRAME_COUNT] as usize;
                return (d >= 1)
                    .then(|| (true, IMAGE_SECTION_HEADER_LANES + (d - 1) * FRAME_LANES + i));
            }
            if let Some(i) = num("task") {
                let l = w.pre_helper[3] as usize;
                return (l >= 1).then(|| (false, L_TASKS + (l - 1) * TASK_LANES + i));
            }
            if let Some(i) = num("h") {
                return helper(L_HELPER_RECORD + i);
            }
            None
        }
    }
}

fn set_lane(w: &mut C6DispatchWitness, at: (bool, usize), v: u128) {
    if at.0 {
        w.pre_image[at.1] = v;
    } else {
        w.pre_helper[at.1] = v;
    }
}

fn get_lane(w: &C6DispatchWitness, at: (bool, usize)) -> u128 {
    if at.0 {
        w.pre_image[at.1]
    } else {
        w.pre_helper[at.1]
    }
}

fn ord_of(a: u128, b: u128) -> Ord3 {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => Ord3::Lt,
        std::cmp::Ordering::Equal => Ord3::Eq,
        std::cmp::Ordering::Greater => Ord3::Gt,
    }
}

/// Coordinated lane edit of `base` (root values `bv`) into class `target`, if
/// one lands there. PROG/DATA-derived roots are never edited: the caller
/// picks bases whose derived roots already agree with the class.
fn realize(
    pr: &Probe,
    p: &Partition,
    target: usize,
    base: &C6DispatchWitness,
    bv: &[u128],
) -> Option<(C6DispatchWitness, Vec<RuleIdC>)> {
    let names = &p.root_names;
    let c = &p.classes[target];
    let mut w = base.clone();
    let caps = base.source_params.geometry().caps;
    let other_of = |r: usize, cur: u128| -> u128 {
        if !p.consts[r].contains(&cur) {
            return cur;
        }
        let free = |hi: u128| (1..=hi).find(|v| !p.consts[r].contains(v)).unwrap_or(1);
        match names[r].as_str() {
            "depth" => free(caps.phys.max_frames as u128),
            "live" => free(caps.max_helper_tasks as u128),
            _ => p.consts[r].iter().max().map_or(1, |m| m + 1),
        }
    };
    // Index lanes first: depth and live position the top frame and top task.
    let mut order: Vec<(usize, Dom)> = c.vars.clone();
    order.sort_by_key(|(r, _)| !matches!(names[*r].as_str(), "depth" | "live"));
    for (r, d) in &order {
        let Some(at) = lane_of(&names[*r], &w) else {
            continue;
        };
        let v = match d {
            Dom::C(v) => *v,
            Dom::Other => other_of(*r, get_lane(&w, at)),
        };
        set_lane(&mut w, at, v);
    }
    let settable = |x: usize, w: &C6DispatchWitness| {
        !matches!(names[x].as_str(), "depth" | "live") && lane_of(&names[x], w).is_some()
    };
    for ((a, b), o) in &c.ords {
        let (mv, fixed, flip) = if settable(*a, &w) {
            (*a, *b, false)
        } else if settable(*b, &w) {
            (*b, *a, true)
        } else {
            return None;
        };
        let at = lane_of(&names[mv], &w)?;
        let f = match lane_of(&names[fixed], &w) {
            Some(fa) => get_lane(&w, fa),
            None => bv[fixed],
        };
        let want = match (o, flip) {
            (Ord3::Eq, _) => Ord3::Eq,
            (Ord3::Lt, false) | (Ord3::Gt, true) => Ord3::Lt,
            _ => Ord3::Gt,
        };
        let v = match want {
            Ord3::Eq => f,
            Ord3::Lt => f.checked_sub(1)?,
            Ord3::Gt => f + 1,
        };
        set_lane(&mut w, at, v);
    }
    let (values, selected) = evaluate(pr, &w).ok()?;
    (p.classify(&values) == target).then_some((w, selected))
}

struct Base {
    label: String,
    probe: usize,
    w: C6DispatchWitness,
    values: Vec<u128>,
}

/// Every real pre-state of every program, with its root values.
fn bases() -> (Vec<Probe>, Vec<Base>) {
    let mut probes = Vec::new();
    let mut out = Vec::new();
    for (label, source) in PROGRAMS {
        let r = run(source);
        let pr = probe(&witness(&r.steps[0].0, &r));
        for (i, (pre, _)) in r.steps.iter().enumerate() {
            let w = witness(pre, &r);
            let (values, _) = evaluate(&pr, &w).expect("real state");
            out.push(Base {
                label: format!("{label}#{i}"),
                probe: probes.len(),
                w,
                values,
            });
        }
        probes.push(pr);
    }
    (probes, out)
}

/// Directed raw-lane cases for every class no program reaches.
struct Directed {
    /// (class, base label, witness, concretely selected rules)
    cases: Vec<(usize, String, C6DispatchWitness, Vec<RuleIdC>)>,
    unrealized: Vec<usize>,
}

fn directed(p: &Partition, filter: &str) -> Directed {
    let reached = reached_classes(p);
    let (probes, bases) = bases();
    let names = &p.root_names;
    let mut cases = Vec::new();
    let mut unrealized = Vec::new();
    for target in 0..p.classes.len() {
        if reached.contains_key(&target) || !p.group_of(target).contains(filter) {
            continue;
        }
        let c = &p.classes[target];
        // Bases whose PROG/DATA-derived roots already lie in the class,
        // best-matching first.
        let mut ranked: Vec<(usize, &Base)> = bases
            .iter()
            .filter(|b| {
                c.vars.iter().all(|(r, d)| {
                    lane_of(&names[*r], &b.w).is_some() || p.abstract_of(*r, b.values[*r]) == *d
                })
            })
            .map(|b| {
                let score = c
                    .vars
                    .iter()
                    .filter(|(r, d)| p.abstract_of(*r, b.values[*r]) == *d)
                    .count()
                    + c.ords
                        .iter()
                        .filter(|((x, y), o)| ord_of(b.values[*x], b.values[*y]) == *o)
                        .count();
                (score, b)
            })
            .collect();
        ranked.sort_by(|x, y| y.0.cmp(&x.0));
        let hit = ranked.iter().take(24).find_map(|(_, b)| {
            realize(&probes[b.probe], p, target, &b.w, &b.values)
                .map(|(w, sel)| (b.label.clone(), w, sel))
        });
        match hit {
            Some((label, w, sel)) => cases.push((target, label, w, sel)),
            None => unrealized.push(target),
        }
    }
    Directed { cases, unrealized }
}

#[test]
fn dispatch_directed_cases_select_their_class_rule() {
    let r = run(LITERAL);
    let (_, p) = partition_for(&witness(&r.steps[0].0, &r));
    let d = directed(&p, "");
    for (class, from, _, selected) in &d.cases {
        assert_eq!(
            selected, &p.classes[*class].selected,
            "class {class} (from {from}): concrete selection differs from the class"
        );
    }
    let mut per_group: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    for (class, ..) in &d.cases {
        per_group.entry(p.group_of(*class)).or_default().0 += 1;
    }
    for class in &d.unrealized {
        per_group.entry(p.group_of(*class)).or_default().1 += 1;
    }
    let rules: BTreeSet<String> = d
        .cases
        .iter()
        .filter_map(|(c, ..)| p.class_rule(*c).map(|r| format!("{r:?}")))
        .collect();
    println!(
        "DISPATCH-DIRECTED {}",
        json!({
            "realized": d.cases.len(),
            "unrealized": d.unrealized.len(),
            "rules_with_directed_case": rules.len(),
            "per_group_realized_unrealized": per_group,
            "unrealized_ids": d.unrealized,
        })
    );
}

#[test]
#[ignore = "T2 directed cases under the SDK check: per rule one directed case accepted, every group mate and publication rule rejects it; DISPATCH_GROUP=<group substring> selects one budget key"]
fn dispatch_directed_cases_sdk() {
    let filter = std::env::var("DISPATCH_GROUP").unwrap_or_default();
    let r = run(LITERAL);
    let (_, p) = partition_for(&witness(&r.steps[0].0, &r));
    let d = directed(&p, &filter);
    let all = supported_rules();
    let publication = [
        RuleIdC::KAdminPublishValue,
        RuleIdC::KAdminPublishError,
        RuleIdC::KAdminPublishExhausted,
    ];
    // One representative directed case per rule (the first class realized).
    let mut seen = BTreeSet::new();
    let mut by_rule: BTreeMap<String, (RuleIdC, Vec<(String, C6DispatchWitness, bool)>)> =
        BTreeMap::new();
    let mut push = |rule: RuleIdC, label: String, w: &C6DispatchWitness, expect: bool| {
        by_rule
            .entry(format!("{rule:?}"))
            .or_insert((rule, Vec::new()))
            .1
            .push((label, w.clone(), expect));
    };
    let mut representatives = Vec::new();
    for (class, from, w, _) in &d.cases {
        let Some(rule) = p.class_rule(*class) else {
            continue;
        };
        if !seen.insert(format!("{rule:?}")) {
            continue;
        }
        let label = format!("class {class} from {from}");
        push(rule, label.clone(), w, true);
        let group = group_label(rule);
        for other in all
            .iter()
            .filter(|x| **x != rule && (group_label(**x) == group || publication.contains(x)))
        {
            push(*other, label.clone(), w, false);
        }
        representatives.push(json!({"class": class, "rule": format!("{rule:?}"), "base": from}));
    }
    let checks = sdk_checks(by_rule);
    println!(
        "DISPATCH-DIRECTED-SDK {}",
        json!({
            "group_filter": filter,
            "representatives": representatives.len(),
            "sdk_checks": checks,
            "cases": representatives,
        })
    );
}

#[test]
fn dispatch_rejects_mutated_pre_lanes() {
    let r = run(FACT);
    let mut circuits = Circuits::new();
    let mut mutated = 0;
    for (pre, fired) in r.steps.iter().take(40) {
        let w = witness(pre, &r);
        // Zero remaining work turns every non-publication step into P2.
        let mut fuel = w.clone();
        fuel.pre_image_mut()[L_REMAINING_WORK] = 0;
        if *fired != RuleIdC::KAdminPublishValue {
            assert!(
                !accepts(&mut circuits, *fired, &fuel),
                "{fired:?} with zero fuel"
            );
            assert!(accepts(
                &mut circuits,
                RuleIdC::KAdminPublishExhausted,
                &fuel
            ));
            mutated += 1;
        }
        // With no active helper, a forged result tag changes the selected group.
        if pre.image.helper.is_some() {
            continue;
        }
        let mut forged = w.clone();
        let rt = forged.pre_helper_mut()[L_RESULT_RECORD];
        forged.pre_helper_mut()[L_RESULT_RECORD] = if rt == 0 { 13 } else { 0 };
        assert!(
            !accepts(&mut circuits, *fired, &forged),
            "{fired:?} accepted with a forged result tag"
        );
        mutated += 1;
    }
    assert!(mutated > 30);
    // Non-running outcome: no rule is selected.
    let mut halted = witness(&r.steps[0].0, &r);
    halted.pre_image_mut()[L_OUTCOME] = 1;
    assert!(!accepts(&mut circuits, r.steps[0].1, &halted));
}

/// U023a diagnostic (SAS-014): one state x one rule. Reduced coverage; it
/// never satisfies the dispatch receipt.
#[test]
#[ignore = "diagnostic (SAS-014): one state x one rule; run alone under doctorwatson heavy"]
fn dispatch_diagnostic_one_state_one_rule() {
    let r = run(FACT);
    let (pre, fired) = &r.steps[0];
    let w = witness(pre, &r);
    let inputs = sym_inputs(&w);
    let mut per_rule = BTreeMap::new();
    let t = Instant::now();
    for rule in supported_rules() {
        let b = emit_c6_dispatch_block(rule, &inputs, "diag").expect("emit");
        per_rule.insert(format!("{rule:?}"), b.instructions.len());
    }
    let emit_all_s = t.elapsed().as_secs_f64();
    let t = Instant::now();
    let c = build_c6_dispatch_circuit(
        *fired,
        w.source_params().clone(),
        w.host_prefix.meta.len(),
        w.host_prefix.prog.len(),
        "diagone",
    )
    .expect("build");
    let build_s = t.elapsed().as_secs_f64();
    let t = Instant::now();
    let ok = check_c6_dispatch_circuit(&c, &w).is_ok();
    let check_s = t.elapsed().as_secs_f64();
    let sel = probe(&w);
    let t = Instant::now();
    let (_, selected) = evaluate(&sel, &w).expect("concrete");
    let concrete_s = t.elapsed().as_secs_f64();
    let counts: Vec<usize> = per_rule.values().copied().collect();
    println!(
        "DISPATCH-DIAG {}",
        json!({
            "format": "moriarty-sherlock-dispatch-diagnostic/1",
            "coverage": "reduced (SAS-014): one state x one rule for build and check; emission-only instruction counts for every supported rule; not an acceptance receipt",
            "program": "fact(3)",
            "state": 0,
            "rule": format!("{fired:?}"),
            "statement_lanes": c.statement_lane_count(),
            "ir_instructions": c.ir().instructions.len(),
            "build_seconds": build_s,
            "check_seconds": check_s,
            "check_accepts": ok,
            "emit_all_rules_seconds": emit_all_s,
            "rules": per_rule.len(),
            "instructions_min": counts.iter().min(),
            "instructions_max": counts.iter().max(),
            "instructions_per_rule": per_rule,
            "selector_instructions": sel.sel.ins.len(),
            "selector_concrete_eval_seconds": concrete_s,
            "selector_selected": selected.iter().map(|r| format!("{r:?}")).collect::<Vec<_>>(),
        })
    );
    assert!(ok);
}
