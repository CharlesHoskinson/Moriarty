//! E2 (design review 2026-11-10, C1c): node SHKIVC06 and run-decider rows against the
//! leaf verifier key. Modeling only: `cost_model` rows of the node over leaf keys keyed
//! from the pinned SRS (downsized), and of the run decider over each node key. No proof.
//!
//! What varies:
//! - sweep K: the C6 digest leaf relation (M2 shape, the A11 leaf) keyed at k = 13 and
//!   16..20; the relation is fixed, only the key degree changes (`setup_vk` keys at
//!   `params.max_k()`). Plus digest leaves filled with range-checked lanes so their own
//!   minimum k is 16..20 (check that a filled leaf and a padded key give the same rows).
//! - sweep A: at leaf k = 18, one `ZkStdLibArch` flag at a time over the ZKIR leaf arch.
//!
//! Every Sherlock C6 emitter uses only arithmetic, range, comparison and `TransientHash`
//! instructions, so any ZKIR universal leaf has the ZKIR arch {poseidon, 4 pow2range
//! columns} (`zkir/src/ir_vm.rs:1641-1668` at midnight-zkir 446c7fe).
use std::{fs, io::Cursor, path::PathBuf, sync::Arc};

use group::Curve;
use midnight_curves::Bls12;
use midnight_proofs::{poly::kzg::params::ParamsKZG, utils::SerdeFormat};
use midnight_transient_crypto::curve::Fr;
use midnight_zk_stdlib::{MidnightVK, Relation, ZkStdLib, ZkStdLibArch, cost_model, setup_vk};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_backend::sherlock_c6_child::{DigestSections, StateOutcome, c6_child_relation};
use mori_recursion::{
    ivc::SherlockIvcRelation,
    run_decider::{CC_STMT, RunDeciderRelation, V_FIN, V_INIT, V_PROG, V_RESULT, V_STEPS},
    standin::StandInLeaf,
    template::Template,
    util::{F, sha256_hex, vk_bytes},
};
use serde_json::{Value, json};

const SRS_SHA256: &str = "1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1";
const CC_INPUTS: [u32; 6] = [CC_STMT, V_PROG, V_INIT, V_STEPS, V_FIN, V_RESULT];

fn srs() -> ParamsKZG<Bls12> {
    let path = std::env::var_os("MORIARTY_PARENT_SRS").expect("MORIARTY_PARENT_SRS");
    let bytes = fs::read(PathBuf::from(path)).unwrap();
    assert_eq!(sha256_hex(&bytes), SRS_SHA256, "official k20 ceremony SRS");
    ParamsKZG::read_custom(&mut Cursor::new(bytes), SerdeFormat::RawBytesUnchecked).unwrap()
}

fn at_k(base: &ParamsKZG<Bls12>, k: u32) -> ParamsKZG<Bls12> {
    let mut p = base.clone();
    p.downsize(k);
    p
}

/// The C6 harness LocalPublic shape (`mori-backend/tests/sherlock_c6_child.rs:14`):
/// `lanes` public Native inputs, each range-checked to 128 bits, one publication.
fn local_public_ir(lanes: usize) -> IrSource {
    let mut code = Vec::new();
    let mut published = Vec::new();
    for i in 0..lanes {
        let id = Identifier(format!("%c6.public.{i}"));
        code.push(Instruction::PublicInput {
            guard: None,
            val_t: IrType::Native,
            output: id.clone(),
        });
        published.push(Operand::Variable(id));
    }
    for op in &published {
        code.push(Instruction::ConstrainBits {
            val: op.clone(),
            bits: 128,
        });
    }
    code.push(Instruction::Impact {
        guard: Operand::Immediate(Fr::from(1u64)),
        inputs: published,
    });
    let ir0 = IrSource {
        version: midnight_zkir_v3::ir::IrMinorVersion::V0,
        inputs: Vec::new(),
        outputs: Vec::new(),
        do_communications_commitment: false,
        instructions: Arc::new(code),
    };
    let mut disk = serde_json::to_value(&ir0).unwrap();
    disk["version"] = json!({"major": 3, "minor": 0});
    IrSource::load(serde_json::to_string(&disk).unwrap().as_bytes()).unwrap()
}

/// Digest leaf over `3 * third` statement lanes (prog / pre / post sections).
fn digest_leaf(third: usize) -> IrSource {
    let sections = DigestSections {
        prog_lanes: third,
        pre_lanes: third,
        post_lanes: third,
    };
    c6_child_relation(
        &local_public_ir(3 * third),
        sections,
        StateOutcome::RUNNING,
        StateOutcome::RUNNING,
    )
    .unwrap()
}

/// The stand-in leaf body under an explicit arch (sweep A).
#[derive(Clone, Debug)]
struct ArchLeaf(ZkStdLibArch);

impl Relation for ArchLeaf {
    type Instance = Vec<F>;
    type Witness = Vec<F>;
    type Error = midnight_proofs::plonk::Error;
    fn format_instance(i: &Self::Instance) -> Result<Vec<F>, Self::Error> {
        Ok(i.clone())
    }
    fn circuit(
        &self,
        std: &ZkStdLib,
        l: &mut impl midnight_proofs::circuit::Layouter<F>,
        i: midnight_proofs::circuit::Value<Self::Instance>,
        w: midnight_proofs::circuit::Value<Self::Witness>,
    ) -> Result<(), Self::Error> {
        StandInLeaf.circuit(std, l, i, w)
    }
    fn used_chips(&self) -> ZkStdLibArch {
        self.0
    }
    fn write_relation<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        w.write_all(format!("E2ARCH{:?}", self.0).as_bytes())
    }
    fn read_relation<R: std::io::Read>(_r: &mut R) -> std::io::Result<Self> {
        Err(std::io::Error::other("probe relation"))
    }
}

fn zkir_arch() -> ZkStdLibArch {
    ZkStdLibArch {
        poseidon: true,
        nr_pow2range_cols: 4,
        ..ZkStdLibArch::default()
    }
}

struct Ctx {
    base: ParamsKZG<Bls12>,
    template: Template,
    decider_cache: Vec<(String, Value)>,
}

impl Ctx {
    /// Node rows over `leaf_vk`; decider rows over the node key when the node fits k=20.
    fn node_and_decider(&mut self, leaf_vk: &MidnightVK, with_decider: bool) -> Value {
        let rel = SherlockIvcRelation::new(4, 20, leaf_vk.clone()).unwrap();
        let m = cost_model(&rel, Some(20));
        let mut out = json!({
            "leaf_k": leaf_vk.k(), "leaf_vk_bytes": vk_bytes(leaf_vk).len(),
            "node_rows": m.rows, "node_model_k": m.k,
            "node_over_2p19": m.rows as i64 - (1 << 19),
            "node_headroom_to_2p20": (1i64 << 20) - m.rows as i64,
        });
        if with_decider && m.k <= 20 {
            let node_vk = setup_vk(&at_k(&self.base, 20), &rel);
            let node_sha = sha256_hex(&vk_bytes(&node_vk));
            drop(rel);
            let drel = RunDeciderRelation {
                nr_cols: 4,
                node_vk,
                s_g2: self.base.s_g2().to_affine(),
                template: self.template.clone(),
                cc_inputs: CC_INPUTS.to_vec(),
            };
            drel.validate().unwrap();
            let dm = cost_model(&drel, None);
            out["node_vk_sha256"] = json!(node_sha);
            out["decider_rows"] = json!(dm.rows);
            out["decider_model_k"] = json!(dm.k);
            out["decider_headroom_to_2p20"] = json!((1i64 << 20) - dm.rows as i64);
            self.decider_cache.push((node_sha, out.clone()));
        }
        println!("E2ROW {out}");
        out
    }
}

#[test]
#[ignore = "E2 modeling probe; MORIARTY_PARENT_SRS, MORIARTY_E2_OUT; node keygen at k=20 for the decider"]
fn e2_node_decider_headroom() {
    let out_path = PathBuf::from(std::env::var_os("MORIARTY_E2_OUT").expect("MORIARTY_E2_OUT"));
    let mut ctx = Ctx {
        base: srs(),
        template: Template::from_any_json(include_bytes!("fixtures/run-template.json")).unwrap(),
        decider_cache: Vec::new(),
    };
    let mut rows = Vec::new();

    // Calibration 1: stand-in leaf at its own k (G-A: 652,437 node rows).
    let lk = cost_model(&StandInLeaf, None).k;
    let vk = setup_vk(&at_k(&ctx.base, lk), &StandInLeaf);
    let mut r = ctx.node_and_decider(&vk, false);
    r["series"] = json!("calibration-standin");
    rows.push(r);

    // Calibration 2 and sweep K (padded key): the A11 digest leaf, 96 lanes.
    let a11 = digest_leaf(32);
    let a11_min_k = cost_model(&a11, None).k;
    for k in [a11_min_k, 16, 17, 18, 19, 20] {
        let vk = setup_vk(&at_k(&ctx.base, k), &a11);
        let mut r = ctx.node_and_decider(&vk, true);
        r["series"] = json!("K-padded-a11-digest-leaf");
        r["leaf_min_k"] = json!(a11_min_k);
        rows.push(r);
    }

    // Sweep K (filled): digest leaves whose own minimum k is 16..20.
    let mut third = 64usize;
    for k in 16u32..=20 {
        // Grow the lane count until the relation needs k on its own.
        loop {
            let m = cost_model(&digest_leaf(third), None);
            if m.k >= k {
                break;
            }
            third = third * 5 / 4 + 1;
        }
        let leaf = digest_leaf(third);
        let lm = cost_model(&leaf, None);
        let vk = setup_vk(&at_k(&ctx.base, lm.k), &leaf);
        let mut r = ctx.node_and_decider(&vk, false);
        r["series"] = json!("K-filled-digest-leaf");
        r["leaf_lanes"] = json!(3 * third);
        r["leaf_rows"] = json!(lm.rows);
        r["leaf_min_k"] = json!(lm.k);
        rows.push(r);
    }

    // Sweep A: one flag at a time over the ZKIR arch, leaf keyed at k = 18.
    let flags: Vec<(&str, fn(&mut ZkStdLibArch))> = vec![
        ("zkir", |_| {}),
        ("zkir-pow2range-1", |a| a.nr_pow2range_cols = 1),
        ("zkir+sha2_256", |a| a.sha2_256 = true),
        ("zkir+sha2_512", |a| a.sha2_512 = true),
        ("zkir+keccak_256", |a| a.keccak_256 = true),
        ("zkir+blake2b", |a| a.blake2b = true),
        ("zkir+jubjub", |a| a.jubjub = true),
        ("zkir+secp256k1", |a| a.secp256k1 = true),
        ("zkir+bls12_381", |a| a.bls12_381 = true),
        ("zkir+base64", |a| a.base64 = true),
        ("zkir+automaton", |a| a.automaton = true),
        ("zkir+jubjub+sha2_256", |a| {
            a.jubjub = true;
            a.sha2_256 = true
        }),
    ];
    for (name, f) in flags {
        let mut arch = zkir_arch();
        f(&mut arch);
        let leaf = ArchLeaf(arch);
        let p18 = at_k(&ctx.base, 18);
        let keyed =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| setup_vk(&p18, &leaf)));
        let Ok(vk) = keyed else {
            rows.push(json!({"series": "A-arch-at-k18", "arch": name, "error": "leaf keygen at k=18 failed"}));
            continue;
        };
        let mut r = ctx.node_and_decider(&vk, false);
        r["series"] = json!("A-arch-at-k18");
        r["arch"] = json!(name);
        rows.push(r);
    }

    let summary = json!({
        "schema": "moriarty-e2-leaf-headroom/1",
        "srs_sha256": SRS_SHA256,
        "node": "SherlockIvcRelation::new(4, 20, leaf_vk), cost_model(Some(20))",
        "decider": "RunDeciderRelation nr_cols 4 over setup_vk(node, k=20), cost_model(None)",
        "rows": rows,
    });
    fs::create_dir_all(&out_path).unwrap();
    fs::write(
        out_path.join("e2.json"),
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .unwrap();
}
