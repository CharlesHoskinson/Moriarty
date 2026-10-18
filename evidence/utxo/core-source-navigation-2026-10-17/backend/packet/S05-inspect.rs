//! Ordinary statement receipt (`mori-k2 inspect`). K4H successor source; never compiled or
//! run by its author.
//!
//! For one request it prints the claimed statement, its BLAKE2b-256 digest and public inputs,
//! every body slot decoded from the statement BYTES in the compiled layout's slot order, and
//! the fixed setup-mode matrices' decision on the claim's full assignment. A reviewer compares
//! the decoded values with gold frozen before execution; the decoder never reads
//! `TransitionValues`. No randomness, key, proof, pairing or file write.
//!
//! The VK digest is caller-declared (no setup exists in this tier). Context bytes are bound by
//! the statement, not authenticated by it.
use crate::claim::{self, PreparedClaim};
use crate::error::DriverError;
use crate::fixed::{self, FixedMatrices};
use crate::hexutil;
use crate::input::ProveRequest;
use crate::relation::SelectedRelation;
use mori_r1cs::abi2::HEADER_LEN;
use mori_r1cs::{SlotEncoding, Tamper};
use serde::Serialize;
use serde_json::Value as Json;

pub const RECEIPT_FORMAT: &str = "moriarty-k2-statement-receipt/1";
pub const RECEIPT_NOTICE: &str = "Ordinary receipt: no setup, key, proof or pairing. The verdict is the fixed setup-mode matrices' decision on this claim's full assignment. The VK digest is caller-declared. Context bytes are bound, not authenticated. Values were proposed by the request, not decided by the host.";

/// One body slot as decoded from the statement bytes.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SlotValue {
    pub label: String,
    /// `uint128` (decimal string), `bool` (true/false) or `wide144` (decimal string).
    pub encoding: &'static str,
    pub value: Json,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedMatrixCheck {
    pub r1cs_sha256: String,
    pub equals_descriptor: bool,
    /// The prove-mode matrices of this claim's synthesis have the fixed digest.
    pub prove_matrix_digest_equal: bool,
    pub constraints: usize,
    pub violated: usize,
    pub first_violated: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatementReceipt {
    pub format: &'static str,
    pub notice: &'static str,
    pub action: String,
    pub program_hash: String,
    pub predicate_digest: String,
    pub abi2_shape_digest: String,
    pub r1cs_sha256: String,
    pub declared_vk_digest: String,
    pub statement_len: usize,
    pub statement_hex: String,
    pub statement_digest_blake2b256: String,
    pub x1: String,
    pub x2: String,
    pub slots: Vec<SlotValue>,
    pub fixed_matrix: FixedMatrixCheck,
}

/// Decodes every body slot of `statement` in layout order. Refuses a statement whose length is
/// not `343 + bodyLen` or whose Bool byte is neither 0 nor 1.
pub fn decode_body(selected: &SelectedRelation, statement: &[u8]) -> Result<Vec<SlotValue>, DriverError> {
    let body_len = selected.descriptor().body_len as usize;
    if statement.len() != HEADER_LEN + body_len {
        return Err(DriverError::Input(format!("statement length {} differs from 343 + {body_len}", statement.len())));
    }
    let mut at = HEADER_LEN;
    let mut out = Vec::new();
    for slot in &selected.relation().layout().slots {
        if slot.kind.is_header() {
            continue;
        }
        let (encoding, width) = match slot.encoding {
            SlotEncoding::UInt => ("uint128", 16),
            SlotEncoding::Bool => ("bool", 1),
            SlotEncoding::Wide => ("wide144", 18),
            other => return Err(DriverError::Backend(format!("slot {} has body encoding {other:?}", slot.label))),
        };
        let bytes = statement
            .get(at..at + width)
            .ok_or_else(|| DriverError::Input(format!("statement too short for slot {}", slot.label)))?;
        let value = match encoding {
            "uint128" => {
                let mut b = [0u8; 16];
                b.copy_from_slice(bytes);
                Json::String(u128::from_be_bytes(b).to_string())
            }
            "bool" => match bytes[0] {
                0 => Json::Bool(false),
                1 => Json::Bool(true),
                x => return Err(DriverError::Input(format!("slot {} byte {x} is not a Bool", slot.label))),
            },
            _ => {
                let top = u16::from_be_bytes([bytes[0], bytes[1]]);
                let mut b = [0u8; 16];
                b.copy_from_slice(&bytes[2..18]);
                let lo = u128::from_be_bytes(b);
                Json::String(if top == 0 { lo.to_string() } else { format!("{top}*2^128+{lo}") })
            }
        };
        out.push(SlotValue { label: slot.label.clone(), encoding, value });
        at += width;
    }
    if at != statement.len() {
        return Err(DriverError::Backend("layout body slots do not cover the statement body".into()));
    }
    Ok(out)
}

/// The receipt of an already-prepared claim.
pub fn receipt(selected: &SelectedRelation, claim: &PreparedClaim) -> Result<StatementReceipt, DriverError> {
    let fixed_matrices = FixedMatrices::of(selected)?;
    let assignment = fixed::prove_assignment(selected, claim, Tamper::none(), true)?;
    if assignment.overrides_used != 0 {
        return Err(DriverError::Backend("a tamper override reached a witness in a receipt synthesis".into()));
    }
    let evaluation = fixed_matrices.evaluate(&assignment.full_assignment)?;
    let d = selected.descriptor();
    Ok(StatementReceipt {
        format: RECEIPT_FORMAT,
        notice: RECEIPT_NOTICE,
        action: d.action.clone(),
        program_hash: d.program_hash.clone(),
        predicate_digest: d.predicate_digest.clone(),
        abi2_shape_digest: d.abi2_shape_digest.clone(),
        r1cs_sha256: d.r1cs_sha256.clone(),
        declared_vk_digest: hexutil::encode(&claim.context.vk_digest),
        statement_len: claim.statement.len(),
        statement_hex: hexutil::encode(&claim.statement),
        statement_digest_blake2b256: hexutil::encode(&claim.statement_digest),
        x1: claim.public_inputs[0].to_string(),
        x2: claim.public_inputs[1].to_string(),
        slots: decode_body(selected, &claim.statement)?,
        fixed_matrix: FixedMatrixCheck {
            r1cs_sha256: fixed_matrices.digest_hex().to_string(),
            equals_descriptor: fixed_matrices.digest_hex() == d.r1cs_sha256,
            prove_matrix_digest_equal: assignment.matrix_digest_hex.as_deref() == Some(fixed_matrices.digest_hex()),
            constraints: evaluation.constraints,
            violated: evaluation.violated,
            first_violated: evaluation.first_violated,
        },
    })
}

/// Proposes the request's claim with a declared VK digest and builds its receipt. Refuses a
/// request whose selector is not the one `selected` was compiled with.
pub fn inspect(selected: &SelectedRelation, request: &ProveRequest, vk_digest: [u8; 32]) -> Result<StatementReceipt, DriverError> {
    if request.selector != *selected.selector() {
        return Err(DriverError::mismatch("request selector", "the selector the relation was compiled with", "a different selector"));
    }
    let claim = claim::prepare_claim(selected, &request.transition, &request.context, vk_digest)?;
    receipt(selected, &claim)
}
