# Expression source extension proposal 02

Status: frozen corrected proposal for two fresh independent decision reviews.
Proposal01 and both request-changes reviews are retained unchanged. This revision
addresses GPT-6 R1–R9 and the Grok lexical, metadata and diagnostic findings. Scope is local
source → typed Core → expression evaluation, consuming the reviewed all40 /1
runtime at base 36ca249. Full SP02, financial-operation transitions, K/proof
correspondence and SP01–SP12 completion remain open.

Repository observations: syntax/0 has nonnegative decimal, text and Boolean
literals; identifier reads/calls; dot projection; +, -, *, six comparisons,
not/and/or; all five action statements. It has no signed literals, numeric type
arguments, record/enum/option/collection construction, index projection, or
observation/schema/vault declarations. Its formatter covers that AST. The funded
elaborator rejects operators, state/const, guards, locals, writes and ensures.
The all40 runtime accepts canonical Core and trusted host-bound Σ, not source.

Design basis: deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md and
SYNTAX-COMPARISON.md select braced declarations, typed financial values, explicit
pre/next/post, named rounding and infix arithmetic. Model comparisons are not
an empirical developer study. This proposal extends that design instead of
making raw Core constructor names or host type aliases the source language.

## Decisions requiring two substantive independent agreeing votes

D1. Add explicit profile `moriarty-expression-source/1` through a separate parser
and evaluation entry point. syntax/0, atomic/1 and funded-source/0 APIs and accepted
bytes retain their meaning. Reuse the bounded lexer/parser with profile-gated
productions; old parser rejects the new header. No implicit profile upgrade.

D2. Extend generic type arguments to include canonical signed decimal integers.
Types spell their complete metadata, with no hidden per-program aliases:
`UInt64`, `UInt128`, `SInt128`, `Bool`, `Text`, `Amount<A>`, `Shares<V,H>`,
`Rate<2>`, `Price<A,B,2>`, `Quantity<Units<EUR,-1,USD,1>,2>`, `Record<R>`,
`Enum<E>`, `Operation<O>`, `Option<T>`, `Collection<T,128>`.
`Units` without arguments is the empty vector. Unit pairs must already be in
strict identifier order, nonzero exponent -128..127; no source normalization
silently repairs malformed metadata. Exact runtime type equality applies.
Bare unsigned expressions mean UInt128; `u64(1)`, `u128(1)`, `i128(-5)` are typed
literal forms. A negative decimal expression is a SInt128 literal, not a general
negation operator. `-0` rejects. There is no unary plus, float or implicit cast.

D3. Add generic intrinsic calls and typed record literals. Only the fixed generic
names `some`, `none`, `collection`, `quantity`, `record` may consume `<` after
an identifier, removing ambiguity with comparisons; these names are reserved
as term binders. Record fields and collection elements retain source order.
All literal metadata must be syntactic literals/identifiers, never evaluated
expressions. Signed mantissas admit unsigned or negative decimal token forms.

| Source | Exact Core lowering |
| --- | --- |
| 1; u64(1); u128(1) | LitUInt(128,1); LitUInt(64,1); LitUInt(128,1) |
| -5; i128(5) | LitSInt(-5); LitSInt(5) |
| true/false; "text" | LitBool; LitText |
| amount(5,A) | LitAmount(asset=A,value=5) |
| shares(5,V,H) | LitShares(vault=V,holder=H,value=5) |
| rate(-5,2) | LitRate(scale=2,mantissa=-5) |
| price(5,A,B,2) | LitPrice(base=A,quote=B,scale=2,mantissa=5) |
| quantity<Units<USD,1>,2>(-5) | LitQuantity(units=[[USD,1]],scale=2,mantissa=-5) |
| local; parameter; obs.name | ReadLocal; ReadArg; ReadObs |
| pre.field; post.field | ReadPre(pre,field); ReadPre(post,field) |
| value.field; value[index] | ProjectField; ProjectIndex |
| access_field(value,"field"); access_index(value,index) | AccessField; AccessIndex |
| record<R> { field: value, other: value } | ConstructRecord(R, lexical fields) |
| E.Member, where E is a registered enum name | ConstructEnum(E,Member) |
| some<T>(value); none<T>() | ConstructSome(T,value); ConstructNone(T) |
| collection<T,N>(values...) | ConstructCollection(T,N,lexical values) |
| +, -, *, floor_div(a,b), ceil_div(a,b) | Add, Sub, Mul, FloorDiv, CeilDiv |
| ==, <, <=, >, >= | Eq, Lt, Lte, Gt, Gte |
| != | Not(Eq(a,b)); both nodes use the comparison occurrence span |
| not, and, or | Not, And, Or, exactly once per operator |
| requires e; let x=e; next.f=e; ensures e | Require, Let, NextWrite, Ensure |
| emit O value; | Emit(O,core(value)), without a reconstructed record |
| emit O { f:e, ... }; | Emit(O,ConstructRecord(Σ.operations[O], lexical fields)) |

Bare variable reads resolve only lexical locals or declared parameters. Schema
fields and observations require explicit views. Enum type names cannot collide
with term binders or pre/post/obs. Unknown calls reject; there are no callbacks,
user functions, macros, recursion or source Core-deserialization escapes.
Access aliases exist only to cover both separately retained reviewed Core names;
they carry the same exact runtime work/error rules as projection syntax.

D4. Host binding remains the immutable reviewed Σ registry and determines all
financial write classes. A source action parameter list must exactly match Σ.args
in names and exact types. Its /1 parameter limit is256 (the total schema
declaration bound), while syntax/0 keeps64; source bytes/tokens and aggregate
schema bounds still apply simultaneously. This initial API admits exactly one action per source,
plus optional unit/party declarations (any subset, with no extras) validating membership in Σ and name-only
`asset A: Asset;` declarations validating membership in Σ.assets. `Asset<Unit>`
annotations reject SOURCE_DECLARATION_TYPE: the expression registry has no
asset/denomination relation and must not silently discard a supplied annotation. It rejects
const/state declarations: no initializer, genesis, constant caching or implicit
state-reset semantics are introduced by an expression bridge. Pre-state and
observations are supplied snapshots validated by the runtime. Every reviewed constructor has a direct source mapping within the explicitly
restricted source domain below, while the complete agreement declaration
language remains open and this limitation is reported explicitly. Exactly one
action is a scoped API limitation, not the final language design; SP02 retains
ownership of multiple actions, source-defined schemas, state/genesis and constant
semantics. No registry name is evidence of source-defined financial authority. The one
source action is selected intrinsically; evaluate accepts no action selector,
and there is no SOURCE_UNKNOWN_ACTION claim against Σ (which has no action
registry). Zero or multiple actions reject SOURCE_ACTION_COUNT.

D5. Source mapping and Core static errors are checked before typed snapshot
validation/reduction; structural envelope errors precede both. Parser errors keep
existing lexical/syntax codes. Elaboration errors use SOURCE_* codes and UTF-8
source spans (action count, unsupported declaration/call/type, intrinsic arity
and literal metadata shape, reserved/duplicate names, parameter/schema mismatch).
Runtime TYPE_*, INPUT_*, ARITH_*, INDEX_RANGE, GUARD_FAILED, ENSURES_FAILED,
VALUE_BOUND, DESCRIPTOR_BOUND and WORK_EXHAUSTED pass through unchanged with source spans and original Core paths.
No rejection includes staged state/descriptors. Enum resolution, builtin-name
reservation and type metadata validation occur before dynamic work. The trusted
schema cannot be supplied/reclassified by an evaluation request.

## Grammar delta (ISO/IEC 14977 EBNF)

The syntax/0 productions remain the baseline except these /1-only replacements.
The finite lexical, source-byte, AST and depth bounds continue to apply. Only
`collection<T,N>` has a /1 argument-list limit of128; every other call remains
bounded by64. The record-field limit remains64. The /1 parameter limit is256, versus64 in syntax/0. Core's stricter limits also apply
after elaboration. All40 means constructor and exact type fidelity within these
simultaneous source bounds, not surjectivity onto every Core-only metadata tree:
for example the source nesting bound64 cannot spell Core-only Option chains of
several thousand wrappers.

```ebnf
emission = "emit", identifier, ( "{", [ effectFields ], "}" | expression ), ";" ;
type = identifier, [ "<", typeArgument, { ",", typeArgument }, ">" ] ;
typeArgument = type | signedInteger ;
signedInteger = [ "-" ], integerToken ;
postfix = primary, { ".", identifier | "[", expression, "]" } ;
ordinaryPrimaryName = ? identifier token except some, none, collection, quantity, record ? ;
primary = integerToken | "-", integerToken | stringToken | "true" | "false"
        | ordinaryPrimaryName, [ "(", [ arguments ], ")" ]
        | genericCall | recordLiteral | "(", expression, ")" ;
genericCall = ("some" | "none" | "collection" | "quantity"),
              "<", typeArgument, { ",", typeArgument }, ">",
              "(", [ arguments ], ")" ;
recordLiteral = "record", "<", type, ">", "{", [ effectFields ], "}" ;
```

The fixed generic names are excluded from the ordinary identifier-primary arm
in /1. Metadata arity/type restrictions are static rules. No trailing commas.
`a < b` remains comparison; `some<UInt64>(u64(1))` is unambiguous.

## Exact source domain, roles and diagnostics

All baseline value/type domains apply unchanged: UInt64/UInt128/SInt128 endpoints,
Text1024 UTF-8 bytes, units at most8 and exponents -128..127 excluding0, scales
0..18, Price distinct assets and UInt128 mantissa, capacities0..128 and exact
capacity equality, acyclic finite schemas, and no Unit data values. No arithmetic
overload, financial interpretation, alias rewriting or implicit coercion is added.

The /1 lexical delta adds `[` and `]` punctuation only in the new parser. The old
lexer continues rejecting them. Existing nested generic closers and splitting
`>=` into `>` followed by `=` in a type context are preserved. Fixed generic names
require their generic syntax; ordinary identifiers retain comparison parsing.

| Intrinsic | Type arguments | Value-argument syntax / arity |
| --- | --- | --- |
| u64/u128/i128 | none | one signedInteger (negative unsigned values reach TYPE_LITERAL) |
| amount | none | unsigned integer, bare asset identifier; exactly2 |
| shares | none | unsigned integer, bare vault, bare holder; exactly3 |
| rate | none | signed integer, unsigned scale; exactly2 |
| price | none | unsigned mantissa, bare base, bare quote, unsigned scale; exactly4 |
| quantity | Units vector, numeric scale | one signedInteger |
| some | one exact finite type | one expression |
| none | one exact finite type | zero expressions |
| collection | one exact finite type, numeric capacity | 0..128 expressions |
| record | one bare record identifier | 0..64 named expression fields |
| floor_div/ceil_div | none | two expressions |
| access_field | none | expression, string literal decoding to a field identifier |
| access_index | none | two expressions; index type exactly UInt64 |

Unsigned syntax restrictions reject SOURCE_LITERAL_SHAPE before Core typing.
Admitted signed numeric syntax outside the chosen exact mathematical domain
reaches the unchanged TYPE_LITERAL check. Nested type syntax is rejected wherever
a bare nominal identifier is required. `record<Record<R>>` therefore rejects
SOURCE_TYPE_SHAPE; the admitted record form is `record<R>`.

Units is source-only metadata, never a value or general type. It is legal only
as the first argument of Quantity or quantity. Bare `Units` means empty; otherwise
its argument list has even length, alternating bare declared unit identifier and
signed decimal exponent. Odd arity, integer-first forms, nested types for a unit
symbol, and Units outside Quantity reject SOURCE_TYPE_SHAPE. Empty `Units<>`
retains parser EMPTY_TYPE_ARGS. Well-shaped but unsorted/duplicate/zero/out-of-range
vectors lower without normalization and reject TYPE_LITERAL when the Core type is
resolved. For an action parameter, any such annotation cannot equal its valid
trusted Σ.args entry and rejects SOURCE_PARAMETER_TYPE in correspondence checking.

Parentheses do not introduce AST nodes: `(pre).field`, `(obs).name` and `(E).Member`
have the same resolution as unparenthesized forms. Whitespace/comments between
minus and decimal are allowed: `- 5` is the same signed literal as `-5`; formatting
emits `-5`. Neither form creates general unary negation. `-0` rejects
SOURCE_LITERAL_SHAPE, and numeric type metadata `-0` rejects SOURCE_TYPE_SHAPE.
`u64(-1)` and `u128(-1)` reach TYPE_LITERAL with width unchanged; amount/shares/price
negative quantum syntax rejects SOURCE_LITERAL_SHAPE. `i128(1+2)` and evaluated
metadata names reject SOURCE_LITERAL_SHAPE. access_field requires a string literal
decoding to the reviewed ASCII identifier language, otherwise SOURCE_LITERAL_SHAPE.

AST additions are `Call.typeArguments` (present only on generic calls),
`RecordExpression {recordType,fields,span}`, `Index {object,index,span}` and an
optional `Emit.expression` alternative to braced fields. Signed decimal primaries
retain IntegerLiteral with their exact signed value string; numeric type arguments
retain Type nodes marked numeric, so no identifier value is mistaken for metadata.
The formatter must preserve these distinctions and Core structure after spans are
erased; it cannot turn `i128(5)` into bare UInt128 `5`. Old ASTs have none of these
new optional properties. No old-profile accepted source acquires a new AST node.

The source domain explicitly excludes trusted schemas containing identifiers
that cannot be spelled by this frontend; it does not silently rename them.
This is a source-domain restriction, not a new Core schema rule. Full eventual
SP02 owns source identity escaping and richer declaration scope if required.

| Name category | Reserved names / rule |
| --- | --- |
| Lexical keywords | profile agreement unit party asset const state action requires let next emit ensures true false not and or; never bare identifiers |
| Fixed generic primaries | some none collection quantity record; never ordinary term or nominal identifiers in /1 |
| All schema metadata identifiers | must be ASCII identifiers admitted by the lexer and not lexical keywords or fixed generic primaries; otherwise SOURCE_SCHEMA_NAME |
| Enum names | additionally cannot be pre/post/obs or collide with args, fields, observations, record names, operations or roster identities; otherwise SOURCE_SCHEMA_NAME |
| Type constructor names | UInt64 UInt128 SInt128 Bool Text Amount Shares Rate Price Quantity Units Record Enum Operation Option Collection Unit Asset; reserved as term binders, meaningful only in their specified type roles |
| Parameter/local binders | cannot be any intrinsic or type constructor name, pre/post/obs/next, an enum name, any declared registry name, or a previous local; a parameter's own Σ.args entry is exempt from its own collision check |
| Metadata field/member names | ordinary intrinsic names such as amount remain permitted; e.g. quote.amount, record<Quote>{amount:e} |
| Optional source declarations/action name | names must be distinct from each other; action name must not be reserved; unit/party/asset may only reaffirm their matching roster entry |

Cross-category registry names other than enum/term ambiguities remain separate
namespaces: an asset and record can share a spellable name because the syntactic
role determines which registry is selected. Bare reads select an argument when
that name is a declared parameter, otherwise a lexical local (unknown names become
TYPE_NAME). No implicit field/observation fallback occurs. `pre.f`, `post.f` and
`obs.f` are recognized before ordinary record projection; enum-qualified members
are recognized next; ordinary record projection is last. The source schema domain
rejects ambiguous enum prefixes. Local/parameter collision checks preserve the
runtime's TYPE_DUPLICATE_BINDER constraint and do not shadow declarations.

`createExpressionSourceV1(schemaCanonicalJSON)` binds immutable trusted schema
text once. Its APIs are `elaborate(source)`, `check(source)` and
`evaluate(source,snapshotCanonicalJSON)`. The last JSON text is a closed object
with exactly Pre, Args, Obs and workInitial. There is no caller Core/AST/schema
argument, no source evaluator callback and no live-object input. Each operation
parses fresh owned JSON trees from primitive strings. Returned Core/results are
fresh data; neither caller mutation nor a previous returned value can change a
later invocation. No mutable cache or alias registry is introduced.

Failure phases, in this exact order:

1. Validate primitive string transport types and finite byte envelopes. For
   evaluate, admit canonical JSON snapshot-envelope structure and workInitial
   decimal/range0..65536, including each snapshot's raw canonical byte ceiling.
   These are structural checks, not typed snapshot validation. Check/elaborate
   have no snapshot input and claim no snapshot admission.
2. Lex/parse the complete source with its exact profile, source/list/token/depth
   bounds and UTF-8 scalar rules; existing syntax codes retain their byte spans.
3. Admit the immutable Σ structure and semantic domain using the reviewed schema
   checker, then validate the explicit source schema/name domain. Check source
   action count, optional declarations and parameter correspondence.
4. Elaborate source nodes in lexical preorder, validating fixed intrinsic arities,
   syntactic metadata roles and type annotation shapes. SOURCE_* failures precede
   Core typing failures; there is no claim of a single lexical ordering across
   these distinct phases. No Core reduction occurs in elaboration.
5. Run the existing whole-Core structural/schema/static checker, including all
   unexecuted operands and statements. Bindings and writes use its exact rules.
6. For evaluate only, validate typed Pre, Args and Obs, in that order, using the
   existing shape, typed bounds and mathematical domain phases.
7. Reduce with the unchanged work meter. Check/elaborate never enter this phase.

The SOURCE_* diagnostic catalog is closed for this version:

| Code | Phase / condition / source span |
| --- | --- |
| SOURCE_ACTION_COUNT | 3; action count differs from1; agreement span |
| SOURCE_DECLARATION | 3; const/state or other unsupported declaration; declaration span |
| SOURCE_DECLARATION_TYPE | 3; asset type is not bare Asset; type span |
| SOURCE_DECLARATION_NAME | 3; optional unit/party/asset absent from matching Σ roster; declaration span |
| SOURCE_DUPLICATE_DECLARATION | 3; second source declaration with an existing source name; second declaration span |
| SOURCE_SCHEMA_NAME | 3; trusted registry name violates the declared source domain/collision rule; synthetic[0,0), because Σ is not source |
| SOURCE_RESERVED_NAME | 3 for action/parameter, 4 for local; reserved binder; binder's containing source node span |
| SOURCE_PARAMETER_NAMES | 3; duplicate source parameter or parameter-name set differs from Σ.args; duplicate parameter span or action span |
| SOURCE_PARAMETER_TYPE | 3; source parameter exact annotation differs from its trusted type; annotation span |
| SOURCE_CALL | 4; unknown non-generic call; call span |
| SOURCE_ARITY | 4; fixed intrinsic value/type argument count mismatch; call or record literal span |
| SOURCE_LITERAL_SHAPE | 4; metadata is not the required literal/identifier or negative-zero value; offending operand span |
| SOURCE_TYPE_SHAPE | 3 for parameter types, 4 otherwise; unknown type form/arity, malformed Units pairing, non-bare nominal index or negative-zero metadata; offending type span |
| SOURCE_OPERATION | 4; operation name absent from Σ or type arguments supplied; emission span |

Unknown generic names are ordinary comparisons/calls under the explicit grammar,
so invalid generic-looking source can fail existing parser syntax codes instead
of SOURCE_CALL. Keyword binders fail parsing before reservation checks. `next.f`
in an expression is a syntax error (next is a keyword), and remains forbidden;
no ReadNext or valid next read is introduced. Malformed [] in /0 is the unchanged
UNEXPECTED_CHAR. All non-SOURCE errors retain their phase/code from the parser or
reviewed wire/schema/Core helpers. Schema defects have synthetic spans; source
parameter annotation differences have the table's source span. No extra SOURCE_*
code may be silently added without updating this versioned source contract.

All failures before phase7 have workUsed0 and expose no post/descriptors. Syntax
and SOURCE_* diagnostics have an empty Core nodePath because no checked Core
occurrence has been admitted. Metadata role/type-shape errors use the offending
argument/type annotation span; action/declaration errors use that source node.
The entire emitted record sugar shares its emission span for both Emit and its
inserted ConstructRecord; direct `emit O e` gives Emit the statement span and e
its own span. Both inserted Not/Eq nodes for != use the comparison span. Other
nodes retain their complete original expression/statement span. ConstructRecord
duplicate-field and missing-field failures use the record constructor span as
in the runtime; action-finalization aggregate failures use the action span.
Runtime paths count lowered Core children, never metadata or source punctuation.
No synthetic source offsets are fabricated. Source spans are [start,end) UTF-8
bytes and remain bounded by the retained source bytes.

Conservative work B is the count of constructor occurrences in the lowered Core
tree: both Boolean branches, Not plus Eq for !=, and Emit plus ConstructRecord
for record sugar. Action wrappers and type/identifier metadata are not nodes.
The checker reports B and rejects any static ceiling violation using the existing
Core AST limit4096 and registered expression work ceiling65536. Request workInitial
may be less than B: unselected branches require no dynamic work, and selected
exhaustion retains the runtime's exact location/charge. No request is required to
fund dead branches, and no source optimization silently changes B or work.

## Representative real source

Trusted Σ declares ordinary `counter:UInt64`, financial `funds:Amount<Cash>`,
observation `ready:Bool`, record `Quote={amount:Amount<Cash>,count:UInt64}`,
enum `Mode={Open,Closed}`, operation `QuoteNotice=Quote`, unit USD, asset Cash.
Its action arguments are `delta:UInt64`. No financial operation is executed.

```text
profile "moriarty-expression-source/1";
agreement QuoteCounter {
  action step(delta: UInt64) {
    requires obs.ready and delta > u64(0);
    let rounded = floor_div(i128(-5), i128(2));
    let quote = record<Quote> { amount: amount(5, Cash), count: delta };
    let choices = collection<UInt64,2>(u64(7), delta);
    let optional = some<Amount<Cash>>(quote.amount);
    let empty = none<Amount<Cash>>();
    let mode = Mode.Open;
    next.counter = pre.counter + choices[u64(1)];
    emit QuoteNotice { amount: quote.amount, count: delta };
    ensures post.counter == pre.counter + delta;
  }
}
```

Result label stays ExpressionPrepared, explicitly local ordinary state plus
uninterpreted typed operation descriptors. No Transfer/Repay implementation is
inferred; direct financial writes fail TYPE_FINANCIAL_WRITE. Existing funded
preparation remains the separate, unchanged route.

## Implementation and verification plan

1. Add the profile-gated AST/parser/formatter extension; preserve all old tests
and add old-profile rejection and generic/comparison ambiguity tests.
2. Add source elaboration and trusted-schema factory; use runtime static checking
without executing during checking, then evaluate only the checked source action.
Keep full source spans and enforce simultaneous source/Core bounds.
3. Test real .mori for all40 mappings, complete typed values, wrong types in dead
Boolean branches, static-error-before-guard, immutable pre/post, duplicate writes,
forbidden next reads, financial write rejection, descriptor/write rollback,
Unicode spans, selected/unselected failures, work exhaustion, host-input ownership
and formatter round trips. Hand-authored Core-only tests cannot establish this.
4. Freeze exact candidate bytes, run TypeScript build and package tests, retain
raw outputs and obtain separate fresh GPT-6 Astra and Grok 4.6 result audits.
Author implementation is not its own approval. No network/proof campaign runs.
