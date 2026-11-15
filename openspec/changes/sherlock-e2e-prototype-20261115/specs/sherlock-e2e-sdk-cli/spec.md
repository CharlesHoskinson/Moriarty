## ADDED Requirements

### Requirement: SCL-001 The settlement command set
SCL-001: The Sherlock SDK SHALL provide the 7 settlement commands `sherlock_plan`, `lfr_call`, `wrap`, `sherlock-wrap-ledger-check`, `accept_args.py`, `t10a-ledger-run.mjs` and `t17.sh`.

Verification: T17 (a) used every command but `sherlock_plan` and `accept_args.py`; one-entry-point `sherlock` binary is deferred (decision D-085). MISSING an end-to-end script that chains all seven; exists in part.

#### Scenario: One call from plan to receipt
- **GIVEN** a proved run of program A
- **WHEN** the user runs plan, `lfr_call`, `wrap`, the ledger check, the argument builder and the ledger runner in that order
- **THEN** the call is included and `receipts.json` holds its transaction hash

#### Scenario: Status of SCL-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: ANC-012 names `sherlock plan`; STL-001. Evidence: openvm `rekey-chips@dcf1b4c:crates/sherlock-lfr/examples/sherlock_plan.rs`; `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RUNBOOK.md:11` (`lfr_call`, `wrap`); `t10q-ntfix@95990a6d3:tools/sherlock-wrap/src/bin/wrap.rs:4`, `ledger-check/src/main.rs:6`; `t23-args` (staged) `tools/t23-suite/accept_args.py`; `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs`, `research/sherlock-t17-preview-20261114/t17.sh:4`. Check: T17 (a) used every command but `sherlock_plan` and `accept_args.py`; one-entry-point `sherlock` binary is deferred (decision D-085). MISSING an end-to-end script that chains all seven.

### Requirement: SCL-002 Exit codes of the ledger runner
SCL-002: When the ledger runner ends, the ledger runner SHALL exit with 0 for a passed verdict, 4 for a failed verdict and 1 for an error.

Verification: T17 (a) pass run (exit 0); MISSING a test of exit 4; exists in part.

#### Scenario: A failed control
- **GIVEN** a run in which control R2 is included
- **WHEN** the runner ends
- **THEN** the exit status is 4

#### Scenario: Status of SCL-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:59`. Check: T17 (a) pass run (exit 0); MISSING a test of exit 4.

### Requirement: SCL-003 Dry run without a network
SCL-003: Where the `dry-run` phase is selected, the ledger runner SHALL run the padding, the JS shape check and the privacy scan with 0 network connections.

Verification: T23 dry run `t23-suite@daa1ca745` (STL-005); exists.

#### Scenario: Suite dry run
- **GIVEN** the 208 suite calls
- **WHEN** the T23 driver runs them in dry-run mode
- **THEN** the table shows 0 mismatch and no transaction is sent

#### Scenario: Status of SCL-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:60`; `t23-suite@16836929b:tools/t23-suite/t23.py:368`. Check: T23 dry run `t23-suite@daa1ca745` (STL-005).

### Requirement: SCL-004 A state folder is bound to one key
SCL-004: If the deployed key recorded in a state folder differs from the `--vk` argument, then the ledger runner SHALL refuse the state folder.

Verification: MISSING a named test; to build.

#### Scenario: New keys need a new home
- **GIVEN** the state of a T10m deployment
- **WHEN** the runner starts with the T25 wrap VK
- **THEN** the runner refuses and the user sets a new `T17_HOME`

#### Scenario: Status of SCL-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RUNBOOK.md:44`. Check: MISSING a named test.

### Requirement: SCL-005 Machine-readable wrap records
SCL-005: When the wrap tool finishes a phase, the wrap tool SHALL print 1 line that starts with `E5JSON` and holds the phase record as JSON.

Verification: every wrap run; the T25 measurements RKY-003 and RKY-004 read these lines; exists.

#### Scenario: The prove record
- **GIVEN** a wrap run at k = 21
- **WHEN** the prove phase ends
- **THEN** the line holds `k`, `keygen_s`, `prove_s`, `proof_bytes`, `key_bytes_midnightvk`, `library_verify_ok` and `public_inputs`

#### Scenario: Status of SCL-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: `t10q-ntfix@95990a6d3:tools/sherlock-wrap/src/bin/wrap.rs:46-48,68-69,90-91`; `ledger-check/src/main.rs:73-86`. Check: every wrap run; the T25 measurements RKY-003 and RKY-004 read these lines.

### Requirement: SCL-006 The contract builds from committed source
SCL-006: When the SDK builds the SherlockLedger contract, the SDK SHALL compile the committed source file with `compactc` 0.36.0.

Verification: T17 (a) (keys byte-equal to the old build, MEASURED); exists.

#### Scenario: A stale build folder
- **GIVEN** a build made from an uncommitted source copy
- **WHEN** T17 runs
- **THEN** T17 builds again from the committed file and compares the zkir and keys byte for byte

#### Scenario: Status of SCL-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:44`, `RUNBOOK.md:14,72`. Check: T17 (a) (keys byte-equal to the old build, MEASURED).

### Requirement: SCL-007 TTL range check
SCL-007: If `T10A_TTL_S` is not above 0 and below 1,209,600, then the ledger runner SHALL stop before any transaction is built.

Verification: MISSING a named test; to build.

#### Scenario: A TTL above the chain bound
- **GIVEN** `T10A_TTL_S` = 2,000,000
- **WHEN** the runner starts
- **THEN** the runner stops with "T10A_TTL_S out of range"

#### Scenario: Status of SCL-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: BAT-006 (the settlement TTL is 3,600 s). Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:192-193`. Check: MISSING a named test.

### Requirement: SCL-008 The suite driver commands
SCL-008: The T23 suite driver SHALL provide the 2 commands `run` and `selftest`.

Verification: T23 dry run; live settle OPEN until the MAP.md section 5 gaps close; exists in part.

#### Scenario: Selftest before a suite run
- **GIVEN** a box with the suite tools
- **WHEN** `t23.py selftest` runs
- **THEN** the driver checks its own fixtures and exits 0

#### Scenario: Status of SCL-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: STL-002, STL-004, STL-005. Evidence: `t23-suite@16836929b:tools/t23-suite/t23.py:659-672`. Check: T23 dry run; live settle OPEN until the MAP.md section 5 gaps close.

### Requirement: SCL-009 The SDK root is named
SCL-009: If `SHERLOCK_SDK_ROOT` is not set, then the SDK node test suite SHALL stop with a message that names the variable.

Verification: MISSING; today the suite fails 3 tests; to build.

#### Scenario: The three failing tests of T1 and T3
- **GIVEN** a shell with no `SHERLOCK_SDK_ROOT`
- **WHEN** the sdk-node suite runs
- **THEN** the suite stops at start with the variable name and reports no test failure

#### Scenario: Status of SCL-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: LPC-018 (sherlock-completion-20261115). Evidence: LOOP.md:20 ("sdk-node fails 3 of 144 in T1 and T3 without SHERLOCK_SDK_ROOT"). Check: MISSING; today the suite fails 3 tests.

### Requirement: SCL-010 The SDK runs without the factory
SCL-010: Where a user runs the Sherlock SDK outside the factory, the Sherlock SDK SHALL take the wallet directory, the pinned wallet address, the chain pin and the box roster from the user's configuration.

Verification: MISSING (no clean-environment run; the pins are constants); to build.

#### Scenario: A clean machine
- **GIVEN** a user with an own wallet, one own GPU box and no access to muninn
- **WHEN** the user runs the seven commands of SCL-001
- **THEN** the call settles and no step names OP-PV-WALLET, muninn or a factory path

#### Scenario: Status of SCL-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: WAL-003, WAL-007, HLP-005, HLP-007 are then read with the user's machines; AGENTS.md product boundary. Decision D-085. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:95,488,507` (genesis, wallet directory variable and wallet pin: the address is a constant today); `t23-suite@16836929b:tools/t23-suite/t23.py:62-70` (box table in code). Check: MISSING (no clean-environment run; the pins are constants).
