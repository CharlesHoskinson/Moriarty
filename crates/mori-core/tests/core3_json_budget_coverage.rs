//! Additive Core-only coverage for `mori_core::json` contracts not covered
//! by `core3_json_budget.rs` (which is preserved unchanged: 24 witnesses,
//! C01-C38, 3 test functions). Every expectation below is derived by hand
//! from `json.rs`, the Core `Error` type, and the pinned serde_json
//! 1.0.151 sources (`de.rs`, `read.rs`, `error.rs`, `number.rs`): root
//! depth 0, one node per value entry excluding object keys, inclusive
//! limits, Nodes before Depth at one entry, byte precheck first,
//! duplicate decoded key before its value, trailing check after the root.
//! Status: UNRUN. Expected values are never computed by the parser under
//! test; large inputs use the stated deterministic constructors.

use mori_core::json::{
    deserialize_strict, parse, parse_budgeted, parse_limited, BudgetedParseError, JsonBudgetKind,
    JsonParseBudget,
};
use serde_json::{json, Value};

const BIG: JsonParseBudget = JsonParseBudget {
    max_bytes: usize::MAX,
    max_nodes: usize::MAX,
    max_depth: usize::MAX,
};

fn budget(max_bytes: usize, max_nodes: usize, max_depth: usize) -> JsonParseBudget {
    JsonParseBudget { max_bytes, max_nodes, max_depth }
}

/// `[` * k + `1` + `]` * k: bytes 2k+1, nodes k+1, max BudgetSeed depth k,
/// serde_json containers k.
fn nested_arrays(k: usize) -> String {
    let mut s = "[".repeat(k);
    s.push('1');
    s.push_str(&"]".repeat(k));
    s
}

fn nested_array_value(k: usize) -> Value {
    let mut v = json!(1);
    for _ in 0..k {
        v = Value::Array(vec![v]);
    }
    v
}

/// `[0,0,...]` with n elements: nodes n+1, max depth 1,
/// bytes 1 + 2n - 1 + 1 = 2n+1 for n >= 1.
fn flat_array(n: usize) -> String {
    let mut s = String::from("[");
    for i in 0..n {
        if i > 0 {
            s.push(',');
        }
        s.push('0');
    }
    s.push(']');
    s
}

fn assert_json_meaning(text: &str, b: JsonParseBudget, meaning: &str) {
    match parse_budgeted(text, b) {
        Err(BudgetedParseError::Json(e)) => {
            assert_eq!(e.code, "mori::core::json", "text {text:?}: exact code");
            assert_eq!(e.offset, None, "text {text:?}: offset");
            assert_eq!(e.step, None, "text {text:?}: step");
            assert!(
                e.message.contains(meaning),
                "text {text:?}: message {:?} must contain {meaning:?}",
                e.message
            );
        }
        other => panic!("text {text:?}: expected Json error containing {meaning:?}, got {other:?}"),
    }
}

fn assert_budget(text: &str, b: JsonParseBudget, kind: JsonBudgetKind, limit: usize) {
    assert_eq!(
        parse_budgeted(text, b),
        Err(BudgetedParseError::Budget { kind, limit }),
        "text {text:?}: exact typed budget refusal"
    );
}

#[test]
fn fractions_and_exponents_are_finite_f64_values() {
    // de.rs: '.'/'e' routes parse as f64; Number stores finite f64 only.
    assert_eq!(parse_budgeted("1.5", BIG), Ok(json!(1.5)));
    assert_eq!(parse_budgeted("-0.25", BIG), Ok(json!(-0.25)));
    assert_eq!(parse_budgeted("0.1", BIG), Ok(json!(0.1)));
    assert_eq!(parse_budgeted("1e2", BIG), Ok(json!(100.0)));
    assert_eq!(parse_budgeted("1E-3", BIG), Ok(json!(0.001)));
    assert_eq!(parse_budgeted("2.5e-1", BIG), Ok(json!(0.25)));
    assert_eq!(parse_budgeted("-1.5e+2", BIG), Ok(json!(-150.0)));
    assert_eq!(parse_budgeted("1e308", BIG).unwrap().as_f64(), Some(1e308));
    // Exponent underflow is finite 0.0, not an error (f64_from_parts).
    assert_eq!(parse_budgeted("1e-999", BIG), Ok(json!(0.0)));
    assert_eq!(parse_budgeted("0e999", BIG), Ok(json!(0.0)));
    let neg_zero = parse_budgeted("-0.0", BIG).unwrap();
    assert_eq!(neg_zero.as_f64().map(f64::is_sign_negative), Some(true));
}

#[test]
fn out_of_range_numbers_are_json_refusals_not_budget_or_nonfinite() {
    // f64_from_parts: infinite result -> NumberOutOfRange before visit_f64.
    assert_json_meaning("1e309", BIG, "number out of range");
    assert_json_meaning("-1e309", BIG, "number out of range");
    let huge = format!("1{}", "0".repeat(400));
    assert_json_meaning(&huge, BIG, "number out of range");
    // Budget refusal still pre-empts an out-of-range body.
    assert_budget("1e309", budget(4, 99, 99), JsonBudgetKind::Bytes, 4);
}

#[test]
fn integer_boundaries_follow_u64_i64_then_f64_derivation() {
    let u64_max = parse_budgeted("18446744073709551615", BIG).unwrap();
    assert_eq!(u64_max.as_u64(), Some(u64::MAX));
    assert!(!u64_max.is_f64());
    let i64_max = parse_budgeted("9223372036854775807", BIG).unwrap();
    assert_eq!(i64_max.as_i64(), Some(i64::MAX));
    let i64_min = parse_budgeted("-9223372036854775808", BIG).unwrap();
    assert_eq!(i64_min.as_i64(), Some(i64::MIN));
    // One past u64::MAX: significand overflow switches to finite f64 (2^64).
    let past = parse_budgeted("18446744073709551616", BIG).unwrap();
    assert!(past.is_f64());
    assert_eq!(past.as_f64(), Some(18446744073709551616.0));
    assert_eq!(past.as_u64(), None);
    let past_neg = parse_budgeted("-18446744073709551616", BIG).unwrap();
    assert_eq!(past_neg.as_f64(), Some(-18446744073709551616.0));
    // serde_json parse_number: -0 has significand 0 and no fraction or
    // exponent, so it routes to F64(-0.0), not U64/I64(0).
    let neg_zero_int = parse_budgeted("-0", BIG).unwrap();
    assert!(neg_zero_int.is_f64());
    assert_eq!(neg_zero_int.as_f64().map(f64::is_sign_negative), Some(true));
    assert_eq!(neg_zero_int, json!(-0.0));
    assert_eq!(parse_budgeted("0", BIG).unwrap().as_u64(), Some(0));
}

#[test]
fn malformed_number_syntax_meanings() {
    assert_json_meaning("01", BIG, "invalid number");
    // Truncated at EOF: serde_json distinguishes EOF from a present
    // invalid character (parse_decimal/parse_exponent/parse_integer).
    assert_json_meaning("1.", BIG, "EOF while parsing a value");
    assert_json_meaning("1.x", BIG, "invalid number");
    assert_json_meaning("1e", BIG, "EOF while parsing a value");
    assert_json_meaning("1eX", BIG, "invalid number");
    assert_json_meaning("1e+", BIG, "EOF while parsing a value");
    assert_json_meaning("1e+X", BIG, "invalid number");
    assert_json_meaning("+1", BIG, "expected value");
    assert_json_meaning(".5", BIG, "expected value");
    assert_json_meaning("NaN", BIG, "expected value");
    assert_json_meaning("Infinity", BIG, "expected value");
    assert_json_meaning("-", BIG, "EOF while parsing a value");
    assert_json_meaning("-x", BIG, "invalid number");
    assert_json_meaning("0x10", BIG, "trailing characters");
}

#[test]
fn unicode_surrogates_controls_and_escapes() {
    // Valid surrogate pair U+1F600 and BMP escapes (read.rs pair decoding).
    assert_eq!(parse_budgeted(r#""\uD83D\uDE00""#, BIG), Ok(json!("\u{1F600}")));
    assert_eq!(parse_budgeted(r#""\u0041\u00e9""#, BIG), Ok(json!("A\u{e9}")));
    assert_eq!(parse_budgeted(r#""a\nb\t\\\"""#, BIG), Ok(json!("a\nb\t\\\"")));
    // Lone / unpaired surrogates refuse on the validate=true str path.
    assert_json_meaning(r#""\uDE00""#, BIG, "lone leading surrogate in hex escape");
    assert_json_meaning(r#""\uD83D""#, BIG, "unexpected end of hex escape");
    assert_json_meaning(r#""\uD83D\u0041""#, BIG, "lone leading surrogate in hex escape");
    assert_json_meaning(r#""\uD83Dx""#, BIG, "unexpected end of hex escape");
    // Malformed escapes and raw control characters.
    assert_json_meaning(r#""\x""#, BIG, "invalid escape");
    assert_json_meaning(r#""\u00G0""#, BIG, "invalid escape");
    assert_json_meaning(r#""\u041""#, BIG, "invalid escape");
    assert_json_meaning("\"a\u{0001}b\"", BIG, "control character");
    assert_json_meaning("\"a\nb\"", BIG, "control character");
}

#[test]
fn decoded_key_equality_and_distinct_strings() {
    // No normalisation: precomposed U+00E9 != e + combining U+0301.
    assert_eq!(
        parse_budgeted(r#"{"\u00e9":1,"e\u0301":2}"#, BIG),
        Ok(json!({"\u{e9}": 1, "e\u{301}": 2}))
    );
    // The preserved original literal appends one '"' after the complete
    // object, so the root succeeds and decoder.end() refuses it as trailing.
    assert_json_meaning(r#"{"\u00e9":1,"e\u0301":2}""#, BIG, "trailing characters");
    assert_ne!(json!("\u{e9}"), json!("e\u{301}"));
    // Keys equal only after decoding are duplicates, before their value:
    // max_nodes 2 admits root + first value; the duplicate key still wins.
    assert_json_meaning(r#"{"\u0061":1,"a":2}""#, budget(99, 2, 99), "duplicate decoded key");
    assert_json_meaning(r#"{"a":1,"\u0061":2}""#, BIG, "duplicate decoded key");
    // Same decoded key in sibling (non-nested) objects is not a duplicate.
    assert_eq!(
        parse_budgeted(r#"[{"x":1},{"x":2}]"#, BIG),
        Ok(json!([{ "x": 1 }, { "x": 2 }]))
    );
    // Preserved original extra-quote form: trailing after the root array.
    assert_json_meaning(r#"[{"x":1},{"x":2}]""#, BIG, "trailing characters");
    // Distinct scalar strings stay distinct values.
    assert_eq!(parse_budgeted(r#"["\u00e9","e\u0301"]"#, BIG), Ok(json!(["\u{e9}", "e\u{301}"])));
    // Preserved original extra-quote form: trailing after the root array.
    assert_json_meaning(r#"["\u00e9","e\u0301"]""#, BIG, "trailing characters");
}

#[test]
fn malformed_structure_meanings_and_trailing_whitespace() {
    assert_json_meaning("{\"a\" 1}", BIG, "expected `:`");
    assert_json_meaning("{\"a\":1 \"b\":2}", BIG, "expected `,` or `}`");
    assert_json_meaning("[1 2]", BIG, "expected `,` or `]`");
    assert_json_meaning("{\"a\":1,}", BIG, "trailing comma");
    assert_json_meaning("{,}", BIG, "key must be a string");
    assert_json_meaning("[1,2", BIG, "EOF while parsing a list");
    assert_json_meaning("{\"a\":1", BIG, "EOF while parsing an object");
    assert_json_meaning("\"abc", BIG, "EOF while parsing a string");
    assert_json_meaning("tru", BIG, "EOF while parsing a value");
    assert_json_meaning("nulL", BIG, "expected ident");
    // JSON whitespace (space/tab/CR/LF) around the root is accepted;
    // any other trailing content is refused after the root.
    assert_eq!(parse_budgeted(" \t\r\n[1] \t\r\n", BIG), Ok(json!([1])));
    assert_eq!(parse_budgeted("{\"a\":1}   ", BIG), Ok(json!({"a": 1})));
    assert_json_meaning("[1] x", BIG, "trailing characters");
    assert_json_meaning("truefalse", BIG, "trailing characters");
}

#[test]
fn serde_recursion_guard_counts_containers_not_budget_depth() {
    // 127 nested containers: serde remaining_depth 128 -> 1, passes.
    // BudgetSeed scalar depth is also 127; nodes 128; bytes 255.
    let ok_text = nested_arrays(127);
    assert_eq!(ok_text.len(), 255);
    assert_eq!(
        parse_budgeted(&ok_text, budget(255, 128, 127)),
        Ok(nested_array_value(127))
    );
    // 128 nested containers: BudgetSeed would admit depth 128 / 129 nodes,
    // but serde refuses the 128th container. Json, never Budget Depth.
    let deep = nested_arrays(128);
    assert_eq!(deep.len(), 257);
    assert_json_meaning(&deep, BIG, "recursion limit exceeded");
    assert_json_meaning(&deep, budget(usize::MAX, usize::MAX, 128), "recursion limit exceeded");
    // At max_depth 127 the depth-127 container entry passes BudgetSeed
    // (127 is not > 127) and serde still refuses that container.
    assert_json_meaning(&deep, budget(usize::MAX, usize::MAX, 127), "recursion limit exceeded");
    // At max_depth 126 BudgetSeed Depth pre-empts serde at depth 127.
    assert_budget(&deep, budget(usize::MAX, usize::MAX, 126), JsonBudgetKind::Depth, 126);
    // Nodes still pre-empts both guards at a single entry.
    assert_budget(&deep, budget(usize::MAX, 3, usize::MAX), JsonBudgetKind::Nodes, 3);
    // Alternating object/array containers count once each in both schemes:
    // {"a":[{"a":1}]} has BudgetSeed nodes 4 (root, array, object, scalar)
    // wait: root object, member array, member object, member scalar = 4
    // nodes, max depth 3, serde containers 3.
    assert_eq!(
        parse_budgeted(r#"{"a":[{"a":1}]}"#, budget(99, 4, 3)),
        Ok(json!({"a": [{"a": 1}]}))
    );
    // Preserved original extra-quote form: trailing after the root object.
    assert_json_meaning(r#"{"a":[{"a":1}]}""#, BIG, "trailing characters");
    assert_budget(r#"{"a":[{"a":1}]}""#, budget(99, 99, 2), JsonBudgetKind::Depth, 2);
}

#[test]
fn budgeted_wide_and_zero_budgets() {
    // Wide flat array: 1000 elements, nodes 1001, bytes 2001, depth 1.
    let wide = flat_array(1000);
    assert_eq!(wide.len(), 2001);
    assert!(parse_budgeted(&wide, budget(2001, 1001, 1)).is_ok());
    assert_budget(&wide, budget(2000, 1001, 1), JsonBudgetKind::Bytes, 2000);
    assert_budget(&wide, budget(2001, 1000, 1), JsonBudgetKind::Nodes, 1000);
    assert_budget(&wide, budget(2001, 1001, 0), JsonBudgetKind::Depth, 0);
    // Zero budgets refuse even an empty-container root as Nodes first.
    assert_budget("[]", budget(99, 0, 0), JsonBudgetKind::Nodes, 0);
    assert_budget("[]", budget(1, 99, 99), JsonBudgetKind::Bytes, 1);
    assert_eq!(parse_budgeted("[]", budget(2, 1, 0)), Ok(json!([])));
}

#[test]
fn legacy_parse_depth_node_and_byte_boundaries() {
    // Legacy Seed: depth > 32 refused. 32 containers + scalar: scalar at
    // depth 32 admitted; 33 containers: scalar at depth 33 refused.
    assert!(parse(&nested_arrays(32)).is_ok());
    let deep_err = parse(&nested_arrays(33)).unwrap_err();
    assert_eq!(deep_err.code, "mori::core::json");
    assert!(deep_err.message.contains("JSON work/depth limit"), "{:?}", deep_err.message);
    // Nodes > 65_536 refused: root + 65_535 elements = 65_536 admitted;
    // root + 65_536 elements = 65_537 refused.
    assert!(parse(&flat_array(65_535)).is_ok());
    let node_err = parse(&flat_array(65_536)).unwrap_err();
    assert_eq!(node_err.code, "mori::core::json");
    assert!(node_err.message.contains("JSON work/depth limit"), "{:?}", node_err.message);
    // Bytes: exactly 1 MiB admitted (quoted string of 1_048_574 'a');
    // one more byte gives the exact legacy message.
    let exact = format!("\"{}\"", "a".repeat(1_048_574));
    assert_eq!(exact.len(), 1_048_576);
    assert!(parse(&exact).is_ok());
    let over = format!("\"{}\"", "a".repeat(1_048_575));
    let byte_err = parse(&over).unwrap_err();
    assert_eq!(byte_err.code, "mori::core::json");
    assert_eq!(byte_err.message, "JSON exceeds 1 MiB");
    // parse_budgeted has no hidden 1 MiB / 32 / 65_536 legacy limits.
    assert!(parse_budgeted(&over, BIG).is_ok());
    assert!(parse_budgeted(&nested_arrays(33), BIG).is_ok());
    assert!(parse_budgeted(&flat_array(65_536), BIG).is_ok());
}

#[test]
fn legacy_parse_limited_and_deserialize_strict_are_unchanged() {
    // parse_limited changes only the byte ceiling and its message.
    assert!(parse_limited("[1]", 3, "too large").is_ok());
    let e = parse_limited("[1]", 2, "too large").unwrap_err();
    assert_eq!(e.code, "mori::core::json");
    assert_eq!(e.message, "too large");
    // Its depth bound is still the legacy 32 even with a huge byte ceiling.
    assert!(parse_limited(&nested_arrays(33), usize::MAX, "too large").is_err());
    // deserialize_strict applies duplicate/depth/node Seed bounds through
    // a direct Deserializer; it performs no trailing end() check itself.
    let mut de = serde_json::Deserializer::from_str(r#"{"x":1,"\u0078":2}""#);
    assert!(deserialize_strict(&mut de).is_err());
    let mut de = serde_json::Deserializer::from_str("[[1]]");
    assert_eq!(deserialize_strict(&mut de).unwrap(), json!([[1]]));
    let binding = nested_arrays(33);
    let mut de = serde_json::Deserializer::from_str(&binding);
    assert!(deserialize_strict(&mut de).is_err());
}
