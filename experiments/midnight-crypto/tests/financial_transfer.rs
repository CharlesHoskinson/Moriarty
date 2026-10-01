#![cfg(feature = "native-proof")]
use midnight_proofs::{dev::MockProver, plonk::Circuit};
use moriarty_midnight_crypto::financial_transfer::{
    PUBLIC_FIELDS, PUBLIC_VERSION, TransferCircuit, fixture_public_values,
};
use serde_json::Value;
fn fixture() -> Value {
    serde_json::from_str(include_str!("../fixtures/moriarty.json")).unwrap()
}
#[test]
fn frozen_core5_fixture_layout_and_constraints() {
    let f = fixture();
    for row in &f["fixtures"].as_array().unwrap()[..2] {
        let values = fixture_public_values(row).unwrap();
        assert_eq!(PUBLIC_VERSION, "moriarty-transfer-numerical/1");
        assert_eq!(values.len(), PUBLIC_FIELDS.len());
        assert_eq!(values[1], 10000);
        assert_eq!(
            values[2],
            row["expected"]["payer"]
                .as_str()
                .unwrap()
                .parse::<u128>()
                .unwrap()
        );
        let circuit = TransferCircuit::new(values);
        MockProver::run(&circuit, vec![circuit.instances()])
            .unwrap()
            .assert_satisfied();
    }
}
#[test]
fn every_numerical_field_is_constrained_and_bad_effect_order_refuses() {
    let f = fixture();
    let values = fixture_public_values(&f["fixtures"][0]).unwrap();
    for index in 0..PUBLIC_FIELDS.len() {
        // Some caps/window endpoints can vary while the relation remains valid.
        // Public inputs must nevertheless equal the committed witness values.
        let circuit = TransferCircuit::new(values);
        let mut public = circuit.instances();
        public[index] += midnight_curves::Fq::from(1);
        assert!(
            MockProver::run(&circuit, vec![public])
                .unwrap()
                .verify()
                .is_err(),
            "unbound {}",
            PUBLIC_FIELDS[index]
        );
    }
    let mut altered = f["fixtures"][0].clone();
    altered["expected_effects"]
        .as_array_mut()
        .unwrap()
        .swap(1, 2);
    assert!(fixture_public_values(&altered).is_err());
}
#[test]
fn invalid_economics_reject_without_host_success_flags() {
    let values = fixture_public_values(&fixture()["fixtures"][0]).unwrap();
    for (index, bad) in [
        (2, 8991),
        (4, 999),
        (6, 9),
        (9, 1011),
        (10, 1009),
        (11, 9),
        (12, 1001),
        (13, 11),
        (14, 2),
        (15, 0),
        (17, 10),
        (19, 2),
        (21, 8991),
        (23, 1009),
        (24, 1000),
        (25, 10),
        (26, 1000),
        (27, 1000),
    ] {
        let mut invalid = values;
        invalid[index] = bad;
        let circuit = TransferCircuit::new(invalid);
        assert!(
            MockProver::run(&circuit, vec![circuit.instances()])
                .unwrap()
                .verify()
                .is_err(),
            "accepted invalid {}",
            PUBLIC_FIELDS[index]
        );
    }
}

fn run_constraint_diagnostics(
    values: [u128; 28],
) -> Result<(), Vec<midnight_proofs::dev::VerifyFailure>> {
    let c = TransferCircuit::new(values);
    let (k, _) = midnight_proofs::dev::RowSizer::min_k(&c, vec![c.instances()]).unwrap();
    assert!(k <= 10, "numerical circuit exceeds reviewed proposal");
    MockProver::run(&c, vec![c.instances()]).unwrap().verify()
}
fn large_valid(amount: u128) -> [u128; 28] {
    let mut v = fixture_public_values(&fixture()["fixtures"][0]).unwrap();
    v[1] = amount;
    v[2] = 0;
    v[3] = u128::MAX - amount;
    v[4] = u128::MAX;
    v[5] = u128::MAX;
    v[6] = u128::MAX;
    v[7] = amount;
    v[8] = 0;
    v[9] = amount;
    v[10] = amount;
    v[11] = 0;
    v[12] = amount;
    v[16] = u128::MAX;
    v[17] = u128::MAX - 1;
    v[18] = 0;
    v[19] = 1;
    v[20] = amount;
    v[21] = 0;
    v[22] = u128::MAX - amount;
    v[23] = u128::MAX;
    v[24] = amount;
    v[25] = amount;
    v[26] = 0;
    v[27] = amount;
    v
}
#[test]
fn integer_boundaries_overflow_underflow_and_zero_amount() {
    for amount in [
        1,
        (1u128 << 64) - 1,
        1u128 << 64,
        (1u128 << 64) + 1,
        (1u128 << 127) - 1,
    ] {
        assert!(run_constraint_diagnostics(large_valid(amount)).is_ok());
    }
    assert!(run_constraint_diagnostics(large_valid(1u128 << 127)).is_err());
    let v = large_valid(1);
    for changes in [
        vec![(1, 0)],
        vec![(3, u128::MAX), (4, 0)],
        vec![(5, u128::MAX), (8, 1), (9, 2), (6, 0)],
        vec![(18, 1), (19, 2)],
        vec![(22, u128::MAX), (23, 0)],
        vec![(7, 0)],
        vec![(7, 1u128 << 127)],
        vec![(10, 1u128 << 127)],
        vec![(11, 1u128 << 127)],
        vec![(12, 1u128 << 127)],
    ] {
        let mut invalid = v;
        for (i, x) in changes {
            invalid[i] = x;
        }
        assert!(run_constraint_diagnostics(invalid).is_err());
    }
    // Isolate zero amount: all numerical effects/deltas would otherwise agree.
    let mut zero = large_valid(1);
    for i in [1, 7, 9, 10, 12, 20, 24, 25, 27] {
        zero[i] = 0;
    }
    zero[3] = u128::MAX;
    zero[22] = u128::MAX;
    assert!(run_constraint_diagnostics(zero).is_err());
}
#[test]
fn out_of_uint128_range_and_negative_field_values_reject() {
    let v = large_valid(1);
    for outside in [
        moriarty_midnight_crypto::financial_transfer::field(u128::MAX)
            + midnight_curves::Fq::from(1),
        -midnight_curves::Fq::from(1),
    ] {
        let mut c = TransferCircuit::new(v);
        c.replace_witness_field(1, outside);
        c.replace_witness_field(2, outside - midnight_curves::Fq::from(1));
        assert!(
            MockProver::run(&c, vec![c.instances()])
                .unwrap()
                .verify()
                .is_err()
        );
    }
}
#[test]
fn row_budget_is_bounded_without_native_proof_setup() {
    let c = TransferCircuit::new(large_valid(1));
    let (k, n) = midnight_proofs::dev::RowSizer::min_k(&c, vec![c.instances()]).unwrap();
    assert!(k <= 10);
    println!(
        "numerical circuit RowSizer k={k}, domain rows={n}; constraint diagnostics only, no SRS/proof"
    );
}

/// Explicitly ignored: root must obtain independent exact-source/resource votes
/// before enabling this command. Only an ephemeral unsafe local test SRS is used.
#[test]
#[ignore = "native setup/proving held for independently reviewed resource proposal"]
fn native_transfer_proof_bounded_k10() {
    use blake2b_simd::State;
    use midnight_curves::{Bls12, Fq};
    use midnight_proofs::{
        plonk::{create_proof, keygen_pk, keygen_vk_with_k, prepare},
        poly::{
            commitment::Guard,
            kzg::{KZGCommitmentScheme, params::ParamsKZG},
        },
        transcript::{CircuitTranscript, Transcript},
    };
    use rand::rngs::OsRng;
    type Scheme = KZGCommitmentScheme<Bls12>;
    let c = TransferCircuit::new(fixture_public_values(&fixture()["fixtures"][0]).unwrap());
    let instances = c.instances();
    let (k, _) = midnight_proofs::dev::RowSizer::min_k(&c, vec![instances.clone()]).unwrap();
    assert!(k <= 10);
    let params = ParamsKZG::<Bls12>::unsafe_setup(k, OsRng);
    let vk = keygen_vk_with_k::<Fq, Scheme, _>(&params, &c.without_witnesses(), k).unwrap();
    let pk = keygen_pk(vk, &c.without_witnesses()).unwrap();
    let mut writer = CircuitTranscript::<State>::init();
    create_proof::<Fq, Scheme, _, _>(&params, &pk, &c, 0, &[&instances], &mut writer, OsRng)
        .unwrap();
    let proof = writer.finalize();
    let check = |bytes: &[u8], public: &[Fq]| {
        let mut reader = CircuitTranscript::<State>::init_from_bytes(bytes);
        let result = prepare(pk.get_vk(), &[], &[public], &mut reader);
        match result {
            Ok(guard) => {
                reader.assert_empty().is_ok() && guard.verify(&params.verifier_params()).is_ok()
            }
            Err(_) => false,
        }
    };
    assert!(check(&proof, &instances));
    for i in 0..instances.len() {
        let mut changed = instances.clone();
        changed[i] += Fq::from(1);
        assert!(
            !check(&proof, &changed),
            "public field {}",
            PUBLIC_FIELDS[i]
        );
    }
    let mut corrupt = proof.clone();
    corrupt[0] ^= 1;
    assert!(!check(&corrupt, &instances));
    assert!(!check(&proof[..proof.len() / 2], &instances));
    let mut trailing = proof.clone();
    trailing.push(0);
    assert!(!check(&trailing, &instances));
    println!(
        "Numerical precursor real KZG proof bytes={} k={k}; pairing and strict EOF checked; no signature/hash/registry/source correspondence/Preview claim",
        proof.len()
    );
}

#[test]
fn arithmetic_slack_hints_cannot_override_constraints() {
    let v = fixture_public_values(&fixture()["fixtures"][0]).unwrap();
    for index in 28..36 {
        let mut c = TransferCircuit::new(v);
        // Deliberately wrong but in-range slack, no host rejection involved.
        c.replace_witness_field(index, midnight_curves::Fq::from(42));
        assert!(
            MockProver::run(&c, vec![c.instances()])
                .unwrap()
                .verify()
                .is_err()
        );
    }
}
