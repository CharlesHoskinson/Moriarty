## ADDED Requirements

Status: DRAFT / SPECIFIED-ONLY. These are Sherlock nonsecurity functional clauses. All scenarios are specified and unexecuted. The unchanged Opus UISEC draft is referenced separately in README; these clauses do not replace it.

### Requirement: UIREQ-001 Fixed interpreter relation
WHEN a supported program is supplied as input, the interpreter SHALL evaluate it using the same versioned transition rules used for other supported programs.

#### Scenario: Two distinct programs
- **WHEN** Factorial and list sum are loaded under the same interpreter configuration.
- **THEN** Each uses the unchanged machine definition; only program/declaration/input data differ.

### Requirement: UIREQ-002 Explicit supported profile
WHEN a program is admitted, tooling SHALL report the exact supported grammar and execution profile.

#### Scenario: First-order fragment
- **WHEN** A program uses named calls, UInt128, Bool, constructor and match expressions.
- **THEN** Its report names those constructs, numeric domains and bounds; higher-order closures/letrec remain explicit extensions.

### Requirement: UIREQ-003 Lexical call scope
WHEN a named call enters its body, the interpreter SHALL evaluate it under the declared parameter bindings and global function table.

#### Scenario: Caller-local preservation
- **WHEN** An outer let binds x=7 while a called function binds its own x=2 and returns 2.
- **THEN** The caller still reads x=7 after return.

### Requirement: UIREQ-004 Strict evaluation order
WHEN an expression has multiple evaluated operands or fields, the interpreter SHALL evaluate them left-to-right.

#### Scenario: Nested arguments
- **WHEN** add(double(2),increment(3)) executes.
- **THEN** The trace finishes double(2) before starting increment(3), returning 8.

### Requirement: UIREQ-005 Named recursive calls
WHEN a named function calls itself or another named function, the interpreter SHALL apply the ordinary call/return relation.

#### Scenario: Self and mutual recursion
- **WHEN** factorial(3), even(4), and odd(5) execute within declared bounds.
- **THEN** They return 6, true, and true without adding program-specific rules.

### Requirement: UIREQ-006 Exact scalar arithmetic
WHEN a scalar primitive executes, the interpreter SHALL produce its declared checked UInt128 or Bool outcome.

#### Scenario: Normal arithmetic
- **WHEN** add(2,3), sub(7,2), mul(3,4), and lt(2,3) execute.
- **THEN** Their results are 5, 5, 12, and true under the selected primitive table.

### Requirement: UIREQ-007 Typed inductive values
WHEN a declared constructor is evaluated, the interpreter SHALL produce a finite immutable value of its declared nominal type.

#### Scenario: Recursive list value
- **WHEN** Cons(1,Cons(2,Nil)) is built under ListUInt128 declarations.
- **THEN** The result has the stated ordered fields and recursive list type.

### Requirement: UIREQ-008 Selected match branch
WHEN a constructor match selects an arm, the interpreter SHALL evaluate that arm with its declared field bindings.

#### Scenario: Tree fold
- **WHEN** sum(Node(Leaf(4),Leaf(5))) executes.
- **THEN** Both leaves are visited through their selected cases and the result is 9.

### Requirement: UIREQ-009 Structural value meaning
WHEN an implementation shares an immutable subvalue physically, the interpreter SHALL preserve its structural observations and declared traversal accounting.

#### Scenario: Repeated subtree
- **WHEN** A tree has two occurrences of Leaf(4), represented either separately or by one shared node.
- **THEN** Its fold returns 8 and charges both semantic visits under the same traversal schedule.

### Requirement: UIREQ-010 Explicit pending return
WHEN a non-tail call is entered, the interpreter SHALL retain its pending caller computation in explicit frames.

#### Scenario: Factorial multiplication
- **WHEN** factorial(3) enters factorial(2).
- **THEN** The pending multiplication by 3 remains represented until the callee returns.

### Requirement: UIREQ-011 Complete pure result
WHEN a pure value returns with no remaining frames, the interpreter SHALL expose that value as a completed pure result.

#### Scenario: Completed versus pending
- **WHEN** The final list-sum return has value 6 and an empty frame stack.
- **THEN** The result is Halted(6), distinct from an intermediate Return(6) with remaining frames.

### Requirement: UIREQ-012 Explicit bounded outcome
WHEN a declared execution bound prevents the next computational step, the interpreter SHALL report the named exhausted bound.

#### Scenario: Finite recursive attempt
- **WHEN** An otherwise valid recursive run reaches its fuel limit after making progress.
- **THEN** The result is Exhausted(Fuel), with consumed work reported separately from an application return.

### Requirement: UIREQ-013 Versioned work observations
WHEN a semantic transition consumes work, tooling SHALL report that work under the selected versioned schedule.

#### Scenario: Same computation in chunks
- **WHEN** The same admitted computation is collected in one trace and in two technical chunks.
- **THEN** Its cumulative logical work is the sum of the same semantic step charges; host time, circuit cost and ledger fees remain separate quantities.

### Requirement: UIREQ-014 Complete checkpoint representation
WHEN a supported execution is checkpointed, its encoding SHALL retain the complete resumable machine state defined by its profile.

#### Scenario: Pending callee snapshot
- **WHEN** The future suspension extension snapshots 10+inner(3) while inner awaits a result.
- **THEN** The snapshot includes the caller frame, control, lexical contexts, reachable values, budgets and applicable financial prefix.

### Requirement: UIREQ-015 Pending-return resumption
WHEN a valid pending observation resumes a supported waiting callee, execution SHALL continue through its retained caller frames.

#### Scenario: Resume inner call
- **WHEN** In the future extension, inner(3) resumes and returns 4 inside 10+inner(3).
- **THEN** The machine returns 14; a false/not-yet-ready observation leaves the waiting checkpoint unchanged.

### Requirement: UIREQ-016 Chunk transparency
WHEN proof production inserts a technical chunk boundary, that boundary SHALL preserve source-visible execution observations.

#### Scenario: Boundary before return
- **WHEN** A pure run is chunked between callee return and caller addition.
- **THEN** It retains the same result, frame sequence, effects, duties and cumulative logical work, without introducing a financial commit.

### Requirement: UIREQ-017 Complete financial observation
WHEN the future financial extension reports a segment outcome, it SHALL expose the complete ordered financial observation defined by its adapter.

#### Scenario: First paid prefix
- **WHEN** Reserve 100/liability 40 pays 30, charges fee 2 and discharges liability 30 at an admitted prefix boundary.
- **THEN** The observation reports reserve 68, liability 10, gross debit 32 and recipient receipt 30, with residual duty 10.

### Requirement: UIREQ-018 Existing await-prefix preservation
WHEN existing Core/2 typed-await behavior is lowered to the universal machine, the adapter SHALL preserve its successful-prefix effects, duties and checkpoint atomicity.

#### Scenario: Await after partial payment
- **WHEN** The preceding successful prefix reaches a typed await.
- **THEN** Its committed prefix and residual duty remain the same as Core/2; exact universal Commit/Yield/Await opcodes remain an open design choice.

### Requirement: UIREQ-019 Local segment failure outcome
WHEN a future financial segment fails locally, the adapter SHALL distinguish its discarded tentative delta from the retained committed prefix, consumed work and residual duties.

#### Scenario: Second segment fails before commit
- **WHEN** After the 68-reserve/10-liability prefix, a later local segment fails before its financial commit.
- **THEN** The retained prefix still reports reserve 68 and liability 10, plus later consumed work; no global ledger rollback is inferred.

### Requirement: UIREQ-020 Target phase mapping
WHEN the compiler maps a financial segment to target phases, it SHALL report the exact correspondence between source outcomes and guaranteed/fallible phase outcomes.

#### Scenario: Permitted retained fee
- **WHEN** The selected target phase policy permits a guaranteed fee to remain after a fallible action fails.
- **THEN** The adapter reports that policy-specific fee and remaining duties separately from local segment rollback; concrete target evidence remains pending.

### Requirement: UIREQ-021 Declared model projection
WHEN a K trace is compared with Quint, the adapter SHALL identify its common domain and rule-to-action projection.

#### Scenario: Finite call-return comparison
- **WHEN** A pure arithmetic/list trace is compared with a bounded Quint instance.
- **THEN** The report names numeric bounds, field maps, hidden steps, state representation differences and unmatched finance/wait behavior; trace agreement is not called refinement.

### Requirement: UIREQ-022 Compiler semantic mapping
WHEN compilation claims support for a construct, its lowering SHALL have a named semantic correspondence obligation.

#### Scenario: New supported program
- **WHEN** A new nested-call/list program is compiled through the public pipeline.
- **THEN** The mapping covers returned values, defined failures, order, resource observations and applicable effects/duties; host evaluation does not supply missing circuit evidence.

### Requirement: UIREQ-023 Native relation inventory
WHEN the interpreter is connected to the native backend, the integration SHALL identify each model transition and its circuit/host/native consumer.

#### Scenario: Fixed relation across programs
- **WHEN** Factorial and tree fold are routed through a candidate native realization of the same Sherlock reference machine.
- **THEN** The inventory names dispatch, scalar, frame and data constraint owners, whether the realization is universal, per-program or a finite family, and the native recursion/custom-decider adapter; no VerifyProof or ledger10 prerequisite is introduced.

### Requirement: UIREQ-024 Optimization preservation
WHEN an optimization replaces a reference operation or representation, its proposal SHALL state a preservation predicate over all declared observations and costs.

#### Scenario: Frame or sharing optimization
- **WHEN** A candidate reduces frame storage or interns equal nodes.
- **THEN** Its record covers values, rejection/exhaustion, evaluation/effect order, boundaries, duties and resource schedule, with evidence and unresolved gaps separately listed.

### Requirement: UIREQ-025 Evidence-class report
WHEN an interpreter capability is reported, its evidence record SHALL identify predicate, source tree, profile, consumer, bounds, target and evidence class.

#### Scenario: Pure models only
- **WHEN** K and Quint source drafts are delivered without execution receipts.
- **THEN** They are reported as specified-only source artifacts; compiler/circuit correspondence, native proof acceptance and ledger qualification remain open.

### Requirement: UIREQ-026 Required Clean correspondence stage
WHEN Sherlock's reference execution design is prepared for compiler/native integration, the design SHALL include the required Lean/Clean semantic, representation and bounded-step correspondence stage.

#### Scenario: Recursive semantics and bounded step circuit
- **WHEN** The K reference fragment and Quint bounded abstraction are handed to the compiler/native design work.
- **THEN** The package includes recursive inductive Lean semantic-value definitions as a required task, a fuel-based step/run relation, a separate fixed-capacity Clean encoding/step-circuit task, source-pinned public reuse inventory, a privately retained Fenrir integration pin/evidence record and named unproved correspondence/backend gaps; the public package includes no private paths/code, and no existing Sherlock proof or Midnight exporter is claimed.
