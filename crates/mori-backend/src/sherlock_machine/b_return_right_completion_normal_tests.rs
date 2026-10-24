// Target: crates/mori-backend/src/sherlock_machine/b_return_right_completion_normal_tests.rs

use std::cell::RefCell;
use std::collections::BTreeMap;

use super::b_return_right_completion::*;
use super::b_scalar_arithmetic::*;
use super::b_scalar_completion::*;
use super::*;

/// Host-side test evaluator for emitted `midnight_zkir_v3::Instruction` streams.
///
/// Grounded on `ACTUAL-PRIOR-EVAL-FUNCTIONS.rs.txt`: evaluates over host `u128`
/// values (not full prime-field `Fr` arithmetic) and resolves `Operand::Immediate`
/// via exact `Fr::from(candidate)` comparison over `0..=1024`.
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
                    return Err(format!("step {index}: ConstrainEq failed ({va} != {vb})"));
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
struct ConcreteFrame {
    tag: u128,
    origin: u128,
    right_op: u128,
    right_left_space: u128,
    right_left_index: u128,
    rest: Vec<u128>,
}

impl ConcreteFrame {
    fn zero(rest_len: usize) -> Self {
        Self {
            tag: 0,
            origin: 0,
            right_op: 0,
            right_left_space: 0,
            right_left_index: 0,
            rest: vec![0; rest_len],
        }
    }

    fn right_f_add(origin: u128, left_space: u128, left_index: u128, rest_len: usize) -> Self {
        Self {
            tag: FRAME_TAG_RIGHT_F as u128,
            origin,
            right_op: BINARY_OP_ADD as u128,
            right_left_space: left_space,
            right_left_index: left_index,
            rest: vec![0; rest_len],
        }
    }
}

#[derive(Clone, Debug)]
struct ConcreteControl {
    tag: u128,
    return_space: u128,
    return_index: u128,
    prim_outcome_tag: u128,
    prim_val_space: u128,
    prim_val_index: u128,
    prim_error_code: u128,
    error_code: u128,
    rest: Vec<u128>,
}

impl ConcreteControl {
    fn return_val(space: u128, index: u128, rest_len: usize) -> Self {
        Self {
            tag: CONTROL_TAG_RETURN as u128,
            return_space: space,
            return_index: index,
            prim_outcome_tag: 0,
            prim_val_space: 0,
            prim_val_index: 0,
            prim_error_code: 0,
            error_code: 0,
            rest: vec![0; rest_len],
        }
    }

    fn prim_value(space: u128, index: u128, rest_len: usize) -> Self {
        Self {
            tag: CONTROL_TAG_PRIM_RESULT as u128,
            return_space: 0,
            return_index: 0,
            prim_outcome_tag: PRIM_OUTCOME_TAG_VALUE as u128,
            prim_val_space: space,
            prim_val_index: index,
            prim_error_code: 0,
            error_code: 0,
            rest: vec![0; rest_len],
        }
    }

    fn prim_error(code: u128, rest_len: usize) -> Self {
        Self {
            tag: CONTROL_TAG_PRIM_RESULT as u128,
            return_space: 0,
            return_index: 0,
            prim_outcome_tag: PRIM_OUTCOME_TAG_ERROR as u128,
            prim_val_space: 0,
            prim_val_index: 0,
            prim_error_code: code,
            error_code: 0,
            rest: vec![0; rest_len],
        }
    }

    fn control_error(code: u128, rest_len: usize) -> Self {
        Self {
            tag: CONTROL_TAG_ERROR as u128,
            return_space: 0,
            return_index: 0,
            prim_outcome_tag: 0,
            prim_val_space: 0,
            prim_val_index: 0,
            prim_error_code: 0,
            error_code: code,
            rest: vec![0; rest_len],
        }
    }
}

#[derive(Clone, Debug)]
struct ConcreteResult {
    present: u128,
    tag: u128,
    val_space: u128,
    val_index: u128,
    error_code: u128,
    selector: u128,
    type_test: u128,
    rest: Vec<u128>,
}

impl ConcreteResult {
    fn absent(rest_len: usize) -> Self {
        Self {
            present: 0,
            tag: 0,
            val_space: 0,
            val_index: 0,
            error_code: 0,
            selector: 0,
            type_test: 0,
            rest: vec![0; rest_len],
        }
    }

    fn scalar_value(space: u128, index: u128, rest_len: usize) -> Self {
        Self {
            present: 1,
            tag: HELPER_RESULT_TAG_SCALAR_VALUE as u128,
            val_space: space,
            val_index: index,
            error_code: 0,
            selector: 0,
            type_test: 0,
            rest: vec![0; rest_len],
        }
    }

    fn scalar_error(code: u128, rest_len: usize) -> Self {
        Self {
            present: 1,
            tag: HELPER_RESULT_TAG_SCALAR_ERROR as u128,
            val_space: 0,
            val_index: 0,
            error_code: code,
            selector: 0,
            type_test: 0,
            rest: vec![0; rest_len],
        }
    }
}

#[derive(Clone, Debug)]
struct ConcreteLegacyState {
    site: u128,
    outcome: u128,
    control: ConcreteControl,
    control_origin: u128,
    helper_present: u128,
    result: ConcreteResult,
    frames: Vec<ConcreteFrame>,
    depth: u128,
    remaining_work: u128,
    consumed_work: u128,
    helper_work: u128,
    helper_visits: u128,
    reference_cursor: u128,
    rest: Vec<u128>,
}

struct LegacyFixture {
    enable: u128,
    pre: ConcreteLegacyState,
    post: ConcreteLegacyState,
}

impl LegacyFixture {
    fn build_symbolic_inputs(
        &self,
    ) -> (
        Operand,
        BLegacyCompletionState,
        BLegacyCompletionState,
        BTreeMap<String, u128>,
    ) {
        let env = RefCell::new(BTreeMap::new());
        // SDK serde `Operand` variables are `%`-prefixed; every caller lane is bound as `%{name}`.
        let bind = |name: &str, val: u128| -> Operand {
            let name = format!("%{name}");
            env.borrow_mut().insert(name.clone(), val);
            Operand::Variable(Identifier(name))
        };
        let bind_u128 = |prefix: &str, val: u128| -> BU128Lanes {
            let le = val.to_le_bytes();
            std::array::from_fn(|i| bind(&format!("{}_b_{}", prefix, i), le[i] as u128))
        };
        let bind_state = |prefix: &str, st: &ConcreteLegacyState| -> BLegacyCompletionState {
            let control = BControlRow {
                tag: bind(&format!("{}_ctrl_tag", prefix), st.control.tag),
                return_value: BValueRef {
                    space: bind(&format!("{}_ctrl_rv_sp", prefix), st.control.return_space),
                    index: bind(&format!("{}_ctrl_rv_ix", prefix), st.control.return_index),
                },
                prim_outcome_tag: bind(
                    &format!("{}_ctrl_pot", prefix),
                    st.control.prim_outcome_tag,
                ),
                prim_value: BValueRef {
                    space: bind(&format!("{}_ctrl_pv_sp", prefix), st.control.prim_val_space),
                    index: bind(&format!("{}_ctrl_pv_ix", prefix), st.control.prim_val_index),
                },
                prim_error_code: bind(&format!("{}_ctrl_pec", prefix), st.control.prim_error_code),
                error_code: bind(&format!("{}_ctrl_ec", prefix), st.control.error_code),
                rest: st
                    .control
                    .rest
                    .iter()
                    .enumerate()
                    .map(|(i, v)| bind(&format!("{}_ctrl_rest_{}", prefix, i), *v))
                    .collect(),
            };
            let helper = BScalarArithmeticRegister {
                present: bind(&format!("{}_hlp_pres", prefix), st.helper_present),
                kind: bind(&format!("{}_hlp_kind", prefix), 0),
                site: bind(&format!("{}_hlp_site", prefix), 0),
                mode: bind(&format!("{}_hlp_mode", prefix), 0),
                phase: bind(&format!("{}_hlp_ph", prefix), 0),
                operand_a: BValueRef {
                    space: bind(&format!("{}_hlp_oa_sp", prefix), 0),
                    index: bind(&format!("{}_hlp_oa_ix", prefix), 0),
                },
                operand_b: BOptValueRef {
                    present: bind(&format!("{}_hlp_ob_pr", prefix), 0),
                    value: BValueRef {
                        space: bind(&format!("{}_hlp_ob_sp", prefix), 0),
                        index: bind(&format!("{}_hlp_ob_ix", prefix), 0),
                    },
                },
                reg_a: bind_u128(&format!("{}_ra_u128", prefix), 0),
                staged: BScalarStagedLanes {
                    tag: bind(&format!("{}_st_tag", prefix), 0),
                    u128_value: bind_u128(&format!("{}_st_u128", prefix), 0),
                    bool_value: bind(&format!("{}_st_bv", prefix), 0),
                    pending_node_tag: bind(&format!("{}_st_pnt", prefix), 0),
                    prim_error_code: bind(&format!("{}_st_pec", prefix), 0),
                    selector: bind(&format!("{}_st_sel", prefix), 0),
                    type_test_ok: bind(&format!("{}_st_tto", prefix), 0),
                },
                rest: vec![bind(&format!("{}_hlp_rest_0", prefix), 0)],
            };
            let result = BHelperResultLanes {
                present: bind(&format!("{}_res_pres", prefix), st.result.present),
                tag: bind(&format!("{}_res_tag", prefix), st.result.tag),
                scalar_value: BValueRef {
                    space: bind(&format!("{}_res_v_sp", prefix), st.result.val_space),
                    index: bind(&format!("{}_res_v_ix", prefix), st.result.val_index),
                },
                scalar_error_code: bind(&format!("{}_res_ec", prefix), st.result.error_code),
                scalar_selector: bind(&format!("{}_res_sel", prefix), st.result.selector),
                scalar_type_test_ok: bind(&format!("{}_res_tt", prefix), st.result.type_test),
                rest: st
                    .result
                    .rest
                    .iter()
                    .enumerate()
                    .map(|(i, v)| bind(&format!("{}_res_rest_{}", prefix, i), *v))
                    .collect(),
            };
            let frames = st
                .frames
                .iter()
                .enumerate()
                .map(|(slot, f)| BFrameRow {
                    tag: bind(&format!("{}_f_{}_tag", prefix, slot), f.tag),
                    origin: bind(&format!("{}_f_{}_orig", prefix, slot), f.origin),
                    right_op: bind(&format!("{}_f_{}_rop", prefix, slot), f.right_op),
                    right_left: BValueRef {
                        space: bind(&format!("{}_f_{}_rl_sp", prefix, slot), f.right_left_space),
                        index: bind(&format!("{}_f_{}_rl_ix", prefix, slot), f.right_left_index),
                    },
                    rest: f
                        .rest
                        .iter()
                        .enumerate()
                        .map(|(j, v)| bind(&format!("{}_f_{}_rest_{}", prefix, slot, j), *v))
                        .collect(),
                })
                .collect();

            BLegacyCompletionState {
                site: bind(&format!("{}_site", prefix), st.site),
                outcome: bind(&format!("{}_outcome", prefix), st.outcome),
                control,
                control_origin: bind(&format!("{}_ctrl_orig", prefix), st.control_origin),
                helper,
                result,
                frames,
                depth: bind(&format!("{}_depth", prefix), st.depth),
                remaining_work: bind_u128(&format!("{}_rem_w", prefix), st.remaining_work),
                consumed_work: bind_u128(&format!("{}_cons_w", prefix), st.consumed_work),
                helper_work: bind_u128(&format!("{}_hlp_w", prefix), st.helper_work),
                helper_visits: bind_u128(&format!("{}_hlp_v", prefix), st.helper_visits),
                reference_cursor: bind_u128(&format!("{}_ref_c", prefix), st.reference_cursor),
                rest: st
                    .rest
                    .iter()
                    .enumerate()
                    .map(|(i, v)| bind(&format!("{}_rest_{}", prefix, i), *v))
                    .collect(),
            }
        };

        let enable_op = bind("enable", self.enable);
        let pre_st = bind_state("pre", &self.pre);
        let post_st = bind_state("post", &self.post);
        drop(bind_state);
        drop(bind_u128);
        drop(bind);
        (enable_op, pre_st, post_st, env.into_inner())
    }
}

fn sample_return_right_value_fixture() -> LegacyFixture {
    let f0 = ConcreteFrame {
        tag: FRAME_TAG_LET_F as u128,
        origin: 101,
        right_op: 0,
        right_left_space: 0,
        right_left_index: 0,
        rest: vec![77, 88],
    };
    let f1 = ConcreteFrame::right_f_add(202, VALUE_SPACE_CONST as u128, 4, 2);
    let f2 = ConcreteFrame::zero(2);

    let pre = ConcreteLegacyState {
        site: SITE_RET_RIGHT as u128,
        outcome: OUTCOME_TAG_RUNNING as u128,
        control: ConcreteControl::return_val(VALUE_SPACE_DATA as u128, 2, 2),
        control_origin: 199,
        helper_present: 0,
        result: ConcreteResult::scalar_value(VALUE_SPACE_DATA as u128, 3, 1),
        frames: vec![f0.clone(), f1, f2.clone()],
        depth: 2,
        remaining_work: 50,
        consumed_work: 10,
        helper_work: 4,
        helper_visits: 1,
        reference_cursor: 7,
        rest: vec![500, 600],
    };
    let post = ConcreteLegacyState {
        site: SITE_PRIM as u128,
        outcome: OUTCOME_TAG_RUNNING as u128,
        control: ConcreteControl::prim_value(VALUE_SPACE_DATA as u128, 3, 2),
        control_origin: 202,
        helper_present: 0,
        result: ConcreteResult::absent(1),
        frames: vec![f0, ConcreteFrame::zero(2), f2],
        depth: 1,
        remaining_work: 49,
        consumed_work: 11,
        helper_work: 4,
        helper_visits: 1,
        reference_cursor: 8,
        rest: vec![500, 600],
    };
    LegacyFixture {
        enable: 1,
        pre,
        post,
    }
}

#[test]
fn test_return_right_add_scalar_value_pops_top_frame_and_stages_prim_value() {
    let fx = sample_return_right_value_fixture();
    let (en, pre, post, env) = fx.build_symbolic_inputs();
    let ir = emit_k_machine_return_right_add(en, &pre, &post).expect("emit return.right");
    eval_zkir(&ir, env).expect("valid ScalarValue return.right transition must pass");

    // Negative: unzeroed popped top frame slot 1 must fail.
    let mut bad_pop = sample_return_right_value_fixture();
    bad_pop.post.frames[1].origin = 202;
    let (en, pre, post, env) = bad_pop.build_symbolic_inputs();
    let ir = emit_k_machine_return_right_add(en, &pre, &post).unwrap();
    assert!(
        eval_zkir(&ir, env).is_err(),
        "unzeroed popped frame must fail"
    );

    // Negative: mutated lower frame slot 0 must fail.
    let mut bad_low = sample_return_right_value_fixture();
    bad_low.post.frames[0].rest[0] = 999;
    let (en, pre, post, env) = bad_low.build_symbolic_inputs();
    let ir = emit_k_machine_return_right_add(en, &pre, &post).unwrap();
    assert!(
        eval_zkir(&ir, env).is_err(),
        "mutated lower frame must fail"
    );

    // Negative: wrong post.control_origin must fail.
    let mut bad_orig = sample_return_right_value_fixture();
    bad_orig.post.control_origin = 199;
    let (en, pre, post, env) = bad_orig.build_symbolic_inputs();
    let ir = emit_k_machine_return_right_add(en, &pre, &post).unwrap();
    assert!(
        eval_zkir(&ir, env).is_err(),
        "wrong control_origin must fail"
    );

    // Negative: unzeroed post.result must fail.
    let mut bad_res = sample_return_right_value_fixture();
    bad_res.post.result.val_index = 3;
    let (en, pre, post, env) = bad_res.build_symbolic_inputs();
    let ir = emit_k_machine_return_right_add(en, &pre, &post).unwrap();
    assert!(
        eval_zkir(&ir, env).is_err(),
        "unzeroed post.result must fail"
    );

    // Negative: non-Add BinaryOp on top RightF must fail.
    let mut bad_op = sample_return_right_value_fixture();
    bad_op.pre.frames[1].right_op = BINARY_OP_SUB as u128;
    let (en, pre, post, env) = bad_op.build_symbolic_inputs();
    let ir = emit_k_machine_return_right_add(en, &pre, &post).unwrap();
    assert!(eval_zkir(&ir, env).is_err(), "non-Add BinaryOp must fail");
}

#[test]
fn test_return_right_add_scalar_error_10_and_11_stages_prim_error() {
    for code in [PRIM_ERROR_ARITH_OVERFLOW, PRIM_ERROR_SORT_MISMATCH] {
        let mut fx = sample_return_right_value_fixture();
        fx.pre.result = ConcreteResult::scalar_error(code, 1);
        fx.post.control = ConcreteControl::prim_error(code, 2);
        let (en, pre, post, env) = fx.build_symbolic_inputs();
        let ir = emit_k_machine_return_right_add(en, &pre, &post).expect("emit return.right err");
        eval_zkir(&ir, env).expect("valid ScalarError return.right transition must pass");
    }

    // Negative: disallowed error code 12 must fail.
    let mut bad_code = sample_return_right_value_fixture();
    bad_code.pre.result = ConcreteResult::scalar_error(12, 1);
    bad_code.post.control = ConcreteControl::prim_error(12, 2);
    let (en, pre, post, env) = bad_code.build_symbolic_inputs();
    let ir = emit_k_machine_return_right_add(en, &pre, &post).unwrap();
    assert!(eval_zkir(&ir, env).is_err(), "error code 12 must fail");

    // Negative: EqualDone result tag must fail in Add return.right emitter.
    let mut bad_tag = sample_return_right_value_fixture();
    bad_tag.pre.result.tag = HELPER_RESULT_TAG_EQUAL_DONE as u128;
    let (en, pre, post, env) = bad_tag.build_symbolic_inputs();
    let ir = emit_k_machine_return_right_add(en, &pre, &post).unwrap();
    assert!(
        eval_zkir(&ir, env).is_err(),
        "EqualDone result tag must fail"
    );
}

#[test]
fn test_primitive_value_transitions_prim_result_to_return_and_preserves_frames() {
    let base = sample_return_right_value_fixture();
    let pre = base.post;
    let mut post = pre.clone();
    post.control = ConcreteControl::return_val(VALUE_SPACE_DATA as u128, 3, 2);
    post.remaining_work = 48;
    post.consumed_work = 12;
    post.reference_cursor = 9;

    let fx = LegacyFixture {
        enable: 1,
        pre,
        post,
    };
    let (en, pre_s, post_s, env) = fx.build_symbolic_inputs();
    let ir = emit_k_machine_primitive_value(en, &pre_s, &post_s).expect("emit primitive.value");
    eval_zkir(&ir, env).expect("valid primitive.value transition must pass");

    // Negative: leftover prim_val_index in post.control must fail.
    let mut bad_pv = fx.clone_for_mutation();
    bad_pv.post.control.prim_val_index = 3;
    let (en, pre_s, post_s, env) = bad_pv.build_symbolic_inputs();
    let ir = emit_k_machine_primitive_value(en, &pre_s, &post_s).unwrap();
    assert!(
        eval_zkir(&ir, env).is_err(),
        "unzeroed prim_value must fail"
    );

    // Negative: non-zero inactive frame slot >= depth must fail.
    let mut bad_frame = fx.clone_for_mutation();
    bad_frame.pre.frames[1].origin = 99;
    bad_frame.post.frames[1].origin = 99;
    let (en, pre_s, post_s, env) = bad_frame.build_symbolic_inputs();
    let ir = emit_k_machine_primitive_value(en, &pre_s, &post_s).unwrap();
    assert!(
        eval_zkir(&ir, env).is_err(),
        "non-zero inactive frame slot must fail"
    );
}

impl LegacyFixture {
    fn clone_for_mutation(&self) -> Self {
        Self {
            enable: self.enable,
            pre: self.pre.clone(),
            post: self.post.clone(),
        }
    }
}

#[test]
fn test_primitive_error_10_and_11_transitions_to_control_error_keeping_running_outcome() {
    for code in [PRIM_ERROR_ARITH_OVERFLOW, PRIM_ERROR_SORT_MISMATCH] {
        let base = sample_return_right_value_fixture();
        let mut pre = base.post;
        pre.control = ConcreteControl::prim_error(code, 2);
        let mut post = pre.clone();
        post.control = ConcreteControl::control_error(code, 2);
        post.remaining_work = 48;
        post.consumed_work = 12;
        post.reference_cursor = 9;

        let fx = LegacyFixture {
            enable: 1,
            pre,
            post,
        };
        let (en, pre_s, post_s, env) = fx.build_symbolic_inputs();
        let ir = emit_k_machine_primitive_error(en, &pre_s, &post_s).expect("emit primitive.error");
        eval_zkir(&ir, env).expect("valid primitive.error transition must pass");

        // Negative: prematurely setting post.outcome = OUTCOME_TAG_REJECTED must fail.
        let mut bad_outcome = fx.clone_for_mutation();
        bad_outcome.post.outcome = OUTCOME_TAG_REJECTED as u128;
        let (en, pre_s, post_s, env) = bad_outcome.build_symbolic_inputs();
        let ir = emit_k_machine_primitive_error(en, &pre_s, &post_s).unwrap();
        assert!(
            eval_zkir(&ir, env).is_err(),
            "premature Outcome::Rejected must fail in primitive.error"
        );
    }
}

#[test]
fn test_chained_return_right_into_primitive_value_and_primitive_error_end_to_end() {
    // 1. Value chain: top-level expression (`depth: 1 -> 0 -> 0`).
    let f0 = ConcreteFrame::right_f_add(777, VALUE_SPACE_DATA as u128, 0, 1);
    let st0 = ConcreteLegacyState {
        site: SITE_RET_RIGHT as u128,
        outcome: OUTCOME_TAG_RUNNING as u128,
        control: ConcreteControl::return_val(VALUE_SPACE_DATA as u128, 1, 1),
        control_origin: 555,
        helper_present: 0,
        result: ConcreteResult::scalar_value(VALUE_SPACE_DATA as u128, 2, 1),
        frames: vec![f0],
        depth: 1,
        remaining_work: 10,
        consumed_work: 20,
        helper_work: 3,
        helper_visits: 1,
        reference_cursor: 100,
        rest: vec![42],
    };
    let st1 = ConcreteLegacyState {
        site: SITE_PRIM as u128,
        outcome: OUTCOME_TAG_RUNNING as u128,
        control: ConcreteControl::prim_value(VALUE_SPACE_DATA as u128, 2, 1),
        control_origin: 777,
        helper_present: 0,
        result: ConcreteResult::absent(1),
        frames: vec![ConcreteFrame::zero(1)],
        depth: 0,
        remaining_work: 9,
        consumed_work: 21,
        helper_work: 3,
        helper_visits: 1,
        reference_cursor: 101,
        rest: vec![42],
    };
    let st2 = ConcreteLegacyState {
        site: SITE_PRIM as u128,
        outcome: OUTCOME_TAG_RUNNING as u128,
        control: ConcreteControl::return_val(VALUE_SPACE_DATA as u128, 2, 1),
        control_origin: 777,
        helper_present: 0,
        result: ConcreteResult::absent(1),
        frames: vec![ConcreteFrame::zero(1)],
        depth: 0,
        remaining_work: 8,
        consumed_work: 22,
        helper_work: 3,
        helper_visits: 1,
        reference_cursor: 102,
        rest: vec![42],
    };

    let fx_step1 = LegacyFixture {
        enable: 1,
        pre: st0,
        post: st1.clone(),
    };
    let (en1, pre1, post1, env1) = fx_step1.build_symbolic_inputs();
    let ir1 = emit_k_machine_return_right_add(en1, &pre1, &post1).unwrap();
    eval_zkir(&ir1, env1).expect("step 1 return.right value at depth 1 -> 0 must pass");

    let fx_step2 = LegacyFixture {
        enable: 1,
        pre: st1,
        post: st2,
    };
    let (en2, pre2, post2, env2) = fx_step2.build_symbolic_inputs();
    let ir2 = emit_k_machine_primitive_value(en2, &pre2, &post2).unwrap();
    eval_zkir(&ir2, env2).expect("step 2 primitive.value at depth 0 must pass");

    // 2. Error chains: ScalarError(10 | 11) -> PrimResult::Error -> Control::Error at depth 0.
    for code in [PRIM_ERROR_ARITH_OVERFLOW, PRIM_ERROR_SORT_MISMATCH] {
        let mut step1 = fx_step1.clone_for_mutation();
        step1.pre.result = ConcreteResult::scalar_error(code, 1);
        step1.post.control = ConcreteControl::prim_error(code, 1);
        let mut step2 = fx_step2.clone_for_mutation();
        step2.pre.control = step1.post.control.clone();
        step2.post.control = ConcreteControl::control_error(code, 1);
        let (en, pre, post, env) = step1.build_symbolic_inputs();
        let rr = emit_k_machine_return_right_add_block(en, &pre, &post, "rr").unwrap();
        eval_zkir(&rr.instructions, env).expect("return.right ScalarError depth 1 -> 0 must pass");
        let (en, pre, post, env) = step2.build_symbolic_inputs();
        let pe = emit_k_machine_primitive_error_block(en, &pre, &post, "pe").unwrap();
        assert_eq!(pe.post_site, BPostSiteContract::TerminalP1);
        eval_zkir(&pe.instructions, env).expect("primitive.error at depth 0 must pass");
    }
}

#[test]
fn test_zero_frame_capacity_primitive_rows_at_depth_zero_and_return_right_refusal() {
    let mut pre = sample_return_right_value_fixture().post;
    pre.frames.clear();
    pre.depth = 0;
    let mut post = pre.clone();
    post.control = ConcreteControl::return_val(VALUE_SPACE_DATA as u128, 3, 2);
    post.remaining_work = 48;
    post.consumed_work = 12;
    post.reference_cursor = 9;
    let fx = LegacyFixture {
        enable: 1,
        pre,
        post,
    };
    let (en, pre_s, post_s, env) = fx.build_symbolic_inputs();
    let pv = emit_k_machine_primitive_value_block(en.clone(), &pre_s, &post_s, "pv").unwrap();
    eval_zkir(&pv.instructions, env).expect("cap-0 primitive.value at depth 0 must pass");
    assert_eq!(
        emit_k_machine_return_right_add_block(en, &pre_s, &post_s, "rr"),
        Err(BReturnRightCompletionEmitError::EmptyFrameCapacity)
    );

    let mut pe = fx.clone_for_mutation();
    pe.pre.control = ConcreteControl::prim_error(PRIM_ERROR_SORT_MISMATCH, 2);
    pe.post.control = ConcreteControl::control_error(PRIM_ERROR_SORT_MISMATCH, 2);
    let (en, pre_s, post_s, env) = pe.build_symbolic_inputs();
    let blk = emit_k_machine_primitive_error_block(en, &pre_s, &post_s, "pe").unwrap();
    eval_zkir(&blk.instructions, env).expect("cap-0 primitive.error at depth 0 must pass");

    // depth 1 with no frame slot: first failure is the depth partition (reached 0 != enable 1).
    let mut deep = fx.clone_for_mutation();
    deep.pre.depth = 1;
    deep.post.depth = 1;
    let (en, pre_s, post_s, env) = deep.build_symbolic_inputs();
    let blk = emit_k_machine_primitive_value_block(en, &pre_s, &post_s, "pv").unwrap();
    let err = eval_zkir(&blk.instructions, env).unwrap_err();
    assert!(err.contains("ConstrainEq failed (0 != 1)"), "{err}");
}

#[test]
fn test_boundary_u128_lanes_carry_borrow_and_deepest_frame_slot() {
    let mut fx = sample_return_right_value_fixture();
    // Pop from the highest physical slot `pre.depth == 3` (`cap == 3`).
    fx.pre.frames[2] = ConcreteFrame::right_f_add(909, VALUE_SPACE_DATA as u128, 5, 2);
    fx.pre.depth = 3;
    fx.post.frames[1] = fx.pre.frames[1].clone();
    fx.post.frames[2] = ConcreteFrame::zero(2);
    fx.post.depth = 2;
    fx.post.control_origin = 909;

    // Cross multi-byte carry/borrow boundaries on u128 counters.
    fx.pre.remaining_work = 0x0100_0000;
    fx.post.remaining_work = 0x00FF_FFFF;
    fx.pre.consumed_work = 0x00FF_FFFF;
    fx.post.consumed_work = 0x0100_0000;
    fx.pre.reference_cursor = 0xFFFF_FFFF;
    fx.post.reference_cursor = 0x0001_0000_0000;

    let (en, pre, post, env) = fx.build_symbolic_inputs();
    let ir = emit_k_machine_return_right_add(en, &pre, &post).unwrap();
    eval_zkir(&ir, env).expect("deepest frame slot and multi-byte counter carries must pass");

    // Negative: remaining_work == 0 underflow must fail.
    let mut underflow = fx.clone_for_mutation();
    underflow.pre.remaining_work = 0;
    underflow.post.remaining_work = u128::MAX;
    let (en, pre, post, env) = underflow.build_symbolic_inputs();
    let ir = emit_k_machine_return_right_add(en, &pre, &post).unwrap();
    assert!(
        eval_zkir(&ir, env).is_err(),
        "remaining_work == 0 underflow must fail"
    );

    // Negative: reference_cursor == u128::MAX overflow must fail.
    let mut overflow = fx.clone_for_mutation();
    overflow.pre.reference_cursor = u128::MAX;
    overflow.post.reference_cursor = 0;
    let (en, pre, post, env) = overflow.build_symbolic_inputs();
    let ir = emit_k_machine_return_right_add(en, &pre, &post).unwrap();
    assert!(
        eval_zkir(&ir, env).is_err(),
        "reference_cursor == u128::MAX overflow must fail"
    );
}

#[test]
fn test_disabled_row_stutter_preserves_state_across_all_three_emitters() {
    let base = sample_return_right_value_fixture();
    let stutter = LegacyFixture {
        enable: 0,
        pre: base.pre.clone(),
        post: base.pre,
    };
    let (en, pre, post, env) = stutter.build_symbolic_inputs();
    for ir in [
        emit_k_machine_return_right_add(en.clone(), &pre, &post).unwrap(),
        emit_k_machine_primitive_value(en.clone(), &pre, &post).unwrap(),
        emit_k_machine_primitive_error(en, &pre, &post).unwrap(),
    ] {
        eval_zkir(&ir, env.clone()).expect("disabled stutter must pass");
    }

    // Negative: mutating reference_cursor when enable == 0 must fail.
    let mut bad_stutter = stutter.clone_for_mutation();
    bad_stutter.post.reference_cursor += 1;
    let (en, pre, post, env) = bad_stutter.build_symbolic_inputs();
    let ir = emit_k_machine_return_right_add(en, &pre, &post).unwrap();
    assert!(
        eval_zkir(&ir, env).is_err(),
        "mutated reference_cursor on disabled row must fail"
    );
}

#[test]
fn test_structural_validation_rejects_malformed_frame_capacity_and_rest_mismatch() {
    let fx = sample_return_right_value_fixture();
    let (en, pre, post, _env) = fx.build_symbolic_inputs();

    let mut empty_pre = pre.clone();
    empty_pre.frames.clear();
    assert_eq!(
        emit_k_machine_return_right_add(en.clone(), &empty_pre, &post),
        Err(BReturnRightCompletionEmitError::EmptyFrameCapacity)
    );

    let mut cap_post = post.clone();
    cap_post.frames.pop();
    assert_eq!(
        emit_k_machine_return_right_add(en.clone(), &pre, &cap_post),
        Err(BReturnRightCompletionEmitError::FrameCapacityMismatch {
            pre_len: 3,
            post_len: 2,
        })
    );

    let mut bad_ctrl_post = post.clone();
    bad_ctrl_post.control.rest.push(en.clone());
    assert_eq!(
        emit_k_machine_primitive_value(en, &pre, &bad_ctrl_post),
        Err(BReturnRightCompletionEmitError::ControlRestLengthMismatch {
            pre_len: 2,
            post_len: 3,
        })
    );
}

fn scoped_outputs(ir: &[Instruction]) -> Vec<String> {
    let mut out = Vec::new();
    for inst in ir {
        match inst {
            Instruction::Add { output, .. }
            | Instruction::Mul { output, .. }
            | Instruction::TestEq { output, .. }
            | Instruction::CondSelect { output, .. } => out.push(output.0.clone()),
            Instruction::DivModPowerOfTwo { outputs, .. } => {
                out.extend(outputs.iter().map(|o| o.0.clone()))
            }
            _ => {}
        }
    }
    out
}

#[test]
fn test_return_right_rejects_const_space_scalar_value() {
    let mut fx = sample_return_right_value_fixture();
    fx.pre.result = ConcreteResult::scalar_value(VALUE_SPACE_CONST as u128, 3, 1);
    fx.post.control = ConcreteControl::prim_value(VALUE_SPACE_CONST as u128, 3, 2);
    let (en, pre, post, env) = fx.build_symbolic_inputs();
    let ir = emit_k_machine_return_right_add(en, &pre, &post).unwrap();
    assert!(
        eval_zkir(&ir, env).is_err(),
        "Const-space ScalarValue must fail"
    );
}

#[test]
fn test_post_site_contracts_bind_prim_and_defer_primitive_dispatch() {
    let mut fx = sample_return_right_value_fixture();
    fx.post.site = 999;
    let (en, pre, post, env) = fx.build_symbolic_inputs();
    let rr = emit_k_machine_return_right_add_block(en.clone(), &pre, &post, "rr").unwrap();
    assert_eq!(rr.post_site, BPostSiteContract::Bound(SITE_PRIM));
    assert!(
        eval_zkir(&rr.instructions, env).is_err(),
        "enabled post.site != Prim must fail"
    );
    let pv = emit_k_machine_primitive_value_block(en.clone(), &pre, &post, "pv").unwrap();
    assert_eq!(pv.post_site, BPostSiteContract::ReturnDispatch);
    let pe = emit_k_machine_primitive_error_block(en, &pre, &post, "pe").unwrap();
    assert_eq!(pe.post_site, BPostSiteContract::TerminalP1);
}

#[test]
fn test_scoped_rows_compose_only_with_per_branch_posts() {
    let fx = sample_return_right_value_fixture();
    let (en, pre, post, env) = fx.build_symbolic_inputs();
    let off = Operand::Immediate(Fr::from(0u128));
    let rr = emit_k_machine_return_right_add_block(en, &pre, &post, "rr").unwrap();
    // Disabled sibling takes its own post witness (`pre`), not the active row's `post`.
    let pv = emit_k_machine_primitive_value_block(off.clone(), &pre, &pre, "pv").unwrap();
    for (scope, ir) in [("rr", &rr.instructions), ("pv", &pv.instructions)] {
        assert!(
            scoped_outputs(ir)
                .iter()
                .all(|n| is_legacy_completion_scope_variable(scope, n))
        );
        assert!(
            env.keys()
                .all(|k| !is_legacy_completion_scope_variable(scope, k))
        );
    }
    let joined = [rr.instructions.clone(), pv.instructions].concat();
    eval_zkir(&joined, env.clone())
        .expect("active row plus disabled sibling on own post must pass");

    let shared = emit_k_machine_primitive_value_block(off.clone(), &pre, &post, "pv").unwrap();
    let joined = [rr.instructions, shared.instructions].concat();
    assert!(
        eval_zkir(&joined, env).is_err(),
        "disabled sibling sharing active post must fail"
    );

    for bad in ["", "_a", "a_", "a__b", "a-b"] {
        assert_eq!(
            emit_k_machine_primitive_error_block(off.clone(), &pre, &pre, bad),
            Err(BReturnRightCompletionEmitError::InvalidScope)
        );
    }
}

/// Pinned SDK JSON boundary as in the public backend and literal bridge: serde_json, version
/// `3.0`, `IrSource::load`, structural equality. Not a check, compile or proof.
fn assert_sdk_json_roundtrip(code: Vec<Instruction>) {
    let ir = IrSource {
        version: Default::default(),
        inputs: vec![],
        outputs: vec![],
        do_communications_commitment: false,
        instructions: Arc::new(code),
    };
    let mut disk = serde_json::to_value(&ir).expect("serde_json serialization");
    disk["version"] = serde_json::json!({"major": 3, "minor": 0});
    let zkir = serde_json::to_string(&disk).expect("serde_json serialization");
    let parsed = IrSource::load(zkir.as_bytes()).expect("pinned SDK IrSource::load");
    assert_eq!(parsed, ir, "pinned SDK JSON round-trip changed the IR");
}

#[test]
fn test_legacy_outputs_are_percent_scoped_and_roundtrip_sdk_json() {
    let base = sample_return_right_value_fixture();
    let stutter = LegacyFixture {
        enable: 0,
        pre: base.pre.clone(),
        post: base.pre,
    };
    let (en, pre, post, env) = stutter.build_symbolic_inputs();
    let blocks = [
        (
            "k_ret_right_add",
            emit_k_machine_return_right_add(en.clone(), &pre, &post).unwrap(),
        ),
        (
            "k_prim_val",
            emit_k_machine_primitive_value(en.clone(), &pre, &post).unwrap(),
        ),
        (
            "k_prim_err",
            emit_k_machine_primitive_error(en.clone(), &pre, &post).unwrap(),
        ),
        (
            "rr",
            emit_k_machine_return_right_add_block(en.clone(), &pre, &post, "rr")
                .unwrap()
                .instructions,
        ),
        (
            "rr_2",
            emit_k_machine_return_right_add_block(en.clone(), &pre, &post, "rr_2")
                .unwrap()
                .instructions,
        ),
        (
            "pv",
            emit_k_machine_primitive_value_block(en.clone(), &pre, &post, "pv")
                .unwrap()
                .instructions,
        ),
        (
            "pe",
            emit_k_machine_primitive_error_block(en, &pre, &post, "pe")
                .unwrap()
                .instructions,
        ),
    ];
    for (scope, ir) in &blocks {
        let outs = scoped_outputs(ir);
        assert_eq!(outs[0], format!("%{scope}__not_enable_0"));
        for (other, _) in &blocks {
            let owned = outs
                .iter()
                .filter(|n| is_legacy_completion_scope_variable(other, n))
                .count();
            assert_eq!(
                owned,
                if other == scope { outs.len() } else { 0 },
                "{scope} vs {other}"
            );
        }
        assert!(
            env.keys()
                .all(|k| !is_legacy_completion_scope_variable(scope, k))
        );
        // The pre-correction unprefixed spelling of an actual output fails SDK serde decoding.
        let old =
            serde_json::to_value(Operand::Variable(Identifier(outs[0][1..].to_string()))).unwrap();
        assert!(
            serde_json::from_value::<Operand>(old).is_err(),
            "{}",
            outs[0]
        );
        let new = serde_json::to_value(Operand::Variable(Identifier(outs[0].clone()))).unwrap();
        assert_eq!(
            serde_json::from_value::<Operand>(new).unwrap(),
            Operand::Variable(Identifier(outs[0].clone()))
        );
    }
    let joined: Vec<Instruction> = blocks.into_iter().flat_map(|(_, ir)| ir).collect();
    let mut seen = std::collections::BTreeSet::new();
    assert!(
        scoped_outputs(&joined).into_iter().all(|n| seen.insert(n)),
        "outputs defined once"
    );
    eval_zkir(&joined, env).expect("disabled rows across distinct scopes must pass");
    assert_sdk_json_roundtrip(joined);
}

type LegacyBlockEmit = fn(
    Operand,
    &BLegacyCompletionState,
    &BLegacyCompletionState,
    &str,
) -> Result<BLegacyCompletionBlock, BReturnRightCompletionEmitError>;

fn legacy_ctor_err(
    emit: LegacyBlockEmit,
    scope: &str,
    fx: &LegacyFixture,
    mutate: impl Fn(&mut Operand, &mut BLegacyCompletionState, &mut BLegacyCompletionState),
) -> Option<BReturnRightCompletionEmitError> {
    let (mut en, mut pre, mut post, _) = fx.build_symbolic_inputs();
    mutate(&mut en, &mut pre, &mut post);
    emit(en, &pre, &post, scope).err()
}

#[test]
fn test_legacy_emitters_reject_unprefixed_and_reserved_caller_operands() {
    let fx = sample_return_right_value_fixture();
    let var = |name: &str| Operand::Variable(Identifier(name.to_string()));
    let collision = Some(BReturnRightCompletionEmitError::TemporaryNameCollision);
    let unprefixed = Some(BReturnRightCompletionEmitError::UnprefixedVariable);
    for (emit, scope) in [
        (
            emit_k_machine_return_right_add_block as LegacyBlockEmit,
            "rr",
        ),
        (
            emit_k_machine_primitive_value_block as LegacyBlockEmit,
            "pv",
        ),
        (
            emit_k_machine_primitive_error_block as LegacyBlockEmit,
            "pe",
        ),
    ] {
        let (en, pre, post, _) = fx.build_symbolic_inputs();
        let produced = scoped_outputs(&emit(en, &pre, &post, scope).unwrap().instructions);
        let (first, last) = (produced[0].as_str(), produced[produced.len() - 1].as_str());
        assert_eq!(first, format!("%{scope}__not_enable_0"));
        assert!(!is_legacy_completion_scope_variable(scope, &first[1..]));
        // Actual freshly emitted names are reserved in every carrier region, inactive lanes included.
        assert_eq!(
            legacy_ctor_err(emit, scope, &fx, |en, _, _| *en = var(first)),
            collision
        );
        assert_eq!(
            legacy_ctor_err(emit, scope, &fx, |_, pre, _| pre.frames[2].rest[1] =
                var(last)),
            collision
        );
        assert_eq!(
            legacy_ctor_err(emit, scope, &fx, |_, pre, _| pre.helper.staged.u128_value
                [7] = var(first)),
            collision
        );
        assert_eq!(
            legacy_ctor_err(emit, scope, &fx, |_, _, post| post.result.rest[0] =
                var(last)),
            collision
        );
        assert_eq!(
            legacy_ctor_err(emit, scope, &fx, |_, _, post| post.control.rest[1] =
                var(first)),
            collision
        );
        assert_eq!(
            legacy_ctor_err(emit, scope, &fx, |_, _, post| post.reference_cursor[15] =
                var(last)),
            collision
        );
        // The pre-correction unprefixed spelling is not an SDK serde variable.
        assert_eq!(
            legacy_ctor_err(emit, scope, &fx, |_, _, post| post.site = var(&first[1..])),
            unprefixed
        );
        // First offending operand in check order (`enable`, then `pre`, then `post`) wins.
        let both = |head: &str, tail: &str| {
            legacy_ctor_err(emit, scope, &fx, |en, _, post| {
                *en = var(head);
                post.rest[1] = var(tail);
            })
        };
        assert_eq!(both("bad", first), unprefixed);
        assert_eq!(both(first, "bad"), collision);
        // Names outside the exact `%{scope}__` namespace stay admitted, including another block's
        // outputs (`%{scope}_2__*`): cross-block uniqueness is the IrSource owner's premise.
        for name in [
            format!("%{scope}_2__not_enable_0"),
            format!("%x_{scope}__not_enable_0"),
            format!("%{scope}.local.temp.0"),
            format!("%{scope}_"),
        ] {
            let name = name.as_str();
            assert!(
                legacy_ctor_err(emit, scope, &fx, |_, _, post| post.site = var(name)).is_none(),
                "{name}"
            );
        }
    }
    // Fixed-scope standalone forms carry the same reserved namespace and check.
    let (en, pre, post, _) = fx.build_symbolic_inputs();
    let fixed = scoped_outputs(&emit_k_machine_primitive_error(en.clone(), &pre, &post).unwrap());
    assert_eq!(fixed[0], "%k_prim_err__not_enable_0");
    let mut forged = post.clone();
    forged.depth = var(fixed[0].as_str());
    assert_eq!(
        emit_k_machine_primitive_error(en, &pre, &forged),
        Err(BReturnRightCompletionEmitError::TemporaryNameCollision)
    );
}
