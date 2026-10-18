{
  "coverage": {
    "all_required_pages_complete": true,
    "unique_pages": 81,
    "bodies": 70,
    "gaps": [
      "SOURCE_AVAILABLE_NOT_FRESHLY_READ: 570 cached third-party crate source files across the 17-package Linux Clap closure (factored losslessly as SHA256-pinned arrays in ORDINARY-CLAP-CLOSURE.proposed.json sha256 7afc898a05ada698cdc17350d690305797dd45cde4036285b08adf0e968fefd1; all 17 Cargo.toml manifests and clap_derive/src/lib.rs entry were read in full, while delegated submodules such as clap_derive::{attr,derives,dummies,item,macros,utils} and clap_builder/syn/anstream internal modules were not freshly read line-by-line).",
      "SOURCE_AVAILABLE_NOT_FRESHLY_READ: 134 unchanged normal mori-core and mori-lexer .rs files in the 138-file R301 snapshot outside the 4 freshly read Core files (compile_artifact.rs, draft_simulation_report.rs, draft_contract.rs, draft_execution.rs), bound by 301-new-simulation-report-normal-core-QA/SOURCE-COMPOSITION.json sha256 573487056eb86c7905cadf8e34d40fcd77c1dcd46b50cb638e9338e006b7803e and rows value sha256 99f0853e86dabeeee0ddb80f5944f6286cce9519b3f5e2ff263021c163d083ea.",
      "PINNED_BINARY_IDENTITY_NOT_SEMANTIC_AUDIT: Reused normal .rlib artifacts (unicode-ident, proc-macro2, quote, serde, serde_json, k256, sha2, thiserror, mori_lexer, R264 mori_core), existing serde/thiserror derive proc-macro shared objects, and rustc/LLVM/GCC15/collect2/bfd/glibc sysroot ELF binaries (275 historicalNormalToolsAndDeps and 121 passiveELFFiles in TOOLS-LINKER-SYSROOT.json sha256 b98a2a96963d81ea225ce4406b8dbab47ecbc345a07fe31cb7cce044caee127a) are bound by byte/SHA256 identity rather than binary decompilation.",
      "OUTSIDE_SELECTED_CLOSURE: Full mori binary crate command modules (other than cli.rs, main.rs, commands/replay.rs, commands/simulate.rs, commands/simulate_sherlock.rs) and mori-backend/mori-ledger/midnight-* SDK/ledger dependencies are outside the allowlisted readset and outside R299 and R301."
    ]
  },
  "identity": {
    "requested": "gemini-4-argon-high",
    "returned": "Gemini 4 Argon (High)",
    "HIGH_evidence_limits": "Fresh designated non-author review in isolated workspace (/tmp/moriarty-r300-shared-cli-source-review-y9gzlvos, CID 5bbfb99c-d07d-4659-81fa-29f4d817d7d9) under standing user-authorized observed Opus-outage fallback; single review call with zero tool/command/write side effects and native_authority=0. Evaluates static source correctness, mechanical move/reexport equivalence, and prospective operation scope for R299 and R301; does not execute any compiler or test, does not observe runtime outputs, and does not provide an independent runtime result oracle."
  },
  "source_decision": {
    "classification": "APPROVED_SOURCE_COMPOSITION_AND_MECHANICAL_EXTRACTION_UNRUN",
    "exact14file_scope": {
      "composed_file_count": 14,
      "r293_own_files": 10,
      "r270_files": 7,
      "overlap_replaced_by_r293": [
        "crates/mori/src/cli.rs",
        "crates/mori/src/commands/simulate.rs",
        "crates/mori/src/commands/simulate_sherlock.rs"
      ],
      "files": [
        {
          "repository_path": "crates/mori/src/cli.rs",
          "bytes": 4925,
          "sha256": "0c6e04aa1ab6710d9d54cd68e5b05b734623b92342a7e4a27d47c6830562cff3",
          "r270_baseline_preimage_sha256": "64b2dac5e7e8de6ec7ae4a839ad2c55f7648c325342651bbb0d895ebafb64e13",
          "r293_preimage_sha256": "6442d689de366286a2cfad6726c847e29739f6bcd1728def9bd9ce61051b98c8",
          "verified": true
        },
        {
          "repository_path": "crates/mori/src/cli_args/replay.rs",
          "bytes": 1044,
          "sha256": "9442ef4fb454798531fa473f66dabfe11568a07b5bdbf51214d8a2ac9fc6e1e7",
          "r293_preimage_sha256": null,
          "verified": true
        },
        {
          "repository_path": "crates/mori/src/cli_args/simulate.rs",
          "bytes": 2733,
          "sha256": "d7d8fbc9599de6e4fd68da9088cc6349528a5b98b277ed79399e805450bb9de1",
          "r293_preimage_sha256": null,
          "verified": true
        },
        {
          "repository_path": "crates/mori/src/cli_args/simulate_selection.rs",
          "bytes": 2521,
          "sha256": "ce606befbb03251be10e199b6f9feff8037f0dc5ef243af8451a2695f4504272",
          "r293_preimage_sha256": null,
          "verified": true
        },
        {
          "repository_path": "crates/mori/src/cli_args.rs",
          "bytes": 500,
          "sha256": "23d4cd2fac3cb2ab97c94cdb8b702fc75db5cea31c728d47cd074b470f6c73b3",
          "r293_preimage_sha256": null,
          "verified": true
        },
        {
          "repository_path": "crates/mori/src/commands/replay.rs",
          "bytes": 5923,
          "sha256": "3c8902d85bb7431c0e6ff77e158ed0c9f9abaac30fde8f784d4f7c90a10c83d0",
          "r293_preimage_sha256": "41c907d748aaff6619c4087af1c4f16622cf3f4e6df6d51fa18d57f7d56e04d0",
          "verified": true
        },
        {
          "repository_path": "crates/mori/src/commands/simulate.rs",
          "bytes": 8561,
          "sha256": "6b00fbb2678cb2ed58d12afdc7a9ca183f386719e5b2f8a89af6f13ecea2b026",
          "r270_baseline_preimage_sha256": "72488636c58101d78fec39f60e2106d40d2c67d44ef2a70c4ccd4d15cd3ac216",
          "r293_preimage_sha256": "51addd56a6428fa1c1cf5999e1387474ff0fcd2abaf032117a3c79c222f6d748",
          "verified": true
        },
        {
          "repository_path": "crates/mori/src/commands/simulate_sherlock.rs",
          "bytes": 2413,
          "sha256": "b8481d0fd220718e71bf2ff37ac4346ce1991fb7b729aaebbe14f6da88a17c48",
          "r293_preimage_sha256": "2f398c7d8d9906a1dde2f694edab2137a6e6ba2ac64fed6a06c24df2023ffc93",
          "verified": true
        },
        {
          "repository_path": "crates/mori/src/commands/simulate_sherlock_tests.rs",
          "bytes": 1749,
          "sha256": "5af09563c02ff312a7e4b1f73b63bc29e425d8ff1836c6aabbe7051dfcf4ef3f",
          "r293_preimage_sha256": null,
          "verified": true
        },
        {
          "repository_path": "crates/mori/src/main.rs",
          "bytes": 509,
          "sha256": "ca64ee8d97ef9b76b32aa0c92de97b065bec9dee3f716b2a8a37d4002099685b",
          "r293_preimage_sha256": "35988592bec7038a7674403a0e897184d1099dd4164e89a3ca0aa22ab0b3a232",
          "verified": true
        },
        {
          "repository_path": "crates/mori/tests/fixtures/sherlock-simulate/CLI-EXPECTATIONS.json",
          "bytes": 3771,
          "sha256": "abe42414166c34472065f56266220fe33cf39ae673334f2a69b0e5ec13617730",
          "r293_preimage_sha256": null,
          "verified": true
        },
        {
          "repository_path": "crates/mori/tests/simulate_arguments.rs",
          "bytes": 2242,
          "sha256": "f0fade9ef441a076bae1ad37eea9bda51fbdf6692ede47ac04fcfa29e2d0fa44",
          "r293_preimage_sha256": null,
          "verified": true
        },
        {
          "repository_path": "crates/mori-core/src/sherlock/compile_artifact/draft_simulation_report.rs",
          "bytes": 2051,
          "sha256": "5b27e4638d17ab474c9d5111437251675d5cd18c3d43cf0b4dcaf2631b1e859c",
          "r293_preimage_sha256": null,
          "verified": true
        },
        {
          "repository_path": "crates/mori-core/src/sherlock/compile_artifact.rs",
          "bytes": 14950,
          "sha256": "9c0749c098cacc523632930a85a1a3c0be1ccf0d5e4a0bccaa35a7a017edd70f",
          "r270_baseline_preimage_sha256": "17ff0a6b88b84c1669bc0594ce561834b83f7436a8028578e918788d6611c99f",
          "verified": true
        }
      ]
    },
    "move_and_type_identity": {
      "single_canonical_definitions": true,
      "no_duplicate_Cli_or_Args": true,
      "SignedProfile_identity": "Defined once in crates/mori/src/cli_args.rs (lines 5-10, 214B verbatim block sha256 903a4d37a7afe647295e0863809e6ef027186e6d2f2285b30927e9026a6fd197) and publicly reexported in crates/mori/src/cli.rs (line 81: pub use crate::cli_args::SignedProfile;). Consumed identically by cli_args::replay::AuthorizationArgs, commands::replay, and cli.",
      "AuthorizationArgs_identity": "Defined once in crates/mori/src/cli_args/replay.rs (lines 6-20, 608B verbatim block sha256 db3918d405184b2f86539113677354985778f7223e5e7eeb0748f11b2d4ed6d9) and publicly reexported in crates/mori/src/commands/replay.rs (line 14: pub use crate::cli_args::replay::AuthorizationArgs;). Flattened into cli_args::simulate::Args via super::replay::AuthorizationArgs and into commands::replay::Args.",
      "simulate_Args_ and_logical_cap_identity": "Defined once in crates/mori/src/cli_args/simulate.rs (Args lines 5-51, 2258B verbatim block sha256 5a0deda6c1ef5cdb097f96331a085606b7194cc58f6e516365082436ee7ed14a; logical_cap lines 52-55, 223B verbatim block sha256 cf86290424d00ea7c16c76fc119db438f26037704ae4bfc49f9b7d6e6feb7b85). Consumed directly by crate::cli::Command::Simulate(simulate::Args) via use crate::cli_args::simulate, reexported at crate::commands::simulate::Args, and included directly by crates/mori/tests/simulate_arguments.rs.",
      "selection_identity": "Defined once in crates/mori/src/cli_args/simulate_selection.rs (lines 7-44, 2265B after-block sha256 a4e12f1337500337fd59b2476672744a2551adb728ca971a5ba070b15be9970f; normalized 2272B pre-block sha256 77abe584b7efd7681f3c6314505ce21a87f9752dc971f3801057ed37fbe25f3b with sole pub(super)->pub change), reexported at crate::cli_args::simulate::selection and reexported as pub(super) use crate::cli_args::simulate::selection in crates/mori/src/commands/simulate_sherlock.rs (line 9)."
    },
    "AuthorizationArgs_present_and_handler_preservation": {
      "AuthorizationArgs_present_verbatim": true,
      "AuthorizationArgs_present_sha256": "875698a30f2532721e631e45eded86eb4e2011bec612f670dee898a2a0152d85",
      "AuthorizationArgs_load_in_commands_replay_verbatim": true,
      "replay_handler_suffix_byteexact": true,
      "replay_handler_suffix_sha256": "80f68b39abd0bff2cc0ca694969e440681d00ba18ef302e79ce0ed01aee3c481",
      "simulate_legacy_and_handler_suffix_byteexact": true,
      "simulate_handler_suffix_sha256": "ee51a2adb728faa52007e8a4b3b0b031907d32c514b396fc0f0ece461b81b72d",
      "simulate_sherlock_handler_suffix_byteexact": true,
      "simulate_sherlock_handler_suffix_sha256": "08a696b96e3e4a8c82745c1df46c25daef33f42b46dd5ff740ac8383125a4eb6",
      "no_policy_guard_default_or_handler_reimplementation": true
    },
    "visibility_API_compatibility": {
      "main_rs_mod_registration": "Adds private mod cli_args; in crates/mori/src/main.rs (line 9).",
      "reexports_preserve_internal_paths": "crate::cli::SignedProfile, crate::commands::replay::AuthorizationArgs, crate::commands::simulate::Args, and crate::commands::simulate::sherlock::selection all resolve to the exact same underlying types/functions at their prior paths and visibilities.",
      "selection_visibility_change": "Sole declaration change is pub(super) fn selection -> pub fn selection in cli_args/simulate_selection.rs (reexported via pub use selected::selection in cli_args/simulate.rs and pub(super) use crate::cli_args::simulate::selection in commands/simulate_sherlock.rs) so tests/simulate_arguments.rs can call cli_args::simulate::selection(&args) while cli_args itself remains a private module of the binary crate.",
      "inherent_impl_split_valid": "Inherent impls for AuthorizationArgs::load (in commands::replay) and simulate::Args::legacy (in commands::simulate) remain within the same crate (mori) when compiling the binary crate, while the SDK-free direct test target compiles only the 6-file cli_args closure without mori_backend.",
      "core_report_module_visibility": "crates/mori-core/src/sherlock/compile_artifact.rs adds pub mod draft_simulation_report; (2 lines, +89B over R258 parent 17ff0a6b88b84c1669bc0594ce561834b83f7436a8028578e918788d6611c99f). draft_simulation_report.rs accesses private parent items super::wire::{self, OwnerWire} and super::CompileFailure::consumer via standard Rust child-module visibility without widening wire or consumer to public."
    },
    "findings": [
      "All 14 composed candidate files in BODY 001..014 match the exact byte counts and SHA256 digests in SOURCE-COMPOSITION.json (BODY 026) and R299/R301 snapshot bindings (BODY 066, BODY 069, BODY 070).",
      "R270 command contract (BODY 030) and output contract (BODY 031) are faithfully implemented by commands/simulate.rs, commands/simulate_sherlock.rs, cli_args/simulate.rs, cli_args/simulate_selection.rs, and mori-core/.../draft_simulation_report.rs: explicit --execution-profile sherlock-helper-microstep/1, --source-profile moriarty-sherlock-draft/1, --entry, --execution-config, and --target are required with zero defaults or source autodetection; legacy Core/staged/replay-trace paths remain untouched when Sherlock flags are omitted; event sink retains no event vector; preparation and post-run execution are kept distinct in DraftSimulationReport; serialization failure maps through run.failure(\"serialization\", ...) preserving evaluation_performed=true, completed=true, and events_forwarded; ExitCode::SUCCESS is returned only for OutcomeC::Halted.",
      "All 6 mechanical moves in MECHANICAL-MOVE-MAP.json (BODY 024) and literal checks in BODY 033 are verified line-for-line against the full R293 preimages (BODY 015..019) and R270 baseline preimages (BODY 020..022)."
    ],
    "gaps": [
      "Source review alone does not execute rustc, linker, or test binaries; all 14 candidate files remain UNRUN.",
      "Full mori binary compilation (which requires mori-backend, mori-ledger, and Midnight SDK/ledger crates) and end-to-end CLI dispatch/JSON report runtime observation are not executed by R299 or R301."
    ]
  },
  "original24row_assessment": {
    "frozen_cases": {
      "fixture_path": "crates/mori/tests/fixtures/sherlock-simulate/CLI-EXPECTATIONS.json",
      "bytes": 3771,
      "sha256": "abe42414166c34472065f56266220fe33cf39ae673334f2a69b0e5ec13617730",
      "r270_freeze_sha256": "ebae691605d1f34aa8c2503339c0ec4bd59d788e0f5c59a274dc08442d5e2e50",
      "r293_freeze_sha256": "5d2dc2079001240a26656c180e827348a726b22545d39337408f52c3e81a52a9",
      "row_count": 24,
      "distribution": "1 valid draft route row (parse=true, route=draft), 4 valid legacy route rows (parse=true, route=legacy), 13 Clap conflict/required-arg rejection rows (parse=false), 4 pure selection rejection rows (parse=true, route=select), 2 config/target missing rejection rows (parse=true, route=config)."
    },
    "actual_production_source_scope": "crates/mori/tests/simulate_arguments.rs (2242B, sha256 f0fade9ef441a076bae1ad37eea9bda51fbdf6692ede47ac04fcfa29e2d0fa44) includes ../src/cli_args.rs and tests the exact production cli_args::simulate::Args type (via clap::Args::augment_args and clap::FromArgMatches::from_arg_matches) and exact production cli_args::simulate::selection(&args) function across all 24 rows, using existing mori_core::sherlock_compile_artifact::draft_contract::select_source.",
    "independence_limits": "CLI-EXPECTATIONS.json is an expectations-first command-shape freeze authored in R270 and reused byte-for-byte in R293, not an independent non-author runtime oracle. The R293 author disclosed prior ordinary-QA context and made no claim of global runtime-oracle blindness.",
    "missing_global_Cli_handler_runtime_evidence": "The direct test in simulate_arguments.rs validates subcommand Args augmentation, FromArgMatches conversion, and pure selection(args) only. It does not construct or parse the top-level global Cli struct or Command enum in cli.rs, does not invoke commands::simulate::run or commands::simulate::sherlock::run, does not read source/config/target files from disk, does not execute run_draft_host or draft_simulation_report::report, and produces no runtime simulation or native/ledger evidence."
  },
  "R299_ordinary_operation": {
    "classification": "ELIGIBLE_ORDINARY_SDK_FREE_CLAP_CLOSURE_AND_DIRECT_ARGS_TEST_OPERATION_FOR_ROOT_SELECTION",
    "exact_plan_sha256": "59d3f1ce603f3927ed1278302c3212c2e443845b012dbc3a3378d4af60ad42f0",
    "exact_phases": [
      "compile_syn",
      "compile_heck",
      "compile_anstyle",
      "compile_utf8parse",
      "compile_anstyle_parse",
      "compile_anstyle_query",
      "compile_colorchoice",
      "compile_is_terminal_polyfill",
      "compile_anstream",
      "compile_clap_lex",
      "compile_strsim",
      "compile_clap_builder",
      "compile_clap_derive",
      "compile_clap",
      "compile_direct_six_source_consumer",
      "list_exact_one_direct_test",
      "shared_production_simulate_arguments_and_selection_match_original_24_cases"
    ],
    "dependency_and_proc_macro_execution_scope": {
      "cached_packages_total": 17,
      "reused_normal_libraries": [
        "unicode-ident 1.0.26 (libunicode_ident-1e23094c28f51ebc.rlib sha256 e3d1f770afba349a7caf95a0868d8d945f9a0d1f725d5ede88c1f8f0bedd7247)",
        "proc-macro2 1.0.107 (libproc_macro2-378d9a773d1eff57.rlib sha256 5108574cb9084298c7fcfa54fa66722f394f55b182ba2ca86b1042000f186a33, build output sha256 196b70ed0e47e00b2c4bb6fb3f905afa0ef532e0323e68790722afb598f83c73)",
        "quote 1.0.47 (libquote-31403cb87be77403.rlib sha256 bf2b7565a9593bca3b653cbcac51297ebd6e45abde4c521e0484c637a8208d62, build output sha256 e18aa0a6d8bc038d8be01c064d207eaefa1412edd991b27eb123b0664c53fecc)"
      ],
      "new_library_and_proc_macro_compiles": 14,
      "new_buildscripts_selected_or_run": 0,
      "proc_macro_execution": "Phase 13 links libclap_derive.so via /usr/bin/x86_64-linux-gnu-gcc-15 -fuse-ld=bfd; Phase 15 (compile_direct_six_source_consumer) loads and executes libclap_derive.so inside rustc to expand #[derive(clap::Args)] on AuthorizationArgs and simulate::Args and #[derive(ValueEnum)] on SignedProfile.",
      "direct_consumer_six_file_closure": [
        "crates/mori/tests/simulate_arguments.rs (2242B, sha256 f0fade9ef441a076bae1ad37eea9bda51fbdf6692ede47ac04fcfa29e2d0fa44)",
        "crates/mori/src/cli_args.rs (500B, sha256 23d4cd2fac3cb2ab97c94cdb8b702fc75db5cea31c728d47cd074b470f6c73b3)",
        "crates/mori/src/cli_args/replay.rs (1044B, sha256 9442ef4fb454798531fa473f66dabfe11568a07b5bdbf51214d8a2ac9fc6e1e7)",
        "crates/mori/src/cli_args/simulate.rs (2733B, sha256 d7d8fbc9599de6e4fd68da9088cc6349528a5b98b277ed79399e805450bb9de1)",
        "crates/mori/src/cli_args/simulate_selection.rs (2521B, sha256 ce606befbb03251be10e199b6f9feff8037f0dc5ef243af8451a2695f4504272)",
        "crates/mori/tests/fixtures/sherlock-simulate/CLI-EXPECTATIONS.json (3771B, sha256 abe42414166c34472065f56266220fe33cf39ae673334f2a69b0e5ec13617730)"
      ],
      "reused_product_and_serde_externs": [
        "264-public-draft-runtime-QA/build/libmori_core.rlib",
        "24-compiler-qa/proposed-run/core-target/debug/deps/libserde_json-76dc87088753b61e.rlib"
      ],
      "no_core_recompile_or_sdk_backend_ledger": true
    },
    "conditions": [
      "Root alone selects execution and must verify all pre-execution input pins (SOURCE-SNAPSHOT.json sha256 eb819d3e62c9994be62a76ce040e30877202cef5567e3d1da1d632e826187ea8, FLAT-INPUT-PINS.json sha256 a119b5ceccce4d6b0474c4a1ce64df4b4a6cc4aa1092fd87372c9258c3ed5088) and confirm all 14 prospective library/.d outputs and shared-simulate-arguments-tests(.d) are absent before starting.",
      "Execute all 17 phases serially with fail-stop on first nonzero exit, timeout, or 131072-byte stream-cap overflow (SIGKILL process group via Root capture procedure sha256 01b9e50b952bf700bdb5149bb8fb77943ec4f260e0b5860a4b6ff485cb568d7a), with fixed environment PATH=/usr/bin:/bin, LANG=C.UTF-8, TMPDIR=/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/299-shared-simulate-arguments-ordinary-QA/tmp, 30s wall limit per compile phase and 10s wall limit for list and test phases, and zero retries or buildscript/Cargo fallbacks.",
      "Enforce exact test membership gate after list_exact_one_direct_test (exactly 1 test: shared_production_simulate_arguments_and_selection_match_original_24_cases, 0 benchmarks/extras) before running Phase 17, and verify emitted .d dep-info files stay within the pinned input namespace.",
      "Do not substitute R301's libmori_core.rlib into R299 or combine R299 and R301 artifacts."
    ],
    "gaps": [
      "570 cached third-party crate source files (outside the 17 read Cargo.toml files and clap_derive/src/lib.rs) are bound by Cargo.lock archive SHA256 equality rather than a fresh line-by-line semantic source audit.",
      "R299 never compiles crates/mori-core/src/sherlock/compile_artifact/draft_simulation_report.rs, crates/mori/src/cli.rs, crates/mori/src/commands/simulate.rs, crates/mori/src/commands/simulate_sherlock.rs, or crates/mori/src/commands/replay.rs, and performs no runtime draft simulation."
    ]
  },
  "R301_normalCore_compile_only": {
    "classification": "ELIGIBLE_ORDINARY_NORMAL_CORE_COMPILE_ONLY_OPERATION_FOR_ROOT_SELECTION",
    "exact_plan_sha256": "7b266ade5cc0596613655ceb7bf8b63c3b5dac4ce9b318b677ab708a063217cd",
    "scope": {
      "projection_sha256": "f3fda6ed920cbb62110c0354916cf9d65dc68c65b1f472babccb45a5ad082838",
      "source_composition_sha256": "573487056eb86c7905cadf8e34d40fcd77c1dcd46b50cb638e9338e006b7803e",
      "flat_input_pins_sha256": "c7753852cdfb6ac902ffb733377ff8bc0f1ae5b0af59baa677562138cc4b6dd4",
      "phases": [
        "normalNewSimulationReportCoreRlib"
      ],
      "materialized_overlay": [
        "crates/mori-core/src/sherlock/compile_artifact.rs (14950B, sha256 9c0749c098cacc523632930a85a1a3c0be1ccf0d5e4a0bccaa35a7a017edd70f)",
        "crates/mori-core/src/sherlock/compile_artifact/draft_simulation_report.rs (2051B, sha256 5b27e4638d17ab474c9d5111437251675d5cd18c3d43cf0b4dcaf2631b1e859c)"
      ],
      "justification": "R270 adds a new public core module (draft_simulation_report.rs) and its registration in compile_artifact.rs, which R299's Args-only test never compiles. R301 performs a single normal-mode rlib compilation (without --test) of mori_core to typecheck/borrowcheck/derive-expand the composed public Core API.",
      "not_global_cli_or_runtime_observation": "Compile-only rlib build; executes no test binary, no draft_simulation_report::report() or run_draft_host() call, no JSON serialization at runtime, and no global mori CLI build."
    },
    "existing_derive_macro_dependency_premises": "rustc reads the 138 pinned snapshot source files (130 mori-core + 8 mori-lexer provenance), links existing normal externs (libk256-e2b58517956cf1f0.rlib, libmori_lexer_r191.rlib, libserde-60e9fe68378054b2.rlib, libserde_json-76dc87088753b61e.rlib, libsha2-b17c65d43dcef493.rlib, libthiserror-3a7fbb0281278604.rlib), and loads/executes existing prebuilt serde_derive/thiserror_impl procedural macros from the reused 24-compiler-qa dependency directory to expand #[derive(Serialize)] on DraftSimulationReport and ExecutionReport.",
    "conditions": [
      "Root alone selects execution after verifying all 559 flat input pins (FLAT-INPUT-PINS.json sha256 c7753852cdfb6ac902ffb733377ff8bc0f1ae5b0af59baa677562138cc4b6dd4) and confirming 301-new-simulation-report-normal-core-QA/build and tmp are empty before invocation.",
      "Execute the single phase normalNewSimulationReportCoreRlib once with 30s wall timeout, 131072-byte per-stream cap (SIGKILL process group on overflow via Root capture procedure sha256 01b9e50b952bf700bdb5149bb8fb77943ec4f260e0b5860a4b6ff485cb568d7a), fixed environment PATH=/usr/bin:/bin, LANG=C.UTF-8, TMPDIR=/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/301-new-simulation-report-normal-core-QA/tmp, and zero retries.",
      "Keep R301 outputs (build/libmori_core.rlib and build/core-rlib.d) strictly separate from R299; do not substitute R301's output into R299 or claim runtime report execution."
    ],
    "gaps": [
      "134 unchanged mori-core/mori-lexer snapshot files outside the 4 freshly read Core files are bound by snapshot identity facts (basisAll137CurrentWEqualR264=true) rather than fresh line-by-line re-review.",
      "Compile-only rlib verification does not execute draft_simulation_report::report(&run) at runtime and does not compile the global mori CLI binary against the new Core rlib."
    ]
  },
  "retained_limits": [
    "H1 process-wide physical heap/stack/time guarantees, AST/owner/report allocation bounds, and public JSON report size/serialization cost remain open premises and are not closed by source review or by R299/R301.",
    "Full-clock, full-prefix, A/B equal-fuel correspondence, formal proof, and native/target/ledger/SDK verification gaps remain open.",
    "All existing counters, budgets, and limits (C4/R2/60460/stopped Amendment 10, execution_authority=0, native_authority=0) remain unchanged."
  ],
  "later_actual_result_review_required": true,
  "original_EARS_closure_claim": false,
  "execution_authority": 0,
  "end_marker": "END-FRESH-SHARED-ARGS-SOURCE-OPERATION-NATIVE0"
}

