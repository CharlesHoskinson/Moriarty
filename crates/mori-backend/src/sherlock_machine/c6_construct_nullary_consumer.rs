//! Full-carrier nullary-constructor producer and LocalPublic/LedgerCall binding.
//! Host-pair encoding does not filter semantic wrong posts. Owner-record input
//! checks full snapshot/event consistency but cannot authenticate provenance.
//! Both modes bind enable/schema/version then complete pre and post carriers;
//! LedgerCall returns actual post state and recomputes its current commitment.
//! Source only: no SDK/native/circuit/proof or current-Core ABI result claimed.
//! Financial capture refusals, complete StepInv, program/state authentication,
//! genuine range and recursive verification obligations remain unchanged.

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
use mori_core::sherlock::helper_machine::complete_observation::{
    CompleteStepRecord, ExpressionSnapshot, HostSnapshot, ImageSnapshot,
};

use mori_core::sherlock::helper_machine::shared::{
    AllocDeltasC, BoundaryKind, ControlC, EventKind, LocatorC, OutcomeC, OutcomeTagC, RefC,
    RuleIdC, SiteTag, StepDisposition,
};
use serde_json::json;
use thiserror::Error;

use super::c6_construct_nullary_step::{
    C6ConstructNullaryStepError, C6ConstructNullaryStepInputs, emit_c6_construct_nullary_step_block,
};
use super::c6_push_step::{
    C6_PADDED_CARRIER_VERSION, C6PaddedCarrierGeometry, C6PushFamilyParams, C6PushHostPrefixLanes,
    C6PushStatementBindingMode, C6PushStepError,
};

const CHECK_KEY_LOCATION: &str = "sherlock_c6_construct_nullary_step_check";

#[derive(Debug, Error)]
pub enum C6ConstructNullaryConsumerError {
    #[error("owner record is not an Applied XAdtEvalConstructNullary event")]
    UnexpectedOwnerStep,
    #[error("owner event/snapshot mismatch in {field}")]
    EventMismatch { field: &'static str },
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
    #[error("C6 ConstructNullary emitter error: {0}")]
    ConstructNullary(#[from] C6ConstructNullaryStepError),
    #[error("composed canonical emitter or SDK backend error: {0}")]
    Backend(String),
}

fn be<E: std::fmt::Display>(e: E) -> C6ConstructNullaryConsumerError {
    C6ConstructNullaryConsumerError::Backend(e.to_string())
}

#[derive(Clone, Debug, PartialEq)]
pub struct C6ConstructNullaryStepWitness {
    enable: bool,
    source_params: C6PushFamilyParams,
    host_prefix: C6PushHostPrefixLanes,
    pre_image: Vec<u128>,
    pre_data: Vec<u128>,
    pre_helper: Vec<u128>,
    post_image: Vec<u128>,
    post_data: Vec<u128>,
    post_helper: Vec<u128>,
}

impl C6ConstructNullaryStepWitness {
    pub fn source_params(&self) -> &C6PushFamilyParams {
        &self.source_params
    }
    pub fn enable(&self) -> bool {
        self.enable
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
}

fn split_state(
    state: &ImageSnapshot,
    caps: C6StateCaps,
    limits: C6HostLimits,
    geom: C6PaddedCarrierGeometry,
) -> Result<(Vec<u128>, Vec<u128>, Vec<u128>), C6ConstructNullaryConsumerError> {
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
/// An encodable wrong post reaches SDK constraints with its current commitment.
pub fn witness_from_host_pair(
    enable: bool,
    pre: &HostSnapshot,
    post: &HostSnapshot,
    cfg: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<C6ConstructNullaryStepWitness, C6ConstructNullaryConsumerError> {
    let params = C6PushFamilyParams::new(cfg.clone(), pre.admission.bounds, *limits)?;
    let geom = params.geometry();
    let caps = params.caps();
    let pp = C6PushHostPrefixLanes::from_host_snapshot(pre, cfg, limits)?;
    let qp = C6PushHostPrefixLanes::from_host_snapshot(post, cfg, limits)?;
    if pp.meta != qp.meta {
        return Err(C6ConstructNullaryConsumerError::HostPrefixMismatch { section: "meta" });
    }
    if pp.prog != qp.prog {
        return Err(C6ConstructNullaryConsumerError::HostPrefixMismatch { section: "prog" });
    }
    if pp.ctx != qp.ctx {
        return Err(C6ConstructNullaryConsumerError::HostPrefixMismatch { section: "ctx" });
    }
    let (pre_image, pre_data, pre_helper) = split_state(&pre.image, caps, *limits, geom)?;
    let (post_image, post_data, post_helper) = split_state(&post.image, caps, *limits, geom)?;
    Ok(C6ConstructNullaryStepWitness {
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

/// Internal host-record consistency, not cryptographic provenance. The
/// publicly constructible record is not an authenticated range statement.
/// Semantic wrong-post testing must use the separate host-pair/raw route.
pub fn witness_from_owner_record(
    record: &CompleteStepRecord,
    cfg: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<C6ConstructNullaryStepWitness, C6ConstructNullaryConsumerError> {
    type E = C6ConstructNullaryConsumerError;
    let mismatch = |field| E::EventMismatch { field };
    let before = &record.before.image;
    let after = &record.after.image;
    let event = &record.event;
    if event.rule_id != RuleIdC::XAdtEvalConstructNullary
        || event.disposition != StepDisposition::Applied
    {
        return Err(E::UnexpectedOwnerStep);
    }
    if before.outcome != OutcomeC::Running
        || before.remaining_work == 0
        || before.helper.is_some()
        || before.result.is_some()
        || !before.tasks.is_empty()
    {
        return Err(mismatch("pre dispatch"));
    }
    let ControlC::Eval { expr } = before.control else {
        return Err(mismatch("pre control"));
    };
    let Some(ExpressionSnapshot::Construct {
        fields,
        constant: Some(cid),
        ..
    }) = record.before.admission.view.expressions.get(expr)
    else {
        return Err(mismatch("pre expression"));
    };
    if fields.len != 0 {
        return Err(mismatch("pre fields.len"));
    }
    let mut expected = before.clone();
    expected.control = ControlC::Return {
        value: RefC::Const(*cid),
    };
    expected.remaining_work = before
        .remaining_work
        .checked_sub(1)
        .ok_or_else(|| mismatch("remaining_work"))?;
    expected.consumed_work = before
        .consumed_work
        .checked_add(1)
        .ok_or_else(|| mismatch("consumed_work"))?;
    expected.reference_cursor = before
        .reference_cursor
        .checked_add(1)
        .ok_or_else(|| mismatch("reference_cursor"))?;
    if *after != expected || record.after.admission != record.before.admission {
        return Err(mismatch("full after/admission"));
    }
    if *event.step_id.run_id != before.run_id
        || event.step_id.ordinal != before.consumed_work
        || event.event_kind != EventKind::LegacyComplete
        || event.site != Some(SiteTag::EvalConstruct)
        || event.helper_kind.is_some()
        || event.work_delta != 1
        || event.helper_work_delta != 0
        || event.helper_visits_delta != 0
        || event.reference_cursor_delta != 1
        || event.alloc_deltas
            != (AllocDeltasC {
                data_nodes: 0,
                data_fields: 0,
                environment_cells: 0,
            })
        || event.value_slot_writes != 0
        || event.logical_depth_before != before.frames.len()
        || event.logical_depth_after != after.frames.len()
        || event.task_cursor_before != 0
        || event.task_cursor_after != 0
        || event.helper_phase_before.is_some()
        || event.helper_phase_after.is_some()
        || event.helper_cursor_before.is_some()
        || event.helper_cursor_after.is_some()
        || event.result_tag.is_some()
        || event.origin_reference_cursor.is_some()
        || event.source_expr_id != before.control_origin
        || event.exhausted_kind.is_some()
        || event.outcome_tag != OutcomeTagC::Running
        || event.outcome_payload != OutcomeC::Running
        || event.boundary != BoundaryKind::None
    {
        return Err(mismatch("complete event"));
    }
    let view = &record.before.admission.view;
    let occurrence = view
        .occurrences
        .get(before.control_origin)
        .ok_or_else(|| mismatch("source occurrence"))?;
    let end = occurrence
        .path
        .start
        .checked_add(occurrence.path.len)
        .ok_or_else(|| mismatch("source occurrence"))?;
    let ast_path = view
        .occurrence_paths
        .get(occurrence.path.start..end)
        .ok_or_else(|| mismatch("source path"))?
        .to_vec();
    if event.source_occurrence
        != (LocatorC::Occurrence {
            owner_is_function: occurrence.owner_is_function,
            function: occurrence.function,
            ast_path,
        })
    {
        return Err(mismatch("source occurrence"));
    }
    witness_from_host_pair(true, &record.before, &record.after, cfg, limits)
}

pub struct C6ConstructNullaryStepCircuit {
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

impl C6ConstructNullaryStepCircuit {
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

fn make_typed(id: Identifier) -> Result<TypedIdentifier, C6ConstructNullaryConsumerError> {
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
) -> Result<Operand, C6ConstructNullaryConsumerError> {
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

pub fn build_c6_construct_nullary_step_circuit(
    source_params: C6PushFamilyParams,
    meta_len: usize,
    prog_len: usize,
    meta_bounds_frames_offset: usize,
    mode: C6PushStatementBindingMode,
    scope: &str,
) -> Result<C6ConstructNullaryStepCircuit, C6ConstructNullaryConsumerError> {
    if scope.trim().is_empty() {
        return Err(C6ConstructNullaryStepError::EmptyScope.into());
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
     -> Result<Operand, C6ConstructNullaryConsumerError> {
        alloc_lane(mode, scope, label, idx, ci, ii, tt)
    };
    let enable = alloc("enable", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let schema = alloc("host_schema", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let carrier = alloc("carrier_version", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let mut vec_of = |label: &str,
                      n: usize,
                      ci: &mut Vec<TypedIdentifier>,
                      ii: &mut Vec<Instruction>,
                      tt: &mut Vec<Operand>|
     -> Result<Vec<Operand>, C6ConstructNullaryConsumerError> {
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
        .map_err(|_| C6ConstructNullaryConsumerError::Backend("pre_ctx".into()))?;
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
        .map_err(|_| C6ConstructNullaryConsumerError::Backend("post_ctx".into()))?;
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
    let block = emit_c6_construct_nullary_step_block(
        &C6ConstructNullaryStepInputs {
            enable,
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
    Ok(C6ConstructNullaryStepCircuit {
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
pub fn check_c6_construct_nullary_step_circuit(
    circuit: &C6ConstructNullaryStepCircuit,
    witness: &C6ConstructNullaryStepWitness,
) -> Result<Vec<Option<usize>>, C6ConstructNullaryConsumerError> {
    if witness.source_params != circuit.source_params {
        return Err(C6ConstructNullaryConsumerError::Backend(
            "witness retained family does not match circuit".into(),
        ));
    }
    if witness.host_prefix.meta.len() != circuit.meta_len {
        return Err(C6ConstructNullaryConsumerError::LengthMismatch {
            section: "meta",
            expected: circuit.meta_len,
            actual: witness.host_prefix.meta.len(),
        });
    }
    if witness.host_prefix.prog.len() != circuit.prog_len {
        return Err(C6ConstructNullaryConsumerError::LengthMismatch {
            section: "prog",
            expected: circuit.prog_len,
            actual: witness.host_prefix.prog.len(),
        });
    }
    if witness.host_prefix.meta_bounds_frames_offset != circuit.meta_bounds_frames_offset {
        return Err(C6ConstructNullaryConsumerError::Backend(
            "meta bounds frames offset mismatch".into(),
        ));
    }
    check_c6_construct_nullary_statement(circuit, &witness.to_statement_frs())
}

/// Raw statement route for every supplied carrier lane, including malformed
/// semantic pre/post values. Shape is the only host refusal here. The current
/// statement's actual post-state tail is used for each fresh LedgerCall
/// communications commitment; a stale witness commitment cannot mask a mutant.
/// This invokes SDK preprocessing only when explicitly called.
pub fn check_c6_construct_nullary_statement(
    circuit: &C6ConstructNullaryStepCircuit,
    statement: &[Fr],
) -> Result<Vec<Option<usize>>, C6ConstructNullaryConsumerError> {
    if statement.len() != circuit.statement_lane_count {
        return Err(C6ConstructNullaryConsumerError::LengthMismatch {
            section: "statement",
            expected: circuit.statement_lane_count,
            actual: statement.len(),
        });
    }
    let post_start = statement
        .len()
        .checked_sub(circuit.geometry.state_total_lanes)
        .ok_or_else(|| {
            C6ConstructNullaryConsumerError::Backend("post-state statement tail".into())
        })?;
    let preimage = match circuit.mode {
        C6PushStatementBindingMode::LocalPublic => ProofPreimage {
            inputs: Vec::new(),
            private_transcript: Vec::new(),
            public_transcript_inputs: statement.to_vec(),
            public_transcript_outputs: statement.to_vec(),
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
            let mut comm = statement.to_vec();
            comm.extend_from_slice(&statement[post_start..]);
            ProofPreimage {
                inputs: statement.to_vec(),
                private_transcript: Vec::new(),
                public_transcript_inputs: statement.to_vec(),
                public_transcript_outputs: Vec::new(),
                binding_input: Fr::from(0u64),
                communications_commitment: Some((transient_commit(&comm, randomness), randomness)),
                key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
            }
        }
    };
    preimage.check(&circuit.ir).map_err(|e| {
        C6ConstructNullaryConsumerError::Backend(format!("SDK ProofPreimage::check failed: {e}"))
    })
}

#[cfg(test)]
#[path = "c6_construct_nullary_tests.rs"]
mod tests;
