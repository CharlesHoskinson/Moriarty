use mori_backend::{
    Target,
    proving::{self, ProofBundle, ProofOptions},
};
use mori_core::{Scenario, UInt, Value};
use std::{collections::BTreeMap, path::PathBuf};

const SOURCE: &str = r#"language "moriarty/1";
agreement AuthoredProof {
  state count: UInt = 7;
  state live: Bool = true;
  action advance(amount: UInt) {
    next count = count + amount;
    next count = count + 1;
    emit "count"(count);
  }
  action toggle() { next live = not live; }
}"#;
fn scenario() -> Scenario {
    Scenario {
        inputs: BTreeMap::from([("amount".into(), Value::UInt(UInt(3)))]),
        ..Default::default()
    }
}
fn options() -> ProofOptions {
    ProofOptions {
        params_dir: PathBuf::from("/not-present/moriarty-proof-tests"),
        max_k: 9,
    }
}
fn assert_code(
    result: Result<(ProofBundle, proving::ProofReceipt), mori_backend::Error>,
    suffix: &str,
) {
    let e = result.unwrap_err();
    assert_eq!(e.code, format!("mori::backend::{suffix}"), "{e}");
}
#[test]
fn refusal_precedes_native_parameters() {
    let mut target = Target::ledger9("local", "anchor");
    target.ledger_revision = "untrusted".into();
    assert_code(
        proving::prove(SOURCE, "advance", &scenario(), &target, &options()),
        "target",
    );
    target = Target::ledger9("local", "anchor");
    assert_code(
        proving::prove(SOURCE, "absent", &scenario(), &target, &options()),
        "action",
    );
    let mut wrong = scenario();
    wrong.inputs.clear();
    assert_code(
        proving::prove(SOURCE, "advance", &wrong, &target, &options()),
        "scenario",
    );
    for max_k in [0, 20, u8::MAX] {
        let mut opt = options();
        opt.max_k = max_k;
        assert_code(
            proving::prove(SOURCE, "advance", &scenario(), &target, &opt),
            "limit",
        );
    }
    let mut opt = options();
    opt.max_k = 1;
    assert_code(
        proving::prove(SOURCE, "advance", &scenario(), &target, &opt),
        "limit",
    );
}
#[test]
fn missing_parameters_are_offline_refusal() {
    assert_code(
        proving::prove(
            SOURCE,
            "advance",
            &scenario(),
            &Target::ledger9("local", "anchor"),
            &options(),
        ),
        "parameters",
    );
}
#[test]
fn financial_fixture_resource_measurement_without_parameters_or_proving() {
    use midnight_transient_crypto::proofs::Zkir;
    let source = include_str!("../../../examples/core/native-payment/payment.mori");
    let artifact = mori_backend::compile(
        &mori_core::parse(source).unwrap(),
        &Target::ledger9("local", "native-proof-qa"),
    )
    .unwrap();
    let ir = midnight_zkir_v3::IrSource::load(artifact.circuits[0].zkir.as_bytes()).unwrap();
    let model = ir.model();
    println!(
        "native_payment_optimal_k={} model_k={} model_rows={}",
        ir.k(),
        model.k(),
        model.rows()
    );
    assert!(ir.k() <= 19);
}
#[test]
fn strict_bundle_schema_and_hex() {
    assert!(ProofBundle::from_json(r#"{"format":"x","format":"y"}"#).is_err());
    assert!(ProofBundle::from_json("{} []").is_err());
    let bundle = ProofBundle {
        format: "moriarty-native-transition-proof/2".into(),
        abi: "moriarty-public-abi/2".into(),
        relation: "moriarty-zkir-relation/2".into(),
        binding: "moriarty-native-transition-binding/2".into(),
        profile: "ledger-9".into(),
        registry: vec![],
        source_hash: "x".into(),
        program_hash: "x".into(),
        target_hash: "x".into(),
        scenario_hash: "x".into(),
        execution_hash: "x".into(),
        action: "advance".into(),
        circuit_k: 9,
        proof_hex: "".into(),
    };
    let current = serde_json::to_value(&bundle).unwrap();
    assert!(ProofBundle::from_json(&current.to_string()).is_ok());
    for field in ["format", "abi", "relation", "binding"] {
        let mut stale = current.clone();
        stale[field] = "stale/1".into();
        assert!(ProofBundle::from_json(&stale.to_string()).is_err());
    }
    for field in ["abi", "relation", "binding", "profile", "registry"] {
        let mut missing = current.clone();
        missing.as_object_mut().unwrap().remove(field);
        assert!(ProofBundle::from_json(&missing.to_string()).is_err());
    }
    let mut json = current;
    json["verifier_key"] = "hostile".into();
    assert!(ProofBundle::from_json(&json.to_string()).is_err());
}
#[test]
fn failed_conditions_and_residual_liabilities_refuse() {
    let source = r#"language "moriarty/1"; agreement Duties {
        asset USD = { network: "local", issuer: "issuer", reference: "usd", scale: 2 };
        action condition(ok: Bool) { requires ok; }
    }"#;
    let mut s = Scenario::default();
    s.inputs.insert("ok".into(), Value::Bool(false));
    let target = Target::ledger9("local", "anchor");
    assert_code(
        proving::prove(source, "condition", &s, &target, &options()),
        "scenario",
    );
    s.inputs.insert("ok".into(), Value::Bool(true));
    s.liabilities.push(mori_core::Liability {
        name: "unpaid".into(),
        asset: mori_core::AssetId {
            network: "local".into(),
            issuer: "issuer".into(),
            reference: "usd".into(),
            scale: 2,
        },
        debtor: "payer".into(),
        creditor: "recipient".into(),
        amount: UInt(1),
    });
    assert_code(
        proving::prove(source, "condition", &s, &target, &options()),
        "unsupported",
    );
}
#[test]
fn corrupted_and_hostile_srs_headers_refuse_before_decode() {
    let directory =
        std::env::temp_dir().join(format!("moriarty-srs-rejection-{}", std::process::id()));
    std::fs::create_dir(&directory).unwrap();
    let source = r#"language "moriarty/1"; agreement Small { action run() { emit "value"(1); } }"#;
    let k = {
        use midnight_transient_crypto::proofs::Zkir;
        let artifact = mori_backend::compile(
            &mori_core::parse(source).unwrap(),
            &Target::ledger9("local", "anchor"),
        )
        .unwrap();
        midnight_zkir_v3::IrSource::load(artifact.circuits[0].zkir.as_bytes())
            .unwrap()
            .k()
    };
    let opt = ProofOptions {
        params_dir: directory.clone(),
        max_k: 9,
    };
    let file = directory.join(format!("bls_midnight_2p{k}"));
    for bytes in [vec![], vec![255; 4], vec![0; 1024], vec![9, 0, 0, 0, 1]] {
        std::fs::write(&file, bytes).unwrap();
        assert_code(
            proving::prove(
                source,
                "run",
                &Scenario::default(),
                &Target::ledger9("local", "anchor"),
                &opt,
            ),
            "parameters",
        );
    }
    std::fs::remove_dir_all(directory).unwrap();
}

/// Explicit invocation requires a caller-provided pinned, local SRS directory.
#[test]
#[ignore = "genuine native k<=9 control; set MORIARTY_TEST_PARAMS_DIR explicitly"]
fn genuine_native_authored_actions_and_mutations() {
    let options = ProofOptions {
        params_dir: std::env::var_os("MORIARTY_TEST_PARAMS_DIR")
            .expect("explicit local parameters required")
            .into(),
        max_k: 9,
    };
    let target = Target::ledger9("local", "anchor");
    {
        use midnight_transient_crypto::proofs::Zkir;
        let artifact = mori_backend::compile(&mori_core::parse(SOURCE).unwrap(), &target).unwrap();
        for circuit in &artifact.circuits {
            let k = midnight_zkir_v3::IrSource::load(circuit.zkir.as_bytes())
                .unwrap()
                .k();
            println!(
                "authored_action={} native_k={k} model_rows={}",
                circuit.action,
                midnight_zkir_v3::IrSource::load(circuit.zkir.as_bytes())
                    .unwrap()
                    .model()
                    .rows()
            );
            assert!(k <= 9, "explicit bounded native test encoding changed");
        }
    }
    let s = scenario();
    let (bundle, receipt) = proving::prove(SOURCE, "advance", &s, &target, &options).unwrap();
    assert_eq!(receipt.qualification, "local-native-transition-proof");
    assert_eq!(receipt.execution.post.state["count"], Value::UInt(UInt(11)));
    assert_eq!(
        proving::verify(SOURCE, "advance", &s, &target, &bundle, &options)
            .unwrap()
            .execution,
        receipt.execution
    );
    for suffix in ["00", "ff0081"] {
        let mut bad = bundle.clone();
        bad.proof_hex.push_str(suffix);
        assert_eq!(
            proving::verify(SOURCE, "advance", &s, &target, &bad, &options)
                .unwrap_err()
                .code,
            "mori::backend::verification"
        );
    }
    for proof_hex in [
        String::new(),
        bundle.proof_hex[..20].to_owned(),
        "00".repeat(proving::MAX_PROOF_BYTES + 1),
        "AA".into(),
    ] {
        let mut bad = bundle.clone();
        bad.proof_hex = proof_hex;
        assert!(proving::verify(SOURCE, "advance", &s, &target, &bad, &options).is_err());
    }
    let mut bad = bundle.clone();
    bad.proof_hex.replace_range(
        ..2,
        if &bundle.proof_hex[..2] == "00" {
            "01"
        } else {
            "00"
        },
    );
    assert_eq!(
        proving::verify(SOURCE, "advance", &s, &target, &bad, &options)
            .unwrap_err()
            .code,
        "mori::backend::verification"
    );
    for field in [
        "source_hash",
        "program_hash",
        "target_hash",
        "scenario_hash",
        "execution_hash",
        "action",
    ] {
        let mut v = serde_json::to_value(&bundle).unwrap();
        v[field] = "wrong".into();
        let bad: ProofBundle = serde_json::from_value(v).unwrap();
        assert_eq!(
            proving::verify(SOURCE, "advance", &s, &target, &bad, &options)
                .unwrap_err()
                .code,
            "mori::backend::claim"
        );
    }
    assert!(
        proving::verify(
            &format!("{SOURCE}\n"),
            "advance",
            &s,
            &target,
            &bundle,
            &options
        )
        .is_err()
    );
    // Alter metadata to match the changed claim, so this control reaches the
    // real verifier with a different independently derived binding_input.
    {
        use sha2::{Digest, Sha256};
        let changed_source = format!("{SOURCE}\n");
        let mut bad = bundle.clone();
        bad.source_hash = Sha256::digest(changed_source.as_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert_eq!(
            proving::verify(&changed_source, "advance", &s, &target, &bad, &options)
                .unwrap_err()
                .code,
            "mori::backend::verification"
        );
    }
    let mut changed = s.clone();
    changed.inputs.insert("amount".into(), Value::UInt(UInt(4)));
    assert!(proving::verify(SOURCE, "advance", &changed, &target, &bundle, &options).is_err());
    {
        use sha2::{Digest, Sha256};
        let hash = |v: &serde_json::Value| -> String {
            Sha256::digest(mori_core::intent::canonical(v).unwrap())
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect()
        };
        let e =
            mori_core::simulate(&mori_core::parse(SOURCE).unwrap(), "advance", &changed).unwrap();
        let mut bad = bundle.clone();
        bad.scenario_hash = hash(&serde_json::to_value(&changed).unwrap());
        bad.execution_hash = hash(&serde_json::to_value(&e).unwrap());
        assert_eq!(
            proving::verify(SOURCE, "advance", &changed, &target, &bad, &options)
                .unwrap_err()
                .code,
            "mori::backend::verification"
        );
        let alternate = Target::ledger9("local", "different-anchor");
        let mut bad = bundle.clone();
        bad.target_hash = hash(&serde_json::to_value(&alternate).unwrap());
        assert_eq!(
            proving::verify(SOURCE, "advance", &s, &alternate, &bad, &options)
                .unwrap_err()
                .code,
            "mori::backend::verification"
        );
    }
    let (other, _) =
        proving::prove(SOURCE, "toggle", &Scenario::default(), &target, &options).unwrap();
    proving::verify(
        SOURCE,
        "toggle",
        &Scenario::default(),
        &target,
        &other,
        &options,
    )
    .unwrap();
    let mut substituted = bundle.clone();
    substituted.proof_hex = other.proof_hex;
    assert_eq!(
        proving::verify(SOURCE, "advance", &s, &target, &substituted, &options)
            .unwrap_err()
            .code,
        "mori::backend::verification"
    );
    let mut explicit = s.clone();
    explicit.state = receipt.execution.post.state.clone();
    let (second, second_receipt) =
        proving::prove(SOURCE, "advance", &explicit, &target, &options).unwrap();
    assert_eq!(
        second_receipt.execution.post.state["count"],
        Value::UInt(UInt(15))
    );
    proving::verify(SOURCE, "advance", &explicit, &target, &second, &options).unwrap();
}

#[test]
fn registered_duties_reach_missing_srs_and_closed_registry_refuses() {
    let target = Target::ledger9("local", "anchor");
    let mut opts = options();
    opts.max_k = 19;
    let settlement = include_str!("../../../examples/core/native-liability/settlement.mori");
    let zero = include_str!("../../../examples/core/native-liability/zero-lifecycle.mori");
    let manifest: serde_json::Value = serde_json::from_str(include_str!(
        "../../../examples/core/native-liability/mori-tests.json"
    ))
    .unwrap();
    let scenarios = [
        include_str!("../../../examples/core/native-liability/partial.scenario.json"),
        include_str!("../../../examples/core/native-liability/full.scenario.json"),
        include_str!("../../../examples/core/native-liability/present-zero.scenario.json"),
    ];
    for (i, text) in scenarios.into_iter().enumerate() {
        let s: Scenario = serde_json::from_str(text).unwrap();
        let (source, action) = if i == 2 {
            (zero, "close")
        } else {
            (settlement, "repay")
        };
        let execution =
            mori_core::simulate(&mori_core::parse(source).unwrap(), action, &s).unwrap();
        assert_eq!(
            serde_json::to_value(&execution.post).unwrap(),
            manifest["cases"][i]["expected"]["post"]
        );
        assert_eq!(
            serde_json::to_value(&execution.effects).unwrap(),
            manifest["cases"][i]["expected"]["effects"]
        );
        assert_code(
            proving::prove(source, action, &s, &target, &opts),
            "parameters",
        );
    }
    for text in [
        include_str!(
            "../../../examples/core/native-liability/reference-broader-wrong-party.scenario.json"
        ),
        include_str!(
            "../../../examples/core/native-liability/reference-broader-hidden-name.scenario.json"
        ),
    ] {
        let s: Scenario = serde_json::from_str(text).unwrap();
        assert!(mori_core::simulate(&mori_core::parse(settlement).unwrap(), "repay", &s).is_ok());
        assert_code(
            proving::prove(settlement, "repay", &s, &target, &opts),
            "unsupported",
        );
    }
}

#[test]
fn stale_proof_revision_refuses_at_schema_boundary() {
    let old = serde_json::json!({"format":"moriarty-native-transition-proof/1","source_hash":"x","program_hash":"x","target_hash":"x","scenario_hash":"x","execution_hash":"x","action":"advance","circuit_k":9,"proof_hex":"00"});
    assert!(ProofBundle::from_json(&old.to_string()).is_err());
}

#[test]
fn source_registry_validates_all_scenario_tuples_and_untouched_slots() {
    let source = include_str!("../../../examples/core/native-liability/settlement.mori");
    let scenario: Scenario = serde_json::from_str(include_str!(
        "../../../examples/core/native-liability/partial.scenario.json"
    ))
    .unwrap();
    let target = Target::ledger9("local", "anchor");
    let opts = ProofOptions {
        max_k: 19,
        ..options()
    };
    // No duty effect is emitted by observe, yet the complete registry persists.
    let mut untouched = scenario.clone();
    untouched.inputs.clear();
    assert_code(
        proving::prove(source, "observe", &untouched, &target, &opts),
        "parameters",
    );
    for mutation in 0..8 {
        let mut bad = untouched.clone();
        let duty = &mut bad.liabilities[1];
        match mutation {
            0 => duty.name = "hidden".into(),
            1 => duty.debtor = "changed".into(),
            2 => duty.creditor = "changed".into(),
            3 => duty.asset.network = "changed".into(),
            4 => duty.asset.issuer = "changed".into(),
            5 => duty.asset.reference = "changed".into(),
            6 => duty.asset.scale = 1,
            _ => {
                let copy = duty.clone();
                bad.liabilities.push(copy);
            }
        }
        assert_code(
            proving::prove(source, "observe", &bad, &target, &opts),
            "unsupported",
        );
    }
    // Absence is allowed for a registered untouched identity and consumes a slot.
    untouched.liabilities.clear();
    assert_code(
        proving::prove(source, "observe", &untouched, &target, &opts),
        "parameters",
    );
}
