# A7 completion requirements

## ADDED Requirements

These are prospective EARS obligations.
The global contract in `openspec/WORK-PACKAGES-EARS.md` also applies.

### Requirement: A7-R01: commit

EARS pattern: event-driven.

WHEN a reviewed unit is committed, the commit SHALL contain only its explicitly owned source and evidence paths.

#### Scenario: A7-R01: commit positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A7-R01: commit failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A7-R02: merge

EARS pattern: event-driven.

WHEN integration is requested, the integration gate SHALL require every existing review prerequisite.

#### Scenario: A7-R02: merge positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A7-R02: merge failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A7-R03: remote

EARS pattern: unwanted-behavior.

IF no verified remote exists, THEN the workstream SHALL report local commits without claiming publication.

#### Scenario: A7-R03: remote positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A7-R03: remote failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A7-R04: scope

EARS pattern: event-driven.

WHEN Candidate A finishes, the roadmap SHALL preserve all outstanding S02 and full-program obligations.

#### Scenario: A7-R04: scope positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A7-R04: scope failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A7-R05: checkpoint

EARS pattern: event-driven.

WHEN the session ends, the workstream SHALL preserve typed recovery records and a consistent snapshot.

#### Scenario: A7-R05: checkpoint positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A7-R05: checkpoint failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.
