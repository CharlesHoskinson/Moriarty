# Opus 5.5 completion review, Scope B: backend security and static relation

- Reviewer: Opus 5.5 (`claude-opus-5-5`), high effort, fresh, non-author.
- Candidate: `e9a2b80b3cdf5bc94b11dd4b7a12a0b0c9810629`. Base: `fe01bc8b7d889427192c304b0e75d0ec719f63db`.
- Worktree delta: only `examples/core/continuations/README.md`. I read it; it is documentation only. It keeps the three labels, keeps the `/4` gates unperformed, and cites no backend code.
- Startup: I loaded the develop SKILL, AGENTS.md and execution-focus. Guarded `status --json` ran from the inherited cwd and showed the historical SP01 campaign. That campaign is operational history only.
- Authority: native authority 0. I ran no Cargo, build, test, SDK, model, VM or prove commands. I did not reset any counter. I used only read-only source, git, `gzip -cd` and grep. I wrote only this file.

## Verdict: **INCOMPLETE**

There is one required source finding (F1) and two required test-coverage gaps (T1, T2). Some named evidence is also unverified (see "Unread / unverified"). I do not approve this subscope. This verdict covers Scope B only. The whole-feature integrated Opus review is still required.

## Reading coverage

Read in full at the candidate tree:
- `crates/mori-backend/src/stage/{artifact,context,frame,lower,mod}.rs`, including the six `frame::tests` units.
- `git diff fe01bc8b e9a2b80b` for `crates/mori-backend/src/{lib,proving,signed_relation}.rs`.
- `lib.rs` from about 780 to 900, covering the `lower_action_segment` prologue: context, pre_state, input, balances, liabilities, pre-frame, guard.
- `Builder::public` at `lib.rs:624-654`.
- `signed_relation::validate_shape` from 505 to 535.
- `crates/mori-backend/tests/stage_relation.rs`, lines 40–190 (all four tests).
- `crates/mori/src/commands/compile.rs` staged branch and `publish_staged_artifact`. I read these only to check the shared typed-interface assumption, not as a CLI review.

Read partially:
- `crates/mori-backend/tests/stages.rs`: test names and assertion lines only (grep). Bodies of lines 1–44 and 114–160 not read in full.

Actual results I read (decompressed stdout, at the candidate tree as captured):
- `backend-frame-01`: 6 passed, 0 failed, 17 filtered.
- `backend-relation-01`: 4 passed, 0 failed.
- `backend-normal-02`: 5 passed, 0 failed, 1 ignored. The ignored test is the legacy byte test (env-gated).
- `legacy-byte-comparator-01`: 1 passed. This is `legacy_artifact_and_every_ir_file_match_baseline_bytes`, 5 filtered.
- The earlier backend capture (4 pass, 1 fail, 1 ignored) stays separate and is not double-counted.

## Predicate validation (local, static, source-inspected; no native soundness claim)

| Predicate | Result | Where |
|---|---|---|
| Exactly 29 context elements | Holds. Count is 29. | `context.rs:66-96` |
| Domain separation from `/3` | Holds. The first element is `moriarty-stage-context/1`. The `/4` artifact, ABI and relation strings differ, and the `/3` context has 18 elements and is unchanged. | `context.rs:67-71` |
| No compiler identity in context | Holds | `context.rs:66-96` |
| Signed Target refused | Holds at both entry points | `lower.rs:19-24`, `context.rs:32-37`, decoder `artifact.rs:187-191` |
| Pending-frame insertion order | Holds. Pre block comes after the pre-liability positions. Post block comes after the post-liability positions and ends the ABI. | `lib.rs` prologue; `frame.rs:231,296`; decoder `artifact.rs:469-499` |
| Typed frame bindings | Holds. `Builder::public` range-checks each binding by type, so Bool events get `ConstrainToBoolean`. | `lib.rs:640`; `frame.rs:578-581` |
| Zero padding | Holds. Pre slots `i ≥ len_in` and post slots `i ≥ len_out` get `ConstrainEq(·,0)` plus constant `"0"`. | `frame.rs:251-265,338-352` |
| Capture origins | Holds. Live post captures are constrained to the actual env wire computed by the segment, not a host value. | `frame.rs:339-341`, `lib.rs` `live` collection |
| Live pre captures | Unconstrained public inputs. This matches the design: `untrusted captures`, `pending-frame-predecessor-binding` unperformed. | `frame.rs:1-7` |
| True guard | Holds. Lowered as a direct `Instruction::Assert` on a Bool wire. A non-Bool guard refuses before any push. | `frame.rs:283-291`, `lib.rs` guard block |
| Liability registry | Holds. The whole source registry is in every segment. Context rejects a registry that differs from the source registry. The decoder checks the count. | `context.rs:47-52`, `artifact.rs:330-332` |
| Disjoint roles | Holds. Pending positions are distinct, disjoint from 0/1 and from effect/balance/liability positions, and no stray pending role exists. | `artifact.rs:423-466` |
| `pending_positions` | Holds. It is a required key, `null` exactly when N=0, and forgery tests cover it. | `artifact.rs:69-71,351-385` |
| N+1 circuits | Holds. No circuit is emitted for a false guard. | `frame.rs:68-188`; `stages.rs` counts |
| Original ordinals | Holds. `enumerate()` runs before `skip/take`. Effect `step` keeps source ordinals (`stages.rs:81,95`). | `lib.rs` loop |
| Decoder validation | Present. It covers format/ABI/relation, unsigned mode, semantics, the three labels, logical profile, digests, the ten unperformed gates in order, contiguous action runs, sites, the capture prefix and continuity, no inputs in advance segments, and logical totals. | `artifact.rs:178-392` |
| Resource fixed point | Present. It uses the same 12-iteration `to_vec_pretty` procedure as `/3`, and the decoder recomputes it. | `lower.rs:113-126`, `artifact.rs:280-290` |
| Unsupported native flags | Holds. `/4` has no `From/Into` with `/3`, so proving, signed and native consumers cannot take it. `/1` guards were added to `compile`, `compile_signed`, `compile_mode`, `signed_relation::admit`, `reconstruct_statement` and `proving::financial_values`, and an Await inside a `/1` match returns a typed refusal. | `stage/mod.rs`, lib/proving/signed diffs |
| Report labels | Holds. `local-reference-stage` / `none` / `untracked` come from `STAGE_LABELS` and the decoder enforces them. | `lower.rs:95-97`, `artifact.rs:195-206` |
| `/1` and `/3` lowering unchanged | Holds in source. `stage=None` keeps the input loop, the full range and the 18-element context. Captured evidence: `legacy-byte-comparator-01` passed and `legacy_counter_abi...` passed. | `lib.rs` diff |

## Findings

**F1 — Required (medium).** `crates/mori-backend/src/stage/lower.rs:112-126`, together with `artifact.rs:177`.
- The `validate()` doc comment says the decoder predicate is "also applied to freshly compiled output."
- `compile_staged` returns the artifact without calling `artifact.validate()`, and the CLI (`compile.rs:22-29`) publishes it unvalidated.
- So compiled `/4` output is never checked against the decoder's frame, overlap and continuity rules before publication. Only the MIXED program is round-tripped, in a test (`stage_relation.rs:133-134`).
- Fix: call `artifact.validate()?` before `Ok(artifact)` in `lower.rs:122`. Otherwise, correct the comment and add decode round-trips for every compiled oracle. I recommend the call.

**T1 — Required hostile coverage.** No backend test exercises:
- the `compile_staged` signed-Target refusal (`lower.rs:19`);
- a context-hash change under single-field perturbation, which would demonstrate domain separation from `/3` and per-segment binding beyond the "distinct" check at `stage_relation.rs:116-127`.

The source is correct, but these are frozen security controls and need their own hostile tests.

**T2 — Required hostile coverage.** The decoder forgery set (`stage_relation.rs:151-173`) has six cases and asserts only `is_err()`. It has no cases for:
- capture continuity (`capture_in ≠ prev capture_out`);
- an `input` role in an advance segment;
- events in segment 0;
- a pending position overlapping an effect, balance or liability position;
- a pending block moved off its insertion point;
- `committed_before` or `plan_total` tampering;
- a label change;
- an `artifact_bytes` mismatch.

Assert the error code or message for each case so that the serde or byte-count layers cannot mask the targeted predicate.

**O1 — Optional.** `lower.rs:18`: `_out` is unused. That is fine, but the doc comment should say no filesystem effect occurs in the backend.

**O2 — Optional / recorded limitation.** The decoder (`artifact.rs:212-216`) checks `context_hash`, `program_hash` and `plan_hash` for digest syntax only. It does not recompute them and does not check zkir against `public_inputs`. A decoded `/4` artifact is therefore shape-valid, not provenance-bound. This is consistent with the unperformed gates, but downstream consumers must not treat decode as binding.

There is no source defect against the frozen design in frame layout, padding, guard, ordering, roles or ordinals. The design is not reopened.

## Unread / unverified (blocks approval)

1. `crates/mori-backend/tests/stages.rs` lines 1–44 and 114–160: bodies not fully read (the oracle sources and the `legacy_*` fixture path setup).
2. Legacy full-byte evidence: I did not read `LEGACY-BYTE-FIXTURE-MANIFEST.json`, `LEGACY-BYTE-FIXTURE-POSTCHECK.json` or the `legacy-baseline-export-01` capture. I did not verify export file hashes with `sha256sum`.
3. `R/CANDIDATE-CAPTURE-BINDING.json` and `R/CANDIDATE-CODE-CHECKSUMS.txt`: not independently checked (`sha256sum --check` not run). I did not open `start.json.gz`/`result.json.gz` for the four captures, so exit codes and command pins are unconfirmed by me; I read stdout only.
4. Rest of `lib.rs` `lower_action_segment` (after about line 900, outside the diff hunks): statement lowering and post-frame emission are seen only through the diff context.
5. Design hostile-family list in `D/design/DESIGN-OPUS-FINAL.md`/`A2`: not re-read in this pass, so T1/T2 are measured against the predicates listed in the prompt, not against the full frozen family list.

Out of scope and not reviewed: CLI and ledger beyond the shared `StagedArtifact`/`compile_staged` interface; Core `stage::{load,plan,validate}`. I assumed Core enforces that captures exclude state names and that plan invariants hold, and that `lower_action_segment` gets a sealed `StagedProgram` whose plan `frame::layouts` re-checks.

To close: fix F1, add T1/T2, and complete unread items 1–4. A follow-up Opus pass can then return APPROVE_SUBSCOPE.
