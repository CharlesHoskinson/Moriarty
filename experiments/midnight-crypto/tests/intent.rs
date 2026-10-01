use midnight_base_crypto::{ecdsa, schnorr};
use midnight_serialize::Serializable;
use rand::rngs::OsRng;
use serde_json::{Value, json};
use std::{
    io::Write,
    process::{Command, Stdio},
};
fn bytes<T: Serializable>(v: &T) -> Vec<u8> {
    let mut b = vec![];
    v.serialize(&mut b).unwrap();
    b
}
fn draft(key: &str, scheme: &str, framing: &str) -> Value {
    json!({"profile":"moriarty-signed-intent/1","core":"moriarty-core/5","sourceProfile":"moriarty-financial-agreement-source/6","authoringProfile":"moriarty-beta/1","agreementId":"Invoice","actionName":"pay","selectedActionId":"TransferLiteralFee","sourceSha256":"00".repeat(32),"domain":{"id":"Midnight","chain":"midnight","network":"preview"},"asset":{"id":"USD","representation":"canonical","scale":"2","symbol":null},"signature":{"scheme":scheme,"publicKeyHex":key,"keyRef":"AliceKey","framing":framing},"intent":{"version":"moriarty-intent/3","advertisedSourceHash":"opaque","advertisedPolicyDigest":"opaque","signer":"Alice","nonce":"n1","preHead":"h1","notBefore":"0","notAfter":"10","grossCap":"101","feeCap":"1","netFloor":"100","failure":"success_only","observations":[],"disclosures":[],"retainedEffects":[],"retainedDuties":[],"delegation":"none","recovery":"none","operation":{"kind":"Transfer","from":"Alice","recipient":"Bob","feeRecipient":"Fee","amount":"100","fee":"1"}}})
}
fn cli(cmd: &str, text: &str) -> (i32, Value) {
    let mut c = Command::new(env!("CARGO_BIN_EXE_moriarty-midnight-crypto"))
        .arg(cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    c.stdin.take().unwrap().write_all(text.as_bytes()).unwrap();
    let o = c.wait_with_output().unwrap();
    (
        o.status.code().unwrap(),
        serde_json::from_slice(&o.stdout).unwrap(),
    )
}
#[test]
fn native_binary_all_schemes_and_modes() {
    for scheme in ["schnorr_bip340", "ecdsa_secp256k1_sha256"] {
        for framing in ["raw", "midnight-sign-data"] {
            let sk = schnorr::SigningKey::sample(OsRng);
            let ek = ecdsa::SigningKey::sample(OsRng);
            let key = if scheme == "schnorr_bip340" {
                bytes(&sk.verifying_key())
            } else {
                bytes(&ek.verifying_key())
            };
            let d = draft(&hex::encode(key), scheme, framing);
            let (code, b) = cli("intent-build", &d.to_string());
            assert_eq!(code, 0, "{b}");
            assert_eq!(b["status"], "IntentBuilt");
            assert_eq!(b["authority_valid"], Value::Null);
            assert_eq!(b["ledger_accepted"], false);
            let msg = hex::decode(b["signing_message_hex"].as_str().unwrap()).unwrap();
            let sig = if scheme == "schnorr_bip340" {
                bytes(&sk.sign(&mut OsRng, &msg))
            } else {
                bytes(&ek.sign(&msg))
            };
            let mut e = json!({"statement":b["statement"],"signatureHex":hex::encode(sig)});
            let (c, v) = cli("intent-verify", &e.to_string());
            assert_eq!(c, 0, "{v}");
            assert_eq!(v["signature_valid"], true);
            e["statement"]["signature"]["keyRef"] = json!("OtherKey");
            let (c, v) = cli("intent-verify", &e.to_string());
            assert_eq!(c, 1, "{v}");
            assert_eq!(v["signature_valid"], false);
        }
    }
}
#[test]
fn rejects_closed_schema_and_ranges() {
    let sk = schnorr::SigningKey::sample(OsRng);
    let d = draft(
        &hex::encode(bytes(&sk.verifying_key())),
        "schnorr_bip340",
        "raw",
    );
    let (_, good) = cli("intent-build", &d.to_string());
    assert_eq!(good["status"], "IntentBuilt");
    let cases = [
        ("/asset/scale", json!("19")),
        ("/intent/operation/amount", json!("01")),
        ("/intent/operation/from", json!("Other")),
        ("/intent/observations", json!(["x"])),
        ("/signature/publicKeyHex", json!("ff".repeat(32))),
        (
            "/intent/notAfter",
            json!("340282366920938463463374607431768211456"),
        ),
    ];
    for (p, v) in cases {
        let mut bad = d.clone();
        *bad.pointer_mut(p).unwrap() = v;
        let (c, r) = cli("intent-build", &bad.to_string());
        assert_eq!(c, 2, "{p}: {r}");
        assert_eq!(r["status"], "IntentRejected");
        assert_eq!(r["signature_valid"], Value::Null);
    }
    let mut bad = d.clone();
    bad["unexpected"] = json!("x");
    assert_eq!(cli("intent-build", &bad.to_string()).0, 2);
    let mut bad = good["statement"].clone();
    bad["ownerProgramSha256"] = json!("00".repeat(32));
    let (c, r) = cli("intent-frame", &bad.to_string());
    assert_eq!(c, 2);
    assert_eq!(r["code"], "hash");
    let duplicate = d
        .to_string()
        .replacen("{", "{\"profile\":\"moriarty-signed-intent/1\",", 1);
    assert_eq!(cli("intent-build", &duplicate).0, 2);
}
#[test]
fn frame_is_deterministic_and_domain_separated() {
    let sk = schnorr::SigningKey::sample(OsRng);
    let mut d = draft(
        &hex::encode(bytes(&sk.verifying_key())),
        "schnorr_bip340",
        "raw",
    );
    d["intent"]["nonce"] = json!("🦉é");
    let (c, b) = cli("intent-build", &d.to_string());
    assert_eq!(c, 0);
    let (c, f) = cli(
        "intent-frame",
        &serde_json::to_string_pretty(&b["statement"]).unwrap(),
    );
    assert_eq!(c, 0);
    assert_eq!(b["frame_hex"], f["frame_hex"]);
    let frame = hex::decode(b["frame_hex"].as_str().unwrap()).unwrap();
    let domain = b"moriarty-signed-intent/1\0";
    assert!(frame.starts_with(domain));
    let n = u32::from_be_bytes(frame[domain.len()..domain.len() + 4].try_into().unwrap()) as usize;
    assert_eq!(n, frame.len() - domain.len() - 4);
    let payload: Value = serde_json::from_slice(&frame[domain.len() + 4..]).unwrap();
    assert_eq!(payload, b["statement"]);
    assert_ne!(
        frame,
        moriarty_midnight_crypto::canonical_message(&b["statement"].to_string()).unwrap()
    );
    let mut changed = d.clone();
    changed["sourceSha256"] = json!("11".repeat(32));
    let (_, c) = cli("intent-build", &changed.to_string());
    assert_ne!(b["frame_hex"], c["frame_hex"]);
    assert_eq!(
        b["statement"]["ownerProgramSha256"],
        c["statement"]["ownerProgramSha256"]
    );
}
#[test]
fn repayment_and_malformed_signature() {
    let sk = schnorr::SigningKey::sample(OsRng);
    let mut d = draft(
        &hex::encode(bytes(&sk.verifying_key())),
        "schnorr_bip340",
        "raw",
    );
    d["selectedActionId"] = json!("RepayAccrualFirst");
    d["intent"]["feeCap"] = json!("0");
    d["intent"]["netFloor"] = json!("0");
    d["intent"]["operation"] = json!({"kind":"Repay","payer":"Alice","obligationId":"Loan","amount":"100","conversion":"identity"});
    let (c, b) = cli("intent-build", &d.to_string());
    assert_eq!(c, 0, "{b}");
    let e = json!({"statement":b["statement"],"signatureHex":"AA".repeat(64)});
    let (c, r) = cli("intent-verify", &e.to_string());
    assert_eq!(c, 2);
    assert_eq!(r["code"], "signature");
    d["intent"]["feeCap"] = json!("1");
    assert_eq!(cli("intent-build", &d.to_string()).0, 2);
    let raw = " ".repeat(65537);
    assert_eq!(cli("intent-build", &raw).0, 2);
}
