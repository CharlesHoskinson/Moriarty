# Capability: sherlock-delivery (DoctorWatson factory workflow)

Status: CONSENSUS v2.1 (council vote 6/6 ACCEPT at 2026-11-07 21:29Z; text unchanged from DRAFT v2.1; amended 2026-11-08 (2) by the owner decisions in `dissent.md` "Resolved 2026-11-08" and "Resolved 2026-11-08 (2)"). Adopts fable-3's workflow text and the E1-E13 additions; refines AGENTS.md and `models/doctorwatson/factory/CONTRACT.md`. Design: `../../workflow.md`.

## ADDED Requirements

### Requirement: SWF-001 Single engine
The workflow SHALL run on `scripts/doctorwatson` with `predicates.yaml`, `status.json`, `runs.jsonl`, `budgets.json` and git as the only state; legacy `scripts/doctorwatson.sh` routes SHALL NOT be referenced.

#### Scenario: Validate
- **WHEN** `scripts/doctorwatson validate` runs on the amended `predicates.yaml`. **THEN** exit 0.

### Requirement: SWF-002 Traceability is checked
Every requirement ID SHALL map to at least one predicate whose `source:` anchor resolves; rows without a check SHALL use `unimplemented.sh`.

#### Scenario: Dangling anchor
- **WHEN** a row's `source:` names a heading that does not exist. **THEN** `check-traceability.py` (F14) exits 1.

### Requirement: SWF-003 Operations are not predicates
A mutating network step SHALL be an operation with id, preconditions, `heavy` command, postcondition, authorization and committed receipt; predicates SHALL observe only.

#### Scenario: Deploy on Preview
- **WHEN** `OP-PV-DEPLOY` runs. **THEN** it runs under `heavy`, its postcondition `A4-preview` becomes checkable, and its receipt is committed.

### Requirement: SWF-004 Heavy work under budget and hypothesis
When heavy work runs, it SHALL use `heavy` with a budget key; a same-command same-head retry after failure SHALL carry `--hypothesis`.

#### Scenario: Bare retry
- **WHEN** a failed command is re-run unchanged at the same head without `--hypothesis`. **THEN** exit 5.

### Requirement: SWF-005 Receipts committed
If a T3/T4 postcondition passes without a committed receipt under `qa/sherlock/receipt-traces/<op>/`, the pass SHALL NOT count.

#### Scenario: Receipt only in target/
- **WHEN** `receipt.json` exists only under `target/`. **THEN** the receipt-first reader fails.

### Requirement: SWF-006 Stop rule consumed
While `stop-rule` exits 6, the owner action SHALL be recorded; resetting the flip counter by unrelated passes SHALL NOT be reported as progress.

#### Scenario: 295 h STOP
- **WHEN** M0 re-earns F4/F11. **THEN** the record labels it bookkeeping and the STOP trace stays.

### Requirement: SWF-007 Target pins
When pins change or a T3 runs, F15 SHALL compare `target-pins.json` with `Cargo.toml` and the SDK lockfile, and the T3 live step SHALL compare the pinned RPC's identity with the pins row.

#### Scenario: Stagenet RPC against Preview row
- **WHEN** the live step reads specVersion 2000000 for a `preview` row. **THEN** T3 fails.

### Requirement: SWF-008 Network field and T4
Rows MAY carry `network:`; tiers SHALL be T0-T4; a T4 row SHALL depend on a T3 row with the same key lineage.

#### Scenario: T4 without T3
- **WHEN** a T4 row has no T3 dependency. **THEN** `validate` refuses.

### Requirement: SWF-009 Mainnet blocked until authorized
The 2026-11-08 amendment (`MORIARTY-COMPLETION-PROGRAM.md:270-283`) supersedes the `:269` mainnet ban. While any of its conditions is unmet (local Docker at 9.1.0.1 passed, Preview qualification receipts committed, Preview rollback rehearsal receipt committed, pre-deploy checklist passed), no mainnet operation SHALL run. Each mainnet operation SHALL be a T4 operation with owner+checklist authorization, SHALL have explicit owner authorization at the time it runs, and SHALL stay within the owner-set per-operation spend ceiling with no automatic retry. The amendment authorizes no specific spend.

#### Scenario: Checklist without rehearsal
- **WHEN** `t4.sh` runs without the rehearsal receipt. **THEN** it refuses.

#### Scenario: Conditions met, no authorization for this operation
- **WHEN** every amendment condition holds but the owner has not authorized this specific operation. **THEN** the operation does not run; an earlier authorization does not carry over.

#### Scenario: Failed mainnet submission
- **WHEN** a mainnet submission fails. **THEN** the operation stops; a retry needs a new owner authorization.

### Requirement: SWF-010 Rollback
When a deployment or key fails its postcondition, rollback SHALL record orphaned/retired, re-pin, use maintenance update only on maintained contracts, redeploy otherwise, and `git revert` the fixture commit without deleting history; when the contract holds active state, recovery SHALL migrate or record funds, duties and consumed statements, only empty instances may be abandoned, and a client re-pin SHALL NOT be recorded as on-chain revocation.

#### Scenario: Unsafe key on a maintained contract
- **WHEN** a key is found unsafe and the declared recovery mode is maintenance replacement.
- **THEN** the replacement is performed, the on-chain key state is read back, the receipt records whether the old key is still accepted (insertion does not revoke) and the residual exposure, and the old address is marked retired only after that observation.

#### Scenario: Unsafe key with funds
- **WHEN** an unsafe key is found on a contract holding funds. **THEN** the receipt records the observed on-chain key state, the funds' disposition and the residual exposure; the instance is not abandoned.

### Requirement: SWF-011 Retired rows
Superseded rows SHALL carry `retired:`; `next` and `check --tier`/`--set` sweeps SHALL skip them, `next` SHALL report a `retired` count, explicit `check <ID>` SHALL still run them, `validate` SHALL refuse an active row depending on a retired row, and composites SHALL be rewired.

#### Scenario: A8 retired, A8p unimplemented
- **WHEN** `next`, `check --tier T3`, `check --set A` and `check A8` run.
- **THEN** FIRST FAILING is `A8p`, `RETIRED: A8 (...)` is listed, the sweeps ignore A8, `check A8` runs and its history is retained.

### Requirement: SWF-012 Optional rows do not gate
Rows marked `optional: true` SHALL be skipped by `next` FIRST FAILING and by `check --tier`/`--set` sweeps unless `--include-optional`; explicit `check <ID>` SHALL run them; `validate` SHALL refuse a mandatory row depending on an optional row; the mandatory milestone row set and its dependencies SHALL be what the tier runner selects. Since 2026-11-08 no B row is optional (SWF-013); the optional rows are the conditionally guarded features: the GPU rows (F23, A24-A27, A21) and the tree row (A22) until gate G-A or their kill criteria keep them, and the S-slot row A20 until G-A fixes S (amended 2026-11-08 (3); the S>1 experiment is retired, S is chosen once at G-A, SRC-014).

#### Scenario: Unimplemented optional row
- **WHEN** a GPU or tree row is optional and `unimplemented.sh`, and every mandatory Preview row passes.
- **THEN** `next` names no failing mandatory row, `qa/tiers/t3.sh` passes, and `check --include-optional` reports the optional row failing.

#### Scenario: Unimplemented mandatory row
- **WHEN** A8p is `unimplemented.sh`. **THEN** `next` names A8p and `t3.sh` fails.

### Requirement: SWF-013 Synthetic rollup: local, then Preview, then Mainnet
The B rows SHALL be Sherlock's synthetic-rollup qualification on FSYN-generated data (synthetic rollup change SSRD-029), run locally first, then on Preview, then on Mainnet (owner decisions 2026-11-08 and 2026-11-08 (2)): B1, B2, B3, B6 and B7 locally; B4p, B5p, B8p and B9p on Preview (OP-PV-SYNTH-ROLLUP); then B4m, B5m, B8m and B9m (OP-MN-SYNTH-ROLLUP), each mainnet row depending on its Preview counterpart. No B row SHALL target stagenet; B4, B5, B8 and B9 stay retired. No row in the Sherlock U023/C4 dependency closure SHALL depend on a B row (owner decision 2026-11-08 (2): "keep them out of C4"). The synthetic-rollup Preview and Mainnet gates SHALL depend on their B rows. A mainnet B operation SHALL meet SWF-009.

#### Scenario: C4 does not wait on the synthetic rollup
- **WHEN** B4p is `unimplemented.sh` and every row of the C4 closure passes.
- **THEN** the Sherlock M6 gate, evaluated on the C4 closure (E15), is eligible, and the synthetic-rollup Preview gate is not.

#### Scenario: Preview before local
- **WHEN** B4p is checked while B3 is not passing. **THEN** B4p is not eligible, because it depends on B3.

#### Scenario: Mainnet before Preview
- **WHEN** B4m is checked while B4p is not passing. **THEN** B4m is not eligible, because it depends on B4p.

#### Scenario: Stagenet row
- **WHEN** an active B row is made to depend on retired B4. **THEN** `validate` refuses.

### Requirement: SWF-014 No Fenrir dependency
If any tracked code, test, script or predicate row reads a path that contains `projectfenrir` or `rollup-subarch`, the T0 guard F17 SHALL fail and name the file and line. Moriarty and Sherlock SHALL carry no Fenrir-owned gate, row, operation, file, repository or live-system dependency (owner decision 2026-11-08 (2)); the synthetic rollup data is generated in this repository (SSRD-030).

#### Scenario: Test reads a Fenrir checkout
- **WHEN** a tracked test loads `../../../projectfenrir-main/recursion/rollup-test-format/v3`. **THEN** F17 fails and names the file and line.

#### Scenario: Clean host
- **WHEN** the B1, B2 and B3 checks run on a host with no Fenrir checkout and no `~/research` directory, after task 1.9. **THEN** they run on in-repository FSYN fixtures.
