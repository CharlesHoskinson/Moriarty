//! A11 (task 2.4): keygen regression at the current pins. Verifying keys only, no proofs.
//!
//! `a11_run_closure` keys the general-run closure over the M2-shape C6 digest leaf written by
//! `mori-backend` `a11_c6_digest_leaf_key_identity` (`$MORIARTY_A11_OUT/leaf/leaf.vk`): the IVC
//! node (SHKIVC06, k = 20), the run decider over the node key and the `acceptRun` template,
//! and the tagged `acceptRun` verifier the `SherlockRun` contract installs.
//! `a11_parent_closure` keys the stagenet closure: the A102 and A80 children, the two-child
//! parent and the terminal decider the stagenet contract installed
//! (`fixtures/stagenet-verifier.json`).
//! Each writes relation bytes, VK bytes and a JSON summary under `$MORIARTY_A11_OUT`; the
//! comparison with the committed manifests is `scripts/qa/a11-manifests.py`.
use std::{fs, io::Cursor, path::PathBuf, sync::Arc, time::Instant};

use group::Curve;
use midnight_curves::Bls12;
use midnight_proofs::{poly::kzg::params::ParamsKZG, utils::SerdeFormat};
use midnight_transient_crypto::proofs::VerifierKey;
use midnight_zk_stdlib::{Relation, cost_model, setup_vk};
use mori_recursion::{
    ivc::{LEAF_LANES, S_MAX, SherlockIvcRelation},
    parent::{
        LinkPins, ParentKeygenPlan, SherlockLinkRelation, keygen_sherlock_decider,
        sherlock_decider_relation,
    },
    run_decider::{CC_STMT, RunDeciderRelation, V_FIN, V_INIT, V_PROG, V_RESULT, V_STEPS},
    template::Template,
    util::{sha256_hex, vk_bytes, vk_from_bytes},
};
use serde_json::{Value, json};

const SRS_SHA256: &str = "1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1";
const CC_INPUTS: [u32; 6] = [CC_STMT, V_PROG, V_INIT, V_STEPS, V_FIN, V_RESULT];

fn env(name: &str) -> PathBuf {
    PathBuf::from(std::env::var_os(name).unwrap_or_else(|| panic!("{name} is required")))
}

fn srs() -> ParamsKZG<Bls12> {
    let bytes = fs::read(env("MORIARTY_PARENT_SRS")).unwrap();
    assert_eq!(sha256_hex(&bytes), SRS_SHA256, "official k20 ceremony SRS");
    let mut c = Cursor::new(bytes);
    ParamsKZG::read_custom(&mut c, SerdeFormat::RawBytesUnchecked).unwrap()
}

fn at_k(base: &ParamsKZG<Bls12>, k: u32) -> ParamsKZG<Bls12> {
    let mut p = base.clone();
    p.downsize(k);
    p
}

fn relation_bytes<R: Relation>(r: &R) -> Vec<u8> {
    let mut b = Vec::new();
    r.write_relation(&mut b).unwrap();
    b
}

fn write(dir: &PathBuf, name: &str, bytes: &[u8]) {
    fs::write(dir.join(name), bytes).unwrap();
}

fn entry(relation: &[u8], vk: &[u8], k: u32, rows: usize, seconds: f64) -> Value {
    json!({
        "relation_sha256": sha256_hex(relation), "relation_bytes": relation.len(),
        "vk_sha256": sha256_hex(vk), "vk_bytes": vk.len(), "k": k, "model_rows": rows,
        "keygen_seconds": seconds,
    })
}

#[test]
#[ignore = "real k20 keygen; MORIARTY_PARENT_SRS, MORIARTY_A11_OUT with leaf/leaf.vk; heavy.lock"]
fn a11_run_closure() {
    let out = env("MORIARTY_A11_OUT");
    let dir = out.join("run");
    fs::create_dir(&dir).expect("new run output directory");
    let base = srs();
    let leaf_raw = fs::read(out.join("leaf/leaf.vk")).unwrap();
    let leaf_vk = vk_from_bytes(&leaf_raw).unwrap();
    assert_eq!(vk_bytes(&leaf_vk), leaf_raw, "leaf key bytes round-trip");

    // Node: SHKIVC06 at k = 20 (task 2.3 / G-A).
    let rel = SherlockIvcRelation::new(4, 20, leaf_vk.clone()).unwrap();
    let m = cost_model(&rel, Some(20));
    assert_eq!(m.k, 20, "node SHKIVC06 stays at k=20");
    let node_rb = relation_bytes(&rel);
    assert_eq!(&node_rb[..8], b"SHKIVC06");
    let t = Instant::now();
    let node_vk = setup_vk(&at_k(&base, 20), &rel);
    let node_s = t.elapsed().as_secs_f64();
    assert_eq!(node_vk.k(), 20);
    let node_raw = vk_bytes(&node_vk);
    write(&dir, "node.relation", &node_rb);
    write(&dir, "node.vk", &node_raw);
    println!(
        "A11 node vk_sha256={} keygen_seconds={node_s:.3}",
        sha256_hex(&node_raw)
    );
    drop(rel);

    // Run decider over the node key and the acceptRun template.
    let template_raw = include_bytes!("fixtures/run-template.json");
    let drel = RunDeciderRelation {
        nr_cols: 4,
        node_vk: node_vk.clone(),
        s_g2: base.s_g2().to_affine(),
        template: Template::from_any_json(template_raw).unwrap(),
        cc_inputs: CC_INPUTS.to_vec(),
    };
    drel.validate().unwrap();
    let dm = cost_model(&drel, None);
    assert!(dm.k <= 20);
    let drb = relation_bytes(&drel);
    let t = Instant::now();
    let dvk = setup_vk(&at_k(&base, dm.k), &drel);
    let dec_s = t.elapsed().as_secs_f64();
    let dec_raw = vk_bytes(&dvk);
    let mut tagged = Vec::new();
    midnight_serialize::tagged_serialize(&VerifierKey::from(dvk.clone()), &mut tagged).unwrap();
    write(&dir, "decider.relation", &drb);
    write(&dir, "decider.vk", &dec_raw);
    write(&dir, "acceptRun.verifier", &tagged);
    // `midnight:verifier-key[vN]:` is the tagged-serialization prefix (SRC-008).
    let head = String::from_utf8_lossy(&tagged[..tagged.len().min(64)]).into_owned();
    let vk_header = head
        .match_indices(':')
        .nth(1)
        .map(|(i, _)| head[..=i].to_string())
        .expect("tagged verifier header");
    let summary = json!({
        "schema": "moriarty-a11-run-closure/1",
        "srs_sha256": SRS_SHA256,
        "leaf": {"vk_sha256": sha256_hex(&leaf_raw), "k": leaf_vk.k()},
        "node": entry(&node_rb, &node_raw, 20, m.rows, node_s),
        "node_magic": "SHKIVC06", "node_leaf_lanes": LEAF_LANES, "node_s_max": S_MAX,
        "decider": entry(&drb, &dec_raw, dm.k, dm.rows, dec_s),
        "contract": {
            "template_sha256": sha256_hex(template_raw),
            "template_source": "crates/mori-recursion/tests/fixtures/run-template.json",
            "acceptRun_verifier_sha256": sha256_hex(&tagged),
            "acceptRun_verifier_bytes": tagged.len(),
            "verifier_key_header": vk_header,
        },
    });
    write(
        &dir,
        "run.json",
        &serde_json::to_vec_pretty(&summary).unwrap(),
    );
    println!("A11 run {summary}");
}

#[test]
#[ignore = "real k20 keygen; MORIARTY_PARENT_SRS, MORIARTY_TEST_PARAMS_DIR, MORIARTY_A11_OUT; heavy.lock"]
fn a11_parent_closure() {
    use mori_backend::proving::{MAX_K, ProofOptions};
    use mori_backend::sherlock_child::ChildKey;
    use mori_backend::sherlock_link::{ReturnChildKey, produce_linked_requests};
    use mori_backend::sherlock_machine::literal_bridge::LiteralSourceIdentity;
    use mori_backend::sherlock_machine::literal_host::LiteralCapacities;
    use mori_backend::sherlock_machine::literal_witness::core_program_capacities;
    use mori_core::sherlock::layout_a::prepare_a;
    use mori_core::sherlock::{Expr, K_SOURCE_SHA256, Program, Value as SValue};

    let out = env("MORIARTY_A11_OUT");
    let dir = out.join("parent");
    fs::create_dir_all(&out).unwrap();
    fs::create_dir(&dir).expect("new parent output directory");
    let options = ProofOptions {
        params_dir: env("MORIARTY_TEST_PARAMS_DIR"),
        max_k: MAX_K,
    };
    // The A3 children, exactly as `sherlock_parent` reconstructs them.
    let capacities = LiteralCapacities {
        program: [2, 8, 2, 2, 2, 2, 1, 1, 1, 1, 4, 4, 5, 2, 2],
        state: [1; 6],
        stack_limit: 1,
    };
    let program = Arc::new(Program {
        functions: Vec::new(),
        entry: Arc::new(Expr::Literal(SValue::UInt(7))),
    });
    let prepared = prepare_a(
        program.clone(),
        core_program_capacities(&capacities).unwrap(),
    )
    .unwrap();
    let identity = LiteralSourceIdentity {
        k_source_sha256: K_SOURCE_SHA256.into(),
        reference_source_sha256: sha256_hex(include_bytes!(
            "../../mori-core/src/sherlock/reference.rs"
        )),
        program_source_sha256: sha256_hex(&prepared.program_bytes().unwrap()),
    };
    let (requests, shape) = produce_linked_requests(program, &capacities, 2, 1, &identity).unwrap();
    let t = Instant::now();
    let a102 = ChildKey::reconstruct(&requests.a102, &options).unwrap();
    let a80 = ReturnChildKey::reconstruct(&requests.a80, &options).unwrap();
    let child_s = t.elapsed().as_secs_f64();
    write(&dir, "a102.vk", a102.vk_bytes());
    write(&dir, "a80.vk", a80.vk_bytes());
    let pins = LinkPins {
        a102_vk_sha256: a102.vk_sha256(),
        a80_vk_sha256: a80.vk_sha256(),
        a102_context_hash: a102.claim().context_hash.clone(),
        a80_context_hash: a80.claim().context_hash.clone(),
        program_lanes: shape.program_lanes.try_into().unwrap(),
        state_lanes: shape.state_lanes.try_into().unwrap(),
    };
    let relation = SherlockLinkRelation::from_pins(
        vk_from_bytes(a102.vk_bytes()).unwrap(),
        vk_from_bytes(a80.vk_bytes()).unwrap(),
        &pins,
    )
    .unwrap();
    let a102_claim = serde_json::to_value(a102.claim()).unwrap();
    let a80_claim = serde_json::to_value(a80.claim()).unwrap();
    drop((a102, a80));

    let base = srs();
    let s_g2 = base.s_g2().to_affine();
    let plan = ParentKeygenPlan::new(&relation).unwrap();
    let (pk, prows) = (plan.model().k, plan.model().rows);
    assert_eq!(pk, 20, "A3 parent is a k20 relation");
    let t = Instant::now();
    let parent_vk = plan.keygen(&base, &s_g2, &dir.join("link")).unwrap();
    let parent_s = t.elapsed().as_secs_f64();
    let parent_rb = fs::read(dir.join("link/link.relation")).unwrap();
    let parent_raw = fs::read(dir.join("link/link.vk")).unwrap();
    drop(relation);

    // Terminal decider of the stagenet contract over the parent key.
    let template_raw = include_bytes!("fixtures/accept-template.json");
    let drel = sherlock_decider_relation(
        parent_vk,
        s_g2,
        Template::from_any_json(template_raw).unwrap(),
    )
    .unwrap();
    let dm = cost_model(&drel, None);
    let t = Instant::now();
    let dvk = keygen_sherlock_decider(&at_k(&base, dm.k), &drel, &dir.join("decider")).unwrap();
    let dec_s = t.elapsed().as_secs_f64();
    let drb = fs::read(dir.join("decider/decider.relation")).unwrap();
    let dec_raw = fs::read(dir.join("decider/decider.vk")).unwrap();
    assert_eq!(vk_bytes(&dvk), dec_raw);
    let summary = json!({
        "schema": "moriarty-a11-parent-closure/1",
        "srs_sha256": SRS_SHA256,
        "a102": {"vk_sha256": sha256_hex(&fs::read(dir.join("a102.vk")).unwrap()), "claim": a102_claim},
        "a80": {"vk_sha256": sha256_hex(&fs::read(dir.join("a80.vk")).unwrap()), "claim": a80_claim},
        "children_keygen_seconds": child_s,
        "parent": entry(&parent_rb, &parent_raw, pk, prows, parent_s),
        "terminal_decider": entry(&drb, &dec_raw, dm.k, dm.rows, dec_s),
        "template_sha256": sha256_hex(template_raw),
        "template_source": "crates/mori-recursion/tests/fixtures/accept-template.json",
    });
    write(
        &dir,
        "parent.json",
        &serde_json::to_vec_pretty(&summary).unwrap(),
    );
    println!("A11 parent {summary}");
}
