//! Pilot grader adaptor (throwaway grader export only).
use midnight_zkir_v3::Instruction;
use mori_core::sherlock::helper_machine::shared::RuleIdC;

use super::c6_step_kit::C6StepInputs;

pub(super) fn emit(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<Vec<Instruction>, String> {
    let mut e = super::skeleton_pilot_cand_step::C6CallBindEmitter::new(scope).map_err(|e| e.to_string())?;
    Ok(e.emit(rule, inputs).map_err(|e| e.to_string())?.instructions)
}
