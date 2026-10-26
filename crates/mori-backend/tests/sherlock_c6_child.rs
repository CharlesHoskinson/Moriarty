//! U023 recursion adapter: a C6 LocalPublic leaf must feed the Sherlock
//! recursion chain (leaf -> parent -> decider). These tests pin the host-side
//! feed shape only: claim, binding lane, complete instance, witness carrier
//! and the recursion-child export. Native prove/verify stay UNOBSERVED here.
use midnight_transient_crypto::curve::Fr;
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_backend::sherlock_c6_child::{C6_CHILD_FORMAT, prepare_c6_child, recursion_child_input};
use std::sync::Arc;

/// LocalPublic relation in the exact C6 harness shape: unguarded Native
/// public inputs, each published by one unconditional Impact, no outputs.
fn local_public_ir(lanes: usize) -> IrSource {
    let mut code = Vec::new();
    let mut published = Vec::new();
    for i in 0..lanes {
        let id = Identifier(format!("%c6.public.{i}"));
        code.push(Instruction::PublicInput {
            guard: None,
            val_t: IrType::Native,
            output: id.clone(),
        });
        published.push(Operand::Variable(id));
    }
    // Real C6 fragments constrain every statement lane; an unconstrained
    // public input is skippable in the SDK nonproving check.
    for op in &published {
        code.push(Instruction::ConstrainBits {
            val: op.clone(),
            bits: 128,
        });
    }
    code.push(Instruction::Impact {
        guard: Operand::Immediate(Fr::from(1u64)),
        inputs: published,
    });
    let ir0 = IrSource {
        version: Default::default(),
        inputs: Vec::new(),
        outputs: Vec::new(),
        do_communications_commitment: false,
        instructions: Arc::new(code),
    };
    // Same pinned loader round-trip the C6 harness applies.
    let mut disk = serde_json::to_value(&ir0).expect("serialize");
    disk["version"] = serde_json::json!({"major": 3, "minor": 0});
    IrSource::load(serde_json::to_string(&disk).expect("encode").as_bytes()).expect("load")
}

#[test]
fn c6_leaf_instance_is_binding_then_statement_and_feeds_recursion_child() {
    let ir = local_public_ir(3);
    let statement = vec![1u128, 2, 3];
    let prepared = prepare_c6_child(&ir, "AppendHelper", &statement).expect("prepare leaf");
    assert_eq!(prepared.claim.format, C6_CHILD_FORMAT);
    assert_eq!(prepared.claim.rule_id, "AppendHelper");
    assert_eq!(prepared.claim.statement_lanes, 3);
    assert_eq!(prepared.instance.len(), 4);
    assert_eq!(prepared.instance[0], prepared.binding);
    assert_eq!(
        prepared.instance[1..],
        [Fr::from(1u128), Fr::from(2u128), Fr::from(3u128)]
    );
    // Witness carrier accepted by the checked child_witness shape.
    let witness = prepared.witness().expect("carrier");
    assert_eq!(witness.pis.len(), 4);
    // Export shape consumed by mori-recursion SherlockChild::from_bytes.
    let feed = recursion_child_input(&prepared, b"vk-bytes", b"proof-bytes");
    assert_eq!(feed.instance_lanes, 4);
    assert_eq!(feed.instance_hex.len(), 4);
    assert_eq!(feed.vk_sha256.len(), 64);
}

#[test]
fn c6_leaf_binding_binds_rule_and_statement() {
    let ir = local_public_ir(3);
    let a = prepare_c6_child(&ir, "AppendHelper", &[1, 2, 3]).expect("a");
    let b = prepare_c6_child(&ir, "AppendHelper", &[1, 2, 4]).expect("b");
    let c = prepare_c6_child(&ir, "BindHelper", &[1, 2, 3]).expect("c");
    assert_ne!(a.binding, b.binding);
    assert_ne!(a.binding, c.binding);
    assert_ne!(a.claim.statement_hash, b.claim.statement_hash);
}

#[test]
fn c6_digest_leaf_exposes_only_five_public_lanes() {
    use mori_backend::sherlock_c6_child::{
        DigestSections, StateOutcome, prepare_digest_c6_child, rule_lane, statement_digests,
    };
    let ir = local_public_ir(3);
    let sections = DigestSections {
        prog_lanes: 1,
        pre_lanes: 1,
        post_lanes: 1,
    };
    let prepared = prepare_digest_c6_child(
        &ir,
        "AppendHelper",
        &[1, 2, 3],
        sections,
        StateOutcome::RUNNING,
        StateOutcome::RUNNING,
        7,
    )
    .expect("prepare");
    assert_eq!(prepared.instance.len(), 5);
    assert_eq!(prepared.instance, prepared.public_statement);
    let digests = statement_digests(
        &[1, 2, 3],
        sections,
        StateOutcome::RUNNING,
        StateOutcome::RUNNING,
    )
    .expect("digests");
    assert_eq!(prepared.public_statement[..3], digests);
    assert_eq!(prepared.public_statement[3], rule_lane("AppendHelper"));
    assert_eq!(prepared.public_statement[4], Fr::from(7u128));
    let witness = prepared.witness().expect("digest carrier");
    assert_eq!(witness.pis.len(), 5);
    assert_eq!(witness.pi_skips.len(), 5);
}

/// Heavy M2: real keygen/prove/verify of one C6 digest leaf under the pinned
/// SRS, in the style of `sherlock_linked_children`. Explicitly enabled only.
#[test]
#[ignore = "real native proofs; requires pinned SRS (MORIARTY_TEST_PARAMS_DIR)"]
fn c6_leaf_is_proved_verified_and_exported() {
    use mori_backend::proving::{MAX_K, ProofOptions};
    use mori_backend::sherlock_c6_child::{
        DigestSections, StateOutcome, prepare_digest_c6_child, prove_c6_child, verify_c6_child,
    };
    let options = ProofOptions {
        params_dir: std::env::var_os("MORIARTY_TEST_PARAMS_DIR")
            .expect("explicit pinned local SRS directory")
            .into(),
        max_k: MAX_K,
    };
    // 96 lanes put the digest relation at a degree with a pinned local SRS.
    let ir = local_public_ir(96);
    let sections = DigestSections {
        prog_lanes: 32,
        pre_lanes: 32,
        post_lanes: 32,
    };
    let statement: Vec<u128> = (0..96).map(|i| i as u128 + 1).collect();
    let start = std::time::Instant::now();
    let child = prove_c6_child(
        &ir,
        "AppendHelper",
        &statement,
        sections,
        StateOutcome::RUNNING,
        StateOutcome::RUNNING,
        7,
        &options,
    )
    .expect("prove C6 digest leaf");
    println!("C6 M2 prove_seconds={:.3}", start.elapsed().as_secs_f64());
    assert!(!child.proof.is_empty());
    assert_eq!(child.instance.len(), 5);
    verify_c6_child(
        &ir,
        "AppendHelper",
        &statement,
        sections,
        StateOutcome::RUNNING,
        StateOutcome::RUNNING,
        7,
        &child,
        &options,
    )
    .expect("verify C6 digest leaf");
    let prepared = prepare_digest_c6_child(
        &ir,
        "AppendHelper",
        &statement,
        sections,
        StateOutcome::RUNNING,
        StateOutcome::RUNNING,
        7,
    )
    .expect("prepare");
    assert_eq!(prepared.instance, child.instance);
    // Mutated proof is refused by independent verification.
    let mut bad = child.clone();
    bad.proof[0] ^= 1;
    assert!(
        verify_c6_child(
            &ir,
            "AppendHelper",
            &statement,
            sections,
            StateOutcome::RUNNING,
            StateOutcome::RUNNING,
            7,
            &bad,
            &options
        )
        .is_err()
    );
    println!(
        "C6 leaf k={} proof_bytes={} instance_lanes={}",
        child.claim.circuit_k,
        child.proof.len(),
        child.instance.len()
    );
}

#[test]
fn c6_leaf_refuses_empty_and_private_input_relations() {
    let ir = local_public_ir(3);
    assert!(prepare_c6_child(&ir, "AppendHelper", &[]).is_err());
    let mut bad = local_public_ir(1);
    let mut code = (*bad.instructions).clone();
    code.insert(
        0,
        Instruction::PrivateInput {
            guard: None,
            val_t: IrType::Native,
            output: Identifier("%c6.private.0".into()),
        },
    );
    bad.instructions = Arc::new(code);
    assert!(prepare_c6_child(&bad, "AppendHelper", &[1]).is_err());
}
