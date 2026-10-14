//! Source/static compilation and operation shape only: no native query or check.
use midnight_coin_structure::coin::{UnshieldedTokenType, UserAddress};
use midnight_ledger::structure::SignatureVerifyingKey;
use mori_core::{Program, Scenario, UInt, Value};
use mori_ledger::{codec, layout, profile::*, relation};

const HEADER: &str = r#"language "moriarty/1"; agreement Reads {
asset A = { network: "local", issuer: "i", reference: "A", scale: 0 };
state x: UInt = 0;
action declare() { liability("d", A, "alice", "bob", 0); }
"#;

fn deployment(source: &str, p: &Program) -> Deployment {
    let public_points = [
        "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        "02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9",
        "02e493dbf1c10d80f3581e4904930b1404cc6c13900ee0758474fa94abe8c4cd13",
        "022f8bde4d1a07209355b4a7250a5c5128e88b84bddc619ab7cba8d569b240efe4",
    ];
    let mut names = std::collections::BTreeSet::from(["alice".to_owned()]);
    for statement in p.actions.iter().flat_map(|a| &a.statements) {
        match statement {
            mori_core::Statement::Transfer { from, to, .. }
            | mori_core::Statement::Fee { from, to, .. } => {
                names.insert(from.clone());
                names.insert(to.clone());
            }
            mori_core::Statement::Liability {
                debtor, creditor, ..
            } => {
                names.insert(debtor.clone());
                names.insert(creditor.clone());
            }
            _ => {}
        }
    }
    assert!(names.len() <= public_points.len());
    let roles = names.into_iter().zip(public_points).map(|(role, point)| {
        let bytes = codec::unhex(point).unwrap();
        let key = <midnight_base_crypto::ecdsa::VerifyingKey as midnight_serialize::Deserializable>::deserialize(&mut bytes.as_slice(), 0).unwrap();
        let key = SignatureVerifyingKey::ECDSA(key);
        RoleBinding { role: role.into(), public_key_hex: codec::native_hex(&key).unwrap(), address_hex: codec::native_hex(&UserAddress::from(key)).unwrap() }
    }).collect();
    let mut d = Deployment {
        profile: PROFILE.into(),
        abi: ABI.into(),
        qualification: "local-unproven".into(),
        phase_policy: "guaranteed-only".into(),
        source_hash: codec::hash(source.as_bytes()),
        program_hash: codec::object_hash(p).unwrap(),
        abi_hash: String::new(),
        templates_hash: layout::templates_hash(),
        toolchain: Toolchain::pinned(),
        limits: Limits::supported(),
        network: "local".into(),
        anchor: "local-unqualified".into(),
        address_hex: String::new(),
        payer: "alice".into(),
        predecessor_hash: String::new(),
        native_state_hash: String::new(),
        parameters_hash: String::new(),
        gas_hex: String::new(),
        operations_hash: String::new(),
        maintenance_hash: String::new(),
        balance_hash: String::new(),
        assets: p
            .assets
            .iter()
            .map(|a| AssetBinding {
                identity: a.identity.clone(),
                token_hex: codec::native_hex(&UnshieldedTokenType(layout::asset_digest(
                    &a.identity,
                )))
                .unwrap(),
            })
            .collect(),
        roles,
        duties: p
            .actions
            .iter()
            .flat_map(|a| &a.statements)
            .filter_map(|s| match s {
                mori_core::Statement::Liability {
                    name,
                    asset,
                    debtor,
                    creditor,
                    ..
                } => Some((
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
                )),
                _ => None,
            })
            .collect::<std::collections::BTreeMap<_, _>>()
            .into_values()
            .collect(),
        economic_bounds: None,
    };
    d.abi_hash = codec::object_hash(
        &layout::admit_source(p, &d).expect("mapped source duty reads must admit"),
    )
    .unwrap();
    d
}
fn source(body: &str) -> String {
    format!("{HEADER} action run() {{ {body} }} }}")
}

#[test]
fn mapped_duty_leaves_compile_through_public_source_consumer() {
    let src = source(
        r#"requires not liability_present("d"); next x = liability_amount("d"); emit "amount"(x);"#,
    );
    let p = mori_core::parse(&src).unwrap();
    let d = deployment(&src, &p);
    let a = relation::compile(&src, &p, &d, "run").unwrap();
    assert_eq!(a.manifest.qualification, "ordinary-ir-unproven");
    assert_eq!(
        a.manifest
            .groups
            .iter()
            .filter(|g| g.origin == "duty-membership")
            .count(),
        10
    );
    assert_eq!(
        a.manifest
            .groups
            .iter()
            .filter(|g| g.origin == "duty-old-read")
            .count(),
        6
    );
    let e = mori_core::simulate(&p, "run", &Scenario::default()).unwrap();
    assert_eq!(e.post.state["x"], Value::UInt(UInt(0)));
}

// Fabricated public in-memory root used only to construct/serialize operations.
// This bypasses predecessor admission deliberately; it is no ledger qualification.
fn producer(
    p: &Program,
    d: &Deployment,
    scenario: &Scenario,
    e: &mori_core::Execution,
) -> mori_ledger::ops::LoweredOps {
    use midnight_base_crypto::{hash::HashOutput, time::Timestamp};
    use midnight_coin_structure::contract::ContractAddress;
    use midnight_ledger::structure::LedgerState;
    use midnight_onchain_runtime::context::CallContext;
    use midnight_onchain_state::state::{ContractState, StateValue};
    use midnight_storage::db::InMemoryDB;
    let state = p
        .states
        .iter()
        .map(|s| {
            (
                layout::name_key("state", &s.name),
                mori_ledger::ops::cell(layout::value(
                    scenario.state.get(&s.name).unwrap_or(&s.initial),
                )),
            )
        })
        .collect();
    let duties = scenario
        .liabilities
        .iter()
        .map(|l| {
            let binding = d.duties.iter().find(|b| b.name == l.name).unwrap();
            (
                layout::name_key("duty", &l.name),
                mori_ledger::ops::cell(layout::duty_value(binding, l.amount.0, &d.roles).unwrap()),
            )
        })
        .collect();
    let root = StateValue::Array(
        vec![
            StateValue::Map(state),
            StateValue::Map(duties),
            StateValue::Map(Default::default()),
        ]
        .into(),
    );
    let address = ContractAddress(HashOutput([7; 32]));
    let pre = layout::AdmittedPredecessor {
        ledger: LedgerState::<InMemoryDB>::new("local"),
        contract: ContractState::new(root, Default::default(), Default::default()),
        address,
        context: CallContext {
            own_address: address,
            tblock: Timestamp::from_secs(0),
            tblock_err: 0,
            parent_block_hash: HashOutput([0; 32]),
            caller: None,
            balance: Default::default(),
            com_indices: Default::default(),
            last_block_time: Timestamp::from_secs(0),
        },
        scenario: scenario.clone(),
        roles: d
            .roles
            .iter()
            .map(|r| {
                (
                    r.role.clone(),
                    layout::NativeRole {
                        key: codec::from_hex(&r.public_key_hex).unwrap(),
                        address: codec::from_hex(&r.address_hex).unwrap(),
                    },
                )
            })
            .collect(),
        assets: d
            .assets
            .iter()
            .map(|a| (a.identity.clone(), codec::from_hex(&a.token_hex).unwrap()))
            .collect(),
        duties: d
            .duties
            .iter()
            .map(|b| (b.name.clone(), b.clone()))
            .collect(),
        all_utxos: vec![],
        selected: vec![],
        gas: Default::default(),
    };
    mori_ledger::ops::lower_ops(p, &pre, e).unwrap()
}
fn scenario(p: &Program, amount: Option<u128>) -> Scenario {
    Scenario {
        liabilities: amount
            .map(|n| mori_core::Liability {
                name: "d".into(),
                asset: p.assets[0].identity.clone(),
                debtor: "alice".into(),
                creditor: "bob".into(),
                amount: UInt(n),
            })
            .into_iter()
            .collect(),
        ..Default::default()
    }
}
#[test]
fn missing_zero_and_wide_reads_have_matching_full_cell_shapes() {
    use midnight_onchain_vm::ops::Op;
    use midnight_transient_crypto::repr::FieldRepr;
    let src = source(r#"let present = liability_present("d"); next x = liability_amount("d");"#);
    let p = mori_core::parse(&src).unwrap();
    let d = deployment(&src, &p);
    let a = relation::compile(&src, &p, &d, "run").unwrap();
    let reference_ir = a.ir_bytes().unwrap();
    let reference_manifest = a.manifest_bytes().unwrap();
    for n in [
        None,
        Some(0),
        Some(7),
        Some(1 << 64),
        Some(1 << 127),
        Some(u128::MAX),
    ] {
        let s = scenario(&p, n);
        let e = mori_core::simulate(&p, "run", &s).unwrap();
        mori_core::replay(&p, "run", &s, &e).unwrap();
        assert_eq!(e.post.state["x"], Value::UInt(UInt(n.unwrap_or(0))));
        let ops = producer(&p, &d, &s, &e);
        let mut expected = vec![];
        expected.extend(mori_ledger::ops::abi_guard());
        for _ in 0..2 {
            expected.extend(mori_ledger::ops::membership(
                1,
                layout::name_key("duty", "d"),
                n.is_some(),
            ));
            if let Some(n) = n {
                expected.extend(mori_ledger::ops::read_cell(
                    1,
                    layout::name_key("duty", "d"),
                    layout::duty_value(&d.duties[0], n, &d.roles).unwrap(),
                ));
            }
        }
        expected.extend(mori_ledger::ops::read_cell(
            0,
            layout::name_key("state", "x"),
            0u128.into(),
        ));
        expected.extend(mori_ledger::ops::write_cell(
            0,
            layout::name_key("state", "x"),
            n.unwrap_or(0).into(),
        ));
        assert_eq!(
            codec::encode(&ops.program).unwrap(),
            codec::encode(&expected).unwrap()
        );
        let groups = a
            .manifest
            .groups
            .iter()
            .filter(|g| n.is_some() || g.origin != "duty-old-read")
            .collect::<Vec<_>>();
        assert_eq!(groups.len(), ops.program.len());
        assert!(
            groups
                .iter()
                .zip(&ops.program)
                .all(|(g, op)| g.field_count == op.field_size())
        );
        let members = ops
            .program
            .iter()
            .filter_map(|op| match op {
                Op::Popeq { result, .. } if *result == true.into() || *result == false.into() => {
                    Some(result)
                }
                _ => None,
            })
            .count();
        assert_eq!(members, 2);
        // Predecessor/input fields are not part of source-static compilation.
        let mut irrelevant = d.clone();
        irrelevant.predecessor_hash = format!("{n:?}");
        irrelevant.native_state_hash = "untrusted".into();
        let again = relation::compile(&src, &p, &irrelevant, "run").unwrap();
        assert_eq!(reference_ir, again.ir_bytes().unwrap());
        assert_eq!(reference_manifest, again.manifest_bytes().unwrap());
    }
}

#[test]
fn ordered_capture_and_self_read_mutation_operands_precede_effects() {
    use midnight_onchain_vm::ops::Op;
    let src = source(
        r#"requires liability_amount("d") == 0;
        liability("d", A, "alice", "bob", 7);
        let saved = liability_amount("d"); discharge("d", 3);
        next x = liability_amount("d");
        ensures saved == 7 and x == 4 and liability_amount("d") == 4;
        discharge("d", liability_amount("d"));
        ensures liability_present("d") and liability_amount("d") == 0;"#,
    );
    let p = mori_core::parse(&src).unwrap();
    let d = deployment(&src, &p);
    let a = relation::compile(&src, &p, &d, "run").unwrap();
    let s = Scenario::default();
    let e = mori_core::simulate(&p, "run", &s).unwrap();
    mori_core::replay(&p, "run", &s, &e).unwrap();
    assert_eq!(e.post.state["x"], Value::UInt(UInt(4)));
    assert_eq!(e.post.liabilities[0].amount, UInt(0));
    let out = producer(&p, &d, &s, &e);
    // Initial requires and accrue endpoint have absent cell reads, every later
    // opening reads the live amount. The captured let never reopens at ensures.
    let amounts = out
        .program
        .iter()
        .zip(&out.bindings)
        .filter_map(|(op, b)| {
            if matches!(b.origin.as_str(), "duty-old-read" | "duty-before") {
                if let Op::Popeq { result, .. } = op {
                    return Some(u128::try_from(&result.value[3..]).unwrap());
                }
            }
            None
        })
        .collect::<Vec<_>>();
    assert_eq!(amounts, vec![7, 7, 4, 4, 4, 4, 0, 0]);
    let step = out
        .bindings
        .iter()
        .filter(|b| b.step == Some(6))
        .map(|b| b.origin.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        step,
        [
            vec!["duty-membership"; 5],
            vec!["duty-old-read"; 3],
            vec!["duty-membership"; 5],
            vec!["duty-before"; 3],
            vec!["duty-after"; 4]
        ]
        .concat()
    );
    // Both compiler and producer record exactly seven expression observations
    // plus three mutation observations, including duplicate leaf IDs at end.
    assert_eq!(
        a.manifest
            .groups
            .iter()
            .filter(|g| g.origin == "duty-membership")
            .count(),
        50
    );
    assert_eq!(
        out.bindings
            .iter()
            .filter(|b| b.origin == "duty-membership")
            .count(),
        50
    );
}

#[test]
fn hostile_static_mapping_and_limits_remain_refused() {
    let src = source(r#"let value = liability_amount("d");"#);
    let p = mori_core::parse(&src).unwrap();
    let d = deployment(&src, &p);
    for (kind, expected) in [
        ("phase", "local-profile"),
        ("missing", "source-duty"),
        ("creditor", "source-duty-identity"),
        ("debtor", "source-duty-identity"),
        ("duplicate", "duty-mapping"),
        ("asset", "duty-mapping"),
        ("limits", "toolchain-or-limits"),
        ("templates", "static-abi-identity"),
        ("abi", "local-profile"),
    ] {
        let mut bad = d.clone();
        match kind {
            "phase" => bad.phase_policy = "fallible".into(),
            "missing" => bad.duties.clear(),
            "creditor" => bad.duties[0].creditor = "alice".into(),
            "debtor" => bad.duties[0].debtor = "bob".into(),
            "duplicate" => bad.duties.push(bad.duties[0].clone()),
            "asset" => bad.duties[0].asset.reference = "wrong".into(),
            "limits" => bad.limits.ops += 1,
            "templates" => bad.templates_hash = "stale".into(),
            "abi" => bad.abi = "different".into(),
            _ => unreachable!(),
        }
        let err = relation::compile(&src, &p, &bad, "run")
            .err()
            .expect("hostile mapping must refuse");
        assert_eq!(err.code, expected, "{kind}");
    }
}

#[test]
fn eager_two_duty_dfs_follows_sorted_state_openings_and_keeps_leaf_multiplicity() {
    use midnight_transient_crypto::repr::FieldRepr;
    use midnight_zkir_v3::{Instruction, ir::Operand};
    let src = source(r#"let discarded = (liability_amount("d") + z) + (liability_amount("a") + x);
        requires (liability_present("d") or true) and (liability_present("a") or true);
        requires liability_amount("d") == liability_amount("d");"#).replace("state x: UInt = 0;", "state x: UInt = 0; state z: UInt = 0; action declare_second() { liability(\"a\", A, \"alice\", \"bob\", 0); }");
    let p = mori_core::parse(&src).unwrap();
    let d = deployment(&src, &p);
    let a = relation::compile(&src, &p, &d, "run").unwrap();
    let s = Scenario::default();
    let e = mori_core::simulate(&p, "run", &s).unwrap();
    let out = producer(&p, &d, &s, &e);
    assert_eq!(out.bindings[3].origin, "source-expression-prefix-state");
    assert_eq!(out.bindings[6].origin, "source-expression-prefix-state");
    let mut state_reads =
        mori_ledger::ops::read_cell(0, layout::name_key("state", "x"), 0u128.into());
    state_reads.extend(mori_ledger::ops::read_cell(
        0,
        layout::name_key("state", "z"),
        0u128.into(),
    ));
    assert_eq!(
        codec::encode(&out.program[3..9].to_vec()).unwrap(),
        codec::encode(&state_reads).unwrap()
    );
    let duty_pushes = out
        .program
        .iter()
        .zip(&out.bindings)
        .filter(|(_, b)| b.origin == "duty-membership")
        .filter_map(|(op, _)| match op {
            midnight_onchain_vm::ops::Op::Push {
                value: midnight_onchain_state::state::StateValue::Cell(v),
                ..
            } => Some((**v).clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        duty_pushes,
        ["d", "a", "d", "a", "d", "d"].map(|n| layout::name_key("duty", n))
    );
    let impacts =
        a.ir.instructions
            .iter()
            .filter_map(|i| match i {
                Instruction::Impact { inputs, .. } => Some(inputs),
                _ => None,
            })
            .zip(&a.manifest.groups)
            .filter(|(_, g)| g.origin == "duty-membership")
            .collect::<Vec<_>>();
    assert_eq!(impacts.len(), duty_pushes.len() * 5);
    let produced = out
        .program
        .iter()
        .zip(&out.bindings)
        .filter(|(_, b)| b.origin == "duty-membership")
        .map(|(op, _)| op.field_vec())
        .collect::<Vec<_>>();
    for (i, ((inputs, _), actual)) in impacts.iter().zip(&produced).enumerate() {
        assert_eq!(inputs.len(), actual.len());
        for (j, (operand, field)) in inputs.iter().zip(actual).enumerate() {
            if i % 5 == 4 && j + 1 == inputs.len() {
                assert!(matches!(operand, Operand::Variable(_)));
            } else {
                assert_eq!(operand, &Operand::Immediate(*field));
            }
        }
    }
    assert!(
        a.ir.instructions
            .iter()
            .any(|i| matches!(i, Instruction::ConstrainToBoolean { .. }))
    );
    assert!(
        a.ir.instructions
            .iter()
            .any(|i| matches!(i, Instruction::ConstrainBits { bits: 128, .. }))
    );
}

#[test]
fn full_escrow_style_multi_payer_source_still_refuses_all_action_admission() {
    let src = source(r#"let amount = liability_amount("d");"#).replace(
        "action declare()",
        "action other() { transfer(A, \"bob\", \"alice\", 0); } action declare()",
    );
    let p = mori_core::parse(&src).unwrap();
    let safe_src = src.replace(
        "transfer(A, \"bob\", \"alice\", 0);",
        "transfer(A, \"alice\", \"bob\", 0);",
    );
    let safe_p = mori_core::parse(&safe_src).unwrap();
    let mut d = deployment(&safe_src, &safe_p);
    d.source_hash = codec::hash(src.as_bytes());
    d.program_hash = codec::object_hash(&p).unwrap();
    let err = relation::compile(&src, &p, &d, "run").err().unwrap();
    assert_eq!(err.code, "one-public-payer");
}

#[test]
fn signed_source_compile_retains_endpoint_and_consuming_authority_order() {
    use midnight_base_crypto::hash::HashOutput;
    use midnight_coin_structure::contract::ContractAddress;
    use mori_ledger::signed::{self, FixedSignerPolicy, SignedStaticBindings, SignedVersion};
    let src = source(r#"let old = liability_amount("d"); next x = old;"#);
    let p = mori_core::parse(&src).unwrap();
    let mut d = deployment(&src, &p);
    d.address_hex = codec::native_hex(&ContractAddress(HashOutput([7; 32]))).unwrap();
    let statics = StaticBindings {
        abi: d.abi.clone(),
        source_hash: d.source_hash.clone(),
        program_hash: d.program_hash.clone(),
        abi_hash: d.abi_hash.clone(),
        templates_hash: d.templates_hash.clone(),
        toolchain: d.toolchain.clone(),
        limits: d.limits.clone(),
        network: d.network.clone(),
        anchor: d.anchor.clone(),
        address_hex: d.address_hex.clone(),
        payer: d.payer.clone(),
        assets: d.assets.clone(),
        roles: d.roles.clone(),
        duties: d.duties.clone(),
        economic_bounds: None,
    };
    // Existing public verifier point, no signing key or key generation.
    let policy = FixedSignerPolicy { profile:signed::POLICY.into(), owner:"alice".into(), beneficiary:"bob".into(), public_key_hex:"0677506737143823730512c5525e0b30acb75906fa7d53f494ba689b78620133d4cdab997110c2f1e02bb16ebef816c9d0a6025386825581a688b9bfa326360a".into() };
    for version in [SignedVersion::LegacyV1, SignedVersion::ConsumingV2] {
        let s = SignedStaticBindings::from_source(&src, &p, &statics, &policy, version).unwrap();
        let a = signed::compile_signed(&s, "run").unwrap();
        let origins = a
            .manifest
            .groups
            .iter()
            .map(|g| g.origin.as_str())
            .collect::<Vec<_>>();
        let source_start = origins.iter().position(|x| *x == "state-write").unwrap();
        assert_eq!(
            origins.iter().filter(|x| **x == "duty-membership").count(),
            15
        ); // pre/source/post
        assert!(
            origins[..source_start]
                .iter()
                .any(|x| *x == "signed-duty-cardinality")
        );
        assert!(
            origins[source_start..]
                .iter()
                .any(|x| *x == "signed-duty-cardinality")
        );
        if version == SignedVersion::ConsumingV2 {
            let pre = origins
                .iter()
                .position(|x| *x == "signed-authority-pre-member")
                .unwrap();
            let write = origins
                .iter()
                .position(|x| *x == "signed-authority-write")
                .unwrap();
            let post = origins
                .iter()
                .position(|x| *x == "signed-authority-post-member")
                .unwrap();
            assert!(pre < source_start && source_start < write && write < post);
        }
        let descriptors = &a
            .manifest
            .signed
            .as_ref()
            .unwrap()
            .semantic_wire_descriptors;
        assert!(
            !descriptors
                .iter()
                .any(|w| w.descriptor.contains("source.read"))
        );
    }
}

#[test]
fn source_ir_constrains_bool_uint128_full_identity_absent_zero_and_repeated_snapshot() {
    use midnight_transient_crypto::{curve::Fr, fab::AlignedValueExt};
    use midnight_zkir_v3::{Instruction, ir::Operand};
    let src = source(r#"requires liability_present("d") or true; let n = liability_amount("d");"#);
    let p = mori_core::parse(&src).unwrap();
    let d = deployment(&src, &p);
    let a = relation::compile(&src, &p, &d, "run").unwrap();
    let instructions = &a.ir.instructions;
    assert!(
        !instructions
            .iter()
            .any(|i| matches!(i, Instruction::PrivateInput { .. }))
    );
    let impacts = instructions
        .iter()
        .filter_map(|i| match i {
            Instruction::Impact { guard, inputs } => Some((guard, inputs)),
            _ => None,
        })
        .zip(&a.manifest.groups)
        .collect::<Vec<_>>();
    let mut constants = vec![];
    layout::duty_value(&d.duties[0], 0, &d.roles)
        .unwrap()
        .value_only_field_repr(&mut constants);
    let mut members = vec![];
    let mut amounts = vec![];
    for i in 0..impacts.len() {
        if impacts[i].1.origin != "duty-membership"
            || impacts
                .get(i + 1)
                .is_some_and(|x| x.1.origin == "duty-membership")
        {
            continue;
        }
        let member = impacts[i].0.1.last().unwrap();
        members.push(member.clone());
        assert!(
            instructions
                .iter()
                .any(|x| matches!(x,Instruction::ConstrainToBoolean { val } if val==member))
        );
        let reads = &impacts[i + 1..i + 4];
        assert!(
            reads
                .iter()
                .all(|((guard, _), g)| *guard == member && g.origin == "duty-old-read")
        );
        let fields = reads[2].0.1;
        let tail = &fields[fields.len() - constants.len()..];
        let amount = tail.last().unwrap();
        amounts.push(amount.clone());
        assert!(
            instructions
                .iter()
                .any(|x| matches!(x,Instruction::ConstrainBits { val,bits:128 } if val==amount))
        );
        for (opening, constant) in tail.iter().zip(&constants).take(constants.len() - 1) {
            let selected = instructions
                .iter()
                .find_map(|x| match x {
                    Instruction::CondSelect { bit, a, b, output }
                        if bit == member
                            && a == &Operand::Immediate(*constant)
                            && b == &Operand::Immediate(Fr::from(0u64)) =>
                    {
                        Some(Operand::Variable(output.clone()))
                    }
                    _ => None,
                })
                .unwrap();
            assert!(instructions.iter().any(
                |x| matches!(x,Instruction::ConstrainEq { a,b } if a==opening && b==&selected)
            ));
        }
        let absent = instructions
            .iter()
            .find_map(|x| match x {
                Instruction::CondSelect { bit, a, b, output }
                    if bit == member && a == &Operand::Immediate(Fr::from(0u64)) && b == amount =>
                {
                    Some(Operand::Variable(output.clone()))
                }
                _ => None,
            })
            .unwrap();
        assert!(instructions.iter().any(|x|matches!(x,Instruction::ConstrainEq { a,b } if a==&absent && b==&Operand::Immediate(Fr::from(0u64)))));
    }
    assert_eq!(members.len(), 2);
    assert!(
        instructions.iter().any(
            |i| matches!(i,Instruction::ConstrainEq { a,b } if a==&members[1] && b==&members[0])
        )
    );
    let live = instructions
        .iter()
        .find_map(|i| match i {
            Instruction::CondSelect { bit, a, b, output }
                if bit == &members[1]
                    && a == &amounts[0]
                    && b == &Operand::Immediate(Fr::from(0u64)) =>
            {
                Some(Operand::Variable(output.clone()))
            }
            _ => None,
        })
        .unwrap();
    assert!(
        instructions
            .iter()
            .any(|i| matches!(i,Instruction::ConstrainEq { a,b } if a==&amounts[1] && b==&live))
    );
}

#[test]
fn checked_in_full_escrow_refuses_one_public_payer_even_for_later_action() {
    let src = include_str!("../../../examples/core/escrow-recovery/escrow.mori");
    let p = mori_core::parse(src).unwrap();
    // A contrasting source binds all actual payment operands to one payer;
    // source duties retain their distinct debtor/creditor identities.
    let safe = src
        .replace("transfer(PAY, \"buyer\"", "transfer(PAY, \"alice\"")
        .replace("fee(PAY, \"buyer\"", "fee(PAY, \"alice\"")
        .replace("transfer(GOODS, \"seller\"", "transfer(GOODS, \"alice\"")
        .replace("transfer(PAY, \"escrow\"", "transfer(PAY, \"alice\"")
        .replace("transfer(GOODS, \"escrow\"", "transfer(GOODS, \"alice\"");
    let supported = mori_core::parse(&safe).unwrap();
    let mut d = deployment(&safe, &supported);
    relation::compile(&safe, &supported, &d, "partial").unwrap();
    d.source_hash = codec::hash(src.as_bytes());
    d.program_hash = codec::object_hash(&p).unwrap();
    for action in ["fund", "partial", "complete", "recover"] {
        assert_eq!(
            relation::compile(src, &p, &d, action).err().unwrap().code,
            "one-public-payer"
        );
    }
}

#[test]
fn reference_oracle_preserves_arithmetic_and_absent_discharge_failures() {
    for (body, amount) in [
        (r#"discharge("d", 0);"#, None),
        (r#"discharge("d", liability_amount("d") + 1);"#, Some(7)),
        (
            r#"liability("d", A, "alice", "bob", liability_amount("d") + 1);"#,
            Some(u128::MAX),
        ),
        (
            r#"let discarded = liability_amount("d") + 1;"#,
            Some(u128::MAX),
        ),
    ] {
        let src = source(body);
        let p = mori_core::parse(&src).unwrap();
        let d = deployment(&src, &p);
        relation::compile(&src, &p, &d, "run").unwrap();
        let s = scenario(&p, amount);
        let before = s.clone();
        assert!(mori_core::simulate(&p, "run", &s).is_err());
        assert_eq!(s, before);
    }
}

#[test]
fn source_static_ir_is_independent_of_action_inputs() {
    let src =
        source(r#"next x = liability_amount("d") + n; emit "current"(liability_present("d"));"#)
            .replace("action run()", "action run(n: UInt)");
    let p = mori_core::parse(&src).unwrap();
    let d = deployment(&src, &p);
    let a = relation::compile(&src, &p, &d, "run").unwrap();
    for n in [0, 1, 9, 1 << 64] {
        let mut s = scenario(&p, Some(7));
        s.inputs.insert("n".into(), Value::UInt(UInt(n)));
        let e = mori_core::simulate(&p, "run", &s).unwrap();
        let out = producer(&p, &d, &s, &e);
        assert_eq!(e.post.state["x"], Value::UInt(UInt(7 + n)));
        assert!(!out.program.is_empty());
        let current = relation::compile(&src, &p, &d, "run").unwrap();
        assert_eq!(a.ir_bytes().unwrap(), current.ir_bytes().unwrap());
        assert_eq!(
            a.manifest_bytes().unwrap(),
            current.manifest_bytes().unwrap()
        );
    }
}

#[test]
fn producer_shared_expr_ids_deduplicate_each_statement_and_refresh_after_write() {
    let src = source(
        r#"let saved = liability_amount("d"); discharge("d", 3); next x = liability_amount("d"); let doubled = liability_amount("d") + liability_amount("d"); ensures saved == 7 and x == 4;"#,
    );
    let mut p = mori_core::parse(&src).unwrap();
    let d = deployment(&src, &p);
    relation::compile(&src, &p, &d, "run").unwrap();
    let action = p.actions.iter_mut().find(|a| a.name == "run").unwrap();
    let shared = match action.statements[0] {
        mori_core::Statement::Let { value, .. } => value,
        _ => unreachable!(),
    };
    if let mori_core::Statement::Next { value, .. } = &mut action.statements[2] {
        *value = shared;
    }
    if let mori_core::Statement::Let { value, .. } = action.statements[3] {
        if let mori_core::Expr::Binary { lhs, rhs, .. } = &mut p.expressions[value as usize] {
            *lhs = shared;
            *rhs = shared;
        }
    }
    mori_core::validate(&p).unwrap();
    let s = scenario(&p, Some(7));
    let e = mori_core::simulate(&p, "run", &s).unwrap();
    mori_core::replay(&p, "run", &s, &e).unwrap();
    assert_eq!(e.post.state["x"], Value::UInt(UInt(4)));
    let out = producer(&p, &d, &s, &e);
    for step in [0, 2, 3] {
        assert_eq!(
            out.bindings
                .iter()
                .filter(|b| b.step == Some(step) && b.origin == "duty-membership")
                .count(),
            5
        );
    }
    // Raw edited IR is not a second way around public source identity.
    assert_eq!(
        relation::compile(&src, &p, &d, "run").err().unwrap().code,
        "source-identity"
    );
}
