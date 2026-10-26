//! Admin publish value fragment over real owner runs of factorial and
//! tree fold. Every run ends with exactly one `KAdminPublishValue` step
//! (Return at depth 0 with helper and result absent: outcome Running ->
//! Halted { value }, charge 0, every other lane preserved).
//!
//! Positive: the owner's publish pair passes the circuit under
//! `ProofPreimage::check`; enable = 0 accepts the identity pair.
//! Rejection: mutated post lanes (outcome, control, work, helper),
//! a mutated pre selection, and the disabled non-identity pair reject.
//! Expected rules come from the owner event stream, never from the
//! emitter.
use std::collections::{BTreeMap, BTreeSet};

use super::super::c6_admin_publish_step::ADMIN_PUBLISH_RULES;
use super::*;
use mori_core::sherlock::helper_machine::c6_phys::{
    C6CodecBudgets, L_CONSUMED_WORK, L_CONTROL, L_ENV_ROOT, L_HELPER_VISITS, L_HELPER_WORK,
    L_OUTCOME, L_REFERENCE_CURSOR, L_REMAINING_WORK,
};
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_HELPER_RECORD, L_RESULT_RECORD};
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

struct Circuits(BTreeMap<String, C6AdminPublishCircuit>);

impl Circuits {
    fn accepts(&mut self, rule: RuleIdC, w: &C6AdminPublishWitness) -> bool {
        let key = format!("{rule:?}");
        let c = self.0.entry(key.clone()).or_insert_with(|| {
            build_c6_admin_publish_circuit(
                rule,
                w.source_params.clone(),
                w.host_prefix.meta.len(),
                w.host_prefix.prog.len(),
                w.host_prefix.meta_bounds_frames_offset,
                &format!("adminpublish{}", key.to_lowercase()),
            )
            .unwrap_or_else(|e| panic!("build {key}: {e}"))
        });
        check_c6_admin_publish_circuit(c, w).is_ok()
    }
}

/// Post-lane mutations the publish rule must reject.
fn post_mutations(w: &C6AdminPublishWitness) -> Vec<(&'static str, C6AdminPublishWitness)> {
    let mut out = Vec::new();
    let mut bump = |label: &'static str, f: &dyn Fn(&mut C6AdminPublishWitness)| {
        let mut m = w.clone();
        f(&mut m);
        out.push((label, m));
    };
    bump("outcome tag", &|m| m.post_image[L_OUTCOME] ^= 1);
    bump("outcome value kind", &|m| m.post_image[L_OUTCOME + 1] ^= 1);
    bump("outcome value index", &|m| m.post_image[L_OUTCOME + 2] += 1);
    bump("control tag", &|m| m.post_image[L_CONTROL] ^= 1);
    bump("control value", &|m| m.post_image[L_CONTROL + 2] += 1);
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
    bump("helper kind", &|m| m.post_helper[L_HELPER_RECORD] ^= 1);
    bump("helper result", &|m| m.post_helper[L_RESULT_RECORD] ^= 1);
    bump("helper work", &|m| m.post_image[L_HELPER_WORK] += 1);
    bump("helper visits", &|m| m.post_image[L_HELPER_VISITS] += 1);
    out
}

fn exercise(source: &str, label: &str) -> BTreeSet<String> {
    let r = run(source);
    let mut circuits = Circuits(BTreeMap::new());
    let mut seen = BTreeSet::new();
    for (i, (pre, post, fired)) in r.steps.iter().enumerate() {
        if !ADMIN_PUBLISH_RULES.contains(fired) {
            continue;
        }
        seen.insert(format!("{fired:?}"));
        let w = witness_from_host_pair(true, pre, post, &r.cfg, &limits()).expect("witness");
        assert!(
            circuits.accepts(*fired, &w),
            "{label} step {i}: owner pair for {fired:?} rejected"
        );
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
fn admin_publish_value_accepts_owner_steps_and_rejects_mutations_on_factorial() {
    let seen = exercise(FACT, "fact(3)");
    assert!(!seen.is_empty(), "factorial fired no publish rule");
}

#[test]
fn admin_publish_value_accepts_owner_steps_and_rejects_mutations_on_tree_fold() {
    let seen = exercise(FOLD, "fold");
    assert!(!seen.is_empty(), "fold fired no publish rule");
}

#[test]
fn admin_publish_value_fires_exactly_once_per_program() {
    for source in [FACT, FOLD] {
        let n = run(source)
            .steps
            .iter()
            .filter(|(_, _, fired)| ADMIN_PUBLISH_RULES.contains(fired))
            .count();
        assert_eq!(n, 1, "publish value must fire exactly once per run");
    }
}

#[test]
fn admin_publish_value_rejects_mutated_pre_selection() {
    let r = run(FACT);
    let mut circuits = Circuits(BTreeMap::new());
    let mut checked = 0;
    for (pre, post, fired) in &r.steps {
        if !ADMIN_PUBLISH_RULES.contains(fired) {
            continue;
        }
        let w = witness_from_host_pair(true, pre, post, &r.cfg, &limits()).expect("witness");
        // An Error control selects publish error, not publish value.
        let mut err = w.clone();
        err.pre_image[L_CONTROL] = 9;
        assert!(
            !circuits.accepts(*fired, &err),
            "{fired:?} with error control"
        );
        // A present helper moves Core out of the P1 publication branch.
        let mut helper = w.clone();
        helper.pre_helper[L_HELPER_RECORD] = 1;
        assert!(
            !circuits.accepts(*fired, &helper),
            "{fired:?} with a helper present"
        );
        // A halted pre outcome is not Running.
        let mut halted = w.clone();
        halted.pre_image[L_OUTCOME] = 1;
        assert!(
            !circuits.accepts(*fired, &halted),
            "{fired:?} with a halted pre outcome"
        );
        checked += 1;
    }
    assert_eq!(checked, 1);
}
