//! Ledger-9 contract-operation route for the Sherlock composed two-step (A102 -> A80)
//! LiteralComplete relation (`LiteralCompleteLayout`, `LiteralCompleteValues`,
//! `LiteralCompleteExecutionRequest`). Endpoint `done` remains Step-mode Constant-space
//! non-canonical (`space == 0`, `activeData == 0`); `projectA`, `Opus F3`/`F5`, `F4`,
//! `StepInv` (`O1`..`O4`), `OBL.G4`, and 39-row dispatch remain open.
//! The installed operation VK is the custom decider: verification uses the
//! existing ProofMarker::proof_verify Real path, not a VerifyProof instruction,
//! ledger10, a Core ProofBundle or a financial call artifact. The statement is
//! bound by the call communication commitment. Its opening is disclosed off-chain and
//! no chain-side decoder exists. One Fr per u128 lane, a guaranteed empty-program
//! Transcript(v2.3) call carrier (after C8 verify.rs:1154 rejected the historical
//! (None, None) transcript hypothesis with CallHasEmptyTranscripts) and a
//! two-element call PI are enforced fail-closed (GAP-2 / C8).
//! Keygen, proving, ledger acceptance and stagenet readback are UNOBSERVED.
use crate::{Error, Result, call_artifacts, codec, parameters::PinnedCallParameters};
use crate::{prepare::RawIntent, proving::ProvenIntent};
use midnight_base_crypto::{
    cost_model::{CostDuration, RunningCost},
    fab::AlignedValue,
    time::Timestamp,
};
use midnight_coin_structure::contract::ContractAddress;
use midnight_ledger::construct::ContractCallPrototype;
use midnight_ledger::structure::{
    ContractCall, Intent, ProofKind, ProofMarker, ProofPreimageVersioned, ProofVersioned,
};
use midnight_ledger::verify::ProofVerificationMode;
use midnight_onchain_runtime::{
    context::{Effects, QueryContext},
    cost_model::CostModel,
    result_mode::ResultModeVerify,
    transcript::Transcript,
};
use midnight_onchain_state::state::{ChargedState, ContractOperation, IrBuf};
use midnight_storage::{arena::Sp, db::InMemoryDB, storage::Array as StorageArray};
use midnight_transient_crypto::commitment::{Pedersen, PedersenRandomness};
use midnight_transient_crypto::curve::Fr;
use midnight_transient_crypto::proofs::{
    KeyLocation, ParamsProver, ParamsProverProvider, Proof, ProofPreimage, ProvingError,
    ProvingKeyMaterial, ProvingProvider, Resolver, VerifierKey, Zkir,
};
use midnight_zkir_v3::IrSource;
use mori_backend::sherlock_machine::literal_bridge::CircuitMode;
use mori_backend::sherlock_machine::literal_complete_bridge::{
    COMPLETE_OPEN_OBLIGATIONS, COMPLETE_SCOPE, LiteralCompleteExecutionRequest,
    LiteralCompleteLayout, LiteralCompleteValues, LiteralEnvelopeValues,
};
use rand::{SeedableRng, rngs::StdRng};
use serde::Serialize;
use std::{borrow::Cow, io, path::Path, sync::Arc};

pub const LAYER: &str = "sherlock-complete-ledger";
pub const COMPOSED_LEDGER_CLOSURE_DOMAIN: &str =
    "moriarty-sherlock-a102-a80-composed-ledger-source-closure/1";
pub const COMPOSED_LEDGER_ENDPOINT_QUALIFICATION: &str = "done-is-step-mode-constant-space-noncanonical-stateblock-space0-activedata0-no-projecta-or-canonical-checkpoint-claim";
pub const COMPOSED_LEDGER_CLOSURE_QUALIFICATION: &str = "binds-proving-source-closure-plus-literal_complete_bridge-sherlock_literal_complete-and-sherlock_transaction_complete; excludes-cfg-test-modules-unlisted-mori-core-files-external-sdk-and-registry-sources-srs-and-unreconciled-post-baseline-shared-registration-edits; not-a-complete-build-or-artifact-transfer-certificate";
pub const COMPOSED_LEDGER_ADDITIONAL_OBLIGATIONS: [&str; 3] = [
    "composed-ledger consumer seam (sherlock_literal_complete / sherlock_transaction_complete) binds Step-mode Constant-space done only; no projectA unfolding, Opus F3 unfolded capacity admission, Opus F5 incremental overflow refusal, or canonical StateBlock correspondence is supplied",
    "caller-supplied ContractState, ChargedState, ContractAddress, CostModel, state/source freshness, chain association, indexer/provider envelopes, funding/fees, signatures, submission, inclusion, finality, and settlement readback remain unauthenticated external assumptions; transcript gas equality and proof bytes do not authenticate state identity",
    "declared_build_closure binds proving::source_closure() plus literal_complete_bridge.rs, sherlock_literal_complete.rs, and sherlock_transaction_complete.rs only; it excludes cfg(test) modules, unlisted workspace files, SDK/registry sources, SRS provenance, and later shared registration edits until Root final composition",
];

fn refusal(code: &'static str) -> Error {
    Error::new(LAYER, code)
}
#[derive(Clone)]
struct CallParams {
    degree: u8,
    params: ParamsProver,
}
impl ParamsProverProvider for CallParams {
    async fn get_params(&self, degree: u8) -> io::Result<ParamsProver> {
        if degree != self.degree {
            return Err(io::Error::other("unavailable immutable parameter degree"));
        }
        Ok(self.params.clone())
    }
}
/// Caller-selected native bounds; no default envelope or Root grant is implied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SherlockCompleteKeyLimits {
    pub max_degree: u8,
    pub max_srs_bytes: u64,
    pub max_prover_key_bytes: u64,
    pub max_verifier_key_bytes: u64,
    /// Explicit statement lane bound, admitted before emission, k or SRS work.
    pub max_statement_lanes: usize,
}
struct KeysInner {
    layout: LiteralCompleteLayout,
    ir: IrSource,
    ir_bytes: Vec<u8>,
    vk: Vec<u8>,
    material: Arc<ProvingKeyMaterial>,
    params: CallParams,
    entry_point: String,
}
/// Built only by source-recompiled SDK keygen; imported key bytes or A102 keys cannot construct it.
#[derive(Clone)]
pub struct SherlockCompleteLedgerKeys {
    inner: Arc<KeysInner>,
}
/// Public composed statement opening, revealed at readback.
/// Both `values.intermediate` and `values.done` are Step-mode Constant-space images
/// (`space == 0`, `activeData == 0`), never canonical-boundary `StateBlock`s.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SherlockCompleteCallOpening {
    pub values: LiteralCompleteValues,
    pub communication_rand: Fr,
}
struct Expected {
    fields: Vec<Fr>,
    commitment: Fr,
    rand: Fr,
}
/// Native work: SDK cost selection, pinned SRS load and keygen.
pub async fn derive_keys(
    layout: LiteralCompleteLayout,
    entry_point: &str,
    params_dir: &Path,
    limits: &SherlockCompleteKeyLimits,
) -> Result<SherlockCompleteLedgerKeys> {
    // Builtin key locations share the resolver namespace, so refuse that prefix.
    if entry_point.is_empty() || entry_point.starts_with("midnight/") {
        return Err(refusal("entry-point"));
    }
    if layout.statement_len() > limits.max_statement_lanes {
        return Err(refusal("statement-lane-limit"));
    }
    let (ir, _) = layout
        .emit(CircuitMode::LedgerCall)
        .map_err(|_| refusal("sherlock-emit"))?;
    let degree = ir.k();
    if degree == 0 || degree > limits.max_degree {
        return Err(refusal("exact-degree-unavailable"));
    }
    let snapshot = PinnedCallParameters::load(params_dir, degree, limits.max_srs_bytes)?;
    if snapshot.degree() != degree {
        return Err(refusal("parameter-degree"));
    }
    let params = CallParams {
        degree,
        params: snapshot.params(),
    };
    let (pk, vk) = ir
        .keygen(&params)
        .await
        .map_err(|_| refusal("sdk-keygen"))?;
    let pk = call_artifacts::encode_bounded(&pk, limits.max_prover_key_bytes)?;
    let vk = call_artifacts::encode_bounded(&vk, limits.max_verifier_key_bytes)?;
    let _: VerifierKey = codec::decode_artifact(&vk)?;
    let ir_bytes = codec::encode(&ir)?;
    let material = Arc::new(ProvingKeyMaterial {
        prover_key: pk,
        verifier_key: vk.clone(),
        ir_source: ir_bytes.clone(),
    });
    Ok(SherlockCompleteLedgerKeys {
        inner: Arc::new(KeysInner {
            layout,
            ir,
            ir_bytes,
            vk,
            material,
            params,
            entry_point: entry_point.into(),
        }),
    })
}
impl SherlockCompleteLedgerKeys {
    pub fn layout(&self) -> &LiteralCompleteLayout {
        &self.inner.layout
    }
    pub fn entry_point(&self) -> &str {
        &self.inner.entry_point
    }
    pub fn verifier_key_bytes(&self) -> &[u8] {
        &self.inner.vk
    }
    pub fn ir_bytes(&self) -> &[u8] {
        &self.inner.ir_bytes
    }
    /// Local operation object for a separately authorized deploy; installs nothing.
    pub fn operation(&self) -> Result<ContractOperation> {
        Ok(ContractOperation::new(
            Some(codec::decode_artifact(&self.inner.vk)?),
            Some(Sp::new(IrBuf(self.inner.ir_bytes.clone()))),
        ))
    }
    fn admit_operation(&self, operation: &ContractOperation) -> Result<()> {
        if operation.v2.is_some()
            || operation.v3.as_ref().map(codec::encode).transpose()? != Some(self.inner.vk.clone())
            || operation.ir.as_deref().map(|ir| ir.0.as_slice())
                != Some(self.inner.ir_bytes.as_slice())
        {
            return Err(refusal("supplied-operation-key-or-ir"));
        }
        Ok(())
    }
    fn expected(
        &self,
        opening: &SherlockCompleteCallOpening,
    ) -> Result<(Vec<u128>, AlignedValue, Expected)> {
        let (statement, input, commitment, fields) =
            compute_expected_call_statement(&self.inner.layout, opening)?;
        Ok((
            statement,
            input,
            Expected {
                fields,
                commitment,
                rand: opening.communication_rand,
            },
        ))
    }
    #[allow(dead_code)]
    fn expected_legacy_placeholder(
        &self,
        opening: &SherlockCompleteCallOpening,
    ) -> Result<(Vec<u128>, AlignedValue, Expected)> {
        let statement = self
            .inner
            .layout
            .statement(&opening.values)
            .map_err(|_| refusal("sherlock-statement"))?;
        let parts: Vec<AlignedValue> = statement.iter().map(|v| AlignedValue::from(*v)).collect();
        let input = AlignedValue::concat(parts.iter());
        let commitment = midnight_onchain_runtime::communication_commitment(
            input.clone(),
            AlignedValue::from(()),
            opening.communication_rand,
        );
        let fields = statement.iter().map(|v| Fr::from(*v)).collect();
        Ok((
            statement,
            input,
            Expected {
                fields,
                commitment,
                rand: opening.communication_rand,
            },
        ))
    }
    // Fail closed: one Fr per u128 lane and no transcript are required, not assumed.
    fn admit_preimage(&self, e: &Expected, p: &ProofPreimage) -> Result<()> {
        if p.key_location.0.as_ref() != self.inner.entry_point
            || p.inputs.as_slice() != e.fields.as_slice()
            || !p.private_transcript.is_empty()
            || !p.public_transcript_inputs.is_empty()
            || !p.public_transcript_outputs.is_empty()
            || p.communications_commitment != Some((e.commitment, e.rand))
        {
            return Err(refusal("sherlock-preimage"));
        }
        Ok(())
    }
}
/// Canonical empty-program [`Transcript`] for C8 call construction with exact
/// runtime charged-state accounting gas.
///
/// Historical C8 note: `sherlock_literal` originally set both
/// `guaranteed_transcript` and `fallible_transcript` to `None`, which
/// `midnight-ledger` (`ledger/src/verify.rs:1154`) rejects in
/// `StandardTransaction::sequencing_check` with
/// `SequencingCheckError::CallHasEmptyTranscripts`. Setting `transcript.gas` to
/// `RunningCost::ZERO` passes `Transaction::well_formed` but is rejected by
/// `LedgerState::apply_actions` -> `QueryContext::run_transcript` ->
/// `QueryContext::query` (`onchain-runtime/src/context.rs`, `QueryContext::query`
/// and `QueryContext::run_transcript`) with
/// `TranscriptRejected::Execution(OnchainProgramError::OutOfGas)` under any
/// runtime [`CostModel`] with non-zero RC-map accounting constants.
///
/// This helper computes the exact `RunningCost` required by
/// `QueryContext::new(pre_call_state.clone(), address).query::<ResultModeVerify>(&[], None, cost)`
/// and verifies the resulting `Transcript<InMemoryDB>` via `QueryContext::run_transcript`
/// with empty effects (`Effects::default()`), an empty VM program (`StorageArray::new()`),
/// and `Some(Sp::new(Transcript::VERSION))` (`2.3`). Callers must supply the actual
/// pre-call [`ChargedState`] and active runtime [`CostModel`]; neither is chain
/// readback or authenticated by the circuit VK.
pub fn empty_verified_transcript(
    address: ContractAddress,
    pre_call_state: &ChargedState<InMemoryDB>,
    cost: &CostModel,
) -> Result<Transcript<InMemoryDB>> {
    let baseline_rcmap_cost =
        cost.gc_rcmap_constant + cost.update_rcmap_constant + cost.get_writes_constant;
    if baseline_rcmap_cost == CostDuration::ZERO
        || cost.gc_rcmap_coeff_keys_removed_size == CostDuration::ZERO
    {
        return Err(refusal("zero-runtime-state-cost-model"));
    }
    let qcontext = QueryContext::new(pre_call_state.clone(), address);
    let query_res = qcontext
        .query::<ResultModeVerify>(&[], None, cost)
        .map_err(|_| refusal("transcript-query-rejected"))?;
    if query_res.gas_cost.bytes_written != 0 {
        return Err(refusal("uncharged-pre-call-state"));
    }
    if query_res.gas_cost == RunningCost::ZERO
        || query_res.gas_cost.compute_time < baseline_rcmap_cost
    {
        return Err(refusal("zero-transcript-gas-forbidden"));
    }
    if query_res.context.effects != Effects::default() || !query_res.events.is_empty() {
        return Err(refusal("unexpected-empty-transcript-effects"));
    }
    let transcript = Transcript {
        gas: query_res.gas_cost,
        effects: Effects::default(),
        program: StorageArray::new(),
        version: Some(Sp::new(Transcript::<InMemoryDB>::VERSION)),
    };
    let verified = qcontext
        .run_transcript(&transcript, cost)
        .map_err(|_| refusal("transcript-run-rejected"))?;
    if verified.gas_cost != transcript.gas
        || verified.context.effects != Effects::default()
        || !verified.events.is_empty()
    {
        return Err(refusal("transcript-gas-or-effects-mismatch"));
    }
    Ok(transcript)
}

fn admit_empty_verified_transcript(
    transcript: &Transcript<InMemoryDB>,
    address: ContractAddress,
    pre_call_state: &ChargedState<InMemoryDB>,
    cost: &CostModel,
) -> Result<()> {
    if transcript.effects != Effects::default()
        || !transcript.program.is_empty()
        || transcript.version.as_deref() != Some(&Transcript::<InMemoryDB>::VERSION)
    {
        return Err(refusal("non-empty-or-unversioned-transcript"));
    }
    if transcript.gas == RunningCost::ZERO {
        return Err(refusal("zero-transcript-gas-forbidden"));
    }
    let expected = empty_verified_transcript(address, pre_call_state, cost)?;
    if transcript != &expected {
        return Err(refusal("transcript-gas-mismatch"));
    }
    Ok(())
}

fn admit_call_transcripts(
    address: ContractAddress,
    pre_call_state: &ChargedState<InMemoryDB>,
    cost: &CostModel,
    guaranteed: Option<&Sp<Transcript<InMemoryDB>, InMemoryDB>>,
    fallible: Option<&Sp<Transcript<InMemoryDB>, InMemoryDB>>,
) -> Result<()> {
    if fallible.is_some() {
        return Err(refusal("unexpected-fallible-transcript"));
    }
    let Some(guaranteed) = guaranteed else {
        return Err(refusal("ledger-rejects-empty-call-transcripts"));
    };
    admit_empty_verified_transcript(guaranteed, address, pre_call_state, cost)
}

/// Unsigned, unproven raw call bound to an explicit pre-call [`ChargedState`]
/// and runtime [`CostModel`]. Fees/dust, deploy, signatures and submission
/// remain target-owner work.
pub fn raw_call_intent(
    keys: &SherlockCompleteLedgerKeys,
    address: ContractAddress,
    pre_call_state: &ChargedState<InMemoryDB>,
    cost: &CostModel,
    opening: &SherlockCompleteCallOpening,
    ttl: Timestamp,
    binding: PedersenRandomness,
) -> Result<RawIntent> {
    let (_, input, expected) = keys.expected(opening)?;
    let guaranteed_transcript = empty_verified_transcript(address, pre_call_state, cost)?;
    let prototype = ContractCallPrototype {
        address,
        entry_point: keys.inner.entry_point.as_bytes().into(),
        op: keys.operation()?,
        guaranteed_public_transcript: Some(guaranteed_transcript),
        fallible_public_transcript: None,
        private_transcript_outputs: vec![],
        input,
        output: AlignedValue::from(()),
        communication_commitment_rand: opening.communication_rand,
        key_location: KeyLocation(Cow::Owned(keys.inner.entry_point.clone())),
    };
    let empty: RawIntent = Intent {
        guaranteed_unshielded_offer: None,
        fallible_unshielded_offer: None,
        actions: Default::default(),
        dust_actions: None,
        ttl,
        binding_commitment: binding,
    };
    let intent = empty.add_call::<ProofPreimage>(prototype);
    admit_raw(keys, &expected, pre_call_state, cost, &intent)?;
    Ok(intent)
}
// Fail closed: require exactly one call carrying the opened commitment, the
// canonical guaranteed empty-program Transcript(v2.3) carrier (with non-zero
// runtime state gas for `(call.address, pre_call_state, cost)` and no fallible
// transcript), and an admitted preimage with empty transcript inputs/outputs.
fn admit_raw(
    keys: &SherlockCompleteLedgerKeys,
    expected: &Expected,
    pre_call_state: &ChargedState<InMemoryDB>,
    cost: &CostModel,
    intent: &RawIntent,
) -> Result<()> {
    if intent.guaranteed_unshielded_offer.is_some()
        || intent.fallible_unshielded_offer.is_some()
        || intent.dust_actions.is_some()
    {
        return Err(refusal("unexpected-raw-intent-offers"));
    }
    if intent.actions.len() != 1 || intent.calls().count() != 1 {
        return Err(refusal("single-raw-call"));
    }
    let call = intent.calls().next().ok_or(refusal("native-call"))?;
    if call.entry_point.0.as_slice() != keys.inner.entry_point.as_bytes()
        || call.communication_commitment != expected.commitment
    {
        return Err(refusal("native-call-identity"));
    }
    admit_call_transcripts(
        call.address,
        pre_call_state,
        cost,
        call.guaranteed_transcript.as_ref(),
        call.fallible_transcript.as_ref(),
    )?;
    let ProofPreimageVersioned::V2(preimage) = &call.proof else {
        return Err(refusal("native-preimage-version"));
    };
    keys.admit_preimage(expected, preimage)
}
#[derive(Clone)]
struct SherlockCompleteResolver {
    location: String,
    material: Arc<ProvingKeyMaterial>,
}
impl Resolver for SherlockCompleteResolver {
    async fn resolve_key(&self, location: KeyLocation) -> io::Result<Option<ProvingKeyMaterial>> {
        Ok((location.0.as_ref() == self.location).then(|| (*self.material).clone()))
    }
}
pub struct SherlockCompleteCallProver {
    keys: SherlockCompleteLedgerKeys,
    resolver: SherlockCompleteResolver,
    expected: Arc<Expected>,
    rng: StdRng,
}
impl SherlockCompleteCallProver {
    pub fn new(
        keys: SherlockCompleteLedgerKeys,
        opening: &SherlockCompleteCallOpening,
        rng: StdRng,
    ) -> Result<Self> {
        let (_, _, expected) = keys.expected(opening)?;
        let resolver = SherlockCompleteResolver {
            location: keys.inner.entry_point.clone(),
            material: keys.inner.material.clone(),
        };
        Ok(Self {
            keys,
            resolver,
            expected: Arc::new(expected),
            rng,
        })
    }
    pub fn from_request(
        keys: SherlockCompleteLedgerKeys,
        request: &LiteralCompleteExecutionRequest,
        communication_rand: Fr,
        rng: StdRng,
    ) -> Result<(SherlockCompleteCallOpening, Self)> {
        let opening = keys.opening_for_request(request, communication_rand)?;
        let prover = Self::new(keys, &opening, rng)?;
        Ok((opening, prover))
    }
}
impl ProvingProvider for SherlockCompleteCallProver {
    async fn check(
        &self,
        preimage: &ProofPreimage,
    ) -> std::result::Result<Vec<Option<usize>>, ProvingError> {
        self.keys.admit_preimage(&self.expected, preimage)?;
        let ir = &self.keys.inner.ir;
        let activity =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| preimage.check(ir)))
                .map_err(|_| refusal("sdk-check-panic"))?
                .map_err(|_| refusal("unsatisfied"))?;
        Ok(activity)
    }
    async fn prove(
        self,
        preimage: &ProofPreimage,
        binding: Option<Fr>,
    ) -> std::result::Result<Proof, ProvingError> {
        self.keys.admit_preimage(&self.expected, preimage)?;
        let mut bound = preimage.clone();
        bound.binding_input = binding.ok_or(refusal("final-binding-required"))?;
        let (proof, activity) = bound
            .prove::<IrSource>(self.rng, &self.keys.inner.params, &self.resolver)
            .await?;
        if !activity.is_empty() {
            return Err(refusal("unexpected-transcript-activity").into());
        }
        Ok(proof)
    }
    fn split(&mut self) -> Self {
        let rng = StdRng::from_rng(&mut self.rng).expect("StdRng is an infallible seed source");
        Self {
            keys: self.keys.clone(),
            resolver: self.resolver.clone(),
            expected: self.expected.clone(),
            rng,
        }
    }
    fn resolver(&self) -> &impl Resolver {
        &self.resolver
    }
}
/// SDK Intent::prove receives the raw intent once after admitting `raw` against
/// the caller-supplied pre-call [`ChargedState`] and runtime [`CostModel`].
pub async fn prove_call(
    raw: &RawIntent,
    segment: u16,
    prover: SherlockCompleteCallProver,
    pre_call_state: &ChargedState<InMemoryDB>,
    cost: &CostModel,
) -> Result<ProvenIntent> {
    admit_raw(&prover.keys, &prover.expected, pre_call_state, cost, raw)?;
    let (_, intent) = raw
        .prove(segment, prover, cost)
        .await
        .map_err(|_| refusal("sdk-intent-proving"))?;
    let call = intent.calls().next().ok_or(refusal("missing-call"))?;
    admit_call_transcripts(
        call.address,
        pre_call_state,
        cost,
        call.guaranteed_transcript.as_ref(),
        call.fallible_transcript.as_ref(),
    )?;
    Ok(intent)
}
/// Local result only: real SDK verification of a supplied proven intent against
/// the caller's expected raw intent, pre-call [`ChargedState`], runtime [`CostModel`],
/// and source-derived operation. This is not a transaction, inclusion, finality,
/// fee, state or settlement readback. Nothing Sherlock-specific is persisted on chain,
/// so third parties need the off-chain opening. local_intent_hash_hex is recomputed
/// for the caller-supplied segment and is not a chain transaction identifier.
#[derive(Clone, Debug, Serialize)]
pub struct SherlockCompleteLocalCallVerification {
    pub scope: &'static str,
    pub relation_scope: &'static str,
    pub endpoint_qualification: &'static str,
    pub context_hash: String,
    pub layout_hash: String,
    pub relation_hash: String,
    pub verifier_key_hash: String,
    pub ir_hash: String,
    pub declared_build_closure: String,
    pub declared_build_closure_qualification: &'static str,
    pub public_inputs: Vec<String>,
    pub statement_values: Vec<String>,
    pub segment: u16,
    pub local_intent_hash_hex: String,
    pub open_obligations: Vec<&'static str>,
}
/// `expected_raw` is the caller's independently retained raw intent, normally the
/// one returned by raw_call_intent. Its address, entry point, TTL, offers, binding,
/// pre-call [`ChargedState`] and runtime [`CostModel`] are the caller's expectation,
/// not chain readback. Installation, apply, fees and inclusion are not established here.
pub fn verify_call(
    keys: &SherlockCompleteLedgerKeys,
    operation: &ContractOperation,
    pre_call_state: &ChargedState<InMemoryDB>,
    cost: &CostModel,
    expected_raw: &RawIntent,
    candidate: &ProvenIntent,
    opening: &SherlockCompleteCallOpening,
    segment: u16,
) -> Result<SherlockCompleteLocalCallVerification> {
    keys.admit_operation(operation)?;
    let (statement, _, expected) = keys.expected(opening)?;
    admit_raw(keys, &expected, pre_call_state, cost, expected_raw)?;
    // Whole-intent equality, as in proving.rs validate_final_envelope. Offers, dust,
    // TTL, binding randomness, call address, entry point, transcript carrier and order,
    // and any extra action must all equal the expected raw intent.
    if candidate.binding_commitment != expected_raw.binding_commitment
        || candidate.erase_proofs().erase_signatures()
            != expected_raw.erase_proofs().erase_signatures()
    {
        return Err(refusal("final-intent-identity"));
    }
    if candidate.actions.len() != 1 || candidate.calls().count() != 1 {
        return Err(refusal("single-call"));
    }
    let call: &ContractCall<ProofMarker, InMemoryDB> =
        candidate.calls().next().ok_or(refusal("missing-call"))?;
    if !matches!(call.proof, ProofVersioned::V3(_))
        || call.communication_commitment != expected.commitment
    {
        return Err(refusal("call-shape-or-commitment"));
    }
    admit_call_transcripts(
        call.address,
        pre_call_state,
        cost,
        call.guaranteed_transcript.as_ref(),
        call.fallible_transcript.as_ref(),
    )?;
    let parent: Pedersen = candidate.binding_commitment.into();
    let pi = call.public_inputs(parent);
    if pi.len() != 2 || pi[1] != expected.commitment {
        return Err(refusal("ledger-public-statement"));
    }
    <ProofMarker as ProofKind<InMemoryDB>>::proof_verify(
        operation,
        &call.proof,
        pi.clone(),
        call,
        ProofVerificationMode::Real,
    )
    .map_err(|_| refusal("ledger-real-proof-verification"))?;
    let intent_hash = candidate
        .erase_proofs()
        .erase_signatures()
        .intent_hash(segment);
    if call.entry_point.0.as_slice() != keys.inner.entry_point.as_bytes() {
        return Err(refusal("call-shape-or-commitment"));
    }
    Ok(SherlockCompleteLocalCallVerification {
        scope: "local-real-proof-verification-of-supplied-composed-a102-a80-intent-against-expected-raw-intent-and-source-derived-operation; step-mode-constant-space-done-noncanonical; not installed, submitted, included or settled",
        relation_scope: COMPLETE_SCOPE,
        endpoint_qualification: COMPOSED_LEDGER_ENDPOINT_QUALIFICATION,
        context_hash: keys.inner.layout.context_hex(),
        layout_hash: codec::hex(&keys.inner.layout.layout_hash()),
        relation_hash: codec::hex(&keys.inner.layout.relation_hash()),
        verifier_key_hash: codec::hash(&keys.inner.vk),
        ir_hash: codec::hash(&keys.inner.ir_bytes),
        declared_build_closure: composed_ledger_source_closure(),
        declared_build_closure_qualification: COMPOSED_LEDGER_CLOSURE_QUALIFICATION,
        public_inputs: pi.iter().map(|f| codec::hex(&f.as_le_bytes())).collect(),
        statement_values: statement.iter().map(|v| v.to_string()).collect(),
        segment,
        local_intent_hash_hex: codec::native_hex(&intent_hash)?,
        open_obligations: composed_open_obligations(),
    })
}

/// Candid declared source closure for the composed-ledger consumer seam.
/// Binds `crate::proving::source_closure()` together with `literal_complete_bridge.rs`,
/// `sherlock_literal_complete.rs`, and `sherlock_transaction_complete.rs`.
/// Does not claim coverage of `cfg(test)` modules, unlisted workspace files,
/// external SDK/registry sources, SRS files, or post-baseline shared registration edits.
pub fn composed_ledger_source_closure() -> String {
    let mut material = String::new();
    material.push_str(COMPOSED_LEDGER_CLOSURE_DOMAIN);
    material.push('\0');
    material.push_str(&crate::proving::source_closure());
    material.push('\0');
    material.push_str(include_str!(
        "../../mori-backend/src/sherlock_machine/literal_complete_bridge.rs"
    ));
    material.push('\0');
    material.push_str(include_str!("sherlock_literal_complete.rs"));
    material.push('\0');
    material.push_str(include_str!("sherlock_transaction_complete.rs"));
    codec::hash(material.as_bytes())
}

/// Complete open-obligation vector preserving all 9 `COMPLETE_OPEN_OBLIGATIONS`
/// from `literal_complete_bridge` plus the 3 composed-ledger consumer qualifications.
pub fn composed_open_obligations() -> Vec<&'static str> {
    let mut out = COMPLETE_OPEN_OBLIGATIONS.to_vec();
    out.extend_from_slice(&COMPOSED_LEDGER_ADDITIONAL_OBLIGATIONS);
    out
}

/// Build and validate a [`LiteralCompleteLayout`] for a [`LiteralCompleteExecutionRequest`]
/// under the caller's explicit `max_statement_lanes` bound.
pub fn build_layout_for_request(
    request: &LiteralCompleteExecutionRequest,
    max_statement_lanes: usize,
) -> Result<LiteralCompleteLayout> {
    let layout =
        LiteralCompleteLayout::new(&request.capacities, &request.identity, max_statement_lanes)
            .map_err(|_| refusal("sherlock-complete-layout"))?;
    if layout.statement_len() > max_statement_lanes {
        return Err(refusal("statement-lane-limit"));
    }
    if request.values.literal_envelope != LiteralEnvelopeValues::step1()
        || request.values.return_envelope != LiteralEnvelopeValues::step2()
    {
        return Err(refusal("sherlock-statement"));
    }
    let statement = layout
        .statement(&request.values)
        .map_err(|_| refusal("sherlock-statement"))?;
    if statement.len() != layout.statement_len() {
        return Err(refusal("sherlock-statement"));
    }
    Ok(layout)
}

/// Compute the ordered `u128` statement, concatenated [`AlignedValue`] call input,
/// communication commitment, and [`Fr`] field vector for a [`LiteralCompleteLayout`]
/// and [`SherlockCompleteCallOpening`].
pub fn compute_expected_call_statement(
    layout: &LiteralCompleteLayout,
    opening: &SherlockCompleteCallOpening,
) -> Result<(Vec<u128>, AlignedValue, Fr, Vec<Fr>)> {
    if opening.values.literal_envelope != LiteralEnvelopeValues::step1()
        || opening.values.return_envelope != LiteralEnvelopeValues::step2()
    {
        return Err(refusal("sherlock-statement"));
    }
    let statement = layout
        .statement(&opening.values)
        .map_err(|_| refusal("sherlock-statement"))?;
    if statement.len() != layout.statement_len() {
        return Err(refusal("sherlock-statement"));
    }
    let parts: Vec<AlignedValue> = statement.iter().map(|v| AlignedValue::from(*v)).collect();
    let input = AlignedValue::concat(parts.iter());
    let commitment = midnight_onchain_runtime::communication_commitment(
        input.clone(),
        AlignedValue::from(()),
        opening.communication_rand,
    );
    let fields = statement.iter().map(|v| Fr::from(*v)).collect();
    Ok((statement, input, commitment, fields))
}

/// Fail-closed preimage admission against a [`LiteralCompleteLayout`], entry point,
/// and [`SherlockCompleteCallOpening`].
pub fn admit_call_preimage(
    layout: &LiteralCompleteLayout,
    entry_point: &str,
    opening: &SherlockCompleteCallOpening,
    preimage: &ProofPreimage,
) -> Result<Vec<u128>> {
    if entry_point.is_empty() || entry_point.starts_with("midnight/") {
        return Err(refusal("entry-point"));
    }
    let (statement, _, commitment, fields) = compute_expected_call_statement(layout, opening)?;
    if preimage.key_location.0.as_ref() != entry_point
        || preimage.inputs.as_slice() != fields.as_slice()
        || !preimage.private_transcript.is_empty()
        || !preimage.public_transcript_inputs.is_empty()
        || !preimage.public_transcript_outputs.is_empty()
        || preimage.communications_commitment != Some((commitment, opening.communication_rand))
    {
        return Err(refusal("sherlock-preimage"));
    }
    Ok(statement)
}

impl SherlockCompleteCallOpening {
    /// Construct a [`SherlockCompleteCallOpening`] from a [`LiteralCompleteExecutionRequest`]
    /// after verifying capacity/source-identity binding and statement admissibility against `layout`.
    pub fn from_request(
        layout: &LiteralCompleteLayout,
        request: &LiteralCompleteExecutionRequest,
        communication_rand: Fr,
    ) -> Result<Self> {
        if layout.capacities() != &request.capacities || layout.identity() != &request.identity {
            return Err(refusal("request-layout-mismatch"));
        }
        let opening = Self {
            values: request.values.clone(),
            communication_rand,
        };
        let _ = compute_expected_call_statement(layout, &opening)?;
        Ok(opening)
    }

    /// Verify that `self` and `layout` match the capacities, source identity, and
    /// public values of `request`.
    pub fn admit_request(
        &self,
        layout: &LiteralCompleteLayout,
        request: &LiteralCompleteExecutionRequest,
    ) -> Result<()> {
        if layout.capacities() != &request.capacities || layout.identity() != &request.identity {
            return Err(refusal("request-layout-mismatch"));
        }
        if self.values != request.values {
            return Err(refusal("request-opening-mismatch"));
        }
        let _ = compute_expected_call_statement(layout, self)?;
        Ok(())
    }
}

impl SherlockCompleteLedgerKeys {
    /// Verify that `request` matches this keyset's bound [`LiteralCompleteLayout`]
    /// and return its validated ordered statement.
    pub fn admit_request(&self, request: &LiteralCompleteExecutionRequest) -> Result<Vec<u128>> {
        if self.inner.layout.capacities() != &request.capacities
            || self.inner.layout.identity() != &request.identity
        {
            return Err(refusal("request-layout-mismatch"));
        }
        self.inner
            .layout
            .statement(&request.values)
            .map_err(|_| refusal("sherlock-statement"))
    }

    /// Build a validated [`SherlockCompleteCallOpening`] bound to `self` from `request`.
    pub fn opening_for_request(
        &self,
        request: &LiteralCompleteExecutionRequest,
        communication_rand: Fr,
    ) -> Result<SherlockCompleteCallOpening> {
        SherlockCompleteCallOpening::from_request(&self.inner.layout, request, communication_rand)
    }
}

/// Native work: validate `request`, build its [`LiteralCompleteLayout`], and derive
/// [`SherlockCompleteLedgerKeys`] along with the ordered statement lanes.
pub async fn derive_keys_for_request(
    request: &LiteralCompleteExecutionRequest,
    entry_point: &str,
    params_dir: &Path,
    limits: &SherlockCompleteKeyLimits,
) -> Result<(SherlockCompleteLedgerKeys, Vec<u128>)> {
    if entry_point.is_empty() || entry_point.starts_with("midnight/") {
        return Err(refusal("entry-point"));
    }
    let layout = build_layout_for_request(request, limits.max_statement_lanes)?;
    let statement = layout
        .statement(&request.values)
        .map_err(|_| refusal("sherlock-statement"))?;
    let keys = derive_keys(layout, entry_point, params_dir, limits).await?;
    Ok((keys, statement))
}

/// Construct an unsigned, unproven raw call intent directly from a
/// [`LiteralCompleteExecutionRequest`] bound to `keys`.
pub fn raw_call_intent_for_request(
    keys: &SherlockCompleteLedgerKeys,
    address: ContractAddress,
    pre_call_state: &ChargedState<InMemoryDB>,
    cost: &CostModel,
    request: &LiteralCompleteExecutionRequest,
    communication_rand: Fr,
    ttl: Timestamp,
    binding: PedersenRandomness,
) -> Result<(SherlockCompleteCallOpening, RawIntent)> {
    let opening = keys.opening_for_request(request, communication_rand)?;
    let raw = raw_call_intent(keys, address, pre_call_state, cost, &opening, ttl, binding)?;
    Ok((opening, raw))
}

/// Verify a proven call intent against `keys`, `operation`, `pre_call_state`, `cost`,
/// `expected_raw`, `request`, and `opening`.
pub fn verify_call_for_request(
    keys: &SherlockCompleteLedgerKeys,
    operation: &ContractOperation,
    pre_call_state: &ChargedState<InMemoryDB>,
    cost: &CostModel,
    expected_raw: &RawIntent,
    candidate: &ProvenIntent,
    request: &LiteralCompleteExecutionRequest,
    opening: &SherlockCompleteCallOpening,
    segment: u16,
) -> Result<SherlockCompleteLocalCallVerification> {
    opening.admit_request(keys.layout(), request)?;
    verify_call(
        keys,
        operation,
        pre_call_state,
        cost,
        expected_raw,
        candidate,
        opening,
        segment,
    )
}

#[cfg(test)]
#[path = "sherlock_literal_complete_normal_tests.rs"]
mod normal_tests;
