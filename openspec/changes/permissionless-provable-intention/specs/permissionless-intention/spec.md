## ADDED Requirements

### Requirement: MOR-001 Permissionless deployment
When a developer submits a supported program with valid required evidence, the public toolchain SHALL permit deployment without project approval.

#### Scenario: Valid boundary
- **WHEN** A new program uses supported constructs and objective validation passes.
- **THEN** Deployment requires no program-registry entry or reviewer receipt.

#### Scenario: Invalid boundary
- **WHEN** The same program lacks required correctness evidence.
- **THEN** Objective verification rejects it regardless of developer reputation.

### Requirement: MOR-002 No workflow dependency
If project workflow metadata is absent, the public toolchain SHALL NOT reject solely because it is absent.

#### Scenario: Valid boundary
- **WHEN** A clean installation has no Foreman, RP03 or project metadata.
- **THEN** Compile, prove and submission interfaces remain available.

#### Scenario: Invalid boundary
- **WHEN** A candidate supplies a council receipt instead of a required proof.
- **THEN** Acceptance rejects the missing proof.

### Requirement: MOR-003 Bound proof relation
When effects are accepted, the verifier SHALL check contract, intent-refinement, transition and predecessor-history claims bound to exact artifacts through the established Core-to-target correspondence relation. Until that relation exists, documentation SHALL label semantic assurance unestablished rather than infer it from native proof verification.

#### Scenario: Valid boundary
- **WHEN** A proof binds program, semantics, signed intent, state, observations, effects and bounds.
- **THEN** Acceptance checks the specified relation.

#### Scenario: Invalid boundary
- **WHEN** The program, intent, predecessor or verifier relation is substituted.
- **THEN** Verification rejects the mismatched binding.

### Requirement: MOR-004 Formal intent fidelity
When a plan is proposed, its proof SHALL preserve signed asset, recipient, fee, liability, lifetime and outcome constraints.

#### Scenario: Valid boundary
- **WHEN** Concrete assets or explicit substitution predicates match the signed object.
- **THEN** The signing display and verification use that same object.

#### Scenario: Invalid boundary
- **WHEN** A solver substitutes a same-name asset outside the predicate.
- **THEN** The plan is rejected.

### Requirement: MOR-005 Open proposal sources
When equivalent valid candidates use different supported proposal paths, acceptance SHALL apply the same objective predicate.

#### Scenario: Valid boundary
- **WHEN** An independent solver and a manual builder produce valid candidates.
- **THEN** Both can use public verification and submission interfaces.

#### Scenario: Invalid boundary
- **WHEN** A relay credential is absent but native submission is valid.
- **THEN** The language imposes no relay-membership requirement.

### Requirement: MOR-006 Residual duties
While an obligation is pending or partially executed, the semantics SHALL retain its liability, authority usage and recovery rights.

#### Scenario: Valid boundary
- **WHEN** A permitted partial payment leaves debt outstanding.
- **THEN** The next state preserves the exact residual debt.

#### Scenario: Invalid boundary
- **WHEN** An external operation times out without nonexecution evidence.
- **THEN** The result remains unresolved rather than falsely settled or safely refunded.

### Requirement: MOR-007 Assumption and privacy reporting
Where a claim depends on external observations or private witnesses, the artifact SHALL identify its assumptions and disclosure boundary.

#### Scenario: Valid boundary
- **WHEN** A transition uses an attested external fact.
- **THEN** The report distinguishes conditional proof correctness from external truth.

#### Scenario: Invalid boundary
- **WHEN** An attestation is supplied instead of a mandatory transition proof.
- **THEN** Acceptance rejects the missing transition proof.

### Requirement: MOR-008 Compositional resources
When programs compose, the checker SHALL preserve consumed receipts, affine authority, persistent liabilities and cumulative costs according to their distinct rules.

#### Scenario: Valid boundary
- **WHEN** Unused revocable authority remains unused.
- **THEN** The checker does not force its exercise.

#### Scenario: Invalid boundary
- **WHEN** Composition drops debt or reuses a consumed receipt.
- **THEN** Validation rejects the invalid composition.

### Requirement: AEO-001 Trust and obligation inventory
When reporting an action, the tool SHALL bind versions and distinguish static discharge, runtime checks, finite tests and established correspondence.

#### Scenario: Valid boundary
- **WHEN** Proof/ledger correspondence is absent.
- **THEN** The report marks it unestablished.

#### Scenario: Invalid boundary
- **WHEN** A required assumption category is missing.
- **THEN** Report completeness fails rather than implying verification.

### Requirement: AEO-002 Exact advisory checking
When checking a supported refinement, the tool SHALL use the pinned runtime arithmetic and distinguish safety, definedness and feasibility.

#### Scenario: Valid boundary
- **WHEN** A bounded supported formula discharges under stated assumptions.
- **THEN** The status is encoding-established, not a ledger proof.

#### Scenario: Invalid boundary
- **WHEN** Assumptions are inconsistent or the solver times out.
- **THEN** The report exposes vacuity or timeout and makes no successful guarantee.

### Requirement: AEO-003 Replay and soundness
When claiming a counterexample, the checker SHALL replay it through the pinned evaluator.

#### Scenario: Valid boundary
- **WHEN** The decoded model reproduces the claimed violated judgment.
- **THEN** The report identifies a validated counterexample and exact bindings.

#### Scenario: Invalid boundary
- **WHEN** Replay disagrees, or a safety claim contradicts the pinned evaluator on the same assumptions and successful-execution judgment.
- **THEN** The checker fails its corpus and records the encoding as unsound; it does not override the runtime.

### Requirement: AEO-004 Bounded synthesis
Where synthesis is enabled, generated candidates SHALL preserve immutable specifications and pass the ordinary validation pipeline.

#### Scenario: Valid boundary
- **WHEN** A typed hole has a supported specification and fixed search budget.
- **THEN** Generated code is presented as a candidate with evidence.

#### Scenario: Invalid boundary
- **WHEN** A candidate improves fitness by weakening fees, limits or intent.
- **THEN** The candidate is rejected.

### Requirement: DEV-001 Internal delivery separation
While Pel runs project work, review and resource controls SHALL remain internal delivery conditions.

#### Scenario: Valid boundary
- **WHEN** A feature task changes a verified candidate.
- **THEN** The task reruns verification and independent review.

#### Scenario: Invalid boundary
- **WHEN** A public program lacks Pel receipts.
- **THEN** Its acceptance does not depend on those receipts.

### Requirement: MOR-009 Midnight ZKIRv3 execution
When a Moriarty contract is compiled for deployment, the toolchain SHALL emit or reproducibly derive a pinned ZKIRv3 artifact executable by the selected Midnight ledger and verifier.

#### Scenario: Actual target artifact
- **WHEN** a newly authored supported program completes the compiler pipeline
- **THEN** evidence binds source, Core, compiler, ZKIRv3 version/instructions, circuit/key and Midnight verifier

#### Scenario: Intermediate artifact only
- **WHEN** only TypeScript execution, K agreement or Compact source exists
- **THEN** the toolchain does not report completed Midnight compilation and execution

### Requirement: MOR-010 Certified primitive substitution
When an optimized primitive replaces a reference computation, its certificate SHALL establish host/reference equivalence, target constraint soundness/completeness and compositional preservation under explicit preconditions and resource semantics. Source producer checks and adversarial witness-side premises SHALL be discharged separately.

#### Scenario: Compositional certificate
- **WHEN** the certificate binds the reference, target fragment, implementation and assumptions
- **THEN** composition discharges each call-site precondition and preserves values, effects, rejection and declared work

#### Scenario: Witness-only evidence
- **WHEN** tests show correct witness generation but invalid satisfying witnesses have not been excluded
- **THEN** the artifact does not claim ZKIRv3 constraint soundness or certified substitution

### Requirement: MOR-011 Ledger phase effects
When settlement can fail after guaranteed effects occur, the proof statement SHALL bound and report those effects and fees separately from fallible effects. Signed intent SHALL specify per-phase authority, replay consumption, fee limits and remedies for each permitted failure transition.

#### Scenario: Fallible phase fails
- **WHEN** the selected Midnight phase layout preserves guaranteed effects after failure
- **THEN** the result retains those effects and applicable costs without claiming global rollback or successful intent completion

#### Scenario: Local atomicity assumed globally
- **WHEN** a lowerer assumes local evaluator rollback erases every ledger fee and effect
- **THEN** correspondence validation rejects that unsupported phase mapping

### Requirement: MOR-012 Complete accounting and persistent liabilities
When a transition is accepted, its bound effect domain and frame SHALL account for all per-asset debits, credits, fees, custody changes and authorized supply changes, while preserving a separate typed liability evolution equation.

#### Scenario: Partial discharge
- **WHEN** an accepted partial payment discharges only part of a liability
- **THEN** opening liability plus authorized creation and accrual minus discharge equals the retained closing liability

#### Scenario: Omitted effect
- **WHEN** a candidate omits an in-scope fee, reserve change or residual duty from its claimed effect set
- **THEN** correspondence and acceptance reject the incomplete accounting
