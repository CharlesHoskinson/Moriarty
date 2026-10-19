use k256::schnorr::SigningKey;
use k256::schnorr::signature::Signer;
use mori_core::{Scenario, UInt, intent::*};
fn fixture() -> (OwnerIntent, SignedIntent) {
    let source = r#"language "moriarty/1"; agreement Counter { state x: UInt = 0; action tick() { next x = x + 1; } }"#;
    let program = mori_core::parse(source).unwrap();
    let key = SigningKey::from_bytes(&[3; 32]).unwrap();
    let authority = Authorization {
        network: "local".into(),
        anchor: "anchor-a".into(),
        program_address: "contract-a".into(),
        target_hash: "ledger9-local-profile".into(),
        signer: "alice".into(),
        public_key_hex: hex(&key.verifying_key().to_bytes()),
        scheme: Scheme::SchnorrBip340,
        nonce: UInt(7),
        predecessor: "head-1".into(),
        not_before: UInt(10),
        not_after: UInt(20),
        limits: vec![],
    };
    let prepared = prepare_intent(
        &program,
        &hash_bytes(source.as_bytes()),
        "tick",
        &Scenario::default(),
        &authority,
    )
    .unwrap();
    let signature: k256::schnorr::Signature = key.sign(&signing_message(&prepared.intent).unwrap());
    let signed = SignedIntent {
        intent: prepared.intent.clone(),
        signature_hex: hex(signature.to_bytes().as_ref()),
    };
    (prepared.intent, signed)
}
#[test]
fn genuine_signature_and_bound_domain() {
    let (expected, signed) = fixture();
    verify_intent(&expected, &signed, UInt(10)).unwrap();
    let mut changed = expected.clone();
    changed.authority.anchor = "anchor-b".into();
    assert!(verify_intent(&changed, &signed, UInt(10)).is_err());
    let mut forged = signed.clone();
    forged.intent.authority.network = "other-network".into();
    assert!(verify_intent(&forged.intent, &forged, UInt(10)).is_err());
}
#[test]
fn tampering_expiry_and_noncanonical_encoding_reject() {
    let (expected, mut signed) = fixture();
    assert!(verify_intent(&expected, &signed, UInt(21)).is_err());
    signed.signature_hex = "00".repeat(64);
    assert!(verify_intent(&expected, &signed, UInt(10)).is_err());
    signed.signature_hex = "FF".repeat(64);
    assert!(verify_intent(&expected, &signed, UInt(10)).is_err());
}
#[test]
fn json_order_does_not_change_signing_bytes() {
    let a = mori_core::json::parse(r#"{"b":"2","a":"1"}"#).unwrap();
    let b = mori_core::json::parse(r#"{"a":"1","b":"2"}"#).unwrap();
    assert_eq!(canonical(&a).unwrap(), canonical(&b).unwrap());
}
