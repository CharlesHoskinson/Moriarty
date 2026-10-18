# Independent positive nominal v2 derivation — withheld from the model author

The seven normal cases were selected and hand-derived from original UIREQ-034, SCON-016 and SCON-016.R02 before v2 reduction-rule authoring. Only declarations, source input identities, frame/control/event payload definitions and ordinary scoping conventions were coordinated with the model author. No forthcoming v2 rule/source implementation or actual model output was read or executed. The author previously implemented unrelated financial observation plumbing; that is disclosed and supplies no nominal semantic expectation.

The JSON was serialized from a finite, hand-unrolled event ledger. Small record-building helpers expanded explicit frames and event rows. They did not load a semantic implementation, walk/evaluate the input AST, choose a source arm, recurse over a value, calculate a program's sum or run a parser/model. Result literals and every source-arm choice were supplied directly in the ledger below. This is independent expectation preparation, not actual model evidence.

## Shared declarations and identities

Schedule type20/group9 has source constructors21 Done(),22 Pay(UInt0,Schedule20),23 Both(Schedule20,Schedule20). Fields always use zero-based source order. ExprId i has explicit input occurrence10*i; this convention names this finite input, not a new language identity algorithm. Function names50/51/52 are sum/first/copy. Expression values contain only UInt/Bool/Nominal(type,ctor,ordered values), with no arena or source occurrence payload. Source occurrence and completed field identities reside separately in events and frames.

The input Both at2000 has left Pay2001 with literal4 at2002 and Done2003, then right Pay2004 with literal5 at2005 and Done2006. The repeated input2100 uses distinct left2101/right2104 Pay expressions, distinct literal2102/2105 and distinct Done2103/2106, although their extensional values are equal. These identities cannot be reconstructed from equal values. Constructor completion orders are respectively2003,2001,2006,2004,2000 and2103,2101,2106,2104,2100. This order records complete child construction before parent construction and strict left-before-right fields.

## Literal result derivations

| Case | Hand derivation |
|---|---|
| construct-asymmetric | Both(Pay(4,Done),Pay(5,Done)); left and right are unequal ordered fields |
| construct-repeated | Both(Pay(4,Done),Pay(4,Done)); equal extensional fields retain distinct source events |
| structural-sum-asymmetric | sum(Done)=0; left4+0=4; right5+0=5;4+5=9 |
| structural-sum-repeated | Two separate Pay4 occurrences each4+0=4; left4+right4=8 |
| ordered-first-arm | Source arm7100 Both(Pay(x,_),_) matches first; x binds4 at field path[0,0]; result4; arms7101–7103 and sum calls are not evaluated |
| recursive-constructor-fields | Outer Both2500 first evaluates copy(Pay4Done) through call2501, then copy(Pay5Done) through call2503; each Pay copy recursively calls copy(Done) from source1222 before its own materialization; result is the asymmetric ordered Both |
| zero-field-constructor | Done2600 has no field requests or synthetic field values; start, separate materialization/build, finish |

The sum body match1000 uses arms7000 Done,7001 Pay(n,t),7002 Both(l,r) in order. Both first misses Done and Pay, then binds l/r at[0]/[1]. Each Pay misses Done, binds n/t at[0]/[1], evaluates n before its tail call, and combines its numeric result only after the nested sum(Done) returns. The left call1031 and right call1033 have different source occurrences even for equal Pay4 inputs. Each complete expected event trace spells out every operation-specific event in the declaration API, including all misses, selections, argument returns, function entries/returns, arithmetic and finishes. The exact trace compares the first matching arm; a bare result4 does not substitute for it.

## Full constructor and recursive snapshots

Every event in every case has a complete normalized expected after-state, plus an initial and final state. The immutable complete Program is stored once as shared_program in its case fixture. State program_binding denotes that exact object with the case's integer entry; events_prefix_length denotes the exact complete prefix of expected_events. Expanding those two references yields all five declared state components: program, control, environment, frames_top_first, events. These references compress repetition without omitting any program, event or state field; a consumer must compare the fully expanded state or enforce the same exact references and prefix. They are not semantic approval based only on a hash.

A ConstructorFields frame contains original constructor ExprId/occurrence/type/ctor, complete declared field types, active cursor/ExprId, evaluated prefix, tail-only remaining expressions and exact saved environment. Actual stack tail is its eventual caller context. A returned field's pre-state keeps that frame unchanged under Return(value); the return event names the original active field ExprId/occurrence, appends its value, and either requests the next field or enters the distinct Materialize control. Materialize holds complete ordered values and original constructor source identity after the field frame is popped. ConstructorBuilt is a subsequent event; no parent allocation is pretended before all fields return.

For recursive construction, outer2500 cursor0 retains active2501,prefix[],remaining[2503] across copy's function calls and matches. During copy(Pay4Done), Pay1220's first field1221 produces4. Its last field1222 calls copy(Done) while prefix[4] remains pending. Done1210 is constructed in the callee, then returns through function call1222. The enclosing field-return event must name completed_expr_id1222/occurrence12220, not callee leaf1210. Pay1220 then materializes and call2501 returns; outer first-field return names2501, never1220. Outer cursor1 retains prefix[Pay4Done] and active2503 across the analogous nested right recursion, whose numeric prefix is5. Only then may Both2500 materialize.

The recursive constructor build source order is1210,1220,1210,1220,2500. Repeated execution of the same copy-body source identity is represented by separate ordered events and distinct caller stack contexts; it is never collapsed into extensional value identity. The snapshots preserve the entire original outer saved environment (610 leftPay4,611 rightPay5,612 sentinel77). A callee environment contains parameter600 only; arm selection adds its local bindings. ArmScope restores that base, FunctionReturn restores the exact caller environment, and the final environment equals the initial environment with no remaining frames.

## Qualification

This is the finite normal reference domain declared in the safe API, with 19 operation-specific event variants available. One event per reference-model advancing operation is an observation convention, not an A/B work/fuel/helper/circuit clock, not a new native tag or error policy. No such counts or correspondences are claimed. Observer-prefix snapshots do not constitute persistent export/import, a canonical codec, native allocation proof, or an actual technical-cut execution. Positive field prefixes are explicit semantic input states for that later independent comparison.

These cases cover self-recursive Schedule, construction, first-match order, unequal/equal-looking source occurrences, named calls and pending fields. They do not establish whole original034/SCON016 closure, a full source parser/typechecker, bounded native generic admission, mutually recursive multi-type runtime, parameterized types, failures/releases, hostile/security behavior, financial lowering, K/Quint/Lean-Clean/native correspondence, persistent import or actual execution. Existing retained gates and designated security owners are unchanged.
