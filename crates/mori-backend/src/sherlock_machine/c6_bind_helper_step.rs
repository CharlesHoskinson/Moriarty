//! C6 Bind helper step: the `b.helper.bind.{entry,pair,empty,type,insert,exit}`
//! rows factorial and fold fire (parameter binding on function entry) as one
//! per-rule relation over the full C6 carriers.
//!
//! Each circuit fixes its rule at build time, selects it with the composed
//! dispatch owner and writes the post state exactly as
//! `families/structural.rs` plans it:
//! - entry (control Enter { Some(f), args }): Bind { common(Enter), f,
//!   params of the PROG function row, args, cursors 0, partial/dup None,
//!   Pairing, Ok };
//! - pair: pushes PairTask { binder, sort } of the PROG parameter at
//!   params.start + p_cursor with the DATA value slot at args.start +
//!   a_cursor; both cursors + 1;
//! - empty: UnwindType when tasks remain, else Finished;
//! - type: the `GBindTypeReject` guard (checked sort of the top task's arg
//!   differs from its sort) pops the task with Bad status (Propagate or
//!   Finished); otherwise dup_cursor = partial and UnwindDup or UnwindInsert;
//! - insert: allocates env cell { binder, arg, previous = partial } at the
//!   active env length (DATA row and count, active and alloc env + 1), pops
//!   the task, partial = that cell, UnwindType or Finished;
//! - exit (Finished, Ok): result BindOk { env: partial }, helper cleared.
//!
//! Charge is the applied helper charge (visits 1 for pair/type/insert). Not
//! constrained here, so rejected: the P3 helper-work refusal, the P6 task and
//! environment cap refusals, and Constructor or Marker arguments at type (the
//! ADT checked sort). Every unnamed lane is preserved; canonicality is
//! composed; enable = 0 is identity.
use midnight_zkir_v3::Instruction;
use midnight_zkir_v3::ir::Operand;
use mori_core::sherlock::helper_machine::c6_phys::{
    CONTROL_LANES, L_ACTIVE, L_ALLOC, L_CONTROL, L_REFERENCE_CURSOR,
};
use mori_core::sherlock::helper_machine::c6_phys_data::{ENV_ROW_LANES, REF_LANES};
use mori_core::sherlock::helper_machine::c6_phys_helper::{
    HELPER_LANES, L_HELPER_RECORD, L_RESULT_RECORD, L_TASKS, RESULT_LANES, TASK_LANES,
};
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_checked_value_read::{C6CheckedValueReadEmitter, C6CheckedValueReadInputs};
use super::c6_program_context_canonical::{
    C6ProgramContextFunctionCaptureInputs, C6ProgramContextParameterCaptureInputs,
};
use super::c6_step_kit::{
    C6StepInputs, Composed, Em, charge_helper, check_shapes, compose_canonical, disabled_identity,
    helper_charge_lanes, mux_row, prefix, preserve_image, select, validate_pre_image,
};

#[derive(Debug, Error)]
pub enum C6BindHelperError {
    #[error("rule {0:?} is not a Bind helper rule of this fragment")]
    Unsupported(RuleIdC),
    #[error("invalid scope")]
    InvalidScope,
    #[error("C6 Bind helper emission: {0}")]
    Emit(String),
}

pub const BIND_HELPER_RULES: [RuleIdC; 6] = [
    RuleIdC::BHelperBindEntry,
    RuleIdC::BHelperBindPair,
    RuleIdC::BHelperBindEmpty,
    RuleIdC::BHelperBindType,
    RuleIdC::BHelperBindInsert,
    RuleIdC::BHelperBindExit,
];

pub fn bind_helper_supported(rule: RuleIdC) -> bool {
    BIND_HELPER_RULES.contains(&rule)
}

const K_BIND: u128 = 5;
const SITE_ENTER: u128 = 19;
const R_BIND_OK: u128 = 7;
const T_PAIR: u128 = 2;
// Bind phases and status (c6_phys_helper bphase/bstatus tags).
const PH_PAIRING: u128 = 0;
const PH_UNWIND_TYPE: u128 = 1;
const PH_UNWIND_DUP: u128 = 2;
const PH_UNWIND_INSERT: u128 = 3;
const PH_PROPAGATE: u128 = 4;
const PH_FINISHED: u128 = 5;
const ST_BAD: u128 = 1;
// Bind record offsets after the 5-lane head.
const B_FN: usize = 5;
const B_PARAMS: usize = 6;
const B_ARGS: usize = 8;
const B_P: usize = 10;
const B_A: usize = 11;
const B_PARTIAL: usize = 12;
const B_DUP: usize = 14;
const B_PHASE: usize = 16;
const B_STATUS: usize = 17;
/// HELPER header lane of the live task count.
const H_TASK_COUNT: usize = 3;
/// DATA header lanes of the env and value-slot counts.
const DATA_ENV_COUNT: usize = 5;
const DATA_VALUE_SLOT_COUNT: usize = 6;

#[derive(Clone, Debug)]
pub struct C6BindHelperBlock {
    pub instructions: Vec<Instruction>,
}

pub struct C6BindHelperEmitter {
    scope: String,
    next_temp: usize,
    composed: Composed,
    reader: C6CheckedValueReadEmitter,
}

/// Task-table rewrite: optional push of `row` at `count`, optional pop of the
/// top row (index `count - 1`), each under its own gate; count follows.
struct TaskOp {
    push: Option<(Operand, Vec<Operand>)>,
    pop: Option<Operand>,
}

impl C6BindHelperEmitter {
    pub fn new(scope: &str) -> Result<Self, C6BindHelperError> {
        if scope.trim().is_empty() || scope.contains('%') || scope.contains(".c6kit.") {
            return Err(C6BindHelperError::InvalidScope);
        }
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            composed: Composed::new(scope).map_err(C6BindHelperError::Emit)?,
            reader: C6CheckedValueReadEmitter::new(&format!("{scope}.bindread"))
                .map_err(|e| C6BindHelperError::Emit(e.to_string()))?,
        })
    }

    pub fn emit(
        &mut self,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<C6BindHelperBlock, C6BindHelperError> {
        if !bind_helper_supported(rule) {
            return Err(C6BindHelperError::Unsupported(rule));
        }
        check_shapes(inputs).map_err(C6BindHelperError::Emit)?;
        let scope = self.scope.clone();
        let mut em = Em {
            scope: &scope,
            next: self.next_temp,
            ins: Vec::new(),
        };
        let r = self.body(&mut em, rule, inputs);
        self.next_temp = em.next;
        r.map_err(C6BindHelperError::Emit)?;
        Ok(C6BindHelperBlock {
            instructions: em.ins,
        })
    }

    fn body(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        let en = inputs.enable.clone();
        let caps = inputs.geometry.caps;
        let bounds = inputs.source_params.admission_bounds();
        let max_tasks = inputs.geometry.helper_tasks_lanes / TASK_LANES;
        prefix(em, inputs);
        let _top = validate_pre_image(em, inputs);
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
        };
        let mut image_extra: Vec<usize> = Vec::new();
        let mut data_writes = false;
        let is_bind = |em: &mut Em, phase: u128| {
            em.geq(en.clone(), rec(0), Em::imm(K_BIND));
            em.geq(en.clone(), rec(B_PHASE), Em::imm(phase));
        };
        // "tasks remain after this step" thresholds: count > n.
        let more_than = |em: &mut Em, n: u128| em.lt128(Em::imm(n), count.clone());
        let top_task = |em: &mut Em| {
            let some = em.lt128(Em::imm(0), count.clone());
            em.gassert(en.clone(), some);
            let top_index = em.add(count.clone(), Em::imm(0));
            // Row r is the top iff r + 1 == count.
            let mut out = vec![Em::imm(0); TASK_LANES];
            for r in 0..max_tasks {
                let hit = em.teq(Em::imm(r as u128 + 1), top_index.clone());
                for (c, o) in out.iter_mut().enumerate() {
                    *o = em.sel(
                        hit.clone(),
                        pre_h[L_TASKS + r * TASK_LANES + c].clone(),
                        o.clone(),
                    );
                }
            }
            em.geq(en.clone(), out[0].clone(), Em::imm(T_PAIR));
            out
        };
        let visits;
        match rule {
            RuleIdC::BHelperBindEntry => {
                em.geq(en.clone(), k[0].clone(), Em::imm(4));
                em.geq(en.clone(), k[1].clone(), Em::imm(1));
                let f = self
                    .composed
                    .progctx_em
                    .emit_prog_function_capture(&C6ProgramContextFunctionCaptureInputs {
                        gate: en.clone(),
                        pre_prog: inputs.pre_prog.clone(),
                        caps,
                        function_index: k[2].clone(),
                    })
                    .map_err(|e| e.to_string())?;
                em.ins.extend(f.instructions);
                em.gassert(en.clone(), f.function_found.clone());
                for slot in h_out.iter_mut() {
                    *slot = Some(Em::imm(0));
                }
                h_out[0] = Some(Em::imm(K_BIND));
                h_out[1] = Some(Em::imm(SITE_ENTER));
                h_out[2] = Some(pre_i[L_REFERENCE_CURSOR].clone());
                h_out[B_FN] = Some(k[2].clone());
                h_out[B_PARAMS] = Some(f.function_row[1].clone());
                h_out[B_PARAMS + 1] = Some(f.function_row[2].clone());
                h_out[B_ARGS] = Some(k[3].clone());
                h_out[B_ARGS + 1] = Some(k[4].clone());
                visits = 0;
            }
            RuleIdC::BHelperBindPair => {
                is_bind(em, PH_PAIRING);
                for (cur, len) in [(B_P, B_PARAMS + 1), (B_A, B_ARGS + 1)] {
                    let lt = em.lt128(rec(cur), rec(len));
                    em.gassert(en.clone(), lt);
                }
                let pidx = em.add(rec(B_PARAMS), rec(B_P));
                let p = self
                    .composed
                    .progctx_em
                    .emit_prog_parameter_capture(&C6ProgramContextParameterCaptureInputs {
                        gate: en.clone(),
                        pre_prog: inputs.pre_prog.clone(),
                        caps,
                        parameter_index: pidx,
                    })
                    .map_err(|e| e.to_string())?;
                em.ins.extend(p.instructions);
                em.gassert(en.clone(), p.parameter_found.clone());
                let aidx = em.add(rec(B_ARGS), rec(B_A));
                let live = em.lt128(aidx.clone(), inputs.pre_data[DATA_VALUE_SLOT_COUNT].clone());
                em.gassert(en.clone(), live);
                let (hit, arg) = mux_row(
                    em,
                    &inputs.pre_data,
                    inputs.geometry.value_slots_base(),
                    caps.phys.max_value_slots,
                    REF_LANES,
                    aidx,
                );
                em.gassert(en.clone(), hit);
                let room = em.lt128(count.clone(), Em::imm(bounds.helper_tasks as u128));
                em.gassert(en.clone(), room);
                let row = vec![
                    Em::imm(T_PAIR),
                    p.parameter_row[0].clone(),
                    p.parameter_row[1].clone(),
                    arg[0].clone(),
                    arg[1].clone(),
                    Em::imm(0),
                    Em::imm(0),
                ];
                tasks.push = Some((Em::imm(1), row));
                h_out[B_P] = Some(em.add(rec(B_P), Em::imm(1)));
                h_out[B_A] = Some(em.add(rec(B_A), Em::imm(1)));
                visits = 1;
            }
            RuleIdC::BHelperBindEmpty => {
                is_bind(em, PH_PAIRING);
                em.geq(en.clone(), rec(B_P), rec(B_PARAMS + 1));
                em.geq(en.clone(), rec(B_A), rec(B_ARGS + 1));
                let more = more_than(em, 0);
                h_out[B_PHASE] = Some(em.sel(more, Em::imm(PH_UNWIND_TYPE), Em::imm(PH_FINISHED)));
                visits = 0;
            }
            RuleIdC::BHelperBindType => {
                is_bind(em, PH_UNWIND_TYPE);
                let top = top_task(em);
                let read = self
                    .reader
                    .emit(&C6CheckedValueReadInputs {
                        gate: en.clone(),
                        ref_kind_is_data: top[3].clone(),
                        ref_index: top[4].clone(),
                        pre_prog: inputs.pre_prog.clone(),
                        pre_data: inputs.pre_data.clone(),
                        caps,
                    })
                    .map_err(|e| e.to_string())?;
                em.ins.extend(read.instructions);
                em.gassert(en.clone(), read.selected_found.clone());
                // Builtin checked sort only (UInt 0, Bool 1, List 2, Tree 3).
                em.gzero(en.clone(), read.is_marker.clone());
                em.gzero(en.clone(), read.is_constructor.clone());
                let mut sort = Em::imm(0);
                sort = em.sel(read.is_bool.clone(), Em::imm(1), sort);
                let list = em.add(read.is_nil.clone(), read.is_cons.clone());
                sort = em.sel(list, Em::imm(2), sort);
                let tree = em.add(read.is_leaf.clone(), read.is_branch.clone());
                sort = em.sel(tree, Em::imm(3), sort);
                let same = em.teq(sort, top[2].clone());
                let reject = em.not(same.clone());
                let more = more_than(em, 1);
                let after_pop = em.sel(more, Em::imm(PH_PROPAGATE), Em::imm(PH_FINISHED));
                let has_partial = rec(B_PARTIAL);
                let unwind = em.sel(
                    has_partial,
                    Em::imm(PH_UNWIND_DUP),
                    Em::imm(PH_UNWIND_INSERT),
                );
                h_out[B_PHASE] = Some(em.sel(reject.clone(), after_pop, unwind));
                h_out[B_STATUS] = Some(em.sel(reject.clone(), Em::imm(ST_BAD), rec(B_STATUS)));
                h_out[B_DUP] = Some(em.sel(reject.clone(), rec(B_DUP), rec(B_PARTIAL)));
                h_out[B_DUP + 1] = Some(em.sel(reject.clone(), rec(B_DUP + 1), rec(B_PARTIAL + 1)));
                tasks.pop = Some(reject);
                visits = 1;
            }
            RuleIdC::BHelperBindInsert => {
                is_bind(em, PH_UNWIND_INSERT);
                let top = top_task(em);
                // Logical (P6) and physical environment capacity.
                let env_alloc = pre_i[L_ALLOC + 2].clone();
                let env_active = pre_i[L_ACTIVE + 2].clone();
                let logical =
                    em.lt128(env_alloc.clone(), Em::imm(bounds.environment_cells as u128));
                em.gassert(en.clone(), logical);
                let physical = em.lt128(
                    env_active.clone(),
                    Em::imm(bounds.environment_cells as u128),
                );
                em.gassert(en.clone(), physical);
                let padded = em.lt128(env_active.clone(), Em::imm(caps.phys.max_env_cells as u128));
                em.gassert(en.clone(), padded);
                let next_alloc = em.add(env_alloc, Em::imm(1));
                em.geq(en.clone(), post_i[L_ALLOC + 2].clone(), next_alloc);
                let next_active = em.add(env_active.clone(), Em::imm(1));
                em.geq(
                    en.clone(),
                    post_i[L_ACTIVE + 2].clone(),
                    next_active.clone(),
                );
                em.geq(
                    en.clone(),
                    inputs.post_data[DATA_ENV_COUNT].clone(),
                    next_active,
                );
                image_extra.extend([L_ALLOC + 2, L_ACTIVE + 2]);
                let cell = [
                    top[1].clone(),
                    top[3].clone(),
                    top[4].clone(),
                    rec(B_PARTIAL),
                    rec(B_PARTIAL + 1),
                ];
                let base = inputs.geometry.env_base();
                let env_lanes = base..base + caps.phys.max_env_cells * ENV_ROW_LANES;
                for r in 0..caps.phys.max_env_cells {
                    let hit = em.teq(Em::imm(r as u128), env_active.clone());
                    for (c, v) in cell.iter().enumerate() {
                        let lane = base + r * ENV_ROW_LANES + c;
                        let want = em.sel(hit.clone(), v.clone(), inputs.pre_data[lane].clone());
                        em.geq(en.clone(), inputs.post_data[lane].clone(), want);
                    }
                }
                for (lane, (a, b)) in inputs
                    .pre_data
                    .iter()
                    .zip(inputs.post_data.iter())
                    .enumerate()
                {
                    if lane != DATA_ENV_COUNT && !env_lanes.contains(&lane) {
                        em.geq(en.clone(), b.clone(), a.clone());
                    }
                }
                data_writes = true;
                tasks.pop = Some(Em::imm(1));
                h_out[B_PARTIAL] = Some(Em::imm(1));
                h_out[B_PARTIAL + 1] = Some(pre_i[L_ACTIVE + 2].clone());
                let more = more_than(em, 1);
                h_out[B_PHASE] = Some(em.sel(more, Em::imm(PH_UNWIND_TYPE), Em::imm(PH_FINISHED)));
                visits = 1;
            }
            RuleIdC::BHelperBindExit => {
                is_bind(em, PH_FINISHED);
                em.gzero(en.clone(), rec(B_STATUS));
                for slot in h_out.iter_mut() {
                    *slot = Some(Em::imm(0));
                }
                r_out = Some([
                    Em::imm(R_BIND_OK),
                    rec(B_PARTIAL),
                    rec(B_PARTIAL + 1),
                    Em::imm(0),
                ]);
                visits = 0;
            }
            _ => unreachable!("checked by bind_helper_supported"),
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
                if let Some((g, _)) = &tasks.push {
                    c = em.add(c, g.clone());
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
                if let Some((g, row)) = &tasks.push {
                    let at = em.teq(Em::imm(r as u128), count.clone());
                    let hit = em.and(at, g.clone());
                    want = em.sel(hit, row[c].clone(), want);
                }
                if let Some(g) = &tasks.pop {
                    let at = em.teq(Em::imm(r as u128 + 1), count.clone());
                    let hit = em.and(at, g.clone());
                    want = em.sel(hit, Em::imm(0), want);
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

/// One-shot emission of the Bind helper relation for `rule`.
pub fn emit_c6_bind_helper_block(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<C6BindHelperBlock, C6BindHelperError> {
    C6BindHelperEmitter::new(scope)?.emit(rule, inputs)
}
