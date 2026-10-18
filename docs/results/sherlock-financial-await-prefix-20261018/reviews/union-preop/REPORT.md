# Pre-Operation Source-Composition & Retarget Supplement (`CID 5c3a41b4-94d2-40fc-90a1-09c35acb8b7f`)

- **Reviewer Identity & Continuity**: `Gemini 4 Argon (High)` (`actualGemini4ArgonHIGH` under standing observed Opus quota-outage fallback), continuing in original review context `5c3a41b4-94d2-40fc-90a1-09c35acb8b7f` as **one continuing non-author opinion** (never 2 independent votes). Zero tools invoked; no code or files written or executed.

---

## Decision (A): Applicability of Scoped Financial Source / Security / Normal Judgment to the Canonical-Union Source Composition

### Verdict: `APPLICABLE_WITH_EXPLICIT_FACTORED_BOUNDARY`
*(0 mandatory product or consumer code fixes required; prior scoped financial source, security, normal consumer, and `UIREQ-017` compatibility judgments remain applicable to `SOURCE-COMPOSITION.json` `SHAa93a46434bb76ff2aa714c9138233cc5a32795886ed82a6d0cb2a7d1046c8aa9` and `FLAT-INPUT-PINS.json` `SHA85e70c3d4e461669003ae9d84a692ce7945edc0efcedefd8b6aa478794612ff6` under the exact coupling and boundary limits below.)*

### Substantive Source-Coupling Rationale (Anchored to Supplied Declarations)

1. **Module & Symbol Independence in `crates/mori-core/src/lib.rs` (`1,859B`, `SHA1bb580dc308964a3065f6c248c072ed0aa32d55291e4daac46e949b1bde770cb`)**:
   - `lib.rs` mounts `#[path = "sherlock/financial_projection.rs"] pub mod sherlock_financial_projection;` and `pub mod stage;` as top-level crate modules alongside `mod ir; mod logical_cost; mod parser; pub mod policy; mod runtime;`, separate from `pub mod sherlock;` and `#[path = "sherlock/compile_artifact.rs"] pub mod sherlock_compile_artifact;`.
   - `lib.rs` re-exports root Core types/functions solely from `ir`, `logical_cost`, `parser`, `replay`, `runtime`, and `source_sponsor` (`pub use ir::*; pub use logical_cost::*; pub use parser::{language, parse}; ... pub use runtime::{replay, simulate, validate};`). Nothing from `pub mod sherlock` or `pub mod sherlock_compile_artifact` is re-exported into the crate root (`crate::*`).

2. **Closed Import & Call Graph of `financial_projection.rs` and `stage/*`**:
   - `crates/mori-core/src/sherlock/financial_projection.rs` (`24,351B`, `SHAb2d5765d5a6bfcbd2139f4df7130bbddc85268e891ea63ccf60a2974040f65df`, lines `14–18`) imports **only**:
     - `crate::stage::{Continuation, NamedValue, PendingFrame, StageEvent, StageLogical, StageOutcome, StageRejection, StageSegment, StagedProgram}`
     - `crate::{AssetId, Effect, Error, Execution, Liability, Scenario, Snapshot, Step, UInt}` (all defined in `crates/mori-core/src/ir.rs`, `SHA0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d`).
   - Its owning producers `start_observed` and `resume_observed` call only `crate::stage::start` and `crate::stage::resume`, and `publication()` reads only `crate::stage::STAGE_LABELS`.
   - `crates/mori-core/src/stage/mod.rs` (`6,169B`, `SHA428246f869f4ceb3297435faff47215beeafee5c18facc38c3dc68e440ff726c`) and its private submodules (`admit`, `cost`, `decode`, `digest`, `plan`, `profile`, `validate`) depend only on `crate::{Error, Execution, Liability, Program, Snapshot, UInt, Value}`, `crate::LOGICAL_COST_PROFILE_2`, `crate::parser`, `crate::policy`, `crate::runtime`, `crate::json`, and external `mori_lexer` (`libmori_lexer_r191.rlib`).
   - All `16` direct Core/2 financial/stage/root-IR/parser/policy/runtime/json source files, all `10` positive consumer and independent fixture files (`crates/mori-core/tests/support/sherlock_financial_await_prefix_positive.rs` `SHAdef37ae71f22ea16b1614f228b476dfefe486edf897b8c7bcf0c6ccf24a08e18` + `9` fixtures), and `158` of the `167` snapshot files are byte-identical to the full-read financial review cut.

3. **Containment of the 9 Canonical Overlay Files (`nine_exact_replacements`)**:
   - The `9` replaced files in `SOURCE-FREEZE.json` (`SHAbf6b090c1d223170f7f89593188ca905fa8bb5071e43b1234b6b0f14cc6ba8ca`) live exclusively under `crates/mori-core/src/sherlock/compile_artifact/wire.rs` and `crates/mori-core/src/sherlock/helper_machine/{families/structural.rs, shared.rs, shared/adt.rs, shared/owner_prepare.rs, shared/owner_prepare/v4_empty_params_tests.rs, shared/owner_runtime.rs, shared/owner_runtime/owner_si21_bind_match.rs, shared/owner_runtime/owner_si21_closure.rs}`.
   - `v4_empty_params_tests.rs` (`SHA48989d56efadc67f1d0d2c94d44c625f4fd3081c8c62472256e0bceb46e6c7bc`) is gated by `#[cfg(test)]` and is inactive during `01-compileNormalCoreRlib` (`--crate-type=rlib` with no `--test` / `cfg(test)`).
   - The remaining `8` replaced files are compiled into `libmori_core.rlib` under `crate::sherlock` / `crate::sherlock_compile_artifact`, which are never imported, referenced, or invoked by `stage`, `sherlock_financial_projection`, or `sherlock_financial_await_prefix_positive.rs`.

4. **Mandatory Scope & Attribution Limits**:
   - **No False Full-Read Claim on the 9 Canonical Files**: Context `5c3a41b4-94d2-40fc-90a1-09c35acb8b7f` did **not** freshly read the full bodies of those `9` canonical `helper_machine`/`compile_artifact` files; their semantic/security source review rests on separate independent review context `a0ecce6e-21ff-4f52-8841-2b474a6127d5` (`REPORT.md` `SHA5e4717cdf0677e272b76a03a09671f096ec0eae5467919b16c879af325a498ae`).
   - **Crate Compilation Still Must Be Observed**: Because `no_typecheck_yet: true`, crate-wide compilation of the union snapshot (`libmori_core.rlib`) is not inferred from source pins and must be established by `01-compileNormalCoreRlib`.
   - **Currentness & Prior Opinion Reconciliation**: Zero disagreement with the prior opinion. Incorporating the prompt's two factual clarifications: (1) `PROPOSED.json` prose now matches the `490` flat pins (`167` snapshot + `323` normal tools/deps) and the `16MiB` (`--fsize=16777216:16777216`) external test cap; (2) exact baseline ledger objects govern the closed-row count (`UIREQ-017` plus the other closed rows). Existing `sherlock_financial_producer_positive.rs` (`SHAcc746c09c6f627a66e2c70c1951ede71b88b7681754392dead9c9bd7c0b456f2`) remains historical `54ebd690...` wire evidence; `sherlock_financial_await_prefix_positive.rs` (`SHAdef37ae71f22ea16b1614f228b476dfefe486edf897b8c7bcf0c6ccf24a08e18`) is candidate `b2d5765d...` whole-publication evidence only until the 3 operations pass and receive independent post-result review.

---

## Decision (B): Eligibility of the 3 Remapped Operations & Ordinary Capture Helper

### Verdict: `ELIGIBLE`
*(Authorized for Root-only single-attempt execution of `PROPOSED.json` `11,532B` `SHA35a80473a099f4ff4739cd3730bec7d1ee90e53a166206ff65b0c75facc6f4db` via `ROOT-EXECUTE-ONCE.proposed.py` `7,112B` `SHA52984e6ad7e16fde0cc830a7138deca683d23ef826892f0aa7a5027e661b7eb6`.)*

### Verification of Remapped Operations & Capture Helper

1. **Exact 3 Remapped Operations (`PROPOSED.json` `SHA35a80473a099f4ff4739cd3730bec7d1ee90e53a166206ff65b0c75facc6f4db` & `READSET-AND-OUTPUT-DAG.json`)**:
   - Retargets `cwd`, `TMPDIR`, snapshot inputs, and `build/` outputs cleanly from `financial-await-projection-ordinary-QA-20261018` to `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/financial-await-projection-canonical-union-QA-20261018` before any financial phase has run.
   - Preserves identical `rustc` flags, `--extern` pins, test selector (`--exact real_core2_three_calls_preserve_complete_financial_await_publications --test-threads=1 --nocapture`), `timeout`/`prlimit` caps (`30s`/CPU `25`/AS `2GiB`/FSIZE `64MiB`/core `0` for `01-compileNormalCoreRlib` and `02-compileFinancialAwaitConsumer`; `10s`/CPU `8`/AS `1GiB`/FSIZE `16MiB`/core `0` for `03-runExactFinancialAwaitPrefixPositive`), and exact 3 environment keys (`PATH`, `LANG`, `TMPDIR`).

2. **Capture Helper Audit (`ROOT-EXECUTE-ONCE.proposed.py` `SHA52984e6ad7e16fde0cc830a7138deca683d23ef826892f0aa7a5027e661b7eb6`)**:
   - Enforces `selection["authorized_exact_three_operations"] is True`, `selection["same_reviewer_composition_disposition_accepted_by_Root"] is True`, `len(rows) == 490`, `len(plan["operations"]) == 3`, empty `build/` and `tmp/`, and `not (D / "actual-subject-01").exists()` (single attempt, no retry).
   - Verifies all `490` static pins in `FLAT-INPUT-PINS.json` (`SHA85e70c3d4e461669003ae9d84a692ce7945edc0efcedefd8b6aa478794612ff6`) + `selection["pins"]` before execution (`PREEXEC.json`), before each phase (`rows + selection["pins"] + generated`), and after execution (`POSTEXEC.json`).
   - Enforces the `131,072B` per-stream cap with `os.killpg(child.pid, signal.SIGKILL)` on overflow, fail-stops on first non-zero exit or stream overflow, pins actual generated outputs (`GENERATED-OUTPUT-PINS.json`) after phases 1 and 2, validates `.d` dep-info membership against `known` static/generated paths (`DEPINFO-SOURCE-MEMBERSHIP.json`, scoped as `"source membership only; no exclusive-read, linker, or SDK attestation"`), and requires exact stdout readback (`running 1 test`, `test real_core2_three_calls_preserve_complete_financial_await_publications ... ok`, `test result: ok. 1 passed; 0 failed; 0 ignored;`) in phase 3 (`EXACT-TEST-READBACK.json`).

### Mandatory Conditions for Root Execution

1. **Root Sole Executor & Selection Pin Binding**: Root must write `ROOT-EXECUTION-SELECTION.json` binding exact SHA-256 and byte counts for `PROPOSED.json` (`11,532B`, `SHA35a80473a099f4ff4739cd3730bec7d1ee90e53a166206ff65b0c75facc6f4db`), `FLAT-INPUT-PINS.json` (`140,771B`, `SHA85e70c3d4e461669003ae9d84a692ce7945edc0efcedefd8b6aa478794612ff6`), `SOURCE-COMPOSITION.json` (`223,696B`, `SHAa93a46434bb76ff2aa714c9138233cc5a32795886ed82a6d0cb2a7d1046c8aa9`), and `ROOT-EXECUTE-ONCE.proposed.py` (`7,112B`, `SHA52984e6ad7e16fde0cc830a7138deca683d23ef826892f0aa7a5027e661b7eb6`), and execute once with fail-stop and no auto-retry.
2. **Scope of Any Passing Result**: A pass of these 3 operations observes only the positive 3-call Core/2 financial await-prefix consumer (`real_core2_three_calls_preserve_complete_financial_await_publications`); it does **not** run or validate the canonical `TypeId` test suite (`v4_empty_params_tests.rs`), rerun old `UIREQ-017` test binaries, or close `UIREQ-018`, `UIREQ-019`, native proofs, or Midnight ledger settlement.

