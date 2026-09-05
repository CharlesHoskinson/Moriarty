# A6 completion requirements

## ADDED Requirements

These are prospective EARS obligations.
The global contract in `openspec/WORK-PACKAGES-EARS.md` also applies.

### Requirement: A6-R01: dossier

EARS pattern: event-driven.

WHEN Candidate A acceptance is requested, its validator SHALL recompute the complete obligation-to-evidence matrix.

#### Scenario: A6-R01: dossier positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A6-R01: dossier failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A6-R02: council

EARS pattern: event-driven.

WHEN Council approval is claimed, the gate SHALL require substantive identity-bound verdicts from all three requested models.

#### Scenario: A6-R02: council positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A6-R02: council failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A6-R03: dissent

EARS pattern: unwanted-behavior.

IF a material review finding remains unresolved, THEN the acceptance gate SHALL refuse approval.

#### Scenario: A6-R03: dissent positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A6-R03: dissent failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A6-R04: unavailable

EARS pattern: unwanted-behavior.

IF released Council transport is unavailable, THEN the workstream SHALL retain the review obligation without reopening Foreman development.

#### Scenario: A6-R04: unavailable positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A6-R04: unavailable failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.
