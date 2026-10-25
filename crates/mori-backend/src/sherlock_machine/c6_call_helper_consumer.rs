//! C6 call helper (Find/EnvGet) consumer: host-pair witness and real SDK builder/checker
//! for `c6_call_helper_step` (LocalPublic full pre+post statement).
//! `ProofPreimage::check` is the SDK's nonproving preprocessing check.
use midnight_transient_crypto::curve::Fr;
use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostConfig, C6HostLimits};
use mori_core::sherlock::helper_machine::complete_observation::HostSnapshot;
use mori_core::sherlock::helper_machine::shared::RuleIdC;

use super::c6_call_helper_step::{C6CallHelperEmitter, call_helper_supported};
use super::c6_push_step::C6PushFamilyParams;
use super::c6_step_harness::{
    C6StepCircuit, C6StepWitness, build_step_circuit, check_step_circuit,
    step_witness_from_host_pair,
};

pub type C6CallHelperWitness = C6StepWitness;
pub type C6CallHelperCircuit = C6StepCircuit;

/// Independent pre and post encodings of one owner step.
pub fn witness_from_host_pair(
    enable: bool,
    pre: &HostSnapshot,
    post: &HostSnapshot,
    cfg: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<C6CallHelperWitness, String> {
    step_witness_from_host_pair(enable, pre, post, cfg, limits)
}

/// One circuit per rule; the rule is a build-time constant.
pub fn build_c6_call_helper_circuit(
    rule: RuleIdC,
    source_params: C6PushFamilyParams,
    meta_len: usize,
    prog_len: usize,
    meta_bounds_frames_offset: usize,
    scope: &str,
) -> Result<C6CallHelperCircuit, String> {
    if !call_helper_supported(rule) {
        return Err(format!("{rule:?} is not a Find/EnvGet helper rule"));
    }
    build_step_circuit(
        format!("sherlock_c6_call_helper_{rule:?}"),
        source_params,
        meta_len,
        prog_len,
        meta_bounds_frames_offset,
        scope,
        |inputs| {
            let mut em = C6CallHelperEmitter::new(&format!("{scope}.callhelper"))
                .map_err(|e| e.to_string())?;
            Ok(em
                .emit(rule, inputs)
                .map_err(|e| e.to_string())?
                .instructions)
        },
    )
}

/// `ProofPreimage::check` preprocessing only (nonproving).
pub fn check_c6_call_helper_circuit(
    circuit: &C6CallHelperCircuit,
    witness: &C6CallHelperWitness,
) -> Result<Vec<Option<usize>>, String> {
    check_step_circuit(circuit, witness)
}

/// Statement lanes of a witness (diagnostic).
pub fn call_helper_statement(witness: &C6CallHelperWitness) -> Vec<Fr> {
    witness.to_statement_frs()
}

#[cfg(test)]
#[path = "c6_call_helper_tests.rs"]
mod c6_call_helper_tests;
