# Sherlock traceability and scope

DRAFT / SPECIFIED-ONLY. The scopes below describe obligations, not passing results. Concrete source inventories are reconciled in [model-coverage.md](model-coverage.md); execution/results remain absent. There is no established K↔Quint, compiler, circuit or ledger correspondence. K pure uses structural values; concrete Quint uses a fixed finite arena. These differences require explicit adapters.

## UIREQ component matrix

All rows currently have evidence **SPECIFIED-ONLY**. Source-inspected membership is not an execution result. Some broader target features remain proposals. The optimization column for every row is UIREQ-024: preserve its declared observation/resource behavior and name new evidence or gaps.

| UIREQ | Observable | K scope | Quint scope | Compiler work | Circuit work | Host work | Scenario membership |
|---|---|---|---|---|---|---|---|
| 001 | Same rules across programs | Generic function-table source | Three fixed algorithms; not generic program input | Program table/input lowering | Fixed dispatch relation | Load/report | D10,D14 |
| 002 | Admitted grammar/profile | Dedicated UInt/Bool/list/tree grammar | Concrete scalar15/fixed-arena instance | Admission/type profile | Admitted operation envelope | Profile report | D01–18 |
| 003 | Lexical calls | Map environments/restoration source | UNMODELED lexical environment | Scope/parameter lowering | Environment/frame transition | Trace replay | D05 |
| 004 | Left-to-right | Explicit evaluation contexts source | Algorithm-specific order only | Order preservation | Transition ordering | Trace export | D04,D18 |
| 005 | Self/mutual recursion | Self/mutual named-call source | Factorial/ListSum/TreeSum cases only | Code references | Call/return dispatch | Program assembly | D10–12 |
| 006 | Exact scalars | UInt128 checked operations; structural equality helper | Scalar15 guarded arithmetic; not UInt128 | Primitive lowering | Exact arithmetic mapping | Outcome report | D06–09 |
| 007 | Typed recursive ADTs | Dedicated UInt list/tree sorts; no generic ADTs | Fixed eight-node arena; no allocation | Declarations/constructors | Node/tag/field mapping | Data encoding | D13–17 |
| 008 | Match branch | Source-ordered dedicated patterns | List/tree algorithm cases, no generic match | Case/binder lowering | Case dispatch | Traversal trace | D13–17 |
| 009 | Structural sharing | Extensional terms; unmetered equality helper | Fixed immutable references; no sharing fixture | Representation adapter | Tree/arena correspondence | Interning/encoding | Repeated-subtree witness |
| 010 | Explicit frames | General explicit frame source | Four algorithm-specific frame tags | Call lowering | Frame push/return | Checkpoint preimages | D10,D16 |
| 011 | Completed pure value | Zero-cost terminal publication | Charged terminal-return tick; terminal stutter free | Outcome lowering | Terminal projection | Result display | D01–18 |
| 012 | Bound outcome | Fuel cap; frame cap error4; input size caps absent | Scalar/frame/fuel instance bounds | Bound admission | Bound transition mapping | Reason/work report | Progress then exhaustion |
| 013 | Work schedule | Machine rules charge1; helpers unmetered | Tick charges1; observations/stutter0; unequal K schedule | Cost-preserving lowering | Separate circuit-cost mapping | Separate logical/time/fee planes | D04,chunk witness |
| 014 | Checkpoint | UNMODELED | Partial factorial snapshot; no durable serialization | Future checkpoint lowering | Future checkpoint relation | Durable serialization | D19 |
| 015 | Wait/resume | UNMODELED | Factorial pending-return ready/not-ready observations | Future await adapter | Future observation/resume mapping | Event/checkpoint interface | D19 |
| 016 | Chunk transparency | No proof-chunk primitive | No proof-chunk action; boundary is local suspension | Chunk/source distinction | Recursive segment adapter | Trace grouping | Chunk witness |
| 017 | Financial observation | UNMODELED | UNMODELED financial effects/duties | Future journal lowering | Future complete-effect relation | Complete financial report | D20 |
| 018 | Core/2 await preservation | UNMODELED | No /2 financial mapping | Future /2 adapter | Future prefix correspondence | Atomic prefix/checkpoint storage | D19,D20 |
| 019 | Local failure | UNMODELED finance | No financial committed/working state | Future failure lowering | Future failure relation | Prefix/work/duty report | D20 failure branch |
| 020 | Ledger phases | UNMODELED | UNMODELED | Future phase lowering | Future phase relation | Actual phase observations | Future retained-fee specimen |
| 021 | K/Quint projection | Structural-value generic evaluator | Algorithm-only fixed arena; projection absent | No compiler transfer | No circuit transfer | Projection/replay tooling | Common-domain traces |
| 022 | Compiler mapping | Source reference only | Algorithm abstraction only | Whole admitted grammar obligation | Distinct circuit obligation | Public pipeline report | D10,D14 new-program pair |
| 023 | Native inventory | Source reference only | Finite abstraction only | Transition lowering | New/existing operation inventory | Native endpoint adapter | Future fixed-relation pair |
| 024 | Optimization | Reference behavior; helper-cost gap | Finite instance; no preservation result | Preservation obligation | Requalification obligation | Cost/evidence ledger | Future optimization proposals |
| 025 | Evidence report | Source class only | Source class only | Separate correspondence class | Separate native class | Tree/profile/target receipts | All rows |
| 026 | Required Clean stage | K→Lean semantic mapping UNPROVED | Bounded validation input only | Native-lowering correspondence OPEN | Bounded Clean step not implemented | Source-pinned inventory/receipts | UIREQ026 scenario |

## Existing U owners

These assignments schedule this draft under the existing roadmap; no parent exit closes here.

| U | Existing responsibility | Relevant draft clauses |
|---|---|---|
| U0 | Profile, semantics and target reconciliation | 001–026 |
| U1 | Semantic basis, bounds, primitive/representation contracts | 003–013,024,026 |
| U2 | General source-to-target compiler and developer consumers | 001–011,017–023,025,026 |
| U3 | Await, partial effects, duties, recovery and cost lifetime | 012–020 |
| U4 | Native recursion, composition and private-history scope | 014–016,021–026; existing Opus requirements unchanged |
| U5 | Optional services, federation and cost planes | 013,017,020,025 as interface inputs only |
| U6 | Full ACTUS/DeFi conformance | Demonstrators are small additions; no replacement coverage |
| U7 | Public developer tooling and release evidence | 001,002,022,025,026 |

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
| M15 | Complete financial library/conformance | Existing U6 denominator retained; D01–20 insufficient |
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
| Second-wave proposals | UIREQ-024 over relevant UIREQ rows | Must state unchanged schedule or explicit versioned change | No measurements yet | Per changed representation/lowering/constraint | NOT STARTED; await root freeze |

## Unchanged Opus inputs and unresolved compatibility

All five Opus brainstorm proposals are linked and hashed in inputs.json. Opus02 favors a universal code-as-data circuit; Opus05 favors starting with per-program step realizations after freezing machine/observations. Both remain live alternatives, with no author-added security judgment. Exact security reasoning and clauses stay in those unchanged external documents.

Concrete K uses dedicated recursive list/tree sorts rather than generic nominal declarations; its helpers have no individual fuel charges, and stack overflow is error4. Those facts require reconciliation with the broader design bound/cost contract and the Opus-owned UISEC-004/UISEC-K-02/UISEC-K-05 proposals. Compatibility is OPEN and substantive security assessment is reserved to Opus. Quint scalar15/refusal/tick accounting differs from exact UInt128/K fuel; it supplies no general code, lexical or financial model.

## Required Clean stage crosswalk

UIREQ-026 is required within existing U0 semantic/profile reconciliation, U1 primitive/representation/bounds, U2 compiler/native correspondence and U4 recursion/composition; U7 retains exact evidence reporting. It refines the existing work rather than creating a new milestone. M1/M5/M6 require the semantic/representation/resource mapping; M2/M3 retain independent target qualification; M10 retains public tooling evidence. MC01/MC04 retain semantic/compiler obligations, MC03/MC05/MC06 retain actual native recursion/financial/private composition obligations, and MC08 retains release evidence. Relevant existing ZR01/ZR06/ZR11/ZR12/ZR13/ZR16 and MNR08 remain unchanged; security interpretation/acceptance stays Opus-owned.

Required chain: K reference → Quint bounded validation → Lean recursive semantic types/fuel-run relation + separate fixed-capacity Clean step/correspondence stage → compiler/native integration. All new definitions/circuits/proofs/export work is unimplemented or unproved. Clean public-core reuse must use exact source hashes; local extensions are not pinned by HEAD alone. No Midnight exporter or backend compatibility follows from existing non-Midnight backends. See clean-stage.md.

Cross-repository obligation for UIREQ-026: Moriarty owns Sherlock language/OpenSpec; the existing private Fenrir Clean fork/worktree owns the required integration artifacts. Its actual revision/source/theorem/native-consumer record must bind the public semantics/model hashes. A public upstream research pin is not that integration pin. Public reporting uses only shareable dependency/evidence references, never private paths or code.
