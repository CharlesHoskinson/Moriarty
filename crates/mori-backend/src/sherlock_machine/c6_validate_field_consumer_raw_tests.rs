//! Independent raw statement, instruction, namespace, receipt-gating,
//! and SDK preprocessing tests for `c6_validate_field_consumer` and
//! `c6_validate_field_step`.
//!
//! UNCOMPILED/UNRUN. Every baseline is admitted through the real public
//! owner (`host_owner_engine().admit` / `admit_run`, or
//! `admit_ext_program` for nominal `Constructor` fixtures) and stepped
//! with real `complete_observation::record_step` through
//! `BHelperValidateEntry` and `BHelperValidateOccurrence` (plus earlier
//! `BHelperValidateField` steps for `cursor > 0` and multi-task stack
//! cases) to the selected `BHelperValidateField` pre-state.
//!
//! Raw mutations are `Vec<Fr>` statements checked by real SDK
//! `ProofPreimage::check` preprocessing (not synthesis, proving,
//! MockProver, or native execution) in BOTH `LocalPublic` and
//! `LedgerCall` modes. `LedgerCall` commitments are recomputed from the
//! CURRENT statement plus CURRENT post on every check; the one
//! stale-commitment case is isolated and labelled.

use std::collections::BTreeSet;
use std::sync::Arc;

use midnight_transient_crypto::curve::Fr;
use midnight_transient_crypto::hash::transient_commit;
use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage};
use midnight_zkir_v3::ir::{IrMinorVersion, Operand};
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use serde_json::json;

use mori_core::sherlock::helper_machine::c6_phys::{
    C6CodecBudgets, L_CONSUMED_WORK, L_CONTROL, L_CONTROL_ORIGIN, L_FRAME_COUNT, L_HELPER_VISITS,
    L_HELPER_WORK, L_OUTCOME, L_REFERENCE_CURSOR, L_REMAINING_WORK,
};
use mori_core::sherlock::helper_machine::c6_phys_data::REF_LANES;
use mori_core::sherlock::helper_machine::c6_phys_helper::{
    L_HELPER_RECORD, L_RESULT_RECORD, L_TASKS, TASK_LANES,
};
use mori_core::sherlock::helper_machine::c6_phys_host::{
    C6_HOST_SCHEMA, C6HostConfig, C6HostLimits, CTX_LANES,
};
use mori_core::sherlock::helper_machine::c6_phys_program::encode_program;
use mori_core::sherlock::helper_machine::complete_observation::{
    CompleteStepRecord, ConstFieldsSnapshot, ConstantSnapshot, HostSnapshot, ObservedStep,
    ProgramSnapshot, capture, record_step as owner_record,
};
use mori_core::sherlock::helper_machine::shared::adt::{CtorIdC, TypeIdC};
use mori_core::sherlock::helper_machine::shared::owner_ext_admit::ExtCapacitiesC;
use mori_core::sherlock::helper_machine::shared::owner_ext_source_program::{
    ExtDomain, ExtProgram,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_t3_uses::{ExtExpr, ExtValue};
use mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::{
    CtorDeclSrc, CtorFieldSrc, ExtTypeBuildError, TypeDeclSrc, TypeExprSrc,
};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::*;

use super::c6_data_canonical::c6_data_geometry;
use super::c6_push_step::{C6_PADDED_CARRIER_VERSION, C6PushStatementBindingMode};
use super::c6_validate_field_consumer::{
    C6ValidateFieldConsumerError, C6ValidateFieldStepCircuit, C6ValidateFieldStepWitness,
    CHECK_KEY_LOCATION, build_c6_validate_field_step_circuit, capture_step_error_label,
    check_c6_validate_field_step_circuit, witness_from_host_pair, witness_from_owner_record,
};
use super::c6_validate_field_step::{
    C6ValidateFieldStepBlock, C6ValidateFieldStepEmitter, C6ValidateFieldStepError,
    C6ValidateFieldStepInputs, emit_c6_validate_field_step_block,
};

const MAX: u128 = u128::MAX;
const SCALAR_PREFIX_LANES: usize = 5;

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
            "capacity: kind={kind:?}, needed={needed:?}, limit={limit}, locator={locator:?}"
        ),
        Tooling::UnsupportedFeature { feature, locator } => {
            format!("unsupported: feature={feature:?}, locator={locator:?}")
        }
    }
}

fn ext_label(e: &ExtTypeBuildError) -> String {
    match e {
        ExtTypeBuildError::CoreTooling(t) => tooling_label(t),
        ExtTypeBuildError::ExtCapacityNotAdmitted {
            kind,
            needed,
            limit,
            locator,
        } => format!(
            "ext capacity: kind={kind:?}, needed={needed:?}, limit={limit}, locator={locator:?}"
        ),
        ExtTypeBuildError::ExtAdmissionRejected {
            reason,
            detail,
            locator,
        } => format!("ext rejected: reason={reason:?}, detail={detail}, locator={locator:?}"),
        ExtTypeBuildError::ExtUnsupportedFeature { feature, locator } => {
            format!("ext unsupported: feature={feature:?}, locator={locator:?}")
        }
        ExtTypeBuildError::Host(v) => format!(
            "ext host: kind={:?}, clause={:?}, rule={:?}, locator={}",
            v.kind,
            v.clause,
            v.rule,
            v.locator.as_deref().unwrap_or("<none>")
        ),
    }
}

fn bounds_with(max_helper_work: u128, helper_tasks: usize) -> BoundsC {
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
        data_nodes: 16,
        data_field_slots: 16,
        environment_cells: 16,
        frames: 16,
        helper_tasks,
        serialization_bytes: 1_000_000,
        max_reference_work: 1_000_000,
        max_helper_work,
        step_cap: 1_000_000,
    }
}

fn bounds(max_helper_work: u128) -> BoundsC {
    bounds_with(max_helper_work, 4)
}

fn layout(b: BoundsC, ext: Option<ExtCapacitiesC>) -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("lv".into(), "1".into()),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds: b,
        capacities: LayoutCapacitiesC {
            name_rows: 32,
            name_bytes: 512,
            occurrence_path_slots: 64,
            value_slots: 512,
        },
        ext_capacities: ext,
        boundary_capacities_present: false,
    }
}

fn cfg(b: BoundsC, ext: Option<ExtCapacitiesC>) -> C6HostConfig {
    C6HostConfig::from_retained_layout(
        &layout(b, ext),
        C6CodecBudgets {
            type_expr_nodes: 256,
            template_bytes: 4096,
        },
    )
}

fn core_cfg(b: BoundsC) -> C6HostConfig {
    cfg(b, None)
}

fn ext_cfg(b: BoundsC) -> C6HostConfig {
    cfg(b, Some(ExtCapacitiesC::all_zero()))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ConstKindWant {
    UInt(u128),
    Bool(bool),
    Nil,
    Cons,
    Leaf,
    Branch,
    Constructor { arity: usize },
    Marker,
}

#[derive(Clone, Copy, Debug)]
struct FieldSpec {
    parent_kind: ConstKindWant,
    parent_marker_free: bool,
    cursor: usize,
    min_expr_id: usize,
    min_tasks_len: usize,
}

fn field_spec(parent_kind: ConstKindWant, parent_marker_free: bool, cursor: usize) -> FieldSpec {
    FieldSpec {
        parent_kind,
        parent_marker_free,
        cursor,
        min_expr_id: 0,
        min_tasks_len: 1,
    }
}

fn const_info(s: &HostSnapshot, id: usize) -> (ConstKindWant, bool, usize, usize, usize) {
    let view = &s.admission.view;
    match view.constants.get(id) {
        Some(ConstantSnapshot::U128 { n, marker_free }) => {
            (ConstKindWant::UInt(*n), *marker_free, 0, 0, 0)
        }
        Some(ConstantSnapshot::Bool { b, marker_free }) => {
            (ConstKindWant::Bool(*b), *marker_free, 0, 0, 0)
        }
        Some(ConstantSnapshot::Nil { marker_free }) => (ConstKindWant::Nil, *marker_free, 0, 0, 0),
        Some(ConstantSnapshot::Cons {
            fields,
            marker_free,
        }) => (
            ConstKindWant::Cons,
            *marker_free,
            2,
            fields.start,
            fields.len,
        ),
        Some(ConstantSnapshot::Leaf {
            fields,
            marker_free,
        }) => (
            ConstKindWant::Leaf,
            *marker_free,
            1,
            fields.start,
            fields.len,
        ),
        Some(ConstantSnapshot::Branch {
            fields,
            marker_free,
        }) => (
            ConstKindWant::Branch,
            *marker_free,
            2,
            fields.start,
            fields.len,
        ),
        Some(ConstantSnapshot::Constructor {
            ctor_id,
            fields,
            marker_free,
            ..
        }) => (
            ConstKindWant::Constructor {
                arity: view.constructors[ctor_id.0].field_len,
            },
            *marker_free,
            view.constructors[ctor_id.0].field_len,
            fields.start,
            fields.len,
        ),
        Some(ConstantSnapshot::U128OutOfRange { marker_free }) => {
            (ConstKindWant::Marker, *marker_free, 0, 0, 0)
        }
        None => panic!("constant id {id} must exist"),
    }
}

fn selected_field_constant(s: &HostSnapshot, w: &FieldSpec) -> Option<usize> {
    if s.image.outcome != OutcomeC::Running
        || s.image.remaining_work == 0
        || s.image.result.is_some()
        || s.image.tasks.len() < w.min_tasks_len
        || s.image.control_origin < w.min_expr_id
    {
        return None;
    }
    match &s.image.helper {
        Some(HelperControlC::Validate {
            common,
            status: TriStatus::Pending,
            ..
        }) if common.parent_resume.is_none() => {}
        _ => return None,
    }
    let Some(HelperTaskC::ValidateTask {
        r: RefC::Const(const_id),
        visit: TaskVisitC::Fields,
        cursor,
    }) = s.image.tasks.last()
    else {
        return None;
    };
    if *cursor != w.cursor {
        return None;
    }
    let (actual_kind, actual_mf, _, _, slice_len) = const_info(s, *const_id);
    if actual_kind == w.parent_kind
        && actual_mf == w.parent_marker_free
        && actual_kind != ConstKindWant::Marker
        && *cursor < slice_len
    {
        Some(*const_id)
    } else {
        None
    }
}

fn search(
    admitted: Admitted,
    mut image: MachineImage,
    w: &FieldSpec,
    label: &str,
) -> (Admitted, MachineImage, HostSnapshot) {
    let initial = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("{label}: capture initial: kind={:?}", e.kind));
    if selected_field_constant(&initial, w).is_some() {
        return (admitted, image, initial);
    }
    for _ in 0..512 {
        let step = owner_record(&admitted, &mut image)
            .unwrap_or_else(|e| panic!("{label}: search: {}", capture_step_error_label(&e)));
        let after = match &step {
            ObservedStep::Event(r) => &r.after,
            ObservedStep::Terminal { after, .. } => {
                panic!(
                    "{label}: terminal before selected PRE: {:?}",
                    after.image.outcome
                )
            }
        };
        if selected_field_constant(after, w).is_some() {
            let sel = capture(&admitted, &image)
                .unwrap_or_else(|e| panic!("{label}: capture: kind={:?}", e.kind));
            assert_eq!(&sel, after, "{label}: capture equals searched PRE");
            return (admitted, image, sel);
        }
    }
    panic!("{label}: genuine owner never reached selected ValidateField PRE")
}

fn reach_core(
    b: BoundsC,
    w: &FieldSpec,
    label: &str,
    make: &dyn Fn() -> CoreProgram,
) -> (Admitted, MachineImage, HostSnapshot) {
    let engine = host_owner_engine();
    let admitted = engine
        .admit(&make(), &b, &layout(b, None))
        .unwrap_or_else(|e| panic!("{label}: admit: {}", tooling_label(&e)));
    let image = engine
        .admit_run(
            &admitted,
            &RunIntC::Nat(64),
            &RunIntC::Nat(8),
            &FinanceInput::Absent,
        )
        .unwrap_or_else(|e| panic!("{label}: genesis: {}", tooling_label(&e)));
    search(admitted, image, w, label)
}

fn reach_ext(
    b: BoundsC,
    w: &FieldSpec,
    label: &str,
    make: &dyn Fn() -> ExtProgram,
) -> (Admitted, MachineImage, HostSnapshot) {
    let (admitted, _) = admit_ext_program(
        make(),
        ExtDomain::GENERIC_ABSENT,
        &b,
        &layout(b, Some(ExtCapacitiesC::all_zero())),
        None,
    )
    .unwrap_or_else(|e| panic!("{label}: admit_ext: {}", ext_label(&e)));
    let engine = host_owner_engine();
    let image = engine
        .admit_run(
            &admitted,
            &RunIntC::Nat(64),
            &RunIntC::Nat(8),
            &FinanceInput::Absent,
        )
        .unwrap_or_else(|e| panic!("{label}: ext genesis: {}", tooling_label(&e)));
    search(admitted, image, w, label)
}

struct Applied {
    record: CompleteStepRecord,
    witness: C6ValidateFieldStepWitness,
}

fn finish_applied(
    admitted: Admitted,
    mut image: MachineImage,
    pre: HostSnapshot,
    config: &C6HostConfig,
    w: &FieldSpec,
    label: &str,
) -> Applied {
    let record = match owner_record(&admitted, &mut image) {
        Ok(ObservedStep::Event(r)) => r,
        Ok(ObservedStep::Terminal { .. }) => panic!("{label}: selected step must be event"),
        Err(e) => panic!("{label}: selected step: {}", capture_step_error_label(&e)),
    };
    assert_eq!(record.before, pre, "{label}: record pre");
    assert!(
        selected_field_constant(&record.before, w).is_some(),
        "{label}: selected PRE"
    );
    assert_eq!(record.event.rule_id, RuleIdC::BHelperValidateField);
    assert_eq!(record.event.disposition, StepDisposition::Applied);
    let witness = witness_from_owner_record(&record, config, &limits())
        .unwrap_or_else(|e| panic!("{label}: owner witness: {e}"));
    let hp = witness_from_host_pair(true, &record.before, &record.after, config, &limits())
        .unwrap_or_else(|e| panic!("{label}: host-pair witness: {e}"));
    assert_eq!(
        hp.to_statement_frs(),
        witness.to_statement_frs(),
        "{label}: routes agree"
    );
    assert!(
        witness.enable
            && !witness.disposition_is_refused_helper_work
            && !witness.disposition_is_refused_helper_tasks,
        "{label}: flags"
    );
    Applied { record, witness }
}

fn run_core(w: &FieldSpec, label: &str, make: &dyn Fn() -> CoreProgram) -> Applied {
    let b = bounds(100);
    let (a, i, pre) = reach_core(b, w, label, make);
    finish_applied(a, i, pre, &core_cfg(b), w, label)
}

fn run_ext(w: &FieldSpec, label: &str, make: &dyn Fn() -> ExtProgram) -> Applied {
    let b = bounds(100);
    let (a, i, pre) = reach_ext(b, w, label, make);
    finish_applied(a, i, pre, &ext_cfg(b), w, label)
}

fn lit_u(n: u128) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::UInt {
            n: i128_class::KIntC::InRange(n),
        },
    }
}
fn lit_list(vs: &[u128]) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::List {
            elements: vs.iter().map(|n| i128_class::KIntC::InRange(*n)).collect(),
        },
    }
}
fn tree_leaf(n: u128) -> CoreTree {
    CoreTree::Leaf {
        n: i128_class::KIntC::InRange(n),
    }
}
fn lit_tree(t: CoreTree) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::Tree { tree: t },
    }
}
fn lit_branch() -> CoreExpr {
    lit_tree(CoreTree::Branch {
        left: Box::new(tree_leaf(1)),
        right: Box::new(tree_leaf(2)),
    })
}
fn lit_list_marker_head() -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::List {
            elements: vec![i128_class::KIntC::Negative, i128_class::KIntC::InRange(2)],
        },
    }
}
fn lit_tree_marker_leaf() -> CoreExpr {
    lit_tree(CoreTree::Leaf {
        n: i128_class::KIntC::AboveU128,
    })
}
fn lit_tree_marker_branch() -> CoreExpr {
    lit_tree(CoreTree::Branch {
        left: Box::new(CoreTree::Leaf {
            n: i128_class::KIntC::AboveU128,
        }),
        right: Box::new(tree_leaf(2)),
    })
}
fn bin(op: BinaryOp, l: CoreExpr, r: CoreExpr) -> CoreExpr {
    CoreExpr::Binary {
        op,
        left: Box::new(l),
        right: Box::new(r),
    }
}
fn prog(entry: CoreExpr) -> CoreProgram {
    CoreProgram {
        functions: Vec::new(),
        entry,
    }
}

fn leaf_tail_witness() -> Applied {
    run_core(
        &field_spec(ConstKindWant::Leaf, true, 0),
        "ValidateField Leaf cursor=0 (tail)",
        &|| prog(lit_tree(tree_leaf(5))),
    )
}

fn cons_nontail_witness() -> Applied {
    run_core(
        &field_spec(ConstKindWant::Cons, true, 0),
        "ValidateField Cons cursor=0 (non-tail)",
        &|| prog(lit_list(&[7])),
    )
}

fn cons_tail_witness() -> Applied {
    run_core(
        &field_spec(ConstKindWant::Cons, true, 1),
        "ValidateField Cons cursor=1 (tail)",
        &|| prog(lit_list(&[7])),
    )
}

fn depth2_tail_witness() -> Applied {
    let w = FieldSpec {
        parent_kind: ConstKindWant::Leaf,
        parent_marker_free: true,
        cursor: 0,
        min_expr_id: 0,
        min_tasks_len: 2,
    };
    run_core(&w, "ValidateField Leaf(1) at depth 2 (tail 2->2)", &|| {
        prog(lit_branch())
    })
}

fn depth2_nontail_witness() -> Applied {
    let w = FieldSpec {
        parent_kind: ConstKindWant::Branch,
        parent_marker_free: true,
        cursor: 0,
        min_expr_id: 0,
        min_tasks_len: 2,
    };
    run_core(
        &w,
        "ValidateField inner Branch at depth 2 (non-tail 2->3)",
        &|| {
            prog(lit_tree(CoreTree::Branch {
                left: Box::new(CoreTree::Branch {
                    left: Box::new(tree_leaf(10)),
                    right: Box::new(tree_leaf(20)),
                }),
                right: Box::new(tree_leaf(30)),
            }))
        },
    )
}

fn nominal_types() -> Vec<TypeDeclSrc> {
    vec![
        TypeDeclSrc {
            name: "Alpha".into(),
            parameters: Vec::new(),
            ctors: vec![CtorDeclSrc {
                name: "MkAlpha".into(),
                fields: vec![CtorFieldSrc {
                    name: "payload".into(),
                    ty: TypeExprSrc::Named {
                        name: "UInt".into(),
                    },
                }],
            }],
        },
        TypeDeclSrc {
            name: "Beta".into(),
            parameters: Vec::new(),
            ctors: vec![CtorDeclSrc {
                name: "MkBeta".into(),
                fields: Vec::new(),
            }],
        },
        TypeDeclSrc {
            name: "Pair".into(),
            parameters: Vec::new(),
            ctors: vec![CtorDeclSrc {
                name: "MkPair".into(),
                fields: vec![
                    CtorFieldSrc {
                        name: "first".into(),
                        ty: TypeExprSrc::Named {
                            name: "UInt".into(),
                        },
                    },
                    CtorFieldSrc {
                        name: "second".into(),
                        ty: TypeExprSrc::Named {
                            name: "UInt".into(),
                        },
                    },
                ],
            }],
        },
    ]
}

fn nominal_ctor_one_field_program(with_marker: bool) -> ExtProgram {
    let field_val = if with_marker {
        i128_class::KIntC::Negative
    } else {
        i128_class::KIntC::InRange(11)
    };
    ExtProgram {
        types: nominal_types(),
        assets: Vec::new(),
        states: Vec::new(),
        inputs: Vec::new(),
        functions: Vec::new(),
        entry: ExtExpr::Literal {
            value: ExtValue::Constructor {
                ty: TypeExprSrc::Named {
                    name: "Alpha".into(),
                },
                ctor: "MkAlpha".into(),
                fields: vec![ExtValue::Core {
                    value: CoreValue::UInt { n: field_val },
                }],
            },
        },
    }
}

fn nominal_ctor_two_fields_program() -> ExtProgram {
    ExtProgram {
        types: nominal_types(),
        assets: Vec::new(),
        states: Vec::new(),
        inputs: Vec::new(),
        functions: Vec::new(),
        entry: ExtExpr::Literal {
            value: ExtValue::Constructor {
                ty: TypeExprSrc::Named {
                    name: "Pair".into(),
                },
                ctor: "MkPair".into(),
                fields: vec![
                    ExtValue::Core {
                        value: CoreValue::UInt {
                            n: i128_class::KIntC::InRange(11),
                        },
                    },
                    ExtValue::Core {
                        value: CoreValue::UInt {
                            n: i128_class::KIntC::InRange(22),
                        },
                    },
                ],
            },
        },
    }
}

// ---------- raw statement harness (5 scalar prefix lanes) ----------

fn circuit_for(
    w: &C6ValidateFieldStepWitness,
    mode: C6PushStatementBindingMode,
) -> C6ValidateFieldStepCircuit {
    build_c6_validate_field_step_circuit(
        w.source_params.clone(),
        w.host_prefix.meta.len(),
        w.host_prefix.prog.len(),
        w.host_prefix.meta_bounds_frames_offset,
        mode,
        "validate_field_raw_test",
    )
    .expect("circuit builds")
}

fn assert_ok_both(w: &C6ValidateFieldStepWitness, label: &str) {
    for mode in [
        C6PushStatementBindingMode::LocalPublic,
        C6PushStatementBindingMode::LedgerCall,
    ] {
        let c = circuit_for(w, mode);
        assert!(
            check_c6_validate_field_step_circuit(&c, w).is_ok(),
            "{label}: typed positive must pass in {mode:?}"
        );
    }
}

fn prefix_len(w: &C6ValidateFieldStepWitness) -> usize {
    w.host_prefix.meta.len() + w.host_prefix.prog.len() + CTX_LANES
}
fn state_len(w: &C6ValidateFieldStepWitness) -> usize {
    w.pre_image.len() + w.pre_data.len() + w.pre_helper.len()
}
fn pre_state_start(w: &C6ValidateFieldStepWitness) -> usize {
    SCALAR_PREFIX_LANES + prefix_len(w)
}
fn post_prefix_start(w: &C6ValidateFieldStepWitness) -> usize {
    pre_state_start(w) + state_len(w)
}
fn post_state_start(w: &C6ValidateFieldStepWitness) -> usize {
    post_prefix_start(w) + prefix_len(w)
}
fn pre_meta_at(_w: &C6ValidateFieldStepWitness, i: usize) -> usize {
    SCALAR_PREFIX_LANES + i
}
fn post_meta_at(w: &C6ValidateFieldStepWitness, i: usize) -> usize {
    post_prefix_start(w) + i
}
fn pre_prog_at(w: &C6ValidateFieldStepWitness, i: usize) -> usize {
    SCALAR_PREFIX_LANES + w.host_prefix.meta.len() + i
}
fn post_prog_at(w: &C6ValidateFieldStepWitness, i: usize) -> usize {
    post_prefix_start(w) + w.host_prefix.meta.len() + i
}
fn pre_ctx_at(w: &C6ValidateFieldStepWitness, i: usize) -> usize {
    SCALAR_PREFIX_LANES + w.host_prefix.meta.len() + w.host_prefix.prog.len() + i
}
fn post_ctx_at(w: &C6ValidateFieldStepWitness, i: usize) -> usize {
    post_prefix_start(w) + w.host_prefix.meta.len() + w.host_prefix.prog.len() + i
}
fn pre_image_at(w: &C6ValidateFieldStepWitness, l: usize) -> usize {
    pre_state_start(w) + l
}
fn post_image_at(w: &C6ValidateFieldStepWitness, l: usize) -> usize {
    post_state_start(w) + l
}
fn pre_data_at(w: &C6ValidateFieldStepWitness, l: usize) -> usize {
    pre_state_start(w) + w.pre_image.len() + l
}
fn post_data_at(w: &C6ValidateFieldStepWitness, l: usize) -> usize {
    post_state_start(w) + w.pre_image.len() + l
}
fn pre_helper_at(w: &C6ValidateFieldStepWitness, abs: usize) -> usize {
    pre_state_start(w) + w.pre_image.len() + w.pre_data.len() + abs
}
fn post_helper_at(w: &C6ValidateFieldStepWitness, abs: usize) -> usize {
    post_state_start(w) + w.pre_image.len() + w.pre_data.len() + abs
}
fn helper_rel(w: &C6ValidateFieldStepWitness, post: bool, rel: usize) -> usize {
    if post {
        post_helper_at(w, L_HELPER_RECORD + rel)
    } else {
        pre_helper_at(w, L_HELPER_RECORD + rel)
    }
}
fn task_rel(w: &C6ValidateFieldStepWitness, post: bool, slot: usize, rel: usize) -> usize {
    let abs = L_TASKS + slot * TASK_LANES + rel;
    if post {
        post_helper_at(w, abs)
    } else {
        pre_helper_at(w, abs)
    }
}

fn replace_prog_in_statement(
    w: &C6ValidateFieldStepWitness,
    s: &mut [Fr],
    prog_view: &ProgramSnapshot,
) {
    let encoded = encode_program(prog_view, &w.source_params.geometry().caps.phys)
        .expect("in-place mutated ProgramSnapshot encodes canonically");
    assert_eq!(
        encoded.len(),
        w.host_prefix.prog.len(),
        "in-place ProgramSnapshot mutation must preserve exact prog lane count"
    );
    for (i, &lane) in encoded.iter().enumerate() {
        let f = Fr::from(lane);
        s[pre_prog_at(w, i)] = f;
        s[post_prog_at(w, i)] = f;
    }
}

fn check_raw(
    c: &C6ValidateFieldStepCircuit,
    w: &C6ValidateFieldStepWitness,
    statement: Vec<Fr>,
    comm_override: Option<Fr>,
) -> Result<Vec<Option<usize>>, C6ValidateFieldConsumerError> {
    let preimage = match c.mode {
        C6PushStatementBindingMode::LocalPublic => ProofPreimage {
            inputs: Vec::new(),
            private_transcript: Vec::new(),
            public_transcript_inputs: statement.clone(),
            public_transcript_outputs: statement,
            binding_input: Fr::from(0u64),
            communications_commitment: None,
            key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
        },
        C6PushStatementBindingMode::LedgerCall => {
            let randomness = Fr::from(0u64);
            let mut comm_inputs = statement.clone();
            comm_inputs.extend(statement[post_state_start(w)..].iter().copied());
            let comm = comm_override.unwrap_or_else(|| transient_commit(&comm_inputs, randomness));
            ProofPreimage {
                inputs: statement.clone(),
                private_transcript: Vec::new(),
                public_transcript_inputs: statement,
                public_transcript_outputs: Vec::new(),
                binding_input: Fr::from(0u64),
                communications_commitment: Some((comm, randomness)),
                key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
            }
        }
    };
    preimage.check(&c.ir).map_err(|e| {
        C6ValidateFieldConsumerError::Backend(format!("SDK ProofPreimage::check failed: {e}"))
    })
}

fn assert_raw_ok(w: &C6ValidateFieldStepWitness, s: Vec<Fr>, label: &str) {
    for mode in [
        C6PushStatementBindingMode::LocalPublic,
        C6PushStatementBindingMode::LedgerCall,
    ] {
        let c = circuit_for(w, mode);
        assert!(
            check_raw(&c, w, s.clone(), None).is_ok(),
            "{label}: raw positive {mode:?}"
        );
    }
}

fn assert_raw_rejects(w: &C6ValidateFieldStepWitness, s: Vec<Fr>, label: &str) {
    for mode in [
        C6PushStatementBindingMode::LocalPublic,
        C6PushStatementBindingMode::LedgerCall,
    ] {
        let c = circuit_for(w, mode);
        match check_raw(&c, w, s.clone(), None) {
            Err(C6ValidateFieldConsumerError::Backend(m)) => assert!(
                m.contains("SDK ProofPreimage::check failed"),
                "{label} {mode:?}: must be SDK preprocessing rejection, got: {m}"
            ),
            other => panic!(
                "{label} {mode:?}: expected SDK rejection, ok={}",
                other.is_ok()
            ),
        }
    }
}

fn disabled_statement(w: &C6ValidateFieldStepWitness) -> Vec<Fr> {
    let mut s = w.to_statement_frs();
    s[0] = Fr::from(0u64);
    s[1] = Fr::from(0u64);
    s[2] = Fr::from(0u64);
    let pre = s[pre_state_start(w)..pre_state_start(w) + state_len(w)].to_vec();
    let at = post_state_start(w);
    s[at..at + state_len(w)].copy_from_slice(&pre);
    s
}

fn two_pow_128() -> Fr {
    Fr::from(MAX) + Fr::from(1u64)
}

#[test]
fn raw_genuine_all_composite_kinds_tail_and_nontail_and_nominal_ctors_both_modes() {
    let cases: Vec<Applied> = vec![
        leaf_tail_witness(),
        run_core(
            &field_spec(ConstKindWant::Leaf, false, 0),
            "Leaf marker_free=false tail",
            &|| prog(lit_tree_marker_leaf()),
        ),
        cons_nontail_witness(),
        cons_tail_witness(),
        run_core(
            &field_spec(ConstKindWant::Cons, false, 0),
            "Cons marker_free=false non-tail",
            &|| prog(lit_list_marker_head()),
        ),
        run_core(
            &field_spec(ConstKindWant::Branch, true, 0),
            "Branch cursor=0 non-tail",
            &|| prog(lit_branch()),
        ),
        run_core(
            &field_spec(ConstKindWant::Branch, true, 1),
            "Branch cursor=1 tail",
            &|| prog(lit_branch()),
        ),
        run_core(
            &field_spec(ConstKindWant::Branch, false, 0),
            "Branch marker_free=false cursor=0 non-tail",
            &|| prog(lit_tree_marker_branch()),
        ),
        run_ext(
            &field_spec(ConstKindWant::Constructor { arity: 1 }, true, 0),
            "1-field Constructor MkAlpha cursor=0 tail",
            &|| nominal_ctor_one_field_program(false),
        ),
        run_ext(
            &field_spec(ConstKindWant::Constructor { arity: 1 }, false, 0),
            "1-field Constructor MkAlpha marker_free=false cursor=0 tail",
            &|| nominal_ctor_one_field_program(true),
        ),
        run_ext(
            &field_spec(ConstKindWant::Constructor { arity: 2 }, true, 0),
            "2-field Constructor MkPair cursor=0 non-tail",
            &nominal_ctor_two_fields_program,
        ),
        run_ext(
            &field_spec(ConstKindWant::Constructor { arity: 2 }, true, 1),
            "2-field Constructor MkPair cursor=1 tail",
            &nominal_ctor_two_fields_program,
        ),
        depth2_tail_witness(),
        depth2_nontail_witness(),
    ];
    for (i, a) in cases.iter().enumerate() {
        assert_ok_both(&a.witness, &format!("genuine case {i}"));
        assert_raw_ok(
            &a.witness,
            a.witness.to_statement_frs(),
            &format!("genuine raw case {i}"),
        );
    }
}

#[test]
fn raw_enabled_selection_outcome_fuel_owner_site_helper_task_visit_and_oob_both_modes() {
    let a = leaf_tail_witness();
    let w = &a.witness;
    let base = w.to_statement_frs();
    assert_raw_ok(w, base.clone(), "Leaf tail base");

    // 1. Pre outcome not Running (`pre_image[L_OUTCOME..+3] != [0, 0, 0]`).
    for (off, val, what) in [
        (0usize, 1u64, "pre outcome tag Halted(1)"),
        (0, 2, "pre outcome tag Error(2)"),
        (0, 3, "pre outcome tag Exhausted(3)"),
        (1, 1, "pre outcome payload 1 non-zero"),
        (2, 1, "pre outcome payload 2 non-zero"),
    ] {
        let mut s = base.clone();
        s[pre_image_at(w, L_OUTCOME + off)] = Fr::from(val);
        assert_raw_rejects(w, s, what);
    }

    // 2. Zero fuel in pre (`pre_image[L_REMAINING_WORK] == 0`).
    let mut s = base.clone();
    s[pre_image_at(w, L_REMAINING_WORK)] = Fr::from(0u64);
    assert_raw_rejects(w, s, "pre remaining_work == 0");

    // 3. `owner_site(pre)` constraints in `C6ValidateFieldStepEmitter`:
    {
        let mut s = base.clone();
        s[pre_image_at(w, L_CONTROL)] = Fr::from(9u64);
        s[post_image_at(w, L_CONTROL)] = Fr::from(9u64);
        s[pre_image_at(w, L_CONTROL + 1)] = Fr::from(2u64);
        s[post_image_at(w, L_CONTROL + 1)] = Fr::from(2u64);
        assert_raw_rejects(w, s, "owner_site: ControlC::Error(9) rejected");
    }
    {
        let mut s = base.clone();
        s[pre_image_at(w, L_CONTROL)] = Fr::from(1u64);
        s[post_image_at(w, L_CONTROL)] = Fr::from(1u64);
        s[pre_image_at(w, L_CONTROL + 1)] = Fr::from(0u64);
        s[post_image_at(w, L_CONTROL + 1)] = Fr::from(0u64);
        s[pre_image_at(w, L_CONTROL + 2)] = Fr::from(0u64);
        s[post_image_at(w, L_CONTROL + 2)] = Fr::from(0u64);
        assert_raw_rejects(
            w,
            s.clone(),
            "owner_site: ControlC::Return(1) at depth==0 rejected",
        );

        let f0 = w.source_params.geometry().frame_slot_offset(0);
        s[pre_image_at(w, L_FRAME_COUNT)] = Fr::from(1u64);
        s[post_image_at(w, L_FRAME_COUNT)] = Fr::from(1u64);
        s[pre_image_at(w, f0)] = Fr::from(3u64);
        s[post_image_at(w, f0)] = Fr::from(3u64);
        assert_raw_ok(
            w,
            s.clone(),
            "owner_site: ControlC::Return(1) at depth==1 with UnaryF frame accepted",
        );

        s[pre_image_at(w, f0)] = Fr::from(10u64);
        s[post_image_at(w, f0)] = Fr::from(10u64);
        assert_raw_rejects(w, s, "active frame tag 10 out of range");
    }
    {
        let mut s = base.clone();
        s[pre_image_at(w, L_CONTROL)] = Fr::from(11u64);
        s[post_image_at(w, L_CONTROL)] = Fr::from(11u64);
        assert_raw_rejects(w, s, "ControlC tag 11 out of range");

        let max_expr = w.source_params.geometry().caps.phys.max_expressions as u128;
        let mut s = base.clone();
        s[pre_image_at(w, L_CONTROL_ORIGIN)] = Fr::from(max_expr);
        s[post_image_at(w, L_CONTROL_ORIGIN)] = Fr::from(max_expr);
        assert_raw_rejects(w, s, "control_origin == max_expressions");
    }

    // 4. Helper control selection (`pre_helper[L_HELPER_RECORD] == 7` and `status == 0`):
    {
        let mut s = base.clone();
        for rel in 0..23 {
            s[helper_rel(w, false, rel)] = Fr::from(0u64);
            s[helper_rel(w, true, rel)] = Fr::from(0u64);
        }
        assert_raw_rejects(w, s, "pre helper None(0) instead of Validate(7)");
    }
    for (bad_status, what) in [
        (
            1u64,
            "pre Validate.status Positive(1) instead of Pending(0)",
        ),
        (2, "pre Validate.status Negative(2) instead of Pending(0)"),
    ] {
        let mut s = base.clone();
        s[helper_rel(w, false, 7)] = Fr::from(bad_status);
        s[helper_rel(w, true, 7)] = Fr::from(bad_status);
        assert_raw_rejects(w, s, what);
    }

    // 5. Task stack and top task selection (`pre_helper[3] > 0`, `top_task[0] == 0`, `top_task[3] == 1`):
    {
        let mut s = base.clone();
        s[pre_helper_at(w, 3)] = Fr::from(0u64);
        for rel in 0..TASK_LANES {
            s[task_rel(w, false, 0, rel)] = Fr::from(0u64);
        }
        assert_raw_rejects(w, s, "pre task stack empty (pre_helper[3] == 0)");
    }
    for (bad_tag, what) in [
        (1u64, "pre top task tag EqualTask(1)"),
        (2, "pre top task tag PairTask(2)"),
    ] {
        let mut s = base.clone();
        s[task_rel(w, false, 0, 0)] = Fr::from(bad_tag);
        assert_raw_rejects(w, s, what);
    }
    // Top task visit `0` (`TaskVisitC::Unvisited`) in pre -> rejected (`BHelperValidateOccurrence` / `Invalid`, not `Field`)!
    {
        let mut s = base.clone();
        s[task_rel(w, false, 0, 3)] = Fr::from(0u64);
        assert_raw_rejects(w, s, "pre top task visit Unvisited(0) instead of Fields(1)");
    }

    // 6. Top task pointing to 0-arity `UInt` (`slice_len == 0`), OOB `RefC::Const(const_count)`, or OOB `RefC::Data(active_data)`:
    {
        // On `leaf_tail_witness`, `const 0` is `U128(5)` (`slice_len == 0`) while `const 1` is `Leaf`.
        // Pointing pre top task to `RefC::Const(0)` (`U128(5)`) with `cursor = 0` violates `cursor < reader.slice_len` (`0 < 0`)!
        let mut s = base.clone();
        s[task_rel(w, false, 0, 2)] = Fr::from(0u64);
        assert_raw_rejects(
            w,
            s,
            "pre top task pointing to 0-arity UInt (slice_len == 0) rejected by cursor < slice_len",
        );
    }
    {
        let const_count = a.record.before.admission.view.constants.len() as u128;
        let mut s = base.clone();
        s[task_rel(w, false, 0, 2)] = Fr::from(const_count);
        assert_raw_rejects(
            w,
            s,
            "top task RefC::Const(const_count) out of active bounds",
        );
    }
    {
        let active_data = w.pre_data[3];
        let mut s = base;
        s[task_rel(w, false, 0, 1)] = Fr::from(1u64);
        s[task_rel(w, false, 0, 2)] = Fr::from(active_data);
        assert_raw_rejects(
            w,
            s,
            "top task RefC::Data(active_data) out of active DATA bounds",
        );
    }
}

#[test]
fn raw_field_at_slice_bounds_constant_fields_and_sc05_child_const_bound_before_p3_and_p6_both_modes()
 {
    let a_cons = cons_nontail_witness();
    let w_cons = &a_cons.witness;
    let base_cons = w_cons.to_statement_frs();
    let prog_cons = &a_cons.record.before.admission.view;
    let parent_cid = w_cons.pre_helper[L_TASKS + 2] as usize;
    let ConstantSnapshot::Cons {
        fields: orig_fields,
        marker_free: orig_mf,
    } = prog_cons.constants[parent_cid]
    else {
        panic!("expected Cons parent");
    };
    assert_eq!(orig_fields, ConstFieldsSnapshot { start: 0, len: 2 });

    // 1. `cursor >= reader.slice_len`: set pre top task `cursor = 2` (`== slice_len`) on `Cons`:
    {
        let mut s = base_cons.clone();
        s[task_rel(w_cons, false, 0, 4)] = Fr::from(2u64);
        assert_raw_rejects(
            w_cons,
            s,
            "pre top task cursor == slice_len (2) rejected by cursor < reader.slice_len",
        );
    }

    // 2. `field_pos >= const_field_count` and 129-bit `slice_start + cursor` overflow:
    {
        let cf_len = prog_cons.constant_fields.len();
        let mut p_oob_start = prog_cons.clone();
        p_oob_start.constants[parent_cid] = ConstantSnapshot::Cons {
            marker_free: orig_mf,
            fields: ConstFieldsSnapshot {
                start: cf_len,
                len: 2,
            },
        };
        let mut s = base_cons.clone();
        replace_prog_in_statement(w_cons, &mut s, &p_oob_start);
        assert_raw_rejects(
            w_cons,
            s,
            "field_pos == const_field_count rejected by field_pos < const_field_count",
        );

        // 129-bit `slice_start + cursor` at `usize::MAX + 1`: set `start = usize::MAX, len = 2` and `cursor = 1`:
        let mut p_max_start = prog_cons.clone();
        p_max_start.constants[parent_cid] = ConstantSnapshot::Cons {
            marker_free: orig_mf,
            fields: ConstFieldsSnapshot {
                start: usize::MAX,
                len: 2,
            },
        };
        let mut s_max = base_cons.clone();
        replace_prog_in_statement(w_cons, &mut s_max, &p_max_start);
        s_max[task_rel(w_cons, false, 0, 4)] = Fr::from(1u64);
        assert_raw_rejects(
            w_cons,
            s_max,
            "slice_start=usize::MAX + cursor=1 rejected by 129-bit field_pos < const_field_count",
        );
    }

    // 3. `SC-05` (`check_written_code_ids`): mutate `constant_fields[0]` to `constants.len()`
    //    (in-place `ProgramSnapshot` mutation, preserving `prog` lane count) and update
    //    `post_helper` pushed child to match `constants.len()` so ONLY `child_ref_index < const_count` fails!
    {
        let oob_cid = prog_cons.constants.len();
        let mut p_sc05 = prog_cons.clone();
        p_sc05.constant_fields[orig_fields.start] = oob_cid;
        let mut s = base_cons.clone();
        replace_prog_in_statement(w_cons, &mut s, &p_sc05);
        s[task_rel(w_cons, true, 1, 2)] = Fr::from(oob_cid as u128);
        assert_raw_rejects(
            w_cons,
            s,
            "SC-05: child RefC::Const(const_count) rejected by child_ref_index < const_count",
        );
    }

    // 4. PS checks precede P3 and P6 in circuit:
    //    Build a genuine P3 witness and a genuine P6 witness on `Cons`, and verify that
    //    corrupting `cursor = 2` (`>= slice_len`) or `constant_fields[0] = const_count` (`SC-05`)
    //    STILL REJECTS in `ProofPreimage::check` even though P3 or P6 preserves `post_helper == pre_helper`!
    {
        let w_spec = field_spec(ConstKindWant::Cons, true, 0);
        let make = &|| prog(lit_list(&[7]));

        let b_p3 = bounds_with(2, 4);
        let (adm_p3, mut img_p3, _) = reach_core(b_p3, &w_spec, "raw P3 Cons", make);
        let ObservedStep::Event(rec_p3) = (match owner_record(&adm_p3, &mut img_p3) {
            Ok(step) => step,
            Err(error) => panic!("P3: {}", capture_step_error_label(&error)),
        }) else {
            panic!("P3 event");
        };
        let wit_p3 =
            witness_from_owner_record(&rec_p3, &core_cfg(b_p3), &limits()).expect("P3 witness");
        let base_p3 = wit_p3.to_statement_frs();
        assert_raw_ok(&wit_p3, base_p3.clone(), "raw P3 Cons positive");

        // On P3, set `cursor = 2` in both `pre_helper` and `post_helper` (so `post_helper == pre_helper` holds):
        let mut s_p3_oob_cursor = base_p3.clone();
        s_p3_oob_cursor[task_rel(&wit_p3, false, 0, 4)] = Fr::from(2u64);
        s_p3_oob_cursor[task_rel(&wit_p3, true, 0, 4)] = Fr::from(2u64);
        assert_raw_rejects(
            &wit_p3,
            s_p3_oob_cursor,
            "PS before P3: cursor == slice_len rejected even on P3 refusal",
        );

        // On P3, set `constant_fields[0] = const_count` (`SC-05`):
        let mut p_sc05 = rec_p3.before.admission.view.clone();
        p_sc05.constant_fields[0] = p_sc05.constants.len();
        let mut s_p3_sc05 = base_p3;
        replace_prog_in_statement(&wit_p3, &mut s_p3_sc05, &p_sc05);
        assert_raw_rejects(
            &wit_p3,
            s_p3_sc05,
            "PS before P3: SC-05 OOB child ConstId rejected even on P3 refusal",
        );

        // Now on P6 (`bounds_with(100, 1)`):
        let b_p6 = bounds_with(100, 1);
        let (adm_p6, mut img_p6, _) = reach_core(b_p6, &w_spec, "raw P6 Cons", make);
        let ObservedStep::Event(rec_p6) = (match owner_record(&adm_p6, &mut img_p6) {
            Ok(step) => step,
            Err(error) => panic!("P6: {}", capture_step_error_label(&error)),
        }) else {
            panic!("P6 event");
        };
        let wit_p6 =
            witness_from_owner_record(&rec_p6, &core_cfg(b_p6), &limits()).expect("P6 witness");
        let base_p6 = wit_p6.to_statement_frs();
        assert_raw_ok(&wit_p6, base_p6.clone(), "raw P6 Cons positive");

        let mut s_p6_sc05 = base_p6;
        replace_prog_in_statement(&wit_p6, &mut s_p6_sc05, &p_sc05);
        assert_raw_rejects(
            &wit_p6,
            s_p6_sc05,
            "PS before P6: SC-05 OOB child ConstId rejected even on P6 refusal",
        );
    }
}

#[test]
fn raw_nominal_constructor_all_six_adt_checked_ctor_arity_bounds_and_slice_len_separation_both_modes()
 {
    let one_app = run_ext(
        &field_spec(ConstKindWant::Constructor { arity: 1 }, true, 0),
        "nominal 1-field MkAlpha cursor=0",
        &|| nominal_ctor_one_field_program(false),
    );
    let w1 = &one_app.witness;
    let base1 = w1.to_statement_frs();
    assert_raw_ok(w1, base1.clone(), "nominal 1-field MkAlpha base");
    let prog1 = &one_app.record.before.admission.view;
    let const_id1 = w1.pre_helper[L_TASKS + 2] as usize;
    let ConstantSnapshot::Constructor {
        type_id: orig_type_id,
        ctor_id: orig_ctor_id,
        fields: orig_fields,
        marker_free: orig_mf,
    } = prog1.constants[const_id1]
    else {
        panic!("expected Constructor constant");
    };
    assert_eq!(orig_type_id, TypeIdC(4));
    assert_eq!(orig_ctor_id, CtorIdC(0));
    assert_eq!(orig_fields, ConstFieldsSnapshot { start: 0, len: 1 });

    // Bound 1 (`type_id >= 4`):
    for built_in_tid in [0usize, 3] {
        let mut p = prog1.clone();
        p.constants[const_id1] = ConstantSnapshot::Constructor {
            marker_free: orig_mf,
            type_id: TypeIdC(built_in_tid),
            ctor_id: orig_ctor_id,
            fields: orig_fields,
        };
        p.types[built_in_tid] = p.types[orig_type_id.0];
        p.constructors[orig_ctor_id.0].owning_type = TypeIdC(built_in_tid);
        let mut s = base1.clone();
        replace_prog_in_statement(w1, &mut s, &p);
        assert_raw_rejects(
            w1,
            s,
            &format!(
                "adt_checked_ctor_arity bound 1: built-in type_id {built_in_tid} < 4 rejected"
            ),
        );
    }

    // Bound 2 (`type_id < type_count`):
    {
        let oob_tid = prog1.types.len();
        let mut p = prog1.clone();
        p.constants[const_id1] = ConstantSnapshot::Constructor {
            marker_free: orig_mf,
            type_id: TypeIdC(oob_tid),
            ctor_id: orig_ctor_id,
            fields: orig_fields,
        };
        p.constructors[orig_ctor_id.0].owning_type = TypeIdC(oob_tid);
        let mut s = base1.clone();
        replace_prog_in_statement(w1, &mut s, &p);
        assert_raw_rejects(
            w1,
            s,
            "adt_checked_ctor_arity bound 2: type_id == type_count rejected",
        );
    }

    // Bound 3 (`ctor_id < ctor_count`):
    {
        let oob_cid = prog1.constructors.len();
        let mut p = prog1.clone();
        p.constants[const_id1] = ConstantSnapshot::Constructor {
            marker_free: orig_mf,
            type_id: orig_type_id,
            ctor_id: CtorIdC(oob_cid),
            fields: orig_fields,
        };
        p.types[orig_type_id.0].ctor_len = oob_cid + 1;
        let mut s = base1.clone();
        replace_prog_in_statement(w1, &mut s, &p);
        assert_raw_rejects(
            w1,
            s,
            "adt_checked_ctor_arity bound 3: ctor_id == ctor_count rejected",
        );
    }

    // Bound 4 (`c_row.owning_type == type_id`):
    {
        let mut p = prog1.clone();
        p.constructors[orig_ctor_id.0].owning_type = TypeIdC(5);
        let mut s = base1.clone();
        replace_prog_in_statement(w1, &mut s, &p);
        assert_raw_rejects(
            w1,
            s,
            "adt_checked_ctor_arity bound 4: crow.owning_type != type_id rejected",
        );
    }

    // Bound 5a (`ctor_id >= t_row.ctor_start`) and 5b (`ctor_id < t_row.ctor_start + t_row.ctor_len`):
    {
        let mut p = prog1.clone();
        p.types[orig_type_id.0].ctor_start = CtorIdC(1);
        let mut s = base1.clone();
        replace_prog_in_statement(w1, &mut s, &p);
        assert_raw_rejects(
            w1,
            s,
            "adt_checked_ctor_arity bound 5a: ctor_id < trow.ctor_start rejected",
        );

        let mut p2 = prog1.clone();
        p2.types[orig_type_id.0].ctor_len = 0;
        let mut s2 = base1.clone();
        replace_prog_in_statement(w1, &mut s2, &p2);
        assert_raw_rejects(
            w1,
            s2,
            "adt_checked_ctor_arity bound 5b: ctor_id >= trow.ctor_start + trow.ctor_len rejected",
        );
    }

    // Bound 6a (`field_start + field_len <= ctor_field_count`) and 6b (129-bit overflow):
    {
        let mut p = prog1.clone();
        p.constructors[orig_ctor_id.0].field_start = p.ctor_fields.len();
        let mut s = base1.clone();
        replace_prog_in_statement(w1, &mut s, &p);
        assert_raw_rejects(
            w1,
            s,
            "adt_checked_ctor_arity bound 6a: field_start + field_len > ctor_field_count rejected",
        );

        let mut p2 = prog1.clone();
        p2.constructors[orig_ctor_id.0].field_start = usize::MAX;
        let mut s2 = base1.clone();
        replace_prog_in_statement(w1, &mut s2, &p2);
        assert_raw_rejects(
            w1,
            s2,
            "adt_checked_ctor_arity bound 6b: field_start=usize::MAX + field_len=1 rejected",
        );
    }

    // Separation of `field_at` (`reader.slice_len`) vs `GValidateFieldNontail` (`nominal_capture.field_len`):
    // (A) On 1-field `Alpha::MkAlpha` (declared `field_len == 1`), spoof `ConstantSnapshot::Constructor.fields.len = 2`
    //     so `reader.slice_len == 2` while declared `checked_arity == 1`. At `cursor == 0`:
    //     - `field_at` checks `0 < reader.slice_len (2)` (valid) and reads `constant_fields[0]`.
    //     - `GValidateFieldNontail` checks `0 + 1 < checked_arity (1)` (FALSE -> tail!).
    //     - Therefore the tail-replacement post-state (`post_helper[3] == 1`) MUST PASS, while a
    //       non-tail push post-state (`post_helper[3] == 2`, following spoofed `slice_len == 2`) MUST REJECT!
    {
        let mut p_spoof2 = prog1.clone();
        p_spoof2.constants[const_id1] = ConstantSnapshot::Constructor {
            marker_free: orig_mf,
            type_id: orig_type_id,
            ctor_id: orig_ctor_id,
            fields: ConstFieldsSnapshot { start: 0, len: 2 },
        };
        let mut s_tail_ok = base1.clone();
        replace_prog_in_statement(w1, &mut s_tail_ok, &p_spoof2);
        assert_raw_ok(
            w1,
            s_tail_ok.clone(),
            "1-field Constructor with spoofed slice_len=2 still takes declared field_len=1 tail replacement",
        );
        let child_cid = w1.post_helper[L_TASKS + 2];
        let mut s_nontail_bad = s_tail_ok;
        s_nontail_bad[post_helper_at(w1, 3)] = Fr::from(2u64);
        s_nontail_bad[task_rel(w1, true, 0, 2)] = Fr::from(const_id1 as u128);
        s_nontail_bad[task_rel(w1, true, 0, 3)] = Fr::from(1u64);
        s_nontail_bad[task_rel(w1, true, 0, 4)] = Fr::from(1u64);
        s_nontail_bad[task_rel(w1, true, 1, 2)] = Fr::from(child_cid);
        assert_raw_rejects(
            w1,
            s_nontail_bad,
            "1-field Constructor with spoofed slice_len=2 rejects non-tail push post-state",
        );
    }

    // (B) On 2-field `Pair::MkPair` (declared `field_len == 2`) at `cursor == 0`, spoof
    //     `ConstantSnapshot::Constructor.fields.len = 1` so `reader.slice_len == 1` while
    //     declared `checked_arity == 2`. At `cursor == 0`:
    //     - `field_at` checks `0 < reader.slice_len (1)` (valid!) and reads `constant_fields[0]`.
    //     - `GValidateFieldNontail` checks `0 + 1 < checked_arity (2)` (TRUE -> non-tail!).
    //     - Therefore the non-tail push post-state (`post_helper[3] == 2`) MUST PASS, while a
    //       tail-replacement post-state (`post_helper[3] == 1`, following spoofed `slice_len == 1`) MUST REJECT!
    {
        let two_app = run_ext(
            &field_spec(ConstKindWant::Constructor { arity: 2 }, true, 0),
            "nominal 2-field MkPair cursor=0",
            &nominal_ctor_two_fields_program,
        );
        let w2 = &two_app.witness;
        let base2 = w2.to_statement_frs();
        let prog2 = &two_app.record.before.admission.view;
        let const_id2 = w2.pre_helper[L_TASKS + 2] as usize;
        let ConstantSnapshot::Constructor {
            type_id: tid2,
            ctor_id: cid2,
            fields: f2,
            marker_free: mf2,
        } = prog2.constants[const_id2]
        else {
            panic!("expected Constructor constant");
        };
        let mut p_spoof1 = prog2.clone();
        p_spoof1.constants[const_id2] = ConstantSnapshot::Constructor {
            marker_free: mf2,
            type_id: tid2,
            ctor_id: cid2,
            fields: ConstFieldsSnapshot {
                start: f2.start,
                len: 1,
            },
        };
        let mut s_nontail_ok = base2;
        replace_prog_in_statement(w2, &mut s_nontail_ok, &p_spoof1);
        assert_raw_ok(
            w2,
            s_nontail_ok.clone(),
            "2-field Constructor at cursor=0 with spoofed slice_len=1 still takes declared field_len=2 non-tail push",
        );
        let child_cid2 = w2.post_helper[L_TASKS + TASK_LANES + 2];
        let mut s_tail_bad = s_nontail_ok;
        s_tail_bad[post_helper_at(w2, 3)] = Fr::from(1u64);
        s_tail_bad[task_rel(w2, true, 0, 2)] = Fr::from(child_cid2);
        s_tail_bad[task_rel(w2, true, 0, 3)] = Fr::from(0u64);
        s_tail_bad[task_rel(w2, true, 0, 4)] = Fr::from(0u64);
        for rel in 0..TASK_LANES {
            s_tail_bad[task_rel(w2, true, 1, rel)] = Fr::from(0u64);
        }
        assert_raw_rejects(
            w2,
            s_tail_bad,
            "2-field Constructor at cursor=0 with spoofed slice_len=1 rejects tail replacement post-state",
        );
    }
}

#[test]
fn raw_nontail_and_tail_task_slots_lower_preservation_and_inactive_padding_both_modes() {
    // 1. Depth-1 non-tail (`cons_nontail_witness`, `tasks.len(): 1 -> 2`):
    let nt1 = cons_nontail_witness();
    let w_nt1 = &nt1.witness;
    let base_nt1 = w_nt1.to_statement_frs();
    let parent_cid = w_nt1.pre_helper[L_TASKS + 2];
    let child_cid = w_nt1.post_helper[L_TASKS + TASK_LANES + 2];
    let other_cid = if child_cid == 0 { 1 } else { 0 };

    for bad_count in [0u64, 1, 3] {
        let mut s = base_nt1.clone();
        s[post_helper_at(w_nt1, 3)] = Fr::from(bad_count);
        assert_raw_rejects(
            w_nt1,
            s,
            &format!("non-tail 1->2 wrong post_helper[3]={bad_count}"),
        );
    }
    // Tamper rewritten parent slot 0 (`[0, 0, parent_cid, 1, 1, 0, 0]`):
    for (rel, bad_val, what) in [
        (0usize, 1u128, "non-tail parent slot 0 tag EqualTask(1)"),
        (1, 1, "non-tail parent slot 0 ref_is_data Data(1)"),
        (
            2,
            child_cid,
            "non-tail parent slot 0 ref_index overwritten with child",
        ),
        (
            3,
            0,
            "non-tail parent slot 0 visit Unvisited(0) instead of Fields(1)",
        ),
        (4, 0, "non-tail parent slot 0 cursor left at 0 instead of 1"),
        (4, 2, "non-tail parent slot 0 cursor 2 instead of 1"),
        (5, 1, "non-tail parent slot 0 pad 5 non-zero"),
        (6, 1, "non-tail parent slot 0 pad 6 non-zero"),
    ] {
        let mut s = base_nt1.clone();
        s[task_rel(w_nt1, true, 0, rel)] = Fr::from(bad_val);
        assert_raw_rejects(w_nt1, s, what);
    }
    // Tamper pushed child slot 1 (`[0, 0, child_cid, 0, 0, 0, 0]`):
    for (rel, bad_val, what) in [
        (
            0usize,
            1u128,
            "non-tail pushed child slot 1 tag EqualTask(1)",
        ),
        (1, 1, "non-tail pushed child slot 1 ref_is_data Data(1)"),
        (2, other_cid, "non-tail pushed child slot 1 wrong ref_index"),
        (
            3,
            1,
            "non-tail pushed child slot 1 visit Fields(1) instead of Unvisited(0)",
        ),
        (4, 1, "non-tail pushed child slot 1 cursor 1 instead of 0"),
        (5, 1, "non-tail pushed child slot 1 pad 5 non-zero"),
        (6, 1, "non-tail pushed child slot 1 pad 6 non-zero"),
    ] {
        let mut s = base_nt1.clone();
        s[task_rel(w_nt1, true, 1, rel)] = Fr::from(bad_val);
        assert_raw_rejects(w_nt1, s, what);
    }
    // Inactive slot 2 non-zero:
    {
        let mut s = base_nt1;
        s[task_rel(w_nt1, true, 2, 2)] = Fr::from(parent_cid);
        assert_raw_rejects(w_nt1, s, "non-tail 1->2 inactive slot 2 non-zero");
    }

    // 2. Depth-2 non-tail (`depth2_nontail_witness`, `tasks.len(): 2 -> 3`):
    //    Lower slot 0 must be preserved unchanged while slot 1 is rewritten and slot 2 is pushed!
    let nt2 = depth2_nontail_witness();
    let w_nt2 = &nt2.witness;
    let base_nt2 = w_nt2.to_statement_frs();
    assert_eq!(w_nt2.pre_helper[3], 2);
    assert_eq!(w_nt2.post_helper[3], 3);
    for rel in 0..TASK_LANES {
        let mut s = base_nt2.clone();
        let at = task_rel(w_nt2, true, 0, rel);
        s[at] = if s[at] == Fr::from(0u64) {
            Fr::from(1u64)
        } else {
            Fr::from(0u64)
        };
        assert_raw_rejects(
            w_nt2,
            s,
            &format!("non-tail 2->3 lower slot 0 lane {rel} tampered"),
        );
    }

    // 3. Depth-2 tail (`depth2_tail_witness`, `tasks.len(): 2 -> 2`):
    //    Lower slot 0 must be preserved unchanged while slot 1 is replaced in place with child!
    let t2 = depth2_tail_witness();
    let w_t2 = &t2.witness;
    let base_t2 = w_t2.to_statement_frs();
    assert_eq!(w_t2.pre_helper[3], 2);
    assert_eq!(w_t2.post_helper[3], 2);
    for rel in 0..TASK_LANES {
        let mut s = base_t2.clone();
        let at = task_rel(w_t2, true, 0, rel);
        s[at] = if s[at] == Fr::from(0u64) {
            Fr::from(1u64)
        } else {
            Fr::from(0u64)
        };
        assert_raw_rejects(
            w_t2,
            s,
            &format!("tail 2->2 lower slot 0 lane {rel} tampered"),
        );
    }
    for (rel, bad_val, what) in [
        (
            3usize,
            1u64,
            "tail 2->2 replaced slot 1 visit Fields(1) instead of Unvisited(0)",
        ),
        (4, 1, "tail 2->2 replaced slot 1 cursor 1 instead of 0"),
    ] {
        let mut s = base_t2.clone();
        s[task_rel(w_t2, true, 1, rel)] = Fr::from(bad_val);
        assert_raw_rejects(w_t2, s, what);
    }
}

#[test]
fn raw_p3_p6_and_tail_at_bound_dispositions_precedence_and_counters_both_modes() {
    let w_cons = field_spec(ConstKindWant::Cons, true, 0);
    let make_cons = &|| prog(lit_list(&[7]));

    // 1. Genuine P6 witness (`bounds_with(100, 1)` on non-tail `Cons` at `cursor == 0`):
    let b_p6 = bounds_with(100, 1);
    let (adm_p6, mut img_p6, _) = reach_core(b_p6, &w_cons, "P6 Cons", make_cons);
    let ObservedStep::Event(rec_p6) = (match owner_record(&adm_p6, &mut img_p6) {
        Ok(step) => step,
        Err(error) => panic!("P6: {}", capture_step_error_label(&error)),
    }) else {
        panic!("P6 event");
    };
    let w_p6 = witness_from_owner_record(&rec_p6, &core_cfg(b_p6), &limits()).expect("P6 witness");
    assert!(!w_p6.disposition_is_refused_helper_work && w_p6.disposition_is_refused_helper_tasks);
    assert_ok_both(&w_p6, "genuine P6 Cons");
    let base_p6 = w_p6.to_statement_frs();

    // Flipping `s[1]` (`p3`) or `s[2]` (`p6`) on P6 rejects:
    {
        let mut s = base_p6.clone();
        s[2] = Fr::from(0u64);
        assert_raw_rejects(&w_p6, s, "P6 with disposition_is_refused_helper_tasks=0");

        let mut s = base_p6.clone();
        s[1] = Fr::from(1u64);
        assert_raw_rejects(&w_p6, s, "P6 with disposition_is_refused_helper_work=1");
    }
    // Wrong P6 outcome (`Running` `[0,0,0]` or `HelperWork` `[3,5,0]` instead of `HelperTasks` `[3,4,0]`):
    for (off, val, what) in [
        (
            0usize,
            0u64,
            "P6 outcome tag Running(0) instead of Exhausted(3)",
        ),
        (1, 0, "P6 ExhaustReason Fuel(0) instead of HelperTasks(4)"),
        (
            1,
            5,
            "P6 ExhaustReason HelperWork(5) instead of HelperTasks(4)",
        ),
        (2, 1, "P6 outcome lane 2 non-zero"),
    ] {
        let mut s = base_p6.clone();
        s[post_image_at(&w_p6, L_OUTCOME + off)] = Fr::from(val);
        assert_raw_rejects(&w_p6, s, what);
    }
    // P6 must charge `remaining_work - 1`, `consumed_work + 1`, `helper_work + 1` and keep `helper_visits` unchanged:
    for (lane, what) in [
        (L_REMAINING_WORK, "P6 remaining_work uncharged"),
        (L_CONSUMED_WORK, "P6 consumed_work uncharged"),
        (L_HELPER_WORK, "P6 helper_work uncharged"),
    ] {
        let mut s = base_p6.clone();
        s[post_image_at(&w_p6, lane)] = s[pre_image_at(&w_p6, lane)];
        assert_raw_rejects(&w_p6, s, what);
    }
    {
        let mut s = base_p6.clone();
        s[post_image_at(&w_p6, L_HELPER_VISITS)] =
            s[pre_image_at(&w_p6, L_HELPER_VISITS)] + Fr::from(1u64);
        assert_raw_rejects(&w_p6, s, "P6 charging helper_visits + 1 rejected");
    }
    // P6 must preserve `post_helper == pre_helper` (rewriting parent cursor to 1 rejects):
    {
        let mut s = base_p6.clone();
        s[task_rel(&w_p6, true, 0, 4)] = Fr::from(1u64);
        assert_raw_rejects(&w_p6, s, "P6 rewriting parent cursor to 1 rejected");
    }
    // On P6, uncharged `helper_visits == u128::MAX` (`post == pre == MAX`) PASSES, while
    // `consumed_work == u128::MAX` REJECTS (charged by P4):
    {
        let mut s_hv_max = base_p6.clone();
        s_hv_max[pre_image_at(&w_p6, L_HELPER_VISITS)] = Fr::from(MAX);
        s_hv_max[post_image_at(&w_p6, L_HELPER_VISITS)] = Fr::from(MAX);
        assert_raw_ok(
            &w_p6,
            s_hv_max,
            "P6 uncharged helper_visits == u128::MAX passes",
        );

        let mut s_cw_max = base_p6;
        s_cw_max[pre_image_at(&w_p6, L_CONSUMED_WORK)] = Fr::from(MAX);
        s_cw_max[post_image_at(&w_p6, L_CONSUMED_WORK)] = Fr::from(0u64);
        assert_raw_rejects(
            &w_p6,
            s_cw_max,
            "P6 charged consumed_work at u128::MAX rejects",
        );
    }

    // 2. P3 dominates would-be P6 (`bounds_with(2, 1)` on non-tail `Cons` at `cursor == 0`):
    let b_p3_dom = bounds_with(2, 1);
    let (adm_p3, mut img_p3, _) = reach_core(b_p3_dom, &w_cons, "P3 dom P6 Cons", make_cons);
    let ObservedStep::Event(rec_p3) = (match owner_record(&adm_p3, &mut img_p3) {
        Ok(step) => step,
        Err(error) => panic!("P3: {}", capture_step_error_label(&error)),
    }) else {
        panic!("P3 event");
    };
    let w_p3 =
        witness_from_owner_record(&rec_p3, &core_cfg(b_p3_dom), &limits()).expect("P3 witness");
    assert!(w_p3.disposition_is_refused_helper_work && !w_p3.disposition_is_refused_helper_tasks);
    let base_p3 = w_p3.to_statement_frs();
    assert_raw_ok(&w_p3, base_p3.clone(), "P3 dominates would-be P6 positive");

    // Claiming `p6 = 1` (either with `p3 = 1` or `p3 = 0` and P6 post-state) when `pre.helper_work == max_helper_work` rejects!
    {
        let mut s_both = base_p3.clone();
        s_both[2] = Fr::from(1u64);
        assert_raw_rejects(
            &w_p3,
            s_both,
            "P3-dominates-P6: claiming p6=1 alongside p3=1 rejected",
        );

        let mut s_p6_instead = base_p3;
        s_p6_instead[1] = Fr::from(0u64);
        s_p6_instead[2] = Fr::from(1u64);
        s_p6_instead[post_image_at(&w_p3, L_REMAINING_WORK)] =
            s_p6_instead[pre_image_at(&w_p3, L_REMAINING_WORK)] - Fr::from(1u64);
        s_p6_instead[post_image_at(&w_p3, L_CONSUMED_WORK)] =
            s_p6_instead[pre_image_at(&w_p3, L_CONSUMED_WORK)] + Fr::from(1u64);
        s_p6_instead[post_image_at(&w_p3, L_HELPER_WORK)] =
            s_p6_instead[pre_image_at(&w_p3, L_HELPER_WORK)] + Fr::from(1u64);
        s_p6_instead[post_image_at(&w_p3, L_OUTCOME + 1)] = Fr::from(4u64); // HelperTasks
        assert_raw_rejects(
            &w_p3,
            s_p6_instead,
            "P3-dominates-P6: taking P6 instead of P3 when helper_work==max rejected",
        );
    }
}

#[test]
fn raw_in_bounds_data_parent_and_data_child_accepted_by_bare_circuit_vs_oob_rejected_both_modes() {
    // Fixture `Let xs = Cons(4, [5]) in 20` produces live DATA in `pre_data`:
    // - `Data(0)` is `DataSnapshot::U128(4)` (`[0, 4, 0, 0]`),
    // - `Data(1)` is `DataSnapshot::Cons { fields: DataFieldsSnapshot { start: 0, len: 2 } }` (`[3, 0, 2, 0]`),
    // - `data_fields[0]` is `RefC::Data(0)` (`[1, 0]`),
    // - `data_fields[1]` is `RefC::Const(const_id_of_list_5)` (`[0, const_id]`).
    let make = || CoreProgram {
        functions: Vec::new(),
        entry: CoreExpr::Let {
            name: "xs".into(),
            value: Box::new(bin(BinaryOp::Cons, lit_u(4), lit_list(&[5]))),
            body: Box::new(lit_tree(tree_leaf(20))),
        },
    };
    let w_spec = FieldSpec {
        parent_kind: ConstKindWant::Leaf,
        parent_marker_free: true,
        cursor: 0,
        min_expr_id: 1,
        min_tasks_len: 1,
    };
    let live = run_core(&w_spec, "DATA-bearing ValidateField", &make);
    let lw = &live.witness;
    assert_eq!(
        lw.pre_data[3], 2,
        "2 active DATA nodes: Data(0)=U128(4), Data(1)=Cons"
    );
    assert_eq!(
        lw.pre_data[4], 2,
        "2 active data_fields: [Data(0), Const(list_5)]"
    );
    let data_geom = c6_data_geometry(&lw.source_params.geometry().caps).expect("data geom");
    let df0_is_data = lw.pre_data[data_geom.fields_base];
    let df0_index = lw.pre_data[data_geom.fields_base + 1];
    let df1_is_data = lw.pre_data[data_geom.fields_base + REF_LANES];
    let df1_index = lw.pre_data[data_geom.fields_base + REF_LANES + 1];
    assert_eq!(
        (df0_is_data, df0_index),
        (1, 0),
        "data_fields[0] is RefC::Data(0)"
    );
    assert_eq!(df1_is_data, 0, "data_fields[1] is RefC::Const");

    // 1. Bare `BHelperValidateField` on `RefC::Data(1)` (`Cons`) at `cursor = 0` (non-tail `1 -> 2`):
    //    `field_at(Data(1), 0)` resolves `data_fields[0] = RefC::Data(0)` (`is_data=1, index=0`),
    //    rewrites slot 0 to `Data(1), Fields, cursor=1`, and pushes `Data(0), Unvisited, cursor=0` at slot 1!
    let mut s_data_c0 = lw.to_statement_frs();
    s_data_c0[task_rel(lw, false, 0, 1)] = Fr::from(1u64); // Data
    s_data_c0[task_rel(lw, false, 0, 2)] = Fr::from(1u64); // Data(1) = Cons
    s_data_c0[task_rel(lw, false, 0, 3)] = Fr::from(1u64); // Fields
    s_data_c0[task_rel(lw, false, 0, 4)] = Fr::from(0u64); // cursor = 0
    s_data_c0[post_helper_at(lw, 3)] = Fr::from(2u64); // non-tail 1 -> 2
    s_data_c0[task_rel(lw, true, 0, 1)] = Fr::from(1u64); // parent Data(1)
    s_data_c0[task_rel(lw, true, 0, 2)] = Fr::from(1u64);
    s_data_c0[task_rel(lw, true, 0, 3)] = Fr::from(1u64); // Fields
    s_data_c0[task_rel(lw, true, 0, 4)] = Fr::from(1u64); // cursor = 1
    s_data_c0[task_rel(lw, true, 1, 1)] = Fr::from(df0_is_data); // child Data(0)
    s_data_c0[task_rel(lw, true, 1, 2)] = Fr::from(df0_index);
    assert_raw_ok(
        lw,
        s_data_c0.clone(),
        "bare C6ValidateFieldStepEmitter accepts in-bounds RefC::Data(1) cursor=0 -> child RefC::Data(0)",
    );
    // Flipping pushed child `is_data` from `1` (`Data`) to `0` (`Const`) rejects:
    let mut s_bad_disc0 = s_data_c0;
    s_bad_disc0[task_rel(lw, true, 1, 1)] = Fr::from(0u64);
    assert_raw_rejects(
        lw,
        s_bad_disc0,
        "Data(1) cursor=0 rejects flipping child discriminator from Data(1) to Const(0)",
    );

    // 2. Bare `BHelperValidateField` on `RefC::Data(1)` (`Cons`) at `cursor = 1` (tail `1 -> 1`):
    //    `field_at(Data(1), 1)` resolves `data_fields[1] = RefC::Const(df1_index)` (`is_data=0`),
    //    replacing slot 0 in place!
    let mut s_data_c1 = lw.to_statement_frs();
    s_data_c1[task_rel(lw, false, 0, 1)] = Fr::from(1u64); // Data
    s_data_c1[task_rel(lw, false, 0, 2)] = Fr::from(1u64); // Data(1) = Cons
    s_data_c1[task_rel(lw, false, 0, 3)] = Fr::from(1u64); // Fields
    s_data_c1[task_rel(lw, false, 0, 4)] = Fr::from(1u64); // cursor = 1
    s_data_c1[post_helper_at(lw, 3)] = Fr::from(1u64); // tail 1 -> 1
    s_data_c1[task_rel(lw, true, 0, 1)] = Fr::from(df1_is_data); // child Const(df1_index)
    s_data_c1[task_rel(lw, true, 0, 2)] = Fr::from(df1_index);
    s_data_c1[task_rel(lw, true, 0, 3)] = Fr::from(0u64); // Unvisited
    s_data_c1[task_rel(lw, true, 0, 4)] = Fr::from(0u64); // cursor = 0
    assert_raw_ok(
        lw,
        s_data_c1.clone(),
        "bare C6ValidateFieldStepEmitter accepts in-bounds RefC::Data(1) cursor=1 -> child RefC::Const",
    );
    // Flipping replaced child `is_data` from `0` (`Const`) to `1` (`Data`) rejects:
    let mut s_bad_disc1 = s_data_c1.clone();
    s_bad_disc1[task_rel(lw, true, 0, 1)] = Fr::from(1u64);
    assert_raw_rejects(
        lw,
        s_bad_disc1,
        "Data(1) cursor=1 rejects flipping child discriminator from Const(0) to Data(1)",
    );
    // Mutating `data_fields[1]` in `pre_data`/`post_data` to `RefC::Const(const_count)` (`SC-05`) rejects:
    let const_count = live.record.before.admission.view.constants.len() as u128;
    let mut s_data_sc05 = s_data_c1;
    s_data_sc05[pre_data_at(lw, data_geom.fields_base + REF_LANES + 1)] = Fr::from(const_count);
    s_data_sc05[post_data_at(lw, data_geom.fields_base + REF_LANES + 1)] = Fr::from(const_count);
    s_data_sc05[task_rel(lw, true, 0, 2)] = Fr::from(const_count);
    assert_raw_rejects(
        lw,
        s_data_sc05,
        "SC-05: data_fields[1] pointing to RefC::Const(const_count) rejected by child_ref_index < const_count",
    );
}

#[test]
fn raw_disabled_identity_nonidentity_range_and_statement_commitment_tamper_both_modes() {
    let a = cons_nontail_witness();
    let w = &a.witness;
    let base = w.to_statement_frs();

    // 1. Disabled identity (`enable == 0`) passes for all 4 boolean combinations of `(p3, p6)`,
    //    and even when `pre == post` carries an arbitrary 128-bit non-canonical helper lane.
    for p3 in [0u64, 1] {
        for p6 in [0u64, 1] {
            let mut s = disabled_statement(w);
            s[1] = Fr::from(p3);
            s[2] = Fr::from(p6);
            assert_raw_ok(
                w,
                s,
                &format!("disabled identity with boolean dispositions p3={p3}, p6={p6}"),
            );
        }
    }
    {
        let mut s = disabled_statement(w);
        s[task_rel(w, false, 0, 1)] = Fr::from(99u64);
        s[task_rel(w, true, 0, 1)] = Fr::from(99u64);
        assert_raw_ok(
            w,
            s,
            "disabled identity accepts arbitrary 128-bit pre==post in top_task[1]",
        );
    }

    // 2. Disabled non-identity (`enable == 0` with `post != pre`) rejects across every section.
    for (at, what) in [
        (post_meta_at(w, 3), "disabled wrong post META"),
        (post_prog_at(w, 2), "disabled wrong post PROG"),
        (post_ctx_at(w, 2), "disabled wrong post CTX"),
        (
            post_image_at(w, L_CONSUMED_WORK),
            "disabled wrong post IMAGE",
        ),
        (post_data_at(w, 3), "disabled wrong post DATA"),
        (post_helper_at(w, 3), "disabled wrong post HELPER"),
    ] {
        let mut s = disabled_statement(w);
        s[at] = s[at] + Fr::from(1u64);
        assert_raw_rejects(w, s, what);
    }

    // 3. `Fr >= 2^128` range rejections, non-boolean `enable = 2` / `p3 = 2` / `p6 = 2`,
    //    wrong `host_schema`, wrong `carrier_version`.
    let big = two_pow_128();
    for (at, what) in [
        (pre_image_at(w, L_HELPER_VISITS), "pre image Fr>=2^128"),
        (post_image_at(w, L_HELPER_VISITS), "post image Fr>=2^128"),
        (helper_rel(w, false, 8), "pre helper Fr>=2^128"),
        (helper_rel(w, true, 8), "post helper Fr>=2^128"),
        (pre_data_at(w, 3), "pre DATA Fr>=2^128"),
        (post_data_at(w, 3), "post DATA Fr>=2^128"),
        (pre_ctx_at(w, 20), "pre CTX Fr>=2^128"),
        (post_ctx_at(w, 20), "post CTX Fr>=2^128"),
        (pre_prog_at(w, 3), "pre PROG Fr>=2^128"),
        (pre_meta_at(w, 3), "pre META Fr>=2^128"),
    ] {
        let mut s = base.clone();
        s[at] = big;
        assert_raw_rejects(w, s, what);
    }
    for (idx, what) in [
        (0usize, "enable=2"),
        (1, "disposition_is_refused_helper_work=2"),
        (2, "disposition_is_refused_helper_tasks=2"),
    ] {
        let mut s = base.clone();
        s[idx] = Fr::from(2u64);
        assert_raw_rejects(w, s, what);
    }
    let mut s = base.clone();
    s[3] = Fr::from(9u64);
    assert_raw_rejects(w, s, "wrong host_schema");
    let mut s = base.clone();
    s[4] = Fr::from(9u64);
    assert_raw_rejects(w, s, "wrong carrier_version");

    // 4. Statement/commitment tamper in `LocalPublic` and `LedgerCall` modes:
    {
        let c_local = circuit_for(w, C6PushStatementBindingMode::LocalPublic);
        let mut bad_out = base.clone();
        bad_out[post_image_at(w, L_CONSUMED_WORK)] =
            bad_out[post_image_at(w, L_CONSUMED_WORK)] + Fr::from(1u64);
        let preimage = ProofPreimage {
            inputs: Vec::new(),
            private_transcript: Vec::new(),
            public_transcript_inputs: base.clone(),
            public_transcript_outputs: bad_out,
            binding_input: Fr::from(0u64),
            communications_commitment: None,
            key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
        };
        assert!(preimage.check(&c_local.ir).is_err());
    }
    {
        let c_ledger = circuit_for(w, C6PushStatementBindingMode::LedgerCall);
        let randomness = Fr::from(0u64);
        let mut stale_inputs = base.clone();
        stale_inputs.extend(base[post_state_start(w)..].iter().copied());
        let stale = transient_commit(&stale_inputs, randomness);

        let mut mutated = base;
        mutated[pre_image_at(w, L_HELPER_VISITS)] = Fr::from(10u64);
        mutated[post_image_at(w, L_HELPER_VISITS)] = Fr::from(11u64);
        assert!(check_raw(&c_ledger, w, mutated.clone(), None).is_ok());
        match check_raw(&c_ledger, w, mutated, Some(stale)) {
            Err(C6ValidateFieldConsumerError::Backend(m)) => assert!(
                m.contains("Communications commitment mismatch"),
                "stale CC category: {m}"
            ),
            other => panic!("stale CC: expected rejection, ok={}", other.is_ok()),
        }
    }
}

#[test]
fn raw_wrapper_only_refusals_labelled_and_separated_from_sdk_check() {
    let a = leaf_tail_witness();
    let w = a.witness.clone();
    let c_local = circuit_for(&w, C6PushStatementBindingMode::LocalPublic);
    let c_ledger = circuit_for(&w, C6PushStatementBindingMode::LedgerCall);

    for (sec, mutate) in [
        (
            "meta",
            (|s: &mut HostSnapshot| s.admission.bounds.frames += 1) as fn(&mut HostSnapshot),
        ),
        ("prog", |s: &mut HostSnapshot| s.admission.view.entry += 1),
    ] {
        let mut bad_after = a.record.after.clone();
        mutate(&mut bad_after);
        match witness_from_host_pair(
            true,
            &a.record.before,
            &bad_after,
            &core_cfg(bounds(100)),
            &limits(),
        ) {
            Err(C6ValidateFieldConsumerError::HostPrefixMismatch { section }) => {
                assert_eq!(section, sec);
            }
            other => panic!("expected HostPrefixMismatch({sec}), ok={}", other.is_ok()),
        }
    }

    let mut short_meta = w.clone();
    short_meta.host_prefix.meta.pop();
    assert!(matches!(
        check_c6_validate_field_step_circuit(&c_local, &short_meta),
        Err(C6ValidateFieldConsumerError::LengthMismatch {
            section: "meta",
            ..
        })
    ));
    let mut short_prog = w.clone();
    short_prog.host_prefix.prog.pop();
    assert!(matches!(
        check_c6_validate_field_step_circuit(&c_local, &short_prog),
        Err(C6ValidateFieldConsumerError::LengthMismatch {
            section: "prog",
            ..
        })
    ));
    let mut bad_off = w.clone();
    bad_off.host_prefix.meta_bounds_frames_offset += 1;
    assert!(matches!(
        check_c6_validate_field_step_circuit(&c_local, &bad_off),
        Err(C6ValidateFieldConsumerError::Backend(m)) if m.contains("meta bounds frames offset mismatch")
    ));
    let mut short_state = w.clone();
    short_state.pre_image.pop();
    assert!(matches!(
        check_c6_validate_field_step_circuit(&c_local, &short_state),
        Err(C6ValidateFieldConsumerError::LengthMismatch {
            section: "statement",
            ..
        })
    ));
    let mut short_frs = w.to_statement_frs();
    short_frs.pop();
    assert!(check_raw(&c_local, &w, short_frs.clone(), None).is_err());
    assert!(check_raw(&c_ledger, &w, short_frs, None).is_err());
}

fn imm(v: u128) -> Operand {
    Operand::Immediate(Fr::from(v))
}

fn valid_inputs(w: &C6ValidateFieldStepWitness) -> C6ValidateFieldStepInputs {
    let hp = &w.host_prefix;
    let ops = |xs: &[u128]| xs.iter().map(|&v| imm(v)).collect::<Vec<_>>();
    let ctx: [Operand; CTX_LANES] = hp
        .ctx
        .iter()
        .map(|&v| imm(v))
        .collect::<Vec<_>>()
        .try_into()
        .expect("ctx lanes");
    C6ValidateFieldStepInputs {
        enable: imm(u128::from(w.enable)),
        disposition_is_refused_helper_work: imm(u128::from(w.disposition_is_refused_helper_work)),
        disposition_is_refused_helper_tasks: imm(u128::from(w.disposition_is_refused_helper_tasks)),
        host_schema: imm(C6_HOST_SCHEMA),
        carrier_version: imm(C6_PADDED_CARRIER_VERSION),
        geometry: w.source_params.geometry(),
        meta_bounds_frames_offset: hp.meta_bounds_frames_offset,
        source_params: w.source_params.clone(),
        pre_meta: ops(&hp.meta),
        pre_prog: ops(&hp.prog),
        pre_ctx: ctx.clone(),
        pre_image: ops(&w.pre_image),
        pre_data: ops(&w.pre_data),
        pre_helper: ops(&w.pre_helper),
        post_meta: ops(&hp.meta),
        post_prog: ops(&hp.prog),
        post_ctx: ctx,
        post_image: ops(&w.post_image),
        post_data: ops(&w.post_data),
        post_helper: ops(&w.post_helper),
    }
}

fn gated_receipt_operands(b: &C6ValidateFieldStepBlock) -> Vec<Operand> {
    vec![
        b.reader.selected_found.clone(),
        b.reader.is_data.clone(),
        b.reader.kind_code.clone(),
        b.reader.is_uint.clone(),
        b.reader.is_bool.clone(),
        b.reader.is_nil.clone(),
        b.reader.is_cons.clone(),
        b.reader.is_leaf.clone(),
        b.reader.is_branch.clone(),
        b.reader.is_constructor.clone(),
        b.reader.is_marker.clone(),
        b.reader.marker_free_present.clone(),
        b.reader.marker_free.clone(),
        b.reader.uint_present.clone(),
        b.reader.uint_value.clone(),
        b.reader.bool_present.clone(),
        b.reader.bool_value.clone(),
        b.reader.ctor_present.clone(),
        b.reader.type_id.clone(),
        b.reader.ctor_id.clone(),
        b.reader.slice_present.clone(),
        b.reader.slice_start.clone(),
        b.reader.slice_len.clone(),
        b.top_task_found.clone(),
        b.top_task_tag.clone(),
        b.top_ref_is_data.clone(),
        b.top_ref_index.clone(),
        b.top_task_visit.clone(),
        b.top_task_cursor.clone(),
        b.field_pos.clone(),
        b.field_found.clone(),
        b.child_ref_is_data.clone(),
        b.child_ref_index.clone(),
        b.nominal_valid.clone(),
        b.checked_arity.clone(),
        b.next_cursor.clone(),
        b.is_nontail.clone(),
    ]
}

fn ungated_receipt_operands(b: &C6ValidateFieldStepBlock) -> Vec<Operand> {
    vec![
        b.nominal_capture.type_count.clone(),
        b.nominal_capture.type_found.clone(),
        b.nominal_capture.ctor_found.clone(),
        b.nominal_capture.owning_type.clone(),
        b.nominal_capture.ctor_start.clone(),
        b.nominal_capture.ctor_len.clone(),
        b.nominal_capture.field_start.clone(),
        b.nominal_capture.field_len.clone(),
        b.nominal_capture.ctor_field_count.clone(),
        b.const_field_count.clone(),
        b.const_field_found.clone(),
        b.const_field_value.clone(),
        b.data_field_count.clone(),
        b.data_field_found.clone(),
        b.data_field_is_data.clone(),
        b.data_field_index.clone(),
        b.is_helper_work_max.clone(),
        b.is_helper_tasks_refused.clone(),
        b.is_applied.clone(),
        b.post_task_count.clone(),
        b.post_top_task_tag.clone(),
        b.post_top_ref_is_data.clone(),
        b.post_top_ref_index.clone(),
        b.post_top_task_visit.clone(),
        b.post_top_task_cursor.clone(),
        b.post_rewritten_parent_cursor.clone(),
    ]
}

fn check_receipt_claims(
    inputs: &C6ValidateFieldStepInputs,
    expected_gated: &[u128; 37],
    expected_ungated: &[u128; 26],
) -> Result<(), String> {
    let block = emit_c6_validate_field_step_block(inputs, "fld_receipt_claim")
        .map_err(|e| e.to_string())?;
    let gated = gated_receipt_operands(&block);
    let ungated = ungated_receipt_operands(&block);
    assert_eq!(gated.len(), 37);
    assert_eq!(ungated.len(), 26);
    let mut ins = Vec::new();
    let mut tr = Vec::new();
    let total_claims = gated.len() + ungated.len();
    for i in 0..total_claims {
        let id = Identifier(format!("%fld_receipt_claim.public.{i}"));
        ins.push(Instruction::PublicInput {
            guard: None,
            val_t: IrType::Native,
            output: id.clone(),
        });
        tr.push(Operand::Variable(id));
    }
    ins.extend(block.instructions);
    for (actual, exp) in gated.into_iter().chain(ungated).zip(tr.iter()) {
        ins.push(Instruction::ConstrainEq {
            a: actual,
            b: exp.clone(),
        });
    }
    ins.push(Instruction::Impact {
        guard: Operand::Immediate(Fr::from(1u64)),
        inputs: tr,
    });
    let ir0 = IrSource {
        version: IrMinorVersion::V0,
        inputs: Vec::new(),
        outputs: Vec::new(),
        do_communications_commitment: false,
        instructions: Arc::new(ins),
    };
    let mut disk = serde_json::to_value(&ir0).map_err(|e| e.to_string())?;
    disk["version"] = json!({ "major": 3, "minor": 0 });
    let serialized = serde_json::to_string(&disk).map_err(|e| e.to_string())?;
    let loaded = IrSource::load(serialized.as_bytes()).map_err(|e| e.to_string())?;
    let mut statement: Vec<Fr> = expected_gated.iter().map(|&v| Fr::from(v)).collect();
    statement.extend(expected_ungated.iter().map(|&v| Fr::from(v)));
    let preimage = ProofPreimage {
        inputs: Vec::new(),
        private_transcript: Vec::new(),
        public_transcript_inputs: statement.clone(),
        public_transcript_outputs: statement,
        binding_input: Fr::from(0u64),
        communications_commitment: None,
        key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
    };
    preimage
        .check(&loaded)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[test]
fn raw_gated_vs_ungated_receipt_contract_by_actual_instructions_and_sdk_check() {
    let two_app = run_ext(
        &field_spec(ConstKindWant::Constructor { arity: 2 }, true, 0),
        "receipt contract 2-field MkPair cursor=0",
        &nominal_ctor_two_fields_program,
    );
    let w = &two_app.witness;
    let parent_cid = w.pre_helper[L_TASKS + 2];
    let child_cid = w.post_helper[L_TASKS + TASK_LANES + 2];

    let mut sym_inputs = valid_inputs(w);
    let en_var = Operand::Variable(Identifier("%sym.enable".into()));
    sym_inputs.enable = en_var.clone();
    let sym_block = emit_c6_validate_field_step_block(&sym_inputs, "sym_fld").expect("sym emit");

    let mut cond_zero_gate_by_out = std::collections::HashMap::new();
    let mut mul_inputs_by_out = std::collections::HashMap::new();
    for ins in &sym_block.instructions {
        match ins {
            Instruction::CondSelect {
                bit,
                b: Operand::Immediate(z),
                output,
                ..
            } if *z == Fr::from(0u64) => {
                cond_zero_gate_by_out.insert(output.0.clone(), bit.clone());
            }
            Instruction::Mul { a, b, output } => {
                mul_inputs_by_out.insert(output.0.clone(), (a.clone(), b.clone()));
            }
            _ => {}
        }
    }
    let gated_ops = gated_receipt_operands(&sym_block);
    for (idx, op) in gated_ops.iter().enumerate() {
        let Operand::Variable(id) = op else {
            panic!("gated receipt {idx} must be a Variable operand");
        };
        let gate_bit = cond_zero_gate_by_out.get(&id.0).unwrap_or_else(|| {
            panic!(
                "gated receipt {idx} ({}) must be defined by CondSelect(..., 0)",
                id.0
            )
        });
        if *op == sym_block.nominal_valid {
            let Operand::Variable(cg_id) = gate_bit else {
                panic!("nominal_valid gate must be ctor_gate variable");
            };
            let (ma, mb) = mul_inputs_by_out
                .get(&cg_id.0)
                .expect("ctor_gate defined by Mul");
            assert_eq!(*ma, en_var);
            assert_eq!(*mb, sym_block.reader.is_constructor);
        } else {
            assert_eq!(
                *gate_bit, en_var,
                "gated receipt {idx} ({}) must be gated directly by enable",
                id.0
            );
        }
    }

    let ledger_circuit = circuit_for(w, C6PushStatementBindingMode::LedgerCall);
    let output_vals = ledger_circuit
        .ir
        .instructions
        .iter()
        .find_map(|i| match i {
            Instruction::Output { vals } => Some(vals),
            _ => None,
        })
        .expect("LedgerCall circuit has Output instruction");
    assert_eq!(output_vals.len(), state_len(w));
    for op in output_vals {
        let Operand::Variable(id) = op else {
            panic!("Output operand must be variable");
        };
        assert!(
            id.0.contains(".input.post_image.")
                || id.0.contains(".input.post_data.")
                || id.0.contains(".input.post_helper."),
            "LedgerCall Output binds only constrained post carrier lanes, got {}",
            id.0
        );
    }

    // Verify concrete values via `ProofPreimage::check` on `Pair::MkPair(11, 22)` at `cursor == 0`:
    // `type_id = 6` (`Pair`), `ctor_id = 2` (`MkPair`), `slice_start = 0`, `slice_len = 2`,
    // `field_pos = 0`, `child_ref_index = child_cid`, `checked_arity = 2`, `next_cursor = 1`, `is_nontail = 1`.
    let exp_gated_en1: [u128; 37] = [
        1, 0, 6, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 6, 2, 1, 0, 2, // reader (23)
        1, 0, 0, parent_cid, 1,
        0, // top_task_found=1, tag=0, is_data=0, idx=parent_cid, visit=1, cursor=0
        0, 1, 0,
        child_cid, // field_pos=0, field_found=1, child_ref_is_data=0, child_ref_index=child_cid
        1, 2, 1, 1, // nominal_valid=1, checked_arity=2, next_cursor=1, is_nontail=1
    ];
    let exp_ungated_en1: [u128; 26] = [
        7, 1, 1, 6, 2, 1, 1, 2, 3, // nominal_capture (9)
        2, 1,
        child_cid, // const_field_count=2, const_field_found=1, const_field_value=child_cid
        0, 0, 0, 0, // data_field_count=0, found=0, is_data=0, index=0
        0, 0, 1, // is_helper_work_max=0, is_helper_tasks_refused=0, is_applied=1
        2, 0, 0, child_cid, 0, 0,
        1, // post_task_count=2, post_top_task=[0,0,child_cid,0,0], post_rewritten_parent_cursor=1
    ];
    let inp_en1 = valid_inputs(w);
    assert!(
        check_receipt_claims(&inp_en1, &exp_gated_en1, &exp_ungated_en1).is_ok(),
        "enable=1 Applied receipt claims must pass ProofPreimage::check"
    );
    let mut bad_gated = exp_gated_en1;
    bad_gated[36] = 0; // claim is_nontail == 0 when it is 1
    assert!(check_receipt_claims(&inp_en1, &bad_gated, &exp_ungated_en1).is_err());

    // `enable == 0` Disabled identity on the same `Pair::MkPair` pre-state:
    // All 37 gated receipt operands equal exact `0`, while ungated parser/table/candidate wires remain non-zero!
    let mut inp_en0 = valid_inputs(w);
    inp_en0.enable = imm(0);
    inp_en0.disposition_is_refused_helper_work = imm(0);
    inp_en0.disposition_is_refused_helper_tasks = imm(0);
    inp_en0.post_image = inp_en0.pre_image.clone();
    inp_en0.post_data = inp_en0.pre_data.clone();
    inp_en0.post_helper = inp_en0.pre_helper.clone();
    let exp_gated_en0 = [0u128; 37];
    let exp_ungated_en0: [u128; 26] = [
        7, 1, 1, 4, 0, 0, 0, 1, 3, // nominal_capture for reader.type_id=0, reader.ctor_id=0
        2, 1,
        child_cid, // const_field_count=2, const_field_found=1 (at field_pos_raw=0), const_field_value=child_cid
        0, 0, 0, 0, // data_field_*
        0, 0, 1, // is_helper_work_max=0, is_helper_tasks_refused=0, is_applied=1
        1, 0, 0, child_cid, 0, 0,
        1, // ungated candidate wires (is_nontail_raw = 0 when enable=0 so post_task_count=1)
    ];
    assert!(
        check_receipt_claims(&inp_en0, &exp_gated_en0, &exp_ungated_en0).is_ok(),
        "enable=0 gated receipts are all 0 while ungated receipts retain non-zero parser/candidate values"
    );
    assert!(check_receipt_claims(&inp_en0, &exp_gated_en0, &[0u128; 26]).is_err());
}

const SEGMENTS: [&str; 10] = [
    ".local.temp.",
    ".c6data.temp.",
    ".c6helper.temp.",
    ".c6meta.temp.",
    ".c6progctx.temp.",
    ".fldread.local.temp.",
    ".fldread.c6progctx.temp.",
    ".fldread.c6data.temp.",
    ".fldnom.c6progctx.temp.",
    ".fldcf.c6progctx.temp.",
];

fn dests(ins: &[Instruction]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for i in ins {
        match i {
            Instruction::Add { output, .. }
            | Instruction::Mul { output, .. }
            | Instruction::Neg { output, .. }
            | Instruction::Not { output, .. }
            | Instruction::CondSelect { output, .. }
            | Instruction::TestEq { output, .. }
            | Instruction::LessThan { output, .. }
            | Instruction::Copy { output, .. } => {
                out.insert(output.0.clone());
            }
            Instruction::DivModPowerOfTwo { outputs, .. } => {
                for o in outputs {
                    out.insert(o.0.clone());
                }
            }
            _ => {}
        }
    }
    out
}

#[test]
fn raw_namespace_all10_segments_all17_groups_retained_serial_and_free_single_shot() {
    let a = leaf_tail_witness();
    let w = &a.witness;
    for seg in SEGMENTS {
        let expected = format!("%validatefldraw{seg}77");
        let reserved = Operand::Variable(Identifier(expected.clone()));
        for group in 0..17usize {
            let mut inputs = valid_inputs(w);
            match group {
                0 => inputs.enable = reserved.clone(),
                1 => inputs.disposition_is_refused_helper_work = reserved.clone(),
                2 => inputs.disposition_is_refused_helper_tasks = reserved.clone(),
                3 => inputs.host_schema = reserved.clone(),
                4 => inputs.carrier_version = reserved.clone(),
                5 => inputs.pre_meta[0] = reserved.clone(),
                6 => inputs.pre_prog[0] = reserved.clone(),
                7 => inputs.pre_ctx[0] = reserved.clone(),
                8 => inputs.pre_image[0] = reserved.clone(),
                9 => inputs.pre_data[0] = reserved.clone(),
                10 => inputs.pre_helper[0] = reserved.clone(),
                11 => inputs.post_meta[0] = reserved.clone(),
                12 => inputs.post_prog[0] = reserved.clone(),
                13 => inputs.post_ctx[0] = reserved.clone(),
                14 => inputs.post_image[0] = reserved.clone(),
                15 => inputs.post_data[0] = reserved.clone(),
                _ => inputs.post_helper[0] = reserved.clone(),
            }
            let mut em = C6ValidateFieldStepEmitter::new("validatefldraw").expect("scope");
            match em.emit(&inputs) {
                Err(C6ValidateFieldStepError::ReservedOperand { identifier }) => {
                    assert_eq!(identifier, expected, "segment {seg} group {group}");
                }
                other => panic!(
                    "segment {seg} group {group}: expected ReservedOperand, ok={}",
                    other.is_ok()
                ),
            }
        }
        match C6ValidateFieldStepEmitter::new(&format!("bad{seg}x")) {
            Err(C6ValidateFieldStepError::InvalidScope) => {}
            other => panic!("scope {seg}: expected InvalidScope, ok={}", other.is_ok()),
        }
    }
    for empty in ["", "   "] {
        match C6ValidateFieldStepEmitter::new(empty) {
            Err(C6ValidateFieldStepError::EmptyScope) => {}
            other => panic!("empty scope {empty:?} ok={}", other.is_ok()),
        }
    }
    for bad_scope in [
        "bad%scope",
        "bad.fldread.scope",
        "bad.fldnom.scope",
        "bad.fldcf.scope",
    ] {
        match C6ValidateFieldStepEmitter::new(bad_scope) {
            Err(C6ValidateFieldStepError::InvalidScope) => {}
            other => panic!(
                "bad scope {bad_scope}: expected InvalidScope, ok={}",
                other.is_ok()
            ),
        }
    }

    let inputs = valid_inputs(w);
    let mut em = C6ValidateFieldStepEmitter::new("validatefldraw_repeat").expect("repeat scope");
    let first = em.emit(&inputs).expect("first retained emit");
    let after_first = em.next_temp();
    let second = em.emit(&inputs).expect("second retained emit");
    assert!(em.next_temp() > after_first);
    let (d1, d2) = (dests(&first.instructions), dests(&second.instructions));
    assert!(
        !d1.is_empty() && !d2.is_empty() && d1.is_disjoint(&d2),
        "retained repeated destinations must be disjoint"
    );

    let f1 = emit_c6_validate_field_step_block(&inputs, "validatefldraw_free").expect("free 1");
    let f2 = emit_c6_validate_field_step_block(&inputs, "validatefldraw_free").expect("free 2");
    assert_eq!(dests(&f1.instructions), dests(&f2.instructions));
    assert!(dests(&f1.instructions).is_disjoint(&d1));
}

#[test]
fn raw_emitter_length_geometry_producer_errors() {
    let a = leaf_tail_witness();
    let w = &a.witness;
    for (section, mutate) in [
        (
            "image",
            (|inp: &mut C6ValidateFieldStepInputs| {
                inp.pre_image.pop();
            }) as fn(&mut C6ValidateFieldStepInputs),
        ),
        ("data", |inp: &mut C6ValidateFieldStepInputs| {
            inp.post_data.pop();
        }),
        ("helper", |inp: &mut C6ValidateFieldStepInputs| {
            inp.post_helper.pop();
        }),
    ] {
        let mut inputs = valid_inputs(w);
        mutate(&mut inputs);
        let mut em = C6ValidateFieldStepEmitter::new("validatefldraw_len").expect("scope");
        match em.emit(&inputs) {
            Err(C6ValidateFieldStepError::LengthMismatch {
                section: actual_sec,
                ..
            }) => assert_eq!(actual_sec, section),
            other => panic!(
                "expected {section} LengthMismatch producer error, ok={}",
                other.is_ok()
            ),
        }
    }
}
