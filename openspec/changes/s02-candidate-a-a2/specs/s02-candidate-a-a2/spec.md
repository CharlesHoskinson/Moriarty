# A2 completion requirements

## ADDED Requirements

These are prospective EARS obligations.
The global contract in `openspec/WORK-PACKAGES-EARS.md` also applies.

### Requirement: A2-I01

EARS pattern: event-driven.

WHEN I01 executes under either signing profile, the A2 model SHALL produce the acceptance result below.

Acceptance result: Unsigned prefunded N4/Alice escrow10/time2; actual parent preparation and registration; no funds moved.

#### Scenario: A2-I01 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I01 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I01 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.

### Requirement: A2-I02

EARS pattern: event-driven.

WHEN I02 executes under either signing profile, the A2 model SHALL produce the acceptance result below.

Acceptance result: Actual fill1 chosen1 by Bob: N2, escrow5, Bob5, used slot1, paid5, allowance5, revision1.

#### Scenario: A2-I02 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I02 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I02 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.

### Requirement: A2-I03

EARS pattern: event-driven.

WHEN I03 executes under either signing profile, the A2 model SHALL produce the acceptance result below.

Acceptance result: Separate actual fill2 chosen1: N0, escrow0, Bob10, used slots1/2, paid10, allowance0, revision2; replay refused.

#### Scenario: A2-I03 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I03 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I03 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.

### Requirement: A2-I04

EARS pattern: event-driven.

WHEN I04 executes under either signing profile, the A2 model SHALL produce the acceptance result below.

Acceptance result: Initial fill/cancel both proposed and verified from identical current context; cancellation wins; fill loser is retained and rejected stale; escrow10 remains.

#### Scenario: A2-I04 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I04 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I04 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.

### Requirement: A2-I05

EARS pattern: event-driven.

WHEN I05 executes under either signing profile, the A2 model SHALL produce the acceptance result below.

Acceptance result: Same race, fill wins; initial cancellation loser is retained and rejected stale; escrow5 remains; second fill remains a distinct possible action.

#### Scenario: A2-I05 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I05 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I05 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.

### Requirement: A2-I06

EARS pattern: event-driven.

WHEN I06 executes under either signing profile, the A2 model SHALL produce the acceptance result below.

Acceptance result: After fill1, new FreshCancelAttempt uses current N2/parent facts and existing signed policy; cancellation yields revision2 without moving money or re-signing nonce0.

#### Scenario: A2-I06 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I06 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I06 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.

### Requirement: A2-I07

EARS pattern: event-driven.

WHEN I07 executes under either signing profile, the A2 model SHALL produce the acceptance result below.

Acceptance result: After cancellation-first, freshly prepare/sign nonce1, actual recover chosen1 by Alice refunds10; N0 and zero escrow; original cancelled parent/nonce0 unchanged.

#### Scenario: A2-I07 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I07 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I07 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.

### Requirement: A2-I08

EARS pattern: event-driven.

WHEN I08 executes under either signing profile, the A2 model SHALL produce the acceptance result below.

Acceptance result: After fill1 then cancellation, fresh nonce1 recovery refunds5; Alice5 and Bob5; cancelled parent remains paid5/allowance5/revision2, nonce0 unchanged.

#### Scenario: A2-I08 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I08 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I08 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.

### Requirement: A2-I09

EARS pattern: event-driven.

WHEN I09 executes under either signing profile, the A2 model SHALL produce the acceptance result below.

Acceptance result: Cancelled unfilled/residual states at deadline100 and101: separately authorized actual NoInput recovery refunds10/5; preserve original cancelled accounting.

#### Scenario: A2-I09 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I09 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I09 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.

### Requirement: A2-I10

EARS pattern: event-driven.

WHEN I10 executes under either signing profile, the A2 model SHALL produce the acceptance result below.

Acceptance result: Supplied recovery input at deadline is actual contract_closed with complete original-state rollback, zero payments/warnings/effects and zero reductions; retain rejection, not a refund.

#### Scenario: A2-I10 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I10 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I10 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.

### Requirement: A2-I11

EARS pattern: event-driven.

WHEN I11 executes under either signing profile, the A2 model SHALL produce the acceptance result below.

Acceptance result: Recovery without cancellation, unsigned/old nonce recovery, duplicate fills, duplicate cancellation and duplicate recovery fail at the appropriate authority boundary.

#### Scenario: A2-I11 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I11 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A2-I11 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.
