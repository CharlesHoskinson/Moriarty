//! C6 Eval schedule consumer: host-pair witness and real SDK builder/checker
//! for `c6_eval_schedule_step` (LocalPublic full pre+post statement).
//! `ProofPreimage::check` is the SDK's nonproving preprocessing check.
use midnight_transient_crypto::curve::Fr;
use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostConfig, C6HostLimits};
use mori_core::sherlock::helper_machine::complete_observation::HostSnapshot;
use mori_core::sherlock::helper_machine::shared::RuleIdC;

use super::c6_eval_schedule_step::{C6EvalScheduleEmitter, eval_schedule_supported};
use super::c6_push_step::C6PushFamilyParams;
use super::c6_step_harness::{
    C6StepCircuit, C6StepWitness, build_step_circuit, check_step_circuit,
    step_witness_from_host_pair,
};

pub type C6EvalScheduleWitness = C6StepWitness;
pub type C6EvalScheduleCircuit = C6StepCircuit;

/// Independent pre and post encodings of one owner step.
pub fn witness_from_host_pair(
    enable: bool,
    pre: &HostSnapshot,
    post: &HostSnapshot,
    cfg: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<C6EvalScheduleWitness, String> {
    step_witness_from_host_pair(enable, pre, post, cfg, limits)
}

/// One circuit per rule; the rule is a build-time constant.
pub fn build_c6_eval_schedule_circuit(
    rule: RuleIdC,
    source_params: C6PushFamilyParams,
    meta_len: usize,
    prog_len: usize,
    meta_bounds_frames_offset: usize,
    scope: &str,
) -> Result<C6EvalScheduleCircuit, String> {
    if !eval_schedule_supported(rule) {
        return Err(format!("{rule:?} is not an Eval schedule rule"));
    }
    build_step_circuit(
        format!("sherlock_c6_eval_schedule_{rule:?}"),
        source_params,
        meta_len,
        prog_len,
        meta_bounds_frames_offset,
        scope,
        |inputs| {
            let mut em = C6EvalScheduleEmitter::new(&format!("{scope}.evalsched"))
                .map_err(|e| e.to_string())?;
            Ok(em
                .emit(rule, inputs)
                .map_err(|e| e.to_string())?
                .instructions)
        },
    )
}

/// `ProofPreimage::check` preprocessing only (nonproving).
pub fn check_c6_eval_schedule_circuit(
    circuit: &C6EvalScheduleCircuit,
    witness: &C6EvalScheduleWitness,
) -> Result<Vec<Option<usize>>, String> {
    check_step_circuit(circuit, witness)
}

/// Statement lanes of a witness (diagnostic).
pub fn eval_schedule_statement(witness: &C6EvalScheduleWitness) -> Vec<Fr> {
    witness.to_statement_frs()
}

#[cfg(test)]
#[path = "c6_eval_schedule_tests.rs"]
mod c6_eval_schedule_tests;
