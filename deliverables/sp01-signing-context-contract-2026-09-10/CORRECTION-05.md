# Exact occurrence and lifecycle correction 05

Status: proposed corrected source contract; independent rereview required.
Original03/04 files and reviews remain unchanged. The original03 Grok retry
PASS_SCOPED did not reproduce the authorization defect; fresh GPT6 review B-1
controls the correction. No original candidate is promoted to approved.

review-red-05.json retains the actual03 checker accepting (a) one exact fill
permission reused at a different predecessor under the unchanged authority,
(b) accrual10 with created0, and (c) terminal success followed by accrual. The
reproducer derives the old checker output to exhibit its behavior; it is not an
independent positive financial oracle. New tests use hand-specified expected
records and exact rejection codes.

## Exact occurrence binding

The closed05 authority replaces reusable exactPlans with exactOccurrences.
Each occurrence has exactly id, predecessorStateHash, predecessorLocation,
predecessorResourceHash, planHash, postStateHash and terminal. Every selected
linear step or branch Action carries occurrenceId. OutcomeIntent requires that
field empty and no exact occurrences; it retains bounded outcome authorization.
ExactPlan requires one matching occurrence and consumes its ID once across the
entire accepted history, including sibling branches and the seed history.
External consumedOccurrences is a trusted source-context input, never a signed
fixture label or a cryptographically authenticated registry claim.

Locations are derived, not caller assertions: linear:i is the predecessor of
linear step i; root is the graph seed output; graph:i:j is output j of node i
in the bounded topological graph. Linear resource commitment is its complete
state hash. Branch resource commitment hashes the entire owned output record,
including account ownership, residual quotas, debt-owner bit, backing/duties,
work and retirement. State equality alone cannot substitute another ownership
record. Exact occurrences additionally bind the selected full plan and resulting
state and terminal designation.

This avoids a hash cycle: state/output bodies and structural locations contain
no authority hash or node hash. Compute those bodies and plan hashes first,
then the exact authority containing their commitments, then linear transition
and graph node hashes which bind that authority. No occurrence commits the
hash of a graph node that already commits the same authority. Node identity,
currentness and independent composition authorization remain checked as before.
The new occurrence binding does not grant Split/Join permission by itself.

A valid occurrence may execute once at its bound predecessor. Reuse at a later
predecessor rejects EXACT_OCCURRENCE_CONSUMED with the original authority unchanged.
A separately authorized second occurrence at the correct second predecessor
passes. A fresh occurrence with an incorrect location/state/resource/plan/post
commitment rejects EXACT_OCCURRENCE_BINDING. Duplicate occurrence IDs reject.
The shared graph occurrence set includes seed consumption, preventing one exact
permission from being copied across sibling residuals.

## Admitted subset lifecycle

Accrue and Repay require created>0 for the bound obligation; accrual cannot create
an obligation implicitly. Existing same-plan funded CreateDebt and matching-cash
Repay checks remain. This is an explicit lifecycle rule for the admitted source
subset; full38 debt statuses, reopening/novation and rate derivation remain the
financial ABI owner's responsibility.

A successfully validated terminal step is absorbing for its complete history.
The checker rejects every subsequent step with TERMINAL_CONTINUATION, before
financial derivation. A terminal seed similarly cannot begin a nonempty branch
graph. Terminal success still first requires its existing net/no-debt/no-duty
predicates. This terminal fact is retained in the checked history/result; no
standalone state snapshot may waive the required seed-history validation.
An unfinished prefix is not falsely marked terminal. No terminal acceptance
signature or on-chain tombstone is claimed by this source validator.

## Checks and retained limits

Run context-checker-05.test.py and branch-checker-05.test.py. The original03/04
regressions remain in these successor suites. New controls cover one-use exact
execution, explicit two-occurrence sequence, external consumption, uncreated
accrual, absorbing terminal behavior, branch occurrence reuse and terminal seed
rejection. Exact hashes and examples are retained in draft-candidate-05.json.

The05 source-example domains are distinct from03/04. Full38 financial ABI,
full expression costing, shared-state composition, real signatures, proofs and
ledger currentness remain open. The correction changes no prior wire/signature
acceptance and performs no private, network, wallet, service or financial action.
