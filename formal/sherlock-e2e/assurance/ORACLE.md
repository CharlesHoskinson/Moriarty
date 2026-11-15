# Assurance oracle: `--step` interface

Context: Assurance (slice B, `research/sherlock-e2e-spec-20261115/bus/ddd-B.md`). Specs: `sherlock-e2e-zero-knowledge` (ZKE-001 to ZKE-007), `sherlock-e2e-stark-security` (SSG-001 to SSG-007); reused ZK-001 to ZK-019, SQG-001 to SQG-016, DG-001 to DG-011, FPR-001 to FPR-019, LPC-004, LPC-016. Lean package: `formal/sherlock-e2e/assurance/lean/` (namespace `Assurance`). Coverage rows: `coverage.tsv`. Findings: `FINDINGS.md`.

## Build and run (odin)

```
formal/sherlock-e2e/scripts/odin-build.sh assurance
ssh odin 'cd ~/sherlock/formal-e2e/assurance/lean && LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --step'
```

Replay command:

```
quint_replay.py --oracle ssh odin 'cd ~/sherlock/formal-e2e/assurance/lean && LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --step' \
  --traces <dir> --op-var lastOp \
  --compare rpDraws,deploySecret,saltDraws,salt,blinders,devices,submitted,transfers,usedSlots,gateBits,gateRecord,recordManifest,recordOpen,manifestD,qualified,lastRunQualified,changes,changeRows,verdicts,doneRows
```

## Protocol

- Request: `{"state": St, "op": N}`. Answer: `{"state": St}`, `{"refused": [CODE, ...]}` (state unchanged) or `{"error": TEXT}` (decode failure).
- `op = code * 16 + arg`, with `arg < 16`. The Quint model keeps the last operation in `lastOp` in this encoding.
- Integers are JSON integers >= 0 (or canonical decimal strings); booleans are JSON booleans; `blinders` and `devices` are JSON arrays of integers (newest first). The state object has exactly the keys below.
- Write the model so that an action fires only when its guard holds; a `refused` answer to a model step is a disagreement.
- Default mode (`kink-implementation/1`): `inputs` = every state field plus `ops`. Each op applies in order; a refused op leaves the state unchanged. Answer `final` with `cells`, or `could-not-apply` / `outside-capability` when the initial state is outside the domain.

## State fields

| Field | Type | Meaning (requirement) |
|---|---|---|
| `testSeedBuild` | bool | the binary has the cargo feature `test-seed` (ZK-014, ZKE-001, ZKE-002); never changes |
| `seedEnv` | bool | `SHERLOCK_TEST_SEED`, `GW_DET`, `GW_CHIP_SEED` or `GW_SEED` is set (ZK-014) |
| `constRpPath` | bool | the chain driver reaches a code path that sets `r_P` from a constant (ZKE-001) |
| `rpDraws` | nat | `r_P` draws of the deployment (ZK-012) |
| `deploySecret` | bool | `deploy.secret` present (ZKE-005) |
| `saltDraws`, `salt` | nat | salt draws; the current run's salt (draw number, 0 = no run) (ZK-013) |
| `blinderDraws` | nat | fresh blinder draws (ZK-003) |
| `blinders` | list nat | blinder of each wrap proof, newest first; a seeded blinder is 0 (ZKE-007, ZK-015) |
| `devices` | list nat | backend of each wrap proof, newest first: 0 CPU, 1 GPU (ZKE-007, ZK-017) |
| `submitted` | nat | proofs submitted to Preview (ZKE-002) |
| `roster` | nat (bit mask) | bit h = host h is owned by the user (ZK-001, ZK-018, ZKE-004) |
| `transfers` | nat | admitted transfers (ZKE-004) |
| `usedSlots` | nat (bit mask) | blinding slots used (ZK-016) |
| `gateBits` | nat, bits 0..5 | G1, G2, G3, G4, G5, FP-8 passed on the key set in use; all = 63 (SSG-006) |
| `unverified` | nat | UNVERIFIED RUNBOOK steps (SQG-016) |
| `confirmedNoFix` | nat | CONFIRMED counterexample rows without a fix commit (SSG-003) |
| `notAttempted` | nat | NOT ATTEMPTED rows (SSG-004) |
| `g5Current` | bool | the G5 mapping names the frozen sets of this key set (SSG-006 scenario) |
| `gateRecord` | bool | gate record written for the key set in use (SSG-006) |
| `recordManifest` | nat | key manifest in the record (SSG-006) |
| `recordOpen` | nat | rows listed open in the record (SSG-004) |
| `manifestD` | nat | key set in use |
| `qualified` | bool | key set status: false `prototype`, true `qualified` (SSG-006, UKS-007) |
| `runRecords` | nat | run records written (SSG-007) |
| `lastRunQualified` | bool | security field of the last run record (SSG-007) |
| `changes`, `changeRows`, `verdicts`, `doneRows` | nat | circuit changes, change-list rows, lead verdicts, rows counted DONE (SSG-005, LPC-004, LPC-016) |

State domain (`Valid`, else `outside-capability`): `rpDraws <= 1`; `deploySecret` implies `rpDraws = 1`; `salt <= saltDraws`; in a production build `blinders` has no duplicate and every element `<= blinderDraws`; `blinders` and `devices` have one length; `qualified` implies `gateRecord`; `gateRecord` implies `gateBits = 63` and `recordManifest = manifestD`; `lastRunQualified` implies `qualified`; `verdicts <= changes`; `changeRows = changes`.

## Operations

`seedGuard` = `seeded-without-feature` when `testSeedBuild = false` and (`seedEnv` or `constRpPath`).

| code | op | arg | guard (refusal codes, in this order) | effect |
|---|---|---|---|---|
| 0 | prepareDeployment | 0 | seedGuard (ZKE-001, ZK-014); `rpDraws = 0` (`already-deployed`, ZK-012) | `rpDraws += 1`, `deploySecret := true` |
| 1 | prepareRun | 0 | seedGuard; `rpDraws = 1` (`not-deployed`); `deploySecret` (`secret-missing`, ZKE-005) | `saltDraws += 1`, `salt := saltDraws + 1` (old value + 1) |
| 2 | loseSecret | 0 | none | `deploySecret := false` |
| 3 | wrapProve d | 0 CPU, 1 GPU | seedGuard; `salt != 0` (`no-run`) | if `testSeedBuild` and `seedEnv`: push 0 to `blinders`; else `blinderDraws += 1` and push the new value; push d to `devices` |
| 4 | submit | 0 | `blinders != []` (`no-proof`); not `testSeedBuild` (`test-seed-binary`, ZKE-002) | `submitted += 1` |
| 5 | transfer h | host | bit h of `roster` (`host-outside-roster`, ZKE-004, ZK-018) | `transfers += 1` |
| 6 | useSlot b | slot | bit b of `usedSlots` is 0 (`slot-reused`, ZK-016) | `usedSlots += 2^b` |
| 7 | gateCheck c | 0..5 | not `gateRecord` (`record-exists`) | set bit c of `gateBits` |
| 8 | writeRecord | 0 | not `gateRecord` (`record-exists`); `gateBits = 63` (`checks-incomplete`, SSG-006, SQG-015); `unverified = 0` (`step-unverified`, SQG-016); `confirmedNoFix = 0` (`confirmed-defect`, SSG-003); `g5Current` (`g5-old-params`) | `gateRecord := true`, `recordManifest := manifestD`, `recordOpen := notAttempted`, `qualified := true` |
| 9 | newKeys | 0 | none | `manifestD += 1`, `gateBits := 0`, `gateRecord := false`, `qualified := false`, `lastRunQualified := false` |
| 10 | writeRunRecord | 0 | none | `runRecords += 1`, `lastRunQualified := qualified` |
| 11 | landChange | 0 | none | `changes += 1`, `changeRows += 1` (SSG-005) |
| 12 | fileVerdict | 0 | `verdicts < changes` (`no-open-change`, LPC-004) | `verdicts += 1` |
| 13 | markDone | 0 | `verdicts = changes` (`no-lead-verdict`, LPC-016) | `doneRows += 1` |
| 14 | fixDefect | 0 | `confirmedNoFix > 0` (`nothing-open`) | `confirmedNoFix -= 1` |
| 15 | verifyStep | 0 | `unverified > 0` (`nothing-open`) | `unverified -= 1` |
| 16 | attempt | 0 | `notAttempted > 0` (`nothing-open`) | `notAttempted -= 1` |
| other | | | `outside-capability` | none |

## Suggested constants and fill

`--const testSeedBuild=false --const roster=7 --const g5Current=true`; fields the model leaves out: `--fill runRecords=0,doneRows=0,transfers=0`.

## Open interpretation points (ledger)

| ID | Point | Status |
|---|---|---|
| AS-1 | ZK-015, ZKE-007 "byte strings differ" is abstracted to "blinders differ"; the ledger acceptance of each proof is slice C (LVR). | held |
| AS-2 | ZK-003 allows a seeded (fixed) blinder only in a `test-seed` build under a seed variable; ZKE-007 then holds only for production builds. The oracle shows both (`pin_zk003_testseed`, `pin_zke007`). | held |
| AS-3 | SSG-004: NOT ATTEMPTED rows do not block the record; they are listed open. SSG-003 and SQG-016 block it. | held |
| AS-4 | The gate checks G1..G5 and FP-8 are bits; their content (SQG-006 to SQG-010, FPR-019) is SPEC-CHECKED. Gate checks are refused once a record exists for the key set (rerun needs `newKeys`). | held |
| AS-5 | `newKeys` sets the status to `prototype` (UKS-007) and clears the checks (SQG-015); the Key Management oracle carries the rekey itself. | held |
