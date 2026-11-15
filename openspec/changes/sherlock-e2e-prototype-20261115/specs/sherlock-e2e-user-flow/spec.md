## ADDED Requirements

### Requirement: EUF-001 The stage order of one call
EUF-001: When a user runs one end-to-end call, the Sherlock toolchain SHALL run the stages in this order: compile, setup, plan, deploy, tape, execute, prove, wrap, ledger check, submit and read-back.

Verification: the T17 (c) runbook run on the T25 keys with one Mycroft program; to build.

#### Scenario: First joined run
- **WHEN** `t02` `increment` runs from the Compact source to a Preview read-back
- **THEN** each stage reads only the inputs that the stage dependency table of `design.md` names for it

#### Scenario: Status of EUF-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: stages wrap to read-back MEASURED on the T10m keys with a general run (`research/sherlock-t17-preview-20261114/RUNBOOK.md:24-32` on `t17-preview` 9e1e7f398); compile to execute MEASURED apart (ECC-010, ERS-005); no run joins all stages. References: `design.md` flow table, LPC-008, STL-002.

### Requirement: EUF-002 Each stage reads its named inputs
EUF-002: The Sherlock toolchain SHALL give each stage only the artifacts and the private state references that the stage dependency table of `design.md` names for the stage.

Verification: schema test T-FLOW-ARTIFACTS (each stage output validates against its schema) and resume test T-FLOW-RESUME (a second call of a deployed program runs tape to read-back with no second compile, setup or deploy); to build.

#### Scenario: Wrong artifact
- **WHEN** the wrap stage receives a proof bundle with no `L_Fr` proof
- **THEN** the wrap stage stops and names the missing artifact

#### Scenario: Second call
- **WHEN** a user calls a deployed program a second time
- **THEN** the call starts at the tape stage and no deployment is sent

#### Scenario: Status of EUF-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. Schemas exist for `sherlock-profile/1` (`SETUP-SPEC.md:20` on `t16-elastic` a7a7b8867) and `rsl-call/1` by fields only (ERS-001). The table has three scopes: per program (compile, plan, deploy), per box (setup) and per call (tape to read-back). References: `architecture/DDD.md` section 6; LFP, WRP, SLC, OBS (slices B and C).

### Requirement: EUF-005 A failed stage before submit stops the call
EUF-005: If a stage before submit fails, then the Sherlock toolchain SHALL send no transaction for the call and record the stage name and the error.

Verification: fault test T-FLOW-STOP (kill the prove stage; no transaction exists and the run record names the stage); to build.

#### Scenario: Wrap check fails on muninn
- **WHEN** the ledger 9.1.0.1 verifier on muninn refuses a wrap made on odin
- **THEN** the call is not submitted

#### Scenario: Status of EUF-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN; the T17 tooling captures node errors (`RESULTS.md`, 10 tooling fixes). Failures after submit are EUF-014. References: FRC and OBS (slice C).

### Requirement: EUF-006 A natively refused Compact call is not proved or settled
EUF-006: If the execute stage rejects a Compact call, then the Sherlock toolchain SHALL start no prove stage and no submit stage for the call.

Verification: T23 suite rows refused-as-native have no proof file and no transaction; exists in the dry run.

#### Scenario: Native refusal
- **WHEN** the native circuit refuses the call
- **THEN** the suite table reads refused-as-native and Preview receives nothing

#### Scenario: Status of EUF-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT in the dry run (`T23-DRYRUN.md:11-19`); LEAD PICK 4 of `MAP.md:70-75` (D-071). The rule is for the Compact route. A general Sherlock run that ends Rejected or Exhausted stays provable (EUI-005), and the demonstration of EUI-002 proves one. References: ERS-009, MYC-LOW-007, GP-001.

### Requirement: EUF-007 No proof on a box without a profile
EUF-007: While a box has no profile for the installed universal key set, the Sherlock toolchain SHALL refuse a prove stage on the box.

Verification: `t7_chain t16 setup OUT` then `t7_chain t16 hw OUT` with `T16_PROFILE` on each box; exists.

#### Scenario: New key set
- **WHEN** T25 makes a new universal key set
- **THEN** each box runs setup again and writes a new `sherlock-profile/1`

#### Scenario: Status of EUF-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: MEASURED on huginn and odin, muninn profile MEASURED on a loaded host (LOOP.md row T16); the product commands `sherlock setup` and `sherlock prove --profile` are names only (`SETUP-SPEC.md:3`). References: ST-001 to ST-008, PF-001 to PF-008, SC-001 to SC-014 (setup), SPL-011, D-014.

### Requirement: EUF-008 One claim on every box
EUF-008: When two boxes that the user owns prove the same call, the Sherlock prover SHALL give byte-equal `claim_pub` values.

Verification: the T16 three-box table (muninn, huginn, odin) and the two-host join; exists for (c) to (e), two-host join to run.

#### Scenario: Reduced memory budget
- **WHEN** huginn proves the call with one leaf at a time
- **THEN** `claim_pub` equals the value from odin

#### Scenario: Status of EUF-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: MEASURED equal `claim_pub` over the T16 runs (Moriarty `t16-elastic` a7a7b8867, RESULTS-CDE.md); OPEN for `R_SL` calls. References: LPC-006, LPC-007, ERS-011.

### Requirement: EUF-009 A second program needs no new universal key
EUF-009: When a developer deploys a second program, the Sherlock toolchain SHALL use the wrap verifier key of the first program, byte for byte.

Verification: T17 (c) with a second program: the installed VK of both SherlockLedger deployments is byte-equal; to build.

#### Scenario: Two deployments
- **WHEN** `t02` and `a05` are deployed
- **THEN** both contracts hold one wrap VK and differ in `C_P` only

#### Scenario: Status of EUF-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN (LOOP row T17 (c)); one SherlockLedger per contract and circuit is LEAD PICK 3 of `MAP.md:70-75` (D-070). The rule itself is MYC-KEY-002 and CPR-003; this requirement adds the two-deployment check on Preview. References: CPR-003, MYC-KEY-002, MYC-SEAL-015, D-004.

### Requirement: EUF-010 The read-back equals the native outcome
EUF-010: When a settled call is read back, the Sherlock toolchain SHALL compare `runs[claim_pub]` with `outcome_pub` and the outcome with the native transcript of the call.

Verification: the compare stage of `t23.py` on the live run; to build.

#### Scenario: Honest call
- **WHEN** the indexer returns `runs[claim_pub]`
- **THEN** it equals `outcome_pub`, and W equals the native words

#### Scenario: Status of EUF-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: read-back MEASURED on T17 (a) (block 115,187); comparison with native OPEN (T23 live). References: STL-004, STL-005, MYC-ACC-007.

### Requirement: EUF-011 The flow needs no maintainer tooling
EUF-011: The Sherlock toolchain SHALL run the flow of EUF-001 with no DoctorWatson, herdr, box bus or agent tool on the user's box.

Verification: clean-box run T-CLEAN-BOX of the T17 (c) runbook on a host with only the released binaries, `compactc` 0.36.0 and a funded Preview wallet; to build.

#### Scenario: Developer machine
- **WHEN** a developer with no access to the Moriarty repository follows the runbook
- **THEN** the call settles on Preview

#### Scenario: Status of EUF-011
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. The runs use `t7_chain`, `t17.sh`, `t23.py` and the lead's box bus. References: AGENTS.md "Product boundary", CF-002, SCL (slice C).

### Requirement: EUF-012 Version pins stop a mixed run
EUF-012: If the `compactc` version, the ZKIR version, the machine version, the universal key-set identity or the ledger version of an artifact differs from the pin of the run, then the Sherlock toolchain SHALL stop the call and name the differing pin.

Verification: test T-FLOW-PINS (a profile of another key set is refused by the prove stage; a ZKIR 3.2 file is refused by compile); to build.

#### Scenario: Profile of an old key set
- **WHEN** the profile names a `keys_d` that differs from the installed keys
- **THEN** the prove stage stops and asks for setup

#### Scenario: Status of EUF-012
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: parts BUILT: ZKIR version (`lower.rs:188-190`), profile `key_root` and `keys_d` (PF-001 to PF-005), Preview chain probe (`PREVIEW-20261115.md:9-13` on `t23-suite`). References: MYC-FE-002, MYC-ACC-004, LVR (slice C), D-005.

### Requirement: EUF-013 The prototype is done on one joined run per suite call
EUF-013: When the prototype is reported complete, the Sherlock toolchain SHALL show, for one universal key set, the flow of EUF-001 on every suite call that is not refused-as-native or n/a.

Verification: check T-EVIDENCE-COMPLETE of ECS-014 on the final suite table, plus the T12 gate logs on the same key set; to build.

#### Scenario: Interface freeze
- **WHEN** the lead presents T15
- **THEN** the suite table, the gate logs and the Preview receipts name one universal key-set identity (the key manifest of UKS-004), and each row records the `prog_d` and `key_root` of its own program

#### Scenario: Status of EUF-013
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. Critical path: T11, FP-3, T12-SLOT15, T24 and the R_SL universal leaf key, then T25, the T12 gate, T23 and T15 (LOOP.md Finish plan). References: LOOP.md line 5 (tested design), LPC-014, ECS-010, ECS-014.

### Requirement: EUF-014 A submitted call is reconciled, not sent again
EUF-014: If a submitted call has no read-back, then the Sherlock toolchain SHALL reconcile the call by its transaction hash and send no second transaction for the same `claim_pub`.

Verification: fault test T-FLOW-RECONCILE (drop the node response after submit; the runner finds the included transaction, completes the read-back and sends nothing); to build.

#### Scenario: Lost response
- **WHEN** the transport fails after the node took the transaction
- **THEN** the receipt is completed from the chain and the call is not resubmitted

#### Scenario: Status of EUF-014
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN in the flow; the recovery rules exist in slice C. Three post-submit outcomes are kept apart: node refusal (a JSON-RPC error), unknown submission status, and included but unread. Found by Astra-A review R-14. References: FRC-004, FRC-006, PVW-007, PVW-008, OBS-002.
