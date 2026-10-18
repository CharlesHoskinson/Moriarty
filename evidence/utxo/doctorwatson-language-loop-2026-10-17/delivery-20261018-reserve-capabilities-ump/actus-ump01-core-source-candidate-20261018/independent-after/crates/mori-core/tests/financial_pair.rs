//! Positive Core/1 financial observations against frozen literal expectations.
use mori_core::{
    AssetId, Effect, Execution, Program, Scenario, UInt, Value as CoreValue, parse, simulate,
    simulate_metered, validate,
};
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

fn run_positive(
    program: &Program,
    scenario: &Scenario,
    oracle: &Value,
    columns: &Value,
    call: &Value,
    statements: &Value,
    case: &Value,
) -> Execution {
    let original = scenario.clone();
    let action = case["action"].as_str().unwrap();
    let ordinary = simulate(program, action, scenario).expect("positive action must succeed");
    compare_execution(&ordinary, oracle, columns, call, statements, case);
    assert_eq!(scenario, &original);

    let cost = &columns["work"]["metered_call_expectation"];
    let cap: UInt = serde_json::from_value(cost["cap"].clone()).unwrap();
    let metered = simulate_metered(program, action, scenario, cap)
        .expect("positive action must fit its independently specified logical cap");
    assert_eq!(serde_json::to_value(&metered.cost).unwrap(), *cost);
    compare_execution(&metered.execution, oracle, columns, call, statements, case);
    assert_eq!(scenario, &original);
    ordinary
}

#[test]
fn linked_installments_match_every_frozen_financial_prefix_and_total_work() {
    let oracle = financial_oracle();
    let cases = positive_cases();
    let invoice = &oracle["invoice"];
    let program = program(fixture!("installment-invoice.mori"), &oracle);
    let first_input: Scenario =
        serde_json::from_str(fixture!("invoice-first.scenario.json")).unwrap();
    let final_input: Scenario =
        serde_json::from_str(fixture!("invoice-final.scenario.json")).unwrap();
    let statements = &invoice["work"]["per_collect_statements"];
    let first = run_positive(
        &program,
        &first_input,
        &oracle,
        invoice,
        &invoice["first"],
        statements,
        case(&cases, "installment-first-47"),
    );

    let successor = Scenario {
        state: first.post.state,
        inputs: final_input.inputs.clone(),
        balances: first.post.balances,
        liabilities: first.post.liabilities,
    };
    assert_eq!(successor, final_input);
    run_positive(
        &program,
        &successor,
        &oracle,
        invoice,
        &invoice["final"],
        statements,
        case(&cases, "installment-final-80"),
    );
}

#[test]
fn approved_multiasset_reserve_matches_every_frozen_financial_prefix_and_total_work() {
    let oracle = financial_oracle();
    let cases = positive_cases();
    let reserve = &oracle["reserve"];
    let program = program(fixture!("conditional-reserve.mori"), &oracle);
    let scenario: Scenario =
        serde_json::from_str(fixture!("reserve-approved.scenario.json")).unwrap();
    run_positive(
        &program,
        &scenario,
        &oracle,
        reserve,
        reserve,
        &reserve["work"]["statements"],
        case(&cases, "approved-reserve-multiasset"),
    );
}

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

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Ump01Bindings {
    pub asset: AssetId,
    pub lender: String,
    pub borrower: String,
    pub principal_duty: String,
}

// Exact selected-fixture JSON integer types. Other decimal/signed rate or
// amount domains are deliberately outside this finite adapter's admission.
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Ump01Fields {
    pub event_date: String,
    pub event_type: String,
    pub payoff: i64,
    pub currency: String,
    pub notional_principal: u64,
    pub nominal_interest_rate: u64,
    pub accrued_interest: u64,
}

fn ump01_uint_duty_amount(execution: &Execution, bindings: &Ump01Bindings) -> Result<UInt, String> {
    execution
        .post
        .liabilities
        .iter()
        .find(|row| row.name == bindings.principal_duty)
        .map(|row| row.amount)
        .ok_or_else(|| "missing actual closing principal duty".into())
}

fn ump01_uint_state(execution: &Execution, name: &str) -> Result<UInt, String> {
    match execution.post.state.get(name) {
        Some(CoreValue::UInt(value)) => Ok(*value),
        _ => Err(format!("missing or non-UInt source state {name}")),
    }
}

fn ump01_integral_result(value: UInt, name: &str) -> Result<u64, String> {
    u64::try_from(value.0).map_err(|_| format!("{name} exceeds selected integer adapter"))
}

/// Project actual public effects/duties/state into the seven present fields.
/// This function reads no fixture results or expected oracle. It preserves no
/// claim that projection alone validates the complete execution: `observe`
/// returns the complete Execution for a separate trace comparison.
fn ump01_project(
    execution: &Execution,
    bindings: &Ump01Bindings,
) -> Result<Ump01Fields, String> {
    if execution.semantics != "moriarty-core/1" || execution.action != "initialExchange" {
        return Err("wrong selected profile/action".into());
    }
    let mut transfer = None;
    let mut event = None;
    let mut principal_effects = 0usize;
    for effect in &execution.effects {
        match effect {
            Effect::Transfer {
                asset,
                from,
                to,
                amount,
            } => {
                if transfer.is_some()
                    || asset != &bindings.asset
                    || from != &bindings.lender
                    || to != &bindings.borrower
                {
                    return Err("wrong or additional selected initial-exchange cashflow".into());
                }
                transfer = Some((asset.reference.clone(), *amount));
            }
            Effect::Liability {
                name,
                asset,
                debtor,
                creditor,
                before,
                after,
            } => {
                if name != &bindings.principal_duty
                    || asset != &bindings.asset
                    || debtor != &bindings.borrower
                    || creditor != &bindings.lender
                {
                    return Err("unexpected duty identity".into());
                }
                if before.0 != 0 || *after != ump01_uint_duty_amount(execution, bindings)? {
                    return Err("principal effect differs from actual closing duty".into());
                }
                principal_effects += 1;
            }
            Effect::Emit { name, value } => {
                if event.is_some() || name != "IED" {
                    return Err("wrong or additional selected event".into());
                }
                let CoreValue::UInt(time) = value else {
                    return Err("event timestamp is not UInt".into());
                };
                event = Some((name.clone(), *time));
            }
            Effect::State { .. } => {}
            _ => return Err("effect outside selected initial-exchange source".into()),
        }
    }
    if principal_effects != 1 {
        return Err("selected issuance requires one actual principal-duty effect".into());
    }
    let (currency, amount) = transfer.ok_or("missing actual cashflow")?;
    let (event_type, time) = event.ok_or("missing actual IED emit")?;
    if ump01_uint_state(execution, "statusTime")? != time {
        return Err("source status time differs from emitted time".into());
    }
    let duty = execution
        .post
        .liabilities
        .iter()
        .find(|row| row.name == bindings.principal_duty)
        .ok_or("missing actual outstanding principal duty")?;
    if duty.asset != bindings.asset
        || duty.debtor != bindings.borrower
        || duty.creditor != bindings.lender
    {
        return Err("closing duty does not retain exact selected identity".into());
    }
    if duty.amount != amount {
        return Err("principal duty differs from actual initial-exchange amount".into());
    }
    let magnitude = i64::try_from(amount.0)
        .map_err(|_| "payoff exceeds selected signed integer adapter")?;
    let payoff = magnitude.checked_neg().ok_or("signed payoff overflow")?;
    let digits = format!("{:014}", time.0);
    if digits.len() != 14 {
        return Err("timestamp outside supplied YYYYMMDDhhmmss encoding".into());
    }
    let event_date = format!(
        "{}-{}-{}T{}:{}:{}",
        &digits[0..4],
        &digits[4..6],
        &digits[6..8],
        &digits[8..10],
        &digits[10..12],
        &digits[12..14]
    );
    Ok(Ump01Fields {
        event_date,
        event_type,
        payoff,
        currency,
        notional_principal: ump01_integral_result(duty.amount, "notionalPrincipal")?,
        nominal_interest_rate: ump01_integral_result(
            ump01_uint_state(execution, "nominalInterestRate")?,
            "nominalInterestRate",
        )?,
        accrued_interest: ump01_integral_result(
            ump01_uint_state(execution, "accruedInterest")?,
            "accruedInterest",
        )?,
    })
}

/// Actual existing public consumer path; no fixture lookup or shadow evaluator.
/// The caller compares the returned complete trace against frozen literals.
fn ump01_observe(
    source: &str,
    scenario: &Scenario,
    bindings: &Ump01Bindings,
) -> Result<(Execution, Ump01Fields), String> {
    let program = mori_core::parse(source).map_err(|error| error.to_string())?;
    mori_core::validate(&program).map_err(|error| error.to_string())?;
    let execution = mori_core::simulate(&program, "initialExchange", scenario)
        .map_err(|error| error.to_string())?;
    let fields = ump01_project(&execution, bindings)?;
    Ok((execution, fields))
}

#[test]
fn actus_ump01_matches_independent_core_trace_and_seven_financial_fields() {
    use sha2::Digest;

    let core_json = fixture!("actus-ump01/independent/CORE-FINANCIAL-WORK.expected.json");
    let actus_json = fixture!("actus-ump01/independent/ACTUS-FINANCIAL-SCHEDULE.expected.json");
    let manifest_json = fixture!("actus-ump01/independent/INPUT-ONLY-MANIFEST.json");
    for (bytes, literal_sha256) in [
        (
            core_json,
            "6711c863fd3cfaf332de85660fb19ab24b7177c07f1bbdd36943e30408053e6e",
        ),
        (
            actus_json,
            "49126886ad8efdfce72252bcbe117634dd3a6b702fc00ff798af71eda5080ce7",
        ),
        (
            manifest_json,
            "b62e24c0040b319733f49d84f5be133a74df49313f5e251c6b7f886cf78455bd",
        ),
    ] {
        assert_eq!(
            mori_core::intent::hex(&sha2::Sha256::digest(bytes.as_bytes())),
            literal_sha256
        );
    }
    let expected: Value = serde_json::from_str(core_json).unwrap();
    let actus: Value = serde_json::from_str(actus_json).unwrap();
    let manifest: Value = serde_json::from_str(manifest_json).unwrap();
    let source = fixture!("actus-ump01/source.mori");
    let opening = fixture!("actus-ump01/opening.scenario.json");
    let bindings_json = fixture!("actus-ump01/subject-bindings.json");
    let operands_json = fixture!("actus-ump01/call-operands.json");
    let normative_json = fixture!("actus-ump01/ump01.normative-operands.json");
    for (name, input) in [
        ("inputs/source.mori", source),
        ("inputs/opening.scenario.json", opening),
        ("inputs/subject-bindings.json", bindings_json),
        ("inputs/call-operands.json", operands_json),
        ("inputs/ump01.normative-operands.json", normative_json),
    ] {
        let pin = manifest["files"]
            .as_array()
            .unwrap()
            .iter()
            .find(|pin| pin["path"] == name)
            .unwrap();
        assert_eq!(
            mori_core::intent::hex(&sha2::Sha256::digest(input.as_bytes())),
            pin["sha256"].as_str().unwrap()
        );
    }
    let bindings: Ump01Bindings = serde_json::from_str(bindings_json).unwrap();
    let scenario: Scenario = serde_json::from_str(opening).unwrap();
    let original = scenario.clone();
    let operands: Value = serde_json::from_str(operands_json).unwrap();
    let calls = operands["calls"].as_array().unwrap();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0]["action"], expected["action"]);
    assert_eq!(calls[0]["inputs"], expected["call_inputs"]);
    assert_eq!(
        serde_json::to_value(&scenario.inputs).unwrap(),
        expected["call_inputs"]
    );
    assert_eq!(operands["language"], expected["source_language"]);
    assert_eq!(operands["semantics"], expected["source_semantics"]);

    let (actual, fields) = ump01_observe(source, &scenario, &bindings).unwrap();
    assert_eq!(scenario, original);
    assert_eq!(actual.action, expected["action"].as_str().unwrap());
    assert_eq!(
        actual.semantics,
        expected["source_semantics"].as_str().unwrap()
    );
    assert_eq!(serde_json::to_value(&actual.pre).unwrap(), expected["pre"]);
    assert_eq!(serde_json::to_value(&actual.post).unwrap(), expected["post"]);
    assert_eq!(
        serde_json::to_value(&actual.effects).unwrap(),
        expected["ordered_effects"]
    );

    let expected_steps = expected["source_prefixes"].as_array().unwrap();
    assert_eq!(actual.steps.len(), expected_steps.len());
    let mut prefix: Vec<Value> = Vec::new();
    for (ordinal, (step, frozen)) in actual.steps.iter().zip(expected_steps).enumerate() {
        assert_eq!(usize::from(step.index), ordinal);
        assert_eq!(json!(step.index), frozen["source_ordinal"]);
        assert_eq!(
            serde_json::to_value(&step.value).unwrap(),
            frozen["typed_result"]
        );
        assert_eq!(
            serde_json::to_value(&step.snapshot).unwrap(),
            frozen["complete_snapshot"]
        );
        let delta = serde_json::to_value(&step.effects).unwrap();
        assert_eq!(delta, frozen["new_effects"]);
        prefix.extend(delta.as_array().unwrap().iter().cloned());
        assert_eq!(json!(prefix), frozen["ordered_effect_history"]);
    }
    assert_eq!(json!(prefix), expected["ordered_effects"]);

    let events = actus["ordered_known_scheduled_financial_events"]
        .as_array()
        .unwrap();
    assert_eq!(events.len(), 1);
    let event = &events[0];
    assert_eq!(fields.event_date, event["event_time"].as_str().unwrap());
    assert_eq!(fields.event_type, event["event_type"].as_str().unwrap());
    assert_eq!(
        fields.payoff.to_string(),
        event["payoff_in_contract_role_perspective"].as_str().unwrap()
    );
    assert_eq!(fields.currency, event["currency"].as_str().unwrap());
    assert_eq!(
        fields.notional_principal.to_string(),
        event["post_notionalPrincipal"].as_str().unwrap()
    );
    assert_eq!(
        fields.nominal_interest_rate.to_string(),
        expected["post"]["state"]["nominalInterestRate"]["value"]
            .as_str()
            .unwrap()
    );
    assert_eq!(
        fields.accrued_interest.to_string(),
        expected["post"]["state"]["accruedInterest"]["value"]
            .as_str()
            .unwrap()
    );
}
