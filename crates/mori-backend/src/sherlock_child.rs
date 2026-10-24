//! Sherlock A102 recursive-child producer. The unchanged `LiteralLayout::emit(LocalPublic)`
//! relation and its admitted statement are proved with the pinned stdlib generic `prove`
//! under `PoseidonState<Fq>`, the transcript Fenrir recl9 `inner::verify_offcircuit` and
//! `inner::verify_child_incircuit` consume. The child instance is the binding followed by
//! every statement lane; nothing is truncated or digested. The Blake2b literal proof, its
//! claim, key location and the nine relation-hash sources are untouched: this file is
//! outside them, so `relation_hash` and the context are unchanged, while the child binding
//! commits this file's own bytes.
//!
//! Witness: `IrSource::preprocess` is crate-private, so the pinned public `Preprocessed`
//! carrier is filled with exactly what `IrSource::circuit` reads for this relation: the
//! Native unguarded `PublicInput` outputs (each its positional lane), the binding, `pis`
//! (enabling the circuit's own consistency check) and the SDK-returned `pi_skips`. Every
//! other cell is synthesized by the circuit from those; no instruction is evaluated here.
//! The SDK's nonproving `ProofPreimage::check` with the real binding runs first.
//! `Preprocessed` is documented unstable: this carrier is valid at ledger 04c9 only.
//!
//! Scope: one charged A102 transition as a child. Not rollup, continuity, recursive or
//! terminal validity; a parent accumulator still needs a reviewed pairing check.
//! Compile/keygen/prove/verify: UNOBSERVED.
use crate::parameters::OfflineParameters;
use crate::proving::{MAX_K, MAX_PROOF_BYTES, ProofOptions, hex};
use crate::sherlock_machine::literal_bridge::{
    CapacityRecord, CircuitMode, LiteralLayout, LiteralSourceIdentity, LiteralTransitionRequest,
    OPEN_OBLIGATIONS, PROFILE, RULE_CHARGE, RULE_START_LINE, RULE_TAG, SCOPE, STATEMENT_FORMAT,
    local_max_statement_lanes,
};
use crate::{Error, LEDGER_REVISION, ZKIR_VERSION, error};
use midnight_circuits::hash::poseidon::PoseidonState;
use midnight_curves::Bls12;
use midnight_proofs::poly::kzg::params::{ParamsKZG, ParamsVerifierKZG};
use midnight_proofs::utils::SerdeFormat;
use midnight_transient_crypto::curve::{Fr, outer};
use midnight_transient_crypto::proofs::{
    KeyLocation, ParamsProver, ParamsProverProvider, ProofPreimage, VERIFIER_MAX_DEGREE, Zkir,
};
use midnight_zk_stdlib::{
    MidnightVK, prove as stdlib_prove, setup_pk, setup_vk, verify as stdlib_verify,
};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::ir_types::{IrType, IrValue};
use midnight_zkir_v3::{Instruction, IrSource, Preprocessed};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Fiat-Shamir hash of the child transcript; Fenrir recl9's `Pos`.
pub type ChildTranscript = PoseidonState<outer::Scalar>;

pub const CHILD_FORMAT: &str = "moriarty-sherlock-a102-poseidon-child/1";
pub const CHILD_BINDING_VERSION: &str = "moriarty-sherlock-a102-poseidon-child-binding/1";
pub const CHILD_TRANSCRIPT: &str = "midnight-zk-stdlib-2.3.5/PoseidonState<Fq>";
pub const CHILD_QUALIFICATION: &str = "local-a102-poseidon-child-proof-unqualified";
pub const CHILD_OPEN_OBLIGATIONS: [&str; 4] = [
    "Fenrir consumption crosses crate graphs only as processed VK, proof and lane bytes; midnight-proofs 0.8.3 versus 0.8.2 byte compatibility and the absence of truncated-challenges on both graphs are unestablished",
    "the key digest a consumer pins must come from this independent reconstruction, never from exported bytes",
    "a parent's collapsed accumulator is not validity until a reviewed terminal fold and pairing check; no terminal decider consumes this child",
    "child and in-circuit verifier resource admission over the full instance is separate and unestablished",
];
const CHECK_KEY_LOCATION: &str = "moriarty-sherlock-a102-poseidon-child-check-unqualified/1";
const ADAPTER_SOURCE: &str = include_str!("sherlock_child.rs");

/// Exact host claim committed by the child's binding lane.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChildClaim {
    pub format: String,
    pub binding: String,
    pub transcript: String,
    pub statement_format: String,
    pub profile: String,
    pub scope: String,
    pub ledger_revision: String,
    pub zkir_version: String,
    pub rule_start_line: u32,
    pub rule_tag: u32,
    pub charge: u32,
    pub capacities: CapacityRecord,
    pub identity: LiteralSourceIdentity,
    pub layout_hash: String,
    pub relation_hash: String,
    pub context_hash: String,
    pub zkir_hash: String,
    pub adapter_source_sha256: String,
    pub circuit_k: u8,
    /// Decimal statement lanes; the child instance is the binding followed by these.
    pub public_values: Vec<String>,
}

/// One produced child: claim, complete public instance, processed key bytes and proof.
#[derive(Clone, Debug)]
pub struct SherlockChildProof {
    pub claim: ChildClaim,
    /// Binding, then context limbs, ProgramBlock, pre, post, rule_tag, charge.
    pub instance: Vec<Fr>,
    /// `MidnightVK::write(Processed)`, the Fenrir `util::vk_from_bytes` form.
    pub vk_bytes: Vec<u8>,
    pub proof: Vec<u8>,
}
impl SherlockChildProof {
    pub fn instance_hex(&self) -> Vec<String> {
        self.instance.iter().map(field_hex).collect()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ChildVerification {
    pub qualification: &'static str,
    pub transcript: &'static str,
    pub context_hash: String,
    pub zkir_hash: String,
    pub adapter_source_sha256: String,
    pub vk_sha256: String,
    pub circuit_k: u8,
    pub instance_lanes: usize,
    pub proof_bytes: u32,
    pub open_obligations: Vec<&'static str>,
}

/// 32-byte little-endian lowercase hex, the Fenrir `util::f_from_hex` input.
pub fn field_hex(value: &Fr) -> String {
    hex(&value.as_le_bytes())
}
fn scalars(values: &[Fr]) -> Vec<outer::Scalar> {
    values.iter().map(|f| f.0).collect()
}

struct Prepared {
    ir: IrSource,
    claim: ChildClaim,
    statement: Vec<u128>,
    instance: Vec<Fr>,
}
fn prepare(request: &LiteralTransitionRequest, options: &ProofOptions) -> Result<Prepared, Error> {
    if options.max_k == 0 || options.max_k > MAX_K {
        return Err(error("limit", "Native k cap must be between 1 and 19."));
    }
    let layout = LiteralLayout::new(
        &request.capacities,
        &request.identity,
        local_max_statement_lanes(),
    )?;
    let statement = layout.statement(&request.values)?;
    if statement.len() + 1 > (1usize << VERIFIER_MAX_DEGREE) {
        return Err(error(
            "limit",
            "Public statement exceeds the pinned verifier capacity.",
        ));
    }
    let (ir, zkir) = layout.emit(CircuitMode::LocalPublic)?;
    let mut claim = ChildClaim {
        format: CHILD_FORMAT.into(),
        binding: CHILD_BINDING_VERSION.into(),
        transcript: CHILD_TRANSCRIPT.into(),
        statement_format: STATEMENT_FORMAT.into(),
        profile: PROFILE.into(),
        scope: SCOPE.into(),
        ledger_revision: LEDGER_REVISION.into(),
        zkir_version: ZKIR_VERSION.into(),
        rule_start_line: RULE_START_LINE as u32,
        rule_tag: RULE_TAG as u32,
        charge: RULE_CHARGE as u32,
        capacities: layout.record(),
        identity: layout.identity.clone(),
        layout_hash: hex(&layout.layout_hash),
        relation_hash: hex(&layout.relation_hash),
        context_hash: layout.context_hex(),
        zkir_hash: hex(&Sha256::digest(zkir.as_bytes())),
        adapter_source_sha256: hex(&Sha256::digest(ADAPTER_SOURCE.as_bytes())),
        // Widest admitted k until the native cost selection below.
        circuit_k: MAX_K,
        public_values: statement.iter().map(|v| v.to_string()).collect(),
    };
    crate::proving::validate_json_profile(&claim)?;
    // Native cost selection, never ordinary metadata.
    let circuit_k = ir.k();
    if circuit_k == 0 || circuit_k > options.max_k {
        return Err(error(
            "limit",
            format!("Circuit k={circuit_k} exceeds the explicit native k cap."),
        ));
    }
    claim.circuit_k = circuit_k;
    let binding = child_binding(&claim)?;
    let instance = std::iter::once(binding)
        .chain(statement.iter().map(|v| Fr::from(*v)))
        .collect();
    Ok(Prepared {
        ir,
        claim,
        statement,
        instance,
    })
}

/// SHA-256 of the domain-separated claim as low128 + 2^128 * high128, reduced by
/// field arithmetic exactly as the literal binding, under a distinct domain.
pub(crate) fn child_binding(claim: &ChildClaim) -> Result<Fr, Error> {
    let hash = crate::digest(&(CHILD_BINDING_VERSION, claim))?;
    let low = u128::from_le_bytes(hash[..16].try_into().expect("16 bytes"));
    let high = u128::from_le_bytes(hash[16..].try_into().expect("16 bytes"));
    Ok(Fr::from(low) + (Fr::from(u128::MAX) + Fr::from(1u64)) * Fr::from(high))
}

/// Pinned SDK nonproving check of `statement` under the real `binding`; returns the
/// SDK `pi_skips`, which must be complete and unskipped.
pub fn check_child_statement(
    ir: &IrSource,
    statement: &[u128],
    binding: Fr,
) -> Result<Vec<Option<usize>>, Error> {
    if statement.is_empty() || statement.len() + 1 > (1usize << VERIFIER_MAX_DEGREE) {
        return Err(error(
            "limit",
            "Public statement exceeds the pinned verifier capacity.",
        ));
    }
    let transcript: Vec<Fr> = statement.iter().copied().map(Fr::from).collect();
    let preimage = ProofPreimage {
        inputs: Vec::new(),
        private_transcript: Vec::new(),
        public_transcript_inputs: transcript.clone(),
        public_transcript_outputs: transcript,
        binding_input: binding,
        communications_commitment: None,
        key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
    };
    let pi_skips = preimage
        .check(ir)
        .map_err(|e| error("proving", e.to_string()))?;
    if pi_skips.len() != statement.len() || pi_skips.iter().any(Option::is_some) {
        return Err(error(
            "statement",
            "Nonproving check returned a different or skipped public statement.",
        ));
    }
    Ok(pi_skips)
}

/// The pinned `Preprocessed` carrier `IrSource::circuit` reads for a LocalPublic
/// relation. Each Native unguarded `PublicInput` output gets its positional lane after
/// checking the unconditional `Impact` publications name the same outputs in the same
/// order. Shape refusals here are prover tooling; the circuit remains the authority.
pub fn child_witness(
    ir: &IrSource,
    instance: &[Fr],
    pi_skips: Vec<Option<usize>>,
) -> Result<Preprocessed, Error> {
    let refuse = |m: &str| error("sherlock-child-witness", m.to_string());
    if !ir.inputs.is_empty() || ir.do_communications_commitment {
        return Err(refuse(
            "Only a LocalPublic relation without raw inputs or communications is supported.",
        ));
    }
    let (binding, lanes) = instance
        .split_first()
        .ok_or_else(|| refuse("Empty child instance."))?;
    let mut assigned = Vec::with_capacity(lanes.len());
    let mut published = Vec::with_capacity(lanes.len());
    for insn in ir.instructions.iter() {
        match insn {
            Instruction::PublicInput {
                guard: None,
                val_t: IrType::Native,
                output,
            } => assigned.push(output),
            Instruction::PublicInput { .. } | Instruction::PrivateInput { .. } => {
                return Err(refuse(
                    "Guarded, non-Native or private inputs are outside this carrier.",
                ));
            }
            Instruction::Impact { guard, inputs } => {
                if *guard != Operand::Immediate(Fr::from(1u64)) {
                    return Err(refuse("Every publication must be unconditionally active."));
                }
                for input in inputs {
                    match input {
                        Operand::Variable(id) => published.push(id),
                        Operand::Immediate(_) => {
                            return Err(refuse("Immediate publication is outside this carrier."));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    if assigned.len() != lanes.len()
        || assigned != published
        || pi_skips.len() != lanes.len()
        || pi_skips.iter().any(Option::is_some)
    {
        return Err(refuse(
            "Statement inputs, publications, skips and instance lanes differ.",
        ));
    }
    let mut memory = HashMap::with_capacity(lanes.len());
    for (id, lane) in assigned.into_iter().zip(lanes) {
        if memory.insert(id.clone(), IrValue::Native(*lane)).is_some() {
            return Err(refuse("Repeated statement identifier."));
        }
    }
    Ok(Preprocessed {
        memory,
        pis: scalars(instance),
        pi_skips,
        binding_input: binding.0,
        comm_comm: None,
    })
}

/// Reconstructed circuit, statement and key; no supplied key is authority.
pub struct ChildKey {
    prepared: Prepared,
    params: ParamsProver,
    vk: MidnightVK,
    vk_bytes: Vec<u8>,
    verifier: ParamsVerifierKZG<Bls12>,
}
impl ChildKey {
    /// Native: k selection, pinned SRS load and `setup_vk`.
    pub fn reconstruct(
        request: &LiteralTransitionRequest,
        options: &ProofOptions,
    ) -> Result<Self, Error> {
        Self::from_prepared(prepare(request, options)?, options)
    }
    fn from_prepared(prepared: Prepared, options: &ProofOptions) -> Result<Self, Error> {
        let k = prepared.claim.circuit_k;
        let provider = OfflineParameters::load(&options.params_dir, k)?;
        let params = futures::executor::block_on(provider.get_params(k))
            .map_err(|e| error("parameters", e.to_string()))?;
        let kzg: &ParamsKZG<Bls12> = &params.0;
        let vk = setup_vk(kzg, &prepared.ir);
        if vk.k() != k {
            return Err(error(
                "keygen",
                "Child key degree differs from the circuit.",
            ));
        }
        let vk_bytes = write_vk(&vk)?;
        let reread = MidnightVK::read(&mut &vk_bytes[..], SerdeFormat::Processed)
            .map_err(|e| error("keygen", e.to_string()))?;
        if write_vk(&reread)? != vk_bytes {
            return Err(error("keygen", "Child key bytes do not round-trip."));
        }
        let verifier = kzg.verifier_params();
        Ok(Self {
            prepared,
            params,
            vk,
            vk_bytes,
            verifier,
        })
    }
    pub fn claim(&self) -> &ChildClaim {
        &self.prepared.claim
    }
    pub fn instance(&self) -> &[Fr] {
        &self.prepared.instance
    }
    pub fn vk_bytes(&self) -> &[u8] {
        &self.vk_bytes
    }
    pub fn vk_sha256(&self) -> String {
        hex(&Sha256::digest(&self.vk_bytes))
    }
    /// Stdlib verification under the child transcript, including the stdlib
    /// public-input count check and transcript end.
    pub fn verify(&self, instance: &[Fr], proof: &[u8]) -> Result<(), Error> {
        if proof.is_empty() || proof.len() > MAX_PROOF_BYTES {
            return Err(error("proof", "Expected a nonempty bounded child proof."));
        }
        stdlib_verify::<IrSource, ChildTranscript>(
            &self.verifier,
            &self.vk,
            &scalars(instance),
            None,
            proof,
        )
        .map_err(|e| error("verification", format!("{e:?}")))
    }
    /// Native: SDK check under the real binding, carrier, `setup_pk`, prove, self-verify.
    pub fn prove(&self) -> Result<SherlockChildProof, Error> {
        let p = &self.prepared;
        let pi_skips = check_child_statement(&p.ir, &p.statement, p.instance[0])?;
        let witness = child_witness(&p.ir, &p.instance, pi_skips)?;
        let pk = setup_pk(&p.ir, &self.vk);
        let kzg: &ParamsKZG<Bls12> = &self.params.0;
        let proof = stdlib_prove::<IrSource, ChildTranscript>(
            kzg,
            &pk,
            &p.ir,
            &scalars(&p.instance),
            witness,
            rand::rngs::OsRng,
        )
        .map_err(|e| error("proving", format!("{e:?}")))?;
        if proof.is_empty() || proof.len() > MAX_PROOF_BYTES {
            return Err(error(
                "limit",
                "Child proof exceeds the nonempty 262144-byte cap.",
            ));
        }
        self.verify(&p.instance, &proof)?;
        Ok(SherlockChildProof {
            claim: p.claim.clone(),
            instance: p.instance.clone(),
            vk_bytes: self.vk_bytes.clone(),
            proof,
        })
    }
}
fn write_vk(vk: &MidnightVK) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    vk.write(&mut bytes, SerdeFormat::Processed)
        .map_err(|e| error("keygen", e.to_string()))?;
    Ok(bytes)
}

/// Native work: pinned SRS, key generation, child proof and producer-side verify.
pub fn prove_sherlock_child(
    request: &LiteralTransitionRequest,
    options: &ProofOptions,
) -> Result<SherlockChildProof, Error> {
    ChildKey::reconstruct(request, options)?.prove()
}

/// Independent verification: an empty or oversized proof refuses first, before k
/// selection, SRS or key work; then claim and instance are rebuilt (compared before any
/// SRS access), then the key (compared bytewise with the export), then stdlib verify.
pub fn verify_sherlock_child(
    request: &LiteralTransitionRequest,
    child: &SherlockChildProof,
    options: &ProofOptions,
) -> Result<ChildVerification, Error> {
    if child.proof.is_empty() || child.proof.len() > MAX_PROOF_BYTES {
        return Err(error("proof", "Expected a nonempty bounded child proof."));
    }
    let prepared = prepare(request, options)?;
    if child.claim != prepared.claim {
        return Err(error(
            "claim",
            "Claim differs from the independently reconstructed A102 child statement.",
        ));
    }
    if child.instance != prepared.instance {
        return Err(error(
            "statement",
            "Child instance differs from the reconstructed binding and statement.",
        ));
    }
    let key = ChildKey::from_prepared(prepared, options)?;
    if child.vk_bytes != key.vk_bytes {
        return Err(error(
            "keygen",
            "Exported child key differs from the reconstructed key.",
        ));
    }
    key.verify(&child.instance, &child.proof)?;
    let claim = key.claim();
    Ok(ChildVerification {
        qualification: CHILD_QUALIFICATION,
        transcript: CHILD_TRANSCRIPT,
        context_hash: claim.context_hash.clone(),
        zkir_hash: claim.zkir_hash.clone(),
        adapter_source_sha256: claim.adapter_source_sha256.clone(),
        vk_sha256: key.vk_sha256(),
        circuit_k: claim.circuit_k,
        instance_lanes: child.instance.len(),
        proof_bytes: child.proof.len() as u32,
        open_obligations: OPEN_OBLIGATIONS
            .iter()
            .chain(CHILD_OPEN_OBLIGATIONS.iter())
            .copied()
            .collect(),
    })
}
