# REPORT: review repair of the current-Core → arkworks 0.4 R1CS compiler (SOURCEONLY)

**Author:** `claude-opus-5-5`, acting as security compiler review-repair author, SOURCEONLY. High effort was requested; I cannot observe the effort level. No other model, subagent or auxiliary tool was used.

**Terminal status:** `REPAIR_SOURCE_COMPLETE_UNEXECUTED_UNREVIEWED`. The successor is unadopted and uncompiled. It needs a new exact full-candidate and delta Opus audit before any E0–E5 command. Nothing here was built, type-checked, tested, set up, keyed, proven or run against a network or ledger.

The origin r2 report is kept verbatim at `history/origin-r2/REPORT.md`.

## 1. The falsifiable predicate and its result

> The mandatory F-01, F-04 and request fixes, and all other findings, have exact changed-byte or explicitly-open dispositions, and no target frame or admission gap is labelled as solved.

**Result: met at the source and documentation level.** Every one of F-01…F-14, the reviewer's C4/C9/C15 amendments and PC-1…PC-4 has a single row in `RESPONSE-MATRIX.md`, marked CHANGED (with exact files and `DIFFS/`), OPEN or RETAINED. F-02, F-03, F-05, F-10 and F-14 stay OPEN. F-04 is relabelled to a target premise and is explicitly **not fixed**. No text in the successor claims frame completeness, P3 coverage, debit authority, ABI coverage or target enforcement.

What would falsify this: a finding without a row, a CHANGED row whose diff does not contain the claimed change, or any successor text that presents F-02/F-04/F-05, frame completeness, Emit enforcement or P3-7 as solved. `RESPONSE-MATRIX.md`, `DELTA.json` and `DIFFS/` are the inputs for that check.

## 2. Separate axes

| Axis | State |
|---|---|
| Implementation | **F-01:** test-only rename. **F-11:** test rename and docs. **F-12:** length validation in `circuit()` plus a new error variant and 2 new tests. **F-04/F-05/F-10:** coverage data, target-obligation constants and docs. The constraint-generating source is byte-identical. |
| Evidence | Source-inspected plus my own lexical checks: `REPAIR-STATIC-CHECKS.json` passes 12/12 on pass 3; passes 1–2 failed on checker defects and are kept. Inputs re-verified by hash (`INPUT-VERIFICATION.json`). There is **no** compile, test, Core or relation execution. |
| Acceptance | None. A fresh Opus full-candidate/delta audit, root authorization and an authorized run are all required. Every U0–U7, M1–M17, MC01–MC08, SP01–SP12 and G01–G24 exit stays open. |

## 3. Startup, authority and inputs

- **Skill and governance.** I read the installed `moriarty-dev` develop skill and `references/execution-focus.md`. I read checkout `AGENTS.md`, `docs/MORIARTY-PRODUCT-CONTRACT.md` and `docs/audits/2026-10-14/SECURITY-ROUTING.md` in full. In `ROADMAP.md` I read the headings and the U2, U3, U6 and U7 rows (the U4/U5 lines were truncated in my search output and were not read). In the consolidated-kernel `spec.md` I read the requirement index and UNI-017 to UNI-022.
- **Guarded status.** The root refreshed it at 13:19. The status record I could see (`guarded-status-20261015-latest.json`) names historical SP01.6 loan-swap blockers. That is operational history and authorizes nothing here. I did not run the CLI, because Bash is limited to `python3` and the root did the refresh. Native authority is zero.
- **Review.** I read `REPORT.md` `a0d1725f…` (hash verified), the corrected request `80aa3bae…`, `HELDOUT-FREEZE.json`, both post-freeze addenda, `REVIEW-MANIFEST.json`, the lock tools and the harness. They were read as data; no reviewer script was imported or run.
- **Candidate.** I read every origin src file and test file, plus Cargo.toml, the manifests, COVERAGE, DESIGN-VOTES, REPORT, FAILURES and EXECUTION-REQUEST.
- **Later ABI review.** `claim-utxo-abi-v1-independent-source-review-r2-20261015/REPORT.md` is `f3ae8556…` (V6). I read its §5 votes.
- **Core.** I re-read `runtime.rs:586-664` for the F-04 and F-05 premises.
- **Verified unchanged** (`INPUT-VERIFICATION.json`):
  - origin SOURCE-MANIFEST `063beb95…` (69/69 files) and BUILD-MANIFEST `2c943b51…` (16/16);
  - the Core closure `f47318a2…`, recomputed, with 70/70 live files matching at HEAD `6c7b1fda…` (read from git metadata);
  - the freeze aggregate `7f6a4d1e…`, recomputed, with 17/17 files;
  - the vector counts: 61 vectors (30 accepted / 31 rejected), 9 static refusals, 26 hostile proposals;
  - the review manifest, 42/42;
  - the lock union: 109 packages, of which 102 are registry crates whose `.crate` hashes equal their lock checksums;
  - the toolchain bytes, equal to the reviewer's observations (hashed, never executed).
- **Not opened:** any `provider-result.private.json`, private reasoning, keys, PK/SRS or proof caches.

## 4. What changed (exact bytes)

The materialization (`COPY-PINS.json`) byte-copied all 69 origin-manifested files to the same paths and verified each one. The 15 files I later edited or replaced, plus the origin SOURCE-MANIFEST, are also kept verbatim under `history/origin-r2/`. `DELTA.json` and `DIFFS/` hold the complete delta.

| File | Change | Finding |
|---|---|---|
| `tests/common/mod.rs` | `pub fn compile(` → `pub fn compile_ok(` | F-01 |
| 7 test files | 24 bare `compile(` → `compile_ok(`; `duties_frame.rs:158` → `mori_r1cs::compile::<ark_bls12_381::Fr>(…)` | F-01 |
| `tests/generic_pipeline.rs` | test renamed to `compile_source_is_deterministic_and_never_reads_program_names`; the module doc names both public paths | F-11 |
| `tests/assignment_shape.rs` (new) | `well_shaped_assignments_still_reach_the_constraint_system`, `malformed_assignment_lengths_are_refused_before_synthesis` | F-12 |
| `src/circuit.rs` | `circuit()` calls `check_assignment_lengths` (inputs, pre/post body, effects, public); `BodyValues` import | F-12 |
| `src/error.rs` | `CompileError::AssignmentShape(String)` + `Display` | F-12 |
| `src/coverage.rs` | `Support::TargetPremise`; the missing-balance REFUSALS row `Constant` → `TargetPremise`; balances and liabilities field rows | F-04, F-05 |
| `src/adapter.rs` | TARGET_OBLIGATIONS: balance-row premise, duty-frame gap, P3-7 disputed wording; `initial_ledger_target_admits` doc says it is not wired | F-04, F-05, F-10, P3-7 |
| `COVERAGE.md` | the same corrections in prose; the refusal semantics now say "except one premise row" | F-04, F-05, F-10, F-12 |
| `BUILD-MANIFEST.json` | regenerated mechanically: `2c943b51…` → **`4e3ba2e2796b425598b9b2abc022c4d6782bf032044aa53e124af4946c543b47`** | identity |
| `REPORT.md` | this report (the origin's is in history) | — |

**Unchanged on purpose** (`REPAIR-STATIC-CHECKS` C5, C6):

- `Cargo.toml`, `gadgets.rs`, `lower.rs`, `plan.rs`, `statement.rs`, `r1cs.rs`, `witness.rs`, `identity.rs`, `frontend.rs`, `staged.rs`, `lib.rs` and `groth16.rs`;
- in `circuit.rs`, the functions `compile`, `compile_source`, `probe`, `synthesize` and the synthesis helpers.

No arithmetic, evaluation, memo, cost or constraint code changed.

**Frozen-oracle invariance** (C3). After undoing the helper rename, every line of the 7 inherited test files and `common/mod.rs` equals the origin, except the enumerated F-11 doc, comment and name lines. No `assert`, `matches!` or `panic!` line changed. The 39 inherited tests keep their assertions, and 2 repair-author tests are added, for 41 in total.

**F-12 invariance by reading.** No candidate test and no reviewer-harness row builds an `Assignment` except through `CompiledRelation::assignment`, which already enforces all lengths. Afterwards they only assign `public` elements in place, which I checked by search. The new check therefore cannot change any frozen outcome. Value types, absent-duty canonicality and every value relation stay with the constraints.

## 5. Build identity

The full record is in `BUILD-IDENTITY-DELTA.json`.

- **Why BUILD-MANIFEST changed.** The mandatory F-04 relabel and the F-12 validation are under `src/`, so PC-1's literal "BUILD-MANIFEST must stay 2c943b51" cannot hold. PC-1 is amended to PC-1r.
- **Bound in the circuit today.** These are header constants: the relation version, Program hash, profile digest, action index, shape digest, `compiler_build_digest` (over the caller-supplied `{BUILD-MANIFEST, Core closure}` digests) and W. A different supplied build digest changes the binding constants and therefore every `r1cs_sha256`, even though no constraint code changed. I did not compute the new digests.
- **Not bound anywhere.** The Cargo.lock, toolchain identity, compiled crate closure, and the link from R1CS to a VK.
- **Proposed only.** E0 recomputes the BUILD-MANIFEST and closure digests. Future catalogue/admission metadata would bind the lock digest, registry-subset digest, toolchain hashes and the observed r1cs digest. This is metadata, not an in-circuit field. Putting it into `CompilerBuild` would need its own vote.
- **The reviewer harness's hard-coded `2c943b51…` is stale for the successor.** Outcomes are unaffected because the encoder and the circuit read the same supplied constant. Its identity columns are not evidence. Only the reviewer may revise the harness.

## 6. Corrected source-only execution request

`CORRECTED-EXECUTION-REQUEST.json` is a proposal only and authorizes nothing.

- **PC-0, new.** A new exact full-candidate and delta Opus audit is required before any command.
- **Inherited counters, intact.** Zero mori-r1cs runs of any candidate, zero authorized runs, zero retries, and native authority 0. No budget is created or reset, and other lanes' counters are untouched.
- **F-06.** Absolute 1.99.0 binaries and `RUSTUP_TOOLCHAIN`. The re-hashed cargo, rustc and `librustc_driver` match the reviewer's observations, and the rustup default `stable` is recorded.
- **F-07.** A checksum-level subset over the 102 reviewed registry rows, pinned by a registry-subset digest. E1hv gets a proposed pair comparator, with controls run on synthetic locks.
- **F-08.** "Already bound in circuit" is separated from "proposed metadata".
- **E4** expects 41 named tests. **E5** declares the stale harness build constant.
- **Copied by value.** These reviewer blocks are carried over unchanged, so identical-text votes can be counted mechanically: `isolated_layout`, `toolchain.selection`, `toolchain.control`, `environment`, `sandbox`, `never`, `interpretation`, E1, E1v, E1h, E2, E3a, E3c and scoring. `text_identity_with_reviewer_request` records the check.

## 7. Votes

The detail is in `REPAIR-VOTES.md`. These are design and resource votes by the repair author, not an independent review.

- **AGREE (literal)** with the reviewer's C4, C9 and C15 amendments. Each now has 2 votes on identical text. My C9 additive clause (metadata, not in-circuit) has 1 vote.
- **PC-1:** AMEND, because the BUILD-MANIFEST invariant cannot hold.
- **PC-2 and PC-3:** AGREE (literal), 2 votes each, plus additive PC-2a and PC-3a with 1 vote each.
- **PC-4:** AMEND, citation erratum only (the votes are in §8/§10, not §9).
- **Request blocks copied by value:** AGREE (literal), 2 votes each.
- **D-13 value:** ABSTAIN. The split is preserved.
- **P3: no vote.** Every disagreement is preserved. V6 `f3ae8556`'s P3-7 CHANGE (authority over every debited account including custody, thread-advance authority, signed intent in the statement) is **not** overruled by the compiler review's AGREE on the older external-account wording. That AGREE does not cite V6, and the two reviews overlapped in time (12:19–13:01Z versus 12:46–13:21Z).
- **Source wording.** The successor's TARGET_OBLIGATIONS no longer implies external-only authority. Reconciliation is deferred to the source owner.

## 8. Required gaps (none waived; no whole-language percentage)

- **First-slice gaps:**
  - F-02 external duties;
  - F-03 conservative bound;
  - F-04 balance presence (the premise is labelled, not fixed);
  - F-05 duty frame and extra rows;
  - F-10 Emit policy wiring;
  - F-14 rejection relation.
  
  A precise duty-frame and presence-bit extension is proposed separately in `DUTY-FRAME-PROPOSAL.md`. It is not implemented and does not widen first-slice execution.
- **No complete P3 frame or ABI coverage is claimed.** DirectFieldV0 stays test-only, with no claimant or outref binding, and is never a claim ABI. No ABI v1 mapping exists while that ABI's full review is pending. Emit stays lowered, and a future target adapter must actually wire the initial-ledger whole-program refusal.
- **Still required:**
  - all of `/2` (staged lowering, codec correspondence, waits, captures, residuals, logical-cost/2, Idle/Pending, Emit, NotAdvanced, D-13);
  - all 20 patterns and the general obligations;
  - complete physical effects (P3), debit authority and signed intent;
  - residual duties;
  - recursion and PCD;
  - private witnesses and handoff;
  - multi-parent composition;
  - the ACTUS and DeFi memberships;
  - Midnight ZKIRv3 target qualification;
  - every U, M, MC, SP and G exit.
- **Scope of this lane.** This is Cardano eUTxO BLS12-381/Groth16 research, not ZKIRv3 or Midnight evidence. There is no adoption and no target claim. Maintainer review and resource controls never become public developer permissions.

## 9. Failures and limitations

The full record is in `REPAIR-FAILURES.json`.

- One guard denial: a compound shell command with `$?`. It was not performed and not retried in that form.
- Two checker-defect passes, both kept.
- **Main limitation: nothing compiled.** F-01 being resolved, the F-12 code type-checking and the 41 tests passing are predictions. The root's own record shows another lane's clean static review contradicted by an actual compile (E0277).
- The lexical checks cannot see type or trait errors.
- The F-12 expectations are mine and not independent.
- The author, reviewer and repair author are the same model family.

## 10. Files

| Path | Purpose |
|---|---|
| `REPORT.md` | This report |
| `RESPONSE-MATRIX.md` | Per-finding dispositions, votes and mandates |
| `REPAIR-VOTES.md` | Literal votes and reasons; P3 positions preserved |
| `CORRECTED-EXECUTION-REQUEST.json` | Source-only proposal; no authority |
| `BUILD-IDENTITY-DELTA.json` | BUILD-MANIFEST change; what is bound in the circuit versus proposed |
| `DUTY-FRAME-PROPOSAL.md` | Separate unimplemented F-04/F-05 extension proposal |
| `REPAIR-FAILURES.json` | Failures, deviations and limitations |
| `REPAIR-EXPECTATIONS/` | F-12 repair-author expectations and their freeze (13:33:19Z) |
| `REPAIR-STATIC-CHECKS.json`, `static-passes/REPAIR-STATIC-CHECKS-PASS-{1,2}.json` | Lexical checks (pass 3 final; earlier passes kept) |
| `INPUT-VERIFICATION.json` | Hash verification of all inputs |
| `COPY-PINS.json` | Per-file copy pins (69 files) |
| `DELTA.json`, `DIFFS/` | Complete delta against the origin manifest |
| `history/origin-r2/` | Verbatim origin bytes of every edited or replaced file, plus the origin SOURCE-MANIFEST |
| `BUILD-MANIFEST.json` | `4e3ba2e2…3b47` (Cargo.toml + src/**) |
| `SOURCE-MANIFEST.json` (+ `.sha256` sidecar) | Every successor file except harness-owned and private ones; its digest is in the sidecar and the final response |
| `repair-tools/` | My stdlib scripts (they import no candidate or reviewer code) and synthetic lock controls |
| Unchanged origin files | `EXPECTED-PACKET/`, `fixtures/`, `CORE-CLOSURE-PINS.json`, `EXECUTION-REQUEST.json`, `FAILURES.json`, `STATIC-CHECKS.json`, `DESIGN-VOTES.md`, `MATH-AND-SPEC-ATTRIBUTION.md`, `STAGED-LOWERING-PLAN.md` and the other origin records, byte-identical as history |

Requested model: `claude-opus-5-5`, high effort. Acting model: `claude-opus-5-5` (runtime-stated). The harness records the terminal identity and usage in `PROCESS.json`.
