# Sherlock Candidate B interface contract

Status: FINAL DESIGN CANDIDATE B / SPECIFIED-ONLY unless an evidence row explicitly says otherwise. These are proposed engineering interfaces, not implemented APIs or accepted correspondence results. This file owns shared names and field meanings used by [design.md](design.md), requirements and the implementation plan. Security requirements, theorem statements, tests and judgments remain with Opus in the unchanged [UISEC document](security-requirements.md) and separate [Candidate B delta](security-candidate-b.md); interface completeness does not assess them.

## Identities, profiles and preparation

`Nat` below is a mathematical natural; every stored index/counter has a width and maximum in `LayoutSchema`. `Id` is a namespace-qualified identifier, not an address or cryptographic commitment. `Digest` names exact source/artifact membership under a separately declared digest format. No digest or hash algorithm is selected by this contract. `Result<A,E>` and `Option<A>` describe partial engineering functions.

```text
ProfileId = sherlock-k-a/1 | sherlock-quint-a/1 | sherlock-helper-microstep/1
SourceOccurrenceId = {programSourceDigest, owner: Entry | FunctionTableIndex(Nat), astPath:[Nat]}
ExpandedStepId = {runId, ordinal:Nat}
ExprId, FunctionRowId, BinderId, TypeId, CtorId, FrameId, EnvRoot, NodeRef : distinct Id types
Bounds = {codeRows, functionRows, codeChildSlots, parameterSlots, patternRows,
          matchPlanRows, matchArmRows,
          typeRows, constructorRows, constructorFieldSlots, maxArity,
          constantNodes, constantFieldSlots,
          dataNodes, dataFieldSlots, environmentCells, frames, helperTasks,
          serializationBytes, maxReferenceWork, maxHelperWork : Nat}
LayoutSchema = {layoutVersion, fieldId, indexWidths, counterWidths,
                scalarLayoutId, recordWidths, bounds:Bounds}
ProgramImage = {programSourceDigest, profileId, layoutVersion, bounds,
                expressions:[ExprRow], orderedChildren:[ExprId],
                functions:[FunctionRow], orderedParameters:[Parameter],
                constants:[ConstantRow], constantFields:[ConstantField],
                sourceMap:ExprId -> SourceOccurrenceId,
                typeDecls:[TypeDecl], ctorLayouts:[CtorLayout], matchPlans:[MatchPlan],
                entry:ExprId, membershipDigest}
ExprRow = {id:ExprId, sourceOccurrenceId:SourceOccurrenceId, tag,
           childSlice, payload}
FunctionRow = {rowId:FunctionRowId, sourceTableIndex:Nat, functionName:Id,
               parameterSlice, returnType:TypeId, body:ExprId}
ConstantRow = {constantId,typeId,ctorId,scalarPayload,orderedFieldSlice}
PreparationFailure = AdmissionRejected(reason,sourceOccurrenceId?)
                   | UnsupportedFeature(feature,locator) | CapacityNotAdmitted(kind,needed,limit)
prepare(program,profileId,bounds,layout)
  -> Result<(ProgramImage,OperationManifest,PreparationReport),PreparationFailure>
fetch(image:ProgramImage,id:ExprId) -> Option<ExprRow>
```

Array notation in `ProgramImage` means bounded active prefixes of fixed-capacity tables in a realized image. Literal constants use bounded immutable rows/ordered field slots, not recursively embedded circuit values. The circuit-facing view uses bounded numeric row/occurrence indices; source paths, digests and reports are separately materialized metadata with declared serialization capacity. `PreparationReport` records source and image membership, used/capacity counts, padding, bytes, preparation work and unsupported features separately from execution work. Preparation traverses the entire admitted program, including unused branches. It does not evaluate application expressions or unroll source recursion. Every syntactic occurrence has a distinct path even when identical subexpressions share a physical record. Repeated execution of one occurrence creates distinct `ExpandedStepId`s. A generated helper/virtual row retains its originating occurrence plus its own expansion index; the two identities are never interchangeable.

Function rows retain original order. Lookup selects the minimum matching `sourceTableIndex`, preserving the existing K behavior even while a whole-program unique-name/closed-body admission checker remains unimplemented. A future unique-name profile may use a direct index only after that premise is checked. Parameter/argument, constructor-field and match-arm order remain source order. Source IDs and compact layout tags remain distinct. Programs sharing function names but differing source bytes have different image/cache identities. A change to layout, bounds, profile or source invalidates preparation reuse.

## Reference state and bounded helpers

```text
SemanticValue = U128(Nat) | Bool(Bool) | Constructor(TypeId,CtorId,[SemanticValue])
Environment = finite map BinderId -> SemanticValue
Outcome = Running | Halted(SemanticValue) | Rejected(errorCode)
        | Exhausted(Fuel | DataNodes | DataFields | EnvironmentCells
                    | HelperTasks | HelperWork | SerializationBytes)
Control = Eval(ExprId) | Return(SemanticValue) | Schedule(frame,next,environment)
        | Invoke(functionName,arguments) | Enter(functionRow,arguments)
        | Cases(value,armCursor,savedEnvironment) | Primitive(op,operands)
        | Helper(HelperControl) | Error(errorCode)
ReferenceState = {programSourceDigest, profileId, layoutVersion, control,
                  controlOrigin:ExprId,
                  environment:Environment, frames:[Frame], values:[SemanticValue],
                  outcome:Outcome, limits:Bounds, remainingWork:Nat, consumedWork:Nat,
                  helperWork:Nat,
                  allocation:{dataNodes,dataFields,environmentCells},referenceCursor:Nat}
Frame = PrimLeft(op,right,savedEnvironment) | PrimRight(op,left,savedEnvironment)
      | Let(binder,body,savedEnvironment) | If(thenExpr,elseExpr,savedEnvironment)
      | Arguments(functionName,completedValues,remainingExprs,savedEnvironment)
      | ConstructorFields(typeId,ctorId,completedValues,remainingExprs,savedEnvironment)
      | Match(orderedArms,savedEnvironment) | Scope(savedEnvironment)
      | ReturnToCaller(expectedType,savedEnvironment)
HelperControl = Find(functionName,rowCursor,resume)
              | EnvGet(binder,cellCursor,resume)
              | EnvPut(binder,value,environmentRoot,resume)
              | Bind(parameterCursor,argumentCursor,partialEnvironment,phase,resume)
              | Append(sourceCursor,destinationCursor,resume)
              | Validate(occurrenceTodo,expectedType,resume)
              | Equal(orderedPairTodo,resume)
              | Match(armCursor,patternTodo,partialBindings,savedEnvironment,resume)
HelperResume = {originRuleId,sourceOccurrenceId,originReferenceCursor,
                savedControl,savedEnvironment,resultSlot,phase}
```

The first implemented model fragment has dedicated list/tree constructors, not this general declaration table. The proposed B reference retains their meanings. `frames` are ordered bottom-to-top; the final element is the next return context. Each frame saves the complete semantic environment it needs, including shadowed locals. Physical liveness/slot analysis may compact this payload only if decoding reconstructs those fields. Calls evaluate arguments left-to-right, push the pending return and install a fresh parameter environment; return restores the caller environment. Physical activation reuse retains every logical frame and its original cap check. Source values are immutable finite terms; DAG references unfold to structural occurrences, with no source address equality or cycles.

One helper transition inspects at most one function row, environment cell, parameter/argument pair, append cell, value occurrence, equality pair or pattern occurrence, then advances its stored cursor. A step may manipulate only a fixed maximum of bounded-width fields under the selected layout; fields/children exceeding that fixed payload are visited in subsequent cursor steps. Binding phases expose duplicate-binder scanning, one binding insertion and type checking; no recursive duplicate search is hidden in `Bind`. Validation and equality preserve ordered occurrence tasks; repeated references create repeated tasks. Match tries arms in order, with fresh tentative bindings discarded on failure. Type/declaration traversal, variable lookup, argument accumulation and structural equality may not call unmetered recursive helpers in B. Fixed-width UInt128 arithmetic is a bounded primitive, independently of variable-size structural traversal.

The initial fixed-step contract is relative to a frozen `Bounds` and field/layout configuration. It does not promise constant cost across capacities or claim that an indexed read is free in a circuit. No whole-state `decode` or whole-value revalidation is allowed as an implicit implementation of a B step. Encoding/decoding is a separate bounded traversal interface; its host work is reported independently. Actual bounded helper rules, payloads and rule inventory remain implementation tasks.

`controlOrigin` follows the source expression that scheduled the current operation; helper/resume/frame records retain it through returns, errors and terminal publication. It begins at the entry expression. Physical virtual rows use their own indices alongside that origin.

### Named schedules and outcomes

| Profile | Charged work | Publication and capacity behavior |
|---|---|---|
| `sherlock-k-a/1` | Exactly the existing K machine-section schedule; pure helpers unmetered | Startup/final-value/error publication zero; nonterminal zero fuel exhausts; stack-limit push rejects error4 |
| `sherlock-quint-a/1` | Existing algorithm-specific ticks, including terminal-return tick | Observation/stutter uncharged; its scalar/frame/fuel instance and attemptedWork are separate |
| `sherlock-helper-microstep/1` | Each completed legacy machine transition costs 1; each explicit helper transition costs 1 | Startup and terminal/error publication zero; nonterminal zero fuel exhausts; legacy frame-cap error4 retained; added dynamic image capacities exhaust by the named kind |

B decomposes a legacy rule into bounded helper work and a separately identified completion event carrying that legacy rule ID. Entering/finishing a helper is an explicit helper transition if not combined with one bounded helper visit or the legacy completion. Empty/missing scans and helper failure exits likewise have explicit finite events. The complete rule inventory must enumerate these events before implementation claims or exact-fuel oracles. Zero-cost recursive helper closure is prohibited in B. Failure precedence is: publish already-final value/error first; otherwise zero fuel yields `Exhausted(Fuel)`; otherwise select the charged rule, perform its existing logical frame-cap check, then the declared data/env/helper-cap check before its update. A frame-cap error4 is a charged failing push followed by free error publication, as in A. Serialization capacity refusal occurs at a requested encoding/checkpoint operation, not spontaneously inside pure evaluation.

`eraseHelpers` maps an in-progress B helper state to its saved legacy pre-state and maps a completed legacy event to its legacy post-state. For a fully mapped interval with identical admitted inputs, no intervening capacity failure and completed helper work, the proposed accounting equation is `W_B = W_A + H_B`; it is a correspondence target, not a proved result. Equal numeric fuel does not imply equal stopping points. A-preserving physical helper rewrites retain `W_A` and count helper visits only in diagnostics; they are not B metering. Added admission/image capacities narrow a domain explicitly. `AdmissionRejected`, `CapacityNotAdmitted` and backend `UnsupportedFeature` are tooling outcomes outside `Outcome`; none may masquerade as an application result or a legacy exhaustion.

The proposed B allocation baseline is monotone for a run: one logical data-node unit for each newly produced scalar/constructor result and its ordered field-slot count, one logical environment-cell unit per successful binding insertion, and reusable indexed frame/helper-task slots governed by logical depth/live-task counts. Prepared constants occupy separate immutable capacities; a literal reference need not copy its whole value. These allocation counters are retained independently of physical interning, map overwrite, slot reuse or DAG sharing. A different reclaiming/logical allocation policy needs an explicit profile decision. A layout must accommodate the admitted logical envelope; extra physical padding/bookkeeping cannot cause an earlier source exhaustion. If it cannot represent that envelope, preparation/realization reports the unsupported capacity domain. Exact per-rule allocation rows remain required alongside charge rows.

`maxHelperWork` independently bounds completed explicit helper events in B; `helperWork` begins at zero and increases by one per completed helper event (including a declared entry/exit event). The pre-work check refuses an event that would exceed this limit with `Exhausted(HelperWork)`. This is distinct from live `helperTasks` storage and total `consumedWork`, which includes legacy completions and charged failure events. `helperVisits` records actual record/occurrence inspections separately; the rule inventory names zero-visit entry/exit events. Physical cache hits must retain those logical event/visit counts. No helper-work counter is retroactively added to A's semantic schedule.

## Bounded image and representation

```text
MachineImage (bounds:Bounds) (F:Field) = {
  identity, controlTag, controlPayload, controlOrigin, activeLengths, environmentRoot,
  code:Vector CodeRow bounds.codeRows,
  data:Vector DataRow bounds.dataNodes,
  dataFields:Vector FieldSlot bounds.dataFieldSlots,
  environments:Vector EnvCell bounds.environmentCells,
  frames:Vector FrameRow bounds.frames,
  helperTasks:Vector HelperTask bounds.helperTasks,
  remainingWork, consumedWork, helperWork, allocationCounters, referenceCursor, outcomeTag, outcomePayload}
ProgramImageView (bounds:Bounds) (F:Field) = {
  identity, activeLengths, entryExprId,
  expressions:Vector BoundedExprRow bounds.codeRows,
  functions:Vector FunctionRow bounds.functionRows,
  children:Vector ExprId bounds.codeChildSlots,
  parameters:Vector ParameterRow bounds.parameterSlots,
  patterns:Vector PatternRow bounds.patternRows,
  matchPlans:Vector PlanNodeRow bounds.matchPlanRows,
  matchArms:Vector ArmRow bounds.matchArmRows,
  types:Vector TypeRow bounds.typeRows,
  constructors:Vector CtorRow bounds.constructorRows,
  constructorFields:Vector CtorFieldRow bounds.constructorFieldSlots,
  constants:Vector ConstantRow bounds.constantNodes,
  constantFields:Vector ConstantField bounds.constantFieldSlots}
BoundedExprRow = {tag,sourceOccurrenceIndex,childStart,childLength,fixedPayload}
programView(program:ProgramImage,layout:LayoutSchema)
  -> Option<ProgramImageView layout.bounds F>
encode(program:ProgramImage,state:ReferenceState,layout:LayoutSchema)
  -> Option<MachineImage layout.bounds F>
decode(program:ProgramImage,image:MachineImage bounds F,layout:LayoutSchema)
  -> Option<ReferenceState>
RepresentationRelation(program,layout,state,image) : Prop
```

`identity` names the program/profile/layout/bounds and field. `ProgramImageView` rows contain bounded tags, IDs, scalar payloads and slices; source paths/full reports remain host metadata. `LayoutSchema.recordWidths` fixes every row/payload width and the table inventory fixes each active length; a configuration is incomplete until those widths and tag cases are inventoried. Program tables may be physically separate immutable inputs; the logical `code` view and its identity must be available identically to each realization. Scalars have an explicit exact-integer limb interpretation; arbitrary field residues are not UInt128. Data rows carry nominal type/constructor IDs and ordered typed field slices; environment cells preserve bindings/shadow order; frame rows preserve ordered payloads. Active lengths and unused slots have a declared encoding policy. The decoder is partial for out-of-domain tags, indices, scalars, cyclic data, missing reachable state or layout mismatch. The engineering relation's intended domain is `decode(...) = Some(state)` with identity/capacity agreement; it is not a theorem about constraints. Encode/decode round trips, admitted-domain coverage and step correspondence are separate open obligations; no total encoding of all recursive values into a fixed image is proposed.

## Arithmetic component boundary

```text
Word_b = {n:Nat | n < 2^b}; Bit = {0,1}; b in {32,64}
addCarry_b(a:Word_b,bv:Word_b,carryIn:Bit) -> {low:Word_b,carryOut:Bit}
subBorrow_b(a:Word_b,bv:Word_b,borrowIn:Bit) -> {low:Word_b,borrowOut:Bit}
mulWide_b(a:Word_b,bv:Word_b) -> {low:Word_b,high:Word_b}
checkedU128Add(a:UInt128,b:UInt128) -> Value(UInt128) | Error(10)
checkedU128Sub(a:UInt128,b:UInt128) -> Value(UInt128) | Error(10)
checkedU128Mul(a:UInt128,b:UInt128) -> Value(UInt128) | Error(10)
```

The defining mathematical equations are `a+bv+carryIn = low + 2^b*carryOut`, `a-bv-borrowIn = low - 2^b*borrowOut`, and `a*bv = low + 2^b*high`. Named fields avoid high/low tuple-order ambiguity. UInt128 limbs are little-endian: `value = sum(limb[i]*2^(b*i))`. Checked wrappers select the exact mathematical result iff it lies in `[0,2^128)`; subtraction underflow/addition overflow/multiplication high128 nonzero yield legacy error10. A multiply-accumulate word carry is a distinct `Word_b` output, never a `Bit` carry. Full multiplication retains all 256 product bits before outcome selection. Division and financial rounding are additional semantic APIs, not implied by these three wrappers.

The byte-based public Clean representation is the comparison baseline. Four public U32 values and two public U64 values both occupy sixteen byte-valued fields; two/four packed-field layouts are new alternatives. This is an inventory observation, not a row estimate. No limb layout or primitive implementation wins without complete-path measurements and the separately owned component review.

## Trace, slices and realization

```text
StepRecord = {stepId:ExpandedStepId, sourceOccurrenceId, ruleId, helperKind?,
              profileId, layoutVersion, pre:ReferenceState, post:ReferenceState,
              workDelta:Nat, helperVisits:Nat, logicalDepthBefore, logicalDepthAfter,
              outcome, boundary: None | Terminal | TechnicalCut | SourceSuspension}
ReferenceSlice = {profileId, start:ReferenceState, records:[StepRecord],
                  prefixWork:[Nat], boundaries:[{position,kind}],
                  end:ReferenceState, stop:None | Outcome | TechnicalCut | SourceSuspension}
Family = Schedule | Scalar | CallBind | FrameReturn | DataMatch
StepPlan = {referenceDefinitionId,profileId,layout,realization,
            operationManifestId,residualStepId?}
StepInput = {program:ProgramImageView bounds F,state:MachineImage bounds F}
StepEvent = {stepOrdinal,sourceExprId:ExprId,ruleId,helperKind?,workDelta,
             boundary,outcomeTag,outcomePayload}
StepOutput = {next:MachineImage bounds F,event:StepEvent,diagnostics}
stepReference(program:ProgramImage,state:ReferenceState) -> StepRecord
realizeStep(plan:StepPlan,input:StepInput)
  -> Result<StepOutput,UnsupportedFeature>
materializeStep(program,layout,preImage,output:StepOutput) -> Option<StepRecord>
advance_m(plan:StepPlan,image:MachineImage bounds F,m:Nat)
  -> Result<ReferenceSlice,UnsupportedFeature>
```

`stepReference` is the specification function on admitted compatible states; normal invalid input is a preparation failure. `StepInput`/`StepOutput` use fixed bounds and the bounded circuit-facing `ProgramImage` view; semantic snapshots in `StepRecord` are a separate host materialization, never unbounded circuit fields. Event `sourceExprId` resolves through the immutable source map, including helper origin; every event/diagnostic payload has a fixed width in the layout. Record identities and source locations do not themselves constitute correctness evidence. An implementation may store state references/deltas in place of duplicate snapshots only with a complete materialization interface. `prefixWork` has `records.length+1` entries, begins at zero and accumulates actual deltas. Consecutive records have equal post/pre states. Ordered records remain recoverable if physical events are partitioned by family. Classification uses control phase and top frame, not merely source opcode.

`advance_m` attempts at most `m` reference events, initially compared at 1/2/4/8. It executes every internal fuel/cap/outcome check in order, stopping at the earliest reference outcome or source suspension, never executing a later prefix then charging retrospectively. Existing terminal publication may produce a zero-charge record even when remaining fuel is zero. Padding is physical, never a reference event. A technical cut retains all pending state and cumulative work and does not produce `Waiting`, an effect or a financial commit. A storage interface `ChunkStore.put/get` serializes the complete image, identity and cursor within its declared byte capacity; its I/O is separate from logical work. Financial suspension fields are reserved extension work below, not present pure-machine claims.

## Witness, constraints and exports

```text
OperationManifest = {manifestId,referenceDefinitionId,profileId,layoutVersion,
  operations:[{operationId,domain,ordering,sourceLocators,witnessDependencies,
               constraintComponentLocator?,witnessProgramKind,
               supportedStatus:Specified | Implemented | Unsupported(reason)}]}
WitnessPlan = {planId,operationManifestId,layoutVersion,inputBindings,
               nodes:[{nodeId,operationId,typedInputs,typedOutputs,dependencies,lastUse}],
               stableTopologicalOrder,outputBindings,slotCounts,programKind}
ConstraintPlan = {planId,operationManifestId,layoutVersion,fieldId,
                  orderedPublicInputs,orderedPrivateInputs,orderedOutputs,
                  components:[{componentId,operationId,sourceLocator,parameters}],
                  requiredCapabilities,omissions}
BackendCapabilities = {backendId,sourceMembership,fieldId,formats,
                       constraintOperations,witnessOperations,
                       lookup,interaction,customGates,structuredWitnessIR,nativeClosures}
ResourceManifest = {referenceScheduleId,ruleInventoryDigest,programImageId,
  layoutVersion,bounds,loweringDigest,frameBytesByFunction,helperScratchBounds,
  referenceWork,helperWork,helperVisits,physicalEvents,logicalFramePeak,physicalFrameBytesPeak,
  activeCells,paddedCells,preparationBytes,serializationBytes,
  hostTime,hostPeakMemory,ioBytes,circuitRows?,degree?,keyBytes?,proofBytes?,
  proverTime?,verifierTime?,targetFees?,measurementScope,unknownMetrics}
compileStep(layout,componentInventory)
  -> Result<(ConstraintPlan,WitnessPlan),UnsupportedFeature>
evaluateWitness(plan:WitnessPlan,bindings,cacheScope:None | ExactRunScope)
  -> Result<(CandidateRows,HostDiagnostics),WitnessFailure>
emitConstraints(plan:ConstraintPlan,capabilities:BackendCapabilities)
  -> Result<ConstraintArtifact,UnsupportedFeature>
emitWitness(plan:WitnessPlan,capabilities:BackendCapabilities)
  -> Result<StructuredWitnessArtifact,UnsupportedFeature>
```

Start with a typed dependency DAG, deterministic stable topological order and uncached evaluation. Record one producer and ordered consumers per value, including the lifetime/last-use boundary. Witness operations and constraint components are separately inventoried; generated metadata may provide tags/locators but cannot generate both semantics and its independent oracle. Native closures, missing operations, unsupported fields/lookup/interact or unexportable witness programs return a located `UnsupportedFeature`. Constraint JSON alone is not a witness plan. `rowGenerated`, cache hit and host equality checks belong to diagnostics. The security interpretation of accepted rows/components remains Opus-owned.

Initial operation IDs are `FieldAdd`, `FieldMul`, `AssertZero`, `LimbExtract`, `CheckedU128Add`, `CheckedU128Sub`, `CheckedU128Mul`, `Select`, `ArenaRead`, `FrameRestore`, plus the helper event classes. These are coverage names; they are not declarations that Clean/Midnight realizes them. Streaming and lifetime reuse follow the uncached baseline. Static preparation caches use exact source/profile/layout/bounds; dynamic caches additionally use immutable input bindings and run scope. Cache reuse preserves each reference occurrence, frame, charge and stop. Resource metrics distinguish unavailable from zero; streamed rows do not establish bounded prover memory. Cost-based shape selection uses later calibrated estimates and never changes reference outcome.

## Bounded static residualization

```text
SpecializeConfig = {layout,staticMask,family:Option<Family>,residualSizeLimit:Nat}
ResidualStepIR = {irId,referenceDefinitionId,profileId,programImageId?,layoutVersion,
                  blocks,dynamicInputs,staticBindings,ruleBoundaryMap,
                  sourceOccurrenceMap,genericFallbacks,omissions}
specialize(referenceDefinitionId,image:Option<ProgramImage>,config:SpecializeConfig)
  -> ResidualStepIR
```

Use a finite memoized worklist keyed by reference-rule location, source occurrence and finite static layout facts. Static facts are immutable code/declarations, operand slices, layout and optional family choice. Control, input data, environment contents, frames/depth, fuel/work, helper cursor, outcomes and future financial state stay dynamic. Fold decoding/static dispatch only. Both successors of a dynamic condition remain. At the residual-size limit emit generic dispatch rather than application exhaustion. Do not specialize on recursion depth, fuel or runtime values, unroll source calls, delete logical frames/cap checks, pre-evaluate source expressions or merge charged boundaries. Empty image bindings yield universal, a fixed image yields per-program, and a fixed family yields family residualization of the same relation. Circuit cost/adoption remains unmeasured.

## Declaration-driven ADT extension and financial boundary

```text
TypeDecl = {nominalId:TypeId,parameters,recursiveGroup,constructors:[CtorDecl]}
CtorDecl = {ctorId:CtorId,sourceOrder,fields:[{fieldId,type}]}
CtorLayout = {nominalId:TypeId,ctorId:CtorId,tag,fields:[{fieldId,type,slotKind}],width}
MatchPlan = {sourceMatchId:SourceOccurrenceId,scrutineeType:TypeId,
             nodes:[PlanNode],entry,armTable,workMap}
PlanNode = Switch(occurrencePath,cases,default) | Bind(binderId,occurrencePath,next)
         | Select(sourceArmId,bodyExprId) | NoMatch
```

First extend to closed monomorphic nominal recursive groups, including mutual references. Assign stable type/constructor/field/binder/occurrence IDs; derive a plain tagged-node layout; compile source-ordered pattern matrices using first-column specialization/default decomposition. Field slot `i` always means source field `i`. Evaluate constructor fields strictly left-to-right before allocation and the scrutinee once before matching. Select the minimum matching source arm index, bind in its fresh arm scope and execute only its body. Pattern occurrences use `(sourceMatchId,sourceArmId,fieldPath)`, never arena addresses. `workMap` carries an ordered reference-event cursor; shared tests/suffixes cannot erase charges or stop positions. Heuristic columns, packing and test sharing are later comparisons. Parameterized types need separately bounded instantiation; closures, GADTs, mutation/cycles and or-patterns remain additional extensions. Existing dedicated K list/tree sorts do not discharge this work.

The financial extension must add complete committed/tentative state, ordered effects, residual duties, pending operations/observations and phase/cost fields to `ReferenceState`, images and observations together. Existing Core/2 successful-prefix typed await remains controlling. Exact universal `Commit`/`Yield`/`Await` opcodes and target-phase mapping stay OPEN for integration/review; no opcode is supplied here. Chunk cuts, caches, match plans and residual blocks cannot create financial commits or move source boundaries. Existing UISEC clauses remain unchanged, with all new security implications referred to their Opus owner.
