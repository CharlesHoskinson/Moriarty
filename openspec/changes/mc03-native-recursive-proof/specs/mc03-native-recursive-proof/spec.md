## ADDED Requirements

### Requirement: Reviewed restart
Native execution SHALL require a changed encoding hypothesis and both named audits before consuming a new bounded resource contract.

#### Scenario: Checked smaller encoding
- **WHEN** both auditors review the implemented smaller encoding and its resource contract
- **THEN** they require checked ranges and commitment preimages preserving every original financial and authority binding before launch.

#### Scenario: Unjustified retry
- **WHEN** dispatch reuses the failed 54-limb setup, unchecked state hashes, or an unreviewed larger k
- **THEN** the execution gate refuses dispatch and preserves the original failed campaign.

### Requirement: Real recursion and independent verification
The harness SHALL prove both financial steps and verify retained bytes in a separate process.

#### Scenario: Valid episode
- **WHEN** the native harness proves the original accrual and due-settlement transitions
- **THEN** a separate process verifies retained proofs, constrained genesis, expected fields, and the discharged final accumulator.

#### Scenario: Invalid proof context
- **WHEN** retained input is missing, truncated, altered, wrong-key, wrong-state, wrong-domain, wrong-intent, forged-genesis, or invalid-predecessor
- **THEN** the separate verifier rejects each mutant for its declared reason.

### Requirement: Hard resource stop
The runner SHALL enforce cumulative time, process-group memory, CPU, output, SRS, k, and attempt limits.

#### Scenario: Recorded bounded run
- **WHEN** a reviewed native campaign terminates
- **THEN** its receipt records actual peak usage, command, input digests, terminal status, and the exact proved predicate, if any.

#### Scenario: Resource failure
- **WHEN** synthesis, proving, a control, timeout, memory, or output enforcement fails
- **THEN** the native contract stops immediately without automatic escalation or a fresh attempt-counter reset.

### Requirement: Independent audit and provenance
The package SHALL bind acceptance evidence to exact sources, commands, environment, outputs, and both required audit identities.

#### Scenario: Audited result
- **WHEN** deterministic checks pass and both independent reviewers have no unresolved blocking finding
- **THEN** the package records accepted scope with the exact reviewed candidate digest.

#### Scenario: Missing or stale audit
- **WHEN** Fable or GPT-6 is unavailable, substituted, stale, or lacks a substantive identity-bound verdict
- **THEN** the package remains pending audit and cannot promote dependent acceptance.

### Requirement: Failed predicate stops promotion
The package SHALL remain incomplete if any required positive or rejection predicate fails.

#### Scenario: Negative control incorrectly accepts
- **WHEN** a required invalid input is accepted or its rejection lacks evidence
- **THEN** verification fails and dependent acceptance remains blocked.
