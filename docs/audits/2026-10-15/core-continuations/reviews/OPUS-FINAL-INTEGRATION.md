# OPUS-FINAL-INTEGRATION — fresh non-author final integrated security review

- **Reviewer:** Claude Opus 5.5 (`claude-opus-5-5`), high effort, fresh session. I am not an author, finisher, classifier or integrator of any reviewed artifact. I used no other reviewers and no orchestration.
- **Candidate:** `dd4e4298322666c140c74343a7827000f578c848`, tree `f954cd00fc26a30a2c7c21bb7995bd6775fbaa35`. Predecessor `7d6c3965`, base `fe01bc8b`. The worktree was clean.
- **Startup:** AGENTS.md and execution-focus loaded. Guarded `status --json` returned only the historical SP01 queue, and I dispatched nothing.
- **Method:** read-only source/git/sha256/plain-capture reads; no build, test, Quint, SDK, native, key, PK, SRS or cache action. Counters unchanged; Authority 0; only this file written.
- Exceeded 650 s to finish required reads.

## Dispositions (whole child feature: local-reference-stage / none / untracked)

| Axis | Disposition |
|---|---|
| SOURCE | **NEEDS_CHANGES (test-only).** Production code: no defect. Two required hostile-test gaps remain: F-1 and F-2. |
| ACTUAL-RESULTS | **APPROVE.** All 86 selected functions are verified as captured (24 last-round, 62 retained). |
| DOCS | **APPROVE.** |
| INTEGRATED | **NEEDS_CHANGES.** F-1 and F-2 are required. Not approval. |

No native, parent, V1, U/M/MC/SP/G, event-authentication or consumption approval is given. All ten `/4` gates stay `unperformed`.

## Identity and delta (independently verified)

- `git diff --quiet 7d6c3965 dd4e4298 -- ':(glob)crates/**/src/**'` exits 0. No crate `src`, Cargo.toml, Cargo.lock or build file changed.
- The only changed code is three test files: `stage_relation.rs` (+30), the new `core_profile_refusals.rs` (134) and `core_continuations.rs` (+60). The other changes are ROADMAP, the guide, the examples README and dossier files. I read the full delta.
- Both absolute manifests pass `sha256sum -c`: 514 dossier entries and 85 source/doc/model entries.
  - Projection check: 514 lines with the exact prefix, and line-aligned spot checks of the 12 new capture entries.
  - Full line-by-line diff: harness-refused.
- Current hashes equal the fresh-capture pins:
  - `stage_relation.rs` `3c5d0111…`, `core_continuations.rs` `816a4320…`, `core_profile_refusals.rs` `ca14acf3…`;
  - ledger `profile`/`static_compile`/`signed/mod`/`Cargo.toml` and CLI `simulate`/`inspect`/`compile_call`;
  - Cargo.lock and the oracle `76075b63…`.
- The five frozen design docs and FROZEN-DESIGN.json (`da18918c…`) match their pins.
- Legacy projection:
  - `git diff --quiet 95df7355 eb2577d6 -- crates/mori-{core,backend,lexer,diagnostics,span} Cargo.*` exits 0, and `eb2577d6` is an ancestor of base.
  - The 64-entry per-blob sha256 projection is root-recorded; I did not recompute it. I did not read, stat or hash the 789 MB baseline executable.

## Actual results

I read the full start/result/stdout/stderr of `backend-relation-03`, `cli-normal-04` and `ledger-profile-refusals-01`.

| Capture | Exit | Passed | Stream hashes vs result.json |
|---|---|---|---|
| `backend-relation-03` | 0 | 10 | recomputed, equal |
| `cli-normal-04` | 0 | 11 | recomputed, equal |
| `ledger-profile-refusals-01` | 0 | 3 | recomputed, equal |

- Every start and result records `native_actions_requested` 0, and both new-test captures record empty leftover lists.
- The test names match the source one to one.
- The pin comparison covers 990 pins (49×4 + 138×2 + 139×2 + 58 + 101 + 81) and 22 streams over 11 captures. I checked each delta against its call path:
  - **README:** never `include_str!`'d.
  - **`core_continuations.rs`:** absent from the backend binaries.
  - **`stage_relation.rs` and `stage_hostile.rs`:** separate test binaries.
  - **`lower.rs` on frame/comparator:** unreached, per the OPUS-SUCCESSOR-B premise.
- Counts (no delta reaches a selected path):
  - 86 = 5+4+13+22+5+10+11+6+3+6+1.
  - 24 last-round = 10+11+3.
  - 62 retained = 44 Core + 5 + 6 + 6 + 1.
- No rerun is summed. The three original failures (`cli-normal-01`, `backend-normal-01`, `quint-lifecycle-03`) are retained and not counted.
- Quint: 2 + 9 = 11 normal mathematical-int examples. I read `ADAPTER`/`LIFECYCLE`/`INPUTS`/`LIFECYCLE-INPUTS` in full.
  - This is accepted as finite normal-example QA only.
  - It has no adversarial domain, no invariants and no Rust correspondence.
- Plain captures are hash-bound to their receipts; `.gz`→plain equality is a root/manifest premise.

## Required findings from earlier reviews: resolution

- **A-C1 / H19 — CLOSED.**
  - **CLI:** `h19_…compile_call…` (`core_continuations.rs`). `/2` is refused by `core_io::program` before the deployment read (`compile_call.rs:60-63`) and before `create_dir` (`:68`). The test checks exact diagnostic equality, empty stdout, and that neither the deployment nor `out` exists.
  - **Ledger:** `static_compile::compile` refuses with `(profile, core1-required)` and Display `mori::ledger::profile: core1-required` under all three authorizations. The invalid-limits control proves the guard at `static_compile.rs:370` precedes `limits.validate()` (`:168`).
  - **Signed:** `SignedStaticBindings::from_source` gives the same refusal for both versions. The wrong-profile control proves `signed/mod.rs:210` precedes `:211`.
  - **`/1` control:** gives `native-relation/source-identity` and `input/native-work-limits`, so the guard is profile-specific.
  - **Fixtures:** public Jubjub generator only; no key, file I/O, verifier or prover.
  - These arms are construction-enforced and unreachable for `/2`:
    - `compile_source_static_roles` is `pub(crate)` and reached only behind `:370`.
    - `relation::compile`/`reopen` refuse by parse or source-identity first (`relation/mod.rs:87-92`).
    - Preparation and `SignedCallRequest::admit` need a sealed `SignedStaticBindings`, whose `pub(crate)` fields leave `from_source` as the only constructor.
    - The Await arms are pre-empted by `require_core1`, which refuses a relabelled Await (`stage_static_cost.rs:515-524`).
- **T4:** a duplicate of H19. It is closed with A-C1 and not counted twice.
- **A-C2 / H16 — CLOSED.** Three exact CLI pins:
  - **`/1 --resume`:** exact key set, the error object with offset 21, attempted/committed `"0"`, the three labels, and the absent paths still absent.
  - **`/2 --trace --target`:** exact JSON `mori::backend::program`/`local-reference-replay`, and exactly 3 directory entries, so no sidecar was written.
  - **`/2 inspect --target`:** exact diagnostic and target unread.
  - Path: `simulate.rs:181-203` and `:66-92`.
- **A-D1 — CLOSED.** Both cap paragraphs match `cost.rs:184-216`: the committed-before check, the CLI cap capped by the stored cap, and the effective cap covering committed plus the next segment. `digest.rs:20-25` hashes only Program and StagePlan, so the cap is outside both digests.
- **B-T3 / H20 — CLOSED.** `legacy_decoder_refuses_staged_artifacts`:
  - The honest `/4` control decodes, and `format`/`abi` are `/4`.
  - The byte path starts `{\n  "format": `. Its error is `is_data` and equals the `/3` format diagnostic once the position suffix is stripped.
  - The sorted `Value` path equals the `/3` ABI diagnostic exactly. No workspace manifest enables `preserve_order`.
  - The forward direction is already covered (`stage_relation.rs:301-318`).

## H1–H28 coverage

Precedence: A2 > A-FINAL > CLARIFICATION > original. Exercised in selected passing captures unless marked.

| H | Evidence | Status |
|---|---|---|
| H1–H14, H24, H25 | `stage_hostile` (premise P-A) | Covered. H13/H14 are characterization pins (P3), not passes. |
| H15 | `stage_static_cost.rs:506-507`: `/1` with `let await` only parses | **F-2 gap** |
| H16 | Core `:502-504`, compat C7, backend `legacy_compilers…`, plus the three new CLI pins | Covered |
| H17, H26 | static-cost proptests, `stage_hostile` h17_h26 | Covered |
| H18 | `compile_staged_refuses_signed_target`: code and message, before load | Identity covered. **No-output part: F-1 gap** |
| H19 | CLI and ledger pins above | Covered |
| H20 | Both directions | Covered |
| H21 | relation pending-role/context tests and frame units (premise P-B) | Covered, structural only |
| H22 | static-cost h22 | Covered (P-A) |
| H23 | compat_v1 ×3, plus C7 | Covered |
| H27 | `frame::public_at`, by construction, plus the position test | Covered. No builder introspection (B's optional limit). |
| H28 | `source_closure_coverage` (6) | Covered |

## Findings

| ID | file:line | Predicate | Severity | Required? | Trace |
|---|---|---|---|---|---|
| F-1 | `crates/mori/tests/core_continuations.rs` (no case); path `crates/mori/src/commands/compile.rs:20-25` | No test shows that `mori compile` with `/2` and a signed Target leaves no `--out` directory. The backend test cannot observe this, because `compile_staged` is pure (`lower.rs:22`, `_out` unused). The CLI orders compile before `create_dir`, so this holds by construction but is not exercised. A-completion assigned H18 to scope B and B-completion assigned the CLI part to scope A, so neither tested it. | LOW (test-only) | **Required** | DESIGN-OPUS.md:295 H18 "unsupported, no out directory"; :232 "must not exist afterwards"; §8 "must exist"; A7 (FINAL:274) |
| F-2 | `crates/mori-core/tests/stage_static_cost.rs:505-507` | H15 requires "unchanged parse **and Execution bytes**" for a `/1` program that uses `await`/`when` as identifiers. Only parse success is asserted. No test compares that program's Program JSON or Execution JSON with independently frozen literals. The compat specimen has no such identifiers. A-completion delegated this to the comparator and B did not take it up. | LOW (test-only) | **Required** | DESIGN-OPUS.md:292 H15; C6 (:81) "parse and run byte-identically" |
| O-1 | `crates/mori-backend/src/lib.rs:377-378` | `compile_signed` with `/2` is not exercised. The identity is pinned through `compile` and the trace test, and the guard is the same helper as the first statement. | INFO | Optional | A9 |
| O-2 | `crates/mori-ledger/src/layout.rs:135-136` | `admit_source` runs validate before the guard, so `/2` gives `source-profile/core-validation`. This fails closed. `admit_bindings_inner` guards first (`:158`). A9's literal "callers" condition is satisfied. | INFO | Optional | A9 |
| — | B-O1/O2/O3, A-1…A-4, the escaped-key nit | Unchanged | — | Optional; not promoted | — |

**Fixes (no design change):**
- **F-1:** add a CLI case `compile delivery.mori --target <signed golden target> --out <absent>`. It should assert exit 1, the `mori::backend::unsupported` diagnostic and `!out.exists()`.
- **F-2:** freeze literal Program and Execution JSON for the `let await = 1; let when = await;` `/1` program, and assert equality through `mori_core::parse`/`simulate`, or through the `/1` CLI envelope.
- Each is one test plus a capture, needing Opus closure classification and fresh non-author review.

## Other required reads completed

- **Design:** A2, A-FINAL, CLARIFICATION, original and FINAL-NONSECURITY, all in full; ADOPTION-SOL in full.
- **Normative:** OSR-1014 (253 lines); the product contract and SECURITY-CURRENTNESS in full; UNI-006/007/008 (`spec.md:58-89`).
  - OSR-1014-10's hostile unpaid-discharge control is met for these programs by H11/H12: payment failure rejects before discharge, atomically. The parent requirement stays open.
  - UNI-007 is not claimed. UNI-008's budget-reset witness is disclosed through A-D1.
- **Code:** `stage_static_cost.rs:1-99` (helpers). Lexer `scan.rs:5-7`: identifiers are ASCII alphanumeric plus `_`, so staged filenames hold no separators (closes A's premise 3).
- **Docs:** dossier README, OBSERVATIONS-FINAL, SOURCE-SCOPE-FINAL, the guide and the ROADMAP/example deltas. All claims are scoped and accurate; B's D-O1 is fixed (README:140-142).

## Premises relied on (not re-verified)

- **P-A.** OPUS-SUCCESSOR-A read the 8 Core stage modules in full and found no required defect. A-COMPLETION read the CLI, ledger and closure source and the Core tests in full.
- **P-B.** OPUS-SUCCESSOR-B read the backend `stage/*` and `lib.rs` lowering in full. B-COMPLETION closed G1–G3 and G6.
- Both premises hold because the current production bytes equal the reviewed `7d6c3965` bytes (verified above).
- Also relied on:
  - the per-blob sha256 projection of the 64 legacy exports;
  - the equality of the plain capture copies with their `.gz` decompressions;
  - the SDK compile-time public-verifier embedding residual, which is unchanged.

No author/classifier report counts as acceptance.

## Publication scope

My scope allows mechanical, verbatim publication of this completed report and its filtered receipts as a metadata-only commit. That commit would bind `dd4e4298`/`f954cd00` and the source hashes externally, as a **NEEDS_CHANGES** record. It is not an approval, and it asserts no downstream, native or parent qualification. The frozen snapshots stay immutable. It permits no product change. I perform none of it.
