## ADDED Requirements

### Requirement: EGV-001 Every Sherlock contract is sealed at deploy
EGV-001: When the deploy runner deploys a Sherlock contract, the deploy runner SHALL set the maintenance authority to an empty committee with threshold 1 and counter 0.

Verification: Quint `research/sherlock-t22-governance-20261114/quint/eraTable.qnt`, invariant `inv_acceptedCheckable` with mutant `maintenance`. MISSING a deploy tool (T22 tasks.md task 4); exists in part.

#### Scenario: No maintenance update is possible
- **GIVEN** a deployed SherlockLedger with the empty committee
- **WHEN** any party submits a verifier-key update
- **THEN** ledger 9.1.0.1 rejects the update

#### Scenario: Status of EGV-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: ERT-001, ELD-001, GOV-009 (change `sherlock-governance`, branch `t22-governance@fe7207819`). Evidence: `t22-governance@fe7207819:research/sherlock-t22-governance-20261114/REPORT.md:8` (ledger rev a2b3012 `verify.rs:1929-1948`; threshold 0 is not sealed; a deploy checks only `counter == 0`, `verify.rs:408`). Check: Quint `research/sherlock-t22-governance-20261114/quint/eraTable.qnt`, invariant `inv_acceptedCheckable` with mutant `maintenance`. MISSING a deploy tool (T22 tasks.md task 4).

### Requirement: EGV-002 Readback gate
EGV-002: If the readback of a deployed contract differs from the maintenance authority `{[], 1, 0}` or from the expected key of an entry, then the deploy runner SHALL stop and mark the contract as unusable.

Verification: MISSING (T22 tasks.md task 4: a sealed deploy tool with the readback gate; controls: the SDK default and threshold 0); to build.

#### Scenario: The SDK default path is caught
- **GIVEN** a deploy through compact-js, which installs a one-key committee
- **WHEN** the readback gate runs
- **THEN** the gate fails and no announcement, attestation or call names the contract

#### Scenario: Status of EGV-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: ERT-002, ELD-002 (governance): committee, threshold, counter and every installed entry key, native read keys included (SLC-003, SLC-004). Evidence: `t22-governance@fe7207819:research/sherlock-t22-governance-20261114/REPORT.md:8-9` (compact-js `executable.js:492`; threshold 0 is not sealed). Check: MISSING (T22 tasks.md task 4: a sealed deploy tool with the readback gate; controls: the SDK default and threshold 0).

### Requirement: EGV-003 Unsealed deployments are path evidence only
EGV-003: If a deployed SherlockLedger is not sealed, then the prototype report SHALL label the receipts of that ledger as transaction-path evidence.

Verification: review of the T17 (c) and T23 reports; MISSING an automated check; exists in part.

#### Scenario: The T17 (a) ledger
- **GIVEN** the ledger at `740653...e274` on Preview
- **WHEN** the prototype report lists it
- **THEN** the row reads "not sealed, transaction-path evidence"

#### Scenario: Status of EGV-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. Evidence: `t22-governance@fe7207819:research/sherlock-t22-governance-20261114/REPORT.md:9,59`. Check: review of the T17 (c) and T23 reports; MISSING an automated check.

### Requirement: EGV-004 One era for each key set
EGV-004: When the T25 key set is deployed, the deploy runner SHALL record exactly 1 era in the SherlockEraTable, with the digest `keyset_d` of the T25 universal key set.

Verification: Quint `eraTable.qnt` invariant `inv_eraKeyUnique` with mutant `reuseKeyset` (exists, model only). MISSING: a deployed table and a check that `keyset_d` binds the whole key set; exists in part.

#### Scenario: A second era for the same key set is refused
- **GIVEN** an era table that holds the T25 era
- **WHEN** an announcement names the same `keyset_d`
- **THEN** the era table refuses the announcement

#### Scenario: Status of EGV-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: ERT-003, ERT-006 (governance). The encoding of `keyset_d` (which keys it commits to, and how per-program ledgers are attested under T22 O8) is not decided: listed under decision D-077. Evidence: `t22-governance@fe7207819:research/sherlock-t22-governance-20261114/REPORT.md:7,12,43`. Check: Quint `eraTable.qnt` invariant `inv_eraKeyUnique` with mutant `reuseKeyset` (exists, model only). MISSING: a deployed table and a check that `keyset_d` binds the whole key set.

### Requirement: EGV-005 The era check enters the single rekey or a premise failure is on record
EGV-005: If the T25 rekey list holds no row for the era-check words and the T22 premise test holds no failed result for premise A2 or A3, then the rekey runner SHALL refuse to start.

Verification: MISSING (no premise test, no list row, no check in the rekey runner). GAP owners: row T22 for the test, slice B (UKS) for the list; to build.

#### Scenario: Today's list is refused
- **GIVEN** the rekey list of 2026-11-15, with no era row, and no premise test result
- **WHEN** the T25 rekey is started
- **THEN** the runner refuses and names T22 tasks 2 and 3

#### Scenario: Status of EGV-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: ELD-003, ELD-004, ELD-006 (primary path: one `active(era_id)` call at the end of each wrap-keyed entry, 85 and 91 lanes); ELD-007 (fallback where premise A2 or A3 fails); RKY-001, RKY-002 (one rekey from one list, completion). Decision D-078. Evidence: `~/.cache/rekey-t11/LIST.md` (36 lines, read 2026-11-15, no era row); `t22-governance@fe7207819:research/sherlock-t22-governance-20261114/REPORT.md:10,58`; `openspec/changes/sherlock-governance/tasks.md:7` ("before T12"). Check: MISSING (no premise test, no list row, no check in the rekey runner). GAP owners: row T22 for the test, slice B (UKS) for the list.

### Requirement: EGV-006 The fallback checks the era at first use
EGV-006: Where the premise test records a failed premise A2 or A3, when a consumer contract uses a claim, the consumer contract SHALL check `active(era_id)` on the era table.

Verification: MISSING: the use-time fallback is not modelled (REPORT.md:53). Quint `eraTable.qnt` covers `inv_noUnsafeUse` with mutant `keepBrokenClaims` for the primary path only; to build.

#### Scenario: A claim of an ended era is refused at use
- **GIVEN** the fallback form and an era with `soundness_end` set
- **WHEN** a consumer uses a claim of that era
- **THEN** the consumer call fails

#### Scenario: Status of EGV-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: ELD-007, CRR-004, CRR-006 (governance). Evidence: `t22-governance@fe7207819:openspec/changes/sherlock-governance/specs/sherlock-consumer-read/spec.md:24,38`. Check: MISSING: the use-time fallback is not modelled (REPORT.md:53). Quint `eraTable.qnt` covers `inv_noUnsafeUse` with mutant `keepBrokenClaims` for the primary path only.

### Requirement: EGV-007 A new key set is a new era
EGV-007: If a tool asks for a verifier-key change on a deployed Sherlock contract, then the deploy runner SHALL refuse the request.

Verification: Quint `eraTable.qnt` invariants `inv_activeEraGoverned` (mutants `noTimelock`, `singleSigner`) and `inv_rotationKeepsClaimKeys` (mutant `rotateRekeys`). Carbon lead verdict MISSING (LPC-012); exists in part.

#### Scenario: A rekey after the prototype
- **GIVEN** a deployed era with key set K1
- **WHEN** a key set K2 is ready
- **THEN** K2 enters as a new era with new sealed ledgers, announced by the Fenrir operator quorum

#### Scenario: Status of EGV-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: GOV-001, GOV-002, GOV-003, ERT-006. Owner words: "a set of keys rotated from the Fenrir operators" and "a functional version system with backwards compatibility like Cardano does with the hard fork combinator" (LOOP.md:44). Evidence: llm-wiki `wiki/log.md:3670,3680`; owner confirmation `wiki/log.md:4107`. Check: Quint `eraTable.qnt` invariants `inv_activeEraGoverned` (mutants `noTimelock`, `singleSigner`) and `inv_rotationKeepsClaimKeys` (mutant `rotateRekeys`). Carbon lead verdict MISSING (LPC-012).

### Requirement: EGV-008 Open governance parameters are listed at the freeze
EGV-008: If a governance parameter among `TL`, `RTL`, `Q`, `n`, `GRID` and `SKEW` has no owner ruling at T15, then the T15 freeze record SHALL list the parameter as open with its recommended value.

Verification: MISSING until the T15 record exists; then a review of the record (LPC-014); to build.

#### Scenario: The recommended values are not rules
- **GIVEN** the recommended `TL` = 7 days and `Q` = 12 of `n` = 16
- **WHEN** the freeze record is written with no ruling on them
- **THEN** the record lists both as open and no spec text states them as rules

#### Scenario: Status of EGV-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. Evidence: `t22-governance@fe7207819:research/sherlock-t22-governance-20261114/OPEN-QUESTIONS.md:27,53,66,82,86,90` and `REPORT.md:14,57`. Check: MISSING until the T15 record exists; then a review of the record (LPC-014). Decision D-077.

### Requirement: EGV-009 Era-check premises are tested on a devnet
EGV-009: When the T22 premise test runs on a ledger 9.1.0.1 devnet, the premise test SHALL record a pass or fail result for each of the 3 premises A1, A2 and A3.

Verification: MISSING (T22 tasks.md tasks 2 and 3); to build.

#### Scenario: A call included after the era end leaves no write
- **GIVEN** a call proved while its era is active
- **WHEN** the call is included after `end_eff`
- **THEN** the ledger state is unchanged

#### Scenario: Status of EGV-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: ELD-003, ELD-004, ELD-007. Evidence: `t22-governance@fe7207819:research/sherlock-t22-governance-20261114/REPORT.md:58` (compile results only; premises untested). Check: MISSING (T22 tasks.md tasks 2 and 3).

### Requirement: EGV-010 A proof-version change is a new table and new ledgers
EGV-010: If a Midnight hard fork is announced that stops the verification of the proof version of the deployed contracts, then the governance operators SHALL deploy a new era table and new era ledgers with a predecessor link before the fork height.

Verification: MISSING (no migration drill, no model); to build.

#### Scenario: A sealed contract gains no key
- **GIVEN** sealed contracts on proof version v2 and an announced fork to v3 only
- **WHEN** the fork height nears
- **THEN** new sealed contracts exist for v3, and one quorum-signed snapshot root of the old accepted roots is marked as a trust downgrade

#### Scenario: Status of EGV-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: T22 O11 (recommended option, not ruled): LEAD PICK for T15 under decision D-077. EGV-007 holds: no verifier-key change on a deployed contract. Evidence: `t22-governance@fe7207819:research/sherlock-t22-governance-20261114/OPEN-QUESTIONS.md:21` (ledger a2b3012: `VerifierKeyInsert` is maintenance). Check: MISSING (no migration drill, no model).
