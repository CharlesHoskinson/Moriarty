# Sherlock concrete model source inventory

Status: SOURCE-INSPECTED, with separately captured SOURCE-SPECIFIC MODEL OBSERVATIONS. The immutable Candidate A snapshot and current repaired Quint source have distinct membership. [observations.json](../../../models/sherlock/observations.json) records the actual commands, hashes, repair lineage and results. No model/build/native execution is performed for Candidate B integration. Existing passes do not validate B interfaces, establish interpreter correctness, or supply cross-layer correspondence.

## K: generic first-order function-table evaluator, restricted recursive values

Source: [K README](../../../models/sherlock/k/README.md), [manifest](../../../models/sherlock/k/manifest.json), `../../../models/sherlock/k/main.k`; unchanged modules `MORIARTY-UI-SYNTAX` and `MORIARTY-UI`.

The function table is an input to `uiRun(code,expression,fuel,stackLimit)`. The same explicit-control/frame rules handle literals, variables, lets, Bool negation/conditionals, add/sub/mul/equality/ordering, named calls, typed returns, dedicated recursive constructors and ordered matching. Names are integer IDs; lexical environments are maps. Admission intends unique function IDs and closed bodies, but there is no independent whole-program static checker. Lookup takes the first equal function ID. Calls check argument binding and declared return types.

The value sorts are UInt128, Bool, UInt128 lists and binary trees with UInt128 leaves. Recursive types are supported through these dedicated sorts. Generic nominal ADT tables, records, arbitrary nesting of lists, polymorphism, higher-order closures and letrec are not modeled. Equality is structural over the value sum, with unequal variants false; helper traversal cost is not individually metered.

All eight direct-term K fixtures supply fuel1000/stackLimit64 and have independently frozen expectations; K rule execution remains unobserved:

| Fixture | Literal expected output | Source scope |
|---|---|---|
| uiFactorial3.ui | uiU(6) | Non-tail recursion |
| uiSum3.ui | uiU(6) | A distinct recursive function table, unchanged rules |
| uiListsum123.ui | uiU(6) | List matching/recursion |
| uiTreesum45.ui | uiU(9) | Branching recursion/pending right call |
| uiBoolLetOrder.ui | uiU(7) | Bool/branch/let |
| uiCallerLocal.ui | uiU(12) | Caller map restoration |
| uiMutualEven4.ui | uiB(true) | Named mutual recursion |
| uiConstructList.ui | Cons(1,Cons(2,Nil)) | Strict constructor expressions |

Startup and terminal publication cost zero; machine-section rules each consume one fuel unit. Scheduling and performing a frame push are separate charged stages. Final value/empty stack and explicit error publish even at zero fuel; other controls at zero exhaust. All frame pushes obey stackLimit; stack overflow currently publishes error4, not a named frame-Exhausted result. Fuel bounds machine transitions, but finite recursive pure helpers perform lookup/binding/matching/equality/validation outside this fuel schedule. Input size, node count, structural depth, byte limits, helper-cost refinement, host time/memory and circuit rows remain unestablished. This is a functional coverage gap relative to UIREQ-012/013's intended bound/reporting contract; it is not silently reported as full compliance.

No financial state, durable wait/resume/checkpoint, multi-segment accounting, addressed heap, hash/circuit relation, native integration or Core/2 correspondence exists in this fragment.

## Quint: three algorithm-specific finite transition systems

Source: [Quint README](../../../models/sherlock/quint/README.md), [source manifest](../../../models/sherlock/quint/source-manifest.json), `../../../models/sherlock/quint/universalMachine.qnt`, `../../../models/sherlock/quint/universalMachineTest.qnt`; unchanged modules `universalMachine` and `universalMachineTest`.

`Program` is exactly `Factorial | ListSum | TreeSum`; it selects algorithm-specific Evaluate/ReturnValue cases and Multiply/AddHead/VisitRight/AddLeft frames. It does not load a K function table, arbitrary code or AST. It has no lexical environments, general let/Bool expressions, mutual recursion, generic matching or runtime allocation. Consequently it does not demonstrate UIREQ-001's arbitrary-supported-program interpreter property or the full K expression language.

Concrete instance: scalar0–15, argument0–4, eight immutable node slots0–7, eight frames, fuel2 or32, six initial scenarios, at most one suspension, one false observation and one accepted event. Nil at reference0 is a real node. The fixed arena holds [1,2,3] at reference3 and Node(Leaf4,Leaf5) at reference7; no generic ADT allocation is modeled. Scalar15 replaces the earlier research proposal's scalar7. Values8,9,12 fit this domain, but arbitrary expressions producing them are still outside this algorithm-only companion.

Eight `Test` scenarios, subsequently observed passing on the repaired source in observations.json, specify factorial3=6, list123=6, tree45=9, yield with pending factorial frames/resume, exhaustion after progress, ordinary factorial4 scalar-bound refusal, terminal stutter and concrete bounds. Hand-traced counts are factorial/list8 ticks and tree6 ticks, with no assertion that they equal K fuel. Each positive-fuel tick charges one unit, including its terminal-return tick; zero-fuel attempt adds attempted work without consumed work. Ready/not-ready observations and terminal stutters charge no logical work. Unlike K's publication rule, this terminal-return tick is charged: a cost/step adapter is required.

The yield scenario waits at accumulator1 with three pending factorial frames and consumedWork5. A false observation increments observation diagnostics while preserving control/frames/snapshot/fuel; it does not leave the entire Machine record byte-identical. A ready event resumes the pending value. This abstract checkpoint has no durable serialization, caller environment, financial state, duties or commit operation. `boundary` is a local interpreter suspension boundary, never a financial commit or proof chunk. Existing Core/2 successful-prefix semantics and exact future Commit/Yield design remain unchanged/open.

## Correspondence and acceptance gaps

A future projection must relate K structural terms to the fixed arena, general K contexts to algorithm-specific frames, and unequal step/cost schedules on a named common subset. K has no Waiting state, so Quint's checkpoint extension has no present K counterpart. No simulation/refinement follows from coincident expected terminal values. There is no compiler, circuit/native or ledger correspondence result.

These model sources retain all twenty demonstrator obligations. D17's nested lists need a new K value/type extension; D19 is not the concrete Quint yield specimen; D20 finance is absent in both. Exact model source checking, execution, traces and any theorem remain separate pending work.

## Actual observations and Candidate B coverage

The existing [observation record](../../../models/sherlock/observations.json) names K source SHA256 `b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41` and repaired Quint main source SHA256 `b7aadd991ff9fd1b0821dcf5d183191323406d923a7394ca464acf3a8565c506`. These are exact-source observations, not executions of this new design.

| Captured check | Actual observation | Limit |
|---|---|---|
| K frontend | Exit0; syntax Kore and compiled text produced | No runtime `definition.kore`; no rule rewriting, fixture run or semantic metatheory observed |
| Quint typecheck | Repaired test module typechecked | Parser/source acceptance only |
| Quint concrete tests | All eight passed | Finite algorithm-specific model, not a general code interpreter |
| Quint sampled run | 32 traces, max32 steps, seed1, no observed `allInv` violation | Sampled, not exhaustive or inductive verification |
| Quint witnesses | All seven named witnesses occurred | Reachability within this source/model instance, not correspondence |

The two Quint repairs added Boolean action-return annotations and then renamed builtin-colliding `next` to `chargedMachine`. The original frozen source and repair lineage remain retained. No model or expected-oracle bytes change for this design edit. K's free terminal publication, stack error4 and unmetered helpers remain actual source properties; Quint's terminal charge, phase=Rejected/reason=StackBound frame refusal and finite Waiting abstraction remain distinct. Its frozen `push` cap boundary uses `depth > MAX_FRAMES`; neither equal cap boundaries nor equal overflow charges with K/B are assumed. The Opus A1 per-profile classifier preserves that native observation rather than mapping it to numeric error4.

| Candidate B contract | Current model coverage | New work before corresponding claim |
|---|---|---|
| ProgramImage/SourceOccurrenceId | K input function order exists; neither model has the proposed indexed image/provenance API | Full admission/preparation/occurrence mapping and two-program comparison consumer |
| Explicit helper microsteps | K pure helper recursion unmetered; Quint is algorithm-specific | Full bounded helper controls/rules and new `sherlock-helper-microstep/1` schedule |
| MachineImage/RepresentationRelation | Quint has its separate fixed scalar15 arena/frames; K structural terms | Partial codec, shared bounded state and complete environment/frame correspondence |
| Named carry/borrow/full-product API | K mathematical UInt128 source; Quint scalar15 | Exact component implementations, layout mapping and cross-limb expectations |
| WitnessPlan/ConstraintPlan/capabilities | Absent | Separate actual operation, dependency and export coverage |
| ReferenceSlice/earliest stop | Individual legacy K rules and separate Quint ticks | Ordered prefix materialization, exact-schedule adapters and cut/resume mapping |
| ResidualStepIR | Absent | Finite static worklist, fallback and unchanged dynamic checks |
| TypeDecl/CtorLayout/MatchPlan | Dedicated K list/tree patterns; fixed Quint node kinds | Declaration/type/ordered-match adapter, including nested/mutual groups |
| Financial/await extension | Pure K absent; Quint has unmatched nonfinancial Waiting | Existing Core/2 successful-prefix/effects/duties adapter and exact opcode/phase policy |

`interfaces.md` defines three distinct identities: `sherlock-k-a/1`, `sherlock-quint-a/1`, and proposed `sherlock-helper-microstep/1`. Naming them does not change model bytes or establish refinement. The completed Opus-owned [A1–A4/B002 dispositions](security-candidate-b.md) supply proposed B accounting, named runtime cap failures, K/B frame error4 with separate frozen Quint StackBound, NoMatch-to-error9, boundary `decodeCanonical` versus runtime `decodeStep`, and provenance dispositions; none is validated by these existing model results. Runtime `decodeStep`/`StepInv`, host `checkStepInv`, boundary-only `project`, physical active lengths versus logical allocation counters, and actual constraint/WF obligations are new specified work, not model results. `materializeStep` uses runtime decoding; ChunkStore puts project and gets decode canonically. Full row/control payload definitions, `Bounds.stepCap`, `admit`, the per-profile resource classifier, outcome-code table and failure charge/allocation rows remain Phase 0 schema-freeze work under the Opus handoff. Identical final literals are insufficient: comparisons require decoded control, lexical maps, ordered frames, structural occurrences, outcome/reason and work/fuel/caps at each mapped boundary. Existing tests/sampling do not implement these mappings.

## Required Lean/Clean stage and separate implementation evidence

[UIREQ-026](specs/universal-interpreter/spec.md) retains Lean semantic/representation definitions and a separately bounded Clean step circuit. The existing Fenrir fork separately delivered a merged pure first-order Lean reference foundation with independent source reviews and eight normal plus exact-remaining-zero publication checks. Its source and private pins are not copied here, and this report does not freshly rerun those checks. That child evidence supplies no K/Lean refinement, B helper/image/ADT correspondence, Clean step circuit, exporter/native or financial acceptance.

Recursive inductive semantic values and finite fuel-runs remain separate from fixed-capacity `MachineImage Bounds F`. [clean-stage.md](clean-stage.md) records the current public API pin and remaining codec/component/export tasks. All existing model, Lean, circuit/native and target evidence classes stay separate. No full U0–U7, M1–M17, MC/SP/G or ZR/MNR exit is completed by this integration.
