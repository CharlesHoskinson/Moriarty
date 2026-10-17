# Ordinary functional closure proposal: UIREQ003–006 and UIREQ008

Status: PROPOSED, not adopted. The frozen R79 candidate03 is unchanged. Root alone can adopt rows after fresh non-author review. This document does not authorize or run any interpreter, compiler, model, SDK, native or provider operation.

Propose ordinary typed-A host closure of UIREQ003, UIREQ005 and UIREQ006. Also propose UIREQ008 from the existing complete F04 capture, source analysis and the previously observed N06 first-match counterexample. Keep UIREQ004 partial/open: its exact nested-arguments scenario now passes, but required generic constructor-field evaluation remains missing. None of these decisions closes Candidate B implementation, the full39 map, generic declarations, correspondence, financial, constraint, native or release obligations.

The original clauses and named scenarios are copied without amendment in PROPOSED-ROWS.json. Each predicate remains the original SHALL clause. The interpretation is the same supported typed-A host realization used for Root's adopted UIREQ001/010/011: one versioned relation, finite immutable Program input, explicit machine state and admitted fixed diagnostic observations. This is not a new supported grammar or an exclusion of retained v1 obligations. UIREQ007/034 and SCON016 still require generic nominal types and constructor continuations; SCON005/006 still require complete A/B implementation maps and observations. No assertion that every interpreter implementation/profile has passed is made.

## Evidence and limits

R82 froze the 12,973-byte test source and independent EXPECTED.json before subject execution. The expected results, work and selected raw typed checkpoints were derived by hand from pinned K-A rules, not by Machine, a helper evaluator, a compiler or a model. R87 independently reviewed the source and supplied exact bounded runtime eligibility. R92 then externally compiled only this test file against the unchanged normal production Core library, enumerated exactly nine selectors, and ran each separately with fail-stop. Every selector returned exit0 and one passed test; all 407 recorded source/dependency/tool/input pins stayed unchanged. The result is actual diagnostic execution against the ordinary production library, not a cfg(test) replacement of the interpreter.

R92 results: lexical case22 charged/23 events; nested arguments48/49; factorial113/114; even120/121; odd145/146; each scalar7/8. Tests check complete independent initial/final snapshots, unchanged full Program throughout, every event's fuel/charge/outcome and source-rule membership, plus case-specific typed intermediate checkpoints. They do not contain a complete independently specified intermediate-state oracle. They do not execute K. Their line tags identify pinned K-source lines, not actual K execution. Ordinary ASCII test symbols are valid typed host names; K comparisons require canonical signed-decimal names. Old full-fixture observations are separate historical runs with numeric-name transcriptions.

R95-READBACK.json records a file-only verification of actual stdout/stderr hashes, selectors, exits, pins, oracle/test freeze and unchanged Machine/types source. This readback does not run subject code or infer semantic correctness from hashes. R83's production library digest is a captured link artifact; it is not a claim that a new full current tree was rebuilt. Production reference.rs and reference_types.rs are byte-identical across R87's frozen source, the R83 source snapshot used by R92, and current source. Current public exports are read separately; historical and current module roots remain separately pinned.

The R87 owner's H1 OPEN/native0 disposition is retained unchanged. These nine fixed normal cases carry only its exact prior eligibility. This assessment adds no security judgment or input authority. Previously observed ordinary underflow belongs to ImageMachine, a separate typed-A image consumer, and is supplementary evidence only; it is not an observed reference::Machine error-path run. Security ownership and negative-test execution authority remain unchanged. Source review supplies the generic-relation argument. A finite sample count alone cannot establish a universal theorem.

## UIREQ003 — propose closed for the original host functional clause

Original condition/response: WHEN a named call enters its body, evaluate it under declared parameter bindings and the global function table. Original scenario: outer x=7, callee binds x=2 and returns2, caller subsequently reads7.

reference.rs:114–138 uses the retained Program.functions table for every Invoke, then bind on declared parameters. bind:367–378 builds a fresh environment from only corresponding typed parameter/value pairs. Bound pushes Return with the caller environment and evaluates the body under that fresh environment. Return:279–286 restores the saved caller environment after the declared result-type check; Scope:288–291 restores let/match scope. There is no branch for a fixture, function name, recursion depth or captured caller variable.

R92 test1 is the exact original scenario. Record10 enters callee body under empty parameters and a Return frame saving x=7; record15 reads callee-local2; record17 restores caller x=7; record19 reads7 with ignored=2. Final result is7, with22 charges. The test's complete rule sequence and expected checkpoints are frozen independently. The broader declared-parameter part is exercised by test2's add(left=4,right=4), test3's n3→2→1→0 and tests4/5's mutually recursive n bindings, while all records retain the complete global Program.

Decision: source mechanism covers the original condition; the exact scenario and complementary parameter/global-table paths are observed. No ordinary source gap remains identified in this relation. This closes neither public compiler admission nor untrusted host-resource admission.

## UIREQ004 — keep partial/open

Original condition covers multiple evaluated operands OR fields, once each left to right. R92 test2 now establishes the exact add(double(2),increment(3)) scenario: Invoke double2, return4, Invoke increment3, return4, Invoke add(4,4), return8, each exactly once. Binary Left/Right and Args frames implement the ordinary supported operand relation.

The full retained requirement also reaches required generic constructor fields. Current Expr has no generic constructor expression or ConstructorFields continuation carrying constructor identity, saved environment, evaluated prefix and remaining field expressions. Dedicated Binary Cons/Branch and Unary Leaf do not establish the required generic constructor-field implementation. SCON016/UIREQ034 remain open. Therefore scenario coverage improves to observed but the whole clause stays partial/open. Missing work is implementation plus independent complete strict-field/recursive-field observations, not another repetition of the now-passing nested-argument case.

## UIREQ005 — propose closed for the original host functional clause

Original condition: a named self/mutual call applies the ordinary call/return relation. Original scenario: factorial3=6, even4=true, odd5=true within declared bounds.

The Invoke/Enter/Bound/Return code described for003 never distinguishes recursive calls. Callee lookup always uses the same retained global function table. Recursion is Program data, with explicit frames and the same type/binding rules. The declared within-bounds premise is explicit; arbitrary termination or admission of unbounded hostile ASTs is not the clause being closed.

R92 tests3–5 cover ALL three original examples. Factorial invocations are n3,2,1,0 and returns (0,1),(1,1),(2,2),(3,6). even4 descends even4,odd3,even2,odd1,even0 and all returns true. odd5 descends odd5,even4,odd3,even2,odd1,even0 and all returns true. Each selected invocation/return and per-event generic-rule/work condition is asserted against literal expectations. The original full-fixture factorial/even captures and N05/N08 independent-prefix observations provide prior complementary evidence, not new runs.

Decision: the generic relation is source-complete for this original condition and all stated scenarios now execute as expected. General all-program refinement and finite-bound-outcome evidence remain separately tracked.

## UIREQ006 — propose closed for the original typed scalar functional clause

Original condition: executing a scalar primitive produces its declared checked UInt128 or Bool outcome. Original scenario: add2,3=5; sub7,2=5; mul3,4=12; lt2,3=true.

Value::UInt stores u128 and Value::Bool stores bool. reference.rs:332–345 dispatches Add/Sub/Mul only on UInt pairs and uses checked_add/checked_sub/checked_mul, mapping absent results to error10. Lt returns Bool(a<b). Eq returns typed structural equality, and unary Not:357–361 returns Boolean negation. Other operand types reach error11. PrimitiveResult translates success to Return and failure to Error before the existing free publication. Source exhausts the declared typed-A scalar table; no wrapping arithmetic or fixture branch substitutes for the checked operation. Structural Eq on compound values is additional existing functionality; this row does not establish its future bounded helper implementation.

R92 tests6–9 are exactly the four original normal cases. Each has the literal eight-rule sequence, PrimitiveResult at record5, Return at6, free publication at7, exact full terminal state and7 charges. The separately observed F06 not(false)/addition and recursive Eq tests exercise further Bool paths. Existing147's ordinary underflow/checkpoint case supplies an actual earlier ImageMachine Rejected10 path with pending caller state retained. That image interpreter is a separate implementation, not reference::Machine. Its inspected source and literal assertions are identified in READBACK as supplementary evidence only; no reference::Machine overflow/underflow, rejection or security test was run in R92. There is no experimental claim over all2^128 inputs. The complete checked implementation is established by finite source case analysis; actual tests establish the stated normal scenario and complementary earlier branches. Max-input addition/multiplication failures have not newly executed in R92 and are not fabricated.

Decision: propose original typed-A scalar fulfillment from complete checked source plus original observed scenario. A reviewer may reject this sufficiency judgment; rejection must name a source gap or a missing required evidence class, not impose stagenet on this pure-host clause. UIREQ030 limb wrappers/constraints, B arithmetic realization, H1 and native/financial requirements remain open.

## UIREQ008 — propose closed from existing complete observations

Original condition: after an ordered constructor match's scrutinee completes, evaluate the first matching source arm under declared field bindings. Original scenario: sum(Node(Leaf4,Leaf5)) returns9; each leaf visited through its case, scrutinee once, unselected arm bodies absent.

Current source evaluates Match's scrutinee once under a saved Match frame:199–203. Return to Match:262–265 passes the completed VALUE to Cases. Cases:140–155 removes the first arm, computes its bindings, proceeds to remaining arms only on None, and on success discards remaining arms and evaluates ONLY the selected body under matched bindings with a scope-restoring frame. matching:383–404 binds list head/tail or tree leaf/branch fields into the saved environment. Later source arms cannot be evaluated by this success branch. This source argument is independent of whether a particular trace happened to return9.

The complete existing F04 original-fixture run is actual capture original_fixture_runs[3],61 records,60 charges, fuel1000→940, stack64, Done9/no-next. Its immutable independently authored oracle is fixture-normal-prefixes.json /traces/1. State12 holds the Branch value;13 misses Leaf;15 binds children Leaf4/Leaf5;32 binds left scalar4;51 binds right scalar5; state60 has Return9/empty frames; publication returns Done9. The complete reviewed comparator reported match across every field and all61 F04 ordinary fuel prefixes, not only these selected locators. R95 extracts the immutable raw record locators without generating expectations.

Existing N06 is a discriminating first-match counterexample: a matching Var arm precedes a matching Leaf arm whose body is99. Independently authored normal-prefixes.json /traces/5 states14–17 select Var, retain the outer Cons bindings, and return4; no state evaluates Literal99. The actual147 log records normal_n06_ordered_list_tree_match PASS; the test's compare_case function checks complete expected prefix states and one-step records. Relevant consumed current files match the recorded147 source pins. This is prior actual evidence, not an R95 rerun or proof of all147 source currentness.

Decision: propose closure of the original ordered-match functional mechanism and complete scenario on supported typed-A patterns. Generic nominal declaration/match-plan implementation remains required by034/SCON016 and is not inferred from dedicated List/Tree. This is not native evidence or a full-general language completion claim.

## Adoption and freshness

PROPOSED-ROWS.json contains exact replacement rows against the separately pinned BASE-CURRENT-EVIDENCE.json. All five new acceptance.status fields remain open;003/005/006/008 have proposedStatus closed and004 remains open. Existing adopted001/010/011 and026partial are untouched. Root must publish the source/result archive, obtain fresh non-author semantic review, then issue an additive adoption record and install only justified rows. The ledger and reviewer must not hash-pin one another cyclically.

Currentness pins include actual assertion/producer source, independent expectations, relevant machine/type/display/fixture and comparator sources. Runtime historical receipts retain their original path strings; sidecar refs use portable public repository paths. Changing a declared consumer source invalidates currentness. No full-tree, universal semantics, complete-B, stagenet or security pass follows from this metadata.
