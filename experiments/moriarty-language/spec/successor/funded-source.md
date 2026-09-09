# Bounded funded source reference

Repository implementation profile: `moriarty-funded-source/0`. This slice connects
`.mori` syntax, typed inspectable Core, and the retained funded-repayment kernel.
It produces local `Prepared` candidates from supplied state. The state is an
unauthenticated projection; there is no signature, proof, ledger acceptance or
source/Core/K correspondence claim. SP02/SP03 and the successor freeze remain open.

Run from the repository root:

```sh
node experiments/moriarty-language/src/successor/simulate-cli.ts simulate \
  experiments/moriarty-language/spec/successor/examples/funded-partial-payment.mori \
  experiments/moriarty-language/spec/successor/examples/funded-partial-payment.invocation.json
```

The CLI writes one JSON result to stdout and exits 0 for `Prepared`, 1 for
`Rejected`, including file, encoding and usage errors. It reads only bounded
regular files; each source and invocation is limited to 65,536 UTF-8 bytes.

## Interfaces and trust boundary

- `elaborateSuccessorSource(source: string): FundedCore` in `elaborate.ts` parses
  the complete source with the existing parser and typechecks every action. It
  throws `SuccessorSyntaxError` or `FundedSourceError`, both carrying a stable
  `code`. The deterministic Core contains only JSON data, including explicit
  types, literal/parameter expressions and ordered emissions.
- `prepareSuccessor(source: string, invocationJson: string): RepaymentResult` in
  `evaluate.ts` recompiles source on every call. It returns the retained kernel's
  `Prepared` or `Rejected` shape. Rejection never contains `post` or `effects`.
- Core has no execution or deserialization API. Serialized or mutated Core cannot
  be passed to preparation as executable input. Live object arguments are rejected
  without coercion or property access. All admitted runtime objects originate in
  the bounded JSON parser.

The invocation has exactly `schemaVersion`, `action`, `arguments`, and `state`.
`schemaVersion` is `moriarty-funded-source/0`; `action` selects a declared action.
`arguments` has exactly its parameter names. Each argument is `{type, value}`.
Parameterized types are `{kind: "Debt" | "Amount", name: "DeclaredName"}`;
other types are `{kind: "UInt" | "Bool" | "Party" | "Asset" | "TransferId" |
"AllocationId" | "ObligationId"}`. Numeric values are canonical unsigned decimal
strings in UInt128; Bool values are JSON booleans. Party and Asset argument values
must name declarations. IDs use the retained ASCII identifier bound.

`state` supplies the complete existing `RepaymentState`, including all balances,
allowances, obligations, conversions, work, reserve, and used-ID tombstones.
The invocation is parsed with standard `JSON.parse`, which keeps the last value
for duplicate object keys. The adapter reserializes the complete parsed state
projection with the lowered actions before the retained kernel's closed-schema validation. Preservation applies
to that parsed projection; the original invocation bytes are not preserved or checked
for canonical lexical form. Source declarations do not initialize, replace, filter
or authenticate this state.

## Supported source

The source header remains `profile "moriarty-successor-syntax/0";` because the
unchanged parser owns that syntax profile. Selecting this preparation API selects
the separate semantic profile. Syntax acceptance alone does not imply execution.

Supported declarations are `unit U;`, `party P;`, `asset A: Asset<U>;`, and actions
with typed parameters and `emit` statements. Units have a separate namespace from
terms, allowing denomination `Cash` and settlement asset `Cash` to share a spelling.
Duplicate units, duplicate terms (parties/assets/actions), duplicate parameters,
parameter shadowing of terms, and reserved built-in term names reject. Asset type
arguments must name declared units. Core preserves each asset's declared unit as
metadata; it does not infer or overwrite obligation conversion terms from it.

Parameter types are `Debt<U>`, `Amount<A>`, `UInt`, `Bool`, `Party`, `Asset`,
`TransferId`, `AllocationId`, and `ObligationId`. Debt and Amount are distinct even
when their parameter names match. Supported expressions are parameter references,
declared party/asset references, UInt128 integer literals, boolean literals,
`debt(INTEGER, U)`, `amount(INTEGER, A)`, and the ID constructors
`TransferId("ID")`, `AllocationId("ID")`, `ObligationId("ID")`.
Bare strings, arbitrary calls and all operators reject. Constructor numeric
arguments must be integer literals; there is no arithmetic or numeric coercion.

Effect fields are closed, mandatory, and unique:

| Emission | Source fields and types |
| --- | --- |
| Transfer | `id: TransferId`, `from: Party`, `to: Party`, `settlementAsset: Asset`, `amount: Amount<A>` |
| Repay | `allocationId: AllocationId`, `transferId: TransferId`, `obligationId: ObligationId`, `payer: Party`, `nominalAmount: Debt<U>` |

`settlementAsset` lowers to the kernel's Transfer `asset` field. The source spelling
avoids the parser's reserved `asset` keyword. Effect type arguments are unsupported.
Field order is canonicalized in Core; emission order remains source order. Up to
128 emissions per action are supported, within the retained parser's other bounds.
An empty action reaches kernel `SCHEMA` rejection. Repay must consume a prior
explicit Transfer in the same action; no Transfer is synthesized. Transfer-only
actions can prepare cash movement with no debt discharge, as permitted by the
retained kernel.

State and const declarations, `next`, `let`, `requires`, `ensures`, projections,
unary/binary/comparison expressions, and all other types/declarations/effects reject
explicitly. A naked `next.principal` subtraction can never become a funded payment.
This profile does not implement the existing syntax-only partial-payment example.

## Financial behavior and rejection precedence

Parsing and static checking precede invocation shape and typed-argument checking.
During lowering, each Transfer's Amount type name must equal its resolved settlement
asset, or preparation returns `SETTLEMENT_UNIT` at that emission index. The runtime
then asks the retained kernel to validate and tentatively execute the complete state
and lowered action list. Kernel rejections retain their exact code and actionIndex.
Only after kernel success does the adapter bind every Repay Debt type name to the
referenced obligation's validated denomination, which the kernel preserves. A mismatch
returns `NOMINAL_UNIT` at that emission index, discarding the tentative candidate.
Thus kernel financial failures take precedence over nominal-unit mismatch, while
settlement-unit mismatch precedes kernel validation. No tentative state or effect is
externally exposed on any failure. Syntax, invocation and static errors have null
actionIndex; `SOURCE_INTERNAL` is the fail-closed unexpected-error fallback.

The unchanged kernel owns funding consumption, payment allocation, nominal conversion,
UInt128 intermediate overflow, insufficient cash, gross allowance, replay history and
financial work. One Transfer and one Repay cost two units of ordinary work; closure
reserve is preserved. Parsing/static host work is bounded separately and is not a
financial work charge. Runtime binding checks do not authorize new financial actions.

The supplied example pays 30 against principal 100: principal becomes 70, payer cash
becomes 70, creditor cash becomes 30, and outstanding residual duty remains. Independent
complete expected results also exercise interest-first and principal-first allocation,
full repayment, crossing the interest boundary, and cash 40/debt 30. The latter debits
cash/allowance by 40 and discharges only 30 nominal debt; unused transfer funding is
not refunded. The source tests additionally cover changed conversion, unrelated duties,
replay tombstones, late rejection, argument perturbation and hostile input boundaries.
These are finite local observations, not proof of the general successor language.
