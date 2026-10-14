# Proposed complete claim Carry and owner assignment

This correction04 rule is part of the generic financial consumer, not an
implemented authorization adapter. It closes the owner-field update left implicit
in draft03. The independent grant/occurrence judgment must bind the complete old
and new records; an AuthorityId in Args does not prove that judgment.

## A1. Exact template and construction

Carry requires destination=Some(freshId) and successor=Some(template), with
`template.id=freshId`. DutyTemplate contains exactly the 16 Duty fields other than
creationHash. No operand supplies the creation hash: G0 computes it from the
actual source/action/operation, new id and creator kind. This avoids requiring a
Core operand to contain a hash that commits to that same operand.

The consumer first derives the permitted successor from the source Duty and the
rules below, then compares all 16 template fields. It copies kind, origin,
resource, nominal, conditionRule, contingentLimit and alternateBeneficiary;
sets original=remaining=entry.quantity, discharged=0 and status to the source's
live condition status, or Open for a positive terminal quantity. backing is the
old optional reservation for one full Carry and None for an unbacked split.
The only proposed assignment fields are obligor, beneficiary and priority, and
only the owner cases below permit them. Every unlisted field frames. In
particular, a successor template cannot create an unrelated claim, change its
units, supply a new condition, enlarge its quantity, or invent new backing.

Read all entries and old owners first, reject duplicate destinations and mixed
owner updates, derive the complete update set, check it, then apply atomically.
Each owned claim and each claim referenced by a live encumbrance has exactly one
full Carry if it is carried at all. Only unbacked Standalone terminal claims can
split, with positive quantities summing exactly to the source remaining amount.
A future condition requires WholeCondition, including when remaining=0. No
claim is paid merely because it is assigned.

For every carried source: remaining becomes 0; discharged becomes old.original;
status becomes Discharged; backing becomes None. All other source fields,
including original, origin and creationHash, remain historical. The successor
hash is new; receipt/history hashes are never rewritten. A live owner references
the successor, whereas the old discharged tombstone need not remain its current
claim. Exact old/new priority must agree with any owning terms record and live
encumbrance priority; changing a pledged claim's priority without the complete
reservation/claim priority grants rejects.

## A2. Closed owner update table

This table describes the complete additional owner changes after A1. Any field
not listed is copied exactly. The source must be that owner's current claim.
Terminal owners with no remaining Duty cannot create a positive successor.
A retired source can never be assigned again.

| Duty origin | Derived owner and permitted changes |
| --- | --- |
| Debt | Replace Debt.claim. Set debtor, creditor and priority from the successor obligor, beneficiary and priority. Principal, interest, denomination, accrual dates/rules, controller and status frame. creationLiabilityAuthority is immutable creation provenance, not live permission to use the original debtor's grant after assignment. |
| RequestInput | Replace Request.inputClaim; set refundRecipient and inputTerms.priority from successor beneficiary/priority. Obligor must remain inputEscrow. inputOwner is the historical funder and remains unchanged, as do controller, inputSink, resource and all input/output counters. The input reservation keeps its owner/quantity and takes the new beneficiary/priority. |
| RequestOutput | Replace Request.outputClaim; set beneficiary and outputTerms.priority from successor beneficiary/priority. Obligor must remain outputEscrow. outputFunder, controller, resource and all counters frame. The output reservation takes the new beneficiary/priority. |
| Message | Replace Message.claim with Some(destination). Both parties, alternate beneficiary and priority must remain unchanged: changing the payload-bound parties requires an explicit message protocol transition, not a local Carry. Reservation.claim alone is rewritten. Domains, payloadHash, amount, deadline, evidence/resolution policy and lifecycle state frame. |
| Event | Replace EventClaim.claim; set obligor, claimant and terms.priority from the successor. stakePayer, stakeSource, stakeSink, stake and funder are historical/funding roles and frame. remaining retains the exact asset quantity. Outcome, evidence, maximumLiability, condition and payout rules frame. A new claimant does not become the historical stake payer. |
| Reward | Replace Reward.claim; set obligor, beneficiary and terms.priority from the successor. holder and recordDateHolder remain eligibility identities; shareUnits, rate, periods, pending/earned/paid, funder and all rules frame. The whole enrolled condition, including future income on that eligibility account, is assigned expressly. Transferring shares alone does not perform this assignment. |
| PositionMargin, PositionFunding, PositionPnl | Use the complete three-claim group rule A3. |
| Standalone | No owner record is rewritten. Explicit old/new-party and priority grants may authorize those fields; all fixed terms and units above frame. The exact source origin remains provenance, including for an allowed terminal split. |

For each actual live Encumbrance whose claim is carried, replace claim with the
unique destination. Its owner, resource, quantity, group, kind, releaseRule and
seizureAuthority frame. Set beneficiary to the successor beneficiary exactly when
old Encumbrance.beneficiary equals source Duty.beneficiary; otherwise retain the
old beneficiary. The independently checked complete affected-set grant must bind
any distinct protected party. SettlementHold creator invariants are stronger:
RequestInput/Output and PositionMargin require exactly their owner's current
beneficiary. Priority equals successor.priority for the RequestInput, RequestOutput and
PositionMargin SettlementHold creators; all other encumbrance priorities frame. No implicit transfer of
the pledged asset or release of a reservation occurs.

## A3. Complete position group

Any party assignment of an Active position-owned Duty requires all three current
claimDefinitions (Margin, Funding, Pnl) to have one full Carry entry, including
zero Conditional roles. A reference-only carry may replace one id with unchanged
parties/priority; it cannot change the position holder or obligor.

For a group assignment, derive newHolder from the Margin successor beneficiary;
its obligor must equal the existing custodian. Derive newObligor from Funding's
successor obligor when the funding metric is nonnegative, otherwise from its
beneficiary. Require Pnl to identify that same newObligor and newHolder using its
own metric sign. For an exactly zero metric the nonnegative convention applies.
Funding/Pnl successor roles must equal (newObligor,newHolder) for nonnegative
metrics and (newHolder,newObligor) for negative metrics. Margin and all three
condition/amount/unit rules remain as A1. This group rule applies to Active positions. Terminal Closed positions use the
separate exact residual rule below and cannot be reopened by Carry.

Replace Position.holder/obligor and the three claim ids; update each corresponding
claimDefinitions.id and terms.priority. The fixed role order remains Margin,
Funding,Pnl. The margin reservation beneficiary becomes newHolder; its owner
remains custodian. All collateral entries receive only the exact A2 claim and
permitted protected-party updates. Position.custodian, feeRecipient, kind, rules,
status and every metric frame. No locked balance moves merely because a claim
is assigned. Any destination that would violate an existing Shares<V,H> index
constraint rejects; Carry has no authority to erase or retag that index. A
separate explicit financial transition must establish a different share position.

A Closed position's metrics, holder and obligor retain its final trading history.
They are not an assertion that a subsequently assigned residual must still be paid
to the original holder. Each remaining positive current Duty has no further mark
or accrual condition. It may have one full Quantity Carry under the explicit
old/new-party grants; rewrite only that role's claim id and priority, and its
actual backing references. Position holder/obligor and all metrics frame. Duty
parties are authoritative for that terminal residual. Zero Discharged roles
remain unchanged, with their old ids; they need no fabricated zero-quantity
successors. If no backing exists, the single-owner rule still prohibits splitting
one scalar role into several ids. Pay/discharge reduces the Duty; it does not
rewrite closed historical metrics or revive the position. This distinction is
part of the final-state refinement, not a callback or a caller lifecycle flag.

Every affected holder, obligor, custodian and collateral beneficiary occurrence
must be covered by the independently checked assignment/novation/priority grants.
One Funding successor alone cannot silently reassign the Pnl or margin claim.

## A4. Registry seeds and complete traversal

RewardDefinition is immutable enrollment configuration. Its claim, beneficiary
and obligor are the initial seed fields, not aliases to a current Reward after
Carry. Enrollment uses the definition only when that exact RewardId has never
been enrolled; current rewards are never re-created from an old definition.
The state retains reward accounts, including zero ones, so this test is finite.
An existing Reward's current claim and parties come from the current record.
Migrate preserves the enrollment seed definition; it does not rewrite source
program bytes or reinterpret a seed as a current claim reference.

The complete current local-claim reference set is Encumbrance.claim, Debt.claim,
Position.claims and claimDefinitions.id, Request.inputClaim/outputClaim,
Message.claim, EventClaim.claim, Reward.claim and Instance.claims. Duty.origin,
creationHash, Event stakeSource and immutable RewardDefinition seed claims are
provenance/configuration, not live references. ExternalClaimId is a separate
namespace and never a local claim-map source or destination.

Run one bounded scan of every state table and its exact declared fields. Use the
old claim-map lookup at each current reference, derive all owner changes above,
then validate the full candidate and effects. Retained terminal owner fields may
continue to reference their zero discharged source if no Carry applies. An
owner with a live positive/Conditional duty must reference the exact new live
Duty; missing or multiply owned links reject. Unlisted fields and unrelated
records are exact copies. All scans, comparisons, hash construction and images
are charged by the primitive schedule in financial-work.md; caller-supplied work
counts and opaque assignment callbacks are inadmissible.

## A5. Source-level challenges

Debt P=80,I=20 can be novated to a new debtor with one full 100-unit successor:
Debt debtor/claim change together, source original100/discharged100/remaining0,
successor original100/discharged0/remaining100. Keeping the old debtor in the
Debt record or citing only creationLiabilityAuthority rejects.

A Request output has 32 cash units still payable after partial claim and cancel.
Assignment to B changes its current output claim, request beneficiary and hold
beneficiary together, retaining hold quantity32 and its escrow owner. A template
alone with B while the hold still names A rejects. Request inputOwner stays the
original funder.

An Event stake payer A can assign the whole contingent payout to B; stakePayer A
and its exact producing-effect hash stay unchanged. Replacing stakeSource with
B's unrelated payment rejects. An enrolled Reward may assign its income condition
to B while holder H still determines eligibility. This is an explicit whole-duty
assignment, not an automatic consequence of H transferring shares.

For position margin50, funding-5, pnl7, assignment H→J and counterparty O→N
requires Margin(custodian,J), Funding(J,N), Pnl(N,J), all three ids and margin hold
beneficiary J. Updating only Margin or using Funding(N,J) rejects. No balance is
credited to J until the actual payout operation runs.
