//! CallBind control fragment over real owner runs of factorial and tree fold.
//!
//! Positive: every (pre, post) pair the owner produced with an CallBind
//! rule passes that rule's circuit under `ProofPreimage::check`; enable = 0
//! accepts the identity pair. Rejection: mutated post lanes (control, frame,
//! next, env, charge, DATA, helper result), a mutated pre selection, and every
//! other CallBind control rule's circuit reject. Expected rules come from the
//! owner event stream, never from the emitter.
use std::collections::{BTreeMap, BTreeSet};

use super::super::c6_call_bind_step::CALL_BIND_RULES;
use super::*;
use mori_core::sherlock::helper_machine::c6_phys::{
    C6CodecBudgets, L_CONSUMED_WORK, L_CONTROL, L_ENV_ROOT, L_REFERENCE_CURSOR, L_REMAINING_WORK,
};
use mori_core::sherlock::helper_machine::c6_phys_helper::L_RESULT_RECORD;
use mori_core::sherlock::helper_machine::complete_observation::capture;
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::{OwnerEngine, StepResult};
use mori_core::sherlock_compile_artifact::draft_execution::draft_genesis;

const CONFIG: &str = include_str!("testdata/dispatch/config.json");
const FACT: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } }\n  entry fact(3);\n}\n";
const FOLD: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fold(t: Tree): UInt { match t { leaf(x) => x, branch(l, r) => fold(l) + fold(r) } }\n  entry fold(branch(leaf(4), leaf(5)));\n}\n";

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
    /// (pre, post, rule the owner fired from pre)
    steps: Vec<(HostSnapshot, HostSnapshot, RuleIdC)>,
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
        let pre = capture(&g.admitted, &image).expect("capture pre");
        match engine.step(&g.admitted, &mut image).expect("owner step") {
            StepResult::Terminal => break,
            StepResult::Event { event } => {
                let post = capture(&g.admitted, &image).expect("capture post");
                steps.push((pre, post, event.rule_id));
            }
        }
    }
    Run { cfg, steps }
}

struct Circuits(BTreeMap<String, C6CallBindCircuit>);

impl Circuits {
    fn accepts(&mut self, rule: RuleIdC, w: &C6CallBindWitness) -> bool {
        let key = format!("{rule:?}");
        let c = self.0.entry(key.clone()).or_insert_with(|| {
            build_c6_call_bind_circuit(
                rule,
                w.source_params.clone(),
                w.host_prefix.meta.len(),
                w.host_prefix.prog.len(),
                w.host_prefix.meta_bounds_frames_offset,
                &format!("callbind{}", key.to_lowercase()),
            )
            .unwrap_or_else(|e| panic!("build {key}: {e}"))
        });
        check_c6_call_bind_circuit(c, w).is_ok()
    }
}

/// Post-lane mutations every CallBind control rule must reject.
fn post_mutations(rule: RuleIdC, w: &C6CallBindWitness) -> Vec<(&'static str, C6CallBindWitness)> {
    let mut out = Vec::new();
    let mut bump = |label: &'static str, f: &dyn Fn(&mut C6CallBindWitness)| {
        let mut m = w.clone();
        f(&mut m);
        out.push((label, m));
    };
    bump("control tag", &|m| m.post_image[L_CONTROL] ^= 1);
    bump("control lane 2", &|m| m.post_image[L_CONTROL + 2] += 1);
    bump("remaining work", &|m| m.post_image[L_REMAINING_WORK] += 1);
    bump("consumed work", &|m| m.post_image[L_CONSUMED_WORK] += 1);
    bump("reference cursor", &|m| {
        m.post_image[L_REFERENCE_CURSOR] += 1
    });
    bump("env root", &|m| m.post_image[L_ENV_ROOT + 1] += 1);
    bump("data lane", &|m| {
        let n = m.post_data.len() - 1;
        m.post_data[n] += 1
    });
    bump("helper result", &|m| m.post_helper[L_RESULT_RECORD] ^= 1);
    if rule == RuleIdC::KMachineBoundValid {
        // ScheduleNext: next expression and the ReturnF frame tag.
        bump("next", &|m| m.post_image[L_CONTROL + 14] += 1);
        bump("frame tag", &|m| m.post_image[L_CONTROL + 1] += 1);
    } else {
        bump("control lane 1", &|m| m.post_image[L_CONTROL + 1] ^= 1);
        bump("args or body", &|m| m.post_image[L_CONTROL + 4] += 1);
    }
    out
}

fn exercise(source: &str, label: &str) -> BTreeSet<String> {
    let r = run(source);
    let mut circuits = Circuits(BTreeMap::new());
    let mut seen = BTreeSet::new();
    for (i, (pre, post, fired)) in r.steps.iter().enumerate() {
        if !CALL_BIND_RULES.contains(fired) {
            continue;
        }
        // Rejection checks on the first step of each rule and every 8th step;
        // the positive check runs on every step.
        let sampled = seen.insert(format!("{fired:?}")) || i % 8 == 0;
        let w = witness_from_host_pair(true, pre, post, &r.cfg, &limits()).expect("witness");
        assert!(
            circuits.accepts(*fired, &w),
            "{label} step {i}: owner pair for {fired:?} rejected"
        );
        if !sampled {
            continue;
        }
        for (what, m) in post_mutations(*fired, &w) {
            assert!(
                !circuits.accepts(*fired, &m),
                "{label} step {i}: {fired:?} accepted a mutated post ({what})"
            );
        }
        for other in CALL_BIND_RULES.iter().filter(|x| *x != fired) {
            assert!(
                !circuits.accepts(*other, &w),
                "{label} step {i}: {other:?} accepted the owner pair of {fired:?}"
            );
        }
        // Disabled: identity accepts, a real step does not.
        let off = witness_from_host_pair(false, pre, pre, &r.cfg, &limits()).expect("witness");
        assert!(
            circuits.accepts(*fired, &off),
            "{label} step {i}: disabled identity"
        );
        let mut off_step = w.clone();
        off_step.enable = false;
        assert!(
            !circuits.accepts(*fired, &off_step),
            "{label} step {i}: disabled accepted a state change"
        );
    }
    seen
}

#[test]
fn call_bind_accepts_owner_steps_and_rejects_mutations_on_factorial() {
    let seen = exercise(FACT, "fact(3)");
    assert!(!seen.is_empty(), "factorial fired no CallBind control rule");
}

#[test]
fn call_bind_accepts_owner_steps_and_rejects_mutations_on_tree_fold() {
    let seen = exercise(FOLD, "fold");
    assert!(!seen.is_empty(), "fold fired no CallBind control rule");
}

#[test]
fn call_bind_covers_every_rule_both_programs_fire() {
    // Which claimed rules the owner fires (circuits run in the two tests above).
    let mut seen = BTreeSet::new();
    for source in [FACT, FOLD] {
        for (_, _, fired) in run(source).steps {
            if CALL_BIND_RULES.contains(&fired) {
                seen.insert(format!("{fired:?}"));
            }
        }
    }
    let want: BTreeSet<String> = CALL_BIND_RULES.iter().map(|r| format!("{r:?}")).collect();
    assert_eq!(
        seen, want,
        "every claimed rule must have a positive owner step"
    );
}

#[test]
fn call_bind_rejects_mutated_pre_selection() {
    let r = run(FACT);
    let mut circuits = Circuits(BTreeMap::new());
    let mut checked = 0;
    for (pre, post, fired) in &r.steps {
        if !CALL_BIND_RULES.contains(fired) {
            continue;
        }
        let w = witness_from_host_pair(true, pre, post, &r.cfg, &limits()).expect("witness");
        // Zero fuel makes Core select publication (P2), not this rule.
        let mut fuel = w.clone();
        fuel.pre_image[L_REMAINING_WORK] = 0;
        assert!(!circuits.accepts(*fired, &fuel), "{fired:?} with zero fuel");
        // A pre lane the rule copies (args.len, function index, body).
        let lane = match fired {
            RuleIdC::KMachineInvokeFind => L_CONTROL + 3,
            RuleIdC::KMachineEnterBind => L_CONTROL + 2,
            _ => L_CONTROL + 5,
        };
        let mut moved = w.clone();
        moved.pre_image[lane] += 1;
        assert!(
            !circuits.accepts(*fired, &moved),
            "{fired:?} with a moved pre lane"
        );
        checked += 1;
    }
    assert!(checked > 5);
}
