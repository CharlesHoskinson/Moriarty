//! C6 CallBind control consumer: host-pair witness and real SDK builder/checker
//! for `c6_call_bind_step` (LocalPublic full pre+post statement).
//! `ProofPreimage::check` is the SDK's nonproving preprocessing check.
use midnight_transient_crypto::curve::Fr;
use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostConfig, C6HostLimits};
use mori_core::sherlock::helper_machine::complete_observation::HostSnapshot;
use mori_core::sherlock::helper_machine::shared::RuleIdC;

use super::c6_call_bind_step::{C6CallBindEmitter, call_bind_supported};
use super::c6_push_step::C6PushFamilyParams;
use super::c6_step_harness::{
    C6StepCircuit, C6StepWitness, build_step_circuit, check_step_circuit,
    step_witness_from_host_pair,
};

pub type C6CallBindWitness = C6StepWitness;
pub type C6CallBindCircuit = C6StepCircuit;

/// Independent pre and post encodings of one owner step.
pub fn witness_from_host_pair(
    enable: bool,
    pre: &HostSnapshot,
    post: &HostSnapshot,
    cfg: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<C6CallBindWitness, String> {
    step_witness_from_host_pair(enable, pre, post, cfg, limits)
}

/// One circuit per rule; the rule is a build-time constant.
pub fn build_c6_call_bind_circuit(
    rule: RuleIdC,
    source_params: C6PushFamilyParams,
    meta_len: usize,
    prog_len: usize,
    meta_bounds_frames_offset: usize,
    scope: &str,
) -> Result<C6CallBindCircuit, String> {
    if !call_bind_supported(rule) {
        return Err(format!("{rule:?} is not a CallBind control rule"));
    }
    build_step_circuit(
        format!("sherlock_c6_call_bind_{rule:?}"),
        source_params,
        meta_len,
        prog_len,
        meta_bounds_frames_offset,
        scope,
        |inputs| {
            let mut em =
                C6CallBindEmitter::new(&format!("{scope}.callbind")).map_err(|e| e.to_string())?;
            Ok(em
                .emit(rule, inputs)
                .map_err(|e| e.to_string())?
                .instructions)
        },
    )
}

/// `ProofPreimage::check` preprocessing only (nonproving).
pub fn check_c6_call_bind_circuit(
    circuit: &C6CallBindCircuit,
    witness: &C6CallBindWitness,
) -> Result<Vec<Option<usize>>, String> {
    check_step_circuit(circuit, witness)
}

/// Statement lanes of a witness (diagnostic).
pub fn call_bind_statement(witness: &C6CallBindWitness) -> Vec<Fr> {
    witness.to_statement_frs()
}

#[cfg(test)]
#[path = "c6_call_bind_tests.rs"]
mod tests;
