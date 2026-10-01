// Local test authority map only; no authenticated ledger key registry.
use midnight_base_crypto::schnorr::SigningKey;
use midnight_serialize::Serializable;
use moriarty_midnight_crypto::{canonical_message, verify};
use rand::rngs::OsRng;
use serde_json::Value;
fn bytes<T: Serializable>(x: &T) -> Vec<u8> {
    let mut b = Vec::new();
    x.serialize(&mut b).unwrap();
    b
}
#[derive(Clone)]
struct LocalBinding {
    domain: String,
    signer: String,
    key_ref: String,
    key: Vec<u8>,
    revoked: bool,
    until: u128,
}
fn eligible(
    intent: &Value,
    key: &[u8],
    map: &[LocalBinding],
    round: u128,
    head: &str,
    consumed: &[String],
) -> bool {
    let Some(domain) = intent["domain"].as_str() else {
        return false;
    };
    let Some(signer) = intent["signer"].as_str() else {
        return false;
    };
    let Some(key_ref) = intent["keyRef"].as_str() else {
        return false;
    };
    let Some(start) = intent["notBefore"]
        .as_str()
        .and_then(|s| s.parse::<u128>().ok())
    else {
        return false;
    };
    let Some(end) = intent["notAfter"]
        .as_str()
        .and_then(|s| s.parse::<u128>().ok())
    else {
        return false;
    };
    let replay =
        serde_json::to_string(&[&intent["domain"], &intent["signer"], &intent["nonce"]]).unwrap();
    start <= round
        && round <= end
        && intent["preHead"] == head
        && !consumed.contains(&replay)
        && map.iter().any(|b| {
            b.domain == domain
                && b.signer == signer
                && b.key_ref == key_ref
                && b.key == key
                && !b.revoked
                && round <= b.until
        })
}
#[test]
fn cryptographically_valid_signatures_do_not_supply_authority_freshness_or_unused_nonce() {
    let f: Value = serde_json::from_str(include_str!("../fixtures/moriarty.json")).unwrap();
    let owner = &f["fixtures"][0]["intent_statement"];
    let intent = &owner["lowered_intent"];
    let sk = SigningKey::sample(OsRng);
    let pk = bytes(&sk.verifying_key());
    let m = canonical_message(&owner.to_string()).unwrap();
    let sig = bytes(&sk.sign(&mut OsRng, &m));
    assert!(verify("schnorr_bip340", &m, &pk, &sig).unwrap());
    let binding = LocalBinding {
        domain: intent["domain"].as_str().unwrap().into(),
        signer: intent["signer"].as_str().unwrap().into(),
        key_ref: intent["keyRef"].as_str().unwrap().into(),
        key: pk.clone(),
        revoked: false,
        until: 10,
    };
    assert!(eligible(intent, &pk, &[binding.clone()], 0, "h0", &[]));
    assert!(eligible(intent, &pk, &[binding.clone()], 10, "h0", &[]));
    assert!(!eligible(intent, &pk, &[binding.clone()], 11, "h0", &[]));
    assert!(!eligible(intent, &pk, &[], 1, "h0", &[]));
    let mut revoked = binding.clone();
    revoked.revoked = true;
    assert!(!eligible(intent, &pk, &[revoked], 1, "h0", &[]));
    let mut expired = binding.clone();
    expired.until = 0;
    assert!(!eligible(intent, &pk, &[expired], 1, "h0", &[]));
    for field in ["domain", "signer", "keyRef"] {
        let mut changed = binding.clone();
        match field {
            "domain" => changed.domain = "Other".into(),
            "signer" => changed.signer = "Other".into(),
            _ => changed.key_ref = "Other".into(),
        };
        assert!(!eligible(intent, &pk, &[changed], 1, "h0", &[]));
    }
    assert!(!eligible(intent, &pk, &[binding.clone()], 1, "h1", &[]));
    let replay =
        serde_json::to_string(&[&intent["domain"], &intent["signer"], &intent["nonce"]]).unwrap();
    assert!(!eligible(
        intent,
        &pk,
        &[binding.clone()],
        1,
        "h0",
        &[replay]
    ));
    assert!(verify("schnorr_bip340", &m, &pk, &sig).unwrap());
    let attacker = SigningKey::sample(OsRng);
    let attacker_pk = bytes(&attacker.verifying_key());
    let attacker_sig = bytes(&attacker.sign(&mut OsRng, &m));
    assert!(verify("schnorr_bip340", &m, &attacker_pk, &attacker_sig).unwrap());
    assert!(!eligible(
        intent,
        &attacker_pk,
        &[binding.clone()],
        1,
        "h0",
        &[]
    ));
    let mut future = intent.clone();
    future["notBefore"] = "2".into();
    assert!(!eligible(&future, &pk, &[binding.clone()], 1, "h0", &[]));
    future["notAfter"] = "1".into();
    assert!(!eligible(&future, &pk, &[binding], 1, "h0", &[]));
}
