// Target: crates/mori-backend/src/sherlock_machine/b_admin_publication_normal_tests.rs

//! UNRUN unit, chain, and security-negative witness tests for `b_admin_publication`
//! (`k.admin.publish.value`, `k.admin.publish.error`, `emit_b_admin_publish_p1`,
//! and the 5-stage `BinaryOp::Add` chain `emit_b_add_to_publication_chain`).
//!
//! Host `u128` instruction evaluation here checks emitted `midnight_zkir_v3::Instruction`
//! constraints directly and remains distinct from prime-field `Fr` / Halo2 proof execution.

use std::cell::RefCell;
use std::collections::BTreeMap;

use super::b_admin_publication::*;
use super::b_return_right_completion::*;
use super::b_scalar_arithmetic::*;
use super::b_scalar_completion::*;
use super::*;

fn eval_operand(op: &Operand, env: &BTreeMap<String, u128>) -> Result<u128, String> {
    match op {
        Operand::Variable(id) => env
            .get(&id.0)
            .copied()
            .ok_or_else(|| format!("unbound variable `{}`", id.0)),
        Operand::Immediate(fr) => {
            for candidate in 0u128..=1024 {
                if *fr == Fr::from(candidate) {
                    return Ok(candidate);
                }
            }
            Err(format!("immediate:{fr:?}"))
        }
    }
}

fn eval_zkir(
    instructions: &[Instruction],
    mut env: BTreeMap<String, u128>,
) -> Result<BTreeMap<String, u128>, String> {
    for (index, inst) in instructions.iter().enumerate() {
        match inst {
            Instruction::CondSelect { bit, a, b, output } => {
                let cond = eval_operand(bit, &env)?;
                if cond > 1 {
                    return Err(format!("step {index}: non-boolean CondSelect bit {cond}"));
                }
                let va = eval_operand(a, &env)?;
                let vb = eval_operand(b, &env)?;
                env.insert(output.0.clone(), if cond == 1 { va } else { vb });
            }
            Instruction::ConstrainToBoolean { val } => {
                let v = eval_operand(val, &env)?;
                if v > 1 {
                    return Err(format!("step {index}: non-boolean value {v}"));
                }
            }
            Instruction::ConstrainBits { val, bits } => {
                let v = eval_operand(val, &env)?;
                if *bits < 128 && (v >> *bits) != 0 {
                    return Err(format!(
                        "step {index}: ConstrainBits({bits}) failed for {v}"
                    ));
                }
            }
            Instruction::ConstrainEq { a, b } => {
                let va = eval_operand(a, &env)?;
                let vb = eval_operand(b, &env)?;
                if va != vb {
                    return Err(format!(
                        "step {index}: ConstrainEq failed ({va} != {vb})"
                    ));
                }
            }
            Instruction::Assert { cond } => {
                let c = eval_operand(cond, &env)?;
                if c != 1 {
                    return Err(format!("step {index}: Assert failed (cond = {c})"));
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
                    return Err(format!(
                        "step {index}: LessThan operand exceeds {bits} bits"
                    ));
                }
                env.insert(output.0.clone(), u128::from(va < vb));
            }
            Instruction::DivModPowerOfTwo { val, bits, outputs } => {
                if outputs.len() != 2 {
                    return Err(format!(
                        "step {index}: DivModPowerOfTwo expected 2 outputs, got {}",
                        outputs.len()
                    ));
                }
                let v = eval_operand(val, &env)?;
                let q = if *bits >= 128 { 0 } else { v >> *bits };
                let r = if *bits >= 128 {
                    v
                } else {
                    v & ((1u128 << *bits) - 1)
                };
                env.insert(outputs[0].0.clone(), q);
                env.insert(outputs[1].0.clone(), r);
            }
            Instruction::Add { a, b, output } => {
                let va = eval_operand(a, &env)?;
                let vb = eval_operand(b, &env)?;
                let sum = va
                    .checked_add(vb)
                    .ok_or_else(|| format!("step {index}: host u128 Add overflow"))?;
                env.insert(output.0.clone(), sum);
            }
            Instruction::Mul { a, b, output } => {
                let va = eval_operand(a, &env)?;
                let vb = eval_operand(b, &env)?;
                let prod = va
                    .checked_mul(vb)
                    .ok_or_else(|| format!("step {index}: host u128 Mul overflow"))?;
                env.insert(output.0.clone(), prod);
            }
            _ => return Err(format!("step {index}: unsupported instruction")),
        }
    }
    Ok(env)
}

#[derive(Clone, Debug)]
struct ConcreteDataSlot {
    tag: u128,
    n: u128,
    bool_b: u128,
    f_start: u128,
    f_len: u128,
    rest: [u128; 1],
}

impl ConcreteDataSlot {
    fn zero() -> Self {
        Self {
            tag: 0,
            n: 0,
            bool_b: 0,
            f_start: 0,
            f_len: 0,
            rest: [0],
        }
    }

    fn u128_row(n: u128) -> Self {
        Self {
            tag: DATA_TAG_U128,
            n,
            bool_b: 0,
            f_start: 0,
            f_len: 0,
            rest: [0],
        }
    }

    fn bool_row(b: u128) -> Self {
        Self {
            tag: VALUE_TAG_BOOL,
            n: 0,
            bool_b: b,
            f_start: 0,
            f_len: 0,
            rest: [0],
        }
    }
}

#[derive(Clone, Debug)]
struct ConcreteFrameSlot {
    tag: u128,
    origin: u128,
    right_op: u128,
    right_left_space: u128,
    right_left_index: u128,
    rest: [u128; 1],
}

impl ConcreteFrameSlot {
    fn zero() -> Self {
        Self {
            tag: 0,
            origin: 0,
            right_op: 0,
            right_left_space: 0,
            right_left_index: 0,
            rest: [0],
        }
    }

    fn right_f_add(origin: u128, left_space: u128, left_index: u128) -> Self {
        Self {
            tag: FRAME_TAG_RIGHT_F,
            origin,
            right_op: BINARY_OP_ADD,
            right_left_space: left_space,
            right_left_index: left_index,
            rest: [0],
        }
    }
}

#[derive(Clone, Debug)]
struct ConcreteHelperState {
    present: u128,
    kind: u128,
    site: u128,
    mode: u128,
    phase: u128,
    op_a_space: u128,
    op_a_index: u128,
    op_b_present: u128,
    op_b_space: u128,
    op_b_index: u128,
    reg_a: u128,
    staged_tag: u128,
    staged_u128: u128,
    staged_err: u128,
    origin_ref_cursor: u128,
}

impl ConcreteHelperState {
    fn absent() -> Self {
        Self {
            present: 0,
            kind: 0,
            site: 0,
            mode: 0,
            phase: 0,
            op_a_space: 0,
            op_a_index: 0,
            op_b_present: 0,
            op_b_space: 0,
            op_b_index: 0,
            reg_a: 0,
            staged_tag: 0,
            staged_u128: 0,
            staged_err: 0,
            origin_ref_cursor: 0,
        }
    }
}

#[derive(Clone, Debug)]
struct ConcreteAdminState {
    site: u128,
    outcome_tag: u128,
    halted_space: u128,
    halted_index: u128,
    rejected_code: u128,
    exhausted_reason: u128,
    ctrl_tag: u128,
    ctrl_rv_space: u128,
    ctrl_rv_index: u128,
    ctrl_pot: u128,
    ctrl_pv_space: u128,
    ctrl_pv_index: u128,
    ctrl_pec: u128,
    ctrl_ec: u128,
    control_origin: u128,
    helper: ConcreteHelperState,
    res_present: u128,
    res_tag: u128,
    res_val_space: u128,
    res_val_index: u128,
    res_err: u128,
    data_active: u128,
    data_rows: Vec<ConcreteDataSlot>,
    frames: Vec<ConcreteFrameSlot>,
    depth: u128,
    remaining_work: u128,
    consumed_work: u128,
    helper_work: u128,
    helper_visits: u128,
    reference_cursor: u128,
    alloc_data_nodes: u128,
    task_live: u128,
}

impl ConcreteAdminState {
    fn base_return_at_depth_zero(
        rv_space: u128,
        rv_index: u128,
        remaining_work: u128,
    ) -> Self {
        Self {
            site: SITE_PRIM,
            outcome_tag: OUTCOME_TAG_RUNNING,
            halted_space: 0,
            halted_index: 0,
            rejected_code: 0,
            exhausted_reason: 0,
            ctrl_tag: CONTROL_TAG_RETURN,
            ctrl_rv_space: rv_space,
            ctrl_rv_index: rv_index,
            ctrl_pot: 0,
            ctrl_pv_space: 0,
            ctrl_pv_index: 0,
            ctrl_pec: 0,
            ctrl_ec: 0,
            control_origin: 303,
            helper: ConcreteHelperState::absent(),
            res_present: 0,
            res_tag: 0,
            res_val_space: 0,
            res_val_index: 0,
            res_err: 0,
            data_active: 2,
            data_rows: vec![ConcreteDataSlot::u128_row(7), ConcreteDataSlot::u128_row(16)],
            frames: vec![ConcreteFrameSlot::zero(), ConcreteFrameSlot::zero()],
            depth: 0,
            remaining_work,
            consumed_work: 12,
            helper_work: 4,
            helper_visits: 2,
            reference_cursor: 8,
            alloc_data_nodes: 2,
            task_live: 0,
        }
    }
}

struct EnvBuilder {
    env: RefCell<BTreeMap<String, u128>>,
}

impl EnvBuilder {
    fn new() -> Self {
        Self {
            env: RefCell::new(BTreeMap::new()),
        }
    }

    fn bind(&self, name: &str, val: u128) -> Operand {
        self.env.borrow_mut().insert(name.to_string(), val);
        Operand::Variable(Identifier(name.to_string()))
    }

    fn bind_u128(&self, prefix: &str, val: u128) -> BU128Lanes {
        let le = val.to_le_bytes();
        std::array::from_fn(|i| self.bind(&format!("{prefix}.{i}"), u128::from(le[i])))
    }

    fn bind_admin_state(&self, prefix: &str, st: &ConcreteAdminState) -> BAdminPublishState {
        let outcome = BOutcomeLanes {
            tag: self.bind(&format!("{prefix}.out.tag"), st.outcome_tag),
            halted_value: BValueRef {
                space: self.bind(&format!("{prefix}.out.hv.sp"), st.halted_space),
                index: self.bind(&format!("{prefix}.out.hv.ix"), st.halted_index),
            },
            rejected_code: self.bind(&format!("{prefix}.out.rej"), st.rejected_code),
            exhausted_reason: self.bind(&format!("{prefix}.out.exh"), st.exhausted_reason),
        };
        let control = BControlRow {
            tag: self.bind(&format!("{prefix}.ctrl.tag"), st.ctrl_tag),
            return_value: BValueRef {
                space: self.bind(&format!("{prefix}.ctrl.rv.sp"), st.ctrl_rv_space),
                index: self.bind(&format!("{prefix}.ctrl.rv.ix"), st.ctrl_rv_index),
            },
            prim_outcome_tag: self.bind(&format!("{prefix}.ctrl.pot"), st.ctrl_pot),
            prim_value: BValueRef {
                space: self.bind(&format!("{prefix}.ctrl.pv.sp"), st.ctrl_pv_space),
                index: self.bind(&format!("{prefix}.ctrl.pv.ix"), st.ctrl_pv_index),
            },
            prim_error_code: self.bind(&format!("{prefix}.ctrl.pec"), st.ctrl_pec),
            error_code: self.bind(&format!("{prefix}.ctrl.ec"), st.ctrl_ec),
            rest: vec![self.bind(&format!("{prefix}.ctrl.rest.0"), 0)],
        };
        let h_rest = BScalarHelperRestLanes {
            origin_reference_cursor: self
                .bind(&format!("{prefix}.h.rest.0"), st.helper.origin_ref_cursor),
            parent_resume_present: self.bind(&format!("{prefix}.h.rest.1"), 0),
            parent_resume_value: self.bind(&format!("{prefix}.h.rest.2"), 0),
            expected_present: self.bind(&format!("{prefix}.h.rest.3"), 0),
            expected_value: self.bind(&format!("{prefix}.h.rest.4"), 0),
        };
        let helper = BScalarArithmeticRegister {
            present: self.bind(&format!("{prefix}.h.pres"), st.helper.present),
            kind: self.bind(&format!("{prefix}.h.kind"), st.helper.kind),
            site: self.bind(&format!("{prefix}.h.site"), st.helper.site),
            mode: self.bind(&format!("{prefix}.h.mode"), st.helper.mode),
            phase: self.bind(&format!("{prefix}.h.phase"), st.helper.phase),
            operand_a: BValueRef {
                space: self.bind(&format!("{prefix}.h.oa.sp"), st.helper.op_a_space),
                index: self.bind(&format!("{prefix}.h.oa.ix"), st.helper.op_a_index),
            },
            operand_b: BOptValueRef {
                present: self.bind(&format!("{prefix}.h.ob.pr"), st.helper.op_b_present),
                value: BValueRef {
                    space: self.bind(&format!("{prefix}.h.ob.sp"), st.helper.op_b_space),
                    index: self.bind(&format!("{prefix}.h.ob.ix"), st.helper.op_b_index),
                },
            },
            reg_a: self.bind_u128(&format!("{prefix}.h.reg_a"), st.helper.reg_a),
            staged: BScalarStagedLanes {
                tag: self.bind(&format!("{prefix}.h.st.tag"), st.helper.staged_tag),
                u128_value: self.bind_u128(&format!("{prefix}.h.st.u128"), st.helper.staged_u128),
                bool_value: self.bind(&format!("{prefix}.h.st.bool"), 0),
                pending_node_tag: self.bind(&format!("{prefix}.h.st.pnt"), 0),
                prim_error_code: self.bind(&format!("{prefix}.h.st.pec"), st.helper.staged_err),
                selector: self.bind(&format!("{prefix}.h.st.sel"), 0),
                type_test_ok: self.bind(&format!("{prefix}.h.st.tto"), 0),
            },
            rest: h_rest.to_rest_vec(),
        };
        let result = BHelperResultLanes {
            present: self.bind(&format!("{prefix}.res.pres"), st.res_present),
            tag: self.bind(&format!("{prefix}.res.tag"), st.res_tag),
            scalar_value: BValueRef {
                space: self.bind(&format!("{prefix}.res.sv.sp"), st.res_val_space),
                index: self.bind(&format!("{prefix}.res.sv.ix"), st.res_val_index),
            },
            scalar_error_code: self.bind(&format!("{prefix}.res.err"), st.res_err),
            scalar_selector: self.bind(&format!("{prefix}.res.sel"), 0),
            scalar_type_test_ok: self.bind(&format!("{prefix}.res.tto"), 0),
            rest: vec![self.bind(&format!("{prefix}.res.rest.0"), 0)],
        };
        let data = BDataTableLanes {
            capacity: st.data_rows.len(),
            active_rows: self.bind(&format!("{prefix}.data.act"), st.data_active),
            rows: st
                .data_rows
                .iter()
                .enumerate()
                .map(|(r, row)| BDataRowLanes {
                    tag: self.bind(&format!("{prefix}.data.r{r}.tag"), row.tag),
                    n: self.bind_u128(&format!("{prefix}.data.r{r}.n"), row.n),
                    bool_b: self.bind(&format!("{prefix}.data.r{r}.bool"), row.bool_b),
                    fields_start: self.bind(&format!("{prefix}.data.r{r}.fs"), row.f_start),
                    fields_len: self.bind(&format!("{prefix}.data.r{r}.fl"), row.f_len),
                    rest: vec![self.bind(&format!("{prefix}.data.r{r}.rest.0"), row.rest[0])],
                })
                .collect(),
        };
        let frames = st
            .frames
            .iter()
            .enumerate()
            .map(|(i, f)| BFrameRow {
                tag: self.bind(&format!("{prefix}.f{i}.tag"), f.tag),
                origin: self.bind(&format!("{prefix}.f{i}.orig"), f.origin),
                right_op: self.bind(&format!("{prefix}.f{i}.rop"), f.right_op),
                right_left: BValueRef {
                    space: self.bind(&format!("{prefix}.f{i}.rl.sp"), f.right_left_space),
                    index: self.bind(&format!("{prefix}.f{i}.rl.ix"), f.right_left_index),
                },
                rest: vec![self.bind(&format!("{prefix}.f{i}.rest.0"), f.rest[0])],
            })
            .collect();
        let counters = BScalarCompletionCounters {
            work: BChargedCounters {
                remaining_work: self.bind_u128(&format!("{prefix}.cnt.rem"), st.remaining_work),
                consumed_work: self.bind_u128(&format!("{prefix}.cnt.con"), st.consumed_work),
                helper_work: self.bind_u128(&format!("{prefix}.cnt.hw"), st.helper_work),
                helper_visits: self.bind_u128(&format!("{prefix}.cnt.hv"), st.helper_visits),
            },
            reference_cursor: self.bind(&format!("{prefix}.cnt.ref_c"), st.reference_cursor),
            alloc_data_nodes: self.bind(&format!("{prefix}.cnt.alloc_dn"), st.alloc_data_nodes),
            alloc_data_fields: self.bind(&format!("{prefix}.cnt.alloc_df"), 0),
            alloc_environment_cells: self.bind(&format!("{prefix}.cnt.alloc_env"), 0),
            depth: self.bind(&format!("{prefix}.cnt.depth"), st.depth),
            task_live: self.bind(&format!("{prefix}.cnt.task_live"), st.task_live),
            active_data_fields: self.bind(&format!("{prefix}.cnt.act_df"), 0),
            active_env: self.bind(&format!("{prefix}.cnt.act_env"), 0),
            active_value_slots: self.bind(&format!("{prefix}.cnt.act_vs"), 0),
            rest: vec![self.bind(&format!("{prefix}.cnt.rest.0"), 77)],
        };
        let reference_cursor_bytes =
            self.bind_u128(&format!("{prefix}.ref_c_bytes"), st.reference_cursor);

        BAdminPublishState {
            site: self.bind(&format!("{prefix}.site"), st.site),
            outcome,
            control,
            control_origin: self.bind(&format!("{prefix}.ctrl_orig"), st.control_origin),
            helper,
            result,
            data,
            frames,
            counters,
            reference_cursor_bytes,
        }
    }
}

#[test]
fn publish_value_halted_const_and_data_refs_preserves_control_and_succeeds_at_zero_fuel() {
    for (space, index, fuel) in [
        (VALUE_SPACE_CONST, 0, 25),
        (VALUE_SPACE_DATA, 1, 0), // zero remaining_work: P1 fires before P2 at cost 0!
    ] {
        let pre = ConcreteAdminState::base_return_at_depth_zero(space, index, fuel);
        let mut post = pre.clone();
        post.outcome_tag = OUTCOME_TAG_HALTED;
        post.halted_space = space;
        post.halted_index = index;

        let builder = EnvBuilder::new();
        let inputs = BAdminPublishInputs {
            enable: builder.bind("%p1v.enable", 1),
            const_active_rows: builder.bind("%p1v.const_active", 2),
            pre: builder.bind_admin_state("%p1v.pre", &pre),
            post: builder.bind_admin_state("%p1v.post", &post),
        };
        let env = builder.env.into_inner();

        let block_v = emit_k_admin_publish_value(&inputs, "p1_val").unwrap();
        let eval_v = eval_zkir(&block_v.instructions, env.clone()).unwrap();
        assert_eq!(eval_operand(&block_v.publish_value, &eval_v).unwrap(), 1);
        assert_eq!(eval_operand(&block_v.publish_error, &eval_v).unwrap(), 0);
        assert_eq!(
            eval_operand(&block_v.published_value.space, &eval_v).unwrap(),
            space
        );
        assert_eq!(
            eval_operand(&block_v.published_value.index, &eval_v).unwrap(),
            index
        );

        let block_u = emit_b_admin_publish_p1(&inputs, "p1_uni_val").unwrap();
        let eval_u = eval_zkir(&block_u.instructions, env).unwrap();
        assert_eq!(eval_operand(&block_u.publish_value, &eval_u).unwrap(), 1);
        assert_eq!(eval_operand(&block_u.publish_error, &eval_u).unwrap(), 0);
    }
}

#[test]
fn publish_error_all_codes_1_through_11_across_zero_and_nonzero_depths_and_zero_fuel() {
    for code in ERROR_CODE_MIN..=ERROR_CODE_MAX {
        for (depth, fuel) in [(0u128, 10u128), (1u128, 0u128)] {
            let mut pre = ConcreteAdminState::base_return_at_depth_zero(0, 0, fuel);
            pre.ctrl_tag = CONTROL_TAG_ERROR;
            pre.ctrl_rv_space = 0;
            pre.ctrl_rv_index = 0;
            pre.ctrl_ec = code;
            pre.depth = depth;
            if depth == 1 {
                pre.frames[0] = ConcreteFrameSlot::right_f_add(500, VALUE_SPACE_CONST, 0);
            }

            let mut post = pre.clone();
            post.outcome_tag = OUTCOME_TAG_REJECTED;
            post.rejected_code = code;

            let builder = EnvBuilder::new();
            let inputs = BAdminPublishInputs {
                enable: builder.bind("%p1e.enable", 1),
                const_active_rows: builder.bind("%p1e.const_active", 2),
                pre: builder.bind_admin_state("%p1e.pre", &pre),
                post: builder.bind_admin_state("%p1e.post", &post),
            };
            let env = builder.env.into_inner();

            let block_e = emit_k_admin_publish_error(&inputs, "p1_err").unwrap();
            let eval_e = eval_zkir(&block_e.instructions, env.clone()).unwrap();
            assert_eq!(eval_operand(&block_e.publish_value, &eval_e).unwrap(), 0);
            assert_eq!(eval_operand(&block_e.publish_error, &eval_e).unwrap(), 1);
            assert_eq!(
                eval_operand(&block_e.published_error_code, &eval_e).unwrap(),
                code
            );

            let block_u = emit_b_admin_publish_p1(&inputs, "p1_uni_err").unwrap();
            let eval_u = eval_zkir(&block_u.instructions, env).unwrap();
            assert_eq!(eval_operand(&block_u.publish_value, &eval_u).unwrap(), 0);
            assert_eq!(eval_operand(&block_u.publish_error, &eval_u).unwrap(), 1);
        }
    }
}

#[test]
fn security_negative_publish_value_and_error_mutations_are_rejected_with_exact_first_failure() {
    let pre_val = ConcreteAdminState::base_return_at_depth_zero(VALUE_SPACE_DATA, 1, 10);
    let mut post_val = pre_val.clone();
    post_val.outcome_tag = OUTCOME_TAG_HALTED;
    post_val.halted_space = VALUE_SPACE_DATA;
    post_val.halted_index = 1;

    let builder = EnvBuilder::new();
    let val_inputs = BAdminPublishInputs {
        enable: builder.bind("%negv.enable", 1),
        const_active_rows: builder.bind("%negv.const_active", 2),
        pre: builder.bind_admin_state("%negv.pre", &pre_val),
        post: builder.bind_admin_state("%negv.post", &post_val),
    };
    let val_env = builder.env.into_inner();
    let val_block = emit_k_admin_publish_value(&val_inputs, "neg_val").unwrap();

    // 1. Old L4 convention clearing post.control.tag to 0 must be rejected.
    let mut cleared_ctrl = val_env.clone();
    cleared_ctrl.insert("%negv.post.ctrl.tag".into(), 0);
    assert!(
        eval_zkir(&val_block.instructions, cleared_ctrl)
            .unwrap_err()
            .contains("ConstrainEq failed"),
        "clearing post.control.tag must fail ConstrainEq"
    );

    // 2. Return with depth > 0 must be rejected by k.admin.publish.value.
    let mut nonzero_depth = val_env.clone();
    nonzero_depth.insert("%negv.pre.cnt.depth".into(), 1);
    nonzero_depth.insert("%negv.post.cnt.depth".into(), 1);
    assert!(
        eval_zkir(&val_block.instructions, nonzero_depth)
            .unwrap_err()
            .contains("ConstrainEq failed (0 != 1)"),
        "depth > 0 on publish.value must fail branch enable ConstrainEq"
    );

    // 3. Out-of-bounds ValueRef index (index = 2 == active.data) must fail SI-13 Assert.
    let mut oob_ref = val_env.clone();
    oob_ref.insert("%negv.pre.ctrl.rv.ix".into(), 2);
    oob_ref.insert("%negv.post.ctrl.rv.ix".into(), 2);
    oob_ref.insert("%negv.post.out.hv.ix".into(), 2);
    assert!(
        eval_zkir(&val_block.instructions, oob_ref)
            .unwrap_err()
            .contains("Assert failed (cond = 0)"),
        "out-of-bounds return_value.index must fail Assert"
    );

    // 4. Charging fuel (decrementing remaining_work) or mutating reference_cursor on P1 must fail.
    let mut charged_fuel = val_env.clone();
    charged_fuel.insert("%negv.post.cnt.rem.0".into(), 9);
    assert!(eval_zkir(&val_block.instructions, charged_fuel).is_err());

    let mut desynced_ref_cursor = val_env.clone();
    desynced_ref_cursor.insert("%negv.pre.ref_c_bytes.0".into(), 9);
    assert!(eval_zkir(&val_block.instructions, desynced_ref_cursor).is_err());

    // 5. ErrorCode domain: 0, 12, and 255 (old A82 u8 domain) must be rejected on publish.error.
    let mut pre_err = ConcreteAdminState::base_return_at_depth_zero(0, 0, 10);
    pre_err.ctrl_tag = CONTROL_TAG_ERROR;
    pre_err.ctrl_ec = 10;
    let mut post_err = pre_err.clone();
    post_err.outcome_tag = OUTCOME_TAG_REJECTED;
    post_err.rejected_code = 10;

    let err_builder = EnvBuilder::new();
    let err_inputs = BAdminPublishInputs {
        enable: err_builder.bind("%nege.enable", 1),
        const_active_rows: err_builder.bind("%nege.const_active", 2),
        pre: err_builder.bind_admin_state("%nege.pre", &pre_err),
        post: err_builder.bind_admin_state("%nege.post", &post_err),
    };
    let err_env = err_builder.env.into_inner();
    let err_block = emit_k_admin_publish_error(&err_inputs, "neg_err").unwrap();

    for bad_code in [0u128, 12u128, 255u128] {
        let mut mutated = err_env.clone();
        mutated.insert("%nege.pre.ctrl.ec".into(), bad_code);
        mutated.insert("%nege.post.ctrl.ec".into(), bad_code);
        mutated.insert("%nege.post.out.rej".into(), bad_code);
        assert!(
            eval_zkir(&err_block.instructions, mutated).is_err(),
            "invalid ErrorCode {bad_code} must be rejected"
        );
    }
}

struct ChainScenario {
    lhs_u128: u128,
    rhs_slot: ConcreteDataSlot,
    bounds_data_nodes: u128,
    initial_remaining_work: u128,
}

fn build_chain_fixture(
    scenario: &ChainScenario,
) -> (BAddToPublicationChainInputs, BTreeMap<String, u128>) {
    let is_rhs_uint = scenario.rhs_slot.tag == DATA_TAG_U128;
    let (sum_u128, overflow) = scenario.lhs_u128.overflowing_add(scenario.rhs_slot.n);
    let is_ok_u128 = is_rhs_uint && !overflow;
    let err_code = if !is_rhs_uint {
        PRIM_ERROR_SORT_MISMATCH
    } else if overflow {
        PRIM_ERROR_ARITH_OVERFLOW
    } else {
        0
    };
    let pre_alloc_dn = 1u128;
    let cap_refused = is_ok_u128 && (pre_alloc_dn >= scenario.bounds_data_nodes);
    let applied = is_ok_u128 && !cap_refused;

    // Stage 0: RetRight / Add / Second with operandA = Const(0) (lhs_u128), operandB = Data(0) (rhs_slot).
    let top_frame = ConcreteFrameSlot::right_f_add(808, VALUE_SPACE_CONST, 0);
    let state0 = ConcreteAdminState {
        site: SITE_RET_RIGHT,
        outcome_tag: OUTCOME_TAG_RUNNING,
        halted_space: 0,
        halted_index: 0,
        rejected_code: 0,
        exhausted_reason: 0,
        ctrl_tag: CONTROL_TAG_RETURN,
        ctrl_rv_space: VALUE_SPACE_DATA,
        ctrl_rv_index: 0,
        ctrl_pot: 0,
        ctrl_pv_space: 0,
        ctrl_pv_index: 0,
        ctrl_pec: 0,
        ctrl_ec: 0,
        control_origin: 404,
        helper: ConcreteHelperState {
            present: HELPER_PRESENT,
            kind: HELPER_KIND_SCALAR,
            site: SITE_RET_RIGHT,
            mode: SCALAR_MODE_ADD,
            phase: SCALAR_PHASE_SECOND,
            op_a_space: VALUE_SPACE_CONST,
            op_a_index: 0,
            op_b_present: OPT_PRESENT,
            op_b_space: VALUE_SPACE_DATA,
            op_b_index: 0,
            reg_a: scenario.lhs_u128,
            staged_tag: SCALAR_STAGED_TAG_NONE,
            staged_u128: 0,
            staged_err: 0,
            origin_ref_cursor: 5,
        },
        res_present: 0,
        res_tag: 0,
        res_val_space: 0,
        res_val_index: 0,
        res_err: 0,
        data_active: 1,
        data_rows: vec![scenario.rhs_slot.clone(), ConcreteDataSlot::zero()],
        frames: vec![top_frame],
        depth: 1,
        remaining_work: scenario.initial_remaining_work,
        consumed_work: 10,
        helper_work: 2,
        helper_visits: 1,
        reference_cursor: 5,
        alloc_data_nodes: pre_alloc_dn,
        task_live: 0,
    };

    // Stage 1: after b.helper.Scalar.arithmetic (phase = Done, staged = U128Value or PrimError).
    let mut state1 = state0.clone();
    state1.helper.phase = SCALAR_PHASE_DONE;
    if is_ok_u128 {
        state1.helper.staged_tag = SCALAR_STAGED_TAG_U128_VALUE;
        state1.helper.staged_u128 = sum_u128;
    } else {
        state1.helper.staged_tag = SCALAR_STAGED_TAG_PRIM_ERROR;
        state1.helper.staged_err = err_code;
    }
    state1.remaining_work -= 1;
    state1.consumed_work += 1;
    state1.helper_work += 1;
    state1.helper_visits += 1;

    // Stage 2: after b.helper.Scalar.exit or b.helper.Scalar.failure.
    let mut state2 = state1.clone();
    state2.remaining_work -= 1;
    state2.consumed_work += 1;
    state2.helper_work += 1;
    if cap_refused {
        state2.outcome_tag = OUTCOME_TAG_EXHAUSTED;
        state2.exhausted_reason = EXHAUST_REASON_DATA_NODES;
    } else if applied {
        state2.helper = ConcreteHelperState::absent();
        state2.data_active = 2;
        state2.data_rows[1] = ConcreteDataSlot::u128_row(sum_u128);
        state2.alloc_data_nodes = pre_alloc_dn + 1;
        state2.res_present = OPT_PRESENT;
        state2.res_tag = HELPER_RESULT_TAG_SCALAR_VALUE;
        state2.res_val_space = VALUE_SPACE_DATA;
        state2.res_val_index = 1;
    } else {
        state2.helper = ConcreteHelperState::absent();
        state2.res_present = OPT_PRESENT;
        state2.res_tag = HELPER_RESULT_TAG_SCALAR_ERROR;
        state2.res_err = err_code;
    }

    // Stage 3: after k.machine.return.right (or stutter if cap_refused).
    let mut state3 = state2.clone();
    if !cap_refused {
        state3.site = SITE_PRIM;
        state3.frames[0] = ConcreteFrameSlot::zero();
        state3.depth = 0;
        state3.control_origin = 808;
        state3.res_present = 0;
        state3.res_tag = 0;
        state3.res_val_space = 0;
        state3.res_val_index = 0;
        state3.res_err = 0;
        state3.ctrl_tag = CONTROL_TAG_PRIM_RESULT;
        state3.ctrl_rv_space = 0;
        state3.ctrl_rv_index = 0;
        if applied {
            state3.ctrl_pot = PRIM_OUTCOME_TAG_VALUE;
            state3.ctrl_pv_space = VALUE_SPACE_DATA;
            state3.ctrl_pv_index = 1;
        } else {
            state3.ctrl_pot = PRIM_OUTCOME_TAG_ERROR;
            state3.ctrl_pec = err_code;
        }
        state3.remaining_work -= 1;
        state3.consumed_work += 1;
        state3.reference_cursor += 1;
    }

    // Stage 4: after k.machine.primitive.value or k.machine.primitive.error (or stutter if cap_refused).
    let mut state4 = state3.clone();
    let mut state4_val_branch = state3.clone();
    let mut state4_err_branch = state3.clone();
    if applied {
        state4.ctrl_tag = CONTROL_TAG_RETURN;
        state4.ctrl_rv_space = VALUE_SPACE_DATA;
        state4.ctrl_rv_index = 1;
        state4.ctrl_pot = 0;
        state4.ctrl_pv_space = 0;
        state4.ctrl_pv_index = 0;
        state4.remaining_work -= 1;
        state4.consumed_work += 1;
        state4.reference_cursor += 1;
        state4_val_branch = state4.clone();
    } else if !cap_refused {
        state4.ctrl_tag = CONTROL_TAG_ERROR;
        state4.ctrl_pot = 0;
        state4.ctrl_pec = 0;
        state4.ctrl_ec = err_code;
        state4.remaining_work -= 1;
        state4.consumed_work += 1;
        state4.reference_cursor += 1;
        state4_err_branch = state4.clone();
    }

    // Stage 5: after P1 k.admin.publish.value or k.admin.publish.error (0 charge; or stutter if cap_refused).
    let mut state5 = state4.clone();
    if applied {
        state5.outcome_tag = OUTCOME_TAG_HALTED;
        state5.halted_space = VALUE_SPACE_DATA;
        state5.halted_index = 1;
    } else if !cap_refused {
        state5.outcome_tag = OUTCOME_TAG_REJECTED;
        state5.rejected_code = err_code;
    }

    let builder = EnvBuilder::new();
    let constants = BValueTableRead {
        capacity: 1,
        active_rows: builder.bind("%ch.const.act", 1),
        rows: vec![BValueRowLanes {
            tag: builder.bind("%ch.const.r0.tag", CONST_TAG_U128),
            n: builder.bind_u128("%ch.const.r0.n", scenario.lhs_u128),
        }],
    };
    let s0 = builder.bind_admin_state("%ch.s0", &state0);
    let s1 = builder.bind_admin_state("%ch.s1", &state1);
    let s2 = builder.bind_admin_state("%ch.s2", &state2);
    // Stage-2 per-branch posts: the enabled row's post is `state2`, the disabled row's is `state1`.
    let (exit_post, fail_post) = if is_ok_u128 { (&state2, &state1) } else { (&state1, &state2) };
    let s2x = builder.bind_admin_state("%ch.s2x", exit_post);
    let s2f = builder.bind_admin_state("%ch.s2f", fail_post);
    let s3 = builder.bind_admin_state("%ch.s3", &state3);
    let pv_admin = builder.bind_admin_state("%ch.pv", &state4_val_branch);
    let pe_admin = builder.bind_admin_state("%ch.pe", &state4_err_branch);
    let s4 = builder.bind_admin_state("%ch.s4", &state4);
    let s5 = builder.bind_admin_state("%ch.s5", &state5);

    let to_leg = |st: &BAdminPublishState| -> BLegacyCompletionState {
        let mut rest = vec![
            st.counters.alloc_data_nodes.clone(),
            st.counters.alloc_data_fields.clone(),
            st.counters.alloc_environment_cells.clone(),
            st.counters.task_live.clone(),
            st.counters.active_data_fields.clone(),
            st.counters.active_env.clone(),
            st.counters.active_value_slots.clone(),
        ];
        rest.extend(st.counters.rest.iter().cloned());
        BLegacyCompletionState {
            site: st.site.clone(),
            outcome: st.outcome.tag.clone(),
            control: st.control.clone(),
            control_origin: st.control_origin.clone(),
            helper: st.helper.clone(),
            result: st.result.clone(),
            frames: st.frames.clone(),
            depth: st.counters.depth.clone(),
            remaining_work: st.counters.work.remaining_work.clone(),
            consumed_work: st.counters.work.consumed_work.clone(),
            helper_work: st.counters.work.helper_work.clone(),
            helper_visits: st.counters.work.helper_visits.clone(),
            reference_cursor: st.reference_cursor_bytes.clone(),
            rest,
        }
    };

    let inputs = BAddToPublicationChainInputs {
        enable: builder.bind("%ch.enable", 1),
        bounds_data_nodes: builder.bind("%ch.bounds.dn", scenario.bounds_data_nodes),
        constants,
        state0: s0,
        state1: s1,
        state2: s2,
        s2_exit_post: s2x,
        s2_fail_post: s2f,
        state3: s3,
        prim_val_post: to_leg(&pv_admin),
        prim_err_post: to_leg(&pe_admin),
        state4: s4,
        state5: s5,
    };
    (inputs, builder.env.into_inner())
}

#[test]
fn chain_add_to_publish_value_positively_executes_all_five_stages() {
    // 7 + 9 = 16 with initial_remaining_work = 4 so Stage 5 (P1) executes at 0 remaining fuel.
    let scenario = ChainScenario {
        lhs_u128: 7,
        rhs_slot: ConcreteDataSlot::u128_row(9),
        bounds_data_nodes: 5,
        initial_remaining_work: 4,
    };
    let (inputs, env) = build_chain_fixture(&scenario);
    let chain = emit_b_add_to_publication_chain(&inputs, "add_val_chain").unwrap();
    let eval = eval_zkir(&chain.instructions, env).unwrap();

    assert_eq!(eval_operand(&chain.exit_enable, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.failure_enable, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.exit_block.applied, &eval).unwrap(), 1);
    assert_eq!(
        eval_operand(&chain.exit_block.cap_refused, &eval).unwrap(),
        0
    );
    assert_eq!(eval_operand(&chain.legacy_enable, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.prim_value_enable, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.prim_error_enable, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.p1_enable, &eval).unwrap(), 1);
    assert_eq!(
        eval_operand(&chain.p1_block.publish_value, &eval).unwrap(),
        1
    );
    assert_eq!(
        eval_operand(&chain.p1_block.published_value.space, &eval).unwrap(),
        VALUE_SPACE_DATA
    );
    assert_eq!(
        eval_operand(&chain.p1_block.published_value.index, &eval).unwrap(),
        1
    );
}

#[test]
fn chain_add_to_publish_error_10_overflow_and_11_sort_mismatch_reaches_rejected() {
    let scenarios = [
        (
            ChainScenario {
                lhs_u128: u128::MAX,
                rhs_slot: ConcreteDataSlot::u128_row(1),
                bounds_data_nodes: 5,
                initial_remaining_work: 4,
            },
            PRIM_ERROR_ARITH_OVERFLOW,
        ),
        (
            ChainScenario {
                lhs_u128: 7,
                rhs_slot: ConcreteDataSlot::bool_row(1),
                bounds_data_nodes: 5,
                initial_remaining_work: 4,
            },
            PRIM_ERROR_SORT_MISMATCH,
        ),
    ];

    for (scenario, expected_code) in scenarios {
        let (inputs, env) = build_chain_fixture(&scenario);
        let chain = emit_b_add_to_publication_chain(&inputs, "add_err_chain").unwrap();
        let eval = eval_zkir(&chain.instructions, env).unwrap();

        assert_eq!(eval_operand(&chain.exit_enable, &eval).unwrap(), 0);
        assert_eq!(eval_operand(&chain.failure_enable, &eval).unwrap(), 1);
        assert_eq!(eval_operand(&chain.legacy_enable, &eval).unwrap(), 1);
        assert_eq!(eval_operand(&chain.prim_value_enable, &eval).unwrap(), 0);
        assert_eq!(eval_operand(&chain.prim_error_enable, &eval).unwrap(), 1);
        assert_eq!(eval_operand(&chain.p1_enable, &eval).unwrap(), 1);
        assert_eq!(
            eval_operand(&chain.p1_block.publish_error, &eval).unwrap(),
            1
        );
        assert_eq!(
            eval_operand(&chain.p1_block.published_error_code, &eval).unwrap(),
            expected_code
        );
    }
}

#[test]
fn chain_add_logical_data_nodes_cap_refusal_stops_at_exhausted_without_legacy_or_p1() {
    // pre_alloc_dn == 1 == bounds_data_nodes: Stage 2 refuses at P6 DataNodes and stops at Exhausted.
    let scenario = ChainScenario {
        lhs_u128: 7,
        rhs_slot: ConcreteDataSlot::u128_row(9),
        bounds_data_nodes: 1,
        initial_remaining_work: 4,
    };
    let (inputs, env) = build_chain_fixture(&scenario);
    let chain = emit_b_add_to_publication_chain(&inputs, "add_cap_chain").unwrap();
    let eval = eval_zkir(&chain.instructions, env).unwrap();

    assert_eq!(eval_operand(&chain.exit_enable, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.failure_enable, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.exit_block.applied, &eval).unwrap(), 0);
    assert_eq!(
        eval_operand(&chain.exit_block.cap_refused, &eval).unwrap(),
        1
    );
    assert_eq!(eval_operand(&chain.legacy_enable, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.prim_value_enable, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.prim_error_enable, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.p1_enable, &eval).unwrap(), 0);
    // The final state is the charged `Exhausted(DataNodes)` refusal; a forged `Halted` fails
    // first at disabled P1's outcome stutter.
    assert_eq!(eval["%ch.s5.out.tag"], OUTCOME_TAG_EXHAUSTED);
    assert_eq!(eval["%ch.s5.out.exh"], EXHAUST_REASON_DATA_NODES);
    assert_eq!(eval["%ch.s5.cnt.rem.0"], 2);
    let (_, mut forged) = build_chain_fixture(&scenario);
    forged.insert("%ch.s5.out.tag".into(), OUTCOME_TAG_HALTED);
    let err = eval_zkir(&chain.instructions, forged).unwrap_err();
    let expected = format!("ConstrainEq failed ({OUTCOME_TAG_HALTED} != {OUTCOME_TAG_EXHAUSTED})");
    assert!(err.contains(&expected), "{err}");
}

#[test]
fn security_negative_chain_handoff_and_stage_mutations_are_rejected() {
    let scenario = ChainScenario {
        lhs_u128: 7,
        rhs_slot: ConcreteDataSlot::u128_row(9),
        bounds_data_nodes: 5,
        initial_remaining_work: 6,
    };
    let (inputs, base_env) = build_chain_fixture(&scenario);
    let chain = emit_b_add_to_publication_chain(&inputs, "add_neg_chain").unwrap();

    let assert_rejects = |mutations: &[(&str, u128)], label: &str| {
        let mut mutated = base_env.clone();
        for (k, v) in mutations {
            mutated.insert((*k).to_string(), *v);
        }
        assert!(
            eval_zkir(&chain.instructions, mutated).is_err(),
            "expected chain rejection for {label}"
        );
    };

    // 1. Mutating Stage 1 staged sum byte 0 (from 16 to 17) must fail Stage 1 arithmetic check.
    assert_rejects(&[("%ch.s1.h.st.u128.0", 17)], "corrupted Stage 1 staged sum");

    // 2. Mutating Stage 2 allocated DataRow[1] payload (from 16 to 17) must fail Stage 2 exit check.
    assert_rejects(
        &[("%ch.s2x.data.r1.n.0", 17)],
        "corrupted Stage 2 appended DataRow",
    );

    // 3. Mutating Stage 3 popped control_origin (from 808 to 404) must fail Stage 3 ReturnRight check.
    assert_rejects(
        &[("%ch.s3.ctrl_orig", 404)],
        "corrupted Stage 3 control_origin handoff",
    );

    // 4. Mutating Stage 4 muxed Return value index (from 1 to 0) must fail Stage 4 mux check.
    assert_rejects(
        &[("%ch.s4.ctrl.rv.ix", 0)],
        "corrupted Stage 4 Return value index",
    );

    // 5. Mutating Stage 5 final Halted value index (from 1 to 0) must fail Stage 5 P1 check.
    assert_rejects(
        &[("%ch.s5.out.hv.ix", 0)],
        "corrupted Stage 5 Halted value index",
    );
}

#[test]
fn admin_publication_constructors_reject_invalid_scopes_shapes_and_temp_collisions() {
    let pre = ConcreteAdminState::base_return_at_depth_zero(VALUE_SPACE_DATA, 1, 10);
    let mut post = pre.clone();
    post.outcome_tag = OUTCOME_TAG_HALTED;
    post.halted_space = VALUE_SPACE_DATA;
    post.halted_index = 1;

    let builder = EnvBuilder::new();
    let base = BAdminPublishInputs {
        enable: builder.bind("%ctor.enable", 1),
        const_active_rows: builder.bind("%ctor.const_active", 2),
        pre: builder.bind_admin_state("%ctor.pre", &pre),
        post: builder.bind_admin_state("%ctor.post", &post),
    };

    assert_eq!(
        emit_k_admin_publish_value(&base, ""),
        Err(BAdminPublicationError::Fetch(ConstructionError::EmptyScope))
    );

    let mut empty_frames = base.clone();
    empty_frames.pre.frames.clear();
    // F4: capacity 0 is admitted; only a pre/post capacity mismatch is refused.
    assert_eq!(
        emit_k_admin_publish_error(&empty_frames, "s"),
        Err(BAdminPublicationError::FrameCapacityMismatch {
            pre_len: 0,
            post_len: 2
        })
    );

    let mut bad_data = base.clone();
    bad_data.pre.data.capacity = 99;
    assert_eq!(
        emit_b_admin_publish_p1(&bad_data, "s"),
        Err(BAdminPublicationError::DataRowCount)
    );

    let mut unprefixed = base.clone();
    unprefixed.const_active_rows = Operand::Variable(Identifier("bad_var".into()));
    assert_eq!(
        emit_b_admin_publish_p1(&unprefixed, "s"),
        Err(BAdminPublicationError::Fetch(
            ConstructionError::UnprefixedVariable
        ))
    );

    let mut colliding = base;
    colliding.post.site = Operand::Variable(Identifier("%s.local.temp.0".into()));
    assert_eq!(
        emit_b_admin_publish_p1(&colliding, "s"),
        Err(BAdminPublicationError::Fetch(
            ConstructionError::TemporaryNameCollision
        ))
    );
}

#[test]
fn p1_admits_zero_frame_capacity_only_at_depth_zero() {
    let mut pre_v = ConcreteAdminState::base_return_at_depth_zero(VALUE_SPACE_DATA, 1, 0);
    pre_v.frames.clear();
    let mut post_v = pre_v.clone();
    post_v.outcome_tag = OUTCOME_TAG_HALTED;
    post_v.halted_space = VALUE_SPACE_DATA;
    post_v.halted_index = 1;
    let mut pre_e = pre_v.clone();
    pre_e.ctrl_tag = CONTROL_TAG_ERROR;
    pre_e.ctrl_rv_space = 0;
    pre_e.ctrl_rv_index = 0;
    pre_e.ctrl_ec = PRIM_ERROR_SORT_MISMATCH;
    let mut post_e = pre_e.clone();
    post_e.outcome_tag = OUTCOME_TAG_REJECTED;
    post_e.rejected_code = PRIM_ERROR_SORT_MISMATCH;
    for (t, pre, post) in [("v", pre_v, post_v), ("e", pre_e, post_e)] {
        let builder = EnvBuilder::new();
        let inputs = BAdminPublishInputs {
            enable: builder.bind(&format!("%z{t}.en"), 1),
            const_active_rows: builder.bind(&format!("%z{t}.ca"), 2),
            pre: builder.bind_admin_state(&format!("%z{t}.pre"), &pre),
            post: builder.bind_admin_state(&format!("%z{t}.post"), &post),
        };
        let block = emit_b_admin_publish_p1(&inputs, "p1_zero").unwrap();
        let env = builder.env.into_inner();
        eval_zkir(&block.instructions, env.clone()).expect("cap-0 P1 at depth 0 must pass");
        let mut deep = env;
        deep.insert(format!("%z{t}.pre.cnt.depth"), 1);
        deep.insert(format!("%z{t}.post.cnt.depth"), 1);
        assert!(eval_zkir(&block.instructions, deep).is_err(), "cap-0 P1 at depth 1 must fail");
    }
}

#[test]
fn chain_branch_posts_site_dispatch_and_scope_boundary_are_enforced() {
    let scenario = ChainScenario {
        lhs_u128: 7,
        rhs_slot: ConcreteDataSlot::u128_row(9),
        bounds_data_nodes: 5,
        initial_remaining_work: 4,
    };
    let (inputs, env) = build_chain_fixture(&scenario);
    let chain = emit_b_add_to_publication_chain(&inputs, "add_hand").unwrap();
    let eval = eval_zkir(&chain.instructions, env.clone()).unwrap();
    assert_eq!(eval_operand(&chain.return_dispatch, &eval).unwrap(), 0);
    let run = |key: &str, val: u128| {
        let mut mutated = env.clone();
        mutated.insert(key.to_string(), val);
        eval_zkir(&chain.instructions, mutated)
    };
    // Depth-0 Return selects P1 (no SiteTag): a forged RetLet site is the first failure.
    let err = run("%ch.pv.site", 9).unwrap_err();
    assert!(err.contains("ConstrainEq failed (9 != 23)"), "{err}");
    // Disabled sibling posts stay pinned to their own pre-states.
    assert!(run("%ch.s2f.res.tag", 1).is_err(), "disabled failure post must stutter");
    assert!(run("%ch.pe.site", 9).is_err(), "disabled primitive.error post must stutter");

    for (name, expected) in [
        ("%add_hand_s4_pv__not_enable_0", ConstructionError::TemporaryNameCollision),
        ("%add_hand_s3_rr__not_enable_0", ConstructionError::TemporaryNameCollision),
        ("add_hand_s4_pv__not_enable_0", ConstructionError::UnprefixedVariable),
        ("%add_hand_chain.local.temp.0", ConstructionError::TemporaryNameCollision),
    ] {
        let mut colliding = inputs.clone();
        colliding.prim_val_post.site = Operand::Variable(Identifier(name.into()));
        assert_eq!(
            emit_b_add_to_publication_chain(&colliding, "add_hand").err(),
            Some(BAdminPublicationError::Fetch(expected))
        );
    }
}

#[test]
fn chain_disabled_stutters_all_stages_and_rejects_forged_stage1_work() {
    let scenario = ChainScenario {
        lhs_u128: 7,
        rhs_slot: ConcreteDataSlot::u128_row(9),
        bounds_data_nodes: 5,
        initial_remaining_work: 4,
    };
    let (inputs, mut env) = build_chain_fixture(&scenario);
    // `enable == 0` with every stage and branch post bound to `state0`'s values.
    env.insert("%ch.enable".into(), 0);
    let s0: Vec<(String, u128)> = env
        .iter()
        .filter_map(|(k, v)| k.strip_prefix("%ch.s0.").map(|s| (s.to_string(), *v)))
        .collect();
    for st in ["s1", "s2", "s2x", "s2f", "s3", "pv", "pe", "s4", "s5"] {
        for (suffix, v) in &s0 {
            env.insert(format!("%ch.{st}.{suffix}"), *v);
        }
    }
    let chain = emit_b_add_to_publication_chain(&inputs, "add_off").unwrap();
    let eval = eval_zkir(&chain.instructions, env.clone()).expect("disabled stutter chain must pass");
    for gate in [
        &chain.exit_enable,
        &chain.failure_enable,
        &chain.legacy_enable,
        &chain.prim_value_enable,
        &chain.prim_error_enable,
        &chain.p1_enable,
    ] {
        assert_eq!(eval_operand(gate, &eval).unwrap(), 0);
    }
    // M-1: only `state1.remainingWork` byte 0 is forged; the stage-1 stutter is the first failure.
    let mut forged = env;
    forged.insert("%ch.s1.cnt.rem.0".into(), 99);
    let err = eval_zkir(&chain.instructions, forged).unwrap_err();
    assert!(err.contains("ConstrainEq failed (99 != 4)"), "{err}");
}

fn chain_ctor_err(
    inputs: &BAddToPublicationChainInputs,
    mutate: impl Fn(&mut BAddToPublicationChainInputs),
) -> Option<BAdminPublicationError> {
    let mut colliding = inputs.clone();
    mutate(&mut colliding);
    emit_b_add_to_publication_chain(&colliding, "ns").err()
}

#[test]
fn chain_rejects_caller_operands_in_every_derived_temporary_namespace() {
    let scenario = ChainScenario {
        lhs_u128: 7,
        rhs_slot: ConcreteDataSlot::u128_row(9),
        bounds_data_nodes: 5,
        initial_remaining_work: 4,
    };
    let (inputs, env) = build_chain_fixture(&scenario);
    let chain = emit_b_add_to_publication_chain(&inputs, "ns").unwrap();
    eval_zkir(&chain.instructions, env).expect("clean baseline must pass");
    let mut defined = std::collections::BTreeSet::new();
    for inst in &chain.instructions {
        match inst {
            Instruction::CondSelect { output, .. }
            | Instruction::TestEq { output, .. }
            | Instruction::LessThan { output, .. }
            | Instruction::Add { output, .. }
            | Instruction::Mul { output, .. } => {
                defined.insert(output.0.clone());
            }
            Instruction::DivModPowerOfTwo { outputs, .. } => {
                defined.extend(outputs.iter().map(|o| o.0.clone()));
            }
            _ => {}
        }
    }
    let var = |name: &str| Operand::Variable(Identifier(name.into()));
    let collision = Some(BAdminPublicationError::Fetch(ConstructionError::TemporaryNameCollision));
    let unprefixed = Some(BAdminPublicationError::Fetch(ConstructionError::UnprefixedVariable));
    let (chain_t, arith_t, const_t, data_t, exit_t, fail_t, p1_t) = (
        "%ns_chain.local.temp.0",
        "%ns_s1_arith.local.temp.0",
        "%ns_s1_arith.const_fetch.temp.0",
        "%ns_s1_arith.data_fetch.temp.0",
        "%ns_s2_exit.local.temp.0",
        "%ns_s2_fail.local.temp.0",
        "%ns_s5_p1.local.temp.0",
    );
    // Each name is an emitted producer output; most target a field that producer never receives.
    for name in [chain_t, arith_t, const_t, data_t, exit_t, fail_t, p1_t] {
        assert!(defined.contains(name), "{name} must be an emitted temporary");
    }
    assert_eq!(chain_ctor_err(&inputs, |c| c.state0.site = var(chain_t)), collision);
    assert_eq!(chain_ctor_err(&inputs, |c| c.state4.site = var(arith_t)), collision);
    assert_eq!(chain_ctor_err(&inputs, |c| c.state5.control_origin = var(const_t)), collision);
    assert_eq!(chain_ctor_err(&inputs, |c| c.constants.rows[0].n[3] = var(data_t)), collision);
    assert_eq!(chain_ctor_err(&inputs, |c| c.state0.helper.phase = var(exit_t)), collision);
    assert_eq!(chain_ctor_err(&inputs, |c| c.prim_err_post.site = var(exit_t)), collision);
    assert_eq!(chain_ctor_err(&inputs, |c| c.state4.site = var(fail_t)), collision);
    assert_eq!(chain_ctor_err(&inputs, |c| c.constants.active_rows = var(fail_t)), collision);
    assert_eq!(chain_ctor_err(&inputs, |c| c.constants.rows[0].tag = var(p1_t)), collision);
    assert_eq!(chain_ctor_err(&inputs, |c| c.state1.helper.phase = var(p1_t)), collision);
    assert_eq!(chain_ctor_err(&inputs, |c| c.bounds_data_nodes = var(p1_t)), collision);
    // Actual legacy RR/primitive temporaries are `%ns_{stage}__*`; the chain reserves them in every
    // field, including fields the legacy row never receives; the unprefixed spelling stays refused.
    for stage in ["s3_rr", "s4_pv", "s4_pe"] {
        let prefix = format!("%ns_{stage}__");
        let name = defined.iter().find(|n| n.starts_with(&prefix)).expect("legacy temporary").clone();
        assert!(is_legacy_completion_scope_variable(&format!("ns_{stage}"), &name), "{name}");
        let name = name.as_str();
        assert_eq!(chain_ctor_err(&inputs, |c| c.state3.site = var(name)), collision);
        assert_eq!(chain_ctor_err(&inputs, |c| c.prim_err_post.rest[0] = var(name)), collision);
        assert_eq!(chain_ctor_err(&inputs, |c| c.state0.helper.phase = var(name)), collision);
        assert_eq!(chain_ctor_err(&inputs, |c| c.constants.rows[0].n[0] = var(name)), collision);
        assert_eq!(chain_ctor_err(&inputs, |c| c.state3.site = var(&name[1..])), unprefixed);
    }
    // First offending operand in check order wins, whichever error it carries.
    let both = |first: &'static str, last: &'static str| {
        chain_ctor_err(&inputs, move |c| {
            c.enable = var(first);
            c.state5.site = var(last);
        })
    };
    assert_eq!(both("bad", chain_t), unprefixed);
    assert_eq!(both(exit_t, "bad"), collision);
    // Sibling names outside the exact temporary namespaces stay admitted.
    for name in [
        "%ns_s2_exit.caller.0",
        "%ns_s1_arith.const_fetch.x",
        "%ns_chain.local.tempo",
        "%ns_s3_rr_x",
        "%ns_s4_pv.local.temp.0",
        "%ns_s4_pe",
    ] {
        assert!(chain_ctor_err(&inputs, |c| c.state5.site = var(name)).is_none(), "{name}");
    }
}

fn chain_defined_outputs(code: &[Instruction]) -> Vec<String> {
    let mut out = Vec::new();
    for inst in code {
        match inst {
            Instruction::CondSelect { output, .. }
            | Instruction::TestEq { output, .. }
            | Instruction::LessThan { output, .. }
            | Instruction::Add { output, .. }
            | Instruction::Mul { output, .. } => out.push(output.0.clone()),
            Instruction::DivModPowerOfTwo { outputs, .. } => {
                out.extend(outputs.iter().map(|o| o.0.clone()))
            }
            _ => {}
        }
    }
    out
}

/// Pinned SDK JSON boundary as in the public backend and literal bridge: serde_json, version
/// `3.0`, `IrSource::load`, structural equality. Not a check, compile or proof.
fn assert_sdk_json_roundtrip(code: &[Instruction]) {
    let ir = IrSource {
        version: Default::default(),
        inputs: vec![],
        outputs: vec![],
        do_communications_commitment: false,
        instructions: Arc::new(code.to_vec()),
    };
    let mut disk = serde_json::to_value(&ir).expect("serde_json serialization");
    disk["version"] = serde_json::json!({"major": 3, "minor": 0});
    let zkir = serde_json::to_string(&disk).expect("serde_json serialization");
    let parsed = IrSource::load(zkir.as_bytes()).expect("pinned SDK IrSource::load");
    assert_eq!(parsed, ir, "pinned SDK JSON round-trip changed the IR");
}

#[test]
fn chain_instruction_stream_roundtrips_pinned_sdk_json_load_in_every_branch() {
    let scenario = |lhs_u128: u128, rhs_slot: ConcreteDataSlot, bounds_data_nodes: u128| ChainScenario {
        lhs_u128,
        rhs_slot,
        bounds_data_nodes,
        initial_remaining_work: 4,
    };
    // Applied value, overflow error 10, sort-mismatch error 11 and DataNodes cap refusal.
    for sc in [
        scenario(7, ConcreteDataSlot::u128_row(9), 5),
        scenario(u128::MAX, ConcreteDataSlot::u128_row(1), 5),
        scenario(7, ConcreteDataSlot::bool_row(1), 5),
        scenario(7, ConcreteDataSlot::u128_row(9), 1),
    ] {
        let (inputs, env) = build_chain_fixture(&sc);
        let chain = emit_b_add_to_publication_chain(&inputs, "ns").unwrap();
        eval_zkir(&chain.instructions, env.clone()).expect("clean baseline must pass");
        let outputs = chain_defined_outputs(&chain.instructions);
        let mut seen = std::collections::BTreeSet::new();
        assert!(outputs.iter().all(|n| seen.insert(n.as_str())), "every output is defined once");
        assert!(outputs.iter().chain(env.keys()).all(|n| n.starts_with('%')));
        for stage in ["ns_s3_rr", "ns_s4_pv", "ns_s4_pe"] {
            assert!(outputs.iter().any(|n| is_legacy_completion_scope_variable(stage, n)), "{stage}");
            assert!(env.keys().all(|k| !is_legacy_completion_scope_variable(stage, k)), "{stage}");
        }
        assert_sdk_json_roundtrip(&chain.instructions);
    }
}

// ---- Sub (BinaryOp::Sub) 5-stage chain: test-only adapters ----
type SubChainHelper = super::b_scalar_subtraction::BScalarArithmeticRegister;
type SubChainCounters = super::b_scalar_subtraction_completion::BScalarSubCompletionCounters;

impl EnvBuilder {
    fn bind_sub_publish_state(&self, prefix: &str, st: &ConcreteAdminState) -> BSubPublishState {
        let outcome = BOutcomeLanes {
            tag: self.bind(&format!("{prefix}.out.tag"), st.outcome_tag),
            halted_value: BValueRef { space: self.bind(&format!("{prefix}.out.hv.sp"), st.halted_space), index: self.bind(&format!("{prefix}.out.hv.ix"), st.halted_index) },
            rejected_code: self.bind(&format!("{prefix}.out.rej"), st.rejected_code),
            exhausted_reason: self.bind(&format!("{prefix}.out.exh"), st.exhausted_reason),
        };
        let control = BControlRow {
            tag: self.bind(&format!("{prefix}.ctrl.tag"), st.ctrl_tag),
            return_value: BValueRef { space: self.bind(&format!("{prefix}.ctrl.rv.sp"), st.ctrl_rv_space), index: self.bind(&format!("{prefix}.ctrl.rv.ix"), st.ctrl_rv_index) },
            prim_outcome_tag: self.bind(&format!("{prefix}.ctrl.pot"), st.ctrl_pot),
            prim_value: BValueRef { space: self.bind(&format!("{prefix}.ctrl.pv.sp"), st.ctrl_pv_space), index: self.bind(&format!("{prefix}.ctrl.pv.ix"), st.ctrl_pv_index) },
            prim_error_code: self.bind(&format!("{prefix}.ctrl.pec"), st.ctrl_pec),
            error_code: self.bind(&format!("{prefix}.ctrl.ec"), st.ctrl_ec),
            rest: vec![self.bind(&format!("{prefix}.ctrl.rest.0"), 0)],
        };
        let h_rest = super::b_scalar_subtraction::BScalarHelperRestLanes {
            origin_reference_cursor: self.bind(&format!("{prefix}.h.rest.0"), st.helper.origin_ref_cursor),
            parent_resume_present: self.bind(&format!("{prefix}.h.rest.1"), 0),
            parent_resume_value: self.bind(&format!("{prefix}.h.rest.2"), 0),
            expected_present: self.bind(&format!("{prefix}.h.rest.3"), 0),
            expected_value: self.bind(&format!("{prefix}.h.rest.4"), 0),
        };
        let helper = SubChainHelper {
            present: self.bind(&format!("{prefix}.h.pres"), st.helper.present),
            kind: self.bind(&format!("{prefix}.h.kind"), st.helper.kind),
            site: self.bind(&format!("{prefix}.h.site"), st.helper.site),
            mode: self.bind(&format!("{prefix}.h.mode"), st.helper.mode),
            phase: self.bind(&format!("{prefix}.h.phase"), st.helper.phase),
            operand_a: super::b_scalar_subtraction::BValueRef { space: self.bind(&format!("{prefix}.h.oa.sp"), st.helper.op_a_space), index: self.bind(&format!("{prefix}.h.oa.ix"), st.helper.op_a_index) },
            operand_b: super::b_scalar_subtraction::BOptValueRef { present: self.bind(&format!("{prefix}.h.ob.pr"), st.helper.op_b_present), value: super::b_scalar_subtraction::BValueRef { space: self.bind(&format!("{prefix}.h.ob.sp"), st.helper.op_b_space), index: self.bind(&format!("{prefix}.h.ob.ix"), st.helper.op_b_index) } },
            reg_a: self.bind_u128(&format!("{prefix}.h.reg_a"), st.helper.reg_a),
            staged: super::b_scalar_subtraction::BScalarStagedLanes { tag: self.bind(&format!("{prefix}.h.st.tag"), st.helper.staged_tag), u128_value: self.bind_u128(&format!("{prefix}.h.st.u128"), st.helper.staged_u128), bool_value: self.bind(&format!("{prefix}.h.st.bool"), 0), pending_node_tag: self.bind(&format!("{prefix}.h.st.pnt"), 0), prim_error_code: self.bind(&format!("{prefix}.h.st.pec"), st.helper.staged_err), selector: self.bind(&format!("{prefix}.h.st.sel"), 0), type_test_ok: self.bind(&format!("{prefix}.h.st.tto"), 0) },
            rest: h_rest.to_rest_vec(),
        };
        let result = BHelperResultLanes {
            present: self.bind(&format!("{prefix}.res.pres"), st.res_present),
            tag: self.bind(&format!("{prefix}.res.tag"), st.res_tag),
            scalar_value: BValueRef { space: self.bind(&format!("{prefix}.res.sv.sp"), st.res_val_space), index: self.bind(&format!("{prefix}.res.sv.ix"), st.res_val_index) },
            scalar_error_code: self.bind(&format!("{prefix}.res.err"), st.res_err),
            scalar_selector: self.bind(&format!("{prefix}.res.sel"), 0),
            scalar_type_test_ok: self.bind(&format!("{prefix}.res.tto"), 0),
            rest: vec![self.bind(&format!("{prefix}.res.rest.0"), 0)],
        };
        let data = BDataTableLanes { capacity: st.data_rows.len(), active_rows: self.bind(&format!("{prefix}.data.act"), st.data_active), rows: st.data_rows.iter().enumerate().map(|(r, row)| BDataRowLanes { tag: self.bind(&format!("{prefix}.data.r{r}.tag"), row.tag), n: self.bind_u128(&format!("{prefix}.data.r{r}.n"), row.n), bool_b: self.bind(&format!("{prefix}.data.r{r}.bool"), row.bool_b), fields_start: self.bind(&format!("{prefix}.data.r{r}.fs"), row.f_start), fields_len: self.bind(&format!("{prefix}.data.r{r}.fl"), row.f_len), rest: vec![self.bind(&format!("{prefix}.data.r{r}.rest.0"), row.rest[0])] }).collect() };
        let frames = st.frames.iter().enumerate().map(|(i, f)| BFrameRow { tag: self.bind(&format!("{prefix}.f{i}.tag"), f.tag), origin: self.bind(&format!("{prefix}.f{i}.orig"), f.origin), right_op: self.bind(&format!("{prefix}.f{i}.rop"), f.right_op), right_left: BValueRef { space: self.bind(&format!("{prefix}.f{i}.rl.sp"), f.right_left_space), index: self.bind(&format!("{prefix}.f{i}.rl.ix"), f.right_left_index) }, rest: vec![self.bind(&format!("{prefix}.f{i}.rest.0"), f.rest[0])] }).collect();
        let counters = SubChainCounters {
            work: super::b_scalar_subtraction::BChargedCounters { remaining_work: self.bind_u128(&format!("{prefix}.cnt.rem"), st.remaining_work), consumed_work: self.bind_u128(&format!("{prefix}.cnt.con"), st.consumed_work), helper_work: self.bind_u128(&format!("{prefix}.cnt.hw"), st.helper_work), helper_visits: self.bind_u128(&format!("{prefix}.cnt.hv"), st.helper_visits) },
            reference_cursor: self.bind(&format!("{prefix}.cnt.ref_c"), st.reference_cursor),
            alloc_data_nodes: self.bind(&format!("{prefix}.cnt.alloc_dn"), st.alloc_data_nodes),
            alloc_data_fields: self.bind(&format!("{prefix}.cnt.alloc_df"), 0),
            alloc_environment_cells: self.bind(&format!("{prefix}.cnt.alloc_env"), 0),
            depth: self.bind(&format!("{prefix}.cnt.depth"), st.depth),
            task_live: self.bind(&format!("{prefix}.cnt.task_live"), st.task_live),
            active_data_fields: self.bind(&format!("{prefix}.cnt.act_df"), 0),
            active_env: self.bind(&format!("{prefix}.cnt.act_env"), 0),
            active_value_slots: self.bind(&format!("{prefix}.cnt.act_vs"), 0),
            rest: vec![self.bind(&format!("{prefix}.cnt.rest.0"), 77)],
        };
        BSubPublishState { site: self.bind(&format!("{prefix}.site"), st.site), outcome, control, control_origin: self.bind(&format!("{prefix}.ctrl_orig"), st.control_origin), helper, result, data, frames, counters, reference_cursor_bytes: self.bind_u128(&format!("{prefix}.ref_c_bytes"), st.reference_cursor) }
    }
}

struct SubChainScenario { lhs_u128: u128, rhs_slot: ConcreteDataSlot, bounds_data_nodes: u128, initial_remaining_work: u128 }

fn sub_chain_right_frame() -> ConcreteFrameSlot { let mut f = ConcreteFrameSlot::right_f_add(808, VALUE_SPACE_CONST, 0); f.right_op = BINARY_OP_SUB; f }

fn build_sub_chain_fixture(scenario: &SubChainScenario) -> (BSubToPublicationChainInputs, BTreeMap<String, u128>) {
    let is_rhs_uint = scenario.rhs_slot.tag == DATA_TAG_U128;
    let difference = if is_rhs_uint { scenario.lhs_u128.checked_sub(scenario.rhs_slot.n) } else { None };
    let applied = difference.is_some();
    let err_code = if !is_rhs_uint { PRIM_ERROR_SORT_MISMATCH } else if applied { 0 } else { PRIM_ERROR_ARITH_OVERFLOW };
    let diff_value = difference.unwrap_or(0);
    let state0 = ConcreteAdminState { site: SITE_RET_RIGHT, outcome_tag: OUTCOME_TAG_RUNNING, halted_space: 0, halted_index: 0, rejected_code: 0, exhausted_reason: 0, ctrl_tag: CONTROL_TAG_RETURN, ctrl_rv_space: VALUE_SPACE_DATA, ctrl_rv_index: 0, ctrl_pot: 0, ctrl_pv_space: 0, ctrl_pv_index: 0, ctrl_pec: 0, ctrl_ec: 0, control_origin: 404, helper: ConcreteHelperState { present: HELPER_PRESENT, kind: HELPER_KIND_SCALAR, site: SITE_RET_RIGHT, mode: super::b_scalar_subtraction::SCALAR_MODE_SUB, phase: SCALAR_PHASE_SECOND, op_a_space: VALUE_SPACE_CONST, op_a_index: 0, op_b_present: OPT_PRESENT, op_b_space: VALUE_SPACE_DATA, op_b_index: 0, reg_a: scenario.lhs_u128, staged_tag: SCALAR_STAGED_TAG_NONE, staged_u128: 0, staged_err: 0, origin_ref_cursor: 5 }, res_present: 0, res_tag: 0, res_val_space: 0, res_val_index: 0, res_err: 0, data_active: 1, data_rows: vec![scenario.rhs_slot.clone(), ConcreteDataSlot::zero()], frames: vec![sub_chain_right_frame()], depth: 1, remaining_work: scenario.initial_remaining_work, consumed_work: 10, helper_work: 2, helper_visits: 1, reference_cursor: 5, alloc_data_nodes: 1, task_live: 0 };
    let mut state1 = state0.clone();
    state1.helper.phase = SCALAR_PHASE_DONE;
    if applied { state1.helper.staged_tag = SCALAR_STAGED_TAG_U128_VALUE; state1.helper.staged_u128 = diff_value; } else { state1.helper.staged_tag = SCALAR_STAGED_TAG_PRIM_ERROR; state1.helper.staged_err = err_code; }
    state1.remaining_work -= 1; state1.consumed_work += 1; state1.helper_work += 1; state1.helper_visits += 1;
    let mut state2 = state1.clone();
    state2.remaining_work -= 1; state2.consumed_work += 1; state2.helper_work += 1;
    state2.helper = ConcreteHelperState::absent();
    state2.res_present = OPT_PRESENT;
    if applied { state2.data_active = 2; state2.data_rows[1] = ConcreteDataSlot::u128_row(diff_value); state2.alloc_data_nodes = 2; state2.res_tag = HELPER_RESULT_TAG_SCALAR_VALUE; state2.res_val_space = VALUE_SPACE_DATA; state2.res_val_index = 1; } else { state2.res_tag = HELPER_RESULT_TAG_SCALAR_ERROR; state2.res_err = err_code; }
    let mut state3 = state2.clone();
    state3.site = SITE_PRIM; state3.frames[0] = ConcreteFrameSlot::zero(); state3.depth = 0; state3.control_origin = 808;
    state3.res_present = 0; state3.res_tag = 0; state3.res_val_space = 0; state3.res_val_index = 0; state3.res_err = 0;
    state3.ctrl_tag = CONTROL_TAG_PRIM_RESULT; state3.ctrl_rv_space = 0; state3.ctrl_rv_index = 0;
    if applied { state3.ctrl_pot = PRIM_OUTCOME_TAG_VALUE; state3.ctrl_pv_space = VALUE_SPACE_DATA; state3.ctrl_pv_index = 1; } else { state3.ctrl_pot = PRIM_OUTCOME_TAG_ERROR; state3.ctrl_pec = err_code; }
    state3.remaining_work -= 1; state3.consumed_work += 1; state3.reference_cursor += 1;
    let mut state4 = state3.clone();
    let mut state4_val_branch = state3.clone();
    let mut state4_err_branch = state3.clone();
    if applied { state4.ctrl_tag = CONTROL_TAG_RETURN; state4.ctrl_rv_space = VALUE_SPACE_DATA; state4.ctrl_rv_index = 1; state4.ctrl_pot = 0; state4.ctrl_pv_space = 0; state4.ctrl_pv_index = 0; state4.remaining_work -= 1; state4.consumed_work += 1; state4.reference_cursor += 1; state4_val_branch = state4.clone(); } else { state4.ctrl_tag = CONTROL_TAG_ERROR; state4.ctrl_pot = 0; state4.ctrl_pec = 0; state4.ctrl_ec = err_code; state4.remaining_work -= 1; state4.consumed_work += 1; state4.reference_cursor += 1; state4_err_branch = state4.clone(); }
    let mut state5 = state4.clone();
    if applied { state5.outcome_tag = OUTCOME_TAG_HALTED; state5.halted_space = VALUE_SPACE_DATA; state5.halted_index = 1; } else { state5.outcome_tag = OUTCOME_TAG_REJECTED; state5.rejected_code = err_code; }
    let builder = EnvBuilder::new();
    let constants = super::b_scalar_subtraction::BValueTableRead { capacity: 1, active_rows: builder.bind("%sb.const.act", 1), rows: vec![super::b_scalar_subtraction::BValueRowLanes { tag: builder.bind("%sb.const.r0.tag", CONST_TAG_U128), n: builder.bind_u128("%sb.const.r0.n", scenario.lhs_u128) }] };
    let s0 = builder.bind_sub_publish_state("%sb.s0", &state0);
    let s1 = builder.bind_sub_publish_state("%sb.s1", &state1);
    let s2 = builder.bind_sub_publish_state("%sb.s2", &state2);
    let (exit_post, fail_post) = if applied { (&state2, &state1) } else { (&state1, &state2) };
    let s2x = builder.bind_sub_publish_state("%sb.s2x", exit_post);
    let s2f = builder.bind_sub_publish_state("%sb.s2f", fail_post);
    let s3 = builder.bind_sub_publish_state("%sb.s3", &state3);
    let pv = builder.bind_sub_publish_state("%sb.pv", &state4_val_branch);
    let pe = builder.bind_sub_publish_state("%sb.pe", &state4_err_branch);
    let s4 = builder.bind_sub_publish_state("%sb.s4", &state4);
    let s5 = builder.bind_sub_publish_state("%sb.s5", &state5);
    let inputs = BSubToPublicationChainInputs { enable: builder.bind("%sb.enable", 1), bounds_data_nodes: builder.bind("%sb.bounds.dn", scenario.bounds_data_nodes), constants, state0: s0, state1: s1, state2: s2, s2_exit_post: s2x, s2_fail_post: s2f, state3: s3, prim_val_post: pv.to_legacy_handoff().state, prim_err_post: pe.to_legacy_handoff().state, state4: s4, state5: s5 };
    (inputs, builder.env.into_inner())
}

#[test]
fn sub_chain_checked_difference_applied_reaches_p1_halted_data_at_depth_zero() {
    // Literal sample: 25 - 7 = 18 (author-specified, not read from candidate output).
    let scenario = SubChainScenario { lhs_u128: 25, rhs_slot: ConcreteDataSlot::u128_row(7), bounds_data_nodes: 5, initial_remaining_work: 4 };
    let (inputs, env) = build_sub_chain_fixture(&scenario);
    let chain = emit_b_sub_to_publication_chain(&inputs, "sub_val_chain").unwrap();
    let eval = eval_zkir(&chain.instructions, env.clone()).unwrap();
    assert_eq!(eval_operand(&chain.arithmetic_block.b_is_uint, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.arithmetic_block.underflow, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.exit_enable, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.failure_enable, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.exit_block.applied, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.exit_block.cap_refused, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.legacy_enable, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.prim_value_enable, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.prim_error_enable, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.return_dispatch, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.p1_enable, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.p1_block.publish_value, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.p1_block.publish_error, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.p1_block.published_value.space, &eval).unwrap(), VALUE_SPACE_DATA);
    assert_eq!(eval_operand(&chain.p1_block.published_value.index, &eval).unwrap(), 1);
    assert_eq!(eval["%sb.s5.out.tag"], OUTCOME_TAG_HALTED);
    assert_eq!(eval["%sb.s5.data.r1.n.0"], 18);
    assert_eq!(eval["%sb.s0.h.mode"], super::b_scalar_subtraction::SCALAR_MODE_SUB);
    assert_eq!(eval["%sb.s0.f0.rop"], BINARY_OP_SUB);
    // Isolated bad witness: forged staged difference 19 instead of 18.
    let mut bad = env; bad.insert("%sb.s1.h.st.u128.0".into(), 19);
    let err = eval_zkir(&chain.instructions, bad).unwrap_err();
    assert!(err.contains("ConstrainEq failed"), "{err}");
}

#[test]
fn sub_chain_genuine_underflow_error10_reaches_p1_rejected10() {
    // Literal sample: 7 - 25 underflows checked UInt128, PrimError 10.
    let scenario = SubChainScenario { lhs_u128: 7, rhs_slot: ConcreteDataSlot::u128_row(25), bounds_data_nodes: 5, initial_remaining_work: 4 };
    let (inputs, env) = build_sub_chain_fixture(&scenario);
    let chain = emit_b_sub_to_publication_chain(&inputs, "sub_under_chain").unwrap();
    let eval = eval_zkir(&chain.instructions, env.clone()).unwrap();
    assert_eq!(eval_operand(&chain.arithmetic_block.b_is_uint, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.arithmetic_block.underflow, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.exit_enable, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.failure_enable, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.failure_block.error_code, &eval).unwrap(), PRIM_ERROR_ARITH_OVERFLOW);
    assert_eq!(eval_operand(&chain.legacy_enable, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.prim_value_enable, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.prim_error_enable, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.p1_enable, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.p1_block.publish_value, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.p1_block.publish_error, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.p1_block.published_error_code, &eval).unwrap(), 10);
    assert_eq!(eval["%sb.s5.out.tag"], OUTCOME_TAG_REJECTED);
    assert_eq!(eval["%sb.s5.out.rej"], 10);
    assert_eq!(eval["%sb.s3.ctrl.pec"], 10);
    // Isolated bad witness: forged typed-handoff PrimError code 11 at state3.
    let mut bad = env; bad.insert("%sb.s3.ctrl.pec".into(), 11);
    let err = eval_zkir(&chain.instructions, bad).unwrap_err();
    assert!(err.contains("ConstrainEq failed"), "{err}");
}

#[test]
fn sub_chain_non_uint_operand_b_error11_reaches_p1_rejected11() {
    // Literal sample: 25 - Bool(1) is a sort mismatch, PrimError 11 (no arithmetic on Bool payload).
    let scenario = SubChainScenario { lhs_u128: 25, rhs_slot: ConcreteDataSlot::bool_row(1), bounds_data_nodes: 5, initial_remaining_work: 4 };
    let (inputs, env) = build_sub_chain_fixture(&scenario);
    let chain = emit_b_sub_to_publication_chain(&inputs, "sub_sort_chain").unwrap();
    let eval = eval_zkir(&chain.instructions, env.clone()).unwrap();
    assert_eq!(eval_operand(&chain.arithmetic_block.b_is_uint, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.exit_enable, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.failure_enable, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.failure_block.error_code, &eval).unwrap(), PRIM_ERROR_SORT_MISMATCH);
    assert_eq!(eval_operand(&chain.failure_block.is_sort_mismatch_11, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.legacy_enable, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.prim_value_enable, &eval).unwrap(), 0);
    assert_eq!(eval_operand(&chain.prim_error_enable, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.p1_enable, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.p1_block.publish_error, &eval).unwrap(), 1);
    assert_eq!(eval_operand(&chain.p1_block.published_error_code, &eval).unwrap(), 11);
    assert_eq!(eval["%sb.s5.out.tag"], OUTCOME_TAG_REJECTED);
    assert_eq!(eval["%sb.s5.out.rej"], 11);
    // Isolated bad witness: forged final Rejected code 10 instead of 11.
    let mut bad = env; bad.insert("%sb.s5.out.rej".into(), 10);
    let err = eval_zkir(&chain.instructions, bad).unwrap_err();
    assert!(err.contains("ConstrainEq failed"), "{err}");
}
