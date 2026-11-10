//! E3 (chiplet study): rows and k of the spike leaf at S = 1, 4, 16; MockProver at
//! S = 16; then the SHKIVC06 node and the run decider over the spike leaf's key.
//!
//! The node takes its leaf key as a `MidnightVK`, which only a stdlib relation can
//! have (`MidnightVK::read` rebuilds the constraint system from `ZkStdLib`). So the
//! node is mirrored here (`ChipNode`): the SHKIVC06 circuit of
//! `mori-recursion/src/ivc.rs` with the leaf verified under a raw `VerifyingKey`.
//! `chip_node_mirror_calibrates` checks the mirror against the real node on a stdlib
//! leaf: equal `cost_model` rows and k.
//!
//! Modeling only: `cost_model`, keys over an unsafe SRS (rows do not depend on the
//! SRS), or the official k20 SRS when `MORIARTY_PARENT_SRS` is set. No proof.
use std::{collections::BTreeMap, fs, io, io::Cursor, path::PathBuf, time::Instant};

use group::{Curve, Group};
use midnight_circuits::{
    instructions::{
        AssertionInstructions, AssignmentInstructions, BinaryInstructions, ControlFlowInstructions,
        PublicInputInstructions, ZeroInstructions,
    },
    types::AssignedNative,
    verifier::{AssignedAccumulator, AssignedVk, fixed_bases},
};
use midnight_curves::Bls12;
use midnight_proofs::{
    circuit::{Layouter, Value},
    dev::{MockProver, cost_model::circuit_model},
    plonk::{ConstraintSystem, Error, VerifyingKey, keygen_vk_with_k},
    poly::{
        EvaluationDomain,
        kzg::{KZGCommitmentScheme, params::ParamsKZG},
    },
    utils::SerdeFormat,
};
use midnight_zk_stdlib::{Relation, ZkStdLib, ZkStdLibArch, cost_model, setup_vk};
use mori_chip::{
    F,
    leaf::{Leaf, LeafWitness},
    rf::{genesis, run32},
    rfcore::{Arch, RfLeaf, RfParams, RfWitness},
    trace::{Regs, run},
};
use mori_recursion::{
    inner::C,
    ivc::{
        IvcWitness, SELF_NAME, SherlockIvcRelation, constrain_collapsed_carry,
        constrain_genesis_neutral, constrain_heights, node_cs,
    },
    run_decider::{CC_STMT, RunDeciderRelation, V_FIN, V_INIT, V_PROG, V_RESULT, V_STEPS},
    standin::StandInLeaf,
    template::Template,
    util::sha256_hex,
};
use rand_chacha::{ChaCha20Rng, rand_core::SeedableRng};
use serde_json::{Value as J, json};

type Vk = VerifyingKey<F, KZGCommitmentScheme<Bls12>>;
const RC: usize = 4;
const CC_INPUTS: [u32; 6] = [CC_STMT, V_PROG, V_INIT, V_STEPS, V_FIN, V_RESULT];
const SRS_SHA256: &str = "1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1";

fn emit(name: &str, v: &J) {
    println!("CHIP-E3 {}", json!({"name": name, "record": v}));
    if let Some(dir) = std::env::var_os("CHIP_OUT") {
        let dir = PathBuf::from(dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join(format!("{name}.json")),
            serde_json::to_vec_pretty(v).unwrap(),
        )
        .unwrap();
    }
}

fn bls_arch(nr_cols: u8) -> ZkStdLibArch {
    ZkStdLibArch {
        bls12_381: true,
        poseidon: true,
        nr_pow2range_cols: nr_cols,
        ..ZkStdLibArch::default()
    }
}

/// Parameters at `k`: the official SRS downsized when `MORIARTY_PARENT_SRS` is set,
/// else an unsafe deterministic SRS.
fn params(k: u32) -> ParamsKZG<Bls12> {
    if let Some(p) = std::env::var_os("MORIARTY_PARENT_SRS") {
        let bytes = fs::read(PathBuf::from(p)).unwrap();
        assert_eq!(sha256_hex(&bytes), SRS_SHA256, "official k20 ceremony SRS");
        let mut base =
            ParamsKZG::read_custom(&mut Cursor::new(bytes), SerdeFormat::RawBytesUnchecked)
                .unwrap();
        base.downsize(k);
        base
    } else {
        ParamsKZG::unsafe_setup(k, ChaCha20Rng::seed_from_u64(7))
    }
}

/// The SHKIVC06 node circuit with the leaf verified under a raw key.
#[derive(Clone, Debug)]
struct ChipNode {
    /// The real node over any leaf: supplies the self-key helpers, which do not
    /// depend on the leaf.
    base: SherlockIvcRelation,
    leaf: Vk,
    self_domain: EvaluationDomain<F>,
    self_cs: ConstraintSystem<F>,
}

impl ChipNode {
    fn new(base: SherlockIvcRelation, leaf: Vk) -> Self {
        let self_cs = node_cs(base.nr_cols, base.self_k);
        let self_domain = EvaluationDomain::new(self_cs.degree() as u32, base.self_k);
        Self {
            base,
            leaf,
            self_domain,
            self_cs,
        }
    }

    fn leaf_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        self.leaf.write(&mut b, SerdeFormat::Processed).unwrap();
        b
    }

    fn leaf_name(&self) -> String {
        format!("shk_leaf_{}", &sha256_hex(&self.leaf_bytes())[..16])
    }
}

impl Relation for ChipNode {
    type Instance = Vec<F>;
    type Witness = IvcWitness;
    type Error = Error;

    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }

    // Steps 1-5 of `SherlockIvcRelation::circuit` (ivc.rs), unchanged except step 4.
    fn circuit(
        &self,
        std: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _instance: Value<Vec<F>>,
        witness: Value<IvcWitness>,
    ) -> Result<(), Error> {
        let verifier = std.verifier();
        let bls = std.bls12_381();
        let scalar = bls.scalar_field_chip();
        let self_vk: AssignedVk<mori_recursion::inner::S> = verifier.assign_vk_as_public_input(
            l,
            SELF_NAME,
            &self.self_domain,
            &self.self_cs,
            witness.as_ref().map(|w| w.self_repr),
        )?;
        let wf = |f: fn(&IvcWitness) -> F| witness.as_ref().map(f);
        let prog_d: AssignedNative<F> = std.assign(l, wf(|w| w.prev.prog_d))?;
        let init_d: AssignedNative<F> = std.assign(l, wf(|w| w.prev.init_d))?;
        let prev_d: AssignedNative<F> = std.assign(l, wf(|w| w.prev.cur_d))?;
        let prev_h: AssignedNative<F> = std.assign(l, wf(|w| F::from(w.prev.h)))?;
        let cur_d: AssignedNative<F> = std.assign(l, wf(|w| w.cur_d))?;
        let rule: AssignedNative<F> = std.assign(l, wf(|w| w.rule))?;
        let binding: AssignedNative<F> = std.assign(l, wf(|w| w.leaf_binding))?;
        let steps: AssignedNative<F> = std.assign(l, wf(|w| F::from(w.leaf_steps)))?;
        let h = constrain_heights(std, l, &prev_h, &steps)?;
        let is_genesis = std.is_zero(l, &prev_h)?;
        let not_genesis = std.not(l, &is_genesis)?;
        let chosen = std.select(l, &is_genesis, &init_d, &prev_d)?;
        std.assert_equal(l, &chosen, &prev_d)?;
        for x in [&prog_d, &init_d, &cur_d, &h] {
            std.constrain_as_public_input(l, x)?;
        }
        let prev_acc = AssignedAccumulator::<mori_recursion::inner::S>::assign(
            l,
            bls,
            scalar,
            1,
            1,
            &[],
            &self.base.self_base_names(),
            witness.as_ref().map(|w| w.prev_acc.clone()),
        )?;
        let prev_acc_pi = verifier.as_public_input(l, &prev_acc)?;
        let neutral = <AssignedAccumulator<mori_recursion::inner::S> as midnight_circuits::types::Instantiable<F>>::as_public_input(&self.base.trivial_acc());
        constrain_genesis_neutral(std, l, &is_genesis, &prev_acc_pi, &neutral)?;
        constrain_collapsed_carry(std, l, &prev_acc_pi, self.base.carry_scalar_lanes())?;
        let mut prev_pi = verifier.as_public_input(l, &self_vk)?;
        prev_pi.extend([
            prog_d.clone(),
            init_d.clone(),
            prev_d.clone(),
            prev_h.clone(),
        ]);
        prev_pi.extend(prev_acc_pi);
        let id = bls.assign_fixed(l, C::identity())?;
        let mut proof_acc = verifier.prepare(
            l,
            &self_vk,
            &[id],
            &[prev_pi.as_slice()],
            witness.as_ref().map(|w| w.prev_proof.clone()),
        )?;
        AssignedAccumulator::scale_by_bit(l, scalar, &not_genesis, &mut proof_acc)?;
        proof_acc.collapse(l, bls, scalar)?;

        // 4. The leaf under its fixed raw key (`verify_child_incircuit_with`, carried
        // = false, collapse = false, with `vk.vk()` replaced by the raw key).
        let leaf_inst = vec![binding, prog_d, prev_d, cur_d, rule, prev_h, steps];
        let name = self.leaf_name();
        let pvk = &self.leaf;
        let assigned_vk = verifier.assign_fixed_vk(
            l,
            &name,
            pvk.get_domain(),
            pvk.cs(),
            pvk.transcript_repr(),
        )?;
        let mut assigned_bases = BTreeMap::new();
        for (n, base) in fixed_bases::<mori_recursion::inner::S>(&name, pvk) {
            assigned_bases.insert(n, bls.assign_fixed(l, base)?);
        }
        let committed = [bls.assign_fixed(l, C::identity())?];
        let mut leaf_acc = verifier.prepare(
            l,
            &assigned_vk,
            &committed,
            &[leaf_inst.as_slice()],
            witness.as_ref().map(|w| w.leaf_proof.clone()),
        )?;
        leaf_acc.resolve_fixed_bases(&assigned_bases);

        let mut next = verifier.accumulate(l, &[proof_acc, prev_acc, leaf_acc])?;
        next.collapse(l, bls, scalar)?;
        verifier.constrain_as_public_input(l, &next)
    }

    fn used_chips(&self) -> ZkStdLibArch {
        bls_arch(self.base.nr_cols)
    }

    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"CHIPNODE")?;
        w.write_all(&[self.base.nr_cols])?;
        w.write_all(&self.base.self_k.to_le_bytes())?;
        w.write_all(&self.leaf_bytes())
    }

    fn read_relation<R: io::Read>(_r: &mut R) -> io::Result<Self> {
        Err(io::Error::other(
            "ChipNode is a measurement mirror; not readable",
        ))
    }
}

fn standin_vk(k: u32) -> midnight_zk_stdlib::MidnightVK {
    setup_vk(&params(k), &StandInLeaf)
}

fn model_json(m: &midnight_proofs::dev::cost_model::CircuitModel) -> J {
    json!({
        "k": m.k, "rows": m.rows, "table_rows": m.table_rows, "max_deg": m.max_deg,
        "advice_columns": m.advice_columns, "fixed_columns": m.fixed_columns,
        "lookups": m.lookups, "permutations": m.permutations,
        "column_queries": m.column_queries, "point_sets": m.point_sets,
        "proof_size": m.size,
    })
}

fn leaf_model(s: usize) -> J {
    let t = Instant::now();
    let m = circuit_model::<F, 48, 32>(&Leaf::<RC>::new(s, None));
    let mut v = model_json(&m);
    v["region_rows"] = json!(Leaf::<RC>::region_rows(s));
    v["seconds"] = json!(t.elapsed().as_secs_f64());
    v
}

fn leaf_vk(s: usize) -> (Vk, u32) {
    let c = Leaf::<RC>::new(s, None);
    let k = circuit_model::<F, 48, 32>(&c).k;
    (keygen_vk_with_k(&params(k), &c, k).unwrap(), k)
}

#[test]
fn spike_leaf_s16_mock_prover() {
    let s = 16;
    let w = LeafWitness::from_trace(&run(Regs::genesis(), s), s);
    assert_eq!(w.steps(), 16);
    let c = Leaf::<RC>::new(s, Some(w.clone()));
    let p = MockProver::run(&c, vec![vec![], w.instance()]).unwrap();
    p.assert_satisfied();
}

/// Query rotations of the keyed leaf (`vk.cs()`), as the node's verifier gadget
/// sees them: it supports -1, 0 and +1 only (`verifier_gadget.rs:604-610`).
#[test]
fn spike_leaf_vk_rotations_are_unit() {
    let (vk, k) = leaf_vk(1);
    let cs = vk.cs();
    let mut bad = Vec::new();
    for (kind, qs) in [
        (
            "advice",
            cs.advice_queries()
                .iter()
                .map(|q| (q.0.index(), q.1.0))
                .collect::<Vec<_>>(),
        ),
        (
            "fixed",
            cs.fixed_queries()
                .iter()
                .map(|q| (q.0.index(), q.1.0))
                .collect(),
        ),
        (
            "instance",
            cs.instance_queries()
                .iter()
                .map(|q| (q.0.index(), q.1.0))
                .collect(),
        ),
    ] {
        println!("CHIP-ROT k={k} {kind}: {qs:?}");
        bad.extend(
            qs.into_iter()
                .filter(|q| !(-1..=1).contains(&q.1))
                .map(|q| (kind, q)),
        );
    }
    assert!(bad.is_empty(), "rotations outside -1..=1: {bad:?}");
}

#[test]
#[ignore = "E3 rows: cost_model of the spike leaf at S = 1, 4, 16 (seconds)"]
fn chip_e3_leaf_rows() {
    let mut out = BTreeMap::new();
    for s in [1usize, 4, 16, 64] {
        out.insert(format!("S{s}"), leaf_model(s));
    }
    emit("e3_leaf_rows", &json!(out));
}

#[test]
#[ignore = "E3 node calibration: cost_model of the real SHKIVC06 node and of ChipNode over one stdlib leaf key; k20 models"]
fn chip_node_mirror_calibrates() {
    let leaf = standin_vk(13);
    let real = SherlockIvcRelation::new(4, 20, leaf.clone()).unwrap();
    let t = Instant::now();
    let rm = cost_model(&real, Some(20));
    let mirror = ChipNode::new(real.clone(), leaf.vk().clone());
    let mm = cost_model(&mirror, Some(20));
    let v = json!({
        "leaf": "mori_recursion::standin::StandInLeaf keyed at k = 13",
        "real_node": model_json(&rm),
        "mirror_node": model_json(&mm),
        "seconds": t.elapsed().as_secs_f64(),
    });
    emit("e3_node_calibration", &v);
    assert_eq!((rm.k, rm.rows), (mm.k, mm.rows), "mirror equals SHKIVC06");
}

#[test]
#[ignore = "E3 node and decider over the spike leaf key: two k20 cost models, one k20 node keygen, decider cost model"]
fn chip_e3_node_and_decider() {
    let standin = standin_vk(13);
    let base = SherlockIvcRelation::new(4, 20, standin).unwrap();
    let mut out = BTreeMap::new();
    let mut node16 = None;
    for s in [1usize, 16] {
        let (vk, k) = leaf_vk(s);
        let node = ChipNode::new(base.clone(), vk.clone());
        let t = Instant::now();
        let m = cost_model(&node, Some(20));
        let cs = vk.cs();
        out.insert(
            format!("S{s}"),
            json!({
                "leaf_k": k,
                "leaf_cs": {
                    "advice": cs.num_advice_columns(), "fixed": cs.num_fixed_columns(),
                    "instance": cs.num_instance_columns(), "selectors": cs.num_selectors(),
                    "lookups": cs.lookups().len(), "permutation_columns": cs.permutation().get_columns().len(),
                    "degree": cs.degree(),
                },
                "node": model_json(&m),
                "node_seconds": t.elapsed().as_secs_f64(),
            }),
        );
        if s == 16 {
            node16 = Some(node);
        }
    }
    // The node rows first, so a cut decider run keeps them.
    emit("e3_node", &json!(out));
    // Run decider over the node keyed on the S = 16 spike leaf.
    let node = node16.unwrap();
    let base_params = params(20);
    let t = Instant::now();
    let node_vk = setup_vk(&base_params, &node);
    let keygen_s = t.elapsed().as_secs_f64();
    let drel = RunDeciderRelation {
        nr_cols: 4,
        node_vk,
        s_g2: base_params.s_g2().to_affine(),
        template: Template::from_any_json(include_bytes!(
            "../../mori-recursion/tests/fixtures/run-template.json"
        ))
        .unwrap(),
        cc_inputs: CC_INPUTS.to_vec(),
    };
    drel.validate().unwrap();
    let dm = cost_model(&drel, None);
    out.insert(
        "decider".into(),
        json!({"over": "ChipNode keyed on the S = 16 spike leaf", "node_keygen_seconds": keygen_s, "model": model_json(&dm),
               "srs": if std::env::var_os("MORIARTY_PARENT_SRS").is_some() { "official k20" } else { "unsafe" }}),
    );
    emit("e3_node_decider", &json!(out));
    let _ = C::generator();
}

// ---------------------------------------------------------------------------
// E5 (register file) and E6 (HASH): `research/sherlock-processor-analogy-20261110/experiments-2.md`.

const E5_ARCHS: [(&str, Arch); 4] = [
    ("wide", Arch::Wide),
    ("wide_inline", Arch::WideInline),
    ("narrow", Arch::Narrow),
    ("staggered", Arch::Staggered),
];

fn arch_of(name: &str) -> Arch {
    E5_ARCHS
        .iter()
        .find(|a| a.0 == name)
        .unwrap_or_else(|| panic!("unknown arch {name}"))
        .1
}

/// HASH capacity presets at S = 16: `(2-input hashes, extra hashes by input count)`.
/// Counts and their derivation: `experiments-2.md` section 4.
fn hash_preset(name: &str) -> (usize, Vec<usize>) {
    let rec = |c: usize| -> Vec<usize> {
        let mut v = vec![3; c];
        v.extend([31, 31]);
        v
    };
    match name {
        "none" => (0, vec![]),
        "dig" => (0, vec![31, 31]),
        "m1c8" => (320, rec(8)),
        "m1p3" => (1920, rec(48)),
        "m2c8" => (64, rec(8)),
        "m3c8" => (41, rec(8)),
        "m3c16" => (47, rec(16)),
        "hash0" => (0, vec![]),
        _ => panic!("unknown hash preset {name}"),
    }
}

/// `slim_<preset>`: the slim HASH key-shape stand-in, filled for as many
/// permutations as the preset's hashes need (2 inputs: 1; 3: 2; 31: 16).
fn rf_leaf(arch: Arch, pad: usize, preset: &str, s: usize) -> RfLeaf {
    // `<preset>+xl<n>`: n extra lookup arguments (node-cost probe).
    let (preset, xl) = match preset.split_once("+xl") {
        Some((p, n)) => (p, n.parse::<usize>().unwrap()),
        None => (preset, 0),
    };
    if let Some(p) = preset.strip_prefix("slim_") {
        let (n2, extra) = hash_preset(p);
        let params = RfParams {
            arch,
            hash: false,
            pad,
            slim_hash: true,
            extra_lookups: xl,
        };
        let mut c = RfLeaf::new(params, s, None);
        c.slim_perms = n2 + extra.iter().map(|n| n.div_ceil(2)).sum::<usize>();
        return c;
    }
    let (n2, extra) = hash_preset(preset);
    let params = RfParams {
        arch,
        hash: preset != "none",
        pad,
        slim_hash: false,
        extra_lookups: xl,
    };
    RfLeaf::new(params, s, None).with_hashes(n2, extra)
}

fn vk_of<Ci: midnight_proofs::plonk::Circuit<F>>(c: &Ci) -> (Vk, u32) {
    let k = circuit_model::<F, 48, 32>(c).k;
    (keygen_vk_with_k(&params(k), c, k).unwrap(), k)
}

fn cs_json(cs: &ConstraintSystem<F>) -> J {
    json!({
        "advice": cs.num_advice_columns(), "fixed": cs.num_fixed_columns(),
        "instance": cs.num_instance_columns(), "selectors": cs.num_selectors(),
        "lookups": cs.lookups().len(), "permutation_columns": cs.permutation().get_columns().len(),
        "degree": cs.degree(), "gates": cs.gates().len(),
        "advice_queries": cs.advice_queries().len(), "fixed_queries": cs.fixed_queries().len(),
    })
}

#[test]
fn rf_leaf_s16_mock_prover_all_layouts() {
    for (name, arch) in E5_ARCHS {
        let w = RfWitness::from_trace(&run32(genesis(), 16), 16);
        assert_eq!(w.steps(), 16);
        let c = RfLeaf::new(
            RfParams {
                arch,
                ..Default::default()
            },
            16,
            Some(w.clone()),
        );
        let p = MockProver::run(&c, vec![vec![], w.instance()]).unwrap();
        p.verify()
            .unwrap_or_else(|e| panic!("{name}: {:?}", &e[..e.len().min(3)]));
    }
}

/// The keyed register-file leaves (every layout, with and without HASH) query
/// rotations -1, 0, +1 only (`slim_m3c8`: the slim HASH key shape).
#[test]
fn rf_leaf_vk_rotations_are_unit() {
    for (name, arch) in E5_ARCHS {
        for preset in ["none", "dig", "slim_m3c8"] {
            let (vk, k) = vk_of(&rf_leaf(arch, 1, preset, 1));
            let cs = vk.cs();
            let mut rots: Vec<i32> = cs
                .advice_queries()
                .iter()
                .map(|q| q.1.0)
                .chain(cs.fixed_queries().iter().map(|q| q.1.0))
                .chain(cs.instance_queries().iter().map(|q| q.1.0))
                .collect();
            rots.sort();
            rots.dedup();
            println!("CHIP-ROT {name} {preset} k={k}: {rots:?}");
            assert!(
                rots.iter().all(|r| (-1..=1).contains(r)),
                "{name} {preset}: {rots:?}"
            );
        }
    }
}

#[test]
#[ignore = "E5 rows: cost_model of the register-file leaf, every layout, S = 1, 4, 16, 64 (seconds)"]
fn chip_e5_leaf_rows() {
    let mut out = BTreeMap::new();
    for (name, arch) in E5_ARCHS {
        for pad in [1usize, 4] {
            if pad > 1 && arch != Arch::Wide {
                continue;
            }
            for s in [1usize, 4, 16, 64] {
                let c = rf_leaf(arch, pad, "none", s);
                let t = Instant::now();
                let m = circuit_model::<F, 48, 32>(&c);
                let mut v = model_json(&m);
                v["region_rows"] = json!(RfLeaf::region_rows(c.params, s, 0));
                v["seconds"] = json!(t.elapsed().as_secs_f64());
                let key = if pad > 1 {
                    format!("{name}_pad{pad}_S{s}")
                } else {
                    format!("{name}_S{s}")
                };
                println!("CHIP-E5 leaf {key} {v}");
                out.insert(key, v);
            }
        }
    }
    emit("e5_leaf_rows", &json!(out));
}

#[test]
#[ignore = "E6 HASH rows: cost_model of the register-file leaf with N Poseidon hashes (seconds)"]
fn chip_e6_hash_rows() {
    let arch_name = std::env::var("CHIP_E6_ARCH").unwrap_or("staggered".into());
    let arch = arch_of(&arch_name);
    let mut out = BTreeMap::new();
    let mut run = |key: String, c: RfLeaf| {
        let m = circuit_model::<F, 48, 32>(&c);
        let mut v = model_json(&m);
        v["main_region_rows"] = json!(RfLeaf::region_rows(c.params, c.s, c.n_hash2));
        v["n_hash2"] = json!(c.n_hash2);
        v["hash_extra"] = json!(c.hash_extra);
        println!("CHIP-E6 {key} {v}");
        out.insert(key, v);
    };
    let base = |n2: usize, extra: Vec<usize>| {
        RfLeaf::new(
            RfParams {
                arch,
                hash: true,
                pad: 1,
                slim_hash: false,
                extra_lookups: 0,
            },
            16,
            None,
        )
        .with_hashes(n2, extra)
    };
    run("nohash_S16".into(), rf_leaf(arch, 1, "none", 16));
    for n2 in [0usize, 1, 2, 8, 64, 256] {
        run(format!("bus2_{n2}"), base(n2, vec![]));
    }
    for n in [2usize, 3, 4, 8, 31, 32] {
        run(format!("free_arity{n}_x8"), base(0, vec![n; 8]));
    }
    for preset in ["dig", "m1c8", "m1p3", "m2c8", "m3c8", "m3c16"] {
        run(format!("preset_{preset}"), rf_leaf(arch, 1, preset, 16));
    }
    emit(&format!("e6_hash_rows_{arch_name}"), &json!(out));
}

/// N stdlib Poseidon hashes of `arity` inputs each (calibration of the HASH rows
/// against the stdlib's own arch).
#[derive(Clone, Debug)]
struct StdHashes {
    n: usize,
    arity: usize,
}

impl Relation for StdHashes {
    type Instance = Vec<F>;
    type Witness = ();
    type Error = Error;

    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }

    fn circuit(
        &self,
        std: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _instance: Value<Vec<F>>,
        _witness: Value<()>,
    ) -> Result<(), Error> {
        for t in 0..self.n {
            let xs = (0..self.arity)
                .map(|i| std.assign(l, Value::known(F::from((t * 100 + i) as u64))))
                .collect::<Result<Vec<AssignedNative<F>>, Error>>()?;
            let _ = std.poseidon(l, &xs)?;
        }
        Ok(())
    }

    fn used_chips(&self) -> ZkStdLibArch {
        ZkStdLibArch {
            poseidon: true,
            ..ZkStdLibArch::default()
        }
    }

    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&(self.n as u64).to_le_bytes())?;
        w.write_all(&(self.arity as u64).to_le_bytes())
    }

    fn read_relation<R: io::Read>(_r: &mut R) -> io::Result<Self> {
        Err(io::Error::other("measurement only"))
    }
}

#[test]
#[ignore = "E6 calibration: stdlib relation with N Poseidon hashes (seconds)"]
fn chip_e6_stdlib_hash() {
    let mut out = BTreeMap::new();
    for (n, arity) in [
        (1usize, 2usize),
        (2, 2),
        (16, 2),
        (64, 2),
        (16, 3),
        (16, 4),
        (4, 31),
    ] {
        let m = cost_model(&StdHashes { n, arity }, None);
        let v = model_json(&m);
        println!("CHIP-E6 stdlib n={n} arity={arity} {v}");
        out.insert(format!("n{n}_arity{arity}"), v);
    }
    emit("e6_stdlib_hash", &json!(out));
}

/// Node (and optionally the run decider) over register-file leaf keys at S = 16.
/// `CHIP_E5_SPECS`: `;`-separated `arch,pad,preset` (default: the four layouts,
/// pad 1, no HASH). `CHIP_E5_DECIDER`: index of the spec whose node the decider
/// is measured over.
#[test]
#[ignore = "E5/E6 node over register-file leaf keys: one k20 node cost model per spec; optional node keygen and decider"]
fn chip_e5_node() {
    let specs = std::env::var("CHIP_E5_SPECS")
        .unwrap_or("wide,1,none;wide_inline,1,none;narrow,1,none;staggered,1,none".into());
    let decider = std::env::var("CHIP_E5_DECIDER")
        .ok()
        .map(|v| v.parse::<usize>().unwrap());
    let standin = standin_vk(13);
    let base = SherlockIvcRelation::new(4, 20, standin).unwrap();
    for (i, spec) in specs.split(';').enumerate() {
        let f: Vec<&str> = spec.split(',').collect();
        let (arch, pad, preset) = (arch_of(f[0]), f[1].parse::<usize>().unwrap(), f[2]);
        let leaf = rf_leaf(arch, pad, preset, 16);
        let t = Instant::now();
        let lm = circuit_model::<F, 48, 32>(&leaf);
        let (vk, k) = vk_of(&leaf);
        let leaf_s = t.elapsed().as_secs_f64();
        let node = ChipNode::new(base.clone(), vk.clone());
        let t = Instant::now();
        let m = cost_model(&node, Some(20));
        let mut v = json!({
            "spec": spec, "s": 16, "leaf_k": k, "leaf_model": model_json(&lm),
            "leaf_cs": cs_json(vk.cs()), "leaf_seconds": leaf_s,
            "node": model_json(&m), "node_seconds": t.elapsed().as_secs_f64(),
        });
        let name = format!("e5_node_{}", spec.replace(',', "_"));
        println!("CHIP-E5 node {v}");
        emit(&name, &v);
        if decider == Some(i) {
            let base_params = params(20);
            let t = Instant::now();
            let node_vk = setup_vk(&base_params, &node);
            let keygen_s = t.elapsed().as_secs_f64();
            let drel = RunDeciderRelation {
                nr_cols: 4,
                node_vk,
                s_g2: base_params.s_g2().to_affine(),
                template: Template::from_any_json(include_bytes!(
                    "../../mori-recursion/tests/fixtures/run-template.json"
                ))
                .unwrap(),
                cc_inputs: CC_INPUTS.to_vec(),
            };
            drel.validate().unwrap();
            let dm = cost_model(&drel, None);
            v["decider"] = json!({"node_keygen_seconds": keygen_s, "model": model_json(&dm),
                "srs": if std::env::var_os("MORIARTY_PARENT_SRS").is_some() { "official k20" } else { "unsafe" }});
            println!("CHIP-E5 decider {v}");
            emit(&name, &v);
        }
    }
}
