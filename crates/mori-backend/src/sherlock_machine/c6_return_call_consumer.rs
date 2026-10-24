//! C6 call-return HostSnapshot-pair witness and actual SDK builder/checker.
//! UNCOMPILED_UNRUN source. No owner-record provenance or validation API is invented.
//! Host pairs are encoded independently; a wrong encodable post is not filtered.
//! Selector true/false is constrained to actual PRE ScalarTypeTest.ok in the emitter.
//! Both LocalPublic and LedgerCall consume the same full statement; LedgerCall binds
//! the constrained post operands and recomputes CURRENT statement+CURRENT post CC.
//! Native key issuance, program membership/authentication, predecessor SI/StepInv,
//! proof synthesis/verify/degree and full39/target acceptance remain unestablished.
//! The preprocessing checker below must not be called before independent review and
//! Root's existing native/SDK execution contract. Source qualification is separate.

use std::sync::Arc;

use midnight_transient_crypto::curve::Fr;
use midnight_transient_crypto::hash::transient_commit;
use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage, Zkir};
use midnight_zkir_v3::ir::{IrMinorVersion, Operand, TypedIdentifier};
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_core::sherlock::helper_machine::c6_phys_data::C6StateCaps;
use mori_core::sherlock::helper_machine::c6_phys_host::{
    C6_HOST_SCHEMA, C6HostConfig, C6HostLimits, CTX_LANES,
};
use mori_core::sherlock::helper_machine::c6_phys_state::{C6OwnedState, C6StateLimits};
use mori_core::sherlock::helper_machine::complete_observation::{HostSnapshot, ImageSnapshot};

use serde_json::json;
use thiserror::Error;

use super::c6_push_step::{
    C6_PADDED_CARRIER_VERSION, C6PaddedCarrierGeometry, C6PushFamilyParams, C6PushHostPrefixLanes,
    C6PushStatementBindingMode, C6PushStepError,
};
use super::c6_return_call_step::{
    C6ReturnCallStepError, C6ReturnCallStepInputs, emit_c6_return_call_step_block,
};

const CHECK_KEY_LOCATION: &str = "sherlock_c6_return_call_step_check";

#[derive(Debug, Error)]
pub enum C6ReturnCallConsumerError {
    #[error("host prefix section {section} mismatch between pre and post state")]
    HostPrefixMismatch { section: &'static str },
    #[error("length mismatch in {section}: expected {expected}, got {actual}")]
    LengthMismatch {
        section: &'static str,
        expected: usize,
        actual: usize,
    },
    #[error("core C6 physical/push error: {0}")]
    Push(#[from] C6PushStepError),
    #[error("C6 ReturnCall emitter error: {0}")]
    ReturnCall(#[from] C6ReturnCallStepError),
    #[error("composed canonical emitter or SDK backend error: {0}")]
    Backend(String),
}

fn be<E: std::fmt::Display>(e: E) -> C6ReturnCallConsumerError {
    C6ReturnCallConsumerError::Backend(e.to_string())
}

#[derive(Clone, Debug, PartialEq)]
pub struct C6ReturnCallStepWitness {
    enable: bool,
    selector_is_valid: bool,
    source_params: C6PushFamilyParams,
    host_prefix: C6PushHostPrefixLanes,
    pre_image: Vec<u128>,
    pre_data: Vec<u128>,
    pre_helper: Vec<u128>,
    post_image: Vec<u128>,
    post_data: Vec<u128>,
    post_helper: Vec<u128>,
}

impl C6ReturnCallStepWitness {
    pub fn source_params(&self) -> &C6PushFamilyParams {
        &self.source_params
    }
    pub fn enable(&self) -> bool {
        self.enable
    }
    pub fn selector_is_valid(&self) -> bool {
        self.selector_is_valid
    }
    pub fn host_prefix(&self) -> &C6PushHostPrefixLanes {
        &self.host_prefix
    }
    pub fn pre_image(&self) -> &[u128] {
        &self.pre_image
    }
    pub fn pre_data(&self) -> &[u128] {
        &self.pre_data
    }
    pub fn pre_helper(&self) -> &[u128] {
        &self.pre_helper
    }
    pub fn post_image(&self) -> &[u128] {
        &self.post_image
    }
    pub fn post_data(&self) -> &[u128] {
        &self.post_data
    }
    pub fn post_helper(&self) -> &[u128] {
        &self.post_helper
    }

    pub fn to_statement_frs(&self) -> Vec<Fr> {
        let mut out = Vec::new();
        out.push(Fr::from(self.enable));
        out.push(Fr::from(self.selector_is_valid));
        out.push(Fr::from(C6_HOST_SCHEMA));
        out.push(Fr::from(C6_PADDED_CARRIER_VERSION));
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
        for l in self
            .host_prefix
            .meta
            .iter()
            .chain(&self.host_prefix.prog)
            .chain(self.host_prefix.ctx.iter())
            .chain(&self.post_image)
            .chain(&self.post_data)
            .chain(&self.post_helper)
        {
            out.push(Fr::from(*l));
        }
        out
    }

    fn post_flat(&self) -> Vec<u128> {
        let mut v = self.post_image.clone();
        v.extend(&self.post_data);
        v.extend(&self.post_helper);
        v
    }
}

fn split_state(
    state: &ImageSnapshot,
    caps: C6StateCaps,
    limits: C6HostLimits,
    geom: C6PaddedCarrierGeometry,
) -> Result<(Vec<u128>, Vec<u128>, Vec<u128>), C6ReturnCallConsumerError> {
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

/// Independent post route: only config/prefix/codec shape is checked here.
/// Selector is an explicit input, constrained to the PRE staged result by the emitter.
/// An encodable wrong post reaches SDK constraints with its current commitment.
pub fn witness_from_host_pair(
    enable: bool,
    selector_is_valid: bool,
    pre: &HostSnapshot,
    post: &HostSnapshot,
    cfg: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<C6ReturnCallStepWitness, C6ReturnCallConsumerError> {
    let params = C6PushFamilyParams::new(cfg.clone(), pre.admission.bounds, *limits)?;
    let geom = params.geometry();
    let caps = params.caps();
    let pp = C6PushHostPrefixLanes::from_host_snapshot(pre, cfg, limits)?;
    let qp = C6PushHostPrefixLanes::from_host_snapshot(post, cfg, limits)?;
    if pp.meta != qp.meta {
        return Err(C6ReturnCallConsumerError::HostPrefixMismatch { section: "meta" });
    }
    if pp.prog != qp.prog {
        return Err(C6ReturnCallConsumerError::HostPrefixMismatch { section: "prog" });
    }
    if pp.ctx != qp.ctx {
        return Err(C6ReturnCallConsumerError::HostPrefixMismatch { section: "ctx" });
    }
    let (pre_image, pre_data, pre_helper) = split_state(&pre.image, caps, *limits, geom)?;
    let (post_image, post_data, post_helper) = split_state(&post.image, caps, *limits, geom)?;
    Ok(C6ReturnCallStepWitness {
        enable,
        selector_is_valid,
        source_params: params,
        host_prefix: pp,
        pre_image,
        pre_data,
        pre_helper,
        post_image,
        post_data,
        post_helper,
    })
}

pub struct C6ReturnCallStepCircuit {
    mode: C6PushStatementBindingMode,
    geometry: C6PaddedCarrierGeometry,
    meta_len: usize,
    prog_len: usize,
    meta_bounds_frames_offset: usize,
    source_params: C6PushFamilyParams,
    statement_lane_count: usize,
    ir: IrSource,
    serialized_zkir: String,
}

impl C6ReturnCallStepCircuit {
    pub fn source_params(&self) -> &C6PushFamilyParams {
        &self.source_params
    }
    pub fn geometry(&self) -> C6PaddedCarrierGeometry {
        self.geometry
    }
    pub fn meta_len(&self) -> usize {
        self.meta_len
    }
    pub fn prog_len(&self) -> usize {
        self.prog_len
    }
    pub fn meta_bounds_frames_offset(&self) -> usize {
        self.meta_bounds_frames_offset
    }
    pub fn statement_lane_count(&self) -> usize {
        self.statement_lane_count
    }
}

fn make_typed(id: Identifier) -> Result<TypedIdentifier, C6ReturnCallConsumerError> {
    serde_json::from_value(json!({ "name": id, "type": IrType::Native })).map_err(be)
}

#[allow(clippy::too_many_arguments)]
fn alloc_lane(
    mode: C6PushStatementBindingMode,
    scope: &str,
    label: &str,
    idx: usize,
    inputs: &mut Vec<TypedIdentifier>,
    ins: &mut Vec<Instruction>,
    tr: &mut Vec<Operand>,
) -> Result<Operand, C6ReturnCallConsumerError> {
    let prefix = match mode {
        C6PushStatementBindingMode::LocalPublic => "public",
        C6PushStatementBindingMode::LedgerCall => "input",
    };
    let id = Identifier(format!("%{scope}.{prefix}.{label}.{idx}"));
    match mode {
        C6PushStatementBindingMode::LocalPublic => ins.push(Instruction::PublicInput {
            guard: None,
            val_t: IrType::Native,
            output: id.clone(),
        }),
        C6PushStatementBindingMode::LedgerCall => inputs.push(make_typed(id.clone())?),
    }
    let op = Operand::Variable(id);
    tr.push(op.clone());
    Ok(op)
}

pub fn build_c6_return_call_step_circuit(
    source_params: C6PushFamilyParams,
    meta_len: usize,
    prog_len: usize,
    meta_bounds_frames_offset: usize,
    mode: C6PushStatementBindingMode,
    scope: &str,
) -> Result<C6ReturnCallStepCircuit, C6ReturnCallConsumerError> {
    if scope.trim().is_empty() {
        return Err(C6ReturnCallStepError::EmptyScope.into());
    }
    let geometry = source_params.geometry();
    let mut call_inputs = Vec::new();
    let mut ins = Vec::new();
    let mut tr = Vec::new();
    let mut alloc = |label: &str,
                     idx: usize,
                     ci: &mut Vec<TypedIdentifier>,
                     ii: &mut Vec<Instruction>,
                     tt: &mut Vec<Operand>|
     -> Result<Operand, C6ReturnCallConsumerError> {
        alloc_lane(mode, scope, label, idx, ci, ii, tt)
    };
    let enable = alloc("enable", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let valid = alloc("selector_is_valid", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let schema = alloc("host_schema", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let carrier = alloc("carrier_version", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let mut vec_of = |label: &str,
                      n: usize,
                      ci: &mut Vec<TypedIdentifier>,
                      ii: &mut Vec<Instruction>,
                      tt: &mut Vec<Operand>|
     -> Result<Vec<Operand>, C6ReturnCallConsumerError> {
        let mut o = Vec::with_capacity(n);
        for i in 0..n {
            o.push(alloc(label, i, ci, ii, tt)?);
        }
        Ok(o)
    };
    let pre_meta = vec_of("pre_meta", meta_len, &mut call_inputs, &mut ins, &mut tr)?;
    let pre_prog = vec_of("pre_prog", prog_len, &mut call_inputs, &mut ins, &mut tr)?;
    let pre_ctx_v = vec_of("pre_ctx", CTX_LANES, &mut call_inputs, &mut ins, &mut tr)?;
    let pre_ctx: [Operand; CTX_LANES] = pre_ctx_v
        .try_into()
        .map_err(|_| C6ReturnCallConsumerError::Backend("pre_ctx".into()))?;
    let pre_image = vec_of(
        "pre_image",
        geometry.image_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let pre_data = vec_of(
        "pre_data",
        geometry.data_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let pre_helper = vec_of(
        "pre_helper",
        geometry.helper_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let post_meta = vec_of("post_meta", meta_len, &mut call_inputs, &mut ins, &mut tr)?;
    let post_prog = vec_of("post_prog", prog_len, &mut call_inputs, &mut ins, &mut tr)?;
    let post_ctx_v = vec_of("post_ctx", CTX_LANES, &mut call_inputs, &mut ins, &mut tr)?;
    let post_ctx: [Operand; CTX_LANES] = post_ctx_v
        .try_into()
        .map_err(|_| C6ReturnCallConsumerError::Backend("post_ctx".into()))?;
    let post_image = vec_of(
        "post_image",
        geometry.image_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let post_data = vec_of(
        "post_data",
        geometry.data_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let post_helper = vec_of(
        "post_helper",
        geometry.helper_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let block = emit_c6_return_call_step_block(
        &C6ReturnCallStepInputs {
            enable,
            selector_is_valid: valid,
            host_schema: schema,
            carrier_version: carrier,
            geometry,
            meta_bounds_frames_offset,
            source_params: source_params.clone(),
            pre_meta,
            pre_prog,
            pre_ctx,
            pre_image,
            pre_data,
            pre_helper,
            post_meta,
            post_prog,
            post_ctx,
            post_image: post_image.clone(),
            post_data: post_data.clone(),
            post_helper: post_helper.clone(),
        },
        scope,
    )?;
    ins.extend(block.instructions);
    let statement_lane_count = tr.len();
    ins.push(Instruction::Impact {
        guard: Operand::Immediate(Fr::from(1u64)),
        inputs: tr,
    });
    let outputs = match mode {
        C6PushStatementBindingMode::LocalPublic => Vec::new(),
        C6PushStatementBindingMode::LedgerCall => {
            let mut vals = Vec::with_capacity(geometry.state_total_lanes);
            vals.extend(post_image);
            vals.extend(post_data);
            vals.extend(post_helper);
            ins.push(Instruction::Output { vals });
            vec![IrType::Native; geometry.state_total_lanes]
        }
    };
    let ir0 = IrSource {
        version: IrMinorVersion::V0,
        inputs: call_inputs,
        outputs,
        do_communications_commitment: matches!(mode, C6PushStatementBindingMode::LedgerCall),
        instructions: Arc::new(ins),
    };
    let mut disk = serde_json::to_value(&ir0).map_err(be)?;
    disk["version"] = json!({ "major": 3, "minor": 0 });
    let serialized_zkir = serde_json::to_string(&disk).map_err(be)?;
    let ir = IrSource::load(serialized_zkir.as_bytes()).map_err(be)?;
    // Instruction count is not degree k / performance qualification.
    Ok(C6ReturnCallStepCircuit {
        mode,
        geometry,
        meta_len,
        prog_len,
        meta_bounds_frames_offset,
        source_params,
        statement_lane_count,
        ir,
        serialized_zkir,
    })
}

/// `ProofPreimage::check` preprocessing only; not synthesis, proving,
/// MockProver, or native execution. Semantic rejection here is the
/// SDK preprocessing category. `key_location` is diagnostic only:
/// not a native verifier issuer, circuit authentication, or ledger
/// deployment.
pub fn check_c6_return_call_step_circuit(
    circuit: &C6ReturnCallStepCircuit,
    witness: &C6ReturnCallStepWitness,
) -> Result<Vec<Option<usize>>, C6ReturnCallConsumerError> {
    if witness.source_params != circuit.source_params {
        return Err(C6ReturnCallConsumerError::Backend(
            "witness retained family does not match circuit".into(),
        ));
    }
    if witness.host_prefix.meta.len() != circuit.meta_len {
        return Err(C6ReturnCallConsumerError::LengthMismatch {
            section: "meta",
            expected: circuit.meta_len,
            actual: witness.host_prefix.meta.len(),
        });
    }
    if witness.host_prefix.prog.len() != circuit.prog_len {
        return Err(C6ReturnCallConsumerError::LengthMismatch {
            section: "prog",
            expected: circuit.prog_len,
            actual: witness.host_prefix.prog.len(),
        });
    }
    if witness.host_prefix.meta_bounds_frames_offset != circuit.meta_bounds_frames_offset {
        return Err(C6ReturnCallConsumerError::Backend(
            "meta bounds frames offset mismatch".into(),
        ));
    }
    let statement = witness.to_statement_frs();
    if statement.len() != circuit.statement_lane_count {
        return Err(C6ReturnCallConsumerError::LengthMismatch {
            section: "statement",
            expected: circuit.statement_lane_count,
            actual: statement.len(),
        });
    }
    let preimage = match circuit.mode {
        C6PushStatementBindingMode::LocalPublic => ProofPreimage {
            inputs: Vec::new(),
            private_transcript: Vec::new(),
            public_transcript_inputs: statement.clone(),
            public_transcript_outputs: statement,
            binding_input: Fr::from(0u64),
            communications_commitment: None,
            key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
        },
        C6PushStatementBindingMode::LedgerCall => {
            // Recomputed from the CURRENT statement and post flat lanes
            // on every call, so a mutated statement in a raw test is
            // checked against its own recomputed commitment, never a
            // stale commitment carried from an earlier witness.
            let randomness = Fr::from(0u64);
            let mut comm = statement.clone();
            comm.extend(witness.post_flat().iter().map(|&l| Fr::from(l)));
            ProofPreimage {
                inputs: statement.clone(),
                private_transcript: Vec::new(),
                public_transcript_inputs: statement,
                public_transcript_outputs: Vec::new(),
                binding_input: Fr::from(0u64),
                communications_commitment: Some((transient_commit(&comm, randomness), randomness)),
                key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
            }
        }
    };
    preimage.check(&circuit.ir).map_err(|e| {
        C6ReturnCallConsumerError::Backend(format!("SDK ProofPreimage::check failed: {e}"))
    })
}

#[cfg(test)]
#[path = "c6_return_call_tests.rs"]
mod tests;
