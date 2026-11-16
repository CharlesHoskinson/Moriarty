## ADDED Requirements

### Requirement: PRT-001 One resident service per card
PRT-001: The Sherlock prover SHALL run as one resident service per card in a plain process selected by `CUDA_VISIBLE_DEVICES`.

Source: FD-40 (A-06, A-18). Label: removes about 167 s of the 301.24 s warm call, DERIVED (FD section 6). Hard rule: no GPU setting or power-limit change.

#### Scenario: Card setup
- **WHEN** the service starts on a card
- **THEN** no MIG, MPS, clock or power setting is changed

### Requirement: PRT-002 One trust domain
PRT-002: The resident service SHALL run its STARK worker and its wrap worker in one trust domain.

Source: FD-40.

#### Scenario: Worker placement
- **WHEN** the service starts
- **THEN** both workers run under the same user on the same box

### Requirement: PRT-003 Deployment blinder
PRT-003: The resident service SHALL draw `r_P` once per deployment and keep it across retries, cancellations and restarts.

Source: FD-42 (A-08, K-8). Gate: E-ZK.

#### Scenario: Restart
- **WHEN** the service restarts
- **THEN** `r_P` is unchanged

### Requirement: PRT-004 Run salt
PRT-004: When a run starts, the resident service SHALL draw a fresh run salt and keep it for that run across retries.

Source: FD-42.

#### Scenario: Retried run
- **WHEN** a run is retried after a cancellation
- **THEN** it keeps its salt and no other run uses it

### Requirement: PRT-005 Wrap blinders
PRT-005: When a wrap proof starts, the resident service SHALL draw fresh wrap blinders used by that proof only.

Source: FD-42.

#### Scenario: Slot reuse
- **WHEN** E-ZK reruns the T11 slot-reuse check on the GPU path
- **THEN** no blinder is used by two proofs

### Requirement: PRT-006 No per-run state between proofs
PRT-006: The resident service SHALL hold no per-run state between proofs.

Source: FD-42.

#### Scenario: Seed override
- **WHEN** a production build receives a seed override
- **THEN** it refuses the override (E-ZK)

### Requirement: PRT-007 One pipeline per card
PRT-007: The resident service SHALL run one pipeline per card.

Source: FD-45 (A-19).

#### Scenario: Second pipeline without E-S7
- **WHEN** E-S7 has not passed
- **THEN** each card runs one pipeline

### Requirement: PRT-008 Two pipelines after E-S7
PRT-008: Where E-S7 has passed, the resident service SHALL run two pipelines per card.

Source: FD-45. Pass bar: more than 1.5 times the calls per hour of one pipeline.

#### Scenario: E-S7 passes
- **WHEN** two pipelines give more than 1.5 times the calls per hour on odin
- **THEN** the service config admits a second pipeline

### Requirement: PRT-009 Case A
PRT-009: The resident service SHALL keep every STARK proof, trace and retained root proof on the user's own boxes.

Source: FD-48 (A-22). Hard rules: Case A, ZK end to end. Cross-owner checkpoints are OQ-2 (`OPEN (owner)`).

#### Scenario: Export
- **WHEN** a client asks the service for its outputs
- **THEN** only the wrap proof and the call vector leave the box
