//! straw-recl9: the recursive ledger-9 proof of a StrawAnchor batch.
//!
//! Staging
//!   stage <history_dir> <batch> <out_dir>               16-transaction batch (mono/rec staging)
//!   stage-mini <batch_dir> <n_slots> <out_dir>          first n slots of a staged batch as a mini batch
//! Leaves (arch {poseidon, jubjub}, Poseidon transcript)
//!   cost-leaf <batch_dir> <n_tx>
//!   keygen-leaf <batch_dir> <n_tx> <leaf_key_dir>
//!   prove-leaves <batch_dir> <leaf_key_dir> <leaves_dir>
//! Node N (arch {bls12_381, poseidon}, Poseidon transcript; it is D's W)
//!   cost-node <batch_dir> <leaf_key_dir> <leaves_dir> [k]
//!   keygen-node <batch_dir> <leaf_key_dir> <leaves_dir> <node_key_dir> [k]
//!   prove-node <batch_dir> <leaf_key_dir> <leaves_dir> <node_key_dir> <w_dir>
//! Decider D (arch {bls12_381, poseidon}, Blake2b transcript; the ledger verifies it)
//!   cost-decider <w_dir> [k]
//!   keygen-decider <w_dir> <key_dir> [k]
//!   prove <w_dir> <key_dir> <out_dir> <K>               per call; same argv/env as straw-decider prove
//!   verify <key_dir> <out_dir> <K>
//! MockProver
//!   mock-leaf <batch_dir> <n_tx> <leaf_index> <case>
//!   mock-invalid <history_dir> <case>                   one history/invalid transaction as a 1-tx leaf
//!   mock-node <batch_dir> <leaf_key_dir> <leaves_dir> <case> [k]
//!   mock-decider <w_dir> <case> [k]
//!
//! prove/mock-decider read BINDING_INPUT, COMM_COMMITMENT, INSTANCE_ID,
//! BATCH_NO (32-byte LE hex), exactly like straw-decider prove.

use std::{
    fs,
    io::BufReader,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use ff::Field;
use group::{prime::PrimeCurveAffine, Curve};
use midnight_circuits::hash::poseidon::PoseidonState;
use midnight_curves::Bls12;
use midnight_proofs::{
    circuit::Value,
    dev::{cost_model::CircuitModel, MockProver},
    poly::kzg::params::{ParamsKZG, ParamsVerifierKZG},
    utils::SerdeFormat,
};
use midnight_zk_stdlib::{cost_model, prove, setup_pk, setup_vk, verify, MidnightCircuit, MidnightVK, Relation};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use serde_json::json;
use straw_recl9::{
    batch::{leaf_segment, BatchWitness, LeafWitness},
    decider::{CommBinding, DeciderRelation, DeciderWitness},
    i1::{
        fp::dom,
        types::{InvalidCase, Point},
    },
    inner::{acc_pi_len, verify_offcircuit},
    leaf::LeafRelation,
    node::{NodeRelation, NodeWitness, N_STATE},
    stage::{self, load_inputs, witness_from, BatchInput, SlotInput},
    template::{Slot, Template},
    transition::TransactionWitness,
    util::{f_from_hex, f_to_hex, peak_rss_kb, sha256_hex, vk_bytes, vk_from_bytes, F},
};

mod sherlock_cli;

type Blake = blake2b_simd::State;
type Pos = PoseidonState<F>;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Official SHA-256 pins of the Midnight SRS files (ledger-9 base-crypto
/// data_provider.rs, as recorded in decider/src/main.rs).
fn srs_pin(k: u32) -> Option<&'static str> {
    match k {
        19 => Some("8e8dc15c4362f05c912f1e770559a3945db3e58a374def416ed5d3e65ad5b10e"),
        20 => Some("1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1"),
        21 => Some("9cf1644a87f0f027ae5fc6278f91d823a6334ff3e338a29e2f2ef57d071ed64d"),
        _ => None,
    }
}

fn srs_file(k: u32) -> Option<PathBuf> {
    let home = PathBuf::from(std::env::var("HOME").unwrap());
    [root().join("srs"), root().join("../chain/params"), home.join(".cache/midnight/zk-params")]
        .into_iter()
        .map(|d| d.join(format!("bls_midnight_2p{k}")))
        .find(|p| p.exists())
}

fn params(k: u32) -> (ParamsKZG<Bls12>, serde_json::Value) {
    let t = Instant::now();
    let p = srs_file(k).unwrap_or_else(|| panic!("no Midnight SRS file 2^{k}"));
    let bytes = fs::read(&p).unwrap();
    let h = sha256_hex(&bytes);
    let pin = srs_pin(k);
    if let Some(pin) = pin {
        assert_eq!(h, pin, "SRS {p:?} does not match the official pin");
    }
    let r = ParamsKZG::read_custom(&mut BufReader::new(&bytes[..]), SerdeFormat::RawBytesUnchecked).expect("read SRS");
    eprintln!("[srs] read 2^{k} from {p:?} in {:.1}s (sha256 {h}, pin match {:?})", t.elapsed().as_secs_f64(), pin.map(|x| x == h));
    (r, json!({"file": p, "sha256": h, "official_pin_match": pin.map(|x| x == h)}))
}

/// The verifier parameters the ledger embeds (bls_midnight_2p14).
fn ledger_vp() -> ParamsVerifierKZG<Bls12> {
    params(14).0.verifier_params()
}

/// [tau]_2 of the ledger's embedded parameters; the SRS g2 must be the
/// standard generator (PG-muse R7).
fn s_g2_checked() -> midnight_curves::G2Affine {
    let p = params(14).0;
    assert_eq!(p.g2().to_affine(), midnight_curves::G2Affine::generator(), "SRS g2 is not the standard generator");
    p.s_g2().to_affine()
}

fn write_json(p: &Path, v: &serde_json::Value) {
    fs::write(p, serde_json::to_string_pretty(v).unwrap() + "\n").unwrap();
}

fn fields_json(v: &[F]) -> serde_json::Value {
    json!(v.iter().map(f_to_hex).collect::<Vec<_>>())
}

fn read_fields(p: &Path) -> Vec<F> {
    let v: serde_json::Value = serde_json::from_slice(&fs::read(p).unwrap()).unwrap();
    let arr = if v.is_array() { v } else { v["fields"].clone() };
    arr.as_array().unwrap().iter().map(|s| f_from_hex(s.as_str().unwrap()).unwrap()).collect()
}

fn template() -> (Template, String) {
    let p = std::env::var("TEMPLATE").map(PathBuf::from).unwrap_or_else(|_| root().join("spec/template.json"));
    (Template::from_any_json(&fs::read(&p).unwrap()).unwrap(), p.display().to_string())
}

fn dom_stmt() -> F {
    let v: serde_json::Value = serde_json::from_slice(&fs::read(root().join("spec/domains.json")).unwrap()).unwrap();
    let d = f_from_hex(v["DOM_STMT"].as_str().unwrap()).unwrap();
    assert_eq!(d, dom::stmt(), "spec DOM_STMT equals the history crate's DOM_STMT");
    d
}

fn free_inputs() -> (F, F, F, F) {
    let get = |k: &str, d: u64| std::env::var(k).map(|s| f_from_hex(&s).unwrap()).unwrap_or(F::from(d));
    (get("BINDING_INPUT", 0xb1d), get("COMM_COMMITMENT", 0), get("INSTANCE_ID", 0x5354524157), get("BATCH_NO", 1))
}

/// Private opening for a bound decider relation. `supplied` is the
/// COMM_RAND_LE value (32-byte LE hex) from the process environment on the
/// production path, or an explicit in-memory fixture on the mock path.
/// Never printed or recorded.
fn comm_rand_from(rel: &DeciderRelation, supplied: Option<&str>) -> stage::Result<F> {
    match rel.comm_binding {
        CommBinding::Free => Ok(F::ZERO),
        CommBinding::BoundPoseidonStmtV1 => {
            let s = supplied
                .ok_or("bound decider prove needs COMM_RAND_LE (private opening)")?;
            if s.len() != 64
                || !s.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
            {
                return Err("COMM_RAND_LE must be 64 lowercase hex characters".into());
            }
            f_from_hex(s).map_err(|e| e.to_string().into())
        }
    }
}

fn threads() -> String {
    std::env::var("RAYON_NUM_THREADS").unwrap_or("all".into())
}

fn cost_json<R: Relation>(rel: &R, k: Option<u32>) -> (serde_json::Value, u32) {
    let t = Instant::now();
    let m = cost_model(rel, k);
    cost_json_from_model(rel, &m, t.elapsed())
}

/// Format an already computed model; this helper never probes a circuit.
/// Parent plans report their actual automatic fitting model, not a minimum claim.
fn cost_json_from_model<R: Relation>(rel: &R, m: &CircuitModel, elapsed: Duration) -> (serde_json::Value, u32) {
    let v = json!({"rows": m.rows, "table_rows": m.table_rows, "k": m.k, "unusable_rows": m.nb_unusable_rows,
        "usable_rows_at_k": (1u64 << m.k) - m.nb_unusable_rows as u64, "advice_columns": m.advice_columns,
        "fixed_columns": m.fixed_columns, "lookups": m.lookups, "permutations": m.permutations,
        "proof_size_model_bytes": m.size, "arch": format!("{:?}", rel.used_chips()),
        "cost_model_seconds": elapsed.as_secs_f64(), "peak_rss_kb": peak_rss_kb()});
    (v, m.k)
}

/// MockProver verdict; the instance column is `pis`, the committed instance
/// column empty.
fn mock_verdict<R: Relation<Instance = Vec<F>>>(rel: &R, pis: Vec<F>, w: R::Witness, k: u32) -> (String, Vec<String>) {
    let circuit = MidnightCircuit::new(rel, Value::known(pis.clone()), Value::known(w), Some(k));
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        MockProver::run(&circuit, vec![vec![], pis.clone()]).map(|p| p.verify())
    }));
    match res {
        Ok(Ok(Ok(()))) => ("ACCEPT".into(), vec![]),
        Ok(Ok(Err(fails))) => {
            let n = fails.len();
            let first: Vec<String> = fails.iter().take(4).map(|f| format!("{f:?}").chars().take(300).collect()).collect();
            (format!("REJECT (constraints: {n} failures)"), first)
        }
        Ok(Err(e)) => (format!("REJECT (synthesis: {e:?})"), vec![]),
        Err(_) => ("REJECT (prover panic: no witness)".into(), vec![]),
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let p = |i: usize| PathBuf::from(&a[i]);
    let opt_k = |i: usize| a.get(i).map(|s| s.parse::<u32>().unwrap());
    let mode = a.get(1).map(String::as_str).unwrap_or("");
    let res: stage::Result<()> = match mode {
        "stage" => stage::stage_batch(&p(2), a[3].parse().unwrap(), &p(4)),
        "stage-mini" => stage_mini(&p(2), a[3].parse().unwrap(), &p(4)),
        "stage-poisoned" => stage_poisoned(&p(2), &p(3)),
        "cost-leaf" => {
            let rel = LeafRelation { n_tx: a[3].parse().unwrap() };
            let _ = load_inputs(&p(2)).unwrap();
            let (v, _) = cost_json(&rel, None);
            println!("{}", json!({"kind": "leaf", "n_tx": rel.n_tx, "cost": v}));
            Ok(())
        }
        "keygen-leaf" => keygen_leaf(&p(2), a[3].parse().unwrap(), &p(4)),
        "prove-leaves" => prove_leaves(&p(2), &p(3), &p(4)),
        "cost-node" => {
            let (rel, _) = node_setup(&p(2), &p(3), &p(4));
            let (v, _) = cost_json(&rel, opt_k(5));
            println!("{}", json!({"kind": "node", "arity": rel.arity, "cost": v}));
            Ok(())
        }
        "keygen-node" => keygen_node(&p(2), &p(3), &p(4), &p(5), opt_k(6)),
        "prove-node" => prove_node(&p(2), &p(3), &p(4), &p(5), &p(6)),
        "cost-decider" => {
            let rel = decider_rel(&p(2));
            let (v, _) = cost_json(&rel, opt_k(3));
            println!("{}", json!({"kind": "decider", "cost": v}));
            Ok(())
        }
        "keygen-decider" => keygen_decider(&p(2), &p(3), opt_k(4)),
        "prove" => prove_call(&p(2), &p(3), &p(4), a[5].parse().unwrap()),
        "verify" => verify_cmd(&p(2), &p(3), a[4].parse().unwrap()),
        "mock-leaf" => mock_leaf(&p(2), a[3].parse().unwrap(), a[4].parse().unwrap(), &a[5]),
        "mock-invalid" => mock_invalid(&p(2), &a[3]),
        "mock-node" => mock_node(&p(2), &p(3), &p(4), &a[5], opt_k(6)),
        "mock-decider" => mock_decider(&p(2), &a[3], opt_k(4)),
        "sherlock-keygen-parent" => sherlock_cli::keygen_parent(&a[2..]),
        "sherlock-prove-parent" => sherlock_cli::prove_parent(&a[2..]),
        "sherlock-verify-parent" => straw_recl9::sherlock_verify::verify_parent(&a[2..]),
        "sherlock-keygen-decider" => sherlock_cli::keygen_decider(&a[2..]),
        _ => Err("usage: see src/main.rs".into()),
    };
    if let Err(e) = res {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

// ---------------------------------------------------------------- staging

/// The first `n` slots of a staged batch as a mini batch: h_first stays,
/// h_last is the n-th slot's post height, state_post is that slot's state
/// after, at that height.
fn stage_mini(batch_dir: &Path, n: usize, out: &Path) -> stage::Result<()> {
    let (slots, b) = load_inputs(batch_dir)?;
    let slots: Vec<SlotInput> = slots[..n].to_vec();
    let last = &slots[n - 1];
    let mut state_post = last.tx.state_after.clone();
    state_post.height = last.post_height;
    let mut l0_index = b.l0_index[..n].to_vec();
    let filler = slots.iter().position(|s| !s.tx.public.kind.is_deposit()).ok_or("no spend")?;
    for (i, s) in slots.iter().enumerate() {
        if s.tx.public.kind.is_deposit() {
            l0_index[i] = filler;
        }
    }
    let mb = BatchInput {
        batch_no: b.batch_no,
        h_first: b.h_first,
        h_last: last.post_height,
        state_pre: b.state_pre.clone(),
        state_post,
        l0_index,
        statement: None,
        source: json!({"mini_of": batch_dir, "n_slots": n}),
    };
    stage::write_dir(out, &slots, &mb)?;
    println!("staged mini batch: first {n} slots of {}", batch_dir.display());
    Ok(())
}

/// STRAWMAN 2.6 poisoned batch (mono's form): batch 2 with
/// history/invalid/unbalanced_outputs_plus_one at slot 4.
fn stage_poisoned(history: &Path, out: &Path) -> stage::Result<()> {
    let tmp = out.with_extension("honest-tmp");
    if !tmp.exists() {
        stage::stage_batch(history, 2, &tmp)?;
    }
    let (mut slots, mut b) = load_inputs(&tmp)?;
    let c: InvalidCase = serde_json::from_slice(&fs::read(history.join("invalid/unbalanced_outputs_plus_one.json"))?)?;
    slots[4].tx = c.tx.ok_or("case has no transaction")?;
    b.l0_index[4] = 4;
    b.source = json!({"poisoned": "batch 2, slot 4 = history/invalid/unbalanced_outputs_plus_one.json"});
    stage::write_dir(out, &slots, &b)?;
    fs::remove_dir_all(&tmp)?;
    println!("staged poisoned batch 2 (slot 4 unbalanced)");
    Ok(())
}

fn batch_witness(batch_dir: &Path) -> BatchWitness {
    let (slots, b) = load_inputs(batch_dir).unwrap();
    let (_, _, iid, bno) = free_inputs();
    witness_from(&slots, &b, iid, bno)
}

// ---------------------------------------------------------------- leaves

fn keygen_leaf(batch_dir: &Path, n_tx: u32, key_dir: &Path) -> stage::Result<()> {
    fs::create_dir_all(key_dir)?;
    let _ = load_inputs(batch_dir)?;
    let rel = LeafRelation { n_tx };
    let (cost, k) = cost_json(&rel, None);
    let (pp, srs) = params(k);
    let t = Instant::now();
    let vk = setup_vk(&pp, &rel);
    let t_vk = t.elapsed().as_secs_f64();
    let vkb = vk_bytes(&vk);
    assert_eq!(vk_bytes(&vk_from_bytes(&vkb)?), vkb, "leaf vk round trip");
    let mut relb = Vec::new();
    rel.write_relation(&mut relb)?;
    fs::write(key_dir.join("leaf.vk"), &vkb)?;
    fs::write(key_dir.join("leaf.relation"), &relb)?;
    write_json(&key_dir.join("keygen.stats.json"), &json!({"kind": "leaf-keygen", "n_tx": n_tx, "cost": cost, "k": k,
        "keygen_vk_s": t_vk, "vk_bytes": vkb.len(), "vk_sha256": sha256_hex(&vkb), "srs": srs, "peak_rss_kb": peak_rss_kb(), "threads": threads()}));
    println!("leaf keygen n_tx={n_tx} k={k} vk {} B in {t_vk:.1}s", vkb.len());
    Ok(())
}

fn load_leaf_key(key_dir: &Path) -> (LeafRelation, MidnightVK) {
    let rel = LeafRelation::read_relation(&mut &fs::read(key_dir.join("leaf.relation")).unwrap()[..]).unwrap();
    let vk = vk_from_bytes(&fs::read(key_dir.join("leaf.vk")).unwrap()).unwrap();
    (rel, vk)
}

fn prove_leaves(batch_dir: &Path, key_dir: &Path, out: &Path) -> stage::Result<()> {
    fs::create_dir_all(out)?;
    let (rel, vk) = load_leaf_key(key_dir);
    let w = batch_witness(batch_dir);
    let leaves = w.leaves(rel.n_tx as usize);
    let k = vk.k() as u32;
    let (pp, srs) = params(k);
    let t = Instant::now();
    let pk = setup_pk(&rel, &vk);
    let t_pk = t.elapsed().as_secs_f64();
    let vp = ledger_vp();
    let name = straw_recl9::inner::vk_name("leaf", &vk);
    let mut stats = vec![];
    for (j, lw) in leaves.iter().enumerate() {
        let seg = leaf_segment(lw).map_err(|e| format!("leaf {j} segment: {e:?}"))?;
        let mut rng = ChaCha20Rng::from_entropy();
        let t = Instant::now();
        let proof = prove::<LeafRelation, Pos>(&pp, &pk, &rel, &seg.fields.to_vec(), lw.clone(), &mut rng)
            .map_err(|e| format!("prove leaf {j}: {e:?}"))?;
        let t_prove = t.elapsed().as_secs_f64();
        let t = Instant::now();
        let ok_std = verify::<LeafRelation, Pos>(&vp, &vk, &seg.fields.to_vec(), None, &proof).is_ok();
        let ok_acc = verify_offcircuit(&vp, &vk, &name, &seg.fields, &proof, false).is_ok();
        let t_verify = t.elapsed().as_secs_f64();
        if !(ok_std && ok_acc) {
            return Err(format!("leaf {j} proof does not verify (stdlib {ok_std}, accumulator {ok_acc})").into());
        }
        let mut bad = seg.fields.to_vec();
        bad[1] += F::ONE;
        let neg = verify::<LeafRelation, Pos>(&vp, &vk, &bad, None, &proof).is_ok();
        fs::write(out.join(format!("leaf-{j}.proof")), &proof)?;
        write_json(&out.join(format!("leaf-{j}.segment.json")), &fields_json(&seg.fields));
        let s = json!({"leaf": j, "prove_s": t_prove, "verify_s": t_verify, "proof_bytes": proof.len(),
            "proof_sha256": sha256_hex(&proof), "verifies_stdlib": ok_std, "verifies_accumulator_pairing": ok_acc,
            "pre_field_plus_one": if neg { "ACCEPTED" } else { "rejected" }});
        eprintln!("[leaf] {s}");
        stats.push(s);
    }
    write_json(&out.join("leaves.stats.json"), &json!({"kind": "leaf-prove", "n_tx": rel.n_tx, "k": k, "keygen_pk_s": t_pk,
        "leaves": stats, "vk_sha256": sha256_hex(&vk_bytes(&vk)), "transcript": "Poseidon (PoseidonState<Fq>), full-width challenges",
        "srs": srs, "peak_rss_kb": peak_rss_kb(), "threads": threads(), "batch_dir": batch_dir}));
    println!("proved {} leaves at k={k}", leaves.len());
    Ok(())
}

fn read_leaves(leaves_dir: &Path, n: usize) -> (Vec<Vec<F>>, Vec<Vec<u8>>) {
    // cost-node needs only the shape; absent leaves give placeholders.
    if !leaves_dir.join("leaf-0.proof").exists() {
        return (vec![vec![F::ZERO; 16]; n], vec![vec![]; n]);
    }
    let segs = (0..n).map(|j| read_fields(&leaves_dir.join(format!("leaf-{j}.segment.json")))).collect();
    let proofs = (0..n).map(|j| fs::read(leaves_dir.join(format!("leaf-{j}.proof"))).unwrap()).collect();
    (segs, proofs)
}

// ---------------------------------------------------------------- node

fn node_setup(batch_dir: &Path, leaf_key_dir: &Path, leaves_dir: &Path) -> (NodeRelation, NodeWitness) {
    let (lrel, leaf_vk) = load_leaf_key(leaf_key_dir);
    let (slots, b) = load_inputs(batch_dir).unwrap();
    let arity = slots.len() / lrel.n_tx as usize;
    let rel = NodeRelation { leaf_vk, arity: arity as u32, span: b.h_last - b.h_first };
    rel.validate().unwrap();
    let (leaf_segments, leaf_proofs) = read_leaves(leaves_dir, arity);
    let w = NodeWitness {
        leaf_segments,
        leaf_proofs,
        h_first: b.h_first,
        h_last: b.h_last,
        state_pre: b.state_pre.clone(),
        state_post: b.state_post.clone(),
    };
    (rel, w)
}

fn keygen_node(batch_dir: &Path, leaf_key_dir: &Path, leaves_dir: &Path, key_dir: &Path, k: Option<u32>) -> stage::Result<()> {
    fs::create_dir_all(key_dir)?;
    let (rel, _) = node_setup(batch_dir, leaf_key_dir, leaves_dir);
    let (cost, kk) = cost_json(&rel, k);
    eprintln!("[node keygen] {cost}");
    let (pp, srs) = params(kk);
    let t = Instant::now();
    let vk = setup_vk(&pp, &rel);
    let t_vk = t.elapsed().as_secs_f64();
    let vkb = vk_bytes(&vk);
    assert_eq!(vk_bytes(&vk_from_bytes(&vkb)?), vkb, "node vk round trip");
    let mut relb = Vec::new();
    rel.write_relation(&mut relb)?;
    let rel2 = NodeRelation::read_relation(&mut &relb[..])?;
    let mut relb2 = Vec::new();
    rel2.write_relation(&mut relb2)?;
    assert_eq!(relb, relb2, "node relation round trip");
    fs::write(key_dir.join("node.vk"), &vkb)?;
    fs::write(key_dir.join("node.relation"), &relb)?;
    write_json(&key_dir.join("keygen.stats.json"), &json!({"kind": "node-keygen", "arity": rel.arity, "cost": cost, "k": kk,
        "keygen_vk_s": t_vk, "vk_bytes": vkb.len(), "relation_bytes": relb.len(), "vk_sha256": sha256_hex(&vkb),
        "leaf_vk_sha256": sha256_hex(&vk_bytes(&rel.leaf_vk)), "srs": srs, "peak_rss_kb": peak_rss_kb(), "threads": threads()}));
    println!("node keygen arity={} k={kk} vk {} B in {t_vk:.1}s", rel.arity, vkb.len());
    Ok(())
}

fn prove_node(batch_dir: &Path, leaf_key_dir: &Path, leaves_dir: &Path, key_dir: &Path, w_dir: &Path) -> stage::Result<()> {
    fs::create_dir_all(w_dir)?;
    let t0 = Instant::now();
    let (rel, w) = node_setup(batch_dir, leaf_key_dir, leaves_dir);
    let relb = fs::read(key_dir.join("node.relation"))?;
    let krel = NodeRelation::read_relation(&mut &relb[..])?;
    assert_eq!(vk_bytes(&krel.leaf_vk), vk_bytes(&rel.leaf_vk), "node key pins another leaf key");
    let vkb = fs::read(key_dir.join("node.vk"))?;
    let vk = vk_from_bytes(&vkb)?;
    let inst = rel.instance(&w)?;
    assert_eq!(inst.len(), N_STATE + acc_pi_len());
    let vp = ledger_vp();
    // Natively, every child must pass its own pairing before a node proof is spent.
    for j in 0..rel.arity as usize {
        verify_offcircuit(&vp, &rel.leaf_vk, &rel.name(), &w.leaf_segments[j], &w.leaf_proofs[j], false)
            .map_err(|e| format!("leaf {j}: {e}"))?;
    }
    let k = vk.k() as u32;
    let (pp, srs) = params(k);
    let t = Instant::now();
    let pk = setup_pk(&krel, &vk);
    let t_pk = t.elapsed().as_secs_f64();
    eprintln!("[node] pk {t_pk:.1}s rss {} kB", peak_rss_kb());
    let mut rng = ChaCha20Rng::from_entropy();
    let t = Instant::now();
    let proof = prove::<NodeRelation, Pos>(&pp, &pk, &krel, &inst, w.clone(), &mut rng).map_err(|e| format!("prove node: {e:?}"))?;
    let t_prove = t.elapsed().as_secs_f64();
    let rss = peak_rss_kb();
    drop(pk);
    drop(pp);
    // Native check of N the way D will see it: prepare, fold the carried
    // accumulator (the last 12 instance fields), one pairing.
    let t = Instant::now();
    let w_name = straw_recl9::inner::vk_name("w", &vk);
    let ok = verify_offcircuit(&vp, &vk, &w_name, &inst, &proof, true).is_ok();
    let t_verify = t.elapsed().as_secs_f64();
    if !ok {
        return Err("node proof fails the native check (prepare, fold, pairing)".into());
    }
    let mut bad = inst.clone();
    bad[0] += F::ONE;
    let neg_pre = verify_offcircuit(&vp, &vk, &w_name, &bad, &proof, true).is_ok();
    let mut bad = inst.clone();
    let n = bad.len();
    bad[n - 3] += F::ONE;
    let neg_acc = verify_offcircuit(&vp, &vk, &w_name, &bad, &proof, true).is_ok();
    fs::write(w_dir.join("w.proof"), &proof)?;
    fs::write(w_dir.join("w.vk"), &vkb)?;
    write_json(&w_dir.join("w.instance.json"), &fields_json(&inst));
    fs::write(
        w_dir.join("SOURCE"),
        format!("recl9 node N (arity {}) over {} leaf proofs of {}; leaf key {}\n", rel.arity, rel.arity, batch_dir.display(), sha256_hex(&vk_bytes(&rel.leaf_vk))),
    )?;
    write_json(&w_dir.join("w.stats.json"), &json!({"kind": "node-prove", "arity": rel.arity, "k": k, "keygen_pk_s": t_pk,
        "prove_s": t_prove, "native_check_s": t_verify, "total_s": t0.elapsed().as_secs_f64(), "proof_bytes": proof.len(),
        "vk_bytes": vkb.len(), "proof_sha256": sha256_hex(&proof), "vk_sha256": sha256_hex(&vkb), "peak_rss_kb_after_prove": rss,
        "native_check": ok, "neg_pre_plus_one": if neg_pre { "ACCEPTED" } else { "rejected" },
        "neg_acc_field_plus_one": if neg_acc { "ACCEPTED" } else { "rejected" },
        "transcript": "Poseidon (PoseidonState<Fq>), full-width challenges", "srs": srs, "threads": threads()}));
    println!("node proof arity={} k={k}: pk {t_pk:.1}s prove {t_prove:.1}s proof {} B, native check ok", rel.arity, proof.len());
    Ok(())
}

// ---------------------------------------------------------------- decider

fn decider_rel(w_dir: &Path) -> DeciderRelation {
    let w_vk = vk_from_bytes(&fs::read(w_dir.join("w.vk")).unwrap()).unwrap();
    let w_inst = read_fields(&w_dir.join("w.instance.json"));
    assert_eq!(w_inst.len(), N_STATE + acc_pi_len(), "W instance is not the 29-field layout");
    let rel = DeciderRelation {
        nr_cols: 4,
        w_vk,
        n_w_instance: w_inst.len() as u32,
        s_g2: s_g2_checked(),
        dom_stmt: dom_stmt(),
        stmt_fields: vec![0, 1, 15, 16, 4, 5, 6, 8, 9, 10, 11, 12, 13],
        template: template().0,
        comm_binding: CommBinding::Free,
    };
    rel.validate().unwrap();
    rel
}

fn d_witness_with_opening(
    rel: &DeciderRelation,
    w_dir: &Path,
    supplied: Option<&str>,
) -> stage::Result<(DeciderWitness, Vec<F>)> {
    let w_inst = read_fields(&w_dir.join("w.instance.json"));
    let w_proof = fs::read(w_dir.join("w.proof")).unwrap();
    let (binding, comm, iid, bno) = free_inputs();
    let stmt = rel.stmt(&w_inst, iid, bno);
    Ok((
        DeciderWitness {
            w_instance: w_inst,
            w_proof,
            instance_id: iid,
            batch_no: bno,
            comm_rand: comm_rand_from(rel, supplied)?,
        },
        rel.template.public_inputs(binding, comm, stmt),
    ))
}

fn d_witness(rel: &DeciderRelation, w_dir: &Path) -> stage::Result<(DeciderWitness, Vec<F>)> {
    d_witness_with_opening(rel, w_dir, std::env::var("COMM_RAND_LE").ok().as_deref())
}

fn keygen_decider(w_dir: &Path, key_dir: &Path, k: Option<u32>) -> stage::Result<()> {
    fs::create_dir_all(key_dir)?;
    let rel = decider_rel(w_dir);
    let (cost, kk) = cost_json(&rel, k);
    eprintln!("[decider keygen] {cost}");
    let (pp, srs) = params(kk);
    let t = Instant::now();
    let vk = setup_vk(&pp, &rel);
    let t_vk = t.elapsed().as_secs_f64();
    let vkb = vk_bytes(&vk);
    assert_eq!(vk_bytes(&vk_from_bytes(&vkb)?), vkb, "decider vk round trip");
    let mut relb = Vec::new();
    rel.write_relation(&mut relb)?;
    let rel2 = DeciderRelation::read_relation(&mut &relb[..])?;
    let mut relb2 = Vec::new();
    rel2.write_relation(&mut relb2)?;
    assert_eq!(relb, relb2, "decider relation round trip");
    fs::write(key_dir.join("decider.vk"), &vkb)?;
    fs::write(key_dir.join("decider.relation"), &relb)?;
    let (_, tsrc) = template();
    write_json(&key_dir.join("keygen.stats.json"), &json!({"kind": "decider-keygen", "cost": cost, "k": kk, "keygen_vk_s": t_vk,
        "vk_bytes": vkb.len(), "relation_bytes": relb.len(), "vk_sha256": sha256_hex(&vkb), "relation_sha256": sha256_hex(&relb),
        "w_vk_sha256": sha256_hex(&vk_bytes(&rel.w_vk)), "template": tsrc, "srs": srs, "peak_rss_kb": peak_rss_kb(), "threads": threads()}));
    println!("decider keygen k={kk} vk {} B in {t_vk:.1}s", vkb.len());
    Ok(())
}

/// Per-call decider proof with the installed key (straw-decider prove argv
/// and env): read decider.relation and decider.vk, derive the pk, prove with
/// the Blake2b transcript, verify with the ledger's verifier parameters, run
/// the public-input negatives, write batch-K.{proof,public.json,stats.json}.
fn prove_call(w_dir: &Path, key_dir: &Path, out: &Path, batch: u32) -> stage::Result<()> {
    let t0 = Instant::now();
    fs::create_dir_all(out)?;
    let rel = DeciderRelation::read_relation(&mut &fs::read(key_dir.join("decider.relation"))?[..])?;
    let vkb = fs::read(key_dir.join("decider.vk"))?;
    let vk = vk_from_bytes(&vkb)?;
    let (wit, pis) = d_witness(&rel, w_dir)?;
    let wvk = fs::read(w_dir.join("w.vk"))?;
    if wvk != vk_bytes(&rel.w_vk) {
        return Err("w_dir's node key differs from the key pinned in the decider".into());
    }
    let vp = ledger_vp();
    verify_offcircuit(&vp, &rel.w_vk, &rel.w_name(), &wit.w_instance, &wit.w_proof, true)
        .map_err(|e| format!("node proof fails the native check: {e}"))?;
    let k = vk.k() as u32;
    let (pp, srs) = params(k);
    let t = Instant::now();
    let pk = setup_pk(&rel, &vk);
    let t_pk = t.elapsed().as_secs_f64();
    eprintln!("[decider] pk {t_pk:.1}s rss {} kB", peak_rss_kb());
    let mut rng = ChaCha20Rng::from_entropy();
    let t = Instant::now();
    let proof = prove::<DeciderRelation, Blake>(&pp, &pk, &rel, &pis, wit.clone(), &mut rng).map_err(|e| format!("prove D: {e:?}"))?;
    let t_prove = t.elapsed().as_secs_f64();
    let rss = peak_rss_kb();
    drop(pk);
    drop(pp);
    let t = Instant::now();
    let ok = verify::<DeciderRelation, Blake>(&vp, &vk, &pis, None, &proof).is_ok();
    let t_verify = t.elapsed().as_secs_f64();
    if !ok {
        return Err("D proof does not verify".into());
    }
    let mut neg = serde_json::Map::new();
    for (idx, slot) in rel.template.slots.iter().enumerate() {
        let mut bad = pis.clone();
        bad[idx] += F::ONE;
        let r = verify::<DeciderRelation, Blake>(&vp, &vk, &bad, None, &proof).is_ok();
        neg.insert(format!("slot{idx}:{slot:?}").chars().take(60).collect(), json!(if r { "ACCEPTED" } else { "rejected" }));
    }
    let mut short = pis.clone();
    short.pop();
    neg.insert("short_vector".into(), json!(if verify::<DeciderRelation, Blake>(&vp, &vk, &short, None, &proof).is_ok() { "ACCEPTED" } else { "rejected" }));
    let mut flipped = proof.clone();
    flipped[100] ^= 1;
    neg.insert("flipped_proof_byte".into(), json!(if verify::<DeciderRelation, Blake>(&vp, &vk, &pis, None, &flipped).is_ok() { "ACCEPTED" } else { "rejected" }));
    let other = rel.stmt(&wit.w_instance, wit.instance_id, wit.batch_no + F::ONE);
    let r = verify::<DeciderRelation, Blake>(&vp, &vk, &rel.template.public_inputs(pis[0], pis[1], other), None, &proof).is_ok();
    neg.insert("stmt_for_batch_no_plus_1".into(), json!(if r { "ACCEPTED" } else { "rejected" }));
    let stmt = pis[rel.template.stmt_index()];
    fs::write(out.join(format!("batch-{batch}.proof")), &proof)?;
    write_json(&out.join(format!("batch-{batch}.public.json")), &json!({
        "layout": "[binding_input, communication_commitment, transcript field encodings] (ledger-9 ContractCall::public_inputs)",
        "public_inputs": fields_json(&pis), "binding_input": f_to_hex(&pis[0]), "communication_commitment": f_to_hex(&pis[1]),
        "stmt": f_to_hex(&stmt), "stmt_slot": rel.template.stmt_index(), "instance_id": f_to_hex(&wit.instance_id),
        "batch_no": f_to_hex(&wit.batch_no), "w_instance": fields_json(&wit.w_instance),
        "encoding": "32-byte little-endian BLS12-381 scalar hex"}));
    write_json(&out.join(format!("batch-{batch}.stats.json")), &json!({
        "kind": "recl9-decider-prove-call", "k": k, "keygen_pk_s": t_pk, "prove_s": t_prove, "verify_s": t_verify,
        "total_s": t0.elapsed().as_secs_f64(), "proof_bytes": proof.len(), "vk_bytes": vkb.len(), "peak_rss_kb_after_prove": rss,
        "peak_rss_kb": peak_rss_kb(), "vk_sha256": sha256_hex(&vkb), "proof_sha256": sha256_hex(&proof),
        "w_vk_sha256": sha256_hex(&wvk), "transcript": "Blake2b (blake2b_simd::State)",
        "verify_params": "bls_midnight_2p14 verifier params (the ledger's embedded file)", "srs": srs, "threads": threads(),
        "negative_checks": neg}));
    println!("D prove-call k={k}: pk {t_pk:.1}s prove {t_prove:.1}s verify {t_verify:.3}s total {:.1}s proof {} B", t0.elapsed().as_secs_f64(), proof.len());
    Ok(())
}

fn verify_cmd(key_dir: &Path, out: &Path, kb: u32) -> stage::Result<()> {
    let vk = vk_from_bytes(&fs::read(key_dir.join("decider.vk"))?)?;
    let proof = fs::read(out.join(format!("batch-{kb}.proof")))?;
    let pj: serde_json::Value = serde_json::from_slice(&fs::read(out.join(format!("batch-{kb}.public.json")))?)?;
    let pis: Vec<F> = pj["public_inputs"].as_array().unwrap().iter().map(|s| f_from_hex(s.as_str().unwrap()).unwrap()).collect();
    let t = Instant::now();
    let ok = verify::<DeciderRelation, Blake>(&ledger_vp(), &vk, &pis, None, &proof).is_ok();
    println!("verify with ledger 2^14 verifier params, stock MidnightVK::read: {ok} in {:.3}s", t.elapsed().as_secs_f64());
    Ok(())
}

// ---------------------------------------------------------------- MockProver

fn print_mock(kind: &str, case: &str, note: &str, k: u32, verdict: (String, Vec<String>), t: Instant) {
    let v = json!({"kind": kind, "case": case, "note": note, "k": k, "verdict": verdict.0, "first_failures": verdict.1,
        "seconds": t.elapsed().as_secs_f64(), "peak_rss_kb": peak_rss_kb()});
    println!("{}", serde_json::to_string(&v).unwrap());
}

/// Leaf-level MockProver: leaf `j` of a staged batch, honest or mutated.
/// Public instance: the honest leaf segment (the prover's best attempt).
fn mock_leaf(batch_dir: &Path, n_tx: u32, j: usize, case: &str) -> stage::Result<()> {
    let rel = LeafRelation { n_tx };
    let bw = batch_witness(batch_dir);
    let honest = bw.leaves(n_tx as usize)[j].clone();
    let mut w = honest.clone();
    let spend = |w: &LeafWitness| w.slots.iter().position(|s| !s.tx.public.kind.is_deposit());
    let note = match case {
        "honest" => "honest leaf".to_string(),
        "link" => {
            // The L0/L1 link (opus-3 section 7 test 5 in this route): each of the
            // 16 wallet public fields must equal the transaction's. Changing the
            // wallet witness of a spend to another spend breaks all of them.
            let i = spend(&w).ok_or("no spend in leaf")?;
            let other = bw.slots.iter().enumerate().find(|(k, s)| !s.tx.public.kind.is_deposit() && s.tx.public.record_hash != w.slots[i].tx.public.record_hash).ok_or("no other spend")?.1.tx.clone();
            w.l0[i] = other;
            format!("slot {i}: wallet (L0) witness replaced by another honest spend")
        }
        "nk" => {
            let i = spend(&w).ok_or("no spend in leaf")?;
            w.l0[i].witness.spender_nk += F::ONE;
            format!("slot {i}: wallet nk + 1 (nk must equal H(DOM_NK, sk))")
        }
        "dep_acc_from" => {
            let d = w.slots.iter().position(|s| s.tx.public.kind.is_deposit()).ok_or("leaf has no deposit")?;
            for i in 0..=d {
                w.slots[i].tx.witness.dep_acc_before += F::ONE;
                if i < d {
                    w.slots[i].tx.witness.dep_acc_after += F::ONE;
                }
            }
            format!("wrong dep_acc_from with the right dep_acc_to (accumulators before deposit slot {d} shifted by one)")
        }
        "deposit_amount" => {
            let d = w.slots.iter().position(|s| s.tx.public.kind.is_deposit()).ok_or("leaf has no deposit")?;
            w.slots[d].tx.witness.deposit_record.as_mut().unwrap().amount += 1;
            format!("deposit slot {d}: record amount + 1")
        }
        "torsion_sig" => {
            let i = spend(&w).ok_or("no spend in leaf")?;
            w.slots[i].tx.public.signature.as_mut().unwrap().r_point = Point { x: F::ZERO, y: -F::ONE };
            format!("slot {i}: signature point R = order-2 point (0, -1)")
        }
        "swap" => {
            w.slots.swap(0, 1);
            w.l0.swap(0, 1);
            "slots 0 and 1 swapped".to_string()
        }
        "dup" => {
            w.slots[1] = w.slots[0].clone();
            w.l0[1] = w.l0[0].clone();
            "slot 1 replaced by a copy of slot 0".to_string()
        }
        "segment" => "honest witness, public segment field 10 (fee_sum) + 1".to_string(),
        _ => return Err(format!("unknown leaf case {case}").into()),
    };
    let mut pis = leaf_segment(&w).unwrap_or_else(|_| leaf_segment(&honest).unwrap()).fields.to_vec();
    if case == "segment" {
        pis[10] += F::ONE;
    }
    let k = cost_model(&rel, None).k;
    let t = Instant::now();
    let v = mock_verdict(&rel, pis, w, k);
    print_mock("leaf", case, &note, k, v, t);
    Ok(())
}

/// One history/invalid transaction case as a 1-tx leaf (the fused slot:
/// wallet rules, transaction rules, link), or the honest control.
fn mock_invalid(history: &Path, case: &str) -> stage::Result<()> {
    let (tx, reason) = if case == "honest" {
        let (_, _, all) = stage::replay(history)?;
        (all.iter().map(|(_, t)| t.clone()).find(|t| !t.public.kind.is_deposit()).unwrap(), "-".to_string())
    } else {
        let c: InvalidCase = serde_json::from_slice(&fs::read(history.join(format!("invalid/{case}.json")))?)?;
        (c.tx.clone().ok_or("not a transaction-level case")?, c.reason_code.clone())
    };
    let h = tx.public.height;
    let w = LeafWitness { slots: vec![TransactionWitness { tx: tx.clone(), pre_height: h, post_height: h }], l0: vec![tx] };
    let rel = LeafRelation { n_tx: 1 };
    let pis = straw_recl9::transition::segment(&w.slots[0]).fields.to_vec();
    let k = cost_model(&rel, None).k;
    let t = Instant::now();
    let v = mock_verdict(&rel, pis, w, k);
    print_mock("leaf-1tx", case, &format!("history/invalid reason {reason}"), k, v, t);
    Ok(())
}

/// Node-level MockProver over real leaf proofs: honest, tampered child
/// proofs, tampered child statements, and the wrapper rules.
fn mock_node(batch_dir: &Path, leaf_key_dir: &Path, leaves_dir: &Path, case: &str, k: Option<u32>) -> stage::Result<()> {
    let (rel, honest) = node_setup(batch_dir, leaf_key_dir, leaves_dir);
    let mut w = honest.clone();
    let mut inst_override: Option<Vec<F>> = None;
    let note = match case {
        "honest" => "honest node over real leaf proofs".to_string(),
        "child_proof_byte" => {
            w.leaf_proofs[1][200] ^= 1;
            "leaf 1 proof: one byte flipped".to_string()
        }
        "child_proof_swap" => {
            w.leaf_proofs.swap(0, 1);
            "leaf proofs 0 and 1 swapped (each verified against the other's segment)".to_string()
        }
        "child_statement" => {
            w.leaf_segments[2][10] += F::ONE;
            "leaf 2 segment fee_sum + 1 (proof unchanged)".to_string()
        }
        "child_order" => {
            w.leaf_segments.swap(1, 2);
            w.leaf_proofs.swap(1, 2);
            "leaves 1 and 2 swapped with their proofs (continuity)".to_string()
        }
        "child_dup" => {
            w.leaf_segments[3] = w.leaf_segments[2].clone();
            w.leaf_proofs[3] = w.leaf_proofs[2].clone();
            "leaf 3 replaced by a copy of leaf 2".to_string()
        }
        "h_first" => {
            w.h_first += 1;
            w.h_last += 1;
            "h_first and h_last + 1".to_string()
        }
        "post_state" => {
            w.state_post.fee_total += 1;
            "endpoint post state fee_total + 1".to_string()
        }
        "public_stmt" => {
            let mut i = rel.instance(&honest)?;
            i[4] += F::ONE;
            inst_override = Some(i);
            "public in_root + 1".to_string()
        }
        "public_acc" => {
            let mut i = rel.instance(&honest)?;
            let n = i.len();
            i[n - 2] += F::ONE;
            inst_override = Some(i);
            "public accumulator field + 1".to_string()
        }
        _ => return Err(format!("unknown node case {case}").into()),
    };
    // The instance a cheating prover would claim: from the mutated witness if
    // it composes natively, otherwise the honest one.
    let inst = match inst_override {
        Some(i) => i,
        None => {
            let mut wi = w.clone();
            // children whose accumulator cannot be prepared natively (flipped bytes)
            // keep the honest accumulator in the claim.
            match rel.instance(&wi) {
                Ok(i) => i,
                Err(_) => {
                    wi.leaf_proofs = honest.leaf_proofs.clone();
                    let st = rel.root_segment(&w).map(|r| NodeRelation::statement(&r, w.h_first, w.h_last));
                    let honest_inst = rel.instance(&honest)?;
                    let mut i = st.unwrap_or_else(|_| honest_inst[..N_STATE].to_vec());
                    i.extend_from_slice(&honest_inst[N_STATE..]);
                    i
                }
            }
        }
    };
    let kk = k.unwrap_or_else(|| cost_model(&rel, None).k);
    let t = Instant::now();
    let v = mock_verdict(&rel, inst, w, kk);
    print_mock("node", case, &note, kk, v, t);
    Ok(())
}

/// Decider-level MockProver over a real node proof.
fn mock_decider(w_dir: &Path, case: &str, k: Option<u32>) -> stage::Result<()> {
    let mut rel = decider_rel(w_dir);
    let bound_case = matches!(
        case,
        "opening_honest" | "opening_wrong_rand" | "opening_cc_plus_one" | "opening_missing_rand"
    );
    if bound_case {
        rel.comm_binding = CommBinding::BoundPoseidonStmtV1;
    }
    // No process-environment mutation: Rust 2024 makes set_var/remove_var
    // unsafe, and this binary does not establish the single-threaded
    // premise their Safety sections require. The opening is passed
    // explicitly; `opening_missing_rand` supplies None and is refused by
    // the same comm_rand_from check as the production bound path, before
    // any MockProver verdict. The later `opening_missing_rand` match arm
    // stays unreachable, as before.
    let mock_opening_hex = f_to_hex(&F::from(0x5eedu64));
    let (honest, hpis) = if case == "opening_missing_rand" {
        d_witness_with_opening(&rel, w_dir, None)?
    } else if bound_case {
        d_witness_with_opening(&rel, w_dir, Some(mock_opening_hex.as_str()))?
    } else {
        d_witness(&rel, w_dir)?
    };
    let mut w = honest.clone();
    let mut pis = hpis.clone();
    if bound_case {
        let stmt = rel.stmt(&w.w_instance, w.instance_id, w.batch_no);
        pis[1] = rel.comm_commitment_for(stmt, w.comm_rand);
    }
    let (binding, comm, _, _) = free_inputs();
    let note = match case {
        "honest" => "honest decider over the real node proof".to_string(),
        "stmt" => {
            pis[rel.template.stmt_index()] += F::ONE;
            "public stmt slot + 1".to_string()
        }
        "tpl" => {
            let i = rel.template.slots.iter().position(|s| matches!(s, Slot::Const(_))).unwrap();
            pis[i] += F::ONE;
            "first template constant slot + 1".to_string()
        }
        "batch_no" => {
            pis = rel.template.public_inputs(binding, comm, rel.stmt(&w.w_instance, w.instance_id, w.batch_no + F::ONE));
            "public stmt for batch_no + 1, witness batch_no unchanged".to_string()
        }
        "prev_root" => {
            let c: InvalidCase = serde_json::from_slice(&fs::read(root().join("../history/invalid/batch_wrong_previous_root.json"))?)?;
            let v = serde_json::to_value(&c)?;
            let claimed = f_from_hex(v["claimed_pre_digest"].as_str().unwrap())?;
            let mut wi = w.w_instance.clone();
            wi[0] = claimed;
            pis = rel.template.public_inputs(binding, comm, rel.stmt(&wi, w.instance_id, w.batch_no));
            "public stmt claims pre = batch_wrong_previous_root claimed digest; node instance unchanged".to_string()
        }
        "w_statement" => {
            w.w_instance[0] += F::ONE;
            pis = rel.template.public_inputs(binding, comm, rel.stmt(&w.w_instance, w.instance_id, w.batch_no));
            "node statement pre + 1 with a consistent public stmt (node proof unchanged)".to_string()
        }
        "w_acc" => {
            let n = w.w_instance.len();
            w.w_instance[n - 7] += F::ONE;
            "node carried accumulator: a coordinate field + 1".to_string()
        }
        "w_proof_byte" => {
            w.w_proof[300] ^= 1;
            "node proof: one byte flipped".to_string()
        }
        "opening_honest" => {
            "bound decider: public cc = Poseidon(comm_rand, stmt), honest private comm_rand".to_string()
        }
        "opening_wrong_rand" => {
            w.comm_rand += F::ONE;
            "bound decider: private comm_rand + 1, honest public cc".to_string()
        }
        "opening_cc_plus_one" => {
            pis[1] += F::ONE;
            "bound decider: public cc + 1, honest private comm_rand".to_string()
        }
        "opening_missing_rand" => {
            return Err(
                "unreachable: bound witness construction must refuse a missing COMM_RAND_LE".into(),
            );
        }
        _ => return Err(format!("unknown decider case {case}").into()),
    };
    let kk = k.unwrap_or_else(|| cost_model(&rel, None).k);
    let t = Instant::now();
    let v = mock_verdict(&rel, pis, w, kk);
    print_mock("decider", case, &note, kk, v, t);
    Ok(())
}

#[cfg(test)]
mod parent_cost_format_tests {
    use super::*;

    /// Arbitrary, distinctive model rows ensure formatting uses the supplied
    /// model instead of silently recomputing LeafRelation's circuit model.
    #[test]
    fn existing_cost_json_contract_formats_the_actual_cached_model() {
        let model = CircuitModel {
            k: 19, rows: 17, table_rows: 23, nb_unusable_rows: 7, max_deg: 3,
            advice_columns: 29, fixed_columns: 31, lookups: 37, permutations: 41,
            column_queries: 43, point_sets: 47, size: 53,
        };
        let rel = LeafRelation { n_tx: 1 };
        let (value, k) = cost_json_from_model(&rel, &model, Duration::from_millis(1250));
        assert_eq!(k, 19);
        for (name, expected) in [
            ("k", 19u64), ("rows", 17), ("table_rows", 23), ("unusable_rows", 7),
            ("usable_rows_at_k", (1u64 << 19) - 7), ("advice_columns", 29),
            ("fixed_columns", 31), ("lookups", 37), ("permutations", 41),
            ("proof_size_model_bytes", 53),
        ] {
            assert_eq!(value[name].as_u64(), Some(expected), "{name}");
        }
        assert_eq!(value["cost_model_seconds"].as_f64(), Some(1.25));
        assert_eq!(value["arch"], format!("{:?}", rel.used_chips()));
        assert!(value["peak_rss_kb"].as_u64().is_some());
        assert_eq!(value.as_object().unwrap().len(), 13);
    }
}
