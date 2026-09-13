# Surface syntax and the type surface

Design slice, 2026-09-13. Companion to
`moriarty-surface-language-2026-09-13.md` (frame inference, flows, profile
deltas) and to the three jet-discipline slices in
`deliverables/jet-discipline-2026-09-13/`. This slice decides how a program is
spelled and what the types say. Every construct carries a one-line note naming
the practice it serves; a construct that could not be given one was cut.

The practices I kept returning to, all older than any language mentioned in the
brief: say a thing once; make illegal states unrepresentable; a literal adapts to
its context, a type never adapts to a literal; precision of money belongs to the
currency, not the expression; overflow is an error, not a value; rounding is a
written decision; a partial operation states at the site what happens when it
fails; case analysis is exhaustive; read-only is a property of a scope, not a
promise in a comment; the contract and the code are checked against each other.

## 1. Declarations

```
agreement LifecycleLoan {
  party Lender, Borrower;
  asset Cash scale 0;
  obligation Loan1: Cash;

  operation Transfer { id: Id, from: Party, to: Party,
                       settlementAsset: Asset = Cash, transferAmount: Cash }

  state phase: u128;
  state paid: Cash;
}
```

**`party`, `asset`, `obligation` are nominal.** `Party` is a closed enumeration
whose members are the declared parties; the same for `Asset` and `Obligation`.
They are identifiers, not strings, so a typo is unbound, not a new party:

```
emit Transfer { from: Lendor, ... }
      error: unbound identifier `Lendor`; parties in scope: Lender, Borrower
```

Serves: make illegal states unrepresentable.

**An obligation carries its denomination in its type.** `obligation Loan1: Cash`
means every projection of `Loan1` is already a `Qty<Cash>`. The runtime
`NOMINAL_UNIT` check (financial slice, `financial-expression-v1.ts:423-445`)
becomes a static fact and the `<Cash>` on `outstanding<Cash>("Loan1")`
disappears because it was never information: the obligation knew.

Serves: say a thing once. The denomination is declared where the obligation is,
and nowhere else.

**One declaration per operation.** `operation Transfer { ... }` is the record.
The data slice establishes field names are a codec concern; the operation
declaration *is* that codec, so its names are the wire names, unchanged. A field
may carry a constant default; an emit that omits it gets the default. `record`
remains for values that are not operations.

Serves: say a thing once. `record XFields` plus `operation X: XFields` was one
concept spelled twice.

**`state paid: Cash`, not `UInt128`.** The original declared `paid` as a bare
integer and converted at every transfer. The type was a lie about what the field
holds.

## 2. Types and literals

### The type grammar a developer sees

```
Cash              ledger amount of asset Cash          (Amount, unsigned 128)
Qty<Cash>         signed quantity, unit vector Cash^1   (Quantity, signed 128)
Qty<Cash/Shares>  unit vector Cash^1 Shares^-1
Qty<Cash^2>       unit vector Cash^2
Price<Cash/Shares>, Rate, Net<Cash>, Scaled<Cash>
u64 u128 u256 i128 i256 Bool Text Id Time
Option<T>  T[n]  (collection, capacity n)
```

The elaborated form is unchanged: `Qty<Cash/Shares>` elaborates to
`['Quantity', [['Cash','1'],['Shares','-1']], scale]` with the full sorted unit
vector, and `Cash + Shares` fails `TYPE_MISMATCH` exactly as it does today.
The surface only changes how the vector is spelled, not what it is.

Serves: make illegal states unrepresentable. Dimensional analysis predates
computing; adding money to shares is the archetypal financial bug.

### Scale belongs to the asset

`asset Cash scale 0;` (or `scale 2` for a currency with cents) fixes the scale of
every `Cash` and `Qty<Cash>` in the agreement. The literal `100 Cash` elaborates
to mantissa `100` at scale 0; under `scale 2` it elaborates to mantissa `10000`,
and `100.005 Cash` is a compile error, not a rounding. The scale parameter does
not disappear from the elaborated type; it stops being something an author
writes per literal.

Serves: precision of money belongs to the currency. ISO 4217 has said so since
1978; `Quantity<Units<Cash,1>,0>` made every author restate it.

### Literals are one construction

Nine literal constructors are one `Lit(type, value)`; the surface reflects that.
A literal is a number with an optional unit, typed from context.

```
100 Cash            Cash or Qty<Cash>, decided by the expected type
0 Cash              same
1/10                a ratio, for accrual terms
true, "text", 60
```

`100 Cash` where a `Cash` (Amount) is expected elaborates to
`Lit(['Amount','Cash'], '100')`; where a `Qty<Cash>` is expected, to
`Lit(['Quantity',[['Cash','1']],'0'], '100')`. With no expected type it defaults
to `Cash`, the ledger's own type, the way Rust defaults an unsuffixed integer.

Serves: the literal adapts to its context; the type never adapts to the literal.

### Range classes: visible as types, never as operations

Six range classes exist (`numericFits`, `financial-expression-types-v1.ts:152-157`):
u64, u128, u256, i128, i256 and the asymmetric `Net`. The developer sees them
only as the result types of expressions: `Cash * Cash` is `Product<Cash,Cash>`
(u256), `Cash * Rate` is `Scaled<Cash>` (i256), `Cash` minus `Cash` as a net
position is `Net<Cash>`. There is no `add_u128`; the author writes `+` and the
result type selects the class, exactly as the static rule does today. Overflow
rejects the transition with `ARITH_RANGE`; there is no wrapping form.

Serves: overflow is an error, not a value (Ada, 1983). The class is part of the
answer, not part of the question.

### Rounding is a written decision

`TYPE_SCALE_DIVISOR` (`financial-expression-v1.ts:347`) today requires the author
to divide a `Scaled<Cash>` by a syntactic literal `10^scale`, and the computation
slice notes this rule is hostile to any jet. The surface never exposes the
divisor:

```
let due = (amount * price).floor();    // Scaled<Cash> -> Cash, FloorDiv by 10^scale
let due = (amount * price).ceil();     // CeilDiv
```

There is no other way to leave `Scaled<Cash>`, so the elaborator always emits
the literal the rule demands, and the rule can later become a property of the
divisor's value without any surface change.

Serves: rounding is a written decision. Every conversion back to money names its
direction.

## 3. Reads and writes

```
action repay(nominal: Qty<Cash>, transfer: Id, allocation: Id) {
  require nominal > 0 Cash;
  let payment = nominal.magnitude;          // Cash
  next.phase = pre.phase + 1;
  next.paid  = pre.paid + payment;
  emit Transfer { id: transfer, from: Borrower, to: Lender, transferAmount: payment };
} ensure {
  post.paid == pre.paid + payment;
  post.Loan1.principal == 80 Cash;
}
```

Three namespaces, three capabilities, no overlap:

- `pre.` is readable everywhere and never writable.
- `next.` is writable in the body and never readable. `let x = next.phase` is a
  type error, as `TYPE_NEXT_READ` already makes it; the surface keeps that.
- `post.` exists only inside the `ensure` block. It is not that `post` reads are
  *forbidden* in the body; the body's environment has no `post` component.

This is Cairo's idea applied: the `ensure` block is the type. Today the gate is a
placement rule (`TYPE_POST_SCOPE`) over a flat statement list; the data slice
shows the honest encoding is two environment types, `Input` for the body and
`Input × Output` for the suffix. The block makes that visible, and it cannot
mutate because it has no `next`.

Serves: read-only is a property of a scope, not a promise. And the survey's rule
holds by construction: every read is the precondition-bound read, and there is no
shorter spelling for a `post` read outside its block because there is no
spelling.

**Financial state reads** drop the `<Cash>` and the string:

```
pre.Loan1.outstanding              Qty<Cash>, from the obligation's type
pre.balance[Lender, Cash]          Cash
post.allowance[Borrower, Cash].spent
```

`balance[party, asset]` is spelled as a lookup because the financial slice found
party-asset lookup to be one of the two irreducible primitives.

**SimplicityHL's namespace idea applies to observations.** Values that arrive
from outside the ledger (`ReadObs`) are spelled `obs.time`, never bare. Arguments
are bare because the caller supplied them under the action's signature; an
observation is a claim about the world and reads as one.

Serves: provenance in the token. A value that could not have been checked by the
ledger should not be spelled like one that was.

## 4. Partiality

Four things fail at runtime: `ProjectSome`, `ProjectVariant`, `ProjectIndex`,
and narrowing `ConvertUInt` (the data slice counts three projections; the
narrowing conversion is the fourth partial and I treat it the same way). The
brief offers Rust's `Option`/`?`/`match` or TypeScript's `?.` and narrowing.

I refuse `?`. It exists to hand the failure to a caller who decides. An action
has no caller in that sense: it either commits or the transition is rejected,
atomically, with a code the ledger records. There is no one to propagate to.

I refuse `?.` more firmly. Optional chaining turns absence into further absence
and keeps going; a missing counterparty must stop the transaction, not produce
`undefined` three lines later.

What survives the timeless-practice test is: a partial operation states at the
site what happens when it fails, and case analysis over a domain value is
exhaustive.

```
match pre.counterparty {
  Some(p) => ...,
  None    => reject NoCounterparty,
}
let p = pre.counterparty else reject NoCounterparty;   // Rust let-else
let n = wide as u64 else reject Overflow;              // narrowing
let x = xs[i];                                         // rejects INDEX_RANGE
let x = xs[i] else reject BadIndex;                    // renamed
```

Two rules, one distinction. `Option` and `Variant` must be matched or given an
`else reject`, because `None` and a wrong tag are domain cases the author may
have simply forgotten. Index and narrowing may be written bare, because
out-of-range is a precondition violation the ledger already names; `else
reject` lets the author name it better. `reject Code` is a diverging expression;
the code is commitment metadata, as `GUARD_FAILED` is today.

Serves: fail at the point of the assumption, with a name.

## 5. What is genuinely new

Two things. Parties as nominal types are not on the list: that is an enum.

**Asset-scoped precision with context-typed literals.** F# has units of measure
but not currency scale; Rust and TypeScript have neither. `asset Cash scale 2`
plus `100 Cash` resolving to `Amount` or `Quantity` by expected type lets the
elaborated `Quantity<Units<Cash,1>,2>` survive intact while never being written.
It removes the largest source of noise in the measured file without deleting a
type parameter from the core.

**Phase-typed state.** `pre`/`next`/`post` as three environments of which the
`ensure` block sees a different type than the body. Cairo has one axis
(read/write); Moriarty has three views with distinct capabilities, and the data
slice's analysis shows the third view is a *type* difference, not a scope rule.
No mainstream language ships write-only namespaces or a suffix scope with a
richer environment than its prefix. It earns its place because it is the
soundness property, made unforgeable by spelling.

## 6. Where I disagree with the brief

**"Six range classes exist; decide whether the developer sees them."** The
framing assumes a choice. The classes are result types; a developer who writes
`Cash * Rate` already sees `Scaled<Cash>` in any hover or error. The real
decision is that they never appear as operation selectors, and that the surface
should not paper over the K/TypeScript disagreement about which classes exist
(computation slice: K has three, TypeScript six). I name six because the
TypeScript is the executing reference; if K is the authority, `u256`, `i256` and
`Net` are provisional and the surface should say so in the profile.

**"Party identity is a candidate for what is new."** It is not new. It is the
1970s practice of an enumerated type, and claiming it as invention would be the
kind of reskin the brief warns against.

## 7. The example, rewritten

The original is 178 lines. This is 72 (measured with `wc -l` on the block, blank
lines included). The 39 `ensures` lines become 14: the frame conditions (`paid`
unchanged, untouched balances) are generated per the companion document's frame
inference, and `outstanding` assertions are dropped because the financial slice
proves `outstanding = principal + accrued` is a maintained invariant. The
remaining assertions are the ones that state a fact about this loan.

```
profile "moriarty-financial-agreement-source/6";

agreement LifecycleLoan {
  party Lender, Borrower;
  asset Cash scale 0;
  obligation Loan1: Cash;

  enum Rounding { none, floor, ceil }
  enum Allocation { AccrualFirst, PrincipalFirst }
  record Conversion { mantissa: u128, scale: u128, rounding: Rounding }
  record AccrualTerms { numerator: u128, denominator: u128, rounding: Rounding,
                        periodSeconds: u64, firstPeriodStart: u64 }

  operation Transfer  { id: Id, from: Party, to: Party, settlementAsset: Asset = Cash, transferAmount: Cash }
  operation Repay     { allocationId: Id, transferId: Id, obligationId: Obligation, payer: Party, nominalAmount: Qty<Cash> }
  operation Originate { obligationId: Obligation, transferId: Id, originationId: Id, debtor: Party, creditor: Party,
                        nominalAmount: Qty<Cash>, denomination: Asset = Cash, settlementAsset: Asset = Cash,
                        conversion: Conversion = Conversion { mantissa: 1, scale: 0, rounding: none },
                        allocationRule: Allocation, accrualTerms: AccrualTerms, nominalLiabilityCap: Qty<Cash> }
  operation Accrue    { accrualId: Id, obligationId: Obligation, periodIndex: u64, observedTime: u64 }

  state phase: u128;
  state paid: Cash;

  action originate(transferId: Id, originationId: Id) {
    next.phase = pre.phase + 1;
    emit Transfer { id: transferId, from: Lender, to: Borrower, transferAmount: 100 Cash };
    emit Originate { obligationId: Loan1, transferId, originationId, debtor: Borrower, creditor: Lender,
                     nominalAmount: 100 Cash, nominalLiabilityCap: 110 Cash, allocationRule: AccrualFirst,
                     accrualTerms: AccrualTerms { numerator: 1, denominator: 10, rounding: floor,
                                                  periodSeconds: 60, firstPeriodStart: 1000 } };
  } ensure {
    post.phase == pre.phase + 1;
    post.Loan1.principal == 100 Cash;   post.Loan1.accrued == 0 Cash;
    post.balance[Lender, Cash] == 0 Cash;   post.balance[Borrower, Cash] == 110 Cash;
    post.allowance[Lender, Cash].spent == 100 Cash;
  }

  action accrue(accrualId: Id, periodIndex: u64, observedTime: u64) {
    next.phase = pre.phase + 1;
    emit Accrue { accrualId, obligationId: Loan1, periodIndex, observedTime };
  } ensure {
    post.phase == pre.phase + 1;
    post.Loan1.principal == 100 Cash;   post.Loan1.accrued == 10 Cash;
  }

  action repay(nominal: Qty<Cash>, transferId: Id, allocationId: Id) {
    require nominal > 0 Cash;
    let payment = nominal.magnitude;
    next.phase = pre.phase + 1;
    next.paid = pre.paid + payment;
    emit Transfer { id: transferId, from: Borrower, to: Lender, transferAmount: payment };
    emit Repay { allocationId, transferId, obligationId: Loan1, payer: Borrower, nominalAmount: nominal };
  } ensure {
    post.paid == pre.paid + payment;
    post.Loan1.principal == 80 Cash;   post.Loan1.accrued == 0 Cash;
    post.balance[Lender, Cash] == 30 Cash;   post.balance[Borrower, Cash] == 80 Cash;
  }

  action settle(transferId: Id, allocationId: Id) {
    let debt = pre.Loan1.outstanding;
    let payment = debt.magnitude;
    require payment > 0 Cash;
    next.phase = pre.phase + 1;
    next.paid = pre.paid + payment;
    emit Transfer { id: transferId, from: Borrower, to: Lender, transferAmount: payment };
    emit Repay { allocationId, transferId, obligationId: Loan1, payer: Borrower, nominalAmount: debt };
  } ensure {
    post.Loan1.outstanding == 0 Cash;
    post.balance[Borrower, Cash] == 0 Cash;   post.balance[Lender, Cash] == 110 Cash;
  }
}
```

Constructs used above: field punning (`transferId,`), say a thing once;
`nominal.magnitude : Cash` on a single-asset `Qty`, the compiler inserts the
no-op ascription `ConstructAmount` is; `require nominal > 0 Cash` for
`is_negative(...) == false` plus `payment > 0`, one guard for one fact. The field
names are the original's because they are the wire codec; the count is not
padded by renaming.

## 8. Grammar sketch

```
agreement   ::= 'profile' STRING ';' 'agreement' IDENT '{' decl* '}'
decl        ::= 'party' IDENT (',' IDENT)* ';'
              | 'asset' IDENT ('scale' INT)? ';'
              | 'obligation' IDENT ':' IDENT ';'
              | 'enum' IDENT '{' IDENT (',' IDENT)* '}'
              | 'record' IDENT '{' field (',' field)* '}'
              | 'operation' IDENT '{' field (',' field)* '}'
              | 'state' IDENT ':' type ';'
              | 'action' IDENT '(' params? ')' block ('ensure' '{' (expr ';')* '}')?
field       ::= IDENT ':' type ('=' expr)?
type        ::= IDENT                                  -- asset (Amount), enum, record
              | 'Qty' '<' units '>' | 'Price' '<' units '>' | 'Scaled' '<' IDENT '>'
              | 'Net' '<' IDENT '>' | 'Option' '<' type '>' | type '[' INT ']'
              | 'u64' | 'u128' | 'u256' | 'i128' | 'i256' | 'Bool' | 'Text' | 'Id' | 'Time'
              | 'Party' | 'Asset' | 'Obligation' | 'Rate'
units       ::= unit (('*' | '/') unit)*
unit        ::= IDENT ('^' INT)?
block       ::= '{' stmt* '}'
stmt        ::= 'let' IDENT '=' expr ('else' 'reject' IDENT)? ';'
              | 'require' expr ';'
              | 'next' '.' IDENT '=' expr ';'
              | 'emit' IDENT '{' (IDENT (':' expr)?) (',' ...)* '}' ';'
expr        ::= literal | IDENT | expr binop expr | 'reject' IDENT
              | view '.' IDENT ('.' IDENT)*                 -- state and obligation reads
              | view '.' ('balance' | 'allowance') '[' expr ',' IDENT ']' ('.' IDENT)?
              | 'obs' '.' IDENT
              | expr '.' IDENT                             -- projection; .magnitude .floor() .ceil()
              | expr '[' expr ']' | expr 'as' type
              | expr 'else' 'reject' IDENT
              | 'match' expr '{' (pattern '=>' expr ',')+ '}'
              | IDENT '{' (IDENT ':' expr) (',' ...)* '}'   -- record literal, type-directed
view        ::= 'pre' | 'post'                              -- 'post' only inside ensure
literal     ::= NUMBER (IDENT)? | INT '/' INT | 'true' | 'false' | STRING
```

`next` is deliberately absent from `expr`: it is a statement head only.

## 9. What the repository does not settle

- Whether a declared obligation is guaranteed present in `pre`. Before
  `originate`, `pre.Loan1.outstanding` fails `MISSING_OBLIGATION` at runtime. I
  treat it as a bare-able partial (section 4) with that code; a stricter design
  would give obligations a lifecycle type, which nothing in the core supports
  today.
- Whether K or TypeScript is the authority on range classes (three versus six).
  The surface names six; if K wins, three of the type names are provisional.
- Whether generic `Lit` should admit derived types (`Product`, `Scaled`, `Net`)
  as literals. This design does not allow it: the only way to obtain a derived
  type is by an operation, which keeps the class algebra honest.
- Whether frame inference lands. The 72-line count assumes it; without it the
  rewrite gains roughly 20 lines of written frame assertions and is still
  about half the original.
