//! Three focused tests for `emit_b_scalar_arithmetic_sub`
//! (`RetRight / Sub / Second -> Done`): an in-range `UInt128` difference with
//! real byte borrows, a `UInt128` underflow staging `PrimError(10)`, and a valid
//! `Bool` operandB staging `PrimError(11)`.
//!
//! Each test calls the actual emitter, evaluates its emitted instructions with a
//! local host interpreter, asserts acceptance plus the extracted block and post
//! lanes, and checks that one wrong-outcome witness fails at the targeted
//! `ConstrainEq` with exact operand values. The interpreter is not SDK, Halo2 or
//! proof execution: values are host `u128` integers with no `Fr` modular wrap
//! (`Add`/`Mul` panic on host overflow), `ConstrainBits`/`LessThan` with
//! `bits >= 128` are unchecked, immediates decode only in `0..=1024`, and other
//! instructions are rejected. Expected values are authored, not independent goldens.

use super::*;
use midnight_transient_crypto::curve::Fr;
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction};
use std::collections::BTreeMap;

const SCOPE: &str = "sub_case";
const CONST_CAPACITY: usize = 3;
const DATA_CAPACITY: usize = 1;
const CONST_ROW_LEFT: u128 = 0;
const CONST_ROW_RIGHT: u128 = 1;
const CONST_ROW_BOOL: u128 = 2;
const SPACE_CONST: u128 = 0; // records.ValueRef space bit 0 = Const
const ORIGIN_REFERENCE_CURSOR: u128 = 7;
// [remainingWork, consumedWork, helperWork, helperVisits]; the first three cross a byte.
const PRE_COUNTERS: [u128; 4] = [0x100, 0xFF, 0x1FF, 0];
const POST_COUNTERS: [u128; 4] = [0xFF, 0x100, 0x200, 1];

fn var(name: &str) -> Operand {
    Operand::Variable(Identifier(name.into()))
}

fn var_u128_lanes(prefix: &str) -> BU128Lanes {
    std::array::from_fn(|i| var(&format!("{prefix}.{i}")))
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum EvalRejection {
    UnboundVariable(String),
    NonBoolean { index: usize, value: u128 },
    BitWidthExceeded { index: usize, value: u128, bits: u32 },
    ConstraintEqFailed { index: usize, left: u128, right: u128 },
    AssertFailed { index: usize, cond: u128 },
    UnsupportedInstruction { index: usize },
}

fn eval_operand(
    op: &Operand,
    env: &BTreeMap<String, u128>,
) -> Result<u128, EvalRejection> {
    match op {
        Operand::Variable(id) => env
            .get(&id.0)
            .copied()
            .ok_or_else(|| EvalRejection::UnboundVariable(id.0.clone())),
        Operand::Immediate(fr) => {
            for candidate in [
                0u128, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 16, 255, 256,
            ] {
                if *fr == Fr::from(candidate) {
                    return Ok(candidate);
                }
            }
            for candidate in 0u128..=1024 {
                if *fr == Fr::from(candidate) {
                    return Ok(candidate);
                }
            }
            Err(EvalRejection::UnboundVariable(format!("immediate:{fr:?}")))
        }
    }
}

fn eval_emitted_instructions(
    instructions: &[Instruction],
    initial_env: &BTreeMap<String, u128>,
) -> Result<BTreeMap<String, u128>, EvalRejection> {
    let mut env = initial_env.clone();
    for (index, inst) in instructions.iter().enumerate() {
        match inst {
            Instruction::CondSelect { bit, a, b, output } => {
                let cond = eval_operand(bit, &env)?;
                if cond > 1 {
                    return Err(EvalRejection::NonBoolean { index, value: cond });
                }
                let va = eval_operand(a, &env)?;
                let vb = eval_operand(b, &env)?;
                env.insert(output.0.clone(), if cond == 1 { va } else { vb });
            }
            Instruction::ConstrainToBoolean { val } => {
                let v = eval_operand(val, &env)?;
                if v > 1 {
                    return Err(EvalRejection::NonBoolean { index, value: v });
                }
            }
            Instruction::ConstrainBits { val, bits } => {
                let v = eval_operand(val, &env)?;
                if *bits < 128 && (v >> *bits) != 0 {
                    return Err(EvalRejection::BitWidthExceeded {
                        index,
                        value: v,
                        bits: *bits,
                    });
                }
            }
            Instruction::ConstrainEq { a, b } => {
                let va = eval_operand(a, &env)?;
                let vb = eval_operand(b, &env)?;
                if va != vb {
                    return Err(EvalRejection::ConstraintEqFailed {
                        index,
                        left: va,
                        right: vb,
                    });
                }
            }
            Instruction::Assert { cond } => {
                let c = eval_operand(cond, &env)?;
                if c != 1 {
                    return Err(EvalRejection::AssertFailed { index, cond: c });
                }
            }
            Instruction::TestEq { a, b, output } => {
                let va = eval_operand(a, &env)?;
                let vb = eval_operand(b, &env)?;
                env.insert(output.0.clone(), u128::from(va == vb));
            }
            Instruction::LessThan { a, b, bits, output } => {
                let va = eval_operand(a, &env)?;
                let vb = eval_operand(b, &env)?;
                if *bits < 128 && ((va >> *bits) != 0 || (vb >> *bits) != 0) {
                    return Err(EvalRejection::BitWidthExceeded {
                        index,
                        value: va.max(vb),
                        bits: *bits,
                    });
                }
                env.insert(output.0.clone(), u128::from(va < vb));
            }
            Instruction::Add { a, b, output } => {
                let va = eval_operand(a, &env)?;
                let vb = eval_operand(b, &env)?;
                env.insert(output.0.clone(), va.checked_add(vb).unwrap());
            }
            Instruction::Mul { a, b, output } => {
                let va = eval_operand(a, &env)?;
                let vb = eval_operand(b, &env)?;
                env.insert(output.0.clone(), va.checked_mul(vb).unwrap());
            }
            _ => return Err(EvalRejection::UnsupportedInstruction { index }),
        }
    }
    Ok(env)
}

fn table(prefix: &str, capacity: usize) -> BValueTableRead {
    BValueTableRead {
        capacity,
        active_rows: var(&format!("{prefix}.active_rows")),
        rows: (0..capacity)
            .map(|row| BValueRowLanes {
                tag: var(&format!("{prefix}.{row}.tag")),
                n: var_u128_lanes(&format!("{prefix}.{row}.n")),
            })
            .collect(),
    }
}

fn register(prefix: &str) -> BScalarArithmeticRegister {
    let v = |field: &str| var(&format!("{prefix}.{field}"));
    BScalarArithmeticRegister {
        present: v("present"),
        kind: v("kind"),
        site: v("site"),
        mode: v("mode"),
        phase: v("phase"),
        operand_a: BValueRef { space: v("a_space"), index: v("a_index") },
        operand_b: BOptValueRef {
            present: v("b_present"),
            value: BValueRef { space: v("b_space"), index: v("b_index") },
        },
        reg_a: var_u128_lanes(&format!("{prefix}.reg_a")),
        staged: BScalarStagedLanes {
            tag: v("staged.tag"),
            u128_value: var_u128_lanes(&format!("{prefix}.staged.u128")),
            bool_value: v("staged.bool"),
            pending_node_tag: v("staged.pending"),
            prim_error_code: v("staged.code"),
            selector: v("staged.selector"),
            type_test_ok: v("staged.type_test"),
        },
        rest: BScalarHelperRestLanes {
            origin_reference_cursor: v("rest.origin_reference_cursor"),
            parent_resume_present: v("rest.parent_resume_present"),
            parent_resume_value: v("rest.parent_resume_value"),
            expected_present: v("rest.expected_present"),
            expected_value: v("rest.expected_value"),
        }
        .to_rest_vec(),
    }
}

fn counters(prefix: &str) -> BChargedCounters {
    BChargedCounters {
        remaining_work: var_u128_lanes(&format!("{prefix}.remaining_work")),
        consumed_work: var_u128_lanes(&format!("{prefix}.consumed_work")),
        helper_work: var_u128_lanes(&format!("{prefix}.helper_work")),
        helper_visits: var_u128_lanes(&format!("{prefix}.helper_visits")),
    }
}

fn sub_inputs() -> BScalarArithmeticSubInputs {
    BScalarArithmeticSubInputs {
        enable: var("%in.enable"),
        constants: table("%in.const", CONST_CAPACITY),
        data: table("%in.data", DATA_CAPACITY),
        pre_helper: register("%in.pre"),
        post_helper: register("%in.post"),
        pre_counters: counters("%in.pre_counters"),
        post_counters: counters("%in.post_counters"),
    }
}

fn bind_u128(env: &mut BTreeMap<String, u128>, prefix: &str, n: u128) {
    for (i, byte) in n.to_le_bytes().into_iter().enumerate() {
        env.insert(format!("{prefix}.{i}"), u128::from(byte));
    }
}

/// Varying helper lanes; every other lane is fixed and identical pre and post.
struct HelperImage {
    phase: u128,
    reg_a: u128,
    b_index: u128,
    staged_tag: u128,
    staged_u128: u128,
    staged_code: u128,
}

fn bind_helper(env: &mut BTreeMap<String, u128>, prefix: &str, h: &HelperImage) {
    for (field, value) in [
        ("present", HELPER_PRESENT),
        ("kind", HELPER_KIND_SCALAR),
        ("site", SITE_RET_RIGHT),
        ("mode", SCALAR_MODE_SUB),
        ("phase", h.phase),
        ("a_space", SPACE_CONST),
        ("a_index", CONST_ROW_LEFT),
        ("b_present", OPT_PRESENT),
        ("b_space", SPACE_CONST),
        ("b_index", h.b_index),
        ("staged.tag", h.staged_tag),
        ("staged.bool", 0),
        ("staged.pending", 0),
        ("staged.code", h.staged_code),
        ("staged.selector", 0),
        ("staged.type_test", 0),
        ("rest.origin_reference_cursor", ORIGIN_REFERENCE_CURSOR),
        ("rest.parent_resume_present", OPT_ABSENT),
        ("rest.parent_resume_value", 0),
        ("rest.expected_present", OPT_ABSENT),
        ("rest.expected_value", 0),
    ] {
        env.insert(format!("{prefix}.{field}"), value);
    }
    bind_u128(env, &format!("{prefix}.reg_a"), h.reg_a);
    bind_u128(env, &format!("{prefix}.staged.u128"), h.staged_u128);
}

fn bind_counters(env: &mut BTreeMap<String, u128>, prefix: &str, values: [u128; 4]) {
    let names = ["remaining_work", "consumed_work", "helper_work", "helper_visits"];
    for (name, value) in names.into_iter().zip(values) {
        bind_u128(env, &format!("{prefix}.{name}"), value);
    }
}

/// Const rows: 0 = U128 left (operandA, so pre.regA = u128(operandA) per S2),
/// 1 = U128 right, 2 = Bool with zero U128 lanes. One U128 Data row stays disabled.
struct SubCase {
    left: u128,
    right: u128,
    b_index: u128,
    staged_tag: u128,
    staged_u128: u128,
    staged_code: u128,
}

fn case_env(c: &SubCase) -> BTreeMap<String, u128> {
    let mut env: BTreeMap<String, u128> = BTreeMap::new();
    env.insert("%in.enable".into(), 1);
    env.insert("%in.const.active_rows".into(), CONST_CAPACITY as u128);
    let const_rows = [(VALUE_TAG_U128, c.left), (VALUE_TAG_U128, c.right), (VALUE_TAG_BOOL, 0)];
    for (row, (tag, n)) in const_rows.into_iter().enumerate() {
        env.insert(format!("%in.const.{row}.tag"), tag);
        bind_u128(&mut env, &format!("%in.const.{row}.n"), n);
    }
    env.insert("%in.data.active_rows".into(), DATA_CAPACITY as u128);
    env.insert("%in.data.0.tag".into(), VALUE_TAG_U128);
    bind_u128(&mut env, "%in.data.0.n", 99);
    let pre = HelperImage {
        phase: SCALAR_PHASE_SECOND,
        reg_a: c.left,
        b_index: c.b_index,
        staged_tag: SCALAR_STAGED_TAG_NONE,
        staged_u128: 0,
        staged_code: 0,
    };
    let post = HelperImage {
        phase: SCALAR_PHASE_DONE,
        reg_a: c.left,
        b_index: c.b_index,
        staged_tag: c.staged_tag,
        staged_u128: c.staged_u128,
        staged_code: c.staged_code,
    };
    bind_helper(&mut env, "%in.pre", &pre);
    bind_helper(&mut env, "%in.post", &post);
    bind_counters(&mut env, "%in.pre_counters", PRE_COUNTERS);
    bind_counters(&mut env, "%in.post_counters", POST_COUNTERS);
    env
}

fn emit_and_eval(
    env: &BTreeMap<String, u128>,
) -> (BScalarArithmeticSubBlock, Result<BTreeMap<String, u128>, EvalRejection>) {
    let block = emit_b_scalar_arithmetic_sub(&sub_inputs(), SCOPE).expect("well-formed fixture");
    let result = eval_emitted_instructions(&block.instructions, env);
    (block, result)
}

fn read(op: &Operand, env: &BTreeMap<String, u128>) -> u128 {
    eval_operand(op, env).expect("bound operand")
}

fn read_lanes(lanes: &BU128Lanes, env: &BTreeMap<String, u128>) -> u128 {
    let mut bytes = [0u8; 16];
    for (i, lane) in lanes.iter().enumerate() {
        bytes[i] = u8::try_from(read(lane, env)).expect("byte lane");
    }
    u128::from_le_bytes(bytes)
}

fn borrow_chain(block: &BScalarArithmeticSubBlock, env: &BTreeMap<String, u128>) -> [u128; 16] {
    std::array::from_fn(|i| read(&block.borrow_chain[i], env))
}

fn assert_done_preserving(block: &BScalarArithmeticSubBlock, env: &BTreeMap<String, u128>, left: u128) {
    let post = &block.preserved.post_helper;
    assert_eq!(read(&post.phase, env), SCALAR_PHASE_DONE);
    assert_eq!(read(&post.mode, env), SCALAR_MODE_SUB);
    assert_eq!(read_lanes(&post.reg_a, env), left);
    for inactive in [
        &post.staged.bool_value,
        &post.staged.pending_node_tag,
        &post.staged.selector,
        &post.staged.type_test_ok,
    ] {
        assert_eq!(read(inactive, env), 0);
    }
    let c = &block.preserved.post_counters;
    let got = [&c.remaining_work, &c.consumed_work, &c.helper_work, &c.helper_visits]
        .map(|lanes| read_lanes(lanes, env));
    assert_eq!(got, POST_COUNTERS);
}

/// The single mutated lane must make the evaluator's first rejecting relation
/// a `ConstrainEq` with exactly these operand values. Earlier relations may
/// read that lane (e.g. the `eff_bytes` 8-bit range on `staged.u128.0`), but
/// none of them rejects the mutated value.
fn assert_rejected_at_eq(env: &BTreeMap<String, u128>, expected: (u128, u128)) {
    let (block, result) = emit_and_eval(env);
    match result {
        Err(EvalRejection::ConstraintEqFailed { index, left, right }) => {
            assert!(matches!(block.instructions[index], Instruction::ConstrainEq { .. }));
            assert_eq!((left, right), expected);
        }
        other => panic!("expected ConstrainEq rejection, got {other:?}"),
    }
}

#[test]
fn sub_uint128_in_range_difference_borrows_across_low_bytes() {
    // a = 2^64 + 3, b = 0x0105: bytes 0..=7 borrow and byte 8 absorbs it.
    const LEFT: u128 = (1 << 64) + 3;
    const RIGHT: u128 = 0x0105;
    const DIFF: u128 = 0xFFFF_FFFF_FFFF_FEFE;
    assert_eq!(LEFT.checked_sub(RIGHT), Some(DIFF), "authored fixture self-check");
    let env = case_env(&SubCase {
        left: LEFT,
        right: RIGHT,
        b_index: CONST_ROW_RIGHT,
        staged_tag: SCALAR_STAGED_TAG_U128_VALUE,
        staged_u128: DIFF,
        staged_code: 0,
    });
    let (block, result) = emit_and_eval(&env);
    let out = result.expect("in-range Sub witness accepted");
    assert_eq!(read(&block.read_tag, &out), VALUE_TAG_U128);
    assert_eq!(read_lanes(&block.read_n, &out), RIGHT);
    assert_eq!(read(&block.b_is_uint, &out), 1);
    assert_eq!(read(&block.underflow, &out), 0);
    assert_eq!(borrow_chain(&block, &out), [1u128, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]);
    let staged = &block.preserved.post_helper.staged;
    assert_eq!(read(&staged.tag, &out), SCALAR_STAGED_TAG_U128_VALUE);
    assert_eq!(read_lanes(&staged.u128_value, &out), DIFF);
    assert_eq!(read(&staged.prim_error_code, &out), 0);
    assert_done_preserving(&block, &out, LEFT);

    // Byte 0 staged as 253, not 254: post + need = 253 + 5 must equal 3 + 256.
    let mut wrong = env.clone();
    wrong.insert("%in.post.staged.u128.0".into(), 253);
    assert_rejected_at_eq(&wrong, (258, 259));
}

#[test]
fn sub_uint128_underflow_stages_prim_error_10_with_zero_payload() {
    // a = 0x100, b = 2^120 + 1: byte 0 borrows, byte 1 absorbs it, byte 15 underflows.
    const LEFT: u128 = 0x100;
    const RIGHT: u128 = (1 << 120) + 1;
    assert_eq!(LEFT.checked_sub(RIGHT), None, "authored fixture self-check");
    let env = case_env(&SubCase {
        left: LEFT,
        right: RIGHT,
        b_index: CONST_ROW_RIGHT,
        staged_tag: SCALAR_STAGED_TAG_PRIM_ERROR,
        staged_u128: 0,
        staged_code: PRIM_ERROR_ARITH_OVERFLOW,
    });
    let (block, result) = emit_and_eval(&env);
    let out = result.expect("underflow Sub witness accepted");
    assert_eq!(read(&block.read_tag, &out), VALUE_TAG_U128);
    assert_eq!(read_lanes(&block.read_n, &out), RIGHT);
    assert_eq!(read(&block.b_is_uint, &out), 1);
    assert_eq!(read(&block.underflow, &out), 1);
    assert_eq!(borrow_chain(&block, &out), [1u128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    let staged = &block.preserved.post_helper.staged;
    assert_eq!(read(&staged.tag, &out), SCALAR_STAGED_TAG_PRIM_ERROR);
    assert_eq!(read_lanes(&staged.u128_value, &out), 0);
    assert_eq!(read(&staged.prim_error_code, &out), 10);
    assert_done_preserving(&block, &out, LEFT);

    // Sort-mismatch code on a UInt underflow: staged 11 against selected 10.
    let mut wrong = env.clone();
    wrong.insert("%in.post.staged.code".into(), PRIM_ERROR_SORT_MISMATCH);
    assert_rejected_at_eq(&wrong, (11, 10));
}

#[test]
fn sub_bool_operand_b_stages_prim_error_11_with_zero_payload() {
    const LEFT: u128 = 42;
    let env = case_env(&SubCase {
        left: LEFT,
        right: 1,
        b_index: CONST_ROW_BOOL,
        staged_tag: SCALAR_STAGED_TAG_PRIM_ERROR,
        staged_u128: 0,
        staged_code: PRIM_ERROR_SORT_MISMATCH,
    });
    let (block, result) = emit_and_eval(&env);
    let out = result.expect("Bool operandB Sub witness accepted");
    assert_eq!(read(&block.read_tag, &out), VALUE_TAG_BOOL);
    assert_eq!(read_lanes(&block.read_n, &out), 0);
    assert_eq!(read(&block.b_is_uint, &out), 0);
    assert_eq!(read(&block.underflow, &out), 0);
    assert_eq!(borrow_chain(&block, &out), [0u128; 16]);
    let staged = &block.preserved.post_helper.staged;
    assert_eq!(read(&staged.tag, &out), SCALAR_STAGED_TAG_PRIM_ERROR);
    assert_eq!(read_lanes(&staged.u128_value, &out), 0);
    assert_eq!(read(&staged.prim_error_code, &out), 11);
    assert_done_preserving(&block, &out, LEFT);

    // Overflow code on a sort mismatch: staged 10 against selected 11.
    let mut wrong = env.clone();
    wrong.insert("%in.post.staged.code".into(), PRIM_ERROR_ARITH_OVERFLOW);
    assert_rejected_at_eq(&wrong, (10, 11));
}
