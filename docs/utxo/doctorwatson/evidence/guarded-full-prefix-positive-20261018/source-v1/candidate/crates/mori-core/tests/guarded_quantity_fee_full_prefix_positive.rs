//! Ordinary positive comparison against a separately frozen arithmetic oracle.
//! The adapters below expose observed public Core types in the oracle's semantic
//! schema. They do not evaluate expressions or generate expected financial data.
//! See the adjacent fixture README for the representation and independence limits.
use mori_core::{
    AssetId, BinaryOp, Effect, Expr, Program, Scenario, Snapshot, Statement, UInt, Value, parse,
    simulate, validate,
};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{Value as Json, json};
use std::collections::BTreeMap;

const SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../fixtures/guarded-full-prefix-independent-positive/inputs/quantity.mori"
));
const INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../fixtures/guarded-full-prefix-independent-positive/inputs/INPUT.json"
));
const ORACLE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../fixtures/guarded-full-prefix-independent-positive/oracle/EXPECTED-SEMANTICS.json"
));

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    id: String,
    action: String,
    scenario: Scenario,
}

fn decode<T: DeserializeOwned>(literal: &Json) -> T {
    serde_json::from_value(literal.clone()).expect("frozen literal has the documented public type")
}

// This is a representation adapter for primitive oracle results, not evaluation.
// The oracle deliberately leaves Step.value's serde representation unspecified.
fn typed_result(literal: &Json) -> Value {
    match literal {
        Json::Bool(value) => Value::Bool(*value),
        Json::String(_) => Value::UInt(decode(literal)),
        _ => panic!("this frozen positive oracle supplies only Bool or UInt results"),
    }
}

fn scalar(value: &Value) -> Json {
    match value {
        Value::UInt(value) => json!(value),
        Value::Bool(value) => json!(value),
    }
}

// A row's presence is part of the comparison. Use all exact identity fields,
// reject duplicate keys in the observed data, and compare complete maps. Thus
// order has no semantic effect and neither absent nor extra rows can disappear.
fn balance_rows(snapshot: &Snapshot) -> BTreeMap<(AssetId, String), UInt> {
    let mut rows = BTreeMap::new();
    for row in &snapshot.balances {
        assert!(rows.insert((row.asset.clone(), row.account.clone()), row.amount).is_none());
    }
    rows
}

type DutyKey = (String, AssetId, String, String);
fn liability_rows(snapshot: &Snapshot) -> BTreeMap<DutyKey, UInt> {
    let mut rows = BTreeMap::new();
    for row in &snapshot.liabilities {
        assert!(rows.insert((
            row.name.clone(), row.asset.clone(), row.debtor.clone(), row.creditor.clone(),
        ), row.amount).is_none());
    }
    rows
}

fn assert_frame(actual: &Snapshot, literal: &Json, context: &str) {
    let expected: Snapshot = decode(literal);
    assert_eq!(actual.state, expected.state, "{context}: complete typed state");
    assert_eq!(balance_rows(actual), balance_rows(&expected), "{context}: complete balances");
    assert_eq!(liability_rows(actual), liability_rows(&expected), "{context}: complete duties");
}

fn balance(snapshot: &Snapshot, asset: &AssetId, account: &str) -> UInt {
    let matches: Vec<_> = snapshot.balances.iter()
        .filter(|row| row.asset == *asset && row.account == account).collect();
    assert_eq!(matches.len(), 1, "one explicit balance for {account} and {asset:?}");
    matches[0].amount
}

fn duty<'a>(snapshot: &'a Snapshot, name: &str, asset: &AssetId, debtor: &str, creditor: &str)
    -> Option<&'a mori_core::Liability>
{
    let matches: Vec<_> = snapshot.liabilities.iter().filter(|row|
        row.name == name && row.asset == *asset && row.debtor == debtor && row.creditor == creditor
    ).collect();
    assert!(matches.len() <= 1);
    matches.first().copied()
}

// Render only the tiny expression shapes actually supplied by the frozen source.
// No ExprId number, allocation order, or general source-to-IR law is asserted.
// An unrepresented lowering makes this scoped adapter fail visibly.
fn expression_text(program: &Program, id: u16) -> String {
    match &program.expressions[usize::from(id)] {
        Expr::Name { name } => name.clone(),
        Expr::Binary { op, lhs, rhs } => {
            assert!(matches!(program.expressions[usize::from(*lhs)], Expr::Name { .. }));
            assert!(matches!(program.expressions[usize::from(*rhs)], Expr::Name { .. }));
            let symbol = match op {
                BinaryOp::Ge => ">=",
                BinaryOp::Sub => "-",
                BinaryOp::Mul => "*",
                BinaryOp::Add => "+",
                BinaryOp::Eq => "==",
                _ => panic!("operator outside the frozen positive specimen"),
            };
            format!("{} {symbol} {}", expression_text(program, *lhs), expression_text(program, *rhs))
        }
        _ => panic!("expression shape outside this scoped representation adapter"),
    }
}

fn quoted(text: &str) -> String {
    serde_json::to_string(text).unwrap()
}

fn statement_text(program: &Program, statement: &Statement) -> String {
    let expression = |id| expression_text(program, id);
    match statement {
        Statement::Requires { predicate } => format!("requires {};", expression(*predicate)),
        Statement::Ensures { predicate } => format!("ensures {};", expression(*predicate)),
        Statement::Let { name, value } => format!("let {name} = {};", expression(*value)),
        Statement::Next { state, value } => format!("next {state} = {};", expression(*value)),
        Statement::Emit { name, value } => format!("emit {}({});", quoted(name), expression(*value)),
        Statement::Transfer { asset, from, to, amount } => format!(
            "transfer({asset}, {}, {}, {});", quoted(from), quoted(to), expression(*amount)
        ),
        Statement::Fee { asset, from, to, amount } => format!(
            "fee({asset}, {}, {}, {});", quoted(from), quoted(to), expression(*amount)
        ),
        Statement::Liability { name, asset, debtor, creditor, amount } => format!(
            "liability({}, {asset}, {}, {}, {});", quoted(name), quoted(debtor),
            quoted(creditor), expression(*amount)
        ),
        Statement::Discharge { name, amount } => format!(
            "discharge({}, {});", quoted(name), expression(*amount)
        ),
        Statement::Await { .. } => panic!("Await is outside this Core/1 specimen"),
    }
}

fn result_uint(value: &Value) -> UInt {
    match value {
        Value::UInt(value) => *value,
        Value::Bool(_) => panic!("financial statement result must be UInt"),
    }
}

// Project actual raw effects and their actual adjacent frames into the independent
// oracle's richer semantic operations. This uses no oracle value as an input and
// performs no expression evaluation. Every raw Effect variant is handled.
fn operation(effect: &Effect, statement: &Statement, result: &Value,
    before: &Snapshot, after: &Snapshot) -> Json
{
    match effect {
        Effect::Transfer { asset, from, to, amount } | Effect::Fee { asset, from, to, amount } => {
            json!({
                "kind": if matches!(effect, Effect::Fee { .. }) { "fee" } else { "transfer" },
                "asset": asset, "from": from, "to": to, "amount": amount,
                "payer_before": balance(before, asset, from),
                "payer_after": balance(after, asset, from),
                "recipient_before": balance(before, asset, to),
                "recipient_after": balance(after, asset, to),
            })
        }
        Effect::Liability { name, asset, debtor, creditor, before: opening, after: closing } => {
            let kind = match statement {
                Statement::Liability { .. } => "liability_add",
                Statement::Discharge { .. } => "discharge",
                _ => panic!("duty effect must belong to a duty statement"),
            };
            let mut projected = json!({
                "kind": kind, "name": name, "asset": asset, "debtor": debtor,
                "creditor": creditor, "amount": result_uint(result),
                "opening_amount": opening, "closing_amount": closing,
                "opening_present": duty(before, name, asset, debtor, creditor).is_some(),
                "closing_present": duty(after, name, asset, debtor, creditor).is_some(),
            });
            if kind == "discharge" {
                projected["balance_change"] = json!(balance_rows(before) != balance_rows(after));
            }
            projected
        }
        Effect::State { name, before, after } => json!({
            "kind": "state_write", "name": name, "before": scalar(before), "after": scalar(after),
        }),
        Effect::Emit { name, value } => json!({ "kind": "emit", "name": name, "value": value }),
    }
}

fn category(operations: &[Json], kinds: &[&str]) -> Vec<Json> {
    operations.iter().filter(|op| kinds.contains(&op["kind"].as_str().unwrap())).cloned().collect()
}

#[derive(Default)]
struct Flow {
    nonfee_debit: u128,
    fee_debit: u128,
    gross_credit: u128,
    fee_credit: u128,
}

// Ordinary small-amount reporting arithmetic over ACTUAL effects. These summaries
// are compared with the frozen literal flow table, never used to derive expectations.
fn assert_account_flows(pre: &Snapshot, post: &Snapshot, effects: &[Effect], oracle: &Json) {
    let rows = oracle.as_array().unwrap();
    assert_eq!(rows.len(), pre.balances.len());
    let mut actual = BTreeMap::new();
    for row in &pre.balances {
        assert!(actual.insert((row.asset.clone(), row.account.clone()), Flow::default()).is_none());
    }
    for effect in effects {
        match effect {
            Effect::Transfer { asset, from, to, amount } => {
                actual.get_mut(&(asset.clone(), from.clone())).unwrap().nonfee_debit += amount.0;
                actual.get_mut(&(asset.clone(), to.clone())).unwrap().gross_credit += amount.0;
            }
            Effect::Fee { asset, from, to, amount } => {
                actual.get_mut(&(asset.clone(), from.clone())).unwrap().fee_debit += amount.0;
                let recipient = actual.get_mut(&(asset.clone(), to.clone())).unwrap();
                recipient.gross_credit += amount.0;
                recipient.fee_credit += amount.0;
            }
            _ => {}
        }
    }
    let mut seen = BTreeMap::new();
    for expected in rows {
        let asset: AssetId = decode(&expected["asset"]);
        let account = expected["account"].as_str().unwrap();
        let key = (asset.clone(), account.to_owned());
        assert!(seen.insert(key.clone(), ()).is_none());
        let flow = &actual[&key];
        let opening = balance(pre, &asset, account);
        let closing = balance(post, &asset, account);
        assert_eq!(json!({
            "account": account, "asset": asset,
            "opening_amount": opening, "closing_amount": closing,
            "opening_present": true, "closing_present": true,
            "nonfee_debit": flow.nonfee_debit.to_string(),
            "fee_debit": flow.fee_debit.to_string(),
            "gross_debit_including_fees": (flow.nonfee_debit + flow.fee_debit).to_string(),
            "gross_credit": flow.gross_credit.to_string(),
            "fee_credit": flow.fee_credit.to_string(),
            "signed_net_flow": (i128::try_from(closing.0).unwrap() - i128::try_from(opening.0).unwrap()).to_string(),
        }), *expected, "complete gross/fee/net account flow for {account}");
    }
    assert_eq!(seen.len(), actual.len());
}

fn assert_conservation(pre: &Snapshot, post: &Snapshot, oracle: &Json) {
    let totals = |snapshot: &Snapshot| {
        let mut totals = BTreeMap::<AssetId, u128>::new();
        for row in &snapshot.balances {
            *totals.entry(row.asset.clone()).or_default() += row.amount.0;
        }
        totals
    };
    let opening = totals(pre);
    let closing = totals(post);
    let mut seen = BTreeMap::new();
    for expected in oracle.as_array().unwrap() {
        let asset: AssetId = decode(&expected["asset"]);
        assert!(seen.insert(asset.clone(), ()).is_none());
        assert_eq!(json!({
            "asset": asset,
            "opening_total": opening[&asset].to_string(),
            "closing_total": closing[&asset].to_string(),
            // The complete effect enum above has no mint/burn/external operation.
            "minted": "0", "burned": "0", "net_external_flow": "0",
        }), *expected, "separate token totals, excluding duties");
    }
    assert_eq!(seen.len(), opening.len());
    assert_eq!(seen.len(), closing.len());
}

fn assert_duty_evolution(pre: &Snapshot, post: &Snapshot, operations: &[Json], oracle: &Json) {
    let rows = oracle.as_array().unwrap();
    assert_eq!(rows.len(), pre.liabilities.len());
    assert_eq!(rows.len(), post.liabilities.len());
    let mut seen = BTreeMap::new();
    for expected in rows {
        let asset: AssetId = decode(&expected["asset"]);
        let name = expected["name"].as_str().unwrap();
        let debtor = expected["debtor"].as_str().unwrap();
        let creditor = expected["creditor"].as_str().unwrap();
        assert!(seen.insert((name.to_owned(), asset.clone(), debtor.to_owned(), creditor.to_owned()), ()).is_none());
        let opening = duty(pre, name, &asset, debtor, creditor).unwrap();
        let closing = duty(post, name, &asset, debtor, creditor).unwrap();
        let mut addition = 0u128;
        let mut discharge = 0u128;
        for op in operations.iter().filter(|op| op["name"].as_str() == Some(name)) {
            match op["kind"].as_str().unwrap() {
                "liability_add" => addition += decode::<UInt>(&op["amount"]).0,
                "discharge" => discharge += decode::<UInt>(&op["amount"]).0,
                _ => {}
            }
        }
        assert_eq!(json!({
            "name": name, "asset": asset, "debtor": debtor, "creditor": creditor,
            "opening_amount": opening.amount, "closing_amount": closing.amount,
            "opening_present": true, "closing_present": true,
            "addition": addition.to_string(), "discharge": discharge.to_string(),
        }), *expected, "duty evolution for {name}");
        assert_eq!(opening.amount.0 + addition - discharge, closing.amount.0);
    }
}

// The full frame checks already cover these rows. Compare the frozen oracle's
// explicit preservation fields too, using actual prefix snapshots throughout.
fn assert_preserved(snapshot: &Snapshot, oracle: &Json) {
    let preserved: BTreeMap<String, Value> = decode(&oracle["preserved_state"]);
    for (name, expected) in preserved {
        assert_eq!(snapshot.state.get(&name), Some(&expected));
    }
    let reserves = oracle["reserve_balances"].as_object().unwrap();
    let reserve_asset = snapshot.balances.iter().find(|row| row.asset.reference == "Reserve")
        .expect("complete frame has the exact Reserve asset").asset.clone();
    let cash_asset = snapshot.balances.iter().find(|row| row.asset.reference == "Cash")
        .expect("complete frame has the exact Cash asset").asset.clone();
    // Exact identities are compared by assert_frame, not inferred from a ticker.
    for (account, expected) in reserves {
        assert_eq!(balance(snapshot, &reserve_asset, account), decode::<UInt>(expected));
    }
    assert_eq!(balance(snapshot, &cash_asset, "witness"), decode::<UInt>(&oracle["witness_cash"]));
    let reserve_duty = snapshot.liabilities.iter().find(|row| row.name == "reserveDuty").unwrap();
    assert_eq!(reserve_duty.amount, decode::<UInt>(&oracle["reserveDuty"]));
    let tombstone = &oracle["zero_liability"];
    let zero = snapshot.liabilities.iter().find(|row|
        Some(row.name.as_str()) == tombstone["name"].as_str()).unwrap();
    assert_eq!(zero.amount, decode::<UInt>(&tombstone["amount_before"]));
    assert_eq!(zero.amount, decode::<UInt>(&tombstone["amount_after"]));
    assert_eq!(tombstone["present_before"], true);
    assert_eq!(tombstone["present_after"], true);
}

#[test]
fn guarded_quantity_fee_matches_every_complete_independent_financial_prefix() {
    let input: Input = serde_json::from_str(INPUT).unwrap();
    let expected: Json = serde_json::from_str(ORACLE).unwrap();
    assert_eq!(expected["schema"], "moriarty-independent-financial-semantic-expectation/1");
    assert_eq!(input.id, expected["input_id"].as_str().unwrap());
    assert_eq!(input.action, expected["action"].as_str().unwrap());
    assert_eq!(input.scenario.inputs, decode::<BTreeMap<String, Value>>(&expected["inputs"]));
    let original = input.scenario.clone();

    // The tested consumer is the public production path; no mock/helper evaluator.
    let program = parse(SOURCE).expect("frozen positive source parses");
    validate(&program).expect("frozen positive source validates");
    assert_eq!(program.language, "moriarty/1");
    assert_eq!(program.semantics, "moriarty-core/1");
    let execution = simulate(&program, &input.action, &input.scenario)
        .expect("frozen ordinary positive scenario executes");
    assert_eq!(execution.semantics, "moriarty-core/1");
    assert_eq!(execution.action, input.action);
    assert_frame(&execution.pre, &expected["opening_frame"], "endpoint pre");
    assert_frame(&execution.post, &expected["closing_frame"], "endpoint post");
    let preservation = &expected["explicit_absence_and_preservation"];
    assert_eq!(execution.pre.balances.len() as u64, preservation["balance_rows_before"].as_u64().unwrap());
    assert_eq!(execution.post.balances.len() as u64, preservation["balance_rows_after"].as_u64().unwrap());
    assert_eq!(execution.pre.liabilities.len() as u64, preservation["liability_rows_before"].as_u64().unwrap());
    assert_eq!(execution.post.liabilities.len() as u64, preservation["liability_rows_after"].as_u64().unwrap());
    assert_preserved(&execution.pre, preservation);
    assert_preserved(&execution.post, preservation);

    let prefixes = expected["statement_prefixes"].as_array().unwrap();
    assert_eq!(prefixes.len(), 18);
    assert_eq!(execution.steps.len(), prefixes.len());
    let mut before = &execution.pre;
    let mut cumulative = Vec::<Json>::new();
    let mut raw_effects = Vec::new();
    let mut bindings = BTreeMap::<String, Value>::new();
    for (index, (step, prefix)) in execution.steps.iter().zip(prefixes).enumerate() {
        let ordinal = index + 1;
        assert_eq!(prefix["statement_ordinal"].as_u64().unwrap(), ordinal as u64);
        assert_eq!(usize::from(step.index), index, "prefix {ordinal}: actual index");
        assert_eq!(statement_text(&program, &step.statement), prefix["source_statement"].as_str().unwrap(),
            "prefix {ordinal}: source statement meaning");
        let value = typed_result(&prefix["evaluated_expression_result"]);
        assert_eq!(step.value.as_ref(), Some(&value), "prefix {ordinal}: typed expression value");
        assert_frame(before, &prefix["opening_frame"], &format!("prefix {ordinal} opening"));
        assert_frame(&step.snapshot, &prefix["closing_frame"], &format!("prefix {ordinal} closing"));
        assert_preserved(&step.snapshot, preservation);
        if matches!(step.statement, Statement::Discharge { .. }) {
            let cash_flow = step.snapshot.balances.iter().map(|row|
                row.amount.0.abs_diff(balance(before, &row.asset, &row.account).0)
            ).sum::<u128>();
            assert_eq!(UInt(cash_flow), decode::<UInt>(&preservation["discharge_cash_flow"]));
        }

        // Oracle input row order is intentionally not assumed to be Core row order.
        // Every changing and preserved state/balance/duty field was compared above.
        let actual_result = step.value.as_ref().unwrap();
        let delta: Vec<_> = step.effects.iter().map(|effect|
            operation(effect, &step.statement, actual_result, before, &step.snapshot)
        ).collect();
        assert_eq!(json!(delta), prefix["statement_semantic_operations"],
            "prefix {ordinal}: complete ordered effect delta");
        cumulative.extend(delta);
        raw_effects.extend(step.effects.iter().cloned());
        assert_eq!(json!(cumulative), prefix["cumulative_semantic_operations"],
            "prefix {ordinal}: complete ordered effect history");
        assert_eq!(json!(category(&cumulative, &["transfer", "fee", "liability_add", "discharge"])),
            prefix["cumulative_economic_effects"], "prefix {ordinal}: economic history");
        assert_eq!(json!(category(&cumulative, &["state_write"])),
            prefix["cumulative_state_writes"], "prefix {ordinal}: state history");
        assert_eq!(json!(category(&cumulative, &["emit"])),
            prefix["cumulative_emissions"], "prefix {ordinal}: emission history");
        if let Statement::Let { name, .. } = &step.statement {
            assert!(bindings.insert(name.clone(), actual_result.clone()).is_none());
        }
        assert_eq!(bindings, decode::<BTreeMap<String, Value>>(&prefix["bindings_after"]),
            "prefix {ordinal}: all captured binding values");
        before = &step.snapshot;
    }
    assert_eq!(bindings, decode::<BTreeMap<String, Value>>(&expected["bindings"]));
    assert_eq!(before, &execution.post, "last complete prefix equals endpoint post");
    assert_eq!(execution.effects, raw_effects, "full raw effects equal all checked ordered deltas");
    assert_eq!(json!(cumulative), expected["ordered_semantic_operations"]);
    assert_eq!(json!(category(&cumulative, &["transfer", "fee", "liability_add", "discharge"])),
        expected["ordered_economic_effects"]);
    assert_eq!(json!(category(&cumulative, &["state_write"])), expected["ordered_state_writes"]);
    assert_eq!(json!(category(&cumulative, &["emit"])), expected["ordered_emissions"]);

    assert_account_flows(&execution.pre, &execution.post, &execution.effects,
        &expected["per_account_asset_flows"]);
    assert_conservation(&execution.pre, &execution.post, &expected["asset_conservation"]);
    assert_duty_evolution(&execution.pre, &execution.post, &cumulative, &expected["liability_evolution"]);
    assert_eq!(input.scenario, original, "production consumer preserves the borrowed scenario");
    // No cost, cap49 refusal, proof, circuit, sponsor, wallet or target operation.
}
