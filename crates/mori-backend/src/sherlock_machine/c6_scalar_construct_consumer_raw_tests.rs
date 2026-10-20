//! Independent raw tests for `c6_scalar_construct_consumer`, disjoint
//! from any positive sibling. Source-only and UNRUN.
//!
//! Every baseline is admitted through the real public owner
//! (`host_owner_engine().admit` / `admit_run`, or actual public
//! `admit_ext_program` for the nominal fixture) and searched with
//! real `record_step` to the actual selected PRE:
//! `(First,Leaf)`, `(Second,Cons)`, or `(Second,Branch)`, with
//! valid SI22 A (`Cons`: UInt; `Branch`: Leaf|Branch), `regA==0`,
//! staged `None`, expected absent, `operandB` present iff binary.
//! No snapshot/helper/frame/record is fabricated, no private
//! `MachineImage` is patched, no table is fit/resized, and no
//! expected post is derived from emitter `post_*` /
//! `is_helper_work_max` ungated receipts.
//!
//! Raw mutations are `Vec<Fr>` statements checked by real SDK
//! `ProofPreimage::check` preprocessing (not synthesis/proof/native)
//! in BOTH modes. LedgerCall commitments are recomputed from the
//! CURRENT statement plus CURRENT post on every check; the one
//! stale-commitment case is isolated and labelled.
//!
//! Marker specimens below are RAW local-relation cases only. Their
//! genuine baselines contain an admitted, not-held Marker row; the
//! raw statement then redirects the effect reference to that row.
//! That intentionally violates retained SI17 for a held Scalar
//! operand and also breaks SI08/SI21 entry-installed operand
//! identity. They are never genuine admitted reachability,
//! full-StepInv, SI17-proof, or authenticated owner records.
//!
//! Retained, not discharged: SI08/SI17/SI21/SI22/StepInv as wholes,
//! ScalarEntry/ScalarTag installation coherence, program membership,
//! P1/native. Exact remaining classes are listed at the end of the
//! delivery note, not claimed here.

use std::collections::BTreeSet;

use midnight_transient_crypto::curve::Fr;
use midnight_transient_crypto::hash::transient_commit;
use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction};

use mori_core::sherlock::helper_machine::c6_phys::{
    C6CodecBudgets, L_CONSUMED_WORK, L_CONTROL, L_HELPER_VISITS,
    L_HELPER_WORK, L_OUTCOME, L_REFERENCE_CURSOR, L_REMAINING_WORK,
};
use mori_core::sherlock::helper_machine::c6_phys_data::{ENV_ROW_LANES, REF_LANES};
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_HELPER_RECORD, L_RESULT_RECORD};
use mori_core::sherlock::helper_machine::c6_phys_host::{
    C6HostConfig, C6HostLimits, CTX_LANES, C6_HOST_SCHEMA,
};
use mori_core::sherlock::helper_machine::complete_observation::{
    capture, record_step as owner_record, CompleteStepRecord, ConstantSnapshot,
    DataSnapshot, HostSnapshot, ObservedStep,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_admit::ExtCapacitiesC;
use mori_core::sherlock::helper_machine::shared::owner_ext_source_program::{
    ExtDomain, ExtProgram,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_t3_uses::{
    ExtExpr, ExtFunction, ExtValue,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::{
    CtorDeclSrc, ExtTypeBuildError, TypeDeclSrc, TypeExprSrc,
};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::*;

use super::{
    build_c6_scalar_construct_step_circuit, capture_step_error_label,
    check_c6_scalar_construct_step_circuit, witness_from_host_pair,
    witness_from_owner_record, C6ScalarConstructConsumerError,
    C6ScalarConstructStepCircuit, C6ScalarConstructStepWitness, CHECK_KEY_LOCATION,
};
use super::super::c6_push_step::{
    C6PushStatementBindingMode, C6_PADDED_CARRIER_VERSION,
};
use super::super::c6_scalar_construct_step::{
    emit_c6_scalar_construct_step_block, C6ScalarConstructStepEmitter,
    C6ScalarConstructStepError, C6ScalarConstructStepInputs,
};

fn tooling_label(e: &Tooling) -> String {
    match e {
        Tooling::AdmissionRejected { reason, detail, locator } => format!(
            "admission rejected: reason={reason:?}, detail={detail}, locator={locator:?}"),
        Tooling::CapacityNotAdmitted { kind, needed, limit, locator } => format!(
            "capacity: kind={kind:?}, needed={needed:?}, limit={limit}, locator={locator:?}"),
        Tooling::UnsupportedFeature { feature, locator } => format!(
            "unsupported: feature={feature:?}, locator={locator:?}"),
    }
}
fn ext_label(e: &ExtTypeBuildError) -> String {
    match e {
        ExtTypeBuildError::CoreTooling(t) => tooling_label(t),
        ExtTypeBuildError::ExtCapacityNotAdmitted { kind, needed, limit, locator } => format!(
            "ext capacity: kind={kind:?}, needed={needed:?}, limit={limit}, locator={locator:?}"),
        ExtTypeBuildError::ExtAdmissionRejected { reason, detail, locator } => format!(
            "ext rejected: reason={reason:?}, detail={detail}, locator={locator:?}"),
        ExtTypeBuildError::ExtUnsupportedFeature { feature, locator } => format!(
            "ext unsupported: feature={feature:?}, locator={locator:?}"),
        ExtTypeBuildError::Host(v) => format!(
            "ext host: kind={:?}, clause={:?}, rule={:?}, locator={}",
            v.kind, v.clause, v.rule, v.locator.as_deref().unwrap_or("<none>")),
    }
}
fn bounds(max_helper_work: u128) -> BoundsC {
    BoundsC {
        code_rows: 64, function_rows: 8, code_child_slots: 64, parameter_slots: 16,
        pattern_rows: 16, match_plan_rows: 16, match_arm_rows: 16, type_rows: 16,
        constructor_rows: 16, constructor_field_slots: 16, max_arity: 4,
        constant_nodes: 64, constant_field_slots: 64, data_nodes: 16,
        data_field_slots: 16, environment_cells: 16, frames: 16, helper_tasks: 4,
        serialization_bytes: 1_000_000, max_reference_work: 1_000_000,
        max_helper_work, step_cap: 1_000_000,
    }
}
fn layout(b: BoundsC, ext: Option<ExtCapacitiesC>) -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("lv".into(), "1".into()), field_id: None, widths: None,
        scalar_layout_id: None, bounds: b,
        capacities: LayoutCapacitiesC {
            name_rows: 32, name_bytes: 512, occurrence_path_slots: 64, value_slots: 512 },
        ext_capacities: ext, boundary_capacities_present: false,
    }
}
fn cfg(b: BoundsC, ext: Option<ExtCapacitiesC>) -> C6HostConfig {
    C6HostConfig::from_retained_layout(&layout(b, ext),
        C6CodecBudgets { type_expr_nodes: 256, template_bytes: 4096 })
}
fn core_cfg(b: BoundsC) -> C6HostConfig { cfg(b, None) }
fn ext_cfg(b: BoundsC) -> C6HostConfig { cfg(b, Some(ExtCapacitiesC::all_zero())) }
fn limits() -> C6HostLimits {
    C6HostLimits { max_total_lanes: 1 << 20, max_state_lanes: 1 << 20,
        max_metadata_bytes: 8192, max_string_bytes: 512, max_width_entries: 64 }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KindWant { UInt(u128), Bool(bool), Nil, Cons, Leaf, Branch, Ctor, AnyUInt }
#[derive(Clone, Copy, Debug)]
struct Spec {
    mode: ScalarMode, phase: ScalarPhase, site: SiteTag,
    a: KindWant, b: Option<KindWant>, a_data: Option<bool>,
    staged: ScalarStaged,
}
fn leaf(a: KindWant, staged: ScalarStaged) -> Spec {
    Spec { mode: ScalarMode::Leaf, phase: ScalarPhase::First, site: SiteTag::RetUnary,
        a, b: None, a_data: None, staged }
}
fn cons(a: KindWant, b: KindWant, staged: ScalarStaged) -> Spec {
    Spec { mode: ScalarMode::Cons, phase: ScalarPhase::Second, site: SiteTag::RetRight,
        a, b: Some(b), a_data: None, staged }
}
fn branch(a: KindWant, b: KindWant, staged: ScalarStaged) -> Spec {
    Spec { mode: ScalarMode::Branch, phase: ScalarPhase::Second, site: SiteTag::RetRight,
        a, b: Some(b), a_data: None, staged }
}
fn operand_kind(s: &HostSnapshot, r: RefC) -> KindWant {
    match r {
        RefC::Const(id) => match s.admission.view.constants.get(id) {
            Some(ConstantSnapshot::U128 { n, .. }) => KindWant::UInt(*n),
            Some(ConstantSnapshot::Bool { b, .. }) => KindWant::Bool(*b),
            Some(ConstantSnapshot::Nil { .. }) => KindWant::Nil,
            Some(ConstantSnapshot::Cons { .. }) => KindWant::Cons,
            Some(ConstantSnapshot::Leaf { .. }) => KindWant::Leaf,
            Some(ConstantSnapshot::Branch { .. }) => KindWant::Branch,
            Some(ConstantSnapshot::Constructor { .. }) => KindWant::Ctor,
            Some(ConstantSnapshot::U128OutOfRange { .. }) =>
                panic!("raw Marker operand is REQUIRED_OPEN, not a delivered positive"),
            None => panic!("selected Const operand must exist"),
        },
        RefC::Data(id) => match s.image.data.get(id) {
            Some(DataSnapshot::U128 { n }) => KindWant::UInt(*n),
            Some(DataSnapshot::Bool { b }) => KindWant::Bool(*b),
            Some(DataSnapshot::Nil) => KindWant::Nil,
            Some(DataSnapshot::Cons { .. }) => KindWant::Cons,
            Some(DataSnapshot::Leaf { .. }) => KindWant::Leaf,
            Some(DataSnapshot::Branch { .. }) => KindWant::Branch,
            Some(DataSnapshot::Constructor { .. }) => KindWant::Ctor,
            None => panic!("selected Data operand must exist"),
        },
    }
}
fn kind_matches(got: KindWant, want: KindWant) -> bool {
    got == want || (want == KindWant::AnyUInt && matches!(got, KindWant::UInt(_)))
}
fn selected_helper(s: &HostSnapshot) -> HelperControlC {
    match s.image.helper { Some(h @ HelperControlC::Scalar { .. }) => h,
        _ => panic!("snapshot must carry a Scalar helper") }
}
fn is_selected(s: &HostSnapshot, w: &Spec) -> bool {
    let helper = match s.image.helper {
        Some(h @ HelperControlC::Scalar { .. }) => h, _ => return false };
    let HelperControlC::Scalar { common, mode, operand_a, operand_b, expected,
        phase, reg_a, staged } = helper else { return false };
    if mode != w.mode || phase != w.phase || common.site != w.site
        || common.parent_resume.is_some() || reg_a != 0
        || staged != ScalarStaged::None || expected.is_some()
        || operand_b.is_some() != w.b.is_some()
        || s.image.result.is_some() || !s.image.tasks.is_empty()
        || s.image.outcome != OutcomeC::Running || s.image.remaining_work == 0
        || common.origin_reference_cursor != s.image.reference_cursor {
        return false;
    }
    if let Some(want_data) = w.a_data {
        if matches!(operand_a, RefC::Data(_)) != want_data { return false; }
    }
    if !kind_matches(operand_kind(s, operand_a), w.a) { return false; }
    // Correct SI22 A-domain is part of PRE, checked before the step.
    match w.mode {
        ScalarMode::Cons => if !matches!(operand_kind(s, operand_a), KindWant::UInt(_)) { return false; }
        ScalarMode::Branch => if !matches!(operand_kind(s, operand_a), KindWant::Leaf | KindWant::Branch) { return false; }
        _ => {}
    }
    match (w.b, operand_b) {
        (None, None) => true,
        (Some(k), Some(rb)) => kind_matches(operand_kind(s, rb), k),
        _ => false,
    }
}
fn search(admitted: Admitted, mut image: MachineImage, w: &Spec, label: &str)
    -> (Admitted, MachineImage, HostSnapshot) {
    for _ in 0..512 {
        let step = owner_record(&admitted, &mut image)
            .unwrap_or_else(|e| panic!("{label}: search: {}", capture_step_error_label(&e)));
        let after = match &step { ObservedStep::Event(r) => &r.after,
            ObservedStep::Terminal { after, .. } =>
                panic!("{label}: terminal before selected PRE: {:?}", after.image.outcome) };
        if is_selected(after, w) {
            let sel = capture(&admitted, &image).unwrap_or_else(|e| panic!(
                "{label}: capture: kind={:?}", e.kind));
            assert_eq!(&sel, after, "{label}: capture equals searched PRE");
            return (admitted, image, sel);
        }
    }
    panic!("{label}: genuine owner never reached selected Construct PRE")
}
fn reach_core(b: BoundsC, w: &Spec, label: &str, make: &dyn Fn() -> CoreProgram)
    -> (Admitted, MachineImage, HostSnapshot) {
    let engine = host_owner_engine();
    let admitted = engine.admit(&make(), &b, &layout(b, None))
        .unwrap_or_else(|e| panic!("{label}: admit: {}", tooling_label(&e)));
    let image = engine.admit_run(&admitted, &RunIntC::Nat(64), &RunIntC::Nat(8), &FinanceInput::Absent)
        .unwrap_or_else(|e| panic!("{label}: genesis: {}", tooling_label(&e)));
    search(admitted, image, w, label)
}
fn reach_ext(b: BoundsC, w: &Spec, label: &str, make: &dyn Fn() -> ExtProgram)
    -> (Admitted, MachineImage, HostSnapshot) {
    let (admitted, _) = admit_ext_program(make(), ExtDomain::GENERIC_ABSENT, &b,
        &layout(b, Some(ExtCapacitiesC::all_zero())), None)
        .unwrap_or_else(|e| panic!("{label}: admit_ext: {}", ext_label(&e)));
    let engine = host_owner_engine();
    let image = engine.admit_run(&admitted, &RunIntC::Nat(64), &RunIntC::Nat(8), &FinanceInput::Absent)
        .unwrap_or_else(|e| panic!("{label}: ext genesis: {}", tooling_label(&e)));
    search(admitted, image, w, label)
}
fn assert_applied(r: &CompleteStepRecord, w: &Spec, label: &str) {
    let (pre, post, e) = (&r.before.image, &r.after.image, &r.event);
    assert!(is_selected(&r.before, w), "{label}: selected PRE");
    assert_eq!(e.rule_id, RuleIdC::BHelperScalarConstruct, "{label}: rule");
    assert_eq!(e.event_kind, EventKind::HelperVisit, "{label}: kind");
    assert_eq!(e.helper_kind, Some(HelperKind::Scalar), "{label}: helper kind");
    assert_eq!(e.disposition, StepDisposition::Applied, "{label}: disposition");
    assert_eq!((e.work_delta, e.helper_work_delta, e.helper_visits_delta), (1, 1, 1), "{label}: deltas");
    assert_eq!(e.reference_cursor_delta, 0, "{label}: cursor delta");
    assert_eq!(e.alloc_deltas, AllocDeltasC { data_nodes: 0, data_fields: 0, environment_cells: 0 }, "{label}: alloc");
    assert_eq!(e.value_slot_writes, 0, "{label}: writes");
    assert!(e.result_tag.is_none() && e.exhausted_kind.is_none(), "{label}: tags");
    assert_eq!(e.outcome_payload, OutcomeC::Running, "{label}: outcome");
    assert_eq!(post.remaining_work, pre.remaining_work - 1, "{label}: remaining");
    assert_eq!(post.consumed_work, pre.consumed_work + 1, "{label}: consumed");
    assert_eq!(post.helper_work, pre.helper_work + 1, "{label}: helper work");
    assert_eq!(post.helper_visits, pre.helper_visits + 1, "{label}: visits");
    assert_eq!(post.reference_cursor, pre.reference_cursor, "{label}: ref cursor");
    assert_eq!(r.after.admission, r.before.admission, "{label}: admission");
    assert_eq!(post.control, pre.control, "{label}: control");
    assert_eq!(post.frames, pre.frames, "{label}: frames");
    assert_eq!(post.env, pre.env, "{label}: env");
    assert_eq!(post.value_slots, pre.value_slots, "{label}: slots");
    assert_eq!(post.tasks, pre.tasks, "{label}: tasks");
    assert_eq!(post.data, pre.data, "{label}: data");
    assert_eq!(post.data_fields, pre.data_fields, "{label}: data fields");
    assert_eq!(post.alloc, pre.alloc, "{label}: alloc counters");
    assert_eq!(post.active, pre.active, "{label}: active");
    let mut want = selected_helper(&r.before);
    if let HelperControlC::Scalar { phase, staged, .. } = &mut want {
        *phase = ScalarPhase::Done; *staged = w.staged;
    }
    assert_eq!(post.helper, Some(want), "{label}: post helper, only phase/staged changed");
    assert!(pre.result.is_none() && post.result.is_none(), "{label}: no result, helper not cleared");
}
struct Applied { record: CompleteStepRecord, witness: C6ScalarConstructStepWitness }
fn finish_applied(admitted: Admitted, mut image: MachineImage, pre: HostSnapshot,
    config: &C6HostConfig, w: &Spec, label: &str) -> Applied {
    let record = match owner_record(&admitted, &mut image) {
        Ok(ObservedStep::Event(r)) => r,
        Ok(ObservedStep::Terminal { .. }) => panic!("{label}: selected step must be event"),
        Err(e) => panic!("{label}: selected step: {}", capture_step_error_label(&e)),
    };
    assert_eq!(record.before, pre, "{label}: record pre");
    assert_applied(&record, w, label);
    let witness = witness_from_owner_record(&record, config, &limits())
        .unwrap_or_else(|e| panic!("{label}: owner witness: {e}"));
    let hp = witness_from_host_pair(true, &record.before, &record.after, config, &limits())
        .unwrap_or_else(|e| panic!("{label}: host-pair witness: {e}"));
    assert_eq!(hp.to_statement_frs(), witness.to_statement_frs(), "{label}: routes agree");
    assert!(witness.enable && !witness.disposition_is_refused_helper_work, "{label}: flags");
    Applied { record, witness }
}
fn run_core(w: &Spec, label: &str, make: &dyn Fn() -> CoreProgram) -> Applied {
    let b = bounds(100);
    let (a, i, pre) = reach_core(b, w, label, make);
    finish_applied(a, i, pre, &core_cfg(b), w, label)
}
fn run_ext(w: &Spec, label: &str, make: &dyn Fn() -> ExtProgram) -> Applied {
    let b = bounds(100);
    let (a, i, pre) = reach_ext(b, w, label, make);
    finish_applied(a, i, pre, &ext_cfg(b), w, label)
}

fn lit_u(n: u128) -> CoreExpr { CoreExpr::Literal { value: CoreValue::UInt { n: i128_class::KIntC::InRange(n) } } }
fn lit_b(v: bool) -> CoreExpr { CoreExpr::Literal { value: CoreValue::Bool { b: v } } }
fn lit_list(vs: &[u128]) -> CoreExpr { CoreExpr::Literal { value: CoreValue::List {
    elements: vs.iter().map(|n| i128_class::KIntC::InRange(*n)).collect() } } }
fn tree_leaf(n: u128) -> CoreTree { CoreTree::Leaf { n: i128_class::KIntC::InRange(n) } }
fn lit_tree(t: CoreTree) -> CoreExpr { CoreExpr::Literal { value: CoreValue::Tree { tree: t } } }
fn bin(op: BinaryOp, l: CoreExpr, r: CoreExpr) -> CoreExpr {
    CoreExpr::Binary { op, left: Box::new(l), right: Box::new(r) } }
fn unary(op: UnaryOp, a: CoreExpr) -> CoreExpr {
    CoreExpr::Unary { op, operand: Box::new(a) } }
fn prog(entry: CoreExpr) -> CoreProgram { CoreProgram { functions: Vec::new(), entry } }
const MAX: u128 = u128::MAX;
fn pending(tag: NodeKind) -> ScalarStaged { ScalarStaged::PendingNode { tag } }
fn err11() -> ScalarStaged { ScalarStaged::PrimError { code: 11 } }

fn leaf_witness() -> Applied {
    run_core(&leaf(KindWant::UInt(5), pending(NodeKind::Leaf)), "Leaf UInt",
        &|| prog(unary(UnaryOp::Leaf, lit_u(5))))
}
fn cons_witness() -> Applied {
    run_core(&cons(KindWant::UInt(7), KindWant::Nil, pending(NodeKind::Cons)), "Cons Nil",
        &|| prog(bin(BinaryOp::Cons, lit_u(7), lit_list(&[]))))
}
fn branch_witness() -> Applied {
    run_core(&branch(KindWant::Leaf, KindWant::Leaf, pending(NodeKind::Branch)), "Branch Leaf",
        &|| prog(bin(BinaryOp::Branch, lit_tree(tree_leaf(5)), lit_tree(tree_leaf(6)))))
}
fn unused_marker_scalar_fn() -> CoreFunction {
    CoreFunction { name: "unused_marker_scalar".into(), parameters: Vec::new(),
        return_type: SortTag::UInt,
        body: CoreExpr::Literal { value: CoreValue::UInt { n: i128_class::KIntC::Negative } } }
}
fn unused_marker_tree_fn() -> CoreFunction {
    CoreFunction { name: "unused_marker_tree".into(), parameters: Vec::new(),
        return_type: SortTag::Tree,
        body: lit_tree(CoreTree::Branch {
            left: Box::new(CoreTree::Leaf { n: i128_class::KIntC::AboveU128 }),
            right: Box::new(tree_leaf(2)) }) }
}
fn prog_unused_markers(entry: CoreExpr) -> CoreProgram {
    CoreProgram { functions: vec![unused_marker_scalar_fn(), unused_marker_tree_fn()], entry }
}
fn marker_leaf_witness() -> Applied {
    run_core(&leaf(KindWant::UInt(5), pending(NodeKind::Leaf)), "MarkerRaw Leaf base",
        &|| prog_unused_markers(unary(UnaryOp::Leaf, lit_u(5))))
}
fn marker_cons_witness() -> Applied {
    run_core(&cons(KindWant::UInt(7), KindWant::Nil, pending(NodeKind::Cons)), "MarkerRaw Cons base",
        &|| prog_unused_markers(bin(BinaryOp::Cons, lit_u(7), lit_list(&[]))))
}
fn marker_branch_witness() -> Applied {
    run_core(&branch(KindWant::Leaf, KindWant::Leaf, pending(NodeKind::Branch)), "MarkerRaw Branch base",
        &|| prog_unused_markers(bin(BinaryOp::Branch, lit_tree(tree_leaf(5)), lit_tree(tree_leaf(6)))))
}
fn find_marker_const_index(s: &HostSnapshot) -> usize {
    s.admission.view.constants.iter().position(|c| match c {
        ConstantSnapshot::U128OutOfRange { marker_free } => !*marker_free,
        _ => false,
    }).expect("genuine baseline must contain an admitted Marker Const row")
}
fn find_transitive_branch_const_index(s: &HostSnapshot) -> usize {
    s.admission.view.constants.iter().position(|c| match c {
        ConstantSnapshot::Branch { marker_free, .. } => !*marker_free,
        _ => false,
    }).expect("genuine baseline must contain a transitively marker_free=false Branch row")
}
fn nominal_leaf_ctor_program() -> ExtProgram {
    ExtProgram {
        types: vec![TypeDeclSrc { name: "Alpha".into(), parameters: Vec::new(),
            ctors: vec![CtorDeclSrc { name: "MkAlpha".into(), fields: Vec::new() }] }],
        assets: Vec::new(), states: Vec::new(), inputs: Vec::new(), functions: Vec::new(),
        entry: ExtExpr::Unary { op: UnaryOp::Leaf, operand: Box::new(ExtExpr::Literal {
            value: ExtValue::Constructor {
                ty: TypeExprSrc::Named { name: "Alpha".into() },
                ctor: "MkAlpha".into(), fields: Vec::new() } }) },
    }
}

// ---------- raw statement harness ----------
fn circuit_for(w: &C6ScalarConstructStepWitness, mode: C6PushStatementBindingMode)
    -> C6ScalarConstructStepCircuit {
    build_c6_scalar_construct_step_circuit(w.source_params.clone(),
        w.host_prefix.meta.len(), w.host_prefix.prog.len(),
        w.host_prefix.meta_bounds_frames_offset, mode, "scalar_construct_raw_test")
        .expect("circuit builds")
}
fn assert_ok_both(w: &C6ScalarConstructStepWitness, label: &str) {
    for mode in [C6PushStatementBindingMode::LocalPublic, C6PushStatementBindingMode::LedgerCall] {
        let c = circuit_for(w, mode);
        assert!(check_c6_scalar_construct_step_circuit(&c, w).is_ok(),
            "{label}: typed positive must pass in {mode:?}");
    }
}
fn prefix_len(w: &C6ScalarConstructStepWitness) -> usize {
    w.host_prefix.meta.len() + w.host_prefix.prog.len() + CTX_LANES }
fn state_len(w: &C6ScalarConstructStepWitness) -> usize {
    w.pre_image.len() + w.pre_data.len() + w.pre_helper.len() }
fn pre_state_start(w: &C6ScalarConstructStepWitness) -> usize { 4 + prefix_len(w) }
fn post_prefix_start(w: &C6ScalarConstructStepWitness) -> usize { pre_state_start(w) + state_len(w) }
fn post_state_start(w: &C6ScalarConstructStepWitness) -> usize { post_prefix_start(w) + prefix_len(w) }
fn pre_meta_at(_w: &C6ScalarConstructStepWitness, i: usize) -> usize { 4 + i }
fn post_meta_at(w: &C6ScalarConstructStepWitness, i: usize) -> usize { post_prefix_start(w) + i }
fn pre_prog_at(w: &C6ScalarConstructStepWitness, i: usize) -> usize { 4 + w.host_prefix.meta.len() + i }
fn post_prog_at(w: &C6ScalarConstructStepWitness, i: usize) -> usize {
    post_prefix_start(w) + w.host_prefix.meta.len() + i }
fn pre_ctx_at(w: &C6ScalarConstructStepWitness, i: usize) -> usize {
    4 + w.host_prefix.meta.len() + w.host_prefix.prog.len() + i }
fn post_ctx_at(w: &C6ScalarConstructStepWitness, i: usize) -> usize {
    post_prefix_start(w) + w.host_prefix.meta.len() + w.host_prefix.prog.len() + i }
fn pre_image_at(w: &C6ScalarConstructStepWitness, l: usize) -> usize { pre_state_start(w) + l }
fn post_image_at(w: &C6ScalarConstructStepWitness, l: usize) -> usize { post_state_start(w) + l }
fn pre_data_at(w: &C6ScalarConstructStepWitness, l: usize) -> usize {
    pre_state_start(w) + w.pre_image.len() + l }
fn post_data_at(w: &C6ScalarConstructStepWitness, l: usize) -> usize {
    post_state_start(w) + w.pre_image.len() + l }
fn pre_helper_at(w: &C6ScalarConstructStepWitness, abs: usize) -> usize {
    pre_state_start(w) + w.pre_image.len() + w.pre_data.len() + abs }
fn post_helper_at(w: &C6ScalarConstructStepWitness, abs: usize) -> usize {
    post_state_start(w) + w.pre_image.len() + w.pre_data.len() + abs }
fn helper_rel(w: &C6ScalarConstructStepWitness, post: bool, rel: usize) -> usize {
    if post { post_helper_at(w, L_HELPER_RECORD + rel) } else { pre_helper_at(w, L_HELPER_RECORD + rel) }
}
fn check_raw(c: &C6ScalarConstructStepCircuit, w: &C6ScalarConstructStepWitness,
    statement: Vec<Fr>, comm_override: Option<Fr>)
    -> Result<Vec<Option<usize>>, C6ScalarConstructConsumerError> {
    let preimage = match c.mode {
        C6PushStatementBindingMode::LocalPublic => ProofPreimage {
            inputs: Vec::new(), private_transcript: Vec::new(),
            public_transcript_inputs: statement.clone(), public_transcript_outputs: statement,
            binding_input: Fr::from(0u64), communications_commitment: None,
            key_location: KeyLocation(CHECK_KEY_LOCATION.into()) },
        C6PushStatementBindingMode::LedgerCall => {
            let randomness = Fr::from(0u64);
            let mut comm_inputs = statement.clone();
            comm_inputs.extend(statement[post_state_start(w)..].iter().copied());
            let comm = comm_override.unwrap_or_else(|| transient_commit(&comm_inputs, randomness));
            ProofPreimage { inputs: statement.clone(), private_transcript: Vec::new(),
                public_transcript_inputs: statement, public_transcript_outputs: Vec::new(),
                binding_input: Fr::from(0u64), communications_commitment: Some((comm, randomness)),
                key_location: KeyLocation(CHECK_KEY_LOCATION.into()) }
        }
    };
    preimage.check(&c.ir).map_err(|e| C6ScalarConstructConsumerError::Backend(
        format!("SDK ProofPreimage::check failed: {e}")))
}
fn assert_raw_ok(w: &C6ScalarConstructStepWitness, s: Vec<Fr>, label: &str) {
    for mode in [C6PushStatementBindingMode::LocalPublic, C6PushStatementBindingMode::LedgerCall] {
        let c = circuit_for(w, mode);
        assert!(check_raw(&c, w, s.clone(), None).is_ok(), "{label}: raw positive {mode:?}");
    }
}
fn assert_raw_rejects(w: &C6ScalarConstructStepWitness, s: Vec<Fr>, label: &str) {
    for mode in [C6PushStatementBindingMode::LocalPublic, C6PushStatementBindingMode::LedgerCall] {
        let c = circuit_for(w, mode);
        match check_raw(&c, w, s.clone(), None) {
            Err(C6ScalarConstructConsumerError::Backend(m)) => assert!(
                m.contains("SDK ProofPreimage::check failed"),
                "{label} {mode:?}: must be SDK preprocessing rejection, got: {m}"),
            other => panic!("{label} {mode:?}: expected SDK rejection, ok={}", other.is_ok()),
        }
    }
}
fn set_both_helper_rel(s: &mut [Fr], w: &C6ScalarConstructStepWitness, rel: usize, v: u128) {
    s[helper_rel(w, false, rel)] = Fr::from(v);
    s[helper_rel(w, true, rel)] = Fr::from(v);
}
fn disabled_statement(w: &C6ScalarConstructStepWitness) -> Vec<Fr> {
    let mut s = w.to_statement_frs(); s[0] = Fr::from(0u64); s[1] = Fr::from(0u64);
    let pre = s[pre_state_start(w)..pre_state_start(w) + state_len(w)].to_vec();
    let at = post_state_start(w); s[at..at + state_len(w)].copy_from_slice(&pre); s
}
fn two_pow_128() -> Fr { Fr::from(MAX) + Fr::from(1u64) }
fn find_const_index(s: &HostSnapshot, want: &str) -> usize {
    s.admission.view.constants.iter().position(|c| match (want, c) {
        ("UInt", ConstantSnapshot::U128 { .. }) => true,
        ("Bool", ConstantSnapshot::Bool { .. }) => true,
        ("Nil", ConstantSnapshot::Nil { .. }) => true,
        ("Leaf", ConstantSnapshot::Leaf { .. }) => true,
        _ => false,
    }).unwrap_or_else(|| panic!("genuine program must contain a {want} Const row"))
}

#[test]
fn raw_genuine_three_shapes_both_modes() {
    let cases: Vec<Applied> = vec![
        leaf_witness(),
        run_core(&leaf(KindWant::UInt(MAX), pending(NodeKind::Leaf)), "Leaf MAX",
            &|| prog(unary(UnaryOp::Leaf, lit_u(MAX)))),
        run_core(&leaf(KindWant::Bool(true), err11()), "Leaf Bool err11",
            &|| prog(unary(UnaryOp::Leaf, lit_b(true)))),
        cons_witness(),
        run_core(&cons(KindWant::UInt(7), KindWant::Cons, pending(NodeKind::Cons)), "Cons Cons",
            &|| prog(bin(BinaryOp::Cons, lit_u(7), lit_list(&[1])))),
        run_core(&cons(KindWant::UInt(7), KindWant::Bool(true), err11()), "Cons Bool err11",
            &|| prog(bin(BinaryOp::Cons, lit_u(7), lit_b(true)))),
        run_core(&Spec { a_data: Some(true),
            ..cons(KindWant::AnyUInt, KindWant::Nil, pending(NodeKind::Cons)) }, "Cons Data A",
            &|| prog(bin(BinaryOp::Cons, bin(BinaryOp::Add, lit_u(1), lit_u(2)), lit_list(&[])))),
        branch_witness(),
        run_core(&branch(KindWant::Branch, KindWant::Branch, pending(NodeKind::Branch)), "Branch Branch",
            &|| prog(bin(BinaryOp::Branch,
                lit_tree(CoreTree::Branch { left: Box::new(tree_leaf(1)), right: Box::new(tree_leaf(2)) }),
                lit_tree(CoreTree::Branch { left: Box::new(tree_leaf(3)), right: Box::new(tree_leaf(4)) })))),
        run_core(&branch(KindWant::Leaf, KindWant::UInt(3), err11()), "Branch UInt err11",
            &|| prog(bin(BinaryOp::Branch, lit_tree(tree_leaf(5)), lit_u(3)))),
        run_ext(&leaf(KindWant::Ctor, err11()), "Leaf nominal Ctor err11",
            &nominal_leaf_ctor_program),
    ];
    for (i, a) in cases.iter().enumerate() {
        assert_ok_both(&a.witness, &format!("genuine case {i}"));
        assert_raw_ok(&a.witness, a.witness.to_statement_frs(), &format!("genuine raw case {i}"));
    }
}

#[test]
fn raw_wrong_post_phase_staged_nodekind_errorcode() {
    for (a, good_payload, label) in [
        (leaf_witness(), 1u128, "Leaf"), (cons_witness(), 0, "Cons"), (branch_witness(), 2, "Branch")] {
        let w = &a.witness; let base = w.to_statement_frs();
        assert_raw_ok(w, base.clone(), label);
        for (rel, v, what) in [(13usize, 1u128, "post phase Second"), (13, 0, "post phase First"),
            (15, 4, "post staged PrimError"), (15, 0, "post staged None"),
            (16, good_payload ^ 1, "wrong NodeKind payload"), (16, 11, "payload 11 on PendingNode")] {
            let mut s = base.clone(); s[helper_rel(w, true, rel)] = Fr::from(v);
            assert_raw_rejects(w, s, &format!("{label}: {what}"));
        }
    }
    let a = run_core(&leaf(KindWant::Bool(true), err11()), "Leaf err base",
        &|| prog(unary(UnaryOp::Leaf, lit_b(true))));
    let w = &a.witness; let base = w.to_statement_frs();
    for v in [10u128, 0, 1, 12] {
        let mut s = base.clone(); s[helper_rel(w, true, 16)] = Fr::from(v);
        assert_raw_rejects(w, s, "wrong PrimError code");
    }
}

#[test]
fn raw_wrong_post_common_refs_rega_expected_parent_result_tasks() {
    let a = cons_witness(); let w = &a.witness; let base = w.to_statement_frs();
    assert_raw_ok(w, base.clone(), "common control");
    // rel: 0 kind,1 site,2 origin,3 parent present,5 mode,6 A is_data,7 A idx,
    // 8 B present,9 B is_data,10 B idx,11 expected present,14 regA
    for (rel, what) in [(0usize, "kind"), (1, "site"), (2, "origin"), (3, "parent_resume"),
        (5, "mode"), (6, "A space"), (7, "A index"), (8, "B presence"), (9, "B space"),
        (10, "B index"), (11, "expected"), (14, "regA")] {
        let mut s = base.clone();
        let at = helper_rel(w, true, rel);
        s[at] = if s[at] == Fr::from(0u64) { Fr::from(1u64) } else { Fr::from(0u64) };
        assert_raw_rejects(w, s, &format!("wrong post helper {what}"));
    }
    let mut s = base.clone();
    s[post_helper_at(w, L_RESULT_RECORD)] = Fr::from(10u64);
    assert_raw_rejects(w, s, "wrong post result present");
    let mut s = base.clone();
    s[post_helper_at(w, 3)] = Fr::from(1u64); // helper header task count lane
    assert_raw_rejects(w, s, "wrong post task count");
    // Same-index distinct Ref space on post A alone is an identity break.
    let mut s = base;
    let at = helper_rel(w, true, 6);
    s[at] = Fr::from(1u64);
    assert_raw_rejects(w, s, "post A Const/Data same-index space toggle");
}

#[test]
fn raw_wrong_post_counters_outcome_preservation() {
    let a = leaf_witness(); let w = &a.witness; let base = w.to_statement_frs();
    for (lane, what) in [(L_REMAINING_WORK, "remaining"), (L_CONSUMED_WORK, "consumed"),
        (L_HELPER_WORK, "helper_work"), (L_HELPER_VISITS, "visits"),
        (L_REFERENCE_CURSOR, "ref cursor must not move"), (L_OUTCOME, "outcome tag"),
        (L_OUTCOME + 1, "outcome payload"), (L_CONTROL, "control preservation")] {
        let mut s = base.clone();
        let at = post_image_at(w, lane);
        s[at] = s[at] + Fr::from(1u64);
        assert_raw_rejects(w, s, &format!("wrong post {what}"));
    }
    // Old active DATA vs post-only mutation: labelled preservation, not padding.
    let mut s = base.clone();
    s[post_data_at(w, 3)] = s[post_data_at(w, 3)] + Fr::from(1u64);
    assert_raw_rejects(w, s, "wrong post DATA active count (preservation)");
    let mut s = base;
    s[post_meta_at(w, 3)] = Fr::from(9u64);
    assert_raw_rejects(w, s, "wrong post META lane (prefix preservation)");
}

#[test]
fn raw_changed_pre_with_coherent_post_companions() {
    let a = cons_witness(); let w = &a.witness; let base = w.to_statement_frs();
    // Each: pre and post common lane changed together, so preservation
    // alone cannot mask the targeted PRE premise rejection.
    for (rel, v, what) in [(14usize, 1u128, "pre regA=1"), (11, 1, "pre expected present"),
        (3, 1, "pre parent_resume present"), (8, 0, "pre B absent for Cons"),
        (5, 0, "pre mode Add"), (0, 2, "pre helper kind not Scalar")] {
        let mut s = base.clone(); set_both_helper_rel(&mut s, w, rel, v);
        assert_raw_rejects(w, s, what);
    }
    // Pre phase: post stays the genuine Done post; only PRE pairing is wrong.
    for v in [0u128, 2] {
        let mut s = base.clone(); s[helper_rel(w, false, 13)] = Fr::from(v);
        assert_raw_rejects(w, s, "pre phase not Second for Cons");
    }
    // Pre staged not None, with post unchanged.
    let mut s = base.clone(); s[helper_rel(w, false, 15)] = Fr::from(4u64);
    s[helper_rel(w, false, 16)] = Fr::from(11u64);
    assert_raw_rejects(w, s, "pre staged not None");
    // Pre result / tasks present, coherently in pre only (post must stay absent).
    let mut s = base; s[pre_helper_at(w, L_RESULT_RECORD)] = Fr::from(10u64);
    assert_raw_rejects(w, s, "pre result present");
}

#[test]
fn raw_invalid_si22_second_a_domain_not_bare_staging() {
    // Cons: point A (pre+post, preserved) at B's Nil row. Bare B-only
    // staging would still produce PendingNode{Cons}; valid SI22
    // requires A UInt, so the stronger relation must reject.
    let a = cons_witness(); let w = &a.witness; let base = w.to_statement_frs();
    let b_idx = a.record.before.image.helper.and_then(|h| match h {
        HelperControlC::Scalar { operand_b: Some(RefC::Const(i)), .. } => Some(i), _ => None })
        .expect("Cons B must be Const Nil in this fixture");
    let mut s = base.clone(); set_both_helper_rel(&mut s, w, 7, b_idx as u128);
    assert_raw_rejects(w, s, "Second/Cons A=Nil invalid SI22, B valid");
    // Branch: point A at a genuine UInt Const row present inside the
    // tree constants; B stays Leaf. Bare staging would succeed.
    let a = branch_witness(); let w = &a.witness; let base = w.to_statement_frs();
    let u_idx = find_const_index(&a.record.before, "UInt");
    let mut s = base; set_both_helper_rel(&mut s, w, 7, u_idx as u128);
    set_both_helper_rel(&mut s, w, 6, 0);
    assert_raw_rejects(w, s, "Second/Branch A=UInt invalid SI22, B valid");
}

#[test]
fn raw_ref_strict_counts_spaces_and_padding() {
    let a = cons_witness(); let w = &a.witness; let base = w.to_statement_frs();
    let const_count = a.record.before.admission.view.constants.len() as u128;
    // Equal-to-count is outside strict membership (not PrimError11).
    let mut s = base.clone(); set_both_helper_rel(&mut s, w, 10, const_count);
    assert_raw_rejects(w, s, "B index == const_count");
    // Same numeric index, Data space: distinct Ref, outside/at active DATA bound here.
    let mut s = base.clone(); set_both_helper_rel(&mut s, w, 9, 1);
    assert_raw_rejects(w, s, "B same index in Data space");
    // Inactive DATA partition: first inactive data-row lane, pre+post
    // coherently nonzero. This is canonical-padding rejection, truthfully
    // distinct from an old-active-DATA preservation mutation above.
    let geom = w.source_params.geometry();
    let active = w.pre_data[3] as usize;
    assert!(active < geom.caps.phys.max_data_nodes, "fixture needs an inactive DATA slot");
    let lane = 7 + active * 5;
    let mut s = base;
    s[pre_data_at(w, lane)] = Fr::from(1u64); s[post_data_at(w, lane)] = Fr::from(1u64);
    assert_raw_rejects(w, s, "nonzero inactive DATA padding");
}

#[test]
fn raw_fr_ge_2pow128_gate2_gate0_and_disabled() {
    let a = leaf_witness(); let w = &a.witness; let base = w.to_statement_frs();
    assert_raw_ok(w, disabled_statement(w), "disabled identity positive");
    let mut s = disabled_statement(w);
    s[post_image_at(w, L_HELPER_VISITS)] = s[post_image_at(w, L_HELPER_VISITS)] + Fr::from(1u64);
    assert_raw_rejects(w, s, "disabled wrong post");
    let big = two_pow_128();
    for (at, what) in [
        (pre_image_at(w, 26), "pre image Fr>=2^128"), (post_image_at(w, 26), "post image Fr>=2^128"),
        (helper_rel(w, false, 14), "pre helper Fr>=2^128"), (helper_rel(w, true, 14), "post helper Fr>=2^128"),
        (pre_data_at(w, 3), "pre DATA Fr>=2^128"), (post_data_at(w, 3), "post DATA Fr>=2^128"),
        (pre_ctx_at(w, 20), "pre CTX Fr>=2^128"), (post_ctx_at(w, 20), "post CTX Fr>=2^128"),
        (pre_prog_at(w, 3), "pre PROG Fr>=2^128"), (pre_meta_at(w, 3), "pre META Fr>=2^128")] {
        let mut s = base.clone(); s[at] = big;
        assert_raw_rejects(w, s, what);
    }
    // gate 2 is not boolean; gate 0 still range-constrains lanes at 128 bits.
    let mut s = base.clone(); s[0] = Fr::from(2u64);
    assert_raw_rejects(w, s, "enable=2");
    let mut s = disabled_statement(w); s[pre_image_at(w, 26)] = big; s[post_image_at(w, 26)] = big;
    assert_raw_rejects(w, s, "gate0 still raw 128-bit range");
    let mut s = base; s[1] = Fr::from(1u64);
    assert_raw_rejects(w, s, "wrong disposition lane on applied");
    let mut s = w.to_statement_frs(); s[2] = Fr::from(9u64);
    assert_raw_rejects(w, s, "wrong host_schema");
    let mut s = w.to_statement_frs(); s[3] = Fr::from(9u64);
    assert_raw_rejects(w, s, "wrong carrier_version");
}

#[test]
fn raw_p3_genuine_and_near_max_counters() {
    let wanted = leaf(KindWant::UInt(5), pending(NodeKind::Leaf));
    let make = || prog(unary(UnaryOp::Leaf, lit_u(5)));
    let (_, _, pre0) = reach_core(bounds(100), &wanted, "P3 probe", &make);
    let hw = pre0.image.helper_work;
    assert!(hw > 0 && hw < 100, "probe helper_work must be usable as exact bound");
    let b = bounds(hw);
    let (admitted, mut image, pre) = reach_core(b, &wanted, "P3 exact", &make);
    assert_eq!(pre.image.helper_work, hw);
    let record = match owner_record(&admitted, &mut image) {
        Ok(ObservedStep::Event(r)) => r,
        Ok(ObservedStep::Terminal { .. }) => panic!("P3 selected step must be event"),
        Err(e) => panic!("P3 step: {}", capture_step_error_label(&e)),
    };
    assert_eq!(record.event.disposition, StepDisposition::RefusedHelperWorkUncharged, "P3 disposition");
    assert_eq!(record.event.exhausted_kind, Some(ExhaustReason::HelperWork), "P3 reason");
    assert_eq!(record.after.image.helper, record.before.image.helper, "P3 helper preserved");
    assert_eq!(record.after.image.consumed_work, record.before.image.consumed_work, "P3 uncharged");
    assert_eq!(record.after.image.outcome,
        OutcomeC::Exhausted { reason: ExhaustReason::HelperWork }, "P3 outcome");
    let w = witness_from_owner_record(&record, &core_cfg(b), &limits()).expect("P3 witness");
    assert!(w.disposition_is_refused_helper_work, "P3 flag from original admission bound");
    assert_ok_both(&w, "genuine P3");
    assert_raw_ok(&w, w.to_statement_frs(), "genuine P3 raw");
    // Near-max visits on an applied base: coherent pre=MAX, post=MAX
    // cannot be the required +1 without wrap; must reject, not wrap.
    let a = leaf_witness(); let w = &a.witness; let base = w.to_statement_frs();
    let mut s = base.clone();
    s[pre_image_at(w, L_HELPER_VISITS)] = Fr::from(MAX);
    s[post_image_at(w, L_HELPER_VISITS)] = Fr::from(MAX);
    assert_raw_rejects(w, s, "applied visits at u128::MAX (no wrap)");
    let mut s = base;
    s[pre_image_at(w, L_HELPER_WORK)] = Fr::from(MAX);
    s[post_image_at(w, L_HELPER_WORK)] = Fr::from(MAX);
    assert_raw_rejects(w, s, "applied helper_work at u128::MAX");
}

#[test]
fn raw_live_call_frame_control_env_slot_preservation() {
    let make = || CoreProgram {
        functions: vec![CoreFunction { name: "f".into(),
            parameters: vec![CoreParameter { name: "a".into(), sort: SortTag::UInt },
                             CoreParameter { name: "b".into(), sort: SortTag::List }],
            return_type: SortTag::List,
            body: bin(BinaryOp::Cons, CoreExpr::Var { name: "a".into() },
                      CoreExpr::Var { name: "b".into() }) }],
        entry: CoreExpr::Call { name: "f".into(), args: vec![lit_u(7), lit_list(&[])] },
    };
    let a = run_core(&cons(KindWant::UInt(7), KindWant::Nil, pending(NodeKind::Cons)),
        "live call Cons", &make);
    let (w, r) = (&a.witness, &a.record);
    assert!(!r.before.image.frames.is_empty() && !r.before.image.env.is_empty()
        && !r.before.image.value_slots.is_empty(), "live fixture needs frame/env/slots");
    assert_ok_both(w, "live control");
    let base = w.to_statement_frs();
    let geom = w.source_params.geometry();
    // First physical frame slot lane (post only).
    let frame_lane = 48usize;
    let mut s = base.clone(); s[post_image_at(w, frame_lane)] =
        s[post_image_at(w, frame_lane)] + Fr::from(1u64);
    assert_raw_rejects(w, s, "wrong post live frame slot");
    // Live env cell 0 binder lane and value-slot 0 index lane (post only).
    let env_lane = geom.env_base();
    let mut s = base.clone(); s[post_data_at(w, env_lane)] =
        s[post_data_at(w, env_lane)] + Fr::from(1u64);
    assert_raw_rejects(w, s, "wrong post live env cell");
    let slot_lane = geom.value_slots_base() + 1;
    let mut s = base; s[post_data_at(w, slot_lane)] =
        s[post_data_at(w, slot_lane)] + Fr::from(1u64);
    assert_raw_rejects(w, s, "wrong post live value slot");
}

#[test]
fn raw_ledger_stale_cc_only_isolated() {
    let a = leaf_witness(); let w = &a.witness; let base = w.to_statement_frs();
    let c = circuit_for(w, C6PushStatementBindingMode::LedgerCall);
    let randomness = Fr::from(0u64);
    let mut stale_inputs = base.clone();
    stale_inputs.extend(base[post_state_start(w)..].iter().copied());
    let stale = transient_commit(&stale_inputs, randomness);
    let mut mutated = base.clone();
    mutated[pre_image_at(w, 26)] = Fr::from(MAX);
    mutated[post_image_at(w, 26)] = Fr::from(MAX);
    let mut current_inputs = mutated.clone();
    current_inputs.extend(mutated[post_state_start(w)..].iter().copied());
    assert_ne!(transient_commit(&current_inputs, randomness), stale, "CC premise");
    assert!(check_raw(&c, w, mutated.clone(), None).is_ok(),
        "paired positive with CURRENT recomputed CC must pass");
    match check_raw(&c, w, mutated, Some(stale)) {
        Err(C6ScalarConstructConsumerError::Backend(m)) => assert!(
            m.contains("Communications commitment mismatch"), "stale CC category: {m}"),
        other => panic!("stale CC: expected rejection, ok={}", other.is_ok()),
    }
}

#[test]
fn raw_producer_and_typed_checker_errors_are_separate_categories() {
    let a = leaf_witness(); let w = a.witness.clone();
    // Host-pair prefix refusal is a producer error, never an SDK check.
    let mut bad_after = a.record.after.clone();
    bad_after.admission.bounds.frames += 1;
    match witness_from_host_pair(true, &a.record.before, &bad_after, &core_cfg(bounds(100)), &limits()) {
        Err(C6ScalarConstructConsumerError::HostPrefixMismatch { section: "meta" }) => {}
        Err(e) => panic!("expected meta HostPrefixMismatch (producer), got other error: {e}"),
        Ok(_) => panic!("expected meta HostPrefixMismatch"),
    }
    // Truncated witness sections are typed length refusals in the checker
    // wrapper, distinct from Backend SDK preprocessing rejection.
    let mut short = w.clone(); short.pre_image.pop();
    let c = circuit_for(&w, C6PushStatementBindingMode::LocalPublic);
    match check_c6_scalar_construct_step_circuit(&c, &short) {
        Err(C6ScalarConstructConsumerError::LengthMismatch { section: "statement", .. }) => {}
        other => panic!("expected statement LengthMismatch, ok={}", other.is_ok()),
    }
    // Wrong-post that still encodes reaches the SDK checker (Backend
    // category above), it is not refused by the host-pair producer.
    let mut wrong_after = a.record.after.clone();
    if let Some(HelperControlC::Scalar { staged, .. }) = &mut wrong_after.image.helper {
        *staged = err11();
    }
    let wrong_w = witness_from_host_pair(true, &a.record.before, &wrong_after,
        &core_cfg(bounds(100)), &limits())
        .expect("encodable wrong post must not be producer-filtered");
    let c = circuit_for(&wrong_w, C6PushStatementBindingMode::LocalPublic);
    match check_c6_scalar_construct_step_circuit(&c, &wrong_w) {
        Err(C6ScalarConstructConsumerError::Backend(m)) => assert!(
            m.contains("SDK ProofPreimage::check failed"), "SDK category: {m}"),
        other => panic!("wrong post must reach SDK checker, ok={}", other.is_ok()),
    }
}

const SEGMENTS: [&str; 8] = [
    ".local.temp.", ".c6data.temp.", ".c6helper.temp.", ".c6meta.temp.",
    ".c6progctx.temp.", ".constructread.local.temp.",
    ".constructread.c6progctx.temp.", ".constructread.c6data.temp.",
];
fn imm(v: u128) -> Operand { Operand::Immediate(Fr::from(v)) }
fn valid_inputs(w: &C6ScalarConstructStepWitness) -> C6ScalarConstructStepInputs {
    let hp = &w.host_prefix;
    let ops = |xs: &[u128]| xs.iter().map(|&v| imm(v)).collect::<Vec<_>>();
    let ctx: [Operand; CTX_LANES] = hp.ctx.iter().map(|&v| imm(v))
        .collect::<Vec<_>>().try_into().expect("ctx lanes");
    C6ScalarConstructStepInputs {
        enable: imm(1),
        disposition_is_refused_helper_work: imm(u128::from(w.disposition_is_refused_helper_work)),
        host_schema: imm(C6_HOST_SCHEMA), carrier_version: imm(C6_PADDED_CARRIER_VERSION),
        geometry: w.source_params.geometry(),
        meta_bounds_frames_offset: hp.meta_bounds_frames_offset,
        source_params: w.source_params.clone(),
        pre_meta: ops(&hp.meta), pre_prog: ops(&hp.prog), pre_ctx: ctx.clone(),
        pre_image: ops(&w.pre_image), pre_data: ops(&w.pre_data), pre_helper: ops(&w.pre_helper),
        post_meta: ops(&hp.meta), post_prog: ops(&hp.prog), post_ctx: ctx,
        post_image: ops(&w.post_image), post_data: ops(&w.post_data), post_helper: ops(&w.post_helper),
    }
}
fn dests(ins: &[Instruction]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for i in ins { match i {
        Instruction::Add { output, .. } | Instruction::Mul { output, .. }
        | Instruction::Neg { output, .. } | Instruction::Not { output, .. }
        | Instruction::CondSelect { output, .. } | Instruction::TestEq { output, .. }
        | Instruction::LessThan { output, .. } | Instruction::Copy { output, .. } =>
            { out.insert(output.0.clone()); }
        Instruction::DivModPowerOfTwo { outputs, .. } =>
            { for o in outputs { out.insert(o.0.clone()); } }
        _ => {}
    } }
    out
}

#[test]
fn raw_namespace_all8_all16_retained_serial_and_free_single_shot() {
    let a = leaf_witness(); let w = &a.witness;
    for seg in SEGMENTS {
        let expected = format!("%constructraw{seg}77");
        let reserved = Operand::Variable(Identifier(expected.clone()));
        for group in 0..16usize {
            let mut inputs = valid_inputs(w);
            match group {
                0 => inputs.enable = reserved.clone(),
                1 => inputs.disposition_is_refused_helper_work = reserved.clone(),
                2 => inputs.host_schema = reserved.clone(),
                3 => inputs.carrier_version = reserved.clone(),
                4 => inputs.pre_meta[0] = reserved.clone(),
                5 => inputs.pre_prog[0] = reserved.clone(),
                6 => inputs.pre_ctx[0] = reserved.clone(),
                7 => inputs.pre_image[0] = reserved.clone(),
                8 => inputs.pre_data[0] = reserved.clone(),
                9 => inputs.pre_helper[0] = reserved.clone(),
                10 => inputs.post_meta[0] = reserved.clone(),
                11 => inputs.post_prog[0] = reserved.clone(),
                12 => inputs.post_ctx[0] = reserved.clone(),
                13 => inputs.post_image[0] = reserved.clone(),
                14 => inputs.post_data[0] = reserved.clone(),
                _ => inputs.post_helper[0] = reserved.clone(),
            }
            let mut em = C6ScalarConstructStepEmitter::new("constructraw").expect("scope");
            match em.emit(&inputs) {
                Err(C6ScalarConstructStepError::ReservedOperand { identifier }) =>
                    assert_eq!(identifier, expected, "segment {seg} group {group}"),
                other => panic!("segment {seg} group {group}: expected ReservedOperand, ok={}", other.is_ok()),
            }
        }
        match C6ScalarConstructStepEmitter::new(&format!("bad{seg}x")) {
            Err(C6ScalarConstructStepError::InvalidScope) => {}
            other => panic!("scope {seg}: expected InvalidScope, ok={}", other.is_ok()),
        }
    }
    match C6ScalarConstructStepEmitter::new("") {
        Err(C6ScalarConstructStepError::EmptyScope) => {}
        other => panic!("empty scope ok={}", other.is_ok()),
    }
    match C6ScalarConstructStepEmitter::new("bad%scope") {
        Err(C6ScalarConstructStepError::InvalidScope) => {}
        other => panic!("percent scope ok={}", other.is_ok()),
    }
    // Retained SAME emitter: serials advance, destinations disjoint.
    let inputs = valid_inputs(w);
    let mut em = C6ScalarConstructStepEmitter::new("constructraw_repeat").expect("repeat scope");
    let first = em.emit(&inputs).expect("first retained emit");
    let after_first = em.next_temp();
    let second = em.emit(&inputs).expect("second retained emit");
    assert!(em.next_temp() > after_first, "retained parent serial advances");
    let (d1, d2) = (dests(&first.instructions), dests(&second.instructions));
    assert!(!d1.is_empty() && !d2.is_empty() && d1.is_disjoint(&d2),
        "retained repeated destinations must be disjoint (no identifier collision)");
    let (r1, r2) = (dests(&first.reader_a.instructions), dests(&second.reader_a.instructions));
    assert!(!r1.is_empty() && r1.is_disjoint(&r2), "reader retained serials disjoint");
    // Free single-shot API is distinct: same scope restarts at serial zero,
    // so its destinations intentionally coincide and must not be composed twice.
    let f1 = emit_c6_scalar_construct_step_block(&inputs, "constructraw_free").expect("free 1");
    let f2 = emit_c6_scalar_construct_step_block(&inputs, "constructraw_free").expect("free 2");
    assert_eq!(dests(&f1.instructions), dests(&f2.instructions),
        "free API is single-shot per scope (documented restart), unlike retained emitter");
    assert!(dests(&f1.instructions).is_disjoint(&d1),
        "different scopes have disjoint destinations");
}

#[test]
fn raw_emitter_length_geometry_producer_errors() {
    let a = leaf_witness(); let w = &a.witness;
    let mut inputs = valid_inputs(w); inputs.pre_image.pop();
    let mut em = C6ScalarConstructStepEmitter::new("constructraw_len").expect("scope");
    match em.emit(&inputs) {
        Err(C6ScalarConstructStepError::LengthMismatch { section: "image", .. }) => {}
        other => panic!("expected image LengthMismatch producer error, ok={}", other.is_ok()),
    }
    let mut inputs = valid_inputs(w); inputs.post_helper.pop();
    let mut em = C6ScalarConstructStepEmitter::new("constructraw_len").expect("scope");
    match em.emit(&inputs) {
        Err(C6ScalarConstructStepError::LengthMismatch { section: "helper", .. }) => {}
        other => panic!("expected helper LengthMismatch producer error, ok={}", other.is_ok()),
    }
}
// --- RAW Marker local relation (intentionally outside SI17) ---
fn raw_marker_statement(a: &Applied, target_idx: usize, is_b: bool,
    staged_tag: u128, payload: u128) -> Vec<Fr> {
    let w = &a.witness;
    let mut s = w.to_statement_frs();
    let (space_rel, idx_rel) = if is_b { (9usize, 10usize) } else { (6usize, 7usize) };
    // Full Ref identity, pre+post coherently: Const space + exact
    // admitted row index already present in genuine PROG. No PROG
    // lane, table framing, or marker_free lane is mutated.
    set_both_helper_rel(&mut s, w, space_rel, 0);
    set_both_helper_rel(&mut s, w, idx_rel, target_idx as u128);
    s[helper_rel(w, true, 15)] = Fr::from(staged_tag);
    s[helper_rel(w, true, 16)] = Fr::from(payload);
    s
}
fn assert_raw_marker_case(a: &Applied, is_b: bool, label: &str) -> Vec<Fr> {
    let w = &a.witness;
    assert_raw_ok(w, w.to_statement_frs(), &format!("{label}: paired genuine positive"));
    let marker_idx = find_marker_const_index(&a.record.before);
    let s = raw_marker_statement(a, marker_idx, is_b, 4, 11);
    // Valid other local premises are the genuine applied ones:
    // Done post phase, applied counters/outcome, old DATA/helper
    // preservation outside phase/staged/effect-ref. Binary A lanes
    // (the stronger SI22 domain operand) are untouched.
    assert_eq!(s[helper_rel(w, true, 13)], Fr::from(2u64), "{label}: post Done");
    assert_eq!(s[pre_image_at(w, L_HELPER_WORK) + 0], w.to_statement_frs()[pre_image_at(w, L_HELPER_WORK)],
        "{label}: pre helper_work is genuine, P3 not faked as Applied");
    assert!(!w.disposition_is_refused_helper_work, "{label}: genuine baseline is Applied, not P3");
    if is_b {
        let base = w.to_statement_frs();
        assert_eq!(s[helper_rel(w, false, 6)], base[helper_rel(w, false, 6)], "{label}: A space preserved");
        assert_eq!(s[helper_rel(w, false, 7)], base[helper_rel(w, false, 7)], "{label}: A index preserved");
    }
    assert_raw_ok(w, s.clone(), &format!("{label}: RAW Marker to Done PrimError11, both modes"));
    s
}
#[test]
fn raw_marker_all_modes_prim_error11_local_relation_both_modes() {
    // Leaf effect operand is A; Cons/Branch effect operand is B.
    // Marker in binary A would instead fail the stronger SI22
    // domain and is deliberately not used as an Error11 positive.
    assert_raw_marker_case(&marker_leaf_witness(), false, "Leaf Marker A");
    assert_raw_marker_case(&marker_cons_witness(), true, "Cons Marker B");
    assert_raw_marker_case(&marker_branch_witness(), true, "Branch Marker B");
}
#[test]
fn raw_marker_wrong_stage_code_phase_rejects_and_stale_cc_isolated() {
    for (a, is_b, pending_payload, label) in [
        (marker_leaf_witness(), false, 1u128, "Leaf Marker"),
        (marker_cons_witness(), true, 0u128, "Cons Marker"),
        (marker_branch_witness(), true, 2u128, "Branch Marker"),
    ] {
        let w = &a.witness;
        let good = assert_raw_marker_case(&a, is_b, label);
        let marker_idx = find_marker_const_index(&a.record.before);
        // Wrong NodeKind staging for a Marker kind.
        assert_raw_rejects(w, raw_marker_statement(&a, marker_idx, is_b, 3, pending_payload),
            &format!("{label}: Marker as PendingNode must reach SDK preprocessing rejection"));
        // Wrong PrimError code.
        assert_raw_rejects(w, raw_marker_statement(&a, marker_idx, is_b, 4, 10),
            &format!("{label}: Marker with code 10"));
        // Wrong post phase (Second), with correct Error11 staging.
        let mut wrong_phase = good.clone();
        wrong_phase[helper_rel(w, true, 13)] = Fr::from(1u64);
        assert_raw_rejects(w, wrong_phase, &format!("{label}: Marker wrong post phase"));
    }
    // Stale CC is separate from the current-statement positives:
    // LedgerCall commitment is recomputed from current statement +
    // current post in assert_raw_ok/check_raw above.
    let a = marker_leaf_witness();
    let w = &a.witness;
    let good = assert_raw_marker_case(&a, false, "Leaf Marker stale-CC pair");
    let base = w.to_statement_frs();
    let c = circuit_for(w, C6PushStatementBindingMode::LedgerCall);
    let randomness = Fr::from(0u64);
    let mut stale_inputs = base.clone();
    stale_inputs.extend(base[post_state_start(w)..].iter().copied());
    let stale = transient_commit(&stale_inputs, randomness);
    assert!(check_raw(&c, w, good.clone(), None).is_ok(),
        "Marker statement with CURRENT recomputed CC must pass");
    match check_raw(&c, w, good, Some(stale)) {
        Err(C6ScalarConstructConsumerError::Backend(m)) => assert!(
            m.contains("Communications commitment mismatch"), "stale CC category: {m}"),
        other => panic!("Marker stale CC: expected rejection, ok={}", other.is_ok()),
    }
}
#[test]
fn raw_transitive_marker_free_false_branch_b_pending_local_only() {
    // A Branch composite containing a Marker has kind Branch and
    // marker_free=false. The local Construct relation stages it as
    // PendingNode Branch; it does not enforce transitive SI17.
    // This is a local-relation observation, not admitted validity.
    let a = marker_branch_witness();
    let w = &a.witness;
    assert_raw_ok(w, w.to_statement_frs(), "transitive Branch paired genuine positive");
    let idx = find_transitive_branch_const_index(&a.record.before);
    let s = raw_marker_statement(&a, idx, true, 3, 2);
    assert_raw_ok(w, s, "RAW transitive marker_free=false Branch B to PendingNode Branch");
}
