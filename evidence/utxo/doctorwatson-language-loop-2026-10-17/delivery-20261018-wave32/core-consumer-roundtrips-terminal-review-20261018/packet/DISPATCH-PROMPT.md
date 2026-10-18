Actual fresh independent NONAUTHOR ordinary source/result review for PR229. Root inspected actual source, receipts, oracle provenance and this full-input scope; prepared-only sentences are historical. Return complete schema response with reviewToken core-consumer-roundtrips-review-30cc92bfe37e422f9e5bc58f38d23956. Requested AGY gemini-4-argon-high HIGH, actual backend identity unattested. Cover each of3complete changed test files, the changed guideline and all4test results with exact existing quotes/JSON pointers and substantive reasoning. Assess full finite candidate, distinguish independent financial oracles, authored wire expectations and shared codec linkage; separate source/result verdicts. Report every physical-only/unread source honestly; do not invent full22file reading. No commands, changes, security judgment, native admission or original parent waiver. Full actual UTXO value is losslessly aliased as explained below; inspect the alias and whole complete independent states/effects.

Ordinary source/result review input for exact PR229 candidate bcdfd4b27a62e8e618153ef1961d60f889dc4d63/tree6ef1a779f5c87299bf340d684cea33082cb1bcb2, direct parent b2786e8123e8d9327265f0fbb373444a7cb30f16. No provider review is dispatched by this packet. Root alone adopts any review and merges. All22changed physical files and actual raw receipts are preserved; CHANGED-SOURCE-PINS.json binds their Git blobs. All three complete test files below are actual Git HEAD, not pre-format source. Actual-source differences have separate full diffs and byte hashes; no canonical/token identity is claimed.

Review the full source/result fidelity, independent versus authored/shared-producer assertions, complete state/effects/field coverage and deliberate gaps, fixture provenance and pre-run custody, Rust guideline conformity, and whether any concrete ordinary source/result issue blocks adoption of this exact tested tree. A verdict concerns these finite four ordinary Core tests only. No security judgment is requested. Do not treat any hash or shared invariant as independent correspondence. The full Artifact/native candidate is UNRUN, absent fromPR229 and outside this review. SDK/proof/key/ledger acceptance and original U/M/MC/SP/G/finance/composition/UTXO exits remain open.

Root receipts record exit0,4passed0failed0filtered across the three exact named Core integration targets in26.81582211600471s, no Midnight compatibility feature. Actual UTXO Execution is captured in full. Its entire JSON Value equals authored.expected.json/execution, mechanically rechecked without product execution. To avoid duplicate output, that complete value appears once below inside authored.expected.json; actual-results/ACTUAL-UTXO-EXECUTION.json is losslessly represented by that exact object pointer, while its distinct original raw bytes and SHA remain physical. All independent11prefix snapshots, step/cumulative effects and pre/poststate appear in the separate complete independent expectation below; no semantic field/prefix was trimmed. Byte identity between those differently shaped files is not claimed. Actual Program/Plan/Continuation outputs were not separately captured; their assertions passed in the test log, and source literals are fully included. Stage Program/Plan linkage hashes use the production canonical codec on authored literals; the independent oracle explicitly leaves ExprIds and exact hashes/codecs unknown. Conditional resume is specified-only and not exercised by this target.

All32financial frozen payloads, all18new plan oracle members and all14new UTXO oracle members are physically supplied with original freezes unchanged. Only the two used full financial Program literals and their sources are inlined from that prior32payload packet. Unused financial outputs do not become test evidence. Full unchanged normative docs and full RUST_GUIDELINES are physical, not inlined; exact changed guideline diff follows. Unchanged production consumer sources are physical; INPUT-SCOPE.json identifies inlined versus available-but-not-forced-read files. No claim of reviewing unread transitive bodies is made. This is an ordinary evidence packet, not an execution or security admission.

## RUST-GUIDELINES.diff
Complete file.
```text
--- base/RUST_GUIDELINES.md
+++ head/RUST_GUIDELINES.md
@@ -249,6 +249,10 @@
   `crates/fixtures/financial-report-contract/`; expected values must not come
   from the evaluator, compiler output or generated baselines. This exception
   adds no authority to execute SDK checks or claim native/proof acceptance.
+- The finite Core `financial_program_wire_contract` target uses independently
+  frozen complete financial Program JSON and the public Program/v1 decoder
+  instead of generated snapshots. It selects no Midnight compatibility feature
+  and makes no compiler-Artifact, SDK, native-proof or ledger acceptance claim.
 - Test programs shared by several crates live in `crates/fixtures/`, which is
   excluded from the workspace. Nothing under `crates/` reads files outside it;
   copy anything needed in.

```

## changed/crates/mori-core/tests/financial_program_wire_contract.rs
Complete file.
```text
//! Complete independently frozen Core/1 wires, without selecting the SDK backend.
use mori_core::{Program, parse, v1};
use serde_json::Value;

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../fixtures/financial-report-contract/",
            $name
        ))
    };
}

fn consume(source: &str, frozen_program: &str) {
    let expected: Value = serde_json::from_str(frozen_program)
        .expect("independently frozen complete Program JSON must decode");
    let actual = parse(source).expect("exact source must parse through the real Core consumer");
    let bytes = serde_json::to_vec(&actual).expect("complete Program must serialize");
    let actual_wire: Value = serde_json::from_slice(&bytes).expect("complete wire must be JSON");
    assert_eq!(
        actual_wire, expected,
        "every Program field and nested cell must match"
    );

    // Actual shared public decoding routes through the frozen Core/1 DTO.
    let reopened: Program =
        serde_json::from_slice(&bytes).expect("public Program wire must reopen");
    assert_eq!(
        reopened, actual,
        "reopening must preserve the complete parsed Program"
    );
    assert_eq!(serde_json::to_value(&reopened).unwrap(), expected);

    // This is a codec/adapter check, separate from the independent equality above.
    let frozen_v1: v1::Program = serde_json::from_str(frozen_program).unwrap();
    let converted: Program = frozen_v1.into();
    assert_eq!(converted, actual);
}

#[test]
fn invoice_complete_core_wire_matches_frozen_32_node_program_and_reopens() {
    consume(
        fixture!("installment-invoice.mori"),
        fixture!("installment-invoice.program.expected.json"),
    );
}

#[test]
fn reserve_complete_core_wire_matches_frozen_33_node_program_and_reopens() {
    consume(
        fixture!("conditional-reserve.mori"),
        fixture!("conditional-reserve.program.expected.json"),
    );
}

```

## changed/crates/mori-core/tests/program_plan_roundtrip.rs
Complete file.
```text
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

#[test]
fn invoice_program_plan_and_zero_duty_continuation_roundtrip_as_complete_values() {
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

    let StageOutcome::Waiting { continuation, .. } =
        stage::start(&program, "pay", scenario, Some(UInt(23))).unwrap()
    else {
        panic!("invoice prefix must wait for its receipt");
    };
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

```

## changed/crates/mori-core/tests/utxo_pattern_exchange.rs
Complete file.
```text
use mori_core::{Scenario, parse, simulate};
use serde_json::Value;
use std::{env, fs::OpenOptions, io::Write};

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../fixtures/utxo-pattern-exchange/",
            $name
        ))
    };
}

#[test]
fn prefunded_exchange_matches_complete_authored_normal_outcome() {
    let program = parse(fixture!("program.mori")).expect("Core/1 example must parse");
    let scenario: Scenario = serde_json::from_str(fixture!("normal.scenario.json"))
        .expect("complete normal scenario must decode");
    let original_scenario = scenario.clone();
    let execution = simulate(&program, "exchange", &scenario)
        .expect("normal prefunded exchange must succeed locally");
    let actual = serde_json::to_value(&execution).expect("Execution must serialize");

    // Root can retain the complete observation before any expectation comparison.
    if let Some(path) = env::var_os("MORI_UTXO_PATTERN_EXECUTION_OUTPUT") {
        let mut bytes = serde_json::to_vec_pretty(&actual).expect("Execution JSON must encode");
        bytes.push(b'\n');
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .expect("explicit fresh Execution output must open");
        file.write_all(&bytes)
            .expect("complete Execution must write");
        file.sync_all().expect("Execution output must persist");
    }

    let independent: Value =
        serde_json::from_str(fixture!("independent/COMPLETE-PREFIXES.expected.json"))
            .expect("independent complete semantic expectation must decode");
    assert_eq!(
        fixture!("program.mori"),
        fixture!("independent/inputs/program.mori")
    );
    assert_eq!(
        fixture!("normal.scenario.json"),
        fixture!("independent/inputs/normal.scenario.json")
    );
    assert_eq!(
        program.language,
        independent["language"]
            .as_str()
            .expect("independent language")
    );
    assert_eq!(
        program.name,
        independent["agreement"]
            .as_str()
            .expect("independent agreement")
    );
    assert_eq!(actual["semantics"], independent["semantics"]);
    assert_eq!(actual["action"], independent["action"]);
    assert_eq!(
        actual["pre"], independent["pre"],
        "independent complete prestate"
    );
    assert_eq!(
        actual["post"], independent["post"],
        "independent complete poststate"
    );

    let actual_steps = actual["steps"].as_array().expect("execution steps");
    let independent_prefixes = independent["prefixes"]
        .as_array()
        .expect("independent complete prefixes");
    assert_eq!(actual_steps.len(), 11);
    assert_eq!(independent_prefixes.len(), 11);
    let mut cumulative_effects = Vec::new();
    for (index, (step, prefix)) in actual_steps.iter().zip(independent_prefixes).enumerate() {
        assert_eq!(
            step["index"], prefix["index"],
            "independent prefix ordinal {index}"
        );
        assert_eq!(
            step["snapshot"], prefix["snapshot"],
            "independent full snapshot at prefix {index}"
        );
        assert_eq!(
            step["effects"], prefix["stepEffects"],
            "independent ordered delta effects at prefix {index}"
        );
        cumulative_effects.extend(
            step["effects"]
                .as_array()
                .expect("ordered per-step effects")
                .iter()
                .cloned(),
        );
        assert_eq!(
            Value::Array(cumulative_effects.clone()),
            prefix["cumulativeEffects"],
            "independent ordered cumulative effects at prefix {index}"
        );
    }
    assert_eq!(actual["effects"].as_array().expect("all effects").len(), 6);
    assert_eq!(
        actual["effects"], independent["effects"],
        "independent complete effects"
    );
    assert_eq!(Value::Array(cumulative_effects), independent["effects"]);

    // The financial oracle does not specify expression IDs, Statement IR or Step.value.
    // Those complete wire fields remain checked against the distinct authored fixture.
    let authored: Value = serde_json::from_str(fixture!("authored.expected.json"))
        .expect("authored complete expectation must decode");
    let expected = &authored["execution"];
    assert_eq!(program.language, "moriarty/1");
    assert_eq!(program.semantics, "moriarty-core/1");
    assert_eq!(program.name, "PrefundedBilateralExchange");
    assert_eq!(program.actions.len(), 1);
    assert_eq!(program.actions[0].name, "exchange");
    assert_eq!(actual["semantics"], expected["semantics"]);
    assert_eq!(actual["action"], expected["action"]);
    assert_eq!(
        actual["pre"], expected["pre"],
        "complete normalized prestate"
    );
    assert_eq!(
        actual["post"], expected["post"],
        "complete normal poststate"
    );

    let expected_steps = expected["steps"].as_array().expect("authored steps");
    assert_eq!(expected_steps.len(), 11);
    for (index, (step, expected_step)) in actual_steps.iter().zip(expected_steps).enumerate() {
        assert_eq!(
            step, expected_step,
            "complete authored wire prefix at statement {index}"
        );
    }
    let actual_effects = actual["effects"].as_array().expect("execution effects");
    let expected_effects = expected["effects"].as_array().expect("authored effects");
    assert_eq!(actual_effects.len(), 6);
    assert_eq!(expected_effects.len(), 6);
    for (index, (effect, expected_effect)) in
        actual_effects.iter().zip(expected_effects).enumerate()
    {
        assert_eq!(
            effect, expected_effect,
            "complete authored ordered effect {index}"
        );
    }
    assert_eq!(
        &actual, expected,
        "all authored public Execution wire fields"
    );
    assert_eq!(
        scenario, original_scenario,
        "borrowed input remains unchanged"
    );
}

```

## financial-freeze/installment-invoice.mori
Complete file.
```text
language "moriarty/1";
agreement InstallmentInvoice {
  asset CASH = { network: "local", issuer: "financial-pair-invoice", reference: "CASH", scale: 2 };
  state paid: UInt = 0;
  state dueDay: UInt = 30;
  state installments: UInt = 0;
  state marker: UInt = 77;
  action register() {
    liability("invoice", CASH, "payer", "seller", 127);
    liability("warranty", CASH, "payer", "auditor", 9);
  }
  action collect(amount: UInt, atDay: UInt) {
    requires amount > 0;
    requires atDay >= dueDay;
    requires liability_present("invoice") and liability_amount("invoice") >= amount;
    let opening = liability_amount("invoice");
    transfer(CASH, "payer", "seller", amount);
    fee(CASH, "payer", "treasury", 3);
    discharge("invoice", amount);
    next paid = paid + amount;
    next dueDay = dueDay + 30;
    next installments = installments + 1;
    ensures liability_amount("invoice") == opening - amount;
    emit "collected"(paid);
  }
}

```

## financial-freeze/conditional-reserve.mori
Complete file.
```text
language "moriarty/1";
agreement ConditionalReserve {
  asset CASH = { network: "local", issuer: "financial-pair-reserve", reference: "CASH", scale: 2 };
  asset BOND = { network: "local", issuer: "financial-pair-reserve", reference: "BOND", scale: 0 };
  state allocated: UInt = 0;
  state closed: Bool = false;
  state marker: UInt = 77;
  action register() {
    liability("allocation", CASH, "reserve", "supplier", 0);
    liability("bond", BOND, "reserve", "auditor", 10);
    liability("emergency", CASH, "reserve", "emergency", 2);
  }
  action allocate(approved: Bool, amount: UInt, bondAmount: UInt) {
    requires approved and not closed;
    requires amount > 0;
    requires amount <= 50;
    let residual = (57 - amount) - 2;
    requires residual >= 5;
    liability("allocation", CASH, "reserve", "supplier", amount);
    transfer(CASH, "reserve", "supplier", amount);
    fee(CASH, "reserve", "treasury", 2);
    discharge("allocation", amount);
    transfer(BOND, "reserve", "auditor", bondAmount);
    discharge("bond", bondAmount);
    next allocated = amount;
    next closed = true;
    ensures liability_amount("allocation") == 0;
    emit "allocated"(allocated);
  }
}

```

## financial-freeze/installment-invoice.program.expected.json
Complete JSON Value; whitespace compacted only.
```json
{"actions":[{"inputs":[],"name":"register","statements":[{"amount":0,"asset":"CASH","creditor":"seller","debtor":"payer","kind":"Liability","name":"invoice"},{"amount":1,"asset":"CASH","creditor":"auditor","debtor":"payer","kind":"Liability","name":"warranty"}]},{"inputs":[{"name":"amount","ty":"UInt"},{"name":"atDay","ty":"UInt"}],"name":"collect","statements":[{"kind":"Requires","predicate":4},{"kind":"Requires","predicate":7},{"kind":"Requires","predicate":12},{"kind":"Let","name":"opening","value":13},{"amount":14,"asset":"CASH","from":"payer","kind":"Transfer","to":"seller"},{"amount":15,"asset":"CASH","from":"payer","kind":"Fee","to":"treasury"},{"amount":16,"kind":"Discharge","name":"invoice"},{"kind":"Next","state":"paid","value":19},{"kind":"Next","state":"dueDay","value":22},{"kind":"Next","state":"installments","value":25},{"kind":"Ensures","predicate":30},{"kind":"Emit","name":"collected","value":31}]}],"assets":[{"identity":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"name":"CASH"}],"expressions":[{"kind":"Literal","value":{"type":"UInt","value":"127"}},{"kind":"Literal","value":{"type":"UInt","value":"9"}},{"kind":"Name","name":"amount"},{"kind":"Literal","value":{"type":"UInt","value":"0"}},{"kind":"Binary","lhs":2,"op":"Gt","rhs":3},{"kind":"Name","name":"atDay"},{"kind":"Name","name":"dueDay"},{"kind":"Binary","lhs":5,"op":"Ge","rhs":6},{"kind":"LiabilityPresent","name":"invoice"},{"kind":"LiabilityAmount","name":"invoice"},{"kind":"Name","name":"amount"},{"kind":"Binary","lhs":9,"op":"Ge","rhs":10},{"kind":"Binary","lhs":8,"op":"And","rhs":11},{"kind":"LiabilityAmount","name":"invoice"},{"kind":"Name","name":"amount"},{"kind":"Literal","value":{"type":"UInt","value":"3"}},{"kind":"Name","name":"amount"},{"kind":"Name","name":"paid"},{"kind":"Name","name":"amount"},{"kind":"Binary","lhs":17,"op":"Add","rhs":18},{"kind":"Name","name":"dueDay"},{"kind":"Literal","value":{"type":"UInt","value":"30"}},{"kind":"Binary","lhs":20,"op":"Add","rhs":21},{"kind":"Name","name":"installments"},{"kind":"Literal","value":{"type":"UInt","value":"1"}},{"kind":"Binary","lhs":23,"op":"Add","rhs":24},{"kind":"LiabilityAmount","name":"invoice"},{"kind":"Name","name":"opening"},{"kind":"Name","name":"amount"},{"kind":"Binary","lhs":27,"op":"Sub","rhs":28},{"kind":"Binary","lhs":26,"op":"Eq","rhs":29},{"kind":"Name","name":"paid"}],"language":"moriarty/1","name":"InstallmentInvoice","semantics":"moriarty-core/1","states":[{"initial":{"type":"UInt","value":"0"},"name":"paid","ty":"UInt"},{"initial":{"type":"UInt","value":"30"},"name":"dueDay","ty":"UInt"},{"initial":{"type":"UInt","value":"0"},"name":"installments","ty":"UInt"},{"initial":{"type":"UInt","value":"77"},"name":"marker","ty":"UInt"}]}
```

## financial-freeze/conditional-reserve.program.expected.json
Complete JSON Value; whitespace compacted only.
```json
{"actions":[{"inputs":[],"name":"register","statements":[{"amount":0,"asset":"CASH","creditor":"supplier","debtor":"reserve","kind":"Liability","name":"allocation"},{"amount":1,"asset":"BOND","creditor":"auditor","debtor":"reserve","kind":"Liability","name":"bond"},{"amount":2,"asset":"CASH","creditor":"emergency","debtor":"reserve","kind":"Liability","name":"emergency"}]},{"inputs":[{"name":"approved","ty":"Bool"},{"name":"amount","ty":"UInt"},{"name":"bondAmount","ty":"UInt"}],"name":"allocate","statements":[{"kind":"Requires","predicate":6},{"kind":"Requires","predicate":9},{"kind":"Requires","predicate":12},{"kind":"Let","name":"residual","value":17},{"kind":"Requires","predicate":20},{"amount":21,"asset":"CASH","creditor":"supplier","debtor":"reserve","kind":"Liability","name":"allocation"},{"amount":22,"asset":"CASH","from":"reserve","kind":"Transfer","to":"supplier"},{"amount":23,"asset":"CASH","from":"reserve","kind":"Fee","to":"treasury"},{"amount":24,"kind":"Discharge","name":"allocation"},{"amount":25,"asset":"BOND","from":"reserve","kind":"Transfer","to":"auditor"},{"amount":26,"kind":"Discharge","name":"bond"},{"kind":"Next","state":"allocated","value":27},{"kind":"Next","state":"closed","value":28},{"kind":"Ensures","predicate":31},{"kind":"Emit","name":"allocated","value":32}]}],"assets":[{"identity":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"name":"CASH"},{"identity":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"name":"BOND"}],"expressions":[{"kind":"Literal","value":{"type":"UInt","value":"0"}},{"kind":"Literal","value":{"type":"UInt","value":"10"}},{"kind":"Literal","value":{"type":"UInt","value":"2"}},{"kind":"Name","name":"approved"},{"kind":"Name","name":"closed"},{"kind":"Not","value":4},{"kind":"Binary","lhs":3,"op":"And","rhs":5},{"kind":"Name","name":"amount"},{"kind":"Literal","value":{"type":"UInt","value":"0"}},{"kind":"Binary","lhs":7,"op":"Gt","rhs":8},{"kind":"Name","name":"amount"},{"kind":"Literal","value":{"type":"UInt","value":"50"}},{"kind":"Binary","lhs":10,"op":"Le","rhs":11},{"kind":"Literal","value":{"type":"UInt","value":"57"}},{"kind":"Name","name":"amount"},{"kind":"Binary","lhs":13,"op":"Sub","rhs":14},{"kind":"Literal","value":{"type":"UInt","value":"2"}},{"kind":"Binary","lhs":15,"op":"Sub","rhs":16},{"kind":"Name","name":"residual"},{"kind":"Literal","value":{"type":"UInt","value":"5"}},{"kind":"Binary","lhs":18,"op":"Ge","rhs":19},{"kind":"Name","name":"amount"},{"kind":"Name","name":"amount"},{"kind":"Literal","value":{"type":"UInt","value":"2"}},{"kind":"Name","name":"amount"},{"kind":"Name","name":"bondAmount"},{"kind":"Name","name":"bondAmount"},{"kind":"Name","name":"amount"},{"kind":"Literal","value":{"type":"Bool","value":true}},{"kind":"LiabilityAmount","name":"allocation"},{"kind":"Literal","value":{"type":"UInt","value":"0"}},{"kind":"Binary","lhs":29,"op":"Eq","rhs":30},{"kind":"Name","name":"allocated"}],"language":"moriarty/1","name":"ConditionalReserve","semantics":"moriarty-core/1","states":[{"initial":{"type":"UInt","value":"0"},"name":"allocated","ty":"UInt"},{"initial":{"type":"Bool","value":false},"name":"closed","ty":"Bool"},{"initial":{"type":"UInt","value":"77"},"name":"marker","ty":"UInt"}]}
```

## changed/crates/fixtures/utxo-pattern-exchange/program.mori
Complete file.
```text
// Ordinary local Core/1 example. These account labels and supplied balances
// do not authenticate escrow ownership, signatories, or any ledger state.
language "moriarty/1";
agreement PrefundedBilateralExchange {
  asset A = { network: "local", issuer: "example-bilateral", reference: "A", scale: 0 };
  asset B = { network: "local", issuer: "example-bilateral", reference: "B", scale: 0 };
  state phase: UInt = 0;
  state revision: UInt = 0;
  state marker: UInt = 77;

  action exchange(quotedA: UInt, quotedB: UInt, serviceFee: UInt) {
    requires phase == 0;
    requires quotedA == 10;
    requires quotedB == 25;
    requires serviceFee == 1;
    transfer(A, "escrow-a", "trader", quotedA);
    transfer(B, "escrow-b", "provider", quotedB);
    fee(A, "fee-payer", "treasury", serviceFee);
    next phase = 1;
    next revision = revision + 1;
    ensures phase == 1;
    emit "exchanged"(quotedA);
  }
}

```

## changed/crates/fixtures/utxo-pattern-exchange/normal.scenario.json
Complete JSON Value; whitespace compacted only.
```json
{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"inputs":{"quotedA":{"type":"UInt","value":"10"},"quotedB":{"type":"UInt","value":"25"},"serviceFee":{"type":"UInt","value":"1"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"10"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"5"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"2"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"25"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"4"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]}
```

## changed/crates/fixtures/utxo-pattern-exchange/authored.expected.json
Complete JSON Value; whitespace compacted only.
```json
{"format":"moriarty.authored-normal-execution-expectation/1","expectationStatus":"AUTHORED_EXPECTATION_NOT_INDEPENDENTLY_FROZEN","subjectExecution":"UNRUN","consumerRevision":"e4d7a2b5709caa18a94f5256d03a0302364b8235","provenance":"This source-preparation agent authored the financial expectations and candidate regression. No independent oracle author, review, freeze, parse, validation or simulation is claimed.","representation":"The current parser appends expression nodes in source order. Literal expected ExprId references are source-derived; neither parser nor compiler was executed to obtain them.","execution":{"semantics":"moriarty-core/1","action":"exchange","pre":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"10"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"5"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"2"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"25"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"4"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"post":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"1"},"revision":{"type":"UInt","value":"5"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"0"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"4"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"13"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"0"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"29"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"steps":[{"index":0,"statement":{"kind":"Requires","predicate":2},"value":{"type":"Bool","value":true},"snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"10"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"5"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"2"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"25"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"4"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"effects":[]},{"index":1,"statement":{"kind":"Requires","predicate":5},"value":{"type":"Bool","value":true},"snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"10"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"5"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"2"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"25"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"4"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"effects":[]},{"index":2,"statement":{"kind":"Requires","predicate":8},"value":{"type":"Bool","value":true},"snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"10"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"5"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"2"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"25"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"4"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"effects":[]},{"index":3,"statement":{"kind":"Requires","predicate":11},"value":{"type":"Bool","value":true},"snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"10"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"5"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"2"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"25"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"4"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"effects":[]},{"index":4,"statement":{"kind":"Transfer","asset":"A","from":"escrow-a","to":"trader","amount":12},"value":{"type":"UInt","value":"10"},"snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"0"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"5"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"13"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"2"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"25"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"4"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"effects":[{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"escrow-a","to":"trader","amount":"10"}]},{"index":5,"statement":{"kind":"Transfer","asset":"B","from":"escrow-b","to":"provider","amount":13},"value":{"type":"UInt","value":"25"},"snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"0"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"5"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"13"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"2"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"0"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"29"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"effects":[{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"from":"escrow-b","to":"provider","amount":"25"}]},{"index":6,"statement":{"kind":"Fee","asset":"A","from":"fee-payer","to":"treasury","amount":14},"value":{"type":"UInt","value":"1"},"snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"0"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"4"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"13"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"0"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"29"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"effects":[{"kind":"Fee","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"fee-payer","to":"treasury","amount":"1"}]},{"index":7,"statement":{"kind":"Next","state":"phase","value":15},"value":{"type":"UInt","value":"1"},"snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"1"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"0"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"4"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"13"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"0"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"29"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"effects":[{"kind":"State","name":"phase","before":{"type":"UInt","value":"0"},"after":{"type":"UInt","value":"1"}}]},{"index":8,"statement":{"kind":"Next","state":"revision","value":18},"value":{"type":"UInt","value":"5"},"snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"1"},"revision":{"type":"UInt","value":"5"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"0"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"4"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"13"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"0"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"29"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"effects":[{"kind":"State","name":"revision","before":{"type":"UInt","value":"4"},"after":{"type":"UInt","value":"5"}}]},{"index":9,"statement":{"kind":"Ensures","predicate":21},"value":{"type":"Bool","value":true},"snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"1"},"revision":{"type":"UInt","value":"5"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"0"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"4"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"13"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"0"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"29"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"effects":[]},{"index":10,"statement":{"kind":"Emit","name":"exchanged","value":22},"value":{"type":"UInt","value":"10"},"snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"1"},"revision":{"type":"UInt","value":"5"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"0"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"4"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"13"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"0"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"29"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"effects":[{"kind":"Emit","name":"exchanged","value":{"type":"UInt","value":"10"}}]}],"effects":[{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"escrow-a","to":"trader","amount":"10"},{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"from":"escrow-b","to":"provider","amount":"25"},{"kind":"Fee","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"fee-payer","to":"treasury","amount":"1"},{"kind":"State","name":"phase","before":{"type":"UInt","value":"0"},"after":{"type":"UInt","value":"1"}},{"kind":"State","name":"revision","before":{"type":"UInt","value":"4"},"after":{"type":"UInt","value":"5"}},{"kind":"Emit","name":"exchanged","value":{"type":"UInt","value":"10"}}]}}
```

## changed/crates/fixtures/utxo-pattern-exchange/independent/COMPLETE-PREFIXES.expected.json
Complete JSON Value; whitespace compacted only.
```json
{"format":"independent-normal-core-complete-prefix-oracle/1","language":"moriarty/1","semantics":"moriarty-core/1","agreement":"PrefundedBilateralExchange","action":"exchange","expectedLocalStatus":"accepted","pre":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"10"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"5"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"2"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"25"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"4"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"prefixes":[{"index":0,"sourceStatement":"requires phase == 0;","snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"10"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"5"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"2"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"25"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"4"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"stepEffects":[],"cumulativeEffects":[]},{"index":1,"sourceStatement":"requires quotedA == 10;","snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"10"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"5"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"2"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"25"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"4"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"stepEffects":[],"cumulativeEffects":[]},{"index":2,"sourceStatement":"requires quotedB == 25;","snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"10"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"5"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"2"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"25"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"4"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"stepEffects":[],"cumulativeEffects":[]},{"index":3,"sourceStatement":"requires serviceFee == 1;","snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"10"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"5"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"2"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"25"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"4"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"stepEffects":[],"cumulativeEffects":[]},{"index":4,"sourceStatement":"transfer(A, \"escrow-a\", \"trader\", quotedA);","snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"0"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"5"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"13"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"2"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"25"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"4"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"stepEffects":[{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"escrow-a","to":"trader","amount":"10"}],"cumulativeEffects":[{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"escrow-a","to":"trader","amount":"10"}]},{"index":5,"sourceStatement":"transfer(B, \"escrow-b\", \"provider\", quotedB);","snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"0"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"5"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"13"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"2"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"0"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"29"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"stepEffects":[{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"from":"escrow-b","to":"provider","amount":"25"}],"cumulativeEffects":[{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"escrow-a","to":"trader","amount":"10"},{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"from":"escrow-b","to":"provider","amount":"25"}]},{"index":6,"sourceStatement":"fee(A, \"fee-payer\", \"treasury\", serviceFee);","snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"0"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"4"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"13"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"0"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"29"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"stepEffects":[{"kind":"Fee","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"fee-payer","to":"treasury","amount":"1"}],"cumulativeEffects":[{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"escrow-a","to":"trader","amount":"10"},{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"from":"escrow-b","to":"provider","amount":"25"},{"kind":"Fee","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"fee-payer","to":"treasury","amount":"1"}]},{"index":7,"sourceStatement":"next phase = 1;","snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"1"},"revision":{"type":"UInt","value":"4"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"0"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"4"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"13"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"0"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"29"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"stepEffects":[{"kind":"State","name":"phase","before":{"type":"UInt","value":"0"},"after":{"type":"UInt","value":"1"}}],"cumulativeEffects":[{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"escrow-a","to":"trader","amount":"10"},{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"from":"escrow-b","to":"provider","amount":"25"},{"kind":"Fee","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"fee-payer","to":"treasury","amount":"1"},{"kind":"State","name":"phase","before":{"type":"UInt","value":"0"},"after":{"type":"UInt","value":"1"}}]},{"index":8,"sourceStatement":"next revision = revision + 1;","snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"1"},"revision":{"type":"UInt","value":"5"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"0"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"4"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"13"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"0"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"29"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"stepEffects":[{"kind":"State","name":"revision","before":{"type":"UInt","value":"4"},"after":{"type":"UInt","value":"5"}}],"cumulativeEffects":[{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"escrow-a","to":"trader","amount":"10"},{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"from":"escrow-b","to":"provider","amount":"25"},{"kind":"Fee","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"fee-payer","to":"treasury","amount":"1"},{"kind":"State","name":"phase","before":{"type":"UInt","value":"0"},"after":{"type":"UInt","value":"1"}},{"kind":"State","name":"revision","before":{"type":"UInt","value":"4"},"after":{"type":"UInt","value":"5"}}]},{"index":9,"sourceStatement":"ensures phase == 1;","snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"1"},"revision":{"type":"UInt","value":"5"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"0"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"4"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"13"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"0"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"29"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"stepEffects":[],"cumulativeEffects":[{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"escrow-a","to":"trader","amount":"10"},{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"from":"escrow-b","to":"provider","amount":"25"},{"kind":"Fee","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"fee-payer","to":"treasury","amount":"1"},{"kind":"State","name":"phase","before":{"type":"UInt","value":"0"},"after":{"type":"UInt","value":"1"}},{"kind":"State","name":"revision","before":{"type":"UInt","value":"4"},"after":{"type":"UInt","value":"5"}}]},{"index":10,"sourceStatement":"emit \"exchanged\"(quotedA);","snapshot":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"1"},"revision":{"type":"UInt","value":"5"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"0"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"4"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"13"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"0"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"29"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"stepEffects":[{"kind":"Emit","name":"exchanged","value":{"type":"UInt","value":"10"}}],"cumulativeEffects":[{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"escrow-a","to":"trader","amount":"10"},{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"from":"escrow-b","to":"provider","amount":"25"},{"kind":"Fee","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"fee-payer","to":"treasury","amount":"1"},{"kind":"State","name":"phase","before":{"type":"UInt","value":"0"},"after":{"type":"UInt","value":"1"}},{"kind":"State","name":"revision","before":{"type":"UInt","value":"4"},"after":{"type":"UInt","value":"5"}},{"kind":"Emit","name":"exchanged","value":{"type":"UInt","value":"10"}}]}],"post":{"state":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"1"},"revision":{"type":"UInt","value":"5"}},"balances":[{"account":"escrow-a","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"0"},{"account":"fee-payer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"4"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"9"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"13"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"amount":"3"},{"account":"escrow-b","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"0"},{"account":"observer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"11"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"29"},{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"amount":"8"}],"liabilities":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}]},"effects":[{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"escrow-a","to":"trader","amount":"10"},{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"from":"escrow-b","to":"provider","amount":"25"},{"kind":"Fee","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"fee-payer","to":"treasury","amount":"1"},{"kind":"State","name":"phase","before":{"type":"UInt","value":"0"},"after":{"type":"UInt","value":"1"}},{"kind":"State","name":"revision","before":{"type":"UInt","value":"4"},"after":{"type":"UInt","value":"5"}},{"kind":"Emit","name":"exchanged","value":{"type":"UInt","value":"10"}}]}
```

## changed/crates/fixtures/utxo-pattern-exchange/independent/DERIVATION.md
Complete file.
```text
The supplied nonempty state is marker77,phase0,revision4. Action inputs are quotedA10,quotedB25,serviceFee1. All four requires are true, so prefix0..3 exactly preserve the complete opening snapshot.

At index4, the A transfer moves10 from escrow-a10 to trader3: escrow-a becomes0 and trader13. All other seven balance rows and both duties stay unchanged. At index5, the B transfer moves25 from escrow-b25 to provider4: escrow-b becomes0 and provider29; all A rows and B observer11/trader8 remain. At index6, the A fee moves1 from fee-payer5 to treasury2, giving4 and3. It does not erase either zero escrow row or alter any duty.

At index7 phase becomes1. At index8 revision reads the scenario/current4 and adds1, becoming5. Marker77 never changes. Index9 checks phase1 and adds no effect. Index10 emits exchanged with UInt10 and leaves all state/balances/duties unchanged. This gives11after-statement complete snapshots and6ordered effects: TransferA10,TransferB25,FeeA1,Statephase0to1,Staterevision4to5,Emit10.

AssetA total29 and assetB total48 are preserved at every prefix. Gross A debit11 consists of transfer10 plus fee1; gross B debit25 is the second transfer. service-B2 and warranty-A7 remain present through every prefix because the program neither creates nor discharges liabilities. No row or residual duty disappears. No external fee, DUST, reserve or native execution claim is inferred.

Complete snapshots/effect payloads are hand-derived literal vectors serialized with Python standard library only. No product parser/evaluator/compiler was imported or run, and no authored expected assertion was read. Python was used only to assemble the literal repeated full snapshots, serialize JSON and verify hashes/cardinalities/conservation. No source execution/operation interpreter was implemented.

```

## changed/crates/fixtures/utxo-pattern-exchange/independent/FINANCIAL-ACCOUNTING.expected.json
Complete JSON Value; whitespace compacted only.
```json
{"assetA":{"identity":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"openingTotal":"29","closingTotal":"29","ordinaryTransferGrossDebit":"10","feeGrossDebit":"1","combinedGrossDebit":"11","fee":"1","reserveDust":"unasserted; no distinct dust concept/input","balances":[{"account":"escrow-a","opening":"10","closing":"0","presentAfter":true},{"account":"fee-payer","opening":"5","closing":"4","presentAfter":true},{"account":"observer","opening":"9","closing":"9","presentAfter":true},{"account":"trader","opening":"3","closing":"13","presentAfter":true},{"account":"treasury","opening":"2","closing":"3","presentAfter":true}]},"assetB":{"identity":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"openingTotal":"48","closingTotal":"48","ordinaryTransferGrossDebit":"25","feeGrossDebit":"0","combinedGrossDebit":"25","fee":"0","balances":[{"account":"escrow-b","opening":"25","closing":"0","presentAfter":true},{"account":"observer","opening":"11","closing":"11","presentAfter":true},{"account":"provider","opening":"4","closing":"29","presentAfter":true},{"account":"trader","opening":"8","closing":"8","presentAfter":true}]},"recipientNetDeltas":[{"account":"trader","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"net":"10"},{"account":"provider","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"net":"25"},{"account":"treasury","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"net":"1"}],"liabilities":{"opening":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}],"created":[],"accrued":[],"discharged":[],"closing":[{"name":"service-B","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"debtor":"provider","creditor":"auditor","amount":"2"},{"name":"warranty-A","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"debtor":"provider","creditor":"trader","amount":"7"}],"presence":"Both opening rows remain present and positive through every prefix; no zero/absence transition occurs.","allUnlistedLiabilities":"Absent; no statement creates a liability."},"state":{"opening":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"0"},"revision":{"type":"UInt","value":"4"}},"closing":{"marker":{"type":"UInt","value":"77"},"phase":{"type":"UInt","value":"1"},"revision":{"type":"UInt","value":"5"}},"scenarioInitializerOverride":"Scenario revision4 is used; declared revision0 does not replace nonempty complete supplied state."},"frame":"Exactly all9balance rows, all3state fields, all2duty rows retained at every prefix. No additional balance/asset/duty rows are expected."}
```

## changed/crates/fixtures/utxo-pattern-exchange/independent/ORDERED-EFFECTS.expected.json
Complete JSON Value; whitespace compacted only.
```json
[{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"escrow-a","to":"trader","amount":"10"},{"kind":"Transfer","asset":{"network":"local","issuer":"example-bilateral","reference":"B","scale":0},"from":"escrow-b","to":"provider","amount":"25"},{"kind":"Fee","asset":{"network":"local","issuer":"example-bilateral","reference":"A","scale":0},"from":"fee-payer","to":"treasury","amount":"1"},{"kind":"State","name":"phase","before":{"type":"UInt","value":"0"},"after":{"type":"UInt","value":"1"}},{"kind":"State","name":"revision","before":{"type":"UInt","value":"4"},"after":{"type":"UInt","value":"5"}},{"kind":"Emit","name":"exchanged","value":{"type":"UInt","value":"10"}}]
```

## changed/crates/fixtures/utxo-pattern-exchange/independent/SEMANTIC-PREMISES-AND-LIMITS.json
Complete JSON Value; whitespace compacted only.
```json
{"basisCommit":"e4d7a2b5709caa18a94f5256d03a0302364b8235","matchingConsumerCommit":"823ce081c00786a0c55b92af81009f9e7c5e6a16","normativeFinancialRules":[{"basis":"core1-normative-readme.md","rule":"Nonempty scenario state supplies exactly all declared values; names resolve in statement order; next updates immediately; requires/ensures checked where written."},{"basis":"core1-normative-readme.md","rule":"Exactasset transfer debits/credits explicit account rows; fee has same balances with distinct Fee effect; no mint/burn/implicit account creation."},{"basis":"core1-normative-readme.md","rule":"Liabilities evolve only by explicit liability/discharge; no such statement appears, so all supplied duties persist."},{"basis":"core1-snapshot-effect-dto.rs.txt","rule":"Expected Snapshot fields state/balances/liabilities and complete State/Transfer/Fee/Emit effect payloads use canonical UInt strings."}],"presentationPremise":{"custodian":"/root/dwu_audit_ledger_native, input-only custody","sourceFactOnly":"runtime.rs scenario_snapshot line631 sorts balances by(asset,account); line664 sorts liabilities byname. AssetId Ord follows network,issuer,reference,scale in ir.rs. This nonnumeric ordering fact was received from sibling; no evaluator transition body/output was read.","expectedOrder":"A escrow-a,fee-payer,observer,trader,treasury; B escrow-b,observer,provider,trader; duties service-B thenwarranty-A.","losslessAlternativeComparison":"A comparator may key balances by full(asset,account) and duties byname only after asserting exactunique fullidentities/cardinalities; effects and prefixes stay ordered."},"selectedRoute":{"consumer":"mori_core::parse -> unmetered mori_core::simulate","logicalCap":null,"measuredLogicalCost":"unasserted: no meter selected","SDKDustNativeOrServiceFeeMeasurement":"unasserted; source Fee(A,1) is local token accounting only"},"notAsserted":["Parser ExprId/Statement IR encoding","Step.value and other evaluator diagnostic presentation not specified by the financial Snapshot/effect oracle","Compilation/proof/nativeSDK/ledger settlement","Any of20pattern families closure or proof of pattern ownership/ledger authority","Negative/hostile cases, atomic rollback on failure or generallanguage equivalence"],"status":"Independent expected financial semantics for one normal admitted program; not execution evidence and no security judgment"}
```

## changed/crates/fixtures/utxo-pattern-exchange/independent/PRE-INPUT-EXPOSURE.json
Complete JSON Value; whitespace compacted only.
```json
{"createdAt":"2026-10-18T13:04:58.229904+00:00","task":"Independent normal complete-prefix oracle for unapplied UTXO pattern real Core consumer","inputCustody":"Awaiting physically separate input-only packet from sibling /root/dwu_audit_ledger_native. No candidate program/test/expected assertions have been read for this task.","normativeBasisCommit":"e4d7a2b5709caa18a94f5256d03a0302364b8235","priorExposure":["Earlier unrelated DevEx init.rs generated financial expectations were accidentally seen and disqualified that earlier blind task.","Prior Core1/Core2/Reserve and Source6 DTO/normative/parser/static-validation and ordinary-result transport source exposure, including Reserve model/oracle/projection expectations.","Current candidate UTXO pattern test expectations, actual outputs and program have not been read by this custodian before its input-only custody packet."],"limits":"No global blindness claim; independent derivation is specific to this new candidate from isolated input-only literals plus normative semantics. No evaluator/compiler import or execution, no security judgment/provider/build/Git/shared changes."}
```

## changed/crates/fixtures/utxo-pattern-exchange/independent/FREEZE.json
Complete JSON Value; whitespace compacted only.
```json
{"frozenAt":"2026-10-18T13:07:05.179678+00:00","status":"FROZEN_BEFORE_ROOT_ACTUAL_EXECUTION_AND_BEFORE_AUTHORED_EXPECTATION_READ","inputOnlyCustodyPath":"/home/charl/moriarty-c2-opus55-restored-20261017/utxo-pattern-core-integration-ready-20261018/ORACLE-INPUT-ONLY","programSha256":"2d12b38d13b950ed4c787ba78c9b613fedb94739ed812a2a815c34224355a25d","scenarioSha256":"b9542be7f0004647c7e5a354aad20505f600e315d74ddf0411a068987d2be341","inputCustodySha256":"fa0209a819a4471b5a258541755f66243626e604fc8c085a1990d47a0c0d982f","method":"Hand/spec financial arithmetic and explicit full-state literal schedule; stdlib serialization/hashes only","prefixCount":11,"initialSnapshotsAdditional":1,"effectCount":6,"balanceRowsEverySnapshot":9,"stateFieldsEverySnapshot":3,"liabilityRowsEverySnapshot":2,"priorExposure":"See PRE-INPUT-EXPOSURE.json. Extensive related priorCore/Reserve/Source6 material is disclosed; no globalblindness claim. This candidate test/expected assertions/actualoutputs have not been read. Current nonnumeric runtime rowordering sourcefact supplied by sibling is disclosed.","productImportsOrExecutions":false,"providerDispatches":0,"authoredCandidateAssertionsRead":false,"securityJudgment":null,"all20Families":"Remain separate and OPEN; this is one normal Core consumer predicate."}
```

## changed/crates/fixtures/utxo-pattern-exchange/independent/FREEZE.sha256
Complete file.
```text
da77426bf72e639f497af9b7ea56c9b2acefc07959c6690749d0817475eb456a  COMPLETE-PREFIXES.expected.json
b0c717ef8823532b6cf35947ce9055353c22d07f9bbc391a9c3510468e4ec7e2  DERIVATION.md
20b8950797835291ecf0d88662688d2a8828eeb7cbe1189033120d2176ccfb19  FINANCIAL-ACCOUNTING.expected.json
d4a07adbabb8dc9abbe3bd6ca1291da32e13d811c6c875f4f6c9fa6e67b1db63  FREEZE.json
545f0ffc877b492a8a03cb93ff8bce5de94884e13cd1bf1321fa66db13c4b07a  ORDERED-EFFECTS.expected.json
bab5c7db2e8d6b7cc83ca0be0b29aad3f643f9f4d0bb2d9914681e09c98bffc4  PRE-INPUT-EXPOSURE.json
da18a0c36bbf352e1f614ecc18ef8a4091193c02bbb97d213f477a8b4630a336  SEMANTIC-PREMISES-AND-LIMITS.json
0d91facda4bcf06e545af2f9ced7c72b2b76220a9b53eb46a5a4caea66ebc4dc  core1-normative-readme.md
06906e225e48618cc6f57a0a44fe8b678cba39b538edab308d79297ade9fba5c  core1-snapshot-effect-dto.rs.txt
fa0209a819a4471b5a258541755f66243626e604fc8c085a1990d47a0c0d982f  inputs/INPUT-ONLY.json
b9542be7f0004647c7e5a354aad20505f600e315d74ddf0411a068987d2be341  inputs/normal.scenario.json
2d12b38d13b950ed4c787ba78c9b613fedb94739ed812a2a815c34224355a25d  inputs/program.mori
d5674c650741103284e0aa8cded4c0e8a0a1be38faa0c65aedb043710caa4396  product-contract.md
2899b2c3c1c1c42a17d089997ecb756dc0e4007697ad2fea711d6aaa254cc82f  MANIFEST.json

```

## independent/program-plan-roundtrip-independent-oracle-20261018/inputs/program.mori
Complete file.
```text
language "moriarty/2";
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
}
```

## independent/program-plan-roundtrip-independent-oracle-20261018/inputs/scenario.json
Complete JSON Value; whitespace compacted only.
```json
{"state":{},"inputs":{"quantity":{"type":"UInt","value":"14"}},"balances":[{"account":"alice","asset":{"network":"local","issuer":"receipt-issuer","reference":"credit","scale":3},"amount":"50"},{"account":"merchant","asset":{"network":"local","issuer":"receipt-issuer","reference":"credit","scale":3},"amount":"0"},{"account":"observer","asset":{"network":"local","issuer":"receipt-issuer","reference":"credit","scale":3},"amount":"4"},{"account":"treasury","asset":{"network":"local","issuer":"receipt-issuer","reference":"credit","scale":3},"amount":"0"}],"liabilities":[]}
```

## independent/program-plan-roundtrip-independent-oracle-20261018/DERIVATION.md
Complete file.
```text
# Independent derivation

Selected input is pay(quantity UInt14), cap23, stopping at first await. Empty scenario state selects principal UInt0. All CREDIT amounts are atoms; scale3 is identity metadata, not a multiplication. owed=14+3=17.

Let preserves the complete snapshot. Liability inserts paid-invoice17, debtor alice and creditor merchant. Transfer subtracts17 from alice50 and adds17 to merchant0: alice33/merchant17. Fee subtracts2 from alice33 and adds2 to treasury0: alice31/treasury2. Observer4 is preserved throughout. Discharge subtracts17 from duty17 and retains its named zero row. Next sets principal14. Thus payer total debit19 consists of transfer17 plus fee2; recipient increase17 and treasury increase2. The four supplied balances total54 before and after. This is finite arithmetic, not a general conservation/security judgment.

At await receipt, no guard evaluation or Await Step occurs on entry. Complete boundary has principal14, balances31/17/4/2 (alice/merchant/observer/treasury), and paid-invoice present0. later-credit remains absent: its declaration in an unused action supplies the read identity but does not execute that action. Positive residual list is empty while the complete duty row is retained. Inputs and lets are captures, not state: quantity14 then owed17. The pending accepted binder is excluded.

Documented work has fresh expression occurrences and a per-statement cache. Let visits quantity, UInt3, Add, plus dispatch:4. Liability, Transfer, Fee, Discharge and Next each visit one expression plus dispatch:2 each. Await entry adds1, total15. The conditional accepted guard visits Name accepted plus dispatch:2. Each of three suffix Emits visits one liability-read leaf plus dispatch:2, so the conditional second segment is8 and full successful total23. False guard attempt2 does not increment committed15. No actual event supplied; suffix consequences are conditional. open_credit has one dispatch plus one literal node=2; this plan-only quantity does not execute it.

Exact global ExprIds, Program/Plan hashes/codecs and target are intentionally unknown. No evaluator/compiler/product imports or outputs, authored expectation arrays or test assertions were used. Previous implementation and report-design exposure is disclosed in provenance.

```

## independent/program-plan-roundtrip-independent-oracle-20261018/EXPECTED-SEMANTICS.json
Complete JSON Value; whitespace compacted only.
```json
{"format":"independent-program-plan-roundtrip-semantic-oracle/1","action":"pay","quantity":"14","owed":"17","asset":{"network":"local","issuer":"receipt-issuer","reference":"credit","scale":3},"amountUnits":"integral atomic units; scale3 does not rescale amounts","grossDebit":"19","transferGross":"17","fee":"2","merchantNetIncrease":"17","treasuryIncrease":"2","aliceNetDecrease":"19","openingBalanceSum":"54","closingBalanceSum":"54","createdDuty":"17","dischargedDuty":"17","remainingPositiveDuties":[],"presentZeroDuty":{"name":"paid-invoice","asset":{"network":"local","issuer":"receipt-issuer","reference":"credit","scale":3},"debtor":"alice","creditor":"merchant","amount":"0"},"absentDutyName":"later-credit","opening":{"state":{"principal":{"type":"UInt","value":"0"}},"balances":[{"account":"alice","asset":{"network":"local","issuer":"receipt-issuer","reference":"credit","scale":3},"amount":"50"},{"account":"merchant","asset":{"network":"local","issuer":"receipt-issuer","reference":"credit","scale":3},"amount":"0"},{"account":"observer","asset":{"network":"local","issuer":"receipt-issuer","reference":"credit","scale":3},"amount":"4"},{"account":"treasury","asset":{"network":"local","issuer":"receipt-issuer","reference":"credit","scale":3},"amount":"0"}],"liabilities":[]},"closingAtAwait":{"state":{"principal":{"type":"UInt","value":"14"}},"balances":[{"account":"alice","asset":{"network":"local","issuer":"receipt-issuer","reference":"credit","scale":3},"amount":"31"},{"account":"merchant","asset":{"network":"local","issuer":"receipt-issuer","reference":"credit","scale":3},"amount":"17"},{"account":"observer","asset":{"network":"local","issuer":"receipt-issuer","reference":"credit","scale":3},"amount":"4"},{"account":"treasury","asset":{"network":"local","issuer":"receipt-issuer","reference":"credit","scale":3},"amount":"2"}],"liabilities":[{"name":"paid-invoice","asset":{"network":"local","issuer":"receipt-issuer","reference":"credit","scale":3},"debtor":"alice","creditor":"merchant","amount":"0"}]},"startEffectCount":5,"startStepCount":6,"capturedValues":[{"name":"quantity","value":{"type":"UInt","value":"14"}},{"name":"owed","value":{"type":"UInt","value":"17"}}],"await":{"site":1,"sourceOrdinal":6,"label":"receipt","guardNotEvaluatedAtStart":true},"prefixFile":"EXPECTED-PREFIXES.json","logicalFile":"EXPECTED-LOGICAL-WORK.json","captureFile":"EXPECTED-CAPTURE-LAYOUT.json","qualification":"Semantic expected local prefix, conditional on the documented normal admitted start; actual program/plan/continuation serialization is not independently established."}
```

## independent/program-plan-roundtrip-independent-oracle-20261018/EXPECTED-CAPTURE-LAYOUT.json
Complete JSON Value; whitespace compacted only.
```json
{"action":"pay","statement_count":10,"sites":[{"site":0,"captures":[]},{"site":1,"label":"receipt","ordinal":6,"binders":[{"name":"accepted","ty":"Bool"}],"captures":[{"name":"quantity","ty":"UInt","origin":"Input","ordinal":0},{"name":"owed","ty":"UInt","origin":"Let","ordinal":0}]}],"frame_pre":{"site":0,"captures":[]},"frame_post":{"site":1,"captures":[{"name":"quantity","value":{"type":"UInt","value":"14"}},{"name":"owed","value":{"type":"UInt","value":"17"}}]},"continuationCaptures":[{"name":"quantity","value":{"type":"UInt","value":"14"}},{"name":"owed","value":{"type":"UInt","value":"17"}}],"excludedFromCaptures":["principal","accepted"],"residual":[],"completeBoundaryLiabilities":[{"name":"paid-invoice","asset":{"network":"local","issuer":"receipt-issuer","reference":"credit","scale":3},"debtor":"alice","creditor":"merchant","amount":"0"}],"note":"Input and Let origin enums distinguish their equal ordinal zero. Pending event binder not yet captured. State read from boundary."}
```

## independent/program-plan-roundtrip-independent-oracle-20261018/EXPECTED-LOGICAL-WORK.json
Complete JSON Value; whitespace compacted only.
```json
{"profile":"moriarty-reference-logical-cost/2","cap":"23","scope":"Adopted finite successful-prefix reference schedule, not target/circuit/native resource admission. Fresh expression occurrence nodes as documented; no exact ExprId or hash derived.","start":{"statements":[{"sourceOrdinal":0,"kind":"Let","distinctVisitedExpressionNodes":3,"expressionDescription":"Name(quantity), Literal(UInt3), Binary(Add)","dispatch":"1","logicalUnits":"4"},{"sourceOrdinal":1,"kind":"Liability","distinctVisitedExpressionNodes":1,"expressionDescription":"Name(owed)","dispatch":"1","logicalUnits":"2"},{"sourceOrdinal":2,"kind":"Transfer","distinctVisitedExpressionNodes":1,"expressionDescription":"Name(owed)","dispatch":"1","logicalUnits":"2"},{"sourceOrdinal":3,"kind":"Fee","distinctVisitedExpressionNodes":1,"expressionDescription":"Literal(UInt2)","dispatch":"1","logicalUnits":"2"},{"sourceOrdinal":4,"kind":"Discharge","distinctVisitedExpressionNodes":1,"expressionDescription":"Name(owed)","dispatch":"1","logicalUnits":"2"},{"sourceOrdinal":5,"kind":"Next","distinctVisitedExpressionNodes":1,"expressionDescription":"Name(quantity)","dispatch":"1","logicalUnits":"2"}],"ordinaryStatementUnits":"14","awaitEntryUnits":"1","awaitGuardVisited":false,"attempted":"15","committed":"15","capHeadroom":"8"},"conditionalResume":{"eventAbsentFromFrozenInputs":true,"trueGuard":{"dispatch":"1","guardNodes":1,"units":"2"},"suffix":[{"sourceOrdinal":7,"kind":"Emit","expression":"liability_present(\"paid-invoice\")","dispatch":"1","expressionNodes":1,"units":"2"},{"sourceOrdinal":8,"kind":"Emit","expression":"liability_present(\"later-credit\")","dispatch":"1","expressionNodes":1,"units":"2"},{"sourceOrdinal":9,"kind":"Emit","expression":"liability_amount(\"paid-invoice\")","dispatch":"1","expressionNodes":1,"units":"2"}],"trueAttempted":"8","trueCommitted":"23","falseAttempted":"2","falseCommitted":"15"},"actionPlans":[{"action":"pay","segment_cost":["15","8"],"committed_before":["0","15"],"plan_total":"23"},{"action":"open_credit","segment_cost":["2"],"committed_before":["0"],"plan_total":"2","basis":"One Liability statement: dispatch1 + Literal(UInt5) node1; no execution selected."}],"notEstablished":["actual metering","cap refusal policy or precedence","authenticated lifetime budget","circuit cost","DUST or target fees","allocation or wall time"]}
```

## independent/program-plan-roundtrip-independent-oracle-20261018/UNKNOWN.json
Complete JSON Value; whitespace compacted only.
```json
{"notIndependentlyEstablished":["Whole Program exact expressions array and ExprIds","Exact serialized statement reference fields","Program or StagePlan canonical codec or hashes","Target/Artifact/compiled relation","Actual parser/load/start/roundtrip outputs and serialization","Actual process exit/stdout/stderr or library JSON bytes","Continuation admission/authentication/history/security conditions","Actual logical meter conformance or resource admission","Concrete resume event: none supplied","Any root tests or assertions not named in INPUT-ONLY.json"],"noInference":"No runtime, native, target, proof, settlement or broad acceptance claim; all parent exits remain unchanged."}
```

## independent/program-plan-roundtrip-independent-oracle-20261018/FREEZE.sha256
Complete file.
```text
bb45ff17e0a8c800ebb2b8b08f8015a4c2a57196aa7d195bea3986833eaae8f0  CONDITIONAL-RESUME.json
1c52d14b25360f0b10db9e0dd3e908c629cc41d9ecdf9ea6aa9adfa8ecdee6af  DERIVATION.md
a79f0927efff34e7e181b4bda859d70bf184871d883699e9998da1973fbccb9c  EXPECTED-CAPTURE-LAYOUT.json
54f8c9ef50a9aea5e0fcdc554dfd25edd9a4f76da2e0db5c461ba8b3347a2a6d  EXPECTED-LOGICAL-WORK.json
6c4897e35bf0b7b29529fb2c622f9919c88dfe0b8fcfad94215505831310ebe8  EXPECTED-PREFIXES.json
78b54a00b117a7f9e09bba50cb459c26aca519004d8535653bb218829ed7296a  EXPECTED-SEMANTICS.json
189d4a943b44c43eecccf922d671b921f15238e3bcb891cd8daed6d12873e2f9  MANIFEST.json
609a9e089797d6f9c773fa6f659d66232e59deec2255244d1c53fd5c41412d8b  PROVENANCE.json
d317d7d51b1572064737040df1f3e6eb9514203fc9ce2508806c4bbeed2c4bcd  README.md
57fef1f112b4f1e8488168bc0164d801f0ca3e03f82122dde734da54b914c41f  STATIC-CHECKS.json
5558d3ad4726788127cbaca0d988fed982cdc4f10b4127905ac5ef6269b85e9b  UNKNOWN.json
52e8102175451a5f759a6e3a4bdb1f942f77ab0aaf1b74a55b7a283934baf503  basis/CORE-CONTINUATIONS.normative.txt
bfad57b3f0ef9cbeb48acc0e7e33b860c92bf6568bb818880d22ea4c3d879384  basis/CORE-EXECUTION-REPORTS.financial.txt
45a40dacb7dab23bb7abf6c4384bf16335116f0f86e700af758d2aae80813adc  basis/CORE-README.normative.txt
f16ccddf29ebe717c1a6c201fc80d16a201afb1b0ca54cd015189a52fe9dbc29  basis/CORE-STAGED-TESTS.normative.txt
540691a84eb308b5557b42812116291954a72ac536f07bc3344f86eaf226d477  inputs/INPUT-ONLY.json
21a191ca9a3cee026cdbdb6bf1604101bd035c2c885b068f64f2d5dcb1db362e  inputs/program.mori
732f67e6599d55d52c081b41dcf8ce74079223bb51d804c1fbb349d2356a8c2d  inputs/scenario.json

```

## actual-results/CHECK-PLAN.json
Complete JSON Value; whitespace compacted only.
```json
{"recordedAt":"2026-10-18T13:18:35.420663+00:00","head":"bcdfd4b27a62e8e618153ef1961d60f889dc4d63","tree":"6ef1a779f5c87299bf340d684cea33082cb1bcb2","base":"b2786e8123e8d9327265f0fbb373444a7cb30f16","argv":["/usr/bin/timeout","--signal=TERM","--kill-after=5s","90s","/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo","test","--manifest-path","/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/Cargo.toml","--frozen","-j1","-p","mori-core","--no-default-features","--test","program_plan_roundtrip","--test","utxo_pattern_exchange","--test","financial_program_wire_contract","--","--test-threads=1"],"environment":{"PATH":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin","RUSTC":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc","RUSTDOC":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc","CARGO_HOME":"/tmp/doctorwatson-core-roundtrips-rtsd00nt/cargo-home","CARGO_TARGET_DIR":"/tmp/doctorwatson-core-roundtrips-rtsd00nt/target","CARGO_NET_OFFLINE":"true","CARGO_INCREMENTAL":"0","CARGO_CACHE_AUTO_CLEAN_FREQUENCY":"never","GIT_CONFIG_NOSYSTEM":"1","GIT_CONFIG_GLOBAL":"/dev/null","GIT_CONFIG_SYSTEM":"/dev/null","LANG":"C.UTF-8","TMPDIR":"/tmp/doctorwatson-core-roundtrips-rtsd00nt/tmp","MORI_UTXO_PATTERN_EXECUTION_OUTPUT":"/home/charl/moriarty-c2-opus55-restored-20261017/core-consumer-roundtrips-check-cycle-20261018/ACTUAL-UTXO-EXECUTION.json"},"ownedTemporaryRoot":"/tmp/doctorwatson-core-roundtrips-rtsd00nt","absentAncestorConfigs":["/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/.cargo/config","/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/.cargo/config.toml","/home/charl/Moriarty/.worktrees/.cargo/config","/home/charl/Moriarty/.worktrees/.cargo/config.toml","/home/charl/Moriarty/.cargo/config","/home/charl/Moriarty/.cargo/config.toml","/home/charl/.cargo/config","/home/charl/.cargo/config.toml","/home/.cargo/config","/home/.cargo/config.toml","/.cargo/config","/.cargo/config.toml","/tmp/doctorwatson-core-roundtrips-rtsd00nt/cwd/.cargo/config","/tmp/doctorwatson-core-roundtrips-rtsd00nt/cwd/.cargo/config.toml","/tmp/doctorwatson-core-roundtrips-rtsd00nt/.cargo/config","/tmp/doctorwatson-core-roundtrips-rtsd00nt/.cargo/config.toml","/tmp/.cargo/config","/tmp/.cargo/config.toml"],"toolPins":[{"path":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo","sha256":"e951141cc55a6cd7b9876d187bd30a3720e6086b99413af95e3b8de1cdd72f14"},{"path":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc","sha256":"f3834d26669b03f6855fa54bd2381443123e860167bc0c7a8dd137e5cf6e5e4f"},{"path":"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc","sha256":"a1321a60578a6b2e3e0ff428201b8618bbd9a9fbcd475c0d0f151bbf17cb0a9e"},{"path":"/usr/bin/timeout","sha256":"f84e76ea4da1e0c4e92254bd0d7dccdabe2c8b3098e734f27997dae3ee5d8e02"}],"independentFreezesVerifiedBeforeRun":[{"path":"/home/charl/moriarty-c2-opus55-restored-20261017/utxo-pattern-core-independent-oracle-20261018","freezeSha256":"9f4ba8ddac07c4823eb2199e0ce2866e54b3c424221ce2cea3172a9660063d00","membersVerified":14},{"path":"/home/charl/moriarty-c2-opus55-restored-20261017/program-plan-roundtrip-independent-oracle-20261018","freezeSha256":"904f438ad11cdcee5698b62973cbdae3d977c53275f86427cdecc42b366589f4","membersVerified":18}],"scope":"Three actual ordinary Core integration targets, four tests; mandatory original compiler/SDK/native/proof/target/financial-family acceptance and consumed SDK allocations unchanged. No native or midnight-compat-tests feature selected.","expectation":"Program/Plan/Continuation whole values; two financial full Program/v1 wires; UTXO complete11prefixes/6effects/preserved duties. No prior passing target repeated."}
```

## actual-results/START.json
Complete JSON Value; whitespace compacted only.
```json
{"startedAt":"2026-10-18T13:18:35.465878+00:00","head":"bcdfd4b27a62e8e618153ef1961d60f889dc4d63","tree":"6ef1a779f5c87299bf340d684cea33082cb1bcb2","argv":["/usr/bin/timeout","--signal=TERM","--kill-after=5s","90s","/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo","test","--manifest-path","/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/Cargo.toml","--frozen","-j1","-p","mori-core","--no-default-features","--test","program_plan_roundtrip","--test","utxo_pattern_exchange","--test","financial_program_wire_contract","--","--test-threads=1"]}
```

## actual-results/CHILD.json
Complete JSON Value; whitespace compacted only.
```json
{"pid":746950,"session":746950}
```

## actual-results/RESULT.json
Complete JSON Value; whitespace compacted only.
```json
{"finishedAt":"2026-10-18T13:19:02.281669+00:00","exitCode":0,"wallSeconds":26.81582211600471,"stdoutSha256":"8a14a44c39058e61d7a99ee09ea746d901f12c785c7230f49474d6fcb61aa0d6","stderrSha256":"266ecbf592befa971d4060369fb5702aa6ab795df7f5099119d5de2d315e5362","ownedSessionRemaining":[],"head":"bcdfd4b27a62e8e618153ef1961d60f889dc4d63","tree":"6ef1a779f5c87299bf340d684cea33082cb1bcb2","sourcePinsUnchanged":true,"workingTreeClean":true}
```

## actual-results/STDOUT.log
Complete file.
```text

running 2 tests
test invoice_complete_core_wire_matches_frozen_32_node_program_and_reopens ... ok
test reserve_complete_core_wire_matches_frozen_33_node_program_and_reopens ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test invoice_program_plan_and_zero_duty_continuation_roundtrip_as_complete_values ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test prefunded_exchange_matches_complete_authored_normal_outcome ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


```

## actual-results/CLEANUP.json
Complete JSON Value; whitespace compacted only.
```json
{"recordedAt":"2026-10-18T13:19:02.367099+00:00","ownedRoot":"/tmp/doctorwatson-core-roundtrips-rtsd00nt","existsAfter":false,"resultRecorded":true}
```

## actual-results/ORACLE-COMPARISON.json
Complete JSON Value; whitespace compacted only.
```json
{"actualExecutionSha256":"cd6e7eac012ab85e41885e39b75a5ffaeceb118d13082bee730aaeaac6c24cf3","oracleSha256":"da77426bf72e639f497af9b7ea56c9b2acefc07959c6690749d0817475eb456a","completePrefixCount":11,"orderedEffectCount":6,"completePrePostAndAllPrefixSnapshotsAndDeltaCumulativeEffectsEqual":true,"limits":"One local finite exchange; Statement/ExprId/Step.value only separately authored expectations. No all20family or target/compiler/native qualification."}
```

## DATA-ONLY-COMPARISONS.json
Complete JSON Value; whitespace compacted only.
```json
{"dataOnly":true,"actualExecutionEqualsAuthoredExecutionAsWholeJsonValue":true,"rawByteIdentityClaimed":false,"actualBytesSha256":"cd6e7eac012ab85e41885e39b75a5ffaeceb118d13082bee730aaeaac6c24cf3","authoredBytesSha256":"3f02d30c00d5c480857c9a76cb4bf258d3adc72f884e99789c9abc792911b8d6","completeIndependentPrePostAnd11PrefixSnapshotsAndDeltaCumulativeEffectsEqual":true,"orderedEffectCount":6,"planTestInputEqualsIndependentInputBytes":true,"planTestInputEqualsIndependentInputIgnoringOneFinalNewline":true,"planActualFullOutcomeCaptured":false,"planLimit":"Test asserts exported complete Program/Plan/Continuation values against authored source literals; independent freeze separately derives boundary/captures/work. It intentionally leaves global ExprIds and canonical hashes/codecs unknown; production canonical helper computes expected linkage hashes. Conditional resume oracle is not executed by this target.","financialWireFullExpectedJsons":"Exact existing independently frozen Program literals,32/33nodes; public decode/v1 checks are codec consistency.","wholePublicSdkArtifactCandidate":"UNRUN and absent fromPR229","runtimeExecutedHere":false}
```

## Exact production consumer excerpts: crates/mori-core/src/ir.rs
```text
179: pub struct Program {
180:     pub language: String,
181:     pub semantics: String,
182:     pub name: String,
183:     pub states: Vec<StateDecl>,
184:     pub assets: Vec<AssetDecl>,
185:     pub expressions: Vec<Expr>,
186:     pub actions: Vec<Action>,
187: }
188: #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]

418: impl From<v1::Program> for Program {
419:     fn from(p: v1::Program) -> Self {
420:         Self {
421:             language: p.language,
422:             semantics: p.semantics,
423:             name: p.name,
424:             states: p.states,
425:             assets: p.assets,
426:             expressions: p.expressions,
427:             actions: p.actions.into_iter().map(Into::into).collect(),
428:         }
429:     }
430: }

456: macro_rules! decode_via_v1 {
457:     ($($ty:ident),*) => {$(
458:         impl<'de> Deserialize<'de> for $ty {
459:             fn deserialize<D: serde::Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
460:                 v1::$ty::deserialize(decoder).map(Self::from)
461:             }
462:         }
463:     )*};
464: }
465: decode_via_v1!(Statement, Action, Program, Step, Execution);
```

## Exact production consumer excerpts: crates/mori-core/src/json.rs
```text
87: pub fn parse(text: &str) -> Result<Value, Error> {
88:     parse_inner(text, MAX_BYTES, "JSON exceeds 1 MiB")
89: }

117: pub fn export(value: &impl serde::Serialize) -> Result<String, Error> {
118:     let json =
119:         serde_json::to_string_pretty(value).map_err(|e| Error::new("json", e.to_string()))?;
120:     parse(&json)?;
121:     let mut display = String::with_capacity(json.len());
122:     for character in json.chars() {
123:         if character.is_ascii() && (!character.is_control() || character == '\n') {
124:             display.push(character);
125:         } else {
126:             for unit in character.encode_utf16(&mut [0; 2]) {
127:                 use std::fmt::Write;
128:                 write!(&mut display, "\\u{unit:04x}").expect("writing to String cannot fail");
129:             }
130:         }
131:         if display.len() >= MAX_BYTES {
132:             return Err(Error::new(
133:                 "json",
134:                 "Escaped JSON and final newline exceed 1 MiB",
135:             ));
136:         }
137:     }
138:     parse(&display)?;
139:     Ok(display)
140: }
```

## Exact production consumer excerpts: crates/mori-core/src/stage/mod.rs
```text
75: pub fn load(src: &str) -> Result<StagedProgram, Error> {
76:     let program = crate::parser::parse_v2(src)?;
77:     profile::require_core2(&program)?;
78:     validate::validate_v2(&program)?;
79:     let mut plan = plan::build(&program)?;
80:     validate::plan_invariants(&program, &plan)?;
81:     cost::fill(&program, &mut plan)?;
82:     validate::cost_invariants(&program, &plan)?;
83:     let program_hash = digest::program_hash(&program)?;
84:     plan.program_hash = digest::hex(&program_hash);
85:     let plan_hash = digest::plan_hash(&plan)?;
86:     Ok(StagedProgram {
87:         program,
88:         plan,
89:         program_hash,
90:         plan_hash,
91:     })
92: }

110: /// frame; `boundary` is the legacy Snapshot. Nothing here is authenticated.
111: #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
112: #[serde(deny_unknown_fields)]
113: pub struct Continuation {
114:     pub format: String,
115:     pub program_hash: String,
116:     pub plan_hash: String,
117:     pub action: String,
118:     pub site: u16,
119:     pub captures: Vec<NamedValue>,
120:     pub logical: StageLogical,
121:     pub boundary: Snapshot,
122: }
123: #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
124: #[serde(deny_unknown_fields)]
```

## Exact production consumer excerpts: crates/mori-core/src/stage/decode.rs
```text
34: fn strict<T: DeserializeOwned>(
35:     text: &str,
36:     max: usize,
37:     too_large: &'static str,
38: ) -> Result<T, Error> {
39:     let value = crate::json::parse_limited(text, max, too_large)?;
40:     serde_json::from_value(value).map_err(|e| Error::new("json", e.to_string()))
41: }
42: 
43: /// Exactly one `Continuation`; unknown fields, a missing `cap` key and
44: /// non-canonical UInt strings are rejected by the type's derives.
45: pub fn continuation(text: &str) -> Result<Continuation, Error> {
46:     strict(text, MAX_CONTINUATION_BYTES, CONTINUATION_TOO_LARGE)
47: }
```

## Plan oracle provenance selected exact fields; all remaining fields physically available
```json
{"priorExposure":["Earlier Core1/Core2 public IR, parser/evaluator, financial reports and continuation implementation/source-review contexts","Author of source-exposed adopted ordinary report/M01 proposals, not their independent behavior proof","Earlier Source6 owner/public-consumer packets and separately frozen capped/max arithmetic oracles"],"caseBlindness":"Did not read this roundtrip case authored test/expected arrays, actual outputs, or implementation bodies. Read only the three permitted case inputs and public normative documentation. Startup AGENTS/product/status also read. No global blindness claim.","operations":"Ordinary file reads/copies and Python JSON/hash/arithmetic scaffolding only; no product imports/evaluation/compiler/runtime/provider/Git/build/test execution.","security":"No security/authentication/identity/cap-admission judgment."}
```

## Actual STDERR final5lines; complete original log physically supplied
```text
83:    Compiling mori-core v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/crates/mori-core)
84:     Finished `test` profile [unoptimized + debuginfo] target(s) in 26.76s
85:      Running tests/financial_program_wire_contract.rs (/tmp/doctorwatson-core-roundtrips-rtsd00nt/target/debug/deps/financial_program_wire_contract-6dbd15ed6676c385)
86:      Running tests/program_plan_roundtrip.rs (/tmp/doctorwatson-core-roundtrips-rtsd00nt/target/debug/deps/program_plan_roundtrip-1535ad40c4f75348)
87:      Running tests/utxo_pattern_exchange.rs (/tmp/doctorwatson-core-roundtrips-rtsd00nt/target/debug/deps/utxo_pattern_exchange-fa630d0a2d6d1d04)
```
