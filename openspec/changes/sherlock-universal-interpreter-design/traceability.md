# Sherlock traceability and scope

FINAL DESIGN CANDIDATE B / SPECIFIED-ONLY; fresh independent review and the existing decision rule remain pending. The scopes below describe obligations, not passing results. Concrete source inventories are reconciled in [model-coverage.md](model-coverage.md); historical Candidate A observations have the exact limited scope below. Every Candidate B acceptance scenario remains unexecuted. There is no established K↔Quint, compiler, circuit or ledger correspondence. K pure uses structural values; concrete Quint uses a fixed finite arena. These differences require explicit adapters.

## UIREQ component matrix

All Candidate B rows have evidence **SPECIFIED-ONLY / UNEXECUTED**. K/Quint scope columns describe unchanged A sources, never implementation of B additions. Source-inspected membership is not an execution result. Some broader target features remain proposals. The optimization column for every row is UIREQ-024: preserve its declared observation/resource behavior and name new evidence or gaps.

| UIREQ | Observable | K scope | Quint scope | Compiler work | Circuit work | Host work | Scenario membership | Task |
|---|---|---|---|---|---|---|---|---|
| 001 | Same rules across programs | Generic function-table source | Three fixed algorithms; not generic program input | Program table/input lowering | Fixed dispatch relation | Load/report | D10,D14 | [B-T02](tasks.md#b-t02-program-preparation) |
| 002 | Admitted grammar/profile | Dedicated UInt/Bool/list/tree grammar | Concrete scalar15/fixed-arena instance | Admission/type profile | Admitted operation envelope | Profile report | D01–18 | [B-T01](tasks.md#b-t01-reference-inventory-profiles-and-independent-expectations) |
| 003 | Lexical calls | Map environments/restoration source | UNMODELED lexical environment | Scope/parameter lowering | Environment/frame transition | Trace replay | D05 | [B-T04](tasks.md#b-t04-bounded-representation) |
| 004 | Left-to-right | Explicit evaluation contexts source | Algorithm-specific order only | Order preservation | Transition ordering | Trace export | D04,D18 | [B-T01](tasks.md#b-t01-reference-inventory-profiles-and-independent-expectations) |
| 005 | Self/mutual recursion | Self/mutual named-call source | Factorial/ListSum/TreeSum cases only | Code references | Call/return dispatch | Program assembly | D10–12 | [B-T02](tasks.md#b-t02-program-preparation) |
| 006 | Exact scalars | UInt128 checked operations; structural equality helper | Scalar15 guarded arithmetic; not UInt128 | Primitive lowering | Exact arithmetic mapping | Outcome report | D06–09 | [B-T05](tasks.md#b-t05-exact-arithmetic-components) |
| 007 | Typed recursive ADTs | Dedicated UInt list/tree sorts; no generic ADTs | Fixed eight-node arena; no allocation | Declarations/constructors | Node/tag/field mapping | Data encoding | D13–17 | [B-T10](tasks.md#b-t10-declaration-and-match-plan-extension) |
| 008 | Match branch | Source-ordered dedicated patterns | List/tree algorithm cases, no generic match | Case/binder lowering | Case dispatch | Traversal trace | D13–17 | [B-T10](tasks.md#b-t10-declaration-and-match-plan-extension) |
| 009 | Structural sharing | Extensional terms; unmetered equality helper | Fixed immutable references; no sharing fixture | Representation adapter | Tree/arena correspondence | Interning/encoding | Repeated-subtree witness | [B-T04](tasks.md#b-t04-bounded-representation) |
| 010 | Explicit frames | General explicit frame source | Four algorithm-specific frame tags | Call lowering | Frame push/return | Checkpoint preimages | D10,D16 | [B-T04](tasks.md#b-t04-bounded-representation) |
| 011 | Completed pure value | Zero-cost terminal publication | Charged terminal-return tick; terminal stutter free | Outcome lowering | Terminal projection | Result display | D01–18 | [B-T01](tasks.md#b-t01-reference-inventory-profiles-and-independent-expectations) |
| 012 | Bound outcome | Fuel cap; frame cap error4; input size caps absent | Scalar/frame/fuel instance bounds | Bound admission | Bound transition mapping | Reason/work report | Progress then exhaustion | [B-T03](tasks.md#b-t03-explicit-helper-schedule) |
| 013 | Work schedule | Machine rules charge1; helpers unmetered | Tick charges1; observations/stutter0; unequal K schedule | Cost-preserving lowering | Separate circuit-cost mapping | Separate logical/time/fee planes | D04,chunk witness | [B-T01](tasks.md#b-t01-reference-inventory-profiles-and-independent-expectations) |
| 014 | Checkpoint | UNMODELED | Partial factorial snapshot; no durable serialization | Future checkpoint lowering | Future checkpoint relation | Durable serialization | D19 | [B-T04](tasks.md#b-t04-bounded-representation) |
| 015 | Wait/resume | UNMODELED | Factorial pending-return ready/not-ready observations | Future await adapter | Future observation/resume mapping | Event/checkpoint interface | D19 | [B-T12](tasks.md#b-t12-existing-core2-financial-preservation) |
| 016 | Chunk transparency | No proof-chunk primitive | No proof-chunk action; boundary is local suspension | Chunk/source distinction | Recursive segment adapter | Trace grouping | Chunk witness | [B-T08](tasks.md#b-t08-prefix-preserving-physical-grouping) |
| 017 | Financial observation | UNMODELED | UNMODELED financial effects/duties | Future journal lowering | Future complete-effect relation | Complete financial report | D20 | [B-T12](tasks.md#b-t12-existing-core2-financial-preservation) |
| 018 | Core/2 await preservation | UNMODELED | No /2 financial mapping | Future /2 adapter | Future prefix correspondence | Atomic prefix/checkpoint storage | D19,D20 | [B-T12](tasks.md#b-t12-existing-core2-financial-preservation) |
| 019 | Local failure | UNMODELED finance | No financial committed/working state | Future failure lowering | Future failure relation | Prefix/work/duty report | D20 failure branch | [B-T12](tasks.md#b-t12-existing-core2-financial-preservation) |
| 020 | Ledger phases | UNMODELED | UNMODELED | Future phase lowering | Future phase relation | Actual phase observations | Future retained-fee specimen | [B-T12](tasks.md#b-t12-existing-core2-financial-preservation) |
| 021 | K/Quint projection | Structural-value generic evaluator | Algorithm-only fixed arena; projection absent | No compiler transfer | No circuit transfer | Projection/replay tooling | Common-domain traces | [B-T11](tasks.md#b-t11-fresh-semanticmodel-correspondence) |
| 022 | Compiler mapping | Source reference only | Algorithm abstraction only | Whole admitted grammar obligation | Distinct circuit obligation | Public pipeline report | D10,D14 new-program pair | [B-T06](tasks.md#b-t06-compilation-and-export-inventory) |
| 023 | Native inventory | Source reference only | Finite abstraction only | Transition lowering | New/existing operation inventory | Native endpoint adapter | Future fixed-relation pair | [B-T13](tasks.md#b-t13-required-cleannative-stage) |
| 024 | Optimization | Reference behavior; helper-cost gap | Finite instance; no preservation result | Preservation obligation | Requalification obligation | Cost/evidence ledger | Future optimization proposals | [B-T14](tasks.md#b-t14-exact-candidate-reporting-and-reviews) |
| 025 | Evidence report | Source class only | Source class only | Separate correspondence class | Separate native class | Tree/profile/target receipts | All rows | [B-T14](tasks.md#b-t14-exact-candidate-reporting-and-reviews) |
| 026 | Required Clean stage | K→Lean semantic mapping UNPROVED | Bounded validation input only | Native-lowering correspondence OPEN | Bounded Clean step not implemented | Source-pinned inventory/receipts | UIREQ026 scenario | [B-T13](tasks.md#b-t13-required-cleannative-stage) |
| 027 | Source/table/occurrence provenance | Unindexed AST; helpers preserve first match | No generic program image | Bounded ordered ProgramImage | Fixed code/input view | Uncached preparation/source maps | D21,D24 | [B-T02](tasks.md#b-t02-program-preparation) |
| 028 | Explicit bounded helpers/new schedule | Unmetered recursive helpers; NOT B | Algorithm-only ticks; NOT B | Cursor/helper lowering | Bounded helper transitions | Helper visits/profile report | D21,D23,D25,D36 | [B-T03](tasks.md#b-t03-explicit-helper-schedule) |
| 029 | Partial image/reference domain | Structural terms; no bounded image | Fixed arena only; no generic encoder | Bounds/layout admission | MachineImage Bounds F relation | Partial encode/decode | D22,D23,D29 | [B-T04](tasks.md#b-t04-bounded-representation) |
| 030 | Named full-result arithmetic | Exact mathematical UInt128/error10 | Scalar15 only | Typed carry/borrow/full product mapping | New checked arithmetic components | Layout/result inventory | D26–28,D36 | [B-T05](tasks.md#b-t05-exact-arithmetic-components) |
| 031 | Constraint/witness/export split | No plans/exporter | No plans/exporter | Separate compilation visitors | ConstraintPlan feature inventory | WitnessPlan/BackendCapabilities | D31 | [B-T06](tasks.md#b-t06-compilation-and-export-inventory) |
| 032 | Ordered earliest-stop prefixes | No grouped-step interface | No grouped-step interface | Boundary-preserving grouping | Common fixed-step/family interface | ReferenceSlice/StepRecord materialization | D32 | [B-T08](tasks.md#b-t08-prefix-preserving-physical-grouping) |
| 033 | Bounded residual step | No residualizer | No residualizer | Finite static worklist/fallback | Universal/family/per-program comparison | ResidualStepIR/source-rule maps | D33 | [B-T09](tasks.md#b-t09-bounded-static-residualization) |
| 034 | Generic declarations/source-first arms | Dedicated UInt list/tree only | Fixed list/tree cases only | TypeDecl/CtorLayout/MatchPlan | New declaration/ordered match components | Layout/occurrence encoding | D34,D35 | [B-T10](tasks.md#b-t10-declaration-and-match-plan-extension) |
| 035 | Deterministic witness dependencies | No witness DAG | No witness DAG | Operation/dependency typing | Separate unchanged constraint interface | Uncached scheduler/materializer | D30 | [B-T07](tasks.md#b-t07-deterministic-witness-baseline) |

## Existing U owners

These assignments schedule this draft under the existing roadmap; no parent exit closes here.

| U | Existing responsibility | Relevant draft clauses |
|---|---|---|
| U0 | Profile, semantics and target reconciliation | 001–035 |
| U1 | Semantic basis, bounds, primitive/representation contracts | 003–013,024,026–035 |
| U2 | General source-to-target compiler and developer consumers | 001–011,017–023,025–035 |
| U3 | Await, partial effects, duties, recovery and cost lifetime | 012–020,029,032 |
| U4 | Native recursion, composition and private-history scope | 014–016,021–035; existing Opus requirements unchanged |
| U5 | Optional services, federation and cost planes | 013,017,020,025,031,035 as interface inputs only |
| U6 | Full ACTUS/DeFi conformance | Demonstrators are small additions; no replacement coverage |
| U7 | Public developer tooling and release evidence | 001,002,022,025–035 |

## M1–M17 retained capability crosswalk

| Existing ID | Retained capability | Draft relationship; full acceptance remains open |
|---|---|---|
| M1 | General source → ledger-9 ZKIR | 001,022,023 |
| M2 | Pinned toolchain and qualified target tuple | 002,025; no invented network/version |
| M3 | Prove/verify/deploy/call and complete effects | 017,020,023,025; no native result here |
| M4 | DUST and Capacity Exchange sponsorship | 013,017,025; actual fees/settlement unmodeled |
| M5 | 64KB transaction, fraud k≤19, replay bound | 012,013,025; no target-capacity claim |
| M6 | Deterministic semantics and replay | 003–013,021,022 |
| M7 | Network/Anchor-bound signed intents | Existing requirement retained; Opus-owned interpretation |
| M8 | Audited in-circuit Schnorr primitive | Existing requirement retained; Opus-owned interpretation |
| M9 | Domain-qualified asset identity | 017,020; identity semantics retained unchanged |
| M10 | simulate/test/verify-intent developer tools | 002,022,025; no command result here |
| M11 | FeePool/Capacity Exchange language contracts | 017–020; demonstrators do not replace contracts |
| M12 | Anchored Express Bus Registry | Future target/interface adapter; no model coverage |
| M13 | Pending/recovery/liabilities/scoped calls | 014–020; pure K omissions explicit |
| M14 | Single-domain swap batch fast path | 017,020; no full swap coverage |
| M15 | Complete financial library/conformance | Existing U6 denominator retained; D01–36 insufficient |
| M16 | Fee capability and cost-plane meters | 013,017,025; service model absent |
| M17 | Federation policy and actual committee | Existing U5 obligation retained; identity unestablished |

## MC, SP and release lineage

| Existing exit | U owner retained | Relationship |
|---|---|---|
| MC01 | U0/U1/U2 | Semantic basis, bound and compiler clauses 001–013,022 |
| MC02 | U2/U7 | Actual Preview integration remains distinct from target qualification; 023/025 |
| MC03 | U4; feasibility U0/U1 | Real recursive financial steps remain open; 016/023 are design inputs |
| MC04 | U2/U3/U4 | Compiler/phase/target mapping 017–023 |
| MC05 | U2 subset; U4 native base and U6 extensions | Existing intention/native financial obligations unchanged; Opus owns security |
| MC06 | U4 with U3 | Actual private successor/split/join and resource composition remain open |
| MC07 | U6 | All original conformance/library cases retained |
| MC08 | U7 | Actual independent public installation and developer journey; 001/025 |

SP01–SP04 retain U0/U1/U2/U4 input obligations; SP05 remains U2; SP06 remains U1-feasibility/U4; SP07/SP08/SP11 remain U6; SP09 retains U2/U4 subset plus U6 requalification; SP10 remains U3/U4; SP12 remains U7. G01–G24 remain individually required. No source draft, model example or optimization closes them. Preserve all 277 ACTUS fixtures/fields, 18 executable types, 32 taxonomy dispositions, DS01–07; all 72 DeFi rows, DA01–24, supplements, three held-outs, eight intent cases and eight regression classes. Preserve sequential, disjoint parallel, shared-state interleaving, atomic synchronization and asynchronous messaging composition.

## ZR/MNR lineage, referenced without new security judgments

The original backend requirement and Opus draft control these obligations. The links below are a coverage index only: no security clause is rewritten or assessed here. UIREQ-023 requires a component inventory; UIREQ-025 requires evidence labels. All listed ZR/MNR exits stay open.

| Existing requirement | Draft interface touchpoint | Design/model limitation |
|---|---|---|
| ZR01 | 002,022,023,025 | Pinned semantic/target capability remains separately qualified |
| ZR02 | 023 | Native recursion route premise; no execution |
| ZR03 | 014,017,023 | State/statement schema interface; Opus requirements unchanged |
| ZR04 | 005,023 | Function recursion is not recursive proof/genesis evidence |
| ZR05 | 023 | Relation identity interface; Opus-owned requirements unchanged |
| ZR06 | 006–010,022,023 | K semantics is not arbitrary-witness circuit correspondence |
| ZR07 | 023 | Mandatory-control requirements remain Opus-owned |
| ZR08 | 023 | Complete native decider obligations remain Opus-owned |
| ZR09 | 013–016,023 | Multi-parent composition is outside first pure fragment |
| ZR10 | 020,023 | Ledger currentness/consumption unmodeled and Opus-owned |
| ZR11 | 017–023 | Financial/phase compiler correspondence absent |
| ZR12 | 012,013,023 | Model bounds do not establish native bound soundness |
| ZR13 | 013,024,025 | No circuit/proof/time/DUST measurements |
| ZR14 | 014,023 | No privacy/handoff assurance from structural snapshots |
| ZR15 | 002,024,025 | No source/profile migration adopted |
| ZR16 | 023,025 | No native proof-mode/ledger qualification |

| Existing refinement | Retained ZR lineage | Draft interface only |
|---|---|---|
| MNR01 | ZR01/03/06 | 022,023,025; transcript/field specification remains separate |
| MNR02 | ZR04/07/08/09/16 | 023; deferred obligations remain Opus-owned |
| MNR03 | ZR01/02/05/09/12/13/15 | 002,023; compatibility remains separate |
| MNR04 | ZR03/07/10/11/16 | 017,020,025; predicted/actual evidence distinguished |
| MNR05 | ZR01/07/16 | 025; model sources cannot become native evidence |
| MNR06 | ZR01/05/08/15/16 | 025; no key/cache work authorized here |
| MNR07 | ZR08/09/12/13 | 013,023; batching/joins outside first fragment |
| MNR08 | ZR01/06/14/16 | 021–025; exact component assurance inventory pending |

Canonical lineage sources: checkout `openspec/changes/consolidated-language-kernel/traceability.md`, `docs/MORIARTY-BACKEND-REQUIREMENTS.md`, `ROADMAP.md` and `docs/MORIARTY-PRODUCT-CONTRACT.md`, all at the recorded source baseline. This package does not edit their wording, status or index.

## Optimization evidence register

| Candidate | Preservation predicate | Logical work impact | Circuit/host/ledger cost evidence | Correspondence gap | Status |
|---|---|---|---|---|---|
| Ten optimization proposals + ten academic/project proposals | UIREQ-024 and UIREQ-027–035; independently authored complete normal expectations | A-preserving physical changes retain sherlock-k-a/1; proposed helper-metered schedule separately named | All complete-path comparisons unmeasured; no winner | Every changed representation/helper/plan/lowering; no port inferred | Integrated into Candidate B source design; adoption and implementation/acceptance OPEN |

## Unchanged Opus inputs and unresolved compatibility

All five Opus brainstorm proposals are linked and hashed in inputs.json. Opus02 favors a universal code-as-data circuit; Opus05 favors starting with per-program step realizations after freezing machine/observations. Both remain live alternatives, with no author-added security judgment. Exact security reasoning and clauses stay in those unchanged external documents.

Concrete K uses dedicated recursive list/tree sorts rather than generic nominal declarations; its helpers have no individual fuel charges, and stack overflow is error4. Those facts require reconciliation with the broader design bound/cost contract and the Opus-owned UISEC-004/UISEC-K-02/UISEC-K-05 proposals. Compatibility is OPEN and substantive security assessment is reserved to Opus. Quint scalar15/refusal/tick accounting differs from exact UInt128/K fuel; it supplies no general code, lexical or financial model.

## Required Clean stage crosswalk

UIREQ-026 is required within existing U0 semantic/profile reconciliation, U1 primitive/representation/bounds, U2 compiler/native correspondence and U4 recursion/composition; U7 retains exact evidence reporting. It refines the existing work rather than creating a new milestone. M1/M5/M6 require the semantic/representation/resource mapping; M2/M3 retain independent target qualification; M10 retains public tooling evidence. MC01/MC04 retain semantic/compiler obligations, MC03/MC05/MC06 retain actual native recursion/financial/private composition obligations, and MC08 retains release evidence. Relevant existing ZR01/ZR06/ZR11/ZR12/ZR13/ZR16 and MNR08 remain unchanged; security interpretation/acceptance stays Opus-owned.

Required chain: K reference → Quint bounded validation → Lean recursive semantic types/fuel-run relation + separate fixed-capacity Clean step/correspondence stage → compiler/native integration. The existing Fenrir pure Lean reference source has narrow normal/publication checks; B-specific helper/image/plan/ADT definitions, all Clean step circuits, correspondence proofs and Midnight export work remain unimplemented or unproved. Clean public-core reuse must use exact source hashes; local extensions are not pinned by HEAD alone. No Midnight exporter or backend compatibility follows from existing non-Midnight backends. See clean-stage.md.

Cross-repository obligation for UIREQ-026: Moriarty owns Sherlock language/OpenSpec; the existing private Fenrir Clean fork/worktree owns the required integration artifacts. Its actual revision/source/theorem/native-consumer record must bind the public semantics/model hashes. A public upstream research pin is not that integration pin. Public reporting uses only shareable dependency/evidence references, never private paths or code.


## Research concept and requirement mapping

These are independent reconstruction proposals, not selected upstream code copies. Source licenses and foreign backend conventions are provenance/adapter considerations; they do not discard useful algorithms. All ten full [academic/project proposals](../../../docs/research/sherlock/papers/README.md) and [optimization10](../../../docs/research/sherlock/optimization/10/proposal.md) were read for this integration. Source facts, proposed APIs and required future observations are distinct.

| Research lane | Integrated functional concept | UIREQ | Unexecuted normal contracts |
|---|---|---|---|
| 01 Evaluator-to-machine derivation | Explicit helper controls, occurrence provenance and live frame fields | 003,004,010,027,028 | D21–25 |
| 02 Functional interpreters/data | Structural arena decode, complete saved caller state and witness planning | 007,009,010,029,031,035 | D22,D23,D29,D30 |
| 03 Resource-aware compilation | Separate resource manifest, logical/physical frame accounting, prefix cost labels | 012,013,024,032 | D23,D25,D32 |
| 04 Clean/export separation | Partial bounded image, constraint/witness/export capability split | 026,029,031 | D29,D31 |
| 05 Exact multiword arithmetic | Named carry/borrow/full-product APIs with checked UInt128 wrappers | 006,030 | D26–28,D36 |
| 06 Witness dataflow | Typed dependency DAG, deterministic uncached baseline and separate emitter | 031,035 | D30,D31 |
| 07 Immutable program images | Full ordered source image, distinct occurrence/expanded IDs, operation partitions | 001,027,031 | D24,D31 |
| 08 Execution families/chunks | One reference relation, common fixed-step envelope and earliest-stop slices | 001,016,023,032 | D10,D14,D32,D33 |
| 09 General recursive constructors | Nominal declaration/layout split and source-priority match plans | 007,008,034 | D34,D35 |
| 10 Static reference-step specialization | Finite static worklist, dynamic recursive state and generic fallback | 005,024,033 | D33 |
| Optimization10 synthesis | Independent oracle, reference prefixes, complete resource planes and disjoint implementation | 013,024,025,027–035 | D01–36 |

## Historical observations that do not validate Candidate B

The immutable Candidate A freeze is `57d830f46fee57c56ed8b854d21f5dd7a695d2977d061b1ef6905eaecd7de30e`. [observations.json](../../../models/sherlock/observations.json) SHA-256 `79dba07952e07e50e623f4b457d9dff0c385cf94ff66ddb6ac4d821455ce7b3c` names exact command/tool/source/bounds membership; it was not rerun or edited for B.

| Exact old artifact/observation | What it records | What remains unvalidated for B |
|---|---|---|
| K main.k SHA-256 `b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41` | Generic function-table CEK source, dedicated UInt list/tree values; unmetered recursive helpers, frame cap Rejected(error4), free terminal/error publication | B helper microsteps/work schedule, all bounded-image/plan/ADT/residual interfaces and any K rewriting/refinement |
| K expected-results.json SHA-256 `b0789d3a80ce7e6a29f3d51e882001ddd68d05f9c92ab8aa66e68a2009fe116e` and eight named fixtures | Independently frozen result literals; exact work oracle explicitly absent | Exact work/fuel prefixes or execution results; do not alter expected files to fit B |
| C1 K frontend observation | syntaxDefinition.kore SHA-256 `c2fc84349503b4389c8b2000285c6aea063c1e3bcdc924f8526a51588d974815` and compiled.txt `4be44d083d11701e7b6a1e67e5b8eaaa4b824536ffa29cedf93a185540a38d3c`; no runtime definition.kore/rule execution | Every new B execution or semantic/circuit/native claim |
| Repaired Quint universalMachine.qnt SHA-256 `b7aadd991ff9fd1b0821dcf5d183191323406d923a7394ca464acf3a8565c506`; tests `587a9beb376d92d8cecc18e4032a8cfbe83882f95623166963e5a9a0505f0e7e` | Three algorithm-specific scalar15/fixed-arena transitions; predecessor source digests `fff2752e36331042f1d86414163bfd714cb2f7bb64a7a125490682ce754fbe08` and `6c152f5c9f38dd553f5e2ed0a0bfd7c94a72a34467a60a4deb5bd4d220b37fe5` remain repair lineage | Generic program input, lexical environment, UInt128, B metering, generic constructors/match plans, representation or compiler correspondence |
| C2/C3/C4 Quint observations | Typecheck; eight tests passed;32 traces, max32 steps, seed1, no observed allInv violation and all seven named witnesses | Exhaustive/inductive verification, K correspondence and every B helper/image/arithmetic/plan/residual/ADT acceptance scenario |
| Quint expected-normal.json SHA-256 `8c208335f18571644c4d51ca5df54b41539b15684b5f6e1049fa3f23f76b4d6f` | Retained independent normal expectations for that finite companion | No full UInt128 or changed-profile oracle |
| Existing Fenrir pure Lean reference source; merged integration record retained privately | Eight normal checks plus exact-remaining-zero publication checks and independent source review; public scope only | No Clean bounded step circuit, soundness/completeness, K/Lean correspondence, B helper/image/plan/ADT extension, exporter/native/financial acceptance |

No old observed `allInv` result is reinterpreted as a new security judgment. Security requirements, tests, theorem design and acceptance belong to Opus, including the unchanged [security-requirements.md](security-requirements.md) and separately authored `security-candidate-b.md`. This functional crosswalk neither authors nor assesses their clauses.

## Retained acceptance denominator and open decisions

Candidate B has 35 Sherlock functional clauses and 36 normal demonstration contracts; the pre-existing canonical 160 clauses are a separate unchanged index, not a denominator of195 or a completion fraction. U0–U7, M1–M17, MC01–MC08, SP01–SP12, G01–G24, ZR01–ZR16 and MNR01–MNR08 keep every existing exit and conformance denominator stated above. No source-design progress closes one.

The [private draft-source observation](../../../docs/audits/2026-10-15/sherlock-draft-source/README.md)
adds scoped preparation support for UIREQ-001, UIREQ-005 and UIREQ-027. Eleven
new tests pass through the real lexer/parser/checker and image bridge, including
factorial(4) = 24 and tree folding = 7 through both host interpreters. The 86
older tests compiled but were filtered out and were not rerun. This unregistered
test-only envelope does not execute a Candidate B acceptance scenario or close
any retained requirement. Full grammar, bounded images, complete prefixes,
cross-layer correspondence, circuits and qualified targets remain open.

Before implementation acceptance, exact helper-rule inventories/payload bounds and profile admission must be reviewed; completed-interval helper erasure and partial representation relations remain unproved. Concrete scalar/arena/frame layouts, backend realization and any measured winner remain open. Generic ADTs beyond the admitted closed fragment and bounded parameter instantiation remain extensions. Core/2 financial await preservation is required, but exact future Commit/Yield/Await opcodes, complete financial machine fields and target phase correspondence are still open. Fresh independent Muse nonsecurity and Opus security reviews plus the existing decision rule are required before architecture adoption. These are maintainer delivery obligations, never public developer membership or compile/prove/deploy permissions.

## Scoped offline input/readback observation

The [offline helper](../../../models/sherlock/STAGENET-OFFLINE-README.md) inventories existing Core/1 inputs and compares all supplied
JSON fields, retaining ordered arrays and missing/present distinctions. Its
captured R2 result records 16 passing normal assertions for the bound source.
This is preparatory tooling for complete readback reporting, with no native
execution or independent semantic oracle. It does not implement the missing
Sherlock/Clean/native adapter or public Moriarty commands and closes no
UIREQ/B/U/M/MC/SP/G exit or Midnight stagenet qualification.
