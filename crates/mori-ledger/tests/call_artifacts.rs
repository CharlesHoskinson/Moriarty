mod support;
use mori_ledger::{
    call_artifacts::{
        ActionKeyMetadata, Leaf, ProvingKeysetManifest, PublicKeysetManifest, public_identity,
        snapshot_proven_call, snapshot_proving_keyset, snapshot_public_keyset,
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
fn public_fixture() -> std::path::PathBuf {
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
    root
}
#[test]
fn public_snapshot_never_reads_optional_pk_even_when_combined_marker_declares_it() {
    let root = public_fixture();
    let mut bounds = limits();
    bounds.max_artifact_bytes = public_bytes(&root);
    let public = snapshot_public_keyset(&root, &bounds).unwrap();
    assert_eq!(public.action_count(), 1);
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(root.join("missing"), root.join("selected.pk.bin")).unwrap();
        assert!(snapshot_public_keyset(&root, &bounds).is_ok());
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

fn public_bytes(root: &std::path::Path) -> u64 {
    [
        "public-keyset.json",
        "source.mori",
        "action-000000.zkir.bin",
        "action-000000.manifest.json",
        "action-000000.vk.bin",
    ]
    .into_iter()
    .map(|n| std::fs::metadata(root.join(n)).unwrap().len())
    .sum()
}
#[test]
fn initial_public_marker_is_bounded_before_json_parse() {
    let root = public_fixture();
    std::fs::write(root.join("public-keyset.json"), b"bad-json").unwrap();
    let mut bounds = limits();
    bounds.max_artifact_bytes = 1;
    let err = snapshot_public_keyset(&root, &bounds).err().unwrap();
    std::fs::remove_dir_all(root).unwrap();
    assert_eq!(err.code, "file-kind-or-size");
}
#[test]
fn initial_proven_marker_is_bounded_before_json_parse() {
    let root = public_fixture();
    std::fs::write(root.join("proven-call.json"), b"bad-json").unwrap();
    let mut bounds = limits();
    bounds.max_artifact_bytes = 1;
    let err = snapshot_proven_call(&root, &bounds).err().unwrap();
    std::fs::remove_dir_all(root).unwrap();
    assert_eq!(err.code, "file-kind-or-size");
}
#[test]
fn secondary_proving_marker_uses_remaining_aggregate_before_json_parse() {
    let root = public_fixture();
    std::fs::write(root.join("proving-keyset.json"), b"bad-json").unwrap();
    let mut bounds = limits();
    bounds.max_artifact_bytes = public_bytes(&root) + 1;
    let err = snapshot_proving_keyset(&root, "settle", &bounds)
        .err()
        .unwrap();
    std::fs::remove_dir_all(root).unwrap();
    assert_eq!(err.code, "file-kind-or-size");
}
#[test]
fn exhausted_secondary_marker_aggregate_refuses_before_open() {
    let root = public_fixture();
    let mut bounds = limits();
    bounds.max_artifact_bytes = public_bytes(&root);
    let err = snapshot_proving_keyset(&root, "settle", &bounds)
        .err()
        .unwrap();
    std::fs::remove_dir_all(root).unwrap();
    assert_eq!(err.code, "aggregate-size");
}
#[test]
fn proving_marker_and_selected_pk_have_exact_cumulative_budget() {
    let root = public_fixture();
    let pk = b"untrusted-opaque-pk";
    let pk_leaf = Leaf {
        bytes: pk.len() as u64,
        sha256: codec::hash(pk),
    };
    let mut public: PublicKeysetManifest =
        serde_json::from_slice(&std::fs::read(root.join("public-keyset.json")).unwrap()).unwrap();
    let proving = ProvingKeysetManifest {
        profile: "moriarty-native-call-proving-keyset/1".into(),
        public_identity: public.identity.clone(),
        action: "settle".into(),
        prover_key: pk_leaf.clone(),
    };
    let proving_bytes = serde_json::to_vec_pretty(&proving).unwrap();
    public.leaves.insert("selected.pk.bin".into(), pk_leaf);
    public.leaves.insert(
        "proving-keyset.json".into(),
        Leaf {
            bytes: proving_bytes.len() as u64,
            sha256: codec::hash(&proving_bytes),
        },
    );
    std::fs::write(
        root.join("public-keyset.json"),
        serde_json::to_vec_pretty(&public).unwrap(),
    )
    .unwrap();
    std::fs::write(root.join("proving-keyset.json"), &proving_bytes).unwrap();
    std::fs::write(root.join("selected.pk.bin"), pk).unwrap();
    let mut bounds = limits();
    bounds.max_artifact_bytes = public_bytes(&root) + proving_bytes.len() as u64 + pk.len() as u64;
    assert!(snapshot_proving_keyset(&root, "settle", &bounds).is_ok());
    bounds.max_artifact_bytes -= 1;
    let err = snapshot_proving_keyset(&root, "settle", &bounds)
        .err()
        .unwrap();
    std::fs::remove_dir_all(root).unwrap();
    assert_eq!(err.code, "aggregate-size");
}
