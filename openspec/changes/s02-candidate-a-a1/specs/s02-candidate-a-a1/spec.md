# A1 completion requirements

## ADDED Requirements

These are prospective EARS obligations.
The global contract in `openspec/WORK-PACKAGES-EARS.md` also applies.

### Requirement: A1-R01: concrete-plans

EARS pattern: event-driven.

WHEN lifecycle planning finishes, each plan SHALL define exact typed helpers, files, expected records, tests, and action dependencies.

#### Scenario: A1-R01: concrete-plans positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A1-R01: concrete-plans failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A1-R02: parent-signature

EARS pattern: event-driven.

WHEN cancellation follows a fill, the model SHALL create a fresh attempt under the existing signed parent policy.

#### Scenario: A1-R02: parent-signature positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A1-R02: parent-signature failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A1-R03: revision

EARS pattern: event-driven.

WHEN two fills commit, the parent state SHALL record revision2.

#### Scenario: A1-R03: revision positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A1-R03: revision failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A1-R04: recovery-authority

EARS pattern: event-driven.

WHEN recovery signing begins, the model SHALL bind a new nonce1 policy to the actual cancelled parent.

#### Scenario: A1-R04: recovery-authority positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A1-R04: recovery-authority failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.

### Requirement: A1-R05: draft-error

EARS pattern: unwanted-behavior.

IF a draft requires re-signing consumed nonce0, THEN the planner SHALL correct the draft without changing common authorization.

#### Scenario: A1-R05: draft-error positive acceptance

- **WHEN** the requirement's stated condition occurs
- **THEN** the recorded behavior SHALL satisfy the requirement above.
- **AND** the acceptance record SHALL identify the exact source and test evidence.

#### Scenario: A1-R05: draft-error failure control

- **WHEN** the observed behavior contradicts the requirement or its required evidence is absent
- **THEN** the package validator SHALL reject the acceptance claim.
- **AND** the failure record SHALL preserve the actual input and result.
