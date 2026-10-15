# Lane 10: rebuild a staged reference-machine specializer

Status: independent nonsecurity research proposal; implementation and measurements are specified-only. Backend choice remains open. Candidate A freeze `57d830f46fee57c56ed8b854d21f5dd7a695d2977d061b1ef6905eaecd7de30e`, design, model-coverage, Clean-stage and optimization02/05/10 are prior context. All four source families below were freshly retrieved online; none is inherited experimental evidence.

**Recommendation, P1:** independently rebuild a small offline specializer around one unchanged Sherlock reference step. First specialize code lookup and dispatch, retaining explicit semantic boundaries. This connects optimization02's indexed ProgramImage, optimization05's realization comparison and optimization10's common consumer. Concepts are transferable even when donor runtimes, implementation languages, licenses or backends differ. No donor code execution or backend adoption is proposed.

**Fresh source facts.** [Futamura's author-hosted revised paper](https://fi.ftmr.info/PE-Museum/PE-Revised1999.pdf), §2 and §3, explains fixing some program inputs to produce a residual program, then specializing an interpreter to a source program. The first projection is the relevant concept; a self-applicable compiler generator is unnecessary. Its scanned PDF required OCR and visual checking; equations should be read from the retained page images.

[Jones/Gomard/Sestoft](https://raspi.itu.dk/people/sestoft/pebook/jonesgomardsestoft.pdf), §§4.4.1–4.4.6, supplies the practical algorithm: partition static/dynamic variables, generate specialized program points and process a worklist. Dynamic conditionals retain both successors. Printed pp82–83 show how a known counter under dynamic control creates infinitely many specializations; generalizing that counter to dynamic prevents this example. This is an engineering warning against unrolling source recursion during preparation.

[Oracle's current Truffle explanation](https://github.com/oracle/graal/blob/f1dce3f84de778485d0ba8d6ddfd4d260b230d0a/truffle/docs/PartialEvaluation.md), lines75–87 and272–357, specializes an interpreter using stable guest code, then explains profiled compilation-final fields and invalidation. Inspected code exposes `PartialEvaluator.doGraphPE` and `CompilerDirectives.CompilationFinal`. **Inference:** immutable ProgramImage facts offer a simpler initial staging boundary than profile-dependent specialization; runtime branches remain residual.

[RPython's implementation documentation](https://rpython.readthedocs.io/en/latest/jit/pyjitpl5.html), “JIT hints,” distinguishes green loop-identifying values from red execution values: code/instruction pointer versus frame/context. Pinned `rpython/rlib/jit.py` implements `JitDriver`, `jit_merge_point` and `can_enter_jit`. **Inference:** use this distinction as a design checklist, while rebuilding an offline residual graph; meta-tracing machine-code generation is a separate algorithm. The official 2009 paper URL returned an access-checkpoint page, so no paper-content claim relies on it.

**Concrete rebuild API and algorithm.**

```text
ProgramImage = {profile, occurrenceRows, orderedChildren,
                functions, constants, sourceMap, declaredBounds}
SpecializeConfig = {layout, staticMask, family?, residualSizeLimit}
prepare(P, profile, bounds) -> ProgramImage + PreparationReport
specialize(referenceStep, image?, config) -> ResidualStepIR
ResidualStepIR = {blocks, dynamicInputs, staticBindings,
                  ruleBoundaryMap, sourceOccurrenceMap, omissions}
executeStep(IR, MachineImage) -> {next, ruleId, workDelta, outcome}
```

Preparation indexes source occurrences without evaluating source computations. Mark immutable code, declarations, operand slices and layout constants static; mark control, environment values, frames, logical depth, remaining/consumed fuel, input data and outcomes dynamic. Bounds may be fixed configuration constants; their runtime checks remain.

Generate blocks with a memoized worklist keyed by reference-rule location, source occurrence and finite static layout facts. Fold code decoding and static tag selection. Emit dynamic branches, runtime calls, frame operations and arithmetic unchanged. Do not key specialization on recursion depth, fuel or runtime locals. If the residual-size limit is reached, emit a generic dispatch block; this is a preparation fallback, not application exhaustion. Source recursion continues through explicit machine state and frames.

The same emitter supports three masks: empty program bindings yield a universal step with ProgramImage input; fixed component selection yields a partially specialized family; binding one immutable image yields a per-program step. All are realizations of the same source relation. This API does not predetermine their actual circuit savings.

**Preservation limits and requirements.** UIREQ-001/004/005/010/012/013/024 govern unchanged interpreter meaning, left-to-right arguments/fields, named recursion, pending returns and exact stop/work observations. Preserve separate scheduling/push stages, every frame-cap check, current K error4 classification and zero-charge terminal publication even at zero fuel. Initial residualization must not constant-fold application expressions into fewer charged source steps. Keep UInt128 behavior exact. Unmetered recursive K helpers remain an unresolved cost boundary.

UIREQ-014–020's financial/checkpoint extension is absent from the current pure K fragment. Later staging must preserve ordered successful-prefix effects, duties, pending checkpoint meaning and local-segment outcomes at every boundary. Moving an await, effect or commit across a residual block is outside this proposal. UIREQ-026 retains the required bounded Clean stage and separate native mapping; no exporter or cross-layer result is assumed.

**Normal comparison and falsifiable predicate, P2:** compare universal, family and per-program outputs on the same eight normal fixtures, including factorial3=6, list123=6, tree45=9, caller-local12 and mutual-even4=true. Freeze independent complete intermediate states and exact-fuel/frame-cap boundary expectations first. Require identical decoded reference prefixes, not only final values. Under one later pinned backend/configuration and reuse count N, claim improvement only if total rows including wrappers decrease and `preparationTime + N*(provingTime + verificationTime)` decreases without shrinking the supported envelope. Report code/key bytes, padding, memory and DUST separately. These predicates are unmeasured; CPU JIT throughput supplies no circuit-cost estimate.

**Counterargument and provenance:** specialization duplicates residual blocks and per-program artifacts; a reusable universal shape may win overall. Retain that dissent. PyPy's inspected scope has MIT terms; Truffle API has UPL1.0, while the inspected Graal compiler file declares GPLv2 plus Classpath exception. Paper reuse permissions are UNKNOWN. Independent conceptual reconstruction remains viable; licensing observations neither select nor block the algorithm. Sources, exact pins, file/line pointers, failures and figure nomination are in sources.json.
