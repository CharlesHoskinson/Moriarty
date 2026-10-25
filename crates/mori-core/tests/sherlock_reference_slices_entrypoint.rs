//! UIREQ-032 public entrypoint: earliest-stop K-A ReferenceSlices of a draft
//! source grouped by 1, 2, 4 and 8 events around a pending callee return.
//!
//! Hand-authored expectations: fact(2) is 2; publication is free (K rule 80,
//! charge 0) so fuel equal to the reference work still publishes Done while one
//! less exhausts; a stack limit of 1 rejects with frame-cap error 4. Grouping
//! must not change the ordered record stream, charges or outcome.
use mori_core::sherlock_compile_artifact::draft_reference_slices::reference_slices;
use serde_json::Value;

const CONFIG: &str = include_str!("fixtures/technical-chunks/config.json");
const TARGET: &str = include_str!("fixtures/technical-chunks/target.json");
const FACT: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } }\n  entry fact(2);\n}\n";

fn config(fuel: Option<u128>, stack: Option<u128>) -> String {
    let mut c: Value = serde_json::from_str(CONFIG).unwrap();
    if let Some(f) = fuel {
        c["fuel"]["value"] = Value::String(f.to_string());
    }
    if let Some(s) = stack {
        c["stack"]["value"] = Value::String(s.to_string());
    }
    c.to_string()
}

fn slices(m: usize, fuel: Option<u128>, stack: Option<u128>) -> Value {
    reference_slices(FACT, &config(fuel, stack), TARGET, m)
        .unwrap_or_else(|f| panic!("{}", serde_json::to_string(&f).unwrap()))
}

fn n(v: &Value) -> u128 {
    v.as_str().unwrap().parse().unwrap()
}

/// The flattened (rule line, charge, depth after, outcome after) stream.
fn stream(r: &Value) -> Vec<(String, String, String, String)> {
    r["slices"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|s| s["records"].as_array().unwrap().clone())
        .map(|x| {
            (
                x["rule_start_line"].as_str().unwrap().to_owned(),
                x["charge"].as_str().unwrap().to_owned(),
                x["frame_check"]["depth_after"].as_str().unwrap().to_owned(),
                x["outcome_after"].to_string(),
            )
        })
        .collect()
}

fn check_grouping(r: &Value, m: usize) {
    let list = r["slices"].as_array().unwrap();
    let mut position = 0u128;
    let mut work = 0u128;
    for (i, s) in list.iter().enumerate() {
        let records = s["records"].as_array().unwrap();
        assert!(!records.is_empty() && records.len() <= m, "slice {i}");
        let mut prefix = 0u128;
        for (j, x) in records.iter().enumerate() {
            position += 1;
            prefix += n(&x["charge"]);
            work += n(&x["charge"]);
            assert_eq!(n(&x["position"]), position);
            assert_eq!(n(&x["internal_position"]), j as u128 + 1);
            assert_eq!(n(&x["prefix_work"]), prefix);
            assert_eq!(n(&x["cumulative_work"]), work);
            assert_eq!(x["frame_check"]["within_limit"], true);
            // Earliest stop: only the very last record may publish, and it is first.
            let published = x["outcome_after"]["kind"] != "Pending";
            assert_eq!(published, x["first_outcome"] == true);
            if published {
                assert_eq!(i + 1, list.len());
                assert_eq!(j + 1, records.len());
            }
        }
        let last = i + 1 == list.len();
        assert_eq!(
            s["stop"]["kind"],
            if last { "outcome" } else { "technical-cut" },
            "slice {i}"
        );
        if !last {
            assert_eq!(s["stop"]["financial_commit"], "none");
            // The next slice resumes from this slice's complete end state.
            assert_eq!(list[i + 1]["start"], s["end"]);
        }
        assert_eq!(s["padding"]["reference_records"], "0");
    }
    assert_eq!(n(&r["events"]), position);
    assert_eq!(n(&r["reference_work"]), work);
    assert_eq!(r["financial_commits"], "0");
    assert_eq!(r["reference_schedule_id"], "sherlock-k-a/1");
}

#[test]
fn grouped_slices_preserve_the_reference_stream_for_1_2_4_8() {
    let base = slices(1, None, None);
    assert_eq!(base["outcome"]["kind"], "Done");
    assert_eq!(base["outcome"]["value"], "uiU(2)");
    let want = stream(&base);
    // The run passes through a callee return into a pending caller frame.
    assert!(
        base["slices"].as_array().unwrap().iter().any(|s| {
            let end = &s["end"];
            end["control"]
                .as_str()
                .is_some_and(|c| c.starts_with("uiRet("))
                && n(&end["logical_depth"]) > 0
        }),
        "no pending-return boundary"
    );
    for m in [1, 2, 4, 8] {
        let r = slices(m, None, None);
        check_grouping(&r, m);
        assert_eq!(stream(&r), want, "m={m}");
        assert_eq!(r["outcome"], base["outcome"], "m={m}");
        let cumulative = &r["cumulative"]["observations"];
        assert_eq!(n(&cumulative["reference_work"]), n(&base["reference_work"]));
    }
}

#[test]
fn zero_fuel_publication_and_frame_cap_keep_their_precedence() {
    let work = n(&slices(1, None, None)["reference_work"]);
    for m in [1, 2, 4, 8] {
        let exact = slices(m, Some(work), None);
        assert_eq!(exact["outcome"]["kind"], "Done", "m={m}");
        let last = exact["slices"].as_array().unwrap().last().unwrap()["records"]
            .as_array()
            .unwrap()
            .last()
            .unwrap()
            .clone();
        assert_eq!(last["rule_start_line"], "80");
        assert_eq!(last["charge"], "0");
        assert_eq!(last["first_outcome"], true);
        let short = slices(m, Some(work - 1), None);
        assert_eq!(short["outcome"]["kind"], "Exhausted", "m={m}");
        let capped = slices(m, None, Some(1));
        assert_eq!(capped["outcome"]["kind"], "Rejected", "m={m}");
        assert_eq!(capped["outcome"]["code"], "4", "m={m}");
        assert_eq!(
            stream(&capped),
            stream(&slices(1, None, Some(1))),
            "frame-cap stream m={m}"
        );
    }
}

#[test]
fn zero_group_budget_is_refused() {
    assert!(reference_slices(FACT, CONFIG, TARGET, 0).is_err());
}
