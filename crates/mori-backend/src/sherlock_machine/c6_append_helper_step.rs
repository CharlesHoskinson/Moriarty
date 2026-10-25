//! C6 Append helper step: the `b.helper.append.{entry,empty,tail,exit}` rows
//! factorial and fold fire (argument accumulation at a RetArgs site) as one
//! per-rule relation over the full C6 carriers.
//!
//! Each circuit fixes its rule at build time, selects it with the composed
//! dispatch owner and writes the post state exactly as
//! `families/environment.rs` plans it:
//! - entry (RetArgs: control Return { value }, top frame ArgsF { completed }):
//!   Append { common(RetArgs), src = completed, value, src_cursor 0,
//!   dest_start = active value slots, staged None, Copy };
//! - empty (src_cursor == src.len, nothing staged): phase Tail;
//! - tail: appends `value` as the next DATA value slot (active value slots
//!   and the DATA value-slot count + 1), phase Done;
//! - exit: result AppendDone { seq = (dest_start, src.len + 1) }, helper
//!   cleared.
//!
//! Charge is the applied helper charge (visits 1 for tail, 0 otherwise); the
//! P3 helper-work refusal is rejected, not constrained. The ADT
//! RetCtorFields entry (`adt_constructor::append_entry`) and the copy/cell
//! rows are not covered by this fragment. Every unnamed lane is preserved;
//! canonicality is composed; enable = 0 is identity.
use midnight_zkir_v3::Instruction;
use midnight_zkir_v3::ir::Operand;
use mori_core::sherlock::helper_machine::c6_phys::{
    CONTROL_LANES, L_ACTIVE, L_CONTROL, L_REFERENCE_CURSOR,
};
use mori_core::sherlock::helper_machine::c6_phys_data::REF_LANES;
use mori_core::sherlock::helper_machine::c6_phys_helper::{
    HELPER_LANES, L_HELPER_RECORD, L_RESULT_RECORD, RESULT_LANES,
};
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_step_kit::{
    C6StepInputs, Composed, Em, charge_helper, check_shapes, compose_canonical, disabled_identity,
    helper_charge_lanes, prefix, preserve_image, select, validate_pre_image,
};

#[derive(Debug, Error)]
pub enum C6AppendHelperError {
    #[error("rule {0:?} is not an Append helper rule of this fragment")]
    Unsupported(RuleIdC),
    #[error("invalid scope")]
    InvalidScope,
    #[error("C6 Append helper emission: {0}")]
    Emit(String),
}

pub const APPEND_HELPER_RULES: [RuleIdC; 4] = [
    RuleIdC::BHelperAppendEntry,
    RuleIdC::BHelperAppendEmpty,
    RuleIdC::BHelperAppendTail,
    RuleIdC::BHelperAppendExit,
];

pub fn append_helper_supported(rule: RuleIdC) -> bool {
    APPEND_HELPER_RULES.contains(&rule)
}

const K_APPEND: u128 = 6;
const SITE_RET_ARGS: u128 = 15;
const R_APPEND_DONE: u128 = 9;
const PHASE_COPY: u128 = 0;
const PHASE_TAIL: u128 = 1;
const PHASE_DONE: u128 = 2;
const FRAME_ARGS: u128 = 6;
// Append record offsets: head 0..5, src 5..7, value 7..9, src_cursor 9,
// dest_start 10, staged 11..14, phase 14.
const A_SRC: usize = 5;
const A_VALUE: usize = 7;
const A_SRC_CURSOR: usize = 9;
const A_DEST: usize = 10;
const A_STAGED: usize = 11;
const A_PHASE: usize = 14;
/// DATA header lane of the value-slot count.
const DATA_VALUE_SLOT_COUNT: usize = 6;

#[derive(Clone, Debug)]
pub struct C6AppendHelperBlock {
    pub instructions: Vec<Instruction>,
}

pub struct C6AppendHelperEmitter {
    scope: String,
    next_temp: usize,
    composed: Composed,
}

impl C6AppendHelperEmitter {
    pub fn new(scope: &str) -> Result<Self, C6AppendHelperError> {
        if scope.trim().is_empty() || scope.contains('%') || scope.contains(".c6kit.") {
            return Err(C6AppendHelperError::InvalidScope);
        }
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            composed: Composed::new(scope).map_err(C6AppendHelperError::Emit)?,
        })
    }

    pub fn emit(
        &mut self,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<C6AppendHelperBlock, C6AppendHelperError> {
        if !append_helper_supported(rule) {
            return Err(C6AppendHelperError::Unsupported(rule));
        }
        check_shapes(inputs).map_err(C6AppendHelperError::Emit)?;
        let scope = self.scope.clone();
        let mut em = Em {
            scope: &scope,
            next: self.next_temp,
            ins: Vec::new(),
        };
        let r = self.body(&mut em, rule, inputs);
        self.next_temp = em.next;
        r.map_err(C6AppendHelperError::Emit)?;
        Ok(C6AppendHelperBlock {
            instructions: em.ins,
        })
    }

    fn body(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        let en = inputs.enable.clone();
        let caps = inputs.geometry.caps;
        prefix(em, inputs);
        let top = validate_pre_image(em, inputs);
        select(&mut self.composed, em, rule, inputs)?;

        let pre_i = &inputs.pre_image;
        let post_i = &inputs.post_image;
        let k = &pre_i[L_CONTROL..L_CONTROL + CONTROL_LANES];
        let pre_h = &inputs.pre_helper;
        let rec = |o: usize| pre_h[L_HELPER_RECORD + o].clone();
        let mut h_out: Vec<Option<Operand>> = vec![None; HELPER_LANES];
        let mut r_out: Option<[Operand; RESULT_LANES]> = None;
        // Image header lanes beyond the charge this rule changes.
        let mut image_extra: Vec<usize> = Vec::new();
        let mut data_writes = false;
        let visits;
        let is_append = |em: &mut Em| em.geq(en.clone(), rec(0), Em::imm(K_APPEND));
        match rule {
            RuleIdC::BHelperAppendEntry => {
                // Return { value } under an ArgsF top frame.
                em.geq(en.clone(), k[0].clone(), Em::imm(1));
                em.geq(en.clone(), top[0].clone(), Em::imm(FRAME_ARGS));
                for slot in h_out.iter_mut() {
                    *slot = Some(Em::imm(0));
                }
                h_out[0] = Some(Em::imm(K_APPEND));
                h_out[1] = Some(Em::imm(SITE_RET_ARGS));
                h_out[2] = Some(pre_i[L_REFERENCE_CURSOR].clone());
                h_out[A_SRC] = Some(top[2].clone());
                h_out[A_SRC + 1] = Some(top[3].clone());
                h_out[A_VALUE] = Some(k[1].clone());
                h_out[A_VALUE + 1] = Some(k[2].clone());
                h_out[A_DEST] = Some(pre_i[L_ACTIVE + 3].clone());
                h_out[A_PHASE] = Some(Em::imm(PHASE_COPY));
                visits = 0;
            }
            RuleIdC::BHelperAppendEmpty => {
                is_append(em);
                em.geq(en.clone(), rec(A_PHASE), Em::imm(PHASE_COPY));
                em.geq(en.clone(), rec(A_STAGED), Em::imm(0));
                em.geq(en.clone(), rec(A_SRC_CURSOR), rec(A_SRC + 1));
                h_out[A_PHASE] = Some(Em::imm(PHASE_TAIL));
                visits = 0;
            }
            RuleIdC::BHelperAppendTail => {
                is_append(em);
                em.geq(en.clone(), rec(A_PHASE), Em::imm(PHASE_TAIL));
                h_out[A_PHASE] = Some(Em::imm(PHASE_DONE));
                // Next value slot = value; counts + 1, within the slot cap.
                let count = inputs.pre_data[DATA_VALUE_SLOT_COUNT].clone();
                let fits = em.lt128(count.clone(), Em::imm(caps.phys.max_value_slots as u128));
                em.gassert(en.clone(), fits);
                let next = em.add(count.clone(), Em::imm(1));
                em.geq(
                    en.clone(),
                    inputs.post_data[DATA_VALUE_SLOT_COUNT].clone(),
                    next.clone(),
                );
                em.geq(en.clone(), post_i[L_ACTIVE + 3].clone(), next);
                image_extra.push(L_ACTIVE + 3);
                let base = inputs.geometry.value_slots_base();
                for r in 0..caps.phys.max_value_slots {
                    let hit = em.teq(Em::imm(r as u128), count.clone());
                    for c in 0..REF_LANES {
                        let lane = base + r * REF_LANES + c;
                        let want =
                            em.sel(hit.clone(), rec(A_VALUE + c), inputs.pre_data[lane].clone());
                        em.geq(en.clone(), inputs.post_data[lane].clone(), want);
                    }
                }
                for (lane, (a, b)) in inputs
                    .pre_data
                    .iter()
                    .zip(inputs.post_data.iter())
                    .enumerate()
                {
                    let in_slots =
                        (base..base + caps.phys.max_value_slots * REF_LANES).contains(&lane);
                    if lane != DATA_VALUE_SLOT_COUNT && !in_slots {
                        em.geq(en.clone(), b.clone(), a.clone());
                    }
                }
                data_writes = true;
                visits = 1;
            }
            RuleIdC::BHelperAppendExit => {
                is_append(em);
                em.geq(en.clone(), rec(A_PHASE), Em::imm(PHASE_DONE));
                for slot in h_out.iter_mut() {
                    *slot = Some(Em::imm(0));
                }
                let len = em.add(rec(A_SRC + 1), Em::imm(1));
                r_out = Some([Em::imm(R_APPEND_DONE), rec(A_DEST), len, Em::imm(0)]);
                visits = 0;
            }
            _ => unreachable!("checked by append_helper_supported"),
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

/// One-shot emission of the Append helper relation for `rule`.
pub fn emit_c6_append_helper_block(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<C6AppendHelperBlock, C6AppendHelperError> {
    C6AppendHelperEmitter::new(scope)?.emit(rule, inputs)
}
