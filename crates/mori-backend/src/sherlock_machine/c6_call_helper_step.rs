//! C6 call helper step: the Find and EnvGet helper rows factorial and fold
//! fire (`b.helper.find.{entry,row,exit}`, `b.helper.envget.{entry,cell,exit}`)
//! as one per-rule relation over the full C6 carriers.
//!
//! Each circuit fixes its rule at build time, selects it with the composed
//! dispatch owner and writes the post helper/result records exactly as
//! `families/environment.rs` plans them:
//! - Find.entry: Find { common(Invoke), function_name, row_cursor 0, Pending };
//! - Find.row: compares the PROG function row at row_cursor by name; a match
//!   sets found = row_cursor and Positive, otherwise row_cursor + 1;
//! - Find.exit: result FindFound { found }, helper cleared;
//! - EnvGet.entry: EnvGet { common(EvalVar), binder of the Var row, cursor =
//!   env root, Pending };
//! - EnvGet.cell: reads the DATA env cell at cursor; a binder match sets found
//!   = cell value and Positive, otherwise cursor = cell previous;
//! - EnvGet.exit: result EnvFound { found }, helper cleared.
//!
//! Charge is the applied helper charge (work 1, helper work 1, visits 1 for
//! row/cell, 0 otherwise); the P3 helper-work refusal is rejected, not
//! constrained. Control, frames, tasks, DATA and every unnamed lane are
//! preserved; canonicality is composed; enable = 0 is identity.
use midnight_zkir_v3::Instruction;
use midnight_zkir_v3::ir::Operand;
use mori_core::sherlock::helper_machine::c6_phys::{
    CONTROL_LANES, L_CONTROL, L_ENV_ROOT, L_REFERENCE_CURSOR,
};
use mori_core::sherlock::helper_machine::c6_phys_data::ENV_ROW_LANES;
use mori_core::sherlock::helper_machine::c6_phys_helper::{
    HELPER_LANES, L_HELPER_RECORD, L_RESULT_RECORD, RESULT_LANES,
};
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_program_context_canonical::{
    C6ProgramContextDispatchCaptureInputs, C6ProgramContextFunctionCaptureInputs,
};
use super::c6_step_kit::{
    C6StepInputs, Composed, Em, charge_helper, check_shapes, compose_canonical, disabled_identity,
    helper_charge_lanes, mux_row, prefix, preserve_image, select, validate_pre_image,
};

#[derive(Debug, Error)]
pub enum C6CallHelperError {
    #[error("rule {0:?} is not a Find/EnvGet helper rule of this fragment")]
    Unsupported(RuleIdC),
    #[error("invalid scope")]
    InvalidScope,
    #[error("C6 call helper emission: {0}")]
    Emit(String),
}

pub const CALL_HELPER_RULES: [RuleIdC; 6] = [
    RuleIdC::BHelperFindEntry,
    RuleIdC::BHelperFindRow,
    RuleIdC::BHelperFindExit,
    RuleIdC::BHelperEnvGetEntry,
    RuleIdC::BHelperEnvGetCell,
    RuleIdC::BHelperEnvGetExit,
];

pub fn call_helper_supported(rule: RuleIdC) -> bool {
    CALL_HELPER_RULES.contains(&rule)
}

// Helper kind tags, site tags and result tags (c6_phys_helper encoders).
const K_ENVGET: u128 = 2;
const K_FIND: u128 = 4;
const SITE_EVAL_VAR: u128 = 1;
const SITE_INVOKE: u128 = 18;
const R_ENV_FOUND: u128 = 2;
const R_FIND_FOUND: u128 = 5;
const POSITIVE: u128 = 1;

#[derive(Clone, Debug)]
pub struct C6CallHelperBlock {
    pub instructions: Vec<Instruction>,
}

pub struct C6CallHelperEmitter {
    scope: String,
    next_temp: usize,
    composed: Composed,
}

impl C6CallHelperEmitter {
    pub fn new(scope: &str) -> Result<Self, C6CallHelperError> {
        if scope.trim().is_empty() || scope.contains('%') || scope.contains(".c6kit.") {
            return Err(C6CallHelperError::InvalidScope);
        }
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            composed: Composed::new(scope).map_err(C6CallHelperError::Emit)?,
        })
    }

    pub fn emit(
        &mut self,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<C6CallHelperBlock, C6CallHelperError> {
        if !call_helper_supported(rule) {
            return Err(C6CallHelperError::Unsupported(rule));
        }
        check_shapes(inputs).map_err(C6CallHelperError::Emit)?;
        let scope = self.scope.clone();
        let mut em = Em {
            scope: &scope,
            next: self.next_temp,
            ins: Vec::new(),
        };
        let r = self.body(&mut em, rule, inputs);
        self.next_temp = em.next;
        r.map_err(C6CallHelperError::Emit)?;
        Ok(C6CallHelperBlock {
            instructions: em.ins,
        })
    }

    fn body(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        let en = inputs.enable.clone();
        let caps = inputs.geometry.caps;
        prefix(em, inputs);
        let _top = validate_pre_image(em, inputs);
        select(&mut self.composed, em, rule, inputs)?;

        let pre_i = &inputs.pre_image;
        let k = &pre_i[L_CONTROL..L_CONTROL + CONTROL_LANES];
        let pre_h = &inputs.pre_helper;
        let rec = |o: usize| pre_h[L_HELPER_RECORD + o].clone();
        // Expected post helper record and result record (None = preserved).
        let mut h_out: Vec<Option<Operand>> = vec![None; HELPER_LANES];
        let mut r_out: Option<[Operand; RESULT_LANES]> = None;
        let entry = |kind: u128, site: u128, h_out: &mut Vec<Option<Operand>>| {
            for slot in h_out.iter_mut() {
                *slot = Some(Em::imm(0));
            }
            h_out[0] = Some(Em::imm(kind));
            h_out[1] = Some(Em::imm(site));
            h_out[2] = Some(pre_i[L_REFERENCE_CURSOR].clone());
        };
        let visits;
        match rule {
            RuleIdC::BHelperFindEntry => {
                em.geq(en.clone(), k[0].clone(), Em::imm(3));
                entry(K_FIND, SITE_INVOKE, &mut h_out);
                h_out[5] = Some(k[1].clone());
                visits = 0;
            }
            RuleIdC::BHelperEnvGetEntry => {
                em.geq(en.clone(), k[0].clone(), Em::imm(0));
                let cap = self
                    .composed
                    .progctx_em
                    .emit_prog_dispatch_capture(&C6ProgramContextDispatchCaptureInputs {
                        gate: en.clone(),
                        pre_prog: inputs.pre_prog.clone(),
                        caps,
                        first_index: k[1].clone(),
                        second_index: k[1].clone(),
                        child_index: Em::imm(0),
                    })
                    .map_err(|e| e.to_string())?;
                em.ins.extend(cap.instructions);
                em.gassert(en.clone(), cap.first_found.clone());
                em.geq(en.clone(), cap.first_row[0].clone(), Em::imm(1));
                entry(K_ENVGET, SITE_EVAL_VAR, &mut h_out);
                h_out[5] = Some(cap.first_row[1].clone());
                h_out[6] = Some(pre_i[L_ENV_ROOT].clone());
                h_out[7] = Some(pre_i[L_ENV_ROOT + 1].clone());
                visits = 0;
            }
            RuleIdC::BHelperFindRow => {
                em.geq(en.clone(), rec(0), Em::imm(K_FIND));
                em.geq(en.clone(), rec(9), Em::imm(0));
                let f = self
                    .composed
                    .progctx_em
                    .emit_prog_function_capture(&C6ProgramContextFunctionCaptureInputs {
                        gate: en.clone(),
                        pre_prog: inputs.pre_prog.clone(),
                        caps,
                        function_index: rec(6),
                    })
                    .map_err(|e| e.to_string())?;
                em.ins.extend(f.instructions);
                em.gassert(en.clone(), f.function_found.clone());
                let eq = em.teq(f.function_row[0].clone(), rec(5));
                let next = em.add(rec(6), Em::imm(1));
                h_out[6] = Some(em.sel(eq.clone(), rec(6), next));
                h_out[7] = Some(eq.clone());
                h_out[8] = Some(em.mul(eq.clone(), rec(6)));
                h_out[9] = Some(eq);
                visits = 1;
            }
            RuleIdC::BHelperEnvGetCell => {
                em.geq(en.clone(), rec(0), Em::imm(K_ENVGET));
                em.geq(en.clone(), rec(11), Em::imm(0));
                em.geq(en.clone(), rec(6), Em::imm(1));
                // The cursor cell must be an active env row.
                let active_env = inputs.pre_data[5].clone();
                let live = em.lt128(rec(7), active_env);
                em.gassert(en.clone(), live);
                let g = &inputs.geometry;
                let (hit, cell) = mux_row(
                    em,
                    &inputs.pre_data,
                    g.env_base(),
                    caps.phys.max_env_cells,
                    ENV_ROW_LANES,
                    rec(7),
                );
                em.gassert(en.clone(), hit);
                let eq = em.teq(cell[0].clone(), rec(5));
                h_out[6] = Some(em.sel(eq.clone(), rec(6), cell[3].clone()));
                h_out[7] = Some(em.sel(eq.clone(), rec(7), cell[4].clone()));
                h_out[8] = Some(em.sel(eq.clone(), Em::imm(1), rec(8)));
                h_out[9] = Some(em.sel(eq.clone(), cell[1].clone(), rec(9)));
                h_out[10] = Some(em.sel(eq.clone(), cell[2].clone(), rec(10)));
                h_out[11] = Some(em.sel(eq, Em::imm(POSITIVE), rec(11)));
                visits = 1;
            }
            RuleIdC::BHelperFindExit => {
                em.geq(en.clone(), rec(0), Em::imm(K_FIND));
                em.geq(en.clone(), rec(9), Em::imm(POSITIVE));
                em.geq(en.clone(), rec(7), Em::imm(1));
                for slot in h_out.iter_mut() {
                    *slot = Some(Em::imm(0));
                }
                r_out = Some([Em::imm(R_FIND_FOUND), rec(8), Em::imm(0), Em::imm(0)]);
                visits = 0;
            }
            RuleIdC::BHelperEnvGetExit => {
                em.geq(en.clone(), rec(0), Em::imm(K_ENVGET));
                em.geq(en.clone(), rec(11), Em::imm(POSITIVE));
                em.geq(en.clone(), rec(8), Em::imm(1));
                for slot in h_out.iter_mut() {
                    *slot = Some(Em::imm(0));
                }
                r_out = Some([Em::imm(R_ENV_FOUND), rec(9), rec(10), Em::imm(0)]);
                visits = 0;
            }
            _ => unreachable!("checked by call_helper_supported"),
        }
        let post_h = &inputs.post_helper;
        for (lane, (a, b)) in pre_h.iter().zip(post_h.iter()).enumerate() {
            let want = if (L_HELPER_RECORD..L_HELPER_RECORD + HELPER_LANES).contains(&lane) {
                h_out[lane - L_HELPER_RECORD].clone()
            } else if (L_RESULT_RECORD..L_RESULT_RECORD + RESULT_LANES).contains(&lane) {
                r_out.as_ref().map(|r| r[lane - L_RESULT_RECORD].clone())
            } else {
                None
            };
            em.geq(en.clone(), b.clone(), want.unwrap_or_else(|| a.clone()));
        }
        preserve_image(em, inputs, &helper_charge_lanes(), false);
        charge_helper(em, inputs, visits);
        for (a, b) in inputs.pre_data.iter().zip(inputs.post_data.iter()) {
            em.geq(en.clone(), b.clone(), a.clone());
        }
        compose_canonical(&mut self.composed, em, inputs)?;
        disabled_identity(em, inputs);
        Ok(())
    }
}

/// One-shot emission of the call helper relation for `rule`.
pub fn emit_c6_call_helper_block(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<C6CallHelperBlock, C6CallHelperError> {
    C6CallHelperEmitter::new(scope)?.emit(rule, inputs)
}
