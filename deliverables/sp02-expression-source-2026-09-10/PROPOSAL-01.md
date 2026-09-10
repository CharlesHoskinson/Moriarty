# Expression source extension proposal 01

Status: proposed decisions; no new surface choice is accepted yet. Scope is local
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
`Rate<2>`, `Price<A,B,2>`, `Quantity<Units<USD,1,EUR,-1>,2>`, `Record<R>`,
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
| emit O { f:e, ... }; | Emit(O,ConstructRecord(Σ.operations[O], lexical fields)) |

Bare variable reads resolve only lexical locals or declared parameters. Schema
fields and observations require explicit views. Enum type names cannot collide
with term binders or pre/post/obs. Unknown calls reject; there are no callbacks,
user functions, macros, recursion or source Core-deserialization escapes.
Access aliases exist only to cover both separately retained reviewed Core names;
they carry the same exact runtime work/error rules as projection syntax.

D4. Host binding remains the immutable reviewed Σ registry and determines all
financial write classes. A source action parameter list must exactly match Σ.args
in names and exact types. This initial API admits exactly one action per source,
plus optional unit/party/asset declarations consistent with Σ. It rejects
const/state declarations: no initializer, genesis, constant caching or implicit
state-reset semantics are introduced by an expression bridge. Pre-state and
observations are supplied snapshots validated by the runtime. Therefore every
reviewed expression is expressible, while the complete agreement declaration
language remains open and this limitation is reported explicitly.

D5. All static errors are checked before snapshots/reduction. Parser errors keep
existing lexical/syntax codes. Elaboration errors use SOURCE_* codes and UTF-8
source spans (unknown action, unsupported declaration/call/type, intrinsic arity
and literal metadata shape, reserved/duplicate names, parameter/schema mismatch).
Runtime TYPE_*, INPUT_*, ARITH_*, INDEX_RANGE, GUARD_FAILED, ENSURES_FAILED and
WORK_EXHAUSTED pass through unchanged with source spans and original Core paths.
No rejection includes staged state/descriptors. Enum resolution, builtin-name
reservation and type metadata validation occur before dynamic work. The trusted
schema cannot be supplied/reclassified by an evaluation request.

## Grammar delta (ISO/IEC 14977 EBNF)

The syntax/0 productions remain the baseline except these /1-only replacements.
The finite lexical, source-byte, AST, list and depth bounds continue to apply;
Core's stricter limits also apply after elaboration.

```ebnf
type = identifier, [ "<", typeArgument, { ",", typeArgument }, ">" ] ;
typeArgument = type | signedInteger ;
signedInteger = [ "-" ], integerToken ;
postfix = primary, { ".", identifier | "[", expression, "]" } ;
primary = integerToken | "-", integerToken | stringToken | "true" | "false"
        | identifier, [ "(", [ arguments ], ")" ]
        | genericCall | recordLiteral | "(", expression, ")" ;
genericCall = ("some" | "none" | "collection" | "quantity"),
              "<", typeArgument, { ",", typeArgument }, ">",
              "(", [ arguments ], ")" ;
recordLiteral = "record", "<", type, ">", "{", [ effectFields ], "}" ;
```

The fixed generic names are excluded from the ordinary identifier-primary arm
in /1. Metadata arity/type restrictions are static rules. No trailing commas.
`a < b` remains comparison; `some<UInt64>(u64(1))` is unambiguous.

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
