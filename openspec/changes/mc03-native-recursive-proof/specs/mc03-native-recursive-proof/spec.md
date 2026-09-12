## ADDED Requirements

### Requirement: Reviewed restart
Native execution SHALL require a changed encoding hypothesis and both named audits before consuming a new bounded resource contract.

#### Scenario: Checked smaller encoding
- **WHEN** both auditors review the implemented smaller encoding and its resource contract
- **THEN** they require checked ranges and commitment preimages preserving every original financial and authority binding before launch.
- **Amended by** `pcd-ledger-anchored-acceptance` requirement "Off-ledger segment certificate": the fixed 54-limb relation and its smaller-encoding retry are retired; SP06 reviews the segment relation instead.

#### Scenario: Unjustified retry
- **WHEN** dispatch reuses the failed 54-limb setup, unchecked state hashes, or an unreviewed larger k
- **THEN** the execution gate refuses dispatch and preserves the original failed campaign.

### Requirement: Real recursion and independent verification
The harness SHALL prove both financial steps and verify retained bytes in a separate process.

#### Scenario: Valid episode
- **WHEN** the native harness proves the original accrual and due-settlement transitions
- **THEN** a separate process verifies retained proofs, constrained genesis, expected fields, and the discharged final accumulator.
- **Amended by** `pcd-ledger-anchored-acceptance` requirement "Off-ledger segment certificate": the fixed two-step episode becomes segment certificates at 1, 10 and 100 steps, still verified from retained bytes in a fresh process.

#### Scenario: Invalid proof context
- **WHEN** retained input is missing, truncated, altered, wrong-key, wrong-state, wrong-domain, wrong-intent, forged-genesis, or invalid-predecessor
- **THEN** the separate verifier rejects each mutant for its declared reason.
- **Amended by** `pcd-ledger-anchored-acceptance` requirement "Bounded native certificates": the rejection list also covers free or mismatched guards, substituted `vk_repr` and unbound inner instances.

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
- **WHEN** Fable 5.1 or GPT-6 is unavailable, substituted, stale, or lacks a substantive identity-bound verdict
- **THEN** the package remains pending audit and cannot promote dependent acceptance.

### Requirement: Failed predicate stops promotion
The package SHALL remain incomplete if any required positive or rejection predicate fails.

#### Scenario: Negative control incorrectly accepts
- **WHEN** a required invalid input is accepted or its rejection lacks evidence
- **THEN** verification fails and dependent acceptance remains blocked.

### Requirement: Complete backend decision before native dispatch
A native campaign SHALL satisfy RP01, RP02 stages F0/F1 and campaign-specific RP03, distinguish fixed-instance IVC from general private multi-input PCD, and retain independent proof-byte verification. An undefined final verifier or handoff interface SHALL prevent dispatch.

#### Scenario: Report requirement omitted
- **WHEN** a candidate omits the applicable requirement or substitutes an earlier narrower experiment
- **THEN** acceptance remains pending under [the report reconciliation](../../../../REPORT-RECONCILIATION-2026-09-07.md).
- **Amended by** `pcd-ledger-anchored-acceptance` requirement "Recursion dependency tracking and stop": the backend decision is the certificate route, gated by the Midnight dependency tracker and a reviewed resource amendment.
