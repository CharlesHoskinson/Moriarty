# A0 completion requirements

## ADDED Requirements

These are prospective EARS obligations.
The global contract in `openspec/WORK-PACKAGES-EARS.md` also applies.

### Requirement: A0-R01: corrected-source

EARS pattern: event-driven.

WHEN boundary intake begins, the reviewer SHALL inspect the final-corrected-source closure and final committed source.

#### Scenario: A0-R01: corrected-source positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A0-R01: corrected-source failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A0-R02: runtime

EARS pattern: event-driven.

WHEN boundary verification runs, the reviewer SHALL execute the tests and inspect every profile and action witness.

#### Scenario: A0-R02: runtime positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A0-R02: runtime failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A0-R03: provenance

EARS pattern: unwanted-behavior.

IF a historical receipt lacks original source bytes, THEN the report SHALL identify the provenance gap.

#### Scenario: A0-R03: provenance positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A0-R03: provenance failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A0-R04: archive

EARS pattern: event-driven.

WHEN the author evidence is archived, the manifest SHALL bind each exact copied file to its original digest.

#### Scenario: A0-R04: archive positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A0-R04: archive failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.
