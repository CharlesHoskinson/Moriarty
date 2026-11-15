## ADDED Requirements

### Requirement: STL-001 acceptRun arguments from the Mycroft statement
STL-001: When the settle runner builds an `acceptRun` call for a suite call, the settle runner SHALL derive `c_p`, `input_pub`, `outcome_pub` and `claim_pub` from the Mycroft statement and the wrap public inputs.

#### Scenario: acceptRun arguments from the Mycroft statement
- **WHEN** the builder runs on the T17 (a) call
- **THEN** its arguments equal the included transaction

#### Scenario: Verification and status of STL-001
- **WHEN** the status is reported
- **THEN** the source is row T23, lane t23-args, the verification is the builder reproduces the T17 (a) accepted arguments byte for byte (MAP.md on `t23-args`), and the status on 2026-11-15 is: OPEN: branch `t23-args` has no commit past main 0265fb3af.

### Requirement: STL-002 Full chain per call
STL-002: When the suite prover proves a suite call, the suite prover SHALL prove app, leaf, N1, N2, N3, `L`, `L_Fr` and the `k = 21` wrap on the rekeyed universal key set.

#### Scenario: Full chain per call
- **WHEN** a call proves on odin
- **THEN** muninn checks its wrap with the ledger 9.1.0.1 verifier before a KEEP

#### Scenario: Verification and status of STL-002
- **WHEN** the status is reported
- **THEN** the source is row T23, the verification is per call: stock verifier and ledger 9.1.0.1 verifier accept, prove and wrap time MEASURED per box, and the status on 2026-11-15 is: OPEN: waits on RKY-001 and T12.

### Requirement: STL-003 Redeploy with slot 15
STL-003: When the rekey and T12 pass, the deploy runner SHALL deploy SherlockLedger with the new wrap VK and the anchor in slot 15 on Preview.

#### Scenario: Redeploy with slot 15
- **WHEN** the installed VK differs
- **THEN** the deploy runner stops

#### Scenario: Verification and status of STL-003
- **WHEN** the status is reported
- **THEN** the source is LOOP.md line 72 (Preview redeploy after T12), owner 2026-11-14 Preview allowed, the verification is contract address, deploy block, installed VK byte-equal to the new wrap VK, and the status on 2026-11-15 is: OPEN.

### Requirement: STL-004 Settlement of every call
STL-004: When a suite call settles, the SherlockLedger contract SHALL set `runs[claim_pub]` to `outcome_pub`.

#### Scenario: Settlement of every call
- **WHEN** the call is included
- **THEN** a ledger read gives `runs[claim_pub] = outcome_pub`

#### Scenario: Verification and status of STL-004
- **WHEN** the status is reported
- **THEN** the source is row T23, the verification is tx hash and block per call, 208 calls, and the status on 2026-11-15 is: OPEN.

### Requirement: STL-005 Mismatch count
STL-005: If a settled outcome differs from the native Preview transcript of T18 (a), then the suite report SHALL record the call as a mismatch.

#### Scenario: Mismatch count
- **WHEN** native refuses a call
- **THEN** Sherlock refuses it too and the table counts refused-as-native

#### Scenario: Verification and status of STL-005
- **WHEN** the status is reported
- **THEN** the source is row T23 (mismatch must be 0), the verification is suite table: pass, refused-as-native, mismatch, blocked with reason. Dry run DONE (t23-suite daa1ca745): 0 mismatch, 62 refused-as-native, 145 blocked, 1 pure, and the status on 2026-11-15 is: dry run DONE, live run OPEN.

### Requirement: STL-006 Part A native rerun
STL-006: When Part A runs, the native runner SHALL rerun the 33 suite programs on Preview and compare each call with the 2026-11-14 transcript.

#### Scenario: Part A native rerun
- **WHEN** Preview had a hard fork
- **THEN** the report names the spec version and splits the comparison

#### Scenario: Verification and status of STL-006
- **WHEN** the status is reported
- **THEN** the source is row T23, the verification is units `t23-partA` and `t23-partA-finalize`, report committed by finalize, and the status on 2026-11-15 is: RUNNING: 3 of 33 programs, 4 calls word-equal at 08:30.
