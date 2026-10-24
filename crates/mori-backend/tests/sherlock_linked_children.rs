//! A2 real proofs. Explicitly enabled by scripts/qa/check-a2.sh; requires pinned SRS.
use mori_backend::proving::{MAX_K, ProofOptions};
use mori_backend::sherlock_link::{
    produce_linked_requests, prove_linked_children, verify_linked_children, write_link_dir,
};
use mori_backend::sherlock_machine::literal_bridge::LiteralSourceIdentity;
use mori_backend::sherlock_machine::literal_host::LiteralCapacities;
use mori_backend::sherlock_machine::literal_witness::core_program_capacities;
use mori_core::sherlock::layout_a::prepare_a;
use mori_core::sherlock::{Expr, K_SOURCE_SHA256, Program, Value};
use sha2::{Digest, Sha256};
use std::{path::PathBuf, sync::Arc, time::Instant};

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[test]
#[ignore = "real native proofs; use scripts/doctorwatson check A2"]
fn linked_children_are_proved_and_verified() {
    let options = ProofOptions {
        params_dir: std::env::var_os("MORIARTY_TEST_PARAMS_DIR")
            .expect("explicit pinned local SRS directory")
            .into(),
        max_k: MAX_K,
    };
    let output = PathBuf::from(
        std::env::var_os("MORIARTY_LINK_OUTPUT_DIR").expect("fresh public artifact directory"),
    );
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
        reference_source_sha256: sha256(include_bytes!(
            "../../mori-core/src/sherlock/reference.rs"
        )),
        program_source_sha256: sha256(&prepared.program_bytes().unwrap()),
    };
    let (requests, shape) = produce_linked_requests(program, &capacities, 2, 1, &identity).unwrap();
    assert_eq!(requests.a102.values.post, requests.a80.values.pre);

    let start = Instant::now();
    let children = prove_linked_children(&requests, &options).expect("prove both child relations");
    println!("A2 prove_seconds={:.3}", start.elapsed().as_secs_f64());
    let start = Instant::now();
    let verified = verify_linked_children(&requests, &children, &options)
        .expect("reconstruct both keys and verify both complete child statements");
    assert_eq!(verified.shape(), shape);
    for (name, child) in [("a102", verified.a102()), ("a80", verified.a80())] {
        assert!(child.proof_bytes > 0);
        assert_eq!(child.instance_lanes, shape.instance_lanes);
        println!(
            "A2 {name} k={} proof_bytes={} instance_lanes={}",
            child.circuit_k, child.proof_bytes, child.instance_lanes
        );
    }
    println!("A2 verify_seconds={:.3}", start.elapsed().as_secs_f64());

    // Exercise rejection on actual proof bytes and actual independently reconstructed keys.
    for a102 in [true, false] {
        let mut bad = children.clone();
        let proof = if a102 {
            &mut bad.a102.proof
        } else {
            &mut bad.a80.proof
        };
        proof[0] ^= 1;
        assert!(verify_linked_children(&requests, &bad, &options).is_err());
    }
    let mut wrong_key = children.clone();
    wrong_key.a80.vk_bytes = children.a102.vk_bytes.clone();
    assert!(verify_linked_children(&requests, &wrong_key, &options).is_err());

    let manifest = write_link_dir(&output, &requests, &children, &verified).unwrap();
    assert_eq!(manifest.children.len(), 2);
    println!("A2 accepted public_artifacts={}", output.display());
}
