# R115 independent recursive-source static expectations

This packet contains populated values for the complete two-program R112 expectation interface. It manually derives source/preparation objects and all twelve ProgramView fields for the exact R100 specimens. No parser, checker, AST bridge, owner preparer, compiler, evaluator, model or native path was executed or imported. No actual artifact/table output or author-computed table records were used. Requested real source-order/type/field contracts were inspected; this is independent expected-data authorship, not a blind source audit or a proof of general compilation correctness.

## Exact input and proposed interface

The177-byte ChainTotal and211-byte BranchTotal files, including final LF, are copied directly from R100 and verified byte-identical to R112's copies. Source digest is lowercase SHA256 of those exact bytes. Source identity is the proposed R112 draft language `moriarty-sherlock-draft/1`, distinct semantics `moriarty-sherlock-dedicated/1`, actual source program name and tagged `{kind:"entry"}` selection. No Core action/index or external input is fabricated.

`EXPECTED.json` has exactly `chain_total` and `branch_total`, each with mandatory source_digest/source/preparation/program_view. It is directly shaped for R112 `TwoProgramExpectations`; no missing field, null wildcard, default table or inferred runtime value is used. The null SourceLocation.function_index for entry is the real typed Option value, not an uncertainty. Owner/source-index paths and indices are canonical decimal strings; source-span endpoints remain numeric u32. Scalar payloads/owner IDs follow the existing OwnerWire strings. All array orders and multiplicities are preserved.

Preparation is the actual proposed dedicated-source report shape: kind, complete original function/parameter declarations, provenance, checked type summary and descriptive owner_mapping. The latter keeps R112's literal `correspondence:"unestablished-until-independent-comparison"`; expected join data does not establish observed correspondence on its own. The interface and source/report schema are proposals pinned by exact source bytes. A later adopted schema or parser/mapping change requires an additive expectation amendment, not silent normalization.

## Normative table order and source inspection

The prior independently frozen B schema is the table-order authority: entry occurrence preorder first, then function bodies in original declaration order; semantic child order; exact UTF8-sorted distinct names; Call argument slots in ExprId order; contiguous Match arms/pattern rows; full occurrence paths appended in row order; functions/parameters in source order; constants in literal ExprId order, postorder within each literal, no interning. Empty slices normalize to start0/len0. DedicatedSorts has no types/constructors/constructor-fields/match-plan tables in ProgramView.

The actual draft parser creates functions before reading entry but explicitly collects provenance entry first, then original bodies. Match children are scrutinee0 and arm-k body k+1; constructors retain their own Binary/Unary nodes. The checker checks every function under fresh parameter sorts, then closed entry. Named calls consult declarations without unfolding recursion. The structural A-to-B bridge preserves names, declarations, children, operators, shallow patterns and sorts without evaluating expressions. Existing owner preparation passes were inspected only to corroborate the already normative row/slice order; no preparer output or table-generation call supplies a cell here. Static OwnerWire clauses and public R112 declarations select representation only.

## ChainTotal: complete row derivation

Names are `[h,sum,t,xs]`, IDs0..3. Entry is Call(sum,one argument), not Literal7. Its argument retains two Binary(Cons) nodes with literals2/5 and literal empty List. Function body root6 is Match(xs,Nil arm0,Cons arm1). Nil arm body8 is literal0. Cons arm body9 is Add(Var(h),Call(sum,Var(t))). There are13 rows/occurrences and21 path components.

Calls0/11 append children `[1,12]`; their slices are0/1 and1/1. The sole function has name1, parameter slice0/1, return UInt and body6. Parameter is binder3/List. Arms are pattern0/body8 and pattern1/body9. Patterns are unit PNil then PCons{x0,y2}. Literal ExprIds2,4,5,8 produce constants UInt2,UInt5,Nil,UInt0 in that order. In particular, source text places the function's0 before entry, but normative owner expression order places its constant last. Every marker_free is true, because the UInts are in range and Nil has no fields. constant_fields is explicitly empty. No constructor runtime result is substituted into constants.

The fresh parameter environment is xs:List. The Nil arm has UInt0; the Cons arm adds h:UInt to the declared UInt return of sum(t:List), so both arm results and the function return are UInt. Entry Cons(2,Cons(5,Nil)) is List, and sum returns UInt. Independent mathematical recurrence gives2+(5+0)=7. This arithmetic result is not a B run/work/summary oracle.

## BranchTotal: complete row derivation

Names are `[fold,l,n,r,t]`, IDs0..4. Entry Call0 retains Binary Branch1, Unary Leaf2 over UInt4 at3, nested Binary Branch4, Unary Leaf5 over UInt1 at6, and Unary Leaf7 over UInt3 at8. Function Match root9 has scrutinee Var(t)10, Leaf arm Var(n)11 and Branch arm Add12 of Call(fold,l)13/14 and Call(fold,r)15/16. There are17 rows/occurrences and35 path components.

Calls0/13/15 append children `[1,14,16]` with slices0/1,1/1,2/1. The sole function is name0, params0/1, return UInt, body9; parameter is binder4/Tree. Arms are pattern0/body11 and pattern1/body12; patterns PLeaf{x2} and PBranch{x1,y3}. Constants are exactly UInt4,UInt1,UInt3, all marker-free, no constant fields. Leaf/Branch remain Unary/Binary expressions, so there are no fabricated pre-folded Tree or UInt8 constant rows.

The parameter t is Tree. Leaf binds n:UInt and returns UInt. Branch binds l/r:Tree; both recursive calls return declared UInt and their Add is UInt. Entry constructor operands have the required Tree/UInt sorts, and entry fold returns UInt. Independent mathematics is4+(1+3)=8. Type checking these declarations does not require or establish recursive execution.

## Complete provenance and type/owner joins

All spans are half-open byte ranges in the original ASCII files. The quoted language token is[9,36). Chain program name[46,56), function declaration[61,140), sum name[64,67), xs[68,70), List[72,76), UInt[79,83). Branch name[46,57), function[62,153), fold[65,69), t[70,71), Tree[73,77), UInt[80,84). Function spans include their outer closing brace; Match spans end at the inner match brace. Entry call spans end before the semicolon. Token span behavior includes string quotes, as the real public lexer convention specifies.

Every expression/pattern/binder/name/type span is explicitly transcribed and checked against the stated literal source substring, without running a lexer/parser. Pattern source sites preserve both arm order and each binder span. Source expressions, owner rows and type-accounting entries share exactly the following declared owner/path order; no pointer ID, fresh-symbol spelling or traversal wildcard remains:

### ChainTotal expression/type/provenance inventory

|ExprId|Owner|Path|Kind|Sort|Byte span|
|---:|---|---|---|---|---|
|0|entry|root|call|UInt|[149,173)|
|1|entry|0|cons|List|[153,172)|
|2|entry|0.0|uint|UInt|[158,159)|
|3|entry|0.1|cons|List|[160,171)|
|4|entry|0.1.0|uint|UInt|[165,166)|
|5|entry|0.1.1|nil|List|[167,170)|
|6|function 0|root|match|UInt|[90,136)|
|7|function 0|0|var|List|[96,98)|
|8|function 0|1|uint|UInt|[108,109)|
|9|function 0|2|add|UInt|[124,134)|
|10|function 0|2.0|var|UInt|[124,125)|
|11|function 0|2.1|call|UInt|[128,134)|
|12|function 0|2.1.0|var|List|[132,133)|

### BranchTotal expression/type/provenance inventory

|ExprId|Owner|Path|Kind|Sort|Byte span|
|---:|---|---|---|---|---|
|0|entry|root|call|UInt|[162,207)|
|1|entry|0|branch|Tree|[167,206)|
|2|entry|0.0|leaf|Tree|[174,181)|
|3|entry|0.0.0|uint|UInt|[179,180)|
|4|entry|0.1|branch|Tree|[182,205)|
|5|entry|0.1.0|leaf|Tree|[189,196)|
|6|entry|0.1.0.0|uint|UInt|[194,195)|
|7|entry|0.1.1|leaf|Tree|[197,204)|
|8|entry|0.1.1.0|uint|UInt|[202,203)|
|9|function 0|root|match|UInt|[91,149)|
|10|function 0|0|var|Tree|[97,98)|
|11|function 0|1|var|UInt|[112,113)|
|12|function 0|2|add|UInt|[130,147)|
|13|function 0|2.0|call|UInt|[130,137)|
|14|function 0|2.0.0|var|Tree|[135,136)|
|15|function 0|2.1|call|UInt|[140,147)|
|16|function 0|2.1.0|var|Tree|[145,146)|

## Validation, discriminator and limits

The static checker verifies all table references, child-preorder/path/owner relationships, exact12-field inventory, parameter/function/arm/pattern indices, names, constants and complete source/owner occurrence joins. `compare_expected.py` pins this exact EXPECTED.json and reproduces R112's selected comparison over source digest, full source/preparation and full ProgramView.44 pure synthetic checks demonstrate both complete golden matches and rejection of a changed selected field in every one of the twelve ProgramView fields, missing empty table, reordered provenance/children, changed source span/type/declaration/binder/digest and an entry rewritten as a literal shortcut. No actual subject output was supplied to these checks.

Selected static field gaps are0 under these pinned proposed contracts. Admission identity/bounds/layout/status, configuration/target/preprocessing extents, complete genesis, plans/evidence outside the interface, B events/work/images/runtime outcome, real CLI dispatch, security/cap decisions and native/formal/full compiler acceptance remain excluded or unobserved. The evidence does not assert the candidate already admits or runs either program. Root owns integration, any designated-owner classification, independent review and future actual comparisons. Underlying old freezes are preserved.
