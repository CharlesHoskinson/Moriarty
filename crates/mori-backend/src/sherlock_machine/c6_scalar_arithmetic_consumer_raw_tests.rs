//! Independent raw constraint/composition tests for
//! `c6_scalar_arithmetic_consumer`.
//! Source-only and UNRUN. Every positive base is admitted through the
//! real public owner (`host_owner_engine`, plain `admit` for builtin
//! programs) and searched with the real `record_step` until the actual
//! selected Scalar arithmetic state. Mutations are applied to raw
//! `Vec<Fr>` statements only and reach real SDK `ProofPreimage::check`
//! preprocessing in both statement modes. LedgerCall communications
//! commitments are recomputed from the CURRENT mutated statement plus
//! its CURRENT mutated post-flat output on every check; the single
//! stale-commitment case is labelled separately and is never used as a
//! relation-rejection proxy.
//!
//! Deliberately unadmitted raw schema fixtures (held Marker) are built
//! as whole `ProgramSnapshot`s through the real public program codec
//! with roundtrip and exact length checks. Such a fixture is a raw
//! common-domain program only; it is not an owner-admitted held
//! Marker and proves nothing about SI17. Raw rows are RAW DOMAIN,
//! never owner-admitted states.
//!
//! Retained premises not discharged here: SI08/SI17/SI21/SI22 and
//! StepInv as wholes, ScalarEntry installation from the actual top
//! frame, program membership, P1 issuer / SourceSpec authentication.
//! `ProofPreimage::check` is preprocessing only: not synthesis,
//! MockProver, proof, native, degree or performance evidence.

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
use mori_core::sherlock::helper_machine::c6_phys_program::{decode_program, encode_program};
use mori_core::sherlock::helper_machine::complete_observation::{
    capture, record_step as owner_record, CaptureStepError, CompleteStepRecord,
    ConstantSnapshot, DataSnapshot, HostSnapshot, ObservedStep,
};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::*;

use super::{
    build_c6_scalar_arithmetic_step_circuit, check_c6_scalar_arithmetic_step_circuit,
    witness_from_host_pair, witness_from_owner_record, C6ScalarArithmeticConsumerError,
    C6ScalarArithmeticStepCircuit, C6ScalarArithmeticStepWitness, CHECK_KEY_LOCATION,
};
use super::super::c6_push_step::{
    C6PushFamilyParams, C6PushHostPrefixLanes, C6PushStatementBindingMode,
    C6PushStepError, C6_PADDED_CARRIER_VERSION,
};
use super::super::c6_scalar_arithmetic_step::{
    C6ScalarArithmeticStepEmitter, C6ScalarArithmeticStepError, C6ScalarArithmeticStepInputs,
};

// ---------- genuine-owner fixture admission ----------

fn violation_label(v: &HostViolation) -> String {
    format!("kind={:?}, clause={:?}, rule={:?}, locator={}",
        v.kind, v.clause, v.rule, v.locator.as_deref().unwrap_or("<none>"))
}
fn capture_label(e: &CaptureStepError) -> String {
    match e {
        CaptureStepError::BeforeObservation(v) =>
            format!("before observation: {}", violation_label(v)),
        CaptureStepError::Step { violation, after, .. } => match after {
            Ok(_) => format!("owner step: {}; after snapshot captured", violation_label(violation)),
            Err(av) => format!("owner step: {}; after observation: {}",
                violation_label(violation), violation_label(av)),
        },
        CaptureStepError::AfterObservation { violation, event, .. } => format!(
            "after observation: {}; event_present={}", violation_label(violation), event.is_some()),
    }
}
fn tooling_label(e: &Tooling) -> String {
    match e {
        Tooling::AdmissionRejected { reason, detail, locator } => format!(
            "admission rejected: reason={reason:?}, detail={detail}, locator={locator:?}"),
        Tooling::CapacityNotAdmitted { kind, needed, limit, locator } => format!(
            "capacity not admitted: kind={kind:?}, needed={needed:?}, limit={limit}, locator={locator:?}"),
        Tooling::UnsupportedFeature { feature, locator } => format!(
            "unsupported feature: feature={feature:?}, locator={locator:?}"),
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
fn capacities() -> LayoutCapacitiesC {
    LayoutCapacitiesC { name_rows: 32, name_bytes: 512, occurrence_path_slots: 64, value_slots: 512 }
}
fn layout(b: BoundsC) -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("lv".into(), "1".into()), field_id: None, widths: None,
        scalar_layout_id: None, bounds: b, capacities: capacities(),
        ext_capacities: None, boundary_capacities_present: false,
    }
}
fn core_cfg(b: BoundsC) -> C6HostConfig {
    C6HostConfig::from_retained_layout(&layout(b),
        C6CodecBudgets { type_expr_nodes: 256, template_bytes: 4096 })
}
fn limits() -> C6HostLimits {
    C6HostLimits { max_total_lanes: 1 << 20, max_state_lanes: 1 << 20,
        max_metadata_bytes: 8192, max_string_bytes: 512, max_width_entries: 64 }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KindWant { UInt(u128), Bool(bool), Nil, Cons, Leaf, Branch, Marker }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Spec { mode: ScalarMode, a: KindWant, b: Option<KindWant> }
fn spec(mode: ScalarMode, a: KindWant, b: Option<KindWant>) -> Spec { Spec { mode, a, b } }
fn is_numeric(m: ScalarMode) -> bool {
    matches!(m, ScalarMode::Add | ScalarMode::Sub | ScalarMode::Mul | ScalarMode::Lt)
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
            Some(ConstantSnapshot::Constructor { .. }) => panic!("Ctor not an arithmetic fixture kind"),
            Some(ConstantSnapshot::U128OutOfRange { .. }) => KindWant::Marker,
            None => panic!("selected Const operand must exist"),
        },
        RefC::Data(id) => match s.image.data.get(id) {
            Some(DataSnapshot::U128 { n }) => KindWant::UInt(*n),
            Some(DataSnapshot::Bool { b }) => KindWant::Bool(*b),
            Some(DataSnapshot::Nil) => KindWant::Nil,
            Some(DataSnapshot::Cons { .. }) => KindWant::Cons,
            Some(DataSnapshot::Leaf { .. }) => KindWant::Leaf,
            Some(DataSnapshot::Branch { .. }) => KindWant::Branch,
            Some(DataSnapshot::Constructor { .. }) => panic!("Ctor not a fixture kind"),
            None => panic!("selected Data operand must exist"),
        },
    }
}
fn selected_helper(s: &HostSnapshot) -> HelperControlC {
    match s.image.helper { Some(h @ HelperControlC::Scalar { .. }) => h,
        _ => panic!("snapshot must carry a Scalar helper") }
}
fn is_selected(s: &HostSnapshot, w: &Spec) -> bool {
    // Search steps through genuine owner intermediates (genesis
    // `helper=None`, helper-free binary schedule, other helpers):
    // those are non-selected, never a panic. All mode/phase/operand
    // checks below are unchanged.
    let helper = match s.image.helper {
        Some(h @ HelperControlC::Scalar { .. }) => h,
        _ => return false,
    };
    let HelperControlC::Scalar { common, mode, operand_a, operand_b, expected,
        phase, reg_a, staged } = helper else { return false };
    let _ = common;
    if mode != w.mode || staged != ScalarStaged::None || expected.is_some() { return false; }
    if operand_b.is_some() != is_numeric(w.mode) { return false; }
    if w.mode == ScalarMode::Not {
        if phase != ScalarPhase::First || reg_a != 0 { return false; }
    } else {
        if phase != ScalarPhase::Second { return false; }
        // SI22 valid domain: operandA UInt and regA equals its payload.
        if operand_kind(s, operand_a) != KindWant::UInt(reg_a) { return false; }
    }
    if operand_kind(s, operand_a) != w.a { return false; }
    match (w.b, operand_b) {
        (None, None) => true,
        (Some(k), Some(rb)) => operand_kind(s, rb) == k,
        _ => false,
    }
}
fn search(admitted: Admitted, mut image: MachineImage, w: &Spec, label: &str)
    -> (Admitted, MachineImage, HostSnapshot) {
    for _ in 0..512 {
        let step = owner_record(&admitted, &mut image)
            .unwrap_or_else(|e| panic!("{label}: search: {}", capture_label(&e)));
        let after = match &step { ObservedStep::Event(r) => &r.after,
            ObservedStep::Terminal { after, .. } =>
                panic!("{label}: terminal before selected state: {:?}", after.image.outcome) };
        if is_selected(after, w) {
            let sel = capture(&admitted, &image)
                .unwrap_or_else(|e| panic!("{label}: capture: {}", violation_label(&e)));
            assert_eq!(&sel, after, "{label}: capture equals searched pre-state");
            return (admitted, image, sel);
        }
    }
    panic!("{label}: genuine owner never reached the selected arithmetic state")
}
fn reach_core(b: BoundsC, w: &Spec, label: &str, make: &dyn Fn() -> CoreProgram)
    -> (Admitted, MachineImage, HostSnapshot) {
    let engine = host_owner_engine();
    let admitted = engine.admit(&make(), &b, &layout(b))
        .unwrap_or_else(|e| panic!("{label}: admit: {}", tooling_label(&e)));
    let image = engine.admit_run(&admitted, &RunIntC::Nat(64), &RunIntC::Nat(8), &FinanceInput::Absent)
        .unwrap_or_else(|e| panic!("{label}: genesis: {}", tooling_label(&e)));
    search(admitted, image, w, label)
}

/// Independently derived expected staged value: Core checked-u128
/// branch denotation computed in host Rust from the actual operand
/// rows of the genuine pre-state, never copied from post lanes.
fn expected_staged(r: &CompleteStepRecord) -> ScalarStaged {
    let HelperControlC::Scalar { mode, operand_a, operand_b, .. } =
        selected_helper(&r.before) else { unreachable!() };
    match mode {
        ScalarMode::Not => match operand_kind(&r.before, operand_a) {
            KindWant::Bool(b) => ScalarStaged::BoolValue { b: !b },
            _ => ScalarStaged::PrimError { code: 11 },
        },
        ScalarMode::Lt => match operand_b.map(|x| operand_kind(&r.before, x)) {
            Some(KindWant::UInt(n)) => {
                let KindWant::UInt(x) = operand_kind(&r.before, operand_a) else { unreachable!() };
                ScalarStaged::BoolValue { b: x < n }
            }
            _ => ScalarStaged::PrimError { code: 11 },
        },
        ScalarMode::Add | ScalarMode::Sub | ScalarMode::Mul => {
            match operand_b.map(|x| operand_kind(&r.before, x)) {
                Some(KindWant::UInt(n)) => {
                    let KindWant::UInt(x) = operand_kind(&r.before, operand_a) else { unreachable!() };
                    let v = match mode { ScalarMode::Add => x.checked_add(n),
                        ScalarMode::Sub => x.checked_sub(n), _ => x.checked_mul(n) };
                    match v { Some(v) => ScalarStaged::U128Value { n: v },
                        None => ScalarStaged::PrimError { code: 10 } }
                }
                _ => ScalarStaged::PrimError { code: 11 },
            }
        }
        _ => panic!("not a five-mode fixture"),
    }
}
fn assert_applied(r: &CompleteStepRecord, w: &Spec, label: &str) {
    let (pre, post, e) = (&r.before.image, &r.after.image, &r.event);
    assert!(is_selected(&r.before, w), "{label}: selected pre");
    assert_eq!(e.rule_id, RuleIdC::BHelperScalarArithmetic, "{label}: rule");
    assert_eq!(e.event_kind, EventKind::HelperVisit, "{label}: kind");
    assert_eq!(e.helper_kind, Some(HelperKind::Scalar), "{label}: helper kind");
    assert_eq!(e.disposition, StepDisposition::Applied, "{label}: disposition");
    assert_eq!((e.work_delta, e.helper_work_delta, e.helper_visits_delta), (1, 1, 1), "{label}: deltas");
    assert_eq!(e.reference_cursor_delta, 0, "{label}: cursor delta");
    assert_eq!(e.alloc_deltas, AllocDeltasC { data_nodes: 0, data_fields: 0, environment_cells: 0 }, "{label}: alloc");
    assert_eq!(e.value_slot_writes, 0, "{label}: slot writes");
    assert!(e.result_tag.is_none() && e.exhausted_kind.is_none(), "{label}: tags");
    assert_eq!(e.outcome_payload, OutcomeC::Running, "{label}: outcome");
    assert_eq!(e.boundary, BoundaryKind::None, "{label}: boundary");
    assert_eq!(e.step_id.ordinal, pre.consumed_work, "{label}: ordinal");
    assert_eq!(*e.step_id.run_id, pre.run_id, "{label}: run id");
    assert_eq!(e.helper_cursor_before, Some(if w.mode == ScalarMode::Not { 0 } else { 1 }), "{label}: cursor before");
    assert_eq!(e.helper_phase_after, Some(HelperPhaseSummaryC::Scalar(ScalarPhase::Done)), "{label}: phase after");
    assert_eq!(e.helper_cursor_after, Some(2), "{label}: cursor after");
    assert_eq!(post.remaining_work, pre.remaining_work - 1, "{label}: remaining");
    assert_eq!(post.consumed_work, pre.consumed_work + 1, "{label}: consumed");
    assert_eq!(post.helper_work, pre.helper_work + 1, "{label}: helper work");
    assert_eq!(post.helper_visits, pre.helper_visits + 1, "{label}: visits");
    assert_eq!(post.reference_cursor, pre.reference_cursor, "{label}: ref cursor");
    // Preservation of every unaffected section.
    assert_eq!(r.after.admission, r.before.admission, "{label}: admission");
    assert_eq!(post.control, pre.control, "{label}: control");
    assert_eq!(post.control_origin, pre.control_origin, "{label}: origin");
    assert_eq!(post.env_root, pre.env_root, "{label}: env root");
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
        *phase = ScalarPhase::Done; *staged = expected_staged(r);
    }
    assert_eq!(post.helper, Some(want), "{label}: complete post helper (regA preserved)");
    assert!(pre.result.is_none() && post.result.is_none(), "{label}: result absent");
}

struct Applied { record: CompleteStepRecord, witness: C6ScalarArithmeticStepWitness }
fn run_core(w: &Spec, label: &str, make: &dyn Fn() -> CoreProgram) -> Applied {
    let b = bounds(100);
    let (admitted, mut image, pre) = reach_core(b, w, label, make);
    let record = match owner_record(&admitted, &mut image) {
        Ok(ObservedStep::Event(r)) => r,
        Ok(ObservedStep::Terminal { .. }) => panic!("{label}: selected step must be an event"),
        Err(e) => panic!("{label}: selected step: {}", capture_label(&e)),
    };
    assert_eq!(record.before, pre, "{label}: record pre equals searched pre");
    assert_applied(&record, w, label);
    let witness = witness_from_owner_record(&record, &core_cfg(b), &limits())
        .unwrap_or_else(|e| panic!("{label}: owner-record witness: {e}"));
    let hp = witness_from_host_pair(true, &record.before, &record.after, &core_cfg(b), &limits())
        .unwrap_or_else(|e| panic!("{label}: host-pair witness: {e}"));
    assert_eq!(hp.to_statement_frs(), witness.to_statement_frs(), "{label}: routes agree");
    assert!(witness.enable() && !witness.disposition_is_refused_helper_work(), "{label}: flags");
    Applied { record, witness }
}

fn lit_u(n: u128) -> CoreExpr { CoreExpr::Literal { value: CoreValue::UInt { n: i128_class::KIntC::InRange(n) } } }
fn lit_b(v: bool) -> CoreExpr { CoreExpr::Literal { value: CoreValue::Bool { b: v } } }
fn bin(op: BinaryOp, l: CoreExpr, r: CoreExpr) -> CoreExpr {
    CoreExpr::Binary { op, left: Box::new(l), right: Box::new(r) } }
fn prog(entry: CoreExpr) -> CoreProgram { CoreProgram { functions: Vec::new(), entry } }
const MAX: u128 = u128::MAX;

fn add_witness() -> Applied {
    run_core(&spec(ScalarMode::Add, KindWant::UInt(7), Some(KindWant::UInt(3))), "base Add 7+3",
        &|| prog(bin(BinaryOp::Add, lit_u(7), lit_u(3))))
}
fn not_witness() -> Applied {
    run_core(&spec(ScalarMode::Not, KindWant::Bool(true), None), "base Not true",
        &|| CoreProgram { functions: Vec::new(), entry: CoreExpr::Unary {
            op: UnaryOp::Not, operand: Box::new(lit_b(true)) } })
}

// ---------- raw statement harness (both modes, recomputed CC) ----------

fn circuit_for(w: &C6ScalarArithmeticStepWitness, mode: C6PushStatementBindingMode)
    -> C6ScalarArithmeticStepCircuit {
    build_c6_scalar_arithmetic_step_circuit(w.source_params().clone(),
        w.host_prefix().meta.len(), w.host_prefix().prog.len(),
        w.host_prefix().meta_bounds_frames_offset, mode, "scalar_arith_raw_test")
        .expect("circuit builds")
}
fn assert_ok_both(w: &C6ScalarArithmeticStepWitness, label: &str) {
    for mode in [C6PushStatementBindingMode::LocalPublic, C6PushStatementBindingMode::LedgerCall] {
        let c = circuit_for(w, mode);
        assert!(check_c6_scalar_arithmetic_step_circuit(&c, w).is_ok(),
            "{label}: typed positive must pass in {mode:?}");
    }
}
fn prefix_len(w: &C6ScalarArithmeticStepWitness) -> usize {
    w.host_prefix().meta.len() + w.host_prefix().prog.len() + CTX_LANES }
fn state_len(w: &C6ScalarArithmeticStepWitness) -> usize {
    w.pre_image().len() + w.pre_data().len() + w.pre_helper().len() }
fn pre_state_start(w: &C6ScalarArithmeticStepWitness) -> usize { 4 + prefix_len(w) }
fn post_prefix_start(w: &C6ScalarArithmeticStepWitness) -> usize { pre_state_start(w) + state_len(w) }
fn post_state_start(w: &C6ScalarArithmeticStepWitness) -> usize { post_prefix_start(w) + prefix_len(w) }
fn pre_prog_at(w: &C6ScalarArithmeticStepWitness, i: usize) -> usize { 4 + w.host_prefix().meta.len() + i }
fn post_prog_at(w: &C6ScalarArithmeticStepWitness, i: usize) -> usize {
    post_prefix_start(w) + w.host_prefix().meta.len() + i }
fn pre_meta_at(w: &C6ScalarArithmeticStepWitness, i: usize) -> usize { 4 + i }
fn post_meta_at(w: &C6ScalarArithmeticStepWitness, i: usize) -> usize { post_prefix_start(w) + i }
fn pre_ctx_at(w: &C6ScalarArithmeticStepWitness, i: usize) -> usize {
    4 + w.host_prefix().meta.len() + w.host_prefix().prog.len() + i }
fn post_ctx_at(w: &C6ScalarArithmeticStepWitness, i: usize) -> usize {
    post_prefix_start(w) + w.host_prefix().meta.len() + w.host_prefix().prog.len() + i }
fn pre_image_at(w: &C6ScalarArithmeticStepWitness, l: usize) -> usize { pre_state_start(w) + l }
fn post_image_at(w: &C6ScalarArithmeticStepWitness, l: usize) -> usize { post_state_start(w) + l }
fn post_data_at(w: &C6ScalarArithmeticStepWitness, l: usize) -> usize {
    post_state_start(w) + w.pre_image().len() + l }
fn pre_helper_at(w: &C6ScalarArithmeticStepWitness, rel: usize) -> usize {
    pre_state_start(w) + w.pre_image().len() + w.pre_data().len() + L_HELPER_RECORD + rel }
fn post_helper_at(w: &C6ScalarArithmeticStepWitness, rel: usize) -> usize {
    post_state_start(w) + w.pre_image().len() + w.pre_data().len() + L_HELPER_RECORD + rel }
fn pre_result_at(w: &C6ScalarArithmeticStepWitness, rel: usize) -> usize {
    pre_state_start(w) + w.pre_image().len() + w.pre_data().len() + L_RESULT_RECORD + rel }
fn post_result_at(w: &C6ScalarArithmeticStepWitness, rel: usize) -> usize {
    post_state_start(w) + w.pre_image().len() + w.pre_data().len() + L_RESULT_RECORD + rel }

fn check_raw(c: &C6ScalarArithmeticStepCircuit, w: &C6ScalarArithmeticStepWitness,
    statement: Vec<Fr>, comm_override: Option<Fr>)
    -> Result<Vec<Option<usize>>, C6ScalarArithmeticConsumerError> {
    // Child module: private `mode`/`ir` of the parent circuit are visible here.
    let preimage = match c.mode {
        C6PushStatementBindingMode::LocalPublic => ProofPreimage {
            inputs: Vec::new(), private_transcript: Vec::new(),
            public_transcript_inputs: statement.clone(), public_transcript_outputs: statement,
            binding_input: Fr::from(0u64), communications_commitment: None,
            key_location: KeyLocation(CHECK_KEY_LOCATION.into()) },
        C6PushStatementBindingMode::LedgerCall => {
            let randomness = Fr::from(0u64);
            let post = statement[post_state_start(w)..].to_vec();
            let mut comm_inputs = statement.clone(); comm_inputs.extend(post.iter().copied());
            let comm = comm_override.unwrap_or_else(|| transient_commit(&comm_inputs, randomness));
            ProofPreimage { inputs: statement.clone(), private_transcript: Vec::new(),
                public_transcript_inputs: statement, public_transcript_outputs: Vec::new(),
                binding_input: Fr::from(0u64), communications_commitment: Some((comm, randomness)),
                key_location: KeyLocation(CHECK_KEY_LOCATION.into()) }
        }
    };
    preimage.check(&c.ir).map_err(|e| C6ScalarArithmeticConsumerError::Backend(
        format!("SDK ProofPreimage::check failed: {e}")))
}
fn assert_raw_ok(w: &C6ScalarArithmeticStepWitness, s: Vec<Fr>, label: &str) {
    for mode in [C6PushStatementBindingMode::LocalPublic, C6PushStatementBindingMode::LedgerCall] {
        let c = circuit_for(w, mode);
        assert!(check_raw(&c, w, s.clone(), None).is_ok(), "{label}: raw positive must pass in {mode:?}");
    }
}
fn assert_raw_rejects(w: &C6ScalarArithmeticStepWitness, s: Vec<Fr>, label: &str) {
    for mode in [C6PushStatementBindingMode::LocalPublic, C6PushStatementBindingMode::LedgerCall] {
        let c = circuit_for(w, mode);
        match check_raw(&c, w, s.clone(), None) {
            Err(C6ScalarArithmeticConsumerError::Backend(m)) => assert!(
                m.contains("SDK ProofPreimage::check failed"),
                "{label} {mode:?}: must be SDK preprocessing rejection, got: {m}"),
            other => panic!("{label} {mode:?}: expected SDK rejection, ok={}", other.is_ok()),
        }
    }
}
fn set_both_prog(s: &mut [Fr], w: &C6ScalarArithmeticStepWitness, i: usize, v: u128) {
    s[pre_prog_at(w, i)] = Fr::from(v); s[post_prog_at(w, i)] = Fr::from(v);
}
fn set_both_helper(s: &mut [Fr], w: &C6ScalarArithmeticStepWitness, rel: usize, v: u128) {
    s[pre_helper_at(w, rel)] = Fr::from(v); s[post_helper_at(w, rel)] = Fr::from(v);
}
fn disabled_statement(w: &C6ScalarArithmeticStepWitness) -> Vec<Fr> {
    let mut s = w.to_statement_frs(); s[0] = Fr::from(0u64); s[1] = Fr::from(0u64);
    let pre = s[pre_state_start(w)..pre_state_start(w) + state_len(w)].to_vec();
    let at = post_state_start(w); s[at..at + state_len(w)].copy_from_slice(&pre); s
}
fn two_pow_128() -> Fr { Fr::from(MAX) + Fr::from(1u64) }

// ---------- genuine five-mode positives, both statement modes ----------

#[test]
fn genuine_five_modes_applied_both_modes() {
    type C = (&'static str, Spec, Box<dyn Fn() -> CoreProgram>);
    let cases: Vec<C> = vec![
        ("Not true", spec(ScalarMode::Not, KindWant::Bool(true), None),
            Box::new(|| prog(CoreExpr::Unary { op: UnaryOp::Not, operand: Box::new(lit_b(true)) }))),
        ("Not false", spec(ScalarMode::Not, KindWant::Bool(false), None),
            Box::new(|| prog(CoreExpr::Unary { op: UnaryOp::Not, operand: Box::new(lit_b(false)) }))),
        ("Not UInt err11", spec(ScalarMode::Not, KindWant::UInt(7), None),
            Box::new(|| prog(CoreExpr::Unary { op: UnaryOp::Not, operand: Box::new(lit_u(7)) }))),
        ("Add small", spec(ScalarMode::Add, KindWant::UInt(7), Some(KindWant::UInt(3))),
            Box::new(|| prog(bin(BinaryOp::Add, lit_u(7), lit_u(3))))),
        ("Add max+0", spec(ScalarMode::Add, KindWant::UInt(MAX), Some(KindWant::UInt(0))),
            Box::new(|| prog(bin(BinaryOp::Add, lit_u(MAX), lit_u(0))))),
        ("Add overflow err10", spec(ScalarMode::Add, KindWant::UInt(MAX), Some(KindWant::UInt(1))),
            Box::new(|| prog(bin(BinaryOp::Add, lit_u(MAX), lit_u(1))))),
        ("Add BoolB err11", spec(ScalarMode::Add, KindWant::UInt(7), Some(KindWant::Bool(false))),
            Box::new(|| prog(bin(BinaryOp::Add, lit_u(7), lit_b(false))))),
        ("Sub small", spec(ScalarMode::Sub, KindWant::UInt(7), Some(KindWant::UInt(3))),
            Box::new(|| prog(bin(BinaryOp::Sub, lit_u(7), lit_u(3))))),
        ("Sub underflow err10", spec(ScalarMode::Sub, KindWant::UInt(0), Some(KindWant::UInt(1))),
            Box::new(|| prog(bin(BinaryOp::Sub, lit_u(0), lit_u(1))))),
        ("Mul small", spec(ScalarMode::Mul, KindWant::UInt(7), Some(KindWant::UInt(3))),
            Box::new(|| prog(bin(BinaryOp::Mul, lit_u(7), lit_u(3))))),
        ("Mul 2^64 squared err10", spec(ScalarMode::Mul, KindWant::UInt(1 << 64), Some(KindWant::UInt(1 << 64))),
            Box::new(|| prog(bin(BinaryOp::Mul, lit_u(1 << 64), lit_u(1 << 64))))),
        ("Mul max*max err10", spec(ScalarMode::Mul, KindWant::UInt(MAX), Some(KindWant::UInt(MAX))),
            Box::new(|| prog(bin(BinaryOp::Mul, lit_u(MAX), lit_u(MAX))))),
        ("Mul max*1", spec(ScalarMode::Mul, KindWant::UInt(MAX), Some(KindWant::UInt(1))),
            Box::new(|| prog(bin(BinaryOp::Mul, lit_u(MAX), lit_u(1))))),
        ("Lt less", spec(ScalarMode::Lt, KindWant::UInt(0), Some(KindWant::UInt(MAX))),
            Box::new(|| prog(bin(BinaryOp::Lt, lit_u(0), lit_u(MAX))))),
        ("Lt equal false", spec(ScalarMode::Lt, KindWant::UInt(7), Some(KindWant::UInt(7))),
            Box::new(|| prog(bin(BinaryOp::Lt, lit_u(7), lit_u(7))))),
        ("Lt greater false", spec(ScalarMode::Lt, KindWant::UInt(MAX), Some(KindWant::UInt(0))),
            Box::new(|| prog(bin(BinaryOp::Lt, lit_u(MAX), lit_u(0))))),
        ("Add Data operandA 42", spec(ScalarMode::Add, KindWant::UInt(42), Some(KindWant::UInt(1))),
            Box::new(|| prog(bin(BinaryOp::Add, bin(BinaryOp::Add, lit_u(40), lit_u(2)), lit_u(1))))),
    ];
    for (label, w, make) in &cases {
        let a = run_core(w, label, make.as_ref());
        // Data-space provenance for the last case, Const elsewhere by construction.
        if *label == "Add Data operandA 42" {
            let HelperControlC::Scalar { operand_a, .. } = selected_helper(&a.record.before)
                else { unreachable!() };
            assert!(matches!(operand_a, RefC::Data(_)), "{label}: operandA must be Data");
        }
        assert_ok_both(&a.witness, label);
    }
}

// ---------- raw wrong post / preserved sections ----------

#[test]
fn raw_wrong_applied_post_fields_reject_both_modes() {
    let a = add_witness(); let w = &a.witness; assert_ok_both(w, "wrong-post control");
    let base = w.to_statement_frs();
    let mut s = base.clone(); s[post_helper_at(w, 13)] = Fr::from(1u64);
    assert_raw_rejects(w, s, "wrong post phase Second-not-Done");
    let mut s = base.clone(); s[post_helper_at(w, 14)] = Fr::from(999u64);
    assert_raw_rejects(w, s, "wrong post regA");
    let mut s = base.clone(); s[post_helper_at(w, 15)] = Fr::from(2u64);
    assert_raw_rejects(w, s, "wrong post staged tag Bool for Add");
    let mut s = base.clone(); s[post_helper_at(w, 16)] = Fr::from(11u64);
    assert_raw_rejects(w, s, "wrong post staged payload");
    let mut s = base.clone();
    s[post_helper_at(w, 7)] = Fr::from(w.pre_helper()[L_HELPER_RECORD + 7] + 1);
    assert_raw_rejects(w, s, "wrong post operandA Ref index");
    let mut s = base.clone();
    s[post_helper_at(w, 10)] = Fr::from(w.pre_helper()[L_HELPER_RECORD + 10] + 1);
    assert_raw_rejects(w, s, "wrong post operandB Ref index");
    let mut s = base.clone(); s[post_result_at(w, 0)] = Fr::from(10u64);
    assert_raw_rejects(w, s, "wrong post result record tag");
    let mut s = base.clone();
    s[post_state_start(w) + w.pre_image().len() + w.pre_data().len() + 3] = Fr::from(1u64);
    assert_raw_rejects(w, s, "wrong post helper task count");
    let mut s = base.clone(); s[post_image_at(w, L_OUTCOME)] = Fr::from(3u64);
    s[post_image_at(w, L_OUTCOME + 1)] = Fr::from(5u64);
    assert_raw_rejects(w, s, "wrong post outcome Exhausted on applied");
    let mut s = base.clone(); s[post_image_at(w, L_REFERENCE_CURSOR)] = Fr::from(999u64);
    assert_raw_rejects(w, s, "wrong post reference cursor");
    let mut s = base.clone(); s[post_image_at(w, L_CONSUMED_WORK)] = Fr::from(999u64);
    assert_raw_rejects(w, s, "wrong post consumed");
    let mut s = base.clone(); s[post_image_at(w, L_HELPER_VISITS)] = Fr::from(999u64);
    assert_raw_rejects(w, s, "wrong post helper visits");
}

#[test]
fn raw_wrong_preserved_sections_reject_both_modes() {
    let a = add_witness(); let w = &a.witness; let base = w.to_statement_frs();
    let mut s = base.clone(); s[post_meta_at(w, 0)] = Fr::from(0u64);
    assert_raw_rejects(w, s, "wrong post META lane");
    let mut s = base.clone(); s[post_prog_at(w, 0)] = Fr::from(0u64);
    assert_raw_rejects(w, s, "wrong post PROG lane");
    let mut s = base.clone(); s[post_ctx_at(w, 2)] = Fr::from(999u64);
    assert_raw_rejects(w, s, "wrong post CTX lane");
    let mut s = base.clone(); s[post_image_at(w, L_CONTROL)] = Fr::from(999u64);
    assert_raw_rejects(w, s, "wrong post image control");
    let mut s = base.clone(); s[post_image_at(w, 48)] = Fr::from(999u64);
    assert_raw_rejects(w, s, "wrong post first frame lane");
    let mut s = base.clone(); s[post_data_at(w, 10)] = Fr::from(777u64);
    assert_raw_rejects(w, s, "wrong post DATA lane");
    // Prefix magic/version on both copies: post==pre alone does not save it.
    let mut s = base.clone(); set_both_prog(&mut s, w, 0, 0);
    assert_raw_rejects(w, s, "wrong PROG magic both copies");
    let mut s = base.clone();
    s[pre_meta_at(w, 1)] = Fr::from(999u64); s[post_meta_at(w, 1)] = Fr::from(999u64);
    assert_raw_rejects(w, s, "wrong META version both copies");
    let mut s = base.clone();
    s[pre_ctx_at(w, 0)] = Fr::from(0u64); s[post_ctx_at(w, 0)] = Fr::from(0u64);
    assert_raw_rejects(w, s, "wrong CTX magic both copies");
}

// ---------- raw Fr / gate / Ref / phase-mode domains ----------

#[test]
fn raw_fr_gate_ref_and_slice_domains() {
    let a = add_witness(); let w = &a.witness; let base = w.to_statement_frs();
    // Preserved uncapped lane (initial fuel, image lane 26): u128::MAX accepted.
    let mut s = base.clone();
    s[pre_image_at(w, 26)] = Fr::from(MAX); s[post_image_at(w, 26)] = Fr::from(MAX);
    assert_raw_ok(w, s, "u128::MAX preserved lane");
    for (val, name) in [(two_pow_128(), "exactly 2^128"), (Fr::from(MAX) + Fr::from(2u64), "above 2^128")] {
        let mut s = base.clone(); s[pre_image_at(w, 26)] = val; s[post_image_at(w, 26)] = val;
        assert_raw_rejects(w, s, &format!("{name} gate1"));
        let mut s = disabled_statement(w); s[pre_image_at(w, 26)] = val; s[post_image_at(w, 26)] = val;
        assert_raw_rejects(w, s, &format!("{name} gate0 unconditional raw128"));
    }
    let mut s = base.clone(); s[0] = Fr::from(2u64);
    assert_raw_rejects(w, s, "enable gate value 2");
    let mut s = base.clone(); set_both_helper(&mut s, w, 6, 2);
    assert_raw_rejects(w, s, "Ref space value 2");
    // Ref index equal to actual active Const count, and capacity padding index.
    let const_count = a.record.before.admission.view.constants.len() as u128;
    let cap = w.source_params().caps().phys.max_constants as u128;
    assert!(const_count < cap, "fixture premise: active count below capacity");
    for (idx, name) in [(const_count, "RefB index equal to active Const count"),
                        (cap, "RefB index at capacity padding")] {
        let mut s = base.clone(); set_both_helper(&mut s, w, 10, idx);
        // Post helper identity follows the mutated pre Ref; reader membership fails.
        assert_raw_rejects(w, s, name);
    }
    let data_count = a.record.before.image.data.len() as u128;
    let mut s = base.clone(); set_both_helper(&mut s, w, 9, 1); set_both_helper(&mut s, w, 10, data_count);
    assert_raw_rejects(w, s, "RefB Data index equal to active DATA count");
    // Derived slice ends at gate0: DATA canonicality off, raw lanes in range.
    let geom = w.source_params().geometry();
    let start = pre_state_start(w) + w.pre_image().len() + geom.data_fields_base() - 2;
    // First DATA row slice lanes are at data offset 7+3 / 7+4 within pre_data.
    let slice_start = pre_state_start(w) + w.pre_image().len() + 7 + 3;
    let _ = start;
    let post_slice = post_state_start(w) + w.pre_image().len() + 7 + 3;
    for (len_v, name) in [(1u128, "disabled slice end exactly 2^128"), (MAX, "disabled slice end 2^129-2")] {
        let mut s = disabled_statement(w);
        s[slice_start] = Fr::from(MAX); s[slice_start + 1] = Fr::from(len_v);
        s[post_slice] = Fr::from(MAX); s[post_slice + 1] = Fr::from(len_v);
        assert_raw_ok(w, s, name);
    }
}

#[test]
fn raw_phase_mode_pairings_reject_both_modes() {
    let a = add_witness(); let w = &a.witness; let base = w.to_statement_frs();
    // Helper lanes: mode rel5, phase rel13. Pre/post are mutated
    // identically, but this does NOT isolate the selected pairing
    // predicate: applied post phase is separately constrained to
    // Done (rel13=2), so First/Second post phases also violate the
    // derived post-phase constraint, and e.g. (Second,Not) also
    // violates Not operandB-absent (rel8) and regA==0 (rel14) on
    // this Add base (regA=7, operandB present). These cases assert
    // only the coupled rejection union in both modes.
    // OPEN (not covered): pairing-predicate isolation with coherent
    // post-phase/staged/optionality/regA companions per mutated pair.
    for (mode, phase, name) in [
        (0u128, 0u128, "First numeric Add"), (7, 1, "Second Not"),
        (0, 2, "Done pre Add"), (6, 0, "Leaf First"), (4, 1, "Cons Second"),
        (5, 1, "Branch Second"), (8, 0, "TestBool First"), (9, 0, "TestType First"),
    ] {
        let mut s = base.clone();
        set_both_helper(&mut s, w, 5, mode); set_both_helper(&mut s, w, 13, phase);
        assert_raw_rejects(w, s, name);
    }
    // Not absent-B optionality: present bit on a Not base must reject.
    let n = not_witness(); let nw = &n.witness; let mut s = nw.to_statement_frs();
    set_both_helper(&mut s, nw, 8, 1);
    assert_raw_rejects(nw, s, "Not with operandB present");
    // Numeric operandA/regA domain mismatch, coherently paired on B side:
    // change regA only (operandB unchanged) on a Second/Add base.
    let mut s = base.clone(); set_both_helper(&mut s, w, 14, 8);
    // Post regA follows pre (helper preservation lane), so SI22
    // denotation fails; the retained post staged payload (10) also
    // differs from the payload derived from regA=8 (11), so this
    // asserts the coupled rejection union, not SI22 isolation.
    assert_raw_rejects(w, s, "Second regA not equal operandA payload");
}

// ---------- arithmetic branch negatives with coherent companions ----------

// ---------- coherent pairing / regA isolation ----------
//
// Emitter facts used for the recomputations below (actual
// `c6_scalar_arithmetic_step.rs`): applied post phase is Done (2);
// post helper lanes other than phase/staged-tag/staged-payload equal
// the pre lanes; `operandB` present iff numeric; Not requires
// `regA == 0`; numeric requires `regA == readerA.uint_value`;
// derived staged value is computed from mode, `regA` (x) and the
// actual operand-B UInt payload (n). P3 is not used here (pre
// helper-work is the genuine applied value, not the retained max).

#[test]
fn coherent_first_numeric_pairings_reject_both_modes() {
    let a = add_witness(); let w = &a.witness; let base = w.to_statement_frs();
    assert_raw_ok(w, base.clone(), "First/numeric paired positive (Second/Add)");
    // Base is Second/Add 7+3. For each numeric mode: pre phase First
    // (pre only; applied post phase stays Done), mode written to BOTH
    // pre rel5 and post rel5 (post rel5 is a preservation lane: the
    // applied helper loop permits post/pre differences only at
    // rel13 phase, rel15 staged tag, rel16 staged payload), post
    // staged recomputed for (7,3): Add 10, Sub 4, Mul 21, Lt false.
    // For Add (mode 0) the post write is value-identical to the base.
    for (mode, tag, payload, name) in [
        (0u128, 1u128, 10u128, "First Add coherent"),
        (1, 1, 4, "First Sub coherent"),
        (2, 1, 21, "First Mul coherent"),
        (3, 2, 0, "First Lt coherent"),
    ] {
        let mut s = base.clone();
        s[pre_helper_at(w, 5)] = Fr::from(mode);
        s[post_helper_at(w, 5)] = Fr::from(mode);
        s[pre_helper_at(w, 13)] = Fr::from(0u64);
        s[post_helper_at(w, 15)] = Fr::from(tag);
        s[post_helper_at(w, 16)] = Fr::from(payload);
        // All other companions unchanged from the applied positive and
        // coherent with the mutated pre mode: pre staged None, post
        // Done, regA=7 in pre and post (readerA row still UInt 7, same
        // Ref kind/index identity), operandB present UInt 3 (readerB
        // row unchanged), expected absent, common site/origin/
        // parent_resume, result None, task count 0, applied P4/P7
        // counters, outcome/cursor and all META/PROG/CTX/image/DATA
        // preservation lanes. LedgerCall CC is recomputed from this
        // mutated statement and its post flat by `check_raw`.
        // Isolation is by this construction only; the generic SDK
        // `Err` text is not used as a per-predicate oracle.
        assert_raw_rejects(w, s, name);
    }
}

#[test]
fn coherent_second_not_pairings_reject_both_modes() {
    let n = not_witness(); let nw = &n.witness; let base = nw.to_statement_frs();
    assert_raw_ok(nw, base.clone(), "Second/Not paired positive (First/Not Bool)");
    // Only pre phase First->Second. Mode Not, Bool operand A,
    // operandB absent, regA=0, post Done with BoolValue{false}.
    let mut s = base.clone();
    s[pre_helper_at(nw, 13)] = Fr::from(1u64);
    assert_raw_rejects(nw, s, "Second Not Bool coherent");

    let e = run_core(&spec(ScalarMode::Not, KindWant::UInt(7), None), "Second Not UInt base",
        &|| prog(CoreExpr::Unary { op: UnaryOp::Not, operand: Box::new(lit_u(7)) }));
    let ew = &e.witness; let base = ew.to_statement_frs();
    assert_raw_ok(ew, base.clone(), "Second/Not paired positive (First/Not UInt err11)");
    let mut s = base.clone();
    s[pre_helper_at(ew, 13)] = Fr::from(1u64);
    // Post Done with PrimError{11} is already the coherent Not/UInt post.
    assert_raw_rejects(ew, s, "Second Not UInt coherent err11");
}

#[test]
fn coherent_done_pairings_reject_both_modes() {
    let a = add_witness(); let w = &a.witness; let base = w.to_statement_frs();
    assert_raw_ok(w, base.clone(), "Done/Add paired positive");
    let mut s = base.clone();
    s[pre_helper_at(w, 13)] = Fr::from(2u64);
    assert_raw_rejects(w, s, "Done Add coherent");

    let n = not_witness(); let nw = &n.witness; let base = nw.to_statement_frs();
    assert_raw_ok(nw, base.clone(), "Done/Not paired positive");
    let mut s = base.clone();
    s[pre_helper_at(nw, 13)] = Fr::from(2u64);
    assert_raw_rejects(nw, s, "Done Not coherent");
}

#[test]
fn coherent_second_rega_mismatch_rejects_both_modes() {
    let a = add_witness(); let w = &a.witness; let base = w.to_statement_frs();
    assert_raw_ok(w, base.clone(), "regA mismatch paired positive");
    // Mutated regA=8 in pre and in post (post helper preservation
    // lane). Derived staged payload is recomputed as 8+3=11, tag
    // U128Value, so the retained rejection is the readerA/regA
    // denotation: actual operand-A row is still UInt 7. This is
    // isolation by construction; the SDK error text is not used as
    // a per-predicate oracle.
    let mut s = base.clone();
    set_both_helper(&mut s, w, 14, 8);
    s[post_helper_at(w, 16)] = Fr::from(11u64);
    assert_raw_rejects(w, s, "Second regA=8 vs operandA UInt 7, staged 11 coherent");
}

// ---------- host producer refusals (normal APIs) ----------

#[test]
fn host_producer_meta_and_prog_prefix_mismatch() {
    let a = add_witness(); let cfg = core_cfg(bounds(100)); let lim = limits();
    witness_from_host_pair(true, &a.record.before, &a.record.after, &cfg, &lim)
        .unwrap_or_else(|e| panic!("prefix paired positive: {e}"));

    // META: run identity is a META lane source. Changing only the
    // post run fuel input keeps layout/PROG/CTX derivation equal and
    // encode-valid, and META is compared first.
    let mut bad = a.record.after.clone();
    bad.image.run_id.fuel_input = RunIntC::Nat(65);
    assert!(matches!(
        witness_from_host_pair(true, &a.record.before, &bad, &cfg, &lim),
        Err(C6ScalarArithmeticConsumerError::HostPrefixMismatch { section: "meta" })),
        "post run-id change must refuse as meta HostPrefixMismatch");

    // PROG: program entry is a PROG lane source, not a META source.
    // The mutated whole ProgramSnapshot still encodes/decodes
    // through the real codec inside `from_host_snapshot`. This
    // mutated clone is a producer input, not an owner-admitted state.
    let mut bad = a.record.after.clone();
    bad.admission.view.entry += 1;
    assert!(matches!(
        witness_from_host_pair(true, &a.record.before, &bad, &cfg, &lim),
        Err(C6ScalarArithmeticConsumerError::HostPrefixMismatch { section: "prog" })),
        "post program-entry change must refuse as prog HostPrefixMismatch");
}

#[test]
fn host_producer_ctx_mismatch_unreachable_by_derivation() {
    // Exact obstruction: `witness_from_host_pair` takes one `cfg`
    // and calls `C6PushHostPrefixLanes::from_host_snapshot` for pre
    // and post with that same `cfg`. That constructor derives
    // `ctx = encode_context(cfg.ext, cfg.budgets)`; it does not read
    // any CTX source from either HostSnapshot. Hence pp.ctx == qp.ctx
    // for every input pair, and `HostPrefixMismatch{section:"ctx"}`
    // is unreachable through this normal API. No fake CTX post is
    // constructed. The reachable assertion is the derivation itself:
    let a = add_witness(); let cfg = core_cfg(bounds(100)); let lim = limits();
    let pp = C6PushHostPrefixLanes::from_host_snapshot(&a.record.before, &cfg, &lim)
        .unwrap_or_else(|e| panic!("pre prefix: {e}"));
    let qp = C6PushHostPrefixLanes::from_host_snapshot(&a.record.after, &cfg, &lim)
        .unwrap_or_else(|e| panic!("post prefix: {e}"));
    assert_eq!(pp.ctx, qp.ctx, "CTX is cfg-derived identically for pre and post");
    witness_from_host_pair(true, &a.record.before, &a.record.after, &cfg, &lim)
        .unwrap_or_else(|e| panic!("ctx-derivation paired positive: {e}"));
}

#[test]
fn host_producer_retained_config_mismatch() {
    let a = add_witness(); let lim = limits();
    witness_from_host_pair(true, &a.record.before, &a.record.after, &core_cfg(bounds(100)), &lim)
        .unwrap_or_else(|e| panic!("config paired positive: {e}"));
    // Check order: `C6PushFamilyParams::new` first, then the pre
    // prefix builder refuses `host.admission.layout != cfg.layout`
    // before any META/PROG/CTX comparison.
    let wrong_cfg = core_cfg(bounds(101));
    assert!(matches!(
        witness_from_host_pair(true, &a.record.before, &a.record.after, &wrong_cfg, &lim),
        Err(C6ScalarArithmeticConsumerError::Push(
            C6PushStepError::ConfigMismatch { field: "layout" }))),
        "retained layout mismatch must be Push(ConfigMismatch layout)");
}

#[test]
fn host_producer_geometry_validate_and_section_length() {
    let a = add_witness(); let w = &a.witness;
    C6ScalarArithmeticStepEmitter::new("producer_control").expect("scope")
        .emit(&valid_inputs(w)).expect("geometry/length paired positive");

    // Emitter order: `source_params.validate_geometry` before caps
    // and before section lengths.
    let mut inputs = valid_inputs(w);
    inputs.geometry.image_total_lanes += 1;
    let mut em = C6ScalarArithmeticStepEmitter::new("producer_geom").expect("scope");
    assert!(matches!(
        em.emit(&inputs),
        Err(C6ScalarArithmeticStepError::Push(
            C6PushStepError::InvalidGeometry { section: "geometry" }))),
        "tampered geometry must be Push(InvalidGeometry)");

    let expected = w.source_params().geometry().image_total_lanes;
    let mut inputs = valid_inputs(w);
    inputs.pre_image.pop();
    let mut em = C6ScalarArithmeticStepEmitter::new("producer_len").expect("scope");
    assert!(matches!(
        em.emit(&inputs),
        Err(C6ScalarArithmeticStepError::LengthMismatch {
            section: "image", expected: e, actual: ac }) if e == expected && ac == expected - 1),
        "short pre_image must be image LengthMismatch with exact counts");
}

#[test]
fn host_producer_checked_geometry_overflow_and_state_limit() {
    let a = add_witness();
    let needed = a.witness.source_params().geometry().state_total_lanes;
    C6PushFamilyParams::new(core_cfg(bounds(100)), bounds(100), limits())
        .expect("params paired positive");

    let mut small = limits(); small.max_state_lanes = needed - 1;
    assert!(matches!(
        C6PushFamilyParams::new(core_cfg(bounds(100)), bounds(100), small),
        Err(C6PushStepError::StateLimitExceeded { needed: n, limit: l })
            if n == needed && l == needed - 1),
        "state lane limit must be StateLimitExceeded with exact counts");

    // Checked geometry precedes allocation: max_frames dominates
    // the image total computation.
    let mut b = bounds(100); b.frames = usize::MAX;
    assert!(matches!(
        C6PushFamilyParams::new(core_cfg(b), bounds(100), limits()),
        Err(C6PushStepError::GeometryOverflow { section: "image" })),
        "frames=MAX must overflow checked image geometry");
}

#[test]
fn host_producer_usize_max_cap_reachable_and_dominated() {
    let a = add_witness(); let w = &a.witness;
    C6ScalarArithmeticStepEmitter::new("producer_cap_control").expect("scope")
        .emit(&valid_inputs(w)).expect("cap paired positive");

    // Reachable MAX cap: `max_constants` is not a padded-lane
    // multiplier, so retained params can be built through the normal
    // `C6PushFamilyParams::new` chain with this cap, geometry
    // validation recomputes the same geometry, and the emitter cap
    // guard is the first refusal.
    let mut b = bounds(100); b.constant_nodes = usize::MAX;
    let params = C6PushFamilyParams::new(core_cfg(b), bounds(100), limits())
        .expect("max_constants=MAX retained params must build");
    assert_eq!(params.caps().phys.max_constants, usize::MAX);
    let mut inputs = valid_inputs(w);
    inputs.geometry = params.geometry();
    inputs.source_params = params;
    let mut em = C6ScalarArithmeticStepEmitter::new("producer_cap").expect("scope");
    assert!(matches!(
        em.emit(&inputs),
        Err(C6ScalarArithmeticStepError::GeometryOverflow { section: "caps" })),
        "max_constants=MAX must reach emitter caps GeometryOverflow");

    // Dominated MAX path: `max_data_nodes` is a lane multiplier in
    // checked geometry, so normal params construction refuses
    // earlier; the emitter caps guard is not reachable for this cap
    // through retained params and is not fabricated here.
    let mut b = bounds(100); b.data_nodes = usize::MAX;
    assert!(matches!(
        C6PushFamilyParams::new(core_cfg(b), bounds(100), limits()),
        Err(C6PushStepError::GeometryOverflow { section: "data.data" })),
        "data_nodes=MAX must overflow checked data geometry first");
}
#[test]
fn raw_branch_wrong_staged_companions_reject() {
    // Low-only false success: 2^64 * 2^64 has low 0 but is error10.
    let a = run_core(&spec(ScalarMode::Mul, KindWant::UInt(1 << 64), Some(KindWant::UInt(1 << 64))),
        "mul high-half", &|| prog(bin(BinaryOp::Mul, lit_u(1 << 64), lit_u(1 << 64))));
    let w = &a.witness; let mut s = w.to_statement_frs();
    s[post_helper_at(w, 15)] = Fr::from(1u64); s[post_helper_at(w, 16)] = Fr::from(0u64);
    assert_raw_rejects(w, s, "Mul 2^64*2^64 wrong low-only U128 success");
    // max*max low is 1, still error10.
    let a = run_core(&spec(ScalarMode::Mul, KindWant::UInt(MAX), Some(KindWant::UInt(MAX))),
        "mul maxmax", &|| prog(bin(BinaryOp::Mul, lit_u(MAX), lit_u(MAX))));
    let w = &a.witness; let mut s = w.to_statement_frs();
    s[post_helper_at(w, 15)] = Fr::from(1u64); s[post_helper_at(w, 16)] = Fr::from(1u64);
    assert_raw_rejects(w, s, "Mul max*max wrong low U128 success");
    // Add carry / Sub borrow error10 with wrong U128 companions.
    let a = run_core(&spec(ScalarMode::Add, KindWant::UInt(MAX), Some(KindWant::UInt(1))),
        "add overflow", &|| prog(bin(BinaryOp::Add, lit_u(MAX), lit_u(1))));
    let w = &a.witness; let mut s = w.to_statement_frs();
    s[post_helper_at(w, 15)] = Fr::from(1u64); s[post_helper_at(w, 16)] = Fr::from(0u64);
    assert_raw_rejects(w, s, "Add overflow wrong U128 0");
    let a = run_core(&spec(ScalarMode::Sub, KindWant::UInt(0), Some(KindWant::UInt(1))),
        "sub underflow", &|| prog(bin(BinaryOp::Sub, lit_u(0), lit_u(1))));
    let w = &a.witness; let mut s = w.to_statement_frs();
    s[post_helper_at(w, 15)] = Fr::from(1u64); s[post_helper_at(w, 16)] = Fr::from(MAX);
    assert_raw_rejects(w, s, "Sub underflow wrong U128 max");
    // Error11 with wrong error10 / wrong payload.
    let a = run_core(&spec(ScalarMode::Add, KindWant::UInt(7), Some(KindWant::Bool(false))),
        "add err11", &|| prog(bin(BinaryOp::Add, lit_u(7), lit_b(false))));
    let w = &a.witness; let mut s = w.to_statement_frs();
    s[post_helper_at(w, 16)] = Fr::from(10u64);
    assert_raw_rejects(w, s, "err11 case with wrong error10 payload");
    // Lt equality must be false; Not UInt must be error11 not Bool.
    let a = run_core(&spec(ScalarMode::Lt, KindWant::UInt(7), Some(KindWant::UInt(7))),
        "lt equal", &|| prog(bin(BinaryOp::Lt, lit_u(7), lit_u(7))));
    let w = &a.witness; let mut s = w.to_statement_frs();
    s[post_helper_at(w, 16)] = Fr::from(1u64);
    assert_raw_rejects(w, s, "Lt equality wrong Bool true");
    let a = run_core(&spec(ScalarMode::Not, KindWant::UInt(7), None),
        "not err11", &|| prog(CoreExpr::Unary { op: UnaryOp::Not, operand: Box::new(lit_u(7)) }));
    let w = &a.witness; let mut s = w.to_statement_frs();
    s[post_helper_at(w, 15)] = Fr::from(2u64); s[post_helper_at(w, 16)] = Fr::from(0u64);
    assert_raw_rejects(w, s, "Not UInt wrong Bool staged");
}

// ---------- P3 / disabled / counter overflow ----------

fn p3_statement(w: &C6ScalarArithmeticStepWitness) -> Vec<Fr> {
    let mut s = w.to_statement_frs();
    let max = w.source_params().admission_bounds().max_helper_work;
    s[pre_image_at(w, L_HELPER_WORK)] = Fr::from(max);
    let pre = s[pre_state_start(w)..pre_state_start(w) + state_len(w)].to_vec();
    let at = post_state_start(w); s[at..at + state_len(w)].copy_from_slice(&pre);
    s[post_image_at(w, L_OUTCOME)] = Fr::from(3u64);
    s[post_image_at(w, L_OUTCOME + 1)] = Fr::from(5u64);
    s[post_image_at(w, L_OUTCOME + 2)] = Fr::from(0u64);
    s[1] = Fr::from(1u64); s
}

#[test]
fn raw_p3_disabled_and_counter_max() {
    let a = add_witness(); let w = &a.witness;
    let p3 = p3_statement(w); assert_raw_ok(w, p3.clone(), "raw P3 Second/Add coherent");
    let mut s = p3.clone(); s[1] = Fr::from(0u64);
    assert_raw_rejects(w, s, "P3 wrong disposition flag");
    let mut s = p3.clone();
    s[post_image_at(w, L_CONSUMED_WORK)] = Fr::from(w.pre_image()[L_CONSUMED_WORK] + 1);
    assert_raw_rejects(w, s, "P3 wrong claimed charged consumed");
    let mut s = p3.clone();
    s[post_image_at(w, L_OUTCOME)] = Fr::from(0u64); s[post_image_at(w, L_OUTCOME + 1)] = Fr::from(0u64);
    assert_raw_rejects(w, s, "P3 wrong Running outcome");
    // P3 for Not/First: phase preserved First, not Done.
    let n = not_witness(); let nw = &n.witness;
    assert_raw_ok(nw, p3_statement(nw), "raw P3 First/Not coherent");
    // Disabled equality positive, wrong disabled post negative.
    let d = disabled_statement(w); assert_raw_ok(w, d.clone(), "disabled equality");
    let mut s = d.clone(); s[post_helper_at(w, 16)] = Fr::from(42u64);
    assert_raw_rejects(w, s, "disabled wrong post staged payload");
    // Charged counter maxima: not P3 (retained max is 100), so overflow guard rejects.
    let base = w.to_statement_frs();
    for (lane, name) in [(L_CONSUMED_WORK, "consumed MAX"), (L_HELPER_VISITS, "visits MAX")] {
        let mut s = base.clone();
        s[pre_image_at(w, lane)] = Fr::from(MAX); s[post_image_at(w, lane)] = Fr::from(0u64);
        assert_raw_rejects(w, s, name);
    }
    let mut s = base.clone();
    s[pre_image_at(w, L_HELPER_WORK)] = Fr::from(MAX); s[post_image_at(w, L_HELPER_WORK)] = Fr::from(0u64);
    assert_raw_rejects(w, s, "helper-work MAX charged overflow (not retained-max P3 equality)");
    let mut s = base.clone();
    s[pre_image_at(w, L_REMAINING_WORK)] = Fr::from(1u64); s[post_image_at(w, L_REMAINING_WORK)] = Fr::from(1u64);
    assert_raw_rejects(w, s, "wrong remaining-work charge");
}

#[test]
fn genuine_p3_exact_retained_bound() {
    let label = "genuine P3 Add";
    let wspec = spec(ScalarMode::Add, KindWant::UInt(40), Some(KindWant::UInt(2)));
    let make = || prog(bin(BinaryOp::Add, lit_u(40), lit_u(2)));
    let (_, _, pre) = reach_core(bounds(100), &wspec, label, &make);
    let hw = pre.image.helper_work; assert!(hw > 0, "{label}: positive helper work");
    let exact = bounds(hw);
    let (admitted, mut image, pre2) = reach_core(exact, &wspec, label, &make);
    assert_eq!(pre2.image.helper_work, exact.max_helper_work);
    let record = match owner_record(&admitted, &mut image) {
        Ok(ObservedStep::Event(r)) => r,
        Ok(ObservedStep::Terminal { .. }) => panic!("{label}: P3 must be an event"),
        Err(e) => panic!("{label}: P3 step: {}", capture_label(&e)),
    };
    assert_eq!(record.event.disposition, StepDisposition::RefusedHelperWorkUncharged, "{label}: disposition");
    assert_eq!((record.event.work_delta, record.event.helper_work_delta, record.event.helper_visits_delta), (0, 0, 0), "{label}: uncharged");
    assert_eq!(record.event.exhausted_kind, Some(ExhaustReason::HelperWork), "{label}: exhausted");
    assert_eq!(record.after.image.outcome,
        OutcomeC::Exhausted { reason: ExhaustReason::HelperWork }, "{label}: outcome");
    assert_eq!(record.after.image.helper, record.before.image.helper, "{label}: helper preserved");
    assert_eq!(record.event.helper_phase_after,
        Some(HelperPhaseSummaryC::Scalar(ScalarPhase::Second)), "{label}: P3 preserves Second");
    assert_eq!(record.event.helper_cursor_after, Some(1), "{label}: P3 preserves cursor 1");
    let witness = witness_from_owner_record(&record, &core_cfg(exact), &limits())
        .unwrap_or_else(|e| panic!("{label}: witness: {e}"));
    assert!(witness.disposition_is_refused_helper_work(), "{label}: witness flag");
    assert_eq!(witness.source_params().admission_bounds(), record.before.admission.bounds,
        "{label}: retained original admission bounds");
    assert_ok_both(&witness, label);
}

// ---------- live env / value-slot and FullRef identity ----------

fn live_call_witness() -> Applied {
    // f(a,b) = a + b, called f(7,7): two distinct Const rows, both UInt 7.
    // At the body's Second/Add state the env chain and the physical
    // value-slot prefix are live (genesis empty; Append#1 Tail only =1;
    // Append#2 copy+Tail =3 physical rows for 2 logical arguments).
    let make = || CoreProgram {
        functions: vec![CoreFunction { name: "f".into(),
            parameters: vec![CoreParameter { name: "a".into(), sort: SortTag::UInt },
                             CoreParameter { name: "b".into(), sort: SortTag::UInt }],
            return_type: SortTag::UInt,
            body: bin(BinaryOp::Add, CoreExpr::Var { name: "a".into() }, CoreExpr::Var { name: "b".into() }) }],
        entry: CoreExpr::Call { name: "f".into(), args: vec![lit_u(7), lit_u(7)] },
    };
    run_core(&spec(ScalarMode::Add, KindWant::UInt(7), Some(KindWant::UInt(7))), "live call Add", &make)
}

#[test]
fn raw_live_env_slot_and_fullref_identity_reject() {
    let a = live_call_witness(); let (w, r) = (&a.witness, &a.record);
    assert!(!r.before.image.env.is_empty(), "live fixture: nonempty env");
    assert_eq!(r.before.image.value_slots.len(), 3, "live fixture: 3 physical slot rows");
    assert_ok_both(w, "live control");
    let HelperControlC::Scalar { operand_a, operand_b, .. } = selected_helper(&r.before)
        else { unreachable!() };
    let (sel_a, sel_b) = match (operand_a, operand_b) {
        (RefC::Const(x), Some(RefC::Const(y))) => (x, y), _ => panic!("live operands must be Const") };
    assert_ne!(sel_a, sel_b, "distinct Const rows");
    for i in [sel_a, sel_b] {
        assert!(matches!(r.before.admission.view.constants[i],
            ConstantSnapshot::U128 { n: 7, .. }), "both rows decode to UInt 7");
    }
    let geom = w.source_params().geometry(); let base = w.to_statement_frs();
    // Post env cell holding selected operandA: mutate only its Ref index lane.
    let cell_pos = r.before.image.env.iter().position(|c| c.value == operand_a)
        .expect("live env cell for operandA");
    let env_lane = geom.env_base() + cell_pos * ENV_ROW_LANES + 2;
    assert_eq!(w.pre_data()[env_lane], sel_a as u128, "pre env Ref index");
    let mut s = base.clone(); s[post_data_at(w, env_lane)] = Fr::from(sel_b as u128);
    assert_raw_rejects(w, s, "wrong post live env-cell Ref");
    // Post value slot holding selected operandA.
    let slot_pos = r.before.image.value_slots.iter().position(|x| *x == operand_a)
        .expect("live value slot for operandA");
    let slot_lane = geom.value_slots_base() + slot_pos * REF_LANES + 1;
    assert_eq!(w.pre_data()[slot_lane], sel_a as u128, "pre slot Ref index");
    let mut s = base.clone(); s[post_data_at(w, slot_lane)] = Fr::from(sel_b as u128);
    assert_raw_rejects(w, s, "wrong post live value-slot Ref");
    // FullRef identity: post helper operandA index -> other equal-payload row.
    let mut s = base.clone(); s[post_helper_at(w, 7)] = Fr::from(sel_b as u128);
    assert_eq!(s[post_helper_at(w, 16)], base[post_helper_at(w, 16)], "derived staged preserved");
    assert_raw_rejects(w, s, "post operand Ref identity, equal payload distinct row");
}

// ---------- raw held-Marker program fixture (codec domain only) ----------

#[test]
fn raw_marker_not_error11_coherent_passes_wrong_bool_rejects() {
    let a = not_witness(); let w = &a.witness; let r = &a.record;
    let HelperControlC::Scalar { operand_a: RefC::Const(ci), .. } = selected_helper(&r.before)
        else { panic!("Not operandA must be Const") };
    let marker_free = match &r.before.admission.view.constants[ci] {
        ConstantSnapshot::Bool { marker_free, .. } => *marker_free,
        _ => panic!("Not selected constant must be Bool") };
    // Whole actual ProgramSnapshot via the real codec; only the selected
    // row changes. This is NOT an owner-admitted held Marker (SI17 open).
    let mut mp = r.before.admission.view.clone();
    mp.constants[ci] = ConstantSnapshot::U128OutOfRange { marker_free };
    let caps = w.source_params().caps().phys;
    let encoded = encode_program(&mp, &caps).unwrap_or_else(|e| panic!("marker encode: {e:?}"));
    let decoded = decode_program(&encoded, &caps).unwrap_or_else(|e| panic!("marker decode: {e:?}"));
    assert_eq!(decoded, mp, "marker codec roundtrip at exact length");
    assert_eq!(encoded.len() + 1, w.host_prefix().prog.len(),
        "Marker row width 2 vs Bool width 3: exactly one lane shorter");
    // Statement over the rebuilt-length program; coherent post is error11.
    let build_stmt = |tag: u128, payload: u128| -> (Vec<Fr>, usize) {
        let hp = w.host_prefix();
        let mut post_helper = w.post_helper().to_vec();
        post_helper[L_HELPER_RECORD + 15] = tag; post_helper[L_HELPER_RECORD + 16] = payload;
        let mut s = vec![Fr::from(w.enable()), Fr::from(w.disposition_is_refused_helper_work()),
            Fr::from(C6_HOST_SCHEMA), Fr::from(C6_PADDED_CARRIER_VERSION)];
        for l in hp.meta.iter().chain(encoded.iter()).chain(hp.ctx.iter())
            .chain(w.pre_image()).chain(w.pre_data()).chain(w.pre_helper()) { s.push(Fr::from(*l)); }
        for l in hp.meta.iter().chain(encoded.iter()).chain(hp.ctx.iter())
            .chain(w.post_image()).chain(w.post_data()).chain(post_helper.iter()) { s.push(Fr::from(*l)); }
        (s, encoded.len())
    };
    let check_marker = |s: Vec<Fr>, plen: usize| -> Vec<bool> {
        [C6PushStatementBindingMode::LocalPublic, C6PushStatementBindingMode::LedgerCall].iter().map(|&mode| {
            let c = build_c6_scalar_arithmetic_step_circuit(w.source_params().clone(),
                w.host_prefix().meta.len(), plen, w.host_prefix().meta_bounds_frames_offset,
                mode, "scalar_arith_raw_test").expect("marker circuit builds");
            // Recomputed commitment over this statement and its post flat.
            let prefix = w.host_prefix().meta.len() + plen + CTX_LANES;
            let post_start = 4 + prefix + state_len(w) + prefix;
            let preimage = match mode {
                C6PushStatementBindingMode::LocalPublic => ProofPreimage {
                    inputs: Vec::new(), private_transcript: Vec::new(),
                    public_transcript_inputs: s.clone(), public_transcript_outputs: s.clone(),
                    binding_input: Fr::from(0u64), communications_commitment: None,
                    key_location: KeyLocation(CHECK_KEY_LOCATION.into()) },
                C6PushStatementBindingMode::LedgerCall => {
                    let randomness = Fr::from(0u64);
                    let mut ci2 = s.clone(); ci2.extend(s[post_start..].iter().copied());
                    ProofPreimage { inputs: s.clone(), private_transcript: Vec::new(),
                        public_transcript_inputs: s.clone(), public_transcript_outputs: Vec::new(),
                        binding_input: Fr::from(0u64),
                        communications_commitment: Some((transient_commit(&ci2, randomness), randomness)),
                        key_location: KeyLocation(CHECK_KEY_LOCATION.into()) }
                }
            };
            preimage.check(&c.ir).is_ok()
        }).collect()
    };
    let (s, plen) = build_stmt(4, 11);
    assert_eq!(check_marker(s, plen), vec![true, true], "Marker Not coherent error11 both modes");
    let (s, plen) = build_stmt(2, 0);
    assert_eq!(check_marker(s, plen), vec![false, false], "Marker Not wrong Bool staged both modes");
}

// ---------- namespace composition: 8 segments x 16 operand groups ----------

const SEGMENTS: [&str; 8] = [
    ".local.temp.", ".c6data.temp.", ".c6helper.temp.", ".c6meta.temp.",
    ".c6progctx.temp.", ".arithread.local.temp.", ".arithread.c6progctx.temp.",
    ".arithread.c6data.temp.",
];
fn imm(v: u128) -> Operand { Operand::Immediate(Fr::from(v)) }
fn valid_inputs(w: &C6ScalarArithmeticStepWitness) -> C6ScalarArithmeticStepInputs {
    let hp = w.host_prefix();
    let ops = |xs: &[u128]| xs.iter().map(|&v| imm(v)).collect::<Vec<_>>();
    let ctx: [Operand; CTX_LANES] = hp.ctx.iter().map(|&v| imm(v))
        .collect::<Vec<_>>().try_into().expect("ctx lanes");
    C6ScalarArithmeticStepInputs {
        enable: imm(1), disposition_is_refused_helper_work: imm(u128::from(w.disposition_is_refused_helper_work())),
        host_schema: imm(C6_HOST_SCHEMA), carrier_version: imm(C6_PADDED_CARRIER_VERSION),
        geometry: w.source_params().geometry(), meta_bounds_frames_offset: hp.meta_bounds_frames_offset,
        source_params: w.source_params().clone(),
        pre_meta: ops(&hp.meta), pre_prog: ops(&hp.prog), pre_ctx: ctx.clone(),
        pre_image: ops(w.pre_image()), pre_data: ops(w.pre_data()), pre_helper: ops(w.pre_helper()),
        post_meta: ops(&hp.meta), post_prog: ops(&hp.prog), post_ctx: ctx,
        post_image: ops(w.post_image()), post_data: ops(w.post_data()), post_helper: ops(w.post_helper()),
    }
}
fn dests(ins: &[Instruction]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for i in ins {
        match i {
            Instruction::Add { output, .. } | Instruction::Mul { output, .. }
            | Instruction::Neg { output, .. } | Instruction::Not { output, .. }
            | Instruction::CondSelect { output, .. } | Instruction::TestEq { output, .. }
            | Instruction::LessThan { output, .. } | Instruction::Copy { output, .. } =>
                { out.insert(output.0.clone()); }
            Instruction::DivModPowerOfTwo { outputs, .. } =>
                { for o in outputs { out.insert(o.0.clone()); } }
            _ => {}
        }
    }
    out
}

#[test]
fn raw_namespace_reserved_scopes_and_repeated_emitter() {
    let a = add_witness(); let w = &a.witness;
    for seg in SEGMENTS {
        let expected = format!("%arithraw{seg}77");
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
            let mut em = C6ScalarArithmeticStepEmitter::new("arithraw").expect("scope");
            match em.emit(&inputs) {
                Err(C6ScalarArithmeticStepError::ReservedOperand { identifier }) =>
                    assert_eq!(identifier, expected, "segment {seg} group {group}"),
                other => panic!("segment {seg} group {group}: expected ReservedOperand, ok={}", other.is_ok()),
            }
        }
        match C6ScalarArithmeticStepEmitter::new(&format!("bad{seg}x")) {
            Err(C6ScalarArithmeticStepError::InvalidScope) => {}
            other => panic!("scope {seg}: expected InvalidScope, ok={}", other.is_ok()),
        }
    }
    match C6ScalarArithmeticStepEmitter::new("") {
        Err(C6ScalarArithmeticStepError::EmptyScope) => {}
        other => panic!("empty scope: ok={}", other.is_ok()),
    }
    match C6ScalarArithmeticStepEmitter::new("bad%scope") {
        Err(C6ScalarArithmeticStepError::InvalidScope) => {}
        other => panic!("percent scope: ok={}", other.is_ok()),
    }
    // Repeated retained emits under one scope: destinations nonempty/disjoint,
    // parent and reader serials advance (next_temp is the parent serial).
    let mut em = C6ScalarArithmeticStepEmitter::new("arithraw_repeat").expect("repeat scope");
    let inputs = valid_inputs(w);
    let first = em.emit(&inputs).expect("first emit");
    let after_first = em.next_temp();
    let second = em.emit(&inputs).expect("second emit");
    assert!(em.next_temp() > after_first, "retained serial advances");
    let (d1, d2) = (dests(&first.instructions), dests(&second.instructions));
    assert!(!d1.is_empty() && !d2.is_empty(), "destinations nonempty");
    assert!(d1.is_disjoint(&d2), "repeated same-scope destinations disjoint");
    let r1 = dests(&first.reader_a.instructions); let r2 = dests(&second.reader_a.instructions);
    assert!(!r1.is_empty() && r1.is_disjoint(&r2), "reader A retained serials advance disjointly");
}

// ---------- stale commitment: separately labelled, not a relation proxy ----------

#[test]
fn raw_stale_commitment_isolated_case() {
    let a = add_witness(); let w = &a.witness; let base = w.to_statement_frs();
    let c = circuit_for(w, C6PushStatementBindingMode::LedgerCall);
    let randomness = Fr::from(0u64);
    let mut stale_inputs = base.clone();
    stale_inputs.extend(base[post_state_start(w)..].iter().copied());
    let stale = transient_commit(&stale_inputs, randomness);
    // Relation-valid mutation only: preserved initial-fuel lane
    // (image lane 26, pre==post; cf. raw_fr_gate_ref_and_slice_domains
    // accepted u128::MAX case). No helper/relation lane changes, so
    // SDK preprocessing reaches its final CC comparison (instructions
    // and transcript checks precede it in IrSource::preprocess).
    let mut mutated = base.clone();
    mutated[pre_image_at(w, 26)] = Fr::from(MAX);
    mutated[post_image_at(w, 26)] = Fr::from(MAX);
    let mut current_inputs = mutated.clone();
    current_inputs.extend(mutated[post_state_start(w)..].iter().copied());
    assert_ne!(transient_commit(&current_inputs, randomness), stale,
        "stale-CC premise: mutated statement must change the commitment");
    // Paired positive: exact same mutated statement/output, CURRENT
    // recomputed commitment (comm_override=None) must pass.
    match check_raw(&c, w, mutated.clone(), None) {
        Ok(_) => {}
        Err(C6ScalarArithmeticConsumerError::Backend(m)) => panic!(
            "stale-CC paired positive (recomputed CC) must pass: {m}"),
        other => panic!("stale-CC paired positive: unexpected ok={}", other.is_ok()),
    }
    // Same mutated statement/output with only the OLD commitment:
    // rejection must be the SDK communications-commitment mismatch,
    // not the generic preprocessing prefix alone.
    match check_raw(&c, w, mutated, Some(stale)) {
        Err(C6ScalarArithmeticConsumerError::Backend(m)) => assert!(
            m.contains("Communications commitment mismatch"),
            "stale CC must fail at CC comparison: {m}"),
        other => panic!("stale CC: expected rejection, ok={}", other.is_ok()),
    }
    let _: C6ScalarArithmeticConsumerError = C6ScalarArithmeticConsumerError::UnexpectedOwnerStep;
}
