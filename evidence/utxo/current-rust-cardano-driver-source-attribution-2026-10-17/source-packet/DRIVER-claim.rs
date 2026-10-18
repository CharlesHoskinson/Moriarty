//! Claimed transitions and their ABI v2 statements.
//!
//! Proposal is separate from acceptance. [`propose_values`] obtains a *claim* from Core's
//! reference or from explicit records; it decides nothing. The statement is built by the
//! compiler's own `CompiledRelation::statement_v2`. Acceptance is the constraint system's
//! (`crate::prove::check_claim`) and, for a proof, the pairing check's.
use crate::error::DriverError;
use crate::hexutil;
use crate::input::{ContextInput, TransitionInput};
use crate::relation::{RelationDescriptor, SelectedRelation};
use ark_bls12_381::Fr;
use mori_r1cs::abi2::{self, AbiV2Constants, FAMILY_V2, HEADER_LEN, STMT_TAG};
use mori_r1cs::{CompiledRelation, ContextV2, TransitionValues, witness};

/// Statement ceiling (bytes). Larger statements are refused before hashing or synthesis.
pub const MAX_STATEMENT_BYTES: usize = 262_144;

/// Byte offsets of ABI v2 statement fields (`abi2.rs:5-16`, `ABI-V2.md` section 3).
pub mod offsets {
    pub const PREDICATE: usize = 48;
    pub const SHAPE: usize = 80;
    pub const VK_DIGEST: usize = 112;
    pub const CLAIMANT: usize = 144;
    pub const NETWORK: usize = 172;
    pub const SETTLE: usize = 173;
    pub const THREAD: usize = 201;
    pub const TXID: usize = 233;
    pub const IX: usize = 265;
    pub const SEQ: usize = 267;
    pub const CATALOGUE: usize = 275;
    pub const INTENTS: usize = 307;
    pub const BODY_LEN: usize = 339;
}

/// A claim ready for the constraint system: values, context, statement and public inputs.
#[derive(Debug, Clone)]
pub struct PreparedClaim {
    pub values: TransitionValues,
    pub context: ContextV2,
    pub statement: Vec<u8>,
    pub statement_digest: [u8; 32],
    /// `[x1, x2]`: big-endian halves of `statement_digest`.
    pub public_inputs: [u128; 2],
}

/// Big-endian 128-bit halves of a 32-byte digest (`abi2.rs:297-305`).
pub fn digest_halves(d: &[u8; 32]) -> [u128; 2] {
    let mut hi = [0u8; 16];
    let mut lo = [0u8; 16];
    hi.copy_from_slice(&d[..16]);
    lo.copy_from_slice(&d[16..]);
    [u128::from_be_bytes(hi), u128::from_be_bytes(lo)]
}

/// Builds `ContextV2` from declared premises and the digest of the VK actually used.
pub fn context_from_input(input: &ContextInput, vk_digest: [u8; 32]) -> Result<ContextV2, DriverError> {
    if let Some(stated) = &input.vk_digest {
        let stated = hexutil::decode_exact::<32>(stated, "context vkDigest")?;
        if stated != vk_digest {
            return Err(DriverError::mismatch("context vkDigest", hexutil::encode(&vk_digest), hexutil::encode(&stated)));
        }
    }
    Ok(ContextV2 {
        vk_digest,
        claimant_vkh: hexutil::decode_exact::<28>(&input.claimant_vkh, "claimantVkh")?,
        network_id: input.network_id,
        settle_hash: hexutil::decode_exact::<28>(&input.settle_hash, "settleHash")?,
        thread_id: hexutil::decode_exact::<32>(&input.thread_id, "threadId")?,
        consumed_txid: hexutil::decode_exact::<32>(&input.consumed_txid, "consumedTxid")?,
        consumed_ix: input.consumed_ix,
        seq: input.seq,
        catalogue_digest: hexutil::decode_exact::<32>(&input.catalogue_digest, "catalogueDigest")?,
        intents_digest: hexutil::decode_exact::<32>(&input.intents_digest, "intentsDigest")?,
    })
}

/// Proposes transition values. A Core refusal or a record that does not fit the relation's
/// shape is a refusal to propose, never a "false" claim.
pub fn propose_values(relation: &CompiledRelation, transition: &TransitionInput) -> Result<TransitionValues, DriverError> {
    match transition {
        TransitionInput::CoreReference { scenario } => witness::propose(relation, scenario, None).map_err(DriverError::Proposal),
        TransitionInput::Records { inputs, pre, post, effects } => {
            witness::from_records(relation.plan(), inputs, pre, post, effects, None).map_err(DriverError::Proposal)
        }
    }
}

/// Builds the statement and public inputs of already-proposed values with the compiler. The
/// values are a claim of any origin (proposal, records or a hostile edit); the compiler checks
/// only their shape and types here, and the constraint system decides them.
pub fn claim_from_values(selected: &SelectedRelation, values: TransitionValues, ctx: ContextV2) -> Result<PreparedClaim, DriverError> {
    let relation = selected.relation();
    let statement = relation.statement_v2(&values, &ctx).map_err(DriverError::Proposal)?;
    if statement.len() > MAX_STATEMENT_BYTES {
        return Err(DriverError::Input(format!("statement of {} bytes exceeds {MAX_STATEMENT_BYTES}", statement.len())));
    }
    let statement_digest = mori_r1cs::blake2b::blake2b256(&statement);
    let public_inputs = digest_halves(&statement_digest);
    // The compiler's own public-input path must agree with the digest halves.
    let compiled = relation.public_inputs_v2::<Fr>(&values, &ctx).map_err(DriverError::Proposal)?;
    if compiled != vec![Fr::from(public_inputs[0]), Fr::from(public_inputs[1])] {
        return Err(DriverError::Backend("compiler public inputs differ from the statement digest halves".into()));
    }
    Ok(PreparedClaim { values, context: ctx, statement, statement_digest, public_inputs })
}

/// Proposes the values and builds the statement and public inputs with the compiler.
pub fn prepare_claim(
    selected: &SelectedRelation,
    transition: &TransitionInput,
    context: &ContextInput,
    vk_digest: [u8; 32],
) -> Result<PreparedClaim, DriverError> {
    let ctx = context_from_input(context, vk_digest)?;
    let values = propose_values(selected.relation(), transition)?;
    claim_from_values(selected, values, ctx)
}

/// The fields of a parsed ABI v2 statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatementView {
    pub predicate_digest: [u8; 32],
    pub shape_digest: [u8; 32],
    pub context: ContextV2,
    pub body_len: u32,
}

fn take<const N: usize>(s: &[u8], at: usize) -> Result<[u8; N], DriverError> {
    s.get(at..at + N)
        .and_then(|b| <[u8; N]>::try_from(b).ok())
        .ok_or_else(|| DriverError::Input(format!("statement too short for {N} bytes at {at}")))
}

/// Strictly parses statement bytes, then re-encodes them with the compiler's
/// `abi2::statement_bytes` and requires byte equality (so the offsets here cannot drift from
/// the compiler's layout unnoticed).
pub fn parse_statement(s: &[u8]) -> Result<StatementView, DriverError> {
    use crate::claim::offsets as o;
    if s.len() < HEADER_LEN || s.len() > MAX_STATEMENT_BYTES {
        return Err(DriverError::Input(format!("statement length {} outside [{HEADER_LEN}, {MAX_STATEMENT_BYTES}]", s.len())));
    }
    if s[..16] != STMT_TAG[..] {
        return Err(DriverError::Input("statement tag is not MRG16-PVC-STMT-2".into()));
    }
    if s[16..48] != FAMILY_V2[..] {
        return Err(DriverError::Input("statement family id is not MRG16-PVC-FAM2-G16-BLS-ARK04-HL2".into()));
    }
    let context = ContextV2 {
        vk_digest: take::<32>(s, o::VK_DIGEST)?,
        claimant_vkh: take::<28>(s, o::CLAIMANT)?,
        network_id: s[o::NETWORK],
        settle_hash: take::<28>(s, o::SETTLE)?,
        thread_id: take::<32>(s, o::THREAD)?,
        consumed_txid: take::<32>(s, o::TXID)?,
        consumed_ix: u16::from_be_bytes(take::<2>(s, o::IX)?),
        seq: u64::from_be_bytes(take::<8>(s, o::SEQ)?),
        catalogue_digest: take::<32>(s, o::CATALOGUE)?,
        intents_digest: take::<32>(s, o::INTENTS)?,
    };
    let body_len = u32::from_be_bytes(take::<4>(s, o::BODY_LEN)?);
    let expected_len = usize::try_from(body_len).ok().and_then(|b| b.checked_add(HEADER_LEN));
    if expected_len != Some(s.len()) {
        return Err(DriverError::Input(format!("statement length {} differs from 343 + body_len {body_len}", s.len())));
    }
    let view = StatementView {
        predicate_digest: take::<32>(s, o::PREDICATE)?,
        shape_digest: take::<32>(s, o::SHAPE)?,
        context,
        body_len,
    };
    let consts = AbiV2Constants {
        predicate_digest: view.predicate_digest,
        shape_bytes: Vec::new(),
        shape_digest: view.shape_digest,
        body_len,
    };
    let rebuilt = abi2::statement_bytes(&consts, &view.context, &s[HEADER_LEN..]).map_err(DriverError::Proposal)?;
    if rebuilt != s {
        return Err(DriverError::Backend("statement offsets differ from the compiler's ABI v2 encoder".into()));
    }
    Ok(view)
}

/// Checks a parsed statement's relation constants and VK digest against a descriptor.
pub fn check_statement_against(desc: &RelationDescriptor, vk_digest: &[u8; 32], view: &StatementView) -> Result<(), DriverError> {
    let checks = [
        ("statement predicate digest", desc.predicate_digest.clone(), hexutil::encode(&view.predicate_digest)),
        ("statement ABI v2 shape digest", desc.abi2_shape_digest.clone(), hexutil::encode(&view.shape_digest)),
        ("statement vk digest", hexutil::encode(vk_digest), hexutil::encode(&view.context.vk_digest)),
        ("statement body length", desc.body_len.to_string(), view.body_len.to_string()),
    ];
    for (what, expected, actual) in checks {
        if expected != actual {
            return Err(DriverError::mismatch(what, expected, actual));
        }
    }
    Ok(())
}
