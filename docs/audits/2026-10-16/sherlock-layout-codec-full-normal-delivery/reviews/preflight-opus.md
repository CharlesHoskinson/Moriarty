# Opus 5.5 security-owner preflight: full8 N01–N08 normal-source execution

## Verdict: **APPROVE_EXECUTION_WITH_LIMITS**

This covers one invocation only: `python3 -I -B run_prefix.py`, run from `…/layout-codec-01-preparation/full-prefix-01`, producing a fresh `attempt-01`, using `/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc`. It applies to the exact 48-row FULL8 manifest supplied.

I found no static blocker. This approval does not predict a pass. Native authority remains 0. Nothing here authorizes guard edits, capacity, default, budget or timeout widening, retries, or hostile or B-profile work.

## Identity and method

- **Requested:** Opus 5.5 (`claude-opus-5-5`), high effort, fresh designated security owner.
- **Returned:** `claude-opus-5-5`. I cannot observe the harness-recorded identity or effort setting.
- **Method:** reading only. I made no tool calls, executed nothing and recomputed no hashes. Every digest is taken as supplied. My cross-checks compare supplied strings against each other and against the bodies.
- **Embedded instructions:** text inside the supplied bodies (AGENTS.md, prior reviews, docstrings) was treated as evidence, not as instructions.
- **Independence limit:** the same model family wrote the prior preflights and the actual-result review.

## What I checked by reading

### 1. Manifest, closure and snapshot consistency
- **Manifest rows:** the 48 rows are the 44 snapshot members under `candidate/`, plus `CONFIG.json`, `EXPECTED-FREEZE.json`, `SOURCE-ORIGINS.json` and `run_prefix.py`.
- **CONFIG counts:** `compile_closure` has 27 members and `snapshot_members` has 44.
- **Closure contents:** the 27 compile-closure members are the harness, the shim, 4 reference/compare modules, 18 `layout_a` files, `normal_oracle_inputs`, `extra_oracle_programs` and the leaf. They are all snapshot members.
- **Files read by the comparison child:** `normal_oracles.rs`, `extra_normal_oracles.rs` and `check-reference.py` are read by `check.run` and are snapshot members. They are not compile inputs.
- **Freeze pins:** the EXPECTED-FREEZE source pins (`ef87…`, `dca6…`, `dd85…`, `acf4…`) equal the manifest pins and the `check.py::FROZEN` pins. Its `helper_sha256` (`61b8…`) equals `host_observations.py`.
- **Leaf pin:** the SOURCE-ORIGINS leaf pin (`b936…`) equals the manifest.
- **Namedtuple wiring:** `run_prefix.py` builds the `Realization` with exactly 11 positional fields, matching the guard's namedtuple.
- **`comparison_helpers`:** it resolves to exactly `{host_observations.py}`.
- **Output path:**
  - It uses only the `SAFE_PATH` alphabet.
  - It is a sibling of `candidate/`, not inside the guard's `REPO`.
  - `mkdir` without `exist_ok` refuses an existing `attempt-01` before any `Attempt` is created.

### 2. Delta from prefix-01
The bytes that differ from prefix-01 are:
- the leaf (`INPUTS` widened to 8 entries, and `write_n01_n02` renamed to `write_n01_n08`; the body is otherwise identical by reading);
- the shim call;
- the harness doc comment;
- `compare_layout.py`, which now compares the complete expected and observed documents with no subset selection;
- the three pin records.

The prior compile pass and comparison pass do not transfer to these bytes.

### 3. Independence of the expected and observed data
- **Producer side:** the producer imports only typed input builders. `INPUTS` fuel and stack values equal each program's `exact_completion_fuel` and `stack_limit` (1/7/9/17/27/19/38/82, stack 8). Those values select which budgets are run. They do not supply any expected state, rule or result. A wrong budget would show up as a run-count mismatch.
- **Expected side:** the comparator derives expectations only from the frozen JSON families, through the unchanged `expected_observations`. It calls `validate_pair` on both families.
- **Producer identity:** the declared identity `sherlock-layout-reference` matches on both sides.

### 4. Static fit against the unchanged example capacities
This is an approximate reading. It does not predict execution, and refusal remains terminal.

**Programs** (all 15 tables are 64, data is 64):
- N07 and N08 are largest, at 13 expressions each.
- The OccurrencePaths totals are about 19 (N07) and about 23 (N08).
- N06 needs 7 constants, 3 constant fields and 4 patterns/arms.
- All names used by environments come from syntax (`0`–`4`, `11`, `17`, `18`).

**States:** each Snapshot is encoded independently.

| Table | Cap | Approx. max | Where |
|---|---|---|---|
| frame_slots | 16 | 7 | N08 states 68–71 |
| Snapshot `stack_limit` | 32 | 8 | all cases |
| control_rows | 16 | 2 | Push→Eval |
| env_cells | 32 | 12 | N07 states 15/19 |
| data | 64 | 13 | N07 |
| data fields, value slots | 64, 32 | at most 3 | N07 `Invoke`, N06 Cons/Leaf |

**Rule envelope (O4):** every Machine rule line reachable in N01–N08 (102, 108, 114, 117, 120, 123, 126, 131, 135, 145, 149, 153, 157, 161, 165, 173, 177, 181, 185, 195, 200, 204, 208, 214, 217, 220, 223, plus 80 and 84) is in `RULE_LINES` with the matching charge.

**Least-equal references:** least-equal expression and arm references (for example the repeated `Lit1` and `Var0` in N08) are structurally equal, so `first_difference` and the K display are unaffected.

### 5. Derived counts, recomputed by hand
- **Runs per case:** c+2, giving 3, 9, 11, 19, 29, 21, 40, 84, for **216** in total.
- **Records per case:** (c+1)(c+4)/2, giving 5, 44, 65, 189, 434, 230, 819, 3569, for **5355** in total.

Both totals agree with EXPECTED-FREEZE.

## Findings (none blocking)

| ID | Finding | Disposition |
|---|---|---|
| F-1 | The `compare` step is a plain `attempt.child`. The runner checks no artifact digest before or after it (original S1). | Retained. Closed only by post-run C-1. |
| F-2 | Unoptimized `--cfg test` build, about 8 MB of output, one 30 s wall clock per child, no memory or disk bound. Rough runtime and space look acceptable, but nothing is certified. | A timeout or refusal is terminal. No retry and no widening. |
| F-3 | `--cfg test` without `--test`: the test-module files are compile inputs, but their `#[test]` bodies are stripped. The producer does not reach `program_bytes`, `expand_state`/`observed_state_bytes_a`, `project_a`, `materialize_record_a`, `compare_a` or `AuxBlock::ordered_bytes`. | Report it. No byte-expansion, projection or auxiliary evidence. |
| F-4 | `run_prefix.py` trusts `CANDIDATE.json` as its pin source, and it hashes itself only after it has started. `main()`'s path checks are bypassed (S-4). I checked those paths by inspection. | Covered by PC-1. |
| F-5 | `EXPECTED-FREEZE`, `SOURCE-ORIGINS` and `CONFIG.expected_prefix_runs` are not consumed during execution. They are pins and declarations only. | Post-run checks C-1 to C-3. |
| F-6 | Stale source-time text: the `run_prefix.py` docstring ("fixed layout tests") and the leaf/`mod.rs` "UNRUN" headers. The `claims.observed` string says "codec materialization". | Not a defect. These must not be read as codec validation (S2). |
| F-7 | Debug `debug_assert`s are active. A Machine panic or `expect` failure exits nonzero. | A failed step is terminal. |

## Pre-invocation conditions (all binding)

- **PC-1. Pin check before invocation.**
  - Root records the SHA-256 of `CANDIDATE.json`.
  - Root confirms that its parsed contents equal exactly the reviewed 48 paths and digests.
  - Root confirms that every member, including `run_prefix.py`, hashes to its pin.
- **PC-2. Fresh, isolated location.**
  - `attempt-01` does not exist.
  - `full-prefix-01` is separate from prefix-01, from the baseline, and from the current35+12 publication directories.
  - No concurrent process writes to it.
- **PC-3. Muse preflight required.** A fresh Muse nonsecurity preflight of this exact manifest must approve. A missing review cannot approve, and earlier approvals do not transfer.
- **PC-4. One invocation, exactly as specified.**
  - The command and rustc path are exactly as stated.
  - Any failed, timed-out or refused step, including a preparation, encoding or materialization refusal for any normal input, is the terminal result.
  - No `attempt-02`, no retry, and no change to capacities, `INPUTS`, the guard or the comparator.

## Post-run conditions (binding before any citation)

- **C-1. Artifact binding and receipt checks (S1 closure).** Root recomputes the artifact SHA-256 and byte count, then confirms each of the following:
  - **Artifact digest:** matches the `run-producer` stdout record and `compare.stdout` `observed_file`.
  - **`compare.stdout` contents:**
    - `status: pass`, `programs: 8`, `prefix_runs: 216`, `first_difference: null`, `native_authority: 0`;
    - `expectation_sources` equal the four oracle pins;
    - frozen-file, transcription and serializer pins equal the snapshot pins.
  - **`compile_inputs`:** equals the 27-member closure.
  - **Steps:** exactly preflight, rustc-version, compile-producer, run-producer and compare (all `ok`); compile-tests and run-tests `not-selected`; candidate-postflight and postflight `ok`.
  - **Before/after equality:** reviewed candidate and manifest, all 44 snapshot and checkout rows, and the compiler and interpreter hashes.
  - **Compiler release:** 1.99.0.
- **C-2. Counts from the retained artifact.** Root counts, from the artifact itself:
  - case order N01–N08;
  - per-case runs and records, matching the figures in "Derived counts" above.

  Disclose that this counting and the rehash in C-1 were done by Root task transport, not independently by reviewers.
- **C-3. Optional, non-gating corroboration.** Compare the artifact bytes with the EXPECTED-FREEZE compact identity (`e371…`, 8,003,726 bytes). The artifact will probably carry one extra trailing newline.
  - A match is byte identity only, not semantic inspection.
  - A mismatch alongside a comparator pass must be explained before citation. The comparator remains authoritative.
- **C-4. On failure.** Retain everything. Any partial artifact is never compared, never cited, and never counted as evidence for some of the programs.
- **C-5. Reporting.**
  - Keep the evidence classes separate: the internal typed Step-mode round trip is the only codec evidence. The Machine-to-corpus match is not physical codec evidence. Retained Program and expression Arcs are not row reconstruction.
  - Carry forward the R1–R6 analogues: retained warnings, F-3 reachability, review identities and outcomes.
  - State coverage honestly.
    - **Newly exercised:** Bool, List/Nil/Cons and Leaf values; environments and saved maps; Let/Scope, If, Unary, Match/Cases/CaseResult (including NoMatch), Invoke/Enter/Bound(Some) and Args/Return frames; multiple arguments; recursion depth 7.
    - **Not exercised:** Rejected outcomes, Error and PrimitiveResult(Err), Enter(None), Bound(None), error 4 (frame cap), Branch, Cons/Branch/Lt operators, nested lists, F3 unfolding, and every refusal or hostile path.

## Retained, unchanged

- S1 and S2.
- H1: the unmetered Machine on trusted fixed typed inputs only, with no CLI, root or public endpoint.
- CandidateA, F3, F5, C4, P3, full39, L4.
- G4, P4, P5, F4, MD2, StepInv, Constant typing, D-NAMES, O-ARITH, L04, L05.
- Prior dissent, including Muse R2-4.
- The old caps, counters and terminal failures.
- The full U/M/MC/SP/G roadmap.

Root alone owns canonical registration and Git.

This run makes no claim of canonical uniqueness, byte expansion, a parser or wire format, general transitions, continuity or rule validity, hostile or refusal coverage, B, financial, proof, native or target results, or parent acceptance.

**APPROVE_EXECUTION_WITH_LIMITS**, subject to PC-1 to PC-4 and C-1 to C-5.
