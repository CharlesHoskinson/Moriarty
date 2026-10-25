//! B4 offline key preparation and in-memory SDK call opening for the S0 anchor.
//! No wallet access or submission; only public artifacts are written.
use ff::Field;
use group::Curve;
use midnight_curves::Bls12;
use midnight_proofs::{
    poly::{commitment::Params, kzg::params::ParamsKZG},
    utils::SerdeFormat,
};
use midnight_transient_crypto::{
    curve::Fr,
    proofs::{PARAMS_VERIFIER, Proof, ProofPreimage, VerifierKey},
};
use midnight_zk_stdlib::{Relation, cost_model, prove, setup_pk, setup_vk, verify};
use mori_recursion::{
    decider::{CommBinding, DeciderRelation, DeciderWitness},
    inner::{verify_offcircuit, vk_name},
    parent::f_from_canonical_hex,
    rollup::i1::fp::dom,
    template::Template,
    util::{F, f_to_hex, peak_rss_kb, sha256_hex, vk_bytes, vk_from_bytes},
};
use std::{
    fs,
    io::{self, Cursor, Read, Write},
    path::{Path, PathBuf},
    time::Instant,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn bad(message: impl std::fmt::Display) -> Box<dyn std::error::Error> {
    io::Error::other(message.to_string()).into()
}
fn write_new(root: &Path, name: &str, bytes: &[u8]) -> Result<()> {
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(root.join(name))?
        .write_all(bytes)?;
    Ok(())
}
fn json_new(root: &Path, name: &str, value: &impl serde::Serialize) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    write_new(root, name, &bytes)
}
fn params(path: &Path) -> Result<ParamsKZG<Bls12>> {
    let raw = fs::read(path)?;
    if raw.len() != 201326980
        || sha256_hex(&raw) != "1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1"
    {
        return Err(bad("SRS differs from the pinned k20 ceremony"));
    }
    let mut cursor = Cursor::new(raw);
    let p = ParamsKZG::read_custom(&mut cursor, SerdeFormat::RawBytesUnchecked)?;
    if cursor.position() != cursor.get_ref().len() as u64 || p.max_k() != 20 {
        return Err(bad("SRS shape"));
    }
    Ok(p)
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct CallOpening {
    binding_input: String,
    preimage: String,
}

fn opening(call: CallOpening, stmt: F, template: &Template) -> Result<(F, Vec<F>)> {
    let raw = hex::decode(call.preimage).map_err(|_| bad("invalid call preimage encoding"))?;
    let mut reader = &raw[..];
    let p: ProofPreimage = midnight_serialize::tagged_deserialize(&mut reader)
        .map_err(|_| bad("invalid call preimage"))?;
    if !reader.is_empty() || p.inputs.len() != 1 || p.inputs[0].0 != stmt {
        return Err(bad("call is not the expected one-input rollup statement"));
    }
    if !p.private_transcript.is_empty() {
        return Err(bad("unexpected private call transcript"));
    }
    // The replay guard has one Popeq query result: settled.member(stmt) is
    // false. This is a public VM query output, not a circuit return value.
    if p.public_transcript_outputs != [Fr(F::ZERO)] {
        return Err(bad("expected the false replay-membership query result"));
    }
    let binding = f_from_canonical_hex(&call.binding_input)?;
    let (comm, rand) = p
        .communications_commitment
        .ok_or_else(|| bad("missing communication opening"))?;
    if midnight_transient_crypto::hash::transient_commit(&p.inputs, rand) != comm {
        return Err(bad("invalid communication opening"));
    }
    let actual = [
        vec![binding, comm.0],
        p.public_transcript_inputs.iter().map(|f| f.0).collect(),
    ]
    .concat();
    if actual != template.public_inputs(binding, comm.0, stmt) {
        return Err(bad(
            "SDK public call vector differs from the complete template",
        ));
    }
    Ok((rand.0, actual))
}

fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 7 {
        return Err(bad(
            "usage: rollup_call prepare|check|prove B3_DIR TEMPLATE KEY_DIR OUTPUT INSTANCE_HEX SRS",
        ));
    }
    if std::env::var("RAYON_NUM_THREADS").as_deref() != Ok("4") {
        return Err(bad("RAYON_NUM_THREADS must be 4"));
    }
    let mode = &args[0];
    let input = PathBuf::from(&args[1]);
    let key = PathBuf::from(&args[3]);
    let out = PathBuf::from(&args[4]);
    let instance_id = f_from_canonical_hex(&args[5])?;
    if instance_id == F::ZERO {
        return Err(bad("B4 deployment id must be nonzero"));
    }
    if !matches!(mode.as_str(), "prepare" | "check" | "prove") {
        return Err(bad("unknown mode"));
    }
    let values: Vec<String> =
        serde_json::from_slice(&fs::read(input.join("parent.instance.json"))?)?;
    let instance = values
        .iter()
        .map(|v| f_from_canonical_hex(v))
        .collect::<io::Result<Vec<_>>>()?;
    if instance.len() != 29 {
        return Err(bad("B3 parent must have 29 lanes"));
    }
    let parent_proof = fs::read(input.join("parent.proof"))?;
    let parent_raw = fs::read(input.join("parent.vk"))?;
    let parent_vk = vk_from_bytes(&parent_raw)?;
    let template_raw = fs::read(&args[2])?;
    let template = Template::from_any_json(&template_raw)?;
    if template.slots.len() != 30
        || template
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| (*s == mori_recursion::template::Slot::Stmt).then_some(i))
            .collect::<Vec<_>>()
            != [11, 25]
    {
        return Err(bad(
            "B4 requires the captured 30-lane replay-check template",
        ));
    }
    let p = params(Path::new(&args[6]))?;
    verify_offcircuit(
        &p.verifier_params(),
        &parent_vk,
        &vk_name("w", &parent_vk),
        &instance,
        &parent_proof,
        true,
    )
    .map_err(bad)?;
    let relation = DeciderRelation {
        nr_cols: 4,
        w_vk: parent_vk,
        n_w_instance: 29,
        s_g2: p.s_g2().to_affine(),
        dom_stmt: dom::stmt(),
        stmt_fields: vec![0, 1, 15, 16, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        template,
        comm_binding: CommBinding::BoundPoseidonStmtV1,
    };
    relation.validate()?;
    let batch = (instance[15] - F::ONE) * F::from(50).invert().unwrap() + F::ONE;
    let stmt = relation.stmt(&instance, instance_id, batch);
    let mut relation_bytes = Vec::new();
    relation.write_relation(&mut relation_bytes)?;
    fs::create_dir_all(&out)?;
    if mode == "prepare" {
        if let Some(parent) = key.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::create_dir(&key)?;
        let started = Instant::now();
        // B3 measured the production bound. Check the actual k20 layout once;
        // do not repeatedly search smaller domains during terminal preparation.
        let model = cost_model(&relation, Some(20));
        if model.k != 20 || model.rows + model.nb_unusable_rows > 1 << 20 {
            return Err(bad("B4 terminal exceeds the measured k20 domain"));
        }
        let vk = setup_vk(&p, &relation);
        let raw = vk_bytes(&vk);
        let ledger_key = VerifierKey::from(vk);
        let mut tagged = Vec::new();
        midnight_serialize::tagged_serialize(&ledger_key, &mut tagged)?;
        write_new(&key, "decider.relation", &relation_bytes)?;
        write_new(&key, "decider.vk", &raw)?;
        write_new(&key, "acceptValidity.verifier", &tagged)?;
        let receipt = serde_json::json!({"schema":"sherlock-b4-preparation/1","parent_vk_sha256":sha256_hex(&parent_raw),
            "relation_sha256":sha256_hex(&relation_bytes),"vk_sha256":sha256_hex(&raw),"tagged_vk_sha256":sha256_hex(&tagged),
            "template_sha256":sha256_hex(&template_raw),"instance_id":f_to_hex(&instance_id),"statement":f_to_hex(&stmt),
            "batch_no":f_to_hex(&batch),"model":model,"keygen_wall_seconds":started.elapsed().as_secs_f64(),
            "peak_rss_bytes":peak_rss_kb()*1024,"submitted":false});
        json_new(&key, "manifest.json", &receipt)?;
        json_new(&out, "preparation.json", &receipt)?;
        println!("{}", receipt);
        return Ok(());
    }
    if fs::read(key.join("decider.relation"))? != relation_bytes {
        return Err(bad("prepared relation differs from requested B4 relation"));
    }
    let raw_vk = fs::read(key.join("decider.vk"))?;
    let vk = vk_from_bytes(&raw_vk)?;
    let call: CallOpening = serde_json::from_reader(std::io::stdin().lock().take(2 * 1024 * 1024))
        .map_err(|_| bad("invalid pipe-only call input"))?;
    let (comm_rand, public) = opening(call, stmt, &relation.template)?;
    if mode == "check" {
        let receipt = serde_json::json!({"schema":"sherlock-b4-call-check/1","statement":f_to_hex(&stmt),
            "public_inputs":public.iter().map(f_to_hex).collect::<Vec<_>>(),"opening_persisted":false,"submitted":false});
        json_new(&out, "call-checked.json", &receipt)?;
        println!("{}", receipt);
        return Ok(());
    }
    let fresh = setup_vk(&p, &relation);
    if vk_bytes(&fresh) != raw_vk {
        return Err(bad(
            "prepared key differs from independent relation reconstruction",
        ));
    }
    let witness = DeciderWitness {
        w_instance: instance,
        w_proof: parent_proof,
        instance_id,
        batch_no: batch,
        comm_rand,
    };
    let started = Instant::now();
    let pk = setup_pk(&relation, &vk);
    let pk_seconds = started.elapsed().as_secs_f64();
    let proving = Instant::now();
    let proof =
        prove::<_, blake2b_simd::State>(&p, &pk, &relation, &public, witness, rand::rngs::OsRng)
            .map_err(|_| bad("call-bound D proving failed"))?;
    let prove_seconds = proving.elapsed().as_secs_f64();
    drop(pk);
    let verifying = Instant::now();
    verify::<DeciderRelation, blake2b_simd::State>(
        &p.verifier_params(),
        &vk,
        &public,
        None,
        &proof,
    )
    .map_err(|_| bad("call-bound D verification failed"))?;
    let local_verify_seconds = verifying.elapsed().as_secs_f64();
    let ledger_key = VerifierKey::from(vk);
    let ledger_verifying = Instant::now();
    ledger_key
        .verify(
            &PARAMS_VERIFIER,
            &Proof(proof.clone()),
            public.iter().copied().map(Fr),
        )
        .map_err(|_| bad("ledger verifier rejected call-bound D"))?;
    let ledger_verify_seconds = ledger_verifying.elapsed().as_secs_f64();
    for lane in 0..public.len() {
        let mut wrong = public.clone();
        wrong[lane] += F::ONE;
        if verify::<DeciderRelation, blake2b_simd::State>(
            &p.verifier_params(),
            &fresh,
            &wrong,
            None,
            &proof,
        )
        .is_ok()
        {
            return Err(bad("changed public input accepted"));
        }
    }
    let mut tagged = Vec::new();
    midnight_serialize::tagged_serialize(&Proof(proof.clone()), &mut tagged)?;
    write_new(&out, "terminal.proof", &proof)?;
    write_new(&out, "terminal.tagged-proof", &tagged)?;
    json_new(
        &out,
        "terminal.public.json",
        &public.iter().map(f_to_hex).collect::<Vec<_>>(),
    )?;
    let receipt = serde_json::json!({"schema":"sherlock-b4-call-proof/1","vk_sha256":sha256_hex(&raw_vk),
        "proof_sha256":sha256_hex(&proof),"statement":f_to_hex(&stmt),"public_input_count":public.len(),
        "keygen_prove_verify_wall_seconds":started.elapsed().as_secs_f64(),"peak_rss_bytes":peak_rss_kb()*1024,
        "pk_keygen_seconds":pk_seconds,"prove_seconds":prove_seconds,"local_verify_seconds":local_verify_seconds,
        "ledger_verify_seconds":ledger_verify_seconds,"timing_scope":"isolated host wall timers; not validator execution time",
        "negative_public_lanes_rejected":public.len(),"opening_persisted":false,"submitted":false});
    json_new(&out, "proof.json", &receipt)?;
    println!("{}", receipt);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use midnight_transient_crypto::proofs::KeyLocation;
    use mori_recursion::template::Slot;

    fn template() -> Template {
        let mut slots = (0..30).map(|i| Slot::Const(F::from(i))).collect::<Vec<_>>();
        slots[0] = Slot::BindingInput;
        slots[1] = Slot::CommunicationCommitment;
        slots[11] = Slot::Stmt;
        slots[25] = Slot::Stmt;
        Template { slots }
    }
    fn preimage(t: &Template) -> ProofPreimage {
        let inputs = vec![Fr(F::from(42))];
        let rand = Fr(F::from(17));
        let comm = midnight_transient_crypto::hash::transient_commit(&inputs, rand);
        let public = t.public_inputs(F::from(9), comm.0, inputs[0].0);
        ProofPreimage {
            inputs,
            private_transcript: vec![],
            public_transcript_inputs: public[2..].iter().copied().map(Fr).collect(),
            public_transcript_outputs: vec![Fr(F::ZERO)],
            binding_input: Fr(F::ZERO),
            communications_commitment: Some((comm, rand)),
            key_location: KeyLocation("sherlock/rollup/acceptValidity".into()),
        }
    }
    fn call(p: &ProofPreimage) -> CallOpening {
        let mut raw = Vec::new();
        midnight_serialize::tagged_serialize(p, &mut raw).unwrap();
        CallOpening {
            binding_input: f_to_hex(&F::from(9)),
            preimage: hex::encode(raw),
        }
    }
    #[test]
    fn pipe_opening_checks_all_public_slots_without_persisting_the_opening() {
        let t = template();
        let p = preimage(&t);
        let (rand, public) = opening(call(&p), F::from(42), &t).unwrap();
        assert_eq!(rand, F::from(17));
        assert_eq!(public.len(), 30);
        assert_eq!(public[11], public[25]);
        assert!(opening(call(&p), F::from(43), &t).is_err());
        let mut p = preimage(&t);
        p.public_transcript_inputs[23].0 += F::ONE;
        assert!(opening(call(&p), F::from(42), &t).is_err());
        let mut p = preimage(&t);
        p.private_transcript.push(Fr(F::ONE));
        assert!(opening(call(&p), F::from(42), &t).is_err());
        let mut p = preimage(&t);
        p.communications_commitment.as_mut().unwrap().0.0 += F::ONE;
        assert!(opening(call(&p), F::from(42), &t).is_err());
    }
    #[test]
    fn accept_validity_preimage_requires_the_false_replay_membership_result() {
        let t = template();
        let mut p = preimage(&t);
        p.public_transcript_outputs.clear();
        assert!(opening(call(&p), F::from(42), &t).is_err());
        let mut p = preimage(&t);
        p.public_transcript_outputs[0] = Fr(F::ONE);
        assert!(opening(call(&p), F::from(42), &t).is_err());
        let mut p = preimage(&t);
        p.public_transcript_outputs.push(Fr(F::ONE));
        assert!(opening(call(&p), F::from(42), &t).is_err());
    }
}
