# Proposed explicit dynamic share constructor

**New reviewed-extension proposal, not published Core behavior.** Preserve all40
expression constructors, including literal-only LitShares. Add exactly one pure
constructor, `ConstructShares`, to form a holder-indexed share *value* from a
computed unsigned quantity. This closes the specific missing introduction rule;
it does not mint shares, transfer a claim, certify vault supply or authorize a
holder. All new types/rules still need independent GPT-6 and Grok design/source
votes and a versioned representation before implementation.

## DS1: Surface, Core and exact type

Proposed compact source spelling, with both type arguments referring to declared
compile-time identities:

```text
let allocation = shares<Vault01, Alice>(calculated_units);
```

`calculated_units` must already have type UInt128. `shares` is a reserved built-in
in this proposed extension, not an arbitrary function-call facility. Vault01 and
Alice are metadata identifiers resolved in Σ.vaults and Σ.parties. Runtime values
cannot choose a type argument. Parameterized source must resolve these identities
through the bounded declared instantiation before producing concrete Core.
A same-spelled vault/party in another network must resolve through the bound
identity registry; textual similarity grants no equivalence.

The exact new Core node shape is:

```json
{"constructor":"ConstructShares","operands":{"holder":"Alice","value":{"constructor":"ReadArg","operands":{"name":"calculated_units"},"span":{"kind":"synthetic","start":"0","end":"0"}},"vault":"Vault01"},"span":{"kind":"synthetic","start":"0","end":"0"}}
```

This is a synthetic Core illustration, not an actual source-span derivation.
Use candidate02's exact N/P canonical key sorting, UTF-8 and byte/node limits.
A real source compiler must replace synthetic spans with the half-open UTF-8
spans of the call and its argument, and retain the source hash. Node operands
are exactly `vault`, `holder`, `value`; no other fields, widths or coercion flags.
`vault`/`holder` are identifier metadata. `value` is exactly one recursively
admitted Core expression. A supplied amount scalar directly in that position
fails INPUT_SCHEMA before typing; literal quantities use the existing LitUInt
node of width128 as its child.

Static rule, after structural admission:

```text
V in Σ.vaults    H in Σ.parties    Σ;Γ;phase ⊢ e : UInt128
-------------------------------------------------------
Σ;Γ;phase ⊢ ConstructShares(V,H,e) : Shares<V,H>
```

Resolve V then H, then typecheck e. An absent V/H rejects TYPE_NAME. A UInt64,
Amount, already-indexed Shares or signed value rejects TYPE_MISMATCH; no width,
asset, holder or vault conversion occurs implicitly. Existing whole-action static
checks still precede all runtime evaluation, so an unknown holder in this node
cannot be hidden behind a false earlier guard.

## DS2: Reduction, cost and frames

ConstructShares is pure, never Unit. On entry consume one ordinary work unit;
then evaluate its one child exactly once, with the unchanged expression contexts
and local budget. Child rejection is propagated without wrapping/erasing its
code or original child span. After a UInt128 result q, return the mathematical
value Shares<V,H>(q), whose existing candidate02 payload is canonical decimal q.
The result W wrapper is the existing `Shares` type with V/H parameters; no new
value encoding is introduced. q=0 and q=2^128−1 are valid pure values. An operation
may separately require positive actual share issuance or available ownership.

No balances, supply, receipts, claims, fields, descriptors, authority or recovery
work change. It does not check whether H currently owns q shares: values can
represent requested or quoted quantities. ShareMint/ShareBurn must still enforce
actual funding, supply, holder and entitlement rules. Constructing the same value
twice does not duplicate financial ownership.

The operational context extension is the single frame
`ConstructShares(V,H,[])`. The total entered-node count is1 plus that of its child;
a ReadArg child therefore costs2. Work1 enters the outer node then fails entering
the child with WORK_EXHAUSTED and local workUsed1. Work2 succeeds with remaining0.
A large pure argument expression retains all its intermediate bounds and costs;
no special unmetered share-conversion helper is created.

## DS3: Source target and independently specified cases

Vault supply stays a separately declared UInt128 financial field; it is not a
Shares<V,H> field pretending that one holder owns all supply. A no-fee homogeneous
vault with S3 and accounting basis Va10 computes1 share for deposit4 under the
explicit floor conversion. After a checked UInt128 result `calculated_units=1`,
`shares<Vault01,Alice>(calculated_units)` returns Shares<Vault01,Alice>(1).
Its later ShareMint still requires actual deposit4, the selected conversion rule,
exit policies and full financial effects. Pure construction alone cannot credit
Alice. General256-bit product/division arithmetic for that computation is part
of the separately proposed financial arithmetic/typed rule work, not provided
by this constructor itself.

| Case | Expected proposed result |
| --- | --- |
| known V/H, UInt128 argument1, initialWork2 | Shares<V,H>(1), remaining0, no effects |
| known V/H, UInt128 argument0 | Valid zero share value; ShareMint positivity remains a separate operation guard |
| known V/H, UInt128 maximum | Valid full-domain share value; exact existing Shares W encoding |
| unknown vault or holder | TYPE_NAME before runtime |
| UInt64 argument1 | TYPE_MISMATCH; no automatic widening |
| Amount<A>(1) argument | TYPE_MISMATCH; no asset/share cast |
| Shares<V,Bob>(1) argument for Alice | TYPE_MISMATCH; cannot erase a holder index |
| checked UInt128 Add(maximum,1) child | Child ARITH_RANGE; no share value returned |
| ReadArg child and initialWork1 | WORK_EXHAUSTED entering child, workUsed1, no effects |
| two constructed share values with same V/H/q | Two equal pure descriptions, zero financial supply increase |
| constructed Shares<V,Alice>(1) passed to an operand requiring Shares<V,Bob> | TYPE_MISMATCH at consumer; identical quantity is insufficient |

These are hand-derived semantic cases, not claimed interpreter executions.
The complete new signature is recorded in `dynamic-shares-signature.json`.
Neither the old40 constructor inventory nor the expression candidate02's tests
are edited. This new41-constructor profile requires its own complete constructor
roster, grammar/formatter/elaboration and evaluator/K changes after the remaining
full financial/signing design is approved. The existing finite source target is
therefore assigned an explicit decision and contract rather than left as an
unnamed future conversion.
