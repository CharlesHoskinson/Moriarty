//! Core-only R4 hostile/budget tests for the public `mori_core::json::parse_budgeted` API.
//!
//! Expectations are frozen in the fixture JSON files and were derived by hand
//! from the contract (root depth 0; one value-node per deserialize entry,
//! including the root, array elements and object member values, excluding
//! object keys; UTF-8 byte length; refusal strictly greater than the limit;
//! Nodes before Depth at one entry; byte precheck before decoding; duplicate
//! decoded key before that key's value; trailing check after a successful
//! root parse). Expected values are never computed by `parse_budgeted`.
//! Core returns a general `serde_json::Value`: scalar, array and object roots
//! are all valid here, and unknown object members are retained. Backend
//! object-root and typed-schema checks are out of scope for this file.

use mori_core::json::{parse_budgeted, BudgetedParseError, JsonBudgetKind, JsonParseBudget};
use serde_json::Value;
use std::collections::BTreeMap;

const WITNESSES: &str = include_str!("fixtures/core3_json_budget/WITNESSES.json");
const EXPECTATIONS: &str = include_str!("fixtures/core3_json_budget/EXPECTATIONS.json");

fn load(text: &str) -> Value {
    serde_json::from_str(text).expect("fixture JSON must parse")
}

fn field<'a>(value: &'a Value, key: &str) -> &'a Value {
    value.get(key).unwrap_or_else(|| panic!("missing field {key} in {value}"))
}

fn usize_field(value: &Value, key: &str) -> usize {
    let raw = field(value, key).as_u64().unwrap_or_else(|| panic!("field {key} must be an unsigned integer"));
    usize::try_from(raw).expect("fixture integer fits usize")
}

fn str_field<'a>(value: &'a Value, key: &str) -> &'a str {
    field(value, key).as_str().unwrap_or_else(|| panic!("field {key} must be a string"))
}

fn witnesses() -> BTreeMap<String, Value> {
    let doc = load(WITNESSES);
    let mut map = BTreeMap::new();
    for witness in field(&doc, "witnesses").as_array().expect("witnesses array") {
        map.insert(str_field(witness, "id").to_owned(), witness.clone());
    }
    map
}

fn budget_of(case: &Value) -> JsonParseBudget {
    let budget = field(case, "budget");
    JsonParseBudget {
        max_bytes: usize_field(budget, "maxBytes"),
        max_nodes: usize_field(budget, "maxNodes"),
        max_depth: usize_field(budget, "maxDepth"),
    }
}

fn kind_of(name: &str) -> JsonBudgetKind {
    match name {
        "Bytes" => JsonBudgetKind::Bytes,
        "Nodes" => JsonBudgetKind::Nodes,
        "Depth" => JsonBudgetKind::Depth,
        other => panic!("unknown budget kind {other}"),
    }
}

#[test]
fn witness_byte_counts_match_materialized_text() {
    for (id, witness) in witnesses() {
        let text = str_field(&witness, "text");
        assert_eq!(text.len(), usize_field(&witness, "utf8Bytes"), "witness {id}: UTF-8 byte count");
        assert!(!str_field(&witness, "derivation").is_empty(), "witness {id}: derivation must be present");
        // valueNodes / maxDepth are either both frozen numbers (valid witness)
        // or both null (refusal witness, counting stops at first failure).
        let nodes = field(&witness, "valueNodes");
        let depth = field(&witness, "maxDepth");
        assert_eq!(nodes.is_null(), depth.is_null(), "witness {id}: node/depth nullness must agree");
    }
}

#[test]
fn budgeted_cases_match_frozen_expectations() {
    let witness_map = witnesses();
    let doc = load(EXPECTATIONS);
    for case in field(&doc, "cases").as_array().expect("cases array") {
        let case_id = str_field(case, "id");
        let witness_id = str_field(case, "witness");
        let witness = witness_map.get(witness_id).unwrap_or_else(|| panic!("case {case_id}: unknown witness {witness_id}"));
        let text = str_field(witness, "text");
        let budget = budget_of(case);
        let expect = field(case, "expect");
        let result = parse_budgeted(text, budget);
        match str_field(expect, "kind") {
            "ok" => {
                let expected_value = field(expect, "value").clone();
                assert_eq!(result, Ok(expected_value), "case {case_id} ({witness_id}): expected Ok value");
            }
            "budget" => {
                let expected = BudgetedParseError::Budget {
                    kind: kind_of(str_field(expect, "budgetKind")),
                    limit: usize_field(expect, "limit"),
                };
                assert_eq!(result, Err(expected), "case {case_id} ({witness_id}): expected exact typed budget refusal");
            }
            "json" => {
                assert!(field(expect, "offset").is_null(), "case {case_id}: frozen offset must be null");
                assert!(field(expect, "step").is_null(), "case {case_id}: frozen step must be null");
                match result {
                    Err(BudgetedParseError::Json(error)) => {
                        assert_eq!(error.code, str_field(expect, "code"), "case {case_id}: exact error code");
                        assert_eq!(error.offset, None, "case {case_id}: offset must be None");
                        assert_eq!(error.step, None, "case {case_id}: step must be None");
                        let meaning = str_field(expect, "messageMeaning");
                        assert!(error.message.contains(meaning), "case {case_id}: message {:?} must carry contractual meaning {:?}", error.message, meaning);
                    }
                    other => panic!("case {case_id} ({witness_id}): expected Json error, got {other:?}"),
                }
            }
            other => panic!("case {case_id}: unknown expect kind {other}"),
        }
    }
}

#[test]
fn budget_display_names_typed_kind_and_limit() {
    // Display text is fixed by json.rs: "JSON parse budget exceeded: {kind:?} limit {limit}".
    let err = parse_budgeted("[1,2,3]", JsonParseBudget { max_bytes: 6, max_nodes: 99, max_depth: 99 }).unwrap_err();
    assert_eq!(err, BudgetedParseError::Budget { kind: JsonBudgetKind::Bytes, limit: 6 });
    assert_eq!(err.to_string(), "JSON parse budget exceeded: Bytes limit 6");
}
