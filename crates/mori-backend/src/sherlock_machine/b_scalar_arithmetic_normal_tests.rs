//! UNRUN constructor, wiring, normal/defined-error, and negative-witness tests
//! for `b_scalar_arithmetic` (`b.helper.Scalar.arithmetic`, `RetRight/Add/Second -> Done`).
//!
//! These tests invoke the real `emit_b_scalar_arithmetic_add` and
//! `emit_public_b_scalar_arithmetic_add` constructors and evaluate/inspect the
//! emitted `midnight_zkir_v3::Instruction` stream directly. Symbolic/off-circuit
//! instruction evaluation here is distinct from real SDK `midnight_zk_stdlib`
//! Halo2 constraint compilation or proof generation, which is not run in this
//! source-authoring delivery.

use super::b_scalar_arithmetic::*;
use super::*;
use std::collections::BTreeMap;

fn var(name: &str) -> Operand {
    Operand::Variable(Identifier(name.into()))
}

fn var_u128_lanes(prefix: &str) -> BU128Lanes {
    std::array::from_fn(|i| var(&format!("{prefix}.{i}")))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExpectedStaged {
    U128(u128),
    PrimError(u128),
}

#[derive(Clone, Debug)]
struct LogicalAddFixture {
    case_id: &'static str,
    enable: u128,
    const_active_rows: u128,
    const_rows: Vec<(u128, u128)>,
    data_active_rows: u128,
    data_rows: Vec<(u128, u128)>,
    pre_present: u128,
    pre_kind: u128,
    pre_site: u128,
    pre_mode: u128,
    pre_phase: u128,
    pre_op_a_space: u128,
    pre_op_a_index: u128,
    pre_op_b_present: u128,
    pre_op_b_space: u128,
    pre_op_b_index: u128,
    pre_reg_a: u128,
    pre_rest: [u128; 5],
    post_present: u128,
    post_kind: u128,
    post_site: u128,
    post_mode: u128,
    post_phase: u128,
    post_op_a_space: u128,
    post_op_a_index: u128,
    post_op_b_present: u128,
    post_op_b_space: u128,
    post_op_b_index: u128,
    post_reg_a: u128,
    post_staged_tag: u128,
    post_staged_u128: u128,
    post_staged_bool: u128,
    post_staged_pending_node: u128,
    post_staged_error_code: u128,
    post_staged_selector: u128,
    post_staged_type_test: u128,
    post_rest: [u128; 5],
    pre_remaining_work: u128,
    pre_consumed_work: u128,
    pre_helper_work: u128,
    pre_helper_visits: u128,
    post_remaining_work: u128,
    post_consumed_work: u128,
    post_helper_work: u128,
    post_helper_visits: u128,
}

impl LogicalAddFixture {
    fn from_safe_case(
        case_id: &'static str,
        op_b_space: BValueSpace,
        op_b_index: u128,
        reg_a: u128,
        op_b_tag: u128,
        op_b_n: u128,
        expected: ExpectedStaged,
    ) -> Self {
        let const_rows = match op_b_space {
            BValueSpace::Const => vec![(VALUE_TAG_U128, reg_a), (op_b_tag, op_b_n)],
            BValueSpace::Data => vec![(VALUE_TAG_U128, reg_a)],
        };
        let data_rows = match op_b_space {
            BValueSpace::Const => vec![(VALUE_TAG_U128, 0)],
            BValueSpace::Data => vec![(op_b_tag, op_b_n)],
        };
        let op_b_space_bit = match op_b_space {
            BValueSpace::Const => 0,
            BValueSpace::Data => 1,
        };
        let (post_staged_tag, post_staged_u128, post_staged_error_code) = match expected {
            ExpectedStaged::U128(n) => (SCALAR_STAGED_TAG_U128_VALUE, n, 0),
            ExpectedStaged::PrimError(code) => (SCALAR_STAGED_TAG_PRIM_ERROR, 0, code),
        };
        // Canonical SAFE-INPUTS rest lanes:
        // [originReferenceCursor=8, parentResume.present=0, parentResume.value=0, expected.present=0, expected.value=0]
        let rest = [8, 0, 0, 0, 0];
        Self {
            case_id,
            enable: 1,
            const_active_rows: const_rows.len() as u128,
            const_rows,
            data_active_rows: 1,
            data_rows,
            pre_present: HELPER_PRESENT,
            pre_kind: HELPER_KIND_SCALAR,
            pre_site: SITE_RET_RIGHT,
            pre_mode: SCALAR_MODE_ADD,
            pre_phase: SCALAR_PHASE_SECOND,
            pre_op_a_space: 0,
            pre_op_a_index: 0,
            pre_op_b_present: OPT_PRESENT,
            pre_op_b_space: op_b_space_bit,
            pre_op_b_index: op_b_index,
            pre_reg_a: reg_a,
            pre_rest: rest,
            post_present: HELPER_PRESENT,
            post_kind: HELPER_KIND_SCALAR,
            post_site: SITE_RET_RIGHT,
            post_mode: SCALAR_MODE_ADD,
            post_phase: SCALAR_PHASE_DONE,
            post_op_a_space: 0,
            post_op_a_index: 0,
            post_op_b_present: OPT_PRESENT,
            post_op_b_space: op_b_space_bit,
            post_op_b_index: op_b_index,
            post_reg_a: reg_a,
            post_staged_tag,
            post_staged_u128,
            post_staged_bool: 0,
            post_staged_pending_node: 0,
            post_staged_error_code,
            post_staged_selector: 0,
            post_staged_type_test: 0,
            post_rest: rest,
            pre_remaining_work: 83,
            pre_consumed_work: 17,
            pre_helper_work: 9,
            pre_helper_visits: 4,
            post_remaining_work: 82,
            post_consumed_work: 18,
            post_helper_work: 10,
            post_helper_visits: 5,
        }
    }

    fn build_symbolic_inputs(&self) -> (BScalarArithmeticAddInputs, BTreeMap<String, u128>) {
        let env = std::cell::RefCell::new(BTreeMap::new());
        let bind = |name: String, val: u128| -> Operand {
            env.borrow_mut().insert(name.clone(), val);
            Operand::Variable(Identifier(name))
        };
        let bind_u128 = |prefix: &str, val: u128| -> BU128Lanes {
            let bytes = val.to_le_bytes();
            std::array::from_fn(|i| bind(format!("{prefix}.{i}"), u128::from(bytes[i])))
        };

        let enable = bind("%in.enable".into(), self.enable);
        let const_active = bind("%in.const.active".into(), self.const_active_rows);
        let const_rows = self
            .const_rows
            .iter()
            .enumerate()
            .map(|(r, (tag, n))| BValueRowLanes {
                tag: bind(format!("%in.const.row.{r}.tag"), *tag),
                n: bind_u128(&format!("%in.const.row.{r}.n"), *n),
            })
            .collect();
        let data_active = bind("%in.data.active".into(), self.data_active_rows);
        let data_rows = self
            .data_rows
            .iter()
            .enumerate()
            .map(|(r, (tag, n))| BValueRowLanes {
                tag: bind(format!("%in.data.row.{r}.tag"), *tag),
                n: bind_u128(&format!("%in.data.row.{r}.n"), *n),
            })
            .collect();

        let pre_rest_lanes = BScalarHelperRestLanes {
            origin_reference_cursor: bind("%in.pre.rest.0".into(), self.pre_rest[0]),
            parent_resume_present: bind("%in.pre.rest.1".into(), self.pre_rest[1]),
            parent_resume_value: bind("%in.pre.rest.2".into(), self.pre_rest[2]),
            expected_present: bind("%in.pre.rest.3".into(), self.pre_rest[3]),
            expected_value: bind("%in.pre.rest.4".into(), self.pre_rest[4]),
        };
        let post_rest_lanes = BScalarHelperRestLanes {
            origin_reference_cursor: bind("%in.post.rest.0".into(), self.post_rest[0]),
            parent_resume_present: bind("%in.post.rest.1".into(), self.post_rest[1]),
            parent_resume_value: bind("%in.post.rest.2".into(), self.post_rest[2]),
            expected_present: bind("%in.post.rest.3".into(), self.post_rest[3]),
            expected_value: bind("%in.post.rest.4".into(), self.post_rest[4]),
        };

        let pre_helper = BScalarArithmeticRegister {
            present: bind("%in.pre.present".into(), self.pre_present),
            kind: bind("%in.pre.kind".into(), self.pre_kind),
            site: bind("%in.pre.site".into(), self.pre_site),
            mode: bind("%in.pre.mode".into(), self.pre_mode),
            phase: bind("%in.pre.phase".into(), self.pre_phase),
            operand_a: BValueRef {
                space: bind("%in.pre.op_a.space".into(), self.pre_op_a_space),
                index: bind("%in.pre.op_a.index".into(), self.pre_op_a_index),
            },
            operand_b: BOptValueRef {
                present: bind("%in.pre.op_b.present".into(), self.pre_op_b_present),
                value: BValueRef {
                    space: bind("%in.pre.op_b.space".into(), self.pre_op_b_space),
                    index: bind("%in.pre.op_b.index".into(), self.pre_op_b_index),
                },
            },
            reg_a: bind_u128("%in.pre.reg_a", self.pre_reg_a),
            staged: BScalarStagedLanes {
                tag: bind("%in.pre.staged.tag".into(), SCALAR_STAGED_TAG_NONE),
                u128_value: bind_u128("%in.pre.staged.u128", 0),
                bool_value: bind("%in.pre.staged.bool".into(), 0),
                pending_node_tag: bind("%in.pre.staged.pending".into(), 0),
                prim_error_code: bind("%in.pre.staged.err".into(), 0),
                selector: bind("%in.pre.staged.sel".into(), 0),
                type_test_ok: bind("%in.pre.staged.ok".into(), 0),
            },
            rest: pre_rest_lanes.to_rest_vec(),
        };

        let post_helper = BScalarArithmeticRegister {
            present: bind("%in.post.present".into(), self.post_present),
            kind: bind("%in.post.kind".into(), self.post_kind),
            site: bind("%in.post.site".into(), self.post_site),
            mode: bind("%in.post.mode".into(), self.post_mode),
            phase: bind("%in.post.phase".into(), self.post_phase),
            operand_a: BValueRef {
                space: bind("%in.post.op_a.space".into(), self.post_op_a_space),
                index: bind("%in.post.op_a.index".into(), self.post_op_a_index),
            },
            operand_b: BOptValueRef {
                present: bind("%in.post.op_b.present".into(), self.post_op_b_present),
                value: BValueRef {
                    space: bind("%in.post.op_b.space".into(), self.post_op_b_space),
                    index: bind("%in.post.op_b.index".into(), self.post_op_b_index),
                },
            },
            reg_a: bind_u128("%in.post.reg_a", self.post_reg_a),
            staged: BScalarStagedLanes {
                tag: bind("%in.post.staged.tag".into(), self.post_staged_tag),
                u128_value: bind_u128("%in.post.staged.u128", self.post_staged_u128),
                bool_value: bind("%in.post.staged.bool".into(), self.post_staged_bool),
                pending_node_tag: bind(
                    "%in.post.staged.pending".into(),
                    self.post_staged_pending_node,
                ),
                prim_error_code: bind("%in.post.staged.err".into(), self.post_staged_error_code),
                selector: bind("%in.post.staged.sel".into(), self.post_staged_selector),
                type_test_ok: bind("%in.post.staged.ok".into(), self.post_staged_type_test),
            },
            rest: post_rest_lanes.to_rest_vec(),
        };

        let pre_counters = BChargedCounters {
            remaining_work: bind_u128("%in.pre.cnt.rem", self.pre_remaining_work),
            consumed_work: bind_u128("%in.pre.cnt.con", self.pre_consumed_work),
            helper_work: bind_u128("%in.pre.cnt.hw", self.pre_helper_work),
            helper_visits: bind_u128("%in.pre.cnt.hv", self.pre_helper_visits),
        };
        let post_counters = BChargedCounters {
            remaining_work: bind_u128("%in.post.cnt.rem", self.post_remaining_work),
            consumed_work: bind_u128("%in.post.cnt.con", self.post_consumed_work),
            helper_work: bind_u128("%in.post.cnt.hw", self.post_helper_work),
            helper_visits: bind_u128("%in.post.cnt.hv", self.post_helper_visits),
        };

        drop(bind_u128);
        drop(bind);
        (
            BScalarArithmeticAddInputs {
                enable,
                constants: BValueTableRead {
                    capacity: self.const_rows.len(),
                    active_rows: const_active,
                    rows: const_rows,
                },
                data: BValueTableRead {
                    capacity: self.data_rows.len(),
                    active_rows: data_active,
                    rows: data_rows,
                },
                pre_helper,
                post_helper,
                pre_counters,
                post_counters,
            },
            env.into_inner(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum EvalRejection {
    UnboundVariable(String),
    NonBoolean {
        index: usize,
        value: u128,
    },
    BitWidthExceeded {
        index: usize,
        value: u128,
        bits: u32,
    },
    ConstraintEqFailed {
        index: usize,
        left: u128,
        right: u128,
    },
    AssertFailed {
        index: usize,
        cond: u128,
    },
    UnsupportedInstruction {
        index: usize,
    },
}

fn eval_operand(op: &Operand, env: &BTreeMap<String, u128>) -> Result<u128, EvalRejection> {
    match op {
        Operand::Variable(id) => env
            .get(&id.0)
            .copied()
            .ok_or_else(|| EvalRejection::UnboundVariable(id.0.clone())),
        Operand::Immediate(fr) => {
            for candidate in [0u128, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 16, 255, 256] {
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

/// Step through the actual emitted `Instruction` stream, computing every wire
/// output via the IR semantics and checking every emitted constraint instruction.
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

fn safe_input_ten_cases() -> Vec<LogicalAddFixture> {
    vec![
        LogicalAddFixture::from_safe_case(
            "const-zero",
            BValueSpace::Const,
            1,
            0,
            VALUE_TAG_U128,
            0,
            ExpectedStaged::U128(0),
        ),
        LogicalAddFixture::from_safe_case(
            "const-small",
            BValueSpace::Const,
            1,
            7,
            VALUE_TAG_U128,
            9,
            ExpectedStaged::U128(16),
        ),
        LogicalAddFixture::from_safe_case(
            "data-byte-carry",
            BValueSpace::Data,
            0,
            65_535,
            VALUE_TAG_U128,
            1,
            ExpectedStaged::U128(65_536),
        ),
        LogicalAddFixture::from_safe_case(
            "const-half-width-carry",
            BValueSpace::Const,
            1,
            18_446_744_073_709_551_615,
            VALUE_TAG_U128,
            1,
            ExpectedStaged::U128(18_446_744_073_709_551_616),
        ),
        LogicalAddFixture::from_safe_case(
            "data-max-plus-zero",
            BValueSpace::Data,
            0,
            u128::MAX,
            VALUE_TAG_U128,
            0,
            ExpectedStaged::U128(u128::MAX),
        ),
        LogicalAddFixture::from_safe_case(
            "const-max-reached",
            BValueSpace::Const,
            1,
            u128::MAX - 1,
            VALUE_TAG_U128,
            1,
            ExpectedStaged::U128(u128::MAX),
        ),
        LogicalAddFixture::from_safe_case(
            "data-defined-overflow",
            BValueSpace::Data,
            0,
            u128::MAX,
            VALUE_TAG_U128,
            1,
            ExpectedStaged::PrimError(PRIM_ERROR_ARITH_OVERFLOW),
        ),
        LogicalAddFixture::from_safe_case(
            "const-defined-overflow",
            BValueSpace::Const,
            1,
            1u128 << 127,
            VALUE_TAG_U128,
            1u128 << 127,
            ExpectedStaged::PrimError(PRIM_ERROR_ARITH_OVERFLOW),
        ),
        LogicalAddFixture::from_safe_case(
            "const-defined-sort-error",
            BValueSpace::Const,
            1,
            7,
            VALUE_TAG_BOOL,
            0,
            ExpectedStaged::PrimError(PRIM_ERROR_SORT_MISMATCH),
        ),
        LogicalAddFixture::from_safe_case(
            "data-defined-sort-error",
            BValueSpace::Data,
            0,
            7,
            VALUE_TAG_NIL,
            0,
            ExpectedStaged::PrimError(PRIM_ERROR_SORT_MISMATCH),
        ),
    ]
}

#[test]
fn safe_inputs_all_ten_cases_accept_and_compute_exact_wires() {
    for case in safe_input_ten_cases() {
        let (inputs, env) = case.build_symbolic_inputs();
        let block = emit_b_scalar_arithmetic_add(&inputs, "arith").unwrap();
        assert_eq!(block.preserved, inputs, "case {}", case.case_id);
        assert!(
            !block.instructions.iter().any(|i| matches!(
                i,
                Instruction::PublicInput { .. } | Instruction::Output { .. }
            )),
            "fragment must not contain PublicInput or Output in case {}",
            case.case_id
        );
        let evaluated = eval_emitted_instructions(&block.instructions, &env)
            .unwrap_or_else(|e| panic!("case {} rejected: {e:?}", case.case_id));

        let expected_tag = match case.pre_op_b_space {
            0 => case.const_rows[case.pre_op_b_index as usize].0,
            _ => case.data_rows[case.pre_op_b_index as usize].0,
        };
        let expected_n = match case.pre_op_b_space {
            0 => case.const_rows[case.pre_op_b_index as usize].1,
            _ => case.data_rows[case.pre_op_b_index as usize].1,
        };
        assert_eq!(
            eval_operand(&block.read_tag, &evaluated).unwrap(),
            expected_tag,
            "case {}",
            case.case_id
        );
        assert_eq!(
            eval_operand(&block.b_is_uint, &evaluated).unwrap(),
            u128::from(expected_tag == VALUE_TAG_U128),
            "case {}",
            case.case_id
        );
        let mut read_bytes = [0u8; 16];
        for i in 0..16 {
            read_bytes[i] = eval_operand(&block.read_n[i], &evaluated).unwrap() as u8;
        }
        assert_eq!(
            u128::from_le_bytes(read_bytes),
            expected_n,
            "case {}",
            case.case_id
        );

        let expected_overflow = if expected_tag == VALUE_TAG_U128 {
            u128::from(case.pre_reg_a.checked_add(expected_n).is_none())
        } else {
            0
        };
        assert_eq!(
            eval_operand(&block.overflow, &evaluated).unwrap(),
            expected_overflow,
            "case {}",
            case.case_id
        );
    }
}

#[test]
fn carry_chain_wires_match_byte_and_half_width_boundaries() {
    // data-byte-carry: 65535 (0xff, 0xff, 0..) + 1 -> carries at byte 0 and byte 1 only.
    let case_byte = &safe_input_ten_cases()[2];
    let (inputs, env) = case_byte.build_symbolic_inputs();
    let block = emit_b_scalar_arithmetic_add(&inputs, "arith_carry").unwrap();
    let evaluated = eval_emitted_instructions(&block.instructions, &env).unwrap();
    for i in 0..16 {
        let expected_c = u128::from(i < 2);
        assert_eq!(
            eval_operand(&block.carry_chain[i], &evaluated).unwrap(),
            expected_c,
            "byte-carry index {i}"
        );
    }

    // const-half-width-carry: (2^64 - 1) + 1 -> carries at bytes 0..8, zero at bytes 8..16.
    let case_half = &safe_input_ten_cases()[3];
    let (inputs_h, env_h) = case_half.build_symbolic_inputs();
    let block_h = emit_b_scalar_arithmetic_add(&inputs_h, "arith_half").unwrap();
    let evaluated_h = eval_emitted_instructions(&block_h.instructions, &env_h).unwrap();
    for i in 0..16 {
        let expected_c = u128::from(i < 8);
        assert_eq!(
            eval_operand(&block_h.carry_chain[i], &evaluated_h).unwrap(),
            expected_c,
            "half-width carry index {i}"
        );
    }
}

#[test]
fn security_negative_witness_mutations_are_rejected_in_circuit() {
    let base_normal = &safe_input_ten_cases()[1]; // const-small: 7 + 9 = 16
    let (inputs, base_env) = base_normal.build_symbolic_inputs();
    let block = emit_b_scalar_arithmetic_add(&inputs, "arith_neg").unwrap();

    let assert_rejects_key = |env: &BTreeMap<String, u128>, key: &str, bad: u128, label: &str| {
        let mut mutated = env.clone();
        mutated.insert(key.into(), bad);
        assert!(
            eval_emitted_instructions(&block.instructions, &mutated).is_err(),
            "expected circuit rejection for {label} ({key} = {bad})"
        );
    };

    // 1. Pre-state branch guard violations
    assert_rejects_key(&base_env, "%in.pre.present", 0, "pre.present=0");
    assert_rejects_key(&base_env, "%in.pre.kind", 1, "pre.kind!=Scalar");
    assert_rejects_key(&base_env, "%in.pre.site", 12, "pre.site!=RetRight");
    assert_rejects_key(&base_env, "%in.pre.mode", 1, "pre.mode!=Add");
    assert_rejects_key(
        &base_env,
        "%in.pre.phase",
        SCALAR_PHASE_FIRST,
        "pre.phase!=Second",
    );
    assert_rejects_key(
        &base_env,
        "%in.pre.op_b.present",
        0,
        "pre.operandB.present=0",
    );
    assert_rejects_key(
        &base_env,
        "%in.pre.op_b.space",
        2,
        "non-boolean operandB space",
    );
    assert_rejects_key(&base_env, "%in.enable", 2, "non-boolean enable");

    // 2. Out-of-bounds fetch index & active_rows exceeding capacity
    assert_rejects_key(
        &base_env,
        "%in.pre.op_b.index",
        2,
        "operandB index == active_rows",
    );
    assert_rejects_key(
        &base_env,
        "%in.const.active",
        3,
        "const.active_rows > capacity",
    );

    // 3. Reserved / Marker tag on fetched operandB row
    assert_rejects_key(&base_env, "%in.const.row.1.tag", 6, "reserved ConstTag 6");
    assert_rejects_key(&base_env, "%in.const.row.1.tag", 7, "Marker ConstTag 7");

    // 4. Wrong post sum, wrong staged tag, wrong error code, non-zero inactive union columns
    assert_rejects_key(
        &base_env,
        "%in.post.staged.u128.0",
        15,
        "wrong sum low byte",
    );
    assert_rejects_key(
        &base_env,
        "%in.post.staged.u128.1",
        1,
        "spurious carry into byte 1",
    );
    assert_rejects_key(
        &base_env,
        "%in.post.staged.u128.0",
        272,
        "post sum byte >= 256",
    );
    assert_rejects_key(&base_env, "%in.pre.reg_a.0", 263, "pre.reg_a byte >= 256");
    assert_rejects_key(
        &base_env,
        "%in.post.staged.tag",
        SCALAR_STAGED_TAG_PRIM_ERROR,
        "forged PrimError tag on representable sum",
    );
    assert_rejects_key(
        &base_env,
        "%in.post.staged.err",
        PRIM_ERROR_ARITH_OVERFLOW,
        "non-zero error code on U128Value",
    );
    assert_rejects_key(
        &base_env,
        "%in.post.staged.bool",
        1,
        "inactive bool_value != 0",
    );
    assert_rejects_key(
        &base_env,
        "%in.post.staged.pending",
        1,
        "inactive pending_node_tag != 0",
    );
    assert_rejects_key(
        &base_env,
        "%in.post.staged.sel",
        1,
        "inactive selector != 0",
    );
    assert_rejects_key(
        &base_env,
        "%in.post.staged.ok",
        1,
        "inactive type_test_ok != 0",
    );

    // 5. Post helper phase and preservation violations
    assert_rejects_key(
        &base_env,
        "%in.post.phase",
        SCALAR_PHASE_SECOND,
        "post.phase!=Done",
    );
    assert_rejects_key(&base_env, "%in.post.present", 0, "post.present mutated");
    assert_rejects_key(&base_env, "%in.post.kind", 1, "post.kind mutated");
    assert_rejects_key(&base_env, "%in.post.site", 10, "post.site mutated");
    assert_rejects_key(&base_env, "%in.post.mode", 1, "post.mode mutated");
    assert_rejects_key(
        &base_env,
        "%in.post.op_a.space",
        1,
        "post.operand_a.space mutated",
    );
    assert_rejects_key(
        &base_env,
        "%in.post.op_a.index",
        1,
        "post.operand_a.index mutated",
    );
    assert_rejects_key(
        &base_env,
        "%in.post.op_b.present",
        0,
        "post.operand_b.present mutated",
    );
    assert_rejects_key(
        &base_env,
        "%in.post.op_b.space",
        1,
        "post.operand_b.space mutated",
    );
    assert_rejects_key(
        &base_env,
        "%in.post.op_b.index",
        0,
        "post.operand_b.index mutated",
    );
    assert_rejects_key(&base_env, "%in.post.reg_a.0", 8, "post.reg_a mutated");
    assert_rejects_key(
        &base_env,
        "%in.post.rest.0",
        9,
        "post.rest[0] originReferenceCursor mutated",
    );
    assert_rejects_key(
        &base_env,
        "%in.post.rest.4",
        1,
        "post.rest[4] expected.value mutated",
    );

    // 6. Charged counter delta violations
    assert_rejects_key(
        &base_env,
        "%in.post.cnt.rem.0",
        83,
        "remainingWork not decremented",
    );
    assert_rejects_key(
        &base_env,
        "%in.post.cnt.con.0",
        17,
        "consumedWork not incremented",
    );
    assert_rejects_key(
        &base_env,
        "%in.post.cnt.hw.0",
        9,
        "helperWork not incremented",
    );
    assert_rejects_key(
        &base_env,
        "%in.post.cnt.hv.0",
        4,
        "helperVisits not incremented",
    );
}

#[test]
fn security_negative_overflow_and_sort_mismatch_cannot_be_forged_or_leak_inactive_lanes() {
    // Overflow case: u128::MAX + 1 -> must stage PrimError(10) and zero u128_value.
    let ovf_case = &safe_input_ten_cases()[6];
    let (ovf_inputs, ovf_env) = ovf_case.build_symbolic_inputs();
    let ovf_block = emit_b_scalar_arithmetic_add(&ovf_inputs, "arith_ovf").unwrap();

    // Forging U128Value(0) on overflow must fail.
    let mut forged_val = ovf_env.clone();
    forged_val.insert("%in.post.staged.tag".into(), SCALAR_STAGED_TAG_U128_VALUE);
    forged_val.insert("%in.post.staged.err".into(), 0);
    assert!(eval_emitted_instructions(&ovf_block.instructions, &forged_val).is_err());

    // Wrong error code (11 instead of 10) on overflow must fail.
    let mut wrong_code = ovf_env.clone();
    wrong_code.insert("%in.post.staged.err".into(), PRIM_ERROR_SORT_MISMATCH);
    assert!(eval_emitted_instructions(&ovf_block.instructions, &wrong_code).is_err());

    // Non-zero byte in inactive u128_value on overflow must fail.
    let mut leaked_u128 = ovf_env.clone();
    leaked_u128.insert("%in.post.staged.u128.0".into(), 1);
    assert!(eval_emitted_instructions(&ovf_block.instructions, &leaked_u128).is_err());

    // Sort mismatch case: Const Bool row -> must stage PrimError(11) and zero u128_value.
    let sort_case = &safe_input_ten_cases()[8];
    let (sort_inputs, sort_env) = sort_case.build_symbolic_inputs();
    let sort_block = emit_b_scalar_arithmetic_add(&sort_inputs, "arith_sort").unwrap();

    // Wrong error code (10 instead of 11) on sort error must fail.
    let mut wrong_sort_code = sort_env.clone();
    wrong_sort_code.insert("%in.post.staged.err".into(), PRIM_ERROR_ARITH_OVERFLOW);
    assert!(eval_emitted_instructions(&sort_block.instructions, &wrong_sort_code).is_err());

    // Non-zero inactive U128.n byte on the fetched Bool row must fail (/notation/inactive).
    let mut dirty_bool_row = sort_env.clone();
    dirty_bool_row.insert("%in.const.row.1.n.0".into(), 1);
    assert!(eval_emitted_instructions(&sort_block.instructions, &dirty_bool_row).is_err());

    // Non-zero byte in post.staged.u128_value on sort error must fail.
    let mut dirty_post_u128 = sort_env;
    dirty_post_u128.insert("%in.post.staged.u128.0".into(), 7);
    assert!(eval_emitted_instructions(&sort_block.instructions, &dirty_post_u128).is_err());
}

#[test]
fn counter_underflow_and_overflow_boundaries_are_rejected() {
    let mut rem_zero = safe_input_ten_cases()[0].clone();
    rem_zero.pre_remaining_work = 0;
    rem_zero.post_remaining_work = u128::MAX;
    let (inputs, env) = rem_zero.build_symbolic_inputs();
    let block = emit_b_scalar_arithmetic_add(&inputs, "arith_rem_zero").unwrap();
    assert!(
        eval_emitted_instructions(&block.instructions, &env).is_err(),
        "remainingWork=0 decrement wrap must be rejected"
    );

    let mut consumed_max = safe_input_ten_cases()[0].clone();
    consumed_max.pre_consumed_work = u128::MAX;
    consumed_max.post_consumed_work = 0;
    let (inputs_c, env_c) = consumed_max.build_symbolic_inputs();
    let block_c = emit_b_scalar_arithmetic_add(&inputs_c, "arith_con_max").unwrap();
    assert!(
        eval_emitted_instructions(&block_c.instructions, &env_c).is_err(),
        "consumedWork=u128::MAX increment wrap must be rejected"
    );
}

#[test]
fn disabled_branch_accepts_divergent_pre_post_and_zero_capacity_unused_space() {
    let mut disabled = safe_input_ten_cases()[1].clone();
    disabled.enable = 0;
    disabled.pre_phase = SCALAR_PHASE_FIRST;
    disabled.pre_mode = 9;
    disabled.pre_op_b_present = 0;
    disabled.pre_op_b_space = 1;
    disabled.pre_op_b_index = 999; // out-of-range raw index ignored when enable = 0
    disabled.post_phase = SCALAR_PHASE_FIRST;
    disabled.post_staged_tag = SCALAR_STAGED_TAG_NONE;
    disabled.post_staged_u128 = 12345;
    disabled.post_remaining_work = 999;
    disabled.data_active_rows = 0;
    disabled.data_rows = vec![]; // zero-capacity Data table

    let (inputs, env) = disabled.build_symbolic_inputs();
    let block = emit_b_scalar_arithmetic_add(&inputs, "arith_disabled").unwrap();
    let evaluated = eval_emitted_instructions(&block.instructions, &env).unwrap();
    assert_eq!(eval_operand(&block.read_tag, &evaluated).unwrap(), 0);
    assert_eq!(eval_operand(&block.b_is_uint, &evaluated).unwrap(), 0);
    for i in 0..16 {
        assert_eq!(eval_operand(&block.read_n[i], &evaluated).unwrap(), 0);
    }

    // Enabling a read from the zero-capacity Data space must be rejected in-circuit.
    let mut enabled_zero_cap = env;
    enabled_zero_cap.insert("%in.enable".into(), 1);
    enabled_zero_cap.insert("%in.pre.phase".into(), SCALAR_PHASE_SECOND);
    enabled_zero_cap.insert("%in.pre.mode".into(), SCALAR_MODE_ADD);
    enabled_zero_cap.insert("%in.pre.op_b.present".into(), OPT_PRESENT);
    assert!(eval_emitted_instructions(&block.instructions, &enabled_zero_cap).is_err());
}

#[test]
fn constructor_rejects_invalid_scopes_shapes_and_identifier_collisions() {
    let (base, _) = safe_input_ten_cases()[0].build_symbolic_inputs();
    assert_eq!(
        emit_b_scalar_arithmetic_add(&base, ""),
        Err(BScalarArithmeticConstructionError::Fetch(
            ConstructionError::EmptyScope
        ))
    );

    let mut bad_const = base.clone();
    bad_const.constants.capacity = 3;
    assert_eq!(
        emit_b_scalar_arithmetic_add(&bad_const, "s"),
        Err(BScalarArithmeticConstructionError::RowCount {
            space: BValueSpace::Const
        })
    );

    let mut bad_data = base.clone();
    bad_data.data.capacity = 2;
    assert_eq!(
        emit_b_scalar_arithmetic_add(&bad_data, "s"),
        Err(BScalarArithmeticConstructionError::RowCount {
            space: BValueSpace::Data
        })
    );

    let mut bad_rest = base.clone();
    bad_rest.post_helper.rest.pop();
    assert_eq!(
        emit_b_scalar_arithmetic_add(&bad_rest, "s"),
        Err(BScalarArithmeticConstructionError::RestLaneCount)
    );

    let mut unprefixed = base.clone();
    unprefixed.post_helper.staged.prim_error_code = var("unprefixed");
    assert_eq!(
        emit_b_scalar_arithmetic_add(&unprefixed, "s"),
        Err(BScalarArithmeticConstructionError::Fetch(
            ConstructionError::UnprefixedVariable
        ))
    );

    for colliding in [
        "%s.local.temp.0",
        "%s.const_fetch.temp.0",
        "%s.data_fetch.temp.0",
    ] {
        let mut collision = base.clone();
        collision.pre_helper.operand_b.value.index = var(colliding);
        assert_eq!(
            emit_b_scalar_arithmetic_add(&collision, "s"),
            Err(BScalarArithmeticConstructionError::Fetch(
                ConstructionError::TemporaryNameCollision
            ))
        );
    }
}

#[test]
fn public_wrapper_emits_declared_io_and_packs_witness() {
    let emitted = emit_public_b_scalar_arithmetic_add(2, 1, 5).unwrap();
    let expected_inputs = 1 + 1 + 2 * 17 + 1 + 17 + 2 * (48 + 5) + 2 * 64;
    assert_eq!(emitted.input_fields.len(), expected_inputs);
    assert_eq!(emitted.output_fields.len(), 19 + expected_inputs);
    assert_eq!(emitted.ir.outputs.len(), emitted.output_fields.len());
    assert!(matches!(
        emitted.ir.instructions.last(),
        Some(Instruction::Output { vals }) if vals.len() == emitted.output_fields.len()
    ));

    let w = BScalarArithmeticWitness {
        enable: true,
        const_active_rows: 2,
        const_rows: vec![(0, 7u128.to_le_bytes()), (0, 9u128.to_le_bytes())],
        data_active_rows: 1,
        data_rows: vec![(0, 0u128.to_le_bytes())],
        pre_helper_lanes: vec![0; 53],
        post_helper_lanes: vec![0; 53],
        pre_counters: [83, 17, 9, 4],
        post_counters: [82, 18, 10, 5],
    };
    let packed = emitted.pack_witness(&w).unwrap();
    assert_eq!(packed.len(), expected_inputs);
    assert_eq!(u128_to_le_immediates(258)[0], immediate(2));
    assert_eq!(u128_to_le_immediates(258)[1], immediate(1));
    assert_eq!(var_u128_lanes("%x").len(), 16);
}
