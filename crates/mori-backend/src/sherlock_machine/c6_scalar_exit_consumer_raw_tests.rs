//! Raw-Fr / statement-level negative tests for `c6_scalar_exit_consumer`
//! (source-only, UNRUN). Genuine U128/Add fixtures are admitted and
//! stepped through the real public owner exactly as the owner tests do;
//! every other fixture is a mutation of that genuine witness, or a
//! direct raw `Vec<Fr>` statement for values the u128 witness API
//! cannot express (2^128 and above). Rejection means the actual SDK
//! `ProofPreimage::check` preprocessing category ("SDK
//! ProofPreimage::check failed"), never a producer-only filter, never
//! a caught panic. This is preprocessing only: not synthesis, proving,
//! MockProver, or native execution. SI08/SI21/SI22 denotation, StepInv,
//! program membership and native P1 issuance remain REQUIRED_OPEN; in
//! particular no test here asserts that a staged U128Value whose
//! payload differs from the K-denotation of its operands is rejected.

use super::{
    build_c6_scalar_exit_step_circuit, capture_step_error_label,
    check_c6_scalar_exit_step_circuit, record_scalar_exit_step, violation_label,
    C6ScalarExitConsumerError, C6ScalarExitStepWitness, CHECK_KEY_LOCATION,
};
use super::super::c6_push_step::C6PushStatementBindingMode;
use super::super::{
    C6ScalarExitStepEmitter, C6ScalarExitStepError, C6ScalarExitStepInputs,
};
use midnight_transient_crypto::curve::Fr;
use midnight_transient_crypto::hash::transient_commit;
use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage, Zkir};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::Identifier;
use mori_core::sherlock::helper_machine::c6_phys::{
    L_ACTIVE, L_ALLOC, L_CONSUMED_WORK, L_CONTROL, L_CONTROL_ORIGIN, L_ENV_ROOT,
    L_FRAME_COUNT, L_HELPER_WORK, L_INITIAL_FUEL, L_OUTCOME, L_REFERENCE_CURSOR,
    L_REMAINING_WORK, L_SEG_ACTIVE, L_SEG_ALLOC, C6_PHYS_MAGIC,
};
use mori_core::sherlock::helper_machine::c6_phys_data::DATA_HEADER_LANES;
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_HELPER_RECORD, L_RESULT_RECORD};
use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostConfig, C6HostLimits, CTX_LANES};
use mori_core::sherlock::helper_machine::complete_observation::{
    capture, record_step as owner_record, HostSnapshot, LayoutSnapshot, ObservedStep,
};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::*;

// ---------- genuine U128/Add fixture (duplicated from the owner tests;
// sibling test modules cannot import each other's private helpers) ----------

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
fn bounds(max_hw: u128, data_nodes: usize, data_fields: usize) -> BoundsC {
    BoundsC {
        code_rows: 64, function_rows: 8, code_child_slots: 64, parameter_slots: 16,
        pattern_rows: 16, match_plan_rows: 16, match_arm_rows: 16, type_rows: 16,
        constructor_rows: 16, constructor_field_slots: 16, max_arity: 4,
        constant_nodes: 64, constant_field_slots: 64, data_nodes,
        data_field_slots: data_fields, environment_cells: 16, frames: 16,
        helper_tasks: 4, serialization_bytes: 1_000_000,
        max_reference_work: 1_000_000, max_helper_work: max_hw, step_cap: 1_000_000,
    }
}
fn capacities() -> LayoutCapacitiesC {
    LayoutCapacitiesC { name_rows: 32, name_bytes: 512,
        occurrence_path_slots: 64, value_slots: 512 }
}
fn layout(b: BoundsC) -> LayoutSchemaC {
    LayoutSchemaC { layout_version: ("lv".into(), "1".into()), field_id: None,
        widths: None, scalar_layout_id: None, bounds: b, capacities: capacities(),
        ext_capacities: None, boundary_capacities_present: false }
}
fn cfg(b: BoundsC) -> C6HostConfig {
    C6HostConfig { layout: LayoutSnapshot {
            layout_version: ("lv".into(), "1".into()), field_id: None, widths: None,
            scalar_layout_id: None, bounds: b, capacities: capacities(),
            boundary_capacities_present: false },
        ext: None,
        budgets: mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets {
            type_expr_nodes: 256, template_bytes: 4096 } }
}
fn limits() -> C6HostLimits {
    C6HostLimits { max_total_lanes: 1 << 20, max_state_lanes: 1 << 20,
        max_metadata_bytes: 8192, max_string_bytes: 512, max_width_entries: 64 }
}
fn generous() -> BoundsC { bounds(100, 16, 16) }
fn lit_u(n: u128) -> CoreExpr {
    CoreExpr::Literal { value: CoreValue::UInt { n: i128_class::KIntC::InRange(n) } }
}
fn u128_program() -> CoreProgram {
    CoreProgram { functions: Vec::new(), entry: CoreExpr::Binary {
        op: BinaryOp::Add, left: Box::new(lit_u(40)), right: Box::new(lit_u(2)) } }
}

fn reach_u128(b: BoundsC) -> (Admitted, MachineImage, HostSnapshot) {
    let engine = host_owner_engine();
    let p = u128_program();
    let admitted = engine.admit(&p, &b, &layout(b))
        .unwrap_or_else(|e| panic!("admit U128: {}", tooling_label(&e)));
    let mut image = engine.admit_run(&admitted, &RunIntC::Nat(64), &RunIntC::Nat(8),
            &FinanceInput::Absent)
        .unwrap_or_else(|e| panic!("genesis U128: {}", tooling_label(&e)));
    for _ in 0..512 {
        let step = owner_record(&admitted, &mut image)
            .unwrap_or_else(|e| panic!("search step: {}", capture_step_error_label(&e)));
        let after = match &step {
            ObservedStep::Event(r) => &r.after,
            ObservedStep::Terminal { after, .. } =>
                panic!("terminal before Scalar Done U128: {:?}", after.image.outcome),
        };
        if let Some(HelperControlC::Scalar {
            phase: ScalarPhase::Done,
            staged: ScalarStaged::U128Value { n: 42 }, .. }) = &after.image.helper {
            let pre = capture(&admitted, &image)
                .unwrap_or_else(|e| panic!("capture selected pre: {}", violation_label(&e)));
            assert_eq!(&pre, after, "capture must equal the searched pre-state");
            return (admitted, image, pre);
        }
    }
    panic!("owner must reach Scalar Done U128Value{{42}}")
}

fn witness_for_bounds(b: BoundsC) -> C6ScalarExitStepWitness {
    let (admitted, mut image, _pre) = reach_u128(b);
    record_scalar_exit_step(&admitted, &mut image, &cfg(b), &limits())
        .expect("selected ScalarExit wrapper step")
}
fn base_witness() -> C6ScalarExitStepWitness {
    let w = witness_for_bounds(generous());
    assert!(!w.disposition_is_refused_helper_work(), "base fixture is Applied");
    assert_ok_both(&w, "genuine base witness");
    w
}

// ---------- both-mode SDK helpers ----------

fn circuit_for(w: &C6ScalarExitStepWitness, mode: C6PushStatementBindingMode)
    -> super::C6ScalarExitStepCircuit {
    build_c6_scalar_exit_step_circuit(w.source_params().clone(),
        w.host_prefix().meta.len(), w.host_prefix().prog.len(),
        w.host_prefix().meta_bounds_frames_offset, mode, "scalar_exit_raw_test")
        .expect("circuit builds")
}
fn assert_ok_both(w: &C6ScalarExitStepWitness, label: &str) {
    for mode in [C6PushStatementBindingMode::LocalPublic,
                 C6PushStatementBindingMode::LedgerCall] {
        let c = circuit_for(w, mode);
        assert!(check_c6_scalar_exit_step_circuit(&c, w).is_ok(),
            "{label}: positive control must pass SDK preprocessing in {mode:?}");
    }
}
fn assert_rejects_both(w: &C6ScalarExitStepWitness, label: &str) {
    for mode in [C6PushStatementBindingMode::LocalPublic,
                 C6PushStatementBindingMode::LedgerCall] {
        let c = circuit_for(w, mode);
        match check_c6_scalar_exit_step_circuit(&c, w) {
            Err(C6ScalarExitConsumerError::Backend(m)) => assert!(
                m.contains("SDK ProofPreimage::check failed"),
                "{label} {mode:?}: rejection must be SDK preprocessing, got: {m}"),
            other => panic!("{label} {mode:?}: expected SDK preprocessing rejection, ok={}",
                other.is_ok()),
        }
    }
}

// ---------- raw-Fr statement harness (values u128 cannot express) ----------

fn two_pow_128() -> Fr { Fr::from(u128::MAX) + Fr::from(1u64) }
fn prefix_len(w: &C6ScalarExitStepWitness) -> usize {
    w.host_prefix().meta.len() + w.host_prefix().prog.len() + CTX_LANES
}
fn state_len(w: &C6ScalarExitStepWitness) -> usize {
    w.pre_image().len() + w.pre_data().len() + w.pre_helper().len()
}
fn pre_state_start(w: &C6ScalarExitStepWitness) -> usize { 4 + prefix_len(w) }
fn post_prefix_start(w: &C6ScalarExitStepWitness) -> usize {
    pre_state_start(w) + state_len(w)
}
fn post_state_start(w: &C6ScalarExitStepWitness) -> usize {
    post_prefix_start(w) + prefix_len(w)
}

/// Mode-aware raw check. For LedgerCall the communications commitment
/// is recomputed from THIS mutated statement plus THIS mutated
/// declared post-state output vector (the statement's post-state
/// segment), so rejection is relation/canonicality, not a stale
/// commitment mismatch. `comm_override` exists only for the explicit
/// commitment-specific negative.
fn check_raw(c: &super::C6ScalarExitStepCircuit, w: &C6ScalarExitStepWitness,
             statement: Vec<Fr>, comm_override: Option<Fr>)
    -> Result<Vec<Option<usize>>, C6ScalarExitConsumerError> {
    let preimage = match c.mode {
        C6PushStatementBindingMode::LocalPublic => ProofPreimage {
            inputs: Vec::new(), private_transcript: Vec::new(),
            public_transcript_inputs: statement.clone(),
            public_transcript_outputs: statement,
            binding_input: Fr::from(0u64), communications_commitment: None,
            key_location: KeyLocation(CHECK_KEY_LOCATION.into()) },
        C6PushStatementBindingMode::LedgerCall => {
            let randomness = Fr::from(0u64);
            let post = statement[post_state_start(w)..].to_vec();
            let mut comm_inputs = statement.clone();
            comm_inputs.extend(post.iter().copied());
            let comm = comm_override
                .unwrap_or_else(|| transient_commit(&comm_inputs, randomness));
            ProofPreimage {
                inputs: statement.clone(), private_transcript: Vec::new(),
                public_transcript_inputs: statement,
                public_transcript_outputs: Vec::new(),
                binding_input: Fr::from(0u64),
                communications_commitment: Some((comm, randomness)),
                key_location: KeyLocation(CHECK_KEY_LOCATION.into()) }
        }
    };
    preimage.check(&c.ir).map_err(|e| C6ScalarExitConsumerError::Backend(
        format!("SDK ProofPreimage::check failed: {e}")))
}
fn assert_raw_ok(w: &C6ScalarExitStepWitness, statement: Vec<Fr>, label: &str) {
    for mode in [C6PushStatementBindingMode::LocalPublic,
                 C6PushStatementBindingMode::LedgerCall] {
        let c = circuit_for(w, mode);
        assert!(check_raw(&c, w, statement.clone(), None).is_ok(),
            "{label}: raw positive control must pass in {mode:?}");
    }
}
fn assert_raw_rejects(w: &C6ScalarExitStepWitness, statement: Vec<Fr>, label: &str) {
    for mode in [C6PushStatementBindingMode::LocalPublic,
                 C6PushStatementBindingMode::LedgerCall] {
        let c = circuit_for(w, mode);
        match check_raw(&c, w, statement.clone(), None) {
            Err(C6ScalarExitConsumerError::Backend(m)) => assert!(
                m.contains("SDK ProofPreimage::check failed"),
                "{label} {mode:?}: must be SDK preprocessing, got: {m}"),
            other => panic!("{label} {mode:?}: expected raw SDK rejection, ok={}",
                other.is_ok()),
        }
    }
}

// section: 0 image, 1 data, 2 helper. Both copies set together so the
// mutation isolates the validator/cap predicate, not preservation.
fn set_both(w: &mut C6ScalarExitStepWitness, section: u8, lane: usize, value: u128) {
    match section {
        0 => { assert_ne!(w.pre_image[lane], value, "image mutation no-op");
               w.pre_image[lane] = value; w.post_image[lane] = value; }
        1 => { assert_ne!(w.pre_data[lane], value, "data mutation no-op");
               w.pre_data[lane] = value; w.post_data[lane] = value; }
        _ => { assert_ne!(w.pre_helper[lane], value, "helper mutation no-op");
               w.pre_helper[lane] = value; w.post_helper[lane] = value; }
    }
}

// Pre-helper-only mutation: the genuine Applied post helper (record
// cleared, exact result) is retained, so a post-helper None-padding
// failure is not introduced as an alternate rejection.
fn set_pre_helper(w: &mut C6ScalarExitStepWitness, lane: usize, value: u128) {
    assert_ne!(w.pre_helper[lane], value, "pre helper mutation no-op");
    w.pre_helper[lane] = value;
}

// Test-only companion builder for the seven staged-domain cases below.
// It rebuilds the post-state from THIS witness's pre-state using only
// the emitter's Applied equations: P4 charges exactly once, Running
// outcome is preserved from the Running pre-state, root alloc/active
// and the DATA header advance by the supplied deltas, the supplied row
// is written at the physical target slot `pre_data[3]`, the supplied
// field refs are appended in order at `pre_data[4]`, the helper record
// is cleared, and the supplied literal result record is installed with
// zero padding. It assumes the caller has already established that the
// fixture is neither P3 nor P6 for the supplied deltas; the assertions
// below check that against the retained admission bounds. This does
// not fabricate an admitted MachineImage: the witness remains a RAW
// mutation of the genuine base witness.
fn finish_derived_applied(
    v: &mut C6ScalarExitStepWitness,
    node_delta: u128,
    field_delta: u128,
    row: [u128; 5],
    field_writes: &[(u128, u128)],
    result: [u128; 3],
) {
    let rec = L_HELPER_RECORD;
    let res = L_RESULT_RECORD;
    let active_data = v.pre_data[3];
    let active_fields = v.pre_data[4];
    assert_eq!(v.pre_image[L_ACTIVE], active_data, "companion active/data tie");
    assert_eq!(v.pre_image[L_ACTIVE + 1], active_fields, "companion active/fields tie");
    assert!(!v.disposition_is_refused_helper_work, "companion is not the P3 bit shape");
    let adm = v.source_params().admission_bounds();
    assert!(v.pre_image[L_ALLOC] + node_delta <= adm.data_nodes as u128,
        "companion delta must not be P6 DataNodes for this fixture");
    assert!(v.pre_image[L_ALLOC + 1] + field_delta <= adm.data_field_slots as u128,
        "companion delta must not be P6 DataFields for this fixture");
    assert_eq!(field_delta as usize, field_writes.len(),
        "field writes are exactly the appended field refs, in operand order");
    assert!(v.pre_image[L_REMAINING_WORK] > 0, "companion keeps the positive-fuel pre-state");
    v.post_image = v.pre_image.clone();
    v.post_data = v.pre_data.clone();
    v.post_helper = v.pre_helper.clone();
    v.post_image[L_REMAINING_WORK] = v.pre_image[L_REMAINING_WORK] - 1;
    v.post_image[L_CONSUMED_WORK] = v.pre_image[L_CONSUMED_WORK] + 1;
    v.post_image[L_HELPER_WORK] = v.pre_image[L_HELPER_WORK] + 1;
    v.post_image[L_ALLOC] = v.pre_image[L_ALLOC] + node_delta;
    v.post_image[L_ALLOC + 1] = v.pre_image[L_ALLOC + 1] + field_delta;
    v.post_image[L_ACTIVE] = v.pre_image[L_ACTIVE] + node_delta;
    v.post_image[L_ACTIVE + 1] = v.pre_image[L_ACTIVE + 1] + field_delta;
    v.post_data[3] = active_data + node_delta;
    v.post_data[4] = active_fields + field_delta;
    if node_delta == 1 {
        let base = DATA_HEADER_LANES + (active_data as usize) * 5;
        v.post_data[base..base + 5].copy_from_slice(&row);
    }
    let fields_base = v.source_params().geometry().data_fields_base();
    for (i, field) in field_writes.iter().enumerate() {
        let base = fields_base + ((active_fields as usize) + i) * 2;
        v.post_data[base] = field.0;
        v.post_data[base + 1] = field.1;
    }
    for lane in rec..res { v.post_helper[lane] = 0; }
    v.post_helper[res] = result[0];
    v.post_helper[res + 1] = result[1];
    v.post_helper[res + 2] = result[2];
    v.post_helper[res + 3] = 0;
}
#[test]
fn raw_fr_range_gate1_gate0_gate2_and_max_positive() {
    let w = base_witness();
    let statement = w.to_statement_frs();
    assert_raw_ok(&w, statement.clone(), "raw statement positive control");
    let fuel_pre = pre_state_start(&w) + L_INITIAL_FUEL;
    let fuel_post = post_state_start(&w) + L_INITIAL_FUEL;
    assert_eq!(statement[fuel_pre], statement[fuel_post], "fuel copies agree");
    // 2^128 on the uncapped, preserved initial-fuel lane, both copies:
    // only the unconditional ConstrainBits(128) can reject (gate 1).
    let mut pow2 = statement.clone();
    pow2[fuel_pre] = two_pow_128(); pow2[fuel_post] = two_pow_128();
    assert_raw_rejects(&w, pow2, "lane at 2^128 gate 1");
    // Above 2^128, same two-sided construction.
    let mut above = statement.clone();
    let v = two_pow_128() + Fr::from(12345u64);
    above[fuel_pre] = v; above[fuel_post] = v;
    assert_raw_rejects(&w, above, "lane above 2^128 gate 1");
    // Raw max 2^128-1 positive control on the same lane, both copies.
    let mut max_ok = statement.clone();
    max_ok[fuel_pre] = Fr::from(u128::MAX); max_ok[fuel_post] = Fr::from(u128::MAX);
    assert_raw_ok(&w, max_ok, "lane at 2^128-1 gate 1");
    // Gate 0: the same 2^128 lane still rejects (ranges are unconditional).
    let mut disabled = w.clone();
    disabled.enable = false;
    disabled.post_image = disabled.pre_image.clone();
    disabled.post_data = disabled.pre_data.clone();
    disabled.post_helper = disabled.pre_helper.clone();
    let mut pow2_g0 = disabled.to_statement_frs();
    assert_eq!(pow2_g0[0], Fr::from(0u64), "disabled gate lane is 0");
    pow2_g0[fuel_pre] = two_pow_128(); pow2_g0[fuel_post] = two_pow_128();
    assert_raw_rejects(&disabled, pow2_g0, "lane at 2^128 gate 0");
    // Gate 0 strictly above 2^128, same preserved lane and both copies.
    let mut above_g0 = disabled.to_statement_frs();
    above_g0[fuel_pre] = v; above_g0[fuel_post] = v;
    assert_raw_rejects(&disabled, above_g0, "lane above 2^128 gate 0");
    // Gate value 2 is outside the unconditional boolean gate domain.
    let mut gate_two = statement.clone(); gate_two[0] = Fr::from(2u64);
    assert_raw_rejects(&w, gate_two, "gate value 2");
    // Gate 0 with a canonical-violating control lane still accepts:
    // enable/disposition domain stays separate from raw ranges.
    let mut disabled_bad = disabled.clone();
    disabled_bad.pre_image[L_CONTROL + 3] = 1;
    disabled_bad.post_image[L_CONTROL + 3] = 1;
    assert_ok_both(&disabled_bad, "gate 0 imposes no canonicality beyond ranges");
}

#[test]
fn enabled_derived_ends_reject_and_disabled_ends_accept() {
    let w = base_witness();
    let geom = w.source_params().geometry();
    let slot = geom.frame_slot_offset(0);
    // Positive canonical MatchF control: arms (0,1), env absent, origin 0.
    // The old RightF lanes are fully rewritten on both copies, including
    // zeroing the absent-env index lane inherited from the RightF origin.
    let mut matchf = w.clone();
    for (off, v) in [(0usize, 5u128), (1, 0), (2, 1), (3, 0), (4, 0), (5, 0)] {
        matchf.pre_image[slot + off] = v; matchf.post_image[slot + off] = v;
    }
    for off in 6..13 { matchf.pre_image[slot + off] = 0; matchf.post_image[slot + off] = 0; }
    assert_ok_both(&matchf, "canonical MatchF control");
    // Derived end exactly 2^128: arms (MAX, 1), 129-bit end rejects gate 1.
    let mut end_pow2 = matchf.clone();
    end_pow2.pre_image[slot + 1] = u128::MAX; end_pow2.post_image[slot + 1] = u128::MAX;
    assert_eq!(end_pow2.pre_image[slot + 4], 0, "absent-env index zero (pre)");
    assert_eq!(end_pow2.post_image[slot + 4], 0, "absent-env index zero (post)");
    assert_rejects_both(&end_pow2, "frame slice end at 2^128");
    // Derived end 2^129-2: arms (MAX, MAX).
    let mut end_max = end_pow2.clone();
    end_max.pre_image[slot + 2] = u128::MAX; end_max.post_image[slot + 2] = u128::MAX;
    assert_rejects_both(&end_max, "frame slice end at 2^129-2");
    // DATA derived end on the RAW old-row fixture defined below: row 0
    // is active in pre (header/active/alloc all 1) and preserved in
    // post, so this is an active-row Cons slice, not an inactive row.
    // Both copies carry slice (MAX, 1), whose derived end is 2^128.
    let mut data_end = raw_old_row_witness();
    let row = DATA_HEADER_LANES;
    for copy in [&mut data_end.pre_data, &mut data_end.post_data] {
        copy[row] = 3;
        copy[row + 1] = 0;
        copy[row + 2] = 0;
        copy[row + 3] = u128::MAX;
        copy[row + 4] = 1;
    }
    assert_rejects_both(&data_end, "active old DATA row slice end at 2^128");
    // Disabled: post == pre in every section; derived ends at 2^128 and
    // 2^129-2 and alloc sums at MAX impose no bound beyond raw128.
    let mut disabled = matchf.clone();
    disabled.enable = false;
    disabled.post_image = disabled.pre_image.clone();
    disabled.post_data = disabled.pre_data.clone();
    disabled.post_helper = disabled.pre_helper.clone();
    disabled.pre_image[slot + 1] = u128::MAX; disabled.post_image[slot + 1] = u128::MAX;
    assert_ok_both(&disabled, "disabled frame slice end at 2^128");
    disabled.pre_image[slot + 2] = u128::MAX; disabled.post_image[slot + 2] = u128::MAX;
    disabled.pre_image[L_ALLOC] = u128::MAX; disabled.post_image[L_ALLOC] = u128::MAX;
    disabled.pre_image[L_ALLOC + 1] = u128::MAX; disabled.post_image[L_ALLOC + 1] = u128::MAX;
    assert_ok_both(&disabled, "disabled slice end 2^129-2 and alloc sums at MAX");
    // Enabled tag-mask separation: gate 2 covered in the raw test above;
    // a nonzero tag lane on a disabled statement is a tag-mask domain
    // case, still accepted at gate 0 (no canonicality is imposed).
    let mut disabled_tag = disabled.clone();
    disabled_tag.pre_image[L_CONTROL] = 9; disabled_tag.post_image[L_CONTROL] = 9;
    assert_ok_both(&disabled_tag, "disabled nonmatching control tag");
    // RAW disabled staged-delta case (not an owner-admitted PendingNode
    // fixture): staged PendingNode with Cons/Branch payload gives
    // field_delta 2 in the emitter's delta derivation. With both alloc
    // lanes at MAX, the disabled sums are MAX+1 (nodes) and MAX+2
    // (fields); gate 0 imposes no bound beyond unconditional raw128.
    // The (MAX, MAX) MatchF slice above already covers end 2^129-2.
    for (mode, payload, label) in [(4u128, 0u128, "Cons"), (5, 2, "Branch")] {
        let mut disabled_pend = w.clone();
        disabled_pend.enable = false;
        disabled_pend.pre_helper[L_HELPER_RECORD + 5] = mode;
        disabled_pend.pre_helper[L_HELPER_RECORD + 15] = 3;
        disabled_pend.pre_helper[L_HELPER_RECORD + 16] = payload;
        disabled_pend.pre_image[L_ALLOC] = u128::MAX;
        disabled_pend.pre_image[L_ALLOC + 1] = u128::MAX;
        disabled_pend.post_image = disabled_pend.pre_image.clone();
        disabled_pend.post_data = disabled_pend.pre_data.clone();
        disabled_pend.post_helper = disabled_pend.pre_helper.clone();
        assert_ok_both(&disabled_pend,
            "disabled PendingNode field_delta 2 alloc sums at MAX (RAW staged fixture)");
        // Raw128 is still unconditional on this disabled fixture.
        let mut raw128 = disabled_pend.to_statement_frs();
        let fp = pre_state_start(&disabled_pend) + L_INITIAL_FUEL;
        let fq = post_state_start(&disabled_pend) + L_INITIAL_FUEL;
        raw128[fp] = two_pow_128(); raw128[fq] = two_pow_128();
        assert_raw_rejects(&disabled_pend, raw128,
            "disabled PendingNode fixture lane at 2^128");
        let _ = label;
    }
}

// ---------- namespaces / composition ----------

fn operand(i: usize) -> Operand { Operand::Variable(Identifier(format!("%in.{i}"))) }
fn scalar_exit_inputs(w: &C6ScalarExitStepWitness) -> C6ScalarExitStepInputs {
    let geom = w.source_params().geometry();
    C6ScalarExitStepInputs {
        enable: operand(900), disposition_is_refused_helper_work: operand(901),
        host_schema: operand(902), carrier_version: operand(903),
        geometry: geom,
        meta_bounds_frames_offset: w.host_prefix().meta_bounds_frames_offset,
        source_params: w.source_params().clone(),
        pre_meta: w.host_prefix().meta.iter().enumerate().map(|(i, _)| operand(i)).collect(),
        pre_prog: w.host_prefix().prog.iter().enumerate().map(|(i, _)| operand(100 + i)).collect(),
        pre_ctx: core::array::from_fn(|i| operand(200 + i)),
        pre_image: (0..geom.image_total_lanes).map(|i| operand(300 + i)).collect(),
        pre_data: (0..geom.data_total_lanes).map(|i| operand(400 + i)).collect(),
        pre_helper: (0..geom.helper_total_lanes).map(|i| operand(500 + i)).collect(),
        post_meta: w.host_prefix().meta.iter().enumerate().map(|(i, _)| operand(600 + i)).collect(),
        post_prog: w.host_prefix().prog.iter().enumerate().map(|(i, _)| operand(700 + i)).collect(),
        post_ctx: core::array::from_fn(|i| operand(800 + i)),
        post_image: (0..geom.image_total_lanes).map(|i| operand(1000 + i)).collect(),
        post_data: (0..geom.data_total_lanes).map(|i| operand(1100 + i)).collect(),
        post_helper: (0..geom.helper_total_lanes).map(|i| operand(1200 + i)).collect(),
    }
}

#[test]
fn namespace_all_five_prefixes_all_sixteen_groups() {
    let w = base_witness();
    for seg in [".local.temp.", ".c6data.temp.", ".c6helper.temp.",
                ".c6meta.temp.", ".c6progctx.temp."] {
        for group in 0..16usize {
            let mut inp = scalar_exit_inputs(&w);
            let bad = Operand::Variable(Identifier(format!("%s{seg}0")));
            match group {
                0 => inp.enable = bad, 1 => inp.disposition_is_refused_helper_work = bad,
                2 => inp.host_schema = bad, 3 => inp.carrier_version = bad,
                4 => inp.pre_meta[0] = bad, 5 => inp.pre_prog[0] = bad,
                6 => inp.pre_ctx[0] = bad, 7 => inp.pre_image[0] = bad,
                8 => inp.pre_data[0] = bad, 9 => inp.pre_helper[0] = bad,
                10 => inp.post_meta[0] = bad, 11 => inp.post_prog[0] = bad,
                12 => inp.post_ctx[0] = bad, 13 => inp.post_image[0] = bad,
                14 => inp.post_data[0] = bad, _ => inp.post_helper[0] = bad,
            }
            assert!(matches!(
                C6ScalarExitStepEmitter::new("s").unwrap().emit(&inp),
                Err(C6ScalarExitStepError::ReservedOperand { .. })),
                "reserved {seg} group {group} must refuse at emitter construction");
        }
    }
}

#[test]
fn namespace_scope_injection_and_empty_rejected() {
    for scope in ["bad%scope", "bad.local.temp.1", "x.c6data.temp.2",
                  "x.c6helper.temp.3", "x.c6meta.temp.4", "x.c6progctx.temp.5"] {
        assert!(matches!(C6ScalarExitStepEmitter::new(scope),
            Err(C6ScalarExitStepError::InvalidScope)), "scope {scope}");
    }
    assert!(matches!(C6ScalarExitStepEmitter::new("  "),
        Err(C6ScalarExitStepError::EmptyScope)), "empty scope");
    assert!(matches!(C6ScalarExitStepEmitter::new(""),
        Err(C6ScalarExitStepError::EmptyScope)), "empty scope (zero len)");
}

fn temps_for(ins: &[midnight_zkir_v3::Instruction], scope: &str, seg: &str)
    -> std::collections::BTreeSet<String> {
    let debug = format!("{:?}", ins);
    let prefix = format!("%{}.{}{}", scope, seg, "");
    let mut out = std::collections::BTreeSet::new();
    let mut rest = debug.as_str();
    while let Some(pos) = rest.find(&prefix) {
        let tail = &rest[pos..];
        let end = tail.find(|ch: char| !(ch.is_ascii_alphanumeric()
            || ch == '%' || ch == '.' || ch == '_')).unwrap_or(tail.len());
        out.insert(tail[..end].to_string());
        rest = &tail[end..];
    }
    out
}
fn max_serial(set: &std::collections::BTreeSet<String>) -> usize {
    set.iter().filter_map(|s| s.rsplit('.').next()
        .and_then(|n| n.parse::<usize>().ok())).max().unwrap_or(0)
}

#[test]
fn namespace_repeated_retained_serials_all_prefixes() {
    let w = base_witness();
    let inp = scalar_exit_inputs(&w);
    let mut retained = C6ScalarExitStepEmitter::new("compose_se").unwrap();
    let first = retained.emit(&inp).unwrap();
    let second = retained.emit(&inp).unwrap();
    // Every composed namespace is actually minted in both calls (this is
    // not parent-debug-only or empty-set disjointness), serials advance,
    // and no identifier is rebound. DATA and HELPER are emitted on both
    // pre and post copies, so their serial ranges strictly grow.
    for seg in ["local.temp.", "c6data.temp.", "c6helper.temp.",
                "c6meta.temp.", "c6progctx.temp."] {
        let a = temps_for(&first.instructions, "compose_se", seg);
        let b = temps_for(&second.instructions, "compose_se", seg);
        assert!(!a.is_empty(), "first emit must mint %{seg} identifiers");
        assert!(!b.is_empty(), "second emit must mint %{seg} identifiers");
        assert!(a.is_disjoint(&b), "retained serials must not rebind %{seg}");
        assert!(max_serial(&b) > max_serial(&a),
            "serials must advance for %{seg}: {:?} vs {:?}", max_serial(&a), max_serial(&b));
    }
    // Single-shot contract stays explicit: a fresh emitter under a new
    // scope starts at serial zero. No safe duplicate fresh same-scope
    // composition in one circuit is claimed.
    let single = super::super::emit_c6_scalar_exit_step_block(&inp, "single_se").unwrap();
    assert!(temps_for(&single.instructions, "single_se", "local.temp.")
        .contains("%single_se.local.temp.0"), "single-shot serial zero");
}

// ---------- all-image canonicality / preservation ----------

#[test]
fn control_all_variants_padding_and_primresult_lanes() {
    let w = base_witness();
    // (tag, payload lanes 1..17, used-lane count including tag lane)
    let cases: [(u128, [u128; 16], usize); 11] = [
        (0, [0; 16], 2), (1, [0; 16], 3), (2, [0; 16], 17),
        (3, [0; 16], 4), (4, [0; 16], 5), (5, [0; 16], 6),
        (6, [0; 16], 7), (7, [0; 16], 11),
        (8, [0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 4),
        (9, [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 2),
        (10, [0; 16], 5),
    ];
    for (tag, rest, used) in cases {
        let mut v = w.clone();
        v.pre_image[L_CONTROL] = tag; v.post_image[L_CONTROL] = tag;
        for (i, value) in rest.iter().enumerate() {
            v.pre_image[L_CONTROL + 1 + i] = *value;
            v.post_image[L_CONTROL + 1 + i] = *value;
        }
        assert_ok_both(&v, "canonical control positive control");
        for lane in used..17usize {
            let mut bad = v.clone();
            bad.pre_image[L_CONTROL + lane] = 1; bad.post_image[L_CONTROL + lane] = 1;
            assert_rejects_both(&bad, "control unused padding lane");
        }
    }
    // PrimResult lanes 4..16: Error sub-tag (lane1=1, code at lane2)
    // requires lane 3 zero; Value Data(1) allows a nonzero lane 3 index.
    let mut prim_err = w.clone();
    set_both(&mut prim_err, 0, L_CONTROL, 8);
    set_both(&mut prim_err, 0, L_CONTROL + 1, 1);
    set_both(&mut prim_err, 0, L_CONTROL + 2, 10);
    assert_eq!(prim_err.pre_image[L_CONTROL + 1], 1, "Error sub-tag lane");
    assert_eq!(prim_err.pre_image[L_CONTROL + 3], 0, "Error lane 3 zero");
    assert_ok_both(&prim_err, "PrimResult Error canonical");
    let mut prim_err_bad = prim_err.clone();
    set_both(&mut prim_err_bad, 0, L_CONTROL + 3, 1);
    assert_rejects_both(&prim_err_bad, "PrimResult Error lane 3 padding");
    for lane in 4..17usize {
        let mut bad = prim_err.clone();
        bad.pre_image[L_CONTROL + lane] = 1; bad.post_image[L_CONTROL + lane] = 1;
        assert_rejects_both(&bad, "PrimResult tail padding lane 4..16");
    }
    let mut prim_val = w.clone();
    set_both(&mut prim_val, 0, L_CONTROL, 8);
    // Base Return lanes already give Value sub-tag 0 (lane +1) and
    // RefC kind Data=1 (lane +2); only the ref index changes 0 -> 1.
    set_both(&mut prim_val, 0, L_CONTROL + 3, 1);
    assert_eq!(prim_val.pre_image[L_CONTROL + 1], 0, "Value sub-tag lane");
    assert_eq!(prim_val.pre_image[L_CONTROL + 2], 1, "Value ref kind Data");
    assert_eq!(prim_val.pre_image[L_CONTROL + 3], 1, "Value ref index 1");
    assert_ok_both(&prim_val, "PrimResult Value Data(1) canonical");
}

#[test]
fn frame_all_variants_inactive_zero_and_padding() {
    let w = base_witness();
    let geom = w.source_params().geometry();
    let slot = geom.frame_slot_offset(0);
    for tag in 0..10u128 {
        let mut v = w.clone();
        v.pre_image[slot] = tag; v.post_image[slot] = tag;
        for off in 1..13 { v.pre_image[slot + off] = 0; v.post_image[slot + off] = 0; }
        assert_ok_both(&v, "canonical frame positive control");
        if tag != 9 {
            let mut bad = v.clone();
            bad.pre_image[slot + 12] = u128::MAX; bad.post_image[slot + 12] = u128::MAX;
            assert_rejects_both(&bad, "frame maximal unused lane");
        }
    }
    // ConstructorFieldsF (used 13) with nonzero origin at the cap rejects.
    let mut ctor = w.clone();
    ctor.pre_image[slot] = 9; ctor.post_image[slot] = 9;
    for (i, value) in [1u128, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1].iter().enumerate() {
        ctor.pre_image[slot + 1 + i] = *value; ctor.post_image[slot + 1 + i] = *value;
    }
    assert_ok_both(&ctor, "ConstructorFieldsF nonzero payload control");
    let mut ctor_bad = ctor.clone();
    let cap = w.source_params().caps().phys.max_expressions as u128;
    ctor_bad.pre_image[slot + 12] = cap; ctor_bad.post_image[slot + 12] = cap;
    assert_rejects_both(&ctor_bad, "ConstructorFieldsF origin at expression cap");
    // Inactive frame slots must be exactly zero (frame_count is 1 here).
    assert_eq!(w.pre_image[L_FRAME_COUNT], 1, "genuine fixture has one frame");
    let inactive = geom.frame_slot_offset(1);
    let mut bad_inactive = w.clone();
    bad_inactive.pre_image[inactive] = 1; bad_inactive.post_image[inactive] = 1;
    assert_rejects_both(&bad_inactive, "inactive frame nonzero lane");
    // Post-only frame/control/origin mutations reach the SDK checker
    // through preservation, not a snapshot assertion.
    let mut post_frame = w.clone(); post_frame.post_image[slot + 1] += 1;
    assert_rejects_both(&post_frame, "post-only frame mutation");
    let mut post_ctl = w.clone(); post_ctl.post_image[L_CONTROL] = 0;
    assert_rejects_both(&post_ctl, "post-only control mutation");
    let mut post_origin = w.clone();
    post_origin.post_image[L_CONTROL_ORIGIN] = post_origin.pre_image[L_CONTROL_ORIGIN] + 1;
    assert_rejects_both(&post_origin, "post-only control-origin mutation");
}

#[test]
fn image_root_caps_env_cursor_and_slice_bounds() {
    let w = base_witness();
    let p = w.source_params().caps().phys;
    // Value-slot root lanes at cap accept, above cap reject (index/count
    // semantics are the emitter's glt_c inclusive-count form).
    for lane in [38usize, 42usize] {
        let cap = p.max_value_slots as u128;
        let mut at_cap = w.clone(); set_both(&mut at_cap, 0, lane, cap);
        assert_ok_both(&at_cap, "root value-slot lane at cap");
        let mut above = w.clone(); set_both(&mut above, 0, lane, cap + 1);
        assert_rejects_both(&above, "root value-slot lane above cap");
    }
    // Tied node lanes 32 (alloc.data_nodes), 35 (active.data) and DATA
    // header lane 3 must move together; emitter equality is
    // pre.alloc.data_nodes == pre.active.data == pre DATA header[3].
    // At cap == admission bound in this fixture, the coherent positive
    // is a P6 DataNodes refusal, NOT an Applied step: Applied would
    // advance the header to cap+1 and write result Data(cap).
    let nodes_cap = p.max_data_nodes as u128;
    let mut at_cap = w.clone();
    at_cap.pre_image[L_ALLOC] = nodes_cap;
    at_cap.pre_image[L_ACTIVE] = nodes_cap;
    at_cap.pre_data[3] = nodes_cap;
    at_cap.post_data = at_cap.pre_data.clone();
    at_cap.post_helper = at_cap.pre_helper.clone();
    at_cap.post_image[L_ALLOC] = nodes_cap;
    at_cap.post_image[L_ACTIVE] = nodes_cap;
    at_cap.post_image[L_OUTCOME] = 3;
    at_cap.post_image[L_OUTCOME + 1] = 1;
    at_cap.post_image[L_OUTCOME + 2] = 0;
    assert_ok_both(&at_cap,
        "tied alloc/active/header at cap: coherent P6 DataNodes refusal");
    let mut above_tied = at_cap.clone();
    above_tied.pre_image[L_ALLOC] = nodes_cap + 1;
    above_tied.pre_image[L_ACTIVE] = nodes_cap + 1;
    above_tied.pre_data[3] = nodes_cap + 1;
    above_tied.post_image[L_ALLOC] = nodes_cap + 1;
    above_tied.post_image[L_ACTIVE] = nodes_cap + 1;
    above_tied.post_data[3] = nodes_cap + 1;
    assert_rejects_both(&above_tied,
        "tied alloc/active/header above cap (competing image-cap and DATA-header-cap predicates; not isolated)");
    // Remaining seg/alloc root lanes: at physical cap accept, above reject.
    for (lane, cap) in [(34usize, p.max_env_cells),
                        (39, p.max_data_nodes), (45, p.max_env_cells)] {
        let mut at_cap = w.clone(); set_both(&mut at_cap, 0, lane, cap as u128);
        assert_ok_both(&at_cap, "root seg lane at cap");
        let mut above = w.clone(); set_both(&mut above, 0, lane, cap as u128 + 1);
        assert_rejects_both(&above, "root seg lane above cap");
    }
    // Env root: absent with nonzero index rejects; present at cap rejects
    // as an index (strict), present below cap accepts.
    let mut env_bad = w.clone();
    assert_eq!(env_bad.pre_image[L_ENV_ROOT], 0, "env root starts absent");
    set_both(&mut env_bad, 0, L_ENV_ROOT + 1, 1);
    assert_rejects_both(&env_bad, "env root absent nonzero index");
    let mut env_at = w.clone();
    set_both(&mut env_at, 0, L_ENV_ROOT, 1);
    set_both(&mut env_at, 0, L_ENV_ROOT + 1, p.max_env_cells as u128);
    assert_rejects_both(&env_at, "env root present index at cap (strict)");
    // Stack limit / frame count caps.
    let mut stack_above = w.clone();
    set_both(&mut stack_above, 0, 22, p.max_frames as u128 + 1);
    assert_rejects_both(&stack_above, "stack limit above frames cap");
    let mut frames_above = w.clone();
    set_both(&mut frames_above, 0, L_FRAME_COUNT, p.max_frames as u128 + 1);
    assert_rejects_both(&frames_above, "frame count above frames cap");
    // Reference cursor / helper visits are preserved lanes: post-only
    // mutation rejects.
    let mut post_cursor = w.clone();
    post_cursor.post_image[L_REFERENCE_CURSOR] += 1;
    assert_rejects_both(&post_cursor, "post-only reference cursor mutation");
}

// Explicitly RAW old-row fixture (not an owner-admitted MachineImage):
// the base first ScalarExit has pre.active.data == 0, so row 0 there
// is the NEW target. This fixture binds one pre-existing active old
// U128 row coherently: pre alloc == active == DATA header == 1, old
// row U128{7} preserved, new U128{42} row at slot 1 in post, post
// header/active/alloc == 2, result ref == pre.active.data == 1.
fn raw_old_row_witness() -> C6ScalarExitStepWitness {
    let mut w = base_witness();
    let row = DATA_HEADER_LANES;
    assert_eq!(w.pre_image[L_ACTIVE], 0, "base first exit has no old DATA row");
    assert_eq!(w.pre_data[3], 0, "base DATA header starts empty");
    w.pre_data[row] = 0; w.pre_data[row + 1] = 7;
    w.pre_data[row + 2] = 0; w.pre_data[row + 3] = 0; w.pre_data[row + 4] = 0;
    w.pre_data[3] = 1;
    w.pre_image[L_ALLOC] = 1; w.pre_image[L_ACTIVE] = 1;
    w.pre_image[L_SEG_ACTIVE] = 1; w.pre_image[L_SEG_ALLOC] = 1;
    w.post_data[row] = 0; w.post_data[row + 1] = 7;
    w.post_data[row + 2] = 0; w.post_data[row + 3] = 0; w.post_data[row + 4] = 0;
    w.post_data[row + 5] = 0; w.post_data[row + 6] = 42;
    w.post_data[row + 7] = 0; w.post_data[row + 8] = 0; w.post_data[row + 9] = 0;
    w.post_data[3] = 2;
    w.post_image[L_ALLOC] = 2; w.post_image[L_ACTIVE] = 2;
    w.post_image[L_SEG_ACTIVE] = 1; w.post_image[L_SEG_ALLOC] = 1;
    w.post_helper[L_RESULT_RECORD + 2] = 1;
    assert_ok_both(&w, "RAW old-row fixture positive control");
    w
}
#[test]
fn data_canonical_and_post_only_preservation() {
    let w = base_witness();
    let geom = w.source_params().geometry();
    // Old (non-target) DATA row post-only mutation, on the RAW
    // old-row fixture: slot 0 is preserved, slot 1 is the new target.
    let old = raw_old_row_witness();
    let mut post_row = old.clone();
    post_row.post_data[DATA_HEADER_LANES + 1] = 8;
    assert_rejects_both(&post_row, "post-only old DATA row mutation (RAW fixture)");
    // Env row and value-slot post-only mutations (env/values preserved
    // exactly and canonicalised post).
    let mut post_env = w.clone();
    post_env.post_data[geom.env_base()] = 1;
    assert_rejects_both(&post_env, "post-only env row mutation");
    let mut post_slot = w.clone();
    post_slot.post_data[geom.value_slots_base() + 1] = 1;
    assert_rejects_both(&post_slot, "post-only value-slot mutation");
    // Inactive DATA row nonzero on both copies rejects (canonicality).
    let mut inactive_row = w.clone();
    let inactive = DATA_HEADER_LANES + 5;
    set_both(&mut inactive_row, 1, inactive, 1);
    assert_rejects_both(&inactive_row, "inactive DATA row nonzero");
    // Header count above cap on both copies; at cap is a competing
    // earlier-stage case and is not claimed as an isolated count check.
    let mut count_above = w.clone();
    let nodes_cap = w.source_params().caps().phys.max_data_nodes as u128;
    set_both(&mut count_above, 1, 3, nodes_cap + 1);
    assert_rejects_both(&count_above, "DATA header count above cap");
    // Active old-row variant padding, on both copies of the RAW
    // old-row fixture (row 0 is active, so variant padding is the
    // predicate; an inactive-row zero failure is not the label).
    let mut nil_bad = raw_old_row_witness();
    for copy in [&mut nil_bad.pre_data, &mut nil_bad.post_data] {
        copy[DATA_HEADER_LANES] = 2;
    }
    assert_rejects_both(&nil_bad, "active Nil old row payload padding (RAW fixture)");
    let mut bool_bad = raw_old_row_witness();
    for copy in [&mut bool_bad.pre_data, &mut bool_bad.post_data] {
        copy[DATA_HEADER_LANES] = 1;
        copy[DATA_HEADER_LANES + 1] = 2;
    }
    assert_rejects_both(&bool_bad, "active Bool old row non-boolean payload (RAW fixture)");
}

// ---------- program / context / config (statement level) ----------

#[test]
fn prog_ctx_meta_statement_and_post_only_equality() {
    let w = base_witness();
    assert_ok_both(&w, "PROG/CTX/META positive control");
    // Both-sided CTX binding mutation (statement duplicates host_prefix
    // into pre and post, so this reaches the bound-CTX predicate).
    let mut ctx_bad = w.clone(); ctx_bad.host_prefix.ctx[20] += 1;
    assert_rejects_both(&ctx_bad, "CTX budget lane binding");
    let mut ctx_ext = w.clone(); ctx_ext.host_prefix.ctx[3] = 1;
    assert_rejects_both(&ctx_ext, "CTX ext presence canonicality");
    // Both-sided META admission-field mutation reaches bound META.
    let mut meta_bad = w.clone(); meta_bad.host_prefix.meta[3] += 1;
    assert_rejects_both(&meta_bad, "META lane through bound composition");
    // Post-only equality mutations, at raw statement level (the u128
    // witness carries one shared prefix, so pre!=post exists only as
    // raw Frs): post PROG, post CTX and post META lanes unequal to pre
    // reject on the unconditional equality, in both modes.
    let statement = w.to_statement_frs();
    let prog_post = post_prefix_start(&w)
        + w.host_prefix().meta.len();
    let mut post_prog = statement.clone();
    post_prog[prog_post] = post_prog[prog_post] + Fr::from(1u64);
    assert_raw_rejects(&w, post_prog, "post-only PROG equality mutation");
    let ctx_post = prog_post + w.host_prefix().prog.len();
    let mut post_ctx = statement.clone();
    post_ctx[ctx_post] = post_ctx[ctx_post] + Fr::from(1u64);
    assert_raw_rejects(&w, post_ctx, "post-only CTX equality mutation");
    let meta_post = post_prefix_start(&w);
    let mut post_meta = statement.clone();
    post_meta[meta_post] = Fr::from(C6_PHYS_MAGIC) + Fr::from(1u64);
    assert_raw_rejects(&w, post_meta, "post-only META equality mutation");
    // Host-prefix / encoder rejects are HOST_STAGE and do not count as
    // the circuit negatives above; they are asserted separately in the
    // owner tests (HostPrefixMismatch) and are not repeated here.
}

#[test]
fn meta_bounds_frames_offset_two_stage() {
    let w = base_witness();
    let shifted_offset = w.host_prefix().meta_bounds_frames_offset + 1;
    let shifted = build_c6_scalar_exit_step_circuit(w.source_params().clone(),
        w.host_prefix().meta.len(), w.host_prefix().prog.len(), shifted_offset,
        C6PushStatementBindingMode::LocalPublic, "scalar_exit_raw_test").unwrap();
    // Stage 1 (host preflight): unchanged witness carries the unshifted
    // offset; refusal is the exact host message, not an SDK failure.
    match check_c6_scalar_exit_step_circuit(&shifted, &w) {
        Err(C6ScalarExitConsumerError::Backend(m)) =>
            assert_eq!(m, "meta bounds frames offset mismatch", "host offset stage: {m}"),
        other => panic!("host offset stage: ok={}", other.is_ok()),
    }
    // Stage 2 (SDK preprocessing): a witness tied to the shifted offset
    // passes host equality and is rejected by ProofPreimage::check.
    let mut tied = w.clone();
    tied.host_prefix.meta_bounds_frames_offset = shifted_offset;
    match check_c6_scalar_exit_step_circuit(&shifted, &tied) {
        Err(C6ScalarExitConsumerError::Backend(m)) => assert!(
            m.contains("SDK ProofPreimage::check failed"), "shifted SDK stage: {m}"),
        other => panic!("shifted SDK stage: ok={}", other.is_ok()),
    }
}

// ---------- counters / refusals ----------

#[test]
fn counters_charges_no_wrap_and_zero_remaining() {
    let w = base_witness();
    // Wrong P4 charges on an Applied post.
    let mut no_hw = w.clone();
    no_hw.post_image[L_HELPER_WORK] = no_hw.pre_image[L_HELPER_WORK];
    assert_rejects_both(&no_hw, "missing helper-work charge");
    let mut no_consumed = w.clone();
    no_consumed.post_image[L_CONSUMED_WORK] = no_consumed.pre_image[L_CONSUMED_WORK];
    assert_rejects_both(&no_consumed, "missing consumed-work charge");
    let mut bad_remaining = w.clone();
    bad_remaining.post_image[L_REMAINING_WORK] = bad_remaining.pre_image[L_REMAINING_WORK];
    assert_rejects_both(&bad_remaining, "missing remaining-work charge");
    // No-wrap: consumed at u128::MAX with wrapped post 0 rejects. This
    // is a raw unreachable-domain case (RAW), not an admitted owner path.
    let mut wrap = w.clone();
    wrap.pre_image[L_CONSUMED_WORK] = u128::MAX;
    wrap.post_image[L_CONSUMED_WORK] = 0;
    assert_rejects_both(&wrap, "consumed-work charge wrap (RAW)");
    let mut wrap_hw = w.clone();
    wrap_hw.pre_image[L_HELPER_WORK] = u128::MAX;
    wrap_hw.post_image[L_HELPER_WORK] = 0;
    assert_rejects_both(&wrap_hw, "helper-work charge wrap (RAW)");
    // Zero remaining work on the selected pre-state: the emitter's
    // positive-fuel gate rejects; this is the distinct P2 driver
    // branch, labelled as such, not a ScalarExit Applied path.
    let mut zero_fuel = w.clone();
    zero_fuel.pre_image[L_REMAINING_WORK] = 0;
    zero_fuel.post_image[L_REMAINING_WORK] = 0;
    assert_rejects_both(&zero_fuel, "zero remaining work selected pre-state");
    // Alloc/active equality-vs-cap boundary: at the genuine fixture the
    // Applied post advances alloc by (1,0); shrinking the post alloc
    // lane back to pre (equality without delta) rejects.
    let mut alloc_eq = w.clone();
    alloc_eq.post_image[L_ALLOC] = alloc_eq.pre_image[L_ALLOC];
    assert_rejects_both(&alloc_eq, "post alloc equality without Applied delta");
    let mut active_eq = w.clone();
    active_eq.post_image[L_ACTIVE] = active_eq.pre_image[L_ACTIVE];
    assert_rejects_both(&active_eq, "post active equality without Applied delta");
}

#[test]
fn p3_p6_dispositions_priority_and_wrong_charges() {
    // Genuine P6 (DataNodes): admission data_nodes = 0, U128 delta 1.
    let p6 = witness_for_bounds(bounds(100, 0, 0));
    assert!(!p6.disposition_is_refused_helper_work(), "P6 is not P3");
    assert_ok_both(&p6, "genuine P6 DataNodes charged refusal");
    // Wrong P6 charge: remove the helper-work charge from the P6 post.
    let mut p6_nocharge = p6.clone();
    p6_nocharge.post_image[L_HELPER_WORK] = p6_nocharge.pre_image[L_HELPER_WORK];
    assert_rejects_both(&p6_nocharge, "P6 refusal without P4 charge");
    // Wrong outcome disposition on the P6 post (Running instead of
    // Exhausted{DataNodes} = [3,1,0]).
    let mut p6_outcome = p6.clone();
    p6_outcome.post_image[L_OUTCOME] = 0;
    p6_outcome.post_image[L_OUTCOME + 1] = 0;
    assert_rejects_both(&p6_outcome, "P6 post with Applied outcome");
    // Genuine P3 wins over simultaneous P6: learn the real helper-work
    // count at Done, then admit with exactly that bound and zero caps.
    let (_a, _i, pre) = reach_u128(generous());
    let hw = pre.image.helper_work;
    assert!(hw > 0, "real Done helper-work count is positive");
    let p3 = witness_for_bounds(bounds(hw, 0, 0));
    assert!(p3.disposition_is_refused_helper_work(), "P3 disposition bit set");
    assert_ok_both(&p3, "genuine P3 uncharged refusal winning over P6");
    // Charged P3 is rejected: add a consumed-work charge to the P3 post.
    let mut p3_charged = p3.clone();
    p3_charged.post_image[L_CONSUMED_WORK] = p3_charged.pre_image[L_CONSUMED_WORK] + 1;
    assert_rejects_both(&p3_charged, "charged P3 refusal");
    // Flipped disposition bit on an Applied witness reaches the circuit.
    let w = base_witness();
    let mut flipped = w.clone(); flipped.disposition_is_refused_helper_work = true;
    assert_rejects_both(&flipped, "disposition bit mismatch");
    // Node-before-field priority is a genuine owner case in the current
    // 19 tests (Branch at bounds(100,0,0)); the circuit-level priority
    // here is the P6 outcome lane: DataNodes [3,1,0], not DataFields.
    assert_eq!(p6.post_image[L_OUTCOME], 3, "P6 outcome tag Exhausted");
    assert_eq!(p6.post_image[L_OUTCOME + 1], 1, "P6 reason DataNodes before DataFields");
}

// ---------- invalid domain ----------

#[test]
fn invalid_domain_staged_mode_site_and_presence() {
    let w = base_witness();
    let rec = L_HELPER_RECORD;
    // Staged-domain companions: each post-state below is rebuilt from
    // its own mutated pre-state by `finish_derived_applied`, so DATA,
    // header, counter, outcome and result-Eq mismatches from retaining
    // the genuine U128 post no longer compete with the named domain
    // predicate. Payloads for None/PrimError/unknown tags are placed in
    // the intersection of their staged canonical domain and the
    // emitter's literal nonallocating fallback result domain; the
    // fallback result tag 13 is the emitter's emitted equation for a
    // nonallocating non-Selector branch, not an invented Core branch.
    let mut none_tag = w.clone();
    set_pre_helper(&mut none_tag, rec + 15, 0);
    set_pre_helper(&mut none_tag, rec + 16, 0);
    finish_derived_applied(&mut none_tag, 0, 0, [0; 5], &[], [13, 0, 0]);
    assert_rejects_both(&none_tag, "staged None tag, derived nonallocating companion");
    let mut none_ctrl = none_tag.clone();
    none_ctrl.pre_helper[rec + 15] = 1; none_ctrl.pre_helper[rec + 16] = 42;
    let a0 = none_ctrl.pre_data[3];
    finish_derived_applied(&mut none_ctrl, 1, 0, [0, 42, 0, 0, 0], &[], [10, 1, a0]);
    assert_ok_both(&none_ctrl, "staged None companion valid U128 control");
    let mut prim_tag = w.clone();
    set_pre_helper(&mut prim_tag, rec + 15, 4);
    set_pre_helper(&mut prim_tag, rec + 16, 1);
    finish_derived_applied(&mut prim_tag, 0, 0, [0; 5], &[], [13, 1, 0]);
    assert_rejects_both(&prim_tag, "staged PrimError tag, derived fallback companion");
    let mut prim_ctrl = prim_tag.clone();
    prim_ctrl.pre_helper[rec + 15] = 1; prim_ctrl.pre_helper[rec + 16] = 42;
    let a0 = prim_ctrl.pre_data[3];
    finish_derived_applied(&mut prim_ctrl, 1, 0, [0, 42, 0, 0, 0], &[], [10, 1, a0]);
    assert_ok_both(&prim_ctrl, "staged PrimError companion valid U128 control");
    let mut unknown_tag = w.clone();
    set_pre_helper(&mut unknown_tag, rec + 15, 7);
    set_pre_helper(&mut unknown_tag, rec + 16, 1);
    finish_derived_applied(&mut unknown_tag, 0, 0, [0; 5], &[], [13, 1, 0]);
    assert_rejects_both(&unknown_tag, "unknown staged tag 7, derived fallback companion");
    let mut unknown_ctrl = unknown_tag.clone();
    unknown_ctrl.pre_helper[rec + 15] = 1; unknown_ctrl.pre_helper[rec + 16] = 42;
    let a0 = unknown_ctrl.pre_data[3];
    finish_derived_applied(&mut unknown_ctrl, 1, 0, [0, 42, 0, 0, 0], &[], [10, 1, a0]);
    assert_ok_both(&unknown_ctrl, "unknown staged tag companion valid U128 control");
    // Mode-only coherence case: node/field deltas still follow the
    // U128 staged tag, so the derived post is the genuine U128 post.
    // The only pre-state corrections beyond the named mode mutation
    // are the mode-9 presence equations: operand_b absent with zero
    // kind/index lanes, expected present with a canonical type lane.
    let mut incoherent = w.clone(); set_pre_helper(&mut incoherent, rec + 5, 9);
    incoherent.pre_helper[rec + 8] = 0;
    incoherent.pre_helper[rec + 9] = 0;
    incoherent.pre_helper[rec + 10] = 0;
    incoherent.pre_helper[rec + 11] = 1;
    incoherent.pre_helper[rec + 12] = 0;
    let a0 = incoherent.pre_data[3];
    finish_derived_applied(&mut incoherent, 1, 0, [0, 42, 0, 0, 0], &[], [10, 1, a0]);
    assert_rejects_both(&incoherent, "mode TestType with U128 staged tag, presence-corrected companion");
    let mut incoherent_ctrl = incoherent.clone();
    incoherent_ctrl.pre_helper[rec + 5] = 0;
    incoherent_ctrl.pre_helper[rec + 8] = w.pre_helper[rec + 8];
    incoherent_ctrl.pre_helper[rec + 9] = w.pre_helper[rec + 9];
    incoherent_ctrl.pre_helper[rec + 10] = w.pre_helper[rec + 10];
    incoherent_ctrl.pre_helper[rec + 11] = 0;
    incoherent_ctrl.pre_helper[rec + 12] = 0;
    let a0 = incoherent_ctrl.pre_data[3];
    finish_derived_applied(&mut incoherent_ctrl, 1, 0, [0, 42, 0, 0, 0], &[], [10, 1, a0]);
    assert_ok_both(&incoherent_ctrl, "mode/staged coherence companion valid Add/U128 control");
    // PendingNode field delta follows the Leaf payload (1), not the
    // Cons mode (which would append two fields only with payload 0).
    let oa = (w.pre_helper[rec + 6], w.pre_helper[rec + 7]);
    let ob = (w.pre_helper[rec + 9], w.pre_helper[rec + 10]);
    let mut node_mismatch = w.clone();
    set_pre_helper(&mut node_mismatch, rec + 15, 3);
    set_pre_helper(&mut node_mismatch, rec + 5, 4);
    set_pre_helper(&mut node_mismatch, rec + 16, 1);
    let a0 = node_mismatch.pre_data[3];
    let f0 = node_mismatch.pre_data[4];
    finish_derived_applied(&mut node_mismatch, 1, 1, [4, 0, 0, f0, 1], &[oa], [10, 1, a0]);
    assert_rejects_both(&node_mismatch, "PendingNode Cons mode with Leaf node tag, derived Leaf companion");
    let mut node_ctrl = node_mismatch.clone();
    node_ctrl.pre_helper[rec + 16] = 0;
    let a0 = node_ctrl.pre_data[3];
    let f0 = node_ctrl.pre_data[4];
    finish_derived_applied(&mut node_ctrl, 1, 2, [3, 0, 0, f0, 2], &[oa, ob], [10, 1, a0]);
    assert_ok_both(&node_ctrl, "PendingNode companion valid Cons control");
    let mut kind_bad = w.clone(); set_pre_helper(&mut kind_bad, rec, 2);
    assert_rejects_both(&kind_bad, "helper kind EnvGet on ScalarExit");
    let mut phase_bad = w.clone(); set_pre_helper(&mut phase_bad, rec + 13, 1);
    assert_rejects_both(&phase_bad, "helper phase Second on exit");
    let mut site_bad = w.clone(); set_pre_helper(&mut site_bad, rec + 1, 14);
    assert_rejects_both(&site_bad, "site outside emitted set");
    let mut ob_absent = w.clone(); set_pre_helper(&mut ob_absent, rec + 8, 0);
    assert_rejects_both(&ob_absent, "binary mode with operand_b absent");
    let mut absent_idx = w.clone();
    set_pre_helper(&mut absent_idx, rec + 8, 0);
    set_pre_helper(&mut absent_idx, rec + 10, 2);
    assert_rejects_both(&absent_idx, "absent operand_b with nonzero index");
    let mut exp_bad = w.clone(); set_pre_helper(&mut exp_bad, rec + 11, 1);
    assert_rejects_both(&exp_bad, "expected present on non-TestType mode");
    let mut resume_bad = w.clone(); set_pre_helper(&mut resume_bad, rec + 3, 1);
    assert_rejects_both(&resume_bad, "parent_resume present");
    let mut result_bad = w.clone(); set_pre_helper(&mut result_bad, L_RESULT_RECORD, 10);
    assert_rejects_both(&result_bad, "pre result present");
    let mut task_bad = w.clone(); set_pre_helper(&mut task_bad, 3, 1);
    assert_rejects_both(&task_bad, "pre task count nonzero");
}

#[test]
fn invalid_domain_helper_result_and_ref_caps() {
    let w = base_witness();
    let rec = L_HELPER_RECORD;
    let res = L_RESULT_RECORD;
    // Helper not cleared / wrong result on the Applied post.
    let mut helper_kept = w.clone(); helper_kept.post_helper[rec] = 1;
    assert_rejects_both(&helper_kept, "helper not cleared on Applied");
    let mut result_tag = w.clone(); result_tag.post_helper[res] = 12;
    assert_rejects_both(&result_tag, "Applied result tag ScalarSelector for U128");
    let mut result_ref = w.clone();
    let at = w.pre_image[L_ACTIVE] ;
    result_ref.post_helper[res + 2] = at + 1;
    assert_rejects_both(&result_ref, "ScalarValue result ref not pre.active.data");
    let mut result_pad = w.clone(); result_pad.post_helper[res + 3] = 1;
    assert_rejects_both(&result_pad, "result record padding lane");
    // Helper post canonicality: a refused-shaped post helper (record
    // retained) on an Applied witness rejects through post HELPER
    // canonicality plus the exact post relation.
    let mut post_helper_bad = w.clone();
    post_helper_bad.post_helper[rec + 15] = 4;
    assert_rejects_both(&post_helper_bad, "post helper staged PrimError");
    // Operand ref cap mismatch: operand_a as Data at the data-nodes cap
    // (strict index) on both helper copies.
    let nodes_cap = w.source_params().caps().phys.max_data_nodes as u128;
    let mut ref_cap = w.clone();
    set_pre_helper(&mut ref_cap, rec + 6, 1);
    set_pre_helper(&mut ref_cap, rec + 7, nodes_cap);
    assert_rejects_both(&ref_cap, "operand_a Data index at cap");
    // Selector/TypeTest payload companions. Presence is corrected to
    // the mutated mode first: TestBool and TestType are non-binary, so
    // operand_b is absent with zero kind/index lanes; TestType also
    // requires expected present. The derived post is then rebuilt by
    // `finish_derived_applied` with no DATA/header advance and the
    // exact forwarded result. COUPLING: the same invalid payload is
    // forwarded into the post result (`result_a = spay`, tag 12 or
    // 13), and HELPER canonicality applies the identical payload
    // domain pre (`gstaged`) and post (result record). A pre-only
    // wholeStep isolation of that subpredicate therefore does not
    // exist in the unchanged relation; these companions isolate the
    // staged/payload domain family from every unrelated DATA, header,
    // presence, counter and result-Eq mismatch, and each is paired
    // with the same fixture at a valid payload. SI21 selector/typetest
    // denotation remains REQUIRED_OPEN.
    let mut sel_bad = w.clone();
    set_pre_helper(&mut sel_bad, rec + 15, 5);
    set_pre_helper(&mut sel_bad, rec + 16, 3);
    set_pre_helper(&mut sel_bad, rec + 5, 8);
    sel_bad.pre_helper[rec + 8] = 0;
    sel_bad.pre_helper[rec + 9] = 0;
    sel_bad.pre_helper[rec + 10] = 0;
    finish_derived_applied(&mut sel_bad, 0, 0, [0; 5], &[], [12, 3, 0]);
    assert_rejects_both(&sel_bad, "Selector staged payload above 2, derived companion (coupled pre/post payload domain)");
    let mut sel_ctrl = sel_bad.clone();
    sel_ctrl.pre_helper[rec + 16] = 2;
    finish_derived_applied(&mut sel_ctrl, 0, 0, [0; 5], &[], [12, 2, 0]);
    assert_ok_both(&sel_ctrl, "Selector companion valid payload NotBool control");
    let mut tt_bad = w.clone();
    set_pre_helper(&mut tt_bad, rec + 15, 6);
    set_pre_helper(&mut tt_bad, rec + 16, 2);
    set_pre_helper(&mut tt_bad, rec + 5, 9);
    tt_bad.pre_helper[rec + 8] = 0;
    tt_bad.pre_helper[rec + 9] = 0;
    tt_bad.pre_helper[rec + 10] = 0;
    tt_bad.pre_helper[rec + 11] = 1;
    tt_bad.pre_helper[rec + 12] = 0;
    finish_derived_applied(&mut tt_bad, 0, 0, [0; 5], &[], [13, 2, 0]);
    assert_rejects_both(&tt_bad, "TypeTest staged payload non-boolean, derived companion (coupled pre/post payload domain)");
    let mut tt_ctrl = tt_bad.clone();
    tt_ctrl.pre_helper[rec + 16] = 1;
    finish_derived_applied(&mut tt_ctrl, 0, 0, [0; 5], &[], [13, 1, 0]);
    assert_ok_both(&tt_ctrl, "TypeTest companion valid payload true control");
    // Root/helper tie, borrow-safe: the cursor value is read before
    // the mutable helper mutation begins.
    let cursor_plus_one = w.pre_image[L_REFERENCE_CURSOR] + 1;
    let mut origin_bad = w.clone();
    set_pre_helper(&mut origin_bad, rec + 2, cursor_plus_one);
    assert_rejects_both(&origin_bad, "helper origin != image reference cursor");
}

// ---------- LedgerCall commitment handling ----------

#[test]
fn ledger_call_semantic_negative_recomputed_commitment() {
    let w = base_witness();
    // Positive LedgerCall control through the consumer checker (which
    // recomputes the ordinary witness commitment).
    let lc = circuit_for(&w, C6PushStatementBindingMode::LedgerCall);
    assert!(check_c6_scalar_exit_step_circuit(&lc, &w).is_ok(),
        "LedgerCall positive control");
    // Semantic negative through the consumer checker in LedgerCall:
    // wrong post result code lane; the checker recomputes the
    // commitment over the mutated post, so this is a relation
    // rejection, not a commitment mismatch.
    let mut bad = w.clone(); bad.post_helper[L_RESULT_RECORD] = 12;
    match check_c6_scalar_exit_step_circuit(&lc, &bad) {
        Err(C6ScalarExitConsumerError::Backend(m)) => assert!(
            m.contains("SDK ProofPreimage::check failed"),
            "LedgerCall negative must be SDK preprocessing: {m}"),
        other => panic!("LedgerCall negative: ok={}", other.is_ok()),
    }
    // Raw-Fr LedgerCall negative: mutate only the post outcome lane of
    // the raw statement to 2^128. The raw helper derives the declared
    // post-state output vector from the mutated statement and
    // recomputes the commitment over it; rejection is therefore the
    // unconditional raw128 range on the post lane, in LedgerCall too.
    let statement = w.to_statement_frs();
    let outcome_post = post_state_start(&w) + L_OUTCOME;
    let mut raw_bad = statement.clone();
    raw_bad[outcome_post] = two_pow_128();
    match check_raw(&lc, &w, raw_bad.clone(), None) {
        Err(C6ScalarExitConsumerError::Backend(m)) => assert!(
            m.contains("SDK ProofPreimage::check failed"),
            "raw LedgerCall negative must be SDK preprocessing: {m}"),
        other => panic!("raw LedgerCall negative: ok={}", other.is_ok()),
    }
    // Explicit commitment-specific negative (separate category, not
    // a relation rejection): a mutated-but-otherwise-accepted raw
    // statement (fuel at 2^128-1 on both copies, as in the paired
    // raw positive) passes with the commitment recomputed over the
    // mutated statement plus its mutated declared post-state vector.
    let outcome_post = post_state_start(&w) + L_OUTCOME;
    let _ = outcome_post;
    let mut valid_mutated = statement.clone();
    let fuel_pre = pre_state_start(&w) + L_INITIAL_FUEL;
    let fuel_post = post_state_start(&w) + L_INITIAL_FUEL;
    valid_mutated[fuel_pre] = Fr::from(u128::MAX);
    valid_mutated[fuel_post] = Fr::from(u128::MAX);
    assert!(check_raw(&lc, &w, valid_mutated.clone(), None).is_ok(),
        "mutated valid statement with recomputed commitment must pass");
    // The stale commitment is the correct commitment for the
    // UNMUTATED statement plus its unmutated post-state outputs
    // (statement ++ statement[post_state_start..]), submitted with
    // the mutated statement. SDK preprocessing checks the
    // communications commitment only after executing the circuit
    // instructions, so the failure category is the commitment
    // mismatch itself, not an earlier range/relation failure.
    let mut stale_inputs = statement.clone();
    stale_inputs.extend(statement[post_state_start(&w)..].iter().copied());
    let stale = transient_commit(&stale_inputs, Fr::from(0u64));
    match check_raw(&lc, &w, valid_mutated, Some(stale)) {
        Err(C6ScalarExitConsumerError::Backend(m)) => {
            assert!(m.contains("SDK ProofPreimage::check failed"),
                "stale commitment negative must fail preprocessing: {m}");
            assert!(m.contains("Communications commitment mismatch"),
                "stale commitment negative must be the commitment category: {m}");
        }
        other => panic!("stale commitment negative: ok={}", other.is_ok()),
    }
    // LocalPublic paired positive for the same raw statement.
    let lp = circuit_for(&w, C6PushStatementBindingMode::LocalPublic);
    assert!(check_raw(&lp, &w, statement, None).is_ok(),
        "LocalPublic raw positive control");
}
