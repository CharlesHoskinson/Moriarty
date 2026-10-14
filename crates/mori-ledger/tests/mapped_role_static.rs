//! Public source-static compilation only. Native refusal tests stop before decoding.
use mori_ledger::static_compile;

#[test]
fn mapped_role_authorization_is_explicit_and_strict() {
    let mapped = static_compile::parse_authorization(r#"{"mode":"unsigned-mapped-roles"}"#);
    assert!(
        mapped.is_ok(),
        "explicit mapped-role mode must parse: {mapped:?}"
    );
    for invalid in [
        r#"{"mode":"unsigned-mapped-roles","signer_policy":{}}"#,
        r#"{"mode":"unsigned-mapped-roles","extra":true}"#,
        r#"{"mode":"unsigned-mapped-roles","extra":null}"#,
        r#"{"mode":"unsigned-mapped-roles","mode":"unsigned"}"#,
        r#"{"mode":"mapped"}"#,
    ] {
        assert!(static_compile::parse_authorization(invalid).is_err());
    }
}

use midnight_coin_structure::coin::{UnshieldedTokenType, UserAddress};
use midnight_ledger::structure::SignatureVerifyingKey;
use mori_core::Program;
use mori_ledger::{codec, layout, profile::*};
use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

const MIXED: &str = r#"language "moriarty/1"; agreement Mapped {
asset A = { network: "local", issuer: "fixture", reference: "A", scale: 0 };
state x: UInt = 0;
action fund(n: UInt) { transfer(A, "alice", "bob", n); }
action pay(n: UInt) { transfer(A, "bob", "treasury", n); fee(A, "alice", "treasury", 1); }
action idle() { next x = x + 1; }
}"#;
fn mode() -> static_compile::StaticAuthorization {
    static_compile::parse_authorization(r#"{"mode":"unsigned-mapped-roles"}"#).unwrap()
}
fn bindings(source: &str) -> (Program, StaticBindings) {
    let p = mori_core::parse(source).unwrap();
    let names = ["alice", "bob", "treasury"];
    let points = [
        "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        "02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9",
    ];
    let roles=names.into_iter().zip(points).map(|(role,point)| {
        let bytes=codec::unhex(point).unwrap();
        let vk=<midnight_base_crypto::ecdsa::VerifyingKey as midnight_serialize::Deserializable>::deserialize(&mut bytes.as_slice(),0).unwrap();
        let key=SignatureVerifyingKey::ECDSA(vk);
        RoleBinding { role:role.into(),public_key_hex:codec::native_hex(&key).unwrap(),address_hex:codec::native_hex(&UserAddress::from(key)).unwrap() }
    }).collect::<Vec<_>>();
    let assets = p
        .assets
        .iter()
        .enumerate()
        .map(|(i, a)| AssetBinding {
            identity: a.identity.clone(),
            token_hex: codec::native_hex(&UnshieldedTokenType(
                midnight_base_crypto::hash::HashOutput([i as u8 + 1; 32]),
            ))
            .unwrap(),
        })
        .collect::<Vec<_>>();
    let mut duty_map = BTreeMap::new();
    for st in p.actions.iter().flat_map(|a| &a.statements) {
        if let mori_core::Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            ..
        } = st
        {
            duty_map.insert(
                name.clone(),
                DutyBinding {
                    name: name.clone(),
                    asset: p
                        .assets
                        .iter()
                        .find(|a| &a.name == asset)
                        .unwrap()
                        .identity
                        .clone(),
                    debtor: debtor.clone(),
                    creditor: creditor.clone(),
                },
            );
        }
    }
    let duties = duty_map.into_values().collect::<Vec<_>>();
    let abi = layout::LedgerAbi {
        profile: ABI.into(),
        language: p.language.clone(),
        semantics: p.semantics.clone(),
        program_hash: codec::object_hash(&p).unwrap(),
        states: p
            .states
            .iter()
            .map(|st| layout::StateField {
                name: st.name.clone(),
                ty: st.ty,
                key_hex: codec::native_hex(&layout::name_key("state", &st.name)).unwrap(),
            })
            .collect(),
        actions: p.actions.clone(),
        assets: assets.clone(),
        roles: roles.clone(),
        duties: duties.clone(),
        templates_hash: layout::templates_hash(),
    };
    let b = StaticBindings {
        abi: ABI.into(),
        source_hash: codec::hash(source.as_bytes()),
        program_hash: abi.program_hash.clone(),
        abi_hash: codec::object_hash(&abi).unwrap(),
        templates_hash: abi.templates_hash,
        toolchain: Toolchain::pinned(),
        limits: Limits::supported(),
        network: "local-static-example".into(),
        anchor: "local-unqualified".into(),
        address_hex: "public-contract-domain".into(),
        payer: "alice".into(),
        assets,
        roles,
        duties,
        economic_bounds: None,
    };
    (p, b)
}
fn limits() -> NativeWorkLimits {
    NativeWorkLimits {
        profile: "moriarty-native-call-work-limits/2".into(),
        max_degree: 1,
        max_actions: 64,
        max_srs_bytes: 1,
        max_prover_key_bytes: 1,
        max_verifier_key_bytes: 1,
        max_input_bytes: 1_048_576,
        max_leaf_bytes: 16_777_216,
        max_artifact_bytes: 16_777_216,
        max_operations: 100_000,
        max_public_inputs: 65_536,
    }
}

fn materialize(bundle: &static_compile::StaticBundle, dir: &Dir) {
    for (name, bytes) in &bundle.leaves().leaves {
        std::fs::write(dir.0.join(name), bytes).unwrap();
    }
    std::fs::write(
        dir.0.join(bundle.leaves().marker_name),
        &bundle.leaves().marker,
    )
    .unwrap();
}
struct Dir(PathBuf);
impl Dir {
    fn new() -> Self {
        static ID: AtomicUsize = AtomicUsize::new(0);
        let p = std::env::temp_dir().join(format!(
            "mori-mapped-role-{}-{}",
            std::process::id(),
            ID.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Dir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn manifest(bundle: &static_compile::StaticBundle, i: usize) -> serde_json::Value {
    serde_json::from_slice(&bundle.leaves().leaves[&format!("action-{i:06}.manifest.json")])
        .unwrap()
}
#[test]
fn mixed_roles_export_reopen_and_debit_inventory_are_source_bound() {
    let (p, b) = bindings(MIXED);
    let a = static_compile::compile(MIXED, &p, &b, &mode(), &limits()).unwrap();
    let marker: serde_json::Value = serde_json::from_slice(&a.leaves().marker).unwrap();
    assert_eq!(
        marker["profile"],
        "moriarty-mapped-role-source-static-bundle/1"
    );
    assert_eq!(
        marker["qualification"],
        "source-static-mapped-roles-unproven-uninstalled-unauthenticated"
    );
    for (i, names) in [(0, vec!["alice"]), (1, vec!["alice", "bob"]), (2, vec![])] {
        let m = manifest(&a, i);
        assert_eq!(m["format"], "moriarty-mapped-role-source-static-relation/1");
        assert_eq!(
            m["qualification"],
            "source-static-mapped-debits-unauthenticated"
        );
        let rows = m["debit_sources"].as_array().unwrap();
        assert_eq!(rows.len(), names.len());
        for (row, name) in rows.iter().zip(names) {
            assert_eq!(
                *row,
                serde_json::to_value(b.roles.iter().find(|r| r.role == name).unwrap()).unwrap()
            );
        }
        assert_eq!(m["compiler_hash"], marker["build"]);
        assert_eq!(
            m["open_predicates"],
            serde_json::json!([
                "debit-source custody and funding ownership",
                "role and beneficiary authorization",
                "authenticated predecessor and deployment",
                "economic intention authority",
                "final intent/history/PCD",
                "native proof and ledger settlement"
            ])
        );
        assert!(m.get("signed").is_none());
    }
    let dir = Dir::new();
    materialize(&a, &dir);
    let reopened = static_compile::reopen(MIXED, &p, &b, &mode(), &dir.0, &limits()).unwrap();
    assert_eq!(a.identity(), reopened.identity());
    assert_eq!(a.leaves().leaves, reopened.leaves().leaves);
    assert_eq!(a.leaves().marker, reopened.leaves().marker);
    assert_eq!(
        static_compile::compile(
            MIXED,
            &p,
            &b,
            &static_compile::StaticAuthorization::Unsigned,
            &limits()
        )
        .err()
        .unwrap()
        .code,
        "one-public-payer"
    );
}
#[test]
fn mapped_statement_errors_have_asset_then_debit_then_recipient_precedence() {
    for (replacement, expected) in [
        (
            "transfer(A, \"missing\", \"missing\", n)",
            "source-debit-role",
        ),
        (
            "transfer(A, \"alice\", \"missing\", n)",
            "source-recipient-role",
        ),
        (
            "transfer(MISSING, \"missing\", \"missing\", n)",
            "core-validation",
        ),
    ] {
        let s = MIXED.replace("transfer(A, \"alice\", \"bob\", n)", replacement);
        let (mut p, b) = bindings(MIXED);
        if expected == "core-validation" {
            if let mori_core::Statement::Transfer {
                asset, from, to, ..
            } = &mut p.actions[0].statements[0]
            {
                *asset = "MISSING".into();
                *from = "missing".into();
                *to = "missing".into();
            }
            assert_eq!(
                static_compile::compile(&s, &p, &b, &mode(), &limits())
                    .err()
                    .unwrap()
                    .code,
                expected
            );
        } else {
            p = mori_core::parse(&s).unwrap();
            let mut binding = b.clone();
            binding.source_hash = codec::hash(s.as_bytes());
            binding.program_hash = codec::object_hash(&p).unwrap();
            assert_eq!(
                static_compile::compile(&s, &p, &binding, &mode(), &limits())
                    .err()
                    .unwrap()
                    .code,
                expected
            );
        }
    }
    let (p, mut b) = bindings(MIXED);
    b.assets.clear();
    b.roles.retain(|r| r.role != "bob");
    assert_eq!(
        static_compile::compile(MIXED, &p, &b, &mode(), &limits())
            .err()
            .unwrap()
            .code,
        "source-asset"
    );
}

fn rewrite_marker(dir: &Dir, marker: &mut serde_json::Value) {
    marker.as_object_mut().unwrap().remove("identity");
    marker["identity"] = codec::object_hash(marker).unwrap().into();
    std::fs::write(
        dir.0.join("static-compile.json"),
        serde_json::to_vec_pretty(marker).unwrap(),
    )
    .unwrap();
}
fn replace_leaf(
    dir: &Dir,
    marker: &mut serde_json::Value,
    name: &str,
    bytes: Vec<u8>,
    action: usize,
    field: &str,
) {
    let hash = codec::hash(&bytes);
    marker["leaves"][name] = serde_json::json!({"sha256":hash,"bytes":bytes.len()});
    marker["actions"][action][field] = hash.into();
    std::fs::write(dir.0.join(name), bytes).unwrap();
    rewrite_marker(dir, marker);
}
#[test]
fn hostile_ir_and_inventory_fail_fresh_recompile_despite_self_consistent_hashes() {
    let (p, b) = bindings(MIXED);
    let a = static_compile::compile(MIXED, &p, &b, &mode(), &limits()).unwrap();
    for inventory in [false, true] {
        let d = Dir::new();
        materialize(&a, &d);
        let mut m: serde_json::Value = serde_json::from_slice(&a.leaves().marker).unwrap();
        if inventory {
            let mut leaf = manifest(&a, 1);
            leaf["debit_sources"] = serde_json::json!([]);
            replace_leaf(
                &d,
                &mut m,
                "action-000001.manifest.json",
                serde_json::to_vec_pretty(&leaf).unwrap(),
                1,
                "manifest_hash",
            );
        } else {
            replace_leaf(
                &d,
                &mut m,
                "action-000001.ir.bin",
                vec![255; 128],
                1,
                "ir_hash",
            );
        }
        let err = static_compile::reopen(MIXED, &p, &b, &mode(), &d.0, &limits())
            .err()
            .unwrap();
        assert_eq!(
            err.code, "fresh-source-mismatch",
            "no generic SDK tagged decoding of hostile bytes"
        );
    }
}
#[test]
fn mapped_mode_profile_and_qualification_cannot_be_upgraded_or_downgraded() {
    let (p, b) = bindings(MIXED);
    let a = static_compile::compile(MIXED, &p, &b, &mode(), &limits()).unwrap();
    for field in ["profile", "qualification", "authorization"] {
        let d = Dir::new();
        materialize(&a, &d);
        let mut m: serde_json::Value = serde_json::from_slice(&a.leaves().marker).unwrap();
        m[field] = match field {
            "profile" => static_compile::PROFILE.into(),
            "qualification" => "native-proven".into(),
            _ => serde_json::json!({"mode":"unsigned"}),
        };
        rewrite_marker(&d, &mut m);
        assert_eq!(
            static_compile::reopen(MIXED, &p, &b, &mode(), &d.0, &limits())
                .err()
                .unwrap()
                .code,
            "marker-identity"
        );
    }
}
#[test]
fn mapped_bindings_retain_role_asset_and_duty_identity_checks() {
    for fault in 0..4 {
        let (p, mut b) = bindings(MIXED);
        let code = match fault {
            0 => {
                b.roles.push(b.roles[0].clone());
                "duplicate-role"
            }
            1 => {
                b.roles[1].public_key_hex = b.roles[0].public_key_hex.clone();
                b.roles[1].address_hex = b.roles[0].address_hex.clone();
                "role-address"
            }
            2 => {
                b.roles[1].address_hex = b.roles[0].address_hex.clone();
                "role-address"
            }
            _ => {
                b.assets.clear();
                "source-asset"
            }
        };
        assert_eq!(
            static_compile::compile(MIXED, &p, &b, &mode(), &limits())
                .err()
                .unwrap()
                .code,
            code
        );
    }
    let s = MIXED.replace(
        "next x = x + 1;",
        "liability(\"d\", A, \"bob\", \"treasury\", 1);",
    );
    let (p, mut b) = bindings(&s);
    b.duties[0].creditor = "alice".into();
    assert_eq!(
        static_compile::compile(&s, &p, &b, &mode(), &limits())
            .err()
            .unwrap()
            .code,
        "source-duty-identity"
    );
}
#[test]
fn stale_abi_rejects_changed_debit_and_fresh_abi_changes_manifest_identity() {
    let (p, b) = bindings(MIXED);
    let original = static_compile::compile(MIXED, &p, &b, &mode(), &limits()).unwrap();
    let s = MIXED.replace(
        "transfer(A, \"alice\", \"bob\", n)",
        "transfer(A, \"treasury\", \"bob\", n)",
    );
    let (next, coherent) = bindings(&s);
    let mut stale = b.clone();
    stale.source_hash = coherent.source_hash.clone();
    stale.program_hash = coherent.program_hash.clone();
    assert_eq!(
        static_compile::compile(&s, &next, &stale, &mode(), &limits())
            .err()
            .unwrap()
            .code,
        "static-abi-identity"
    );
    let altered = static_compile::compile(&s, &next, &coherent, &mode(), &limits()).unwrap();
    assert_eq!(
        original.leaves().leaves["action-000000.ir.bin"],
        altered.leaves().leaves["action-000000.ir.bin"],
        "recipient IR alone does not establish debit identity"
    );
    assert_ne!(original.identity(), altered.identity());
    assert_ne!(manifest(&original, 0), manifest(&altered, 0));
    let d = Dir::new();
    materialize(&altered, &d);
    assert_eq!(
        static_compile::reopen(MIXED, &p, &b, &mode(), &d.0, &limits())
            .err()
            .unwrap()
            .code,
        "marker-identity"
    );
}
#[test]
fn mapped_export_and_reopen_honor_action_operation_input_leaf_and_aggregate_limits() {
    let (p, b) = bindings(MIXED);
    let a = static_compile::compile(MIXED, &p, &b, &mode(), &limits()).unwrap();
    let d = Dir::new();
    materialize(&a, &d);
    for field in 0..5 {
        let mut l = limits();
        match field {
            0 => l.max_actions = 1,
            1 => l.max_operations = 1,
            2 => l.max_public_inputs = 1,
            3 => l.max_leaf_bytes = 1,
            _ => l.max_artifact_bytes = 1,
        }
        assert!(static_compile::compile(MIXED, &p, &b, &mode(), &l).is_err());
        assert!(static_compile::reopen(MIXED, &p, &b, &mode(), &d.0, &l).is_err());
    }
    let mut l = limits();
    l.max_artifact_bytes = (a.total_bytes() - 1) as u64;
    assert_eq!(
        static_compile::reopen(MIXED, &p, &b, &mode(), &d.0, &l)
            .err()
            .unwrap()
            .code,
        "aggregate-work-limit"
    );
}
