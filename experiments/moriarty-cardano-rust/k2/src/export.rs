//! Public artefacts: the setup record and the proof export, with strict parsers.
//!
//! Neither document contains proving-key bytes, setup randomness or prover blinding. Every
//! field that duplicates another (identities copied from the setup record, decoded context
//! fields, digests, public inputs, the Cardano section) is re-derived by [`check_export`] and
//! must be byte-equal; a document whose parts disagree is refused, never reconciled.
use crate::claim::{self, MAX_STATEMENT_BYTES, PreparedClaim, StatementView};
use crate::encoding::{self, PROOF_LEN, VK_LEN};
use crate::error::DriverError;
use crate::hexutil;
use crate::input::parse_json;
use crate::relation::RelationDescriptor;
use ark_bls12_381::Bls12_381;
use ark_groth16::{Proof, VerifyingKey};
use mori_r1cs::ContextV2;
use mori_r1cs::abi2::FAMILY_V2;
use mori_r1cs::identity::{self, Digest32};
use serde::{Deserialize, Serialize};

pub const SETUP_FORMAT: &str = "moriarty-k2-groth16-setup/1";
pub const PROOF_FORMAT: &str = "moriarty-k2-groth16-proof/1";
pub const TRUST_PROFILE: &str = "single-party-test-setup/1";
pub const SETUP_NOTICE: &str = "TEST-ONLY single-party Groth16 setup sampled from OsRng; not a ceremony. Whoever controlled the setup process could forge proofs for this relation. VK provenance is declared by this record; no verifier can check that the VK was generated from r1csSha256.";
pub const CURVE: &str = "BLS12-381";
pub const SNARK: &str = "Groth16 (ark-groth16 =0.4.0, LibsnarkReduction)";
pub const ENCODING: &str = "arkworks 0.4 compressed points (Zcash BLS12-381 layout); VK 488 bytes, proof 192 bytes";

/// Fixed non-claims carried by every proof export.
pub const PREMISES: [&str; 6] = [
    "The proof shows, under setup honesty and Groth16 knowledge soundness, that the statement's constant header and body satisfy the named compiled moriarty-core/1 relation.",
    "Context bytes (vk digest, claimant, network, settle hash, thread, consumed outref, seq, catalogue digest, intents digest) are caller-declared premises: bound by the proof, not authenticated by it. S2 recomputes each from its authenticated ScriptContext and datum.",
    "Trust profile single-party-test-setup/1: whoever ran setup could forge proofs for this relation.",
    "Groth16 proofs are malleable: the proof bytes are not a unique claim identifier.",
    "Signed intents, debit authority, frame completeness, physical asset delta, replay and phase outcomes are target obligations outside this proof.",
    "No ledger admission, deployment, settlement, Preprod or Midnight result is implied.",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SetupRecord {
    pub format: String,
    pub trust_profile: String,
    pub notice: String,
    pub relation: RelationDescriptor,
    pub curve: String,
    pub snark: String,
    pub encoding: String,
    /// The 488-byte C2 VK, lowercase hex.
    pub verifying_key_hex: String,
    /// BLAKE2b-256 of the VK bytes (C2 `env.vk_digest`, catalogue `vk_digest`).
    pub vk_digest_blake2b256: String,
    /// Declared custody link to the proving-key file (SHA-256 of its exact bytes).
    pub proving_key_sha256: String,
    pub proving_key_bytes: u64,
    pub driver: String,
}

/// A setup record that passed strict parsing, with its decoded VK.
#[derive(Debug, Clone)]
pub struct ParsedSetup {
    pub record: SetupRecord,
    pub vk: VerifyingKey<Bls12_381>,
    pub vk_bytes: Vec<u8>,
    pub vk_digest: [u8; 32],
    /// SHA-256 of the exact record file bytes.
    pub record_sha256: Digest32,
}

fn same(what: &str, expected: &str, actual: &str) -> Result<(), DriverError> {
    if expected == actual { Ok(()) } else { Err(DriverError::mismatch(what, expected, actual)) }
}

fn fixed(what: &str, got: &str, want: &str) -> Result<(), DriverError> {
    if got == want { Ok(()) } else { Err(DriverError::Unsupported(format!("{what} {got:?}; expected {want:?}"))) }
}

/// Canonical bytes of a public document: Core's ASCII-safe pretty JSON plus a final newline.
pub fn document_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, DriverError> {
    let text = mori_core::json::export(value).map_err(|e| DriverError::Backend(format!("export: {e}")))?;
    let mut out = text.into_bytes();
    out.push(b'\n');
    Ok(out)
}

/// Strict setup-record parsing: profile strings, descriptor syntax, strict VK decoding and the
/// VK digest. `bytes` must be the exact file bytes (their SHA-256 is the record identity).
pub fn parse_setup_record(bytes: &[u8]) -> Result<ParsedSetup, DriverError> {
    let record: SetupRecord = parse_json(bytes, "setup record")?;
    fixed("setup record format", &record.format, SETUP_FORMAT)?;
    fixed("trust profile", &record.trust_profile, TRUST_PROFILE)?;
    fixed("notice", &record.notice, SETUP_NOTICE)?;
    fixed("curve", &record.curve, CURVE)?;
    fixed("snark", &record.snark, SNARK)?;
    fixed("encoding", &record.encoding, ENCODING)?;
    fixed("driver", &record.driver, crate::DRIVER_ID)?;
    record.relation.check_profile()?;
    let vk_bytes = hexutil::decode_exact::<VK_LEN>(&record.verifying_key_hex, "verifyingKeyHex")?.to_vec();
    let vk = encoding::decode_vk(&vk_bytes)?;
    let vk_digest = mori_r1cs::blake2b::blake2b256(&vk_bytes);
    same("vkDigestBlake2b256", &hexutil::encode(&vk_digest), &record.vk_digest_blake2b256)?;
    hexutil::decode_exact::<32>(&record.proving_key_sha256, "provingKeySha256")?;
    if record.proving_key_bytes == 0 {
        return Err(DriverError::Input("provingKeyBytes must be positive".into()));
    }
    Ok(ParsedSetup { record, vk, vk_bytes, vk_digest, record_sha256: identity::digest_bytes(bytes) })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContextExport {
    pub vk_digest: String,
    pub claimant_vkh: String,
    pub network_id: u8,
    pub settle_hash: String,
    pub thread_id: String,
    pub consumed_txid: String,
    pub consumed_ix: u16,
    pub seq: u64,
    pub catalogue_digest: String,
    pub intents_digest: String,
}

pub fn context_export(ctx: &ContextV2) -> ContextExport {
    ContextExport {
        vk_digest: hexutil::encode(&ctx.vk_digest),
        claimant_vkh: hexutil::encode(&ctx.claimant_vkh),
        network_id: ctx.network_id,
        settle_hash: hexutil::encode(&ctx.settle_hash),
        thread_id: hexutil::encode(&ctx.thread_id),
        consumed_txid: hexutil::encode(&ctx.consumed_txid),
        consumed_ix: ctx.consumed_ix,
        seq: ctx.seq,
        catalogue_digest: hexutil::encode(&ctx.catalogue_digest),
        intents_digest: hexutil::encode(&ctx.intents_digest),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PublicInputsExport {
    /// `int_be(digest[0..16])`, canonical decimal.
    pub x1: String,
    /// `int_be(digest[16..32])`, canonical decimal.
    pub x2: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OutRefExport {
    pub transaction_id: String,
    pub output_index: u16,
}

/// `EnvV2` field values (`types.ak:227-238`) except `expiry`, which the transaction builder
/// chooses and the proof does not bind. The driver produces no Plutus Data/CBOR.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EnvV2Export {
    pub version: u8,
    pub claim_kind: u8,
    pub family_id: String,
    pub predicate_digest: String,
    pub vk_digest: String,
    pub claimant_vkh: String,
    pub settle_outref: OutRefExport,
    pub statement_digest: String,
}

/// The digests of a catalogue `ActionEntryV2` (`types.ak:73-82`); `advancers` is the
/// catalogue author's choice and is not produced here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ActionEntryExport {
    pub name: String,
    pub index: u16,
    pub predicate_digest: String,
    pub vk_digest: String,
    pub shape_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CardanoExport {
    pub env_v2: EnvV2Export,
    pub catalogue_action_entry: ActionEntryExport,
    /// Domain `/2` intent `authority.target_hash`: `pred:vk:shape` (194 bytes, ABI-V2.md 7.2).
    pub intent_target_hash_v2: String,
    /// `cardano-plutus-v3:` + hex(settle hash) from the declared context (S2 derives its own).
    pub intent_anchor: String,
}

pub fn cardano_export(desc: &RelationDescriptor, vk_digest: &[u8; 32], ctx: &ContextV2, statement_digest: &[u8; 32]) -> CardanoExport {
    let vk = hexutil::encode(vk_digest);
    CardanoExport {
        env_v2: EnvV2Export {
            version: 2,
            claim_kind: 0,
            family_id: hexutil::encode(FAMILY_V2),
            predicate_digest: desc.predicate_digest.clone(),
            vk_digest: vk.clone(),
            claimant_vkh: hexutil::encode(&ctx.claimant_vkh),
            settle_outref: OutRefExport { transaction_id: hexutil::encode(&ctx.consumed_txid), output_index: ctx.consumed_ix },
            statement_digest: hexutil::encode(statement_digest),
        },
        catalogue_action_entry: ActionEntryExport {
            name: desc.action.clone(),
            index: desc.action_index,
            predicate_digest: desc.predicate_digest.clone(),
            vk_digest: vk.clone(),
            shape_digest: desc.abi2_shape_digest.clone(),
        },
        intent_target_hash_v2: format!("{}:{}:{}", desc.predicate_digest, vk, desc.abi2_shape_digest),
        intent_anchor: format!("cardano-plutus-v3:{}", hexutil::encode(&ctx.settle_hash)),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProofExport {
    pub format: String,
    /// SHA-256 of the exact setup-record file bytes (declared link, checked by `verify`).
    pub setup_record_sha256: String,
    pub program_hash: String,
    pub source_sha256: Option<String>,
    pub action: String,
    pub action_index: u16,
    pub compiler_build_digest: String,
    pub r1cs_sha256: String,
    pub predicate_digest: String,
    pub abi2_shape_digest: String,
    pub vk_digest_blake2b256: String,
    pub context: ContextExport,
    pub statement_hex: String,
    pub statement_digest_blake2b256: String,
    pub public_inputs: PublicInputsExport,
    /// The 192-byte C2 proof, lowercase hex (C2 `Issue { vk, proof }` takes the record's VK).
    pub proof_hex: String,
    pub cardano: CardanoExport,
    pub premises: Vec<String>,
}

pub fn parse_proof_export(bytes: &[u8]) -> Result<ProofExport, DriverError> {
    let e: ProofExport = parse_json(bytes, "proof export")?;
    fixed("proof export format", &e.format, PROOF_FORMAT)?;
    Ok(e)
}

/// Builds the export of a proof. The caller runs [`check_export`] on its serialized bytes.
pub fn build_export(setup: &ParsedSetup, claim: &PreparedClaim, proof: &Proof<Bls12_381>) -> Result<ProofExport, DriverError> {
    let desc = &setup.record.relation;
    Ok(ProofExport {
        format: PROOF_FORMAT.into(),
        setup_record_sha256: setup.record_sha256.hex(),
        program_hash: desc.program_hash.clone(),
        source_sha256: desc.source_sha256.clone(),
        action: desc.action.clone(),
        action_index: desc.action_index,
        compiler_build_digest: desc.compiler_build_digest.clone(),
        r1cs_sha256: desc.r1cs_sha256.clone(),
        predicate_digest: desc.predicate_digest.clone(),
        abi2_shape_digest: desc.abi2_shape_digest.clone(),
        vk_digest_blake2b256: hexutil::encode(&setup.vk_digest),
        context: context_export(&claim.context),
        statement_hex: hexutil::encode(&claim.statement),
        statement_digest_blake2b256: hexutil::encode(&claim.statement_digest),
        public_inputs: PublicInputsExport { x1: claim.public_inputs[0].to_string(), x2: claim.public_inputs[1].to_string() },
        proof_hex: hexutil::encode(&encoding::encode_proof(proof)?),
        cardano: cardano_export(desc, &setup.vk_digest, &claim.context, &claim.statement_digest),
        premises: PREMISES.iter().map(|s| (*s).to_string()).collect(),
    })
}

/// Everything a verifier needs after the export passed every non-cryptographic check.
#[derive(Debug, Clone)]
pub struct CheckedExport {
    pub proof: Proof<Bls12_381>,
    pub public_inputs: [u128; 2],
    pub statement: Vec<u8>,
    pub statement_digest: [u8; 32],
    pub view: StatementView,
}

/// Every non-pairing check of a proof export against a parsed setup record.
pub fn check_export(setup: &ParsedSetup, e: &ProofExport) -> Result<CheckedExport, DriverError> {
    fixed("proof export format", &e.format, PROOF_FORMAT)?;
    let desc = &setup.record.relation;
    same("setupRecordSha256", &setup.record_sha256.hex(), &e.setup_record_sha256)?;
    same("programHash", &desc.program_hash, &e.program_hash)?;
    same("sourceSha256", &format!("{:?}", desc.source_sha256), &format!("{:?}", e.source_sha256))?;
    same("action", &desc.action, &e.action)?;
    same("actionIndex", &desc.action_index.to_string(), &e.action_index.to_string())?;
    same("compilerBuildDigest", &desc.compiler_build_digest, &e.compiler_build_digest)?;
    same("r1csSha256", &desc.r1cs_sha256, &e.r1cs_sha256)?;
    same("predicateDigest", &desc.predicate_digest, &e.predicate_digest)?;
    same("abi2ShapeDigest", &desc.abi2_shape_digest, &e.abi2_shape_digest)?;
    same("vkDigestBlake2b256", &hexutil::encode(&setup.vk_digest), &e.vk_digest_blake2b256)?;

    let statement = hexutil::decode_bounded(&e.statement_hex, MAX_STATEMENT_BYTES, "statementHex")?;
    let view = claim::parse_statement(&statement)?;
    claim::check_statement_against(desc, &setup.vk_digest, &view)?;
    if context_export(&view.context) != e.context {
        return Err(DriverError::mismatch("context fields", "the statement's context bytes", "different declared fields"));
    }
    let statement_digest = mori_r1cs::blake2b::blake2b256(&statement);
    same("statementDigestBlake2b256", &hexutil::encode(&statement_digest), &e.statement_digest_blake2b256)?;
    let public_inputs = claim::digest_halves(&statement_digest);
    let x1 = hexutil::parse_u128_decimal(&e.public_inputs.x1, "x1")?;
    let x2 = hexutil::parse_u128_decimal(&e.public_inputs.x2, "x2")?;
    same("x1", &public_inputs[0].to_string(), &x1.to_string())?;
    same("x2", &public_inputs[1].to_string(), &x2.to_string())?;
    let proof_bytes = hexutil::decode_exact::<PROOF_LEN>(&e.proof_hex, "proofHex")?;
    let proof = encoding::decode_proof(&proof_bytes)?;
    if cardano_export(desc, &setup.vk_digest, &view.context, &statement_digest) != e.cardano {
        return Err(DriverError::mismatch("cardano section", "values derived from the statement and record", "different values"));
    }
    if e.premises.len() != PREMISES.len() || e.premises.iter().zip(PREMISES).any(|(a, b)| a != b) {
        return Err(DriverError::mismatch("premises", "the fixed premise list", "a different list"));
    }
    Ok(CheckedExport { proof, public_inputs, statement, statement_digest, view })
}
