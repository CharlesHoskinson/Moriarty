# Correction04 disposition and consequential choices

Draft03 candidate 7942eb0af701fc9bebeb796b979a56fdd37b2311a89787b90a35bfe755af36e1
received GPT-6 REQUEST_CHANGES and Grok APPROVE_SCOPED. The GPT-6 findings remain
blocking until this distinct correction receives fresh independent source review.
Both previous pure-extension design votes are retained. No acceptance flags,
runtime, K sources, or protected expression files change in this correction.

## Direct blocking findings

| Finding | Concrete correction |
| --- | --- |
| Computed Exchange outputs/fees could not enter the closed MovementPolicy input | CurrentMovement contains the actual leg endpoints, resource, quantity, fee, operation, origin and event index. Binding forces those exact fields. policy-events-04 lists every required operation and physical/book/allocated event. No minimum-output operand can substitute for the actual net leg. |
| Generated record fields lacked concrete sources | ReservationTerms, DutyTerms, origin variants, exact request/message reservations, event funding provenance, reward enrollment, position roles and imported supply now have closed records. field-generation-04 lists every constructor/update field. |
| ShareMint referred to unspecified new claims | Share ownership is the explicit proportional vault entitlement. Mint does not invent a fixed-amount Duty or a legal right. A declared claim-bearing issuance still needs its actual linked DebtCreate. |
| Work/mode and complete authority were not integrated | The resource account map and envelope now have an explicit source contract; complete independent mode/branch/authority composition remains a mandatory named integration task. No generic consumer may substitute an allowed=true callback. |
| Boolean/version and stale constructor wording | Nine exact Boolean /1 dependencies are retained. Current docs consistently preserve 40 plus eight constructors and do not silently change strict/selective evaluation. |

## Additional concrete defects exposed during correction

Negative accrual d<0 uses q=-d: subtract q from interest and remaining and add q
to discharged. Adding negative d to discharged violated nonnegativity and
original=discharged+remaining. Rebase must first close the old reward period at
C.now, just as share eligibility changes do. Otherwise 100 shares at rate .1 for
0..10 then .2 for 10..20 wrongly accrue 400 instead of 300.

A fully reserved margin50 cannot produce an earlier payout receipt. Separately
unlocking it would break the position's margin/reservation invariant. The new
PositionAdjust/Close consumer releases and pays internally with exact policy,
cap and effect checks. The witness with new input20 and payout60 ends at owned10,
reserved10. Prior MarginPayout allocations are no longer accepted.

At vault assets10/supply3, deposit4 should quote floor(4*3/10)=1 share. Requiring
the deposit payment first changes the denominator to14 and wrongly quotes0.
ShareMint/Burn now quote before their own flows, then execute exact asset/fee
legs and share issue/burn. Removed payments operands cannot be counted again.
Deposit/redeem floor and mint/withdraw ceiling remain the same 56-node pure
reference formula; this example is initialized and zero-fee, not a full product.

A Duty successor containing creationHash could require an operand to know its own
source commitment. DutyTemplate omits only that hash; the kernel compares every
other derived field and constructs the hash from the actual creator. Assignment
updates all owner roles and reservations atomically. Immutable stakePayer and
Reward eligibility holder do not change when the payout beneficiary changes.
RewardDefinition remains immutable enrollment seed configuration. PositionClose
sets its tentative terminal state before the residual map, so the map can
actually Pay/discharge instead of being blocked by an Active future-condition
rule. Closed metrics retain trading history; current residual Duties govern
later assigned payments without reopening the position.

## Choices requiring fresh consequential design votes

1. Actual internally bound MovementPolicy/ReleasePolicy inputs and operation-keyed
   RuleId families, including explicit authenticated release valuation.
2. Explicit proportional share entitlements, external-claim reference namespace,
   imported supply baseline and unborn-only Genesis construction.
3. Internal vault asset/fee flows and internal position release/payout, replacing
   circular or mispriced prior-receipt requirements without relaxing caps.
4. WholeCondition Carry, including zero current amounts, and the exact complete
   owner-assignment table. No implicit novation, share-index cast or legal claim.
5. Source-inaccessible final ResourceEnvelope, immutable WorkCheckpoint,
   committed Ordinary/Recovery action classification and the exact fixed final
   accounting epilogue. Source-selected node work remains actual, not prepaid.

The source inventory/checker is intentionally narrow. It verifies exact types,
field coverage and selected deterministic equations; it does not evaluate every
consumer or prove every refinement. Fresh reviewers must read the complete
consumer, field, owner, rule and work contract and its remaining integration
boundary, not infer acceptance from green source checks.
