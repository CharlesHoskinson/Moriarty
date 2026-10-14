//! Versioned canonical owner intent for the native core.
//! Signature checks establish possession of a key, not ledger authority or proof validity.
use crate::{
    AssetId, Effect, Error, Execution, Liability, Program, SEMANTICS, Scenario, UInt, simulate,
};
use k256::{ecdsa, schnorr};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub const INTENT_PROFILE: &str = "moriarty-owner-intent/2";
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Scheme {
    SchnorrBip340,
    EcdsaSecp256k1Sha256,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssetLimit {
    pub asset: AssetId,
    pub beneficiary: String,
    pub gross_cap: UInt,
    pub fee_cap: UInt,
    pub net_floor: UInt,
    pub liability_cap: UInt,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Authorization {
    pub network: String,
    pub anchor: String,
    pub program_address: String,
    pub target_hash: String,
    pub signer: String,
    pub public_key_hex: String,
    pub scheme: Scheme,
    pub nonce: UInt,
    pub predecessor: String,
    pub not_before: UInt,
    pub not_after: UInt,
    pub limits: Vec<AssetLimit>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct OwnerIntent {
    pub profile: String,
    pub source_hash: String,
    pub program_hash: String,
    pub semantics: String,
    pub action: String,
    pub scenario_hash: String,
    pub assets: Vec<AssetId>,
    pub authority: Authorization,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreparedIntent {
    pub intent: OwnerIntent,
    pub signing_message_hex: String,
    pub signing_message_sha256: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SignedIntent {
    pub intent: OwnerIntent,
    pub signature_hex: String,
}

pub fn hash_bytes(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}
pub fn canonical<T: Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    fn sort(v: serde_json::Value) -> serde_json::Value {
        match v {
            serde_json::Value::Object(map) => {
                let sorted: std::collections::BTreeMap<_, _> =
                    map.into_iter().map(|(k, v)| (k, sort(v))).collect();
                serde_json::Value::Object(sorted.into_iter().collect())
            }
            serde_json::Value::Array(a) => {
                serde_json::Value::Array(a.into_iter().map(sort).collect())
            }
            v => v,
        }
    }
    let value = serde_json::to_value(value).map_err(|e| Error::new("intent", e.to_string()))?;
    serde_json::to_vec(&sort(value)).map_err(|e| Error::new("intent", e.to_string()))
}
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn unhex(text: &str, len: usize) -> Result<Vec<u8>, Error> {
    if text.len() != len * 2
        || !text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(Error::new(
            "signature",
            "noncanonical signature/key encoding",
        ));
    }
    (0..text.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&text[i..i + 2], 16)
                .map_err(|_| Error::new("signature", "invalid hex"))
        })
        .collect()
}
fn check_key(authority: &Authorization) -> Result<(), Error> {
    match authority.scheme {
        Scheme::SchnorrBip340 => {
            schnorr::VerifyingKey::from_bytes(&unhex(&authority.public_key_hex, 32)?)
                .map_err(|_| Error::new("signature", "invalid BIP340 public key"))?;
        }
        Scheme::EcdsaSecp256k1Sha256 => {
            let key = unhex(&authority.public_key_hex, 33)?;
            if !matches!(key[0], 2 | 3) {
                return Err(Error::new("signature", "expected compressed SEC1 key"));
            }
            ecdsa::VerifyingKey::from_sec1_bytes(&key)
                .map_err(|_| Error::new("signature", "invalid ECDSA public key"))?;
        }
    }
    Ok(())
}
pub fn prepare_intent(
    program: &Program,
    source_hash: &str,
    action: &str,
    scenario: &Scenario,
    authority: &Authorization,
) -> Result<PreparedIntent, Error> {
    crate::validate(program)?;
    if !program.actions.iter().any(|a| a.name == action) {
        return Err(Error::new("intent", "unknown action"));
    }
    if source_hash.len() != 64
        || !source_hash
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        || [
            &authority.network,
            &authority.anchor,
            &authority.program_address,
            &authority.target_hash,
            &authority.signer,
            &authority.predecessor,
        ]
        .iter()
        .any(|s| s.is_empty() || s.len() > 1024 || s.chars().any(char::is_control))
        || authority.not_after < authority.not_before
        || authority.limits.len() > 256
    {
        return Err(Error::new(
            "intent",
            "invalid source/domain/validity binding",
        ));
    }
    let mut assets: Vec<_> = program.assets.iter().map(|a| a.identity.clone()).collect();
    assets.sort();
    let mut seen = BTreeSet::new();
    for limit in &authority.limits {
        if !assets.contains(&limit.asset)
            || limit.asset.network != authority.network
            || limit.beneficiary.is_empty()
            || !seen.insert(limit.asset.clone())
        {
            return Err(Error::new(
                "intent",
                "duplicate or mismatched asset authority",
            ));
        }
    }
    check_key(authority)?;
    let intent = OwnerIntent {
        profile: INTENT_PROFILE.into(),
        source_hash: source_hash.into(),
        program_hash: hash_bytes(&canonical(program)?),
        semantics: SEMANTICS.into(),
        action: action.into(),
        scenario_hash: hash_bytes(&canonical(scenario)?),
        assets,
        authority: authority.clone(),
    };
    let message = signing_message(&intent)?;
    Ok(PreparedIntent {
        intent,
        signing_message_hex: hex(&message),
        signing_message_sha256: hash_bytes(&message),
    })
}
pub fn signing_message(intent: &OwnerIntent) -> Result<Vec<u8>, Error> {
    let payload = canonical(intent)?;
    if payload.len() > 65536 {
        return Err(Error::new("intent", "canonical intent exceeds 64 KiB"));
    }
    let mut bytes = b"moriarty-owner-intent/2\0".to_vec();
    bytes.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    bytes.extend(payload);
    Ok(bytes)
}
pub fn verify_intent(expected: &OwnerIntent, signed: &SignedIntent, at: UInt) -> Result<(), Error> {
    if expected != &signed.intent
        || expected.profile != INTENT_PROFILE
        || expected.semantics != SEMANTICS
    {
        return Err(Error::new("intent", "signed intent binding mismatch"));
    }
    if at < expected.authority.not_before || at > expected.authority.not_after {
        return Err(Error::new("intent", "intent is outside validity interval"));
    }
    check_key(&expected.authority)?;
    let sig = unhex(&signed.signature_hex, 64)?;
    let message = signing_message(expected)?;
    match expected.authority.scheme {
        Scheme::SchnorrBip340 => {
            let key =
                schnorr::VerifyingKey::from_bytes(&unhex(&expected.authority.public_key_hex, 32)?)
                    .map_err(|_| Error::new("signature", "invalid BIP340 key"))?;
            let signature = schnorr::Signature::try_from(sig.as_slice())
                .map_err(|_| Error::new("signature", "invalid BIP340 signature"))?;
            // Midnight's native Verifier hashes the canonical message with SHA-256.
            use k256::schnorr::signature::Verifier;
            key.verify(&message, &signature)
                .map_err(|_| Error::new("signature", "signature rejected"))?;
        }
        Scheme::EcdsaSecp256k1Sha256 => {
            use k256::ecdsa::signature::Verifier;
            let key = ecdsa::VerifyingKey::from_sec1_bytes(&unhex(
                &expected.authority.public_key_hex,
                33,
            )?)
            .map_err(|_| Error::new("signature", "invalid ECDSA key"))?;
            let signature = ecdsa::Signature::from_slice(&sig)
                .map_err(|_| Error::new("signature", "invalid ECDSA signature"))?;
            if signature.normalize_s().is_some() {
                return Err(Error::new(
                    "signature",
                    "noncanonical high-S ECDSA signature",
                ));
            }
            key.verify(&message, &signature)
                .map_err(|_| Error::new("signature", "signature rejected"))?;
        }
    }
    Ok(())
}

/// Check all owner debits, fees, net goals and closing liabilities by exact asset identity.
pub fn check_effects(
    signer: &str,
    limits: &[AssetLimit],
    effects: &[Effect],
    closing: &[Liability],
) -> Result<(), Error> {
    let mut unique = BTreeSet::new();
    for l in limits {
        if !unique.insert(&l.asset) {
            return Err(Error::new("authority", "duplicate asset limit"));
        }
    }
    for effect in effects {
        let touched = match effect {
            Effect::Transfer { asset, from, .. } | Effect::Fee { asset, from, .. }
                if from == signer =>
            {
                Some(asset)
            }
            Effect::Liability { asset, debtor, .. } if debtor == signer => Some(asset),
            _ => None,
        };
        if touched.is_some_and(|asset| !limits.iter().any(|l| &l.asset == asset)) {
            return Err(Error::new("authority", "effect has no asset authority"));
        }
    }
    for liability in closing {
        if liability.debtor == signer && !limits.iter().any(|l| l.asset == liability.asset) {
            return Err(Error::new(
                "authority",
                "closing liability has no authority",
            ));
        }
    }
    for limit in limits {
        let mut gross = 0_u128;
        let mut fees = 0_u128;
        let mut credit = 0_u128;
        let mut debit = 0_u128;
        let add = |a: &mut u128, b: u128| -> Result<(), Error> {
            *a = a
                .checked_add(b)
                .ok_or_else(|| Error::new("authority", "accounting overflow"))?;
            Ok(())
        };
        for effect in effects {
            match effect {
                Effect::Transfer {
                    asset,
                    from,
                    to,
                    amount,
                } if asset == &limit.asset => {
                    if from == signer {
                        add(&mut gross, amount.0)?;
                    }
                    if to == &limit.beneficiary {
                        add(&mut credit, amount.0)?;
                    }
                    if from == &limit.beneficiary {
                        add(&mut debit, amount.0)?;
                    }
                }
                Effect::Fee {
                    asset,
                    from,
                    to: _,
                    amount,
                } if asset == &limit.asset => {
                    if from == signer {
                        add(&mut gross, amount.0)?;
                        add(&mut fees, amount.0)?;
                    }
                    if from == &limit.beneficiary && from != signer {
                        add(&mut debit, amount.0)?;
                    }
                }
                _ => {}
            }
        }
        let obligations = closing
            .iter()
            .filter(|l| l.asset == limit.asset && l.debtor == signer)
            .try_fold(0_u128, |a, l| a.checked_add(l.amount.0))
            .ok_or_else(|| Error::new("authority", "liability overflow"))?;
        let net = credit.checked_sub(debit).and_then(|v| v.checked_sub(fees));
        if gross > limit.gross_cap.0
            || fees > limit.fee_cap.0
            || obligations > limit.liability_cap.0
            || net.is_none_or(|n| n < limit.net_floor.0)
        {
            return Err(Error::new(
                "authority",
                "gross, fee, net or liability constraint exceeded",
            ));
        }
    }
    Ok(())
}
pub fn verify_and_simulate(
    program: &Program,
    source_hash: &str,
    action: &str,
    scenario: &Scenario,
    authority: &Authorization,
    signed: &SignedIntent,
    at: UInt,
) -> Result<Execution, Error> {
    let expected = prepare_intent(program, source_hash, action, scenario, authority)?;
    verify_intent(&expected.intent, signed, at)?;
    let execution = simulate(program, action, scenario)?;
    check_effects(
        &authority.signer,
        &authority.limits,
        &execution.effects,
        &execution.post.liabilities,
    )?;
    Ok(execution)
}

/// Authenticates before metered local execution, then checks complete effects.
/// The caller's unsigned cap is outside the unchanged owner signing message.
/// Preparation, signature and effect checks are outside this logical cost profile.
/// Late authority refusal retains consumed work and exposes no execution.
pub fn verify_and_simulate_metered(
    program: &Program,
    source_hash: &str,
    action: &str,
    scenario: &Scenario,
    authority: &Authorization,
    signed: &SignedIntent,
    at: UInt,
    cap: UInt,
) -> Result<crate::MeteredExecution, crate::MeteredExecutionError> {
    let before_execution = |error| crate::MeteredExecutionError {
        error,
        cost: crate::CostReport {
            profile: crate::LOGICAL_COST_PROFILE.into(),
            cap,
            used: UInt(0),
        },
    };
    let expected = prepare_intent(program, source_hash, action, scenario, authority)
        .map_err(before_execution)?;
    verify_intent(&expected.intent, signed, at).map_err(before_execution)?;
    let metered = crate::simulate_metered(program, action, scenario, cap)?;
    check_effects(
        &authority.signer,
        &authority.limits,
        &metered.execution.effects,
        &metered.execution.post.liabilities,
    )
    .map_err(|error| crate::MeteredExecutionError {
        error,
        cost: metered.cost.clone(),
    })?;
    Ok(metered)
}
