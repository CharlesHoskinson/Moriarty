//! A3: consume the exact public A2 artifacts, reconstruct child keys and prove at k20.
use group::Curve;
use midnight_proofs::{poly::kzg::params::ParamsKZG, utils::SerdeFormat};
use mori_backend::proving::{MAX_K, ProofOptions};
use mori_backend::sherlock_child::ChildKey;
use mori_backend::sherlock_link::{ReturnChildKey, produce_linked_requests};
use mori_backend::sherlock_machine::literal_bridge::LiteralSourceIdentity;
use mori_backend::sherlock_machine::literal_host::LiteralCapacities;
use mori_backend::sherlock_machine::literal_witness::core_program_capacities;
use mori_core::sherlock::layout_a::prepare_a;
use mori_core::sherlock::{Expr, K_SOURCE_SHA256, Program, Value};
use mori_recursion::{
    child::SherlockChild,
    parent::{
        LinkPins, ParentKeygenPlan, SherlockLinkRelation, SherlockLinkWitness, prove_link_parent,
        write_w_dir,
    },
    util::{F, f_from_hex, sha256_hex, vk_from_bytes},
};
use sha2::{Digest, Sha256};
use std::{fs, io::Cursor, path::PathBuf, sync::Arc, time::Instant};

#[test]
#[ignore = "real k20 proof; use scripts/doctorwatson check A3"]
fn parent_verifies_both_linked_children_at_k20() {
    let options = ProofOptions {
        params_dir: std::env::var_os("MORIARTY_TEST_PARAMS_DIR").unwrap().into(),
        max_k: MAX_K,
    };
    let input = PathBuf::from(std::env::var_os("MORIARTY_A2_LINK_DIR").unwrap());
    let output = PathBuf::from(std::env::var_os("MORIARTY_PARENT_OUTPUT_DIR").unwrap());
    let capacities = LiteralCapacities {
        program: [2, 8, 2, 2, 2, 2, 1, 1, 1, 1, 4, 4, 5, 2, 2],
        state: [1; 6],
        stack_limit: 1,
    };
    let program = Arc::new(Program {
        functions: Vec::new(),
        entry: Arc::new(Expr::Literal(Value::UInt(7))),
    });
    let prepared = prepare_a(
        program.clone(),
        core_program_capacities(&capacities).unwrap(),
    )
    .unwrap();
    let identity = LiteralSourceIdentity {
        k_source_sha256: K_SOURCE_SHA256.into(),
        reference_source_sha256: sha256_hex(include_bytes!(
            "../../mori-core/src/sherlock/reference.rs"
        )),
        program_source_sha256: sha256_hex(&prepared.program_bytes().unwrap()),
    };
    let (requests, shape) = produce_linked_requests(program, &capacities, 2, 1, &identity).unwrap();
    let a102_key = ChildKey::reconstruct(&requests.a102, &options).unwrap();
    let a80_key = ReturnChildKey::reconstruct(&requests.a80, &options).unwrap();
    let pins = LinkPins {
        a102_vk_sha256: a102_key.vk_sha256(),
        a80_vk_sha256: a80_key.vk_sha256(),
        a102_context_hash: a102_key.claim().context_hash.clone(),
        a80_context_hash: a80_key.claim().context_hash.clone(),
        program_lanes: shape.program_lanes.try_into().unwrap(),
        state_lanes: shape.state_lanes.try_into().unwrap(),
    };
    let read_child = |name: &str, pin: &str, expected: Vec<F>| {
        let raw = fs::read(input.join(format!("{name}.vk"))).unwrap();
        let lanes: Vec<String> =
            serde_json::from_slice(&fs::read(input.join(format!("{name}.instance.json"))).unwrap())
                .unwrap();
        let instance = lanes
            .iter()
            .map(|s| f_from_hex(s).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            instance, expected,
            "A2 instance must equal independent reconstruction"
        );
        SherlockChild::from_bytes(
            &raw,
            pin,
            shape.instance_lanes,
            instance,
            fs::read(input.join(format!("{name}.proof"))).unwrap(),
        )
        .unwrap()
    };
    let a102 = read_child(
        "a102",
        &pins.a102_vk_sha256,
        a102_key.instance().iter().map(|x| x.0).collect(),
    );
    let a80 = read_child(
        "a80",
        &pins.a80_vk_sha256,
        a80_key.instance().iter().map(|x| x.0).collect(),
    );
    let relation = SherlockLinkRelation::from_pins(
        vk_from_bytes(a102_key.vk_bytes()).unwrap(),
        vk_from_bytes(a80_key.vk_bytes()).unwrap(),
        &pins,
    )
    .unwrap();
    let witness = SherlockLinkWitness::from_children(&a102, &a80, 0);
    let plan = ParentKeygenPlan::new(&relation).unwrap();
    println!(
        "A3 model_k={} model_seconds={:.3}",
        plan.model().k,
        plan.model_elapsed().as_secs_f64()
    );
    assert_eq!(plan.model().k, 20, "A3 requires a production k20 parent");

    // Authenticate before RawBytesUnchecked decoding. Pin retained from the official ceremony.
    let srs_path = PathBuf::from(std::env::var_os("MORIARTY_PARENT_SRS").unwrap());
    assert_eq!(fs::metadata(&srs_path).unwrap().len(), 201_326_980);
    let bytes = fs::read(srs_path).unwrap();
    assert_eq!(
        hex::encode(Sha256::digest(&bytes)),
        "1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1"
    );
    assert_eq!(bytes.get(..4), Some(20u32.to_le_bytes().as_slice()));
    let mut cursor = Cursor::new(bytes);
    let params = ParamsKZG::<midnight_curves::Bls12>::read_custom(
        &mut cursor,
        SerdeFormat::RawBytesUnchecked,
    )
    .unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
    drop(cursor);
    let s_g2 = params.s_g2().to_affine();
    fs::create_dir(&output).unwrap();
    let start = Instant::now();
    let vk = plan.keygen(&params, &s_g2, &output.join("key")).unwrap();
    println!("A3 keygen_seconds={:.3}", start.elapsed().as_secs_f64());
    let start = Instant::now();
    let parent = prove_link_parent(&params, &s_g2, &relation, &vk, &witness).unwrap();
    assert_eq!(parent.instance.len(), 28);
    assert!(!parent.proof.is_empty());
    let mut wrong_instance = parent.instance.clone();
    wrong_instance[0] += F::from(1u64);
    assert!(
        midnight_zk_stdlib::verify::<
            SherlockLinkRelation,
            midnight_circuits::hash::poseidon::PoseidonState<F>,
        >(
            &params.verifier_params(),
            &parent.vk,
            &wrong_instance,
            None,
            &parent.proof,
        )
        .is_err()
    );
    println!(
        "A3 prove_verify_seconds={:.3} proof_bytes={} instance_lanes={}",
        start.elapsed().as_secs_f64(),
        parent.proof.len(),
        parent.instance.len()
    );
    write_w_dir(
        &output.join("parent"),
        &parent,
        "Moriarty A3: fixed A102 to A80 at height zero",
    )
    .unwrap();
    println!("A3 accepted public_artifacts={}", output.display());
}
