//! Genuine-owner tests for `c6_scalar_exit_consumer` (source-only, UNRUN).
//!
//! Every fixture is admitted and stepped only through the real public
//! owner (`host_owner_engine().admit` / `admit_run`) and the real
//! `complete_observation::record_step` bounded search to the real
//! Scalar Done staged pre-state fixed by the Core scalar fragment and
//! the SI-21 staged denotation. The selected step is then taken ONCE
//! through `record_scalar_exit_step` on the selected image. For full
//! snapshot/event assertions a second image is obtained by an
//! identical fresh admission + replay (the real `MachineImage` type
//! carries no `Clone` impl in the supplied Core, so this deterministic
//! replay is the clone-equivalent), proven equal at the selected
//! pre-state and stepped exactly once through the raw `record_step`.
//! No pending helper/Done/staged image is manufactured, no private
//! engine state or admission guard is mutated, and no counter/config
//! is adjusted after admission to force an owner path.

use super::super::c6_push_step::C6PushStatementBindingMode;
use super::{
    C6ScalarExitConsumerError, C6ScalarExitStepWitness, build_c6_scalar_exit_step_circuit,
    capture_step_error_label, check_c6_scalar_exit_step_circuit, record_scalar_exit_step,
    violation_label, witness_from_host_pair, witness_from_owner_record,
};
use mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets;
use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostConfig, C6HostLimits};
use mori_core::sherlock::helper_machine::complete_observation::{
    CompleteStepRecord, DataFieldsSnapshot, DataSnapshot, HostSnapshot, LayoutSnapshot,
    ObservedStep, capture, record_step as owner_record,
};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::*;

fn tooling_label(e: &Tooling) -> String {
    match e {
        Tooling::AdmissionRejected {
            reason,
            detail,
            locator,
        } => format!("admission rejected: reason={reason:?}, detail={detail}, locator={locator:?}"),
        Tooling::CapacityNotAdmitted {
            kind,
            needed,
            limit,
            locator,
        } => format!(
            "capacity not admitted: kind={kind:?}, needed={needed:?}, limit={limit}, locator={locator:?}"
        ),
        Tooling::UnsupportedFeature { feature, locator } => {
            format!("unsupported feature: feature={feature:?}, locator={locator:?}")
        }
    }
}

fn bounds(max_hw: u128, data_nodes: usize, data_fields: usize) -> BoundsC {
    BoundsC {
        code_rows: 64,
        function_rows: 8,
        code_child_slots: 64,
        parameter_slots: 16,
        pattern_rows: 16,
        match_plan_rows: 16,
        match_arm_rows: 16,
        type_rows: 16,
        constructor_rows: 16,
        constructor_field_slots: 16,
        max_arity: 4,
        constant_nodes: 64,
        constant_field_slots: 64,
        data_nodes,
        data_field_slots: data_fields,
        environment_cells: 16,
        frames: 16,
        helper_tasks: 4,
        serialization_bytes: 1_000_000,
        max_reference_work: 1_000_000,
        max_helper_work: max_hw,
        step_cap: 1_000_000,
    }
}
fn capacities() -> LayoutCapacitiesC {
    LayoutCapacitiesC {
        name_rows: 32,
        name_bytes: 512,
        occurrence_path_slots: 64,
        value_slots: 512,
    }
}
fn layout(b: BoundsC) -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("lv".into(), "1".into()),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds: b,
        capacities: capacities(),
        ext_capacities: None,
        boundary_capacities_present: false,
    }
}
fn cfg(b: BoundsC) -> C6HostConfig {
    C6HostConfig {
        layout: LayoutSnapshot {
            layout_version: ("lv".into(), "1".into()),
            field_id: None,
            widths: None,
            scalar_layout_id: None,
            bounds: b,
            capacities: capacities(),
            boundary_capacities_present: false,
        },
        ext: None,
        budgets: C6CodecBudgets {
            type_expr_nodes: 256,
            template_bytes: 4096,
        },
    }
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Want {
    U128(u128),
    Bool(bool),
    Leaf,
    Cons,
    Branch,
    Selector(BoolSelector),
    TypeTest(bool),
}
impl Want {
    fn delta(&self) -> (usize, usize) {
        match self {
            Want::U128(_) | Want::Bool(_) => (1, 0),
            Want::Leaf => (1, 1),
            Want::Cons | Want::Branch => (1, 2),
            Want::Selector(_) | Want::TypeTest(_) => (0, 0),
        }
    }
    fn matches(&self, s: ScalarStaged) -> bool {
        match (self, s) {
            (Want::U128(n), ScalarStaged::U128Value { n: m }) => n == &m,
            (Want::Bool(b), ScalarStaged::BoolValue { b: m }) => b == &m,
            (
                Want::Leaf,
                ScalarStaged::PendingNode {
                    tag: NodeKind::Leaf,
                },
            ) => true,
            (
                Want::Cons,
                ScalarStaged::PendingNode {
                    tag: NodeKind::Cons,
                },
            ) => true,
            (
                Want::Branch,
                ScalarStaged::PendingNode {
                    tag: NodeKind::Branch,
                },
            ) => true,
            (Want::Selector(x), ScalarStaged::Selector { sel }) => x == &sel,
            (Want::TypeTest(x), ScalarStaged::TypeTest { ok }) => x == &ok,
            _ => false,
        }
    }
}

fn lit_u(n: u128) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::UInt {
            n: i128_class::KIntC::InRange(n),
        },
    }
}
fn lit_b(b: bool) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::Bool { b },
    }
}
fn tree_leaf(n: u128) -> CoreTree {
    CoreTree::Leaf {
        n: i128_class::KIntC::InRange(n),
    }
}

/// Fixture programs: the denotation is fixed by the Core scalar entry
/// (RetRight Binary, RetUnary Unary, RetIf TestBool, RetCall TestType).
fn branch_program(w: &Want) -> CoreProgram {
    let entry = match w {
        Want::U128(_) => CoreExpr::Binary {
            op: BinaryOp::Add,
            left: Box::new(lit_u(40)),
            right: Box::new(lit_u(2)),
        },
        Want::Bool(_) => CoreExpr::Binary {
            op: BinaryOp::Lt,
            left: Box::new(lit_u(2)),
            right: Box::new(lit_u(3)),
        },
        Want::Leaf => CoreExpr::Unary {
            op: UnaryOp::Leaf,
            operand: Box::new(lit_u(9)),
        },
        Want::Cons => CoreExpr::Binary {
            op: BinaryOp::Cons,
            left: Box::new(lit_u(7)),
            right: Box::new(CoreExpr::Literal {
                value: CoreValue::List {
                    elements: vec![i128_class::KIntC::InRange(3)],
                },
            }),
        },
        Want::Branch => CoreExpr::Binary {
            op: BinaryOp::Branch,
            left: Box::new(CoreExpr::Literal {
                value: CoreValue::Tree { tree: tree_leaf(1) },
            }),
            right: Box::new(CoreExpr::Literal {
                value: CoreValue::Tree {
                    tree: CoreTree::Branch {
                        left: Box::new(tree_leaf(2)),
                        right: Box::new(tree_leaf(3)),
                    },
                },
            }),
        },
        Want::Selector(sel) => CoreExpr::If {
            cond: Box::new(match sel {
                BoolSelector::NotBool => lit_u(7),
                BoolSelector::True => lit_b(true),
                BoolSelector::False => lit_b(false),
            }),
            then_expr: Box::new(lit_u(1)),
            else_expr: Box::new(lit_u(0)),
        },
        Want::TypeTest(ok) => {
            return CoreProgram {
                functions: vec![CoreFunction {
                    name: "f".into(),
                    parameters: Vec::new(),
                    return_type: SortTag::UInt,
                    body: if *ok { lit_u(42) } else { lit_b(true) },
                }],
                entry: CoreExpr::Call {
                    name: "f".into(),
                    args: Vec::new(),
                },
            };
        }
    };
    CoreProgram {
        functions: Vec::new(),
        entry,
    }
}

/// Fresh admission plus bounded real `record_step` search to the real
/// selected Scalar Done pre-state. R5 respected: `initial_fuel` (64)
/// is `<= layout.capacities.value_slots` (512).
fn reach(p: &CoreProgram, b: BoundsC, w: &Want) -> (Admitted, MachineImage, HostSnapshot) {
    let engine = host_owner_engine();
    let admitted = engine
        .admit(p, &b, &layout(b))
        .unwrap_or_else(|e| panic!("admit {w:?}: {}", tooling_label(&e)));
    let mut image = engine
        .admit_run(
            &admitted,
            &RunIntC::Nat(64),
            &RunIntC::Nat(8),
            &FinanceInput::Absent,
        )
        .unwrap_or_else(|e| panic!("genesis {w:?}: {}", tooling_label(&e)));
    for _ in 0..512 {
        let step = owner_record(&admitted, &mut image)
            .unwrap_or_else(|e| panic!("search step: {}", capture_step_error_label(&e)));
        let after = match &step {
            ObservedStep::Event(r) => &r.after,
            ObservedStep::Terminal { after, .. } => panic!(
                "terminal before selected state {w:?}: {:?}",
                after.image.outcome
            ),
        };
        if let Some(HelperControlC::Scalar {
            phase: ScalarPhase::Done,
            staged,
            ..
        }) = &after.image.helper
        {
            if w.matches(*staged) {
                let pre = capture(&admitted, &image)
                    .unwrap_or_else(|e| panic!("capture selected pre: {}", violation_label(&e)));
                assert_eq!(&pre, after, "capture must equal the searched pre-state");
                return (admitted, image, pre);
            }
        }
    }
    panic!("owner must reach selected Scalar Done state {w:?}")
}

fn replay_record(p: &CoreProgram, b: BoundsC, w: &Want) -> CompleteStepRecord {
    let (admitted, mut image, _) = reach(p, b, w);
    match owner_record(&admitted, &mut image) {
        Ok(ObservedStep::Event(r)) => r,
        Ok(ObservedStep::Terminal { .. }) => panic!("selected step must be an event {w:?}"),
        Err(_) => panic!("selected raw record step must succeed {w:?}"),
    }
}

fn assert_applied(r: &CompleteStepRecord, w: &Want) {
    let (pre, post, e) = (&r.before.image, &r.after.image, &r.event);
    let (dn, df) = w.delta();
    assert_eq!(e.rule_id, RuleIdC::BHelperScalarExit);
    assert_eq!(e.event_kind, EventKind::HelperExit);
    assert_eq!(e.disposition, StepDisposition::Applied);
    assert_eq!((e.work_delta, e.helper_work_delta), (1, 1));
    assert_eq!(
        e.alloc_deltas,
        AllocDeltasC {
            data_nodes: dn as i64,
            data_fields: df as i64,
            environment_cells: 0
        }
    );
    assert_eq!(post.remaining_work, pre.remaining_work - 1);
    assert_eq!(post.consumed_work, pre.consumed_work + 1);
    assert_eq!(post.helper_work, pre.helper_work + 1);
    assert!(post.helper.is_none(), "helper cleared on Applied");
    assert_eq!(post.outcome, OutcomeC::Running);
    // Untouched state preserved exactly.
    assert_eq!(post.frames, pre.frames);
    assert_eq!(post.env, pre.env);
    assert_eq!(post.value_slots, pre.value_slots);
    assert_eq!(post.tasks, pre.tasks);
    assert_eq!(post.control, pre.control);
    assert_eq!(post.control_origin, pre.control_origin);
    assert_eq!(post.env_root, pre.env_root);
    assert_eq!(post.stack_limit, pre.stack_limit);
    assert_eq!(post.initial_fuel, pre.initial_fuel);
    assert_eq!(post.run_id, pre.run_id);
    assert_eq!(post.seg_base, pre.seg_base);
    assert_eq!(post.helper_visits, pre.helper_visits);
    assert_eq!(post.reference_cursor, pre.reference_cursor);
    assert_eq!(post.active.env, pre.active.env);
    assert_eq!(post.active.value_slots, pre.active.value_slots);
    assert_eq!(r.after.admission, r.before.admission);
    assert_eq!(post.alloc.data_nodes, pre.alloc.data_nodes + dn);
    assert_eq!(post.alloc.data_fields, pre.alloc.data_fields + df);
    assert_eq!(post.alloc.environment_cells, pre.alloc.environment_cells);
    assert_eq!(post.active.data, pre.active.data + dn);
    assert_eq!(post.active.data_fields, pre.active.data_fields + df);
    assert_eq!(post.data.len(), pre.data.len() + dn);
    assert_eq!(post.data_fields.len(), pre.data_fields.len() + df);
    assert_eq!(
        post.data[..pre.data.len()],
        pre.data[..],
        "old DATA rows preserved"
    );
    assert_eq!(
        post.data_fields[..pre.data_fields.len()],
        pre.data_fields[..],
        "old DATA fields preserved"
    );
    let (oa, ob) = match pre.helper {
        Some(HelperControlC::Scalar {
            operand_a,
            operand_b,
            ..
        }) => (operand_a, operand_b),
        _ => panic!("selected pre must carry Scalar helper"),
    };
    if dn == 1 {
        let want_row = match w {
            Want::U128(n) => DataSnapshot::U128 { n: *n },
            Want::Bool(b) => DataSnapshot::Bool { b: *b },
            Want::Leaf => DataSnapshot::Leaf {
                fields: DataFieldsSnapshot {
                    start: pre.active.data_fields,
                    len: 1,
                },
            },
            Want::Cons => DataSnapshot::Cons {
                fields: DataFieldsSnapshot {
                    start: pre.active.data_fields,
                    len: 2,
                },
            },
            Want::Branch => DataSnapshot::Branch {
                fields: DataFieldsSnapshot {
                    start: pre.active.data_fields,
                    len: 2,
                },
            },
            _ => unreachable!(),
        };
        assert_eq!(
            post.data[pre.active.data], want_row,
            "new row at pre.active.data (physical index, not alloc counter)"
        );
        assert_eq!(
            post.result,
            Some(HelperResultC::ScalarValue {
                value: RefC::Data(pre.active.data)
            })
        );
        if df >= 1 {
            assert_eq!(
                post.data_fields[pre.active.data_fields], oa,
                "operand_a first"
            );
        }
        if df == 2 {
            assert_eq!(
                post.data_fields[pre.active.data_fields + 1],
                ob.expect("binary operand_b"),
                "operand_b second"
            );
        }
    } else {
        assert_eq!(post.data, pre.data);
        assert_eq!(post.data_fields, pre.data_fields);
        match w {
            Want::Selector(sel) => assert_eq!(
                post.result,
                Some(HelperResultC::ScalarSelector { sel: *sel })
            ),
            Want::TypeTest(ok) => {
                assert_eq!(post.result, Some(HelperResultC::ScalarTypeTest { ok: *ok }))
            }
            _ => unreachable!(),
        }
    }
}

fn assert_checks(w: &C6ScalarExitStepWitness) {
    for mode in [
        C6PushStatementBindingMode::LocalPublic,
        C6PushStatementBindingMode::LedgerCall,
    ] {
        let circuit = build_c6_scalar_exit_step_circuit(
            w.source_params().clone(),
            w.host_prefix().meta.len(),
            w.host_prefix().prog.len(),
            w.host_prefix().meta_bounds_frames_offset,
            mode,
            "scalar_exit_owner_test",
        )
        .expect("circuit builds");
        assert!(
            check_c6_scalar_exit_step_circuit(&circuit, w).is_ok(),
            "owner-produced witness must pass SDK preprocessing in {mode:?}"
        );
    }
}

struct Applied {
    record: CompleteStepRecord,
    witness: C6ScalarExitStepWitness,
}
fn run_applied(w: &Want, b: BoundsC) -> Applied {
    let p = branch_program(w);
    let (admitted, mut image, pre) = reach(&p, b, w);
    let record = replay_record(&p, b, w);
    assert_eq!(
        record.before, pre,
        "deterministic replay reaches the same pre"
    );
    let witness = record_scalar_exit_step(&admitted, &mut image, &cfg(b), &limits())
        .expect("selected wrapper step");
    // Observation only, not a second owner step: the wrapper has taken
    // its one step on the original selected image.
    let original_after = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("capture original wrapper post: {}", violation_label(&e)));
    assert_eq!(
        original_after, record.after,
        "original wrapper-step post must equal the replayed raw-step post"
    );
    assert_applied(&record, w);
    assert!(!witness.disposition_is_refused_helper_work);
    Applied { record, witness }
}
fn exact_bounds(w: &Want) -> BoundsC {
    let (dn, df) = w.delta();
    bounds(100, dn.max(1), df)
}
fn generous() -> BoundsC {
    bounds(100, 16, 16)
}

#[test]
fn u128_add_applied_full_effects_and_both_mode_checks() {
    // alloc+delta == admission exactly (0+1 == 1): Applied boundary.
    let a = run_applied(&Want::U128(42), bounds(100, 1, 0));
    assert_checks(&a.witness);
}
#[test]
fn bool_lt_applied_full_effects_and_both_mode_checks() {
    let a = run_applied(&Want::Bool(true), bounds(100, 1, 0));
    assert_checks(&a.witness);
}
#[test]
fn pending_leaf_applied_full_effects_and_both_mode_checks() {
    let a = run_applied(&Want::Leaf, bounds(100, 1, 1));
    assert_checks(&a.witness);
}
#[test]
fn pending_cons_applied_full_effects_and_both_mode_checks() {
    let a = run_applied(&Want::Cons, bounds(100, 1, 2));
    assert_checks(&a.witness);
}
#[test]
fn pending_branch_applied_full_effects_and_both_mode_checks() {
    let a = run_applied(&Want::Branch, bounds(100, 1, 2));
    assert_checks(&a.witness);
}
#[test]
fn selector_true_applied_no_alloc_and_both_mode_checks() {
    let a = run_applied(&Want::Selector(BoolSelector::True), bounds(100, 0, 0));
    assert_checks(&a.witness);
}
#[test]
fn selector_false_applied_no_alloc_and_both_mode_checks() {
    let a = run_applied(&Want::Selector(BoolSelector::False), bounds(100, 0, 0));
    assert_checks(&a.witness);
}
#[test]
fn selector_not_bool_applied_no_alloc_and_both_mode_checks() {
    let a = run_applied(&Want::Selector(BoolSelector::NotBool), bounds(100, 0, 0));
    assert_checks(&a.witness);
}
#[test]
fn type_test_true_applied_no_alloc_and_both_mode_checks() {
    let a = run_applied(&Want::TypeTest(true), bounds(100, 0, 0));
    assert_checks(&a.witness);
}
#[test]
fn type_test_false_applied_no_alloc_and_both_mode_checks() {
    let a = run_applied(&Want::TypeTest(false), bounds(100, 0, 0));
    assert_checks(&a.witness);
}

#[derive(Clone, Copy)]
enum Neg {
    U128Payload,
    BoolPayload,
    LeafTag,
    ConsOrder,
    BranchLen,
    SelectorWrong,
    TypeTestWrong,
    ResultRef,
    ControlOrigin,
}
fn assert_sdk_rejects(w: &Want, neg: Neg) {
    let p = branch_program(w);
    let record = replay_record(&p, generous(), w);
    let (pre, mut post) = (record.before.clone(), record.after.clone());
    let at = pre.image.active.data;
    match neg {
        Neg::U128Payload => post.image.data[at] = DataSnapshot::U128 { n: 43 },
        Neg::BoolPayload => post.image.data[at] = DataSnapshot::Bool { b: false },
        Neg::LeafTag => {
            post.image.data[at] = DataSnapshot::Cons {
                fields: DataFieldsSnapshot {
                    start: pre.image.active.data_fields,
                    len: 1,
                },
            }
        }
        Neg::ConsOrder => post.image.data_fields.swap(
            pre.image.active.data_fields,
            pre.image.active.data_fields + 1,
        ),
        Neg::BranchLen => {
            post.image.data[at] = DataSnapshot::Branch {
                fields: DataFieldsSnapshot {
                    start: pre.image.active.data_fields,
                    len: 1,
                },
            }
        }
        Neg::SelectorWrong => {
            post.image.result = Some(HelperResultC::ScalarSelector {
                sel: BoolSelector::False,
            })
        }
        Neg::TypeTestWrong => post.image.result = Some(HelperResultC::ScalarTypeTest { ok: false }),
        Neg::ResultRef => {
            post.image.result = Some(HelperResultC::ScalarValue {
                value: RefC::Data(at + 1),
            })
        }
        Neg::ControlOrigin => post.image.control_origin = pre.image.control_origin + 1,
    }
    // Paired positive control first.
    let good = witness_from_host_pair(true, &pre, &record.after, &cfg(generous()), &limits())
        .expect("positive control witness");
    // The semantic wrong post must encode (producer/host-stage success);
    // rejection must then come from the SDK checker itself.
    let bad = witness_from_host_pair(true, &pre, &post, &cfg(generous()), &limits())
        .expect("wrong post must witness (HOST_STAGE ok, not circuit rejection)");
    let circuit = build_c6_scalar_exit_step_circuit(
        good.source_params().clone(),
        good.host_prefix().meta.len(),
        good.host_prefix().prog.len(),
        good.host_prefix().meta_bounds_frames_offset,
        C6PushStatementBindingMode::LocalPublic,
        "scalar_exit_owner_neg",
    )
    .expect("circuit builds");
    assert!(
        check_c6_scalar_exit_step_circuit(&circuit, &good).is_ok(),
        "paired positive control must pass SDK preprocessing"
    );
    let err = check_c6_scalar_exit_step_circuit(&circuit, &bad)
        .expect_err("wrong post must be rejected by SDK preprocessing");
    assert!(
        format!("{err}").contains("SDK ProofPreimage::check failed"),
        "rejection must be the SDK preprocessing category, got: {err}"
    );
}
#[test]
fn sdk_negative_u128_payload() {
    assert_sdk_rejects(&Want::U128(42), Neg::U128Payload);
    assert_sdk_rejects(&Want::U128(42), Neg::ResultRef);
}
#[test]
fn sdk_negative_bool_payload_and_leaf_tag() {
    assert_sdk_rejects(&Want::Bool(true), Neg::BoolPayload);
    assert_sdk_rejects(&Want::Leaf, Neg::LeafTag);
}
#[test]
fn sdk_negative_field_order_len_selector_typetest_and_preservation() {
    assert_sdk_rejects(&Want::Cons, Neg::ConsOrder);
    assert_sdk_rejects(&Want::Branch, Neg::BranchLen);
    assert_sdk_rejects(&Want::Selector(BoolSelector::True), Neg::SelectorWrong);
    assert_sdk_rejects(&Want::Selector(BoolSelector::True), Neg::ControlOrigin);
    assert_sdk_rejects(&Want::TypeTest(true), Neg::TypeTestWrong);
}

fn assert_refusal(
    r: &CompleteStepRecord,
    disp: StepDisposition,
    reason: ExhaustReason,
    charged: bool,
) {
    let (pre, post, e) = (&r.before.image, &r.after.image, &r.event);
    assert_eq!(e.rule_id, RuleIdC::BHelperScalarExit);
    assert_eq!(e.disposition, disp);
    assert_eq!(e.exhausted_kind, Some(reason));
    assert_eq!(post.outcome, OutcomeC::Exhausted { reason });
    assert_eq!(post.helper, pre.helper, "helper preserved on refusal");
    assert_eq!(post.result, pre.result);
    assert_eq!(post.data, pre.data);
    assert_eq!(post.data_fields, pre.data_fields);
    assert_eq!(post.alloc, pre.alloc);
    assert_eq!(post.active, pre.active);
    if charged {
        assert_eq!((e.work_delta, e.helper_work_delta), (1, 1), "P6 charged");
        assert_eq!(post.remaining_work, pre.remaining_work - 1);
        assert_eq!(post.consumed_work, pre.consumed_work + 1);
        assert_eq!(post.helper_work, pre.helper_work + 1);
    } else {
        assert_eq!((e.work_delta, e.helper_work_delta), (0, 0), "P3 uncharged");
        assert_eq!(post.remaining_work, pre.remaining_work);
        assert_eq!(post.consumed_work, pre.consumed_work);
        assert_eq!(post.helper_work, pre.helper_work);
    }
}
fn run_refusal(w: &Want, b: BoundsC, disp: StepDisposition, reason: ExhaustReason) {
    let p = branch_program(w);
    let (admitted, mut image, pre) = reach(&p, b, w);
    let record = replay_record(&p, b, w);
    assert_eq!(record.before, pre);
    assert_refusal(
        &record,
        disp,
        reason,
        disp == StepDisposition::RefusedCapCharged,
    );
    let witness = record_scalar_exit_step(&admitted, &mut image, &cfg(b), &limits())
        .expect("selected refusal wrapper step");
    // Observation only, not a second owner step: the wrapper has taken
    // its one step on the original selected image.
    let original_after = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("capture original wrapper post: {}", violation_label(&e)));
    assert_eq!(
        original_after, record.after,
        "original refusal wrapper-step post must equal the replayed raw-step post"
    );
    assert_eq!(
        witness.disposition_is_refused_helper_work,
        disp == StepDisposition::RefusedHelperWorkUncharged
    );
    assert_checks(&witness);
}
#[test]
fn p3_helper_work_uncharged_wins_over_simultaneous_p6() {
    let w = Want::U128(42);
    let p = branch_program(&w);
    let (_, _, pre) = reach(&p, generous(), &w);
    // Helper-work count at the real Done state, learned from a prior
    // real run; the refusing run is then admitted with exactly that
    // initial bound (no post-admission mutation).
    let hw = pre.image.helper_work;
    assert!(hw > 0);
    run_refusal(
        &w,
        bounds(hw, 0, 0),
        StepDisposition::RefusedHelperWorkUncharged,
        ExhaustReason::HelperWork,
    );
}
#[test]
fn p6_data_nodes_charged_refusal_and_equality_boundary() {
    // 0+1 > 0 refuses DataNodes (charged); 0+1 == 1 applied (see U128 test).
    run_refusal(
        &Want::U128(42),
        bounds(100, 0, 0),
        StepDisposition::RefusedCapCharged,
        ExhaustReason::DataNodes,
    );
}
#[test]
fn p6_data_fields_refusal_and_node_before_fields_priority() {
    run_refusal(
        &Want::Leaf,
        bounds(100, 1, 0),
        StepDisposition::RefusedCapCharged,
        ExhaustReason::DataFields,
    );
    run_refusal(
        &Want::Branch,
        bounds(100, 0, 0),
        StepDisposition::RefusedCapCharged,
        ExhaustReason::DataNodes,
    );
}
#[test]
fn selector_nonallocating_at_full_allocation_caps_applies() {
    let a = run_applied(&Want::Selector(BoolSelector::True), bounds(100, 0, 0));
    assert_eq!(a.record.event.disposition, StepDisposition::Applied);
    assert_checks(&a.witness);
}

#[test]
fn wrong_record_event_snapshot_cross_pairs_rejected() {
    // `CompleteStepRecord` is publicly constructible: these mutated
    // records carry no provenance authentication and must fail only on
    // internal event/snapshot consistency (`EventMismatch`).
    let a = run_applied(&Want::U128(42), exact_bounds(&Want::U128(42)));
    let mut r1 = a.record.clone();
    r1.event.disposition = StepDisposition::RefusedCapCharged;
    let mut r2 = a.record.clone();
    r2.event.result_tag = Some(HelperResultTagC::ScalarSelector);
    let mut r3 = a.record.clone();
    let at = r3.before.image.active.data;
    r3.after.image.data[at] = DataSnapshot::U128 { n: 999 };
    let mut r4 = a.record.clone();
    r4.after = r4.before.clone();
    for r in [r1, r2, r3, r4] {
        assert!(
            matches!(
                witness_from_owner_record(&r, &cfg(generous()), &limits()),
                Err(C6ScalarExitConsumerError::EventMismatch { .. })
            ),
            "wrong record must fail event/snapshot consistency"
        );
    }
}
#[test]
fn wrapper_err_after_one_step_does_not_roll_back() {
    let p = branch_program(&Want::U128(42));
    let engine = host_owner_engine();
    let b = generous();
    let admitted = engine
        .admit(&p, &b, &layout(b))
        .unwrap_or_else(|e| panic!("admit: {}", tooling_label(&e)));
    let mut image = engine
        .admit_run(
            &admitted,
            &RunIntC::Nat(64),
            &RunIntC::Nat(8),
            &FinanceInput::Absent,
        )
        .unwrap_or_else(|e| panic!("genesis: {}", tooling_label(&e)));
    let before = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("capture before: {}", violation_label(&e)));
    // Genesis is not the selected Scalar exit: the wrapper takes its
    // one owner step and only then refuses; the image stays advanced.
    assert!(matches!(
        record_scalar_exit_step(&admitted, &mut image, &cfg(b), &limits()),
        Err(C6ScalarExitConsumerError::UnexpectedOwnerStep)
    ));
    let after = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("capture after: {}", violation_label(&e)));
    assert_ne!(before, after, "wrapper Err must not imply no step happened");
}
