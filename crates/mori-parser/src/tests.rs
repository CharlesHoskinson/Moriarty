use std::fmt::Write;
use std::path::Path;

use mori_ast::{Ast, NodeIdx};

use mori_diagnostics::Diagnostic;
use mori_diagnostics::render::render_plain;

use crate::{MAX_DEPTH, MAX_FIELDS, MAX_ITEMS, parse};

/// Parses the source and snapshots the printed tree.
macro_rules! assert_parse_snapshot {
    ($source:literal) => {{
        let source = indoc::indoc!($source);
        let ast = parse(source).unwrap_or_else(|error| panic!("expected a tree, got: {error}"));
        insta::with_settings!({
            description => format!("Code:\n\n{source}"),
            omit_expression => true,
        }, {
            insta::assert_snapshot!(ast.print());
        });
    }};
}

/// Parses the source and snapshots the rendered error.
macro_rules! assert_parse_error_snapshot {
    ($source:literal) => {{
        let source = indoc::indoc!($source);
        let error = parse(source).expect_err("expected a parse error");
        insta::with_settings!({
            description => format!("Code:\n\n{source}"),
            omit_expression => true,
        }, {
            insta::assert_snapshot!(render_plain(&error, "test.mori", source));
        });
    }};
}

/// Parses the source and snapshots every node's span text, in tree order.
macro_rules! assert_spans_snapshot {
    ($source:literal) => {{
        let source = indoc::indoc!($source);
        let ast = parse(source).unwrap_or_else(|error| panic!("expected a tree, got: {error}"));
        insta::with_settings!({
            description => format!("Code:\n\n{source}"),
            omit_expression => true,
        }, {
            insta::assert_snapshot!(spans(&ast));
        });
    }};
}

fn spans(ast: &Ast<'_>) -> String {
    fn visit(ast: &Ast<'_>, node: NodeIdx, depth: usize, out: &mut String) {
        let tag = format!("{:?}", ast.tag(node));
        let text = ast.span(node).source_text(ast.source);
        writeln!(out, "{}{tag:<12} {text:?}", "  ".repeat(depth)).unwrap();
        for child in ast.children(node) {
            visit(ast, child, depth + 1, out);
        }
    }
    let mut out = String::new();
    visit(ast, NodeIdx::ROOT, 0, &mut out);
    out
}

fn error_code(source: &str) -> Option<String> {
    let error = parse(source).err()?;
    Some(error.code().expect("parse errors have codes").to_string())
}

/// Wraps declarations in a minimal agreement.
fn agreement(body: &str) -> String {
    format!("profile \"moriarty-beta/1\";\nagreement Test {{\n{body}\n}}\n")
}

// Trees

#[test]
fn empty_agreement() {
    assert_parse_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Empty {}
        "#
    );
}

#[test]
fn declarations_and_action() {
    assert_parse_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          const price: Qty<USD> = 10.00 USD;
          const memo: String = "thanks";
          const ok = true;
          const failure = SuccessOnly;
          const nothing = None;
          action pay uses invoice;
        }
        "#
    );
}

#[test]
fn operator_precedence_and_associativity() {
    assert_parse_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Math {
          const a = 1 + 2 * 3 - 4;
          const b = (1 + 2) * 3;
          const c = 10 - 2 - 3;
          const d = 2 * 3 * 4;
        }
        "#
    );
}

#[test]
fn lists_records_and_calls() {
    assert_parse_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Shapes {
          const empty_list = [];
          const assets = [USD, GOLD,];
          const empty_record = {};
          const nested = { inner: { value: 1, }, list: [1, 2] };
          const valid = rounds(domain: Preview, from: 0, to: 10);
          const swap = amm.swap_exact_input(pool: pool, input: 5 USD,);
          const amount = atoms(asset: USD, value: 1_000);
        }
        "#
    );
}

#[test]
fn nested_type_annotations() {
    assert_parse_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Types {
          const a: Account<Preview> = alice;
          const b: Map<String, List<Qty<USD>>> = c;
        }
        "#
    );
}

#[test]
fn keywords_as_field_names() {
    assert_parse_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Fields {
          account alice = { domain: Preview, id: "alice" };
          const r = { asset: USD, pool: p, action: x };
        }
        "#
    );
}

#[test]
fn comments_are_ignored() {
    assert_parse_snapshot!(
        r#"
        // The agreement header.
        profile "moriarty-beta/1"; /* inline */
        agreement Commented {
          const fee = 0.10 /* cents */ USD; // trailing
        }
        "#
    );
}

#[test]
fn node_spans() {
    assert_spans_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Spans {
          const a: Map<String, Qty<USD>> = (1 + 2) * 3 - 4;
          const b = [1, 2,];
          const c = {};
          const d = { key: 10.50 USD, nested: [], };
          const e = amm.swap_exact_input(pool: p, input: 5 USD,);
          const f = rounds();
          const g = None;
          action pay uses e;
        }
        "#
    );
}

// Errors

#[test]
fn missing_profile() {
    assert_parse_error_snapshot!(
        r#"
        agreement Invoice {}
        "#
    );
}

#[test]
fn wrong_profile() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/2";
        agreement Invoice {}
        "#
    );
}

#[test]
fn profile_is_not_a_string() {
    assert_parse_error_snapshot!(
        r#"
        profile moriarty;
        agreement Invoice {}
        "#
    );
}

#[test]
fn missing_semicolon_at_end_of_line() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const price = 10
          const fee = 1;
        }
        "#
    );
}

#[test]
fn missing_equals() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const price 10.00 USD;
        }
        "#
    );
}

#[test]
fn keyword_as_declaration_name() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const asset = 1;
        }
        "#
    );
}

#[test]
fn misspelled_declaration_keyword() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          cosnt price = 10;
        }
        "#
    );
}

#[test]
fn unknown_declaration_keyword() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          let price = 10;
        }
        "#
    );
}

#[test]
fn unclosed_agreement() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const price = 10;
        "#
    );
}

#[test]
fn file_ends_mid_declaration() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const price =
        "#
    );
}

#[test]
fn code_after_agreement() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {}
        const stray = 1;
        "#
    );
}

#[test]
fn quantity_without_space() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const price = 10USD;
        }
        "#
    );
}

#[test]
fn decimal_without_asset() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const rate = 1.5;
        }
        "#
    );
}

#[test]
fn dotted_name_without_call() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Swap {
          const op = amm.swap_exact_input;
        }
        "#
    );
}

#[test]
fn duplicate_record_field() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Records {
          const r = { name: "usd", id: "usd", id: "dollar" };
        }
        "#
    );
}

#[test]
fn missing_comma_between_fields() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Records {
          const r = { name: "usd" id: "dollar" };
        }
        "#
    );
}

#[test]
fn missing_comma_between_list_items() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Lists {
          const sizes = [1, 2 3];
        }
        "#
    );
}

#[test]
fn missing_comma_between_call_arguments() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Valid {
          const window = rounds(domain: Preview from: 0, to: 10);
        }
        "#
    );
}

#[test]
fn unclosed_record() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Records {
          const r = { name: "usd";
        }
        "#
    );
}

#[test]
fn unclosed_list() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Lists {
          const sizes = [1, 2;
        }
        "#
    );
}

#[test]
fn unclosed_parenthesis() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Math {
          const total = (1 + 2 * 3;
        }
        "#
    );
}

#[test]
fn missing_type() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Types {
          const price: = 10;
        }
        "#
    );
}

#[test]
fn unclosed_type_arguments() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Types {
          const price: Qty<USD = 10.00 USD;
        }
        "#
    );
}

#[test]
fn missing_value() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const price = ;
        }
        "#
    );
}

#[test]
fn positional_call_argument() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Valid {
          const window = rounds(Preview, 0, 10);
        }
        "#
    );
}

#[test]
fn action_without_uses() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          action pay invoice;
        }
        "#
    );
}

#[test]
fn lex_errors_pass_through() {
    assert_parse_error_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const amount = 01;
        }
        "#
    );
}

// Bounds

#[test]
fn depth_bound() {
    let nested = |depth: usize| {
        let parens = depth - 1;
        agreement(&format!(
            "const x = {}1{};",
            "(".repeat(parens),
            ")".repeat(parens)
        ))
    };
    assert_eq!(error_code(&nested(MAX_DEPTH)), None);
    assert_eq!(
        error_code(&nested(MAX_DEPTH + 1)).as_deref(),
        Some("mori::syntax::too_deep")
    );
}

#[test]
fn field_bound() {
    let record = |fields: usize| {
        let fields: Vec<String> = (0..fields).map(|i| format!("f{i}: 1")).collect();
        agreement(&format!("const r = {{ {} }};", fields.join(", ")))
    };
    assert_eq!(error_code(&record(MAX_FIELDS)), None);
    assert_eq!(
        error_code(&record(MAX_FIELDS + 1)).as_deref(),
        Some("mori::syntax::too_many_fields")
    );
}

#[test]
fn item_bound() {
    let items = |count: usize| {
        let items: Vec<String> = (0..count).map(|i| format!("const c{i} = 1;")).collect();
        agreement(&items.join("\n"))
    };
    assert_eq!(error_code(&items(MAX_ITEMS)), None);
    assert_eq!(
        error_code(&items(MAX_ITEMS + 1)).as_deref(),
        Some("mori::syntax::too_many_items")
    );
}

// Fixtures

/// Every program in `crates/fixtures/` parses, except the deliberately invalid one.
#[test]
fn fixtures_parse() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures");
    let mut paths = Vec::new();
    for entry in std::fs::read_dir(&fixtures).expect("fixtures directory exists") {
        let entry = entry.expect("directory entry is readable");
        let file_type = entry.file_type().expect("fixture file type is readable");
        let path = entry.path();
        if file_type.is_file() && path.extension().is_some_and(|extension| extension == "mori") {
            paths.push(path);
        }
    }
    paths.sort();
    let names: Vec<_> = paths
        .iter()
        .map(|path| {
            path.file_name()
                .expect("fixture has a file name")
                .to_str()
                .expect("fixture file name is UTF-8")
        })
        .collect();
    assert_eq!(
        names,
        [
            "amm.mori",
            "bridge.mori",
            "derivatives.mori",
            "governance.mori",
            "invalid.mori",
            "lending.mori",
            "lesson-repay.mori",
            "lesson-transfer.mori",
            "oracle.mori",
            "repayment.mori",
            "signed-repay-ecdsa-raw.mori",
            "signed-transfer-ecdsa-wallet.mori",
            "signed-transfer-schnorr-raw.mori",
            "stablecoin.mori",
            "staking.mori",
            "valid.mori",
        ],
        "the complete original top-level fixture set is present"
    );
    let mut checked = 0;
    for path in paths {
        let source = std::fs::read_to_string(&path).expect("fixture is readable");
        let result = parse(&source);
        if path.ends_with("invalid.mori") {
            let code = result
                .err()
                .and_then(|error| error.code().map(|code| code.to_string()));
            assert_eq!(code.as_deref(), Some("mori::lex::leading_zero"));
        } else if let Err(error) = result {
            panic!("{} failed to parse: {error}", path.display());
        }
        checked += 1;
    }
    assert_eq!(checked, 16);
}

// Ordinary diagnostic refinements. Literal expectations were frozen before the
// candidate parser changes; these cases do not exercise depth/resource policy.
fn assert_normal_diagnostic(
    source: &str,
    expected_code: &str,
    expected_message: &str,
    expected_first: usize,
    expected_labels: &[(usize, usize, bool)],
) {
    let error = parse(source).expect_err("the literal normal syntax error must be refused");
    assert_eq!(error.code().map(|code| code.to_string()).as_deref(), Some(expected_code));
    assert_eq!(error.to_string(), expected_message);
    assert_eq!(error.first_label_offset(), Some(expected_first));
    let labels: Vec<_> = error.labels().expect("the error has source labels")
        .map(|label| (label.offset(), label.len(), label.primary()))
        .collect();
    assert_eq!(labels, expected_labels);
}

#[test]
fn normal_missing_values() {
    // missing_rhs_before_const
    assert_normal_diagnostic(
        r#"profile "moriarty-beta/1";
agreement Test {
  const price =
  const fee = 1;
}
"#,
        "mori::syntax::expected_value",
        "I was partway through the declaration of `price` when I found the keyword `const`.",
        62,
        &[(62, 5, true), (46, 5, false)],
    );
    // missing_rhs_before_action
    assert_normal_diagnostic(
        r#"profile "moriarty-beta/1";
agreement Test {
  const price =
  action pay uses invoice;
}
"#,
        "mori::syntax::expected_value",
        "I was partway through the declaration of `price` when I found the keyword `action`.",
        62,
        &[(62, 6, true), (46, 5, false)],
    );
}

#[test]
fn normal_unclosed_lists() {
    // unclosed_list_before_const
    assert_normal_diagnostic(
        r#"profile "moriarty-beta/1";
agreement Test {
  const sizes = [1, 2
  const other = 3;
}
"#,
        "mori::syntax::unclosed_list",
        "I was partway through a list when I found the keyword `const`.",
        68,
        &[(68, 5, true), (60, 1, false)],
    );
    // unclosed_list_before_action
    assert_normal_diagnostic(
        r#"profile "moriarty-beta/1";
agreement Test {
  const sizes = [1, 2
  action pay uses invoice;
}
"#,
        "mori::syntax::unclosed_list",
        "I was partway through a list when I found the keyword `action`.",
        68,
        &[(68, 6, true), (60, 1, false)],
    );
}

#[test]
fn normal_unknown_words() {
    // keyword_profile_item
    assert_normal_diagnostic(
        r#"profile "moriarty-beta/1";
agreement Test {
  profile "moriarty-beta/1";
}
"#,
        "mori::syntax::unknown_item",
        "`profile` cannot start a declaration or action.",
        46,
        &[(46, 7, false)],
    );
    // keyword_tag_item
    assert_normal_diagnostic(
        r#"profile "moriarty-beta/1";
agreement Test {
  SuccessOnly;
}
"#,
        "mori::syntax::unknown_item",
        "`SuccessOnly` cannot start a declaration or action.",
        46,
        &[(46, 11, false)],
    );
}

#[test]
fn normal_numeric_field_commas() {
    // numeric_record_field_comma
    assert_normal_diagnostic(
        r#"profile "moriarty-beta/1";
agreement Test {
  const r = { scale: 2 id: "usd" };
}
"#,
        "mori::syntax::missing_comma",
        "It looks like a comma is missing between two fields.",
        66,
        &[(66, 0, false)],
    );
    // numeric_call_field_comma
    assert_normal_diagnostic(
        r#"profile "moriarty-beta/1";
agreement Test {
  const window = rounds(from: 0 to: 10);
}
"#,
        "mori::syntax::missing_comma",
        "It looks like a comma is missing between two arguments.",
        75,
        &[(75, 0, false)],
    );
}

#[test]
fn normal_primary_order() {
    // duplicate_field_primary_first
    assert_normal_diagnostic(
        r#"profile "moriarty-beta/1";
agreement Test {
  const r = { id: "usd", id: "dollar" };
}
"#,
        "mori::syntax::duplicate_field",
        "The field `id` appears twice.",
        69,
        &[(69, 2, true), (58, 2, false)],
    );
    // unclosed_agreement_primary_first
    assert_normal_diagnostic(
        r#"profile "moriarty-beta/1";
agreement Test {
  const price = 10;
"#,
        "mori::syntax::unclosed_agreement",
        "The agreement `Test` never closes.",
        63,
        &[(63, 0, true), (37, 4, false)],
    );
}

#[test]
fn normal_preserved_list_commas() {
    // keyword_call_missing_list_comma
    assert_normal_diagnostic(
        r#"profile "moriarty-beta/1";
agreement Test {
  const xs = [1 asset(value: 2)];
}
"#,
        "mori::syntax::missing_comma",
        "It looks like a comma is missing between two items.",
        59,
        &[(59, 0, false)],
    );
    // bare_keyword_missing_list_comma
    assert_normal_diagnostic(
        r#"profile "moriarty-beta/1";
agreement Test {
  const xs = [1 pool];
}
"#,
        "mori::syntax::missing_comma",
        "It looks like a comma is missing between two items.",
        59,
        &[(59, 0, false)],
    );
}

#[test]
fn normal_preserved_refusals() {
    // adjacent_identifier_quantity
    assert_normal_diagnostic(
        r#"profile "moriarty-beta/1";
agreement Test {
  const price = 10USD;
}
"#,
        "mori::syntax::quantity_without_space",
        "A quantity needs a space between the number and its asset.",
        60,
        &[(60, 5, false)],
    );
    // decimal_keyword_is_not_asset_identifier
    assert_normal_diagnostic(
        r#"profile "moriarty-beta/1";
agreement Test {
  const price = 10.50 asset;
}
"#,
        "mori::syntax::decimal_without_asset",
        "A decimal number needs an asset.",
        60,
        &[(60, 5, false)],
    );
    // adjacent_keyword_remains_not_quantity
    assert_normal_diagnostic(
        r#"profile "moriarty-beta/1";
agreement Test {
  const price = 10const;
}
"#,
        "mori::syntax::missing_semicolon",
        "I was partway through the declaration of `price` when I found the keyword `const`.",
        62,
        &[(62, 5, true)],
    );
}

#[test]
fn normal_word_namespaces_fields_types_and_multiline_quantities_stay_admitted() {
    use mori_ast::NodeTag;
    use mori_lexer::TokenKind;
    let source = r#"profile "moriarty-beta/1";
agreement Test {
  const values = [pool, true, None, SuccessOnly];
  const keyword_call = asset(action: pool);
  const dotted = policy.action(asset: USD);
  const qty = 10.50 /* units */
    USD;
  const integer_qty = 7
    USD;
  const typed: asset<policy> = None;
  const fields = { asset: USD, action: pool };
}
"#;
    let ast = parse(source).expect("existing syntax forms remain admitted");
    assert_eq!(ast.source, source);
    assert_eq!(ast.token_text(ast.main_token(NodeIdx::ROOT)), "Test");
    let items: Vec<_> = ast.list(NodeIdx::ROOT).collect();
    assert_eq!(items.len(), 7);
    let names: Vec<_> = items.iter().map(|&item| {
        assert_eq!(ast.tag(item), NodeTag::Declaration);
        ast.token_text(ast.token_after(ast.main_token(item), 1))
    }).collect();
    assert_eq!(names, ["values", "keyword_call", "dotted", "qty", "integer_qty", "typed", "fields"]);
    let values: Vec<_> = items.iter().map(|&item| ast.rhs_node(item).unwrap()).collect();
    assert_eq!(values.iter().map(|&node| ast.tag(node)).collect::<Vec<_>>(),
        [NodeTag::Array, NodeTag::Call, NodeTag::Call, NodeTag::Quantity,
         NodeTag::Quantity, NodeTag::Tag, NodeTag::Record]);
    let list: Vec<_> = ast.list(values[0]).collect();
    assert_eq!(list.iter().map(|&node| (ast.tag(node), ast.token_text(ast.main_token(node)))).collect::<Vec<_>>(),
        [(NodeTag::Reference, "pool"), (NodeTag::Bool, "true"),
         (NodeTag::Tag, "None"), (NodeTag::Tag, "SuccessOnly")]);
    assert_eq!(ast.tokens.kind(ast.main_token(list[0])), TokenKind::KwPool);
    assert_eq!(ast.span(values[1]).source_text(source), "asset(action: pool)");
    assert_eq!(ast.span(values[2]).source_text(source), "policy.action(asset: USD)");
    assert_eq!(ast.span(values[3]).source_text(source), "10.50 /* units */\n    USD");
    assert_eq!(ast.span(values[4]).source_text(source), "7\n    USD");
    for &quantity in &values[3..5] {
        let asset = ast.token_after(ast.main_token(quantity), 1);
        assert_eq!(ast.tokens.kind(asset), TokenKind::Ident);
        assert_eq!(ast.token_text(asset), "USD");
    }
    let ty = ast.lhs_node(items[5]).unwrap();
    assert_eq!(ast.tag(ty), NodeTag::Type);
    assert_eq!(ast.token_text(ast.main_token(ty)), "asset");
    let args: Vec<_> = ast.list(ty).collect();
    assert_eq!(args.len(), 1);
    assert_eq!(ast.tag(args[0]), NodeTag::Type);
    assert_eq!(ast.token_text(ast.main_token(args[0])), "policy");
    let record: Vec<_> = ast.list(values[6]).collect();
    assert_eq!(record.iter().map(|&field| {
        assert_eq!(ast.tag(field), NodeTag::Field);
        let value = ast.lhs_node(field).unwrap();
        (ast.token_text(ast.main_token(field)), ast.tag(value), ast.token_text(ast.main_token(value)))
    }).collect::<Vec<_>>(), [("asset", NodeTag::Reference, "USD"), ("action", NodeTag::Reference, "pool")]);
}
