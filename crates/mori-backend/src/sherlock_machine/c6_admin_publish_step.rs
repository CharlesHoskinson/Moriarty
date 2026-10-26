//! C6 admin publish step: the `k.admin.publish.value` row every
//! factorial and tree fold run fires last, as one per-rule relation
//! over the full C6 carriers.
//!
//! The circuit fixes its rule at build time and selects it with the
//! composed dispatch owner (P1 publication: outcome Running, helper
//! and result absent, control `Return { value }`, depth 0). The only
//! write is the outcome: `Halted { value }` (image lanes 23-25), with
//! the value reference copied from the preserved Return control.
//! Charge is 0 (publication fires before the P2 fuel refusal, so a
//! zero remaining-work pre-state is accepted); every other image,
//! DATA and helper lane is preserved exactly. Canonicality is
//! composed; enable = 0 is identity.
use midnight_zkir_v3::Instruction;
use mori_core::sherlock::helper_machine::c6_phys::{L_CONTROL, L_OUTCOME};
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_step_kit::{
    C6StepInputs, Composed, Em, check_shapes, compose_canonical, disabled_identity, prefix,
    preserve_image, select, validate_pre_image,
};

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

const OUTCOME_HALTED: u128 = 1;

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

    fn body(&mut self, em: &mut Em, _rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        let en = inputs.enable.clone();
        prefix(em, inputs);
        let _top = validate_pre_image(em, inputs);
        select(&mut self.composed, em, _rule, inputs)?;

        let pre_i = &inputs.pre_image;
        let post_i = &inputs.post_image;
        // Dispatch (P1) owns outcome Running, helper/result absent,
        // control Return and depth 0; the write is Halted { value }.
        em.geq(
            en.clone(),
            post_i[L_OUTCOME].clone(),
            Em::imm(OUTCOME_HALTED),
        );
        em.geq(
            en.clone(),
            post_i[L_OUTCOME + 1].clone(),
            pre_i[L_CONTROL + 1].clone(),
        );
        em.geq(
            en.clone(),
            post_i[L_OUTCOME + 2].clone(),
            pre_i[L_CONTROL + 2].clone(),
        );
        preserve_image(
            em,
            inputs,
            &[L_OUTCOME, L_OUTCOME + 1, L_OUTCOME + 2],
            false,
        );
        for (a, b) in inputs.pre_data.iter().zip(inputs.post_data.iter()) {
            em.geq(en.clone(), b.clone(), a.clone());
        }
        for (a, b) in inputs.pre_helper.iter().zip(inputs.post_helper.iter()) {
            em.geq(en.clone(), b.clone(), a.clone());
        }
        compose_canonical(&mut self.composed, em, inputs)?;
        disabled_identity(em, inputs);
        Ok(())
    }
}

/// One-shot emission of the admin publish relation for `rule`.
pub fn emit_c6_admin_publish_block(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<C6AdminPublishBlock, C6AdminPublishError> {
    C6AdminPublishEmitter::new(scope)?.emit(rule, inputs)
}
