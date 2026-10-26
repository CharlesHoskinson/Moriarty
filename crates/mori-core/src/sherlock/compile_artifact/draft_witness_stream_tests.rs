//! G5 acceptance: the host witness stream over the real K-A machine.
//! Contiguity means each step's post-state is the next step's pre-state,
//! at both the digest level (`post_d`/`pre_d`) and the canonical
//! snapshot level; the stream ends in the terminal publication.
use super::*;
use serde_json::Value;

const CONFIG: &str = include_str!("../../../tests/fixtures/draft-compile/config.json");
const TARGET: &str = include_str!("../../../tests/fixtures/draft-compile/target.json");

const FACT3: &str = "language \"moriarty-sherlock-draft/1\";\n\
program Fact3 {\n\
  fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } }\n\
  entry fact(3);\n\
}\n";

const LIST123: &str = "language \"moriarty-sherlock-draft/1\";\n\
program List123 {\n\
  fn sum(xs: List): UInt { match xs { nil => 0, cons(h, t) => h + sum(t) } }\n\
  entry sum(cons(1, cons(2, cons(3, nil))));\n\
}\n";

fn stream(source: &str) -> Value {
    // The shared fixture caps fuel at 64, below fact(3)'s step count; the
    // qualification runs use a larger fuel bound (and fuel <= value_slots).
    let mut config: Value = serde_json::from_str(CONFIG).expect("fixture config parses");
    config["fuel"]["value"] = Value::String("1000".to_owned());
    config["stack"]["value"] = Value::String("64".to_owned());
    witness_stream(source, &config.to_string(), TARGET).unwrap_or_else(|failure| {
        panic!(
            "witness stream failed at {}: {}",
            failure.stage, failure.actual_error
        )
    })
}

fn assert_contiguous_halts_at_six(stream: &Value) {
    assert_eq!(stream["format"], "moriarty-sherlock-witness-stream/1");
    let steps = stream["steps"].as_array().expect("steps array");
    assert!(!steps.is_empty(), "stream must contain steps");
    for (index, step) in steps.iter().enumerate() {
        assert_eq!(step["index"], index.to_string());
        assert!(step["rule"].is_string(), "step {index} names its rule");
        assert_eq!(step["pre_d"], step["pre"]["digest"], "step {index} pre_d");
        assert_eq!(
            step["post_d"], step["post"]["digest"],
            "step {index} post_d"
        );
        if index + 1 < steps.len() {
            assert_eq!(
                step["post_d"],
                steps[index + 1]["pre_d"],
                "step {index} post_d must equal next pre_d"
            );
            assert_eq!(
                step["post"],
                steps[index + 1]["pre"],
                "step {index} post state must equal next pre state"
            );
        }
    }
    let last = steps.last().expect("last step");
    assert_eq!(last["rule"], "PublishDone");
    assert_eq!(stream["final"]["outcome"]["kind"], "Done");
    assert_eq!(stream["final"]["result"], "uiU(6)");
    assert_eq!(stream["final"]["post_d"], last["post_d"]);
    let rules = stream["rules_exercised"].as_array().expect("rules list");
    assert!(
        rules.iter().any(|rule| rule == "PublishDone"),
        "exercised rules must include PublishDone"
    );
}

#[test]
fn fact3_stream_is_contiguous_and_halts_at_six() {
    let stream = stream(FACT3);
    assert_contiguous_halts_at_six(&stream);
    let rules = stream["rules_exercised"].as_array().expect("rules list");
    assert!(
        rules.iter().any(|rule| rule == "Invoke"),
        "fact3 must exercise Invoke, got {rules:?}"
    );
}

#[test]
fn list123_stream_is_contiguous_and_halts_at_six() {
    let stream = stream(LIST123);
    assert_contiguous_halts_at_six(&stream);
    let rules = stream["rules_exercised"].as_array().expect("rules list");
    assert!(
        rules.iter().any(|rule| rule == "CaseHit"),
        "list123 must exercise CaseHit, got {rules:?}"
    );
}
