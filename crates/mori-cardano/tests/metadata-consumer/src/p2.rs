use mori_cardano::{
    AccountBinding, AccountClass, DutyCell, ProjectionBody, ProjectionData, prepare_projection,
};
use mori_core::{Balance, Execution, Program, Scenario, UInt};
use serde_json::Value;

fn data(v: &Value) -> ProjectionData {
    if let Some(i) = v.get("integer") {
        assert_eq!(v.as_object().unwrap().len(), 1);
        ProjectionData::Integer(UInt(i.as_str().unwrap().parse().unwrap()))
    } else {
        assert_eq!(v.as_object().unwrap().len(), 2);
        ProjectionData::Constr {
            constructor: v["constructor"].as_u64().unwrap(),
            fields: v["fields"].as_array().unwrap().iter().map(data).collect(),
        }
    }
}

fn body(v: &Value) -> ProjectionBody {
    let states = v["state_values"]
        .as_array()
        .unwrap()
        .iter()
        .map(data)
        .collect();
    let framed = v["framed_amounts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| UInt(x.as_str().unwrap().parse().unwrap()))
        .collect();
    let duties = v["duties"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| {
            assert_eq!(x["constructor"], 0);
            let fields = x["fields"].as_array().unwrap();
            assert_eq!(fields.len(), 2);
            let present: u128 = fields[0]["integer"].as_str().unwrap().parse().unwrap();
            assert!(present <= 1);
            DutyCell::new(
                present == 1,
                UInt(fields[1]["integer"].as_str().unwrap().parse().unwrap()),
            )
            .unwrap()
        })
        .collect();
    ProjectionBody::new(states, framed, duties)
}

pub(super) fn bindings(text: &str) -> Vec<AccountBinding> {
    let j: Value = serde_json::from_str(text).unwrap();
    j["accounts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| AccountBinding {
            name: a["name"].as_str().unwrap().to_owned(),
            class: match a["class"].as_str().unwrap() {
                "ScriptCustodied" => AccountClass::ScriptCustodied,
                "KeyHeldExternal" => AccountClass::KeyHeldExternal,
                _ => panic!("Unknown class"),
            },
        })
        .collect()
}

fn verify(
    source: &str,
    expected_program: &str,
    binding_text: &str,
    case_text: &str,
    static_name: &str,
) {
    let expected: Value = serde_json::from_str(case_text).unwrap();
    let ctx = prepare_projection(source, &bindings(binding_text)).unwrap();
    let inspection = ctx.inspection();
    assert_eq!(
        inspection.source.program,
        serde_json::from_str::<Program>(expected_program).unwrap()
    );
    assert_eq!(
        inspection.format,
        "moriarty-cardano-projection-inspection/2"
    );
    assert_eq!(inspection.qualification, "UNIT");
    assert_eq!(
        inspection.binding_rule,
        "R5S-06 symbolic names; no physical credentials"
    );
    let expected_layout = match static_name {
        "invoice" => include_str!("../../fixtures/installment-invoice.expected-layout.json"),
        "reserve" => include_str!("../../fixtures/conditional-reserve.expected-layout.json"),
        _ => panic!("Unexpected independent source"),
    };
    assert_eq!(
        serde_json::to_value(&inspection.layout).unwrap(),
        serde_json::from_str::<Value>(expected_layout).unwrap()
    );

    let fixed: Value = serde_json::from_str(include_str!(
        "../../p2-fixtures/STATIC-CANDIDATE-EXPECTATIONS.json"
    ))
    .unwrap();
    assert_eq!(
        serde_json::to_value(&inspection.unreferenced_accounts).unwrap(),
        fixed[static_name]["unreferenced_accounts"]
    );
    assert_eq!(
        inspection.actions.len(),
        fixed[static_name]["actions"].as_array().unwrap().len()
    );
    for (actual, gold) in inspection
        .actions
        .iter()
        .zip(fixed[static_name]["actions"].as_array().unwrap())
    {
        assert_eq!(actual.action, gold["action"].as_str().unwrap());
        assert_eq!(
            actual.logical_work,
            UInt(gold["W"].as_str().unwrap().parse().unwrap())
        );
        assert_eq!(
            serde_json::to_value(&actual.required_signers).unwrap(),
            gold["required_signers"]
        );
    }
    let pre = body(&expected["pre_body"]);
    let post = body(&expected["post_body"]);
    assert_eq!(serde_json::to_value(&pre).unwrap(), expected["pre_body"]);
    assert_eq!(serde_json::to_value(&post).unwrap(), expected["post_body"]);
    let before: Vec<Balance> = serde_json::from_value(expected["premises_pre"].clone()).unwrap();
    let after: Vec<Balance> = serde_json::from_value(expected["premises_post"].clone()).unwrap();
    let inputs: Vec<_> = expected["typed_inputs"]
        .as_array()
        .unwrap()
        .iter()
        .map(data)
        .collect();
    assert_eq!(serde_json::to_value(&inputs).unwrap(), expected["typed_inputs"]);
    let action = expected["action"].as_str().unwrap();
    let scenario = ctx
        .scenario_from_body(&pre, &before, action, &inputs)
        .unwrap();
    let post_scenario = ctx
        .scenario_from_body(&post, &after, action, &inputs)
        .unwrap();
    assert_eq!(
        scenario,
        serde_json::from_value::<Scenario>(expected["expected_scenario_from_pre_body"].clone())
            .unwrap()
    );
    assert_eq!(
        post_scenario,
        serde_json::from_value::<Scenario>(expected["expected_scenario_from_post_body"].clone())
            .unwrap()
    );
    let execution: Execution =
        serde_json::from_value(expected["expected_completed_reference"]["execution"].clone())
            .unwrap();
    assert_eq!(
        serde_json::to_value(ctx.project_body(&execution.pre).unwrap()).unwrap(),
        expected["pre_body"]
    );
    assert_eq!(
        serde_json::to_value(ctx.project_body(&execution.post).unwrap()).unwrap(),
        expected["post_body"]
    );
    let actual = ctx
        .project_reference(&pre, &before, action, &inputs)
        .unwrap();
    assert_eq!(actual.execution, execution);
    assert_eq!(
        serde_json::to_value(actual).unwrap(),
        expected["expected_completed_reference"]
    );
    assert_eq!(
        serde_json::to_value(&before).unwrap(),
        expected["premises_pre"]
    );
}

macro_rules! invoice_case {
    ($name:ident,$file:literal) => {
        #[test]
        fn $name() {
            verify(
                include_str!("../../fixtures/installment-invoice.mori"),
                include_str!("../../fixtures/installment-invoice.expected-program.json"),
                include_str!("../../fixtures/installment-invoice.projection-bindings.json"),
                include_str!(concat!("../../p2-fixtures/", $file)),
                "invoice",
            );
        }
    };
}
macro_rules! reserve_case {
    ($name:ident,$file:literal) => {
        #[test]
        fn $name() {
            verify(
                include_str!("../../fixtures/conditional-reserve.mori"),
                include_str!("../../fixtures/conditional-reserve.expected-program.json"),
                include_str!("../../fixtures/conditional-reserve.projection-bindings.json"),
                include_str!(concat!("../../p2-fixtures/", $file)),
                "reserve",
            );
        }
    };
}
invoice_case!(invoice_open, "invoice-open.p2-candidate.json");
invoice_case!(invoice_partial, "invoice-partial.p2-candidate.json");
invoice_case!(invoice_finish, "invoice-finish.p2-candidate.json");
reserve_case!(reserve_allocate, "reserve-allocate.p2-candidate.json");
reserve_case!(reserve_rebate4, "reserve-rebate4.p2-candidate.json");
reserve_case!(reserve_rebate2, "reserve-rebate2.p2-candidate.json");
