## ADDED Requirements

### Requirement: RKY-001 One rekey from one list
RKY-001: When the rekey runs, the rekey runner SHALL regenerate the `L`, `L_Fr` and wrap keys once from every row of `~/.cache/rekey-t11/LIST.md`.

#### Scenario: One rekey from one list
- **WHEN** a row is added after the keygen
- **THEN** a second keygen is needed and the record shows it

#### Scenario: Verification and status of RKY-001
- **WHEN** the status is reported
- **THEN** the source is single rekey list, owner-confirmed 2026-11-15, the verification is keygen record with the sha256 of LIST.md and the commit of every row, and the status on 2026-11-15 is: OPEN: waits on A1 to A7.

### Requirement: RKY-002 Every list row has a commit
RKY-002: If a row of the rekey list has no commit on its branch, then the rekey runner SHALL refuse to start.

#### Scenario: Every list row has a commit
- **WHEN** the T12-SLOT15 row is uncommitted
- **THEN** the rekey does not start

#### Scenario: Verification and status of RKY-002
- **WHEN** the status is reported
- **THEN** the source is LEAD PICK (this audit), the verification is a check script over LIST.md rows, and the status on 2026-11-15 is: GAP: FP-3, T12-SLOT15 and the range checks have no LIST.md row or no commit.

### Requirement: RKY-003 Wrap rows at k = 21
RKY-003: When the rekey completes, the rekey runner SHALL measure the wrap at `k = 21` with at most 1,782,573 rows.

#### Scenario: Wrap rows at k = 21
- **WHEN** the wrap needs more rows
- **THEN** the rekey stops and a lever list is written

#### Scenario: Verification and status of RKY-003
- **WHEN** the status is reported
- **THEN** the source is hard rule never k = 22, DG-002, the verification is wrap row count MEASURED on the new key (T10m: 1,685,845), and the status on 2026-11-15 is: OPEN.

### Requirement: RKY-004 Wrap VK size
RKY-004: When the rekey completes, the rekey runner SHALL measure a wrap verifying key of at most 50,000 bytes.

#### Scenario: Wrap VK size
- **WHEN** the VK is read by the ledger
- **THEN** the ledger 9.1.0.1 verifier parses it

#### Scenario: Verification and status of RKY-004
- **WHEN** the status is reported
- **THEN** the source is hard rule, DG-003, the verification is VK bytes MEASURED (T10m: 1,325), and the status on 2026-11-15 is: OPEN.

### Requirement: RKY-005 100-bit security on the new keys
RKY-005: When the rekey completes, the D3 calculator SHALL give every term at least 100 bits and every grindable term at least 101 bits on each new key.

#### Scenario: 100-bit security on the new keys
- **WHEN** a term is 100.9 bits and grindable
- **THEN** the gate fails

#### Scenario: Verification and status of RKY-005
- **WHEN** the status is reported
- **THEN** the source is hard rule proven 100-bit security, SQG-004, the verification is D3 on L, L_Fr and the R_SL leaf keys, and the status on 2026-11-15 is: OPEN.
