# Recursion phase profile: tasks

Boxes record observed outcomes only. Paths are relative to the repository root. Run every build and proof through `scripts/doctorwatson heavy` inside `capped heavy`.

## 1. Design check

- [x] 1.1 Model the accounting rules in Quint: `models/recursion/profile.qnt` and `models/recursion/profile_test.qnt`. Cover RPP-003 (partition sums, 5% bounds, one owner per interval), RPP-004 (peak-delta formula, null counters) and RPP-005 (qualification needs the upper bound). Seed one mutant per rule.
  Verify: `quint test models/recursion/profile_test.qnt --main profile_test` passes and every mutant is caught.
  Observed 2026-10-26: 25 passing, 7 mutants caught, 7 invariants clean; see `models/recursion/PROFILE-RESULTS.md`.

## 2. Identity and isolation (tests first)

- [x] 2.1 Write `scripts/qa/tests/test_profile_identity.py`. Test the archive checksum, the patch checksum and the refusal of a 0.8.2 control (RPP-001-02).
- [x] 2.2 Create the workspace `tools/profile/` with its own `Cargo.lock`. Vendor midnight-proofs 0.8.3 at `tools/profile/vendor/midnight-proofs-0.8.3/`. Check the archive SHA-256 `19b2e4b1…6a702dcc`.
  Verify: `python3 -B scripts/qa/profile-recursion.py prepare --check-only`.
- [x] 2.3 Add a T0 check that the main workspace never enables `recursion-phase-profile` (RPP-007-02).
  Verify: `git diff --exit-code Cargo.lock` after the change, and the T0 check fails on a seeded feature leak.
  Observed 2026-10-26: `qa/tiers/t0.sh` runs `scripts/qa/check-profile-isolation.py`; the seeded-leak unit tests fail the check as required; the main `Cargo.lock` is unchanged.
  Implemented standalone isolation check and seeded-leak tests; T0 wiring is reserved for the lead.

## 3. Observation feature (tests first)

- [x] 3.1 Write tests for the bounded event buffer, overflow, timer failure and the reentrancy guard in `tools/profile/tests/profile_events.rs`. The tests sit in the harness crate, because the vendored crate's own dev-dependencies (`criterion`) are not in the offline cache.
  Verify: `RUSTC_WRAPPER= CARGO_TARGET_DIR=tools/profile/target cargo test --offline --manifest-path tools/profile/Cargo.toml --features recursion-phase-profile --test profile_events`.
  Observed 2026-10-26: 11 passed, 0 failed (overflow, reentrancy, regressing clock, returned-error span, thread ids, public-only JSON, opt-in flush).
- [x] 3.2 Add the default-off feature `recursion-phase-profile` and RAII timers at every phase boundary of RPP-002, including returned-error paths.
  Verify: the 3.1 command, plus `cargo check` of `tools/profile` with and without the feature.
  Observed 2026-10-26 (lead): the 3.1 tests pass. A whitespace-insensitive review of the vendored diff shows only span wrappers around existing expressions; the multiopen fold changed formatting only. A dropped, unfinished span records status 2 (returned error). Proof behavior is still unverified until task 5.1 runs the on/off proofs.
  Observed 2026-10-26: feature-on/off isolated `cargo check --offline` passed (four jobs, `tools/profile/target`, sandbox-blocked `RUSTC_WRAPPER` unset); event-test verification remains blocked offline.
- [x] 3.3 Add caller timers around key load, `setup_pk`, prove and verify in the harness crate `tools/profile/src/`, which calls the `mori-recursion` and `mori-backend` APIs. The main crates stay unchanged: the registry midnight-proofs has no `recursion-phase-profile` feature, so forwarding it from a main crate would break production builds (RPP-007).
  Observed 2026-10-26: both prescribed harness `cargo check` variants passed; 56 Python tests passed; `check-profile-isolation.py` reported OK, `prepare --check-only` verified the nonempty patch SHA-256, and `git diff --exit-code Cargo.lock` passed; no proof was run.
- [x] 3.4 Add the RSS sampler, process CPU clocks, typed byte counters and unavailable-counter reasons.
  Verify: `RUSTC_WRAPPER= CARGO_TARGET_DIR=tools/profile/target CARGO_BUILD_JOBS=4 cargo test --offline --manifest-path tools/profile/Cargo.toml --features recursion-phase-profile --tests`.
  Observed 2026-10-26: 20 integration tests passed (8 counters, 11 events, 1 sidecar), including strict interior RSS attribution, boundary-only quality, process CPU/I/O deltas, unavailable/regressing counters, overlapping proofs, sampler shutdown and expected verifier-control errors. Collection lives in the harness; the vendor adds only an optional boundary callback and shared monotonic clock. No proof was run.

## 4. Sidecar, validator and reducer (tests first)

- [x] 4.1 Write `scripts/qa/tests/test_profile_reduce.py`. Cover RPP-003-01 to 03, RPP-004-01 and 02, RPP-005-01 and 02, RPP-006-02 and RPP-009-01 and 02 with synthetic sidecars.
- [x] 4.2 Implement schema and semantic validation against `openspec/changes/recursion-phase-profile/profile.schema.json`.
- [x] 4.3 Implement `scripts/qa/profile-recursion.py` with `prepare`, `calibrate`, `one`, `reduce` and `decide`. Write sidecars under `qa/sherlock/profiles/<run>/`.
  Verify: `python3 -B -m unittest discover -s scripts/qa/tests -p 'test_profile_*.py'`.
  Observed 2026-10-26: 47 unittest tests pass with synthetic sidecars; no build or profile run was executed. Command execution still requires the task 3 executable and feature.

## 5. Small fixtures and overhead

- [ ] 5.1 Profile A2 (A102 k13, A80 k12) with observation on and off. Check that stock verification accepts both and rejects the malformed controls (RPP-006-01).
  Code implemented: `profile-a2` reproduces the linked public fixture with child caller timers, raw counter sidecars, stock verification and malformed-proof/statement plus wrong-key controls. `one --workload A2` plans both variants under `a2-linked-children`; resident-PK requests are refused because the public child APIs rebuild the PK. Real on/off proofs and qualification remain unrun; this box stays unchecked.
  Observed 2026-10-26: `RUSTC_WRAPPER= CARGO_TARGET_DIR=tools/profile/target CARGO_BUILD_JOBS=4 cargo check --offline --manifest-path tools/profile/Cargo.toml --bins` passed both without the feature and with `--features recursion-phase-profile`; 61 Python tests passed, isolation reported OK, and `prepare --check-only` verified the regenerated patch/identity. These are code checks, not proof-run evidence.
- [ ] 5.2 Run at least five paired on/off A2 runs. Report the one-sided 95% overhead bound (RPP-005).
  Verify: `python3 -B scripts/qa/profile-recursion.py calibrate --workload A2 --pairs 5`.
  Amended 2026-11-08 (3): this is task 2.8 of `openspec/changes/sherlock-vm-consensus-20261107/tasks.md` (row A15, part); it runs only after that change's memory guard (task 2.0, F18) has landed.
- [ ] 5.3 GP node-chain workload (added 2026-11-08 (3) by the Sherlock speed re-plan, `research/sherlock-speed-replan-20261108/order.md` §2; task 2.7 of the consensus change, row F19): re-pin `tools/profile/Cargo.toml` to zk-stdlib 2.4.0 / circuits 7.3.0 and regenerate `tools/profile/Cargo.lock` offline after the main workspace's task 2.1; add `tools/profile/src/bin/profile-gp.rs` (the `gp_fold_and_key` chain through the public APIs: key load, `setup_pk`, each node `prove` and `verify`, decider keygen and prove, ledger verify; stand-in leaves via `MORIARTY_GP_STANDIN_STEPS`, real leaves via `MORIARTY_GP_LEAF_DIR`; `--threads` recorded in the sidecar host block; CPU% and RSS per phase; the node row count with its per-gadget breakdown); add the `GP` workload to `BUDGETS` and `planned_commands` (budget `c0-gp-chain`, seeded 1,800 s / 48 GB); add `decide` rows C11-C14 for the four levers (threads, k=19, S, GPU; tree/concurrency criteria from `tree.md` §7).
  Verify: `prepare --check-only`, `check-profile-isolation.py`, `unittest discover -s scripts/qa/tests -p 'test_profile_*.py'`, `cargo check` of `tools/profile` with and without the feature.
  Code implemented 2026-11-08: `scripts/qa/check-f19.sh` runs the four verify steps (F19). `prepare` also checks the profile pins against the root and finds the archive under `$CARGO_HOME`; the vendor `Cargo.lock` (ignored by the vendor's own `.gitignore`, so absent from clones) is restored from the archive. The schema gains workload `GP` and proof shapes `leaf`, `node`, `run_decider`. No GP proof has run; the node row breakdown runs only inside `profile-gp`.

## 6. Full matrix

- [ ] 6.1 Profile the GP node chain (amended 2026-11-08 (3); was A3 P and A4 D, whose rows are retired): `one --workload GP --cache-mode resident_pk --variant both --run` at `RAYON_NUM_THREADS` 4, 10 and 20 (run ids `c0-gp-t4`, `c0-gp-t10`, `c0-gp-t20`) under `c0-gp-chain` and `node-threads-{4,10,20}`; then the concurrency probe (three genesis node proofs at 6 threads under `heavy --concurrent 3 --aggregate-cap 100G`, budget `node-concurrent-3`). This is task 2.9 of the consensus change (rows A15, A17, A23). The decider is profiled in the same run under `a4-terminal-d`.
- [ ] 6.2 Profile the B3 one-batch command with all wrapper intervals.
- [ ] 6.3 Run `decide`. Write `wiki-llm/recursion-perf/profiles/c0-first-profile.md` with the deciding metric of each row C1–C10 and, since 2026-11-08 (3), C11–C14 (threads, k=19, S, GPU) plus d, m, q, U, f3 and the wall per thread count that gate G-A of the consensus change reads.
  Verify: `python3 -B scripts/qa/profile-recursion.py decide --run <run>` exits 0, and every row has a metric or an explicit unresolved reason.

## 7. Regression

- [ ] 7.1 Run the normal regression checks with profiling off (amended 2026-11-08 (3): A3 and A4 are retired stagenet rows; the regression rows are A1, A15 and the chain rows A18, A19, A19b of the consensus change, plus B3).
  Verify: `scripts/doctorwatson check A1 A15 B3` pass, and the chain rows pass when their milestone reaches them.
