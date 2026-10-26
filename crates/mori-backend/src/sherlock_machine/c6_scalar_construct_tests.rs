//! Scalar construct fragment over real owner runs that build values.
//!
//! Positive: every (pre, post) pair the owner produced with
//! `BHelperScalarConstruct` passes the circuit under
//! `ProofPreimage::check`; enable = 0 accepts the identity pair.
//! Rejection: mutated post lanes (control, work, DATA, helper staged /
//! phase), a mutated pre selection, and the disabled non-identity pair
//! reject. Expected rules come from the owner event stream, never from
//! the emitter. Leaf mode fires on `leaf(..)` (phase First), Branch on
//! `branch(..)` and Cons on `cons(..)` (both phase Second).
use std::collections::{BTreeMap, BTreeSet};

use super::super::c6_scalar_construct_step::SCALAR_CONSTRUCT_RULES;
use super::*;
use mori_core::sherlock::helper_machine::c6_phys::{
    C6CodecBudgets, L_CONSUMED_WORK, L_CONTROL, L_ENV_ROOT, L_HELPER_VISITS, L_HELPER_WORK,
    L_REFERENCE_CURSOR, L_REMAINING_WORK,
};
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_HELPER_RECORD, L_RESULT_RECORD};
use mori_core::sherlock::helper_machine::complete_observation::capture;
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::{
    HelperControlC, OwnerEngine, ScalarMode, StepResult,
};
use mori_core::sherlock_compile_artifact::draft_execution::draft_genesis;

const CONFIG: &str = include_str!("testdata/dispatch/config.json");
const FACT: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } }\n  entry fact(3);\n}\n";
const BUILD: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn build(): Tree { branch(leaf(1), leaf(2)) }\n  entry build();\n}\n";
const CONS: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn mk(): List { cons(1, cons(2, nil)) }\n  entry mk();\n}\n";

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

struct Circuits(BTreeMap<String, C6ScalarConstructCircuit>);

impl Circuits {
    fn accepts(&mut self, rule: RuleIdC, w: &C6ScalarConstructWitness) -> bool {
        let key = format!("{rule:?}");
        let c = self.0.entry(key.clone()).or_insert_with(|| {
            build_c6_scalar_construct_circuit(
                rule,
                w.source_params.clone(),
                w.host_prefix.meta.len(),
                w.host_prefix.prog.len(),
                w.host_prefix.meta_bounds_frames_offset,
                &format!("scalarconstruct{}", key.to_lowercase()),
            )
            .unwrap_or_else(|e| panic!("build {key}: {e}"))
        });
        check_c6_scalar_construct_circuit(c, w).is_ok()
    }
}

/// Post-lane mutations the Scalar construct rule must reject.
fn post_mutations(w: &C6ScalarConstructWitness) -> Vec<(&'static str, C6ScalarConstructWitness)> {
    let mut out = Vec::new();
    let mut bump = |label: &'static str, f: &dyn Fn(&mut C6ScalarConstructWitness)| {
        let mut m = w.clone();
        f(&mut m);
        out.push((label, m));
    };
    bump("control tag", &|m| m.post_image[L_CONTROL] ^= 1);
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
    bump("helper kind", &|m| m.post_helper[L_HELPER_RECORD] ^= 1);
    bump("helper mode", &|m| m.post_helper[L_HELPER_RECORD + 5] += 1);
    bump("helper phase", &|m| {
        m.post_helper[L_HELPER_RECORD + 13] += 1
    });
    bump("helper staged tag", &|m| {
        m.post_helper[L_HELPER_RECORD + 15] ^= 1
    });
    bump("helper staged payload", &|m| {
        m.post_helper[L_HELPER_RECORD + 16] += 1
    });
    bump("helper work", &|m| m.post_image[L_HELPER_WORK] += 1);
    bump("helper visits", &|m| m.post_image[L_HELPER_VISITS] += 1);
    out
}

fn exercise(source: &str, label: &str) -> BTreeSet<String> {
    let r = run(source);
    let mut circuits = Circuits(BTreeMap::new());
    let mut seen = BTreeSet::new();
    for (i, (pre, post, fired)) in r.steps.iter().enumerate() {
        if !SCALAR_CONSTRUCT_RULES.contains(fired) {
            continue;
        }
        let sampled = seen.insert(format!("{fired:?}")) || i % 8 == 0;
        let w = witness_from_host_pair(true, pre, post, &r.cfg, &limits()).expect("witness");
        assert!(
            circuits.accepts(*fired, &w),
            "{label} step {i}: owner pair for {fired:?} rejected"
        );
        if !sampled {
            continue;
        }
        for (what, m) in post_mutations(&w) {
            assert!(
                !circuits.accepts(*fired, &m),
                "{label} step {i}: {fired:?} accepted a mutated post ({what})"
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
fn scalar_construct_accepts_owner_steps_and_rejects_mutations_on_build() {
    let seen = exercise(BUILD, "build");
    assert!(!seen.is_empty(), "build fired no Scalar construct rule");
}

#[test]
fn scalar_construct_accepts_owner_steps_and_rejects_mutations_on_cons() {
    let seen = exercise(CONS, "cons");
    assert!(!seen.is_empty(), "cons fired no Scalar construct rule");
}

#[test]
fn scalar_construct_covers_leaf_branch_and_cons_modes() {
    // Modes of the claimed rule across the programs (circuits run above).
    let mut modes = BTreeSet::new();
    for source in [FACT, BUILD, CONS] {
        for (pre, _, fired) in run(source).steps {
            if !SCALAR_CONSTRUCT_RULES.contains(&fired) {
                continue;
            }
            let Some(HelperControlC::Scalar { mode, .. }) = pre.image.helper else {
                panic!("construct step without a Scalar helper");
            };
            modes.insert(format!("{mode:?}"));
        }
    }
    let want: BTreeSet<String> = [ScalarMode::Leaf, ScalarMode::Branch, ScalarMode::Cons]
        .iter()
        .map(|m| format!("{m:?}"))
        .collect();
    assert_eq!(modes, want, "Leaf, Branch and Cons modes must all fire");
}

#[test]
fn scalar_construct_rejects_mutated_pre_selection() {
    let r = run(BUILD);
    let mut circuits = Circuits(BTreeMap::new());
    let mut checked = 0;
    for (pre, post, fired) in &r.steps {
        if !SCALAR_CONSTRUCT_RULES.contains(fired) {
            continue;
        }
        let w = witness_from_host_pair(true, pre, post, &r.cfg, &limits()).expect("witness");
        // Zero fuel makes Core select exhaustion (P2), not this rule.
        let mut fuel = w.clone();
        fuel.pre_image[L_REMAINING_WORK] = 0;
        assert!(!circuits.accepts(*fired, &fuel), "{fired:?} with zero fuel");
        // Pre helper visits feed the charge.
        let mut visits = w.clone();
        visits.pre_image[L_HELPER_VISITS] += 1;
        assert!(
            !circuits.accepts(*fired, &visits),
            "{fired:?} with moved visits"
        );
        // A forged pre mode moves Core to another Scalar rule.
        let mut forged = w.clone();
        forged.pre_helper[L_HELPER_RECORD + 5] = 0;
        assert!(
            !circuits.accepts(*fired, &forged),
            "{fired:?} with a forged mode"
        );
        checked += 1;
    }
    assert!(checked > 0);
}
