# A4 completion requirements

## ADDED Requirements

These are prospective EARS obligations.
The global contract in `openspec/WORK-PACKAGES-EARS.md` also applies.

### Requirement: A4-R01: inventory

EARS pattern: event-driven.

WHEN integrated traces are exported, the exporter SHALL preserve every mandatory scenario and signing profile.

#### Scenario: A4-R01: inventory positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A4-R01: inventory failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A4-R02: fields

EARS pattern: event-driven.

WHEN an agreement result is checked, the independent checker SHALL compare every frozen Core result field and ordered effect.

#### Scenario: A4-R02: fields positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A4-R02: fields failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A4-R03: authority

EARS pattern: event-driven.

WHEN cancellation or consumption is checked, the authority checker SHALL validate the retained policy, attempt, parent, and nonce history.

#### Scenario: A4-R03: authority positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A4-R03: authority failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A4-R04: omission

EARS pattern: unwanted-behavior.

IF an expected record is absent or duplicated, THEN the inventory checker SHALL reject the export.

#### Scenario: A4-R04: omission positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A4-R04: omission failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A4-R05: mutation

EARS pattern: unwanted-behavior.

IF a security-critical mutant survives, THEN the acceptance validator SHALL block the package pending a supported disposition.

#### Scenario: A4-R05: mutation positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A4-R05: mutation failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.
