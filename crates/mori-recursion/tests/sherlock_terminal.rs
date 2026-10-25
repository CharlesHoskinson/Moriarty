//! A4 requires the exact installed verifier, not a fresh unrelated terminal key.
use ff::Field;
use group::Curve;
use midnight_curves::Bls12;
use midnight_onchain_state::state::{ContractState, EntryPointBuf};
use midnight_proofs::{poly::kzg::params::ParamsKZG, utils::SerdeFormat};
use midnight_storage::db::InMemoryDB;
use midnight_transient_crypto::{
    curve::Fr,
    proofs::{PARAMS_VERIFIER, Proof},
};
use midnight_zk_stdlib::{Relation, cost_model, prove, setup_pk, verify};
use mori_recursion::{
    decider::{DeciderRelation, DeciderWitness},
    inner::{verify_offcircuit, vk_name},
    parent::{SherlockLinkRelation, keygen_sherlock_decider, sherlock_decider_relation},
    template::Template,
    util::{F, f_from_hex, sha256_hex, vk_bytes, vk_from_bytes},
};
use std::{fs, io::Cursor, path::PathBuf, time::Instant};

const INSTALLED_VK_SHA256: &str =
    "bd9e9d89ad0a5570571160b29dc8b2d89e11e3c6065b46638a2e978f9218776e";

#[test]
#[ignore = "real terminal proof; use scripts/doctorwatson check A4"]
fn terminal_proof_matches_the_installed_stagenet_verifier() {
    let path = |name| PathBuf::from(std::env::var_os(name).unwrap());
    let input = path("MORIARTY_A3_PARENT_DIR");
    let output = path("MORIARTY_TERMINAL_OUTPUT_DIR");
    let state_raw = fs::read(path("MORIARTY_DEPLOYED_CONTRACT_STATE")).unwrap();
    assert!(state_raw.len() <= 16 * 1024 * 1024);
    let state: ContractState<InMemoryDB> =
        midnight_serialize::tagged_deserialize(&state_raw[..]).unwrap();
    let mut roundtrip = Vec::new();
    midnight_serialize::tagged_serialize(&state, &mut roundtrip).unwrap();
    assert_eq!(roundtrip, state_raw);
    let entry = EntryPointBuf::from(b"acceptValidity".as_slice());
    let operation = state.operations.get(&entry).unwrap();
    let installed = operation.latest().expect("installed v3 verifier");
    let installed_bytes = installed.original_bytes();
    assert_eq!(sha256_hex(&installed_bytes), INSTALLED_VK_SHA256);
    let installed_vk = vk_from_bytes(&installed_bytes).unwrap();

    let srs_path = path("MORIARTY_PARENT_SRS");
    assert_eq!(fs::metadata(&srs_path).unwrap().len(), 201_326_980);
    let bytes = fs::read(srs_path).unwrap();
    assert_eq!(
        sha256_hex(&bytes),
        "1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1"
    );
    let mut cursor = Cursor::new(bytes);
    let params =
        ParamsKZG::<Bls12>::read_custom(&mut cursor, SerdeFormat::RawBytesUnchecked).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
    drop(cursor);
    let vp = params.verifier_params();
    let parent_raw = fs::read(input.join("w.vk")).unwrap();
    let parent_vk = vk_from_bytes(&parent_raw).unwrap();
    let parent_proof = fs::read(input.join("w.proof")).unwrap();
    let lanes: Vec<String> =
        serde_json::from_slice(&fs::read(input.join("w.instance.json")).unwrap()).unwrap();
    let parent_instance = lanes
        .iter()
        .map(|s| f_from_hex(s).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(parent_instance.len(), 28);
    verify::<SherlockLinkRelation, midnight_circuits::hash::poseidon::PoseidonState<F>>(
        &vp,
        &parent_vk,
        &parent_instance,
        None,
        &parent_proof,
    )
    .unwrap();
    verify_offcircuit(
        &vp,
        &parent_vk,
        &vk_name("w", &parent_vk),
        &parent_instance,
        &parent_proof,
        true,
    )
    .unwrap();
    let template =
        Template::from_any_json(include_bytes!("fixtures/accept-template.json")).unwrap();
    let relation =
        sherlock_decider_relation(parent_vk, params.s_g2().to_affine(), template).unwrap();
    let start = Instant::now();
    let model = cost_model(&relation, None);
    println!(
        "A4 model_k={} rows={} model_seconds={:.3} parent_vk_sha256={}",
        model.k,
        model.rows,
        start.elapsed().as_secs_f64(),
        sha256_hex(&parent_raw)
    );
    assert_eq!(
        model.k,
        u32::from(installed_vk.k()),
        "terminal degree must match the installed key"
    );
    fs::create_dir(&output).unwrap();
    let start = Instant::now();
    let vk = keygen_sherlock_decider(&params, &relation, &output.join("key")).unwrap();
    let candidate = vk_bytes(&vk);
    println!(
        "A4 keygen_seconds={:.3} reconstructed_vk_sha256={} installed_vk_sha256={}",
        start.elapsed().as_secs_f64(),
        sha256_hex(&candidate),
        INSTALLED_VK_SHA256
    );
    assert!(
        candidate == installed_bytes,
        "A4 terminal key mismatch: reconstructed={} installed={}; the exact deployed relation is required",
        sha256_hex(&candidate),
        INSTALLED_VK_SHA256
    );

    // The communication opening exists only in memory and is never logged or persisted.
    let mut rng = rand::rngs::OsRng;
    let witness = DeciderWitness {
        w_instance: parent_instance,
        w_proof: parent_proof,
        instance_id: f_from_hex("489080d3e57cacab615cd0cde4c87914b47c4ff0ab5857d865ee2474ecd45c00")
            .unwrap(),
        batch_no: F::ZERO,
        comm_rand: F::random(&mut rng),
    };
    let stmt = relation.stmt(&witness.w_instance, witness.instance_id, witness.batch_no);
    let public = relation.template.public_inputs(
        F::from(1u64),
        relation.comm_commitment_for(stmt, witness.comm_rand),
        stmt,
    );
    let pk = setup_pk(&relation, &vk);
    let start = Instant::now();
    let proof = prove::<DeciderRelation, blake2b_simd::State>(
        &params, &pk, &relation, &public, witness, &mut rng,
    )
    .unwrap();
    verify::<DeciderRelation, blake2b_simd::State>(&vp, &installed_vk, &public, None, &proof)
        .unwrap();
    installed
        .verify(
            &PARAMS_VERIFIER,
            &Proof(proof.clone()),
            public.iter().copied().map(Fr),
        )
        .unwrap();
    for lane in 0..public.len() {
        let mut bad = public.clone();
        bad[lane] += F::ONE;
        assert!(
            verify::<DeciderRelation, blake2b_simd::State>(&vp, &installed_vk, &bad, None, &proof)
                .is_err()
        );
    }
    let mut bad = proof.clone();
    bad[0] ^= 1;
    assert!(
        verify::<DeciderRelation, blake2b_simd::State>(&vp, &installed_vk, &public, None, &bad)
            .is_err()
    );
    fs::write(output.join("terminal.proof"), &proof).unwrap();
    fs::write(
        output.join("terminal.public.json"),
        serde_json::to_vec_pretty(
            &public
                .iter()
                .map(mori_recursion::util::f_to_hex)
                .collect::<Vec<_>>(),
        )
        .unwrap(),
    )
    .unwrap();
    let mut relation_bytes = Vec::new();
    relation.write_relation(&mut relation_bytes).unwrap();
    fs::write(output.join("terminal.relation"), relation_bytes).unwrap();
    println!(
        "A4 accepted proof_bytes={} prove_verify_seconds={:.3} public_artifacts={}",
        proof.len(),
        start.elapsed().as_secs_f64(),
        output.display()
    );
}
