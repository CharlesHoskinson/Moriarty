# Surface sugar and its desugaring rules

Design proposal, 2026-09-13. Companion to
`moriarty-surface-language-2026-09-13.md` (kinds, unit literals, profile
extension) and the jet-discipline deliverables of the same date. Target:
a source profile `/7` that elaborates to the existing Core/4 constructor
set with no new constructor. Every construct is defined by an expansion
into `/5` source; the elaborated Core stays the artifact of record.

## 0. Two rules every construct must pass

**Substitution, never binding.** `financial-expression-v1.ts:347` checks
that the divisor of a `ScaledAmount` is *syntactically* a `LitUInt` node.
A `Let`-bound copy of the same literal fails `TYPE_SCALE_DIVISOR`. So no
expansion in this document introduces a `Let`. Where a default or a
literal is used at several sites, the expansion copies the node at each
site. The cost of that choice is honest: a copied literal is charged once
per site, exactly as the hand-written form is charged today.

**Term-count parity with the twin.** The work meter is published in the
result (`workRemaining`, v1.ts:36-37), so term count is an observable.
Every construct here has a *twin*: the `/5` source a careful author would
have written by hand. The construct is admissible only if its elaborated
Core is byte-identical to the twin's, which makes its work consequence
zero *relative to the twin*. Where a construct generates terms the author
would otherwise have omitted (the frame), the cost is stated and the
elaborator reports it. Nothing here expands into `Select`, `And` or `Or`,
so the one-unit drift the computation slice found never arises from sugar.

The survey rule applies throughout: the sound spelling must be the short
one, or the only one.

## 1. Frame inference

Practice served: *infer what the compiler can prove, state what it
cannot*. The compiler can prove nothing about what the author did not
touch unless it knows the footprint of every effect, so the default has
to be defined in terms of footprints, not assignments alone. The brief's
"everything not assigned is unchanged" is incomplete: `emit Transfer`
assigns nothing and changes four ledger cells.

### Domain

The frame ranges over the **declared domain** only:

- ordinary state fields (`state paid: UInt128;`),
- `balance`, `allowance_remaining`, `allowance_spent` for every declared
  `party` x declared `asset`,
- `principal`, `accrued`, `outstanding` for every declared `obligation`.

`loan-lifecycle.state.json` carries a party `Other` and an asset `Token`
that the source never names. Those rows are outside the frame and the
evidence record says so (`frame.domain` below). Binding on-ledger state
to the declared domain is the open question the companion document
lists; this document does not invent an answer. The `obligation`
declaration is the companion's; until it exists the obligation domain is
the set of string literals used as `obligationId` or as an obligation
read's identity, and the evidence record marks that interim rule.

### Footprints

A pinned table, part of profile `/7`, one row per protected operation:

| Operation | Touches |
|---|---|
| Transfer, Fee | `balance(from)`, `balance(to)`, `allowance_*(from)` |
| Originate | `principal/accrued/outstanding(obligationId)` |
| Accrue | `accrued/outstanding(obligationId)` |
| Repay | `principal/accrued/outstanding(obligationId)`, `allowance_*(payer)` |

The table is derived from `financial-lifecycle.ts` and the repository
does not yet state it in one place; pinning it is the first task, and an
operation with no row cannot be emitted from `/7`.

### Default, escape hatch, denial

```
action accrue(accrualId: Text, periodIndex: UInt64, observedTime: UInt64) {
  emit Accrue { accrualId, obligationId: Loan1, periodIndex, observedTime };
  ensures post_accrued<Cash>(Loan1) == accrued<Cash>(Loan1) + 10 Cash;
}
```

Default: for every domain cell not in the `written` set (v1.ts:150) and
not in the footprint of any emitted operation, the elaborator appends
`ensures post_X == pre_X`. Touched cells get no generated condition; the
author writes what holds there, or nothing.

Escape hatch: `action settle() modifies balances<Cash>, obligations { ... }`
removes the named regions from the frame. Regions are `state.<field>`,
`balances<A>`, `allowances<A>`, `obligations`, or a single cell such as
`balance<Cash>(Lender)`. The word is a promise the author is making about
breadth, and it is the dangerous case, so it is explicit.

Denial: `action migrate() modifies * { ... }` generates no frame. The
evidence record then carries `"frame": "denied"` rather than an empty
list, so a reviewer cannot mistake a denied frame for a vacuous one.

### Expansion and what the artifact contains

Each generated condition is exactly the `/5` line the author would have
written, elaborated through the same lowering:

```
Ensure(Eq(ReadPostBalance(Cash, LitText "Lender"), ReadBalance(Cash, LitText "Lender")))
```

The elaborated Core carries every generated `Ensure` as an ordinary
suffix statement, so it is hashed, evaluated and rejected
(`ENSURES_FAILED`) exactly like a typed one. Beside the Core the
elaborator emits a `frame` block:

```json
{ "action": "originate",
  "domain": { "parties": ["Lender","Borrower"], "assets": ["Cash"],
              "obligations": ["Loan1"], "obligationDomain": "declared" },
  "footprint": ["balance<Cash>(Lender)", "balance<Cash>(Borrower)",
                "allowance_remaining<Cash>(Lender)", "allowance_spent<Cash>(Lender)",
                "principal<Cash>(Loan1)", "accrued<Cash>(Loan1)", "outstanding<Cash>(Loan1)"],
  "generated": ["post.paid == pre.paid",
                "post_allowance_remaining<Cash>(Borrower) == allowance_remaining<Cash>(Borrower)",
                "post_allowance_spent<Cash>(Borrower) == allowance_spent<Cash>(Borrower)"],
  "suffixWork": "18" }
```

Work: one generated cell costs 6 units (`Ensure`, `Eq`, two reads, two
identity literals), the same as the hand-written line. Relative to the
twin, zero. Relative to an author who wrote no frame, linear in the
domain; `frame.suffixWork` reports it so `workInitial` can be set. No
declared-cost rule is needed unless the frame is one day checked outside
the suffix, which is not proposed.

Eleven of the thirty-nine `ensures` in the example are fixture values on
*untouched* cells (`post_allowance_remaining<Cash>("Borrower") ==
amount<Cash>(110)` in `originate`). The frame replaces them with
`post == pre`, which holds in every state rather than one snapshot. A
contract should not know its test fixture.

## 2. Flow syntax

Practice served: *put the obligation in the grammar, never in a
convention*. The audit found that the capability assertion proves the
lender knows a secret and nothing debits the lender: both value legs draw
on the submitting transaction's offer. Amounts are constrained; party
attribution is not.

`/5` already requires `from:` on every `Transfer`; the hole is that
nothing downstream consumes it, and the 5.1a handover says there is "no
source-level slot to bind authority". So the flow form does two things:
the payer becomes a `party` identifier rather than `Text`, and the
elaborator emits a *funding claim* the ledger binding must discharge or
the artifact is not admissible. The grammar cannot discharge the claim;
it can make its absence impossible.

```
flow = party, "pays", expression, "to", party, [ "as", expression ], ";" ;
```

`Lender pays 100 Cash to Borrower as transferId;` expands to

```
emit Transfer { id: transferId, from: "Lender", to: "Borrower",
                settlementAsset: "Cash", transferAmount: amount<Cash>(100) };
```

plus, outside the Core term, in the elaboration output:

```json
{ "funding": [ { "descriptor": 0, "payer": "Lender", "asset": "Cash", "amount": "100" } ] }
```

In `/7` the spellings `emit Transfer` and `emit Fee` are rejected
(`SOURCE_FLOW_REQUIRED`); `pays` is the only way to move value, and
`pays` has no form without a payer. A fee is `Borrower pays 1 Cash to
Treasury as fee feeId;`. The payer and payee positions accept declared
`party` names only; `"Lendor"` is a parse error.

Work: the emit costs what it costs today (`Emit`, `ConstructRecord`, five
field nodes). The funding claim is not a Core term and is charged 0, a
declared cost stated here so it is on record; if the claim is ever
checked in-circuit it needs a declared constant, not the cost of an
expansion. Survey rule: the sound spelling is the only spelling.

## 3. Defaults and elision

Practice served: *make the common case short and the dangerous case
explicit*. Eighty lines of the example restate fields that never vary.

```
operation Transfer {
  id: Text;
  from: Party;
  to: Party;
  settlementAsset: Text = "Cash";
  transferAmount: Amount<Cash>;
}
```

Rules, each decided:

1. A field may be elided at an emit site only if the operation declares a
   default. There is no inference from context.
2. A default is a *closed literal*: a literal, or a record literal whose
   every field is a closed literal. No reads, no arguments. This keeps
   the copied node syntactically a literal (rule 0) and keeps the default
   diffable as a value.
3. `from`, `to`, `payer`, `debtor`, `creditor` may not carry defaults.
   Party attribution is the dangerous case and is written at every site.
4. Field punning follows Rust exactly: `{ transferId, originationId }`
   means `{ transferId: transferId, originationId: originationId }`. It
   is familiar and it introduces no new meaning.
5. Nested record literals drop the `record<Row>` prefix when the field
   type fixes the record: `conversion: { mantissa: 1, scale: 0, rounding: "none" }`.
6. Unit literals from the companion design (`100 Cash`) elaborate to
   `LitAmount` or `LitQuantity` by the expected type of the position;
   in an `==` the read's type decides. An unresolvable position is a
   static error, never a guess.

Expansion: every elided field becomes the node the author would have
typed, copied at the site. The inline field list replaces the separate
`record XFields`; the elaborator emits the record under its old name so
Core `/4` sees no change. Work: zero relative to the twin. Survey rule:
elision removes restatement, not a check.

## 4. Implicit machinery

Practice served: *hide what has exactly one correct form*. The phase
counter has one: `next.phase = pre.phase + 1` followed by
`ensures post.phase == pre.phase + 1`. An author who writes it can only
get it wrong.

```
state phase: Sequence;
```

`Sequence` is a type, following Cairo's lesson that a property enforced
by a type cannot drift from an annotation. A field of that type is
incremented by every action; `next.phase = ...` on it is rejected
(`SOURCE_SEQUENCE_WRITE`); `pre.phase` and `post.phase` remain readable.
The elaborator inserts both lines, so the increment is in the Core and
the postcondition is in the evidence. Wire type is `UInt128`, unchanged.

The rule for when hiding is safe, applied to everything in this section
and refused for everything else: a construct may be hidden when (a) it
has exactly one correct expansion, independent of the site; (b) the
expansion is pinned and reviewable; (c) it removes no check and adds no
shorter unsound read. The repay guard (`requires is_negative(nominal) ==
false; requires payment > 0;`) fails (a): whether zero is a valid payment
is the author's decision, so it stays visible.

Work: 10 units per action, identical to the hand-written form (4 for the
write, 6 for the ensure).

## 5. The desugaring contract

Practice served: *if you cannot diff it, you cannot review it*. Cairo
pins generated source; Moriarty's artifacts are hash-bound and
independently audited, so the bar is higher: the expansion must be pinned
*and* proven equal to a hand-written program.

Every construct has a directory `spec/successor/sugar/<construct>/` with
numbered cases. Each case is four files:

| File | Content |
|---|---|
| `NN.mori` | the `/7` source |
| `NN.twin.mori` | the `/5` source the author would have written, by hand |
| `NN.core.json` | canonical Core/4 from elaborating either |
| `NN.frame.json` | the `frame` and `funding` blocks and per-statement work |

The conformance test is one assertion per case:
`elaborate7(NN.mori) == elaborate5(NN.twin.mori)`, byte-for-byte, and
both equal to `NN.core.json`. This defines each construct by its twin,
proves the parity claim of section 0 mechanically, and makes any change
to an expansion rule show up in review as a diff against a file a human
wrote. The Core hash remains the admitted artifact; the surface digest is
recorded as provenance only.

Cases required before a construct is admitted: the `loan-lifecycle`
origination action, one `modifies` and one `modifies *` case, one punning
case, one case per default-carrying operation, one `SOURCE_FLOW_REQUIRED`
and one `SOURCE_SEQUENCE_WRITE` rejection, and one case whose twin fails
`TYPE_SCALE_DIVISOR` under a `Let` rewrite, kept as a regression for
rule 0.

## 6. Before and after

Before, `loan-lifecycle.mori` lines 57-98, **42 lines**, with the record
and operation declarations it depends on adding 46 more.

After, with `party Lender; party Borrower; obligation Loan1; asset Cash;
state phase: Sequence; state paid: UInt128;` and this operation:

```
operation Originate {
  obligationId: Obligation; transferId: Text; originationId: Text;
  debtor: Party; creditor: Party;
  nominalAmount: Quantity<Units<Cash,1>,0>;
  denomination: Text = "Cash";
  settlementAsset: Text = "Cash";
  conversion: ConversionFields = { mantissa: 1, scale: 0, rounding: "none" };
  allocationRule: Text = "AccrualFirst";
  accrualTerms: AccrualTermsFields;
  nominalLiabilityCap: Quantity<Units<Cash,1>,0>;
}
```

the action is **11 lines**:

```
action originate(transferId: Text, originationId: Text) {
  Lender pays 100 Cash to Borrower as transferId;
  emit Originate {
    obligationId: Loan1, transferId, originationId,
    debtor: Borrower, creditor: Lender,
    nominalAmount: 100 Cash, nominalLiabilityCap: 110 Cash,
    accrualTerms: { numerator: 1, denominator: 10, rounding: "floor",
                    periodSeconds: 60, firstPeriodStart: 1000 }
  };
  ensures post_balance<Cash>(Lender) == balance<Cash>(Lender) - 100 Cash;
}
```

The elaborated Core for this action contains: the phase write and its
postcondition; the `Transfer` emit with all five fields; the `Originate`
emit with all twelve; the author's one `ensures`; and the generated frame
(`paid`, Borrower's two allowance cells). Six of the original twelve
postconditions were fixture values on touched cells (`post_principal ==
100`, `post_balance(Borrower) == 110`); they are the author's to keep as
`ensures` if wanted, and the example keeps one to show the delta form.
The `frame` block lists the seven-cell footprint and the three generated
conditions. Suffix work: 6 (phase) + 6 (author) + 18 (frame) = 30 units,
against 6 + 72 = 78 in the original, because the fixture-valued lines are
gone; every remaining unit corresponds to a line in the twin.

## 7. Where this disagrees with the brief

- "Everything not assigned is unchanged" is the wrong default on its own;
  the default is *not assigned and not in an emitted footprint*, and the
  footprint table is the thing to pin first.
- The frame does not carry "all 39 conditions", because eleven of them
  are not frame conditions. It carries every non-effect, as `post == pre`,
  which is stronger than the fixture values it replaces.
- Flow syntax closes the hole at the source and names it at the seam. It
  cannot close the seam; claiming the grammar alone fixes the funder
  problem would be the ZKsync error from the other side.
- The companion's `Loan1.accrue(at)` and `10% per 60s` forms are not
  adopted: they infer from context, which rule 3.1 forbids, and their
  twins need rate-scale decisions the repository has not made.
