# Original-fixture independent normal expectations: environment group

Status: hand-derived source expectations, specified-only/unexecuted. The sole write directory is this runtime package. No working-tree source, Git state, original fixture, original oracle or subject implementation was changed. No evaluator/compiler/circuit/witness output, actual K capture or subject code supplied an expected value, state, source rule or count. Python only substituted fixed text aliases, serialized explicitly supplied state rows and fuel-domain literals, and checked literal structure.

The falsifiable future predicate is: for F06bool, F07caller and F08mutual, an independently executed producer on the pinned source must supply exactly the complete code/control/environment/top-first frames/depth/stack limit/initial and remaining fuel/output/work state at every frozen charged boundary; each incoming rule must identify the listed pinned `main.k` source line. Startup and outcome publication must agree separately. Every ordinary initial budget from zero through exact completion plus one has a literal prefix row. Equality of terminal values alone does not satisfy the predicate. This package records no observation of that predicate.

`input-pins.json` binds the source bytes and records the historical pin-order caveat: the initial read batch exposed the three fixture texts just before pin creation. The normative `main.k` and normal literal bodies were first read from the pinned copies. The extra-normal pair was later copied and hashed solely to check preservation of the complete original eight-case/216-prefix set. This history is retained explicitly. Root supplied the observed baseline commit/tree; this worker did not run Git. The authoritative semantic source is pinned `main.k`, SHA256 `b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41`.

`fixture-normal-programs.json` and `fixture-normal-prefixes.json` use the existing literal format, `sherlock-k-a/1` profile and six-field sorted compact JSON input digest convention. `code_ref` resolves the entire ordered immutable table in `fixture-normal-programs.json`. Every state overlays the same fixed fields as the existing normal pack; `states[0]` is post-startup. The input case ID is part of its digest. Prefixes use the existing five-column reconstruction: initial budget, boundary state index, remaining budget, published output and publication rule line. The minimal-completion runs use the original stack limit 64. Original fuel 1000, original fixture-byte digest and unchanged source run remain separate metadata; each `original_fixture_run` supplies its full-work boundary and remaining fuel without replacing the minimal prefixes.

All declared machine steps cost one A fuel unit. Startup at line74, successful publication at line80 and exhausted publication at line84 cost zero. Helpers are normalized inside their owning charged edge and remain unmetered in A. There is no inferred zero helper cost, B helper work or attempted-work diagnostic.

| Case | Frozen fixture | Expected result | Exact charges | Ordinary initial budgets | Maximum depth | Original fuel remaining |
| --- | --- | --- | --- | --- | --- | --- |
| F06bool | uiBoolLetOrder.ui | uiU(7) | 20 | 0..21 | 3 | 980 |
| F07caller | uiCallerLocal.ui | uiU(12) | 21 | 0..22 | 3 | 979 |
| F08mutual | uiMutualEven4.ui | uiB(true) | 120 | 0..121 | 7 | 880 |

The source derivations below state observable transition facts and arithmetic counts. The complete controls and all frame/environment literals are in the JSON and the fixed-row transcription file.

## F06bool

Boundaries0..4 take the initial let through declaration, frame push, literal5 and binding. At4, environment is `0 |-> uiU(5)` and the saved scope frame contains `.Map`. Boundaries5..6 declare and push the conditional frame. Boundaries7..11 declare/push the unary-not frame, return false, compute the primitive true, and return true. At8..9 the top-first stack is unary-not, saved conditional, saved scope. At10..11 the unary frame has been removed. Boundary12 chooses the true branch and restores the conditional's saved environment; literal99 has no evaluation boundary.

Boundaries13..19 evaluate the selected addition in strict left-to-right order: declaration, push, variable5, right operand, literal2, primitive7, returned7. The right frame retains exactly left value5. Boundary20 pops the lexical scope and restores `.Map`, giving empty frames and returned7. The charge groups are `4+2+5+1+7+1=20`. No earlier returned value with a nonempty stack is terminal. Fuel19 therefore exhausts with returned7 and one scope frame; fuel20 publishes done7 for free at remaining0; fuel21 publishes the same done7 with remaining1.

## F07caller

Boundaries0..4 bind caller variable0 to10 with a scope frame saving `.Map`. Boundaries5..6 declare/push the outer addition's left frame, which saves caller environment10 and unevaluated right variable0. Boundaries7..10 evaluate call5's literal argument2, then pop its argument frame into invoke5. Boundaries11..14 perform named lookup, typed parameter binding, explicit return-frame declaration and actual push. Boundary13 still has caller environment10: `uiPush` has not yet switched to callee environment2. Boundary14 has callee environment2 and three top-first frames: return to environment10, outer-left, scope.

Boundary15 returns callee variable2. Boundary16 uses the typed return frame to restore caller environment10 while retaining the outer-left and scope frames. Boundaries17..20 switch to the right operand, return caller variable10, compute primitive12, and return12. Boundary21 restores `.Map` by popping the lexical scope. The groups are `4+2+4+4+1+1+4+1=21`. Fuel20 still retains the scope frame, so it exhausts; fuel21 publishes done12 at remaining0; fuel22 leaves remaining1.

## F08mutual

The ordered immutable table contains function6 followed by function7. Function6 returns true in its zero branch and otherwise calls7; function7 returns false in its zero branch and otherwise calls6. Both bind a fresh UInt parameter map and return Bool. The input is a named call6 on literal4. No recursive body expansion changes the source table.

Boundaries0..8 evaluate the initial literal argument, invoke/lookup function6, create typed binding4, declare the return frame saving `.Map`, and push it to enter body6 with parameter4. A call's `uiBound` edge declares `uiPush` while retaining the caller environment; only the next push edge installs the new parameter map. This is represented separately at7/8 and at31/32,55/56,79/80,103/104.

| Parameter/body | Body-entry boundary | Caller map saved by newest return frame | Next invocation boundary | Next body-entry boundary |
| --- | --- | --- | --- | --- |
| 4 / function6 | 8 | .Map | 28: function7 with3 | 32 |
| 3 / function7 | 32 | 0 -> 4 | 52: function6 with2 | 56 |
| 2 / function6 | 56 | 0 -> 3 | 76: function7 with1 | 80 |
| 1 / function7 | 80 | 0 -> 2 | 100: function6 with0 | 104 |
| 0 / function6 | 104 | 0 -> 1 | no further call | base returns at115 |

Each nonbase body descent takes24 charges from its entry to the next body entry. The false equality test and branch use10: conditional declaration/push, equality declaration/push, variable, right operand switch, zero literal, equality primitive, returned false, false branch. Strict evaluation of the subtraction argument and invocation uses10: argument declaration/push, subtraction declaration/push, variable, right switch, literal1, subtraction primitive, returned result, invocation. Lookup/binding/return declaration/push uses4. The table spells out each of the four distinct descents; there is no generated recursive state routine.

At104 the top-first return frames save environments1,2,3,4,`.Map`. Boundaries105..114 perform the zero equality and take the true branch; boundary115 returns true from its literal. Peak stack depth7 occurs while the base equality has its left/right frame above the conditional and five saved return frames. Boundaries116..120 pop one typed Bool return frame per charge, restoring parameter maps1,2,3,4,then `.Map` respectively. Each earlier returned true still has a return frame and is nonterminal.

Total charge is `8 + 4*24 + 11 + 5 = 120`. Fuel119 exhausts with `uiRet(uiB(true))`, environment4 and one return frame saving `.Map`; fuel120 publishes true at remaining0; fuel121 leaves remaining1. For the original fuel1000 run, all listed boundary states retain initial fuel1000 and remaining fuel `1000-charged_steps`, with final remaining880.

## Rule-source membership

`rule-source-labels.json` records each used rule's exact first line from pinned `main.k` and its line number. Rules102/108 evaluate valid literals/bound variables;114/117/120/123 declare let/binary/unary/conditional frames;131 declares nonempty arguments;135 pushes a frame;145 binds a let;149 switches binary operands;153/157 create primitive results;161/165 select conditional branches;181 completes argument evaluation;185 restores a typed function caller;195 restores lexical scope;200 looks up a named function;204 binds parameters;208 declares the return frame;223 returns a primitive result. Labels identify exact source rules, not fresh invented K rule attributes.

## Boundaries and limitations

These three provisional normal cases fill only their original frozen-fixture table gaps. Original normal JSON bytes, including the separately stored extra pair, are preserved. There is no hostile test, security judgment, theorem, B capacity claim, financial result, cross-layer correspondence, K execution, compiler execution, circuit result, native proof or target result. All retained product gates remain open. Root integrates and obtains the applicable independent audits separately.
