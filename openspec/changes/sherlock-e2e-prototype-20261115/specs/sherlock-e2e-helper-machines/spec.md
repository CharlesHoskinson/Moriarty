## ADDED Requirements

### Requirement: HLP-001 muninn checks every box wrap
HLP-001: When a box returns a wrap proof, the muninn ledger check SHALL verify the proof with the ledger 9.1.0.1 verifier before the result is kept.

Verification: the w3b record (exists); per call in row T23 (OPEN); exists in part.

#### Scenario: A huginn wrap
- **GIVEN** 24 wrap proofs made on huginn
- **WHEN** muninn checks them
- **THEN** 24 of 24 are Ok and the result reads KEEP

#### Scenario: Status of HLP-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: LVR-001, LVR-004; STL-002 scenario. The result is the published type `WrapVerdict` that slice B consumes. Evidence: loop rule "every wrap proof made on a box is checked on muninn with the ledger 9.1.0.1 verifier before a KEEP" (LOOP.md:96); llm-wiki `wiki/log.md:4108`. Check: the w3b record (exists); per call in row T23 (OPEN).

### Requirement: HLP-002 A refused box wrap is dropped
HLP-002: If the muninn ledger check refuses a wrap proof of a box, then the settle runner SHALL submit no call that carries the proof.

Verification: MISSING a negative run. GAP owner: row T23 driver; to build.

#### Scenario: A box wrap that does not verify
- **GIVEN** a wrap proof of odin that ledger 9.1.0.1 refuses
- **WHEN** muninn checks the proof
- **THEN** the result is dropped and the call is proved again

#### Scenario: Status of HLP-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: LVR-005, LVR-007; HLP-008 for a transfer or identity mismatch. Evidence: loop rule (LOOP.md:96). Check: MISSING a negative run. GAP owner: row T23 driver.

### Requirement: HLP-003 The argument bundle stays on the proving box
HLP-003: When a box finishes the proof of a call, the box SHALL run the argument builder and return to muninn only the 4 public words `c_p`, `input_pub`, `outcome_pub` and `claim_pub` with the wrap output.

Verification: MISSING (`t23-prove-result/1` has no `bundle` path yet, MAP.md:85); to build.

#### Scenario: muninn compares the four words
- **GIVEN** a `t23-prove-result/1` from odin
- **WHEN** muninn runs `check_prove_result`
- **THEN** the four words equal the recomputed lanes and no private part is in the result

#### Scenario: Status of HLP-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: SLC-006, SLC-009; ZK-001 as fixed on spec-audit. Evidence: MAP.md:51,85,90 (branch `t23-args`, staged, not committed). Check: MISSING (`t23-prove-result/1` has no `bundle` path yet, MAP.md:85).

### Requirement: HLP-004 Private parts never leave the box
HLP-004: The proving box SHALL keep `prog_d`, the `r_P` reference, the run salt and the call tape in box-private state.

Verification: MISSING (no scan of box result files); to build.

#### Scenario: The result file is scanned
- **GIVEN** a result file that a box sends to muninn
- **WHEN** the privacy scan runs on the file
- **THEN** no `prog_d`, `r_P`, salt or tape word is in the file

#### Scenario: Status of HLP-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: ZK-001, ZK-012, ZK-013, PRD-003; OBS-003 for the scan. Evidence: MAP.md:74,85 (branch `t23-args`, staged). Check: MISSING (no scan of box result files).

### Requirement: HLP-005 Case A only
HLP-005: The Sherlock SDK SHALL provide 0 commands and 0 options that send a witness, a STARK proof, a prover checkpoint or a shard plan to a host that the user does not own.

Verification: MISSING (ZK-018 has no configuration test); to build.

#### Scenario: An outsourced prover is asked for
- **GIVEN** a configuration that names a host of another owner
- **WHEN** the prover reads the configuration
- **THEN** the prover refuses the configuration

#### Scenario: Status of HLP-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: ZK-018 (sherlock-completion-20261115), ZK-001, ZK-009, PRD-002 (a helper-made slot is refused). Owner 2026-11-15: "we will start with case A and a future design will allow for outsources components using the Veritee model from the MPC Research thread". Evidence: `~/Projects/Moriarty/llm-wiki/wiki/log.md:4111-4113`. Check: MISSING (ZK-018 has no configuration test).

### Requirement: HLP-006 Case B needs its own change
HLP-006: If a design proposes proving on a host that the user does not own, then the design SHALL be a new OpenSpec change on the Veritee seald Sessions model.

Verification: MISSING until T15; then a review of the freeze record (LPC-014); to build.

#### Scenario: A helper feature request
- **GIVEN** a request to outsource the wrap MSM
- **WHEN** the lead triages the request
- **THEN** the request goes to a new change and no requirement of this change is edited

#### Scenario: Status of HLP-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: ZK-009 (no remote unmasked MSM, NTT, quotient or Merkle hash). Evidence: `~/Projects/Moriarty/llm-wiki/wiki/log.md:4113`. Check: review at T15 (LPC-014); no automated check.

### Requirement: HLP-007 The wallet is not on a proving box
HLP-007: If a job asks a box that holds no wallet directory to sign or submit a transaction, then the box SHALL refuse the job.

Verification: MISSING (a rule in BOXES.md; no check); to build.

#### Scenario: odin is asked to settle
- **GIVEN** the factory rehearsal and a job for odin that names the ledger runner in call mode
- **WHEN** the box agent reads the job
- **THEN** the agent refuses and posts the refusal on its bus

#### Scenario: Status of HLP-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: WAL-007. Evidence: Finish plan box table (LOOP.md:58, "wallet stays here"). Check: MISSING (a rule in BOXES.md; no check).

### Requirement: HLP-008 A box result is bound to its job
HLP-008: If the SHA-256 of a received box file differs from the box report or the public call vector differs from the call vector of the requested job, then the muninn ledger check SHALL drop the result before the proof is verified.

Verification: MISSING: a corrupted-file test and a test with a valid proof of another job; to build.

#### Scenario: A valid proof of another job
- **GIVEN** a wrap output of job 12 returned for job 13
- **WHEN** muninn compares the call vector with job 13
- **THEN** lanes 20, 34 and 39 differ and the result is dropped

#### Scenario: Status of HLP-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: HLP-001, HLP-003, SLC-009; `BoxResult {path, sha256, box}` of slice B (BXF); the key identity is checked by WRP-008. Evidence: loop rule "files move only through muninn with path + sha256" (LOOP.md:96); `check_prove_result` compares the four words (MAP.md:51). Check: MISSING: a corrupted-file test and a test with a valid proof of another job.
