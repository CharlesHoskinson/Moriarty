//! Raw ordinary native compiler/VM controls, with public QA scalars only.
//! No genuine derivation/proof capability is constructed or native work run.
#[path = "../../tests/support/mod.rs"]
mod support;
use super::*;
use crate::{prepare, relation};
use midnight_curves::{
    Fr as Scalar, JubjubSubgroup,
    pairing::group::{Group, ff::Field},
};
use midnight_transient_crypto::{curve::Fr, fab::AlignedValueExt, hash::transient_hash};
use mori_core::{Balance, Liability, Scenario};
fn key(scalar: u64) -> String {
    let (x, y) = schnorr::coordinates(JubjubSubgroup::generator() * Scalar::from(scalar));
    format!("{}{}", schnorr::native_hex(x), schnorr::native_hex(y))
}
fn sign(message: Fr, secret: u64, nonce: u64) -> String {
    let k = Scalar::from(nonce);
    let (rx, ry) = schnorr::coordinates(JubjubSubgroup::generator() * k);
    let (px, py) = schnorr::coordinates(JubjubSubgroup::generator() * Scalar::from(secret));
    let h = transient_hash(&[Fr::from(schnorr::FENS_DOMAIN), rx, ry, px, py, message]);
    let mut wide = [0; 64];
    wide[..32].copy_from_slice(&h.as_le_bytes());
    let response = k + Scalar::from_bytes_wide(&wide) * Scalar::from(secret);
    format!(
        "{}{}{}",
        schnorr::native_hex(rx),
        schnorr::native_hex(ry),
        codec::hex(&response.to_bytes())
    )
}
fn policy(beneficiary: &str) -> FixedSignerPolicy {
    FixedSignerPolicy {
        profile: POLICY.into(),
        owner: "alice".into(),
        beneficiary: beneficiary.into(),
        public_key_hex: key(7),
    }
}
fn request(f: &support::Fixture, caps: [u128; 4]) -> SignedCallRequest {
    let q = &f.request;
    SignedCallRequest {
        profile: REQUEST.into(),
        action: q.action.clone(),
        inputs: q.inputs.clone(),
        selected_spends: q.selected_spends.clone(),
        segment: q.segment,
        ttl_hex: q.ttl_hex.clone(),
        nonce: q.nonce,
        block_context_hex: q.block_context_hex.clone(),
        communication_randomness_hex: q.communication_randomness_hex.clone(),
        binding_randomness_hex: q.binding_randomness_hex.clone(),
        not_before: UInt(1),
        not_after: UInt(3),
        at: UInt(2),
        limits: f
            .program
            .assets
            .iter()
            .map(|a| mori_core::intent::AssetLimit {
                asset: a.identity.clone(),
                beneficiary: "bob".into(),
                gross_cap: UInt(caps[0]),
                fee_cap: UInt(caps[1]),
                net_floor: UInt(caps[2]),
                liability_cap: UInt(caps[3]),
            })
            .collect(),
    }
}
fn signed_parameters(mut f: support::Fixture) -> support::Fixture {
    let mut ledger: layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
    let mut params = (*ledger.parameters).clone();
    // Explicit local QA parameter fixture, not the SDK/Preview default policy.
    params.limits.min_time_to_dismiss = midnight_base_crypto::cost_model::CostDuration::SECOND;
    ledger.parameters = midnight_storage::arena::Sp::new(params);
    support::repin(&mut f, &ledger);
    f
}
#[test]
fn nonce_v2_new_signed_install_refuses_legacy_seed() {
    let source = include_str!("../../tests/fixtures/ToggleCounter.mori");
    let f = signed_parameters(support::fixture(source, "step", Scenario::default()));
    let error = admit_signed_local_seed(
        &f.source,
        &f.program,
        &f.deployment,
        &f.predecessor,
        &f.parameters,
        &policy("alice"),
    )
    .err()
    .expect("new signed installation must reject legacy seed");
    assert_eq!(error.code, "legacy-signed-seed");
}
#[test]
fn nonce_v2_descriptor_discriminator_rejects_mixed_fields() {
    let source = include_str!("../../tests/fixtures/ToggleCounter.mori");
    let f = support::fixture(source, "step", Scenario::default());
    let d = SignedInstalledDeployment {
        profile: "moriarty-ledger-installed-signed-local/2".into(),
        qualification: QUALIFICATION.into(),
        phase_policy: "guaranteed-only".into(),
        storage_abi: "moriarty-ledger-signed-public-abi/2".into(),
        signed_abi: "moriarty-ledger-signed-abi/2".into(),
        bindings: StaticBindings::legacy(&f.deployment),
        frame: FrameBindings::legacy(&f.deployment),
        keyset_identity: String::new(),
        signer_policy: policy("alice"),
    };
    SignedInstalledDeployment::from_json(&serde_json::to_string(&d).unwrap()).unwrap();
    for field in ["profile", "storage_abi", "signed_abi"] {
        let mut mixed = serde_json::to_value(&d).unwrap();
        mixed[field] = match field {
            "profile" => PROFILE,
            "storage_abi" => ABI,
            _ => SIGNED_ABI,
        }
        .into();
        assert!(SignedInstalledDeployment::from_json(&mixed.to_string()).is_err());
    }
}
fn metered(running: u128, amount: u128, charge: Option<u128>) -> support::Fixture {
    let source = include_str!("../../tests/fixtures/MeteredRemittance.mori");
    let p = mori_core::parse(source).unwrap();
    let asset = p.assets[0].identity.clone();
    signed_parameters(support::fixture(
        source,
        "settle",
        Scenario {
            inputs: BTreeMap::from([("amount".into(), Value::UInt(UInt(amount)))]),
            state: BTreeMap::from([("running".into(), Value::UInt(UInt(running)))]),
            balances: [("alice", 100), ("bob", 5), ("treasury", 1)]
                .into_iter()
                .map(|(account, amount)| Balance {
                    account: account.into(),
                    asset: asset.clone(),
                    amount: UInt(amount),
                })
                .collect(),
            liabilities: charge
                .into_iter()
                .map(|amount| Liability {
                    name: "charge".into(),
                    asset: asset.clone(),
                    debtor: "alice".into(),
                    creditor: "bob".into(),
                    amount: UInt(amount),
                })
                .collect(),
        },
    ))
}
struct Ordinary {
    s: SignedStaticBindings,
    a: relation::Artifact,
    d: SignedInstalledDeployment,
    q: SignedCallRequest,
    package: CallSigningPackage,
    attachment: SignedCallAttachment,
}
fn ordinary(f: &support::Fixture, q: SignedCallRequest, policy: FixedSignerPolicy) -> Ordinary {
    ordinary_version(f, q, policy, SignedVersion::LegacyV1)
}
fn ordinary_version(
    f: &support::Fixture,
    q: SignedCallRequest,
    policy: FixedSignerPolicy,
    version: SignedVersion,
) -> Ordinary {
    let s = SignedStaticBindings::from_source(
        &f.source,
        &f.program,
        &StaticBindings::legacy(&f.deployment),
        &policy,
        version,
    )
    .unwrap();
    q.admit(&s, &q.action).unwrap();
    let a = compile_signed(&s, &q.action).unwrap();
    let d = SignedInstalledDeployment {
        profile: version.profile().into(),
        qualification: QUALIFICATION.into(),
        phase_policy: "guaranteed-only".into(),
        storage_abi: version.storage_abi().into(),
        signed_abi: version.signed_abi().into(),
        bindings: s.bindings.clone(),
        frame: FrameBindings::legacy(&f.deployment),
        keyset_identity: String::new(),
        signer_policy: policy,
    };
    // Real keyless seed admission, source evaluation and native query/partition.
    let frame = layout::admit_signed_frame(
        &s,
        &a.manifest.abi,
        &d.frame,
        codec::decode_native(&f.predecessor).unwrap(),
        layout::OperationPolicy::Keyless,
    )
    .unwrap();
    let pre = layout::attach_request(frame, &s.bindings, &q.native()).unwrap();
    let b = prepare::prepare_admitted_signed(
        &f.program,
        &s.bindings,
        &d.frame,
        pre,
        codec::decode_native(&f.parameters).unwrap(),
        &q.native(),
        None,
        PROFILE,
        &codec::object_hash(&d).unwrap(),
        codec::OUTPUT_BYTES as u64,
        codec::OUTPUT_BYTES as u64,
        Some(&s),
        None,
    )
    .unwrap();
    let package =
        preparation::build_package(&s, &d, &f.predecessor, &f.parameters, &q, &a, &b).unwrap();
    let attachment = SignedCallAttachment {
        profile: ATTACHMENT.into(),
        payload: package.payload.clone(),
        signature_hex: sign(message(&package.payload).unwrap(), 7, 11),
    };
    schnorr::verify_attachment(
        &s.policy.public_key_hex,
        &attachment.signature_hex,
        message(&package.payload).unwrap(),
    )
    .unwrap();
    Ordinary {
        s,
        a,
        d,
        q,
        package,
        attachment,
    }
}
fn raw(f: &support::Fixture, o: &Ordinary) -> Result<prepare::PreparedBundle> {
    let frame = layout::admit_signed_frame(
        &o.s,
        &o.a.manifest.abi,
        &o.d.frame,
        codec::decode_native(&f.predecessor)?,
        layout::OperationPolicy::Keyless,
    )?;
    let pre = layout::attach_request(frame, &o.s.bindings, &o.q.native())?;
    let input = preparation::typed_input(&o.s, &o.q, &o.package, &o.attachment)?;
    // Bypass only the public host cap check. This is actual integrated IR.check
    // and actual native VM/processed/communications preparation, never keygen.
    prepare::prepare_admitted_signed(
        &f.program,
        &o.s.bindings,
        &o.d.frame,
        pre,
        codec::decode_native(&f.parameters)?,
        &o.q.native(),
        Some(&o.a),
        PROFILE,
        &codec::object_hash(&o.d)?,
        codec::OUTPUT_BYTES as u64,
        codec::OUTPUT_BYTES as u64,
        Some(&o.s),
        Some(input),
    )
}
#[test]
fn signed_metered_literal_complete_native_results_and_attachment_specific_communications() {
    for (running, amount, charge, caps, total, duty, alice, bob) in [
        (4, 7, None, [16, 2, 12, 4], 11, 4, 84, 19),
        (31, 3, Some(9), [8, 2, 4, 9], 34, 9, 92, 11),
    ] {
        let f = metered(running, amount, charge);
        let mut o = ordinary(&f, request(&f, caps), policy("bob"));
        let b = raw(&f, &o).unwrap();
        assert_eq!(b.readback.execution.effects.len(), 8);
        assert_eq!(
            b.readback.execution.post.state["running"],
            Value::UInt(UInt(total))
        );
        assert_eq!(
            b.readback.execution.post.state["enabled"],
            Value::Bool(true)
        );
        assert_eq!(b.readback.execution.post.liabilities[0].amount, UInt(duty));
        for (account, n) in [("alice", alice), ("bob", bob), ("treasury", 3)] {
            assert_eq!(
                b.readback
                    .prospective_balances
                    .iter()
                    .find(|l| l.account == account)
                    .unwrap()
                    .amount,
                UInt(n)
            );
        }
        let rebuilt =
            preparation::build_package(&o.s, &o.d, &f.predecessor, &f.parameters, &o.q, &o.a, &b)
                .unwrap();
        assert_eq!(rebuilt, o.package);
        let checked = b.readback.relation.as_ref().unwrap();
        assert!(checked.noop_count > 0 || charge.is_some());
        assert!(checked.raw_op_count > 120);
        assert!(o.a.ir.do_communications_commitment);
        assert_eq!(o.a.ir.outputs.len(), 0);
        assert_eq!(o.a.manifest.input_count, 18);
        let sm = o.a.manifest.signed.as_ref().unwrap();
        assert_eq!(sm.typed_argument_count, 16);
        assert_eq!(sm.flattened_field_count, 18);
        let call = b.native.intent.calls().next().unwrap();
        let old = call.communication_commitment;
        o.attachment.signature_hex = sign(message(&o.package.payload).unwrap(), 7, 13);
        let b2 = raw(&f, &o).unwrap();
        assert_eq!(
            o.package,
            preparation::build_package(&o.s, &o.d, &f.predecessor, &f.parameters, &o.q, &o.a, &b2)
                .unwrap()
        );
        assert_ne!(
            old,
            b2.native
                .intent
                .calls()
                .next()
                .unwrap()
                .communication_commitment
        );
    }
}
#[test]
fn freshly_signed_tighter_caps_fail_actual_integrated_constraint_not_host_gate() {
    for (running, amount, charge, caps) in
        [(4, 7, None, [16, 2, 12, 4]), (31, 3, Some(9), [8, 2, 4, 9])]
    {
        let f = metered(running, amount, charge);
        let positive = ordinary(&f, request(&f, caps), policy("bob"));
        raw(&f, &positive).unwrap();
        for i in 0..4 {
            let mut tight = caps;
            tight[i] = if i == 2 { tight[i] + 1 } else { tight[i] - 1 };
            let o = ordinary(&f, request(&f, tight), policy("bob"));
            schnorr::verify_attachment(
                &o.s.policy.public_key_hex,
                &o.attachment.signature_hex,
                message(&o.package.payload).unwrap(),
            )
            .unwrap();
            assert_eq!(raw(&f, &o).err().unwrap().code, "unsatisfied", "cap {i}");
        }
    }
    let mut f = metered(4, 7, Some(9));
    f.request.action = "freeze".into();
    f.request.inputs.clear();
    let o = ordinary(&f, request(&f, [0, 0, 0, 9]), policy("bob"));
    let b = raw(&f, &o).unwrap();
    assert_eq!(b.readback.execution.effects.len(), 2);
    assert_eq!(b.readback.execution.post.liabilities[0].amount, UInt(9));
    assert_eq!(
        b.readback.execution.post.state["running"],
        Value::UInt(UInt(4))
    );
    assert_eq!(
        b.readback.execution.post.state["enabled"],
        Value::Bool(false)
    );
    let o = ordinary(&f, request(&f, [0, 0, 0, 8]), policy("bob"));
    assert_eq!(raw(&f, &o).err().unwrap().code, "unsatisfied");
}
#[test]
fn state_only_toggle_and_reset_still_require_actual_signature() {
    let source = include_str!("../../tests/fixtures/ToggleCounter.mori");
    for (count, action, active, delta, next) in [
        (31, "step", true, 1, 32),
        (255, "step", true, 1, 256),
        (256, "reset", false, 0, 0),
    ] {
        let f = signed_parameters(support::fixture(
            source,
            action,
            Scenario {
                inputs: if action == "step" {
                    BTreeMap::from([("delta".into(), Value::UInt(UInt(delta)))])
                } else {
                    BTreeMap::new()
                },
                state: BTreeMap::from([
                    ("count".into(), Value::UInt(UInt(count))),
                    ("active".into(), Value::Bool(active)),
                ]),
                ..Default::default()
            },
        ));
        let mut o = ordinary(&f, request(&f, [0; 4]), policy("alice"));
        let b = raw(&f, &o).unwrap();
        assert!(o.q.limits.is_empty());
        assert_eq!(
            b.readback.execution.post.state["count"],
            Value::UInt(UInt(next))
        );
        assert_eq!(
            b.readback.execution.post.state["active"],
            Value::Bool(!active)
        );
        assert_eq!(
            b.readback.execution.effects.len(),
            if action == "step" { 4 } else { 3 }
        );
        o.attachment.signature_hex = sign(message(&o.package.payload).unwrap(), 13, 11);
        assert_eq!(raw(&f, &o).err().unwrap().code, "unsatisfied");
    }
}
#[test]
fn strict_schema_source_policy_and_typed_layout_controls() {
    let f = metered(4, 7, None);
    let o = ordinary(&f, request(&f, [16, 2, 12, 4]), policy("bob"));
    let text = serde_json::to_string(&o.s.policy).unwrap();
    for bad in [
        text.replace("{", "{\"private\":true,"),
        text.replace("{", "{\"profile\":\"duplicate\","),
        text.replace("public_key_hex", "missing"),
    ] {
        assert!(FixedSignerPolicy::from_json(&bad).is_err());
        assert!(serde_json::from_str::<FixedSignerPolicy>(&bad).is_err());
    }
    for kind in ["request", "attachment", "package"] {
        let text = match kind {
            "request" => serde_json::to_string(&o.q).unwrap(),
            "attachment" => serde_json::to_string(&o.attachment).unwrap(),
            _ => serde_json::to_string(&o.package).unwrap(),
        };
        let bad = text.replace("{", "{\"unknown\":false,");
        match kind {
            "request" => assert!(SignedCallRequest::from_json(&bad).is_err()),
            "attachment" => assert!(SignedCallAttachment::from_json(&bad).is_err()),
            _ => assert!(CallSigningPackage::from_json(&bad).is_err()),
        }
    }
    let mut bounds = o.s.bindings.clone();
    bounds.economic_bounds = Some(Bounds {
        owner: "alice".into(),
        limits: o.q.limits.clone(),
    });
    assert_eq!(
        SignedStaticBindings::from_source(
            &f.source,
            &f.program,
            &bounds,
            &o.s.policy,
            SignedVersion::LegacyV1
        )
        .err()
        .unwrap()
        .code,
        "economic-bounds-forbidden"
    );
    let b = raw(&f, &o).unwrap();
    let midnight_ledger::structure::ProofPreimageVersioned::V2(p) =
        &b.native.intent.calls().next().unwrap().proof
    else {
        panic!()
    };
    for tweak in 0..3 {
        let mut changed = (**p).clone();
        match tweak {
            0 => {
                changed.inputs.pop();
            }
            1 => changed.inputs.push(Fr::from(0)),
            _ => changed.inputs.swap(0, 1),
        }
        assert!(relation::check(&o.a, &changed).is_err());
    }
    assert_eq!(
        o.a.ir
            .instructions
            .iter()
            .filter(|i| matches!(i, midnight_zkir_v3::Instruction::Output { .. }))
            .count(),
        1
    );
    assert!(
        !o.a.ir
            .instructions
            .iter()
            .any(|i| matches!(i, midnight_zkir_v3::Instruction::PrivateInput { .. }))
    );
    assert_eq!(
        serde_json::to_value(&o.a.ir.inputs[1]).unwrap()["type"],
        serde_json::to_value(midnight_zkir_v3::ir_types::IrType::JubjubPoint).unwrap()
    );
}
fn preimage(b: &prepare::PreparedBundle) -> midnight_transient_crypto::proofs::ProofPreimage {
    let midnight_ledger::structure::ProofPreimageVersioned::V2(p) =
        &b.native.intent.calls().next().unwrap().proof
    else {
        panic!()
    };
    (**p).clone()
}
fn recommit(p: &mut midnight_transient_crypto::proofs::ProofPreimage) {
    let (_, rand) = p.communications_commitment.unwrap();
    p.communications_commitment = Some((
        midnight_transient_crypto::hash::transient_commit(&p.inputs[..], rand),
        rand,
    ));
}
fn changed_artifact(
    a: &relation::Artifact,
    instructions: Vec<midnight_zkir_v3::Instruction>,
) -> relation::Artifact {
    let mut ir = a.ir.clone();
    ir.instructions = std::sync::Arc::new(instructions);
    let mut manifest = a.manifest.clone();
    manifest.ir_hash = codec::hash(&codec::encode(&ir).unwrap());
    relation::Artifact { ir, manifest }
}
#[test]
fn precise_cap_assertion_ablation_controls_keep_the_honest_relation_intact() {
    let f = metered(4, 7, None);
    for i in 0..4 {
        let mut cap = [16, 2, 12, 4];
        cap[i] = if i == 2 { cap[i] + 1 } else { cap[i] - 1 };
        let o = ordinary(&f, request(&f, cap), policy("bob"));
        let pre = layout::decode_predecessor(
            &f.program,
            &o.a.manifest.abi,
            &f.deployment,
            codec::decode_native(&f.predecessor).unwrap(),
            &o.q.native(),
        )
        .unwrap();
        let b = prepare::prepare_admitted_signed(
            &f.program,
            &o.s.bindings,
            &o.d.frame,
            pre,
            codec::decode_native(&f.parameters).unwrap(),
            &o.q.native(),
            None,
            PROFILE,
            &codec::object_hash(&o.d).unwrap(),
            codec::OUTPUT_BYTES as u64,
            codec::OUTPUT_BYTES as u64,
            Some(&o.s),
            Some(preparation::typed_input(&o.s, &o.q, &o.package, &o.attachment).unwrap()),
        )
        .unwrap();
        let p = preimage(&b);
        assert_eq!(relation::check(&o.a, &p).err().unwrap().code, "unsatisfied");
        let asserts =
            o.a.ir
                .instructions
                .iter()
                .enumerate()
                .filter(|(_, ins)| matches!(ins, midnight_zkir_v3::Instruction::Assert { .. }))
                .map(|(idx, _)| idx)
                .collect::<Vec<_>>();
        let target = asserts[asserts.len() - 4 + i];
        let ablated = changed_artifact(
            &o.a,
            o.a.ir
                .instructions
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx != target)
                .map(|(_, i)| i.clone())
                .collect(),
        );
        relation::check(&ablated, &p).unwrap(); // Diagnostic only, not an adopted relation.
    }
}
#[test]
fn typed_curve_scalar_fixed_key_fens_and_communications_hostile_controls() {
    use midnight_curves::{
        Fq, JubjubAffine, JubjubExtended,
        pairing::group::ff::{Field, PrimeField},
    };
    let f = metered(4, 7, None);
    let o = ordinary(&f, request(&f, [16, 2, 12, 4]), policy("bob"));
    let b = raw(&f, &o).unwrap();
    let original = preimage(&b);
    let torsion = JubjubAffine::from_raw_unchecked(Fq::ZERO, -Fq::ONE);
    let mixed = JubjubAffine::from(
        JubjubExtended::from(JubjubSubgroup::generator()) + JubjubExtended::from(torsion),
    );
    for (case, x, y) in [
        ("identity", Fr::from(0), Fr::from(1)),
        ("offcurve", Fr::from(1), Fr::from(1)),
        ("torsion", Fr(Fq::ZERO), Fr(-Fq::ONE)),
        ("mixed", Fr(mixed.get_u()), Fr(mixed.get_v())),
    ] {
        let text = format!("{}{}", schnorr::native_hex(x), schnorr::native_hex(y));
        assert!(schnorr::decode_key(&text).is_err(), "host {case}");
        for base in [1, 3] {
            let mut p = original.clone();
            p.inputs[base] = x;
            p.inputs[base + 1] = y;
            recommit(&mut p);
            let refused = relation::check(&o.a, &p).err().unwrap();
            assert_eq!(refused.code, "unsatisfied", "typed {case}");
        }
    }
    let mut r = (-Scalar::ONE).to_bytes();
    for byte in &mut r {
        let (next, carry) = byte.overflowing_add(1);
        *byte = next;
        if !carry {
            break;
        }
    }
    let mut r_plus = r;
    for byte in &mut r_plus {
        let (next, carry) = byte.overflowing_add(1);
        *byte = next;
        if !carry {
            break;
        }
    }
    for bytes in [r, r_plus] {
        let mut p = original.clone();
        p.inputs[5] = Fr::from_le_bytes(&bytes).unwrap();
        recommit(&mut p);
        assert_eq!(relation::check(&o.a, &p).err().unwrap().code, "unsatisfied");
        let mut sig = o.attachment.signature_hex[..128].to_string();
        sig.push_str(&codec::hex(&bytes));
        assert!(schnorr::decode_signature(&sig).is_err());
    }
    for scalar in [Scalar::ZERO, Scalar::ONE, -Scalar::ONE] {
        let mut p = original.clone();
        p.inputs[5] = Fr::from_le_bytes(&scalar.to_bytes()).unwrap();
        recommit(&mut p);
        let sig = format!(
            "{}{}",
            &o.attachment.signature_hex[..128],
            codec::hex(&scalar.to_bytes())
        );
        schnorr::decode_signature(&sig).unwrap();
        assert_eq!(relation::check(&o.a, &p).err().unwrap().code, "unsatisfied");
    }
    let native_modulus = midnight_curves::Fq::MODULUS;
    assert!(!native_modulus.is_empty());
    assert!(schnorr::decode_native_hex(&"ff".repeat(32)).is_err());
    assert!(schnorr::decode_key(&key(7).to_uppercase()).is_err());
    let mut attacker = original.clone();
    let pk = schnorr::coordinates(schnorr::decode_key(&key(13)).unwrap());
    let sig = sign(message(&o.package.payload).unwrap(), 13, 11);
    schnorr::verify_attachment(&key(13), &sig, message(&o.package.payload).unwrap()).unwrap();
    let witness = schnorr::decode_signature(&sig).unwrap();
    let rr = schnorr::coordinates(witness.r);
    attacker.inputs[1] = pk.0;
    attacker.inputs[2] = pk.1;
    attacker.inputs[3] = rr.0;
    attacker.inputs[4] = rr.1;
    attacker.inputs[5] = witness.response;
    recommit(&mut attacker);
    assert_eq!(
        relation::check(&o.a, &attacker).err().unwrap().code,
        "unsatisfied"
    );
    // Ablating only source-fixed PK equality isolates that mandatory predicate.
    let coordinates = schnorr::coordinates(schnorr::decode_key(&key(7)).unwrap());
    let ablated=changed_artifact(&o.a,o.a.ir.instructions.iter().filter(|ins|!matches!(ins,midnight_zkir_v3::Instruction::ConstrainEq{b:midnight_zkir_v3::ir::Operand::Immediate(v),..} if *v==coordinates.0 || *v==coordinates.1)).cloned().collect());
    relation::check(&ablated, &attacker).unwrap();
    for i in [0, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16, 17] {
        let mut p = original.clone();
        p.inputs[i] = p.inputs[i] + Fr::from(1);
        recommit(&mut p);
        assert!(relation::check(&o.a, &p).is_err(), "signed typed field {i}");
    }
    let mut in_range = original.clone();
    in_range.inputs[13] = Fr::from(3);
    recommit(&mut in_range);
    relation::check(&o.a, &in_range).unwrap();
    in_range.inputs[13] = Fr::from(4);
    recommit(&mut in_range);
    assert_eq!(
        relation::check(&o.a, &in_range).err().unwrap().code,
        "unsatisfied"
    );
    let mut mismatch = original.clone();
    mismatch.communications_commitment.as_mut().unwrap().0 =
        mismatch.communications_commitment.unwrap().0 + Fr::from(1);
    assert_eq!(
        relation::check(&o.a, &mismatch).err().unwrap().code,
        "unsatisfied"
    );
    let mut private = original.clone();
    private.private_transcript.push(Fr::from(0));
    assert_eq!(
        relation::check(&o.a, &private).err().unwrap().code,
        "preimage-domain"
    );
}
#[test]
fn complete_native_observation_count_and_untouched_duty_amount_are_constrained() {
    let mut f = metered(4, 7, Some(9));
    f.request.action = "freeze".into();
    f.request.inputs.clear();
    let o = ordinary(&f, request(&f, [0, 0, 0, 9]), policy("bob"));
    let b = raw(&f, &o).unwrap();
    let original = preimage(&b);
    for (origin, which) in [
        ("signed-state-cardinality", 0),
        ("signed-duty-cardinality", 0),
        ("signed-duty-opening", 0),
        ("signed-duty-opening", 1),
    ] {
        let positions = b
            .native
            .lowered
            .bindings
            .iter()
            .filter(|binding| binding.origin == origin)
            .map(|binding| binding.op as usize)
            .filter(|i| {
                matches!(
                    b.native.lowered.program[*i],
                    midnight_onchain_vm::ops::Op::Popeq { .. }
                )
            })
            .collect::<Vec<_>>();
        let index = positions[which];
        let ops = b.native.lowered.program.clone();
        let input_start = ops[..index]
            .iter()
            .map(midnight_transient_crypto::repr::FieldRepr::field_size)
            .sum::<usize>();
        let mut output_start = 0;
        for op in &ops[..index] {
            if let midnight_onchain_vm::ops::Op::Popeq { result, .. } = op {
                let mut fields = vec![];
                result.value_only_field_repr(&mut fields);
                output_start += fields.len();
            }
        }
        let midnight_onchain_vm::ops::Op::Popeq { result, .. } = &ops[index] else {
            panic!()
        };
        let mut fields = vec![];
        result.value_only_field_repr(&mut fields);
        let mut p = original.clone();
        let input_index =
            input_start + midnight_transient_crypto::repr::FieldRepr::field_size(&ops[index]) - 1;
        let output_index = output_start + fields.len() - 1;
        p.public_transcript_inputs[input_index] =
            p.public_transcript_inputs[input_index] + Fr::from(1);
        p.public_transcript_outputs[output_index] =
            p.public_transcript_outputs[output_index] + Fr::from(1);
        assert_eq!(
            relation::check(&o.a, &p).err().unwrap().code,
            "unsatisfied",
            "{origin} {which}"
        );
    }
    for charge in [None, Some(0)] {
        let mut f = metered(4, 7, charge);
        f.request.action = "freeze".into();
        f.request.inputs.clear();
        let o = ordinary(&f, request(&f, [0, 0, 0, 0]), policy("bob"));
        let b = raw(&f, &o).unwrap();
        assert_eq!(
            b.readback.execution.post.liabilities.len(),
            usize::from(charge.is_some())
        );
        assert_eq!(
            o.package
                .payload
                .wire_projection
                .iter()
                .find(|v| v.descriptor == "post.duty.charge.present")
                .unwrap()
                .value,
            Value::Bool(charge.is_some())
        );
    }
}
#[test]
fn original_sdk_default_parameters_retain_honest_signed_partition_refusal() {
    let source = include_str!("../../tests/fixtures/ToggleCounter.mori");
    let f = support::fixture(
        source,
        "step",
        Scenario {
            inputs: BTreeMap::from([("delta".into(), Value::UInt(UInt(1)))]),
            ..Default::default()
        },
    );
    let s = SignedStaticBindings::from_source(
        &f.source,
        &f.program,
        &StaticBindings::legacy(&f.deployment),
        &policy("alice"),
        SignedVersion::LegacyV1,
    )
    .unwrap();
    let a = compile_signed(&s, "step").unwrap();
    let q = request(&f, [0; 4]);
    let pre = layout::decode_predecessor(
        &f.program,
        &a.manifest.abi,
        &f.deployment,
        codec::decode_native(&f.predecessor).unwrap(),
        &q.native(),
    )
    .unwrap();
    assert_eq!(
        prepare::prepare_admitted_signed(
            &f.program,
            &s.bindings,
            &FrameBindings::legacy(&f.deployment),
            pre,
            codec::decode_native(&f.parameters).unwrap(),
            &q.native(),
            None,
            PROFILE,
            &codec::object_hash(&f.deployment).unwrap(),
            codec::OUTPUT_BYTES as u64,
            codec::OUTPUT_BYTES as u64,
            Some(&s),
            None
        )
        .err()
        .unwrap()
        .code,
        "all-fallible"
    );
}
#[test]
fn fixed_owner_must_match_native_payer_even_for_state_only_source() {
    let f = signed_parameters(support::fixture(
        include_str!("../../tests/fixtures/ToggleCounter.mori"),
        "reset",
        Scenario::default(),
    ));
    let mut d = StaticBindings::legacy(&f.deployment);
    let remittance = metered(4, 7, None);
    d.roles.push(
        remittance
            .deployment
            .roles
            .iter()
            .find(|r| r.role == "bob")
            .unwrap()
            .clone(),
    );
    let abi = layout::admit_bindings(&f.program, &d).unwrap();
    d.abi_hash = codec::object_hash(&abi).unwrap();
    let mut p = policy("bob");
    p.owner = "bob".into();
    assert!(
        SignedStaticBindings::from_source(&f.source, &f.program, &d, &p, SignedVersion::LegacyV1)
            .is_err()
    );
}
#[test]
fn direct_request_decoding_rejects_duplicate_source_input_names() {
    let f = metered(4, 7, None);
    let q = request(&f, [16, 2, 12, 4]);
    let text = serde_json::to_string(&q).unwrap();
    let bad = text.replace(
        "\"inputs\":{",
        "\"inputs\":{\"amount\":{\"type\":\"UInt\",\"value\":\"8\"},",
    );
    assert!(SignedCallRequest::from_json(&bad).is_err());
    assert!(serde_json::from_str::<SignedCallRequest>(&bad).is_err());
}
#[test]
fn general_source_admission_bounds_unused_actions_and_ordered_limits() {
    let f = metered(4, 7, None);
    let s = SignedStaticBindings::from_source(
        &f.source,
        &f.program,
        &StaticBindings::legacy(&f.deployment),
        &policy("bob"),
        SignedVersion::LegacyV1,
    )
    .unwrap();
    let mut q = request(&f, [16, 2, 12, 4]);
    q.limits.clear();
    assert_eq!(
        s.admit_request(&q, "settle").err().unwrap().code,
        "complete-ordered-limits"
    );
    q = request(&f, [16, 2, 12, 4]);
    q.limits.push(q.limits[0].clone());
    assert_eq!(
        s.admit_request(&q, "settle").err().unwrap().code,
        "complete-ordered-limits"
    );
    q = request(&f, [16, 2, 12, 4]);
    q.limits[0].beneficiary = "treasury".into();
    assert!(s.admit_request(&q, "settle").is_err());
    q = request(&f, [16, 2, 12, 4]);
    q.at = UInt(0);
    assert_eq!(
        s.admit_request(&q, "settle").err().unwrap().code,
        "request-scope"
    );
    let mut d = s.bindings.clone();
    d.duties = (0..17)
        .map(|i| DutyBinding {
            name: format!("extra-{i}"),
            ..s.duties[0].clone()
        })
        .collect();
    assert_eq!(
        SignedStaticBindings::from_source(
            &f.source,
            &f.program,
            &d,
            &s.policy,
            SignedVersion::LegacyV1
        )
        .err()
        .unwrap()
        .code,
        "scope-limit"
    );
    let source = f.source.replace(
        "action freeze() {",
        "action freeze() { transfer(QA,\"bob\",\"alice\",1);",
    );
    let p = mori_core::parse(&source).unwrap();
    let mut d = s.bindings.clone();
    d.source_hash = codec::hash(source.as_bytes());
    d.program_hash = codec::object_hash(&p).unwrap();
    assert_eq!(
        SignedStaticBindings::from_source(&source, &p, &d, &s.policy, SignedVersion::LegacyV1)
            .err()
            .unwrap()
            .code,
        "one-public-payer"
    );
    let source = f.source.replace(
        "action freeze() {",
        "action freeze() { liability(\"other\",QA,\"bob\",\"alice\",1);",
    );
    let p = mori_core::parse(&source).unwrap();
    let mut d = s.bindings.clone();
    d.source_hash = codec::hash(source.as_bytes());
    d.program_hash = codec::object_hash(&p).unwrap();
    d.duties.push(DutyBinding {
        name: "other".into(),
        asset: f.program.assets[0].identity.clone(),
        debtor: "bob".into(),
        creditor: "alice".into(),
    });
    let abi = layout::admit_bindings(&p, &d).unwrap();
    d.abi_hash = codec::object_hash(&abi).unwrap();
    assert_eq!(
        SignedStaticBindings::from_source(&source, &p, &d, &s.policy, SignedVersion::LegacyV1)
            .err()
            .unwrap()
            .code,
        "duty-registry"
    );
    let a = compile_signed(&s, "settle").unwrap();
    let b = compile_signed(&s, "freeze").unwrap();
    assert_ne!(a.manifest.ir_hash, b.manifest.ir_hash);
    assert_eq!(b.manifest.signed.as_ref().unwrap().signer_policy, s.policy);
    let mut another = s.policy.clone();
    another.public_key_hex = key(13);
    let another = SignedStaticBindings::from_source(
        &f.source,
        &f.program,
        &s.bindings,
        &another,
        SignedVersion::LegacyV1,
    )
    .unwrap();
    assert_ne!(
        compile_signed(&another, "freeze").unwrap().manifest.ir_hash,
        b.manifest.ir_hash
    );
    let bytes = a.manifest_bytes().unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert!(v.get("input_count").is_none());
    assert_eq!(v["signed"]["typed_argument_count"], 16);
    assert_eq!(v["signed"]["flattened_field_count"], 18);
    let d = StaticBindings::legacy(&f.deployment);
    let a = crate::relation::compile_bindings(&f.source, &f.program, &d, "settle").unwrap();
    assert!(
        serde_json::from_slice::<serde_json::Value>(&a.manifest_bytes().unwrap())
            .unwrap()
            .get("input_count")
            .is_some()
    );
}
#[test]
fn native_transcript_and_transport_mutations_cannot_become_signed_success() {
    let f = metered(4, 7, None);
    let o = ordinary(&f, request(&f, [16, 2, 12, 4]), policy("bob"));
    let b = raw(&f, &o).unwrap();
    let original = preimage(&b);
    let mut no_comm = o.a.ir.clone();
    no_comm.do_communications_commitment = false;
    let mut manifest = o.a.manifest.clone();
    manifest.ir_hash = codec::hash(&codec::encode(&no_comm).unwrap());
    let changed = relation::Artifact {
        ir: no_comm,
        manifest,
    };
    assert_eq!(
        relation::check(&changed, &original).err().unwrap().code,
        "preimage-domain"
    );
    let mut output = original.clone();
    output.public_transcript_outputs.push(Fr::from(0));
    assert_eq!(
        relation::check(&o.a, &output).err().unwrap().code,
        "unsatisfied"
    );
    let mut outputs = original.clone();
    outputs.public_transcript_outputs.swap(1, 2);
    assert_eq!(
        relation::check(&o.a, &outputs).err().unwrap().code,
        "unsatisfied"
    );
    for domain in [MESSAGE_DOMAIN, schnorr::FENS_DOMAIN as u128] {
        let instructions=o.a.ir.instructions.iter().map(|ins|match ins {midnight_zkir_v3::Instruction::TransientHash{inputs,output} if matches!(inputs.first(),Some(midnight_zkir_v3::ir::Operand::Immediate(v)) if *v==Fr::from(domain))=>{let mut inputs=inputs.clone();inputs[0]=midnight_zkir_v3::ir::Operand::Immediate(Fr::from(domain+1));midnight_zkir_v3::Instruction::TransientHash{inputs,output:output.clone()}},_=>ins.clone()}).collect();
        let changed = changed_artifact(&o.a, instructions);
        assert_eq!(
            relation::check(&changed, &original).err().unwrap().code,
            "unsatisfied"
        );
    }
    let mut scalar = original.clone();
    scalar.inputs[5] = Fr::from(0);
    recommit(&mut scalar);
    let asserts =
        o.a.ir
            .instructions
            .iter()
            .enumerate()
            .filter(|(_, i)| matches!(i, midnight_zkir_v3::Instruction::Assert { .. }))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
    let fens_assert = asserts[asserts.len() - 7];
    let ablated = changed_artifact(
        &o.a,
        o.a.ir
            .instructions
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != fens_assert)
            .map(|(_, i)| i.clone())
            .collect(),
    );
    relation::check(&ablated, &scalar).unwrap();
    assert_eq!(
        relation::check(&o.a, &scalar).err().unwrap().code,
        "unsatisfied"
    );
    let mut r = (-Scalar::ONE).to_bytes();
    for byte in &mut r {
        let (value, carry) = byte.overflowing_add(1);
        *byte = value;
        if !carry {
            break;
        }
    }
    let mut shifted = original.clone();
    shifted.inputs[5] = shifted.inputs[5] + Fr::from_le_bytes(&r).unwrap();
    recommit(&mut shifted);
    assert_eq!(
        relation::check(&o.a, &shifted).err().unwrap().code,
        "unsatisfied"
    );
}
#[test]
fn extra_unknown_zero_native_duty_refuses_and_bound_untouched_duties_are_complete() {
    use midnight_onchain_state::state::StateValue;
    let mut f = metered(4, 7, Some(9));
    let mut state: layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
    let address = codec::from_hex(&f.deployment.address_hex).unwrap();
    let mut contract = state.index(address).unwrap();
    let StateValue::Array(root) = &*contract.data.get() else {
        panic!()
    };
    let StateValue::Map(map) = root.get(1).unwrap() else {
        panic!()
    };
    let unknown = DutyBinding {
        name: "unknown".into(),
        ..f.deployment.duties[0].clone()
    };
    let map = map.insert(
        layout::name_key("duty", &unknown.name),
        crate::ops::cell(layout::duty_value(&unknown, 0, &f.deployment.roles).unwrap()),
    );
    let root = root.insert(1, StateValue::Map(map)).unwrap();
    contract.data = midnight_onchain_state::state::ChargedState::new(StateValue::Array(root));
    state.contract = state.contract.insert(address, contract);
    support::repin(&mut f, &state);
    assert_eq!(
        crate::local_deployment::admit_local_seed(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters
        )
        .err()
        .unwrap()
        .code,
        "unknown-duty"
    );
    let mut f = metered(4, 7, None);
    let d = DutyBinding {
        name: "retained".into(),
        ..f.deployment.duties[0].clone()
    };
    f.deployment.duties.push(d);
    f.deployment.abi_hash =
        codec::object_hash(&layout::admit_source(&f.program, &f.deployment).unwrap()).unwrap();
    let o = ordinary(&f, request(&f, [16, 2, 12, 4]), policy("bob"));
    let b = raw(&f, &o).unwrap();
    assert!(
        o.package
            .payload
            .wire_projection
            .iter()
            .any(|v| v.descriptor == "post.duty.retained.present" && v.value == Value::Bool(false))
    );
    assert_eq!(b.readback.execution.post.liabilities.len(), 1);
}
#[test]
fn general_financial_arithmetic_zero_self_payments_overflow_and_net_underflow() {
    let source = r#"language "moriarty/1"; agreement Arithmetic {asset QA={network:"local",issuer:"qa",reference:"unit",scale:0}; action run(amount:UInt) {transfer(QA,"alice","alice",amount);transfer(QA,"alice","bob",amount);fee(QA,"alice","treasury",2);} }"#;
    let p = mori_core::parse(source).unwrap();
    let asset = p.assets[0].identity.clone();
    for amount in [0, 3] {
        let f = signed_parameters(support::fixture(
            source,
            "run",
            Scenario {
                inputs: BTreeMap::from([("amount".into(), Value::UInt(UInt(amount)))]),
                balances: [("alice", 100), ("bob", 5), ("treasury", 1)]
                    .into_iter()
                    .map(|(account, n)| Balance {
                        account: account.into(),
                        asset: asset.clone(),
                        amount: UInt(n),
                    })
                    .collect(),
                ..Default::default()
            },
        ));
        let o = ordinary(&f, request(&f, [2 * amount + 2, 2, 0, 0]), policy("bob"));
        if amount == 0 {
            assert_eq!(raw(&f, &o).err().unwrap().code, "unsatisfied");
        } else {
            let b = raw(&f, &o).unwrap();
            assert_eq!(b.readback.execution.effects.len(), 3);
            let p = preimage(&b);
            let mut overflow = p;
            overflow.inputs[0] = Fr::from(u128::MAX);
            recommit(&mut overflow);
            assert_eq!(
                relation::check(&o.a, &overflow).err().unwrap().code,
                "unsatisfied"
            );
        }
    }
    let source = r#"language "moriarty/1"; agreement SelfPay {asset QA={network:"local",issuer:"qa",reference:"unit",scale:0};action run(amount:UInt) {transfer(QA,"alice","alice",amount);} }"#;
    let p = mori_core::parse(source).unwrap();
    let f = signed_parameters(support::fixture(
        source,
        "run",
        Scenario {
            inputs: BTreeMap::from([("amount".into(), Value::UInt(UInt(3)))]),
            balances: vec![Balance {
                account: "alice".into(),
                asset: p.assets[0].identity.clone(),
                amount: UInt(100),
            }],
            ..Default::default()
        },
    ));
    let mut q = request(&f, [3, 0, 0, 0]);
    q.limits[0].beneficiary = "alice".into();
    let o = ordinary(&f, q, policy("alice"));
    raw(&f, &o).unwrap();
}
#[test]
fn complete_signed_source_map_preflight_refuses_unused_key_mutation_and_downgrade() {
    use crate::call_artifacts::{ActionKeyMetadata, ImportedPublicKeyset, PublicKeysetManifest};
    let f = metered(4, 7, None);
    let o = ordinary(&f, request(&f, [16, 2, 12, 4]), policy("bob"));
    let limits = NativeWorkLimits {
        profile: "moriarty-native-call-work-limits/1".into(),
        max_degree: 19,
        max_actions: 4,
        max_srs_bytes: 128 * 1024 * 1024,
        max_prover_key_bytes: 1,
        max_verifier_key_bytes: 1024,
        max_input_bytes: 1048576,
        max_leaf_bytes: 16777216,
        max_artifact_bytes: 16777216,
        max_operations: 10000,
        max_public_inputs: 10000,
    };
    let artifacts = crate::local_deployment::compile_all_policy(
        &f.source,
        &f.program,
        &o.s.bindings,
        &limits,
        Some((&o.s.policy, o.s.version)),
    )
    .unwrap();
    let action_bytes = artifacts
        .iter()
        .map(|a| {
            (
                a.ir_bytes().unwrap(),
                a.manifest_bytes().unwrap(),
                b"untrusted-opaque-public-key".to_vec(),
            )
        })
        .collect::<Vec<_>>();
    let actions = artifacts
        .iter()
        .enumerate()
        .map(|(ordinal, a)| ActionKeyMetadata {
            name: a.manifest.action.clone(),
            ordinal,
            ir_hash: codec::hash(&action_bytes[ordinal].0),
            manifest_hash: codec::hash(&action_bytes[ordinal].1),
            vk_hash: codec::hash(&action_bytes[ordinal].2),
            degree: 9,
            srs_digest: crate::parameters::expected_digest(9).unwrap().into(),
            srs_bytes: 1,
        })
        .collect();
    let mut manifest = PublicKeysetManifest {
        profile: KEYSET.into(),
        signer_policy: Some(o.s.policy.clone()),
        bindings: o.s.bindings.clone(),
        build: crate::proving::source_closure(),
        actions,
        identity: String::new(),
        leaves: Default::default(),
    };
    manifest.identity = crate::call_artifacts::public_identity(&manifest).unwrap();
    let mut d = o.d.clone();
    d.keyset_identity = manifest.identity.clone();
    let mut imported = ImportedPublicKeyset {
        manifest,
        source: f.source.as_bytes().to_vec(),
        action_bytes,
        snapshot_total: 0,
    };
    // Pure preflight returns source artifacts only; it mints no public VK owner.
    super::deployment::preflight(&f.source, &f.program, &d, &o.s.policy, &imported, &limits)
        .unwrap();
    let original = imported.action_bytes[1].0.clone();
    imported.action_bytes[1].0.push(0);
    assert_eq!(
        super::deployment::preflight(&f.source, &f.program, &d, &o.s.policy, &imported, &limits)
            .err()
            .unwrap()
            .code,
        "source-ir-parameter-metadata"
    );
    imported.action_bytes[1].0 = original;
    imported.action_bytes.pop();
    assert_eq!(
        super::deployment::preflight(&f.source, &f.program, &d, &o.s.policy, &imported, &limits)
            .err()
            .unwrap()
            .code,
        "action-set"
    );
    let mut legacy = d.clone();
    legacy.profile = crate::profile::INSTALLED_PROFILE.into();
    assert!(legacy.validate().is_err());
    let mut abi = d;
    abi.signed_abi = ABI.into();
    assert!(abi.validate().is_err());
}
#[test]
fn signed_public_artifact_reader_never_accesses_optional_prover_leaves() {
    use crate::call_artifacts::{ActionKeyMetadata, PublicKeysetManifest, leaf, public_identity};
    let root =
        std::env::temp_dir().join(format!("mori-signed-public-reader-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let f = metered(4, 7, None);
    let bindings = StaticBindings::legacy(&f.deployment);
    let signer = policy("bob");
    let statics = SignedStaticBindings::from_source(
        &f.source,
        &f.program,
        &bindings,
        &signer,
        SignedVersion::LegacyV1,
    )
    .unwrap();
    let schema = compile_signed(&statics, "settle")
        .unwrap()
        .manifest_bytes()
        .unwrap();
    // Only untrusted publication/import bytes. No fake derived-key owner.
    let mut leaves = BTreeMap::new();
    for (name, bytes) in [
        ("source.mori", f.source.as_bytes()),
        ("action-000000.zkir.bin", b"untrusted-ir".as_slice()),
        ("action-000000.manifest.json", schema.as_slice()),
        ("action-000000.vk.bin", b"untrusted-vk".as_slice()),
    ] {
        std::fs::write(root.join(name), bytes).unwrap();
        leaves.insert(name.into(), leaf(bytes));
    }
    let actions = vec![ActionKeyMetadata {
        name: "settle".into(),
        ordinal: 0,
        ir_hash: codec::hash(b"untrusted-ir"),
        manifest_hash: codec::hash(&schema),
        vk_hash: codec::hash(b"untrusted-vk"),
        degree: 9,
        srs_digest: "untrusted-metadata".into(),
        srs_bytes: 1,
    }];
    leaves.insert(
        "selected.pk.bin".into(),
        crate::call_artifacts::Leaf {
            sha256: "invalid-optional".into(),
            bytes: u64::MAX,
        },
    );
    leaves.insert(
        "proving-keyset.json".into(),
        crate::call_artifacts::Leaf {
            sha256: "invalid-optional".into(),
            bytes: u64::MAX,
        },
    );
    std::fs::create_dir(root.join("selected.pk.bin")).unwrap();
    let mut manifest = PublicKeysetManifest {
        profile: KEYSET.into(),
        signer_policy: Some(policy("bob")),
        bindings,
        build: "untrusted-build".into(),
        actions,
        identity: String::new(),
        leaves,
    };
    manifest.identity = public_identity(&manifest).unwrap();
    std::fs::write(
        root.join("public-keyset.json"),
        serde_json::to_vec(&manifest).unwrap(),
    )
    .unwrap();
    let limits = NativeWorkLimits {
        profile: "moriarty-native-call-work-limits/1".into(),
        max_degree: 19,
        max_actions: 4,
        max_srs_bytes: 1,
        max_prover_key_bytes: 1,
        max_verifier_key_bytes: 1024,
        max_input_bytes: 1048576,
        max_leaf_bytes: 1048576,
        max_artifact_bytes: 4194304,
        max_operations: 10000,
        max_public_inputs: 10000,
    };
    assert_eq!(
        snapshot_signed_public_keyset(&root, &limits, SignedVersion::LegacyV1)
            .unwrap()
            .action_count(),
        1
    );
    assert!(crate::call_artifacts::snapshot_public_keyset(&root, &limits).is_err());
    assert!(
        snapshot_signed_proving_keyset(&root, "settle", &limits, SignedVersion::LegacyV1).is_err()
    );
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn signed_work_limits_cannot_widen_existing_ordinary_envelopes() {
    let mut l = NativeWorkLimits {
        profile: "moriarty-native-call-work-limits/1".into(),
        max_degree: 19,
        max_actions: 4,
        max_srs_bytes: 1,
        max_prover_key_bytes: 1,
        max_verifier_key_bytes: 1024,
        max_input_bytes: 1048576,
        max_leaf_bytes: 16777216,
        max_artifact_bytes: 16777216,
        max_operations: 10000,
        max_public_inputs: 10000,
    };
    validate_work_limits(&l).unwrap();
    l.max_input_bytes += 1;
    assert_eq!(
        validate_work_limits(&l).err().unwrap().code,
        "ordinary-envelope"
    );
    l.max_input_bytes -= 1;
    l.max_leaf_bytes += 1;
    assert!(validate_work_limits(&l).is_err());
    l.max_leaf_bytes -= 1;
    l.max_artifact_bytes += 1;
    assert!(validate_work_limits(&l).is_err());
}
#[test]
fn typed_wire_projection_serializes_canonical_scalar_without_redundant_type_object() {
    let field = WireProjection {
        position: 0,
        descriptor: "input.amount".into(),
        ty: mori_core::Type::UInt,
        value: Value::UInt(UInt(7)),
    };
    assert_eq!(
        serde_json::to_value(&field).unwrap(),
        serde_json::json!({"position":0,"descriptor":"input.amount","type":"UInt","value":"7"})
    );
    let field = WireProjection {
        position: 1,
        descriptor: "pre.state.enabled".into(),
        ty: mori_core::Type::Bool,
        value: Value::Bool(true),
    };
    assert_eq!(serde_json::to_value(&field).unwrap()["value"], true);
    assert!(serde_json::from_str::<WireProjection>(r#"{"position":0,"descriptor":"input.amount","type":"UInt","value":{"type":"UInt","value":"7"}}"#).is_err());
}

fn consuming_fixture(mut f: support::Fixture, policy: &FixedSignerPolicy) -> support::Fixture {
    use midnight_onchain_state::state::{ChargedState, StateValue};
    let s = SignedStaticBindings::from_source(
        &f.source,
        &f.program,
        &StaticBindings::legacy(&f.deployment),
        policy,
        SignedVersion::ConsumingV2,
    )
    .unwrap();
    let mut ledger: layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
    let address = codec::from_hex(&f.deployment.address_hex).unwrap();
    let mut contract = ledger.index(address).unwrap();
    let StateValue::Array(root) = &*contract.data.get() else {
        panic!()
    };
    let authorization = layout::NativeMap::default().insert(
        nonce::domain_key(),
        crate::ops::cell(
            midnight_base_crypto::hash::HashOutput(nonce::nonce_domain(&s).unwrap()).into(),
        ),
    );
    contract.data = ChargedState::new(StateValue::Array(
        vec![
            root.get(0).unwrap().clone(),
            root.get(1).unwrap().clone(),
            root.get(2).unwrap().clone(),
            StateValue::Map(authorization),
        ]
        .into(),
    ));
    ledger.contract = ledger.contract.insert(address, contract);
    support::repin(&mut f, &ledger);
    f.deployment.profile = CONSUMING_SEED.into();
    f
}
#[test]
fn nonce_v2_empty_actions_consume_shared_nonce() {
    let source = "language \"moriarty/1\"; agreement NonceOnly { state active: Bool = true; action first() { ensures active; } action second() { ensures active; } }";
    let policy = policy("alice");
    let f = consuming_fixture(
        signed_parameters(support::fixture(source, "first", Scenario::default())),
        &policy,
    );
    admit_signed_local_seed(
        &f.source,
        &f.program,
        &f.deployment,
        &f.predecessor,
        &f.parameters,
        &policy,
    )
    .unwrap();
    let o = ordinary_version(&f, request(&f, [0; 4]), policy, SignedVersion::ConsumingV2);
    let b = raw(&f, &o).unwrap();
    assert_eq!(o.package.payload.storage_abi, CONSUMING_STORAGE_ABI);
    let root: midnight_onchain_state::state::StateValue<midnight_storage::db::InMemoryDB> =
        codec::decode_native(&codec::unhex(&b.readback.semantic_successor_hex).unwrap()).unwrap();
    let map = nonce::authority_map(&o.s, &root).unwrap();
    assert_eq!(map.size(), 2);
    assert_eq!(
        layout::cell(&map.get(&o.q.nonce.0.into()).unwrap()).unwrap(),
        true.into()
    );
    assert_eq!(b.readback.execution.effects.len(), 0);
}

fn nonce_limits() -> NativeWorkLimits {
    NativeWorkLimits {
        profile: "moriarty-native-call-work-limits/1".into(),
        max_degree: 19,
        max_actions: 4,
        max_srs_bytes: 128 * 1024 * 1024,
        max_prover_key_bytes: 1,
        max_verifier_key_bytes: 1024,
        max_input_bytes: 1048576,
        max_leaf_bytes: 16777216,
        max_artifact_bytes: 16777216,
        max_operations: 10000,
        max_public_inputs: 10000,
    }
}
fn nonce_only() -> (support::Fixture, FixedSignerPolicy) {
    let source = "language \"moriarty/1\"; agreement NonceOnly { state active: Bool = true; action first() { ensures active; } action second() { ensures active; } }";
    let policy = policy("alice");
    (
        consuming_fixture(
            signed_parameters(support::fixture(source, "first", Scenario::default())),
            &policy,
        ),
        policy,
    )
}
fn queried_supplied_fixture(
    f: &support::Fixture,
    o: &Ordinary,
    b: &prepare::PreparedBundle,
) -> support::Fixture {
    let mut successor = f.clone();
    let mut ledger: layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
    let address = codec::from_hex(&f.deployment.address_hex).unwrap();
    let mut contract = ledger.index(address).unwrap();
    contract.data =
        codec::decode_native(&codec::unhex(&b.readback.charged_successor_hex).unwrap()).unwrap();
    ledger.contract = ledger.contract.insert(address, contract);
    successor.predecessor = codec::encode(&ledger).unwrap();
    let d = deployment::derive_signed_supplied_frame_core(
        &o.s,
        &o.d,
        &successor.predecessor,
        &successor.parameters,
        &nonce_limits(),
        layout::OperationPolicy::Keyless,
    )
    .unwrap();
    successor.deployment.predecessor_hash = d.frame.predecessor_hash;
    successor.deployment.native_state_hash = d.frame.native_state_hash;
    successor
}
#[test]
fn nonce_v2_unordered_seven_then_zero_uses_production_path() {
    let (f, policy) = nonce_only();
    let mut q = request(&f, [0; 4]);
    q.nonce = UInt(7);
    let a = ordinary_version(&f, q, policy.clone(), SignedVersion::ConsumingV2);
    let result = raw(&f, &a).unwrap();
    let s = queried_supplied_fixture(&f, &a, &result);
    let mut q = request(&s, [0; 4]);
    q.nonce = UInt(0);
    q.action = "second".into();
    let b = ordinary_version(&s, q, policy, SignedVersion::ConsumingV2);
    let result = raw(&s, &b).unwrap();
    assert_eq!(
        result.readback.authority.as_ref().unwrap().pre_nonce_count,
        UInt(2)
    );
    assert_eq!(
        result.readback.authority.as_ref().unwrap().post_nonce_count,
        UInt(3)
    );
    let root = codec::from_hex(&result.readback.semantic_successor_hex).unwrap();
    let map = nonce::authority_map(&b.s, &root).unwrap();
    assert!(map.get(&7u128.into()).is_some() && map.get(&0u128.into()).is_some());
    assert_eq!(
        result.readback.execution.pre,
        result.readback.execution.post
    );
}
#[test]
fn nonce_v2_same_predecessor_two_packages_serialize_current_root() {
    use midnight_onchain_runtime::context::QueryContext;
    let (f, policy) = nonce_only();
    let mut q = request(&f, [0; 4]);
    q.nonce = UInt(7);
    let a = ordinary_version(&f, q, policy.clone(), SignedVersion::ConsumingV2);
    let a_result = raw(&f, &a).unwrap();
    let mut q = request(&f, [0; 4]);
    q.nonce = UInt(0);
    q.action = "second".into();
    let b = ordinary_version(&f, q.clone(), policy.clone(), SignedVersion::ConsumingV2);
    let b_result = raw(&f, &b).unwrap();
    let s = queried_supplied_fixture(&f, &a, &a_result);
    let fresh = ordinary_version(&s, q, policy, SignedVersion::ConsumingV2);
    assert_eq!(
        preparation::check_attachment_payload(
            &fresh.s,
            &fresh.package,
            &b.attachment,
            &nonce_limits()
        )
        .unwrap_err()
        .code,
        "attachment-payload"
    );
    let ledger: layout::NativeState = codec::decode_native(&s.predecessor).unwrap();
    let address = codec::from_hex(&s.deployment.address_hex).unwrap();
    let contract = ledger.index(address).unwrap();
    let mut context = QueryContext::new(contract.data, address);
    // Empty source actions isolate the authorization pre-Size opening as the
    // first differing native observation; call context is retained unchanged.
    context.call_context = b_result.native.context.call_context.clone();
    let call = b_result.native.intent.calls().next().unwrap();
    assert!(
        context
            .run_transcript(
                call.guaranteed_transcript.as_deref().unwrap(),
                &ledger.parameters.cost_model.runtime_cost_model
            )
            .is_err()
    );
    raw(&s, &fresh).unwrap();
}
#[test]
fn nonce_v2_authority_observation_is_mandatory() {
    let (f, policy) = nonce_only();
    let o = ordinary_version(&f, request(&f, [0; 4]), policy, SignedVersion::ConsumingV2);
    let frame = layout::admit_signed_frame(
        &o.s,
        &o.a.manifest.abi,
        &o.d.frame,
        codec::decode_native(&f.predecessor).unwrap(),
        layout::OperationPolicy::Keyless,
    )
    .unwrap();
    let pre = layout::attach_request(frame, &o.s.bindings, &o.q.native()).unwrap();
    let execution = mori_core::simulate(&f.program, &o.q.action, &pre.scenario).unwrap();
    assert_eq!(
        crate::ops::lower_signed_ops(&f.program, &pre, &execution, &o.s, None)
            .err()
            .unwrap()
            .code,
        "authority-observation"
    );
    assert_eq!(
        context::projection(&o.s, &execution, &o.q, &o.q.action, None)
            .unwrap_err()
            .code,
        "authority-readback"
    );
}

#[test]
fn nonce_v2_each_dynamic_key_is_constrained() {
    use midnight_onchain_vm::ops::Op;
    use midnight_transient_crypto::repr::FieldRepr;
    let (f, policy) = nonce_only();
    let o = ordinary_version(&f, request(&f, [0; 4]), policy, SignedVersion::ConsumingV2);
    let b = raw(&f, &o).unwrap();
    for origin in [
        "signed-authority-pre-member",
        "signed-authority-write",
        "signed-authority-post-member",
    ] {
        let index = b
            .native
            .lowered
            .bindings
            .iter()
            .find(|binding| {
                binding.origin == origin
                    && matches!(
                        b.native.lowered.program[binding.op as usize],
                        Op::Push { storage: false, .. }
                    )
            })
            .unwrap()
            .op as usize;
        let prefix: usize = b.native.lowered.program[..index]
            .iter()
            .map(FieldRepr::field_size)
            .sum();
        let mut altered = preimage(&b);
        altered.public_transcript_inputs
            [prefix + b.native.lowered.program[index].field_size() - 1] = altered
            .public_transcript_inputs[prefix + b.native.lowered.program[index].field_size() - 1]
            + Fr::from(1);
        recommit(&mut altered);
        assert_eq!(
            relation::check(&o.a, &altered).unwrap_err().code,
            "unsatisfied",
            "{origin}"
        );
        let mut instructions = (*o.a.ir.instructions).clone();
        let impact = instructions
            .iter_mut()
            .filter(|i| matches!(i, midnight_zkir_v3::Instruction::Impact { .. }))
            .nth(index)
            .unwrap();
        let midnight_zkir_v3::Instruction::Impact { inputs, .. } = impact else {
            panic!()
        };
        let last = inputs.len() - 1;
        inputs[last] = crate::relation::arithmetic::field(
            altered.public_transcript_inputs
                [prefix + b.native.lowered.program[index].field_size() - 1],
        );
        relation::check(&changed_artifact(&o.a, instructions), &altered).unwrap();
    }
    for origin in [
        "signed-authority-domain",
        "signed-authority-pre-count",
        "signed-authority-pre-member",
        "signed-authority-post-count",
        "signed-authority-post-member",
    ] {
        let index = b
            .native
            .lowered
            .bindings
            .iter()
            .find(|binding| {
                binding.origin == origin
                    && matches!(
                        b.native.lowered.program[binding.op as usize],
                        Op::Popeq { .. }
                    )
            })
            .unwrap()
            .op as usize;
        let prefix: usize = b.native.lowered.program[..index]
            .iter()
            .map(FieldRepr::field_size)
            .sum();
        let output_prefix: usize = b.native.lowered.program[..index]
            .iter()
            .filter_map(|op| {
                if let Op::Popeq { result, .. } = op {
                    let mut fields = vec![];
                    result.value_only_field_repr(&mut fields);
                    Some(fields.len())
                } else {
                    None
                }
            })
            .sum();
        let mut altered = preimage(&b);
        altered.public_transcript_inputs
            [prefix + b.native.lowered.program[index].field_size() - 1] = altered
            .public_transcript_inputs[prefix + b.native.lowered.program[index].field_size() - 1]
            + Fr::from(1);
        let Op::Popeq { result, .. } = &b.native.lowered.program[index] else {
            panic!()
        };
        let mut fields = vec![];
        result.value_only_field_repr(&mut fields);
        altered.public_transcript_outputs[output_prefix + fields.len() - 1] =
            altered.public_transcript_outputs[output_prefix + fields.len() - 1] + Fr::from(1);
        recommit(&mut altered);
        assert_eq!(
            relation::check(&o.a, &altered).unwrap_err().code,
            "unsatisfied",
            "{origin}"
        );
    }
}
#[test]
fn nonce_v2_frame_preserves_installed_policy_commitments() {
    let (f, policy) = nonce_only();
    let o = ordinary_version(&f, request(&f, [0; 4]), policy, SignedVersion::ConsumingV2);
    let b = raw(&f, &o).unwrap();
    let s = queried_supplied_fixture(&f, &o, &b);
    let mut altered = o.d.clone();
    altered.frame.operations_hash = "00".repeat(32);
    assert_eq!(
        deployment::derive_signed_supplied_frame_core(
            &o.s,
            &altered,
            &s.predecessor,
            &s.parameters,
            &nonce_limits(),
            layout::OperationPolicy::Keyless
        )
        .unwrap_err()
        .code,
        "contract-frame"
    );
    for d in [&o.d, &altered] {
        assert_eq!(d.bindings, o.s.bindings);
    }
    let mut legacy = o.d.clone();
    legacy.profile = PROFILE.into();
    legacy.storage_abi = ABI.into();
    legacy.signed_abi = SIGNED_ABI.into();
    assert_eq!(
        deployment::derive_signed_supplied_frame_core(
            &o.s,
            &legacy,
            &s.predecessor,
            &s.parameters,
            &nonce_limits(),
            layout::OperationPolicy::Keyless
        )
        .unwrap_err()
        .code,
        "supplied-frame-policy"
    );
}

fn edited_contract(
    f: &support::Fixture,
    edit: impl FnOnce(
        &mut midnight_onchain_state::state::ContractState<midnight_storage::db::InMemoryDB>,
    ),
) -> support::Fixture {
    let mut changed = f.clone();
    let mut ledger: layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
    let address = codec::from_hex(&f.deployment.address_hex).unwrap();
    let mut contract = ledger.index(address).unwrap();
    edit(&mut contract);
    ledger.contract = ledger.contract.insert(address, contract);
    changed.deployment.profile = crate::profile::PROFILE.into();
    support::repin(&mut changed, &ledger);
    changed.deployment.profile = f.deployment.profile.clone();
    changed
}
fn prepare_nonce_fixture(
    f: &support::Fixture,
    s: &SignedStaticBindings,
    q: &SignedCallRequest,
) -> Result<prepare::PreparedBundle> {
    let abi = layout::admit_bindings(&f.program, &s.bindings)?;
    let frame = layout::admit_signed_frame(
        s,
        &abi,
        &FrameBindings::legacy(&f.deployment),
        codec::decode_native(&f.predecessor)?,
        layout::OperationPolicy::Keyless,
    )?;
    let pre = layout::attach_request(frame, &s.bindings, &q.native())?;
    prepare::prepare_admitted_signed(
        &f.program,
        &s.bindings,
        &FrameBindings::legacy(&f.deployment),
        pre,
        codec::decode_native(&f.parameters)?,
        &q.native(),
        None,
        s.version.profile(),
        &codec::object_hash(&f.deployment)?,
        codec::OUTPUT_BYTES as u64,
        codec::OUTPUT_BYTES as u64,
        Some(s),
        None,
    )
}
#[test]
fn nonce_v2_authority_rejects_bad_sentinel_marker_and_root() {
    use midnight_onchain_state::state::StateValue;
    let (f, policy) = nonce_only();
    let s = SignedStaticBindings::from_source(
        &f.source,
        &f.program,
        &StaticBindings::legacy(&f.deployment),
        &policy,
        SignedVersion::ConsumingV2,
    )
    .unwrap();
    let ledger: layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
    let root = ledger
        .index(codec::from_hex(&f.deployment.address_hex).unwrap())
        .unwrap()
        .data
        .get();
    let StateValue::Array(a) = &*root else {
        panic!()
    };
    let maps = nonce::authority_map(&s, &root).unwrap();
    assert_eq!(
        nonce::admit_authority(
            &s,
            &StateValue::Array(
                vec![
                    a.get(0).unwrap().clone(),
                    a.get(1).unwrap().clone(),
                    a.get(2).unwrap().clone()
                ]
                .into()
            ),
            UInt(7)
        )
        .err()
        .unwrap()
        .code,
        "signed-storage-root"
    );
    assert!(layout::root_maps(&root).is_err());
    for (map, expected) in [
        (layout::NativeMap::default(), "authority-capacity"),
        (
            maps.insert(
                nonce::domain_key(),
                crate::ops::cell(midnight_base_crypto::hash::HashOutput([0; 32]).into()),
            ),
            "authority-domain",
        ),
        (
            maps.insert(9u128.into(), crate::ops::cell(false.into())),
            "authority-map",
        ),
        (
            maps.insert(
                midnight_base_crypto::hash::HashOutput([3; 32]).into(),
                crate::ops::cell(true.into()),
            ),
            "authority-map",
        ),
    ] {
        let altered = StateValue::Array(a.insert(3, StateValue::Map(map)).unwrap());
        assert_eq!(
            nonce::admit_authority(&s, &altered, UInt(7))
                .err()
                .unwrap()
                .code,
            expected
        );
    }
    let used = StateValue::Array(
        a.insert(
            3,
            StateValue::Map(maps.insert(7u128.into(), crate::ops::cell(true.into()))),
        )
        .unwrap(),
    );
    assert_eq!(
        nonce::admit_authority(&s, &used, UInt(7))
            .err()
            .unwrap()
            .code,
        "nonce-used"
    );
    assert_eq!(
        crate::local_deployment::admit_local_seed(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters
        )
        .err()
        .unwrap()
        .code,
        "local-profile"
    );
}
#[test]
fn nonce_v2_seed_requires_all_keyless_actions() {
    let (f, policy) = nonce_only();
    let changed = edited_contract(&f, |contract| {
        contract.operations = contract.operations.insert(
            b"unknown".as_slice().into(),
            midnight_onchain_state::state::ContractOperation::new(None, None),
        );
    });
    assert_eq!(
        admit_signed_local_seed(
            &changed.source,
            &changed.program,
            &changed.deployment,
            &changed.predecessor,
            &changed.parameters,
            &policy
        )
        .err()
        .unwrap()
        .code,
        "operation-set"
    );
}
#[test]
fn nonce_v2_capacity_and_successor_byte_guards_use_actual_preparation() {
    use midnight_onchain_state::state::StateValue;
    let (f, policy) = nonce_only();
    let s = SignedStaticBindings::from_source(
        &f.source,
        &f.program,
        &StaticBindings::legacy(&f.deployment),
        &policy,
        SignedVersion::ConsumingV2,
    )
    .unwrap();
    let filled = edited_contract(&f, |contract| {
        let StateValue::Array(root) = &*contract.data.get() else {
            panic!()
        };
        let mut map = nonce::authority_map(&s, &StateValue::Array(root.clone()))
            .unwrap()
            .clone();
        for n in 0..12151u128 {
            map = map.insert(n.into(), crate::ops::cell(true.into()));
        }
        contract.data = midnight_onchain_state::state::ChargedState::new(StateValue::Array(
            root.insert(3, StateValue::Map(map)).unwrap(),
        ));
    });
    let mut q = request(&filled, [0; 4]);
    q.nonce = UInt(u128::MAX);
    let b = prepare_nonce_fixture(&filled, &s, &q).unwrap();
    assert_eq!(
        b.readback.authority.as_ref().unwrap().post_nonce_count,
        UInt(12153)
    );
    let full = edited_contract(&filled, |contract| {
        contract.data =
            codec::decode_native(&codec::unhex(&b.readback.charged_successor_hex).unwrap())
                .unwrap();
    });
    q.nonce = UInt(u128::MAX - 1);
    assert_eq!(
        prepare_nonce_fixture(&full, &s, &q).err().unwrap().code,
        "authority-capacity"
    );
    // Actual queried successors exercise both serialized production guard objects.
    let small = prepare_nonce_fixture(&f, &s, &request(&f, [0; 4])).unwrap();
    let charged: midnight_onchain_state::state::ChargedState<midnight_storage::db::InMemoryDB> =
        codec::decode_native(&codec::unhex(&small.readback.charged_successor_hex).unwrap())
            .unwrap();
    let ledger: layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
    let charged_size = codec::encode(&charged).unwrap().len() as u64;
    assert_eq!(
        nonce::check_successor_input(&s, &ledger, &charged, charged_size - 1)
            .unwrap_err()
            .code,
        "successor-input-size"
    );
    // Charged serialization fits at its exact boundary; the complete ledger
    // envelope still refuses under that same cap.
    assert_eq!(
        nonce::check_successor_input(&s, &ledger, &charged, charged_size)
            .unwrap_err()
            .code,
        "successor-input-size"
    );
    nonce::check_successor_input(&s, &ledger, &charged, codec::INPUT_BYTES as u64).unwrap();
}
#[test]
fn nonce_v2_financial_effects_and_extensions_are_complete() {
    let f = consuming_fixture(metered(4, 7, None), &policy("bob"));
    let o = ordinary_version(
        &f,
        request(&f, [16, 2, 12, 4]),
        policy("bob"),
        SignedVersion::ConsumingV2,
    );
    let b = raw(&f, &o).unwrap();
    assert_eq!(b.readback.execution.effects.len(), 8);
    assert_eq!(
        b.readback.execution.post.state["running"],
        Value::UInt(UInt(11))
    );
    assert_eq!(b.readback.execution.post.liabilities[0].amount, UInt(4));
    assert_eq!(
        b.readback.authority.as_ref().unwrap().post_nonce_count,
        UInt(2)
    );
    assert_eq!(
        b.readback.preserved_frame.commitments["contract_extensions"],
        codec::native_hash(&layout::NativeMap::default()).unwrap()
    );
    assert_eq!(b.readback.evidence.ledger_apply, "not-performed");
    assert_eq!(
        b.readback.evidence.consumption,
        "queried-supplied-state-nonce-consumed-unqualified"
    );
}
#[test]
fn nonce_v2_default_partition_remains_refused() {
    let mut f = metered(4, 7, None);
    let mut ledger: layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
    ledger.parameters = layout::NativeState::new("local").parameters;
    support::repin(&mut f, &ledger);
    let policy = policy("bob");
    let f = consuming_fixture(f, &policy);
    let s = SignedStaticBindings::from_source(
        &f.source,
        &f.program,
        &StaticBindings::legacy(&f.deployment),
        &policy,
        SignedVersion::ConsumingV2,
    )
    .unwrap();
    assert_eq!(
        prepare_nonce_fixture(&f, &s, &request(&f, [16, 2, 12, 4]))
            .err()
            .unwrap()
            .code,
        "all-fallible"
    );
}
#[test]
fn nonce_v2_full_frame_mutations_refuse() {
    let (f, policy) = nonce_only();
    let o = ordinary_version(&f, request(&f, [0; 4]), policy, SignedVersion::ConsumingV2);
    for which in 0..3 {
        let changed = edited_contract(&f, |contract| match which {
            0 => {
                contract.operations = contract.operations.insert(
                    b"unknown".as_slice().into(),
                    midnight_onchain_state::state::ContractOperation::new(None, None),
                )
            }
            1 => contract.maintenance_authority.counter += 1,
            _ => {
                contract.balance = contract.balance.insert(
                    midnight_coin_structure::coin::TokenType::Unshielded(
                        midnight_coin_structure::coin::UnshieldedTokenType(
                            midnight_base_crypto::hash::HashOutput([8; 32]),
                        ),
                    ),
                    1,
                )
            }
        });
        assert_eq!(
            deployment::derive_signed_supplied_frame_core(
                &o.s,
                &o.d,
                &changed.predecessor,
                &changed.parameters,
                &nonce_limits(),
                layout::OperationPolicy::Keyless
            )
            .unwrap_err()
            .code,
            "contract-frame",
            "{which}"
        );
    }
}
#[test]
fn nonce_v2_cross_address_host_and_ir_refuse() {
    let (f, policy) = nonce_only();
    let o = ordinary_version(
        &f,
        request(&f, [0; 4]),
        policy.clone(),
        SignedVersion::ConsumingV2,
    );
    let b = raw(&f, &o).unwrap();
    let mut binding = o.s.bindings.clone();
    binding.address_hex = codec::native_hex(&midnight_coin_structure::contract::ContractAddress(
        midnight_base_crypto::hash::HashOutput([9; 32]),
    ))
    .unwrap();
    let alternate = SignedStaticBindings::from_source(
        &f.source,
        &f.program,
        &binding,
        &policy,
        SignedVersion::ConsumingV2,
    )
    .unwrap();
    assert_eq!(
        nonce::nonce_domain(&o.s).unwrap(),
        nonce::nonce_domain(&alternate).unwrap()
    );
    let a = compile_signed(&alternate, &o.q.action).unwrap();
    assert_ne!(
        o.a.manifest.signed.as_ref().unwrap().context_sha256,
        a.manifest.signed.as_ref().unwrap().context_sha256
    );
    assert_ne!(o.a.ir_bytes().unwrap(), a.ir_bytes().unwrap());
    let mut original = preimage(&b);
    recommit(&mut original);
    assert_eq!(
        relation::check(&a, &original).unwrap_err().code,
        "unsatisfied"
    );
    let mut foreign = o.package.clone();
    foreign.payload.address_hex = binding.address_hex;
    assert_eq!(
        preparation::check_attachment_payload(&alternate, &foreign, &o.attachment, &nonce_limits())
            .unwrap_err()
            .code,
        "attachment-payload"
    );
}

#[test]
fn nonce_v2_false_predicate_with_authority_write_refuses() {
    use midnight_onchain_vm::ops::Op;
    use midnight_transient_crypto::repr::FieldRepr;
    let (f, policy) = nonce_only();
    let o = ordinary_version(&f, request(&f, [0; 4]), policy, SignedVersion::ConsumingV2);
    let b = raw(&f, &o).unwrap();
    assert!(
        b.native
            .lowered
            .bindings
            .iter()
            .any(|b| b.origin == "signed-authority-write")
    );
    let false_state = edited_contract(&f, |contract| {
        let midnight_onchain_state::state::StateValue::Array(root) = &*contract.data.get() else {
            panic!()
        };
        let map = layout::map(root.get(0).unwrap()).unwrap().insert(
            layout::name_key("state", "active"),
            crate::ops::cell(false.into()),
        );
        contract.data = midnight_onchain_state::state::ChargedState::new(
            midnight_onchain_state::state::StateValue::Array(
                root.insert(0, midnight_onchain_state::state::StateValue::Map(map))
                    .unwrap(),
            ),
        );
    });
    // The actual original source `ensures active` refuses before querying.
    assert_eq!(
        prepare_nonce_fixture(&false_state, &o.s, &o.q)
            .err()
            .unwrap()
            .code,
        "core-refusal"
    );
    let mut altered = preimage(&b);
    let mut program = b.native.lowered.program.clone();
    let mut source_operand = None;
    for binding in &b.native.lowered.bindings {
        let index = binding.op as usize;
        if !["signed-state-opening", "source-expression-prefix-state"]
            .contains(&binding.origin.as_str())
            || !matches!(program[index], Op::Popeq { .. })
        {
            continue;
        }
        let input_prefix: usize = program[..index].iter().map(FieldRepr::field_size).sum();
        let output_prefix: usize = program[..index]
            .iter()
            .filter_map(|op| {
                if let Op::Popeq { result, .. } = op {
                    let mut fields = vec![];
                    result.value_only_field_repr(&mut fields);
                    Some(fields.len())
                } else {
                    None
                }
            })
            .sum();
        altered.public_transcript_inputs[input_prefix + program[index].field_size() - 1] =
            Fr::from(0);
        altered.public_transcript_outputs[output_prefix] = Fr::from(0);
        program[index] = Op::Popeq {
            cached: false,
            result: false.into(),
        };
        if binding.origin == "source-expression-prefix-state" {
            let impact =
                o.a.ir
                    .instructions
                    .iter()
                    .filter(|i| matches!(i, midnight_zkir_v3::Instruction::Impact { .. }))
                    .nth(index)
                    .unwrap();
            let midnight_zkir_v3::Instruction::Impact { inputs, .. } = impact else {
                panic!()
            };
            source_operand = inputs.last().cloned();
        }
    }
    assert!(
        b.native
            .context
            .query(
                &program,
                Some(b.native.gas_cap),
                &b.native.params.cost_model.runtime_cost_model
            )
            .is_err()
    );
    // Re-sign matching false semantic wires with the public QA scalar. Nonce
    // groups and typed nonce remain honest; only original source truth differs.
    let mut package = o.package.clone();
    for wire in &mut package.payload.wire_projection {
        if wire.descriptor == "pre.state.active" || wire.descriptor == "post.state.active" {
            wire.value = Value::Bool(false);
        }
    }
    package.payload.wire_projection_sha256 =
        codec::object_hash(&package.payload.wire_projection).unwrap();
    package.payload_sha256 = codec::object_hash(&package.payload).unwrap();
    package.message_hex = schnorr::native_hex(message(&package.payload).unwrap());
    let attachment = SignedCallAttachment {
        profile: ATTACHMENT.into(),
        payload: package.payload.clone(),
        signature_hex: sign(message(&package.payload).unwrap(), 7, 11),
    };
    let input = preparation::typed_input(&o.s, &o.q, &package, &attachment).unwrap();
    altered.inputs.clear();
    input.value_only_field_repr(&mut altered.inputs);
    recommit(&mut altered);
    assert_eq!(
        relation::check(&o.a, &altered).unwrap_err().code,
        "unsatisfied"
    );
    // Diagnostic only: remove the one actual source assertion. The correlated
    // counterfeit passes this weaker relation, isolating the source predicate.
    let mut instructions = (*o.a.ir.instructions).clone();
    let mut changed = 0;
    for instruction in &mut instructions {
        if let midnight_zkir_v3::Instruction::Assert { cond } = instruction {
            if Some(cond.clone()) == source_operand {
                *cond = crate::relation::arithmetic::imm(1);
                changed += 1;
            }
        }
    }
    assert_eq!(changed, 1);
    relation::check(&changed_artifact(&o.a, instructions), &altered).unwrap();
}

#[test]
fn nonce_v2_whole_successor_and_sentinel_drift_refuse() {
    use midnight_onchain_state::state::StateValue;
    use midnight_onchain_vm::ops::{Key, Op};
    let (f, policy) = nonce_only();
    let mut q = request(&f, [0; 4]);
    q.nonce = UInt(7);
    let a = ordinary_version(&f, q, policy.clone(), SignedVersion::ConsumingV2);
    let result = raw(&f, &a).unwrap();
    let s = queried_supplied_fixture(&f, &a, &result);
    let mut q = request(&s, [0; 4]);
    q.nonce = UInt(0);
    let b = ordinary_version(&s, q, policy, SignedVersion::ConsumingV2);
    let result = raw(&s, &b).unwrap();
    let observation =
        nonce::admit_authority(&b.s, &result.native.context.state.get(), UInt(0)).unwrap();
    let root: StateValue<midnight_storage::db::InMemoryDB> =
        codec::from_hex(&result.readback.semantic_successor_hex).unwrap();
    nonce::NonceAuthorityReadback::checked(&b.s, &observation, &root).unwrap();
    let StateValue::Array(array) = &root else {
        panic!()
    };
    let map = nonce::authority_map(&b.s, &root).unwrap();
    for (map, expected) in [
        (map.remove(&0u128.into()), "authority-successor"),
        (
            map.insert(9u128.into(), crate::ops::cell(true.into())),
            "authority-successor",
        ),
        (map.remove(&7u128.into()), "authority-successor"),
        (
            map.remove(&nonce::domain_key()).insert(
                midnight_base_crypto::hash::HashOutput([3; 32]).into(),
                crate::ops::cell(
                    midnight_base_crypto::hash::HashOutput(nonce::nonce_domain(&b.s).unwrap())
                        .into(),
                ),
            ),
            "authority-domain",
        ),
    ] {
        let altered = StateValue::Array(array.insert(3, StateValue::Map(map)).unwrap());
        assert_eq!(
            nonce::NonceAuthorityReadback::checked(&b.s, &observation, &altered)
                .unwrap_err()
                .code,
            expected
        );
    }
    let program = &result.native.lowered.program;
    let bindings = &result.native.lowered.bindings;
    let without_write = program
        .iter()
        .zip(bindings)
        .filter(|(_, binding)| binding.origin != "signed-authority-write")
        .map(|(op, _)| op.clone())
        .collect::<Vec<_>>();
    assert!(
        result
            .native
            .context
            .query(
                &without_write,
                Some(result.native.gas_cap),
                &result.native.params.cost_model.runtime_cost_model
            )
            .is_err()
    );
    let post_count = bindings
        .iter()
        .find(|binding| binding.origin == "signed-authority-post-count")
        .unwrap()
        .op as usize;
    let mut extra = program.clone();
    extra.splice(
        post_count..post_count,
        crate::ops::write_cell(3, 9u128.into(), true.into()),
    );
    assert!(
        result
            .native
            .context
            .query(
                &extra,
                Some(result.native.gas_cap),
                &result.native.params.cost_model.runtime_cost_model
            )
            .is_err()
    );
    let domain_index = bindings
        .iter()
        .find(|binding| {
            binding.origin == "signed-authority-domain"
                && matches!(program[binding.op as usize], Op::Idx { .. })
        })
        .unwrap()
        .op as usize;
    let mut drift = program.clone();
    drift[domain_index] = Op::Idx {
        cached: false,
        push_path: false,
        path: vec![
            Key::Value(3u8.into()),
            Key::Value(midnight_base_crypto::hash::HashOutput([3; 32]).into()),
        ]
        .into(),
    };
    assert!(
        result
            .native
            .context
            .query(
                &drift,
                Some(result.native.gas_cap),
                &result.native.params.cost_model.runtime_cost_model
            )
            .is_err()
    );
}
