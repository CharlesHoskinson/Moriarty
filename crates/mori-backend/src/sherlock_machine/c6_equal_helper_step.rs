//! C6 Equal helper step: the `b.helper.equal.{entry,pair,different,exit}`
//! rows factorial and fold fire (structural equality at a RetRight site)
//! as one per-rule relation over the full C6 carriers.
//!
//! Each circuit fixes its rule at build time, selects it with the composed
//! dispatch owner and writes the post state exactly as
//! `families/structural.rs` plans it:
//! - entry (control Return { value: right }, top frame RightF { left }):
//!   Equal { common(RetRight), left, right, Pending } and one EqualTask
//!   { left, right, Unvisited, 0 } pushed;
//! - pair: the top EqualTask is shallow-equal (the dispatch owner checked
//!   kind and payloads); a leaf (checked arity 0 of the left value) pops
//!   it, otherwise it is rewritten to { left, right, Fields, 0 };
//! - different: the top EqualTask is shallow-different (dispatch owner);
//!   it pops and the status becomes Negative;
//! - exit (Positive, or Negative with no live tasks): allocates the Bool
//!   data node `status == Positive` at the active DATA count, sets result
//!   EqualDone { Data(active) } and clears the helper.
//!
//! Charge is the applied helper charge (visits 1 for pair/different, 0
//! otherwise). Not constrained here, so rejected: the P3 helper-work
//! refusal and the task/data cap refusals. Every unnamed lane is
//! preserved; canonicality is composed; enable = 0 is identity.
use midnight_zkir_v3::Instruction;
use midnight_zkir_v3::ir::Operand;
use mori_core::sherlock::helper_machine::c6_phys::{
    CONTROL_LANES, L_ACTIVE, L_ALLOC, L_CONTROL, L_REFERENCE_CURSOR,
};
use mori_core::sherlock::helper_machine::c6_phys_data::{DATA_HEADER_LANES, DATA_ROW_LANES};
use mori_core::sherlock::helper_machine::c6_phys_helper::{
    HELPER_LANES, L_HELPER_RECORD, L_RESULT_RECORD, L_TASKS, RESULT_LANES, TASK_LANES,
};
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_checked_value_read::{C6CheckedValueReadEmitter, C6CheckedValueReadInputs};
use super::c6_step_kit::{
    C6StepInputs, Composed, Em, charge_helper, check_shapes, compose_canonical, disabled_identity,
    helper_charge_lanes, prefix, preserve_image, select, validate_pre_image,
};

#[derive(Debug, Error)]
pub enum C6EqualHelperError {
    #[error("rule {0:?} is not an Equal helper rule of this fragment")]
    Unsupported(RuleIdC),
    #[error("invalid scope")]
    InvalidScope,
    #[error("C6 Equal helper emission: {0}")]
    Emit(String),
}

pub const EQUAL_HELPER_RULES: [RuleIdC; 4] = [
    RuleIdC::BHelperEqualEntry,
    RuleIdC::BHelperEqualPair,
    RuleIdC::BHelperEqualDifferent,
    RuleIdC::BHelperEqualExit,
];

pub fn equal_helper_supported(rule: RuleIdC) -> bool {
    EQUAL_HELPER_RULES.contains(&rule)
}

const K_EQUAL: u128 = 8;
const SITE_RET_RIGHT: u128 = 11;
const R_EQUAL_DONE: u128 = 14;
const FRAME_RIGHT: u128 = 2;
const T_EQUAL_TASK: u128 = 1;
const TRI_POSITIVE: u128 = 1;
const TRI_NEGATIVE: u128 = 2;
// Equal record offsets after the 5-lane head: left 5..7, right 7..9, status 9.
const E_LEFT: usize = 5;
const E_RIGHT: usize = 7;
const E_STATUS: usize = 9;
/// HELPER header lane of the live task count.
const H_TASK_COUNT: usize = 3;
/// DATA header lane of the data-node count.
const DATA_NODE_COUNT: usize = 3;

#[derive(Clone, Debug)]
pub struct C6EqualHelperBlock {
    pub instructions: Vec<Instruction>,
}

pub struct C6EqualHelperEmitter {
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

impl C6EqualHelperEmitter {
    pub fn new(scope: &str) -> Result<Self, C6EqualHelperError> {
        if scope.trim().is_empty() || scope.contains('%') || scope.contains(".c6kit.") {
            return Err(C6EqualHelperError::InvalidScope);
        }
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            composed: Composed::new(scope).map_err(C6EqualHelperError::Emit)?,
            reader: C6CheckedValueReadEmitter::new(&format!("{scope}.equalread"))
                .map_err(|e| C6EqualHelperError::Emit(e.to_string()))?,
        })
    }

    pub fn emit(
        &mut self,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<C6EqualHelperBlock, C6EqualHelperError> {
        if !equal_helper_supported(rule) {
            return Err(C6EqualHelperError::Unsupported(rule));
        }
        check_shapes(inputs).map_err(C6EqualHelperError::Emit)?;
        let scope = self.scope.clone();
        let mut em = Em {
            scope: &scope,
            next: self.next_temp,
            ins: Vec::new(),
        };
        let r = self.body(&mut em, rule, inputs);
        self.next_temp = em.next;
        r.map_err(C6EqualHelperError::Emit)?;
        Ok(C6EqualHelperBlock {
            instructions: em.ins,
        })
    }

    fn body(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        let en = inputs.enable.clone();
        let caps = inputs.geometry.caps;
        let max_tasks = inputs.geometry.helper_tasks_lanes / TASK_LANES;
        prefix(em, inputs);
        let top = validate_pre_image(em, inputs);
        select(&mut self.composed, em, rule, inputs)?;

        let pre_i = &inputs.pre_image;
        let post_i = &inputs.post_image;
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
        let mut image_extra: Vec<usize> = Vec::new();
        let mut data_writes = false;
        let is_equal = |em: &mut Em| em.geq(en.clone(), rec(0), Em::imm(K_EQUAL));
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
            em.geq(en.clone(), out[0].clone(), Em::imm(T_EQUAL_TASK));
            out
        };
        let visits;
        match rule {
            RuleIdC::BHelperEqualEntry => {
                // Return { right } under a RightF { left } top frame.
                em.geq(en.clone(), k[0].clone(), Em::imm(1));
                em.geq(en.clone(), top[0].clone(), Em::imm(FRAME_RIGHT));
                for slot in h_out.iter_mut() {
                    *slot = Some(Em::imm(0));
                }
                h_out[0] = Some(Em::imm(K_EQUAL));
                h_out[1] = Some(Em::imm(SITE_RET_RIGHT));
                h_out[2] = Some(pre_i[L_REFERENCE_CURSOR].clone());
                h_out[E_LEFT] = Some(top[2].clone());
                h_out[E_LEFT + 1] = Some(top[3].clone());
                h_out[E_RIGHT] = Some(k[1].clone());
                h_out[E_RIGHT + 1] = Some(k[2].clone());
                tasks.push = Some(vec![
                    Em::imm(T_EQUAL_TASK),
                    top[2].clone(),
                    top[3].clone(),
                    k[1].clone(),
                    k[2].clone(),
                    Em::imm(0),
                    Em::imm(0),
                ]);
                visits = 0;
            }
            RuleIdC::BHelperEqualPair => {
                is_equal(em);
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
                // Leaf: checked arity of the left value is zero.
                let leaf = em.teq(read.slice_len.clone(), Em::imm(0));
                let branch = em.not(leaf.clone());
                tasks.pop = Some(leaf);
                tasks.rewrite = Some((
                    branch,
                    vec![
                        Em::imm(T_EQUAL_TASK),
                        task[1].clone(),
                        task[2].clone(),
                        task[3].clone(),
                        task[4].clone(),
                        Em::imm(1),
                        Em::imm(0),
                    ],
                ));
                visits = 1;
            }
            RuleIdC::BHelperEqualDifferent => {
                is_equal(em);
                let _task = top_task(em);
                tasks.pop = Some(Em::imm(1));
                h_out[E_STATUS] = Some(Em::imm(TRI_NEGATIVE));
                visits = 1;
            }
            RuleIdC::BHelperEqualExit => {
                is_equal(em);
                let positive = em.teq(rec(E_STATUS), Em::imm(TRI_POSITIVE));
                // Allocate Bool { status == Positive } at the active count.
                let active = inputs.pre_data[DATA_NODE_COUNT].clone();
                let fits = em.lt128(active.clone(), Em::imm(caps.phys.max_data_nodes as u128));
                em.gassert(en.clone(), fits);
                let next = em.add(active.clone(), Em::imm(1));
                em.geq(
                    en.clone(),
                    inputs.post_data[DATA_NODE_COUNT].clone(),
                    next.clone(),
                );
                em.geq(en.clone(), post_i[L_ACTIVE].clone(), next.clone());
                em.geq(en.clone(), post_i[L_ALLOC].clone(), next);
                image_extra.extend([L_ACTIVE, L_ALLOC]);
                let base = DATA_HEADER_LANES;
                let row = [Em::imm(1), positive, Em::imm(0), Em::imm(0), Em::imm(0)];
                for r in 0..caps.phys.max_data_nodes {
                    let hit = em.teq(Em::imm(r as u128), active.clone());
                    for (c, v) in row.iter().enumerate() {
                        let lane = base + r * DATA_ROW_LANES + c;
                        let want = em.sel(hit.clone(), v.clone(), inputs.pre_data[lane].clone());
                        em.geq(en.clone(), inputs.post_data[lane].clone(), want);
                    }
                }
                let nodes = base..base + caps.phys.max_data_nodes * DATA_ROW_LANES;
                for (lane, (a, b)) in inputs
                    .pre_data
                    .iter()
                    .zip(inputs.post_data.iter())
                    .enumerate()
                {
                    if lane != DATA_NODE_COUNT && !nodes.contains(&lane) {
                        em.geq(en.clone(), b.clone(), a.clone());
                    }
                }
                data_writes = true;
                for slot in h_out.iter_mut() {
                    *slot = Some(Em::imm(0));
                }
                r_out = Some([Em::imm(R_EQUAL_DONE), Em::imm(1), active, Em::imm(0)]);
                visits = 0;
            }
            _ => unreachable!("checked by equal_helper_supported"),
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
        let mut mutable = helper_charge_lanes();
        mutable.extend(image_extra);
        preserve_image(em, inputs, &mutable, false);
        charge_helper(em, inputs, visits);
        if !data_writes {
            for (a, b) in inputs.pre_data.iter().zip(inputs.post_data.iter()) {
                em.geq(en.clone(), b.clone(), a.clone());
            }
        }
        compose_canonical(&mut self.composed, em, inputs)?;
        disabled_identity(em, inputs);
        Ok(())
    }
}

/// One-shot emission of the Equal helper relation for `rule`.
pub fn emit_c6_equal_helper_block(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<C6EqualHelperBlock, C6EqualHelperError> {
    C6EqualHelperEmitter::new(scope)?.emit(rule, inputs)
}
