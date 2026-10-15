# Sherlock: thirty-six normal demonstration contracts

All expectations below are independently written literals, not generated evaluator output. Candidate B acceptance status for all 36 rows: SPECIFIED / UNEXECUTED. Historical A observations are recorded separately and do not execute or validate these B contracts. The source inventory is reconciled in [model-coverage.md](model-coverage.md). Construct availability and an exact fixture are distinguished below; neither is an executed result. Fuel literals are pending rule/cost freeze. Finite first instances cannot establish unbounded support.

Notation: functions named here are ordinary program inputs, not new semantic primitives. `sum` follows list/tree shape; `double(n)=n+n`, `increment(n)=n+1`; `fact(0)=1`, `fact(n)=n*fact(n−1)`; `sumN(0)=0`, `sumN(n)=n+sumN(n−1)`. Each program supplies its declarations and bodies.

| ID | Normal specimen | Independent expected observation | Proposed model coverage / remaining gap | UIREQ |
|---|---|---|---|---|
| D01 | literal 7 | Halted UInt128 7 | K literal construct present; no exact fixture; Quint has no general literal-expression program | 006,011 |
| D02 | let x=2 in x+3 | 5 | K let/add constructs present; no exact fixture; Quint has no let/environment | 003,006 |
| D03 | if true then 9 else 4 | 9; only then branch evaluated | K if/Bool constructs present (separate uiBoolLetOrder expects7); Quint has no general Bool/if expressions | 004,006 |
| D04 | add(add(1,2),3) | 6; left nested add completes before right literal | K add/context constructs present; no exact fixture; Quint lacks arbitrary expression loading | 004,013 |
| D05 | let x=7 in add(identity(2),x) | 9; caller x remains 7 | K scope/call constructs present (separate uiCallerLocal expects12); Quint has no lexical environment | 003,010 |
| D06 | add(2,3) | 5 | K add present; Quint arithmetic only within its three algorithm transitions | 006 |
| D07 | sub(7,2) | 5 | K sub present; no exact fixture; Quint has no general sub-expression program | 006 |
| D08 | mul(3,4) | 12 | K mul present;12 fits scalar15 but Quint has no arbitrary multiply-expression program | 006,021 |
| D09 | eq(3,3) | Bool true | K structural equality present; pure-helper traversal accounting open; Quint lacks generic equality expressions | 006 |
| D10 | fact(3) | 6; pending non-tail multiplication frames | Exact K uiFactorial3 fixture has no rewrite observation; historical A Quint factorial3Test passed; B acceptance unexecuted | 001,005,010 |
| D11 | sumN(3) | 6; repeated call/return | Exact K uiSum3 fixture; Quint has no separate sumN program | 005,010 |
| D12 | even(4), with even/odd mutual calls | true | Exact K uiMutualEven4 fixture; Quint has no mutual-recursive code table | 005 |
| D13 | sum(Nil) | 0 | K Nil/list-case constructs present; Quint visits Nil within list123 but has no empty-list initial scenario | 007,008 |
| D14 | sum([1,2,3]) | 6; three head visits | Exact K uiListsum123 has no rewrite observation; historical A Quint listSum123Test passed; B acceptance unexecuted | 001,007,008 |
| D15 | mapIncrement([1,2]) | [2,3] | K UInt-list constructors/calls support expression vocabulary; no map fixture; Quint has no allocation/map program | 007,009 |
| D16 | sum(Node(Leaf(4),Leaf(5))) | 9; left result retained while right executes | Exact K uiTreesum45 has no rewrite observation; historical A Quint treeSum45Test passed;9 fits scalar15; B acceptance unexecuted | 008,010 |
| D17 | sumNested([[1,2],[],[3]]) | 6 | UNMODELED in both: K lists contain UInt only, not lists; Quint has fixed UInt list/tree arena | 007,008 |
| D18 | add(double(2),increment(3)) | 8; double completes before increment starts | K nested named-call/add vocabulary present; no exact fixture;8 fits scalar15 but Quint has no such program | 003,004 |
| D19 | 10+inner(3), inner waits then returns 4 | Waiting retains caller frame; resumed result 14 | K wait UNMODELED; Quint has factorial pending-return snapshot, not this general caller/environment case; durable adapter future | 014,015,018 |
| D20 | Two paid segments from reserve 100/liability 40 | Prefix: reserve68/liability10/debit32/receipt30; successful continuation: reserve57/liability0/debit43/receipt40; local pre-commit failure retains prefix68/10 and later work | UNMODELED finance in both; Quint has no working/committed store or duties; exact universal boundary/phase mapping OPEN | 017–020 |

The first machine-generality pair is D10 plus D14 with unchanged semantics/configuration; D16 adds branching recursion. D19/D20 are full-scope obligations, not prerequisites for claiming the pure fragment's narrower source inventory. Additional normal witnesses include a run exhausting fuel after progress, repeated-subtree traversal charged twice, and false/not-ready observation preserving a waiting checkpoint. They supplement these twenty contracts and remain unexecuted.


## Candidate B interface and implementation comparisons

The following sixteen contracts integrate the ten [academic/project proposals](../../../docs/research/sherlock/papers/README.md) and the [optimization comparison contract](../../../docs/research/sherlock/optimization/10/proposal.md). They use the shared [interface contract](interfaces.md). All values below are independently specified mathematical/source expectations. A separate oracle author must freeze complete intermediate reference states, remaining/consumed work and ordinary exact-prefix limits before implementation comparisons; this table contains no invented fuel thresholds. Shared metadata must never generate both behavior and oracle. No model, build, native, proof or benchmark execution accompanies these additions.

| ID | Normal specimen | Independent expected observation | Comparison and remaining gap | UIREQ |
|---|---|---|---|---|
| D21 | combine(a,b,c)=a+10*b+100*c, called with (1,2,3) | 321; arguments evaluated and bound in source order | Direct AST versus ProgramImage and binding cursors; generic Quint code/lexical model absent | 003,004,027,028 |
| D22 | let x=7 in add(let x=4 in x,x) | 11; final caller environment still maps x to7 | Full-map versus persistent/slot environments; exact intermediate oracle pending | 003,010,029 |
| D23 | sum(Node(Leaf(4),Leaf(4))) with shared or separately allocated leaves | 8; two semantic leaf visits under the same schedule | Structural tree/arena decoding and equality cursors; helper physical visits and bytes reported separately | 007,009,013,028,029 |
| D24 | Two source occurrences of increment(3), plus an unused declared function, in one immutable program image | Each selected call returns4; distinct source sites and original full function order remain present | Cold/warm preparation and expanded-step/source-site mapping; no selected-path-only preparation or recursive expansion | 001,005,022,027 |
| D25 | Ordered lookup past unusedFunction to identity(3) and equality of two independently built [1,2] values | Function result3 and equality true; helper cursor resumes the same pending caller state | A-preserving physical helper path and proposed helper-metered profile compared separately; exact work/caps pending independent freeze | 009,012,013,028 |
| D26 | (2^32−3)+8 | 2^32+5 | Named addCarry_b and checkedU128Add across byte/four-U32/two-U64 layouts; no actual component/circuit result | 006,030 |
| D27 | (2^64+9)−7 | 2^64+2 | Named subBorrow_b and checkedU128Sub, preserving limb order and exact semantic meaning | 006,030 |
| D28 | (2^48+1)·3 | 3·2^48+3 | Named full mulWide_b outputs and checkedU128Mul; full intermediate product retained before checked selection | 006,030 |
| D29 | Encode/decode fact(3) while fact(2) is active with multiplication-by3 pending | Reconstructed control/environment/ordered frame and structural values equal the independent reference state | MachineImage Bounds F and RepresentationRelation; in-domain capacities and exact snapshot pending; no representation theorem claimed | 010,014,026,029 |
| D30 | Materialize the same tree45 pending-right-call StepRecord through uncached, cold and warm witness paths | Same complete decoded reference prefix and final9; same occurrence work under each identical selected schedule | Typed DAG order, input bindings, lifetime/streaming variants and cache scope; host visits/time/memory separate | 013,024,035 |
| D31 | Compile one supported scalar/frame step and request each export capability separately | Distinct ConstraintPlan/WitnessPlan and feature dispositions identify actual emitted content | Basic JSON, structured witness, native closure, lookup/interact/custom-gate/field support inventoried independently; tooling Unsupported has no application-result meaning | 022,023,025,026,031 |
| D32 | Group fact(3) around a pending return with m1/2/4/8 and ordinary finite fuel prefixes | Final6 when complete; at every cut the same earliest reference state/stop, pending frames and exact cumulative work as m1 | ReferenceSlice/StepRecord oracle before implementation; preserve A free publication and frame-cap classification; padding creates no source work or commit | 010,012,013,016,032 |
| D33 | Universal/family/per-program residual steps on fact3 and list123 under the same profile/bounds | Each returns6 with identical decoded ordered reference prefixes and stops | Finite static worklist falls back to generic dispatch at residual limit; dynamic recursion/fuel/frames/caps retained; no measured realization winner | 001,005,024,033 |
| D34 | New Schedule declaration and ordered arms from UIREQ-034 on Both(Pay(4,Done),Pay(5,Done)) | Result4; first source arm only; scrutinee evaluated once | Direct ordered matching versus first-column MatchPlan; field paths, binding environment and source priority retained | 004,007,008,034 |
| D35 | Structural sum of that Schedule; then shared/unshared Both(Pay(4,Done),Pay(4,Done)) | 9 for first value,8 for repeated value; both child occurrences traversed | New declaration/layout required; direct/shared plan comparisons retain structural identity, workMap and pending frames | 007,009,013,034 |
| D36 | Sum UInt128 amounts [10^12,2·10^12,3·10^12] | 6·10^12 | Normal composed list/arithmetic workload beyond scalar15; exact primitives, helpers and layouts require new implementation/evidence | 006,007,028,030 |

The eight historical K fixture expectations (factorial3=6, sum3=6, list123=6, tree45=9, Bool/let7, caller-local12, mutual-even4=true and the constructed-list literal) remain unchanged. D05's caller-local9 and D22's shadowing11 are separate contracts, not edits to caller-local12. Old fixture sources, expected JSON and model observations are immutable for this design edit. Their exact hashes and scope are in [traceability.md](traceability.md#historical-observations-that-do-not-validate-candidate-b).

D19/D20 retain existing Core/2 successful-prefix await effects, residual duties and checkpoint atomicity as independent acceptance inputs: proofchunk ≠ financialcommit. Exact future Commit/Yield/Await opcodes and target phase mapping remain open. All required nonsecurity observations include ordered effects, folded financial state, fees/gross debit, duties, control/environment/frame meaning, outcome/reason and selected-schedule work when applicable. Pure demonstrations omit those extensions explicitly; they never close financial/native or conformance exits. Security scenarios and judgments belong exclusively to Opus and are not supplied here.
