# Jet discipline: the financial and effect slice (17 constructors)

## Verdict

None of the seventeen constructors in this slice is a Simplicity Core term, and only two of them are irreducibly primitive. The twelve financial reads are projections out of state values that the evaluator already holds as inputs (`financialPre`, bound in the constructor; `financialPost`, bound by `bindFinancialPost`), so each is `take`/`drop` navigation plus one keyed lookup, and the pre/post pair differ in nothing but which of the two values is projected. `Outstanding` is a jet with a trivial correctness proof because the lifecycle state model maintains `outstanding = principal + accrued` as a checked invariant on every path. The five statements are not expressions at all: `Require` and `Ensure` are Simplicity `Assertion` terms, `Let` is a binder that `pair` threading eliminates, and `Emit` and `NextWrite` are output effects with no Simplicity analogue below the Primitive layer. The irreducible primitive content of the slice is a keyed lookup into obligations and a keyed lookup into party-asset records; everything else is derived.

## Table

| Constructor | Layer | Derivable from | Jet candidate | Proof obligation |
|---|---|---|---|---|
| ReadPrincipal | Core projection over a primitive lookup | `lookupObligation(pre, id)` then field `principal` | yes | lookup agreement only |
| ReadAccrued | Core projection over a primitive lookup | same lookup, field `accrued` | yes | lookup agreement only |
| ReadOutstanding | Core (derived) | `Add(ReadPrincipal, ReadAccrued)` | yes | `outstanding = principal + accrued` (invariant, see below) |
| ReadBalance | Core projection over a primitive lookup | `lookupPartyAsset(pre.balances, id, asset)`, field `amount` | yes | lookup agreement only |
| ReadAllowanceRemaining | Core projection over a primitive lookup | `lookupPartyAsset(pre.allowances, id, asset)`, field `remaining` | yes | lookup agreement only |
| ReadAllowanceSpent | Core projection over a primitive lookup | same lookup, field `spent` | yes | lookup agreement only |
| ReadPostPrincipal | same as ReadPrincipal over `post` | `ReadPrincipal` with phase = post | yes | phase gate preserved (`TYPE_POST_SCOPE`) |
| ReadPostAccrued | same over `post` | `ReadAccrued` with phase = post | yes | as above |
| ReadPostOutstanding | Core (derived) | `Add(ReadPostPrincipal, ReadPostAccrued)` | yes | invariant plus phase gate |
| ReadPostBalance | same over `post` | `ReadBalance` with phase = post | yes | phase gate preserved |
| ReadPostAllowanceRemaining | same over `post` | `ReadAllowanceRemaining` with phase = post | yes | phase gate preserved |
| ReadPostAllowanceSpent | same over `post` | `ReadAllowanceSpent` with phase = post | yes | phase gate preserved |
| Require | Assertion | `assertr`/`fail` on a `Bool` sum | no (it is the assertion, not a wrapper) | failure code `GUARD_FAILED` is metadata, not semantics |
| Ensure | Assertion, suffix-scoped | `Require` plus a placement rule | no | placement rule (`TYPE_STATEMENT_PLACEMENT`) is a static check outside the term |
| Let | Core binder | `pair` threading / de Bruijn | no | duplicate-binder check becomes scoping |
| NextWrite | Statement-layer output effect | not derivable inside a pure term | no | none in Core; must live in a writer monad or an outer statement layer |
| Emit | Statement-layer output effect | not derivable inside a pure term | no | descriptor bound (`DESCRIPTOR_BOUND`) is a resource check outside Core |

## The reads are projections, not effects

The evaluator does not consult a ledger when a read is reduced. It consults a map it built from a value it was given. `financial-expression-v1.ts:157-166` shows the state arriving as a constructor argument and being indexed once:

```ts
constructor(schema: Schema, contract = FINANCIAL_EXPRESSION_CONTRACT_V1, financialPre?: RepaymentState | LifecycleState) {
  ...
  for (const item of financialPre.obligations) this.obligations.set(item.id, item);
  for (const item of financialPre.balances) this.balances.set(pairKey(item.party, item.asset), item);
  for (const item of financialPre.allowances) this.allowances.set(pairKey(item.party, item.asset), item);
```

The reduction rule at lines 423-445 is then a two-way branch on phase, one keyed `get`, a unit check, and a field projection:

```ts
const obligation = (post ? this.postObligations : this.obligations).get(identity);
if (obligation === undefined) fail('MISSING_OBLIGATION');
if (obligation.denomination !== o.unit) fail('NOMINAL_UNIT');
result = obligation[field];
```

In Simplicity terms `financialPre` is a component of the environment tuple, the phase branch is `take`/`drop`, the field projection is `take`/`drop` again, and the only part that is not Core is the keyed lookup with its `MISSING_*` failure. Note that even that lookup is not a `Primitive` in the `Prim.sem : t A B -> A -> env -> option B` sense of `Primitive.v:24`, because the thing being looked up is an input value rather than the ambient `env`; it is a partial function on a value, which in Simplicity is a Core term over a bounded map plus an `assertl`. Whether Moriarty should model the state as a Core value (bounded, since `LIFECYCLE_BOUNDS.collectionCapacity` caps the collections) or as a `Prim.env` is a design choice the repository does not settle; the code is consistent with either, and the difference matters only for what the Merkle root commits to.

The `NOMINAL_UNIT` check is a second assertion, and `numericFits(type, result)` at line 435 a third. All three are `Assertion`-layer, not lookup semantics.

## Outstanding is a sum, and the state model says so

The user asked whether rounding or the liability cap could break `outstanding = principal + accrued`. They cannot, because the lifecycle model asserts the equality on admission and rebuilds it on every transition. Admission, `financial-lifecycle.ts:854-859`:

```ts
const sum = addU128(p, a);
if (sum === null || sum !== o) {
  return bad('INVARIANT');
}
```

Repayment, lines 1614-1619: `nextOutstanding = nextPrincipal + nextAccrued` and all three are written together. Accrual, lines 1791-1795: interest is rounded once (`floor`/`ceil` per `AccrualRounding`) and then added to both `previousAccrued` and `previousOutstanding`, so rounding enters the two sides identically. The liability cap at line 1818 rejects the whole transition (`LIABILITY_CAP_EXCEEDED`) rather than clamping one component. The same invariant is present in `repayment.ts:594-596` for the older `RepaymentState`, and in the lifecycle K semantics at `moriarty.k:80` and `:98` as `(P +Int I ==Int O)`.

So `ReadOutstanding` is a jet of `Add(ReadPrincipal, ReadAccrued)` and the proof obligation is discharged by the state invariant, not by a rounding case analysis. The only residual is `ARITH_RANGE`: `Add` in the expression language checks `numericFits` on a `Quantity` of the obligation's unit, while the stored field is already `UInt128Text` bounded by `SIGNED128_MAX` (line 1815), so the derived path cannot fail where the native path succeeds. What the repository does not settle is whether the invariant is guaranteed for a `financialPost` handed to `bindFinancialPost` by the host; the admission function is the only enforcement point, and `continueSuffix` at line 589 does not re-admit.

## Pre and post: one read, one phase bit, one gate that must survive

`ReadPost*` differs from `Read*` in exactly two places: the map it projects from (`post ? this.postObligations : this.obligations`) and the static gate at line 310, `if (FINANCIAL_POST_READS.has(k) && !ensure) fail('TYPE_POST_SCOPE')`. Everything else, operand roles (lines 73-86), type (lines 312-318), and failure codes, is shared. Six reads parameterised by a phase is the honest shape.

The gate is the part that must not be lost. The evaluator only binds `financialPost` after the kernel has run (`bindFinancialPost`, line 493, called from `continueSuffix`), and `begin` marks a statement `ensure` only from the first `Ensure` onward (lines 476-482). A `ReadPost*` in the prefix is therefore rejected at typing time, not at runtime, which is the soundness property that distinguishes this design from a read-whatever-is-current model. If the collapse is done as a `phase` operand on a single constructor, the gate becomes a typing rule keyed on the operand, which is exactly what `ReadPre` already does with its `view` operand at lines 283-285 (`TYPE_NEXT_READ`, `TYPE_POST_SCOPE`). The rule "post-phase reads type only under `Ensure`" is unchanged; what changes is that it is stated once for a phase bit instead of six times for six names. There is no shorter unsafe spelling because `phase = post` outside an `Ensure` suffix still fails to type. The thing that would break the property is a Simplicity-style encoding that threads `post` as an ordinary environment component visible to the prefix; the phase must be a type-level distinction (a different environment type for prefix and suffix terms), not a value in one environment.

## The five statements

`Require` and `Ensure` are `Assertion.assertr`/`fail` on a `Bool`: `financial-expression-v1.ts:412` reduces both to "if false, fail with a code", and the K rule `expression-v1.k:126` does the same through `exGuardCode`. The difference between them is placement, not meaning. Simplicity's `AssertionSem_mixin` (`Alg.v:461-472`) maps these to `mzero` in a `CIMonadZero`, which is precisely the `fail` semantics here; the error code is commitment metadata like Simplicity's `hash256` argument, not part of denotation.

`Let` binds a name into `this.locals` (line 413) and into `localTypes` (line 306). Nothing reads it except `ReadLocal`. It is `pair` threading with a name; in a nameless term it disappears and `ReadLocal` becomes `take`/`drop`. The duplicate-binder check at lines 238-243 is a shadowing rule that a positional environment makes unnecessary.

`NextWrite` and `Emit` produce outputs. `NextWrite` appends to `this.writes` (line 414), which becomes the `post` overlay at line 520; `Emit` appends a descriptor and enforces resource bounds (lines 415-422). Simplicity has no output effect below delegation, because its programs are predicates; the analogue is that the expression returns a `(writes, descriptors)` value and the host applies it. Both belong to a statement layer outside the term language, with `NextWrite` modelled as construction of the post record and `Emit` as construction of a bounded list. Neither is a jet candidate, because a jet must be denotationally the identity on a term that already exists.

## Irreducible count: two

Two things in this slice cannot be expressed as Core terms plus assertions over the input state: a keyed lookup of an obligation by identifier, and a keyed lookup of a party-asset record (used by balance, allowance remaining and allowance spent). Everything else reduces: five of the six reads are field projections after one of those lookups, the sixth is an `Add`, the post variants are a phase bit, `Require`/`Ensure` are one assertion form with a placement rule, `Let` is a binder, and `Emit`/`NextWrite` are output construction outside the term language. Seventeen names carry two primitive operations. Even those two may be Core if the state is modelled as a bounded value, which the capacity bound at `LIFECYCLE_BOUNDS.collectionCapacity` permits; the repository does not say which representation the Merkle commitment should see, and that is the one open design question this slice leaves.
