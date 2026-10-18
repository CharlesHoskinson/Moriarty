//! Canonical public v3 Claim compatibility only; no native or parameter APIs.
use mori_backend::proving::{BINDING_VERSION, TransitionClaim};
use mori_core::intent::{canonical, hash_bytes};
use serde_json::Value;
#[test]
fn published_claim_decode_preserves_every_field_and_binding_preimage() {
    let mut value: Value = serde_json::from_str(include_str!(
        "../../../deliverables/native-owner-intent-2026-10-11/bundle.json"
    ))
    .unwrap();
    value.as_object_mut().unwrap().remove("proof_hex");
    let expected = canonical(&value).unwrap();
    let claim = TransitionClaim::from_json(&value.to_string()).unwrap();
    assert_eq!(canonical(&claim).unwrap(), expected);
    assert_eq!(
        hash_bytes(&expected),
        "7e6d6432dce0fc79625803448e48e30851131951cbe02b72a41b90722711cb8d"
    );
    let frame = canonical(&(BINDING_VERSION, &claim)).unwrap();
    assert_eq!(frame.len(), 11134);
    assert_eq!(
        hash_bytes(&frame),
        "685d7226e5d4638f981645a6d7a779310dd099041efad3315bc1fe27c3baf313"
    );
    assert_eq!(
        claim.execution_hash,
        "dfe157404292474ebede25b35f8bc0117fe856b2ed114c4f8bbe88707e9b51a3"
    );
    for key in [
        "authorization",
        "context_hash",
        "authorization_mode",
        "public_values",
    ] {
        let mut missing = value.clone();
        missing.as_object_mut().unwrap().remove(key);
        assert!(
            TransitionClaim::from_json(&missing.to_string()).is_err(),
            "{key}"
        );
    }
    let mut unknown = value.clone();
    unknown["signature_hex"] = "00".into();
    assert!(TransitionClaim::from_json(&unknown.to_string()).is_err());
}
