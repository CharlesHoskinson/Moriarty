use std::path::PathBuf;

use midnight_circuits::hash::poseidon::PoseidonChip;
use midnight_circuits::instructions::hash::HashCPU;
use midnight_transient_crypto::curve::{Fr, outer};
use mori_backend::sherlock_c6_child::{
    DigestSections, StateOutcome, hash_tree_offcircuit, statement_digests,
};
use mori_backend::sherlock_c6_witness_stream::{
    C6_WITNESS_STREAM_FORMAT, for_each_witness, write_witness_stream,
};
use mori_backend::sherlock_child::field_hex;
use mori_backend::sherlock_machine::c6_dispatch_consumer::witness_from_host_snapshot;
use mori_core::sherlock::helper_machine::complete_observation::capture;
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::{OwnerEngine, StepResult};
use mori_core::sherlock_compile_artifact::draft_execution::{draft_genesis, run_draft_host};

const LITERAL: &str = "language \"moriarty-sherlock-draft/1\"; program P { entry 7; }";
const ADD: &str = "language \"moriarty-sherlock-draft/1\"; program P { entry 3 + 4; }";
const FACT: &str = "language \"moriarty-sherlock-draft/1\"; program P { fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } } entry fact(3); }";

fn config(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/c6-witness-stream")
        .join(name)
}

// Independent transcription of ivc::state_digest and run_decider's final opening.
fn frozen_state_digest(outcome: StateOutcome, lanes_d: Fr) -> Fr {
    Fr(<PoseidonChip<outer::Scalar> as HashCPU<
        outer::Scalar,
        outer::Scalar,
    >>::hash(&[
        Fr::from(u128::from_le_bytes(*b"SHK_SD\0\0\0\0\0\0\0\0\0\0")).0,
        Fr::from(outcome.tag).0,
        Fr::from(outcome.value).0,
        lanes_d.0,
    ]))
}

fn check_every_carrier_lane(lanes: &[u128], outcome: StateOutcome, state_d: Fr) -> usize {
    // Retain unchanged subtrees so checking every padded lane costs O(n log n),
    // not O(n^2). Each changed leaf is propagated through the entire hash path.
    let mut levels = vec![lanes.iter().copied().map(Fr::from).collect::<Vec<_>>()];
    while levels.last().unwrap().len() > 4 {
        levels.push(
            levels
                .last()
                .unwrap()
                .chunks(4)
                .map(hash_tree_offcircuit)
                .collect(),
        );
    }
    assert_eq!(
        state_d,
        frozen_state_digest(outcome, hash_tree_offcircuit(&levels[0]))
    );
    for i in 0..lanes.len() {
        let mut index = i;
        let mut changed = levels[0][i] + Fr::from(1u64);
        for level in &levels {
            let start = index / 4 * 4;
            let mut chunk = level[start..(start + 4).min(level.len())].to_vec();
            chunk[index % 4] = changed;
            changed = hash_tree_offcircuit(&chunk);
            index /= 4;
        }
        assert_ne!(
            state_d,
            frozen_state_digest(outcome, changed),
            "carrier lane {i}"
        );
    }
    lanes.len()
}

fn exercise(source: &str, config_name: &str, expected_steps: u64, expected_result: u128) {
    let path = config(config_name);
    let config_text = std::fs::read_to_string(&path).unwrap();
    let mut genesis = draft_genesis(source, &config_text)
        .unwrap_or_else(|e| panic!("{}", serde_json::to_string(&e).unwrap()));
    let engine = host_owner_engine();
    let mut previous = None;
    let mut count = 0;
    let mut mutations = 0;
    let summary = for_each_witness(source, &path, |step| {
        assert_eq!(step.h_pre, count);
        assert_eq!(step.pre_outcome, StateOutcome::RUNNING);
        if let Some(post_d) = previous {
            assert_eq!(step.pre_d, post_d, "link at step {count}");
        } else {
            assert_eq!(step.pre_d, step.init_d);
        }
        let pre = capture(&genesis.admitted, &genesis.image).unwrap();
        let dispatch = witness_from_host_snapshot(
            true,
            &pre,
            step.witness.source_params.cfg(),
            &step.witness.source_params.limits(),
        )
        .unwrap();
        let statement = step.witness.to_statement_frs();
        assert_eq!(
            &statement[..dispatch.to_statement_frs().len()],
            dispatch.to_statement_frs()
        );
        match engine.step(&genesis.admitted, &mut genesis.image).unwrap() {
            StepResult::Event { event } => assert_eq!(step.rule, event.rule_id),
            StepResult::Terminal => panic!("extra witness"),
        }
        let (lanes, sections) = step.digest_statement();
        assert_eq!(
            statement_digests(&lanes, sections, step.pre_outcome, step.post_outcome).unwrap(),
            [step.prog_d, step.pre_d, step.post_d]
        );
        assert_eq!(
            step.pre_d,
            frozen_state_digest(step.pre_outcome, step.pre_lanes_d)
        );
        assert_eq!(
            step.post_d,
            frozen_state_digest(step.post_outcome, step.post_lanes_d)
        );
        if count == 0 {
            mutations += check_every_carrier_lane(
                &lanes[..sections.prog_lanes + sections.pre_lanes],
                step.pre_outcome,
                step.pre_d,
            );
        }
        if step.post_outcome.tag == 1 {
            let mut post = lanes[..sections.prog_lanes].to_vec();
            post.extend(&lanes[sections.prog_lanes + sections.pre_lanes..]);
            mutations += check_every_carrier_lane(&post, step.post_outcome, step.post_d);
        }
        previous = Some(step.post_d);
        count += 1;
        Ok(())
    })
    .unwrap();
    assert_eq!(count, expected_steps);
    assert_eq!(summary.n_steps, count);
    assert_eq!(summary.result, expected_result);
    assert_eq!(Some(summary.fin_d), previous);
    assert_eq!(
        summary.fin_d,
        frozen_state_digest(StateOutcome::halted(expected_result), summary.lanes_d)
    );
    assert!(matches!(
        engine.step(&genesis.admitted, &mut genesis.image).unwrap(),
        StepResult::Terminal
    ));
    // This is the same public execution route used by `mori simulate`.
    let simulated = run_draft_host(
        source,
        &config_text,
        r#"{"kind":"host-only-fixture","name":"c6-witness-stream"}"#,
        &mut |_| {},
    )
    .unwrap_or_else(|e| panic!("{}", serde_json::to_string(&e).unwrap()));
    assert_eq!(simulated.completed.summary.events, u128::from(count));
    let observed = simulated
        .observe()
        .unwrap_or_else(|_| panic!("observe failed"));
    let mori_core::sherlock::helper_machine::shared::OutcomeC::Halted { value } =
        *observed.image.outcome
    else {
        panic!("not halted")
    };
    use mori_core::sherlock::helper_machine::{
        raw::{ConstRow, DataRow},
        shared::RefC,
    };
    let result = match value {
        RefC::Const(i) => match observed.admission.view.constants[i] {
            ConstRow::U128 { n, .. } => n,
            _ => panic!("not UInt"),
        },
        RefC::Data(i) => match observed.image.data[i] {
            DataRow::U128 { n } => n,
            _ => panic!("not UInt"),
        },
    };
    assert_eq!(summary.result, result);
    println!("steps={count} result={result} carrier_mutations={mutations}");
}

#[test]
fn literal_stream_links_genesis_through_halted_result() {
    exercise(LITERAL, "provisional-host-config.json", 6, 7);
}

#[test]
fn addition_stream_links_genesis_through_halted_result() {
    exercise(ADD, "provisional-host-config.json", 20, 7);
}

#[test]
fn factorial_stream_links_genesis_through_halted_result() {
    exercise(FACT, "small-host-config.json", 293, 6);
}

#[test]
fn every_lane_in_an_uneven_poseidon_tree_is_bound() {
    // Cross two tree levels and a short final chunk; include a full-width UInt.
    let mut lanes: Vec<_> = (0u64..19).map(Fr::from).collect();
    lanes[18] = Fr::from(u128::MAX);
    let digest = frozen_state_digest(StateOutcome::RUNNING, hash_tree_offcircuit(&lanes));
    for i in 0..lanes.len() {
        let original = lanes[i];
        lanes[i] = lanes[i] + Fr::from(1u64);
        assert_ne!(
            digest,
            frozen_state_digest(StateOutcome::RUNNING, hash_tree_offcircuit(&lanes)),
            "lane {i}"
        );
        lanes[i] = original;
    }
}

#[test]
fn json_lines_are_lossless_public_carriers() {
    let source = "language \"moriarty-sherlock-draft/1\"; program P { entry 340282366920938463463374607431768211455; }";
    let mut bytes = Vec::new();
    let summary =
        write_witness_stream(source, &config("small-host-config.json"), &mut bytes).unwrap();
    assert_eq!(summary.result, u128::MAX);
    let text = String::from_utf8(bytes).unwrap();
    assert!(text.ends_with('\n'));
    let rows: Vec<serde_json::Value> = text
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    assert_eq!(rows.len(), 6);
    for (i, row) in rows.iter().enumerate() {
        assert_eq!(row["format"], C6_WITNESS_STREAM_FORMAT);
        assert_eq!(row["h_pre"], i.to_string());
        assert_eq!(row["prog_d"], field_hex(&summary.prog_d));
        assert_eq!(row["init_d"], field_hex(&summary.init_d));
        assert!(row["witness"]["pre"]["image"].is_array());
        assert!(row["witness"]["post"]["helper"].is_array());
        assert!(row.get("source").is_none());
        assert!(row.get("config_path").is_none());
        let w = &row["witness"];
        let read = |v: &serde_json::Value| -> Vec<u128> {
            v.as_array()
                .unwrap()
                .iter()
                .map(|s| s.as_str().unwrap().parse().unwrap())
                .collect()
        };
        let mut lanes = vec![
            u128::from(w["enable"].as_bool().unwrap()),
            w["host_schema"].as_str().unwrap().parse().unwrap(),
            w["carrier_version"].as_str().unwrap().parse().unwrap(),
        ];
        for section in ["meta", "prog", "ctx"] {
            lanes.extend(read(&w[section]));
        }
        let prog_lanes = lanes.len();
        for section in ["image", "data", "helper"] {
            lanes.extend(read(&w["pre"][section]));
        }
        let pre_lanes = lanes.len() - prog_lanes;
        for section in ["image", "data", "helper"] {
            lanes.extend(read(&w["post"][section]));
        }
        let sections = DigestSections {
            prog_lanes,
            pre_lanes,
            post_lanes: lanes.len() - prog_lanes - pre_lanes,
        };
        let post_outcome = if i == 5 {
            StateOutcome::halted(u128::MAX)
        } else {
            StateOutcome::RUNNING
        };
        let digests =
            statement_digests(&lanes, sections, StateOutcome::RUNNING, post_outcome).unwrap();
        for (name, digest) in ["prog_d", "pre_d", "post_d"].into_iter().zip(digests) {
            assert_eq!(row[name], field_hex(&digest));
        }
    }
    assert!(text.contains(&format!("\"{}\"", u128::MAX)));
    assert_eq!(
        rows.last().unwrap()["post_outcome"]["value"],
        u128::MAX.to_string()
    );
}

#[test]
fn bad_source_config_and_non_uint_terminal_are_errors() {
    let path = config("small-host-config.json");
    assert!(write_witness_stream("not a program", &path, &mut Vec::new()).is_err());
    assert!(write_witness_stream(LITERAL, &config("absent.json"), &mut Vec::new()).is_err());
    let boolean = "language \"moriarty-sherlock-draft/1\"; program P { entry true; }";
    assert!(write_witness_stream(boolean, &path, &mut Vec::new()).is_err());
}
