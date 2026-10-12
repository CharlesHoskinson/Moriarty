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
#[derive(Serialize)]
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
#[derive(Serialize)]
pub struct PreservedFrame {
    pub scope: &'static str,
    pub commitments: BTreeMap<String, String>,
}
fn preserved_frame(pre: &layout::AdmittedPredecessor) -> Result<PreservedFrame> {
    let s = &pre.ledger;
    Ok(PreservedFrame {
        scope: "unchanged-supplied-ledger-not-an-applied-successor",
        commitments: [
            ("complete_ledger", codec::native_hash(s)?),
            ("network", codec::native_hash(&s.network_id)?),
            ("parameters", codec::native_hash(&*s.parameters)?),
            ("locked_pool", codec::native_hash(&s.locked_pool)?),
            ("reserve_pool", codec::native_hash(&s.reserve_pool)?),
            (
                "block_reward_pool",
                codec::native_hash(&s.block_reward_pool)?,
            ),
            ("bridge_receiving", codec::native_hash(&s.bridge_receiving)?),
            (
                "unclaimed_block_rewards",
                codec::native_hash(&s.unclaimed_block_rewards)?,
            ),
            ("treasury", codec::native_hash(&s.treasury)?),
            ("zswap", codec::native_hash(&*s.zswap)?),
            ("contract_map", codec::native_hash(&s.contract)?),
            ("utxo_state", codec::native_hash(&*s.utxo)?),
            (
                "replay_protection",
                codec::native_hash(&*s.replay_protection)?,
            ),
            ("dust", codec::native_hash(&*s.dust)?),
            (
                "contract_operations",
                codec::native_hash(&pre.contract.operations)?,
            ),
            (
                "contract_maintenance",
                codec::native_hash(&pre.contract.maintenance_authority)?,
            ),
            (
                "contract_balance",
                codec::native_hash(&pre.contract.balance)?,
            ),
            (
                "contract_charged_predecessor",
                codec::native_hash(&pre.contract.data)?,
            ),
            (
                "contract_extensions",
                codec::native_hash(&layout::root_maps(pre.contract.data.get_ref())?[2])?,
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
    bundle: PreparedBundle,
    artifact: crate::relation::Artifact,
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
fn json_bytes<T: Serialize>(v: &T) -> Result<Vec<u8>> {
    let bytes = serde_json::to_vec_pretty(v).map_err(|_| failure("input", "public-json"))?;
    if bytes.len() > codec::OUTPUT_BYTES {
        return Err(failure("input", "artifact-size"));
    }
    Ok(bytes)
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
    let lowered = ops::lower_ops(p, &pre, &execution)?;
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
    let input = AlignedValue::concat(inputs.iter());
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
    let transcript_bytes = codec::encode(&transcript)?;
    let decoded: Transcript<InMemoryDB> = codec::decode_artifact(&transcript_bytes)?;
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
        relation_leaves.insert("raw-intent.bin".into(), codec::encode(&intent)?);
        relation_leaves.insert("raw-call.bin".into(), codec::encode(&call)?);
        relation_leaves.insert("raw-preimage.bin".into(), codec::encode(&**preimage)?);
        relation_leaves.insert("processed-preimage.bin".into(), codec::encode(&bound)?);
        relation_leaves.insert("processed-call.bin".into(), codec::encode(&processed_call)?);
        relation_leaves.insert(
            "processed-transcript.bin".into(),
            codec::encode(&processed_transcript)?,
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
            noop_cost_hex: codec::native_hex(&processed.noop_cost)?,
            processed_measured_gas_hex: codec::native_hex(&actual.gas_cost)?,
            processed_declared_gas_hex: codec::native_hex(&processed_transcript.gas)?,
            parent_binding_commitment_hex: codec::native_hex(&parent)?,
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
        operation_hex: codec::native_hex(&*operation)?,
        input_hex: codec::native_hex(&input)?,
        output_hex: codec::native_hex(&output)?,
        communication_randomness_hex: request.communication_randomness_hex.clone(),
        binding_randomness_hex: request.binding_randomness_hex.clone(),
        communication_commitment_hex: codec::native_hex(&commitment)?,
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
        binding_input_hex: codec::native_hex(&preimage.binding_input)?,
    };
    let frame = preserved_frame(&pre)?;
    if frame.commitments["complete_ledger"] != d.predecessor_hash
        || frame.commitments["contract_operations"] != d.operations_hash
        || frame.commitments["contract_maintenance"] != d.maintenance_hash
        || frame.commitments["contract_balance"] != d.balance_hash
    {
        return Err(failure("full-readback", "preserved-native-frame"));
    }
    let readback = Readback {
        relation: relation_readback,
        preserved_frame: frame,
        execution,
        prospective_balances: offer.balances,
        effects_hex: codec::native_hex(&direct.context.effects)?,
        effects: direct.context.effects.clone(),
        charged_successor_hex: codec::native_hex(&direct.context.state)?,
        semantic_successor_hex: codec::native_hex(&*direct.context.state.get())?,
        native_events_hex: direct
            .events
            .iter()
            .map(codec::native_hex)
            .collect::<Result<_>>()?,
        measured_gas_hex: codec::native_hex(&direct.gas_cost)?,
        declared_gas_hex: codec::native_hex(&transcript.gas)?,
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
        leaves.insert("relation-check.json".into(), json_bytes(r)?);
    }
    leaves.insert("abi.json".into(),json_bytes(&serde_json::json!({"layout":abi,"op_bindings":lowered.bindings,"claim_bindings":lowered.claim_bindings}))?);
    leaves.insert("ops.bin".into(), codec::encode(&lowered.program)?);
    leaves.insert("guaranteed-transcript.bin".into(), transcript_bytes);
    leaves.insert(
        "successor-data.bin".into(),
        codec::encode(&direct.context.state)?,
    );
    leaves.insert("offer.bin".into(), codec::encode(&offer.offer)?);
    leaves.insert("call.json".into(), json_bytes(&public)?);
    leaves.insert("readback.json".into(), json_bytes(&readback)?);
    let descriptions: BTreeMap<_, _> = leaves
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                serde_json::json!({"sha256":codec::hash(v),"bytes":v.len()}),
            )
        })
        .collect();
    let marker = json_bytes(
        &serde_json::json!({"profile":PROFILE,"abi":ABI,"source_hash":d.source_hash,"program_hash":d.program_hash,"native_state_hash":d.native_state_hash,"predecessor_hash":d.predecessor_hash,"parameters_hash":d.parameters_hash,"deployment_hash":codec::object_hash(d)?,"request_hash":codec::object_hash(request)?,"native_call_hash":codec::native_hash(call)?,"leaves":descriptions,"evidence":readback.evidence}),
    )?;
    if leaves.values().map(Vec::len).sum::<usize>() + marker.len() > codec::OUTPUT_BYTES {
        return Err(failure("input", "artifact-size"));
    }
    Ok(PreparedBundle {
        native: NativePreparation {
            intent,
            context,
            lowered,
            params,
            gas_cap: pre.gas,
            segment: request.segment,
            processed_transcript: processed_expectation,
        },
        readback,
        leaves,
        marker,
    })
}
