//! C6 dispatch owner consumer: host-snapshot witness and real SDK
//! builder/checker for the per-rule dispatch relation of `c6_dispatch_step`.
//!
//! Statement (LocalPublic): enable, host_schema, carrier_version, then the
//! pre META, PROG, CTX, IMAGE, DATA and HELPER lanes. One circuit per rule; the
//! rule is a build-time constant, so a verifier that knows the key knows the
//! rule. `ProofPreimage::check` here is the SDK's nonproving preprocessing
//! check, not synthesis, keygen, proving or native verification.
use std::sync::Arc;

use midnight_transient_crypto::curve::Fr;
use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage};
use midnight_zkir_v3::ir::{IrMinorVersion, Operand};
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_core::sherlock::helper_machine::c6_phys_data::C6StateCaps;
use mori_core::sherlock::helper_machine::c6_phys_host::{
    C6_HOST_SCHEMA, C6HostConfig, C6HostLimits, CTX_LANES,
};
use mori_core::sherlock::helper_machine::c6_phys_state::{C6OwnedState, C6StateLimits};
use mori_core::sherlock::helper_machine::complete_observation::{HostSnapshot, ImageSnapshot};
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use serde_json::json;
use thiserror::Error;

use super::c6_dispatch_step::{C6DispatchError, C6DispatchInputs, emit_c6_dispatch_block};
use super::c6_push_step::{
    C6_PADDED_CARRIER_VERSION, C6PaddedCarrierGeometry, C6PushFamilyParams, C6PushHostPrefixLanes,
    C6PushStepError,
};

#[derive(Debug, Error)]
pub enum C6DispatchConsumerError {
    #[error("length mismatch in {section}: expected {expected}, got {actual}")]
    LengthMismatch {
        section: &'static str,
        expected: usize,
        actual: usize,
    },
    #[error("core C6 physical/push error: {0}")]
    Push(#[from] C6PushStepError),
    #[error("C6 dispatch emitter error: {0}")]
    Dispatch(#[from] C6DispatchError),
    #[error("SDK backend error: {0}")]
    Backend(String),
}

fn be<E: std::fmt::Display>(e: E) -> C6DispatchConsumerError {
    C6DispatchConsumerError::Backend(e.to_string())
}

#[derive(Clone, Debug, PartialEq)]
pub struct C6DispatchWitness {
    enable: bool,
    source_params: C6PushFamilyParams,
    host_prefix: C6PushHostPrefixLanes,
    pre_image: Vec<u128>,
    pre_data: Vec<u128>,
    pre_helper: Vec<u128>,
}

impl C6DispatchWitness {
    pub fn source_params(&self) -> &C6PushFamilyParams {
        &self.source_params
    }
    pub fn pre_image_mut(&mut self) -> &mut Vec<u128> {
        &mut self.pre_image
    }
    pub fn pre_helper_mut(&mut self) -> &mut Vec<u128> {
        &mut self.pre_helper
    }
    pub fn to_statement_frs(&self) -> Vec<Fr> {
        let mut out = vec![
            Fr::from(self.enable),
            Fr::from(C6_HOST_SCHEMA),
            Fr::from(C6_PADDED_CARRIER_VERSION),
        ];
        for l in self
            .host_prefix
            .meta
            .iter()
            .chain(&self.host_prefix.prog)
            .chain(self.host_prefix.ctx.iter())
            .chain(&self.pre_image)
            .chain(&self.pre_data)
            .chain(&self.pre_helper)
        {
            out.push(Fr::from(*l));
        }
        out
    }
}

type Sections = (Vec<u128>, Vec<u128>, Vec<u128>);

fn split_state(
    state: &ImageSnapshot,
    caps: C6StateCaps,
    limits: C6HostLimits,
    geom: C6PaddedCarrierGeometry,
) -> Result<Sections, C6DispatchConsumerError> {
    let padded = super::c6_push_step::encode_owned_state_to_padded(
        &C6OwnedState::from_image(state),
        caps,
        C6StateLimits {
            max_total_lanes: limits.max_state_lanes,
        },
    )?;
    let flat = padded.to_flat_lanes();
    let i = geom.image_total_lanes;
    let d = i + geom.data_total_lanes;
    Ok((flat[..i].to_vec(), flat[i..d].to_vec(), flat[d..].to_vec()))
}

/// The actual pre snapshot; no rule or selection is supplied by the host.
pub fn witness_from_host_snapshot(
    enable: bool,
    pre: &HostSnapshot,
    cfg: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<C6DispatchWitness, C6DispatchConsumerError> {
    let params = C6PushFamilyParams::new(cfg.clone(), pre.admission.bounds, *limits)?;
    let geom = params.geometry();
    let caps = params.caps();
    let host_prefix = C6PushHostPrefixLanes::from_host_snapshot(pre, cfg, limits)?;
    let (pre_image, pre_data, pre_helper) = split_state(&pre.image, caps, *limits, geom)?;
    Ok(C6DispatchWitness {
        enable,
        source_params: params,
        host_prefix,
        pre_image,
        pre_data,
        pre_helper,
    })
}

pub struct C6DispatchCircuit {
    rule: RuleIdC,
    meta_len: usize,
    prog_len: usize,
    source_params: C6PushFamilyParams,
    statement_lane_count: usize,
    ir: IrSource,
}

impl C6DispatchCircuit {
    pub fn rule(&self) -> RuleIdC {
        self.rule
    }
    pub fn statement_lane_count(&self) -> usize {
        self.statement_lane_count
    }
}

pub fn build_c6_dispatch_circuit(
    rule: RuleIdC,
    source_params: C6PushFamilyParams,
    meta_len: usize,
    prog_len: usize,
    scope: &str,
) -> Result<C6DispatchCircuit, C6DispatchConsumerError> {
    let geometry = source_params.geometry();
    let mut ins = Vec::new();
    let mut tr = Vec::new();
    let mut alloc = |label: &str, idx: usize, ins: &mut Vec<Instruction>| {
        let id = Identifier(format!("%{scope}.public.{label}.{idx}"));
        ins.push(Instruction::PublicInput {
            guard: None,
            val_t: IrType::Native,
            output: id.clone(),
        });
        let op = Operand::Variable(id);
        tr.push(op.clone());
        op
    };
    let enable = alloc("enable", 0, &mut ins);
    let schema = alloc("host_schema", 0, &mut ins);
    let carrier = alloc("carrier_version", 0, &mut ins);
    let mut vec_of = |label: &str, n: usize, ins: &mut Vec<Instruction>| {
        (0..n).map(|i| alloc(label, i, ins)).collect::<Vec<_>>()
    };
    let _pre_meta = vec_of("pre_meta", meta_len, &mut ins);
    let pre_prog = vec_of("pre_prog", prog_len, &mut ins);
    let _pre_ctx = vec_of("pre_ctx", CTX_LANES, &mut ins);
    let pre_image = vec_of("pre_image", geometry.image_total_lanes, &mut ins);
    let pre_data = vec_of("pre_data", geometry.data_total_lanes, &mut ins);
    let pre_helper = vec_of("pre_helper", geometry.helper_total_lanes, &mut ins);
    // Schema and carrier version are fixed constants of this relation.
    for (lane, value) in [
        (schema, C6_HOST_SCHEMA),
        (carrier, C6_PADDED_CARRIER_VERSION),
    ] {
        let id = Identifier(format!("%{scope}.local.fixed.{value}"));
        ins.push(Instruction::CondSelect {
            bit: enable.clone(),
            a: lane,
            b: Operand::Immediate(Fr::from(value)),
            output: id.clone(),
        });
        ins.push(Instruction::ConstrainEq {
            a: Operand::Variable(id),
            b: Operand::Immediate(Fr::from(value)),
        });
    }
    let block = emit_c6_dispatch_block(
        rule,
        &C6DispatchInputs {
            enable,
            caps: geometry.caps,
            pre_prog,
            pre_image,
            pre_data,
            pre_helper,
        },
        scope,
    )?;
    ins.extend(block.instructions);
    let statement_lane_count = tr.len();
    ins.push(Instruction::Impact {
        guard: Operand::Immediate(Fr::from(1u64)),
        inputs: tr,
    });
    let ir0 = IrSource {
        version: IrMinorVersion::V0,
        inputs: Vec::new(),
        outputs: Vec::new(),
        do_communications_commitment: false,
        instructions: Arc::new(ins),
    };
    let mut disk = serde_json::to_value(&ir0).map_err(be)?;
    disk["version"] = json!({ "major": 3, "minor": 0 });
    let serialized = serde_json::to_string(&disk).map_err(be)?;
    let ir = IrSource::load(serialized.as_bytes()).map_err(be)?;
    Ok(C6DispatchCircuit {
        rule,
        meta_len,
        prog_len,
        source_params,
        statement_lane_count,
        ir,
    })
}

/// `ProofPreimage::check` preprocessing only (nonproving).
pub fn check_c6_dispatch_circuit(
    circuit: &C6DispatchCircuit,
    witness: &C6DispatchWitness,
) -> Result<Vec<Option<usize>>, C6DispatchConsumerError> {
    if witness.source_params != circuit.source_params {
        return Err(C6DispatchConsumerError::Backend(
            "witness retained family does not match circuit".into(),
        ));
    }
    for (section, expected, actual) in [
        ("meta", circuit.meta_len, witness.host_prefix.meta.len()),
        ("prog", circuit.prog_len, witness.host_prefix.prog.len()),
    ] {
        if expected != actual {
            return Err(C6DispatchConsumerError::LengthMismatch {
                section,
                expected,
                actual,
            });
        }
    }
    let statement = witness.to_statement_frs();
    if statement.len() != circuit.statement_lane_count {
        return Err(C6DispatchConsumerError::LengthMismatch {
            section: "statement",
            expected: circuit.statement_lane_count,
            actual: statement.len(),
        });
    }
    let preimage = ProofPreimage {
        inputs: Vec::new(),
        private_transcript: Vec::new(),
        public_transcript_inputs: statement.clone(),
        public_transcript_outputs: statement,
        binding_input: Fr::from(0u64),
        communications_commitment: None,
        key_location: KeyLocation(format!("sherlock_c6_dispatch_{:?}", circuit.rule).into()),
    };
    preimage.check(&circuit.ir).map_err(|e| {
        C6DispatchConsumerError::Backend(format!("SDK ProofPreimage::check failed: {e}"))
    })
}

#[cfg(test)]
#[path = "c6_dispatch_tests.rs"]
mod tests;
