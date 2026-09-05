# S10: Complete developer interface

## ADDED Requirements

Status: specified-only completion contract.
The controlling XML remains authoritative.
Existing S01/S02 contracts remain unchanged.

### Requirement: S10-EARS-01 package response

EARS pattern: event-driven.

WHEN S10 execution starts, the Moriarty workstream SHALL produce the required outputs listed below.

Required outputs and behavior, from XML v1.3:

> Reconcile and implement the minimum safety spine. Validate all API and data contracts across supported environments.

#### Scenario: S10 required delivery

- **WHEN** the sprint submits its outputs
- **THEN** the sprint validator SHALL check every listed output and behavior.
- **AND** its report SHALL identify exact artifacts and acceptance evidence.

### Requirement: S10-EARS-02 incomplete evidence

EARS pattern: unwanted-behavior.

IF a required S10 result is missing or inconclusive, THEN its validator SHALL refuse sprint completion.

#### Scenario: S10 omitted required output

- **WHEN** one listed output or required result is absent
- **THEN** the validator SHALL retain an open sprint status.
- **AND** missing work SHALL NOT establish architectural infeasibility.

### Requirement: S10-EARS-03 review boundary

EARS pattern: state-driven.

WHILE mandatory S10 review remains incomplete, the integration gate SHALL refuse sprint acceptance.

#### Scenario: S10 missing Council member

- **WHEN** one required identity-bound Council verdict is absent
- **THEN** the sprint acceptance record SHALL remain open.

## Package execution fields

Dependencies follow the controlling XML sequence and declared artifact dependencies.
The lower-numbered sprint alone does not prove an input is ready.
Inputs: controlling XML, adopted scope, prior accepted manifests, and applicable source locks.
Planned outputs: `evidence/program-roadmap/s10/manifest.json` and `validation.json`.
These paths are future acceptance artifacts, not generated results.
Failure outcome: `open-s10-acceptance`.
Rollback: preserve prior accepted artifacts and use reviewed revert commits.
Semantic scope: apply only explicitly accepted semantic motions.

## Tasks

- [ ] Pin exact inputs before implementation.
- [ ] Decompose the required outputs into concrete implementation tasks.
- [ ] Define package-specific tests and negative controls.
- [ ] Implement the required outputs through the common architecture.
- [ ] Recompute the package manifest and acceptance evidence.
- [ ] Obtain independent and Council review.
- [ ] Integrate only the accepted scope.

## Specification check

```bash
openspec validate moriarty-roadmap-completion --strict --no-interactive
```

This checks documentation structure, not sprint implementation.
