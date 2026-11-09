//! Skeleton-coding pilot grader
//! (`research/skeleton-coding-20261108/pilot-protocol.md`).
//!
//! The grader compares a candidate C6 step emitter with the reference
//! emitter for the same component. Both are built with the same
//! LocalPublic statement (`c6_step_harness::build_step_circuit`), and both
//! are checked with the SDK's nonproving `ProofPreimage::check`.
//!
//! Step circuits have no private inputs: every lane is public, so the IR
//! relation is a predicate over the statement. A soundness finding is a
//! raw-statement mutant of an honest owner step that the reference rejects
//! and the candidate accepts (the candidate relation is larger than the
//! reference relation). A static finding is a statement lane that the
//! reference reads and the candidate never reads. The oracle is the
//! reference: a defect that the reference shares is not found.
//!
//! The stage is "IR preprocessing" (SAS-004): the ZKIR-to-PLONK compiler
//! and the stdlib gadgets are shared by all candidates and are out of scope.
//! Outcomes reuse [`crate::qa_mutation::MutantOutcome`]; only a
//! `ProofPreimage::check` refusal counts as `Rejected`.
use std::collections::{BTreeMap, BTreeSet};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::time::{Duration, Instant};

use midnight_curves::Fq;
use midnight_curves::pairing::group::ff::Field;
use midnight_transient_crypto::curve::Fr;
use midnight_zkir_v3::Instruction;
use mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets;
use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostConfig, C6HostLimits, CTX_LANES};
use mori_core::sherlock::helper_machine::complete_observation::{HostSnapshot, capture};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::{OwnerEngine, RuleIdC, StepResult};
use mori_core::sherlock_compile_artifact::draft_execution::draft_genesis;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{RngCore, SeedableRng};
use serde_json::{Value, json};

use crate::qa_mutation::MutantOutcome;

use super::c6_step_harness::{
    C6StepCircuit, C6StepWitness, build_step_circuit, check_step_statement,
    step_witness_from_host_pair,
};
use super::c6_step_kit::C6StepInputs;

pub(super) type EmitFn = fn(RuleIdC, &C6StepInputs, &str) -> Result<Vec<Instruction>, String>;

const CONFIG: &str = include_str!("testdata/dispatch/config.json");
const FACT: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } }\n  entry fact(3);\n}\n";
const FOLD: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fold(t: Tree): UInt { match t { leaf(x) => x, branch(l, r) => fold(l) + fold(r) } }\n  entry fold(branch(leaf(4), leaf(5)));\n}\n";
const BUILD: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn build(): Tree { branch(leaf(1), leaf(2)) }\n  entry build();\n}\n";
const CONS: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn mk(): List { cons(1, cons(2, nil)) }\n  entry mk();\n}\n";

/// The honest-trace programs: the union of the programs the six pilot
/// components' own tests run.
pub(super) const PROGRAMS: [(&str, &str); 4] = [
    ("fact3", FACT),
    ("fold", FOLD),
    ("build", BUILD),
    ("cons", CONS),
];

macro_rules! reference_emit {
    ($m:ident, $em:ident) => {
        |rule: RuleIdC, inputs: &C6StepInputs, scope: &str| -> Result<Vec<Instruction>, String> {
            let mut e = super::$m::$em::new(scope).map_err(|e| e.to_string())?;
            Ok(e.emit(rule, inputs)
                .map_err(|e| e.to_string())?
                .instructions)
        }
    };
}

/// One pilot component: a C6 step fragment with a uniform emitter interface.
pub(super) struct Component {
    pub name: &'static str,
    pub rules: &'static [RuleIdC],
    pub reference: EmitFn,
}

pub(super) fn components() -> Vec<Component> {
    use super::{
        c6_admin_publish_step as ap, c6_append_helper_step as ah, c6_call_bind_step as cb,
        c6_call_helper_step as ch, c6_eval_schedule_step as es, c6_scalar_construct_step as sc,
    };
    vec![
        Component {
            name: "c6_admin_publish",
            rules: &ap::ADMIN_PUBLISH_RULES,
            reference: reference_emit!(c6_admin_publish_step, C6AdminPublishEmitter),
        },
        Component {
            name: "c6_call_bind",
            rules: &cb::CALL_BIND_RULES,
            reference: reference_emit!(c6_call_bind_step, C6CallBindEmitter),
        },
        Component {
            name: "c6_scalar_construct",
            rules: &sc::SCALAR_CONSTRUCT_RULES,
            reference: reference_emit!(c6_scalar_construct_step, C6ScalarConstructEmitter),
        },
        Component {
            name: "c6_append_helper",
            rules: &ah::APPEND_HELPER_RULES,
            reference: reference_emit!(c6_append_helper_step, C6AppendHelperEmitter),
        },
        Component {
            name: "c6_call_helper",
            rules: &ch::CALL_HELPER_RULES,
            reference: reference_emit!(c6_call_helper_step, C6CallHelperEmitter),
        },
        Component {
            name: "c6_eval_schedule",
            rules: &es::EVAL_SCHEDULE_RULES,
            reference: reference_emit!(c6_eval_schedule_step, C6EvalScheduleEmitter),
        },
    ]
}

pub(super) fn component(name: &str) -> Option<Component> {
    components().into_iter().find(|c| c.name == name)
}

fn limits() -> C6HostLimits {
    C6HostLimits {
        max_total_lanes: 1 << 20,
        max_state_lanes: 1 << 20,
        max_metadata_bytes: 8192,
        max_string_bytes: 512,
        max_width_entries: 64,
    }
}

/// One owner step of a component rule, encoded as a step witness.
#[derive(Clone)]
pub(super) struct Honest {
    pub program: &'static str,
    pub step: usize,
    pub rule: RuleIdC,
    pub witness: C6StepWitness,
    /// The disabled identity pair of the same pre-state.
    pub disabled: C6StepWitness,
}

/// Owner steps of `rules` over [`PROGRAMS`], at most `per_rule` per rule
/// (the first ones in program order).
pub(super) fn honest_steps(rules: &[RuleIdC], per_rule: usize) -> Result<Vec<Honest>, String> {
    let mut out = Vec::new();
    let mut taken: BTreeMap<String, usize> = BTreeMap::new();
    for (program, source) in PROGRAMS {
        let g = draft_genesis(source, CONFIG).map_err(|f| {
            format!(
                "genesis {program}: {}",
                serde_json::to_string(&f).unwrap_or_default()
            )
        })?;
        let cfg = C6HostConfig::from_retained_layout(
            &g.layout,
            C6CodecBudgets {
                type_expr_nodes: 128,
                template_bytes: 4096,
            },
        );
        let engine = host_owner_engine();
        let mut image = g.image;
        let mut steps: Vec<(HostSnapshot, HostSnapshot, RuleIdC)> = Vec::new();
        loop {
            let pre = capture(&g.admitted, &image).map_err(|e| format!("capture pre: {e:?}"))?;
            match engine
                .step(&g.admitted, &mut image)
                .map_err(|e| format!("owner step: {e:?}"))?
            {
                StepResult::Terminal => break,
                StepResult::Event { event } => {
                    let post =
                        capture(&g.admitted, &image).map_err(|e| format!("capture post: {e:?}"))?;
                    steps.push((pre, post, event.rule_id));
                }
            }
        }
        for (i, (pre, post, fired)) in steps.iter().enumerate() {
            if !rules.contains(fired) {
                continue;
            }
            let n = taken.entry(format!("{fired:?}")).or_insert(0);
            if *n >= per_rule {
                continue;
            }
            *n += 1;
            let witness = step_witness_from_host_pair(true, pre, post, &cfg, &limits())?;
            let disabled = step_witness_from_host_pair(false, pre, pre, &cfg, &limits())?;
            out.push(Honest {
                program,
                step: i,
                rule: *fired,
                witness,
                disabled,
            });
        }
    }
    Ok(out)
}

/// The labelled statement layout of a witness: (section, length) in order.
pub(super) fn sections(w: &C6StepWitness) -> Vec<(&'static str, usize)> {
    let (m, p) = (w.host_prefix.meta.len(), w.host_prefix.prog.len());
    vec![
        ("enable", 1),
        ("host_schema", 1),
        ("carrier_version", 1),
        ("pre_meta", m),
        ("pre_prog", p),
        ("pre_ctx", CTX_LANES),
        ("pre_image", w.pre_image.len()),
        ("pre_data", w.pre_data.len()),
        ("pre_helper", w.pre_helper.len()),
        ("post_meta", m),
        ("post_prog", p),
        ("post_ctx", CTX_LANES),
        ("post_image", w.post_image.len()),
        ("post_data", w.post_data.len()),
        ("post_helper", w.post_helper.len()),
    ]
}

/// Mutation kinds over raw statement lanes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Kind {
    PlusOne,
    MinusOne,
    Zero,
    /// A uniformly random field element (seeded).
    Random,
    /// pre lane i and post lane i of the same section both +1: catches a
    /// relation that only checks preservation and not the pre guard.
    PairedPlusOne,
}

impl Kind {
    fn label(self) -> &'static str {
        match self {
            Kind::PlusOne => "plus_one",
            Kind::MinusOne => "minus_one",
            Kind::Zero => "zero",
            Kind::Random => "random",
            Kind::PairedPlusOne => "paired_plus_one",
        }
    }
}

/// One planned mutant: base witness, lane, kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Mutant {
    pub base: usize,
    pub disabled: bool,
    pub section: &'static str,
    pub offset: usize,
    pub lane: usize,
    pub kind: Kind,
}

/// Every (base, lane, kind) mutant of the statement layout `secs`, for
/// `bases` honest steps (each with an enabled and a disabled base).
pub(super) fn mutant_space(secs: &[(&'static str, usize)], bases: usize) -> Vec<Mutant> {
    let mut start = BTreeMap::new();
    let mut at = 0usize;
    for (s, n) in secs {
        start.insert(*s, (at, *n));
        at += n;
    }
    let mut out = Vec::new();
    for base in 0..bases {
        for disabled in [false, true] {
            for (s, n) in secs {
                let (s0, _) = start[s];
                for off in 0..*n {
                    for kind in [Kind::PlusOne, Kind::MinusOne, Kind::Zero, Kind::Random] {
                        out.push(Mutant {
                            base,
                            disabled,
                            section: s,
                            offset: off,
                            lane: s0 + off,
                            kind,
                        });
                    }
                    if let Some(rest) = s.strip_prefix("pre_") {
                        let post = format!("post_{rest}");
                        if start.contains_key(post.as_str()) {
                            out.push(Mutant {
                                base,
                                disabled,
                                section: s,
                                offset: off,
                                lane: s0 + off,
                                kind: Kind::PairedPlusOne,
                            });
                        }
                    }
                }
            }
        }
    }
    out
}

/// Seeded stratified sample of at most `cap` mutants: shuffle inside each
/// (disabled, section) stratum, then take round-robin across strata, so every
/// stratum receives mutants before any stratum receives a second share.
pub(super) fn sample(space: Vec<Mutant>, cap: Option<usize>, seed: u64) -> Vec<Mutant> {
    let Some(cap) = cap else { return space };
    if cap >= space.len() {
        return space;
    }
    let mut strata: BTreeMap<(bool, &'static str), Vec<Mutant>> = BTreeMap::new();
    for m in space {
        strata.entry((m.disabled, m.section)).or_default().push(m);
    }
    let mut rng = StdRng::seed_from_u64(seed);
    let mut lists: Vec<Vec<Mutant>> = strata
        .into_values()
        .map(|mut v| {
            v.shuffle(&mut rng);
            v.reverse();
            v
        })
        .collect();
    let mut out = Vec::with_capacity(cap);
    while out.len() < cap {
        let mut any = false;
        for l in lists.iter_mut() {
            if out.len() >= cap {
                break;
            }
            if let Some(m) = l.pop() {
                out.push(m);
                any = true;
            }
        }
        if !any {
            break;
        }
    }
    out
}

fn fr_plus(a: Fr, d: Fq) -> Fr {
    Fr(a.0 + d)
}

/// Apply `m` to the statement of its base. `None` when the mutant equals
/// the base (for example `Zero` on a lane that is already zero).
pub(super) fn apply(
    m: &Mutant,
    base: &[Fr],
    secs: &[(&'static str, usize)],
    rng: &mut StdRng,
) -> Option<Vec<Fr>> {
    let mut s = base.to_vec();
    let one = Fq::ONE;
    match m.kind {
        Kind::PlusOne => s[m.lane] = fr_plus(s[m.lane], one),
        Kind::MinusOne => s[m.lane] = fr_plus(s[m.lane], -one),
        Kind::Zero => s[m.lane] = Fr::from(0u64),
        Kind::Random => s[m.lane] = Fr(Fq::random(&mut *rng)),
        Kind::PairedPlusOne => {
            let rest = m.section.strip_prefix("pre_")?;
            let post = format!("post_{rest}");
            let mut at = 0usize;
            let mut post_lane = None;
            for (name, n) in secs {
                if *name == post.as_str() {
                    post_lane = Some(at + m.offset);
                }
                at += n;
            }
            let pl = post_lane?;
            s[m.lane] = fr_plus(s[m.lane], one);
            s[pl] = fr_plus(s[pl], one);
        }
    }
    if s == base { None } else { Some(s) }
}

fn panic_text(p: &(dyn std::any::Any + Send)) -> String {
    if let Some(s) = p.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = p.downcast_ref::<String>() {
        s.clone()
    } else {
        "non-string panic".into()
    }
}

/// Typed outcome of one `ProofPreimage::check`.
pub(super) fn verdict(c: &C6StepCircuit, st: Vec<Fr>, timeout: Option<Duration>) -> MutantOutcome {
    let t0 = Instant::now();
    let r = catch_unwind(AssertUnwindSafe(|| check_step_statement(c, st)));
    let el = t0.elapsed();
    let out = match r {
        Ok(Ok(_)) => MutantOutcome::Accepted,
        Ok(Err(e)) if e.starts_with("SDK ProofPreimage::check failed") => {
            MutantOutcome::Rejected { failures: 1 }
        }
        Ok(Err(e)) => MutantOutcome::SynthesisError(e),
        Err(p) => MutantOutcome::Panicked(panic_text(&*p)),
    };
    match timeout {
        Some(t) if el > t => MutantOutcome::TimedOut {
            elapsed_ms: el.as_millis(),
        },
        _ => out,
    }
}

/// Built circuits of one emitter, one per rule, with the instruction lists.
pub(super) struct Built {
    pub circuits: BTreeMap<String, C6StepCircuit>,
    pub instructions: BTreeMap<String, Vec<Instruction>>,
    pub errors: BTreeMap<String, String>,
}

pub(super) fn build_all(emit: EmitFn, tag: &str, honest: &[Honest]) -> Built {
    let mut b = Built {
        circuits: BTreeMap::new(),
        instructions: BTreeMap::new(),
        errors: BTreeMap::new(),
    };
    for h in honest {
        let key = format!("{:?}", h.rule);
        if b.circuits.contains_key(&key) || b.errors.contains_key(&key) {
            continue;
        }
        let w = &h.witness;
        let scope = format!("{tag}{}", key.to_lowercase());
        let mut captured = Vec::new();
        let r = catch_unwind(AssertUnwindSafe(|| {
            build_step_circuit(
                format!("skeleton_pilot_{tag}_{key}"),
                w.source_params.clone(),
                w.host_prefix.meta.len(),
                w.host_prefix.prog.len(),
                w.host_prefix.meta_bounds_frames_offset,
                &scope,
                |inputs| {
                    let ins = emit(h.rule, inputs, &format!("{scope}.step"))?;
                    captured = ins.clone();
                    Ok(ins)
                },
            )
        }));
        match r {
            Ok(Ok(c)) => {
                b.circuits.insert(key.clone(), c);
                b.instructions.insert(key, captured);
            }
            Ok(Err(e)) => {
                b.errors.insert(key, e);
            }
            Err(p) => {
                b.errors.insert(key, format!("panic: {}", panic_text(&*p)));
            }
        }
    }
    b
}

/// Statement lanes ("section.offset") that the body reads, and the number of
/// private-input instructions. Reads are found by the public identifier
/// strings `%<scope>.public.<section>.<i>` in each body instruction.
pub(super) fn reads(ins: &[Instruction]) -> (BTreeSet<String>, usize) {
    let mut out = BTreeSet::new();
    let mut private = 0;
    for i in ins {
        let v = serde_json::to_value(i).unwrap_or(Value::Null);
        let text = v.to_string();
        if text.contains("PrivateInput") || text.contains("private_input") {
            private += 1;
        }
        let mut rest = text.as_str();
        while let Some(at) = rest.find(".public.") {
            let tail = &rest[at + ".public.".len()..];
            let end = tail
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '.'))
                .unwrap_or(tail.len());
            out.insert(tail[..end].to_string());
            rest = &tail[end..];
        }
    }
    (out, private)
}

fn outcome_json(o: &MutantOutcome) -> Value {
    match o {
        MutantOutcome::Accepted => json!({"label": o.label()}),
        MutantOutcome::Rejected { failures } => json!({"label": o.label(), "failures": failures}),
        MutantOutcome::SynthesisError(m) | MutantOutcome::Panicked(m) => {
            json!({"label": o.label(), "message": m.chars().take(400).collect::<String>()})
        }
        MutantOutcome::TimedOut { elapsed_ms } => {
            json!({"label": o.label(), "elapsed_ms": elapsed_ms})
        }
    }
}

/// Grading parameters.
pub(super) struct Plan {
    pub per_rule: usize,
    pub max_mutants: Option<usize>,
    pub seed: u64,
    pub timeout: Option<Duration>,
    /// Wall budget for the whole grade; the mutant loop stops when it is spent
    /// and the receipt declares the shortfall (run < planned).
    pub wall_budget: Option<Duration>,
}

/// Grade `cand` against the component reference. Returns the summary
/// receipt and the per-mutant records (JSON lines).
pub(super) fn grade(
    c: &Component,
    cand: EmitFn,
    plan: &Plan,
) -> Result<(Value, Vec<Value>), String> {
    let t0 = Instant::now();
    let honest = honest_steps(c.rules, plan.per_rule)?;
    if honest.is_empty() {
        return Err(format!("{}: no owner step fires its rules", c.name));
    }
    let reference = build_all(c.reference, "ref", &honest);
    if !reference.errors.is_empty() {
        return Err(format!("reference build failed: {:?}", reference.errors));
    }
    let candidate = build_all(cand, "cand", &honest);
    let mut records = Vec::new();

    // Static: lanes the reference body reads that the candidate never reads.
    let mut static_findings = Vec::new();
    let mut private_inputs = 0;
    let mut ins_counts = BTreeMap::new();
    for (key, ref_ins) in &reference.instructions {
        let (rr, _) = reads(ref_ins);
        let ci = candidate.instructions.get(key);
        ins_counts.insert(
            key.clone(),
            json!({"reference": ref_ins.len(), "candidate": ci.map(|v| v.len())}),
        );
        if let Some(ci) = ci {
            let (cr, p) = reads(ci);
            private_inputs += p;
            for lane in rr.difference(&cr) {
                static_findings.push(json!({"rule": key, "lane": lane}));
            }
        }
    }

    // Completeness: honest enabled pairs and disabled identities.
    let mut honest_rows = Vec::new();
    let mut complete = candidate.errors.is_empty();
    for h in &honest {
        let key = format!("{:?}", h.rule);
        for (what, w) in [("enabled", &h.witness), ("disabled_identity", &h.disabled)] {
            let st = w.to_statement_frs();
            let r = verdict(&reference.circuits[&key], st.clone(), plan.timeout);
            let k = candidate
                .circuits
                .get(&key)
                .map(|cc| verdict(cc, st, plan.timeout));
            let ok = matches!(k, Some(MutantOutcome::Accepted));
            complete &= ok;
            honest_rows.push(json!({
                "program": h.program, "step": h.step, "rule": key, "pair": what,
                "reference": outcome_json(&r),
                "candidate": k.as_ref().map(outcome_json),
            }));
        }
    }

    // Raw-statement mutants.
    let secs = sections(&honest[0].witness);
    let space = mutant_space(&secs, honest.len());
    let space_len = space.len();
    let planned = sample(space, plan.max_mutants, plan.seed);
    let mut rng = StdRng::seed_from_u64(plan.seed ^ 0x5EED_5EED);
    let mut tally: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut by_section: BTreeMap<&'static str, (BTreeSet<usize>, usize, usize)> = BTreeMap::new();
    let mut findings = Vec::new();
    let mut stopped_by_wall_budget = false;
    for (id, m) in planned.iter().enumerate() {
        if plan.wall_budget.is_some_and(|b| t0.elapsed() > b) {
            stopped_by_wall_budget = true;
            break;
        }
        let h = &honest[m.base];
        let key = format!("{:?}", h.rule);
        let base = if m.disabled { &h.disabled } else { &h.witness }.to_statement_frs();
        let Some(st) = apply(m, &base, &secs, &mut rng) else {
            *tally.entry("identical_to_base").or_default() += 1;
            continue;
        };
        let r = verdict(&reference.circuits[&key], st.clone(), plan.timeout);
        let k = match candidate.circuits.get(&key) {
            Some(cc) => verdict(cc, st, plan.timeout),
            None => MutantOutcome::SynthesisError("candidate build failed".into()),
        };
        let class = match (&r, &k) {
            (MutantOutcome::Rejected { .. }, MutantOutcome::Accepted) => "soundness",
            (MutantOutcome::Accepted, MutantOutcome::Rejected { .. }) => "overconstraint",
            (MutantOutcome::Rejected { .. }, MutantOutcome::Rejected { .. }) => "agree_rejected",
            (MutantOutcome::Accepted, MutantOutcome::Accepted) => "agree_accepted",
            _ => "untested",
        };
        *tally.entry(class).or_default() += 1;
        let e = by_section.entry(m.section).or_default();
        e.0.insert(m.offset);
        e.1 += 1;
        if class == "soundness" {
            e.2 += 1;
        }
        let rec = json!({
            "id": id, "rule": key, "program": h.program, "step": h.step,
            "base": if m.disabled { "disabled_identity" } else { "enabled" },
            "section": m.section, "offset": m.offset, "lane": m.lane, "kind": m.kind.label(),
            "reference": outcome_json(&r), "candidate": outcome_json(&k), "class": class,
        });
        if class == "soundness" && findings.len() < 50 {
            findings.push(rec.clone());
        }
        records.push(rec);
    }
    let run = records.len();
    let soundness = tally.get("soundness").copied().unwrap_or(0);
    let untested = tally.get("untested").copied().unwrap_or(0);
    let coverage: Vec<Value> = secs
        .iter()
        .map(|(s, n)| {
            let (lanes, cnt, sf) = by_section.get(s).cloned().unwrap_or_default();
            json!({"section": s, "lanes": n, "lanes_mutated": lanes.len(), "mutants": cnt, "soundness": sf})
        })
        .collect();
    let built = candidate.errors.is_empty();
    let sound = built && soundness == 0 && static_findings.is_empty() && private_inputs == 0;
    let summary = json!({
        "schema": "skeleton-pilot-grade/1",
        "component": c.name,
        "stage": "ir_preprocessing: SDK ProofPreimage::check (nonproving); compiler and stdlib out of scope",
        "oracle": "reference emitter verdict on the same raw statement",
        "plan": {"per_rule": plan.per_rule, "max_mutants": plan.max_mutants, "seed": plan.seed,
                 "timeout_ms": plan.timeout.map(|t| t.as_millis()),
                 "wall_budget_s": plan.wall_budget.map(|t| t.as_secs())},
        "build": {"ok": built, "errors": candidate.errors, "instructions": ins_counts},
        "honest": honest_rows,
        "complete": complete,
        "static": {"private_inputs": private_inputs, "lanes_read_by_reference_not_candidate": static_findings},
        "mutants": {"space": space_len, "planned": planned.len(), "run": run,
                    "stopped_by_wall_budget": stopped_by_wall_budget,
                    "coverage_fraction": if space_len == 0 { 0.0 } else { run as f64 / space_len as f64 },
                    "by_class": tally, "by_section": coverage},
        "findings": findings,
        "verdict": {"built": built, "complete": complete, "sound": sound,
                    "primary_pass": sound && complete, "soundness_mutants": soundness,
                    "untested_mutants": untested},
        "wall_seconds": t0.elapsed().as_secs_f64(),
    });
    Ok((summary, records))
}

/// Write a summary and its JSON-lines records under `dir`.
pub(super) fn write_receipt(dir: &str, summary: &Value, records: &[Value]) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let mut s = serde_json::to_string_pretty(summary).map_err(|e| e.to_string())?;
    s.push('\n');
    std::fs::write(format!("{dir}/grade.json"), s).map_err(|e| e.to_string())?;
    let mut lines = String::new();
    for r in records {
        lines.push_str(&r.to_string());
        lines.push('\n');
    }
    std::fs::write(format!("{dir}/mutants.jsonl"), lines).map_err(|e| e.to_string())
}

fn env_plan() -> Plan {
    let get = |k: &str| std::env::var(k).ok().filter(|v| !v.is_empty());
    Plan {
        per_rule: get("SKELETON_PILOT_PER_RULE")
            .and_then(|v| v.parse().ok())
            .unwrap_or(2),
        max_mutants: get("SKELETON_PILOT_MAX_MUTANTS").and_then(|v| v.parse().ok()),
        seed: get("SKELETON_PILOT_SEED")
            .and_then(|v| v.parse().ok())
            .unwrap_or(20261108),
        timeout: get("SKELETON_PILOT_TIMEOUT_MS")
            .and_then(|v| v.parse().ok())
            .map(Duration::from_millis),
        wall_budget: get("SKELETON_PILOT_WALL_BUDGET_S")
            .and_then(|v| v.parse().ok())
            .map(Duration::from_secs),
    }
}

/// Grader entry point. Driven by `scripts/qa/skeleton-pilot.py grade`:
/// SKELETON_PILOT_COMPONENT names the component, SKELETON_PILOT_TARGET is
/// `candidate` (the installed `skeleton_pilot_cand::emit`) or `reference`
/// (the reference graded against itself), SKELETON_PILOT_OUT the receipt dir.
#[test]
#[ignore = "pilot grader; run through scripts/qa/skeleton-pilot.py grade"]
fn skeleton_pilot_grade() {
    let name = std::env::var("SKELETON_PILOT_COMPONENT").expect("SKELETON_PILOT_COMPONENT");
    let out = std::env::var("SKELETON_PILOT_OUT").expect("SKELETON_PILOT_OUT");
    let target = std::env::var("SKELETON_PILOT_TARGET").unwrap_or_else(|_| "candidate".into());
    let c = component(&name).unwrap_or_else(|| panic!("unknown component {name}"));
    let cand: EmitFn = match target.as_str() {
        "candidate" => super::skeleton_pilot_cand::emit,
        "reference" => c.reference,
        other => panic!("unknown target {other}"),
    };
    let (mut summary, records) = grade(&c, cand, &env_plan()).expect("grade");
    summary["target"] = json!(target);
    write_receipt(&out, &summary, &records).expect("write receipt");
    println!(
        "SKELETON_PILOT_VERDICT {}",
        serde_json::to_string(&summary["verdict"]).unwrap_or_default()
    );
}

/// Weakened admin publish: the post outcome tag lane is replaced by the
/// constant 1 before emission, so the candidate never reads it.
fn weakened_admin_publish(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<Vec<Instruction>, String> {
    use mori_core::sherlock::helper_machine::c6_phys::L_OUTCOME;
    let mut inputs = inputs.clone();
    inputs.post_image[L_OUTCOME] = midnight_zkir_v3::ir::Operand::Immediate(Fr::from(1u64));
    let mut e = super::c6_admin_publish_step::C6AdminPublishEmitter::new(scope)
        .map_err(|e| e.to_string())?;
    Ok(e.emit(rule, &inputs)
        .map_err(|e| e.to_string())?
        .instructions)
}

#[test]
fn mutant_space_covers_every_section_and_sample_is_stratified() {
    let secs = vec![
        ("enable", 1),
        ("pre_image", 3),
        ("post_image", 3),
        ("pre_data", 2),
    ];
    let space = mutant_space(&secs, 1);
    // 9 lanes x 4 kinds, plus 3 paired image lanes (pre_data has no post
    // section here), for the enabled and the disabled base.
    assert_eq!(space.len(), 2 * (9 * 4 + 3));
    let a = sample(space.clone(), Some(8), 7);
    let b = sample(space.clone(), Some(8), 7);
    assert_eq!(a, b, "sampling is deterministic for a seed");
    let strata: BTreeSet<(bool, &str)> = a.iter().map(|m| (m.disabled, m.section)).collect();
    assert_eq!(
        strata.len(),
        8,
        "8 strata, one mutant each before any repeat"
    );
    assert_eq!(sample(space.clone(), None, 7).len(), space.len());
}

#[test]
fn apply_changes_exactly_the_planned_lanes() {
    let secs = vec![("enable", 1), ("pre_image", 2), ("post_image", 2)];
    let base: Vec<Fr> = (0..5u64).map(Fr::from).collect();
    let mut rng = StdRng::seed_from_u64(1);
    let m = |section, offset, lane, kind| Mutant {
        base: 0,
        disabled: false,
        section,
        offset,
        lane,
        kind,
    };
    let p = apply(
        &m("pre_image", 1, 2, Kind::PairedPlusOne),
        &base,
        &secs,
        &mut rng,
    )
    .unwrap();
    assert_eq!(p[2], Fr::from(3u64));
    assert_eq!(p[4], Fr::from(5u64));
    assert_eq!(p[3], base[3]);
    assert!(apply(&m("enable", 0, 0, Kind::Zero), &base, &secs, &mut rng).is_none());
    let r = apply(&m("post_image", 0, 3, Kind::Random), &base, &secs, &mut rng).unwrap();
    assert_ne!(r[3], base[3]);
    let _ = rng.next_u64();
}

/// The reference graded against itself has no finding and is complete; a
/// weakened emitter that drops the post outcome tag is caught by the static
/// lane check and by a directed raw-statement mutant (the sampled mutants may
/// miss it: SAS-014). Circuit test: run it
/// through `scripts/doctorwatson heavy` with RUST_TEST_THREADS=1.
#[test]
fn grader_finds_dropped_constraint_and_clears_reference() {
    let c = component("c6_admin_publish").unwrap();
    let plan = Plan {
        per_rule: 1,
        max_mutants: Some(120),
        seed: 3,
        timeout: None,
        wall_budget: None,
    };
    let (s, _) = grade(&c, c.reference, &plan).unwrap();
    assert_eq!(s["verdict"]["primary_pass"], json!(true), "{s}");
    assert_eq!(s["verdict"]["soundness_mutants"], json!(0));
    let (w, recs) = grade(&c, weakened_admin_publish, &plan).unwrap();
    // The weakened emitter pins post outcome to Halted, so it accepts the
    // enabled pair and rejects the disabled identity pair (post outcome there
    // is Running): incomplete by construction, and unsound by the static check.
    let pair = |what: &str| {
        w["honest"]
            .as_array()
            .unwrap()
            .iter()
            .find(|h| h["pair"] == json!(what))
            .map(|h| h["candidate"]["label"].clone())
    };
    assert_eq!(pair("enabled"), Some(json!("accepted")), "{w}");
    assert_eq!(pair("disabled_identity"), Some(json!("rejected")), "{w}");
    assert_eq!(w["verdict"]["sound"], json!(false));
    let lanes = w["static"]["lanes_read_by_reference_not_candidate"].to_string();
    assert!(lanes.contains("post_image."), "{lanes}");
    let sampled_hit = recs.iter().any(|r| r["class"] == json!("soundness"));
    eprintln!("weakened: sampled soundness mutants found: {sampled_hit}");
    // Directed: post outcome tag + 1 on the honest step.
    let tag = mori_core::sherlock::helper_machine::c6_phys::L_OUTCOME;
    let honest = honest_steps(c.rules, 1).unwrap();
    let secs = sections(&honest[0].witness);
    let lane = secs
        .iter()
        .take_while(|(n, _)| *n != "post_image")
        .map(|(_, n)| n)
        .sum::<usize>()
        + tag;
    let r = build_all(c.reference, "ref", &honest);
    let k = build_all(weakened_admin_publish, "cand", &honest);
    let key = format!("{:?}", honest[0].rule);
    let mut st = honest[0].witness.to_statement_frs();
    st[lane] = fr_plus(st[lane], Fq::ONE);
    assert!(matches!(
        verdict(&r.circuits[&key], st.clone(), None),
        MutantOutcome::Rejected { .. }
    ));
    assert_eq!(
        verdict(&k.circuits[&key], st, None),
        MutantOutcome::Accepted
    );
}
