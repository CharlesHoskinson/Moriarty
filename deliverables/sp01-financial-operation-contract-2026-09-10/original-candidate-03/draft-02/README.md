# SP01 financial-operation contract — source draft

**Status: proposed and incomplete; no freeze or execution admission.** This draft
moves the next RP01 obligation from named operations to explicit financial state,
funding, staged effects and surviving claims. It preserves all38 archived proposed
Core operations and all24 action targets, plus AS01–AS11 and AT01–AT08. That is
inventory coverage, not financial conformance or acceptance of every operation.
It does not replace the active funded profile or alter expression candidate02.

Read [state and expression composition](STATE-AND-COMPOSITION.md), then the
[38-operation table](OPERATIONS.md) and full [operation records](operations.json). The [declarative rule interface](DECLARATIVE-RULE-INTERFACE.md)
and [typed rule families](rule-signatures.json) replace opaque callbacks with
bounded pure Core bodies and explicit numeric results. Complete schema/consumer
instantiation is still required; this mechanism alone cannot close full RP01.
The [crosswalk](crosswalk.json) retains each actual source target/asset requirement,
its candidate composition and its remaining obligation. The
[mixed arithmetic proposal](MIXED-ARITHMETIC-PROPOSAL.md) gives an explicit proposed
type/rounding path for the original AMM fragment, directed prices and signed
rates. The [explicit dynamic share constructor](DYNAMIC-SHARES-CONSTRUCTOR.md) adds a
proposed41st pure constructor while preserving all40 existing constructors;
`shares<V,H>(q)` creates a value, with no implicit mint or claim mutation. All new choices require independent review before adoption.

Concrete improvements in this draft:

- Each cash credit has a named debit, admitted issuance or explicit unpaid claim.
  Prior transfer receipts may settle debt without debiting the payer twice.
  Receipt allocations cannot be reused for two duties.
- Principal, accrued interest, fees, nominal debt and actual settlement assets
  have distinct equations. P100/I10 repaid7 interest-first leaves P100/I3.
- Pledging6 of10 then seizing2 leaves4 pledged and4 free, with independently
  denominated remaining debt. The exact seizure/collateral rule still needs a
  complete atomic plan; the arithmetic alone does not admit a liquidation.
- Requests separate committed input from output entitlement. The independent
  100-share/40-filled/52-cash/20-claimed trace can refund60 pending shares while
  preserving32 cash owed and all100 gross allowance consumed.
- Messages bind asset, quantity, escrow, recipient, payload, domains and evidence.
  `Pending` alone cannot establish remote absence or authorize a refund.
- The combined semantics proposal evaluates the body, then financial descriptors,
  then Ensure on the merged financial/ordinary post-state. Current expression
  candidate02 does not already implement that combined relation.
- The original constant-product expression has exact typed intermediate and
  rounding proposals: output19,743, quotient remainder162,290,000. No unit erasure,
  unbounded arithmetic or host pricing callback is proposed.

[Root's13 request/message cases](independent-request-cases-01.md) are independently
written proposed challenges and retained unchanged. Each of38 operation rows has
an author-derived positive/rejection discriminator. The
[bounded arithmetic/inventory check](check-draft.py) checks17 calculations, exact
source rows and all101 protected expression pins. The separate root arithmetic
script checks selected conservation inequalities. These are **not** a Moriarty
interpreter, a K proof, complete contextual histories or tested ledger behavior.
The [actual check result](draft-checks-03.json) records this limited scope.

## Next concrete semantic gap

Instantiate the closed financial schemas and operation contexts in Σ, then define
one **complete combined relation** for these38 rows, without unchecked RuleRef
holes. The first discriminating full history should join the existing interest-
first repayment and request residual cases: exact source/Core, original funding,
receipts, full before/after records, authority, work, ordered effects and Ensure.
A rehashed invalid sibling must fail the computed first predicate, not a supplied
fixture flag. This is a specification/independent-fixture task before runtime/K
implementation; it does not justify another partial runtime profile.

Before full RP01 can freeze, also close these exact decisions:

1. Resolve target-specific financial equations and models still marked open:
   ACTUS day-count/negative accrual/capitalization; AMM fee/reserve/tick iteration;
   position/funding/contingent payouts; event-claim split/merge; rate versus balance
   rebase; parameter updates; external settlement and message finality policies.
   A missing rule rejects; it cannot be hidden by keeping only loan/swap.
2. Define the finite policy/authority, record-date, encumbrance/atomic seizure and
   claim-migration relations and complete input/output schema. Proposed escrow
   requests do not stand in for all credit-bearing asynchronous protocols.
3. Review the explicit ConstructShares decision and mixed arithmetic
   extension with representation/bounds/cases. Surface short-circuit/Core strict
   Boolean reconciliation is a separate parent-owned draft. Neither requirement
   is removed by the current expression table rejecting it.
4. Co-define canonical signing/display for every new financial field, exact
   operation error codes/origin spans/privacy, whole-state/effect budgets and
   reconstructible helper cost. Complete split/join/authority/duty/work partitions
   and the three held-outs/eight intent cases remain in the full map.
5. Implement and independently compare source elaboration, evaluator and Felleisen–
   Hieb/K relations after the complete approved specification. Preserve all named
   metatheorems and full277 ACTUS/72 historical DeFi denominator. Actual Docker,
   Midnight Preview financial effects and mandatory PCD remain separate hard gates.

The asset report informs required design questions, not proven external legal
rights or protocol conformance. Cached source versions, source facts and proposed
choices are recorded in [source-inspection.json](source-inspection.json). No new
web acquisition, private data access, compiler, proof, service, wallet or ledger
operation occurred in this task. No new acceptance flag was changed.

The [preserved draft01 snapshot](draft-01-preservation.json) retains the earlier
bytes and checks. Draft02 adds the rule interface, complete-range NetAmount,
explicit refund recipients and atomic input/output reservation updates. No
independent root example was changed.
