Fresh nonauthor Gemini HIGH ORDINARY SOURCE review, new context a2. Review the46 NEW TEST/FIXTURE paths only at HEAD718b1b020718c6b23a12846d32d5d2db9f14de7c/tree1e3fc1912705ce349353accaee7924644e684012. Full physical packet /home/charl/moriarty-c2-opus55-restored-20261017/core2-complete-source-review-packet-a2, MANIFEST SHA256 d4bed337d31e7c4c21a52a70feada599914aa0ddb5430cc6d25f9e97a05b0df6. Previous a1 timed out with no vote and remains preserved; this context changes approach. PRIMARY EVIDENCE INLINE BELOW is99791 bytes: exact full formatted test; whole5-node Program/filled Plan; all3 entire report/raw ASCII stdout pairs; unchanged source/scenario/events; full DERIVATION; complete PROVENANCE/exposure/bytecopy metadata Values; original43payload FREEZE; needed real public APIs. Each report and corresponding stdout file is raw-byte identical and shares one exact body with two named paths; INLINE-BYTE-CUSTODY proves full reconstruction. Metadata views discard no JSON value/array/field; original raw files remain physical. No goldens generated or adapted.

Do NOT reread745 unchanged files or redo the already-approved4doc/receipt audit. All50 original candidate paths/baselines and317 local source/manifest/context members remain physically pinned under full-source/, optional only when a specific source question requires them. PRIOR-FOUR-DOC-DECISIONS.json pins the two existing fresh ordinary+designated-owner decisions, root chronology clarification and exact identity of the4 adopted document/receipt files to reviewed after bytes. Use that record only for the4file scope; new46 test/fixture files require your independent source judgment. No new M03 identity/security condition or blanket security judgment is asked. Overall merge still requires valid ordinary source+separate actual-result review; this source context must not infer correctness from any result or request another consumed ordinary allocation.

Check meaningful actual assertions through true consumers: stage::load whole Program+filled Plan; stage::start/resume/outcome_report3whole library reports; CARGO_BIN_EXE_mori3real processes with complete report JSON, exact public ASCII stdout, exit0 and empty stderr. Only actual immediately preceding waiting continuations feed resumes in typed library and CLI JSON forms. False preserves entire continuation/no segment; true retains release0 and empty positive residual. Attempted5/2/6 and committed5/5/11. Preserve all fields/statements/ExprIds/typed values/full frames/snapshots/effect deltas/order/duties/null/absence/digests. No mirror evaluator/report construction, expected continuation runtime seed, mock/host success, filtering/sorting/normalization or expected generation. Library pretty bytes are outside the selected contract; only public CLI bytes are exact. Inputs initialcap11/omitted resume override are frozen choices, no new policy. Current formatting differs only in one argv-array whitespace layout; full before/after/diff receipts are physically present. All43 frozen payloads/no-LF preimages/zero-byte stderr/original-partial freeze/exposure remain unchanged.

Custodian is NOT blind/preimplementation/wholly-unexposed: adopted public DTO/normative reads incidentally displayed residual helper and InputDecl-to-StageBinder conversion; pre-exposure partial financial facts plus adopted clauses selected new expectations. Source author was implementation-exposed. Inspect faithful consumption of adopted premises, not new semantics/admission/security requirements. All14 gaps remain OPEN with original dispositions; no general authentication/consumption/conservation/constraint/proof/ledger, full composition/finance/U2 or U0–U7/M1–M17/MC/SP/G acceptance. Public developer use remains permissionless. Direct SDK/backend/proof/native imports/calls are absent; exact --no-default-features selected target/ordinary toolchain assumptions are physically recorded, not an execution grant.

No shell/commands/Cargo/rustfmt/parser/CLI/compiler/model/SDK/proof/network/mutation/private material. Begin with this primary, optionally read precise files only if required for a concrete gap. Return ONE complete6field JSON, token doctorwatson-core2-complete-source-ordinary-20261018-a2-whole-review; verdict approve-source/changes-required/abstain. Bounds: summary<=1200chars; <=5findings<=600chars; <=5limitations<=400chars; filesRead<=20 (may cite PROMPT.md inline bundle plus specific physical reads, not all745). Findings must clearly scope approval to46new paths with existing4 decisions/identity reliance, no narrowed partial predicate. No actual result claim; separate fresh result audit remains independent. You have420s wall; concise final JSON without further broad file enumeration.

# Compact exact source-review primary

Raw bodies are unchanged. Report/stdout aliases share a body only after exact raw-byte identity. JSON metadata read views are complete Values; original raw files remain physical. INLINE-BYTE-CUSTODY.json records exact source path/ranges/offsets/hashes.

===== BEGIN EXACT BODY full-source/candidate/crates/mori/tests/core2_escrow_complete.rs bytes=7374 SHA256=c8a48dbccc77169822b57246b91d842573d274af31d54930d65a4d06012ddfbc =====
//! Complete local Core/2 escrow observations through public consumers.
use mori_core::{
    Scenario, UInt,
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
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/core2-escrow-complete")
}

fn json_fixture(name: &str) -> Value {
    serde_json::from_slice(&fs::read(fixtures().join(name)).unwrap()).unwrap()
}

fn assert_report(actual: &Value, name: &str, attempted: &str, committed: &str) {
    assert_eq!(
        actual,
        &json_fixture(&format!("{name}.report.expected.json")),
        "{name}: complete report"
    );
    assert_eq!(actual["attempted"], attempted, "{name}: attempted work");
    assert_eq!(actual["committed"], committed, "{name}: committed work");
}

fn assert_outcome(
    outcome: StageOutcome,
    name: &str,
    attempted: &str,
    committed: &str,
) -> Option<Continuation> {
    let continuation = match &outcome {
        StageOutcome::Waiting { continuation, .. } => Some(continuation.clone()),
        StageOutcome::Complete { .. } => None,
    };
    let actual = stage::outcome_report(Ok(outcome));
    assert_report(&actual, name, attempted, committed);
    continuation
}

#[test]
fn escrow_public_load_matches_complete_program_and_filled_plan() {
    let source = fs::read_to_string(fixtures().join("escrow.mori")).unwrap();
    let program = stage::load(&source).unwrap();
    assert_eq!(
        serde_json::to_value(program.program()).unwrap(),
        json_fixture("escrow.program.expected.json")
    );
    assert_eq!(
        serde_json::to_value(program.plan()).unwrap(),
        json_fixture("escrow.plan.expected.json")
    );
}

#[test]
fn escrow_library_start_false_observation_and_delivery_match_complete_reports() {
    let source = fs::read_to_string(fixtures().join("escrow.mori")).unwrap();
    let program = stage::load(&source).unwrap();
    let scenario: Scenario = serde_json::from_value(json_fixture("escrow.scenario.json")).unwrap();
    let false_event = stage::decode::event(
        &fs::read_to_string(fixtures().join("delivery-false.event.json")).unwrap(),
    )
    .unwrap();
    let true_event = stage::decode::event(
        &fs::read_to_string(fixtures().join("delivery-true.event.json")).unwrap(),
    )
    .unwrap();

    let started = stage::start(&program, "deliver", scenario, Some(UInt(11))).unwrap();
    let first = assert_outcome(started, "start", "5", "5").unwrap();
    let before_observation = first.clone();
    let observed = stage::resume(&program, first, false_event, Some("deliver"), None).unwrap();
    let pending = assert_outcome(observed, "delivery-false", "2", "5").unwrap();
    assert_eq!(pending, before_observation);

    let funded_boundary = pending.boundary.clone();
    let pending_frame = PendingFrame {
        site: pending.site,
        captures: pending.captures.clone(),
    };
    let completed = stage::resume(&program, pending, true_event, Some("deliver"), None).unwrap();
    let StageOutcome::Complete {
        segment, residual, ..
    } = &completed
    else {
        panic!("delivery must complete");
    };
    assert_eq!(segment.frame_pre, pending_frame);
    assert_eq!(segment.execution.pre, funded_boundary);
    assert!(residual.is_empty());
    assert_eq!(segment.execution.post.liabilities[0].name, "release");
    assert_eq!(segment.execution.post.liabilities[0].amount, UInt(0));
    assert!(assert_outcome(completed, "delivery-true", "6", "11").is_none());
}

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Project {
    root: PathBuf,
}

impl Project {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-core2-escrow-complete-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let project = Self { root };
        for name in [
            "escrow.mori",
            "escrow.scenario.json",
            "delivery-false.event.json",
            "delivery-true.event.json",
        ] {
            fs::copy(fixtures().join(name), project.root.join(name)).unwrap();
        }
        project
    }

    fn assert_case(&self, name: &str, args: &[&str], attempted: &str, committed: &str) -> Value {
        let output = Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.root)
            .args([
                "--color",
                "never",
                "simulate",
                "escrow.mori",
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
            fs::read(fixtures().join("empty.stderr.expected.bin")).unwrap(),
            "{name}: exact empty stderr"
        );
        assert!(output.stdout.is_ascii(), "{name}: ASCII stdout");
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_report(&actual, name, attempted, committed);
        assert_eq!(
            output.stdout,
            fs::read(fixtures().join(format!("{name}.stdout.expected.json"))).unwrap(),
            "{name}: exact public stdout bytes"
        );
        actual
    }

    fn save_actual_continuation(&self, continuation: &Value) {
        fs::write(
            self.root.join("actual-continuation.json"),
            serde_json::to_vec(continuation).unwrap(),
        )
        .unwrap();
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn escrow_cli_start_false_observation_and_delivery_match_complete_public_bytes() {
    let project = Project::new();
    let started = project.assert_case(
        "start",
        &["--scenario", "escrow.scenario.json", "--logical-cap", "11"],
        "5",
        "5",
    );
    let first_continuation = started["continuation"].clone();
    project.save_actual_continuation(&started["continuation"]);
    let observed = project.assert_case(
        "delivery-false",
        &[
            "--resume",
            "actual-continuation.json",
            "--event",
            "delivery-false.event.json",
        ],
        "2",
        "5",
    );
    assert_eq!(observed["continuation"], first_continuation);
    assert_eq!(observed["advanced"], false);
    assert!(observed["segment"].is_null());
    project.save_actual_continuation(&observed["continuation"]);
    let completed = project.assert_case(
        "delivery-true",
        &[
            "--resume",
            "actual-continuation.json",
            "--event",
            "delivery-true.event.json",
        ],
        "6",
        "11",
    );
    assert_eq!(
        completed["segment"]["execution"]["pre"],
        observed["continuation"]["boundary"]
    );
    assert!(completed["residual"].as_array().unwrap().is_empty());
    let closing_duty = &completed["segment"]["execution"]["post"]["liabilities"][0];
    assert_eq!(closing_duty["name"], "release");
    assert_eq!(closing_duty["amount"], "0");
}
===== END EXACT BODY full-source/candidate/crates/mori/tests/core2_escrow_complete.rs =====

===== BEGIN EXACT BODY full-source/independent-oracle/escrow.program.expected.json bytes=1765 SHA256=595abd45404c509e3b9300c39ed6ef2c5a409315a0f88aa4771a33a6b8df47c1 =====
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
          "name": "release"
        },
        {
          "binders": [
            {
              "name": "ready",
              "ty": "Bool"
            }
          ],
          "guard": 2,
          "kind": "Await",
          "label": "delivery"
        },
        {
          "amount": 3,
          "asset": "PAY",
          "from": "escrow",
          "kind": "Transfer",
          "to": "bob"
        },
        {
          "amount": 4,
          "kind": "Discharge",
          "name": "release"
        }
      ]
    }
  ],
  "assets": [
    {
      "identity": {
        "issuer": "continuation-escrow-oracle",
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
      "name": "ready"
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
  "name": "ContinuationEscrowOracle",
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
===== END EXACT BODY full-source/independent-oracle/escrow.program.expected.json =====

===== BEGIN EXACT BODY full-source/independent-oracle/escrow.plan.expected.json bytes=786 SHA256=fe8463054c617311e270b1ba0e904350a84899158e550260b39cef174e0305c6 =====
{
  "actions": [
    {
      "action": "deliver",
      "committed_before": [
        "0",
        "5"
      ],
      "plan_total": "11",
      "segment_cost": [
        "5",
        "6"
      ],
      "sites": [
        {
          "binders": [
            {
              "name": "ready",
              "ty": "Bool"
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
          "label": "delivery",
          "ordinal": 2,
          "site": 1
        }
      ],
      "statement_count": 5
    }
  ],
  "format": "moriarty-stage-plan/1",
  "program_hash": "511bcac385cba4fc9b2452b71013a0cfdaa507661684c8f357a7dc647368465e"
}
===== END EXACT BODY full-source/independent-oracle/escrow.plan.expected.json =====

===== BEGIN EXACT BODY full-source/independent-oracle/start.report.expected.json = full-source/independent-oracle/start.stdout.expected.json bytes=11376 SHA256=affd4b4523dd4bcc865ae2cb97e80e8ee592f77a463fcef1a0a8d7d7cc50182c =====
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
          "amount": "90",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "bob",
          "amount": "0",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "escrow",
          "amount": "10",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "observer",
          "amount": "42",
          "asset": {
            "issuer": "continuation-escrow-oracle",
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
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "escrow",
          "name": "release"
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
      "cap": "11",
      "committed": "5",
      "profile": "moriarty-reference-logical-cost/2"
    },
    "plan_hash": "0405200b4e9758ffdd0daede5db60ccb0698f504f7f17b1531450fc99c19ce25",
    "program_hash": "511bcac385cba4fc9b2452b71013a0cfdaa507661684c8f357a7dc647368465e",
    "site": 1
  },
  "qualification": "local-reference-stage",
  "residual": [
    {
      "amount": "10",
      "asset": {
        "issuer": "continuation-escrow-oracle",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      },
      "creditor": "bob",
      "debtor": "escrow",
      "name": "release"
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
            "issuer": "continuation-escrow-oracle",
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
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "before": "0",
          "creditor": "bob",
          "debtor": "escrow",
          "kind": "Liability",
          "name": "release"
        }
      ],
      "post": {
        "balances": [
          {
            "account": "alice",
            "amount": "90",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "0",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "escrow",
            "amount": "10",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-escrow-oracle",
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
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "escrow",
            "name": "release"
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
            "amount": "100",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "0",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "escrow",
            "amount": "0",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-escrow-oracle",
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
                "issuer": "continuation-escrow-oracle",
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
                "amount": "90",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "0",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "42",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
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
                "issuer": "continuation-escrow-oracle",
                "network": "local",
                "reference": "PAY",
                "scale": 0
              },
              "before": "0",
              "creditor": "bob",
              "debtor": "escrow",
              "kind": "Liability",
              "name": "release"
            }
          ],
          "index": 1,
          "snapshot": {
            "balances": [
              {
                "account": "alice",
                "amount": "90",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "0",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "42",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
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
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "escrow",
                "name": "release"
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
            "name": "release"
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
===== END EXACT BODY full-source/independent-oracle/start.report.expected.json = full-source/independent-oracle/start.stdout.expected.json =====

===== BEGIN EXACT BODY full-source/independent-oracle/delivery-false.report.expected.json = full-source/independent-oracle/delivery-false.stdout.expected.json bytes=2524 SHA256=15188506d4800b167b93c550b12630312e48dd80db52f4b2fe66b2101ccd8d3b =====
{
  "advanced": false,
  "attempted": "2",
  "authentication": "none",
  "committed": "5",
  "consumption": "untracked",
  "continuation": {
    "action": "deliver",
    "boundary": {
      "balances": [
        {
          "account": "alice",
          "amount": "90",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "bob",
          "amount": "0",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "escrow",
          "amount": "10",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "observer",
          "amount": "42",
          "asset": {
            "issuer": "continuation-escrow-oracle",
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
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "escrow",
          "name": "release"
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
      "cap": "11",
      "committed": "5",
      "profile": "moriarty-reference-logical-cost/2"
    },
    "plan_hash": "0405200b4e9758ffdd0daede5db60ccb0698f504f7f17b1531450fc99c19ce25",
    "program_hash": "511bcac385cba4fc9b2452b71013a0cfdaa507661684c8f357a7dc647368465e",
    "site": 1
  },
  "qualification": "local-reference-stage",
  "residual": [
    {
      "amount": "10",
      "asset": {
        "issuer": "continuation-escrow-oracle",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      },
      "creditor": "bob",
      "debtor": "escrow",
      "name": "release"
    }
  ],
  "segment": null,
  "status": "waiting"
}
===== END EXACT BODY full-source/independent-oracle/delivery-false.report.expected.json = full-source/independent-oracle/delivery-false.stdout.expected.json =====

===== BEGIN EXACT BODY full-source/independent-oracle/delivery-true.report.expected.json = full-source/independent-oracle/delivery-true.stdout.expected.json bytes=11980 SHA256=72c8938500f666de59da2dbceff22a86c943614cb34dffd47be17ae1a3e943ad =====
{
  "attempted": "6",
  "authentication": "none",
  "committed": "11",
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
            "issuer": "continuation-escrow-oracle",
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
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "before": "10",
          "creditor": "bob",
          "debtor": "escrow",
          "kind": "Liability",
          "name": "release"
        }
      ],
      "post": {
        "balances": [
          {
            "account": "alice",
            "amount": "90",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "10",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "escrow",
            "amount": "0",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-escrow-oracle",
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
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "escrow",
            "name": "release"
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
            "amount": "90",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "0",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "escrow",
            "amount": "10",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-escrow-oracle",
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
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "escrow",
            "name": "release"
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
                "amount": "90",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "0",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "42",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
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
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "escrow",
                "name": "release"
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
                "name": "ready",
                "ty": "Bool"
              }
            ],
            "guard": 2,
            "kind": "Await",
            "label": "delivery"
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
                "issuer": "continuation-escrow-oracle",
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
                "amount": "90",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "0",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "42",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
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
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "escrow",
                "name": "release"
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
            "amount": 3,
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
                "issuer": "continuation-escrow-oracle",
                "network": "local",
                "reference": "PAY",
                "scale": 0
              },
              "before": "10",
              "creditor": "bob",
              "debtor": "escrow",
              "kind": "Liability",
              "name": "release"
            }
          ],
          "index": 4,
          "snapshot": {
            "balances": [
              {
                "account": "alice",
                "amount": "90",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "0",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "42",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
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
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "escrow",
                "name": "release"
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
            "amount": 4,
            "kind": "Discharge",
            "name": "release"
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
===== END EXACT BODY full-source/independent-oracle/delivery-true.report.expected.json = full-source/independent-oracle/delivery-true.stdout.expected.json =====

===== BEGIN EXACT BODY full-source/independent-oracle/escrow.mori bytes=452 SHA256=2eb952dd267922c968764154e9d29fbc17f40f5e45fe8b5dac74d229f3d5d60a =====
language "moriarty/2";
agreement ContinuationEscrowOracle {
  asset PAY = { network: "local", issuer: "continuation-escrow-oracle", reference: "PAY", scale: 0 };
  state marker: UInt = 77;
  action deliver(amount: UInt) {
    transfer(PAY, "alice", "escrow", amount);
    liability("release", PAY, "escrow", "bob", amount);
    await delivery(ready: Bool) when ready;
    transfer(PAY, "escrow", "bob", amount);
    discharge("release", amount);
  }
}
===== END EXACT BODY full-source/independent-oracle/escrow.mori =====

===== BEGIN EXACT BODY full-source/independent-oracle/escrow.scenario.json bytes=1046 SHA256=358fd3b13c0c7021c801d8b057eb934a10a7ec199e450ec0858e40f6234e6be2 =====
{
  "balances": [
    {
      "account": "alice",
      "amount": "100",
      "asset": {
        "issuer": "continuation-escrow-oracle",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      }
    },
    {
      "account": "bob",
      "amount": "0",
      "asset": {
        "issuer": "continuation-escrow-oracle",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      }
    },
    {
      "account": "escrow",
      "amount": "0",
      "asset": {
        "issuer": "continuation-escrow-oracle",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      }
    },
    {
      "account": "observer",
      "amount": "42",
      "asset": {
        "issuer": "continuation-escrow-oracle",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      }
    }
  ],
  "inputs": {
    "amount": {
      "type": "UInt",
      "value": "10"
    }
  },
  "liabilities": [],
  "state": {
    "marker": {
      "type": "UInt",
      "value": "77"
    }
  }
}
===== END EXACT BODY full-source/independent-oracle/escrow.scenario.json =====

===== BEGIN EXACT BODY full-source/independent-oracle/delivery-false.event.json bytes=213 SHA256=4365e299e8b20413548127aaeb2ef75669f576961b593d8e7516fcdbc0ec9d4e =====
{
  "action": "deliver",
  "format": "moriarty-stage-event/1",
  "label": "delivery",
  "values": [
    {
      "name": "ready",
      "value": {
        "type": "Bool",
        "value": false
      }
    }
  ]
}
===== END EXACT BODY full-source/independent-oracle/delivery-false.event.json =====

===== BEGIN EXACT BODY full-source/independent-oracle/delivery-true.event.json bytes=212 SHA256=53648485b97d3e5d2122a8900ae88738298eb5ba046356bc5759b79c69e273b2 =====
{
  "action": "deliver",
  "format": "moriarty-stage-event/1",
  "label": "delivery",
  "values": [
    {
      "name": "ready",
      "value": {
        "type": "Bool",
        "value": true
      }
    }
  ]
}
===== END EXACT BODY full-source/independent-oracle/delivery-true.event.json =====

===== BEGIN EXACT BODY full-source/independent-oracle/DERIVATION.md bytes=15773 SHA256=225fc9ea1ab395a209d886d19367466c9c16b62451ac0ab4d60b74352460d451 =====
# Complete independent original escrow expectations after adoption

Status: **new post-adoption literal expectations; all actual consumers UNRUN**. This packet freezes the exact original escrow Program, fully filled StagePlan, local canonical preimages/digests, original inputs, complete start/false/true public outcome reports, complete public ASCII stdout bytes, frames/effects/duties/captures/typed values/ExprIds and logical work. The falsifiable future predicate is whole-Program, whole-Plan, whole-report and whole-public-stdout equality for these three sequential positive invocations. Expected continuations never become runtime inputs.

## Immutable predecessor and selected authority

The unchanged partial packet is copied in full under original-partial/, including its original freeze SHA `ffd4cb7351e1c55541c748094ab424587d9346b5a31a7c26ac5f932b4febc327`. Every original member and hash was checked before copying. Its missing-premise list, STOP and conditional work classifications remain historically correct; no original file or claim was revised. The four active input copies match those original bytes exactly.

The new ordinary DESIGN-ADOPTION selects M01/M02/M04–M07 from two agreeing contexts. The separate designated-owner M03 DESIGN-ADOPTION selects the exact local digest representation; only normative owner finding F4 was extracted, with no other finding displayed. The proposal documents retain their original UNADOPTED labels; later adoption receipts select them prospectively. Two normative excerpts and F4 are copied under basis/. An adoption's packet-manifest digest identifies that manifest, not the individual Markdown primary. INVENTORY distinguishes all hash subjects.

Source exposure is explicit. Development skill, AGENTS, product contract and read-only status were restored. The original source/scenario/events and pre-exposure independent snapshots/effects were read, followed by the adopted normative design. Public DTO windows in report REVIEW-PACKET.md109–216 and220–278 also displayed the small residual helper and InputDecl-to-StageBinder conversion. This was incidental implementation exposure; this packet is **not wholly implementation-unexposed**, blinded or preimplementation. The complete financial frames, residual10/zero and event meanings had already been independently frozen in the original partial packet before that display. Those pre-exposure facts and normative M04/M05, rather than the displayed helper, select the new residual/report values. M02 already specifies StageBinder's exact name/ty fields. No other implementation body or owner implementation-trace finding was inspected or used. FIELD-PREMISES.json records the basis by field family.

No parser, evaluator, plan builder, compiler or exporter generated expected data. No candidate test/sourceproposal, actual generated Program/report/stdout, current financial test output, private/cache/provider/config material or network was read. No Mori/product/build/Cargo/rustc/rustfmt/model/SDK/VM/proof/native/wallet or Git mutation was run. Plain independently authored Python copied bytes, assembled hand-chosen JSON, checked literal arithmetic and encoded/hashed those values. Only this new outside-Git directory was written. Historical startup blockers and all prior exits/counters remain unchanged.

## Exact whole Program and filled plan

The unchanged452-byte source declares language moriarty/2, agreement ContinuationEscrowOracle, PAY identity `(local,continuation-escrow-oracle,PAY,scale0)`, state marker/UInt/initial77 and action deliver(amount:UInt). All seven Program fields are present. State initializer77 and all declaration/party/binder metadata allocate no expressions. Adopted M01 gives exactly five fresh global nodes in source order:0 Name amount,1 Name amount,2 Name ready,3 Name amount,4 Name amount. There is no sharing, folding, synthetic node, declaration registry or selected-action filter.

The five statements are exactly Transfer/PAY/alice/escrow/amount0; Liability/release/PAY/escrow/bob/amount1; Await/delivery/[ready:Bool]/guard2; Transfer/PAY/escrow/bob/amount3; Discharge/release/amount4. Every field and root is in escrow.program.expected.json. There is no uninvoked additional action in this source. Await.guard2, statement ordinal2 and plan site1 have different meanings even though two happen to use the number2.

M02 makes the only await site1 (one-based occurrence), original action ordinal2 (zero-based). Capture layout is exactly one slot `{name:amount,ty:UInt,origin:Input,ordinal:0}`; amount is the first action input. Pending await's own ready binder is excluded. StageBinder is ready/Bool. ActionPlan retains action deliver, statement_count5, one site and all static cost fields. Segment costs are UInt strings[5,6], committed_before[0,5] and plan_total11. StagePlan contains exactly format moriarty-stage-plan/1, complete program_hash and actions; no plan_hash field is inserted into its own preimage.

## Independent local canonical preimages and identities

M03 F4 defines unframed SHA256 over compact UTF8 JSON with recursively lexical object keys, arrays unchanged, no insignificant whitespace and no trailing LF. Program includes all seven fields, complete arrays and every action. Type/origin strings and UInt decimal strings retain their exact public representations; u8/u16 remain numeric. StagePlan is fully populated with format, program_hash, sites and static costs before hashing. This is not source-file hashing or the public pretty stdout representation.

For these finite inputs every string/key is printable ASCII; independently written Python JSON encoding with sort_keys=True, separators=(',',':'), ensure_ascii=False therefore implements the selected compact UTF8 recipe without any cross-serializer escape ambiguity. program.canonical-preimage.json is934 bytes and hashes to **511bcac385cba4fc9b2452b71013a0cfdaa507661684c8f357a7dc647368465e**. plan.canonical-preimage.json is406 bytes and hashes to **0405200b4e9758ffdd0daede5db60ccb0698f504f7f17b1531450fc99c19ce25**. Both files end in a closing brace, not LF. LOCAL-IDENTITIES.json records exact byte sizes and digest subjects. Separate pretty Program/Plan expectation files have LF and different file hashes; they are not substituted as canonical preimages.

These are deterministic local observational descriptors under the adopted designated-owner choice. They provide no authentication, authorization, consumption tracking, replay protection, history proof, cryptographic soundness conclusion, proof or ledger validity. This custodian makes no new security judgment about that choice.

## Complete source-state trajectory and emitted traces

Opening supplied marker is typed UInt77; input amount typed UInt10. PAY balances in canonical account order are alice100,bob0,escrow0,observer42, total142; liabilities[]. All complete frames are inherited from the original pre-exposure independent freeze. No supplied row, preserved field or present-zero tombstone is dropped.

Start runs Transfer amount10 at original index0/root0: alice90,bob0,escrow10,observer42; no duty yet. Step.value is UInt10, delta one Transfer(alice,escrow,10). Liability at index1/root1 creates release/PAY/escrow/bob10; Step.value is added UInt10, delta Liability0→10. Complete funded boundary retains all four balances, marker77 and present release10. Reaching await ordinal2 charges entry1 but emits no Await Step and does not evaluate ready. Start execution.pre is opening; post is funded; steps contain exactly indices0,1; effects are exactly these two ordered deltas.

Start frame_pre is idle `{site:0,captures:[]}`; frame_post is pending `{site:1,captures:[{name:amount,value:UInt10}]}`. Continuation has exactly format moriarty-continuation/1, independently derived program_hash/plan_hash, action deliver, site1, ordered captures amount10, logical {profile:moriarty-reference-logical-cost/2,cap:"11",committed:"5"}, and the full funded boundary. ready is a binder declaration, not an incoming capture or state field.

The original false event supplies ready Boolfalse. Its guard attempts dispatch1+node2=2, returns normal waiting, advanced=false, segment=null. The entire continuation is unchanged, including cap11, committed5, input capture10 and funded boundary. No public Step, guard value, effect or synthetic snapshot is added. Residual is the complete release10 duty tuple. No source prefix is repeated and no false event binder is appended to the returned capture list.

The original later true event supplies ready Booltrue. Its segment frame_pre is the actual pending frame. Execution.pre is the funded boundary. The first Step is the exact Await statement at original index2/root2, outer typed Booltrue, complete unchanged funded snapshot and effects[]. Index3/root3 Transfer10 changes escrow10→0,bob0→10 with UInt10 and one new Transfer. Index4/root4 Discharge10 reduces release10→present0, reports the discharged UInt10 (not closing0) and one Liability10→0 effect. Complete post is alice90,bob10,escrow0,observer42, marker77 and present release0. Frame_post is idle site0/captures[]. Event ready and capture amount do not become Snapshot.state. No continuation is projected on completion.

StageSegment has exactly action/frame_pre/frame_post/execution. Execution has exactly semantics moriarty-core/2, action/pre/post/steps/effects; every Step has exactly index/statement/value/snapshot/effects. Effects are statement deltas, execution.effects their segment-local ordered concatenation. The complete derived cross-segment history is funding Transfer, creation Liability, delivery Transfer, discharge Liability. A false observation contributes nothing. FINANCIAL-AND-WORK.json retains every full frame, delta, segment prefix and cross-segment prefix as derived expectations, not a new wire/history certificate.

## Financial and work accounting

Alice gross debit10 funds escrow; delivery is a separate escrow gross debit10 to Bob, whose net receipt is10. No explicit fees, refunds, mint, burn or rounding operation occurs; local fees and rounding dust are0. PAY total142 is unchanged at every independently frozen prefix. Marker77 and observer42 are preserved. Release starts absent, creation adds10, false observation retains10, and delivery explicitly discharges10 to a present-zero tombstone. Positive residual10 survives both waiting reports; completion residual[] excludes the zero row while the complete post retains it. Pending escrow10 is distinct from an inferred additional reserve or authenticated funding claim. No interest/day-count/compounding convention is supplied by duty creation.

M07 now prospectively adopts the schedule formerly conditional in the partial freeze: each ordinary dispatch1 plus distinct Name1; await entry1 without guard; resume dispatch1 plus guard Name1. Static successful segment0 costs2+2+1=5; segment1 costs2+2+2=6. Thus segment_cost[5,6], committed_before[0,5], plan_total11. Start attempts5 and reports committed5; false attempts2 but preserves committed5; true attempts6 and reports committed11. Total attempted13 differs from final successful-prefix committed11. Fresh caches per statement and eager boolean evaluation remain the adopted schedule, though these single-name expressions need no cache-hit decision. Trace lengths2/none/3 cannot replace work counts5/2/6.

The cap11 start input and omitted resume overrides are unchanged choices from the original packet, not a new resource/admission decision. The stored cap remains11 in waiting continuations. These expected attempted/committed values are now consequences of the adopted local schedule, not an actual observed meter result or authenticated lifetime consumption. Parsing, normalization, validation, allocation, account lookup, wall time, DUST/service/external/native/circuit/proof/ledger costs remain unmeasured.

## Public report projection, bytes and future consumption

All reports carry qualification local-reference-stage, authentication none and consumption untracked. Waiting start has exactly these plus status waiting, advanced true, segment/full continuation, attempted5, committed5 and residual release10. Waiting false substitutes advanced false, segment null and attempted2, preserving the entire continuation and residual. Complete has the three labels plus status complete, segment, attempted6, committed11,residual[]; advanced and continuation are absent, not null. The report object is not derived StageOutcome enum serialization.

Each `*.report.expected.json` is the full structural JSON expectation. Each separate `*.stdout.expected.json` freezes the whole public ASCII document with recursive lexical object keys, array order retained, two-space LF indentation, standard JSON punctuation/quotes and exactly one final LF. All chosen values contain only printable ASCII strings; Python ensure_ascii=True pretty serialization is an independently selected implementation of the finite M06 recipe for these values. No actual exporter supplied these bytes. Each positive printed invocation expects exit0 and the empty stderr byte file. Public stdout files are distinct from compact identity preimages and test aggregate output. The adopted contract explicitly separates library pretty bytes; **no library pretty-byte assertion is supplied**, because its own exact ordering/newline contract has not been selected here.

CASES.json lists exactly these three invocations and input cap/overrides. A future sequential test must obtain each resume input from the immediately preceding actual waiting result, compare it with the independent expected continuation, and pass the actual value forward. Frozen expected continuations are comparison data only. Complete Program/Plan/report JSON and public byte equality must remain separate predicates. Do not normalize/reorder actual arrays, discard fields/zero rows, replace typed values with roots, use only final balances, or regenerate goldens after a mismatch. CLI argument plumbing/process invocation must be inspected separately; this packet invents no new command syntax.

## Remaining evidence boundaries

M01–M07 now supply all selected positive local wire/representation/identity/work premises. No unresolved placeholder is present in the complete Program/Plan/three reports. This is expectation completeness under adopted premises, not observed admission, source-to-IR correspondence, plan-builder agreement, continuation validation, report/exporter correctness, process/I/O behavior or real execution. Those predicates remain UNRUN. Library pretty bytes remain outside the selected contract. No refusal/error/security test or judgment is authored.

All original fourteen staged gaps remain open with their original individual dispositions in the preserved normative section/partial record: historical line pins, report internals, /2 semantics, format/keys, parser ExprIds, admission premises, error codes, trace bounds, Step.value, Await presence, /2 decode/replay, MOR mapping, boundary-order provenance/R7 contradiction and false-resume cap refusal. New design selection and expectations do not close those empirical or owner predicates. R3 canonical freezes, R7 contradiction, prior failures, profiles/member sets and all U0–U7/M1–M17/MC/SP/G, financial-family/ACTUS/DeFi, staged/composition/private/recursive/native/target/proof/ledger/release requirements retain their original status. Root's separate financial test observation is neither read nor used here.

INVENTORY.json binds all payload sizes/hashes/provenance and exact public design identities. FREEZE.sha256 binds every payload including inventory, excluding itself. The final freeze must be announced to root before consumer-author use. No full language or U2 completion follows.
===== END EXACT BODY full-source/independent-oracle/DERIVATION.md =====

===== BEGIN EXACT BODY full-source/independent-oracle/FREEZE.sha256 bytes=4085 SHA256=2fd5c05c3a25af78a8f71ab74056f9c6a4342ab42f303a3a0090c360cb6e5f22 =====
0263863551aee60c9317f3c5aecc4505a8c9f5b3fdfb024cd33420c0f8038bc2  CASES.json
225fc9ea1ab395a209d886d19367466c9c16b62451ac0ab4d60b74352460d451  DERIVATION.md
23e6a2b3a9e5cd1c32257eb109ff22f7eab574357416b2124c460d6c10ec30e3  FIELD-PREMISES.json
c09871ce28d932ba72428021669dac0d010fce9a6d17e646592ec60248594841  FINANCIAL-AND-WORK.json
270dc58fcb526fba0d54aba1b409a7200aa52e250c873b38590ecefa38f21dc5  INVENTORY.json
1efddc6352afeeec54b563e377b84ade31b9a978e4ebad06a7c372a5500fa85f  LOCAL-IDENTITIES.json
75d528c3ae7fdff555e8fddf9b5f44c55bca40556e8f205179fc93fbc96adbdf  MISSING-PREMISES-RESOLUTION.json
dc07bfc4b8cc9e1ffb4d5408f01479eb7b94895274c00a4dee10cd4707d1bd1a  PROVENANCE.json
17b2af91056891282863330e642bf24a06e65efe3752484ecd860bf48a38f8bd  STATIC-CHECKS.json
d96af01cb1aece70ff6e5cc078715d0ede19df2c85086d3bdb9a551cb6358af3  basis/M03-DESIGN-ADOPTION.json
cbd1eace71075d10e745c1de5dd036b621a70afd8bd7065cc8e51f0fba437ede  basis/M03-F4.txt
0f4889903654f62f965b5a69bb50d2c25605e415f8c052e69c51ebe5a958f160  basis/M03-SOURCE-MANIFEST.json
ceca2fb78112a47b50d424da71bff6eae0e6ee739171f49d4ddade29e7e61308  basis/ORDINARY-DESIGN-ADOPTION.json
4fc2b13f2cc74d007370c7b44e91189b72d3f4425fb469c48eafad503cfcabeb  basis/ORDINARY-REVIEWED-MANIFEST.json
c9db43ed2699049b247e2c2ef900f17746d42ee32a61a2b45a0eb1c3f543f78f  basis/arena-normative-clauses.md
daf5ab3a771d1d94d4919131165a100a1cac58696bf2ebf54c158fe27ee74c97  basis/ordinary-normative-clauses.md
4365e299e8b20413548127aaeb2ef75669f576961b593d8e7516fcdbc0ec9d4e  delivery-false.event.json
15188506d4800b167b93c550b12630312e48dd80db52f4b2fe66b2101ccd8d3b  delivery-false.report.expected.json
15188506d4800b167b93c550b12630312e48dd80db52f4b2fe66b2101ccd8d3b  delivery-false.stdout.expected.json
53648485b97d3e5d2122a8900ae88738298eb5ba046356bc5759b79c69e273b2  delivery-true.event.json
72c8938500f666de59da2dbceff22a86c943614cb34dffd47be17ae1a3e943ad  delivery-true.report.expected.json
644f74eaeaf35a54fcbf6ef7217cbf4bd405ef27a9e3006eca7a5ddae5733458  delivery-true.segment.expected.json
72c8938500f666de59da2dbceff22a86c943614cb34dffd47be17ae1a3e943ad  delivery-true.stdout.expected.json
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  empty.stderr.expected.bin
2eb952dd267922c968764154e9d29fbc17f40f5e45fe8b5dac74d229f3d5d60a  escrow.mori
fe8463054c617311e270b1ba0e904350a84899158e550260b39cef174e0305c6  escrow.plan.expected.json
595abd45404c509e3b9300c39ed6ef2c5a409315a0f88aa4771a33a6b8df47c1  escrow.program.expected.json
358fd3b13c0c7021c801d8b057eb934a10a7ec199e450ec0858e40f6234e6be2  escrow.scenario.json
abed4cd1ba7c9f67f77510bbce9b6ca8f56c0ecae4b682248b2b6d9252cff94e  original-partial/AVAILABLE-EXPECTATIONS.json
7bb941e1d29943224395e49df3be2cb8084a802f7be1aa65c9cb5ca27c996089  original-partial/DERIVATION.md
ffd4cb7351e1c55541c748094ab424587d9346b5a31a7c26ac5f932b4febc327  original-partial/FREEZE.sha256
7662cda70d1e3fb28293bb28a21bf19df1dadc04e8cfb1a46069a2c4cf538a03  original-partial/INVENTORY.json
9d647301a479cd82aee9bbc345d281f7c9cf5a137fb4ab13e883eba2274a5ace  original-partial/MISSING-PREMISES.json
4365e299e8b20413548127aaeb2ef75669f576961b593d8e7516fcdbc0ec9d4e  original-partial/delivery-false.event.json
53648485b97d3e5d2122a8900ae88738298eb5ba046356bc5759b79c69e273b2  original-partial/delivery-true.event.json
2eb952dd267922c968764154e9d29fbc17f40f5e45fe8b5dac74d229f3d5d60a  original-partial/escrow.mori
358fd3b13c0c7021c801d8b057eb934a10a7ec199e450ec0858e40f6234e6be2  original-partial/escrow.scenario.json
0405200b4e9758ffdd0daede5db60ccb0698f504f7f17b1531450fc99c19ce25  plan.canonical-preimage.json
511bcac385cba4fc9b2452b71013a0cfdaa507661684c8f357a7dc647368465e  program.canonical-preimage.json
affd4b4523dd4bcc865ae2cb97e80e8ee592f77a463fcef1a0a8d7d7cc50182c  start.report.expected.json
a607e43093e30a40bb8484ea444301017eed4a864f1564064af24ad303c0f669  start.segment.expected.json
affd4b4523dd4bcc865ae2cb97e80e8ee592f77a463fcef1a0a8d7d7cc50182c  start.stdout.expected.json
e6a7ec9f4410ef5574a89bfc8b8f2e4e6e474ce64ecc6600022aab8cc765c7f8  waiting.continuation.expected.json
===== END EXACT BODY full-source/independent-oracle/FREEZE.sha256 =====

===== BEGIN COMPLETE JSON METADATA VIEW full-source/independent-oracle/PROVENANCE.json rawSourceSHA256=dc07bfc4b8cc9e1ffb4d5408f01479eb7b94895274c00a4dee10cd4707d1bd1a =====
{"exposure_limit":"residual values already independently frozen in partial before display; adopted normative projection chooses same facts; conversion introduces no new value beyond normative StageBinder shape; no implementation inference used","old_partial_unchanged":true,"original_partial_freeze_sha256":"ffd4cb7351e1c55541c748094ab424587d9346b5a31a7c26ac5f932b4febc327","output_scope":"/home/charl/moriarty-c2-opus55-restored-20261017/composition-escrow-complete-independent-oracles","payloads":{"CASES.json":{"basis":["original chosen future inputs unchanged","M05/M06 exact positive projection/process","M03 adopted local diagnostic scope"],"kind":"new independent literal expectation"},"FIELD-PREMISES.json":{"basis":["explicit per-field provenance; distinguishes prior facts/new adopted observations/incidental exposure"],"kind":"new independent literal expectation"},"FINANCIAL-AND-WORK.json":{"basis":["original partial pre-exposure complete financial frames","adopted M01/M02/M04/M05/M07"],"kind":"new independent literal expectation"},"LOCAL-IDENTITIES.json":{"basis":["adopted designated-owner M03 F4 only; no new judgment"],"kind":"new independent literal expectation"},"basis/M03-DESIGN-ADOPTION.json":{"kind":"exact byte copy","source":"/home/charl/moriarty-c2-opus55-restored-20261017/core2-local-identity-adoption-cycle/DESIGN-ADOPTION.json","source_sha256":"d96af01cb1aece70ff6e5cc078715d0ede19df2c85086d3bdb9a551cb6358af3"},"basis/M03-F4.txt":{"kind":"exact decoded normative F4 JSON string plus terminal LF; no other finding displayed","pointer":"/agreedRequiredFields/findings/3","source":"/home/charl/moriarty-c2-opus55-restored-20261017/jobs/doctorwatson-core2-local-identity-boundary-owner-20261018-a1/AGREED-REQUIRED-FIELDS.json","source_sha256":"9f61c93302bf3ac5f1bb041bc38e727396b58402048889082a2265a142893f31"},"basis/M03-SOURCE-MANIFEST.json":{"kind":"exact reviewed packet metadata copy; listed implementation bodies not inspected","source":"/home/charl/moriarty-c2-opus55-restored-20261017/core2-local-identity-owner-packet/MANIFEST.json","source_sha256":"0f4889903654f62f965b5a69bb50d2c25605e415f8c052e69c51ebe5a958f160"},"basis/ORDINARY-DESIGN-ADOPTION.json":{"kind":"exact byte copy","source":"/home/charl/moriarty-c2-opus55-restored-20261017/core2-report-adoption-cycle/DESIGN-ADOPTION.json","source_sha256":"ceca2fb78112a47b50d424da71bff6eae0e6ee739171f49d4ddade29e7e61308"},"basis/ORDINARY-REVIEWED-MANIFEST.json":{"kind":"exact reviewed packet metadata copy; listed implementation bodies not inspected","source":"/home/charl/moriarty-c2-opus55-restored-20261017/core2-report-design-review-packet/MANIFEST.json","source_sha256":"4fc2b13f2cc74d007370c7b44e91189b72d3f4425fb469c48eafad503cfcabeb"},"basis/arena-normative-clauses.md":{"inclusive_lines":[1,85],"kind":"exact public normative excerpt; no implementation excerpt copied","source":"/home/charl/moriarty-c2-opus55-restored-20261017/core2-arena-contract-proposal/REVIEW-PACKET.md","source_sha256":"0a19400754a089f262810f7cb02497c8c280f702c13841985648914822876922"},"basis/ordinary-normative-clauses.md":{"inclusive_lines":[1,108],"kind":"exact public normative excerpt; no implementation excerpt copied","source":"/home/charl/moriarty-c2-opus55-restored-20261017/core2-report-contract-proposal/REVIEW-PACKET.md","source_sha256":"136b55a5aafeb6bf0ea131678a832747aa57f15ef208acf6eb7ef93ad2a306e0"},"delivery-false.event.json":{"kind":"exact byte copy","source":"/home/charl/moriarty-c2-opus55-restored-20261017/composition-escrow-independent-oracles/delivery-false.event.json","source_sha256":"4365e299e8b20413548127aaeb2ef75669f576961b593d8e7516fcdbc0ec9d4e"},"delivery-false.report.expected.json":{"basis":["adopted M01\u2013M07","original partial pre-exposure frames/effects/duties"],"kind":"new independent literal expectation"},"delivery-false.stdout.expected.json":{"basis":["M06 exact public ASCII lexical pretty bytes and terminal LF; not library serializer bytes"],"kind":"new independent literal expectation"},"delivery-true.event.json":{"kind":"exact byte copy","source":"/home/charl/moriarty-c2-opus55-restored-20261017/composition-escrow-independent-oracles/delivery-true.event.json","source_sha256":"53648485b97d3e5d2122a8900ae88738298eb5ba046356bc5759b79c69e273b2"},"delivery-true.report.expected.json":{"basis":["adopted M01\u2013M07","original partial pre-exposure frames/effects/duties"],"kind":"new independent literal expectation"},"delivery-true.segment.expected.json":{"basis":["M04 guard Step original index2/typed true/delta[]","M05 frames/Execution","original partial pre-exposure snapshots/effects"],"kind":"new independent literal expectation"},"delivery-true.stdout.expected.json":{"basis":["M06 exact public ASCII lexical pretty bytes and terminal LF; not library serializer bytes"],"kind":"new independent literal expectation"},"empty.stderr.expected.bin":{"kind":"independent literal empty stderr under adopted M06"},"escrow.mori":{"kind":"exact byte copy","source":"/home/charl/moriarty-c2-opus55-restored-20261017/composition-escrow-independent-oracles/escrow.mori","source_sha256":"2eb952dd267922c968764154e9d29fbc17f40f5e45fe8b5dac74d229f3d5d60a"},"escrow.plan.expected.json":{"basis":["M01","M02 site/capture layout","M03 full filled plan before hashing","M07 static successful-segment costs"],"kind":"new independent literal expectation"},"escrow.program.expected.json":{"basis":["literal unchanged source","M01 whole Program/fresh global arena"],"kind":"new independent literal expectation"},"escrow.scenario.json":{"kind":"exact byte copy","source":"/home/charl/moriarty-c2-opus55-restored-20261017/composition-escrow-independent-oracles/escrow.scenario.json","source_sha256":"358fd3b13c0c7021c801d8b057eb934a10a7ec199e450ec0858e40f6234e6be2"},"original-partial/AVAILABLE-EXPECTATIONS.json":{"kind":"exact byte copy","source":"/home/charl/moriarty-c2-opus55-restored-20261017/composition-escrow-independent-oracles/AVAILABLE-EXPECTATIONS.json","source_sha256":"abed4cd1ba7c9f67f77510bbce9b6ca8f56c0ecae4b682248b2b6d9252cff94e"},"original-partial/DERIVATION.md":{"kind":"exact byte copy","source":"/home/charl/moriarty-c2-opus55-restored-20261017/composition-escrow-independent-oracles/DERIVATION.md","source_sha256":"7bb941e1d29943224395e49df3be2cb8084a802f7be1aa65c9cb5ca27c996089"},"original-partial/FREEZE.sha256":{"kind":"exact byte copy","source":"/home/charl/moriarty-c2-opus55-restored-20261017/composition-escrow-independent-oracles/FREEZE.sha256","source_sha256":"ffd4cb7351e1c55541c748094ab424587d9346b5a31a7c26ac5f932b4febc327"},"original-partial/INVENTORY.json":{"kind":"exact byte copy","source":"/home/charl/moriarty-c2-opus55-restored-20261017/composition-escrow-independent-oracles/INVENTORY.json","source_sha256":"7662cda70d1e3fb28293bb28a21bf19df1dadc04e8cfb1a46069a2c4cf538a03"},"original-partial/MISSING-PREMISES.json":{"kind":"exact byte copy","source":"/home/charl/moriarty-c2-opus55-restored-20261017/composition-escrow-independent-oracles/MISSING-PREMISES.json","source_sha256":"9d647301a479cd82aee9bbc345d281f7c9cf5a137fb4ab13e883eba2274a5ace"},"original-partial/delivery-false.event.json":{"kind":"exact byte copy","source":"/home/charl/moriarty-c2-opus55-restored-20261017/composition-escrow-independent-oracles/delivery-false.event.json","source_sha256":"4365e299e8b20413548127aaeb2ef75669f576961b593d8e7516fcdbc0ec9d4e"},"original-partial/delivery-true.event.json":{"kind":"exact byte copy","source":"/home/charl/moriarty-c2-opus55-restored-20261017/composition-escrow-independent-oracles/delivery-true.event.json","source_sha256":"53648485b97d3e5d2122a8900ae88738298eb5ba046356bc5759b79c69e273b2"},"original-partial/escrow.mori":{"kind":"exact byte copy","source":"/home/charl/moriarty-c2-opus55-restored-20261017/composition-escrow-independent-oracles/escrow.mori","source_sha256":"2eb952dd267922c968764154e9d29fbc17f40f5e45fe8b5dac74d229f3d5d60a"},"original-partial/escrow.scenario.json":{"kind":"exact byte copy","source":"/home/charl/moriarty-c2-opus55-restored-20261017/composition-escrow-independent-oracles/escrow.scenario.json","source_sha256":"358fd3b13c0c7021c801d8b057eb934a10a7ec199e450ec0858e40f6234e6be2"},"plan.canonical-preimage.json":{"basis":"adopted M03 F4, complete literal Program/filled StagePlan","kind":"independent compact UTF8 canonical preimage; no LF"},"program.canonical-preimage.json":{"basis":"adopted M03 F4, complete literal Program/filled StagePlan","kind":"independent compact UTF8 canonical preimage; no LF"},"start.report.expected.json":{"basis":["adopted M01\u2013M07","original partial pre-exposure frames/effects/duties"],"kind":"new independent literal expectation"},"start.segment.expected.json":{"basis":["M04 complete trace and entry suppression","M05 frames/Execution","original partial pre-exposure snapshots/effects"],"kind":"new independent literal expectation"},"start.stdout.expected.json":{"basis":["M06 exact public ASCII lexical pretty bytes and terminal LF; not library serializer bytes"],"kind":"new independent literal expectation"},"waiting.continuation.expected.json":{"basis":["M02 captures/site","M03 digests","M05 complete DTO keys","M07 committed prefix","original partial pre-exposure funded boundary"],"kind":"new independent literal expectation"}},"prohibited_not_read_or_run":["parser/runtime/plan-builder/compiler/exporter/test implementations beyond exactly disclosed incidental snippets","sourceproposal candidate tests","actual generated IR/reports/outputs or current financial test results","Mori/product/build/model/SDK/VM/proof/native/network/private data or Git mutations"],"read_surfaces":["develop/AGENTS/product contract/status; original partial freeze","report REVIEW-PACKET.md1\u2013108 normative proposal; arena REVIEW-PACKET.md1\u201385 normative supplement","ordinary+M03 design-adoption receipts; only F4 decoded finding from owner agreedRequiredFields","report REVIEW-PACKET.md109\u2013216 public DTO block incidentally displayed residual helper","report REVIEW-PACKET.md220\u2013278 public layout DTO block incidentally displayed InputDecl-to-StageBinder conversion"],"status":"new independent literal derivation after explicit adoption; no blinding/preimplementation/wholly implementation-unexposed claim"}
===== END COMPLETE JSON METADATA VIEW full-source/independent-oracle/PROVENANCE.json =====

===== BEGIN COMPLETE JSON METADATA VIEW full-source/independent-oracle/FIELD-PREMISES.json rawSourceSHA256=23e6a2b3a9e5cd1c32257eb109ff22f7eab574357416b2124c460d6c10ec30e3 =====
{"Program_expressions_and_statement_roots":[{"index":0,"node":{"kind":"Name","name":"amount"},"statement":{"amount":0,"asset":"PAY","from":"alice","kind":"Transfer","to":"escrow"}},{"index":1,"node":{"kind":"Name","name":"amount"},"statement":{"amount":1,"asset":"PAY","creditor":"bob","debtor":"escrow","kind":"Liability","name":"release"}},{"index":2,"node":{"kind":"Name","name":"ready"},"statement":{"binders":[{"name":"ready","ty":"Bool"}],"guard":2,"kind":"Await","label":"delivery"}},{"index":3,"node":{"kind":"Name","name":"amount"},"statement":{"amount":3,"asset":"PAY","from":"escrow","kind":"Transfer","to":"bob"}},{"index":4,"node":{"kind":"Name","name":"amount"},"statement":{"amount":4,"kind":"Discharge","name":"release"}}],"StagePlan_actions_statement_count_sites_binders_capture_layout":"M02 + original literal action/input/await binder, source ordinal2 and input ordinal0","StagePlan_static_cost_fields":"M07 dispatch1/node1; await entry1; guard dispatch1/node1; static [5,6],[0,5],11","Step_members_values_and_await_presence":"M04; financial UInt10; Await Booltrue; entry absent; false segment null; no synthetic false guard Step","attempted_committed_and_cost_identity_fields":"M07 adopted successful-prefix convention, M05 projection, start cap11 from original chosen input","capture_values_and_frame_sites":"M02/M05; amount UInt10 at site1, idle site0 captures[]; ready not recaptured into unchanged false continuation","envelope_keysets_presence_null_and_residual":"M05 exact status table and pre-exposure independently derived residual release10 while waiting/none-positive after complete","full_snapshots_effects_duties_history":"original independent partial freeze ffd4cb7351e1c55541c748094ab424587d9346b5a31a7c26ac5f932b4febc327 predates incidental helper display; adopted M04/M05 selects attachment/projection only","library_pretty":"not asserted; no exact separate adopted byte premise","local_digest_preimages_and_strings":"M03 F4 selected by designated owner adoption; compact independently serialized full Program and filled StagePlan; not a new security decision","post_adoption_exposure":"not blind/preimplementation/wholly implementation-unexposed; public DTOs and incidental residual helper/InputDecl-to-StageBinder conversion displayed, recorded; no body used to choose new goldens","public_Program_all_fields":"literal original source + M01 whole-source grammar/arena supplement","remaining_missing_selected_wire_premises":[],"retained_open":"all14 staged gaps and full original roadmap/target/proof/ledger/security-owner exits remain; no actual observation supplied","stdout_exit_stderr":"M06 literal ASCII recursive lexical keys/two spaces/single LF, exit0/empty stderr; actual consumer unrun"}
===== END COMPLETE JSON METADATA VIEW full-source/independent-oracle/FIELD-PREMISES.json =====

===== BEGIN COMPLETE JSON METADATA VIEW BYTECOPY-VALIDATION.json rawSourceSHA256=0b5ea88722e0754b6db72708b9730d625cde67c9986e922b0cdd504c0894a3d2 =====
{"format":"moriarty-core2-escrow-bytecopy/1","source":"/home/charl/moriarty-c2-opus55-restored-20261017/composition-escrow-complete-independent-oracles","freeze_sha256":"2fd5c05c3a25af78a8f71ab74056f9c6a4342ab42f303a3a0090c360cb6e5f22","payload_count":43,"freeze_copied":true,"all_bytes_equal":true,"new_readme_outside_freeze":true,"members":[{"path":"CASES.json","bytes":2009,"sha256":"0263863551aee60c9317f3c5aecc4505a8c9f5b3fdfb024cd33420c0f8038bc2"},{"path":"DERIVATION.md","bytes":15773,"sha256":"225fc9ea1ab395a209d886d19367466c9c16b62451ac0ab4d60b74352460d451"},{"path":"FIELD-PREMISES.json","bytes":3428,"sha256":"23e6a2b3a9e5cd1c32257eb109ff22f7eab574357416b2124c460d6c10ec30e3"},{"path":"FINANCIAL-AND-WORK.json","bytes":29991,"sha256":"c09871ce28d932ba72428021669dac0d010fce9a6d17e646592ec60248594841"},{"path":"INVENTORY.json","bytes":20975,"sha256":"270dc58fcb526fba0d54aba1b409a7200aa52e250c873b38590ecefa38f21dc5"},{"path":"LOCAL-IDENTITIES.json","bytes":629,"sha256":"1efddc6352afeeec54b563e377b84ade31b9a978e4ebad06a7c372a5500fa85f"},{"path":"MISSING-PREMISES-RESOLUTION.json","bytes":1496,"sha256":"75d528c3ae7fdff555e8fddf9b5f44c55bca40556e8f205179fc93fbc96adbdf"},{"path":"PROVENANCE.json","bytes":11996,"sha256":"dc07bfc4b8cc9e1ffb4d5408f01479eb7b94895274c00a4dee10cd4707d1bd1a"},{"path":"STATIC-CHECKS.json","bytes":776,"sha256":"17b2af91056891282863330e642bf24a06e65efe3752484ecd860bf48a38f8bd"},{"path":"basis/M03-DESIGN-ADOPTION.json","bytes":1503,"sha256":"d96af01cb1aece70ff6e5cc078715d0ede19df2c85086d3bdb9a551cb6358af3"},{"path":"basis/M03-F4.txt","bytes":3532,"sha256":"cbd1eace71075d10e745c1de5dd036b621a70afd8bd7065cc8e51f0fba437ede"},{"path":"basis/M03-SOURCE-MANIFEST.json","bytes":2301,"sha256":"0f4889903654f62f965b5a69bb50d2c25605e415f8c052e69c51ebe5a958f160"},{"path":"basis/ORDINARY-DESIGN-ADOPTION.json","bytes":2609,"sha256":"ceca2fb78112a47b50d424da71bff6eae0e6ee739171f49d4ddade29e7e61308"},{"path":"basis/ORDINARY-REVIEWED-MANIFEST.json","bytes":4857,"sha256":"4fc2b13f2cc74d007370c7b44e91189b72d3f4425fb469c48eafad503cfcabeb"},{"path":"basis/arena-normative-clauses.md","bytes":15772,"sha256":"c9db43ed2699049b247e2c2ef900f17746d42ee32a61a2b45a0eb1c3f543f78f"},{"path":"basis/ordinary-normative-clauses.md","bytes":25081,"sha256":"daf5ab3a771d1d94d4919131165a100a1cac58696bf2ebf54c158fe27ee74c97"},{"path":"delivery-false.event.json","bytes":213,"sha256":"4365e299e8b20413548127aaeb2ef75669f576961b593d8e7516fcdbc0ec9d4e"},{"path":"delivery-false.report.expected.json","bytes":2524,"sha256":"15188506d4800b167b93c550b12630312e48dd80db52f4b2fe66b2101ccd8d3b"},{"path":"delivery-false.stdout.expected.json","bytes":2524,"sha256":"15188506d4800b167b93c550b12630312e48dd80db52f4b2fe66b2101ccd8d3b"},{"path":"delivery-true.event.json","bytes":212,"sha256":"53648485b97d3e5d2122a8900ae88738298eb5ba046356bc5759b79c69e273b2"},{"path":"delivery-true.report.expected.json","bytes":11980,"sha256":"72c8938500f666de59da2dbceff22a86c943614cb34dffd47be17ae1a3e943ad"},{"path":"delivery-true.segment.expected.json","bytes":10884,"sha256":"644f74eaeaf35a54fcbf6ef7217cbf4bd405ef27a9e3006eca7a5ddae5733458"},{"path":"delivery-true.stdout.expected.json","bytes":11980,"sha256":"72c8938500f666de59da2dbceff22a86c943614cb34dffd47be17ae1a3e943ad"},{"path":"empty.stderr.expected.bin","bytes":0,"sha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"},{"path":"escrow.mori","bytes":452,"sha256":"2eb952dd267922c968764154e9d29fbc17f40f5e45fe8b5dac74d229f3d5d60a"},{"path":"escrow.plan.expected.json","bytes":786,"sha256":"fe8463054c617311e270b1ba0e904350a84899158e550260b39cef174e0305c6"},{"path":"escrow.program.expected.json","bytes":1765,"sha256":"595abd45404c509e3b9300c39ed6ef2c5a409315a0f88aa4771a33a6b8df47c1"},{"path":"escrow.scenario.json","bytes":1046,"sha256":"358fd3b13c0c7021c801d8b057eb934a10a7ec199e450ec0858e40f6234e6be2"},{"path":"original-partial/AVAILABLE-EXPECTATIONS.json","bytes":25950,"sha256":"abed4cd1ba7c9f67f77510bbce9b6ca8f56c0ecae4b682248b2b6d9252cff94e"},{"path":"original-partial/DERIVATION.md","bytes":7752,"sha256":"7bb941e1d29943224395e49df3be2cb8084a802f7be1aa65c9cb5ca27c996089"},{"path":"original-partial/FREEZE.sha256","bytes":691,"sha256":"ffd4cb7351e1c55541c748094ab424587d9346b5a31a7c26ac5f932b4febc327"},{"path":"original-partial/INVENTORY.json","bytes":5406,"sha256":"7662cda70d1e3fb28293bb28a21bf19df1dadc04e8cfb1a46069a2c4cf538a03"},{"path":"original-partial/MISSING-PREMISES.json","bytes":7530,"sha256":"9d647301a479cd82aee9bbc345d281f7c9cf5a137fb4ab13e883eba2274a5ace"},{"path":"original-partial/delivery-false.event.json","bytes":213,"sha256":"4365e299e8b20413548127aaeb2ef75669f576961b593d8e7516fcdbc0ec9d4e"},{"path":"original-partial/delivery-true.event.json","bytes":212,"sha256":"53648485b97d3e5d2122a8900ae88738298eb5ba046356bc5759b79c69e273b2"},{"path":"original-partial/escrow.mori","bytes":452,"sha256":"2eb952dd267922c968764154e9d29fbc17f40f5e45fe8b5dac74d229f3d5d60a"},{"path":"original-partial/escrow.scenario.json","bytes":1046,"sha256":"358fd3b13c0c7021c801d8b057eb934a10a7ec199e450ec0858e40f6234e6be2"},{"path":"plan.canonical-preimage.json","bytes":406,"sha256":"0405200b4e9758ffdd0daede5db60ccb0698f504f7f17b1531450fc99c19ce25"},{"path":"program.canonical-preimage.json","bytes":934,"sha256":"511bcac385cba4fc9b2452b71013a0cfdaa507661684c8f357a7dc647368465e"},{"path":"start.report.expected.json","bytes":11376,"sha256":"affd4b4523dd4bcc865ae2cb97e80e8ee592f77a463fcef1a0a8d7d7cc50182c"},{"path":"start.segment.expected.json","bytes":8176,"sha256":"a607e43093e30a40bb8484ea444301017eed4a864f1564064af24ad303c0f669"},{"path":"start.stdout.expected.json","bytes":11376,"sha256":"affd4b4523dd4bcc865ae2cb97e80e8ee592f77a463fcef1a0a8d7d7cc50182c"},{"path":"waiting.continuation.expected.json","bytes":1850,"sha256":"e6a7ec9f4410ef5574a89bfc8b8f2e4e6e474ce64ecc6600022aab8cc765c7f8"}]}
===== END COMPLETE JSON METADATA VIEW BYTECOPY-VALIDATION.json =====

===== BEGIN EXACT BODY full-source/candidate/crates/mori-core/src/stage/mod.rs [lines1..90] bytes=3179 SHA256=6618fde4c873381a9bcf337c9fa3f833f36900c87b83cc10ccfc52f9e0b08d6e =====
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
===== END EXACT BODY full-source/candidate/crates/mori-core/src/stage/mod.rs [lines1..90] =====

===== BEGIN EXACT BODY full-source/candidate/crates/mori-core/src/stage/report.rs bytes=1517 SHA256=25e2b88e527a3f9b90951a92fc92ba8c17dd6f16040529dcac0356bb36707b4f =====
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
===== END EXACT BODY full-source/candidate/crates/mori-core/src/stage/report.rs =====

===== BEGIN EXACT BODY full-source/candidate/crates/mori-core/src/stage/admit.rs [lines220..392] bytes=5736 SHA256=dbe157dd039385dc51ed1d2cf7894d5ed03518b65ff29608a351034f68158ef9 =====
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
===== END EXACT BODY full-source/candidate/crates/mori-core/src/stage/admit.rs [lines220..392] =====

===== BEGIN EXACT BODY full-source/candidate/crates/mori/src/commands/simulate.rs [lines1..67] bytes=2525 SHA256=6ff006f67a9fe35ed035dbc70e636ed095f39e5bc946153f36cfda655f75e365 =====
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,

    /// Action to run
    #[arg(long, value_name = "NAME", required_unless_present = "resume")]
    pub action: Option<String>,

    /// Scenario JSON with the local state and inputs
    #[arg(long, value_name = "FILE", required_unless_present = "resume")]
    pub scenario: Option<PathBuf>,
    /// Resume a moriarty/2 continuation (local, unauthenticated, consumption untracked)
    #[arg(
        long,
        value_name = "FILE",
        requires = "event",
        conflicts_with_all = ["scenario", "trace", "target", "profile", "authority", "at", "signature"]
    )]
    pub resume: Option<PathBuf>,
    /// Stage event JSON for --resume (at most 64 KiB)
    #[arg(long, value_name = "FILE", requires = "resume")]
    pub event: Option<PathBuf>,
    /// Generate a complete public replay sidecar against a trusted Target
    #[arg(long)]
    pub trace: bool,
    /// Cap local statement and uncached expression attempts (other costs unmeasured)
    #[arg(long, value_name = "CANONICAL_U128", value_parser = logical_cap, conflicts_with = "trace")]
    pub logical_cap: Option<mori_core::UInt>,
    /// Trusted Target JSON (required with --trace)
    #[arg(long, value_name = "FILE")]
    pub target: Option<std::path::PathBuf>,
    #[command(flatten)]
    pub authorization: super::replay::AuthorizationArgs,
}

impl Args {
    /// Legacy start inputs. Clap requires both whenever --resume is absent.
    fn legacy(&self) -> super::Result<(&str, &Path)> {
        match (self.action.as_deref(), self.scenario.as_deref()) {
            (Some(action), Some(scenario)) => Ok((action, scenario)),
            _ => Err(super::core_io::operation(
                "--action and --scenario are required without --resume",
            )),
        }
    }
}

fn logical_cap(value: &str) -> Result<mori_core::UInt, String> {
    serde_json::from_value(serde_json::Value::String(value.into()))
        .map_err(|_| "expected canonical unsigned decimal string in u128 range".into())
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    // Clap has already refused every --resume/--event conflict, so a
    // malformed invocation reads no file.
    if let Some(resume) = &args.resume {
        return run_resume(&args, resume);
    }
    let (action, scenario_path) = args.legacy()?;
    if args.trace {
        let path = args
===== END EXACT BODY full-source/candidate/crates/mori/src/commands/simulate.rs [lines1..67] =====

===== BEGIN EXACT BODY full-source/candidate/crates/mori/src/commands/simulate.rs [lines161..242] bytes=3109 SHA256=9c25131f3cd6030756d499cbd2b84431fb8b82c5c83a42e2c5dc829478b4729b =====
fn print_stage_result(
    result: Result<mori_core::stage::StageOutcome, mori_core::stage::StageRejection>,
) -> super::Result<ExitCode> {
    let exit = if result.is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    };
    super::core_io::print(&mori_core::stage::outcome_report(result))?;
    Ok(exit)
}

/// `--resume`: reads the source, then the continuation, then the event.
/// Bounded-read, strict-decode, load and admission failures are labelled stage
/// rejections; a source the filesystem cannot read keeps the legacy error.
/// Nothing is recorded, so the same file can be resumed again (untracked).
fn run_resume(args: &Args, resume: &Path) -> super::Result<ExitCode> {
    use mori_core::stage::{self, StageOutcome, StageRejection, decode};
    let event_path = args
        .event
        .as_deref()
        .ok_or_else(|| super::core_io::operation("--resume requires --event"))?;
    let source = super::core_io::read_source(&args.file)?;
    let result = (|| -> Result<StageOutcome, StageRejection> {
        let program = stage::load(&source).map_err(no_work)?;
        let continuation = decode::read_bounded(
            resume,
            decode::MAX_CONTINUATION_BYTES,
            decode::CONTINUATION_TOO_LARGE,
        )
        .and_then(|text| decode::continuation(&text))
        .map_err(no_work)?;
        let event =
            decode::read_bounded(event_path, decode::MAX_EVENT_BYTES, decode::EVENT_TOO_LARGE)
                .and_then(|text| decode::event(&text))
                .map_err(no_work)?;
        stage::resume(
            &program,
            continuation,
            event,
            args.action.as_deref(),
            args.logical_cap,
        )
    })();
    print_stage_result(result)
}

/// A `/2` start. Load and scenario failures are labelled stage rejections; the
/// scenario keeps the legacy 1 MiB strict reader.
fn run_stage_start(
    args: &Args,
    source: &str,
    action: &str,
    scenario: &Path,
) -> super::Result<ExitCode> {
    use mori_core::stage::{self, StageOutcome, StageRejection, decode};
    let result = (|| -> Result<StageOutcome, StageRejection> {
        let program = stage::load(source).map_err(no_work)?;
        let text = decode::read_bounded(scenario, 1_048_576, "Scenario JSON exceeds 1 MiB")
            .map_err(no_work)?;
        let value = mori_core::json::parse(&text).map_err(no_work)?;
        let scenario = serde_json::from_value(value).map_err(|error| {
            no_work(mori_core::Error {
                code: "mori::core::json".into(),
                message: error.to_string(),
                offset: None,
                step: None,
            })
        })?;
        stage::start(&program, action, scenario, args.logical_cap)
    })();
    print_stage_result(result)
}

/// A refusal before any segment evaluation: no work attempted or committed.
fn no_work(error: mori_core::Error) -> mori_core::stage::StageRejection {
    mori_core::stage::StageRejection {
        error,
        attempted: mori_core::UInt(0),
        committed: mori_core::UInt(0),
    }
}
===== END EXACT BODY full-source/candidate/crates/mori/src/commands/simulate.rs [lines161..242] =====

===== BEGIN EXACT BODY full-source/candidate/crates/mori/src/commands/core_io.rs [lines247..251] bytes=171 SHA256=7f1351583d72965b909c84ede5884a48fba83dc868dde6f57ebcd4f165a868cd =====
pub fn print<T: Serialize>(value: &T) -> super::Result<()> {
    let display = mori_core::json::export(value).map_err(operation)?;
    println!("{display}");
    Ok(())
}
===== END EXACT BODY full-source/candidate/crates/mori/src/commands/core_io.rs [lines247..251] =====

===== BEGIN EXACT BODY full-source/candidate/crates/mori-core/src/stage/decode.rs [lines33..51] bytes=650 SHA256=c59051e24ab33eea94a3fe60c269c4cd7fbfeb2ce57b32877aaf46221f633313 =====

fn strict<T: DeserializeOwned>(
    text: &str,
    max: usize,
    too_large: &'static str,
) -> Result<T, Error> {
    let value = crate::json::parse_limited(text, max, too_large)?;
    serde_json::from_value(value).map_err(|e| Error::new("json", e.to_string()))
}

/// Exactly one `Continuation`; unknown fields, a missing `cap` key and
/// non-canonical UInt strings are rejected by the type's derives.
pub fn continuation(text: &str) -> Result<Continuation, Error> {
    strict(text, MAX_CONTINUATION_BYTES, CONTINUATION_TOO_LARGE)
}

pub fn event(text: &str) -> Result<StageEvent, Error> {
    strict(text, MAX_EVENT_BYTES, EVENT_TOO_LARGE)
}
===== END EXACT BODY full-source/candidate/crates/mori-core/src/stage/decode.rs [lines33..51] =====

END PRIMARY. Return the concise whole source review; preserve scope/read limits and all14 OPEN gaps.
