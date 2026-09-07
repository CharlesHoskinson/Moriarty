## ADDED Requirements

### Requirement: Complete delivery coverage
The schedule SHALL retain every MC01-MC08 requirement and required financial behavior with named sprint ownership.

#### Scenario: Covered completion program
- **WHEN** the plan validator reads the original specifications and sprint register
- **THEN** each requirement has an existing sprint owner and all prerequisite references form an acyclic graph.

#### Scenario: Missing required behavior
- **WHEN** a required fixture, action, held-out case or operator lacks a closure owner
- **THEN** plan validation or the SP01 behavior audit rejects coverage and completion remains blocked.

### Requirement: Stage-specific execution admission
Each actionful task SHALL use current accepted-profile stage evidence and a separately admitted RP03 campaign record.

#### Scenario: Current evidence
- **WHEN** every required stage and candidate-specific resource/review record is valid
- **THEN** the owning package may dispatch only its recorded commands within its allocation.

#### Scenario: Plan mistaken for execution authority
- **WHEN** a sprint is merely specified, a campaign ID is null, or a required native control is incomplete
- **THEN** dispatch rejects without resetting historical counters.

### Requirement: Honest formal and completion claims
The program SHALL distinguish language specifications, tests, K proofs, native proofs and actual ledger acceptance.

#### Scenario: Reproducible accepted result
- **WHEN** required deterministic checks and retained proof/chain verification pass for the exact accepted lineage
- **THEN** completion may cite only that scope with both named current result audits and source provenance.

#### Scenario: Narrow evidence substituted
- **WHEN** a planning review, fixed example, mock certificate or old profile is used to close a broader requirement
- **THEN** completion rejects and the unmet requirement remains open.
