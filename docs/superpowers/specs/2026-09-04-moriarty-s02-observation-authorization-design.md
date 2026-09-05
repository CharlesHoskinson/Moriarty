# S02 common observation and authorization design

Status: independently reviewed bounded experiment design. Classification: repository
observation and proposed bounded-model specification. No model has run and no
architecture is selected by this document.

This supplements the reviewed four-architecture design and contract plan.
It fixes common interfaces before model logic is written. It does not change
Core or semantic scope `0.0.0-e00.2`.

## Repository facts that constrain the model

The frozen Core credits an accepted deposit to an account but emits no
`Payment` for that credit. The local S01 payment extractor is therefore not a
complete financial observation function. Wallet and escrow locations remain
distinct even when their owner is the same principal.

Core input failure restores the original accounts, choices, continuation, and
minimum time. It returns no payments or warnings and zero reductions. Timeout
reduction with no supplied input can commit refunds; timeout reduction followed
by a supplied input can instead reach `contract_closed` and roll back everything.

Sequential `Pay` constructors reduce within one transaction. Two distinct
installment consumptions need a `When`/input boundary between their payments.

The Compact specialization checks Alice's witness for Alice funding and Bob's
witness for Bob funding and decision. Expiry has no participant-witness check.
These operational checks are not the proposed S01 envelope authorization.

Locators: `moriarty/core.py` at `_apply_input` and `compute_transaction`;
`moriarty/intent.py` at `effects_from_payments`;
`experiments/moriarty-core-swap/swap.compact` at its exported circuits.

## Shared interfaces, separate execution semantics

The following names describe model interfaces, not production wire schemas.
Exact Quint encodings belong in the next implementation plan.

| Interface | Required content |
| --- | --- |
| `Location` | Opaque wallet principal or escrow account identity; wallet and escrow are never merged by owner name. |
| `Transfer` | Source location, destination location, asset, and positive exact integer quantity; one occurrence accounts for both debit and credit. |
| `EffectTrace` | Ordered transfer occurrences; authorization compares multiplicities and never deduplicates or checks only net deltas. |
| `Ledger` | Total map over every modeled location/asset key; initialization supplies every key. |
| `CoreResultObservation` | Accepted flag, exact error, ordered warnings with requested/paid values, ordered Core payments, accounts, choices, continuation, minimum time, and reduction count. |
| `CandidateObservation` | Actual complete effects, proposed successor, semantic outcome, applicable Core projection, artifact/call identity, predecessor bindings, and evidence availability. |
| `AuthorityKey` | Domain, principal, and nonce; changing plan identity does not create a fresh nonce namespace. |
| `SignedPolicy` | Principal, key, profile, permitted effects and conditions, refund rules, validity, capabilities, disclosure rules, cancellation policy, mechanism/version bindings, and resolved-plan or bounded-intent binding. |
| `PreSignCheckRecord` | Proposed policy and, for after-resolve only, the complete resolved plan; checked content, context, enforcement identity, and validity disposition. It grants no signature or execution authority. |
| `VerificationRecord` | Exact policies, actual proposal/effects, predecessor, environment, version, consumption state, evidence level, and valid/unavailable/invalid disposition. |
| `ParentConsumption` | Signed parent, permitted slots, used slots, original budget, paid quantity, remaining allowance, cancellation state, and consumption revision. |
| `DisplayProjection` | Presentation only; changing it cannot change signed policy or authorization results. |

Common code may supply transfer arithmetic and authorization predicates. It
must not determine which financial transitions a candidate can execute. Each
candidate separately defines `propose`, `observe`, and `commitSuccessor` over
its own representation. C's two successor computations and bridge relation
remain independent of the common envelope check.

Deposits produce wallet-to-escrow effects only after accepted input matching.
Payouts produce escrow-to-recipient effects. Rejection commits no effects.
The Core payment list remains a separate, unchanged projection; do not insert
deposit payments into the frozen Core result to make it match the new view.

## Per-principal authority, not an implicit joint signer

Instantiate the S01-style check separately for each affected principal and
require their conjunction. Every wallet or escrow debit needs applicable
authority, and the combined check must cover every actual effect. Bob's choice
selects only a branch already permitted by Alice's signed escrow policy. It
does not itself authorize a debit of Alice's assets.

Each principal's check also evaluates that policy's complete relevant proposal,
including required incoming consideration and conditional outcomes. Covering
all debits is necessary but is not sufficient to satisfy those hard predicates.

This conjunction is a bounded composition proposal, not a proved composition
theorem. Signature authenticity remains an explicit external premise. A model
boolean must not be reported as a real signature or proof.

For the swap, each principal has nonce 0 for funding and nonce 1 for the
subsequent escrow disposition. A terminal transaction checks the disposition
policies for every funded escrow. An early refund checks only funded escrows.
Funding does not manufacture the later signature: disposition policies must
be explicitly signed in the model's lifecycle.

Under `SignAfterResolve`, first resolve the complete plan and perform the full
pre-sign plan check against each principal's proposed policy. Signing requires
that successful pre-sign record and unchanged applicable plan/context bindings.
It cannot circularly require the signature that the check enables. Then verify
the signed authorization and recheck execution-state freshness before commitment.

Under `SignBeforeResolve`, first check the bounded policy, authorization limits,
disclosure policy, assumption manifest, and pinned enforcement identity before
signing. No concrete plan exists or is labeled pre-sign verified at that point.
Resolution and complete signed-plan execution verification occur afterward.
Positive scenarios assume participants provide the modeled signatures. This
establishes no cooperation or availability claim.

## Two-installment fixture and cancellation economics

The installment experiment starts with a declared, pre-funded Alice escrow of
ten units. That initial balance is a fixture assumption, not a reproduced
funding trace. The signed parent permits slots 1 and 2, each paying five units
to Bob. Existing `When`/`Choice` boundaries separate the payments in A's fixture.

The first fill claims the parent nonce and records slot 1. Slot 2 remains
available only through that exact active parent and validated residual. A new
standalone policy cannot reuse the claimed nonce. The residual binds parent,
recipient, asset, mechanism, permitted remaining slot, and consumption history.
`paid + remainingAllowance = 10`; remaining allowance is not spendable authority
after cancellation. `SignAfterResolve` binds the full two-slot plan, not only
the first payment followed by an invented second authorization.

Cancellation only revokes remaining parent payment authority. It does not
release money, erase value, or reuse the cancelled fill nonce for recovery.
A separate recovery policy under nonce 1 permits only refund of the remaining
escrow to Alice after cancellation. Recovery is a distinct verified transaction
and has its own positive witness. A's recovery path uses a refund branch and
`Close`; no new Core constructor is required. Other candidates must provide
the same observable recovery under their distinct representations.

Preserve every existing registry witness identifier. Recovery adds these
mandatory named subscenarios, not a replacement top-level witness vocabulary:

- `cancel-wins/recovery-before-any-fill`: cancel the unused parent, then refund
  ten units to Alice under the separate recovery policy.
- `fill-wins/recovery-after-first-fill`: fill slot 1, reject the stale prepared
  cancellation, accept a fresh cancellation, and refund the remaining five
  units to Alice under the separate recovery policy. Slot 2 remains unauthorized.

Every candidate and both signing profiles must cover both subscenarios under
S02-05 and preserve their complete paths under S02-09. The coverage map and
validator must require these subscenario IDs explicitly, including their final
escrow, payment, refund, and authority states. Merely observing a cancellation
flag does not satisfy recovery coverage. Recovery also has a per-action witness.

Track financial value separately from remaining authorization:
`escrowBalance + paidToBob + refundedToAlice = 10`. A refund reduces escrow
and increases returned value; it does not consume another installment slot or
turn the cancelled parent allowance back into an active capability.

A prepared fill and cancellation bind the same consumption revision. The
winner invalidates the loser. A later fresh cancellation may revoke the
remaining allowance after an earlier fill. A cancelled authorization attempt
is not mislabeled as completed financial settlement while escrow is still held.

## Time, identity, and execution boundaries

Use concrete transaction times 1, 2, 100, and 101: two ordered pre-deadline
values, the deadline, and a post-deadline value. This refines the original time
classes so the E00 `time_before_state` error is testable. Physical environment
time progresses monotonically; stale proposed transaction time remains an
adversarial input, not backward physical time.

Use two nonce values per principal and workload, and two implementation
versions. An environment anchor or version change never wraps back and makes
stale evidence fresh. Bind the relevant full predecessor state as well as the
external anchor. Do not use a two-value transaction counter that aliases later
financial states to earlier ones.

Resolve, pre-sign check, sign, environment change, execution verify, and commit
remain separate actions. Permit environment changes between these boundaries;
a stale pre-sign record cannot authorize signing a substituted plan.
Commit requires that the actual proposal, predecessor, environment, version,
time conditions, and consumption state still match the checked bindings.
Otherwise reject or reverify before any financial or authority change.
Before-resolve signing cannot carry a concrete-plan verification result.

For a two-slot resolved plan, explicitly bind each slot's expected predecessor
and the parent/residual relation; a fresh state after slot 1 is not an excuse
to accept an arbitrary slot-2 plan. Environment substitution still invalidates
the applicable execution binding.

## Independent correspondence and limits

Candidate extraction cannot also be its own correctness oracle. Preserve the
full Core result independently of complete transfer extraction. Corrupting
either extraction or the abstraction map must be observable to an independent
check. Include funded deadline commit and rollback pairs, stale-time rejection,
and conservation checks over initial balances and actual transfers.

The modeled outcome supports only its declared abstract evidence level. No
real signature, authenticated private-effect extraction, Compact/ledger
correspondence, production cost, ACTUS result, or human preference follows.
The added installment recovery witness and finer pre-deadline time values are
explicit experiment refinements, not accepted Core semantic motions.

## Design review resolution

Independent review found that the initial draft incorrectly placed signing
before all verification. This revision distinguishes pre-sign checks from
signed execution verification and follows the frozen S01 profile boundaries.
It also closes the recovery witness mapping through two mandatory subscenarios
under existing registry identifiers and gates S02-05/S02-09. Debit coverage is
explicitly insufficient without each policy's incoming and conditional terms.

Independent re-review approved these corrections at `d742a5b` with no remaining
material design issue. That approval does not establish model execution,
implementation correspondence, or a proof.
