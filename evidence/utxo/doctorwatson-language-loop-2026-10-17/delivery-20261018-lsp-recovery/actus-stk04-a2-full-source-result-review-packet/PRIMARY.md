
===== Review contract / dispatch hold =====
Fresh nonauthor ordinary Gemini HIGH exact-source/actual-result/guarded-integration review. Root GO authorizes exactly one fresh420s review context after final packet checks. No product execution or provider retry is authorized. Review the entire changed formatted test and direct four-field adapter, every independent semantic oracle value, actual public Core runtime/DTO consumer, literal SHA identity qualification, original compile failure and minimal fix, actual selected PASS, cleanup/resource/exposure records and final integration facts. Do not run products/providers or alter files. No security/authorization/soundness judgment sought. Return a complete whole JSON object under the frozen requested schema; preserve dissent and explicit abstentions. Full physical source, original oracle and raw logs are available. No global-blind, all-wire or all-seven-field ACTUS claim is permitted. No full fixture/family pass is counted. Old two financial tests were filtered out; they were not rerun.

===== crates/mori-core/tests/financial_pair.rs exact SHA256 eb77351e8cdf167de0a9b77cec14f10f248b9916fd70c2047da787d5aaa2f1d4 =====
//! Positive Core/1 financial observations against independently frozen literals.
use mori_core::{Execution, Program, Scenario, UInt, parse, simulate, simulate_metered, validate};
use serde_json::{Value, json};

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../fixtures/financial-pair/",
            $name
        ))
    };
}

fn financial_oracle() -> Value {
    serde_json::from_str(fixture!("financial-and-work.json")).unwrap()
}

fn positive_cases() -> Value {
    serde_json::from_str(fixture!("mori-tests.json")).unwrap()
}

fn case<'a>(cases: &'a Value, name: &str) -> &'a Value {
    assert_eq!(cases["profile"], "moriarty-tests/1");
    let case = cases["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["name"] == name)
        .unwrap();
    assert_eq!(case["expected"]["status"], "accepted");
    case
}

fn program(source: &str, oracle: &Value) -> Program {
    let program = parse(source).expect("frozen positive Core source must parse");
    validate(&program).expect("frozen positive Core program must validate");
    assert_eq!(program.language, oracle["language"].as_str().unwrap());
    assert_eq!(program.semantics, oracle["semantics"].as_str().unwrap());
    program
}

// Decode the oracle's column encoding using only frozen identities and literal
// cells. No expression is evaluated and no expected amount is calculated here.
fn snapshot_from_columns(columns: &Value, row: &Value, identities: &Value) -> Value {
    let mut snapshot = identities.clone();
    let state_columns = columns["state_columns"].as_array().unwrap();
    let state_cells = row["state"].as_array().unwrap();
    let state = snapshot["state"].as_object_mut().unwrap();
    assert_eq!(state.len(), state_columns.len());
    assert_eq!(state_columns.len(), state_cells.len());
    for (column, cell) in state_columns.iter().zip(state_cells) {
        let (name, ty) = column.as_str().unwrap().split_once(':').unwrap();
        let value = state.get_mut(name).unwrap();
        assert_eq!(value["type"], ty);
        value["value"] = cell.clone();
    }

    let balance_columns = columns["balance_columns"].as_array().unwrap();
    let balance_cells = row["balances"].as_array().unwrap();
    let balances = snapshot["balances"].as_array_mut().unwrap();
    assert_eq!(balances.len(), balance_columns.len());
    assert_eq!(balance_columns.len(), balance_cells.len());
    for ((balance, column), cell) in balances.iter_mut().zip(balance_columns).zip(balance_cells) {
        assert_eq!(
            column.as_str().unwrap(),
            format!(
                "{}/{}",
                balance["account"].as_str().unwrap(),
                balance["asset"]["reference"].as_str().unwrap()
            )
        );
        balance["amount"] = cell.clone();
    }

    let duty_columns = columns["duty_columns"].as_array().unwrap();
    let duty_cells = row["duties"].as_array().unwrap();
    let identities = snapshot["liabilities"].as_array().unwrap();
    assert_eq!(identities.len(), duty_columns.len());
    assert_eq!(duty_columns.len(), duty_cells.len());
    let mut duties = Vec::new();
    for ((identity, column), cell) in identities.iter().zip(duty_columns).zip(duty_cells) {
        assert_eq!(
            column.as_str().unwrap(),
            format!(
                "{}:{}:{}:{}",
                identity["name"].as_str().unwrap(),
                identity["asset"]["reference"].as_str().unwrap(),
                identity["debtor"].as_str().unwrap(),
                identity["creditor"].as_str().unwrap()
            )
        );
        if !cell.is_null() {
            let mut duty = identity.clone();
            duty["amount"] = cell.clone();
            duties.push(duty);
        }
    }
    snapshot["liabilities"] = Value::Array(duties);
    snapshot
}

fn compare_execution(
    actual: &Execution,
    oracle: &Value,
    columns: &Value,
    call: &Value,
    statements: &Value,
    case: &Value,
) {
    let expected = &case["expected"];
    assert_eq!(actual.semantics, oracle["semantics"].as_str().unwrap());
    assert_eq!(actual.action, case["action"].as_str().unwrap());
    assert_eq!(actual.action, columns["action"].as_str().unwrap());
    assert_eq!(
        serde_json::to_value(&actual.pre).unwrap(),
        snapshot_from_columns(columns, &call["pre"], &expected["post"])
    );
    assert_eq!(
        serde_json::to_value(&actual.post).unwrap(),
        expected["post"]
    );
    assert_eq!(
        serde_json::to_value(&actual.effects).unwrap(),
        expected["effects"]
    );

    let prefixes = call["prefixes"].as_array().unwrap();
    let statements = statements.as_array().unwrap();
    assert_eq!(actual.steps.len(), prefixes.len());
    assert_eq!(actual.steps.len(), statements.len());
    for ((step, prefix), statement) in actual.steps.iter().zip(prefixes).zip(statements) {
        assert_eq!(json!(step.index), prefix["index"]);
        assert_eq!(json!(step.index), statement["index"]);
        assert_eq!(
            serde_json::to_value(&step.statement).unwrap()["kind"],
            statement["kind"]
        );
        assert_eq!(
            serde_json::to_value(&step.snapshot).unwrap(),
            snapshot_from_columns(columns, prefix, &expected["post"])
        );
        let effects: Vec<Value> = prefix["effects"]
            .as_array()
            .unwrap()
            .iter()
            .map(|index| {
                let index = usize::try_from(index.as_u64().unwrap()).unwrap();
                expected["effects"].as_array().unwrap()[index].clone()
            })
            .collect();
        assert_eq!(serde_json::to_value(&step.effects).unwrap(), json!(effects));
    }
}

// Inline omits unchanged unselected run_positive helper only. Full exact test physical, SHA256
// eb77351e8cdf167de0a9b77cec14f10f248b9916fd70c2047da787d5aaa2f1d4.
// Omitted SHA256 c7a0be3a75642aaceae627b44ea3aea97b2f04f108e1fded8598dbdc2ff5b1af matches main7396d7d3.
// Oldtwo filtered out, not rerun; complete selected added code follows.

// Inline omits oldtwo unselected test bodies only. Full exact test physical, SHA256
// eb77351e8cdf167de0a9b77cec14f10f248b9916fd70c2047da787d5aaa2f1d4.
// Omitted SHA256 0b6885cc58eece35759142528258df57dcc32c57870f92b3a92f1299f63a8f4d matches main7396d7d3.
// Oldtwo filtered out, not rerun; complete selected added code follows.

// Semantic comparison only: the independent freeze explicitly leaves numeric
// ExprIds, whole wire values and three ACTUS state-result mappings unasserted.
fn stk04_independent_cashflow(execution: &Execution, expected_asset: &Value) -> Value {
    let mut payoff = None;
    let mut currency = None;
    let mut event = None;
    for effect in &execution.effects {
        match effect {
            mori_core::Effect::Transfer {
                asset,
                from,
                to,
                amount,
            } => {
                assert!(payoff.is_none(), "one source-authored cashflow per call");
                assert_eq!(serde_json::to_value(asset).unwrap(), *expected_asset);
                currency = Some(asset.reference.clone());
                payoff = Some(match (from.as_str(), to.as_str()) {
                    ("investor", "counterparty") => format!("-{}", amount.0),
                    ("counterparty", "investor") => amount.0.to_string(),
                    _ => panic!("unexpected source cashflow parties"),
                });
            }
            mori_core::Effect::Emit { name, value } => {
                assert!(event.is_none(), "one source-authored event per call");
                let mori_core::Value::UInt(time) = value else {
                    panic!("event must carry the source timestamp")
                };
                event = Some((name.as_str(), time.0));
            }
            mori_core::Effect::State { .. } => {}
            _ => panic!("independent normal trace has no other effect kind"),
        }
    }
    let (event_type, timestamp) = event.unwrap();
    // Mechanical display of the finite supplied YYYYMMDDhhmmss encoding.
    // This is no schedule generator, calendar implementation or clock oracle.
    let digits = format!("{timestamp:014}");
    assert_eq!(digits.len(), 14);
    let date = format!(
        "{}-{}-{}T{}:{}:{}",
        &digits[0..4],
        &digits[4..6],
        &digits[6..8],
        &digits[8..10],
        &digits[10..12],
        &digits[12..14]
    );
    json!({
        "eventDate": date,
        "eventType": event_type,
        "payoff_signed_decimal": payoff.unwrap(),
        "currency": currency.unwrap()
    })
}

#[test]
fn actus_stk04_matches_independent_core_trace_and_four_cashflow_fields() {
    use sha2::Digest;

    let oracle: Value =
        serde_json::from_str(fixture!("actus-stk04/independent/oracle.json")).unwrap();
    let operands: Value = serde_json::from_str(fixture!("actus-stk04/call-operands.json")).unwrap();
    let source = fixture!("actus-stk04/source.mori");
    let opening = fixture!("actus-stk04/opening.scenario.json");
    assert_eq!(
        mori_core::intent::hex(&sha2::Sha256::digest(source.as_bytes())),
        oracle["input_subject"]["source_sha256"].as_str().unwrap()
    );
    assert_eq!(
        mori_core::intent::hex(&sha2::Sha256::digest(opening.as_bytes())),
        oracle["input_subject"]["opening_scenario_sha256"]
            .as_str()
            .unwrap()
    );
    assert_eq!(
        mori_core::intent::hex(&sha2::Sha256::digest(
            fixture!("actus-stk04/call-operands.json").as_bytes()
        )),
        oracle["input_subject"]["call_operands_sha256"]
            .as_str()
            .unwrap()
    );
    let program = parse(source).unwrap();
    validate(&program).unwrap();
    assert_eq!(
        program.language,
        oracle["profile"]["language"].as_str().unwrap()
    );
    assert_eq!(
        program.semantics,
        oracle["profile"]["semantics"].as_str().unwrap()
    );
    let mut scenario: Scenario = serde_json::from_str(opening).unwrap();
    let calls = oracle["calls"].as_array().unwrap();
    let inputs = operands["calls"].as_array().unwrap();
    let records = oracle["cashflow_records"].as_array().unwrap();
    assert_eq!(calls.len(), inputs.len());
    assert_eq!(calls.len(), records.len());
    let mut previous_actual_post: Option<mori_core::Snapshot> = None;
    let mut pair_effects = Vec::new();
    for ((call, input), record) in calls.iter().zip(inputs).zip(records) {
        let action = input["action"].as_str().unwrap();
        assert_eq!(input["action"], call["action"]);
        assert_eq!(input["inputs"], call["input_values"]);
        scenario.inputs = serde_json::from_value(input["inputs"].clone()).unwrap();
        let original = scenario.clone();
        let actual = simulate(&program, action, &scenario).unwrap();
        assert_eq!(scenario, original);
        assert_eq!(actual.action, action);
        assert_eq!(
            actual.semantics,
            oracle["profile"]["semantics"].as_str().unwrap()
        );
        if let Some(previous) = &previous_actual_post {
            assert_eq!(&actual.pre, previous);
        }
        assert_eq!(serde_json::to_value(&actual.pre).unwrap(), call["pre"]);
        assert_eq!(serde_json::to_value(&actual.post).unwrap(), call["post"]);
        assert_eq!(
            serde_json::to_value(&actual.effects).unwrap(),
            call["effects"]
        );
        let expected_steps = call["steps"].as_array().unwrap();
        assert_eq!(actual.steps.len(), expected_steps.len());
        let mut prefix: Vec<Value> = Vec::new();
        for (ordinal, (step, expected)) in actual.steps.iter().zip(expected_steps).enumerate() {
            assert_eq!(usize::from(step.index), ordinal);
            assert_eq!(json!(step.index), expected["index"]);
            assert_eq!(
                serde_json::to_value(&step.value).unwrap(),
                expected["value"]
            );
            assert_eq!(
                serde_json::to_value(&step.snapshot).unwrap(),
                expected["snapshot"]
            );
            let effects = serde_json::to_value(&step.effects).unwrap();
            assert_eq!(effects, expected["effects"]);
            prefix.extend(effects.as_array().unwrap().iter().cloned());
            assert_eq!(json!(prefix), expected["effect_prefix"]);

            // Retain all specified statement metadata. Remove only each
            // explicitly unasserted ExprId field, requiring its typed presence.
            let mut statement = serde_json::to_value(&step.statement).unwrap();
            let object = statement.as_object_mut().unwrap();
            for member in expected["statement_contract"]["wire_exprid_members_unasserted"]
                .as_array()
                .unwrap()
            {
                let reference = object.remove(member.as_str().unwrap()).unwrap();
                assert!(reference.is_u64());
            }
            assert_eq!(
                statement,
                expected["statement_contract"]["known_wire_members"]
            );
        }
        assert_eq!(json!(prefix), call["effects"]);
        let cashflow = stk04_independent_cashflow(&actual, &oracle["accounting"]["asset"]);
        assert_eq!(
            cashflow,
            json!({
                "eventDate": record["eventDate"],
                "eventType": record["eventType"],
                "payoff_signed_decimal": record["payoff_signed_decimal"],
                "currency": record["currency"]
            })
        );
        assert_eq!(
            serde_json::to_value(&actual.post.state["statusTime"]).unwrap(),
            record["core_status_time_after"]
        );
        pair_effects.extend(
            serde_json::to_value(&actual.effects)
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .cloned(),
        );
        previous_actual_post = Some(actual.post.clone());
        // Expected state is comparison-only: the actual complete poststate
        // supplies the next call, while inputs come from input custody.
        scenario = Scenario {
            state: actual.post.state,
            inputs: Default::default(),
            balances: actual.post.balances,
            liabilities: actual.post.liabilities,
        };
    }
    assert_eq!(
        json!(pair_effects),
        oracle["complete_ordered_effect_history"]
    );
}


===== crates/mori-core/src/runtime.rs exact SHA256 eceac176f673180e83533ce55a99afa5bdf3f97a548bbb1c463dac3f84c83930 =====
use crate::policy::{CostPlane, Meter};
use crate::*;
use std::collections::{BTreeMap, BTreeSet};

fn failure(code: &str, message: impl Into<String>) -> Error {
    Error::new(code, message)
}
pub(crate) fn identifier(name: &str) -> bool {
    let mut bytes = name.bytes();
    matches!(bytes.next(), Some(b'A'..=b'Z' | b'a'..=b'z'))
        && name.len() <= 64
        && bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
        && !matches!(
            name,
            "language"
                | "agreement"
                | "state"
                | "asset"
                | "action"
                | "input"
                | "let"
                | "next"
                | "requires"
                | "ensures"
                | "emit"
                | "transfer"
                | "fee"
                | "liability"
                | "discharge"
                | "true"
                | "false"
                | "and"
                | "or"
                | "not"
                | "UInt"
                | "Bool"
        )
}
fn names<'a>(values: impl Iterator<Item = &'a str>, kind: &str) -> Result<(), Error> {
    let mut seen = BTreeSet::new();
    for name in values {
        if !identifier(name) || !seen.insert(name) {
            return Err(failure(
                "duplicate",
                format!("Invalid or duplicate {kind} name {name:?}."),
            ));
        }
    }
    Ok(())
}
pub(crate) fn expr_type(
    program: &Program,
    id: ExprId,
    env: &BTreeMap<String, Type>,
    depth: usize,
) -> Result<Type, Error> {
    type_cached(
        program,
        id,
        env,
        depth,
        &mut vec![None; program.expressions.len()],
    )
}
fn type_cached(
    program: &Program,
    id: ExprId,
    env: &BTreeMap<String, Type>,
    depth: usize,
    cache: &mut [Option<Type>],
) -> Result<Type, Error> {
    if depth > 64 {
        return Err(failure("bound", "Expression depth exceeds 64."));
    }
    let index = usize::from(id);
    let node = program
        .expressions
        .get(index)
        .ok_or_else(|| failure("expression", "Expression index out of bounds."))?;
    if let Some(ty) = cache[index] {
        return Ok(ty);
    }
    let result = match node {
        Expr::LiabilityPresent { .. } => Ok(Type::Bool),
        Expr::LiabilityAmount { .. } => Ok(Type::UInt),
        Expr::Literal { value } => Ok(value.ty()),
        Expr::Name { name } => env
            .get(name)
            .copied()
            .ok_or_else(|| failure("name", format!("Unknown or forward name {name:?}."))),
        Expr::Not { value } => {
            if type_cached(program, *value, env, depth + 1, cache)? != Type::Bool {
                return Err(failure("type", "not requires Bool."));
            }
            Ok(Type::Bool)
        }
        Expr::Binary { op, lhs, rhs } => {
            let a = type_cached(program, *lhs, env, depth + 1, cache)?;
            let b = type_cached(program, *rhs, env, depth + 1, cache)?;
            let expected = match op {
                BinaryOp::And | BinaryOp::Or => Type::Bool,
                _ => Type::UInt,
            };
            if a != b || (!matches!(op, BinaryOp::Eq | BinaryOp::Ne) && a != expected) {
                return Err(failure("type", "Binary operands have incompatible types."));
            }
            Ok(
                if matches!(op, BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul) {
                    Type::UInt
                } else {
                    Type::Bool
                },
            )
        }
    }?;
    cache[index] = Some(result);
    Ok(result)
}
/// Validates even manually constructed/deserialized IR before interpretation.
pub fn validate(program: &Program) -> Result<(), Error> {
    if program.language != LANGUAGE || program.semantics != SEMANTICS {
        return Err(failure(
            "profile",
            "Unsupported language or semantics identity.",
        ));
    }
    if !identifier(&program.name) {
        return Err(failure(
            "name",
            "Agreement name must be a non-reserved ASCII identifier of at most 64 bytes.",
        ));
    }
    if program.actions.is_empty() {
        return Err(failure("program", "Program needs at least one action."));
    }
    if program.states.len() + program.assets.len() + program.actions.len() > MAX_DECLARATIONS
        || program.expressions.len() > MAX_EXPRESSIONS
    {
        return Err(failure(
            "bound",
            "Program exceeds finite declaration/expression bounds.",
        ));
    }
    names(
        program
            .states
            .iter()
            .map(|s| s.name.as_str())
            .chain(program.assets.iter().map(|a| a.name.as_str())),
        "declaration",
    )?;
    names(program.actions.iter().map(|a| a.name.as_str()), "action")?;
    let mut identities = BTreeSet::new();
    for asset in &program.assets {
        let id = &asset.identity;
        if id.network.is_empty()
            || id.issuer.is_empty()
            || id.reference.is_empty()
            || id.network.len() > 1024
            || id.issuer.len() > 1024
            || id.reference.len() > 1024
            || id.scale > 38
            || !identities.insert(id)
        {
            return Err(failure(
                "asset",
                "Asset identities must be complete, unique and use scale 0..38.",
            ));
        }
        if program.assets.iter().any(|other| {
            other.identity.network == id.network
                && other.identity.issuer == id.issuer
                && other.identity.reference == id.reference
                && other.identity.scale != id.scale
        }) {
            return Err(failure(
                "asset",
                "Same asset identity cannot have conflicting scales.",
            ));
        }
    }
    let mut depths = Vec::with_capacity(program.expressions.len());
    for (i, e) in program.expressions.iter().enumerate() {
        let refs = match e {
            Expr::Binary { lhs, rhs, .. } => vec![*lhs, *rhs],
            Expr::Not { value } => vec![*value],
            _ => vec![],
        };
        let mut depth = 0;
        for r in refs {
            if usize::from(r) >= i {
                return Err(failure(
                    "expression",
                    "Expression references must point to earlier nodes; cycles are forbidden.",
                ));
            }
            depth = depth.max(depths[usize::from(r)] + 1);
        }
        if depth > 64 {
            return Err(failure("bound", "Expression depth exceeds 64."));
        }
        depths.push(depth);
    }
    let state_types: BTreeMap<_, _> = program
        .states
        .iter()
        .map(|s| (s.name.clone(), s.ty))
        .collect();
    for state in &program.states {
        if state.initial.ty() != state.ty {
            return Err(failure(
                "type",
                format!("Initial state {:?} has wrong type.", state.name),
            ));
        }
    }
    for action in &program.actions {
        if action.inputs.len() > MAX_DECLARATIONS || action.statements.len() > MAX_STATEMENTS {
            return Err(failure(
                "bound",
                "Action exceeds input or statement bounds.",
            ));
        }
        names(action.inputs.iter().map(|i| i.name.as_str()), "input")?;
        let mut env = state_types.clone();
        for input in &action.inputs {
            if env.insert(input.name.clone(), input.ty).is_some()
                || program.assets.iter().any(|a| a.name == input.name)
            {
                return Err(failure("duplicate", "Input cannot shadow a declaration."));
            }
        }
        for statement in &action.statements {
            match statement {
                // Same identity as `stage::require_core1`; only an in-memory
                // program can carry Await here, since `/1` JSON cannot.
                Statement::Await { .. } => {
                    return Err(failure("profile", "Await requires moriarty-core/2."));
                }
                Statement::Let { name, value } => {
                    let ty = expr_type(program, *value, &env, 0)?;
                    if !identifier(name)
                        || env.insert(name.clone(), ty).is_some()
                        || program.assets.iter().any(|a| a.name == *name)
                    {
                        return Err(failure("duplicate", "let cannot shadow an existing name."));
                    }
                }
                Statement::Requires { predicate } | Statement::Ensures { predicate } => {
                    if expr_type(program, *predicate, &env, 0)? != Type::Bool {
                        return Err(failure("type", "Predicate must have type Bool."));
                    }
                }
                Statement::Next { state, value } => {
                    let expected = state_types
                        .get(state)
                        .ok_or_else(|| failure("name", format!("Unknown state {state:?}.")))?;
                    if expr_type(program, *value, &env, 0)? != *expected {
                        return Err(failure("type", "State update has wrong type."));
                    }
                }
                Statement::Emit { name, value } => {
                    if name.is_empty() || name.len() > 1024 {
                        return Err(failure("name", "Event name must contain 1..1024 bytes."));
                    }
                    expr_type(program, *value, &env, 0)?;
                }
                Statement::Transfer {
                    asset,
                    from,
                    to,
                    amount,
                }
                | Statement::Fee {
                    asset,
                    from,
                    to,
                    amount,
                } => {
                    validate_asset(program, asset)?;
                    if from.is_empty() || to.is_empty() || from.len() > 1024 || to.len() > 1024 {
                        return Err(failure("account", "Account identities cannot be empty."));
                    }
                    require_uint(program, *amount, &env)?;
                }
                Statement::Liability {
                    name,
                    asset,
                    debtor,
                    creditor,
                    amount,
                } => {
                    validate_asset(program, asset)?;
                    if name.is_empty()
                        || debtor.is_empty()
                        || creditor.is_empty()
                        || name.len() > 1024
                        || debtor.len() > 1024
                        || creditor.len() > 1024
                    {
                        return Err(failure(
                            "liability",
                            "Liability identities cannot be empty.",
                        ));
                    }
                    require_uint(program, *amount, &env)?;
                }
                Statement::Discharge { name, amount } => {
                    if name.is_empty() || name.len() > 1024 {
                        return Err(failure(
                            "liability",
                            "Liability name must contain 1..1024 bytes.",
                        ));
                    }
                    require_uint(program, *amount, &env)?;
                }
            }
        }
    }
    validate_liability_reads(program)?;
    Ok(())
}
// New reads require a source-fixed identity. Legacy Core external duties and
// discharge-only programs keep their existing validation rules.
pub(crate) fn validate_liability_reads(p: &Program) -> Result<(), Error> {
    let mut reads = BTreeMap::new();
    for expr in &p.expressions {
        if let Expr::LiabilityPresent { name } | Expr::LiabilityAmount { name } = expr {
            if name.is_empty() || name.len() > 1024 {
                return Err(failure(
                    "liability",
                    "Liability read name must contain 1..1024 bytes.",
                ));
            }
            reads.insert(name.as_str(), None);
        }
    }
    for stmt in p.actions.iter().flat_map(|a| &a.statements) {
        if let Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            ..
        } = stmt
            && let Some(found) = reads.get_mut(name.as_str())
        {
            let identity = (
                validate_asset(p, asset)?,
                debtor.as_str(),
                creditor.as_str(),
            );
            if found.is_some_and(|previous| previous != identity) {
                return Err(failure(
                    "liability",
                    format!("Conflicting source liability identity for {name:?}."),
                ));
            }
            *found = Some(identity);
        }
    }
    for (name, identity) in reads {
        if identity.is_none() {
            return Err(failure(
                "liability",
                format!("Liability read {name:?} has no source-derived identity."),
            ));
        }
    }
    Ok(())
}

pub(crate) fn require_uint(
    p: &Program,
    id: ExprId,
    env: &BTreeMap<String, Type>,
) -> Result<(), Error> {
    if expr_type(p, id, env, 0)? != Type::UInt {
        Err(failure(
            "type",
            "Financial amount requires UInt atomic units.",
        ))
    } else {
        Ok(())
    }
}
pub(crate) fn validate_asset<'a>(p: &'a Program, name: &str) -> Result<&'a AssetId, Error> {
    p.assets
        .iter()
        .find(|a| a.name == name)
        .map(|a| &a.identity)
        .ok_or_else(|| failure("asset", format!("Unknown asset {name:?}.")))
}

fn evaluate(
    p: &Program,
    id: ExprId,
    env: &BTreeMap<String, Value>,
    snapshot: &Snapshot,
    meter: &mut Option<&mut Meter>,
) -> Result<Value, Error> {
    eval_cached(
        p,
        id,
        env,
        snapshot,
        &mut vec![None; p.expressions.len()],
        meter,
    )
}
fn charge_logical(meter: &mut Option<&mut Meter>) -> Result<(), Error> {
    if let Some(meter) = meter.as_deref_mut() {
        meter.charge(CostPlane::Logical, UInt(1))?;
    }
    Ok(())
}
fn eval_cached(
    p: &Program,
    id: ExprId,
    env: &BTreeMap<String, Value>,
    snapshot: &Snapshot,
    cache: &mut [Option<Value>],
    meter: &mut Option<&mut Meter>,
) -> Result<Value, Error> {
    let i = usize::from(id);
    if let Some(value) = &cache[i] {
        return Ok(value.clone());
    }
    // Charge the attempted node before its children or operation. Cache hits
    // above remain free, and evaluate creates a new cache for each statement.
    charge_logical(meter)?;
    let result = match &p.expressions[i] {
        Expr::LiabilityPresent { name } | Expr::LiabilityAmount { name } => {
            let row = snapshot
                .liabilities
                .binary_search_by(|l| l.name.as_str().cmp(name.as_str()))
                .ok()
                .map(|index| &snapshot.liabilities[index]);
            if matches!(&p.expressions[i], Expr::LiabilityPresent { .. }) {
                Value::Bool(row.is_some())
            } else {
                Value::UInt(row.map_or(UInt(0), |l| l.amount))
            }
        }
        Expr::Literal { value } => value.clone(),
        Expr::Name { name } => env
            .get(name)
            .cloned()
            .ok_or_else(|| failure("name", format!("Unknown name {name:?}.")))?,
        Expr::Not { value } => match eval_cached(p, *value, env, snapshot, cache, meter)? {
            Value::Bool(b) => Value::Bool(!b),
            _ => return Err(failure("type", "not expects Bool.")),
        },
        Expr::Binary { op, lhs, rhs } => {
            let a = eval_cached(p, *lhs, env, snapshot, cache, meter)?;
            let b = eval_cached(p, *rhs, env, snapshot, cache, meter)?;
            match (op, a, b) {
                (BinaryOp::Add, Value::UInt(UInt(a)), Value::UInt(UInt(b))) => {
                    Value::UInt(UInt(a.checked_add(b).ok_or_else(|| {
                        failure("overflow", "UInt addition exceeds u128.")
                    })?))
                }
                (BinaryOp::Sub, Value::UInt(UInt(a)), Value::UInt(UInt(b))) => {
                    Value::UInt(UInt(a.checked_sub(b).ok_or_else(|| {
                        failure("underflow", "UInt subtraction is negative.")
                    })?))
                }
                (BinaryOp::Mul, Value::UInt(UInt(a)), Value::UInt(UInt(b))) => {
                    Value::UInt(UInt(a.checked_mul(b).ok_or_else(|| {
                        failure("overflow", "UInt multiplication exceeds u128.")
                    })?))
                }
                (BinaryOp::Eq, a, b) => Value::Bool(a == b),
                (BinaryOp::Ne, a, b) => Value::Bool(a != b),
                (BinaryOp::Lt, Value::UInt(a), Value::UInt(b)) => Value::Bool(a < b),
                (BinaryOp::Le, Value::UInt(a), Value::UInt(b)) => Value::Bool(a <= b),
                (BinaryOp::Gt, Value::UInt(a), Value::UInt(b)) => Value::Bool(a > b),
                (BinaryOp::Ge, Value::UInt(a), Value::UInt(b)) => Value::Bool(a >= b),
                (BinaryOp::And, Value::Bool(a), Value::Bool(b)) => Value::Bool(a && b),
                (BinaryOp::Or, Value::Bool(a), Value::Bool(b)) => Value::Bool(a || b),
                _ => return Err(failure("type", "Invalid operand types.")),
            }
        }
    };
    cache[i] = Some(result.clone());
    Ok(result)
}
fn amount(
    p: &Program,
    id: ExprId,
    env: &BTreeMap<String, Value>,
    snapshot: &Snapshot,
    meter: &mut Option<&mut Meter>,
) -> Result<UInt, Error> {
    match evaluate(p, id, env, snapshot, meter)? {
        Value::UInt(v) => Ok(v),
        _ => Err(failure("type", "Expected UInt amount.")),
    }
}
// Core also supports incoming duties without a source declaration. Only names
// declared by Liability statements have a source-fixed identity; do not impose
// the backend's closed, sixteen-entry compiler profile on general Core input.
fn liability_identities(p: &Program) -> Result<BTreeMap<&str, (&AssetId, &str, &str)>, Error> {
    let mut identities = BTreeMap::new();
    for stmt in p.actions.iter().flat_map(|action| &action.statements) {
        if let Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            ..
        } = stmt
        {
            let identity = (
                validate_asset(p, asset)?,
                debtor.as_str(),
                creditor.as_str(),
            );
            if let Some(existing) = identities.insert(name.as_str(), identity)
                && existing != identity
            {
                return Err(failure(
                    "liability",
                    format!("Conflicting source liability identity for {name:?}."),
                ));
            }
        }
    }
    Ok(identities)
}
pub(crate) fn scenario_snapshot(p: &Program, a: &Action, s: &Scenario) -> Result<Snapshot, Error> {
    if s.balances.len() > 4096 || s.liabilities.len() > 1024 {
        return Err(failure(
            "bound",
            "Scenario exceeds 4,096 balances or 1,024 liabilities.",
        ));
    }
    let added_liabilities = a
        .statements
        .iter()
        .filter(|stmt| matches!(stmt, Statement::Liability { .. }))
        .count();
    let trace_records =
        p.states.len() + s.balances.len() + s.liabilities.len() + added_liabilities + 1;
    if trace_records.saturating_mul(a.statements.len() + 2) > MAX_TRACE_CELLS {
        return Err(failure(
            "bound",
            "Scenario and action exceed 65,536 trace snapshot records.",
        ));
    }
    if !s.state.is_empty() && s.state.len() != p.states.len() {
        return Err(failure(
            "state",
            "Provide either initial state {} or every declared state value.",
        ));
    }
    let mut state = BTreeMap::new();
    for d in &p.states {
        let value = if s.state.is_empty() {
            d.initial.clone()
        } else {
            s.state
                .get(&d.name)
                .ok_or_else(|| failure("state", format!("Missing state {:?}.", d.name)))?
                .clone()
        };
        if value.ty() != d.ty {
            return Err(failure(
                "state",
                format!("Wrong type for state {:?}.", d.name),
            ));
        }
        state.insert(d.name.clone(), value);
    }
    if s.inputs.len() != a.inputs.len() {
        return Err(failure(
            "input",
            "Scenario must provide exactly the selected action's inputs.",
        ));
    }
    for input in &a.inputs {
        if s.inputs.get(&input.name).map(Value::ty) != Some(input.ty) {
            return Err(failure(
                "input",
                format!("Missing or wrong type for input {:?}.", input.name),
            ));
        }
    }
    let ids: BTreeSet<_> = p.assets.iter().map(|a| a.identity.clone()).collect();
    let mut keys = BTreeSet::new();
    let mut balances = s.balances.clone();
    for b in &balances {
        if !ids.contains(&b.asset) {
            return Err(failure(
                "asset",
                format!(
                    "Undeclared exact asset identity for account {:?}.",
                    b.account
                ),
            ));
        }
        if b.account.is_empty()
            || b.account.len() > 1024
            || !keys.insert((b.asset.clone(), b.account.clone()))
        {
            return Err(failure(
                "balance",
                "Duplicate or invalid balance account/asset.",
            ));
        }
    }
    for stmt in &a.statements {
        if let Statement::Transfer {
            asset, from, to, ..
        }
        | Statement::Fee {
            asset, from, to, ..
        } = stmt
        {
            let id = validate_asset(p, asset)?;
            for account in [from, to] {
                if !keys.contains(&(id.clone(), account.clone())) {
                    return Err(failure(
                        "balance",
                        format!("Missing explicit balance for {account:?} and asset {asset:?}."),
                    ));
                }
            }
        }
    }
    balances.sort_by(|a, b| (&a.asset, &a.account).cmp(&(&b.asset, &b.account)));
    let identities = liability_identities(p)?;
    let mut liabilities = s.liabilities.clone();
    let mut names = BTreeSet::new();
    for l in &liabilities {
        if !ids.contains(&l.asset) {
            return Err(failure(
                "asset",
                "Liability uses undeclared exact asset identity.",
            ));
        }
        if l.name.is_empty()
            || l.name.len() > 1024
            || l.debtor.is_empty()
            || l.creditor.is_empty()
            || l.debtor.len() > 1024
            || l.creditor.len() > 1024
            || !names.insert(l.name.clone())
        {
            return Err(failure(
                "liability",
                "Invalid or duplicate liability identity.",
            ));
        }
        if let Some(identity) = identities.get(l.name.as_str())
            && *identity != (&l.asset, l.debtor.as_str(), l.creditor.as_str())
        {
            return Err(failure(
                "liability",
                format!("Existing liability identity differs for {:?}.", l.name),
            ));
        }
    }
    liabilities.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(Snapshot {
        state,
        balances,
        liabilities,
    })
}
fn move_funds(
    snapshot: &mut Snapshot,
    id: &AssetId,
    from: &str,
    to: &str,
    amount: UInt,
) -> Result<(), Error> {
    let f = snapshot
        .balances
        .iter()
        .position(|b| b.asset == *id && b.account == from)
        .ok_or_else(|| failure("balance", "Missing source balance."))?;
    let t = snapshot
        .balances
        .iter()
        .position(|b| b.asset == *id && b.account == to)
        .ok_or_else(|| failure("balance", "Missing recipient balance."))?;
    let debit = snapshot.balances[f]
        .amount
        .0
        .checked_sub(amount.0)
        .ok_or_else(|| failure("balance", format!("Insufficient balance for {from:?}.")))?;
    if f != t {
        let credit = snapshot.balances[t]
            .amount
            .0
            .checked_add(amount.0)
            .ok_or_else(|| failure("overflow", "Recipient balance exceeds u128."))?;
        snapshot.balances[f].amount = UInt(debit);
        snapshot.balances[t].amount = UInt(credit);
    }
    Ok(())
}
/// Shared scalar statement execution for legacy actions and admitted segments.
fn execute_statement(
    program: &Program,
    stmt: &Statement,
    env: &mut BTreeMap<String, Value>,
    current: &mut Snapshot,
    meter: &mut Option<&mut Meter>,
    effects: &mut Vec<Effect>,
) -> Result<Option<Value>, Error> {
    charge_logical(meter)?;
    match stmt {
        Statement::Await { guard, .. } => {
            let v = evaluate(program, *guard, env, current, meter)?;
            Ok(Some(v))
        }
        Statement::Let { name, value } => {
            let v = evaluate(program, *value, env, current, meter)?;
            env.insert(name.clone(), v.clone());
            Ok(Some(v))
        }
        Statement::Requires { predicate } | Statement::Ensures { predicate } => {
            let v = evaluate(program, *predicate, env, current, meter)?;
            if v != Value::Bool(true) {
                return Err(failure(
                    if matches!(stmt, Statement::Requires { .. }) {
                        "requires"
                    } else {
                        "ensures"
                    },
                    "Action predicate evaluated to false.",
                ));
            }
            Ok(Some(v))
        }
        Statement::Next { state, value } => {
            let after = evaluate(program, *value, env, current, meter)?;
            let before = current.state[state].clone();
            current.state.insert(state.clone(), after.clone());
            env.insert(state.clone(), after.clone());
            effects.push(Effect::State {
                name: state.clone(),
                before,
                after: after.clone(),
            });
            Ok(Some(after))
        }
        Statement::Emit { name, value } => {
            let v = evaluate(program, *value, env, current, meter)?;
            effects.push(Effect::Emit {
                name: name.clone(),
                value: v.clone(),
            });
            Ok(Some(v))
        }
        Statement::Transfer {
            asset,
            from,
            to,
            amount: expr,
        }
        | Statement::Fee {
            asset,
            from,
            to,
            amount: expr,
        } => {
            let amount = amount(program, *expr, env, current, meter)?;
            let id = validate_asset(program, asset)?.clone();
            move_funds(current, &id, from, to, amount)?;
            effects.push(if matches!(stmt, Statement::Fee { .. }) {
                Effect::Fee {
                    asset: id,
                    from: from.clone(),
                    to: to.clone(),
                    amount,
                }
            } else {
                Effect::Transfer {
                    asset: id,
                    from: from.clone(),
                    to: to.clone(),
                    amount,
                }
            });
            Ok(Some(Value::UInt(amount)))
        }
        Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            amount: expr,
        } => {
            let amount = amount(program, *expr, env, current, meter)?;
            let id = validate_asset(program, asset)?.clone();
            let found = current.liabilities.iter().position(|l| l.name == *name);
            let before = if let Some(index) = found {
                let l = &current.liabilities[index];
                if l.asset != id || l.debtor != *debtor || l.creditor != *creditor {
                    return Err(failure("liability", "Existing liability identity differs."));
                }
                l.amount
            } else {
                UInt(0)
            };
            let after = UInt(
                before
                    .0
                    .checked_add(amount.0)
                    .ok_or_else(|| failure("overflow", "Liability exceeds u128."))?,
            );
            if let Some(index) = found {
                current.liabilities[index].amount = after;
            } else {
                current.liabilities.push(Liability {
                    name: name.clone(),
                    asset: id.clone(),
                    debtor: debtor.clone(),
                    creditor: creditor.clone(),
                    amount: after,
                });
                current.liabilities.sort_by(|a, b| a.name.cmp(&b.name));
            }
            effects.push(Effect::Liability {
                name: name.clone(),
                asset: id,
                debtor: debtor.clone(),
                creditor: creditor.clone(),
                before,
                after,
            });
            Ok(Some(Value::UInt(amount)))
        }
        Statement::Discharge { name, amount: expr } => {
            let amount = amount(program, *expr, env, current, meter)?;
            let l = current
                .liabilities
                .iter_mut()
                .find(|l| l.name == *name)
                .ok_or_else(|| {
                    failure("liability", format!("Unknown residual liability {name:?}."))
                })?;
            let before = l.amount;
            let after =
                UInt(before.0.checked_sub(amount.0).ok_or_else(|| {
                    failure("liability", "Discharge exceeds residual liability.")
                })?);
            l.amount = after;
            effects.push(Effect::Liability {
                name: name.clone(),
                asset: l.asset.clone(),
                debtor: l.debtor.clone(),
                creditor: l.creditor.clone(),
                before,
                after,
            });
            Ok(Some(Value::UInt(amount)))
        }
    }
}

/// Executes one finite action on a clone. A refusal exposes no committed effects.
pub fn simulate(program: &Program, action: &str, scenario: &Scenario) -> Result<Execution, Error> {
    simulate_with_meter(program, action, scenario, None)
}
pub(crate) fn simulate_with_meter(
    program: &Program,
    action: &str,
    scenario: &Scenario,
    mut meter: Option<&mut Meter>,
) -> Result<Execution, Error> {
    validate(program)?;
    let selected = program
        .actions
        .iter()
        .find(|a| a.name == action)
        .ok_or_else(|| failure("action", format!("Unknown action {action:?}.")))?;
    let pre = scenario_snapshot(program, selected, scenario)?;
    let mut current = pre.clone();
    let mut env = current.state.clone();
    env.extend(scenario.inputs.clone());
    let mut steps = vec![];
    let mut all_effects = vec![];
    for (i, stmt) in selected.statements.iter().enumerate() {
        let mut effects = vec![];
        let result = execute_statement(
            program,
            stmt,
            &mut env,
            &mut current,
            &mut meter,
            &mut effects,
        );
        let value = result.map_err(|mut e| {
            e.step = Some(i as u16);
            e
        })?;
        all_effects.extend(effects.clone());
        steps.push(Step {
            index: i as u16,
            statement: stmt.clone(),
            value,
            snapshot: current.clone(),
            effects,
        });
    }
    Ok(Execution {
        semantics: SEMANTICS.into(),
        action: action.into(),
        pre,
        post: current,
        steps,
        effects: all_effects,
    })
}
/// Environment seed supplied only by stage admission.
pub enum SegmentEntry<'a> {
    Start {
        inputs: &'a [crate::stage::NamedValue],
    },
    Resume {
        site: u16,
        captures: &'a [crate::stage::NamedValue],
        event: &'a [crate::stage::NamedValue],
    },
}

pub enum SegmentRun {
    NotAdvanced,
    Ran {
        execution: Box<Execution>,
        exit_site: u16,
        captures_out: Vec<crate::stage::NamedValue>,
    },
}

/// Runs an already-admitted finite source segment with the same scalar executor
/// as legacy simulation. The caller owns admission, publication and work policy.
pub(crate) fn run_segment(
    sp: &crate::stage::StagedProgram,
    action: usize,
    entry: SegmentEntry<'_>,
    boundary: Snapshot,
    meter: &mut Meter,
) -> Result<SegmentRun, Error> {
    let program = sp.program();
    let selected = &program.actions[action];
    let plan = &sp.plan().actions[action];
    let pre = boundary;
    let mut current = pre.clone();
    let mut env = pre.state.clone();
    let mut meter = Some(meter);
    let mut steps = Vec::new();
    let mut all_effects = Vec::new();
    let begin = match entry {
        SegmentEntry::Start { inputs } => {
            env.extend(inputs.iter().map(|v| (v.name.clone(), v.value.clone())));
            0
        }
        SegmentEntry::Resume {
            site,
            captures,
            event,
        } => {
            env.extend(
                captures
                    .iter()
                    .chain(event)
                    .map(|v| (v.name.clone(), v.value.clone())),
            );
            let ordinal = usize::from(plan.sites[usize::from(site) - 1].ordinal);
            let statement = &selected.statements[ordinal];
            let mut effects = Vec::new();
            // execute_statement charges dispatch once, then visits the guard.
            let value = execute_statement(
                program,
                statement,
                &mut env,
                &mut current,
                &mut meter,
                &mut effects,
            )
            .map_err(|mut error| {
                error.step = Some(ordinal as u16);
                error
            })?;
            if value != Some(Value::Bool(true)) {
                return Ok(SegmentRun::NotAdvanced);
            }
            steps.push(Step {
                index: ordinal as u16,
                statement: statement.clone(),
                value,
                snapshot: pre.clone(),
                effects,
            });
            ordinal + 1
        }
    };
    let mut exit_site = 0;
    let mut captures_out = Vec::new();
    for (ordinal, statement) in selected.statements.iter().enumerate().skip(begin) {
        if matches!(statement, Statement::Await { .. }) {
            // Entry charge is attempted now; guard and its Step belong to resume.
            charge_logical(&mut meter).map_err(|mut error| {
                error.step = Some(ordinal as u16);
                error
            })?;
            let site = plan
                .sites
                .iter()
                .find(|s| usize::from(s.ordinal) == ordinal)
                .ok_or_else(|| failure("stage_site", "Await site is absent from stage plan."))?;
            exit_site = site.site;
            for slot in &site.captures {
                let value = env
                    .get(&slot.name)
                    .cloned()
                    .ok_or_else(|| failure("name", format!("Unknown name {:?}.", slot.name)))?;
                captures_out.push(crate::stage::NamedValue {
                    name: slot.name.clone(),
                    value,
                });
            }
            break;
        }
        let mut effects = Vec::new();
        let value = execute_statement(
            program,
            statement,
            &mut env,
            &mut current,
            &mut meter,
            &mut effects,
        )
        .map_err(|mut error| {
            error.step = Some(ordinal as u16);
            error
        })?;
        all_effects.extend(effects.clone());
        steps.push(Step {
            index: ordinal as u16,
            statement: statement.clone(),
            value,
            snapshot: current.clone(),
            effects,
        });
    }
    Ok(SegmentRun::Ran {
        execution: Box::new(Execution {
            semantics: program.semantics.clone(),
            action: selected.name.clone(),
            pre,
            post: current,
            steps,
            effects: all_effects,
        }),
        exit_site,
        captures_out,
    })
}

/// Recomputes all steps and rejects at the first deterministic divergence.
pub fn replay(
    program: &Program,
    action: &str,
    scenario: &Scenario,
    expected: &Execution,
) -> Result<(), Error> {
    let actual = simulate(program, action, scenario)?;
    if actual.semantics != expected.semantics
        || actual.action != expected.action
        || actual.pre != expected.pre
    {
        return Err(failure(
            "replay",
            "Execution identity or predecessor differs.",
        ));
    }
    for i in 0..actual.steps.len().max(expected.steps.len()) {
        if actual.steps.get(i) != expected.steps.get(i) {
            let mut e = failure("replay", format!("First divergence at step {i}."));
            e.step = Some(i as u16);
            return Err(e);
        }
    }
    if actual.effects != expected.effects || actual.post != expected.post {
        return Err(failure(
            "replay",
            "Complete effect sequence or poststate differs.",
        ));
    }
    Ok(())
}


===== crates/mori-core/src/ir.rs exact SHA256 0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d =====
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const LANGUAGE: &str = "moriarty/1";
pub const SEMANTICS: &str = "moriarty-core/1";
pub const MAX_EXPRESSIONS: usize = 8192;
pub const MAX_STATEMENTS: usize = 1024;
pub const MAX_DECLARATIONS: usize = 256;
pub const MAX_TRACE_CELLS: usize = 65_536;

/// Unsigned integral atomic units; JSON uses canonical decimal strings.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UInt(pub u128);
impl Serialize for UInt {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}
impl<'de> Deserialize<'de> for UInt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        if s.is_empty()
            || !s.bytes().all(|b| b.is_ascii_digit())
            || (s.len() > 1 && s.starts_with('0'))
        {
            return Err(serde::de::Error::custom(
                "expected canonical unsigned decimal string",
            ));
        }
        s.parse().map(UInt).map_err(serde::de::Error::custom)
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Type {
    UInt,
    Bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "value", deny_unknown_fields)]
pub enum Value {
    UInt(UInt),
    Bool(bool),
}
impl Value {
    pub fn ty(&self) -> Type {
        match self {
            Self::UInt(_) => Type::UInt,
            Self::Bool(_) => Type::Bool,
        }
    }
}
pub type ExprId = u16;
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Expr {
    LiabilityPresent {
        name: String,
    },
    LiabilityAmount {
        name: String,
    },
    Literal {
        value: Value,
    },
    Name {
        name: String,
    },
    Binary {
        op: BinaryOp,
        lhs: ExprId,
        rhs: ExprId,
    },
    Not {
        value: ExprId,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StateDecl {
    pub name: String,
    pub ty: Type,
    pub initial: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InputDecl {
    pub name: String,
    pub ty: Type,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct AssetId {
    pub network: String,
    pub issuer: String,
    pub reference: String,
    pub scale: u8,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssetDecl {
    pub name: String,
    pub identity: AssetId,
}
/// Serialize-only on the shared type: decoding always goes through the frozen
/// `v1` wire, so JSON can never yield `Await`.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Statement {
    Await {
        label: String,
        binders: Vec<InputDecl>,
        guard: ExprId,
    },
    Let {
        name: String,
        value: ExprId,
    },
    Requires {
        predicate: ExprId,
    },
    Ensures {
        predicate: ExprId,
    },
    Next {
        state: String,
        value: ExprId,
    },
    Emit {
        name: String,
        value: ExprId,
    },
    Transfer {
        asset: String,
        from: String,
        to: String,
        amount: ExprId,
    },
    Fee {
        asset: String,
        from: String,
        to: String,
        amount: ExprId,
    },
    Liability {
        name: String,
        asset: String,
        debtor: String,
        creditor: String,
        amount: ExprId,
    },
    Discharge {
        name: String,
        amount: ExprId,
    },
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Action {
    pub name: String,
    pub inputs: Vec<InputDecl>,
    pub statements: Vec<Statement>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Program {
    pub language: String,
    pub semantics: String,
    pub name: String,
    pub states: Vec<StateDecl>,
    pub assets: Vec<AssetDecl>,
    pub expressions: Vec<Expr>,
    pub actions: Vec<Action>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Balance {
    pub account: String,
    pub asset: AssetId,
    pub amount: UInt,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Liability {
    pub name: String,
    pub asset: AssetId,
    pub debtor: String,
    pub creditor: String,
    pub amount: UInt,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub state: BTreeMap<String, Value>,
    pub inputs: BTreeMap<String, Value>,
    pub balances: Vec<Balance>,
    pub liabilities: Vec<Liability>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub state: BTreeMap<String, Value>,
    pub balances: Vec<Balance>,
    pub liabilities: Vec<Liability>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Effect {
    State {
        name: String,
        before: Value,
        after: Value,
    },
    Transfer {
        asset: AssetId,
        from: String,
        to: String,
        amount: UInt,
    },
    Fee {
        asset: AssetId,
        from: String,
        to: String,
        amount: UInt,
    },
    Liability {
        name: String,
        asset: AssetId,
        debtor: String,
        creditor: String,
        before: UInt,
        after: UInt,
    },
    Emit {
        name: String,
        value: Value,
    },
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub index: u16,
    pub statement: Statement,
    pub value: Option<Value>,
    pub snapshot: Snapshot,
    pub effects: Vec<Effect>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Execution {
    pub semantics: String,
    pub action: String,
    pub pre: Snapshot,
    pub post: Snapshot,
    pub steps: Vec<Step>,
    pub effects: Vec<Effect>,
}
/// Frozen `moriarty-core/1` decode wire: the baseline (fe01bc8b) derives,
/// moved verbatim. Type names are kept so serde's `expecting` text, unknown
/// variant lists and error messages stay byte-identical. There is no Await.
pub mod v1 {
    use super::{AssetDecl, Effect, Expr, ExprId, InputDecl, Snapshot, StateDecl, Value};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(tag = "kind", deny_unknown_fields)]
    pub enum Statement {
        Let {
            name: String,
            value: ExprId,
        },
        Requires {
            predicate: ExprId,
        },
        Ensures {
            predicate: ExprId,
        },
        Next {
            state: String,
            value: ExprId,
        },
        Emit {
            name: String,
            value: ExprId,
        },
        Transfer {
            asset: String,
            from: String,
            to: String,
            amount: ExprId,
        },
        Fee {
            asset: String,
            from: String,
            to: String,
            amount: ExprId,
        },
        Liability {
            name: String,
            asset: String,
            debtor: String,
            creditor: String,
            amount: ExprId,
        },
        Discharge {
            name: String,
            amount: ExprId,
        },
    }
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(deny_unknown_fields)]
    pub struct Action {
        pub name: String,
        pub inputs: Vec<InputDecl>,
        pub statements: Vec<Statement>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(deny_unknown_fields)]
    pub struct Program {
        pub language: String,
        pub semantics: String,
        pub name: String,
        pub states: Vec<StateDecl>,
        pub assets: Vec<AssetDecl>,
        pub expressions: Vec<Expr>,
        pub actions: Vec<Action>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(deny_unknown_fields)]
    pub struct Step {
        pub index: u16,
        pub statement: Statement,
        pub value: Option<Value>,
        pub snapshot: Snapshot,
        pub effects: Vec<Effect>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(deny_unknown_fields)]
    pub struct Execution {
        pub semantics: String,
        pub action: String,
        pub pre: Snapshot,
        pub post: Snapshot,
        pub steps: Vec<Step>,
        pub effects: Vec<Effect>,
    }
}
impl From<v1::Statement> for Statement {
    fn from(s: v1::Statement) -> Self {
        use v1::Statement as W;
        match s {
            W::Let { name, value } => Self::Let { name, value },
            W::Requires { predicate } => Self::Requires { predicate },
            W::Ensures { predicate } => Self::Ensures { predicate },
            W::Next { state, value } => Self::Next { state, value },
            W::Emit { name, value } => Self::Emit { name, value },
            W::Transfer {
                asset,
                from,
                to,
                amount,
            } => Self::Transfer {
                asset,
                from,
                to,
                amount,
            },
            W::Fee {
                asset,
                from,
                to,
                amount,
            } => Self::Fee {
                asset,
                from,
                to,
                amount,
            },
            W::Liability {
                name,
                asset,
                debtor,
                creditor,
                amount,
            } => Self::Liability {
                name,
                asset,
                debtor,
                creditor,
                amount,
            },
            W::Discharge { name, amount } => Self::Discharge { name, amount },
        }
    }
}
impl From<v1::Action> for Action {
    fn from(a: v1::Action) -> Self {
        Self {
            name: a.name,
            inputs: a.inputs,
            statements: a.statements.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<v1::Program> for Program {
    fn from(p: v1::Program) -> Self {
        Self {
            language: p.language,
            semantics: p.semantics,
            name: p.name,
            states: p.states,
            assets: p.assets,
            expressions: p.expressions,
            actions: p.actions.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<v1::Step> for Step {
    fn from(s: v1::Step) -> Self {
        Self {
            index: s.index,
            statement: s.statement.into(),
            value: s.value,
            snapshot: s.snapshot,
            effects: s.effects,
        }
    }
}
impl From<v1::Execution> for Execution {
    fn from(e: v1::Execution) -> Self {
        Self {
            semantics: e.semantics,
            action: e.action,
            pre: e.pre,
            post: e.post,
            steps: e.steps.into_iter().map(Into::into).collect(),
            effects: e.effects,
        }
    }
}
// Every direct, nested or generic decode of a shared type (CLI, replay,
// backend bundles) is routed through `v1`, so no decode site can be missed.
macro_rules! decode_via_v1 {
    ($($ty:ident),*) => {$(
        impl<'de> Deserialize<'de> for $ty {
            fn deserialize<D: serde::Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
                v1::$ty::deserialize(decoder).map(Self::from)
            }
        }
    )*};
}
decode_via_v1!(Statement, Action, Program, Step, Execution);
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)]
#[error("{code}: {message}")]
pub struct Error {
    pub code: String,
    pub message: String,
    pub offset: Option<u32>,
    pub step: Option<u16>,
}
impl Error {
    pub(crate) fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: format!("mori::core::{code}"),
            message: message.into(),
            offset: None,
            step: None,
        }
    }
}


===== Complete selected public exports (full unchanged lib.rs physical) =====
pub use ir::*;
pub use logical_cost::*;
pub use parser::{language, parse};
pub use replay::{DetailedReplayError, ReplayDifference, compare_execution, replay_detailed};
pub use runtime::{replay, simulate, validate};
pub use source_sponsor::*;

===== Existing public intent::hex exact function (full file available physically) =====
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}


===== crates/fixtures/financial-pair/actus-stk04/source.mori exact SHA256 2a1b921fe038520969fb1ef26dd97dfc7116644c93fe9c5de3806b999ab2412b =====
language "moriarty/1";
agreement StockPurchaseTermination {
  asset EUR = { network: "actus-reference", issuer: "fixture-only", reference: "EUR", scale: 0 };
  state purchased: Bool = false;
  state terminated: Bool = false;
  state statusTime: UInt = 20121230000000;
  state notionalPrincipal: UInt = 0;
  state nominalInterestRate: UInt = 0;
  state accruedInterest: UInt = 0;

  action purchase(eventTime: UInt) {
    requires not purchased;
    requires not terminated;
    requires eventTime == 20130101000000;
    transfer(EUR, "investor", "counterparty", 1200);
    next statusTime = eventTime;
    next purchased = true;
    emit "PRD"(eventTime);
  }

  action terminate(eventTime: UInt) {
    requires purchased;
    requires not terminated;
    requires eventTime == 20161231235959;
    transfer(EUR, "counterparty", "investor", 3000);
    next statusTime = eventTime;
    next terminated = true;
    emit "TD"(eventTime);
  }
}


===== crates/fixtures/financial-pair/actus-stk04/opening.scenario.json exact SHA256 656e74005ec45a8ec8d32a274782692081328aad7e478f17529ed9f617049f9d =====
{
  "state": {},
  "inputs": {
    "eventTime": {
      "type": "UInt",
      "value": "20130101000000"
    }
  },
  "balances": [
    {
      "account": "counterparty",
      "asset": {
        "network": "actus-reference",
        "issuer": "fixture-only",
        "reference": "EUR",
        "scale": 0
      },
      "amount": "3000"
    },
    {
      "account": "investor",
      "asset": {
        "network": "actus-reference",
        "issuer": "fixture-only",
        "reference": "EUR",
        "scale": 0
      },
      "amount": "2000"
    }
  ],
  "liabilities": []
}


===== crates/fixtures/financial-pair/actus-stk04/call-operands.json exact SHA256 f54fdd2e30d5d1cb2a7644ef15f4a42b87717d619d583f8dbe993fde27bb4344 =====
{
  "profile": "moriarty/1",
  "semantics": "moriarty-core/1",
  "public_consumer": "mori_core::parse; mori_core::validate; mori_core::simulate",
  "calls": [
    {
      "action": "purchase",
      "scenario": "opening.scenario.json",
      "inputs": {
        "eventTime": {
          "type": "UInt",
          "value": "20130101000000"
        }
      }
    },
    {
      "action": "terminate",
      "scenario_construction": {
        "state": "actual preceding Execution.post.state",
        "balances": "actual preceding Execution.post.balances",
        "liabilities": "actual preceding Execution.post.liabilities"
      },
      "inputs": {
        "eventTime": {
          "type": "UInt",
          "value": "20161231235959"
        }
      }
    }
  ],
  "logical_cap": null,
  "note": "No metered invocation is selected. Any independently derived work schedule is separate from observed work or target fees."
}


===== crates/fixtures/financial-pair/actus-stk04/stk04.normative-operands.json exact SHA256 72b270a283beda840817ed70370cf6a0163a24159e1309aedfb2cb1a6a7c63ee =====
{
  "identifier": "stk04",
  "terms": {
    "contractType": "STK",
    "contractID": "stk04",
    "contractRole": "RPA",
    "contractDealDate": "2012-12-28T00:00:00",
    "statusDate": "2012-12-30T00:00:00",
    "notionalPrincipal": "1000",
    "currency": "EUR",
    "purchaseDate": "2013-01-01T00:00:00",
    "priceAtPurchaseDate": "1200",
    "terminationDate": "2016-12-31T23:59:59",
    "priceAtTerminationDate": "3000",
    "endOfMonthConvention": "EOM"
  },
  "to": "2016-12-31T23:59:59",
  "dataObserved": {},
  "eventsObserved": []
}


===== Complete independent oracle JSON value, whitespace compacted; raw SHA256 a697b6405effee41bc4860f5419673ae942726bb0318fa5a6c5c7b3d39266906; physical raw bytes retained =====
{"accounting":{"asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0},"closing_balance_sum":"5000","closing_balances":{"counterparty":"1200","investor":"3800"},"core_fee_amount":"0","core_fee_effect_count":0,"core_liabilities_closing":[],"core_liabilities_opening":[],"gross_credit":{"counterparty":"1200","investor":"3000"},"gross_debit":{"counterparty":"3000","investor":"1200"},"liability_effect_count":0,"net_delta":{"counterparty":"-1800","investor":"1800"},"note":"The three zero-valued Core state slots are UInt state, not duty rows or established ACTUS result mappings. External/native fees and equity delivery are unasserted.","opening_balance_sum":"5000","opening_balances":{"counterparty":"3000","investor":"2000"},"present_zero_liability_created":false,"signed_investor_cashflow_sum":"1800"},"calls":[{"action":"purchase","effects":[{"amount":"1200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0},"from":"investor","kind":"Transfer","to":"counterparty"},{"after":{"type":"UInt","value":"20130101000000"},"before":{"type":"UInt","value":"20121230000000"},"kind":"State","name":"statusTime"},{"after":{"type":"Bool","value":true},"before":{"type":"Bool","value":false},"kind":"State","name":"purchased"},{"kind":"Emit","name":"PRD","value":{"type":"UInt","value":"20130101000000"}}],"input_values":{"eventTime":{"type":"UInt","value":"20130101000000"}},"post":{"balances":[{"account":"counterparty","amount":"4200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"800","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":true},"statusTime":{"type":"UInt","value":"20130101000000"},"terminated":{"type":"Bool","value":false}}},"pre":{"balances":[{"account":"counterparty","amount":"3000","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"2000","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":false},"statusTime":{"type":"UInt","value":"20121230000000"},"terminated":{"type":"Bool","value":false}}},"steps":[{"effect_prefix":[],"effects":[],"index":0,"snapshot":{"balances":[{"account":"counterparty","amount":"3000","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"2000","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":false},"statusTime":{"type":"UInt","value":"20121230000000"},"terminated":{"type":"Bool","value":false}}},"statement_contract":{"kind":"Requires","known_wire_members":{"kind":"Requires"},"source":"requires not purchased;","wire_exprid_members_unasserted":["predicate"]},"value":{"type":"Bool","value":true}},{"effect_prefix":[],"effects":[],"index":1,"snapshot":{"balances":[{"account":"counterparty","amount":"3000","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"2000","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":false},"statusTime":{"type":"UInt","value":"20121230000000"},"terminated":{"type":"Bool","value":false}}},"statement_contract":{"kind":"Requires","known_wire_members":{"kind":"Requires"},"source":"requires not terminated;","wire_exprid_members_unasserted":["predicate"]},"value":{"type":"Bool","value":true}},{"effect_prefix":[],"effects":[],"index":2,"snapshot":{"balances":[{"account":"counterparty","amount":"3000","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"2000","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":false},"statusTime":{"type":"UInt","value":"20121230000000"},"terminated":{"type":"Bool","value":false}}},"statement_contract":{"kind":"Requires","known_wire_members":{"kind":"Requires"},"source":"requires eventTime == 20130101000000;","wire_exprid_members_unasserted":["predicate"]},"value":{"type":"Bool","value":true}},{"effect_prefix":[{"amount":"1200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0},"from":"investor","kind":"Transfer","to":"counterparty"}],"effects":[{"amount":"1200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0},"from":"investor","kind":"Transfer","to":"counterparty"}],"index":3,"snapshot":{"balances":[{"account":"counterparty","amount":"4200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"800","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":false},"statusTime":{"type":"UInt","value":"20121230000000"},"terminated":{"type":"Bool","value":false}}},"statement_contract":{"kind":"Transfer","known_wire_members":{"asset":"EUR","from":"investor","kind":"Transfer","to":"counterparty"},"source":"transfer(EUR, \"investor\", \"counterparty\", 1200);","wire_exprid_members_unasserted":["amount"]},"value":{"type":"UInt","value":"1200"}},{"effect_prefix":[{"amount":"1200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0},"from":"investor","kind":"Transfer","to":"counterparty"},{"after":{"type":"UInt","value":"20130101000000"},"before":{"type":"UInt","value":"20121230000000"},"kind":"State","name":"statusTime"}],"effects":[{"after":{"type":"UInt","value":"20130101000000"},"before":{"type":"UInt","value":"20121230000000"},"kind":"State","name":"statusTime"}],"index":4,"snapshot":{"balances":[{"account":"counterparty","amount":"4200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"800","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":false},"statusTime":{"type":"UInt","value":"20130101000000"},"terminated":{"type":"Bool","value":false}}},"statement_contract":{"kind":"Next","known_wire_members":{"kind":"Next","state":"statusTime"},"source":"next statusTime = eventTime;","wire_exprid_members_unasserted":["value"]},"value":{"type":"UInt","value":"20130101000000"}},{"effect_prefix":[{"amount":"1200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0},"from":"investor","kind":"Transfer","to":"counterparty"},{"after":{"type":"UInt","value":"20130101000000"},"before":{"type":"UInt","value":"20121230000000"},"kind":"State","name":"statusTime"},{"after":{"type":"Bool","value":true},"before":{"type":"Bool","value":false},"kind":"State","name":"purchased"}],"effects":[{"after":{"type":"Bool","value":true},"before":{"type":"Bool","value":false},"kind":"State","name":"purchased"}],"index":5,"snapshot":{"balances":[{"account":"counterparty","amount":"4200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"800","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":true},"statusTime":{"type":"UInt","value":"20130101000000"},"terminated":{"type":"Bool","value":false}}},"statement_contract":{"kind":"Next","known_wire_members":{"kind":"Next","state":"purchased"},"source":"next purchased = true;","wire_exprid_members_unasserted":["value"]},"value":{"type":"Bool","value":true}},{"effect_prefix":[{"amount":"1200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0},"from":"investor","kind":"Transfer","to":"counterparty"},{"after":{"type":"UInt","value":"20130101000000"},"before":{"type":"UInt","value":"20121230000000"},"kind":"State","name":"statusTime"},{"after":{"type":"Bool","value":true},"before":{"type":"Bool","value":false},"kind":"State","name":"purchased"},{"kind":"Emit","name":"PRD","value":{"type":"UInt","value":"20130101000000"}}],"effects":[{"kind":"Emit","name":"PRD","value":{"type":"UInt","value":"20130101000000"}}],"index":6,"snapshot":{"balances":[{"account":"counterparty","amount":"4200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"800","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":true},"statusTime":{"type":"UInt","value":"20130101000000"},"terminated":{"type":"Bool","value":false}}},"statement_contract":{"kind":"Emit","known_wire_members":{"kind":"Emit","name":"PRD"},"source":"emit \"PRD\"(eventTime);","wire_exprid_members_unasserted":["value"]},"value":{"type":"UInt","value":"20130101000000"}}]},{"action":"terminate","effects":[{"amount":"3000","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0},"from":"counterparty","kind":"Transfer","to":"investor"},{"after":{"type":"UInt","value":"20161231235959"},"before":{"type":"UInt","value":"20130101000000"},"kind":"State","name":"statusTime"},{"after":{"type":"Bool","value":true},"before":{"type":"Bool","value":false},"kind":"State","name":"terminated"},{"kind":"Emit","name":"TD","value":{"type":"UInt","value":"20161231235959"}}],"input_values":{"eventTime":{"type":"UInt","value":"20161231235959"}},"post":{"balances":[{"account":"counterparty","amount":"1200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"3800","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":true},"statusTime":{"type":"UInt","value":"20161231235959"},"terminated":{"type":"Bool","value":true}}},"pre":{"balances":[{"account":"counterparty","amount":"4200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"800","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":true},"statusTime":{"type":"UInt","value":"20130101000000"},"terminated":{"type":"Bool","value":false}}},"steps":[{"effect_prefix":[],"effects":[],"index":0,"snapshot":{"balances":[{"account":"counterparty","amount":"4200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"800","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":true},"statusTime":{"type":"UInt","value":"20130101000000"},"terminated":{"type":"Bool","value":false}}},"statement_contract":{"kind":"Requires","known_wire_members":{"kind":"Requires"},"source":"requires purchased;","wire_exprid_members_unasserted":["predicate"]},"value":{"type":"Bool","value":true}},{"effect_prefix":[],"effects":[],"index":1,"snapshot":{"balances":[{"account":"counterparty","amount":"4200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"800","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":true},"statusTime":{"type":"UInt","value":"20130101000000"},"terminated":{"type":"Bool","value":false}}},"statement_contract":{"kind":"Requires","known_wire_members":{"kind":"Requires"},"source":"requires not terminated;","wire_exprid_members_unasserted":["predicate"]},"value":{"type":"Bool","value":true}},{"effect_prefix":[],"effects":[],"index":2,"snapshot":{"balances":[{"account":"counterparty","amount":"4200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"800","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":true},"statusTime":{"type":"UInt","value":"20130101000000"},"terminated":{"type":"Bool","value":false}}},"statement_contract":{"kind":"Requires","known_wire_members":{"kind":"Requires"},"source":"requires eventTime == 20161231235959;","wire_exprid_members_unasserted":["predicate"]},"value":{"type":"Bool","value":true}},{"effect_prefix":[{"amount":"3000","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0},"from":"counterparty","kind":"Transfer","to":"investor"}],"effects":[{"amount":"3000","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0},"from":"counterparty","kind":"Transfer","to":"investor"}],"index":3,"snapshot":{"balances":[{"account":"counterparty","amount":"1200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"3800","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":true},"statusTime":{"type":"UInt","value":"20130101000000"},"terminated":{"type":"Bool","value":false}}},"statement_contract":{"kind":"Transfer","known_wire_members":{"asset":"EUR","from":"counterparty","kind":"Transfer","to":"investor"},"source":"transfer(EUR, \"counterparty\", \"investor\", 3000);","wire_exprid_members_unasserted":["amount"]},"value":{"type":"UInt","value":"3000"}},{"effect_prefix":[{"amount":"3000","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0},"from":"counterparty","kind":"Transfer","to":"investor"},{"after":{"type":"UInt","value":"20161231235959"},"before":{"type":"UInt","value":"20130101000000"},"kind":"State","name":"statusTime"}],"effects":[{"after":{"type":"UInt","value":"20161231235959"},"before":{"type":"UInt","value":"20130101000000"},"kind":"State","name":"statusTime"}],"index":4,"snapshot":{"balances":[{"account":"counterparty","amount":"1200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"3800","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":true},"statusTime":{"type":"UInt","value":"20161231235959"},"terminated":{"type":"Bool","value":false}}},"statement_contract":{"kind":"Next","known_wire_members":{"kind":"Next","state":"statusTime"},"source":"next statusTime = eventTime;","wire_exprid_members_unasserted":["value"]},"value":{"type":"UInt","value":"20161231235959"}},{"effect_prefix":[{"amount":"3000","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0},"from":"counterparty","kind":"Transfer","to":"investor"},{"after":{"type":"UInt","value":"20161231235959"},"before":{"type":"UInt","value":"20130101000000"},"kind":"State","name":"statusTime"},{"after":{"type":"Bool","value":true},"before":{"type":"Bool","value":false},"kind":"State","name":"terminated"}],"effects":[{"after":{"type":"Bool","value":true},"before":{"type":"Bool","value":false},"kind":"State","name":"terminated"}],"index":5,"snapshot":{"balances":[{"account":"counterparty","amount":"1200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"3800","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":true},"statusTime":{"type":"UInt","value":"20161231235959"},"terminated":{"type":"Bool","value":true}}},"statement_contract":{"kind":"Next","known_wire_members":{"kind":"Next","state":"terminated"},"source":"next terminated = true;","wire_exprid_members_unasserted":["value"]},"value":{"type":"Bool","value":true}},{"effect_prefix":[{"amount":"3000","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0},"from":"counterparty","kind":"Transfer","to":"investor"},{"after":{"type":"UInt","value":"20161231235959"},"before":{"type":"UInt","value":"20130101000000"},"kind":"State","name":"statusTime"},{"after":{"type":"Bool","value":true},"before":{"type":"Bool","value":false},"kind":"State","name":"terminated"},{"kind":"Emit","name":"TD","value":{"type":"UInt","value":"20161231235959"}}],"effects":[{"kind":"Emit","name":"TD","value":{"type":"UInt","value":"20161231235959"}}],"index":6,"snapshot":{"balances":[{"account":"counterparty","amount":"1200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}},{"account":"investor","amount":"3800","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0}}],"liabilities":[],"state":{"accruedInterest":{"type":"UInt","value":"0"},"nominalInterestRate":{"type":"UInt","value":"0"},"notionalPrincipal":{"type":"UInt","value":"0"},"purchased":{"type":"Bool","value":true},"statusTime":{"type":"UInt","value":"20161231235959"},"terminated":{"type":"Bool","value":true}}},"statement_contract":{"kind":"Emit","known_wire_members":{"kind":"Emit","name":"TD"},"source":"emit \"TD\"(eventTime);","wire_exprid_members_unasserted":["value"]},"value":{"type":"UInt","value":"20161231235959"}}]}],"cashflow_records":[{"core_status_time_after":{"type":"UInt","value":"20130101000000"},"currency":"EUR","eventDate":"2013-01-01T00:00:00","eventType":"PRD","formula":"X=1; R(RPA)=+1; PRD=1*(+1)*(-1)*1200=-1200","payoff_signed_decimal":"-1200","status_date_after":"2013-01-01T00:00:00","unasserted_reference_result_fields":["notionalPrincipal","nominalInterestRate","accruedInterest"]},{"core_status_time_after":{"type":"UInt","value":"20161231235959"},"currency":"EUR","eventDate":"2016-12-31T23:59:59","eventType":"TD","formula":"X=1; R(RPA)=+1; TD=1*(+1)*3000=3000","payoff_signed_decimal":"3000","status_date_after":"2016-12-31T23:59:59","unasserted_reference_result_fields":["notionalPrincipal","nominalInterestRate","accruedInterest"]}],"comparison_contract":{"assert":["semantics/action","complete normalized pre/post state, balances and liabilities","all source statement ordinals/kinds and non-ExprId metadata","every typed Step.value and complete Step.snapshot","every ordered Step.effects delta, derived prefix and execution.effects","purchase actual complete post equals terminate actual complete pre","four selected ACTUS eventDate/eventType/payoff/currency semantic fields"],"do_not_assert":["numeric ExprId references or whole serialized statements","whole Program or Execution JSON equality","exact stdout/key-order/bytes","CostReport/used/cap or native/service fee measurements","ACTUS notionalPrincipal/nominalInterestRate/accruedInterest result-field values"],"successor_rule":"Terminate scenario MUST use actual purchase Execution.post state/balances/liabilities with only its new eventTime input; expected snapshots are comparison-only."},"complete_ordered_effect_history":[{"amount":"1200","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0},"from":"investor","kind":"Transfer","to":"counterparty"},{"after":{"type":"UInt","value":"20130101000000"},"before":{"type":"UInt","value":"20121230000000"},"kind":"State","name":"statusTime"},{"after":{"type":"Bool","value":true},"before":{"type":"Bool","value":false},"kind":"State","name":"purchased"},{"kind":"Emit","name":"PRD","value":{"type":"UInt","value":"20130101000000"}},{"amount":"3000","asset":{"issuer":"fixture-only","network":"actus-reference","reference":"EUR","scale":0},"from":"counterparty","kind":"Transfer","to":"investor"},{"after":{"type":"UInt","value":"20161231235959"},"before":{"type":"UInt","value":"20130101000000"},"kind":"State","name":"statusTime"},{"after":{"type":"Bool","value":true},"before":{"type":"Bool","value":false},"kind":"State","name":"terminated"},{"kind":"Emit","name":"TD","value":{"type":"UInt","value":"20161231235959"}}],"format":"doctorwatson-independent-stk04-semantic-oracle/1","input_subject":{"call_operands":"call-operands.json","call_operands_sha256":"f54fdd2e30d5d1cb2a7644ef15f4a42b87717d619d583f8dbe993fde27bb4344","normative_operands":"stk04.normative-operands.json","normative_operands_sha256":"72b270a283beda840817ed70370cf6a0163a24159e1309aedfb2cb1a6a7c63ee","opening_scenario":"opening.scenario.json","opening_scenario_sha256":"656e74005ec45a8ec8d32a274782692081328aad7e478f17529ed9f617049f9d","source":"source.mori","source_sha256":"2a1b921fe038520969fb1ef26dd97dfc7116644c93fe9c5de3806b999ab2412b"},"profile":{"language":"moriarty/1","metered_call_selected":false,"selected_consumer":"public financial_pair via parse/validate/simulate","semantics":"moriarty-core/1"},"status":"FROZEN_EXPECTATIONS_ONLY_CONSUMERS_UNRUN"}


===== SCOPE-AND-EXPOSURE.json =====
{"predicate":"Exact a2 formatted public financial_pair selected case accepts two linked ordinary Core semantic traces, all14 typed Step values/full snapshots/deltas/prefixes/eight ordered effects and four ACTUS cashflow semantic fields.","source":{"commit":"df149c245d632853665a8a704e5e0f89176427f1","tree":"a98ca0190564c9a078a091ac8a4eefc0c793f7bd","testSha256":"eb77351e8cdf167de0a9b77cec14f10f248b9916fd70c2047da787d5aaa2f1d4"},"authorship":"Source author had implementation and prior author-derived expected exposure. Independent custodian derived from input-only custody and normative/DTO clauses; not globally blind. Preserve independent PROVENANCE.json and original author-derived historical freeze without relabelling.","comparison":"Direct actual public parse/validate/simulate calls; independent expected state is comparison-only; actual purchase.post supplies terminate scenario; four-field adapter reads actual Transfer direction/currency/amount and actual Emit tag/timestamp. Source zero state slots are not ACTUS mappings.","shaQualification":"Three digest comparisons bind literal source/scenario/call byte strings via unchanged SHA256, formatted by existing public intent::hex byte formatter. These are source identity checks, not semantic proof or financial oracle.","unknown":["ACTUS notionalPrincipal, nominalInterestRate, accruedInterest result mappings","all numeric ExprId values, whole Program/Execution wire and exact stdout bytes","actual logical work/cap/CostReport; work.json conceptual18/17 is conditional only","general calendar/EOM/scheduling/dividends/equity delivery/accrual/day-count/performance-state mapping","native/proof/ledger/Preview/Fenrir/fees/target correspondence","full fixture/type/family acceptance; original all277/18/32/DS01-07 retained"],"evidenceAxis":"Original compile failure is failure before all financial assertions. Repaired test actual1PASS/2filtered on exact df149/a98ca. Oldtwo financial test bodies compile but were NOT rerun. Newbase reuse is not a fresh run and requires actual selected-source/input/dependency invariance plus root guarded integration facts.","ownership":"ordinary nonsecurity review only; no security/authorization/soundness judgment sought","rootDispatchHold":"Released by explicit root GO for one420s GeminiHIGH context only; product runs/loops remain unauthorized.","newExecution":"none by packet author","currentIntegrationFacts":"FINAL-INTEGRATION-FACTS.json"}

===== ORACLE-COMPACTION.json =====
{"kind":"ordinary lossless JSON whitespace compaction for review transport; not a new golden or freeze","rawPath":"full-source/crates/fixtures/financial-pair/actus-stk04/independent/oracle.json","rawSha256":"a697b6405effee41bc4860f5419673ae942726bb0318fa5a6c5c7b3d39266906","rawBytes":49826,"compactPath":"inline/oracle.compact.json","compactSha256":"4bdc5383ed3857cf7e362de6ec0d4a322b31bba8636d045ec6cbf79657b18063","compactBytes":23678,"verification":"JSON parse/value equality, no field omission or expected value calculation; raw byte identity asserted only for physical original","independentFreezeSha256":"3acc0f0064164026aecbf100bebd3c8da455510ae2d9ea83bb621065192ca3ab"}

===== Minimal root a2 fix; full failed source and formatting diff retained physically =====
diff --git a/crates/mori-core/tests/financial_pair.rs b/crates/mori-core/tests/financial_pair.rs
index dc9fbc13..4afdc47a 100644
--- a/crates/mori-core/tests/financial_pair.rs
+++ b/crates/mori-core/tests/financial_pair.rs
@@ -302,20 +302,19 @@ fn actus_stk04_matches_independent_core_trace_and_four_cashflow_fields() {
     let source = fixture!("actus-stk04/source.mori");
     let opening = fixture!("actus-stk04/opening.scenario.json");
     assert_eq!(
-        format!("{:x}", sha2::Sha256::digest(source.as_bytes())),
+        mori_core::intent::hex(&sha2::Sha256::digest(source.as_bytes())),
         oracle["input_subject"]["source_sha256"].as_str().unwrap()
     );
     assert_eq!(
-        format!("{:x}", sha2::Sha256::digest(opening.as_bytes())),
+        mori_core::intent::hex(&sha2::Sha256::digest(opening.as_bytes())),
         oracle["input_subject"]["opening_scenario_sha256"]
             .as_str()
             .unwrap()
     );
     assert_eq!(
-        format!(
-            "{:x}",
-            sha2::Sha256::digest(fixture!("actus-stk04/call-operands.json").as_bytes())
-        ),
+        mori_core::intent::hex(&sha2::Sha256::digest(
+            fixture!("actus-stk04/call-operands.json").as_bytes()
+        )),
         oracle["input_subject"]["call_operands_sha256"]
             .as_str()
             .unwrap()


===== Actual result / command / resources / cleanup; all raw originals remain physical =====
{"initial":{"recordedAt":"2026-10-18T11:04:40.844565+00:00","sourceCommit":"7392aaf0a708339c1010679f133dd9e2f02d4b34","sourceTree":"e6349c348ac24359a64c075f0550cd7519f97c19","steps":[{"step":"stk04-independent-semantic-trace","finishedAt":"2026-10-18T11:04:40.768972+00:00","exitCode":101,"wallSeconds":25.926733923999564,"sourceCommitAfter":"7392aaf0a708339c1010679f133dd9e2f02d4b34","sourceTreeAfter":"e6349c348ac24359a64c075f0550cd7519f97c19","sourceStatusAfter":"","stdoutSha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855","stderrSha256":"9422cb4618d5819f36546902c431a5a540775de9f72350a4871bbad5ace19dae","attemptConsumed":true}],"temporaryRootExistsAfter":false,"allStepsCompleted":true,"allExitZero":false,"scope":"One STK04 Core semantic test/two real public Core calls/14complete typed step values and snapshots/eight ordered effects/four known cashflow fields; no full7-field ACTUS fixture/native/proof/ledger qualification."},"a2":{"recordedAt":"2026-10-18T11:08:07.631940+00:00","sourceCommit":"df149c245d632853665a8a704e5e0f89176427f1","sourceTree":"a98ca0190564c9a078a091ac8a4eefc0c793f7bd","steps":[{"step":"stk04-independent-semantic-trace","finishedAt":"2026-10-18T11:08:07.549573+00:00","exitCode":0,"wallSeconds":26.594056537000142,"sourceCommitAfter":"df149c245d632853665a8a704e5e0f89176427f1","sourceTreeAfter":"a98ca0190564c9a078a091ac8a4eefc0c793f7bd","sourceStatusAfter":"","stdoutSha256":"ec1e213a62a46c9b60e099251595e63cc8b9ff346497704672f8902bd2c13195","stderrSha256":"0aa8ada6949f9c1a04122f3618408a81bb6a7dd494cdba27fa0a1d861cdda10e","attemptConsumed":true}],"temporaryRootExistsAfter":false,"allStepsCompleted":true,"allExitZero":true,"scope":"One STK04 Core semantic test/two real public Core calls/14complete typed step values and snapshots/eight ordered effects/four known cashflow fields; no full7-field ACTUS fixture/native/proof/ledger qualification."},"a2stdout":"\nrunning 1 test\ntest actus_stk04_matches_independent_core_trace_and_four_cashflow_fields ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s\n\n","a2start":{"startedAt":"2026-10-18T11:07:40.899183+00:00","sourceCommit":"df149c245d632853665a8a704e5e0f89176427f1","sourceTree":"a98ca0190564c9a078a091ac8a4eefc0c793f7bd","sourceStatus":"","argv":["/usr/bin/timeout","--signal=TERM","--kill-after=5s","90s","/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo","test","--manifest-path","/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/Cargo.toml","--frozen","-j1","-p","mori-core","--no-default-features","--test","financial_pair","actus_stk04_matches_independent_core_trace_and_four_cashflow_fields","--","--exact","--test-threads=1"],"cwd":"/tmp/doctorwatson-actus-stk04-d126qz4o/cwd","environment":{"PATH":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin","RUSTC":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc","RUSTDOC":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc","CARGO_HOME":"/tmp/doctorwatson-actus-stk04-d126qz4o/cargo-home","CARGO_TARGET_DIR":"/tmp/doctorwatson-actus-stk04-d126qz4o/target","CARGO_NET_OFFLINE":"true","CARGO_INCREMENTAL":"0","CARGO_CACHE_AUTO_CLEAN_FREQUENCY":"never","GIT_CONFIG_NOSYSTEM":"1","GIT_CONFIG_GLOBAL":"/dev/null","GIT_CONFIG_SYSTEM":"/dev/null","LANG":"C.UTF-8","TMPDIR":"/tmp/doctorwatson-actus-stk04-d126qz4o/tmp"},"sourcePinsVerified":549,"attempt":1,"attemptLimit":1},"initialOwnedProcessCleanup":{"ownedSession":619936,"remainingSessionMembers":[],"scope":"Owned SID census only; no syscall/detached-process exclusion."},"initialTempCleanup":{"recordedAt":"2026-10-18T11:04:40.844383+00:00","ownedTemporaryRoot":"/tmp/doctorwatson-actus-stk04-cqcjy261","existsAfter":false,"scope":"Only owned temporary root removed, public cache symlinks not traversed."},"a2OwnedProcessCleanup":{"ownedSession":624398,"remainingSessionMembers":[],"scope":"Owned SID census only; no syscall/detached-process exclusion."},"a2TempCleanup":{"recordedAt":"2026-10-18T11:08:07.631744+00:00","ownedTemporaryRoot":"/tmp/doctorwatson-actus-stk04-d126qz4o","existsAfter":false,"scope":"Only owned temporary root removed, public cache symlinks not traversed."},"resourceQualification":"Each attempt1/limit1; timeout90s/killgrace5s; -j1/frozen/offline/no-default-features; explicit13environment vars/no HOME; fresh own tmp+target; no install/fetch/warmup. Both original consumed attempts retained, no cap reset. Cleanup is owned SID census/temp deletion, not detached-process/syscall exclusion. Full exact CHECK-PLAN/START/RESULT/raw stdout/stderr/run_actual.py/tool/source pins remain in actual-cycle/.","requiredPhysicalRead":["actual-cycle/stk04-independent-semantic-trace/STDERR.log","actual-cycle/source-fix-a2/stk04-independent-semantic-trace/STDERR.log","actual-cycle/source-fix-a2/ROOT-CAUSE.json","full-source/crates/fixtures/financial-pair/actus-stk04/independent/PROVENANCE.json","full-source/crates/fixtures/financial-pair/actus-stk04/independent/UNASSERTED.json","full-source/crates/fixtures/financial-pair/actus-stk04/independent/reference-field-dispositions.json","full-source/crates/fixtures/financial-pair/actus-stk04/independent/RETENTION.json","full-source/crates/mori-core/src/parser.rs","full-source/crates/mori-lexer/src/lib.rs","full-source/Cargo.toml","full-source/Cargo.lock"]}

===== FINAL current-source / actual-result / guarded-integration facts =====
{"currentSourceCommit":"6c6e29651dc01322bb7eb8a18fc98b3c1e818231","currentSourceTree":"b44043fba64fa2bd396d18412adb3472434c70ac","actualMain":"74a909d69639df8823a527a4d608a7516ccd69a2","actualMainTree":"f18c2346ff435f9ac95a7f6f56457c2edde3f71c","observedPassedCommit":"df149c245d632853665a8a704e5e0f89176427f1","observedPassedTree":"a98ca0190564c9a078a091ac8a4eefc0c793f7bd","clean_at_verification":true,"all42CandidatePathsByteIdentical":true,"upstreamDeltaCount":86,"upstreamDeltaComposition":"PR216 composition36 paths plus PR215 multi-condition50 paths; no other upstream delta","changedPriorPhysicalFiles":[{"path":"crates/mori-core/Cargo.toml","passed_sha256":"33c7ce868c5e6b46412e6d185d1e7d51a2044293190733b0a996200ee1b8943f","current_sha256":"93318da98ab974c2d30210bd868f98fa5c4af60390395dcbe4f65669e815ac7c"},{"path":"crates/mori-core/src/stage/mod.rs","passed_sha256":"0f0a7b97be3d893369fe99a9a50060ceda7724bf26a067870a543e4ea5de80c3","current_sha256":"e3c4a463319056d0a32ac192c57b3f587584976e08974f4dec04c9e881f80f23"},{"path":"crates/mori/Cargo.toml","passed_sha256":"317bf14a6f3b4b26f223ac9dc93823e7d185333a161bd3ad3f0cf1e0fb6dd05f","current_sha256":"2202f89a5c40b7bbe503ebb66a406226c2e446ab46580e1c2ff5d4f870ecea1b"},{"path":"crates/mori/src/commands/test.rs","passed_sha256":"24d82ff71d2417f1434bea24d1be9c55f1ea854406866ea5d54140bf59eb38a0","current_sha256":"6d79546f1bf9c6271ff45ca4e58b514cc2fc6f5a6b970b55f6b3df4e6788aec8"}],"unchangedExistingPinnedSourceFiles":545,"currentPhysicalSourceFiles":631,"rootReceiptPath":"actual-cycle/current-main-integration/CURRENT-INTEGRATION.json","rootReceiptSha256":"bacc66147a6fe5cc88fb775729cb3f4d6021b95263ad5c329fb3e8a21397b9a3","fullBaseDeltaPath":"actual-cycle/current-main-integration/SOURCE-DELTA.diff","fullBaseDeltaSha256":"7281c0ac15127a937a13ef6a48bdfc227e8d94d626e57ce26ded448349ea6409","consumerSelection":"-p mori-core --no-default-features --test financial_pair actus_stk04_matches_independent_core_trace_and_four_cashflow_fields -- --exact --test-threads=1","reuseQuestion":"Assess whether precise current source/config/dependency changes require a concrete retest; do not assume pass carry. Current default-OFF core2-disjoint-draft feature/module and unselected mori CLI/tests/fixtures are changed; runtime/parser/DTO/lexer/lockfile/root manifest and candidate42 are unchanged. No new run on current6c6e is claimed.","guard":"If approved, root must recheck actual main74a909/currentJ6c6e/treeb44043/all42 and selected source/config/fixtures unchanged immediately before integration; stale guard reopens review.","providerAuthorization":"Root GO authorizes one fresh ordinary Gemini HIGH420s context after packet checks; no product execution or provider retry loop."}

===== Full current mori-core Cargo feature/dependency declaration =====
[package]
name = "mori-core"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[features]
core2-disjoint-draft = []
# Opt-in and non-default: enables only the Midnight-native signing-frame
# compatibility test. Ordinary mori-core consumers and SDK-free test
# selection never pull these dependencies.
midnight-compat-tests = ["dep:midnight-base-crypto", "dep:midnight-serialize"]

[dependencies]
mori-lexer.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
sha2.workspace = true
k256.workspace = true
# Optional and off by default; pulled in only by the midnight-compat-tests
# feature for the native compatibility test.
midnight-base-crypto = { workspace = true, optional = true }
midnight-serialize = { workspace = true, optional = true }

[dev-dependencies]
proptest.workspace = true

===== Current staged module feature entry; full after body physical =====
//! Staged `moriarty/2` programs and their local continuation interface.
//!
//! Local and unsigned only. Event binders and captures are caller-chosen values
//! (no authentication), a continuation can be resumed any number of times (no
//! consumption tracking), and no stage authority exists. Outputs carry
//! `STAGE_LABELS` so these limits stay visible.
mod admit;
mod cost;
pub mod decode;
mod digest;
#[cfg(feature = "core2-disjoint-draft")]
pub mod disjoint;
mod plan;
mod profile;
mod report;
mod validate;

