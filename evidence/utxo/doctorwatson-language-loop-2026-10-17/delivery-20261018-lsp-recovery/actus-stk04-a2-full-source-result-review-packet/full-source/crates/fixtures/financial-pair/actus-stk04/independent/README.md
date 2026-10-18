# Independent STK04 semantic expectations

Status: independent expectations frozen before any consumer check; public
financial_pair remains UNRUN. This is one finite two-call cashflow/state/effect
bridge for tests/actus-tests-stk.json/stk04, not all-field ACTUS conformance.

`oracle.json` uses its own explicit semantic-oracle schema. It records complete
Core pre/post snapshots, every source statement's typed value and complete
snapshot, every ordered delta and derived effect prefix, and the two selected
ACTUS cashflow records. Compare only its listed known fields. It is NOT a full
serialized Execution/Program or stdout golden: numeric ExprIds and exact byte
rendering are deliberately unasserted, never filled from producer output.

Source/scenario/call operands are exact input-custody copies. Terminate must use
the ACTUAL purchase.post for state, balances and liabilities; expected boundaries
are comparisons only. Full source and profile/input hashes are recorded. Public
DTO shape and normative excerpts are under basis; there was no evaluator,
compiler, test, Git, checkout or network action in this lane.

Reference-result field keys are retained explicitly: eventDate, eventType,
payoff and currency have independent semantic values; notionalPrincipal,
nominalInterestRate and accruedInterest remain UNASSERTED for ACTUS reference
comparison. The corresponding extra Core state fields ARE fully checked as
source-declared unchanged zeros. Those two claims must never be conflated.
Upstream payoff JSON number/string representation is also not selected by this
semantic oracle; signed decimal strings here are exact oracle arithmetic.

`work.json` freezes a separate CONCEPTUAL source-node schedule, purchase18 and
terminate17. No metered call was selected. Missing source-to-IR allocation and
node-shape correspondence means no CostReport.used/cap or runtime-meter result
is asserted. No additional consumer call is authorized by these tables.

All277 fixtures/fields,18 executable types,32 taxonomy dispositions and DS01–07
remain mandatory. Three unsupported present stk04 fields stay outstanding; no
full fixture/type/family pass is counted. Full ACTUS schedules, share delivery,
rate/accrual, performance state, native fees/proof/ledger and parent exits remain
open. No security, conservation theorem or settlement judgment is made.

The full original upstream fixture files, including results, remain unchanged
in the separate input-custody packet. This lane deliberately did not read those
result values or the historical authored oracle/test proposal. The original
42-payload custody manifest and complete277-member/field-key census retain their
identities. Incidental prior unrelated exposure is declared in PROVENANCE.json.

MANIFEST.json binds every payload except itself and FREEZE.sha256; the freeze
binds every payload including the manifest, excluding itself. Static checks
verify only expected-data arithmetic/structure and hashes, not actual behavior.
