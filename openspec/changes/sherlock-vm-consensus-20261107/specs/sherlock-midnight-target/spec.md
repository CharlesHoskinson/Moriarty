# Capability: sherlock-midnight-target (ledger 9.1.0.x pins, networks, fees, keys)

Status: CONSENSUS v2.1 (council vote 6/6 ACCEPT at 2026-11-07 21:29Z; text unchanged from DRAFT v2.1; amended 2026-11-08 (2) by the owner decisions in `dissent.md` "Resolved 2026-11-08" and "Resolved 2026-11-08 (2)"; amended 2026-11-10 by the design review C7: SMN-003 and SMN-006 name the ledger source constants and the `LedgerParameters` fields separately). Sources: https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-9.1.0.0 and `.../ledger-9.1.0.1`, https://docs.midnight.network/relnotes/ledger-9, https://docs.midnight.network/relnotes/support-matrix, `wiki/midnight-ledger-9-hardfork.md`. Design: `../../design.md` §5.

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
When the contract is compiled with Compact 0.36.0, the acceptRun template SHALL be recaptured and diffed; transcripts SHALL be canonical. (Amended 2026-11-10.) The decider statement SHALL be checked against the public-input cap 65,536 and the contract against the 256-operation cap, both source constants at ledger `a2b30128` (`MAX_CALL_PUBLIC_INPUTS`, `MAX_CONTRACT_OPERATIONS`); transaction-byte, block and cost limits SHALL be read from the target `LedgerParameters`; each verifier key SHALL be at most 50,000 raw bytes.

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
(Amended 2026-11-10.) If a deploy or call would exceed the public-input cap (65,536, a source constant), the 256-operation cap (a source constant), the 50,000-byte verifier-key limit, or a transaction-byte, block or cost limit of the target `LedgerParameters`, preflight SHALL refuse before submission.

#### Scenario: Oversize statement
- **WHEN** a synthetic statement exceeds 65,536 public inputs, or a verifier key exceeds 50,000 bytes. **THEN** refusal before any transaction.

### Requirement: SMN-007 Maintenance authority
`SherlockRun` SHALL be deployed with a maintenance authority on Preview and on Mainnet (decision 2026-11-08, made by the lead under the owner's delegation). The authority is a trust assumption: its holder can install a verifier key and so can change what the contract accepts. The owner SHALL hold the authority, and its public identity SHALL be published in `qa/sherlock/target-pins.json`, which holds no secrets. Every maintenance update SHALL be preceded by a passing A11 keygen regression and SHALL produce a committed receipt of the observed on-chain key state. midnight-node #2375 SHALL be checked before any key removal. Key replacement SHALL be rehearsed on Preview before mainnet, with the on-chain key state read back. A later change may freeze or renounce the authority once the keys are stable.

#### Scenario: Rehearsal
- **WHEN** deploy -> bad key -> maintenance replace -> readback runs on Preview. **THEN** the rehearsal receipt, including whether the old key is still accepted on chain, is committed before `t4.sh` can pass.

#### Scenario: Deploy without the declared authority
- **WHEN** a deploy names no maintenance authority, or names one that differs from the identity in `target-pins.json`. **THEN** the deploy script refuses before submission.

#### Scenario: Maintenance update without a fresh regression
- **WHEN** a maintenance update is prepared and A11 has not passed with its dependency closure unchanged. **THEN** the update does not run.

#### Scenario: Key removal
- **WHEN** a maintenance update removes a verifier key. **THEN** the status of midnight-node #2375 is checked and recorded first, and the receipt records the observed on-chain key state after the update.

#### Scenario: Maintained contract with active state
- **WHEN** a maintained `SherlockRun` holds consumed statements or funds and a key must be replaced. **THEN** it is not discarded; recovery records migration or residual exposure (SWF-010).
