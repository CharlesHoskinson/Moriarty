# OPUS-SUCCESSOR-B — fresh independent backend security review (scope B)

- Reviewer: Claude Opus 5.5 (`claude-opus-5-5`), high effort, fresh session. Not an author, finisher or classifier of any reviewed artifact. No other reviewers or orchestration were used.
- Candidate `7d6c3965b89d713575ade6a8dddc4f1780b3fdd3`, base `fe01bc8b7d889427192c304b0e75d0ec719f63db`, historical `e9a2b80b`. The worktree was clean at the start and was not changed.
- Startup: guarded `status --json` was run. It reported only the historical SP01 queue and no pending transactions. Authority is 0 and the counters were not touched.
- Nothing was built, tested, formatted, modelled or proved. No VM, native, network, key, PK, SRS or proving-cache access. The public baseline executable was not inspected.
- **Terminal status: SOURCE ACCEPT (scope B, local source-static `/2` only), ACTUAL-RESULTS ACCEPT for the backend captures with one binding gap (G1), DOCS INCOMPLETE (not reviewed).** No required finding is open. This is not acceptance of the whole feature, native qualification, or any U0–U7/M1–M17/MC/SP/G exit. All ten `/4` gates stay unperformed.

## Premises relied on (stated, not independently established)
- **P1. Design hashes.** `FROZEN-DESIGN.json` precedence is A2 > FINAL > CLARIFICATION > original > NONSECURITY. I recomputed the sha256 of all five design docs and they match the pins.
- **P2. Oracle hash.** `ORACLE-CONTINUATIONS.json` hashes to `76075b63…9640` in both D/oracle and R. The test fixture `normal-oracle.json` has the same hash, per SOURCE-SCOPE-CURRENT. I did not rehash that fixture copy.
- **P3. Root 169-blob binding.** I treat it as historical. I did not rely on it; I hashed the current backend files myself (below).
- **P4. Runtime stream copies.** The raw runtime files under `R/checks/*` are treated as the uncompressed streams of the retained `D/checks/*.gz`. This is **G1**: in this session the harness refused every `gzip -dc`, `zcat` and Python-gzip command. Binding is shown only by: (a) raw stream sha256 equals the `stdout_sha256`/`stderr_sha256` inside `result.json`; (b) `sha256sum -c D/SHA256SUMS` passes for all 417 entries, which covers the `.gz` bytes.

## Current byte verification (independent)
- `sha256sum -c SHA256SUMS`: all 417 lines OK.
- I hashed these current files and each matches SOURCE-SCOPE-CURRENT and the start pins of backend-normal-03 and backend-relation-02:
  - `mori-backend/src/{lib,proving,signed_relation}.rs`
  - `stage/{artifact,context,frame,lower,mod}.rs`
  - `tests/{stage_relation,stages}.rs`
  - Examples: `lower.rs` `0e3d7aac…`, `stage_relation.rs` `e6b35eeb…`, `lib.rs` `b081a6af…`.
- The e9a2→7d6c repair diff touches four crate files. In the backend it is `lower.rs`, a staged-only change of `+validate()` and comments, plus test additions. `lib.rs`, `proving.rs` and `signed_relation.rs` are unchanged since e9a2.

## SOURCE disposition: ACCEPT (scope B)

**Files read in full:**
- `stage/mod.rs`, `artifact.rs`, `context.rs`, `frame.rs` (including all six unit tests, 362–631) and `lower.rs`.
- `lib.rs` 500–1228: Builder, `public`, `range`, `checked_mul`, `expression`, `lower_action`, and the whole of `lower_action_segment` through statement lowering, the post frame, context, Output and the IR round-trip.
- The `lib.rs` changed hunks at 18–21 and 362–390.
- The `proving.rs` and `signed_relation.rs` diffs, `validate_shape` 505–565 (unsigned branch) and `/3` `context_hash`.
- `tests/stages.rs` 1–187 and `tests/stage_relation.rs` 1–453, every test body.

**Verified predicates (frozen A6/A7/A8/A1.1):**
- **Role positions, A7.**
  - The pre block is emitted after the pre-liability frame (`lib.rs:892-900`). Captures and events are bound from `PreFrame` only.
  - Inputs are bound only when `entry_site==0` (`lib.rs:807-814`). The guard is a single `Assert` on the evaluated wire (`frame.rs:283-291`, `lib.rs:901-906`).
  - The post block is emitted after the post-liability frame (`lib.rs:1143-1155`).
  - All five pending roles are emitted only via `frame::public_at`→`Builder::public` (`frame.rs:211-227`), which is H27 by construction.
- **Padding.**
  - Pre slots with `i>=len_in` get `expected=immediate(0)` and constant `"0"` (`frame.rs:251-260`).
  - Post slots with `i>=len_out` get immediate 0 and `"0"`. Live post slots get `expected=` the actual computed wire, and the start segment carries no host value (`frame.rs:338-350`, test `frame.rs:544-545`).
  - A Bool padding slot is boolean-checked by `range` and equals 0.
- **Typed wire.** `emit_post` refuses a live name or type mismatch (`frame.rs:320-327`). `assert_guard` refuses a non-Bool guard (`frame.rs:284`).
- **Effect ordinals.** `enumerate()` runs before `skip/take`, so `step` is the original ordinal (`lib.rs:909-913`). Tests: `stages.rs:81,95`.
- **Await refusal.** An Await inside a range returns a typed profile refusal (`lib.rs:923`). Segment ranges exclude await ordinals (`frame.rs:161-162`).
- **N+1 circuits.** `layouts` re-derives sites from source Awaits in order (`frame.rs:96-133`) and requires plan segment vectors of length N+1 (`frame.rs:86-94`). Captures are prefix-monotone (`frame.rs:142-152`) and `replay_steps` follows the A7 formula (`frame.rs:163-166`).
- **Zero-await.**
  - `zero(l)` is used and `emit_pre`/`emit_post` emit nothing (`frame.rs:232-237,302-308`).
  - The decoder N=0 branch refuses any pending role, width, capture, event or `Some` (`artifact.rs:351-363`).
  - `Option::deserialize` makes the key required (`artifact.rs:70-71`), per A1.1 C1.
- **Context (A8).**
  - The array has exactly 29 elements in the frozen order (`context.rs:66-96`). Element 1 is `"moriarty-stage-context/1"`, distinct from `/3` `CONTEXT_FORMAT`. `/3` has 18 elements, so the domains are separated.
  - It refuses a signed Target, a non-`/2` program, a registry mismatch, or a layout/action-name mismatch (`context.rs:32-52`).
  - Constants 0/1 come from this hash in the same way as `/3` (`lib.rs:1174-1181`).
- **Decoder (A6 rules 1–8, A1.1 C1).**
  - Rules 1–2 and 7: `artifact.rs:179-226`.
  - Contiguous runs and unique actions: `artifact.rs:230-245`.
  - Rule 3, sites: `artifact.rs:317-325`.
  - Rule 6, no inputs in advance segments: `artifact.rs:327-329`.
  - Rule 5, width-L prefix, continuity and the shrink ban: `artifact.rs:365-378`.
  - Segment 0 has no events: `artifact.rs:379`.
  - Rule 4, in range, distinct, roles/names/types/constants, disjoint from 0/1/effect/balance/liability positions, no stray pending role, contiguous at both insertion points, post block ends the ABI: `artifact.rs:397-501`.
  - Rule 8, logical committed-before chain and plan total: `artifact.rs:340-350,388-390`.
  - Totals and the byte fixed point: `artifact.rs:253-292`.
- **`/1` refusals.** `require_core1` is at the `compile`, `compile_signed` and `compile_mode` entries (`lib.rs:366,378,388`). It is also at `proving::financial_values`, `signed_relation::admit` and `reconstruct_statement`. Await arms return a typed refusal (A9) and there is no `unreachable!`.
- **Separate type.** `/4` is a distinct type with no `From`/`Into` with `/3` `Artifact`, so proving and signed consumers cannot take it (`stage/mod.rs:1-6`).

**Prior B required items:**
- **F1 — CLOSED.** `compile_staged` now calls `artifact.validate()` at the byte-count fixed point before returning (`lower.rs:125-128`). No unvalidated fresh output is returned. The 12-iteration bound refuses if the count does not stabilise (`lower.rs:117-132`).
- **T1 — CLOSED.**
  - A signed Target is refused at entry, before `load` (`lower.rs:23-28`). Test `stage_relation.rs:250-265` pins the code and message for a valid source and for an unparseable one, and shows the policy-stripped target compiles.
  - Context single-field perturbation (anchor, `max_instructions`, `max_artifact_bytes`, and an unread state initializer) changes every circuit context while program/plan hashes stay fixed (`stage_relation.rs:269-297`).
  - Domain separation (`stage_relation.rs:301-318`).
- **T2 — CLOSED, with the caveat that I could not locate the prior B report text defining T2 (no OPUS-*REVIEW*.md match).** I take T2 to be the exact-diagnostic forgeries. `stage_relation.rs:328-453` has 14 forgeries, each asserting its own predicate's message. I traced each one through `validate` order:
  - Each refusal is reachable before any earlier check fires.
  - The +2 `artifact_bytes` forgery can never be a fixed point.
  - The overlap forgeries hit the overlap check (`artifact.rs:456`) and not the range check, because the targets are valid positions.
  - Fresh round-trip admission covers zero-await, empty-capture width-0 and multi-await shapes (`stage_relation.rs:227-246`).

**Findings.** All are optional, non-blocking and inside the frozen known limits.
- **B-O1 (Low, optional, A6 rule 4 / "decoder syntax-only" known limit).** In unsigned mode, `validate_shape` checks only positions 0/1 (`signed_relation.rs:512-536`). The `/4` decoder never checks that effect, balance or liability descriptor positions are in range or carry matching roles. It only checks that they are disjoint from pending positions (`artifact.rs:441-458`).
  - Predicate: a decoded artifact may hold out-of-range or mis-roled non-pending descriptors.
  - Trace: this is not required by frozen A6, which constrains pending positions only. It is consistent with the documented absence of provenance binding (`lower.rs:19-21`).
  - Recommendation: add descriptor range/role checks before any consumer relies on decoded `/4`.
- **B-O2 (Info, optional).** The decoder cannot check that `event` binders equal the entry site's binders, or that advance segments have B≥0 binders matching source. There is no source in the decoder and no recomputation of program, plan or context digests. This is the known limit, and the gates `pending-frame-predecessor-binding`, `stage-event-authentication`, `stage-authority-and-consumption` and `arbitrary-witness-stage-soundness` remain unperformed.
- **B-O3 (Info, optional).** The context perturbation test does not cover every Target field (for example `network`, which is refused by the asset-domain check at `lower.rs:32`) or every frame term. By construction, all 29 elements are in the digest (`context.rs:66-96`).

## ACTUAL-RESULTS disposition: ACCEPT for backend captures, subject to G1

| Capture | Selection | Observed (raw stream) | Binding |
|---|---|---|---|
| backend-normal-03 | fresh | `stages.rs`: 5 passed, 1 ignored (the env-gated baseline test), exit 0 | stdout `ab8d27c7…`, stderr `0b8c05af…` = result.json. Start pins `lower.rs`/`stage_relation.rs`/`lib.rs` = current. |
| backend-relation-02 | fresh | `stage_relation.rs`: 9 passed, names = 4 original + 5 repair tests, exit 0 | stdout `15582f58…`, stderr `ca2e176f…` = result.json. Pins current. |
| backend-frame-01 | carried forward | 6 `stage::frame::tests` passed, 17 filtered | stdout `2a82f3b0…` = result.json |
| legacy-byte-comparator-01 | carried forward | 1 passed (`legacy_artifact_and_every_ir_file_match_baseline_bytes`), 5 filtered | stdout `c564debe…` = result.json |

**Carry-forward projection (CAPTURE-PIN-COMPARISON-CURRENT lines 142-202).** Both carried-forward captures list four pin deltas. I assessed each against the call path:
- **`lower.rs`.** The only change is `validate()` inside `compile_staged`. The frame unit tests (`frame.rs:432-631`) call only `layouts`, `emit_pre`, `emit_post`, `assert_guard` and `Builder::public`, never `compile_staged`. The legacy comparator (`stages.rs:156-170`) calls `mori_backend::compile`→`compile_mode`→`lower_action`→`lower_action_segment(.., None)`, which never reaches `stage::lower`. **Not called.**
- **`stage_relation.rs`.** A separate integration-test binary, not linked into the lib unit-test binary or the `stages` binary. **Not called.**
- **`stage_hostile.rs`.** A mori-core test binary. **Not called.**
- **`examples/.../README.md`.** Documentation only.

No rerun is required. The 5+9 fresh results are not added to, or summed with, the retained original failures. I did not verify the 78 = 49 + 29 total beyond the backend rows (5+9 fresh; 6+1 carried forward).

## DOCS disposition: INCOMPLETE (not reviewed — no approval)

## Unread required coverage (explicit gaps — not approved by absence of evidence)
- **G1.** Direct decompression of `D/checks/*/{start,result,stdout,stderr}.gz` was blocked by harness permission. Raw `R` copies were read and hash-bound only as described in P4. I did not read all of `start.json` (only the backend pins).
- **G2.** Legacy baseline: `LEGACY-BYTE-FIXTURE-MANIFEST.json`, `LEGACY-BYTE-FIXTURE-POSTCHECK.json`, the `legacy-baseline-export-01` capture, and the public artifact plus IR leaf-set hash/source projection `95df…eb2577` — **not read**.
- **G3.** Quint prefix and lifecycle sources, adapters, the retained 03 failure (QNT404), the import fix, all normal model captures and finite-field limits — **not read**. No model QA disposition is given.
- **G4.** OBSERVATIONS-CURRENT.json, README, and all docs: the product contract, the spec (UNI006/007/008, lines 59-90), SECURITY-CURRENTNESS, security-reconciliation and FINAL-NONSECURITY — **not read**. The current 84-path check manifest was not independently counted.
- **G5.** The full frozen hostile family list in DESIGN-OPUS.md and CLARIFICATION, beyond A10 and A2, was not cross-walked test by test. I did not read the original source-author and reviewer reports.
- **G6.** The capture-pin comparison was read for the backend-frame-01 and legacy-byte-comparator-01 rows only. The other 8 selected captures' pin rows were not read.

Scope A (Core/CLI internals, admit/validate/cost/decode, CLI commands, ledger closure) is explicitly excluded.

Because of G1–G6, the scope-B review **as a whole is INCOMPLETE**. Backend source acceptance and backend actual-result acceptance, as scoped above, stand independently. They do not constitute feature, model, documentation or native approval.
