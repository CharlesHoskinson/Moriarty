use std::process::Command;
#[test]
fn actual_mori_dispatch_exposes_all_signed_consumers() {
    let output = Command::new(env!("CARGO_BIN_EXE_mori"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).unwrap();
    for name in [
        "keygen-signed-call",
        "prepare-signed-call",
        "prove-signed-call",
        "verify-signed-call",
    ] {
        assert!(help.contains(name), "missing actual command: {name}");
    }
}
#[test]
fn four_actual_command_help_contracts_and_missing_input_refusals() {
    for name in [
        "keygen-signed-call",
        "prepare-signed-call",
        "prove-signed-call",
        "verify-signed-call",
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_mori"))
            .args([name, "--help"])
            .output()
            .unwrap();
        assert!(output.status.success());
        let help = String::from_utf8(output.stdout).unwrap();
        for flag in [
            "--signer-policy",
            "--action",
            "--work-limits",
            "--params-dir",
        ] {
            assert!(help.contains(flag), "{name}: {flag}");
        }
        let absent = Command::new(env!("CARGO_BIN_EXE_mori"))
            .arg(name)
            .output()
            .unwrap();
        assert!(!absent.status.success());
        if name.contains("prove") || name.contains("verify") {
            assert!(help.contains("--signed-attachment"));
        } else {
            assert!(!help.contains("--signed-attachment"));
        }
    }
}
#[path = "../../mori-ledger/tests/support/mod.rs"]
mod support;
#[test]
fn signed_keygen_actual_consumer_refuses_invalid_policy_before_native_work() {
    let root = std::env::temp_dir().join(format!("mori-signed-cli-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let f = support::payment();
    for (name,data) in [("source.mori",f.source.as_bytes().to_vec()),("deployment.json",serde_json::to_vec(&f.deployment).unwrap()),("state.bin",f.predecessor),("parameters.bin",f.parameters),("policy.json",b"{\"profile\":\"moriarty-ledger-fixed-jubjub-policy/1\",\"owner\":\"alice\",\"beneficiary\":\"bob\",\"public_key_hex\":\"bad\"}".to_vec())] {std::fs::write(root.join(name),data).unwrap();}
    let limits = mori_ledger::profile::NativeWorkLimits {
        profile: "moriarty-native-call-work-limits/1".into(),
        max_degree: 19,
        max_actions: 4,
        max_srs_bytes: 1,
        max_prover_key_bytes: 1,
        max_verifier_key_bytes: 1,
        max_input_bytes: 1048576,
        max_leaf_bytes: 1048576,
        max_artifact_bytes: 4194304,
        max_operations: 10000,
        max_public_inputs: 10000,
    };
    std::fs::write(
        root.join("limits.json"),
        serde_json::to_vec(&limits).unwrap(),
    )
    .unwrap();
    let mut command = Command::new(env!("CARGO_BIN_EXE_mori"));
    command
        .arg("keygen-signed-call")
        .arg(root.join("source.mori"));
    for (flag, name) in [
        ("--signer-policy", "policy.json"),
        ("--seed-deployment", "deployment.json"),
        ("--seed-state", "state.bin"),
        ("--parameters", "parameters.bin"),
        ("--params-dir", "no-srs"),
        ("--work-limits", "limits.json"),
        ("--out", "out"),
    ] {
        command.arg(flag).arg(root.join(name));
    }
    command.args(["--action", "settle"]);
    for profile in [
        "moriarty-native-call-work-limits/1",
        "moriarty-native-call-work-limits/2",
    ] {
        let mut current = limits.clone();
        current.profile = profile.into();
        std::fs::write(
            root.join("limits.json"),
            serde_json::to_vec(&current).unwrap(),
        )
        .unwrap();
        let result = command.output().unwrap();
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("strict-fens-refusal"));
    }
    assert!(!root.join("out").exists());
    assert!(!root.join("no-srs").exists());
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn actual_prepare_prove_verify_refuse_unsigned_profile_before_key_or_attachment_reads() {
    use mori_ledger::{
        profile::{FrameBindings, StaticBindings},
        signed::*,
    };
    let root =
        std::env::temp_dir().join(format!("mori-signed-downgrade-cli-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let f = support::payment();
    let policy=FixedSignerPolicy{profile:POLICY.into(),owner:"alice".into(),beneficiary:"bob".into(),public_key_hex:"0677506737143823730512c5525e0b30acb75906fa7d53f494ba689b78620133d4cdab997110c2f1e02bb16ebef816c9d0a6025386825581a688b9bfa326360a".into()};
    policy.validate().unwrap();
    // Untrusted signed-shaped DTO with an explicit legacy downgrade profile.
    let deployment=SignedInstalledDeployment{profile:mori_ledger::profile::INSTALLED_PROFILE.into(),qualification:QUALIFICATION.into(),phase_policy:"guaranteed-only".into(),storage_abi:mori_ledger::profile::ABI.into(),signed_abi:SIGNED_ABI.into(),bindings:serde_json::from_value::<StaticBindings>(serde_json::json!({"abi":f.deployment.abi,"source_hash":f.deployment.source_hash,"program_hash":f.deployment.program_hash,"abi_hash":f.deployment.abi_hash,"templates_hash":f.deployment.templates_hash,"toolchain":f.deployment.toolchain,"limits":f.deployment.limits,"network":f.deployment.network,"anchor":f.deployment.anchor,"address_hex":f.deployment.address_hex,"payer":f.deployment.payer,"assets":f.deployment.assets,"roles":f.deployment.roles,"duties":f.deployment.duties,"economic_bounds":null})).unwrap(),frame:FrameBindings{predecessor_hash:f.deployment.predecessor_hash,native_state_hash:f.deployment.native_state_hash,parameters_hash:f.deployment.parameters_hash,gas_hex:f.deployment.gas_hex,operations_hash:f.deployment.operations_hash,maintenance_hash:f.deployment.maintenance_hash,balance_hash:f.deployment.balance_hash},keyset_identity:String::new(),signer_policy:policy.clone()};
    std::fs::write(root.join("source.mori"), &f.source).unwrap();
    std::fs::write(
        root.join("policy.json"),
        serde_json::to_vec(&policy).unwrap(),
    )
    .unwrap();
    std::fs::write(
        root.join("deployment.json"),
        serde_json::to_vec(&deployment).unwrap(),
    )
    .unwrap();
    let limits = mori_ledger::profile::NativeWorkLimits {
        profile: "moriarty-native-call-work-limits/1".into(),
        max_degree: 19,
        max_actions: 4,
        max_srs_bytes: 1,
        max_prover_key_bytes: 1,
        max_verifier_key_bytes: 1,
        max_input_bytes: 1048576,
        max_leaf_bytes: 1048576,
        max_artifact_bytes: 4194304,
        max_operations: 10000,
        max_public_inputs: 10000,
    };
    std::fs::write(
        root.join("limits.json"),
        serde_json::to_vec(&limits).unwrap(),
    )
    .unwrap();
    for name in [
        "prepare-signed-call",
        "prove-signed-call",
        "verify-signed-call",
    ] {
        let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
        c.args(["--quiet", name]).arg(root.join("source.mori"));
        for (flag, path) in [
            ("--signer-policy", "policy.json"),
            ("--deployment", "deployment.json"),
            ("--predecessor", "unread-state.bin"),
            ("--parameters", "unread-parameters.bin"),
            ("--request", "unread-request.json"),
            ("--params-dir", "unread-srs"),
            ("--work-limits", "limits.json"),
        ] {
            c.arg(flag).arg(root.join(path));
        }
        c.args(["--action", "settle"]);
        if name == "prove-signed-call" {
            c.arg("--keyset").arg(root.join("unread-keyset"));
        } else {
            c.arg("--public-keyset").arg(root.join("unread-keyset"));
        }
        if name != "prepare-signed-call" {
            c.arg("--signed-attachment")
                .arg(root.join("unread-attachment.json"));
        }
        if name == "verify-signed-call" {
            c.arg("--proof").arg(root.join("unread-proof"));
        } else {
            c.arg("--out").arg(root.join("out"));
        }
        let result = c.output().unwrap();
        assert!(!result.status.success(), "{name}");
        assert!(result.stdout.is_empty());
        assert!(
            String::from_utf8_lossy(&result.stderr).contains("deployment-profile"),
            "{name}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(!root.join("out").exists());
    }
    std::fs::remove_dir_all(root).unwrap();
}
