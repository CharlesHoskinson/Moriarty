#![cfg(feature = "native-proof")]
// Small local PLONK/KZG field relation, not the complete S0 financial circuit.
use blake2b_simd::State;
use midnight_curves::{Bls12, Fq};
use midnight_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{
        Advice, Circuit, Column, ConstraintSystem, Constraints, Error, Instance, Selector,
        create_proof, keygen_pk, keygen_vk_with_k, prepare,
    },
    poly::{
        Rotation,
        commitment::Guard,
        kzg::{KZGCommitmentScheme, params::ParamsKZG},
    },
    transcript::{CircuitTranscript, Transcript},
};
use rand::rngs::OsRng;
#[derive(Clone, Debug)]
struct Config {
    cells: [Column<Advice>; 3],
    instance: Column<Instance>,
    enabled: Selector,
}
#[derive(Clone)]
struct DebitRelation {
    values: [Value<Fq>; 3],
}
impl Circuit<Fq> for DebitRelation {
    type Config = Config;
    type FloorPlanner = SimpleFloorPlanner;
    fn without_witnesses(&self) -> Self {
        Self {
            values: [Value::unknown(); 3],
        }
    }
    fn configure(meta: &mut ConstraintSystem<Fq>) -> Config {
        let cells = [
            meta.advice_column(),
            meta.advice_column(),
            meta.advice_column(),
        ];
        let instance = meta.instance_column();
        meta.enable_equality(instance);
        for c in cells {
            meta.enable_equality(c);
        }
        let enabled = meta.selector();
        meta.create_gate("pre = post + gross", |m| {
            let pre = m.query_advice(cells[0], Rotation::cur());
            let post = m.query_advice(cells[1], Rotation::cur());
            let gross = m.query_advice(cells[2], Rotation::cur());
            Constraints::with_selector(enabled, vec![pre - post - gross])
        });
        Config {
            cells,
            instance,
            enabled,
        }
    }
    fn synthesize(&self, c: Config, mut l: impl Layouter<Fq>) -> Result<(), Error> {
        let cells = l.assign_region(
            || "debit",
            |mut r| {
                c.enabled.enable(&mut r, 0)?;
                let mut a = Vec::new();
                for i in 0..3 {
                    a.push(r.assign_advice(|| "value", c.cells[i], 0, || self.values[i])?);
                }
                Ok(a)
            },
        )?;
        for (i, a) in cells.iter().enumerate() {
            l.constrain_instance(a.cell(), c.instance, i)?;
        }
        Ok(())
    }
}
#[test]
fn real_kzg_proof_pairs_and_rejects_wrong_public_inputs_and_bytes() {
    type Scheme = KZGCommitmentScheme<Bls12>;
    let params = ParamsKZG::<Bls12>::unsafe_setup(6, OsRng); // Ephemeral unsafe test SRS; never production.
    let values = [Fq::from(10000), Fq::from(8990), Fq::from(1010)];
    let circuit = DebitRelation {
        values: values.map(Value::known),
    };
    let vk = keygen_vk_with_k::<Fq, Scheme, _>(&params, &circuit.without_witnesses(), 6).unwrap();
    let pk = keygen_pk(vk, &circuit.without_witnesses()).unwrap();
    let mut transcript = CircuitTranscript::<State>::init();
    create_proof::<Fq, Scheme, _, _>(
        &params,
        &pk,
        &circuit,
        0,
        &[&values],
        &mut transcript,
        OsRng,
    )
    .unwrap();
    let proof = transcript.finalize();
    let check = |bytes: &[u8], instances: &[Fq]| {
        let mut t = CircuitTranscript::<State>::init_from_bytes(bytes);
        match prepare(pk.get_vk(), &[], &[instances], &mut t) {
            Ok(guard) => guard.verify(&params.verifier_params()).is_ok(),
            Err(_) => false,
        }
    };
    assert!(check(&proof, &values));
    let mut bad = values;
    bad[2] = Fq::from(1011);
    assert!(!check(&proof, &bad));
    let mut corrupted = proof.clone();
    corrupted[0] ^= 1;
    assert!(!check(&corrupted, &values));
    assert!(!check(&proof[..proof.len() / 2], &values));
    let invalid = DebitRelation {
        values: [
            Value::known(Fq::from(10000)),
            Value::known(Fq::from(8990)),
            Value::known(Fq::from(1011)),
        ],
    };
    let mut t = CircuitTranscript::<State>::init();
    let generated =
        create_proof::<Fq, Scheme, _, _>(&params, &pk, &invalid, 0, &[&bad], &mut t, OsRng);
    generated.expect("release assertion behavior must emit the invalid-witness test proof");
    assert!(!check(&t.finalize(), &bad));
    println!(
        "Real KZG proof bytes={} k=6; final pairing verified, changed inputs/bytes/truncation/invalid witness rejected; field equality only, no UInt128 range or financial compiler correspondence",
        proof.len()
    );
}
