# Sherlock concrete model source inventory

Status: SOURCE-INSPECTED / UNEXECUTED. This inventory reconciles the actual K README/manifest and Quint README/sources at the hashes in inputs.json. Source membership and hand-derived expectations are evidence of a reviewable draft. No K/Quint parser, evaluator, test, simulation or prover has run for this inventory. No interpreter correctness or cross-layer correspondence is claimed.

## K: generic first-order function-table evaluator, restricted recursive values

Source: [K README](../../../models/sherlock/k/README.md), [manifest](../../../models/sherlock/k/manifest.json), `../../../models/sherlock/k/main.k`; unchanged modules `MORIARTY-UI-SYNTAX` and `MORIARTY-UI`.

The function table is an input to `uiRun(code,expression,fuel,stackLimit)`. The same explicit-control/frame rules handle literals, variables, lets, Bool negation/conditionals, add/sub/mul/equality/ordering, named calls, typed returns, dedicated recursive constructors and ordered matching. Names are integer IDs; lexical environments are maps. Admission intends unique function IDs and closed bodies, but there is no independent whole-program static checker. Lookup takes the first equal function ID. Calls check argument binding and declared return types.

The value sorts are UInt128, Bool, UInt128 lists and binary trees with UInt128 leaves. Recursive types are supported through these dedicated sorts. Generic nominal ADT tables, records, arbitrary nesting of lists, polymorphism, higher-order closures and letrec are not modeled. Equality is structural over the value sum, with unequal variants false; helper traversal cost is not individually metered.

All eight direct-term fixtures supply fuel1000/stackLimit64 and have independently frozen, unexecuted expectations:

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

Eight unexecuted `Test` scenarios specify factorial3=6, list123=6, tree45=9, yield with pending factorial frames/resume, exhaustion after progress, ordinary factorial4 scalar-bound refusal, terminal stutter and concrete bounds. Hand-traced counts are factorial/list8 ticks and tree6 ticks, with no assertion that they equal K fuel. Each positive-fuel tick charges one unit, including its terminal-return tick; zero-fuel attempt adds attempted work without consumed work. Ready/not-ready observations and terminal stutters charge no logical work. Unlike K's publication rule, this terminal-return tick is charged: a cost/step adapter is required.

The yield scenario waits at accumulator1 with three pending factorial frames and consumedWork5. A false observation increments observation diagnostics while preserving control/frames/snapshot/fuel; it does not leave the entire Machine record byte-identical. A ready event resumes the pending value. This abstract checkpoint has no durable serialization, caller environment, financial state, duties or commit operation. `boundary` is a local interpreter suspension boundary, never a financial commit or proof chunk. Existing Core/2 successful-prefix semantics and exact future Commit/Yield design remain unchanged/open.

## Correspondence and acceptance gaps

A future projection must relate K structural terms to the fixed arena, general K contexts to algorithm-specific frames, and unequal step/cost schedules on a named common subset. K has no Waiting state, so Quint's checkpoint extension has no present K counterpart. No simulation/refinement follows from coincident expected terminal values. There is no compiler, circuit/native or ledger correspondence result.

These source drafts retain all twenty demonstrator obligations. D17's nested lists need a new K value/type extension; D19 is not the concrete Quint yield specimen; D20 finance is absent in both. Exact model source checking, execution, traces and any theorem remain separate pending work.

## Required next Lean/Clean stage

[UIREQ-026](specs/universal-interpreter/spec.md) requires Lean semantic/representation definitions and a separately bounded Clean step circuit after K/Quint design validation. These are new required tasks; no Sherlock Lean/Clean source or proof is delivered by the current model inventory. Recursive inductive semantic values and fuel-based calls/runs remain separate from fixed-capacity circuit encoding.
