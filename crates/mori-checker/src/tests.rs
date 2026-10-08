use std::fmt::Write;
use std::path::Path;

use mori_ast::{NodeIdx, NodeTag};
use mori_diagnostics::render::render_plain;

use mori_lexer::{TokenIdx, TokenKind, decode_string};

use crate::calls::CALLS;
use crate::{CheckResult, Checked, DeclIdx, ValueIdx, ValueTag, check};

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

/// Parses and checks the source, which must pass, and snapshots each
/// declaration's computed value.
macro_rules! assert_values_snapshot {
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
            insta::assert_snapshot!(values(&result.checked));
        });
    }};
}

/// Each declaration's name and value.
fn values(checked: &Checked<'_>) -> String {
    let mut out = String::new();
    for index in 0..checked.declarations.len() {
        let decl = DeclIdx::new(index);
        let value = checked
            .value(decl)
            .expect("accepted declarations have values");
        writeln!(out, "{} = {}", checked.name(decl), show(checked, value)).unwrap();
    }
    out
}

/// A value written out, with amounts in both decimal and smallest units.
fn show(checked: &Checked<'_>, value: ValueIdx) -> String {
    let values = &checked.values;
    let ast = checked.ast;
    let key = |raw: u16| ast.token_text(TokenIdx::new(usize::from(raw)));
    let pairs = |value| {
        values
            .pairs(value)
            .map(|(name, field)| format!("{}: {}", key(name), show(checked, field)))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let row = value.index();
    match values.tag(value) {
        ValueTag::String => decode_string(key(values.rows.a()[row])),
        ValueTag::Bool => (values.rows.a()[row] == 1).to_string(),
        ValueTag::Tag => ["None", "SuccessOnly"][usize::from(values.rows.a()[row])].to_owned(),
        ValueTag::Scalar => values.amount(value).to_string(),
        ValueTag::Quantity => {
            let asset = values.asset(value);
            let scale_value = checked.value(asset).and_then(|entity| {
                values
                    .pairs(entity)
                    .find(|&(name, _)| key(name) == "scale")
                    .map(|(_, scale)| values.amount(scale) as usize)
            });
            let scale = scale_value.unwrap_or(0);
            let atoms = values.amount(value);
            let digits = format!("{atoms:0>width$}", width = scale + 1);
            let (whole, fraction) = digits.split_at(digits.len() - scale);
            let point = if scale > 0 { "." } else { "" };
            format!("{whole}{point}{fraction} {} ({atoms})", checked.name(asset))
        }
        ValueTag::List => {
            let items: Vec<String> = values
                .items(value)
                .map(|item| show(checked, item))
                .collect();
            format!("[{}]", items.join(", "))
        }
        ValueTag::Record => format!("{{ {} }}", pairs(value)),
        ValueTag::Call => {
            // The dotted name runs from the call's first token to its `(`.
            let call = values.node(value);
            let name: String = (ast.main_token(call).index()..)
                .map(TokenIdx::new)
                .take_while(|&token| ast.tokens.kind(token) != TokenKind::LParen)
                .map(|token| ast.token_text(token))
                .collect();
            format!("{name}({})", pairs(value))
        }
        ValueTag::Entity => {
            let decl = values.entity(value);
            format!(
                "{} {}",
                ast.token_text(ast.main_token(checked.declarations.node()[decl.index()])),
                checked.name(decl)
            )
        }
    }
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
          account bob = { domain: Preview, id: "bob" };
          account carol = { domain: Preview, id: "carol" };
          const price = 10.00 USD;
          const total = price + 0.10 USD;
          intent invoice = {
            operation: transfer(from: alice, to: bob, fee_to: carol, value: total, fee: 0.10 USD),
          };
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
          intent pay = {};
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
          intent invoice = {};
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
          intent invoice = {};
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

// Values

#[test]
fn values_are_computed_exactly() {
    assert_values_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Values {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          asset SHARE = { domain: Preview, id: "share", scale: 0, representation: "native" };
          const price: Qty<USD> = 10.5 USD;
          const fee = 0.10 USD;
          const total = (price + fee) * 3 - 1 USD;
          const count: Scalar = 2 * 3 + 1;
          const exact = atoms(asset: USD, value: 1_000);
          const lower = min(a: price, b: fee);
          const higher = max(a: 7, b: 9);
          const shares = 4 SHARE;
          const memo: String = "café";
          const ok: Bool = true;
          const failure_mode = SuccessOnly;
          const amounts = [1 USD, 2.5 USD,];
          const window = rounds(domain: Preview, from: 0, to: 10);
          const record = { a: 1, nested: { b: fee } };
          const home: Domain = Preview;
        }
        "#
    );
}

#[test]
fn every_call_is_listed_once() {
    let names: std::collections::HashSet<&str> = CALLS.iter().map(|call| call.name).collect();
    assert_eq!(CALLS.len(), 30);
    assert_eq!(names.len(), CALLS.len());
}

// Value errors

#[test]
fn adding_different_assets() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Values {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          asset GOLD = { domain: Preview, id: "gold", scale: 3, representation: "native" };
          const total = 10 USD + 5 GOLD;
        }
        "#
    );
}

#[test]
fn adding_a_number_to_an_amount() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Values {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          const tip = 5;
          const total = 10 + 5 USD;
          const other = 5 USD - tip;
        }
        "#
    );
}

#[test]
fn multiplying_two_amounts() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Values {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          const area = 2 USD * 3 USD;
        }
        "#
    );
}

#[test]
fn arithmetic_on_a_string() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Values {
          const label = "fee" + 1;
        }
        "#
    );
}

#[test]
fn subtraction_below_zero() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Values {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          const left = 5 USD - 10.25 USD;
        }
        "#
    );
}

#[test]
fn numbers_too_large() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Values {
          const literal = 340282366920938463463374607431768211456;
          const product = 340282366920938463463374607431768211455 * 2;
        }
        "#
    );
}

#[test]
fn too_many_decimal_places() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Values {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          asset SHARE = { domain: Preview, id: "share", scale: 0, representation: "native" };
          const price = 10.505 USD;
          const shares = 1.5 SHARE;
        }
        "#
    );
}

#[test]
fn amount_of_a_non_asset() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Values {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          account alice = { domain: Preview, id: "alice" };
          const owed = 10 alice;
        }
        "#
    );
}

#[test]
fn call_errors() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Calls {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          account alice = { domain: Preview, id: "alice" };
          const a = rouns(domain: Preview, from: 0, to: 10);
          const b = rounds(domain: Preview, from: 0);
          const c = rounds(domain: Preview, from: 0, to: 10, too: 1);
          const d = rounds(domain: alice, from: 0, to: 10);
        }
        "#
    );
}

#[test]
fn min_of_different_assets() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Values {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          asset GOLD = { domain: Preview, id: "gold", scale: 3, representation: "native" };
          const lower = min(a: 1 USD, b: 1 GOLD);
        }
        "#
    );
}

#[test]
fn asset_scale_errors() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Assets {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          asset A = 5;
          asset B = { domain: Preview, id: "b", representation: "native" };
          asset C = { domain: Preview, id: "c", scale: 19, representation: "native" };
          asset D = { domain: Preview, id: "d", scale: "2", representation: "native" };
        }
        "#
    );
}

#[test]
fn type_annotation_errors() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Types {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          domain Other = { id: "other", chain: "midnight", network: "other" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          asset GOLD = { domain: Preview, id: "gold", scale: 3, representation: "native" };
          account alice = { domain: Preview, id: "alice" };
          const a: Qyt<USD> = 1 USD;
          const b: Qty<USD> = 1 GOLD;
          const c: Scalar = "one";
          const d: Qty = 1 USD;
          const e: Account<Other> = alice;
          const f: Asset = alice;
        }
        "#
    );
}

#[test]
fn independent_value_errors_in_one_list() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Values {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          const price = 10 USD;
          const list = [pirce, 1.234 USD, price];
        }
        "#
    );
}

// Declaration rules

#[test]
fn field_errors() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Fields {
          domain Preview = { id: "preview", chain: "midnight" };
          domain Main = { id: "main", chain: "midnight", network: "main", netwrok: "x" };
          account alice = { domain: "Preview", id: "alice" };
        }
        "#
    );
}

#[test]
fn ids_must_be_unique_and_valid() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Ids {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          domain Mainnet = { id: "main", chain: "midnight", network: "main" };
          account alice = { domain: Preview, id: "alice" };
          account alice2 = { domain: Preview, id: "alice" };
          account alice_main = { domain: Mainnet, id: "alice" };
          account bad = { domain: Preview, id: "not-valid" };
          account reserved = { domain: Preview, id: "transfer" };
        }
        "#
    );
}

#[test]
fn assets_must_share_the_domain() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Domains {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          domain Other = { id: "other", chain: "midnight", network: "other" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          asset GOLD = { domain: Other, id: "gold", scale: 3, representation: "native" };
          obligation loan = { domain: Preview, id: "loan", asset: GOLD };
          share_class shares = { domain: Preview, id: "shares", backing: GOLD };
        }
        "#
    );
}

#[test]
fn pool_rules() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Pools {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          domain Other = { id: "other", chain: "midnight", network: "other" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          asset GOLD = { domain: Other, id: "gold", scale: 3, representation: "native" };
          pool empty = { domain: Preview, id: "empty", assets: [] };
          pool mixed = { domain: Preview, id: "mixed", assets: [USD, GOLD, USD] };
        }
        "#
    );
}

#[test]
fn list_and_choice_fields() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Lists {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          account alice = { domain: Preview, id: "alice" };
          grant g = { signers: [alice, "bob"] };
          stage s = { domain: Preview, reads: "balance", authority: 5 };
          episode e = { id: "e", stages: [g] };
        }
        "#
    );
}

// Call rules

#[test]
fn round_window_must_not_be_inverted() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Rounds {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          const window = rounds(domain: Preview, from: 10, to: 5);
        }
        "#
    );
}

#[test]
fn transfer_rules() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Transfers {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          domain Other = { id: "other", chain: "midnight", network: "other" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          asset GOLD = { domain: Preview, id: "gold", scale: 3, representation: "native" };
          asset EUR = { domain: Other, id: "eur", scale: 2, representation: "native" };
          account alice = { domain: Preview, id: "alice" };
          account bob = { domain: Preview, id: "bob" };
          account carol = { domain: Other, id: "carol" };
          account dave = { domain: Preview, id: "dave" };
          const same = transfer(from: alice, to: bob, fee_to: alice, value: 1 USD, fee: 0 USD);
          const far = transfer(from: alice, to: carol, fee_to: bob, value: 1 USD, fee: 0 USD);
          const fee = transfer(from: alice, to: bob, fee_to: dave, value: 1 USD, fee: 0 GOLD);
          const foreign = transfer(from: alice, to: bob, fee_to: carol, value: 1 EUR, fee: 0 EUR);
        }
        "#
    );
}

#[test]
fn repay_rules() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Repay {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          domain Other = { id: "other", chain: "midnight", network: "other" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          asset GOLD = { domain: Preview, id: "gold", scale: 3, representation: "native" };
          account alice = { domain: Preview, id: "alice" };
          account carol = { domain: Other, id: "carol" };
          obligation loan = { domain: Preview, id: "loan", asset: USD };
          const wrong_asset = repay(obligation: loan, payer: alice, amount: 5 GOLD);
          const wrong_domain = repay(obligation: loan, payer: carol, amount: 5 USD);
        }
        "#
    );
}

#[test]
fn amm_rules() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Amm {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          asset GOLD = { domain: Preview, id: "gold", scale: 3, representation: "native" };
          asset EUR = { domain: Preview, id: "eur", scale: 2, representation: "native" };
          account alice = { domain: Preview, id: "alice" };
          pool amm = { domain: Preview, id: "amm", assets: [USD, GOLD] };
          const swap = amm.swap_exact_input(
            pool: amm,
            owner: alice,
            input: 10 USD,
            output_asset: EUR,
            net_floor: 1 GOLD,
            fee_cap: 0.1 EUR,
          );
        }
        "#
    );
}

#[test]
fn stablecoin_needs_instrument_assets() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Stable {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          account alice = { domain: Preview, id: "alice" };
          instrument coin = { domain: Preview, id: "coin", asset: USD };
          const mint = stablecoin.mint(instrument: coin, owner: alice, supply: 1 USD, backing: 1 USD);
        }
        "#
    );
}

#[test]
fn bridge_crosses_domains_only_at_its_endpoints() {
    assert_values_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Bridge {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          domain Cardano = { id: "cardano", chain: "cardano", network: "mainnet" };
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native" };
          account alice = { domain: Preview, id: "alice" };
          const out = bridge.escrow(owner: alice, amount: 5 USD, destination: Cardano, claim_id: "c1");
        }
        "#
    );
}

#[test]
fn family_arguments_share_one_domain() {
    assert_check_errors_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Family {
          domain Preview = { id: "preview", chain: "midnight", network: "preview" };
          domain Other = { id: "other", chain: "midnight", network: "other" };
          asset USD = { domain: Other, id: "usd", scale: 2, representation: "native" };
          account alice = { domain: Preview, id: "alice" };
          share_class shares = { domain: Other, id: "shares", backing: USD };
          const deposit = staking.deposit(owner: alice, shares: shares, backing: 5 USD);
        }
        "#
    );
}
