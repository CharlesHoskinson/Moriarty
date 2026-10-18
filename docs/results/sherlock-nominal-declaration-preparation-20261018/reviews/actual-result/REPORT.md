```json
{
  "request_id": "B06-NOMINAL-ACTUAL-RESULTS-006027",
  "CID": "29f6efee-2b5e-4f3f-b663-bad4292ff558",
  "model": "Gemini 4 Argon (High) via gemini-4-argon-high standing fallback under Root-observed Opus quota outage (not Opus)",
  "cwd": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/212-word-public-integration-final-review/actual-same-owner-review-01/review-session",
  "independence": {
    "status": "CONFIRMED_NON_AUTHOR",
    "nominal_8_files_author": "bb6043a1-63f1-4de9-b26f-ef50139233e3",
    "pass_b_583b_correction_author": "d2f4cab5-a214-4450-af91-35feb60db39f",
    "independent_oracle_and_consumer_author": "external_independent_planner",
    "retained_unrelated_history": "Prior R203/R212 word reviews and R402 additive host_owner_engine_with_word_layout constructor authorship retained; zero authorship of nominal candidate files, Pass-B fix, or nominal oracle/consumer.",
    "tools_or_commands_invoked": 0
  },
  "actual_result_coverage": {
    "phase1_core_compile": {
      "launch_sha256": "92d3e265bb4ab7e3058958a99e083fd4b45796e90b33862fb6aa403db367dd3a",
      "result_sha256": "5d64394dd06694fb2ca16c563a1960d4edeb38a63bcfaaa320da0a62dd49dc6f",
      "exit_code": 0,
      "elapsed_seconds": 3.9878708139985974,
      "stdout_bytes": 0,
      "stderr_bytes": 4862,
      "stderr_sha256": "abba7b5a147ba4c0242442c017816d8814a4e2ba407519c7b67c572638699634",
      "warnings": "9 pre-existing unused warnings in unrelated modules; 0 warnings in candidate-03 nominal files",
      "rlib_sha256": "2eaa593cb7ee79ba5e6d09ec59fb14b846b4f34e17d79aace3b43b85ceae90f9",
      "depinfo_sha256": "ca7c051f7e90d276bf413c27fd5f58adc21da10bd8dc2c5bc73072312d7f5dd1",
      "input_pins_474_pre_post_equal": true
    },
    "phase2_consumer_compile": {
      "launch_sha256": "7a2eb90cc2dc8579214011ff6de65c1f40ed8a675db4fb286122f3fd10df4656",
      "result_sha256": "97b23b6851cb68c64e42c1a3bdbf459819ab3772f66d1c6896488a3501c581cd",
      "exit_code": 0,
      "elapsed_seconds": 0.3386072620014602,
      "stdout_bytes": 0,
      "stderr_bytes": 0,
      "consumer_source_sha256": "0d7d21b44cfdf43bc35aae8ca3eadc09f38ba8e656108340ac5372e7633ea6ca",
      "test_elf_sha256": "1d05301b9b7df56ea35ec796842b1a78d7de9e8bfe6e48f85830a5c6c3403bd5",
      "depinfo_sha256": "b5ba2ea2a4b8667259585a3aafc20514afeb4377dcbab9d48fd0af034150a31f"
    },
    "phase3_list_exact_one_test": {
      "launch_sha256": "950ff08fb00c0b89e4e83dc94661bc6184da8c93e03b25a958cf6251142009b4",
      "result_sha256": "49a256dedbc6903923e832049f1eb7df29248a863a5f58272edcd4597c1e9d8d",
      "exit_code": 0,
      "elapsed_seconds": 0.0035452539996185806,
      "stdout_bytes": 100,
      "stdout_sha256": "8cb4934eb0b4c14a44d7dc1d34ef104f1cf77ec41f27716cf0f8e1a272f40da4",
      "stderr_bytes": 0,
      "exact_list_match": true
    },
    "phase4_run_exact_positive_test": {
      "launch_sha256": "4a17d828071170d75249d4b587ffeb483cf0128588f8659277c32bc1d78577f3",
      "result_sha256": "3375b45dbdd52f71c80c0f97a8202f71e5078a885c1b80a6faac331f42c4472c",
      "remaining3_summary_sha256": "a46da070d235be8ddcaae4f16068255250fb31b04e6ef593e384e3e371f361bb",
      "exit_code": 0,
      "elapsed_seconds": 0.003976133000833215,
      "stdout_bytes": 196,
      "stdout_sha256": "75cda8dbab7cf9c740f736e27885a505992dd1c46d1f7a3873e16bd59b148fa0",
      "stderr_bytes": 0,
      "all_480_inputs_pre_post_equal": true,
      "oracle_bindings_verified": {
        "oracle_freeze_sha256": "74dae0e40284c71001c69e72f0199a0e49e845c20593da497db195823b56c2eb",
        "input_freeze_sha256": "c1392540798055e1c3bd86961e82545d4da53618d8d02581d6045916b7aa75cc",
        "local_type_occurrences_sha256": "a08a37c01f079678b566a6747d3d4a0bc6c034c78d1c72b8eb6c993c30502f8c",
        "expected_freeze_sha256": "3d4996a1fa4b87da0614f0df781b5ba3ab9db93a8dc82da3ac861f0b9c89d9b1"
      }
    }
  },
  "source_emission_qualification": {
    "projection_sha256": "2783f9a66fc4d10ab6d5d9be11457b37ec8002f3bd3141be69d5ba07ba73b9cd",
    "evidence_basis": "Textrustc dep-info files (core-rlib.d 53572B and nominal-declaration-tests.d 2393B) plus file SHA256 pins; not ELF binary disassembly.",
    "core_emitted_source_count": 83,
    "candidate03_emitted_overlay_count": 7,
    "unemitted_candidate03_file": "crates/mori-core/src/sherlock/helper_machine/shared/owner_prepare/nominal_decl_producer_tests.rs (df3c3519de06055d860f4389820713ec95e8be01a49ff0344b2771cfbfed3b98) is #[cfg(test)]-gated and was neither emitted into libmori_core.rlib nor executed.",
    "consumer_emitted_file_count": 3,
    "consumer_emitted_files": [
      "sherlock_nominal_declarations_independent.rs (0d7d21b4...)",
      "LOCAL-TYPE-OCCURRENCES.freeze.json (a08a37c0...)",
      "EXPECTED.freeze.json (3d4996a1...)"
    ],
    "input_freeze_qualification": "INPUT.freeze.json (c1392540...) is a pinned source-level specification contract whose Wrap<T>/Zeta/Alpha declarations are constructed literally in Rust test code; it is not an include_str! dependency in nominal-declaration-tests.d."
  },
  "factual_metadata_corrections_accepted": {
    "source_review_page_count_qualification": {
      "sha256": "227122be606d38aa243726c31f5ba072172a23f011b0c46c9f9e851301982bb2",
      "accepted_as_factual_metadata_only": true,
      "note": "Prior raw report text stated 40 pages; actual read coverage (3d35aa7b... / 7d568d10...) completed all 41 pages across 24 bodies in full LF-compatible reads."
    },
    "root_scope_wording_correction": {
      "sha256": "827e05756eb2d44c31861c39a304fd65b8bbbf3df9a816da335b43c19b4a0ff9",
      "accepted_as_factual_metadata_only": true,
      "note": "Ordinary Rust test ELF nominal-declaration-tests executed --list and --exact positive test; Native06 mori_backend SDK ELF was not executed."
    }
  },
  "actual_result_disposition": "APPROVE",
  "claim_scope": "Normal host declaration-only nominal preparation (prepare_nominal_declarations, walk_declaration_type_occurrences, 12-name ASCII Pass-B interning excluding type parameters, FIFO tables, and arity bounds) verified by 1 executed independent positive test. No MachineImage, runtime transition, author unit-test execution, negative-test execution, Native06 SDK execution, or proof/target claim.",
  "current_source_compatibility": {
    "anchors": {
      "original_006_027_md_sha256": "7ae8bf2c070b913f78979d292d198a384395d84db20a65077c645a9e91952979",
      "closures_projection_sha256": "3d5e5283118b627f8de7a939e141da6712aa441aa7822c07232c1df641b00bb5",
      "proposed_rebind_sha256": "1d866ea67f3a1564a02ba2f8c7a334572147028f7f782b7ac3f1d73c93383431",
      "legacy_preservation_proof_sha256": "be2a89167247ef5b4ae2ecdf23859d888ef25b0ef4c9ce3d1c92227494b6cf41"
    },
    "UIREQ-006": {
      "verdict": "APPROVE_CONDITIONAL_ROOT_ADOPTION",
      "rationale": "UIREQ-006 requires scalar primitives (add, sub, mul, lt) to produce declared checked UInt128 or Bool outcomes. Its closure rests on typed-A scalar evaluation (29705882...) and cites shared.rs only as an ancillary implementation/citation reference. Reversing the single 10-line module-registration hunk at the bottom of shared.rs (479eb398... -> 66f9eea2..., endLine 652 -> 662) reconstructs all 43,892 original bytes; all scalar types and pure_leaves::checked_{add,sub,mul} are byte-identical.",
      "evidence_limits": "Historical R92 execution receipts retain original preimage hashes and were not rerun; currentness reader is NOT_RUN until Root installs candidate-03 and records adoption."
    },
    "UIREQ-027": {
      "verdict": "APPROVE_CONDITIONAL_ROOT_ADOPTION",
      "rationale": "UIREQ-027 requires supported DedicatedSorts preparation to produce a ProgramImage preserving source membership, original function-table order, and SourceOccurrenceId provenance. All 69 non-candidate source pins match @W@. In shared.rs (479eb398... -> 66f9eea2...) and owner_prepare.rs (625cade4... -> ae9b29a7...), removing the additive 10,786B nominal-declaration block and 1-line #[cfg(test)] registration reconstructs all 59,455 original owner_prepare.rs bytes. prepare_core, check_program_view, admit, admit_run, collect_names, name_id, and DedicatedSorts ProgramView indexing are 100% unmodified, and CheckedTypeDeclView rejects non-empty declarations under DedicatedSorts.",
      "evidence_limits": "Historical R77/R96/R109/R115 receipts retain their recorded preimage hashes and were not rerun under the new hashes; currentness reader is NOT_RUN until Root installation and limitations-note append."
    }
  },
  "mandatory_corrections": [],
  "preserved_ledger_scope": {
    "ledger_sha256": "655cc020da399728823fd81891f9c287990d680e7f3aa15a8584cb73473e1a9d",
    "original_closed_count": 12,
    "original_open_count": 23,
    "refinements_closed_count": 0,
    "refinements_open_count": 25,
    "explicitly_open": ["UIREQ-004", "UIREQ-007", "UIREQ-030", "UIREQ-034", "SCON-016", "SCON-016.R02"]
  }
}
```

