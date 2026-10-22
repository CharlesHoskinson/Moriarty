# FSYN tasks

All boxes refer to observed outcomes. Nothing is checked in this draft. Paths are relative to the repository root. `$R` is `~/research/rollup-subarch-2026-10-21`. `$F` is `~/projectfenrir/recursion/rollup-test-format`. Run checks locally; hosted CI stays off.

## P0 Foundation (runs now)

- [ ] 0.1 Write tests for the cut check first: `models/sherlock/rollup-synth/tests/test_cut.py`.
- [ ] 0.2 Implement `models/sherlock/rollup-synth/cut.py` with `check` and `refresh` commands.
  Verify: `python3 -B models/sherlock/rollup-synth/cut.py check --cut openspec/changes/sherlock-synthetic-rollup-data/design-cut.json`.
- [ ] 0.3 Write extractor tests with one agreeing and one disagreeing constant pair.
- [ ] 0.4 Implement `models/sherlock/rollup-synth/extract.py`. Write `constants.json`, `layouts.json`, `rules.json` under `models/sherlock/rollup-synth/extracted/`.
  Constants include `DOM_FWDR2`, `DOM_XCM`, `DOM_HT` and `DOM_HB` from the exit specs.
  Verify: `python3 -B models/sherlock/rollup-synth/extract.py --cut openspec/changes/sherlock-synthetic-rollup-data/design-cut.json --out models/sherlock/rollup-synth/extracted`.
- [ ] 0.5 Create the standalone package `models/sherlock/rollup-synth/fsyn/` with its own `Cargo.lock`. Pin the RTF-018 crate set.
- [ ] 0.6 Write stream tests: one changed stream leaves other streams byte-identical.
- [ ] 0.7 Implement `fsyn/src/rng.rs` with the named SHA-256 counter streams of design section 4.1.
- [ ] 0.8 Write the S0 byte-identity test against `rtf generate` output.
- [ ] 0.9 Implement the S0 emitter for FRTF v3, both profiles, minimal and b1 to b6.
  Verify: `cargo test --manifest-path models/sherlock/rollup-synth/fsyn/Cargo.toml --locked s0_`.
- [ ] 0.10 Compare trees. Command: `diff -r /tmp/fsyn-s0 /tmp/rtf-v3`, after `rtf generate --out /tmp/rtf-v3`.
- [ ] 0.11 Run `rtf verify --dir /tmp/fsyn-s0`. Record the output in the G0 record.
- [ ] 0.12 Add catalog schema `sherlock-benchmark-workloads/3` tests. Keep every /2 case byte-identical.
- [ ] 0.13 Gate G0: record `models/sherlock/rollup-synth/gates/G0.json` with every command, exit code and digest.

## P1 Production shape and oracles (runs now)

- [ ] 1.1 Write schedule tests: 50 headers, 6 s period, record cap 642, FROZEN cap 560.
- [ ] 1.2 Implement `fsyn/src/schedule.rs` and the share counter. Test share counts 6,225, 7,615 and 8,192.
- [ ] 1.3 Write event tests for each leaf step kind and each of the eight active forced outcomes.
- [ ] 1.4 Implement `fsyn/src/events.rs` with the three presets of design section 4.3.
- [ ] 1.5 Implement `fsyn/src/replay.rs` over the 23-field `S5` state. Label its output internal consistency.
- [ ] 1.6 Run `python3 ~/research/fenrir-rollup-design-2026-10-19/scripts/build-rev20.py --check` inside `cut.py check`.
- [ ] 1.7 Implement emitters in `fsyn/src/emit/`: `stmt`, `ext_digest`, `bc_digest`, `stmt_da`, `stmt_dm`, `stmt_lc`, FDAC, claim leaves.
- [ ] 1.7a Write exit tests first: the shared routed key per note-exit transition, FWDR2 leaves, hold at execution, one hold per capability, applied release leaves the note unspent then withdrawal applies, and route-specific TRANSFER versus WITHDRAW/escape/payout disclosure.
- [ ] 1.7b Implement exit kinds, holds and `public.json` and `private.json` transcripts in `fsyn/src/emit/exit.rs`.
  Verify: `quint test $R/models/exit_core.qnt --main exit_tests` and forward replays against `exit_fixed`.
- [ ] 1.7c Implement key-preserving rotation and migration scenarios with wind-down exits.
- [ ] 1.8 Implement the Celestia chain, synthetic validator set, NMT shares and range proofs.
- [ ] 1.9 Write the fidelity lint: every emitter needs a row in `models/sherlock/rollup-synth/fidelity.json`.
- [ ] 1.10 Implement forward replay emission to `models/sherlock/rollup-synth/replay/<case>.qnt`.
  Verify: `quint test models/sherlock/rollup-synth/replay/<case>.qnt --main <case>`.
- [ ] 1.11 Implement reverse replay from ITF traces.
  Sample: `quint run $R/models/cel_auth.qnt --main cel_fixed --max-samples 2000 --n-traces 2000 --max-steps 20 --out-itf <out>`.
- [ ] 1.12 Run the DA table check: `python3 $R/models/gen_da_table.py --check`.
- [ ] 1.13 Run the parameter gate: `python3 $R/models/liv_params_check.py` against extracted constants.
- [ ] 1.14 Write golden vector files in `models/sherlock/rollup-synth/golden/`. Bind each to its source and cut hash.
- [ ] 1.15 Write property tests for every row of design section 6.3. Use `proptest` with saved regression seeds.
- [ ] 1.16 Generate S1 (batches 1 to 6), S2 (two headers) and S3 (one batch). Record realized counts.
- [ ] 1.17 Gate G1: record `gates/G1.json` with replay, property, golden and Lean assertion results.

## P2 Adversarial, mutation and fixtures (runs now)

- [ ] 2.1 Write the coverage check: 14 adversary classes and 2 exit attackers each need one case.
- [ ] 2.2 Emit the adversarial families of design section 7.4 under `fixtures/<fid>/adversarial/`.
- [ ] 2.3 Import the 11 FRTF v2 and 8 FRTF v3 invalid cases with their layers.
- [ ] 2.4 Register generator mutants in `models/sherlock/rollup-synth/mutants.json`.
- [ ] 2.5 Run the existing Quint mutants, the 11 `exit_core.qnt` mutants included. Each fails at least one replay.
- [ ] 2.6 Add a bolero harness for `frtf-x/1` decode. Run it 300 s on each format change.
- [ ] 2.7 Release `fsyn-fixtures/1`. Write `MANIFEST.json` and `FIXTURE.json` per fixture.
- [ ] 2.8 Pin Celestia nmt test vectors by remote, branch and full commit. Store receipts under `raw/`.
- [ ] 2.9 Regenerate the release on a second host. Compare manifests byte for byte.
- [ ] 2.10 Add `fsyn cut check` to the ordinary DoctorWatson corpus check.
  Verify: `./scripts/doctorwatson.sh` reports the cut check result.
- [ ] 2.11 Write the handoff for the Fenrir format owner: `frtf-x/1` layout, open questions, findings.
- [ ] 2.12 Gate G2: record `gates/G2.json` with mutant kills, class coverage and two-host identity.

## P3 Profile A recursion (trigger: A0, A1, A2, re-keyed D at k <= 20; SRPOC-002 observation)

- [ ] 3.1 Confirm the trigger artefacts and record their hashes in `gates/P3-trigger.json`.
- [ ] 3.2 Benchmark S0 minimal, b1, b2, b4 and b6 per layer. Write SBENCH /2 samples.
- [ ] 3.3 Measure the tree critical path from the last record to the D proof.
- [ ] 3.4 Run every FRTF invalid case. Record the rejecting layer and reason code.
- [ ] 3.5 Gate G3: qualify SBENCH `initial-deployed-baseline` and `full-rollup-series`.

## P4 Profile B leaves (trigger: production keys at k <= 20 on pinned crates)

- [ ] 4.1 Confirm the trigger keys and record their hashes in `gates/P4-trigger.json`.
- [ ] 4.2 Measure throughput per leaf type with 1,000 seeded inputs a key.
- [ ] 4.3 Run the S2 headers through `L_inc`, DAD and DMD. Report E-RB-8 header costs.
- [ ] 4.4 Run the S3 batch. Report V, I, the fleet factor and hosts at 100 TPS.
- [ ] 4.5 Measure decider and contract call costs: bytes and DUST per call.
- [ ] 4.6 Gate G4: publish the E-RB-8 report against 45.0 to 101.3 M core-s.

## P5 Sustained (trigger: G4 and an allocated host set or GPU candidate)

- [ ] 5.1 Run S4 at each offered load point for at least 12 batches.
- [ ] 5.2 Publish the settled-TPS curve with measured and projected series labelled.
- [ ] 5.3 Deliver E-RB-2 input sets: 1,000 inputs and expected public instances per production key.
- [ ] 5.4 Gate G5: publish `fsyn-bench-report/1` with raw samples, failures and censored runs.

## Close-out

- [ ] 6.1 Update the Sherlock Graphify layer, the wiki log and the Diataxis how-to for FSYN.
- [ ] 6.2 Obtain the current designated independent reviews before each gate record is cited.
