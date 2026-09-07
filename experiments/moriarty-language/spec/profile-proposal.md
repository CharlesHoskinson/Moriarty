# MC01 first language profile proposal

Status: S2 proposal, not frozen, implemented, or independently approved.
Author: root architect. Date: 2026-09-07 UTC.
Profile identifier: `moriarty-bounded-atomic/1` (proposed).

This proposal resolves first-profile design choices for MC01 1.1 and D.1/D.2.
It does not close those tasks. The complete target crosswalk and both result audits remain required.
The controlling acceptance remains the MC01 OpenSpec package at commit `2aa863565fa1faae759f5bef47d2a786534fd627`.

## Developer surface

Use a generic bounded transition language for both ACTUS events and DeFi actions.
Financial packages supply checked terms, initial states, event rules, and named properties.
Package names do not create new trusted primitives or prove conformance.

The proposed source has explicit version, state, action, and effect declarations.
Identifiers use ASCII letters, digits, and underscore, starting with a letter, with at most 64 characters.
Reject reserved prototype names and duplicate declarations in every namespace.
External labels remain typed text values, never object keys or implicit identifiers.

Illustrative transition fragment, not an executable or complete agreement:

```text
action accrue_period() {
  guard state.cursor == uint(0), "period already accrued";
  let numerator = state.notional * uint(8) * uint(31);
  let interest = floor_div(numerator, uint(100) * uint(365));
  set principal_due = amount(500000000, USD_micro);
  set interest_due = interest;
  set notional = state.notional - state.principal_due;
  set cursor = uint(1);
  emit DueCreated {
    event: text("lam01:period1:PR"), amount: state.principal_due
  };
}
```

The complete loan must also emit the interest due and bind each due to its debtor, creditor, denomination, and settlement policy.
The fragment only illustrates syntax and sequential state reads. It is not a valid fixture by itself.
In particular, this proposal does not use a package-selection form as a substitute for an authoring language.

Proposed statement forms are `guard`, single-definition `let`, `set`, and `emit`.
Proposed expression forms are typed literals, state, argument, observation, constant and local reads, remaining lifetime, comparisons, Boolean operators, multiplication, addition, subtraction, and `floor_div(n,d)`.
There is no slash operator or `floor(e)` expression. Parentheses determine grouping. `not` binds first, then multiplication, then addition/subtraction, comparisons, `and`, and `or`; `floor_div` is a primary call.
Reject chained comparisons. Boolean `and` and `or` short-circuit left to right.
The accompanying grammar.ebnf proposes the EBNF and UTF-8 byte source spans. It requires review before parser implementation.

There are no user-defined functions, recursion, loops, dynamic evaluation, or unbounded collections in this profile.
Every action is a finite straight-line list. Checked package expansion must satisfy the same final Core bounds.
Unsupported operations produce source-located diagnostics before elaboration.

## Types, units, and numeric behavior

Separate `UInt128`, `Bool`, `Text`, and nominal `Amount<Unit>` in the typed authoring AST.
The initial Core stores `UInt128` and `String`. Bool is limited to expressions and locals.
Amounts lower to UInt128 only with a retained checked unit map bound into the semantic manifest.
Reject Bool state fields until a reviewed Core extension supplies their representation.

Use `uint(123)`, `text("123")`, and `amount(123, USD_micro)` as distinct literals.
Numeric text never becomes an integer by inspecting its contents.
Integer tokens are canonical unsigned decimal, with no sign, exponent, separator, or leading zero except `0`.
Reject values above `2^128 - 1`.

Addition, subtraction, ordering, and equality require equal operand types and equal nominal units.
Multiplication adds unit exponents. Division subtracts them. UInt128 has the empty unit vector.
Compound quantities are expression-only, with at most eight unit components and each exponent between -16 and 16.
Assignment requires the resulting vector to match the declared scalar or nominal amount type exactly.
This permits the swap numerator A*B divided by denominator A to produce B without unchecked unit erasure.
Ordered comparisons accept numeric operands only. Equality also accepts equal Text or Bool types.
Rational rates use explicit numerator and denominator fields. Unit inference checks their use.
Division by zero, underflow, or any intermediate overflow rejects the entire action.
Every intermediate uses the same 128-bit limit, including products before division.
No algebraic rearrangement may change overflow, rounding, or short-circuit behavior.

Nonnegative integer division rounds down at each explicit division node.
Loan interest uses one final division after the checked products.
Swap pricing uses one final division after the checked numerator and denominator calculations.
There is no hidden floating-point conversion or global financial tolerance.
Each financial field records unit, derivation, rounding point, remainder disposition, and comparison policy.

`USD_micro` means a millionth of the reference USD denomination.
It does not identify a ledger token. Settlement requires a separately bound asset and conversion policy.
Pool asset units remain distinct, even if both tokens use the same display symbol.
Missing settlement mappings reject settlement construction. Local arithmetic can still display a due.

Signed financial values require a future version with a defined range, direction, and rounding for negative values.
Calendar operations require a future version with explicit calculation dates, payment dates, and conventions.
Quantity-times-price and bounded convergence require explicit typing and resource extensions.
None may reinterpret existing unsigned bytes or silently fall back to host arithmetic.

## Canonical representations and finite bounds

Use an explicit schema version and semantic profile on every public wire envelope.
Canonical keys use the same restricted ASCII identifier alphabet as source identifiers.
External identifiers and event labels are values with separate validation rules.
Use lexicographic ASCII key order, UTF-8 encoding, and JSON string escaping with a single canonical spelling.
Reject duplicate keys before object construction, noncanonical wire bytes, lone surrogates, unknown fields, and numeric JSON tokens.
Do not normalize text silently. Integers on the wire are canonical decimal strings.
Booleans are JSON booleans only where the schema permits them.

The normative limits are in `bounds.json`. Source, decoded AST, full program manifest,
compact signing envelope, and result are distinct encodings and all applicable limits
hold simultaneously:

| Resource | Bound |
|---|---:|
| Source, AST, manifest, signing envelope, or result | 65,536 UTF-8 bytes each |
| AST and full manifest decoded depth, root at zero | 40 |
| Compact signing and result decoded depth, root at zero | 16 |
| Nodes in each decoded object | 8,192 |
| Keys per record | 64 |
| Language Text | 256 UTF-8 bytes |
| Canonical-object text | 4,096 UTF-8 bytes and 4,096 JavaScript code units |
| Each identifier | 64 ASCII characters |
| Entrypoints, state fields, or action fields | 64 each |
| Instructions or locals per entrypoint | 64 each |
| Expression depth, root at one | 16 |
| Total expression nodes per entrypoint | 256 |
| Effects per entrypoint | 16 |

These are simultaneous limits. A value satisfying one bound may still fail another.
The full program-hash preimage contains Core instructions and uses the manifest depth-40
limit. The depth-16 signing object contains only a compact program reference plus input
and authority data; it never embeds expressions. Both use the exact
`moriarty-canonical-json/1` rules in `typed-schemas.md`. A program is rejected if its
actual aggregate bytes or nodes fail even when every per-action count fits.
Test UTF-8 byte counts independently from JavaScript string lengths.

Bind the profile's total lifecycle allowance, horizon, and initial state at authenticated genesis.
Every accepted action consumes one allowance, even when it emits no transfer.
No resume, redeployment, split, or join may reset an existing instance's allowance.
The first profile excludes split/join and Pending execution with explicit diagnostics.
Later composition must define conservation of residual budgets before it becomes supported.

## Transition judgment and correctness scope

The proposed interface remains:

```text
parse(source) -> Result<AgreementAST, Diagnostic[]>
check(ast, profile) -> Result<TypedAgreement, Diagnostic[]>
elaborate(typed) -> CoreProgram
evaluate(program, state, action, authority, observations)
  -> Rejected | Complete | Pending
```

`Rejected` returns diagnostics and exposes no committed partial state or effects.
`Complete` means the selected atomic plan completed under its declared outcome policy.
It does not mean the entire agreement or all future financial obligations are discharged.
The result must carry separate agreement status and all residual obligations.
The first profile never produces `Pending`. Requests requiring Pending reject as unsupported.

Guards and expressions read the current working state in statement order.
Effects capture values at the emit position. Locals cannot reference future locals.
After successful evaluation, increment revision and decrement remaining allowance exactly once.
The transition receipt includes all writes, effects, obligations, resource counts, and authority consumption.
Expected effects include fees and every permitted intermediate recipient.
Gross debit caps count fees and do not net refunds. Outcome credits are net of fees.

Keep exact-plan authority and outcome-intent authority as distinct tagged modes.
Preserve historical `IntentEffects` signatures in their original version domain.
Use `IntentRefinement` only with the new explicit mode and semantic domain.
Bind program/profile version, instance, principal, nonce, validity interval, predecessors, and all required claims.
Do not silently relabel old signed statements.

Local evaluation does not establish durable replay protection, oracle truth, or actual proof acceptance.
MC04/MC05 must enforce these checks at the ledger acceptance boundary.
Every accepted proof must discharge its precise contract, intent, transition, and history predicates.
This proposal supplies no native proof, universal correctness theorem, or production acceptance path.

## Required developer examples and rejection controls

The loan example is the first LAM period, not a complete LAM contract.
Initial notional is 5,000,000,000 micro-USD. Principal payment is 500,000,000.
Interest is `floor(5,000,000,000 * 8 * 31 / (100 * 365)) = 33,972,602`.
Settlement is 533,972,602. Remaining contractual notional is 4,500,000,000 after the sample closes.
Expose that remainder explicitly. Never display the sample's `closed` flag as debt-free status.
Retain both PR and IP obligation identities despite their combined sample transition.
The floor-rounded sample is not exact conformance to the upstream finite-decimal cashflow.

The swap example uses reserves 1,000,000 and 2,000,000, input 10,000, and price multiplier 997/1000.
Expected output is 19,743. New reserves are 1,010,000 and 1,980,257.
Pricing retains the fee in the input reserve. It does not emit an extra fee transfer.
The provider's closure action returns both remaining reserves and cannot be starved by the last permitted swap.

Implementation must compare every field against the independent retained reference calculations.
Required negative examples include wrong units, denominator zero, overflowing numerator, and numeric-looking text.
Also reject wrong settlement asset/recipient, wrong event order, insufficient minOut, extra effects, and exhausted lifetime.
Include a positive boundary example beside every capacity or authority rejection.

## Target extension obligations

| Decision | MC01 foundation | MC07 obligation |
|---|---|---|
| DS-01 | Distinct calculation/payment dates and versioned event order | Exact business-day semantics and pam08/pam09 |
| DS-02 | Lossless convention identifiers | CSMP/SCMP source discrepancy |
| DS-03 | Explicit numeric and bounded-iteration extension boundary | Independently derive and check ANN initialization |
| DS-04 | Defined signed-value and quantity/price extension boundary | COM cashflow signs and quantity rule |
| DS-05 | Finite lifetime, horizon, observations, and closure reserve | CLM call/redemption behavior |
| DS-06 | Stable event IDs with separate external labels | FXOUT settlement vector and labels |
| DS-07 | Separate episode and full-contract scope | FUTUR margin behavior and taxonomy discrepancy |

The two examples do not establish coverage of the complete target corpus.
A complete 32-row ACTUS requirements and 72-row DeFi crosswalk remains required before profile freeze.
Every row must state supported behavior, rejected behavior, required extension, source locator, and evidence status.
All 277 ACTUS fixtures and 72 DeFi rows remain required for MC07 acceptance.

## Initial Compact mapping decision

Preserve checked 128-bit arithmetic and exact evaluation order during lowering.
Insert range, underflow, and divisor checks where the target does not enforce the source rule directly.
Maintain source maps for guards, assignments, and effects.
Reject unsupported nominal-unit erasure, wire encodings, dynamic effects, and unbound settlement policies.
Require generated positive loan/swap examples and mutation tests before calling the mapping implemented.
Successful compilation alone is not compiler-to-ledger correspondence.

## Version changes and next gate

Any change to numeric meaning, codec, lifetime, effects, or claim tags creates a new semantic version.
Recompute program identifiers, signature domains, proofs, certificates, theorem instances, and audits affected by that change.
Historical artifacts remain scoped to their original bytes and semantics.
Do not reuse them as evidence for a changed relation.

Next: review the proposed EBNF, finish typed schemas and the full target crosswalk, and independently review this decision packet.
Then add failing frontend and full-field trace tests before implementing the parser and elaborator.
Worker dispatch remains stopped until the execution contract can enforce the approved budget predicates.

## Source basis

- `docs/superpowers/specs/2026-09-06-moriarty-unified-semantics-design.md`.
- `docs/research/2026-09-06-intents-report-integration.md`.
- `evidence/moriarty-completion-program-2026-09-07/resume-20260907T043818Z/source-intake.md`.
- `evidence/moriarty-completion-program-2026-09-07/resume-20260907T043818Z/reference-calculations.json`.
- `evidence/moriarty-design-sprint-2026-09-06/actus-32-requirements.csv`.
- `evidence/moriarty-design-sprint-2026-09-06/defi-72-requirements.csv`.

These source paths are relative to the repository root. This entire document is a proposal unless explicitly described as a retained observation.
