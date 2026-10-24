//! UNRUN tests for `c6_checked_value_read`.
//!
//! Denotation labels used below:
//! - "raw codec domain": lanes produced by the public Core program/data
//!   codecs and admitted by the reader's emitted raw128/boolean gates.
//! - "owner-admitted held-value domain": NOT established here. In
//!   particular, `marker_free` is asserted only as the exact raw Const
//!   lane exported by this reader. SI17's held-value marker predicate is
//!   a retained premise and is neither erased nor enforced by these
//!   tests.
//!
//! Output order used by every literal `Exp` in this file:
//! [selected_found, is_data, kind_code,
//!  is_uint, is_bool, is_nil, is_cons, is_leaf, is_branch,
//!  is_constructor, is_marker,
//!  marker_free_present, marker_free,
//!  uint_present, uint_value,
//!  bool_present, bool_value,
//!  ctor_present, type_id, ctor_id,
//!  slice_present, slice_start, slice_len]
//!
//! Kind codes: 0 UInt, 1 Bool, 2 Nil, 3 Cons, 4 Leaf, 5 Branch,
//! 6 Constructor, 7 Marker.

use super::c6_checked_value_read::{
    emit_c6_checked_value_read_block, scalar_operand_view, C6CheckedValueReadBlock,
    C6CheckedValueReadEmitter, C6CheckedValueReadError, C6CheckedValueReadInputs,
};
use super::c6_data_canonical::c6_data_geometry;
use midnight_proofs::circuit::Value;
use midnight_proofs::dev::MockProver;
use midnight_transient_crypto::curve::{outer, Fr};
use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage};
use midnight_zk_stdlib::MidnightCircuit;
use midnight_zkir_v3::ir::{IrMinorVersion, Operand};
use midnight_zkir_v3::ir_types::{IrType, IrValue};
use midnight_zkir_v3::{Identifier, Instruction, IrSource, Preprocessed};
use mori_core::sherlock::helper_machine::c6_phys::{C6PhysCaps, C6_PHYS_MAGIC, C6_PHYS_VERSION};
use mori_core::sherlock::helper_machine::c6_phys_data::{
    encode_data_row, C6StateCaps, DATA_HEADER_LANES, DATA_ROW_LANES, DATA_SECTION_TAG,
};
use mori_core::sherlock::helper_machine::c6_phys_program::encode_program;
use mori_core::sherlock::helper_machine::complete_observation::{
    ConstFieldsSnapshot, ConstantSnapshot, DataFieldsSnapshot, DataSnapshot,
    ExpressionSnapshot, ProgramSnapshot,
};
use mori_core::sherlock::helper_machine::shared::adt::{CtorIdC, TypeIdC};
use mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::{
    CtorDeclSrc, CtorFieldSrc, TypeDeclSrc, TypeExprSrc,
};
use serde_json::json;
use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;

type Exp = [u128; 23];
const ZERO: Exp = [0; 23];
const OUT_KIND: usize = 2;
const OUT_UINT_VALUE: usize = 14;

fn caps() -> C6StateCaps {
    let mut phys = C6PhysCaps::test_caps();
    // Test-local capacity for the advertised nine-row `std_data`
    // fixture. With `DATA_ROW_LANES == 5`, the DATA partition is
    // `[7, 52)`; the field, env, and value partitions follow it
    // through `c6_data_geometry`. This changes no production cap or
    // admission policy.
    phys.max_data_nodes = 9;
    phys.max_env_cells = 2;
    phys.max_value_slots = 2;
    C6StateCaps { phys, max_data_field_slots: 8, max_helper_tasks: 0 }
}

fn cf(start: usize, len: usize) -> ConstFieldsSnapshot {
    ConstFieldsSnapshot { start, len }
}
fn df(start: usize, len: usize) -> DataFieldsSnapshot {
    DataFieldsSnapshot { start, len }
}

fn empty_program() -> ProgramSnapshot {
    ProgramSnapshot {
        entry: 0,
        expressions: Vec::new(),
        occurrences: Vec::new(),
        occurrence_paths: Vec::new(),
        children: Vec::new(),
        functions: Vec::new(),
        parameters: Vec::new(),
        arms: Vec::new(),
        patterns: Vec::new(),
        pattern_children: Vec::new(),
        types: Vec::new(),
        type_args: Vec::new(),
        constructors: Vec::new(),
        ctor_fields: Vec::new(),
        decl_param_counts: Vec::new(),
        decl_ctor_counts: Vec::new(),
        decl_templates: Vec::new(),
        plan_nodes: Vec::new(),
        plan_cases: Vec::new(),
        field_paths: Vec::new(),
        constants: Vec::new(),
        constant_fields: Vec::new(),
        names: Vec::new(),
    }
}

fn prog_with_constants(constants: Vec<ConstantSnapshot>) -> Vec<u128> {
    let mut p = empty_program();
    p.constants = constants;
    encode_program(&p, &caps().phys).expect("test program encodes")
}

fn prog_of(p: &ProgramSnapshot) -> Vec<u128> {
    encode_program(p, &caps().phys).expect("test program encodes")
}

fn data_with(rows: Vec<DataSnapshot>) -> Vec<u128> {
    let c = caps();
    let g = c6_data_geometry(&c).expect("data geometry");
    let mut out = vec![0u128; g.total];
    out[0] = C6_PHYS_MAGIC;
    out[1] = C6_PHYS_VERSION;
    out[2] = DATA_SECTION_TAG;
    out[3] = rows.len() as u128;
    // data_fields, env and value_slots are empty in these fixtures.
    for (i, row) in rows.iter().enumerate() {
        let b = g.data_base + i * DATA_ROW_LANES;
        out[b..b + DATA_ROW_LANES].copy_from_slice(&encode_data_row(row));
    }
    out
}

/// Const fixture, indices:
/// 0 U128/0/mf1, 1 U128/MAX/mf0, 2 Bool/false/mf1, 3 Bool/true/mf0,
/// 4 Nil/mf1, 5 Cons/mf0, 6 Leaf/mf1, 7 Branch/mf0,
/// 8 Constructor/mf1, 9 Marker/mf0, 10 Marker/mf1.
fn std_consts() -> Vec<ConstantSnapshot> {
    vec![
        ConstantSnapshot::U128 { marker_free: true, n: 0 },
        ConstantSnapshot::U128 { marker_free: false, n: u128::MAX },
        ConstantSnapshot::Bool { marker_free: true, b: false },
        ConstantSnapshot::Bool { marker_free: false, b: true },
        ConstantSnapshot::Nil { marker_free: true },
        ConstantSnapshot::Cons { marker_free: false, fields: cf(1, 2) },
        ConstantSnapshot::Leaf { marker_free: true, fields: cf(3, 1) },
        ConstantSnapshot::Branch { marker_free: false, fields: cf(0, 2) },
        ConstantSnapshot::Constructor {
            marker_free: true,
            type_id: TypeIdC(1),
            ctor_id: CtorIdC(2),
            fields: cf(2, 1),
        },
        ConstantSnapshot::U128OutOfRange { marker_free: false },
        ConstantSnapshot::U128OutOfRange { marker_free: true },
    ]
}

/// DATA fixture, indices:
/// 0 U128/MAX, 1 Bool/false, 2 Nil, 3 Cons, 4 Leaf, 5 Branch,
/// 6 Constructor, 7 Bool/true, 8 U128/0.
fn std_data() -> Vec<DataSnapshot> {
    vec![
        DataSnapshot::U128 { n: u128::MAX },
        DataSnapshot::Bool { b: false },
        DataSnapshot::Nil,
        DataSnapshot::Cons { fields: df(0, 2) },
        DataSnapshot::Leaf { fields: df(2, 1) },
        DataSnapshot::Branch { fields: df(3, 0) },
        DataSnapshot::Constructor {
            type_id: TypeIdC(1),
            ctor_id: CtorIdC(1),
            fields: df(1, 2),
        },
        DataSnapshot::Bool { b: true },
        DataSnapshot::U128 { n: 0 },
    ]
}

fn outputs_of(b: &C6CheckedValueReadBlock) -> Vec<Operand> {
    vec![
        b.selected_found.clone(),
        b.is_data.clone(),
        b.kind_code.clone(),
        b.is_uint.clone(),
        b.is_bool.clone(),
        b.is_nil.clone(),
        b.is_cons.clone(),
        b.is_leaf.clone(),
        b.is_branch.clone(),
        b.is_constructor.clone(),
        b.is_marker.clone(),
        b.marker_free_present.clone(),
        b.marker_free.clone(),
        b.uint_present.clone(),
        b.uint_value.clone(),
        b.bool_present.clone(),
        b.bool_value.clone(),
        b.ctor_present.clone(),
        b.type_id.clone(),
        b.ctor_id.clone(),
        b.slice_present.clone(),
        b.slice_start.clone(),
        b.slice_len.clone(),
    ]
}

fn build_ir(prog_len: usize, data_len: usize) -> Result<(IrSource, Vec<Identifier>), String> {
    let total = 3 + prog_len + data_len + 23;
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut transcript: Vec<Operand> = Vec::new();
    let mut public_ids: Vec<Identifier> = Vec::new();
    for i in 0..total {
        let id = Identifier(format!("%cvr_test.public.{i}"));
        instructions.push(Instruction::PublicInput {
            guard: None,
            val_t: IrType::Native,
            output: id.clone(),
        });
        transcript.push(Operand::Variable(id.clone()));
        public_ids.push(id);
    }
    let inputs = C6CheckedValueReadInputs {
        gate: transcript[0].clone(),
        ref_kind_is_data: transcript[1].clone(),
        ref_index: transcript[2].clone(),
        pre_prog: transcript[3..3 + prog_len].to_vec(),
        pre_data: transcript[3 + prog_len..3 + prog_len + data_len].to_vec(),
        caps: caps(),
    };
    let block = emit_c6_checked_value_read_block(&inputs, "cvr_test")
        .map_err(|e| e.to_string())?;
    // Capture the exported operands in their declared order before
    // moving the instruction vector out of the block.
    let outputs = outputs_of(&block);
    instructions.extend(block.instructions);
    // Explicit output claims: every exported projection is constrained
    // to an independently supplied public expected lane. This is the
    // circuit-row-binding surface for wrong-claim negatives; no host
    // struct comparison is used as a substitute.
    let expected = &transcript[3 + prog_len + data_len..];
    for (out, exp) in outputs.into_iter().zip(expected.iter()) {
        instructions.push(Instruction::ConstrainEq { a: out, b: exp.clone() });
    }
    instructions.push(Instruction::Impact {
        guard: Operand::Immediate(Fr::from(1u64)),
        inputs: transcript,
    });
    let ir = IrSource {
        version: IrMinorVersion::V0,
        inputs: Vec::new(),
        outputs: Vec::new(),
        do_communications_commitment: false,
        instructions: Arc::new(instructions),
    };
    let mut disk = serde_json::to_value(&ir).map_err(|e| e.to_string())?;
    disk["version"] = json!({ "major": 3, "minor": 0 });
    let serialized = serde_json::to_string(&disk).map_err(|e| e.to_string())?;
    let loaded = IrSource::load(serialized.as_bytes()).map_err(|e| e.to_string())?;
    Ok((loaded, public_ids))
}

fn statement_u128(
    gate: u128,
    is_data: u128,
    index: u128,
    prog: &[u128],
    data: &[u128],
    exp: &Exp,
) -> Vec<Fr> {
    let mut s = vec![Fr::from(gate), Fr::from(is_data), Fr::from(index)];
    s.extend(prog.iter().map(|&v| Fr::from(v)));
    s.extend(data.iter().map(|&v| Fr::from(v)));
    s.extend(exp.iter().map(|&v| Fr::from(v)));
    s
}

/// Real pinned-SDK preprocessing check only (`ProofPreimage::check`).
/// UNRUN here. No synthesis/proof/native claim follows from this helper.
fn run_check_fr(statement: Vec<Fr>, prog_len: usize, data_len: usize) -> Result<(), String> {
    let (loaded, _) = build_ir(prog_len, data_len)?;
    let preimage = ProofPreimage {
        inputs: Vec::new(),
        private_transcript: Vec::new(),
        public_transcript_inputs: statement.clone(),
        public_transcript_outputs: statement,
        binding_input: Fr::from(0u64),
        communications_commitment: None,
        key_location: KeyLocation("sherlock_c6_checked_value_read_check".into()),
    };
    preimage.check(&loaded).map(|_| ()).map_err(|e| e.to_string())
}

fn run_check(
    gate: u128,
    is_data: u128,
    index: u128,
    prog: &[u128],
    data: &[u128],
    exp: &Exp,
) -> Result<(), String> {
    run_check_fr(
        statement_u128(gate, is_data, index, prog, data, exp),
        prog.len(),
        data.len(),
    )
}

fn two_pow_128() -> Fr {
    Fr::from(u128::MAX) + Fr::from(1u64)
}

#[derive(Debug, PartialEq, Eq)]
enum SynthesisRejection {
    ConstraintFailure(String),
    SynthesisFailure(String),
}

/// Actual MockProver synthesis through the emitted reader circuit.
/// UNRUN here. Preprocessing rejection remains distinguishable through
/// `run_check_fr`; a panic is never converted into a rejection category.
fn run_synthesis_fr(
    statement: Vec<Fr>,
    prog_len: usize,
    data_len: usize,
) -> Result<(), SynthesisRejection> {
    let (ir, public_ids) =
        build_ir(prog_len, data_len).map_err(SynthesisRejection::SynthesisFailure)?;
    let mut memory: HashMap<Identifier, IrValue> = HashMap::new();
    for (id, value) in public_ids.iter().zip(statement.iter()) {
        memory.insert(id.clone(), IrValue::Native(*value));
    }
    let instance: Vec<outer::Scalar> = statement.iter().map(|v| v.0).collect();
    let mut pis: Vec<outer::Scalar> = vec![Fr::from(0u64).0];
    pis.extend(instance.iter().copied());
    let preimage = Preprocessed {
        memory,
        pis: pis.clone(),
        pi_skips: vec![None],
        binding_input: Fr::from(0u64).0,
        comm_comm: None,
    };
    let circuit = MidnightCircuit::new(
        &ir,
        Value::known(instance.clone()),
        Value::known(preimage),
        Some(16),
    );
    match MockProver::<outer::Scalar>::run(&circuit, vec![Vec::new(), pis]) {
        Ok(prover) => prover
            .verify()
            .map_err(|e| SynthesisRejection::ConstraintFailure(format!("{e:?}"))),
        Err(e) => Err(SynthesisRejection::SynthesisFailure(format!("{e:?}"))),
    }
}

#[test]
fn const_all_eight_variants_full_projections() {
    let prog = prog_with_constants(std_consts());
    let data = data_with(vec![DataSnapshot::Nil]);
    // Literal expectations in Exp order. marker_free_present=1 for all
    // Const refs; payload/ctor/slice lanes are exact-zero when inactive.
    let cases: [(usize, Exp); 11] = [
        (0, [1,0,0, 1,0,0,0,0,0,0,0, 1,1, 1,0, 0,0, 0,0,0, 0,0,0]),
        (1, [1,0,0, 1,0,0,0,0,0,0,0, 1,0, 1,u128::MAX, 0,0, 0,0,0, 0,0,0]),
        (2, [1,0,1, 0,1,0,0,0,0,0,0, 1,1, 0,0, 1,0, 0,0,0, 0,0,0]),
        (3, [1,0,1, 0,1,0,0,0,0,0,0, 1,0, 0,0, 1,1, 0,0,0, 0,0,0]),
        (4, [1,0,2, 0,0,1,0,0,0,0,0, 1,1, 0,0, 0,0, 0,0,0, 0,0,0]),
        (5, [1,0,3, 0,0,0,1,0,0,0,0, 1,0, 0,0, 0,0, 0,0,0, 1,1,2]),
        (6, [1,0,4, 0,0,0,0,1,0,0,0, 1,1, 0,0, 0,0, 0,0,0, 1,3,1]),
        (7, [1,0,5, 0,0,0,0,0,1,0,0, 1,0, 0,0, 0,0, 0,0,0, 1,0,2]),
        (8, [1,0,6, 0,0,0,0,0,0,1,0, 1,1, 0,0, 0,0, 1,1,2, 1,2,1]),
        (9, [1,0,7, 0,0,0,0,0,0,0,1, 1,0, 0,0, 0,0, 0,0,0, 0,0,0]),
        (10, [1,0,7, 0,0,0,0,0,0,0,1, 1,1, 0,0, 0,0, 0,0,0, 0,0,0]),
    ];
    for (index, exp) in cases {
        assert!(
            run_check(1, 0, index as u128, &prog, &data, &exp).is_ok(),
            "Const index {index} full projection"
        );
    }
}

#[test]
fn data_all_seven_variants_full_projections() {
    let prog = prog_with_constants(vec![ConstantSnapshot::Nil { marker_free: true }]);
    let data = data_with(std_data());
    // marker_free_present=0 and marker_free=0 for every DATA ref.
    let cases: [(usize, Exp); 9] = [
        (0, [1,1,0, 1,0,0,0,0,0,0,0, 0,0, 1,u128::MAX, 0,0, 0,0,0, 0,0,0]),
        (1, [1,1,1, 0,1,0,0,0,0,0,0, 0,0, 0,0, 1,0, 0,0,0, 0,0,0]),
        (2, [1,1,2, 0,0,1,0,0,0,0,0, 0,0, 0,0, 0,0, 0,0,0, 0,0,0]),
        (3, [1,1,3, 0,0,0,1,0,0,0,0, 0,0, 0,0, 0,0, 0,0,0, 1,0,2]),
        (4, [1,1,4, 0,0,0,0,1,0,0,0, 0,0, 0,0, 0,0, 0,0,0, 1,2,1]),
        (5, [1,1,5, 0,0,0,0,0,1,0,0, 0,0, 0,0, 0,0, 0,0,0, 1,3,0]),
        (6, [1,1,6, 0,0,0,0,0,0,1,0, 0,0, 0,0, 0,0, 1,1,1, 1,1,2]),
        (7, [1,1,1, 0,1,0,0,0,0,0,0, 0,0, 0,0, 1,1, 0,0,0, 0,0,0]),
        (8, [1,1,0, 1,0,0,0,0,0,0,0, 0,0, 1,0, 0,0, 0,0,0, 0,0,0]),
    ];
    for (index, exp) in cases {
        assert!(
            run_check(1, 1, index as u128, &prog, &data, &exp).is_ok(),
            "DATA index {index} full projection"
        );
    }
}

#[test]
fn marker_free_raw_lane_both_ways_is_not_admission() {
    // Raw codec domain only. Both Marker rows, and a non-Marker row
    // with marker_free=false, keep the exact raw lane. This test makes
    // no SI17 held-value/admitted-domain claim, because this leaf
    // exports no such predicate.
    let prog = prog_with_constants(std_consts());
    let data = data_with(vec![DataSnapshot::Nil]);
    let marker_false: Exp =
        [1,0,7, 0,0,0,0,0,0,0,1, 1,0, 0,0, 0,0, 0,0,0, 0,0,0];
    let marker_true: Exp =
        [1,0,7, 0,0,0,0,0,0,0,1, 1,1, 0,0, 0,0, 0,0,0, 0,0,0];
    assert!(run_check(1, 0, 9, &prog, &data, &marker_false).is_ok());
    assert!(run_check(1, 0, 10, &prog, &data, &marker_true).is_ok());
    // Wrong raw marker_free claim reaches the circuit and rejects.
    assert!(run_check(1, 0, 9, &prog, &data, &marker_true).is_err());
    assert!(run_check(1, 0, 10, &prog, &data, &marker_false).is_err());
}

#[test]
fn same_numeric_index_const_vs_data() {
    let prog = prog_with_constants(std_consts());
    let data = data_with(std_data());
    // Index 2 is Const Bool/false, but DATA Nil. The views must not
    // coincide in kind, payload presence, or marker_free presence.
    let const_exp: Exp =
        [1,0,1, 0,1,0,0,0,0,0,0, 1,1, 0,0, 1,0, 0,0,0, 0,0,0];
    let data_exp: Exp =
        [1,1,2, 0,0,1,0,0,0,0,0, 0,0, 0,0, 0,0, 0,0,0, 0,0,0];
    assert!(run_check(1, 0, 2, &prog, &data, &const_exp).is_ok());
    assert!(run_check(1, 1, 2, &prog, &data, &data_exp).is_ok());
    assert!(run_check(1, 0, 2, &prog, &data, &data_exp).is_err());
    assert!(run_check(1, 1, 2, &prog, &data, &const_exp).is_err());
}

#[test]
fn first_last_equal_beyond_and_padded_membership() {
    let prog = prog_with_constants(vec![
        ConstantSnapshot::U128 { marker_free: true, n: 10 },
        ConstantSnapshot::Bool { marker_free: true, b: true },
        ConstantSnapshot::Nil { marker_free: false },
    ]);
    let data = data_with(vec![
        DataSnapshot::U128 { n: 10 },
        DataSnapshot::Bool { b: false },
    ]);
    let first_c: Exp =
        [1,0,0, 1,0,0,0,0,0,0,0, 1,1, 1,10, 0,0, 0,0,0, 0,0,0];
    let last_c: Exp =
        [1,0,2, 0,0,1,0,0,0,0,0, 1,0, 0,0, 0,0, 0,0,0, 0,0,0];
    let last_d: Exp =
        [1,1,1, 0,1,0,0,0,0,0,0, 0,0, 0,0, 1,0, 0,0,0, 0,0,0];
    assert!(run_check(1, 0, 0, &prog, &data, &first_c).is_ok(), "first Const");
    assert!(run_check(1, 0, 2, &prog, &data, &last_c).is_ok(), "last Const");
    assert!(run_check(1, 1, 1, &prog, &data, &last_d).is_ok(), "last DATA");
    // Strict actual-count membership. caps().phys.max_constants is far
    // above 3 and DATA capacity is 9, above the two active rows here,
    // so capacity cannot impersonate membership. Expected lanes are
    // irrelevant on rejection, but a concrete non-coincidental claim
    // is supplied.
    assert!(run_check(1, 0, 3, &prog, &data, &last_c).is_err(), "Const equal-to-count");
    assert!(run_check(1, 0, 10, &prog, &data, &last_c).is_err(), "Const beyond count, within cap");
    assert!(run_check(1, 1, 2, &prog, &data, &last_d).is_err(), "DATA equal-to-count");
    assert!(run_check(1, 1, 3, &prog, &data, &last_d).is_err(), "DATA padded inactive slot");
}

#[test]
fn zero_constants_and_zero_active_data() {
    let prog = prog_with_constants(Vec::new());
    let data = data_with(Vec::new());
    assert!(run_check(1, 0, 0, &prog, &data, &ZERO).is_err(), "zero Const count");
    assert!(run_check(1, 1, 0, &prog, &data, &ZERO).is_err(), "zero DATA count");
    assert!(run_check(0, 0, 0, &prog, &data, &ZERO).is_ok(), "gate 0 Const zero tables");
    assert!(run_check(0, 1, 0, &prog, &data, &ZERO).is_ok(), "gate 0 DATA zero tables");
}

#[test]
fn shifted_prog_offset_and_unrelated_row_mutation() {
    // Structural codec fixture only; no admitted-owner-program claim.
    // The shifted program puts variable expressions and a nested,
    // multi-byte UTF-8 declaration template before the constants
    // stream. The constants stream therefore starts at a different
    // lane offset; the test never uses that offset as authority.
    let constants = vec![
        ConstantSnapshot::Bool { marker_free: true, b: false },
        ConstantSnapshot::U128 { marker_free: true, n: 4242 },
        ConstantSnapshot::Nil { marker_free: false },
        ConstantSnapshot::Constructor {
            marker_free: false,
            type_id: TypeIdC(1),
            ctor_id: CtorIdC(1),
            fields: cf(0, 1),
        },
    ];
    let mut base = empty_program();
    base.constants = constants.clone();
    base.names = vec!["a".to_string()];

    let mut shifted = empty_program();
    shifted.expressions = vec![
        ExpressionSnapshot::Literal { constant: 0 },
        ExpressionSnapshot::Var { binder: 0 },
        ExpressionSnapshot::Literal { constant: 1 },
    ];
    shifted.decl_param_counts = vec![1];
    shifted.decl_ctor_counts = vec![1];
    shifted.decl_templates = vec![TypeDeclSrc {
        name: "Boîte☃".to_string(),
        parameters: vec!["α".to_string()],
        ctors: vec![CtorDeclSrc {
            name: "MkBoîte".to_string(),
            fields: vec![CtorFieldSrc {
                name: "champ☃".to_string(),
                ty: TypeExprSrc::Apply {
                    name: "Boîte".to_string(),
                    args: vec![
                        TypeExprSrc::Param { name: "α".to_string() },
                        TypeExprSrc::UInt,
                    ],
                },
            }],
        }],
    }];
    shifted.constants = constants;
    shifted.names = vec!["héllo☃".to_string(), "seconde".to_string()];

    let data = data_with(vec![DataSnapshot::Nil]);
    let exp_u: Exp =
        [1,0,0, 1,0,0,0,0,0,0,0, 1,1, 1,4242, 0,0, 0,0,0, 0,0,0];
    let exp_ctor: Exp =
        [1,0,6, 0,0,0,0,0,0,1,0, 1,0, 0,0, 0,0, 1,1,1, 1,0,1];
    let base_lanes = prog_of(&base);
    let shifted_lanes = prog_of(&shifted);
    assert_ne!(base_lanes.len(), shifted_lanes.len(), "fixture really shifts the stream");
    for lanes in [&base_lanes, &shifted_lanes] {
        assert!(run_check(1, 0, 1, lanes, &data, &exp_u).is_ok(), "shifted U128");
        assert!(run_check(1, 0, 3, lanes, &data, &exp_ctor).is_ok(), "shifted Constructor");
    }
    // Unrelated-row mutation: change constant 0 only; selected rows
    // 1 and 3 keep the same literal projections.
    let mut mutated = shifted.clone();
    mutated.constants[0] = ConstantSnapshot::Bool { marker_free: false, b: true };
    let mutated_lanes = prog_of(&mutated);
    assert!(run_check(1, 0, 1, &mutated_lanes, &data, &exp_u).is_ok());
    assert!(run_check(1, 0, 3, &mutated_lanes, &data, &exp_ctor).is_ok());
}

#[test]
fn wrong_output_claims_const_and_data() {
    let prog = prog_with_constants(std_consts());
    let data = data_with(std_data());
    let good: Exp =
        [1,0,0, 1,0,0,0,0,0,0,0, 1,0, 1,u128::MAX, 0,0, 0,0,0, 0,0,0];
    assert!(run_check(1, 0, 1, &prog, &data, &good).is_ok(), "paired positive");
    // Each mutation changes exactly one claimed projection family.
    // Indices follow the Exp order at the top of this file.
    for (lane, value, label) in [
        (0usize, 0u128, "selected_found"),
        (1, 1, "is_data"),
        (2, 1, "kind"),
        (3, 0, "is_uint"),
        (11, 0, "marker_free_present"),
        (12, 1, "marker_free"),
        (13, 0, "uint_present"),
        (14, u128::MAX - 1, "uint_value"),
        (15, 1, "bool_present on UInt"),
        (16, 1, "bool_value on UInt"),
        (17, 1, "ctor_present on UInt"),
        (18, 1, "type_id on UInt"),
        (20, 1, "slice_present on UInt"),
        (21, 1, "slice_start on UInt"),
    ] {
        let mut bad = good;
        bad[lane] = value;
        assert!(run_check(1, 0, 1, &prog, &data, &bad).is_err(), "wrong claim {label}");
    }
    // DATA Constructor: wrong ctor id and slice claims reject, with a
    // paired positive using the same variable source operands.
    let good_d: Exp =
        [1,1,6, 0,0,0,0,0,0,1,0, 0,0, 0,0, 0,0, 1,1,1, 1,1,2];
    assert!(run_check(1, 1, 6, &prog, &data, &good_d).is_ok(), "DATA ctor positive");
    for (lane, value, label) in [
        (18usize, 2u128, "DATA type_id"),
        (19, 2, "DATA ctor_id"),
        (20, 0, "DATA slice_present"),
        (21, 0, "DATA slice_start"),
        (22, 1, "DATA slice_len"),
    ] {
        let mut bad = good_d;
        bad[lane] = value;
        assert!(run_check(1, 1, 6, &prog, &data, &bad).is_err(), "wrong DATA claim {label}");
    }
}

#[test]
fn bool_selected_payload_wrong_claims_const_and_data() {
    let prog = prog_with_constants(std_consts());
    let data = data_with(std_data());

    // Const Bool/false at index 2. Lane 16 is `bool_value` in the
    // declared Exp order; the paired positive is asserted first.
    let const_false: Exp =
        [1,0,1, 0,1,0,0,0,0,0,0, 1,1, 0,0, 1,0, 0,0,0, 0,0,0];
    assert!(run_check(1, 0, 2, &prog, &data, &const_false).is_ok(), "Const Bool false");
    let mut bad = const_false;
    bad[16] = 1;
    assert!(run_check(1, 0, 2, &prog, &data, &bad).is_err(), "Const Bool false claimed true");

    // Const Bool/true at index 3.
    let const_true: Exp =
        [1,0,1, 0,1,0,0,0,0,0,0, 1,0, 0,0, 1,1, 0,0,0, 0,0,0];
    assert!(run_check(1, 0, 3, &prog, &data, &const_true).is_ok(), "Const Bool true");
    let mut bad = const_true;
    bad[16] = 0;
    assert!(run_check(1, 0, 3, &prog, &data, &bad).is_err(), "Const Bool true claimed false");

    // DATA Bool/false at index 1, exercising the DATA storage path.
    let data_false: Exp =
        [1,1,1, 0,1,0,0,0,0,0,0, 0,0, 0,0, 1,0, 0,0,0, 0,0,0];
    assert!(run_check(1, 1, 1, &prog, &data, &data_false).is_ok(), "DATA Bool false");
    let mut bad = data_false;
    bad[16] = 1;
    assert!(run_check(1, 1, 1, &prog, &data, &bad).is_err(), "DATA Bool false claimed true");

    // DATA Bool/true at index 7.
    let data_true: Exp =
        [1,1,1, 0,1,0,0,0,0,0,0, 0,0, 0,0, 1,1, 0,0,0, 0,0,0];
    assert!(run_check(1, 1, 7, &prog, &data, &data_true).is_ok(), "DATA Bool true");
    let mut bad = data_true;
    bad[16] = 0;
    assert!(run_check(1, 1, 7, &prog, &data, &bad).is_err(), "DATA Bool true claimed false");
}

#[test]
fn tampered_payload_tag_index_and_active_count() {
    let data = data_with(std_data());
    let exp_u: Exp =
        [1,0,0, 1,0,0,0,0,0,0,0, 1,0, 1,u128::MAX, 0,0, 0,0,0, 0,0,0];
    let prog = prog_with_constants(std_consts());
    assert!(run_check(1, 0, 1, &prog, &data, &exp_u).is_ok(), "paired positive");
    // Selected Const payload tamper, rebuilt through the public codec
    // (no lane-offset authority is used).
    let mut tampered_consts = std_consts();
    tampered_consts[1] = ConstantSnapshot::U128 { marker_free: false, n: 7 };
    let tampered_prog = prog_with_constants(tampered_consts);
    assert!(run_check(1, 0, 1, &tampered_prog, &data, &exp_u).is_err(), "payload tamper");
    // Selected index tamper: same expected U128/MAX claim, index 0.
    assert!(run_check(1, 0, 0, &prog, &data, &exp_u).is_err(), "index tamper");
    // Selected DATA tag tamper at the same index.
    let exp_d: Exp =
        [1,1,0, 1,0,0,0,0,0,0,0, 0,0, 1,u128::MAX, 0,0, 0,0,0, 0,0,0];
    assert!(run_check(1, 1, 0, &prog, &data, &exp_d).is_ok(), "DATA paired positive");
    let mut tampered_rows = std_data();
    tampered_rows[0] = DataSnapshot::Bool { b: true };
    let tampered_data = data_with(tampered_rows);
    assert!(run_check(1, 1, 0, &prog, &tampered_data, &exp_d).is_err(), "DATA tag tamper");
    // Active-count tamper: keep only the first DATA row active through
    // the public fixture builder, then select the formerly active
    // second row. Membership, not a host expected-struct change, fails.
    let short_data = data_with(vec![DataSnapshot::U128 { n: u128::MAX }]);
    let exp_second: Exp =
        [1,1,1, 0,1,0,0,0,0,0,0, 0,0, 0,0, 1,0, 0,0,0, 0,0,0];
    assert!(run_check(1, 1, 1, &prog, &short_data, &exp_second).is_err(), "active-count tamper");
}

#[test]
fn raw_pow128_index_prog_payload_data_payload_preprocessing_only() {
    // Distinct raw paths: reference index, a Const U128 payload lane
    // reached through PROG, and a DATA U128 payload lane. Sentinel
    // values are located by value in this test fixture only; no lane
    // offset is used as producer/consumer authority.
    const SENTINEL: u128 = 0x1122_3344_5566_7788_99aa_bbcc_ddee_ff00;
    let prog = prog_with_constants(vec![
        ConstantSnapshot::U128 { marker_free: true, n: SENTINEL },
    ]);
    let data = data_with(vec![DataSnapshot::U128 { n: SENTINEL }]);
    let exp_c: Exp =
        [1,0,0, 1,0,0,0,0,0,0,0, 1,1, 1,SENTINEL, 0,0, 0,0,0, 0,0,0];
    let base = statement_u128(1, 0, 0, &prog, &data, &exp_c);
    assert!(run_check_fr(base.clone(), prog.len(), data.len()).is_ok(), "raw positive");

    let mut by_index = base.clone();
    by_index[2] = two_pow_128();
    assert!(run_check_fr(by_index, prog.len(), data.len()).is_err(), "index at 2^128");

    let prog_pos = prog.iter().position(|&v| v == SENTINEL).expect("sentinel in PROG");
    let mut by_prog = base.clone();
    by_prog[3 + prog_pos] = two_pow_128();
    assert!(run_check_fr(by_prog, prog.len(), data.len()).is_err(), "PROG payload at 2^128");

    let data_pos = data.iter().position(|&v| v == SENTINEL).expect("sentinel in DATA");
    let mut by_data = base.clone();
    by_data[3 + prog.len() + data_pos] = two_pow_128();
    assert!(run_check_fr(by_data, prog.len(), data.len()).is_err(), "DATA payload at 2^128");
    // No synthesis assertion is made for 2^128: actual SDK
    // decomposition may panic before constraints, as documented for
    // the DATA leaf. This family is preprocessing-only.
}

#[test]
fn gate_zero_disabled_malformed_gate_two_and_kind_two() {
    let mut prog = prog_with_constants(std_consts());
    let mut data = data_with(std_data());
    // Malformed full tables, still raw128-representable: wrong PROG
    // magic/version lanes and a noncanonical DATA tag/header lane.
    prog[0] = 42;
    prog[1] = 99;
    data[2] = 0;
    let g = c6_data_geometry(&caps()).expect("data geometry");
    data[g.data_base] = 7;
    assert!(run_check(0, 0, 0, &prog, &data, &ZERO).is_ok(), "gate 0 Const malformed");
    assert!(run_check(0, 1, 3, &prog, &data, &ZERO).is_ok(), "gate 0 DATA malformed");
    assert!(run_check(1, 0, 0, &prog, &data, &ZERO).is_err(), "gate 1 malformed paired");
    // Unconditional domains: isolate gate 2 and ref-kind 2 from a
    // valid gate-1 / Const statement. Only the intended raw lane is
    // mutated, so rejection is not an unrelated wrong-output claim.
    let good_prog = prog_with_constants(std_consts());
    let good_data = data_with(std_data());
    let valid: Exp =
        [1,0,0, 1,0,0,0,0,0,0,0, 1,1, 1,0, 0,0, 0,0,0, 0,0,0];
    assert!(
        run_check(1, 0, 0, &good_prog, &good_data, &valid).is_ok(),
        "paired gate-1 Const positive"
    );
    let mut gate2 = statement_u128(1, 0, 0, &good_prog, &good_data, &valid);
    gate2[0] = Fr::from(2u64);
    assert!(run_check_fr(gate2, good_prog.len(), good_data.len()).is_err(), "gate 2");
    let mut kind2 = statement_u128(1, 0, 0, &good_prog, &good_data, &valid);
    kind2[1] = Fr::from(2u64);
    assert!(run_check_fr(kind2, good_prog.len(), good_data.len()).is_err(), "ref kind 2");
}

fn var(id: String) -> Operand {
    Operand::Variable(Identifier(id))
}

fn inputs_valid() -> C6CheckedValueReadInputs {
    let prog_len = prog_with_constants(std_consts()).len();
    let data_len = data_with(std_data()).len();
    C6CheckedValueReadInputs {
        gate: var("%in.gate".to_string()),
        ref_kind_is_data: var("%in.kind".to_string()),
        ref_index: var("%in.index".to_string()),
        pre_prog: (0..prog_len).map(|i| var(format!("%in.prog.{i}"))).collect(),
        pre_data: (0..data_len).map(|i| var(format!("%in.data.{i}"))).collect(),
        caps: caps(),
    }
}

#[test]
fn reserved_operands_all_prefixes_all_groups() {
    // Actual composed prefixes for this reader: local (parent),
    // c6progctx (PROG capture), c6data (DATA canonical).
    for seg in [".local.temp.", ".c6progctx.temp.", ".c6data.temp."] {
        for group in 0..5usize {
            let mut inputs = inputs_valid();
            let bad = var(format!("%s{seg}0"));
            match group {
                0 => inputs.gate = bad,
                1 => inputs.ref_kind_is_data = bad,
                2 => inputs.ref_index = bad,
                3 => inputs.pre_prog[0] = bad,
                _ => inputs.pre_data[0] = bad,
            }
            assert!(
                matches!(
                    C6CheckedValueReadEmitter::new("s")
                        .expect("scope")
                        .emit(&inputs),
                    Err(C6CheckedValueReadError::ReservedOperand { .. })
                ),
                "reserved {seg} group {group}"
            );
        }
    }
}

#[test]
fn scope_refusals() {
    assert!(matches!(
        C6CheckedValueReadEmitter::new(""),
        Err(C6CheckedValueReadError::EmptyScope)
    ));
    assert!(matches!(
        C6CheckedValueReadEmitter::new("  "),
        Err(C6CheckedValueReadError::EmptyScope)
    ));
    for scope in [
        "bad%scope",
        "x.local.temp.1",
        "x.c6progctx.temp.1",
        "x.c6data.temp.1",
        ".local.temp.",
    ] {
        assert!(
            matches!(
                C6CheckedValueReadEmitter::new(scope),
                Err(C6CheckedValueReadError::InvalidScope)
            ),
            "scope {scope}"
        );
    }
}

fn temp_ids(instructions: &[Instruction], prefix: &str) -> BTreeSet<String> {
    let debug = format!("{:?}", instructions);
    let mut out = BTreeSet::new();
    let mut rest = debug.as_str();
    while let Some(pos) = rest.find(prefix) {
        let tail = &rest[pos..];
        let end = tail
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '%' || c == '.' || c == '_'))
            .unwrap_or(tail.len());
        out.insert(tail[..end].to_string());
        rest = &tail[end..];
    }
    out
}

fn max_serial(ids: &BTreeSet<String>) -> usize {
    ids.iter()
        .filter_map(|s| s.rsplit('.').next().and_then(|n| n.parse::<usize>().ok()))
        .max()
        .unwrap_or(0)
}

#[test]
fn retained_emitter_serial_advancement() {
    let inputs = inputs_valid();
    let mut emitter = C6CheckedValueReadEmitter::new("cvrcompose").expect("emitter");
    assert_eq!(emitter.scope(), "cvrcompose");
    let first = emitter.emit(&inputs).expect("first emit");
    let first_next = emitter.next_temp();
    let second = emitter.emit(&inputs).expect("second emit");
    assert!(emitter.next_temp() > first_next, "parent serial advances");
    for seg in ["local.temp.", "c6progctx.temp.", "c6data.temp."] {
        let prefix = format!("%cvrcompose.{seg}");
        let a = temp_ids(&first.instructions, &prefix);
        let b = temp_ids(&second.instructions, &prefix);
        assert!(!a.is_empty(), "first emit mints {prefix}");
        assert!(!b.is_empty(), "second emit mints {prefix}");
        assert!(a.is_disjoint(&b), "no redeclaration for {prefix}");
        assert!(max_serial(&b) > max_serial(&a), "serial advances for {prefix}");
    }
}

#[test]
fn free_function_and_scalar_operand_view_host_alias() {
    // Host structural alias check only; circuit semantics are covered
    // by the statement tests above. The free function is single-shot
    // per scope: this asserts its serial-zero form is emitted, not a
    // repeated-free-function rejection property.
    let prog = prog_with_constants(std_consts());
    let data = data_with(std_data());
    let imm = |v: u128| Operand::Immediate(Fr::from(v));
    let inputs = C6CheckedValueReadInputs {
        gate: imm(1),
        ref_kind_is_data: imm(0),
        ref_index: imm(1),
        pre_prog: prog.iter().map(|&v| imm(v)).collect(),
        pre_data: data.iter().map(|&v| imm(v)).collect(),
        caps: caps(),
    };
    let block = emit_c6_checked_value_read_block(&inputs, "cvrhost").expect("free emit");
    assert!(!block.instructions.is_empty());
    assert!(
        temp_ids(&block.instructions, "%cvrhost.local.temp.").contains("%cvrhost.local.temp.0"),
        "single-shot serial zero"
    );
    let view = scalar_operand_view(&block);
    assert_eq!(view.kind_code, block.kind_code);
    assert_eq!(view.is_marker, block.is_marker);
    assert_eq!(view.uint_present, block.uint_present);
    assert_eq!(view.uint_value, block.uint_value);
    assert_eq!(view.bool_present, block.bool_present);
    assert_eq!(view.bool_value, block.bool_value);
    assert_eq!(view.ctor_present, block.ctor_present);
    assert_eq!(view.type_id, block.type_id);
    assert_eq!(view.ctor_id, block.ctor_id);
    assert_eq!(view.slice_present, block.slice_present);
    assert_eq!(view.slice_start, block.slice_start);
    assert_eq!(view.slice_len, block.slice_len);
}

#[test]
fn mockprover_synthesis_positive_and_wrong_kind() {
    let prog = prog_with_constants(vec![
        ConstantSnapshot::U128 { marker_free: true, n: 42 },
    ]);
    let data = data_with(vec![DataSnapshot::U128 { n: 7 }]);
    let exp: Exp =
        [1,0,0, 1,0,0,0,0,0,0,0, 1,1, 1,42, 0,0, 0,0,0, 0,0,0];
    let statement = statement_u128(1, 0, 0, &prog, &data, &exp);
    assert!(
        run_synthesis_fr(statement.clone(), prog.len(), data.len()).is_ok(),
        "synthesis positive"
    );
    // Wrong kind claim at the explicit expected-output operand. All
    // source operands remain the genuine variable statement lanes;
    // rejection must be an unsatisfied-constraint outcome.
    let mut wrong = statement;
    let kind_pos = 3 + prog.len() + data.len() + OUT_KIND;
    wrong[kind_pos] = Fr::from(1u64);
    assert!(
        matches!(
            run_synthesis_fr(wrong, prog.len(), data.len()),
            Err(SynthesisRejection::ConstraintFailure(_))
        ),
        "wrong kind must be a constraint failure"
    );
    // A second representable wrong-claim negative on the UInt payload
    // lane, keeping the kind claim correct.
    let exp_bad: Exp =
        [1,0,0, 1,0,0,0,0,0,0,0, 1,1, 1,43, 0,0, 0,0,0, 0,0,0];
    let wrong_payload = statement_u128(1, 0, 0, &prog, &data, &exp_bad);
    let _ = OUT_UINT_VALUE;
    assert!(
        matches!(
            run_synthesis_fr(wrong_payload, prog.len(), data.len()),
            Err(SynthesisRejection::ConstraintFailure(_))
        ),
        "wrong UInt payload must be a constraint failure"
    );
}
