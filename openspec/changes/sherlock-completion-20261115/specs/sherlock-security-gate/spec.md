## ADDED Requirements

### Requirement: SQG-014 G3 legs
SQG-014: When G3 runs, the security gate SHALL compare the transcript vectors of the CPU prover, the GPU prover, the stark-backend verifier, `L` inside `L_Fr` and the wrap on the rekeyed keys.

#### Scenario: G3 legs
- **WHEN** one leg gives another challenge
- **THEN** G3 fails and names the leg

#### Scenario: Verification and status of SQG-014
- **WHEN** the status is reported
- **THEN** the source is row T12 (LOOP.md line 34), the verification is RUNBOOK G3 (`research/sherlock-tested-design-20261113/t12/RUNBOOK.md` on `t12-gate` 90f9b4df9), and the status on 2026-11-15 is: OPEN: the three legs are not built.

### Requirement: SQG-015 G4, G1, G2, FP-8 and the ledger on the new keys
SQG-015: When the rekey completes, the security gate SHALL run G4 with the `L_Fr` part, G1, G2, FP-8 and the ledger 9.1.0.1 check on the new keys.

#### Scenario: G4, G1, G2, FP-8 and the ledger on the new keys
- **WHEN** a gate fails twice
- **THEN** row T12 stops (AFK rule)

#### Scenario: Verification and status of SQG-015
- **WHEN** the status is reported
- **THEN** the source is row T12, the verification is RUNBOOK steps with logs, rows G1 to G4 of ARCHITECTURE 1.9 read passed, I-27, I-28, I-30, I-34, I-37 closed, and the status on 2026-11-15 is: OPEN.

### Requirement: SQG-016 No pass on an unverified step
SQG-016: If a RUNBOOK step is UNVERIFIED, then the security gate SHALL report the gate as not passed.

#### Scenario: No pass on an unverified step
- **WHEN** a step has no log
- **THEN** the gate is not passed

#### Scenario: Verification and status of SQG-016
- **WHEN** the status is reported
- **THEN** the source is LEAD PICK (this audit), wiki log 08:47 (two steps UNVERIFIED), the verification is RUNBOOK review, and the status on 2026-11-15 is: OPEN.
