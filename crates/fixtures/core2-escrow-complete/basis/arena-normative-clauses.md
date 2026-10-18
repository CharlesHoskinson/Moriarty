# M01: finite Core/2 source-to-arena contract proposal

Status: **UNADOPTED, implementation-source-exposed ordinary design candidate**, 2026-10-18. Assigned author GPT-6.1 Sol HIGH. No parser/evaluator was executed and no generated or expected Program was authored. No independent vote, behavioral proof or canonical program/plan identity is claimed. The goal is a finite representation premise from which a separate independent custodian can later derive the original escrow's full Program before an actual run.

The exact additive patch is core2-arena.patch. Its base is the VIRTUAL AFTER-STATE of the unchanged frozen core2-report-contract-proposal/core2-report.patch, not the current repository files. The two-stage patch chain was reconstructed only in memory. The existing report-only proposal, its all14-gap table and its exact files remain unchanged; the new M01 section is a separate supplement to its deliberate omission of arena rules. SOURCE-IDENTITY.json pins both patch-chain stages and source files. No patch was applied to the checkout.

## Decisions for two fresh independent votes

Review the complete additions below and return ADOPT / REVISE / ABSTAIN with substantive reasoning for each choice: (1) category-filtered lexical declaration arrays/all actions; (2) initializer, metadata, input and binder exclusion from the arena; (3) one global fresh-occurrence left-first postorder arena without interning or folding; (4) exact statement roots and Await guard timing independent of runtime; (5) scoped expression grammar/precedence and explicit treatment of the undocumented inequality spelling. Missing or unavailable reviewers provide no vote. No reviewer was invoked by this lane. Identity/security topics are not part of these votes.

The corrected precedent is precise: PR203's source/DTO-derived derivation and two exact fixtures supply finite comparison premises. They do not adopt a general Core/1 grammar or frontend normalization, and do not establish Core/2 equivalence. This candidate makes Core/2's own rule explicit rather than silently importing that precedent. Existing adoption records and actual results are not reclassified here.

## Complete added documentation section

Append after the frozen report-only proposed after-state in docs/CORE-STAGED-TESTS.md. The original M01 omission/disposition remains part of that earlier proposal; this separately marked supplement offers the missing ordinary choice without claiming adoption or closure.

## Proposed M01 arena supplement (unadopted)

This is a separate ordinary representation proposal supplementing the preceding frozen report-only candidate, which deliberately left M01 unselected. Its SHALL clauses are UNADOPTED and implementation-source-exposed. They describe the complete public Program representation of the finite documented positive source forms below; they grant no admission or execution authority. The report-only candidate's M01 disclaimers and all fourteen individual OPEN gap rows remain unchanged. This supplement supplies a prospective representation choice, not an adopted premise, parser result, independent Program golden or completed GAP-05/M01 exit.

The two Core/1 full-Program fixtures from PR203 are finite precedent for source-order arrays, initializer placement and fresh expression occurrences. Their post-adoption derivation does not establish a general Core/1 grammar, general normalization or Core/1/Core/2 equivalence. Core/2 adopts nothing by inheritance here: each choice below is independently proposed and needs two substantive independent design votes and a recorded adoption before new independent expectations are frozen.

### Whole Program and declaration order

For already admitted source beginning language "moriarty/2", Program SHALL contain exactly language:"moriarty/2", semantics:"moriarty-core/2", name:the agreement identifier, states, assets, expressions and actions. No source hash, selected-action field, Scenario, capture, plan, registry, work counter or runtime result SHALL be inserted. Type SHALL serialize as "UInt" or "Bool". A typed UInt literal SHALL use {type:"UInt",value:its canonical unsigned decimal string}; a Bool literal SHALL use {type:"Bool",value:true or false}. Literal decoding follows the existing admitted numeric/string syntax; no decimal scaling, evaluation or new lexical normalization is selected.

StateDecl SHALL retain exactly name, ty and initial, with the literal initial stored directly as a typed Value. State declarations SHALL occur in Program.states in their own lexical declaration order. AssetDecl SHALL retain exactly name and identity; identity SHALL contain network, issuer and reference as decoded strings and scale as its numeric u8 value. Asset declarations SHALL occur in Program.assets in their own lexical declaration order. Action SHALL retain exactly name, inputs and statements; Program.actions SHALL retain every action, including unused actions, in lexical declaration order. Interleaving declaration kinds does not introduce a combined declaration-order field; each array is the source sequence filtered to its own kind. These representation rules do not authorize forward references or alter existing declaration admission.

A state's initializer, asset scale/identity, agreement/profile/action identifiers, input and binder declarations and statement metadata SHALL allocate no expression nodes. They SHALL NOT be converted into action expressions or synthetic statements. Action.inputs SHALL contain exactly {name,ty} entries: signature inputs in order, followed by any existing admitted leading input declarations in body order. Those leading declarations SHALL NOT count as statements. Await.binders SHALL retain exactly the written {name,ty} entries in binder declaration order, not action inputs. Runtime inputs, event values and captures SHALL NOT be substituted for declarations or expressions in Program.

### One fresh global expression arena

Program.expressions SHALL start empty and be shared across all actions in their lexical declaration order. Within each action, process statements in their written order and allocate only the statement's expression tree as below. Do not reset or rebase the arena at an action, statement, await, segment or selected-action boundary. Retain all nodes from unused actions. An otherwise empty selected action does not imply an empty whole Program arena.

For each source expression occurrence, allocate fresh nodes in left-to-right postorder. To append a node, set its ExprId to the current global array length, append exactly that node, then return that numeric zero-based index. Existing expression bounds and u16 representation remain unchanged. Do not intern repeated names, repeated literal values or structurally equal subexpressions, within or across statements/actions. No constant folding, algebraic reassociation, common-subexpression sharing, inserted coercion or desugared helper node is selected for this source-to-Program representation. Array position SHALL be the ExprId; no extra id member SHALL be inserted in a node.

| Source form | Exact appended node and allocation order |
| --- | --- |
| UInt or Bool literal in an action expression | One fresh Literal {value:typed decoded literal} |
| Ordinary name occurrence | One fresh Name {name:written identifier}; no substitution with a declaration, reference index or runtime value |
| Parenthesized expression | Its inner allocation/root only; no grouping node |
| Prefix not expression | Allocate its operand first, then one Not {value:operand ExprId} |
| Binary expression | Allocate the complete left subtree, then the complete right subtree, then one Binary {op:the selected operator, lhs:left ExprId, rhs:right ExprId} |
| liability_present("literal duty name") | One LiabilityPresent {name:decoded literal duty name}; quoted argument is metadata, not a Literal child |
| liability_amount("literal duty name") | One LiabilityAmount {name:decoded literal duty name}; quoted argument is metadata, not a Literal child |

Every brace form denotes exactly the listed fields plus kind carrying the variant name. Child fields are numeric ExprIds into this Program; they are not typed semantic values or source offsets. For this fresh-postorder source mapping, a child's index is lower than its parent's. Each repeated textual occurrence still allocates its own subtree. No rule here changes the separate admission of manually supplied DAGs or other profiles.

The finite documented expression grammar uses atoms above, parentheses, strongest prefix not, then decreasing binary precedence: * (Mul); + and - (Add/Sub); ==, <, <=, >, >= (Eq/Lt/Le/Gt/Ge); and (And); or (Or). Operators at one binary precedence are left-associative; explicit parentheses determine a different tree before postorder allocation. Minus is binary here; no unary-negative, division, generic function call, branching or implicit decimal node is introduced. This is a proposed finite representation grammar for already admitted forms, not a general frontend normalization claim.

The public DTO also contains BinaryOp::Ne. Current parser source recognizes it through token pair not followed by =, while the public README does not document that spelling and the shared lexer has no exclamation punctuation. This supplement SHALL NOT canonize that parser-specific spelling, invent != support or silently repair the language. Source inequality spelling/admission remains a separate ordinary grammar decision; if the intended spelling is !=, corresponding lexer/parser source repair and independent checks would be required. Such forms are outside this finite grammar's correspondence claim, without retiring any broader language obligation. Their potential future node shape remains Binary with op:"Ne" once source syntax is independently specified. No implementation mismatch is established for the chosen documented forms by this source-only review; conformance still needs real checks.

### Exact statement boundaries, metadata and roots

Each source statement SHALL contribute one Statement in Action.statements in written order, after its expression root is allocated. Statement boundaries SHALL not allocate nodes, reset numbering, append a unit/success result, or insert a statement. A semicolon ends the existing statement form; braces and leading input declarations are not Statement entries. Each expression-bearing field SHALL retain its numeric root ExprId, even where the expression's evaluated amount or boolean happens to have the same numeric spelling.

| Source statement form | Exact public Statement fields in addition to kind | Expression root |
| --- | --- | --- |
| let name = e; | Let: name, value | value is root(e) |
| requires e; | Requires: predicate | predicate is root(e) |
| ensures e; | Ensures: predicate | predicate is root(e) |
| next state = e; | Next: state, value | value is root(e) |
| emit "event name"(e); | Emit: name, value | name is decoded metadata; value is root(e) |
| transfer(asset,"from","to",e); | Transfer: asset, from, to, amount | asset is the alias, parties decoded metadata; amount is root(e) |
| fee(asset,"from","to",e); | Fee: asset, from, to, amount | same metadata roles; amount is root(e) |
| liability("name",asset,"debtor","creditor",e); | Liability: name, asset, debtor, creditor, amount | duty/parties decoded metadata and asset alias; amount is root(e) |
| discharge("name",e); | Discharge: name, amount | duty name decoded metadata; amount is root(e) |
| await label(name:Type,...) when e; | Await: label, binders, guard | label/binder declarations allocate nothing; guard is root(e) |

Ordinary expressions in an Await guard SHALL follow the same fresh allocation rule at that await's lexical statement position. A guard reference to one of its event binders SHALL allocate a Name occurrence; declaring that binder allocates none. Guard allocation SHALL not wait for runtime resume, and SHALL not depend on the event's value or whether a later invocation waits, advances or rejects. Await.guard's ExprId SHALL remain distinct in meaning from the action statement ordinal, plan site number and report Step.index. Name binding/type and financial/await execution semantics remain separate existing obligations.

No selected-action filtering, added declaration for admission, hidden arena node or runtime trace normalization SHALL alter this whole-source Program mapping. Retain unused declarations exactly when actually present in the source. No Mint, Burn or dedicated accrual construct is introduced, and broader missing financial obligations remain. Existing report-only site/capture, trace, envelope, rendering and conditional-work proposals remain separate unadopted choices.

### M01 boundary and independent freeze

This supplement proposes an ordinary representation premise sufficient to derive a complete Program for the original finite escrow source from its actual declarations and expression occurrences. It supplies no such Program fixture, numeric arena specimen, parser output or expected execution. M01 and GAP-05 remain OPEN until the exact chosen premises are independently adopted, a distinct custodian derives/freezes the complete Program without using this source-exposed author or parser output as its oracle, and authorized real full-Program checks are observed. Recorded public-premise adoption may enable that independent pre-run derivation while the source-correspondence exit remains OPEN awaiting real checks. The original partial escrow freeze, MISSING-PREMISES.json, derivation, all fourteen individual gap dispositions, R7 contradiction, original report candidate, freezes/failures and counters SHALL remain unchanged.

Exact program/plan digest preimages, algorithms, encodings and identity values, authenticated inputs/history, commitment/soundness/security conditions remain designated-owner UNASSESSED. Source-file and proposal-byte digests identify inspected artifacts only, not canonical Program/Plan identity. This supplement supplies no admission, decoder/replay, proof, target or runtime result and closes no U0–U7, M1–M17, MC/SP/G, financial, composition or partial/conditional parent exit.

## Complete added OpenSpec child paragraph

Insert under existing UNI-018 after the frozen report-only Core/2 paragraph, before UNI-019. No existing line or top-level ID is changed.

**Proposed M01 Core/2 arena supplement (unadopted).** The [separate M01 supplement](../../../../../docs/CORE-STAGED-TESTS.md#proposed-m01-arena-supplement-unadopted) independently proposes complete Program representation for its finite documented positive grammar: per-kind declarations/actions in lexical order, signature then leading-body inputs, literal initializers/asset metadata/binders outside the expression arena, and one global fresh-occurrence arena across all statements and actions. Child expressions SHALL allocate left-to-right before their parent, without interning/folding or selected-action filtering; statement expression fields, including Await.guard, SHALL retain their numeric roots. Metadata and statement boundaries SHALL allocate no nodes. The two Core/1 PR203 Program fixtures are finite precedent, not general grammar authority or Core/2 equivalence. The undocumented Ne source spelling is not canonized; any intended alternative needs a separate ordinary grammar/source repair decision. M01/GAP-05 and all fourteen gaps remain OPEN pending independent design votes, recorded adoption, distinct independent freeze and authorized real checks. Exact Program/Plan identities/digest preimages and all authentication/history/commitment/soundness/security decisions remain designated-owner UNASSESSED; original freezes and the report-only candidate retain their scope unchanged.

