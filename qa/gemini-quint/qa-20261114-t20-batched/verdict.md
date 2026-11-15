# Verdict: T20 batched Midnight writes (qa-20261114-t20-batched)

Lead verdict on the Gemini Carbon Quint lane of 2026-11-14 for T20, over all six fixes: write accounting (Q1), consumer soundness (Q2), data availability (Q3), chunk cap (Q4), registry epochs (Q5).

Lane details:
- Driver and model: interactive `agy --model gemini-4-carbon-high` in herdr pane `carbon-t20` (w1:p15, opened for this lane, closed after). The lane ran before and after the muninn reboot of 18:45; the agy session survived, and the lane resumed after a network error.
- Sources: Moriarty `t20-batched` at `3fdb08cb4` (`SOURCES.md`).
- The lane did not run quint: its sandbox has no `heavy-run`. The lead ran all 38 commands (`rerun.sh`, `lead-cmds.txt`).
- `REPORT.md` is the lane report. It is data. The lead checked each finding against the compiled contracts.

## Lead repair of the lane model

As delivered (`lane-original/`, `rerun-results-unrepaired.txt`), the test file does not typecheck and the sound instance stops with a runtime error. There are two causes:
1. The instance modules (`t20_batched`, `t20_batched_m_*`) import the base with `.*` and do not export it, so a test module that imports an instance sees no names (QNT404). Repair: each test module imports `t20_batched_base` with the parameters of its instance.
2. `hash2` and `hash3` are Cantor pairs. Nested hashes overflow the simulator's integers (QNT601). Repair: keep the parity split (H2 even, H3 odd, both at least 10) and reduce modulo the prime 1,000,000,007. `exhaustiveHashSeparationTest` still passes.

After the repair (`rerun-results.txt`): both files typecheck; `invAll` holds (`[ok]`) on `t20_batched` (strict) and `t20_batched_grace`; all 8 `reach*` witnesses are reached; all 12 mutants violate `invAll` (exit 1), the 10 required ones included. Of 44 deterministic and exhaustive tests, 35 pass. The 6 exhaustive pure checks are among them. The 9 failures all have one cause: the lane writes `.then(X.fail())` in the middle of a run. Quint leaves the state unset after `fail()`, so the next step stops with QNT502 "Variable st not set". This is a harness error, not a design result. The claims of those 9 tests are covered by the exhaustive checks, the mutants and the compiled-contract tests below, except one: the honest-use test of `nullifier_keyed_by_pos` is among the 9 and has no compiled control, so that mutant's non-vacuity is not shown (the honest-use tests of `nullifier_keyed_by_root` and `no_root_read` are backed by the compiled controls).

## 1. Verdict

| Question | Verdict |
| :-- | :-- |
| Q1 write accounting | **CONSISTENT**. SherlockLedger writes are bounded by (epochs with members) + (user-paid runs) + (registry epochs with a change), whatever the number of claims, consumers, keys and the depth. The rate qualifier of F-5 is added to RESULTS |
| Q2 consumer soundness | **SOUND WITH CONDITIONS**. Wrap keys on the two write entries at deployment (F-1); `L` builds `claims_root` only from verified run and foreign heads (F-2, T8b); a consumer computes its leaf in circuit (F-3) |
| Q3 data availability | **SOUND WITH CONDITIONS**. A withheld list delays use and never admits a non-member; the `acceptRun` fallback shares the nullifier when the salt is kept (F-6) |
| Q4 chunk cap | **CONSISTENT**. No measured entry chunks; the cap does not narrow GP-1 to GP-4; a TTL below 1,755 s is refused (F-7) |
| Q5 registry epochs | **SOUND WITH CONDITIONS** (N-8 open). The strict check refuses a stale root and can starve under fast churn; grace trades that for one epoch of a revoked key (F-4) |

## 2. Findings and their counterexamples

Rows are in `tests/quint-regressions/COUNTEREXAMPLES.md`, section `t20-batched`.

| Finding | Verdict | Action |
| :-- | :-- | :-- |
| F-1 write entries carry native keys in the test harness | OPEN DESIGN | A deployment rule (kit LEDGER CONTRACT: wrap key on `acceptRun` and `acceptCollection`, native keys on the read entries). The harness tests consumer logic and keeps native keys on purpose. The next deploy (T10a runner) installs the wrap key |
| F-2 `L` must exclude `ADM` and `VIEW` heads from `claims_root` | OPEN DESIGN | Stated in W-2 (seal premises are never leaves). It is a T8b obligation; the lane mutant `unbound_claims_root` covers it |
| F-3 the empty slot 0 has a valid path in every root that is not full | GUARD HOLDS on `sherlock-claim-consumer`; CONFIRMED on the control `consumer-control-no-kind-tag` | Test `lane F-3` (compiled). W-2 now requires the consumer to compute its leaf in circuit |
| F-4 strict registry check starves under churn; grace admits a revoked key | OPEN DESIGN (N-8) | W-4 states both options and the epoch-length condition |
| F-5 `1 / 3,600` tx/s holds only up to 291 jobs/s | CONFIRMED (prose) | RESULTS fix 1 per-second row qualified |
| F-6 the fallback needs the salt; a foreign claim has no `acceptRun` | OPEN DESIGN | W-2 data availability text extended |
| F-7 TTL below 1,755 s gives `K_MAX` = 0 | OPEN DESIGN | W-5 and ANC-012 refuse such a plan (text only, not in `chunkCap.qnt`, which fixes TTL at 3,600 s; LEAD pick for T15) |
| F-8 one claim at two slots of one root | GUARD HOLDS | Test `lane F-8` (compiled): the second slot is refused, "claim already used". The lead model `claimsRoot.qnt` uses sets and cannot express it; the lane model covers it |
| F-9.1 lead `registryEpoch.qnt`: an add after a revoke in one epoch stays revoked | CONFIRMED (lead model), FIXED | `pos_revoke_then_add_registered`, `pos_add_then_revoke_not_registered` fail before and pass after |
| F-9.2 lead `registryEpoch.qnt`: a no-op change writes | CONFIRMED (lead model), FIXED | `pos_noop_add_no_write` fails before and passes after |
| F-9.3 lead tests run 0 tests without `--match` | NOT REPRODUCED | `run-model.sh` passes `--match '^(pos\|neg\|mut)_'`; the `.out` files show the passing counts |

No finding is a defect of `sherlock-ledger.compact` or `sherlock-claim-consumer.compact` as built. No contract change, no key change.
