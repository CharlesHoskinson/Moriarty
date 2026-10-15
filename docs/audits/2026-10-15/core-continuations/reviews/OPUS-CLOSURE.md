# OPUS-CLOSURE: fresh independent full-child closure security and integrated review

- **Reviewer:** Claude Opus 5.5 (`claude-opus-5-5`), high effort, in a fresh session. I am not an author, classifier or integrator of any reviewed artifact. I used no other reviewers and dispatched nothing.
- **Candidate:** `0e42392d691d474e5a343765fba318b621f4b2bc`, tree `7cccf9c45a7a446e18bd5237cdf2aad6436176ca`. Predecessor `dd4e4298…`, base `fe01bc8b…`. The worktree was clean.
- **Startup:** I loaded AGENTS.md. Guarded `status --json` returned only the historical SP01 queue, and I dispatched nothing.
- **Method:** read-only git, source, sha256 and plain-capture inspection. No build, test, Quint, SDK or native action, and no key, PK, SRS or cache access. Authority stays 0. The only files written are this report and its result file.
- **Timing:** not instrumented. The run may have exceeded 650 s.

## Dispositions (whole local child feature: `local-reference-stage` / `none` / `untracked`)

| Axis | Disposition |
|---|---|
| SOURCE | **APPROVE** |
| ACTUAL-RESULTS | **APPROVE** (captured-local: 88 = 17 fresh + 71 retained; not a rerun of 88) |
| DOCS | **APPROVE** |
| INTEGRATED | **APPROVE**: qualified whole-child closure only |

- **F-1/H18: CLOSED.**
- **F-2/H15: CLOSED.**
- There is no remaining mandatory finding.
- This is not V1, parent, native, event-authentication or consumption approval:
  - all ten `/4` native/target gates stay `unperformed`;
  - native call `/2` stays unsupported;
  - the Fenrir tuple stays unknown;
  - every U0–U7, M1–M17 and MC/SP/G exit stays OPEN.
- The dd4 Opus report stays a historical NEEDS_CHANGES record. This report judges a new candidate and does not rewrite that report.

## Prior independent premises (byte-identical, explicit)

| Premise | Basis |
|---|---|
| **P1.** `R/OPUS-FINAL-INTEGRATION.md` (sha256 `6c8df4b8…`, identical to the dossier copy) | Covers dd4 SOURCE (production: no defect), H1–H28 mapping, all normative reads, A-C1/H19, A-C2/H16, A-D1, B-T3/H20 closures, and RESULTS/DOCS APPROVE. It relies in turn on OPUS-SUCCESSOR-A/B and the A/B completions, which cover the production modules at `7d6c3965`. |
| **P2.** `R/MUSE-FINAL-INTEGRATION.md` (`0a3cd8d4…`, identical to the dossier copy) | Nonsecurity SOURCE/RESULTS/DOCS APPROVE at dd4. |
| **Validity** | I independently ran `git diff --stat 7d6c3965 0e42392d -- crates/**/src/** Cargo.lock **/Cargo.toml **/build.rs examples/core/native-owner-intent/golden-vector.json`. It is empty, so the production bytes are identical to those reviewed in P1 and P2. |
| **Author material, not acceptance** | `OPUS-LAST-TWO-TESTS*.md`, the freeze note and the integration JSON are authorship evidence only. |

## Full dd4..candidate delta (60 files, all inspected)

**Source:**
- Two pure appends:
  - `compat_v1.rs` +21 (hunk `@@ -120,3 +120,24`);
  - `core_continuations.rs` +17 (`@@ -274,3 +274,20`).
- No line of a prior test body was removed or changed. The pre-hashes `c1f75cf8…` and `816a4320…` equal the dd4 pins.
- Two new literal fixtures.

**Docs:**
- ROADMAP, the guide, the examples README, the dossier README, REVIEW-STATUS, quint-lifecycle README/OBSERVATIONS (88 count).
- New closure scope and observations, copied dd4 reviews and receipts, the historical dd4 binding, and capture `.gz` files.
- `PRODUCTION-SOURCE-PROJECTION-CLOSURE` and `CAPTURE-PIN-COMPARISON-CLOSURE`.

**Frozen snapshots:** the hashes of `SOURCE-SCOPE{,-CURRENT,-FINAL}.json` and `OBSERVATIONS{,-CURRENT,-FINAL}.json` equal the values recorded in OBSERVATIONS-CLOSURE (`e4cd30a7`, `beafba22`, `fed94690`, `d8aff09b`, `b037a98f`, `f5f6aa17`). None of them is in the delta.

**Oracle:** `76075b63…` is unchanged at both copies (source manifest).

## F-1 / H18: CLOSED

**Test:** `crates/mori/tests/core_continuations.rs:277-293` (`h18_core_two_compile_signed_target_is_unsupported_before_output_directory`).

**What the test does:**
- **Target literal.** `SIGNED_TARGET` matches the decoded `target_canonical_utf8` at `examples/core/native-owner-intent/golden-vector.json:1863` (the golden vector's sha256 `9c035c8e…` is unchanged). I compared it field by field: anchor, revision, limits, network, profile, `signed_policy{beneficiary,owner,program_address}` and zkir_version. It is public Target data with no key material.
- **Signed precondition.** The test asserts `Target::from_json(...).signed_policy.is_some()`, so it cannot pass vacuously on an unsigned literal.
- **Inputs.** It runs public `mori compile` on `delivery.mori` (`language "moriarty/2"`) with `--target` and an absent `--out`. The only temp inputs are `source.mori` and `target.json`.
- **Assertions:**
  - exit `Some(1)`;
  - empty stdout;
  - exact diagnostic `mori::command: mori::backend::unsupported: Staged moriarty/2 relations are unsigned; a signed Target is refused.`;
  - `!out.exists()`;
  - exactly 2 directory entries.

**Production path, unchanged since `7d6c3965`:**
- `crates/mori/src/commands/compile.rs:20-23`: `/2` reads the Target, then calls `compile_staged`.
- `crates/mori-backend/src/stage/lower.rs:23-28`: the signed refusal is the first statement, before `stage::load` (`:29`) and `target.validate()` (`:30`).
- `create_dir(out)` happens only at `compile.rs:25`, after compile and serialization succeed.

**Why the test is meaningful:**
- If `create_dir` were moved before compile, `!out.exists()` and the entry count would fail.
- If the guard were removed, the valid delivery program would proceed and the diagnostic, exit and no-output assertions would fail.

This meets DESIGN-OPUS H18 ("unsupported, no out directory") at the public surface that the prior F-1 identified.

**Actual result:** `cli-normal-05` records exit 0, 12 passed, and a line `test h18_core_two_compile_signed_target_is_unsupported_before_output_directory ... ok`. The other 11 names equal the dd4 set.

## F-2 / H15: CLOSED

**Test:** `crates/mori-core/tests/compat_v1.rs:124-143` (`h15_await_and_when_identifiers_keep_program_and_execution_bytes`).

**Source:** the `/1` program uses `let await = 1; let when = await; next count = when;`.

**What the test asserts:**
- `format!("{}\n", json::export(&program))` equals `program.json` exactly: 881 bytes, `27f77197…`.
- The simulated Execution export plus newline equals `execution.json` exactly: 2092 bytes, `88315015…`.
- Strict decode of each fixture equals the parsed or simulated value. This is an additional check.

**Fixture inspection:** I read both fixtures in full.

**Independent base check:** `git grep` at `fe01bc8b` `crates/mori-core/src/ir.rs` gives the following field orders, and both fixtures follow them:
- `Program`: language, semantics, name, states, assets, expressions, actions (`:172-179`);
- `Execution`: semantics, action, pre, post, steps, effects (`:256-262`);
- `Step`: index, statement, value, snapshot, effects (`:247-252`);
- `Value` is tagged `type`/`value`, and `Expr`/`Statement` are tagged `kind`.
- The expression arena (Literal 1, Name await, Name when), the Let/Next statement indices, the pre 0 / post 1 values and the single `State` effect agree with simulate semantics.
- `git diff fe01bc8b..candidate -- crates/mori-core/src/json.rs` touches only `parse`/`parse_limited`, so `export` is unchanged since base.

**Freeze order:**
- Payload fixture mtimes are 00:42:56 and 00:43:01 local. The test sources are 00:43:46 and 00:43:51, and the captures ran at 06:49Z.
- The payload fixture hashes equal the committed ones.
- The freeze note states that nothing was parsed, simulated or exported to produce them.
- Nothing in the source derives expectations from candidate output.

**Actual result:** `core-compat-02` records exit 0, 5 passed, and `h15_… ok`. The other 4 names equal the dd4 set. This satisfies H15/C6 "unchanged parse and Execution bytes".

## ACTUAL-RESULTS

| Capture | Exit | Passed | stdout / stderr sha256, recomputed | Pins |
|---|---|---|---|---|
| `cli-normal-05` | 0 | 12 | `d65fa202…` / `1c5cf9a6…`, equal to result and OBSERVATIONS-CLOSURE | Equal to current hashes for both test files, both fixtures, `compile.rs` `8ec1daaa`, Cargo.lock `dfffb853`, golden vector `9c035c8e` |
| `core-compat-02` | 0 | 5 | `2cca9062…` / `8097a524…`, equal | Same as above. Its argv is `cargo +1.99.0 --offline --locked test -p mori-core --test compat_v1` |

Both runs record `native_actions_requested: 0` and `public_fixture_leftovers: []`.

**Counts:**
- 88 = 5+5+13+22+5+10+12+6+3+6+1.
- 17 = 12+5.
- 71 = 40 Core + 15 backend + 9 ledger + 6 frame + 1 legacy.
- No rerun is summed. The failed captures stay uncounted.
- Quint 2+9=11 stays separate, with no Rust correspondence.

**Pin comparison:** I verified 1086 = 49+142+49+49+138+138+142+58+139+101+81. I assessed every disclosed delta by call path:
- **New in this round:** `compat_v1.rs` (separate test binary; no `include_str!` of it anywhere in `crates`), `core_continuations.rs` (CLI test binary only) and the examples README (never `include_str!`'d). No retained selection reaches these.
- **Carried from earlier rounds:** `lower.rs`, `stage_relation.rs` and `stage_hostile.rs`. They are unchanged since dd4 and covered in P1.
- **Fresh captures:** their only delta is the README.

**Manifests:**
- `sha256sum -c` passes for `CLOSURE-SOURCE-ABSOLUTE-SHA256SUMS` (87 entries). These are direct WT paths, so no projection premise is needed.
- `sha256sum -c` also passes for `CLOSURE-DOSSIER-ABSOLUTE-SHA256SUMS` (561 entries), which has the same line count as `D/SHA256SUMS`. The harness refused a line-by-line projection diff, so dossier projection equality is not independently diffed.

## DOCS

- The guide, ROADMAP, examples README and dossier README consistently state:
  - 88 = 17 fresh + 71 retained, not a rerun;
  - Quint 11 separate;
  - the 58/78/86 snapshots are historical;
  - closure reviews pending at the candidate;
  - local, unmerged, all exits open.
- The 71 breakdown in the guide is arithmetically correct.
- REVIEW-STATUS preserves the dd4 Opus NEEDS_CHANGES and labels the last-two author as "not independent approval".
- The advisory/unauthenticated stored-cap disclosure (A-D1) and the labels are outside the delta and unchanged.

## H1–H28 carry-forward

Coverage is exactly as in P1's table, with two rows updated:
- **H15:** now covered by `compat_v1.rs:124-143` plus `core-compat-02`.
- **H18:** now covered by the backend identity test plus `core_continuations.rs:277-293` and `cli-normal-05`.

O-1 and O-2 and all earlier optional notes stay optional and are not promoted.

## Findings

No required finding. One optional style nit: `core_continuations.rs:276` has no blank line before the appended comment.

## Limitations

- No execution was performed. Results are as captured.
- Equality of the `.gz` files with the plain captures is a premise; `zcat` was refused.
- The base byte behaviour of the H15 fixtures is supported by derivation and structure, not by a base-built run.
- The 71 retained functions are not rerun.
- The production semantics are carried by P1/P2, which rest on byte identity.

## Publication

I **allow** the separate, mechanical, metadata-only publication commit:
- copy this report and filtered receipts verbatim;
- bind `0e42392d…`/`7cccf9c4…` and the source hashes in an external FINAL-REVIEW-BINDING;
- update the maintained dossier status and checksums.

Frozen scopes and observations stay untouched. The commit permits no source change and no qualification upgrade.
