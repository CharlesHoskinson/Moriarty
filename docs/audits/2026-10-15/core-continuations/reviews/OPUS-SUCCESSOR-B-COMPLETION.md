# OPUS-SUCCESSOR-B-COMPLETION: scope B, G1–G6 only

- **Reviewer:** Claude Opus 5.5 (`claude-opus-5-5`), high effort, fresh session. I did not author, finish or classify any reviewed artifact. No other reviewers or orchestration were used.
- **Candidate:** `7d6c3965b89d…fdd3`, base `fe01bc8b`.
- **Startup:** HEAD was verified and the worktree was clean. Guarded `status --json` ran and showed only the historical SP01 queue with no pending transactions. Authority is 0 and the counters were not touched.
- **No execution:** nothing was built, tested or modelled; no Quint, SDK, VM, native, network or key/PK/SRS access. The baseline executable was not read, stat'ed or hashed.
- **Read-only:** I used only sha256sum, head/tail/grep and Read. I wrote only this file.
- **Harness limits this session:** Python, writing temp files and `git show`/`rev-parse` of object blobs were refused. Every comparison below is a direct sha256sum output compared with the recorded value.
- **Premise:** I rely on the prior scope-B report R/OPUS-SUCCESSOR-B.md (SOURCE ACCEPT, backend ACTUAL-RESULTS ACCEPT) as an independent premise, not as author approval. I did not reread the full backend.

## Terminal dispositions (scope B only)

| Area | Disposition |
|---|---|
| SOURCE (carried forward, current bytes verified) | **ACCEPT**, carried from OPUS-SUCCESSOR-B. The current `lower.rs` `0e3d7aac…`, `stage_relation.rs` `e6b35eeb…`, `lib.rs` `b081a6af…` and oracle `76075b63…` equal the fresh start pins. |
| ACTUAL-RESULTS (backend, legacy, carry-forward) | **ACCEPT**. G1, G2 and G6 are closed. |
| HOSTILE COVERAGE (G5) | **NEEDS_CHANGES (test-only)**. One required B gap (T3/H20). One required gap at the scope-A/consumer boundary (T4/H19). |
| MODEL-QA (G3) | **ACCEPT as finite normal-example QA only**. No security assurance or Rust correspondence. |
| DOCS (G4) | **INCOMPLETE**. One optional wording finding. The required normative reads listed at the end were not completed. |

None of this approves the whole feature, native qualification, any of the ten `/4` gates (all `unperformed`), or any U0–U7/M1–M17/MC/SP/G exit.

## G1: capture readability and binding (CLOSED)

**Plain copies match the manifest.**
- `R/public-capture-text/MANIFEST.json` lists 100 entries: 25 captures × {start, result, stdout, stderr}.
- I ran sha256sum on all 100 plain copies. Each hash equals its manifest `plain_sha256`, compared in order.
- Compressed-side equality is Muse's premise, plus the 417/417 `SHA256SUMS` result recorded in OPUS-SUCCESSOR-B.

**Stream hashes match `result.json`.** For every capture I inspected, the stdout/stderr hashes I computed equal the `stdout_sha256`/`stderr_sha256` in its `result.json`:
- backend-normal-03, backend-relation-02, backend-frame-01;
- legacy-byte-comparator-01, legacy-baseline-export-01;
- quint-prefix-01..04;
- quint-lifecycle-01, -02, -03, -03b, -04.

**Full start fields read** for backend-relation-02, legacy-baseline-export-01, quint-prefix-01/02 and quint-lifecycle-02. For the rest I read the argument and pin lines.
- `args`: cargo `+1.99.0 --offline --locked`, `-j2`, `--test-threads=1`, under `flock` and `timeout`.
- The comparator ran with `--ignored --exact legacy_artifact_and_every_ir_file_match_baseline_bytes` and `MORIARTY_STAGE_V1_BASELINE=R/legacy-byte-baseline`.
- `source_pins` and `classification_sha256` are present.
- `native_actions_requested: 0` appears in every start and result. This records that **0 native actions were requested**. It does not claim the instrumented SDK internals were 0.

**Raw results:**
- backend-normal-03: exit 0, 5.46 s, `5 passed; 0 failed; 1 ignored`. The ignored test is the env-gated comparator.
- backend-relation-02: exit 0, 2.67 s, 9 passed. The names match the 9 test fns in `stage_relation.rs`.
- backend-frame-01: exit 0, 10.73 s, 6 passed, 17 filtered.
- legacy-byte-comparator-01: exit 0, 0.77 s, 1 passed, 5 filtered.

## G2: legacy byte baseline (CLOSED, bounded)

- **Leaf set.** `D/legacy-baseline` holds exactly 4 leaves: `source.mori` `4ad04eed…`, `target.json` `8cd664ce…`, `artifact.json` `fc58d238…` and `000-deliver.zkir` `0afa370b…`.
- **Manifest match.** These equal `LEGACY-BYTE-FIXTURE-MANIFEST.json` `files`/`export_files`, the R `legacy-byte-baseline/*` copies, and `baseline-export/{artifact.json,000-deliver.zkir}`. All were hashed by me. The bidirectional leaf set is equal.
- **Postcheck.** `LEGACY-BYTE-FIXTURE-POSTCHECK.json` records the leaf set and hashes unchanged.
- **Source specimen.** `source.mori` equals `oracle-compat-v1.mori` and `examples/.../atomic-compatibility.mori`, so it is the `/1` compatibility specimen.
- **Export capture.** legacy-baseline-export-01 ran `baseline-mori compile`, exit 0. stderr is "Compiled 1 circuits into …/baseline-export" and stdout is empty. The binary hash `89ae66c4…` is recorded only. I did not inspect it.
- **Source projection.** `R/legacy-baseline-source/MANIFEST.json` (`bde9aca0…`) has `git_revision 95df7355…` and 64 file entries.
  - I hashed two members: backend `lib.rs` `4485b650…` and `proving.rs` `4417d42c…`. Both equal their manifest entries.
  - I could not run the git-blob cross-check against `95df` (harness refused). `eb2577d6` appears only as the PR52 merge-base in prior reports. I treat "Core/backend unchanged 95df→eb2577" as a root-recorded premise.
- **Comparator path.** `stages.rs:156-170` uses `mori_backend::compile`, the `/1` path. The current pins for `lib.rs` and `stages.rs` are unchanged from the capture.
- **Known limit, kept:** this is one specimen and one passing comparator. It is not a whole-corpus or CLI-byte regression claim.

## G3: Quint MODEL-QA (CLOSED for meaning)

**Sources read in full:** `continuation_prefix.qnt` (103 lines), `continuation_lifecycle.qnt` (201) and `continuation_lifecycle_test.qnt` (198).
- The `quint-prefix/` copies equal the lifecycle copies by hash. The prefix test `b99f5e91…` equals the capture pins.

**Oracle values checked by hand:**
- Escrow start: 90/10, duty 10, site 1, cost 5. Delivery(true): escrow→bob 10, duty 0, committed 11.
- Reserve start: held = 40+2 = 42, basis 35, duty 47, cost 11.
- Approval 41 < 42 waits with attempted 4. Approval 43 sets allocated 42, basis 78, 58/42, duty 5, committed 26.
- Settlement(true): fee 2 and Emit 43, 56/42/2, committed 36.
- Approval 51 gives basis 86. Settlement(true) is then BusinessRejected at step 9 with attempted 6, `c` unchanged.

All of these equal the README oracle tables (`README.md:52-89`).

**Import fix.** The diff of the historical and current test files is exactly two `import continuation_prefix.{Escrow,Reserve}` lines. This matches `QUINT-LIFECYCLE-TEST-IMPORT-FIX.patch`.
- lifecycle-03 pins the old test `9a923041…`: exit 1, four QNT404 "Name 'Reserve' not found". This failure is retained.
- 03b and 04 pin the new test `dc39c999…`: exit 0. 04 shows 9 named tests passing once each.

**Runs:**
- prefix-02: `--max-steps=1 --max-samples=4`, witness 4/4.
- prefix-04: 2 tests passing.
- lifecycle-02: `--max-steps=4 --max-samples=32`, witnesses 32/21/19/10, trace lengths 2–5.
- The model typecheck/run passes (prefix-01/02/03, lifecycle-01/02) were not rerun after the fix, which is correct.

**Meaning and limits.**
- These are mathematical `int`, finite normal-event, sampled positive witnesses. The source comment says "No invalid-event or hostile domain" (`continuation_lifecycle.qnt:71`), and there are no invariants.
- So there is **no security assurance, no exhaustiveness and no Rust correspondence**.
- The counts (2 tests / 4 samples, 9 tests / 32 traces) are correctly kept separate from the 78.

## G4: DOCS (INCOMPLETE)

**Read:**
- The dossier `README.md` (238 lines) in full, `OBSERVATIONS-CURRENT.json` in full, and `REVIEW-STATUS.json` (pending keys).
- `spec.md:58-90` (UNI-006/007/008), `MORIARTY-PRODUCT-CONTRACT.md` via a scope grep (lines 3–82 hits), and `SECURITY-CURRENTNESS-2026-10-14.md` via a scope grep.
- `SOURCE-SCOPE-CURRENT.json` path structure.

**Verified:**
- The manifest has 84 paths: 56 `files` (lines 21–406) and 28 `documentation_and_model_inputs` (415–550). There is 1 extra `path` key (line 7), which is the historical SOURCE-SCOPE.json reference.
- `successor_commit: PENDING_ROOT_FREEZE` is the expected self-reference and is bound externally to `7d6c3965`.
- The README claims are scoped correctly:
  - the three labels;
  - ten gates unperformed;
  - authority 0;
  - all parent gates open;
  - no ledger readback (`README.md:36-41`);
  - "bounded specimen" (`:151-153`);
  - Quint not exhaustive or corresponding (`:174-175`);
  - reviews PENDING (`:9`, `:221-223`).
- No authority or consumption promise is made. UNI-006/007/008 are not claimed closed. The README frames the work as a local static subset only.

**Finding:**
- **D-O1 (Low, optional).** `README.md:135-137` says the checks include "both failed runs … and all four Quint commands". The retained set actually has **three** failed captures (cli-normal-01, backend-normal-01, quint-lifecycle-03) and **nine** Quint captures. The lifecycle failure is disclosed separately at `:169-172`, so nothing is hidden, but the sentence is inaccurate.
  - Trace: OBSERVATIONS-CURRENT accuracy (FINAL-NONSECURITY doc accuracy).
  - Fix: "all three failed runs … and all nine Quint commands."

## G5: hostile family crosswalk H15–H27 (precedence A2 > FINAL > CLARIFICATION > original)

| H | Frozen predicate (source) | Backend evidence | Status |
|---|---|---|---|
| H15, H16, H17, H22–H26 | Core/CLI (DESIGN-OPUS.md:292-299, FINAL:333-336) | — | **Scope-A boundary**, not reviewed by B |
| H18 | Signed Target `/2` → `unsupported` (OPUS:295) | `compile_staged_refuses_signed_target` (`stage_relation.rs:250-265`), passed in relation-02 | Covered (backend). The "no out directory" part is CLI, scope A. |
| H19 | `CompileCall` with `/2` refused before emission (OPUS:296; CLARIFICATION:181 assigns it to `stage_relation.rs`) | Source guard at `mori-ledger/src/static_compile.rs:370`. No test in any `crates/*/tests` contains `moriarty/2` together with compile-call. | **T4 gap, see below** |
| H20 | `/3` decoder on `/4` **and vice versa** rejected (OPUS:297; FINAL:227) | `/4` decoder refusing `/3` strings: `staged_versions_are_domain_separated_from_v3` (`stage_relation.rs:301-318`). `/3` decoder on a `/4` manifest: none. | **T3 gap** |
| H21 | Structural: pending bindings have an expected wire or constant, padding is zero-constrained, `s0≠s1` contexts, both ≠ `/3` (OPUS:298) | `pending_roles_occur_only_at_recorded_positions` (`:64-112`), frame units (`frame.rs:544-545` per premise), `contexts_are_per_circuit…` (`:116-127`), `contexts_bind…` and domain separation (`:269-318`) | Covered, structural only (OPUS:301) |
| H27 | Pending roles emitted only via `frame.rs` (FINAL:268,336; A2:52) | By construction, `frame::public_at`→`Builder::public` (premise from OPUS-SUCCESSOR-B), plus the role/position equality test (`:64-112`) | Covered. This is construction plus position test, not builder introspection. Optional limit. |
| Decoder rules | A6 (14 forgeries) | `decoder_refusals_name_the_targeted_predicate` (`:329-453`): each forgery asserts its own message and `forged≠honest` | Covered (T2 closed) |

**T3. Required, Low, test-only.**
- Predicate: no test feeds a serialized `/4` StagedArtifact to the `/3` `Artifact` decoder and asserts rejection.
- The source holds by construction: `decode_version` requires the exact `/3` strings (`crates/mori-backend/src/lib.rs:291-310`).
- Trace: DESIGN-OPUS.md:297 H20 "and vice versa"; FINAL:227; CLARIFICATION:181.
- Fix: add one relation test, `serde_json::from_value::<mori_backend::Artifact>(to_value(compile(MIXED)))` must be `Err` and name `ARTIFACT_FORMAT`.

**T4. Required, Low, test-only; consumer path, scope-A/consumer boundary.**
- Predicate: no captured test runs CompileCall or `static_compile` on a `/2` program.
- The source guard exists at `static_compile.rs:370` and `relation/mod.rs:361`.
- Trace: DESIGN-OPUS.md:296 H19; CLARIFICATION:181.
- If scope A's completion identifies an existing test, this closes. My grep found none.

**Optional, kept as known limits:**
- B-O1, B-O2 and B-O3 from OPUS-SUCCESSOR-B.
- H27 is shown without builder introspection.
- H1–H14, H24 and H28 are scope A (H28 is closure, `ledger-source-closure-02`).

## G6: pin comparison and 78 binding (CLOSED for B; A rows by premise)

**Totals.** All 10 rows of `execution-inputs/CAPTURE-PIN-COMPARISON-CURRENT.json` were read.
- Pin counts: 49×4 + 138×3 + 58 + 101 + 81 = **850**. Streams: 10×2 = **20**. Each row has exit 0 and stream hashes that match.
- Passes: 5+4+13+22+5+9+7+6+6+1 = **78**.
  - Fresh: 22+5+9+7+6 = **49**.
  - Carried forward: 5+4+13+6+1 = **29**.
- Scope **B = 21**: 14 fresh (backend-normal-03 5 + relation-02 9) plus 7 carried forward (frame 6 + comparator 1).
- Scope **A = 57**: 35 fresh (core-hostile-02 22, cli-normal-03 7, ledger-source-closure-02 6) plus 22 carried forward (core-normal 5, compat 4, static-cost 13).
- Superseded runs (backend-normal-01/02, relation-01, cli-01/02, core-hostile-01, closure-01) are not counted. The failures stay historical.

**Deltas.**
- **README `ece5958…`/`6aa01dfb…`→`905fa30f…`** (all 10 rows). I hashed the current README; it equals `905fa30f…`. It is not referenced by any stage test; the only README hit in tests is unrelated `init.rs:143` scaffolding. Documentation only, not called.
- **`stage_hostile.rs` `56f04e4f…`→`e8d3af12…`** (core-normal, compat, static-cost, frame, comparator). It is a separate mori-core test binary, so it is not linked into the backend lib-unit or `stages` binaries. For the frame and comparator rows: not called. For the Core rows: scope A assesses, and I do not substitute an opinion.
- **`lower.rs` and `stage_relation.rs`** (frame, comparator rows). Not called, per the OPUS-SUCCESSOR-B call-path premise. I verified the current hashes `0e3d7aac…` and `e6b35eeb…`.
- **No delta is hidden.** ledger-source-closure-02 has 0 deltas.

## Unread or unverified (blocks DOCS approval; not approved by absence)

1. `openspec/.../security-reconciliation-2026-10-14.md` (253 lines): not read.
2. `design/FINAL-NONSECURITY-DESIGN.md`: not read in full.
3. `docs/CORE-CONTINUATIONS.md`, `ROADMAP.md` and `examples/core/continuations/README.md`: only header/grep reads.
4. The product contract and SECURITY-CURRENTNESS were read only by grep, not line by line.
5. The 84 current manifest hashes were not each recomputed. I verified only the pins named above.
6. The git-blob check of the 95df projection was harness-refused (only 2 of 64 member hashes were checked).
7. The `quint-*/ADAPTER.md`, `LIFECYCLE.md` and `INPUTS.json` files were not read; their hashes are pinned in the start files.

**Net for scope B:** SOURCE and ACTUAL-RESULTS ACCEPT; MODEL-QA ACCEPT (limited); hostile coverage NEEDS_CHANGES (T3 required, T4 required at the boundary); DOCS INCOMPLETE. The integrated Opus review must still combine A and B. No native or full-parent approval is given.
