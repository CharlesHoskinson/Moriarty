mod support;
use mori_ledger::{
    call_artifacts::{
        ActionKeyMetadata, Leaf, PublicKeysetManifest, public_identity, snapshot_public_keyset,
    },
    codec,
    profile::{NativeWorkLimits, StaticBindings},
};
use std::{
    collections::BTreeMap,
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
fn limits() -> NativeWorkLimits {
    NativeWorkLimits {
        profile: "moriarty-native-call-work-limits/1".into(),
        max_degree: 19,
        max_actions: 4,
        max_srs_bytes: 1024,
        max_prover_key_bytes: 1024,
        max_verifier_key_bytes: 1024,
        max_input_bytes: 1_048_576,
        max_leaf_bytes: 1_048_576,
        max_artifact_bytes: 4_194_304,
        max_operations: 10000,
        max_public_inputs: 10000,
    }
}
#[test]
fn public_snapshot_never_reads_optional_pk_even_when_combined_marker_declares_it() {
    let root = std::env::temp_dir().join(format!(
        "mori-public-reader-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir(&root).unwrap();
    let f = support::payment();
    let d = serde_json::to_value(&f.deployment).unwrap();
    let fields = [
        "abi",
        "source_hash",
        "program_hash",
        "abi_hash",
        "templates_hash",
        "toolchain",
        "limits",
        "network",
        "anchor",
        "address_hex",
        "payer",
        "assets",
        "roles",
        "duties",
        "economic_bounds",
    ];
    let bindings: StaticBindings = serde_json::from_value(serde_json::Value::Object(
        fields
            .into_iter()
            .map(|k| (k.into(), d[k].clone()))
            .collect(),
    ))
    .unwrap();
    let mut leaves = BTreeMap::new();
    for (name, bytes) in [
        ("source.mori", f.source.as_bytes()),
        ("action-000000.zkir.bin", b"opaque-ir".as_slice()),
        ("action-000000.manifest.json", b"opaque-manifest".as_slice()),
        ("action-000000.vk.bin", b"opaque-vk".as_slice()),
    ] {
        std::fs::write(root.join(name), bytes).unwrap();
        leaves.insert(
            name.into(),
            Leaf {
                sha256: codec::hash(bytes),
                bytes: bytes.len() as u64,
            },
        );
    }
    // A non-existent PK leaf with nonsensical metadata must be irrelevant to the
    // public snapshot. These are untrusted parsing bytes, no derived capability.
    leaves.insert(
        "selected.pk.bin".into(),
        Leaf {
            sha256: "not-a-digest".into(),
            bytes: u64::MAX,
        },
    );
    let action = ActionKeyMetadata {
        name: "settle".into(),
        ordinal: 0,
        ir_hash: codec::hash(b"opaque-ir"),
        manifest_hash: codec::hash(b"opaque-manifest"),
        vk_hash: codec::hash(b"opaque-vk"),
        degree: 9,
        srs_digest: "untrusted-metadata".into(),
        srs_bytes: 1,
    };
    let mut marker = PublicKeysetManifest {
        profile: "moriarty-native-call-public-keyset/1".into(),
        bindings,
        build: "untrusted-build".into(),
        actions: vec![action],
        identity: String::new(),
        leaves,
    };
    marker.identity = public_identity(&marker).unwrap();
    std::fs::write(
        root.join("public-keyset.json"),
        serde_json::to_vec_pretty(&marker).unwrap(),
    )
    .unwrap();
    let public = snapshot_public_keyset(&root, &limits()).unwrap();
    assert_eq!(public.action_count(), 1);
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(root.join("missing"), root.join("selected.pk.bin")).unwrap();
        assert!(snapshot_public_keyset(&root, &limits()).is_ok());
    }
    std::fs::remove_dir_all(&root).unwrap();
}
#[test]
fn mandatory_native_limits_reject_zero_unknown_and_duplicate_fields() {
    let good = serde_json::to_string(&limits()).unwrap();
    assert!(NativeWorkLimits::from_json(&good).is_ok());
    let mut zero = limits();
    zero.max_srs_bytes = 0;
    assert!(NativeWorkLimits::from_json(&serde_json::to_string(&zero).unwrap()).is_err());
    assert!(NativeWorkLimits::from_json(&good.replace("{", "{\"max_degree\":9,")).is_err());
    assert!(NativeWorkLimits::from_json(&good.replace("{", "{\"expected_vk\":\"hash\",")).is_err());
}
