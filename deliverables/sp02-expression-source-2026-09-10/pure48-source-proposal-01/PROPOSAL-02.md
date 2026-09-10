# Pure48 source extension — proposal01

Status: proposed source choices; no implementation or new source tests run.
Author: GPT-6 source implementation agent. Requires two agreeing substantive
independent source-design votes, then fresh exact implementation/result audits.
The existing source candidate and its review inputs remain frozen.

## Inspected baseline and scope

Repository observation: published runtime commit
`902ef61958b653f305024fe5933baadfb5885017` provides
`src/successor/financial-expression-{v1,types-v1}.ts` under
`experiments/moriarty-language/`. Its exact runtime contract is
`moriarty-financial-expression-contract/1`: the original40 constructors, eight
additions, UInt256, Variant and five indexed numeric intermediate types. The
preserved design inputs under
`deliverables/sp02-financial-pure-expression-2026-09-10/inputs/` retain their
historical proposed-only labels; publication and runtime inspection establish
the current implementation observation, not retroactive edits to those inputs.

Source baseline is commit `894caefe5359d141889696e0da0be0283b0ebce2`, manifest
SHA256 `acf5c2c872a86b4ca3551f951e245cf5d0cb4e08fa78dfb41cc3f213340f393a`.
The source40 candidate is awaiting independent result verdicts. This proposal
does not accept that candidate or alter its bytes.

Proposed capability: developers author all48 expression constructors in real
`.mori`, with exact indexed types, check/format/elaborate/evaluate APIs, spans and
work. Pure values and emitted descriptors remain local descriptions; this API
does not execute financial operations, funded preparation or ledger effects.

## Decisions for the next pair of source-design votes

- P1: select the explicit new header `profile "moriarty-financial-expression-source/1";`
  and API `createFinancialExpressionSourceV1(schemaCanonicalJSON)`. Preserve
  syntax/0 and expression-source/1 entry points, meaning and work. Never infer
  a runtime from syntax or upgrade a profile automatically.
- P2: retain existing literal functions and add generic dynamic amount/share
  constructors, `u256(integer)`, `to_uint<W>(expression)`, direct indexed types,
  and the exact existing runtime arithmetic overloads described below.
- P3: use `variant<Family, Tag>(payload)` and `project_variant<Tag>(value)`;
  preserve existing Option introduction and add `some_value(value)`. The comma
  spelling is an explicit source decision replacing the earlier illustrative
  `variant<Family.Tag>` sketch. It reuses existing type-argument syntax and needs
  no qualified-type AST or general member/function dispatch.
- P4: add lazy `condition ? consequent : alternative` with the exact precedence,
  static checks, spans and work rules below; no eager function substitute.
- P5: retain the closed diagnostics, transport, ownership and bounds of source/1,
  with the financial schema/type/name domain and check API adaptation below.

Alternatives considered: extending expression-source/1 in place would silently
change its admitted types/overloads and violate its profile promise. Exposing Core
constructors or a host type-alias registry would avoid parser work but prevent
ordinary self-describing source authoring. The separate profile with a small
grammar delta is recommended. Dotted variant metadata is readable but adds a
second nominal-metadata grammar without a required expressive gain.

## Exact types and scalar introductions

Add source type spellings with exact array lowering (N is a canonical signed
integer type argument; identities are bare identifier type arguments):

| Source | Exact type |
| --- | --- |
| `UInt256` | `["UInt256"]` |
| `Variant<F>` | `["Variant",F]` |
| `AmountProduct<A,B>` | `["AmountProduct",A,B]` |
| `ScaledAmount<A,N>` | `["ScaledAmount",A,N]` |
| `SignedScaledAmount<A,N>` | `["SignedScaledAmount",A,N]` |
| `SignedAmount<A>` | `["SignedAmount",A]` |
| `NetAmount<A>` | `["NetAmount",A]` |

All old type spellings remain. Option/Collection may recursively contain these
types. There are no aliases supplied per program. A/B must be declared assets;
AmountProduct requires A <= B in identifier order (equal indices denote A²).
Scale N is0..18. Never silently sort an explicitly authored type; the runtime's
Mul result sorts its two input asset indices as its established rule. Variant F
resolves in trusted `variantTypes`; tags have exact distinct payload types, with
1..128 cases and no recursive record/variant/operation type cycles. These and
all existing schema/value bounds remain the runtime's rules.

`u256(123)` lowers to LitUInt(width="256",value="123") with one node.
Like u64/u128/i128, its operand must be a source IntegerLiteral, not an expression
that happens to evaluate to an integer. Parentheses remain AST-transparent.
Bare positive integers still mean UInt128; bare negative integers still mean
SInt128. No new signed256 literal or dynamic Rate/Price/Quantity constructor is
invented: none exists in the48 runtime. The new indexed intermediate types enter
through typed args/pre/obs, existing arithmetic and Variant/Option/record values.

`amount(5, USD)` and `shares(5, Vault, Alice)` retain literal lowering and one
node. `amount<USD>(5)` and `shares<Vault,Alice>(5)` always lower to dynamic
construction over LitUInt128, two nodes, even for a literal argument. The generic
form never folds into LitAmount/LitShares. `amount(q,USD)` remains a literal-shape
error when q is an identifier. `u256(q)` is not a width conversion.

## Eight exact source-to-Core mappings

Every row has one own node charge and half-open UTF-8 source span covering the
whole call/conditional; recursively lowered children retain their real spans.
Metadata is not an evaluated child and carries no node charge.

| Constructor | Source | Exact operands and result |
| --- | --- | --- |
| ConstructAmount | `amount<A>(e)` | asset=A, value=lower(e); e:UInt128 → Amount<A> |
| ConstructShares | `shares<V,H>(e)` | vault=V, holder=H, value=lower(e); e:UInt128 → Shares<V,H> |
| ConstructVariant | `variant<F,T>(e)` | family=F, tag=T, value=lower(e); e:Σ.variantTypes[F][T] → Variant<F> |
| ProjectVariant | `project_variant<T>(e)` | tag=T, value=lower(e); e:Variant<F> → Σ.variantTypes[F][T] |
| ProjectSome | `some_value(e)` | value=lower(e); e:Option<T> → T |
| ConvertUInt | `to_uint<W>(e)` | width=W, value=lower(e); e:UInt64/128/256 → UIntW; W exactly64/128/256 |
| ScalarValue | `quanta(e)`, `mantissa(e)`, `is_negative(e)`, `magnitude(e)` | component respectively quanta/mantissa/negative/magnitude, value=lower(e); exact table below |
| Select | `c ? t : f` | condition=lower(c), consequent=lower(t), alternative=lower(f); c:Bool, arms exactly same type T |

Variant metadata is resolved in the roles shown. Family/tag cannot be an evaluated
expression, numeric argument or nested type. Projection infers the family from
the child's exact type; a valid different active tag rejects VARIANT_CASE, an
undeclared tag rejects TYPE_NAME statically. No enum-prefix interpretation is
used for variant construction. There is no tag-test/refinement or match facility
in the48 runtime; source must not imply one. A known tag can come from an existing
explicit rule/condition; unsafe projection remains an explicit partial operation.

`some<T>(e)`, `none<T>()`, and equality retain /1 behavior. `some_value(none<T>())`
rejects OPTION_NONE dynamically. A guard such as
`x != none<UInt128>() and some_value(x) > 0` may skip the projection, but the whole
right operand still typechecks and counts toward B.

ConvertUInt accepts only unsigned scalar widths. Both widening and narrowing
remain nodes, including a same-width conversion; narrowing checks the exact value
and rejects ARITH_RANGE. It never erases an asset/holder/scale, rescales a number,
accepts signed values or changes a variant case. Unit erasure must be explicit:
`to_uint<256>(quanta(a))` is valid for a:Amount<A>;
`to_uint<256>(a)` rejects TYPE_MISMATCH.

| Scalar source / child | Result |
| --- | --- |
| quanta / Amount<A>, Shares<V,H> | UInt128 |
| mantissa / Price<A,B,S> | UInt128 |
| mantissa / Rate<S>, Quantity<U,S> | SInt128 |
| is_negative / Rate, Quantity, SignedAmount, NetAmount | Bool |
| magnitude / Rate, Quantity, NetAmount | UInt128 |
| magnitude / SignedAmount | UInt256 |

No other overload is admitted, including SignedScaledAmount projection. These
functions expose raw quanta/mantissa/sign/magnitude, not a different asset or
spendable entitlement. Rebuilding a pure Amount after projection grants no
availability, funding, minting, transfer or signed-debit authority.

## Grammar, names, AST and formatting delta

In the new profile only, lex `?` as punctuation. `:` is already punctuation.
Keep source65536 bytes, tokens8192, AST8192/depth64, declarations/statements256,
record fields64, collection arguments128, ordinary arguments64, parameters256.
No new bound or optimistic-work admission is proposed.

Replace the expression entry by:

```ebnf
expression = conditional ;
conditional = disjunction, [ "?", expression, ":", conditional ] ;
```

Add to primary (exclude these names from the ordinary-primary alternative):

```ebnf
dynamicOrLiteral = ( "amount" | "shares" ), [ typeArgs ],
                   "(", [ arguments ], ")" ;
financialGeneric = ( "variant" | "project_variant" | "to_uint" ),
                   typeArgs, "(", [ arguments ], ")" ;
```

`u256`, `some_value`, and the four scalar functions use existing ordinary call
grammar. `typeArgs`, arguments and all postfix forms retain /1 definitions.
For amount/shares, immediate `<` selects generic form, `(` selects literal form;
there is no lookup-dependent ambiguity or arbitrary generic call admission.
`variant<F.Tag>` fails parsing at `.`. Empty <> fails parsing, as in /1.

Add `Conditional{condition,consequent,alternative,span}` to the expression AST;
do not encode it as Call or eagerly lowered lets. Existing Call.typeArguments
suffices for all new generic forms. Formatter precedence puts Conditional below
Or and preserves right association, e.g. `(a ? b : c) ? d : e` needs the shown
condition parentheses. Preserve arm order; never swap/fold equivalent choices.
Postfix access on a conditional requires grouping, e.g. `(c ? x : y).field`.
Roundtrip projects out spans/parentheses only, not constructor/type metadata or
extra conversion nodes; AST and lowered-Core meaning must both agree.

New type/intrinsic names are reserved as term binders in this profile only.
The original expression-source/1 admission stays unchanged: for example, a
previously valid /1 binder named `u256` remains valid there. Keep reservation
tables profile-local; do not append financial names to shared /1 reserved sets.
For source metadata admission, extend /1's fixed-generic-name exclusion with
amount, shares, variant, project_variant, to_uint. Include variant family and tag
names in schema-name checks and variant family names in binder collision checks.
Thus schemas using these newly fixed names as identities fail SOURCE_SCHEMA_NAME
under this new profile; do not rename them. Other nominal names can overlap a
type/function spelling where existing metadata roles unambiguously permit it.
Keep /1 enum-prefix collision rules and additionally reject an enum-family prefix
that collides with a variant-family name. Keyword-named metadata remains outside
this component's explicit source domain. Full source identity escaping remains a
SP02 obligation, not hidden aliases or silent loss of metadata.

## Select, arithmetic, work and diagnostics

Select has lower precedence than `or`, right association, and both arms retained.
The runtime checks all children in condition/consequent/alternative order, then
checks condition Bool and exact equal arm types. It rejects top-level Unit or
Operation arm types; no inferred common supertype, width or asset cast. The source
checker delegates to this exact static order rather than short-circuiting checking
because the condition is a literal. Nested data follows existing runtime rules.

At runtime charge Select once, evaluate condition once, then only the chosen
arm. Path child indices are0/1/2. Propagate the chosen child's original failure;
unselected dynamic errors and work are absent. B counts all source-lowered Core
nodes in both arms, plus statement wrappers, inserted != Not/Eq and every explicit
conversion. It never requires workInitial >= B.

Expression `true ? 7 : floor_div(1,0)` has B6 and actual work3; false has ARITH_DENOMINATOR
at the division after5 entries. In `let x = ...;` add one Let node: B7, success4,
false failure6. Budget3 on that Let enters Select and condition, then fails at
the selected literal with workUsed3. No source constant folding changes this.

Keep `*`, floor_div, ceil_div as the source routes to established numeric overloads:
UInt256 same-width arithmetic/comparisons; Amount×UInt128→Amount;
Amount×Amount→AmountProduct; Amount×correctly oriented Price→ScaledAmount;
Amount×Rate→SignedScaledAmount; dimensional product/Amount cancellation;
scaled amount division by the literal128-bit10^scale. No plain `/`, reassociation,
implicit widening or helper expansion. `floor_div(a * p, 10000)` at scale4 is
valid; replacing10000 with an equal-valued local, `to_uint<128>(10000)`, or u64
rejects TYPE_SCALE_DIVISOR. The accepted literal u128(10000) remains LitUInt128.
SignedAmount/NetAmount are not granted new arithmetic merely because they have
source type spellings; use only the published runtime's closed overload table.

Use existing source codes: malformed grammar→UNEXPECTED_TOKEN (or existing
lexical/bound code); wrong generic/call count→SOURCE_ARITY; wrong nominal/numeric
metadata role→SOURCE_TYPE_SHAPE; nonliteral u256/literal forms→SOURCE_LITERAL_SHAPE;
unrecognized intrinsic→SOURCE_CALL. Structurally valid width17 reaches runtime
TYPE_LITERAL; unknown family/tag/asset reaches TYPE_NAME; wrong child width/type
reaches TYPE_MISMATCH; overflow/None/wrong active tag reach the runtime errors above.
Metadata-shape failures use the offending metadata span; arity uses whole call;
runtime semantic failures use their real enclosing Core node span/path. No new
error family is needed. SOURCE_CALL is the inspected spelling in source40's
lowerer and proposal03; all diagnostics must retain that closed set.

Preserve source/1 phase order: transport/envelope; parse; trusted schema and source
name/declaration checks; lower; whole-action static checking; typed snapshots;
reduction. All pre-reduction errors have workUsed0. The outer snapshot transport
cap is2000000 UTF-8 bytes before parsing; Pre/Args/Obs/schema retain independent
65536-byte bounds. Exactly one source action still matches the complete Σ.args
set and exact types. Input strings/results remain independently owned; invalid
results expose neither tentative state nor descriptors.

## Representative source and trusted bindings

This is a specified-only source example, not an executed fixture:

```text
profile "moriarty-financial-expression-source/1";
agreement VaultQuote {
  action quote(deposit: UInt128, supply: UInt128, valuation: UInt128,
               useSupplied: Bool, supplied: Option<UInt128>) {
    requires valuation > 0;
    let calculated = to_uint<128>(floor_div(
      to_uint<256>(deposit) * to_uint<256>(supply),
      to_uint<256>(valuation)));
    let selected = useSupplied ? some_value(supplied) : calculated;
    let allocation = shares<Vault, Alice>(selected);
    let offered = amount<USD>(deposit);
    let quote = variant<Quote, Allocation>(allocation);
    let recovered = project_variant<Allocation>(quote);
    next.last = quanta(recovered);
    emit Notice { allocation: recovered, offered: offered };
    ensures post.last == selected;
  }
}
```

Bind sorted rosters assets=[USD], vaults=[Vault], parties=[Alice], units=[];
variantTypes.Quote={Allocation:Shares<Vault,Alice>, Cash:Amount<USD>} (distinct
payloads); recordTypes.NoticeFields={allocation:Shares<Vault,Alice>,offered:Amount<USD>};
operations.Notice=NoticeFields; fields.last={type:UInt128,writeClass:ordinary};
args exactly the five source parameter types; observations={}. This is explicit
trusted schema binding, not a hidden source type-alias registry.
Proposed input deposit4/supply3/valuation10/useSupplied=false/supplied=None,last0
must return last1, one Notice describing shares1/offered4, and no financial effects.
The widening/divide/narrow expression alone has9 entered nodes; preceding Let
makes10. Source true/None must fail OPTION_NONE and publish no state/descriptors.

## Exact implementation and verification plan (not yet executed)

After source40 result approval and these design votes, use a new isolated worktree
on the integrated source40+published pure48 tree. Preserve all original evidence.

Modify only these shared files initially:

- `src/successor/frontend.ts`: explicit financial profile parser entry, gated `?`,
  Conditional AST and profile-specific generic-name handling. Old entry behavior
  must remain exact; do not mutate exported /1 admission arrays.
- `src/successor/format.ts`: explicit financial formatter entry and precedence.
- `src/successor/financial-expression-v1.ts`: additive `.check()` with a separate
  check-result type, following source40's reviewed shape/schema/whole-static path.
  Current published API has evaluate only. Do not emulate check by evaluating with
  fake snapshots or budget0; do not change evaluate's result union/behavior.
- `package.json`: financial-expression source demo command.

Create `src/successor/financial-expression-source-{frontend,types,lower,v1}.ts`:
named profile entry points; exact type/intrinsic extension; lower Conditional and
reuse the existing controlled source-lowering helper; source-only API adapting
financial schema/static checker/evaluator. Reuse /1 helpers only where semantics
are identical. Small additive exports are permissible after review if required;
do not introduce a configurable compiler framework or caller-provided callbacks.
Public `.check/.elaborate/.evaluate` shapes mirror /1 with the new source/contract
constants; returned Core is inspection data, never accepted as evaluator input.

Create spec `financial-expression-source.md`, complete
`financial-expression-source-grammar.ebnf`, example vault-quote.mori and canonical
schema/snapshots JSON, executable `examples/financial-expression-source.mjs`, and
`tests/financial-expression-source-{profile,v1}.test.mjs`, all under the existing
language package. Cross-link scoped docs after approval; do not relabel old evidence.

Required actual-source cases (specified-only):

1. One real program reaches all48 constructor tags, including all40 preserved
   forms; a separate table checks each new row's exact Core operands/types/spans.
   Constructor coverage alone does not establish overload coverage.
2. Literal-vs-dynamic amount/shares and u256-vs-to_uint: exact nodes/work;0/MAX;
   nonliteral literal rejection; UInt64/256/Amount child mismatch; explicit
   narrowing success/failure; unsigned256 MAX+1 input/literal/arithmetic bounds.
3. All seven new source types in args, records, Option and Collection; sorted and
   unsorted AmountProduct; variant unknown family/tag, duplicate payload type,
   cycle, invalid payload, wrong active case; scalar overload matrix including
   signed minimum magnitude and rejected unsupported pairs.
4. Vault full UInt128 product/max result/narrow overflow/zero divisor/each rounding
   direction; AMM dimensional product/cancellation; Price orientation and exact
   scale-literal distinction; negative Rate floor/ceil. Expected equations come
   from retained independent arithmetic oracles, not the implementation itself.
5. Select true/false/condition failure; unselected divide/projection error skipped;
   wrong type in unselected arm still rejected; nested right association,
   consequent nesting, Or precedence, postfix grouping; budgets and B above.
6. Failed Require/Ensure/selected projection after tentative write+emit rolls back;
   non-ASCII preceding text gives exact UTF-8 failure span/nodePath/work. Exercise
   whole-schema and dead-branch failures before snapshots/reduction.
7. Formatter idempotence, AST projection and Core correspondence for conditional
   arms/generic metadata; profile rejection of new syntax under both old APIs;
   rerun old40 representative values/errors/work unchanged under its original
   profile, all package controls and funded/atomic regressions.
8. Name/metadata keyword collisions;128-case/collection and256-parameter bounds;
   source/core/value/depth limits,2m envelope precedence; schema/snapshot hostile
   objects, returned-Core mutation isolation; additive `.check()` does not reduce.

Run existing package build/test and the new real-source demo; retain initial
failures and corrected runs. Freeze exact files and logs for fresh GPT-6+Grok
result audits. No financial/native/Preview campaigns are part of these commands.

## Full SP02 obligations remain

This is an expression component, not full agreement authoring. SP02 still owns
source-defined record/enum/variant/operation and financial schema declarations,
assets with denomination/identity authority, constants/genesis/state initialization,
multiple actions with separate signatures and deterministic action selection,
source identity escaping, obligation/request/time/valuation distinctions,
parameterized identity instantiation, full lexical/static/canonical specification,
developer CLI check/format agreement with APIs and stable machine-readable errors,
actual-participant matched syntax evaluation, and positive/negative complete loan,
swap, partial-payment and request sources. Trusted-schema one-action convenience
must not become the final language scope. Financial transitions, signing/native/K
correspondence, mandatory proofs, ACTUS/DeFi and Preview acceptance retain their
existing owners/gates. Authoring48 pure nodes closes none of those obligations.
