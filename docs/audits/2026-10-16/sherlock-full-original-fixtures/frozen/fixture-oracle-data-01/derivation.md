# Provisional independent source expectations: F03/F04/F05

Status: hand-derived, specified-only, unexecuted, unreviewed. This packet is an
independent Oracle O source preparation for profile `sherlock-k-a/1`. It does
not report a subject-program result or establish correspondence, a theorem,
security acceptance, Phase 0 acceptance, finance, Candidate B or native support.

The falsifiable comparison predicate is: for each listed original source
fixture and each ordinary budget from zero through exact completion plus one,
a future profile-A producer agrees on the complete control, environment,
top-first frames, depth, ordered immutable code table, fuel fields, output and
charged work, including separate startup/publication edges. Future results
must preserve discrepancies rather than modifying these expectations to fit.

## Input boundary and identity

The worker loaded the Moriarty development skill, checkout `AGENTS.md`, product
contract, current execution-focus guidance and implementation-plan independent
Oracle O/Phase 0 requirements. The guarded read-only status reported historical
SP01 evidence blockers; no historical dispatch was attempted. The parent
selected the exact disjoint output directory and supplied provisional baseline
commit `c9f7cf6a712c7e6069cdc7a3ecdd86fb79817429` and tree
`fdbfef3b90b6e304e49d673b20ae262f30997aff`. No Git command was used by this worker.

`input-pins.json` records source hashes before derivation. Main semantics and
the three assigned fixtures were pinned before their bodies were read. The
pre-existing normal program/prefix pack supplied the literal data format,
profile, source locator convention and input-digest convention. The extra pack
was pinned for preservation; its contents supplied no fixture expectations.
The four existing normal/extra-normal oracle files remain untouched.

No runtime/reference implementation, circuit, witness, evaluator, compiler
output, new subject implementation or actual K capture was read. No K, Lean,
Quint, Rust, Cargo, SDK, native, version, hosted-CI, key or cache command ran.
The only executable preparation is Python serialization and assertions over
already derived fixed literal tables. `serialize-literals.py` contains every
explicit row, with formatting aliases for repeated K source text and complete
saved environment/frame suffixes. It contains no expression interpreter, rule
selection, semantic dispatch, fixed-point search or generic transition engine.

## Representation and publication

`fixture-normal-programs.json` uses the existing input format and SHA256
convention. Its original-fixture metadata retains the exact source file hash,
fuel 1000 and stack limit 64. All minimal prefix inputs also use stack limit
64; only the ordinary budget changes. No original fixture is rewritten.

`fixture-normal-prefixes.json` uses the unchanged existing representation:
each state supplies full control, environment and top-first frame array,
depth, remaining fuel and charged index; fixed fields supply the immutable
code reference, initial fuel, stack limit and pending output. Every code
reference resolves a source input by both case ID and input digest. The array
order reconstructs nested `uiFrame` terms ending in `uiNoFrames()`; K Map
entry order has no semantic effect.

The stored `states` use exact-completion initial fuel. For every ordinary
budget, the explicit five-column prefix row supplies initial fuel, boundary
index, remaining fuel, published output and publication source rule. State
indices and every intervening charged state are retained by the existing
reconstruction convention. Work equals the boundary index. Every budget
less than exact completion exhausts at rule84. Exact completion publishes
success using rule80 with remaining zero; completion-plus-one publishes the
same result with remaining one. No terminal machine charge is added.

Each `original_fixture_run` separately reconstructs all original-budget states:
the same complete control/environment/frame/code snapshots are overlaid with
initial fuel 1000 and an explicitly listed remaining-fuel value for every
state. It includes the original initial configuration, startup metadata,
publication metadata and complete terminal snapshot. This is a source
expectation for the full frozen fixture, distinct from minimal ordinary
prefixes. Original terminal remaining fuel is 907, 940 and 987 for F03, F04
and F05 respectively.

All successful literals and intermediate UInt values are in UInt128 range.
Pure helper normalization is included in the charged rule that calls it,
according to pinned main.k; helper work is unmetered in A and is never filled
with zero. No helper-microstep B count is assigned.

## Exact source locators

The pinned `main.k` has no explicit user rule labels for these rules. This
packet uses the existing exact `rule_start_line`/`via_rule_start_line` labels.

| Source rule start line | Source rule meaning used by these fixtures |
|---|---|
| 74 | Startup installs code, fuel, initial fuel, stack limit and entry control; charge 0 |
| 80 | Empty-stack return publishes `uiDone`; charge 0 |
| 84 | Nonterminal zero-fuel control publishes `uiExhausted`; charge 0 |
| 102 | Valid literal evaluation returns its value; charge 1 |
| 108 | Bound variable evaluation returns its value; charge 1 |
| 117 | Binary expression schedules a separate left-frame push; charge 1 |
| 126 | Match expression schedules a separate match-frame push; charge 1 |
| 131 | Nonempty call schedules its first argument frame; charge 1 |
| 135 | Scheduled frame push installs its requested environment; charge 1 |
| 149 | Left operand return replaces left frame with right frame; charge 1 |
| 153 | Right operand return removes frame and produces normalized primitive result; charge 1 |
| 173 | Match scrutinee return removes match frame and begins ordered cases; charge 1 |
| 181 | Last argument return removes argument frame and invokes; charge 1 |
| 185 | Correctly typed return removes return frame and restores caller; charge 1 |
| 195 | Scope return removes scope frame and restores saved environment; charge 1 |
| 200 | Invoke resolves the named function in the immutable code table; charge 1 |
| 204 | Enter computes normalized parameter bindings; charge 1 |
| 208 | Bound parameters schedule a separate saved-caller return-frame push; charge 1 |
| 214 | Ordered case testing produces normalized match/no-match control; charge 1 |
| 217 | Unmatched arm advances to the next source arm; charge 1 |
| 220 | Matched arm schedules a separate scope-frame push; charge 1 |
| 223 | Primitive value returns as a value; charge 1 |

Pure helper sites used include validity (230–235), successful type checks
(238/240/241), checked addition (250/254), cons construction (259), named
lookup (282), argument append (286), parameter binding (290/291/294),
successful nil/cons/leaf/branch matching (301/302/304/305), nil/leaf-arm
no-match guards (307/310) and their kind classification (316–319).
These are source locators rather
than extra charged events; the machine edge labels above remain authoritative.

## F03: uiListsum123.ui

Source facts: function ID3 has one list parameter ID0, a nil arm returning
zero, then a cons arm binding head1/tail2 and evaluating
`head + call3(tail)`. The entry supplies literal list `[1,2,3]`.

Hand-derived expectations: exact result `uiU(6)`, charged work 93, maximum
frame depth 11. The initial call takes eight charged rules
`131,135,102,181,200,204,208,135`. Each nonempty list has a nine-rule match
sequence `126,135,108,173,214,217,214,220,135`, including the nil-arm miss.
The add starts with four rules `117,135,108,149`, retaining the head value in
the pending right frame. The recursive tail call then takes eight rules
`131,135,108,181,200,204,208,135`. There are three such nonempty levels.

The nil callee takes ten remaining local rules
`126,135,108,173,214,220,135,102,195,185`, including scope and typed return.
Each suspended addition then takes four rules `153,223,195,185`. The exact
count is `8 + 3*(9+4+8) + 10 + 3*4 = 93`.

| State interval | Fixed literal milestone |
|---|---|
| 0–8 | Initial argument, lookup, fresh list123 parameter environment, return frame |
| 9–17 | List123 matching; cons scope contains IDs0,1,2 |
| 18–21 | Head1 returned; pending add-right frame stores `uiU(1)` |
| 22–29 | Tail `[2,3]` callee entry; saved caller still contains list123/head1/tail23 |
| 30–38 | List23 matching; fresh cons scope contains list23/head2/tail3 |
| 39–42 | Pending add-right frame stores `uiU(2)` above outer pending add1 |
| 43–50 | Tail `[3]` callee entry with full list23 caller restoration frame |
| 51–59 | List3 matching; fresh cons scope contains list3/head3/nil |
| 60–63 | Pending add-right frame stores `uiU(3)` above add2 and add1 |
| 64–71 | Nil callee entry with full list3 caller restoration frame |
| 72–81 | Nil matches first arm; literal zero, scope exit and nil return |
| 82–85 | `3+0=3`, primitive return, list3 scope exit, restore list23 caller |
| 86–89 | `2+3=5`, primitive return, list23 scope exit, restore list123 caller |
| 90–93 | `1+5=6`, primitive return, list123 scope exit, restore empty caller |

All four callee parameter environments are fresh. Successful cons scopes
retain parameter0 alongside head1/tail2. No recursive call inherits a caller's
head/tail bindings as its own parameter environment. The deepest nil scope
retains four return frames, four scope frames and three pending additions.

## F04: uiTreesum45.ui

Source facts: function ID4 has one tree parameter ID0, a leaf arm binding
value1, then a branch arm binding left1/right2 and evaluating
`call4(left) + call4(right)`. Entry supplies branch(leaf4,leaf5).

Hand-derived expectations: exact result `uiU(9)`, charged work 60, maximum
frame depth 5. Initial call takes eight rules. Branch matching takes nine
rules, including the leaf-arm miss. Binary scheduling/push takes two rules.
Each leaf call takes eight call rules and ten local leaf rules, for 18.
The left-return switch takes one rule149. Primitive reduction/return takes
two rules153/223. Root scope exit and typed return take two rules195/185.
The exact count is `8 + 9 + 2 + 18 + 1 + 18 + 2 + 2 = 60`.

| State interval | Fixed literal milestone |
|---|---|
| 0–8 | Initial branch argument and fresh root callee |
| 9–17 | Leaf-arm miss, branch-arm match, full left/right branch scope |
| 18–19 | Left frame retains the exact right-tree call and branch environment |
| 20–27 | Left leaf call; caller branch environment saved in return frame |
| 28–37 | Leaf4 matches first arm; binding1 is UInt4; restore full branch caller |
| 38 | Binary frame becomes right frame storing UInt4; right-tree call begins |
| 39–46 | Right leaf call; caller branch environment saved independently |
| 47–56 | Leaf5 matches first arm; binding1 is UInt5; restore branch caller |
| 57–58 | Checked `4+5=9` then primitive return |
| 59–60 | Root scope exit then typed return to empty caller |

The left leaf's pending outer frame preserves the exact right call while
its own parameter0 is leaf4. The right leaf retains the completed left value4.
Leaf scopes restore their own parameter-only environment before the function
return restores the root branch environment.

## F05: uiConstructList.ui

Source facts: there are no functions. Entry is the source-ordered expression
`cons(1,cons(2,nil))` using binary cons primitives.

Hand-derived expectations: exact result
`uiCons(uiU(1),uiCons(uiU(2),uiNil()))`, charged work 13, maximum frame depth2.
A binary over two literals takes seven charged rules
`117,135,102,149,102,153,223`. Replacing the outer right literal's one rule
with the seven-rule inner binary gives `7+7-1=13`.

| State interval | Fixed literal milestone |
|---|---|
| 0–4 | Outer binary schedules left1, then stores it in a right frame |
| 5–9 | Inner binary schedules left2, then evaluates nil with both right frames retained |
| 10–11 | Construct exact inner cons2/nil value and return it |
| 12–13 | Construct exact outer cons1/inner value and return it |

The environment remains empty in every state. The inner right frame contains
UInt2 and sits above the outer right frame containing UInt1. Field order is
retained in both primitive values.

## Prepared artifacts and limitations

The three tables contain 169 states: 94+61+14. Their ordinary budget tables
contain 172 rows: 95+62+15, covering every integer budget through completion
plus one. The original-budget overlays also contain 169 explicit rows.
Source-static assertions check JSON parsing, digest conventions, field
completeness, row counts, frame/depth agreement, fuel arithmetic, terminal
snapshots and preservation of all input pins. These checks do not evaluate
the programs and do not establish agreement with a producer.

No existing normal case, prefix or fixture was mutated. No helper-microstep,
finance, hostile/security, formal, native or target expectation is supplied.
Future integration, producer execution, independent review and per-layer
comparison remain outside this packet's completion claim.
