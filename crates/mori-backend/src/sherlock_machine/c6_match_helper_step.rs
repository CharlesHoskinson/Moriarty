//! C6 Match helper step, first half: the `b.helper.match.{entry,arm,
//! occurrence,field,binding}` rows factorial and tree fold fire while
//! trying match arms, as one per-rule relation over the full C6 carriers.
//! (The second half — distinct, next, selected, exit, failure — is the
//! sibling fragment owner's; this emitter refuses those rules.)
//!
//! Each circuit fixes its rule at build time, selects it with the composed
//! dispatch owner and writes the post state exactly as
//! `families/structural.rs` (`plan_match`, `match_entry_payload`,
//! `match_arm_payload`, `plan_match_postarm`) plans it:
//! - entry (control Cases { scrutinee, arms, saved_env }): Match { common
//!   (Cases), scrutinee, arm = arms.start, saved/partial = saved_env,
//!   PVar, phase Arm, Pending };
//! - arm: the PROG arm at `arm` and its pattern row give pat_tag/x/y;
//!   PVar stages the scrutinee and goes to Bind, every other pattern goes
//!   to Tag. Wildcard/Constructor patterns are rejected (Core refuses
//!   them for this helper). The control-origin `Match { plan: None }`
//!   precondition is owned by the composed dispatch selection;
//! - occurrence (Tag): the checked kind of the scrutinee against the
//!   pattern kind (PNil/PCons/PLeaf/PBranch): mismatch to NoMatchReady,
//!   PNil to Ready, PLeaf to Field (field 0), PCons/PBranch to Distinct;
//! - field (Field): `field_at(scrutinee, field_idx)` (checked read slice
//!   plus the Const-field or DATA-field table) is staged, phase Bind;
//! - binding (Bind): allocates the env cell { binder, staged, previous =
//!   partial } (binder is x for PVar or field 0, else y), partial becomes
//!   that cell, staged clears; PVar/PLeaf or field 1 finish to Ready,
//!   otherwise field 1 is staged next.
//!
//! Charge is the applied helper charge (visits 1 for arm, occurrence,
//! field and binding). Not constrained here, so rejected: the P3
//! helper-work refusal and the environment cap refusals. Every unnamed
//! lane is preserved; canonicality is composed; enable = 0 is identity.
use midnight_zkir_v3::Instruction;
use midnight_zkir_v3::ir::Operand;
use mori_core::sherlock::helper_machine::c6_phys::{
    CONTROL_LANES, L_ACTIVE, L_ALLOC, L_CONTROL, L_REFERENCE_CURSOR,
};
use mori_core::sherlock::helper_machine::c6_phys_data::{ENV_ROW_LANES, REF_LANES};
use mori_core::sherlock::helper_machine::c6_phys_helper::{
    HELPER_LANES, L_HELPER_RECORD, L_RESULT_RECORD, RESULT_LANES,
};
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_checked_value_read::{C6CheckedValueReadEmitter, C6CheckedValueReadInputs};
use super::c6_program_context_canonical::{
    C6ProgramContextArmCaptureInputs, C6ProgramContextConstFieldCaptureInputs,
    C6ProgramContextPatternCaptureInputs,
};
use super::c6_step_kit::{
    C6StepInputs, Composed, Em, charge_helper, check_shapes, compose_canonical, disabled_identity,
    helper_charge_lanes, mux_row, prefix, preserve_image, select, validate_pre_image,
};

#[derive(Debug, Error)]
pub enum C6MatchHelperError {
    #[error("rule {0:?} is not a Match helper rule of this fragment")]
    Unsupported(RuleIdC),
    #[error("invalid scope")]
    InvalidScope,
    #[error("C6 Match helper emission: {0}")]
    Emit(String),
}

pub const MATCH_HELPER_RULES: [RuleIdC; 5] = [
    RuleIdC::BHelperMatchEntry,
    RuleIdC::BHelperMatchArm,
    RuleIdC::BHelperMatchOccurrence,
    RuleIdC::BHelperMatchField,
    RuleIdC::BHelperMatchBinding,
];

pub fn match_helper_supported(rule: RuleIdC) -> bool {
    MATCH_HELPER_RULES.contains(&rule)
}

const K_MATCH: u128 = 9;
const SITE_CASES: u128 = 21;
// Match phases (c6_phys_helper mphase tags).
const PH_ARM: u128 = 0;
const PH_TAG: u128 = 1;
const PH_DISTINCT: u128 = 2;
const PH_FIELD: u128 = 3;
const PH_BIND: u128 = 4;
const PH_READY: u128 = 5;
const PH_NO_MATCH_READY: u128 = 6;
// Pattern tags.
const PAT_VAR: u128 = 0;
const PAT_NIL: u128 = 1;
const PAT_CONS: u128 = 2;
const PAT_LEAF: u128 = 3;
const PAT_BRANCH: u128 = 4;
// Match record offsets after the 5-lane head.
const M_SCRUT: usize = 5;
const M_ARM: usize = 7;
const M_SAVED: usize = 8;
const M_PARTIAL: usize = 10;
const M_PAT: usize = 12;
const M_X: usize = 13;
const M_Y: usize = 14;
const M_FIELD_IDX: usize = 15;
const M_STAGED: usize = 16;
const M_PHASE: usize = 19;
/// DATA header lanes of the field and env counts.
const DATA_FIELD_COUNT: usize = 4;
const DATA_ENV_COUNT: usize = 5;

#[derive(Clone, Debug)]
pub struct C6MatchHelperBlock {
    pub instructions: Vec<Instruction>,
}

pub struct C6MatchHelperEmitter {
    scope: String,
    next_temp: usize,
    composed: Composed,
    reader: C6CheckedValueReadEmitter,
}

impl C6MatchHelperEmitter {
    pub fn new(scope: &str) -> Result<Self, C6MatchHelperError> {
        if scope.trim().is_empty() || scope.contains('%') || scope.contains(".c6kit.") {
            return Err(C6MatchHelperError::InvalidScope);
        }
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            composed: Composed::new(scope).map_err(C6MatchHelperError::Emit)?,
            reader: C6CheckedValueReadEmitter::new(&format!("{scope}.matchread"))
                .map_err(|e| C6MatchHelperError::Emit(e.to_string()))?,
        })
    }

    pub fn emit(
        &mut self,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<C6MatchHelperBlock, C6MatchHelperError> {
        if !match_helper_supported(rule) {
            return Err(C6MatchHelperError::Unsupported(rule));
        }
        check_shapes(inputs).map_err(C6MatchHelperError::Emit)?;
        let scope = self.scope.clone();
        let mut em = Em {
            scope: &scope,
            next: self.next_temp,
            ins: Vec::new(),
        };
        let r = self.body(&mut em, rule, inputs);
        self.next_temp = em.next;
        r.map_err(C6MatchHelperError::Emit)?;
        Ok(C6MatchHelperBlock {
            instructions: em.ins,
        })
    }

    fn body(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        let en = inputs.enable.clone();
        let caps = inputs.geometry.caps;
        let bounds = inputs.source_params.admission_bounds();
        prefix(em, inputs);
        let _top = validate_pre_image(em, inputs);
        // BISECT: select(&mut self.composed, em, rule, inputs)?;

        let pre_i = &inputs.pre_image;
        let post_i = &inputs.post_image;
        let k = &pre_i[L_CONTROL..L_CONTROL + CONTROL_LANES];
        let pre_h = &inputs.pre_helper;
        let rec = |o: usize| pre_h[L_HELPER_RECORD + o].clone();
        let mut h_out: Vec<Option<Operand>> = vec![None; HELPER_LANES];
        let mut r_out: Option<[Operand; RESULT_LANES]> = None;
        let mut image_extra: Vec<usize> = Vec::new();
        let mut data_writes = false;
        let is_match = |em: &mut Em, phase: u128| {
            em.geq(en.clone(), rec(0), Em::imm(K_MATCH));
            em.geq(en.clone(), rec(M_PHASE), Em::imm(phase));
        };
        let visits;
        match rule {
            RuleIdC::BHelperMatchEntry => {
                // Cases { value, arms, saved_env } control (tag 6).
                em.geq(en.clone(), k[0].clone(), Em::imm(6));
                for slot in h_out.iter_mut() {
                    *slot = Some(Em::imm(0));
                }
                h_out[0] = Some(Em::imm(K_MATCH));
                h_out[1] = Some(Em::imm(SITE_CASES));
                h_out[2] = Some(pre_i[L_REFERENCE_CURSOR].clone());
                h_out[M_SCRUT] = Some(k[1].clone());
                h_out[M_SCRUT + 1] = Some(k[2].clone());
                h_out[M_ARM] = Some(k[3].clone());
                h_out[M_SAVED] = Some(k[5].clone());
                h_out[M_SAVED + 1] = Some(k[6].clone());
                h_out[M_PARTIAL] = Some(k[5].clone());
                h_out[M_PARTIAL + 1] = Some(k[6].clone());
                h_out[M_PHASE] = Some(Em::imm(PH_ARM));
                visits = 0;
            }
            RuleIdC::BHelperMatchArm => {
                is_match(em, PH_ARM);
                let arm = self
                    .composed
                    .progctx_em
                    .emit_prog_arm_capture(&C6ProgramContextArmCaptureInputs {
                        gate: en.clone(),
                        pre_prog: inputs.pre_prog.clone(),
                        caps,
                        arm_index: rec(M_ARM),
                    })
                    .map_err(|e| e.to_string())?;
                em.ins.extend(arm.instructions);
                em.gassert(en.clone(), arm.arm_found.clone());
                let pat = self
                    .composed
                    .progctx_em
                    .emit_prog_pattern_capture(&C6ProgramContextPatternCaptureInputs {
                        gate: en.clone(),
                        pre_prog: inputs.pre_prog.clone(),
                        caps,
                        pattern_index: arm.arm_row[0].clone(),
                    })
                    .map_err(|e| e.to_string())?;
                em.ins.extend(pat.instructions);
                em.gassert(en.clone(), pat.pattern_found.clone());
                let tag = pat.pattern_row[0].clone();
                // Only PVar/PNil/PCons/PLeaf/PBranch enter this helper.
                let supported = em.lt128(tag.clone(), Em::imm(5));
                em.gassert(en.clone(), supported);
                let is_var = em.teq(tag.clone(), Em::imm(PAT_VAR));
                let is_nil = em.teq(tag.clone(), Em::imm(PAT_NIL));
                let x = em.sel(is_nil, Em::imm(0), pat.pattern_row[1].clone());
                let is_cons = em.teq(tag.clone(), Em::imm(PAT_CONS));
                let is_branch = em.teq(tag.clone(), Em::imm(PAT_BRANCH));
                let binds_y = em.add(is_cons, is_branch);
                let y = em.sel(binds_y, pat.pattern_row[2].clone(), Em::imm(0));
                h_out[M_PAT] = Some(tag);
                h_out[M_X] = Some(x);
                h_out[M_Y] = Some(y);
                h_out[M_STAGED] = Some(is_var.clone());
                h_out[M_STAGED + 1] = Some(em.mul(is_var.clone(), rec(M_SCRUT)));
                h_out[M_STAGED + 2] = Some(em.mul(is_var, rec(M_SCRUT + 1)));
                let to_bind = em.teq(pat.pattern_row[0].clone(), Em::imm(PAT_VAR));
                h_out[M_PHASE] = Some(em.sel(to_bind, Em::imm(PH_BIND), Em::imm(PH_TAG)));
                visits = 1;
            }
            RuleIdC::BHelperMatchOccurrence => {
                is_match(em, PH_TAG);
                let read = self
                    .reader
                    .emit(&C6CheckedValueReadInputs {
                        gate: en.clone(),
                        ref_kind_is_data: rec(M_SCRUT),
                        ref_index: rec(M_SCRUT + 1),
                        pre_prog: inputs.pre_prog.clone(),
                        pre_data: inputs.pre_data.clone(),
                        caps,
                    })
                    .map_err(|e| e.to_string())?;
                em.ins.extend(read.instructions);
                em.gassert(en.clone(), read.selected_found.clone());
                let pat = rec(M_PAT);
                let p_nil = em.teq(pat.clone(), Em::imm(PAT_NIL));
                let p_cons = em.teq(pat.clone(), Em::imm(PAT_CONS));
                let p_leaf = em.teq(pat.clone(), Em::imm(PAT_LEAF));
                let p_branch = em.teq(pat.clone(), Em::imm(PAT_BRANCH));
                let known_a = em.add(p_nil.clone(), p_cons.clone());
                let known_b = em.add(p_leaf.clone(), p_branch.clone());
                let known = em.add(known_a, known_b);
                em.geq(en.clone(), known, Em::imm(1));
                let ok_cons = em.mul(p_cons.clone(), read.is_cons.clone());
                let ok_leaf = em.mul(p_leaf.clone(), read.is_leaf.clone());
                let ok_branch = em.mul(p_branch, read.is_branch.clone());
                let mut kind_ok = em.mul(p_nil.clone(), read.is_nil.clone());
                kind_ok = em.add(kind_ok, ok_cons);
                kind_ok = em.add(kind_ok, ok_leaf);
                kind_ok = em.add(kind_ok, ok_branch);
                let mismatch = em.not(kind_ok.clone());
                // Match: PNil -> Ready, PLeaf -> Field, PCons/PBranch -> Distinct.
                let mut on_match = em.sel(p_nil, Em::imm(PH_READY), Em::imm(PH_DISTINCT));
                on_match = em.sel(p_leaf.clone(), Em::imm(PH_FIELD), on_match);
                h_out[M_PHASE] = Some(em.sel(mismatch, Em::imm(PH_NO_MATCH_READY), on_match));
                h_out[M_FIELD_IDX] = Some(em.sel(p_leaf, Em::imm(0), rec(M_FIELD_IDX)));
                visits = 1;
            }
            RuleIdC::BHelperMatchField => {
                is_match(em, PH_FIELD);
                let read = self
                    .reader
                    .emit(&C6CheckedValueReadInputs {
                        gate: en.clone(),
                        ref_kind_is_data: rec(M_SCRUT),
                        ref_index: rec(M_SCRUT + 1),
                        pre_prog: inputs.pre_prog.clone(),
                        pre_data: inputs.pre_data.clone(),
                        caps,
                    })
                    .map_err(|e| e.to_string())?;
                em.ins.extend(read.instructions);
                em.gassert(en.clone(), read.selected_found.clone());
                em.gassert(en.clone(), read.slice_present.clone());
                let in_slice = em.lt128(rec(M_FIELD_IDX), read.slice_len.clone());
                em.gassert(en.clone(), in_slice);
                let pos = em.add(read.slice_start.clone(), rec(M_FIELD_IDX));
                // Const scrutinee: the field is a constant index in PROG;
                // Data scrutinee: a RefC row of the DATA field table.
                let cf = self
                    .composed
                    .progctx_em
                    .emit_prog_const_field_capture(&C6ProgramContextConstFieldCaptureInputs {
                        gate: en.clone(),
                        pre_prog: inputs.pre_prog.clone(),
                        caps,
                        field_index: pos.clone(),
                    })
                    .map_err(|e| e.to_string())?;
                em.ins.extend(cf.instructions);
                let is_data = rec(M_SCRUT);
                let not_data = em.not(is_data.clone());
                let cf_gate = em.and(en.clone(), not_data);
                em.gassert(cf_gate, cf.field_found.clone());
                let (hit, dfield) = mux_row(
                    em,
                    &inputs.pre_data,
                    inputs.geometry.data_fields_base(),
                    caps.max_data_field_slots,
                    REF_LANES,
                    pos.clone(),
                );
                let in_data = em.lt128(pos, inputs.pre_data[DATA_FIELD_COUNT].clone());
                let data_gate = em.and(en.clone(), is_data.clone());
                em.gassert(data_gate.clone(), in_data);
                em.gassert(data_gate, hit);
                let child_kind = em.sel(is_data.clone(), dfield[0].clone(), Em::imm(0));
                let child_index = em.sel(is_data, dfield[1].clone(), cf.const_index.clone());
                h_out[M_STAGED] = Some(Em::imm(1));
                h_out[M_STAGED + 1] = Some(child_kind);
                h_out[M_STAGED + 2] = Some(child_index);
                h_out[M_PHASE] = Some(Em::imm(PH_BIND));
                visits = 1;
            }
            RuleIdC::BHelperMatchBinding => {
                is_match(em, PH_BIND);
                em.geq(en.clone(), rec(M_STAGED), Em::imm(1));
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
                let is_var = em.teq(rec(M_PAT), Em::imm(PAT_VAR));
                let first_field = em.teq(rec(M_FIELD_IDX), Em::imm(0));
                let not_var = em.not(is_var.clone());
                let var_or_first = em.mul(not_var, first_field);
                let use_x = em.add(is_var.clone(), var_or_first);
                // use_x is 1 exactly for PVar or field 0 (booleans, disjoint).
                let is_leaf = em.teq(rec(M_PAT), Em::imm(PAT_LEAF));
                let binder = em.sel(use_x, rec(M_X), rec(M_Y));
                let cell = [
                    binder,
                    rec(M_STAGED + 1),
                    rec(M_STAGED + 2),
                    rec(M_PARTIAL),
                    rec(M_PARTIAL + 1),
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
                h_out[M_PARTIAL] = Some(Em::imm(1));
                h_out[M_PARTIAL + 1] = Some(pre_i[L_ACTIVE + 2].clone());
                h_out[M_STAGED] = Some(Em::imm(0));
                h_out[M_STAGED + 1] = Some(Em::imm(0));
                h_out[M_STAGED + 2] = Some(Em::imm(0));
                // Ready for PVar/PLeaf or after field 1; else stage field 1.
                let second_field = em.teq(rec(M_FIELD_IDX), Em::imm(1));
                let leaf_or_var = em.sel(is_leaf, Em::imm(1), is_var);
                let done = em.sel(second_field, Em::imm(1), leaf_or_var);
                h_out[M_PHASE] = Some(em.sel(done.clone(), Em::imm(PH_READY), Em::imm(PH_FIELD)));
                h_out[M_FIELD_IDX] = Some(em.sel(done, rec(M_FIELD_IDX), Em::imm(1)));
                visits = 1;
            }
            _ => unreachable!("checked by match_helper_supported"),
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

/// One-shot emission of the Match helper relation for `rule`.
pub fn emit_c6_match_helper_block(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<C6MatchHelperBlock, C6MatchHelperError> {
    C6MatchHelperEmitter::new(scope)?.emit(rule, inputs)
}
