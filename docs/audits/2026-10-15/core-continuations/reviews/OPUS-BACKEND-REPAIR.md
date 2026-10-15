# Opus 5.5 bounded backend source repair (F1, T1, T2): handoff

## Identity and authority
- Author: Opus 5.5 (`claude-opus-5-5`), high effort. This is security source authorship only. It is not review, approval or acceptance.
- Candidate: HEAD `e9a2b80b3cdf5bc94b11dd4b7a12a0b0c9810629`. The worktree's only delta is `examples/core/continuations/README.md` (docs). `git diff --quiet HEAD -- crates/` was clean before and after this work.
- Startup: I read AGENTS.md and the develop SKILL, and ran guarded `status --json`. It shows the historical SP01 campaign, which is operational history only.
- Inputs: `OPUS-COMPLETION-BACKEND.md` (F1, T1, T2, O1, O2).
- I did not mutate the worktree. I ran no Cargo, build, test, fmt, Quint, SDK, model, VM, prove, keygen, native, ledger or network actions. Native authority stays 0, and no counters were touched.
- Writes went only to `R/opus-backend-repair/` and to this file.

## Preimages (worktree = HEAD blobs)
| Path | sha256 |
|---|---|
| `crates/mori-backend/src/stage/lower.rs` | `db8ba77a5e742a6461894e3421263f7b1550bb1c1abc901b41cb34fd0423ae28` |
| `crates/mori-backend/tests/stage_relation.rs` (7234 B) | `498d7a08df334247f8ab40a7069a3e46a3031c56b68c14b39422ceb83ccd316e` |

## Payload (`R/opus-backend-repair/`)
| File | sha256 |
|---|---|
| `lower.rs` (full replacement) | `0e3d7aacc96b4379a235761bd25fb9478c4ab6ff70d7cd3649acb11b7e796724` |
| `stage_relation.rs` (full replacement) | `e6b35eebba3d6729230abbdf2c5c3f60f17340ea6f985ac33f390bd9b1cf6926` |
| `lower.rs.patch` | `54dc844430f1a844cb02b81d6747b1729c698f493fd1e64b7f714ba7512045af` |
| `stage_relation.rs.patch` | `e30d9180b75514ab626ebad7233e3d4b3c641f28dc5d7692cbee61cd01a7aaa6` |
| `OPUS-BACKEND-REPAIR.patch` (the two patches concatenated, `diff -u`, a/ b/ labels) | `bddb590f06054ca7a431cd8c0af04ce7c1a226767388b3d3948098329a9d67f5` |

Integrator: copy the two full files verbatim. Then check that each target hash matches, or apply the combined patch with `-p1` against the preimages above.

Only two files are touched. No NEEDS_SOURCE.

## F1 fix (`lower.rs`)
- In the fixed-point loop, once `count == artifact_bytes`, the code now runs `artifact.validate()?;` before `return Ok(artifact)`. The CLI (`compile.rs:22-29`) therefore cannot publish anything the `/4` decoder would refuse.
- `artifact.rs:177` says the predicate is "also applied to freshly compiled output". That is now true, so `artifact.rs` needs no change.
- O1 and O2 are folded into the doc comment, which is ordinary non-security documentation:
  - The backend performs no filesystem effect.
  - Fresh output is validated before it is returned.
  - The predicate is shape-only. It does not recompute the program, plan or context digests, so neither a decoded nor a fresh artifact is provenance-bound.
- No `/1` or `/3` code is touched. `lower.rs` is staged-only, and legacy lowering and its bytes are unchanged.
- Validation runs after the fixed point, so the returned artifact's `artifact_bytes` equals what `check_totals` recomputes from the same `to_vec_pretty`.

**Non-narrowing argument (source facts):**
- `stage::load` already refuses non-core/2 programs (`mori-core stage/mod.rs:73`), so the `SEMANTICS_2` check cannot newly reject anything.
- An empty program gives empty runs and empty logical metadata, and still passes.
- Zero-await circuits take the `n == 0` branch, which requires exactly what `frame` emits.
- `frame::layouts` already enforces prefix-monotone captures (`frame.rs:141-152`). That is what the decoder's width-L prefix and continuity rules require.
- The MIXED round-trip already passed at the candidate. The new test adds RESERVE (two awaits) and EMPTY_CAPTURE (an await with an empty capture layout, plus a zero-await action) as positive controls.

If validate ever refused fresh output, the compile would fail closed. It would not publish.

## Tests: 5 new, 0 changed, all unexecuted
- The old file is an exact byte prefix of the new one: the sha256 of the first 7234 bytes is `498d7a08…6e`, equal to the preimage. The diff has 0 removed lines.
- The 4 original tests, including `decoder_refuses_forged_frames_and_fields`, are unchanged. The oracle sources (RESERVE, MIXED) are unchanged.
- The expected `stage_relation` count goes from 4 to 9.

New tests:
1. `fresh_validation_admits_every_valid_frozen_shape`: RESERVE, MIXED and EMPTY_CAPTURE all compile through the new validate. Each round-trips byte-identically, and `artifact_bytes` equals the envelope length. For EMPTY_CAPTURE, `tick` has 2 circuits with width 0 and empty pre_captures, and `idle` has `pending_positions = None`.
2. `compile_staged_refuses_signed_target` (T1): uses the signed Target from the static `include_str!` golden vector, which signed_relation tests already use.
   - For both MIXED and a garbage source, it asserts the exact code `mori::backend::unsupported` and the exact message. The garbage source shows the refusal happens before loading.
   - Positive control: the same Target with `signed_policy = None` compiles.
3. `contexts_bind_single_target_and_program_fields` (T1):
   - Each single-field change (`anchor`, `max_instructions + 1`, `max_artifact_bytes - 1`) changes every circuit's context, while program_hash and plan_hash stay equal.
   - Changing the unused `marker` initializer from 77 to 78 changes program_hash and every context.
   - Identical inputs reproduce identical contexts.
4. `staged_versions_are_domain_separated_from_v3` (T1):
   - The context format is `moriarty-stage-context/1`, and none of the four `/4` strings equals a `/3` string.
   - A `/4` envelope with a `/3` `format`, `abi` or `relation` refuses with the exact "Expected …/4 …" artifact message.
5. `decoder_refusals_name_the_targeted_predicate` (T2): uses honest RESERVE, which decodes. Each of the 14 forgeries asserts `forged != honest` and that the error contains the full `mori::backend::artifact: <message>`. Preconditions are asserted on the honest artifact first: 3 circuits, `c1.capture_in == c0.capture_out` and non-empty, event non-empty, `public_inputs[2].role == "pre_state"`, and non-empty effect, balance and liability frames.

How the expected messages follow from the validation order (`artifact.rs:178-501`):
- The checks run in this order: version, unsigned, semantics, labels, profile, digests, gates, runs and logical length; then, for each segment, sites, `validate_shape` (positions 0/1 and the unsigned descriptor-empty check only), input role, liability length, balance identity, logical, captures, events, and `check_pending` (widths, bindings, distinct, overlap, stray, contiguity); then plan_total; then totals; and last, `artifact_bytes`.
- Every forgery keeps the exact serde shape, so `deny_unknown_fields` cannot fire.
- The forgeries and their expected refusals:
  - continuity: popping from `c1.capture_in` leaves a valid prefix but breaks `== c0.capture_out` → "Capture layout of segment 1 of \"allocate\"…".
  - input role: `c1` position 2 relabelled `input` → "Advance segments have no input positions."
  - events in segment 0 → "Event layout differs…".
  - overlap: an effect position set to `pre_site`, a balance `pre_position` set to `post_site`, or a liability `post_amount_position` set to `pre_captures[0]`. Bindings stay intact, so the first failure is overlap → "Pending positions overlap other declared positions."
  - off the insertion point: swap the `pre_site` and `pre_captures[0]` bindings and their recorded positions. Bindings, distinctness, overlap and the stray check all pass, so contiguity fails → "Pending blocks are not at their frame insertion points."
  - stray pending role: position 2 relabelled `event` → "A pending role appears outside the declared positions."
  - `committed_before` +1 on segment 1 → "Logical work of segment 1…".
  - `plan_total` +1 → "Plan total … differs from its segments."
  - each of the three labels → "Staged qualification labels differ from the profile."
  - `artifact_bytes` +2 → "Staged artifact byte count is not its envelope size." +2 can never land on a fixed point, even across a digit boundary.

Unverified assumptions, which fail loudly rather than mask: the EMPTY_CAPTURE grammar and empty capture layout; position 2 being `pre_state`; the golden Target's validity once its policy is stripped; and the Display of a serde `try_from` custom error containing `code: message`. None of these was executed.

## Check classification (Opus, static; nothing executed)
Call closure of the new path: `compile_staged` → `StagedArtifact::validate` → `authorization::decode_digest` → `schnorr::unhex` (hex), `signed_relation::validate_shape`, `check_action`/`check_pending`/`check_totals`, and `serde_json::to_vec_pretty`. All of it is pure computation. `grep` finds no `std::fs`, `env`, `process` or `net` under `stage/`. New test inputs are inline strings plus a compile-time `include_str!` of a static JSON data file. They use no env vars, network, keys, SRS or proving cache.

- **BACKEND-PROPOSED-CHECKS.json, exact 2 commands (`--test stages`, `--test stage_relation`): ordinary.** These are offline, locked, pinned-toolchain local test builds. `stages.rs` calls `compile_staged` at line 45, so its non-ignored tests now also exercise validate, which is pure. The env-gated `#[ignore]` legacy baseline test (lines 156-159) must still be reported ignored. The expected `stage_relation` result is 9 tests; the earlier run reported 4.
- **CLI-PROPOSED-CHECKS.json, exact 1 command (`mori --test core_continuations`, 7 `#[test]`): ordinary.** The `mori compile` staged branch is unchanged apart from validate running before `publish_staged_artifact`, which is the same existing filesystem publication into the test's output directory. Validate adds no effect. Expect 7 tests.
- **FRAME-PROPOSED-CHECKS.json (`--lib stage::frame::tests::`, 6 selected): may reuse its prior class.** The library recompiles with the `lower.rs` delta, but `frame.rs` never references `compile_staged` or `lower`, so the selected frame tests cannot reach the changed code. A rerun is not required on security grounds. If it is rerun, the expected result is unchanged.

## Worktree drift observed (not mine)
At handoff, `git status` also shows `M crates/mori-ledger/tests/source_closure_coverage.rs` (+127 lines) and `M docs/CORE-CONTINUATIONS.md`. Neither was present at session start. I did not write them; all of my writes went to R only. HEAD is still `e9a2b80b`, and `crates/mori-backend` is still clean, so both preimages above stay valid. Those changes are out of scope here and need their own owner and classification. I did not read them.

## Still owed
None of these tests have been executed. Integration, the actual check results, and an independent full-source plus actual-results Opus audit of this repair are still required. Two other Opus reviewers are reading the frozen source; this repair changes bytes, so their findings apply to `e9a2b80` only. Native authority stays 0.
