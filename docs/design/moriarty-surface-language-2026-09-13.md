# A high-level surface language for Moriarty

Design proposal, 2026-09-13. Grounded in a survey of four ZK language designs
(Mina o1js, Starknet Cairo, Aztec Noir, ZKsync) and in measurements of the
current Moriarty source.

## 1. The problem, measured

`spec/successor/examples/loan-lifecycle.mori` is 178 lines:

| Category | Lines | Share |
| --- | ---: | ---: |
| `emit` field restatements | 80 | 45% |
| `record` declarations | 46 | 26% |
| `ensures` clauses | 39 | 22% |
| actual logic (`let`, `next.`, `requires`) | 13 | 7% |

Seven percent of the file is the thing the author meant to say. The rest is
ceremony the compiler already has enough information to write itself.

Three specific taxes:

**The frame problem.** Most `ensures` lines state what does *not* change:
`ensures post.paid == pre.paid`. Every action that touches one obligation must
hand-write the non-effect on every balance, allowance and obligation it did not
touch. This is quadratic in parties and obligations.

**Double declaration.** Every operation needs a `record XFields { ... }` and then
an `operation X: XFields`. The record exists only to be named once.

**Untyped identity.** `"Lender"`, `"Borrower"`, `"Loan1"` are `Text`. Nothing
stops `emit Transfer { from: "Lendor", ... }`. A typo is a different party.

## 2. What the survey says to steal

**From Cairo: mutability as a type, not an annotation.** Cairo has no `#[view]`.
Read-only is `self: @ContractState`, mutable is `ref self: ContractState`, and
the wrapper generator reads the signature. The annotation cannot drift from the
body because there is no annotation. Moriarty's `pre.`/`next.` split is already
this idea; keep it and extend it.

**From Cairo: auditable desugaring.** Cairo pins *generated source* in golden
test data, so the expansion of every attribute is reviewable. Moriarty's entire
culture is hash-bound evidence. Sugar that cannot be diffed is sugar that cannot
be audited, and would be rejected here on principle.

**From Noir: the hint/verify triad.** Calling an `unconstrained fn` requires an
`unsafe { }` block, the parser *warns if there is no `// Safety:` comment*, and a
dataflow pass checks that every returned value is later re-constrained against an
argument or constant. Three independent mechanisms for one hazard.

**From Noir: private by default, with the opt-out inline in the signature.**
One keyword, at the boundary, with a hard error if omitted where it matters.

**From o1js: harvest annotations that already exist.** `@method` reads
TypeScript's `design:paramtypes` reflection metadata, so the parameter types the
author already wrote *become* the circuit's public input signature. No schema
file, no second declaration. It then silently injects the contract address and
token id as witness arguments.

**From ZKsync, inverted.** Matter Labs kept Solidity's syntax while changing
deployment, addressing, nonces, value transfer and `tx.origin`. Developers get
the illusion of portability, which is worse than an obviously foreign language:
the code compiles and the address is wrong. The lesson is to put a semantic
difference in the *type system*, where the compiler enforces it, never in build
flags or sentinel constants where only documentation can.

## 3. What to refuse to copy

**o1js makes the unsound read shorter than the sound one.**

```ts
let x = this.x.get();        // does NOT prove x matches on-chain state
this.x.requireEquals(x);     // omit this line and the read is unconstrained
```

Their own docs carry a "Caution" on `get()`, and the API ships a
`requireNothing()` escape hatch labelled "DANGER ZONE". Their v2 state module has
no `requireEquals` at all, which says they are abandoning the pattern.

Moriarty already wins here: `pre.phase` *is* the precondition-bound read, and
there is no shorter unsound form to reach for. **This property is now a rule: no
sugar may introduce a shorter spelling that is less safe.**

**Noir's visibility does not propagate.** `Visibility` is attached to parameters,
never enters the type, never unifies, and never taints downstream values. It is
an ABI annotation wearing the costume of an information-flow type system. A
financial language wants visibility to propagate through joins with an explicit
declassification operator as the only escape.

**Cairo's redundant coordinated declarations.** Mounting a component requires
`component!(...)`, a `#[substorage(v0)]` field, and an event enum variant — three
declarations the plugin then cross-validates against each other. If the compiler
can check the redundancy, it has enough information to remove it.

## 4. The proposal

### 4.1 Frame inference by default

The single highest-value change. Everything not assigned in an action is
unchanged, and the compiler emits the frame conditions. `ensures` is reserved for
what the author wants to assert *in addition*.

```
action accrue(at: Time) {
  Loan1.accrue(at);
}
```

The compiler emits, and the evidence record retains, the full frame: `paid`
unchanged, every untouched balance and allowance unchanged, every other
obligation unchanged. Authors stop writing 39 lines to say "nothing else moved",
and reviewers still get all 39 in the generated artifact.

An author who wants to *deny* the frame says so explicitly:

```
action settle() modifies balances, obligations { ... }
```

### 4.2 Parties, assets and obligations are kinds, not Text

```
party Lender;
party Borrower;
asset Cash;
obligation Loan1 : Loan;
```

A misspelled party is now a compile error rather than a different party. This
also gives the compiler the domain it needs to generate frame conditions in 4.1.

### 4.3 Flow syntax that makes the payer mandatory

The independent audit of task 5.1a found that the lender capability assertion
proves the lender *knows the secret* but does not make the lender the *funder*:
both value legs draw on the submitting transaction's offer, and nothing debits
the borrower. Amounts are constrained; party attribution is not.

A surface language can close that by construction, because a flow with no payer
is not writable:

```
Lender pays 100 Cash to Borrower;
```

desugars to the `Transfer` emit *and* the payer binding, and there is no spelling
of a transfer that omits either side. This is the ZKsync lesson applied: put the
obligation in the grammar, not in a convention.

### 4.4 One declaration per operation

```
operation Transfer {
  id: Text;
  from: Party;
  to: Party;
  asset: Asset = Cash;      // default: omit at the emit site
  amount: Amount;
}
```

The separate `record TransferFields` disappears. Fields with defaults need not be
restated on every emit, which is most of the 80 emit lines.

### 4.5 Unit literals

`100 Cash` replaces `quantity<Units<Cash,1>,0>(100)`, and `10% per 60s` replaces
the five-field `AccrualTermsFields` record. Dimensional analysis stays: adding
`Cash` to `Shares` remains a type error, and the elaborated form keeps the full
unit vector.

### 4.6 Profile evolution by extension, not by copy

`financial-lifecycle-v2.ts` is 1,994 lines, of which 1,959 are byte-identical to
v1 — a 98% copy. In the same directory, `financial-expression-v2.ts` and `-v3.ts`
are 5-line re-export shims. The cheap idiom already existed and was walked past.

```
profile 6 extends 5 {
  + operation Fee { ... }
  + state authority: Authority;
}
```

A new profile declares its delta. The compiler derives the full profile and can
mechanically answer "what changed between 5 and 6", which today requires diffing
two near-identical files.

## 5. Before and after

Current, 40 lines for the origination action. Proposed:

```
action originate() {
  Lender pays 100 Cash to Borrower;
  Loan1 = originate 100 Cash from Lender to Borrower
           at 10% per 60s from t0
           cap 110 Cash;
}
```

Everything else — the `Transfer` emit with its six fields, the `Originate` emit
with its twelve, the conversion and accrual-terms records, the phase increment,
and thirteen `ensures` lines — is generated. The generated form is what gets
hashed, reviewed and admitted, exactly as today.

## 6. Non-negotiables

1. **Desugaring is auditable.** Every construct has a golden test pinning its
   expansion. The elaborated form, not the surface form, remains the artifact of
   record for hashing and admission.
2. **No shorter unsafe spelling.** Sugar may never introduce a form that is both
   shorter and less sound than what it replaces.
3. **Off-circuit computation carries Noir's triad.** Any value computed outside
   the constraint system needs a syntactic marker, a mandatory justification, and
   a dataflow check that it was re-constrained.
4. **Semantic differences live in the type system**, never in flags or
   conventions.
5. **The frame is generated, not assumed.** Inference removes the author's
   obligation to write frame conditions; it does not remove the conditions.

## 7. Open questions

- Does visibility need to propagate, and what is the declassification operator?
  Noir's flat lattice is the wrong model, but the right one is not yet chosen.
- How does frame inference interact with the on-ledger gap? Allowances and the
  four replay registers currently have no on-ledger counterpart, so a generated
  frame condition over them is unenforceable at the ledger boundary.
- Should the surface language be parsed to the existing Core, or should Core gain
  a typed party/obligation layer first? The former ships sooner; the latter stops
  the string-typed identity problem at its root.
