//! M7 blocking check: does ledger 9 `VerifierKey::verify` accept a key whose constraint
//! system is not a `ZkStdLib` architecture (the G4 lane shape is custom gates)?
//!
//! Toy: 8 advice columns (one G4 lane), one degree-5 custom gate, one lookup into a fixed
//! table, a committed instance column plus 1,026 public inputs (the `2 + L_TX` statement),
//! the same column order and transcript (Blake2b) as a stdlib circuit. Keygen on the
//! ceremony SRS, a blinded proof (OsRng), library verify, then the ledger path:
//! `MidnightVK` framing (arch header, k, nb_pi, vk) -> `VerifierKey::deserialize` ->
//! `VerifierKey::verify(&PARAMS_VERIFIER, ..)`.
//! usage: M7_SRS=<bls_midnight_2p10..> m7_ledger_toy
use std::{fs, io::Cursor};

use ff::Field;
use group::Group;
use midnight_curves::{Bls12, G1Projective};
use midnight_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{
        Advice, Circuit, Column, ConstraintSystem, Constraints, Error, Instance, Selector,
        TableColumn, create_proof, keygen_pk, keygen_vk_with_k, prepare,
    },
    poly::{
        Rotation,
        commitment::Params,
        kzg::{KZGCommitmentScheme, params::ParamsKZG},
    },
    transcript::{CircuitTranscript, Transcript},
    utils::SerdeFormat,
};
use midnight_serialize::{Deserializable, Serializable};
use midnight_transient_crypto::proofs::{PARAMS_VERIFIER, Proof, VerifierKey};
use midnight_zk_stdlib::ZkStdLibArch;
use rand::rngs::OsRng;
use serde_json::json;

type F = midnight_curves::Fq;
type B2 = blake2b_simd::State;
const NPI: usize = 1026;
const K: u32 = 11;

#[derive(Clone, Debug)]
struct Cfg {
    adv: [Column<Advice>; 8],
    q: Selector,
    t: TableColumn,
    inst: Column<Instance>,
}

#[derive(Clone, Default)]
struct LaneToy {
    w: Vec<F>,
}

impl Circuit<F> for LaneToy {
    type Config = Cfg;
    type FloorPlanner = SimpleFloorPlanner;
    type Params = ();
    fn without_witnesses(&self) -> Self {
        LaneToy { w: vec![F::ZERO; NPI] }
    }
    fn configure(meta: &mut ConstraintSystem<F>) -> Cfg {
        let _committed = meta.instance_column();
        let inst = meta.instance_column();
        meta.enable_equality(inst);
        let adv: [Column<Advice>; 8] = std::array::from_fn(|_| meta.advice_column());
        meta.enable_equality(adv[0]);
        let q = meta.complex_selector();
        let t = meta.lookup_table_column();
        meta.create_gate("pow5", |m| {
            let a = m.query_advice(adv[0], Rotation::cur());
            let b = m.query_advice(adv[1], Rotation::cur());
            let a2 = a.clone() * a.clone();
            Constraints::with_selector(q, vec![a2.clone() * a2 * a - b])
        });
        meta.lookup("range8", |m| {
            let s = m.query_selector(q);
            let c = m.query_advice(adv[2], Rotation::cur());
            vec![(s * c, t)]
        });
        Cfg { adv, q, t, inst }
    }
    fn synthesize(&self, c: Cfg, mut l: impl Layouter<F>) -> Result<(), Error> {
        l.assign_table(
            || "t",
            |mut t| {
                for i in 0..256u64 {
                    t.assign_cell(|| "t", c.t, i as usize, || Value::known(F::from(i)))?;
                }
                Ok(())
            },
        )?;
        let cells = l.assign_region(
            || "lane",
            |mut r| {
                let mut out = vec![];
                for (i, w) in self.w.iter().enumerate() {
                    c.q.enable(&mut r, i)?;
                    let a = r.assign_advice(|| "a", c.adv[0], i, || Value::known(*w))?;
                    r.assign_advice(|| "b", c.adv[1], i, || Value::known(w.pow([5u64])))?;
                    r.assign_advice(|| "c", c.adv[2], i, || Value::known(F::from((i % 256) as u64)))?;
                    out.push(a.cell());
                }
                Ok(out)
            },
        )?;
        for (i, cell) in cells.into_iter().enumerate() {
            l.constrain_instance(cell, c.inst, i)?;
        }
        Ok(())
    }
}

fn main() {
    let srs = fs::read(std::env::var("M7_SRS").expect("M7_SRS")).unwrap();
    let mut params =
        ParamsKZG::<Bls12>::read_custom(&mut Cursor::new(srs), SerdeFormat::RawBytesUnchecked)
            .unwrap();
    params.downsize(K);
    let w: Vec<F> = (0..NPI).map(|_| F::random(OsRng)).collect();
    let circ = LaneToy { w: w.clone() };
    let vk = keygen_vk_with_k::<F, KZGCommitmentScheme<Bls12>, _>(&params, &circ, K).unwrap();
    let pk = keygen_pk(vk.clone(), &circ).unwrap();
    let proof = {
        let mut tr = CircuitTranscript::<B2>::init();
        create_proof::<F, KZGCommitmentScheme<Bls12>, CircuitTranscript<B2>, LaneToy>(
            &params, &pk, &[circ], 1, &[&[&[], &w]], &mut tr, &mut OsRng,
        )
        .unwrap();
        tr.finalize()
    };
    // Library verify with the true key (PARAMS_VERIFIER is the same ceremony's G2 part).
    let lib_ok = {
        let mut tr = CircuitTranscript::<B2>::init_from_bytes(&proof);
        let g = prepare::<F, KZGCommitmentScheme<Bls12>, CircuitTranscript<B2>>(
            &vk,
            &[&[G1Projective::identity()]],
            &[&[&w]],
            &mut tr,
        );
        match g {
            Ok(g) => tr.assert_empty().is_ok() && g.check(&params.verifier_params()),
            Err(_) => false,
        }
    };
    let mut vk_raw = vec![];
    vk.write(&mut vk_raw, SerdeFormat::Processed).unwrap();
    // Ledger path under several stdlib architecture headers.
    let archs = [
        ("poseidon,pow2range4", ZkStdLibArch { poseidon: true, nr_pow2range_cols: 4, ..Default::default() }),
        ("default", ZkStdLibArch::default()),
        ("poseidon,pow2range1", ZkStdLibArch { poseidon: true, ..Default::default() }),
    ];
    let mut rows = vec![];
    for (name, arch) in archs {
        let mut framed = vec![];
        arch.write(&mut framed).unwrap();
        framed.push(K as u8);
        framed.extend_from_slice(&(NPI as u32).to_le_bytes());
        framed.extend_from_slice(&vk_raw);
        // VerifierKey's own Serializable: u64 length prefix then the bytes.
        let mut ser = vec![];
        Serializable::serialize(&framed, &mut ser).unwrap();
        let key = VerifierKey::deserialize(&mut &ser[..], 0).unwrap();
        let pi: Vec<midnight_transient_crypto::curve::Fr> =
            w.iter().map(|x| midnight_transient_crypto::curve::Fr(*x)).collect();
        let res = key.verify(&PARAMS_VERIFIER, &Proof(proof.clone()), pi.into_iter());
        rows.push(json!({"arch": name, "framed_vk_bytes": framed.len(), "ledger_verify": format!("{res:?}")}));
    }
    println!(
        "{}",
        json!({"phase": "m7_ledger_toy", "k": K, "public_inputs": NPI, "proof_bytes": proof.len(),
               "raw_vk_bytes": vk_raw.len(), "library_verify_ok": lib_ok, "ledger": rows})
    );
}
