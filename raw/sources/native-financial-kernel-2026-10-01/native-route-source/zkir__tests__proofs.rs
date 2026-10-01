// This file is part of midnight-ledger.
// Copyright (C) Midnight Foundation
// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[path = "common/mod.rs"]
mod common;

#[cfg(test)]
mod proof_tests {
    use super::common::{TestParams, TestResolver};
    use group::{Group, ff::Field};
    use midnight_curves::{Fr as JubjubFr, JubjubSubgroup, curve25519, k256, p256};
    use midnight_zkir::{
        Identifier, IrSource, Preprocessed, ir_instructions::encode::encode_offcircuit,
        ir_types::IrValue,
    };
    use rand::{SeedableRng, rngs::OsRng};
    use rand_chacha::ChaCha20Rng;
    #[cfg(feature = "proptest")]
    use serialize::randomised_serialization_test;
    use serialize::{Deserializable, Serializable};
    use std::borrow::Cow;
    use std::collections::HashMap;
    use transient_crypto::curve::EmbeddedGroupAffine;
    use transient_crypto::hash::transient_hash;
    use transient_crypto::proofs::Proof;
    #[cfg(feature = "proptest")]
    use transient_crypto::proofs::{
        KeyLocation, PARAMS_VERIFIER, ProofPreimage, VerifierKey, Zkir,
    };

    type ProverKey = transient_crypto::proofs::ProverKey<IrSource>;

    #[actix_rt::test]
    async fn test_extension_attack() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v_0", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "assert", "cond": "%v_0" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        const N: u64 = 512;
        let proof = ir
            .prove_unchecked(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                pk,
                Preprocessed {
                    memory: HashMap::from([(
                        Identifier("v0".to_string()),
                        IrValue::Native(1.into()),
                    )]),
                    pis: (0..N).map(Into::into).collect(),
                    pi_skips: vec![],
                    binding_input: 0.into(),
                    comm_comm: None,
                },
            )
            .await;
        // Either proving should have failed, or verification should fail:
        let verify =
            proof.and_then(|proof| vk.verify(&PARAMS_VERIFIER, &proof, (0..N).map(Into::into)));
        assert!(verify.is_err());
    }

    #[actix_rt::test]
    async fn test_minimal_proof() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v_0", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "assert", "cond": "%v_0" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let mut pk_data = Vec::new();
        let mut vk_data = Vec::new();
        Serializable::serialize(&pk, &mut pk_data).unwrap();
        Serializable::serialize(&vk, &mut vk_data).unwrap();
        let pk_fmt = format!("{:#?}", pk);
        let vk_fmt = format!("{:#?}", vk);
        let pk: ProverKey = Deserializable::deserialize(&mut &pk_data[..], 0).unwrap();
        let vk: VerifierKey = Deserializable::deserialize(&mut &vk_data[..], 0).unwrap();
        pk.init().unwrap();
        vk.init().unwrap();
        dbg!(pk_fmt == format!("{:#?}", pk));
        dbg!(vk_fmt == format!("{:#?}", vk));
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: vec![1.into()],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
        assert!(
            vk.verify(&PARAMS_VERIFIER, &proof, [43.into()].into_iter())
                .is_err()
        );
    }

    #[actix_rt::test]
    async fn test_htc_proof() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v_0", "type": "Scalar<BLS12-381>" },
              { "name": "%v_1", "type": "Scalar<BLS12-381>" },
              { "name": "%v_2", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "hash_to_curve", "inputs": ["%v_0", "%v_1", "%v_2"], "output": "%p_0" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let mut pk_data = Vec::new();
        let mut vk_data = Vec::new();
        Serializable::serialize(&pk, &mut pk_data).unwrap();
        Serializable::serialize(&vk, &mut vk_data).unwrap();
        let pk_fmt = format!("{:#?}", pk);
        let pk: ProverKey = Deserializable::deserialize(&mut &pk_data[..], 0).unwrap();
        pk.init().unwrap();
        dbg!(pk_fmt == format!("{:#?}", pk));
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: vec![1.into(), 2.into(), 3.into()],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    // Note: The impact instruction here doesn't correspond to real Impact VM bytecode.
    // Real impact instructions contain encoded opcodes (0x10 for push, 0x30 for dup, etc.).
    // We're keeping this simplified form for historical reasons - it still exercises the
    // prover's public input handling even if it's not a semantically valid Impact program.
    #[actix_rt::test]
    async fn test_hash_proof() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v_0", "type": "Scalar<BLS12-381>" },
              { "name": "%v_1", "type": "Scalar<BLS12-381>" },
              { "name": "%v_2", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "transient_hash", "inputs": ["%v_0", "%v_1", "%v_2"], "output": "%v_3" },
               { "op": "impact", "guard": "0x01", "inputs": ["%v_3"] }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();
        let x = transient_hash(&[1.into(), 2.into(), 3.into()]);

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let mut pk_data = Vec::new();
        let mut vk_data = Vec::new();
        Serializable::serialize(&pk, &mut pk_data).unwrap();
        Serializable::serialize(&vk, &mut vk_data).unwrap();
        let pk_fmt = format!("{:#?}", pk);
        let pk: ProverKey = Deserializable::deserialize(&mut &pk_data[..], 0).unwrap();
        pk.init().unwrap();
        dbg!(pk_fmt == format!("{:#?}", pk));
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: vec![1.into(), 2.into(), 3.into()],
            private_transcript: vec![],
            public_transcript_inputs: vec![x],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into(), x].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_std_hashes_proof() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v_0", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "persistent_hash", "alignment": [ { "tag": "atom", "value": { "tag": "bytes", "length": 1 } } ], "inputs": ["%v_0"], "output": "%v_1" },
               { "op": "keccak256", "alignment": [ { "tag": "atom", "value": { "tag": "bytes", "length": 1 } } ], "inputs": ["%v_0"], "output": "%v_2" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let mut pk_data = Vec::new();
        let mut vk_data = Vec::new();
        Serializable::serialize(&pk, &mut pk_data).unwrap();
        Serializable::serialize(&vk, &mut vk_data).unwrap();
        let pk_fmt = format!("{:#?}", pk);
        let pk: ProverKey = Deserializable::deserialize(&mut &pk_data[..], 0).unwrap();
        pk.init().unwrap();
        dbg!(pk_fmt == format!("{:#?}", pk));
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: vec![(42).into()],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_sha512_proof() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v_0", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "sha512", "alignment": [ { "tag": "atom", "value": { "tag": "bytes", "length": 1 } } ], "inputs": ["%v_0"], "output": "%v_1" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let mut pk_data = Vec::new();
        let mut vk_data = Vec::new();
        Serializable::serialize(&pk, &mut pk_data).unwrap();
        Serializable::serialize(&vk, &mut vk_data).unwrap();
        let pk_fmt = format!("{:#?}", pk);
        let pk: ProverKey = Deserializable::deserialize(&mut &pk_data[..], 0).unwrap();
        pk.init().unwrap();
        dbg!(pk_fmt == format!("{:#?}", pk));
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: vec![(42).into()],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_ec_proof() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%p0", "type": "Point<Jubjub>" },
              { "name": "%s0", "type": "Scalar<BLS12-381>" },
              { "name": "%s1", "type": "Scalar<Jubjub>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "jubjub_scalar_from_native", "native": "%s0", "output": "%s0d" },
               { "op": "encode", "input": "%s0d", "outputs": ["%s0e"] },
               { "op": "ec_mul", "a": "%p0", "scalar": "%s0d", "output": "%p1" },
               { "op": "ec_mul_generator", "scalar": "%s1", "output": "%p2" },
               { "op": "add", "a": "%p1", "b": "%p2", "output": "%p3" },
               { "op": "private_input", "type": "Point<Jubjub>", "guard": null, "output": "%p4" },
               { "op": "ec_mul", "a": "%p4", "scalar": "%s0d", "output": "%p5" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let mut pk_data = Vec::new();
        let mut vk_data = Vec::new();
        Serializable::serialize(&pk, &mut pk_data).unwrap();
        Serializable::serialize(&vk, &mut vk_data).unwrap();
        let pk_fmt = format!("{:#?}", pk);
        let pk: ProverKey = Deserializable::deserialize(&mut &pk_data[..], 0).unwrap();
        pk.init().unwrap();
        dbg!(pk_fmt == format!("{:#?}", pk));
        let mut pk_data = Vec::new();
        let mut vk_data = Vec::new();
        Serializable::serialize(&pk, &mut pk_data).unwrap();
        Serializable::serialize(&vk, &mut vk_data).unwrap();
        let pk_fmt = format!("{:#?}", pk);
        let pk: ProverKey = Deserializable::deserialize(&mut &pk_data[..], 0).unwrap();
        pk.init().unwrap();
        dbg!(pk_fmt == format!("{:#?}", pk));
        let p = EmbeddedGroupAffine::generator();
        let q: EmbeddedGroupAffine = JubjubSubgroup::random(OsRng).into();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: vec![p.x().unwrap(), p.y().unwrap(), (-1).into(), 63.into()],
            private_transcript: vec![q.x().unwrap(), q.y().unwrap()],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    /// Exercises the arithmetic, equality and selection instructions on
    /// `Scalar<Jubjub>` in-circuit. These became expressible once
    /// midnight-circuits implemented `ArithInstructions`, `EqualityInstructions`,
    /// `AssertionInstructions` and `ControlFlowInstructions` for the native
    /// Edwards chip's `AssignedScalar`.
    ///
    /// The circuit checks two identities that pin down every instruction under
    /// test: `a + (-a) == 0` and `(a * b) * b^-1 == a`.
    #[actix_rt::test]
    async fn test_jubjub_scalar_arith_proof() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%a", "type": "Scalar<Jubjub>" },
              { "name": "%b", "type": "Scalar<Jubjub>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "neg", "a": "%a", "output": "%na" },
               { "op": "add", "a": "%a", "b": "%na", "output": "%zero" },
               { "op": "jubjub_scalar_from_native", "native": "0x00", "output": "%c0" },
               { "op": "constrain_eq", "a": "%zero", "b": "%c0" },
               { "op": "mul", "a": "%a", "b": "%b", "output": "%ab" },
               { "op": "inv", "a": "%b", "output": "%binv" },
               { "op": "mul", "a": "%ab", "b": "%binv", "output": "%a2" },
               { "op": "constrain_eq", "a": "%a", "b": "%a2" },
               { "op": "test_eq", "a": "%a", "b": "%a2", "output": "%t" },
               { "op": "assert", "cond": "%t" },
               { "op": "cond_select", "bit": "%t", "a": "%a2", "b": "%b", "output": "%sel" },
               { "op": "ec_mul_generator", "scalar": "%sel", "output": "%p" },
               { "op": "ec_mul_generator", "scalar": "%a", "output": "%q" },
               { "op": "constrain_eq", "a": "%p", "b": "%q" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: vec![12345.into(), 6789.into()],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    /// Exercises `to_bytes` / `from_bytes` on `Scalar<Jubjub>`. Unlike the
    /// other field types, a Jubjub scalar is held in-circuit as a bit vector
    /// that is not constrained to be canonical, so both directions go through
    /// an explicit reduction modulo the group order.
    ///
    /// The circuit round-trips a scalar through both instructions, pins the
    /// byte encodings of a witnessed scalar and of a small constant against the
    /// off-circuit reference, and reduces an all-`0xff` byte string, which
    /// exceeds the group order.
    #[actix_rt::test]
    async fn test_jubjub_scalar_bytes_proof() {
        use midnight_zkir::ir_instructions::from_bytes::from_bytes_offcircuit;
        use midnight_zkir::ir_instructions::to_bytes::to_bytes_offcircuit;
        use midnight_zkir::ir_types::IrType;

        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%s",   "type": "Scalar<Jubjub>" },
              { "name": "%raw", "type": "Bytes<32>"      }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "to_bytes", "input": "%s", "output": "%s_bytes" },
               { "op": "from_bytes", "bytes": "%s_bytes", "type": "Scalar<Jubjub>", "output": "%s_back" },
               { "op": "constrain_eq", "a": "%s_back", "b": "%s" },

               { "op": "private_input", "type": "Bytes<32>", "guard": null, "output": "%s_bytes_exp" },
               { "op": "constrain_eq", "a": "%s_bytes", "b": "%s_bytes_exp" },

               { "op": "from_bytes", "bytes": "%raw", "type": "Scalar<Jubjub>", "output": "%raw_scalar" },
               { "op": "private_input", "type": "Scalar<Jubjub>", "guard": null, "output": "%raw_scalar_exp" },
               { "op": "constrain_eq", "a": "%raw_scalar", "b": "%raw_scalar_exp" },

               { "op": "load_constant", "type": "Scalar<Jubjub>", "encoding": ["0x07"], "output": "%c" },
               { "op": "to_bytes", "input": "%c", "output": "%c_bytes" },
               { "op": "private_input", "type": "Bytes<32>", "guard": null, "output": "%c_bytes_exp" },
               { "op": "constrain_eq", "a": "%c_bytes", "b": "%c_bytes_exp" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let scalar_val = JubjubFr::random(OsRng);
        let raw_bytes = [0xffu8; 32];

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        let inputs: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::JubjubScalar(scalar_val)),
            encode(IrValue::Bytes(raw_bytes.to_vec())),
        ]
        .concat();

        let scalar_bytes_exp = to_bytes_offcircuit(&IrValue::JubjubScalar(scalar_val)).unwrap();
        let raw_scalar_exp = from_bytes_offcircuit(&IrType::JubjubScalar, &raw_bytes).unwrap();
        // A small constant is assigned from a short bit vector, so its byte
        // encoding exercises the zero-padding path of `to_bytes`.
        let const_bytes_exp =
            to_bytes_offcircuit(&IrValue::JubjubScalar(JubjubFr::from(7))).unwrap();

        let private_transcript: Vec<transient_crypto::curve::Fr> = [
            encode(scalar_bytes_exp),
            encode(raw_scalar_exp),
            encode(const_bytes_exp),
        ]
        .concat();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_divmod_proof() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v_0", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "div_mod_power_of_two", "val": "%v_0", "bits": 3, "outputs": ["%v_1", "%v_2"] },
               { "op": "private_input", "type": "Scalar<BLS12-381>", "guard": null, "output": "%v_3" },
               { "op": "private_input", "type": "Scalar<BLS12-381>", "guard": null, "output": "%v_4" },
               { "op": "constrain_eq", "a": "%v_1", "b": "%v_3" },
               { "op": "constrain_eq", "a": "%v_2", "b": "%v_4" },
               { "op": "reconstitute_field", "divisor": "%v_1", "modulus": "%v_2", "bits": 3, "output": "%v_5" },
               { "op": "constrain_eq", "a": "%v_5", "b": "%v_0" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let mut pk_data = Vec::new();
        let mut vk_data = Vec::new();
        Serializable::serialize(&pk, &mut pk_data).unwrap();
        Serializable::serialize(&vk, &mut vk_data).unwrap();
        let pk_fmt = format!("{:#?}", pk);
        let vk_fmt = format!("{:#?}", vk);
        let pk: ProverKey = Deserializable::deserialize(&mut &pk_data[..], 0).unwrap();
        let vk: VerifierKey = Deserializable::deserialize(&mut &vk_data[..], 0).unwrap();
        pk.init().unwrap();
        vk.init().unwrap();
        dbg!(pk_fmt == format!("{:#?}", pk));
        dbg!(vk_fmt == format!("{:#?}", vk));
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: vec![20.into()],
            private_transcript: vec![2.into(), 4.into()],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_keygen_and_serialize_eq() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v_0", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "assert", "cond": "%v_0" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();
        let vk_kzg1 = ir.keygen_vk(&TestParams).await.unwrap();
        let vk_kzg2 = ir.keygen_vk(&TestParams).await.unwrap();
        assert_eq!(&vk_kzg1, &vk_kzg2);
        let mut bytes = Vec::new();
        serialize::tagged_serialize(&vk_kzg1, &mut bytes).unwrap();
        let vk_kzg3: VerifierKey = serialize::tagged_deserialize(&mut &bytes[..]).unwrap();
        assert_eq!(&vk_kzg1, &vk_kzg3);
    }

    #[cfg(feature = "proptest")]
    randomised_serialization_test!(VerifierKey);
    #[cfg(feature = "proptest")]
    randomised_serialization_test!(Proof);

    #[actix_rt::test]
    async fn test_immediate_values() {
        // v_2 = v_0 + 5, constrain_eq(v_1, v_2)
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v_0", "type": "Scalar<BLS12-381>" },
              { "name": "%v_1", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "add", "a": "%v_0", "b": "0x05", "output": "%v_2" },
               { "op": "constrain_eq", "a": "%v_1", "b": "%v_2" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        // Test with v_0 = 10, v_1 = 15
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: vec![10.into(), 15.into()],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_immediate_add_and_cond_select() {
        // v_2 = v_0 + 1, v_3 = test_eq(v_1, v_2), assert(v_3), v_4 = v_3 ? 2 : 3
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v_0", "type": "Scalar<BLS12-381>" },
              { "name": "%v_1", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "add", "a": "%v_0", "b": "0x01", "output": "%v_2" },
               { "op": "test_eq", "a": "%v_1", "b": "%v_2", "output": "%v_3" },
               { "op": "assert", "cond": "%v_3" },
               { "op": "cond_select", "bit": "%v_3", "a": "0x02", "b": "0x03", "output": "%v_4" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        // v_0 = 5, v_1 = 6
        let preimage = ProofPreimage {
            binding_input: 99.into(),
            communications_commitment: None,
            inputs: vec![5.into(), 6.into()],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [99.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_immediate_copy() {
        // v_1 = copy(0x42), constrain_eq(v_0, v_1)
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v_0", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "copy", "val": "0x42", "output": "%v_1" },
               { "op": "constrain_eq", "a": "%v_0", "b": "%v_1" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        // Input must be 0x42 = 66 for proof to succeed
        let preimage = ProofPreimage {
            binding_input: 123.into(),
            communications_commitment: None,
            inputs: vec![66.into()],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [123.into()].into_iter())
            .unwrap();
    }

    // Note: Same as test_hash_proof - the impact instruction here is not real Impact VM
    // bytecode, just a simplified test case kept for historical reasons.
    #[actix_rt::test]
    async fn test_immediate_with_public_inputs() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v_0", "type": "Scalar<BLS12-381>" },
              { "name": "%v_1", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "constrain_bits", "val": "%v_0", "bits": 8 },
               { "op": "constrain_bits", "val": "%v_1", "bits": 248 },
               { "op": "cond_select", "bit": "%v_0", "a": "0x00", "b": "0x01", "output": "%v_2" },
               { "op": "assert", "cond": "%v_2" },
               { "op": "impact", "guard": "0x01", "inputs": ["0x30"] }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        let preimage = ProofPreimage {
            binding_input: 48.into(),
            communications_commitment: None,
            inputs: vec![0.into(), 42.into()],
            private_transcript: vec![],
            public_transcript_inputs: vec![48.into()],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [48.into(), 48.into()].into_iter())
            .unwrap();
    }

    // Regression test: a guarded-off `impact` must contribute a *zeroed* public
    // input, and the `prepare` and `synthesize` runs must agree on this. A guard
    // of "0x00" zeroes the `0x30` input, so the public input vector is
    // [binding_input, 0] and the value is recorded as skipped.
    #[actix_rt::test]
    async fn test_impact_guarded_off_zeroes_public_input() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v_0", "type": "Scalar<BLS12-381>" },
              { "name": "%v_1", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "constrain_bits", "val": "%v_0", "bits": 8 },
               { "op": "constrain_bits", "val": "%v_1", "bits": 248 },
               { "op": "cond_select", "bit": "%v_0", "a": "0x00", "b": "0x01", "output": "%v_2" },
               { "op": "assert", "cond": "%v_2" },
               { "op": "impact", "guard": "0x00", "inputs": ["0x30"] }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        // The impact is guarded off, so nothing is contributed to the public
        // transcript inputs.
        let preimage = ProofPreimage {
            binding_input: 48.into(),
            communications_commitment: None,
            inputs: vec![0.into(), 42.into()],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        // The guarded-off impact input is zeroed in the public input vector.
        vk.verify(&PARAMS_VERIFIER, &proof, [48.into(), 0.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_immediate_little_endian_encoding() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v_0", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "constrain_eq", "a": "%v_0", "b": "0x0001" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        // v_0 must be 256 (little-endian interpretation of 0x0001)
        let preimage = ProofPreimage {
            binding_input: 77.into(),
            communications_commitment: None,
            inputs: vec![256.into()],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [77.into()].into_iter())
            .unwrap();

        // Test 0x0100 is interpreted as 1 (bytes [01, 00] = 1 + 256*0)
        let ir_raw2 = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v_0", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "constrain_eq", "a": "%v_0", "b": "0x0100" }
           ]
        }"#;
        let ir2 = IrSource::load(ir_raw2.as_bytes()).unwrap();
        let (pk2, vk2) = ir2.keygen(&TestParams).await.unwrap();

        // v_0 must be 1 (little-endian interpretation of 0x0100)
        let preimage2 = ProofPreimage {
            binding_input: 88.into(),
            communications_commitment: None,
            inputs: vec![1.into()],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof2, _) = preimage2
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk2.clone(),
                    vk: vk2.clone(),
                    ir: ir2.clone(),
                },
            )
            .await
            .unwrap();
        vk2.verify(&PARAMS_VERIFIER, &proof2, [88.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_jubjub_point_ops() {
        // Exercises test_eq (asserted), constrain_eq, cond_select, and neg on JubjubPoint
        // in a single circuit so every op is actively tested without dead values.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%p0", "type": "Point<Jubjub>" },
              { "name": "%p1", "type": "Point<Jubjub>" },
              { "name": "%bit", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [
           ],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "test_eq", "a": "%p0", "b": "%p1", "output": "%v0" },
               { "op": "assert", "cond": "%v0" },
               { "op": "constrain_eq", "a": "%p0", "b": "%p1" },
               { "op": "cond_select", "bit": "%bit", "a": "%p0", "b": "%p1", "output": "%p2" },
               { "op": "constrain_eq", "a": "%p2", "b": "%p0" },
               { "op": "neg", "a": "%p0", "output": "%p0_neg" },
               { "op": "private_input", "type": "Point<Jubjub>", "guard": null, "output": "%p0_neg_priv" },
               { "op": "constrain_eq", "a": "%p0_neg", "b": "%p0_neg_priv" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        // p0 == p1 == generator, bit == 1
        let p = EmbeddedGroupAffine::generator();
        let neg_p: EmbeddedGroupAffine = (-JubjubSubgroup::generator()).into();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: vec![
                p.x().unwrap(),
                p.y().unwrap(),
                p.x().unwrap(),
                p.y().unwrap(),
                1.into(),
            ],
            private_transcript: vec![neg_p.x().unwrap(), neg_p.y().unwrap()],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_jubjub_point_test_eq_unequal() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%p0", "type": "Point<Jubjub>" },
              { "name": "%p1", "type": "Point<Jubjub>" }
           ],
           "outputs": [
           ],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "test_eq", "a": "%p0", "b": "%p1", "output": "%v0" },
               { "op": "not", "a": "%v0", "output": "%v1" },
               { "op": "assert", "cond": "%v1" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        let p = EmbeddedGroupAffine::generator();
        let q: EmbeddedGroupAffine = JubjubSubgroup::random(OsRng).into();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: vec![
                p.x().unwrap(),
                p.y().unwrap(),
                q.x().unwrap(),
                q.y().unwrap(),
            ],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_jubjub_point_constrain_eq_fails_on_unequal() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%p0", "type": "Point<Jubjub>" },
              { "name": "%p1", "type": "Point<Jubjub>" }
           ],
           "outputs": [
           ],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "constrain_eq", "a": "%p0", "b": "%p1" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        // Different points: constrain_eq should fail
        let p = EmbeddedGroupAffine::generator();
        let q: EmbeddedGroupAffine = JubjubSubgroup::random(OsRng).into();
        let preimage_fail = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: vec![
                p.x().unwrap(),
                p.y().unwrap(),
                q.x().unwrap(),
                q.y().unwrap(),
            ],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let result = preimage_fail
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await;
        assert!(
            result.is_err(),
            "constrain_eq on different JubjubPoints should fail"
        );
    }

    #[actix_rt::test]
    async fn test_jubjub_point_cond_select_fails_when_bit_zero() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%p0", "type": "Point<Jubjub>" },
              { "name": "%p1", "type": "Point<Jubjub>" },
              { "name": "%bit", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [
           ],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "cond_select", "bit": "%bit", "a": "%p0", "b": "%p1", "output": "%p2" },
               { "op": "constrain_eq", "a": "%p2", "b": "%p0" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        let p = EmbeddedGroupAffine::generator();
        let q: EmbeddedGroupAffine = JubjubSubgroup::random(OsRng).into();

        // bit=0 selects p1 (!=p0), constrain_eq(p2, p0) should fail
        let preimage_fail = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: vec![
                p.x().unwrap(),
                p.y().unwrap(),
                q.x().unwrap(),
                q.y().unwrap(),
                0.into(),
            ],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let result = preimage_fail
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await;
        assert!(
            result.is_err(),
            "cond_select with bit=0 should select p1, failing constrain_eq against p0"
        );
    }

    #[test]
    fn test_invalid_operand_no_percent_prefix() {
        // Variables without '%' prefix should fail to deserialize
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v_0", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "assert", "cond": "v_0" }
           ]
        }"#;
        let result = IrSource::load(ir_raw.as_bytes());
        assert!(
            result.is_err(),
            "Should reject identifier without '%' prefix"
        );
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("Invalid operand format"),
            "Error message: {}",
            err
        );
        assert!(
            err.contains("Variables must start with '%'"),
            "Error message: {}",
            err
        );
    }

    #[test]
    fn test_invalid_operand_odd_length_hex() {
        // Hex immediates with odd length should fail to deserialize
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v_0", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "copy", "val": "0x1", "output": "%v_1" }
           ]
        }"#;
        let result = IrSource::load(ir_raw.as_bytes());
        assert!(result.is_err(), "Should reject odd-length hex string");
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("odd number of digits"),
            "Error message: {}",
            err
        );
    }

    #[test]
    fn test_invalid_operand_malformed_identifier() {
        // Random strings that don't follow conventions should be rejected
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "foo", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "assert", "cond": "foo" }
           ]
        }"#;
        let result = IrSource::load(ir_raw.as_bytes());
        assert!(result.is_err(), "Should reject malformed identifier");
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("Invalid operand format"),
            "Error message: {}",
            err
        );
    }

    #[actix_rt::test]
    async fn test_secp256k1_proof() {
        // Single circuit exercising all three Secp256k1 types.
        // Base, Scalar and Point values are typed inputs; arithmetic results
        // are checked via private inputs.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%id", "type": "Point<Secp256k1>"  },
              { "name": "%p0", "type": "Point<Secp256k1>"  },
              { "name": "%p1", "type": "Point<Secp256k1>"  },
              { "name": "%b0", "type": "Base<Secp256k1>"   },
              { "name": "%b1", "type": "Base<Secp256k1>"   },
              { "name": "%s0", "type": "Scalar<Secp256k1>" },
              { "name": "%s1", "type": "Scalar<Secp256k1>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "add", "a": "%p0", "b": "%p1", "output": "%p2" },
               { "op": "add", "a": "%b0", "b": "%b1", "output": "%b2" },
               { "op": "add", "a": "%s0", "b": "%s1", "output": "%s2" },
               { "op": "mul", "a": "%b0", "b": "%b1", "output": "%b_prod" },
               { "op": "mul", "a": "%s0", "b": "%s1", "output": "%s_prod" },
               { "op": "neg", "a": "%p0", "output": "%p0_neg" },
               { "op": "neg", "a": "%b0", "output": "%b0_neg" },
               { "op": "neg", "a": "%s0", "output": "%s0_neg" },
               { "op": "inv", "a": "%b0", "output": "%b0_inv" },
               { "op": "inv", "a": "%s0", "output": "%s0_inv" },
               { "op": "private_input", "type": "Point<Secp256k1>",  "guard": null, "output": "%p2_priv"     },
               { "op": "private_input", "type": "Base<Secp256k1>",   "guard": null, "output": "%b_prod_priv" },
               { "op": "private_input", "type": "Scalar<Secp256k1>", "guard": null, "output": "%s_prod_priv" },
               { "op": "private_input", "type": "Point<Secp256k1>",  "guard": null, "output": "%p0_neg_priv" },
               { "op": "private_input", "type": "Base<Secp256k1>",   "guard": null, "output": "%b0_neg_priv" },
               { "op": "private_input", "type": "Scalar<Secp256k1>", "guard": null, "output": "%s0_neg_priv" },
               { "op": "private_input", "type": "Base<Secp256k1>",   "guard": null, "output": "%b0_inv_priv" },
               { "op": "private_input", "type": "Scalar<Secp256k1>", "guard": null, "output": "%s0_inv_priv" },
               { "op": "constrain_eq", "a": "%p2",     "b": "%p2_priv"     },
               { "op": "constrain_eq", "a": "%b_prod", "b": "%b_prod_priv" },
               { "op": "constrain_eq", "a": "%p0_neg", "b": "%p0_neg_priv" },
               { "op": "constrain_eq", "a": "%b0_neg", "b": "%b0_neg_priv" },
               { "op": "test_eq",      "a": "%s_prod", "b": "%s_prod_priv", "output": "%sp_eq" },
               { "op": "assert",       "cond": "%sp_eq" },
               { "op": "test_eq",      "a": "%s0_neg", "b": "%s0_neg_priv", "output": "%sn_eq" },
               { "op": "assert",       "cond": "%sn_eq" },
               { "op": "constrain_eq", "a": "%b0_inv", "b": "%b0_inv_priv" },
               { "op": "test_eq",      "a": "%s0_inv", "b": "%s0_inv_priv", "output": "%si_eq" },
               { "op": "assert",       "cond": "%si_eq" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let id = k256::K256::identity();
        let p0 = k256::K256::random(OsRng);
        let p1 = k256::K256::random(OsRng);
        let b0 = k256::Fp::random(OsRng);
        let b1 = k256::Fp::random(OsRng);
        let s0 = k256::Fq::random(OsRng);
        let s1 = k256::Fq::random(OsRng);

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        let inputs: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Secp256k1Point(id)),
            encode(IrValue::Secp256k1Point(p0)),
            encode(IrValue::Secp256k1Point(p1)),
            encode(IrValue::Secp256k1Base(b0)),
            encode(IrValue::Secp256k1Base(b1)),
            encode(IrValue::Secp256k1Scalar(s0)),
            encode(IrValue::Secp256k1Scalar(s1)),
        ]
        .concat();

        let private_transcript: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Secp256k1Point(p0 + p1)),
            encode(IrValue::Secp256k1Base(b0 * b1)),
            encode(IrValue::Secp256k1Scalar(s0 * s1)),
            encode(IrValue::Secp256k1Point(-p0)),
            encode(IrValue::Secp256k1Base(-b0)),
            encode(IrValue::Secp256k1Scalar(-s0)),
            encode(IrValue::Secp256k1Base(Option::from(b0.invert()).unwrap())),
            encode(IrValue::Secp256k1Scalar(Option::from(s0.invert()).unwrap())),
        ]
        .concat();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_coordinates_proof() {
        // Exercises affine coordinates polymorphically across the
        // supported curve point types. Each extracted coordinate is
        // checked against a private input carrying the expected value.
        // A point is then reconstructed from the extracted coordinates
        // and compared to the original point.
        use midnight_zkir::ir_instructions::into_coordinates::into_coordinates_offcircuit;

        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%jp", "type": "Point<Jubjub>"    },
              { "name": "%sp", "type": "Point<Secp256k1>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "into_coordinates", "point": "%jp", "outputs": ["%jx", "%jy"] },
               { "op": "into_coordinates", "point": "%sp", "outputs": ["%sx", "%sy"] },
               { "op": "private_input", "type": "Scalar<BLS12-381>", "guard": null, "output": "%jx_exp" },
               { "op": "private_input", "type": "Scalar<BLS12-381>", "guard": null, "output": "%jy_exp" },
               { "op": "private_input", "type": "Base<Secp256k1>",   "guard": null, "output": "%sx_exp" },
               { "op": "private_input", "type": "Base<Secp256k1>",   "guard": null, "output": "%sy_exp" },
               { "op": "constrain_eq", "a": "%jx", "b": "%jx_exp" },
               { "op": "constrain_eq", "a": "%jy", "b": "%jy_exp" },
               { "op": "constrain_eq", "a": "%sx", "b": "%sx_exp" },
               { "op": "constrain_eq", "a": "%sy", "b": "%sy_exp" },
               { "op": "from_coordinates", "inputs": ["%jx", "%jy"], "output": "%jp_reconstructed" },
               { "op": "from_coordinates", "inputs": ["%sx", "%sy"], "output": "%sp_reconstructed" },
               { "op": "constrain_eq", "a": "%jp_reconstructed", "b": "%jp" },
               { "op": "constrain_eq", "a": "%sp_reconstructed", "b": "%sp" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let jp = JubjubSubgroup::random(OsRng);
        let sp = k256::K256::random(OsRng);

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        let inputs: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::JubjubPoint(jp)),
            encode(IrValue::Secp256k1Point(sp)),
        ]
        .concat();

        let (px, py) = into_coordinates_offcircuit(&IrValue::JubjubPoint(jp)).unwrap();
        let (sx, sy) = into_coordinates_offcircuit(&IrValue::Secp256k1Point(sp)).unwrap();

        let private_transcript: Vec<transient_crypto::curve::Fr> =
            [encode(px), encode(py), encode(sx), encode(sy)].concat();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_bytes_conversion_proof() {
        // Exercises to_bytes / from_bytes on Native, Secp256k1Base,
        // Secp256k1Scalar and Curve25519Scalar:
        //   1. Round-trips a typed value through to_bytes then
        //      from_bytes and checks it matches the original.
        //   2. Converts fixed, non-canonical byte strings (all 0xff,
        //      which exceeds every one of these fields' moduli) via
        //      from_bytes and checks the in-circuit result against the
        //      off-circuit reference implementation, exercising the
        //      modular-reduction behavior documented on the instruction.
        //      The Curve25519Scalar case uses a Bytes<64> input, the
        //      SHA-512-output shape needed by ed25519.
        use midnight_zkir::ir_instructions::from_bytes::from_bytes_offcircuit;
        use midnight_zkir::ir_types::IrType;

        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%native",        "type": "Scalar<BLS12-381>"  },
              { "name": "%secp_base",     "type": "Base<Secp256k1>"    },
              { "name": "%secp_scalar",   "type": "Scalar<Secp256k1>"  },
              { "name": "%c25519_scalar", "type": "Scalar<Curve25519>" },
              { "name": "%raw",           "type": "Bytes<32>"          },
              { "name": "%raw_wide",      "type": "Bytes<64>"          }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "to_bytes", "input": "%native",        "output": "%native_bytes" },
               { "op": "to_bytes", "input": "%secp_base",     "output": "%base_bytes"   },
               { "op": "to_bytes", "input": "%secp_scalar",   "output": "%scalar_bytes" },
               { "op": "to_bytes", "input": "%c25519_scalar", "output": "%c25519_bytes" },

               { "op": "from_bytes", "bytes": "%native_bytes", "type": "Scalar<BLS12-381>",  "output": "%native_back" },
               { "op": "from_bytes", "bytes": "%base_bytes",   "type": "Base<Secp256k1>",    "output": "%base_back"   },
               { "op": "from_bytes", "bytes": "%scalar_bytes", "type": "Scalar<Secp256k1>",  "output": "%scalar_back" },
               { "op": "from_bytes", "bytes": "%c25519_bytes", "type": "Scalar<Curve25519>", "output": "%c25519_back" },

               { "op": "constrain_eq", "a": "%native_back", "b": "%native"        },
               { "op": "constrain_eq", "a": "%base_back",   "b": "%secp_base"     },
               { "op": "constrain_eq", "a": "%scalar_back", "b": "%secp_scalar"   },
               { "op": "constrain_eq", "a": "%c25519_back", "b": "%c25519_scalar" },

               { "op": "from_bytes", "bytes": "%raw",      "type": "Scalar<BLS12-381>",  "output": "%raw_native" },
               { "op": "from_bytes", "bytes": "%raw",      "type": "Base<Secp256k1>",    "output": "%raw_base"   },
               { "op": "from_bytes", "bytes": "%raw",      "type": "Scalar<Secp256k1>",  "output": "%raw_scalar" },
               { "op": "from_bytes", "bytes": "%raw_wide", "type": "Scalar<Curve25519>", "output": "%raw_c25519" },

               { "op": "private_input", "type": "Scalar<BLS12-381>",  "guard": null, "output": "%raw_native_exp" },
               { "op": "private_input", "type": "Base<Secp256k1>",    "guard": null, "output": "%raw_base_exp"   },
               { "op": "private_input", "type": "Scalar<Secp256k1>",  "guard": null, "output": "%raw_scalar_exp" },
               { "op": "private_input", "type": "Scalar<Curve25519>", "guard": null, "output": "%raw_c25519_exp" },

               { "op": "constrain_eq", "a": "%raw_native", "b": "%raw_native_exp" },
               { "op": "constrain_eq", "a": "%raw_base",   "b": "%raw_base_exp"   },
               { "op": "constrain_eq", "a": "%raw_scalar", "b": "%raw_scalar_exp" },
               { "op": "constrain_eq", "a": "%raw_c25519", "b": "%raw_c25519_exp" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let native_val: transient_crypto::curve::Fr = rand::random();
        let base_val = k256::Fp::random(OsRng);
        let scalar_val = k256::Fq::random(OsRng);
        let c25519_val = curve25519::Scalar::random(&mut OsRng);
        let raw_bytes = [0xffu8; 32];
        let raw_wide_bytes = [0xffu8; 64];

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        let inputs: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Native(native_val)),
            encode(IrValue::Secp256k1Base(base_val)),
            encode(IrValue::Secp256k1Scalar(scalar_val)),
            encode(IrValue::Curve25519Scalar(c25519_val)),
            encode(IrValue::Bytes(raw_bytes.to_vec())),
            encode(IrValue::Bytes(raw_wide_bytes.to_vec())),
        ]
        .concat();

        let raw_native_exp = from_bytes_offcircuit(&IrType::Native, &raw_bytes).unwrap();
        let raw_base_exp = from_bytes_offcircuit(&IrType::Secp256k1Base, &raw_bytes).unwrap();
        let raw_scalar_exp = from_bytes_offcircuit(&IrType::Secp256k1Scalar, &raw_bytes).unwrap();
        let raw_c25519_exp =
            from_bytes_offcircuit(&IrType::Curve25519Scalar, &raw_wide_bytes).unwrap();

        let private_transcript: Vec<transient_crypto::curve::Fr> = [
            encode(raw_native_exp),
            encode(raw_base_exp),
            encode(raw_scalar_exp),
            encode(raw_c25519_exp),
        ]
        .concat();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_bytes_natives_proof() {
        // Exercises bytes_into_natives / bytes_from_natives at lengths below, at,
        // and on either side of the 31-byte limb boundary:
        //   1. Packs a Bytes(n) into ceil(n / 31) native field elements and checks
        //      every limb against the off-circuit reference.
        //   2. Unpacks the limbs with bytes_from_natives and checks equality with
        //      the original value, exercising the full roundtrip.
        use midnight_zkir::ir_instructions::from_bytes::from_bytes_offcircuit;
        use midnight_zkir::ir_types::IrType;

        const BYTES_PER_LIMB: usize = 31;

        for n in [1usize, BYTES_PER_LIMB, 32, 33, 70] {
            let nb_limbs = n.div_ceil(BYTES_PER_LIMB);
            let limbs: Vec<String> = (0..nb_limbs).map(|i| format!("%l{i}")).collect();
            let limb_list = limbs
                .iter()
                .map(|l| format!("\"{l}\""))
                .collect::<Vec<_>>()
                .join(", ");

            let mut instructions = vec![
                format!(
                    r#"{{ "op": "bytes_into_natives", "bytes": "%b", "outputs": [{limb_list}] }}"#
                ),
                format!(
                    r#"{{ "op": "bytes_from_natives", "inputs": [{limb_list}], "len": {n}, "output": "%b_back" }}"#
                ),
                r#"{ "op": "constrain_eq", "a": "%b_back", "b": "%b" }"#.to_string(),
            ];
            for l in limbs.iter() {
                instructions.push(format!(
                    r#"{{ "op": "private_input", "type": "Scalar<BLS12-381>", "guard": null, "output": "{l}_exp" }}"#
                ));
            }
            for l in limbs.iter() {
                instructions.push(format!(
                    r#"{{ "op": "constrain_eq", "a": "{l}", "b": "{l}_exp" }}"#
                ));
            }

            let ir_raw = format!(
                r#"{{
                   "version": {{ "major": 3, "minor": 0 }},
                   "inputs": [
                      {{ "name": "%b", "type": "Bytes<{n}>" }}
                   ],
                   "outputs": [],
                   "do_communications_commitment": false,
                   "instructions": [{}]
                }}"#,
                instructions.join(",\n")
            );
            let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

            // Bytes chosen so that the last byte of every limb is non-zero, which
            // exercises the most significant byte of each field element.
            let bytes: Vec<u8> = (0..n).map(|i| (i + 1) as u8).collect();

            let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
                encode_offcircuit(&v)
                    .into_iter()
                    .map(|x| x.try_into().unwrap())
                    .collect()
            };

            let inputs: Vec<transient_crypto::curve::Fr> = encode(IrValue::Bytes(bytes.clone()));

            // Compute the expected limbs independently of the packing under test:
            // every chunk of (at most) 31 bytes, zero-padded to 32, read as a
            // little-endian native field element.
            let private_transcript: Vec<transient_crypto::curve::Fr> = bytes
                .chunks(BYTES_PER_LIMB)
                .flat_map(|chunk| {
                    let mut buf = [0u8; 32];
                    buf[..chunk.len()].copy_from_slice(chunk);
                    encode(from_bytes_offcircuit(&IrType::Native, &buf).unwrap())
                })
                .collect();

            let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
            let preimage = ProofPreimage {
                binding_input: 42.into(),
                communications_commitment: None,
                inputs,
                private_transcript,
                public_transcript_inputs: vec![],
                public_transcript_outputs: vec![],
                key_location: KeyLocation(Cow::Borrowed("builtin")),
            };
            let (proof, _) = preimage
                .prove::<IrSource>(
                    &mut ChaCha20Rng::from_seed([42; 32]),
                    &TestParams,
                    &TestResolver {
                        pk: pk.clone(),
                        vk: vk.clone(),
                        ir: ir.clone(),
                    },
                )
                .await
                .unwrap();
            vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
                .unwrap();
        }
    }

    #[actix_rt::test]
    async fn test_reverse_bytes_proof() {
        // Exercises reverse:
        //   1. Reverses the byte order of a Bytes(32) and checks the result
        //      against an off-circuit reference computed by reversing the bytes.
        //   2. Reverses the reversed value again and checks it equals the
        //      original, exercising the involutive round-trip.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%b", "type": "Bytes<32>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "reverse", "bytes": "%b",   "output": "%rev"      },
               { "op": "reverse", "bytes": "%rev", "output": "%rev_rev"  },
               { "op": "constrain_eq", "a": "%rev_rev", "b": "%b" },
               { "op": "private_input", "type": "Bytes<32>", "guard": null, "output": "%rev_exp" },
               { "op": "constrain_eq", "a": "%rev", "b": "%rev_exp" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let bytes: [u8; 32] = std::array::from_fn(|i| (i + 1) as u8);
        let mut rev_bytes = bytes;
        rev_bytes.reverse();

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        let inputs: Vec<transient_crypto::curve::Fr> = encode(IrValue::Bytes(bytes.to_vec()));
        let private_transcript: Vec<transient_crypto::curve::Fr> =
            encode(IrValue::Bytes(rev_bytes.to_vec()));

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_native_inv_proof() {
        // Verifies native field inversion: v0 * inv(v0) == 1.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%v0", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "inv", "a": "%v0",          "output": "%v0_inv" },
               { "op": "mul", "a": "%v0", "b": "%v0_inv", "output": "%one"   },
               { "op": "constrain_eq", "a": "%one", "b": "0x01" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();
        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: vec![7.into()],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk,
                    vk: vk.clone(),
                    ir,
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_secp256r1_proof() {
        // Single circuit exercising all three Secp256r1 types.
        // Base, Scalar and Point values are typed inputs; arithmetic results
        // are checked via private inputs.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%id", "type": "Point<Secp256r1>"  },
              { "name": "%p0", "type": "Point<Secp256r1>"  },
              { "name": "%p1", "type": "Point<Secp256r1>"  },
              { "name": "%b0", "type": "Base<Secp256r1>"   },
              { "name": "%b1", "type": "Base<Secp256r1>"   },
              { "name": "%s0", "type": "Scalar<Secp256r1>" },
              { "name": "%s1", "type": "Scalar<Secp256r1>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "add", "a": "%p0", "b": "%p1", "output": "%p2" },
               { "op": "add", "a": "%b0", "b": "%b1", "output": "%b2" },
               { "op": "add", "a": "%s0", "b": "%s1", "output": "%s2" },
               { "op": "mul", "a": "%b0", "b": "%b1", "output": "%b_prod" },
               { "op": "mul", "a": "%s0", "b": "%s1", "output": "%s_prod" },
               { "op": "neg", "a": "%p0", "output": "%p0_neg" },
               { "op": "neg", "a": "%b0", "output": "%b0_neg" },
               { "op": "neg", "a": "%s0", "output": "%s0_neg" },
               { "op": "inv", "a": "%b0", "output": "%b0_inv" },
               { "op": "inv", "a": "%s0", "output": "%s0_inv" },
               { "op": "private_input", "type": "Point<Secp256r1>",  "guard": null, "output": "%p2_priv"     },
               { "op": "private_input", "type": "Base<Secp256r1>",   "guard": null, "output": "%b_prod_priv" },
               { "op": "private_input", "type": "Scalar<Secp256r1>", "guard": null, "output": "%s_prod_priv" },
               { "op": "private_input", "type": "Point<Secp256r1>",  "guard": null, "output": "%p0_neg_priv" },
               { "op": "private_input", "type": "Base<Secp256r1>",   "guard": null, "output": "%b0_neg_priv" },
               { "op": "private_input", "type": "Scalar<Secp256r1>", "guard": null, "output": "%s0_neg_priv" },
               { "op": "private_input", "type": "Base<Secp256r1>",   "guard": null, "output": "%b0_inv_priv" },
               { "op": "private_input", "type": "Scalar<Secp256r1>", "guard": null, "output": "%s0_inv_priv" },
               { "op": "constrain_eq", "a": "%p2",     "b": "%p2_priv"     },
               { "op": "constrain_eq", "a": "%b_prod", "b": "%b_prod_priv" },
               { "op": "constrain_eq", "a": "%p0_neg", "b": "%p0_neg_priv" },
               { "op": "constrain_eq", "a": "%b0_neg", "b": "%b0_neg_priv" },
               { "op": "test_eq",      "a": "%s_prod", "b": "%s_prod_priv", "output": "%sp_eq" },
               { "op": "assert",       "cond": "%sp_eq" },
               { "op": "test_eq",      "a": "%s0_neg", "b": "%s0_neg_priv", "output": "%sn_eq" },
               { "op": "assert",       "cond": "%sn_eq" },
               { "op": "constrain_eq", "a": "%b0_inv", "b": "%b0_inv_priv" },
               { "op": "test_eq",      "a": "%s0_inv", "b": "%s0_inv_priv", "output": "%si_eq" },
               { "op": "assert",       "cond": "%si_eq" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let id = p256::P256::identity();
        let p0 = p256::P256::random(OsRng);
        let p1 = p256::P256::random(OsRng);
        let b0 = p256::Fp::random(OsRng);
        let b1 = p256::Fp::random(OsRng);
        let s0 = p256::Fq::random(OsRng);
        let s1 = p256::Fq::random(OsRng);

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        let inputs: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Secp256r1Point(id)),
            encode(IrValue::Secp256r1Point(p0)),
            encode(IrValue::Secp256r1Point(p1)),
            encode(IrValue::Secp256r1Base(b0)),
            encode(IrValue::Secp256r1Base(b1)),
            encode(IrValue::Secp256r1Scalar(s0)),
            encode(IrValue::Secp256r1Scalar(s1)),
        ]
        .concat();

        let private_transcript: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Secp256r1Point(p0 + p1)),
            encode(IrValue::Secp256r1Base(b0 * b1)),
            encode(IrValue::Secp256r1Scalar(s0 * s1)),
            encode(IrValue::Secp256r1Point(-p0)),
            encode(IrValue::Secp256r1Base(-b0)),
            encode(IrValue::Secp256r1Scalar(-s0)),
            encode(IrValue::Secp256r1Base(Option::from(b0.invert()).unwrap())),
            encode(IrValue::Secp256r1Scalar(Option::from(s0.invert()).unwrap())),
        ]
        .concat();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_secp256r1_ec_mul_proof() {
        // Proves p0 * s0 via in-circuit ec_mul on Point<Secp256r1>, and G * s0
        // via ec_mul_generator; the results are checked against private inputs
        // carrying the off-circuit products.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%p0", "type": "Point<Secp256r1>"  },
              { "name": "%s0", "type": "Scalar<Secp256r1>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "ec_mul", "a": "%p0", "scalar": "%s0", "output": "%p1" },
               { "op": "private_input", "type": "Point<Secp256r1>", "guard": null, "output": "%p1_priv" },
               { "op": "constrain_eq", "a": "%p1", "b": "%p1_priv" },

               { "op": "ec_mul_generator", "scalar": "%s0", "output": "%pg" },
               { "op": "private_input", "type": "Point<Secp256r1>", "guard": null, "output": "%pg_priv" },
               { "op": "constrain_eq", "a": "%pg", "b": "%pg_priv" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let p0 = p256::P256::random(OsRng);
        let s0 = p256::Fq::random(OsRng);

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        let inputs: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Secp256r1Point(p0)),
            encode(IrValue::Secp256r1Scalar(s0)),
        ]
        .concat();

        let private_transcript: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Secp256r1Point(p0 * s0)),
            encode(IrValue::Secp256r1Point(p256::P256::generator() * s0)),
        ]
        .concat();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_secp256r1_coordinates_proof() {
        // Secp256r1 counterpart of test_coordinates_proof: extracts affine
        // coordinates of a Point<Secp256r1>, checks them against private inputs
        // carrying the expected values, then reconstructs the point from the
        // extracted coordinates and compares it to the original.
        use midnight_zkir::ir_instructions::into_coordinates::into_coordinates_offcircuit;

        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%pp", "type": "Point<Secp256r1>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "into_coordinates", "point": "%pp", "outputs": ["%px", "%py"] },
               { "op": "private_input", "type": "Base<Secp256r1>", "guard": null, "output": "%px_exp" },
               { "op": "private_input", "type": "Base<Secp256r1>", "guard": null, "output": "%py_exp" },
               { "op": "constrain_eq", "a": "%px", "b": "%px_exp" },
               { "op": "constrain_eq", "a": "%py", "b": "%py_exp" },
               { "op": "from_coordinates", "inputs": ["%px", "%py"], "output": "%pp_reconstructed" },
               { "op": "constrain_eq", "a": "%pp_reconstructed", "b": "%pp" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let pp = p256::P256::random(OsRng);

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        let inputs: Vec<transient_crypto::curve::Fr> = encode(IrValue::Secp256r1Point(pp));

        let (px, py) = into_coordinates_offcircuit(&IrValue::Secp256r1Point(pp)).unwrap();

        let private_transcript: Vec<transient_crypto::curve::Fr> =
            [encode(px), encode(py)].concat();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_secp256r1_bytes_conversion_proof() {
        // Secp256r1 counterpart of test_bytes_conversion_proof, exercising
        // to_bytes / from_bytes on Secp256r1Base and Secp256r1Scalar:
        //   1. Round-trips a typed value through to_bytes then
        //      from_bytes and checks it matches the original.
        //   2. Converts a fixed, non-canonical 32-byte string (all 0xff,
        //      which exceeds both fields' moduli) via from_bytes and checks
        //      the in-circuit result against the off-circuit reference
        //      implementation, exercising the modular-reduction behavior
        //      documented on the instruction.
        use midnight_zkir::ir_instructions::from_bytes::from_bytes_offcircuit;
        use midnight_zkir::ir_types::IrType;

        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%secp256r1_base",   "type": "Base<Secp256r1>"   },
              { "name": "%secp256r1_scalar", "type": "Scalar<Secp256r1>" },
              { "name": "%raw",         "type": "Bytes<32>"    }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "to_bytes", "input": "%secp256r1_base",   "output": "%base_bytes"   },
               { "op": "to_bytes", "input": "%secp256r1_scalar", "output": "%scalar_bytes" },

               { "op": "from_bytes", "bytes": "%base_bytes",   "type": "Base<Secp256r1>",   "output": "%base_back"   },
               { "op": "from_bytes", "bytes": "%scalar_bytes", "type": "Scalar<Secp256r1>", "output": "%scalar_back" },

               { "op": "constrain_eq", "a": "%base_back",   "b": "%secp256r1_base"   },
               { "op": "constrain_eq", "a": "%scalar_back", "b": "%secp256r1_scalar" },

               { "op": "from_bytes", "bytes": "%raw", "type": "Base<Secp256r1>",   "output": "%raw_base"   },
               { "op": "from_bytes", "bytes": "%raw", "type": "Scalar<Secp256r1>", "output": "%raw_scalar" },

               { "op": "private_input", "type": "Base<Secp256r1>",   "guard": null, "output": "%raw_base_exp"   },
               { "op": "private_input", "type": "Scalar<Secp256r1>", "guard": null, "output": "%raw_scalar_exp" },

               { "op": "constrain_eq", "a": "%raw_base",   "b": "%raw_base_exp"   },
               { "op": "constrain_eq", "a": "%raw_scalar", "b": "%raw_scalar_exp" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let base_val = p256::Fp::random(OsRng);
        let scalar_val = p256::Fq::random(OsRng);
        let raw_bytes = [0xffu8; 32];

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        let inputs: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Secp256r1Base(base_val)),
            encode(IrValue::Secp256r1Scalar(scalar_val)),
            encode(IrValue::Bytes(raw_bytes.to_vec())),
        ]
        .concat();

        let raw_base_exp = from_bytes_offcircuit(&IrType::Secp256r1Base, &raw_bytes).unwrap();
        let raw_scalar_exp = from_bytes_offcircuit(&IrType::Secp256r1Scalar, &raw_bytes).unwrap();

        let private_transcript: Vec<transient_crypto::curve::Fr> =
            [encode(raw_base_exp), encode(raw_scalar_exp)].concat();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_secp256r1_point_constrain_eq_fails_on_unequal() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%p0", "type": "Point<Secp256r1>" },
              { "name": "%p1", "type": "Point<Secp256r1>" }
           ],
           "outputs": [
           ],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "constrain_eq", "a": "%p0", "b": "%p1" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        // Different points: constrain_eq should fail
        let p = p256::P256::random(OsRng);
        let q = p256::P256::random(OsRng);
        assert_ne!(p, q);

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        let preimage_fail = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: [
                encode(IrValue::Secp256r1Point(p)),
                encode(IrValue::Secp256r1Point(q)),
            ]
            .concat(),
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let result = preimage_fail
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await;
        assert!(
            result.is_err(),
            "constrain_eq on different Secp256r1 points should fail"
        );
    }

    #[actix_rt::test]
    async fn test_curve25519_proof() {
        // Single circuit exercising all three Curve25519 types.
        // Base, Scalar and Point values are typed inputs; arithmetic results
        // are checked via private inputs.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%id", "type": "Point<Curve25519>"  },
              { "name": "%p0", "type": "Point<Curve25519>"  },
              { "name": "%p1", "type": "Point<Curve25519>"  },
              { "name": "%b0", "type": "Base<Curve25519>"   },
              { "name": "%b1", "type": "Base<Curve25519>"   },
              { "name": "%s0", "type": "Scalar<Curve25519>" },
              { "name": "%s1", "type": "Scalar<Curve25519>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "add", "a": "%p0", "b": "%p1", "output": "%p2" },
               { "op": "add", "a": "%b0", "b": "%b1", "output": "%b2" },
               { "op": "add", "a": "%s0", "b": "%s1", "output": "%s2" },
               { "op": "mul", "a": "%b0", "b": "%b1", "output": "%b_prod" },
               { "op": "mul", "a": "%s0", "b": "%s1", "output": "%s_prod" },
               { "op": "neg", "a": "%p0", "output": "%p0_neg" },
               { "op": "neg", "a": "%b0", "output": "%b0_neg" },
               { "op": "neg", "a": "%s0", "output": "%s0_neg" },
               { "op": "inv", "a": "%b0", "output": "%b0_inv" },
               { "op": "inv", "a": "%s0", "output": "%s0_inv" },
               { "op": "private_input", "type": "Point<Curve25519>",  "guard": null, "output": "%p2_priv"     },
               { "op": "private_input", "type": "Base<Curve25519>",   "guard": null, "output": "%b_prod_priv" },
               { "op": "private_input", "type": "Scalar<Curve25519>", "guard": null, "output": "%s_prod_priv" },
               { "op": "private_input", "type": "Point<Curve25519>",  "guard": null, "output": "%p0_neg_priv" },
               { "op": "private_input", "type": "Base<Curve25519>",   "guard": null, "output": "%b0_neg_priv" },
               { "op": "private_input", "type": "Scalar<Curve25519>", "guard": null, "output": "%s0_neg_priv" },
               { "op": "private_input", "type": "Base<Curve25519>",   "guard": null, "output": "%b0_inv_priv" },
               { "op": "private_input", "type": "Scalar<Curve25519>", "guard": null, "output": "%s0_inv_priv" },
               { "op": "constrain_eq", "a": "%p2",     "b": "%p2_priv"     },
               { "op": "constrain_eq", "a": "%b_prod", "b": "%b_prod_priv" },
               { "op": "constrain_eq", "a": "%p0_neg", "b": "%p0_neg_priv" },
               { "op": "constrain_eq", "a": "%b0_neg", "b": "%b0_neg_priv" },
               { "op": "test_eq",      "a": "%s_prod", "b": "%s_prod_priv", "output": "%sp_eq" },
               { "op": "assert",       "cond": "%sp_eq" },
               { "op": "test_eq",      "a": "%s0_neg", "b": "%s0_neg_priv", "output": "%sn_eq" },
               { "op": "assert",       "cond": "%sn_eq" },
               { "op": "constrain_eq", "a": "%b0_inv", "b": "%b0_inv_priv" },
               { "op": "test_eq",      "a": "%s0_inv", "b": "%s0_inv_priv", "output": "%si_eq" },
               { "op": "assert",       "cond": "%si_eq" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let id = curve25519::Curve25519Subgroup::identity();
        let p0 = curve25519::Curve25519Subgroup::random(OsRng);
        let p1 = curve25519::Curve25519Subgroup::random(OsRng);
        let b0 = curve25519::Fp::random(OsRng);
        let b1 = curve25519::Fp::random(OsRng);
        let s0 = <curve25519::Scalar as Field>::random(OsRng);
        let s1 = <curve25519::Scalar as Field>::random(OsRng);

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        let inputs: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Curve25519Point(id)),
            encode(IrValue::Curve25519Point(p0)),
            encode(IrValue::Curve25519Point(p1)),
            encode(IrValue::Curve25519Base(b0)),
            encode(IrValue::Curve25519Base(b1)),
            encode(IrValue::Curve25519Scalar(s0)),
            encode(IrValue::Curve25519Scalar(s1)),
        ]
        .concat();

        let private_transcript: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Curve25519Point(p0 + p1)),
            encode(IrValue::Curve25519Base(b0 * b1)),
            encode(IrValue::Curve25519Scalar(s0 * s1)),
            encode(IrValue::Curve25519Point(-p0)),
            encode(IrValue::Curve25519Base(-b0)),
            encode(IrValue::Curve25519Scalar(-s0)),
            encode(IrValue::Curve25519Base(Option::from(b0.invert()).unwrap())),
            encode(IrValue::Curve25519Scalar(Field::invert(&s0).unwrap())),
        ]
        .concat();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_curve25519_ec_mul_proof() {
        // Proves p0 * s0 via in-circuit ec_mul on Point<Curve25519>, and
        // G * s0 via ec_mul_generator; the results are checked against
        // private inputs carrying the off-circuit products.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%p0", "type": "Point<Curve25519>"  },
              { "name": "%s0", "type": "Scalar<Curve25519>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "ec_mul", "a": "%p0", "scalar": "%s0", "output": "%p1" },
               { "op": "private_input", "type": "Point<Curve25519>", "guard": null, "output": "%p1_priv" },
               { "op": "constrain_eq", "a": "%p1", "b": "%p1_priv" },

               { "op": "ec_mul_generator", "scalar": "%s0", "output": "%pg" },
               { "op": "private_input", "type": "Point<Curve25519>", "guard": null, "output": "%pg_priv" },
               { "op": "constrain_eq", "a": "%pg", "b": "%pg_priv" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let p0 = curve25519::Curve25519Subgroup::random(OsRng);
        let s0 = <curve25519::Scalar as Field>::random(OsRng);

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        let inputs: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Curve25519Point(p0)),
            encode(IrValue::Curve25519Scalar(s0)),
        ]
        .concat();

        let private_transcript: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Curve25519Point(p0 * s0)),
            encode(IrValue::Curve25519Point(
                curve25519::Curve25519Subgroup::generator() * s0,
            )),
        ]
        .concat();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_curve25519_coordinates_proof() {
        // Curve25519 counterpart of test_coordinates_proof: extracts affine
        // coordinates of a Point<Curve25519>, checks them against private
        // inputs carrying the expected values, then reconstructs the point
        // from the extracted coordinates and compares it to the original.
        use midnight_zkir::ir_instructions::into_coordinates::into_coordinates_offcircuit;

        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%pp", "type": "Point<Curve25519>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "into_coordinates", "point": "%pp", "outputs": ["%px", "%py"] },
               { "op": "private_input", "type": "Base<Curve25519>", "guard": null, "output": "%px_exp" },
               { "op": "private_input", "type": "Base<Curve25519>", "guard": null, "output": "%py_exp" },
               { "op": "constrain_eq", "a": "%px", "b": "%px_exp" },
               { "op": "constrain_eq", "a": "%py", "b": "%py_exp" },
               { "op": "from_coordinates", "inputs": ["%px", "%py"], "output": "%pp_reconstructed" },
               { "op": "constrain_eq", "a": "%pp_reconstructed", "b": "%pp" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let pp = curve25519::Curve25519Subgroup::random(OsRng);

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        let inputs: Vec<transient_crypto::curve::Fr> = encode(IrValue::Curve25519Point(pp));

        let (px, py) = into_coordinates_offcircuit(&IrValue::Curve25519Point(pp)).unwrap();

        let private_transcript: Vec<transient_crypto::curve::Fr> =
            [encode(px), encode(py)].concat();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    // Runs `EcMulGenerator -> ToBytes -> FromBytes` on `scalar` in-circuit,
    // checking the compressed bytes against the off-circuit `ToBytes` and the
    // decoded point against the original one. Returns the compressed bytes.
    async fn point_bytes_roundtrip_proof(curve: &str, scalar: IrValue) -> Vec<u8> {
        use midnight_zkir::ir_instructions::to_bytes::to_bytes_offcircuit;

        let p = match &scalar {
            IrValue::Secp256k1Scalar(s) => IrValue::Secp256k1Point(k256::K256::generator() * s),
            IrValue::Secp256r1Scalar(s) => IrValue::Secp256r1Point(p256::P256::generator() * s),
            IrValue::Curve25519Scalar(s) => {
                IrValue::Curve25519Point(curve25519::Curve25519Subgroup::generator() * s)
            }
            IrValue::JubjubScalar(s) => IrValue::JubjubPoint(JubjubSubgroup::generator() * s),
            _ => unreachable!(),
        };
        let compressed: Vec<u8> = to_bytes_offcircuit(&p).unwrap().try_into().unwrap();

        let ir_raw = format!(
            r#"{{
           "version": {{ "major": 3, "minor": 0 }},
           "inputs": [
              {{ "name": "%s", "type": "Scalar<{curve}>" }}
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               {{ "op": "ec_mul_generator", "scalar": "%s", "output": "%p" }},
               {{ "op": "to_bytes", "input": "%p", "output": "%c" }},
               {{ "op": "private_input", "type": "Bytes<{n}>", "guard": null, "output": "%c_exp" }},
               {{ "op": "constrain_eq", "a": "%c", "b": "%c_exp" }},
               {{ "op": "from_bytes", "bytes": "%c", "type": "Point<{curve}>", "output": "%q" }},
               {{ "op": "constrain_eq", "a": "%q", "b": "%p" }}
           ]
        }}"#,
            n = compressed.len()
        );
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: encode(scalar),
            private_transcript: encode(IrValue::Bytes(compressed.clone())),
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();

        compressed
    }

    #[actix_rt::test]
    async fn test_curve25519_point_bytes_proof() {
        let s = <curve25519::Scalar as Field>::random(OsRng);
        point_bytes_roundtrip_proof("Curve25519", IrValue::Curve25519Scalar(s)).await;
    }

    #[actix_rt::test]
    async fn test_jubjub_point_bytes_proof() {
        let s = JubjubFr::random(OsRng);
        point_bytes_roundtrip_proof("Jubjub", IrValue::JubjubScalar(s)).await;

        let s = JubjubFr::ZERO;
        let c = point_bytes_roundtrip_proof("Jubjub", IrValue::JubjubScalar(s)).await;
        let mut expected = vec![0u8; 32];
        expected[0] = 1;
        assert_eq!(c, expected);
    }

    #[actix_rt::test]
    async fn test_secp256k1_point_bytes_proof() {
        let s = k256::Fq::random(OsRng);
        point_bytes_roundtrip_proof("Secp256k1", IrValue::Secp256k1Scalar(s)).await;

        // The identity has unconstrained coordinates in-circuit.
        let s = k256::Fq::ZERO;
        let c = point_bytes_roundtrip_proof("Secp256k1", IrValue::Secp256k1Scalar(s)).await;
        assert_eq!(c, vec![0u8; 33]);
    }

    #[actix_rt::test]
    async fn test_secp256r1_point_bytes_proof() {
        let s = p256::Fq::random(OsRng);
        point_bytes_roundtrip_proof("Secp256r1", IrValue::Secp256r1Scalar(s)).await;

        // The identity has unconstrained coordinates in-circuit.
        let s = p256::Fq::ZERO;
        let c = point_bytes_roundtrip_proof("Secp256r1", IrValue::Secp256r1Scalar(s)).await;
        assert_eq!(c, vec![0u8; 33]);
    }

    #[actix_rt::test]
    async fn test_curve25519_bytes_conversion_proof() {
        // Curve25519 counterpart of test_bytes_conversion_proof, exercising
        // to_bytes / from_bytes on Curve25519Base and Curve25519Scalar:
        //   1. Round-trips a typed value through to_bytes then
        //      from_bytes and checks it matches the original.
        //   2. Converts fixed, non-canonical byte strings (all 0xff,
        //      which exceeds both fields' moduli) via from_bytes and checks
        //      the in-circuit result against the off-circuit reference
        //      implementation, exercising the modular-reduction behavior
        //      documented on the instruction. The scalar case uses a Bytes<64>
        //      input, the SHA-512-output shape needed by ed25519.
        use midnight_zkir::ir_instructions::from_bytes::from_bytes_offcircuit;
        use midnight_zkir::ir_types::IrType;

        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%c25519_base",   "type": "Base<Curve25519>"   },
              { "name": "%c25519_scalar", "type": "Scalar<Curve25519>" },
              { "name": "%raw",           "type": "Bytes<32>"          },
              { "name": "%raw_wide",      "type": "Bytes<64>"          }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "to_bytes", "input": "%c25519_base",   "output": "%base_bytes"   },
               { "op": "to_bytes", "input": "%c25519_scalar", "output": "%scalar_bytes" },

               { "op": "from_bytes", "bytes": "%base_bytes",   "type": "Base<Curve25519>",   "output": "%base_back"   },
               { "op": "from_bytes", "bytes": "%scalar_bytes", "type": "Scalar<Curve25519>", "output": "%scalar_back" },

               { "op": "constrain_eq", "a": "%base_back",   "b": "%c25519_base"   },
               { "op": "constrain_eq", "a": "%scalar_back", "b": "%c25519_scalar" },

               { "op": "from_bytes", "bytes": "%raw",      "type": "Base<Curve25519>",   "output": "%raw_base"   },
               { "op": "from_bytes", "bytes": "%raw_wide", "type": "Scalar<Curve25519>", "output": "%raw_scalar" },

               { "op": "private_input", "type": "Base<Curve25519>",   "guard": null, "output": "%raw_base_exp"   },
               { "op": "private_input", "type": "Scalar<Curve25519>", "guard": null, "output": "%raw_scalar_exp" },

               { "op": "constrain_eq", "a": "%raw_base",   "b": "%raw_base_exp"   },
               { "op": "constrain_eq", "a": "%raw_scalar", "b": "%raw_scalar_exp" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let base_val = curve25519::Fp::random(OsRng);
        let scalar_val = <curve25519::Scalar as Field>::random(OsRng);
        let raw_bytes = [0xffu8; 32];
        let raw_wide_bytes = [0xffu8; 64];

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        let inputs: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Curve25519Base(base_val)),
            encode(IrValue::Curve25519Scalar(scalar_val)),
            encode(IrValue::Bytes(raw_bytes.to_vec())),
            encode(IrValue::Bytes(raw_wide_bytes.to_vec())),
        ]
        .concat();

        let raw_base_exp = from_bytes_offcircuit(&IrType::Curve25519Base, &raw_bytes).unwrap();
        let raw_scalar_exp =
            from_bytes_offcircuit(&IrType::Curve25519Scalar, &raw_wide_bytes).unwrap();

        let private_transcript: Vec<transient_crypto::curve::Fr> =
            [encode(raw_base_exp), encode(raw_scalar_exp)].concat();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_curve25519_point_constrain_eq_fails_on_unequal() {
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%p0", "type": "Point<Curve25519>" },
              { "name": "%p1", "type": "Point<Curve25519>" }
           ],
           "outputs": [
           ],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "constrain_eq", "a": "%p0", "b": "%p1" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        // Different points: constrain_eq should fail
        let p = curve25519::Curve25519Subgroup::random(OsRng);
        let q = curve25519::Curve25519Subgroup::random(OsRng);
        assert_ne!(p, q);

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        let preimage_fail = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: [
                encode(IrValue::Curve25519Point(p)),
                encode(IrValue::Curve25519Point(q)),
            ]
            .concat(),
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let result = preimage_fail
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await;
        assert!(
            result.is_err(),
            "constrain_eq on different Curve25519 points should fail"
        );
    }

    #[actix_rt::test]
    async fn test_bool_ops() {
        // Exercises the Bool type end-to-end in a single proof: Bool inputs
        // (assign), neg (logical NOT), cond_select between two Bools,
        // constrain_eq and test_eq on Bools. Expected results are checked
        // against Bool private inputs so every op is actively constrained.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%b0",  "type": "Bool" },
              { "name": "%b1",  "type": "Bool" },
              { "name": "%bit", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "neg", "a": "%b0", "output": "%nb0" },
               { "op": "neg", "a": "%nb0", "output": "%b0_again" },
               { "op": "constrain_eq", "a": "%b0_again", "b": "%b0" },
               { "op": "cond_select", "bit": "%bit", "a": "%b0", "b": "%b1", "output": "%sel" },
               { "op": "private_input", "type": "Bool", "guard": null, "output": "%sel_exp" },
               { "op": "constrain_eq", "a": "%sel", "b": "%sel_exp" },
               { "op": "private_input", "type": "Bool", "guard": null, "output": "%nb0_exp" },
               { "op": "constrain_eq", "a": "%nb0", "b": "%nb0_exp" },
               { "op": "test_eq", "a": "%b0", "b": "%sel", "output": "%b0_eq_sel" },
               { "op": "assert", "cond": "%b0_eq_sel" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        // b0 = true, b1 = false, bit = 1 (selects a == b0).
        // => nb0 = !b0 = false, b0_again = !nb0 = true == b0, sel = b0 = true.
        let inputs: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Bool(true)),
            encode(IrValue::Bool(false)),
            vec![1.into()],
        ]
        .concat();

        let private_transcript: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Bool(true)),  // %sel_exp
            encode(IrValue::Bool(false)), // %nb0_exp
        ]
        .concat();

        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_bool_constrain_eq_fails_on_unequal() {
        // constrain_eq on two different Bools must make proving fail.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%b0", "type": "Bool" },
              { "name": "%b1", "type": "Bool" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "constrain_eq", "a": "%b0", "b": "%b1" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        // b0 = true, b1 = false: constrain_eq should fail.
        let inputs: Vec<transient_crypto::curve::Fr> =
            [encode(IrValue::Bool(true)), encode(IrValue::Bool(false))].concat();

        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let result = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await;
        assert!(result.is_err(), "constrain_eq on unequal Bools should fail");
    }

    #[actix_rt::test]
    async fn test_bool_gates() {
        // Exercises the `and`, `or` and `xor` boolean gates over lists of Bool
        // values (varying arity), checking each result against the constant
        // Bool inputs %t (true) and %f (false).
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%t", "type": "Bool" },
              { "name": "%f", "type": "Bool" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "and", "inputs": ["%t", "%t", "%t"], "output": "%and_ttt" },
               { "op": "and", "inputs": ["%t", "%f"],       "output": "%and_tf"  },
               { "op": "and", "inputs": ["%t"],             "output": "%and_t"   },
               { "op": "or",  "inputs": ["%f", "%f"],       "output": "%or_ff"   },
               { "op": "or",  "inputs": ["%f", "%t"],       "output": "%or_ft"   },
               { "op": "xor", "inputs": ["%t", "%t"],       "output": "%xor_tt"  },
               { "op": "xor", "inputs": ["%t", "%f"],       "output": "%xor_tf"  },
               { "op": "xor", "inputs": ["%t", "%t", "%t"], "output": "%xor_ttt" },
               { "op": "constrain_eq", "a": "%and_ttt", "b": "%t" },
               { "op": "constrain_eq", "a": "%and_tf",  "b": "%f" },
               { "op": "constrain_eq", "a": "%and_t",   "b": "%t" },
               { "op": "constrain_eq", "a": "%or_ff",   "b": "%f" },
               { "op": "constrain_eq", "a": "%or_ft",   "b": "%t" },
               { "op": "constrain_eq", "a": "%xor_tt",  "b": "%f" },
               { "op": "constrain_eq", "a": "%xor_tf",  "b": "%t" },
               { "op": "constrain_eq", "a": "%xor_ttt", "b": "%t" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        // %t = true, %f = false.
        let inputs: Vec<transient_crypto::curve::Fr> =
            [encode(IrValue::Bool(true)), encode(IrValue::Bool(false))].concat();

        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_bool_gate_wrong_result_fails() {
        // and(true, false) == false, so constraining it to equal true must
        // make proving fail.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%t", "type": "Bool" },
              { "name": "%f", "type": "Bool" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "and", "inputs": ["%t", "%f"], "output": "%r" },
               { "op": "constrain_eq", "a": "%r", "b": "%t" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();
        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };
        let inputs: Vec<transient_crypto::curve::Fr> =
            [encode(IrValue::Bool(true)), encode(IrValue::Bool(false))].concat();

        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let result = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await;
        assert!(result.is_err(), "and(true, false) == true should fail");
    }

    #[actix_rt::test]
    async fn test_bool_gate_empty_inputs_fails() {
        // A boolean gate over an empty input list must be rejected (the
        // underlying gadget has no identity element wired up here). The
        // off-circuit preprocess pass (`check`) surfaces this as a clean error.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%t", "type": "Bool" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "and", "inputs": [], "output": "%r" },
               { "op": "constrain_eq", "a": "%r", "b": "%t" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: vec![1.into()],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let err = preimage
            .check(&ir)
            .expect_err("boolean gate with empty inputs should be rejected");
        assert!(
            err.to_string().contains("at least one input"),
            "unexpected error: {err}"
        );
    }

    #[actix_rt::test]
    async fn test_byte_ops() {
        // Exercises the Byte type end-to-end in a single proof: Byte inputs
        // (assign), cond_select between two Bytes, constrain_eq and test_eq on
        // Bytes, and a Byte private input. `neg` is intentionally not supported
        // for Byte.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%a",   "type": "Byte" },
              { "name": "%b",   "type": "Byte" },
              { "name": "%bit", "type": "Scalar<BLS12-381>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "cond_select", "bit": "%bit", "a": "%a", "b": "%b", "output": "%sel" },
               { "op": "constrain_eq", "a": "%sel", "b": "%a" },
               { "op": "private_input", "type": "Byte", "guard": null, "output": "%a_priv" },
               { "op": "constrain_eq", "a": "%a", "b": "%a_priv" },
               { "op": "test_eq", "a": "%a", "b": "%sel", "output": "%a_eq_sel" },
               { "op": "assert", "cond": "%a_eq_sel" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };

        // a = 7, b = 200, bit = 1 (selects a) => sel = a = 7.
        let inputs: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Byte(7)),
            encode(IrValue::Byte(200)),
            vec![1.into()],
        ]
        .concat();

        let private_transcript: Vec<transient_crypto::curve::Fr> = encode(IrValue::Byte(7));

        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_byte_constrain_eq_fails_on_unequal() {
        // constrain_eq on two different Bytes must make proving fail.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%a", "type": "Byte" },
              { "name": "%b", "type": "Byte" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "constrain_eq", "a": "%a", "b": "%b" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();

        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };
        // a = 7, b = 8: constrain_eq should fail.
        let inputs: Vec<transient_crypto::curve::Fr> =
            [encode(IrValue::Byte(7)), encode(IrValue::Byte(8))].concat();

        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let result = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await;
        assert!(result.is_err(), "constrain_eq on unequal Bytes should fail");
    }

    #[actix_rt::test]
    async fn test_bytes_n_proof() {
        // Exercises a Bytes(n) value with n != 32 (spanning two 31-byte chunks)
        // end-to-end: assign, test_eq (asserted), and constrain_eq against a
        // Bytes(48) private input.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%b", "type": "Bytes<48>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "test_eq", "a": "%b", "b": "%b", "output": "%eq" },
               { "op": "assert", "cond": "%eq" },
               { "op": "private_input", "type": "Bytes<48>", "guard": null, "output": "%bp" },
               { "op": "constrain_eq", "a": "%b", "b": "%bp" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();
        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };
        let bytes: Vec<u8> = (0..48u8).collect();

        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: encode(IrValue::Bytes(bytes.clone())),
            private_transcript: encode(IrValue::Bytes(bytes)),
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_bytes_concat_and_nth() {
        // `concat` builds a Bytes value from Byte and/or Bytes operands;
        // `nth` extracts the byte at a constant position.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%a", "type": "Byte" },
              { "name": "%b", "type": "Byte" },
              { "name": "%s", "type": "Bytes<2>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "concat", "inputs": ["%a", "%b"], "output": "%ab" },
               { "op": "concat", "inputs": ["%s", "%a"], "output": "%sa" },
               { "op": "nth", "bytes": "%ab", "index": 0, "output": "%ab0" },
               { "op": "nth", "bytes": "%ab", "index": 1, "output": "%ab1" },
               { "op": "nth", "bytes": "%sa", "index": 2, "output": "%sa2" },
               { "op": "constrain_eq", "a": "%ab0", "b": "%a" },
               { "op": "constrain_eq", "a": "%ab1", "b": "%b" },
               { "op": "constrain_eq", "a": "%sa2", "b": "%a" },
               { "op": "private_input", "type": "Bytes<3>", "guard": null, "output": "%sa_exp" },
               { "op": "constrain_eq", "a": "%sa", "b": "%sa_exp" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();
        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };
        // a = 10, b = 20, s = [30, 40] => sa = concat(s, a) = [30, 40, 10].
        let inputs: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Byte(10)),
            encode(IrValue::Byte(20)),
            encode(IrValue::Bytes(vec![30, 40])),
        ]
        .concat();
        let private_transcript = encode(IrValue::Bytes(vec![30, 40, 10]));

        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_nth_out_of_bounds_fails() {
        // `nth` with index >= n must be rejected (surfaced cleanly by the
        // off-circuit preprocess pass).
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%s", "type": "Bytes<2>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "nth", "bytes": "%s", "index": 2, "output": "%x" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();
        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: encode(IrValue::Bytes(vec![0, 1])),
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let err = preimage
            .check(&ir)
            .expect_err("nth out of bounds should be rejected");
        assert!(
            err.to_string().contains("out of bounds"),
            "unexpected error: {err}"
        );
    }

    #[actix_rt::test]
    async fn test_bytes_slice() {
        // `slice` extracts a contiguous sub-range; combined here with `reverse`.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%b", "type": "Bytes<6>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "slice", "bytes": "%b", "start": 1, "len": 3, "output": "%mid" },
               { "op": "slice", "bytes": "%b", "start": 3, "len": 3, "output": "%tail" },
               { "op": "reverse", "bytes": "%mid", "output": "%midrev" },
               { "op": "private_input", "type": "Bytes<3>", "guard": null, "output": "%mid_exp" },
               { "op": "private_input", "type": "Bytes<3>", "guard": null, "output": "%tail_exp" },
               { "op": "private_input", "type": "Bytes<3>", "guard": null, "output": "%midrev_exp" },
               { "op": "constrain_eq", "a": "%mid",    "b": "%mid_exp" },
               { "op": "constrain_eq", "a": "%tail",   "b": "%tail_exp" },
               { "op": "constrain_eq", "a": "%midrev", "b": "%midrev_exp" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();
        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };
        // b = [10,20,30,40,50,60] => mid = [20,30,40], tail = [40,50,60],
        // midrev = [40,30,20].
        let inputs = encode(IrValue::Bytes(vec![10, 20, 30, 40, 50, 60]));
        let private_transcript: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Bytes(vec![20, 30, 40])),
            encode(IrValue::Bytes(vec![40, 50, 60])),
            encode(IrValue::Bytes(vec![40, 30, 20])),
        ]
        .concat();

        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs,
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_slice_out_of_bounds_fails() {
        // start + len > n must be rejected (off-circuit preprocess).
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [
              { "name": "%b", "type": "Bytes<4>" }
           ],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "slice", "bytes": "%b", "start": 2, "len": 3, "output": "%x" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();
        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: encode(IrValue::Bytes(vec![0, 1, 2, 3])),
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let err = preimage
            .check(&ir)
            .expect_err("slice out of bounds should be rejected");
        assert!(
            err.to_string().contains("out of bounds"),
            "unexpected error: {err}"
        );
    }

    #[actix_rt::test]
    async fn test_constant_proof() {
        // `load_constant` loads a fixed value of any type: `encoding` is the
        // value's encoded form (the field elements `encode` produces), written
        // as hex immediates. Each constant is checked against a private input
        // carrying the same value.
        //
        // The encodings are spelled out explicitly here as a reference for
        // producers (e.g. Compact):
        //   Scalar<BLS12-381> 42       -> ["0x2a"]
        //   Byte 7                     -> ["0x07"]
        //   Bytes<3> [10, 20, 30]      -> ["0x0a141e"]        (little-endian)
        //   Point<Jubjub> generator    -> [<x>, <y>]          (affine coords)
        use group::Group;

        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "load_constant", "type": "Scalar<BLS12-381>", "encoding": ["0x2a"], "output": "%cn" },
               { "op": "load_constant", "type": "Byte", "encoding": ["0x07"], "output": "%cby" },
               { "op": "load_constant", "type": "Bytes<3>", "encoding": ["0x0a141e"], "output": "%cbs" },
               { "op": "load_constant", "type": "Point<Jubjub>", "encoding": ["0xe56fd518a3fe2d515f37be7f655c3104eef572b1e37ed35ea31c123a67c4a53e", "0xcb550cd538ea0cc1138480408e6eaab9b36c613f0dd3f7784fdb6eea837b1357"], "output": "%cp" },
               { "op": "private_input", "type": "Scalar<BLS12-381>", "guard": null, "output": "%pn" },
               { "op": "private_input", "type": "Byte", "guard": null, "output": "%pby" },
               { "op": "private_input", "type": "Bytes<3>", "guard": null, "output": "%pbs" },
               { "op": "private_input", "type": "Point<Jubjub>", "guard": null, "output": "%pp" },
               { "op": "constrain_eq", "a": "%cn",  "b": "%pn" },
               { "op": "constrain_eq", "a": "%cby", "b": "%pby" },
               { "op": "constrain_eq", "a": "%cbs", "b": "%pbs" },
               { "op": "constrain_eq", "a": "%cp",  "b": "%pp" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();
        let (pk, vk) = ir.keygen(&TestParams).await.unwrap();

        let encode = |v: IrValue| -> Vec<transient_crypto::curve::Fr> {
            encode_offcircuit(&v)
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect()
        };
        // Private inputs carrying the same values the constants decode to.
        let private_transcript: Vec<transient_crypto::curve::Fr> = [
            encode(IrValue::Native(42.into())),
            encode(IrValue::Byte(7)),
            encode(IrValue::Bytes(vec![10, 20, 30])),
            encode(IrValue::JubjubPoint(JubjubSubgroup::generator())),
        ]
        .concat();

        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: vec![],
            private_transcript,
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        let (proof, _) = preimage
            .prove::<IrSource>(
                &mut ChaCha20Rng::from_seed([42; 32]),
                &TestParams,
                &TestResolver {
                    pk: pk.clone(),
                    vk: vk.clone(),
                    ir: ir.clone(),
                },
            )
            .await
            .unwrap();
        vk.verify(&PARAMS_VERIFIER, &proof, [42.into()].into_iter())
            .unwrap();
    }

    #[test]
    fn test_constant_bad_encoding_rejected() {
        // A Native constant needs exactly one field element; two is invalid and
        // must be rejected at IR preprocess.
        let ir_raw = r#"{
           "version": { "major": 3, "minor": 0 },
           "inputs": [],
           "outputs": [],
           "do_communications_commitment": false,
           "instructions": [
               { "op": "load_constant", "type": "Scalar<BLS12-381>", "encoding": ["0x01", "0x02"], "output": "%c" }
           ]
        }"#;
        let ir = IrSource::load(ir_raw.as_bytes()).unwrap();
        let preimage = ProofPreimage {
            binding_input: 42.into(),
            communications_commitment: None,
            inputs: vec![],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            key_location: KeyLocation(Cow::Borrowed("builtin")),
        };
        assert!(
            preimage.check(&ir).is_err(),
            "a Native constant with 2 field elements should be rejected"
        );
    }
}
