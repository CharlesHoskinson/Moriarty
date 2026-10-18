# Exact current formatted source and independent positive expectations

Raw bodies are byte-exact; report/stdout aliases share one body only after exact equality. Compact metadata views preserve every JSON field/array/value with raw originals physical. Full DERIVATION.md and the complete current stage/admit.rs remain physical for precise follow-up. Actual terminal receipts will be appended before final packet freeze.

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

===== BEGIN full-source/crates/fixtures/core2-multi-condition-complete/start.report.expected.json MODE raw ALIASES ["full-source/crates/fixtures/core2-multi-condition-complete/start.stdout.expected.json"] =====
{
  "advanced": true,
  "attempted": "5",
  "authentication": "none",
  "committed": "5",
  "consumption": "untracked",
  "continuation": {
    "action": "deliver",
    "boundary": {
      "balances": [
        {
          "account": "alice",
          "amount": "10",
          "asset": {
            "issuer": "multi-condition-input",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "bob",
          "amount": "2",
          "asset": {
            "issuer": "multi-condition-input",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "escrow",
          "amount": "13",
          "asset": {
            "issuer": "multi-condition-input",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "observer",
          "amount": "9",
          "asset": {
            "issuer": "multi-condition-input",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        }
      ],
      "liabilities": [
        {
          "amount": "10",
          "asset": {
            "issuer": "multi-condition-input",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "escrow",
          "name": "delivery"
        }
      ],
      "state": {
        "marker": {
          "type": "UInt",
          "value": "77"
        }
      }
    },
    "captures": [
      {
        "name": "amount",
        "value": {
          "type": "UInt",
          "value": "10"
        }
      }
    ],
    "format": "moriarty-continuation/1",
    "logical": {
      "cap": null,
      "committed": "5",
      "profile": "moriarty-reference-logical-cost/2"
    },
    "plan_hash": "e49b50878c7e89463420a00703ec29b697ca9729dc0ba48f428227f93e2031ba",
    "program_hash": "41a9bcd572d3bbbccf5d797cc8d364a1a82a7637e8f0ebdf94b1e3871cbb66f3",
    "site": 1
  },
  "qualification": "local-reference-stage",
  "residual": [
    {
      "amount": "10",
      "asset": {
        "issuer": "multi-condition-input",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      },
      "creditor": "bob",
      "debtor": "escrow",
      "name": "delivery"
    }
  ],
  "segment": {
    "action": "deliver",
    "execution": {
      "action": "deliver",
      "effects": [
        {
          "amount": "10",
          "asset": {
            "issuer": "multi-condition-input",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "from": "alice",
          "kind": "Transfer",
          "to": "escrow"
        },
        {
          "after": "10",
          "asset": {
            "issuer": "multi-condition-input",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "before": "0",
          "creditor": "bob",
          "debtor": "escrow",
          "kind": "Liability",
          "name": "delivery"
        }
      ],
      "post": {
        "balances": [
          {
            "account": "alice",
            "amount": "10",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "2",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "escrow",
            "amount": "13",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "9",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "10",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "escrow",
            "name": "delivery"
          }
        ],
        "state": {
          "marker": {
            "type": "UInt",
            "value": "77"
          }
        }
      },
      "pre": {
        "balances": [
          {
            "account": "alice",
            "amount": "20",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "2",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "escrow",
            "amount": "3",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "9",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          }
        ],
        "liabilities": [],
        "state": {
          "marker": {
            "type": "UInt",
            "value": "77"
          }
        }
      },
      "semantics": "moriarty-core/2",
      "steps": [
        {
          "effects": [
            {
              "amount": "10",
              "asset": {
                "issuer": "multi-condition-input",
                "network": "local",
                "reference": "PAY",
                "scale": 0
              },
              "from": "alice",
              "kind": "Transfer",
              "to": "escrow"
            }
          ],
          "index": 0,
          "snapshot": {
            "balances": [
              {
                "account": "alice",
                "amount": "10",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "2",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "13",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "9",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              }
            ],
            "liabilities": [],
            "state": {
              "marker": {
                "type": "UInt",
                "value": "77"
              }
            }
          },
          "statement": {
            "amount": 0,
            "asset": "PAY",
            "from": "alice",
            "kind": "Transfer",
            "to": "escrow"
          },
          "value": {
            "type": "UInt",
            "value": "10"
          }
        },
        {
          "effects": [
            {
              "after": "10",
              "asset": {
                "issuer": "multi-condition-input",
                "network": "local",
                "reference": "PAY",
                "scale": 0
              },
              "before": "0",
              "creditor": "bob",
              "debtor": "escrow",
              "kind": "Liability",
              "name": "delivery"
            }
          ],
          "index": 1,
          "snapshot": {
            "balances": [
              {
                "account": "alice",
                "amount": "10",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "2",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "13",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "9",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              }
            ],
            "liabilities": [
              {
                "amount": "10",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "escrow",
                "name": "delivery"
              }
            ],
            "state": {
              "marker": {
                "type": "UInt",
                "value": "77"
              }
            }
          },
          "statement": {
            "amount": 1,
            "asset": "PAY",
            "creditor": "bob",
            "debtor": "escrow",
            "kind": "Liability",
            "name": "delivery"
          },
          "value": {
            "type": "UInt",
            "value": "10"
          }
        }
      ]
    },
    "frame_post": {
      "captures": [
        {
          "name": "amount",
          "value": {
            "type": "UInt",
            "value": "10"
          }
        }
      ],
      "site": 1
    },
    "frame_pre": {
      "captures": [],
      "site": 0
    }
  },
  "status": "waiting"
}
===== END full-source/crates/fixtures/core2-multi-condition-complete/start.report.expected.json =====

===== BEGIN full-source/crates/fixtures/core2-multi-condition-complete/document-only.report.expected.json MODE raw ALIASES ["full-source/crates/fixtures/core2-multi-condition-complete/document-only.stdout.expected.json", "full-source/crates/fixtures/core2-multi-condition-complete/amount-only.report.expected.json", "full-source/crates/fixtures/core2-multi-condition-complete/amount-only.stdout.expected.json"] =====
{
  "advanced": false,
  "attempted": "6",
  "authentication": "none",
  "committed": "5",
  "consumption": "untracked",
  "continuation": {
    "action": "deliver",
    "boundary": {
      "balances": [
        {
          "account": "alice",
          "amount": "10",
          "asset": {
            "issuer": "multi-condition-input",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "bob",
          "amount": "2",
          "asset": {
            "issuer": "multi-condition-input",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "escrow",
          "amount": "13",
          "asset": {
            "issuer": "multi-condition-input",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "observer",
          "amount": "9",
          "asset": {
            "issuer": "multi-condition-input",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        }
      ],
      "liabilities": [
        {
          "amount": "10",
          "asset": {
            "issuer": "multi-condition-input",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "escrow",
          "name": "delivery"
        }
      ],
      "state": {
        "marker": {
          "type": "UInt",
          "value": "77"
        }
      }
    },
    "captures": [
      {
        "name": "amount",
        "value": {
          "type": "UInt",
          "value": "10"
        }
      }
    ],
    "format": "moriarty-continuation/1",
    "logical": {
      "cap": null,
      "committed": "5",
      "profile": "moriarty-reference-logical-cost/2"
    },
    "plan_hash": "e49b50878c7e89463420a00703ec29b697ca9729dc0ba48f428227f93e2031ba",
    "program_hash": "41a9bcd572d3bbbccf5d797cc8d364a1a82a7637e8f0ebdf94b1e3871cbb66f3",
    "site": 1
  },
  "qualification": "local-reference-stage",
  "residual": [
    {
      "amount": "10",
      "asset": {
        "issuer": "multi-condition-input",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      },
      "creditor": "bob",
      "debtor": "escrow",
      "name": "delivery"
    }
  ],
  "segment": null,
  "status": "waiting"
}
===== END full-source/crates/fixtures/core2-multi-condition-complete/document-only.report.expected.json =====

===== BEGIN full-source/crates/fixtures/core2-multi-condition-complete/both.report.expected.json MODE raw ALIASES ["full-source/crates/fixtures/core2-multi-condition-complete/both.stdout.expected.json"] =====
{
  "attempted": "10",
  "authentication": "none",
  "committed": "15",
  "consumption": "untracked",
  "qualification": "local-reference-stage",
  "residual": [],
  "segment": {
    "action": "deliver",
    "execution": {
      "action": "deliver",
      "effects": [
        {
          "amount": "10",
          "asset": {
            "issuer": "multi-condition-input",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "from": "escrow",
          "kind": "Transfer",
          "to": "bob"
        },
        {
          "after": "0",
          "asset": {
            "issuer": "multi-condition-input",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "before": "10",
          "creditor": "bob",
          "debtor": "escrow",
          "kind": "Liability",
          "name": "delivery"
        }
      ],
      "post": {
        "balances": [
          {
            "account": "alice",
            "amount": "10",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "12",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "escrow",
            "amount": "3",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "9",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "0",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "escrow",
            "name": "delivery"
          }
        ],
        "state": {
          "marker": {
            "type": "UInt",
            "value": "77"
          }
        }
      },
      "pre": {
        "balances": [
          {
            "account": "alice",
            "amount": "10",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "2",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "escrow",
            "amount": "13",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "9",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "10",
            "asset": {
              "issuer": "multi-condition-input",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "escrow",
            "name": "delivery"
          }
        ],
        "state": {
          "marker": {
            "type": "UInt",
            "value": "77"
          }
        }
      },
      "semantics": "moriarty-core/2",
      "steps": [
        {
          "effects": [],
          "index": 2,
          "snapshot": {
            "balances": [
              {
                "account": "alice",
                "amount": "10",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "2",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "13",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "9",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              }
            ],
            "liabilities": [
              {
                "amount": "10",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "escrow",
                "name": "delivery"
              }
            ],
            "state": {
              "marker": {
                "type": "UInt",
                "value": "77"
              }
            }
          },
          "statement": {
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
          "value": {
            "type": "Bool",
            "value": true
          }
        },
        {
          "effects": [
            {
              "amount": "10",
              "asset": {
                "issuer": "multi-condition-input",
                "network": "local",
                "reference": "PAY",
                "scale": 0
              },
              "from": "escrow",
              "kind": "Transfer",
              "to": "bob"
            }
          ],
          "index": 3,
          "snapshot": {
            "balances": [
              {
                "account": "alice",
                "amount": "10",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "12",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "3",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "9",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              }
            ],
            "liabilities": [
              {
                "amount": "10",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "escrow",
                "name": "delivery"
              }
            ],
            "state": {
              "marker": {
                "type": "UInt",
                "value": "77"
              }
            }
          },
          "statement": {
            "amount": 7,
            "asset": "PAY",
            "from": "escrow",
            "kind": "Transfer",
            "to": "bob"
          },
          "value": {
            "type": "UInt",
            "value": "10"
          }
        },
        {
          "effects": [
            {
              "after": "0",
              "asset": {
                "issuer": "multi-condition-input",
                "network": "local",
                "reference": "PAY",
                "scale": 0
              },
              "before": "10",
              "creditor": "bob",
              "debtor": "escrow",
              "kind": "Liability",
              "name": "delivery"
            }
          ],
          "index": 4,
          "snapshot": {
            "balances": [
              {
                "account": "alice",
                "amount": "10",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "12",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "3",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "9",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              }
            ],
            "liabilities": [
              {
                "amount": "0",
                "asset": {
                  "issuer": "multi-condition-input",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "escrow",
                "name": "delivery"
              }
            ],
            "state": {
              "marker": {
                "type": "UInt",
                "value": "77"
              }
            }
          },
          "statement": {
            "amount": 8,
            "kind": "Discharge",
            "name": "delivery"
          },
          "value": {
            "type": "UInt",
            "value": "10"
          }
        }
      ]
    },
    "frame_post": {
      "captures": [],
      "site": 0
    },
    "frame_pre": {
      "captures": [
        {
          "name": "amount",
          "value": {
            "type": "UInt",
            "value": "10"
          }
        }
      ],
      "site": 1
    }
  },
  "status": "complete"
}
===== END full-source/crates/fixtures/core2-multi-condition-complete/both.report.expected.json =====

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

===== BEGIN full-source/crates/mori-core/src/stage/mod.rs MODE raw ALIASES [] =====
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
===== END full-source/crates/mori-core/src/stage/mod.rs =====

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
