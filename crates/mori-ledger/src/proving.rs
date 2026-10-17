//! Actual SDK call proving, with source-derived immutable material.
//!
//! Native entry points below execute cost/key/proof/crypto work when called.
//! Ordinary preparation/tests must not call them. Supplied local state and an
//! exact supplied operation do not authenticate installation or settlement.
//! No imported manifest/hash can construct the private derived key capability.
use crate::{
    Error, KeyExportDiagnostic, KeyExportFailureReason, KeyExportKind, Result, codec,
    prepare::{PreparedNativeCall, RawIntent},
    profile::Deployment,
    relation,
};
use midnight_ledger::structure::{
    ContractAction, ContractCall, Intent, IntentHash, ProofKind, ProofMarker, ProofPreimageMarker,
    ProofVersioned, Utxo,
};
use midnight_ledger::verify::ProofVerificationMode;
use midnight_onchain_state::state::{ContractOperation, IrBuf};
use midnight_proofs::poly::commitment::Params;
use midnight_serialize::{Serializable, Tagged};
use midnight_storage::{arena::Sp, db::InMemoryDB};
use midnight_transient_crypto::{
    commitment::{Pedersen, PedersenRandomness},
    curve::Fr,
    proofs::{
        KeyLocation, ParamsProver, ParamsProverProvider, Proof, ProofPreimage, ProvingKeyMaterial,
        ProvingProvider, Resolver, Zkir,
    },
};
use midnight_zkir_v3::IrSource;
use mori_core::Program;
use rand::{SeedableRng, rngs::StdRng};
use serde::Serialize;
use std::{
    io::{self, Write},
    sync::Arc,
};

pub type ProvenIntent = Intent<(), ProofMarker, PedersenRandomness, InMemoryDB>;
fn refusal(code: &'static str) -> Error {
    Error::new("native-proving", code)
}

/// Caller-selected resource bounds, without invented default PK/SRS envelopes.
/// This is a library work bound, not maintainer campaign admission.
pub struct KeyGenerationLimits {
    pub max_degree: u8,
    pub max_prover_key_bytes: usize,
    pub max_verifier_key_bytes: usize,
}
struct BoundedWriter {
    bytes: Vec<u8>,
    cap: usize,
    first_failure: Option<FailedKeyWrite>,
}
struct FailedKeyWrite {
    reason: KeyExportFailureReason,
    accepted_bytes: usize,
    failed_write_bytes: usize,
    attempted_minimum: Option<usize>,
}
fn check_key_write(
    accepted: usize,
    chunk: usize,
    cap: usize,
) -> std::result::Result<usize, KeyExportFailureReason> {
    let total = accepted
        .checked_add(chunk)
        .ok_or(KeyExportFailureReason::ByteCountOverflow)?;
    if total > cap {
        Err(KeyExportFailureReason::ByteCapExceeded)
    } else {
        Ok(total)
    }
}
impl Write for BoundedWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.first_failure.is_some() {
            return Err(io::Error::other("generated key export already refused"));
        }
        if let Err(reason) = check_key_write(self.bytes.len(), bytes.len(), self.cap) {
            self.first_failure.get_or_insert_with(|| FailedKeyWrite {
                reason,
                accepted_bytes: self.bytes.len(),
                failed_write_bytes: bytes.len(),
                attempted_minimum: self.bytes.len().checked_add(bytes.len()),
            });
            return Err(io::Error::other(
                "generated key exceeds caller resource limit",
            ));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
fn encode_key<T: Serializable + Tagged>(
    value: &T,
    action: &str,
    key_kind: KeyExportKind,
    degree: u8,
    cap: usize,
) -> Result<Vec<u8>> {
    let mut writer = BoundedWriter {
        bytes: Vec::new(),
        cap,
        first_failure: None,
    };
    // One existing tagged serialization pass. Writer evidence survives a
    // wrapped/replaced error or a serializer which ignores a refused write.
    let serialized = midnight_serialize::tagged_serialize(value, &mut writer);
    let failure = writer.first_failure.map(|failure| {
        (
            failure.reason,
            failure.accepted_bytes,
            Some(failure.failed_write_bytes),
            failure.attempted_minimum,
        )
    });
    if let Some((reason, accepted_bytes, failed_write_bytes, attempted_minimum)) =
        failure.or_else(|| {
            serialized.err().map(|_| {
                (
                    KeyExportFailureReason::Serialization,
                    writer.bytes.len(),
                    None,
                    None,
                )
            })
        })
    {
        return Err(
            refusal("generated-key-size-or-encode").with_key_export(KeyExportDiagnostic {
                action: action.into(),
                key_kind,
                discovered_degree: degree,
                byte_cap: cap,
                accepted_bytes,
                failed_write_bytes,
                attempted_minimum,
                reason,
            }),
        );
    }
    Ok(writer.bytes)
}
// Declared source identity, not a certificate for the complete Rust build or SRS.
// mori-backend non-test modules are named by path from an inventory. cfg(test)
// modules, mori-core files outside this list, and SDK/registry sources are not covered.
pub(crate) fn source_closure() -> String {
    codec::hash(
        concat!(
            include_str!("proving.rs"),
            include_str!("relation/mod.rs"),
            include_str!("relation/compiler.rs"),
            include_str!("relation/arithmetic.rs"),
            include_str!("relation/event.rs"),
            include_str!("relation/process.rs"),
            include_str!("signed/mod.rs"),
            include_str!("signed/nonce.rs"),
            include_str!("signed/schnorr.rs"),
            include_str!("signed/constraints.rs"),
            include_str!("signed/context.rs"),
            include_str!("signed/deployment.rs"),
            include_str!("signed/preparation.rs"),
            include_str!("../third-party/jubjub-fens/NOTICE.md"),
            include_str!("../third-party/jubjub-fens/LICENSE-MIT"),
            include_str!("../third-party/jubjub-fens/LICENSE-APACHE"),
            include_str!("prepare.rs"),
            include_str!("ops.rs"),
            include_str!("layout.rs"),
            include_str!("profile.rs"),
            include_str!("custody.rs"),
            include_str!("codec.rs"),
            include_str!("lib.rs"),
            include_str!("parameters.rs"),
            include_str!("local_deployment.rs"),
            include_str!("static_compile.rs"),
            include_str!("call_artifacts.rs"),
            include_str!("sherlock_literal.rs"),
            include_str!("../../mori/Cargo.toml"),
            include_str!("../../mori/src/cli.rs"),
            include_str!("../../mori/src/commands/mod.rs"),
            include_str!("../../mori/src/commands/call_io.rs"),
            include_str!("../../mori/src/commands/compile_call.rs"),
            include_str!("../../mori/src/commands/signed_call_io.rs"),
            include_str!("../../mori/src/commands/keygen_signed_call.rs"),
            include_str!("../../mori/src/commands/prepare_signed_call.rs"),
            include_str!("../../mori/src/commands/prove_signed_call.rs"),
            include_str!("../../mori/src/commands/verify_signed_call.rs"),
            include_str!("../../mori/src/commands/keygen_call.rs"),
            include_str!("../../mori/src/commands/prove_call.rs"),
            include_str!("../../mori/src/commands/verify_call.rs"),
            include_str!("../Cargo.toml"),
            include_str!("../../../Cargo.toml"),
            include_str!("../../../Cargo.lock"),
            include_str!("../../mori-core/Cargo.toml"),
            include_str!("../../mori-core/src/lib.rs"),
            include_str!("../../mori-core/src/parser.rs"),
            include_str!("../../mori-core/src/ir.rs"),
            include_str!("../../mori-core/src/runtime.rs"),
            include_str!("../../mori-core/src/logical_cost.rs"),
            include_str!("../../mori-core/src/source_sponsor.rs"),
            include_str!("../../mori-core/src/json.rs"),
            include_str!("../../mori-core/src/intent.rs"),
            include_str!("../../mori-core/src/policy.rs"),
            include_str!("../../mori-core/src/replay.rs"),
            include_str!("../../mori-lexer/Cargo.toml"),
            include_str!("../../mori-lexer/src/lib.rs"),
            include_str!("../../mori-lexer/src/scan.rs"),
            include_str!("../../mori-lexer/src/token.rs"),
            include_str!("../../mori-lexer/src/string.rs"),
            include_str!("../../mori-lexer/src/number.rs"),
            include_str!("../../mori-lexer/src/diagnostics.rs"),
            include_str!("../../mori-core/src/stage/admit.rs"),
            include_str!("../../mori-core/src/stage/cost.rs"),
            include_str!("../../mori-core/src/stage/decode.rs"),
            include_str!("../../mori-core/src/stage/digest.rs"),
            include_str!("../../mori-core/src/stage/mod.rs"),
            include_str!("../../mori-core/src/stage/plan.rs"),
            include_str!("../../mori-core/src/stage/profile.rs"),
            include_str!("../../mori-core/src/stage/validate.rs"),
            include_str!("../../../models/sherlock/literal-step/MAPPING.md"),
            include_str!("../../mori-backend/Cargo.toml"),
            include_str!("../../mori-backend/src/lib.rs"),
            include_str!("../../mori-backend/src/authorization.rs"),
            include_str!("../../mori-backend/src/liabilities.rs"),
            include_str!("../../mori-backend/src/parameters.rs"),
            include_str!("../../mori-backend/src/proving.rs"),
            include_str!("../../mori-backend/src/replay.rs"),
            include_str!("../../mori-backend/src/schnorr.rs"),
            include_str!("../../mori-backend/src/signed_relation.rs"),
            include_str!("../../mori-backend/src/source_projection.rs"),
            include_str!("../../mori-backend/src/stage/mod.rs"),
            include_str!("../../mori-backend/src/stage/artifact.rs"),
            include_str!("../../mori-backend/src/stage/context.rs"),
            include_str!("../../mori-backend/src/stage/frame.rs"),
            include_str!("../../mori-backend/src/stage/lower.rs"),
            include_str!("../../mori-backend/src/sherlock_machine/mod.rs"),
            include_str!("../../mori-backend/src/sherlock_machine/fetch.rs"),
            include_str!("../../mori-backend/src/sherlock_machine/enabled_fetch.rs"),
            include_str!("../../mori-backend/src/sherlock_machine/literal_step.rs"),
            include_str!("../../mori-backend/src/sherlock_machine/literal_bridge.rs")
        )
        .as_bytes(),
    )
}
#[derive(Clone)]
struct OfflineCallParams {
    degree: u8,
    params: ParamsProver,
}
impl ParamsProverProvider for OfflineCallParams {
    async fn get_params(&self, degree: u8) -> io::Result<ParamsProver> {
        if degree != self.degree {
            return Err(io::Error::other("unavailable immutable parameter degree"));
        }
        Ok(self.params.clone())
    }
}
struct PublicKeySnapshot {
    ir: Vec<u8>,
    manifest: Vec<u8>,
    vk: Vec<u8>,
    closure: String,
}
/// In-memory public derivation capability, created only by source-recompiled SDK
/// keygen. This contains no PK or prover parameters. Exported bytes are public
/// data, not a durable trusted-import certificate. No Deserialize/From exists.
/// The capability records source keygen with supplied initialized parameters;
/// it does not authenticate SRS provenance, trusted setup or correct SDK keygen.
#[derive(Clone)]
/// ```compile_fail
/// let key = mori_ledger::proving::PublicDerivedCallKey { snapshot: Default::default() };
/// ```
pub struct PublicDerivedCallKey {
    snapshot: Arc<PublicKeySnapshot>,
}
impl PublicDerivedCallKey {
    pub fn ir_bytes(&self) -> &[u8] {
        &self.snapshot.ir
    }
    pub fn verifier_key_bytes(&self) -> &[u8] {
        &self.snapshot.vk
    }
    pub fn source_manifest_bytes(&self) -> &[u8] {
        &self.snapshot.manifest
    }
    /// Construct a supplied local operation object. This installs nothing and
    /// makes no claim about a ledger's authorization policy or authenticated state.
    pub fn local_operation(&self) -> Result<ContractOperation> {
        Ok(ContractOperation::new(
            Some(codec::decode_artifact(&self.snapshot.vk)?),
            Some(Sp::new(IrBuf(self.snapshot.ir.clone()))),
        ))
    }
    pub(crate) fn admit(&self, artifact: &relation::Artifact) -> Result<()> {
        if self.snapshot.ir != artifact.ir_bytes()?
            || self.snapshot.manifest != artifact.manifest_bytes()?
            || self.snapshot.closure != source_closure()
        {
            return Err(refusal("derived-source-identity"));
        }
        Ok(())
    }
    pub(crate) fn admit_operation(&self, operation: &ContractOperation) -> Result<()> {
        if operation.v2.is_some()
            || operation.v3.as_ref().map(codec::encode).transpose()?
                != Some(self.snapshot.vk.clone())
            || operation.ir.as_deref().map(|ir| ir.0.as_slice())
                != Some(self.snapshot.ir.as_slice())
        {
            return Err(refusal("supplied-operation-key-or-ir"));
        }
        Ok(())
    }
}
struct KeySnapshot {
    action: String,
    material: ProvingKeyMaterial,
}
impl KeySnapshot {
    fn resolve(&self, location: &KeyLocation) -> Option<ProvingKeyMaterial> {
        (location.0.as_ref() == self.action).then(|| self.material.clone())
    }
}
/// Source-derived in-memory capability; arbitrary imported bytes cannot create it.
/// Constructing it runs actual SDK cost selection and key generation. Cloning
/// shares the immutable source/key/parameter snapshot for later calls, while
/// each proving call receives a fresh RNG and consumes its own raw owner.
#[derive(Clone)]
pub struct DerivedCallMaterial {
    artifact: Arc<relation::Artifact>,
    keys: Arc<KeySnapshot>,
    params: OfflineCallParams,
    public: PublicDerivedCallKey,
}
impl DerivedCallMaterial {
    pub(crate) fn prover_key_bytes(&self) -> &[u8] {
        &self.keys.material.prover_key
    }
    pub fn public_key(&self) -> PublicDerivedCallKey {
        self.public.clone()
    }
}
/// Native work: source-recompile first, then derive the exact SDK PK/VK from that
/// circuit and already initialized immutable parameters. This performs no SRS read
/// or download. The caller must establish SRS provenance and native resource authority.
pub async fn generate_call_keys(
    source: &str,
    program: &Program,
    deployment: &Deployment,
    action: &str,
    params: ParamsProver,
    limits: KeyGenerationLimits,
) -> Result<DerivedCallMaterial> {
    let artifact = relation::compile(source, program, deployment, action)?;
    if limits.max_prover_key_bytes == 0
        || limits.max_verifier_key_bytes == 0
        || limits.max_verifier_key_bytes > codec::OUTPUT_BYTES
    {
        return Err(refusal("key-resource-limits"));
    }
    let parameter_degree =
        u8::try_from(params.as_ref().max_k()).map_err(|_| refusal("parameter-degree"))?;
    if parameter_degree > limits.max_degree {
        return Err(refusal("parameter-degree-limit"));
    }
    // k() is native cost work, never ordinary metadata or an ordinary test call.
    let degree = artifact.ir.k();
    if degree > limits.max_degree || degree != parameter_degree {
        return Err(refusal("exact-degree-unavailable"));
    }
    derive_artifact_material(artifact, action, params, degree, limits).await
}
pub(crate) async fn derive_artifact_material(
    artifact: relation::Artifact,
    action: &str,
    params: ParamsProver,
    degree: u8,
    limits: KeyGenerationLimits,
) -> Result<DerivedCallMaterial> {
    let params = OfflineCallParams { degree, params };
    let (pk, vk) = artifact
        .ir
        .keygen(&params)
        .await
        .map_err(|_| refusal("sdk-keygen"))?;
    let pk = encode_key(
        &pk,
        action,
        KeyExportKind::Prover,
        degree,
        limits.max_prover_key_bytes,
    )?;
    let vk = encode_key(
        &vk,
        action,
        KeyExportKind::Verifier,
        degree,
        limits.max_verifier_key_bytes,
    )?;
    // The SDK lazy wrapper enforces its fixed 50,000 inner VK byte cap.
    let _: midnight_transient_crypto::proofs::VerifierKey = codec::decode_artifact(&vk)?;
    let ir = artifact.ir_bytes()?;
    let public = PublicDerivedCallKey {
        snapshot: Arc::new(PublicKeySnapshot {
            ir: ir.clone(),
            manifest: artifact.manifest_bytes()?,
            vk: vk.clone(),
            closure: source_closure(),
        }),
    };
    let keys = Arc::new(KeySnapshot {
        action: action.into(),
        material: ProvingKeyMaterial {
            prover_key: pk,
            verifier_key: vk,
            ir_source: ir,
        },
    });
    Ok(DerivedCallMaterial {
        artifact: Arc::new(artifact),
        keys,
        params,
        public,
    })
}
pub(crate) async fn derive_public_artifact(
    artifact: &relation::Artifact,
    params: &crate::parameters::PinnedCallParameters,
    degree: u8,
    cap: usize,
) -> Result<PublicDerivedCallKey> {
    let vk = artifact
        .ir
        .keygen_vk(params)
        .await
        .map_err(|_| refusal("sdk-keygen-vk"))?;
    let vk = encode_key(
        &vk,
        &artifact.manifest.action,
        KeyExportKind::Verifier,
        degree,
        cap,
    )?;
    let _: midnight_transient_crypto::proofs::VerifierKey = codec::decode_artifact(&vk)?;
    Ok(PublicDerivedCallKey {
        snapshot: Arc::new(PublicKeySnapshot {
            ir: artifact.ir_bytes()?,
            manifest: artifact.manifest_bytes()?,
            vk,
            closure: source_closure(),
        }),
    })
}
#[derive(Clone)]
pub struct SourceCallResolver {
    keys: Arc<KeySnapshot>,
}
impl Resolver for SourceCallResolver {
    async fn resolve_key(&self, location: KeyLocation) -> io::Result<Option<ProvingKeyMaterial>> {
        Ok(self.keys.resolve(&location))
    }
}
pub struct SourceCallProver {
    artifact: Arc<relation::Artifact>,
    resolver: SourceCallResolver,
    params: OfflineCallParams,
    rng: StdRng,
}
impl SourceCallProver {
    /// Cryptographic RNG supplied by the caller; split derives independent streams.
    pub fn new(material: DerivedCallMaterial, rng: StdRng) -> Self {
        Self {
            artifact: material.artifact,
            resolver: SourceCallResolver {
                keys: material.keys,
            },
            params: material.params,
            rng,
        }
    }
}
fn bound_preimage(
    artifact: &relation::Artifact,
    preimage: &ProofPreimage,
    binding: Option<Fr>,
) -> Result<(ProofPreimage, Vec<Option<usize>>)> {
    let activity = relation::check(artifact, preimage)?;
    let mut bound = preimage.clone();
    bound.binding_input = binding.ok_or(refusal("final-binding-required"))?;
    if relation::check(artifact, &bound)? != activity {
        return Err(refusal("binding-changed-activity"));
    }
    Ok((bound, activity))
}
fn fork_rng(rng: &mut StdRng) -> StdRng {
    StdRng::from_rng(rng).expect("StdRng is an infallible seed source")
}
impl ProvingProvider for SourceCallProver {
    async fn check(&self, preimage: &ProofPreimage) -> anyhow::Result<Vec<Option<usize>>> {
        if preimage.key_location.0.as_ref() != self.resolver.keys.action {
            return Err(refusal("snapshot-key-location").into());
        }
        Ok(relation::check(&self.artifact, preimage)?)
    }
    async fn prove(self, preimage: &ProofPreimage, binding: Option<Fr>) -> anyhow::Result<Proof> {
        if preimage.key_location.0.as_ref() != self.resolver.keys.action {
            return Err(refusal("snapshot-key-location").into());
        }
        let (bound, activity) = bound_preimage(&self.artifact, preimage, binding)?;
        let (proof, actual_activity) = bound
            .prove::<IrSource>(self.rng, &self.params, &self.resolver)
            .await?;
        if actual_activity != activity {
            return Err(refusal("proving-activity-mismatch").into());
        }
        Ok(proof)
    }
    fn split(&mut self) -> Self {
        // StdRng is an infallible seed source, unlike an OS/network source here.
        let rng = fork_rng(&mut self.rng);
        Self {
            artifact: self.artifact.clone(),
            resolver: self.resolver.clone(),
            params: self.params.clone(),
            rng,
        }
    }
    fn resolver(&self) -> &impl Resolver {
        &self.resolver
    }
}
fn expected_intent(prepared: &PreparedNativeCall) -> Result<RawIntent> {
    let native = &prepared.bundle().native;
    if native.intent.actions.len() != 1 || native.intent.calls().count() != 1 {
        return Err(refusal("single-raw-call"));
    }
    let mut expected = native.intent.clone();
    let mut call = expected
        .calls()
        .next()
        .ok_or(refusal("missing-raw-call"))?
        .clone();
    call.guaranteed_transcript = Some(Sp::new(
        native
            .processed_transcript
            .clone()
            .ok_or(refusal("missing-checked-projection"))?,
    ));
    expected.actions = vec![ContractAction::<ProofPreimageMarker, InMemoryDB>::Call(
        Sp::new(call),
    )]
    .into();
    Ok(expected)
}
#[derive(Debug, Serialize)]
pub struct CallVerification {
    pub scope: &'static str,
    pub open_predicates: Vec<&'static str>,
    pub ir_hash: String,
    pub verifier_key_hash: String,
    pub public_inputs: Vec<String>,
    pub final_intent_hash_hex: String,
    pub prospective_outputs_hex: Vec<String>,
    pub charged_successor_hex: String,
    pub measured_gas_hex: String,
}
fn validate_final_envelope<P: ProofKind<InMemoryDB>>(
    prepared: &PreparedNativeCall,
    candidate: &Intent<(), P, PedersenRandomness, InMemoryDB>,
) -> Result<()> {
    let expected = expected_intent(prepared)?;
    // This whole-intent equality covers offers, TTL, randomness, call order and
    // extra deploy/maintain actions; calls().count() alone would miss them.
    if candidate.binding_commitment != expected.binding_commitment
        || candidate.erase_proofs().erase_signatures() != expected.erase_proofs().erase_signatures()
    {
        return Err(refusal("final-intent-identity"));
    }
    Ok(())
}
fn validate_final_intent(
    prepared: &PreparedNativeCall,
    candidate: &ProvenIntent,
) -> Result<CallVerification> {
    validate_final_envelope(prepared, candidate)?;
    let call = candidate
        .calls()
        .next()
        .ok_or(refusal("missing-final-call"))?;
    if !matches!(call.proof, ProofVersioned::V3(_)) {
        return Err(refusal("current-proof-version-required"));
    }
    let native = &prepared.bundle().native;
    let transcript = call
        .guaranteed_transcript
        .as_deref()
        .ok_or(refusal("missing-guaranteed-transcript"))?;
    let actual = native
        .context
        .run_transcript(transcript, &native.params.cost_model.runtime_cost_model)
        .map_err(|_| refusal("final-vm-refusal"))?;
    crate::prepare::validate_query(&native.lowered, &actual)?;
    let readback = &prepared.bundle().readback;
    let projection = readback
        .relation
        .as_ref()
        .ok_or(refusal("missing-checked-projection"))?;
    if codec::hex(&crate::call_artifacts::encode_bounded(
        &actual.context.state,
        native.output_bytes,
    )?) != readback.charged_successor_hex
        || codec::native_hex(&actual.gas_cost)? != projection.processed_measured_gas_hex
        || codec::native_hex(&transcript.gas)? != projection.processed_declared_gas_hex
        || !transcript.gas.within_bounds(&native.gas_cap)
        || !actual.gas_cost.within_bounds(&transcript.gas)
        || actual
            .gas_cost
            .bytes_written
            .saturating_sub(actual.gas_cost.bytes_deleted)
            > native.params.limits.block_limits.bytes_written
    {
        return Err(refusal("final-charged-state-or-gas"));
    }
    let parent: Pedersen = candidate.binding_commitment.into();
    let pi = call.public_inputs(parent);
    let pi_hex: Vec<_> = pi.iter().map(|f| codec::hex(&f.as_le_bytes())).collect();
    if pi_hex != projection.public_inputs {
        return Err(refusal("final-public-inputs"));
    }
    let (intent_hash, outputs) = reconstruct_outputs(candidate, native.segment)?;
    Ok(CallVerification {
        scope: "local-unsigned-call-proof-supplied-parameters-and-operation-unqualified",
        open_predicates: vec![
            "independently trusted SRS provenance and setup",
            "independent exact SDK keygen correspondence",
            "full source-constraint correspondence theorem",
            "authenticated installed operation and predecessor",
            "signatures and intent authority",
            "replay protection, fees, application and settlement",
            "recursive history/private composition",
        ],
        ir_hash: prepared.artifact().manifest.ir_hash.clone(),
        verifier_key_hash: String::new(),
        public_inputs: pi_hex,
        final_intent_hash_hex: codec::native_hex(&intent_hash)?,
        prospective_outputs_hex: outputs,
        charged_successor_hex: codec::hex(&crate::call_artifacts::encode_bounded(
            &actual.context.state,
            native.output_bytes,
        )?),
        measured_gas_hex: codec::native_hex(&actual.gas_cost)?,
    })
}
fn reconstruct_outputs<P: ProofKind<InMemoryDB>>(
    intent: &Intent<(), P, PedersenRandomness, InMemoryDB>,
    segment: u16,
) -> Result<(IntentHash, Vec<String>)> {
    let intent_hash = intent
        .erase_proofs()
        .erase_signatures()
        .intent_hash(segment);
    let outputs = intent
        .guaranteed_unshielded_offer
        .as_ref()
        .into_iter()
        .flat_map(|offer| offer.outputs.iter())
        .enumerate()
        .map(|(i, output)| {
            let output_no = u32::try_from(i).map_err(|_| refusal("output-count"))?;
            codec::native_hex(&Utxo {
                value: output.value,
                owner: output.owner,
                type_: output.type_,
                intent_hash,
                output_no,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok((intent_hash, outputs))
}
/// Native verification: independently checked source/preparation and a public
/// derivation capability, with an exact supplied current operation. No PK or
/// prover parameters are held by the public key. Required dependency feature
/// proof-verifying is explicit; ProofMarker + Real calls the ledger verifier.
pub fn verify_proven_call(
    prepared: &PreparedNativeCall,
    key: &PublicDerivedCallKey,
    operation: &ContractOperation,
    candidate: &ProvenIntent,
) -> Result<CallVerification> {
    key.admit(prepared.artifact())?;
    key.admit_operation(operation)?;
    let mut readback = validate_final_intent(prepared, candidate)?;
    let call: &ContractCall<ProofMarker, InMemoryDB> = candidate
        .calls()
        .next()
        .ok_or(refusal("missing-final-call"))?;
    let parent: Pedersen = candidate.binding_commitment.into();
    <ProofMarker as ProofKind<InMemoryDB>>::proof_verify(
        operation,
        &call.proof,
        call.public_inputs(parent),
        call,
        ProofVerificationMode::Real,
    )
    .map_err(|_| refusal("ledger-real-proof-verification"))?;
    readback.verifier_key_hash = codec::hash(key.verifier_key_bytes());
    Ok(readback)
}
pub struct ProvenNativeCall {
    intent: ProvenIntent,
    verification: CallVerification,
}
impl ProvenNativeCall {
    pub fn intent(&self) -> &ProvenIntent {
        &self.intent
    }
    pub fn verification(&self) -> &CallVerification {
        &self.verification
    }
}
/// Native proving consumer. SDK gets the raw intent exactly once; verification
/// uses the actual final SDK statement and exact supplied derived operation.
pub async fn prove_prepared_call(
    prepared: PreparedNativeCall,
    material: DerivedCallMaterial,
    operation: &ContractOperation,
    rng: StdRng,
) -> Result<ProvenNativeCall> {
    let key = material.public_key();
    key.admit(prepared.artifact())?;
    key.admit_operation(operation)?;
    let native = &prepared.bundle().native;
    let (_, intent) = prepared
        .raw_intent()
        .prove(
            native.segment,
            SourceCallProver::new(material, rng),
            &native.params.cost_model.runtime_cost_model,
        )
        .await
        .map_err(|_| refusal("sdk-intent-proving"))?;
    let verification = verify_proven_call(&prepared, &key, operation, &intent)?;
    Ok(ProvenNativeCall {
        intent,
        verification,
    })
}

#[cfg(test)]
#[path = "../tests/support/mod.rs"]
mod test_support;

#[cfg(test)]
mod tests {
    use super::test_support as support;
    use super::*;
    use rand::RngCore;
    use std::borrow::Cow;

    fn prepared() -> PreparedNativeCall {
        let f = support::payment();
        crate::prepare::prepare_native_call(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters,
            &f.request,
        )
        .unwrap()
    }
    #[test]
    fn binding_overwrite_is_required_and_checked_activity_is_stable() {
        let prepared = prepared();
        let call = prepared.raw_intent().calls().next().unwrap();
        let midnight_ledger::structure::ProofPreimageVersioned::V2(raw) = &call.proof else {
            panic!("raw version")
        };
        let expected = relation::check(prepared.artifact(), raw).unwrap();
        assert_eq!(
            bound_preimage(prepared.artifact(), raw, None)
                .unwrap_err()
                .code,
            "final-binding-required"
        );
        let (bound, activity) =
            bound_preimage(prepared.artifact(), raw, Some(29u64.into())).unwrap();
        assert_eq!(bound.binding_input, 29u64.into());
        assert_eq!(raw.binding_input, 0u64.into());
        assert_eq!(activity, expected);
        let mut wrong = (**raw).clone();
        wrong.key_location = KeyLocation(Cow::Borrowed("midnight/zswap/spend"));
        assert!(bound_preimage(prepared.artifact(), &wrong, Some(29u64.into())).is_err());
    }
    #[test]
    fn resolver_lookup_snapshot_is_immutable_and_never_dispatches_builtins() {
        // Deliberately opaque bytes test lookup/copy semantics only. No derived
        // capability, key decode, provider.prove or cryptographic success exists.
        let keys = Arc::new(KeySnapshot {
            action: "settle".into(),
            material: ProvingKeyMaterial {
                prover_key: vec![1, 2],
                verifier_key: vec![3, 4],
                ir_source: vec![5, 6],
            },
        });
        let resolver = SourceCallResolver { keys };
        let location = KeyLocation(Cow::Borrowed("settle"));
        let mut copy = futures::executor::block_on(resolver.resolve_key(location.clone()))
            .unwrap()
            .unwrap();
        copy.ir_source[0] = 99;
        copy.prover_key.clear();
        copy.verifier_key.push(9);
        let actual = futures::executor::block_on(resolver.clone().resolve_key(location))
            .unwrap()
            .unwrap();
        assert_eq!(actual.ir_source, vec![5, 6]);
        assert_eq!(actual.prover_key, vec![1, 2]);
        assert_eq!(actual.verifier_key, vec![3, 4]);
        for unknown in [
            "other",
            "midnight/zswap/spend",
            "midnight/zswap/output",
            "midnight/dust/spend",
        ] {
            assert!(
                futures::executor::block_on(
                    resolver.resolve_key(KeyLocation(Cow::Borrowed(unknown)))
                )
                .unwrap()
                .is_none()
            );
        }
    }
    #[test]
    fn key_export_ignored_writer_failure_cannot_accept_truncated_encoding() {
        struct IgnoresFailure;
        impl Tagged for IgnoresFailure {
            fn tag() -> std::borrow::Cow<'static, str> {
                "diagnostic-fixture".into()
            }
            fn tag_unique_factor() -> String {
                "diagnostic-fixture".into()
            }
        }
        impl Serializable for IgnoresFailure {
            fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
                let _ = writer.write_all(&[0; 4]);
                Ok(())
            }
            fn serialized_size(&self) -> usize {
                panic!("key export must not add a size pass")
            }
        }
        let header_len = b"midnight:diagnostic-fixture:".len();
        assert!(
            encode_key(
                &IgnoresFailure,
                "settle",
                KeyExportKind::Prover,
                0,
                header_len
            )
            .is_err()
        );
    }
    #[test]
    fn bounded_generated_serialization_refuses_before_appending_over_limit() {
        let mut writer = BoundedWriter {
            bytes: vec![],
            cap: 3,
            first_failure: None,
        };
        writer.write_all(&[1, 2]).unwrap();
        assert!(writer.write_all(&[3, 4]).is_err());
        assert_eq!(writer.bytes, vec![1, 2]);
        assert!(writer.write_all(&[3]).is_err());
        assert_eq!(writer.bytes, vec![1, 2]);
    }
    const FIXTURE_HEADER: &[u8] = b"midnight:diagnostic-fixture:";
    #[derive(Clone, Copy)]
    enum EncodingBehavior {
        Success,
        FailBefore,
        FailAfterPrefix,
        Misleading,
        Wrapped,
        Ignored,
    }
    struct EncodingFixture {
        behavior: EncodingBehavior,
        calls: std::cell::Cell<usize>,
    }
    impl Tagged for EncodingFixture {
        fn tag() -> std::borrow::Cow<'static, str> {
            "diagnostic-fixture".into()
        }
        fn tag_unique_factor() -> String {
            "diagnostic-fixture".into()
        }
    }
    impl Serializable for EncodingFixture {
        fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
            self.calls.set(self.calls.get() + 1);
            match self.behavior {
                EncodingBehavior::FailBefore => Err(io::Error::other("sensitive fixture message")),
                EncodingBehavior::Misleading => Err(io::Error::other(
                    "generated key exceeds caller resource limit",
                )),
                EncodingBehavior::FailAfterPrefix => {
                    writer.write_all(&[1, 2])?;
                    Err(io::Error::other("sensitive fixture message"))
                }
                EncodingBehavior::Wrapped | EncodingBehavior::Ignored => {
                    writer.write_all(&[1, 2])?;
                    let rejected = writer.write_all(&[3, 4]);
                    assert!(rejected.is_err());
                    assert!(writer.write_all(&[5]).is_err());
                    if matches!(self.behavior, EncodingBehavior::Wrapped) {
                        Err(io::Error::other("replacement serializer error"))
                    } else {
                        Ok(())
                    }
                }
                EncodingBehavior::Success => writer.write_all(&[1, 2]),
            }
        }
        fn serialized_size(&self) -> usize {
            panic!("key export must not add a size pass")
        }
    }
    fn encoding_fixture(behavior: EncodingBehavior) -> EncodingFixture {
        EncodingFixture {
            behavior,
            calls: std::cell::Cell::new(0),
        }
    }
    #[test]
    fn key_export_writer_boundaries_record_first_refusal_without_append() {
        let mut writer = BoundedWriter {
            bytes: vec![],
            cap: 3,
            first_failure: None,
        };
        assert_eq!(writer.write(&[]).unwrap(), 0);
        writer.write_all(&[1, 2, 3]).unwrap();
        assert!(writer.write_all(&[4]).is_err());
        let failure = writer.first_failure.as_ref().unwrap();
        assert_eq!(failure.reason, KeyExportFailureReason::ByteCapExceeded);
        assert_eq!(
            (
                failure.accepted_bytes,
                failure.failed_write_bytes,
                failure.attempted_minimum
            ),
            (3, 1, Some(4))
        );
        assert!(writer.write(&[]).is_err());
        assert_eq!(writer.bytes, vec![1, 2, 3]);
        let mut zero = BoundedWriter {
            bytes: vec![],
            cap: 0,
            first_failure: None,
        };
        assert_eq!(zero.write(&[]).unwrap(), 0);
        assert!(zero.write_all(&[1]).is_err());
        assert_eq!(zero.bytes, Vec::<u8>::new());
        assert_eq!(zero.first_failure.unwrap().attempted_minimum, Some(1));
    }
    #[test]
    fn key_export_checked_count_overflow_has_no_wrapped_minimum() {
        assert_eq!(
            check_key_write(usize::MAX - 1, 2, usize::MAX),
            Err(KeyExportFailureReason::ByteCountOverflow)
        );
        assert_eq!((usize::MAX - 1).checked_add(2), None);
        assert_eq!(
            check_key_write(2, 2, 3),
            Err(KeyExportFailureReason::ByteCapExceeded)
        );
        assert_eq!(check_key_write(2, 1, 3), Ok(3));
    }
    #[test]
    fn key_export_serializer_failures_have_exact_prefix_count_and_no_raw_message() {
        for (behavior, suffix) in [
            (EncodingBehavior::FailBefore, 0),
            (EncodingBehavior::FailAfterPrefix, 2),
            (EncodingBehavior::Misleading, 0),
        ] {
            let fixture = encoding_fixture(behavior);
            let error = encode_key(
                &fixture,
                "settle",
                KeyExportKind::Verifier,
                0,
                FIXTURE_HEADER.len() + 4,
            )
            .unwrap_err();
            let diagnostic = error.key_export().unwrap();
            assert_eq!(diagnostic.reason, KeyExportFailureReason::Serialization);
            assert_eq!(diagnostic.accepted_bytes, FIXTURE_HEADER.len() + suffix);
            assert_eq!(diagnostic.failed_write_bytes, None);
            assert_eq!(diagnostic.attempted_minimum, None);
            assert_eq!(fixture.calls.get(), 1);
            assert_eq!(
                error.to_string(),
                "mori::ledger::native-proving: generated-key-size-or-encode"
            );
            assert!(!format!("{error:?}").contains("sensitive fixture message"));
        }
    }
    #[test]
    fn key_export_writer_failure_takes_precedence_over_wrapped_or_ignored_serializer() {
        for behavior in [EncodingBehavior::Wrapped, EncodingBehavior::Ignored] {
            for kind in [KeyExportKind::Prover, KeyExportKind::Verifier] {
                let fixture = encoding_fixture(behavior);
                let cap = FIXTURE_HEADER.len() + 3;
                let error = encode_key(&fixture, "actual-action", kind, 19, cap).unwrap_err();
                assert_eq!(
                    error.key_export(),
                    Some(&KeyExportDiagnostic {
                        action: "actual-action".into(),
                        key_kind: kind,
                        discovered_degree: 19,
                        byte_cap: cap,
                        accepted_bytes: FIXTURE_HEADER.len() + 2,
                        failed_write_bytes: Some(2),
                        attempted_minimum: Some(FIXTURE_HEADER.len() + 4),
                        reason: KeyExportFailureReason::ByteCapExceeded,
                    })
                );
                assert_eq!(fixture.calls.get(), 1);
            }
        }
    }
    #[test]
    fn key_export_success_is_exact_tagged_one_pass_and_header_cap_is_counted() {
        let fixture = encoding_fixture(EncodingBehavior::Success);
        let mut expected = FIXTURE_HEADER.to_vec();
        expected.extend_from_slice(&[1, 2]);
        assert_eq!(
            encode_key(&fixture, "settle", KeyExportKind::Prover, 0, expected.len()).unwrap(),
            expected
        );
        assert_eq!(fixture.calls.get(), 1);
        let error = encode_key(
            &fixture,
            "settle",
            KeyExportKind::Verifier,
            0,
            expected.len() - 1,
        )
        .unwrap_err();
        assert_eq!(
            error.key_export().unwrap().reason,
            KeyExportFailureReason::ByteCapExceeded
        );
        assert_eq!(
            error.key_export().unwrap().attempted_minimum,
            Some(expected.len())
        );
        let fixture = encoding_fixture(EncodingBehavior::Success);
        let error = encode_key(&fixture, "settle", KeyExportKind::Verifier, 0, 0).unwrap_err();
        assert_eq!(fixture.calls.get(), 0);
        assert_eq!(error.key_export().unwrap().accepted_bytes, 0);
        assert_eq!(
            error.key_export().unwrap().reason,
            KeyExportFailureReason::ByteCapExceeded
        );
    }
    #[test]
    fn key_export_v2_effective_pk_cap_reaches_the_same_one_pass_writer() {
        let mut limits = crate::profile::NativeWorkLimits {
            profile: "moriarty-native-call-work-limits/1".into(),
            max_degree: 19,
            max_actions: 1,
            max_srs_bytes: 1,
            max_prover_key_bytes: (FIXTURE_HEADER.len() + 2) as u64,
            max_verifier_key_bytes: 1,
            max_input_bytes: 1,
            max_leaf_bytes: (FIXTURE_HEADER.len() + 1) as u64,
            max_artifact_bytes: 128,
            max_operations: 1,
            max_public_inputs: 1,
        };
        let fixture = encoding_fixture(EncodingBehavior::Success);
        let error = encode_key(
            &fixture,
            "selected",
            KeyExportKind::Prover,
            0,
            limits.prover_key_cap().unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            error.key_export().unwrap().byte_cap,
            FIXTURE_HEADER.len() + 1
        );
        assert_eq!(
            error.key_export().unwrap().reason,
            KeyExportFailureReason::ByteCapExceeded
        );
        limits.profile = "moriarty-native-call-work-limits/2".into();
        assert!(
            encode_key(
                &fixture,
                "selected",
                KeyExportKind::Prover,
                0,
                limits.prover_key_cap().unwrap()
            )
            .is_ok()
        );
        assert_eq!(fixture.calls.get(), 2); // One payload call per existing adapter invocation.
    }
    #[test]
    fn rng_forks_consume_distinct_cryptographic_streams() {
        let mut parent = StdRng::seed_from_u64(17);
        let mut first = fork_rng(&mut parent);
        let mut second = fork_rng(&mut parent);
        let mut a = [0u8; 64];
        let mut b = [0u8; 64];
        first.fill_bytes(&mut a);
        second.fill_bytes(&mut b);
        assert_ne!(a, b);
    }
    #[test]
    fn final_output_ids_follow_processed_intent_instead_of_old_prospective_hash() {
        let prepared = prepared();
        let mut inspected = prepared.raw_intent().clone();
        let call: ContractCall<ProofPreimageMarker, InMemoryDB> =
            codec::decode_artifact(&prepared.bundle().leaves["processed-call.bin"]).unwrap();
        inspected.actions = vec![ContractAction::Call(Sp::new(call))].into();
        let (new_hash, encoded) = reconstruct_outputs(&inspected, 1).unwrap();
        let old_hash = prepared
            .raw_intent()
            .erase_proofs()
            .erase_signatures()
            .intent_hash(1);
        assert_ne!(old_hash, new_hash);
        assert_eq!(
            new_hash,
            inspected.erase_proofs().erase_signatures().intent_hash(1)
        );
        let outputs: Vec<Utxo> = encoded
            .iter()
            .map(|v| codec::from_hex(v).unwrap())
            .collect();
        assert_eq!(outputs.iter().map(|o| o.value).sum::<u128>(), 100);
        let mut values = outputs.iter().map(|o| o.value).collect::<Vec<_>>();
        values.sort();
        // Independent authored payment fixture charges fee 2: 100 - 70 - 2 = 28.
        assert_eq!(values, vec![2, 28, 70]);
        for (i, output) in outputs.iter().enumerate() {
            assert_eq!(output.intent_hash, new_hash);
            assert_eq!(output.output_no, i as u32);
        }
        let old: Vec<Utxo> = prepared
            .bundle()
            .readback
            .prospective_outputs_hex
            .iter()
            .map(|v| codec::from_hex(v).unwrap())
            .collect();
        assert!(old.iter().all(|o| o.intent_hash == old_hash));
    }
    #[test]
    fn whole_intent_envelope_refuses_extra_actions_and_changed_public_fields() {
        let prepared = prepared();
        let mut inspected = prepared.raw_intent().clone();
        let call: ContractCall<ProofPreimageMarker, InMemoryDB> =
            codec::decode_artifact(&prepared.bundle().leaves["processed-call.bin"]).unwrap();
        inspected.actions = vec![ContractAction::Call(Sp::new(call.clone()))].into();
        // This is only an ordinary shape comparison of a preimage-marked
        // inspection call. It never supplies a fake proof to a native verifier.
        validate_final_envelope(&prepared, &inspected).unwrap();
        assert!(validate_final_envelope(&prepared, prepared.raw_intent()).is_err());
        let f = support::payment();
        let state: crate::layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
        let address = codec::from_hex(&f.deployment.address_hex).unwrap();
        let additions = [
            ContractAction::Call(Sp::new(call.clone())),
            ContractAction::Deploy(Sp::new(midnight_ledger::structure::ContractDeploy {
                initial_state: state.index(address).unwrap(),
                nonce: midnight_base_crypto::hash::HashOutput([0; 32]),
            })),
            ContractAction::Maintain(midnight_ledger::structure::MaintenanceUpdate {
                address,
                updates: Default::default(),
                counter: 0,
                signatures: Default::default(),
            }),
        ];
        for extra in additions {
            let mut changed = inspected.clone();
            changed.actions = vec![ContractAction::Call(Sp::new(call.clone())), extra].into();
            assert_eq!(
                validate_final_envelope(&prepared, &changed)
                    .unwrap_err()
                    .code,
                "final-intent-identity"
            );
        }
        let mut changed = inspected.clone();
        changed.ttl = midnight_base_crypto::time::Timestamp::from_secs(101);
        assert!(validate_final_envelope(&prepared, &changed).is_err());
        let mut changed = inspected.clone();
        changed.guaranteed_unshielded_offer = None;
        assert!(validate_final_envelope(&prepared, &changed).is_err());
        let mut wrong_call = call.clone();
        wrong_call.communication_commitment.0 += Fr::from(1u64).0;
        let mut changed = inspected.clone();
        changed.actions = vec![ContractAction::Call(Sp::new(wrong_call))].into();
        assert!(validate_final_envelope(&prepared, &changed).is_err());
        let mut second_expansion = call;
        let mut transcript = (**second_expansion.guaranteed_transcript.as_ref().unwrap()).clone();
        let mut program: Vec<crate::ops::NativeOp> = Vec::from(&transcript.program);
        program.insert(0, midnight_onchain_vm::ops::Op::Noop { n: 1 });
        transcript.program = program.into();
        second_expansion.guaranteed_transcript = Some(Sp::new(transcript));
        let mut changed = inspected;
        changed.actions = vec![ContractAction::Call(Sp::new(second_expansion))].into();
        assert!(validate_final_envelope(&prepared, &changed).is_err());
    }
}

/// Actual installed consumer; operation comes from the admitted owner, never a
/// proof/key bundle argument. SDK Intent::prove receives its raw intent once.
pub async fn prove_installed_call(
    prepared: crate::prepare::PreparedInstalledCall,
    derived: crate::local_deployment::DerivedLocalDeployment,
    rng: StdRng,
) -> Result<ProvenInstalledCall> {
    if prepared.context.signed.is_some() {
        return Err(refusal("unsigned-profile"));
    }
    prove_installed_call_inner(prepared, derived, rng).await
}
pub(crate) async fn prove_installed_call_inner(
    prepared: crate::prepare::PreparedInstalledCall,
    derived: crate::local_deployment::DerivedLocalDeployment,
    rng: StdRng,
) -> Result<ProvenInstalledCall> {
    if prepared.public.identity() != derived.public.identity()
        || prepared.prepared.artifact().manifest.action != derived.action
    {
        return Err(refusal("installed-proving-policy"));
    }
    let context = prepared.proof_context();
    let inspection = prepared.bundle().leaves.clone();
    let readback = prepared.bundle().readback.clone();
    let mut result = prove_prepared_call(
        prepared.prepared,
        derived.selected,
        &prepared.operation,
        rng,
    )
    .await?;
    result.verification.scope = if context.signed.is_some() {
        crate::signed::RESULT_SCOPE
    } else {
        "local-unsigned-source-derived-installed-call-unqualified"
    };
    let final_readback = crate::prepare::final_installed_readback(&readback, result.verification());
    Ok(ProvenInstalledCall {
        native: result,
        context,
        inspection,
        final_readback,
    })
}
pub fn verify_installed_call(
    prepared: &crate::prepare::PreparedInstalledCall,
    candidate: &ProvenIntent,
) -> Result<CallVerification> {
    if prepared.context.signed.is_some() {
        return Err(refusal("unsigned-profile"));
    }
    verify_installed_call_inner(prepared, candidate)
}
pub(crate) fn verify_installed_call_inner(
    prepared: &crate::prepare::PreparedInstalledCall,
    candidate: &ProvenIntent,
) -> Result<CallVerification> {
    let mut result = verify_proven_call(
        &prepared.prepared,
        prepared
            .public
            .key(&prepared.prepared.artifact().manifest.action)?,
        &prepared.operation,
        candidate,
    )?;
    result.scope = if prepared.context.signed.is_some() {
        crate::signed::RESULT_SCOPE
    } else {
        "local-unsigned-source-derived-installed-call-unqualified"
    };
    Ok(result)
}

/// A genuine native installed result owns its final semantic readback and the
/// separately preserved inspection archive. There is no imported constructor.
/// ```compile_fail
/// let _: mori_ledger::proving::ProvenInstalledCall = serde_json::from_str("{}").unwrap();
/// ```
pub struct ProvenInstalledCall {
    native: ProvenNativeCall,
    context: crate::call_artifacts::ProofContext,
    inspection: std::collections::BTreeMap<String, Vec<u8>>,
    final_readback: crate::prepare::Readback,
}
impl ProvenInstalledCall {
    pub fn intent(&self) -> &ProvenIntent {
        self.native.intent()
    }
    pub fn verification(&self) -> &CallVerification {
        self.native.verification()
    }
    pub fn readback(&self) -> &crate::prepare::Readback {
        &self.final_readback
    }
    pub(crate) fn context(&self) -> &crate::call_artifacts::ProofContext {
        &self.context
    }
    pub(crate) fn inspection(&self) -> &std::collections::BTreeMap<String, Vec<u8>> {
        &self.inspection
    }
}
