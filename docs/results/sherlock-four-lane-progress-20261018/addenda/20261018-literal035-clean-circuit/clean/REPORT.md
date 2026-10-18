{
  "schema": "moriarty.sherlock.clean_b06.final_source_actual_result_merge_review.v1",
  "review_id": "CLEAN-B06-FINAL-SOURCE-ACTUAL-RESULT-MERGE-REVIEW-20261018",
  "reviewer_identity": {
    "role": "independent_non_author_reviewer",
    "session_continuation": "same_reviewer_continuation_in_original_cwd",
    "conversation_id": "162f2a2e-167e-4c73-a37f-25e93b112428",
    "cwd": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/formal-three-profile-fresh-review-20261018/clean-batch01/revision02/fresh-provider-cwd",
    "requested_and_returned_model": "gemini-4-argon",
    "effort": "HIGH",
    "routing_context": "standing_observed_opus_unavailability_until_2026-10-19_fallback",
    "authored_b01_b02_b03_b05_b06": false,
    "tools_executed_in_this_turn": 0
  },
  "final_source_disposition": "APPROVE",
  "actual_result_and_scope_disposition": "APPROVE",
  "fork_merge_scope_disposition": "APPROVE",
  "approved_fork_merge_scope": {
    "target_branch_scope": "Authorized Fenrir Clean fork F-branch (R-only staging/merge/push by Root alone)",
    "disposition": "APPROVE",
    "exact_four_final_theorem_sources": [
      {
        "module": "Clean.Sherlock.NominalObservationTheorems",
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-clean-20261018/batch06-operational-correction02/afterimage/clean/fork/Clean/Sherlock/NominalObservationTheorems.lean",
        "bytes": 7166,
        "sha256": "8847bfeae3c79830b620b5eac3d3b56c00c9c045f38c19f4aaceb1354aae3fef",
        "theorem_count": 11,
        "disposition": "APPROVE"
      },
      {
        "module": "Clean.Sherlock.NominalCallerTheorems",
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-clean-20261018/batch06-operational-correction04/afterimage/clean/fork/Clean/Sherlock/NominalCallerTheorems.lean",
        "bytes": 11239,
        "sha256": "30528ea6b5da3a17e1e68894e87363436c9ea0edd7c0e92c2815ff2a80906612",
        "theorem_count": 13,
        "disposition": "APPROVE"
      },
      {
        "module": "Clean.Sherlock.NominalArmPriorityTheorems",
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-clean-20261018/batch06-operational-correction05/afterimage/clean/fork/Clean/Sherlock/NominalArmPriorityTheorems.lean",
        "bytes": 8128,
        "sha256": "85cda0a6b78097597ab34668d121be2e05328c519c4051d22fd273b6837db600",
        "theorem_count": 7,
        "disposition": "APPROVE"
      },
      {
        "module": "Clean.Sherlock.NominalConstructorCorrespondence",
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-clean-20261018/batch06-constructor-correspondence01/afterimage/clean/fork/Clean/Sherlock/NominalConstructorCorrespondence.lean",
        "bytes": 10378,
        "sha256": "4ffa6e8ee219f8e7153a00d9cd8b4882152d201e5cc75f7d5905004a5f4b5022",
        "theorem_count": 7,
        "disposition": "APPROVE"
      }
    ],
    "exact_two_qualified_publication_documents": [
      {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-clean-20261018/batch06-final-source-result-review01/proposed-publication-docs02/sherlock-nominal-proof-receipts-20261018.json",
        "bytes": 5059,
        "sha256": "46b774b6105afe611cbe42ddd18cbd9abaf42dd7b713a997a43dc00318df1559",
        "disposition": "APPROVE"
      },
      {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-clean-20261018/batch06-final-source-result-review01/proposed-publication-docs02/sherlock-nominal-proof-scope-20261018.md",
        "bytes": 2878,
        "sha256": "716db3725a83cd763ef99171f365943775456e1184f8f449c8dbff9750ff6fbe",
        "disposition": "APPROVE"
      }
    ],
    "excluded_from_merge_scope": [
      "Generated .olean, .olean.private, .olean.server, .ir, and .ir.sig compiler artifacts",
      "Private expected/oracle data and comparison artifacts",
      "Provider review transcripts and runtime scratch directories",
      "Untracked ReferenceJson / Driver files"
    ]
  },
  "verified_actual_kernel_results": {
    "toolchain": "leanprover/lean4:v4.33.1",
    "total_passed_b06_modules": 4,
    "total_passed_b06_theorems": 38,
    "prefix_artifact_reuse_progression": "25 -> 30 -> 35 -> 40 (no passing dependency or module rerun)",
    "modules": [
      {
        "module": "Clean.Sherlock.NominalConstructorCorrespondence",
        "source_sha256": "4ffa6e8ee219f8e7153a00d9cd8b4882152d201e5cc75f7d5905004a5f4b5022",
        "result_sha256": "b5ab2d000fe596dd3002eb59c43a7c21bfc891f6ab33b64bced8a022e18aca2a",
        "command_sha256": "ed9170e821a98050bf20a324db49385cfe35c793e28ed65405e86cb773ea826d",
        "actual_exit": 0,
        "elapsed_seconds": 1.6013619610021124,
        "stdout_bytes": 0,
        "stderr_bytes": 0,
        "olean_sha256": "6a5c8a47845e2736af9752d830de26daf36913cfa8a53348fd331e1ec72883f2",
        "theorems": 7,
        "warnings": 0
      },
      {
        "module": "Clean.Sherlock.NominalObservationTheorems",
        "source_sha256": "8847bfeae3c79830b620b5eac3d3b56c00c9c045f38c19f4aaceb1354aae3fef",
        "result_sha256": "8cbedc3ba32d77bc8961e83d15bd460d64dbab5419d1a957e3bc4b85feab9034",
        "command_sha256": "5fed5cacafbeaf32e2c43c7c10eb63bd75f6ecb629dd273b9612bea121a4937a",
        "actual_exit": 0,
        "elapsed_seconds": 0.7302542269899277,
        "stdout_bytes": 648,
        "stdout_sha256": "85677641ab6ca3f8fb8bc9807ab43715b412d8c23c4ead3ceaa20bdf2f905b4c",
        "stderr_bytes": 0,
        "olean_sha256": "bcb9d214ee6a2a38b6089ad9d4c85331ecfd09651dd5dea6a6438efb09b35234",
        "theorems": 11,
        "warnings": 2,
        "warning_details": "2 retained linter.unnecessarySimpa warnings at lines 142 and 144; zero errors or sorry"
      },
      {
        "module": "Clean.Sherlock.NominalCallerTheorems",
        "source_sha256": "30528ea6b5da3a17e1e68894e87363436c9ea0edd7c0e92c2815ff2a80906612",
        "result_sha256": "8cc0cf921bf390cf71df566b501479446b77d3aa8a6618a1a2d01a37047a4cf7",
        "command_sha256": "92e644e11a9bd5633342dfd03c9d9908f3d2507840fb1f3c19e92c6e9ad52b5d",
        "actual_exit": 0,
        "elapsed_seconds": 1.0594692039885558,
        "stdout_bytes": 0,
        "stderr_bytes": 0,
        "olean_sha256": "0336907bf7fcd9a0f4e0be8a0e620c235f64cd785763aed14a5a37fd49e1b94d",
        "theorems": 13,
        "warnings": 0
      },
      {
        "module": "Clean.Sherlock.NominalArmPriorityTheorems",
        "source_sha256": "85cda0a6b78097597ab34668d121be2e05328c519c4051d22fd273b6837db600",
        "result_sha256": "7d661afc906f63223660e2795f922476dfcb5a70f03bc6098ef357001b20e82c",
        "command_sha256": "65238d0fd1dde47b79e9a922b97b964512bce5382da1898c51ff6dd6aff70069",
        "actual_exit": 0,
        "elapsed_seconds": 1.0134392280015163,
        "stdout_bytes": 0,
        "stderr_bytes": 0,
        "olean_sha256": "aa8347f166ad089643764f16f1e42ab61f0b01a24d94e4755d47765df4e4a8ac",
        "theorems": 7,
        "warnings": 0
      }
    ]
  },
  "concrete_findings": [
    {
      "id": "FINDING-B06-FINAL-01",
      "severity": "INFO",
      "category": "exact_proof_and_layout_repairs_verified",
      "location": "NominalObservationTheorems.lean:34, NominalCallerTheorems.lean:26,43-44,54-55, NominalArmPriorityTheorems.lean:121-123,132-134",
      "summary": "All repairs across the three modified B06 modules are strictly limited to proof tactics and structure-update whitespace, preserving every definition, theorem statement, and hypothesis byte-for-byte in meaning.",
      "details": "(1) `NominalObservationTheorems.lean` (revision02, 7166B, SHA256 `8847bfeae3c79830b620b5eac3d3b56c00c9c045f38c19f4aaceb1354aae3fef`) changed only line 34 (`| zero => rfl` -> `| zero => simp [advance]`), discharging `0 + second = second` via `@[simp] theorem Nat.zero_add` (`02-Init-Data-Nat-Basic.lean-133-143.txt`). (2) `NominalCallerTheorems.lean` (revision04, 11239B, SHA256 `30528ea6b5da3a17e1e68894e87363436c9ea0edd7c0e92c2815ff2a80906612`) changed only three proof-script sites: qualifying `Clean.Sherlock.NominalSemantics.bind` at line 26 so `simp` does not confuse it with `Bind.bind`; adding `@[simp] theorem Option.bind_eq_bind` (`05-Init-Data-Option-Lemmas.lean-195-202.txt`) before the actual installed left-some law `@[simp] theorem Option.bind_some (a) (f : α → Option β) : (some a).bind f = f a` (`03-Init-Data-Option-Basic.lean-119-126.txt`) at lines 43-44; and using `simp only [bindArguments, hTail, Option.bind_eq_bind, Option.bind_some, hCondition, Bool.false_eq_true, ↓reduceIte]` (`Bool.lean-202-208.txt`) on the negative branch at lines 54-55 so `hCondition` rewrites the guard to `false = true` and `Bool.false_eq_true` reduces it to `False` before `↓reduceIte`. (3) `NominalArmPriorityTheorems.lean` (revision05, 8128B, SHA256 `85cda0a6b78097597ab34668d121be2e05328c519c4051d22fd273b6837db600`) changed only whitespace indentation in `wildcard_first` (lines 121-123) and `binder_first` (lines 132-134), moving `control := .choose ...` to column 13 and its continuation `(` to column 15 to satisfy `sepByIndent`'s `checkColGe` parser rule (`01-Lean-Parser-Term.lean-343-361.txt`, `02-Lean-Parser-Extra.lean-202-205.txt`) while leaving all non-whitespace tokens identical."
    },
    {
      "id": "FINDING-B06-FINAL-02",
      "severity": "INFO",
      "category": "historical_failures_unknown_metadata_and_prior_review_correction",
      "location": "ROOT-TERMINAL-DISPOSITION.json, original-Observation/Caller/Arm failure receipts, RETAINED-CONTEXT.json",
      "summary": "All historical compiler diagnostics, the unknown child exit/elapsed metadata for Caller revision 03, and corrections to the prior review's predictions are accurately preserved.",
      "details": "The prior 25232B review (`FINAL-RESPONSE.txt` SHA256 `b0da584d6c77dfcffe81e9733781ddceb28589c3b5fecc6709cf72cdfdac7aa6`, CLI exit 0, sticky MODEL ERROR) rightly flagged `NominalObservationTheorems.lean:34` and the missing `Option.bind_eq_bind` bridge in `NominalCallerTheorems.lean`, but misstated the direction of `Option.bind_some` (suggesting a nonexistent `Option.some_bind`), missed the `bind` vs `Bind.bind` qualification error at `NominalCallerTheorems.lean:26` and the negative-branch `Bool.false_eq_true` step at line 55, and missed the two `sepByIndent` record-layout syntax errors in `NominalArmPriorityTheorems.lean:122,132`. Furthermore, in `Caller` revision 03 (`ROOT-TERMINAL-DISPOSITION.json` SHA256 `75629735e9b03709ab2fd946107d160714b002544c1535edf7d6288da08a9a11`), the child exit code and elapsed time are explicitly recorded as unknown because a post-capture Root recipe KeyError (`afterEachProfileIntegrity` instead of `postIntegrityCheck`) discarded the returned child process metadata while retaining the full 1519B stdout (`475eaacd620c99d5e5544ad2658d6f6b097b44b76574a6d650abda56765b1440`) and passing the 35-artifact post-integrity check (`post35IntegrityExit: 0`). Neither exit code nor elapsed time is fabricated for that run."
    },
    {
      "id": "FINDING-B06-FINAL-03",
      "severity": "INFO",
      "category": "publication_documents_audit",
      "location": "sherlock-nominal-proof-receipts-20261018.json, sherlock-nominal-proof-scope-20261018.md",
      "summary": "Both proposed publication documents match the exact source hashes, result hashes, timings, 38 theorem names, retained warnings, failure/unknown-metadata history, and formal scope limits.",
      "details": "`sherlock-nominal-proof-receipts-20261018.json` (5059B, SHA256 `46b774b6105afe611cbe42ddd18cbd9abaf42dd7b713a997a43dc00318df1559`) and `sherlock-nominal-proof-scope-20261018.md` (2878B, SHA256 `716db3725a83cd763ef99171f365943775456e1184f8f449c8dbff9750ff6fbe`) accurately bind every byte count, SHA256 digest, elapsed time, and theorem list across all 4 modules (11 + 13 + 7 + 7 = 38 theorems), explicitly disclose the 2 `unnecessarySimpa` warnings in `NominalObservationTheorems.lean`, the unknown child exit/elapsed metadata on failed `Caller` revision 03, the CLI exit 0 / sticky MODEL ERROR status of prior provider reviews, and all non-claims and open gates."
    }
  ],
  "retained_limits_and_open_gates": {
    "provider_terminal_status_separation": "Prior provider reviews ended with CLI exit 0 and sticky model terminal ERROR; sticky model terminal ERROR is never relabeled as provider SUCCESS, and actual Lean 4.33.1 kernel receipts alone establish theorem checking.",
    "formal_theorem_scope_boundaries": [
      "Observation theorems (`NominalObservationTheorems.lean`, 11 theorems) prove finite trace validity, prefix/final determinism, length bound (`<= count`), and absence of recorded terminal stutters over the actual `NominalSemantics.step` for arbitrary `State` and `Nat` observer budget, without proving source-program termination or physical B/A cost accounting.",
      "Caller theorems (`NominalCallerTheorems.lean`, 13 theorems) prove parameter-only support for `bindArguments` and saved-caller restoration across `functionReturn`, `letScope`, `armScope`, and pending `constructorFields`; call-entry and constructor-field return theorems retain explicit `findFunction`, `bindArguments`, `nth`, and `findExpr` premises, while unconditional return corollaries specialize `resultType` to the returned value's own canonical tag `valueType value` rather than full recursive `HasType` preservation.",
      "Arm-priority theorems (`NominalArmPriorityTheorems.lean`, 7 theorems) prove rejected-prefix steps and first-match selection under finite structural `matchPattern` or inductive `PatternMatches` premises, with unconditional first-arm priority for `.wildcard` and `.binder`.",
      "Constructor-correspondence theorems (`NominalConstructorCorrespondence.lean`, 7 theorems) establish partial boundary projection (`projectBoundary`) and commutation with B01's `ConstructorFields.beginFields`/`returnField` and `NominalValue.finishBuild` under explicit source, layout, arity, cursor, and tag checks, without erasing multi-step child evaluation intervals into a single stuttering step."
    ],
    "open_product_and_verification_gates": [
      "Production/compiler `MatchPlan` correspondence and full recursive source admission/type preservation remain open.",
      "Physical B/A helper cost accounting and whole-machine/Rust/arena refinement remain open.",
      "Circuit DSL invariants, constraint soundness, PCD, native/SDK execution, and financial/Preview/stagenet settlement acceptance remain open.",
      "Literal Lean-record roundtrip of optional descriptive `TypeDecl.name` and `ConstructorDecl.name` labels remains unsupported by the normalized wire port.",
      "Separately observed 5-case / 112-state runtime agreement contains no `Match` or recursive-function cases and is neither a premise nor a replacement for these 38 theorems."
    ]
  },
  "full_read_coverage": {
    "all_inline_bodies_read_in_full": true,
    "retained_five_dependency_modules_bound_in_same_cid": true,
    "unread_or_clipped": [],
    "inspected_inline_artifacts": [
      {
        "path": "NominalObservationTheorems.lean",
        "bytes": 7166,
        "sha256": "8847bfeae3c79830b620b5eac3d3b56c00c9c045f38c19f4aaceb1354aae3fef"
      },
      {
        "path": "NominalCallerTheorems.lean",
        "bytes": 11239,
        "sha256": "30528ea6b5da3a17e1e68894e87363436c9ea0edd7c0e92c2815ff2a80906612"
      },
      {
        "path": "NominalArmPriorityTheorems.lean",
        "bytes": 8128,
        "sha256": "85cda0a6b78097597ab34668d121be2e05328c519c4051d22fd273b6837db600"
      },
      {
        "path": "NominalConstructorCorrespondence.lean",
        "bytes": 10378,
        "sha256": "4ffa6e8ee219f8e7153a00d9cd8b4882152d201e5cc75f7d5905004a5f4b5022"
      },
      {
        "path": "batch06-operational-theorems/PROOF-SCOPE.json",
        "bytes": 4531,
        "sha256": "a025de4e88c9cb948a613ece20c9bcd4ff7558c6d1a303680e7172b01c3e2003"
      },
      {
        "path": "batch06-constructor-correspondence01/PROOF-SCOPE.json",
        "bytes": 2227,
        "sha256": "bcf2a999fb83c24302a34cdbfef5e2df14be04345936f715ba35a54316df4d15"
      },
      {
        "path": "NominalObservationTheorems/RESULT.json",
        "bytes": 2769,
        "sha256": "8cbedc3ba32d77bc8961e83d15bd460d64dbab5419d1a957e3bc4b85feab9034"
      },
      {
        "path": "NominalObservationTheorems/COMMAND.json",
        "bytes": 3255,
        "sha256": "5fed5cacafbeaf32e2c43c7c10eb63bd75f6ecb629dd273b9612bea121a4937a"
      },
      {
        "path": "NominalObservationTheorems/stdout.bin",
        "bytes": 648,
        "sha256": "85677641ab6ca3f8fb8bc9807ab43715b412d8c23c4ead3ceaa20bdf2f905b4c"
      },
      {
        "path": "NominalObservationTheorems/stderr.bin",
        "bytes": 0,
        "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
      },
      {
        "path": "NominalCallerTheorems/RESULT.json",
        "bytes": 2723,
        "sha256": "8cc0cf921bf390cf71df566b501479446b77d3aa8a6618a1a2d01a37047a4cf7"
      },
      {
        "path": "NominalCallerTheorems/COMMAND.json",
        "bytes": 3235,
        "sha256": "92e644e11a9bd5633342dfd03c9d9908f3d2507840fb1f3c19e92c6e9ad52b5d"
      },
      {
        "path": "NominalCallerTheorems/stdout.bin",
        "bytes": 0,
        "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
      },
      {
        "path": "NominalCallerTheorems/stderr.bin",
        "bytes": 0,
        "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
      },
      {
        "path": "NominalArmPriorityTheorems/RESULT.json",
        "bytes": 2756,
        "sha256": "7d661afc906f63223660e2795f922476dfcb5a70f03bc6098ef357001b20e82c"
      },
      {
        "path": "NominalArmPriorityTheorems/COMMAND.json",
        "bytes": 3255,
        "sha256": "65238d0fd1dde47b79e9a922b97b964512bce5382da1898c51ff6dd6aff70069"
      },
      {
        "path": "NominalArmPriorityTheorems/stdout.bin",
        "bytes": 0,
        "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
      },
      {
        "path": "NominalArmPriorityTheorems/stderr.bin",
        "bytes": 0,
        "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
      },
      {
        "path": "NominalConstructorCorrespondence/RESULT.json",
        "bytes": 2788,
        "sha256": "b5ab2d000fe596dd3002eb59c43a7c21bfc891f6ab33b64bced8a022e18aca2a"
      },
      {
        "path": "NominalConstructorCorrespondence/COMMAND.json",
        "bytes": 3294,
        "sha256": "ed9170e821a98050bf20a324db49385cfe35c793e28ed65405e86cb773ea826d"
      },
      {
        "path": "NominalConstructorCorrespondence/stdout.bin",
        "bytes": 0,
        "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
      },
      {
        "path": "NominalConstructorCorrespondence/stderr.bin",
        "bytes": 0,
        "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
      },
      {
        "path": "historical-original-Observation-zero-rfl/RESULT.json",
        "bytes": 1051,
        "sha256": "55d620cf88c29c801844680d33c174bf68744c0e4229adced1acbf61f049ac61"
      },
      {
        "path": "historical-original-Observation-zero-rfl/stdout.bin",
        "bytes": 1104,
        "sha256": "a952899c312669264b17d77f9858ede255e0dad8036e7a24a821384a2ca3f962"
      },
      {
        "path": "historical-original-Observation-zero-rfl/stderr.bin",
        "bytes": 0,
        "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
      },
      {
        "path": "historical-original-Caller-three-errors/RESULT.json",
        "bytes": 1044,
        "sha256": "875fc0ddc366aafc0b2af0a94862427128054fdf6b5d90eb2cfba4c8764b91ab"
      },
      {
        "path": "historical-original-Caller-three-errors/stdout.bin",
        "bytes": 4652,
        "sha256": "97d8e8f7556a87c70a3df86455563a3484c1b8ec99d15350db91f52eff85fe2e"
      },
      {
        "path": "historical-original-Caller-three-errors/stderr.bin",
        "bytes": 0,
        "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
      },
      {
        "path": "historical-Caller-revision03-false-eq-true/ROOT-TERMINAL-DISPOSITION.json",
        "bytes": 1246,
        "sha256": "75629735e9b03709ab2fd946107d160714b002544c1535edf7d6288da08a9a11"
      },
      {
        "path": "historical-Caller-revision03-false-eq-true/stdout.bin",
        "bytes": 1519,
        "sha256": "475eaacd620c99d5e5544ad2658d6f6b097b44b76574a6d650abda56765b1440"
      },
      {
        "path": "historical-Caller-revision03-false-eq-true/stderr.bin",
        "bytes": 0,
        "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
      },
      {
        "path": "historical-original-Arm-two-record-layout-errors/RESULT.json",
        "bytes": 1056,
        "sha256": "1684cf33d8fb5b8dae8cb65e8ff60e0e8ca264f00dd72f8731d080c6f5f8c3a6"
      },
      {
        "path": "historical-original-Arm-two-record-layout-errors/stdout.bin",
        "bytes": 482,
        "sha256": "d12ba60859a916ef086455e10a6f1b3a67e7fd7f17dfca6f02677470924c8b41"
      },
      {
        "path": "historical-original-Arm-two-record-layout-errors/stderr.bin",
        "bytes": 0,
        "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
      },
      {
        "path": "batch06-operational-correction02/UNIQUE-EDIT-AND-INVERSE.json",
        "bytes": 1183,
        "sha256": "c390cd22822b00c696ed72d53d3425bb0ebe29eac8bb158937f0996e7fb43c84"
      },
      {
        "path": "batch06-operational-correction02/SOURCE.diff",
        "bytes": 502,
        "sha256": "78909fb7287cd611620abd5e5c7cc04d14d9c0a4a7a504a20a24df2952d908c1"
      },
      {
        "path": "batch06-operational-correction03/PROOF-EDIT-AND-INVERSE.json",
        "bytes": 2149,
        "sha256": "58cecf28a25a6f098ad4bfa442411666055dafc25a947035b8e4267540ef727e"
      },
      {
        "path": "batch06-operational-correction03/SOURCE.diff",
        "bytes": 1652,
        "sha256": "e84d16effc9b3dbd6b6709c2840a83c035544c2dae382efc38ecfde45ce04f9d"
      },
      {
        "path": "batch06-operational-correction04/ONE-EDIT-AND-INVERSE.json",
        "bytes": 1168,
        "sha256": "50ca46f45af6bb72bbd7cce84f41b7e08556ae8659155e233975b0c1dfe2ab88"
      },
      {
        "path": "batch06-operational-correction04/SOURCE.diff",
        "bytes": 554,
        "sha256": "5867d87dcda5355560d6af76e885e3e38359296dd9e5d1dc6ebe976a678fa8ba"
      },
      {
        "path": "batch06-operational-correction05/SYNTAX-EDIT-AND-INVERSE.json",
        "bytes": 1493,
        "sha256": "3c19507c213848af74f5b7981ebd68a4769704ff408a5f59f20f648672d114e5"
      },
      {
        "path": "batch06-operational-correction05/SOURCE.diff",
        "bytes": 1703,
        "sha256": "c239fa3344e074556bfd8b04a1ec6635f0ecd021770436516093a26bac9010ef"
      },
      {
        "path": "SOURCE-AND-STATEMENT-PRESERVATION.json",
        "bytes": 5412,
        "sha256": "52171c7777bbb9d87bfe39c4c06ab72147754b2f57044cde2bb9be6e1f700d7c"
      },
      {
        "path": "RETAINED-CONTEXT.json",
        "bytes": 3718,
        "sha256": "30a0fae70829ee00c0b07178967784a8f1a1217076f3178e91b74d202d07ed20"
      },
      {
        "path": "REVIEW-SCOPE.revision02.json",
        "bytes": 1948,
        "sha256": "ef2aeb1ea47b547c27b53a902f70bd03ef9079fd8cfc7897cd5aa2f61c30c5d3"
      },
      {
        "path": "sherlock-nominal-proof-receipts-20261018.json",
        "bytes": 5059,
        "sha256": "46b774b6105afe611cbe42ddd18cbd9abaf42dd7b713a997a43dc00318df1559"
      },
      {
        "path": "sherlock-nominal-proof-scope-20261018.md",
        "bytes": 2878,
        "sha256": "716db3725a83cd763ef99171f365943775456e1184f8f449c8dbff9750ff6fbe"
      },
      {
        "path": "01-Init-Prelude.lean-1764-1779.txt",
        "bytes": 819,
        "sha256": "bed0d68bd97748f4f905b100ffe3ad02cb93ec247444b4429fe5cbcb6ccc0cd8"
      },
      {
        "path": "02-Init-Data-Nat-Basic.lean-133-143.txt",
        "bytes": 342,
        "sha256": "77fde11bf8914a961da7bda2b35b1d33ff18e81683b2e9a116ab50a9f10634cb"
      },
      {
        "path": "03-Init-Data-Option-Basic.lean-119-126.txt",
        "bytes": 376,
        "sha256": "dded4483f8710d33459adb37368be255441f98ab007cd62b6f0971532bc5de03"
      },
      {
        "path": "04-Init-Data-Option-Basic.lean-570-577.txt",
        "bytes": 161,
        "sha256": "4ca0bf05fb09af69406efa1deb553d50483789f15dab88463b748d77033b0482"
      },
      {
        "path": "05-Init-Data-Option-Lemmas.lean-195-202.txt",
        "bytes": 239,
        "sha256": "d09d2b863d5d81ad947fe1f1ba3dfa65d39dae8e8619cd166735a002a30f4ba6"
      },
      {
        "path": "Bool.lean-202-208.txt",
        "bytes": 247,
        "sha256": "9ae0cff57fff078874675cc2399e4a8e1db3e24918953a1f142cb3a5d4445f66"
      },
      {
        "path": "01-Lean-Parser-Term.lean-343-361.txt",
        "bytes": 808,
        "sha256": "afba2e0206d4bc81e75535b05075524ff3ce528d1b477d404ec95d357d45d758"
      },
      {
        "path": "02-Lean-Parser-Extra.lean-202-205.txt",
        "bytes": 352,
        "sha256": "2ff2be7d6e21bf2f03edf0603d679826e2dd9bf0161e6fbc0d2839735d8bdf09"
      },
      {
        "path": "07-inputs-14-TOOLS-AND-ENV-SUMMARY.json",
        "bytes": 2971,
        "sha256": "b0afbf098e447143997c04780fe299f9cadc09b3cf4adf1ef1d694bef6694d9b"
      }
    ]
  },
  "exact_remaining_gaps": [
    "Root-only R-staging, Git commit/merge, and push of the approved 4 final theorem source files and 2 qualified publication documents to the authorized Fenrir Clean fork F-branch.",
    "Open downstream roadmap gates: production/compiler MatchPlan correspondence, full recursive source admission/type preservation, physical B/A accounting, whole-machine/circuit/native/SDK/financial/stagenet acceptance."
  ],
  "new_subject_executions_requested_or_observed": 0
}

