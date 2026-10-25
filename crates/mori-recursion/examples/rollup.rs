//! Local stock-prover B3 stages. Only public artifacts and measurements are written.
use ff::Field;
use group::Curve;
use midnight_circuits::hash::poseidon::PoseidonState;
use midnight_curves::Bls12;
use midnight_proofs::{
    circuit::Value,
    dev::MockProver,
    poly::{commitment::Params, kzg::params::ParamsKZG},
    transcript::{Hashable, Sampleable, TranscriptHash},
    utils::SerdeFormat,
};
use midnight_zk_stdlib::{
    MidnightCircuit, Relation, cost_model, prove, setup_pk, setup_vk, verify,
};
use mori_recursion::{
    decider::{CommBinding, DeciderRelation, DeciderWitness},
    inner::{verify_offcircuit, vk_name},
    rollup::{
        i1::fp::dom,
        parent::{ParentWitness, RollupParent},
    },
    template::Template,
    util::{F, f_from_hex, f_to_hex, peak_rss_kb, sha256_hex, vk_bytes, vk_from_bytes},
};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use serde_json::json;
use std::{
    fs,
    io::{self, Cursor, Write},
    path::{Path, PathBuf},
    time::Instant,
};

#[cfg(not(feature = "sherlock-rollup-prefix"))]
use mori_recursion::rollup::batch::{BatchRelation as BatchLeaf, load_batch};
#[cfg(feature = "sherlock-rollup-prefix")]
use mori_recursion::rollup::uniform::UniformBatch as BatchLeaf;

#[cfg(feature = "sherlock-rollup-prefix")]
const OFFLINE_TEMPLATE: &[u8] = include_bytes!("../tests/fixtures/rollup-template.json");
#[cfg(not(feature = "sherlock-rollup-prefix"))]
const OFFLINE_TEMPLATE: &[u8] = include_bytes!("../tests/fixtures/accept-template.json");

type Pos = PoseidonState<F>;
type BoxError = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, BoxError>;
const STOCK_PROVER: &str = "midnight-proofs 0.8.3 registry stock";
const PROVER: &str = match option_env!("SHERLOCK_PROVER_ID") {
    Some(identity) => identity,
    None => STOCK_PROVER,
};
// The separate maintainer Loki package defines this feature. The stock
// workspace deliberately has no GPU dependency or public GPU build option.
#[allow(unexpected_cfgs)]
const GPU_BUILD: bool = cfg!(feature = "gpu");

fn error(msg: impl std::fmt::Display) -> BoxError {
    io::Error::other(msg.to_string()).into()
}
fn read_instance(dir: &Path, name: &str) -> Result<Vec<F>> {
    let values: Vec<String> =
        serde_json::from_slice(&fs::read(dir.join(format!("{name}.instance.json")))?)?;
    Ok(values
        .iter()
        .map(|v| f_from_hex(v))
        .collect::<io::Result<Vec<_>>>()?)
}
fn write_json(path: &Path, value: &impl serde::Serialize) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    fs::write(path, bytes)?;
    Ok(())
}
fn read_params(path: &Path, k: u32) -> Result<ParamsKZG<Bls12>> {
    let raw = fs::read(path)?;
    if raw.len() != 201326980
        || sha256_hex(&raw) != "1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1"
    {
        return Err(error("SRS differs from pinned k20 ceremony"));
    }
    let mut cursor = Cursor::new(raw);
    let mut params = ParamsKZG::<Bls12>::read_custom(&mut cursor, SerdeFormat::RawBytesUnchecked)?;
    if cursor.position() != cursor.get_ref().len() as u64 || params.max_k() != 20 || k > 20 {
        return Err(error("SRS shape/degree"));
    }
    params.downsize(k);
    Ok(params)
}

struct CountBytes(u64);
impl Write for CountBytes {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        self.0 += b.len() as u64;
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn prove_stage<R, H>(
    name: &str,
    dir: &Path,
    srs: &Path,
    relation: &R,
    instance: Vec<F>,
    witness: R::Witness,
) -> Result<()>
where
    R: Relation<Instance = Vec<F>, Error = midnight_proofs::plonk::Error>,
    H: TranscriptHash,
    midnight_curves::G1Projective: Hashable<H>,
    F: Hashable<H> + Sampleable<H>,
{
    prove_many::<R, H>(
        name,
        srs,
        relation,
        vec![(dir.to_path_buf(), instance, witness)],
    )
}

fn prove_many<R, H>(
    name: &str,
    srs: &Path,
    relation: &R,
    inputs: Vec<(PathBuf, Vec<F>, R::Witness)>,
) -> Result<()>
where
    R: Relation<Instance = Vec<F>, Error = midnight_proofs::plonk::Error>,
    H: TranscriptHash,
    midnight_curves::G1Projective: Hashable<H>,
    F: Hashable<H> + Sampleable<H>,
{
    let total = Instant::now();
    let start = Instant::now();
    let probe = cost_model(relation, None);
    let model = cost_model(relation, Some(probe.k));
    if model.k > 20 {
        return Err(error(format!("{name} exceeds k20")));
    }
    let model_seconds = start.elapsed().as_secs_f64();
    let model_rss = peak_rss_kb() * 1024;
    eprintln!("{name} modeled k={} rows={}", model.k, model.rows);
    let start = Instant::now();
    let params = read_params(srs, model.k)?;
    let parameter_seconds = start.elapsed().as_secs_f64();
    let parameter_rss = peak_rss_kb() * 1024;
    let start = Instant::now();
    let vk = setup_vk(&params, relation);
    let vk_seconds = start.elapsed().as_secs_f64();
    let vk_rss = peak_rss_kb() * 1024;
    let raw_vk = vk_bytes(&vk);
    let start = Instant::now();
    let pk = setup_pk(relation, &vk);
    let pk_seconds = start.elapsed().as_secs_f64();
    let pk_rss = peak_rss_kb() * 1024;
    let start = Instant::now();
    let mut counter = CountBytes(0);
    pk.write(&mut counter, SerdeFormat::RawBytes)?;
    let pk_size_seconds = start.elapsed().as_secs_f64();
    let count = inputs.len();
    for (dir, instance, witness) in inputs {
        fs::create_dir_all(&dir)?;
        eprintln!("{name} keygen complete peak_rss={pk_rss}B; proving {PROVER}");
        let start = Instant::now();
        let proof = prove::<R, H>(
            &params,
            &pk,
            relation,
            &instance,
            witness,
            ChaCha20Rng::from_entropy(),
        )
        .map_err(|e| error(format!("{name} prove: {e:?}")))?;
        let prove_seconds = start.elapsed().as_secs_f64();
        let prove_rss = peak_rss_kb() * 1024;
        let start = Instant::now();
        verify::<R, H>(&params.verifier_params(), &vk, &instance, None, &proof)
            .map_err(|e| error(format!("{name} verify: {e:?}")))?;
        let verify_seconds = start.elapsed().as_secs_f64();
        let mut changed = instance.clone();
        changed[0] += F::ONE;
        if verify::<R, H>(&params.verifier_params(), &vk, &changed, None, &proof).is_ok() {
            return Err(error("changed instance accepted"));
        }
        let mut relation_bytes = Vec::new();
        relation.write_relation(&mut relation_bytes)?;
        fs::write(dir.join(format!("{name}.relation")), &relation_bytes)?;
        fs::write(dir.join(format!("{name}.vk")), &raw_vk)?;
        fs::write(dir.join(format!("{name}.proof")), &proof)?;
        write_json(
            &dir.join(format!("{name}.instance.json")),
            &instance.iter().map(f_to_hex).collect::<Vec<_>>(),
        )?;
        let receipt = json!({"schema":"sherlock-b3-stage/1","layer":name,"prover":PROVER,
            "model":model,"domain_capacity":1u64<<model.k,"model_seconds":model_seconds,
            "parameter_read_downsize_seconds":parameter_seconds,"vk_keygen_seconds":vk_seconds,
            "pk_keygen_seconds":pk_seconds,"pk_measurement_seconds":pk_size_seconds,
            "prove_seconds":prove_seconds,"local_verify_seconds":verify_seconds,"process_wall_seconds":total.elapsed().as_secs_f64(),
            "peak_rss_bytes":peak_rss_kb()*1024,"rss_scope":"process lifetime, includes oracle/model/parameters/keygen/prove/verify",
        "phase_peak_rss_bytes":{"model":model_rss,"parameters":parameter_rss,"vk":vk_rss,"pk":pk_rss,"prove":prove_rss},
        "phase_rss_scope":"process cumulative high-water snapshots at phase end, not isolated phase allocation",
            "proof_bytes":proof.len(),"vk_bytes":raw_vk.len(),"pk_raw_bytes":counter.0,
            "proof_sha256":sha256_hex(&proof),"vk_sha256":sha256_hex(&raw_vk),"relation_sha256":sha256_hex(&relation_bytes),
            "instance_sha256":sha256_hex(&fs::read(dir.join(format!("{name}.instance.json")))?),
            "negative_changed_instance":"rejected","threads":4,"shared_keygen_proofs":count});
        write_json(&dir.join(format!("{name}.stats.json")), &receipt)?;
        println!(
            "PASS {name}: {} proof bytes, {:.3}s prove, {:.3}s verify",
            proof.len(),
            prove_seconds,
            verify_seconds
        );
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args == ["identity"] {
        println!(
            "{}",
            json!({"prover":PROVER,"fixed_key_prefix":cfg!(feature="sherlock-rollup-prefix"),"gpu":GPU_BUILD})
        );
        return Ok(());
    }
    if args.len() != 5 {
        return Err(error(
            "usage: rollup <mock|leaf|parent|decider|verify> <corpus> <batch 1..6> <output> <k20-srs>",
        ));
    }
    let mode = &args[0];
    if (mode == "verify" || mode == "prefix-verify") && PROVER != STOCK_PROVER {
        return Err(error(
            "independent verification requires the retained stock binary",
        ));
    }
    let corpus = PathBuf::from(&args[1]);
    let batch: u64 = args[2].parse()?;
    let out = PathBuf::from(&args[3]);
    let srs = PathBuf::from(&args[4]);
    fs::create_dir_all(&out)?;
    if std::env::var("RAYON_NUM_THREADS").as_deref() != Ok("4") {
        return Err(error("RAYON_NUM_THREADS must be 4"));
    }
    #[cfg(feature = "sherlock-rollup-prefix")]
    if mode.starts_with("prefix-") {
        return prefix_stage(mode, &corpus, batch, &out, &srs);
    }
    #[cfg(not(feature = "sherlock-rollup-prefix"))]
    let (leaf, w, expected) = load_batch(&corpus, batch)?;
    #[cfg(feature = "sherlock-rollup-prefix")]
    let (leaf, w, expected) = BatchLeaf::load(&corpus, batch)?;
    match mode.as_str() {
        "mock" => {
            let model = cost_model(&leaf, None);
            if model.k > 20 {
                return Err(error("batch exceeds k20"));
            }
            let c = MidnightCircuit::new(
                &leaf,
                Value::known(expected.clone()),
                Value::known(w),
                Some(model.k),
            );
            MockProver::run(&c, vec![vec![], expected])?
                .verify()
                .map_err(|_| error("honest batch constraints unsatisfied"))?;
            println!("PASS complete v3 batch MockProver k{}", model.k);
            Ok(())
        }
        "leaf" => prove_stage::<_, Pos>("leaf", &out, &srs, &leaf, expected, w),
        "parent" => {
            let relation = RollupParent {
                child_vk: vk_from_bytes(&fs::read(out.join("leaf.vk"))?)?,
            };
            let witness = ParentWitness {
                instance: read_instance(&out, "leaf")?,
                proof: fs::read(out.join("leaf.proof"))?,
            };
            if witness.instance != expected {
                return Err(error("leaf differs from corpus"));
            }
            let params = read_params(&srs, 20)?;
            verify::<BatchLeaf, Pos>(
                &params.verifier_params(),
                &relation.child_vk,
                &expected,
                None,
                &witness.proof,
            )
            .map_err(|e| error(format!("child: {e:?}")))?;
            let instance = relation.instance(&witness).map_err(error)?;
            drop(params);
            prove_stage::<_, Pos>("parent", &out, &srs, &relation, instance, witness)
        }
        "decider" => {
            let params = read_params(&srs, 20)?;
            let relation = DeciderRelation {
                nr_cols: 4,
                w_vk: vk_from_bytes(&fs::read(out.join("parent.vk"))?)?,
                n_w_instance: 29,
                s_g2: params.s_g2().to_affine(),
                dom_stmt: dom::stmt(),
                stmt_fields: vec![0, 1, 15, 16, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
                template: Template::from_any_json(OFFLINE_TEMPLATE)?,
                comm_binding: CommBinding::BoundPoseidonStmtV1,
            };
            relation.validate()?;
            let witness = DeciderWitness {
                w_instance: read_instance(&out, "parent")?,
                w_proof: fs::read(out.join("parent.proof"))?,
                instance_id: F::ZERO,
                batch_no: F::from(batch),
                comm_rand: F::ZERO,
            };
            if witness.w_instance[..17] != expected {
                return Err(error("parent statement differs from corpus"));
            }
            verify_offcircuit(
                &params.verifier_params(),
                &relation.w_vk,
                &vk_name("w", &relation.w_vk),
                &witness.w_instance,
                &witness.w_proof,
                true,
            )
            .map_err(error)?;
            let stmt = relation.stmt(&witness.w_instance, witness.instance_id, witness.batch_no);
            let instance = relation.template.public_inputs(
                F::ZERO,
                relation.comm_commitment_for(stmt, F::ZERO),
                stmt,
            );
            drop(params);
            prove_stage::<_, blake2b_simd::State>(
                "decider", &out, &srs, &relation, instance, witness,
            )
        }
        "verify" => verify_bundle(&out, &srs, &leaf, &expected, batch, true),
        _ => Err(error("unknown stage")),
    }
}

fn verify_bundle(
    out: &Path,
    srs: &Path,
    leaf: &impl Relation<Instance = Vec<F>, Error = midnight_proofs::plonk::Error>,
    expected: &Vec<F>,
    batch: u64,
    reconstruct_keys: bool,
) -> Result<()> {
    let started = Instant::now();
    let params = read_params(srs, 20)?;
    // Reconstruct every key from its expected relation in this separate process.
    let raw = fs::read(out.join("leaf.vk"))?;
    let leaf_vk = vk_from_bytes(&raw)?;
    let mut p = read_params(srs, leaf_vk.k() as u32)?;
    if reconstruct_keys && vk_bytes(&setup_vk(&p, leaf)) != raw {
        return Err(error(
            "leaf key differs from independently reconstructed relation",
        ));
    }
    if read_instance(out, "leaf")? != *expected {
        return Err(error("leaf statement differs from corpus"));
    }
    let proof = fs::read(out.join("leaf.proof"))?;
    verify::<BatchLeaf, Pos>(&params.verifier_params(), &leaf_vk, expected, None, &proof)
        .map_err(|e| error(format!("leaf verify: {e:?}")))?;
    let parent = RollupParent { child_vk: leaf_vk };
    let w = ParentWitness {
        instance: expected.clone(),
        proof,
    };
    let parent_expected = parent.instance(&w).map_err(error)?;
    if read_instance(out, "parent")? != parent_expected {
        return Err(error("parent accumulator/statement differs"));
    }
    let raw = fs::read(out.join("parent.vk"))?;
    let parent_vk = vk_from_bytes(&raw)?;
    p.downsize(parent_vk.k() as u32);
    if reconstruct_keys && vk_bytes(&setup_vk(&p, &parent)) != raw {
        return Err(error("parent key differs from independent reconstruction"));
    }
    let proof = fs::read(out.join("parent.proof"))?;
    verify::<RollupParent, Pos>(
        &params.verifier_params(),
        &parent_vk,
        &parent_expected,
        None,
        &proof,
    )
    .map_err(|e| error(format!("parent verify: {e:?}")))?;
    verify_offcircuit(
        &params.verifier_params(),
        &parent_vk,
        &vk_name("w", &parent_vk),
        &parent_expected,
        &proof,
        true,
    )
    .map_err(error)?;
    let d = DeciderRelation {
        nr_cols: 4,
        w_vk: parent_vk,
        n_w_instance: 29,
        s_g2: params.s_g2().to_affine(),
        dom_stmt: dom::stmt(),
        stmt_fields: vec![0, 1, 15, 16, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        template: Template::from_any_json(OFFLINE_TEMPLATE)?,
        comm_binding: CommBinding::BoundPoseidonStmtV1,
    };
    let raw = fs::read(out.join("decider.vk"))?;
    let dvk = vk_from_bytes(&raw)?;
    let p = read_params(srs, dvk.k() as u32)?;
    if reconstruct_keys && vk_bytes(&setup_vk(&p, &d)) != raw {
        return Err(error("decider key differs from independent reconstruction"));
    }
    let stmt = d.stmt(&parent_expected, F::ZERO, F::from(batch));
    let pis = d
        .template
        .public_inputs(F::ZERO, d.comm_commitment_for(stmt, F::ZERO), stmt);
    if read_instance(out, "decider")? != pis {
        return Err(error("terminal statement differs from corpus"));
    }
    let proof = fs::read(out.join("decider.proof"))?;
    verify::<DeciderRelation, blake2b_simd::State>(
        &params.verifier_params(),
        &dvk,
        &pis,
        None,
        &proof,
    )
    .map_err(|e| error(format!("decider verify: {e:?}")))?;
    let mut bad = pis.clone();
    bad[d.template.stmt_index()] += F::ONE;
    if verify::<DeciderRelation, blake2b_simd::State>(
        &params.verifier_params(),
        &dvk,
        &bad,
        None,
        &proof,
    )
    .is_ok()
    {
        return Err(error("wrong terminal statement accepted"));
    }
    write_json(
        &out.join("verified.json"),
        &json!({"schema":"sherlock-b3-verified/1","batch":batch,"transactions":16,"headers":50,
        "layers":["leaf","parent","decider"],"stock_verification":true,"independent_key_reconstruction":true,"shared_key_reconstruction_reused":!reconstruct_keys,
        "separate_process_wall_seconds":started.elapsed().as_secs_f64(),"terminal_statement":f_to_hex(&stmt),
        "scope":"S0 v3 synthetic era-zero batch, stock recursion and local terminal verification; no ledger call"}),
    )?;
    println!(
        "PASS separately reconstructed and stock-verified leaf/parent/decider; wrong statement rejected"
    );
    Ok(())
}

#[cfg(feature = "sherlock-rollup-prefix")]
fn prefix_stage(mode: &str, corpus: &Path, count: u64, out: &Path, srs: &Path) -> Result<()> {
    if ![2, 4, 6].contains(&count) {
        return Err(error("prefix must contain 2, 4 or 6 complete batches"));
    }
    let batch_dir = |batch| out.join(format!("batch-{batch:02}"));
    let mut leaves = Vec::new();
    for batch in 1..=count {
        let (_, witness, expected) = BatchLeaf::load(corpus, batch)?;
        leaves.push((batch_dir(batch), expected, witness));
    }
    match mode {
        "prefix-leaf" => prove_many::<_, Pos>("leaf", srs, &BatchLeaf, leaves),
        "prefix-parent" => {
            let raw = fs::read(batch_dir(1).join("leaf.vk"))?;
            let relation = RollupParent {
                child_vk: vk_from_bytes(&raw)?,
            };
            let params = read_params(srs, 20)?;
            let mut inputs = Vec::new();
            for (dir, expected, _) in leaves {
                if fs::read(dir.join("leaf.vk"))? != raw || read_instance(&dir, "leaf")? != expected
                {
                    return Err(error("prefix leaf key/statement differs"));
                }
                let witness = ParentWitness {
                    instance: expected,
                    proof: fs::read(dir.join("leaf.proof"))?,
                };
                verify::<BatchLeaf, Pos>(
                    &params.verifier_params(),
                    &relation.child_vk,
                    &witness.instance,
                    None,
                    &witness.proof,
                )
                .map_err(|e| error(format!("prefix child: {e:?}")))?;
                inputs.push((dir, relation.instance(&witness).map_err(error)?, witness));
            }
            drop(params);
            prove_many::<_, Pos>("parent", srs, &relation, inputs)
        }
        "prefix-decider" => {
            let raw = fs::read(batch_dir(1).join("parent.vk"))?;
            let params = read_params(srs, 20)?;
            let relation = DeciderRelation {
                nr_cols: 4,
                w_vk: vk_from_bytes(&raw)?,
                n_w_instance: 29,
                s_g2: params.s_g2().to_affine(),
                dom_stmt: dom::stmt(),
                stmt_fields: vec![0, 1, 15, 16, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
                template: Template::from_any_json(OFFLINE_TEMPLATE)?,
                comm_binding: CommBinding::BoundPoseidonStmtV1,
            };
            relation.validate()?;
            let mut inputs = Vec::new();
            for (i, (dir, expected, _)) in leaves.into_iter().enumerate() {
                let parent = read_instance(&dir, "parent")?;
                if fs::read(dir.join("parent.vk"))? != raw
                    || parent.len() != 29
                    || parent[..17] != expected
                {
                    return Err(error("prefix parent key/statement differs"));
                }
                let witness = DeciderWitness {
                    w_instance: parent,
                    w_proof: fs::read(dir.join("parent.proof"))?,
                    instance_id: F::ZERO,
                    batch_no: F::from(i as u64 + 1),
                    comm_rand: F::ZERO,
                };
                verify_offcircuit(
                    &params.verifier_params(),
                    &relation.w_vk,
                    &vk_name("w", &relation.w_vk),
                    &witness.w_instance,
                    &witness.w_proof,
                    true,
                )
                .map_err(error)?;
                let stmt =
                    relation.stmt(&witness.w_instance, witness.instance_id, witness.batch_no);
                let public = relation.template.public_inputs(
                    F::ZERO,
                    relation.comm_commitment_for(stmt, F::ZERO),
                    stmt,
                );
                inputs.push((dir, public, witness));
            }
            drop(params);
            prove_many::<_, blake2b_simd::State>("decider", srs, &relation, inputs)
        }
        "prefix-verify" => {
            let first = batch_dir(1);
            // Reconstruct the three keys once in this separate process. Every
            // following batch must use those exact bytes, not a supplied new key.
            for (i, (dir, expected, _)) in leaves.into_iter().enumerate() {
                for layer in ["leaf", "parent", "decider"] {
                    for suffix in ["vk", "relation"] {
                        let file = format!("{layer}.{suffix}");
                        if fs::read(dir.join(&file))? != fs::read(first.join(&file))? {
                            return Err(error("prefix changed its fixed relation or key"));
                        }
                    }
                }
                verify_bundle(&dir, srs, &BatchLeaf, &expected, i as u64 + 1, i == 0)?;
            }
            Ok(())
        }
        _ => Err(error("unknown prefix stage")),
    }
}
