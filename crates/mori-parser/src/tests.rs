use std::path::Path;

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
        Some("BETA_DEPTH_BOUND")
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
        Some("BETA_FIELD_BOUND")
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
        Some("BETA_DECLARATION_BOUND")
    );
}

// Parity

/// Every example shipped with the TypeScript beta must parse, except the one
/// that is deliberately invalid.
#[test]
fn beta_examples_parse() {
    let beta = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../packages/moriarty-beta");
    let mut checked = 0;
    for directory in ["examples", "ai/examples"] {
        visit(&beta.join(directory), &mut |path| {
            let source = std::fs::read_to_string(path).expect("example is readable");
            let result = parse(&source);
            if path.ends_with("ai/examples/invalid.mori") {
                assert_eq!(
                    result
                        .err()
                        .and_then(|e| e.code().map(|c| c.to_string()))
                        .as_deref(),
                    Some("BETA_NUMBER"),
                    "{}",
                    path.display()
                );
            } else if let Err(error) = result {
                panic!("{} failed to parse: {error}", path.display());
            }
            checked += 1;
        });
    }
    assert!(checked >= 14, "only found {checked} examples");
}

fn visit(directory: &Path, f: &mut impl FnMut(&Path)) {
    for entry in std::fs::read_dir(directory).expect("example directory exists") {
        let path = entry.expect("directory entry is readable").path();
        if path.is_dir() {
            visit(&path, f);
        } else if path.extension().is_some_and(|ext| ext == "mori") {
            f(&path);
        }
    }
}
