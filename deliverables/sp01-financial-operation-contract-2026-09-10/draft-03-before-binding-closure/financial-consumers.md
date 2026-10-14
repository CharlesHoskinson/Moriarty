# Deterministic financial operation consumers — proposed draft03

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
They are never accepted from a caller. ObjectChange hashes are over the canonical
records present in the complete retained pre/post snapshots; no missing preimage
is allowed. Unchanged fields/lists are retained. Delta lists use deterministic
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
as a payment. Receipt ID is the structured action/predecessor/operation-index/movement-index
identity's SHA256, with a fixed domain separator `MoriartyMovementV1`; the complete
preimage uses canonical JSON and is retained. It is not a network transaction ID.
`allocate(uses,purpose,source,recipient,resource,required)` scans the ordered distinct
receipt/quantity uses, checks same-action prior origin and exact roles/resource,
exact requested purpose, required receipt class, positive quantities and
sum=required, and increments allocated amounts at most
receipt units. Reusing a receipt in multiple uses in one list rejects; separate
operations may allocate disjoint remainder. Allocation changes no cash a second
time. Ordinary leftover transfer quantity remains a transfer, not a paid debt.
The purpose/class matrix is closed: ShareFee requires Fee; DebtIssuance requires
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

Require asset quantity. FeeQuote computes the fee from its bound basis/constants;
compare to quantity exactly. Execute one fee move. No repayment allocation can
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
transfer receipt is invented. All requested new/changed claims in claimMap must
be validated by the generic continuity rule and admitted claim records/rule,
otherwise reject. Native issuance capability and external backing remain explicit
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

Require fresh ID, positive P, I0, consistent nominal denomination, valid parties
and DebtCreationPolicy. InventoryLoan requires allocation of exact lender→debtor
funding computed by its declared nominal settlement rule. IssuedDebt requires an
admitted explicit issuance effect in this same action, allocated once to this
creation. AuthorizedNonCash requires its distinct signed liability grant and
creates no cash. Insert debt/duty and increase cumulative nominal created by P;
creation caps do not refresh after payment. No initial debt is silently inherited
from a caller's prefilled record.

## DebtAccrue

Require period.start=lastAccrualEnd and valid end/evidence. AccrualQuote returns
sign plus magnitude in the same denomination. For positive delta add to I with
checked O=P+I. For negative delta, the profile must bind an explicit negative-
accrual disposition: reduce I only up to existing I; any excess requires explicit
credit duty creation under that profile's complete declared rule, otherwise
reject AMOUNT_RANGE. It cannot reduce P by default. Update endpoint once; cash
unchanged. A profile implementing only nonnegative accrual rejects negative as
its declared financial predicate, not an undefined operation.

## DebtRepay

Require nominal matches debt denomination and positive value<=P+I. DebtAllocation
returns dP,dI; check bounds and dP+dI=nominal. SettlementQuote returns required
payment and permitted excess in the exact settlement asset/roles. Allocate the
required earlier debtor→creditor payment; any excess must be a separately explicit
fee/refund or carried duty, not silently swallowed. Subtract dP,dI; remaining
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

Require Shares<V,holder>, asset-side quantity and method Deposit or Mint.
VaultConversion computes exact quantities/assetFee under its declared direction;
match operands. Allocate depositor→vault net assets and any separately emitted
fee movement, without double allocation. Increase holder share balance and vault
shareSupply by computed shares; record ShareMint supply delta. Zero shares rejects.
The operation creates exactly the declared proportional/other profile share claim;
no token issuance is inferred. Conversion basis/fees/loss assumptions stay in the
reviewed profile, not a generic1:1 default.

## ShareBurn

Require Shares<V,holder>, available shares and method Withdraw or Redeem.
VaultConversion computes exact assets/shares/fee; match operands. Allocate prior
vault→beneficiary net output and explicitly separated fees. Decrease holder shares
and vault supply by computed shares. Actual output funding is mandatory; NAV alone
is insufficient. Any pending/other claim remains unless explicitly mapped.

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
The rule computes the numeric metric values for its fixed kind; the consumer
matches requested metrics and checks mandatory invariants: range lower<upper,
liquidity>=0; other quantities/types within exact domains; collateral references
exist; funding receipt uses support every new funded margin/stake claim. Insert
position and named duties. Kind-specific formulas are profile bodies, not
arbitrary callbacks. All exposure/claim identities are fixed by the descriptor,
never chosen in numeric rule outputs.

## PositionAdjust

PositionAdjustment returns numeric deltas matching changes. Add each to the same
metric slot in canonical field order with its exact unsigned/signed/indexed bounds.
Check lower<upper for range kind, margin/funding rules and exact earlier funded
receipt uses. Signed P&L/funding remain claims/exposures, not spendable balances.
Negative resulting unsigned quantities reject. Claims cannot be removed or
reassigned by a numeric delta; explicit debt/claim operations are required.

## PositionClose

PositionClosing fixes closing quantities. Check every attached claim is either
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
exact earlier sender→escrow allocation and fresh reservation/duty q; q0 requires
reservation None and creates no cash duty. No remote cash/acceptance is inferred.

## MessageReceive

Require Pending and authenticated exact payload/domain/recipient/finality/conflict
evidence before MessageResolution. The reference settlement form releases local
reservation q and moves escrow→recipient q, then Received; q0 changes status only.
No remote settlement is claimed unless its separately bound evidence relation
establishes it. Other messaging forms require another explicit profile relation.

## MessageRefund

Require Pending and authenticated exact absence/conflict/recovery evidence under
the bound model; local Pending alone is insufficient. MessageResolution must
permit refund. Release q reservation and move escrow→bound refundRecipient q;
set Refunded and close the local duty. Keep history; no second payout/currentness
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
record-date beneficiary payment in the reward asset. Decrease pending/increase paid
and discharge only that amount. No second cash credit; unpaid reward persists.

## Slash

SlashQuantity computes the exact requested positive share units within current
slashable balance; check specific authority/evidence. Reference burn form reduces
share account units, corresponding holder share balance and vault supply once,
and increases lostShares. Existing earned reward and unrelated debt frame unchanged.
A different loss mechanism must have a separately explicit profile consumer;
calling this burn form a universal staking/slashing rule is forbidden.

## Rebase

RebaseRate computes nonnegative newRate matching operand. Update rate only;
share units/supply and nominal debt do not change. This reference is share-rate
revaluation, not a token balance rebase; the two have separate economic observations.

## Observe

ObservationPolicy validates exact immutable authenticated observation reference,
field/type/domain/freshness. Append observation-use record only. No new observation
snapshot, financial mutation, external truth or grant is produced.

## Genesis

Require fresh unborn instance and exact program declarations. FundingInput records
must exactly match the authenticated context's external input set, with unique
IDs/ownership and no prior consumption. Initial balances equal those inputs
allocated to their bound owners plus explicitly admitted initial issuance; all
initial liabilities and claims need their exact creation rule and authority.
No prior same-action transfer receipt is fabricated for an unborn ledger. Initial
supply tracks only declared issuance, not imported inventory. InitialWork equals
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
