## ADDED Requirements

Routing for new work: the temporary Gemini 4 Argon through AGY assignment in [AGENTS.md](../../../../../AGENTS.md) supersedes standing Opus routing below until Opus availability is observed. Historical Opus-authored requirements and frozen gates retain their original identities and scope.

Status: FINAL DESIGN CANDIDATE B / SPECIFIED-ONLY; pending fresh independent Muse nonsecurity and Opus security reviews and the existing decision rule. These are Sherlock nonsecurity functional clauses. All scenarios are specified and unexecuted. The unchanged Opus UISEC draft is referenced separately in README; these clauses do not replace it. UIREQ-001–026 retain their IDs; UIREQ-027–035 add missing atomic functional obligations. All 35 clauses and their scenarios are unexecuted Candidate B acceptance. Shared record/signature definitions are in [interfaces.md](../../interfaces.md). The canonical 160-clause language index and all inherited exits remain separate.

### Requirement: UIREQ-001 Fixed interpreter relation
WHEN a supported program is supplied as input, the interpreter SHALL evaluate it using the same versioned transition rules used for other supported programs.

#### Scenario: Two distinct programs
- **WHEN** Factorial and list sum are loaded under the same interpreter configuration.
- **THEN** Each uses the unchanged machine definition; only program/declaration/input data differ.

### Requirement: UIREQ-002 Explicit supported profile
WHEN a program is admitted, tooling SHALL report its exact supported grammar and versioned execution profile.

#### Scenario: First-order fragment
- **WHEN** A program uses named calls, UInt128, Bool, constructor and match expressions.
- **THEN** Its report names those constructs, numeric domains, schedule and code/data/frame/environment/serialization/helper-work bounds; closed-body and unique-function admission remain explicit implementation work, while higher-order closures/letrec remain extensions. AdmissionRejected and tooling Unsupported are distinguished from runtime outcomes.

### Requirement: UIREQ-003 Lexical call scope
WHEN a named call enters its body, the interpreter SHALL evaluate it under the declared parameter bindings and global function table.

#### Scenario: Caller-local preservation
- **WHEN** An outer let binds x=7 while a called function binds its own x=2 and returns 2.
- **THEN** The caller still reads x=7 after return.

### Requirement: UIREQ-004 Strict evaluation order
WHEN an expression has multiple evaluated operands or fields, the interpreter SHALL evaluate them once each from left to right.

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
WHEN an ordered constructor match completes its scrutinee, the interpreter SHALL evaluate the first matching source arm under its declared field bindings.

#### Scenario: Tree fold
- **WHEN** sum(Node(Leaf(4),Leaf(5))) executes.
- **THEN** Both leaves are visited through their selected cases and the result is 9; the scrutinee is evaluated once and unselected arm bodies do not execute.

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
WHEN a declared execution profile cannot perform its next transition within its runtime bounds, the interpreter SHALL publish the bound outcome defined by that profile.

#### Scenario: Finite recursive attempt
- **WHEN** An otherwise valid recursive run reaches its fuel limit after making progress.
- **THEN** The result is Exhausted(Fuel), with consumed work reported separately from an application return. Under sherlock-k-a/1 a frame push at capacity remains Rejected(error4); Candidate B new node/environment/helper-scratch/serialization capacity outcomes are named Exhausted(capacityKind). AdmissionRejected and tooling Unsupported remain outside the application outcome schema; their domain/refinement premises are pending.

### Requirement: UIREQ-013 Versioned work observations
WHEN a semantic transition consumes work, tooling SHALL report its resource observations under the selected versioned ResourceManifest.

#### Scenario: Same computation in chunks
- **WHEN** The same admitted computation is collected in one trace and in two technical chunks.
- **THEN** Its cumulative reference work is the sum of the same semantic charges; physical transitions, helper visits, logical frame depth, physical frame/environment bytes, preparation/serialization bytes, host time/memory, circuit rows/degree, proof/key bytes and ledger fees remain separate fields. sherlock-k-a/1 and sherlock-quint-a/1 retain their unequal terminal charges; sherlock-helper-microstep/1 is a new proposed schedule, without same-fuel equivalence or a measured cost winner.

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
- **THEN** The inventory names dispatch, scalar, frame and data constraint owners, whether the realization is universal, per-program or a finite family, its common bounded StepInput/StepOutput interface and the native recursion/custom-decider adapter; complete semantic StepRecord snapshots are produced separately by host materializeStep. No VerifyProof or ledger10 prerequisite is introduced.

#### Scenario: Stagenet end-to-end qualification
- **WHEN** Sherlock/Moriarty native target behavior is submitted for acceptance.
- **THEN** The evidence includes actual supported source compilation, proof production and independent verification, stagenet deployment and call, and complete state/effects readback against independent expectations under pinned network/toolchain/relation identities. Unperformed stages stay not run; this scenario grants no execution authority or transfer of Preview/Fenrir evidence.

### Requirement: UIREQ-024 Optimization preservation
WHEN an optimization replaces a reference operation or representation, its proposal SHALL state a preservation predicate over all declared observations and costs.

#### Scenario: Frame or sharing optimization
- **WHEN** A candidate reduces frame storage or interns equal nodes.
- **THEN** Its record covers values, rejection/exhaustion, evaluation/effect order, boundaries, duties and resource schedule, with evidence and unresolved gaps separately listed.

### Requirement: UIREQ-025 Evidence-class report
WHEN an interpreter capability is reported, its evidence record SHALL identify predicate, source tree, profile, consumer, bounds, target and evidence class.

#### Scenario: Pure models only
- **WHEN** Unchanged Candidate A model observations accompany the new Candidate B design.
- **THEN** The report identifies exactly the source-specific K frontend-only check and repaired scalar15 Quint tests/samples, then labels every new B helper/image/arithmetic/plan/residual/ADT acceptance scenario unexecuted. Old observations do not validate changed B semantics; compiler/circuit correspondence, native proof acceptance and ledger qualification remain open.

### Requirement: UIREQ-026 Required Clean correspondence stage
WHEN Sherlock's reference execution design is prepared for compiler/native integration, the design SHALL include the required Lean/Clean semantic, representation and bounded-step correspondence stage.

#### Scenario: Recursive semantics and bounded step circuit
- **WHEN** The K reference fragment and Quint bounded abstraction are handed to the compiler/native design work.
- **THEN** The package includes recursive inductive Lean semantic-value definitions as a required task, a fuel-based step/run relation, a separate fixed-capacity Clean encoding/step-circuit task, source-pinned public reuse inventory, a privately retained Fenrir integration pin/evidence record and named unproved correspondence/backend gaps. The existing pure Lean reference source and its eight normal plus exact-remaining-zero publication checks are historical narrow evidence; they do not implement B bounded images, Clean transition circuits, correspondence or exporters. The public package includes no private paths/code; no Midnight exporter is claimed.

#### Scenario: Iterate reference models and handwritten circuits
- **WHEN** A Sherlock transition slice or circuit optimization changes.
- **THEN** Its comparison package follows the [correctness loop](../../../../../docs/SHERLOCK-CORRECTNESS-LOOP.md), identifies K/Quint projections, Lean/Clean and handwritten component membership, independent complete-state/prefix expectations, first divergences and each missing/unrun layer. Observed model agreement does not supply constraint, theorem or native evidence; the workflow grants no execution authority.

### Requirement: UIREQ-027 Indexed program-image provenance
WHEN preparation indexes a supported source program, it SHALL produce a ProgramImage preserving source membership, original function-table order and SourceOccurrenceId provenance.

#### Scenario: Distinct occurrences and recursive references
- **WHEN** Two source sites contain the same expression and a named function recursively calls itself.
- **THEN** They retain distinct SourceOccurrenceId values while recursive calls remain function-table references; expanded implementation-step IDs map back to those sites. Ordered child/parameter slices and first-match lookup remain as in the source until a separately admitted unique-function profile applies; preparation neither evaluates the selected path nor unrolls recursion.

### Requirement: UIREQ-028 Bounded helper microsteps
WHEN the proposed sherlock-helper-microstep/1 profile performs helper work, it SHALL advance through explicit helper controls with a declared finite bound per transition.

#### Scenario: Lookup and equality cursors
- **WHEN** A lookup crosses two ordered entries or equality visits two occurrences of a shared subtree.
- **THEN** Function/environment lookup, binding, validation, argument/field accumulation, equality and match cursors expose each bounded helper transition and complete resume state. The proposed schedule charges one per completed legacy machine transition and one per helper transition, with startup, terminal publication and error publication zero-cost; code/data/environment/frame/serialization/helper-work capacities are declared. The mapped A state remains at the origin until the helper interval completes. W_B = W_A + H_B is only a proposed completed-interval mapping under common-domain/cap premises, never a same-fuel or implemented-refinement claim.

### Requirement: UIREQ-029 Bounded representation domain
WHEN a ReferenceState is offered to the bounded representation adapter, it SHALL return a MachineImage Bounds F related by RepresentationRelation or an explicit out-of-domain result.

#### Scenario: Complete pending-state encoding
- **WHEN** An in-domain factorial state retains a pending multiplication, caller environment and a helper cursor.
- **THEN** Partial encode/decode retain code references, ordered frames, control, reachable structural values, active lengths, fuel/work, outcome and helper state under the selected field/layout/capacities. A value outside those capacities has no represented-image claim; immutable sharing unfolds to structural occurrences and physical identity is not source equality. Encode/decode and circuit correspondence remain unimplemented/unproved.

### Requirement: UIREQ-030 Typed arithmetic component API
WHEN arithmetic lowering selects a word layout, it SHALL expose named full-result components with the exact checked UInt128 semantic wrapper.

#### Scenario: Carry, borrow and full product
- **WHEN** b is 32 or 64 and checkedU128Add/Sub/Mul use little-endian limbs.
- **THEN** addCarry_b(a,bv,carryIn) returns named {low,carryOut}, subBorrow_b(a,bv,borrowIn) returns {low,borrowOut}, and mulWide_b(a,bv) returns {low,high}; carry/borrow bits and multiply-accumulate whole-word carries have distinct types. The wrappers retain K mathematical-result/error10 meaning, including the full up-to-256-bit product before checked UInt128 selection. Independently specified normal results are (2^32−3)+8 = 2^32+5, (2^64+9)−7 = 2^64+2, and (2^48+1)·3 = 3·2^48+3; no layout or row-cost winner is selected.

### Requirement: UIREQ-031 Separate compilation/export plans
WHEN a bounded step is prepared for a backend, tooling SHALL report separate ConstraintPlan and WitnessPlan artifacts with their BackendCapabilities disposition.

#### Scenario: Structured witness export and unsupported features
- **WHEN** One step uses structured witness instructions and a requested export also encounters a native witness closure, lookup or interaction.
- **THEN** OperationManifest identifies typed operations, ordered inputs/outputs, source/component membership and omissions; constraint export, witness-program export, lookup/interact/custom-gate support and field/layout support are listed separately. Unsupported features yield tooling Unsupported, with no claim that basic constraint JSON supplies witness computation or that any non-Midnight format is a Midnight/ZKIR exporter.

### Requirement: UIREQ-032 Earliest-stop reference slices
WHEN multiple reference transitions share one physical step or chunk, the implementation SHALL expose the ordered ReferenceSlice through its earliest reference stop.

#### Scenario: Grouped pending return
- **WHEN** A candidate groups up to 1, 2, 4 or 8 reference transitions around a callee return under the same schedule/bounds.
- **THEN** StepRecord preserves each internal position, rule/source occurrence, prefix charge, logical frame check and first outcome. Under sherlock-k-a/1 zero-fuel terminal/error publication and frame-cap Rejected(error4) keep their original precedence; padding adds physical work only. Technical cuts retain complete resumable state and create no financial commit or movement of an existing Core/2 await prefix.

### Requirement: UIREQ-033 Bounded static residualization
WHEN a reference step is specialized, the specializer SHALL produce bounded ResidualStepIR preserving every dynamic reference boundary.

#### Scenario: Three realization masks
- **WHEN** Empty, fixed-family or fixed-ProgramImage static bindings are applied to the same reference step.
- **THEN** Only immutable code/declarations/layout facts are folded; runtime control, data, environments, frames, fuel/work, capacity checks and outcomes remain dynamic. A finite worklist and residual-size limit fall back to generic dispatch without application exhaustion. Recursion remains runtime calls, with no depth/fuel/local-value specialization or source recursion unrolling; rule/source-occurrence maps retain intermediate stops and charges in universal, family and per-program realizations.

### Requirement: UIREQ-034 Declaration-driven ordered matching
WHEN the generic ADT extension elaborates an admitted closed nominal declaration group, it SHALL produce TypeDecl/CtorLayout/MatchPlan mappings preserving the first matching source arm.

#### Scenario: New recursive Schedule declaration
- **WHEN** Schedule = Done | Pay(UInt128,Schedule) | Both(Schedule,Schedule) is matched by ordered arms Both(Pay(x,_),_)→x; Both(_,r)→sum(r); Pay(x,t)→x+sum(t); Done→0 on Both(Pay(4,Done),Pay(5,Done)).
- **THEN** The selected arm is source arm one and the result is 4; a separate structural sum is 9. Nominal/constructor identity, source field order, binder occurrence paths and workMap are retained independently of compact tags or arena indices. Closed monomorphic mutually recursive groups are the initial extension; parameterized specialization needs its own finite bound, and closures/GADTs/or-patterns remain outside this adapter. Existing dedicated list/tree forms do not count as this extension's implementation.

### Requirement: UIREQ-035 Deterministic witness dataflow
WHEN a WitnessPlan is evaluated, its baseline SHALL execute a typed dependency DAG in a deterministic uncached order to materialize complete StepRecords.

#### Scenario: Independent normal comparison baseline
- **WHEN** The same admitted factorial/list/tree or pending-caller state is materialized without caching, with cold caches and with warm caches.
- **THEN** Stable dependency ordering and explicit bindings produce identical decoded reference prefixes, ordered frames/environments, outcomes and reference-work observations against independently authored expectations. Cache/lifetime/streaming variants retain the uncached baseline; reusable keys include exact program/profile/layout and immutable inputs, while helper visits, copies, live values and memory remain separate diagnostics. Neither those expectations nor semantic behavior are generated from the materializer/DAG metadata.
