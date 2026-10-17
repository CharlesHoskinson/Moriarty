//! Ledger-9 contract-operation route for the Sherlock A102 Literal relation.
//! The installed operation VK is the custom decider: verification uses the
//! existing ProofMarker::proof_verify Real path, not a VerifyProof instruction,
//! ledger10, a Core ProofBundle or a financial call artifact. The statement is
//! bound by the call communication commitment. Its opening is disclosed off-chain and
//! no chain-side decoder exists. One Fr per u128 lane, a transcript-free call and
//! a two-element call PI are unverified SDK hypotheses, enforced fail-closed (GAP-2).
//! Keygen, proving, ledger acceptance and stagenet readback are UNOBSERVED.
use crate::{Error, Result, call_artifacts, codec, parameters::PinnedCallParameters};
use crate::{prepare::RawIntent, proving::ProvenIntent};
use midnight_base_crypto::{fab::AlignedValue, time::Timestamp};
use midnight_coin_structure::contract::ContractAddress;
use midnight_ledger::construct::ContractCallPrototype;
use midnight_ledger::structure::{
    ContractCall, Intent, ProofKind, ProofMarker, ProofPreimageVersioned, ProofVersioned,
};
use midnight_ledger::verify::ProofVerificationMode;
use midnight_onchain_runtime::cost_model::CostModel;
use midnight_onchain_state::state::{ContractOperation, IrBuf};
use midnight_storage::{arena::Sp, db::InMemoryDB};
use midnight_transient_crypto::commitment::{Pedersen, PedersenRandomness};
use midnight_transient_crypto::curve::Fr;
use midnight_transient_crypto::proofs::{
    KeyLocation, ParamsProver, ParamsProverProvider, Proof, ProofPreimage, ProvingKeyMaterial,
    ProvingProvider, Resolver, VerifierKey, Zkir,
};
use midnight_zkir_v3::IrSource;
use mori_backend::sherlock_machine::literal_bridge::{
    CircuitMode, LiteralLayout, LiteralValues, OPEN_OBLIGATIONS,
};
use rand::{SeedableRng, rngs::StdRng};
use serde::Serialize;
use std::{borrow::Cow, io, path::Path, sync::Arc};

fn refusal(code: &'static str) -> Error {
    Error::new("sherlock-ledger", code)
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
pub struct SherlockKeyLimits {
    pub max_degree: u8,
    pub max_srs_bytes: u64,
    pub max_prover_key_bytes: u64,
    pub max_verifier_key_bytes: u64,
    /// Explicit statement lane bound, admitted before emission, k or SRS work.
    pub max_statement_lanes: usize,
}
struct KeysInner {
    layout: LiteralLayout,
    ir: IrSource,
    ir_bytes: Vec<u8>,
    vk: Vec<u8>,
    material: Arc<ProvingKeyMaterial>,
    params: CallParams,
    entry_point: String,
}
/// Built only by source-recompiled SDK keygen; imported key bytes cannot construct it.
#[derive(Clone)]
pub struct SherlockLedgerKeys {
    inner: Arc<KeysInner>,
}
/// Public statement opening, revealed at readback.
pub struct SherlockCallOpening {
    pub values: LiteralValues,
    pub communication_rand: Fr,
}
struct Expected {
    fields: Vec<Fr>,
    commitment: Fr,
    rand: Fr,
}
/// Native work: SDK cost selection, pinned SRS load and keygen.
pub async fn derive_keys(layout: LiteralLayout, entry_point: &str, params_dir: &Path, limits: &SherlockKeyLimits) -> Result<SherlockLedgerKeys> {
    // Builtin key locations share the resolver namespace, so refuse that prefix.
    if entry_point.is_empty() || entry_point.starts_with("midnight/") {
        return Err(refusal("entry-point"));
    }
    if layout.statement_len() > limits.max_statement_lanes {
        return Err(refusal("statement-lane-limit"));
    }
    let (ir, _) = layout.emit(CircuitMode::LedgerCall).map_err(|_| refusal("sherlock-emit"))?;
    let degree = ir.k();
    if degree == 0 || degree > limits.max_degree {
        return Err(refusal("exact-degree-unavailable"));
    }
    let snapshot = PinnedCallParameters::load(params_dir, degree, limits.max_srs_bytes)?;
    if snapshot.degree() != degree {
        return Err(refusal("parameter-degree"));
    }
    let params = CallParams { degree, params: snapshot.params() };
    let (pk, vk) = ir.keygen(&params).await.map_err(|_| refusal("sdk-keygen"))?;
    let pk = call_artifacts::encode_bounded(&pk, limits.max_prover_key_bytes)?;
    let vk = call_artifacts::encode_bounded(&vk, limits.max_verifier_key_bytes)?;
    let _: VerifierKey = codec::decode_artifact(&vk)?;
    let ir_bytes = codec::encode(&ir)?;
    let material = Arc::new(ProvingKeyMaterial { prover_key: pk, verifier_key: vk.clone(), ir_source: ir_bytes.clone() });
    Ok(SherlockLedgerKeys {
        inner: Arc::new(KeysInner { layout, ir, ir_bytes, vk, material, params, entry_point: entry_point.into() }),
    })
}
impl SherlockLedgerKeys {
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
            || operation.ir.as_deref().map(|ir| ir.0.as_slice()) != Some(self.inner.ir_bytes.as_slice())
        {
            return Err(refusal("supplied-operation-key-or-ir"));
        }
        Ok(())
    }
    fn expected(&self, opening: &SherlockCallOpening) -> Result<(Vec<u128>, AlignedValue, Expected)> {
        let statement = self.inner.layout.statement(&opening.values).map_err(|_| refusal("sherlock-statement"))?;
        let parts: Vec<AlignedValue> = statement.iter().map(|v| AlignedValue::from(*v)).collect();
        let input = AlignedValue::concat(parts.iter());
        let commitment = midnight_onchain_runtime::communication_commitment(
            input.clone(), AlignedValue::from(()), opening.communication_rand,
        );
        let fields = statement.iter().map(|v| Fr::from(*v)).collect();
        Ok((statement, input, Expected { fields, commitment, rand: opening.communication_rand }))
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
/// Unsigned, unproven raw call. Fees/dust, deploy, signatures and submission
/// remain target-owner work.
pub fn raw_call_intent(keys: &SherlockLedgerKeys, address: ContractAddress, opening: &SherlockCallOpening, ttl: Timestamp, binding: PedersenRandomness) -> Result<RawIntent> {
    let (_, input, expected) = keys.expected(opening)?;
    let prototype = ContractCallPrototype {
        address,
        entry_point: keys.inner.entry_point.as_bytes().into(),
        op: keys.operation()?,
        guaranteed_public_transcript: None,
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
    admit_raw(keys, &expected, &intent)?;
    Ok(intent)
}
// Fail closed: require exactly one transcript-free call carrying the opened
// commitment and an admitted preimage. SDK acceptance of this shape is not assumed.
fn admit_raw(keys: &SherlockLedgerKeys, expected: &Expected, intent: &RawIntent) -> Result<()> {
    if intent.actions.len() != 1 || intent.calls().count() != 1 {
        return Err(refusal("single-raw-call"));
    }
    let call = intent.calls().next().ok_or(refusal("native-call"))?;
    if call.communication_commitment != expected.commitment
        || call.guaranteed_transcript.is_some()
        || call.fallible_transcript.is_some()
    {
        return Err(refusal("native-call-identity"));
    }
    let ProofPreimageVersioned::V2(preimage) = &call.proof else {
        return Err(refusal("native-preimage-version"));
    };
    keys.admit_preimage(expected, preimage)
}
#[derive(Clone)]
struct SherlockResolver {
    location: String,
    material: Arc<ProvingKeyMaterial>,
}
impl Resolver for SherlockResolver {
    async fn resolve_key(&self, location: KeyLocation) -> io::Result<Option<ProvingKeyMaterial>> {
        Ok((location.0.as_ref() == self.location).then(|| (*self.material).clone()))
    }
}
pub struct SherlockCallProver {
    keys: SherlockLedgerKeys,
    resolver: SherlockResolver,
    expected: Arc<Expected>,
    rng: StdRng,
}
impl SherlockCallProver {
    pub fn new(keys: SherlockLedgerKeys, opening: &SherlockCallOpening, rng: StdRng) -> Result<Self> {
        let (_, _, expected) = keys.expected(opening)?;
        let resolver = SherlockResolver { location: keys.inner.entry_point.clone(), material: keys.inner.material.clone() };
        Ok(Self { keys, resolver, expected: Arc::new(expected), rng })
    }
}
impl ProvingProvider for SherlockCallProver {
    async fn check(&self, preimage: &ProofPreimage) -> anyhow::Result<Vec<Option<usize>>> {
        self.keys.admit_preimage(&self.expected, preimage)?;
        let ir = &self.keys.inner.ir;
        let activity = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| preimage.check(ir)))
            .map_err(|_| refusal("sdk-check-panic"))?
            .map_err(|_| refusal("unsatisfied"))?;
        Ok(activity)
    }
    async fn prove(self, preimage: &ProofPreimage, binding: Option<Fr>) -> anyhow::Result<Proof> {
        self.keys.admit_preimage(&self.expected, preimage)?;
        let mut bound = preimage.clone();
        bound.binding_input = binding.ok_or(refusal("final-binding-required"))?;
        let (proof, activity) = bound.prove::<IrSource>(self.rng, &self.keys.inner.params, &self.resolver).await?;
        if !activity.is_empty() {
            return Err(refusal("unexpected-transcript-activity").into());
        }
        Ok(proof)
    }
    fn split(&mut self) -> Self {
        let rng = StdRng::from_rng(&mut self.rng).expect("StdRng is an infallible seed source");
        Self { keys: self.keys.clone(), resolver: self.resolver.clone(), expected: self.expected.clone(), rng }
    }
    fn resolver(&self) -> &impl Resolver {
        &self.resolver
    }
}
/// SDK Intent::prove receives the raw intent once.
pub async fn prove_call(raw: &RawIntent, segment: u16, prover: SherlockCallProver, cost: &CostModel) -> Result<ProvenIntent> {
    let (_, intent) = raw.prove(segment, prover, cost).await.map_err(|_| refusal("sdk-intent-proving"))?;
    Ok(intent)
}
/// Local result only: real SDK verification of a supplied proven intent against
/// the caller's expected raw intent and the source-derived operation. This is not
/// a transaction, inclusion, finality, fee, state or settlement readback. Nothing
/// Sherlock-specific is persisted on chain, so third parties need the off-chain
/// opening. local_intent_hash_hex is recomputed for the caller-supplied segment
/// and is not a chain transaction identifier.
#[derive(Debug, Serialize)]
pub struct SherlockLocalCallVerification {
    pub scope: &'static str,
    pub context_hash: String,
    pub verifier_key_hash: String,
    pub ir_hash: String,
    pub declared_build_closure: String,
    pub public_inputs: Vec<String>,
    pub statement_values: Vec<String>,
    pub segment: u16,
    pub local_intent_hash_hex: String,
    pub open_obligations: Vec<&'static str>,
}
/// `expected_raw` is the caller's independently retained raw intent, normally the
/// one returned by raw_call_intent. Its address, entry point, TTL, offers and
/// binding are the caller's expectation, not chain readback. Installation, apply,
/// fees and inclusion are not established here.
pub fn verify_call(keys: &SherlockLedgerKeys, operation: &ContractOperation, expected_raw: &RawIntent, candidate: &ProvenIntent, opening: &SherlockCallOpening, segment: u16) -> Result<SherlockLocalCallVerification> {
    keys.admit_operation(operation)?;
    let (statement, _, expected) = keys.expected(opening)?;
    admit_raw(keys, &expected, expected_raw)?;
    // Whole-intent equality, as in proving.rs validate_final_envelope. Offers, dust,
    // TTL, binding randomness, call address, entry point and order, and any extra
    // action must all equal the expected raw intent.
    if candidate.binding_commitment != expected_raw.binding_commitment
        || candidate.erase_proofs().erase_signatures() != expected_raw.erase_proofs().erase_signatures()
    {
        return Err(refusal("final-intent-identity"));
    }
    if candidate.actions.len() != 1 || candidate.calls().count() != 1 {
        return Err(refusal("single-call"));
    }
    let call: &ContractCall<ProofMarker, InMemoryDB> = candidate.calls().next().ok_or(refusal("missing-call"))?;
    if !matches!(call.proof, ProofVersioned::V3(_))
        || call.guaranteed_transcript.is_some()
        || call.fallible_transcript.is_some()
        || call.communication_commitment != expected.commitment
    {
        return Err(refusal("call-shape-or-commitment"));
    }
    let parent: Pedersen = candidate.binding_commitment.into();
    let pi = call.public_inputs(parent);
    if pi.len() != 2 || pi[1] != expected.commitment {
        return Err(refusal("ledger-public-statement"));
    }
    <ProofMarker as ProofKind<InMemoryDB>>::proof_verify(
        operation, &call.proof, pi.clone(), call, ProofVerificationMode::Real,
    )
    .map_err(|_| refusal("ledger-real-proof-verification"))?;
    let intent_hash = candidate.erase_proofs().erase_signatures().intent_hash(segment);
    Ok(SherlockLocalCallVerification {
        scope: "local-real-proof-verification-of-supplied-intent-against-expected-raw-intent-and-source-derived-operation; not installed, submitted, included or settled",
        context_hash: keys.inner.layout.context_hex(),
        verifier_key_hash: codec::hash(&keys.inner.vk),
        ir_hash: codec::hash(&keys.inner.ir_bytes),
        declared_build_closure: crate::proving::source_closure(),
        public_inputs: pi.iter().map(|f| codec::hex(&f.as_le_bytes())).collect(),
        statement_values: statement.iter().map(|v| v.to_string()).collect(),
        segment,
        local_intent_hash_hex: codec::native_hex(&intent_hash)?,
        open_obligations: OPEN_OBLIGATIONS.to_vec(),
    })
}
