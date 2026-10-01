//! TEST-ONLY ephemeral signing fixture for signed intent /1. This is not a production signing command.
//!
//! Usage: `intent-fixture <schnorr_bip340|ecdsa_secp256k1_sha256> <raw|midnight-sign-data> < draft.json`
//!
//! Reads one draft statement (at most 65536 bytes), generates a native secret key from OsRng in memory only,
//! sets `signature.publicKeyHex`, `signature.scheme` and `signature.framing`, calls the real
//! `intent::run("intent-build", ...)`, signs the returned `signing_message_hex` natively, self-checks with
//! `intent-verify` and prints only `{"statement":...,"signatureHex":...}`. The secret key is never printed,
//! stored or returned. A valid signature proves possession of this throwaway key only; it is not account authority.
use midnight_base_crypto::{ecdsa, schnorr};
use midnight_serialize::Serializable;
use moriarty_midnight_crypto::{codec, intent};
use rand::rngs::OsRng;
use serde_json::{Value, json};
use std::io::Read;

fn bytes<T: Serializable>(v: &T) -> Vec<u8> {
    let mut b = vec![];
    v.serialize(&mut b).expect("in-memory serialization");
    b
}
/// Failures go to stderr as public text; stdout stays empty.
fn fail(code: &str, error: impl std::fmt::Display) -> ! {
    eprintln!("{}", json!({"status":"FixtureRejected","code":code,"error":error.to_string(),"test_only":true}));
    std::process::exit(2);
}
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [scheme, framing] = args.as_slice() else {
        fail("usage", "intent-fixture <schnorr_bip340|ecdsa_secp256k1_sha256> <raw|midnight-sign-data> < draft.json");
    };
    if !["schnorr_bip340", "ecdsa_secp256k1_sha256"].contains(&scheme.as_str()) {
        fail("usage", "unsupported scheme");
    }
    if !["raw", "midnight-sign-data"].contains(&framing.as_str()) {
        fail("usage", "unsupported framing");
    }
    let mut raw = Vec::new();
    if let Err(e) = std::io::stdin().take(65537).read_to_end(&mut raw) {
        fail("input", e);
    }
    if raw.len() > 65536 {
        fail("input", "draft byte limit");
    }
    let text = std::str::from_utf8(&raw).unwrap_or_else(|e| fail("input", e));
    let mut draft = codec::parse(text).unwrap_or_else(|e| fail("input", e));
    let (public, signer): (Vec<u8>, Box<dyn Fn(&[u8]) -> Vec<u8>>) = if scheme == "schnorr_bip340" {
        let key = schnorr::SigningKey::sample(OsRng);
        (bytes(&key.verifying_key()), Box::new(move |m| bytes(&key.sign(&mut OsRng, m))))
    } else {
        let key = ecdsa::SigningKey::sample(OsRng);
        (bytes(&key.verifying_key()), Box::new(move |m| bytes(&key.sign(m))))
    };
    let Some(signature) = draft.get_mut("signature").and_then(Value::as_object_mut) else {
        fail("schema", "draft.signature object is required");
    };
    signature.insert("scheme".into(), json!(scheme));
    signature.insert("publicKeyHex".into(), json!(hex::encode(public)));
    signature.insert("framing".into(), json!(framing));
    let built = intent::run("intent-build", &draft.to_string()).unwrap_or_else(|e| fail(e.code, e.error));
    let message = hex::decode(built["signing_message_hex"].as_str().unwrap_or_default()).unwrap_or_else(|e| fail("native", e));
    let artifact = json!({"statement": built["statement"], "signatureHex": hex::encode(signer(&message))});
    let checked = intent::run("intent-verify", &artifact.to_string()).unwrap_or_else(|e| fail(e.code, e.error));
    if checked["signature_valid"] != json!(true) {
        fail("native", "self-check did not verify");
    }
    println!("{artifact}");
}
