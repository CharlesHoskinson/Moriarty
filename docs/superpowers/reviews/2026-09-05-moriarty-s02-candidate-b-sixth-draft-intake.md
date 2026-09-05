# Candidate B fifth/sixth-draft intake: not adopted

Classification: root source review and frozen-reference experiment observation,
2026-09-05. Drafts are archived verbatim in
`evidence/s02-model-comparison/candidate-b-plan-intake/`. They are planning inputs,
not implementations or grounds for rejecting the intent-Core architecture.

The author reported closure of the fourth-draft intake, but direct root review
does not support that claim. Do not implement the sixth draft unchanged.

## Concrete findings

1. The sixth draft's BState omits discharged dependencies and exclusions even
   though its enabling rule reads both. BGraph and the node-row type are absent;
   coreError/nativeError refer to undeclared optional/error types. There is no
   concrete state/update contract for the obligation graph.
2. Its refund/deadline reduction formula equals the number of refunded accounts.
   This omits If or expired When reductions. Existing exact corpus records give
   swap voluntary refund: 2 payments/3 reductions; empty timeout: 0/1;
   Alice-only timeout: 1/2; funded timeout: 2/3; installment timeout: 1/2.
   Direct recovery-choice refunds are 1/1. A single refund formula cannot cover
   these different paths. The corpus is actual Quint output already compared
   with frozen Python, not a new execution of Candidate B.
3. The negative-deposit diagnostic names canonical phase SA, whose declaration
   accepts Deposit10, and supplies -1. Frozen Python returns no_matching_input,
   not non_positive_deposit. Root executed and retained the exact request/output
   in `sixth-draft-negative-deposit-probe.json`. A separate diagnostic graph with
   a literal nonpositive declared deposit is required to test exact-match rejection.
4. Timeout table rows mention T100 only, excluding T101 without explanation.
   Ordinary input rows have no before-deadline condition. The prose does not
   define priority/rollback precisely enough to prevent an expired input from
   discharging its ordinary obligation.
5. BResult unconditionally contains raw Core fields, but native errors supposedly
   have no Core result. Use a sum type, not a record with an impossible absence.
   The complete rejected request must preserve workload/graph identity as well
   as node, before-state, input and clock.
6. Wrong input identity, out-of-bounds choices, missing input and closed-contract
   requests have no total classification table. The two supplied rejection
   examples do not define the finite attempted-input space.

## Disposition and next correction

The next contract must define graph state independently of any A interpreter:
literal ordered obligations, per-workload total graph maps, fulfilled/excluded
sets, and native guarded discharge/update rules. Define total finite request and
result variants, then a separate library-to-frozen-Core observation map with
explicit per-node reduction counts, full choices, time and continuations.
Native execution must not call A or copy the reference result. Reference checking
must compare its output independently. Each diagnostic needs its own actual
literal graph/workload and request. Authority and nonce state remain in the
shared envelope; do not duplicate or weaken it.

This is a failed draft-quality gate, not a failed architecture experiment,
model-checking counterexample, Council rejection, S02 stop or semantic motion.
Root will resolve the concrete contract before any B implementation. Preserve
both drafts and the earlier fourth-draft intake rather than treating an author's
completion statement as acceptance.
