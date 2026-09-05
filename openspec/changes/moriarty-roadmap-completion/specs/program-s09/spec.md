# S09: Proof-of-intent vertical slice

## ADDED Requirements

Status: specified-only completion contract.
The controlling XML remains authoritative.
Existing S01/S02 contracts remain unchanged.

### Requirement: S09-EARS-01 package response

EARS pattern: event-driven.

WHEN S09 execution starts, the Moriarty workstream SHALL produce the required outputs listed below.

Required outputs and behavior, from XML v1.3:

> Link mechanized semantics, translation evidence, real proof verification, plan verification, signing, and observed settlement for one application.

#### Scenario: S09 required delivery

- **WHEN** the sprint submits its outputs
- **THEN** the sprint validator SHALL check every listed output and behavior.
- **AND** its report SHALL identify exact artifacts and acceptance evidence.

### Requirement: S09-EARS-02 incomplete evidence

EARS pattern: unwanted-behavior.

IF a required S09 result is missing or inconclusive, THEN its validator SHALL refuse sprint completion.

#### Scenario: S09 omitted required output

- **WHEN** one listed output or required result is absent
- **THEN** the validator SHALL retain an open sprint status.
- **AND** missing work SHALL NOT establish architectural infeasibility.

### Requirement: S09-EARS-03 review boundary

EARS pattern: state-driven.

WHILE mandatory S09 review remains incomplete, the integration gate SHALL refuse sprint acceptance.

#### Scenario: S09 missing Council member

- **WHEN** one required identity-bound Council verdict is absent
- **THEN** the sprint acceptance record SHALL remain open.

## Package execution fields

Dependencies follow the controlling XML sequence and declared artifact dependencies.
The lower-numbered sprint alone does not prove an input is ready.
Inputs: controlling XML, adopted scope, prior accepted manifests, and applicable source locks.
Planned outputs: `evidence/program-roadmap/s09/manifest.json` and `validation.json`.
These paths are future acceptance artifacts, not generated results.
Failure outcome: `open-s09-acceptance`.
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
