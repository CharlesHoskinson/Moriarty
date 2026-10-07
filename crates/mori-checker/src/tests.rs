use std::fmt::Write;
use std::path::Path;

use mori_ast::{NodeIdx, NodeTag};
use mori_diagnostics::render::render_plain;

use crate::{CheckResult, check};

/// Parses and checks the source, which must pass, and snapshots what each
/// name resolved to.
macro_rules! assert_check_snapshot {
    ($source:literal) => {{
        let source = indoc::indoc!($source);
        let ast = mori_parser::parse(source).unwrap_or_else(|error| panic!("parse failed: {error}"));
        let result = check(&ast);
        let errors: Vec<String> = result.diagnostics.iter().map(ToString::to_string).collect();
        assert!(errors.is_empty(), "expected no errors, got: {errors:?}");
        insta::with_settings!({
            description => format!("Code:\n\n{source}"),
            omit_expression => true,
        }, {
            insta::assert_snapshot!(resolutions(&result));
        });
    }};
}

/// Parses and checks the source, which must fail, and snapshots every
/// rendered error.
macro_rules! assert_check_errors_snapshot {
    ($source:literal) => {{
        let source = indoc::indoc!($source);
        let ast = mori_parser::parse(source).unwrap_or_else(|error| panic!("parse failed: {error}"));
        let result = check(&ast);
        assert!(!result.is_accepted(), "expected check errors");
        let rendered: Vec<String> = result
            .diagnostics
            .iter()
            .map(|diagnostic| render_plain(diagnostic, "test.mori", source))
            .collect();
        insta::with_settings!({
            description => format!("Code:\n\n{source}"),
            omit_expression => true,
        }, {
            insta::assert_snapshot!(rendered.join("\n"));
        });
    }};
}

/// Each reference, quantity and action, and the declaration it names.
fn resolutions(result: &CheckResult<'_>) -> String {
    let checked = &result.checked;
    let ast = checked.ast;
    let mut out = String::new();
    for index in 0..ast.nodes.len() {
        let node = NodeIdx::new(index);
        let used = match ast.tag(node) {
            NodeTag::Reference | NodeTag::Action | NodeTag::Quantity => {
                ast.span(node).source_text(ast.source)
            }
            _ => continue,
        };
        let target = match checked.resolution(node) {
            Some(decl) => {
                let node = checked.declarations.node()[decl.index()];
                format!(
                    "{} {}",
                    ast.token_text(ast.main_token(node)),
                    checked.name(decl)
                )
            }
            None => "unresolved".to_owned(),
        };
        writeln!(out, "{used:<28} -> {target}").unwrap();
    }
    out
}

// Resolution

#[test]
fn names_resolve_to_earlier_declarations() {
    assert_check_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          account alice = { domain: Preview, id: "alice" };
          const price = 10.00 USD;
          const total = price + 0.10 USD;
          intent invoice = { operation: transfer(from: alice, value: total) };
          action pay uses invoice;
        }
        "#
    );
}

/// Every fixture program uses only names declared above their use.
#[test]
fn fixtures_resolve() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures");
    for entry in std::fs::read_dir(&fixtures).expect("fixtures directory exists") {
        let path = entry.expect("directory entry is readable").path();
        let source = std::fs::read_to_string(&path).expect("fixture is readable");
        let Ok(ast) = mori_parser::parse(&source) else {
            continue;
        };
        let result = check(&ast);
        let errors: Vec<String> = result.diagnostics.iter().map(ToString::to_string).collect();
        assert!(errors.is_empty(), "{}: {errors:?}", path.display());
    }
}

// Errors

#[test]
fn unknown_name_with_suggestion() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const price = 10;
          const total = pirce + 1;
        }
        "#
    );
}

#[test]
fn unknown_name_without_suggestion() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const total = shipping + 1;
        }
        "#
    );
}

#[test]
fn unknown_quantity_asset() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const USD = 1;
          const price = 10 USDC;
        }
        "#
    );
}

#[test]
fn used_before_declared() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const total = price + 1;
          const price = 10;
        }
        "#
    );
}

#[test]
fn refers_to_itself() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const count = count + 1;
        }
        "#
    );
}

#[test]
fn duplicate_declaration() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const price = 10;
          const price = 12;
        }
        "#
    );
}

#[test]
fn action_named_like_a_declaration() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          intent pay = { operation: transfer() };
          action pay uses pay;
        }
        "#
    );
}

#[test]
fn action_used_as_value() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          intent invoice = { operation: transfer() };
          action pay uses invoice;
          const copy = pay;
        }
        "#
    );
}

#[test]
fn action_uses_non_intent() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const price = 10;
          action pay uses price;
        }
        "#
    );
}

#[test]
fn reserved_agreement_name() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement transfer {}
        "#
    );
}

#[test]
fn reserved_action_name() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          intent invoice = { operation: transfer() };
          action amount uses invoice;
        }
        "#
    );
}

#[test]
fn failed_declarations_do_not_cascade() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const base = missing;
          const price = base + 1;
          const total = price * 2;
        }
        "#
    );
}

#[test]
fn independent_errors_are_all_reported() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const price = pirce;
          const fee = 1;
          const fee = 2;
          const total = later;
          const later = 3;
        }
        "#
    );
}
