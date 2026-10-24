//! Bound-mode tests through the actual public SDK preprocessing path
//! (`IrSource::check` over `PublicInput` operands). UNRUN: no SDK,
//! compiler, or MockProver execution has occurred.
//!
//! Harness truth: most bound witnesses are produced by first mutating
//! a `BoundsC` or `LayoutSnapshot` fixture and then re-encoding that
//! mutated metadata with `encode_metadata` through `lanes(..)`. They
//! are not mutated after encoding. The resulting raw lane vectors are
//! supplied as public-transcript statement values to `IrSource::check`
//! while the retained binding stays fixed at the original fixture, so
//! a rejection reaches gadget preprocessing rather than a host
//! producer filter: the host encoder accepts the structurally valid
//! mutated metadata, and the SDK check evaluates the bound gadget
//! against the raw lanes. Exceptions that act after encoding are the
//! malformed-UTF-8 lane replacement and shifted-offset parameter in
//! `shifted_offset_and_malformed_utf8_reject`, and the direct `Fr`
//! statement values in
//! `raw_fr_lane_and_gate_at_2_pow_128_reject_in_preprocessing`.
//!
//! Fixtures and source paths: the good metadata is
//! `meta(adm_bounds(), layout())`, encoded by
//! `mori_core::sherlock::helper_machine::c6_phys_host::encode_metadata`.
//! The standard retained binding is `binding()`, built by the
//! production `C6MetaConfigBinding::new(adm_bounds(), layout(),
//! limits())` in this module's parent `c6_meta_canonical.rs`; the
//! frames offset comes from the production
//! `super::super::c6_push_step::meta_admission_bounds_frames_offset`.
//! Variant bindings in the scalar and empty-width tests are built by
//! the same constructor from that variant's unmutated layout, and the
//! mutated lanes for those cases are re-encoded from that variant
//! layout. Retaining a fixed binding and raw SDK operands does reach
//! the gadget, but this harness is not a private independent oracle.
//!
//! Category honesty: every `is_err` here is an `IrSource::check`
//! PREPROCESSING failure only. Assertion, equality, and bit-bound
//! bails are indistinguishable at this category. Nothing here is
//! circuit synthesis, MockProver satisfaction, a constraint-category
//! proof, proving, or native evidence. A synthesis-level test remains
//! a separate mandatory item before any such claim.

use super::*;
use midnight_transient_crypto::curve::Fr;
use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage, Zkir};
use midnight_zkir_v3::ir::Operand as SdkOperand;
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_core::sherlock::helper_machine::c6_phys_host::encode_metadata;
use mori_core::sherlock::helper_machine::complete_observation::WidthsSnapshot;
use mori_core::sherlock::helper_machine::shared::{
    DigestC, FinanceInput, GrammarDomain, IdentityC, LayoutCapacitiesC, RunIdC, RunIntC,
};
use std::sync::Arc;

use super::super::c6_push_step::meta_admission_bounds_frames_offset;

fn limits() -> C6HostLimits {
    C6HostLimits { max_total_lanes: 1 << 20, max_state_lanes: 1 << 20,
        max_metadata_bytes: 8192, max_string_bytes: 512, max_width_entries: 64 }
}

fn adm_bounds() -> BoundsC {
    let mut b = BoundsC::default();
    b.code_rows = 16; b.frames = 3; b.serialization_bytes = 1_000_000;
    b.max_reference_work = 77; b.step_cap = 9;
    b
}

fn layout_bounds() -> BoundsC {
    // Deliberately different from adm_bounds(): the two blocks are
    // bound to their own retained values, never equated.
    let mut b = BoundsC::default();
    b.code_rows = 24; b.frames = 5; b.serialization_bytes = 2_000_000;
    b.max_helper_work = 55;
    b
}

fn layout() -> LayoutSnapshot {
    LayoutSnapshot {
        layout_version: ("lv".into(), "1".into()),
        field_id: Some(("fid".into(), "".into())),
        widths: Some(WidthsSnapshot {
            index_widths: vec![("a".into(), 8), ("a".into(), 16), ("bé".into(), 32)],
            counter_widths: vec![],
            record_widths: vec![("z".into(), 64)],
        }),
        scalar_layout_id: None,
        bounds: layout_bounds(),
        capacities: LayoutCapacitiesC { name_rows: 16, name_bytes: 256, occurrence_path_slots: 32, value_slots: 2 },
        boundary_capacities_present: true,
    }
}

fn scalar_layout() -> LayoutSnapshot {
    let mut lay = layout();
    lay.scalar_layout_id = Some(("s".into(), "1".into()));
    lay
}

fn empty_widths_layout() -> LayoutSnapshot {
    let mut lay = layout();
    lay.widths = Some(WidthsSnapshot {
        index_widths: vec![],
        counter_widths: vec![],
        record_widths: vec![],
    });
    lay
}

fn ident(v: &str) -> IdentityC {
    IdentityC { profile_id: 2, grammar_domain: GrammarDomain::GenericNominalExtensionReserved,
        finance: FinanceInput::Absent, layout_version: (v.into(), "1".into()),
        layout_digest: DigestC::Unselected, bounds_digest: DigestC::Unselected,
        field_id: None, program_membership_digest: DigestC::Unselected }
}

fn meta(admission: BoundsC, layout: LayoutSnapshot) -> mori_core::sherlock::helper_machine::c6_phys_host::C6HostMetadata {
    mori_core::sherlock::helper_machine::c6_phys_host::C6HostMetadata {
        identity: ident("c6"), bounds: admission, layout,
        run_id: RunIdC { identity: ident("c6"), fuel_input: RunIntC::Nat(64), stack_limit_input: RunIntC::Nat(2) },
    }
}

fn binding() -> C6MetaConfigBinding {
    C6MetaConfigBinding::new(adm_bounds(), layout(), limits()).unwrap()
}

fn binding_for(lay: LayoutSnapshot) -> C6MetaConfigBinding {
    C6MetaConfigBinding::new(adm_bounds(), lay, limits()).unwrap()
}

fn lanes(m: &mori_core::sherlock::helper_machine::c6_phys_host::C6HostMetadata) -> Vec<u128> {
    encode_metadata(m, &limits()).unwrap()
}

fn lanes_for_layout(lay: LayoutSnapshot) -> Vec<u128> {
    lanes(&meta(adm_bounds(), lay))
}

fn vars(prefix: &str, n: usize) -> Vec<Operand> {
    (0..n).map(|i| Operand::Variable(Identifier(format!("%x.{prefix}.{i}")))).collect()
}

fn true_offset() -> usize {
    let m = meta(adm_bounds(), layout());
    meta_admission_bounds_frames_offset(&m, &lanes(&m)).unwrap()
}

fn fr_lanes(values: &[u128]) -> Vec<Fr> {
    values.iter().map(|&v| Fr::from(v)).collect()
}

fn two_pow_128_fr() -> Fr {
    let mut bytes = [0u8; 32];
    bytes[16] = 1;
    Fr::from_le_bytes(&bytes).unwrap()
}

/// SDK check over raw public operands with actual `Fr` statement
/// values and an explicit in-range-or-host-refused offset parameter.
/// `Ok`/`Err` is the actual preprocessing result category; nothing
/// here is a host filter except the emitter's own pre-emission shape
/// admission, which the positive controls keep satisfied.
fn check_fr(gate: Fr, pre: Vec<Fr>, post: Vec<Fr>, bound: Option<C6MetaConfigBinding>, offset: usize) -> Result<Vec<Option<usize>>, String> {
    let gate_op = Operand::Variable(Identifier("%x.gate".into()));
    let pre_ops = vars("pre", pre.len());
    let post_ops = vars("post", post.len());
    let mut instructions = Vec::new();
    for op in std::iter::once(&gate_op).chain(pre_ops.iter()).chain(post_ops.iter()) {
        if let Operand::Variable(id) = op {
            instructions.push(Instruction::PublicInput { guard: None, val_t: IrType::Native, output: id.clone() });
        }
    }
    let block = match &bound {
        Some(b) => emit_c6_meta_bound_canonical_block(&C6MetaBoundCanonicalInputs {
            gate: gate_op.clone(), pre_meta: pre_ops.clone(), post_meta: post_ops.clone(),
            limits: limits(), meta_bounds_frames_offset: offset, binding: b.clone(),
        }, "x").map_err(|e| e.to_string())?,
        None => emit_c6_meta_canonical_block(&C6MetaCanonicalInputs {
            gate: gate_op.clone(), pre_meta: pre_ops.clone(), post_meta: post_ops.clone(),
            limits: limits(), meta_bounds_frames_offset: offset,
        }, "x").map_err(|e| e.to_string())?,
    };
    instructions.extend(block.instructions);
    let mut statement_ops = vec![gate_op];
    statement_ops.extend(pre_ops);
    statement_ops.extend(post_ops);
    instructions.push(Instruction::Impact { guard: SdkOperand::Immediate(Fr::from(1u64)), inputs: statement_ops });
    let ir = IrSource { version: Default::default(), inputs: vec![], outputs: vec![],
        do_communications_commitment: false, instructions: Arc::new(instructions) };
    let mut statement = vec![gate];
    statement.extend(pre);
    statement.extend(post);
    let preimage = ProofPreimage { inputs: vec![], private_transcript: vec![],
        public_transcript_inputs: statement.clone(), public_transcript_outputs: statement,
        binding_input: Fr::from(0u64), communications_commitment: None,
        key_location: KeyLocation("c6_meta_bound_test".into()) };
    ir.check(&preimage).map_err(|e| format!("{e}"))
}

/// SDK check over raw public operands with an explicit offset.
fn check_with_offset(gate: u128, pre: Vec<u128>, post: Vec<u128>, bound: Option<C6MetaConfigBinding>, offset: usize) -> Result<Vec<Option<usize>>, String> {
    check_fr(Fr::from(gate), fr_lanes(&pre), fr_lanes(&post), bound, offset)
}

/// SDK check over raw public operands at the true offset derived
/// from the good fixture. `Ok`/`Err` is the actual preprocessing
/// result category; nothing here is a host filter.
fn check(gate: u128, pre: Vec<u128>, post: Vec<u128>, bound: Option<C6MetaConfigBinding>) -> Result<Vec<Option<usize>>, String> {
    check_with_offset(gate, pre, post, bound, true_offset())
}

/// One isolated mutated witness: the fixed retained binding must
/// reject it in preprocessing, and the same structurally valid raw
/// lanes must remain eligible without a binding.
fn assert_bound_case(m: Vec<u128>, bound: C6MetaConfigBinding, label: &str) {
    assert!(check(1, m.clone(), m.clone(), Some(bound)).is_err(), "{label}: bound must reject");
    assert!(check(1, m.clone(), m, None).is_ok(), "{label}: unbound must stay eligible");
}

fn assert_standard_bound_case(m: Vec<u128>, label: &str) {
    assert_bound_case(m, binding(), label);
}

fn set_bound(b: &mut BoundsC, index: usize) {
    match index {
        0 => b.code_rows += 1, 1 => b.function_rows += 1, 2 => b.code_child_slots += 1,
        3 => b.parameter_slots += 1, 4 => b.pattern_rows += 1, 5 => b.match_plan_rows += 1,
        6 => b.match_arm_rows += 1, 7 => b.type_rows += 1, 8 => b.constructor_rows += 1,
        9 => b.constructor_field_slots += 1, 10 => b.max_arity += 1, 11 => b.constant_nodes += 1,
        12 => b.constant_field_slots += 1, 13 => b.data_nodes += 1, 14 => b.data_field_slots += 1,
        15 => b.environment_cells += 1, 16 => b.frames += 1, 17 => b.helper_tasks += 1,
        18 => b.serialization_bytes += 1, 19 => b.max_reference_work += 1,
        20 => b.max_helper_work += 1, _ => b.step_cap += 1,
    }
}

fn set_capacity(lay: &mut LayoutSnapshot, index: usize) {
    match index {
        0 => lay.capacities.name_rows += 1,
        1 => lay.capacities.name_bytes += 1,
        2 => lay.capacities.occurrence_path_slots += 1,
        _ => lay.capacities.value_slots += 1,
    }
}

#[test]
fn valid_bound_meta_accepted_and_unbound_also_accepted() {
    let good = lanes(&meta(adm_bounds(), layout()));
    assert!(check(1, good.clone(), good.clone(), Some(binding())).is_ok());
    assert!(check(1, good.clone(), good, None).is_ok());
}

#[test]
fn every_admission_and_layout_bound_field_is_bound() {
    for index in 0..22 {
        let mut adm = adm_bounds(); set_bound(&mut adm, index);
        let mutated = lanes(&meta(adm, layout()));
        assert!(check(1, mutated.clone(), mutated.clone(), Some(binding())).is_err(), "admission {index}");
        assert!(check(1, mutated.clone(), mutated, None).is_ok(), "admission {index} unbound stays eligible");
        let mut lay = layout(); set_bound(&mut lay.bounds, index);
        let mutated = lanes(&meta(adm_bounds(), lay));
        assert!(check(1, mutated.clone(), mutated.clone(), Some(binding())).is_err(), "layout {index}");
        assert!(check(1, mutated.clone(), mutated, None).is_ok(), "layout {index} unbound stays eligible");
    }
}

#[test]
fn capacities_each_capacity_is_bound_individually() {
    let names = ["name_rows", "name_bytes", "occurrence_path_slots", "value_slots"];
    for index in 0..4 {
        let mut lay = layout();
        set_capacity(&mut lay, index);
        assert_standard_bound_case(lanes_for_layout(lay), names[index]);
    }
}

#[test]
fn layout_shape_mutations_are_bound_with_unbound_controls() {
    let mut lay = layout(); lay.boundary_capacities_present = false;
    assert_standard_bound_case(lanes_for_layout(lay), "boundary presence");
    let mut lay = layout(); lay.layout_version = ("lx".into(), "1".into());
    assert_standard_bound_case(lanes_for_layout(lay), "layout version first component");
    let mut lay = layout(); lay.field_id = None;
    assert_standard_bound_case(lanes_for_layout(lay), "field_id presence");
    let mut lay = layout(); lay.scalar_layout_id = Some(("s".into(), "1".into()));
    assert_standard_bound_case(lanes_for_layout(lay), "scalar presence None to Some");
    let mut lay = layout();
    if let Some(w) = &mut lay.widths { w.index_widths.swap(0, 1); }
    assert_standard_bound_case(lanes_for_layout(lay), "width order/duplicates");
}

#[test]
fn variable_layout_content_is_bound_with_unbound_controls() {
    let mut lay = layout(); lay.layout_version.1 = "2".into();
    assert_standard_bound_case(lanes_for_layout(lay), "layout_version second component");
    let mut lay = layout(); lay.field_id = Some(("fie".into(), "".into()));
    assert_standard_bound_case(lanes_for_layout(lay), "field_id first component, second present-empty");
    let mut lay = layout(); lay.field_id = Some(("fid".into(), "x".into()));
    assert_standard_bound_case(lanes_for_layout(lay), "field_id second component empty to non-empty");
    let mut lay = layout(); lay.widths = None;
    assert_standard_bound_case(lanes_for_layout(lay), "widths Some to None");
    let mut lay = layout();
    if let Some(w) = &mut lay.widths { w.index_widths.push(("q".into(), 8)); }
    assert_standard_bound_case(lanes_for_layout(lay), "index width list count");
    let mut lay = layout();
    if let Some(w) = &mut lay.widths { w.counter_widths.push(("n".into(), 8)); }
    assert_standard_bound_case(lanes_for_layout(lay), "counter width list count");
    let mut lay = layout();
    if let Some(w) = &mut lay.widths { w.record_widths.push(("y".into(), 16)); }
    assert_standard_bound_case(lanes_for_layout(lay), "record width list count");
    let mut lay = layout();
    if let Some(w) = &mut lay.widths { w.index_widths[0].0 = "b".into(); }
    assert_standard_bound_case(lanes_for_layout(lay), "width name byte, ASCII name, count and width unchanged");
    let mut lay = layout();
    if let Some(w) = &mut lay.widths { w.index_widths[2].0 = "bè".into(); }
    assert_standard_bound_case(lanes_for_layout(lay), "width name byte, non-ASCII name, count and width unchanged");
    let mut lay = layout();
    if let Some(w) = &mut lay.widths { w.index_widths[0].1 = 9; }
    assert_standard_bound_case(lanes_for_layout(lay), "u32 width value, name and count unchanged");
}

#[test]
fn scalar_some_binding_and_empty_width_lists_are_bound() {
    let good_scalar = lanes_for_layout(scalar_layout());
    assert!(check(1, good_scalar.clone(), good_scalar.clone(), Some(binding_for(scalar_layout()))).is_ok(),
        "scalar Some positive control against Some binding");
    assert!(check(1, good_scalar.clone(), good_scalar, None).is_ok(),
        "scalar Some positive control unbound");
    let mut lay = scalar_layout(); lay.scalar_layout_id = Some(("t".into(), "1".into()));
    assert_bound_case(lanes_for_layout(lay), binding_for(scalar_layout()), "scalar first component against Some binding");
    let mut lay = scalar_layout(); lay.scalar_layout_id = Some(("s".into(), "2".into()));
    assert_bound_case(lanes_for_layout(lay), binding_for(scalar_layout()), "scalar second component against Some binding");
    let mut lay = scalar_layout(); lay.scalar_layout_id = None;
    assert_bound_case(lanes_for_layout(lay), binding_for(scalar_layout()), "scalar Some to None against Some binding");
    let good_empty = lanes_for_layout(empty_widths_layout());
    assert!(check(1, good_empty.clone(), good_empty.clone(), Some(binding_for(empty_widths_layout()))).is_ok(),
        "widths Some with three empty lists positive control");
    assert!(check(1, good_empty.clone(), good_empty, None).is_ok(),
        "widths Some with three empty lists unbound");
    let mut lay = empty_widths_layout(); lay.widths = None;
    assert_bound_case(lanes_for_layout(lay), binding_for(empty_widths_layout()),
        "widths Some empty lists to None against empty-list binding");
}

#[test]
fn runtime_identity_and_run_values_stay_free_with_identical_ir() {
    let mut other = meta(adm_bounds(), layout());
    other.identity = ident("d7");
    other.run_id.identity = ident("e8");
    other.run_id.fuel_input = RunIntC::Nat(9_999);
    other.run_id.stack_limit_input = RunIntC::NatAboveU128;
    let a = lanes(&meta(adm_bounds(), layout()));
    let b = lanes(&other);
    assert_eq!(a.len(), b.len());
    assert!(check(1, b.clone(), b.clone(), Some(binding())).is_ok(), "same-shape runtime variation eligible");
    let ops = |n: usize| (vars("pre", n), vars("post", n));
    let (ap, aq) = ops(a.len());
    let (bp, bq) = ops(b.len());
    let gate = Operand::Variable(Identifier("%x.gate".into()));
    let offset = meta_admission_bounds_frames_offset(&meta(adm_bounds(), layout()), &a).unwrap();
    let mk = |pre: Vec<Operand>, post: Vec<Operand>| emit_c6_meta_bound_canonical_block(
        &C6MetaBoundCanonicalInputs { gate: gate.clone(), pre_meta: pre, post_meta: post,
            limits: limits(), meta_bounds_frames_offset: offset, binding: binding() }, "x").unwrap().instructions;
    assert_eq!(mk(ap, aq), mk(bp, bq), "same declared shape yields identical IR");
}

#[test]
fn pre_and_post_are_bound_independently() {
    let good = lanes(&meta(adm_bounds(), layout()));
    assert!(check(1, good.clone(), good.clone(), Some(binding())).is_ok(), "good/good positive control");
    let mut bad_adm = adm_bounds(); bad_adm.frames += 1;
    let bad = lanes(&meta(bad_adm, layout()));
    assert!(check(1, good.clone(), bad.clone(), Some(binding())).is_err(), "admission frames post only");
    assert!(check(1, bad.clone(), good.clone(), Some(binding())).is_err(), "admission frames pre only");
    assert!(check(1, good.clone(), bad.clone(), None).is_ok(), "admission frames post only unbound");
    assert!(check(1, bad, good.clone(), None).is_ok(), "admission frames pre only unbound");
    let mut lay = layout(); set_bound(&mut lay.bounds, 0);
    let bad = lanes_for_layout(lay);
    assert!(check(1, good.clone(), bad.clone(), Some(binding())).is_err(), "layout bounds post only");
    assert!(check(1, bad.clone(), good.clone(), Some(binding())).is_err(), "layout bounds pre only");
    assert!(check(1, good.clone(), bad.clone(), None).is_ok(), "layout bounds post only unbound");
    assert!(check(1, bad, good.clone(), None).is_ok(), "layout bounds pre only unbound");
    let mut lay = layout(); lay.layout_version.1 = "2".into();
    let bad = lanes_for_layout(lay);
    assert!(check(1, good.clone(), bad.clone(), Some(binding())).is_err(), "variable layout post only");
    assert!(check(1, bad.clone(), good.clone(), Some(binding())).is_err(), "variable layout pre only");
    assert!(check(1, good.clone(), bad.clone(), None).is_ok(), "variable layout post only unbound");
    assert!(check(1, bad, good, None).is_ok(), "variable layout pre only unbound");
}

#[test]
fn gate_zero_admits_max_lanes_and_gate_two_rejects_in_preprocessing() {
    let good = lanes(&meta(adm_bounds(), layout()));
    let maxed = vec![u128::MAX; good.len()];
    assert!(check(0, maxed.clone(), maxed.clone(), Some(binding())).is_ok(), "gate 0 bound: raw 128-bit lanes only");
    assert!(check(0, maxed.clone(), maxed, None).is_ok(), "gate 0 unbound: raw 128-bit lanes only");
    assert!(check(2, good.clone(), good, Some(binding())).is_err(), "gate 2 fails boolean constraint in preprocessing");
}

#[test]
fn shifted_offset_and_malformed_utf8_reject() {
    let m = meta(adm_bounds(), layout());
    let good = lanes(&m);
    let offset = meta_admission_bounds_frames_offset(&m, &good).unwrap();
    assert!(offset > 0 && offset < good.len(), "shifted offset remains in range");
    assert!(check_with_offset(1, good.clone(), good.clone(), Some(binding()), offset).is_ok(), "positive control: true offset bound");
    assert!(check_with_offset(1, good.clone(), good.clone(), Some(binding()), offset - 1).is_err(), "negative: shifted offset bound reaches phase/sub tie");
    assert!(check_with_offset(1, good.clone(), good.clone(), None, offset).is_ok(), "positive control: true offset unbound");
    assert!(check_with_offset(1, good.clone(), good.clone(), None, offset - 1).is_err(), "negative: shifted offset unbound reaches phase/sub tie");
    let mut bad = good.clone();
    let pos = bad.iter().position(|&v| v == 99).unwrap();
    bad[pos] = 0xFF;
    assert!(check(1, bad.clone(), bad, Some(binding())).is_err(), "malformed UTF-8");
}

#[test]
fn limits_mismatch_and_reserved_operand_refuse_before_emission() {
    let mut wrong = limits(); wrong.max_width_entries = 65;
    let inputs = C6MetaBoundCanonicalInputs { gate: Operand::Variable(Identifier("%x.gate".into())),
        pre_meta: vars("pre", 8), post_meta: vars("post", 8),
        limits: wrong, meta_bounds_frames_offset: 0, binding: binding() };
    assert!(matches!(emit_c6_meta_bound_canonical_block(&inputs, "x"), Err(C6MetaCanonicalError::BindingLimitsMismatch)));
    let reserved = C6MetaBoundCanonicalInputs { gate: Operand::Variable(Identifier("%x.c6meta.temp.0".into())),
        pre_meta: vars("pre", 8), post_meta: vars("post", 8),
        limits: limits(), meta_bounds_frames_offset: 0, binding: binding() };
    assert!(matches!(emit_c6_meta_bound_canonical_block(&reserved, "x"), Err(C6MetaCanonicalError::ReservedOperand { .. })));
    assert!(C6MetaConfigBinding::new(adm_bounds(), layout(), {
        let mut l = limits(); l.max_string_bytes = 1; l
    }).is_err(), "constructor refuses layout the retained limits cannot carry");
}

#[test]
fn stateful_emitter_reuses_no_temps_across_bound_blocks() {
    let good = lanes(&meta(adm_bounds(), layout()));
    let mk = || C6MetaBoundCanonicalInputs { gate: Operand::Variable(Identifier("%x.gate".into())),
        pre_meta: vars("pre", good.len()), post_meta: vars("post", good.len()),
        limits: limits(),
        meta_bounds_frames_offset: meta_admission_bounds_frames_offset(&meta(adm_bounds(), layout()), &good).unwrap(),
        binding: binding() };
    let mut emitter = C6MetaCanonicalEmitter::new("x").unwrap();
    let first = emitter.emit_bound(&mk()).unwrap();
    let serial_after_first = emitter.next_temp();
    assert!(serial_after_first > 0 && !first.instructions.is_empty());
    let second = emitter.emit_bound(&mk()).unwrap();
    assert!(emitter.next_temp() > serial_after_first && !second.instructions.is_empty());
}

#[test]
fn raw_fr_lane_and_gate_at_2_pow_128_reject_in_preprocessing() {
    let good = lanes(&meta(adm_bounds(), layout()));
    let offset = true_offset();
    let maxed_fr = vec![Fr::from(u128::MAX); good.len()];
    assert!(check_fr(Fr::from(0u64), maxed_fr.clone(), maxed_fr.clone(), Some(binding()), offset).is_ok(), "gate 0 bound: Fr 2^128 - 1 lanes stay inside ConstrainBits(128)");
    assert!(check_fr(Fr::from(0u64), maxed_fr.clone(), maxed_fr, None, offset).is_ok(), "gate 0 unbound: Fr 2^128 - 1 lanes stay inside ConstrainBits(128)");
    let mut over_bound = fr_lanes(&good);
    over_bound[0] = two_pow_128_fr();
    assert!(check_fr(Fr::from(0u64), over_bound.clone(), over_bound.clone(), Some(binding()), offset).is_err(), "bound: raw Fr lane at 2^128 fails unconditional 128-bit preprocessing");
    assert!(check_fr(Fr::from(0u64), over_bound.clone(), over_bound, None, offset).is_err(), "unbound: raw Fr lane at 2^128 fails unconditional 128-bit preprocessing");
    let good_fr = fr_lanes(&good);
    assert!(check_fr(two_pow_128_fr(), good_fr.clone(), good_fr.clone(), Some(binding()), offset).is_err(), "bound: raw Fr gate at 2^128 is not boolean in preprocessing");
    assert!(check_fr(two_pow_128_fr(), good_fr.clone(), good_fr, None, offset).is_err(), "unbound: raw Fr gate at 2^128 is not boolean in preprocessing");
}
