## ADDED Requirements

### Requirement: RFB-001 The semantic reference covers the full language
The REFERENCE BRIDGE SHALL supply a semantic evaluator (R1) for nominal types, constructors of any arity, planned matches, structured inputs and every outcome. The normative artifact SHALL be the choice of OD-21 (OWNER), with the recommendation a separate evaluator over `moriarty-sherlock-nominal/1`. The work SHALL be sized by X13 first. The language SHALL NOT be cut to a supported subset.

#### Scenario: Corpus with wide constructors and structured input
- **WHEN** the corpus runs, including a constructor of more than 8 fields across a leaf boundary, a structured input, a planned match and every outcome
- **THEN** the coverage report has zero gaps (host tests `bridge_corpus_zero_gaps`, `bridge_wide_constructor_across_leaf`, `bridge_structured_input`, `bridge_planned_match`, `bridge_every_outcome`)

### Requirement: RFB-002 The event map is explicit and total
The bridge SHALL map helper events to owner events and owner events to source steps (R2), with the free terminal publication. `n_steps` SHALL be the count of enabled owner events, publication included and padding excluded (SRC-003). A helper-event count SHALL NOT be equated with a source-step count without this map.

#### Scenario: Step count
- **WHEN** the owner events of a corpus run are mapped to source steps
- **THEN** `n_steps` equals the enabled owner-event count (host test `bridge_step_count_matches_owner_events`)

### Requirement: RFB-003 The projection is total and every rule is covered
The projection (R3) from the owner-machine state at an owner-event boundary to the reference state SHALL be total for every form of R1. `UnsupportedGeneric` and `UnmappedRule` SHALL NOT occur on the corpus. The corpus SHALL fire all 120 enabled rules, or SHALL record why a source program cannot reach a rule.

#### Scenario: All rules fired
- **WHEN** the corpus runs with a constructed case for each of the 18 rules that the X1 corpus did not fire
- **THEN** the report lists all 120 rules as fired or with a recorded reason (host test `bridge_rules_fired_cover_inventory`)

### Requirement: RFB-004 A gap fails the gate
The coverage report (R4) SHALL list rules fired, forms projected and gaps per run. A gap SHALL fail the harness and SHALL NOT be skipped. The whole-leaf harness SHALL be an acceptance gate only after the report has zero gaps. The bridge SHALL be trusted host diagnostics only and SHALL NOT be cited as a soundness proof.

#### Scenario: Seeded divergence
- **WHEN** a divergence is seeded in the owner machine (a wrong field order in a constructor, or a dropped publication step)
- **THEN** the harness reports it and fails (negative controls of the kit)

#### Scenario: Projection gap
- **WHEN** a projection gap occurs on a corpus case
- **THEN** the harness fails and does not skip the case (host test `bridge_corpus_zero_gaps`), and the kit has no Lean obligation of its own, so its evidence is these host tests only
