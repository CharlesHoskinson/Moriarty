//! E4 (design review 2026-11-10, C1/C3): prove one k = 20 stand-in C6 digest leaf through
//! the leaf prover (`C6DigestKey`), with the pinned ceremony `bls_midnight_2p20`.
//!
//! Stand-in: the C6 LocalPublic shape (`tests/sherlock_c6_child.rs:14`, `L` public lanes,
//! each range-checked to 128 bits) plus `U` fragment-like filler units over the lanes
//! (per unit: `Mul`, `Add`, `LessThan` 64-bit, `CondSelect`), wrapped by the digest relation
//! (Poseidon trees over prog/pre/post). `U` is chosen so the digest relation needs k = 20
//! on its own (rows above 2^19), not a small circuit keyed at k = 20. It proves nothing
//! about the Sherlock machine; it has the ZKIR leaf arch and fills the k = 20 domain.
use std::{path::PathBuf, sync::Arc, time::Instant};

use midnight_transient_crypto::curve::Fr;
use midnight_zk_stdlib::cost_model;
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_backend::proving::{C6_LEAF_MAX_K, ProofOptions};
use mori_backend::sherlock_c6_child::{
    C6DigestKey, DigestSections, StateOutcome, c6_child_relation,
};
use serde_json::json;

const THIRD: usize = 1024;

fn standin_ir(lanes: usize, units: usize) -> IrSource {
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
    let v = |s: String| Operand::Variable(Identifier(s));
    let mut acc = published[0].clone();
    for u in 0..units {
        let a = published[u % lanes].clone();
        let b = published[(u * 7 + 1) % lanes].clone();
        let m = format!("%e4.m.{u}");
        let s = format!("%e4.s.{u}");
        let lt = format!("%e4.lt.{u}");
        let c = format!("%e4.c.{u}");
        code.push(Instruction::Mul {
            a: acc.clone(),
            b: b.clone(),
            output: Identifier(m.clone()),
        });
        code.push(Instruction::Add {
            a: v(m),
            b: a.clone(),
            output: Identifier(s.clone()),
        });
        code.push(Instruction::LessThan {
            a: a.clone(),
            b: b.clone(),
            bits: 64,
            output: Identifier(lt.clone()),
        });
        code.push(Instruction::CondSelect {
            bit: v(lt),
            a: v(s),
            b: a,
            output: Identifier(c.clone()),
        });
        acc = v(c);
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

fn sections() -> DigestSections {
    DigestSections {
        prog_lanes: THIRD,
        pre_lanes: THIRD,
        post_lanes: THIRD,
    }
}

fn digest_rows(units: usize) -> (u32, usize) {
    let rel = c6_child_relation(
        &standin_ir(3 * THIRD, units),
        sections(),
        StateOutcome::RUNNING,
        StateOutcome::RUNNING,
    )
    .unwrap();
    let m = cost_model(&rel, None);
    (m.k, m.rows)
}

/// Smallest filler that makes the digest relation need k = 20, plus a margin so the rows
/// sit well above 2^19 (target 0.75 x 2^20).
fn pick_units() -> (usize, u32, usize) {
    if let Some(u) = std::env::var("MORIARTY_E4_UNITS")
        .ok()
        .and_then(|s| s.parse().ok())
    {
        let (k, rows) = digest_rows(u);
        return (u, k, rows);
    }
    let (_, r0) = digest_rows(0);
    let (_, r1) = digest_rows(4096);
    let per_unit = (r1 - r0) as f64 / 4096.0;
    let target = 0.75 * (1u64 << 20) as f64;
    let units = ((target - r0 as f64) / per_unit).ceil().max(0.0) as usize;
    let (k, rows) = digest_rows(units);
    println!("E4PICK r0={r0} per_unit={per_unit:.3} units={units} k={k} rows={rows}");
    (units, k, rows)
}

#[test]
#[ignore = "E4: real k=20 keygen and prove; MORIARTY_TEST_PARAMS_DIR with bls_midnight_2p20; heavy.lock"]
fn e4_prove_one_k20_standin_leaf() {
    let wall = Instant::now();
    let (units, k, rows) = pick_units();
    assert_eq!(k, 20, "stand-in must need k = 20 on its own (rows {rows})");
    assert!(rows > 1 << 19);
    let model_s = wall.elapsed().as_secs_f64();
    let options = ProofOptions {
        params_dir: std::env::var_os("MORIARTY_TEST_PARAMS_DIR")
            .expect("explicit pinned local SRS directory")
            .into(),
        max_k: C6_LEAF_MAX_K,
    };
    let ir = standin_ir(3 * THIRD, units);
    let statement: Vec<u128> = (0..3 * THIRD).map(|i| i as u128 + 1).collect();
    let t = Instant::now();
    let key = C6DigestKey::reconstruct(
        &ir,
        "E4StandIn",
        &statement,
        sections(),
        StateOutcome::RUNNING,
        StateOutcome::RUNNING,
        0,
        &options,
    )
    .expect("k=20 C6 digest leaf keygen");
    let srs_and_vk_s = t.elapsed().as_secs_f64();
    assert_eq!(key.prepared().claim.circuit_k, 20);
    let (proof, laps) = key.prove_timed().expect("k=20 leaf proof");
    let summary = json!({
        "schema": "moriarty-e4-leaf-k20/1",
        "relation": "C6 digest leaf stand-in: LocalPublic 3x1024 lanes (128-bit range checks) + U filler units (Mul, Add, LessThan64, CondSelect), digest trees",
        "units": units, "lanes": 3 * THIRD, "k": k, "model_rows": rows,
        "rayon_threads": std::env::var("RAYON_NUM_THREADS").ok(),
        "seconds": {
            "model_pick": model_s, "srs_load_and_vk": srs_and_vk_s,
            "witness_check": laps[0], "setup_pk": laps[1], "prove": laps[2], "verify": laps[3],
            "process_wall": wall.elapsed().as_secs_f64(),
        },
        "proof_bytes": proof.proof.len(),
        "vk_sha256": key.vk_sha256(),
    });
    println!("E4 {summary}");
    if let Some(out) = std::env::var_os("MORIARTY_E4_OUT") {
        let out = PathBuf::from(out);
        std::fs::create_dir_all(&out).unwrap();
        let tag = std::env::var("MORIARTY_E4_TAG").unwrap_or_else(|_| "solo".into());
        std::fs::write(
            out.join(format!("e4-{tag}.json")),
            serde_json::to_vec_pretty(&summary).unwrap(),
        )
        .unwrap();
    }
}
