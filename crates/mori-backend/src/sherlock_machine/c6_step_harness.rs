//! Shared LocalPublic witness/builder/checker for kit-based C6 step fragments.
//!
//! The statement is the same full pre+post carrier statement as the existing
//! C6 consumers (`c6_return_left_consumer`): enable, host schema, carrier
//! version, then pre META/PROG/CTX/IMAGE/DATA/HELPER and post
//! META/PROG/CTX/IMAGE/DATA/HELPER lanes. The builder serializes and reloads the
//! ZKIR through `IrSource::load`; the checker is the SDK's nonproving
//! `ProofPreimage::check` (not synthesis, keygen, proving or verification).
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
use serde_json::json;

use super::c6_push_step::{
    C6_PADDED_CARRIER_VERSION, C6PaddedCarrierGeometry, C6PushFamilyParams, C6PushHostPrefixLanes,
};
use super::c6_step_kit::C6StepInputs;

fn bk<E: std::fmt::Display>(e: E) -> String {
    e.to_string()
}

/// One step's pre and post carriers, encoded independently from two host
/// snapshots. Lanes are public so tests can mutate a witness.
#[derive(Clone, Debug, PartialEq)]
pub struct C6StepWitness {
    pub enable: bool,
    pub source_params: C6PushFamilyParams,
    pub host_prefix: C6PushHostPrefixLanes,
    pub pre_image: Vec<u128>,
    pub pre_data: Vec<u128>,
    pub pre_helper: Vec<u128>,
    pub post_image: Vec<u128>,
    pub post_data: Vec<u128>,
    pub post_helper: Vec<u128>,
}

impl C6StepWitness {
    pub fn to_statement_frs(&self) -> Vec<Fr> {
        let mut out = vec![
            Fr::from(self.enable),
            Fr::from(C6_HOST_SCHEMA),
            Fr::from(C6_PADDED_CARRIER_VERSION),
        ];
        let prefix = || {
            self.host_prefix
                .meta
                .iter()
                .chain(&self.host_prefix.prog)
                .chain(self.host_prefix.ctx.iter())
        };
        for l in prefix()
            .chain(&self.pre_image)
            .chain(&self.pre_data)
            .chain(&self.pre_helper)
        {
            out.push(Fr::from(*l));
        }
        for l in prefix()
            .chain(&self.post_image)
            .chain(&self.post_data)
            .chain(&self.post_helper)
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
) -> Result<Sections, String> {
    let padded = super::c6_push_step::encode_owned_state_to_padded(
        &C6OwnedState::from_image(state),
        caps,
        C6StateLimits {
            max_total_lanes: limits.max_state_lanes,
        },
    )
    .map_err(bk)?;
    let flat = padded.to_flat_lanes();
    let i = geom.image_total_lanes;
    let d = i + geom.data_total_lanes;
    Ok((flat[..i].to_vec(), flat[i..d].to_vec(), flat[d..].to_vec()))
}

/// Independent pre/post encoding; no rule, selection or effect is supplied.
pub fn step_witness_from_host_pair(
    enable: bool,
    pre: &HostSnapshot,
    post: &HostSnapshot,
    cfg: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<C6StepWitness, String> {
    let params = C6PushFamilyParams::new(cfg.clone(), pre.admission.bounds, *limits).map_err(bk)?;
    let geom = params.geometry();
    let caps = params.caps();
    let pp = C6PushHostPrefixLanes::from_host_snapshot(pre, cfg, limits).map_err(bk)?;
    let qp = C6PushHostPrefixLanes::from_host_snapshot(post, cfg, limits).map_err(bk)?;
    if pp.meta != qp.meta || pp.prog != qp.prog || pp.ctx != qp.ctx {
        return Err("host prefix differs between pre and post".into());
    }
    let (pre_image, pre_data, pre_helper) = split_state(&pre.image, caps, *limits, geom)?;
    let (post_image, post_data, post_helper) = split_state(&post.image, caps, *limits, geom)?;
    Ok(C6StepWitness {
        enable,
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

pub struct C6StepCircuit {
    key: String,
    meta_len: usize,
    prog_len: usize,
    meta_bounds_frames_offset: usize,
    source_params: C6PushFamilyParams,
    statement_lane_count: usize,
    ir: IrSource,
}

impl C6StepCircuit {
    pub fn key(&self) -> &str {
        &self.key
    }
    pub fn statement_lane_count(&self) -> usize {
        self.statement_lane_count
    }
}

/// Allocates the LocalPublic statement, runs `emit` over it and reloads the
/// serialized ZKIR through the SDK loader.
pub(super) fn build_step_circuit(
    key: String,
    source_params: C6PushFamilyParams,
    meta_len: usize,
    prog_len: usize,
    meta_bounds_frames_offset: usize,
    scope: &str,
    emit: impl FnOnce(&C6StepInputs) -> Result<Vec<Instruction>, String>,
) -> Result<C6StepCircuit, String> {
    if scope.trim().is_empty() || scope.contains('%') {
        return Err("invalid scope".into());
    }
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
    let host_schema = alloc("host_schema", 0, &mut ins);
    let carrier_version = alloc("carrier_version", 0, &mut ins);
    let mut vec_of = |label: &str, n: usize, ins: &mut Vec<Instruction>| {
        (0..n).map(|i| alloc(label, i, ins)).collect::<Vec<_>>()
    };
    let pre_meta = vec_of("pre_meta", meta_len, &mut ins);
    let pre_prog = vec_of("pre_prog", prog_len, &mut ins);
    let pre_ctx: [Operand; CTX_LANES] = vec_of("pre_ctx", CTX_LANES, &mut ins)
        .try_into()
        .map_err(|_| "pre_ctx".to_string())?;
    let pre_image = vec_of("pre_image", geometry.image_total_lanes, &mut ins);
    let pre_data = vec_of("pre_data", geometry.data_total_lanes, &mut ins);
    let pre_helper = vec_of("pre_helper", geometry.helper_total_lanes, &mut ins);
    let post_meta = vec_of("post_meta", meta_len, &mut ins);
    let post_prog = vec_of("post_prog", prog_len, &mut ins);
    let post_ctx: [Operand; CTX_LANES] = vec_of("post_ctx", CTX_LANES, &mut ins)
        .try_into()
        .map_err(|_| "post_ctx".to_string())?;
    let post_image = vec_of("post_image", geometry.image_total_lanes, &mut ins);
    let post_data = vec_of("post_data", geometry.data_total_lanes, &mut ins);
    let post_helper = vec_of("post_helper", geometry.helper_total_lanes, &mut ins);
    let body = emit(&C6StepInputs {
        enable,
        host_schema,
        carrier_version,
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
        post_image,
        post_data,
        post_helper,
    })?;
    ins.extend(body);
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
    let mut disk = serde_json::to_value(&ir0).map_err(bk)?;
    disk["version"] = json!({ "major": 3, "minor": 0 });
    let serialized = serde_json::to_string(&disk).map_err(bk)?;
    let ir = IrSource::load(serialized.as_bytes()).map_err(bk)?;
    Ok(C6StepCircuit {
        key,
        meta_len,
        prog_len,
        meta_bounds_frames_offset,
        source_params,
        statement_lane_count,
        ir,
    })
}

/// `ProofPreimage::check` preprocessing only (nonproving).
pub(super) fn check_step_circuit(
    circuit: &C6StepCircuit,
    witness: &C6StepWitness,
) -> Result<Vec<Option<usize>>, String> {
    if witness.source_params != circuit.source_params {
        return Err("witness retained family does not match circuit".into());
    }
    if witness.host_prefix.meta.len() != circuit.meta_len
        || witness.host_prefix.prog.len() != circuit.prog_len
        || witness.host_prefix.meta_bounds_frames_offset != circuit.meta_bounds_frames_offset
    {
        return Err("host prefix shape does not match circuit".into());
    }
    let statement = witness.to_statement_frs();
    if statement.len() != circuit.statement_lane_count {
        return Err(format!(
            "statement length: expected {}, got {}",
            circuit.statement_lane_count,
            statement.len()
        ));
    }
    let preimage = ProofPreimage {
        inputs: Vec::new(),
        private_transcript: Vec::new(),
        public_transcript_inputs: statement.clone(),
        public_transcript_outputs: statement,
        binding_input: Fr::from(0u64),
        communications_commitment: None,
        key_location: KeyLocation(circuit.key.clone().into()),
    };
    preimage
        .check(&circuit.ir)
        .map_err(|e| format!("SDK ProofPreimage::check failed: {e}"))
}
