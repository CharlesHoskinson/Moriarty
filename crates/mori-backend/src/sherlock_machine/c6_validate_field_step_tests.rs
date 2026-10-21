//! Genuine-owner positive, refusal, precedence, and boundary tests for
//! C6 `BHelperValidateField`.
//!
//! UNCOMPILED/UNRUN. Every fixture is a fresh actual public Core or Ext
//! program admitted through the real public owner
//! (`host_owner_engine().admit` / `admit_run`, or `admit_ext_program`
//! for nominal `Constructor` literals) and stepped with real
//! `complete_observation::record_step` through `BHelperValidateEntry`
//! and `BHelperValidateOccurrence` (plus earlier `BHelperValidateField`
//! steps for `cursor > 0` and deeper task-stack cases) to the selected
//! `BHelperValidateField` pre-state:
//! * `pre.outcome == OutcomeC::Running`, `pre.remaining_work > 0`, and
//!   `owner_site(pre)` well-defined
//! * `pre.helper == Some(HelperControlC::Validate { common, root, status: TriStatus::Pending })`
//!   with `common.parent_resume == None` and `pre.result == None`
//! * `!pre.tasks.is_empty()` with
//!   `pre.tasks.last() == Some(HelperTaskC::ValidateTask { r, visit: TaskVisitC::Fields, cursor })`
//! * `pre.field_at(&r, &cursor)` well-defined and `SC-05`
//!   (`check_written_code_ids`) satisfied on the child reference
//! * `pre.checked_arity(&pre.kind_of(&r)?)` well-defined
//!
//! Coverage includes:
//! * Genuine `BHelperValidateField` steps across `Leaf` (`arity == 1`,
//!   tail `cursor == 0`), `Cons` (`arity == 2`, non-tail `cursor == 0`
//!   and tail `cursor == 1`), `Branch` (`arity == 2`, non-tail
//!   `cursor == 0` and tail `cursor == 1`), 1-field nominal
//!   `Constructor` (`Alpha::MkAlpha`, `arity == 1`, tail `cursor == 0`),
//!   and 2-field nominal `Constructor` (`Pair::MkPair`, `arity == 2`,
//!   non-tail `cursor == 0` and tail `cursor == 1`), verified in both
//!   `LocalPublic` and `LedgerCall` modes with `SCALAR_PREFIX_LANES == 5`.
//! * Non-tail (`cursor + 1 < checked_arity`) rewriting parent top task
//!   at `pre_helper[3] - 1` to `TaskVisitC::Fields, cursor + 1` and
//!   pushing `ValidateTask { r: child, visit: TaskVisitC::Unvisited, cursor: 0 }`
//!   at `pre_helper[3]` (`tasks.len() + 1`) vs tail
//!   (`cursor + 1 == checked_arity`) replacing top task in place at
//!   `pre_helper[3] - 1` with
//!   `ValidateTask { r: child, visit: TaskVisitC::Unvisited, cursor: 0 }`
//!   (`tasks.len()` unchanged).
//! * Nested same-typed distinct child order (`OuterPair` with two
//!   `Alpha` fields `Alpha(11)` and `Alpha(22)`) and deeper task stacks
//!   (`tasks.len() == 2` and `3` on nested `Branch`), verifying lower
//!   task slot preservation on both non-tail push and tail replacement
//!   in both modes.
//! * Composite literals containing a `Marker` child (`Leaf`, `Cons`,
//!   `Branch`, and `Constructor` with `KIntC::Negative` /
//!   `KIntC::AboveU128`): `BHelperValidateField` schedules the `Marker`
//!   child as `Unvisited, cursor = 0` (`Applied`, `status == Pending`),
//!   while the subsequent step on that `Marker` child selects
//!   `RuleIdC::BHelperValidateInvalid`.
//! * Non-zero expression origin and non-empty `frames`, `env`, `data`,
//!   `data_fields`, `value_slots`, and `reference_cursor` preservation.
//! * P6 (`StepDisposition::RefusedCapCharged` /
//!   `ExhaustReason::HelperTasks`) on non-tail when
//!   `pre.tasks.len() + 1 > admission.bounds.helper_tasks` (charging P4
//!   `remaining_work - 1`, `consumed_work + 1`, `helper_work + 1`,
//!   leaving `helper_visits` unchanged, and leaving all tasks unchanged
//!   with no parent rewrite or child push), tail-at-bound
//!   (`pre.tasks.len() == admission.bounds.helper_tasks` with
//!   `!is_nontail`) succeeding as `StepDisposition::Applied`, P3
//!   (`StepDisposition::RefusedHelperWorkUncharged` /
//!   `ExhaustReason::HelperWork`) uncharged refusal, and P3 dominating
//!   would-be P6 when both `pre.helper_work == max_helper_work` and
//!   `is_nontail && pre.tasks.len() + 1 > helper_tasks` hold, all tested
//!   against both matching layout bounds and larger static layout
//!   capacities in both modes.
//! * P2 fuel-zero refusal (`RuleIdC::KAdminPublishExhausted`),
//!   non-rollback wrong-rule wrapper refusals (`BHelperValidateEntry`,
//!   `BHelperValidateOccurrence`, `BHelperValidateInvalid`,
//!   `BHelperValidateEmpty`), independent host-pair canonical wrong
//!   post reaching `ProofPreimage::check` in both modes, and
//!   `witness_validate_field_from_owner_record` internal consistency
//!   and PS-before-P3/P6 rejections.

use super::c6_push_step::C6PushStatementBindingMode;
use super::{
    build_c6_validate_field_step_circuit, check_c6_validate_field_step_circuit,
    record_validate_field_step, witness_validate_field_from_host_pair,
    witness_validate_field_from_owner_record, C6ValidateFieldConsumerError,
    C6ValidateFieldStepWitness,
};
use mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets;
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_TASKS, TASK_LANES};
use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostConfig, C6HostLimits, CTX_LANES};
use mori_core::sherlock::helper_machine::complete_observation::{
    capture, record_step as owner_record, CaptureStepError, CompleteStepRecord, ConstantSnapshot,
    HostSnapshot, ObservedStep,
};
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

const SCALAR_PREFIX_LANES: usize = 5;

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
            Err(av) => format!(
                "owner step: {}; after observation: {}",
                violation_label(violation),
                violation_label(av)
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

fn generous() -> BoundsC {
    bounds_with(100, 4)
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
        Some(ConstantSnapshot::Nil { marker_free }) => {
            (ConstKindWant::Nil, *marker_free, 0, 0, 0)
        }
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
            type_id,
            ctor_id,
            fields,
            marker_free,
        }) => {
            assert!(type_id.0 >= 4, "nominal constructor type_id >= 4");
            let trow = &view.types[type_id.0];
            let crow = &view.constructors[ctor_id.0];
            assert_eq!(crow.owning_type, *type_id, "nominal owning_type");
            assert!(
                ctor_id.0 >= trow.ctor_start.0 && ctor_id.0 < trow.ctor_start.0 + trow.ctor_len,
                "nominal ctor range"
            );
            assert!(
                crow.field_start + crow.field_len <= view.ctor_fields.len(),
                "nominal field range"
            );
            (
                ConstKindWant::Constructor {
                    arity: crow.field_len,
                },
                *marker_free,
                crow.field_len,
                fields.start,
                fields.len,
            )
        }
        Some(ConstantSnapshot::U128OutOfRange { marker_free }) => {
            (ConstKindWant::Marker, *marker_free, 0, 0, 0)
        }
        None => panic!("constant id {id} must exist in admission view"),
    }
}

fn selected_field_top(s: &HostSnapshot, w: &FieldSpec) -> Option<(usize, usize, usize, usize)> {
    if s.image.outcome != OutcomeC::Running
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
    let (actual_kind, actual_mf, checked_arity, slice_start, slice_len) = const_info(s, *const_id);
    if actual_kind != w.parent_kind
        || actual_mf != w.parent_marker_free
        || actual_kind == ConstKindWant::Marker
        || *cursor >= slice_len
    {
        return None;
    }
    let child_const_id = s.admission.view.constant_fields[slice_start + *cursor];
    Some((*const_id, *cursor, checked_arity, child_const_id))
}

fn search(
    admitted: Admitted,
    mut image: MachineImage,
    w: &FieldSpec,
    label: &str,
) -> (Admitted, MachineImage, HostSnapshot) {
    let initial = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("{label}: capture initial PRE: {}", violation_label(&e)));
    if selected_field_top(&initial, w).is_some() {
        return (admitted, image, initial);
    }
    for _ in 0..512 {
        let step = owner_record(&admitted, &mut image)
            .unwrap_or_else(|e| panic!("{label}: search: {}", capture_label(&e)));
        let after = match &step {
            ObservedStep::Event(r) => &r.after,
            ObservedStep::Terminal { after, .. } => {
                panic!(
                    "{label}: terminal before selected ValidateField PRE: {:?}",
                    after.image.outcome
                )
            }
        };
        if selected_field_top(after, w).is_some() {
            let sel = capture(&admitted, &image)
                .unwrap_or_else(|e| panic!("{label}: capture PRE: {}", violation_label(&e)));
            assert_eq!(&sel, after, "{label}: capture equals searched PRE");
            return (admitted, image, sel);
        }
    }
    panic!("{label}: genuine owner never reached selected ValidateField PRE")
}

fn reach_core_fuel(
    b: BoundsC,
    fuel: u128,
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
            &RunIntC::Nat(fuel),
            &RunIntC::Nat(8),
            &FinanceInput::Absent,
        )
        .unwrap_or_else(|e| panic!("{label}: genesis: {}", tooling_label(&e)));
    search(admitted, image, w, label)
}

fn reach_core(
    b: BoundsC,
    w: &FieldSpec,
    label: &str,
    make: &dyn Fn() -> CoreProgram,
) -> (Admitted, MachineImage, HostSnapshot) {
    reach_core_fuel(b, 64, w, label, make)
}

fn reach_ext(
    b: BoundsC,
    w: &FieldSpec,
    label: &str,
    make: &dyn Fn() -> ExtProgram,
) -> (Admitted, MachineImage, HostSnapshot) {
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
    search(admitted, image, w, label)
}

fn assert_preserved(r: &CompleteStepRecord, label: &str) {
    let (pre, post) = (&r.before.image, &r.after.image);
    assert_eq!(r.after.admission, r.before.admission, "{label}: admission");
    assert_eq!(post.run_id, pre.run_id, "{label}: run_id");
    assert_eq!(post.control, pre.control, "{label}: control");
    assert_eq!(post.control_origin, pre.control_origin, "{label}: origin");
    assert_eq!(post.env_root, pre.env_root, "{label}: env root");
    assert_eq!(post.stack_limit, pre.stack_limit, "{label}: stack");
    assert_eq!(post.initial_fuel, pre.initial_fuel, "{label}: fuel");
    assert_eq!(post.seg_base, pre.seg_base, "{label}: seg_base");
    assert_eq!(post.frames, pre.frames, "{label}: frames");
    assert_eq!(post.env, pre.env, "{label}: env");
    assert_eq!(post.value_slots, pre.value_slots, "{label}: slots");
    assert_eq!(post.data, pre.data, "{label}: DATA");
    assert_eq!(post.data_fields, pre.data_fields, "{label}: DATA fields");
    assert_eq!(post.alloc, pre.alloc, "{label}: alloc");
    assert_eq!(post.active, pre.active, "{label}: active");
    assert_eq!(
        post.reference_cursor, pre.reference_cursor,
        "{label}: ref cursor"
    );
    assert_eq!(
        post.helper, pre.helper,
        "{label}: helper control record preserved unchanged"
    );
    assert!(
        pre.result.is_none() && post.result.is_none(),
        "{label}: result absent"
    );
}

fn assert_common_event(r: &CompleteStepRecord, want_after_tasks_len: usize, label: &str) {
    let (pre, post, e) = (&r.before.image, &r.after.image, &r.event);
    assert_eq!(e.rule_id, RuleIdC::BHelperValidateField, "{label}: rule");
    assert_eq!(e.event_kind, EventKind::HelperVisit, "{label}: event kind");
    assert_eq!(
        e.helper_kind,
        Some(HelperKind::Validate),
        "{label}: helper kind"
    );
    assert_eq!(e.site, Some(SiteTag::EvalLiteral), "{label}: site");
    assert_eq!(e.reference_cursor_delta, 0, "{label}: cursor delta 0");
    assert_eq!(
        e.alloc_deltas,
        AllocDeltasC {
            data_nodes: 0,
            data_fields: 0,
            environment_cells: 0,
        },
        "{label}: no alloc"
    );
    assert_eq!(e.value_slot_writes, 0, "{label}: slot writes");
    assert!(e.result_tag.is_none(), "{label}: no result tag");
    assert_eq!(e.step_id.ordinal, pre.consumed_work, "{label}: ordinal");
    assert_eq!(*e.step_id.run_id, pre.run_id, "{label}: step run id");
    let Some(HelperControlC::Validate {
        common,
        status: TriStatus::Pending,
        ..
    }) = &pre.helper
    else {
        panic!("{label}: expected active Validate Pending");
    };
    assert_eq!(
        e.origin_reference_cursor,
        Some(common.origin_reference_cursor),
        "{label}: origin cursor"
    );
    assert_eq!(e.source_expr_id, pre.control_origin, "{label}: source expr");
    {
        let occ = &r.before.admission.view.occurrences[pre.control_origin];
        let path =
            r.before.admission.view.occurrence_paths[occ.path.start..occ.path.start + occ.path.len]
                .to_vec();
        assert_eq!(
            e.source_occurrence,
            LocatorC::Occurrence {
                owner_is_function: occ.owner_is_function,
                function: occ.function,
                ast_path: path,
            },
            "{label}: source occurrence"
        );
    }
    assert_eq!(
        e.logical_depth_before,
        pre.frames.len(),
        "{label}: depth before"
    );
    assert_eq!(
        e.logical_depth_after,
        post.frames.len(),
        "{label}: depth after"
    );
    assert_eq!(
        e.task_cursor_before,
        pre.tasks.len(),
        "{label}: task before"
    );
    assert_eq!(
        e.task_cursor_after,
        post.tasks.len(),
        "{label}: task after"
    );
    assert_eq!(
        e.helper_phase_before,
        Some(HelperPhaseSummaryC::Tri(TriStatus::Pending)),
        "{label}: phase before"
    );
    assert_eq!(
        e.helper_cursor_before,
        Some(pre.tasks.len()),
        "{label}: cursor before"
    );
    assert_eq!(
        e.helper_phase_after,
        Some(HelperPhaseSummaryC::Tri(TriStatus::Pending)),
        "{label}: phase after"
    );
    assert_eq!(
        e.helper_cursor_after,
        Some(want_after_tasks_len),
        "{label}: cursor after"
    );
}

fn assert_applied(
    r: &CompleteStepRecord,
    witness: &C6ValidateFieldStepWitness,
    w: &FieldSpec,
    label: &str,
) {
    let (parent_const_id, cursor, checked_arity, child_const_id) =
        selected_field_top(&r.before, w)
            .unwrap_or_else(|| panic!("{label}: selected PRE must match FieldSpec"));
    let is_nontail = cursor + 1 < checked_arity;
    let (pre, post, e) = (&r.before.image, &r.after.image, &r.event);
    let prefix_len = pre.tasks.len() - 1;
    let want_after_len = if is_nontail {
        pre.tasks.len() + 1
    } else {
        pre.tasks.len()
    };
    assert_preserved(r, label);
    assert_common_event(r, want_after_len, label);
    assert_eq!(
        e.disposition,
        StepDisposition::Applied,
        "{label}: disposition"
    );
    assert_eq!(
        (e.work_delta, e.helper_work_delta, e.helper_visits_delta),
        (1, 1, 1),
        "{label}: applied deltas (work=1, helper_work=1, helper_visits=1)"
    );
    assert!(e.exhausted_kind.is_none(), "{label}: no exhausted kind");
    assert_eq!(e.outcome_tag, OutcomeTagC::Running, "{label}: outcome tag");
    assert_eq!(e.outcome_payload, OutcomeC::Running, "{label}: outcome");
    assert_eq!(e.boundary, BoundaryKind::None, "{label}: boundary");
    assert_eq!(
        post.remaining_work,
        pre.remaining_work - 1,
        "{label}: remaining-1"
    );
    assert_eq!(
        post.consumed_work,
        pre.consumed_work + 1,
        "{label}: consumed+1"
    );
    assert_eq!(
        post.helper_work,
        pre.helper_work + 1,
        "{label}: helper_work+1"
    );
    assert_eq!(
        post.helper_visits,
        pre.helper_visits + 1,
        "{label}: helper_visits+1"
    );
    assert_eq!(post.outcome, OutcomeC::Running, "{label}: post outcome");

    let parent_ref = RefC::Const(parent_const_id);
    let child_ref = RefC::Const(child_const_id);
    let top_slot_base = L_TASKS + prefix_len * TASK_LANES;
    assert_eq!(
        &post.tasks[..prefix_len],
        &pre.tasks[..prefix_len],
        "{label}: lower tasks 0..{prefix_len} preserved in HostSnapshot"
    );
    assert_eq!(
        &witness.post_helper()[L_TASKS..top_slot_base],
        &witness.pre_helper()[L_TASKS..top_slot_base],
        "{label}: lower task carrier lanes 0..{prefix_len} preserved"
    );
    if is_nontail {
        assert_eq!(
            post.tasks.len(),
            pre.tasks.len() + 1,
            "{label}: non-tail increments tasks.len() by 1"
        );
        assert_eq!(
            post.tasks[prefix_len],
            HelperTaskC::ValidateTask {
                r: parent_ref,
                visit: TaskVisitC::Fields,
                cursor: cursor + 1,
            },
            "{label}: non-tail rewrites parent slot {prefix_len} to Fields, cursor+1"
        );
        assert_eq!(
            post.tasks.last(),
            Some(&HelperTaskC::ValidateTask {
                r: child_ref,
                visit: TaskVisitC::Unvisited,
                cursor: 0,
            }),
            "{label}: non-tail pushes Unvisited child task at top slot"
        );
        assert_eq!(
            witness.post_helper()[3],
            (pre.tasks.len() + 1) as u128,
            "{label}: post_helper[3] incremented on non-tail"
        );
        assert_eq!(
            &witness.post_helper()[top_slot_base..top_slot_base + TASK_LANES],
            &[
                0u128,
                0,
                parent_const_id as u128,
                1,
                (cursor + 1) as u128,
                0,
                0
            ],
            "{label}: post_helper parent slot rewritten to [0, 0, parent, 1, cursor+1, 0, 0]"
        );
        let push_slot_base = top_slot_base + TASK_LANES;
        assert_eq!(
            &witness.post_helper()[push_slot_base..push_slot_base + TASK_LANES],
            &[0u128, 0, child_const_id as u128, 0, 0, 0, 0],
            "{label}: post_helper pushed child slot equals [0, 0, child, 0, 0, 0, 0]"
        );
    } else {
        assert_eq!(
            post.tasks.len(),
            pre.tasks.len(),
            "{label}: tail replacement preserves tasks.len()"
        );
        assert_eq!(
            post.tasks.last(),
            Some(&HelperTaskC::ValidateTask {
                r: child_ref,
                visit: TaskVisitC::Unvisited,
                cursor: 0,
            }),
            "{label}: tail replaces top task in place with Unvisited child, cursor=0"
        );
        assert_eq!(
            witness.post_helper()[3],
            pre.tasks.len() as u128,
            "{label}: post_helper[3] unchanged on tail replacement"
        );
        assert_eq!(
            &witness.post_helper()[top_slot_base..top_slot_base + TASK_LANES],
            &[0u128, 0, child_const_id as u128, 0, 0, 0, 0],
            "{label}: post_helper top slot replaced in place with [0, 0, child, 0, 0, 0, 0]"
        );
    }
}

fn assert_both_modes(witness: &C6ValidateFieldStepWitness, label: &str) {
    let prefix_lanes =
        witness.host_prefix().meta.len() + witness.host_prefix().prog.len() + CTX_LANES;
    let state_lanes =
        witness.pre_image().len() + witness.pre_data().len() + witness.pre_helper().len();
    assert_eq!(
        witness.to_statement_frs().len(),
        SCALAR_PREFIX_LANES + 2 * (prefix_lanes + state_lanes),
        "{label}: statement has exact 5 scalar prefix lanes (enable, p3, p6, host_schema, carrier_version)"
    );
    for mode in [
        C6PushStatementBindingMode::LocalPublic,
        C6PushStatementBindingMode::LedgerCall,
    ] {
        let circuit = build_c6_validate_field_step_circuit(
            witness.source_params().clone(),
            witness.host_prefix().meta.len(),
            witness.host_prefix().prog.len(),
            witness.host_prefix().meta_bounds_frames_offset,
            mode,
            "validate_field_genuine",
        )
        .unwrap_or_else(|e| panic!("{label}: build {mode:?}: {e}"));
        check_c6_validate_field_step_circuit(&circuit, witness)
            .unwrap_or_else(|e| panic!("{label}: check {mode:?}: {e}"));
    }
}

struct Applied {
    record: CompleteStepRecord,
    witness: C6ValidateFieldStepWitness,
}

fn finish_applied(
    admitted: Admitted,
    mut image: MachineImage,
    pre: HostSnapshot,
    replay: CompleteStepRecord,
    config: &C6HostConfig,
    w: &FieldSpec,
    label: &str,
) -> Applied {
    assert_eq!(replay.before, pre, "{label}: replay PRE equals original PRE");
    let witness = record_validate_field_step(&admitted, &mut image, config, &limits())
        .unwrap_or_else(|e| panic!("{label}: wrapper selected step once: {e}"));
    let original_post = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("{label}: capture original POST: {}", violation_label(&e)));
    assert_eq!(
        original_post, replay.after,
        "{label}: original POST == independently replayed POST"
    );
    assert_applied(&replay, &witness, w, label);
    let rec_wit = witness_validate_field_from_owner_record(&replay, config, &limits())
        .unwrap_or_else(|e| panic!("{label}: owner-record witness: {e}"));
    let hp_wit = witness_validate_field_from_host_pair(
        true,
        &replay.before,
        &replay.after,
        config,
        &limits(),
    )
    .unwrap_or_else(|e| panic!("{label}: host-pair witness: {e}"));
    assert_eq!(
        witness.to_statement_frs(),
        rec_wit.to_statement_frs(),
        "{label}: wrapper == record witness"
    );
    assert_eq!(
        witness.to_statement_frs(),
        hp_wit.to_statement_frs(),
        "{label}: wrapper == host-pair witness"
    );
    assert!(
        witness.enable()
            && !witness.disposition_is_refused_helper_work()
            && !witness.disposition_is_refused_helper_tasks(),
        "{label}: applied flags (enable=true, p3=false, p6=false)"
    );
    assert_both_modes(&witness, label);
    Applied {
        record: replay,
        witness,
    }
}

fn run_core(w: &FieldSpec, label: &str, make: &dyn Fn() -> CoreProgram) -> Applied {
    let b = generous();
    let (admitted, image, pre) = reach_core(b, w, label, make);
    let (radm, mut rimg, _) = reach_core(b, w, label, make);
    let replay = match owner_record(&radm, &mut rimg) {
        Ok(ObservedStep::Event(r)) => r,
        Ok(ObservedStep::Terminal { .. }) => panic!("{label}: replay selected step terminal"),
        Err(e) => panic!("{label}: replay selected step: {}", capture_label(&e)),
    };
    finish_applied(admitted, image, pre, replay, &core_cfg(b), w, label)
}

fn run_ext(w: &FieldSpec, label: &str, make: &dyn Fn() -> ExtProgram) -> Applied {
    let b = generous();
    let (admitted, image, pre) = reach_ext(b, w, label, make);
    let (radm, mut rimg, _) = reach_ext(b, w, label, make);
    let replay = match owner_record(&radm, &mut rimg) {
        Ok(ObservedStep::Event(r)) => r,
        Ok(ObservedStep::Terminal { .. }) => panic!("{label}: replay selected ext step terminal"),
        Err(e) => panic!("{label}: replay selected ext step: {}", capture_label(&e)),
    };
    finish_applied(admitted, image, pre, replay, &ext_cfg(b), w, label)
}

fn lit_u(n: u128) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::UInt {
            n: i128_class::KIntC::InRange(n),
        },
    }
}
fn lit_list(v: &[u128]) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::List {
            elements: v.iter().map(|n| i128_class::KIntC::InRange(*n)).collect(),
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
        TypeDeclSrc {
            name: "OuterPair".into(),
            parameters: Vec::new(),
            ctors: vec![CtorDeclSrc {
                name: "MkOuter".into(),
                fields: vec![
                    CtorFieldSrc {
                        name: "left_box".into(),
                        ty: TypeExprSrc::Named {
                            name: "Alpha".into(),
                        },
                    },
                    CtorFieldSrc {
                        name: "right_box".into(),
                        ty: TypeExprSrc::Named {
                            name: "Alpha".into(),
                        },
                    },
                ],
            }],
        },
    ]
}

fn ext_prog(entry: ExtExpr) -> ExtProgram {
    ExtProgram {
        types: nominal_types(),
        assets: Vec::new(),
        states: Vec::new(),
        inputs: Vec::new(),
        functions: Vec::new(),
        entry,
    }
}

fn ext_ctor_one_field(val: i128_class::KIntC) -> ExtExpr {
    ExtExpr::Literal {
        value: ExtValue::Constructor {
            ty: TypeExprSrc::Named {
                name: "Alpha".into(),
            },
            ctor: "MkAlpha".into(),
            fields: vec![(
                "payload".into(),
                ExtValue::Core {
                    value: CoreValue::UInt { n: val },
                },
            )],
        },
    }
}

fn ext_ctor_two_fields(first: u128, second: u128) -> ExtExpr {
    ExtExpr::Literal {
        value: ExtValue::Constructor {
            ty: TypeExprSrc::Named {
                name: "Pair".into(),
            },
            ctor: "MkPair".into(),
            fields: vec![
                (
                    "first".into(),
                    ExtValue::Core {
                        value: CoreValue::UInt {
                            n: i128_class::KIntC::InRange(first),
                        },
                    },
                ),
                (
                    "second".into(),
                    ExtValue::Core {
                        value: CoreValue::UInt {
                            n: i128_class::KIntC::InRange(second),
                        },
                    },
                ),
            ],
        },
    }
}

fn ext_nested_outer_pair(left_val: u128, right_val: u128) -> ExtExpr {
    ExtExpr::Literal {
        value: ExtValue::Constructor {
            ty: TypeExprSrc::Named {
                name: "OuterPair".into(),
            },
            ctor: "MkOuter".into(),
            fields: vec![
                (
                    "left_box".into(),
                    ExtValue::Constructor {
                        ty: TypeExprSrc::Named {
                            name: "Alpha".into(),
                        },
                        ctor: "MkAlpha".into(),
                        fields: vec![(
                            "payload".into(),
                            ExtValue::Core {
                                value: CoreValue::UInt {
                                    n: i128_class::KIntC::InRange(left_val),
                                },
                            },
                        )],
                    },
                ),
                (
                    "right_box".into(),
                    ExtValue::Constructor {
                        ty: TypeExprSrc::Named {
                            name: "Alpha".into(),
                        },
                        ctor: "MkAlpha".into(),
                        fields: vec![(
                            "payload".into(),
                            ExtValue::Core {
                                value: CoreValue::UInt {
                                    n: i128_class::KIntC::InRange(right_val),
                                },
                            },
                        )],
                    },
                ),
            ],
        },
    }
}

#[test]
fn genuine_validate_field_leaf_cons_branch_and_nominal_arity1_arity2_tail_and_nontail_both_modes() {
    // 1. Leaf (`checked_arity == 1`, `cursor == 0` -> tail replacement `1 -> 1`).
    run_core(
        &field_spec(ConstKindWant::Leaf, true, 0),
        "ValidateField Leaf cursor=0 (tail)",
        &|| prog(lit_tree(tree_leaf(42))),
    );

    // 2. Cons (`checked_arity == 2`, `cursor == 0` -> non-tail `1 -> 2`; `cursor == 1` -> tail `1 -> 1`).
    run_core(
        &field_spec(ConstKindWant::Cons, true, 0),
        "ValidateField Cons cursor=0 (non-tail)",
        &|| prog(lit_list(&[7])),
    );
    run_core(
        &field_spec(ConstKindWant::Cons, true, 1),
        "ValidateField Cons cursor=1 (tail)",
        &|| prog(lit_list(&[7])),
    );

    // 3. Branch (`checked_arity == 2`, `cursor == 0` -> non-tail `1 -> 2`; `cursor == 1` -> tail `1 -> 1`).
    run_core(
        &field_spec(ConstKindWant::Branch, true, 0),
        "ValidateField Branch cursor=0 (non-tail)",
        &|| prog(lit_branch()),
    );
    run_core(
        &field_spec(ConstKindWant::Branch, true, 1),
        "ValidateField Branch cursor=1 (tail)",
        &|| prog(lit_branch()),
    );

    // 4. 1-field nominal Constructor (`Alpha::MkAlpha(19)`, `checked_arity == 1`, `cursor == 0` -> tail `1 -> 1`).
    run_ext(
        &field_spec(ConstKindWant::Constructor { arity: 1 }, true, 0),
        "ValidateField 1-field Constructor MkAlpha cursor=0 (tail)",
        &|| ext_prog(ext_ctor_one_field(i128_class::KIntC::InRange(19))),
    );

    // 5. 2-field nominal Constructor (`Pair::MkPair(11, 22)`, `checked_arity == 2`):
    //    - `cursor == 0` (non-tail `1 -> 2`, pushing first child `UInt(11)`)
    //    - `cursor == 1` (tail `1 -> 1`, replacing top with second child `UInt(22)`)
    let pair_c0 = run_ext(
        &field_spec(ConstKindWant::Constructor { arity: 2 }, true, 0),
        "ValidateField 2-field Constructor MkPair cursor=0 (non-tail)",
        &|| ext_prog(ext_ctor_two_fields(11, 22)),
    );
    let child_c0 = match pair_c0.record.after.image.tasks.last() {
        Some(HelperTaskC::ValidateTask {
            r: RefC::Const(cid),
            visit: TaskVisitC::Unvisited,
            cursor: 0,
        }) => const_info(&pair_c0.record.after, *cid).0,
        _ => panic!("expected Unvisited Const child at cursor=0"),
    };
    assert_eq!(child_c0, ConstKindWant::UInt(11), "MkPair cursor=0 pushes first field UInt(11)");

    let pair_c1 = run_ext(
        &field_spec(ConstKindWant::Constructor { arity: 2 }, true, 1),
        "ValidateField 2-field Constructor MkPair cursor=1 (tail)",
        &|| ext_prog(ext_ctor_two_fields(11, 22)),
    );
    let child_c1 = match pair_c1.record.after.image.tasks.last() {
        Some(HelperTaskC::ValidateTask {
            r: RefC::Const(cid),
            visit: TaskVisitC::Unvisited,
            cursor: 0,
        }) => const_info(&pair_c1.record.after, *cid).0,
        _ => panic!("expected Unvisited Const child at cursor=1"),
    };
    assert_eq!(child_c1, ConstKindWant::UInt(22), "MkPair cursor=1 replaces top with second field UInt(22)");
}

#[test]
fn nested_same_typed_distinct_child_order_and_lower_task_stack_preservation_both_modes() {
    let b = generous();

    // 1. Nested nominal `OuterPair { left_box: Alpha { payload: 11 }, right_box: Alpha { payload: 22 } }`:
    //    Both fields of `OuterPair` have the same nominal type `Alpha`, and their inner fields both
    //    have type `UInt`. Stepping every `BHelperValidateField` transition verifies:
    //    - `OuterPair(cursor=0)` (non-tail `1 -> 2`) schedules `left_box` (`Alpha(11)`) first,
    //    - `left_box(cursor=0)` at `tasks.len() == 2` (tail `2 -> 2`) replaces slot 1 with `UInt(11)`
    //      while preserving lower slot 0 (`OuterPair(cursor=1)`),
    //    - `OuterPair(cursor=1)` (tail `1 -> 1`) schedules `right_box` (`Alpha(22)`) second,
    //    - `right_box(cursor=0)` (tail `1 -> 1`) replaces slot 0 with `UInt(22)`.
    {
        let (admitted, _) = admit_ext_program(
            ext_prog(ext_nested_outer_pair(11, 22)),
            ExtDomain::GENERIC_ABSENT,
            &b,
            &layout(b, Some(ExtCapacitiesC::all_zero())),
            None,
        )
        .expect("admit nested OuterPair");
        let engine = host_owner_engine();
        let mut image = engine
            .admit_run(
                &admitted,
                &RunIntC::Nat(64),
                &RunIntC::Nat(8),
                &FinanceInput::Absent,
            )
            .expect("genesis nested OuterPair");
        let mut replay_img = image.clone();
        let host_cfg = ext_cfg(b);
        let mut field_steps = Vec::new();

        for step_idx in 0..32 {
            let pre_snap = capture(&admitted, &image).expect("capture pre");
            let replay_step = owner_record(&admitted, &mut replay_img).expect("replay step");
            let ObservedStep::Event(replay_rec) = replay_step else {
                break;
            };
            if replay_rec.event.rule_id == RuleIdC::BHelperValidateField {
                let Some(HelperTaskC::ValidateTask {
                    r: RefC::Const(parent_id),
                    visit: TaskVisitC::Fields,
                    cursor,
                }) = pre_snap.image.tasks.last()
                else {
                    panic!("expected Fields top task");
                };
                let (parent_kind, parent_mf, _, _, _) = const_info(&pre_snap, *parent_id);
                let w_spec = FieldSpec {
                    parent_kind,
                    parent_marker_free: parent_mf,
                    cursor: *cursor,
                    min_expr_id: 0,
                    min_tasks_len: pre_snap.image.tasks.len(),
                };
                let app = finish_applied(
                    admitted.clone(),
                    image.clone(),
                    pre_snap.clone(),
                    replay_rec.clone(),
                    &host_cfg,
                    &w_spec,
                    &format!("nested OuterPair field at step {step_idx}"),
                );
                let Some(HelperTaskC::ValidateTask {
                    r: RefC::Const(child_id),
                    visit: TaskVisitC::Unvisited,
                    cursor: 0,
                }) = app.record.after.image.tasks.last()
                else {
                    panic!("expected Unvisited child at post top");
                };
                let (child_kind, _, _, _, _) = const_info(&app.record.after, *child_id);
                field_steps.push((
                    parent_kind,
                    *cursor,
                    pre_snap.image.tasks.len(),
                    app.record.after.image.tasks.len(),
                    child_kind,
                ));
                let _ = owner_record(&admitted, &mut image).expect("advance main image");
            } else {
                let _ = owner_record(&admitted, &mut image).expect("advance non-field step");
                if replay_rec.event.rule_id == RuleIdC::KMachineEvalLiteralValid {
                    break;
                }
            }
        }
        assert_eq!(
            field_steps,
            vec![
                (
                    ConstKindWant::Constructor { arity: 2 },
                    0,
                    1,
                    2,
                    ConstKindWant::Constructor { arity: 1 }
                ),
                (
                    ConstKindWant::Constructor { arity: 1 },
                    0,
                    2,
                    2,
                    ConstKindWant::UInt(11)
                ),
                (
                    ConstKindWant::Constructor { arity: 2 },
                    1,
                    1,
                    1,
                    ConstKindWant::Constructor { arity: 1 }
                ),
                (
                    ConstKindWant::Constructor { arity: 1 },
                    0,
                    1,
                    1,
                    ConstKindWant::UInt(22)
                ),
            ],
            "same-typed fields of OuterPair must schedule left_box(11) at cursor=0 (1->2), then 11 at depth 2 (2->2), then right_box(22) at cursor=1 (1->1), then 22 (1->1)"
        );
    }

    // 2. Depth-2 non-tail (`2 -> 3`) and depth-3 tail (`3 -> 3`) via nested
    //    `Branch(Branch(Leaf(10), Leaf(20)), Leaf(30))`:
    //    Verifies preservation of all lower task slots (`0..prefix_len`) on both
    //    non-tail push `2 -> 3` and tail replacement `3 -> 3` in both modes.
    {
        let engine = host_owner_engine();
        let tree_prog = prog(lit_tree(CoreTree::Branch {
            left: Box::new(CoreTree::Branch {
                left: Box::new(tree_leaf(10)),
                right: Box::new(tree_leaf(20)),
            }),
            right: Box::new(tree_leaf(30)),
        }));
        let admitted = engine
            .admit(&tree_prog, &b, &layout(b, None))
            .expect("admit nested Branch");
        let mut image = engine
            .admit_run(
                &admitted,
                &RunIntC::Nat(64),
                &RunIntC::Nat(8),
                &FinanceInput::Absent,
            )
            .expect("genesis nested Branch");
        let mut replay_img = image.clone();
        let host_cfg = core_cfg(b);
        let mut saw_2_to_3 = false;
        let mut saw_3_to_3 = false;

        for step_idx in 0..32 {
            let pre_snap = capture(&admitted, &image).expect("capture pre");
            let replay_step = owner_record(&admitted, &mut replay_img).expect("replay step");
            let ObservedStep::Event(replay_rec) = replay_step else {
                break;
            };
            if replay_rec.event.rule_id == RuleIdC::BHelperValidateField {
                let Some(HelperTaskC::ValidateTask {
                    r: RefC::Const(parent_id),
                    visit: TaskVisitC::Fields,
                    cursor,
                }) = pre_snap.image.tasks.last()
                else {
                    panic!("expected Fields top task");
                };
                let (parent_kind, parent_mf, _, _, _) = const_info(&pre_snap, *parent_id);
                let w_spec = FieldSpec {
                    parent_kind,
                    parent_marker_free: parent_mf,
                    cursor: *cursor,
                    min_expr_id: 0,
                    min_tasks_len: pre_snap.image.tasks.len(),
                };
                let app = finish_applied(
                    admitted.clone(),
                    image.clone(),
                    pre_snap.clone(),
                    replay_rec.clone(),
                    &host_cfg,
                    &w_spec,
                    &format!("nested Branch field at step {step_idx}"),
                );
                if pre_snap.image.tasks.len() == 2 && app.record.after.image.tasks.len() == 3 {
                    saw_2_to_3 = true;
                }
                if pre_snap.image.tasks.len() == 3 && app.record.after.image.tasks.len() == 3 {
                    saw_3_to_3 = true;
                }
                let _ = owner_record(&admitted, &mut image).expect("advance main image");
            } else {
                let _ = owner_record(&admitted, &mut image).expect("advance non-field step");
                if replay_rec.event.rule_id == RuleIdC::KMachineEvalLiteralValid {
                    break;
                }
            }
        }
        assert!(
            saw_2_to_3 && saw_3_to_3,
            "must observe both non-tail 2->3 and tail 3->3 ValidateField transitions with lower slot preservation"
        );
    }
}

#[test]
fn marker_child_scheduled_by_validate_field_before_later_validate_invalid_both_modes() {
    // `BHelperValidateField` schedules a `Marker` child (`ConstantSnapshot::U128OutOfRange`)
    // as an `Unvisited, cursor = 0` child task with `StepDisposition::Applied` and
    // `TriStatus::Pending`; the next step on that `Marker` child task selects
    // `RuleIdC::BHelperValidateInvalid` (which sets `TriStatus::Negative`).
    let leaf_m = run_core(
        &field_spec(ConstKindWant::Leaf, false, 0),
        "ValidateField Leaf with Marker child (tail)",
        &|| prog(lit_tree_marker_leaf()),
    );
    let cons_m = run_core(
        &field_spec(ConstKindWant::Cons, false, 0),
        "ValidateField Cons with Marker head child (non-tail)",
        &|| prog(lit_list_marker_head()),
    );
    let branch_m = run_core(
        &field_spec(ConstKindWant::Branch, false, 1),
        "ValidateField Branch cursor=0 schedules Leaf(Marker)",
        &|| prog(lit_tree_marker_branch()),
    );
    let ctor_m = run_ext(
        &field_spec(ConstKindWant::Constructor { arity: 1 }, false, 0),
        "ValidateField Constructor MkAlpha with Marker child (tail)",
        &|| ext_prog(ext_ctor_one_field(i128_class::KIntC::Negative)),
    );
    let _ = branch_m;

    for (label, app) in [
        ("Leaf Marker child", &leaf_m),
        ("Cons Marker head child", &cons_m),
        ("Constructor Marker child", &ctor_m),
    ] {
        let Some(HelperTaskC::ValidateTask {
            r: RefC::Const(child_cid),
            visit: TaskVisitC::Unvisited,
            cursor: 0,
        }) = app.record.after.image.tasks.last()
        else {
            panic!("{label}: post top task must be Unvisited Const child");
        };
        assert_eq!(
            const_info(&app.record.after, *child_cid).0,
            ConstKindWant::Marker,
            "{label}: scheduled child is ConstantSnapshot::U128OutOfRange (Marker)"
        );
    }

    // Verify the next step after `BHelperValidateField` on `Leaf(AboveU128)` selects
    // `RuleIdC::BHelperValidateInvalid` and is refused by `record_validate_field_step` (non-rollback).
    let b = generous();
    let (admitted, mut image, _) = reach_core(
        b,
        &field_spec(ConstKindWant::Leaf, false, 0),
        "Leaf Marker next-step probe",
        &|| prog(lit_tree_marker_leaf()),
    );
    let _ = record_validate_field_step(&admitted, &mut image, &core_cfg(b), &limits())
        .expect("ValidateField on Leaf(Marker) succeeds");
    let mut twin = image.clone();
    let next_step = owner_record(&admitted, &mut twin).expect("twin next step");
    let ObservedStep::Event(rec_inv) = next_step else {
        panic!("expected ValidateInvalid event");
    };
    assert_eq!(rec_inv.event.rule_id, RuleIdC::BHelperValidateInvalid);
    match record_validate_field_step(&admitted, &mut image, &core_cfg(b), &limits()) {
        Err(C6ValidateFieldConsumerError::UnexpectedOwnerStep) => {}
        other => panic!(
            "expected UnexpectedOwnerStep on BHelperValidateInvalid, ok={}",
            other.is_ok()
        ),
    }
    let after_refusal = capture(&admitted, &image).expect("capture after refusal");
    assert_eq!(
        after_refusal, rec_inv.after,
        "wrapper refusal on BHelperValidateInvalid is non-rollback"
    );
}

#[test]
fn non_zero_expr_non_empty_frames_env_data_slots_cursor_preserved_both_modes() {
    let make = || CoreProgram {
        functions: vec![CoreFunction {
            name: "pair_sum".into(),
            parameters: vec![
                CoreParameter {
                    name: "p_list".into(),
                    sort: SortTag::List,
                },
                CoreParameter {
                    name: "p_num".into(),
                    sort: SortTag::UInt,
                },
            ],
            return_type: SortTag::List,
            body: bin(BinaryOp::Cons, lit_u(99), lit_list(&[20])),
        }],
        entry: CoreExpr::Let {
            name: "xs".into(),
            value: Box::new(bin(BinaryOp::Cons, lit_u(4), lit_list(&[5]))),
            body: Box::new(CoreExpr::Call {
                name: "pair_sum".into(),
                args: vec![
                    CoreExpr::Var { name: "xs".into() },
                    bin(BinaryOp::Add, lit_u(10), lit_u(11)),
                ],
            }),
        },
    };
    let w = FieldSpec {
        parent_kind: ConstKindWant::Cons,
        parent_marker_free: true,
        cursor: 0,
        min_expr_id: 1,
        min_tasks_len: 1,
    };
    let applied = run_core(&w, "ValidateField deep non-empty state", &make);
    let pre = &applied.record.before.image;
    let post = &applied.record.after.image;
    let ControlC::Eval { expr } = pre.control else {
        panic!("expected ControlC::Eval");
    };
    assert!(expr > 0, "selected expr must be non-zero");
    assert_eq!(pre.control_origin, expr, "SI20 control_origin == expr");
    assert!(!pre.frames.is_empty(), "pre frames must be non-empty");
    assert!(!pre.env.is_empty(), "pre env must be non-empty");
    assert!(!pre.data.is_empty(), "pre DATA nodes must be non-empty");
    assert!(
        !pre.data_fields.is_empty(),
        "pre DATA fields must be non-empty"
    );
    assert!(
        !pre.value_slots.is_empty(),
        "pre value_slots must be non-empty"
    );
    assert!(pre.consumed_work > 0, "pre consumed_work > 0");
    assert!(pre.helper_work > 0, "pre helper_work > 0");
    assert!(pre.helper_visits > 0, "pre helper_visits > 0");
    assert!(pre.reference_cursor > 0, "pre reference_cursor > 0");
    assert_eq!(
        post.helper_visits,
        pre.helper_visits + 1,
        "ValidateField visits_of == 1 increments non-zero helper_visits"
    );
    assert_eq!(
        post.tasks.len(),
        pre.tasks.len() + 1,
        "non-tail Cons field step pushes Unvisited child task"
    );
}

fn assert_p3(r: &CompleteStepRecord, w: &FieldSpec, label: &str) {
    assert!(
        selected_field_top(&r.before, w).is_some(),
        "{label}: P3 selected PRE"
    );
    assert_preserved(r, label);
    assert_common_event(r, r.before.image.tasks.len(), label);
    let (pre, post, e) = (&r.before.image, &r.after.image, &r.event);
    assert_eq!(
        pre.helper_work, r.before.admission.bounds.max_helper_work,
        "{label}: PRE helper_work at exact original admission bound"
    );
    assert_eq!(
        e.disposition,
        StepDisposition::RefusedHelperWorkUncharged,
        "{label}: P3 disposition"
    );
    assert_eq!(
        (e.work_delta, e.helper_work_delta, e.helper_visits_delta),
        (0, 0, 0),
        "{label}: P3 zero charges"
    );
    assert_eq!(
        e.exhausted_kind,
        Some(ExhaustReason::HelperWork),
        "{label}: P3 exhausted kind"
    );
    assert_eq!(
        e.outcome_tag,
        OutcomeTagC::Exhausted,
        "{label}: P3 outcome tag"
    );
    assert_eq!(
        e.outcome_payload,
        OutcomeC::Exhausted {
            reason: ExhaustReason::HelperWork
        },
        "{label}: P3 outcome payload"
    );
    assert_eq!(e.boundary, BoundaryKind::Terminal, "{label}: P3 boundary");
    assert_eq!(
        post.remaining_work, pre.remaining_work,
        "{label}: P3 remaining"
    );
    assert_eq!(
        post.consumed_work, pre.consumed_work,
        "{label}: P3 consumed"
    );
    assert_eq!(
        post.helper_work, pre.helper_work,
        "{label}: P3 helper_work"
    );
    assert_eq!(
        post.helper_visits, pre.helper_visits,
        "{label}: P3 visits"
    );
    assert_eq!(
        post.tasks, pre.tasks,
        "{label}: P3 tasks preserved unchanged"
    );
    assert_eq!(
        post.outcome,
        OutcomeC::Exhausted {
            reason: ExhaustReason::HelperWork
        },
        "{label}: P3 post outcome"
    );
}

fn assert_p6(r: &CompleteStepRecord, w: &FieldSpec, label: &str) {
    let (_, cursor, checked_arity, _) =
        selected_field_top(&r.before, w).unwrap_or_else(|| panic!("{label}: P6 selected PRE"));
    assert!(cursor + 1 < checked_arity, "{label}: P6 requires non-tail");
    assert_preserved(r, label);
    assert_common_event(r, r.before.image.tasks.len(), label);
    let (pre, post, e) = (&r.before.image, &r.after.image, &r.event);
    assert!(
        pre.helper_work < r.before.admission.bounds.max_helper_work,
        "{label}: P6 requires helper_work < max_helper_work"
    );
    assert!(
        pre.tasks.len() + 1 > r.before.admission.bounds.helper_tasks,
        "{label}: P6 requires tasks.len() + 1 > admission.bounds.helper_tasks"
    );
    assert_eq!(
        e.disposition,
        StepDisposition::RefusedCapCharged,
        "{label}: P6 disposition"
    );
    assert_eq!(
        (e.work_delta, e.helper_work_delta, e.helper_visits_delta),
        (1, 1, 0),
        "{label}: P6 charges work=1 and helper_work=1 with helper_visits=0"
    );
    assert_eq!(
        e.exhausted_kind,
        Some(ExhaustReason::HelperTasks),
        "{label}: P6 exhausted kind HelperTasks"
    );
    assert_eq!(
        e.outcome_tag,
        OutcomeTagC::Exhausted,
        "{label}: P6 outcome tag"
    );
    assert_eq!(
        e.outcome_payload,
        OutcomeC::Exhausted {
            reason: ExhaustReason::HelperTasks
        },
        "{label}: P6 outcome payload"
    );
    assert_eq!(e.boundary, BoundaryKind::Terminal, "{label}: P6 boundary");
    assert_eq!(
        post.remaining_work,
        pre.remaining_work - 1,
        "{label}: P6 remaining_work - 1"
    );
    assert_eq!(
        post.consumed_work,
        pre.consumed_work + 1,
        "{label}: P6 consumed_work + 1"
    );
    assert_eq!(
        post.helper_work,
        pre.helper_work + 1,
        "{label}: P6 helper_work + 1"
    );
    assert_eq!(
        post.helper_visits, pre.helper_visits,
        "{label}: P6 helper_visits unchanged"
    );
    assert_eq!(
        post.tasks, pre.tasks,
        "{label}: P6 preserves all tasks unchanged (no parent rewrite, no child push)"
    );
    assert_eq!(
        post.outcome,
        OutcomeC::Exhausted {
            reason: ExhaustReason::HelperTasks
        },
        "{label}: P6 post outcome"
    );
}

#[test]
fn p3_p6_and_tail_at_bound_precedence_and_retained_admission_bounds_both_modes() {
    // 1. P6 (`StepDisposition::RefusedCapCharged` / `ExhaustReason::HelperTasks`):
    //    Non-tail `Cons([7])` at `cursor == 0` (`tasks.len() == 1`, `is_nontail == true`)
    //    with `bounds.helper_tasks == 1` and `max_helper_work == 100`.
    //    Verified against both matching layout `helper_tasks == 1` and larger layout
    //    `helper_tasks == 4` (`core_cfg(generous())`).
    {
        let w = field_spec(ConstKindWant::Cons, true, 0);
        let make = &|| prog(lit_list(&[7]));
        let b_p6 = bounds_with(100, 1);
        for (host_cfg, cfg_label) in [
            (core_cfg(b_p6), "matching layout helper_tasks=1"),
            (
                core_cfg(generous()),
                "retained admission helper_tasks=1 vs larger layout helper_tasks=4",
            ),
        ] {
            let full_label = format!("P6 non-tail Cons at helper_tasks=1 ({cfg_label})");
            let (admitted, mut image, pre) = reach_core(b_p6, &w, &full_label, make);
            let (radm, mut rimg, _) = reach_core(b_p6, &w, &full_label, make);
            let replay = match owner_record(&radm, &mut rimg) {
                Ok(ObservedStep::Event(r)) => r,
                _ => panic!("{full_label}: P6 replay must produce event"),
            };
            assert_eq!(replay.before, pre);
            assert_p6(&replay, &w, &full_label);
            let witness = record_validate_field_step(&admitted, &mut image, &host_cfg, &limits())
                .unwrap_or_else(|e| panic!("{full_label}: P6 wrapper: {e}"));
            let post = capture(&admitted, &image).expect("P6 capture POST");
            assert_eq!(post, replay.after);
            assert!(
                witness.enable()
                    && !witness.disposition_is_refused_helper_work()
                    && witness.disposition_is_refused_helper_tasks(),
                "{full_label}: P6 flags (p3=false, p6=true)"
            );
            assert_both_modes(&witness, &full_label);
        }
    }

    // 2. Tail-at-bound (`StepDisposition::Applied`, NOT P6!):
    //    (a) `Leaf(7)` at `cursor == 0` (`checked_arity == 1`, `is_nontail == false`) with
    //        `bounds.helper_tasks == 1` (`pre.tasks.len() == 1 == bounds.helper_tasks`):
    //        replaces top task in place (`task_live` delta `0`), so `Applied` succeeds!
    {
        let w_leaf = field_spec(ConstKindWant::Leaf, true, 0);
        let make_leaf = &|| prog(lit_tree(tree_leaf(7)));
        let b_tight = bounds_with(100, 1);
        let (admitted, image, pre) = reach_core(b_tight, &w_leaf, "tail-at-bound Leaf", make_leaf);
        let (radm, mut rimg, _) =
            reach_core(b_tight, &w_leaf, "tail-at-bound Leaf replay", make_leaf);
        let replay = match owner_record(&radm, &mut rimg) {
            Ok(ObservedStep::Event(r)) => r,
            _ => panic!("tail-at-bound Leaf must produce Applied event"),
        };
        let _ = finish_applied(
            admitted,
            image,
            pre,
            replay,
            &core_cfg(b_tight),
            &w_leaf,
            "tail-at-bound Leaf at tasks.len()==helper_tasks==1",
        );
    }
    //    (b) Nested depth-2 tail-at-bound (`OuterPair` -> `left_box` `Alpha(11)` at `cursor == 0`,
    //        `tasks.len() == 2 == bounds.helper_tasks` when `bounds.helper_tasks == 2`):
    //        `OuterPair(cursor=0)` is non-tail `1 -> 2` (within `helper_tasks == 2`), and then
    //        `left_box(cursor=0)` is tail `2 -> 2` AT `tasks.len() == helper_tasks == 2`, which
    //        succeeds as `StepDisposition::Applied`!
    {
        let w_depth2_tail = FieldSpec {
            parent_kind: ConstKindWant::Constructor { arity: 1 },
            parent_marker_free: true,
            cursor: 0,
            min_expr_id: 0,
            min_tasks_len: 2,
        };
        let make_outer = &|| ext_prog(ext_nested_outer_pair(11, 22));
        let b_tight2 = bounds_with(100, 2);
        let (admitted, image, pre) =
            reach_ext(b_tight2, &w_depth2_tail, "depth-2 tail-at-bound", make_outer);
        assert_eq!(pre.image.tasks.len(), 2);
        let (radm, mut rimg, _) = reach_ext(
            b_tight2,
            &w_depth2_tail,
            "depth-2 tail-at-bound replay",
            make_outer,
        );
        let replay = match owner_record(&radm, &mut rimg) {
            Ok(ObservedStep::Event(r)) => r,
            _ => panic!("depth-2 tail-at-bound must produce Applied event"),
        };
        let _ = finish_applied(
            admitted,
            image,
            pre,
            replay,
            &ext_cfg(b_tight2),
            &w_depth2_tail,
            "depth-2 tail-at-bound at tasks.len()==helper_tasks==2",
        );
    }

    // 3. P3 (`StepDisposition::RefusedHelperWorkUncharged` / `ExhaustReason::HelperWork`):
    //    - Tail `Leaf(7)` at `helper_work == max_helper_work == 2`,
    //    - Non-tail `Cons([7])` at `helper_work == max_helper_work == 2` (`helper_tasks == 4`),
    //    - **P3 dominates would-be P6**: Non-tail `Cons([7])` at `cursor == 0` with BOTH
    //      `pre.helper_work == max_helper_work == 2` AND `bounds.helper_tasks == 1`
    //      (`pre.tasks.len() + 1 == 2 > helper_tasks == 1`):
    //      P3 wins (`RefusedHelperWorkUncharged`, uncharged, `p3 = true`, `p6 = false`)!
    for (w, b_p3, label, make) in [
        (
            field_spec(ConstKindWant::Leaf, true, 0),
            bounds_with(2, 4),
            "P3 tail Leaf at helper_work=2",
            &|| prog(lit_tree(tree_leaf(7))) as CoreProgram,
        ),
        (
            field_spec(ConstKindWant::Cons, true, 0),
            bounds_with(2, 4),
            "P3 non-tail Cons at helper_work=2, helper_tasks=4",
            &|| prog(lit_list(&[7])),
        ),
        (
            field_spec(ConstKindWant::Cons, true, 0),
            bounds_with(2, 1),
            "P3 dominates would-be P6 on non-tail Cons (helper_work=2==max, tasks+1=2>helper_tasks=1)",
            &|| prog(lit_list(&[7])),
        ),
    ] {
        for (host_cfg, cfg_label) in [
            (core_cfg(b_p3), "matching layout bounds"),
            (
                core_cfg(generous()),
                "retained admission bounds vs larger layout bounds",
            ),
        ] {
            let full_label = format!("{label} ({cfg_label})");
            let (admitted, mut image, pre) = reach_core(b_p3, &w, &full_label, make);
            let (radm, mut rimg, _) = reach_core(b_p3, &w, &full_label, make);
            let replay = match owner_record(&radm, &mut rimg) {
                Ok(ObservedStep::Event(r)) => r,
                _ => panic!("{full_label}: P3 replay must produce event"),
            };
            assert_eq!(replay.before, pre);
            assert_p3(&replay, &w, &full_label);
            let witness = record_validate_field_step(&admitted, &mut image, &host_cfg, &limits())
                .unwrap_or_else(|e| panic!("{full_label}: P3 wrapper: {e}"));
            let post = capture(&admitted, &image).expect("P3 capture POST");
            assert_eq!(post, replay.after);
            assert!(
                witness.enable()
                    && witness.disposition_is_refused_helper_work()
                    && !witness.disposition_is_refused_helper_tasks(),
                "{full_label}: P3 flags (p3=true, p6=false even when would-be P6 holds)"
            );
            assert_both_modes(&witness, &full_label);
        }
    }
}

#[test]
fn p2_fuel_zero_and_wrapper_wrong_rule_refusals_are_nonrollback() {
    // 1. P2 fuel exhaustion (`initial_fuel == 2`):
    //    `BHelperValidateEntry` + `BHelperValidateOccurrence` consume the 2 fuel
    //    (`remaining_work: 2 -> 1 -> 0`). At the `BHelperValidateField` pre-state
    //    (`remaining_work == 0`), `owner_step` selects `RuleIdC::KAdminPublishExhausted`,
    //    and `record_validate_field_step` refuses with `UnexpectedOwnerStep` (non-rollback).
    let w = field_spec(ConstKindWant::Leaf, true, 0);
    let make = &|| prog(lit_tree(tree_leaf(7)));
    let (admitted, mut image, pre) =
        reach_core_fuel(generous(), 2, &w, "P2 fuel 0 at ValidateField", make);
    assert_eq!(pre.image.remaining_work, 0, "P2: PRE fuel is zero");
    assert!(
        selected_field_top(&pre, &w).is_some(),
        "P2: Validate Pending + Fields top task before driver P2"
    );
    let (radm, mut rimg, _) =
        reach_core_fuel(generous(), 2, &w, "P2 fuel 0 at ValidateField replay", make);
    let replay = match owner_record(&radm, &mut rimg) {
        Ok(ObservedStep::Event(r)) => r,
        _ => panic!("P2: replay must publish Exhausted event"),
    };
    assert_eq!(replay.event.rule_id, RuleIdC::KAdminPublishExhausted);
    assert_eq!(
        replay.event.disposition,
        StepDisposition::ExhaustedFuelUncharged
    );
    match record_validate_field_step(&admitted, &mut image, &core_cfg(generous()), &limits()) {
        Err(C6ValidateFieldConsumerError::UnexpectedOwnerStep) => {}
        other => panic!(
            "P2: wrapper must refuse with UnexpectedOwnerStep, ok={}",
            other.is_ok()
        ),
    }
    let post = capture(&admitted, &image).expect("P2 capture POST");
    assert_eq!(post, replay.after, "P2: wrapper refusal is non-rollback");

    // 2. Wrong-rule refusals across `Leaf(7)` walk:
    //    - Step 1 (`BHelperValidateEntry`) -> `UnexpectedOwnerStep` (non-rollback)
    //    - Step 2 (`BHelperValidateOccurrence`) -> `UnexpectedOwnerStep` (non-rollback)
    //    - Step 3 (`BHelperValidateField`) -> `Ok(witness)`!
    //    - Step 4 (`BHelperValidateOccurrence` on child `UInt(7)`) -> `UnexpectedOwnerStep` (non-rollback)
    //    - Step 5 (`BHelperValidateEmpty`) -> `UnexpectedOwnerStep` (non-rollback)
    let b = generous();
    let engine = host_owner_engine();
    let admitted = engine
        .admit(&prog(lit_tree(tree_leaf(7))), &b, &layout(b, None))
        .expect("admit Leaf(7)");
    let mut img = engine
        .admit_run(
            &admitted,
            &RunIntC::Nat(64),
            &RunIntC::Nat(8),
            &FinanceInput::Absent,
        )
        .expect("genesis Leaf(7)");

    for step_label in ["Step 1 Entry", "Step 2 Occurrence"] {
        let before = capture(&admitted, &img).expect("capture before");
        match record_validate_field_step(&admitted, &mut img, &core_cfg(b), &limits()) {
            Err(C6ValidateFieldConsumerError::UnexpectedOwnerStep) => {}
            other => panic!("{step_label}: expected UnexpectedOwnerStep, ok={}", other.is_ok()),
        }
        let after = capture(&admitted, &img).expect("capture after");
        assert_ne!(after, before, "{step_label}: non-rollback");
    }

    let field_wit = record_validate_field_step(&admitted, &mut img, &core_cfg(b), &limits())
        .expect("Step 3 is BHelperValidateField on Leaf(7)");
    assert_both_modes(&field_wit, "Leaf(7) step 3 ValidateField");

    for step_label in ["Step 4 child Occurrence", "Step 5 Empty"] {
        let before = capture(&admitted, &img).expect("capture before");
        match record_validate_field_step(&admitted, &mut img, &core_cfg(b), &limits()) {
            Err(C6ValidateFieldConsumerError::UnexpectedOwnerStep) => {}
            other => panic!("{step_label}: expected UnexpectedOwnerStep, ok={}", other.is_ok()),
        }
        let after = capture(&admitted, &img).expect("capture after");
        assert_ne!(after, before, "{step_label}: non-rollback");
    }
}

#[test]
fn host_pair_canonical_wrong_post_reaches_sdk_checker_both_modes() {
    let nontail_app = run_core(
        &field_spec(ConstKindWant::Cons, true, 0),
        "wrong-post base non-tail Cons cursor=0",
        &|| prog(lit_list(&[7])),
    );
    let tail_app = run_core(
        &field_spec(ConstKindWant::Leaf, true, 0),
        "wrong-post base tail Leaf cursor=0",
        &|| prog(lit_tree(tree_leaf(42))),
    );

    // 1. Non-tail Cons cursor=0 where parent slot 0 cursor is left at 0 instead of rewritten to 1.
    let mut unincremented_parent = nontail_app.record.after.clone();
    if let Some(HelperTaskC::ValidateTask { cursor, .. }) =
        unincremented_parent.image.tasks.first_mut()
    {
        *cursor = 0;
    }

    // 2. Non-tail Cons cursor=0 where child is not pushed (replaced in place like tail `1 -> 1`).
    let mut replaced_instead_of_pushed = nontail_app.record.after.clone();
    replaced_instead_of_pushed.image.tasks.remove(0);

    // 3. Non-tail Cons cursor=0 where pushed child has `visit = TaskVisitC::Fields` instead of `Unvisited`.
    let mut child_visit_fields = nontail_app.record.after.clone();
    if let Some(HelperTaskC::ValidateTask { visit, .. }) = child_visit_fields.image.tasks.last_mut()
    {
        *visit = TaskVisitC::Fields;
    }

    // 4. Non-tail Cons cursor=0 where pushed child points to wrong constant `RefC::Const(0)` (or `1`).
    let mut wrong_child_ref = nontail_app.record.after.clone();
    if let Some(HelperTaskC::ValidateTask {
        r: RefC::Const(cid),
        ..
    }) = wrong_child_ref.image.tasks.last_mut()
    {
        *cid = if *cid == 0 { 1 } else { 0 };
    }

    // 5. Tail Leaf cursor=0 where child is pushed (`1 -> 2`) instead of replacing parent in place (`1 -> 1`).
    let mut pushed_on_tail = tail_app.record.after.clone();
    let mut parent_rewritten = tail_app.record.before.image.tasks[0].clone();
    if let HelperTaskC::ValidateTask { cursor, .. } = &mut parent_rewritten {
        *cursor = 1;
    }
    pushed_on_tail.image.tasks.insert(0, parent_rewritten);

    // 6. Tail Leaf cursor=0 where `helper_visits` is not incremented.
    let mut uncharged_visits = tail_app.record.after.clone();
    uncharged_visits.image.helper_visits = tail_app.record.before.image.helper_visits;

    for (before, wrong_after, label) in [
        (
            &nontail_app.record.before,
            &unincremented_parent,
            "non-tail parent cursor left at 0",
        ),
        (
            &nontail_app.record.before,
            &replaced_instead_of_pushed,
            "non-tail replaced in place instead of pushed",
        ),
        (
            &nontail_app.record.before,
            &child_visit_fields,
            "non-tail pushed child with visit=Fields",
        ),
        (
            &nontail_app.record.before,
            &wrong_child_ref,
            "non-tail pushed wrong child constant id",
        ),
        (
            &tail_app.record.before,
            &pushed_on_tail,
            "tail pushed child instead of replacing in place",
        ),
        (
            &tail_app.record.before,
            &uncharged_visits,
            "tail uncharged helper_visits",
        ),
    ] {
        let witness = witness_validate_field_from_host_pair(
            true,
            before,
            wrong_after,
            &core_cfg(generous()),
            &limits(),
        )
        .unwrap_or_else(|e| panic!("{label}: host-pair must not semantically filter: {e}"));
        for mode in [
            C6PushStatementBindingMode::LocalPublic,
            C6PushStatementBindingMode::LedgerCall,
        ] {
            let circuit = build_c6_validate_field_step_circuit(
                witness.source_params().clone(),
                witness.host_prefix().meta.len(),
                witness.host_prefix().prog.len(),
                witness.host_prefix().meta_bounds_frames_offset,
                mode,
                "validate_field_wrong_post",
            )
            .unwrap_or_else(|e| panic!("{label}: build {mode:?}: {e}"));
            match check_c6_validate_field_step_circuit(&circuit, &witness) {
                Err(C6ValidateFieldConsumerError::Backend(m)) => assert!(
                    m.contains("SDK ProofPreimage::check failed"),
                    "{label} {mode:?}: SDK preprocessing category required, got: {m}"
                ),
                other => panic!(
                    "{label} {mode:?}: expected checker rejection, ok={}",
                    other.is_ok()
                ),
            }
        }
    }
}

#[test]
fn owner_record_mutations_reject_exact_host_consistency_and_ps_before_p3_p6() {
    let nontail = run_core(
        &field_spec(ConstKindWant::Cons, true, 0),
        "record-mutation Cons cursor=0",
        &|| prog(lit_list(&[7])),
    );
    let tail = run_core(
        &field_spec(ConstKindWant::Leaf, true, 0),
        "record-mutation Leaf cursor=0",
        &|| prog(lit_tree(tree_leaf(42))),
    );

    // 1. Wrong `rule_id`:
    let mut bad_rule = nontail.record.clone();
    bad_rule.event.rule_id = RuleIdC::BHelperValidateOccurrence;
    assert!(matches!(
        witness_validate_field_from_owner_record(&bad_rule, &core_cfg(generous()), &limits()),
        Err(C6ValidateFieldConsumerError::UnexpectedOwnerStep)
    ));

    // 2. Claiming P6 on a tail step at `tasks.len() == helper_tasks == 1`:
    let mut fake_p6_on_tail = tail.record.clone();
    fake_p6_on_tail.before.admission.bounds.helper_tasks = 1;
    fake_p6_on_tail.after.admission.bounds.helper_tasks = 1;
    fake_p6_on_tail.event.disposition = StepDisposition::RefusedCapCharged;
    fake_p6_on_tail.event.exhausted_kind = Some(ExhaustReason::HelperTasks);
    fake_p6_on_tail.event.outcome_tag = OutcomeTagC::Exhausted;
    fake_p6_on_tail.event.outcome_payload = OutcomeC::Exhausted {
        reason: ExhaustReason::HelperTasks,
    };
    fake_p6_on_tail.event.boundary = BoundaryKind::Terminal;
    fake_p6_on_tail.event.helper_visits_delta = 0;
    fake_p6_on_tail.after.image.helper_visits = fake_p6_on_tail.before.image.helper_visits;
    fake_p6_on_tail.after.image.tasks = fake_p6_on_tail.before.image.tasks.clone();
    assert!(matches!(
        witness_validate_field_from_owner_record(
            &fake_p6_on_tail,
            &core_cfg(bounds_with(100, 1)),
            &limits()
        ),
        Err(C6ValidateFieldConsumerError::EventMismatch {
            field: "snapshot disposition p6"
        })
    ));

    // 3. Claiming P6 when P3 (`pre.helper_work == max_helper_work`) also holds:
    let mut fake_p6_when_p3 = nontail.record.clone();
    let hw = fake_p6_when_p3.before.image.helper_work;
    fake_p6_when_p3.before.admission.bounds.max_helper_work = hw;
    fake_p6_when_p3.after.admission.bounds.max_helper_work = hw;
    fake_p6_when_p3.before.admission.bounds.helper_tasks = 1;
    fake_p6_when_p3.after.admission.bounds.helper_tasks = 1;
    fake_p6_when_p3.event.disposition = StepDisposition::RefusedCapCharged;
    assert!(matches!(
        witness_validate_field_from_owner_record(
            &fake_p6_when_p3,
            &core_cfg(bounds_with(hw, 1)),
            &limits()
        ),
        Err(C6ValidateFieldConsumerError::EventMismatch {
            field: "snapshot disposition p3"
        })
    ));

    // 4. PS checks precede P3 and P6 in `validate_owner_record`:
    //    Build a genuine P3 record (`pre.helper_work == max_helper_work == 2`), then corrupt:
    //    (a) `cursor >= slice_len` (`cursor = 2` on `Cons`),
    //    (b) `child_cid >= constants.len()` (`SC-05` `check_written_code_ids`),
    //    and verify `validate_owner_record` rejects at the PS check even though P3 holds!
    let w_cons = field_spec(ConstKindWant::Cons, true, 0);
    let b_p3 = bounds_with(2, 1);
    let (adm_p3, mut img_p3, _) =
        reach_core(b_p3, &w_cons, "PS-before-P3 base", &|| prog(lit_list(&[7])));
    let ObservedStep::Event(rec_p3) = owner_record(&adm_p3, &mut img_p3).expect("P3 step") else {
        panic!("expected P3 event");
    };

    let mut ps_oob_cursor = rec_p3.clone();
    if let Some(HelperTaskC::ValidateTask { cursor, .. }) =
        ps_oob_cursor.before.image.tasks.last_mut()
    {
        *cursor = 2; // == slice_len(2)
    }
    ps_oob_cursor.after.image.tasks = ps_oob_cursor.before.image.tasks.clone();
    assert!(matches!(
        witness_validate_field_from_owner_record(&ps_oob_cursor, &core_cfg(b_p3), &limits()),
        Err(C6ValidateFieldConsumerError::EventMismatch {
            field: "snapshot const field index"
        })
    ));

    let mut ps_sc05_child = rec_p3;
    let oob_cid = ps_sc05_child.before.admission.view.constants.len();
    ps_sc05_child.before.admission.view.constant_fields[0] = oob_cid;
    ps_sc05_child.after.admission.view.constant_fields[0] = oob_cid;
    assert!(matches!(
        witness_validate_field_from_owner_record(&ps_sc05_child, &core_cfg(b_p3), &limits()),
        Err(C6ValidateFieldConsumerError::EventMismatch {
            field: "snapshot child const id"
        })
    ));
}
