# Sherlock: first twenty normal demonstration contracts

All expectations below are independently written literals, not generated evaluator output. Status for every row: SPECIFIED / UNEXECUTED. The source inventory is reconciled in [model-coverage.md](model-coverage.md). Construct availability and an exact fixture are distinguished below; neither is an executed result. Fuel literals are pending rule/cost freeze. Finite first instances cannot establish unbounded support.

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
| D10 | fact(3) | 6; pending non-tail multiplication frames | Exact K uiFactorial3 fixture and Quint factorial3Test source; both unexecuted | 001,005,010 |
| D11 | sumN(3) | 6; repeated call/return | Exact K uiSum3 fixture; Quint has no separate sumN program | 005,010 |
| D12 | even(4), with even/odd mutual calls | true | Exact K uiMutualEven4 fixture; Quint has no mutual-recursive code table | 005 |
| D13 | sum(Nil) | 0 | K Nil/list-case constructs present; Quint visits Nil within list123 but has no empty-list initial scenario | 007,008 |
| D14 | sum([1,2,3]) | 6; three head visits | Exact K uiListsum123 and Quint listSum123Test sources; both unexecuted | 001,007,008 |
| D15 | mapIncrement([1,2]) | [2,3] | K UInt-list constructors/calls support expression vocabulary; no map fixture; Quint has no allocation/map program | 007,009 |
| D16 | sum(Node(Leaf(4),Leaf(5))) | 9; left result retained while right executes | Exact K uiTreesum45 and Quint treeSum45Test sources;9 fits scalar15; both unexecuted | 008,010 |
| D17 | sumNested([[1,2],[],[3]]) | 6 | UNMODELED in both: K lists contain UInt only, not lists; Quint has fixed UInt list/tree arena | 007,008 |
| D18 | add(double(2),increment(3)) | 8; double completes before increment starts | K nested named-call/add vocabulary present; no exact fixture;8 fits scalar15 but Quint has no such program | 003,004 |
| D19 | 10+inner(3), inner waits then returns 4 | Waiting retains caller frame; resumed result 14 | K wait UNMODELED; Quint has factorial pending-return snapshot, not this general caller/environment case; durable adapter future | 014,015,018 |
| D20 | Two paid segments from reserve 100/liability 40 | Prefix: reserve68/liability10/debit32/receipt30; successful continuation: reserve57/liability0/debit43/receipt40; local pre-commit failure retains prefix68/10 and later work | UNMODELED finance in both; Quint has no working/committed store or duties; exact universal boundary/phase mapping OPEN | 017–020 |

The first machine-generality pair is D10 plus D14 with unchanged semantics/configuration; D16 adds branching recursion. D19/D20 are full-scope obligations, not prerequisites for claiming the pure fragment's narrower source inventory. Additional normal witnesses include a run exhausting fuel after progress, repeated-subtree traversal charged twice, and false/not-ready observation preserving a waiting checkpoint. They supplement these twenty contracts and remain unexecuted.
