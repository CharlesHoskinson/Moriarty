## ADDED Requirements

### Requirement: OBS-001 Receipt of an included call
OBS-001: When a deploy or a call is included, the receipts writer SHALL record the contract address, the transaction hash, the block height, the status, the transaction bytes and the paid fee.

Verification: `receipts.json` of T17 (a); exists.

#### Scenario: The honest T17 call
- **GIVEN** the included call A
- **WHEN** receipts are written
- **THEN** the record shows tx `0x196fd6eb...3d57`, block 115,187, SUCCESS, 6,780 bytes and the fee

#### Scenario: Status of OBS-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. References: STL-004. This record is the published type `SettlementReceipt` that slice A consumes. Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:15-19`, `t17.sh:117-125`. Check: `receipts.json` of T17 (a).

### Requirement: OBS-002 Receipt of a refused call
OBS-002: When the node refuses a call, the receipts writer SHALL record the SHA-256 of the submitted bytes and the JSON-RPC error of the node.

Verification: `receipts.json` of T17 (a); exists.

#### Scenario: Control R1
- **GIVEN** the refused control R1
- **WHEN** receipts are written
- **THEN** the record holds tx sha256 `15aa3ba4...` and `1010: Invalid Transaction: Custom error: 115`

#### Scenario: Status of OBS-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. References: PVW-007. Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:23,29`. Check: `receipts.json` of T17 (a).

### Requirement: OBS-003 Privacy scan of every settled call
OBS-003: When the ledger runner builds or reads back a call, the ledger runner SHALL scan the call vector, the transcript and the ledger state for the forbidden private words.

Verification: T17 (a) verdict fields `scan` and `smallScan`; rerun for each T23 call (to build in the T23 driver); exists in part.

#### Scenario: No private word reaches the chain
- **GIVEN** the forbidden words `prog_d`, `pack(prog_d)`, `pack(r_P)`, the `R` operands, `n_steps`, the `h_hi` limbs and the `r_P` words
- **WHEN** the scan runs on call A
- **THEN** no surface holds any of them

#### Scenario: Status of OBS-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: ZK-002, ZK-007, LDG-002 (no plain `prog_d` or `n_steps` as call data). Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:178,397-403,434,461,750-751`; `research/sherlock-t17-preview-20261114/RESULTS.md:22`. Check: T17 (a) verdict fields `scan` and `smallScan`; rerun for each T23 call (to build in the T23 driver).

### Requirement: OBS-004 The scanner proves that it fires
OBS-004: If a control word is absent from a scanned surface, then the ledger runner SHALL fail the privacy scan.

Verification: T17 (a) (`scannerFires`); MISSING a test with a broken walker; exists in part.

#### Scenario: A dead walker on one surface
- **GIVEN** a scanner that no longer walks the indexed state
- **WHEN** the scan runs
- **THEN** the control words `claim_pub`, `outcome_pub` and anchor are missing on that surface and the scan fails

#### Scenario: Status of OBS-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:50,397-403`. Check: T17 (a) (`scannerFires`); MISSING a test with a broken walker.

### Requirement: OBS-005 Step log
OBS-005: When a runner step starts or ends, the Preview runner SHALL append 1 line with a timestamp and the step name to `runs.log`.

Verification: `~/.cache/t17/logs/runs.log` of T17 (a); exists.

#### Scenario: A skipped step is visible
- **GIVEN** a finished deploy
- **WHEN** `t17.sh all` runs again
- **THEN** `runs.log` holds "skip deploy (ok)"

#### Scenario: Status of OBS-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/t17.sh:25-26`, `RUNBOOK.md:42`. Check: `~/.cache/t17/logs/runs.log` of T17 (a).

### Requirement: OBS-006 Input manifest
OBS-006: When the `check` step runs, the Preview runner SHALL write the SHA-256 of each copied input file, of the `L_Fr` key and of each binary to `manifest.sha256`.

Verification: `receipts.json` of T17 (a); exists.

#### Scenario: The receipts name the inputs
- **GIVEN** a finished run
- **WHEN** `receipts.json` is written
- **THEN** the file carries the manifest

#### Scenario: Status of OBS-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RUNBOOK.md:18`. Check: `receipts.json` of T17 (a).

### Requirement: OBS-007 Times for each call and box
OBS-007: When a suite call is proved, the T23 suite driver SHALL record the box name and the seconds of the 5 stages leaf, node, `L`, `L_Fr` and wrap.

Verification: the T23 suite table; OPEN (no prove run yet); exists in part.

#### Scenario: A speed claim names one box
- **GIVEN** results of one call on huginn and on odin
- **WHEN** the report compares times
- **THEN** each comparison uses runs of one box only

#### Scenario: Status of OBS-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: GPW-007 (a MEASURED time needs its run record and host load); STL-002; LPC-005. Evidence: `t23-suite@16836929b:tools/t23-suite/t23.py:283`; loop rule "each box keeps its own baseline" (LOOP.md:96). Check: the T23 suite table; OPEN (no prove run yet).

### Requirement: OBS-008 Wallet failures keep their cause
OBS-008: If wallet balancing fails, then the ledger runner SHALL record the wallet stage and the whole error chain in `calls.<tag>.walletAttempts`.

Verification: T17 (a) first run record; exists.

#### Scenario: The indexer gives no HTTP response
- **GIVEN** a network loss during balancing
- **WHEN** the runner records the failure
- **THEN** the record holds the stage `balanceUnboundTransaction` and the chain down to the transport error

#### Scenario: Status of OBS-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. References: WAL-002 (the chain is scrubbed). Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:42`, `RUNBOOK.md:58`; `tools/sherlock-sdk/t10a-ledger-run.mjs:529`. Check: T17 (a) first run record.

### Requirement: OBS-009 Row results are MEASURED, DERIVED or ESTIMATE
OBS-009: When a RESULTS file reports a figure, the RESULTS file SHALL label the figure as MEASURED, DERIVED or ESTIMATE.

Verification: review by the lead verdict of each row (LPC-016); MISSING a linter; exists in part.

#### Scenario: A sum of two measured fees
- **GIVEN** the two paid fees of T17 (a)
- **WHEN** the total is reported
- **THEN** the total reads DERIVED

#### Scenario: Status of OBS-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:5,31`; `t20-batched@5dc1be2ee:research/sherlock-t20-batched-20261114/RESULTS.md:5`. Check: review by the lead verdict of each row (LPC-016); MISSING a linter.

### Requirement: OBS-010 Structural validation of the record
OBS-010: When the integration record changes, the tested-design loop SHALL run `openspec validate --strict` on every Sherlock change.

Verification: `openspec validate --strict` (exists, structural only); exists.

#### Scenario: A broken requirement block
- **GIVEN** a change whose requirement has no scenario
- **WHEN** the validation runs
- **THEN** the validation fails and the record change is not committed

#### Scenario: Status of OBS-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. References: Row T2 has a second, separate condition: no ARCHITECTURE row contradicts a committed closure test. That semantic check is an audit, not this command: the audit of 2026-11-15 is `spec-audit:research/sherlock-spec-audit-20261115/AUDIT.md` (15 fixes); LPC-017 keeps one open contradiction. Evidence: rows T1 and T2 (LOOP.md:18-19), `tested@52b71e250`. Check: `openspec validate --strict` (exists, structural only).

### Requirement: OBS-011 Every loop event is logged
OBS-011: When a lane starts, ends, decides or measures, the tested-design loop SHALL append 1 entry to `llm-wiki/wiki/log.md`.

Verification: wiki lint (exists); exists.

#### Scenario: A decision entry
- **GIVEN** the owner's ruling on case A
- **WHEN** the lead records the ruling
- **THEN** `log.md` holds a dated `decision` entry with the owner's words

#### Scenario: Status of OBS-011
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: loop rule (LOOP.md:94); `~/Projects/Moriarty/llm-wiki/wiki/log.md:4111-4113`. Check: wiki lint (exists).

### Requirement: OBS-012 The prover GUI page
OBS-012: The prover GUI page SHALL show the proof stages, the program source and the schedule of each box for a run in 2 modes, live from the daemon bridge and replay from a recorded run.

Verification: MISSING (page beyond the stub). GAP owner: row T21; to build.

#### Scenario: Replay without a daemon
- **GIVEN** a recorded run and no running daemon
- **WHEN** the page opens in replay mode
- **THEN** the page shows leaf, nodes, `L`, `L_Fr` and wrap in order, the source of the program and the shard schedule of each box

#### Scenario: Status of OBS-012
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: LPC-011 (completion); the PGUI requirements (165 IDs) in `prover-gui/requirements/` on branch `prover-gui` stay normative for the views. Owner 2026-11-14: "View code, source, and the flow through the design and how the co-processor is elastic". Slice B owns the daemon bridge. Evidence: row T21 (LOOP.md:43), status "stage 1 requirements". Check: MISSING (page beyond the stub). GAP owner: row T21.
