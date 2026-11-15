## ADDED Requirements

### Requirement: PVW-001 The Preview genesis is pinned
PVW-001: If the genesis hash that the RPC node reports differs from the pinned Preview genesis `0x3c096de2...3796`, then the settle runner SHALL stop before any wallet step.

Verification: MISSING a negative test; the assert ran on every T17 (a) step; to build.

#### Scenario: Another chain behind the Preview URL
- **GIVEN** `T10_NET=preview` and a node of another chain
- **WHEN** the runner reads block 0
- **THEN** the runner stops with "the RPC genesis is not the pinned Preview genesis"

#### Scenario: Status of PVW-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: MID-006. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:91-95,225-227`. Check: MISSING a negative test; the assert ran on every T17 (a) step.

### Requirement: PVW-002 No Mainnet transaction
PVW-002: The settle runner SHALL send a transaction to 2 chains only: the local devnet `undeployed1` and the pinned Preview genesis.

Verification: MISSING a negative test; to build.

#### Scenario: A Mainnet endpoint is refused
- **GIVEN** an RPC URL of Mainnet
- **WHEN** the runner starts in either mode
- **THEN** the chain check fails and nothing is sent

#### Scenario: Status of PVW-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: MID-006 (Mainnet follows SWF-009 of `sherlock-vm-consensus-20261107`); loop rule "still owner-gated: Mainnet" (LOOP.md:95). Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:4,91-93,227-229`; `research/sherlock-t17-preview-20261114/RUNBOOK.md:3`. Check: MISSING a negative test.

### Requirement: PVW-003 Fees from the chain parameters
PVW-003: When the settle runner balances a Preview transaction, the settle runner SHALL price the fee with the ledger parameters read from the Preview chain.

Verification: T17 (a) deploy (fee 1,896,654,628,571,429 specks, `RESULTS.md:16`) and call (`RESULTS.md:19`); exists.

#### Scenario: Initial parameters underprice Preview
- **GIVEN** Preview at height 113,894 with write factor 2.2857
- **WHEN** the wallet prices a deploy
- **THEN** the fee uses the chain values, not `LedgerParameters.initialParameters()`

#### Scenario: Status of PVW-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:35`, `RUNBOOK.md:62`. Check: T17 (a) deploy (fee 1,896,654,628,571,429 specks, `RESULTS.md:16`) and call (`RESULTS.md:19`).

### Requirement: PVW-004 Own proof server
PVW-004: When the Preview runner starts, the Preview runner SHALL start its own `proof-server:9.0.1` container on 127.0.0.1 port 6341.

Verification: `t17.sh check` in T17 (a); exists.

#### Scenario: The devnet port is left alone
- **GIVEN** the e6 devnet project on port 6340
- **WHEN** `t17.sh check` runs
- **THEN** the container `t17-proof-server` answers on 6341 and port 6340 is untouched

#### Scenario: Status of PVW-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RUNBOOK.md:15,26,63`. Check: `t17.sh check` in T17 (a).

### Requirement: PVW-005 One command for each step
PVW-005: The Preview runner SHALL expose the 7 steps `check`, `balance-before`, `deploy`, `call`, `refusals`, `balance-after` and `receipts` as one command each.

Verification: T17 (a) verdict record `A, scan, smallScan, R1, R2, R3, R4: true` (`RESULTS.md:9`); exists.

#### Scenario: The runbook reproduces the run
- **GIVEN** the inputs listed in the runbook
- **WHEN** `./t17.sh all` runs in a user unit
- **THEN** the seven steps run in order and `receipts.json` is written

#### Scenario: Status of PVW-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. References: LPC-008 (parts (b) and (c) of row T17). Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/t17.sh:4,151-153`, `RUNBOOK.md:24-40`. Check: T17 (a) verdict record `A, scan, smallScan, R1, R2, R3, R4: true` (`RESULTS.md:9`).

### Requirement: PVW-006 Tamper controls on Preview
PVW-006: When the readback of the included honest call arrives, the Preview runner SHALL submit the 4 controls R1 (changed public input), R2 (flipped proof bit), R4 (other program) and R3 (replay).

Verification: T17 (a); rerun in LPC-008 on the T25 keys; exists.

#### Scenario: The node refuses each control
- **GIVEN** the included honest call A
- **WHEN** the four controls are submitted
- **THEN** each gets `1010: Invalid Transaction: Custom error: 115` and none is included

#### Scenario: Status of PVW-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:23-26`, `RUNBOOK.md:48-56`; `tools/sherlock-sdk/t10a-ledger-run.mjs:795-818`. Check: T17 (a); rerun in LPC-008 on the T25 keys.

### Requirement: PVW-007 A failed submit without a node verdict fails the run
PVW-007: If a submitted control ends with no inclusion, no inclusion timeout and no JSON-RPC error from the node, then the Preview runner SHALL record `no-node-verdict` and fail the verdict.

Verification: T17 (a) first run (the 18:40 network loss, `RESULTS.md:42`). MISSING an automated fault test; exists in part.

#### Scenario: A lost network is not a refusal
- **GIVEN** a control whose submit fails on a closed socket
- **WHEN** the verdict is computed
- **THEN** the control reads `no-node-verdict` and the run fails

#### Scenario: Status of PVW-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: The four results are exclusive, in this order: `included`, `not-included-timeout` (PVW-008), `refused-by-node`, `no-node-verdict`. Decision D-081. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:556-573`; `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:43`. Check: T17 (a) first run (the 18:40 network loss, `RESULTS.md:42`). MISSING an automated fault test.

### Requirement: PVW-008 An inclusion timeout is not a refusal
PVW-008: If a submitted call is not seen in a block within 600 s on Preview, then the Preview runner SHALL record `not-included-timeout` for the call.

Verification: MISSING an automated test; to build.

#### Scenario: Slow inclusion
- **GIVEN** `T10A_INBLOCK_S` = 600
- **WHEN** no InBlock event arrives in 600 s
- **THEN** the verdict is `not-included-timeout`, which no refusal check accepts

#### Scenario: Status of PVW-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:196,572`; `research/sherlock-t17-preview-20261114/t17.sh:19`. Check: MISSING an automated test.

### Requirement: PVW-009 Final prototype deploy after the gate
PVW-009: When the T12 gate passes on the T25 key set, the deploy runner SHALL deploy the prototype SherlockLedger contracts on Preview with the existing Preview wallet.

Verification: Preview receipts of T17 (c) and T23; OPEN; exists in part.

#### Scenario: Prototype keys are not redeployed as final
- **GIVEN** the T10m prototype keys
- **WHEN** the final prototype deploy is asked for
- **THEN** the runner waits for the T12 pass on the T25 keys

#### Scenario: Status of PVW-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: STL-003, LPC-008. Evidence: lead decision "Preview redeploy: after T12 on the frozen keys" (LOOP.md:88); owner 2026-11-14 allows Preview deploys and calls with the existing wallet (LOOP.md:95). Check: Preview receipts of T17 (c) and T23; OPEN.

### Requirement: PVW-010 The final call proves on a GPU box of the user
PVW-010: When the final prototype call is proved, the Preview runner SHALL run the `L_Fr` prove and the wrap prove on a GPU box that the user owns.

Verification: MISSING (the hook is not built). GAP owner: row T17 (c); to build.

#### Scenario: The prover hook is missing today
- **GIVEN** the runner of T17 (a), which proves on muninn
- **WHEN** T17 (c) starts
- **THEN** the run needs a `T10A_PROVER` hook for huginn or odin

#### Scenario: Status of PVW-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: LPC-008; HLP-001 (muninn checks the box wrap). Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:49`. Check: MISSING (the hook is not built). GAP owner: row T17 (c).

### Requirement: PVW-011 The validium contract checks program and roots
PVW-011: If a Sherlock claim offered to the validium contract has another program commitment or an input root that differs from the stored root, then the validium contract SHALL refuse the batch.

Verification: MISSING (branch `t19-validium` not started). GAP owner: row T19; to build.

#### Scenario: A stale root
- **GIVEN** a stored root r1 and a batch proved from an older root r0
- **WHEN** the batch is submitted on Preview
- **THEN** the validium contract refuses the batch and the stored root stays r1

#### Scenario: Status of PVW-011
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: LPC-009 (completion) keeps the whole T19 test: at least 2 chained batches accepted, the stored root set to the new root, an invalid batch and a stale root refused with the state unchanged, and equal proof size across batch counts. This requirement adds only the two ledger refusals. Dependencies: slice A opens the hidden input and outcome of the rollup statement; slice B proves the GP-2 chain and the GP-4 fold. SLC-001 read entry `runAccepted`. Evidence: row T19 (LOOP.md:41), status "waits T17 (b)"; no contract source exists. Check: MISSING (branch `t19-validium` not started). GAP owner: row T19.

### Requirement: PVW-012 An included control fails the verdict
PVW-012: If a tamper control is included in a block, then the Preview runner SHALL record `included` for the control and fail the verdict.

Verification: MISSING a test (no control was ever included); to build.

#### Scenario: A verifier that accepts a flipped bit
- **GIVEN** control R2 included at some block
- **WHEN** the verdict is computed
- **THEN** R2 reads `included`, the verdict fails and the exit status is 4

#### Scenario: Status of PVW-012
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: SCL-002; PVW-006. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:59,556-560,572`. Check: MISSING a test (no control was ever included).

### Requirement: PVW-013 The final deploy needs the gate record of its key
PVW-013: If the gate record of the wrap key to deploy lacks a pass for RKY-003, RKY-004, RKY-005, the T12 gate, the zero-knowledge checks ZK-012 to ZK-017 or the 5 wrap checks of WRP-012, then the deploy runner SHALL refuse the final prototype deploy.

Verification: MISSING (no gate record format, no check in the deploy runner); to build.

#### Scenario: A key with no D3 result
- **GIVEN** a T25 wrap key whose record has rows and VK bytes and no 100-bit result
- **WHEN** the final deploy is asked for
- **THEN** the runner refuses and names RKY-005

#### Scenario: Status of PVW-013
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: RKY-003 to RKY-005, SQG-014 to SQG-016, ZK-012 to ZK-017 (completion); WRP-012; PVW-009. One gate record gates the deploy: it holds the WRP-012 qualification passes too (FAL-WRAP-1, LEAD PICK for T15). Slice B produces the keys and the evidence; slice C checks the record for the exact key (sha256 of `wrap.vk`). Evidence: LOOP.md:47 (T25 done-when: every gate reruns PASS on the new keys); `~/.cache/lead/CHECKLIST-20261115.md:12-14`. Check: MISSING (no gate record format, no check in the deploy runner).
