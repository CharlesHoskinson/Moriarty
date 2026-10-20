# Sherlock circuit optimizations

Status: DRAFT / PROPOSED, not adopted, implemented, validated or accepted. Public source reference: `35931362ca435b2ed10014ee052870716c8f10d3`. All scenarios are proposed acceptance contracts, not execution receipts.

This additive package preserves the [original requirements](../../../sherlock-universal-interpreter-design/requirements.md), [completion refinements](../../../sherlock-completion-convergence/requirements.md), [recursive experiment](../../../sherlock-stagenet-recursive-rollup/requirements.md) and [product contract](../../../../../docs/MORIARTY-PRODUCT-CONTRACT.md). Existing designated-owner requirements are carried forward without new security analysis or policy.

## ADDED Requirements

### Requirement: SOPT-001 Observed recursive baseline before integration

WHEN an optimization is proposed for integration, integration SHALL NOT proceed until an actual Midnight stagenet recursive child/parent/terminal baseline with its exact source, statements, keys, target receipts and complete readback is preserved; integration means any change within the traced source/context identity membership (SOPT-010) of the native recursive child, parent or terminal path, while isolated source preparation MAY precede that observation and changes outside that membership are unaffected.

Retains: SCON-022, SCON-023, SRPOC-001, SRPOC-002, SRPOC-007. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-001-01 preserved behavior

- **GIVEN** a separately prepared helper extraction and an observed recursive stagenet baseline
- **WHEN** integration is considered
- **THEN** the unchanged baseline artifacts and full readback remain available under their original identities

#### Scenario: SOPT-001-02 boundary or incomplete evidence

- **GIVEN** only a literal or local recursive result exists
- **WHEN** isolated source preparation finishes
- **THEN** the candidate remains isolated and integration awaits the missing actual recursive stagenet baseline

### Requirement: SOPT-002 Ordered source extraction equivalence

WHEN the duplicated publication utilities are extracted, the emitters SHALL preserve the ordered emitted operations modulo an explicit checked bijective temporary-variable renaming that is the identity on every non-temporary identifier, whose image lies inside the reserved temporary namespace enforced by the TemporaryNameCollision check and is disjoint from every input operand; they SHALL preserve the exact input/output ABI, construction errors, refusal behavior, complete state and statement, and preserve is_reserved_{return,error}_publication_variable and the %{scope}.local.temp. prefix or record any change as an explicit ABI change.

Retains: UIREQ-024, UIREQ-030, UIREQ-031, SCON-005, SCON-012, SCON-019. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-002-01 preserved behavior

- **GIVEN** independent old-emitter outputs for both publication branches
- **WHEN** the extracted emitters process the same admitted inputs
- **THEN** ordered operations and complete observations agree under the declared bijective renaming and identical ABI

#### Scenario: SOPT-002-02 boundary or incomplete evidence

- **GIVEN** a candidate that changes operation order, error locator or an operand, or a renaming that maps a temporary onto a non-temporary identifier
- **WHEN** the ordered comparison runs
- **THEN** the first divergence is retained and source-extraction acceptance remains open

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

WHEN an optimization candidate changes a transition slice covered by baseline grouping or residualization (UIREQ-032/033, SCON-014/015), it SHALL retain the baseline-required widths 1, 2, 4 and 8, every intermediate complete boundary, earliest stop, terminal-before-fuel, observer-zero/no-next and equivalent residualized behavior under explicit bounds.

Retains: UIREQ-010, UIREQ-012, UIREQ-013, UIREQ-016, UIREQ-032, UIREQ-033, SCON-007, SCON-008, SCON-014, SCON-015. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-007-01 preserved behavior

- **GIVEN** a supported slice and independently frozen complete prefixes
- **WHEN** widths 1, 2, 4 and 8 and bounded residualization are compared
- **THEN** each internal boundary and earliest stop matches the ungrouped baseline

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

WHERE a fused final-parent/terminal experiment is selected, it SHALL preserve the child statement, existing application predicate, accumulation discharge, complete financial and communication duties and exact ledger-call binding under a separately qualified relation and key, and SHALL account for lost reusable-parent work.

Retains: UIREQ-017, UIREQ-018, UIREQ-019, UIREQ-020, UIREQ-023, UIREQ-024, SCON-010, SCON-011, SCON-022, SCON-024, SRPOC-002, SRPOC-007. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-009-01 preserved behavior

- **GIVEN** separate and fused candidates for the same child statement and application predicate
- **WHEN** one and repeated envelope workloads are compared
- **THEN** discharge, duties and call binding remain intact and lost parent reuse is included in total cost

#### Scenario: SOPT-009-02 boundary or incomplete evidence

- **GIVEN** a fused candidate with equal public vectors but unqualified relation/key or missing duty evidence
- **WHEN** adoption is considered
- **THEN** fusion remains a separate unvalidated experiment and no lower k is assumed

### Requirement: SOPT-010 Fresh source context and key qualification

WHEN a file, feature, setting or context input within an artifact's traced source/context identity membership changes (including logging-only or expected-equivalent changes), the candidate SHALL record fresh source, relation, context and applicable key identities for each affected artifact and retain the existing qualification obligations before those artifacts are consumed; artifacts whose traced membership excludes the change SHALL retain their identities and qualification status.

Retains: UIREQ-023, UIREQ-024, UIREQ-025, SCON-001, SCON-004, SCON-022, SRPOC-002, SRPOC-005. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-010-01 preserved behavior

- **GIVEN** a source-only extraction or logging change within an artifact's traced identity membership
- **WHEN** a candidate context is prepared
- **THEN** fresh identities and applicable qualification status are recorded for affected artifacts even if ordered operations agree; artifacts whose traced membership excludes the change retain their identities and qualification status

#### Scenario: SOPT-010-02 boundary or incomplete evidence

- **GIVEN** an old key/context receipt and changed inputs within that artifact's traced identity membership
- **WHEN** the changed candidate is reported
- **THEN** the old receipt remains historical and does not qualify the new source cut

### Requirement: SOPT-011 Matched public workload comparison

WHEN an optimization benchmark is compared to baseline, it SHALL use the existing experiment-selected manifest with matched deterministic public seed, input program source, ordered workload, transaction and batch counts, caps, profile, cache state, thread settings, host hardware/OS identity, build/toolchain identity and target, record both baseline and candidate implementation/circuit source identities even when they differ, and record any mismatch in the matched controls as a comparison limitation.

Retains: SCON-024, SRPOC-003, SRPOC-004, SRPOC-005, SRPOC-008. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-011-01 preserved behavior

- **GIVEN** baseline and candidate using the same selected manifest and all matched controls
- **WHEN** a comparative result is published
- **THEN** the exact transaction/batch counts, seed, input program source, both implementation/circuit source identities and matched controls, including host hardware/OS and build/toolchain identities, accompany each result

#### Scenario: SOPT-011-02 boundary or incomplete evidence

- **GIVEN** a reduced fixture, different cache/thread setting, host hardware/OS or build/toolchain identity, or unsupported batch shape
- **WHEN** comparison is requested
- **THEN** the mismatch is explicit and the result is not relabelled as the full workload or a matched speedup

### Requirement: SOPT-012 Per-layer resource measurements

WHEN local optimization cost is reported, the report SHALL separate child, parent and terminal parameter loading/generation, prepare/witness/serialization, key generation, proving and independent verification with wall time, user/system CPU and peak RSS, actual k, domain capacity versus used rows, columns, lookup rows and maximum gate degree (distinct from k), and proof, verifying-key and full transaction bytes, retaining the actual sample count and raw per-sample measurements; unavailable metrics SHALL name the reason.

Retains: UIREQ-024, UIREQ-025, SCON-024, SRPOC-005, SRPOC-009. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-012-01 preserved behavior

- **GIVEN** matched measurements with raw phase records for each layer
- **WHEN** the comparison is summarized
- **THEN** every requested metric, including parameter loading/generation and maximum gate degree separately from k, is reported with units, phase, actual sample count and raw per-sample values or explicitly unknown with its reason

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

WHEN an optimization disposition changes from proposed to validated and then retain, reject or defer, the decision SHALL cite its exact candidate, independent correctness evidence, observed matched metrics or explicit unknowns and regressions, preserving the original baseline and first-divergence/failure records without inventing performance thresholds; a maintenance retain disposition for a change within the integration membership defined by SOPT-001 SHALL remain isolated until that baseline is preserved, while changes outside that membership are unaffected.

Retains: UIREQ-024, UIREQ-025, SCON-024, SRPOC-008, SRPOC-009. Status: proposed. QA: [qa.md](../../qa.md#clause-checks).

#### Scenario: SOPT-015-01 preserved behavior

- **GIVEN** a source cleanup that passes equivalence and reduces net maintenance but has unknown native cost
- **WHEN** a disposition is recorded
- **THEN** source-cleanup acceptance and performance adoption have distinct scoped conclusions; maintenance retain within SOPT-001 integration membership remains isolated until its baseline is preserved, while changes outside that membership are unaffected

#### Scenario: SOPT-015-02 boundary or incomplete evidence

- **GIVEN** a sharing or fusion experiment with worse matched cost or failed equivalence
- **WHEN** the result is disposed
- **THEN** retain, reject or defer cites the regression and preserves the baseline and failed evidence

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
