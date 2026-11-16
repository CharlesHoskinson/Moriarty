## ADDED Requirements

### Requirement: STC-001 Resident synced wallet
STC-001: The resident service SHALL hold a synced wallet and a prebuilt transaction template per entry.

Source: FD-47 (C-17 (a), chair decision, upheld). Label: 547.7 s of the Preview call was neither proving nor inclusion, DERIVED (K-13, unsplit, includes test overhead). Dissent: astra-1, astra-2, astra-3 keep the client out of the prover design.

#### Scenario: Call start
- **WHEN** a call starts
- **THEN** no wallet sync runs on the call path

### Requirement: STC-002 Transaction built before the tail
STC-002: When a call is proved, the resident service SHALL build the transaction before the tail starts.

Source: FD-47.

#### Scenario: Tail start
- **WHEN** `L` starts
- **THEN** the transaction body waits only for the wrap proof

### Requirement: STC-003 Client stages reported apart
STC-003: The Sherlock performance report SHALL time wallet sync and build and native check and submit and inclusion and readback apart from proving.

Source: FD-47 (C-17 (b) reporting). No client latency target is stated until E-CLIENT (pass bar: build to submit at or under 30 s, ESTIMATE bar).

#### Scenario: E-CLIENT
- **WHEN** E-CLIENT runs one call with a resident wallet
- **THEN** each client stage has its own MEASURED time

### Requirement: STC-004 Tail-only rebind for S2
STC-004: When an S2 transaction is rebuilt before inclusion, the resident service SHALL re-prove only `L` and `L_Fr` and the wrap from the retained root proof.

Source: FD-37 (C-18 (a) for S2). Evidence: lane 0 is outside every record and lane 1 outside the fallback record (`fp3.rs:11`, `:21-24`). No saving is credited until E-REBIND passes.

#### Scenario: Rebuilt acceptRun
- **WHEN** E-REBIND rebuilds a real `acceptRun` transaction
- **THEN** only the `L`, `L_Fr` and wrap proofs change and the rebuilt transaction settles

### Requirement: STC-005 Root proof retained until inclusion
STC-005: While an S2 or S3 transaction is not included, the resident service SHALL keep its root proof on the user's box.

Source: FD-37, FD-48.

#### Scenario: Inclusion
- **WHEN** the transaction is included
- **THEN** the retained root proof is released

### Requirement: STC-006 S3 rebind after E-REBIND
STC-006: Where E-REBIND has shown that a rebuilt collection transaction leaves the collector record unchanged, the resident service SHALL re-prove only `L` and `L_Fr` and the wrap for a rebuilt S3 transaction.

Source: FD-37 after the tie audit (C-18 S3 part reversed to (b); recorded dissent of the chair).

#### Scenario: Why S3 waits for E-REBIND
- **WHEN** the S3 rule is reviewed
- **THEN** it reads: the collector's own record carries lane 1 `comm` (`fp3.rs:8`) and a `reg_root` tape read, so a rebuild can change a proof below `L`; tail-only rebind is not claimed for an S1 Compact record

#### Scenario: Collector record changes
- **WHEN** a rebuild changes `comm` or follows a `reg_root` change
- **THEN** the collector run is re-proved before the tail
