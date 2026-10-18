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
