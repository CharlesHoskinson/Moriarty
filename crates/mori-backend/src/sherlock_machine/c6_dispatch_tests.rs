//! Dispatch owner over real owner runs of factorial and tree fold.
//!
//! Positive: at every pre-state of both runs, the dispatch circuit of the
//! rule the owner actually fired passes `ProofPreimage::check`.
//! Rejection: the circuit of every other supported rule rejects the same
//! pre-state (all supported rules on sampled states, the fired rule's group
//! and site mates on every state), and mutated pre lanes reject the fired
//! rule's circuit. Expected rules come from the owner event stream, never
//! from the dispatch emitter.
use std::collections::BTreeMap;

use super::super::c6_dispatch_step::{dispatch_supported, supported_rules};
use super::*;
use mori_core::sherlock::helper_machine::c6_phys::{C6CodecBudgets, L_REMAINING_WORK};
use mori_core::sherlock::helper_machine::c6_phys_helper::L_RESULT_RECORD;
use mori_core::sherlock::helper_machine::complete_observation::capture;
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::{OwnerEngine, StepResult};
use mori_core::sherlock_compile_artifact::draft_execution::draft_genesis;

/// Small admitted bounds (16 frames, 48 data/env rows, 8 tasks) that fit both runs.
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
    fn get(&mut self, rule: RuleIdC, w: &C6DispatchWitness) -> &C6DispatchCircuit {
        let key = format!("{rule:?}");
        self.by_rule.entry(key.clone()).or_insert_with(|| {
            build_c6_dispatch_circuit(
                rule,
                w.source_params().clone(),
                w.host_prefix.meta.len(),
                w.host_prefix.prog.len(),
                &format!("dispatch{}", key.to_lowercase()),
            )
            .unwrap_or_else(|e| panic!("build {key}: {e}"))
        })
    }
}

fn accepts(c: &mut Circuits, rule: RuleIdC, w: &C6DispatchWitness) -> bool {
    check_c6_dispatch_circuit(c.get(rule, w), w).is_ok()
}

fn exercise(source: &str, label: &str) -> usize {
    let r = run(source);
    assert!(!r.steps.is_empty());
    let all = supported_rules();
    let mut circuits = Circuits {
        by_rule: BTreeMap::new(),
    };
    let mut checks = 0usize;
    for (i, (pre, fired)) in r.steps.iter().enumerate() {
        assert!(
            dispatch_supported(*fired),
            "{label}: fired {fired:?} not supported"
        );
        let w = witness_from_host_snapshot(true, pre, &r.cfg, &limits()).expect("witness");
        assert!(
            accepts(&mut circuits, *fired, &w),
            "{label} step {i}: fired rule {fired:?} rejected by its dispatch circuit"
        );
        // Every supported rule on sampled states; the fired rule's neighbours
        // (previous and next fired rules) on every state.
        let mut others: Vec<RuleIdC> = if i % 16 == 0 {
            all.clone()
        } else {
            let mut v = Vec::new();
            if i > 0 {
                v.push(r.steps[i - 1].1);
            }
            if let Some((_, next)) = r.steps.get(i + 1) {
                v.push(*next);
            }
            v.extend([
                RuleIdC::KAdminPublishValue,
                RuleIdC::KAdminPublishExhausted,
                RuleIdC::BBookkeepingTaskRelease,
            ]);
            v
        };
        others.retain(|x| x != fired);
        for other in others {
            checks += 1;
            assert!(
                !accepts(&mut circuits, other, &w),
                "{label} step {i}: {other:?} accepted although the owner fired {fired:?}"
            );
        }
        // Disabled: enable = 0 accepts any rule's circuit (identity).
        let off = witness_from_host_snapshot(false, pre, &r.cfg, &limits()).expect("witness");
        assert!(accepts(&mut circuits, RuleIdC::KAdminPublishError, &off));
    }
    checks
}

#[test]
fn dispatch_selects_exactly_the_fired_rule_on_factorial() {
    assert!(exercise(FACT, "fact(3)") > 0);
}

#[test]
fn dispatch_selects_exactly_the_fired_rule_on_tree_fold() {
    assert!(exercise(FOLD, "fold") > 0);
}

#[test]
fn dispatch_rejects_mutated_pre_lanes() {
    let r = run(FACT);
    let mut circuits = Circuits {
        by_rule: BTreeMap::new(),
    };
    let mut mutated = 0;
    for (pre, fired) in r.steps.iter().take(40) {
        let w = witness_from_host_snapshot(true, pre, &r.cfg, &limits()).expect("witness");
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
}
