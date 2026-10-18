# eUTXO conformance and private feasibility

## ADDED Requirements

### Requirement: EU-CON-001 Frozen inputs and drift refusal
WHEN a conformance run starts, THEN the runner SHALL hash every input and refuse changed bytes before evaluation.

#### Scenario: EU-CON-001 positive
- **WHEN** all hashes equal the frozen manifest
- **THEN** the run retains paths, SHA-256, tool versions, UTC times, and exit statuses

#### Scenario: EU-CON-001 negative
- **WHEN** one source, key, blueprint, expected envelope, or node configuration hash differs
- **THEN** the run records INPUT_DRIFT and produces no approval

### Requirement: EU-CON-002 Complete layered correspondence
WHEN a family case completes, THEN the runner SHALL compare complete source, ABI, UPLC, and ledger envelopes at their respective boundaries.

#### Scenario: EU-CON-002 positive
- **WHEN** source acceptance and compiled transition agree on every retained state and ordered effect field
- **THEN** the receipt binds complete prestate, poststate, effects, work, context, and source arguments

#### Scenario: EU-CON-002 negative
- **WHEN** one field is missing, extra, reordered, altered, or copied from observed output
- **THEN** the comparison fails with a JSON path and both values

### Requirement: EU-CON-003 Exact failure identity
WHEN a control fails, THEN the runner SHALL preserve the complete failure object and distinguish source, admission, UPLC, transport, and ledger failure layers.

#### Scenario: EU-CON-003 positive
- **WHEN** release-work-one-short uses the frozen oracle19 case
- **THEN** source WORK_EXHAUSTED and workUsed 48 remain source facts

#### Scenario: EU-CON-003 negative
- **WHEN** a reporter replaces GUARD_FAILED with PlutusFailure or E12 success with source NONEMPTY_OBLIGATIONS
- **THEN** the reporter rejects the relabeled receipt

### Requirement: EU-CON-004 Twenty-family completeness
WHEN an assessment is generated, THEN the assessment SHALL retain exactly all twenty inventory IDs with owners, tasks, tests, receipts, and scope.

#### Scenario: EU-CON-004 positive
- **WHEN** each inventory ID has source, representation, admission, UPLC, native, and private fields
- **THEN** coverage records implementation gaps without narrowing the requested goal

#### Scenario: EU-CON-004 negative
- **WHEN** a family is omitted or missing implementation is relabeled an external feasibility blocker
- **THEN** coverage fails with the omitted ID or unsupported disposition

### Requirement: EU-CON-005 Independent expected values
WHEN a new test is frozen, THEN the reviewer SHALL derive its expected complete envelope before observing implementation output.

#### Scenario: EU-CON-005 positive
- **WHEN** expected values use source semantics and an independent financial derivation
- **THEN** the expected-value receipt predates the candidate observation and names its derivation

#### Scenario: EU-CON-005 negative
- **WHEN** expected effects are assigned from positive.effects or generated outputs become their own oracle
- **THEN** the test is inadmissible as independent behavioral evidence

### Requirement: EU-CON-006 Deterministic differential replay
WHEN a differential case runs, THEN the runner SHALL replay its pinned source and canonical ABI against generated spending UPLC and compare complete outcomes.

#### Scenario: EU-CON-006 positive
- **WHEN** a varied Lender pre-balance 7 is released
- **THEN** the successor contains Lender Cash 107 and every unrelated field matches

#### Scenario: EU-CON-006 negative
- **WHEN** the lowerer always emits the control post with Lender Cash 100
- **THEN** full-envelope comparison fails and the counterexample is retained

### Requirement: EU-CON-007 Bounded adversarial generation
WHEN a property campaign runs, THEN the runner SHALL pin generator version, seed, bounds, case bytes, shrink trace, and full expected envelopes.

#### Scenario: EU-CON-007 positive
- **WHEN** a seeded case mutates one nonce, domain, head, asset, argument, observation, or output allocation
- **THEN** replay reproduces the same verdict and minimal counterexample

#### Scenario: EU-CON-007 negative
- **WHEN** an unrecorded random ID or seed prevents exact replay
- **THEN** the receipt is incomplete and cannot pass conformance

### Requirement: EU-CON-008 Semantics-preserving metamorphic controls
WHEN a metamorphic case runs, THEN the runner SHALL check only transformations justified by the selected source semantics and compare complete transformed envelopes.

#### Scenario: EU-CON-008 positive
- **WHEN** an unrelated Witness Cash 5 row is added to an admitted prestate
- **THEN** Witness Cash 5 survives while the selected transfer and work match the independent expectation

#### Scenario: EU-CON-008 negative
- **WHEN** a test assumes balance-list reordering or effect reordering is semantic equivalence
- **THEN** the transformation is refused unless the versioned semantics prove that equivalence

### Requirement: EU-CON-009 Native control discrimination
WHEN a native failure is assessed, THEN the runner SHALL identify its reached ledger phase and verify complete post-failure state plus collateral exposure.

#### Scenario: EU-CON-009 positive
- **WHEN** a correctly signed wrong-state spend reaches phase two
- **THEN** the ledger script rejects and the authenticated head remains unchanged with collateral accounted separately

#### Scenario: EU-CON-009 negative
- **WHEN** a stale signature fails InvalidWitnessesUTXOW before script execution
- **THEN** the receipt remains a witness failure and cannot prove script rejection

### Requirement: EU-CON-010 Scoped review gates
WHEN a capability approval is requested, THEN the assessor SHALL require independent design, exact-candidate code, and actual-result reviews for that capability.

#### Scenario: EU-CON-010 positive
- **WHEN** two required independent design votes bind the exact revision03 candidate
- **THEN** only the reviewed T1 design scope is recorded as approved

#### Scenario: EU-CON-010 negative
- **WHEN** OpenSpec validation, source success, or a Stage00 graph vote is offered as private or twenty-family approval
- **THEN** approval is refused and missing scoped evidence remains explicit

### Requirement: EU-CON-011 Fail-closed certificate and private source
WHEN unsupported certificate or private syntax is offered to the current source profile, THEN source admission SHALL reject it without publishing a prepared transition.

#### Scenario: EU-CON-011 positive
- **WHEN** current /6 checks unsupported-certificate.mori and unsupported-private.mori
- **THEN** OPERATION_BINDING and UNKNOWN_DECLARATION are preserved as measured refusal envelopes

#### Scenario: EU-CON-011 negative
- **WHEN** host verified=true or an opaque certificate label attempts to bypass source rejection
- **THEN** no proof capability or private acceptance is recorded

### Requirement: EU-CON-012 Measured proof capability seam
WHEN a private feasibility probe is planned, THEN the investigator SHALL identify exact callable prover, verifier, ledger exports, versions, and the whole required relation before testing.

#### Scenario: EU-CON-012 positive
- **WHEN** a real pinned native verifier export and reproducible valid proof fixture exist
- **THEN** the probe names exact calls and measures success plus mutated-proof rejection

#### Scenario: EU-CON-012 negative
- **WHEN** only BLS primitives, a draft API, a mock verifier, or an unrelated proof is present
- **THEN** the probe records the missing relation or export and its engineering dependency

### Requirement: EU-CON-013 Whole private transition binding
WHEN a private proof is submitted to a future verifier, THEN the verifier SHALL bind keys, domain, program, instance, predecessor, nonce, full states, effects, intent caps, and invariants.

#### Scenario: EU-CON-013 positive
- **WHEN** the proof relation and public-input order bind every required field to the accepted ledger head
- **THEN** valid proof verification and ledger acceptance have independent exact-input receipts

#### Scenario: EU-CON-013 negative
- **WHEN** one key, opening, domain, field, public input, head, nonce, effect, or cap is substituted
- **THEN** the verifier or ledger rejects at a recorded reached layer without accepting the substituted transition

### Requirement: EU-CON-014 Proof malformed-input controls
WHEN a future native proof verifier receives malformed proof data, THEN the verifier SHALL reject wrong arity, aliases, malformed points, infinity, invalid openings, and surplus public inputs.

#### Scenario: EU-CON-014 positive
- **WHEN** the exact pinned verifier accepts its complete valid transition fixture
- **THEN** the baseline acceptance is retained independently of malformed controls

#### Scenario: EU-CON-014 negative
- **WHEN** a bit flip, wrong key, missing opening, aliased field, or surplus input is supplied
- **THEN** each control rejects through the actual verifier without host-computed success flags

### Requirement: EU-CON-015 Ledger-anchored PCD distinction
WHEN a PCD claim is assessed, THEN the assessor SHALL require authenticated predecessor consumption, immutable relation authority, bounded certificate checks, and ledger acceptance.

#### Scenario: EU-CON-015 positive
- **WHEN** a pinned ledger-anchored Moriarty relation extends one authenticated predecessor
- **THEN** the receipt records proof validity, head discipline, full effects, and actual ledger result separately

#### Scenario: EU-CON-015 negative
- **WHEN** public compatibility or standalone Nova or SlimNova arithmetic proof is offered as complete history compliance
- **THEN** PCD, mandatory Preview, and Complete remain unverified

### Requirement: EU-CON-016 Feasibility disposition and engineering duty
WHEN a required capability is absent, THEN the assessor SHALL distinguish a measured feasibility limit from unfinished implementation and retain a concrete closure task.

#### Scenario: EU-CON-016 positive
- **WHEN** an export inspection finds no Cardano whole-transition proof verifier for the selected profile
- **THEN** the result names the absent API, reproduction command, relation requirements, and engineering dependency

#### Scenario: EU-CON-016 negative
- **WHEN** missing code alone is offered as impossibility or permission to drop the family
- **THEN** the disposition is rejected and the inventory obligation remains open

### Requirement: EU-CON-017 Assessment and graph provenance
WHEN a final assessment or graph update is prepared, THEN the publisher SHALL bind actual evidence, preserve historical identities and dissent, and distinguish extraction from semantic review.

#### Scenario: EU-CON-017 positive
- **WHEN** a later separately authorized graph update uses reviewed frozen receipts
- **THEN** edges identify source locations, evidence tiers, actual Luna/Grok/Sol roles, and claim scope

#### Scenario: EU-CON-017 negative
- **WHEN** structural extraction or allPassed flags are interpreted as semantic or native verification
- **THEN** the graph claim is refused and the historical receipt remains intact

### Requirement: EU-CON-018 Versioned feature amendment
WHEN a new source or backend feature is proposed, THEN the designer SHALL document measured need, versioned source and compiled semantics, complete boundary controls, and independent design review.

#### Scenario: EU-CON-018 positive
- **WHEN** a missing private declaration or lifecycle operation needs a measured language capability
- **THEN** a new profile amendment preserves frozen /5 and /6 semantics with positive and failure tests

#### Scenario: EU-CON-018 negative
- **WHEN** a caller boolean or unreviewed policy hash-cycle change silently modifies reviewed T1 scope
- **THEN** adoption is refused pending scoped design review and exact-candidate checks
