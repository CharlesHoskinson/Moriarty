# S02 authorization interface intake

Status: read-only architectural advice, not an approved implementation or model result.
The observation/authorization design remains authoritative.

An independent architecture agent recommended generic structural records.
`Context[s]` binds complete candidate semantic state, ledger, environment, and
nonrecursive consumption facts. `Proposal[s, a]` additionally binds the full
candidate artifact/call payload, successor, effects, outcome, time, disclosures,
and exercised capabilities. For C, `s` includes both complete calculus states.
The common checker can compare `s` but cannot compute candidate successors.

Keep active-parent envelopes in a separate authoritative registry.
Do not recursively embed signed plans inside their own consumption snapshots.
Each exact plan slot binds its expected predecessor and consumption successor.
After-resolve authorization binds both installment slots before signing.
Selecting the second signed slot cannot replace its recipient, effects, or mechanism.

Pre-sign records retain full unsigned checked inputs and computed dispositions.
Before-resolve checks have no concrete proposal argument.
After-resolve checks include the entire resolved plan without a signature premise.
Signing and execution records compare complete bound inputs at their boundaries.
Solver-selected `checked` or `safe` booleans cannot establish any local predicate.

Per-principal conditions evaluate incoming consideration and conditional outcomes
over the complete effect trace. Debit-only projections are insufficient.
Every actual debit occurrence requires authority, including funded escrow debits.
Refund policies have explicit destinations and conditions.

Separate three trust categories:

- Locally computed: structural equality, transfer arithmetic, conditions,
  ownership, nonce/residual transitions, validity, and rejection preservation.
- External premises: signature authenticity, authenticated state/environment,
  complete effect evidence, assumption evidence, and settlement evidence.
  Each premise needs an exact bound payload and available/unavailable/invalid status.
- Untrusted proposals: solver plan, effects, successor, outcome, summary,
  artifact/call, residual, transaction time, and requested evidence level.

The pending implementation plan must settle complete candidate state/call types,
finite condition cases, each evidence payload and freshness rule, and finite
nonwrapping environment bounds. Opaque two-value identity fields cannot substitute
for full state, policy, artifact, or call contents.

Consumption foundation plan `9450d15` implements only the independent bookkeeping
slice. Its fixture policy is not the full generic authorization contract above.
It cannot authorize a financial transition or satisfy cancellation-recovery coverage.
