# Exact current source/result plus ONE unmerged finite composition shape

REPORT-DAG + roundtrip preserves all complete reports and8raw files. Full originals remain physical; no oracle/test changes. Prospective composition is NOT an observed merge.

===== BEGIN full-source/crates/mori/tests/core2_multi_condition_complete.rs MODE raw ALIASES [] =====
//! Complete local observations of one heterogeneous conjunctive await guard.
use mori_core::{
    Scenario,
    stage::{self, Continuation, PendingFrame, StageOutcome},
};
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/core2-multi-condition-complete")
}

fn inputs() -> PathBuf {
    fixtures().join("input-custody")
}

fn json_fixture(name: &str) -> Value {
    serde_json::from_slice(&fs::read(fixtures().join(name)).unwrap()).unwrap()
}

fn assert_report(actual: &Value, name: &str) {
    assert_eq!(
        actual,
        &json_fixture(&format!("{name}.report.expected.json")),
        "{name}: complete report"
    );
}

fn waiting(outcome: StageOutcome, name: &str) -> Continuation {
    assert_report(&stage::outcome_report(Ok(outcome.clone())), name);
    let StageOutcome::Waiting { continuation, .. } = outcome else {
        panic!("{name}: expected pending computation");
    };
    continuation
}

fn event(name: &str) -> stage::StageEvent {
    stage::decode::event(&fs::read_to_string(inputs().join(name)).unwrap()).unwrap()
}

#[test]
fn multi_condition_public_load_matches_complete_program_and_filled_plan() {
    let source = fs::read_to_string(inputs().join("multi-condition.mori")).unwrap();
    let program = stage::load(&source).unwrap();
    assert_eq!(
        serde_json::to_value(program.program()).unwrap(),
        json_fixture("multi-condition.program.expected.json")
    );
    assert_eq!(
        serde_json::to_value(program.plan()).unwrap(),
        json_fixture("multi-condition.plan.expected.json")
    );
}

#[test]
fn multi_condition_library_requires_both_observations_in_the_same_event() {
    let source = fs::read_to_string(inputs().join("multi-condition.mori")).unwrap();
    let program = stage::load(&source).unwrap();
    let scenario: Scenario =
        serde_json::from_slice(&fs::read(inputs().join("multi-condition.scenario.json")).unwrap())
            .unwrap();
    let mut pending = waiting(
        stage::start(&program, "deliver", scenario, None).unwrap(),
        "start",
    );
    for (name, file) in [
        ("document-only", "document-only.event.json"),
        ("amount-only", "amount-only.event.json"),
    ] {
        let before_observation = pending.clone();
        let observed =
            stage::resume(&program, pending, event(file), Some("deliver"), None).unwrap();
        pending = waiting(observed, name);
        assert_eq!(pending, before_observation, "{name}: actual continuation");
    }
    let pending_boundary = pending.boundary.clone();
    let pending_frame = PendingFrame {
        site: pending.site,
        captures: pending.captures.clone(),
    };
    let completed = stage::resume(
        &program,
        pending,
        event("both.event.json"),
        Some("deliver"),
        None,
    )
    .unwrap();
    let StageOutcome::Complete { segment, .. } = &completed else {
        panic!("both observations must complete the computation");
    };
    assert_eq!(segment.execution.pre, pending_boundary);
    assert_eq!(segment.frame_pre, pending_frame);
    assert_report(&stage::outcome_report(Ok(completed)), "both");
}

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Project {
    root: PathBuf,
}

impl Project {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-core2-multi-condition-complete-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let project = Self { root };
        for name in [
            "multi-condition.mori",
            "multi-condition.scenario.json",
            "document-only.event.json",
            "amount-only.event.json",
            "both.event.json",
        ] {
            fs::copy(inputs().join(name), project.root.join(name)).unwrap();
        }
        project
    }

    fn assert_case(&self, name: &str, args: &[&str]) -> Value {
        let output = Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.root)
            .args([
                "--color",
                "never",
                "simulate",
                "multi-condition.mori",
                "--action",
                "deliver",
            ])
            .args(args)
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "{name}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            output.stderr,
            fs::read(fixtures().join(format!("{name}.stderr.expected.txt"))).unwrap(),
            "{name}: exact empty stderr"
        );
        assert!(output.stdout.is_ascii(), "{name}: ASCII stdout");
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_report(&actual, name);
        assert_eq!(
            output.stdout,
            fs::read(fixtures().join(format!("{name}.stdout.expected.json"))).unwrap(),
            "{name}: exact public stdout bytes"
        );
        actual
    }

    fn resume(&self, previous: &Value, name: &str, event: &str) -> Value {
        fs::write(
            self.root.join("actual-continuation.json"),
            serde_json::to_vec(&previous["continuation"]).unwrap(),
        )
        .unwrap();
        self.assert_case(
            name,
            &["--resume", "actual-continuation.json", "--event", event],
        )
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn multi_condition_cli_requires_both_observations_and_matches_complete_public_bytes() {
    let project = Project::new();
    let mut previous =
        project.assert_case("start", &["--scenario", "multi-condition.scenario.json"]);
    for (name, file) in [
        ("document-only", "document-only.event.json"),
        ("amount-only", "amount-only.event.json"),
    ] {
        let observed = project.resume(&previous, name, file);
        assert_eq!(
            observed["continuation"], previous["continuation"],
            "{name}: actual continuation"
        );
        previous = observed;
    }
    let completed = project.resume(&previous, "both", "both.event.json");
    assert_eq!(
        completed["segment"]["execution"]["pre"],
        previous["continuation"]["boundary"]
    );
    assert_eq!(
        completed["segment"]["frame_pre"]["captures"],
        previous["continuation"]["captures"]
    );
    assert_eq!(
        completed["segment"]["frame_pre"]["site"],
        previous["continuation"]["site"]
    );
}
===== END full-source/crates/mori/tests/core2_multi_condition_complete.rs =====

===== BEGIN full-source/crates/fixtures/core2-multi-condition-complete/input-custody/multi-condition.mori MODE raw ALIASES [] =====
language "moriarty/2";
agreement MultiConditionDelivery {
  asset PAY = { network: "local", issuer: "multi-condition-input", reference: "PAY", scale: 0 };
  state marker: UInt = 77;
  action deliver(amount: UInt) {
    transfer(PAY, "alice", "escrow", amount);
    liability("delivery", PAY, "escrow", "bob", amount);
    await release(documentObserved: Bool, approvedAmount: UInt) when documentObserved and approvedAmount >= amount;
    transfer(PAY, "escrow", "bob", amount);
    discharge("delivery", amount);
  }
}
===== END full-source/crates/fixtures/core2-multi-condition-complete/input-custody/multi-condition.mori =====

===== BEGIN full-source/crates/fixtures/core2-multi-condition-complete/input-custody/multi-condition.scenario.json MODE raw ALIASES [] =====
{
  "state": {
    "marker": {
      "type": "UInt",
      "value": "77"
    }
  },
  "inputs": {
    "amount": {
      "type": "UInt",
      "value": "10"
    }
  },
  "balances": [
    {
      "account": "alice",
      "asset": {
        "network": "local",
        "issuer": "multi-condition-input",
        "reference": "PAY",
        "scale": 0
      },
      "amount": "20"
    },
    {
      "account": "bob",
      "asset": {
        "network": "local",
        "issuer": "multi-condition-input",
        "reference": "PAY",
        "scale": 0
      },
      "amount": "2"
    },
    {
      "account": "escrow",
      "asset": {
        "network": "local",
        "issuer": "multi-condition-input",
        "reference": "PAY",
        "scale": 0
      },
      "amount": "3"
    },
    {
      "account": "observer",
      "asset": {
        "network": "local",
        "issuer": "multi-condition-input",
        "reference": "PAY",
        "scale": 0
      },
      "amount": "9"
    }
  ],
  "liabilities": []
}
===== END full-source/crates/fixtures/core2-multi-condition-complete/input-custody/multi-condition.scenario.json =====

===== BEGIN full-source/crates/fixtures/core2-multi-condition-complete/input-custody/document-only.event.json MODE raw ALIASES [] =====
{
  "format": "moriarty-stage-event/1",
  "action": "deliver",
  "label": "release",
  "values": [
    {
      "name": "documentObserved",
      "value": {
        "type": "Bool",
        "value": true
      }
    },
    {
      "name": "approvedAmount",
      "value": {
        "type": "UInt",
        "value": "9"
      }
    }
  ]
}
===== END full-source/crates/fixtures/core2-multi-condition-complete/input-custody/document-only.event.json =====

===== BEGIN full-source/crates/fixtures/core2-multi-condition-complete/input-custody/amount-only.event.json MODE raw ALIASES [] =====
{
  "format": "moriarty-stage-event/1",
  "action": "deliver",
  "label": "release",
  "values": [
    {
      "name": "documentObserved",
      "value": {
        "type": "Bool",
        "value": false
      }
    },
    {
      "name": "approvedAmount",
      "value": {
        "type": "UInt",
        "value": "12"
      }
    }
  ]
}
===== END full-source/crates/fixtures/core2-multi-condition-complete/input-custody/amount-only.event.json =====

===== BEGIN full-source/crates/fixtures/core2-multi-condition-complete/input-custody/both.event.json MODE raw ALIASES [] =====
{
  "format": "moriarty-stage-event/1",
  "action": "deliver",
  "label": "release",
  "values": [
    {
      "name": "documentObserved",
      "value": {
        "type": "Bool",
        "value": true
      }
    },
    {
      "name": "approvedAmount",
      "value": {
        "type": "UInt",
        "value": "12"
      }
    }
  ]
}
===== END full-source/crates/fixtures/core2-multi-condition-complete/input-custody/both.event.json =====

===== BEGIN full-source/crates/fixtures/core2-multi-condition-complete/multi-condition.program.expected.json MODE raw ALIASES [] =====
{
  "actions": [
    {
      "inputs": [
        {
          "name": "amount",
          "ty": "UInt"
        }
      ],
      "name": "deliver",
      "statements": [
        {
          "amount": 0,
          "asset": "PAY",
          "from": "alice",
          "kind": "Transfer",
          "to": "escrow"
        },
        {
          "amount": 1,
          "asset": "PAY",
          "creditor": "bob",
          "debtor": "escrow",
          "kind": "Liability",
          "name": "delivery"
        },
        {
          "binders": [
            {
              "name": "documentObserved",
              "ty": "Bool"
            },
            {
              "name": "approvedAmount",
              "ty": "UInt"
            }
          ],
          "guard": 6,
          "kind": "Await",
          "label": "release"
        },
        {
          "amount": 7,
          "asset": "PAY",
          "from": "escrow",
          "kind": "Transfer",
          "to": "bob"
        },
        {
          "amount": 8,
          "kind": "Discharge",
          "name": "delivery"
        }
      ]
    }
  ],
  "assets": [
    {
      "identity": {
        "issuer": "multi-condition-input",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      },
      "name": "PAY"
    }
  ],
  "expressions": [
    {
      "kind": "Name",
      "name": "amount"
    },
    {
      "kind": "Name",
      "name": "amount"
    },
    {
      "kind": "Name",
      "name": "documentObserved"
    },
    {
      "kind": "Name",
      "name": "approvedAmount"
    },
    {
      "kind": "Name",
      "name": "amount"
    },
    {
      "kind": "Binary",
      "lhs": 3,
      "op": "Ge",
      "rhs": 4
    },
    {
      "kind": "Binary",
      "lhs": 2,
      "op": "And",
      "rhs": 5
    },
    {
      "kind": "Name",
      "name": "amount"
    },
    {
      "kind": "Name",
      "name": "amount"
    }
  ],
  "language": "moriarty/2",
  "name": "MultiConditionDelivery",
  "semantics": "moriarty-core/2",
  "states": [
    {
      "initial": {
        "type": "UInt",
        "value": "77"
      },
      "name": "marker",
      "ty": "UInt"
    }
  ]
}
===== END full-source/crates/fixtures/core2-multi-condition-complete/multi-condition.program.expected.json =====

===== BEGIN full-source/crates/fixtures/core2-multi-condition-complete/multi-condition.plan.expected.json MODE raw ALIASES [] =====
{
  "actions": [
    {
      "action": "deliver",
      "committed_before": [
        "0",
        "5"
      ],
      "plan_total": "15",
      "segment_cost": [
        "5",
        "10"
      ],
      "sites": [
        {
          "binders": [
            {
              "name": "documentObserved",
              "ty": "Bool"
            },
            {
              "name": "approvedAmount",
              "ty": "UInt"
            }
          ],
          "captures": [
            {
              "name": "amount",
              "ordinal": 0,
              "origin": "Input",
              "ty": "UInt"
            }
          ],
          "label": "release",
          "ordinal": 2,
          "site": 1
        }
      ],
      "statement_count": 5
    }
  ],
  "format": "moriarty-stage-plan/1",
  "program_hash": "41a9bcd572d3bbbccf5d797cc8d364a1a82a7637e8f0ebdf94b1e3871cbb66f3"
}
===== END full-source/crates/fixtures/core2-multi-condition-complete/multi-condition.plan.expected.json =====

===== BEGIN full-source/crates/fixtures/core2-multi-condition-complete/FINANCE-AND-WORK.json MODE json-value ALIASES [] =====
{"asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0},"balance_order":["alice","bob","escrow","observer"],"basis":"Exact source/scenario/event literals; adopted Core2 report/work/arena/identity clauses; explicit finite Transfer/Liability/Discharge arithmetic premise, not Core1 report-schema inheritance","complete_snapshots":{"after_delivery":{"balances":[{"account":"alice","amount":"10","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}},{"account":"bob","amount":"12","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}},{"account":"escrow","amount":"3","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}},{"account":"observer","amount":"9","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}}],"liabilities":[{"amount":"10","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0},"creditor":"bob","debtor":"escrow","name":"delivery"}],"state":{"marker":{"type":"UInt","value":"77"}}},"after_discharge":{"balances":[{"account":"alice","amount":"10","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}},{"account":"bob","amount":"12","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}},{"account":"escrow","amount":"3","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}},{"account":"observer","amount":"9","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}}],"liabilities":[{"amount":"0","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0},"creditor":"bob","debtor":"escrow","name":"delivery"}],"state":{"marker":{"type":"UInt","value":"77"}}},"after_funding":{"balances":[{"account":"alice","amount":"10","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}},{"account":"bob","amount":"2","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}},{"account":"escrow","amount":"13","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}},{"account":"observer","amount":"9","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}}],"liabilities":[],"state":{"marker":{"type":"UInt","value":"77"}}},"after_liability":{"balances":[{"account":"alice","amount":"10","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}},{"account":"bob","amount":"2","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}},{"account":"escrow","amount":"13","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}},{"account":"observer","amount":"9","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}}],"liabilities":[{"amount":"10","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0},"creditor":"bob","debtor":"escrow","name":"delivery"}],"state":{"marker":{"type":"UInt","value":"77"}}},"initial":{"balances":[{"account":"alice","amount":"20","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}},{"account":"bob","amount":"2","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}},{"account":"escrow","amount":"3","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}},{"account":"observer","amount":"9","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0}}],"liabilities":[],"state":{"marker":{"type":"UInt","value":"77"}}}},"cumulative_finance":{"bob_net_receipt":"10","closing_liability":"present-zero","created_liability":"10","discharged_liability":"10","explicit_fees":"0","gross_debits_by_account":{"alice":"10","escrow":"10"},"inapplicable":["mint","burn","dedicated interest/accrual","fee recipient","division/rounding/dust","refund/cancellation","external fee/ledger phase"],"internal_transfer_turnover":"20","net_balance_delta":{"alice":"-10","bob":"10","escrow":"0","observer":"0"},"opening_liability":"absent","preserved":"marker UInt77; observer9; escrow opening3 remains after onward delivery; all four balance rows and exact asset identity retained","warning":"Escrow opening3 is simply preserved pre-existing balance; no reserve floor/refund/dust policy is asserted. Liability creation/discharge are separate from token movement."},"logical":{"attempted_per_invocation":["5","6","6","10"],"cap":null,"committed_per_invocation":["5","5","5","15"],"cost_derivation":["start: (1+1)+(1+1)+await-entry1=5","each resume guard: dispatch1+distinct nodes2,3,4,5,6=6; both Boolean operands eagerly visited","true suffix: guard6+(1+1)+(1+1)=10"],"cumulative_attempted":["5","11","17","27"],"false_attempts_not_committed":"12","not_equated_to":["circuit cost","DUST","native fee","wall time","authenticated lifetime budget"],"per_statement_node_ids":{"0":[0],"1":[1],"2":[2,3,4,5,6],"3":[7],"4":[8]},"plan_total":"15","profile":"moriarty-reference-logical-cost/2","statement_dispatch":"1","static_committed_before":["0","5"],"static_segment_cost":["5","10"],"sum_attempted":"27"},"ordered_effects":[{"amount":"10","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0},"from":"alice","kind":"Transfer","to":"escrow"},{"after":"10","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0},"before":"0","creditor":"bob","debtor":"escrow","kind":"Liability","name":"delivery"},{"amount":"10","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0},"from":"escrow","kind":"Transfer","to":"bob"},{"after":"0","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0},"before":"10","creditor":"bob","debtor":"escrow","kind":"Liability","name":"delivery"}],"per_invocation":[{"attempted":"5","balance_delta":["-10","0","10","0"],"committed":"5","effects":[{"amount":"10","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0},"from":"alice","kind":"Transfer","to":"escrow"},{"after":"10","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0},"before":"0","creditor":"bob","debtor":"escrow","kind":"Liability","name":"delivery"}],"event":null,"explicit_fee":"0","guard_evaluated":false,"liability_closing":"10","liability_created":"10","liability_discharged":"0","liability_opening":"absent","name":"start","residual":"10","transfer_gross_debit":"10","transfer_recipient_credit":"10"},{"attempted":"6","balance_delta":["0","0","0","0"],"committed":"5","effects":[],"event":{"approvedAmount":"9","documentObserved":true},"explicit_fee":"0","guard":"true and (9 >= 10) = false","liability_closing":"10","liability_created":"0","liability_discharged":"0","liability_opening":"10","name":"document-only","residual":"10","transfer_gross_debit":"0","transfer_recipient_credit":"0"},{"attempted":"6","balance_delta":["0","0","0","0"],"committed":"5","effects":[],"event":{"approvedAmount":"12","documentObserved":false},"explicit_fee":"0","guard":"false and (12 >= 10) = false; eagerly evaluate both","liability_closing":"10","liability_created":"0","liability_discharged":"0","liability_opening":"10","name":"amount-only","residual":"10","transfer_gross_debit":"0","transfer_recipient_credit":"0"},{"attempted":"10","balance_delta":["0","10","-10","0"],"committed":"15","effects":[{"amount":"10","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0},"from":"escrow","kind":"Transfer","to":"bob"},{"after":"0","asset":{"issuer":"multi-condition-input","network":"local","reference":"PAY","scale":0},"before":"10","creditor":"bob","debtor":"escrow","kind":"Liability","name":"delivery"}],"event":{"approvedAmount":"12","documentObserved":true},"explicit_fee":"0","guard":"true and (12 >= 10) = true","liability_closing":"present-zero","liability_created":"0","liability_discharged":"10","liability_opening":"10","name":"both","residual":"0","transfer_gross_debit":"10","transfer_recipient_credit":"10"}],"total_asset_after":"34","total_asset_before":"34"}
===== END full-source/crates/fixtures/core2-multi-condition-complete/FINANCE-AND-WORK.json =====

===== BEGIN full-source/crates/fixtures/core2-multi-condition-complete/INDEPENDENT-FREEZE.json MODE json-value ALIASES [] =====
{"all14gaps_retained":true,"attempted":["5","6","6","10"],"basis":"post-adoption source/DTO/normative exposed; not blind or preimplementation; no current candidate implementation/test/actual output read","committed":["5","5","5","15"],"complete_objects":{"Program":1,"StagePlan":1,"canonical_preimages":2,"empty_stderr":4,"outcome_reports":4,"public_stdout":4},"input_freeze":{"path":"input-custody/FREEZE.sha256","sha256":"8cb2fc008d04891aeb24d8f99bd2eb3cf1926e02a4a725cefaf241138e0041d8"},"inventory":{"listed_files":46,"path":"PAYLOAD-INVENTORY.json","sha256":"50d4a23dd314026b5d7a855022c3d4882cdcdaaa64d323a0974ffa3daa8efd88"},"no_runtime_expected_input":"Future consumers must use only actual previous continuation, never this expected continuation","plan_hash":"e49b50878c7e89463420a00703ec29b697ca9729dc0ba48f428227f93e2031ba","program_hash":"41a9bcd572d3bbbccf5d797cc8d364a1a82a7637e8f0ebdf94b1e3871cbb66f3","static_verification":"pins, graph, integer equations, complete prefix/effect consistency, ASCII/singleLF; no Mori behavior observed","status":"FROZEN; independent expected data, product execution UNRUN","sum_attempted":"27","unchanged_previous_composition_freeze":"12135cbac3e899da1f063ba2f8e31a75f9ca1fa4d51d62fdb8dff635bfce3848"}
===== END full-source/crates/fixtures/core2-multi-condition-complete/INDEPENDENT-FREEZE.json =====

===== BEGIN full-source/crates/fixtures/core2-multi-condition-complete/PREPARATION.json MODE json-value ALIASES [] =====
{"actual_product_execution":"UNRUN","all14staged_gaps_and_allcomposition_modes_retained":true,"basis_pins":[{"bytes":14609,"copy":"basis/ORIGINAL-MPLR-REQUIREMENTS.md","kind":"existing public requirement/plan/research status only; not executable recovery semantics","sha256":"dc9423f021fefe67a131ea8ca5e6b6ccc764429729519a19a0f2c2d5c169c319","source":"/home/charl/Moriarty/.worktrees/utxo/openspec/changes/partial-and-conditional-transactions/specs/partial-conditional-transactions/spec.md"},{"bytes":3031,"copy":"basis/ORIGINAL-C0-C3-PLAN.md","kind":"existing public requirement/plan/research status only; not executable recovery semantics","sha256":"2e2bcb638e3a9ec482c616cf018a473cd1c85b93d77c4a69ecb42fe54b4e81dc","source":"/home/charl/Moriarty/.worktrees/utxo/openspec/changes/partial-and-conditional-transactions/implementation-plan.md"},{"bytes":3409,"copy":"basis/ORIGINAL-MPLR005-RESEARCH-STATUS.md","kind":"existing public requirement/plan/research status only; not executable recovery semantics","sha256":"ce2aa8bbc519e77d38d7f949b8918b12884f20c03c267b6d66a445dd662f20c6","source":"/home/charl/Moriarty/.worktrees/utxo/wiki/research/mplr/MPLR-005.md"}],"candidate_finite_missing_recovery_subjects":["controlled recoverable amount versus effects already committed or external","fees/refund/forward-compensation and liability opening/addition/discharge/closing equations","terminal cancellation/pending continuation relation and later ordinary resume meaning","observation/expiry/correlation premises required by chosen recovery branch","selected logical work/committed-prefix/status/field projection"],"coordinate":"publication_fixture_author preparing R/core2-next-stage-input-custody; current preferred cancellation/compensation under source diagnosis; no input packet read before frozen permission","independent_public_requirement_observation":"MPLR005/010/017 and C0/C3 plan require controlled refunds versus new compensation, actual history/residual duties and explicit staged accounting; wikiMPLR005 explicitly leaves concrete syntax/transition/proof/target choices open","not_claimed":"no actual API/runtime security judgment; no source function body or author expected output read; requirement absence here is not proof of unsupported current implementation","previous_composition_freeze_sha256":"12135cbac3e899da1f063ba2f8e31a75f9ca1fa4d51d62fdb8dff635bfce3848","prior_exposure":["earlier R3stagefixture/report custody potentially related original source examples","earlier independent Core1/Core2/Source6/composition oracle authoring; current composition74payloadfreeze immutable","earlier public DTO reads and incidental unrelated residual/InputDecl-conversion snippets, explicitly disclosed","current source-diagnosis summary from author only, no implementation body or output values"],"status":"PRE-INPUT checkpoint; author capability/input/normative freeze pending; no numerical/continuation output golden supplied","write_scope":"/home/charl/moriarty-c2-opus55-restored-20261017/core2-next-stage-independent-oracles"}
===== END full-source/crates/fixtures/core2-multi-condition-complete/PREPARATION.json =====

===== BEGIN full-source/crates/fixtures/core2-multi-condition-complete/GAPS-AND-LIMITS.json MODE json-value ALIASES [] =====
{"actual_execution":"UNRUN","library_pretty_bytes":"UNASSERTED","local_qualifications":{"authentication":"none","consumption":"untracked","qualification":"local-reference-stage"},"original_staged_gaps":[{"id":"GAP-01","status":"OPEN; no closure inferred from this new freeze","subject":"line numbers"},{"id":"GAP-02","status":"OPEN; no closure inferred from this new freeze","subject":"stage report internals"},{"id":"GAP-03","status":"OPEN; no closure inferred from this new freeze","subject":"/2 semantics string"},{"id":"GAP-04","status":"OPEN; no closure inferred from this new freeze","subject":"key order/format"},{"id":"GAP-05","status":"OPEN; no closure inferred from this new freeze","subject":"parser ExprId allocation"},{"id":"GAP-06","status":"OPEN; no closure inferred from this new freeze","subject":"stage admission premises"},{"id":"GAP-07","status":"OPEN; no closure inferred from this new freeze","subject":"stage-specific error codes"},{"id":"GAP-08","status":"OPEN; no closure inferred from this new freeze","subject":"/2 trace bound"},{"id":"GAP-09","status":"OPEN; no closure inferred from this new freeze","subject":"normative Step.value source"},{"id":"GAP-10","status":"OPEN; no closure inferred from this new freeze","subject":"normative Await presence"},{"id":"GAP-11","status":"OPEN; no closure inferred from this new freeze","subject":"/2 decode/replay"},{"id":"GAP-12","status":"OPEN; no closure inferred from this new freeze","subject":"MOR mapping"},{"id":"GAP-13","status":"OPEN; no closure inferred from this new freeze","subject":"boundary-order provenance/R7 contradiction"},{"id":"GAP-14","status":"OPEN; no closure inferred from this new freeze","subject":"false-resume cap refusal"}],"parent_exits":"All U0-U7, M1-M17, MC/SP/G, partial/conditional, full finance, ACTUS/DeFi and composition exits, counters/failures/original freezes unchanged","selected_normal_positive_fields":"Fully specified under adopted finite /2 premises; no remaining invented scalar, ID, frame or work field","separate_recovery_missing_premises":["adopted cancellation/compensation transition over pending frame and already committed effects","controlled refundable amount versus new forward compensation; fees and duty equations","terminal continuation/recovery/late-result relation","recovery-specific work and report projection"],"unassessed_not_asserted":["admission/order/cap-refusal policy","authentication/signature/document validation","decoder/replay/single-use","compiler soundness","proof/target/native/ledger acceptance","all general composition modes","complete full financial language including mint/burn/dedicated accrual"]}
===== END full-source/crates/fixtures/core2-multi-condition-complete/GAPS-AND-LIMITS.json =====

===== BEGIN full-source/crates/fixtures/core2-multi-condition-complete/PROCESS-EXPECTATIONS.json MODE json-value ALIASES [] =====
{"execution":"UNRUN; ordinary accepted positive invocation predictions only","false_tuple_policy":"Full independent event tuples; do not accumulate a previous true documentObserved value into another event.","invocations":[{"exit":0,"name":"start","stderr":"start.stderr.expected.txt","stdout":"start.stdout.expected.json","stdout_encoding":"ASCII, recursive lexical keys, LF, two-space indent, exactly one terminal LF"},{"exit":0,"name":"document-only","stderr":"document-only.stderr.expected.txt","stdout":"document-only.stdout.expected.json","stdout_encoding":"ASCII, recursive lexical keys, LF, two-space indent, exactly one terminal LF"},{"exit":0,"name":"amount-only","stderr":"amount-only.stderr.expected.txt","stdout":"amount-only.stdout.expected.json","stdout_encoding":"ASCII, recursive lexical keys, LF, two-space indent, exactly one terminal LF"},{"exit":0,"name":"both","stderr":"both.stderr.expected.txt","stdout":"both.stdout.expected.json","stdout_encoding":"ASCII, recursive lexical keys, LF, two-space indent, exactly one terminal LF"}],"library_pretty_bytes":"UNASSERTED: no transfer from public ASCII stdout contract","runtime_linkage":"Resume exclusively from actual immediately preceding Waiting.continuation. Never feed an expectation into runtime."}
===== END full-source/crates/fixtures/core2-multi-condition-complete/PROCESS-EXPECTATIONS.json =====

===== BEGIN full-source/crates/fixtures/core2-multi-condition-complete/FREEZE.sha256 MODE raw ALIASES [] =====
c8f2ede8014e29f705024a168e204df8bfc933ca9fb3a9fcbce7d661206e2225  BYTE-FACTS.json
7a091f72ce2002da7a6ed6561f003e4a31efbfba979aaf8ef2e2b0c8aaa80c94  DERIVATION.md
5675fa09c67c5d33bf959fba9ee3c9284dcb4ef00c4b6c2d46d58c18bfca9211  FINANCE-AND-WORK.json
3cec384445ad42b453ad45502970cd743a2b769e44c4f5d3bf632d2df8969e04  GAPS-AND-LIMITS.json
d4413a54fdbbb73138562228581073b296388da658529f85364d6981d39d3e6c  INDEPENDENT-FREEZE.json
1cb80dfa2d54d19dffefd830363cfddc98bf9c3807e536874b395a44fb2ca36a  LOCAL-IDENTITIES.json
50d4a23dd314026b5d7a855022c3d4882cdcdaaa64d323a0974ffa3daa8efd88  PAYLOAD-INVENTORY.json
f4885fb01cbaa0949bbac7d18f734402732fb2f3d11a1bc04b4de4edf4257b85  PREPARATION.json
160d8aa5103f5e975ef698318a618c059498343878d8861f014b6fb2519e4361  PROCESS-EXPECTATIONS.json
c0c07831b3e8bd5110a4cca8b602f531c5744c9979dc71a5cdc2aca43fce3a1a  README.md
2fb7dc16d0eb84486c87e8dae299fd9c0203f29450c5cdd0b4d7df3ae54c9dfd  amount-only.report.expected.json
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  amount-only.stderr.expected.txt
2fb7dc16d0eb84486c87e8dae299fd9c0203f29450c5cdd0b4d7df3ae54c9dfd  amount-only.stdout.expected.json
2e2bcb638e3a9ec482c616cf018a473cd1c85b93d77c4a69ecb42fe54b4e81dc  basis/ORIGINAL-C0-C3-PLAN.md
dc9423f021fefe67a131ea8ca5e6b6ccc764429729519a19a0f2c2d5c169c319  basis/ORIGINAL-MPLR-REQUIREMENTS.md
ce2aa8bbc519e77d38d7f949b8918b12884f20c03c267b6d66a445dd662f20c6  basis/ORIGINAL-MPLR005-RESEARCH-STATUS.md
b9b67ff9c5da6a35afd3af9c8a5fd03cc1341a5d1aca7d165a0dbcf3a182edae  both.report.expected.json
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  both.stderr.expected.txt
b9b67ff9c5da6a35afd3af9c8a5fd03cc1341a5d1aca7d165a0dbcf3a182edae  both.stdout.expected.json
bbdf140afdd6760fe310cd1ca7d960fa58632da809b4b98aa79ebc677cc07d17  derive_literal_oracles.py
2fb7dc16d0eb84486c87e8dae299fd9c0203f29450c5cdd0b4d7df3ae54c9dfd  document-only.report.expected.json
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  document-only.stderr.expected.txt
2fb7dc16d0eb84486c87e8dae299fd9c0203f29450c5cdd0b4d7df3ae54c9dfd  document-only.stdout.expected.json
8cb2fc008d04891aeb24d8f99bd2eb3cf1926e02a4a725cefaf241138e0041d8  input-custody/FREEZE.sha256
23383735a67be020df6167aa256ccb9866593fa22583d1e745d84215b7d99af4  input-custody/INPUT-MANIFEST.json
dedbef06fb279698495579ee48c53d60e17c9c31ca08a3a5a25267cabdbe33dd  input-custody/NORMATIVE-REFERENCES.json
840e4695e9a854ea702b9ff394b3277e45b5d1288e5154dc7549a0ee94e120af  input-custody/README.md
25685ebe095eb564b2c23580e2ec4b731e32660f4ec20c0f4b8caea4391733b4  input-custody/SCHEDULE.json
30eb6007979d1f0fd1876d5158f22f1561626875bf4280bcad451f054653bbe4  input-custody/amount-only.event.json
2c195283983fc5a944b34170a69b8f4c72c92a672a19708e0c884204558a6a8e  input-custody/both.event.json
cbb356a95f36b02bebf500fd9e23eedfd775905a9beaaaefda55a43827aeed00  input-custody/document-only.event.json
6f6a27f436066d4f84c28fcbfbe4d465c3f5358ad284e87350a2ee588dcf7549  input-custody/multi-condition.mori
38f72d8b63dc94e7268748a17b9eba8819ebd5e78ac6da901f17606b2139a0f2  input-custody/multi-condition.scenario.json
f0593c5e67916ac0b51420da29085feb27c79fddcf96048e9ccb8d806c8e0f4f  input-custody/normative-basis/docs/CORE-CONTINUATIONS.md
9d36a1ea979a52608a832d4b5429a6ca4e392091880326501071614808973b1e  input-custody/normative-basis/docs/CORE-EXECUTION-REPORTS.md
4ad9df1dc5111535800862cb6921751fb65d0db54c969a8a29bda9d7178315c3  input-custody/normative-basis/docs/CORE-STAGED-TESTS.md
70f7ccd294f9fb03496ee384f03b6eeaea21b45e61c3e3191131957ca20504b1  input-custody/normative-basis/openspec/changes/partial-and-conditional-transactions/design.md
2e2bcb638e3a9ec482c616cf018a473cd1c85b93d77c4a69ecb42fe54b4e81dc  input-custody/normative-basis/openspec/changes/partial-and-conditional-transactions/implementation-plan.md
dc9423f021fefe67a131ea8ca5e6b6ccc764429729519a19a0f2c2d5c169c319  input-custody/normative-basis/openspec/changes/partial-and-conditional-transactions/specs/partial-conditional-transactions/spec.md
764d0b430db401a6593d07a3bfb9703dfe889deb5d9865238ae662c9dd5f9959  input-custody/normative-basis/openspec/changes/partial-and-conditional-transactions/tasks.md
ce2aa8bbc519e77d38d7f949b8918b12884f20c03c267b6d66a445dd662f20c6  input-custody/normative-basis/wiki/research/mplr/MPLR-005.md
e49b50878c7e89463420a00703ec29b697ca9729dc0ba48f428227f93e2031ba  multi-condition.plan.canonical.bin
2ed590041553486147c3077247835ab87f8d3bb3c90e1227a16f256b87009a00  multi-condition.plan.expected.json
41a9bcd572d3bbbccf5d797cc8d364a1a82a7637e8f0ebdf94b1e3871cbb66f3  multi-condition.program.canonical.bin
76c008b05057b7bc8cc0bfe992143fa20205432ebe44d58554e438cac79cf195  multi-condition.program.expected.json
0f2c08a73786b8e2cc4fcd71a73f2ec3fc1830e3cedc9999095670ec804cc50e  start.report.expected.json
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  start.stderr.expected.txt
0f2c08a73786b8e2cc4fcd71a73f2ec3fc1830e3cedc9999095670ec804cc50e  start.stdout.expected.json
===== END full-source/crates/fixtures/core2-multi-condition-complete/FREEZE.sha256 =====

===== BEGIN full-source/crates/mori-core/src/stage/report.rs MODE raw ALIASES [] =====
//! Complete qualified JSON envelopes for the local Core/2 outcome API.
use super::{STAGE_LABELS, StageOutcome, StageRejection};
use serde_json::{Value, json};

/// Render an outcome or rejection without executing a program or performing I/O.
/// Every envelope retains the explicit local-stage qualifications.
pub fn outcome_report(result: Result<StageOutcome, StageRejection>) -> Value {
    let mut report = match result {
        Ok(StageOutcome::Waiting {
            advanced,
            segment,
            continuation,
            attempted,
            residual,
        }) => {
            let committed = continuation.logical.committed;
            json!({"status":"waiting","advanced":advanced,"segment":segment,
                "continuation":continuation,"attempted":attempted,
                "committed":committed,"residual":residual})
        }
        Ok(StageOutcome::Complete {
            segment,
            committed,
            attempted,
            residual,
        }) => json!({"status":"complete","segment":segment,"attempted":attempted,
                "committed":committed,"residual":residual}),
        Err(rejected) => json!({"status":"rejected","error":rejected.error,
            "attempted":rejected.attempted,"committed":rejected.committed}),
    };
    let object = report
        .as_object_mut()
        .expect("stage reports are JSON objects");
    for (name, value) in STAGE_LABELS {
        object.insert(name.into(), Value::String(value.into()));
    }
    report
}
===== END full-source/crates/mori-core/src/stage/report.rs =====

===== BEGIN actual-cycle/FORMATTING.diff MODE raw ALIASES [] =====
--- /home/charl/moriarty-c2-opus55-restored-20261017/core2-next-stage-source-proposal/after/crates/mori/tests/core2_multi_condition_complete.rs	2026-10-18 04:53:26.009999638 -0600
+++ /home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori/tests/core2_multi_condition_complete.rs	2026-10-18 04:56:39.303008575 -0600
@@ -61,10 +61,9 @@
 fn multi_condition_library_requires_both_observations_in_the_same_event() {
     let source = fs::read_to_string(inputs().join("multi-condition.mori")).unwrap();
     let program = stage::load(&source).unwrap();
-    let scenario: Scenario = serde_json::from_slice(
-        &fs::read(inputs().join("multi-condition.scenario.json")).unwrap(),
-    )
-    .unwrap();
+    let scenario: Scenario =
+        serde_json::from_slice(&fs::read(inputs().join("multi-condition.scenario.json")).unwrap())
+            .unwrap();
     let mut pending = waiting(
         stage::start(&program, "deliver", scenario, None).unwrap(),
         "start",
@@ -185,7 +184,8 @@
 #[test]
 fn multi_condition_cli_requires_both_observations_and_matches_complete_public_bytes() {
     let project = Project::new();
-    let mut previous = project.assert_case("start", &["--scenario", "multi-condition.scenario.json"]);
+    let mut previous =
+        project.assert_case("start", &["--scenario", "multi-condition.scenario.json"]);
     for (name, file) in [
         ("document-only", "document-only.event.json"),
         ("amount-only", "amount-only.event.json"),
===== END actual-cycle/FORMATTING.diff =====

===== BEGIN actual-cycle/WHITESPACE-CUSTODY.json MODE raw ALIASES [] =====
{
  "initialCheckExit": 2,
  "exactExceptions": [
    {
      "path": "crates/fixtures/core2-multi-condition-complete/basis/ORIGINAL-MPLR-REQUIREMENTS.md",
      "sha256": "dc9423f021fefe67a131ea8ca5e6b6ccc764429729519a19a0f2c2d5c169c319",
      "reason": "Exact immutable normative source blank EOF; preserve independently frozen bytes."
    },
    {
      "path": "crates/fixtures/core2-multi-condition-complete/input-custody/normative-basis/openspec/changes/partial-and-conditional-transactions/specs/partial-conditional-transactions/spec.md",
      "sha256": "dc9423f021fefe67a131ea8ca5e6b6ccc764429729519a19a0f2c2d5c169c319",
      "reason": "Exact immutable normative source blank EOF; preserve independently frozen bytes."
    }
  ],
  "allOtherPathsExit": 0
}
===== END actual-cycle/WHITESPACE-CUSTODY.json =====

===== BEGIN actual-cycle/WHITESPACE.log MODE raw ALIASES [] =====
crates/fixtures/core2-multi-condition-complete/basis/ORIGINAL-MPLR-REQUIREMENTS.md:200: new blank line at EOF.
crates/fixtures/core2-multi-condition-complete/input-custody/normative-basis/openspec/changes/partial-and-conditional-transactions/specs/partial-conditional-transactions/spec.md:200: new blank line at EOF.
===== END actual-cycle/WHITESPACE.log =====

===== BEGIN CURRENT-START-RESUME-EXCERPT.rs MODE raw ALIASES [] =====
/// Runs segment 0 of `action`. The scenario passes the existing
/// `scenario_snapshot`, then the plan-total cap preflight (C3, attempted 0).
/// The pre frame is locally assumed idle (P3); `cap` becomes the stored cap.
pub fn start(
    sp: &StagedProgram,
    action: &str,
    scenario: Scenario,
    cap: Option<UInt>,
) -> Result<StageOutcome, StageRejection> {
    let refuse = |error: Error| rejected(error, UInt(0), UInt(0));
    let index = sp
        .action_index(action)
        .ok_or_else(|| refuse(Error::new("action", format!("Unknown action {action:?}."))))?;
    let selected = &sp.program().actions[index];
    let plan = sp
        .plan()
        .actions
        .get(index)
        .filter(|plan| plan.action == action)
        .ok_or_else(|| {
            refuse(Error::new(
                "stage_site",
                "Action is absent from the stage plan.",
            ))
        })?;
    let pre = runtime::scenario_snapshot(sp.program(), selected, &scenario).map_err(refuse)?;
    let limit = cost::start_preflight(plan, cap).map_err(refuse)?;
    let mut inputs = Vec::with_capacity(selected.inputs.len());
    for input in &selected.inputs {
        let value = scenario.inputs.get(&input.name).cloned().ok_or_else(|| {
            refuse(Error::new(
                "input",
                format!("Missing or wrong type for input {:?}.", input.name),
            ))
        })?;
        inputs.push(NamedValue {
            name: input.name.clone(),
            value,
        });
    }
    // A boundary that R7 would refuse must not start an awaiting action:
    // its continuation could never be resumed (H25).
    if !plan.sites.is_empty() {
        decode::canonical_boundary(sp, index, &inputs, &pre).map_err(refuse)?;
    }
    let mut meter = cost::segment_meter(limit);
    let run = runtime::run_segment(
        sp,
        index,
        SegmentEntry::Start { inputs: &inputs },
        pre,
        &mut meter,
    );
    Admitted {
        sp,
        action: index,
        entry: 0,
        committed: UInt(0),
        stored_cap: cap,
        frame_pre: PendingFrame::idle(),
    }
    .finish(run, &meter, None)
}

/// Resumes `c` with event `e`, applying R0 then R1-R7 in order. The first
/// failure rejects with attempted 0 and the input's committed value. A false
/// guard returns the input continuation unchanged, with its stored cap; the
/// CLI cap only checks this invocation.
pub fn resume(
    sp: &StagedProgram,
    c: Continuation,
    e: StageEvent,
    cli_action: Option<&str>,
    cli_cap: Option<UInt>,
) -> Result<StageOutcome, StageRejection> {
    // Echoes the unchanged input; it is unverified when R1-R6 refuse.
    let kept = c.logical.committed;
    let refuse = |code: &str, message: &str| rejected(Error::new(code, message), UInt(0), kept);
    let refused = |error: Error| rejected(error, UInt(0), kept);
    // R0: an explicitly selected action must be the continuation's action.
    if cli_action.is_some_and(|selected| selected != c.action) {
        return Err(refuse(
            "stage_site",
            "Selected action differs from the continuation action.",
        ));
    }
    // R1
    if c.format != CONTINUATION_FORMAT || c.logical.profile != LOGICAL_COST_PROFILE_2 {
        return Err(refuse(
            "stage_format",
            "Unsupported continuation format or logical profile.",
        ));
    }
    // R2
    profile::require_core2(sp.program()).map_err(refused)?;
    if !digest::matches(&c.program_hash, &sp.program_hash())
        || !digest::matches(&c.plan_hash, &sp.plan_hash())
    {
        return Err(refuse(
            "stage_program",
            "Continuation does not belong to this program and stage plan.",
        ));
    }
    // R3
    let Some(index) = sp.action_index(&c.action) else {
        return Err(refuse("stage_site", "Continuation action does not exist."));
    };
    let Some(plan) = sp
        .plan()
        .actions
        .get(index)
        .filter(|plan| plan.action == c.action)
    else {
        return Err(refuse(
            "stage_site",
            "Action is absent from the stage plan.",
        ));
    };
    if c.site == 0 || usize::from(c.site) > plan.sites.len() {
        return Err(refuse(
            "stage_site",
            "Continuation site is not a pending await of this action.",
        ));
    }
    let site = &plan.sites[usize::from(c.site) - 1];
    // R4
    if !layout_matches(&c.captures, site.captures.iter(), |s| {
        (s.name.as_str(), s.ty)
    }) {
        return Err(refuse(
            "stage_capture",
            "Captures differ from the pending site layout.",
        ));
    }
    // R5
    if e.format != STAGE_EVENT_FORMAT
        || e.action != c.action
        || e.label != site.label
        || !layout_matches(&e.values, site.binders.iter(), |b| (b.name.as_str(), b.ty))
    {
        return Err(refuse(
            "stage_event",
            "Event does not match the pending await.",
        ));
    }
    // R6
    let limit = cost::resume_preflight(plan, c.site, &c.logical, cli_cap).map_err(refused)?;
    // R7
    decode::canonical_boundary(sp, index, &c.captures, &c.boundary).map_err(refused)?;
    let mut meter = cost::segment_meter(limit);
    let run = runtime::run_segment(
        sp,
        index,
        SegmentEntry::Resume {
            site: c.site,
            captures: &c.captures,
            event: &e.values,
        },
        c.boundary.clone(),
        &mut meter,
    );
    Admitted {
        sp,
        action: index,
        entry: c.site,
        committed: kept,
        stored_cap: c.logical.cap,
        frame_pre: PendingFrame {
            site: c.site,
            captures: c.captures.clone(),
        },
    }
    .finish(run, &meter, Some(c))
}
===== END CURRENT-START-RESUME-EXCERPT.rs =====

===== BEGIN actual-cycle/multi-condition-complete/START.json MODE raw ALIASES [] =====
{
  "startedAt": "2026-10-18T10:57:14.659619+00:00",
  "sourceCommit": "e0d0b4c7e80f53ead164358a68d804dc18c68d89",
  "sourceTree": "c1701271cd6afc56092fab416137505aa68d31a8",
  "sourceStatus": "",
  "argv": [
    "/usr/bin/timeout",
    "--signal=TERM",
    "--kill-after=5s",
    "90s",
    "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
    "test",
    "--manifest-path",
    "/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/Cargo.toml",
    "--frozen",
    "-j1",
    "-p",
    "mori",
    "--no-default-features",
    "--test",
    "core2_multi_condition_complete",
    "--",
    "--test-threads=1"
  ],
  "cwd": "/tmp/doctorwatson-multi-condition-pknhuxef/cwd",
  "environment": {
    "PATH": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin",
    "RUSTC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc",
    "RUSTDOC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc",
    "CARGO_HOME": "/tmp/doctorwatson-multi-condition-pknhuxef/cargo-home",
    "CARGO_TARGET_DIR": "/tmp/doctorwatson-multi-condition-pknhuxef/target",
    "CARGO_NET_OFFLINE": "true",
    "CARGO_INCREMENTAL": "0",
    "CARGO_CACHE_AUTO_CLEAN_FREQUENCY": "never",
    "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_CONFIG_GLOBAL": "/dev/null",
    "GIT_CONFIG_SYSTEM": "/dev/null",
    "LANG": "C.UTF-8",
    "TMPDIR": "/tmp/doctorwatson-multi-condition-pknhuxef/tmp"
  },
  "sourcePinsVerified": 557,
  "attempt": 1,
  "attemptLimit": 1
}
===== END actual-cycle/multi-condition-complete/START.json =====

===== BEGIN actual-cycle/multi-condition-complete/RESULT.json MODE raw ALIASES [] =====
{
  "finishedAt": "2026-10-18T10:58:08.362014+00:00",
  "exitCode": 0,
  "wallSeconds": 53.648904955000035,
  "sourceCommitAfter": "e0d0b4c7e80f53ead164358a68d804dc18c68d89",
  "sourceTreeAfter": "c1701271cd6afc56092fab416137505aa68d31a8",
  "sourceStatusAfter": "",
  "stdoutSha256": "c47854f6198309a63d2343f201eeb587ee4c683fd6c6a2538eea697246ded36a",
  "stderrSha256": "6b976c2cc28181fa30d43aa5a2c05240df9e50fd3894e3acafd35a972fc12cfb",
  "attemptConsumed": true
}
===== END actual-cycle/multi-condition-complete/RESULT.json =====

===== BEGIN actual-cycle/multi-condition-complete/CHILD.json MODE raw ALIASES [] =====
{
  "pid": 613484,
  "session": 613484
}
===== END actual-cycle/multi-condition-complete/CHILD.json =====

===== BEGIN actual-cycle/multi-condition-complete/PROCESS-CLEANUP.json MODE raw ALIASES [] =====
{
  "ownedSession": 613484,
  "remainingSessionMembers": [],
  "scope": "Owned SID census only; no syscall/detached-process exclusion."
}
===== END actual-cycle/multi-condition-complete/PROCESS-CLEANUP.json =====

===== BEGIN actual-cycle/multi-condition-complete/STDOUT.log MODE raw ALIASES [] =====

running 3 tests
test multi_condition_cli_requires_both_observations_and_matches_complete_public_bytes ... ok
test multi_condition_library_requires_both_observations_in_the_same_event ... ok
test multi_condition_public_load_matches_complete_program_and_filled_plan ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

===== END actual-cycle/multi-condition-complete/STDOUT.log =====

===== BEGIN actual-cycle/multi-condition-complete/STDERR.log MODE raw ALIASES [] =====
   Compiling proc-macro2 v1.0.107
   Compiling unicode-ident v1.0.26
   Compiling quote v1.0.47
   Compiling syn v3.0.6
   Compiling cfg-if v1.0.5
   Compiling synstructure v0.14.0
   Compiling zerofrom-derive v0.1.8
   Compiling zerofrom v0.1.8
   Compiling yoke-derive v0.8.4
   Compiling typenum v1.20.1
   Compiling stable_deref_trait v1.2.1
   Compiling yoke v0.8.3
   Compiling zerovec-derive v0.11.6
   Compiling zerovec v0.11.8
   Compiling displaydoc v0.2.7
   Compiling zeroize v1.9.1
   Compiling writeable v0.6.4
   Compiling version_check v0.9.5
   Compiling libc v0.2.190
   Compiling generic-array v0.14.9
   Compiling tinystr v0.8.4
   Compiling litemap v0.8.3
   Compiling icu_locale_fallback_data v2.3.0
   Compiling rustix v1.1.5
   Compiling icu_locale_core v2.3.0
   Compiling potential_utf v0.1.6
   Compiling zerotrie v0.2.5
   Compiling subtle v2.6.1
   Compiling const-oid v0.9.6
   Compiling linux-raw-sys v0.12.1
   Compiling icu_segmenter_data v2.3.0
   Compiling bitflags v2.13.2
   Compiling icu_provider v2.3.1
   Compiling utf8_iter v1.0.4
   Compiling icu_collections v2.3.0
   Compiling icu_locale_fallback v2.3.0
   Compiling smallvec v1.16.2
   Compiling icu_segmenter v2.3.0
   Compiling unicode-width v0.2.2
   Compiling is_ci v1.2.0
   Compiling supports-color v3.0.2
   Compiling textwrap v0.16.4
   Compiling terminal_size v0.4.4
   Compiling block-buffer v0.10.4
   Compiling crypto-common v0.1.6
   Compiling getrandom v0.2.17
   Compiling unicode-width v0.1.14
   Compiling supports-hyperlinks v3.2.0
   Compiling owo-colors v4.4.0
   Compiling supports-unicode v3.0.0
   Compiling miette v7.6.0
   Compiling rand_core v0.6.4
   Compiling digest v0.10.7
   Compiling syn v2.0.119
   Compiling soa-rs-derive v1.0.1
   Compiling mori-span v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-span)
   Compiling der v0.7.10
   Compiling memchr v2.8.3
   Compiling crossbeam-utils v0.8.23
   Compiling mori-diagnostics v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-diagnostics)
   Compiling soa-rs v1.0.1
   Compiling crossbeam-epoch v0.9.21
   Compiling mori-lexer v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-lexer)
   Compiling spki v0.7.3
   Compiling ff v0.13.1
   Compiling hybrid-array v0.4.15
   Compiling serde_core v1.0.229
   Compiling base16ct v0.2.0
   Compiling crossbeam-deque v0.8.8
   Compiling sec1 v0.7.3
   Compiling group v0.13.0
   Compiling pkcs8 v0.10.2
   Compiling hmac v0.12.1
   Compiling crypto-bigint v0.5.5
   Compiling once_cell v1.21.4
   Compiling utf8parse v0.2.2
   Compiling zmij v1.0.23
   Compiling anstyle-parse v1.0.0
   Compiling elliptic-curve v0.13.8
   Compiling rfc6979 v0.4.0
   Compiling crypto-common v0.2.2
   Compiling block-buffer v0.12.1
   Compiling mori-ast v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-ast)
   Compiling aho-corasick v1.1.5
   Compiling signature v2.2.0
   Compiling cpufeatures v0.2.17
   Compiling rayon-core v1.13.0
   Compiling serde_json v1.0.151
   Compiling getrandom v0.4.3
   Compiling colorchoice v1.0.5
   Compiling serde v1.0.229
   Compiling anstyle-query v1.1.5
   Compiling regex-syntax v0.8.11
   Compiling const-oid v0.10.2
   Compiling thiserror v2.0.21
   Compiling is_terminal_polyfill v1.70.2
   Compiling anstyle v1.0.14
   Compiling anstream v1.0.0
   Compiling digest v0.11.3
   Compiling regex-automata v0.4.18
   Compiling sha2 v0.10.9
   Compiling ecdsa v0.16.9
   Compiling bstr v1.13.1
   Compiling serde_derive v1.0.229
   Compiling thiserror-impl v2.0.21
   Compiling same-file v1.0.6
   Compiling heck v0.5.0
   Compiling clap_lex v1.1.1
   Compiling itoa v1.0.18
   Compiling log v0.4.34
   Compiling similar v2.7.0
   Compiling strsim v0.11.1
   Compiling cpufeatures v0.3.1
   Compiling sha2 v0.11.0
   Compiling clap_builder v4.6.7
   Compiling globset v0.4.20
   Compiling clap_derive v4.6.7
   Compiling walkdir v2.5.0
   Compiling k256 v0.13.4
   Compiling mori-parser v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-parser)
   Compiling either v1.19.0
   Compiling fastrand v2.5.0
   Compiling tempfile v3.27.0
   Compiling rayon v1.12.0
   Compiling mori-fmt v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-fmt)
   Compiling mori-core v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-core)
   Compiling ignore v0.4.33
   Compiling clap v4.6.7
   Compiling mori-checker v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-checker)
   Compiling console v0.16.6
   Compiling insta v1.49.0
   Compiling indoc v2.0.7
   Compiling mori v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 53.60s
     Running tests/core2_multi_condition_complete.rs (/tmp/doctorwatson-multi-condition-pknhuxef/target/debug/deps/core2_multi_condition_complete-bab38b7da15ee630)
===== END actual-cycle/multi-condition-complete/STDERR.log =====

===== BEGIN actual-cycle/RESULT.json MODE raw ALIASES [] =====
{
  "recordedAt": "2026-10-18T10:58:08.464505+00:00",
  "sourceCommit": "e0d0b4c7e80f53ead164358a68d804dc18c68d89",
  "sourceTree": "c1701271cd6afc56092fab416137505aa68d31a8",
  "steps": [
    {
      "step": "multi-condition-complete",
      "finishedAt": "2026-10-18T10:58:08.362014+00:00",
      "exitCode": 0,
      "wallSeconds": 53.648904955000035,
      "sourceCommitAfter": "e0d0b4c7e80f53ead164358a68d804dc18c68d89",
      "sourceTreeAfter": "c1701271cd6afc56092fab416137505aa68d31a8",
      "sourceStatusAfter": "",
      "stdoutSha256": "c47854f6198309a63d2343f201eeb587ee4c683fd6c6a2538eea697246ded36a",
      "stderrSha256": "6b976c2cc28181fa30d43aa5a2c05240df9e50fd3894e3acafd35a972fc12cfb",
      "attemptConsumed": true
    }
  ],
  "temporaryRootExistsAfter": false,
  "allStepsCompleted": true,
  "allExitZero": true,
  "scope": "Three complete multi-condition public load/library/CLI checks, four ordinary outcomes each and actual continuation links; all14 gaps/recovery/authentication/native/proof/ledger remain unobserved."
}
===== END actual-cycle/RESULT.json =====

===== BEGIN actual-cycle/CLEANUP.json MODE raw ALIASES [] =====
{
  "recordedAt": "2026-10-18T10:58:08.464297+00:00",
  "ownedTemporaryRoot": "/tmp/doctorwatson-multi-condition-pknhuxef",
  "existsAfter": false,
  "scope": "Only owned temporary root removed, public cache symlinks not traversed."
}
===== END actual-cycle/CLEANUP.json =====

===== BEGIN GUARDED-INTEGRATION.json MODE json-value ALIASES [] =====
{"assessed_main":"7396d7d316ab72957bca9709483f4638daa0d5cb","assessed_main_tree":"7e7a6fc1980b67c06ba9ceda78a6f11eaf682227","tested_source":"e0d0b4c7e80f53ead164358a68d804dc18c68d89","tested_tree":"c1701271cd6afc56092fab416137505aa68d31a8","tested_base_main":"7396d7d316ab72957bca9709483f4638daa0d5cb","base_to_assessed_main_name_status":[],"pinned_source_files":557,"candidate_new_paths_absent_main":50,"existing_pinned_files_different":[],"all_current_I_source_pins_still_match":true,"integration_guard":"Root rechecks current main and all selected runtime/dependency/fixture/test bytes before merge. Relevant later changes require exact reassessment; no automatic rerun solely for unrelated doc/unselected-test changes. Source6 changes already belong to this tested7396base; no old588d source compatibility inferred.","status":"Read-only snapshot before review; not merge observation"}
===== END GUARDED-INTEGRATION.json =====

===== BEGIN full-source/crates/fixtures/core2-multi-condition-complete/start.stderr.expected.txt MODE raw ALIASES ["full-source/crates/fixtures/core2-multi-condition-complete/document-only.stderr.expected.txt", "full-source/crates/fixtures/core2-multi-condition-complete/amount-only.stderr.expected.txt", "full-source/crates/fixtures/core2-multi-condition-complete/both.stderr.expected.txt"] =====

===== END full-source/crates/fixtures/core2-multi-condition-complete/start.stderr.expected.txt =====

===== BEGIN REPORT-DAG.json MODE json-value ALIASES [] =====
{"format":"one-off-review-json-dag/1","scope":"Review-only complete report encoding; not product fixture format, evaluator, serializer contract or test dependency","encoding":"nodes[tag,payload]: v=exact scalar, a=ordered child IDs, o=ordered [member key,child ID] pairs. IDs are topological. roots select complete JSON objects. Preserve object/array order; do not insert/delete fields.","render":"Exact finite reconstruction for these reports only: json.dumps(decoded,ensure_ascii=True,indent=2) plus one LF. Full original files remain physical.","nodes":[["v",true],["v","5"],["v","none"],["v","untracked"],["v","deliver"],["v","alice"],["v","10"],["v","multi-condition-input"],["v","local"],["v","PAY"],["v",0],["o",[["issuer",7],["network",8],["reference",9],["scale",10]]],["o",[["account",5],["amount",6],["asset",11]]],["v","bob"],["v","2"],["o",[["account",13],["amount",14],["asset",11]]],["v","escrow"],["v","13"],["o",[["account",16],["amount",17],["asset",11]]],["v","observer"],["v","9"],["o",[["account",19],["amount",20],["asset",11]]],["a",[12,15,18,21]],["v","delivery"],["o",[["amount",6],["asset",11],["creditor",13],["debtor",16],["name",23]]],["a",[24]],["v","UInt"],["v","77"],["o",[["type",26],["value",27]]],["o",[["marker",28]]],["o",[["balances",22],["liabilities",25],["state",29]]],["v","amount"],["o",[["type",26],["value",6]]],["o",[["name",31],["value",32]]],["a",[33]],["v","moriarty-continuation/1"],["v",null],["v","moriarty-reference-logical-cost/2"],["o",[["cap",36],["committed",1],["profile",37]]],["v","e49b50878c7e89463420a00703ec29b697ca9729dc0ba48f428227f93e2031ba"],["v","41a9bcd572d3bbbccf5d797cc8d364a1a82a7637e8f0ebdf94b1e3871cbb66f3"],["v",1],["o",[["action",4],["boundary",30],["captures",34],["format",35],["logical",38],["plan_hash",39],["program_hash",40],["site",41]]],["v","local-reference-stage"],["v","Transfer"],["o",[["amount",6],["asset",11],["from",5],["kind",44],["to",16]]],["v","0"],["v","Liability"],["o",[["after",6],["asset",11],["before",46],["creditor",13],["debtor",16],["kind",47],["name",23]]],["a",[45,48]],["v","20"],["o",[["account",5],["amount",50],["asset",11]]],["v","3"],["o",[["account",16],["amount",52],["asset",11]]],["a",[51,15,53,21]],["a",[]],["o",[["balances",54],["liabilities",55],["state",29]]],["v","moriarty-core/2"],["a",[45]],["o",[["balances",22],["liabilities",55],["state",29]]],["o",[["amount",10],["asset",9],["from",5],["kind",44],["to",16]]],["o",[["effects",58],["index",10],["snapshot",59],["statement",60],["value",32]]],["a",[48]],["o",[["amount",41],["asset",9],["creditor",13],["debtor",16],["kind",47],["name",23]]],["o",[["effects",62],["index",41],["snapshot",30],["statement",63],["value",32]]],["a",[61,64]],["o",[["action",4],["effects",49],["post",30],["pre",56],["semantics",57],["steps",65]]],["o",[["captures",34],["site",41]]],["o",[["captures",55],["site",10]]],["o",[["action",4],["execution",66],["frame_post",67],["frame_pre",68]]],["v","waiting"],["o",[["advanced",0],["attempted",1],["authentication",2],["committed",1],["consumption",3],["continuation",42],["qualification",43],["residual",25],["segment",69],["status",70]]],["v",false],["v","6"],["o",[["advanced",72],["attempted",73],["authentication",2],["committed",1],["consumption",3],["continuation",42],["qualification",43],["residual",25],["segment",36],["status",70]]],["v","15"],["o",[["amount",6],["asset",11],["from",16],["kind",44],["to",13]]],["o",[["after",46],["asset",11],["before",6],["creditor",13],["debtor",16],["kind",47],["name",23]]],["a",[76,77]],["v","12"],["o",[["account",13],["amount",79],["asset",11]]],["a",[12,80,53,21]],["o",[["amount",46],["asset",11],["creditor",13],["debtor",16],["name",23]]],["a",[82]],["o",[["balances",81],["liabilities",83],["state",29]]],["v",2],["v","documentObserved"],["v","Bool"],["o",[["name",86],["ty",87]]],["v","approvedAmount"],["o",[["name",89],["ty",26]]],["a",[88,90]],["v",6],["v","Await"],["v","release"],["o",[["binders",91],["guard",92],["kind",93],["label",94]]],["o",[["type",87],["value",0]]],["o",[["effects",55],["index",85],["snapshot",30],["statement",95],["value",96]]],["a",[76]],["v",3],["o",[["balances",81],["liabilities",25],["state",29]]],["v",7],["o",[["amount",101],["asset",9],["from",16],["kind",44],["to",13]]],["o",[["effects",98],["index",99],["snapshot",100],["statement",102],["value",32]]],["a",[77]],["v",4],["v",8],["v","Discharge"],["o",[["amount",106],["kind",107],["name",23]]],["o",[["effects",104],["index",105],["snapshot",84],["statement",108],["value",32]]],["a",[97,103,109]],["o",[["action",4],["effects",78],["post",84],["pre",30],["semantics",57],["steps",110]]],["o",[["action",4],["execution",111],["frame_post",68],["frame_pre",67]]],["v","complete"],["o",[["attempted",6],["authentication",2],["committed",75],["consumption",3],["qualification",43],["residual",55],["segment",112],["status",113]]]],"roots":{"start":71,"document-only":74,"amount-only":74,"both":114}}
===== END REPORT-DAG.json =====

===== BEGIN REPORT-DAG-ROUNDTRIP.json MODE json-value ALIASES [] =====
{"dag_sha256":"621f671cb3019033fec18defcbe2efde955365e83c8401dd71fe624456e3c313","nodes":115,"four_roots":{"start":71,"document-only":74,"amount-only":74,"both":114},"all8_raw_files_roundtrip":[{"path":"full-source/crates/fixtures/core2-multi-condition-complete/start.report.expected.json","root":71,"raw_bytes":11237,"sha256":"0f2c08a73786b8e2cc4fcd71a73f2ec3fc1830e3cedc9999095670ec804cc50e","full_json_value_equal":true,"exact_raw_bytes_equal":true},{"path":"full-source/crates/fixtures/core2-multi-condition-complete/start.stdout.expected.json","root":71,"raw_bytes":11237,"sha256":"0f2c08a73786b8e2cc4fcd71a73f2ec3fc1830e3cedc9999095670ec804cc50e","full_json_value_equal":true,"exact_raw_bytes_equal":true},{"path":"full-source/crates/fixtures/core2-multi-condition-complete/document-only.report.expected.json","root":74,"raw_bytes":2495,"sha256":"2fb7dc16d0eb84486c87e8dae299fd9c0203f29450c5cdd0b4d7df3ae54c9dfd","full_json_value_equal":true,"exact_raw_bytes_equal":true},{"path":"full-source/crates/fixtures/core2-multi-condition-complete/document-only.stdout.expected.json","root":74,"raw_bytes":2495,"sha256":"2fb7dc16d0eb84486c87e8dae299fd9c0203f29450c5cdd0b4d7df3ae54c9dfd","full_json_value_equal":true,"exact_raw_bytes_equal":true},{"path":"full-source/crates/fixtures/core2-multi-condition-complete/amount-only.report.expected.json","root":74,"raw_bytes":2495,"sha256":"2fb7dc16d0eb84486c87e8dae299fd9c0203f29450c5cdd0b4d7df3ae54c9dfd","full_json_value_equal":true,"exact_raw_bytes_equal":true},{"path":"full-source/crates/fixtures/core2-multi-condition-complete/amount-only.stdout.expected.json","root":74,"raw_bytes":2495,"sha256":"2fb7dc16d0eb84486c87e8dae299fd9c0203f29450c5cdd0b4d7df3ae54c9dfd","full_json_value_equal":true,"exact_raw_bytes_equal":true},{"path":"full-source/crates/fixtures/core2-multi-condition-complete/both.report.expected.json","root":114,"raw_bytes":11953,"sha256":"b9b67ff9c5da6a35afd3af9c8a5fd03cc1341a5d1aca7d165a0dbcf3a182edae","full_json_value_equal":true,"exact_raw_bytes_equal":true},{"path":"full-source/crates/fixtures/core2-multi-condition-complete/both.stdout.expected.json","root":114,"raw_bytes":11953,"sha256":"b9b67ff9c5da6a35afd3af9c8a5fd03cc1341a5d1aca7d165a0dbcf3a182edae","full_json_value_equal":true,"exact_raw_bytes_equal":true}],"oracle_files_changed":false,"runtime_or_producer_used":false}
===== END REPORT-DAG-ROUNDTRIP.json =====

===== BEGIN finite-composition/composition-after/crates/mori-core/Cargo.toml MODE raw ALIASES [] =====
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
===== END finite-composition/composition-after/crates/mori-core/Cargo.toml =====

===== BEGIN finite-composition/composition-after/crates/mori/Cargo.toml MODE raw ALIASES [] =====
[package]
name = "mori"
version = "0.1.0"
description = "Command-line interface for the Moriarty toolchain"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true

[features]
core2-disjoint-draft = ["mori-core/core2-disjoint-draft"]
default = ["native"]
source6-expansion = ["dep:mori-source6", "mori-source6/source6-transfer-slice"]
native = [
    "dep:mori-backend",
    "dep:mori-ledger",
    "dep:futures",
    "dep:rand",
    "dep:midnight-ledger",
    "dep:midnight-base-crypto",
    "dep:midnight-coin-structure",
    "dep:midnight-onchain-state",
    "dep:midnight-onchain-runtime",
    "dep:midnight-storage",
    "dep:midnight-transient-crypto",
    "dep:midnight-serialize",
]

[dependencies]
anstream.workspace = true
anstyle.workspace = true
clap.workspace = true
ignore.workspace = true
mori-ast.workspace = true
mori-checker.workspace = true
mori-core.workspace = true
mori-backend = { workspace = true, optional = true }
mori-ledger = { workspace = true, optional = true }
mori-diagnostics = { workspace = true, features = ["fancy"] }
mori-fmt.workspace = true
mori-lexer.workspace = true
mori-parser.workspace = true
mori-source6 = { workspace = true, optional = true }
mori-span.workspace = true
rayon.workspace = true
futures = { workspace = true, optional = true }
rand = { workspace = true, optional = true }
serde.workspace = true
serde_json.workspace = true
sha2.workspace = true
terminal_size.workspace = true
thiserror.workspace = true

midnight-ledger = { workspace = true, optional = true }
midnight-base-crypto = { workspace = true, optional = true }
midnight-coin-structure = { workspace = true, optional = true }
midnight-onchain-state = { workspace = true, optional = true }
midnight-onchain-runtime = { workspace = true, optional = true }
midnight-storage = { workspace = true, optional = true }
midnight-transient-crypto = { workspace = true, optional = true }
midnight-serialize = { workspace = true, optional = true }

[dev-dependencies]
indoc.workspace = true
insta.workspace = true

[[test]]
name = "check_compile_io"
required-features = ["native"]

[[test]]
name = "check_static_compile_io"
required-features = ["native"]

[[test]]
name = "prepare_call"
required-features = ["native"]

[[test]]
name = "installed_call"
required-features = ["native"]

[[test]]
name = "signed_installed_call"
required-features = ["native"]

[[test]]
name = "native_proofs"
required-features = ["native"]

[[test]]
name = "signed_intents"
required-features = ["native"]

[[test]]
name = "replay"
required-features = ["native"]

[[test]]
name = "replay_challenge"
required-features = ["native"]

[[test]]
name = "enforcement_report"
required-features = ["native"]

[[test]]
name = "mapped_role_static_io"
required-features = ["native"]

[[test]]
name = "source6_expand"
required-features = ["source6-expansion"]
===== END finite-composition/composition-after/crates/mori/Cargo.toml =====

===== BEGIN finite-composition/composition-after/crates/mori-core/src/stage/mod.rs MODE raw ALIASES [] =====
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

pub use crate::LOGICAL_COST_PROFILE_2;
pub use admit::{resume, start};
pub use plan::{ActionPlan, AwaitSite, CaptureSlot, Origin, StageBinder, StagePlan};
pub use profile::{require_core1, require_core2};
pub use report::outcome_report;

use crate::{Error, Execution, Liability, Program, Snapshot, UInt, Value};
use serde::{Deserialize, Serialize};

pub const LANGUAGE_2: &str = "moriarty/2";
pub const SEMANTICS_2: &str = "moriarty-core/2";
pub const STAGE_PLAN_FORMAT: &str = "moriarty-stage-plan/1";
pub const CONTINUATION_FORMAT: &str = "moriarty-continuation/1";
pub const STAGE_EVENT_FORMAT: &str = "moriarty-stage-event/1";
pub const MAX_AWAITS_PER_ACTION: usize = 16;
pub const MAX_EVENT_BINDERS: usize = 16;
pub const MAX_CAPTURE_SLOTS: usize = 256;
pub const MAX_CONTINUATION_BYTES: usize = 4 * 1_048_576;
pub const MAX_EVENT_BYTES: usize = 65_536;
/// Inserted verbatim into every `/2` output object.
pub const STAGE_LABELS: [(&str, &str); 3] = [
    ("qualification", "local-reference-stage"),
    ("authentication", "none"),
    ("consumption", "untracked"),
];

/// A `/2` program that passed profile, validation, plan invariants and static
/// costing, with its canonical digests. Only `load` constructs one; there is
/// no public constructor, `Default` or `Deserialize`.
#[derive(Debug, Clone)]
pub struct StagedProgram {
    program: Program,
    plan: StagePlan,
    program_hash: [u8; 32],
    plan_hash: [u8; 32],
}
impl StagedProgram {
    pub fn program(&self) -> &Program {
        &self.program
    }
    pub fn plan(&self) -> &StagePlan {
        &self.plan
    }
    pub fn program_hash(&self) -> [u8; 32] {
        self.program_hash
    }
    pub fn plan_hash(&self) -> [u8; 32] {
        self.plan_hash
    }
    /// Declaration index of an action; plan and program actions share it.
    pub(crate) fn action_index(&self, name: &str) -> Option<usize> {
        self.program.actions.iter().position(|a| a.name == name)
    }
}

/// Parses, admits and plans a `moriarty/2` source. Legacy `parse`/`validate`
/// remain `/1`-only, so a `/2` source fails every legacy consumer.
pub fn load(src: &str) -> Result<StagedProgram, Error> {
    let program = crate::parser::parse_v2(src)?;
    profile::require_core2(&program)?;
    validate::validate_v2(&program)?;
    let mut plan = plan::build(&program)?;
    validate::plan_invariants(&program, &plan)?;
    cost::fill(&program, &mut plan)?;
    validate::cost_invariants(&program, &plan)?;
    let program_hash = digest::program_hash(&program)?;
    plan.program_hash = digest::hex(&program_hash);
    let plan_hash = digest::plan_hash(&plan)?;
    Ok(StagedProgram {
        program,
        plan,
        program_hash,
        plan_hash,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NamedValue {
    pub name: String,
    pub value: Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageLogical {
    pub profile: String,
    /// Mandatory key; `None` is written as `null`.
    #[serde(deserialize_with = "Option::deserialize")]
    pub cap: Option<UInt>,
    pub committed: UInt,
}
/// Resume input. `site` and `captures` are the executing action's pending
/// frame; `boundary` is the legacy Snapshot. Nothing here is authenticated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Continuation {
    pub format: String,
    pub program_hash: String,
    pub plan_hash: String,
    pub action: String,
    pub site: u16,
    pub captures: Vec<NamedValue>,
    pub logical: StageLogical,
    pub boundary: Snapshot,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageEvent {
    pub format: String,
    pub action: String,
    pub label: String,
    pub values: Vec<NamedValue>,
}
/// Local frame of one action. `site == 0` exactly when `captures` is empty
/// and the action is idle; no padding is represented locally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PendingFrame {
    pub site: u16,
    pub captures: Vec<NamedValue>,
}
impl PendingFrame {
    pub(crate) fn idle() -> Self {
        Self {
            site: 0,
            captures: Vec::new(),
        }
    }
}
/// Output only. Segment history is never decoded or trusted on resume, so no
/// `Deserialize` exists over the shared Await-bearing `Statement`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StageSegment {
    pub action: String,
    pub frame_pre: PendingFrame,
    pub frame_post: PendingFrame,
    pub execution: Execution,
}
/// `residual` lists every post-boundary liability with a positive amount, in
/// the existing complete `Liability` representation.
// Public outcome; boxing `Waiting` would change every caller's pattern.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum StageOutcome {
    Waiting {
        advanced: bool,
        segment: Option<StageSegment>,
        continuation: Continuation,
        attempted: UInt,
        residual: Vec<Liability>,
    },
    Complete {
        segment: StageSegment,
        committed: UInt,
        attempted: UInt,
        residual: Vec<Liability>,
    },
}
/// A rejected invocation publishes no segment, effects or continuation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, thiserror::Error)]
#[error("{error}")]
pub struct StageRejection {
    pub error: Error,
    pub attempted: UInt,
    pub committed: UInt,
}

/// Positive residual duties of a boundary, in its (name-sorted) order.
pub(crate) fn residual(snapshot: &Snapshot) -> Vec<Liability> {
    snapshot
        .liabilities
        .iter()
        .filter(|l| l.amount.0 > 0)
        .cloned()
        .collect()
}
===== END finite-composition/composition-after/crates/mori-core/src/stage/mod.rs =====

===== BEGIN finite-composition/composition-after/crates/mori/src/commands/test.rs MODE raw ALIASES [] =====
use super::core_io;
use crate::shell::Shell;
use serde::Deserialize;
use std::{path::PathBuf, process::ExitCode};

#[cfg(feature = "core2-disjoint-draft")]
mod disjoint_test;
mod stage_test;

#[derive(Debug, clap::Args)]
pub struct Args {
    pub dir: PathBuf,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    profile: String,
    cases: Vec<Case>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    name: String,
    source: PathBuf,
    scenario: PathBuf,
    action: String,
    expected: Expected,
}
#[derive(Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
enum Expected {
    Accepted {
        post: mori_core::Snapshot,
        effects: Vec<mori_core::Effect>,
    },
    Rejected {
        code: String,
    },
}

pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let value: serde_json::Value = core_io::read_json(&args.dir.join("mori-tests.json"))?;
    if value.get("profile").and_then(serde_json::Value::as_str) == Some("moriarty-tests/2") {
        return stage_test::run(&args.dir, value, shell);
    }
    if value.get("profile").and_then(serde_json::Value::as_str)
        == Some("moriarty-tests/disjoint-draft/1")
    {
        #[cfg(feature = "core2-disjoint-draft")]
        return disjoint_test::run(&args.dir, value, shell);
        #[cfg(not(feature = "core2-disjoint-draft"))]
        return Err(core_io::operation(
            "disjoint draft requires feature core2-disjoint-draft",
        ));
    }
    let manifest: Manifest = serde_json::from_value(value).map_err(core_io::operation)?;
    if manifest.profile != "moriarty-tests/1"
        || manifest.cases.is_empty()
        || manifest.cases.len() > 256
    {
        return Err(core_io::operation(
            "expected a nonempty bounded moriarty-tests/1 manifest",
        ));
    }
    let mut names = std::collections::BTreeSet::new();
    let mut reports = Vec::new();
    let mut failed = 0;
    for case in manifest.cases {
        if case.name.is_empty() || !names.insert(case.name.clone()) {
            return Err(core_io::operation(
                "test case names must be nonempty and unique",
            ));
        }
        let source = core_io::read_source(&args.dir.join(&case.source))?;
        let scenario: mori_core::Scenario = core_io::read_json(&args.dir.join(&case.scenario))?;
        let result = mori_core::parse(&source)
            .and_then(|p| mori_core::simulate(&p, &case.action, &scenario));
        let passed = match (&case.expected, &result) {
            (Expected::Accepted { post, effects }, Ok(actual)) => {
                post == &actual.post && effects == &actual.effects
            }
            (Expected::Rejected { code }, Err(error)) => code == &error.code,
            _ => false,
        };
        if !passed {
            failed += 1;
        }
        reports.push(serde_json::json!({"name":case.name,"passed":passed,"actual":result}));
    }
    shell.status(
        "Tested",
        format!("{} passed; {} failed", reports.len() - failed, failed),
    );
    let report = aggregate_report(failed == 0, reports)?;
    println!("{report}");
    Ok(if failed == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

/// Pretty report identical to exporting the whole value, without applying the
/// single-document 1 MiB cap to the aggregate. Every case is exported, bounded
/// and escaped by the unchanged mori_core::json::export; only trusted
/// structural bytes are added here. Nothing is written until all cases export.
fn aggregate_report(passed: bool, mut cases: Vec<serde_json::Value>) -> super::Result<String> {
    // Field order follows serde_json::Map exactly as the legacy json! wrapper did.
    let mut order = serde_json::Map::new();
    order.insert("status".into(), serde_json::Value::Null);
    order.insert("cases".into(), serde_json::Value::Null);
    let mut out = String::from("{");
    for (field, key) in order.keys().enumerate() {
        out.push_str(if field == 0 { "\n  " } else { ",\n  " });
        if key == "status" {
            out.push_str(if passed {
                r#""status": "passed""#
            } else {
                r#""status": "failed""#
            });
            continue;
        }
        out.push_str(r#""cases": ["#);
        let count = cases.len();
        for (index, case) in std::mem::take(&mut cases).into_iter().enumerate() {
            let case = mori_core::json::export(&case).map_err(core_io::operation)?;
            out.push_str(if index == 0 { "\n    " } else { ",\n    " });
            // Pretty JSON escapes newlines inside strings, so every raw newline
            // is structural and takes the enclosing array-element indentation.
            out.push_str(&case.replace('\n', "\n    "));
        }
        out.push_str(if count == 0 { "]" } else { "\n  ]" });
    }
    out.push_str("\n}");
    Ok(out)
}

#[cfg(test)]
mod report_tests {
    use super::aggregate_report;
    use serde_json::{Value, json};

    const HOSTILE: &str = "x\n}\"\\\r\u{1b}[31m\u{7f}\u{85}\u{202e}\u{1f9ea}";

    fn case(index: usize, padding: usize) -> Value {
        json!({
            "name": format!("case-{index:03}{HOSTILE}"),
            "passed": index.is_multiple_of(2),
            "actual": {"Ok": {
                "pad": "\u{202e}".repeat(padding),
                "empty": [],
                "none": {},
                "nested": [[{"k": [1, -2, 3.5, null, true, HOSTILE]}]]
            }}
        })
    }

    fn whole(passed: bool, cases: &[Value]) -> Value {
        json!({"status": if passed { "passed" } else { "failed" }, "cases": cases})
    }

    #[test]
    fn small_aggregate_is_byte_identical_to_whole_export() {
        for passed in [true, false] {
            for count in [1, 2, 3] {
                let cases: Vec<_> = (0..count).map(|index| case(index, 1)).collect();
                let legacy = mori_core::json::export(&whole(passed, &cases)).unwrap();
                assert_eq!(aggregate_report(passed, cases).unwrap(), legacy);
            }
        }
    }

    #[test]
    fn full_256_aggregate_exceeds_whole_cap_and_round_trips_safely() {
        let cases: Vec<_> = (0..256).map(|index| case(index, 1024)).collect();
        let expected = whole(false, &cases);
        assert!(mori_core::json::export(&expected).is_err());
        let text = aggregate_report(false, cases).unwrap();
        assert!(text.len() > 1_048_576);
        assert!(text.is_ascii());
        assert!(text.bytes().all(|b| b == b'\n' || !b.is_ascii_control()));
        assert!(text.ends_with("\n}"));
        assert_eq!(serde_json::from_str::<Value>(&text).unwrap(), expected);
    }

    #[test]
    fn any_individually_oversized_case_fails_before_output() {
        // Raw pretty size over 1 MiB, then raw size under 1 MiB but escaped size over it.
        for pad in [
            json!("x".repeat(1_048_576)),
            json!("\u{202e}".repeat(200_000)),
        ] {
            let mut cases: Vec<_> = (0..256).map(|index| case(index, 1)).collect();
            cases[255]["actual"]["Ok"]["pad"] = pad;
            assert!(mori_core::json::export(&cases[255]).is_err());
            assert!(aggregate_report(true, cases).is_err());
        }
    }

    #[test]
    fn per_case_depth_limit_applies_and_aggregate_is_not_reader_profile() {
        // Case root is depth 0 under export, so the innermost scalar of
        // `actual` sits at nested + 1 and the reader accepts depth 32.
        let deep = |nested: usize| {
            let actual = (0..nested).fold(json!(0), |inner, _| json!([inner]));
            vec![json!({"name": "deep", "passed": true, "actual": actual})]
        };
        let cases = deep(31);
        assert!(mori_core::json::export(&whole(true, &cases)).is_err());
        let text = aggregate_report(true, cases.clone()).unwrap();
        assert!(mori_core::json::parse(&text).is_err());
        assert_eq!(
            serde_json::from_str::<Value>(&text).unwrap(),
            whole(true, &cases)
        );
        assert!(aggregate_report(true, deep(32)).is_err());
    }
}
===== END finite-composition/composition-after/crates/mori/src/commands/test.rs =====

===== BEGIN finite-composition/FACTS.json MODE json-value ALIASES [] =====
{"recordedAt":"2026-10-18T11:01:31.802312+00:00","actualMain":"7396d7d316ab72957bca9709483f4638daa0d5cb","prospectiveMainNOTYETMERGED":{"candidateHead":"7ef60cd5e67752dbdfbbfce32a06b40ee60e2870","candidateTree":"2e7037c419161d53549fdb55bfb7d0193bdbe94b","requiredMergeParents":["7396d7d316ab72957bca9709483f4638daa0d5cb","7ef60cd5e67752dbdfbbfce32a06b40ee60e2870"],"requiredResultTree":"2e7037c419161d53549fdb55bfb7d0193bdbe94b","exact36ChangedPathsAndBlobs":[{"path":"crates/fixtures/core-disjoint-product/approval-true.json","blob":"acf1f0f770aabdd2fcefdeb7b62cb4f7b6794836","sha256":"33bb3b6a2b2f6c0175f9c6398668ba7124db736d1da41e6424c70f3a0ad14cbc"},{"path":"crates/fixtures/core-disjoint-product/left-first-0.json","blob":"8580069ed85cc924162739c813e8184433d72e52","sha256":"59cd7742cde3e57fcf366b0a350d4a6d14a3b57475f18384ded995582ad5d13b"},{"path":"crates/fixtures/core-disjoint-product/left-first-1.json","blob":"b32161b0a34216f02ebbe10910bbaa9f4cb90505","sha256":"7fa1ae071d23fdc5b57bc53d1340a9b82b56938c79c86c01f01398bb5a0b94a3"},{"path":"crates/fixtures/core-disjoint-product/left-first-2.json","blob":"3b3590e0e1e1bab6cc66c2dbb58055101272d8c7","sha256":"4d8cb6085af945962e3522b9c7acc6029cf8fef0bdffdeae6c5fd4f4b3531f96"},{"path":"crates/fixtures/core-disjoint-product/left-first-3.json","blob":"14f2f174d4f089311e64a0d351ba8cb69721d544","sha256":"75fe97ee33932fea34992fa45bad8220d5b8f02c8f924d0e97ae5ac1e856d87a"},{"path":"crates/fixtures/core-disjoint-product/left-first-4.json","blob":"d7b0c9e68a2079fc4010d2fb04cd5770eb59b040","sha256":"82a2a6e16e1321b7ee0dde70f219658218e748d54a3121c80ca15efcdf25562b"},{"path":"crates/fixtures/core-disjoint-product/mori-tests.json","blob":"45a7be754d6b298524d7e5f3bcbf567f0465e1ea","sha256":"76371519e3759ac87a85a63be6187a9eb6bacf84c499d112b5dd6c9c99c1654a"},{"path":"crates/fixtures/core-disjoint-product/public-report.json","blob":"df23b6e6a5604b16f0211e10559faf79bbc44506","sha256":"0f51b2c2c1eb3d259d3c5b69689924761822e7f9b50809a4497450b2037153f5"},{"path":"crates/fixtures/core-disjoint-product/quote43.json","blob":"7ddadbfeef07f2569d03f738cc2313c607bc39e0","sha256":"43093a5c32bdadea307bc89149d17a472dedcd12d55d762cf28d35227b0b9b6f"},{"path":"crates/fixtures/core-disjoint-product/receipt-true.json","blob":"d314b62a39b252abfc13df3dcf817e2dc6f2110a","sha256":"a730a5b45e532dfeb4e26860cc0d15265b893bbaa81bf95c79d1214ce92feee2"},{"path":"crates/fixtures/core-disjoint-product/reserve-plan.json","blob":"03fa954f70205efe0a3cfcd6d3db4137ef86e59c","sha256":"cba369458324aee66d01c39839b4705271097e3bb16081816c43518663c962d3"},{"path":"crates/fixtures/core-disjoint-product/reserve-plan.preimage.bin","blob":"6e731b090aa9750f14888adc0311feae95f7c88b","sha256":"634bf5ef487e9f4b2a2828d89a6dd2afe158e0edc9514437fbae04245dbf7ef7"},{"path":"crates/fixtures/core-disjoint-product/reserve-program.json","blob":"5974991b00b917bd6eda83679facd027aa396d07","sha256":"d5b804e4fc0ba4925f8ad0960e3f4c0ee22ef1c743ec4d9ed26bbf500cf73a84"},{"path":"crates/fixtures/core-disjoint-product/reserve-program.preimage.bin","blob":"d0309d05e3cf10f045ad1c20e4a9b63229e68408","sha256":"60ff43afae17f7d7d4f32e734f4cb841aff28552a7ec8223948a55e3b25b550a"},{"path":"crates/fixtures/core-disjoint-product/reserve-scenario.json","blob":"a162ae9dcf9c0942bf2e2ca1bdd44de4c757916c","sha256":"acddee29960a1d8850d0df27b0151d24da3247f2c8a37bbe9a8297a17b3971a4"},{"path":"crates/fixtures/core-disjoint-product/reserve.mori","blob":"6e93406d2eb1544c6b1ec6633563fb0bce7dae06","sha256":"260f50443d31e58f62d83c8f7f00f3c110e3c116d216feb93f680f0b92224530"},{"path":"crates/fixtures/core-disjoint-product/right-first-0.json","blob":"8580069ed85cc924162739c813e8184433d72e52","sha256":"59cd7742cde3e57fcf366b0a350d4a6d14a3b57475f18384ded995582ad5d13b"},{"path":"crates/fixtures/core-disjoint-product/right-first-1.json","blob":"20bcca5e2025f022578050cdb6a49b621910c47f","sha256":"326a78bf53739bd3f03a5fd389b1d89cc5579ef5cffabd4121c02050af0ddbf7"},{"path":"crates/fixtures/core-disjoint-product/right-first-2.json","blob":"d75232e5f153a977e08a1fa236950a7d1b3bcf8d","sha256":"c2e7b71577fda7b6e450cd31afe89bb02a1f779db4ef915c4e20be00800a7200"},{"path":"crates/fixtures/core-disjoint-product/right-first-3.json","blob":"0d4a3c904c075cf451f460be7b29873c5317792e","sha256":"2be5c32ff9b12e36358c9c127e230e650e49299d33893e074c838e99a2ae944b"},{"path":"crates/fixtures/core-disjoint-product/right-first-4.json","blob":"dac13efb9f56854c5513c9e487a3c89583c0d7ed","sha256":"a458c0c3895a6aac63d528402a570b4420ce025306b03ed2ff30005b3759a674"},{"path":"crates/fixtures/core-disjoint-product/settlement-true.json","blob":"f605ac1240e3ceb728ceadb5e2bc2456b37efb3a","sha256":"a2fa104de81f75dd66b094334426badaf30939c250acd2f6abe3043520ceb07a"},{"path":"crates/fixtures/core-disjoint-product/two-deliveries-plan.json","blob":"9b18495205c4220b829d8ea647de4009a443ff13","sha256":"80160125dbe2983bba582de5bd958d7ff473d3704fd6c8a7811688dec4455225"},{"path":"crates/fixtures/core-disjoint-product/two-deliveries-plan.preimage.bin","blob":"a09c8d88be9e5cefbadccdb2b10d099c8238a90b","sha256":"d9f7cba42cd4ce1dc1f70544a3592b8a0aba984c8f5cd779c3716d26ca1ac575"},{"path":"crates/fixtures/core-disjoint-product/two-deliveries-program.json","blob":"e3f93dda4f6697fcef92a6c515f6cb60607356cd","sha256":"a641b80643a3ac9d5a93cd4875988477f66021ac780307bcea01ca43a49489ec"},{"path":"crates/fixtures/core-disjoint-product/two-deliveries-program.preimage.bin","blob":"fecd38a8db762454987809ea316882a92cf675c9","sha256":"582b7923087e42b46c318576ad4bb786904b7ac0691a6c4580ed2deb187afe44"},{"path":"crates/fixtures/core-disjoint-product/two-deliveries-scenario.json","blob":"d5db34984ebc2bd764db196e8b59edb53ecf2020","sha256":"5081ad889c98bed4e0ccd31f8f741ac856188523e6885d54832f60681e42e514"},{"path":"crates/fixtures/core-disjoint-product/two-deliveries.mori","blob":"13b93f69e50d30bbd8a6b6e45eacd598ee4e095a","sha256":"b1f89331a3992e8f5135d9af6361a2347d0b45c0be74b0962c63ceb48de640c7"},{"path":"crates/mori-core/Cargo.toml","blob":"19c2cd0223936323a7de57e8454d0370100b9823","sha256":"93318da98ab974c2d30210bd868f98fa5c4af60390395dcbe4f65669e815ac7c"},{"path":"crates/mori-core/src/stage/disjoint.rs","blob":"52c15b01c891183058ddab1efba2938b71d71706","sha256":"5e7aac9b6053657b06362957b14a09b7db5ccf74027f2b97ddeec112923047f1"},{"path":"crates/mori-core/src/stage/mod.rs","blob":"dd76e1992680f32d9a25484e5dc6f0b59664326e","sha256":"e3c4a463319056d0a32ac192c57b3f587584976e08974f4dec04c9e881f80f23"},{"path":"crates/mori-core/tests/disjoint_program_plan.rs","blob":"9e426e2fef3f27aa6181de91fa86a8580a8fc58e","sha256":"fab716ef5fdfb1bd51774333367ec53a5fe8cab20a433afb462c6804c363ebb8"},{"path":"crates/mori/Cargo.toml","blob":"0e82c7610e65d1aa70a734e3043a4b9e4f39815b","sha256":"2202f89a5c40b7bbe503ebb66a406226c2e446ab46580e1c2ff5d4f870ecea1b"},{"path":"crates/mori/src/commands/test.rs","blob":"8ae62d28f2447fbf775f0eaa7ccec5f5db0ed1f9","sha256":"6d79546f1bf9c6271ff45ca4e58b514cc2fc6f5a6b970b55f6b3df4e6788aec8"},{"path":"crates/mori/src/commands/test/disjoint_test.rs","blob":"9904c55ad02af09bb35603c1b280838bd2d30584","sha256":"ce01456416e68b37457f20f3cd12ae4b715716d2be3c801d3cc0c2ddada88ce1"},{"path":"crates/mori/tests/disjoint_test_public.rs","blob":"e2c7019475f000442e50711e0cd550525e15f127","sha256":"b5b49d18e5b8d031728e5118bb7fbc4909585af8c927702b8cfae5af8efc7aa9"}]},"multiConditionObservedHead":"e0d0b4c7e80f53ead164358a68d804dc18c68d89","multiConditionObservedTree":"c1701271cd6afc56092fab416137505aa68d31a8","expectedCombinedTree":"f18c2346ff435f9ac95a7f6f56457c2edde3f71c","all50MultiConditionPathsExactUnchangedInCombinedTree":true,"status":"Mechanical public Git facts only. Does not approve future main or claim composition already merged. Reviewer may explicitly assess this ONE finite known sibling integration shape/tree/blob allowlist in addition to actual pinned base; root must verify final actual merge parents/tree/commit and no further changes. No blanket future-base permission/security/result-carry judgment."}
===== END finite-composition/FACTS.json =====

===== BEGIN ADOPTED-OBSERVATION-CLAUSES.md MODE raw ALIASES [] =====
## Adopted finite Core/2 observation contract

This section defines implementation-source-exposed ordinary observation design adopted on 2026-10-18 under the [recorded design decision](../evidence/utxo/doctorwatson-language-loop-2026-10-17/core2-report-adoption-20261018/DESIGN-ADOPTION.json). Its SHALL clauses are adopted design requirements, not runtime observations, independent goldens or permission for execution. They specify ordinary finite local report observations of already admitted Rust `moriarty/2` / `moriarty-core/2` invocations. They do not redesign financial semantics, admission, cap/refusal policy, continuation validation or security. Existing Core/1 NR-01–06 and financial-report adoption scopes remain separate. Original oracle freezes and all fourteen draft gaps remain unchanged and open; the per-gap dispositions below prevent inferred closure.

M03 local identity design was separately adopted by its designated owner under the [M03 decision](../evidence/utxo/doctorwatson-language-loop-2026-10-17/core2-report-adoption-20261018/M03-DESIGN-ADOPTION.json); the verbatim owner F4 clause appears below. This ordinary author supplies no identity or security decision. The String fields program_hash and plan_hash below retain their report projection. Printed report formatting SHALL NOT become their hash preimage. Global Core/2 expression allocation/sharing is specified in the separately adopted M01 supplement below; this document supplies no concrete Program/Plan identity value.

### Site and capture representation

For one selected action, AwaitSite.site SHALL be its one-based await occurrence in source order; site0 SHALL represent idle entry/completion, with captures:[]. AwaitSite.ordinal SHALL be the zero-based position of that await in the action's complete statements array. These numeric u16 JSON fields SHALL remain distinct from labels, ExprIds and segment-array positions. StagePlan actions SHALL retain program declaration order and statement_count SHALL count all source statements. Site numbering restarts per action.

A site's capture layout SHALL include all action inputs in declaration order, then all preceding let bindings and earlier accepted event binders in source-statement order, each prior event's binders in their declaration order. No liveness pruning is selected. It SHALL exclude state fields and the pending await's own binders. CaptureSlot SHALL contain exactly name, ty, origin and ordinal. Origin Input SHALL use the zero-based input declaration ordinal; Origin Let SHALL use the binding statement's zero-based source ordinal; Origin Event SHALL use the originating await statement's zero-based source ordinal. Multiple event binders at one await therefore share an ordinal and retain binder-array order. StageBinder SHALL contain exactly name and ty. Actual captures SHALL be ordered NamedValue objects with exactly name and typed value, without repeating origin/ordinal; they SHALL match their layout order and retain values captured at their binding positions. Reading current state from a boundary SHALL NOT recapture earlier lets. Empty captures alone do not imply idle; a positive pending site may have no captures. These are layout observations, not an authenticated-input rule or a Program/Plan identity construction.

### Step attachment, values and complete segment snapshots

Every emitted Step SHALL retain exactly index, statement, value, snapshot and effects. Index SHALL be the zero-based original action statement ordinal; it SHALL NOT be renumbered per segment. Statement SHALL be the exact admitted public IR statement, including its numeric ExprId references into that Program's expressions array. Statement amount/value/predicate/guard references SHALL NOT be replaced by evaluated values. This requirement fixes reference meaning and exact comparison, not the parser's global arena allocation.

At start or after an accepted guard, reaching the next await SHALL end the segment before guard evaluation and SHALL emit no Step for that await entry. A normal true resume SHALL begin its segment with exactly one Step for the pending Await: outer value typed Bool true, snapshot equal to the segment pre-boundary and effects:[]. The exact Await statement SHALL retain kind, label, binders and guard ExprId. Its accepted event tuple SHALL NOT replace Step.value. A normal false resume SHALL emit no segment and therefore no public guard Step, value, effects or synthetic snapshot. The next true resume SHALL not repeat the earlier financial prefix. A newly reached later await SHALL again contribute no entry Step.

For each other normally completed statement in an emitted segment, the present outer Step.value SHALL be its typed result: Let's captured RHS; Bool true for Requires/Ensures at their written positions; Next's RHS before assignment; Emit's emitted value; Transfer/Fee's evaluated amount; Liability's added amount; Discharge's discharged amount. UInt SHALL be {type:"UInt",value:canonical unsigned decimal string}; Bool SHALL be {type:"Bool",value:JSON boolean}. Null or omission SHALL NOT replace those results. Liability's result is not its closing amount; Discharge's result is not the remaining duty. This is an adopted Core/2 attachment rule, not inheritance from the Core/1 design or an interest/accrual convention. No Mint, Burn or dedicated accrual construct is introduced; their broader missing financial obligations remain unchanged.

Each Step.snapshot SHALL contain exactly complete state, balances and liabilities after that statement, including preserved rows and present-zero duties. Inputs/captures SHALL NOT become state fields. Balances SHALL retain full account/AssetId/amount rows ordered by exact (asset,account), with AssetId (network,issuer,reference,scale); liabilities SHALL retain full name/AssetId/debtor/creditor/amount rows ordered by name. State values SHALL retain typed values. Empty state SHALL be {}, and empty balance/liability lists []. Segment execution.pre SHALL be the complete starting boundary: the normalized scenario on start, or input continuation.boundary on an accepted resume. Execution.post SHALL equal the last Step.snapshot, or pre if there are no Steps. This local report frame supplies no authenticated-domain or history claim and does not replace boundary admission conditions.

Step.effects SHALL contain only the new statement delta: Let/Requires/Ensures and an accepted Await have []; Next has one State {name,before,after}; Emit one Emit {name,value}; Transfer one Transfer {asset,from,to,amount}; Fee one Fee with those same field names; Liability creation/addition and Discharge each one Liability {name,asset,debtor,creditor,before,after}. The variant name SHALL be the kind field. Effect assets SHALL be full AssetIds, effect amounts/before/after canonical UInt strings; State/Emit values retain typed Value objects. Existing financial transitions determine the values. In already admitted zero/self cases the corresponding effect SHALL remain present even if numeric state does not change. Liability creation SHALL insert its row, addition preserve identity, and discharge retain a closing-zero row; payment remains separate. An effect before zero alone SHALL NOT collapse absent opening duty and present-zero opening duty. Execution.effects SHALL be the ordered concatenation of Step deltas within that segment, retaining repetitions. Cross-segment prefixes are derived histories, not an extra wire field or an authenticated history certificate.

### Status projection and process contract

The public outcome_report envelope SHALL be distinct from derived enum serialization. Every root SHALL have exactly the three qualification members qualification:"local-reference-stage", authentication:"none", consumption:"untracked", plus the applicable members below. All listed members SHALL be present; no others SHALL be added by this projection.

| Typed outcome / ordinary invocation | Additional exact root members and projection |
| --- | --- |
| Waiting after start reaches an await, or true resume reaches a later await | status:"waiting", advanced:true, segment:full StageSegment, continuation:full Continuation, attempted:UInt string, committed:continuation.logical.committed, residual:positive duties of continuation.boundary |
| Waiting after an admitted false resume | status:"waiting", advanced:false, segment:null, continuation:unchanged input Continuation, attempted:UInt string, committed:that continuation.logical.committed, residual:its boundary's positive duties |
| Complete at start or after true resume | status:"complete", segment:full StageSegment, attempted:UInt string, committed:the typed Complete committed amount, residual:positive duties of segment.execution.post; advanced and continuation SHALL be absent, not null |
| Existing StageRejection supplied to the renderer | status:"rejected", error:full typed Error, attempted:its UInt string, committed:its UInt string; advanced, segment, continuation and residual SHALL be absent; no error condition/code/order or refusal-work rule is selected here |

StageSegment SHALL contain exactly action, frame_pre, frame_post and execution. Execution SHALL contain exactly semantics:"moriarty-core/2", action, pre, post, steps and effects. Action names SHALL agree with the selected action. PendingFrame SHALL contain exactly numeric site and ordered captures. Start's frame_pre SHALL be idle; an accepted resume's frame_pre SHALL equal its input pending frame. Waiting frame_post SHALL equal continuation's site/captures and execution.post SHALL equal continuation.boundary. Completion frame_post SHALL be idle. A start with no statements before its first await SHALL still emit a non-null segment with steps:[], effects:[] and pre=post. A normally complete empty action SHALL likewise emit its full empty segment; no synthetic Step is added.

Continuation SHALL retain exactly format:"moriarty-continuation/1", program_hash:String, plan_hash:String, action, site, captures, logical and boundary. StageLogical SHALL retain exactly profile:"moriarty-reference-logical-cost/2", mandatory cap (canonical UInt string or null), and committed (canonical UInt string). A successful waiting start SHALL project the supplied start cap or null; an advancing waiting resume SHALL preserve input continuation.logical.cap, without replacing its reported stored value with a per-call override. This output projection selects no cap admission/editability policy. Positive residual rows SHALL retain the full Liability DTO in boundary order; present-zero rows remain in the complete boundary but are excluded from residual. No report in this scope claims settlement. Error SHALL retain code, message, offset and step, with existing numeric/null representation, without adopting any stage-specific code or admission precedence.

For a successfully printed ordinary public simulate waiting/complete invocation within the existing bounded exporter, stdout SHALL be one complete ASCII JSON report followed by exactly one final LF, with no ordinary status text on stderr, and exit0. Report objects SHALL order member names lexicographically by their original string keys; arrays SHALL retain their prescribed order. Nonempty objects/arrays SHALL use LF-separated entries, two ASCII spaces of indentation per nesting level, comma at each nonfinal entry, colon then one space for object values, and a closing delimiter at the parent's indentation. Empty containers SHALL be {} or []. No trailing spaces, BOM or extra document SHALL appear. Numbers for u16 fields and scale SHALL be unsigned decimal JSON numbers; UInt fields remain strings, booleans lowercase and null literal null. Strings SHALL escape quote/backslash, use the short JSON escapes for backspace/form-feed/LF/CR/tab, encode remaining U+0000–001F and U+007F with lowercase four-digit Unicode escapes, and encode every non-ASCII scalar as lowercase UTF-16 Unicode escape(s), including a surrogate pair when required. Other ASCII characters, including slash, SHALL remain literal. These printing rules concern the report only, not digest preimages. They are adopted independently inspectable rendering choices; no output bytes are generated here. Existing exporter/input bounds and failure handling remain unchanged.

The staged test harness SHALL compare entire JSON values from the same projection while passing actual continuations, not expectations, to successive events. Its aggregate is a separate consumer; no per-invocation byte golden SHALL transfer to the aggregate or vice versa. The existing library pretty-byte check remains separate from public ASCII-export/stdout evidence. Rejected-envelope key projection above adopts no error-byte/process or hostile-case result. Complete stdout/exit/stderr conformance requires its own independently frozen bytes and authorized actual consumer observation.

### Local logical schedule and reporting authority

The adopted schedule for moriarty-reference-logical-cost/2 SHALL charge one unit before each ordinary statement dispatch and each distinct visited expression node, with a fresh node cache per statement. Both Boolean operands SHALL retain existing eager evaluation. Reaching the next await SHALL charge entry1 without visiting its guard and without emitting a Step. Resuming SHALL charge a separate dispatch1 plus the guard's distinct nodes. An admitted false guard SHALL report that invocation's attempted guard work and preserve the input logical.committed; a true guard SHALL charge that guard once and then the executed suffix, including any next await entry. No trace length SHALL substitute for this work count.

For an action with n awaits, segment indices SHALL be0..n: segment0 starts at action entry, segment j>0 begins with the guard dispatch of site j, and a successful segment stops before the next await guard while including its entry charge. ActionPlan.segment_cost SHALL hold n+1 successful segment costs in that order. committed_before[j] SHALL be the sum of earlier successful segment costs, with committed_before[0]=0; plan_total SHALL be their full sum. A normally successful segment j SHALL report attempted=segment_cost[j] and committed=committed_before[j]+segment_cost[j]. A waiting envelope SHALL project committed from its continuation; Complete SHALL use its typed field. False observations SHALL not accumulate into committed, although they remain separately attempted work. This is a local successful-prefix reporting convention, not authenticated lifetime-budget, financial commitment or history authority. No cap-editability, admission, resource ceiling or cap-refusal precedence policy is changed; those decisions remain with their owner.

The previously frozen escrow arithmetic (attempted5/2/6 and committed5/5/11) remains historically CONDITIONAL in its original partial freeze. The [2026-10-18 design adoption](../evidence/utxo/doctorwatson-language-loop-2026-10-17/core2-report-adoption-20261018/DESIGN-ADOPTION.json) establishes this schedule and prefix convention for a NEW independent freeze; it does not upgrade that original record or claim an observed meter pass. A distinct independent custodian SHALL derive and freeze complete new work expectations before an authorized comparison. Original conditional records SHALL remain unchanged. Logical interpretation units SHALL remain separate from circuit cost, DUST, service fees, recovery reserves, wall time and allocation.

===== END ADOPTED-OBSERVATION-CLAUSES.md =====
