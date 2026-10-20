//! Independent raw negatives for `c6_scalar_entry_consumer`.
//!
//! Source-only and UNRUN. Genuine baselines are admitted through the
//! real public owner (`host_owner_engine().admit` / `admit_ext_program`
//! + `admit_run`) and searched with the real public
//! `complete_observation::record_step` until the actual BEFORE-Entry
//! pre-state: helper absent, result absent, no tasks, `Running`,
//! positive fuel, `ControlC::Return`, and a selected TOP frame. The
//! existing Tag `is_selected` helper is NOT used: it chooses AFTER
//! ScalarEntry (Scalar `First` helper), not this BEFORE-Entry state.
//! No private `MachineImage` is mutated, no `HostSnapshot` or
//! `CompleteStepRecord` is synthesized for a positive baseline, no
//! frame/helper field is patched into a fabricated state, and no
//! impossible helper-work cap is chosen. Mutations of a captured
//! actual pair appear only below, labelled as independent negatives.
//!
//! Category discipline: a mutated witness checked through
//! `check_c6_scalar_entry_step_circuit` recomputes the CURRENT
//! statement and CURRENT post-flat communication commitment inside
//! that checker, so an `Err` there is a relation (SDK preprocessing)
//! rejection. The stale-commitment test builds its own preimage with
//! the OLD commitment on purpose and is a distinct category; it is
//! never counted as a relation negative. Host-prefix / length
//! refusals from the producer/checker shell are asserted as their
//! own typed categories, distinct from the SDK string.
//!
//! REQUIRED_OPEN, not claimed by this child: exhaustive raw mutation
//! of every one of the 16 operand groups across all five composed
//! namespaces (`local`, `c6data`, `c6helper`, `c6meta`, `c6progctx`)
//! as raw statement lanes; every individual PROG/CTX lane; the
//! 129-bit derived slice-end boundary inside the composed DATA
//! canonical block (no 129-bit receipt is invented here and mere SDK
//! bit-count behaviour is not presented as testing it); every old
//! DATA table row beyond the single first active row tag lane covered
//! below, every other lane of that row, and every frame slot; the
//! full genuine 11-shape positive sibling, which remains REQUIRED
//! SEPARATE source; and all inherited
//! SI08/SI15/SI17/SI20/SI21/SI22/`StepInv`/program membership/native
//! items.

use super::{
    build_c6_scalar_entry_step_circuit, check_c6_scalar_entry_step_circuit,
    record_scalar_entry_step, witness_from_host_pair as witness_entry_from_host_pair,
    witness_from_owner_record as witness_entry_from_owner_record,
    C6ScalarEntryConsumerError, C6ScalarEntryStepCircuit, C6ScalarEntryStepWitness,
};
use super::super::c6_push_step::C6PushStatementBindingMode;
use super::super::c6_scalar_entry_step::{
    C6ScalarEntryStepEmitter, C6ScalarEntryStepError, C6ScalarEntryStepInputs,
};
use midnight_transient_crypto::curve::Fr;
use midnight_transient_crypto::hash::transient_commit;
use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::Identifier;
use mori_core::sherlock::helper_machine::c6_phys::{
    L_CONTROL, L_FRAME_COUNT, L_HELPER_VISITS, L_HELPER_WORK, L_OUTCOME,
    L_REFERENCE_CURSOR, L_REMAINING_WORK, L_CONSUMED_WORK,
};
use mori_core::sherlock::helper_machine::c6_phys_helper::L_HELPER_RECORD;
use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostConfig, C6HostLimits};
use mori_core::sherlock::helper_machine::complete_observation::{
    capture, record_step as owner_record, CaptureStepError, CompleteStepRecord,
    HostSnapshot, ObservedStep,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_admit::{
    admit_ext_program, ExtCapacitiesC,
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

// ---------- labels (no Debug expect on non-Debug error carriers) ----------

fn violation_label(v: &HostViolation) -> String {
    format!("kind={:?}, clause={:?}, rule={:?}, locator={}",
        v.kind, v.clause, v.rule, v.locator.as_deref().unwrap_or("<none>"))
}
fn capture_label(e: &CaptureStepError) -> String {
    match e {
        CaptureStepError::BeforeObservation(v) =>
            format!("before observation: {}", violation_label(v)),
        CaptureStepError::Step { violation, after, .. } => match after {
            Ok(_) => format!("owner step: {}; after captured", violation_label(violation)),
            Err(av) => format!("owner step: {}; after observation: {}",
                violation_label(violation), violation_label(av)),
        },
        CaptureStepError::AfterObservation { violation, event, .. } => format!(
            "after observation: {}; event_present={}", violation_label(violation), event.is_some()),
    }
}
fn tooling_label(e: &Tooling) -> String {
    match e {
        Tooling::AdmissionRejected { reason, detail, locator } =>
            format!("admission rejected: reason={reason:?}, detail={detail}, locator={locator:?}"),
        Tooling::CapacityNotAdmitted { kind, needed, limit, locator } =>
            format!("capacity not admitted: kind={kind:?}, needed={needed:?}, limit={limit}, locator={locator:?}"),
        Tooling::UnsupportedFeature { feature, locator } =>
            format!("unsupported feature: feature={feature:?}, locator={locator:?}"),
    }
}

fn ext_label(e: &ExtTypeBuildError) -> String {
    match e {
        ExtTypeBuildError::CoreTooling(t) => tooling_label(t),
        ExtTypeBuildError::ExtCapacityNotAdmitted { kind, needed, limit, locator } => format!(
            "ext capacity: kind={kind:?}, needed={needed:?}, limit={limit}, locator={locator:?}"
        ),
        ExtTypeBuildError::ExtAdmissionRejected { reason, detail, locator } => format!(
            "ext admission rejected: reason={reason:?}, detail={detail}, locator={locator:?}"
        ),
        ExtTypeBuildError::ExtUnsupportedFeature { feature, locator } => format!(
            "ext unsupported feature: feature={feature:?}, locator={locator:?}"
        ),
        ExtTypeBuildError::Host(v) => format!("ext host violation: {}", violation_label(v)),
    }
}

// ---------- retained fixture config (actual supplied fixture API) ----------

fn bounds(max_helper_work: u128) -> BoundsC {
    BoundsC { code_rows: 64, function_rows: 8, code_child_slots: 64,
        parameter_slots: 16, pattern_rows: 16, match_plan_rows: 16,
        match_arm_rows: 16, type_rows: 16, constructor_rows: 16,
        constructor_field_slots: 16, max_arity: 4, constant_nodes: 64,
        constant_field_slots: 64, data_nodes: 16, data_field_slots: 16,
        environment_cells: 16, frames: 16, helper_tasks: 4,
        serialization_bytes: 1_000_000, max_reference_work: 1_000_000,
        max_helper_work, step_cap: 1_000_000 }
}
fn capacities() -> LayoutCapacitiesC {
    LayoutCapacitiesC { name_rows: 32, name_bytes: 512,
        occurrence_path_slots: 64, value_slots: 512 }
}
fn layout(b: BoundsC, ext: Option<ExtCapacitiesC>) -> LayoutSchemaC {
    LayoutSchemaC { layout_version: ("lv".into(), "1".into()), field_id: None,
        widths: None, scalar_layout_id: None, bounds: b,
        capacities: capacities(), ext_capacities: ext,
        boundary_capacities_present: false }
}
fn cfg(b: BoundsC, ext: Option<ExtCapacitiesC>) -> C6HostConfig {
    C6HostConfig::from_retained_layout(&layout(b, ext),
        mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets {
            type_expr_nodes: 256, template_bytes: 4096 })
}
fn limits() -> C6HostLimits {
    C6HostLimits { max_total_lanes: 1 << 20, max_state_lanes: 1 << 20,
        max_metadata_bytes: 8192, max_string_bytes: 512, max_width_entries: 64 }
}

// ---------- fresh program factories (fresh carriers on every replay) ----------

fn lit_u(n: u128) -> CoreExpr {
    CoreExpr::Literal { value: CoreValue::UInt { n: i128_class::KIntC::InRange(n) } }
}
fn make_right_add() -> CoreProgram {
    CoreProgram { functions: Vec::new(), entry: CoreExpr::Binary {
        op: BinaryOp::Add, left: Box::new(lit_u(2)), right: Box::new(lit_u(3)) } }
}

fn make_mul_data_left() -> CoreProgram {
    // The inner Add is genuinely executed before the selected outer
    // Mul Entry. Its ScalarExit allocates the Add result as Data(0).
    CoreProgram { functions: Vec::new(), entry: CoreExpr::Binary {
        op: BinaryOp::Mul,
        left: Box::new(CoreExpr::Binary {
            op: BinaryOp::Add, left: Box::new(lit_u(1)), right: Box::new(lit_u(2)) }),
        right: Box::new(lit_u(3)) } }
}
fn make_ctor_fields() -> ExtProgram {
    ExtProgram {
        types: vec![TypeDeclSrc { name: "Alpha".into(), parameters: Vec::new(),
            ctors: vec![CtorDeclSrc { name: "MkAlpha".into(),
                fields: vec![CtorFieldSrc { name: "x".into(), ty: TypeExprSrc::UInt }] }] }],
        assets: Vec::new(), states: Vec::new(), inputs: Vec::new(),
        functions: Vec::new(),
        entry: ExtExpr::Construct {
            ty: TypeExprSrc::Named { name: "Alpha".into() }, ctor: "MkAlpha".into(),
            fields: vec![ExtExpr::Literal { value: ExtValue::Core {
                value: CoreValue::UInt { n: i128_class::KIntC::InRange(7) } } }] },
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind { RightAdd, MulDataLeft, CtorFields }

// BEFORE-Entry selection, authored from Return + top frame + helper
// absent. This is deliberately not the Tag AFTER-Entry predicate.
fn is_entry_pre(s: &HostSnapshot, kind: Kind) -> bool {
    let img = &s.image;
    if img.helper.is_some() || img.result.is_some() || !img.tasks.is_empty()
        || img.outcome != OutcomeC::Running || img.remaining_work == 0 {
        return false;
    }
    if !matches!(img.control, ControlC::Return { .. }) { return false; }
    match (kind, img.frames.last()) {
        (Kind::RightAdd, Some(FrameRowC::RightF { op: BinaryOp::Add, .. })) => true,
        // Inner Add has RightF{Add}; only the later outer RightF{Mul},
        // whose left Ref is the allocated Data result, is selected.
        (Kind::MulDataLeft, Some(FrameRowC::RightF { op: BinaryOp::Mul, .. })) => true,
        (Kind::CtorFields,
            Some(FrameRowC::ConstructorFieldsF { checked: false, .. })) => true,
        _ => false,
    }
}

fn admit_right_add() -> (Admitted, MachineImage, C6HostConfig) {
    let b = bounds(100);
    let engine = host_owner_engine();
    let admitted = engine.admit(&make_right_add(), &b, &layout(b, None))
        .unwrap_or_else(|e| panic!("right_add admit: {}", tooling_label(&e)));
    let image = engine.admit_run(&admitted, &RunIntC::Nat(64), &RunIntC::Nat(8),
        &FinanceInput::Absent)
        .unwrap_or_else(|e| panic!("right_add genesis: {}", tooling_label(&e)));
    (admitted, image, cfg(b, None))
}

fn admit_mul_data_left() -> (Admitted, MachineImage, C6HostConfig) {
    let b = bounds(100);
    let engine = host_owner_engine();
    let admitted = engine.admit(&make_mul_data_left(), &b, &layout(b, None))
        .unwrap_or_else(|e| panic!("mul_data_left admit: {}", tooling_label(&e)));
    let image = engine.admit_run(&admitted, &RunIntC::Nat(64), &RunIntC::Nat(8),
        &FinanceInput::Absent)
        .unwrap_or_else(|e| panic!("mul_data_left genesis: {}", tooling_label(&e)));
    (admitted, image, cfg(b, None))
}
fn admit_ctor_fields() -> (Admitted, MachineImage, C6HostConfig) {
    let b = bounds(100);
    let (admitted, _report) = admit_ext_program(make_ctor_fields(),
        ExtDomain::GENERIC_ABSENT, &b,
        &layout(b, Some(ExtCapacitiesC::all_zero())), None)
        .unwrap_or_else(|e| panic!("ctor_fields admit_ext_program: {}", ext_label(&e)));
    let engine = host_owner_engine();
    let image = engine.admit_run(&admitted, &RunIntC::Nat(64), &RunIntC::Nat(8),
        &FinanceInput::Absent)
        .unwrap_or_else(|e| panic!("ctor_fields genesis: {}", tooling_label(&e)));
    (admitted, image, cfg(b, Some(ExtCapacitiesC::all_zero())))
}

struct Pair {
    before: HostSnapshot,
    after: HostSnapshot,
    witness: C6ScalarEntryStepWitness,
    host_cfg: C6HostConfig,
}

fn genuine_pair(kind: Kind) -> Pair {
    let (admitted, mut image, host_cfg) = match kind {
        Kind::RightAdd => admit_right_add(),
        Kind::MulDataLeft => admit_mul_data_left(),
        Kind::CtorFields => admit_ctor_fields(),
    };
    let lim = limits();
    for _ in 0..512 {
        let before = capture(&admitted, &image)
            .unwrap_or_else(|e| panic!("capture pre: {}", violation_label(&e)));
        if is_entry_pre(&before, kind) {
            let witness = record_scalar_entry_step(&admitted, &mut image, &host_cfg, &lim)
                .unwrap_or_else(|e| panic!("record_scalar_entry_step: {e}"));
            let after = capture(&admitted, &image)
                .unwrap_or_else(|e| panic!("capture post: {}", violation_label(&e)));
            return Pair { before, after, witness, host_cfg };
        }
        match owner_record(&admitted, &mut image) {
            Ok(ObservedStep::Event(_)) => {}
            Ok(ObservedStep::Terminal { .. }) => panic!("terminal before Entry pre-state"),
            Err(e) => panic!("search step: {}", capture_label(&e)),
        }
    }
    panic!("genuine owner did not reach the BEFORE-Entry pre-state")
}

fn genuine_raw_record(kind: Kind) -> CompleteStepRecord {
    let (admitted, mut image, _c) = match kind {
        Kind::RightAdd => admit_right_add(),
        Kind::MulDataLeft => admit_mul_data_left(),
        Kind::CtorFields => admit_ctor_fields(),
    };
    for _ in 0..512 {
        let before = capture(&admitted, &image)
            .unwrap_or_else(|e| panic!("replay capture: {}", violation_label(&e)));
        if is_entry_pre(&before, kind) {
            return match owner_record(&admitted, &mut image) {
                Ok(ObservedStep::Event(r)) => r,
                Ok(ObservedStep::Terminal { .. }) => panic!("replay terminal at Entry"),
                Err(e) => panic!("replay step: {}", capture_label(&e)),
            };
        }
        match owner_record(&admitted, &mut image) {
            Ok(ObservedStep::Event(_)) => {}
            Ok(ObservedStep::Terminal { .. }) => panic!("replay terminal before Entry"),
            Err(e) => panic!("replay search: {}", capture_label(&e)),
        }
    }
    panic!("replay did not reach the BEFORE-Entry pre-state")
}

// ---------- circuit / check helpers (child access to private fields) ----------

fn circuits(w: &C6ScalarEntryStepWitness) -> (C6ScalarEntryStepCircuit, C6ScalarEntryStepCircuit) {
    let mk = |mode, scope: &str| build_c6_scalar_entry_step_circuit(
        w.source_params().clone(), w.host_prefix().meta.len(),
        w.host_prefix().prog.len(), w.host_prefix().meta_bounds_frames_offset,
        mode, scope).unwrap_or_else(|e| panic!("build circuit {scope}: {e}"));
    (mk(C6PushStatementBindingMode::LocalPublic, "entryrawlocal"),
     mk(C6PushStatementBindingMode::LedgerCall, "entryrawledger"))
}

fn assert_check_ok(w: &C6ScalarEntryStepWitness, label: &str) {
    let (local, ledger) = circuits(w);
    for (name, circuit) in [("LocalPublic", &local), ("LedgerCall", &ledger)] {
        let skips = check_c6_scalar_entry_step_circuit(circuit, w)
            .unwrap_or_else(|e| panic!("{label}/{name}: paired positive must check first: {e}"));
        assert!(skips.iter().all(|s| s.is_none()), "{label}/{name}: unexpected pi skips");
    }
}

fn assert_relation_rejects(w: &C6ScalarEntryStepWitness, label: &str) {
    let (local, ledger) = circuits(w);
    for (name, circuit) in [("LocalPublic", &local), ("LedgerCall", &ledger)] {
        match check_c6_scalar_entry_step_circuit(circuit, w) {
            Ok(_) => panic!("{label}/{name}: mutated witness must be relation-rejected"),
            Err(e) => assert!(format!("{e}").contains("SDK ProofPreimage::check failed"),
                "{label}/{name}: expected SDK relation category, got: {e}"),
        }
    }
}

fn reject_with<F: Fn(&mut C6ScalarEntryStepWitness)>(
    base: &C6ScalarEntryStepWitness, label: &str, f: F,
) {
    let mut w = base.clone();
    f(&mut w);
    assert_relation_rejects(&w, label);
}

// Statement layout offsets, from the consumer's own statement order.
struct Off { pre_img: usize, pre_data: usize, pre_helper: usize,
    post_img: usize, post_data: usize, post_helper: usize, post_base: usize,
    post_prog: usize, post_ctx: usize }
fn offsets(w: &C6ScalarEntryStepWitness) -> Off {
    let g = w.source_params().geometry();
    let m = w.host_prefix().meta.len();
    let p = w.host_prefix().prog.len();
    let pre_img = 4 + m + p + 22;
    let pre_data = pre_img + g.image_total_lanes;
    let pre_helper = pre_data + g.data_total_lanes;
    let post_base = pre_helper + g.helper_total_lanes;
    let post_img = post_base + m + p + 22;
    let post_data = post_img + g.image_total_lanes;
    let post_helper = post_data + g.data_total_lanes;
    let post_prog = post_base + m;
    let post_ctx = post_prog + p;
    Off { pre_img, pre_data, pre_helper, post_img, post_data, post_helper, post_base,
        post_prog, post_ctx }
}

fn raw_check(circuit: &C6ScalarEntryStepCircuit, statement: Vec<Fr>,
    post_flat: Vec<Fr>) -> Result<Vec<Option<usize>>, C6ScalarEntryConsumerError> {
    let preimage = match circuit.mode {
        C6PushStatementBindingMode::LocalPublic => ProofPreimage {
            inputs: Vec::new(), private_transcript: Vec::new(),
            public_transcript_inputs: statement.clone(),
            public_transcript_outputs: statement,
            binding_input: Fr::from(0u64), communications_commitment: None,
            key_location: KeyLocation("sherlock_c6_scalar_entry_step_check".into()) },
        C6PushStatementBindingMode::LedgerCall => {
            let randomness = Fr::from(0u64);
            let mut comm = statement.clone();
            comm.extend(post_flat.iter().copied());
            ProofPreimage { inputs: statement.clone(), private_transcript: Vec::new(),
                public_transcript_inputs: statement, public_transcript_outputs: Vec::new(),
                binding_input: Fr::from(0u64),
                communications_commitment: Some((transient_commit(&comm, randomness), randomness)),
                key_location: KeyLocation("sherlock_c6_scalar_entry_step_check".into()) }
        }
    };
    preimage.check(&circuit.ir)
        .map_err(|e| C6ScalarEntryConsumerError::Backend(
            format!("SDK ProofPreimage::check failed: {e}")))
}

fn post_flat_of(statement: &[Fr], w: &C6ScalarEntryStepWitness) -> Vec<Fr> {
    let o = offsets(w);
    let g = w.source_params().geometry();
    statement[o.post_img..o.post_img + g.state_total_lanes].to_vec()
}

fn fr_2pow128() -> Fr {
    let mut bytes = [0u8; 32];
    bytes[16] = 1;
    Fr::from_le_bytes(&bytes).expect("2^128 is a valid Fr by byte construction")
}

const REC: usize = L_HELPER_RECORD; // 4: helper record base in helper lanes

fn assert_applied_shape(p: &Pair, site: u128, mode: u128,
    exp_present: u128, kind: Kind) {
    let w = &p.witness;
    assert_eq!(w.pre_helper()[REC], 0, "pre helper must be absent");
    assert_eq!(w.post_helper()[REC], 1, "post helper kind must be Scalar");
    assert_eq!(w.post_helper()[REC + 1], site, "installed site");
    assert_eq!(w.post_helper()[REC + 2], w.pre_image()[L_REFERENCE_CURSOR],
        "origin_reference_cursor must equal pre.reference_cursor");
    assert_eq!(w.post_helper()[REC + 3], 0, "parent_resume present flag");
    assert_eq!(w.post_helper()[REC + 4], 0, "parent_resume index padding");
    assert_eq!(w.post_helper()[REC + 5], mode, "installed mode");
    assert_eq!(w.post_helper()[REC + 11], exp_present, "expected presence");
    assert_eq!(w.post_helper()[REC + 13], 0, "phase First");
    assert_eq!(w.post_helper()[REC + 14], 0, "reg_a");
    assert_eq!(w.post_helper()[REC + 15], 0, "staged tag None");
    assert_eq!(w.post_helper()[REC + 16], 0, "staged payload");
    assert_eq!(w.post_image()[L_REMAINING_WORK], w.pre_image()[L_REMAINING_WORK] - 1);
    assert_eq!(w.post_image()[L_CONSUMED_WORK], w.pre_image()[L_CONSUMED_WORK] + 1);
    assert_eq!(w.post_image()[L_HELPER_WORK], w.pre_image()[L_HELPER_WORK] + 1);
    assert_eq!(w.post_image()[L_HELPER_VISITS], w.pre_image()[L_HELPER_VISITS],
        "helper_visits: Entry adds zero visits");
    assert_eq!(w.post_image()[L_REFERENCE_CURSOR], w.pre_image()[L_REFERENCE_CURSOR]);
    assert_eq!(w.post_data(), w.pre_data(), "DATA preserved");
    // Full frames/control preserved: every image lane outside the
    // charged counters and outcome is identical.
    for lane in 0..w.pre_image().len() {
        if ![L_OUTCOME, L_OUTCOME + 1, L_OUTCOME + 2, L_REMAINING_WORK,
            L_CONSUMED_WORK, L_HELPER_WORK].contains(&lane) {
            assert_eq!(w.post_image()[lane], w.pre_image()[lane],
                "preserved image lane {lane}");
        }
    }
    let _ = kind;
}

// ---------- paired positives first (both modes) ----------

#[test]
fn paired_positive_right_add_both_modes() {
    let p = genuine_pair(Kind::RightAdd);
    assert_applied_shape(&p, 11, 0, 0, Kind::RightAdd);
    // Exact Ref identity: operand A is the RightF left Ref, B is the
    // Return value Ref (control-relative lanes at L_CONTROL+1/+2).
    assert_eq!(p.witness.post_helper()[REC + 8], 1, "operand B present");
    assert_check_ok(&p.witness, "right_add");
    // Independent replay through the raw owner recorder agrees exactly.
    let record = genuine_raw_record(Kind::RightAdd);
    assert_eq!(record.before, p.before, "replay before must equal consumer before");
    assert_eq!(record.after, p.after, "replay after must equal consumer after");
    let by_record = witness_entry_from_owner_record(&record, &p.host_cfg, &limits())
        .unwrap_or_else(|e| panic!("witness_from_owner_record: {e}"));
    let by_pair = witness_entry_from_host_pair(true, &record.before, &record.after,
        &p.host_cfg, &limits())
        .unwrap_or_else(|e| panic!("witness_from_host_pair: {e}"));
    assert_eq!(by_record, p.witness, "owner-record route must equal consumer witness");
    assert_eq!(by_pair, p.witness, "host-pair route must equal consumer witness");
    assert_check_ok(&by_record, "right_add_record_route");
}

#[test]
fn paired_positive_ctor_fields_both_modes() {
    let p = genuine_pair(Kind::CtorFields);
    // RetCtorFields (25), TestType (9), expected Some(field_type = UInt = 0).
    assert_applied_shape(&p, 25, 9, 1, Kind::CtorFields);
    assert_eq!(p.witness.post_helper()[REC + 12], 0, "exact field_type TypeId copy");
    assert_eq!(p.witness.post_helper()[REC + 8], 0, "operand B absent");
    assert_check_ok(&p.witness, "ctor_fields");
    let record = genuine_raw_record(Kind::CtorFields);
    assert_eq!(record.before, p.before);
    assert_eq!(record.after, p.after);
}

#[test]
fn raw_active_data_row_preservation_both_modes() {
    let p = genuine_pair(Kind::MulDataLeft);
    let w = &p.witness;

    // Full DATA codec partition: header lanes 0..6, count at lane 3,
    // first DATA row at lane 7 with five lanes per row.
    assert_eq!(w.pre_data()[3], 1,
        "genuine inner Add must allocate exactly one ACTIVE DATA row before outer Entry");
    assert_eq!(w.post_data()[3], 1,
        "outer Entry post-state must retain exactly one ACTIVE DATA row");
    assert_eq!(w.pre_data(), w.post_data(), "Entry must preserve DATA exactly");
    assert!(w.source_params().geometry().caps.phys.max_data_nodes > 0,
        "active DATA row slot 0 must be within the retained maximum");
    assert_eq!(w.pre_data()[7], 0,
        "first ACTIVE DATA row at DATA_HEADER_LANES must have U128 tag 0");
    assert_eq!(w.pre_data()[8], 3,
        "first ACTIVE DATA row payload must be the genuine inner result 1 + 2");
    assert_eq!(w.post_data()[7], 0,
        "post first ACTIVE DATA row tag must remain U128");
    assert_eq!(w.post_data()[8], 3,
        "post first ACTIVE DATA row payload must remain 3");

    // The installed outer helper copies the exact active Ref identity.
    assert_eq!(w.post_helper()[REC + 6], 1,
        "outer Mul operand A must be in Data space");
    assert_eq!(w.post_helper()[REC + 7], 0,
        "outer Mul operand A must be exactly Data(0)");

    assert_check_ok(w, "active_data_row");

    // Witness-lane mutation: relation rejection in both modes, with
    // check_c6_scalar_entry_step_circuit recomputing the current CC.
    reject_with(w, "mutated first ACTIVE DATA row tag", |m| {
        m.post_data[7] = 1;
    });

    // Raw Fr statement mutation of the same actual active lane.
    let (local, ledger) = circuits(w);
    let o = offsets(w);
    let stmt = w.to_statement_frs();
    for (name, circuit) in [("LocalPublic", &local), ("LedgerCall", &ledger)] {
        let mut s = stmt.clone();
        s[o.post_data + 7] = Fr::from(1u128);
        let pf = post_flat_of(&s, w);
        let err = raw_check(circuit, s, pf)
            .err()
            .unwrap_or_else(|| panic!("mutated ACTIVE DATA row tag must reject in {name}"));
        assert!(format!("{err}").contains("SDK ProofPreimage::check failed"),
            "ACTIVE DATA rejection must be the SDK relation category in {name}, got {err}");
    }
}

// ---------- installed-helper domain negatives (witness lanes) ----------

#[test]
fn raw_wrong_installed_helper_domain() {
    let base = genuine_pair(Kind::RightAdd).witness;
    assert_check_ok(&base, "installed_base");
    reject_with(&base, "wrong site", |w| { w.post_helper[REC + 1] = 12; });
    reject_with(&base, "wrong mode", |w| { w.post_helper[REC + 5] = 1; });
    reject_with(&base, "wrong operand A space", |w| {
        w.post_helper[REC + 6] = 1 - w.post_helper[REC + 6]; });
    reject_with(&base, "wrong operand A index", |w| {
        w.post_helper[REC + 7] += 1; });
    reject_with(&base, "wrong operand A full-ref swap to return ref", |w| {
        w.post_helper[REC + 6] = w.pre_image()[L_CONTROL + 1];
        w.post_helper[REC + 7] = w.pre_image()[L_CONTROL + 2]; });
    reject_with(&base, "wrong operand B presence", |w| {
        w.post_helper[REC + 8] = 0; w.post_helper[REC + 9] = 0;
        w.post_helper[REC + 10] = 0; });
    reject_with(&base, "wrong operand B index", |w| {
        w.post_helper[REC + 10] += 1; });
    reject_with(&base, "wrong expected presence on Right", |w| {
        w.post_helper[REC + 11] = 1; w.post_helper[REC + 12] = 0; });
    reject_with(&base, "wrong common origin cursor", |w| {
        w.post_helper[REC + 2] += 1; });
    reject_with(&base, "wrong parent_resume present", |w| {
        w.post_helper[REC + 3] = 1; });
    reject_with(&base, "wrong phase", |w| { w.post_helper[REC + 13] = 1; });
    reject_with(&base, "wrong reg_a", |w| { w.post_helper[REC + 14] = 1; });
    reject_with(&base, "wrong staged", |w| { w.post_helper[REC + 15] = 1; });
    reject_with(&base, "helper kind cleared", |w| { w.post_helper[REC] = 0; });
    reject_with(&base, "result record installed", |w| {
        w.post_helper[27] = 10; });
    reject_with(&base, "task record installed", |w| {
        w.post_helper[3] = 1; });
}

#[test]
fn raw_wrong_expected_on_ctor_fields() {
    let base = genuine_pair(Kind::CtorFields).witness;
    assert_check_ok(&base, "ctor_expected_base");
    reject_with(&base, "wrong expected value nominal-vs-builtin", |w| {
        w.post_helper[REC + 12] = 4; });
    reject_with(&base, "wrong expected presence on CtorFields", |w| {
        w.post_helper[REC + 11] = 0; w.post_helper[REC + 12] = 0; });
    reject_with(&base, "wrong site on CtorFields", |w| {
        w.post_helper[REC + 1] = 16; });
}

// ---------- counters, P3, preservation ----------

#[test]
fn raw_wrong_counters_and_preservation() {
    let base = genuine_pair(Kind::RightAdd).witness;
    assert_check_ok(&base, "counters_base");
    reject_with(&base, "remaining not charged", |w| {
        let v = w.pre_image()[L_REMAINING_WORK]; w.post_image[L_REMAINING_WORK] = v; });
    reject_with(&base, "consumed not charged", |w| {
        let v = w.pre_image()[L_CONSUMED_WORK]; w.post_image[L_CONSUMED_WORK] = v; });
    reject_with(&base, "helper_work not charged", |w| {
        let v = w.pre_image()[L_HELPER_WORK]; w.post_image[L_HELPER_WORK] = v; });
    reject_with(&base, "helper_visits charged on Entry", |w| {
        w.post_image[L_HELPER_VISITS] += 1; });
    reject_with(&base, "reference_cursor moved on Entry", |w| {
        w.post_image[L_REFERENCE_CURSOR] += 1; });
    reject_with(&base, "wrong post outcome", |w| { w.post_image[L_OUTCOME] = 1; });
    reject_with(&base, "wrong post control", |w| { w.post_image[L_CONTROL] = 0; });
    reject_with(&base, "wrong post env root", |w| { w.post_image[20] += 1; });
    reject_with(&base, "wrong post frame lane", |w| {
        let depth = w.pre_image()[L_FRAME_COUNT] as usize;
        let top = 48 + (depth - 1) * 13;
        w.post_image[top + 4] += 1; });
    reject_with(&base, "wrong post DATA count lane", |w| { w.post_data[3] += 1; });
    reject_with(&base, "wrong pre top op Eq-not-Entry", |w| {
        let depth = w.pre_image()[L_FRAME_COUNT] as usize;
        let top = 48 + (depth - 1) * 13;
        w.pre_image[top + 1] = 3; });
    reject_with(&base, "pre helper present", |w| { w.pre_helper[REC] = 1; });
    reject_with(&base, "pre outcome not Running", |w| { w.pre_image[L_OUTCOME] = 1; });
}

// Coherent P3 shape built from the actual captured pre (labelled
// independent negative construction): pre.helper_work set to the
// retained admission max (100), disposition true, Exhausted
// HelperWork outcome, helper still absent, no charges.
fn p3_witness(base: &C6ScalarEntryStepWitness) -> C6ScalarEntryStepWitness {
    let mut w = base.clone();
    w.pre_image[L_HELPER_WORK] = 100;
    w.post_image.copy_from_slice(&w.pre_image.to_vec());
    w.post_image[L_OUTCOME] = 3;
    w.post_image[L_OUTCOME + 1] = 5;
    w.post_image[L_OUTCOME + 2] = 0;
    w.post_data.copy_from_slice(&w.pre_data.to_vec());
    w.post_helper.copy_from_slice(&w.pre_helper.to_vec());
    w.disposition_is_refused_helper_work = true;
    w
}

#[test]
fn raw_p3_coherent_and_wrong_p3() {
    let base = genuine_pair(Kind::RightAdd).witness;
    let p3 = p3_witness(&base);
    assert_check_ok(&p3, "coherent_p3");
    reject_with(&p3, "P3 wrong disposition flag", |w| {
        w.disposition_is_refused_helper_work = false; });
    reject_with(&p3, "P3 helper installed", |w| {
        let post = base.post_helper().to_vec();
        w.post_helper.copy_from_slice(&post); });
    reject_with(&p3, "P3 outcome Running", |w| {
        w.post_image[L_OUTCOME] = 0; w.post_image[L_OUTCOME + 1] = 0; });
    reject_with(&p3, "P3 charged remaining", |w| {
        w.post_image[L_REMAINING_WORK] -= 1; });
    reject_with(&p3, "P3 helper_work above max is a different pre", |w| {
        w.pre_image[L_HELPER_WORK] = 101; w.post_image[L_HELPER_WORK] = 101; });
}

// A selector change with its coherent companion post is a different
// genuine shape and must pass; the incoherent half alone rejects
// (covered by "wrong mode" / "wrong pre top op" above).
#[test]
fn raw_coherent_selector_companion_passes() {
    let base = genuine_pair(Kind::RightAdd).witness;
    let mut w = base.clone();
    let depth = w.pre_image()[L_FRAME_COUNT] as usize;
    let top = 48 + (depth - 1) * 13;
    w.pre_image[top + 1] = 1; // BinaryOp::Sub in both pre and post frames
    w.post_image[top + 1] = 1;
    w.post_helper[REC + 5] = 1; // ScalarMode::Sub companion
    assert_check_ok(&w, "coherent_sub_shape");
}

// ---------- disabled (gate 0) domain ----------

#[test]
fn raw_disabled_gate0_domain() {
    let base = genuine_pair(Kind::RightAdd).witness;
    let mut off = base.clone();
    off.enable = false;
    off.post_image.copy_from_slice(&off.pre_image.to_vec());
    off.post_data.copy_from_slice(&off.pre_data.to_vec());
    off.post_helper.copy_from_slice(&off.pre_helper.to_vec());
    assert_check_ok(&off, "gate0 honest disabled equality");
    reject_with(&off, "gate0 post differs from pre", |w| {
        w.post_helper[REC] = 1; });
}

// ---------- raw Fr statement mutations (independent Fr route) ----------

#[test]
fn raw_fr_statement_mutations() {
    let p = genuine_pair(Kind::RightAdd);
    let w = &p.witness;
    assert_check_ok(w, "raw_fr_base");
    let (_local, ledger) = circuits(w);
    let o = offsets(w);
    let stmt = w.to_statement_frs();

    // Wrong post META lane 4. POST META begins at post_base.
    let mut s = stmt.clone(); s[o.post_base + 4] = Fr::from(999u128);
    let pf = post_flat_of(&s, w);
    assert!(raw_check(&ledger, s, pf).is_err(), "wrong post META lane must reject");
    // Wrong first post PROG lane. PROG begins at post_base + meta_len.
    let mut s = stmt.clone(); s[o.post_prog] = Fr::from(999u128);
    let pf = post_flat_of(&s, w);
    assert!(raw_check(&ledger, s, pf).is_err(), "wrong post PROG lane must reject");
    // Wrong post CTX lane 4. This asserts CTX+4 only, not CTX+0.
    let mut s = stmt.clone(); s[o.post_ctx + 4] = Fr::from(999u128);
    let pf = post_flat_of(&s, w);
    assert!(raw_check(&ledger, s, pf).is_err(), "wrong post CTX lane 4 must reject");
    // Wrong pre META lane.
    let mut s = stmt.clone(); s[4] = Fr::from(999u128);
    let pf = post_flat_of(&s, w);
    assert!(raw_check(&ledger, s, pf).is_err(), "wrong pre META lane must reject");
    // Wrong config/schema and carrier lanes.
    let mut s = stmt.clone(); s[2] = Fr::from(999u128);
    let pf = post_flat_of(&s, w);
    assert!(raw_check(&ledger, s, pf).is_err(), "wrong host_schema must reject");
    let mut s = stmt.clone(); s[3] = Fr::from(999u128);
    let pf = post_flat_of(&s, w);
    assert!(raw_check(&ledger, s, pf).is_err(), "wrong carrier_version must reject");
    // Wrong first INACTIVE padded DATA row tag. DATA_HEADER_LANES is
    // 7, so lane 7 is DATA row slot 0, not a header lane. This
    // literal/literal RightAdd baseline has no active DATA row and
    // is not an old ACTIVE DATA preservation test.
    assert_eq!(w.pre_data()[3], 0,
        "RightAdd baseline DATA count must be 0 before Entry");
    assert_eq!(w.post_data()[3], 0,
        "RightAdd baseline DATA count must remain 0 after Entry");
    assert!(w.source_params().geometry().caps.phys.max_data_nodes > 0,
        "first inactive padded DATA row slot 0 must be within the retained maximum");
    let mut s = stmt.clone(); s[o.post_data + 7] = Fr::from(1u128);
    let pf = post_flat_of(&s, w);
    assert!(raw_check(&ledger, s, pf).is_err(),
        "wrong first inactive padded DATA row tag must reject");
    // Gate value 2 is not a boolean, at the raw Fr level.
    let mut s = stmt.clone(); s[0] = Fr::from(2u128);
    let pf = post_flat_of(&s, w);
    assert!(raw_check(&ledger, s, pf).is_err(), "enable=2 must reject");
    // Fr >= 2^128 on pre image, post image, and post helper lanes:
    // unconditional 128-bit lane constraints reject even though no
    // u128 witness could carry these values.
    for (label, idx) in [("pre image raw128", o.pre_img + L_HELPER_WORK),
        ("post image raw128", o.post_img + L_HELPER_WORK),
        ("post helper raw128", o.post_helper + REC + 2),
        ("pre helper raw128", o.pre_helper + REC + 2)] {
        let mut s = stmt.clone(); s[idx] = fr_2pow128();
        let pf = post_flat_of(&s, w);
        assert!(raw_check(&ledger, s, pf).is_err(), "{label} must reject");
    }
    // Coherent gate-0 positive control: post equals pre everywhere.
    let mut off_w = w.clone();
    off_w.enable = false;
    off_w.post_image.copy_from_slice(&off_w.pre_image.to_vec());
    off_w.post_data.copy_from_slice(&off_w.pre_data.to_vec());
    off_w.post_helper.copy_from_slice(&off_w.pre_helper.to_vec());
    let off_stmt = off_w.to_statement_frs();
    let (local, _) = circuits(w);
    for (name, circuit) in [("LocalPublic", &local), ("LedgerCall", &ledger)] {
        let pf = post_flat_of(&off_stmt, w);
        assert!(raw_check(circuit, off_stmt.clone(), pf).is_ok(),
            "gate0 coherent equality must pass in {name}");
    }
    // Gate-0 raw128 companion: the same lane is changed in pre and post,
    // so disabled equality still holds and only the unconditional
    // 128-bit lane constraint can reject.
    for (name, circuit) in [("LocalPublic", &local), ("LedgerCall", &ledger)] {
        let mut s = off_stmt.clone();
        s[o.pre_img + L_HELPER_WORK] = fr_2pow128();
        s[o.post_img + L_HELPER_WORK] = fr_2pow128();
        let pf = post_flat_of(&s, w);
        assert!(raw_check(circuit, s, pf).is_err(), "gate0 raw128 must reject in {name}");
    }
}

#[test]
fn raw_stale_commitment_is_distinct_category() {
    let p = genuine_pair(Kind::RightAdd);
    let w = &p.witness;
    let (_local, ledger) = circuits(w);
    let stmt = w.to_statement_frs();
    let good_pf = post_flat_of(&stmt, w);

    // Positive control: the unchanged, relation-valid statement with
    // its current LedgerCall commitment passes.
    raw_check(&ledger, stmt.clone(), good_pf.clone())
        .unwrap_or_else(|e| panic!("current statement/current CC must pass: {e}"));

    // Distinct relation negative: mutate the statement and let
    // raw_check recompute the current commitment. This must not be
    // reported as a stale-commitment failure.
    let o = offsets(w);
    let mut related = stmt.clone();
    related[o.post_helper + REC + 1] = Fr::from(12u128);
    let related_pf = post_flat_of(&related, w);
    let related_err = raw_check(&ledger, related, related_pf)
        .err()
        .unwrap_or_else(|| panic!("mutated statement with current CC must relation-reject"));
    assert!(
        !format!("{related_err}").contains("Communications commitment mismatch"),
        "relation negative must not wear the stale-CC category: {related_err}"
    );

    // Stale-CC-only negative: present the unchanged valid statement,
    // but retain a commitment computed from a different statement.
    let randomness = Fr::from(0u64);
    let mut stale_basis = stmt.clone();
    stale_basis[o.post_helper + REC + 1] = Fr::from(12u128);
    stale_basis.extend(good_pf.iter().copied());
    let stale = ProofPreimage { inputs: stmt.clone(), private_transcript: Vec::new(),
        public_transcript_inputs: stmt, public_transcript_outputs: Vec::new(),
        binding_input: Fr::from(0u64),
        communications_commitment: Some((transient_commit(&stale_basis, randomness), randomness)),
        key_location: KeyLocation("sherlock_c6_scalar_entry_step_check".into()) };
    let stale_err = stale.check(&ledger.ir)
        .err()
        .unwrap_or_else(|| panic!("stale commitment must reject"));
    assert!(
        format!("{stale_err}").contains("Communications commitment mismatch"),
        "stale-CC negative must reach the commitment category, got: {stale_err}"
    );
}

#[test]
fn raw_producer_refusal_categories_are_distinct() {
    let p = genuine_pair(Kind::RightAdd);
    let (local, _ledger) = circuits(&p.witness);
    // Checker-shell length refusal, before any SDK preprocessing.
    let mut w = p.witness.clone();
    w.host_prefix.meta.pop();
    match check_c6_scalar_entry_step_circuit(&local, &w) {
        Err(C6ScalarEntryConsumerError::LengthMismatch { section: "meta", .. }) => {}
        other => panic!("expected typed meta LengthMismatch producer category, got {other:?}"),
    }
    // Host-pair prefix refusal: a post snapshot whose layout no longer
    // matches the retained config is a producer error, never an SDK
    // relation result.
    let mut bad_post = p.after.clone();
    bad_post.admission.layout.bounds.frames += 1;
    match witness_entry_from_host_pair(true, &p.before, &bad_post, &p.host_cfg, &limits()) {
        Err(e) => assert!(!format!("{e}").contains("SDK ProofPreimage::check failed"),
            "producer refusal must not wear the SDK category: {e}"),
        Ok(_) => panic!("mismatched post prefix must be producer-refused"),
    }
}

// ---------- namespace / retained-serial discipline ----------

fn operand_vec(lanes: &[u128]) -> Vec<Operand> {
    lanes.iter().map(|&v| Operand::Immediate(Fr::from(v))).collect()
}
fn entry_inputs(w: &C6ScalarEntryStepWitness) -> C6ScalarEntryStepInputs {
    let hp = w.host_prefix();
    let ctx: [Operand; 22] = operand_vec(&hp.ctx).try_into()
        .expect("ctx lane count is fixed by the consumer statement");
    C6ScalarEntryStepInputs {
        enable: Operand::Immediate(Fr::from(w.enable())),
        disposition_is_refused_helper_work:
            Operand::Immediate(Fr::from(w.disposition_is_refused_helper_work())),
        host_schema: Operand::Immediate(Fr::from(
            mori_core::sherlock::helper_machine::c6_phys_host::C6_HOST_SCHEMA)),
        carrier_version: Operand::Immediate(Fr::from(1u128)),
        geometry: w.source_params().geometry(),
        meta_bounds_frames_offset: hp.meta_bounds_frames_offset,
        source_params: w.source_params().clone(),
        pre_meta: operand_vec(&hp.meta), pre_prog: operand_vec(&hp.prog),
        pre_ctx: ctx.clone(),
        pre_image: operand_vec(w.pre_image()), pre_data: operand_vec(w.pre_data()),
        pre_helper: operand_vec(w.pre_helper()),
        post_meta: operand_vec(&hp.meta), post_prog: operand_vec(&hp.prog),
        post_ctx: ctx,
        post_image: operand_vec(w.post_image()), post_data: operand_vec(w.post_data()),
        post_helper: operand_vec(w.post_helper()),
    }
}

fn local_temp_ids(
    instructions: &[midnight_zkir_v3::Instruction],
) -> std::collections::BTreeSet<String> {
    use midnight_zkir_v3::Instruction as I;
    let mut out = std::collections::BTreeSet::new();
    for instruction in instructions {
        let outputs: Vec<&Identifier> = match instruction {
            I::Add { output, .. }
            | I::Mul { output, .. }
            | I::Neg { output, .. }
            | I::Inv { output, .. }
            | I::Not { output, .. }
            | I::TestEq { output, .. }
            | I::LessThan { output, .. }
            | I::CondSelect { output, .. }
            | I::Copy { output, .. }
            | I::PublicInput { output, .. }
            | I::PrivateInput { output, .. } => vec![output],
            I::Encode { outputs, .. } | I::DivModPowerOfTwo { outputs, .. } => {
                outputs.iter().collect()
            }
            I::IntoCoordinates { outputs, .. } => vec![&outputs.0, &outputs.1],
            I::Bytes32IntoLowHigh { outputs, .. } => vec![&outputs.0, &outputs.1],
            _ => Vec::new(),
        };
        for id in outputs {
            if id.0.contains(".local.temp.") {
                out.insert(id.0.clone());
            }
        }
    }
    out
}

#[test]
fn raw_namespace_reserved_and_repeated_serials() {
    let w = genuine_pair(Kind::RightAdd).witness;
    // Reserved composed operand under the c6data prefix is host-refused.
    let mut em = C6ScalarEntryStepEmitter::new("entryrawns")
        .unwrap_or_else(|e| panic!("emitter new: {e}"));
    let mut inputs = entry_inputs(&w);
    inputs.enable = Operand::Variable(Identifier("%entryrawns.c6data.temp.0".into()));
    match em.emit(&inputs) {
        Err(C6ScalarEntryStepError::ReservedOperand { .. }) => {}
        other => panic!("expected ReservedOperand, got {other:?}"),
    }
    // Each of the five composed segments in a scope is refused.
    for seg in [".local.temp.", ".c6data.temp.", ".c6helper.temp.",
        ".c6meta.temp.", ".c6progctx.temp."] {
        let scope = format!("bad{seg}x");
        assert!(matches!(C6ScalarEntryStepEmitter::new(&scope),
            Err(C6ScalarEntryStepError::InvalidScope)),
            "scope containing {seg} must be refused");
    }
    // Repeated retained serials: two emits under one retained emitter
    // both succeed and never reuse a local temporary identifier.
    let mut em = C6ScalarEntryStepEmitter::new("entryrawserial")
        .unwrap_or_else(|e| panic!("emitter new: {e}"));
    let inputs = entry_inputs(&w);
    let first = em.emit(&inputs).unwrap_or_else(|e| panic!("first emit: {e}"));
    let after_first = em.next_temp();
    let second = em.emit(&inputs).unwrap_or_else(|e| panic!("second emit: {e}"));
    assert!(em.next_temp() > after_first && after_first > 0,
        "retained serial must advance across emits");
    let a = local_temp_ids(&first.instructions);
    let b = local_temp_ids(&second.instructions);
    assert!(!a.is_empty() && a.is_disjoint(&b),
        "repeated retained emits must not reuse local temporaries");
}
