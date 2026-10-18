//! Bounded public sidecars reconstructed from caller-trusted inputs.
//! Local reference replay only; no proof, parameters or private witness path.
use crate::{
    ARTIFACT_FORMAT, Error, Target, UNSIGNED_MODE,
    authorization::{self, ExpectedAuthority, SignedIntent},
    error, proving, signed_relation,
};
use midnight_transient_crypto::proofs::Zkir;
use midnight_zkir_v3::IrSource;
use mori_core::{Execution, ReplayDifference, Scenario, UInt};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const RECORD_FORMAT: &str = "moriarty-reference-replay/1";
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayRecord {
    pub body: ReplayBody,
    pub record_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayBody {
    pub format: String,
    pub language: String,
    pub semantics: String,
    pub network: String,
    pub anchor: String,
    pub program_name: String,
    #[serde(deserialize_with = "Option::deserialize")]
    pub program_address: Option<String>,
    pub artifact_format: String,
    pub zkir_hash: String,
    #[serde(deserialize_with = "Option::deserialize")]
    pub authority_hash: Option<String>,
    pub input_hash: String,
    pub trace_hash: String,
    pub source_step_count: u32,
    pub claim: proving::TransitionClaim,
    pub execution: Execution,
}
#[derive(Debug, Clone, Copy)]
pub enum ReplayAuthorization<'a> {
    Unsigned,
    Signed {
        authority: &'a ExpectedAuthority,
        at: UInt,
        signature: &'a SignedIntent,
    },
}
#[derive(Debug, Clone, Serialize, thiserror::Error)]
#[serde(tag = "kind", content = "detail", rename_all = "snake_case")]
pub enum ReplayError {
    #[error("{0}")]
    Input(Error),
    #[error("Reference divergence at {}", .0.field)]
    Divergence(ReplayDifference),
}
fn bounded(value: &impl Serialize) -> Result<(), Error> {
    mori_core::json::export(value).map_err(|e| error("limit", e.to_string()))?;
    Ok(())
}
/// The new sidecar framing is bounded; old v3 preimages are unchanged.
fn public_hash(value: &impl Serialize) -> Result<String, Error> {
    bounded(value)?;
    let bytes =
        mori_core::intent::canonical(value).map_err(|e| error("serialization", e.to_string()))?;
    let text = std::str::from_utf8(&bytes).map_err(|e| error("serialization", e.to_string()))?;
    mori_core::json::parse(text).map_err(|e| error("limit", e.to_string()))?;
    Ok(mori_core::intent::hash_bytes(&bytes))
}
impl ReplayRecord {
    /// Strict schema/encoding validation only: hashes never hide semantic divergence.
    pub fn from_json(text: &str) -> Result<Self, Error> {
        let value = mori_core::json::parse(text).map_err(|e| error("json", e.to_string()))?;
        // Sidecar-only required presence; the legacy Step decoder remains unchanged.
        if let Some(steps) = value
            .pointer("/body/execution/steps")
            .and_then(Value::as_array)
        {
            for step in steps {
                if !step.as_object().is_some_and(|o| o.contains_key("value")) {
                    return Err(error(
                        "json",
                        "Replay Step requires an explicit value field.",
                    ));
                }
            }
        }
        let record: Self =
            serde_json::from_value(value).map_err(|e| error("json", e.to_string()))?;
        record.validate()?;
        Ok(record)
    }
    fn validate(&self) -> Result<(), Error> {
        bounded(self)?;
        let b = &self.body;
        if b.format != RECORD_FORMAT
            || b.language != mori_core::LANGUAGE
            || b.semantics != mori_core::SEMANTICS
            || b.artifact_format != ARTIFACT_FORMAT
        {
            return Err(error("replay", "Unsupported reference record revision."));
        }
        if b.source_step_count as usize > mori_core::MAX_STATEMENTS
            || b.execution.steps.len() > mori_core::MAX_STATEMENTS
        {
            return Err(error(
                "limit",
                "Replay source-step count exceeds Core bounds.",
            ));
        }
        for d in [
            &self.record_hash,
            &b.zkir_hash,
            &b.input_hash,
            &b.trace_hash,
        ] {
            authorization::decode_digest(d)?;
        }
        if let Some(d) = &b.authority_hash {
            authorization::decode_digest(d)?;
        }
        b.claim.validate()?;
        match b.claim.authorization_mode.as_str() {
            UNSIGNED_MODE if b.program_address.is_none() && b.authority_hash.is_none() => {}
            authorization::INTENT_PROFILE
                if b.program_address.is_some() && b.authority_hash.is_some() => {}
            _ => {
                return Err(error(
                    "replay",
                    "Record public identities disagree with authorization mode.",
                ));
            }
        }
        Ok(())
    }
}
/// Generate a full public record after trusted source/Target/attachment admission.
/// Statement count and circuit k are observations, not work or native certificates.
pub fn generate(
    source: &str,
    action: &str,
    scenario: &Scenario,
    target: &Target,
    authorization: ReplayAuthorization<'_>,
) -> Result<ReplayRecord, Error> {
    // Direct typed API inputs receive the same public bounds as file callers.
    bounded(scenario)?;
    bounded(target)?;
    let (statement, authority_hash) = match authorization {
        ReplayAuthorization::Unsigned => (
            signed_relation::reconstruct(source, action, scenario, target, None)?,
            None,
        ),
        ReplayAuthorization::Signed {
            authority,
            at,
            signature,
        } => {
            bounded(authority)?;
            bounded(signature)?;
            let statement = signed_relation::accept(
                source, action, scenario, target, authority, at, signature,
            )?;
            (statement, Some(public_hash(authority)?))
        }
    };
    let ir = IrSource::load(statement.circuit.zkir.as_bytes())
        .map_err(|e| error("zkir", e.to_string()))?;
    let claim = proving::claim_from_reconstruction(&statement, action, scenario, target, ir.k())?;
    let at = claim.authorization.as_ref().map(|a| a.at);
    let input_hash = public_hash(&(
        "moriarty-reference-input/1",
        &claim.source_hash,
        &claim.program_hash,
        &claim.action,
        &claim.scenario_hash,
        &claim.target_hash,
        &authority_hash,
        at,
    ))?;
    let trace_hash = public_hash(&("moriarty-reference-trace/1", &statement.execution))?;
    let body = ReplayBody {
        format: RECORD_FORMAT.into(),
        language: statement.language,
        semantics: statement.semantics,
        network: target.network.clone(),
        anchor: target.anchor.clone(),
        program_name: statement.program_name,
        program_address: target
            .signed_policy
            .as_ref()
            .map(|p| p.program_address.clone()),
        artifact_format: ARTIFACT_FORMAT.into(),
        zkir_hash: mori_core::intent::hash_bytes(statement.circuit.zkir.as_bytes()),
        authority_hash,
        input_hash,
        trace_hash,
        source_step_count: statement.execution.steps.len() as u32,
        claim,
        execution: statement.execution,
    };
    let record_hash = public_hash(&(RECORD_FORMAT, &body))?;
    let record = ReplayRecord { body, record_hash };
    record.validate()?;
    Ok(record)
}
fn unequal(actual: &Value, candidate: &Value, path: &str) -> Result<(), ReplayError> {
    if actual != candidate {
        return Err(ReplayError::Divergence(ReplayDifference {
            step: None,
            field: path.into(),
        }));
    }
    Ok(())
}
/// Replay from independent caller roots, comparing semantic fields before derived hashes.
pub fn replay(
    source: &str,
    action: &str,
    scenario: &Scenario,
    target: &Target,
    authorization: ReplayAuthorization<'_>,
    candidate: &ReplayRecord,
) -> Result<(), ReplayError> {
    candidate.validate().map_err(ReplayError::Input)?;
    let actual =
        generate(source, action, scenario, target, authorization).map_err(ReplayError::Input)?;
    compare_records(&actual, candidate)
}
// Keep one comparison order for ordinary replay and challenge export.
fn compare_records(actual: &ReplayRecord, candidate: &ReplayRecord) -> Result<(), ReplayError> {
    let a = serde_json::to_value(actual)
        .map_err(|e| ReplayError::Input(error("serialization", e.to_string())))?;
    let b = serde_json::to_value(candidate)
        .map_err(|e| ReplayError::Input(error("serialization", e.to_string())))?;
    for field in [
        "format",
        "language",
        "semantics",
        "network",
        "anchor",
        "program_name",
        "program_address",
        "artifact_format",
        "zkir_hash",
        "authority_hash",
        "input_hash",
    ] {
        unequal(
            &a["body"][field],
            &b["body"][field],
            &format!("/body/{field}"),
        )?;
    }
    for field in [
        "abi",
        "relation",
        "binding",
        "profile",
        "registry",
        "format",
        "source_hash",
        "program_hash",
        "target_hash",
        "scenario_hash",
        "action",
        "circuit_k",
        "authorization_mode",
        "context_hash",
        "financial_bindings",
        "authorization_bindings",
    ] {
        unequal(
            &a["body"]["claim"][field],
            &b["body"]["claim"][field],
            &format!("/body/claim/{field}"),
        )?;
    }
    if actual.body.claim.authorization.is_some() {
        for field in ["payload_hash", "public_key_hex", "at"] {
            unequal(
                &a["body"]["claim"]["authorization"][field],
                &b["body"]["claim"]["authorization"][field],
                &format!("/body/claim/authorization/{field}"),
            )?;
        }
    }
    mori_core::compare_execution(&actual.body.execution, &candidate.body.execution).map_err(
        |mut d| {
            d.field = format!("/body/execution{}", d.field);
            ReplayError::Divergence(d)
        },
    )?;
    for field in ["execution_hash", "public_values"] {
        unequal(
            &a["body"]["claim"][field],
            &b["body"]["claim"][field],
            &format!("/body/claim/{field}"),
        )?;
    }
    if actual.body.claim.authorization.is_some() {
        unequal(
            &a["body"]["claim"]["authorization"]["message_hex"],
            &b["body"]["claim"]["authorization"]["message_hex"],
            "/body/claim/authorization/message_hex",
        )?;
    }
    for field in ["source_step_count", "trace_hash"] {
        unequal(
            &a["body"][field],
            &b["body"][field],
            &format!("/body/{field}"),
        )?;
    }
    unequal(&a["record_hash"], &b["record_hash"], "/record_hash")
}

pub const CHALLENGE_FORMAT: &str = "moriarty-reference-challenge/1";
/// Local diagnostic output cap, including ASCII escapes and the final newline.
/// This is not a Fenrir transaction limit or a measured replay certificate.
pub const MAX_CHALLENGE_BYTES: usize = 4096;

/// Public identities copied exclusively from independent reconstruction.
/// The input and record hashes also bind the attachment-free signed statement.
/// Signature bytes, R, s, proving keys and private witnesses are excluded.
#[derive(Debug, Clone, Serialize)]
pub struct ReplayChallengeContext {
    pub language: String,
    pub semantics: String,
    pub network: String,
    pub anchor: String,
    pub program_name: String,
    pub program_address: Option<String>,
    pub action: String,
    pub source_hash: String,
    pub program_hash: String,
    pub target_hash: String,
    pub scenario_hash: String,
    pub zkir_hash: String,
    pub authority_hash: Option<String>,
    pub authorization_mode: String,
    pub at: Option<UInt>,
    pub input_hash: String,
    pub trace_hash: String,
    pub record_hash: String,
}
/// A bounded first-divergence diagnostic, not a proof or fraud-game transcript.
/// The candidate digest hashes the entire canonical typed record, including
/// its possibly false declared hashes. It never trusts its record_hash label.
#[derive(Debug, Clone, Serialize)]
pub struct ReplayChallenge {
    pub format: String,
    pub qualification: String,
    pub trusted: ReplayChallengeContext,
    pub candidate_digest: String,
    pub divergence: ReplayDifference,
}
impl ReplayChallenge {
    /// Prepare complete bytes before any file creation or publication.
    pub fn to_json(&self) -> Result<String, Error> {
        let text = mori_core::json::export(self).map_err(|e| error("limit", e.to_string()))?;
        if text.len() >= MAX_CHALLENGE_BYTES {
            return Err(error(
                "limit",
                "Escaped challenge and final newline exceed 4096 bytes.",
            ));
        }
        Ok(text)
    }
}
/// Reconstruct once from separate roots and use the ordinary replay order.
/// Equality returns no challenge. Invalid input returns no diagnostic record.
/// Exporters must call to_json before publication to enforce the output cap.
pub fn challenge(
    source: &str,
    action: &str,
    scenario: &Scenario,
    target: &Target,
    authorization: ReplayAuthorization<'_>,
    candidate: &ReplayRecord,
) -> Result<Option<ReplayChallenge>, ReplayError> {
    candidate.validate().map_err(ReplayError::Input)?;
    let actual =
        generate(source, action, scenario, target, authorization).map_err(ReplayError::Input)?;
    let divergence = match compare_records(&actual, candidate) {
        Ok(()) => return Ok(None),
        Err(ReplayError::Divergence(d)) => d,
        Err(error) => return Err(error),
    };
    let candidate_digest = public_hash(candidate).map_err(ReplayError::Input)?;
    let b = actual.body;
    let claim = b.claim;
    let trusted = ReplayChallengeContext {
        language: b.language,
        semantics: b.semantics,
        network: b.network,
        anchor: b.anchor,
        program_name: b.program_name,
        program_address: b.program_address,
        action: claim.action,
        source_hash: claim.source_hash,
        program_hash: claim.program_hash,
        target_hash: claim.target_hash,
        scenario_hash: claim.scenario_hash,
        zkir_hash: b.zkir_hash,
        authority_hash: b.authority_hash,
        authorization_mode: claim.authorization_mode,
        at: claim.authorization.map(|a| a.at),
        input_hash: b.input_hash,
        trace_hash: b.trace_hash,
        record_hash: actual.record_hash,
    };
    Ok(Some(ReplayChallenge {
        format: CHALLENGE_FORMAT.into(),
        qualification: "local-reference-replay-diagnostic".into(),
        trusted,
        candidate_digest,
        divergence,
    }))
}
