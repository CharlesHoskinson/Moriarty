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
        signer_policy: None,
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

fn proving_fixture() -> (std::path::PathBuf, u64, u64) {
    proving_fixture_profile(false)
}
fn proving_fixture_profile(signed: bool) -> (std::path::PathBuf, u64, u64) {
    let root = public_fixture();
    // Opaque transport bytes, not an SDK key; larger than every public file
    // together so a byte budget can admit the public set without the key.
    let pk = vec![0x42; 1 << 17];
    let pk_leaf = Leaf {
        bytes: pk.len() as u64,
        sha256: codec::hash(&pk),
    };
    let mut public: PublicKeysetManifest =
        serde_json::from_slice(&std::fs::read(root.join("public-keyset.json")).unwrap()).unwrap();
    if signed {
        public.profile = mori_ledger::signed::KEYSET.into();
        public.signer_policy = Some(mori_ledger::signed::FixedSignerPolicy {
            profile: mori_ledger::signed::POLICY.into(), owner: "alice".into(), beneficiary: "bob".into(),
            public_key_hex: "0677506737143823730512c5525e0b30acb75906fa7d53f494ba689b78620133d4cdab997110c2f1e02bb16ebef816c9d0a6025386825581a688b9bfa326360a".into(),
        });
        if signed {
            let f = support::payment();
            let statics = mori_ledger::signed::SignedStaticBindings::from_source(
                &f.source,
                &f.program,
                &public.bindings,
                public.signer_policy.as_ref().unwrap(),
                mori_ledger::signed::SignedVersion::LegacyV1,
            )
            .unwrap();
            let schema = mori_ledger::signed::compile_signed(&statics, "settle")
                .unwrap()
                .manifest_bytes()
                .unwrap();
            std::fs::write(root.join("action-000000.manifest.json"), &schema).unwrap();
            public.leaves.insert(
                "action-000000.manifest.json".into(),
                Leaf {
                    bytes: schema.len() as u64,
                    sha256: codec::hash(&schema),
                },
            );
            public.actions[0].manifest_hash = codec::hash(&schema);
        }
        public.identity = public_identity(&public).unwrap();
    }
    let proving = ProvingKeysetManifest {
        profile: if signed {
            mori_ledger::signed::KEYSET
        } else {
            "moriarty-native-call-proving-keyset/1"
        }
        .into(),
        public_identity: public.identity.clone(),
        action: "settle".into(),
        prover_key: pk_leaf.clone(),
    };
    let marker = serde_json::to_vec_pretty(&proving).unwrap();
    public.leaves.insert("selected.pk.bin".into(), pk_leaf);
    public.leaves.insert(
        "proving-keyset.json".into(),
        Leaf {
            bytes: marker.len() as u64,
            sha256: codec::hash(&marker),
        },
    );
    std::fs::write(
        root.join("public-keyset.json"),
        serde_json::to_vec_pretty(&public).unwrap(),
    )
    .unwrap();
    std::fs::write(root.join("proving-keyset.json"), &marker).unwrap();
    std::fs::write(root.join("selected.pk.bin"), &pk).unwrap();
    let public_total = public_bytes(&root) + marker.len() as u64;
    assert!(public_total < pk.len() as u64);
    (root, public_total, pk.len() as u64)
}
fn snapshot_profile(
    root: &std::path::Path,
    bounds: &NativeWorkLimits,
    signed: bool,
) -> mori_ledger::Result<mori_ledger::call_artifacts::ImportedProvingKeyset> {
    if signed {
        mori_ledger::signed::snapshot_signed_proving_keyset(
            root,
            "settle",
            bounds,
            mori_ledger::signed::SignedVersion::LegacyV1,
        )
    } else {
        snapshot_proving_keyset(root, "settle", bounds)
    }
}
fn v2_bounds(public: u64, pk: u64) -> NativeWorkLimits {
    let mut bounds = limits();
    bounds.profile = "moriarty-native-call-work-limits/2".into();
    bounds.max_leaf_bytes = public;
    bounds.max_artifact_bytes = public;
    bounds.max_prover_key_bytes = pk;
    bounds
}
#[test]
fn v2_signed_and_unsigned_reader_domain_boundaries_and_preopen_priorities() {
    for signed in [false, true] {
        let (root, public, pk) = proving_fixture_profile(signed);
        let mut bounds = v2_bounds(public, pk);
        assert!(snapshot_profile(&root, &bounds, signed).is_ok());
        let mut vk_lowered = bounds.clone();
        vk_lowered.max_verifier_key_bytes = b"opaque-vk".len() as u64 - 1;
        assert_eq!(
            snapshot_profile(&root, &vk_lowered, signed)
                .err()
                .unwrap()
                .code,
            "leaf-size"
        );
        bounds.max_prover_key_bytes -= 1;
        std::fs::remove_file(root.join("selected.pk.bin")).unwrap();
        assert_eq!(
            snapshot_profile(&root, &bounds, signed).err().unwrap().code,
            "leaf-size"
        );
        #[cfg(unix)]
        {
            use std::{ffi::CString, os::unix::ffi::OsStrExt};
            let path = CString::new(root.join("selected.pk.bin").as_os_str().as_bytes()).unwrap();
            assert_eq!(unsafe { libc::mkfifo(path.as_ptr(), 0o600) }, 0);
            assert_eq!(
                snapshot_profile(&root, &bounds, signed).err().unwrap().code,
                "leaf-size"
            );
        }
        bounds.max_prover_key_bytes += 1;
        bounds.max_artifact_bytes = public_bytes(&root);
        std::fs::remove_file(root.join("proving-keyset.json")).unwrap();
        assert_eq!(
            snapshot_profile(&root, &bounds, signed).err().unwrap().code,
            "aggregate-size"
        );
        std::fs::remove_dir_all(root).unwrap();
    }
}
#[test]
fn v2_reader_refuses_opaque_pk_length_hash_and_marker_changes() {
    for signed in [false, true] {
        for (case, expected) in [
            ("short", "leaf-identity"),
            ("long", "file-kind-or-size"),
            ("changed", "leaf-identity"),
            ("descriptor", "proving-leaf-identity"),
            ("action", "proving-manifest"),
            ("identity", "proving-manifest"),
            ("profile", "proving-manifest"),
        ] {
            let (root, public, pk) = proving_fixture_profile(signed);
            if matches!(case, "short" | "long" | "changed") {
                let size = match case {
                    "short" => pk - 1,
                    "long" => pk + 1,
                    _ => pk,
                };
                std::fs::write(root.join("selected.pk.bin"), vec![0x43; size as usize]).unwrap();
            } else {
                let mut marker: ProvingKeysetManifest = serde_json::from_slice(
                    &std::fs::read(root.join("proving-keyset.json")).unwrap(),
                )
                .unwrap();
                match case {
                    "descriptor" => marker.prover_key.sha256 = "changed".into(),
                    "action" => marker.action = "wrong".into(),
                    "identity" => marker.public_identity = "wrong".into(),
                    _ => marker.profile = "wrong".into(),
                }
                std::fs::write(
                    root.join("proving-keyset.json"),
                    serde_json::to_vec_pretty(&marker).unwrap(),
                )
                .unwrap();
            }
            let bounds = v2_bounds(public + 512, pk);
            assert_eq!(
                snapshot_profile(&root, &bounds, signed).err().unwrap().code,
                expected,
                "{signed}: {case}"
            );
            std::fs::remove_dir_all(root).unwrap();
        }
    }
}
#[test]
fn v2_cold_public_reader_ignores_unacquired_optional_pk_and_proving_files() {
    for signed in [false, true] {
        let (root, _, _) = proving_fixture_profile(signed);
        let mut marker: PublicKeysetManifest =
            serde_json::from_slice(&std::fs::read(root.join("public-keyset.json")).unwrap())
                .unwrap();
        for name in ["selected.pk.bin", "proving-keyset.json"] {
            marker.leaves.insert(
                name.into(),
                Leaf {
                    bytes: u64::MAX,
                    sha256: "not-a-digest".into(),
                },
            );
            std::fs::remove_file(root.join(name)).unwrap();
        }
        std::fs::write(
            root.join("public-keyset.json"),
            serde_json::to_vec_pretty(&marker).unwrap(),
        )
        .unwrap();
        let bounds = v2_bounds(public_bytes(&root), 1);
        let cold = || {
            if signed {
                mori_ledger::signed::snapshot_signed_public_keyset(
                    &root,
                    &bounds,
                    mori_ledger::signed::SignedVersion::LegacyV1,
                )
            } else {
                snapshot_public_keyset(&root, &bounds)
            }
        };
        assert!(cold().is_ok());
        #[cfg(unix)]
        {
            use std::{ffi::CString, os::unix::ffi::OsStrExt};
            std::os::unix::fs::symlink(root.join("absent"), root.join("selected.pk.bin")).unwrap();
            let path =
                CString::new(root.join("proving-keyset.json").as_os_str().as_bytes()).unwrap();
            assert_eq!(unsafe { libc::mkfifo(path.as_ptr(), 0o600) }, 0);
            assert!(cold().is_ok());
            std::fs::remove_file(root.join("selected.pk.bin")).unwrap();
            std::fs::remove_file(root.join("proving-keyset.json")).unwrap();
            let path = CString::new(root.join("selected.pk.bin").as_os_str().as_bytes()).unwrap();
            assert_eq!(unsafe { libc::mkfifo(path.as_ptr(), 0o600) }, 0);
            std::os::unix::fs::symlink(root.join("absent"), root.join("proving-keyset.json"))
                .unwrap();
            assert!(cold().is_ok());
        }
        // Branch inspection also verifies ignored names never reach read_leaf;
        // this is not a timing-only observation.
        std::fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn v2_actual_signed_and_unsigned_reopen_refuse_lowered_domains_before_preflight() {
    use mori_ledger::profile::{FrameBindings, InstalledDeployment};
    for signed in [false, true] {
        let (root, public, pk) = proving_fixture_profile(signed);
        let original = v2_bounds(public, pk);
        let imported = snapshot_profile(&root, &original, signed).unwrap();
        let manifest: PublicKeysetManifest =
            serde_json::from_slice(&std::fs::read(root.join("public-keyset.json")).unwrap())
                .unwrap();
        let f = support::payment();
        let frame = FrameBindings {
            predecessor_hash: f.deployment.predecessor_hash,
            native_state_hash: f.deployment.native_state_hash,
            parameters_hash: f.deployment.parameters_hash,
            gas_hex: f.deployment.gas_hex,
            operations_hash: f.deployment.operations_hash,
            maintenance_hash: f.deployment.maintenance_hash,
            balance_hash: f.deployment.balance_hash,
        };
        // Deliberately invalid profile is a source-preflight sentinel. The
        // lowered import budget must win before this sentinel or native work.
        let descriptor = InstalledDeployment {
            profile: "invalid-preflight-sentinel".into(),
            qualification: mori_ledger::profile::INSTALLED_QUALIFICATION.into(),
            phase_policy: "guaranteed-only".into(),
            bindings: manifest.bindings.clone(),
            frame: frame.clone(),
            keyset_identity: manifest.identity.clone(),
        };
        for lower_pk in [true, false] {
            let mut lowered = original.clone();
            if lower_pk {
                lowered.max_prover_key_bytes -= 1;
            } else {
                lowered.max_artifact_bytes -= 1;
            }
            let error = if signed {
                let policy = manifest.signer_policy.clone().unwrap();
                let signed_descriptor = mori_ledger::signed::SignedInstalledDeployment {
                    profile: descriptor.profile.clone(),
                    qualification: mori_ledger::signed::QUALIFICATION.into(),
                    phase_policy: "guaranteed-only".into(),
                    storage_abi: mori_ledger::profile::ABI.into(),
                    signed_abi: mori_ledger::signed::SIGNED_ABI.into(),
                    bindings: manifest.bindings.clone(),
                    frame: frame.clone(),
                    keyset_identity: manifest.identity.clone(),
                    signer_policy: policy.clone(),
                };
                futures::executor::block_on(
                    mori_ledger::signed::reopen_signed_local_proving_deployment(
                        &f.source,
                        &f.program,
                        &signed_descriptor,
                        &policy,
                        &imported,
                        "settle",
                        &root.join("never-open-parameters"),
                        &lowered,
                    ),
                )
                .err()
                .unwrap()
            } else {
                futures::executor::block_on(
                    mori_ledger::local_deployment::reopen_local_proving_deployment(
                        &f.source,
                        &f.program,
                        &descriptor,
                        &imported,
                        "settle",
                        &root.join("never-open-parameters"),
                        &lowered,
                    ),
                )
                .err()
                .unwrap()
            };
            assert_eq!(
                error.code, "import-work-limit",
                "signed={signed} lower-pk={lower_pk}"
            );
        }
        if signed {
            let policy = manifest.signer_policy.clone().unwrap();
            let d = mori_ledger::signed::SignedInstalledDeployment {
                profile: descriptor.profile.clone(),
                qualification: mori_ledger::signed::QUALIFICATION.into(),
                phase_policy: "guaranteed-only".into(),
                storage_abi: mori_ledger::profile::ABI.into(),
                signed_abi: mori_ledger::signed::SIGNED_ABI.into(),
                bindings: manifest.bindings.clone(),
                frame,
                keyset_identity: manifest.identity,
                signer_policy: policy.clone(),
            };
            let mut legacy = original.clone();
            legacy.profile = "moriarty-native-call-work-limits/1".into();
            legacy.max_actions = 0;
            let error = futures::executor::block_on(
                mori_ledger::signed::reopen_signed_local_proving_deployment(
                    &f.source,
                    &f.program,
                    &d,
                    &policy,
                    &imported,
                    "wrong-action",
                    &root.join("never-open-parameters"),
                    &legacy,
                ),
            )
            .err()
            .unwrap();
            assert_eq!(
                error.code, "import-work-limit",
                "preserve v1 action refusal before later limits validation"
            );
        }
        std::fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn v1_signed_matching_action_invalid_unrelated_field_reaches_original_validation() {
    use mori_ledger::profile::FrameBindings;
    let (root, public, pk) = proving_fixture_profile(true);
    let imported = snapshot_profile(&root, &v2_bounds(public, pk), true).unwrap();
    let manifest: PublicKeysetManifest =
        serde_json::from_slice(&std::fs::read(root.join("public-keyset.json")).unwrap()).unwrap();
    let f = support::payment();
    let policy = manifest.signer_policy.unwrap();
    let d = mori_ledger::signed::SignedInstalledDeployment {
        profile: mori_ledger::signed::PROFILE.into(),
        qualification: mori_ledger::signed::QUALIFICATION.into(),
        phase_policy: "guaranteed-only".into(),
        storage_abi: mori_ledger::profile::ABI.into(),
        signed_abi: mori_ledger::signed::SIGNED_ABI.into(),
        bindings: manifest.bindings,
        frame: FrameBindings {
            predecessor_hash: f.deployment.predecessor_hash,
            native_state_hash: f.deployment.native_state_hash,
            parameters_hash: f.deployment.parameters_hash,
            gas_hex: f.deployment.gas_hex,
            operations_hash: f.deployment.operations_hash,
            maintenance_hash: f.deployment.maintenance_hash,
            balance_hash: f.deployment.balance_hash,
        },
        keyset_identity: manifest.identity,
        signer_policy: policy.clone(),
    };
    let mut legacy = limits();
    legacy.max_leaf_bytes = pk;
    legacy.max_prover_key_bytes = pk;
    legacy.max_artifact_bytes = public + pk;
    legacy.max_operations = 0;
    let error =
        futures::executor::block_on(mori_ledger::signed::reopen_signed_local_proving_deployment(
            &f.source,
            &f.program,
            &d,
            &policy,
            &imported,
            "settle",
            &root.join("never-open-parameters"),
            &legacy,
        ))
        .err()
        .unwrap();
    assert_eq!(error.layer, "input");
    assert_eq!(error.code, "native-work-limits");
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn v2_proving_snapshot_separates_selected_pk_from_public_leaf_and_aggregate() {
    let (root, public_total, pk_bytes) = proving_fixture();
    let mut bounds = limits();
    bounds.max_leaf_bytes = public_total;
    bounds.max_artifact_bytes = public_total;
    bounds.max_prover_key_bytes = pk_bytes;
    assert!(snapshot_proving_keyset(&root, "settle", &bounds).is_err());
    bounds.profile = "moriarty-native-call-work-limits/2".into();
    let result = snapshot_proving_keyset(&root, "settle", &bounds);
    assert!(result.is_ok(), "{:?}", result.err());
    std::fs::remove_dir_all(root).unwrap();
}
