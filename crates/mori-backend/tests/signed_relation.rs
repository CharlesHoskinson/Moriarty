//! Signed compiler and host reconstruction. Ordinary compile/reference checks
//! only: no SRS, key generation, proving, MockProver or native verification.
#![recursion_limit = "256"]
use midnight_curves::pairing::group::Group;
use midnight_curves::{Fr as Scalar, JubjubAffine, JubjubExtended, JubjubSubgroup};
use midnight_transient_crypto::{curve::Fr, hash::transient_hash};
use midnight_zkir_v3::{Instruction, IrSource, ir::Operand};
use mori_backend::{
    Target,
    authorization::{ExpectedAuthority, SignedIntent},
    proving::{self, ProofOptions},
    schnorr, signed_relation,
};
use mori_core::{Scenario, UInt};
use serde_json::{Value, json};

fn golden() -> Value {
    serde_json::from_str(include_str!(
        "../../../examples/core/native-owner-intent/golden-vector.json"
    ))
    .unwrap()
}
fn golden_target() -> Target {
    Target::from_json(golden()["target_canonical_utf8"].as_str().unwrap()).unwrap()
}
fn golden_scenario() -> Scenario {
    serde_json::from_value(golden()["scenario"].clone()).unwrap()
}
fn authority_of(payload: &Value) -> ExpectedAuthority {
    let mut a = serde_json::Map::new();
    for k in [
        "profile",
        "network",
        "anchor",
        "program_address",
        "target_hash",
        "owner",
        "beneficiary",
        "public_key_hex",
        "nonce",
        "predecessor",
        "not_before",
        "not_after",
        "limits",
    ] {
        a.insert(k.into(), payload[k].clone());
    }
    ExpectedAuthority::from_json(&Value::Object(a).to_string()).unwrap()
}
fn golden_authority() -> ExpectedAuthority {
    authority_of(&golden()["payload"])
}
fn golden_signed() -> SignedIntent {
    SignedIntent::from_json(&golden()["signed_wrapper"].to_string()).unwrap()
}
fn without(value: &Value, field: &str) -> Value {
    let mut v = value.clone();
    v.as_object_mut().unwrap().remove(field);
    v
}
fn decimal_hex(decimal: &str) -> String {
    let mut n = [0u8; 32];
    for d in decimal.bytes() {
        let mut carry = u16::from(d - b'0');
        for b in n.iter_mut() {
            let v = u16::from(*b) * 10 + carry;
            *b = v as u8;
            carry = v >> 8;
        }
        assert_eq!(carry, 0);
    }
    mori_core::intent::hex(&n)
}
fn params() -> ProofOptions {
    ProofOptions {
        params_dir: "/nonexistent/moriarty-signed-task2".into(),
        max_k: 19,
    }
}

// Public QA-only signer for test fixtures; no wallet or production custody path.
fn xy(p: JubjubSubgroup) -> (Fr, Fr) {
    let a = JubjubAffine::from(JubjubExtended::from(p));
    (Fr(a.get_u()), Fr(a.get_v()))
}
fn qa_sign(secret: u64, nonce: u64, message: Fr) -> String {
    let g = JubjubSubgroup::generator();
    let (x, k) = (Scalar::from(secret), Scalar::from(nonce));
    let (r, pk) = (g * k, g * x);
    let (rx, ry) = xy(r);
    let (px, py) = xy(pk);
    let h = transient_hash(&[Fr::from(0x46454e53u64), rx, ry, px, py, message]);
    let mut wide = [0u8; 64];
    wide[..32].copy_from_slice(&h.as_le_bytes());
    let s = k + Scalar::from_bytes_wide(&wide) * x;
    let mut bytes = rx.as_le_bytes();
    bytes.extend(ry.as_le_bytes());
    bytes.extend(s.to_bytes());
    mori_core::intent::hex(&bytes)
}

#[test]
fn signed_target_policy_is_omitted_when_absent_and_null_refuses() {
    let unsigned = Target::ledger9("local", "anchor");
    let json = serde_json::to_value(&unsigned).unwrap();
    assert!(json.get("signed_policy").is_none());
    let mut null = json.clone();
    null["signed_policy"] = Value::Null;
    assert!(Target::from_json(&null.to_string()).is_err());
    let signed = golden_target();
    assert_eq!(
        mori_core::intent::canonical(&signed).unwrap(),
        golden()["target_canonical_utf8"]
            .as_str()
            .unwrap()
            .as_bytes()
    );
    let mut extra = serde_json::to_value(&signed).unwrap();
    extra["signed_policy"]["key"] = "x".into();
    assert!(Target::from_json(&extra.to_string()).is_err());
    let mut control = serde_json::to_value(&signed).unwrap();
    control["signed_policy"]["owner"] = "al\u{7}ice".into();
    assert!(Target::from_json(&control.to_string()).is_err());
}

#[test]
fn golden_signed_compile_stages_exact_context_and_authorization_layout() {
    let g = golden();
    let program = mori_core::parse(g["source_utf8"].as_str().unwrap()).unwrap();
    let artifact = mori_backend::compile_signed(&program, &golden_target()).unwrap();
    assert_eq!(artifact.format, "moriarty-zkir-artifact/3");
    assert_eq!(artifact.abi, "moriarty-public-abi/3");
    assert_eq!(artifact.relation, "moriarty-zkir-relation/3");
    assert_eq!(
        artifact.authorization_mode,
        "moriarty-jubjub-owner-intent/1"
    );
    let c = artifact
        .circuits
        .iter()
        .find(|c| c.action == "repay")
        .unwrap();
    assert_eq!(c.context_hash, g["context_hash"]);
    assert_eq!(c.context_hash, g["payload"]["relation_hash"]);
    // All 28 financial descriptors keep role/name/type/asset/constant and order.
    let financial = g["financial_bindings"].as_array().unwrap();
    assert_eq!(c.public_inputs.len(), 28);
    for (i, (b, want)) in c.public_inputs.iter().zip(financial).enumerate() {
        let mut have = serde_json::to_value(b).unwrap();
        have["position"] = json!(i);
        assert_eq!(have, without(want, "value"), "financial position {i}");
    }
    let auth = g["authorization_bindings"].as_array().unwrap();
    assert_eq!(c.authorization_bindings.len(), 15);
    for (b, want) in c.authorization_bindings.iter().zip(auth) {
        assert_eq!(serde_json::to_value(b).unwrap(), without(want, "value"));
    }
    assert_eq!(
        serde_json::to_value(&c.private_inputs).unwrap(),
        json!([{"name":"public_key","ty":"Point<Jubjub>"},{"name":"R","ty":"Point<Jubjub>"},{"name":"response","ty":"Native"}])
    );
    assert_eq!(
        serde_json::to_value(&c.effect_frame).unwrap(),
        g["effect_frame"]
    );
    assert_eq!(
        serde_json::to_value(&c.balance_frame).unwrap(),
        g["balance_frame"]
    );
    assert_eq!(
        serde_json::to_value(&c.liability_frame).unwrap(),
        g["liability_frame"]
    );
    assert_eq!(c.replay_steps, 6);
    // Unchanged unsigned source behavior remains available, with its own context.
    let unsigned =
        mori_backend::compile(&program, &Target::ledger9("local", "native-liability-qa")).unwrap();
    assert_eq!(unsigned.authorization_mode, "unsigned");
    let u = unsigned
        .circuits
        .iter()
        .find(|c| c.action == "repay")
        .unwrap();
    assert!(u.authorization_bindings.is_empty() && u.private_inputs.is_empty());
    assert_eq!(u.public_inputs.len(), 28);
    assert_ne!(u.context_hash, c.context_hash);
}

#[test]
fn golden_zkir_binds_direct_financial_operands_message_key_and_unconditional_impacts() {
    let g = golden();
    let program = mori_core::parse(g["source_utf8"].as_str().unwrap()).unwrap();
    let artifact = mori_backend::compile_signed(&program, &golden_target()).unwrap();
    let c = artifact
        .circuits
        .iter()
        .find(|c| c.action == "repay")
        .unwrap();
    let ir = IrSource::load(c.zkir.as_bytes()).unwrap();
    assert!(!ir.do_communications_commitment && ir.inputs.is_empty());
    let mut publics = Vec::new();
    let mut privates = Vec::new();
    let mut impacts = Vec::new();
    let mut hashes = Vec::new();
    let mut asserts = 0;
    for ins in ir.instructions.iter() {
        match ins {
            Instruction::PublicInput { guard, output, .. } => {
                assert!(guard.is_none());
                publics.push(Operand::Variable(output.clone()));
            }
            Instruction::PrivateInput { guard, val_t, .. } => {
                assert!(guard.is_none());
                privates.push(serde_json::to_value(val_t).unwrap());
            }
            Instruction::Impact { guard, inputs } => {
                assert_eq!(guard, &Operand::Immediate(Fr::from(1u64)));
                impacts.extend(inputs.iter().cloned());
            }
            Instruction::TransientHash { inputs, .. } => hashes.push(inputs.clone()),
            Instruction::Assert { .. } => asserts += 1,
            _ => {}
        }
    }
    // Every one of N+11+4A positions is read and published unconditionally, in order.
    assert_eq!(publics.len(), 43);
    assert_eq!(impacts, publics);
    assert_eq!(
        privates,
        vec![
            json!("Point<Jubjub>"),
            json!("Point<Jubjub>"),
            json!("Scalar<BLS12-381>")
        ]
    );
    let message = hashes
        .iter()
        .find(|h| h.len() == 75)
        .expect("75-field MOI1 hash");
    assert_eq!(message[0], Operand::Immediate(Fr::from(0x31494f4du64)));
    assert_eq!(message[1], Operand::Immediate(Fr::from(1u64)));
    assert_eq!(message[6], Operand::Immediate(Fr::from(28u64)));
    for i in 0..28 {
        let tag = if c.public_inputs[i].ty == mori_core::Type::Bool {
            1u64
        } else {
            2
        };
        assert_eq!(
            message[7 + 2 * i],
            Operand::Immediate(Fr::from(tag)),
            "tag {i}"
        );
        assert_eq!(
            message[8 + 2 * i],
            publics[i],
            "direct financial operand {i}"
        );
    }
    assert_eq!(message[63], Operand::Immediate(Fr::from(1u64)));
    // payload limbs, caps and metadata are the public authorization identifiers.
    assert_eq!(message[4..6], publics[28..30]);
    assert_eq!(message[66..70], publics[39..43]);
    assert_eq!(
        message[70..75],
        [&publics[33..36], &publics[36..38]].concat()
    );
    let limbs = g["asset_limbs"].as_array().unwrap();
    for (i, limb) in limbs.iter().enumerate() {
        let n: u128 = limb.as_str().unwrap().parse().unwrap();
        assert_eq!(message[64 + i], Operand::Immediate(Fr::from(n)));
    }
    let challenge = hashes
        .iter()
        .find(|h| h.len() == 6)
        .expect("FENS challenge");
    assert_eq!(challenge[0], Operand::Immediate(Fr::from(0x46454e53u64)));
    assert!(
        asserts >= 1,
        "strict FENS equation is an unconditional Assert"
    );
}

#[test]
fn golden_preparation_reconstructs_exact_payload_message_and_projections() {
    let g = golden();
    let prepared = signed_relation::prepare_intent(
        g["source_utf8"].as_str().unwrap(),
        "repay",
        &golden_scenario(),
        &golden_target(),
        &golden_authority(),
        UInt(15),
    )
    .unwrap();
    assert_eq!(prepared.profile, "moriarty-jubjub-prepared-intent/1");
    assert_eq!(
        mori_core::intent::canonical(&prepared.payload).unwrap(),
        g["payload_canonical_utf8"].as_str().unwrap().as_bytes()
    );
    assert_eq!(prepared.payload_hash, g["payload_hash"]);
    assert_eq!(prepared.context_hash, g["context_hash"]);
    assert_eq!(prepared.message_hex, g["message_hex_le32"]);
    let financial = g["financial_bindings"].as_array().unwrap();
    assert_eq!(prepared.financial_projection.len(), 28);
    for (v, want) in prepared.financial_projection.iter().zip(financial) {
        let have = serde_json::to_value(&v.binding).unwrap();
        assert_eq!(json!(v.position), want["position"]);
        assert_eq!(have, without(&without(want, "value"), "position"));
        let raw = match &v.value {
            mori_core::Value::UInt(n) => n.0.to_string(),
            mori_core::Value::Bool(b) => u8::from(*b).to_string(),
        };
        assert_eq!(
            raw,
            want["value"].as_str().unwrap(),
            "position {}",
            v.position
        );
    }
    let auth = g["authorization_bindings"].as_array().unwrap();
    assert_eq!(prepared.authorization_projection.len(), 15);
    for (v, want) in prepared.authorization_projection.iter().zip(auth) {
        assert_eq!(json!(v.position), want["position"]);
        assert_eq!(v.value, decimal_hex(want["value"].as_str().unwrap()));
    }
    let program = mori_core::parse(g["source_utf8"].as_str().unwrap()).unwrap();
    assert_eq!(
        prepared.review,
        mori_core::simulate(&program, "repay", &golden_scenario()).unwrap()
    );
    prepared.validate().unwrap();
}

#[test]
fn golden_attachments_verify_under_reconstruction_and_interval() {
    let g = golden();
    let source = g["source_utf8"].as_str().unwrap();
    let run = |signed: &SignedIntent, at: u128| {
        signed_relation::verify_intent(
            source,
            "repay",
            &golden_scenario(),
            &golden_target(),
            &golden_authority(),
            UInt(at),
            signed,
        )
    };
    let execution = run(&golden_signed(), 15).unwrap();
    assert_eq!(execution.action, "repay");
    for at in [10, 20] {
        run(&golden_signed(), at).unwrap();
    }
    for at in [9, 21] {
        assert_eq!(
            run(&golden_signed(), at).unwrap_err().code,
            "mori::backend::authorization"
        );
    }
    // Independent alternative attachment (public QA nonce 23) for the same payload.
    let mut alternative = golden_signed();
    alternative.signature_hex = "fec0b07c971073c462c88d722fb5fe8df764311114dbcc79e46263b74a863850e92947318a0258f88a6869c98a7e96733662780d4936401c0e732a4c87fb651a98b9a4ecb5630fefa96bd5f061388d910c95dabfa7f2c089c2d3651ca422e900".into();
    run(&alternative, 15).unwrap();
    let mut flipped = golden_signed();
    let mut bytes = flipped.signature_hex.into_bytes();
    bytes[130] = if bytes[130] == b'0' { b'1' } else { b'0' };
    flipped.signature_hex = String::from_utf8(bytes).unwrap();
    assert_eq!(
        run(&flipped, 15).unwrap_err().code,
        "mori::backend::signature"
    );
}

#[test]
fn trusted_authority_target_and_payload_substitutions_refuse() {
    let g = golden();
    let source = g["source_utf8"].as_str().unwrap();
    let check =
        |target: &Target, authority: &ExpectedAuthority, signed: &SignedIntent, src: &str| {
            signed_relation::verify_intent(
                src,
                "repay",
                &golden_scenario(),
                target,
                authority,
                UInt(15),
                signed,
            )
            .unwrap_err()
            .code
        };
    let (t, a, s) = (golden_target(), golden_authority(), golden_signed());
    for (field, value) in [
        ("network", json!("other")),
        ("anchor", json!("other")),
        ("program_address", json!("other")),
        ("owner", json!("bob")),
        ("beneficiary", json!("alice")),
        ("nonce", json!("2")),
        ("predecessor", json!("local/head/1")),
        ("not_before", json!("11")),
        ("target_hash", json!("00".repeat(32))),
        (
            "public_key_hex",
            json!(
                "0677506737143823730512c5525e0b30acb75906fa7d53f494ba689b78620133d4cdab997110c2f1e02bb16ebef816c9d0a6025386825581a688b9bfa326360a"
            ),
        ),
    ] {
        let mut au = serde_json::to_value(&a).unwrap();
        au[field] = value;
        if field == "beneficiary" {
            au["limits"][0]["beneficiary"] = "alice".into();
        }
        let au = ExpectedAuthority::from_json(&au.to_string()).unwrap();
        assert_eq!(
            check(&t, &au, &s, source),
            "mori::backend::authorization",
            "{field}"
        );
    }
    let mut au = serde_json::to_value(&a).unwrap();
    au["limits"][0]["gross_cap"] = "10".into();
    let au = ExpectedAuthority::from_json(&au.to_string()).unwrap();
    assert_eq!(check(&t, &au, &s, source), "mori::backend::authorization");
    // Raw source bytes are part of the signed identity even if the Program is equal.
    assert_eq!(
        check(&t, &a, &s, &format!("{source}\n")),
        "mori::backend::authorization"
    );
    let mut asset = serde_json::to_value(&a).unwrap();
    asset["limits"][0]["asset"]["scale"] = 1.into();
    let asset = ExpectedAuthority::from_json(&asset.to_string()).unwrap();
    assert_eq!(
        check(&t, &asset, &s, source),
        "mori::backend::authorization"
    );
    let unsigned = Target::ledger9("local", "native-liability-qa");
    assert_eq!(check(&unsigned, &a, &s, source), "mori::backend::target");
}

#[test]
fn signed_admission_refuses_second_payer_debtor_aliases_and_seventeen_before_allocation() {
    let g = golden();
    let base = g["source_utf8"].as_str().unwrap();
    let target = golden_target();
    let code = |src: &str| {
        mori_backend::compile_signed(&mori_core::parse(src).unwrap(), &target)
            .unwrap_err()
            .code
    };
    let unexecuted = |body: &str| {
        base.replace(
            "action observe() {",
            &format!("action other() {{ {body} }}\n  action observe() {{"),
        )
    };
    assert_eq!(
        code(&unexecuted("transfer(QA, \"carol\", \"bob\", 1);")),
        "mori::backend::signed"
    );
    assert_eq!(
        code(&unexecuted("fee(QA, \"bob\", \"treasury\", 1);")),
        "mori::backend::signed"
    );
    assert_eq!(
        code(&unexecuted(
            "liability(\"loan\", QA, \"carol\", \"alice\", 1);"
        )),
        "mori::backend::signed"
    );
    // Core already refuses identity aliases in source and in a direct Program.
    let alias = base.replace(
        "  state settled",
        "  asset QB = { network: \"local\", issuer: \"native-liability-qa\", reference: \"QA\", scale: 0 };\n  state settled",
    );
    assert!(mori_core::parse(&alias).is_err());
    let mut program = mori_core::parse(base).unwrap();
    let mut copy = program.assets[0].clone();
    copy.name = "QB".into();
    program.assets.push(copy);
    assert_eq!(
        mori_backend::compile_signed(&program, &target)
            .unwrap_err()
            .code,
        "mori::backend::program"
    );
    let assets: String = (0..17)
        .map(|i| format!("  asset A{i} = {{ network: \"local\", issuer: \"i\", reference: \"r{i:02}\", scale: 0 }};\n"))
        .collect();
    let many = base.replace("  state settled", &format!("{assets}  state settled"));
    assert_eq!(code(&many), "mori::backend::limit");
    let duties: String = (0..17)
        .map(|i| format!("liability(\"d{i:02}\", QA, \"alice\", \"bob\", 0);"))
        .collect();
    assert_eq!(code(&unexecuted(&duties)), "mori::backend::limit");
    // Exactly sixteen assets and sixteen owner duties remain admissible.
    let sixteen: String = (0..15)
        .map(|i| format!("  asset A{i} = {{ network: \"local\", issuer: \"i\", reference: \"r{i:02}\", scale: 0 }};\n"))
        .collect();
    let full_duties: String = (0..14)
        .map(|i| format!("liability(\"d{i:02}\", QA, \"alice\", \"bob\", 0);"))
        .collect();
    let full = base
        .replace("  state settled", &format!("{sixteen}  state settled"))
        .replace(
            "action observe() {",
            &format!("action other() {{ {full_duties} }}\n  action observe() {{"),
        );
    let artifact =
        mori_backend::compile_signed(&mori_core::parse(&full).unwrap(), &target).unwrap();
    assert_eq!(artifact.resources.liability_registry_entries, 16);
    let observe = artifact
        .circuits
        .iter()
        .find(|c| c.action == "observe")
        .unwrap();
    assert_eq!(observe.authorization_bindings.len(), 11 + 4 * 16);
    assert_eq!(observe.liability_frame.len(), 16);
    // Zero assets and zero duties: pure state action still requires authorization.
    let pure = "language \"moriarty/1\"; agreement Pure { state n: UInt = 0; action bump() { next n = n + 1; } }";
    let mut t = target.clone();
    t.signed_policy.as_mut().unwrap().program_address = "local/Pure".into();
    let pure = mori_backend::compile_signed(&mori_core::parse(pure).unwrap(), &t).unwrap();
    assert_eq!(pure.circuits[0].authorization_bindings.len(), 11);
    assert_eq!(pure.circuits[0].private_inputs.len(), 3);
}

#[test]
fn unsigned_and_signed_entry_points_refuse_the_other_target_mode() {
    let g = golden();
    let source = g["source_utf8"].as_str().unwrap();
    let program = mori_core::parse(source).unwrap();
    assert_eq!(
        mori_backend::compile(&program, &golden_target())
            .unwrap_err()
            .code,
        "mori::backend::target"
    );
    assert_eq!(
        mori_backend::compile_signed(&program, &Target::ledger9("local", "a"))
            .unwrap_err()
            .code,
        "mori::backend::target"
    );
    assert_eq!(
        proving::prove(
            source,
            "repay",
            &golden_scenario(),
            &golden_target(),
            &params()
        )
        .unwrap_err()
        .code,
        "mori::backend::target"
    );
}

#[test]
fn honest_signed_prove_reaches_missing_parameters_and_bad_inputs_refuse_earlier() {
    let g = golden();
    let source = g["source_utf8"].as_str().unwrap();
    let prove = |signed: &SignedIntent, authority: &ExpectedAuthority, at: u128| {
        proving::prove_signed(
            source,
            "repay",
            &golden_scenario(),
            &golden_target(),
            authority,
            UInt(at),
            signed,
            &params(),
        )
        .unwrap_err()
        .code
    };
    assert_eq!(
        prove(&golden_signed(), &golden_authority(), 15),
        "mori::backend::parameters"
    );
    let mut flipped = golden_signed();
    flipped.signature_hex.replace_range(130..132, "00");
    assert_eq!(
        prove(&flipped, &golden_authority(), 15),
        "mori::backend::signature"
    );
    assert_eq!(
        prove(&golden_signed(), &golden_authority(), 21),
        "mori::backend::authorization"
    );
    let mut payload = serde_json::to_value(golden_signed()).unwrap();
    payload["payload"]["nonce"] = "2".into();
    let changed = SignedIntent::from_json(&payload.to_string()).unwrap();
    assert_eq!(
        prove(&changed, &golden_authority(), 15),
        "mori::backend::authorization"
    );
    let mut opt = params();
    opt.max_k = 0;
    assert_eq!(
        proving::prove_signed(
            source,
            "repay",
            &golden_scenario(),
            &golden_target(),
            &golden_authority(),
            UInt(15),
            &golden_signed(),
            &opt,
        )
        .unwrap_err()
        .code,
        "mori::backend::limit"
    );
}

fn payment() -> (&'static str, Scenario, Target, ExpectedAuthority, Value) {
    (
        include_str!("../../../examples/core/native-owner-intent/payment.mori"),
        serde_json::from_str(include_str!(
            "../../../examples/core/native-owner-intent/payment.scenario.json"
        ))
        .unwrap(),
        Target::from_json(include_str!(
            "../../../examples/core/native-owner-intent/payment.target.json"
        ))
        .unwrap(),
        ExpectedAuthority::from_json(include_str!(
            "../../../examples/core/native-owner-intent/payment.authority.json"
        ))
        .unwrap(),
        serde_json::from_str(include_str!(
            "../../../examples/core/native-owner-intent/payment.expected.json"
        ))
        .unwrap(),
    )
}
fn with_caps(authority: &ExpectedAuthority, caps: &Value) -> ExpectedAuthority {
    let mut a = serde_json::to_value(authority).unwrap();
    for k in ["gross_cap", "fee_cap", "net_floor", "liability_cap"] {
        a["limits"][0][k] = caps[k].clone();
    }
    ExpectedAuthority::from_json(&a.to_string()).unwrap()
}

#[test]
fn independent_payment_expectations_accept_exact_caps_and_each_tighter_cap_refuses() {
    let (source, scenario, target, authority, expected) = payment();
    let exact = with_caps(&authority, &expected["caps_accept"]);
    let prepared =
        signed_relation::prepare_intent(source, "settle", &scenario, &target, &exact, UInt(15))
            .unwrap();
    let e = &prepared.review;
    for (account, amount) in expected["post_balances"].as_object().unwrap() {
        let b = e
            .post
            .balances
            .iter()
            .find(|b| &b.account == account)
            .unwrap();
        assert_eq!(
            b.amount.0.to_string(),
            amount.as_str().unwrap(),
            "{account}"
        );
    }
    for (name, amount) in expected["post_liabilities"].as_object().unwrap() {
        let l = e.post.liabilities.iter().find(|l| &l.name == name).unwrap();
        assert_eq!(l.amount.0.to_string(), amount.as_str().unwrap(), "{name}");
    }
    let kinds: Vec<_> = e
        .effects
        .iter()
        .map(|f| serde_json::to_value(f).unwrap()["kind"].clone())
        .collect();
    assert_eq!(Value::Array(kinds), expected["effect_kinds"]);
    for (state, value) in expected["post_state"].as_object().unwrap() {
        assert_eq!(
            serde_json::to_value(&e.post.state[state]).unwrap(),
            json!({"type":"UInt","value":value})
        );
    }
    // Complete ordered effects from the independent brief:
    // Transfer70 / Fee2 / Liability50->30 / State0->70 / Emit70.
    let asset = json!({"network":"local","issuer":"signed-native-qa","reference":"QA","scale":0});
    assert_eq!(
        serde_json::to_value(&e.effects).unwrap(),
        json!([
            {"kind":"Transfer","asset":asset,"from":"alice","to":"bob","amount":"70"},
            {"kind":"Fee","asset":asset,"from":"alice","to":"treasury","amount":"2"},
            {"kind":"Liability","name":"payment","asset":asset,"debtor":"alice","creditor":"bob","before":"50","after":"30"},
            {"kind":"State","name":"paid","before":{"type":"UInt","value":"0"},"after":{"type":"UInt","value":"70"}},
            {"kind":"Emit","name":"paid","value":{"type":"UInt","value":"70"}}
        ])
    );
    // A freshly QA-signed exact statement passes host acceptance.
    let message = schnorr::decode_native_hex(&prepared.message_hex).unwrap();
    let signed = SignedIntent {
        payload: prepared.payload.clone(),
        signature_hex: qa_sign(7, 29, message),
    };
    signed_relation::verify_intent(
        source,
        "settle",
        &scenario,
        &target,
        &exact,
        UInt(15),
        &signed,
    )
    .unwrap();
    for (caps, word) in expected["individual_cap_refusals"]
        .as_array()
        .unwrap()
        .iter()
        .zip(["gross", "fee", "net", "liability"])
    {
        let tight = with_caps(&authority, caps);
        let err =
            signed_relation::prepare_intent(source, "settle", &scenario, &target, &tight, UInt(15))
                .unwrap_err();
        assert_eq!(err.code, "mori::backend::authorization");
        assert!(err.message.contains(word), "{word}: {err}");
        // Verification refuses the cap from reconstruction before the attachment;
        // freshly re-signed circuit refusals are ordinary-checker unit tests.
        let mut payload = serde_json::to_value(&prepared.payload).unwrap();
        for k in ["gross_cap", "fee_cap", "net_floor", "liability_cap"] {
            payload["limits"][0][k] = caps[k].clone();
        }
        let signed = SignedIntent {
            payload: serde_json::from_value(payload).unwrap(),
            signature_hex: qa_sign(7, 31, message),
        };
        let err = signed_relation::verify_intent(
            source,
            "settle",
            &scenario,
            &target,
            &tight,
            UInt(15),
            &signed,
        )
        .unwrap_err();
        assert!(err.message.contains(word), "{word}: {err}");
    }
}

const ADMISSION: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/programs/signed-admission/"
);

#[test]
fn independent_source_label_cases_refuse_before_lowering() {
    use sha2::{Digest, Sha256};
    let manifest: Value =
        serde_json::from_slice(&std::fs::read(format!("{ADMISSION}manifest.json")).unwrap())
            .unwrap();
    let (_, _, target, _, _) = payment();
    let cases = manifest["source_cases"].as_array().unwrap();
    assert_eq!(cases.len(), 12);
    for case in cases {
        let file = case["file"].as_str().unwrap();
        let bytes = std::fs::read(format!("{ADMISSION}{file}")).unwrap();
        assert_eq!(
            mori_core::intent::hex(&Sha256::digest(&bytes)),
            case["source_sha256"]
        );
        let program = mori_core::parse(std::str::from_utf8(&bytes).unwrap())
            .unwrap_or_else(|e| panic!("{file}: Core parse {e}"));
        let result = mori_backend::compile_signed(&program, &target);
        match case["expected"].as_str().unwrap() {
            "signed-label-refusal" => {
                let err = result.unwrap_err();
                assert_eq!(err.code, "mori::backend::signed", "{file}: {err}");
                assert!(err.message.contains("label"), "{file}: {err}");
            }
            "signed-label-admission" => {
                let a = result.unwrap_or_else(|e| panic!("{file}: {e}"));
                assert!(
                    a.circuits
                        .iter()
                        .any(|c| c.action == case["selected_action"])
                );
            }
            "unknown-duty-refusal" => {
                let err = result.unwrap_err();
                assert_eq!(err.code, "mori::backend::unsupported", "{file}: {err}");
                assert!(
                    err.message.contains("no source-derived liability"),
                    "{file}: {err}"
                );
            }
            other => panic!("{other}"),
        }
        // Unsigned behavior stays separate: no signed label rule applies, but a
        // discharge-only file still names an unregistered duty.
        let unsigned = mori_backend::compile(&program, &Target::ledger9("local", "anchor"));
        if file.starts_with("discharge-only") {
            let err = unsigned.unwrap_err();
            assert_eq!(err.code, "mori::backend::unsupported", "{file}: {err}");
        } else {
            unsigned.unwrap_or_else(|e| panic!("{file}: unsigned {e}"));
        }
    }
}

fn golden_artifact_json() -> Value {
    let program = mori_core::parse(golden()["source_utf8"].as_str().unwrap()).unwrap();
    serde_json::to_value(mori_backend::compile_signed(&program, &golden_target()).unwrap()).unwrap()
}
fn decodes(v: &Value) -> bool {
    serde_json::from_value::<mori_backend::Artifact>(v.clone()).is_ok()
}

#[test]
fn required_null_descriptor_members_refuse_when_deleted() {
    let manifest: Value =
        serde_json::from_slice(&std::fs::read(format!("{ADMISSION}manifest.json")).unwrap())
            .unwrap();
    let artifact = golden_artifact_json();
    assert!(decodes(&artifact));
    let repay = artifact["circuits"]
        .as_array()
        .unwrap()
        .iter()
        .position(|c| c["action"] == "repay")
        .unwrap();
    for case in manifest["descriptor_cases"].as_array().unwrap() {
        let file = case["file"].as_str().unwrap();
        let text = std::fs::read_to_string(format!("{ADMISSION}{file}")).unwrap();
        let mut deleted: Value = serde_json::from_str(&text).unwrap();
        deleted.as_object_mut().unwrap().remove("position");
        assert!(
            serde_json::from_value::<mori_backend::PublicBinding>(deleted).is_err(),
            "{file}"
        );
        let position = case["position"].as_u64().unwrap() as usize;
        let mut baseline = case["baseline"].clone();
        baseline.as_object_mut().unwrap().remove("position");
        assert_eq!(
            artifact["circuits"][repay]["public_inputs"][position], baseline,
            "{file}"
        );
        let mut bad = artifact.clone();
        bad["circuits"][repay]["public_inputs"][position]
            .as_object_mut()
            .unwrap()
            .remove(case["removed_member"].as_str().unwrap());
        assert!(!decodes(&bad), "{file} embedded in a full v3 artifact");
    }
    // Static frame Option members are explicit null and required.
    for member in ["name", "asset", "from", "to", "liability", "operation"] {
        let mut bad = artifact.clone();
        bad["circuits"][repay]["effect_frame"][3]
            .as_object_mut()
            .unwrap()
            .remove(member);
        assert!(!decodes(&bad), "effect member {member}");
    }
}

#[test]
fn whole_artifact_mode_target_and_descriptor_shape_must_agree() {
    let signed = golden_artifact_json();
    let program = mori_core::parse(golden()["source_utf8"].as_str().unwrap()).unwrap();
    let unsigned = serde_json::to_value(
        mori_backend::compile(&program, &Target::ledger9("local", "native-liability-qa")).unwrap(),
    )
    .unwrap();
    assert!(decodes(&signed) && decodes(&unsigned));
    let mutate = |base: &Value, f: &dyn Fn(&mut Value)| {
        let mut v = base.clone();
        f(&mut v);
        decodes(&v)
    };
    assert!(!mutate(&signed, &|v| v["authorization_mode"] = "unsigned".into()));
    assert!(!mutate(&signed, &|v| {
        v["target"].as_object_mut().unwrap().remove("signed_policy");
    }));
    assert!(!mutate(
        &signed,
        &|v| v["circuits"][0]["authorization_bindings"] = json!([])
    ));
    assert!(!mutate(&signed, &|v| v["circuits"][0]["private_inputs"] = json!([])));
    assert!(!mutate(&signed, &|v| {
        v["circuits"][0]["private_inputs"]
            .as_array_mut()
            .unwrap()
            .swap(0, 2);
    }));
    assert!(!mutate(&signed, &|v| {
        v["circuits"][0]["authorization_bindings"]
            .as_array_mut()
            .unwrap()
            .swap(0, 1);
    }));
    assert!(!mutate(
        &signed,
        &|v| v["circuits"][0]["authorization_bindings"][4]["ty"] = "UInt".into()
    ));
    assert!(!mutate(
        &signed,
        &|v| v["circuits"][0]["authorization_bindings"][0]["position"] = 0.into()
    ));
    assert!(!mutate(&signed, &|v| v["circuits"][0]["context_hash"] = "x".into()));
    assert!(!mutate(&unsigned, &|v| v["authorization_mode"] =
        "moriarty-jubjub-owner-intent/1".into()));
    assert!(!mutate(&unsigned, &|v| {
        v["circuits"][0]["private_inputs"] = signed["circuits"][0]["private_inputs"].clone()
    }));
}

#[test]
fn preparation_preflights_exported_report_and_signed_wrapper_budget() {
    let (source, _, target, authority, _) = payment();
    // Escaped Unicode untouched balances: compact report bytes fit, while the
    // bounded public exporter refuses. Preparation must refuse, not truncate.
    let asset = json!({"network":"local","issuer":"signed-native-qa","reference":"QA","scale":0});
    let mut scenario: Value = serde_json::from_str(include_str!(
        "../../../examples/core/native-owner-intent/payment.scenario.json"
    ))
    .unwrap();
    scenario["inputs"] = json!({});
    for i in 0..120 {
        scenario["balances"].as_array_mut().unwrap().push(
            json!({"account":format!("{i:03}{}", "Ж".repeat(500)),"asset":asset,"amount":"1"}),
        );
    }
    let scenario: Scenario = serde_json::from_value(scenario).unwrap();
    let mut caps = serde_json::to_value(&authority).unwrap();
    caps["limits"][0]["net_floor"] = "0".into();
    caps["limits"][0]["liability_cap"] = "55".into();
    let observe = ExpectedAuthority::from_json(&caps.to_string()).unwrap();
    let err =
        signed_relation::prepare_intent(source, "observe", &scenario, &target, &observe, UInt(15))
            .unwrap_err();
    assert_eq!(err.code, "mori::backend::limit", "{err}");

    // Signed wrapper: a 192-hex attachment adds exactly 223 canonical bytes, so
    // the largest signable payload is 65313 bytes. Long legal labels reach it.
    let wide = |predecessor: usize, issuer: usize| {
        let network = "n".repeat(1000);
        let beneficiary = "b".repeat(1000);
        let reference = |i: usize| format!("r{i:02}{}", "r".repeat(997));
        let assets: String = (0..16)
            .map(|i| format!("  asset A{i:02} = {{ network: \"{network}\", issuer: \"{}\", reference: \"{}\", scale: 0 }};\n", "i".repeat(issuer), reference(i)))
            .collect();
        let src = format!(
            "language \"moriarty/1\";\nagreement Wide {{\n{assets}  state n: UInt = 0;\n  action bump() {{ next n = n + 1; }}\n}}\n"
        );
        let mut t = Target::ledger9(network.clone(), "local/signed-native-qa");
        t.signed_policy = target.signed_policy.clone();
        let policy = t.signed_policy.as_mut().unwrap();
        policy.program_address = "local/Wide".into();
        policy.beneficiary = beneficiary.clone();
        let mut a = serde_json::to_value(&authority).unwrap();
        a["network"] = network.clone().into();
        a["program_address"] = "local/Wide".into();
        a["beneficiary"] = beneficiary.clone().into();
        a["target_hash"] = mori_core::intent::hex(&{
            use sha2::{Digest, Sha256};
            Sha256::digest(mori_core::intent::canonical(&t).unwrap())
        })
        .into();
        a["predecessor"] = "p".repeat(predecessor).into();
        a["limits"] = (0..16)
            .map(|i| json!({"asset":{"network":network,"issuer":"i".repeat(issuer),"reference":reference(i),"scale":0},
                "beneficiary":beneficiary,"gross_cap":"0","fee_cap":"0","net_floor":"0","liability_cap":"0"}))
            .collect();
        let a = ExpectedAuthority::from_json(&a.to_string()).unwrap();
        signed_relation::prepare_intent(&src, "bump", &Scenario::default(), &t, &a, UInt(15))
    };
    let size = |p: &mori_backend::authorization::PreparedIntent| {
        mori_core::intent::canonical(&p.payload).unwrap().len()
    };
    // Each issuer byte appears once per limit (16 bytes of payload).
    let probe = size(&wide(1, 500).unwrap());
    let issuer = 500 + (65313 - 600 - probe) / 16;
    let base_len = size(&wide(1, issuer).unwrap());
    assert!(
        base_len <= 65313 - 584 && base_len + 1023 > 65314,
        "{base_len}"
    );
    let fit = 1 + 65313 - base_len;
    let largest = wide(fit, issuer).unwrap();
    assert_eq!(size(&largest), 65313);
    let wrapper = json!({"payload":largest.payload,"signature_hex":"0".repeat(192)});
    assert_eq!(mori_core::intent::canonical(&wrapper).unwrap().len(), 65536);
    let err = wide(fit + 1, issuer).unwrap_err();
    assert_eq!(err.code, "mori::backend::authorization", "{err}");
    assert!(err.message.contains("wrapper"), "{err}");
}

mod shapes {
    use mori_backend::{Target, authorization::ExpectedAuthority};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/support/signed_shapes.rs"
    ));
}

// Actual integrated shapes; load/model only. No parameter access or native work.
// All generated signatures use the fixed PUBLIC QA scalar7/nonce29 helper above.
#[test]
fn exact_signed_integrated_shape_measurements() {
    use midnight_transient_crypto::proofs::Zkir;
    use sha2::{Digest, Sha256};
    use std::path::PathBuf;
    let digest = |bytes: &[u8]| mori_core::intent::hex(&Sha256::digest(bytes));
    let canonical = |v: &Value| mori_core::intent::canonical(v).unwrap();
    let output = std::env::var_os("MORIARTY_SIGNED_MEASUREMENT_DIR").map(PathBuf::from);
    if let Some(dir) = &output {
        std::fs::create_dir_all(dir).unwrap();
    }
    let mut cases = Vec::new();
    let g = golden();
    cases.push((
        "golden-repayment".to_owned(),
        g["source_utf8"].as_str().unwrap().to_owned(),
        "repay".to_owned(),
        golden_scenario(),
        golden_target(),
        golden_authority(),
        Some(golden_signed()),
    ));
    let (source, scenario, target, authority, _) = payment();
    cases.push((
        "independent-payment".to_owned(),
        source.to_owned(),
        "settle".to_owned(),
        scenario.clone(),
        target.clone(),
        authority.clone(),
        None,
    ));
    let mut observe: Value = serde_json::to_value(&authority).unwrap();
    observe["limits"][0]["net_floor"] = "0".into();
    observe["limits"][0]["liability_cap"] = "55".into();
    let mut observation = scenario;
    observation.inputs.clear();
    cases.push((
        "signed-observation".to_owned(),
        source.to_owned(),
        "observe".to_owned(),
        observation,
        target.clone(),
        ExpectedAuthority::from_json(&observe.to_string()).unwrap(),
        None,
    ));
    for (label, assets, duties, action) in [
        ("zero-registry", 0, 0, "settle"),
        ("full-sixteen-registry", 1, 16, "settle"),
        ("max-sixteen-assets-duties", 16, 16, "settle"),
        ("max-sixteen-observation", 16, 16, "observe"),
    ] {
        let (source, scenario, t, a) = shapes::fixture(assets, duties, action, &target, &authority);
        cases.push((
            label.to_owned(),
            source,
            action.to_owned(),
            scenario,
            t,
            a,
            None,
        ));
    }
    let mut records = Vec::new();
    for (label, source, action, scenario, target, authority, fixed) in cases {
        let program = mori_core::parse(&source).unwrap();
        let artifact = mori_backend::compile_signed(&program, &target).unwrap();
        let c = artifact
            .circuits
            .iter()
            .find(|c| c.action == action)
            .unwrap();
        let preparation = signed_relation::prepare_intent(
            &source,
            &action,
            &scenario,
            &target,
            &authority,
            UInt(15),
        );
        let (prepared, refusal) = match preparation {
            Ok(p) => (Some(p), None),
            Err(e) => {
                assert_eq!(label, "max-sixteen-assets-duties");
                assert_eq!(e.code, "mori::backend::limit");
                assert!(e.message.contains("JSON exceeds 1 MiB"));
                (None, Some(e.message))
            }
        };
        let signed = prepared.as_ref().map(|p| {
            fixed.unwrap_or_else(|| SignedIntent {
                payload: p.payload.clone(),
                signature_hex: qa_sign(7, 29, schnorr::decode_native_hex(&p.message_hex).unwrap()),
            })
        });
        if let Some(signed) = &signed {
            signed_relation::verify_intent(
                &source,
                &action,
                &scenario,
                &target,
                &authority,
                UInt(15),
                signed,
            )
            .unwrap();
        }
        let ir = IrSource::load(c.zkir.as_bytes()).unwrap();
        let model = ir.model();
        let financial = c.public_inputs.len();
        let assets = program.assets.len();
        let public = financial + c.authorization_bindings.len();
        assert_eq!(c.authorization_bindings.len(), 11 + 4 * assets);
        assert_eq!(c.private_inputs.len(), 3);
        if let Some(p) = &prepared {
            assert_eq!(p.financial_projection.len(), financial);
            assert_eq!(p.authorization_projection.len(), 11 + 4 * assets);
        }
        let public_reads = ir
            .instructions
            .iter()
            .filter(|i| matches!(i, Instruction::PublicInput { .. }))
            .count();
        let impacts = ir
            .instructions
            .iter()
            .filter(|i| matches!(i, Instruction::Impact { .. }))
            .count();
        assert_eq!(public_reads, public);
        assert_eq!(impacts, public);
        let hash_lengths: Vec<_> = ir
            .instructions
            .iter()
            .filter_map(|i| {
                if let Instruction::TransientHash { inputs, .. } = i {
                    Some(inputs.len())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(hash_lengths, [13 + 2 * financial + 6 * assets, 6]);
        if label == "independent-payment" {
            assert_eq!(prepared.as_ref().unwrap().review.post.liabilities.len(), 3);
            assert_eq!(prepared.as_ref().unwrap().review.effects.len(), 5);
        }
        let artifact_json = canonical(&serde_json::to_value(&artifact).unwrap());
        let scenario_json = canonical(&serde_json::to_value(&scenario).unwrap());
        let target_json = canonical(&serde_json::to_value(&target).unwrap());
        let authority_json = canonical(&serde_json::to_value(&authority).unwrap());
        let prepared_json = canonical(&serde_json::to_value(&prepared).unwrap());
        let payload_json =
            canonical(&serde_json::to_value(signed.as_ref().map(|s| &s.payload)).unwrap());
        let wrapper_json = canonical(&serde_json::to_value(&signed).unwrap());
        let layout_json = canonical(
            &json!({"financial":c.public_inputs,"authorization":c.authorization_bindings,"private":c.private_inputs,"effects":c.effect_frame,"balances":c.balance_frame,"duties":c.liability_frame,"replay_steps":c.replay_steps}),
        );
        let record = json!({"fixture":label,"action":action,"at":"15","qualification":"ordinary-host-and-IR-model-only",
            "source_sha256":digest(source.as_bytes()),"program_sha256":artifact.program_hash,"scenario_sha256":digest(&scenario_json),"target_sha256":digest(&target_json),"authority_sha256":digest(&authority_json),
            "artifact_sha256":digest(&artifact_json),"artifact_bytes":artifact_json.len(),"layout_sha256":digest(&layout_json),"zkir_sha256":digest(c.zkir.as_bytes()),"zkir_bytes":c.zkir.len(),
            "payload_sha256":signed.as_ref().map(|_| digest(&payload_json)),"prepared_sha256":prepared.as_ref().map(|_| digest(&prepared_json)),"signed_wrapper_sha256":signed.as_ref().map(|_| digest(&wrapper_json)),"signature_hex":signed.as_ref().map(|s| &s.signature_hex),
            "context_sha256":c.context_hash,"message_hex":prepared.as_ref().map(|p| &p.message_hex),"preparation_refusal":refusal,"public_key_hex":authority.public_key_hex,
            "asset_count":assets,"duty_registry_count":artifact.resources.liability_registry_entries,"duty_frame_count":c.liability_frame.len(),"balance_frame_count":c.balance_frame.len(),"effect_count":c.effect_frame.len(),
            "financial_public_values":financial,"authorization_public_values":c.authorization_bindings.len(),"public_values":public,"public_field_elements":public,"financial_uint_values":c.public_inputs.iter().filter(|b| b.ty == mori_core::Type::UInt).count(),"financial_bool_values":c.public_inputs.iter().filter(|b| b.ty == mori_core::Type::Bool).count(),"private_typed_inputs":3,"private_flattened_fields":5,"message_fields":hash_lengths[0],"fens_challenge_fields":6,
            "instructions":c.instruction_count,"public_input_instructions":public_reads,"impact_instructions":impacts,"replay_steps":c.replay_steps,
            "optimal_k":ir.k(),"model_k":model.k(),"model_rows":model.rows(),"supported_native_degree":ir.k() <= proving::MAX_K,"native_degree_ceiling":proving::MAX_K});
        println!("SIGNED_MEASUREMENT {record}");
        if let Some(dir) = &output {
            for (ext, bytes) in [
                ("mori", source.as_bytes()),
                ("scenario.json", scenario_json.as_slice()),
                ("target.json", target_json.as_slice()),
                ("authority.json", authority_json.as_slice()),
                ("artifact.json", artifact_json.as_slice()),
                ("layout.json", layout_json.as_slice()),
                ("prepared.json", prepared_json.as_slice()),
                ("payload.json", payload_json.as_slice()),
                ("signed.json", wrapper_json.as_slice()),
                ("zkir", c.zkir.as_bytes()),
            ] {
                if prepared.is_none()
                    && ["prepared.json", "payload.json", "signed.json"].contains(&ext)
                {
                    continue;
                }
                std::fs::write(dir.join(format!("{label}.{ext}")), bytes).unwrap();
            }
        }
        // Engineering support is an objective degree bound. Unsupported full
        // shapes are retained and explicitly refused; never alter their slots.
        if ir.k() > proving::MAX_K {
            let err = proving::prove_signed(
                &source,
                &action,
                &scenario,
                &target,
                &authority,
                UInt(15),
                signed.as_ref().unwrap(),
                &params(),
            )
            .unwrap_err();
            assert_eq!(err.code, "mori::backend::limit");
        }
        records.push(record);
        if label == "zero-registry" {
            let basis = strict_signature_basis(&ir, financial);
            let mut disk = serde_json::to_value(&basis).unwrap();
            disk["version"] = json!({"major":3,"minor":0});
            let bytes = serde_json::to_vec(&disk).unwrap();
            let basis = IrSource::load(bytes.as_slice()).unwrap();
            let basis_model = basis.model();
            let record = json!({"fixture":"strict-signature-basis","qualification":"isolated-crypto-IR-model-only-excludes-MOI1-and-financial-caps","instructions":basis.instructions.len(),"public_values":3,"private_typed_inputs":3,"private_flattened_fields":5,"optimal_k":basis.k(),"model_k":basis_model.k(),"model_rows":basis_model.rows(),"zkir_sha256":digest(&bytes)});
            println!("SIGNED_MEASUREMENT {record}");
            if let Some(dir) = &output {
                std::fs::write(dir.join("strict-signature-basis.zkir"), bytes).unwrap();
            }
            records.push(record);
        }
    }
    if let Some(dir) = &output {
        std::fs::write(
            dir.join("measurements.json"),
            serde_json::to_vec_pretty(&records).unwrap(),
        )
        .unwrap();
    }
}

// Extract the strict cryptographic instruction sequence from the actual
// compiler, replacing MOI1 with its separately public digest. No result here
// qualifies the integrated circuit. Input types and strict checks stay intact.
fn strict_signature_basis(ir: &IrSource, financial: usize) -> IrSource {
    let reads: Vec<_> = ir
        .instructions
        .iter()
        .filter(|i| matches!(i, Instruction::PublicInput { .. }))
        .cloned()
        .collect();
    let mut instructions = Vec::new();
    let mut message = None;
    for position in [financial + 2, financial + 3, financial + 4] {
        let i = reads[position].clone();
        let Instruction::PublicInput { output, .. } = &i else {
            unreachable!()
        };
        let value = Operand::Variable(output.clone());
        if position == financial + 4 {
            message = Some(value.clone());
        }
        instructions.push(i);
        instructions.push(Instruction::Impact {
            guard: Operand::Immediate(Fr::from(1)),
            inputs: vec![value],
        });
    }
    let start = ir
        .instructions
        .iter()
        .position(|i| matches!(i, Instruction::PrivateInput { .. }))
        .unwrap();
    let moi = (start..ir.instructions.len())
        .find(|&i| matches!(ir.instructions[i], Instruction::TransientHash { .. }))
        .unwrap();
    let fens = (moi + 1..ir.instructions.len())
        .find(|&i| matches!(ir.instructions[i], Instruction::TransientHash { .. }))
        .unwrap();
    let end = (fens + 1..ir.instructions.len())
        .find(|&i| matches!(ir.instructions[i], Instruction::Assert { .. }))
        .unwrap();
    assert!(matches!(
        ir.instructions[moi + 1],
        Instruction::ConstrainEq { .. }
    ));
    for i in start..=end {
        if i == moi || i == moi + 1 {
            continue;
        }
        let mut instruction = ir.instructions[i].clone();
        if i == fens {
            let Instruction::TransientHash { inputs, .. } = &mut instruction else {
                unreachable!()
            };
            inputs[5] = message.clone().unwrap();
        }
        instructions.push(instruction);
    }
    instructions.push(Instruction::Output { vals: vec![] });
    IrSource {
        version: ir.version.clone(),
        inputs: vec![],
        outputs: vec![],
        do_communications_commitment: false,
        instructions: std::sync::Arc::new(instructions),
    }
}
