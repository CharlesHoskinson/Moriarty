# A3 completion requirements

## ADDED Requirements

These are prospective EARS obligations.
The global contract in `openspec/WORK-PACKAGES-EARS.md` also applies.

### Requirement: A3-S01

EARS pattern: event-driven.

WHEN S01 executes under either signing profile, the A3 model SHALL produce the acceptance result below.

Acceptance result: Start unfunded canonical swap; prepare/sign each depositor nonce0 independently; actual Alice10 and Bob20 deposits fund the corresponding accounts.

#### Scenario: A3-S01 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A3-S01 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A3-S01 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.

### Requirement: A3-S02

EARS pattern: event-driven.

WHEN S02 executes under either signing profile, the A3 model SHALL produce the acceptance result below.

Acceptance result: Prepare/register required disposition nonce1 policies under current facts; actual settle1 transfers TokenA10 to Bob and TokenB20 to Alice; N0/zero escrow.

#### Scenario: A3-S02 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A3-S02 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A3-S02 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.

### Requirement: A3-S03

EARS pattern: event-driven.

WHEN S03 executes under either signing profile, the A3 model SHALL produce the acceptance result below.

Acceptance result: Actual settle0 voluntarily refunds both owners with exact ordered effects and authority keys.

#### Scenario: A3-S03 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A3-S03 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A3-S03 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.

### Requirement: A3-S04

EARS pattern: event-driven.

WHEN S04 executes under either signing profile, the A3 model SHALL produce the acceptance result below.

Acceptance result: At100 and101, actual NoInput timeout refunds funded accounts; separate Alice-only timeout refunds10 with exact required authorities.

#### Scenario: A3-S04 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A3-S04 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A3-S04 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.

### Requirement: A3-S05

EARS pattern: event-driven.

WHEN S05 executes under either signing profile, the A3 model SHALL produce the acceptance result below.

Acceptance result: Supplied settlement/refund input at deadline rolls back to the original candidate minimumTime/accounts/choices; no speculative timeout state is retained.

#### Scenario: A3-S05 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A3-S05 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A3-S05 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.

### Requirement: A3-S06

EARS pattern: event-driven.

WHEN S06 executes under either signing profile, the A3 model SHALL produce the acceptance result below.

Acceptance result: Explicit environment advance stales prepared/verified snapshots; retain and reject stale records, or create a legally distinct fresh attempt where the finite inventory permits.

#### Scenario: A3-S06 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A3-S06 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A3-S06 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.

### Requirement: A3-S07

EARS pattern: event-driven.

WHEN S07 executes under either signing profile, the A3 model SHALL produce the acceptance result below.

Acceptance result: Empty timeout: actual accepted no-effect Core result, explicit envelope rejection; no invented effect or claimed successful commitment.

#### Scenario: A3-S07 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A3-S07 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A3-S07 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.

### Requirement: A3-S08

EARS pattern: event-driven.

WHEN S08 executes under either signing profile, the A3 model SHALL produce the acceptance result below.

Acceptance result: Wrong actor/nonce, changed second planned operation, unused program node, reordered effects, altered result and stale policy facts cannot obtain signing or commitment.

#### Scenario: A3-S08 after-resolution

- **GIVEN** SignAfterResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A3-S08 before-resolution

- **GIVEN** SignBeforeResolve and the specified predecessor
- **WHEN** the actual guarded scenario executes
- **THEN** the complete recorded result SHALL equal the acceptance result above.

#### Scenario: A3-S08 result substitution

- **WHEN** a result field or authority binding is substituted
- **THEN** the scenario checker SHALL reject the substituted result.
- **AND** the original request and result SHALL remain available.
