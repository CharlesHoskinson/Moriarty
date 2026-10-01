//! Readonly bounded stdin verifier; no key generation or signing command.
use moriarty_midnight_crypto::{canonical_message, codec, verify, wallet_message};
use serde::Deserialize;
use serde_json::{Value, json};
use std::io::{self, Read};
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    scheme: String,
    statement: Value,
    public_key_hex: String,
    signature_hex: String,
    wallet_prefix: bool,
}
fn run() -> Result<Value, String> {
    let mut raw = Vec::new();
    io::stdin()
        .take(65537)
        .read_to_end(&mut raw)
        .map_err(|e| e.to_string())?;
    if raw.len() > 65536 {
        return Err("request byte limit".into());
    }
    let text = std::str::from_utf8(&raw).map_err(|e| e.to_string())?;
    let command = std::env::args()
        .nth(1)
        .ok_or("usage: moriarty-midnight-crypto frame|verify < request.json")?;
    let value = codec::parse(text)?;
    if command == "frame" {
        let b = canonical_message(text)?;
        return Ok(
            json!({"status":"ExperimentalFrame","hex":hex::encode(&b),"sha256":hex::encode(midnight_base_crypto::hash::persistent_hash(&b).0)}),
        );
    }
    if command != "verify" {
        return Err("unknown command".into());
    }
    let r: Request = serde_json::from_value(value).map_err(|e| e.to_string())?;
    if r.public_key_hex.len() > 66 || r.signature_hex.len() != 128 {
        return Err("exact raw key/signature lengths required".into());
    }
    let key = hex::decode(&r.public_key_hex).map_err(|e| e.to_string())?;
    let sig = hex::decode(&r.signature_hex).map_err(|e| e.to_string())?;
    let b = canonical_message(&r.statement.to_string())?;
    let message = if r.wallet_prefix {
        wallet_message(&b)
    } else {
        b
    };
    Ok(
        json!({"status":"SignatureChecked","signature_valid":verify(&r.scheme,&message,&key,&sig)?,"scheme":r.scheme,"authority_valid":null,"snapshot_membership_valid":null,"transition_valid":null,"ledger_accepted":false,"qualification":"experimental-cryptographic-check-only"}),
    )
}
fn main() {
    match run() {
        Ok(v) => {
            let bad = v.get("signature_valid") == Some(&Value::Bool(false));
            println!("{v}");
            if bad {
                std::process::exit(1);
            }
        }
        Err(e) => {
            println!(
                "{}",
                json!({"status":"Rejected","error":e,"ledger_accepted":false})
            );
            std::process::exit(2);
        }
    }
}
