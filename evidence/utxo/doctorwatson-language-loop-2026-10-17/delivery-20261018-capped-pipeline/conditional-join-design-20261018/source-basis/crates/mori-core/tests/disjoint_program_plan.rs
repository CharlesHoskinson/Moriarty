#![cfg(feature = "core2-disjoint-draft")]

use mori_core::{intent, stage};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};

#[test]
fn complete_child_programs_and_costed_plans_match_independent_literals() {
    let fixtures =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../fixtures/core-disjoint-product");
    let expected: Vec<_> = ["two-deliveries", "reserve"]
        .into_iter()
        .map(|name| {
            let program: Value = serde_json::from_slice(
                &fs::read(fixtures.join(format!("{name}-program.json"))).unwrap(),
            )
            .unwrap();
            let plan: Value = serde_json::from_slice(
                &fs::read(fixtures.join(format!("{name}-plan.json"))).unwrap(),
            )
            .unwrap();
            let program_preimage =
                fs::read(fixtures.join(format!("{name}-program.preimage.bin"))).unwrap();
            let plan_preimage =
                fs::read(fixtures.join(format!("{name}-plan.preimage.bin"))).unwrap();
            (name, program, plan, program_preimage, plan_preimage)
        })
        .collect();

    // Whole independent objects include unused actions and all expression nodes.
    for (name, program, plan, program_preimage, plan_preimage) in expected {
        let source = fs::read_to_string(fixtures.join(format!("{name}.mori"))).unwrap();
        let actual = stage::load(&source).unwrap();
        assert_eq!(
            serde_json::to_value(actual.program()).unwrap(),
            program,
            "{name} Program"
        );
        assert_eq!(
            serde_json::to_value(actual.plan()).unwrap(),
            plan,
            "{name} Plan"
        );
        assert_eq!(
            intent::canonical(actual.program()).unwrap(),
            program_preimage,
            "{name} Program preimage"
        );
        assert_eq!(
            intent::canonical(actual.plan()).unwrap(),
            plan_preimage,
            "{name} Plan preimage"
        );
        let program_hash: [u8; 32] = Sha256::digest(&program_preimage).into();
        let plan_hash: [u8; 32] = Sha256::digest(&plan_preimage).into();
        assert_eq!(
            actual.program_hash(),
            program_hash,
            "{name} local Program digest"
        );
        assert_eq!(actual.plan_hash(), plan_hash, "{name} local Plan digest");
    }
}
