//! UNRUN constructor, wiring, normal/cap-refusal/error-transfer, and
//! security-negative witness tests for `b_scalar_completion`
//! (`b.helper.Scalar.exit` and `b.helper.Scalar.failure` on `RetRight / Add / Done`).
//!
//! These tests invoke the real `emit_b_scalar_exit_add_u128` and
//! `emit_b_scalar_failure_add` constructors and evaluate/inspect the emitted
//! `midnight_zkir_v3::Instruction` stream directly. Symbolic/off-circuit
//! instruction evaluation here is distinct from real SDK `midnight_zk_stdlib`
//! Halo2 constraint compilation or proof generation, which is not run in this
//! source-authoring delivery.

use super::b_scalar_arithmetic::*;
use super::b_scalar_completion::*;
use super::*;
use std::collections::BTreeMap;

fn var(name: &str) -> Operand {
    Operand::Variable(Identifier(name.into()))
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RawDataSlot {
    tag: u128,
    n: u128,
    bool_b: u128,
    fields_start: u128,
    fields_len: u128,
    rest: [u128; 2],
}

impl RawDataSlot {
    fn zero() -> Self {
        Self {
            tag: 0,
            n: 0,
            bool_b: 0,
            fields_start: 0,
            fields_len: 0,
            rest: [0, 0],
        }
    }

    fn u128_row(n: u128) -> Self {
        Self {
            tag: DATA_TAG_U128,
            n,
            bool_b: 0,
            fields_start: 0,
            fields_len: 0,
            rest: [0, 0],
        }
    }
}

#[derive(Clone, Debug)]
struct ExitAddU128Fixture {
    case_id: &'static str,
    enable: u128,
    bounds_data_nodes: u128,
    pre_active_data: u128,
    post_active_data: u128,
    pre_data_rows: Vec<RawDataSlot>,
    post_data_rows: Vec<RawDataSlot>,
    pre_reg_a: u128,
    staged_u128: u128,
    applied: bool,
    pre_alloc_data_nodes: u128,
    post_alloc_data_nodes: u128,
    pre_remaining_work: u128,
    pre_consumed_work: u128,
    pre_helper_work: u128,
    pre_helper_visits: u128,
    post_remaining_work: u128,
    post_consumed_work: u128,
    post_helper_work: u128,
    post_helper_visits: u128,
}

impl ExitAddU128Fixture {
    fn new_applied(
        case_id: &'static str,
        capacity: usize,
        prefix_rows: Vec<RawDataSlot>,
        pre_alloc_data_nodes: u128,
        bounds_data_nodes: u128,
        pre_reg_a: u128,
        staged_u128: u128,
    ) -> Self {
        let pre_active = prefix_rows.len();
        assert!(pre_active < capacity);
        assert!(pre_alloc_data_nodes < bounds_data_nodes);
        let mut pre_data_rows = prefix_rows.clone();
        pre_data_rows.resize(capacity, RawDataSlot::zero());
        let mut post_data_rows = pre_data_rows.clone();
        post_data_rows[pre_active] = RawDataSlot::u128_row(staged_u128);
        Self {
            case_id,
            enable: 1,
            bounds_data_nodes,
            pre_active_data: pre_active as u128,
            post_active_data: (pre_active + 1) as u128,
            pre_data_rows,
            post_data_rows,
            pre_reg_a,
            staged_u128,
            applied: true,
            pre_alloc_data_nodes,
            post_alloc_data_nodes: pre_alloc_data_nodes + 1,
            pre_remaining_work: 82,
            pre_consumed_work: 18,
            pre_helper_work: 10,
            pre_helper_visits: 5,
            post_remaining_work: 81,
            post_consumed_work: 19,
            post_helper_work: 11,
            post_helper_visits: 5,
        }
    }

    fn new_cap_refused(
        case_id: &'static str,
        capacity: usize,
        prefix_rows: Vec<RawDataSlot>,
        pre_alloc_data_nodes: u128,
        bounds_data_nodes: u128,
        pre_reg_a: u128,
        staged_u128: u128,
    ) -> Self {
        let pre_active = prefix_rows.len();
        assert!(pre_active <= capacity);
        assert!(pre_alloc_data_nodes >= bounds_data_nodes);
        let mut pre_data_rows = prefix_rows;
        pre_data_rows.resize(capacity, RawDataSlot::zero());
        let post_data_rows = pre_data_rows.clone();
        Self {
            case_id,
            enable: 1,
            bounds_data_nodes,
            pre_active_data: pre_active as u128,
            post_active_data: pre_active as u128,
            pre_data_rows,
            post_data_rows,
            pre_reg_a,
            staged_u128,
            applied: false,
            pre_alloc_data_nodes,
            post_alloc_data_nodes: pre_alloc_data_nodes,
            pre_remaining_work: 82,
            pre_consumed_work: 18,
            pre_helper_work: 10,
            pre_helper_visits: 5,
            post_remaining_work: 81,
            post_consumed_work: 19,
            post_helper_work: 11,
            post_helper_visits: 5,
        }
    }

    fn build_symbolic_inputs(&self) -> (BScalarExitAddU128Inputs, BTreeMap<String, u128>) {
        let env = std::cell::RefCell::new(BTreeMap::new());
        let bind = |name: String, val: u128| -> Operand {
            env.borrow_mut().insert(name.clone(), val);
            Operand::Variable(Identifier(name))
        };
        let bind_u128 = |prefix: &str, val: u128| -> BU128Lanes {
            let bytes = val.to_le_bytes();
            std::array::from_fn(|i| bind(format!("{prefix}.{i}"), u128::from(bytes[i])))
        };
        let bind_data_row = |prefix: &str, slot: &RawDataSlot| -> BDataRowLanes {
            BDataRowLanes {
                tag: bind(format!("{prefix}.tag"), slot.tag),
                n: bind_u128(&format!("{prefix}.n"), slot.n),
                bool_b: bind(format!("{prefix}.bool"), slot.bool_b),
                fields_start: bind(format!("{prefix}.f_start"), slot.fields_start),
                fields_len: bind(format!("{prefix}.f_len"), slot.fields_len),
                rest: vec![
                    bind(format!("{prefix}.rest.0"), slot.rest[0]),
                    bind(format!("{prefix}.rest.1"), slot.rest[1]),
                ],
            }
        };

        let enable = bind("%in.enable".into(), self.enable);
        let bounds_data_nodes = bind("%in.bounds.data_nodes".into(), self.bounds_data_nodes);

        let pre_data = BDataTableLanes {
            capacity: self.pre_data_rows.len(),
            active_rows: bind("%in.pre.data.active".into(), self.pre_active_data),
            rows: self
                .pre_data_rows
                .iter()
                .enumerate()
                .map(|(r, s)| bind_data_row(&format!("%in.pre.data.row.{r}"), s))
                .collect(),
        };
        let post_data = BDataTableLanes {
            capacity: self.post_data_rows.len(),
            active_rows: bind("%in.post.data.active".into(), self.post_active_data),
            rows: self
                .post_data_rows
                .iter()
                .enumerate()
                .map(|(r, s)| bind_data_row(&format!("%in.post.data.row.{r}"), s))
                .collect(),
        };

        let pre_rest = BScalarHelperRestLanes {
            origin_reference_cursor: bind("%in.pre.h.rest.0".into(), 8),
            parent_resume_present: bind("%in.pre.h.rest.1".into(), 0),
            parent_resume_value: bind("%in.pre.h.rest.2".into(), 0),
            expected_present: bind("%in.pre.h.rest.3".into(), 0),
            expected_value: bind("%in.pre.h.rest.4".into(), 0),
        };
        let pre_helper = BScalarArithmeticRegister {
            present: bind("%in.pre.h.present".into(), HELPER_PRESENT),
            kind: bind("%in.pre.h.kind".into(), HELPER_KIND_SCALAR),
            site: bind("%in.pre.h.site".into(), SITE_RET_RIGHT),
            mode: bind("%in.pre.h.mode".into(), SCALAR_MODE_ADD),
            phase: bind("%in.pre.h.phase".into(), SCALAR_PHASE_DONE),
            operand_a: BValueRef {
                space: bind("%in.pre.h.op_a.space".into(), 0),
                index: bind("%in.pre.h.op_a.index".into(), 0),
            },
            operand_b: BOptValueRef {
                present: bind("%in.pre.h.op_b.present".into(), OPT_PRESENT),
                value: BValueRef {
                    space: bind("%in.pre.h.op_b.space".into(), 0),
                    index: bind("%in.pre.h.op_b.index".into(), 1),
                },
            },
            reg_a: bind_u128("%in.pre.h.reg_a", self.pre_reg_a),
            staged: BScalarStagedLanes {
                tag: bind("%in.pre.h.staged.tag".into(), SCALAR_STAGED_TAG_U128_VALUE),
                u128_value: bind_u128("%in.pre.h.staged.u128", self.staged_u128),
                bool_value: bind("%in.pre.h.staged.bool".into(), 0),
                pending_node_tag: bind("%in.pre.h.staged.pending".into(), 0),
                prim_error_code: bind("%in.pre.h.staged.err".into(), 0),
                selector: bind("%in.pre.h.staged.sel".into(), 0),
                type_test_ok: bind("%in.pre.h.staged.ok".into(), 0),
            },
            rest: pre_rest.to_rest_vec(),
        };

        let post_h_val = |v: u128| -> u128 { if self.applied { 0 } else { v } };
        let post_rest = BScalarHelperRestLanes {
            origin_reference_cursor: bind("%in.post.h.rest.0".into(), post_h_val(8)),
            parent_resume_present: bind("%in.post.h.rest.1".into(), 0),
            parent_resume_value: bind("%in.post.h.rest.2".into(), 0),
            expected_present: bind("%in.post.h.rest.3".into(), 0),
            expected_value: bind("%in.post.h.rest.4".into(), 0),
        };
        let post_helper = BScalarArithmeticRegister {
            present: bind("%in.post.h.present".into(), post_h_val(HELPER_PRESENT)),
            kind: bind("%in.post.h.kind".into(), post_h_val(HELPER_KIND_SCALAR)),
            site: bind("%in.post.h.site".into(), post_h_val(SITE_RET_RIGHT)),
            mode: bind("%in.post.h.mode".into(), post_h_val(SCALAR_MODE_ADD)),
            phase: bind("%in.post.h.phase".into(), post_h_val(SCALAR_PHASE_DONE)),
            operand_a: BValueRef {
                space: bind("%in.post.h.op_a.space".into(), 0),
                index: bind("%in.post.h.op_a.index".into(), 0),
            },
            operand_b: BOptValueRef {
                present: bind("%in.post.h.op_b.present".into(), post_h_val(OPT_PRESENT)),
                value: BValueRef {
                    space: bind("%in.post.h.op_b.space".into(), 0),
                    index: bind("%in.post.h.op_b.index".into(), post_h_val(1)),
                },
            },
            reg_a: bind_u128("%in.post.h.reg_a", post_h_val(self.pre_reg_a)),
            staged: BScalarStagedLanes {
                tag: bind(
                    "%in.post.h.staged.tag".into(),
                    post_h_val(SCALAR_STAGED_TAG_U128_VALUE),
                ),
                u128_value: bind_u128("%in.post.h.staged.u128", post_h_val(self.staged_u128)),
                bool_value: bind("%in.post.h.staged.bool".into(), 0),
                pending_node_tag: bind("%in.post.h.staged.pending".into(), 0),
                prim_error_code: bind("%in.post.h.staged.err".into(), 0),
                selector: bind("%in.post.h.staged.sel".into(), 0),
                type_test_ok: bind("%in.post.h.staged.ok".into(), 0),
            },
            rest: post_rest.to_rest_vec(),
        };

        let pre_result = BHelperResultLanes {
            present: bind("%in.pre.res.present".into(), 0),
            tag: bind("%in.pre.res.tag".into(), 0),
            scalar_value: BValueRef {
                space: bind("%in.pre.res.val.space".into(), 0),
                index: bind("%in.pre.res.val.index".into(), 0),
            },
            scalar_error_code: bind("%in.pre.res.err".into(), 0),
            scalar_selector: bind("%in.pre.res.sel".into(), 0),
            scalar_type_test_ok: bind("%in.pre.res.ok".into(), 0),
            rest: vec![bind("%in.pre.res.rest.0".into(), 0)],
        };
        let post_result = BHelperResultLanes {
            present: bind(
                "%in.post.res.present".into(),
                if self.applied { OPT_PRESENT } else { 0 },
            ),
            tag: bind(
                "%in.post.res.tag".into(),
                if self.applied {
                    HELPER_RESULT_TAG_SCALAR_VALUE
                } else {
                    0
                },
            ),
            scalar_value: BValueRef {
                space: bind(
                    "%in.post.res.val.space".into(),
                    if self.applied { VALUE_SPACE_DATA } else { 0 },
                ),
                index: bind(
                    "%in.post.res.val.index".into(),
                    if self.applied {
                        self.pre_active_data
                    } else {
                        0
                    },
                ),
            },
            scalar_error_code: bind("%in.post.res.err".into(), 0),
            scalar_selector: bind("%in.post.res.sel".into(), 0),
            scalar_type_test_ok: bind("%in.post.res.ok".into(), 0),
            rest: vec![bind("%in.post.res.rest.0".into(), 0)],
        };

        let pre_outcome = BOutcomeLanes {
            tag: bind("%in.pre.out.tag".into(), OUTCOME_TAG_RUNNING),
            halted_value: BValueRef {
                space: bind("%in.pre.out.halt.space".into(), 0),
                index: bind("%in.pre.out.halt.index".into(), 0),
            },
            rejected_code: bind("%in.pre.out.rej".into(), 0),
            exhausted_reason: bind("%in.pre.out.exh".into(), 0),
        };
        let post_outcome = BOutcomeLanes {
            tag: bind(
                "%in.post.out.tag".into(),
                if self.applied {
                    OUTCOME_TAG_RUNNING
                } else {
                    OUTCOME_TAG_EXHAUSTED
                },
            ),
            halted_value: BValueRef {
                space: bind("%in.post.out.halt.space".into(), 0),
                index: bind("%in.post.out.halt.index".into(), 0),
            },
            rejected_code: bind("%in.post.out.rej".into(), 0),
            exhausted_reason: bind(
                "%in.post.out.exh".into(),
                if self.applied {
                    0
                } else {
                    EXHAUST_REASON_DATA_NODES
                },
            ),
        };

        let pre_counters = BScalarCompletionCounters {
            work: BChargedCounters {
                remaining_work: bind_u128("%in.pre.cnt.rem", self.pre_remaining_work),
                consumed_work: bind_u128("%in.pre.cnt.con", self.pre_consumed_work),
                helper_work: bind_u128("%in.pre.cnt.hw", self.pre_helper_work),
                helper_visits: bind_u128("%in.pre.cnt.hv", self.pre_helper_visits),
            },
            reference_cursor: bind("%in.pre.cnt.ref_cur".into(), 8),
            alloc_data_nodes: bind("%in.pre.cnt.alloc_dn".into(), self.pre_alloc_data_nodes),
            alloc_data_fields: bind("%in.pre.cnt.alloc_df".into(), 2),
            alloc_environment_cells: bind("%in.pre.cnt.alloc_env".into(), 2),
            depth: bind("%in.pre.cnt.depth".into(), 1),
            task_live: bind("%in.pre.cnt.tasks".into(), 0),
            active_data_fields: bind("%in.pre.cnt.act_df".into(), 2),
            active_env: bind("%in.pre.cnt.act_env".into(), 2),
            active_value_slots: bind("%in.pre.cnt.act_vs".into(), 2),
            rest: vec![bind("%in.pre.cnt.rest.0".into(), 100)],
        };
        let post_counters = BScalarCompletionCounters {
            work: BChargedCounters {
                remaining_work: bind_u128("%in.post.cnt.rem", self.post_remaining_work),
                consumed_work: bind_u128("%in.post.cnt.con", self.post_consumed_work),
                helper_work: bind_u128("%in.post.cnt.hw", self.post_helper_work),
                helper_visits: bind_u128("%in.post.cnt.hv", self.post_helper_visits),
            },
            reference_cursor: bind("%in.post.cnt.ref_cur".into(), 8),
            alloc_data_nodes: bind("%in.post.cnt.alloc_dn".into(), self.post_alloc_data_nodes),
            alloc_data_fields: bind("%in.post.cnt.alloc_df".into(), 2),
            alloc_environment_cells: bind("%in.post.cnt.alloc_env".into(), 2),
            depth: bind("%in.post.cnt.depth".into(), 1),
            task_live: bind("%in.post.cnt.tasks".into(), 0),
            active_data_fields: bind("%in.post.cnt.act_df".into(), 2),
            active_env: bind("%in.post.cnt.act_env".into(), 2),
            active_value_slots: bind("%in.post.cnt.act_vs".into(), 2),
            rest: vec![bind("%in.post.cnt.rest.0".into(), 100)],
        };

        drop(bind_data_row);
        drop(bind_u128);
        drop(bind);
        (
            BScalarExitAddU128Inputs {
                enable,
                bounds_data_nodes,
                pre_data,
                post_data,
                pre_helper,
                post_helper,
                pre_result,
                post_result,
                pre_outcome,
                post_outcome,
                pre_counters,
                post_counters,
            },
            env.into_inner(),
        )
    }
}

#[derive(Clone, Debug)]
struct FailureAddFixture {
    case_id: &'static str,
    enable: u128,
    error_code: u128,
    pre_reg_a: u128,
    pre_data_rows: Vec<RawDataSlot>,
    active_data: u128,
    alloc_data_nodes: u128,
}

impl FailureAddFixture {
    fn new(case_id: &'static str, error_code: u128, pre_reg_a: u128) -> Self {
        Self {
            case_id,
            enable: 1,
            error_code,
            pre_reg_a,
            pre_data_rows: vec![RawDataSlot::u128_row(42), RawDataSlot::zero()],
            active_data: 1,
            alloc_data_nodes: 3,
        }
    }

    fn build_symbolic_inputs(&self) -> (BScalarFailureAddInputs, BTreeMap<String, u128>) {
        let exit_template = ExitAddU128Fixture::new_applied(
            self.case_id,
            self.pre_data_rows.len(),
            vec![self.pre_data_rows[0].clone()],
            self.alloc_data_nodes,
            self.alloc_data_nodes + 1,
            self.pre_reg_a,
            0,
        );
        let (exit_inputs, mut env) = exit_template.build_symbolic_inputs();
        env.insert("%in.enable".into(), self.enable);
        env.insert("%in.pre.h.staged.tag".into(), SCALAR_STAGED_TAG_PRIM_ERROR);
        env.insert("%in.pre.h.staged.err".into(), self.error_code);
        env.insert("%in.post.data.active".into(), self.active_data);
        env.insert("%in.post.cnt.alloc_dn".into(), self.alloc_data_nodes);
        env.insert("%in.post.res.present".into(), OPT_PRESENT);
        env.insert("%in.post.res.tag".into(), HELPER_RESULT_TAG_SCALAR_ERROR);
        env.insert("%in.post.res.val.space".into(), 0);
        env.insert("%in.post.res.val.index".into(), 0);
        env.insert("%in.post.res.err".into(), self.error_code);
        env.insert("%in.post.out.tag".into(), OUTCOME_TAG_RUNNING);
        env.insert("%in.post.out.exh".into(), 0);

        (
            BScalarFailureAddInputs {
                enable: exit_inputs.enable,
                pre_data: exit_inputs.pre_data,
                post_data: exit_inputs.post_data,
                pre_helper: exit_inputs.pre_helper,
                post_helper: exit_inputs.post_helper,
                pre_result: exit_inputs.pre_result,
                post_result: exit_inputs.post_result,
                pre_outcome: exit_inputs.pre_outcome,
                post_outcome: exit_inputs.post_outcome,
                pre_counters: exit_inputs.pre_counters,
                post_counters: exit_inputs.post_counters,
            },
            env,
        )
    }
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

fn eval_operand(op: &Operand, env: &BTreeMap<String, u128>) -> Result<u128, EvalRejection> {
    match op {
        Operand::Variable(id) => env
            .get(&id.0)
            .copied()
            .ok_or_else(|| EvalRejection::UnboundVariable(id.0.clone())),
        Operand::Immediate(fr) => {
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

#[test]
fn exit_applied_covers_zero_nonzero_max_payloads_first_and_last_slots_and_prefix() {
    let existing_row_0 = RawDataSlot {
        tag: VALUE_TAG_BOOL,
        n: 0,
        bool_b: 1,
        fields_start: 0,
        fields_len: 0,
        rest: [7, 9],
    };
    let existing_row_1 = RawDataSlot::u128_row(65_536);

    let cases = vec![
        // First physical slot (index 0 of 2), zero payload, zero existing alloc_data_nodes
        ExitAddU128Fixture::new_applied("first-slot-zero", 2, vec![], 0, 2, 0, 0),
        // Middle slot (index 1 of 3), small sum 16, non-zero alloc_data_nodes > active.data
        ExitAddU128Fixture::new_applied(
            "mid-slot-nonzero-alloc",
            3,
            vec![existing_row_0.clone()],
            4,
            6,
            7,
            16,
        ),
        // Last physical slot (index 2 of 3), u128::MAX payload, exact cap boundary alloc+1 == bounds
        ExitAddU128Fixture::new_applied(
            "last-slot-max-at-cap-boundary",
            3,
            vec![existing_row_0, existing_row_1],
            9,
            10,
            u128::MAX - 1,
            u128::MAX,
        ),
    ];

    for case in cases {
        let (inputs, env) = case.build_symbolic_inputs();
        let block = emit_b_scalar_exit_add_u128(&inputs, "exit_ok").unwrap();
        assert_eq!(block.preserved, inputs, "case {}", case.case_id);
        let evaluated = eval_emitted_instructions(&block.instructions, &env)
            .unwrap_or_else(|e| panic!("case {} rejected: {e:?}", case.case_id));

        assert_eq!(eval_operand(&block.applied, &evaluated).unwrap(), 1);
        assert_eq!(eval_operand(&block.cap_refused, &evaluated).unwrap(), 0);
        assert_eq!(
            eval_operand(&block.allocated_index, &evaluated).unwrap(),
            case.pre_active_data
        );
        let mut alloc_bytes = [0u8; 16];
        for i in 0..16 {
            alloc_bytes[i] = eval_operand(&block.allocated_n[i], &evaluated).unwrap() as u8;
        }
        assert_eq!(u128::from_le_bytes(alloc_bytes), case.staged_u128);
        for (slot, sel) in block.write_selectors.iter().enumerate() {
            assert_eq!(
                eval_operand(sel, &evaluated).unwrap(),
                u128::from(slot as u128 == case.pre_active_data)
            );
        }
    }
}

#[test]
fn exit_refused_cap_charged_retains_helper_and_arenas_and_sets_exhausted_data_nodes() {
    let cases = vec![
        // Zero capacity / zero bound: first allocation triggers Exhausted(DataNodes), charged
        ExitAddU128Fixture::new_cap_refused("zero-bound-cap-refused", 0, vec![], 0, 0, 7, 16),
        // Logical alloc_data_nodes == bounds_data_nodes even when physical slot exists
        ExitAddU128Fixture::new_cap_refused(
            "logical-cap-reached-with-free-physical-slot",
            3,
            vec![RawDataSlot::u128_row(99)],
            5,
            5,
            100,
            250,
        ),
    ];

    for case in cases {
        let (inputs, env) = case.build_symbolic_inputs();
        let block = emit_b_scalar_exit_add_u128(&inputs, "exit_cap").unwrap();
        let evaluated = eval_emitted_instructions(&block.instructions, &env)
            .unwrap_or_else(|e| panic!("case {} rejected: {e:?}", case.case_id));
        assert_eq!(eval_operand(&block.applied, &evaluated).unwrap(), 0);
        assert_eq!(eval_operand(&block.cap_refused, &evaluated).unwrap(), 1);
        assert_eq!(eval_operand(&block.allocated_index, &evaluated).unwrap(), 0);
        for i in 0..16 {
            assert_eq!(eval_operand(&block.allocated_n[i], &evaluated).unwrap(), 0);
        }
    }
}

#[test]
fn failure_transfers_codes_10_and_11_clears_helper_and_preserves_data_and_visits() {
    for (case_id, code) in [
        ("overflow-10", PRIM_ERROR_ARITH_OVERFLOW),
        ("sort-mismatch-11", PRIM_ERROR_SORT_MISMATCH),
    ] {
        let fixture = FailureAddFixture::new(case_id, code, 42);
        let (inputs, env) = fixture.build_symbolic_inputs();
        let block = emit_b_scalar_failure_add(&inputs, "fail_ok").unwrap();
        assert_eq!(block.preserved, inputs);
        let evaluated = eval_emitted_instructions(&block.instructions, &env)
            .unwrap_or_else(|e| panic!("case {case_id} rejected: {e:?}"));

        assert_eq!(eval_operand(&block.error_code, &evaluated).unwrap(), code);
        assert_eq!(
            eval_operand(&block.is_overflow_10, &evaluated).unwrap(),
            u128::from(code == PRIM_ERROR_ARITH_OVERFLOW)
        );
        assert_eq!(
            eval_operand(&block.is_sort_mismatch_11, &evaluated).unwrap(),
            u128::from(code == PRIM_ERROR_SORT_MISMATCH)
        );
    }
}

#[test]
fn security_negative_exit_applied_mutations_are_rejected_in_circuit() {
    let fixture = ExitAddU128Fixture::new_applied(
        "neg-applied",
        2,
        vec![RawDataSlot::u128_row(55)],
        3,
        5,
        7,
        16,
    );
    let (inputs, base_env) = fixture.build_symbolic_inputs();
    let block = emit_b_scalar_exit_add_u128(&inputs, "exit_neg").unwrap();

    let assert_rejects = |key: &str, bad: u128, label: &str| {
        let mut mutated = base_env.clone();
        mutated.insert(key.into(), bad);
        assert!(
            eval_emitted_instructions(&block.instructions, &mutated).is_err(),
            "expected circuit rejection for {label} ({key} = {bad})"
        );
    };

    // 1. Pre-state guards (phase, mode, site, staged tag, outcome, absent result)
    assert_rejects("%in.pre.h.phase", SCALAR_PHASE_SECOND, "pre.phase != Done");
    assert_rejects(
        "%in.pre.h.staged.tag",
        SCALAR_STAGED_TAG_PRIM_ERROR,
        "pre.staged.tag == PrimError on exit",
    );
    assert_rejects("%in.pre.out.tag", OUTCOME_TAG_EXHAUSTED, "pre.outcome != Running");
    assert_rejects("%in.pre.res.present", 1, "pre.result already present");

    // 2. Appended row payload, tag, inactive union columns, and existing prefix row preservation
    assert_rejects("%in.post.data.row.1.tag", VALUE_TAG_BOOL, "new DataRow wrong tag");
    assert_rejects("%in.post.data.row.1.n.0", 17, "new DataRow wrong U128 byte 0");
    assert_rejects("%in.post.data.row.1.bool", 1, "new DataRow dirty inactive bool_b");
    assert_rejects(
        "%in.post.data.row.1.f_start",
        1,
        "new DataRow dirty inactive fields_start",
    );
    assert_rejects("%in.post.data.row.1.rest.0", 1, "new DataRow dirty rest column");
    assert_rejects("%in.post.data.row.0.n.0", 54, "existing prefix row 0 mutated");
    assert_rejects("%in.pre.data.row.1.n.0", 1, "inactive pre slot 1 non-zero");

    // 3. New DataRef / result / active.data / alloc.dataNodes deltas
    assert_rejects("%in.post.data.active", 1, "active.data not incremented");
    assert_rejects("%in.post.cnt.alloc_dn", 3, "alloc.dataNodes not incremented");
    assert_rejects("%in.post.res.present", 0, "post.result not set");
    assert_rejects(
        "%in.post.res.tag",
        HELPER_RESULT_TAG_SCALAR_ERROR,
        "post.result wrong tag",
    );
    assert_rejects(
        "%in.post.res.val.space",
        VALUE_SPACE_CONST,
        "post.result space != Data",
    );
    assert_rejects("%in.post.res.val.index", 0, "post.result index != pre.active.data");
    assert_rejects("%in.post.res.err", 10, "post.result dirty error_code");

    // 4. clearHelper obligations (every post.helper column must be zeroed)
    assert_rejects("%in.post.h.present", 1, "post.helper.present not cleared");
    assert_rejects("%in.post.h.site", SITE_RET_RIGHT, "post.helper.site not cleared");
    assert_rejects("%in.post.h.reg_a.0", 7, "post.helper.reg_a not cleared");
    assert_rejects("%in.post.h.staged.u128.0", 16, "post.helper.staged.u128 not cleared");
    assert_rejects("%in.post.h.rest.0", 8, "post.helper.rest[0] not cleared");

    // 5. Counter preservation: helperVisits must NOT increment at exit!
    assert_rejects("%in.post.cnt.hv.0", 6, "helperVisits incremented at exit");
    assert_rejects("%in.post.cnt.rem.0", 82, "remainingWork not decremented");
    assert_rejects("%in.post.cnt.con.0", 18, "consumedWork not incremented");
    assert_rejects("%in.post.cnt.hw.0", 10, "helperWork not incremented");
    assert_rejects("%in.post.cnt.ref_cur", 9, "referenceCursor mutated at exit");
    assert_rejects("%in.post.cnt.depth", 2, "depth mutated at exit");
}

#[test]
fn security_negative_cap_branch_and_physical_capacity_mismatch_are_rejected() {
    // 1. When alloc_data_nodes == bounds_data_nodes, attempting to apply allocation/clearHelper fails.
    let mut forged_apply = ExitAddU128Fixture::new_applied(
        "forged-apply-at-cap",
        2,
        vec![RawDataSlot::u128_row(1)],
        5,
        6,
        7,
        16,
    );
    forged_apply.bounds_data_nodes = 5; // now pre_alloc (5) >= bounds (5), so cap_refused == 1!
    let (inputs, env) = forged_apply.build_symbolic_inputs();
    let block = emit_b_scalar_exit_add_u128(&inputs, "exit_forged_apply").unwrap();
    assert!(eval_emitted_instructions(&block.instructions, &env).is_err());

    // 2. On RefusedCapCharged, clearing helper or mutating outcome reason must fail.
    let cap_case = ExitAddU128Fixture::new_cap_refused(
        "cap-refused-strict",
        2,
        vec![RawDataSlot::u128_row(1)],
        5,
        5,
        7,
        16,
    );
    let (cap_inputs, cap_env) = cap_case.build_symbolic_inputs();
    let cap_block = emit_b_scalar_exit_add_u128(&cap_inputs, "exit_cap_strict").unwrap();

    let mut cleared_on_cap = cap_env.clone();
    cleared_on_cap.insert("%in.post.h.present".into(), 0);
    assert!(eval_emitted_instructions(&cap_block.instructions, &cleared_on_cap).is_err());

    let mut wrong_reason = cap_env.clone();
    wrong_reason.insert("%in.post.out.exh".into(), EXHAUST_REASON_DATA_FIELDS);
    assert!(eval_emitted_instructions(&cap_block.instructions, &wrong_reason).is_err());

    // 3. E04 physical capacity bound isolation: when logical cap allows allocation
    // (pre.alloc.dataNodes = 5 < bounds.dataNodes = 10, so applied == 1) and E03
    // logical counter increment is satisfied (post.alloc.dataNodes = 6), a witness
    // at physical capacity (pre.active.data = 2 == capacity) with consistent Applied
    // delta (post.active.data = 3) reaches E04 and is rejected at the E04
    // `post_exceeds_cap` check (`b.eq(post_exceeds_cap, immediate(0))` at index 144,
    // which precedes `b.assert(guarded_physical_cap)` at index 148 and
    // `b.gated_eq(enable, eff_post_active_data, active_data_plus)` at index 152).
    let mut phys_full = cap_env;
    phys_full.insert("%in.bounds.data_nodes".into(), 10);
    phys_full.insert("%in.post.cnt.alloc_dn".into(), 6);
    phys_full.insert("%in.pre.data.active".into(), 2); // equals capacity 2
    phys_full.insert("%in.post.data.active".into(), 3);
    assert_eq!(
        cap_block.instructions[142],
        Instruction::LessThan {
            a: immediate(2),
            b: var("%exit_cap_strict.local.temp.68"),
            bits: 128,
            output: Identifier("%exit_cap_strict.local.temp.70".into()),
        }
    );
    assert_eq!(
        cap_block.instructions[144],
        Instruction::ConstrainEq {
            a: var("%exit_cap_strict.local.temp.70"),
            b: immediate(0),
        }
    );
    assert_eq!(
        eval_emitted_instructions(&cap_block.instructions, &phys_full),
        Err(EvalRejection::ConstraintEqFailed {
            index: 144,
            left: 1,
            right: 0,
        })
    );
}

#[test]
fn security_negative_failure_mutations_are_rejected_in_circuit() {
    let fixture = FailureAddFixture::new("neg-fail", PRIM_ERROR_ARITH_OVERFLOW, 7);
    let (inputs, base_env) = fixture.build_symbolic_inputs();
    let block = emit_b_scalar_failure_add(&inputs, "fail_neg").unwrap();

    let assert_rejects = |key: &str, bad: u128, label: &str| {
        let mut mutated = base_env.clone();
        mutated.insert(key.into(), bad);
        assert!(
            eval_emitted_instructions(&block.instructions, &mutated).is_err(),
            "expected circuit rejection for {label} ({key} = {bad})"
        );
    };

    assert_rejects("%in.pre.h.staged.err", 9, "invalid ErrorCode 9 not in {10,11}");
    assert_rejects("%in.pre.h.staged.err", 12, "invalid ErrorCode 12 not in {10,11}");
    assert_rejects("%in.pre.h.staged.u128.0", 1, "non-zero inactive staged u128 byte");
    assert_rejects(
        "%in.post.res.err",
        PRIM_ERROR_SORT_MISMATCH,
        "mismatched result error code 11 vs 10",
    );
    assert_rejects(
        "%in.post.out.tag",
        OUTCOME_TAG_REJECTED,
        "premature Rejected outcome publication at HelperFail",
    );
    assert_rejects("%in.post.data.active", 2, "spurious Data allocation on failure");
    assert_rejects("%in.post.cnt.alloc_dn", 4, "spurious alloc.dataNodes increment");
    assert_rejects("%in.post.h.present", 1, "helper not cleared on failure");
    assert_rejects("%in.post.cnt.hv.0", 6, "helperVisits incremented on failure");
}

#[test]
fn disabled_exit_and_failure_branches_do_not_constrain_post_state() {
    let mut exit_disabled =
        ExitAddU128Fixture::new_applied("disabled-exit", 1, vec![], 0, 1, 7, 16);
    exit_disabled.enable = 0;
    exit_disabled.post_active_data = 99;
    exit_disabled.post_alloc_data_nodes = 99;
    exit_disabled.post_remaining_work = 999;
    let (exit_inputs, mut exit_env) = exit_disabled.build_symbolic_inputs();
    exit_env.insert("%in.pre.h.phase".into(), SCALAR_PHASE_FIRST);
    exit_env.insert("%in.post.out.tag".into(), OUTCOME_TAG_REJECTED);
    let exit_block = emit_b_scalar_exit_add_u128(&exit_inputs, "exit_dis").unwrap();
    let eval_exit = eval_emitted_instructions(&exit_block.instructions, &exit_env).unwrap();
    assert_eq!(eval_operand(&exit_block.applied, &eval_exit).unwrap(), 0);
    assert_eq!(eval_operand(&exit_block.cap_refused, &eval_exit).unwrap(), 0);

    let mut fail_disabled = FailureAddFixture::new("disabled-fail", PRIM_ERROR_ARITH_OVERFLOW, 7);
    fail_disabled.enable = 0;
    let (fail_inputs, mut fail_env) = fail_disabled.build_symbolic_inputs();
    fail_env.insert("%in.pre.h.staged.err".into(), 0);
    fail_env.insert("%in.post.res.present".into(), 0);
    let fail_block = emit_b_scalar_failure_add(&fail_inputs, "fail_dis").unwrap();
    let eval_fail = eval_emitted_instructions(&fail_block.instructions, &fail_env).unwrap();
    assert_eq!(eval_operand(&fail_block.error_code, &eval_fail).unwrap(), 0);
}

#[test]
fn completion_constructors_reject_invalid_scopes_shapes_and_temp_collisions() {
    let fixture = ExitAddU128Fixture::new_applied("ctor", 1, vec![], 0, 1, 0, 0);
    let (base, _) = fixture.build_symbolic_inputs();

    assert_eq!(
        emit_b_scalar_exit_add_u128(&base, ""),
        Err(BScalarCompletionConstructionError::Fetch(
            ConstructionError::EmptyScope
        ))
    );

    let mut bad_cap = base.clone();
    bad_cap.pre_data.capacity = 2;
    assert_eq!(
        emit_b_scalar_exit_add_u128(&bad_cap, "s"),
        Err(BScalarCompletionConstructionError::DataRowCount)
    );

    let mut bad_row_rest = base.clone();
    bad_row_rest.post_data.rows[0].rest.pop();
    assert_eq!(
        emit_b_scalar_exit_add_u128(&bad_row_rest, "s"),
        Err(BScalarCompletionConstructionError::DataRowRestLaneCount)
    );

    let mut bad_h_rest = base.clone();
    bad_h_rest.post_helper.rest.pop();
    assert_eq!(
        emit_b_scalar_exit_add_u128(&bad_h_rest, "s"),
        Err(BScalarCompletionConstructionError::HelperRestLaneCount)
    );

    let mut bad_res_rest = base.clone();
    bad_res_rest.post_result.rest.pop();
    assert_eq!(
        emit_b_scalar_exit_add_u128(&bad_res_rest, "s"),
        Err(BScalarCompletionConstructionError::ResultRestLaneCount)
    );

    let mut bad_cnt_rest = base.clone();
    bad_cnt_rest.post_counters.rest.pop();
    assert_eq!(
        emit_b_scalar_exit_add_u128(&bad_cnt_rest, "s"),
        Err(BScalarCompletionConstructionError::CounterRestLaneCount)
    );

    let mut unprefixed = base.clone();
    unprefixed.bounds_data_nodes = var("bad_var");
    assert_eq!(
        emit_b_scalar_exit_add_u128(&unprefixed, "s"),
        Err(BScalarCompletionConstructionError::Fetch(
            ConstructionError::UnprefixedVariable
        ))
    );

    let mut colliding = base;
    colliding.post_outcome.exhausted_reason = var("%s.local.temp.0");
    assert_eq!(
        emit_b_scalar_exit_add_u128(&colliding, "s"),
        Err(BScalarCompletionConstructionError::Fetch(
            ConstructionError::TemporaryNameCollision
        ))
    );
}
