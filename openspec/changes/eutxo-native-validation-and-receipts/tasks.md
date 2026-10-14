# Implementation tasks

All tasks are prospective and unchecked. These commands are specified only.
Set `NATIVE_RUN_ROOT` to the fresh absolute run directory from the reviewed manifest.
Reject an unset or foreign run directory before any command.

## 1. candidate

- [ ] NAT-T01a Write adversarial tests at `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/candidate.test.mjs`.
- [ ] NAT-T01b Implement `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/candidate.mjs` for EU-NAT-001.
- [ ] NAT-T01c Run `node --test /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/candidate.test.mjs`.
- [ ] NAT-T01d Preserve stdout, stderr, exit code, and hashes at `$NATIVE_RUN_ROOT/checks/candidate/`.

## 2. resources

- [ ] NAT-T02a Write adversarial tests at `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/resources.test.mjs`.
- [ ] NAT-T02b Implement `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/resources.mjs` for EU-NAT-002, EU-NAT-003, EU-NAT-011.
- [ ] NAT-T02c Run `node --test /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/resources.test.mjs`.
- [ ] NAT-T02d Preserve stdout, stderr, exit code, and hashes at `$NATIVE_RUN_ROOT/checks/resources/`.

## 3. pins

- [ ] NAT-T03a Write adversarial tests at `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/pins.test.mjs`.
- [ ] NAT-T03b Implement `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/pins.mjs` for EU-NAT-004, EU-NAT-005.
- [ ] NAT-T03c Run `node --test /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/pins.test.mjs`.
- [ ] NAT-T03d Preserve stdout, stderr, exit code, and hashes at `$NATIVE_RUN_ROOT/checks/pins/`.

## 4. time

- [ ] NAT-T04a Write adversarial tests at `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/time.test.mjs`.
- [ ] NAT-T04b Implement `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/time.mjs` for EU-NAT-006, EU-NAT-007, EU-NAT-008.
- [ ] NAT-T04c Run `node --test /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/time.test.mjs`.
- [ ] NAT-T04d Preserve stdout, stderr, exit code, and hashes at `$NATIVE_RUN_ROOT/checks/time/`.

## 5. oracles

- [ ] NAT-T05a Write adversarial tests at `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/oracles.test.mjs`.
- [ ] NAT-T05b Implement `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/oracles.mjs` for EU-NAT-009.
- [ ] NAT-T05c Run `node --test /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/oracles.test.mjs`.
- [ ] NAT-T05d Preserve stdout, stderr, exit code, and hashes at `$NATIVE_RUN_ROOT/checks/oracles/`.

## 6. body

- [ ] NAT-T06a Write adversarial tests at `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/body.test.mjs`.
- [ ] NAT-T06b Implement `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/body.mjs` for EU-NAT-010.
- [ ] NAT-T06c Run `node --test /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/body.test.mjs`.
- [ ] NAT-T06d Preserve stdout, stderr, exit code, and hashes at `$NATIVE_RUN_ROOT/checks/body/`.

## 7. receipts

- [ ] NAT-T07a Write adversarial tests at `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/receipts.test.mjs`.
- [ ] NAT-T07b Implement `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/receipts.mjs` for EU-NAT-012, EU-NAT-013, EU-NAT-014, EU-NAT-015.
- [ ] NAT-T07c Run `node --test /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/receipts.test.mjs`.
- [ ] NAT-T07d Preserve stdout, stderr, exit code, and hashes at `$NATIVE_RUN_ROOT/checks/receipts/`.

## 8. controls

- [ ] NAT-T08a Write adversarial tests at `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/controls.test.mjs`.
- [ ] NAT-T08b Implement `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/controls.mjs` for EU-NAT-016, EU-NAT-017.
- [ ] NAT-T08c Run `node --test /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/controls.test.mjs`.
- [ ] NAT-T08d Preserve stdout, stderr, exit code, and hashes at `$NATIVE_RUN_ROOT/checks/controls/`.

## 9. readback

- [ ] NAT-T09a Write adversarial tests at `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/readback.test.mjs`.
- [ ] NAT-T09b Implement `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/readback.mjs` for EU-NAT-018.
- [ ] NAT-T09c Run `node --test /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/readback.test.mjs`.
- [ ] NAT-T09d Preserve stdout, stderr, exit code, and hashes at `$NATIVE_RUN_ROOT/checks/readback/`.

## 10. families

- [ ] NAT-T10a Write adversarial tests at `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/families.test.mjs`.
- [ ] NAT-T10b Implement `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/families.mjs` for EU-NAT-019, EU-NAT-020, EU-NAT-021, EU-NAT-022, EU-NAT-023, EU-NAT-024.
- [ ] NAT-T10c Run `node --test /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/families.test.mjs`.
- [ ] NAT-T10d Preserve stdout, stderr, exit code, and hashes at `$NATIVE_RUN_ROOT/checks/families/`.

## 11. race

- [ ] NAT-T11a Write adversarial tests at `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/race.test.mjs`.
- [ ] NAT-T11b Implement `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/race.mjs` for EU-NAT-025.
- [ ] NAT-T11c Run `node --test /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/race.test.mjs`.
- [ ] NAT-T11d Preserve stdout, stderr, exit code, and hashes at `$NATIVE_RUN_ROOT/checks/race/`.

## 12. budgets

- [ ] NAT-T12a Write adversarial tests at `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/budgets.test.mjs`.
- [ ] NAT-T12b Implement `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/budgets.mjs` for EU-NAT-026.
- [ ] NAT-T12c Run `node --test /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/budgets.test.mjs`.
- [ ] NAT-T12d Preserve stdout, stderr, exit code, and hashes at `$NATIVE_RUN_ROOT/checks/budgets/`.

## 13. repro

- [ ] NAT-T13a Write adversarial tests at `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/repro.test.mjs`.
- [ ] NAT-T13b Implement `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/repro.mjs` for EU-NAT-027.
- [ ] NAT-T13c Run `node --test /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/repro.test.mjs`.
- [ ] NAT-T13d Preserve stdout, stderr, exit code, and hashes at `$NATIVE_RUN_ROOT/checks/repro/`.

## 14. matrix

- [ ] NAT-T14a Write adversarial tests at `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/matrix.test.mjs`.
- [ ] NAT-T14b Implement `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/matrix.mjs` for EU-NAT-028.
- [ ] NAT-T14c Run `node --test /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/tests/matrix.test.mjs`.
- [ ] NAT-T14d Preserve stdout, stderr, exit code, and hashes at `$NATIVE_RUN_ROOT/checks/matrix/`.

## Final execution and review

- [ ] NAT-T15 Freeze scripts, tests, source bindings, dependency interfaces, and current tool pins in `$NATIVE_RUN_ROOT/CANDIDATE-SHA256.json`.
- [ ] NAT-T16 Obtain required current native implementation reviews on that complete candidate.
- [ ] NAT-T17 Record bounded authority and exclusive ownership in `$NATIVE_RUN_ROOT/resource-plan.json`.
- [ ] NAT-T18 Run `node /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/run.mjs --manifest $NATIVE_RUN_ROOT/run-manifest.json --mode local-native`.
- [ ] NAT-T19 Run `node /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/check.mjs --run R --inventory /home/charl/moriarty-eutxo-validation-2026-10-12/PATTERN-INVENTORY.json`.
- [ ] NAT-T20 Preserve every public transaction notification in `$NATIVE_RUN_ROOT/host-notifications.jsonl`.
- [ ] NAT-T21 Write `$NATIVE_RUN_ROOT/RESULT-MATRIX.json` and `$NATIVE_RUN_ROOT/REPRODUCTION.json`.
- [ ] NAT-T22 Obtain independent result review on all failures, refusals, measurements, and complete readbacks.
- [ ] NAT-T23 Stop only matching task-owned process groups through `node /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/native-validation/resources.mjs --cleanup $NATIVE_RUN_ROOT/resource-plan.json`.

Expected exit controls: 0 means the declared predicate passed, 2 means a refused prerequisite, and 3 means comparison failure.
A process exit never substitutes for a native acceptance observation.
Inspect output completeness after every command.
