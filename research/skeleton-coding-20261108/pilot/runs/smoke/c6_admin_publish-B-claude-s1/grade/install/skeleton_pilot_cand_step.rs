//! C6 admin publish step: the `k.admin.publish.value` row every factorial and
//! tree fold run fires last, as one per-rule relation over the full C6
//! carriers.
//!
//! Reference (`owner_runtime.rs` `step`, P1 at ~3709-3733): with outcome
//! Running, helper and result absent, control `Return { value }` and no frames,
//! the owner sets `outcome = Halted { value }` and returns a Publish event
//! (`publish_event`: charge 0, no alloc deltas). P1 runs before the P2
//! `remaining_work == 0` refusal, so a zero remaining-work pre is accepted.
//!
//! Statement lanes (`C6StepInputs`):
//! - enable, host_schema, carrier_version: `fix_prefix`.
//! - pre/post META, PROG, CTX: `fix_prefix` (unconditional equality + ranges).
//! - pre IMAGE: `validate_pre`, `select_rule` (outcome, control, depth),
//!   `write_outcome` (control value ref 3..5), `preserve_image`.
//! - pre DATA / pre HELPER: `select_rule` (helper/result absent).
//! - post IMAGE 23..26 (outcome tag + ref): `write_outcome` only.
//! - post IMAGE every other lane (header incl. control, fuel/work/cursor
//!   lanes 26..32, alloc/active/seg lanes, frame count, all frame slots):
//!   `preserve_image`.
//! - post DATA: `preserve_data`. post HELPER: `preserve_helper`.
//!
//! Size choices (the relation is otherwise exactly the skeleton's):
//! - Preserved post lanes are fixed by one *unconditional* equality, which
//!   is both the enable = 1 preservation and the enable = 0 identity; the
//!   outcome lanes use `post = enable ? new : pre`. Hence `disabled_identity`
//!   has nothing left to emit, and post lanes need no separate range checks.
//! - Selection states the dispatch owner's `KAdminPublishValue` path
//!   (`running ∧ no_helper ∧ no_result ∧ control == Return ∧ depth == 0`)
//!   directly instead of composing the full dispatch block (PROG walk, data
//!   reads): every other dispatch assertion is gated on facts this path
//!   excludes, so the relation is the same.
//! - Depth is 0, so frame validation reduces to "every slot is zero".
//! - Post DATA/HELPER equal pre, so only pre DATA/HELPER canonicality is
//!   composed.
//!
//! Global invariants:
//! 1. Every post IMAGE/DATA/HELPER lane is fixed under enable = 1 by exactly
//!    one of `write_outcome` (lanes 23..26) or a preserve function (all
//!    others); no lane is left free and none is constrained twice.
//! 2. enable = 0 accepts exactly the identity pair.
//! 3. enable is boolean and every statement lane is range-checked to 128 bits
//!    (directly, or by unconditional equality with a checked lane).
//! 4. Charge is 0: no work, helper, visit, cursor or alloc lane moves, and no
//!    gate requires remaining_work > 0 (P1 precedes P2).
//! 5. The pre carriers decode to an owner state (composed canonicality +
//!    `validate_pre`), and post canonicality is implied by pre canonicality
//!    plus the write (the copied ref is the pre control's validated ref).
//! 6. The relation is per-rule: `emit` rejects every rule not in
//!    `ADMIN_PUBLISH_RULES`.
//! 7. Temporaries stay in the emitter's scope namespace; the retained
//!    `Composed` serials make repeated `emit` calls collision free.
use midnight_zkir_v3::Instruction;
use midnight_zkir_v3::ir::Operand;
use mori_core::sherlock::helper_machine::c6_phys::{
    C6_PHYS_MAGIC, C6_PHYS_VERSION, CONTROL_LANES, IMAGE_SECTION_HEADER_LANES, L_ACTIVE, L_ALLOC,
    L_CONTROL, L_CONTROL_ORIGIN, L_ENV_ROOT, L_FRAME_COUNT, L_MAGIC, L_OUTCOME, L_SEG_ACTIVE,
    L_SEG_ALLOC, L_STACK_LIMIT, L_VERSION,
};
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_HELPER_RECORD, L_RESULT_RECORD};
use mori_core::sherlock::helper_machine::c6_phys_host::C6_HOST_SCHEMA;
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_data_canonical::C6DataCanonicalInputs;
use super::c6_helper_canonical::C6HelperCanonicalInputs;
use super::c6_meta_canonical::C6MetaBoundCanonicalInputs;
use super::c6_program_context_canonical::{
    C6ProgramContextCanonicalInputs, C6ProgramContextCtxBindingInputs,
};
use super::c6_push_step::C6_PADDED_CARRIER_VERSION;
use super::c6_step_kit::{C6StepInputs, Composed, Em, check_shapes};

#[derive(Debug, Error)]
pub enum C6AdminPublishError {
    #[error("rule {0:?} is not an admin publish rule of this fragment")]
    Unsupported(RuleIdC),
    #[error("invalid scope")]
    InvalidScope,
    #[error("C6 admin publish emission: {0}")]
    Emit(String),
}

pub const ADMIN_PUBLISH_RULES: [RuleIdC; 1] = [RuleIdC::KAdminPublishValue];

pub fn admin_publish_supported(rule: RuleIdC) -> bool {
    ADMIN_PUBLISH_RULES.contains(&rule)
}

/// Outcome tag of `Halted` (`c6_phys::put_outcome`).
const OUTCOME_HALTED: u128 = 1;
/// Post image lanes written: outcome tag, ref kind, ref index (23, 24, 25).
const OUTCOME_LANES: [usize; 3] = [L_OUTCOME, L_OUTCOME + 1, L_OUTCOME + 2];
/// Pre control `Return { value }` ref lanes (`put_control`: o[1], o[2]).
const CONTROL_VALUE_REF: [usize; 2] = [L_CONTROL + 1, L_CONTROL + 2];
/// Control tag of `Return` (`put_control`).
const CONTROL_RETURN: u128 = 1;

#[derive(Clone, Debug)]
pub struct C6AdminPublishBlock {
    pub instructions: Vec<Instruction>,
}

pub struct C6AdminPublishEmitter {
    scope: String,
    next_temp: usize,
    composed: Composed,
}

impl C6AdminPublishEmitter {
    /// Requires: nothing. Ensures: `scope` is non-empty, has no `%` and no
    /// `.c6kit.`; the composed emitters are created in that scope.
    pub fn new(scope: &str) -> Result<Self, C6AdminPublishError> {
        if scope.trim().is_empty() || scope.contains('%') || scope.contains(".c6kit.") {
            return Err(C6AdminPublishError::InvalidScope);
        }
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            composed: Composed::new(scope).map_err(C6AdminPublishError::Emit)?,
        })
    }

    /// Requires: `inputs` allocated by `build_step_circuit`.
    /// Ensures: `rule` is supported and shapes check (else Err, no
    /// instructions); the returned block is the whole relation (`body`).
    pub fn emit(
        &mut self,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<C6AdminPublishBlock, C6AdminPublishError> {
        if !admin_publish_supported(rule) {
            return Err(C6AdminPublishError::Unsupported(rule));
        }
        check_shapes(inputs).map_err(C6AdminPublishError::Emit)?;
        let scope = self.scope.clone();
        let mut em = Em {
            scope: &scope,
            next: self.next_temp,
            ins: Vec::new(),
        };
        let r = self.body(&mut em, rule, inputs);
        self.next_temp = em.next;
        r.map_err(C6AdminPublishError::Emit)?;
        Ok(C6AdminPublishBlock {
            instructions: em.ins,
        })
    }

    /// Calls every function below once, in order: fix_prefix, validate_pre,
    /// select_rule, write_outcome, preserve_image, preserve_data,
    /// preserve_helper, compose_canonical, disabled_identity.
    fn body(
        &mut self,
        em: &mut Em<'_>,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<(), String> {
        fix_prefix(em, inputs);
        validate_pre(em, inputs);
        select_rule(&mut self.composed, em, rule, inputs)?;
        write_outcome(em, inputs);
        preserve_image(em, inputs);
        preserve_data(em, inputs);
        preserve_helper(em, inputs);
        compose_canonical(&mut self.composed, em, inputs)?;
        disabled_identity(em, inputs);
        Ok(())
    }
}

/// Ensures: enable ∈ {0,1}; schema/carrier constants; every lane < 2^128;
/// post META/PROG/CTX = pre (ungated); META/CTX magic+version under enable.
///
/// Every pre lane is range-checked here. Every post lane is unconditionally
/// equal to a checked lane (META/PROG/CTX here; non-outcome IMAGE, DATA and
/// HELPER in the preserve functions) or, for the three outcome lanes, to
/// `enable ? (constant | checked pre lane) : checked pre lane` with enable
/// boolean, so post ranges are implied and not re-checked.
fn fix_prefix(em: &mut Em<'_>, inputs: &C6StepInputs) {
    let en = inputs.enable.clone();
    em.cbool(en.clone());
    em.ceq(inputs.host_schema.clone(), Em::imm(C6_HOST_SCHEMA));
    em.ceq(
        inputs.carrier_version.clone(),
        Em::imm(C6_PADDED_CARRIER_VERSION),
    );
    for (a, b) in inputs
        .pre_meta
        .iter()
        .zip(inputs.post_meta.iter())
        .chain(inputs.pre_prog.iter().zip(inputs.post_prog.iter()))
        .chain(inputs.pre_ctx.iter().zip(inputs.post_ctx.iter()))
    {
        em.cbits(a.clone());
        em.ceq(b.clone(), a.clone());
    }
    em.geq(
        en.clone(),
        inputs.pre_meta[0].clone(),
        Em::imm(C6_PHYS_MAGIC),
    );
    em.geq(
        en.clone(),
        inputs.pre_meta[1].clone(),
        Em::imm(C6_PHYS_VERSION),
    );
    em.geq(
        en.clone(),
        inputs.pre_ctx[0].clone(),
        Em::imm(C6_PHYS_MAGIC),
    );
    em.geq(en, inputs.pre_ctx[1].clone(), Em::imm(C6_PHYS_VERSION));
    for l in inputs
        .pre_image
        .iter()
        .chain(inputs.pre_data.iter())
        .chain(inputs.pre_helper.iter())
    {
        em.cbits(l.clone());
    }
}

/// Ensures under enable: the pre image root is representation-valid
/// (`c6_phys` decode / `PreImage::bind`): magic/version, Running payload
/// lanes zero, `Return { value }` control with a capped ref (`cap_ref`) and
/// zero padding, origin/env/stack-limit/alloc/segment caps, ACTIVE = DATA
/// header 3..7, and every frame slot zero (depth 0 is fixed by
/// `select_rule`, so every slot is inactive and no frame is validated).
fn validate_pre(em: &mut Em<'_>, inputs: &C6StepInputs) {
    let en = inputs.enable.clone();
    let caps = &inputs.geometry.caps;
    let p = &caps.phys;
    let pre_i = &inputs.pre_image;
    em.geq(en.clone(), pre_i[L_MAGIC].clone(), Em::imm(C6_PHYS_MAGIC));
    em.geq(
        en.clone(),
        pre_i[L_VERSION].clone(),
        Em::imm(C6_PHYS_VERSION),
    );
    // Running payload (`get_outcome`: tag 0 => lanes at+1..at+3 zero); the
    // tag itself is fixed by `select_rule`.
    for lane in &OUTCOME_LANES[1..] {
        em.gzero(en.clone(), pre_i[*lane].clone());
    }
    // Return { value }: ref kind boolean, index under its cap; padding zero.
    em.gated_validate_ref(en.clone(), pre_i, CONTROL_VALUE_REF[0], p);
    for off in 3..CONTROL_LANES {
        em.gzero(en.clone(), pre_i[L_CONTROL + off].clone());
    }
    em.gated_lt_index(
        en.clone(),
        pre_i[L_CONTROL_ORIGIN].clone(),
        p.max_expressions,
    );
    em.gated_validate_env(
        en.clone(),
        pre_i[L_ENV_ROOT].clone(),
        pre_i[L_ENV_ROOT + 1].clone(),
        p.max_env_cells,
    );
    em.gated_lt_count(en.clone(), pre_i[L_STACK_LIMIT].clone(), p.max_frames);
    for (at, cap) in [
        (L_ALLOC, p.max_data_nodes),
        (L_ALLOC + 1, caps.max_data_field_slots),
        (L_ALLOC + 2, p.max_env_cells),
        (L_SEG_ALLOC, p.max_data_nodes),
        (L_SEG_ALLOC + 1, caps.max_data_field_slots),
        (L_SEG_ALLOC + 2, p.max_env_cells),
        (L_SEG_ACTIVE, p.max_data_nodes),
        (L_SEG_ACTIVE + 1, caps.max_data_field_slots),
        (L_SEG_ACTIVE + 2, p.max_env_cells),
        (L_SEG_ACTIVE + 3, p.max_value_slots),
    ] {
        em.gated_lt_count(en.clone(), pre_i[at].clone(), cap);
    }
    for off in 0..4 {
        em.geq(
            en.clone(),
            pre_i[L_ACTIVE + off].clone(),
            inputs.pre_data[3 + off].clone(),
        );
    }
    // Depth 0: every frame slot is inactive, hence zero.
    for lane in &pre_i[IMAGE_SECTION_HEADER_LANES..] {
        em.gzero(en.clone(), lane.clone());
    }
}

/// Ensures under enable: outcome Running, helper and result absent, control
/// tag Return (1), depth 0, i.e. the owner selects exactly this rule.
///
/// These five premises are exactly the dispatch owner's `path` for
/// `KAdminPublishValue` (`running ∧ quiet ∧ ct == 1 ∧ depth_zero`). The only
/// other assertions the dispatch block emits are gated on an Eval or Cases
/// control tag, which `ct == 1` excludes, so the composed block would add no
/// constraint here; it is not emitted. P0 precedes P1, and P1 precedes the
/// P2 fuel check, so remaining work is not consulted.
fn select_rule(
    _composed: &mut Composed,
    em: &mut Em<'_>,
    rule: RuleIdC,
    inputs: &C6StepInputs,
) -> Result<(), String> {
    if rule != RuleIdC::KAdminPublishValue {
        return Err(format!("{rule:?} is not selected by this fragment"));
    }
    let en = inputs.enable.clone();
    let pre_i = &inputs.pre_image;
    let pre_h = &inputs.pre_helper;
    em.gzero(en.clone(), pre_i[L_OUTCOME].clone()); // P0: Running
    em.gzero(en.clone(), pre_h[L_HELPER_RECORD].clone()); // helper None
    em.gzero(en.clone(), pre_h[L_RESULT_RECORD].clone()); // result None
    em.geq(
        en.clone(),
        pre_i[L_CONTROL].clone(),
        Em::imm(CONTROL_RETURN),
    );
    em.gzero(en, pre_i[L_FRAME_COUNT].clone()); // frames empty
    Ok(())
}

/// Fixes post IMAGE `OUTCOME_LANES` in both enable states:
/// `post = enable ? Halted { pre control value } : pre`, i.e. post[23] = 1,
/// post[24] = pre[3], post[25] = pre[4] under enable (`put_outcome` /
/// `put_ref`). The enable = 0 identity of these lanes is folded in (one
/// select and one equality per lane).
fn write_outcome(em: &mut Em<'_>, inputs: &C6StepInputs) {
    let en = inputs.enable.clone();
    let pre_i = &inputs.pre_image;
    let post_i = &inputs.post_image;
    let new = [
        Em::imm(OUTCOME_HALTED),
        pre_i[CONTROL_VALUE_REF[0]].clone(),
        pre_i[CONTROL_VALUE_REF[1]].clone(),
    ];
    for (lane, want) in OUTCOME_LANES.iter().zip(new) {
        let v = em.sel(en.clone(), want, pre_i[*lane].clone());
        em.ceq(post_i[*lane].clone(), v);
    }
}

/// Fixes post IMAGE every lane except `OUTCOME_LANES` to pre,
/// unconditionally: under enable this is the rule's preservation (charge 0,
/// P1 writes only `img.outcome`); under enable = 0 it is the identity.
fn preserve_image(em: &mut Em<'_>, inputs: &C6StepInputs) {
    for (lane, (a, b)) in inputs
        .pre_image
        .iter()
        .zip(inputs.post_image.iter())
        .enumerate()
    {
        if !OUTCOME_LANES.contains(&lane) {
            em.ceq(b.clone(), a.clone());
        }
    }
}

/// Fixes post DATA = pre DATA (unconditional: preservation and identity).
fn preserve_data(em: &mut Em<'_>, inputs: &C6StepInputs) {
    preserve_all(em, &inputs.pre_data, &inputs.post_data);
}

/// Fixes post HELPER = pre HELPER (unconditional: preservation and identity;
/// absent helper/result stay absent).
fn preserve_helper(em: &mut Em<'_>, inputs: &C6StepInputs) {
    preserve_all(em, &inputs.pre_helper, &inputs.post_helper);
}

fn preserve_all(em: &mut Em<'_>, pre: &[Operand], post: &[Operand]) {
    for (a, b) in pre.iter().zip(post.iter()) {
        em.ceq(b.clone(), a.clone());
    }
}

/// Ensures under enable: every carrier is canonical. META uses the bound
/// emitter over (pre, post); PROG/CTX and the CTX binding over pre (post =
/// pre); DATA and HELPER over pre only, since post equals pre exactly.
fn compose_canonical(
    composed: &mut Composed,
    em: &mut Em<'_>,
    inputs: &C6StepInputs,
) -> Result<(), String> {
    let en = inputs.enable.clone();
    let caps = inputs.geometry.caps;
    let mb = composed
        .meta_em
        .emit_bound(&C6MetaBoundCanonicalInputs {
            gate: en.clone(),
            pre_meta: inputs.pre_meta.clone(),
            post_meta: inputs.post_meta.clone(),
            limits: inputs.source_params.limits(),
            meta_bounds_frames_offset: inputs.meta_bounds_frames_offset,
            binding: inputs.source_params.meta_binding(),
        })
        .map_err(|e| e.to_string())?;
    em.ins.extend(mb.instructions);
    let pc = composed
        .progctx_em
        .emit(&C6ProgramContextCanonicalInputs {
            gate: en.clone(),
            pre_prog: inputs.pre_prog.clone(),
            pre_ctx: inputs.pre_ctx.clone(),
            caps,
        })
        .map_err(|e| e.to_string())?;
    em.ins.extend(pc.instructions);
    let cx = composed
        .progctx_em
        .emit_ctx_binding(&C6ProgramContextCtxBindingInputs {
            gate: en.clone(),
            pre_ctx: inputs.pre_ctx.clone(),
            expected_ctx: inputs.source_params.expected_ctx(),
        })
        .map_err(|e| e.to_string())?;
    em.ins.extend(cx.instructions);
    let db = composed
        .data_em
        .emit(&C6DataCanonicalInputs {
            gate: en.clone(),
            data: inputs.pre_data.clone(),
            caps,
        })
        .map_err(|e| e.to_string())?;
    em.ins.extend(db.instructions);
    let hb = composed
        .helper_em
        .emit(&C6HelperCanonicalInputs {
            gate: en,
            helper: inputs.pre_helper.clone(),
            caps,
        })
        .map_err(|e| e.to_string())?;
    em.ins.extend(hb.instructions);
    Ok(())
}

/// Ensures under enable = 0: post = pre on every IMAGE/DATA/HELPER lane.
///
/// Emits nothing: every non-outcome IMAGE lane and every DATA/HELPER lane is
/// already equal to pre unconditionally (preserve functions), and the outcome
/// lanes select pre when enable = 0 (`write_outcome`). Every other gate is
/// selected on enable, so enable = 0 leaves only the identity plus the
/// ungated prefix and range checks, which an identity pair over a
/// range-valid pre satisfies.
fn disabled_identity(_em: &mut Em<'_>, _inputs: &C6StepInputs) {}
