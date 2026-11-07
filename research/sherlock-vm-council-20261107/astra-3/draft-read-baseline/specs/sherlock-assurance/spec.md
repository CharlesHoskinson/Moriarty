# Capability: sherlock-assurance (evidence, negative tests, coverage)

Status: DRAFT v1, specified-only. Adopts ASR-001..012 (astra-1), SND-007 (astra-2) and astra-3's scope clauses; refines UISEC-012, UIREQ-021/023/024. Design: `../../assurance.md`.

## ADDED Requirements

### Requirement: SAS-001 Typed evidence record
The assurance reporter SHALL record evidence class, dependency fingerprints, profile, bounds, command, tool versions, artifact hashes and remaining assumptions for every result.

#### Scenario: Sample labelled as theorem
- **WHEN** a Quint sampled run is recorded with class `formal theorem`.
- **THEN** the schema check rejects the record.

### Requirement: SAS-002 Stale evidence
When a declared dependency changes (HEAD, pin, key, runtime, chain instance, configuration, artifact), dependent evidence SHALL be marked stale until revalidated.

#### Scenario: Key change at unchanged HEAD
- **WHEN** the installed decider VK changes without a source commit.
- **THEN** A6/A8-shape evidence is stale.

### Requirement: SAS-003 Honest negative evidence
If a negative test fails before the claimed verifier stage, the reporter SHALL label that stage untested and refuse constraint-rejection credit.

#### Scenario: Synthesis error counted as rejection
- **WHEN** `qa_mutation::run_once` maps a synthesis error to `Rejected` (`qa_mutation.rs:458-486`).
- **THEN** the campaign is labelled partial and `assert_sound` fails until zero such outcomes.

### Requirement: SAS-004 Mutation campaign detail
When a mutation campaign closes a constraint obligation, the report SHALL identify the reached layer, mutant, seed, case count, sampled/exhaustive status, site, allowlist, a weakened control, and SHALL include fresh malicious-witness proofs.

#### Scenario: 70-lane decider record
- **WHEN** the historical M4 record (`sherlock_gp.rs:323-349`, public inputs mutated under an existing proof) is cited.
- **THEN** it is labelled "existing-proof public-input binding", not malicious-witness soundness, with source hash and run id.

### Requirement: SAS-005 Complete trace comparison
When a trace comparison closes a semantic obligation, it SHALL compare complete ordered observations through a declared projection.

#### Scenario: Equal final value, different environment
- **WHEN** an intermediate environment is mutated and the final result is unchanged.
- **THEN** the comparator reports the first divergence.

### Requirement: SAS-006 Undischarged obligations
While recursive evidence has an undischarged obligation, the verifier SHALL refuse final-valid classification.

#### Scenario: Skipped decider
- **WHEN** only node proofs verify. **THEN** no final-valid label.

### Requirement: SAS-007 Manifest-bound target receipt
When a target result qualifies a candidate, the checker SHALL match call, key, statement and effects to the candidate's manifest and SHALL NOT select by modification time or by any call at the address.

#### Scenario: Reversed mtimes
- **WHEN** two candidate directories have reversed modification times.
- **THEN** the same manifest-named candidate is selected (today `check-a10.sh:7-24` picks the newest).

### Requirement: SAS-008 Optimization preservation (Optional)
Where an optimization is enabled, qualification SHALL retain the specified semantics and re-run affected adversarial and target checks.

#### Scenario: Node at k=19
- **WHEN** the node is reduced below 2^19 rows. **THEN** the refusal set and link tests re-run before the new key is installed.

### Requirement: SAS-009 Formal claim artifact
When a release claims a formal property, the report SHALL name a checkable proof artifact, theorem, assumptions and source closure.

#### Scenario: Lean contract cited
- **WHEN** a `clean/fork` module is cited for refinement. **THEN** the report names the declaration and states that kernel checking does not establish Rust or circuit refinement.

### Requirement: SAS-010 Rule admission evidence
When a release admits a native rule, the inventory SHALL name evidence for invariant initialization, preservation and every fragment-composition premise.

#### Scenario: Lane assumed canonical
- **WHEN** a lane that dispatch assumes canonical is altered in a raw witness.
- **THEN** the composed leaf rejects it or a checked invariant argument excludes it.

### Requirement: SAS-011 Cross-layer re-verification
When a profile, compiler, proof system, key or ledger adapter changes, affected interfaces SHALL be re-verified before submission.

#### Scenario: Proof tag era
- **WHEN** a `ProofVersioned::V2` artifact is offered to a 9.1.0.1 target. **THEN** preflight refuses.

### Requirement: SAS-012 Release report classes
A release report SHALL distinguish implemented behaviour, executed evidence, proved properties and open obligations.

#### Scenario: Generated from tasks
- **WHEN** a report is produced from `tasks.md`. **THEN** unexecuted and unproved items remain open.

### Requirement: SAS-013 Dispatch receipt coverage
The dispatch receipt SHALL cover every admitted rule and priority boundary by a committed partition derived from the complete dispatch guard dependencies, with directed cases for rules absent from P1-P3, every competing rule per class, coordinated and raw-field mutations, fired-rule acceptance on every step, and SHALL be labelled a heuristic unless mechanically derived.

#### Scenario: Partition record
- **WHEN** the receipt lists partition fields, class count, per-class rules tested, directed cases, mutations, budget keys and preprocessing scope.
- **THEN** `verify_receipt` accepts it; a receipt whose partition omits `equal_differ` value reads is labelled heuristic.

### Requirement: SAS-014 Reduced-scope diagnostics
When a diagnostic uses fewer states, rules or mutations than the acceptance check, its evidence SHALL say so and SHALL NOT satisfy the receipt.

#### Scenario: One state, one rule
- **WHEN** the M3 instrumentation run checks one state and one rule to time `ProofPreimage::check`.
- **THEN** its record is a timing diagnostic; the inventory still reports the receipt absent.
