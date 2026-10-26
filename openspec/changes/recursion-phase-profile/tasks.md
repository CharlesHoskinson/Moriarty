# Recursion phase profile: tasks

Nothing is checked. Boxes record observed outcomes only. Paths are relative to the repository root. Run every build and proof through `scripts/doctorwatson heavy` inside `capped heavy`.

## 1. Design check

- [x] 1.1 Model the accounting rules in Quint: `models/recursion/profile.qnt` and `models/recursion/profile_test.qnt`. Cover RPP-003 (partition sums, 5% bounds, one owner per interval), RPP-004 (peak-delta formula, null counters) and RPP-005 (qualification needs the upper bound). Seed one mutant per rule.
  Verify: `quint test models/recursion/profile_test.qnt --main profile_test` passes and every mutant is caught.
  Observed 2026-10-26: 25 passing, 7 mutants caught, 7 invariants clean; see `models/recursion/PROFILE-RESULTS.md`.

## 2. Identity and isolation (tests first)

- [ ] 2.1 Write `scripts/qa/tests/test_profile_identity.py`. Test the archive checksum, the patch checksum and the refusal of a 0.8.2 control (RPP-001-02).
- [ ] 2.2 Create the workspace `tools/profile/` with its own `Cargo.lock`. Vendor midnight-proofs 0.8.3 at `tools/profile/vendor/midnight-proofs-0.8.3/`. Check the archive SHA-256 `19b2e4b1…6a702dcc`.
  Verify: `python3 -B scripts/qa/profile-recursion.py prepare --check-only`.
- [ ] 2.3 Add a T0 check that the main workspace never enables `recursion-phase-profile` (RPP-007-02).
  Verify: `git diff --exit-code Cargo.lock` after the change, and the T0 check fails on a seeded feature leak.

## 3. Observation feature (tests first)

- [ ] 3.1 Write tests for the bounded event buffer, overflow, timer failure and the reentrancy guard in `tools/profile/vendor/midnight-proofs-0.8.3/tests/profile_events.rs`.
- [ ] 3.2 Add the default-off feature `recursion-phase-profile` and RAII timers at every phase boundary of RPP-002, including returned-error paths.
  Verify: `cargo test --manifest-path tools/profile/Cargo.toml --locked --features recursion-phase-profile profile_`.
- [ ] 3.3 Add caller timers around key load, `setup_pk`, prove and verify in `crates/mori-recursion` and `crates/mori-backend`, behind the same feature name.
- [ ] 3.4 Add the RSS sampler, process CPU clocks, typed byte counters and unavailable-counter reasons.

## 4. Sidecar, validator and reducer (tests first)

- [ ] 4.1 Write `scripts/qa/tests/test_profile_reduce.py`. Cover RPP-003-01 to 03, RPP-004-01 and 02, RPP-005-01 and 02, RPP-006-02 and RPP-009-01 and 02 with synthetic sidecars.
- [ ] 4.2 Implement schema and semantic validation against `openspec/changes/recursion-phase-profile/profile.schema.json`.
- [ ] 4.3 Implement `scripts/qa/profile-recursion.py` with `prepare`, `calibrate`, `one`, `reduce` and `decide`. Write sidecars under `qa/sherlock/profiles/<run>/`.
  Verify: `python3 -B -m pytest scripts/qa/tests/test_profile_*.py`.

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
