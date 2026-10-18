//! Full-value local Core/2 JSON exports and continuation reopening.
use mori_core::stage::{self, StageOutcome};
use mori_core::{Scenario, UInt, intent, json};
use serde_json::json;

const SOURCE: &str = r#"language "moriarty/2";
agreement InvoiceReceipt {
    state principal: UInt = 0;
    asset CREDIT = {
        network: "local", issuer: "receipt-issuer", reference: "credit", scale: 3
    };
    action pay(quantity: UInt) {
        let owed = quantity + 3;
        liability("paid-invoice", CREDIT, "alice", "merchant", owed);
        transfer(CREDIT, "alice", "merchant", owed);
        fee(CREDIT, "alice", "treasury", 2);
        discharge("paid-invoice", owed);
        next principal = quantity;
        await receipt(accepted: Bool) when accepted;
        emit "present-zero"(liability_present("paid-invoice"));
        emit "absent"(liability_present("later-credit"));
        emit "closed-amount"(liability_amount("paid-invoice"));
    }
    action open_credit() {
        liability("later-credit", CREDIT, "merchant", "alice", 5);
    }
}"#;

const EXPECTED_WAITING_PREFIX: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../fixtures/invoice-waiting-prefix/expected-prefix.json"
));

#[test]
fn invoice_program_plan_and_zero_duty_continuation_roundtrip_as_complete_values() {
    let expected_prefix: serde_json::Value = serde_json::from_str(EXPECTED_WAITING_PREFIX).unwrap();
    let asset = json!({
        "network": "local", "issuer": "receipt-issuer", "reference": "credit", "scale": 3
    });
    // Literal expectations follow source declaration order and parser::push.
    // State initializers and asset scales do not allocate expression nodes.
    let expected_program = json!({
        "language": "moriarty/2",
        "semantics": "moriarty-core/2",
        "name": "InvoiceReceipt",
        "states": [{"name": "principal", "ty": "UInt",
            "initial": {"type": "UInt", "value": "0"}}],
        "assets": [{"name": "CREDIT", "identity": asset.clone()}],
        "expressions": [
            {"kind": "Name", "name": "quantity"},
            {"kind": "Literal", "value": {"type": "UInt", "value": "3"}},
            {"kind": "Binary", "op": "Add", "lhs": 0, "rhs": 1},
            {"kind": "Name", "name": "owed"},
            {"kind": "Name", "name": "owed"},
            {"kind": "Literal", "value": {"type": "UInt", "value": "2"}},
            {"kind": "Name", "name": "owed"},
            {"kind": "Name", "name": "quantity"},
            {"kind": "Name", "name": "accepted"},
            {"kind": "LiabilityPresent", "name": "paid-invoice"},
            {"kind": "LiabilityPresent", "name": "later-credit"},
            {"kind": "LiabilityAmount", "name": "paid-invoice"},
            {"kind": "Literal", "value": {"type": "UInt", "value": "5"}}
        ],
        "actions": [
            {"name": "pay", "inputs": [{"name": "quantity", "ty": "UInt"}],
             "statements": [
                {"kind": "Let", "name": "owed", "value": 2},
                {"kind": "Liability", "name": "paid-invoice", "asset": "CREDIT",
                    "debtor": "alice", "creditor": "merchant", "amount": 3},
                {"kind": "Transfer", "asset": "CREDIT", "from": "alice",
                    "to": "merchant", "amount": 4},
                {"kind": "Fee", "asset": "CREDIT", "from": "alice",
                    "to": "treasury", "amount": 5},
                {"kind": "Discharge", "name": "paid-invoice", "amount": 6},
                {"kind": "Next", "state": "principal", "value": 7},
                {"kind": "Await", "label": "receipt",
                    "binders": [{"name": "accepted", "ty": "Bool"}], "guard": 8},
                {"kind": "Emit", "name": "present-zero", "value": 9},
                {"kind": "Emit", "name": "absent", "value": 10},
                {"kind": "Emit", "name": "closed-amount", "value": 11}
             ]},
            {"name": "open_credit", "inputs": [], "statements": [
                {"kind": "Liability", "name": "later-credit", "asset": "CREDIT",
                    "debtor": "merchant", "creditor": "alice", "amount": 12}
            ]}
        ]
    });
    // Hashes use the production canonical codec over independent literal
    // values. This checks linkage, not an independent canonical-byte oracle.
    let expected_program_hash = intent::hash_bytes(&intent::canonical(&expected_program).unwrap());
    let expected_plan = json!({
        "format": "moriarty-stage-plan/1",
        "program_hash": expected_program_hash.clone(),
        "actions": [
            {"action": "pay", "statement_count": 10,
             "sites": [{"site": 1, "ordinal": 6, "label": "receipt",
                "binders": [{"name": "accepted", "ty": "Bool"}],
                "captures": [
                    {"name": "quantity", "ty": "UInt", "origin": "Input", "ordinal": 0},
                    {"name": "owed", "ty": "UInt", "origin": "Let", "ordinal": 0}
                ]}],
             "segment_cost": ["15", "8"],
             "committed_before": ["0", "15"], "plan_total": "23"},
            {"action": "open_credit", "statement_count": 1, "sites": [],
             "segment_cost": ["2"], "committed_before": ["0"], "plan_total": "2"}
        ]
    });
    let expected_plan_hash = intent::hash_bytes(&intent::canonical(&expected_plan).unwrap());
    let expected_continuation = json!({
        "format": "moriarty-continuation/1",
        "program_hash": expected_program_hash,
        "plan_hash": expected_plan_hash,
        "action": "pay", "site": 1,
        "captures": [
            {"name": "quantity", "value": {"type": "UInt", "value": "14"}},
            {"name": "owed", "value": {"type": "UInt", "value": "17"}}
        ],
        "logical": {"profile": "moriarty-reference-logical-cost/2",
            "cap": "23", "committed": "15"},
        "boundary": {
            "state": {"principal": {"type": "UInt", "value": "14"}},
            "balances": [
                {"account": "alice", "asset": asset.clone(), "amount": "31"},
                {"account": "merchant", "asset": asset.clone(), "amount": "17"},
                {"account": "observer", "asset": asset.clone(), "amount": "4"},
                {"account": "treasury", "asset": asset.clone(), "amount": "2"}
            ],
            // Discharge retains the zero row. The uninvoked action only
            // registers later-credit; it creates no boundary row.
            "liabilities": [{"name": "paid-invoice", "asset": asset.clone(),
                "debtor": "alice", "creditor": "merchant", "amount": "0"}]
        }
    });
    let scenario: Scenario = serde_json::from_value(json!({
        "state": {},
        "inputs": {"quantity": {"type": "UInt", "value": "14"}},
        "balances": [
            {"account": "alice", "asset": asset.clone(), "amount": "50"},
            {"account": "merchant", "asset": asset.clone(), "amount": "0"},
            {"account": "observer", "asset": asset.clone(), "amount": "4"},
            {"account": "treasury", "asset": asset, "amount": "0"}
        ],
        "liabilities": []
    }))
    .unwrap();

    let program = stage::load(SOURCE).unwrap();
    let program_text = json::export(program.program()).unwrap();
    let plan_text = json::export(program.plan()).unwrap();
    assert_eq!(json::parse(&program_text).unwrap(), expected_program);
    assert_eq!(json::parse(&plan_text).unwrap(), expected_plan);

    let StageOutcome::Waiting {
        advanced,
        segment,
        continuation,
        attempted,
        residual,
    } = stage::start(&program, "pay", scenario, Some(UInt(23))).unwrap()
    else {
        panic!("invoice prefix must wait for its receipt");
    };
    assert_eq!(advanced, expected_prefix["advanced"].as_bool().unwrap());
    assert_eq!(
        serde_json::to_value(attempted).unwrap(),
        expected_prefix["attempted"]
    );
    assert_eq!(
        serde_json::to_value(&residual).unwrap(),
        expected_prefix["residual"]
    );
    assert_eq!(
        serde_json::to_value(segment.expect("start must publish its complete prefix")).unwrap(),
        expected_prefix["segment"]
    );
    let continuation_text = json::export(&continuation).unwrap();
    let reopened = stage::decode::continuation(&continuation_text).unwrap();
    assert_eq!(
        json::parse(&continuation_text).unwrap(),
        expected_continuation
    );
    assert_eq!(reopened, continuation);
    assert_eq!(
        serde_json::to_value(&reopened).unwrap(),
        expected_continuation
    );
}
