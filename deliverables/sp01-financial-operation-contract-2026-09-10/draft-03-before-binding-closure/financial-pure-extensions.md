# Exact pure extensions needed by the full financial interface

Proposed only; preserve all40 original constructors and the separately reviewed
Boolean /1 proposal. In addition to ConstructShares and the two Variant constructors,
this document adds ConvertUInt, ProjectSome and ScalarValue:46 constructors total.
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

New `ScalarValue(component,value)` has metadata component `"quanta"|"mantissa"`
and one evaluated child. It is a deliberate pure numeric projection, never a
conversion to another financial resource:

| Component / child type | Exact result |
| --- | --- |
| quanta / Amount<A> | UInt128, exact integer quanta |
| quanta / Shares<V,H> | UInt128, exact integer share units |
| mantissa / Price<A,B,S> | UInt128, exact unscaled mantissa |
| mantissa / Rate<S> | SInt128, exact signed mantissa |
| mantissa / Quantity<U,S> | SInt128, exact signed mantissa |

No other overload applies. Option/Variant must first be explicitly projected to
their correct case. NetAmount, compound records and Debt have no accidental
ScalarValue overload. Source uses reserved `quanta(expr)` or `mantissa(expr)`;
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
