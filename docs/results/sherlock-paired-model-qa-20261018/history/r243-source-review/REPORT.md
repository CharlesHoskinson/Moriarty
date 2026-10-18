```json
{
  "reviewer": {
    "role": "fresh-nonauthor-designated-ordinary-semantic-source-reviewer",
    "sourceAuthorExcluded": "Sol is source author and not an independent reviewer",
    "requestedModelIdentity": "gemini-4-argon-high (HIGH thinking effort, standing fallback for observed Opus 5.5 quota outage; no availability probe/retry/resume)",
    "returnedModelIdentity": "Gemini 4 Argon (High)",
    "highLimitsAndDiscipline": {
      "thinkingEffort": "HIGH",
      "toolsOrShellInvoked": false,
      "editsOrImportsOrTestsRun": false,
      "compilerModelSdkNativeNetworkPrivateActions": false,
      "decisionAuthority": "Independent review judgment only; Root alone executes or adopts"
    }
  },
  "coverage": {
    "fullBodiesReviewedInline": [
      {
        "id": "BODY1",
        "path": "models/sherlock/qa/check.py",
        "bytes": 81758,
        "role": "Candidate QA checker with explicit paired K/Quint supplied-run comparison and PAIRED_PROJECTION_CONTRACT"
      },
      {
        "id": "BODY2",
        "path": "models/sherlock/qa/test_paired_projection.py",
        "bytes": 10170,
        "role": "10 ordinary supplied-data unittest regressions for paired projection and F03 driver text inspection"
      },
      {
        "id": "BODY3",
        "path": "models/sherlock/quint/observations/f03ListsumObservationTest.qnt",
        "bytes": 1189,
        "role": "Literal F03 recursive list-sum input driver for unchanged kProfileMachine"
      },
      {
        "id": "BODY4",
        "path": "models/sherlock/qa/host_observations.py",
        "bytes": 4912,
        "role": "Unchanged 11-field literal snapshot overlay and observation comparator"
      },
      {
        "id": "BODY5",
        "path": "models/sherlock/qa/source_reference_facts.py",
        "bytes": 1484,
        "role": "Unchanged explicit input byte-read inventory helper"
      },
      {
        "id": "BODY6",
        "path": "models/sherlock/quint/kProfileMachine.qnt",
        "bytes": 18880,
        "role": "Unchanged generic frozen K-A Quint state machine"
      },
      {
        "id": "BODY7",
        "path": "models/sherlock/k/fixtures/uiListsum123.ui",
        "bytes": 422,
        "role": "Original K recursive list-sum fixture (historical fuel 1000 vs selected exact completion fuel 93)"
      },
      {
        "id": "BODY8",
        "path": "241-paired-model-ordinary-unit-qa/PROPOSED.json",
        "bytes": 6830,
        "role": "Prospective 30s 10-test ordinary Python unittest execution plan over 13 local snapshot files"
      }
    ],
    "omissionsAndNamedSourceAvailableGaps": [
      "models/sherlock/reference/check-reference.py (13260 bytes, pinned SHA256 verified before compile/exec inside check_rust_transcription during qa.run(); not supplied inline for fresh full-body semantic re-audit)",
      "models/sherlock/k/main.k (16249 bytes, pinned SHA256 read/checked as bytes only; not supplied inline for fresh full-body re-audit)",
      "Frozen corpus JSON and Rust transcription files in the 13-file snapshot (normal-programs.json, normal-prefixes.json, extra-normal-programs.json, extra-normal-prefixes.json, normal_oracles.rs, extra_normal_oracles.rs; metadata/digest-pinned data, not re-audited inline)",
      "Independent original F03 oracle fixtures under docs/audits/2026-10-16/sherlock-full-original-fixtures/frozen/fixture-oracle-data-01 (fixture-normal-programs.json and fixture-normal-prefixes.json; SHA256-pinned in F03_ORACLE_PINS, not supplied inline and not in the 13-file unit-test snapshot)",
      "Underlying Python/OS runtime environment and existing 254 unit tests (not selected in this prospective 10-test run); scope-specific ABSTAIN on unprovided whole bodies rather than inventing a native or full-corpus gate"
    ]
  },
  "sourceVerdict": {
    "verdict": "PASS_SOURCE_REVIEW_WITH_RETENTION_GAPS",
    "summary": "The explicit paired K/Quint supplied-run comparator in check.py, the 10 ordinary tests in test_paired_projection.py, and the literal F03 driver in f03ListsumObservationTest.qnt are semantically sound within their declared non-authenticating, finite supplied-trace comparator scope.",
    "verifiedProperties": {
      "defaultCorpusPreservation": "check.py:run() unconditionally validates main.k, all 8 default programs (N01-N08), 208 state rows, 200 charged edges, 8 publications, 216 prefix rows, and both Rust literal transcriptions before entering paired_mode.",
      "elevenStateFields": "host_observations.py:literal_snapshot() constructs all 11 fields (code_k, entry_k, control_k, environment_k, frames_top_first_k, depth, stack_limit, remaining_fuel, initial_fuel, output_k, charged_steps); first_difference() enforces exact type, key, and array-order equality across initial, ordered records[].before/after, and terminal snapshots.",
      "ruleChargeAndTerminalEnabledness": "Each record compares rule_start_line and charge (1 for ordinary steps, 0 for terminal publication rule 80/82/84), plus run-level terminal_has_next and any undeclared extra fields.",
      "independentOraclePlusPairComparison": "compare_paired_observations() compares K against the independent oracle and Quint against the independent oracle via compare_supplied_observations(), and separately compares K directly against Quint (excluding only top-level producer). Overall status is mismatch if either per-layer oracle check or the direct paired check fails.",
      "selectionMissingDuplicateAndOmissions": "_selected_observations() enforces exact integer fuel (type(row['initial'].get('initial_fuel')) is int, rejecting bool/float), raises ValueError if selected_programs != 1 or selected_runs != 1 (rejecting both missing and duplicate entries), preserves envelope keys, and reports exact omitted_programs and omitted_runs counts.",
      "provenanceAndNonAuthentication": "Per-layer summaries report producer_authenticity='not-established' and execution_receipt='not-supplied', retaining projection, evidence, and raw_context diagnostics without treating them as execution proofs.",
      "numericBoundsAndContract": "PAIRED_PROJECTION_CONTRACT and compare_paired_observations() report UInt128 bounds (0..340282366920938463463374607431768211455), initial_fuel, stack_limit, observed_stack_limit, observed_logical_frame_peak, 9 fieldMaps covering all 11 state fields + records + terminal_has_next, ruleToAction, 4 hiddenSteps items (including unmeasured K helper visits/work 'not0' and raw post-publication rule0/charge0 Quint stutters), and unmatchedFinanceWait.",
      "f03DriverAndMachineAlignment": "f03ListsumObservationTest.qnt supplies only a literal Program table (function name 3, param 0, PCons head 1 / tail 2, recursive Call({name: 3, arguments: List(9)}) at ExprId 8, input list [1,2,3], entry 0) to unchanged kProfileMachine::initProgram(f03Program, 93, 64).then(94.reps(i => step)). Tracing against kProfileMachine.qnt confirms 93 charged transitions + 1 free publication (rule 80, charge 0) = 94 steps, result UInt(6), and logical frame peak 11 (at MatchFrame/ScopeFrame inside the base-case nil call), with no .expect() or helper definitions.",
      "tenTestsDiscrimination": "All 10 unittest methods in test_paired_projection.py accurately discriminate full match, intermediate saved environment divergence with equal terminal state, agreed wrong rule_start_line against independent oracle, boundary fields (charge, rule_start_line, terminal_has_next, extra_state), program identity precedence (/programs/0/code_k), omitted row counts and unauthenticated provenance retention, missing selected run ValueError, contract completeness, preservation of default 8/216 corpus counts in qa.run(), and F03 literal driver constraints."
    }
  },
  "findings": {
    "mandatoryBlocking": [],
    "nonBlocking": [
      {
        "id": "NB-01",
        "severity": "non-blocking",
        "file": "models/sherlock/qa/check.py",
        "anchor": "for layer, artifact in artifacts.items():\n        observed, _ = _selected_observations(\n            artifact['observations'], case_id=case_id, initial_fuel=initial_fuel)\n        selected[layer] = observed\n        difference, summary = compare_supplied_observations(artifact, families, layer=layer,\n            selection=dict(case_id=case_id, initial_fuel=initial_fuel))",
        "detail": "In compare_paired_observations(), artifact['observations'] is indexed directly before compare_supplied_observations() validates top-level artifact keys via fields(artifact, ('format', 'observations', 'projection') + extra, ...). A malformed artifact dict missing 'observations' raises KeyError('observations') rather than the descriptive ValueError('<layer> artifact: missing observations'), though main() catches both."
      },
      {
        "id": "NB-02",
        "severity": "non-blocking",
        "file": "models/sherlock/qa/check.py",
        "anchor": "paired_difference = adapter.compare_observations(left, right, first_difference)\n    if paired_difference is not None:\n        paired_difference['raw_context'] = {layer: layer_raw_context(artifact, paired_difference)\n                                            for layer, artifact in artifacts.items()}",
        "detail": "When K and Quint differ at /programs/0/... rather than /runs/0/..., adapter.compare_observations() returns paired_difference without case_id or initial_fuel, so paired_difference['raw_context'] resolves to raw_context_status='unavailable' (whereas layers[layer]['raw_context'] remains populated via the explicit context_key)."
      },
      {
        "id": "NB-03",
        "severity": "non-blocking",
        "file": "models/sherlock/qa/check.py",
        "anchor": "if selected_case == 'F03':\n            fixture_documents = []\n            for name, pin in F03_ORACLE_PINS.items():\n                path = repo / F03_ORACLE_ROOT / name",
        "detail": "The selected_case == 'F03' file-loading path in run() depends on F03_ORACLE_ROOT (docs/audits/2026-10-16/sherlock-full-original-fixtures/frozen/fixture-oracle-data-01), which is outside the 13-file unit-test snapshot in PROPOSED.json and therefore not exercised by test_paired_projection.py (which tests qa.run() with selected_case='N01')."
      }
    ]
  },
  "unitTestClassification": {
    "targetPlan": "241-paired-model-ordinary-unit-qa/PROPOSED.json",
    "eligibility": "ELIGIBLE",
    "classification": "ORDINARY_NONNATIVE_PYTHON_UNITTEST_ONLY",
    "command": [
      "/usr/bin/timeout",
      "--signal=KILL",
      "30s",
      "/usr/bin/python3",
      "-B",
      "-m",
      "unittest",
      "-v",
      "test_paired_projection"
    ],
    "conditions": {
      "wallSeconds": 30,
      "perStreamKeptBytes": 131072,
      "capturePolicy": "Root subprocess/selectors capture with process-group KILL on overcap, fail-stop, no retry",
      "environment": {
        "PATH": "/usr/bin:/bin",
        "LANG": "C.UTF-8",
        "TMPDIR": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/241-paired-model-ordinary-unit-qa/tmp"
      },
      "prohibitedOperationsVerifiedAbsent": "No K, Quint, Lean, Rust, Cargo, rustc, SDK, host probe, network, or model/interpreter CLI execution; check-reference.py is compiled/imported in-process only for serialize_oracles() after SHA256 verification; f03ListsumObservationTest.qnt is read as plain text only; nativeAuthority=0."
    },
    "gaps": [
      "Only the 10 new tests in test_paired_projection.py are selected; the existing 254 tests are not selected.",
      "Tests operate on synthetic N01 comparator fixtures and default frozen N01-N08 corpus files; they do not execute K or Quint or load the external F03_ORACLE_PINS files.",
      "check-reference.py (13KB), main.k (16KB), the 6 frozen corpus/Rust oracle files, and the Python/OS runtime are trusted local inputs (named source-available gaps, not fresh full-body semantic proof)."
    ]
  },
  "original021MissingActualCaptures": {
    "kF03CaptureStatus": "UNOBSERVED (historical uiListsum123.ui uses fuel 1000; no actual K trace captured at selected fuel 93)",
    "quintF03CaptureStatus": "UNOBSERVED (f03ListsumObservationTest.qnt is SOURCE_ONLY / UNRUN; no actual Quint ITF trace captured at fuel 93)",
    "uireq021Status": "OPEN / UNCLOSED (adapter comparison logic, projection contract, and literal F03 driver are source-reviewed, but actual paired F03 K and Quint model execution receipts and comparison outputs do not yet exist)"
  },
  "retainedNoScopeProofClaim": {
    "claim": "NONE",
    "details": "Supplied-data comparator validation and finite trace agreement do not establish model execution authenticity, whole-grammar or numerical-domain semantic refinement, formal proof, B-level helper/financial/wait correspondence, or Rust host/native/target qualification."
  },
  "executionOccurred": false
}
```

