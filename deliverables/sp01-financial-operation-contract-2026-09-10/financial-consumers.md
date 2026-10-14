# Deterministic financial operation consumers — proposed correction04

This specifies state-update mechanics for all38 names against the closed reference
schema. Product formulas are finite declarative rule bodies, with their exact
closed parameter/result types; missing/ill-typed bodies reject **profile admission**.
A profile cannot claim full38 support while omitting a required rule or replacing
one by OP_RULE_UNDEFINED at runtime. No implementation or economic conformance
is established here. Earlier draft tables are history where signatures differ.

## Common algorithm, quantities, effects and failures

Use the canonical argument record from financial-operation-bindings.json. Before
the body, validate the full schema/state/context and all statically reachable
rule definitions. Context authentication, ledger/native identity binding and
current predecessor consumption are mandatory external judgments, not unverified
Boolean input fields. Their separate specifications/implementations remain open;
preparation alone cannot certify them.

Process descriptors in emission order. Each owns a local tentative state, receipt
allocations and effect builder; failure discards the whole action. Each successful
operation adds exactly one OperationEffect. Its movements, all before/after
balance/supply/reservation/debt/counter deltas, changed-object hashes, claim map,
receipt uses, observation IDs and rule IDs are constructed from actual computation.
They are never accepted from a caller. ObjectChange hashes are over the canonical records carried directly in each
ObjectChange.before/after ChangedObject payload. This retains intermediate
preimages even when a later operation changes the same object again. Before=None
and beforeHash=None denotes fresh insertion; otherwise hash and payload must
both be present and agree. after and afterHash are mandatory; tombstones retain
retired objects. The first/last touched states must reconcile with complete
action pre/post snapshots, and consecutive intermediate images must agree. Unchanged fields/lists are retained. Delta lists use deterministic
first-touch order; repeated updates coalesce at their first position with original
before/final after, while movement/receipt order remains literal execution order.

`units(q)` returns the integer payload of a declared FinancialQuantity variant;
`resource(q)` returns Asset A or Share V; `holder(q)` exists only for Shares<V,H>.
These are fixed typed state-operation destructors, charged below, not new pure
numeric casts. `quantity(resource,owner,n)` constructs the matching declared
Amount<A> or Shares<V,owner> variant; a missing receiver variant rejects
RESOURCE_TYPE. No type is created dynamically. Asset-only rows explicitly reject
share variants. An exact resource match ignores holder only when the operation
moves shares between separately checked source/destination parties; the output
then has the destination holder index. All other equalities preserve full type.

`move(s,d,r,n,fee,release)` requires n>0,s!=d; exact source/destination and resource,
scoped authority and policy, sufficient available balance or the specifically
bound release, and cumulative gross/fee caps. It subtracts n from source and adds
n to destination, constructs one ordered movement and fresh deterministic receipt,
increases sender gross n and, if fee, fees n. Incoming net-goal classification is
fixed in immutable authority/profile context, never chosen by the movement caller.
A balance entry absent at a receiver may be inserted with prior zero, within
bounds; existing zero entries remain. No refund decreases counters. Move does not
change represented supply. All physical input/change/fee effects of a ledger
transaction remain a later separate native observation/correspondence obligation.

Move receipts have kind Ordinary or Fee. Mint creates an Issuance receipt without
an ordinary cash movement; it is retained in receiptsCreated and cannot be used
as a payment. ReceiptId is its own nominal record wrapping Hash, not a name-constrained ID.
Its hash is SHA256(canonical JSON of ["MoriartyMovementV1", predecessor.hex,
sourceHash.hex, action, decimal(operationIndex), decimal(legIndex), receiptClass]).
All components are retained in context/origin/movement or receiptsCreated.
Indices are zero-based; each emitted receipt uses its ordinal among that
operation's created receipts, including Mint's Issuance receipt at index0. It is not a network transaction ID.
`allocate(uses,purpose,source,recipient,resource,required)` scans the ordered distinct
receipt/quantity uses, checks same-action prior origin and exact roles/resource,
exact requested purpose, required receipt class, positive quantities and
sum=required, and increments allocated amounts at most
receipt units. Reusing a receipt in multiple uses in one list rejects; separate
operations may allocate disjoint remainder. Allocation changes no cash a second
time. Ordinary leftover transfer quantity remains a transfer, not a paid debt.
The purpose/class matrix is closed: ShareFee and PositionFee require Fee; DebtIssuance requires
Issuance; Funding,DebtRepayment,ShareFunding,SharePayout,StakeFunding,RewardPayment,
EventPayment,ClaimPayment,MarginFunding and MarginPayout require Ordinary.
An operation chooses its required purpose; the caller cannot relabel a Fee receipt
as DebtRepayment. A list containing a use of another purpose rejects RECEIPT_USE
before sum checking. Share mint/burn split their payments by the explicitly
required ShareFunding/SharePayout and ShareFee purposes, validate each group
against independently computed quantities, and require all uses consumed exactly
once. The same rule applies to mixed margin/fee/claim settlement in positions.

Each operation runs guards in this order: identity/lookup/version; actor/grant/
policy scope; current status; required authenticated observation binding; evaluate
rule bodies in named order and compare exact outputs; typed amount/domain and
arithmetic; then each ordered movement/allocation step with its own
funding/reservation/receipt checks followed by its own cap checks; resulting record/frame
invariants; complete result bounds. Within each stage use argument/field canonical
order and movement lexical order. Movement1 cap failure precedes movement2
funding failure; all changes still rollback. This is precise per-leg order,
not a claim that all future legs' funding is checked before any cap. Admission/static failures precede the action.
`Require` body failures precede all operation failures; final Ensure sees merged
ordinary/financial post and runs only after all operations. No supplied failure
label controls execution. Errors use the actual source Emit origin and failing
rule/substep; private operands never enter a public diagnostic.

`POLICY_REJECTED`, `RULE_AMOUNT_MISMATCH`, `RESOURCE_TYPE`, `RESOURCE_MISMATCH`,
`IDENTITY`, `STATUS`, `OBSERVATION_BINDING`, `AMOUNT_RANGE`, `INSUFFICIENT_FUNDS`,
`RESERVATION`, `RECEIPT_USE`, `CAP`, `CLAIM_CONTINUITY`, `FINANCIAL_BOUND` and
`WORK_EXHAUSTED` identify the stages above; existing rule arithmetic codes are
preserved. Failed preparation returns no state/effects. Local consumed-work
reporting never increases stored authority or permits an automatic retry.

## Transfer

Require quantity source-holder index for shares. Execute one nonfee move using
the stated source/recipient/quantity. Optional release is valid only for the
matching restricted release ticket below. No other debt/claim/supply changes.
A resource's attached ownership/claim transfer policy must be instantiated and
approved in MovementPolicy; a bare receipt-token transfer is not presumed to
assign external legal rights.

## Fee

Require asset quantity. FeeArgs.basis must be the same asset as quantity. FeeQuote.basis is exactly
units(FeeArgs.basis), while numerator/denominator are immutable constants;
compare its computed fee to quantity exactly. Execute one fee move. No repayment allocation can
also count these same quanta as interest/principal. Fee income is goal-eligible
only if the fixed receiving goal classification admits it.

## Exchange

Require two distinct asset resources. ExchangeQuote fixes output, inputFee and
outputFee from exact bound input/reserves/price and constants. Check its concrete
role bindings match the operand assets. Require output>=minimumOutput, and
outputFee<=output. Execute in this order: payer→pool input; payer→inputFeeRecipient
inputFee if nonzero; pool→payee output−outputFee if nonzero; pool→outputFeeRecipient
outputFee if nonzero. MinimumOutput denotes **net received**, so check
output−outputFee>=minimumOutput as well. Full reserve/cash deltas come from those
moves; pure quote values are not extra balances. Exact-output intents add their
separate equality check. Other debt/claims frame unchanged.

## Mint

Require asset quantity, exact issuer capability and IssuancePolicy. Increase
recipient balance and represented cumulative issued quantity by n; no funded
transfer receipt is invented. ClaimMap transforms existing claims only; it cannot create an unbacked fresh
claim from IssuancePolicy. MintArgs.debtClaim=None declares no new debt in this
operation and is permitted only when AssetDefinition.requiresLinkedDebt=false.
Some(debtId) declares a required linked debt, whether or not the asset requires
one. It creates an action-local pending obligation to link this exact Issuance
receipt to that later fresh DebtCreate(kind=IssuedDebt). Whole-action validation
rejects if the declared link is absent, mismatched or partially allocated. Thus
unpromised token minting does not automatically create debt, while declared
claim-bearing issuance cannot omit its associated liability. Native issuance capability and external backing remain explicit
acceptance premises; a source rule cannot supply them.

## Burn

Require available asset quantity and RetirementPolicy. Decrease holder balance;
increase cumulative burned quantity, requiring represented supply sufficient.
Validate explicit claimMap. Unmentioned debt/redemption claims remain unchanged.
Burn itself pays no cash and cannot satisfy a settlement receipt requirement.

## Lock

Require fresh lock ID, quantity>0, valid claim/owner/beneficiary/resource, unique
priority in its collateral group and LockPolicy. Insert exact reservation from
available balance; total exclusive reserved may not exceed owned. Set tombstone
false. Balance owned and nominal debt remain unchanged.

## Unlock

Check exact live lock, positive quantity<=remaining and ReleasePolicy. Ordinary
release reduces reservation and makes that amount available. Seize creates an
action-local restricted ticket bound to this source/resource/quantity, the
explicit recipient and a strictly later Transfer operation index. It reduces the
persistent reservation but the ticket remains included in unavailable balance
until consumed by that exact Transfer; no other spend can use it. Every ticket
must be fully consumed before Ensure or the action rejects RESERVATION. Thus an
owner cannot divert released collateral between descriptors. Remaining lock
priority/claim persists; zero reservation retains a tombstone. Debt changes only
through a separately funded/authorized debt operation.

## DebtCreate

Require fresh ID, principal P>0, initial interest exactly0 (zero is valid and
required), consistent nominal denomination, valid parties
and DebtCreationPolicy. InventoryLoan requires exact creditor→debtor Funding allocation. The explicit
NominalConversion denomination equals P and settlement.asset is fixed before
receipt inspection; denominator>0. Required cash is floor/ceil(P*numerator/
denominator) as named, using UInt256 product and UInt128 result check. IssuedDebt requires exactly one prior Issuance receipt whose Mint.debtClaim
is Some(this debt.id), source issuer=this debtor and recipient=this creditor.
Settlement conversion binds that receipt asset and requires its complete minted
quantity equal the computed conversion of P. Allocate it once under DebtIssuance;
partial or extra receipts reject. A generic issuer capability or rule Bool does
not substitute for the exact typed linkage. AuthorizedNonCash requires its distinct signed liability grant and
creates no cash. Insert debt and a fresh derived Duty at debt.claim: kind NominalDebt,
obligor=debtor,beneficiary=creditor,resource=None,nominal=Some(P denomination),
original=remaining=P,discharged=0,priority=debt.priority,backing=None,
conditionRule=debt.conversionRule,status=Open. Derive creationHash from the
canonical calling operation/arguments/authority/rule/predecessor tuple; no caller
may choose a different duty record. Increase cumulative nominal created by P;
creation caps do not refresh after payment. No initial debt is silently inherited
from a caller's prefilled record.

## DebtAccrue

Require period.start=lastAccrualEnd and valid end/evidence. AccrualQuote returns
sign plus magnitude in the same denomination. For positive delta add to I with
checked O=P+I. Debt.negativeAccrual is the closed Reject|ReduceInterest choice. Reject rejects
all negative results. ReduceInterest requires magnitude<=I, subtracts only I and
discharges that same nominal duty amount; otherwise reject AMOUNT_RANGE. Neither
case reduces P or creates a reverse debt implicitly. A separately authorized
DebtCreate may represent a credit under a complete product profile; the accrued
operation does not hide such a creation. Negative zero rejects AMOUNT_RANGE. Update endpoint once; cash
unchanged. A profile implementing only nonnegative accrual rejects negative as
its declared financial predicate, not an undefined operation.

## DebtRepay

Require nominal matches debt denomination and positive value<=P+I. DebtAllocation
returns dP,dI; check bounds and dP+dI=nominal. SettlementQuote returns required
payment and permitted excess in the exact settlement asset/roles. Allocate the
required earlier debtor→creditor DebtRepayment allocation exactly. The rule
result permittedExcess is an upper bound on unallocated ordinary payment receipt
remainder, not another debt discharge. This descriptor allocates exactly required
payment, never excess. A retained positive excess must be resolved by an explicit
refund transfer, Fee receipt or carried Duty entry under the complete action
contract; absent that relation the action fails CLAIM_CONTINUITY. The numerical
ceiling alone cannot manufacture a refund or paid-fee classification. Subtract dP,dI; remaining
same debt/duty, Settled iff both0. Never debit/credit cash again.

## DebtWriteOff

LossAllocation computes exact principal/interest deltas; match operands, require
positive total and component bounds. Check creditor/scoped discharge authority,
subtract components and update duty discharge explicitly. No funds appear.
Residual same claim/priority survives. Pure impairment is not nominal writeoff.

## DebtCapitalize

Require exact nominal tag,0<d<=I and CapitalizationPolicy. Set P'=P+d,I'=I−d,
checked O unchanged. Record changed principal in full effects/signing; this is
not a token transfer or a fresh borrowing allowance. Creation-cap treatment is
explicitly conservative here: charge d against lifetime nominal creation capacity
unless that signed capability specifically contains a separate capitalization cap;
the reference schema has no separate cap and therefore charges the existing cap.

## ShareMint

Require Shares<V,holder>, exact asset-side quantity and Deposit or Mint. Evaluate
VaultConversion against the actual current state before this operation's own
flows; compare assets/shares to Args exactly. Args.assets is total depositor
debit; require assetFee<=assets and shares>0. Execute real holder→vault.account
assets-assetFee and holder→feeRecipient assetFee moves, then increase shares and
supply. Zero legs omit; every nonzero cash flow has actual policy/cap/effect checks.
No prior payments operand/allocation exists. The explicit ShareEntitlement relation
in record-generation-04.md is updated; no fixed-cash Duty is invented.

## ShareBurn

Require available Shares<V,holder> and Withdraw or Redeem. Evaluate VaultConversion
before the operation's own flows and compare exact operands. Args.assets is net
holder output; actual vault debit is assets+assetFee, checked128. Execute real
vault.account→holder assets and account→feeRecipient assetFee, then burn shares
and reduce supply. No prior receipt is required or double-allocated. NAV does not
substitute for actual available exit assets. Pending independent claims persist.

## ShareConvert

Require same holder, distinct vaults, available source and ShareConversion output
matching destination. Retire source shares and mint destination shares, with both
vault supplies updated. The claim map must move the complete corresponding
entitlement/backing, conserve unlike-unit value under the declared conversion,
and retire old claim exactly once. If backing resources differ, the required
funded exchange must precede and be independently allocated; absent evidence
rejects CLAIM_CONTINUITY. No arbitrary policy result can supply backing.

## PositionOpen

Require fresh identity, kind-specific finite metrics and PositionOpening output.
The rule returns signed deltas from a zero baseline for its fixed kind. The
consumer applies the numeric algorithm below, preserving each requested metric
slot's resource identity, and requires the result equal the requested metrics.
It checks mandatory invariants: range lower<upper,
liquidity>=0; other quantities/types within exact domains; collateral references
exist; PositionArgs.amounts is an explicit typed input/output/fee triple, not an
implicit host-built record. Position.custodian and feeRecipient fix all payment
roles. Allocate holder→custodian amounts.input as MarginFunding and holder→feeRecipient
amounts.fee as PositionFee. Opening output must be0. Adjustment/closure output is
the exact internal funded payout in record-generation-04.md, not a prior
MarginPayout allocation. Each group has
its exact declared resource; opening requires output0. The margin resource
matches input/output and margin= input−output; fee cannot enter margin. Insert
position and named duties only with separately validated claim creation grants. Kind-specific formulas are profile bodies, not
arbitrary callbacks. All exposure/claim identities are fixed by the descriptor,
never chosen in numeric rule outputs.

## PositionAdjust

PositionAdjustment returns numeric deltas matching changes. Add each to the same
metric slot in canonical field order with its exact unsigned/signed/indexed bounds.
Check lower<upper for range kind, allocate only the exact input/fee groups,
and perform the internal reservation release+payout in record-generation-04.md. The exact margin delta is input−output in the unchanged margin resource;
computed marginDelta must equal it. Other changes in exposure are not funded
cash. Reject unallocated or wrong-resource payment uses. Signed P&L/funding remain claims/exposures, not spendable balances.
Negative resulting unsigned quantities reject. Claims cannot be removed or
reassigned by a numeric delta; explicit debt/claim operations are required.

## PositionClose

PositionClosing computes deltas that must leave liquidity, notional and margin
zero; lower/upper/expiry stay unchanged and funding/Pnl claims cannot disappear
without the following claim map. Input is0 and output is the released margin, paid internally once with real
movement policy/caps/effects. Fee is separately funded; prior MarginPayout uses
reject instead of becoming a second payout. Check
every attached claim is either
fully paid through matched receipts, validly discharged under authority or
assigned exactly once in the explicit claim map. Release only the corresponding
permitted collateral; unpaid fees/debt survive as mapped duties. Set Closed and
retain tombstone/history; other positions frame unchanged.

## RequestOpen

Escrow mode only in this generic record. Require C>0,F=R=E=K=0,P=C,fillOpen=true,
input/output resources and roles bound, inputOwner/refundRecipient agreement or
explicit signed assignment. RequestAdmission and exact prior inputOwner→inputEscrow
allocation fund C. Create input reservation C and input claim C; create zero output
reservation/duty with fixed identities, resource and beneficiary. No output
entitlement or cash is inferred from committed shares.

## RequestFulfill

Require current open request and positive inputFill<=P. RequestConversion computes
outputEntitlement and fees; match all operands. **Fee convention:** inputFill is
the total consumed input including inputFee; outputEntitlement is net beneficiary
entitlement after outputFee. Require inputFee<=inputFill. Release input reservation
by inputFill; move inputFill−inputFee escrow→sink if positive, then inputFee
escrow→inputFeeRecipient if positive. Output funder moves outputEntitlement to
outputEscrow and outputFee to outputFeeRecipient, omitting zero legs. Increase
output reservation by outputEntitlement. Set P−=inputFill,F+=inputFill,E+=
outputEntitlement; close fillOpen when P0. Input claim decreases consumed input;
output duty increases exact net entitlement. Every fee remains a concrete flow,
not a missing conservation term. Insufficient backing anywhere rolls back all.

## RequestClaim

Require0<q<=E−K and bound controller/beneficiary. RequestClaimPolicy checks local
policy; release q of the exact output reservation and move outputEscrow→beneficiary
q. K+=q; residual E−K and reservation remain equal. Input counters and fillOpen do
not change. Finished requires P0 and E=K; no generic status erases claims.

## RequestCancel

RequestCancellation and evidence must allow cancellation on current predecessor.
Require P>0; release P input reservation and move inputEscrow→refundRecipient P.
R+=P,P=0,fillOpen=false; E/K/output reservation unchanged. Previously claimed cash
does not prohibit this unrelated pending-input cancellation. Gross stays consumed.

## MessageSend

MessageAdmission binds Pending message and payload/domain/roles. q>0 requires
exact earlier sender→escrow allocation and fresh reservation/duty q at the two
present IDs; q0 requires reservation=None and claim=None and creates no cash duty. No remote cash/acceptance is inferred.

## MessageReceive

Require Pending and authenticated exact payload/domain/recipient/finality/conflict
evidence before MessageResolution. The reference settlement form releases local
reservation q and moves escrow→recipient q, then sets reservation quantity0
with a tombstone, matching local duty remaining0/discharged+=q/status=Discharged,
and message state Received. Require that duty remaining was exactly q before
release, preserving original=discharged+remaining. q0 changes status only.
No remote settlement is claimed unless its separately bound evidence relation
establishes it. Other messaging forms require another explicit profile relation.

## MessageRefund

Require Pending and authenticated exact absence/conflict/recovery evidence under
the bound model; local Pending alone is insufficient. MessageResolution must
permit refund. Release q reservation and move escrow→bound refundRecipient q;
set reservation quantity0 with a tombstone, matching local duty
remaining0/discharged+=q/status=Discharged, then Refunded. Require its previous
remaining exactly q. q0 has no reservation/duty mutation. Keep history; no second payout/currentness
reuse; gross cannot decrease.

## EventClaimOpen

EventAdmission validates exact funded stake, finite outcomes, fixed claimant/
obligor and payout/credit model. Allocate earlier stake payment, insert Open
claim with outcome/evidence None and zero realized payout. Its contingent duty
remains explicitly conditional, not spendable cash. No arbitrary maximum payoff
comes from an input Boolean.

## EventClaimResolve

Require Open, supported outcome and authenticated resolution evidence. EventPayout
computes exact entitlement from stake/outcome/profile inputs; store typed remaining
entitlement, outcome/evidence and Resolved. Create/update the fixed obligor→claimant
duty. A large legitimate option payout needs that exact model and signed exposure;
the fixed1-payout fixture must reject a caller's1,000,000 substitution.

## EventClaimSettle

Require Resolved and positive same-asset amount<=remaining. EventSettlement plus
exact prior funding-account→claimant receipt allocation supports it. Reduce claim/
duty remaining by amount; Settled iff0. No second cash credit or source-free payout.

## RewardAccrue

RewardQuote binds the original record-date holder, period and authenticated reward
source. Require nonoverlapping next period and exact result amount/resource. Add
pending/earned and advance endpoint, updating named duty. No spendable cash credit;
later share transfer cannot retroactively change the record-date beneficiary.

## RewardClaim

RewardPayment must admit positive amount<=pending; allocate exact prior funder→
current explicitly assigned beneficiary payment in the reward asset. Decrease pending/increase paid
and discharge only that amount. No second cash credit; unpaid reward persists.

## Slash

SlashQuantity computes the exact requested positive share units within current
slashable balance; check specific authority/evidence. Reference burn form reduces
share account units, corresponding holder share balance and vault supply once,
and increases lostShares. Existing earned reward and unrelated debt frame unchanged.
A different loss mechanism must have a separately explicit profile consumer;
calling this burn form a universal staking/slashing rule is forbidden.

## Rebase

RebaseRate returns UInt128 newRateMantissa, checked<=2^127−1 and equal to the
nonnegative exact mantissa of the operand at the existing scale. The consumer
reconstructs only that same Rate scale; no undeclared pure Rate cast occurs. Require lastAccrualEnd=C.now before changing the accrual rate, otherwise
REWARD_PERIOD_STALE. Update rate only;
share units/supply and nominal debt do not change. This reference is share-rate
revaluation, not a token balance rebase; the two have separate economic observations.

## Observe

ObservationPolicy validates exact immutable authenticated observation reference,
field/type/domain/freshness. Append observation-use record only. No new observation
snapshot, financial mutation, external truth or grant is produced.

## Genesis

Require fresh unborn instance and exact program declarations. FundingInput records
must exactly match the authenticated context's external input set, with unique
IDs/ownership and no prior consumption. Initial balances equal those inputs allocated to their bound owners.
The exact registry-derived Genesis constructor in record-generation-04.md replaces
initialState; there is no implicit initial issuance or prefilled liability table.
No prior same-action transfer receipt is fabricated for an unborn ledger. Initial
supply sets imported to exact inventory and issued/burned to0. InitialWork equals
the signed initial allocation; subtract all actual body/helper/operation costs.
Insert one instance and exact bounded initial financial state. Missing funding/
creation derivation rejects; GenesisPolicy alone cannot authorize it.

## Activate

ModePolicy plus exact live authority/spec/key window permits Dormant→Live or
Paused→Live only. Revoked never activates. Preserve all other financial state,
duties/caps/work/history except actual charge.

## Pause

Require Live and ModePolicy. Set Paused; preserve balances/claims and declared
servicing/recovery rights. The fixed profile defines which later operations are
permitted in each mode; it cannot silently treat pause as debt discharge.

## Revoke

Require non-Revoked and specific revocation authority/ModePolicy. Set instance
Revoked, retaining reason in effects. Do not mutate a global spec registry,
reset authority or erase duties. Existing recovery rules remain explicit.

## Migrate

Require exact current source hash/unconsumed predecessor, destination version/
policy and MigrationPolicy. Validate the complete finite claim/resource map:
no duplicate source quantity, all retired claims have successor/discharge,
beneficiaries/denominations/priorities remain unless explicitly authorized, no
backing is copied, no increased authority or work. Change only the declared
program/policy and mapped representation fields; retain consumption lineage and
subtract actual charge. A missing destination rule/schema rejects admission,
not a fallback to the old program. External ledger consumption is still required.

## Signed position calculation and closed claim-map consumption

A SignedDelta is `{negative:Bool,magnitude:UInt256}`; negative zero is invalid.
Its mathematical integer is −m iff negative else m. PositionNumericResult has
exactly eight slots; no absent slot means unchanged. Apply in canonical field
name order to the corresponding Metrics scalar. For opening, use zero values
with the operand's fixed quantity tags as baseline. Adjustment uses current
values. Closing uses current values and the explicit zero/frame requirements.
UInt128 slots must finish0..2^128−1, expiry0..2^64−1, lower/upper signed128 and
NetAmount funding/Pnl within±(2^128−1). Addition uses bounded signed512 intermediate
and one final range check; no wraparound or caller-directed resource retagging.
Zero deltas explicitly preserve a slot. A rule can construct SignedDelta using
ConstructRecord, Bool and UInt256 values; ScalarValue.magnitude/negative exposes
signed typed inputs without an undeclared cast. The product-specific relation
between liquidity/exposure/market observations is still the required typed body.

A claim-map entry never changes parties or quantity merely because they appear in
an arbitrary new record. Its source resolves in the operation's independently
computed affected set. Sum all entries for that source and require exactly its
remaining quantity; each Quantity entry is positive (the exact WholeCondition case in
record-generation-04.md permits zero), and duplicate destination allocation
or backing quanta reject. Currency/resource is exact unless a supplied explicit
NominalConversion matches that source denomination and fixed target asset.

- Carry requires destination=Some, successor=Some, payments empty and a fresh
  destination ID equal successor.id. Preserve obligor/beneficiary/priority/type
  and quantity exactly. An identity/denomination change additionally requires
  the precisely scoped authenticated assignment relation. successor.original=
  successor.remaining=entry.quantity and discharged0. Backing allocation moves
  to that successor once; it is not copied. Source is discharged by that quantity.
- Pay requires destination=None, successor=None and exact prior ClaimPayment
  allocations from source duty obligor to beneficiary, in its resource, or exact
  explicit nominal conversion result. It discharges only entry.quantity. A fee
  receipt is invalid. The caller cannot choose another recipient or amount.
- AuthorizedDischarge requires destination=None, successor=None, payments empty,
  conversion=None and authenticated creditor discharge scope for this exact claim
  and quantity. It records discharge, never a payment or a created asset.

The generic frame also covers claimed-but-unmentioned duties: ordinary operations
keep them unchanged; migration/position closure enumerates the complete affected
set and rejects omissions. Updates preserve original=discharged+remaining;
accrual/new entitlement increases original and remaining by the same amount,
while payment/refund/writeoff reduces remaining and increases discharged. Zero retains the duty; Conditional remains live until its exact owner condition
terminates. Only a terminal zero obligation has Discharged status. Creation of a fresh duty records exact
source rule/authority/origin commitment and original=remaining,discharged0.

This is a proposed generic relation, not proof of valid legal assignment or a
complete permission grammar. The independent full38 signing/authority binding
must establish the specified creation/discharge/assignment judgments over these
exact records; an authorization merely permitting cash Transfer is insufficient.
No accepted financial result may replace that judgment with `allowed=true`.

## Correction04 precedence

Use record-generation-04.md for every fresh or updated field, rule-family dispatch,
explicit share entitlement, persistent stake provenance, Genesis/Migrate and the
resource envelope. This file supplies the operation order/equations where that
correction does not replace them. All generated object images are listed in
field-generation-04.json. Generic 38-operation support still requires exact full38
authority, implementations, cases and correspondence; source text is not execution.
