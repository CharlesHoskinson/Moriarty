//! E3 (design review 2026-11-10, C2 / dissent 2): cost of an in-circuit canonical-genesis
//! check in the leaf, against off-chain recomputation of `init_d`. Modeling only.
//!
//! The prototype gates on `h_pre == 0` arithmetically (`TestEq`, then per pre-carrier lane
//! `CondSelect(is_genesis, canon, lane)` and `ConstrainEq(_, lane)`); ZKIR instruction guards
//! do not constrain. `canon` is an immediate for a lane that is equal across programs and a
//! copy of a prefix (META/PROG/CTX) lane where one prefix lane matches for every program.
//! Lanes neither rule explains are reported; their derivation is not modeled.
use std::{path::PathBuf, sync::Arc, time::Instant};

use midnight_transient_crypto::curve::Fr;
use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage};
use midnight_zk_stdlib::cost_model;
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_backend::sherlock_c6_child::{DigestSections, StateOutcome, c6_child_relation};
use mori_backend::sherlock_c6_witness_stream::{C6WitnessStep, for_each_witness};
use mori_core::sherlock::helper_machine::complete_observation::capture;
use mori_core::sherlock_compile_artifact::draft_execution::draft_genesis;
use serde_json::{Value, json};

const LITERAL: &str = "language \"moriarty-sherlock-draft/1\"; program P { entry 7; }";
const ADD: &str = "language \"moriarty-sherlock-draft/1\"; program P { entry 3 + 4; }";
const FACT: &str = "language \"moriarty-sherlock-draft/1\"; program P { fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } } entry fact(3); }";
const LIST123: &str = "language \"moriarty-sherlock-draft/1\";\n\
program List123 {\n\
  fn sum(xs: List): UInt { match xs { nil => 0, cons(h, t) => h + sum(t) } }\n\
  entry sum(cons(1, cons(2, cons(3, nil))));\n\
}\n";

fn config_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/c6-witness-stream/provisional-host-config.json")
}

/// Step 0 of the witness stream (genesis pre carriers) for `source` under `config`.
fn genesis_step(source: &str, config: &std::path::Path) -> (C6WitnessStep, f64) {
    let mut first = None;
    let t = Instant::now();
    let _ = for_each_witness(source, config, |step| {
        first = Some(step);
        Err(mori_backend::Error {
            code: "e3".into(),
            message: "stop after step 0".into(),
        })
    });
    let s = t.elapsed().as_secs_f64();
    (first.expect("step 0"), s)
}

struct Genesis {
    name: &'static str,
    prefix: Vec<u128>,
    pre: Vec<u128>,
    post_lanes: usize,
    meta: usize,
    prog: usize,
    ctx: usize,
    image: usize,
    data: usize,
    helper: usize,
    seconds_to_step0: f64,
}

fn genesis(name: &'static str, source: &str, config: &std::path::Path) -> Genesis {
    let (step, s) = genesis_step(source, config);
    let (lanes, sections) = step.digest_statement();
    let w = &step.witness;
    Genesis {
        name,
        prefix: lanes[..sections.prog_lanes].to_vec(),
        pre: lanes[sections.prog_lanes..sections.prog_lanes + sections.pre_lanes].to_vec(),
        post_lanes: sections.post_lanes,
        meta: w.host_prefix.meta.len(),
        prog: w.host_prefix.prog.len(),
        ctx: w.host_prefix.ctx.len(),
        image: w.pre_image.len(),
        data: w.pre_data.len(),
        helper: w.pre_helper.len(),
        seconds_to_step0: s,
    }
}

/// How each pre lane's canonical value is obtained.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Canon {
    Const(u128),
    Prefix(usize),
    Unexplained,
}

fn classify(gs: &[&Genesis]) -> Vec<Canon> {
    let n = gs[0].pre.len();
    (0..n)
        .map(|j| {
            let v0 = gs[0].pre[j];
            if gs.iter().all(|g| g.pre[j] == v0) {
                return Canon::Const(v0);
            }
            let p = gs[0].prefix.len();
            (0..p)
                .find(|&i| gs.iter().all(|g| g.prefix.get(i) == Some(&g.pre[j])))
                .map_or(Canon::Unexplained, Canon::Prefix)
        })
        .collect()
}

fn var(s: String) -> Operand {
    Operand::Variable(Identifier(s))
}

/// The gated check: `is_genesis = (h == 0)`, then for every pre lane
/// `ConstrainEq(CondSelect(is_genesis, canon, lane), lane)`.
fn check_instructions(
    h: Operand,
    pre: impl Fn(usize) -> Operand,
    prefix: impl Fn(usize) -> Operand,
    canon: &[Canon],
) -> Vec<Instruction> {
    let g = Identifier("%e3.is_genesis".into());
    let mut code = vec![Instruction::TestEq {
        a: h,
        b: Operand::Immediate(Fr::from(0u64)),
        output: g.clone(),
    }];
    for (j, c) in canon.iter().enumerate() {
        let a = match c {
            Canon::Const(v) => Operand::Immediate(Fr::from(*v)),
            Canon::Prefix(i) => prefix(*i),
            // Callers resolve unexplained lanes first (see `resolved`).
            Canon::Unexplained => unreachable!("unexplained lane reached the check"),
        };
        let s = Identifier(format!("%e3.sel.{j}"));
        code.push(Instruction::CondSelect {
            bit: Operand::Variable(g.clone()),
            a,
            b: pre(j),
            output: s.clone(),
        });
        code.push(Instruction::ConstrainEq {
            a: Operand::Variable(s),
            b: pre(j),
        });
    }
    code
}

fn load(code: Vec<Instruction>) -> IrSource {
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

/// LocalPublic shape (`tests/sherlock_c6_child.rs:14`): public lanes, 128-bit range checks.
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
    load(code)
}

/// Standalone check relation: private prefix and pre lanes, public `h`, the check.
fn standalone(p: usize, canon: &[Canon], with_check: bool) -> IrSource {
    let mut code = Vec::new();
    for i in 0..p {
        code.push(Instruction::PrivateInput {
            guard: None,
            val_t: IrType::Native,
            output: Identifier(format!("%e3.p.{i}")),
        });
    }
    for j in 0..canon.len() {
        code.push(Instruction::PrivateInput {
            guard: None,
            val_t: IrType::Native,
            output: Identifier(format!("%e3.c.{j}")),
        });
    }
    let h = Identifier("%e3.h".into());
    code.push(Instruction::PublicInput {
        guard: None,
        val_t: IrType::Native,
        output: h.clone(),
    });
    code.push(Instruction::Impact {
        guard: Operand::Immediate(Fr::from(1u64)),
        inputs: vec![Operand::Variable(h.clone())],
    });
    if with_check {
        code.extend(check_instructions(
            Operand::Variable(h),
            |j| var(format!("%e3.c.{j}")),
            |i| var(format!("%e3.p.{i}")),
            canon,
        ));
    }
    load(code)
}

fn nonproving(ir: &IrSource, prefix: &[u128], pre: &[u128], h: u64) -> bool {
    let private: Vec<Fr> = prefix.iter().chain(pre).map(|v| Fr::from(*v)).collect();
    let preimage = ProofPreimage {
        inputs: Vec::new(),
        private_transcript: private,
        public_transcript_inputs: vec![Fr::from(h)],
        public_transcript_outputs: vec![Fr::from(h)],
        binding_input: Fr::from(1u64),
        communications_commitment: None,
        key_location: KeyLocation("moriarty-e3-genesis-check/1".into()),
    };
    preimage.check(ir).is_ok()
}

fn model(ir: &IrSource) -> Value {
    let m = cost_model(ir, None);
    json!({"rows": m.rows, "k": m.k})
}

#[test]
#[ignore = "E3 modeling probe; light (cost models and host genesis only)"]
fn e3_canonical_genesis_check_cost() {
    let cfg = config_path();
    let gs = vec![
        genesis("literal", LITERAL, &cfg),
        genesis("add", ADD, &cfg),
        genesis("fact3", FACT, &cfg),
        genesis("list123", LIST123, &cfg),
    ];
    // Carrier lengths are per program (a result in itself). Lanes are classified across
    // the programs whose pre carrier has fact(3)'s length, fact(3) first.
    let fact = &gs[2];
    let group: Vec<&Genesis> = std::iter::once(fact)
        .chain(
            gs.iter()
                .filter(|g| g.name != fact.name && g.pre.len() == fact.pre.len()),
        )
        .collect();
    let canon = classify(&group);
    let consts = canon
        .iter()
        .filter(|c| matches!(c, Canon::Const(_)))
        .count();
    let zero_consts = canon.iter().filter(|c| **c == Canon::Const(0)).count();
    let copies = canon
        .iter()
        .filter(|c| matches!(c, Canon::Prefix(_)))
        .count();
    let unexplained: Vec<usize> = canon
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == Canon::Unexplained)
        .map(|(j, _)| j)
        .collect();

    // Same per-lane gadget for an unexplained lane, with fact(3)'s value as the immediate:
    // the row cost is identical, the derivation logic such a lane needs is not modeled.
    let canon: Vec<Canon> = canon
        .iter()
        .enumerate()
        .map(|(j, c)| match c {
            Canon::Unexplained => Canon::Const(fact.pre[j]),
            c => *c,
        })
        .collect();

    // Fuel and stack limit: which sections change when only the config value changes.
    let tmp = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let base_cfg: Value = serde_json::from_str(&std::fs::read_to_string(&cfg).unwrap()).unwrap();
    let mut varied = Vec::new();
    for (field, value) in [("fuel", "3999"), ("stack", "127")] {
        let mut c = base_cfg.clone();
        c[field]["value"] = json!(value);
        let path = tmp.join(format!("e3-{field}.json"));
        std::fs::write(&path, c.to_string()).unwrap();
        let v = genesis("fact3-varied", FACT, &path);
        let base = &gs[2];
        let diff = |a: &[u128], b: &[u128]| -> Vec<usize> {
            (0..a.len().max(b.len()))
                .filter(|&i| a.get(i) != b.get(i))
                .collect()
        };
        varied.push(json!({
            "field": field, "value": value,
            "prefix_lanes_changed": diff(&base.prefix, &v.prefix),
            "pre_lanes_changed": diff(&base.pre, &v.pre),
            "prefix_len": v.prefix.len(), "pre_len": v.pre.len(),
        }));
    }

    // Rows: the digest leaf at the real section sizes, with and without the check.
    let g = &gs[2];
    let (p, n, m) = (g.prefix.len(), g.pre.len(), g.post_lanes);
    let sections = DigestSections {
        prog_lanes: p,
        pre_lanes: n,
        post_lanes: m,
    };
    let leaf = c6_child_relation(
        &local_public_ir(p + n + m),
        sections,
        StateOutcome::RUNNING,
        StateOutcome::RUNNING,
    )
    .unwrap();
    let mut with = (*leaf.instructions).clone();
    with.extend(check_instructions(
        var("%c6.public.h".into()),
        |j| var(format!("%c6.public.{}", p + j)),
        |i| var(format!("%c6.public.{i}")),
        &canon,
    ));
    let leaf_with = IrSource {
        instructions: Arc::new(with),
        ..leaf.clone()
    };
    let leaf_base_m = cost_model(&leaf, None);
    let leaf_with_m = cost_model(&leaf_with, None);

    // Standalone: the check alone over the same lanes, and its inputs-only baseline.
    let sa_base = standalone(p, &canon, false);
    let sa_check = standalone(p, &canon, true);
    let sa_base_m = cost_model(&sa_base, None);
    let sa_check_m = cost_model(&sa_check, None);

    // Nonproving: canonical at h=0 accepted; one non-canonical lane refused at h=0 and
    // accepted at h=3 (the check is gated on genesis only).
    let probe_lane = canon
        .iter()
        .position(|c| matches!(c, Canon::Const(_)))
        .expect("a constant lane");
    let mut forged = g.pre.clone();
    forged[probe_lane] = forged[probe_lane].wrapping_add(1);
    let np = json!({
        "canonical_h0_accepted": nonproving(&sa_check, &g.prefix, &g.pre, 0),
        "forged_h0_refused": !nonproving(&sa_check, &g.prefix, &forged, 0),
        "forged_h3_accepted": nonproving(&sa_check, &g.prefix, &forged, 3),
        "forged_lane": probe_lane,
    });
    assert_eq!(np["canonical_h0_accepted"], json!(true));
    assert_eq!(np["forged_h0_refused"], json!(true));
    assert_eq!(np["forged_h3_accepted"], json!(true));

    // Off-chain alternative: recompute init_d's inputs on the host.
    let src = std::fs::read_to_string(&cfg).unwrap();
    let t = Instant::now();
    let dg = draft_genesis(FACT, &src)
        .unwrap_or_else(|e| panic!("{}", serde_json::to_string(&e).unwrap()));
    let genesis_s = t.elapsed().as_secs_f64();
    let t = Instant::now();
    let _snap = capture(&dg.admitted, &dg.image).unwrap();
    let capture_s = t.elapsed().as_secs_f64();

    let summary = json!({
        "schema": "moriarty-e3-genesis-check/1",
        "config": "crates/mori-backend/tests/fixtures/c6-witness-stream/provisional-host-config.json",
        "programs": gs.iter().map(|g| json!({
            "name": g.name, "prefix": g.prefix.len(), "meta": g.meta, "prog": g.prog, "ctx": g.ctx,
            "pre": g.pre.len(), "image": g.image, "data": g.data, "helper": g.helper,
            "post": g.post_lanes,
            "pre_nonzero": g.pre.iter().filter(|v| **v != 0).count(),
            "seconds_to_step0": g.seconds_to_step0,
        })).collect::<Vec<_>>(),
        "canon": {"classified_over": group.iter().map(|g| g.name).collect::<Vec<_>>(),
                  "pre_lanes": canon.len(), "const": consts, "const_zero": zero_consts,
                  "prefix_copy": copies, "unexplained": unexplained.len(),
                  "unexplained_lanes": unexplained},
        "config_dependence": varied,
        "leaf_digest_only": {"base": json!({"rows": leaf_base_m.rows, "k": leaf_base_m.k}),
                              "with_check": json!({"rows": leaf_with_m.rows, "k": leaf_with_m.k}),
                              "delta_rows": leaf_with_m.rows as i64 - leaf_base_m.rows as i64},
        "standalone": {"inputs_only": model(&sa_base), "with_check": model(&sa_check),
                       "delta_rows": sa_check_m.rows as i64 - sa_base_m.rows as i64},
        "nonproving": np,
        "offchain": {"draft_genesis_seconds": genesis_s, "capture_seconds": capture_s,
                     "to_step0_seconds_fact3": g.seconds_to_step0},
    });
    println!("E3 {summary}");
    if let Some(out) = std::env::var_os("MORIARTY_E3_OUT") {
        let out = PathBuf::from(out);
        std::fs::create_dir_all(&out).unwrap();
        std::fs::write(
            out.join("e3.json"),
            serde_json::to_vec_pretty(&summary).unwrap(),
        )
        .unwrap();
    }
}
