# Sherlock circuit optimizations

## ADDED Requirements

### Requirement: SOPT-001 Observed recursive baseline before integration
WHEN any change within the traced source/context identity membership (SOPT-010) of the native recursive child, parent or terminal path is proposed, that change is integration and SHALL NOT proceed before an actual Midnight stagenet baseline with exact source, statements, keys, target receipts and complete readback is preserved.

Retains: SCON-022, SCON-023, SRPOC-001, SRPOC-002, SRPOC-007. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-001-01 preserved behavior
- **GIVEN** a separately prepared helper extraction and an actual Midnight stagenet recursive child/parent/terminal baseline that preserves its exact source, statements, keys, target receipts, and complete readback
- **AND** the proposed change lies within that path's traced source/context identity membership as defined by SOPT-010
- **WHEN** integration is considered
- **THEN** the unchanged baseline artifacts and full readback remain available under their original identities

#### Scenario: SOPT-001-02 boundary or incomplete evidence
- **GIVEN** only a literal or local recursive result exists
- **WHEN** isolated source preparation finishes
- **THEN** isolated source preparation MAY precede the baseline, the candidate remains isolated, integration awaits the missing actual recursive stagenet baseline, and changes outside the traced membership remain unaffected

### Requirement: SOPT-002 Ordered source extraction equivalence
WHEN utilities are extracted, the emitters SHALL preserve ordered operations modulo an explicitly checked temp-ID bijection. They SHALL preserve the exact input/output ABI, construction errors, refusal behavior, complete state, statement, reserved publication variables, and temporary prefix, or record an explicit ABI change.

Retains: UIREQ-024, UIREQ-030, UIREQ-031, SCON-005, SCON-012, SCON-019. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-002-01 preserved behavior
- **GIVEN** independent old-emitter outputs for both publication branches
- **WHEN** the extracted emitters process the same admitted inputs
- **THEN** ordered operations and complete observations agree under an explicit checked bijective temporary-variable renaming. The renaming is identity on every non-temporary identifier, its image lies within the reserved temporary namespace enforced by TemporaryNameCollision, and it is disjoint from every input operand. Preserve `is_reserved_{return,error}_publication_variable` and the `%{scope}.local.temp.` prefix.

#### Scenario: SOPT-002-02 boundary or incomplete evidence
- **GIVEN** a candidate that changes operation order, error locator, or an operand, or a renaming that maps a temporary onto a non-temporary identifier or input operand
- **WHEN** the ordered comparison runs
- **THEN** the first divergence is retained and source-extraction acceptance remains open. Record any change to the exact input/output ABI as an explicit ABI change.

### Requirement: SOPT-003 Net maintenance accounting

WHEN source extraction is assessed, its report SHALL count total maintained lines before and after including shared helpers, adapters and validation code, and SHALL distinguish measured maintenance reduction from unmeasured runtime, constraint and proving effects.

Retains: UIREQ-024, UIREQ-025, SCON-024, SRPOC-009. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-003-01 preserved behavior

- **GIVEN** a helper extraction with added adapters and comparison code
- **WHEN** maintenance impact is reported
- **THEN** all added and removed maintained lines enter the net count with no inferred runtime benefit

#### Scenario: SOPT-003-02 boundary or incomplete evidence

- **GIVEN** identical source utilities but no native measurement
- **WHEN** a performance summary is prepared
- **THEN** rows, k, CPU, RSS and proof bytes remain explicitly unmeasured

### Requirement: SOPT-004 Typed layout and encoding seam

WHEN child, parent or terminal consumers share typed layout or encoding declarations, each consumer SHALL retain every lane, width, range, padding rule, domain, public-field order, deterministic traversal and source provenance while retaining separate constraint, witness and export plans.

Retains: UIREQ-027, UIREQ-029, UIREQ-031, UIREQ-035, SCON-001, SCON-002, SCON-004, SCON-012, SCON-013. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-004-01 preserved behavior

- **GIVEN** declared child, parent and terminal layouts with separate plans
- **WHEN** a shared typed descriptor is introduced
- **THEN** each consumer emits its exact original lane order, padding, domain and provenance with deterministic plans

#### Scenario: SOPT-004-02 boundary or incomplete evidence

- **GIVEN** a descriptor with one reordered public lane or changed inactive padding
- **WHEN** the consumer-specific comparison runs
- **THEN** the changed consumer is identified and layout acceptance remains open

### Requirement: SOPT-005 Independent layout expectations

WHEN shared production layout or codec code is checked, the comparison SHALL use independently authored complete expected fields and bytes frozen before candidate execution, rather than expectations generated by that shared production descriptor or its walkers.

Retains: UIREQ-024, UIREQ-035, SCON-012, SCON-013, SCON-019. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-005-01 preserved behavior

- **GIVEN** independently frozen expected images and bytes for contrasting fixtures
- **WHEN** shared encoding is compared
- **THEN** all complete fields and ordered bytes match those independent expectations

#### Scenario: SOPT-005-02 boundary or incomplete evidence

- **GIVEN** expected bytes generated by the candidate production layout
- **WHEN** a layout comparison is reported
- **THEN** the result is labelled internal agreement and cannot close the independent comparison

### Requirement: SOPT-006 All selectors within organizational families

WHEN rule implementations are organized into eight families, the inventory SHALL retain exactly the 39 distinct semantic selectors in rule-families.json, their original guards, state deltas, priorities, fuel charges and first-failure behavior, including the three free events A080, A082 and A084.

Retains: UIREQ-003, UIREQ-004, UIREQ-006, UIREQ-008, UIREQ-010, UIREQ-012, UIREQ-013, SCON-005. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-006-01 preserved behavior

- **GIVEN** the exact 39-selector baseline map
- **WHEN** eight organizational families are emitted in an inventory
- **THEN** every selector appears once with its original charge, guard, delta, priority and first failure

#### Scenario: SOPT-006-02 boundary or incomplete evidence

- **GIVEN** a family implementation that merges two selectors or charges terminal publication
- **WHEN** the per-selector comparison runs
- **THEN** the distinct changed event is reported and the family cannot replace the baseline

### Requirement: SOPT-007 Required grouping and residualization
WHEN a slice covered by baseline grouping or residualization changes, the candidate SHALL preserve baseline widths, intermediate boundaries, earliest stop, terminal-before-fuel, observer-zero/no-next, and bounded equivalent residualization.

Retains: UIREQ-010, UIREQ-012, UIREQ-013, UIREQ-016, UIREQ-032, UIREQ-033, SCON-007, SCON-008, SCON-014, SCON-015. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-007-01 preserved behavior
- **GIVEN** a supported slice covered by UIREQ-032/033 and SCON-014/015 with independently frozen complete prefixes
- **WHEN** baseline widths 1, 2, 4 and 8 and bounded residualization under explicit bounds are compared
- **THEN** every complete intermediate boundary and earliest stop matches the ungrouped baseline, including terminal-before-fuel and observer-zero/no-next behavior

#### Scenario: SOPT-007-02 boundary or incomplete evidence

- **GIVEN** a terminal event before the end of a group or an observer budget of zero
- **WHEN** the grouped path is observed
- **THEN** later semantic work does not occur and the original terminal/no-next observation is preserved

### Requirement: SOPT-008 Conditional physical constraint sharing

WHERE a physical-sharing experiment removes repeated same-cell range or preservation constraints, the candidate SHALL identify the exact constrained cells, predicates and activation domains and retain existing designated-owner verification of dominance and equivalence of the accepted relation at every intermediate boundary.

Retains: UIREQ-024, UIREQ-026, SCON-005, SCON-008, SCON-015, SCON-024. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-008-01 preserved behavior

- **GIVEN** an exact same-cell proposal with the existing owner-verified activation and dominance evidence
- **WHEN** physical sharing is evaluated
- **THEN** all intermediate boundaries and the accepted relation remain equivalent under the stated domains

#### Scenario: SOPT-008-02 boundary or incomplete evidence

- **GIVEN** matching honest witnesses but missing accepted-relation or activation evidence
- **WHEN** physical sharing is assessed
- **THEN** the missing owner evidence is explicit and the baseline constraints remain the integration reference

### Requirement: SOPT-009 Optional fused final parent and terminal
WHERE fusion is selected, it SHALL preserve the child statement, application predicate, accumulation discharge, financial/communication duties, and exact ledger-call binding. It SHALL account for lost parent reuse.

Retains: UIREQ-017, UIREQ-018, UIREQ-019, UIREQ-020, UIREQ-023, UIREQ-024, SCON-010, SCON-011, SCON-022, SCON-024, SRPOC-002, SRPOC-007. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-009-01 preserved behavior
- **GIVEN** separate and fused candidates for the same child statement and application predicate, with a separately qualified relation and key
- **WHEN** one-envelope and repeated-envelope workloads are compared
- **THEN** accumulation discharge, financial and communication duties, and exact call binding remain intact, and the comparison includes lost reusable-parent work

#### Scenario: SOPT-009-02 boundary or incomplete evidence
- **GIVEN** a fused candidate with equal public vectors but an unqualified relation/key or missing duty evidence
- **WHEN** adoption is considered
- **THEN** fusion remains a separate unvalidated experiment and no lower k is assumed

### Requirement: SOPT-010 Context and key qualification
WHEN any file, feature, setting or context input within an artifact's traced source/context identity membership changes, including logging-only or expected-equivalent changes, the candidate SHALL record fresh source, relation, context and applicable key identities for each affected artifact and retain existing qualification obligations before use. Artifacts outside the changed membership SHALL retain their identities and qualification status.


#### Scenario: SOPT-010-01 preserved behavior
- **GIVEN** a file, feature, setting, or context-input change within an artifact's traced source/context identity membership, including a logging-only or expected-equivalent change
- **WHEN** a candidate context is prepared
- **THEN** record fresh source, relation, context, and applicable key identities and qualification status for each affected artifact, even if ordered operations agree. Artifacts whose traced membership excludes the change retain their identities and qualification status

#### Scenario: SOPT-010-02 boundary or incomplete evidence

- **GIVEN** an old key/context receipt and changed inputs within that artifact's traced identity membership
- **WHEN** the changed candidate is reported
- **THEN** the old receipt remains historical and does not qualify the new source cut

### Requirement: SOPT-011 Matched public workload comparison
WHEN an optimization benchmark is compared with baseline, it SHALL use the experiment-selected manifest and matched controls, record both implementation/circuit source identities, and report each control mismatch as a limitation.

Retains: SCON-024, SRPOC-003, SRPOC-004, SRPOC-005, SRPOC-008. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-011-01 preserved behavior
- **GIVEN** baseline and candidate using the same experiment-selected manifest, deterministic public seed, input-program source, ordered workload, transaction and batch counts, caps, profile, cache state, thread settings, host hardware/OS identity, build/toolchain identity, and target
- **WHEN** a comparative result is published
- **THEN** report both baseline and candidate implementation/circuit source identities even when they differ, and include every matched control with each result

#### Scenario: SOPT-011-02 boundary or incomplete evidence

- **GIVEN** a reduced fixture, different cache/thread setting, host hardware/OS or build/toolchain identity, or unsupported batch shape
- **WHEN** comparison is requested
- **THEN** the mismatch is explicit and the result is not relabelled as the full workload or a matched speedup

### Requirement: SOPT-012 Per-layer resource measurements
WHEN local optimization cost is reported, the report SHALL separate child, parent and terminal phases, each with wall time, user/system CPU and peak RSS, and give actual k, capacity versus used rows, columns, lookup rows, maximum gate degree (distinct from k), proof, verifying-key and full transaction bytes, actual sample count and raw samples. Unavailable metrics SHALL name their reason.


#### Scenario: SOPT-012-01 preserved behavior
- **GIVEN** matched measurements with raw records for child, parent, and terminal parameter loading/generation, prepare/witness/serialization, key generation, proving, and independent verification
- **WHEN** the comparison is summarized
- **THEN** report parameter loading/generation, prepare/witness/serialization, key generation, proving, and independent verification with wall time, user/system CPU, and peak RSS. Report actual k, domain capacity versus used rows, columns, lookup rows, and maximum gate degree distinct from k. Report proof, verifying-key, and full-transaction bytes, actual sample count, and raw per-sample measurements. Name the reason for each unavailable metric

#### Scenario: SOPT-012-02 boundary or incomplete evidence

- **GIVEN** domain capacity known but used rows or independent verify CPU unavailable
- **WHEN** the metric table is filled
- **THEN** capacity is not substituted for used rows and missing measurements remain unknown

### Requirement: SOPT-013 Target latency and complete readback

WHEN target optimization results are reported, the report SHALL separate local independent cryptographic verification, submission, inclusion, finality and settlement readback, retain complete observed effects and existing target acceptance evidence, and attribute validator CPU only to actual validator-side measurement.

Retains: SCON-022, SRPOC-002, SRPOC-006, SRPOC-007, SRPOC-009. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-013-01 preserved behavior

- **GIVEN** local verification timings and complete target receipts/readback
- **WHEN** target performance is reported
- **THEN** submission, inclusion, finality and readback are separated from local verification and complete effects remain visible

#### Scenario: SOPT-013-02 boundary or incomplete evidence

- **GIVEN** only an RPC response duration or submitted transaction ID
- **WHEN** validator or finality performance is summarized
- **THEN** validator CPU and unobserved settlement stages remain unknown rather than inferred

### Requirement: SOPT-014 Same-slice formal and handwritten QA

WHEN an optimized transition slice is validated, its QA SHALL align independently specified complete-state and prefix expectations across Clean, finite Quint, K and handwritten circuits on the declared common domain, preserving explicit projection, numeric, helper, native and target scope gaps and local-only CI.

Retains: UIREQ-021, UIREQ-024, UIREQ-025, UIREQ-026, SCON-017, SCON-018, SCON-019, SCON-020. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-014-01 preserved behavior

- **GIVEN** one declared transition slice with independently frozen complete prefixes
- **WHEN** the existing four lanes compare that slice
- **THEN** lane-specific results, adapters and remaining gaps are recorded without inferred downstream acceptance

#### Scenario: SOPT-014-02 boundary or incomplete evidence

- **GIVEN** a finite Quint pass or K/host equality with missing Clean/native correspondence
- **WHEN** QA is summarized
- **THEN** the finite observation retains its scope and the missing correspondence stays open

### Requirement: SOPT-015 Evidence-based optimization disposition
WHEN a disposition changes from proposed to validated and then retain, reject or defer, the decision SHALL cite the exact candidate, independent correctness evidence, matched metrics or explicit unknowns, regressions, and preserved baseline/first-failure records. It SHALL invent no performance threshold.

Retains: UIREQ-024, UIREQ-025, SCON-024, SRPOC-008, SRPOC-009. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-015-01 preserved behavior
- **GIVEN** a source cleanup that passes equivalence and reduces net maintenance but has unknown native cost
- **WHEN** a disposition is recorded
- **THEN** source-cleanup acceptance and performance adoption have distinct scoped conclusions. A maintenance retain disposition within the SOPT-001 integration membership remains isolated until its baseline is preserved; changes outside that membership are unaffected

#### Scenario: SOPT-015-02 boundary or incomplete evidence

- **GIVEN** a sharing or fusion experiment with worse matched cost or failed equivalence
- **WHEN** the result is disposed
- **THEN** retain, reject or defer cites the regression and preserves the baseline, first-divergence record, and failure evidence

### Requirement: SOPT-016 Retained requirements and evidence boundaries

WHEN this optimization package is used, its reports SHALL retain UIREQ-001–035, SCON-001–025, SRPOC-001–009 and all existing financial, formal, resource, security-owner and roadmap obligations unchanged, distinguish proposed specifications from implementation, observed validation and acceptance, and add no new completion denominator or public developer permission.

Retains: UIREQ-025, SCON-023, SRPOC-001, SRPOC-008, SRPOC-009. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-016-01 preserved behavior

- **GIVEN** this unimplemented draft and retained requirement families
- **WHEN** coverage is summarized
- **THEN** all SOPT clauses remain proposed with no implementation, validation or acceptance closure and retained requirements keep their original counts

#### Scenario: SOPT-016-02 boundary or incomplete evidence

- **GIVEN** a draft specification pass or source comparison alone
- **WHEN** a full Sherlock or roadmap completion claim is considered
- **THEN** the evidence remains scoped and no original clause or exit closes by implication
