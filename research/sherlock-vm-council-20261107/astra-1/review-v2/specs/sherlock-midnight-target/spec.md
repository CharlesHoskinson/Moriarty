# Capability: sherlock-midnight-target (ledger 9.1.0.x pins, networks, fees, keys)

Status: DRAFT v2 (revision round 1, 2026-11-07 21:30Z). Sources: https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-9.1.0.0 and `.../ledger-9.1.0.1`, https://docs.midnight.network/relnotes/ledger-9, https://docs.midnight.network/relnotes/support-matrix, `wiki/midnight-ledger-9-hardfork.md`. Design: `../../design.md` §5.

## ADDED Requirements

### Requirement: SMN-001 Pins
The workspace SHALL pin the ledger 9.1.0.1 line and the matching crate and SDK set, recorded in `qa/sherlock/target-pins.json`.

#### Scenario: Build at new pins
- **WHEN** `cargo build --locked -p mori-backend -p mori-recursion -p mori-ledger` runs. **THEN** it succeeds and F15 passes.

### Requirement: SMN-002 Historical keys preserved
When pins change, SRC-009 SHALL run and historical keys and receipts SHALL remain historical evidence.

#### Scenario: Stagenet A-chain record
- **WHEN** the regression runs. **THEN** `stagenet-verifier.json` is kept and labelled historical, not deleted or marked false.

### Requirement: SMN-003 Template and limits
When the contract is compiled with Compact 0.36.0, the acceptRun template SHALL be recaptured and diffed; transcripts SHALL be canonical; the decider statement and the contract SHALL be checked against the public-input limit and the 256-operation cap.

#### Scenario: Local acceptance with mainnet parameters
- **WHEN** `validate_call` runs on a proof-verifying local 9.1.0.1 ledger with mainnet `LedgerParameters`. **THEN** the positive call passes and the five mutations are refused.

### Requirement: SMN-004 Chain identity
Receipts and pins rows SHALL include network name, genesis hash, `specVersion` and ledger revision; local provenance and live identity SHALL be recorded separately with the deployed-source gap stated.

#### Scenario: Pre-reset Preview receipt
- **WHEN** a receipt names network `preview` with the pre-2026-11-07 genesis. **THEN** it is rejected for the current Preview.

### Requirement: SMN-005 Fees and DUST
When fees are estimated or recorded, tooling SHALL use 9.1.0.x parameters, pay with at least two DUST spends while #838 is open, and register DUST with lead time.

#### Scenario: Single-spend call
- **WHEN** a call is funded by exactly one DUST spend. **THEN** preflight warns and the operation uses two.

### Requirement: SMN-006 Limit refusal
If a deploy or call would exceed the public-input limit, the 256-operation cap or the block gas limit, preflight SHALL refuse before submission.

#### Scenario: Oversize statement
- **WHEN** a synthetic statement exceeds the limit read from `LedgerParameters`. **THEN** refusal before any transaction.

### Requirement: SMN-007 Maintenance authority
If `SherlockRun` is deployed without a maintenance authority, the receipt SHALL mark it immutable and state its recovery mode (disposable only while it holds no active state; otherwise redeploy-and-retire under SWF-010's active-state rule); a maintained contract's key replacement SHALL be rehearsed on Preview before mainnet with the on-chain key state read back. Immutable deployments remain permitted.

#### Scenario: Rehearsal
- **WHEN** deploy -> bad key -> maintenance replace -> readback runs on Preview. **THEN** the rehearsal receipt, including whether the old key is still accepted on chain, is committed before `t4.sh` can pass.

#### Scenario: Immutable contract with active state
- **WHEN** an immutable `SherlockRun` holds consumed statements or funds. **THEN** it is not marked disposable; recovery records migration or residual exposure.
