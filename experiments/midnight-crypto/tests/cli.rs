//! Exercise the actual bounded stdin consumer, with memory-only ephemeral keys.
use midnight_base_crypto::{ecdsa, schnorr};
use midnight_serialize::Serializable;
use moriarty_midnight_crypto::{canonical_message, wallet_message};
use rand::rngs::OsRng;
use serde_json::{Value, json};
use std::{
    io::Write,
    process::{Command, Stdio},
};
fn bytes<T: Serializable>(v: &T) -> Vec<u8> {
    let mut b = Vec::new();
    v.serialize(&mut b).unwrap();
    b
}
fn call(command: &str, input: &[u8]) -> (i32, Value) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_moriarty-midnight-crypto"))
        .arg(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    let output = child.wait_with_output().unwrap();
    (
        output.status.code().unwrap(),
        serde_json::from_slice(&output.stdout).unwrap(),
    )
}
#[test]
fn real_cli_verifies_both_schemes_and_keeps_financial_gates_unqualified() {
    let statement = json!({"recipient":"é😀","amount":"1000"});
    for scheme in ["schnorr_bip340", "ecdsa_secp256k1_sha256"] {
        for prefix in [false, true] {
            let raw = canonical_message(&statement.to_string()).unwrap();
            let m = if prefix { wallet_message(&raw) } else { raw };
            let (pk, sig) = if scheme == "schnorr_bip340" {
                let sk = schnorr::SigningKey::sample(OsRng);
                (bytes(&sk.verifying_key()), bytes(&sk.sign(&mut OsRng, &m)))
            } else {
                let sk = ecdsa::SigningKey::sample(OsRng);
                (bytes(&sk.verifying_key()), bytes(&sk.sign(&m)))
            };
            let mut request = json!({"scheme":scheme,"statement":statement,"public_key_hex":hex::encode(pk),"signature_hex":hex::encode(sig),"wallet_prefix":prefix});
            let (exit, out) = call("verify", request.to_string().as_bytes());
            assert_eq!(exit, 0);
            assert_eq!(out["signature_valid"], true);
            assert_eq!(out["ledger_accepted"], false);
            for field in [
                "authority_valid",
                "snapshot_membership_valid",
                "transition_valid",
            ] {
                assert!(out[field].is_null());
            }
            request["statement"]["amount"] = "1001".into();
            let (exit, out) = call("verify", request.to_string().as_bytes());
            assert_eq!(exit, 1);
            assert_eq!(out["signature_valid"], false);
            request["extra"] = true.into();
            let (exit, out) = call("verify", request.to_string().as_bytes());
            assert_eq!(exit, 2);
            assert_eq!(out["status"], "Rejected");
        }
    }
}
#[test]
fn real_cli_refuses_malformed_duplicate_unknown_and_oversized_requests() {
    for input in [
        b"{}".as_slice(),
        b"{\"scheme\":\"x\",\"scheme\":\"y\"}",
        b"{\"n\":1}",
        b"\xff",
        b"{} {}",
    ] {
        assert_eq!(call("verify", input).0, 2);
    }
    assert_eq!(call("frame", &vec![b' '; 65537]).0, 2);
    assert_eq!(call("unknown", b"{}").0, 2);
    let (exit, out) = call("frame", br#"{"b":"2","a":"1"}"#);
    assert_eq!(exit, 0);
    assert_eq!(out["status"], "ExperimentalFrame");
    assert_eq!(
        out["hex"],
        hex::encode(canonical_message(r#"{"a":"1","b":"2"}"#).unwrap())
    );
}
