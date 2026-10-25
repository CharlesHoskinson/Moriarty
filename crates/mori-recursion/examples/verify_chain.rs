//! Separate read-only verifier: uses stock upstream verification, never producer modules.
use ff::{Field, PrimeField};
use group::Group;
use midnight_circuits::{
    hash::poseidon::{PoseidonChip, PoseidonState},
    instructions::hash::HashCPU,
    types::Instantiable,
    verifier::{
        Accumulator, AssignedAccumulator, BlstrsEmulation, Msm, SelfEmulation, fixed_bases,
    },
};
use midnight_curves::{Bls12, Fq};
use midnight_onchain_state::state::{ContractState, EntryPointBuf};
use midnight_proofs::{
    plonk,
    poly::kzg::{KZGCommitmentScheme, params::ParamsKZG},
    transcript::{CircuitTranscript, Transcript},
    utils::SerdeFormat,
};
use midnight_storage::db::InMemoryDB;
use midnight_transient_crypto::{
    curve::Fr,
    proofs::{PARAMS_VERIFIER, Proof},
};
use midnight_zk_stdlib::{MidnightVK, batch_verify};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, error::Error, fs, io::Cursor, path::Path, time::Instant};

type S = BlstrsEmulation;
type AP = <S as SelfEmulation>::AssignedPoint;
fn sha(b: &[u8]) -> String {
    hex::encode(Sha256::digest(b))
}
fn field(s: &str) -> Fq {
    assert_eq!(s.len(), 64);
    let raw: [u8; 32] = hex::decode(s).unwrap().try_into().unwrap();
    Option::from(Fq::from_repr(raw)).expect("canonical scalar")
}
fn instance(p: impl AsRef<Path>) -> Vec<Fq> {
    let lanes: Vec<String> = serde_json::from_slice(&fs::read(p).unwrap()).unwrap();
    lanes.iter().map(|s| field(s)).collect()
}
fn key(raw: &[u8]) -> MidnightVK {
    let mut reader = raw;
    let k = MidnightVK::read(&mut reader, SerdeFormat::Processed).unwrap();
    assert!(reader.is_empty());
    let mut bytes = Vec::new();
    k.write(&mut bytes, SerdeFormat::Processed).unwrap();
    assert_eq!(bytes, raw);
    k
}
fn domain(s: &str) -> Fq {
    let mut bytes = [0u8; 16];
    bytes[..s.len()].copy_from_slice(s.as_bytes());
    Fq::from_u128(u128::from_le_bytes(bytes))
}
fn hash(s: &str, xs: &[Fq]) -> Fq {
    <PoseidonChip<Fq> as HashCPU<Fq, Fq>>::hash(&[&[domain(s)], xs].concat())
}
fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();
    assert_eq!(
        args.len(),
        6,
        "verify_chain CHILD_DIR PARENT_DIR TERMINAL_DIR STATE SRS PINS"
    );
    let (children, parent, terminal) = (
        Path::new(&args[0]),
        Path::new(&args[1]),
        Path::new(&args[2]),
    );
    let start = Instant::now();
    let srs = fs::read(&args[4])?;
    assert_eq!(srs.len(), 201_326_980);
    assert_eq!(
        sha(&srs),
        "1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1"
    );
    let mut cursor = Cursor::new(srs);
    let params = ParamsKZG::<Bls12>::read_custom(&mut cursor, SerdeFormat::RawBytesUnchecked)?;
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
    drop(cursor);
    let vp = params.verifier_params();
    let deployment: serde_json::Value = serde_json::from_slice(&fs::read(&args[5])?)?;
    let mut vks = Vec::new();
    let mut pis = Vec::new();
    let mut proofs = Vec::new();
    let mut hashes = Vec::new();
    for (dir, name, pin) in [
        (
            children,
            "a102",
            "a6e6ed282daba635711e5d15863441a4888eb28407ec3899ee8722b5e1b02ab9",
        ),
        (
            children,
            "a80",
            "65414136eabb682558c39e8a76d8d5f436641fe2d76a87bbe88a5b26b093c104",
        ),
        (
            parent,
            "w",
            deployment["parent_vk_sha256"].as_str().unwrap(),
        ),
    ] {
        let raw = fs::read(dir.join(format!("{name}.vk")))?;
        assert_eq!(sha(&raw), pin);
        hashes.push(Sha256::digest(&raw));
        vks.push(key(&raw));
        pis.push(instance(dir.join(format!("{name}.instance.json"))));
        let proof = fs::read(dir.join(format!("{name}.proof")))?;
        println!(
            "artifact {name} vk={} proof={} instance={}",
            pin,
            sha(&proof),
            sha(&fs::read(dir.join(format!("{name}.instance.json")))?)
        );
        proofs.push(proof);
    }
    // This verifies each proof's own PLONK pairing. The carried pairing is checked below.
    batch_verify::<PoseidonState<Fq>>(&vp, &vks, &pis, &proofs)?;
    for i in 0..3 {
        let mut bad = proofs[i].clone();
        bad[0] ^= 1;
        assert!(batch_verify::<PoseidonState<Fq>>(&vp, &vks[i..=i], &pis[i..=i], &[bad]).is_err());
        let mut bad = pis[i].clone();
        bad[0] += Fq::ONE;
        assert!(
            batch_verify::<PoseidonState<Fq>>(&vp, &vks[i..=i], &[bad], &proofs[i..=i]).is_err()
        );
    }
    assert!(batch_verify::<PoseidonState<Fq>>(&vp, &vks[1..2], &pis[0..1], &proofs[0..1]).is_err());
    let (a, b, w) = (&pis[0], &pis[1], &pis[2]);
    assert_eq!((a.len(), b.len(), w.len()), (419, 419, 28));
    assert_eq!(a[3..159], b[3..159]);
    assert_eq!(a[288..417], b[159..288]);
    assert_eq!(&a[417..], &[Fq::from(3), Fq::ONE]);
    assert_eq!(&b[417..], &[Fq::ZERO, Fq::ZERO]);
    let mut limbs = Vec::new();
    for digest in &hashes[..2] {
        for half in digest.as_chunks::<16>().0 {
            limbs.push(Fq::from_u128(u128::from_le_bytes(*half)));
        }
    }
    limbs.extend([Fq::from(419), Fq::from(156), Fq::from(129)]);
    let keys_id = hash("SHK_KEYS", &limbs);
    let prog = hash("SHK_PROG", &a[3..159]);
    let pre = hash("SHK_STATE", &a[159..288]);
    let mid = hash("SHK_STATE", &a[288..417]);
    let post = hash("SHK_STATE", &b[288..417]);
    let trace = hash(
        "SHK_TRACE",
        &[
            keys_id,
            a[1],
            a[2],
            b[1],
            b[2],
            prog,
            pre,
            Fq::ZERO,
            a[0],
            Fq::from(3),
            Fq::ONE,
            mid,
            Fq::ONE,
            b[0],
            Fq::ZERO,
            Fq::ZERO,
            post,
            Fq::from(2),
            Fq::from(2),
            Fq::ONE,
        ],
    );
    assert_eq!(
        &w[..16],
        &[
            keys_id,
            a[1],
            a[2],
            b[1],
            b[2],
            prog,
            pre,
            Fq::ZERO,
            mid,
            post,
            Fq::from(2),
            a[0],
            b[0],
            Fq::from(2),
            Fq::ONE,
            trace
        ]
    );
    // Rebuild the carried accumulator from these exact two proof byte strings.
    let mut bases = BTreeMap::new();
    let mut owns = Vec::new();
    for (i, prefix) in ["sherlock_a102", "sherlock_a80"].iter().enumerate() {
        let name = format!("{prefix}_{}", &hex::encode(hashes[i])[..16]);
        let fixed = fixed_bases::<S>(&name, vks[i].vk());
        let mut transcript = CircuitTranscript::<PoseidonState<Fq>>::init_from_bytes(&proofs[i]);
        let dual =
            plonk::prepare::<Fq, KZGCommitmentScheme<Bls12>, CircuitTranscript<PoseidonState<Fq>>>(
                vks[i].vk(),
                &[&[midnight_curves::G1Projective::identity()]],
                &[&[&pis[i]]],
                &mut transcript,
            )?;
        transcript.assert_empty()?;
        owns.push(Accumulator::<S>::from_dual_msm(dual, &name, &fixed));
        bases.extend(fixed);
    }
    let mut folded = Accumulator::<S>::accumulate(&owns);
    folded.resolve_fixed_bases(&bases);
    folded.collapse();
    assert_eq!(
        <AssignedAccumulator<S> as Instantiable<Fq>>::as_public_input(&folded),
        w[16..]
    );
    let side = |fields: &[Fq]| {
        assert_eq!(fields[5], Fq::ONE);
        let point =
            <AP as Instantiable<Fq>>::from_public_input(&fields[..5]).expect("subgroup point");
        Msm::<S>::new(&[point], &[Fq::ONE], &BTreeMap::new())
    };
    let carried = Accumulator::<S>::new(side(&w[16..22]), side(&w[22..28]));
    assert!(
        carried.check(&vp, &BTreeMap::new()),
        "parent carried pairing"
    );
    let state_raw = fs::read(&args[3])?;
    let state: ContractState<InMemoryDB> = midnight_serialize::tagged_deserialize(&state_raw[..])?;
    let mut roundtrip = Vec::new();
    midnight_serialize::tagged_serialize(&state, &mut roundtrip)?;
    assert_eq!(roundtrip, state_raw);
    let op = state
        .operations
        .get(&EntryPointBuf::from(b"acceptValidity".as_slice()))
        .unwrap();
    let installed = op.latest().unwrap();
    assert_eq!(
        sha(&installed.original_bytes()),
        deployment["vk_sha256"].as_str().unwrap()
    );
    assert_eq!(
        fs::read(terminal.join("key/decider.vk"))?,
        installed.original_bytes()
    );
    let public = instance(terminal.join("terminal.public.json"));
    assert_eq!(public.len(), 15);
    let template: serde_json::Value =
        serde_json::from_slice(include_bytes!("../tests/fixtures/accept-template.json"))?;
    for (slot, value) in template["constants"].as_object().unwrap() {
        assert_eq!(
            public[slot.parse::<usize>()?],
            field(value.as_str().unwrap())
        );
    }
    let id = field("489080d3e57cacab615cd0cde4c87914b47c4ff0ab5857d865ee2474ecd45c00");
    assert_eq!(
        public[10],
        hash("SHK_STMT", &[&[id, Fq::ZERO], &w[..16]].concat())
    );
    let proof = Proof(fs::read(terminal.join("terminal.proof"))?);
    installed.verify(&PARAMS_VERIFIER, &proof, public.iter().copied().map(Fr))?;
    for i in 0..public.len() {
        let mut bad = public.clone();
        bad[i] += Fq::ONE;
        assert!(
            installed
                .verify(&PARAMS_VERIFIER, &proof, bad.into_iter().map(Fr))
                .is_err()
        );
    }
    let mut bad = proof.0.clone();
    bad[0] ^= 1;
    assert!(
        installed
            .verify(
                &PARAMS_VERIFIER,
                &Proof(bad),
                public.iter().copied().map(Fr)
            )
            .is_err()
    );
    println!(
        "PASS independent verifier: 4 proofs, carried pairing, full A2->A3->A4 statement correspondence; 23 mutations refused; seconds={:.3}",
        start.elapsed().as_secs_f64()
    );
    Ok(())
}
