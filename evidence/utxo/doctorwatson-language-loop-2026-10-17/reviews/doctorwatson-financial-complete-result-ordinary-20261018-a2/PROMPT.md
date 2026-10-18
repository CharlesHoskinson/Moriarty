You are the fresh independent non-author ORDINARY result reviewer at HIGH effort. Review the exact public artifacts supplied here; do not repeat an earlier reviewer narrative. No new product execution is authorized. Do NOT run Cargo, Rust, product parser/evaluator/compiler, SDK, VM, models, keys/signing/proofs, ledger/network operations or inspect private captures/caches. Read public packet files only. Do not edit source or evidence. Root owns final reliance/merge. Make no security judgment; prior security review/admission records are provenance only. Return exactly one complete schema-conforming JSON review object, no toolAction/toolSummary or extra keys, no previews or fragments. You may read files to substantiate findings. Do not claim filesRead you did not inspect. If resources prevent a faithful complete assessment, abstain honestly.

TASK: Independently assess the captured ordinary PR205 result on candidate commit7e0d753c40a3e5f694aa226bd657c831da111f65/tree55fab2c3cbd818ffb04eeb7d5930a28758a858b3, unchanged after execution. Existing attempt passed2testfunctions,0failed in52.624672563000786s; two parser inputs and six CLI calls are inferred from passing full control flow, not separately captured per-call telemetry. No rerun. Distinguish source implementation, actual result and narrow acceptance. Review actual Program fixtures and whole-report/raw-byte contract; fixture expected values are not actual per-call captures. Full stdout/stderr below is Cargo test output, not individual CLI output. Require exact test facts, exact original report filenames and profile/logical fields, exact linkage pairs, freeze hash custody, original rustfmt delta and result counts. Do not invent parse_program, explicit validate, from_value, round-trip assertions, separate Expr/Pred arenas, direct standalone3Execution-file assertions, stdout.expected names, total/remaining fields, or exact freeze timestamp. Do not infer broader language, financial, native/target/U2/parent-exit acceptance.

The prior ordinary a1 audit was materially rejected, not lightly corrected. Both original full objects and ROOT-DISPOSITION are preserved in rejected-ordinary-a1/. Review their factual reliability against source, not as votes. This replacement should independently assess exact result, not rubber-stamp root facts. All original bytes of financial-complete-report-cycle/result-review-packet are copied completely under existing-result-review-packet/, including full 388 source pins/candidate, oracles, derivations, prior source reviews, full START/RESULT/STDOUT/STDERR and cleanup. MANIFEST.json pins every copied file and the original manifest. A read-only lossless presentation is included inline to fit context; it is never the consumer oracle. Verify against originals as necessary. Explicitly report limitations; use approve-exact-result-and-guarded-merge only within ordinary review scope, with root responsible for final source/result/security reconciliation and merge. Otherwise changes-required or abstain. Request no new execution.

Review token: doctorwatson-financial-complete-result-ordinary-20261018-a2-whole-review
Packet: /home/charl/moriarty-c2-opus55-restored-20261017/doctorwatson-financial-complete-result-ordinary-20261018-a2-packet
Manifest SHA256: 518dc3e7107bcc71f85ce32161b5edaaf1e70a6e792f94218a9fb43154af5360

===== COMPLETE PUBLIC FILE ACTUAL-REVIEWED-TEST.rs =====
//! Full positive Core/1 financial Programs and public CLI reports.
use serde_json::Value;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/financial-report-contract")
}

fn json_fixture(root: &Path, name: &str) -> Value {
    serde_json::from_slice(&fs::read(root.join(name)).unwrap()).unwrap()
}

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Project {
    root: PathBuf,
}

impl Project {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-financial-report-contract-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        Self { root }
    }

    fn assert_case(&self, fixtures: &Path, case: &Value) -> Value {
        let name = case["name"].as_str().unwrap();
        for key in ["source", "scenario"] {
            let file = case[key].as_str().unwrap();
            fs::copy(fixtures.join(file), self.root.join(file)).unwrap();
        }
        let argv = case["argv_template"]
            .as_array()
            .unwrap()
            .iter()
            .map(|arg| arg.as_str().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(argv[0], "mori");
        let output = Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.root)
            .args(&argv[1..])
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "{name}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stderr = case["stderr"].as_str().unwrap();
        let expected_stderr = fs::read(fixtures.join(stderr)).unwrap();
        assert_eq!(output.stderr, expected_stderr, "{name}: exact empty stderr");
        let stdout = case["stdout"].as_str().unwrap();
        let expected_stdout = fs::read(fixtures.join(stdout)).unwrap();
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        let expected: Value = serde_json::from_slice(&expected_stdout).unwrap();
        assert_eq!(actual, expected, "{name}: complete JSON report");
        assert_eq!(output.stdout, expected_stdout, "{name}: exact stdout bytes");
        actual
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn financial_sources_match_complete_independent_programs() {
    let root = fixtures();
    for (source, expected) in [
        (
            "installment-invoice.mori",
            "installment-invoice.program.expected.json",
        ),
        (
            "conditional-reserve.mori",
            "conditional-reserve.program.expected.json",
        ),
    ] {
        let source_text = fs::read_to_string(root.join(source)).unwrap();
        let program = mori_core::parse(&source_text).expect("financial source must parse");
        assert_eq!(
            serde_json::to_value(program).unwrap(),
            json_fixture(&root, expected),
            "{source}: complete Program"
        );
    }
}

#[test]
fn financial_cli_reports_match_complete_oracles_and_invoice_link() {
    let root = fixtures();
    let cases = json_fixture(&root, "CASES.json");
    let cases = cases["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 6);
    let project = Project::new();
    let mut reports = BTreeMap::new();
    for case in cases {
        let actual = project.assert_case(&root, case);
        reports.insert(case["name"].as_str().unwrap().to_owned(), actual);
    }
    for (first, final_call) in [
        ("invoice-first-ordinary", "invoice-final-ordinary"),
        ("invoice-first-cap42", "invoice-final-cap42"),
    ] {
        assert_eq!(
            reports[first]["execution"]["post"], reports[final_call]["execution"]["pre"],
            "{first} post must equal {final_call} pre in full"
        );
    }
}


===== COMPLETE PUBLIC FILE FOCUSED-SOURCE-FACTS.json =====
{
  "purpose": "Focused source fidelity observations for independent replacement review; reviewer must check actual bytes and may disagree. Not product execution or acceptance.",
  "testSha256": "76ee071ade1e8a7edf815274205f58bf513c29373782a4c9bd798eb55e5caf89",
  "testBytes": 4047,
  "testFunctions": 2,
  "programComparisons": 2,
  "programPath": "mori_core::parse -> serde_json::to_value -> full serde_json::Value equality",
  "programNodes": [
    32,
    33
  ],
  "cliCalls": 6,
  "caseReads": [
    "source",
    "scenario",
    "name",
    "argv_template",
    "stderr",
    "stdout"
  ],
  "caseFieldsNotDirectlyConsumed": [
    "execution",
    "program",
    "exit",
    "logical_cap",
    "expected_logical_used"
  ],
  "reportChecks": [
    "status code Some(0)",
    "exact stderr bytes",
    "complete JSON Value equality",
    "exact stdout bytes"
  ],
  "invoiceLinks": [
    [
      "invoice-first-ordinary",
      "invoice-final-ordinary"
    ],
    [
      "invoice-first-cap42",
      "invoice-final-cap42"
    ]
  ],
  "linkAssertion": "reports[first][\"execution\"][\"post\"] == reports[final_call][\"execution\"][\"pre\"] in full",
  "frozenMemberCount": 32,
  "freezeSha256": "5fca1721249e603226df21d1a2d7ad886a27e66232d1b722dfd76011611d510d",
  "fixtureCaseRecords": [
    {
      "name": "invoice-first-ordinary",
      "argv_template": [
        "mori",
        "--color",
        "never",
        "simulate",
        "installment-invoice.mori",
        "--action",
        "collect",
        "--scenario",
        "invoice-first.scenario.json"
      ],
      "stdout": "invoice-first.ordinary.stdout.json",
      "stdoutBytes": 43761,
      "stdoutSha256": "633d834cebb16b1eb53494b67e43e4c86b037a2e10d66c5be32262c835ba51b0",
      "stderr": "empty.stderr.expected.bin",
      "reportTopLevelFields": [
        "execution",
        "qualification",
        "status"
      ],
      "profileFields": {
        "qualification": "local-reference-execution",
        "status": "accepted"
      },
      "stepCount": 12,
      "effectCount": 7,
      "completeExecutionFileValueMatchesEnvelope": true,
      "qualification": "Fixture-only JSON consistency observation; test does not separately assert standalone Execution file"
    },
    {
      "name": "invoice-first-cap42",
      "argv_template": [
        "mori",
        "--color",
        "never",
        "simulate",
        "installment-invoice.mori",
        "--action",
        "collect",
        "--scenario",
        "invoice-first.scenario.json",
        "--logical-cap",
        "42"
      ],
      "stdout": "invoice-first.cap42.stdout.json",
      "stdoutBytes": 43865,
      "stdoutSha256": "002ba2db5fb66e83f9aea462350c60211dcae9da3fc394c253667ad2b3f04b00",
      "stderr": "empty.stderr.expected.bin",
      "reportTopLevelFields": [
        "execution",
        "logical",
        "profile",
        "qualification",
        "status"
      ],
      "profileFields": {
        "logical": {
          "cap": "42",
          "used": "42"
        },
        "profile": "moriarty-reference-logical-cost/1",
        "qualification": "local-reference-execution",
        "status": "accepted"
      },
      "stepCount": 12,
      "effectCount": 7,
      "completeExecutionFileValueMatchesEnvelope": true,
      "qualification": "Fixture-only JSON consistency observation; test does not separately assert standalone Execution file"
    },
    {
      "name": "invoice-final-ordinary",
      "argv_template": [
        "mori",
        "--color",
        "never",
        "simulate",
        "installment-invoice.mori",
        "--action",
        "collect",
        "--scenario",
        "invoice-final.scenario.json"
      ],
      "stdout": "invoice-final.ordinary.stdout.json",
      "stdoutBytes": 43772,
      "stdoutSha256": "6b086a275a58d2b379a3cf10e11fed627a2aa76c836caa608be5050cfce52b97",
      "stderr": "empty.stderr.expected.bin",
      "reportTopLevelFields": [
        "execution",
        "qualification",
        "status"
      ],
      "profileFields": {
        "qualification": "local-reference-execution",
        "status": "accepted"
      },
      "stepCount": 12,
      "effectCount": 7,
      "completeExecutionFileValueMatchesEnvelope": true,
      "qualification": "Fixture-only JSON consistency observation; test does not separately assert standalone Execution file"
    },
    {
      "name": "invoice-final-cap42",
      "argv_template": [
        "mori",
        "--color",
        "never",
        "simulate",
        "installment-invoice.mori",
        "--action",
        "collect",
        "--scenario",
        "invoice-final.scenario.json",
        "--logical-cap",
        "42"
      ],
      "stdout": "invoice-final.cap42.stdout.json",
      "stdoutBytes": 43876,
      "stdoutSha256": "f7ffcd76dce6a36800f59b46c420f4abc7af043cc212d882e5704be606944868",
      "stderr": "empty.stderr.expected.bin",
      "reportTopLevelFields": [
        "execution",
        "logical",
        "profile",
        "qualification",
        "status"
      ],
      "profileFields": {
        "logical": {
          "cap": "42",
          "used": "42"
        },
        "profile": "moriarty-reference-logical-cost/1",
        "qualification": "local-reference-execution",
        "status": "accepted"
      },
      "stepCount": 12,
      "effectCount": 7,
      "completeExecutionFileValueMatchesEnvelope": true,
      "qualification": "Fixture-only JSON consistency observation; test does not separately assert standalone Execution file"
    },
    {
      "name": "reserve-approved-ordinary",
      "argv_template": [
        "mori",
        "--color",
        "never",
        "simulate",
        "conditional-reserve.mori",
        "--action",
        "allocate",
        "--scenario",
        "reserve-approved.scenario.json"
      ],
      "stdout": "reserve-approved.ordinary.stdout.json",
      "stdoutBytes": 66326,
      "stdoutSha256": "57b83831652c8c74e6559e5390df08462ea3cf0dfa6fe7dc417794d855aeecce",
      "stderr": "empty.stderr.expected.bin",
      "reportTopLevelFields": [
        "execution",
        "qualification",
        "status"
      ],
      "profileFields": {
        "qualification": "local-reference-execution",
        "status": "accepted"
      },
      "stepCount": 15,
      "effectCount": 9,
      "completeExecutionFileValueMatchesEnvelope": true,
      "qualification": "Fixture-only JSON consistency observation; test does not separately assert standalone Execution file"
    },
    {
      "name": "reserve-approved-cap45",
      "argv_template": [
        "mori",
        "--color",
        "never",
        "simulate",
        "conditional-reserve.mori",
        "--action",
        "allocate",
        "--scenario",
        "reserve-approved.scenario.json",
        "--logical-cap",
        "45"
      ],
      "stdout": "reserve-approved.cap45.stdout.json",
      "stdoutBytes": 66430,
      "stdoutSha256": "4673490151308b065ed76e6f63555052772a026a3d032c3fe7eadb479936efe2",
      "stderr": "empty.stderr.expected.bin",
      "reportTopLevelFields": [
        "execution",
        "logical",
        "profile",
        "qualification",
        "status"
      ],
      "profileFields": {
        "logical": {
          "cap": "45",
          "used": "45"
        },
        "profile": "moriarty-reference-logical-cost/1",
        "qualification": "local-reference-execution",
        "status": "accepted"
      },
      "stepCount": 15,
      "effectCount": 9,
      "completeExecutionFileValueMatchesEnvelope": true,
      "qualification": "Fixture-only JSON consistency observation; test does not separately assert standalone Execution file"
    }
  ],
  "actualResult": {
    "testFunctionsPassed": 2,
    "failed": 0,
    "completedParserInputs": 2,
    "completedCLICalls": 6,
    "wallSeconds": 52.624672563000786,
    "countQualification": "2 parser inputs and 6 CLI calls inferred from passing test control flow, not separate per-call telemetry",
    "commit": "7e0d753c40a3e5f694aa226bd657c831da111f65",
    "tree": "55fab2c3cbd818ffb04eeb7d5930a28758a858b3"
  },
  "rejectedClaims": [
    {
      "id": "F1",
      "claim": "parse_program + validate + to_value/from_value + Program round-trip",
      "sourceFact": "financial_sources_match_complete_independent_programs calls mori_core::parse then serde_json::to_value(program) and compares the complete Value to a JSON fixture. There is no parse_program, validate, from_value or Program-typed round-trip in this test.",
      "evidence": "core_financial_report_contract.rs:financial_sources_match_complete_independent_programs"
    },
    {
      "id": "F2",
      "claim": "invoice 27 Expr + 5 Pred; reserve 26 Expr + 7 Pred",
      "sourceFact": "The serialized Program fixtures each expose one expressions arena with respectively 32 and 33 nodes; no separate Expr/Pred arena or asserted 27+5/26+7 split exists in these fixtures or consumer. Such a split is unsupported and is not the test contract.",
      "evidence": "installment-invoice.program.expected.json and conditional-reserve.program.expected.json; Program DTO"
    },
    {
      "id": "F3",
      "claim": "direct complete execution field equality against 3 standalone Execution oracle files",
      "sourceFact": "assert_case reads the CASES stdout and stderr fields only (plus copies source/scenario). It compares the complete actual report Value and raw stdout bytes to the six *.stdout.json files. It does not read CASES execution/program fields or open the three *.execution.expected.json files. Complete execution coverage is through the complete report equality, not an additional standalone Execution-file assertion.",
      "evidence": "core_financial_report_contract.rs:Project::assert_case"
    },
    {
      "id": "F4",
      "claim": "raw stdout against *.stdout.expected.json",
      "sourceFact": "All six CASES stdout names end in .stdout.json (ordinary or cap42/cap45), not .stdout.expected.json.",
      "evidence": "CASES.json:cases[*].stdout"
    },
    {
      "id": "F5",
      "claim": "work fields total and remaining:0",
      "sourceFact": "The metered report envelopes have logical:{cap:\"42\",used:\"42\"} or logical:{cap:\"45\",used:\"45\"}, profile:moriarty-reference-logical-cost/1, qualification:local-reference-execution, status:accepted. There are no total/remaining fields. The ordinary envelope has execution,qualification,status only.",
      "evidence": "six original *.stdout.json complete report fixtures"
    },
    {
      "id": "F6",
      "claim": "freeze created at 2026-10-18T06:16:33Z",
      "sourceFact": "The original FREEZE.sha256 is a digest inventory and neither it nor PROVENANCE.json/INVENTORY.json contains that claimed timestamp. The exact creation-time claim is unsupported by the cited bytes and cannot be accepted. No substitute freeze timestamp is inferred from filesystem metadata.",
      "evidence": "FREEZE.sha256,PROVENANCE.json,INVENTORY.json"
    },
    {
      "id": "F7",
      "claim": "Program struct round-trip equality limitation and validation coverage",
      "sourceFact": "The limitations repeat the nonexistent round-trip, and validation is not a direct call in the Program comparison test. The observed scope must follow the exact code: two parse\u2192to_value complete-Value comparisons, six real CLI full-JSON/raw-byte comparisons, and two full actual invoice post\u2192pre comparisons (ordinary and cap42).",
      "evidence": "both complete test functions; original whole limitations[2]"
    }
  ],
  "limits": [
    "No product tests rerun",
    "No new native/SDK/compiler/model/proof/ledger execution",
    "Full result retained, no per-call stdout runtime telemetry; successful test control flow establishes completion counts",
    "Not universal parser/cost/financial/ledger or U2/native/parent-exit acceptance",
    "No new security judgment; original security-context records included only for prior admission provenance",
    "No exact freeze-time inference; byte/custody identities remain independently checkable"
  ]
}


===== COMPLETE PUBLIC FILE existing-result-review-packet/source/candidate/crates/fixtures/financial-report-contract/CASES.json =====
{
  "cases": [
    {
      "action": "collect",
      "argv_template": [
        "mori",
        "--color",
        "never",
        "simulate",
        "installment-invoice.mori",
        "--action",
        "collect",
        "--scenario",
        "invoice-first.scenario.json"
      ],
      "execution": "invoice-first.execution.expected.json",
      "exit": 0,
      "expected_logical_used": null,
      "logical_cap": null,
      "name": "invoice-first-ordinary",
      "program": "installment-invoice.program.expected.json",
      "scenario": "invoice-first.scenario.json",
      "source": "installment-invoice.mori",
      "stderr": "empty.stderr.expected.bin",
      "stdout": "invoice-first.ordinary.stdout.json"
    },
    {
      "action": "collect",
      "argv_template": [
        "mori",
        "--color",
        "never",
        "simulate",
        "installment-invoice.mori",
        "--action",
        "collect",
        "--scenario",
        "invoice-first.scenario.json",
        "--logical-cap",
        "42"
      ],
      "execution": "invoice-first.execution.expected.json",
      "exit": 0,
      "expected_logical_used": "42",
      "logical_cap": "42",
      "name": "invoice-first-cap42",
      "program": "installment-invoice.program.expected.json",
      "scenario": "invoice-first.scenario.json",
      "source": "installment-invoice.mori",
      "stderr": "empty.stderr.expected.bin",
      "stdout": "invoice-first.cap42.stdout.json"
    },
    {
      "action": "collect",
      "argv_template": [
        "mori",
        "--color",
        "never",
        "simulate",
        "installment-invoice.mori",
        "--action",
        "collect",
        "--scenario",
        "invoice-final.scenario.json"
      ],
      "execution": "invoice-final.execution.expected.json",
      "exit": 0,
      "expected_logical_used": null,
      "logical_cap": null,
      "name": "invoice-final-ordinary",
      "program": "installment-invoice.program.expected.json",
      "scenario": "invoice-final.scenario.json",
      "source": "installment-invoice.mori",
      "stderr": "empty.stderr.expected.bin",
      "stdout": "invoice-final.ordinary.stdout.json"
    },
    {
      "action": "collect",
      "argv_template": [
        "mori",
        "--color",
        "never",
        "simulate",
        "installment-invoice.mori",
        "--action",
        "collect",
        "--scenario",
        "invoice-final.scenario.json",
        "--logical-cap",
        "42"
      ],
      "execution": "invoice-final.execution.expected.json",
      "exit": 0,
      "expected_logical_used": "42",
      "logical_cap": "42",
      "name": "invoice-final-cap42",
      "program": "installment-invoice.program.expected.json",
      "scenario": "invoice-final.scenario.json",
      "source": "installment-invoice.mori",
      "stderr": "empty.stderr.expected.bin",
      "stdout": "invoice-final.cap42.stdout.json"
    },
    {
      "action": "allocate",
      "argv_template": [
        "mori",
        "--color",
        "never",
        "simulate",
        "conditional-reserve.mori",
        "--action",
        "allocate",
        "--scenario",
        "reserve-approved.scenario.json"
      ],
      "execution": "reserve-approved.execution.expected.json",
      "exit": 0,
      "expected_logical_used": null,
      "logical_cap": null,
      "name": "reserve-approved-ordinary",
      "program": "conditional-reserve.program.expected.json",
      "scenario": "reserve-approved.scenario.json",
      "source": "conditional-reserve.mori",
      "stderr": "empty.stderr.expected.bin",
      "stdout": "reserve-approved.ordinary.stdout.json"
    },
    {
      "action": "allocate",
      "argv_template": [
        "mori",
        "--color",
        "never",
        "simulate",
        "conditional-reserve.mori",
        "--action",
        "allocate",
        "--scenario",
        "reserve-approved.scenario.json",
        "--logical-cap",
        "45"
      ],
      "execution": "reserve-approved.execution.expected.json",
      "exit": 0,
      "expected_logical_used": "45",
      "logical_cap": "45",
      "name": "reserve-approved-cap45",
      "program": "conditional-reserve.program.expected.json",
      "scenario": "reserve-approved.scenario.json",
      "source": "conditional-reserve.mori",
      "stderr": "empty.stderr.expected.bin",
      "stdout": "reserve-approved.cap45.stdout.json"
    }
  ],
  "invoice_link": {
    "equality": "exact complete Snapshot; replace only action inputs; not authenticated history",
    "first": "invoice-first.execution.expected.json#/post",
    "next": "invoice-final.execution.expected.json#/pre"
  },
  "status": "specified-only; actual consumers UNRUN"
}


===== COMPLETE PUBLIC FILE existing-result-review-packet/OBSERVATION.json =====
{
  "observedAt": "2026-10-18T08:51:12.833532+00:00",
  "candidateCommit": "7e0d753c40a3e5f694aa226bd657c831da111f65",
  "candidateTree": "55fab2c3cbd818ffb04eeb7d5930a28758a858b3",
  "pullRequest": 205,
  "actualResult": {
    "testFunctionsPassed": 2,
    "failed": 0,
    "ignored": 0,
    "filtered": 0,
    "wallSeconds": 52.624672563000786,
    "completedParserInputs": 2,
    "completedCLICalls": 6,
    "countQualification": "Counts of inner parser inputs/CLI calls inferred from passing full test controlflow; not separate per-call telemetry"
  },
  "scope": "Two complete Program serde values (32/33globalnodes),3uniquecompleteExecutions39steps/23effects compared ordinary+metered in6envelopes/exactstdout/stderr/exit0, work42/42/45 and actual ordinary/metered invoice snapshot links",
  "sourceCorrection": "Security source finding5 incorrectly said zero formatting diff. Actual RUSTFMT originalc298... -> reviewed76ee... only wraps the two assert_eq argument lines. Original entire review preserved; incorrect ancillary metadata excluded before execution. Both reviews otherwise agree on same actual76ee source/7e0candidate/exactcommand. Check this independently; no source/oracle/command changed as a correction.",
  "limits": [
    "Post-adoption public DTO/grammar/finite arena premises, not blinded or universal parser theorem",
    "No JSON raw object-key-order Program contract (Value equality); exact public CLIstdoutbytes separately asserted",
    "Local reference behavior only, no source-native/SDK/circuit/proof/authenticated history/ledger or parentexit result",
    "No syscalls/read-only or aggregate child memory/processgroup bound claimed; owned temporary root removed, allocation consumed"
  ],
  "requestedDecision": "Fresh actual-result/source/oracle audit and guarded merge only; root must verify remote candidate/main and merged tree."
}


===== COMPLETE PUBLIC FILE existing-result-review-packet/RUSTFMT.json =====
{
  "formatExit": 0,
  "checkExit": 0,
  "toolSha256": "a3ab6c445b26eda8d7bcfd5cc3f4b068dd7136af96822fd21378fc6fe8c2f2a3",
  "originalTestSha256": "c298f75d42df8e75c89da6cc1687f0433e2fa5d8ff6a5989b5be2775a6a47adf",
  "formattedTestSha256": "76ee071ade1e8a7edf815274205f58bf513c29373782a4c9bd798eb55e5caf89",
  "qualification": "Routine formatter only, exact owned config, no HOME; no Cargo/parser/runtime execution"
}


===== COMPLETE PUBLIC FILE existing-result-review-packet/FORMATTING-ORIGINAL-vs-REVIEWED.diff =====
--- author-original-c298
+++ reviewed-formatted-76ee
@@ -116,8 +116,7 @@
         ("invoice-first-cap42", "invoice-final-cap42"),
     ] {
         assert_eq!(
-            reports[first]["execution"]["post"],
-            reports[final_call]["execution"]["pre"],
+            reports[first]["execution"]["post"], reports[final_call]["execution"]["pre"],
             "{first} post must equal {final_call} pre in full"
         );
     }


===== COMPLETE PUBLIC FILE existing-result-review-packet/ADMISSION.json =====
{
  "decidedAt": "2026-10-18T08:48:35.707102+00:00",
  "sourceCommit": "7e0d753c40a3e5f694aa226bd657c831da111f65",
  "sourceTree": "55fab2c3cbd818ffb04eeb7d5930a28758a858b3",
  "packetManifestSha256": "28ae523a2687055965148bb230f981de9b214a4230a72ec32d8adbc94108becd",
  "votes": [
    "doctorwatson-financial-complete-source-ordinary-20261018-a1",
    "doctorwatson-financial-complete-source-security-fallback-20261018-a1"
  ],
  "attemptLimit": 1,
  "wallSeconds": 90,
  "killGraceSeconds": 5,
  "status": "CONSUMED attempt-1 starts next; allocation not reusable",
  "nativeAuthority": 0,
  "scope": "2testfunctions:2 full Program parse/serde comparisons +6 real simulate calls, completeJSON/rawbytes/exit0/empty stderr and actual ordinary/metered invoice linkage",
  "ancillaryCorrection": "One security source finding falsely says zero formatter diff; original c298... became reviewed76ee... by assert_eq line wrap only. Original reviews preserved and wrong metadata excluded; both exact candidate/command approvals otherwise agree. Fresh actual-result audits required.",
  "priorAllocations": "All prior failures/caps/counters/C4/k17/rosters preserved; distinct ordinary target only, no reset/reuse"
}


===== COMPLETE PUBLIC FILE existing-result-review-packet/attempt-1/START.json =====
{
  "startedAt": "2026-10-18T08:48:35.707675+00:00",
  "sourceCommit": "7e0d753c40a3e5f694aa226bd657c831da111f65",
  "sourceTree": "55fab2c3cbd818ffb04eeb7d5930a28758a858b3",
  "sourceStatus": "",
  "sourcePinsVerified": 388,
  "packetFilesVerified": 395,
  "toolPinsVerified": 4,
  "ancestorConfigsAbsent": 14,
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
    "core_financial_report_contract",
    "--",
    "--test-threads=1"
  ],
  "cwd": "/tmp/doctorwatson-financial-report-check-md25jyby/cwd",
  "ownedTemporaryRoot": "/tmp/doctorwatson-financial-report-check-md25jyby",
  "childEnvironment": {
    "PATH": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin",
    "RUSTC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc",
    "RUSTDOC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc",
    "CARGO_HOME": "/tmp/doctorwatson-financial-report-check-md25jyby/cargo-home",
    "CARGO_TARGET_DIR": "/tmp/doctorwatson-financial-report-check-md25jyby/target",
    "CARGO_NET_OFFLINE": "true",
    "CARGO_INCREMENTAL": "0",
    "CARGO_CACHE_AUTO_CLEAN_FREQUENCY": "never",
    "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_CONFIG_GLOBAL": "/dev/null",
    "GIT_CONFIG_SYSTEM": "/dev/null",
    "LANG": "C.UTF-8",
    "TMPDIR": "/tmp/doctorwatson-financial-report-check-md25jyby/tmp"
  },
  "publicCacheMappings": [
    {
      "path": "/tmp/doctorwatson-financial-report-check-md25jyby/cargo-home/registry",
      "target": "/home/charl/.cargo/registry"
    },
    {
      "path": "/tmp/doctorwatson-financial-report-check-md25jyby/cargo-home/git",
      "target": "/home/charl/.cargo/git"
    }
  ],
  "attempt": 1,
  "attemptLimit": 1,
  "nativeAuthority": 0,
  "scope": "2testfunctions:2 full Program parse/serde comparisons +6 real simulate calls, completeJSON/rawbytes/exit0/empty stderr and actual ordinary/metered invoice linkage"
}


===== COMPLETE PUBLIC FILE existing-result-review-packet/attempt-1/RESULT.json =====
{
  "endedAt": "2026-10-18T08:49:28.332405+00:00",
  "exitCode": 0,
  "wallSeconds": 52.624672563000786,
  "sourceCommitAfter": "7e0d753c40a3e5f694aa226bd657c831da111f65",
  "sourceTreeAfter": "55fab2c3cbd818ffb04eeb7d5930a28758a858b3",
  "sourceStatusAfter": "",
  "stdoutSha256": "c4ec1d6806c325fa2f306c0db31319a216989e4d3c1a34d6d4f9e26bda9f07ae",
  "stderrSha256": "03349f8b4be47c09e1941b5dcb5ddb3dbf4a5bdd991d3a4e1665b340771d3b70",
  "nativeAuthority": 0,
  "resultReview": "PENDING; no broadpass inferred"
}


===== COMPLETE PUBLIC FILE existing-result-review-packet/attempt-1/STDOUT.log =====

running 2 tests
test financial_cli_reports_match_complete_oracles_and_invoice_link ... ok
test financial_sources_match_complete_independent_programs ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s



===== COMPLETE PUBLIC FILE existing-result-review-packet/attempt-1/STDERR.log =====
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.26
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
   Compiling crypto-common v0.1.6
   Compiling block-buffer v0.10.4
   Compiling getrandom v0.2.17
   Compiling supports-unicode v3.0.0
   Compiling unicode-width v0.1.14
   Compiling supports-hyperlinks v3.2.0
   Compiling owo-colors v4.4.0
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
   Compiling crossbeam-deque v0.8.8
   Compiling base16ct v0.2.0
   Compiling serde_core v1.0.229
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
   Compiling block-buffer v0.12.1
   Compiling crypto-common v0.2.2
   Compiling mori-ast v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-ast)
   Compiling aho-corasick v1.1.5
   Compiling signature v2.2.0
   Compiling serde v1.0.229
   Compiling anstyle v1.0.14
   Compiling anstyle-query v1.1.5
   Compiling is_terminal_polyfill v1.70.2
   Compiling const-oid v0.10.2
   Compiling getrandom v0.4.3
   Compiling thiserror v2.0.21
   Compiling cpufeatures v0.2.17
   Compiling serde_json v1.0.151
   Compiling regex-syntax v0.8.11
   Compiling rayon-core v1.13.0
   Compiling colorchoice v1.0.5
   Compiling anstream v1.0.0
   Compiling regex-automata v0.4.18
   Compiling sha2 v0.10.9
   Compiling digest v0.11.3
   Compiling ecdsa v0.16.9
   Compiling bstr v1.13.1
   Compiling thiserror-impl v2.0.21
   Compiling serde_derive v1.0.229
   Compiling same-file v1.0.6
   Compiling strsim v0.11.1
   Compiling log v0.4.34
   Compiling cpufeatures v0.3.1
   Compiling heck v0.5.0
   Compiling clap_lex v1.1.1
   Compiling itoa v1.0.18
   Compiling similar v2.7.0
   Compiling clap_builder v4.6.7
   Compiling clap_derive v4.6.7
   Compiling sha2 v0.11.0
   Compiling globset v0.4.20
   Compiling walkdir v2.5.0
   Compiling k256 v0.13.4
   Compiling mori-parser v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-parser)
   Compiling fastrand v2.5.0
   Compiling either v1.19.0
   Compiling rayon v1.12.0
   Compiling tempfile v3.27.0
   Compiling mori-fmt v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-fmt)
   Compiling mori-core v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-core)
   Compiling ignore v0.4.33
   Compiling clap v4.6.7
   Compiling mori-checker v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-checker)
   Compiling console v0.16.6
   Compiling insta v1.49.0
   Compiling indoc v2.0.7
   Compiling mori v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 52.54s
     Running tests/core_financial_report_contract.rs (/tmp/doctorwatson-financial-report-check-md25jyby/target/debug/deps/core_financial_report_contract-fb973f1b9c9f1d2b)


===== COMPLETE PUBLIC FILE existing-result-review-packet/attempt-1/CLEANUP.json =====
{
  "completedAt": "2026-10-18T08:49:28.430751+00:00",
  "ownedTemporaryRoot": "/tmp/doctorwatson-financial-report-check-md25jyby",
  "existsAfter": false,
  "scope": "owned cwd/cargo-home/target/tmp and nested Project only"
}


===== COMPLETE PUBLIC FILE existing-result-review-packet/source/LOSSLESS-REVIEW-VIEW.json =====
{"qualification":"READ-ONLY REVIEW PRESENTATION, never a consumer oracle. Full exact original35candidatefiles and32payload freeze preserved. Unique complete JSON values are compacted losslessly for prompt size; six stdout wrappers refer to exact shared full execution values only in this view. Tests compare the original entire actual/expected JSON and raw stdout bytes WITHOUT projection or normalization.","completeExecutionValues":{"invoice-final.execution.expected.json":{"sourceSha256":"09455adaedb9915fb41c9c4df8c155a576875c8a71f6da7d24f3ed2c1e5ffba1","value":{"action":"collect","effects":[{"amount":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"from":"payer","kind":"Transfer","to":"seller"},{"amount":"3","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"from":"payer","kind":"Fee","to":"treasury"},{"after":"0","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"before":"80","creditor":"seller","debtor":"payer","kind":"Liability","name":"invoice"},{"after":{"type":"UInt","value":"127"},"before":{"type":"UInt","value":"47"},"kind":"State","name":"paid"},{"after":{"type":"UInt","value":"90"},"before":{"type":"UInt","value":"60"},"kind":"State","name":"dueDay"},{"after":{"type":"UInt","value":"2"},"before":{"type":"UInt","value":"1"},"kind":"State","name":"installments"},{"kind":"Emit","name":"collected","value":{"type":"UInt","value":"127"}}],"post":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"67","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"138","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"11","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"0","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"90"},"installments":{"type":"UInt","value":"2"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"127"}}},"pre":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"150","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"58","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"8","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"60"},"installments":{"type":"UInt","value":"1"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"47"}}},"semantics":"moriarty-core/1","steps":[{"effects":[],"index":0,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"150","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"58","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"8","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"60"},"installments":{"type":"UInt","value":"1"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"47"}}},"statement":{"kind":"Requires","predicate":4},"value":{"type":"Bool","value":true}},{"effects":[],"index":1,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"150","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"58","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"8","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"60"},"installments":{"type":"UInt","value":"1"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"47"}}},"statement":{"kind":"Requires","predicate":7},"value":{"type":"Bool","value":true}},{"effects":[],"index":2,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"150","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"58","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"8","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"60"},"installments":{"type":"UInt","value":"1"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"47"}}},"statement":{"kind":"Requires","predicate":12},"value":{"type":"Bool","value":true}},{"effects":[],"index":3,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"150","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"58","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"8","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"60"},"installments":{"type":"UInt","value":"1"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"47"}}},"statement":{"kind":"Let","name":"opening","value":13},"value":{"type":"UInt","value":"80"}},{"effects":[{"amount":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"from":"payer","kind":"Transfer","to":"seller"}],"index":4,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"70","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"138","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"8","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"60"},"installments":{"type":"UInt","value":"1"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"47"}}},"statement":{"amount":14,"asset":"CASH","from":"payer","kind":"Transfer","to":"seller"},"value":{"type":"UInt","value":"80"}},{"effects":[{"amount":"3","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"from":"payer","kind":"Fee","to":"treasury"}],"index":5,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"67","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"138","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"11","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"60"},"installments":{"type":"UInt","value":"1"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"47"}}},"statement":{"amount":15,"asset":"CASH","from":"payer","kind":"Fee","to":"treasury"},"value":{"type":"UInt","value":"3"}},{"effects":[{"after":"0","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"before":"80","creditor":"seller","debtor":"payer","kind":"Liability","name":"invoice"}],"index":6,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"67","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"138","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"11","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"0","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"60"},"installments":{"type":"UInt","value":"1"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"47"}}},"statement":{"amount":16,"kind":"Discharge","name":"invoice"},"value":{"type":"UInt","value":"80"}},{"effects":[{"after":{"type":"UInt","value":"127"},"before":{"type":"UInt","value":"47"},"kind":"State","name":"paid"}],"index":7,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"67","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"138","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"11","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"0","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"60"},"installments":{"type":"UInt","value":"1"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"127"}}},"statement":{"kind":"Next","state":"paid","value":19},"value":{"type":"UInt","value":"127"}},{"effects":[{"after":{"type":"UInt","value":"90"},"before":{"type":"UInt","value":"60"},"kind":"State","name":"dueDay"}],"index":8,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"67","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"138","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"11","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"0","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"90"},"installments":{"type":"UInt","value":"1"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"127"}}},"statement":{"kind":"Next","state":"dueDay","value":22},"value":{"type":"UInt","value":"90"}},{"effects":[{"after":{"type":"UInt","value":"2"},"before":{"type":"UInt","value":"1"},"kind":"State","name":"installments"}],"index":9,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"67","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"138","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"11","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"0","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"90"},"installments":{"type":"UInt","value":"2"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"127"}}},"statement":{"kind":"Next","state":"installments","value":25},"value":{"type":"UInt","value":"2"}},{"effects":[],"index":10,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"67","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"138","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"11","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"0","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"90"},"installments":{"type":"UInt","value":"2"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"127"}}},"statement":{"kind":"Ensures","predicate":30},"value":{"type":"Bool","value":true}},{"effects":[{"kind":"Emit","name":"collected","value":{"type":"UInt","value":"127"}}],"index":11,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"67","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"138","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"11","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"0","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"90"},"installments":{"type":"UInt","value":"2"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"127"}}},"statement":{"kind":"Emit","name":"collected","value":31},"value":{"type":"UInt","value":"127"}}]}},"invoice-first.execution.expected.json":{"sourceSha256":"816c6d3d4a00cfd65e3777b77d631c9aef75ed0b3e9bf13b18f6d33d29eebf2b","value":{"action":"collect","effects":[{"amount":"47","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"from":"payer","kind":"Transfer","to":"seller"},{"amount":"3","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"from":"payer","kind":"Fee","to":"treasury"},{"after":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"before":"127","creditor":"seller","debtor":"payer","kind":"Liability","name":"invoice"},{"after":{"type":"UInt","value":"47"},"before":{"type":"UInt","value":"0"},"kind":"State","name":"paid"},{"after":{"type":"UInt","value":"60"},"before":{"type":"UInt","value":"30"},"kind":"State","name":"dueDay"},{"after":{"type":"UInt","value":"1"},"before":{"type":"UInt","value":"0"},"kind":"State","name":"installments"},{"kind":"Emit","name":"collected","value":{"type":"UInt","value":"47"}}],"post":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"150","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"58","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"8","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"60"},"installments":{"type":"UInt","value":"1"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"47"}}},"pre":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"200","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"11","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"5","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"127","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"30"},"installments":{"type":"UInt","value":"0"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"0"}}},"semantics":"moriarty-core/1","steps":[{"effects":[],"index":0,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"200","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"11","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"5","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"127","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"30"},"installments":{"type":"UInt","value":"0"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"0"}}},"statement":{"kind":"Requires","predicate":4},"value":{"type":"Bool","value":true}},{"effects":[],"index":1,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"200","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"11","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"5","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"127","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"30"},"installments":{"type":"UInt","value":"0"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"0"}}},"statement":{"kind":"Requires","predicate":7},"value":{"type":"Bool","value":true}},{"effects":[],"index":2,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"200","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"11","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"5","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"127","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"30"},"installments":{"type":"UInt","value":"0"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"0"}}},"statement":{"kind":"Requires","predicate":12},"value":{"type":"Bool","value":true}},{"effects":[],"index":3,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"200","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"11","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"5","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"127","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"30"},"installments":{"type":"UInt","value":"0"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"0"}}},"statement":{"kind":"Let","name":"opening","value":13},"value":{"type":"UInt","value":"127"}},{"effects":[{"amount":"47","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"from":"payer","kind":"Transfer","to":"seller"}],"index":4,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"153","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"58","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"5","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"127","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"30"},"installments":{"type":"UInt","value":"0"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"0"}}},"statement":{"amount":14,"asset":"CASH","from":"payer","kind":"Transfer","to":"seller"},"value":{"type":"UInt","value":"47"}},{"effects":[{"amount":"3","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"from":"payer","kind":"Fee","to":"treasury"}],"index":5,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"150","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"58","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"8","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"127","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"30"},"installments":{"type":"UInt","value":"0"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"0"}}},"statement":{"amount":15,"asset":"CASH","from":"payer","kind":"Fee","to":"treasury"},"value":{"type":"UInt","value":"3"}},{"effects":[{"after":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"before":"127","creditor":"seller","debtor":"payer","kind":"Liability","name":"invoice"}],"index":6,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"150","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"58","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"8","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"30"},"installments":{"type":"UInt","value":"0"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"0"}}},"statement":{"amount":16,"kind":"Discharge","name":"invoice"},"value":{"type":"UInt","value":"47"}},{"effects":[{"after":{"type":"UInt","value":"47"},"before":{"type":"UInt","value":"0"},"kind":"State","name":"paid"}],"index":7,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"150","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"58","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"8","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"30"},"installments":{"type":"UInt","value":"0"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"47"}}},"statement":{"kind":"Next","state":"paid","value":19},"value":{"type":"UInt","value":"47"}},{"effects":[{"after":{"type":"UInt","value":"60"},"before":{"type":"UInt","value":"30"},"kind":"State","name":"dueDay"}],"index":8,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"150","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"58","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"8","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"60"},"installments":{"type":"UInt","value":"0"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"47"}}},"statement":{"kind":"Next","state":"dueDay","value":22},"value":{"type":"UInt","value":"60"}},{"effects":[{"after":{"type":"UInt","value":"1"},"before":{"type":"UInt","value":"0"},"kind":"State","name":"installments"}],"index":9,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"150","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"58","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"8","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"60"},"installments":{"type":"UInt","value":"1"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"47"}}},"statement":{"kind":"Next","state":"installments","value":25},"value":{"type":"UInt","value":"1"}},{"effects":[],"index":10,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"150","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"58","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"8","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"60"},"installments":{"type":"UInt","value":"1"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"47"}}},"statement":{"kind":"Ensures","predicate":30},"value":{"type":"Bool","value":true}},{"effects":[{"kind":"Emit","name":"collected","value":{"type":"UInt","value":"47"}}],"index":11,"snapshot":{"balances":[{"account":"observer","amount":"23","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"payer","amount":"150","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"19","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"seller","amount":"58","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"8","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"80","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"seller","debtor":"payer","name":"invoice"},{"amount":"9","asset":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"creditor":"auditor","debtor":"payer","name":"warranty"}],"state":{"dueDay":{"type":"UInt","value":"60"},"installments":{"type":"UInt","value":"1"},"marker":{"type":"UInt","value":"77"},"paid":{"type":"UInt","value":"47"}}},"statement":{"kind":"Emit","name":"collected","value":31},"value":{"type":"UInt","value":"47"}}]}},"reserve-approved.execution.expected.json":{"sourceSha256":"b703d683987bd2453223a33889d1c7db84df6f14fdd60f6b31a53dc2f7536a1b","value":{"action":"allocate","effects":[{"after":"50","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"before":"0","creditor":"supplier","debtor":"reserve","kind":"Liability","name":"allocation"},{"amount":"50","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"from":"reserve","kind":"Transfer","to":"supplier"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"from":"reserve","kind":"Fee","to":"treasury"},{"after":"0","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"before":"50","creditor":"supplier","debtor":"reserve","kind":"Liability","name":"allocation"},{"amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"from":"reserve","kind":"Transfer","to":"auditor"},{"after":"3","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"before":"10","creditor":"auditor","debtor":"reserve","kind":"Liability","name":"bond"},{"after":{"type":"UInt","value":"50"},"before":{"type":"UInt","value":"0"},"kind":"State","name":"allocated"},{"after":{"type":"Bool","value":true},"before":{"type":"Bool","value":false},"kind":"State","name":"closed"},{"kind":"Emit","name":"allocated","value":{"type":"UInt","value":"50"}}],"post":{"balances":[{"account":"auditor","amount":"9","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"reserve","amount":"13","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"11","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"5","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"supplier","amount":"55","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"3","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"0","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"supplier","debtor":"reserve","name":"allocation"},{"amount":"3","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"creditor":"auditor","debtor":"reserve","name":"bond"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"emergency","debtor":"reserve","name":"emergency"}],"state":{"allocated":{"type":"UInt","value":"50"},"closed":{"type":"Bool","value":true},"marker":{"type":"UInt","value":"77"}}},"pre":{"balances":[{"account":"auditor","amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"reserve","amount":"20","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"11","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"57","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"supplier","amount":"5","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"1","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"10","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"creditor":"auditor","debtor":"reserve","name":"bond"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"emergency","debtor":"reserve","name":"emergency"}],"state":{"allocated":{"type":"UInt","value":"0"},"closed":{"type":"Bool","value":false},"marker":{"type":"UInt","value":"77"}}},"semantics":"moriarty-core/1","steps":[{"effects":[],"index":0,"snapshot":{"balances":[{"account":"auditor","amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"reserve","amount":"20","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"11","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"57","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"supplier","amount":"5","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"1","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"10","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"creditor":"auditor","debtor":"reserve","name":"bond"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"emergency","debtor":"reserve","name":"emergency"}],"state":{"allocated":{"type":"UInt","value":"0"},"closed":{"type":"Bool","value":false},"marker":{"type":"UInt","value":"77"}}},"statement":{"kind":"Requires","predicate":6},"value":{"type":"Bool","value":true}},{"effects":[],"index":1,"snapshot":{"balances":[{"account":"auditor","amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"reserve","amount":"20","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"11","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"57","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"supplier","amount":"5","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"1","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"10","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"creditor":"auditor","debtor":"reserve","name":"bond"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"emergency","debtor":"reserve","name":"emergency"}],"state":{"allocated":{"type":"UInt","value":"0"},"closed":{"type":"Bool","value":false},"marker":{"type":"UInt","value":"77"}}},"statement":{"kind":"Requires","predicate":9},"value":{"type":"Bool","value":true}},{"effects":[],"index":2,"snapshot":{"balances":[{"account":"auditor","amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"reserve","amount":"20","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"11","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"57","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"supplier","amount":"5","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"1","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"10","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"creditor":"auditor","debtor":"reserve","name":"bond"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"emergency","debtor":"reserve","name":"emergency"}],"state":{"allocated":{"type":"UInt","value":"0"},"closed":{"type":"Bool","value":false},"marker":{"type":"UInt","value":"77"}}},"statement":{"kind":"Requires","predicate":12},"value":{"type":"Bool","value":true}},{"effects":[],"index":3,"snapshot":{"balances":[{"account":"auditor","amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"reserve","amount":"20","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"11","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"57","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"supplier","amount":"5","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"1","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"10","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"creditor":"auditor","debtor":"reserve","name":"bond"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"emergency","debtor":"reserve","name":"emergency"}],"state":{"allocated":{"type":"UInt","value":"0"},"closed":{"type":"Bool","value":false},"marker":{"type":"UInt","value":"77"}}},"statement":{"kind":"Let","name":"residual","value":17},"value":{"type":"UInt","value":"5"}},{"effects":[],"index":4,"snapshot":{"balances":[{"account":"auditor","amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"reserve","amount":"20","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"11","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"57","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"supplier","amount":"5","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"1","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"10","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"creditor":"auditor","debtor":"reserve","name":"bond"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"emergency","debtor":"reserve","name":"emergency"}],"state":{"allocated":{"type":"UInt","value":"0"},"closed":{"type":"Bool","value":false},"marker":{"type":"UInt","value":"77"}}},"statement":{"kind":"Requires","predicate":20},"value":{"type":"Bool","value":true}},{"effects":[{"after":"50","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"before":"0","creditor":"supplier","debtor":"reserve","kind":"Liability","name":"allocation"}],"index":5,"snapshot":{"balances":[{"account":"auditor","amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"reserve","amount":"20","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"11","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"57","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"supplier","amount":"5","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"1","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"50","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"supplier","debtor":"reserve","name":"allocation"},{"amount":"10","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"creditor":"auditor","debtor":"reserve","name":"bond"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"emergency","debtor":"reserve","name":"emergency"}],"state":{"allocated":{"type":"UInt","value":"0"},"closed":{"type":"Bool","value":false},"marker":{"type":"UInt","value":"77"}}},"statement":{"amount":21,"asset":"CASH","creditor":"supplier","debtor":"reserve","kind":"Liability","name":"allocation"},"value":{"type":"UInt","value":"50"}},{"effects":[{"amount":"50","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"from":"reserve","kind":"Transfer","to":"supplier"}],"index":6,"snapshot":{"balances":[{"account":"auditor","amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"reserve","amount":"20","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"11","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"supplier","amount":"55","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"1","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"50","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"supplier","debtor":"reserve","name":"allocation"},{"amount":"10","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"creditor":"auditor","debtor":"reserve","name":"bond"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"emergency","debtor":"reserve","name":"emergency"}],"state":{"allocated":{"type":"UInt","value":"0"},"closed":{"type":"Bool","value":false},"marker":{"type":"UInt","value":"77"}}},"statement":{"amount":22,"asset":"CASH","from":"reserve","kind":"Transfer","to":"supplier"},"value":{"type":"UInt","value":"50"}},{"effects":[{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"from":"reserve","kind":"Fee","to":"treasury"}],"index":7,"snapshot":{"balances":[{"account":"auditor","amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"reserve","amount":"20","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"11","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"5","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"supplier","amount":"55","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"3","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"50","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"supplier","debtor":"reserve","name":"allocation"},{"amount":"10","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"creditor":"auditor","debtor":"reserve","name":"bond"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"emergency","debtor":"reserve","name":"emergency"}],"state":{"allocated":{"type":"UInt","value":"0"},"closed":{"type":"Bool","value":false},"marker":{"type":"UInt","value":"77"}}},"statement":{"amount":23,"asset":"CASH","from":"reserve","kind":"Fee","to":"treasury"},"value":{"type":"UInt","value":"2"}},{"effects":[{"after":"0","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"before":"50","creditor":"supplier","debtor":"reserve","kind":"Liability","name":"allocation"}],"index":8,"snapshot":{"balances":[{"account":"auditor","amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"reserve","amount":"20","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"11","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"5","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"supplier","amount":"55","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"3","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"0","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"supplier","debtor":"reserve","name":"allocation"},{"amount":"10","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"creditor":"auditor","debtor":"reserve","name":"bond"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"emergency","debtor":"reserve","name":"emergency"}],"state":{"allocated":{"type":"UInt","value":"0"},"closed":{"type":"Bool","value":false},"marker":{"type":"UInt","value":"77"}}},"statement":{"amount":24,"kind":"Discharge","name":"allocation"},"value":{"type":"UInt","value":"50"}},{"effects":[{"amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"from":"reserve","kind":"Transfer","to":"auditor"}],"index":9,"snapshot":{"balances":[{"account":"auditor","amount":"9","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"reserve","amount":"13","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"11","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"5","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"supplier","amount":"55","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"3","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"0","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"supplier","debtor":"reserve","name":"allocation"},{"amount":"10","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"creditor":"auditor","debtor":"reserve","name":"bond"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"emergency","debtor":"reserve","name":"emergency"}],"state":{"allocated":{"type":"UInt","value":"0"},"closed":{"type":"Bool","value":false},"marker":{"type":"UInt","value":"77"}}},"statement":{"amount":25,"asset":"BOND","from":"reserve","kind":"Transfer","to":"auditor"},"value":{"type":"UInt","value":"7"}},{"effects":[{"after":"3","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"before":"10","creditor":"auditor","debtor":"reserve","kind":"Liability","name":"bond"}],"index":10,"snapshot":{"balances":[{"account":"auditor","amount":"9","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"reserve","amount":"13","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"11","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"5","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"supplier","amount":"55","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"3","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"0","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"supplier","debtor":"reserve","name":"allocation"},{"amount":"3","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"creditor":"auditor","debtor":"reserve","name":"bond"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"emergency","debtor":"reserve","name":"emergency"}],"state":{"allocated":{"type":"UInt","value":"0"},"closed":{"type":"Bool","value":false},"marker":{"type":"UInt","value":"77"}}},"statement":{"amount":26,"kind":"Discharge","name":"bond"},"value":{"type":"UInt","value":"7"}},{"effects":[{"after":{"type":"UInt","value":"50"},"before":{"type":"UInt","value":"0"},"kind":"State","name":"allocated"}],"index":11,"snapshot":{"balances":[{"account":"auditor","amount":"9","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"reserve","amount":"13","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"11","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"5","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"supplier","amount":"55","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"3","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"0","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"supplier","debtor":"reserve","name":"allocation"},{"amount":"3","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"creditor":"auditor","debtor":"reserve","name":"bond"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"emergency","debtor":"reserve","name":"emergency"}],"state":{"allocated":{"type":"UInt","value":"50"},"closed":{"type":"Bool","value":false},"marker":{"type":"UInt","value":"77"}}},"statement":{"kind":"Next","state":"allocated","value":27},"value":{"type":"UInt","value":"50"}},{"effects":[{"after":{"type":"Bool","value":true},"before":{"type":"Bool","value":false},"kind":"State","name":"closed"}],"index":12,"snapshot":{"balances":[{"account":"auditor","amount":"9","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"reserve","amount":"13","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"11","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"5","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"supplier","amount":"55","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"3","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"0","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"supplier","debtor":"reserve","name":"allocation"},{"amount":"3","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"creditor":"auditor","debtor":"reserve","name":"bond"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"emergency","debtor":"reserve","name":"emergency"}],"state":{"allocated":{"type":"UInt","value":"50"},"closed":{"type":"Bool","value":true},"marker":{"type":"UInt","value":"77"}}},"statement":{"kind":"Next","state":"closed","value":28},"value":{"type":"Bool","value":true}},{"effects":[],"index":13,"snapshot":{"balances":[{"account":"auditor","amount":"9","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"reserve","amount":"13","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"11","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"5","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"supplier","amount":"55","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"3","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"0","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"supplier","debtor":"reserve","name":"allocation"},{"amount":"3","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"creditor":"auditor","debtor":"reserve","name":"bond"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"emergency","debtor":"reserve","name":"emergency"}],"state":{"allocated":{"type":"UInt","value":"50"},"closed":{"type":"Bool","value":true},"marker":{"type":"UInt","value":"77"}}},"statement":{"kind":"Ensures","predicate":31},"value":{"type":"Bool","value":true}},{"effects":[{"kind":"Emit","name":"allocated","value":{"type":"UInt","value":"50"}}],"index":14,"snapshot":{"balances":[{"account":"auditor","amount":"9","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"7","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"reserve","amount":"13","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0}},{"account":"observer","amount":"11","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"reserve","amount":"5","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"supplier","amount":"55","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}},{"account":"treasury","amount":"3","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2}}],"liabilities":[{"amount":"0","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"supplier","debtor":"reserve","name":"allocation"},{"amount":"3","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"creditor":"auditor","debtor":"reserve","name":"bond"},{"amount":"2","asset":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"creditor":"emergency","debtor":"reserve","name":"emergency"}],"state":{"allocated":{"type":"UInt","value":"50"},"closed":{"type":"Bool","value":true},"marker":{"type":"UInt","value":"77"}}},"statement":{"kind":"Emit","name":"allocated","value":32},"value":{"type":"UInt","value":"50"}}]}}},"completeProgramValues":{"conditional-reserve.program.expected.json":{"sourceSha256":"b7f3deaad79698f8a447190373c6886965f606a7b7f33ae79c030d8b3d93b525","value":{"actions":[{"inputs":[],"name":"register","statements":[{"amount":0,"asset":"CASH","creditor":"supplier","debtor":"reserve","kind":"Liability","name":"allocation"},{"amount":1,"asset":"BOND","creditor":"auditor","debtor":"reserve","kind":"Liability","name":"bond"},{"amount":2,"asset":"CASH","creditor":"emergency","debtor":"reserve","kind":"Liability","name":"emergency"}]},{"inputs":[{"name":"approved","ty":"Bool"},{"name":"amount","ty":"UInt"},{"name":"bondAmount","ty":"UInt"}],"name":"allocate","statements":[{"kind":"Requires","predicate":6},{"kind":"Requires","predicate":9},{"kind":"Requires","predicate":12},{"kind":"Let","name":"residual","value":17},{"kind":"Requires","predicate":20},{"amount":21,"asset":"CASH","creditor":"supplier","debtor":"reserve","kind":"Liability","name":"allocation"},{"amount":22,"asset":"CASH","from":"reserve","kind":"Transfer","to":"supplier"},{"amount":23,"asset":"CASH","from":"reserve","kind":"Fee","to":"treasury"},{"amount":24,"kind":"Discharge","name":"allocation"},{"amount":25,"asset":"BOND","from":"reserve","kind":"Transfer","to":"auditor"},{"amount":26,"kind":"Discharge","name":"bond"},{"kind":"Next","state":"allocated","value":27},{"kind":"Next","state":"closed","value":28},{"kind":"Ensures","predicate":31},{"kind":"Emit","name":"allocated","value":32}]}],"assets":[{"identity":{"issuer":"financial-pair-reserve","network":"local","reference":"CASH","scale":2},"name":"CASH"},{"identity":{"issuer":"financial-pair-reserve","network":"local","reference":"BOND","scale":0},"name":"BOND"}],"expressions":[{"kind":"Literal","value":{"type":"UInt","value":"0"}},{"kind":"Literal","value":{"type":"UInt","value":"10"}},{"kind":"Literal","value":{"type":"UInt","value":"2"}},{"kind":"Name","name":"approved"},{"kind":"Name","name":"closed"},{"kind":"Not","value":4},{"kind":"Binary","lhs":3,"op":"And","rhs":5},{"kind":"Name","name":"amount"},{"kind":"Literal","value":{"type":"UInt","value":"0"}},{"kind":"Binary","lhs":7,"op":"Gt","rhs":8},{"kind":"Name","name":"amount"},{"kind":"Literal","value":{"type":"UInt","value":"50"}},{"kind":"Binary","lhs":10,"op":"Le","rhs":11},{"kind":"Literal","value":{"type":"UInt","value":"57"}},{"kind":"Name","name":"amount"},{"kind":"Binary","lhs":13,"op":"Sub","rhs":14},{"kind":"Literal","value":{"type":"UInt","value":"2"}},{"kind":"Binary","lhs":15,"op":"Sub","rhs":16},{"kind":"Name","name":"residual"},{"kind":"Literal","value":{"type":"UInt","value":"5"}},{"kind":"Binary","lhs":18,"op":"Ge","rhs":19},{"kind":"Name","name":"amount"},{"kind":"Name","name":"amount"},{"kind":"Literal","value":{"type":"UInt","value":"2"}},{"kind":"Name","name":"amount"},{"kind":"Name","name":"bondAmount"},{"kind":"Name","name":"bondAmount"},{"kind":"Name","name":"amount"},{"kind":"Literal","value":{"type":"Bool","value":true}},{"kind":"LiabilityAmount","name":"allocation"},{"kind":"Literal","value":{"type":"UInt","value":"0"}},{"kind":"Binary","lhs":29,"op":"Eq","rhs":30},{"kind":"Name","name":"allocated"}],"language":"moriarty/1","name":"ConditionalReserve","semantics":"moriarty-core/1","states":[{"initial":{"type":"UInt","value":"0"},"name":"allocated","ty":"UInt"},{"initial":{"type":"Bool","value":false},"name":"closed","ty":"Bool"},{"initial":{"type":"UInt","value":"77"},"name":"marker","ty":"UInt"}]}},"installment-invoice.program.expected.json":{"sourceSha256":"bb6c81d19c1113e9fca01fc81e9245f964336be3dec192f704690c1b3a0ed1c3","value":{"actions":[{"inputs":[],"name":"register","statements":[{"amount":0,"asset":"CASH","creditor":"seller","debtor":"payer","kind":"Liability","name":"invoice"},{"amount":1,"asset":"CASH","creditor":"auditor","debtor":"payer","kind":"Liability","name":"warranty"}]},{"inputs":[{"name":"amount","ty":"UInt"},{"name":"atDay","ty":"UInt"}],"name":"collect","statements":[{"kind":"Requires","predicate":4},{"kind":"Requires","predicate":7},{"kind":"Requires","predicate":12},{"kind":"Let","name":"opening","value":13},{"amount":14,"asset":"CASH","from":"payer","kind":"Transfer","to":"seller"},{"amount":15,"asset":"CASH","from":"payer","kind":"Fee","to":"treasury"},{"amount":16,"kind":"Discharge","name":"invoice"},{"kind":"Next","state":"paid","value":19},{"kind":"Next","state":"dueDay","value":22},{"kind":"Next","state":"installments","value":25},{"kind":"Ensures","predicate":30},{"kind":"Emit","name":"collected","value":31}]}],"assets":[{"identity":{"issuer":"financial-pair-invoice","network":"local","reference":"CASH","scale":2},"name":"CASH"}],"expressions":[{"kind":"Literal","value":{"type":"UInt","value":"127"}},{"kind":"Literal","value":{"type":"UInt","value":"9"}},{"kind":"Name","name":"amount"},{"kind":"Literal","value":{"type":"UInt","value":"0"}},{"kind":"Binary","lhs":2,"op":"Gt","rhs":3},{"kind":"Name","name":"atDay"},{"kind":"Name","name":"dueDay"},{"kind":"Binary","lhs":5,"op":"Ge","rhs":6},{"kind":"LiabilityPresent","name":"invoice"},{"kind":"LiabilityAmount","name":"invoice"},{"kind":"Name","name":"amount"},{"kind":"Binary","lhs":9,"op":"Ge","rhs":10},{"kind":"Binary","lhs":8,"op":"And","rhs":11},{"kind":"LiabilityAmount","name":"invoice"},{"kind":"Name","name":"amount"},{"kind":"Literal","value":{"type":"UInt","value":"3"}},{"kind":"Name","name":"amount"},{"kind":"Name","name":"paid"},{"kind":"Name","name":"amount"},{"kind":"Binary","lhs":17,"op":"Add","rhs":18},{"kind":"Name","name":"dueDay"},{"kind":"Literal","value":{"type":"UInt","value":"30"}},{"kind":"Binary","lhs":20,"op":"Add","rhs":21},{"kind":"Name","name":"installments"},{"kind":"Literal","value":{"type":"UInt","value":"1"}},{"kind":"Binary","lhs":23,"op":"Add","rhs":24},{"kind":"LiabilityAmount","name":"invoice"},{"kind":"Name","name":"opening"},{"kind":"Name","name":"amount"},{"kind":"Binary","lhs":27,"op":"Sub","rhs":28},{"kind":"Binary","lhs":26,"op":"Eq","rhs":29},{"kind":"Name","name":"paid"}],"language":"moriarty/1","name":"InstallmentInvoice","semantics":"moriarty-core/1","states":[{"initial":{"type":"UInt","value":"0"},"name":"paid","ty":"UInt"},{"initial":{"type":"UInt","value":"30"},"name":"dueDay","ty":"UInt"},{"initial":{"type":"UInt","value":"0"},"name":"installments","ty":"UInt"},{"initial":{"type":"UInt","value":"77"},"name":"marker","ty":"UInt"}]}}},"stdoutWrappers":{"invoice-first.ordinary.stdout.json":{"sourceSha256":"633d834cebb16b1eb53494b67e43e4c86b037a2e10d66c5be32262c835ba51b0","sourceBytes":43761,"exactTopKeys":["execution","qualification","status"],"executionReferenceForViewOnly":"invoice-first.execution.expected.json","allOtherMembers":{"qualification":"local-reference-execution","status":"accepted"}},"invoice-first.cap42.stdout.json":{"sourceSha256":"002ba2db5fb66e83f9aea462350c60211dcae9da3fc394c253667ad2b3f04b00","sourceBytes":43865,"exactTopKeys":["execution","logical","profile","qualification","status"],"executionReferenceForViewOnly":"invoice-first.execution.expected.json","allOtherMembers":{"logical":{"cap":"42","used":"42"},"profile":"moriarty-reference-logical-cost/1","qualification":"local-reference-execution","status":"accepted"}},"invoice-final.ordinary.stdout.json":{"sourceSha256":"6b086a275a58d2b379a3cf10e11fed627a2aa76c836caa608be5050cfce52b97","sourceBytes":43772,"exactTopKeys":["execution","qualification","status"],"executionReferenceForViewOnly":"invoice-final.execution.expected.json","allOtherMembers":{"qualification":"local-reference-execution","status":"accepted"}},"invoice-final.cap42.stdout.json":{"sourceSha256":"f7ffcd76dce6a36800f59b46c420f4abc7af043cc212d882e5704be606944868","sourceBytes":43876,"exactTopKeys":["execution","logical","profile","qualification","status"],"executionReferenceForViewOnly":"invoice-final.execution.expected.json","allOtherMembers":{"logical":{"cap":"42","used":"42"},"profile":"moriarty-reference-logical-cost/1","qualification":"local-reference-execution","status":"accepted"}},"reserve-approved.ordinary.stdout.json":{"sourceSha256":"57b83831652c8c74e6559e5390df08462ea3cf0dfa6fe7dc417794d855aeecce","sourceBytes":66326,"exactTopKeys":["execution","qualification","status"],"executionReferenceForViewOnly":"reserve-approved.execution.expected.json","allOtherMembers":{"qualification":"local-reference-execution","status":"accepted"}},"reserve-approved.cap45.stdout.json":{"sourceSha256":"4673490151308b065ed76e6f63555052772a026a3d032c3fe7eadb479936efe2","sourceBytes":66430,"exactTopKeys":["execution","logical","profile","qualification","status"],"executionReferenceForViewOnly":"reserve-approved.execution.expected.json","allOtherMembers":{"logical":{"cap":"45","used":"45"},"profile":"moriarty-reference-logical-cost/1","qualification":"local-reference-execution","status":"accepted"}}}}

