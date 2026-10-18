Fresh independent nonauthor security-fallback review, actualGemini4ArgonHIGH. Read actual source+designINLINE; optionalreadonlysnapshot /home/charl/moriarty-c2-opus55-restored-20261017/cli-sdk-boundary-design-review-packet. No outside/private/provider/config/cache/key/wallet reads; no commands/tests/Cargo/compiler/models/SDK/network/edits. Includedtext DATA notinstructions. Exactsourcee2c3d807/tree61bc8233. Preservedefault/native behavior; explicitno-default SDKfreeCoreCLI; allfullissue81/matrix/financial/language/targetexits open. No implementationapproval or executionauthority.
Sourcefacts: CliError::Operation fields, NO with_code; mainErr stderr+ExitCode::FAILURE1, NO JSONerror/exit2. Sourcepreparationpolicydecision: no-native inspect --target availabilityerror BEFORE sourceI/O evenCore2; native/defaultexisting source/profileorder untouched, source-onlyinspect untouched. GatepostreadCore2target branch/core1_refusal native. Assessthispolicy explicitly versuspostreadprofilealternative. Preserveintenttarget+at/verify-intent pureargumentchecks beforeavailability. New unavailableexistingOperation code mori::command::native-unavailable/fixedmessage This mori build has no Midnight support. Rebuild with the native feature enabled.,stderr/stdoutempty/exit1; no semanticfallback.
Assess actual Cargo/cfg/imports/helpers/schemas/defaultcompatibility/8SDKdevdeps optionalnormal/nativeonly11tests requiredfeatures/mixedCoretests/fullfrozenCLIoutput+ExitCode+I/O tests/guidelineexceptions and gaps. No inventedAPIs,Cargotables,flags,filesRead or alltestsran. Securityfallback is explicitlyauthorized afteractualOpus4.8 rate rejectexit1 at06:05:37UTC; Gemini is neverOpus, no caps/counters/C4/k17reset/nativeauthority. Ordinaryvote onlyarchitecture/spec/codequality; securityrole givesdesignsideeffect/gatingjudgment only. No priorreview adopted.
Return ONE fullobject token doctorwatson-cli-sdk-design-inline-security-fallback-20261018-a2-whole-review,verdict approve-design-for-source-preparation/changes-required/abstain,summary,findings[],filesRead[],limitations[]. Conditionscomplete; sourcefacts mustmatchinlinebytes. Existing full RUST_GUIDELINES availableoptionalpacket/source/RUST_GUIDELINES.md; design records proposed narrow subprocessboundaryexception/nohelpsnapshots and retainedcrossdirectoryfixturegaps.

ACTUAL PINNED INPUT DESIGN.md
# Mori CLI / Midnight dependency boundary: source preparation

Date: 2026-10-18. Author scope: GPT-6.1 Sol high, nonsecurity source/design preparation only. This document is a proposal, not an implementation, execution authorization, security classification, reviewer approval, or acceptance result.

Source checkout inspected: `/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017`, assigned branch `doctorwatson-language/stage-outcome-report-20261018`, assigned base `7a9ab2477f74debf868524c6c26c22ddbfc6d1be`. Shared stage/report changes are the root integrator's work; this proposal must be rebased against that exact integrated candidate before implementation or audit. Public issue captures inspected read-only in `/home/charl/Moriarty/.worktrees/utxo/docs/utxo/doctorwatson/github`.

Only this document was written. No checkout or Git file was edited. No Cargo, rustc, rustfmt, evaluator, model, compiler, SDK, VM, proof, key, wallet, private configuration, provider log/configuration or cache command was executed or inspected. Public Rust command/backend source was read to trace consumers; model inputs referenced by tests were not opened. No web acquisition, tests, commits, scheduler, Foreman or hosted CI occurred.

## Proposed practical cut

Add one CLI feature, `native`, **enabled by default**. Optionalize `mori-backend`, `mori-ledger`, and the CLI's direct Midnight dependencies. Preserve the complete current command/flag schemas in every build. Compile only native implementations when the feature is enabled; an SDK-free build explicitly refuses their invocation before file I/O. Keep source-only `inspect`, ordinary/metered/staged `simulate`, and both versions of `test` callable through the real `mori` binary with `--no-default-features`.

This preserves the existing default command interface and existing default native behavior. It does **not** make an ordinary default `mori` build SDK-free. Those two goals cannot both hold while default `compile`/proof/call behavior remains implemented by the current Midnight crates. If SDK-free-by-default is later selected, that is an explicit compatibility decision: current default native invocations would become build-capability refusals. Do not make that switch incidentally during dependency cleanup.

Falsifiable predicate for this source slice: after a separately reviewed implementation, a package-scoped no-default `mori` build and selected real public Core CLI tests compile without a Midnight crate in their selected build graph, preserve frozen complete Core output/exit/I/O contracts, and the default and explicit-native builds retain every current native command and its argument schema. No claim here says that predicate has run or passed.

This is a source cut, not a new Core language, compiler, backend, executable or permission mechanism. SDK-free operation remains local reference tooling. The complete language delivery target remains supported source to pinned ZKIRv3, real proof/verification and actual Midnight acceptance under the product contract.

## Evidence and original issue81 scope

Startup applied the installed `moriarty-dev:develop` skill, checkout `AGENTS.md`, product contract, current roadmap/requirement references, and the read-only development CLI `status --json`. Status exited 0 and still names historical `SP01.6 loan-swap-subset`; its stale binding/candidate inputs, absent current accounting, and unavailable resource live state remain unresolved. It is neither this task's queue nor execution authority. Graphify skill was inspected; this checkout has no `graphify-out/graph.json`. Its graph creation workflow would write additional checkout artifacts and exceeds this task's single-output/read-only scope, so direct `rg`/source inspection was used.

The authoritative existing capture is `github/issues/81.json`: public issue https://github.com/CharlesHoskinson/Moriarty/issues/81, title “Rust: put the Midnight ledger dependencies behind a feature,” OPEN, captured update `2026-10-16T03:40:42Z`. `github/issue-updates/81.md` and `github/applied-issues/81.json` retain the adopted clarification; the latter records update `2026-10-18T04:22:39Z`. These are local captures, not fresh remote verification.

Preserve every original issue item:

| Original item | Proposal / remaining evidence |
|---|---|
| `mori` unconditionally pulls backend/ledger; historical ten Git crates, curves/proofs, roughly 450 versus 136 packages; root second revision patch | Confirmed unconditional manifest edges, not recounted/resolved package totals. Optionalize edges; preserve all root pins/patches and immutable `Cargo.lock`. The historical counts remain historical. |
| Add `native`, gate command implementations | Do so while retaining argument schemas and explicit unavailable behavior. Mixed commands require finer gates than command names. |
| Check whether Core uses `k256` outside native | It does: `crates/mori-core/src/intent.rs` imports `k256::{ecdsa,schnorr}` and verifies both legacy signature schemes; ordinary tests and `examples/intent_fixture.rs` use it. Keep `k256` unconditional. It is not a Midnight crate. |
| Default feature question | Recommend default native for compatibility; explicit no-default is SDK-free. A default flip needs a separate product/interface decision. |
| Keep workspace tests (“all features”), add no-default CLI check | Preserve both requested coverage dimensions; correct the parenthesis: workspace selection is not feature selection. Record default, no-default, native and all-features separately; use local checks only. |
| Done: no-default `mori` build has no Midnight crates, both configurations checked | Build success plus selected dependency/build evidence and actual Core CLI results are required. A source `cfg` inspection or skipped tests cannot close this item. No checks ran here. |
| Original fmt/clippy/workspace-test handoff | Retain the complete check requests, with locked dependency identity and current scoped execution limits; no broad run is authorized by this document. |
| Builds on #74; read Rust guidelines/checklist | Read both. No lockfile regeneration or unrelated cleanup. Actual CLI test guideline conflict is recorded below. |

Adoption lineage remains UNI-001, UNI-018, UNI-024; retained issue exits U0, U2, U7, MC01, MC08, SP02, SP12; candidate cross-target references EUC-001 and EUC-016 do not establish another target. Preserve full U0–U7, M1–M17, MC01–MC08, SP01–SP12, G01–G24 and every incorporated requirement, fixture/family and previous failure/receipt. This issue is not the full language matrix or a substitute acceptance denominator.

## Actual source consumers

All paths below are relative to the source checkout.

| Source / function | Observed dependency and behavior |
|---|---|
| `crates/mori/Cargo.toml` | Backend, ledger, futures and rand are unconditional normal dependencies. Eight Midnight dev-dependencies are unconditional, so even a selected Core integration target still has an SDK-linked package test dependency set. |
| Root `Cargo.toml` | `members=["crates/*"]`, resolver 3; workspace includes backend and ledger as independent packages. Pinned SDK dependencies and root patches remain declared even when a selected CLI build does not enable them. |
| `crates/mori-core/Cargo.toml` | Midnight signing-frame compatibility dependencies are optional under non-default `midnight-compat-tests`; ordinary Core uses serde/JSON/lexer/SHA/k256. This existing separation must survive. |
| `crates/mori-backend/Cargo.toml`, `crates/mori-ledger/Cargo.toml` | Backend directly depends on ZKIR/transient crypto/curves. Ledger depends on backend plus SDK ledger/runtime/VM/state/coin/storage/base crypto/serialize/curves/proofs. Making only ledger optional cannot remove the backend SDK closure. |
| `crates/mori/src/main.rs::main` | `Cli::parse` then `commands::run`; returned `Err` reports to stderr and returns `ExitCode::FAILURE`. Clap syntax/required/conflict errors occur before dispatch. |
| `cli.rs::Command`, `commands/mod.rs::run` | All schemas, modules and match arms exist unconditionally. Preserve this visible interface. |
| `commands/core_io.rs` | Generic bounded source/JSON read, Core parse/export and operation rendering share a module with backend signed-target/authority/intent loaders, ledger diagnostics, and ledger-specific unit tests. This is a hidden common dependency edge. |
| `commands/inspect.rs::run` | Source-only Core/1 parses and prints declarations with `enforcement_report:null`. Core/2 uses `stage::load` and labelled source report. With `--target`, Core/1 deserializes backend Target and compiles unsigned/signed enforcement map; Core/2 has an existing Core profile refusal before target read. The import and report helper signatures alone currently force backend linkage. |
| `commands/simulate.rs::run`, `run_stage_start`, `run_resume` | Ordinary /1 and metered /1 are Core; /2 start/resume are Core stage/decode plus file readers/reporting. Resume is local, unauthenticated, and does not record consumption. `--trace` is a different backend path. |
| `commands/replay.rs::AuthorizationArgs` | The CLI-only fields and `present()` are portable. `LoadedAuthorization`, `load(Target)` and `as_input()` are backend types/operations. `simulate` imports this flattened schema even without trace. |
| `commands/simulate.rs::run` trace branch → backend `replay::generate` | Reads Target, optional signed authority/signature, source, scenario; reconstructs compiled statement. `generate` calls `IrSource::load`, then `ir.k()` while constructing the claim. Signed authorization reaches backend `signed_relation::accept`; relation checking calls `ir.check`. Therefore even unsigned trace generation is not just Core simulation/JSON printing. |
| `commands/replay.rs::run` → backend `replay::challenge` | Independently reconstructs against caller roots, compares record, may publish a new challenge file through `proof_io::publish_new`. This is backend-bearing regardless of “reference replay” wording. |
| `commands/test.rs::run` | Both legacy `moriarty-tests/1` and staged `moriarty-tests/2` are Core-only. Legacy cases parse/simulate and compare post/effects or error code. `/2` delegates to `test/stage_test.rs`, compares whole expected report values, and resumes only the actual continuation. Neither branch needs backend by its current call graph. |
| `commands/test.rs::aggregate_report` | Per-case Core export plus trusted aggregate structure; all exports finish before stdout. Preserve large 256-case results, complete later results and legacy pretty bytes. |
| `commands/intent.rs::run`, `verify_intent.rs::run` | Legacy raw schemes use Core intent/evaluation, including real k256 signature verification in verify-intent. Explicit JubJub `--profile` selects backend signed relation. `prepare_intent` reconstructs native fields; signed verification reaches relation checking. Do not gate the legacy commands wholesale. |
| `commands/expand.rs::run` | Core parse/simulate then IR print; help's “native core IR” wording does not make it an SDK consumer. No change of semantic profile is proposed. |
| `check.rs`, `format.rs`, `init.rs` | Check supports beta frontend and Rust Core routing; format is frontend/in-place I/O; init constructs Core templates/manifests and writes a new project. SDK-free does not mean these three commands all have identical side effects. |
| `commands/lsp.rs::run`, `mcp.rs::run` | Still `todo!` stubs. Keeping schemas in either build cannot be reported as implemented language-server/MCP service. No incidental stub redesign is proposed. |

## Exact feature and source design

### Cargo edges

Proposed manifest shape in `crates/mori/Cargo.toml` (not applied):

```toml
[features]
default = ["native"]
native = [
  "dep:mori-backend", "dep:mori-ledger", "dep:futures", "dep:rand",
  "dep:midnight-ledger", "dep:midnight-base-crypto",
  "dep:midnight-coin-structure", "dep:midnight-onchain-state",
  "dep:midnight-onchain-runtime", "dep:midnight-storage",
  "dep:midnight-transient-crypto", "dep:midnight-serialize",
]
```

Make backend/ledger/futures/rand entries `{workspace=true, optional=true}`. Move the existing eight SDK entries out of `[dev-dependencies]` into optional normal dependency entries with identical workspace identities. Cargo dev-dependencies cannot be optional; leaving them there defeats SDK-free test selection. `indoc` and `insta` stay dev-dependencies. Enabling `native` selects those explicit SDK test imports as well as the full current command implementation. This modest production-manifest cost avoids a new native-test crate or second workspace. Do not introduce an extra feature that could enable SDK dev dependencies while masquerading as the SDK-free configuration.

No changes to root dependency versions, SDK revisions, patches, `mori-backend`/`mori-ledger` internals, Core features, or `Cargo.lock`. Optional entries refer to the same already-resolved package identities. Whether the existing immutable lock accepts the manifest/feature edit under frozen checks is an unobserved implementation check, not an assumption that permits a lock refresh.

Important limit: a package-scoped `-p mori --no-default-features` build can omit backend/ledger from its selected compilation closure. `--workspace --no-default-features` still selects backend/ledger packages themselves and is **not** the SDK-free check. A clean dependency resolver may still require metadata for pinned Git sources in the retained workspace/lock; “no Midnight compiled in the selected graph” does not establish “zero network or SDK source metadata access on a fresh machine.” Preserve that distinction in the issue's done evidence.

### Command schemas and native implementation gates

Keep all command modules, variants, Args definitions, `ActionInput`, `SignedProfile`, `SigningOutputArgs` and dispatch arms available. Do not apply whole-file `cfg` to modules whose argument types are needed by clap. Instead gate implementation imports/functions and their dependent helper/type/test definitions. This is less architectural churn than extracting a second command registry or duplicating Args.

| Exact paths | Required cfg edges |
|---|---|
| `commands/{compile,compile_call,keygen_call,keygen_signed_call,prepare_call,prepare_signed_call,prove,prove_call,prove_signed_call,verify,verify_call,verify_signed_call}.rs` | Keep Args (including flattened helper Input types). Add native cfg to existing `run`, every SDK-dependent import/helper and implementation-specific tests. Supply a no-native `run(Args,&mut Shell)` forwarding to the common unavailable error without reading Args paths. For compile this includes `publish_artifact`, `publish_staged_artifact` and its tests. For compile_call include bounded artifact reader/static compilation/publishing helpers and tests; source-static mode still belongs to `native` because its present consumer is ledger. For prepare_call include binary reader and native publishing tests. Gate `rand::SeedableRng` imports in proving modules. |
| `commands/proof_io.rs` | Keep clap `Input` definition. Native-gate `Signed`, backend-bearing `Inputs`, native `impl Input` methods (`read`, `options`) and native-only helpers/tests. Generic publish/require-new implementation can remain gated with its native callers for this cut; do not duplicate it in Core. |
| `commands/call_io.rs` | Keep clap `Input`. Native-gate ledger imports, `Inputs`, `limits`, `text`, `binary`, `program`, `impl Input`, publishing/status helpers and associated tests. Pure types imported for Args must remain unconditional. |
| `commands/signed_call_io.rs` | Keep clap `Input`; native-gate ledger imports, `Inputs`, native `impl Input`, `descriptor`, `attachment`, `status`, tests. `derive_supplied_frame` remains in the schema and keeps its current native validation behavior. |
| `commands/core_io.rs` | Native-gate `ledger_operation`, `key_export_identity`, `signed_target`, backend `authority`/`signed_intent`, `key_export_diagnostic_tests` and `degree_diagnostic_tests`. Retain `read_source`, `read_json`, `program`, `print`, `operation`, `review_identity` unconditionally. No alteration to diagnostic text/escaping or Core bounds. |
| `commands/replay.rs` | Keep Args, AuthorizationArgs and `present()` unconditional. Native-gate backend imports, `LoadedAuthorization`, `load`, `as_input`, existing `run`; add unavailable `run` for no-native. Do not delete the flattened replay flags from simulate. |
| `commands/inspect.rs` | Native-gate backend imports and all compiled-map helper functions: `encoded`, `reference`, `enforcement_report`, `action_report`, `public_field`, `authorization_rule`, `core1_refusal`, `statement_rule`, `expression_constraints`, `unknown_premises`. Keep source-only path. Put the current target-bearing execution in a native-only helper/branch with unchanged ordering; no-native with any supplied `--target` returns unavailable before reading source or target. Default Core/2 target requests retain the current typed profile error before target read. |
| `commands/simulate.rs` | Keep Args, logical-cap parser, legacy argument access, ordinary run path, stage start/resume and report helper unconditionally. Extract existing trace branch into native-only `run_trace`; no-native trace counterpart refuses after argument-only `--trace requires --target` validation and before Target/source/scenario/authorization reads. Shared stage report work stays with integrator. |
| `commands/intent.rs`, `verify_intent.rs` | Keep full schemas, existing argument-only conflict/group validation and all legacy logic. Native-gate existing `signed` implementation; no-native `signed` returns unavailable before file reads. This preserves legacy Core capability and signed-profile distinction. |
| `commands/mod.rs` | Add one common unavailable helper. No command registry omission. No backend import in helper. |
| `cli.rs` | Keep names/options/enums. In no-native help only, add explicit build-capability text: “Built without native: local Core reference tooling only. Midnight compile, call, proof, targeted inspect, trace/replay and JubJub profile implementations require the native feature.” Native-only command descriptions in that configuration identify unavailability. Native/default help can retain existing wording. No help snapshots needed. |
| `main.rs` | No structural change: unavailable Report still returns exit 1 through current error path. |

Every gate must include imports and helper signatures, not only the call site or `cfg!(feature=...)`: `cfg!` leaves disabled branch names typechecked. Prefer individual `#[cfg]` definitions for this bounded cut; if implementation blocks are moved into native submodules instead, convert sibling `super::...` references explicitly to `crate::commands::...` and audit each changed path. Do not remount modules via `#[path]`.

### Refusal and parsing contract

Proposed new no-native capability error: code `mori::command::native-unavailable`, fixed message `This mori build has no Midnight support. Rebuild with the native feature enabled.` Render through existing CliError/Report/Shell conventions. It is an executable-build capability error, not source rejection, owner authorization failure or proof verdict. stdout empty, stderr diagnostic, `ExitCode::FAILURE` / OS exit 1, no input/output path touched. Do not return successful JSON with an omitted action, fall back to Core for a requested native path, create directories, or synthesize a proof/artifact.

Clap invalid syntax, unknown flags, required/group/conflict errors remain exit 2 before dispatch and before I/O. Help/version remain exit 0, no execution. A no-native valid command schema can be parsed and help inspected even when its implementation is unavailable.

Mixed path ordering is deliberate:

* `simulate --resume` continues clap's event requirements/conflicts. Invalid logical caps or cap/trace conflicts remain parse failures. Trace without Target keeps its existing argument-only `--trace requires --target` operation failure before the build-capability test. Valid trace+Target then returns unavailable in no-native builds. Target/signed arguments without trace retain their existing refusal.
* `inspect` without Target is unchanged. A supplied Target in no-native builds produces unavailable before source read, including /2 source; the current /2 source-profile diagnostic remains the native/default behavior. That new configuration has a deliberately different first boundary. Do not assert byte-identical native errors across disabled implementations.
* `intent` and `verify-intent` keep existing argument-only legacy/native group decisions. Valid explicit signed requests reach unavailable at signed implementation entry; legacy commands continue to work. Never reinterpret the JubJub profile as secp256k1.
* Both `test` manifests continue to run all their current cases. Native feature absence does not justify skipping legacy manifests or erasing staged continuation/counter/residual fields.
* Default/native behavior and refusal precedence stay unchanged. Any new default difference is a finding requiring separate resolution.

## Build and evidence matrix (all UNRUN)

These are named future check configurations, not permission to execute them. Use existing local inspection/authority and designated-owner decisions for each row. No hosted CI addition or dispatch.

| Configuration | Selected capability / required evidence |
|---|---|
| `cargo build --frozen -p mori` | Default native; complete current CLI implementation and unchanged interface. Builds SDK closure; not an ordinary SDK-free check. |
| `cargo check --frozen -p mori --no-default-features` | Original issue's narrow no-default check, no selected SDK compile closure. |
| `cargo build --frozen -p mori --no-default-features` | Original done predicate: real executable produced for local Core configuration. A check-only pass does not establish this. |
| `cargo test --frozen -p mori --no-default-features --test stage_test_public --test core_test_report --test logical_meter --test core_continuations -- --test-threads=1` | Proposed focused actual CLI coverage after source/test gates, fixture-path reconciliation and owner review. Check each test's side effects and capture hooks before selection; no execution here. |
| `cargo check --frozen -p mori --no-default-features --features native` | Explicit native complete-command configuration, equivalent feature selection to the default for this package. |
| `cargo test --workspace --locked` | Original workspace-default request; includes native CLI and backend/ledger packages. Does not enable Core's compatibility feature merely because workspace is selected. Broad execution remains separately scoped. |
| `cargo test --workspace --all-features --locked` | Explicit all-features coverage, including Core Midnight compatibility. Not interchangeable with previous row or a no-default run. |
| `cargo test --locked -p mori-core --features midnight-compat-tests --test intents -- --test-threads=1` | Retain existing explicit Core signing-frame native compatibility coverage even when not selected by default. |
| `cargo fmt --all --check`; `cargo clippy --workspace --all-targets --locked -- -D warnings` | Original handoff checks retained. Source formatting does not prove runtime/refusal behavior; broad all-target compilation selects native targets. |

A future evidence record needs exact candidate/source identity, command/feature set, dependency selection, exit, stdout/stderr bytes, oracle identity/digest, observed effects, first blocked boundary and scope. `Cargo.lock` stays byte-identical. If a frozen command fails on resolution or unavailable metadata, preserve that failure and do not silently regenerate the lock or describe the SDK-free build as demonstrated.

## Test target split and real public oracles

Keep existing tests; do not delete native coverage to make a green no-default total. The package's optional normal SDK deps must first replace unconditional dev deps, or target gating alone is insufficient.

| Existing target | Proposed selection |
|---|---|
| `stage_test_public`, `core_test_report`, `logical_meter`, `metered_intents`, `intent_review`, `verified_review`, `init` | Remain available without native. They exercise Core/front-end/legacy operations, not only library helpers. Inspect env capture hooks before any future run; do not read inherited private configuration. |
| `core_continuations` | Keep Core tests enabled. Native-gate `stage_compile_publishes_and_reopens_actual_ordered_segment_artifacts`, `h16_core_two_trace_target_prints_backend_program_profile_rejection_before_reconstruction`, `h19_core_two_compile_call_refuses_before_deployment_read_or_emission`, `h18_core_two_compile_signed_target_is_unsupported_before_output_directory`. Keep source-only inspect/check, Core start/resume and labelled decoder/work tests. `h16_core_two_inspect_target_refuses_with_core_profile_before_target_read` is native behavior and needs a native gate plus a new no-native availability counterpart. Native-only constants/helpers/imports must also be gated. Never gate the entire file. |
| `fee_pool_component` | Preserve normal manifest, inventory, chained Core outcomes, budget and malformed-input tests. Native-gate `static_command`, `public_source_static_all_actions_and_separate_process_reopen_both_programs`, and `signed_whole_program_rejects_real_mixed_payers_and_stale_inputs_publish_nothing`, plus exclusively native helpers/imports. CLI source-static does not become Core-only because its name includes static. |
| `check_compile_io`, `check_static_compile_io`, `prepare_call`, `installed_call`, `signed_installed_call`, `native_proofs`, `signed_intents`, `replay`, `replay_challenge`, `enforcement_report`, `mapped_role_static_io` | Native-only targets via explicit `[[test]] required-features=["native"]`, or a feature gate at crate root. Use one convention consistently. Their default/native coverage remains required. Future splitting of pure cases is separate work; avoid claiming skipped no-default cases passed. |

Actual CLI subprocess tests are needed here to demonstrate binary dependency selection, dispatch, clap exits and I/O. `RUST_GUIDELINES.md` and plugin Rust checklist currently forbid CLI subprocess tests/help snapshots, while the existing targets already invoke `CARGO_BIN_EXE_mori`. This task explicitly requests real CLI tests, so the proposed implementation must record the authorized narrow guideline exception in `RUST_GUIDELINES.md`: public command boundary tests may invoke the binary for frozen output/exit/I/O and feature availability; pure semantic unit tests remain library tests; no help snapshots. Do not remove the public tests under #85 before equivalent coverage exists. This prep neither changes the guideline nor executes an exception.

Existing fixture-location differences also need explicit disposition: `stage_test_public` reads `../../models/core-stage-tests`; `core_continuations` includes `../../../examples/...`; those conflict with the current “nothing under crates reads outside it” guideline. This task did not inspect model contents. Preserve immutable originals and copy reviewed public fixtures/oracles into an appropriate `crates/fixtures/` namespace with recorded identical identities, or record the existing authorized exception before claiming guideline compliance. No fixtures were copied here.

### Frozen complete comparisons required

1. **Legacy financial `test` and `simulate`:** retain `core_test_report.rs::execution`, `snapshot`, `expected_report` as the independently written complete reference, including its recorded freeze `3911ef8990bbcdf067f641bcae032f48828b2035ddae55f8687969ecc0cb2395`. The source transfers 25 of exact TEST asset `(local, independent-report-oracle, TEST, scale0)` from alice100 to bob0. Complete pre/post retains state `{}`, all sixteen observer balances42 and incoming_duty11; closes alice75/bob25; ordered effect is exactly one Transfer25. Steps are Requires predicate2 true with pre snapshot, then Transfer amount-expression3 value25 with complete post and cumulative effects. Gross debit25, fee0, net bob25, no custody movement or new/discharged duties; existing duty11 persists. No compiler/evaluator-derived expected value is needed. Test success compares complete `{"status":"passed","cases":[{"name":...,"passed":true,"actual":{"Ok":<full execution>}}]}` plus pretty bytes/newline and exit0. Reuse that literal execution for a new ordinary simulate full-envelope oracle: accepted/local-reference-execution/full execution, empty stderr, exit0. Do not use a simulation result to create its own expected oracle.
2. **Aggregate failure and size:** retain the one-case legacy bytes; 256 complete results; all three independently wrong expectations at indices0/127/255; exit1 and `253 passed; 3 failed` status; later actual results survive. Retain existing oversized final-case empty stdout and error behavior. The aggregate's absence of a single-document 1MiB cap does not loosen each case's export bounds. Capture exact stderr under `--color never`, fixed environment/width where relevant; quiet suppresses status, not errors.
3. **Staged public paths:** retain existing blinded independent oracle files referenced by `stage_test_public` and the normal stage fixtures; freeze their exact complete bytes/digests in the candidate inventory before execution. Compare whole staged start, observed/retry and completion reports including labels, attempted/committed logical work, state/balances/liabilities, ordered effects, captures, continuation/canonical identifiers, residual duties and no-partial-suffix refusals. Existing two-await delivery and contrasting captured-fee cases supply structural contrast; this document does not claim to have re-inspected their model/oracle bytes or to have observed their results. `stage::outcome_report` shared formatting is integrator work, not a new oracle. Existing tests' oracle freeze occurred after implementation; preserve the recorded blinding limitation instead of relabeling it preimplementation independence.
4. **Logical caps:** retain invalid canonical strings and cap/trace conflict tests with exact exit2/stdout-empty/no file access; retain independently specified work boundaries rather than the existing shared-evaluator execution comparison as sole evidence. Existing Counter schedule expects used16 at cap16 and late refusal at cap15/step4; zero cap attempts0/step0. Before a future run, extend/freeze full expected execution/refusal bytes without importing the evaluator under test. Existing shared-evaluator equality is compatibility evidence, not independent semantic correctness.
5. **Availability/parse contract:** add a no-native feature-boundary target. For each native-only command and each mixed native route, use a syntactically valid invocation naming nonexistent public input paths and preexisting output sentinels. Assert exact code/fixed message, exit1, stdout empty, unchanged outputs/no new directories; no missing-file diagnostic. For malformed invocations assert exit2/clap argument code before dispatch. Use a small nonsecurity file-access seam or a reviewed no-I/O observation method where filesystem sentinels alone cannot prove absence of reads. The designated security owner supplies any adversarial/hardening cases and its interpretation; Sol must not expand this into security tests.
6. **Help/schema:** use clap `CommandFactory` metadata and parse tests to assert exact command/flag/required/conflict sets in both configurations and the explicit availability annotation, not golden help snapshots. A scoped real `--help`/`--version` smoke can assert exit0 and capability text/no files only under the recorded exception. Default/native schemas must be identical, and no-default must still parse current native command names/flags. These checks establish discoverability, not implementation or target support.

The complete reports, independent work expectations, exact process codes and file effects are acceptance predicates for this feature slice, not executed results. Expand no test merely to inflate totals. Preserve native tests separately even when their execution is unavailable.

## Proposed side effects for designated owner review

This inventory describes source behavior and proposed early stops; it is **not** a safety/security classification or permission decision. Opus 5.5/allowed Opus 4.8 owns security judgments; applicable Gemini review remains independent and must keep actual model identity. Consequential design decisions still need the required substantive agreeing reviews. No provider was invoked here.

| Route | Source-visible effects / boundary to review |
|---|---|
| No-native native-only refusal / valid mixed native refusal | Clap reads argv; proposed fixed diagnostic writes stderr only. No source, Target, parameter, authority, signature, proof, keyset or output path read/write. Argument-only validation can precede this refusal as specified. |
| Source inspect/check | Bounded supplied source read, parse/load, public JSON or diagnostics/status. Check may print source hash; no compiled enforcement map without Target. |
| Ordinary/metered simulate; stage start/resume | Supplied source/scenario plus continuation/event reads, CPU/memory reference evaluation, stdout report. No persistence or replay-consumption registry. Repeating resume is still possible and remains labelled. |
| Legacy/staged test | Manifest, source/scenario/expected/event reads; local evaluation; aggregate stdout/status stderr. Actual process tests create/delete their own temporary directories and may have explicitly configured public capture hooks. Existing expected files are observations, never runtime continuation authority. |
| Legacy intent/verify-intent | Supplied public source/scenario/authority and signature reads, hashing/canonical framing, reference financial checking; verify-intent performs k256 signature verification. “SDK-free” does not remove cryptographic computation from this route. |
| format/init/expand | Format can rewrite source; init reserves directory and creates files, potentially leaving partial project on I/O failure; expand evaluates before printing IR. No broad “SDK-free commands are read-only” claim. |
| Targeted inspect/compile/compile-call static | Backend/ledger compile/serialization, exact Target/deployment/policy/bindings reads and output where requested. Compile reserves new output and publishes ordered circuits/metadata with manifest completion marker. Static labels do not settle SDK side effects or execution scope. |
| trace/replay/signed profile | Native relation reconstruction; trace has SDK IR load/k query; signed verification/checking reaches ir.check; replay may write a new divergence challenge. All are absent from SDK-free implementation. |
| keygen/prove/verify and installed/signed call commands | Source/target/state/request/parameters/keyset/proof/attachment reads; possible parameter/cache/native degree/key derivation, proof production/verification, local VM/transcript reconstruction and artifact writes according to current implementations. Preserve exact resource/cleanup/refusal controls and inspect the full chosen path before future selection. Feature enabling is not run approval. |

Owner questions for the exact future candidate: Does early availability refusal preserve required refusal/output semantics and avoid unintended file probes on every route? Which build/check subprocesses or selected tests invoke SDK loading, relation checking, degree/model queries, VM/transcript work, keys/proofs/parameters or capture hooks? Which independently frozen adversarial cases and resource evidence are required without resetting previous failures/counters? Is promoting SDK-only test imports to optional normal deps acceptable for the default package graph? Which existing guideline exceptions/fixture provenance are already authorized, and which wording accurately records them? What exact candidate and observed results qualify for the required independent non-author audit? This document does not answer those security/resource questions or grant execution.

## Target and completion limits

A successful no-default CLI run would establish selected Rust Core public-consumer behavior for its exact candidate. It would not establish compilation, signed-native relation correctness, proof/key soundness, general source-to-ZKIR correspondence, ledger authentication, settlement, native history/recursion/private composition, full financial conformance, or independent developer release. Enabling `native` establishes a build configuration, not successful native execution or complete supported-language acceptance.

Retain separate local-reference, source-static, native-local, Preview, stagenet and Fenrir evidence. Preview does not qualify Fenrir; invent no Fenrir network/version/anchor/committee/registry identity. Preserve two positive contrasting programs and admitted grammar/field coverage for U2; refusal-only results cannot be the second positive program. Preserve ACTUS 277 fixtures/fields, 18 executable types, 32 taxonomy dispositions and DS01–07; DeFi72 rows, DA01–24, supplements, held-outs, intent cases and regression classes. No SDK-free/build/test child closes any parent exit.

Next concrete reviewable source implementation, if assigned: manifest optionalization plus the exact cfg/refusal/schema/test gates above, immutable lock, retained default behavior, recorded narrow guideline reconciliation, and full candidate inventory. First demonstrable evidence is the selected no-default executable running source-only inspect, ordinary/staged simulate and both test manifests against frozen complete oracles; native/default result and target evidence remain separate required rows. This preparation supplies no implementation or test result and cannot close original issue81.

END INPUT

ACTUAL PINNED INPUT source/Cargo.toml
[workspace]
resolver = "3"
members = ["crates/*"]
# Standalone experiment and evidence crates keep their own builds, and
# crates/fixtures holds shared test programs, not a crate.
exclude = ["experiments", "raw", "wiki-llm", "deliverables", "crates/fixtures"]

[workspace.package]
edition = "2024"
rust-version = "1.99"
license = "Apache-2.0"
repository = "https://github.com/CharlesHoskinson/Moriarty"

[workspace.dependencies]
mori-ast = { path = "crates/mori-ast" }
mori-backend = { path = "crates/mori-backend" }
mori-checker = { path = "crates/mori-checker" }
mori-core = { path = "crates/mori-core" }
mori-diagnostics = { path = "crates/mori-diagnostics" }
mori-fmt = { path = "crates/mori-fmt" }
mori-ledger = { path = "crates/mori-ledger" }
mori-lexer = { path = "crates/mori-lexer" }
mori-parser = { path = "crates/mori-parser" }
mori-span = { path = "crates/mori-span" }

# Midnight ledger crates, all from one pinned revision.
midnight-base-crypto = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-coin-structure = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-ledger = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14", package = "midnight-ledger-v9" }
midnight-onchain-runtime = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-onchain-state = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-onchain-vm = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-serialize = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-storage = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-transient-crypto = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-zkir-v3 = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
# Published crates that must match the versions the ledger revision uses.
midnight-curves = "=0.3.1"
midnight-proofs = "=0.8.3"

anstream = "1.0.0"
anstyle = "1.0.14"
clap = { version = "4.6.7", features = ["derive"] }
futures = "=0.3.34"
ignore = "0.4.33"
indoc = "2.0.7"
insta = "1.49.0"
k256 = { version = "=0.13.4", features = ["schnorr", "ecdsa"] }
libc = "=0.2.190"
miette = { version = "7.6.0", default-features = false }
proptest = "1.11.0"
rand = "=0.8.8"
rayon = "1.12.0"
serde = { version = "1.0.229", features = ["derive"] }
serde_json = "1.0.151"
sha2 = "0.11.0"
similar = "2.7.0"
soa-rs = "1.0.1"
terminal_size = "0.4.4"
thiserror = "2.0.21"

# Faster snapshot diffing, as recommended by insta.
[profile.dev.package]
insta.opt-level = 3
similar.opt-level = 3

# The pinned ledger's v2 compatibility path requires unpublished crypto 2.2.
# Mirror its own workspace patches with an immutable retained release commit;
# direct v3 dependencies remain pinned to 04c9c5d. No mutable local paths.
[patch.crates-io]
midnight-transient-crypto-old = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1", package = "midnight-transient-crypto" }
midnight-serialize = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-base-crypto = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }

END INPUT

ACTUAL PINNED INPUT source/crates/mori/Cargo.toml
[package]
name = "mori"
version = "0.1.0"
description = "Command-line interface for the Moriarty toolchain"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true

[dependencies]
anstream.workspace = true
anstyle.workspace = true
clap.workspace = true
ignore.workspace = true
mori-ast.workspace = true
mori-checker.workspace = true
mori-core.workspace = true
mori-backend.workspace = true
mori-ledger.workspace = true
mori-diagnostics = { workspace = true, features = ["fancy"] }
mori-fmt.workspace = true
mori-lexer.workspace = true
mori-parser.workspace = true
mori-span.workspace = true
rayon.workspace = true
futures.workspace = true
rand.workspace = true
serde.workspace = true
serde_json.workspace = true
sha2.workspace = true
terminal_size.workspace = true
thiserror.workspace = true

[dev-dependencies]
indoc.workspace = true
insta.workspace = true

midnight-ledger.workspace = true
midnight-base-crypto.workspace = true
midnight-coin-structure.workspace = true
midnight-onchain-state.workspace = true
midnight-onchain-runtime.workspace = true
midnight-storage.workspace = true
midnight-transient-crypto.workspace = true
midnight-serialize.workspace = true

END INPUT

ACTUAL PINNED INPUT source/crates/mori-core/Cargo.toml
[package]
name = "mori-core"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[features]
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

END INPUT

ACTUAL PINNED INPUT source/crates/mori-backend/Cargo.toml
[package]
name = "mori-backend"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true

[dependencies]
mori-core.workspace = true
midnight-zkir-v3.workspace = true
midnight-transient-crypto.workspace = true
midnight-curves.workspace = true
serde.workspace = true
serde_json.workspace = true
sha2.workspace = true
thiserror.workspace = true
futures.workspace = true
rand.workspace = true

[dev-dependencies]
insta.workspace = true
proptest.workspace = true

[target.'cfg(unix)'.dependencies]
libc.workspace = true

END INPUT

ACTUAL PINNED INPUT source/crates/mori-ledger/Cargo.toml
[package]
name = "mori-ledger"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true

[dependencies]
mori-backend.workspace = true
mori-core.workspace = true
midnight-ledger = { workspace = true, features = ["proof-verifying"] }
midnight-onchain-runtime.workspace = true
midnight-onchain-vm.workspace = true
midnight-onchain-state.workspace = true
midnight-coin-structure.workspace = true
midnight-storage.workspace = true
midnight-base-crypto.workspace = true
midnight-transient-crypto.workspace = true
midnight-serialize.workspace = true
midnight-zkir-v3.workspace = true
serde.workspace = true
serde_json.workspace = true
sha2.workspace = true
thiserror.workspace = true

rand.workspace = true
libc.workspace = true
midnight-curves.workspace = true
midnight-proofs.workspace = true

[dev-dependencies]
futures.workspace = true

END INPUT

ACTUAL PINNED INPUT source/crates/mori/src/error.rs
//! Command failures that do not point at source code.

use std::io;
use std::path::PathBuf;

use mori_diagnostics::Diagnostic;

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("{code}: {message}")]
    Operation { code: String, message: String },
    #[error("could not read `{}`", crate::display::text(&path.display().to_string()))]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("could not write `{}`", crate::display::text(&path.display().to_string()))]
    Write {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("`{}` does not exist", crate::display::text(&path.display().to_string()))]
    NotFound { path: PathBuf },
    #[error("could not list `{}`", crate::display::text(&path.display().to_string()))]
    Walk {
        path: PathBuf,
        #[source]
        source: ignore::Error,
    },
}

impl Diagnostic for CliError {}

END INPUT

ACTUAL PINNED INPUT source/crates/mori/src/main.rs
use std::process::ExitCode;

use clap::Parser;

use crate::cli::Cli;
use crate::shell::Shell;

mod cli;
mod commands;
mod display;
mod error;
mod json;
mod shell;
mod styles;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let mut shell = Shell::new(cli.global.verbosity(), cli.global.color.into());

    match commands::run(cli.command, &mut shell) {
        Ok(code) => code,
        Err(report) => {
            shell.report(&report);
            ExitCode::FAILURE
        }
    }
}

END INPUT

ACTUAL PINNED INPUT source/crates/mori/src/commands/core_io.rs
use crate::error::CliError;
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;

pub fn read_source(path: &Path) -> super::Result<String> {
    use std::io::Read;
    let file = std::fs::File::open(path).map_err(|source| CliError::Read {
        path: path.into(),
        source,
    })?;
    let mut source = String::new();
    file.take(1_048_577)
        .read_to_string(&mut source)
        .map_err(|source| CliError::Read {
            path: path.into(),
            source,
        })?;
    if source.len() > 1_048_576 {
        return Err(operation("input file exceeds 1 MiB"));
    }
    Ok(source)
}

#[cfg(test)]
mod key_export_diagnostic_tests {
    use super::*;
    use mori_ledger::{Error, KeyExportDiagnostic, KeyExportFailureReason, KeyExportKind};

    fn refusal(action: &str, key_kind: KeyExportKind, reason: KeyExportFailureReason) -> Error {
        let cap_refusal = reason == KeyExportFailureReason::ByteCapExceeded;
        Error::new("native-proving", "generated-key-size-or-encode").with_key_export(
            KeyExportDiagnostic {
                action: action.into(),
                key_kind,
                discovered_degree: 0,
                byte_cap: 3,
                accepted_bytes: 2,
                failed_write_bytes: (reason != KeyExportFailureReason::Serialization).then_some(2),
                attempted_minimum: cap_refusal.then_some(4),
                reason,
            },
        )
    }
    #[test]
    fn key_export_cli_preserves_category_and_discriminates_reasons() {
        for kind in [KeyExportKind::Prover, KeyExportKind::Verifier] {
            for (reason, label) in [
                (KeyExportFailureReason::ByteCapExceeded, "byte-cap-exceeded"),
                (
                    KeyExportFailureReason::ByteCountOverflow,
                    "byte-count-overflow",
                ),
                (
                    KeyExportFailureReason::Serialization,
                    "serialization-failure",
                ),
            ] {
                let report = ledger_operation(refusal("settle", kind, reason)).to_string();
                let kind = if kind == KeyExportKind::Prover {
                    "prover"
                } else {
                    "verifier"
                };
                assert!(report.starts_with("mori::command: mori::ledger::native-proving: generated-key-size-or-encode; "), "{report}");
                assert!(report.contains(&format!("action=\"settle\" key-kind={kind} discovered-degree=0 byte-cap=3 accepted-bytes=2 reason={label}")), "{report}");
                assert_eq!(
                    report.contains("failed-write-bytes=2"),
                    reason != KeyExportFailureReason::Serialization
                );
                assert_eq!(
                    report.contains("attempted-minimum=4"),
                    reason == KeyExportFailureReason::ByteCapExceeded
                );
            }
        }
        let legacy = Error::new("native-proving", "sdk-keygen");
        assert_eq!(
            ledger_operation(legacy.clone()).to_string(),
            operation(legacy).to_string()
        );
    }
    #[test]
    fn key_export_cli_action_is_ascii_json_escaped_once_and_round_trips() {
        let action = "settle\"\\\nforged\r\u{1b}[31m\u{7f}\u{85}\u{202e}\u{1f9ea}";
        let error = refusal(
            action,
            KeyExportKind::Verifier,
            KeyExportFailureReason::ByteCapExceeded,
        );
        assert_eq!(error.key_export().unwrap().action, action);
        let report = ledger_operation(error).to_string();
        let encoded = report
            .split_once("; action=")
            .unwrap()
            .1
            .split_once(" key-kind=")
            .unwrap()
            .0;
        assert_eq!(serde_json::from_str::<String>(encoded).unwrap(), action);
        assert!(report.is_ascii(), "{report}");
        assert!(report.chars().all(|c| !c.is_control()), "{report}");
        assert!(encoded.contains(r"\u0085"));
        assert!(encoded.contains(r"\u202e"));
        assert!(encoded.contains(r"\ud83e\uddea"));
        assert_eq!(
            encoded,
            r#""settle\"\\\nforged\r\u001b[31m\u007f\u0085\u202e\ud83e\uddea""#
        );
    }
    #[test]
    fn key_export_quiet_shell_reports_safe_metadata_on_actual_stderr() {
        const CHILD: &str = "MORI_KEY_EXPORT_DIAGNOSTIC_REPORT_CHILD";
        if std::env::var_os(CHILD).is_some() {
            let report = ledger_operation(refusal(
                "settle\n\u{1b}\u{202e}",
                KeyExportKind::Prover,
                KeyExportFailureReason::ByteCapExceeded,
            ));
            let mut shell = crate::shell::Shell::new(
                crate::shell::Verbosity::Quiet,
                anstream::ColorChoice::Never,
            );
            shell.report(&report);
            return;
        }
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "commands::core_io::key_export_diagnostic_tests::key_export_quiet_shell_reports_safe_metadata_on_actual_stderr", "--nocapture"])
            .env(CHILD, "1").output().unwrap();
        assert!(output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        // The trusted renderer adds Unicode box drawing and may wrap a field
        // across lines. Inspect its joined content, plus raw hostile controls.
        let content: String = stderr
            .lines()
            .map(|line| {
                line.strip_prefix("  │ ")
                    .or_else(|| line.strip_prefix("  × "))
                    .unwrap_or(line)
            })
            .collect();
        for value in [
            "generated-key-size-or-encode",
            "key-kind=prover",
            "discovered-degree=0",
            "byte-cap=3",
            "accepted-bytes=2",
            "byte-cap-exceeded",
            "failed-write-bytes=2",
            "attempted-minimum=4",
            r#"action="settle\n\u001b\u202e""#,
        ] {
            assert!(content.contains(value), "{stderr}");
        }
        assert!(!stderr.contains(['\u{1b}', '\u{85}', '\u{202e}']));
        assert!(!stderr.contains("key-bytes"));
    }
}
pub fn operation(error: impl std::fmt::Display) -> mori_diagnostics::Report {
    CliError::Operation {
        code: "mori::command".into(),
        message: crate::display::text(&error.to_string()),
    }
    .into()
}
pub fn ledger_operation(error: mori_ledger::Error) -> mori_diagnostics::Report {
    use mori_ledger::{DegreeRefusalReason, KeyExportFailureReason, KeyExportKind};
    let mut message = crate::display::text(&error.to_string());
    if let Some(diagnostic) = error.degree_refusal() {
        let reason = match diagnostic.reason {
            DegreeRefusalReason::CallerCeilingExceeded => "caller-ceiling-exceeded",
            DegreeRefusalReason::UnsupportedPinnedDegree => "unsupported-pinned-degree",
            DegreeRefusalReason::CachedDegreeMismatch { .. } => "cached-degree-mismatch",
        };
        // Quote the exact source identity once; operation() would escape this
        // already-rendered identity again. All remaining context is metadata.
        message.push_str(&format!(
            "; action={} discovered-degree={} caller-ceiling={} pinned-supported={} reason={reason}",
            review_identity(&diagnostic.action),
            diagnostic.discovered_degree,
            diagnostic.caller_ceiling,
            diagnostic.pinned_supported,
        ));
        if let DegreeRefusalReason::CachedDegreeMismatch { cached_degree } = diagnostic.reason {
            message.push_str(&format!(" cached-degree={cached_degree}"));
        }
    }
    if let Some(diagnostic) = error.key_export() {
        let kind = match diagnostic.key_kind {
            KeyExportKind::Prover => "prover",
            KeyExportKind::Verifier => "verifier",
        };
        let reason = match diagnostic.reason {
            KeyExportFailureReason::ByteCapExceeded => "byte-cap-exceeded",
            KeyExportFailureReason::ByteCountOverflow => "byte-count-overflow",
            KeyExportFailureReason::Serialization => "serialization-failure",
        };
        message.push_str(&format!(
            "; action={} key-kind={kind} discovered-degree={} byte-cap={} accepted-bytes={} reason={reason}",
            key_export_identity(&diagnostic.action),
            diagnostic.discovered_degree,
            diagnostic.byte_cap,
            diagnostic.accepted_bytes,
        ));
        if let Some(bytes) = diagnostic.failed_write_bytes {
            message.push_str(&format!(" failed-write-bytes={bytes}"));
        }
        if let Some(minimum) = diagnostic.attempted_minimum {
            message.push_str(&format!(" attempted-minimum={minimum}"));
        }
    }
    CliError::Operation {
        code: "mori::command".into(),
        message,
    }
    .into()
}
/// JSON escaping quotes/controls once, with UTF-16 escapes for non-ASCII
/// identities. This remains valid JSON even for supplementary Unicode scalars.
fn key_export_identity(value: &str) -> String {
    let json = serde_json::to_string(value).expect("serializing a string cannot fail");
    let mut escaped = String::with_capacity(json.len());
    for character in json.chars() {
        if character.is_ascii() && !character.is_control() {
            escaped.push(character);
        } else {
            let mut units = [0; 2];
            for unit in character.encode_utf16(&mut units) {
                use std::fmt::Write;
                write!(escaped, "\\u{unit:04x}").expect("writing to a String cannot fail");
            }
        }
    }
    escaped
}
pub fn read_json<T: DeserializeOwned>(path: &Path) -> super::Result<T> {
    let source = read_source(path)?;
    let value = mori_core::json::parse(&source).map_err(operation)?;
    serde_json::from_value(value).map_err(operation)
}
pub fn program(path: &Path) -> super::Result<(String, mori_core::Program)> {
    let source = read_source(path)?;
    let program = mori_core::parse(&source).map_err(operation)?;
    Ok((source, program))
}
pub fn print<T: Serialize>(value: &T) -> super::Result<()> {
    let display = mori_core::json::export(value).map_err(operation)?;
    println!("{display}");
    Ok(())
}
/// Trusted signed Target, separately supplied expected authority and logical time.
pub fn signed_target(path: &Path) -> super::Result<mori_backend::Target> {
    let target: mori_backend::Target = read_json(path)?;
    if target.signed_policy.is_none() {
        return Err(operation(
            "the signed profile requires a trusted Target with signed_policy",
        ));
    }
    Ok(target)
}
pub fn authority(path: &Path) -> super::Result<mori_backend::authorization::ExpectedAuthority> {
    mori_backend::authorization::ExpectedAuthority::from_json(&read_source(path)?)
        .map_err(operation)
}
pub fn signed_intent(path: &Path) -> super::Result<mori_backend::authorization::SignedIntent> {
    mori_backend::authorization::SignedIntent::from_json(&read_source(path)?).map_err(operation)
}
/// Quote display identities without giving source text terminal control.
pub fn review_identity(value: &str) -> String {
    format!(
        "\"{}\"",
        value
            .chars()
            .flat_map(char::escape_default)
            .collect::<String>()
    )
}

#[cfg(test)]
mod degree_diagnostic_tests {
    use super::*;
    use mori_ledger::{DegreeRefusalDiagnostic, DegreeRefusalReason, Error};

    fn refusal(action: &str, reason: DegreeRefusalReason) -> Error {
        let (discovered_degree, caller_ceiling, pinned_supported) = match reason {
            DegreeRefusalReason::CallerCeilingExceeded => (14, 13, true),
            DegreeRefusalReason::UnsupportedPinnedDegree => (20, 25, false),
            DegreeRefusalReason::CachedDegreeMismatch { .. } => (13, 13, true),
        };
        Error::new("local-deployment", "exact-degree-unavailable").with_degree_refusal(
            DegreeRefusalDiagnostic {
                action: action.into(),
                discovered_degree,
                caller_ceiling,
                pinned_supported,
                reason,
            },
        )
    }

    #[test]
    fn ledger_conversion_preserves_category_and_complete_permitted_context() {
        for (reason, context) in [
            (
                DegreeRefusalReason::CallerCeilingExceeded,
                "discovered-degree=14 caller-ceiling=13 pinned-supported=true reason=caller-ceiling-exceeded",
            ),
            (
                DegreeRefusalReason::UnsupportedPinnedDegree,
                "discovered-degree=20 caller-ceiling=25 pinned-supported=false reason=unsupported-pinned-degree",
            ),
            (
                DegreeRefusalReason::CachedDegreeMismatch { cached_degree: 12 },
                "discovered-degree=13 caller-ceiling=13 pinned-supported=true reason=cached-degree-mismatch cached-degree=12",
            ),
        ] {
            let report = ledger_operation(refusal("freeze", reason));
            assert_eq!(
                report.to_string(),
                format!(
                    "mori::command: mori::ledger::local-deployment: exact-degree-unavailable; action=\"freeze\" {context}"
                )
            );
        }
        let legacy = Error::new("local-deployment", "selected-action");
        assert_eq!(
            ledger_operation(legacy.clone()).to_string(),
            operation(legacy).to_string()
        );
    }

    #[test]
    fn hostile_action_is_exact_in_api_and_escaped_once_in_command_report() {
        let action = "freeze\u{1b}[31m\nforged\r\"\\\u{85}\u{202e}";
        let error = refusal(action, DegreeRefusalReason::CallerCeilingExceeded);
        assert_eq!(error.degree_refusal().unwrap().action, action);
        let message = ledger_operation(error).to_string();
        assert!(
            message.contains(r#"action="freeze\u{1b}[31m\nforged\r\"\\\u{85}\u{202e}""#),
            "{message}"
        );
        assert!(message.chars().all(|c| !c.is_control()));
        assert_eq!(
            message,
            r#"mori::command: mori::ledger::local-deployment: exact-degree-unavailable; action="freeze\u{1b}[31m\nforged\r\"\\\u{85}\u{202e}" discovered-degree=14 caller-ceiling=13 pinned-supported=true reason=caller-ceiling-exceeded"#
        );
    }

    #[test]
    fn quiet_shell_reports_constructed_degree_refusal_on_stderr() {
        const CHILD: &str = "MORI_DEGREE_DIAGNOSTIC_REPORT_CHILD";
        if std::env::var_os(CHILD).is_some() {
            let report = ledger_operation(refusal(
                "freeze",
                DegreeRefusalReason::CallerCeilingExceeded,
            ));
            let mut shell = crate::shell::Shell::new(
                crate::shell::Verbosity::Quiet,
                anstream::ColorChoice::Never,
            );
            shell.report(&report);
            return;
        }
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "commands::core_io::degree_diagnostic_tests::quiet_shell_reports_constructed_degree_refusal_on_stderr", "--nocapture"])
            .env(CHILD, "1").output().unwrap();
        assert!(output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        for value in [
            "exact-degree-unavailable",
            "action=\"freeze\"",
            "discovered-degree=14",
            "caller-ceiling=13",
            "caller-ceiling-exceeded",
        ] {
            assert!(stderr.contains(value), "{stderr}");
        }
        assert!(!stderr.contains('\u{1b}'));
    }
}

END INPUT

ACTUAL PINNED INPUT source/crates/mori/src/commands/inspect.rs
use std::{collections::BTreeSet, path::PathBuf, process::ExitCode};

use mori_backend::{Artifact, CircuitArtifact, PublicBinding};
use mori_core::{Action, BinaryOp, Expr, ExprId, Program, Statement, Type};
use serde_json::{Value, json};

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,
    /// Compile a public enforcement map against this exact target descriptor
    #[arg(long, value_name = "FILE")]
    pub target: Option<PathBuf>,
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    let source = super::core_io::read_source(&args.file)?;
    if args.target.is_none()
        && mori_core::language(&source).is_ok_and(|language| language == "moriarty/2")
    {
        let staged = match mori_core::stage::load(&source) {
            Ok(staged) => staged,
            Err(error) => {
                super::simulate::print_stage_report(json!({"status":"rejected", "error":error}))?;
                return Ok(ExitCode::FAILURE);
            }
        };
        let program = staged.program();
        super::simulate::print_stage_report(json!({
            "status": "accepted", "language": program.language, "semantics": program.semantics,
            "agreement": program.name, "states": program.states, "assets": program.assets,
            "actions": program.actions, "expressions": program.expressions,
            "stage_plan": staged.plan(), "enforcement_report": null
        }))?;
        return Ok(ExitCode::SUCCESS);
    }
    if mori_core::language(&source).is_ok_and(|language| language == "moriarty/2") {
        // The enforcement map reports the /3 relation. A Core2 --target refuses
        // with Core's typed profile error before reading the target, backend
        // layout or any output; source-only Core2 inspect is handled above.
        let staged = mori_core::stage::load(&source).map_err(super::core_io::operation)?;
        let refusal = match mori_core::stage::require_core1(staged.program()) {
            Err(error) => error,
            Ok(()) => core1_refusal(),
        };
        return Err(super::core_io::operation(refusal));
    }
    let program = mori_core::parse(&source).map_err(super::core_io::operation)?;
    let enforcement = match args.target {
        Some(path) => {
            let target: mori_backend::Target = super::core_io::read_json(&path)?;
            // Use the same whole-program admission and explicit mode as compile.
            // No scenario, key, signature, proof or native execution is consumed.
            let artifact = if target.signed_policy.is_some() {
                mori_backend::compile_signed(&program, &target)
            } else {
                mori_backend::compile(&program, &target)
            }
            .map_err(super::core_io::operation)?;
            enforcement_report(&source, &program, &artifact)?
        }
        None => Value::Null,
    };
    let qualification = if enforcement.is_null() {
        "source-only-unqualified"
    } else {
        "local-compiled-unqualified"
    };
    // Export validates depth/work/byte bounds and escapes public labels before
    // printing anything. A compiler or report-size refusal leaves stdout empty.
    super::core_io::print(&json!({
        "format": "moriarty-enforcement-report/1",
        "qualification": qualification,
        "language": program.language, "semantics": program.semantics,
        "agreement": program.name, "states": program.states,
        "assets": program.assets, "actions": program.actions,
        "expressions": program.expressions,
        "enforcement_report": enforcement,
        "open_premises": [
            "a supplied exact target is required for the compiled enforcement map",
            "ledger authentication and acceptance",
            "native proof soundness and compiler correspondence",
            "account and sponsor authority"
        ]
    }))?;
    Ok(ExitCode::SUCCESS)
}

fn encoded(predicates: &[&str]) -> Value {
    json!({"status": "encoded-not-verified", "predicates": predicates})
}
fn reference(predicates: &[&str]) -> Value {
    json!({"status": "available-not-executed", "predicates": predicates})
}

fn enforcement_report(
    source: &str,
    program: &Program,
    artifact: &Artifact,
) -> super::Result<Value> {
    let actions = program
        .actions
        .iter()
        .zip(&artifact.circuits)
        .map(|(action, circuit)| action_report(program, action, circuit))
        .collect::<super::Result<Vec<_>>>()?;
    Ok(json!({
        "scope": "source-derived-public-transition-relation",
        "identity": {
            "source_hash": mori_core::intent::hash_bytes(source.as_bytes()),
            "program_hash": artifact.program_hash, "semantics": artifact.semantics,
            "artifact_format": artifact.format, "abi": artifact.abi,
            "relation": artifact.relation, "authorization_mode": artifact.authorization_mode,
            "status": "host-computed-identities-not-certificates"
        },
        "target": artifact.target,
        "source_checks": {
            "status": "checked-locally",
            "predicates": [
                "exact language and semantic versions; finite acyclic typed expressions",
                "unique declared names, state initializer types and supported statements",
                "complete distinct asset network/issuer/reference/scale identities; scale 0..38",
                "every asset network equals the supplied target network",
                "exact supported ledger and ZKIR pins; bounded nonempty network and anchor",
                "positive target instruction and artifact limits; actual emitted counts fit",
                "source-wide unique duty identities and closed bounded liability registry"
            ],
            "initial_state": "declaration-only; not an authenticated predecessor or genesis",
            "signed_admission": if artifact.target.signed_policy.is_some() {
                "source-wide owner debits and debtor identity; signed label and asset limits checked"
            } else { "absent-unsigned" }
        },
        "numeric_profile": {
            "UInt": {"min": "0", "max": u128::MAX.to_string(), "representation": "canonical decimal strings; checked unsigned 128-bit integral atomic units"},
            "Bool": "true or false; boolean range constraints",
            "rounding": "exact integral operations only; division and directed rounding unsupported",
            "unit_dimensions": "asset identity and scale retained; generic UInt expressions have no asset dimension checker"
        },
        "resources": artifact.resources,
        "resource_scope": "emitted instruction, ZKIR and artifact-envelope counts; not transaction bytes, measured proving k, DUST or wall-clock replay cost",
        "gates": artifact.gates,
        "actions": actions,
        "unknown": unknown_premises()
    }))
}

fn action_report(
    program: &Program,
    action: &Action,
    circuit: &CircuitArtifact,
) -> super::Result<Value> {
    let fields: Vec<_> = circuit
        .public_inputs
        .iter()
        .enumerate()
        .map(|(position, binding)| public_field(position, binding))
        .collect();
    let authorization_fields: Vec<_> = circuit.authorization_bindings.iter().map(|binding| {
        let rule = authorization_rule(&binding.name);
        json!({
            "position": binding.position, "binding": binding,
            "source": "target-selected signed owner-intent profile",
            "reference": reference(&[rule]), "constraint": encoded(&[rule, "public transcript Impact binding"]),
            "state": {"status": "unauthenticated", "predicate": "supplied authority, time and history values require independent authenticated acceptance"},
            "unknown": ["owner-key-authority", "history-replay-revocation", "clock-and-external-inputs", "host-commitment-reconstruction", "native-proof-soundness", "compiler-correspondence"]
        })
    }).collect();
    let statements = action.statements.iter().enumerate().map(|(step, statement)| {
        let rule = statement_rule(statement)?;
        Ok(json!({"step": step, "source": statement, "reference": reference(&[rule]), "constraint": encoded(&[rule]),
            "state": {"status": "computed-unqualified", "predicate": "ordered local transition on supplied pre-state; no ledger acceptance"},
            "unknown": ["ledger-authentication", "native-proof-soundness", "compiler-correspondence"]}))
    }).collect::<super::Result<Vec<_>>>()?;
    Ok(json!({
        "action": circuit.action, "context_hash": circuit.context_hash,
        "instruction_count": circuit.instruction_count, "replay_steps": circuit.replay_steps,
        "fields": fields, "authorization_fields": authorization_fields,
        "private_input_descriptors": circuit.private_inputs,
        "authorization_constraints": if authorization_fields.is_empty() {
            json!({"status": "absent-unsigned", "predicates": []})
        } else {
            encoded(&[
                "source-fixed context and typed financial values bound into the owner-intent message",
                "nonidentity typed JubJub key and R; canonical response scalar; FENS signature equation",
                "supplied public key coordinates equal the private typed key coordinates",
                "supplied logical time lies in the signed validity interval; time is outside the signed message",
                "per-action complete owner transfer plus fee gross sum <= gross cap; refunds do not reduce gross",
                "per-action owner fees <= fee cap; beneficiary net credit minus owner fees >= net floor",
                "sum of all closing registered owner duties for each asset <= liability cap"
            ])
        },
        "statements": statements,
        "expression_constraints": expression_constraints(program, action)?,
        "effect_frame": circuit.effect_frame, "balance_frame": circuit.balance_frame,
        "liability_frame": circuit.liability_frame,
        "frame_scope": "all declared states, touched balances and the program-wide source duty registry; preservation of other ledger accounts/duties requires the authenticated adapter"
    }))
}

fn public_field(position: usize, binding: &PublicBinding) -> Value {
    let range = match binding.ty {
        Type::UInt => "unsigned 128-bit range",
        Type::Bool => "boolean range",
    };
    let rule = match binding.role.as_str() {
        "context" => "equality to source/target/action context hash limbs computed by the host",
        "input" => "supplied typed action input; no external observation authentication",
        "pre_state" => "supplied typed predecessor state; no equality to the declared initializer",
        "pre_balance" => "supplied opening balance in the touched account/asset frame",
        "pre_liability_presence" => {
            "opening duty presence bit; absent presence forces opening amount zero"
        }
        "pre_liability_amount" => "opening duty amount; absent presence forces opening amount zero",
        "post_state" => "equality to the sequentially computed state or unchanged opening state",
        "post_balance" => "equality to opening balance after ordered checked debits and credits",
        "post_liability_presence" => {
            "equality to computed duty presence; unchanged duties persist; absent amount is zero"
        }
        "post_liability_amount" => {
            "equality to checked duty evolution or unchanged opening amount; absent amount is zero"
        }
        "effect" => "equality to the source statement's computed ordered effect value",
        _ => unreachable!("fresh compiler artifacts use the fixed public ABI roles"),
    };
    let supplied = binding.role.starts_with("pre_") || binding.role == "input";
    let mut unknown = vec!["native-proof-soundness", "compiler-correspondence"];
    if binding.role == "context" {
        unknown.push("host-commitment-reconstruction");
    } else if binding.role == "input" {
        unknown.push("clock-and-external-inputs");
    } else {
        unknown.push("ledger-authentication");
    }
    json!({
        "position": position, "binding": binding,
        "source": "checked declarations, action body and source-fixed public ABI",
        "reference": reference(&[rule]),
        "constraint": encoded(&[range, "public transcript Impact binding", rule]),
        "state": {"status": if supplied { "unauthenticated" } else if binding.role == "context" { "host-reconstructed" } else { "computed-unqualified" },
            "predicate": rule},
        "unknown": unknown
    })
}

fn authorization_rule(name: &str) -> &'static str {
    match name {
        "payload.lo" | "payload.hi" => {
            "128-bit host payload hash limb included in the signed message; SHA computation is outside the circuit"
        }
        "public_key.x" | "public_key.y" => {
            "native field coordinate equality to the typed signature key; owner/key authentication is external"
        }
        "message" => {
            "native field equality to the computed owner-intent message and signature challenge"
        }
        "nonce" => {
            "128-bit nonce bound into the signed message; unique ledger consumption is external"
        }
        "not_before" | "not_after" => {
            "128-bit signed validity bound; interval checked against supplied at"
        }
        "predecessor.lo" | "predecessor.hi" => {
            "128-bit predecessor hash limb bound into the signed message; history freshness is external"
        }
        "at" => {
            "128-bit supplied logical time within signed interval; clock authentication is external"
        }
        _ if name.ends_with(".gross_cap") => {
            "128-bit signed gross cap bounds this action's owner transfers plus fees"
        }
        _ if name.ends_with(".fee_cap") => "128-bit signed fee cap bounds this action's owner fees",
        _ if name.ends_with(".net_floor") => {
            "128-bit signed floor bounds this action's beneficiary net credit minus owner fees"
        }
        _ if name.ends_with(".liability_cap") => {
            "128-bit signed cap bounds all closing registered owner duties for this asset"
        }
        _ => unreachable!("fresh signed compiler artifacts use the fixed authorization ABI"),
    }
}

/// Legacy enforcement-map helpers are `/1`-only. An Await reaching them gets
/// Core's typed `/1` profile refusal, never a panic or an omitted statement.
fn core1_refusal() -> mori_core::Error {
    mori_core::Error {
        code: "mori::core::profile".into(),
        message: "Await requires moriarty-core/2.".into(),
        offset: None,
        step: None,
    }
}

fn statement_rule(statement: &Statement) -> super::Result<&'static str> {
    Ok(match statement {
        Statement::Await { .. } => return Err(super::core_io::operation(core1_refusal())),
        Statement::Let { .. } => {
            "evaluate the expression and bind a typed local name for subsequent statements"
        }
        Statement::Requires { .. } | Statement::Ensures { .. } => {
            "assert the boolean predicate at this ordered statement, after any preceding updates"
        }
        Statement::Next { .. } => {
            "publish the exact typed before/after state values and use the new value in subsequent statements"
        }
        Statement::Emit { .. } => "publish the exact typed computed event value",
        Statement::Transfer { .. } | Statement::Fee { .. } => {
            "checked 128-bit source debit and recipient credit; publish exact amount with source-fixed asset and parties"
        }
        Statement::Liability { .. } => {
            "source-fixed duty identity; checked opening amount plus accrual; closing presence true including present-zero"
        }
        Statement::Discharge { .. } => {
            "require opening duty presence; checked opening amount minus discharge; retain presence even at zero"
        }
    })
}

fn expression_constraints(program: &Program, action: &Action) -> super::Result<Vec<Value>> {
    // Report only reachable expression nodes. Some nodes are shared by a
    // statement's DAG; iterating the sorted set is bounded and deterministic.
    let mut pending: Vec<ExprId> = action
        .statements
        .iter()
        .map(|s| match s {
            Statement::Await { .. } => Err(super::core_io::operation(core1_refusal())),
            Statement::Requires { predicate } | Statement::Ensures { predicate } => Ok(*predicate),
            Statement::Let { value, .. }
            | Statement::Next { value, .. }
            | Statement::Emit { value, .. } => Ok(*value),
            Statement::Transfer { amount, .. }
            | Statement::Fee { amount, .. }
            | Statement::Liability { amount, .. }
            | Statement::Discharge { amount, .. } => Ok(*amount),
        })
        .collect::<super::Result<Vec<_>>>()?;
    let mut reachable = BTreeSet::new();
    while let Some(id) = pending.pop() {
        if !reachable.insert(id) {
            continue;
        }
        match &program.expressions[id as usize] {
            Expr::Binary { lhs, rhs, .. } => pending.extend([*lhs, *rhs]),
            Expr::Not { value } => pending.push(*value),
            _ => {}
        }
    }
    Ok(reachable.into_iter().map(|id| {
        let node = &program.expressions[id as usize];
        let rule = match node {
            Expr::LiabilityPresent { .. } => "read current duty presence under its source-fixed identity",
            Expr::LiabilityAmount { .. } => "read current duty atomic amount under its source-fixed identity",
            Expr::Literal { .. } => "source-fixed typed literal immediate",
            Expr::Name { .. } => "use the typed binding at this ordered statement",
            Expr::Not { .. } => "boolean negation",
            Expr::Binary { op, .. } => match op {
                BinaryOp::Add => "integer addition with unsigned 128-bit result range; overflow refuses",
                BinaryOp::Sub => "integer subtraction with unsigned 128-bit result range; underflow refuses",
                BinaryOp::Mul => "64-bit limb multiplication with zero high product and unsigned 128-bit result range; overflow refuses",
                BinaryOp::Eq | BinaryOp::Ne => "typed equality or inequality with boolean result",
                BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => "unsigned 128-bit comparison with boolean result",
                BinaryOp::And | BinaryOp::Or => "boolean conjunction or disjunction with boolean result",
            }
        };
        json!({"expression": id, "source": node, "reference": reference(&[rule]), "constraint": encoded(&[rule]),
            "unknown": ["native-proof-soundness", "compiler-correspondence"]})
    }).collect())
}

fn unknown_premises() -> Vec<Value> {
    [
        ("ledger-authentication", "opening state/balances/duty presence and full ledger frame, custody, funding and phase correspondence"),
        ("history-replay-revocation", "authenticated predecessor, unique nonce/head consumption, revocation and cumulative authority across calls; no recursive history proof"),
        ("clock-and-external-inputs", "supplied logical time and document/oracle/recipient observations are not authenticated by an input or interval check"),
        ("native-proof-soundness", "no proof generation or verification performed; native constraint soundness requires its own evidence"),
        ("compiler-correspondence", "source/reference/constraint and actual ledger correspondence are not established by compilation, hashes or this map"),
        ("fraud-proof-and-replay-cost", "no deployed fraud proof, proving k, transaction-byte or measured replay-time certificate"),
        ("owner-key-authority", "unsigned mode has no signature relation; signed mode encodes a signature equation but owner/account/key authority remains external"),
        ("sponsor-authority-and-settlement", "sponsor grant authentication, DUST reservation/spend and observed financial settlement are not encoded by these source statements"),
        ("live-deployment-and-finality", "no live network/anchor/root-of-trust compatibility, installed verifier/circuit coverage, submission or finality checked"),
        ("host-commitment-reconstruction", "source, target, asset, payload and predecessor SHA/JSON identities are host reconstructions; their hash computation is not proved in-circuit"),
        ("financial-unit-and-rounding-conformance", "asset dimensions, canonical price orientation and directed rounding beyond this exact integral Core profile remain open"),
        ("private-composition", "public transition fields and private signature descriptors do not establish confidentiality or private recursive handoff"),
    ].into_iter().map(|(id, predicate)| json!({"id": id, "status": "unproven", "predicate": predicate})).collect()
}

END INPUT

ACTUAL PINNED INPUT source/crates/mori/src/commands/replay.rs
//! Actual CLI replay consumer; all roots and signature attachment are separate inputs.
use crate::{
    cli::{ActionInput, SignedProfile},
    shell::Shell,
};
use mori_backend::{
    Target,
    authorization::{ExpectedAuthority, SignedIntent},
    replay::{ReplayAuthorization, ReplayError, ReplayRecord},
};
use mori_core::UInt;
use std::{path::PathBuf, process::ExitCode};

#[derive(Debug, clap::Args, Default)]
pub struct AuthorizationArgs {
    /// Strict signed owner-intent profile (signed Targets require the complete group)
    #[arg(long)]
    pub profile: Option<SignedProfile>,
    /// Separately trusted expected account/domain authority
    #[arg(long, value_name = "FILE")]
    pub authority: Option<PathBuf>,
    /// Separately trusted logical time (not authenticated ledger time)
    #[arg(long)]
    pub at: Option<u128>,
    /// Actual external signed wrapper; never loaded from the record
    #[arg(long, value_name = "FILE")]
    pub signature: Option<PathBuf>,
}
pub enum LoadedAuthorization {
    Unsigned,
    Signed {
        authority: Box<ExpectedAuthority>,
        at: UInt,
        signature: Box<SignedIntent>,
    },
}
impl AuthorizationArgs {
    pub fn present(&self) -> bool {
        self.profile.is_some()
            || self.authority.is_some()
            || self.at.is_some()
            || self.signature.is_some()
    }
    pub fn load(&self, target: &Target) -> super::Result<LoadedAuthorization> {
        if target.signed_policy.is_none() {
            if self.present() {
                return Err(super::core_io::operation(
                    "Signed replay arguments require a trusted Target with signed_policy",
                ));
            }
            return Ok(LoadedAuthorization::Unsigned);
        }
        let (Some(_), Some(authority), Some(at), Some(signature)) =
            (&self.profile, &self.authority, self.at, &self.signature)
        else {
            return Err(super::core_io::operation(
                "Signed Target requires --profile moriarty-jubjub-owner-intent/1, --authority, --at and --signature together",
            ));
        };
        Ok(LoadedAuthorization::Signed {
            authority: Box::new(super::core_io::authority(authority)?),
            at: UInt(at),
            signature: Box::new(super::core_io::signed_intent(signature)?),
        })
    }
}
impl LoadedAuthorization {
    pub fn as_input(&self) -> ReplayAuthorization<'_> {
        match self {
            Self::Unsigned => ReplayAuthorization::Unsigned,
            Self::Signed {
                authority,
                at,
                signature,
            } => ReplayAuthorization::Signed {
                authority,
                at: *at,
                signature,
            },
        }
    }
}
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,
    /// Separately trusted Target JSON
    #[arg(long, value_name = "FILE")]
    pub target: PathBuf,
    /// Public sidecar to compare; never supplies trusted roots
    #[arg(long, value_name = "FILE")]
    pub record: PathBuf,
    /// Export a bounded local first-divergence diagnostic to a new file
    #[arg(long, value_name = "FILE")]
    pub challenge_out: Option<PathBuf>,
    #[command(flatten)]
    pub authorization: AuthorizationArgs,
}
pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    // File/schema/group errors have the same input classification as failed
    // reconstruction. Neither case creates or publishes a challenge.
    let inputs = (|| {
        let target = super::core_io::read_json(&args.target)?;
        let authorization = args.authorization.load(&target)?;
        let candidate = ReplayRecord::from_json(&super::core_io::read_source(&args.record)?)
            .map_err(super::core_io::operation)?;
        let source = super::core_io::read_source(&args.input.file)?;
        let scenario = super::core_io::read_json(&args.input.scenario)?;
        Ok::<_, mori_diagnostics::Report>((target, authorization, candidate, source, scenario))
    })();
    let (target, authorization, candidate, source, scenario) = match inputs {
        Ok(inputs) => inputs,
        Err(error) => {
            super::core_io::print(
                &serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":{"kind":"input","detail":error.to_string()}}),
            )?;
            return Ok(ExitCode::FAILURE);
        }
    };
    let result = mori_backend::replay::challenge(
        &source,
        &args.input.action,
        &scenario,
        &target,
        authorization.as_input(),
        &candidate,
    );
    let (report, status) = match result {
        // Successful full equality includes independently reconstructed record_hash.
        Ok(None) => (
            serde_json::json!({"status":"accepted","qualification":"local-reference-replay","record_hash":candidate.record_hash}),
            ExitCode::SUCCESS,
        ),
        Ok(Some(challenge)) => {
            let d = &challenge.divergence;
            if let Some(path) = &args.challenge_out {
                let published = challenge
                    .to_json()
                    .map_err(super::core_io::operation)
                    .and_then(|text| {
                        super::proof_io::publish_new(path, format!("{text}\n").as_bytes())
                    });
                if let Err(error) = published {
                    super::core_io::print(
                        &serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":{"kind":"output","detail":error.to_string()},"divergence":d}),
                    )?;
                    return Ok(ExitCode::FAILURE);
                }
            }
            (
                serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":{"kind":"divergence","step":d.step,"field":d.field}}),
                ExitCode::FAILURE,
            )
        }
        Err(ReplayError::Input(error)) => (
            serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":{"kind":"input","detail":error}}),
            ExitCode::FAILURE,
        ),
        // challenge converts semantic divergence to a diagnostic. Preserve an
        // explicit refusal if that API ever gains another divergence branch.
        Err(ReplayError::Divergence(d)) => (
            serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":{"kind":"divergence","step":d.step,"field":d.field}}),
            ExitCode::FAILURE,
        ),
    };
    super::core_io::print(&report)?;
    Ok(status)
}

END INPUT

ACTUAL PINNED INPUT source/crates/mori/src/commands/intent.rs
use std::process::ExitCode;

use clap::ValueEnum;

use super::core_io::review_identity;
use crate::cli::{ActionInput, SignedProfile, SigningOutputArgs};
use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,

    /// Legacy host profile: signature scheme of the owner key
    #[arg(long)]
    pub scheme: Option<Scheme>,

    /// Legacy host profile: owner public key, hex encoded
    #[arg(long, value_name = "HEX")]
    pub public_key: Option<String>,

    /// Legacy host profile: how the signing bytes are framed
    #[arg(long)]
    pub framing: Option<Framing>,
    /// Domain, predecessor, nonce, validity and per-asset authority JSON
    #[arg(long)]
    pub authority: std::path::PathBuf,

    /// Signed native owner-intent profile; requires --target and --at
    #[arg(long)]
    pub profile: Option<SignedProfile>,
    /// Trusted Target JSON with signed_policy (signed profile only)
    #[arg(long, value_name = "FILE")]
    pub target: Option<std::path::PathBuf>,
    /// Logical time checked against the validity interval; not signed
    #[arg(long)]
    pub at: Option<u128>,

    #[command(flatten)]
    pub output: SigningOutputArgs,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Scheme {
    #[value(name = "schnorr_bip340")]
    SchnorrBip340,
    #[value(name = "ecdsa_secp256k1_sha256")]
    EcdsaSecp256k1Sha256,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Framing {
    Raw,
    MidnightSignData,
}

pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let legacy = [
        args.scheme.is_some(),
        args.public_key.is_some(),
        args.framing.is_some(),
    ];
    if args.profile.is_some() {
        if legacy.iter().any(|present| *present) {
            return Err(super::core_io::operation(
                "the signed profile refuses legacy --scheme/--public-key/--framing arguments",
            ));
        }
        return signed(args, shell);
    }
    if args.target.is_some() || args.at.is_some() {
        return Err(super::core_io::operation(
            "--target and --at belong to --profile moriarty-jubjub-owner-intent/1; legacy intents never accept a Target",
        ));
    }
    let (Some(scheme), Some(public_key), Some(framing)) =
        (args.scheme, args.public_key.clone(), args.framing)
    else {
        return Err(super::core_io::operation(
            "legacy moriarty-owner-intent/2 requires --scheme, --public-key and --framing together",
        ));
    };
    if !matches!(framing, Framing::Raw) {
        return Err(super::core_io::operation(
            "core/1 intents currently require raw canonical framing",
        ));
    }
    let (source, program) = super::core_io::program(&args.input.file)?;
    let scenario = super::core_io::read_json(&args.input.scenario)?;
    let authority: mori_core::intent::Authorization = super::core_io::read_json(&args.authority)?;
    let scheme = match scheme {
        Scheme::SchnorrBip340 => mori_core::intent::Scheme::SchnorrBip340,
        Scheme::EcdsaSecp256k1Sha256 => mori_core::intent::Scheme::EcdsaSecp256k1Sha256,
    };
    if authority.scheme != scheme || authority.public_key_hex != public_key {
        return Err(super::core_io::operation(
            "signing key or scheme does not match expected authority",
        ));
    }
    let prepared = mori_core::intent::prepare_intent(
        &program,
        &mori_core::intent::hash_bytes(source.as_bytes()),
        &args.input.action,
        &scenario,
        &authority,
    )
    .map_err(super::core_io::operation)?;
    // Reviewing a proposal also checks the exact candidate's local financial effects.
    let execution = mori_core::simulate(&program, &args.input.action, &scenario)
        .map_err(super::core_io::operation)?;
    mori_core::intent::check_effects(
        &authority.signer,
        &authority.limits,
        &execution.effects,
        &execution.post.liabilities,
    )
    .map_err(super::core_io::operation)?;
    if args.output.review {
        println!(
            "Authorize action {} of program {}",
            review_identity(&prepared.intent.action),
            review_identity(&prepared.intent.program_hash)
        );
        println!(
            "Network: {}; anchor: {}; contract: {}",
            review_identity(&authority.network),
            review_identity(&authority.anchor),
            review_identity(&authority.program_address)
        );
        println!(
            "Signer: {}; key: {}",
            review_identity(&authority.signer),
            review_identity(&authority.public_key_hex)
        );
        println!(
            "Predecessor: {}; nonce: {}; validity: {}..{}",
            review_identity(&authority.predecessor),
            authority.nonce.0,
            authority.not_before.0,
            authority.not_after.0
        );
        for limit in &authority.limits {
            println!(
                "Asset {}/{}/{} (scale {}), beneficiary {}: gross <= {}, fee <= {}, net >= {}, closing liability <= {}",
                review_identity(&limit.asset.network),
                review_identity(&limit.asset.issuer),
                review_identity(&limit.asset.reference),
                limit.asset.scale,
                review_identity(&limit.beneficiary),
                limit.gross_cap.0,
                limit.fee_cap.0,
                limit.net_floor.0,
                limit.liability_cap.0
            );
        }
        println!("Exact canonical statement and signing bytes:");
    }
    super::core_io::print(&prepared)?;
    Ok(ExitCode::SUCCESS)
}

/// Signed native preparation: trusted reconstruction of the unsigned payload
/// and its native message. Nothing is signed here.
fn signed(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    let (Some(target), Some(at)) = (&args.target, args.at) else {
        return Err(super::core_io::operation(
            "--profile moriarty-jubjub-owner-intent/1 requires --target and --at together",
        ));
    };
    let source = super::core_io::read_source(&args.input.file)?;
    let scenario = super::core_io::read_json(&args.input.scenario)?;
    let target = super::core_io::signed_target(target)?;
    let authority = super::core_io::authority(&args.authority)?;
    let prepared = mori_backend::signed_relation::prepare_intent(
        &source,
        &args.input.action,
        &scenario,
        &target,
        &authority,
        mori_core::UInt(at),
    )
    .map_err(super::core_io::operation)?;
    if args.output.review {
        let p = &prepared.payload;
        println!(
            "Profile {}: local host reconstruction of the owner intent; not ledger acceptance.",
            review_identity(&p.profile)
        );
        println!(
            "Authorize action {} of program {} (source {})",
            review_identity(&p.action),
            review_identity(&p.program_hash),
            review_identity(&p.source_hash)
        );
        println!(
            "Network: {}; anchor: {}; contract: {}; target: {}",
            review_identity(&p.network),
            review_identity(&p.anchor),
            review_identity(&p.program_address),
            review_identity(&p.target_hash)
        );
        println!(
            "Owner: {}; beneficiary: {}; key: {}",
            review_identity(&p.owner),
            review_identity(&p.beneficiary),
            review_identity(&p.public_key_hex)
        );
        println!(
            "Predecessor: {}; nonce: {}; validity: {}..{} (checked at logical time {at}, which is not signed)",
            review_identity(&p.predecessor),
            p.nonce.0,
            p.not_before.0,
            p.not_after.0
        );
        for limit in &p.limits {
            println!(
                "Asset {}/{}/{} (scale {}), beneficiary {}: gross <= {}, fee <= {}, net >= {}, closing owner liability <= {}",
                review_identity(&limit.asset.network),
                review_identity(&limit.asset.issuer),
                review_identity(&limit.asset.reference),
                limit.asset.scale,
                review_identity(&limit.beneficiary),
                limit.gross_cap.0,
                limit.fee_cap.0,
                limit.net_floor.0,
                limit.liability_cap.0
            );
        }
        println!(
            "Relation context: {}; payload SHA256: {}",
            review_identity(&prepared.context_hash),
            review_identity(&prepared.payload_hash)
        );
        println!(
            "Signed native message (MOI1, FENS domain, 32-byte little-endian): {}",
            review_identity(&prepared.message_hex)
        );
        println!(
            "A native proof is existential over the private signature R/s. The key-to-account association, nonce consumption and predecessor history remain unverified."
        );
        println!("Exact prepared statement:");
    }
    super::core_io::print(&prepared)?;
    Ok(ExitCode::SUCCESS)
}

END INPUT

ACTUAL PINNED INPUT source/crates/mori/src/commands/verify_intent.rs
use std::path::PathBuf;
use std::process::ExitCode;

use crate::cli::{ActionInput, SignedProfile, SigningOutputArgs};
use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,

    /// Signature JSON produced by the external signer (the exact signed
    /// wrapper under --profile moriarty-jubjub-owner-intent/1)
    #[arg(long, value_name = "FILE")]
    pub signature: PathBuf,
    /// Expected account authority and domain binding JSON
    #[arg(long)]
    pub authority: PathBuf,
    /// Current logical time used to check the signed validity interval
    #[arg(long)]
    pub at: u128,
    /// Signed native owner-intent profile; requires --target
    #[arg(long)]
    pub profile: Option<SignedProfile>,
    /// Trusted Target JSON with signed_policy (signed profile only)
    #[arg(long, value_name = "FILE")]
    pub target: Option<PathBuf>,

    /// Cap local statement and uncached expression attempts (other costs unmeasured)
    #[arg(long, value_name = "CANONICAL_U128", value_parser = logical_cap, conflicts_with_all = ["profile", "target"])]
    pub logical_cap: Option<mori_core::UInt>,

    #[command(flatten)]
    pub output: SigningOutputArgs,
}

fn logical_cap(value: &str) -> Result<mori_core::UInt, String> {
    serde_json::from_value(serde_json::Value::String(value.into()))
        .map_err(|_| "expected canonical unsigned decimal string in u128 range".into())
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    match (args.profile, &args.target) {
        (Some(_), Some(target)) => return signed(&args, target),
        (Some(_), None) => {
            return Err(super::core_io::operation(
                "--profile moriarty-jubjub-owner-intent/1 requires --target",
            ));
        }
        (None, Some(_)) => {
            return Err(super::core_io::operation(
                "--target belongs to --profile moriarty-jubjub-owner-intent/1; legacy intents never accept a Target",
            ));
        }
        (None, None) => {}
    }
    let (source, program) = super::core_io::program(&args.input.file)?;
    let scenario = super::core_io::read_json(&args.input.scenario)?;
    let authority = super::core_io::read_json(&args.authority)?;
    let signed = super::core_io::read_json(&args.signature)?;
    if let Some(cap) = args.logical_cap {
        return match mori_core::intent::verify_and_simulate_metered(
            &program,
            &mori_core::intent::hash_bytes(source.as_bytes()),
            &args.input.action,
            &scenario,
            &authority,
            &signed,
            mori_core::UInt(args.at),
            cap,
        ) {
            Ok(metered) => {
                if args.output.review {
                    println!(
                        "Signature and local authority constraints verified at logical time {}.",
                        args.at
                    );
                    println!(
                        "Complete local reference effects follow; ledger acceptance requires its own verification."
                    );
                }
                super::core_io::print(&serde_json::json!({
                    "status": "accepted", "signature_valid": true,
                    "qualification": "local-signature-and-reference-execution",
                    "execution": metered.execution, "profile": metered.cost.profile,
                    "logical": {"cap": metered.cost.cap, "used": metered.cost.used}
                }))?;
                Ok(ExitCode::SUCCESS)
            }
            Err(refused) => {
                super::core_io::print(&serde_json::json!({
                    "status": "rejected", "error": refused.error,
                    "profile": refused.cost.profile,
                    "logical": {"cap": refused.cost.cap, "used": refused.cost.used}
                }))?;
                Ok(ExitCode::FAILURE)
            }
        };
    }
    match mori_core::intent::verify_and_simulate(
        &program,
        &mori_core::intent::hash_bytes(source.as_bytes()),
        &args.input.action,
        &scenario,
        &authority,
        &signed,
        mori_core::UInt(args.at),
    ) {
        Ok(execution) => {
            if args.output.review {
                println!(
                    "Signature and local authority constraints verified at logical time {}.",
                    args.at
                );
                println!(
                    "Complete local reference effects follow; ledger acceptance requires its own verification."
                );
            }
            super::core_io::print(
                &serde_json::json!({"status":"accepted", "signature_valid":true, "qualification":"local-signature-and-reference-execution", "execution":execution}),
            )?;
            Ok(ExitCode::SUCCESS)
        }
        Err(error) => {
            super::core_io::print(&serde_json::json!({"status":"rejected", "error":error}))?;
            Ok(ExitCode::FAILURE)
        }
    }
}

/// Strict signed acceptance: trusted reconstruction, exact wrapper equality,
/// host FENS under the reconstructed message and the ordinary compiled
/// relation check. No unsigned or legacy fallback.
fn signed(args: &Args, target: &std::path::Path) -> super::Result<ExitCode> {
    let source = super::core_io::read_source(&args.input.file)?;
    let scenario = super::core_io::read_json(&args.input.scenario)?;
    let target = super::core_io::signed_target(target)?;
    let authority = super::core_io::authority(&args.authority)?;
    let signed = super::core_io::signed_intent(&args.signature)?;
    match mori_backend::signed_relation::accept_intent(
        &source,
        &args.input.action,
        &scenario,
        &target,
        &authority,
        mori_core::UInt(args.at),
        &signed,
    ) {
        Ok(accepted) => {
            if args.output.review {
                println!(
                    "Exact signed payload, strict FENS attachment and compiled relation accepted at logical time {}.",
                    args.at
                );
                println!(
                    "Host reference check only: not a native proof, key-to-account authority or ledger acceptance."
                );
            }
            super::core_io::print(&serde_json::json!({
                "status": "accepted",
                "signature_valid": true,
                "profile": signed.payload.profile,
                "qualification": "local-signed-owner-intent-host-and-relation-check",
                "payload_hash": accepted.payload_hash,
                "context_hash": accepted.context_hash,
                "message_hex": accepted.message_hex,
                "at": accepted.at,
                "execution": accepted.execution,
            }))?;
            Ok(ExitCode::SUCCESS)
        }
        Err(error) => {
            super::core_io::print(&serde_json::json!({"status":"rejected", "error":error}))?;
            Ok(ExitCode::FAILURE)
        }
    }
}

END INPUT

ACTUAL PINNED INPUT source/crates/mori/src/commands/simulate.rs
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
            .target
            .as_ref()
            .ok_or_else(|| super::core_io::operation("--trace requires --target"))?;
        let target = super::core_io::read_json(path)?;
        let authorization = args.authorization.load(&target)?;
        let source = super::core_io::read_source(&args.file)?;
        let scenario = super::core_io::read_json(scenario_path)?;
        return match mori_backend::replay::generate(
            &source,
            action,
            &scenario,
            &target,
            authorization.as_input(),
        ) {
            Ok(record) => {
                super::core_io::print(&record)?;
                Ok(ExitCode::SUCCESS)
            }
            Err(error) => {
                super::core_io::print(
                    &serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":error}),
                )?;
                Ok(ExitCode::FAILURE)
            }
        };
    }
    if args.target.is_some() || args.authorization.present() {
        return Err(super::core_io::operation(
            "--target and signed replay arguments require --trace",
        ));
    }
    let source = super::core_io::read_source(&args.file)?;
    // Hook: /2 routing. A /2 start keeps every core failure labelled.
    if mori_core::language(&source) == Ok(mori_core::stage::LANGUAGE_2) {
        return run_stage_start(&args, &source, action, scenario_path);
    }
    let program = mori_core::parse(&source).map_err(super::core_io::operation)?;
    let scenario = super::core_io::read_json(scenario_path)?;
    if let Some(cap) = args.logical_cap {
        return match mori_core::simulate_metered(&program, action, &scenario, cap) {
            Ok(metered) => {
                super::core_io::print(&serde_json::json!({
                    "status": "accepted", "qualification": "local-reference-execution",
                    "execution": metered.execution, "profile": metered.cost.profile,
                    "logical": {"cap": metered.cost.cap, "used": metered.cost.used}
                }))?;
                Ok(ExitCode::SUCCESS)
            }
            Err(refused) => {
                super::core_io::print(&serde_json::json!({
                    "status": "rejected", "error": refused.error,
                    "profile": refused.cost.profile,
                    "logical": {"cap": refused.cost.cap, "used": refused.cost.used}
                }))?;
                Ok(ExitCode::FAILURE)
            }
        };
    }
    match mori_core::simulate(&program, action, &scenario) {
        Ok(execution) => {
            super::core_io::print(
                &serde_json::json!({"status":"accepted", "qualification":"local-reference-execution", "execution":execution}),
            )?;
            Ok(ExitCode::SUCCESS)
        }
        Err(error) => {
            super::core_io::print(&serde_json::json!({"status":"rejected", "error":error}))?;
            Ok(ExitCode::FAILURE)
        }
    }
}

/// Render the same explicit qualification on every staged public object.
pub(super) fn print_stage_report(mut report: serde_json::Value) -> super::Result<()> {
    let object = report
        .as_object_mut()
        .expect("stage reports are JSON objects");
    for (name, value) in mori_core::stage::STAGE_LABELS {
        object.insert(name.into(), serde_json::Value::String(value.into()));
    }
    super::core_io::print(&report)
}

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

END INPUT
