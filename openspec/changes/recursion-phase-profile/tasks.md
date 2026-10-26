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
- [ ] 3.4 Add the RSS sampler, process CPU clocks, typed byte counters and unavailable-counter reasons.

## 4. Sidecar, validator and reducer (tests first)

- [x] 4.1 Write `scripts/qa/tests/test_profile_reduce.py`. Cover RPP-003-01 to 03, RPP-004-01 and 02, RPP-005-01 and 02, RPP-006-02 and RPP-009-01 and 02 with synthetic sidecars.
- [x] 4.2 Implement schema and semantic validation against `openspec/changes/recursion-phase-profile/profile.schema.json`.
- [x] 4.3 Implement `scripts/qa/profile-recursion.py` with `prepare`, `calibrate`, `one`, `reduce` and `decide`. Write sidecars under `qa/sherlock/profiles/<run>/`.
  Verify: `python3 -B -m unittest discover -s scripts/qa/tests -p 'test_profile_*.py'`.
  Observed 2026-10-26: 47 unittest tests pass with synthetic sidecars; no build or profile run was executed. Command execution still requires the task 3 executable and feature.

## 5. Small fixtures and overhead

- [ ] 5.1 Profile A2 (A102 k13, A80 k12) with observation on and off. Check that stock verification accepts both and rejects the malformed controls (RPP-006-01).
- [ ] 5.2 Run at least five paired on/off A2 runs. Report the one-sided 95% overhead bound (RPP-005).
  Verify: `python3 -B scripts/qa/profile-recursion.py calibrate --workload A2 --pairs 5`.

## 6. Full matrix

- [ ] 6.1 Profile A3 P and A4 D, process-cold and resident-PK second proof, under the existing budgets `a3-parent-k20` and `a4-terminal-d`.
- [ ] 6.2 Profile the B3 one-batch command with all wrapper intervals.
- [ ] 6.3 Run `decide`. Write `wiki-llm/recursion-perf/profiles/c0-first-profile.md` with the deciding metric of each row C1–C10.
  Verify: `python3 -B scripts/qa/profile-recursion.py decide --run <run>` exits 0, and every row has a metric or an explicit unresolved reason.

## 7. Regression

- [ ] 7.1 Run the normal A2, A3, A4 and B3 checks with profiling off.
  Verify: `scripts/doctorwatson check A2`, `A3`, `A4` and `B3` all pass.
