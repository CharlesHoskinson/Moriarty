# Security source review: Core2 normal financial consumer packet

**Verdict: APPROVE, scoped to SOURCE ONLY.**

- **Candidate membership SHA256 (as supplied):** `96521effe3e13486e72a02c4b5be52cba27f154dcd99018c2ef8dcb7f7483091`
- **Reviewer:** claude-opus-5-5 (security owner). I am a fresh non-author.
- **What this verdict does not approve:** any build or run, any command, any execution budget, admission, or the Phase 7 `financial_compiler_projection_matches` predicate. Native authority stays at zero.

I had no tools, so I recomputed none of the hashes. That includes the membership hash, the 113 KB literals hash and every frozen-member hash. All hashes in this report are as stated by the packet.

## Confirmed source properties

1. **Await binding without widening v1 deserialization.** The generated file builds `Program`, `Statement::Await`, `Step`, `Execution`, `StagePlan` and `StageOutcome` directly as Rust struct and enum literals.
   - `ir.rs` is unchanged. Its `decode_via_v1!` routing still covers every shared type.
   - No `Deserialize` impl is added. The consumer only uses `Serialize`, for printing.
2. **Every expected field is bound.**
   - In the translator, `fields()` rejects any key-set mismatch for structs, enum variants and the single-key `StageOutcome`.
   - In Rust, struct literals and the field-naming patterns in `expected_view` (no `..`) fail to compile if a field is omitted or added.
   - `expected_segment` names all six `ExecutionObservation` fields. `ResumeFinancialObservation` names all five of its fields.
3. **Actual and expected values come from separate sources.**

   | Field | Actual value comes from | Expected value comes from |
   |---|---|---|
   | Source | `.mori` via `include_str!` | — |
   | Scenario, action, start cap | Scenario file and `inputs.json` | — |
   | Event | Event file (`event_input`) | `expected.json` `event` |
   | CLI action and cap | `inputs.json` (`cli_*_input`) | `expected.json` `cli_action` / `cli_cap` |
   | Resume continuation | The published actual continuation, keyed by `continuation_from` | `expected.json` `input` |
   | Outcome | `stage::start` / `stage::resume` result | `expected.json` outcome |

   - Actual arguments are captured before owned clones are passed to `stage::resume`.
4. **Expected values do not depend on production code.**
   - Expected views are built by local `expected_view` and `expected_segment`. They never call `project_*`.
   - `hash_text` re-implements hex encoding independently.
   - The translator imports no subject code and contains no evaluator.
   - The `.mori` sources are compile-time inputs only.
5. **Comparison depth.** For every invocation the test compares:
   - the full `Program`;
   - the full `StagePlan`;
   - both canonical digests;
   - the full `Result<StageOutcome, StageRejection>`;
   - the projected `FinancialObservation`.

   For every resume it also compares the full `ResumeFinancialObservation`. A production `Err` fails closed against the Waiting or Complete expectation.
6. **No host financial interpreter.** There is no evaluator in the consumer, the translator or the projection.
7. **Fixture gaps are preserved.**
   - All five resume CLI flags are null on both the input and expected sides.
   - There is no Error or Rejected expectation and no hostile case.
   - R0 and the CLI-cap branches are not exercised.
8. **Spot-checked literal correspondence.** I compared the following against `expected.json` and the input files, with no discrepancy found:
   - F01/F02 scenarios and events;
   - IR expression IDs 13, 30, 39 (F02) and 4–6, 20 (F01);
   - plan captures, origins and ordinals;
   - `segment_cost`, `committed_before` and `attempted` per invocation;
   - snapshots at several steps;
   - aggregate effects;
   - the present-zero `grantDue` row and the empty residual.

   This was a sampled check, not a full byte diff.

## Compile plausibility (read-only, not compiled)

**No compile defect found in the consumer, the literals or the isolated root.** I checked:
- glob imports: `crate::*` and `stage::{self, *}` have no ambiguity in the isolated root;
- `[].into()` for `BTreeMap` through the single-impl `From<[(K,V);N]>`;
- `r#from` (a valid raw identifier);
- `json!` values: `$value:expr` keeps `actual_view == expected_resume` grouped;
- NLL ordering before `actual` is moved;
- `Result` `PartialEq` inference;
- `include!` with multiple items, and `include_str!` resolving to the same directory under both layouts;
- `#[path]` to `stage/mod.rs` resolving submodules in that directory.

Requirements that follow from the closure sources:
- **Toolchain:** rustc 1.82 or later (`Option::is_none_or` in `intent.rs`; `let … else` in `parser.rs`).
- **Edition:** must match the pinned manifest.
- **Warnings:** the isolated root will likely raise dead-code warnings. Do not deny warnings.

## Findings (none blocking source approval)

**S1 — Medium (evidence under a once-only run).** One `#[test]` runs F01 and then F02.
- Any F01 panic suppresses every F02 observation.
- Recommendation: split it into `…_f01` and `…_f02` tests.
- Applying this changes membership and needs a delta re-review.

**S2 — Medium (run-scope input).** The isolated root compiles existing `#[cfg(test)]` unit tests in `json.rs` and `stage/cost.rs`.
- A run without a filter would also execute them.
- libtest hides stdout of passing tests by default.

**S3 — Medium (closure input).** Dependency versions and features are not pinned in the packet.
- There is no lockfile.
- `mori-lexer`'s own dependencies are not listed in `external_crates`.
- Compile-time code execution from proc-macros (`serde_derive`, `thiserror`) and any `build.rs` belongs to the build classification.

**S4 — Low (registration route only).** Two issues:
- `use crate::*` against the real `lib.rs` could produce a glob ambiguity (for example `Program`, `Value`, `Step` or `Effect` if `sherlock` is glob-re-exported). The `lib.rs` bytes were not provided.
- The patch also makes `sherlock_financial_projection` part of `mori-core`'s public API. `committed_logical_work` can echo an unvalidated input counter. Visibility (`pub` versus `pub(crate)` or test-only before Phase 7 review) is a Root decision.

**S5 — Low (translator robustness, regeneration only).** `lit()` does not check types:
- `Bool` is taken from Python truthiness, so the string `"false"` becomes `true`.
- `UInt` text is inserted unchecked, so `040` or `1_0` would compile even though v1 decoding rejects them.
- `Value` and `Map` have no key-set check.
- Any operation other than `start` is treated as resume.
- Python's `json` silently collapses duplicate keys.

The committed generated file is what compiles, and my sampled checks of it are clean. Harden the translator before any regeneration: add `isinstance` checks, a canonical-decimal check and a duplicate-key `object_pairs_hook`.

**S6 — Info (coverage boundary).** This test exercises the library API, not the CLI.
- Events, scenarios and continuations never pass through `decode::event`, `decode::continuation` or `json::parse`. Only the internal `emit_checked` round-trip is exercised.
- Continuations are passed in memory.
- Do not report this as file-decoding coverage.

**S7 — Info (label discipline).** `STAGE_LABELS` is printed only on the load line.
- The `accepted` invocation resumes a continuation byte-equal to one already resumed, which is documented untracked-consumption behaviour.
- Retained evidence should carry the labels per line or in a run header.
- A pass must not be cited as replay protection.

**S8 — Info (provenance).** Two copies of `SHERLOCK-IMPLEMENTATION-PLAN.md` differ:
- context copy: `14488f23…`
- oracle normative copy: `1e63734b…`

Confirm which revision the supplied Phase 7 excerpt comes from. Confirm the change does not alter the predicate the oracle was frozen against.

## Classification of the proposed stage path and closure

- **Path:** in-process `stage::load`, then `stage::start`, then `stage::resume` repeated, using existing Core/2 code unchanged.
- **Kind:** an ordinary local reference-stage diagnostic, test-only (`cfg(test)`).
- **I/O and dependencies:**
  - no runtime filesystem I/O;
  - no network;
  - no unsafe code in the candidate files;
  - no SDK, native, proof, ledger or target code.
- **Isolated root:** SDK-free at the source level. Nothing in the shown closure imports Midnight. `k256` enters through `intent::canonical`, which `stage::digest` uses.
- **Registration route:** not SDK-free under an ordinary `cargo test`, because of `mori-core`'s Midnight dev-dependencies.

## Exact future classification inputs Root still needs

1. The build artifact for the isolated root: the manifest or `rustc` invocation, crate name, edition, `--test`, the `--extern` map, and the toolchain version.
2. Pinned dependency versions and features: a lockfile or equivalent covering `serde` (derive), `serde_json`, `thiserror`, `sha2`, `k256` (`schnorr`/`ecdsa`), and `mori-lexer`'s dependencies. Build `--offline --locked` from a local source, with no registry fetch.
3. A pre-build hash check of every compiled byte against `SOURCE-CLOSURE.json`, `INPUT-MEMBERSHIP.json` and candidate membership.
   - This covers the source-closure files, the packet `.rs` files and the `include_str!` `.mori` targets.
   - Check from a read-only staging copy to avoid a check-then-use gap.
4. Whether compilation counts against the 30 s budget. A cold build of `serde_derive` and `k256` probably exceeds it, so classify the build separately so a timeout is not misread.
5. The exact test invocation:
   - an `--exact` filter on the consumer test path;
   - `--nocapture` (or `--show-output`);
   - `--test-threads=1`;
   - stdout, stderr and the exit status all retained.
6. Optionally, a deterministic `python3 -I` regeneration of the literals, byte-compared to `96238b98…`.
7. A disposition on S1.
8. For the registration route only: the `lib.rs` bytes for the glob-conflict check, and the public-visibility decision.

## Limitations and unresolved questions

- **Bytes not supplied, so not reviewed:**
  - `NORMATIVE-FREEZE.json`, `VALIDATION.json`, `accounting.json`;
  - the oracle's `transcribe_literals.py` and `validate_literals.py`;
  - the normative docs and spec copies;
  - `context/…/lib.rs` and `context/…/tests/continuations.rs`;
  - both `Cargo.toml` snapshots;
  - `mori-lexer` sources.
- **Not verifiable from the packet:**
  - the freeze ordering;
  - the claim that the projection author never received expected values.
- **Literals:** reviewed by sampling, not diffed exhaustively.
- **Compile plausibility:** established by reading only.
- **Agreement with the oracle:** no claim is made that actual results will match.
- **Not addressed by this packet and still outside my verdict:** CLI decode coverage, non-null CLI flags, rejection and hostile cases, authenticated or consumed continuations, and any compiler, target or Phase 7 claim.
