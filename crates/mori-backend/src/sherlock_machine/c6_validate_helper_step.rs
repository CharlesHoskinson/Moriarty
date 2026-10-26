//! C6 Validate helper step: the
//! `b.helper.validate.{entry,occurrence,empty,exit}` rows factorial and
//! fold fire (literal validation at an EvalLiteral site) as one per-rule
//! relation over the full C6 carriers.
//!
//! Each circuit fixes its rule at build time, selects it with the composed
//! dispatch owner and writes the post state exactly as
//! `families/structural.rs` plans it:
//! - entry (control Eval { expr } of a Literal { constant } PROG row):
//!   Validate { common(EvalLiteral), root = Const(constant), Pending }
//!   and one ValidateTask { Const(constant), Unvisited, 0 } pushed;
//! - occurrence: the top ValidateTask is unvisited and not a Marker (the
//!   dispatch owner checked); a leaf (checked arity 0) pops it, otherwise
//!   it is rewritten to { r, Fields, 0 };
//! - empty: no live tasks while Pending, so the status becomes Positive;
//! - exit: Positive, so the result becomes Validated { ok: true } and
//!   the helper is cleared.
//!
//! Charge is the applied helper charge (visits 1 for occurrence, 0
//! otherwise). Not constrained here, so rejected: the P3 helper-work
//! refusal and the task cap refusal. Every unnamed lane is preserved;
//! canonicality is composed; enable = 0 is identity.
use midnight_zkir_v3::Instruction;
use midnight_zkir_v3::ir::Operand;
use mori_core::sherlock::helper_machine::c6_phys::{CONTROL_LANES, L_CONTROL, L_REFERENCE_CURSOR};
use mori_core::sherlock::helper_machine::c6_phys_helper::{
    HELPER_LANES, L_HELPER_RECORD, L_RESULT_RECORD, L_TASKS, RESULT_LANES, TASK_LANES,
};
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_checked_value_read::{C6CheckedValueReadEmitter, C6CheckedValueReadInputs};
use super::c6_program_context_canonical::C6ProgramContextExpressionCaptureInputs;
use super::c6_step_kit::{
    C6StepInputs, Composed, Em, charge_helper, check_shapes, compose_canonical, disabled_identity,
    helper_charge_lanes, prefix, preserve_image, select, validate_pre_image,
};

#[derive(Debug, Error)]
pub enum C6ValidateHelperError {
    #[error("rule {0:?} is not a Validate helper rule of this fragment")]
    Unsupported(RuleIdC),
    #[error("invalid scope")]
    InvalidScope,
    #[error("C6 Validate helper emission: {0}")]
    Emit(String),
}

pub const VALIDATE_HELPER_RULES: [RuleIdC; 4] = [
    RuleIdC::BHelperValidateEntry,
    RuleIdC::BHelperValidateOccurrence,
    RuleIdC::BHelperValidateEmpty,
    RuleIdC::BHelperValidateExit,
];

pub fn validate_helper_supported(rule: RuleIdC) -> bool {
    VALIDATE_HELPER_RULES.contains(&rule)
}

const K_VALIDATE: u128 = 7;
const SITE_EVAL_LITERAL: u128 = 0;
const R_VALIDATED: u128 = 1;
const T_VALIDATE_TASK: u128 = 0;
const EXPR_LITERAL: u128 = 0;
const TRI_POSITIVE: u128 = 1;
// Validate record offsets after the 5-lane head: root 5..7, status 7.
const V_ROOT: usize = 5;
const V_STATUS: usize = 7;
/// HELPER header lane of the live task count.
const H_TASK_COUNT: usize = 3;

#[derive(Clone, Debug)]
pub struct C6ValidateHelperBlock {
    pub instructions: Vec<Instruction>,
}

pub struct C6ValidateHelperEmitter {
    scope: String,
    next_temp: usize,
    composed: Composed,
    reader: C6CheckedValueReadEmitter,
}

/// Task-table rewrite: optional push of `row` at `count`, optional gated
/// pop of the top row, optional gated rewrite of the top row.
struct TaskOp {
    push: Option<Vec<Operand>>,
    pop: Option<Operand>,
    rewrite: Option<(Operand, Vec<Operand>)>,
}

impl C6ValidateHelperEmitter {
    pub fn new(scope: &str) -> Result<Self, C6ValidateHelperError> {
        if scope.trim().is_empty() || scope.contains('%') || scope.contains(".c6kit.") {
            return Err(C6ValidateHelperError::InvalidScope);
        }
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            composed: Composed::new(scope).map_err(C6ValidateHelperError::Emit)?,
            reader: C6CheckedValueReadEmitter::new(&format!("{scope}.validateread"))
                .map_err(|e| C6ValidateHelperError::Emit(e.to_string()))?,
        })
    }

    pub fn emit(
        &mut self,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<C6ValidateHelperBlock, C6ValidateHelperError> {
        if !validate_helper_supported(rule) {
            return Err(C6ValidateHelperError::Unsupported(rule));
        }
        check_shapes(inputs).map_err(C6ValidateHelperError::Emit)?;
        let scope = self.scope.clone();
        let mut em = Em {
            scope: &scope,
            next: self.next_temp,
            ins: Vec::new(),
        };
        let r = self.body(&mut em, rule, inputs);
        self.next_temp = em.next;
        r.map_err(C6ValidateHelperError::Emit)?;
        Ok(C6ValidateHelperBlock {
            instructions: em.ins,
        })
    }

    fn body(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        let en = inputs.enable.clone();
        let caps = inputs.geometry.caps;
        let max_tasks = inputs.geometry.helper_tasks_lanes / TASK_LANES;
        prefix(em, inputs);
        let _top = validate_pre_image(em, inputs);
        select(&mut self.composed, em, rule, inputs)?;

        let pre_i = &inputs.pre_image;
        let k = &pre_i[L_CONTROL..L_CONTROL + CONTROL_LANES];
        let pre_h = &inputs.pre_helper;
        let rec = |o: usize| pre_h[L_HELPER_RECORD + o].clone();
        let count = pre_h[H_TASK_COUNT].clone();
        let mut h_out: Vec<Option<Operand>> = vec![None; HELPER_LANES];
        let mut r_out: Option<[Operand; RESULT_LANES]> = None;
        let mut tasks = TaskOp {
            push: None,
            pop: None,
            rewrite: None,
        };
        let is_validate = |em: &mut Em| em.geq(en.clone(), rec(0), Em::imm(K_VALIDATE));
        // The top task row (index count - 1), one-hot muxed.
        let top_task = |em: &mut Em| {
            let some = em.lt128(Em::imm(0), count.clone());
            em.gassert(en.clone(), some);
            let mut out = vec![Em::imm(0); TASK_LANES];
            for r in 0..max_tasks {
                let hit = em.teq(Em::imm(r as u128 + 1), count.clone());
                for (c, o) in out.iter_mut().enumerate() {
                    *o = em.sel(
                        hit.clone(),
                        pre_h[L_TASKS + r * TASK_LANES + c].clone(),
                        o.clone(),
                    );
                }
            }
            em.geq(en.clone(), out[0].clone(), Em::imm(T_VALIDATE_TASK));
            out
        };
        let visits;
        match rule {
            RuleIdC::BHelperValidateEntry => {
                // Eval { expr } of a Literal { constant } PROG row.
                em.geq(en.clone(), k[0].clone(), Em::imm(0));
                let cap = self
                    .composed
                    .progctx_em
                    .emit_prog_expression_capture(&C6ProgramContextExpressionCaptureInputs {
                        gate: en.clone(),
                        pre_prog: inputs.pre_prog.clone(),
                        caps,
                        selected_index: k[1].clone(),
                    })
                    .map_err(|e| e.to_string())?;
                em.ins.extend(cap.instructions);
                em.gassert(en.clone(), cap.selected_found.clone());
                em.geq(en.clone(), cap.row[0].clone(), Em::imm(EXPR_LITERAL));
                let constant = cap.row[1].clone();
                for slot in h_out.iter_mut() {
                    *slot = Some(Em::imm(0));
                }
                h_out[0] = Some(Em::imm(K_VALIDATE));
                h_out[1] = Some(Em::imm(SITE_EVAL_LITERAL));
                h_out[2] = Some(pre_i[L_REFERENCE_CURSOR].clone());
                // RefC::Const(constant): kind 0, index constant.
                h_out[V_ROOT] = Some(Em::imm(0));
                h_out[V_ROOT + 1] = Some(constant.clone());
                tasks.push = Some(vec![
                    Em::imm(T_VALIDATE_TASK),
                    Em::imm(0),
                    constant,
                    Em::imm(0),
                    Em::imm(0),
                    Em::imm(0),
                    Em::imm(0),
                ]);
                visits = 0;
            }
            RuleIdC::BHelperValidateOccurrence => {
                is_validate(em);
                let task = top_task(em);
                let read = self
                    .reader
                    .emit(&C6CheckedValueReadInputs {
                        gate: en.clone(),
                        ref_kind_is_data: task[1].clone(),
                        ref_index: task[2].clone(),
                        pre_prog: inputs.pre_prog.clone(),
                        pre_data: inputs.pre_data.clone(),
                        caps,
                    })
                    .map_err(|e| e.to_string())?;
                em.ins.extend(read.instructions);
                em.gassert(en.clone(), read.selected_found.clone());
                em.gzero(en.clone(), read.is_marker.clone());
                // Leaf: checked arity of the visited value is zero.
                let leaf = em.teq(read.slice_len.clone(), Em::imm(0));
                let branch = em.not(leaf.clone());
                tasks.pop = Some(leaf);
                tasks.rewrite = Some((
                    branch,
                    vec![
                        Em::imm(T_VALIDATE_TASK),
                        task[1].clone(),
                        task[2].clone(),
                        Em::imm(1),
                        Em::imm(0),
                        Em::imm(0),
                        Em::imm(0),
                    ],
                ));
                visits = 1;
            }
            RuleIdC::BHelperValidateEmpty => {
                is_validate(em);
                h_out[V_STATUS] = Some(Em::imm(TRI_POSITIVE));
                visits = 0;
            }
            RuleIdC::BHelperValidateExit => {
                is_validate(em);
                em.geq(en.clone(), rec(V_STATUS), Em::imm(TRI_POSITIVE));
                for slot in h_out.iter_mut() {
                    *slot = Some(Em::imm(0));
                }
                r_out = Some([Em::imm(R_VALIDATED), Em::imm(1), Em::imm(0), Em::imm(0)]);
                visits = 0;
            }
            _ => unreachable!("checked by validate_helper_supported"),
        }
        // HELPER post: record, result, task table and count.
        let task_lanes = L_TASKS..L_TASKS + max_tasks * TASK_LANES;
        let post_h = &inputs.post_helper;
        for (lane, (a, b)) in pre_h.iter().zip(post_h.iter()).enumerate() {
            let want = if (L_HELPER_RECORD..L_HELPER_RECORD + HELPER_LANES).contains(&lane) {
                h_out[lane - L_HELPER_RECORD]
                    .clone()
                    .unwrap_or_else(|| a.clone())
            } else if (L_RESULT_RECORD..L_RESULT_RECORD + RESULT_LANES).contains(&lane) {
                r_out
                    .as_ref()
                    .map(|r| r[lane - L_RESULT_RECORD].clone())
                    .unwrap_or_else(|| a.clone())
            } else if lane == H_TASK_COUNT {
                let mut c = count.clone();
                if let Some(_row) = &tasks.push {
                    c = em.add(c, Em::imm(1));
                }
                if let Some(g) = &tasks.pop {
                    // post + pop = pre (no underflow: pop needs a top task).
                    let back = em.add(b.clone(), g.clone());
                    em.geq(en.clone(), back, count.clone());
                    continue;
                }
                c
            } else if task_lanes.contains(&lane) {
                let r = (lane - L_TASKS) / TASK_LANES;
                let c = (lane - L_TASKS) % TASK_LANES;
                let mut want = a.clone();
                if let Some(row) = &tasks.push {
                    let at = em.teq(Em::imm(r as u128), count.clone());
                    want = em.sel(at, row[c].clone(), want);
                }
                if let Some(g) = &tasks.pop {
                    let at = em.teq(Em::imm(r as u128 + 1), count.clone());
                    let hit = em.and(at, g.clone());
                    want = em.sel(hit, Em::imm(0), want);
                }
                if let Some((g, row)) = &tasks.rewrite {
                    let at = em.teq(Em::imm(r as u128 + 1), count.clone());
                    let hit = em.and(at, g.clone());
                    want = em.sel(hit, row[c].clone(), want);
                }
                want
            } else {
                a.clone()
            };
            em.geq(en.clone(), b.clone(), want);
        }
        let mutable = helper_charge_lanes();
        preserve_image(em, inputs, &mutable, false);
        charge_helper(em, inputs, visits);
        for (a, b) in inputs.pre_data.iter().zip(inputs.post_data.iter()) {
            em.geq(en.clone(), b.clone(), a.clone());
        }
        compose_canonical(&mut self.composed, em, inputs)?;
        disabled_identity(em, inputs);
        Ok(())
    }
}

/// One-shot emission of the Validate helper relation for `rule`.
pub fn emit_c6_validate_helper_block(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<C6ValidateHelperBlock, C6ValidateHelperError> {
    C6ValidateHelperEmitter::new(scope)?.emit(rule, inputs)
}
