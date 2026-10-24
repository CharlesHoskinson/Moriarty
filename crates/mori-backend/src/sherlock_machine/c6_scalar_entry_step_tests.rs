//! Genuine-owner positive tests for `c6_scalar_entry_step` /
//! `c6_scalar_entry_consumer`.
//!
//! Source-only and UNRUN. Every positive fixture is admitted through
//! the real public owner (`host_owner_engine().admit` for Core,
//! `admit_ext_program` for Ext) and `admit_run`, then stepped with the
//! real public `complete_observation::record_step` until the actual
//! BEFORE-Entry pre-state: `ControlC::Return`, nonempty frame stack,
//! absent helper/result, Running, positive fuel, and an actual selected
//! TOP frame. Search stops immediately BEFORE Entry; the selected step
//! is then taken exactly once through the consumer wrapper
//! `record_scalar_entry_step` on the original image and exactly once
//! through raw `record_step` on a freshly replayed program. No private
//! `MachineImage`, synthesized `HostSnapshot`, fabricated helper, or
//! tuned-to-POST state is constructed for a positive baseline.
//! Mutations of a captured actual pair appear only in the clearly
//! labelled negative tests at the end.
//!
//! DELIVERED_SOURCE: all 11 shapes (RetRight Add/Sub/Mul/Lt/Cons/
//! Branch, RetUnary Leaf/Not, RetIf TestBool, RetCall TestType,
//! RetCtorFields unchecked TestType) in both `LocalPublic` and
//! `LedgerCall`; builtin `ReturnF` expected `0..=3`; nominal `>= 4`
//! `ReturnF`; genuine nonempty nominal `ConstructorFieldsF` with exact
//! nominal field type; Const and Data operand identities where the
//! genuine programs reach them; exact P3 at the original separate
//! admission `max_helper_work`; wrong-rule/terminal wrapper refusal
//! with the documented non-rollback behaviour; owner-record internal
//! consistency mutations and crossed-record rejection (internal
//! consistency only, never authenticated provenance); one encodable
//! wrong installed mode reaching the SDK checker in both modes.
//!
//! REQUIRED_OPEN / not claimed here: raw-domain exhaustive/namespace
//! vectors, broad adversarial raw mutation module, Fr >= 2^128 and
//! gate-domain negatives, SI08/SI15/SI17/SI20/SI21/SI22, full StepInv,
//! program membership, constructor-arity validation (the Entry plan
//! does not call it), native issuance, degree/k claims.

use super::c6_push_step::C6PushStatementBindingMode;
use super::{
    C6ScalarEntryConsumerError, C6ScalarEntryStepWitness, build_c6_scalar_entry_step_circuit,
    check_c6_scalar_entry_step_circuit, record_scalar_entry_step,
    witness_scalar_entry_from_host_pair, witness_scalar_entry_from_owner_record,
};
use mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets;
use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostConfig, C6HostLimits};
use mori_core::sherlock::helper_machine::complete_observation::{
    CaptureStepError, CompleteStepRecord, HostSnapshot, ImageSnapshot, ObservedStep, capture,
    record_step as owner_record,
};
use mori_core::sherlock::helper_machine::shared::adt::{CtorIdC, TypeIdC};
use mori_core::sherlock::helper_machine::shared::i128_class::KIntC;
use mori_core::sherlock::helper_machine::shared::owner_ext_admit::{
    ExtCapacitiesC, admit_ext_program,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_source_program::{
    ExtDomain, ExtProgram,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_t3_uses::{
    ExtExpr, ExtFunction, ExtValue,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::{
    CtorDeclSrc, CtorFieldSrc, ExtTypeBuildError, TypeDeclSrc, TypeExprSrc,
};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::*;

// ---------------------------------------------------------------- labels

fn violation_label(v: &HostViolation) -> String {
    format!(
        "kind={:?}, clause={:?}, rule={:?}, locator={}",
        v.kind,
        v.clause,
        v.rule,
        v.locator.as_deref().unwrap_or("<none>")
    )
}

fn capture_label(e: &CaptureStepError) -> String {
    match e {
        CaptureStepError::BeforeObservation(v) => {
            format!("before observation: {}", violation_label(v))
        }
        CaptureStepError::Step {
            violation, after, ..
        } => match after {
            Ok(_) => format!(
                "owner step: {}; after snapshot captured",
                violation_label(violation)
            ),
            Err(after_violation) => format!(
                "owner step: {}; after observation: {}",
                violation_label(violation),
                violation_label(after_violation)
            ),
        },
        CaptureStepError::AfterObservation {
            violation, event, ..
        } => format!(
            "after observation: {}; event_present={}",
            violation_label(violation),
            event.is_some()
        ),
    }
}

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
        } => format!(
            "ext admission rejected: reason={reason:?}, detail={detail}, locator={locator:?}"
        ),
        ExtTypeBuildError::ExtUnsupportedFeature { feature, locator } => {
            format!("ext unsupported feature: feature={feature:?}, locator={locator:?}")
        }
        ExtTypeBuildError::Host(v) => format!("ext host violation: {}", violation_label(v)),
    }
}

// ---------------------------------------------------------------- retained cfg

fn bounds(max_helper_work: u128) -> BoundsC {
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
        helper_tasks: 4,
        serialization_bytes: 1_000_000,
        max_reference_work: 1_000_000,
        max_helper_work,
        step_cap: 1_000_000,
    }
}

fn generous() -> BoundsC {
    bounds(100)
}

fn capacities() -> LayoutCapacitiesC {
    LayoutCapacitiesC {
        name_rows: 32,
        name_bytes: 512,
        occurrence_path_slots: 64,
        value_slots: 512,
    }
}

fn layout(b: BoundsC, ext: Option<ExtCapacitiesC>) -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("lv".into(), "1".into()),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds: b,
        capacities: capacities(),
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

// ---------------------------------------------------------------- factories
// Every factory constructs a fresh non-Clone carrier on each call, so a
// replay never reuses a moved Core/Ext program.

fn lit_u(n: u128) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::UInt {
            n: KIntC::InRange(n),
        },
    }
}
fn lit_b(b: bool) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::Bool { b },
    }
}
fn lit_list() -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::List {
            elements: vec![KIntC::InRange(2)],
        },
    }
}
fn lit_tree() -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::Tree {
            tree: CoreTree::Leaf {
                n: KIntC::InRange(3),
            },
        },
    }
}
fn bin(op: BinaryOp, left: CoreExpr, right: CoreExpr) -> CoreExpr {
    CoreExpr::Binary {
        op,
        left: Box::new(left),
        right: Box::new(right),
    }
}
fn core(entry: CoreExpr) -> CoreProgram {
    CoreProgram {
        functions: Vec::new(),
        entry,
    }
}
fn core_ret(sort: SortTag, body: CoreExpr) -> CoreProgram {
    CoreProgram {
        functions: vec![CoreFunction {
            name: "f".into(),
            parameters: Vec::new(),
            return_type: sort,
            body,
        }],
        entry: CoreExpr::Call {
            name: "f".into(),
            args: Vec::new(),
        },
    }
}

fn mk_add() -> CoreProgram {
    core(bin(BinaryOp::Add, lit_u(7), lit_u(5)))
}
fn mk_sub() -> CoreProgram {
    core(bin(BinaryOp::Sub, lit_u(9), lit_u(4)))
}
fn mk_mul_data_left() -> CoreProgram {
    // Left operand of the outer Mul is the Data result of the inner Add.
    core(bin(
        BinaryOp::Mul,
        bin(BinaryOp::Add, lit_u(1), lit_u(2)),
        lit_u(3),
    ))
}
fn mk_lt() -> CoreProgram {
    core(bin(BinaryOp::Lt, lit_u(3), lit_u(9)))
}
fn mk_cons() -> CoreProgram {
    core(bin(BinaryOp::Cons, lit_u(1), lit_list()))
}
fn mk_branch() -> CoreProgram {
    core(bin(BinaryOp::Branch, lit_tree(), lit_tree()))
}
fn mk_leaf() -> CoreProgram {
    core(CoreExpr::Unary {
        op: UnaryOp::Leaf,
        operand: Box::new(lit_u(4)),
    })
}
fn mk_not() -> CoreProgram {
    core(CoreExpr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(lit_b(true)),
    })
}
fn mk_if() -> CoreProgram {
    core(CoreExpr::If {
        cond: Box::new(lit_b(true)),
        then_expr: Box::new(lit_u(1)),
        else_expr: Box::new(lit_u(2)),
    })
}
fn mk_ret_uint() -> CoreProgram {
    core_ret(SortTag::UInt, lit_u(9))
}
fn mk_ret_bool() -> CoreProgram {
    core_ret(SortTag::Bool, lit_b(true))
}
fn mk_ret_list() -> CoreProgram {
    core_ret(SortTag::List, lit_list())
}
fn mk_ret_tree() -> CoreProgram {
    core_ret(SortTag::Tree, lit_tree())
}

fn alpha_decl() -> TypeDeclSrc {
    TypeDeclSrc {
        name: "Alpha".into(),
        parameters: Vec::new(),
        ctors: vec![CtorDeclSrc {
            name: "MkAlpha".into(),
            fields: Vec::new(),
        }],
    }
}
fn mk_nominal_ret() -> ExtProgram {
    ExtProgram {
        types: vec![alpha_decl()],
        assets: Vec::new(),
        states: Vec::new(),
        inputs: Vec::new(),
        functions: vec![ExtFunction {
            name: "f".into(),
            parameters: Vec::new(),
            return_type: TypeExprSrc::Named {
                name: "Alpha".into(),
            },
            body: ExtExpr::Literal {
                value: ExtValue::Constructor {
                    ty: TypeExprSrc::Named {
                        name: "Alpha".into(),
                    },
                    ctor: "MkAlpha".into(),
                    fields: Vec::new(),
                },
            },
        }],
        entry: ExtExpr::Call {
            name: "f".into(),
            args: Vec::new(),
        },
    }
}
fn mk_ctor_fields() -> ExtProgram {
    // Nonempty genuine unchecked ConstructorFieldsF: Box.MkBox has two
    // fields; the first field value is an Alpha constructor value, so
    // the exact expected field type at Entry is nominal Alpha (>= 4).
    ExtProgram {
        types: vec![
            alpha_decl(),
            TypeDeclSrc {
                name: "Box".into(),
                parameters: Vec::new(),
                ctors: vec![CtorDeclSrc {
                    name: "MkBox".into(),
                    fields: vec![
                        CtorFieldSrc {
                            name: "item".into(),
                            ty: TypeExprSrc::Named {
                                name: "Alpha".into(),
                            },
                        },
                        CtorFieldSrc {
                            name: "flag".into(),
                            ty: TypeExprSrc::Bool,
                        },
                    ],
                }],
            },
        ],
        assets: Vec::new(),
        states: Vec::new(),
        inputs: Vec::new(),
        functions: Vec::new(),
        entry: ExtExpr::Construct {
            ty: TypeExprSrc::Named { name: "Box".into() },
            ctor: "MkBox".into(),
            fields: vec![
                ExtExpr::Literal {
                    value: ExtValue::Constructor {
                        ty: TypeExprSrc::Named {
                            name: "Alpha".into(),
                        },
                        ctor: "MkAlpha".into(),
                        fields: Vec::new(),
                    },
                },
                ExtExpr::Literal {
                    value: ExtValue::Core {
                        value: CoreValue::Bool { b: true },
                    },
                },
            ],
        },
    }
}

#[derive(Clone, Copy)]
enum Make {
    Core(fn() -> CoreProgram),
    Ext(fn() -> ExtProgram),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Space {
    Const,
    Data,
}

#[derive(Clone, Copy)]
struct Case {
    name: &'static str,
    make: Make,
    site: SiteTag,
    mode: ScalarMode,
    expected: Option<TypeIdC>,
    a_space: Space,
    b_present: bool,
}

// ---------------------------------------------------------------- independent expectation

/// Independent expected Entry helper, derived in the test from the
/// frozen predicate table and the actual PRE snapshot. This is never
/// producer output used as its own expectation.
fn expected_helper(before: &ImageSnapshot) -> Result<(SiteTag, HelperControlC), ()> {
    let value = match before.control {
        ControlC::Return { value } => value,
        _ => return Err(()),
    };
    let top = before.frames.last().ok_or(())?;
    let (site, mode, operand_a, operand_b, expected) = match *top {
        FrameRowC::RightF { op, left, .. } => {
            let mode = match op {
                BinaryOp::Add => ScalarMode::Add,
                BinaryOp::Sub => ScalarMode::Sub,
                BinaryOp::Mul => ScalarMode::Mul,
                BinaryOp::Lt => ScalarMode::Lt,
                BinaryOp::Cons => ScalarMode::Cons,
                BinaryOp::Branch => ScalarMode::Branch,
                BinaryOp::Eq => return Err(()), // Equal, not ScalarEntry
            };
            (SiteTag::RetRight, mode, left, Some(value), None)
        }
        FrameRowC::UnaryF { op, .. } => {
            let mode = match op {
                UnaryOp::Leaf => ScalarMode::Leaf,
                UnaryOp::Not => ScalarMode::Not,
            };
            (SiteTag::RetUnary, mode, value, None, None)
        }
        FrameRowC::IfF { .. } => (SiteTag::RetIf, ScalarMode::TestBool, value, None, None),
        FrameRowC::ReturnF { return_type, .. } => (
            SiteTag::RetCall,
            ScalarMode::TestType,
            value,
            None,
            Some(return_type),
        ),
        FrameRowC::ConstructorFieldsF {
            field_type,
            checked: false,
            ..
        } => (
            SiteTag::RetCtorFields,
            ScalarMode::TestType,
            value,
            None,
            Some(field_type),
        ),
        _ => return Err(()),
    };
    Ok((
        site,
        HelperControlC::Scalar {
            common: HelperCommon {
                site,
                origin_reference_cursor: before.reference_cursor,
                parent_resume: None,
            },
            mode,
            operand_a,
            operand_b,
            expected,
            phase: ScalarPhase::First,
            reg_a: 0,
            staged: ScalarStaged::None,
        },
    ))
}

fn is_entry_pre(snapshot: &HostSnapshot) -> bool {
    let image = &snapshot.image;
    image.helper.is_none()
        && image.result.is_none()
        && image.tasks.is_empty()
        && image.outcome == OutcomeC::Running
        && image.remaining_work > 0
        && !image.frames.is_empty()
        && expected_helper(&snapshot.image).is_ok()
}

fn is_target_entry_pre(snapshot: &HostSnapshot, case: &Case) -> bool {
    if !is_entry_pre(snapshot) {
        return false;
    }
    let Ok((site, helper)) = expected_helper(&snapshot.image) else {
        return false;
    };
    let HelperControlC::Scalar {
        mode,
        operand_a,
        operand_b,
        expected,
        ..
    } = helper
    else {
        return false;
    };
    site == case.site
        && mode == case.mode
        && expected == case.expected
        && ref_space(operand_a) == case.a_space
        && operand_b.is_some() == case.b_present
}

fn ref_space(reference: RefC) -> Space {
    match reference {
        RefC::Const(_) => Space::Const,
        RefC::Data(_) => Space::Data,
    }
}

// ---------------------------------------------------------------- reach / replay

fn search_entry(
    admitted: Admitted,
    mut image: MachineImage,
    case: &Case,
) -> (Admitted, MachineImage, HostSnapshot) {
    let label = case.name;
    for _ in 0..512 {
        let before = capture(&admitted, &image).unwrap_or_else(|e| {
            panic!(
                "{label}: capture during Entry search: {}",
                violation_label(&e)
            )
        });
        if is_target_entry_pre(&before, case) {
            return (admitted, image, before);
        }
        match owner_record(&admitted, &mut image) {
            Ok(ObservedStep::Event(_)) => {}
            Ok(ObservedStep::Terminal { after, .. }) => panic!(
                "{label}: terminal before selected Entry pre-state: {:?}",
                after.image.outcome
            ),
            Err(e) => panic!("{label}: search step: {}", capture_label(&e)),
        }
    }
    panic!("{label}: bounded Entry search exhausted 512 steps without the selected pre-state")
}

fn admit_core(b: BoundsC, make: fn() -> CoreProgram, label: &str) -> (Admitted, MachineImage) {
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
    (admitted, image)
}

fn admit_ext(b: BoundsC, make: fn() -> ExtProgram, label: &str) -> (Admitted, MachineImage) {
    let (admitted, _report) = admit_ext_program(
        make(),
        ExtDomain::GENERIC_ABSENT,
        &b,
        &layout(b, Some(ExtCapacitiesC::all_zero())),
        None,
    )
    .unwrap_or_else(|e| panic!("{label}: admit_ext_program: {}", ext_label(&e)));
    let engine = host_owner_engine();
    let image = engine
        .admit_run(
            &admitted,
            &RunIntC::Nat(64),
            &RunIntC::Nat(8),
            &FinanceInput::Absent,
        )
        .unwrap_or_else(|e| panic!("{label}: ext genesis: {}", tooling_label(&e)));
    (admitted, image)
}

fn reach(b: BoundsC, case: &Case) -> (Admitted, MachineImage, HostSnapshot, C6HostConfig) {
    match case.make {
        Make::Core(make) => {
            let (admitted, image) = admit_core(b, make, case.name);
            let (admitted, image, pre) = search_entry(admitted, image, case);
            (admitted, image, pre, core_cfg(b))
        }
        Make::Ext(make) => {
            let (admitted, image) = admit_ext(b, make, case.name);
            let (admitted, image, pre) = search_entry(admitted, image, case);
            (admitted, image, pre, ext_cfg(b))
        }
    }
}

fn replay(b: BoundsC, case: &Case) -> CompleteStepRecord {
    let (admitted, mut image, _pre, _host_cfg) = reach(b, case);
    match owner_record(&admitted, &mut image) {
        Ok(ObservedStep::Event(record)) => record,
        Ok(ObservedStep::Terminal { .. }) => {
            panic!("{}: selected raw replay step must be an event", case.name)
        }
        Err(e) => panic!(
            "{}: selected raw replay step: {}",
            case.name,
            capture_label(&e)
        ),
    }
}

// ---------------------------------------------------------------- assertions / SDK

fn assert_applied(case: &Case, record: &CompleteStepRecord) {
    let label = case.name;
    let before = &record.before;
    let after = &record.after;
    let event = &record.event;
    let (site, want_helper) = expected_helper(&before.image)
        .unwrap_or_else(|_| panic!("{label}: replay before-state must derive an Entry helper"));
    assert_eq!(site, case.site, "{label}: independent site");
    match want_helper {
        HelperControlC::Scalar {
            mode,
            operand_a,
            operand_b,
            expected,
            ..
        } => {
            assert_eq!(mode, case.mode, "{label}: independent mode");
            assert_eq!(
                expected, case.expected,
                "{label}: independent expected TypeId"
            );
            assert_eq!(
                ref_space(operand_a),
                case.a_space,
                "{label}: operand A Ref space"
            );
            assert_eq!(
                operand_b.is_some(),
                case.b_present,
                "{label}: operand B presence"
            );
        }
        _ => panic!("{label}: independent helper must be Scalar"),
    }

    assert_eq!(event.rule_id, RuleIdC::BHelperScalarEntry, "{label}: rule");
    assert_eq!(
        event.event_kind,
        EventKind::HelperEnter,
        "{label}: event kind"
    );
    assert_eq!(
        event.helper_kind,
        Some(HelperKind::Scalar),
        "{label}: helper kind"
    );
    assert_eq!(
        event.disposition,
        StepDisposition::Applied,
        "{label}: disposition"
    );
    assert_eq!(event.site, Some(site), "{label}: event site");
    assert_eq!(
        event.origin_reference_cursor,
        Some(before.image.reference_cursor),
        "{label}: event origin cursor"
    );
    assert_eq!(event.work_delta, 1, "{label}: work delta");
    assert_eq!(event.helper_work_delta, 1, "{label}: helper work delta");
    assert_eq!(
        event.helper_visits_delta, 0,
        "{label}: Entry visits delta must be zero"
    );
    assert_eq!(
        event.reference_cursor_delta, 0,
        "{label}: reference cursor delta"
    );
    assert_eq!(
        event.alloc_deltas,
        AllocDeltasC {
            data_nodes: 0,
            data_fields: 0,
            environment_cells: 0
        },
        "{label}: alloc deltas"
    );
    assert_eq!(event.value_slot_writes, 0, "{label}: slot writes");
    assert_eq!(
        event.helper_phase_before, None,
        "{label}: helper phase before"
    );
    assert_eq!(
        event.helper_cursor_before, None,
        "{label}: helper cursor before"
    );
    assert_eq!(
        event.helper_phase_after,
        Some(HelperPhaseSummaryC::Scalar(ScalarPhase::First)),
        "{label}: helper phase after"
    );
    assert_eq!(
        event.helper_cursor_after,
        Some(0),
        "{label}: helper cursor after"
    );
    assert_eq!(event.result_tag, None, "{label}: result tag");

    assert_eq!(
        after.image.helper,
        Some(want_helper),
        "{label}: installed helper exact"
    );
    assert_eq!(
        after.image.remaining_work,
        before.image.remaining_work - 1,
        "{label}: remaining"
    );
    assert_eq!(
        after.image.consumed_work,
        before.image.consumed_work + 1,
        "{label}: consumed"
    );
    assert_eq!(
        after.image.helper_work,
        before.image.helper_work + 1,
        "{label}: helper work"
    );
    assert_eq!(
        after.image.helper_visits, before.image.helper_visits,
        "{label}: visits preserved"
    );
    assert_eq!(
        after.image.reference_cursor, before.image.reference_cursor,
        "{label}: reference cursor preserved"
    );
    // Full preservation outside the installed helper and charged counters.
    assert_eq!(
        after.admission, before.admission,
        "{label}: admission preserved"
    );
    assert_eq!(
        after.image.run_id, before.image.run_id,
        "{label}: run id preserved"
    );
    assert_eq!(
        after.image.control, before.image.control,
        "{label}: control preserved"
    );
    assert_eq!(
        after.image.frames, before.image.frames,
        "{label}: frames not popped"
    );
    assert_eq!(
        after.image.env_root, before.image.env_root,
        "{label}: env root preserved"
    );
    assert_eq!(after.image.env, before.image.env, "{label}: env preserved");
    assert_eq!(
        after.image.data, before.image.data,
        "{label}: DATA preserved"
    );
    assert_eq!(
        after.image.data_fields, before.image.data_fields,
        "{label}: DATA fields preserved"
    );
    assert_eq!(
        after.image.value_slots, before.image.value_slots,
        "{label}: slots preserved"
    );
    assert_eq!(
        after.image.tasks, before.image.tasks,
        "{label}: tasks preserved"
    );
    assert_eq!(after.image.result, None, "{label}: result stays absent");
    assert_eq!(
        after.image.alloc, before.image.alloc,
        "{label}: alloc preserved"
    );
    assert_eq!(
        after.image.active, before.image.active,
        "{label}: active preserved"
    );
    assert_eq!(
        after.image.outcome,
        OutcomeC::Running,
        "{label}: outcome Running"
    );
}

fn check_both_modes(witness: &C6ScalarEntryStepWitness, label: &str) {
    for (mode, scope) in [
        (
            C6PushStatementBindingMode::LocalPublic,
            "scalar_entry_local",
        ),
        (
            C6PushStatementBindingMode::LedgerCall,
            "scalar_entry_ledger",
        ),
    ] {
        let circuit = build_c6_scalar_entry_step_circuit(
            witness.source_params().clone(),
            witness.host_prefix().meta.len(),
            witness.host_prefix().prog.len(),
            witness.host_prefix().meta_bounds_frames_offset,
            mode,
            scope,
        )
        .unwrap_or_else(|e| panic!("{label}: build {mode:?}: {e}"));
        check_c6_scalar_entry_step_circuit(&circuit, witness)
            .unwrap_or_else(|e| panic!("{label}: SDK check {mode:?} must pass: {e}"));
    }
}

fn run_case(b: BoundsC, case: &Case) {
    let (admitted, mut image, pre, host_cfg) = reach(b, case);
    assert!(
        is_target_entry_pre(&pre, case),
        "{}: reached state must satisfy the case-target Entry preconditions",
        case.name
    );
    let witness = record_scalar_entry_step(&admitted, &mut image, &host_cfg, &limits())
        .unwrap_or_else(|e| panic!("{}: wrapper selected step: {e}", case.name));
    assert!(witness.enable(), "{}: wrapper witness enable", case.name);
    assert!(
        !witness.disposition_is_refused_helper_work(),
        "{}: generous case must be applied, not P3",
        case.name
    );
    let wrapper_after = capture(&admitted, &image).unwrap_or_else(|e| {
        panic!(
            "{}: capture wrapper after: {}",
            case.name,
            violation_label(&e)
        )
    });

    let record = replay(b, case);
    assert_eq!(
        &record.before, &pre,
        "{}: replay before equals original pre",
        case.name
    );
    assert_eq!(
        record.after, wrapper_after,
        "{}: original wrapper POST equals independent replay POST",
        case.name
    );
    assert_applied(case, &record);

    let from_record = witness_scalar_entry_from_owner_record(&record, &host_cfg, &limits())
        .unwrap_or_else(|e| panic!("{}: owner-record witness: {e}", case.name));
    let from_pair = witness_scalar_entry_from_host_pair(
        true,
        &record.before,
        &record.after,
        &host_cfg,
        &limits(),
    )
    .unwrap_or_else(|e| panic!("{}: host-pair witness: {e}", case.name));
    assert_eq!(
        from_record, from_pair,
        "{}: owner-record and host-pair witnesses agree",
        case.name
    );
    assert_eq!(
        from_record.to_statement_frs(),
        from_pair.to_statement_frs(),
        "{}: both routes encode the same captured real pair",
        case.name
    );
    assert_eq!(
        witness, from_pair,
        "{}: wrapper witness equals host-pair witness",
        case.name
    );
    check_both_modes(&from_pair, case.name);
}

// ---------------------------------------------------------------- tests: 7 #[test] fns

#[test]
fn all_eleven_shapes_both_modes() {
    let cases = [
        Case {
            name: "RetRight/Add",
            make: Make::Core(mk_add),
            site: SiteTag::RetRight,
            mode: ScalarMode::Add,
            expected: None,
            a_space: Space::Const,
            b_present: true,
        },
        Case {
            name: "RetRight/Sub",
            make: Make::Core(mk_sub),
            site: SiteTag::RetRight,
            mode: ScalarMode::Sub,
            expected: None,
            a_space: Space::Const,
            b_present: true,
        },
        Case {
            name: "RetRight/Mul/DataLeft",
            make: Make::Core(mk_mul_data_left),
            site: SiteTag::RetRight,
            mode: ScalarMode::Mul,
            expected: None,
            a_space: Space::Data,
            b_present: true,
        },
        Case {
            name: "RetRight/Lt",
            make: Make::Core(mk_lt),
            site: SiteTag::RetRight,
            mode: ScalarMode::Lt,
            expected: None,
            a_space: Space::Const,
            b_present: true,
        },
        Case {
            name: "RetRight/Cons",
            make: Make::Core(mk_cons),
            site: SiteTag::RetRight,
            mode: ScalarMode::Cons,
            expected: None,
            a_space: Space::Const,
            b_present: true,
        },
        Case {
            name: "RetRight/Branch",
            make: Make::Core(mk_branch),
            site: SiteTag::RetRight,
            mode: ScalarMode::Branch,
            expected: None,
            a_space: Space::Const,
            b_present: true,
        },
        Case {
            name: "RetUnary/Leaf",
            make: Make::Core(mk_leaf),
            site: SiteTag::RetUnary,
            mode: ScalarMode::Leaf,
            expected: None,
            a_space: Space::Const,
            b_present: false,
        },
        Case {
            name: "RetUnary/Not",
            make: Make::Core(mk_not),
            site: SiteTag::RetUnary,
            mode: ScalarMode::Not,
            expected: None,
            a_space: Space::Const,
            b_present: false,
        },
        Case {
            name: "RetIf/TestBool",
            make: Make::Core(mk_if),
            site: SiteTag::RetIf,
            mode: ScalarMode::TestBool,
            expected: None,
            a_space: Space::Const,
            b_present: false,
        },
        Case {
            name: "RetCall/TestType/UInt",
            make: Make::Core(mk_ret_uint),
            site: SiteTag::RetCall,
            mode: ScalarMode::TestType,
            expected: Some(TypeIdC(0)),
            a_space: Space::Const,
            b_present: false,
        },
        Case {
            name: "RetCtorFields/TestType/Alpha",
            make: Make::Ext(mk_ctor_fields),
            site: SiteTag::RetCtorFields,
            mode: ScalarMode::TestType,
            expected: Some(TypeIdC(4)),
            a_space: Space::Const,
            b_present: false,
        },
    ];
    assert_eq!(cases.len(), 11);
    for case in &cases {
        run_case(generous(), case);
    }
}

#[test]
fn builtin_returnf_expected_0_to_3_both_modes() {
    let cases = [
        Case {
            name: "RetCall/UInt0",
            make: Make::Core(mk_ret_uint),
            site: SiteTag::RetCall,
            mode: ScalarMode::TestType,
            expected: Some(TypeIdC(0)),
            a_space: Space::Const,
            b_present: false,
        },
        Case {
            name: "RetCall/Bool1",
            make: Make::Core(mk_ret_bool),
            site: SiteTag::RetCall,
            mode: ScalarMode::TestType,
            expected: Some(TypeIdC(1)),
            a_space: Space::Const,
            b_present: false,
        },
        Case {
            name: "RetCall/List2",
            make: Make::Core(mk_ret_list),
            site: SiteTag::RetCall,
            mode: ScalarMode::TestType,
            expected: Some(TypeIdC(2)),
            a_space: Space::Const,
            b_present: false,
        },
        Case {
            name: "RetCall/Tree3",
            make: Make::Core(mk_ret_tree),
            site: SiteTag::RetCall,
            mode: ScalarMode::TestType,
            expected: Some(TypeIdC(3)),
            a_space: Space::Const,
            b_present: false,
        },
    ];
    for case in &cases {
        run_case(generous(), case);
    }
}

#[test]
fn nominal_returnf_and_ctorfields_exact_both_modes() {
    let nominal_ret = Case {
        name: "RetCall/TestType/Alpha4",
        make: Make::Ext(mk_nominal_ret),
        site: SiteTag::RetCall,
        mode: ScalarMode::TestType,
        expected: Some(TypeIdC(4)),
        a_space: Space::Const,
        b_present: false,
    };
    run_case(generous(), &nominal_ret);

    let ctor_case = Case {
        name: "RetCtorFields/BoxItemAlpha4",
        make: Make::Ext(mk_ctor_fields),
        site: SiteTag::RetCtorFields,
        mode: ScalarMode::TestType,
        expected: Some(TypeIdC(4)),
        a_space: Space::Const,
        b_present: false,
    };
    let record = replay(generous(), &ctor_case);
    assert_applied(&ctor_case, &record);
    // The full nominal ConstructorFieldsF frame is preserved untouched.
    match record.before.image.frames.last() {
        Some(FrameRowC::ConstructorFieldsF {
            type_id,
            ctor_id,
            index,
            field_type,
            checked: false,
            ..
        }) => {
            assert_eq!(*type_id, TypeIdC(5), "Box is the second nominal type");
            assert_eq!(
                *ctor_id,
                CtorIdC(5),
                "MkBox follows MkAlpha in the ctor table"
            );
            assert_eq!(*index, 0, "first field is being evaluated at Entry");
            assert_eq!(
                *field_type,
                TypeIdC(4),
                "first field exact nominal type Alpha"
            );
        }
        other => {
            panic!("genuine pre-state top must be unchecked ConstructorFieldsF, got {other:?}")
        }
    }
    assert_eq!(
        record.after.image.frames, record.before.image.frames,
        "ConstructorFieldsF frame must not be popped or rewritten at Entry"
    );
    let host_cfg = ext_cfg(generous());
    let witness = witness_scalar_entry_from_host_pair(
        true,
        &record.before,
        &record.after,
        &host_cfg,
        &limits(),
    )
    .unwrap_or_else(|e| panic!("ctor-fields host-pair witness: {e}"));
    check_both_modes(&witness, ctor_case.name);
}

#[test]
fn exact_p3_at_original_separate_admission_bound() {
    let case = Case {
        name: "P3/RetRight/Add",
        make: Make::Core(mk_add),
        site: SiteTag::RetRight,
        mode: ScalarMode::Add,
        expected: None,
        a_space: Space::Const,
        b_present: true,
    };
    // Learn the genuinely reached pre helper_work under a generous
    // bound, then replay fresh with the separate admission bound set
    // exactly to it. Layout bounds are never substituted for this.
    let (_a, _i, generous_pre, _c) = reach(generous(), &case);
    let at_max = generous_pre.image.helper_work;
    assert!(
        at_max > 0,
        "genuine Entry pre must have positive helper work history"
    );
    let p3_bounds = bounds(at_max);
    let (_a2, _i2, p3_pre, _c2) = reach(p3_bounds, &case);
    assert_eq!(
        p3_pre.image.helper_work, at_max,
        "fresh P3 replay reaches the same pre"
    );
    assert!(p3_pre.image.helper.is_none(), "P3 pre helper absent");

    let record = replay(p3_bounds, &case);
    let event = &record.event;
    assert_eq!(event.rule_id, RuleIdC::BHelperScalarEntry);
    assert_eq!(event.event_kind, EventKind::HelperEnter);
    assert_eq!(event.helper_kind, Some(HelperKind::Scalar));
    assert_eq!(
        event.disposition,
        StepDisposition::RefusedHelperWorkUncharged
    );
    assert_eq!(event.exhausted_kind, Some(ExhaustReason::HelperWork));
    assert_eq!(
        event.outcome_payload,
        OutcomeC::Exhausted {
            reason: ExhaustReason::HelperWork
        }
    );
    assert_eq!(
        event.site,
        Some(SiteTag::RetRight),
        "P3 site via event fallback"
    );
    assert_eq!(
        event.origin_reference_cursor,
        Some(record.before.image.reference_cursor),
        "P3 origin cursor via pre fallback"
    );
    assert_eq!(event.helper_phase_before, None);
    assert_eq!(
        event.helper_phase_after, None,
        "P3 installs no helper phase"
    );
    assert_eq!(event.helper_cursor_before, None);
    assert_eq!(
        event.helper_cursor_after, None,
        "P3 installs no helper cursor"
    );
    assert_eq!(event.work_delta, 0);
    assert_eq!(event.helper_work_delta, 0);
    assert_eq!(event.helper_visits_delta, 0);
    assert_eq!(event.reference_cursor_delta, 0);
    assert_eq!(
        event.alloc_deltas,
        AllocDeltasC {
            data_nodes: 0,
            data_fields: 0,
            environment_cells: 0
        }
    );
    assert_eq!(
        record.after.image.helper, None,
        "P3 helper stays absent post"
    );
    assert_eq!(
        record.after.image.remaining_work,
        record.before.image.remaining_work
    );
    assert_eq!(
        record.after.image.consumed_work,
        record.before.image.consumed_work
    );
    assert_eq!(
        record.after.image.helper_work,
        record.before.image.helper_work
    );
    assert_eq!(
        record.after.image.outcome,
        OutcomeC::Exhausted {
            reason: ExhaustReason::HelperWork
        }
    );

    let (admitted, mut image, _pre, host_cfg) = reach(p3_bounds, &case);
    let _ = &admitted;
    let witness = record_scalar_entry_step(&admitted, &mut image, &host_cfg, &limits())
        .unwrap_or_else(|e| panic!("P3 wrapper witness: {e}"));
    assert!(witness.enable());
    assert!(
        witness.disposition_is_refused_helper_work(),
        "P3 disposition flag from PRE bound"
    );
    let from_pair = witness_scalar_entry_from_host_pair(
        true,
        &record.before,
        &record.after,
        &host_cfg,
        &limits(),
    )
    .unwrap_or_else(|e| panic!("P3 host-pair witness: {e}"));
    assert_eq!(witness, from_pair);
    check_both_modes(&from_pair, case.name);
}

#[test]
fn wrong_rule_and_terminal_wrapper_refusal_nonrollback() {
    let b = generous();
    let host_cfg = core_cfg(b);
    // Wrong rule: genesis control is Eval, not Return+selected frame.
    // The wrapper takes exactly that one owner step before refusing;
    // Err does NOT mean "no step happened" and no rollback occurs.
    let (admitted, mut image) = admit_core(b, mk_add, "wrong-rule");
    let before = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("wrong-rule before: {}", violation_label(&e)));
    let err = record_scalar_entry_step(&admitted, &mut image, &host_cfg, &limits())
        .err()
        .unwrap_or_else(|| panic!("wrong-rule wrapper must refuse"));
    assert!(
        matches!(err, C6ScalarEntryConsumerError::UnexpectedOwnerStep),
        "wrong-rule refusal category, got {err}"
    );
    let after = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("wrong-rule after: {}", violation_label(&e)));
    assert_ne!(
        before, after,
        "documented non-rollback: the one step persists"
    );
    assert_eq!(
        after.image.consumed_work,
        before.image.consumed_work + 1,
        "exactly one owner step was taken before refusal"
    );

    // Terminal: wrapper refusal with no step available; image unchanged.
    let (admitted2, mut image2, _pre, _c) = reach(
        b,
        &Case {
            name: "terminal-seed",
            make: Make::Core(mk_add),
            site: SiteTag::RetRight,
            mode: ScalarMode::Add,
            expected: None,
            a_space: Space::Const,
            b_present: true,
        },
    );
    for _ in 0..512 {
        match owner_record(&admitted2, &mut image2) {
            Ok(ObservedStep::Event(_)) => {}
            Ok(ObservedStep::Terminal { .. }) => break,
            Err(e) => panic!("terminal search step: {}", capture_label(&e)),
        }
    }
    let term_before = capture(&admitted2, &image2)
        .unwrap_or_else(|e| panic!("terminal before: {}", violation_label(&e)));
    assert_ne!(
        term_before.image.outcome,
        OutcomeC::Running,
        "fixture must be terminal"
    );
    let err2 = record_scalar_entry_step(&admitted2, &mut image2, &host_cfg, &limits())
        .err()
        .unwrap_or_else(|| panic!("terminal wrapper must refuse"));
    assert!(
        matches!(err2, C6ScalarEntryConsumerError::UnexpectedOwnerStep),
        "got {err2}"
    );
    let term_after = capture(&admitted2, &image2)
        .unwrap_or_else(|e| panic!("terminal after: {}", violation_label(&e)));
    assert_eq!(
        term_before, term_after,
        "terminal wrapper performs no mutation"
    );
}

#[test]
fn owner_record_mutations_and_crossed_record_rejected() {
    // Independent negative tests only: mutations of a captured actual
    // pair. Rejection here is internal-consistency only; it is never
    // described as authenticated provenance.
    let b = generous();
    let host_cfg = core_cfg(b);
    let add_case = Case {
        name: "mutate/Add",
        make: Make::Core(mk_add),
        site: SiteTag::RetRight,
        mode: ScalarMode::Add,
        expected: None,
        a_space: Space::Const,
        b_present: true,
    };
    let sub_case = Case {
        name: "mutate/Sub",
        make: Make::Core(mk_sub),
        site: SiteTag::RetRight,
        mode: ScalarMode::Sub,
        expected: None,
        a_space: Space::Const,
        b_present: true,
    };
    let record = replay(b, &add_case);
    witness_scalar_entry_from_owner_record(&record, &host_cfg, &limits())
        .unwrap_or_else(|e| panic!("unmutated owner record must validate: {e}"));

    let mutations: [(&str, fn(&mut CompleteStepRecord)); 6] = [
        ("rule", |r| r.event.rule_id = RuleIdC::BHelperScalarTag),
        ("site", |r| r.event.site = Some(SiteTag::RetUnary)),
        ("origin cursor", |r| {
            r.event.origin_reference_cursor = Some(999)
        }),
        ("installed mode", |r| {
            if let Some(HelperControlC::Scalar { mode, .. }) = &mut r.after.image.helper {
                *mode = ScalarMode::Sub;
            }
        }),
        ("helper phase after", |r| {
            r.event.helper_phase_after = Some(HelperPhaseSummaryC::Scalar(ScalarPhase::Done));
        }),
        ("disposition", |r| {
            r.event.disposition = StepDisposition::Published
        }),
    ];
    for (label, mutate) in mutations {
        let mut mutated = record.clone();
        mutate(&mut mutated);
        assert!(
            witness_scalar_entry_from_owner_record(&mutated, &host_cfg, &limits()).is_err(),
            "owner-record mutation category must reject: {label}"
        );
    }

    let sub_record = replay(b, &sub_case);
    assert_applied(&add_case, &record);
    assert_applied(&sub_case, &sub_record);
    match (record.after.image.helper, sub_record.after.image.helper) {
        (
            Some(HelperControlC::Scalar { mode: add_mode, .. }),
            Some(HelperControlC::Scalar { mode: sub_mode, .. }),
        ) => {
            assert_eq!(add_mode, ScalarMode::Add);
            assert_eq!(sub_mode, ScalarMode::Sub);
        }
        other => panic!("genuine crossed fixtures must install Scalar helpers, got {other:?}"),
    }
    assert_ne!(record.after, sub_record.after);
    let crossed = CompleteStepRecord {
        before: record.before.clone(),
        event: record.event.clone(),
        after: sub_record.after.clone(),
    };
    assert!(
        witness_scalar_entry_from_owner_record(&crossed, &host_cfg, &limits()).is_err(),
        "crossed before/after record must reject"
    );
}

#[test]
fn encodable_wrong_post_reaches_sdk_check_both_modes() {
    // Independent negative test only: one canonical wrong installed
    // mode mutated from a captured actual pair. The host-pair route
    // must encode it (no host semantic filter); the SDK checker in
    // each mode is the rejecting layer. The paired positive is checked
    // first in `all_eleven_shapes_both_modes`.
    let b = generous();
    let host_cfg = core_cfg(b);
    let case = Case {
        name: "wrongpost/Add",
        make: Make::Core(mk_add),
        site: SiteTag::RetRight,
        mode: ScalarMode::Add,
        expected: None,
        a_space: Space::Const,
        b_present: true,
    };
    let record = replay(b, &case);
    let mut wrong_after = record.after.clone();
    match &mut wrong_after.image.helper {
        Some(HelperControlC::Scalar { mode, .. }) => *mode = ScalarMode::Sub,
        other => panic!("genuine POST must install a Scalar helper, got {other:?}"),
    }
    let witness = witness_scalar_entry_from_host_pair(
        true,
        &record.before,
        &wrong_after,
        &host_cfg,
        &limits(),
    )
    .unwrap_or_else(|e| {
        panic!("encodable wrong POST must reach the checker, not a host filter: {e}")
    });
    for (mode, scope) in [
        (
            C6PushStatementBindingMode::LocalPublic,
            "scalar_entry_wrong_local",
        ),
        (
            C6PushStatementBindingMode::LedgerCall,
            "scalar_entry_wrong_ledger",
        ),
    ] {
        let circuit = build_c6_scalar_entry_step_circuit(
            witness.source_params().clone(),
            witness.host_prefix().meta.len(),
            witness.host_prefix().prog.len(),
            witness.host_prefix().meta_bounds_frames_offset,
            mode,
            scope,
        )
        .unwrap_or_else(|e| panic!("wrong-post build {mode:?}: {e}"));
        let err = check_c6_scalar_entry_step_circuit(&circuit, &witness)
            .err()
            .unwrap_or_else(|| panic!("wrong installed mode must fail SDK check in {mode:?}"));
        assert!(
            err.to_string().contains("SDK ProofPreimage::check failed"),
            "rejection must be the SDK preprocessing category in {mode:?}, got {err}"
        );
    }
}
