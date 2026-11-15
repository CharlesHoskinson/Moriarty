## ADDED Requirements

### Requirement: LPC-001 GP hooks of T8b
LPC-001: When row T8b closes, the tested-design loop SHALL record I-13 and the four GP hook rows as FROZEN with real-proof evidence or as FREEZE-PENDING with a measured reason.

#### Scenario: GP hooks of T8b
- **WHEN** a hook is not built
- **THEN** its row reads FREEZE-PENDING with the reason

#### Scenario: Verification and status of LPC-001
- **WHEN** the status is reported
- **THEN** the source is row T8b, the verification is ARCHITECTURE 1.12 rows and `scripts/doctorwatson check` receipts, and the status on 2026-11-15 is: PARTIAL (openvm 89bee35, Moriarty b1ff56ba1).

### Requirement: LPC-002 STACKS and VALUE DIGEST in MERGE-WORK
LPC-002: When the STACKS AIR and VALUE DIGEST land in the inline kernel, the MERGE-WORK key SHALL have at most 5,400 columns.

#### Scenario: STACKS and VALUE DIGEST in MERGE-WORK
- **WHEN** the key has 5,417 columns
- **THEN** row T8c splits CORE again or raises the bound with a measurement

#### Scenario: Verification and status of LPC-002
- **WHEN** the status is reported
- **THEN** the source is owner ruling 2026-11-14 (5,400), lead condition in LOOP.md line 71, the verification is G1 column count of the MERGE-WORK key, and the status on 2026-11-15 is: MISSING from the checklist and from LOOP.md: row T8c is named in row T8b but has no row.

### Requirement: LPC-003 X8 in-circuit verifier and L over SET
LPC-003: When row T8c runs, the tested-design loop SHALL build the X8 in-circuit Compact verifier and the `L` check over a SET root on real proofs.

#### Scenario: X8 in-circuit verifier and L over SET
- **WHEN** a forged SET root is offered to `L`
- **THEN** `L` refuses it

#### Scenario: Verification and status of LPC-003
- **WHEN** the status is reported
- **THEN** the source is row T8b open items, the verification is real-proof tests with forged inputs refused, and the status on 2026-11-15 is: MISSING from the checklist.

### Requirement: LPC-004 Quint lanes for circuit changes
LPC-004: When a circuit change lands after openvm c4bd312, the tested-design loop SHALL run a Carbon Quint lane on it and file a lead verdict.

#### Scenario: Quint lanes for circuit changes
- **WHEN** a mutant survives
- **THEN** the lane reports it and the lead reruns it

#### Scenario: Verification and status of LPC-004
- **WHEN** the status is reported
- **THEN** the source is row T13, Carbon rule (LOOP.md line 81), the verification is `qa/gemini-quint/<lane>/verdict.md` per change, and the status on 2026-11-15 is: OPEN: batch 2 lanes and the merge of e5-wrap 70e728635 into tested are pending.

### Requirement: LPC-005 Final timing on the new keys
LPC-005: When the rekey completes, the timing runner SHALL measure warm and cold call time and peak memory on huginn and odin, each against its own baseline.

#### Scenario: Final timing on the new keys
- **WHEN** a box is loaded
- **THEN** the run record holds the load and the claim compares one box only

#### Scenario: Verification and status of LPC-005
- **WHEN** the status is reported
- **THEN** the source is rows T14, T14b, box rule (LOOP.md line 80), the verification is RESULTS with run records per box, wrap checked on muninn, and the status on 2026-11-15 is: OPEN: w3b KEEP (warm 12.89 s on huginn) on T10m keys.

### Requirement: LPC-006 Two run lengths under one plan
LPC-006: When `pos_plan_schedules` runs two run lengths under one `plan_d`, the node SHALL accept both runs.

#### Scenario: Two run lengths under one plan
- **WHEN** the longer run proves
- **THEN** it keeps its `prog_d` and the keys do not change

#### Scenario: Verification and status of LPC-006
- **WHEN** the status is reported
- **THEN** the source is row T16 (b), SPL-011, the verification is `pos_plan_schedules` with two run lengths, and the status on 2026-11-15 is: PARTIAL: two run lengths open.

### Requirement: LPC-007 Scale up and across hardware
LPC-007: When the rekey gives a leaf that proves more than 5 rules, the elastic demo SHALL prove one program at 1x, 2x, 4x and 8x run length under the same keys.

#### Scenario: Scale up and across hardware
- **WHEN** leaves from muninn and huginn are joined
- **THEN** one N1 accepts them and `claim_pub` is byte-equal

#### Scenario: Verification and status of LPC-007
- **WHEN** the status is reported
- **THEN** the source is row T16 (c) to (e), the verification is RESULTS-CDE MEASURED tables, one `claim_pub` per run, and the status on 2026-11-15 is: MISSING from the checklist: (c) BLOCKED on the rekey, (d) PARTIAL (muninn runs and the two-host join queued).

### Requirement: LPC-008 Preview prototype (b) and (c)
LPC-008: When T11 and T12 pass, the prototype runner SHALL run the honest call and the refusals on Preview with the prover on huginn and a second program.

#### Scenario: Preview prototype (b) and (c)
- **WHEN** a tampered proof is sent
- **THEN** the node refuses it

#### Scenario: Verification and status of LPC-008
- **WHEN** the status is reported
- **THEN** the source is row T17, the verification is Preview receipts and RUNBOOK.md, and the status on 2026-11-15 is: MISSING from the checklist: (b) waits on T10q, (c) on T11, T12 and a huginn prover hook.

### Requirement: LPC-009 Validium demo
LPC-009: When T17 (b) passes, the validium runner SHALL settle at least 2 batches on Preview and refuse an invalid batch and a stale root.

#### Scenario: Validium demo
- **WHEN** a batch uses a stale root
- **THEN** the L2 contract refuses it

#### Scenario: Verification and status of LPC-009
- **WHEN** the status is reported
- **THEN** the source is row T19, the verification is Preview receipts, equal proof size across batch counts, and the status on 2026-11-15 is: waits on T17 (b).

### Requirement: LPC-010 T20 merge
LPC-010: When T10q closes, the tested-design loop SHALL merge `t20-batched` 5dc1be2ee into the integration branch.

#### Scenario: T20 merge
- **WHEN** the merge lands
- **THEN** ANC-012 reads owner-accepted

#### Scenario: Verification and status of LPC-010
- **WHEN** the status is reported
- **THEN** the source is LOOP.md line 42 (merge after T10q), the verification is `git merge-base --is-ancestor 5dc1be2ee tested`, and the status on 2026-11-15 is: MISSING from the checklist. The ANC-012 text on `t20-batched` still says "LEAD pick for T15 confirmation" for `K_MAX`, which the owner accepted on 2026-11-14.

### Requirement: LPC-011 Prover GUI beyond the stub
LPC-011: When the five build lanes merge, the prover GUI SHALL run live against the T14b daemon and in replay mode.

#### Scenario: Prover GUI beyond the stub
- **WHEN** the daemon is down
- **THEN** the GUI runs in replay mode

#### Scenario: Verification and status of LPC-011
- **WHEN** the status is reported
- **THEN** the source is row T21, PGUI requirements on branch `prover-gui`, the verification is the PGUI Playwright harness, and the status on 2026-11-15 is: running.

### Requirement: LPC-012 T22 Carbon verdict
LPC-012: When the T22 governance change is reviewed, the tested-design loop SHALL file a Carbon Quint lead verdict for `sherlock-governance`.

#### Scenario: T22 Carbon verdict
- **WHEN** a confirmed defect
- **THEN** row T22 reopens

#### Scenario: Verification and status of LPC-012
- **WHEN** the status is reported
- **THEN** the source is row T22, Carbon rule, the verification is `qa/gemini-quint/<t22 lane>/verdict.md`, and the status on 2026-11-15 is: OPEN: no T22 verdict on any branch.

### Requirement: LPC-013 Mycroft KRS gates
LPC-013: When the T18 (d) deliverables are committed, the Mycroft factory SHALL pass `gates.sh` and `lake build --wfail`.

#### Scenario: Mycroft KRS gates
- **WHEN** a mutant survives
- **THEN** the factory records it as argued or kills it

#### Scenario: Verification and status of LPC-013
- **WHEN** the status is reported
- **THEN** the source is row T18 (d), the verification is `~/.cache/t18d/gates.log`, KRS emit, coverage, mutations, compare, and the status on 2026-11-15 is: PARTIAL: gates PASS MEASURED 06:30, the deliverables are staged and not committed in `mycroft-factory`.

### Requirement: LPC-014 Interface freeze
LPC-014: When every row is DONE with a lead verdict or waits on the owner, the tested-design loop SHALL present the interface freeze to the owner.

#### Scenario: Interface freeze
- **WHEN** a row has no lead verdict
- **THEN** it does not count DONE

#### Scenario: Verification and status of LPC-014
- **WHEN** the status is reported
- **THEN** the source is row T15, the verification is owner sign-off, and the status on 2026-11-15 is: waits.

### Requirement: LPC-015 Pushes
LPC-015: If a push targets a branch other than `t10q` before T15, then the tested-design loop SHALL hold the push.

#### Scenario: Pushes
- **WHEN** FP-3 passes
- **THEN** the t10q branch push is allowed (owner order)

#### Scenario: Verification and status of LPC-015
- **WHEN** the status is reported
- **THEN** the source is AFK rule (LOOP.md line 79), O11 lead decision (line 73), the verification is `git log origin/<branch>` unchanged, and the status on 2026-11-15 is: t10q push held on FP-3.

### Requirement: LPC-016 Lead verdict for each DONE row
LPC-016: If a row reaches DONE without a lead verdict in `qa/gemini-quint`, then the tested-design loop SHALL count the row as not DONE for T15.

#### Scenario: Lead verdict for each DONE row
- **WHEN** T17 (a) is DONE
- **THEN** it needs a verdict before T15

#### Scenario: Verification and status of LPC-016
- **WHEN** the status is reported
- **THEN** the source is Carbon rule (LOOP.md line 81), the verification is verdict file per row, and the status on 2026-11-15 is: MISSING verdicts: T10q-ntfix, T11, T12 part 1, T17 (a), T18 (c) and (d), T22, T23.

### Requirement: LPC-017 Design documents follow the rulings
LPC-017: When an owner ruling changes a formula, the tested-design loop SHALL update FINAL-DESIGN.md and ARCHITECTURE.md to the ruled form.

#### Scenario: Design documents follow the rulings
- **WHEN** the grep runs
- **THEN** no lock value uses `DOM_LOCK`

#### Scenario: Verification and status of LPC-017
- **WHEN** the status is reported
- **THEN** the source is owner ruling 2026-11-14 ("x_j lock form is canon (replaces P2(DOM_LOCK) in ANC-002 and FINAL-DESIGN 1.6)"), the verification is grep of `DOM_LOCK` in the two documents, and the status on 2026-11-15 is: GAP: `research/sherlock-processor-final-20261111/FINAL-DESIGN.md` and `research/sherlock-final-design-20261111/ARCHITECTURE.md` keep the `P2(DOM_LOCK)` form.

### Requirement: LPC-018 T3 parent constraints fire
LPC-018: If a parent constraint of T3 never fires in a test, then the Carbon lane t3-airs SHALL add a forged input that fires it.

#### Scenario: T3 parent constraints fire
- **WHEN** a forged input runs
- **THEN** the named error fires

#### Scenario: Verification and status of LPC-018
- **WHEN** the status is reported
- **THEN** the source is row T3 open items, the verification is tests for `G1Error::Optional`, `ShapeError::Lengths` and the `from_wrapin` assert, and the status on 2026-11-15 is: MISSING from the checklist. sdk-node fails 3 of 144 without `SHERLOCK_SDK_ROOT`.

### Requirement: LPC-019 Mycroft open premises
LPC-019: When the Mycroft design closes, the Mycroft Lean library SHALL discharge premise S1 or keep it as a named hypothesis.

#### Scenario: Mycroft open premises
- **WHEN** S1 stays
- **THEN** the report names it

#### Scenario: Verification and status of LPC-019
- **WHEN** the status is reported
- **THEN** the source is row T18 (b) open items, the verification is `lake build --wfail`, SORRIES list, and the status on 2026-11-15 is: MISSING from the checklist: premise S1, tasks 3b.1 to 3b.3.

### Requirement: LPC-020 Rows named on the box queues
LPC-020: If a box queue names a row that LOOP.md does not list, then the lead SHALL add the row to LOOP.md.

#### Scenario: Rows named on the box queues
- **WHEN** the queue names T24
- **THEN** LOOP.md has a T24 row

#### Scenario: Verification and status of LPC-020
- **WHEN** the status is reported
- **THEN** the source is LEAD PICK (this audit), the verification is LOOP.md rows against `~/.cache/box-bus/all.log`, and the status on 2026-11-15 is: GAP: rows T24 and T25 are staged in the main working copy of LOOP.md (wiki 09:11), not committed.
