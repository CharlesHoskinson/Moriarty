//! The actual pinned native caller, bounded at unsigned/unproven preparation.
use crate::{Error, Result, codec, custody, layout, ops, profile::*};
use midnight_base_crypto::fab::AlignedValue;
use midnight_ledger::{
    construct::{ContractCallPrototype, PreTranscript, partition_transcripts},
    structure::{
        Intent, LedgerParameters, ProofPreimageMarker, ProofPreimageVersioned, Transaction, Utxo,
    },
};
use midnight_onchain_runtime::{
    context::{Effects, QueryContext},
    transcript::Transcript,
};
use midnight_storage::{arena::Sp, db::InMemoryDB};
use midnight_transient_crypto::{
    commitment::PedersenRandomness,
    curve::Fr,
    fab::AlignedValueExt,
    proofs::{KeyLocation, ProofPreimage},
    repr::FieldRepr,
};
use mori_core::{Balance, Execution, Program};
use serde::Serialize;
use std::{borrow::Cow, collections::BTreeMap};
#[derive(Debug, Clone, Serialize)]
pub struct Evidence {
    pub qualification: &'static str,
    pub relation: &'static str,
    pub proof_verification: &'static str,
    pub signature_verification: &'static str,
    pub ledger_apply: &'static str,
    pub consumption: &'static str,
    pub fees_paid: &'static str,
    pub settlement: &'static str,
    pub deployed_qualification: &'static str,
}
#[derive(Clone, Serialize)]
pub struct Readback {
    pub execution: Execution,
    pub prospective_balances: Vec<Balance>,
    pub effects_hex: String,
    #[serde(skip)]
    pub effects: Effects<InMemoryDB>,
    pub charged_successor_hex: String,
    pub semantic_successor_hex: String,
    pub native_events_hex: Vec<String>,
    pub measured_gas_hex: String,
    pub declared_gas_hex: String,
    pub retained_utxos_hex: Vec<String>,
    pub prospective_outputs_hex: Vec<String>,
    pub effect_outputs: Vec<custody::EffectOutput>,
    pub change_indices: Vec<u32>,
    pub requires_reconstruction_after_final_intent: bool,
    pub evidence: Evidence,
    pub preserved_frame: PreservedFrame,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<crate::relation::CheckedReadback>,
}
#[derive(Clone, Serialize)]
pub struct PreservedFrame {
    pub scope: &'static str,
    pub commitments: BTreeMap<String, String>,
}
fn preserved_frame(
    pre: &layout::AdmittedPredecessor,
    output: &OutputCodec,
) -> Result<PreservedFrame> {
    let s = &pre.ledger;
    Ok(PreservedFrame {
        scope: "unchanged-supplied-ledger-not-an-applied-successor",
        commitments: [
            ("complete_ledger", output.native_hash(s)?),
            ("network", output.native_hash(&s.network_id)?),
            ("parameters", output.native_hash(&*s.parameters)?),
            ("locked_pool", output.native_hash(&s.locked_pool)?),
            ("reserve_pool", output.native_hash(&s.reserve_pool)?),
            (
                "block_reward_pool",
                output.native_hash(&s.block_reward_pool)?,
            ),
            ("bridge_receiving", output.native_hash(&s.bridge_receiving)?),
            (
                "unclaimed_block_rewards",
                output.native_hash(&s.unclaimed_block_rewards)?,
            ),
            ("treasury", output.native_hash(&s.treasury)?),
            ("zswap", output.native_hash(&*s.zswap)?),
            ("contract_map", output.native_hash(&s.contract)?),
            ("utxo_state", output.native_hash(&*s.utxo)?),
            (
                "replay_protection",
                output.native_hash(&*s.replay_protection)?,
            ),
            ("dust", output.native_hash(&*s.dust)?),
            (
                "contract_operations",
                output.native_hash(&pre.contract.operations)?,
            ),
            (
                "contract_maintenance",
                output.native_hash(&pre.contract.maintenance_authority)?,
            ),
            (
                "contract_balance",
                output.native_hash(&pre.contract.balance)?,
            ),
            (
                "contract_charged_predecessor",
                output.native_hash(&pre.contract.data)?,
            ),
            (
                "contract_extensions",
                output.native_hash(&layout::root_maps(pre.contract.data.get_ref())?[2])?,
            ),
        ]
        .into_iter()
        .map(|(k, v)| (k.into(), v))
        .collect(),
    })
}
pub type RawIntent = Intent<(), ProofPreimageMarker, PedersenRandomness, InMemoryDB>;

/// Raw owner returned by the actual checked prepare path. Inspection projections
/// remain separate; SDK Intent::prove inserts Noops itself.
pub struct PreparedNativeCall {
    pub(crate) bundle: PreparedBundle,
    pub(crate) artifact: crate::relation::Artifact,
}
impl PreparedNativeCall {
    pub fn bundle(&self) -> &PreparedBundle {
        &self.bundle
    }
    pub fn raw_intent(&self) -> &RawIntent {
        &self.bundle.native.intent
    }
    pub fn artifact(&self) -> &crate::relation::Artifact {
        &self.artifact
    }
    /// Consume the owner for ordinary publication; this cannot construct a new
    /// typed proving owner from subsequently edited public bundle fields.
    pub fn into_bundle(self) -> PreparedBundle {
        self.bundle
    }
}
pub(crate) struct NativePreparation {
    pub intent: RawIntent,
    pub context: QueryContext<InMemoryDB>,
    pub lowered: ops::LoweredOps,
    pub params: LedgerParameters,
    pub gas_cap: midnight_base_crypto::cost_model::RunningCost,
    pub segment: u16,
    pub processed_transcript: Option<Transcript<InMemoryDB>>,
    pub output_bytes: u64,
}
pub struct PreparedBundle {
    pub(crate) native: NativePreparation,
    pub readback: Readback,
    pub leaves: BTreeMap<String, Vec<u8>>,
    pub marker: Vec<u8>,
}
#[derive(Serialize)]
struct CallPublic {
    address_hex: String,
    entry_point: String,
    operation_hex: String,
    input_hex: String,
    output_hex: String,
    communication_randomness_hex: String,
    binding_randomness_hex: String,
    communication_commitment_hex: String,
    key_location: String,
    guaranteed_transcript_hash: String,
    private_transcript_outputs: Vec<String>,
    preimage_inputs: Vec<String>,
    public_transcript_inputs: Vec<String>,
    public_transcript_outputs: Vec<String>,
    binding_input_hex: String,
}
fn failure(layer: &'static str, code: &'static str) -> Error {
    Error::new(layer, code)
}
pub fn validate_balance(
    balance: &BTreeMap<(midnight_coin_structure::coin::TokenType, u16), i128>,
) -> Result<()> {
    if balance.values().any(|n| *n != 0) {
        return Err(failure("native-balance", "nonzero-delta"));
    }
    Ok(())
}
pub fn validate_query(
    lowered: &ops::LoweredOps,
    actual: &midnight_onchain_runtime::context::QueryResults<
        midnight_onchain_vm::result_mode::ResultModeVerify,
        InMemoryDB,
    >,
) -> Result<()> {
    if *actual.context.state.get() != lowered.expected_root
        || actual.context.effects != lowered.expected_effects
        || actual.events != lowered.events
    {
        return Err(failure("full-readback", "direct-state-effects-events"));
    }
    Ok(())
}
/// Validates declarations which run_transcript itself does not inspect.
pub fn validate_transcript(
    expected: &Transcript<InMemoryDB>,
    candidate: &Transcript<InMemoryDB>,
    lowered: &ops::LoweredOps,
    actual: &midnight_onchain_runtime::context::QueryResults<
        midnight_onchain_vm::result_mode::ResultModeVerify,
        InMemoryDB,
    >,
    cap: &midnight_base_crypto::cost_model::RunningCost,
) -> Result<()> {
    if Vec::from(&candidate.program) != lowered.program {
        return Err(failure("full-readback", "transcript-program"));
    }
    if candidate.version.as_deref() != Some(&Transcript::<InMemoryDB>::VERSION) {
        return Err(failure("full-readback", "transcript-version"));
    }
    if candidate.gas != expected.gas {
        return Err(failure("full-readback", "partition-gas-identity"));
    }
    if !candidate.gas.within_bounds(cap) || !actual.gas_cost.within_bounds(&candidate.gas) {
        return Err(failure("native-partition", "gas-ceiling"));
    }
    if candidate.effects != actual.context.effects || candidate.effects != lowered.expected_effects
    {
        return Err(failure("full-readback", "declared-effects"));
    }
    Ok(())
}
pub fn prepare_call(
    source: &str,
    p: &Program,
    d: &Deployment,
    predecessor_bytes: &[u8],
    parameter_bytes: &[u8],
    request: &CallRequest,
) -> Result<PreparedBundle> {
    prepare_call_inner(
        source,
        p,
        d,
        predecessor_bytes,
        parameter_bytes,
        request,
        None,
    )
}
/// Source-recompile and ordinary-check the actual native raw owner. This remains
/// a local unsigned keyless preparation and retains legacy installed-key refusal.
pub fn prepare_native_call(
    source: &str,
    p: &Program,
    d: &Deployment,
    predecessor_bytes: &[u8],
    parameter_bytes: &[u8],
    request: &CallRequest,
) -> Result<PreparedNativeCall> {
    let artifact = crate::relation::compile(source, p, d, &request.action)?;
    prepare_native_call_checked(
        source,
        p,
        d,
        predecessor_bytes,
        parameter_bytes,
        request,
        &artifact,
    )
}
/// Recompile and admit an externally supplied source artifact before owning the
/// actual raw intent. No mutable caller-built Artifact selects the trusted IR.
pub fn prepare_native_call_checked(
    source: &str,
    p: &Program,
    d: &Deployment,
    predecessor_bytes: &[u8],
    parameter_bytes: &[u8],
    request: &CallRequest,
    artifact: &crate::relation::Artifact,
) -> Result<PreparedNativeCall> {
    let expected = crate::relation::admit(
        source,
        p,
        d,
        &request.action,
        &artifact.ir_bytes()?,
        &artifact.manifest_bytes()?,
    )?;
    let bundle = prepare_call_inner(
        source,
        p,
        d,
        predecessor_bytes,
        parameter_bytes,
        request,
        Some(&expected),
    )?;
    Ok(PreparedNativeCall {
        bundle,
        artifact: expected,
    })
}
/// Checked ordinary native relation preparation; no key/proof/signature/apply.
pub fn prepare_call_checked(
    source: &str,
    p: &Program,
    d: &Deployment,
    predecessor_bytes: &[u8],
    parameter_bytes: &[u8],
    request: &CallRequest,
    artifact: &crate::relation::Artifact,
) -> Result<PreparedBundle> {
    prepare_native_call_checked(
        source,
        p,
        d,
        predecessor_bytes,
        parameter_bytes,
        request,
        artifact,
    )
    .map(PreparedNativeCall::into_bundle)
}
fn prepare_call_inner(
    source: &str,
    p: &Program,
    d: &Deployment,
    predecessor_bytes: &[u8],
    parameter_bytes: &[u8],
    request: &CallRequest,
    relation: Option<&crate::relation::Artifact>,
) -> Result<PreparedBundle> {
    if source.len() > codec::INPUT_BYTES
        || codec::hash(source.as_bytes()) != d.source_hash
        || mori_core::parse(source).map_err(|_| failure("source-profile", "source-parse"))? != *p
    {
        return Err(failure("identity", "raw-source"));
    }
    let abi = layout::admit_source(p, d)?;
    let state: layout::NativeState = codec::decode_native(predecessor_bytes)?;
    let params: LedgerParameters = codec::decode_native(parameter_bytes)?;
    if codec::hash(parameter_bytes) != d.parameters_hash || *state.parameters != params {
        return Err(failure("identity", "parameters"));
    }
    if !p.actions.iter().any(|a| a.name == request.action) {
        return Err(failure("identity", "selected-action"));
    }
    let pre = layout::decode_predecessor(p, &abi, d, state, request)?;
    prepare_admitted(
        p,
        &StaticBindings::legacy(d),
        &FrameBindings::legacy(d),
        pre,
        params,
        request,
        relation,
        PROFILE,
        &codec::object_hash(d)?,
        codec::OUTPUT_BYTES as u64,
        codec::OUTPUT_BYTES as u64,
    )
}
struct OutputCodec(u64);
impl OutputCodec {
    fn encode<T: midnight_serialize::Serializable + midnight_serialize::Tagged>(
        &self,
        value: &T,
    ) -> Result<Vec<u8>> {
        if self.0 == codec::OUTPUT_BYTES as u64 {
            return codec::encode(value);
        }
        crate::call_artifacts::encode_bounded(value, self.0)
    }
    fn native_hex<T: midnight_serialize::Serializable + midnight_serialize::Tagged>(
        &self,
        value: &T,
    ) -> Result<String> {
        Ok(codec::hex(&self.encode(value)?))
    }
    fn native_hash<T: midnight_serialize::Serializable + midnight_serialize::Tagged>(
        &self,
        value: &T,
    ) -> Result<String> {
        Ok(codec::hash(&self.encode(value)?))
    }
    fn json_bytes(&self, value: &impl Serialize) -> Result<Vec<u8>> {
        if self.0 == codec::OUTPUT_BYTES as u64 {
            let bytes =
                serde_json::to_vec_pretty(value).map_err(|_| failure("input", "public-json"))?;
            if bytes.len() > codec::OUTPUT_BYTES {
                return Err(failure("input", "artifact-size"));
            }
            return Ok(bytes);
        }
        crate::call_artifacts::json_bytes(value, self.0)
    }
}
fn prepare_admitted(
    p: &Program,
    d: &StaticBindings,
    f: &FrameBindings,
    pre: layout::AdmittedPredecessor,
    params: LedgerParameters,
    request: &CallRequest,
    relation: Option<&crate::relation::Artifact>,
    profile: &'static str,
    descriptor_hash: &str,
    output_bytes: u64,
    aggregate_bytes: u64,
) -> Result<PreparedBundle> {
    prepare_admitted_signed(
        p,
        d,
        f,
        pre,
        params,
        request,
        relation,
        profile,
        descriptor_hash,
        output_bytes,
        aggregate_bytes,
        None,
        None,
    )
}
pub(crate) fn prepare_admitted_signed(
    p: &Program,
    d: &StaticBindings,
    f: &FrameBindings,
    pre: layout::AdmittedPredecessor,
    params: LedgerParameters,
    request: &CallRequest,
    relation: Option<&crate::relation::Artifact>,
    profile: &'static str,
    descriptor_hash: &str,
    output_bytes: u64,
    aggregate_bytes: u64,
    signed: Option<&crate::signed::SignedStaticBindings>,
    full_input: Option<AlignedValue>,
) -> Result<PreparedBundle> {
    let encoder = OutputCodec(output_bytes);
    let abi = layout::admit_bindings(p, d)?;
    let execution = mori_core::simulate(p, &request.action, &pre.scenario)
        .map_err(|_| failure("semantic", "core-refusal"))?;
    if let Some(bounds) = &d.economic_bounds {
        mori_core::intent::check_effects(
            &bounds.owner,
            &bounds.limits,
            &execution.effects,
            &execution.post.liabilities,
        )
        .map_err(|_| failure("semantic", "economic-bounds"))?;
    }
    let lowered = if let Some(signed) = signed {
        ops::lower_signed_ops(p, &pre, &execution, signed)?
    } else {
        ops::lower_ops(p, &pre, &execution)?
    };
    let mut context = QueryContext::new(pre.contract.data.clone(), pre.address);
    context.call_context = pre.context.clone();
    let direct = context
        .query(
            &lowered.program,
            Some(pre.gas),
            &params.cost_model.runtime_cost_model,
        )
        .map_err(|_| failure("native-VM", "query-refusal"))?;
    validate_query(&lowered, &direct)?;
    if direct
        .gas_cost
        .bytes_written
        .saturating_sub(direct.gas_cost.bytes_deleted)
        > params.limits.block_limits.bytes_written
    {
        return Err(failure("native-VM", "net-written-bound"));
    }
    let offer = custody::prepare_offer(&pre, &execution)?;
    custody::validate_offer(&pre, &execution, &offer.offer)?;
    let action = p
        .actions
        .iter()
        .find(|a| a.name == request.action)
        .ok_or(failure("identity", "action"))?;
    let inputs = action
        .inputs
        .iter()
        .map(|i| layout::value(&request.inputs[&i.name]))
        .collect::<Vec<_>>();
    let input = full_input.unwrap_or_else(|| AlignedValue::concat(inputs.iter()));
    let output = AlignedValue::from(());
    let comm_rand: Fr = codec::from_hex(&request.communication_randomness_hex)?;
    let binding_rand: PedersenRandomness = codec::from_hex(&request.binding_randomness_hex)?;
    let commitment = midnight_onchain_runtime::communication_commitment(
        input.clone(),
        output.clone(),
        comm_rand,
    );
    let pairs = partition_transcripts(
        &[PreTranscript {
            context: context.clone(),
            program: lowered.program.clone(),
            comm_comm: Some(commitment),
        }],
        &params,
    )
    .map_err(|_| failure("native-partition", "partition-refusal"))?;
    let (guaranteed, fallible) = pairs
        .into_iter()
        .next()
        .ok_or(failure("native-partition", "missing-pair"))?;
    let transcript = guaranteed.ok_or(failure("unsupported-phase", "all-fallible"))?;
    if fallible.is_some() {
        return Err(failure("unsupported-phase", "fallible-transcript"));
    }
    let repeated = context
        .run_transcript(&transcript, &params.cost_model.runtime_cost_model)
        .map_err(|_| failure("native-VM", "transcript-refusal"))?;
    if repeated.context.state != direct.context.state
        || repeated.context.effects != direct.context.effects
        || repeated.events != direct.events
        || repeated.gas_cost != direct.gas_cost
    {
        return Err(failure("full-readback", "repeated-complete-result"));
    }
    validate_transcript(&transcript, &transcript, &lowered, &repeated, &pre.gas)?;
    let transcript_bytes = encoder.encode(&transcript)?;
    let decoded: Transcript<InMemoryDB> =
        crate::call_artifacts::decode_bounded(&transcript_bytes, output_bytes)?;
    if decoded != transcript {
        return Err(failure("full-readback", "decoded-transcript"));
    }
    let operation = pre
        .contract
        .operations
        .get(&request.action.as_bytes().into())
        .ok_or(failure("identity", "operation"))?;
    let prototype = ContractCallPrototype {
        address: pre.address,
        entry_point: request.action.as_bytes().into(),
        op: (*operation).clone(),
        guaranteed_public_transcript: Some(transcript.clone()),
        fallible_public_transcript: None,
        private_transcript_outputs: vec![],
        input: input.clone(),
        output: output.clone(),
        communication_commitment_rand: comm_rand,
        key_location: KeyLocation(Cow::Owned(request.action.clone())),
    };
    let ttl = codec::from_hex(&request.ttl_hex)?;
    let empty: Intent<(), ProofPreimageMarker, PedersenRandomness, InMemoryDB> = Intent {
        guaranteed_unshielded_offer: Some(Sp::new(offer.offer.clone())),
        fallible_unshielded_offer: None,
        actions: Default::default(),
        dust_actions: None,
        ttl,
        binding_commitment: binding_rand,
    };
    let intent = empty.add_call::<ProofPreimage>(prototype);
    let call = intent
        .calls()
        .next()
        .ok_or(failure("full-readback", "native-call"))?;
    if call.communication_commitment != commitment
        || call.guaranteed_transcript.as_deref() != Some(&transcript)
        || call.fallible_transcript.is_some()
    {
        return Err(failure("full-readback", "native-call-identity"));
    }
    let ProofPreimageVersioned::V2(preimage) = &call.proof else {
        return Err(failure("full-readback", "native-preimage-version"));
    };
    let public_inputs = lowered
        .program
        .iter()
        .flat_map(|op| op.field_vec())
        .collect::<Vec<_>>();
    let mut public_outputs = Vec::new();
    for op in &lowered.program {
        if let midnight_onchain_vm::ops::Op::Popeq { result, .. } = op {
            result.value_only_field_repr(&mut public_outputs);
        }
    }
    if !preimage.private_transcript.is_empty()
        || preimage.public_transcript_inputs != public_inputs
        || preimage.public_transcript_outputs != public_outputs
        || preimage.communications_commitment != Some((commitment, comm_rand))
    {
        return Err(failure("full-readback", "native-preimage"));
    }
    let mut relation_leaves = BTreeMap::new();
    let mut processed_expectation = None;
    let relation_readback = if let Some(artifact) = relation {
        let activity = crate::relation::check(artifact, preimage)?;
        let processed = crate::relation::process(
            artifact,
            &lowered.program,
            &activity,
            &params.cost_model.runtime_cost_model,
        )?;
        let mut processed_transcript = transcript.clone();
        processed_transcript.program = processed.program.clone().into();
        processed_transcript.gas.compute_time += processed.noop_cost;
        let actual = context
            .run_transcript(&processed_transcript, &params.cost_model.runtime_cost_model)
            .map_err(|_| failure("native-relation", "processed-VM-refusal"))?;
        validate_query(&lowered, &actual)?;
        if actual.context.state != direct.context.state {
            return Err(failure("native-relation", "processed-charged-state"));
        }
        let mut expected_gas = direct.gas_cost;
        expected_gas.compute_time += processed.noop_cost;
        if actual.gas_cost != expected_gas
            || !processed_transcript.gas.within_bounds(&pre.gas)
            || !actual.gas_cost.within_bounds(&processed_transcript.gas)
            || processed_transcript.version.as_deref() != Some(&Transcript::<InMemoryDB>::VERSION)
        {
            return Err(failure("native-relation", "processed-gas-or-version"));
        }
        let mut processed_call = call.clone();
        processed_call.guaranteed_transcript = Some(Sp::new(processed_transcript.clone()));
        let parent: midnight_transient_crypto::commitment::Pedersen =
            intent.binding_commitment.into();
        let pi = processed_call.public_inputs(parent);
        let mut expected_pi = vec![pi[0], commitment];
        expected_pi.extend(processed.operation_fields);
        if expected_pi != pi {
            return Err(failure("native-relation", "processed-public-projection"));
        }
        let mut bound = (**preimage).clone();
        bound.binding_input = pi[0];
        if crate::relation::check(artifact, &bound)? != activity {
            return Err(failure("native-relation", "processed-binding-check"));
        }
        processed_call.proof = ProofPreimageVersioned::V2(std::sync::Arc::new(bound.clone()));
        processed_expectation = Some(processed_transcript.clone());
        relation_leaves.insert("raw-intent.bin".into(), encoder.encode(&intent)?);
        relation_leaves.insert("raw-call.bin".into(), encoder.encode(&call)?);
        relation_leaves.insert("raw-preimage.bin".into(), encoder.encode(&**preimage)?);
        relation_leaves.insert("processed-preimage.bin".into(), encoder.encode(&bound)?);
        relation_leaves.insert(
            "processed-call.bin".into(),
            encoder.encode(&processed_call)?,
        );
        relation_leaves.insert(
            "processed-transcript.bin".into(),
            encoder.encode(&processed_transcript)?,
        );
        relation_leaves.insert("relation-manifest.json".into(), artifact.manifest_bytes()?);
        relation_leaves.insert("relation.zkir.bin".into(), artifact.ir_bytes()?);
        Some(crate::relation::CheckedReadback {
            scope: "ordinary-sdk-ir-checked-projection-unproven-provisional-binding",
            processed_call_use: "inspection-only; SDK Intent::prove consumes the raw call and inserts Noops itself",
            ir_hash: artifact.manifest.ir_hash.clone(),
            activity,
            raw_op_count: lowered.program.len(),
            processed_op_count: processed.program.len(),
            noop_count: processed.noop_count,
            inactive_fields: processed.inactive_fields,
            noop_cost_hex: encoder.native_hex(&processed.noop_cost)?,
            processed_measured_gas_hex: encoder.native_hex(&actual.gas_cost)?,
            processed_declared_gas_hex: encoder.native_hex(&processed_transcript.gas)?,
            parent_binding_commitment_hex: encoder.native_hex(&parent)?,
            public_inputs: pi.iter().map(|f| codec::hex(&f.as_le_bytes())).collect(),
        })
    } else {
        None
    };
    let erased = intent.erase_proofs().erase_signatures();
    let _deferred = offer
        .offer
        .clone()
        .well_formed(1, &erased)
        .map_err(|_| failure("native-offer", "native-shape"))?;
    // Deliberately leave the returned signature closure unexecuted.
    let tx = Transaction::from_intents(
        d.network.clone(),
        [(1u16, intent.clone())].into_iter().collect(),
    );
    let balance = tx
        .balance(None)
        .map_err(|_| failure("native-balance", "conversion-or-intermediate-overflow"))?;
    validate_balance(&balance)?;
    let intent_hash = erased.intent_hash(1);
    let projected = offer
        .offer
        .outputs
        .iter()
        .enumerate()
        .map(|(i, o)| Utxo {
            value: o.value,
            owner: o.owner,
            type_: o.type_,
            intent_hash,
            output_no: i as u32,
        })
        .collect::<Vec<_>>();
    let public = CallPublic {
        address_hex: d.address_hex.clone(),
        entry_point: request.action.clone(),
        operation_hex: encoder.native_hex(&*operation)?,
        input_hex: encoder.native_hex(&input)?,
        output_hex: encoder.native_hex(&output)?,
        communication_randomness_hex: request.communication_randomness_hex.clone(),
        binding_randomness_hex: request.binding_randomness_hex.clone(),
        communication_commitment_hex: encoder.native_hex(&commitment)?,
        key_location: request.action.clone(),
        guaranteed_transcript_hash: codec::hash(&transcript_bytes),
        private_transcript_outputs: vec![],
        preimage_inputs: preimage
            .inputs
            .iter()
            .map(|f| codec::hex(&f.as_le_bytes()))
            .collect(),
        public_transcript_inputs: public_inputs
            .iter()
            .map(|f| codec::hex(&f.as_le_bytes()))
            .collect(),
        public_transcript_outputs: public_outputs
            .iter()
            .map(|f| codec::hex(&f.as_le_bytes()))
            .collect(),
        binding_input_hex: encoder.native_hex(&preimage.binding_input)?,
    };
    let frame = preserved_frame(&pre, &encoder)?;
    if frame.commitments["complete_ledger"] != f.predecessor_hash
        || frame.commitments["contract_operations"] != f.operations_hash
        || frame.commitments["contract_maintenance"] != f.maintenance_hash
        || frame.commitments["contract_balance"] != f.balance_hash
    {
        return Err(failure("full-readback", "preserved-native-frame"));
    }
    let readback = Readback {
        relation: relation_readback,
        preserved_frame: frame,
        execution,
        prospective_balances: offer.balances,
        effects_hex: encoder.native_hex(&direct.context.effects)?,
        effects: direct.context.effects.clone(),
        charged_successor_hex: encoder.native_hex(&direct.context.state)?,
        semantic_successor_hex: encoder.native_hex(&*direct.context.state.get())?,
        native_events_hex: direct
            .events
            .iter()
            .map(codec::native_hex)
            .collect::<Result<_>>()?,
        measured_gas_hex: encoder.native_hex(&direct.gas_cost)?,
        declared_gas_hex: encoder.native_hex(&transcript.gas)?,
        retained_utxos_hex: offer
            .retained
            .iter()
            .map(codec::native_hex)
            .collect::<Result<_>>()?,
        prospective_outputs_hex: projected
            .iter()
            .map(codec::native_hex)
            .collect::<Result<_>>()?,
        effect_outputs: offer.effect_outputs,
        change_indices: offer.change_indices,
        requires_reconstruction_after_final_intent: true,
        evidence: Evidence {
            qualification: if relation.is_some() {
                "local-relation-checked-unproven"
            } else {
                "local-unproven"
            },
            relation: if relation.is_some() {
                "source-static-native-ordinary-ir-check"
            } else {
                "unestablished"
            },
            proof_verification: "not-performed",
            signature_verification: "not-performed",
            ledger_apply: "not-performed",
            consumption: "not-performed",
            fees_paid: "not-performed",
            settlement: "not-performed",
            deployed_qualification: "not-performed",
        },
    };
    let mut leaves: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    leaves.append(&mut relation_leaves);
    if let Some(r) = &readback.relation {
        leaves.insert("relation-check.json".into(), encoder.json_bytes(r)?);
    }
    leaves.insert("abi.json".into(),encoder.json_bytes(&serde_json::json!({"layout":abi,"op_bindings":lowered.bindings,"claim_bindings":lowered.claim_bindings}))?);
    leaves.insert("ops.bin".into(), encoder.encode(&lowered.program)?);
    leaves.insert("guaranteed-transcript.bin".into(), transcript_bytes);
    leaves.insert(
        "successor-data.bin".into(),
        encoder.encode(&direct.context.state)?,
    );
    leaves.insert("offer.bin".into(), encoder.encode(&offer.offer)?);
    leaves.insert("call.json".into(), encoder.json_bytes(&public)?);
    leaves.insert("readback.json".into(), encoder.json_bytes(&readback)?);
    let descriptions: BTreeMap<_, _> = leaves
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                serde_json::json!({"sha256":codec::hash(v),"bytes":v.len()}),
            )
        })
        .collect();
    let marker = encoder.json_bytes(
        &serde_json::json!({"profile":profile,"abi":ABI,"source_hash":d.source_hash,"program_hash":d.program_hash,"native_state_hash":f.native_state_hash,"predecessor_hash":f.predecessor_hash,"parameters_hash":f.parameters_hash,"deployment_hash":descriptor_hash,"request_hash":codec::object_hash(request)?,"native_call_hash":codec::native_hash(call)?,"leaves":descriptions,"evidence":readback.evidence}),
    )?;
    let cap = NativeWorkLimits::cap(aggregate_bytes)?;
    leaves.values().try_fold(marker.len(), |n, bytes| {
        crate::call_artifacts::add_total(n, bytes.len(), cap)
    })?;
    Ok(PreparedBundle {
        native: NativePreparation {
            intent,
            context,
            lowered,
            params,
            gas_cap: pre.gas,
            segment: request.segment,
            processed_transcript: processed_expectation,
            output_bytes,
        },
        readback,
        leaves,
        marker,
    })
}

/// Installed owner retains the actual selected operation extracted only after
/// complete source-derived operation policy admission.
/// ```compile_fail
/// let owner = mori_ledger::prepare::PreparedInstalledCall { prepared: Default::default(), operation: Default::default(), public: Default::default(), context: Default::default() };
/// ```
pub struct PreparedInstalledCall {
    pub(crate) prepared: PreparedNativeCall,
    pub(crate) operation: midnight_onchain_state::state::ContractOperation,
    pub(crate) public: crate::local_deployment::PublicLocalDeployment,
    pub(crate) context: crate::call_artifacts::ProofContext,
}
impl PreparedInstalledCall {
    pub fn bundle(&self) -> &PreparedBundle {
        self.prepared.bundle()
    }
    pub fn proof_context(&self) -> crate::call_artifacts::ProofContext {
        self.context.clone()
    }
}
pub fn prepare_installed_native_call(
    source: &str,
    p: &Program,
    d: &InstalledDeployment,
    predecessor: &[u8],
    parameters: &[u8],
    request: &CallRequest,
    public: &crate::local_deployment::PublicLocalDeployment,
    limits: &NativeWorkLimits,
) -> Result<PreparedInstalledCall> {
    limits.validate()?;
    d.validate_profile()?;
    if source.len() > NativeWorkLimits::cap(limits.max_input_bytes)? {
        return Err(failure("input", "source-input-work-limit"));
    }
    crate::call_artifacts::check_json_input(d, limits.max_input_bytes)?;
    crate::call_artifacts::check_json_input(request, limits.max_input_bytes)?;
    if p.actions.len() > NativeWorkLimits::cap(limits.max_actions)? {
        return Err(failure("input", "action-work-limit"));
    }
    let artifact = public
        .admit_source(source, p, d, limits)?
        .into_iter()
        .find(|a| a.manifest.action == request.action)
        .ok_or(failure("identity", "selected-action"))?;
    let state: layout::NativeState =
        crate::call_artifacts::decode_bounded(predecessor, limits.max_input_bytes)?;
    let params: LedgerParameters =
        crate::call_artifacts::decode_bounded(parameters, limits.max_input_bytes)?;
    if codec::hash(parameters) != d.frame.parameters_hash || *state.parameters != params {
        return Err(failure("identity", "parameters"));
    }
    let abi = layout::admit_bindings(p, &d.bindings)?;
    let frame = layout::admit_frame(
        p,
        &abi,
        &d.bindings,
        &d.frame,
        state,
        layout::OperationPolicy::Installed { public, limits },
    )?;
    let pre = layout::attach_request(frame, &d.bindings, request)?;
    let operation = (*pre
        .contract
        .operations
        .get(&request.action.as_bytes().into())
        .ok_or(failure("identity", "operation"))?)
    .clone();
    let bundle = prepare_admitted(
        p,
        &d.bindings,
        &d.frame,
        pre,
        params,
        request,
        Some(&artifact),
        INSTALLED_PROFILE,
        &codec::object_hash(d)?,
        limits.max_leaf_bytes,
        limits.max_artifact_bytes,
    )?;
    admit_preparation_work(&bundle, limits)?;
    let context = crate::call_artifacts::ProofContext {
        signed: None,
        source_hash: d.bindings.source_hash.clone(),
        keyset_identity: d.keyset_identity.clone(),
        deployment_hash: codec::object_hash(d)?,
        predecessor_hash: codec::hash(predecessor),
        parameters_hash: codec::hash(parameters),
        request_hash: codec::object_hash(request)?,
        action: request.action.clone(),
    };
    Ok(PreparedInstalledCall {
        prepared: PreparedNativeCall { bundle, artifact },
        operation,
        public: public.clone(),
        context,
    })
}

pub(crate) fn admit_preparation_work(
    bundle: &PreparedBundle,
    limits: &NativeWorkLimits,
) -> Result<()> {
    crate::call_artifacts::admit_intent_work(&bundle.native.intent, limits)?;
    let mut processed = bundle.native.intent.clone();
    let mut call = processed
        .calls()
        .next()
        .ok_or(failure("identity", "operation"))?
        .clone();
    call.guaranteed_transcript = Some(Sp::new(
        bundle
            .native
            .processed_transcript
            .clone()
            .ok_or(failure("native-relation", "processed-transcript"))?,
    ));
    processed.actions = vec![midnight_ledger::structure::ContractAction::Call(Sp::new(
        call,
    ))]
    .into();
    crate::call_artifacts::admit_intent_work(&processed, limits)
}
#[cfg(test)]
#[path = "../tests/support/mod.rs"]
mod installed_test_support;
#[cfg(test)]
mod installed_work_tests {
    use super::*;
    #[test]
    fn actual_raw_processed_operations_and_sdk_pi_have_exact_ordinary_boundaries() {
        let f = super::installed_test_support::payment();
        let prepared = prepare_native_call(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters,
            &f.request,
        )
        .unwrap();
        let b = prepared.bundle();
        let readback = b.readback.relation.as_ref().unwrap();
        let max_ops = readback.raw_op_count.max(readback.processed_op_count) as u64;
        let mut limits = NativeWorkLimits {
            profile: "moriarty-native-call-work-limits/1".into(),
            max_degree: 19,
            max_actions: 1,
            max_srs_bytes: 1,
            max_prover_key_bytes: 1,
            max_verifier_key_bytes: 1,
            max_input_bytes: 1,
            max_leaf_bytes: 1,
            max_artifact_bytes: 1,
            max_operations: max_ops,
            max_public_inputs: readback.public_inputs.len() as u64,
        };
        admit_preparation_work(b, &limits).unwrap();
        limits.max_operations = max_ops - 1;
        assert!(admit_preparation_work(b, &limits).is_err());
        limits.max_operations = max_ops;
        limits.max_public_inputs -= 1;
        assert!(admit_preparation_work(b, &limits).is_err());
    }
}

pub(crate) fn final_installed_readback(
    prepared: &Readback,
    verified: &crate::proving::CallVerification,
) -> Readback {
    let mut final_readback = prepared.clone();
    final_readback.prospective_outputs_hex = verified.prospective_outputs_hex.clone();
    final_readback.charged_successor_hex = verified.charged_successor_hex.clone();
    final_readback.measured_gas_hex = verified.measured_gas_hex.clone();
    if let Some(projection) = &prepared.relation {
        final_readback.declared_gas_hex = projection.processed_declared_gas_hex.clone();
    }
    final_readback.requires_reconstruction_after_final_intent = false;
    final_readback.evidence.qualification = if verified.scope == crate::signed::RESULT_SCOPE {
        crate::signed::QUALIFICATION
    } else {
        INSTALLED_QUALIFICATION
    };
    if verified.scope == crate::signed::RESULT_SCOPE {
        final_readback.evidence.signature_verification = "strict-host-fens-fixed-compiled-policy";
    }
    final_readback.evidence.proof_verification = "ledger-real";
    final_readback
}
