use midnight_ledger::structure::LedgerState;
use midnight_storage::db::InMemoryDB;
use mori_ledger::{
    codec::{decode_native, encode},
    profile::CallRequest,
};

#[test]
fn native_input_is_exact_and_canonical() {
    let state = LedgerState::<InMemoryDB>::new("local");
    let bytes = encode(&state).unwrap();
    let actual: LedgerState<InMemoryDB> = decode_native(&bytes).unwrap();
    assert_eq!(actual, state);
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(decode_native::<LedgerState<InMemoryDB>>(&trailing).is_err());
    assert!(decode_native::<LedgerState<InMemoryDB>>(&bytes[..bytes.len() - 1]).is_err());
}

#[test]
fn request_rejects_untrusted_outcomes_and_private_fields() {
    for text in [
        r#"{"action":"run","post_state":{}}"#,
        r#"{"action":"run","private_key":"secret"}"#,
        r#"{"action":"run","action":"other"}"#,
    ] {
        assert!(CallRequest::from_json(text).is_err());
    }
}

#[test]
fn generated_artifact_roundtrip_keeps_output_cap_separate_from_input_cap() {
    let values = vec![3u8; 1_048_577];
    let bytes = encode(&values).unwrap();
    assert!(decode_native::<Vec<u8>>(&bytes).is_err());
    let actual: Vec<u8> = mori_ledger::codec::decode_artifact(&bytes).unwrap();
    assert_eq!(actual, values);
    let oversized = vec![0u8; mori_ledger::codec::OUTPUT_BYTES + 1];
    assert!(mori_ledger::codec::decode_artifact::<Vec<u8>>(&oversized).is_err());
}
#[test]
fn independent_literal_abi_sha256_and_scale_vectors() {
    use mori_ledger::{codec, layout};
    assert_eq!(
        codec::hex(&layout::digest(
            "moriarty-ledger-public-abi/1:state",
            &[b"paid"]
        )),
        "f0f316e7b9fb9479b794e51c824dadf00366732b50801cee71135b2947dd47dc"
    );
    assert_eq!(
        codec::hex(&layout::digest(
            "moriarty-ledger-public-abi/1:duty",
            &[b"payment"]
        )),
        "d77277ec7533cbbe9407285bea7eb827e11fdec2d0018973f5d4252a95e44adc"
    );
    let mut asset = mori_core::AssetId {
        network: "local".into(),
        issuer: "signed-native-qa".into(),
        reference: "QA".into(),
        scale: 0,
    };
    assert_eq!(
        codec::hex(&layout::asset_digest(&asset).0),
        "ad9d2fc822151d2beac852c6b6ca146c5bea7b2fb8c60de4ebca13af8c4386e4"
    );
    asset.scale = 6;
    assert_eq!(
        codec::hex(&layout::asset_digest(&asset).0),
        "bb43b8a50c51398fdf2a4e4b481057d60de392d93dfa4e5a7f4638ac8f095c99"
    );
}
