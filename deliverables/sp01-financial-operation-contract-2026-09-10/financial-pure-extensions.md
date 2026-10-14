# Exact pure extensions needed by the full financial interface

Proposed only; preserve all40 original constructors and the separately reviewed
Boolean /1 proposal. In addition to ConstructShares and the two Variant constructors,
this document adds ConvertUInt, ProjectSome, ScalarValue, ConstructAmount and Select:
48 constructors total.
These are explicit choices for independent review, not published/runtime behavior.

## UInt256 and ConvertUInt

Add finite UInt256 domain0..2^256−1, type encoding `["UInt256"]`, canonical unsigned
decimal scalar, one value node. Extend LitUInt's width metadata to64|128|256, with
exact corresponding static bound. Add same-width UInt256 Add/Sub/Mul/FloorDiv/
CeilDiv and comparisons, preserving checked intermediates, positive divisors,
Euclidean rounding and existing left-to-right semantics. No implicit width mixing.
An operation can still demand UInt128 and reject a wider value at typing.

New `ConvertUInt(width,value)` uses metadata width as canonical decimal
`"64"|"128"|"256"` and one evaluated UInt64/UInt128/UInt256 child. Its result is
the selected UInt width. Static wrong child type is TYPE_MISMATCH; unknown width
is TYPE_LITERAL after closed structural admission. On entry charge1, evaluate
child once, check q<=2^width−1; otherwise ARITH_RANGE at the conversion. Returning
exact q changes no state/effects. No signed, Amount, Shares, Rate, Quantity or
Variant input is accepted. In particular ConvertUInt does not erase nominal units.

Proposed source spelling `to_uint<256>(q)` (and64/128) is a reserved checked
built-in, not arbitrary generics or function dispatch. Its exact node:
`N("ConvertUInt",{"width":"256","value":N(child)},P)` with the ordinary N/P
record encoding. Widening and narrowing both retain their nodes/cost/spans.

The vault rule `floor(a*S/Va)` has the exact pure Core shape:

```text
ConvertUInt(128,
  FloorDiv(
    Mul(ConvertUInt(256,ReadArg(a)),ConvertUInt(256,ReadArg(S))),
    ConvertUInt(256,ReadArg(Va))))
```

With three ReadArg leaves, it has9 entered nodes; CeilDiv has the same count.
Va0 fails at division after evaluating its operands. a,S,Va are UInt128 quanta
bound separately to their nominal roles; the rule does not confuse supply with
Shares<V,H>. Each a*S fits unsigned256 across the full UInt128 input domain.
After division, narrowing rejects a result greater than UInt128 maximum. A final
result that fits is not rejected merely because the128-bit intermediate product
would overflow. The original128-bit Mul remains unchanged and still rejects
such an intermediate when written without explicit widening.

The four conversion directions are unchanged: deposit floors assets*supply/value;
exact mint ceils shares*value/supply; exact withdrawal ceils assets*supply/value;
redemption floors shares*value/supply. This does not define zero-supply or fee
behavior by default. Root's immutable20-case independent vault oracle includes
full-domain products, final overflow, rounding, zero denominator and input bounds.
They are specified arithmetic, not executed Moriarty/K cases.

## ProjectSome

New `ProjectSome(value)` has one evaluated child Option<T> and returns exact T.
Wrong child type is TYPE_MISMATCH. Charge1, evaluate child once; Some(v) returns v,
None rejects OPTION_NONE at the projection. Child failures retain their original
code/span. No state/effect change or copying of authority occurs.

Proposed source spelling `some_value(expr)` is an explicit partial projection.
`not (x == none<T>) and predicate(some_value(x))` can be guarded under the separately
proposed short-circuit Boolean /1 semantics; whole-tree typing and conservative
node bounds still include the right branch. On None, the false left operand skips
projection; directly projecting None rejects. This is not a new implicit match
language. ProjectVariant remains separate and rejects a different variant tag.

Exact node is `N("ProjectSome",{"value":N(child)},P)`. It preserves the existing
Option encoding (None[], Some[payload]); no alternative tag convention is added.
For ReadArg child, success costs2; with budget1 the child entry fails. Source
compiler must retain real call/child spans; synthetic examples cannot claim them.

## ScalarValue: explicit quanta or mantissa access

New `ScalarValue(component,value)` has metadata component `"quanta"|"mantissa"|"negative"|"magnitude"`
and one evaluated child. It is a deliberate pure numeric projection, never a
conversion to another financial resource:

| Component / child type | Exact result |
| --- | --- |
| quanta / Amount<A> | UInt128, exact integer quanta |
| quanta / Shares<V,H> | UInt128, exact integer share units |
| mantissa / Price<A,B,S> | UInt128, exact unscaled mantissa |
| mantissa / Rate<S> | SInt128, exact signed mantissa |
| mantissa / Quantity<U,S> | SInt128, exact signed mantissa |
| negative / Rate<S>, Quantity<U,S>, SignedAmount<A>, NetAmount<A> | Bool, true exactly when the mathematical scalar is negative |
| magnitude / Rate<S>, Quantity<U,S>, NetAmount<A> | UInt128, exact absolute scalar (including magnitude2^127 for signed128 minimum) |
| magnitude / SignedAmount<A> | UInt256, exact absolute scalar (including magnitude2^255 for signed256 minimum) |

No other overload applies. Option/Variant must first be explicitly projected to
their correct case. Compound records and Debt have no ScalarValue overload. Source uses reserved
`quanta(expr)`, `mantissa(expr)`, `is_negative(expr)` or `magnitude(expr)`;
Unit/scale/asset metadata is deliberately not the scalar's type. A caller wanting
nominally safe money multiplication should use the typed arithmetic overloads;
explicit projection makes any loss of static unit information visible in source
and Core, and does not confer permission to reconstruct a different asset.

Static resolve component then child type; wrong pairing TYPE_MISMATCH. Charge1,
evaluate child once, return exact scalar in the table's existing range. Source
value remains unchanged and no financial resource is spent or copied. The exact
node uses only `component` and `value` operands plus ordinary span. Roundtripping
cannot delete it or its cost. ReadArg Amount<AssetA>(5) projected through quanta
returns UInt1285 with2 nodes; direct ConvertUInt(256,that Amount) still rejects.

## Rule bounds under Boolean /1

Whole Core syntax is typechecked and its conservative node-entry upper bound
counts both Boolean children. Actual work counts only entered nodes under the
new left-to-right short-circuit policy. An unselected operand has no dynamic
arithmetic/projection error or charge, but its static error still rejects the
action before evaluation. Rule maxNodeEntries equals full syntax node count,
not actual selected-node count. Preserve this difference in work certificates;
there is no caller-provided optimistic work field accepted as evidence.

## ConstructAmount: explicit dynamic amount construction

New `ConstructAmount(asset,value)` has identifier metadata asset and one evaluated
UInt128 child. Resolve asset in Σ.assets, then require exactly UInt128. Unknown
asset is TYPE_NAME; another width, Rate, Shares or Amount child is TYPE_MISMATCH.
Its result is Amount<asset>, the exact child quanta at that declared asset index.
There is no exchange, movement, mint, authority or claim creation. Quantity bounds
already follow from UInt128; normal aggregate/result bound checks still apply.

Charge1 before evaluating the child once; retain child error/span and work failure.
The result encodes with the existing Amount scalar representation. The exact node
is `N("ConstructAmount",{"asset":A,"value":N(child)},P)`; source reserved built-in
is `amount<A>(q)`. This parallels `shares<V,H>(q)` with its distinct nominal type.
No automatic cast is inserted into arbitrary source arithmetic. The financial
operation consumer must still establish actual availability, asset identity,
policy, signed debit scope and gross/fee counters. Constructing Amount<USD>(5)
from UInt1285 cannot turn five held AssetA into USD.

For a ReadArg UInt128 child, exact entered work is2; budget1 fails on the child.
MAX=2^128−1 constructs successfully; a supplied MAX+1 never reaches construction
because UInt128 input admission rejects. A UInt256(MAX) child rejects statically;
`amount<A>(to_uint<128>(wide))` states checked narrowing explicitly. A subsequent
Transfer expecting AssetB rejects the differently indexed Amount<A>.

The eight additions preserve all40 original constructors. The companion
`financial-pure-signatures.json` lists exact metadata, evaluated children, results,
errors and source forms. No generic function evaluator or new ambient cast is
introduced. Expressions may build values whose later financial use is rejected;
construction is not proof of a spendable balance.

## Select: an explicit conditional value, not a host branch

New `Select(condition,consequent,alternative)` has three evaluated-expression
operands and no metadata. Statically type all three in that lexical order;
condition must be Bool and the arms must have the identical complete finite
pure value type T. Differing assets, holders, scales, widths or Record/Variant
names are TYPE_MISMATCH even in an arm that would not run. Statement constructors
remain forbidden in pure positions. A rule additionally excludes Unit/Operation
inputs/results as specified in DR1. No least-upper-bound or automatic cast exists.

On entry charge1, evaluate condition once, then evaluate only the selected arm.
True selects consequent; false selects alternative. Return that value with no
additional conversion or state/effects. The skipped arm has no dynamic charge
or dynamic failure. Preserve a selected child's actual error/span. Full syntax
admission/typechecking and the conservative node bound include both arms; actual
work includes only the chosen one, consistently with Boolean /1.

Exact node: `N("Select",{"condition":N(c),"consequent":N(t),
"alternative":N(f)},P)`. Its Felleisen–Hieb value contexts add
`Select(E,eT,eF)`; reduction Select(true,eT,eF)→eT and
Select(false,eT,eF)→eF chooses the next evaluated expression. The parent entry
charge happens once, not again at branch selection. Existing context/error
propagation applies to the selected child; no context reduces an unchosen arm.

Proposed reserved source syntax is `condition ? consequent : alternative`.
The grammar level is lower precedence than Or and associates to the right:
`conditional ::= orExpression [ "?" expression ":" conditional ]`.
The consequent expression may itself be a conditional. Parentheses select other
association. It is an expression, not an implicit statement branch or user
function. The source elaborator must retain condition/arm spans and both arm ASTs.
Canonical expression files are not changed by this proposal.

Select(true,LitUInt128(7),FloorDiv(LitUInt128(1),LitUInt128(0))) returns7,
actual work3, conservative tree bound6. Switching true to false yields DIV_ZERO
at the division after5 actual entries. Select(true,Amount<A>(1),Amount<B>(1))
rejects statically. If the condition itself fails, neither arm runs. Budget2
fails entering the selected literal even though the skipped arm is harmless.

This addition is necessary for one finite declarative rule body to select vault
conversion directions, or compute a signed delta's magnitude without an undeclared
host if/absolute-value operation. It does not itself supply any financial formula.
`reference-vault-rule.json` gives one exact four-direction, zero-fee body with
explicit synthetic spans and an initialized-vault domain. Its source/schema
and arithmetic checks are specified-only, not execution by Moriarty or K.
