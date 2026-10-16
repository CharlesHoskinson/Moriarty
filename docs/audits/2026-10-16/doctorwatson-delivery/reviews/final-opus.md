# Moriarty DoctorWatson: final candidate and actual-result security audit

**Verdict: APPROVE_WITH_SCOPE**

**Merge blockers: none.**

**Reviewer:** Opus 5.5, high effort, security owner. This was a fresh context with no authoring history. Opus authored the wrapper and its guard tests in an earlier context, so a shared model-family blind spot is possible. The independent Muse nonsecurity review is still needed for that reason.

**Candidate:** base `4172d561…`, staged tree `f9104554…`, 184-file manifest.

**Approved scope:**
- Merge of the stdlib-only `--host` lane, `Machine::run_steps`, the seven bounded tests, the fixed-input producer and the documentation/evidence set.
- The approval covers only the post-constructor, normal, fixed and trusted 8-program / 216-prefix host domain.

---

## 1. What this review did and did not establish

**Done**
- Read every supplied source and evidence file as untrusted data.
- Checked each recorded hash binding against the candidate manifest by text comparison.
- Re-derived the coverage arithmetic.
- Traced `run_steps`, the producer, the comparator and the wrapper against the frozen corpus contract.
- Re-checked preflight items M1 and M2 by inspection.

**Not done**
- No tools, commands, hashing, decompression or execution.
- I did not recompute any SHA-256.
- I did not decompress the 8 MB trace or the `source/*.gz` snapshots. For those I rely on the integrator's statement that it verified the decompressed SHA (`cfdc36e0…`) and every consumed snapshot hash.
- Manifest files whose contents were not supplied, and which I therefore did not review byte-for-byte:
  - `design-01/*`, `design-02/*`
  - `ordinary-01/*`, `ordinary-02/*`
  - `entrypoint-host-tests/*`, `entrypoint-tests/*`
  - `stop-counters/*`, `performance-02/REPORT.md`
  - `host-GREEN/*`, `host-GREEN-outer/*`
  - every `*.gz`

---

## 2. Verified findings

### 2.1 `Machine::run_steps` (`crates/mori-core/src/sherlock/reference.rs` ~76–98)
- It delegates every event to the existing `step()`.
- Free terminal publication counts as one event, and collection stops right after the first non-`Pending` outcome.
- A budget of `max_events = 0` is a pure no-op.
- An already-terminal machine returns `[]` and does not change.
- Later calls resume the same machine.
- It adds no fuel or resource policy and changes no existing rule, charge or line.
- The predecessor RED compile error (E0599 at `reference.rs:9`, call site `normal_observations.rs:141`) shows that the method was missing before this candidate and that the producer uses it.

### 2.2 Producer (`models/sherlock/reference/normal_observations.rs`)
- It builds only fixed typed ASTs from `normal_program`, runs `Machine::new(program, fuel, 8)` for every fuel value in `0..=E+1`, and requests `fuel + 1` events.
- `fuel + 1` is always at least the number of charged steps plus one publication, so the event budget never truncates a run:
  - When fuel ≤ E, the run produces fuel+1 records.
  - When fuel = E+1, it produces E+1 records.
- It records `terminal_has_next` with a real extra `step()` call after the snapshot. Truncation or a missing publication would therefore show up as a mismatch.
- `charged_steps: null` for a counter that cannot be valid becomes a type mismatch, not a silent clamp.
- JSON escaping is correct for the ASCII K domain.

### 2.3 Coverage arithmetic (independently re-derived)
- **Runs:** Σ(E+2) = 3+9+11+19+29+21+40+84 = **216**.
- **Records:** Σ[(E+1)(E+2)/2 + (E+1)] = 5+44+65+189+434+230+819+3569 = **5,355**.
- **Budget sum:** the `INPUTS` budgets add up to 200, which equals the 200 charged edges.
- The 5,355 figure is the size of the expected projection. It is not printed by the comparator. Because `first_difference` compares arrays at full length, `match` implies all 5,355 records are equal.

### 2.4 Comparator (`check.py --host-observations`, `host_observations.py`)
- Expected values are rebuilt only from the frozen corpus overlay. No rule is evaluated.
- The comparison is strict:
  - JSON parsing rejects duplicate members and non-finite numbers.
  - Types and keys must match exactly, including extra members.
  - Array order and length are checked.
- All 8 program identities are compared before any run.
- The supplied artifact is never presented as authenticated execution.

### 2.5 Wrapper (`models/sherlock/qa/run_host.py`)

**Attempt directory and source snapshot**
- Fresh exclusive attempt directory outside the checkout, restricted to the `SAFE_PATH` character set.
- Source members must be regular files reached without symlinks. Each is read once into a snapshot, and every child reads the snapshot, not the live checkout.

**Compiler acceptance**
- The compiler must be the real binary in a sysroot layout; a rustup proxy is refused.
- `rustc -vV` must report a release equal to the pinned `1.99.0`.

**Child process boundary**
- Children run with no shell and a three-variable environment.
- stdin is `/dev/null`, each child gets a new session, and a 30 s wall clock ends in a process-group SIGKILL.
- There is no retry.

**Closure and result checks**
- rustc dep-info must equal the declared closure exactly: 7 producer files and 8 test files.
- libtest must report exactly 7 named tests, all `ok`, with 0 filtered.
- Each artifact's SHA is re-checked before it runs.
- The comparison is bound to the produced artifact's SHA and to the snapshot hashes of the 7 files it read.

**Postflight and fail-stop**
- Postflight re-hashes the checkout, the snapshot, the interpreter and the compiler.
- The first failure marks every dependent step `not-run`.
- Children never receive caller secrets. The only environment values recorded are `PATH`, `LANG`, `TMPDIR` and the two `DOCTORWATSON_*` paths.

### 2.6 Actual results against the preflight predicates

| Predicate (preflight-opus) | Observed | Status |
|---|---|---|
| (1) RED copy included ordinary tests | 53 tests ran in `host-RED-outer/stderr` | ✓ |
| (2) RED: version ok, exactly one E0599 `run_steps`, dependents `not-run`, postflight ok, `failed:compile-producer` | `host-RED/receipt.json`, `logs/compile-producer.stderr` | ✓ |
| (3) GREEN: all 8 steps ok, `success`, pass/match, 8/216, SHAs recorded | `host-FINAL/receipt.json`, `comparison.json` | ✓ |
| (4) Outer receipt binds ordinary QA | `host-FINAL-outer/result.json` before/after hashes for all `test_*.py`; 79 tests in stderr | ✓ |

### 2.7 Hash binding to the exact candidate
- Every executable or consumed input hash in `host-FINAL/receipt.json` (24 members) and `design-FINAL`/`host-FINAL-outer` (13 members) matches the candidate manifest or the supplied unchanged context files.
- This includes `reference.rs cc6887…`, `normal_oracle_inputs.rs d02cf4…`, `reference_bounded_tests.rs 5e4c41…`, `run_host.py afa530…`, `check.py 8793b5…`, `normal_observations.rs 4cf4a6…`, `doctorwatson.sh 769baa…`, `test_run_host.py d0dfdf…`, `test_doctorwatson_entrypoint.py 924b76…` and all `.qnt` files.
- The internal chain is consistent: receipt `2f95f5…` ↔ `result.json` ↔ outer stdout ↔ manifest, and comparison `e53652…` ↔ receipt ↔ manifest.
- **The actual results bind to the candidate bytes for every consumed file.**

### 2.8 Preflight M1: uncompiled umbrella edits (agree with the disposition)
I checked the module wiring by inspection only.
- `checkpoint_normal_tests.rs` uses `no_functions`, `expr` and `uint`, and `helper_call_adapter_tests.rs` uses `normal_program`. All four reach both files through `reference_oracles_tests`' explicit private import via `use super::*`.
- Neither file uses `boolean`, `var`, `add` or `call`.
- `#[path = "extra_oracle_programs.rs"]` resolves relative to `sherlock/` in both build contexts.
- `super::super::{…}` resolves to `sherlock`, which re-exports `reference_types::*`.
- I found no glob-name collisions.

**Residual risk:** the `mod.rs` umbrella (`cargo test -p mori-core`, and the pinned `check-reference.py --rustc` path) was not compiled. Sibling modules outside the two supplied children were not inspected. This is accurately disclosed in the delivery README.

### 2.9 Preflight M2: wrapper/`--host` tests (resolved)
- 10 entrypoint tests (6 existing + 4 `--host`) and 22 wrapper guard tests (3+3+2+3+3+3+5) are present and passed in the final run of 79.
- 79 = 33 + 10 + 14 + 22.
- None of these launches rustc. The CLI cases stop before preflight.

### 2.10 Claims and limits
I checked README.md, ROADMAP.md, `docs/DOCTORWATSON*.md`, the OpenSpec proposal/requirements, `models/sherlock/qa/README.md`, `RESULT.json` and the delivery README.
- The counts match: 79 / 7 / 8 / 216 / 5,355 / 34+1 / 10k / 60 / seed 1 / 8 zero witnesses / 3.611 s / 31.733 s.
- The following are stated as unrun: K, Sherlock interpreter Quint, Lean/Clean, circuit, native and stagenet.
- No document closes a parent exit or changes native0, C4 or the counters.
- No automatic hosted CI and no orchestration infrastructure are added.

---

## 3. Nonblocking findings
Fix these in a follow-up or note them in the PR. None of them invalidates the observed results.

**S-1. `PREDECESSOR-SOURCE.json` is time-scoped and stale against the final tree.**
- Its `scope` says "NOT EXECUTED", yet it was executed as `host-RED`.
- It marks `test_doctorwatson_entrypoint.py` (`a07dc2…`) `differs_from_current: false`, but the candidate has `924b76…`.
- It omits `test_run_host.py`, which exists in the final tree.
- "Current" here means the preflight candidate (`preflight-CANDIDATE.json`).
- The RED conclusion still holds: the wrapper `afa530…` is byte-identical, and the differences affect only the pre-wrapper QA test set.
- Leave the file unchanged as an immutable record. Add one line to the delivery README saying that "current" in that file means the preflight candidate.

**S-2. `run_steps` retains memory and its own doc has no explicit H1 line** (`reference.rs` ~76–84).
- It keeps up to `max_events` records, each with two full snapshot clones. That grows memory linearly, whereas `run()` discards its records.
- H1 is stated at the type level (`reference.rs` 4–8) and in the docs for `step` and `run`, and nothing here wires `run_steps` to untrusted input.
- Add "not safe for untrusted input (H1); retained records grow linearly with `max_events`" to its doc the next time this file changes.

**S-3. The HOST-OBSERVATIONS.md wording is inaccurate (preflight N2).**
- `models/sherlock/qa/HOST-OBSERVATIONS.md` ~line 11 says "never expected trace/output data".
- But `INPUTS` (`normal_observations.rs` ~29–32) hard-codes `exact_completion_fuel`, which is corpus metadata equal to the expected work.
- This is not circular: a wrong value changes the run membership and fails the comparison.
- The file is in the hashed host closure, so rewording it requires a fresh host rerun. Defer that to the next host cycle.

**S-4. The comparison artifact labels Rust-host as not run** (`check.py` `run()` `execution_layers`).
- `host-FINAL/comparison.json` reports `Rust-host: not-run` with `execution_receipt: not-supplied`.
- That is accurate for the SDK-free checker by itself and is documented. The wrapper receipt is the execution authority.
- A reader of `comparison.json` alone could be misled. A future host-mode note in the report would help.

**S-5. Retained preflight limits.**
- *N1:* the wrapper does not parse `first_difference` from `comparison.json` when the comparison exits nonzero. The raw file is kept and SF-03 is still met through it.
- *N3 (stated limits, not defects):*
  - Snapshot files are 0444, but their directories stay writable. The subdirectories use the default mode from `preflight` `mkdir(parents=True)`, so the docstring's "read-only snapshot" overstates this.
  - `run_host.py` is hashed after it has already been loaded.
  - Descendants of a child that exits normally are not swept.
  - There are no memory or disk caps.
  - The sysroot, linker, libc, stdlib and kernel are trusted and unhashed.
  - Authenticity is limited to a trusted checkout.

**S-6. Some negative paths are unobserved end to end.**
- No wrapper-level run has been observed for a comparison mismatch (`failed:compare`), a timeout kill, or an artifact changing before execution.
- The guards for these exist and are reviewed. Only the RED compile path is observed.
- The RED attempt never compiled the seven tests, so they were not observed failing before the implementation existed. They are byte-identical in the predecessor, which supports the claim that the tests came first.

**S-7. `run_steps` cut/resume is tested only on N01/N02.**
- The width 1/2/4/8, zero-event and continuation tests (`reference_bounded_tests.rs`) cover N01 and N02.
- For N03–N08 the producer uses a single grouped call per run.
- Risk is low because `run_steps` is a thin loop over `step()`, which all 216 prefixes validate.
- `docs/DOCTORWATSON-CI.md` should say "on N01/N02".

**S-8. Workflow-model coverage.**
- `testPassed` is now a zero-hit random witness.
- So the random sampler never reaches review, merge or completion, and `exactReviews`, `exactMerges` and `observedCompletion` are exercised only by the 34 deterministic scenarios and the directed walk.
- This is reported honestly and is design evidence only.
- I checked that `canFinishCurrent` is equivalent to `canFinish(jobToken(…))`, and that the four SF-14 counter scenarios match `applyEdit`/`applySharedInput`.

**S-9. Editorial and provenance items.**
- In `docs/DOCTORWATSON-CI.md`, the "Workflow design commands" section ends with a paragraph about "the two commands above". That paragraph describes the default lane, so it is misplaced.
- In `normal_oracle_inputs.rs`, the extra trailing blank line is kept deliberately so the bytes match the execution snapshot. `git diff --check` reports only that line.
- Receipts publish local absolute paths (`/home/charl/…`) and the toolchain layout. This follows existing repository practice; no secret was found in the supplied captures.
- The `host-RED/build/*.d` and `host-FINAL/build/*.d` files were supplied but are not in the 184-file manifest. I treated them as context. The FINAL `.d` hashes match the receipt; the RED `.d` has no receipt binding.

---

## 4. Conditions for merge
These are process conditions, not defects in the candidate.

1. A fresh Muse final nonsecurity review of the same tree `f9104554…`, as required by SF-09.
2. The PR must record the base `4172d561…`, the staged tree `f9104554…`, and the receipt and output digests. The receipts bind file bytes but not the Git tree.
3. Before pushing to GitHub, run a quick credential/secret scan on the unsupplied captured logs listed in §1. Every supplied sibling capture showed only the minimal child environment, but publishing cannot be undone.
4. Any change to a consumed input refreshes the host and design results and requires a new review.

---

## 5. Unchanged scope
- Constructor startup (`uiRun`/rule 74) is not executed.
- K diagnostic text is a projection, not a canonical codec or a typed equivalence proof.
- K, Sherlock interpreter Quint, Lean, Clean, handwritten circuits, the native ledger and Midnight stagenet are all unrun.
- The mori-core umbrella test build is unrun.
- Native authority stays 0. C4, the counters, the frozen terminal attempts and every U0–U7, M1–M17 and MC/SP/G exit are unchanged.
- This approval establishes the bounded local DoctorWatson host lane and its first real Sherlock delivery cycle, and nothing more.
