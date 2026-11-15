## ADDED Requirements

### Requirement: WAL-001 The phrase is read in process only
WAL-001: When the settle runner needs the Preview wallet, the settle runner SHALL read the wallet phrase inside its own process from the mode-0600 phrase file of the wallet directory.

Verification: MISSING an automated check (a process-argument and environment scan while the runner runs); to build.

#### Scenario: No shell step sees the phrase
- **GIVEN** `t17.sh`
- **WHEN** a Preview step runs
- **THEN** no shell variable, argument or environment entry holds the phrase

#### Scenario: Status of WAL-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:114-115,487-491`; `research/sherlock-t17-preview-20261114/RUNBOOK.md:5`. This spec was written from the code and the runbook; the wallet directory was not opened. Check: MISSING an automated check (a process-argument and environment scan while the runner runs).

### Requirement: WAL-002 Secrets are scrubbed from every message
WAL-002: When the settle runner writes a message to a log, a state file, an error record or a child process, the settle runner SHALL replace each occurrence of the wallet phrase, of the master seed and of a derived private key with `<seed>`.

Verification: MISSING: a marker-secret test over logs, state, error chains, child-process arguments and receipts; derived keys are not scrubbed today; exists in part.

#### Scenario: A node error that echoes the seed
- **GIVEN** an error text that contains a marker secret
- **WHEN** the runner records the node error
- **THEN** the record holds `<seed>` in its place

#### Scenario: Status of WAL-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: WAL-005, OBS-008; BRIEF.md: wallet handling is never logged. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:116,569` (`scrub` covers the phrase and the master seed hex only). Check: MISSING: a marker-secret test over logs, state, error chains, child-process arguments and receipts; derived keys are not scrubbed today.

### Requirement: WAL-003 The wallet address is pinned
WAL-003: If the address derived from the wallet phrase differs from the wallet address pinned in the run configuration, then the settle runner SHALL stop before the wallet starts.

Verification: MISSING a negative test; the pin is not configurable yet; exists in part.

#### Scenario: Another wallet directory
- **GIVEN** the factory rehearsal, whose pinned address is OP-PV-WALLET, and `MORIARTY_PREVIEW_WALLET_DIR` naming another wallet
- **WHEN** the runner derives the address
- **THEN** the runner stops with "not the OP-PV-WALLET wallet"

#### Scenario: Status of WAL-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: SCL-010 (a user outside the factory pins the user's own address). Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:488,507` (the pin is a constant in the runner today). Check: MISSING a negative test; the pin is not configurable yet.

### Requirement: WAL-004 One sender at a time
WAL-004: While the settle runner syncs, balances, submits or waits for a Preview transaction, the settle runner SHALL hold an exclusive `flock` on `~/.cache/preview-wallet.lock`.

Verification: T17 (a) with the T18 deploys on the same wallet (`RESULTS.md:40`); MISSING a contention test; exists in part.

#### Scenario: Two agents share the wallet
- **GIVEN** a Mycroft deploy and a T17 call at the same time
- **WHEN** both reach the wallet
- **THEN** the second waits for the lock and the log shows the wait time

#### Scenario: Status of WAL-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:537-545`; `research/sherlock-t17-preview-20261114/RUNBOOK.md:68`. Check: T17 (a) with the T18 deploys on the same wallet (`RESULTS.md:40`); MISSING a contention test.

### Requirement: WAL-005 Receipts hold public fields only
WAL-005: When the receipts writer builds `receipts.json`, the receipts writer SHALL copy only the fields of its allow-list from the run state.

Verification: `receipts.json` of T17 (a); MISSING a test with a marker field; exists in part.

#### Scenario: A new private field in the state
- **GIVEN** a run state with a new field outside the allow-list
- **WHEN** receipts are written
- **THEN** the field is absent from `receipts.json`

#### Scenario: Status of WAL-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/t17.sh:117-125`. Check: `receipts.json` of T17 (a); MISSING a test with a marker field.

### Requirement: WAL-006 The balance step writes public balances only
WAL-006: When the balance phase runs, the settle runner SHALL write the public balances of the wallet and no key material.

Verification: T17 (a) balance records; exists.

#### Scenario: Balance before and after
- **GIVEN** the wallet at height 113,939
- **WHEN** `balance-before` runs
- **THEN** the file holds the tDUST and NIGHT balances, the chain tip and the wallet address

#### Scenario: Status of WAL-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:597-605`; `research/sherlock-t17-preview-20261114/RESULTS.md:27`. Check: T17 (a) balance records.

### Requirement: WAL-007 One machine holds the wallet
WAL-007: The settle runner SHALL submit every Preview transaction from the 1 machine that holds the wallet directory.

Verification: MISSING (no check that a proving box holds no wallet directory). GAP owner: row T23 driver; to build.

#### Scenario: A box proves and the wallet machine submits
- **GIVEN** the factory rehearsal, where muninn holds the wallet, and a wrap proof made on odin
- **WHEN** the call settles
- **THEN** odin returns the proof and the 4 public words, and muninn submits

#### Scenario: Status of WAL-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: HLP-001, HLP-003, HLP-007. Decision D-080. Evidence: Finish plan box table "wallet stays here" (LOOP.md:58). Check: MISSING (no check that a proving box holds no wallet directory). GAP owner: row T23 driver.

### Requirement: WAL-008 No key material in the repository
WAL-008: The Moriarty repository SHALL hold 0 wallet phrases, seeds, private keys and deployment blinders `r_P`.

Verification: MISSING a secret-scan gate in the commit hook; to build.

#### Scenario: The run state stays on the host
- **GIVEN** the run state `~/.cache/t17/preview/t10a-state.json`
- **WHEN** T17 commits its results
- **THEN** only `receipts.json` enters the repository

#### Scenario: Status of WAL-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: ZK-012 (`r_P` per deployment, box-private); MAP.md pick 3 ("never in the repo, never on chain"). Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:3`; MAP.md:74 (branch `t23-args`, staged); test constants `r_P = 1000..1007` are not deployment secrets (MAP.md:68). Check: MISSING a secret-scan gate in the commit hook.

### Requirement: WAL-009 An existing wallet is never overwritten
WAL-009: If a wallet phrase file exists in the wallet directory, then the wallet setup tool SHALL refuse to create a wallet in that directory.

Verification: `tools/sherlock-sdk/preview-session.test.mjs` (no-overwrite test, exists on branch `preview-deploy`); exists.

#### Scenario: A second setup run
- **GIVEN** a wallet directory of mode 0700 with a phrase file of mode 0600
- **WHEN** the setup tool runs again
- **THEN** the tool stops with "a wallet seed already exists" and changes no file

#### Scenario: Status of WAL-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. References: WAL-001; AGENTS.md rule to preserve existing wallets and keys. Evidence: `preview-deploy@78592bf59:tools/sherlock-sdk/preview-session.mjs:36-50,110` (directory 0700, exclusive create with flag `wx`, mode 0600 asserted); read from code only. Check: `tools/sherlock-sdk/preview-session.test.mjs` (no-overwrite test, exists on branch `preview-deploy`).

### Requirement: WAL-010 Permissions are checked before the phrase is read
WAL-010: If the mode of the wallet directory is wider than 0700 or the mode of the phrase file is wider than 0600, then the settle runner SHALL stop before the phrase is read.

Verification: MISSING on the read path of the ledger runner; the setup tool asserts both modes (`preview-session.mjs:49-50`); to build.

#### Scenario: A group-readable phrase file
- **GIVEN** a phrase file of mode 0640
- **WHEN** the settle runner starts a Preview step
- **THEN** the runner stops and names the file mode, and reads nothing

#### Scenario: Status of WAL-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: WAL-001, WAL-009. Decision D-080. Evidence: `preview-deploy@78592bf59:tools/sherlock-sdk/preview-session.mjs:36-38,49-50` asserts 0700 and 0600 in the setup tool; the ledger runner reads the file with no mode check (`t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:487-491`). Read from code only. Check: MISSING on the read path of the ledger runner; a test with a synthetic directory and a marker file.
