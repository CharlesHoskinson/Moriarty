# Capability: sherlock-delivery (DoctorWatson factory workflow)

Status: DRAFT v1, specified-only. Adopts fable-3's workflow text and the E1-E10 additions; refines AGENTS.md and `models/doctorwatson/factory/CONTRACT.md`. Design: `../../workflow.md`.

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
While `MORIARTY-COMPLETION-PROGRAM.md:269` forbids mainnet submission, no mainnet operation SHALL run; it SHALL require owner+checklist authorization and a Preview rollback rehearsal receipt.

#### Scenario: Checklist without rehearsal
- **WHEN** `t4.sh` runs without the rehearsal receipt. **THEN** it refuses.

### Requirement: SWF-010 Rollback
When a deployment or key fails its postcondition, rollback SHALL record orphaned/retired, re-pin, use maintenance update only on maintained contracts, redeploy otherwise, and `git revert` the fixture commit without deleting history.

#### Scenario: Unsafe key on a maintained contract
- **WHEN** a key is found unsafe. **THEN** a new key is inserted, the old address is retired (insertion does not revoke), and the receipt records both.

### Requirement: SWF-011 Retired rows
Superseded rows SHALL carry `retired:`; `next` SHALL skip them for FIRST FAILING, report a `retired` count and keep their results and `check` runnable.

#### Scenario: A8 retired, A8-preview unimplemented
- **WHEN** `next` runs. **THEN** FIRST FAILING is `A8-preview` and `RETIRED: A8 (...)` is listed.
