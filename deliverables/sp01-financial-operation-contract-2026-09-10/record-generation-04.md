# Exact financial record generation — proposed correction04

This proposal replaces the missing creation/update relations in draft03. It is a
source contract for independent review, not an implemented evaluator, K relation,
complete product corpus, signed transition or Midnight acceptance. The unchanged
38 names remain. The accompanying field-generation-04.json names every field of
each creator and each changed-record role; a frame means byte-for-byte preservation
of every other field. In a conflict this document and the correction04 schema
replace the draft03 construction prose. Product rules return their closed numeric
or predicate results; they never return a freely selected financial poststate.

## G0. Inputs, keys and exact hashes

A is the already typed Args; C is authenticated OperationContext; S is the current
tentative financial state before this descriptor; O is its actual Emit Origin.
Rules evaluate against this current S, not an unrelated original snapshot. T is
a statically typed term from the fixed algorithm, never caller-computed evidence.

All id-keyed tables sort by exact ASCII id.name. Balances sort by the typed tuple
(owner,resource); supplies by resource; cash counters by (authority,party,resource)
and nominal counters by (authority,denomination). A full scan checks uniqueness.
A missing row rejects except the explicitly permitted fresh insertions below.
GroupId is a label scoped by C.instance. It does not reference an absent group
registry. Live reservations have distinct priorities within each scoped group.

W is the exact proposed canonical typed-value encoding. CreationHash is SHA256
of canonical ["MoriartyCreation2",kind,W(id),role,C.predecessor.hex,C.sourceHash.hex,
C.profileHash.hex,W(O),W(C.authority),SHA256(canonical W(A)).hex]. kind and role are
the literal creator roles in the inventory. This binds actual operands as well
as source/profile/origin and authority identity. ClaimMapEntry.successor is
DutyTemplate, which has every Duty field except creationHash; the actual Duty
gets the derived hash. No Args field contains its own generated creation hash,
and neither an output commitment nor an unbound caller hash enters its preimage.
ObjectHash is SHA256 of canonical ["MoriartyObject1",W(value)] with its exact type.
ObjectChange.id is canonical JSON text of the table's complete typed key tuple.
Before/after payloads and hashes refer to that operation's actual images. None
is allowed for a genuinely absent before image only. Tombstones keep after images.

## G1. Computed movement and release inputs

Each nonzero policy event in policy-events-04.json creates CurrentMovement with exactly eight fields:
source,recipient,resource,quantity,fee,outerOperation,origin,legIndex. These are the
actual computed leg endpoints/resource/UInt128 amount/fee class, current operation
name, O, and zero-based policy-event ordinal. It cannot be supplied in Args,
read from arbitrary state, or replaced by the minimum output of an exchange.

The resource's AssetDefinition.policy or VaultDefinition.policy selects the
PolicyDefinition in the current admitted registry. Only Migrate's destination
representation event uses the exact admitted destination registry; the source
event still uses the old registry. A missing definition is never a fallback. A full scan of movementBindings selects the unique row whose
operation equals outerOperation. Its rule must resolve to a MovementPolicy
RuleInstance for that same operation and exact concrete role tuple. Duplicate,
absent or mismatched bindings reject RULE_BINDING. Transfer/Fee explicit policy
must equal this resolved resource policy. For every leg, rule binding roots named
CurrentMovement select this generated record. MovementPolicyInput.from/to/resource/
quantity must bind its source/recipient/resource/quantity respectively. They may
not bind Operand, Constant, or a different state field. frozen and policyInputs
retain their restriction scan and authenticated observation sources. Each actual
invocation, including repeated same-rule invocations, is retained in ruleUses.

CurrentRelease contains lock (the exact old Encumbrance), quantity, remainingAfter,
debtOutstanding,collateralValueAfter,outerOperation,origin. quantity and
remainingAfter are computed checked q and old.quantity-q. SettlementHold has
explicit non-credit inputs debtOutstanding=collateralValueAfter=0. Pledge resolves the exact lock.claim Duty and uses its remaining liability units
as debtOutstanding; for Debt origin, also require equality with the linked P+I.
The collateral value comes from an authenticated Observation payload of the
closed ReleaseValuation type, purpose Release, explicitly selected in that rule
instance's readSet. Require payload.lock=lock.id, claim=lock.claim,
resource=lock.resource, quantityAfter=remainingAfter, liabilityResource=Duty.resource
and liabilityNominal=Duty.nominal. The ordinary domain/issuer/schema/time/hash and
authentication judgments must hold. collateralValueAfter=payload.valueAfter.
Thus the input is an explicit value quote in the exact liability units for this
remaining asset or share collateral, not an unnamed valuation callback, a share
unit erased to cash, or a claim that the quote is realizable exit liquidity. A
profile that computes this fact must supply its actual authenticated valuation
model; a ReleasePolicy true cannot certify that source. A missing exact quote or
unbound zero substitution rejects. Other collateral is not silently added to this value.
ReleasePolicyInput.quantity/remainingReservation/debtOutstanding/
collateralValueAfter bind only these corresponding generated fields. Release
selection is the old lock.releaseRule, supporting the actual outer operation.
Request/Message/Position internal releases use this same helper before movement.
It is not enough that a lifecycle policy returns true.

Movement ordering remains the per-leg order in financial-consumers.md: compute
rules and quantities, then for each leg check its policy, funding/reservation,
and caps before constructing its updates. A cap error on leg1 precedes a funding
error on leg2. All tentative legs roll back on any error. A zero omitted leg
creates no receipt, counter debit, policy invocation or event ordinal. Book-only
events (for example Mint, Lock and ShareMint) call policy with their exact
computed units/roles but create no cash movement or duplicate counter debit.
They need not have distinct source/recipient parties; that guard belongs to a
physical move. A policy event's ordinal is not an emitted Receipt ordinal.

## G2. Common balance, supply, receipt and duty constructors

A movement subtracts its exact q from the matching source balance and adds q to
the recipient balance; an absent recipient balance starts at zero. Asset/resource
identity and share holder indices are reconstructed from the declared quantity
variant, not inferred from a ticker. Existing zero balances remain.

Supply fields are resource, imported, issued, burned. Genesis derives imported
from authenticated input inventory. It is immutable thereafter. checked256
imported+issued-burned must be UInt128 and equal the sum of matching owned units.
Mint/ShareMint increment issued; Burn/ShareBurn/Slash increment burned; ShareConvert
increments source burned and destination issued. Imports can therefore be burned
without pretending they were issued by this program. A share resource's represented
total also equals its vault.shareSupply. These are represented inventory equations,
not a claim about total external/native token supply.

A created Receipt has deterministic id as defined in financial-consumers.md,
kind from the actual movement/Mint, source/recipient from that event, the exact
quantity, O, and allocated=0. receiptsCreated is immutable creation evidence.
Later receiptAllocations are ordered events. The current available amount is
creation amount minus all earlier valid allocations of that id. The evaluator
never retroactively rewrites a prior OperationEffect.receiptsCreated. Persistent
stake provenance therefore retains exact producing origin, not only an action-local
receipt id. New ShareConvert backing purposes require Ordinary receipts; Fee and
Issuance remain inadmissible as cash backing.

Every Duty has 17 fields. A creator sets all fields through a role in the inventory:
id,kind,obligor,beneficiary,alternateBeneficiary,origin,resource,nominal,original,
discharged,remaining,priority,backing,conditionRule,contingentLimit,status,creationHash.
Exactly one resource or nominal is Some. original=discharged+remaining always.
Conditional is a live future/alternative obligation even with remaining=0. It
cannot be omitted from affected-set/signing checks. Non-message duties have
alternateBeneficiary=None. A Some alternate is exercised only by that exact
MessageReceive/MessageRefund consumer, never an ordinary ClaimMap Pay.

A ClaimMap Carry successor copies kind, parties, origin, units, priority,
conditionRule, contingentLimit and alternate beneficiary from its source unless
an exact typed assignment relation authorizes that particular field change. It
sets id=entry.destination, original=remaining=entry.quantity, discharged=0,
creationHash=the Carry creator hash, and moves its specified backing allocation
once. It cannot copy the full old reservation to two successors. Conditional
claims cannot be split, paid, discharged or assigned merely by quantity=0: their
future obligation requires the exact occurrence/condition authority and complete
owner-reference rewrite. Source and successor status are derived from that
condition's terminal/live state; numeric zero alone does not determine them.

### Complete conditional portions

ClaimMapEntry.scope is exactly Quantity or WholeCondition. hasFuture(d,S) is false for a Discharged tombstone. For a live duty it is a
closed owner lookup: Debt→Outstanding; RequestInput→inputPending>0; RequestOutput→
fillOpen; Message→Pending with positive quantity; Event→Open; Reward→the retained
enrolled account; PositionMargin/Funding/Pnl→Active position; Standalone→Conditional
status. Every origin must resolve to that exact owner and its exact claim field.
A Discharged historical duty retains its origin but need not remain the owner's
current claim id after a checked Carry. Every live duty must match its owner.
No runtime callback supplies hasFuture. For an origin with no future, Quantity
requires positive entry.quantity, and the ordinary complete partition equation.

A future obligation requires exactly one WholeCondition entry for that source,
disposition Carry, quantity=source.remaining (zero allowed), one fresh successor,
no payment and no conversion. Preserve origin, units, conditionRule, contingentLimit, backing and status;
party/priority changes are permitted only through the complete OWNER-ASSIGNMENT-04.md
rewrite and its exact old/new-party and affected-set authority obligations; carry the entire condition
and rewrite its complete owner reference, not just its current numerical balance.
The explicit authority occurrence must bind that future condition even when its
current amount is zero. A WholeCondition may not split, change units, Pay, or AuthorizedDischarge.
It may assign the complete obligation only through the explicitly supported
owner cases in OWNER-ASSIGNMENT-04.md; a cash-transfer grant is insufficient. An explicit matching lifecycle operation must first
terminate a condition before the ordinary terminal quantity rules apply. This
keeps Migrate of a zero pending output/reward well-defined without erasing risk.

A source with any live Encumbrance referencing it also permits only one full
Carry successor, even if Duty.backing=None: scan all encumbrances, not just one
backing field. Rewrite their claim ids to that successor exactly once; all quantities/owners/resources/groups/release rules frame; beneficiary/priority
changes follow the exact OWNER-ASSIGNMENT-04.md table and grants. Source
Duty.backing becomes None and successor.backing retains the same actual optional
reservation id. Its source becomes Discharged. Partial unbacked standalone
quantities can split; a collateral/owner split requires explicit financial
operations creating the actual new claims and reservations, not an ambiguous
ClaimMap allocation without those fields.

ShareConvert changes its exact free share entitlements and funded backing. It
does not automatically convert other liabilities that remain denominated in the
old share resource. ClaimMap Carry preserves those units; any different-unit
liability requires its explicit financial conversion/settlement composition.
ClaimMap cannot interpret a numerical share-conversion quote as arbitrary
permission to rewrite an independent debt or future condition.

## Transfer, Fee, Exchange, Mint and Burn

Transfer/Fee/Exchange use G1/G2 for all balances, receipts, counters and effects.
Exchange uses actual quote legs, including output-outputFee, never minimumOutput
as the policy amount. A share balance change additionally executes G8 below.

Mint creates no Duty directly. Optional promised debt links the exact Issuance
receipt to a later DebtCreate in the same action. No promise is invented for an
asset whose declared mint creates no debt. Burn changes only represented inventory
and explicit complete claim-map effects. Both preserve imported.

### External rights references are not local duties

AssetDefinition.externalClaim now has Option<ExternalClaimId>, a distinct nominal
sort from local ClaimId. It resolves a RegistryDefinition/FinancialState
externalClaims row. ExternalClaimDefinition has exactly id, issuer,
rightsDocumentHash, termsHash and verificationPolicy. All are explicit immutable
profile declarations; the document/issuer authentication requirement applies to
those exact commitments. No local amount, payout Duty, title guarantee or legal
enforcement is inferred from this reference or possession of the token.

An actual payable or nominal promise still requires an exact local Duty creator
and its authority. Genesis can import an external-asset token and its explicit
rights reference without making a dangling local Duty id or fabricating one.
Transfer preserves this asset-class reference unless the independently declared
asset policy permits that transfer; it does not silently assign a separate local
claim. Migrate preserves the complete referenced definition byte-for-byte. A
local ClaimMap cannot rewrite ExternalClaimId, even with the same display name.

## Lock and Unlock

Lock copies all supplied Encumbrance fields after independent owner/resource/claim,
group/priority, ReleasePolicy binding, seizure-grant and available-balance checks;
tombstone must be false and quantity positive. It does not create the referenced
Duty. Unlock frames all fields except quantity and tombstone. quantity'=quantity-q;
tombstone'=(quantity'=0). A seizure ticket remains unavailable until its exact
later Transfer consumes it. No omitted ticket may survive Ensure.

## DebtCreate, DebtAccrue, DebtRepay, DebtWriteOff and DebtCapitalize

DebtCreate requires A.debt.interest=typed0, status=Outstanding and
lastAccrualEnd=C.now; the remaining fields copy their exact typed operand after
identity/rule/liability checks. Historical backdating is not an unnamed input.
The fresh Duty has origin Debt(debt.id), kind NominalDebt, debtor/creditor roles,
nominal=Some(principal denomination), resource=None, original=remaining=P,
discharged=0, priority=debt.priority, backing=None, conditionRule=conversionRule,
alternateBeneficiary=None, contingentLimit=None, status=Open and exact creator hash.

Accrual updates interest and lastAccrualEnd only in Debt. A positive increment d
also adds d to Duty.original/remaining. For an allowed negative signed increment d, define q=-d>0; subtract q from interest
and remaining and add q to discharged; original and principal stay unchanged; principal is unchanged. Repay/WriteOff
subtract their exact dP/dI, add dP+dI to discharged and subtract it from remaining.
Debt status becomes Settled after a full repayment, WrittenOff after a full
writeoff, otherwise Outstanding. Duty is Open iff remaining>0, else Discharged.
Capitalize changes only P/I and nominal creation counter; the complete Duty frames
because outstanding nominal quantity and creditor promise are unchanged.

## ShareMint, ShareBurn and ShareConvert: explicit entitlement

The generic vault constructor creates a proportional share entitlement, not an
unnamed fixed-cash Duty or implicit legal claim:
ShareEntitlement(V,H)=(balance of Shares<V,H>, V.shareSupply, V.account,
V.underlying, V.conversion). These are declared typed fields/relations. Conversion
rules, actual available exit assets, fees, reserves and separate liabilities remain
explicit. There is no missing share ClaimId to conjure. Separate promised cash or
external rights must be declared as their actual Duty and authority, not attributed
to an accounting interface alone.

Within this model distinct vaults may not claim the same (account,underlying)
owned inventory as their entire backing. The pair must be unique. This is an
explicit logical-account invariant, not a claim that distinct controllers/wallets
are required. A shared native custodian needs an independently bound partition
into those logical accounts; duplicate aggregate backing cannot pass the ledger
correspondence relation. Whole owned inventory in that pair is the vault's
backing; donation changes backing without changing shares. Other assets are not
silently included in NAV. A product-specific valuation rule must bind them
explicitly. Vault conversion here is asset-side, not an automatic share wrapper.

ShareMint/Burn first evaluate their exact conversion rule against the actual
current pre-flow state. They have no prior-payment operands in correction04.
ShareMint then moves holder→vault.account assets-assetFee and holder→feeRecipient
assetFee. ShareBurn moves vault.account→holder net assets and vault.account→
feeRecipient assetFee. Zero fees omit a leg. Actual movement policy, available
funds, gross/fee caps and complete effects apply. Only then update exact holder
shares, issued/burned and vault.shareSupply. No preceding receipt is required or
allocated for those same flows. A caller-supplied quote still must match the rule.
This fixes the pre-funding price cycle: assets10/supply3, deposit4 quotes1 share
from the pre-flow10; looking at a pre-funded14 would incorrectly quote0. Other duties frame unless an explicit claim map applies.

ShareConvert fixes sourceBacking and destinationBacking in its rule result and
in its exact typed operands. Same underlying requires equal backing quanta and
an earlier Ordinary sourceVault.account→destinationVault.account allocation under
ShareConvertBacking. Distinct underlying requires two actual prior Exchange
legs: sourceVault.account→that Exchange.pool sourceBacking and that same pool→
destinationVault.account destinationBacking, under ShareConvertSource and
ShareConvertDestination. Both must originate from the same exact prior Exchange
OperationEffect and its matched input/output roles, not arbitrary co-use of two
transfers. Fee movements never count. The independently computed source/destination
share quantities and complete explicit claim map then govern burn/mint; all other
vaults frame. No rule Boolean supplies missing backing.

## PositionOpen, PositionAdjust and PositionClose

PositionOpen requires exactly three PositionClaimDefinitions, ordered Margin,
Funding,Pnl, with distinct ids equal position.claims in that order. Static terms
and IDs come from these typed operands; no numeric result chooses them. Remaining
Position fields copy the operand after exact zero-baseline rule comparison;
status=Active. Margin's definition requires a Some ReservationDefinition whose id
appears in collateral. Funding/Pnl definitions require reservation=None. Other
collateral ids must already resolve and remain unchanged.

Margin's fresh hold has id/terms from its definition, owner=custodian,
beneficiary=holder, resource=resource(metrics.margin), quantity=margin, kind=SettlementHold,
claim=Margin id, seizureAuthority=None, tombstone=false (also at zero). Its Duty
has origin PositionMargin(id), same resource/roles/backing, original=remaining=
margin, discharged=0, and supplied DutyTerms. Open if positive, otherwise
Conditional while the position can acquire margin. No initial claim can be
created without the independently bound liability/beneficiary grants.

Funding/Pnl duties use their exact metric asset, terms and origin PositionFunding/
PositionPnl. Positive metric means obligor→holder, negative holder→obligor; zero
uses obligor→holder and Conditional. original=remaining=absolute metric,
discharged=0, backing=None, alternate/limit=None. This is an explicit signed
contract obligation, not spendable cash or an implicit pure numeric cast.

Adjust uses the exact signed metric computation. Input funding and fee remain
exact earlier MarginFunding/PositionFee allocations, but output is an actual
internal payout. MarginPayout uses in A.payments reject. Extend the margin hold
and Duty.original/remaining by the exact newly allocated input. Release exact
output from that extended hold through G1, then move custodian→holder output
with real movement/cap checks. This order also handles input20/output60 on old
margin50: reserve50→70→10 and owned70→10. It does not require an impossible prior
Transfer from fully reserved custody. Margin hold changes by the funded
input-output delta; its Duty original increases on added margin and its
discharged increases on released margin. A release calls G1. Funding/Pnl replace
the old pending mark with the new one: original'=original+abs(new),
discharged'=discharged+old.remaining, remaining'=abs(new). This preserves cumulative
history through a sign change, whose discharged/created and party changes require
their exact authority occurrences. Bounds apply to those cumulative fields too.
Close requires input0 and pays its released margin through that same actual
internal payout, with no prior MarginPayout receipt. After the exact close rule and internal margin payout, set the candidate
Position.status=Closed before evaluating its residual claim map. This terminates
that position's future condition for the map; otherwise hasFuture(Active) would
wrongly forbid its required Pay/discharge dispositions. It resolves every
funding/Pnl/collateral claim through the complete map against that terminal
candidate. Retained zero claims become Discharged at this step. Positive
residuals obey OWNER-ASSIGNMENT-04 A3: closed metrics are history, while current
Duty amount/parties control any retained or assigned payment. It cannot release unrelated referenced collateral wholesale.

## RequestOpen, RequestFulfill, RequestClaim and RequestCancel

RequestOpen copies typed static fields and requires mode=Escrow, C>0, P=C,
F=R=E=K=0, fillOpen=true. Input/outputTerms each explicitly supply group, priority
and releaseRule. The two reservation IDs and two claim IDs must be distinct. Resources and
escrow parties may coincide; their separate holds must still satisfy the summed
reservation invariant. No separate-wallet/controller requirement is introduced. Each fresh hold
copies its side's terms; kind=SettlementHold, seizureAuthority=None,
tombstone=false, claim=that side's fixed claim id. Input owner=inputEscrow,
beneficiary=refundRecipient, quantity=C; output owner=outputEscrow,
beneficiary=beneficiary, quantity=0.

Input Duty: origin=RequestInput(id), obligor=inputEscrow, beneficiary=refundRecipient,
resource=inputResource, original=remaining=C, discharged=0, priority=inputTerms.priority,
backing=inputReservation, conditionRule=cancelRule, status=Open. Output Duty:
origin=RequestOutput(id), obligor=outputEscrow, beneficiary=request.beneficiary,
resource=outputResource, all three quantities0, priority=outputTerms.priority,
backing=outputReservation, conditionRule=claimRule, status=Conditional. Both have
nominal/alternate/limit=None and exact role-specific creator hashes.

Fulfill f consumes the exact input hold by f atomically with its input movements;
P'=P-f,F'=F+f,inputDuty.remaining'=remaining-f,discharged'=discharged+f.
For net output e: E'=E+e, outputHold.quantity'=quantity+e,
outputDuty.original'=original+e,remaining'=remaining+e. Fees keep their own legs.
Claim k changes K/output hold/output duty only, with discharged+=k. Cancel moves
remaining P to bound refundRecipient, R+=P,P=0,fillOpen=false and input hold/duty
zero. Already accrued output remains owed. Every internal release uses G1.

After every request transition input Duty is Open iff P>0 else Discharged;
output Duty is Open iff E>K, otherwise Conditional iff P>0, otherwise Discharged.
Input hold tombstone iff P=0. Output hold tombstone iff P=0 and E=K. Thus claiming
current output before later fills does not tombstone its still-needed identity.

## MessageSend, MessageReceive and MessageRefund

MessageSend requires Pending. q=0 requires reservation/claim/terms all None;
q>0 requires all Some. A positive hold uses explicit terms/id, owner=escrow,
beneficiary=sender, resource, quantity=q, kind=SettlementHold,
claim=message.claim, seizureAuthority=None,tombstone=false. It represents custody
pending a branch, not an ordinary sender right to bypass message resolution.

The Duty has origin Message(id), obligor=escrow, beneficiary=recipient,
alternateBeneficiary=Some(refundRecipient), resource=Some(resource), nominal=None,
original=remaining=q,discharged=0,priority=terms.priority,backing=reservation,
conditionRule=resolutionRule,contingentLimit=None,status=Conditional,creator hash.
Receive/Refund must select that exact matching message branch with authenticated
current evidence; after G1 release and the selected recipient/refund movement,
hold quantity=0/tombstone=true and duty remaining=0,discharged+=q,status=Discharged.
Message state is Received or Refunded. q=0 changes status only. No branch permits
a second consumption or a refund that resets cumulative gross.

## EventClaimOpen, EventClaimResolve and EventClaimSettle

EventClaimOpen has explicit stakeSink, funder, maximumLiability and DutyTerms.
It allocates exact initial claimant→stakeSink Ordinary stake payments and
sets immutable stakePayer=that initial claimant. Later checked beneficiary
assignment never rewrites the historical stake payer or receipt origin. stakeSource is
reconstructed from those producing receipts' exact predecessor/source/action and
operation index, allocation list and producing OperationEffect hash. All stake
uses must share that producing operation; otherwise reject RECEIPT_USE. EventClaimOpenArgs.claim is EventDefinition, containing only declared static
terms plus payoutAsset. It has no stakeSource, outcome, evidence, remaining or
status field. The consumer alone constructs these: outcome/evidence=None,
remaining=typed zero payout asset, status=Open.

The fresh Duty is Contingent, origin Event(id), obligor/claimant, payout resource,
original=discharged=remaining=0, priority/conditionRule from terms, backing=None,
alternate/nominal=None, contingentLimit=Some(maximumLiability), status=Conditional,
creator hash. The immutable EventAdmission/model and exact signed liability scope
must bind that explicit maximum; a supplied larger number is not self-authorizing.
Resolve requires q<=stored maximum, then original/remaining+=q,contingentLimit=None,
status=Open iff q>0 else Discharged; Event gets actual outcome/evidence, remaining=q,
status=Resolved. Settle allocates exact funder→claimant cash and reduces Event and
Duty remaining while increasing discharged; Event Settled iff0. Stake proceeds
and actual payout funder are distinct explicit roles.

## G8. Reward enrollment and share eligibility

A VaultDefinition contains finite explicit RewardDefinitions; each binds one
holder, vault, reward id, claim id, output asset, rate, parties, three rule ids and
DutyTerms. Duplicate (vault,holder,id), claim IDs or conflicting definitions reject.
An unlisted holder acquires no invented reward promise. Genesis may enroll exactly
those declarations; first share credit to a listed holder also enrolls a missing
record. This is a fixed holder-account reward reference model, not a claim that all
staking protocols use it or that transferring a wrapper conveys all past income.

Reward initializer copies its definition's static fields; shareUnits=actual owned
Shares<V,H> after the triggering change, beneficiary=definition.beneficiary, pending=earned=paid=typed zero output
asset, lostShares=0,lastAccrualEnd=recordDate=C.now,recordDateHolder=holder.
Its fresh Duty has origin Reward(id), kind Contingent, obligor/beneficiary roles,
output asset, all quantity counters0, priority/conditionRule from terms,
backing/alternate/nominal/limit=None,status=Conditional,creator hash.

Before changing units of an existing enrolled record, require lastAccrualEnd=C.now;
otherwise REWARD_PERIOD_STALE and the action rejects. An explicit RewardAccrue must
first close the prior period. After the change set shareUnits to actual new holder
balance and recordDate=C.now; recordDateHolder remains the immutable holder.
Pending past income and its Duty keep their current beneficiary; no implicit income
assignment occurs. Each share-changing operation scans all matching definitions
and records in canonical order; its effects and work include these changes.

RewardAccrue adds its exact result to pending/earned and Duty.original/remaining,
advances lastAccrualEnd, and leaves recordDateHolder fixed. RewardClaim subtracts
pending/adds paid and Duty.discharged, subtracting Duty.remaining. Duty is Open
while pending>0 and Conditional at zero because future enrolled eligibility remains.
Slash performs the reference share burn once, increments lostShares and uses this
same eligibility update after checking the accrual endpoint. Earned rewards stay.
Rebase requires lastAccrualEnd=C.now before changing rate; otherwise
REWARD_PERIOD_STALE. Its current rate is an accrual input, so repricing an earlier
period is forbidden. An explicit prior RewardAccrue closes that period. Rebase
then changes only rate; it cannot retrospectively change already earned rewards. No opaque new-record callback occurs in those operations.

## Genesis

GenesisArgs no longer accepts arbitrary initialState. Its input financial
state must be unborn: every financial-state collection is empty. Otherwise
STATUS rejects before any creation; this initializer never clears unrelated
existing balances, claims or instances. A new independent instance needs its
own unborn state/consumption lineage, not a destructive reinitialization of a
shared existing FinancialState. Resolve one immutable
ProgramReference matching C.program/version/policy/profileHash. Its complete
RegistryDefinition is part of the authenticated profile commitment. Copy its asset,
party,nominal, external-claim-reference and vault declarations; derive each vault.shareSupply from actual
funded share balances rather than a caller declaration. Registry zero placeholder
supply must be0. Aggregate the exact unique authenticated FundingInputs by their
owner/resource. Create balances and supplies imported=sum,issued=burned=0.
There is no hidden initial issuance; use a separate explicit Mint after Genesis.

Create cash counter authority/id/party from each exact fresh AuthorityDefinition
id/principal and its ResourceBudget resource/grossCap/feeCap/netGoal; create
nominal counter id/denomination/creationCap from the same authority and each
NominalBudget. All identity/duplicate/budget/seed relations must validate first; created/gross/fees/eligibleIncoming start0 only for a fresh authority seed.
No predecessor authority or allocation is reset. The separately specified exact
seed/parent relation is required; GenesisPolicy Bool is insufficient. All debt,
position,request,message,event and encumbrance tables start empty. Reward enrollment
uses only the exact named immutable RewardDefinitions above and creates the stated
zero Conditional duties. Other liabilities are created by their ordinary explicit
operations, not by a prefilled financial snapshot.

Create Instance.id=A.instance,authority=A.authority,program=C.program,
policy=C.policy,specHash=C.profileHash,mode=Dormant,predecessor=C.predecessor,
claims=all actually created duties in key order. A separate kernel-owned InstanceResources record uses the exact original
signed initial ordinary/recovery allocation and actual charged debit, not a new
caller-selected budget. Instance itself has no work fields. A.initialWork must equal its signed ordinary allocation.
No nested Genesis or second initial funding consumption is permitted.

## Activate, Pause and Revoke

These change only the selected Instance.mode to Live, Paused or Revoked after the
specified transition, authority and evidence checks. All fields frame except the
common predecessor/complete claim-index update; resource work stays in G10. Revoke reason is retained
in exact Args/effects. No administrative operation erases financial liabilities.

## Migrate

This consumer rebinds representation/program, not an implicit economic exchange.
Any economic conversion uses explicit prior Exchange/ShareConvert/payment/debt
operations and their full effects in the same transition. Resolve the exact
immutable destination ProgramReference by A.toProgram/toPolicy and its admitted
version/profile. Missing or ambiguous destination rejects. Both source and destination must
use this admitted common financial descriptor ABI and declared finite quantity
types; a different unknown type schema is not an automatic migration target.

ResourceAlias is a finite bijection between source/destination ResourceIds. An
asset alias must preserve nativeIdentifier, domain, quantum, issuer, version, network, ownership/claim
policy and mapped external claim identity; a share alias preserves the complete vault
account, underlying under the same alias map, conversion and reward obligations.
Two different economic resources may not be made equivalent by equal spelling.
Duplicate sources/destinations reject. All unmapped referenced definitions must
remain identically present in the destination registry. Copy destination static
definitions only after those comparisons; carry actual dynamic vault supply.

Apply aliases by a schema-directed traversal to every ResourceId and every
FinancialQuantity/NetQuantity/SignedQuantity/Price payload index. A quantity's
quanta/sign/scale/holder remain identical; the destination exact type must exist.
No alias changes a nominal denomination, party or magnitude. All ordered state
collections are reconstructed and resorted by their exact new key; key collision
rejects. This finite traversal uses declared type structure, never a text-name
search over JSON. Hash/receipt/history origin bytes remain unchanged.

Claim mapping uses the complete independently derived affected set. A standalone
claim may have several exact positive successors with partitioned backing. A claim
owned by a scalar Debt/Request/Message/Event/Reward/Position field requires one
full Carry successor while that owner remains live; a split needs an explicit
separate financial composition, not a dangling scalar reference. Pay/discharge
cannot leave a nonzero or Conditional owner pointing to a retired duty.
Rewrite actual claim-reference fields only: Encumbrance.claim, Debt.claim, Position.claims/claimDefinitions.id,
Request.inputClaim/outputClaim, Message.claim, EventClaim.claim, Reward.claim,
Instance.claims. A complete scan checks that all old
references either map once or retain a still-live old duty. Carry rewrites Duty.id
and linked backing exactly as G2; the old duty stays a discharged tombstone.

Set selected Instance.program/policy/specHash from destination; preserve authority,
all cash/nominal counters and remaining work except actual debit. Instance.predecessor
is C.predecessor, the input commitment; the next output commitment is computed
outside its own payload. Full destination typing, claim/reference, backing, supply,
mode and capacity checks run after rewriting. No arbitrary poststate, source-free
conversion or policy Bool substitutes for this traversal and its exact grants.

## G9. Complete effects and preparation boundary

Effects retain all created/mutated images, exact deltas, every receipt allocation,
actual rule calls and observation uses, including internal helpers. Empty lists
are present. Capacity overflow rejects; no evidence is dropped to fit. Every
successful operation updates the selected Instance.claims to the complete sorted
duty-id list, including Conditional duties and retained tombstones. The input-predecessor field is fixed before Ensure. Final work is computed in
the separate G10 resource envelope. Ensure sees the complete stable merged
financial/ordinary business state; no later resource bookkeeping changes it. The separate signing/parent-budget composition must bind this
exact financial judgment; this document does not claim that integration is done.

## G10. Stable poststate and resource envelope

Proposed choice: computation accounting is a kernel-owned envelope, not a
source-visible business field. FinancialState.instances contain Instance without
ordinaryWork/recoveryWork. FinancialPrepared.resources is ResourceEnvelope with
ordered InstanceResources(instance,ordinaryWork,recoveryWork) and the input
predecessor. This is mandatory final acceptance data, not optional diagnostics.
FinancialPrepared.ordinaryRemaining/recoveryRemaining equal the selected envelope
entry. No resource budget disappears from ledger/report/proof correspondence.

The Core financial field has type Record<FinancialState> for both Pre and Post.
Ensure receives only this complete business state plus the ordinary post and
original immutable arguments/observations. Source-visible records contain no path
to ResourceEnvelope, InstanceResources or FinancialPrepared. These are kernel-only
types: reject any source field/argument/observation/constant/rule input/result,
record/collection/variant/Option alias, or constructor annotation whose transitive
type graph reaches them. State selectors start only at FinancialState tables.
Context selectors expose only OperationContext, which has an explicit immutable
WorkCheckpoint(ordinaryBefore,recoveryBefore,predecessor) copied before body
evaluation. It never denotes current or final work. Whole-record projections and
local aliases preserve that restriction; an alias does not create a privileged
read. Unknown dynamic-field access is already forbidden by static typing.

After business state validation and Ensure, final output encoding/bounds contribute
actual logical cost, then the fixed accounting epilogue in financial-work.md
computes the envelope once. No fixed point, retry, guessed caller count or change
to /1 actual selected-node entry charges occurs. Failure in that epilogue discards
the same candidate. Semantic validity is the conjunction of the stable business
post/Ensure judgment and the independently checked exact resource-debit judgment.

The resource account is derived before body evaluation from the immutable exact
profile resourceAccounts map at C.action. This is not an Args flag or a guess
from authority.operations membership. The map's finite key set equals the
program's complete action declaration set, with Ordinary or Recovery for each.
Its bytes are committed with program/source/profile. FinancialPrepared retains
the selected class for independent checking. Every body/rule/helper/Ensure/final
charge uses that one account; the other account stays unchanged. Both remain
subject to their authenticated original cap. The separate action/mode/servicing
authority judgment is still mandatory: a classification does not itself permit
an action, a branch partition or a recovery reset.

OWNER-ASSIGNMENT-04.md is the exact Carry role/reference rewrite table. A supplied
successor template is a request to the consumer, not an accepted poststate. Every
non-assignment field must equal its derived value; every designated changed field
must satisfy that table and the independent exact authority judgment.
