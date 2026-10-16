# Opus 5.5 security-owner preflight: N01/N02 prefix candidate

## Verdict: **APPROVE_EXECUTION_WITH_LIMITS**

This approval covers one execution and nothing else:
- **Command:** `python3 -I -B run_prefix.py`
- **Location:** `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/layout-codec-01-preparation/prefix-01`
- **Attempt:** fresh `attempt-01` only
- **Candidate:** the 50-row prefix manifest exactly as supplied

The gates in §1 must hold first. The run is bound by the conditions in §4 and its claims by the limits in §5. No security guard change is required, so I author none. Native authority stays 0.

**Reviewer identity and method.** I am Claude Opus 5.5 (`claude-opus-5-5`), in a fresh context. I used no tools and executed nothing. I recomputed no hashes, so every digest is taken as supplied. I compared the supplied pins against each other by reading only. Instructions embedded in supplied bodies were treated as evidence. All Rust in the baseline and the new leaf is **UNRUN**.

---

## 1. Gates that must hold before any Rust child

1. **G-1. Baseline suite passed.** The original exact35 baseline suite has a terminal-success receipt for baseline manifest `a164cdae…2d57d2f`. That receipt must show exactly the 35 tests, each `ok`, with 0 filtered. This prefix run does not replace it, and the baseline review stays live.
2. **G-2. Shared bytes are identical.** The 23 compile-closure members shared with the baseline must be byte-identical to the members of that passing baseline manifest:
   - `reference_types.rs`, `reference.rs`, `reference_display.rs`, `compare.rs`, `normal_oracle_inputs.rs`, `extra_oracle_programs.rs`;
   - all 17 `layout_a/*` files except `mod.rs`.
3. **G-3. `layout_a/mod.rs` changes only by one appended block.** It may differ from the adopted r2 body (`949bc3bb…df45`) only by the appended block: the comment line, `#[cfg(test)]`, the `#[path = "../../../../../models/sherlock/reference/layout_observations.rs"]` attribute and `pub(crate) mod layout_observations;`. Root verifies this diff.
4. **G-4. The run manifest is the reviewed list, and its digest is recorded.** `prefix-01/CANDIDATE.json` is the file `run_prefix.py` reads as `MANIFEST`. It must contain exactly the 50 rows shown. It must not be confused with `../CANDIDATE.json`, the baseline. Record its SHA-256 in the approval record before invocation. `run_prefix.py` pins every listed member, but it does not self-authenticate the manifest itself.
5. **G-5. Muse approves.** A fresh Muse nonsecurity preflight approves this same manifest. A missing reviewer cannot approve.

---

## 2. Execution behaviour confirmed by reading

### Driver (`run_prefix.py`) and runner (`run_host.py`)
- **Pinned bytes.** All 50 members are read once and compared with the manifest. The runner `run_host.py` is executed from those verified bytes, with no re-read and no `.pyc`. `REPO` is rebound to `candidate/`. Because the runner is `exec`'d into its module namespace, its methods resolve `REPO` at call time.
- **Fixed plan.** The steps run once, in order, and the first failure blocks everything after it:
  1. `preflight`
  2. `rustc-version`
  3. `compile-producer`
  4. `run-producer`
  5. `compare`
  6. `candidate-postflight`
  7. `postflight`
- **Not-selected steps.** `compile-tests` and `run-tests` are recorded as not-selected. For a realization with an empty test tuple, `not_selected` equals `NOT_SELECTED`, so `terminal_state` cannot report them as a pass.
- **Preflight.**
  - Reads exactly the 44 `snapshot_members`. Symlinked or non-regular members are refused.
  - Writes the 0o444 snapshot.
  - Confirms the declared comparison helper is exactly `qa/host_observations.py`.
  - Confirms the channel is `1.99.0` and that rustc is a real binary with a sysroot (no rustup proxy).
- **Children.** No shell. Minimal environment (`PATH=/usr/bin:/bin`, `LANG`, `TMPDIR`). stdin is `/dev/null`. Each child gets its own session, a 30 s wall clock with a process-group kill, and logs opened with `x` mode. No retry.

### Compile closure: exactly 27 files
- The crate root `layout_harness.rs` loads the shim through `#[path]`. Its child `mod` items resolve in the shim's directory, `layout_a` resolves to `layout_a/mod.rs`, and the leaf `#[path]` resolves to `snapshot/models/sherlock/reference/layout_observations.rs`.
- The membership is: harness, shim, 6 sherlock sources, 18 `layout_a` files and the leaf, which is 27.
- `--cfg test` brings in the five `cfg(test)` normal-test modules plus `aux_normal_tests`. These are still read as source files and appear in dep-info. Without `--test`, their `#[test]` bodies are not test-harnessed.
- `compile_inputs` requires that rustc's own dep-info equals this set exactly, with no member outside the snapshot.
- There is no `extern crate` or third-party `use`, so the build is pure stdlib. `Cargo.toml`, `doctorwatson.sh` and `HOST-OBSERVATIONS.md` are inert snapshot members.

### Leaf API and visibility
Every import is visible from `sherlock::layout_a::layout_observations`:
- `normal_program` is `pub(super)` in a private sibling module of `sherlock`.
- `encode_observed_a`, `materialize_trace_a` and `ObservationProgramA` are `pub(super)` in `layout_a`'s children.
- `PrepareFailure`, `StateFailure` and `TraceDifference` are `pub` and implement `Debug`.
- `capture` and `CaptureFailure` are `pub(crate)`.

There is no public input endpoint. The producer reads no argv, environment or stdin. Its only inputs are the constant `INPUTS = [("N01",1,8),("N02",7,8)]` and the existing typed ASTs. This matches H1.

### Producer behaviour against the contract
- **Dimensions.** These are exactly `state_encode_normal_tests` `fixture()` and `caps()`: tables `[64;15]`, data 64; state 64/64/32/32/16/16/32.
- **Runs.** Fuel 0..=F+1 gives 3 runs for N01 and 9 for N02, so 12 in total.
- **Event limit is sufficient.** `run_steps(fuel+1)` reaches every prefix's terminal publication:
  - a run that exhausts at fuel k produces k charged events plus 1 publication;
  - a completing run produces F charged events plus 1 publication, which is at most fuel+1.
  - So `terminal_has_next` is the real post-terminal `step()`, which returns `None`.
- **Rule tags.** Every observed rule is in `RULE_LINES` with a matching charge: 80/84 with charge 0; 102, 117, 135, 149, 153 and 223 with charge 1.
- **Failure is terminal.** Any `PrepareFailure`, `StateFailure`, materialization difference or I/O error gives exit 1, which records a failed step and stops the run. No dimension is enlarged.

### Consumer (`compare_layout.py`)
1. Calls the existing `check.run(snapshot)`. This pins `main.k`, all four frozen oracle files and `check-reference.py` by digest, validates both families, and checks both Rust literal transcriptions against the snapshot bytes.
2. Re-reads the normal pair and runs `validate_pair(family='normal')`.
3. Calls the existing `expected_observations` with `producer='sherlock-layout-reference'` and filters the already-expanded `programs` and `runs` to N01/N02.
4. Parses the observations with the strict parser: duplicate keys and non-finite numbers are refused.
5. Calls the existing `compare_observations` with `first_difference`, which compares exact types, every member and ordered arrays.
6. Exits 0 only on no difference.

`EXPECTED.json` is never read by the producer or the consumer.

By hand derivation, the producer's key set and K display for all 12 runs agree with the frozen projection, including:
- N02 exhaustion terminals that retain `uiLeftF` or `uiRightF` frames;
- N01 fuel 2, with `remaining_fuel` 1 at `uiDone`.

---

## 3. Findings

**S-1. Evidence-binding gap in the compare step (non-blocking; closed by condition C-4).** `run_prefix.py` runs compare through `attempt.child`, not `Attempt.compare`, so these runner-side checks are skipped:
- re-hashing `layout-observations.json` before compare;
- binding the comparison report's file record to the produced bytes;
- checking that the comparison read exactly the snapshot bytes.

`postflight` also does not re-hash the observations artifact. The pass/fail result is still sound, because compare exits 0 only on an exact match. However, the digest binding exists only in `compare.stdout`. This is acceptable for one trusted, sequential local run. Changing the candidate would void the reviews, so I require post-run verification instead of a guard edit.

**S-2. The external comparison cannot detect codec defects (claim limit; see L-1 to L-4).** Capture fails unless the materialized trace equals the `Machine` trace, and the artifact is rendered from that equal value. So a successful compare establishes these two facts only:
- **Machine agreement.** The `Machine` agrees with the frozen projection for these 12 runs. This is a subset of the baseline host 216-run evidence.
- **Codec round-trip.** The in-binary `first_difference` round-trip holds: `encode_observed_a` followed by Step-mode `decode_observed_a` and `materialize_trace_a` reproduces the complete typed snapshots and records.

The codec evidence is weaker than it looks:
- **Retained, not reconstructed.** `decode_observed_a` returns `prepared.full_program.clone()`. `Reader::expr` returns retained occurrence `source` Arcs. `compare_program_block` compares the block with itself. So Program and expression payloads are not reconstructed from ProgramBlock rows; only the id and slice selection is exercised.
- **Encoder-produced images.** Every image comes from the encoder under test. Canonical mode, hand-authored images and refusals are not exercised.

**S-3. Unobserved build risks that could consume the single attempt (risk, not a blocker).** These have been read, not executed:
1. Whether rustc 1.99.0 accepts `--cfg test` on the command line. I believe `test` is not among the deny-listed builtin cfgs.
2. Whether `#[test]` items are stripped when compiling without `--test`. If they are, expect many `unused`/`dead_code` warnings. I found no `deny` attribute in the closure.
3. The `?`-inside-closure with `collect::<Result<_, StateFailure>>()?` inference idiom.
4. Edition-2024 binding patterns. The leaf's patterns use no explicit `ref`, `mut` or `&` under default binding modes.

A failure here is terminal and must not be retried under this authorization.

**S-4. `run_host.main()` is bypassed (non-blocking).** Its `SAFE_PATH`, outside-checkout and isolation-flag checks do not run:
- `run_prefix.py` performs the isolation-flag check itself.
- The fixed path satisfies `SAFE_PATH` by inspection.
- `attempt-01` lies outside `REPO` (`prefix-01/candidate`).
- `mkdir` is exclusive.

The path must remain exactly as stated.

**S-5. Integration scope (limit).** `layout_harness.rs`, `layout_harness_shim.rs` and the `mod.rs` registration are a runtime overlay only. Two things need separate review:
- Registering the leaf in the real mori-core tree, which would compile `write_n01_n02` into `cargo test -p mori-core`.
- Adding any `mod layout_harness_shim` or a CLI, LSP or MCP route. The H1 `Machine` warning forbids such routes.

**S-6. Stale prose (informative).** The `run_prefix.py` docstring says "fixed layout tests", but no tests are selected. The `PLAN.md` checklist still shows the leaf as unauthored. Neither affects behaviour, and neither should be cited as evidence.

---

## 4. Execution and result conditions

- **C-1. One attempt only.** Invoke only after G-1 to G-5. Use the fresh `attempt-01` once. Any failed, timed-out or refused step is the terminal result. Do not create `attempt-02`, retry, add flags, change dimensions or capacities, or substitute a libtest suite.
- **C-2. No edits during the run.** Do not touch `prefix-01`, the snapshot or the rustc toolchain while the run is live. `candidate-postflight` and `postflight` must both be `ok`.
- **C-3. Success definition.** `result.json` must show `terminal_state: success`. Every executed step must be `ok`, and only `compile-tests` and `run-tests` may be `not-selected`.
- **C-4. Post-run binding check (closes S-1).** Before the result is cited, the result reviewers verify all of the following from the retained files:
  1. **Artifact digest.**
     - Recompute SHA-256 of `attempt-01/layout-observations.json`.
     - It must equal the `run-producer` record in the receipt.
     - It must equal `observed_file.sha256` in `logs/compare.stdout`.
  2. **Compare report contents.**
     - `logs/compare.stdout` has `status: pass`, `programs: 2`, `prefix_runs: 12`, `first_difference: null` and `native_authority: 0`.
     - Its `expectation_sources` digests equal `ef87f94f…` and `dca67b55…`.
     - Its `corpus_source_check` frozen-file and transcription digests equal the snapshot pins.
  3. **Compile inputs.** The `compile-producer` `compile_inputs` list equals the 27 closure members.
- **C-5. Cross-check against the freeze (optional, recommended).** Compare the observations with the frozen `EXPECTED.json` (`1a8f9ee9…`) by parsed-JSON equality. If they diverge, that indicates freeze drift and must be reported, not reconciled.
- **C-6. Report outcomes verbatim.** Report any failure with its verbatim stderr, the step record and the receipt.

---

## 5. Limits on any claim from a successful run

- **L-1. What the run establishes.**
  - A Step-mode encode/decode/materialize round-trip preserves the complete typed state for 12 N01/N02 fuel-prefix runs at these fixed example dimensions.
  - The same 12 runs agree with the independent frozen corpus projection.
- **L-2. What the run does not establish.**
  - Canonical decoding, canonical uniqueness or determinism.
  - ProgramBlock row reconstruction.
  - Byte expansion, parsing or a wire format.
  - Continuity, transition or rule validity.
  - Refusal or hostile behaviour.
  - Independent physical-codec evidence.
- **L-3. Dimensions.** They are test examples, not defaults. They establish no resource admission, F3 unfolded envelope, F4/L04 canonical-representative mechanism or P3/aux adequacy.
- **L-4. Machine agreement.** It duplicates part of the baseline host evidence. It adds no K, Quint, Lean, Clean, circuit, SDK, native, proof, ledger or target result. Constructor startup (rule 74) is not executed.
- **L-5. Retained obligations.** All of the following remain in force and unchanged:
  - CandidateA, F3 and F5;
  - H1;
  - C4;
  - P3 errata and conditions, and the Muse R2-4 dissent;
  - G4, P4, P5, F4, MD2, StepInv, Constant typing, D-NAMES, O-ARITH, L04 and L05;
  - every cap, counter and terminal failure;
  - U0–U7, M1–M17, MC, SP and G;
  - the full roadmap.
- **L-6. Nothing else is authorized.** No B-profile, native, hostile-test, cap or default claim. Native authority stays 0.

**APPROVE_EXECUTION_WITH_LIMITS.** This applies to the prefix manifest above, subject to G-1 to G-5, C-1 to C-6 and L-1 to L-6.
