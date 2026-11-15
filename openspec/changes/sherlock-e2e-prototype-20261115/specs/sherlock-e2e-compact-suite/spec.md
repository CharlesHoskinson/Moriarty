## ADDED Requirements

### Requirement: ECS-001 The suite manifest
ECS-001: When `setup_suite.py` builds the suite, the suite setup tool SHALL record for each program its source repository, path and commit, the file sha256 values, the compile result and the per-circuit `k`, rows and ZKIR operation counts in a manifest of schema `t18-compact-suite-manifest/1`.

Verification: `mycroft/tests/compact-suite/MANIFEST.json` validates against the schema name and lists 36 programs; exists.

#### Scenario: Program that does not compile natively
- **WHEN** stock `compactc` refuses `d05-kitties`
- **THEN** the manifest records the compile failure and the suite makes no call for it

#### Scenario: Status of ECS-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (worktree `mycroft-factory`, `MANIFEST.json:2-80`; compactc "0.36.0 (b5d9e4dfc 2026-11-03)", `MANIFEST.json:3-6`); 33 of 36 compile, w01, d05 and d07 do not (`PREVIEW.md:32`). References: LOOP row T18 (a), MYC-LOW-018.

### Requirement: ECS-002 One native reference per call
ECS-002: When the native driver runs a suite call, the native driver SHALL store the transcript, the preimage and one reference record of schema `mycroft-native-ref/1` under `native/<id>/transcripts/`.

Verification: `tools/ref_vectors.py` regenerates the reference records and gives byte-equal files; exists.

#### Scenario: Rejected native call
- **WHEN** the native circuit rejects a call
- **THEN** the reference record holds the rejection and no transcript words

#### Scenario: Status of ECS-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (`mycroft/tests/compact-suite/README.md:13-29`); 208 calls, 146 ok and 62 rejected natively (`RESULTS.md:5`). References: MYC-TRN-008, STL-006.

### Requirement: ECS-003 The Part B stages
ECS-003: When the T23 runner runs a call, the T23 runner SHALL run the stages compile, prove, settle and compare in that order.

Verification: `tools/t23-suite/t23.py selftest`; exists.

#### Scenario: Compile stage differs
- **WHEN** the compile stage gives W that differs from the native reference
- **THEN** the call is a mismatch and the prove stage does not run

#### Scenario: Status of ECS-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (worktree `t23-suite` 16836929b, `t23.py:9-31`); dry run DONE at daa1ca745 with prover odin and settle dry. References: LOOP row T23, STL-002, STL-004.

### Requirement: ECS-004 One result class per call
ECS-004: The T23 runner SHALL give each call exactly one of the classes pass, refused-as-native, mismatch, blocked, prove-failed and n/a.

Verification: `t23.py selftest` covers `classify`; the suite table sums to 208; exists.

#### Scenario: Pure circuit
- **WHEN** a circuit has no ZKIR, no proof and no transaction (h02 `nativeMethods`)
- **THEN** the class is n/a

#### Scenario: Status of ECS-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (`t23.py:398-457`); dry run: pass 0, refused-as-native 62, mismatch 0, blocked 145, prove-failed 0, n/a 1 (`research/sherlock-t23-suite-20261115/T23-DRYRUN.md:11-19`). References: STL-005.

### Requirement: ECS-005 A blocked call names its reason
ECS-005: If the T23 runner cannot prove or compare a call, then the T23 runner SHALL class the call as blocked with one reason text.

Verification: the suite table column reason is non-empty for every blocked row; exists.

#### Scenario: Proof with no native Preview transcript
- **WHEN** a call proves and no passing native Preview transcript exists for it
- **THEN** the class is blocked with the reason "no native Preview transcript"

#### Scenario: Status of ECS-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (`t23.py:61`, `:141-152`, `:454`); reasons in the dry run: R_SL chip set 139, kit jubjub 3, kit secp256k1 2, kit keccak256 1 (`T23-DRYRUN.md:25-30`). References: MYC-ACC-008.

### Requirement: ECS-006 Kit reasons follow the machine version
ECS-006: While the machine version holds the kit of a call, the T23 runner SHALL not class the call as blocked for that kit.

Verification: `t23.py selftest` case T-KIT-REASONS (with jubjub, secp256k1 and keccak256 enabled, the 6 kit calls reach the prove stage); to build.

#### Scenario: After T24
- **WHEN** the T24 kits are in the T25 key set
- **THEN** the dry run shows 0 calls blocked on a kit

#### Scenario: Status of ECS-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. `t23.py:141-152` still reads "kit secp256k1 deferred to a later chip set" and "kit tier3 (keccak256) deferred", against the owner ruling "yes to both" (wiki 2026-11-15 09:06). References: ECC-007, CF-013, RSL-004 to RSL-006, D-019.

### Requirement: ECS-007 Live dispatch needs its switch
ECS-007: While `T23_ALLOW_DISPATCH` is not set to 1, the T23 runner SHALL send no call to a prover box.

Verification: `t23.py selftest` case for `--dispatch` without the variable; exists.

#### Scenario: Dry mode
- **WHEN** the runner starts with `--mode dry`
- **THEN** no box bus message is written

#### Scenario: Status of ECS-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (`t23.py:28-31`). References: LOOP.md loop rules (heavy jobs through `heavy-slot`), ELA (slice B).

### Requirement: ECS-008 Preview settlement needs its switch
ECS-008: While `T23_ALLOW_PREVIEW` is not set to 1, the T23 runner SHALL submit no Preview transaction.

Verification: dry run record "0 Preview transactions" (`T23-DRYRUN.md:3`); exists.

#### Scenario: Settle dry
- **WHEN** the runner settles with `--settle dry`
- **THEN** the wallet is not opened

#### Scenario: Status of ECS-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. References: PVW and WAL (slice C), owner 2026-11-14 Preview allowed with the existing wallet.

### Requirement: ECS-009 A derived refusal is labelled
ECS-009: While a refused-as-native row has no executed `R_SL` rejection, the T23 runner SHALL label the row DERIVED in the suite table.

Verification: suite table column evidence for the 62 refused rows; exists.

#### Scenario: Executed refusal
- **WHEN** a tape exists for a rejected call and the executor rejects it
- **THEN** the row evidence reads MEASURED with the reject code

#### Scenario: Status of ECS-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT: the 62 rows are DERIVED, not run, because the suite holds no native preimage or tape for a rejected call (`t23.py:398-457`). Gap: an executed refusal for each of the 62 calls; owner lane Mycroft (T18 d). References: MYC-ACC-008, MYC-LOW-006.

### Requirement: ECS-010 The final suite table
ECS-010: When the T23 live run ends, the T23 runner SHALL write one suite table with, per call, the class, the Preview transaction hash and block, the comparison with the native transcript, and the prove and wrap time per box.

Verification: the T23 live run on the T25 keys, with every wrap checked on muninn by the ledger 9.1.0.1 verifier; to build.

#### Scenario: Mismatch in the live run
- **WHEN** one settled outcome differs from the native transcript
- **THEN** the table counts 1 mismatch and the row T23 is not done

#### Scenario: Status of ECS-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN: waits on RKY-001, the T12 gate and the `acceptRun` argument map for `R_SL` (ERS-006 to ERS-008). References: STL-002 to STL-005, LOOP row T23 done-when.

### Requirement: ECS-011 Circuit and call counts are stated one way
ECS-011: The T23 runner SHALL report 33 programs, 106 circuits and 208 calls, where the 106 circuits include the pure circuit of h02.

Verification: counts of `out/suite-coverage.json` against the suite table; exists.

#### Scenario: Count check
- **WHEN** the suite table is written
- **THEN** its rows sum to 208 calls over 106 circuits

#### Scenario: Status of ECS-011
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: `RESULTS.md:5` counts 105 proof circuits and `REPORT.md:8` counts 106 with the pure h02 `nativeMethods`; both describe one suite. References: ECC-010.

### Requirement: ECS-012 The mismatch count states its denominator
ECS-012: When the T23 runner reports the mismatch count, the T23 runner SHALL report with it the count of calls that reached the compare stage after a proof.

Verification: the suite table line "Mismatch: n of m compared after a proof" (exists, `t23.py:494`) and the structured field `compared` with `t23.py selftest` case T-COMPARED (to build).

#### Scenario: Dry run
- **WHEN** no call is proved
- **THEN** the table reads mismatch 0 of 0 compared

#### Scenario: Status of ECS-012
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: display BUILT, structured field and regression test OPEN. The dry run reports 0 mismatch with 0 proved comparisons (pass 0, blocked 145), so its zero is vacuous (`T23-DRYRUN.md:11-21`). Found by the Astra-A draft. References: STL-005, MYC-ACC-008.

### Requirement: ECS-013 One identity per call in every count
ECS-013: The T23 runner SHALL map each recorded scenario call and each executed vector to one identity of program, circuit, label and index, and use that identity in every count that the suite table states.

Verification: check T-CALL-ID (the 142 executed vectors, the 139 equal calls and the 208 recorded calls resolve to one identity list with no duplicate); to build.

#### Scenario: 142 against 208
- **WHEN** the table is written
- **THEN** each of the 142 executed vectors names the recorded call or calls that it covers

#### Scenario: Status of ECS-013
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. `SUITE-COVERAGE.md:5`, `:119`, `LIST.md:30` and LOOP.md row T23 state 142 vectors, 139 calls and 208 calls over different scopes. Found by the Astra-A draft. References: ECS-011, ECC-010.

### Requirement: ECS-014 Completion needs the whole evidence of each call
ECS-014: When the T23 runner reports the live run complete, the T23 runner SHALL hold for each eligible call of the pinned manifest the proof and key-set identity, the stock and ledger 9.1.0.1 verification results, the included Preview transaction and block, the read-back and the native comparison.

Verification: check T-EVIDENCE-COMPLETE over the whole manifest: an empty, omitted, duplicate or extra vector fails it, and deleting any one evidence item of one call fails it; to build.

#### Scenario: Missing receipt
- **WHEN** one eligible call has a proof and no included transaction
- **THEN** the run is not complete and the table names the call

#### Scenario: Status of ECS-014
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. `classify` can give pass after a dry settlement (`t23.py:371`, `:444-448`), live settlement stops by design (`t23.py:378`), and `compare` returns equal on an empty vector list (`t23.py:222-240`). Found by Astra-A review R-17. References: STL-002, STL-004, STL-005, ECS-010, ECS-012, ECS-013.

### Requirement: ECS-015 A dry settle is not a pass
ECS-015: If the T23 runner settled a call without a Preview transaction, then the T23 runner SHALL class the call as blocked with the reason `preview off`.

Verification: a `t23.py selftest` case with `T23_ALLOW_DISPATCH=1` and no `T23_ALLOW_PREVIEW`; to build.

#### Scenario: Dispatch on, Preview off
- **GIVEN** a call that proves and compares equal to the native transcript
- **WHEN** settle runs dry because Preview is off
- **THEN** the class is blocked with the reason `preview off`, not pass

#### Scenario: Status of ECS-015
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. Added for FAL-CONFORMANCE-1 (LEAD PICK for T15). References: ECS-004, ECS-005, ECS-008, ECS-010, ECS-014. Evidence: `classify` can give pass after a dry settlement (`t23.py:371`, `:444-448`; ECS-014 status). Check: MISSING the selftest case.
