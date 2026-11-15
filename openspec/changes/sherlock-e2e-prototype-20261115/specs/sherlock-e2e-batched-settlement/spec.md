## ADDED Requirements

### Requirement: BAT-001 Epoch close
BAT-001: When the first job of an epoch arrives, the epoch operator SHALL close the epoch at the earlier of 3,600 s after that job and 1,048,576 claims.

Verification: Quint `models/sherlock/quint/t20-batched/epochSettle.qnt` (`Inv`, mutants `per_job_accept_run`, `collection_drops_job`, `split_epoch`). No operator code exists: MISSING a run; exists in part.

#### Scenario: A quiet epoch closes on time
- **GIVEN** an epoch whose first job arrived at time t
- **WHEN** 3 jobs arrive before t + 3,600 s
- **THEN** the epoch closes at t + 3,600 s with one collection of 3 claims

#### Scenario: Status of BAT-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: LDG-005, LDG-006 as amended on `t20-batched` (one collection per epoch, no write for an empty epoch); hard rule "Midnight writes batched, one root per checkpoint". Evidence: `t20-batched@5dc1be2ee:research/sherlock-t20-batched-20261114/RESULTS.md:33,54`; epoch 3,600 s and depth 20 are owner-confirmed T20 picks (LOOP.md:106). Check: Quint `models/sherlock/quint/t20-batched/epochSettle.qnt` (`Inv`, mutants `per_job_accept_run`, `collection_drops_job`, `split_epoch`). No operator code exists: MISSING a run.

### Requirement: BAT-002 Operator jobs do not use acceptRun
BAT-002: If the submitter of a proved job is not the user who owns the run, then the settle runner SHALL route the job to the epoch collection and send no `acceptRun` for the job.

Verification: Quint `epochSettle.qnt` invariant `OperatorWritesPerEpochAtMostOne` with mutant `per_job_accept_run`. MISSING in the runner: the contract cannot tell an operator from a user; exists in part.

#### Scenario: Per-job acceptRun by an operator is a defect
- **GIVEN** an operator with 3 proved jobs in one epoch
- **WHEN** the epoch closes
- **THEN** the ledger sees 1 transaction, not 3

#### Scenario: Status of BAT-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: LDG-002 second paragraph and LDG-006 on `t20-batched`. Evidence: `t20-batched@5dc1be2ee:tools/sherlock-sdk/sherlock-ledger.compact:52-53`; `RESULTS.md:31-33`. Check: Quint `epochSettle.qnt` invariant `OperatorWritesPerEpochAtMostOne` with mutant `per_job_accept_run`. MISSING in the runner: the contract cannot tell an operator from a user.

### Requirement: BAT-003 Write counts in every settlement report
BAT-003: When a settlement report is written, the settle runner SHALL state the transaction count, the bytes on chain and the wrap proof count for each request, for each epoch and for each day.

Verification: the T20 tables. MISSING in a runner: no settle report prints these counts yet; exists in part.

#### Scenario: Counts at the ceiling of one prover
- **GIVEN** one prover at 492 jobs per day and a 3,600 s epoch
- **WHEN** the report is written
- **THEN** the report shows at most 24 transactions and about 163 KB per day with batching, against 492 transactions and 3.34 MB per day without

#### Scenario: Status of BAT-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. Evidence: `t20-batched@5dc1be2ee:research/sherlock-t20-batched-20261114/RESULTS.md:11-15,45-56` (MEASURED 6,783 bytes for `acceptRun`, DERIVED rates, ESTIMATE 6,797 bytes for `acceptCollection`). Check: the T20 tables. MISSING in a runner: no settle report prints these counts yet.

### Requirement: BAT-004 Suite settlement counts
BAT-004: When the T23 suite report is written, the suite report SHALL state the number of SherlockLedger deploys and the number of `acceptRun` transactions of the run.

Verification: the T23 suite table (STL-004); live run OPEN; exists in part.

#### Scenario: The planned T23 counts
- **GIVEN** 33 programs, 106 circuits and 208 calls, of which 62 are refused as native and 1 is pure
- **WHEN** the suite settles each remaining call by its own user's `acceptRun`
- **THEN** the report shows 145 `acceptRun` transactions and at most 106 deploys (DERIVED: 208 - 62 - 1, and one ledger for each circuit under SLC-005)

#### Scenario: Status of BAT-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: STL-004, STL-005; SLC-005. Evidence: LOOP.md:40,45 (33 programs, 106 circuits, 208 calls; dry run 145 blocked, 62 refused-as-native, 1 pure). A suite call is the user's own run, so W-1 admits `acceptRun` (`t20-batched@5dc1be2ee:tools/sherlock-sdk/sherlock-ledger.compact:52-53`). Check: the T23 suite table (STL-004); live run OPEN.

### Requirement: BAT-005 Chunk cap
BAT-005: If an entry needs more than 2 chunks, then the plan tool SHALL refuse the entry at plan time, at every intent TTL.

Verification: Quint model of fix 4 on `t20-batched`; MISSING a test of `sherlock_plan` at TTL 14,400 s (openvm `rekey-chips@dcf1b4c:crates/sherlock-lfr/examples/sherlock_plan.rs`); exists in part.

#### Scenario: A longer TTL does not raise the cap
- **GIVEN** an intent TTL of 14,400 s
- **WHEN** an entry needs 3 chunks
- **THEN** the plan tool refuses and names the entry, its `L_e` and `K_MAX` = 2

#### Scenario: Status of BAT-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: ANC-012 on `t20-batched`. Supersedes its formula `min(floor(TTL / 1,755 s), 18)` above 2: the owner accepted the cap as `K_MAX = 2` (llm-wiki `wiki/log.md:3502-3503`; hard rule in BRIEF.md). Evidence: `t20-batched@5dc1be2ee:openspec/changes/sherlock-processor-final-20261111/specs/sherlock-anchor-chunks/spec.md:31-32`; `research/sherlock-t20-batched-20261114/RESULTS.md:112-118` (no measured entry needs a chunk: largest 474 of 1,015 words). Check: Quint model of fix 4 on `t20-batched`; MISSING a test of `sherlock_plan` at TTL 14,400 s (openvm `rekey-chips@dcf1b4c:crates/sherlock-lfr/examples/sherlock_plan.rs`).

### Requirement: BAT-006 Settlement intent TTL
BAT-006: When the settle runner builds an intent on Preview with no TTL given by the user, the settle runner SHALL set the intent TTL to 3,600 s.

Verification: T17 (a) through `t17.sh` (exists). MISSING: the runner default and a test of a direct start; exists in part.

#### Scenario: The runner default
- **GIVEN** the ledger runner started directly, with `T10A_TTL_S` unset
- **WHEN** an intent is built on Preview
- **THEN** the TTL is 3,600 s; today it is 14,400 s

#### Scenario: Status of BAT-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: ANC-012 (default intent TTL 3,600 s; below 1,755 s no chunk); SCL-007 bounds an explicit value. Decision D-074 (OWNER 2026-11-15). Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/t17.sh:19` sets 3,600 s only through `t17.sh`; runner default `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:192` is 14,400 s (ruling O6, LOOP.md:75); Preview `global_ttl` 1,209,600 s (`RUNBOOK.md:73`). Check: T17 (a) through `t17.sh` (exists). MISSING: the runner default and a test of a direct start.

### Requirement: BAT-007 Membership data for each submitter
BAT-007: When an epoch closes, the epoch operator SHALL give the submitter of each job the slot index and the 20 sibling words of the job's claim.

Verification: `t20-ledger-test.out` (7 tests of fix 2); Quint `claimsRoot.qnt`. No operator code: MISSING a run; exists in part.

#### Scenario: A consumer uses a claim with no ledger write
- **GIVEN** an accepted `claims_root` and one job's slot and path
- **WHEN** the consumer contract calls `useRunClaim`
- **THEN** the consumer proves the path, reads `rootAccepted` and writes only its own nullifier

#### Scenario: Status of BAT-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: LDG-004, LDG-005, CRR-007. Evidence: `t20-batched@5dc1be2ee:research/sherlock-t20-batched-20261114/RESULTS.md:66,72,87`. Check: `t20-ledger-test.out` (7 tests of fix 2); Quint `claimsRoot.qnt`. No operator code: MISSING a run.

### Requirement: BAT-008 The leaf list is published off chain
BAT-008: When an epoch closes, the epoch operator SHALL publish the leaf list of the epoch off chain at 32 bytes for each claim.

Verification: Quint `claimsRoot.qnt` witness "a withheld list". MISSING a run; exists in part.

#### Scenario: A withheld list admits no non-member
- **GIVEN** an operator that withholds the list
- **WHEN** a consumer offers a claim that is not a member
- **THEN** the path check fails and the claim is refused

#### Scenario: Status of BAT-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. Evidence: `t20-batched@5dc1be2ee:research/sherlock-t20-batched-20261114/RESULTS.md:87,91`. Check: Quint `claimsRoot.qnt` witness "a withheld list". MISSING a run.

### Requirement: BAT-009 Collections wait for the L collection form
BAT-009: While the universal key set holds no key for the `L` collection form, the settle runner SHALL refuse to submit an `acceptCollection` call.

Verification: MISSING. GAP owner: slice B (GPH), row T8c; to build.

#### Scenario: The prototype settles by acceptRun
- **GIVEN** the T25 key set without the `L` collection form
- **WHEN** an operator asks for a collection submit
- **THEN** the runner refuses and names the missing form

#### Scenario: Status of BAT-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. Evidence: `t20-batched@5dc1be2ee:research/sherlock-t20-batched-20261114/RESULTS.md:70` ("no code on this branch; it belongs to T8b L over SET, which is not built"); LPC-001 to LPC-003 (row T8c). Check: MISSING. GAP owner: slice B (GPH), row T8c.

### Requirement: BAT-010 An epoch closes before its era ends
BAT-010: While an era end `end_eff` is announced, when the time reaches `end_eff` minus the collection proof time and the inclusion time, the epoch operator SHALL close the open epoch.

Verification: MISSING (no operator code, not in a Quint model); to build.

#### Scenario: An epoch near the boundary
- **GIVEN** an epoch opened 600 s before that limit
- **WHEN** the limit is reached
- **THEN** the epoch closes early and its root lands before `end_eff`

#### Scenario: Status of BAT-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: ERT-009, UC-9 (governance); BAT-001 stays the upper bound. The two budgets are parameters; 175.5 s warm is the DERIVED collection proof time. Under decision D-077 (OWNER 2026-11-15). Evidence: `t22-governance@fe7207819:research/sherlock-t22-governance-20261114/OPEN-QUESTIONS.md:70,86`. Check: MISSING (no operator code, not in a Quint model).

### Requirement: BAT-011 No run starts that cannot land in its era
BAT-011: If the proving time plus the inclusion time of a new run ends after the announced `end_eff` of the accepting era, then the settle runner SHALL refuse to start the run for that era.

Verification: MISSING (no code; `eraTable.qnt` has the lazy end, not the prover rule); to build.

#### Scenario: An intent that would outlive the era
- **GIVEN** `end_eff` in 500 s and a proving time of 757 s
- **WHEN** a user starts a run
- **THEN** the runner refuses and names the next era

#### Scenario: Status of BAT-011
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: ERT-004, GOV-006 (governance); the TTL of an admitted intent ends before `end_eff`. Evidence: `t22-governance@fe7207819:research/sherlock-t22-governance-20261114/OPEN-QUESTIONS.md:70`. Check: MISSING (no code; `eraTable.qnt` has the lazy end, not the prover rule).

### Requirement: BAT-012 Collection settlement is shown or listed as open
BAT-012: If the T15 freeze record holds no `acceptCollection` receipt on the T25 key set, then the T15 freeze record SHALL list batched collection settlement as open.

Verification: MISSING (the T15 record does not exist). GAP owner: slice B (GPH), row T8c; to build.

#### Scenario: The prototype settles by acceptRun only
- **GIVEN** a T23 run with 145 `acceptRun` receipts and no collection receipt
- **WHEN** the freeze record is written
- **THEN** the record lists the `L` collection form, the claims tree proof and the seventh DIGEST tag `SHK_FCLAIM` as open work of row T8c

#### Scenario: Status of BAT-012
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: LDG-003 to LDG-007 as amended on `t20-batched`; BAT-009; LPC-001 to LPC-003, LPC-014 (completion). Evidence: `t20-batched@5dc1be2ee:research/sherlock-t20-batched-20261114/RESULTS.md:70,97` (no circuit code; the new tag is a wrap key constant that rides the T8b rekey). Check: MISSING (the T15 record does not exist). GAP owner: slice B (GPH), row T8c.
