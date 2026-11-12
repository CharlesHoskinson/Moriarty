//! M7 tests: the BLS12-381 Fr p3 field, the Midnight Poseidon permutation, the PROPOSAL 3.6
//! leaf / node / transcript rules, and one small AIR proved and verified with the engine.
//!
//! Run: cargo test --release -p stark-bench-ovm --features bls --test m7_bls_fr -- --nocapture

use ff::{Field as FfField, PrimeField as FfPrimeField};
use midnight_circuits::{
    hash::poseidon::{constants::PoseidonField, PoseidonChip},
    instructions::hash::HashCPU,
};
use num_bigint::BigUint;
use openvm_poseidon2_air::{Poseidon2Config, Poseidon2SubChip};
use openvm_stark_backend::{
    codec::{Decode, Encode},
    hasher::MerkleHasher,
    p3_field::{Field, PrimeCharacteristicRing, PrimeField, PrimeField32, TwoAdicField},
    p3_symmetric::Permutation,
    prover::{ColMajorMatrix, DeviceDataTransporter, AirProvingContext, ProvingContext},
    proof::Proof,
    transcript::multi_field::MultiFieldTranscript,
    AirRef, FiatShamirTranscript, StarkEngine,
};
use openvm_stark_sdk::config::{
    app_params_with_100_bits_security,
    baby_bear_bls12381_poseidon::{
        cap_fixed, cap_var, fixed_len_hash, pack_words, BabyBearBls12381PoseidonConfig as SC,
        BabyBearBls12381PoseidonCpuEngine as Eng, Bls12381Fr as Fr, MidnightFq as Fq,
        MidnightMerkleHasher, MidnightPoseidonFr, Transcript, InnerTranscript,
    },
};
use p3_baby_bear::BabyBear;
use rand::{rngs::StdRng, Rng, SeedableRng};

type F = BabyBear;

fn rnd_fq(rng: &mut StdRng) -> Fq {
    let mut b = [0u8; 64];
    rng.fill(&mut b[..]);
    <Fq as ff::FromUniformBytes<64>>::from_uniform_bytes(&b)
}

fn fq_big(x: &Fq) -> BigUint {
    BigUint::from_bytes_le(x.to_repr().as_ref())
}

const R_HEX: &str = "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001";

#[test]
fn field_matches_ff() {
    let r = BigUint::parse_bytes(R_HEX.as_bytes(), 16).unwrap();
    assert_eq!(Fr::order(), r);
    assert_eq!(Fr::bits(), 255);
    assert_eq!(Fr::ZERO.0, Fq::ZERO);
    assert_eq!(Fr::ONE.0, Fq::ONE);
    assert_eq!(Fr::TWO.0, Fq::ONE.double());
    assert_eq!(Fr::NEG_ONE.0, -Fq::ONE);
    assert_eq!(Fr::GENERATOR.0, Fq::MULTIPLICATIVE_GENERATOR);
    assert_eq!(Fr::TWO_ADICITY as u32, Fq::S);
    let w = Fr::two_adic_generator(32);
    assert_eq!(w.0, Fq::ROOT_OF_UNITY);
    assert_eq!(w.exp_power_of_2(32), Fr::ONE);
    assert_ne!(w.exp_power_of_2(31), Fr::ONE);
    assert_eq!(Fr::two_adic_generator(1), Fr::NEG_ONE);
    let mut rng = StdRng::seed_from_u64(7);
    for _ in 0..2000 {
        let (a, b) = (rnd_fq(&mut rng), rnd_fq(&mut rng));
        let (x, y) = (Fr(a), Fr(b));
        assert_eq!((x + y).0, a + b);
        assert_eq!((x - y).0, a - b);
        assert_eq!((x * y).0, a * b);
        assert_eq!((-x).0, -a);
        assert_eq!(x.square().0, a.square());
        assert_eq!(x.double().0, a.double());
        assert_eq!((x.halve() + x.halve()).0, a);
        if !bool::from(a.is_zero()) {
            assert_eq!(x.inverse().0, a.invert().unwrap());
            assert_eq!((y / x).0, b * a.invert().unwrap());
        }
        assert_eq!(x.as_canonical_biguint(), fq_big(&a));
        assert_eq!(Fr::from_biguint(fq_big(&a)).unwrap(), x);
        assert_eq!(Fr::from_biguint(fq_big(&a) + &r).unwrap(), x);
        assert_eq!(x.cmp(&y), fq_big(&a).cmp(&fq_big(&b)));
        // serde round trip and canonical rejection
        let js = serde_json::to_string(&x).unwrap();
        assert_eq!(serde_json::from_str::<Fr>(&js).unwrap(), x);
        let u: u128 = rng.random();
        assert_eq!(Fr::from_u128(u).as_canonical_biguint(), BigUint::from(u));
        let i: i64 = rng.random();
        let fi = Fr::from_i64(i);
        if i < 0 {
            assert_eq!(fi.as_canonical_biguint(), &r - BigUint::from(i.unsigned_abs()));
        } else {
            assert_eq!(fi.as_canonical_biguint(), BigUint::from(i as u64));
        }
        let v: Fr = rng.random();
        assert!(v.as_canonical_biguint() < r);
    }
    let mut rb = [0u8; 32];
    let rbytes = r.to_bytes_le();
    rb[..rbytes.len()].copy_from_slice(&rbytes);
    assert!(Fr::from_canonical_bytes(rb).is_none());
    let js = serde_json::to_string(&rb.to_vec()).unwrap();
    assert!(serde_json::from_str::<Fr>(&js).is_err());
    assert_eq!(
        Fr::from_u64_limbs([u64::MAX; 4]).as_canonical_biguint(),
        ((BigUint::from(1u8) << 256u32) - 1u8) % &r
    );
}

#[test]
fn packing_constants() {
    let t = InnerTranscript::new(MidnightPoseidonFr::new());
    assert_eq!(t.packing_constants(), (8, 5));
}

/// Independent no-skip Poseidon from midnight-circuits' published constants:
/// for each round r: add RC[r], S-box (all cells in full rounds, the last cell in partial
/// rounds), multiply by MDS. Rounds 0..4 and 64..68 are full.
fn reference_perm(state: &mut [Fq; 3]) {
    let rc = <Fq as PoseidonField>::ROUND_CONSTANTS;
    let mds = <Fq as PoseidonField>::MDS;
    assert_eq!(rc.len(), 68);
    for (r, k) in rc.iter().enumerate() {
        for i in 0..3 {
            state[i] += k[i];
        }
        let full = r < 4 || r >= 64;
        for i in 0..3 {
            if full || i == 2 {
                state[i] = state[i].square().square() * state[i];
            }
        }
        let s = *state;
        for i in 0..3 {
            state[i] = mds[i][0] * s[0] + mds[i][1] * s[1] + mds[i][2] * s[2];
        }
    }
}

#[test]
fn permutation_matches_midnight() {
    let p = MidnightPoseidonFr::new();
    let fixed: Vec<[Fq; 3]> = vec![
        [Fq::ZERO, Fq::ZERO, Fq::ZERO],
        [Fq::from(1u64), Fq::from(2u64), Fq::from(3u64)],
        [-Fq::ONE, -Fq::ONE, -Fq::ONE],
        [Fq::ZERO, Fq::ZERO, cap_var(3).0],
    ];
    let mut rng = StdRng::seed_from_u64(11);
    let mut all = fixed.clone();
    for _ in 0..200 {
        all.push([rnd_fq(&mut rng), rnd_fq(&mut rng), rnd_fq(&mut rng)]);
    }
    for (k, s0) in all.iter().enumerate() {
        let mut a = *s0;
        p.permute_ff(&mut a);
        let mut b = *s0;
        reference_perm(&mut b);
        assert_eq!(a, b, "vector {k}");
        let c = p.permute(s0.map(Fr));
        assert_eq!(c.map(|x| x.0), a);
        if k < fixed.len() {
            println!("PERM_VECTOR {k}: in {:?} out {:?}", s0, a);
        }
    }
}

#[test]
fn sponges_match_midnight_and_spec() {
    let p = MidnightPoseidonFr::new();
    let mut rng = StdRng::seed_from_u64(13);
    // u = 0 is Midnight's native fixed-length hash.
    for l in 0..9usize {
        let xs: Vec<Fq> = (0..l).map(|_| rnd_fq(&mut rng)).collect();
        let native = <PoseidonChip<Fq> as HashCPU<Fq, Fq>>::hash(&xs);
        let ours = fixed_len_hash(&p, 0, &xs.iter().copied().map(Fr).collect::<Vec<_>>());
        assert_eq!(ours.0, native, "u=0 l={l}");
        let tagged = fixed_len_hash(&p, 1, &xs.iter().copied().map(Fr).collect::<Vec<_>>());
        if l > 0 {
            assert_ne!(tagged, ours);
        }
    }
    assert_eq!(cap_fixed(1, 5).as_canonical_biguint(), BigUint::from(5u8) + (BigUint::from(1u8) << 66u32));
    assert_eq!(
        cap_var(3).as_canonical_biguint(),
        (BigUint::from(1u8) << 64u32) + BigUint::from(3u8) * (BigUint::from(1u8) << 66u32)
    );
    // Leaf (u = 1): pack 8 words per Fr, then the fixed-length sponge.
    let h = MidnightMerkleHasher::new(p.clone());
    for w in [1usize, 2, 7, 8, 9, 15, 16, 17, 24, 33, 100] {
        let words: Vec<F> = (0..w).map(|_| F::from_u32(rng.random::<u32>() % F::ORDER_U32)).collect();
        let packed: Vec<Fr> = words.chunks(8).map(pack_words).collect();
        assert_eq!(packed.len(), w.div_ceil(8));
        let expect = fixed_len_hash(&p, 1, &packed);
        assert_eq!(h.hash_slice(&words)[0], expect, "leaf w={w}");
        // pack check: sum b_i 2^(31 i)
        let mut acc = BigUint::from(0u8);
        for (i, x) in words[..w.min(8)].iter().enumerate() {
            acc += BigUint::from(x.as_canonical_u32()) << (31 * i);
        }
        assert_eq!(packed[0].as_canonical_biguint(), acc);
    }
    // Node (u = 2).
    for _ in 0..20 {
        let (a, b) = (Fr(rnd_fq(&mut rng)), Fr(rnd_fq(&mut rng)));
        assert_eq!(h.compress([a], [b])[0], fixed_len_hash(&p, 2, &[a, b]));
        let mut s = [a.0, b.0, Fq::from(2u64) + Fq::from_u128(2u128 << 66)];
        p.permute_ff(&mut s);
        assert_eq!(h.compress([a], [b])[0].0, s[0]);
    }
}

#[test]
fn transcript_rules() {
    let p = MidnightPoseidonFr::new();
    // First squeeze of an empty tagged transcript: perm(0, 0, 2^64 + 3 * 2^66), output s_1,
    // digit b_0 = s_1 mod p.
    let mut t = Transcript::new(p.clone());
    let s0 = FiatShamirTranscript::<SC>::sample(&mut t);
    let st = p.permute([Fr::ZERO, Fr::ZERO, cap_var(3)]);
    let pbig = BigUint::from(F::ORDER_U32);
    let s1 = st[1].as_canonical_biguint();
    assert_eq!(BigUint::from(s0.as_canonical_u32()), &s1 % &pbig);
    let s0b = FiatShamirTranscript::<SC>::sample(&mut t);
    assert_eq!(BigUint::from(s0b.as_canonical_u32()), (&s1 / &pbig) % &pbig);
    // With a zero initial state the transcript equals the stock one; with the tag it differs.
    let ops = |t: &mut dyn FnMut(u8, u32) -> Option<u32>| {
        let mut out = vec![];
        let mut rng = StdRng::seed_from_u64(17);
        for _ in 0..300 {
            let op = rng.random::<u8>() % 3;
            let v = rng.random::<u32>() % F::ORDER_U32;
            if let Some(x) = t(op, v) {
                out.push(x);
            }
        }
        out
    };
    let run = |mut tr: InnerTranscript| {
        let mut f = move |op: u8, v: u32| -> Option<u32> {
            match op {
                0 => {
                    FiatShamirTranscript::<SC>::observe(&mut tr, F::from_u32(v));
                    None
                }
                1 => Some(FiatShamirTranscript::<SC>::sample(&mut tr).as_canonical_u32()),
                _ => {
                    FiatShamirTranscript::<SC>::observe_commit(&mut tr, [Fr::from_u32(v)]);
                    None
                }
            }
        };
        ops(&mut f)
    };
    let zero_state = run(InnerTranscript::new_with_sponge_state(p.clone(), [Fr::ZERO; 3]));
    let stock = run(MultiFieldTranscript::new(p.clone()));
    assert_eq!(zero_state, stock);
    let tagged = run(Transcript::new(p.clone()).0);
    assert_ne!(tagged, stock);
    assert_eq!(tagged.len(), stock.len());
}

#[test]
fn small_air_proves_and_verifies() {
    let mut rng = StdRng::seed_from_u64(0x5e7);
    let chip = Poseidon2SubChip::<F, 1>::new(Poseidon2Config::<F>::default().constants);
    let airs: Vec<AirRef<SC>> = vec![chip.air.clone()];
    let engine = Eng::new(app_params_with_100_bits_security(12));
    let (pk, vk) = engine.keygen(&airs);
    let d_pk = engine.device().transport_pk_to_device(&pk);
    let inputs: Vec<[F; 16]> = (0..1usize << 8)
        .map(|_| std::array::from_fn(|_| F::from_u32(rng.random::<u32>() % (1 << 30))))
        .collect();
    let trace = chip.generate_trace(inputs);
    let cm = ColMajorMatrix::from_row_major(&trace);
    let ctx = ProvingContext::new(vec![(
        0,
        AirProvingContext::simple_no_pis(engine.device().transport_matrix_to_device(&cm)),
    )]);
    let proof = engine.prove(&d_pk, ctx).expect("prove");
    engine.verify(&vk, &proof).expect("verify");
    let bytes = proof.encode_to_vec().unwrap();
    let back = Proof::<SC>::decode_from_bytes(&bytes).unwrap();
    engine.verify(&vk, &back).expect("verify decoded");
    // tampered commitment is rejected
    let mut bad = back.clone();
    bad.common_main_commit = [bad.common_main_commit[0] + Fr::ONE];
    assert!(engine.verify(&vk, &bad).is_err());
    println!("SMALL_AIR proof bytes {}", bytes.len());
}
