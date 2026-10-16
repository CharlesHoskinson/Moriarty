# Verdict: **APPROVE_WITH_SCOPE**

This approves the bounded PUREHOST sequence (A) predecessor RED and then (B) current-candidate run. It applies only under the preconditions below. It is not a merge approval. The result audit and final source audit stay fresh and separate.

## Basis and limits of this review

- I reviewed the pasted text only. I made no tool calls and did not hash anything. I cannot confirm that the pasted bytes match the MANIFEST SHA-256s, so root must bind them.
- I saw the full producer and test compile closure: `normal_observations.rs`, `reference_types.rs`, `reference.rs`, `reference_display.rs`, `compare.rs`, `normal_oracle_inputs.rs`, `extra_oracle_programs.rs` and `reference_bounded_tests.rs`.
- I did not see `normal_oracles.rs`, `extra_normal_oracles.rs`, the oracle JSONs, `main.k`, `helper_call_adapter_tests.rs` or `checkpoint_normal_tests.rs`. The frozen files are pinned by `check.py`.
- My compile and pass predictions below come from reading the code. They are not observations. Where I am unsure (dep-info contents, libtest line format), the wrapper fails closed as a step failure, not a false pass.

## What I checked and found sound

**`Machine::run_steps`**
- It stops after `max_events`, on `None`, or right after the first non-Pending publication.
- A zero budget is a no-op, and a later call resumes the same machine. This matches the doc comment and adds no new fuel or resource policy.

**Bounded tests (7, reference_bounded_tests.rs)**
- I traced N01 at fuel 0, 1 and 2, and N02 at fuel 0–8, against the machine rules.
  - N02 rules in order: 117, 135, 102, 149, 102, 153, 223, then 80 or 84.
  - The literal boundaries and the overlay (`remaining = F − index`) match those traces.
- Expected records are never computed by the Machine. The mixed-call test labels itself as internal agreement only.
- Cut/resume, zero-event and post-terminal cases are covered.
- The names match `EXPECTED_TESTS`, with the crate-root prefix `reference_bounded_tests::`.

**Producer module wiring**
- Nested `#[path]` resolves relative to the real file's directory, so `extra_oracle_programs.rs` is found.
- Each `super::`/glob chain resolves from the producer's crate root, including `super::super::{BinaryOp,…}` through the root's private glob.
- Edition 2024 matches the workspace.
- I found no glob name collisions.
- `cfg(test)` strips `reference_bounded_tests.rs` from the producer's dep-info, so `PRODUCER_CLOSURE` has 7 files and `TEST_CLOSURE` has 8.

**Coverage arithmetic**
- Σ(F+2) = 216 runs.
- Σ[(F+1)(F+2)/2 + (F+1)] = 5 + 44 + 65 + 189 + 434 + 230 + 819 + 3569 = **5355** records.
- This matches the stated expectation. `INPUTS` budgets sum to 200, which matches the charged edges.

**Producer JSON**
- Its member set matches `expected_observations`, and comparison order is identity → programs → runs.
- K text is ASCII, so the escaping is sufficient.

**`run_host.py` security and fail-stop**
- Fresh exclusive attempt directory, outside the checkout, with a path alphabet guard.
- Every member is a regular, non-symlink file, read once into a 0o444 snapshot.
- The compiler is the real toolchain binary (rustup proxy rejected). The release equals the pinned channel, and `rustc -vV` is checked.
- No shell. Minimal environment, stdin from /dev/null, its own session, 30 s wall clock, process-group SIGKILL, no retry.
- Dep-info must be exactly equal to the declared closure.
- libtest must report exactly 7 named tests, all ok, with 0 filtered.
- Each artifact SHA is rechecked before it executes. The comparison is bound to the produced artifact's SHA and to the snapshot hashes of all 7 files it read.
- The postflight compares checkout, snapshot, interpreter and compiler hashes before and after.
- The first failure marks every later step `not-run`. An interrupt is still finalized as `interrupted:<step>`.

**Authority**
- No Cargo, SDK, network, install, K/Quint/Lean/Clean, circuit, prover, ledger or key action.
- Only fixed trusted ASTs are run. The H1 exposure is limited to 8 inputs and 216 budgets.
- Memory and disk are explicitly not bounded.

## Preconditions for this run

1. **(A) The copy must include the ordinary QA tests.** Run_host's `SOURCE_PATHS` does not contain `test_check.py`, `test_host_observations.py` or `test_doctorwatson_entrypoint.py`.
   - If the minimal copy contains only `SOURCE_PATHS`, `unittest discover` runs 0 tests. Python ≥3.12 (3.14.4 here) then exits 5, and `set -e` stops before `run_host.py`.
   - That would be a red result for the wrong reason, with no compile attempted.
   - Copy those three files as regular files. Do not use symlinks, because `read_sources` rejects them.
2. **(A) RED predicate.** Require all of the following:
   - `rustc-version` is ok.
   - `compile-producer` fails, and its stderr shows exactly one error: E0599, no method `run_steps` on `Machine`.
   - `compile-tests` through `compare` are `not-run`, and the postflight is ok.
   - Terminal state is `failed:compile-producer`.
   - Any other compile error means the result is not the intended RED. Note that the 7 tests' own pre-implementation failure is not separately observed, because that step is blocked.
3. **(B) GREEN predicate.** Require all of the following:
   - Terminal state is `success`.
   - All 8 steps are ok: preflight, rustc-version, both compiles, run-tests, run-producer, compare, postflight.
   - The comparison shows `pass`/`match`, with 8 programs and 216 runs.
   - The artifact, output and comparison SHAs are recorded in the receipt.
4. **The outer receipt must bind the ordinary-QA stage.** Its output, and the `test_*.py` hashes, are not in run_host's receipt.

## Merge-blocking findings (do not block this run)

- **M1 – Uncompiled edits to the existing test closure.**
  - `reference_oracles_tests.rs` now imports only `expr, no_functions, normal_program, uint` from the moved `normal_oracle_inputs`. `boolean`, `var`, `add` and `call` are no longer in its scope.
  - If `helper_call_adapter_tests.rs` or `checkpoint_normal_tests.rs`, which I did not see, use any of those through `super::`, the `mod.rs` umbrella build breaks.
  - The edits to `mod.rs` are also uncompiled.
  - Before merge, either compile that closure under separate admission, or show by inspection that no removed name is used. Otherwise record it as unverified.
- **M2 – No tests for the wrapper or `--host` wiring.** Contract predicate 2 requires ordinary CLI tests of the entrypoint and of fail-stop behavior. There are none for:
  - `--host` ordering (QA, then run_host) and a missing or empty `DOCTORWATSON_RUSTC`;
  - `libtest_results` cases: filtered, extra or failed tests;
  - `compile_inputs` cases: an undeclared or unread file, or one outside the snapshot;
  - `toolchain_channel`, and `compiler_record` rejecting a proxy.

  The (A) and (B) runs give one actual RED and one actual GREEN. They do not test these negative branches.

## Non-blocking findings

- **N1.** When the comparison exits 1, `child()` raises before `comparison.json` is parsed. The receipt then loses `first_difference`, although the raw stdout file is kept. Parse and record it before applying the exit check.
- **N2.** HOST-OBSERVATIONS.md says the producer "never" imports expected data, but `INPUTS` hardcodes the corpus `exact_completion_fuel`. These budgets only select inputs, and a wrong value would fail the run count. Reword it as corpus-derived input selection.
- **N3.** These are limits to state; none is a defect:
  - Snapshot directories stay writable; only the files are 0o444.
  - `run_host.py` is hashed after the interpreter has already loaded it.
  - A nonexistent rustc is recorded as `failed:preflight`, not `blocked`.
  - A child that exits normally can leave grandchildren behind; the group is killed only on timeout.

## Unchanged scope

- The constructor startup step (`uiRun`/rule74) is not executed.
- The K diagnostic text is a projection, not a codec.
- K, Quint, Lean, Clean, handwritten-circuit, native and stagenet layers all stay unrun.
- Native authority stays at zero. C4, the frozen arithmetic attempts and every counter are unchanged.
- The DoctorWatson workflow Quint edits are separate design evidence. I checked that the four new scenarios agree with `applyEdit`/`applySharedInput`, and the suite totals 34. The full 10k sampler is still pending.
