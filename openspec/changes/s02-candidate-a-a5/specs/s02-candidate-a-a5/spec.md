# A5 completion requirements

## ADDED Requirements

These are prospective EARS obligations.
The global contract in `openspec/WORK-PACKAGES-EARS.md` also applies.

### Requirement: A5-R01: factoring

EARS pattern: event-driven.

WHEN a model is factored, the workstream SHALL preserve the original semantics and compare the complete existing corpus.

#### Scenario: A5-R01: factoring positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A5-R01: factoring failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A5-R02: bounds

EARS pattern: event-driven.

WHEN model checking starts, its manifest SHALL declare domains, depth, properties, fairness, tool versions, and resource limits.

#### Scenario: A5-R02: bounds positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A5-R02: bounds failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A5-R03: completion

EARS pattern: event-driven.

WHEN a bounded verification claim is submitted, the validator SHALL require a terminal checker result for each claimed property.

#### Scenario: A5-R03: completion positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A5-R03: completion failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A5-R04: resource-failure

EARS pattern: unwanted-behavior.

IF checking terminates before state exploration, THEN the workstream SHALL report incomplete verification without claiming an architecture counterexample.

#### Scenario: A5-R04: resource-failure positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A5-R04: resource-failure failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A5-R05: no-weakening

EARS pattern: unwanted-behavior.

IF factoring removes a mandatory case or property, THEN the reviewer SHALL reject the factoring proposal.

#### Scenario: A5-R05: no-weakening positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A5-R05: no-weakening failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.
