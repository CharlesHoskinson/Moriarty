//! Candidate slot of the skeleton-coding pilot grader (`skeleton_pilot`).
//!
//! On `main` no candidate is installed. `scripts/qa/skeleton-pilot.py grade`
//! overwrites this file in a throwaway worktree with an adaptor to the
//! candidate emitter (installed as `skeleton_pilot_cand_step.rs`); that
//! worktree never lands.
use midnight_zkir_v3::Instruction;
use mori_core::sherlock::helper_machine::shared::RuleIdC;

use super::c6_step_kit::C6StepInputs;

pub(super) fn emit(
    _rule: RuleIdC,
    _inputs: &C6StepInputs,
    _scope: &str,
) -> Result<Vec<Instruction>, String> {
    Err("no skeleton pilot candidate is installed".into())
}
