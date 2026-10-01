//! Closed owner-intent protocol. Signature validity does not establish account authority.
use crate::{codec, verify, wallet_message};
use midnight_base_crypto::{ecdsa, hash::persistent_hash, schnorr};
use midnight_serialize::Deserializable;
use serde_json::{Value, json};
#[derive(Debug)]
pub struct IntentError {
    pub code: &'static str,
    pub error: String,
}
type R<T> = Result<T, IntentError>;
fn err(code: &'static str, error: impl Into<String>) -> IntentError {
    IntentError {
        code,
        error: error.into(),
    }
}
fn object<'a>(v: &'a Value, keys: &[&str]) -> R<&'a serde_json::Map<String, Value>> {
    let m = v
        .as_object()
        .ok_or_else(|| err("schema", "expected object"))?;
    if m.len() != keys.len() || keys.iter().any(|k| !m.contains_key(*k)) {
        return Err(err("schema", "missing or unknown object field"));
    }
    Ok(m)
}
fn text(v: &Value) -> R<&str> {
    let s = v.as_str().ok_or_else(|| err("schema", "expected string"))?;
    if s.is_empty() || s.len() > 1024 || s.chars().any(|c| c <= '\u{1f}' || c == '\u{7f}') {
        return Err(err("schema", "invalid bounded scalar text"));
    }
    Ok(s)
}
fn id(v: &Value) -> R<()> {
    let s = text(v)?;
    let b = s.as_bytes();
    if b.len() > 64
        || !b[0].is_ascii_alphabetic()
        || b.iter().any(|c| !c.is_ascii_alphanumeric() && *c != b'_')
    {
        return Err(err("schema", "invalid identifier"));
    }
    Ok(())
}
fn constant(v: &Value, want: &str) -> R<()> {
    if v.as_str() != Some(want) {
        Err(err("schema", format!("expected {want}")))
    } else {
        Ok(())
    }
}
fn decimal(v: &Value, max: u128) -> R<u128> {
    let s = text(v)?;
    if s.len() > 39 || !s.bytes().all(|b| b.is_ascii_digit()) || (s.len() > 1 && s.starts_with('0'))
    {
        return Err(err("range", "noncanonical decimal"));
    }
    let n = s
        .parse::<u128>()
        .map_err(|_| err("range", "integer overflow"))?;
    if n > max {
        return Err(err("range", "integer out of range"));
    }
    Ok(n)
}
fn hexbytes(v: &Value, n: usize, code: &'static str) -> R<Vec<u8>> {
    let s = v
        .as_str()
        .ok_or_else(|| err(code, "expected hexadecimal string"))?;
    if s.len() != n * 2
        || !s
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(err(code, "incorrect lowercase hexadecimal encoding"));
    }
    hex::decode(s).map_err(|_| err(code, "invalid hex"))
}
fn canonical(v: &Value) -> R<Vec<u8>> {
    fn sorted(v: &Value) -> Value {
        match v {
            Value::Object(m) => {
                let mut keys: Vec<_> = m.keys().collect();
                keys.sort();
                Value::Object(
                    keys.into_iter()
                        .map(|k| (k.clone(), sorted(&m[k])))
                        .collect(),
                )
            }
            Value::Array(a) => Value::Array(a.iter().map(sorted).collect()),
            _ => v.clone(),
        }
    }
    serde_json::to_vec(&sorted(v)).map_err(|e| err("schema", e.to_string()))
}
fn frame(domain: &[u8], v: &Value) -> R<Vec<u8>> {
    let p = canonical(v)?;
    let len = u32::try_from(p.len()).map_err(|_| err("range", "payload length overflow"))?;
    let mut b = domain.to_vec();
    b.extend_from_slice(&len.to_be_bytes());
    b.extend_from_slice(&p);
    Ok(b)
}
fn projection(v: &Value) -> Value {
    let mut m = serde_json::Map::new();
    for k in [
        "authoringProfile",
        "core",
        "sourceProfile",
        "agreementId",
        "actionName",
        "selectedActionId",
        "domain",
        "asset",
        "intent",
    ] {
        m.insert(k.into(), v[k].clone());
    }
    Value::Object(m)
}
fn validate(v: &Value, build: bool) -> R<()> {
    let mut keys = vec![
        "profile",
        "core",
        "sourceProfile",
        "authoringProfile",
        "agreementId",
        "actionName",
        "selectedActionId",
        "sourceSha256",
        "domain",
        "asset",
        "signature",
        "intent",
    ];
    if !build {
        keys.push("ownerProgramSha256");
    }
    object(v, &keys)?;
    for (k, w) in [
        ("profile", "moriarty-signed-intent/1"),
        ("core", "moriarty-core/5"),
        ("sourceProfile", "moriarty-financial-agreement-source/6"),
        ("authoringProfile", "moriarty-beta/1"),
    ] {
        constant(&v[k], w)?;
    }
    id(&v["agreementId"])?;
    id(&v["actionName"])?;
    hexbytes(&v["sourceSha256"], 32, "hash")?;
    let d = &v["domain"];
    object(d, &["id", "chain", "network"])?;
    id(&d["id"])?;
    text(&d["chain"])?;
    text(&d["network"])?;
    let a = &v["asset"];
    object(a, &["id", "representation", "scale", "symbol"])?;
    id(&a["id"])?;
    text(&a["representation"])?;
    decimal(&a["scale"], 18)?;
    if !a["symbol"].is_null() {
        text(&a["symbol"])?;
    }
    let sig = &v["signature"];
    object(sig, &["scheme", "publicKeyHex", "keyRef", "framing"])?;
    text(&sig["keyRef"])?;
    let scheme = text(&sig["scheme"])?;
    let key = match scheme {
        "schnorr_bip340" => hexbytes(&sig["publicKeyHex"], 32, "key")?,
        "ecdsa_secp256k1_sha256" => hexbytes(&sig["publicKeyHex"], 33, "key")?,
        _ => return Err(err("schema", "unsupported signature scheme")),
    };
    match scheme {
        "schnorr_bip340" => {
            schnorr::VerifyingKey::deserialize(&mut &key[..], 0)
                .map_err(|e| err("key", e.to_string()))?;
        }
        _ => {
            // Some upstream point decoders accept alternative SEC1 tag bytes.
            // The signing protocol permits only the canonical compressed tags.
            if !matches!(key[0], 2 | 3) {
                return Err(err("key", "ECDSA key requires canonical SEC1 prefix 02 or 03"));
            }
            ecdsa::VerifyingKey::deserialize(&mut &key[..], 0)
                .map_err(|e| err("key", e.to_string()))?;
        }
    };
    if !matches!(sig["framing"].as_str(), Some("raw" | "midnight-sign-data")) {
        return Err(err("schema", "unsupported framing"));
    }
    let i = &v["intent"];
    object(
        i,
        &[
            "version",
            "advertisedSourceHash",
            "advertisedPolicyDigest",
            "signer",
            "nonce",
            "preHead",
            "notBefore",
            "notAfter",
            "grossCap",
            "feeCap",
            "netFloor",
            "failure",
            "observations",
            "disclosures",
            "retainedEffects",
            "retainedDuties",
            "delegation",
            "recovery",
            "operation",
        ],
    )?;
    constant(&i["version"], "moriarty-intent/3")?;
    constant(&i["failure"], "success_only")?;
    constant(&i["delegation"], "none")?;
    constant(&i["recovery"], "none")?;
    for k in [
        "observations",
        "disclosures",
        "retainedEffects",
        "retainedDuties",
    ] {
        if i[k].as_array().is_none_or(|a| !a.is_empty()) {
            return Err(err("schema", "policy collection must be empty"));
        }
    }
    for k in [
        "advertisedSourceHash",
        "advertisedPolicyDigest",
        "nonce",
        "preHead",
    ] {
        text(&i[k])?;
    }
    id(&i["signer"])?;
    if decimal(&i["notBefore"], u128::MAX)? > decimal(&i["notAfter"], u128::MAX)? {
        return Err(err("range", "reversed validity"));
    }
    let money = i128::MAX as u128;
    let gross = decimal(&i["grossCap"], money)?;
    let cap = decimal(&i["feeCap"], money)?;
    let floor = decimal(&i["netFloor"], money)?;
    let o = &i["operation"];
    let kind = o["kind"]
        .as_str()
        .ok_or_else(|| err("schema", "missing operation kind"))?;
    let amount = decimal(&o["amount"], money)?;
    if amount == 0 {
        return Err(err("range", "amount must be positive"));
    }
    match kind {
        "Transfer" => {
            object(
                o,
                &["kind", "from", "recipient", "feeRecipient", "amount", "fee"],
            )?;
            constant(&v["selectedActionId"], "TransferLiteralFee")?;
            for k in ["from", "recipient", "feeRecipient"] {
                id(&o[k])?;
            }
            if o["from"] != i["signer"]
                || o["from"] == o["recipient"]
                || o["from"] == o["feeRecipient"]
                || o["recipient"] == o["feeRecipient"]
            {
                return Err(err("schema", "transfer identity mismatch or alias"));
            }
            let fee = decimal(&o["fee"], money)?;
            let total = amount
                .checked_add(fee)
                .filter(|x| *x <= money)
                .ok_or_else(|| err("range", "gross overflow"))?;
            if total > gross || fee > cap || amount < floor {
                return Err(err("range", "transfer exceeds owner bounds"));
            }
        }
        "Repay" => {
            object(
                o,
                &["kind", "payer", "obligationId", "amount", "conversion"],
            )?;
            constant(&v["selectedActionId"], "RepayAccrualFirst")?;
            constant(&o["conversion"], "identity")?;
            id(&o["payer"])?;
            id(&o["obligationId"])?;
            if o["payer"] != i["signer"] {
                return Err(err("schema", "payer mismatch"));
            }
            if amount > gross || cap != 0 || floor != 0 {
                return Err(err("range", "invalid repayment bounds"));
            }
        }
        _ => return Err(err("schema", "unsupported operation")),
    };
    if !build {
        let supplied = hexbytes(&v["ownerProgramSha256"], 32, "hash")?;
        let actual = persistent_hash(&frame(b"moriarty-owner-program/1\0", &projection(v))?).0;
        if supplied != actual {
            return Err(err("hash", "owner program digest mismatch"));
        }
    }
    Ok(())
}
/// Native canonical encoder and verifier. Does not establish source matching or authority.
pub fn run(command: &str, text_input: &str) -> R<Value> {
    let input = codec::parse(text_input).map_err(|e| err("input", e))?;
    let (mut statement, signature) = match command {
        "intent-build" | "intent-frame" => (input, None),
        "intent-verify" => {
            object(&input, &["statement", "signatureHex"])?;
            (
                input["statement"].clone(),
                Some(hexbytes(&input["signatureHex"], 64, "signature")?),
            )
        }
        _ => return Err(err("input", "unknown intent command")),
    };
    validate(&statement, command == "intent-build")?;
    if command == "intent-build" {
        let h = persistent_hash(&frame(
            b"moriarty-owner-program/1\0",
            &projection(&statement),
        )?)
        .0;
        statement["ownerProgramSha256"] = json!(hex::encode(h));
    }
    let f = frame(b"moriarty-signed-intent/1\0", &statement)?;
    if f.len() > 16384 {
        return Err(err("range", "signing frame byte limit"));
    }
    let scheme = statement["signature"]["scheme"].as_str().unwrap();
    let framing = statement["signature"]["framing"].as_str().unwrap();
    let message = if framing == "midnight-sign-data" {
        wallet_message(&f)
    } else {
        f.clone()
    };
    let mut out = json!({"status":if command=="intent-build"{"IntentBuilt"}else if command=="intent-frame"{"IntentFrame"}else{"IntentSignatureChecked"},"statement":statement,"frame_hex":hex::encode(&f),"frame_sha256":hex::encode(persistent_hash(&f).0),"signing_message_hex":hex::encode(&message),"scheme":scheme,"framing":framing,"authority_valid":null,"snapshot_membership_valid":null,"transition_valid":null,"ledger_accepted":false,"qualification":"signature-protocol-only"});
    if let Some(sig) = signature {
        let key = hex::decode(statement["signature"]["publicKeyHex"].as_str().unwrap()).unwrap();
        let valid = verify(scheme, &message, &key, &sig).map_err(|e| err("signature", e))?;
        out["signature_valid"] = json!(valid);
    }
    Ok(out)
}
