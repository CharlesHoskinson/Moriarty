//! UIREQ-023: the hand-authored native relation table against the rules that
//! actual factorial and tree-fold owner runs exercise, and against the native
//! fragment files on disk.
//!
//! Fails when an exercised rule has no table row, when a row names a rule
//! neither program exercises, when a named fragment/consumer file, entry
//! function or rule name is absent, or when an unclaimed fragment file appears.
use std::collections::BTreeSet;
use std::path::PathBuf;

use mori_core::sherlock_compile_artifact::draft_execution::run_draft_host;
use mori_core::sherlock_compile_artifact::native_inventory::{
    COMPILED_AND_RUN, DISPATCH, OTHER_MACHINE, RECEIPT_DIR, RECEIPTED, RuleTally, SOURCE_ONLY,
    TABLE, TABLE_PROVENANCE, UNEXERCISED_C6, report,
};

const CONFIG: &str = include_str!("fixtures/technical-chunks/config.json");
const TARGET: &str = include_str!("fixtures/technical-chunks/target.json");

const FACT: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } }\n  entry fact(3);\n}\n";
const FOLD: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fold(t: Tree): UInt { match t { leaf(x) => x, branch(l, r) => fold(l) + fold(r) } }\n  entry fold(branch(leaf(4), leaf(5)));\n}\n";

fn tally(source: &str) -> RuleTally {
    let mut t = RuleTally::default();
    run_draft_host(source, CONFIG, TARGET, &mut |e| t.observe(e))
        .unwrap_or_else(|f| panic!("run failed: {}", serde_json::to_string(&f).unwrap()));
    t
}

fn fragment_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../mori-backend/src/sherlock_machine")
}

fn read(file: &str) -> String {
    std::fs::read_to_string(fragment_dir().join(file))
        .unwrap_or_else(|e| panic!("fragment file {file} missing: {e}"))
}

fn assert_entry(file: &str, entry: &str, names: &str) {
    let text = read(file);
    assert!(
        text.contains(&format!("pub fn {entry}(")),
        "{file} does not define pub fn {entry}"
    );
    assert!(text.contains(names), "{file} never names {names}");
}

#[test]
fn table_rows_are_exactly_the_exercised_rules() {
    let fact = tally(FACT);
    let fold = tally(FOLD);
    let exercised: BTreeSet<&str> = fact.rules().chain(fold.rules()).collect();
    let rows: BTreeSet<&str> = TABLE.iter().map(|r| r.rule).collect();
    assert_eq!(rows.len(), TABLE.len(), "duplicate table row");
    let missing: Vec<_> = exercised.difference(&rows).collect();
    let extra: Vec<_> = rows.difference(&exercised).collect();
    assert!(
        missing.is_empty(),
        "exercised rules without a row: {missing:?}"
    );
    assert!(
        extra.is_empty(),
        "rows for rules neither program exercises: {extra:?}"
    );
    // Hand-checked anchors: factorial compares and branches, fold matches.
    assert!(fact.rules().any(|r| r == "KMachineEvalIf"));
    assert!(!fact.rules().any(|r| r.starts_with("BHelperMatch")));
    assert!(fold.rules().any(|r| r == "BHelperMatchSelected"));
    assert!(!fold.rules().any(|r| r == "KMachineEvalIf"));
}

#[test]
fn every_named_fragment_exists_and_names_its_rule() {
    for row in TABLE {
        if let Some(f) = &row.fragment {
            assert_entry(f.file, f.entry, row.rule);
            if let Some(c) = f.consumer {
                read(c);
            }
        }
        for n in row.narrow {
            assert_entry(n.file, n.entry, n.names);
        }
    }
    for (rule, f) in UNEXERCISED_C6 {
        assert!(
            !TABLE.iter().any(|r| r.rule == *rule),
            "{rule} listed as unexercised but has a row"
        );
        assert_entry(f.file, f.entry, rule);
        if let Some(c) = f.consumer {
            read(c);
        }
    }
    for (file, rule, entry) in OTHER_MACHINE {
        assert_entry(file, entry, rule);
    }
    assert_entry(DISPATCH.file, DISPATCH.entry, "owner_select");
    read(DISPATCH.consumer.unwrap());
}

#[test]
fn no_unclaimed_fragment_file_appears() {
    let mut claimed: BTreeSet<String> = BTreeSet::new();
    for row in TABLE {
        if let Some(f) = &row.fragment {
            claimed.insert(f.file.into());
            claimed.extend(f.consumer.map(String::from));
        }
        claimed.extend(row.narrow.iter().map(|n| n.file.to_string()));
    }
    for (_, f) in UNEXERCISED_C6 {
        claimed.insert(f.file.into());
        claimed.extend(f.consumer.map(String::from));
    }
    claimed.extend(OTHER_MACHINE.iter().map(|(f, _, _)| f.to_string()));
    claimed.insert(DISPATCH.file.into());
    claimed.extend(DISPATCH.consumer.map(String::from));
    let mut unclaimed = Vec::new();
    for entry in std::fs::read_dir(fragment_dir()).expect("fragment dir") {
        let name = entry.unwrap().file_name().into_string().unwrap();
        if !name.ends_with(".rs") || name.ends_with("_tests.rs") {
            continue;
        }
        let fragment =
            name.ends_with("_step.rs") || name.ends_with("_consumer.rs") || name.starts_with("b_");
        if fragment && !claimed.contains(&name) {
            unclaimed.push(name);
        }
    }
    assert!(
        unclaimed.is_empty(),
        "unclaimed fragment files: {unclaimed:?}"
    );
}

#[test]
fn inventory_report_is_fixed_across_programs() {
    let fact = report("fact(3)", &tally(FACT));
    let fold = report("fold(branch(leaf(4), leaf(5)))", &tally(FOLD));
    for r in [&fact, &fold] {
        assert_eq!(r["format"], "moriarty-sherlock-native-inventory/1");
        assert_eq!(r["table_provenance"], TABLE_PROVENANCE);
        assert_eq!(r["table_provenance"], "hand-authored");
        assert_eq!(r["realization"]["kind"], "finite-family");
        assert_eq!(r["realization"]["universal"], false);
        assert_eq!(r["realization"]["per_program"], false);
        assert_eq!(r["prerequisites"]["verify_proof"], false);
        assert_eq!(r["prerequisites"]["ledger10"], false);
        assert_eq!(r["recursion_adapter"]["route"], "ledger-9 custom decider");
        assert_eq!(r["recursion_adapter"]["c6_fragments_wired"], false);
        assert_eq!(r["step_records"]["native_produces_step_records"], false);
        for owner in ["dispatch", "scalar", "frame", "data"] {
            assert!(r["owners"][owner].is_object(), "owner {owner} missing");
        }
        assert_eq!(r["summary"]["unlisted"].as_array().unwrap().len(), 0);
        // Honest: most exercised rules have no C6 fragment yet.
        assert_eq!(r["summary"]["complete"], false);
    }
    assert_eq!(fact["table_sha256"], fold["table_sha256"]);
    assert_eq!(fact["table"], fold["table"]);
    assert_eq!(fact["summary"]["rules"], 46);
    assert_eq!(fold["summary"]["rules"], 57);
    // Authored from the table: of the 46 fact rules, the 9 without a C6
    // fragment are BHelperEqual{Entry,Pair,Different,Exit},
    // BHelperValidate{Entry,Occurrence,Empty,Exit} and KAdminPublishValue.
    assert_eq!(fact["summary"]["with_c6_fragment"], 37);
    assert_eq!(fact["summary"]["gaps"].as_array().unwrap().len(), 9);
    // fold adds the 10 BHelperMatch rules and BHelperScalarConstruct as gaps.
    assert_eq!(fold["summary"]["with_c6_fragment"], 41);
    assert_eq!(fold["summary"]["gaps"].as_array().unwrap().len(), 16);
}

#[test]
fn receipted_fragments_match_committed_receipts() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let dir = root.join(RECEIPT_DIR);
    let mut on_disk: BTreeSet<String> = BTreeSet::new();
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries {
            let name = entry.unwrap().file_name().into_string().unwrap();
            if let Some(rule) = name.strip_suffix(".json") {
                on_disk.insert(rule.to_owned());
            }
        }
    }
    let listed: BTreeSet<String> = RECEIPTED.iter().map(|r| r.to_string()).collect();
    assert_eq!(
        listed, on_disk,
        "RECEIPTED and the committed receipt files differ"
    );
    let fact = report("fact(3)", &tally(FACT));
    let table = &fact["table"];
    let mut fragments: Vec<(String, serde_json::Value)> = Vec::new();
    for row in table["rows"].as_array().unwrap() {
        if !row["fragment"].is_null() {
            fragments.push((
                row["rule"].as_str().unwrap().into(),
                row["fragment"].clone(),
            ));
        }
    }
    for f in table["unexercised_c6_fragments"]
        .as_array()
        .unwrap()
        .iter()
        .chain(table["dispatch_fragments"].as_array().unwrap())
    {
        fragments.push((f["rule"].as_str().unwrap().into(), f.clone()));
    }
    for (rule, f) in &fragments {
        if listed.contains(rule) {
            assert_eq!(f["evidence"], COMPILED_AND_RUN, "{rule}");
            let path = format!("{RECEIPT_DIR}/{rule}.json");
            assert_eq!(f["receipt"], path.as_str(), "{rule}");
            let text = std::fs::read_to_string(root.join(&path)).unwrap();
            let rc: serde_json::Value = serde_json::from_str(&text).unwrap();
            assert_eq!(rc["format"], "moriarty-sherlock-fragment-receipt/1");
            assert_eq!(rc["rule"], rule.as_str());
            for k in ["file", "entry", "consumer"] {
                assert_eq!(rc[k], f[k], "{rule} receipt {k}");
            }
            assert_eq!(rc["failed"], 0);
        } else {
            assert_eq!(f["evidence"], SOURCE_ONLY, "{rule}");
            assert!(f.get("receipt").is_none(), "{rule}");
        }
    }
    let dispatch = &fact["owners"]["dispatch"];
    if listed.contains("dispatch") {
        assert_eq!(dispatch["native_selection"]["evidence"], COMPILED_AND_RUN);
        let sections = dispatch["section_constraints"].as_array().unwrap();
        assert!(
            sections
                .iter()
                .any(|s| s.as_str().unwrap().ends_with(DISPATCH.file))
        );
    } else {
        assert!(dispatch["native_selection"].is_null());
        assert!(dispatch["section_constraints"].is_null());
    }
}
