# What is missing from the Spending surface

Subject: Part I section 3 of `2026-09-11-defi-kernel-sdk-interface-design.md`.

Two conclusions govern the rest of this document.

**On (a), "no approvals at all".** It is not achievable as an absolute, and the design's own routing
premise is what defeats it. But the guarantee is worth keeping, because the distinction it draws is
exactly the distinction that decided who lost money in the two largest incidents in this product
category. What must change is not the guarantee but the sentence that claims it leaves "nothing to
revoke, nothing to audit" — and, more importantly, the sentence that says caps are enforced "where
value moves", which is the formulation that lets granted authority go unmetered.

**On (b), settlement-asset price movement.** Section 3 denominates every cap in an asset whose value
against everything being spent is unfixed, unreferenced, and untimed. This is not a rounding concern:
it makes "bounded loss" a bound of unknown size, in both directions, precisely during the market
conditions where a cap is the only thing protecting the user. The payments industry treats a locked
rate as a *priced product with a validity window and a void-outside-band rule*, and that structure is
what the kernel is missing.

---

## GAPS

Ranked by how badly the absence hurts.

### 1. Caps are measured where value moves, so authority the kernel grants is never metered — and the routes that need such authority are the normal case, not the exception

Section 3 states two things that combine badly. It says "there are no approvals" anywhere, on any
chain, for any counterparty; and it says caps are "enforced where value moves, not in the interface."
If the first were true the second would be harmless. The first is not true.

**Why standing authority cannot be eliminated.** Three structural facts, in increasing order of
severity for this design:

*Asynchronous fills need authority that outlives the signing moment.* Every deployed intent system
pulls the user's input asset at fill time — minutes or hours after signing, by a party the user never
named. UniswapX reactors "pull input tokens from the swapper to the fillContract using permit2
`permitWitnessTransferFrom` with the order as witness." CoW Protocol goes further and requires a
*pre-existing* allowance to a single relayer before any order can settle; the interface "prompts you
to give vault relayer an 'unlimited' allowance for the sell token."

*The primitive that delivers expiring, signature-scoped authority is itself bought with an unlimited
standing one.* Permit2 is the mechanism that makes per-intent authority possible — but first "users
must approve the `Permit2` contract through the specific token contract," "commonly for the maximum
`uint256` amount."

*The one primitive that truly grants no standing authority exists only where the asset issuer put it
there.* ERC-3009 carries an exact amount, a `validAfter`/`validBefore` window and a random nonce, and
grants no allowance at all — but it "requires explicit token-contract support—it cannot be retrofitted
onto existing ERC-20 contracts." The same holds for ERC-2612: "older tokens like WETH, MKR, and the
original USDT do not [implement it]... even when the consuming protocol supports it," because the
bytecode is immutable. For those assets the documented fallback is to "route through Uniswap's Permit2
contract" — back to the unlimited approval above. Note also that ERC-3009's exactness cuts the other
way: it authorises a *precise value*, not a ceiling, so it cannot express a reservation at all.

Outside one execution model the shape differs and the hazard is worse: a Solana token account "stores
one current delegate and one delegated amount at a time," and "approving a new delegate replaces the
previous delegate and allowance on the account" — so a kernel that quietly grants itself a delegate
destroys whatever delegation the user already had. And EIP-7702, the newest delegation primitive, is
persistent by construction: "A poorly implemented delegate can allow a malicious actor to take near
complete control over a signer's EOA," and "There is no safe way to provide this interface."

**Why the unmetered path matters more than the wording.** This is the highest-severity finding in the
survey of shipped policy engines. Rhinestone's spending-limit policy enforces
`alreadySpent + approvedAmount <= spendingLimit` across `transfer`, `transferFrom`, `approve` **and**
`increaseAllowance`. Alchemy's allowlist module likewise applies its limits to `approve` as well as
`transfer`. They do this because a policy that meters only settlement is trivially escapable: the
delegate grants itself authority inside the cap and draws on it outside the cap.

The same hole exists for authority exercised by *signature* rather than by movement. SmartSession
gates signature authority as a surface separate from execution authority
(`enableERC1271Policies`/`disableERC1271Policies`, `erc7739Policies`), and ZeroDev's signature policy
bounds the `allowedCallers` "allowed to validate messages signed by the signer" — because on any venue
that settles off-ledger, a signature *is* a spend, and no local execution occurs for a cap to
intercept. Section 7's promise not to "sign anything you cannot see" addresses opacity, not metering.

**Concrete failure.** A route requires a bounded allowance on a venue that pulls asynchronously. The
allowance is granted within the cap. The intent completes, reports `Settled`, and the authority is
not fully consumed. Nothing metered the remainder: the period cap shows it as unspent, and the
counterparty can draw on it after the intent that justified it has expired. The user's bounded loss
is now the cap *plus* the residual. The variant through signatures is worse because nothing local
happens at all: the kernel signs an off-ledger order, no value moves, no cap is touched, a second
intent is authorised against a budget already committed, and the two together exceed what the user
signed.

**Evidence that this is the realised loss channel for exactly this product category.** LI.FI, 16 July
2024: ~$11.6M from 153 wallets, and the incident report is explicit that "the vulnerability was
limited to infinite approvals and did not affect finite approvals, which is the default setting within
the LI.FI API, SDK, and widget." Socket/Bungee, 16 January 2024: ~$3.3M from ~700 victims; attackers
"targeted wallets with infinite approvals to Socket contracts," via an unvalidated parameter on a
newly added route that let them inject a `transferFrom`. Both were routers/aggregators — the role this
kernel plays — and in both a freshly deployed component turned a standing approval into direct user
loss. revoke.cash's register shows the pattern at scale: BadgerDAO Dec 2021 $120.3M, Radiant Oct 2024
$60M, bZx Nov 2021 $55M, with the principle stated plainly: "If that contract gets exploited,
attackers can use your approval to steal everything you approved."

The mitigations are cheap and already exist in the primitives: Permit2 allowance entries carry an
`expiration` — "timestamp at which a spender's token allowances become invalid" — and an `amount`
where only `type(uint160).max` means unlimited, and `approve()` with expiration `0` "expires
permissions at `block.timestamp`." Finite-and-expiring is a different risk class from infinite, and
the LI.FI report is the citation that the distinction decided the outcome.

### 2. Every cap is denominated in the settlement asset, but nothing fixes what that asset is worth, when, or against what

Section 3 sets caps "in your settlement asset"; section 1 adds that execution costs everywhere are
"billed to you in your settlement asset, inside the caps you signed." Between quote and settlement two
prices move: the settlement asset against the assets actually being spent, and the execution-cost
assets against the settlement asset. Part I names no reference, no validity window, no tolerance band,
and no bearer of movement.

The payments industry has a fully worked structure here. Stripe's FX Quotes API exists because
"Payments FX occurs when the presentment currency differs from the settlement currency," and its
properties are the ones Part I lacks:

- **A lock has an explicit, short, enumerated duration** — `five_minutes`, `hour`, `day` — with
  `lock_expires_at`, and `lock_duration: none` for an unlocked informational rate.
- **A lock is priced.** `duration_premium` is "the fee charged for the extended rate quote," tabulated
  by duration and currency group: 0.07%/0.10%/0.20% at 5min/1hr/24hr for major pairs, 0.12%/0.15%/0.30%
  for the rest. Certainty about a future rate is a product with a price, not a free property of a quote.
- **A lock is void beyond a band.** "An extended rate quote created for payments has a 3.5% rate
  threshold... If an exchange rate exceeds these thresholds, the extended rate quote is invalidated,
  with `lock_status` changing to `expired`." Using it then returns a typed error
  (`payment_intent_fx_quote_invalid`) and fires an `fx_quote.expired` webhook. The system *refuses*
  rather than silently repricing.
- **Refunds do not honour the original rate.** Stripe "converts disputed or refunded payments back to
  the presentment currency at the current exchange rate, instead of at the previous rate" — an
  asymmetry documented rather than hidden.
- Where a payment method is slower than even a 24-hour lock, the fallback is explicit: "we use the
  mid-market rate to process the payment."

The valuation side is equally load-bearing, and it is where the design's single-denomination choice
bites. Chainlink's guidance distinguishes market-price feeds from exchange-rate feeds that "report the
internal redemption rates for an asset," warns that "assets with low liquidity or volume can be
volatile or difficult to price," and recommends for stablecoins that protocols "assess upside risk and
evaluate setting an explicit valuation ceiling for each stablecoin, along with mechanisms that provide
proactive peg protection," plus freshness checks and circuit breakers. The failure mode when a cap
trusts a market quote under stress is well documented: an oracle reporting a fully-backed asset at
$0.90 liquidates positions that were never undercollateralised, and those liquidations push the price
lower. During stress "the exchange quote is the noisiest signal available" while issuer redemption at
par is the informative one.

Meanwhile the shipped policy modules deliberately refuse to do what section 3 does. MetaMask's
`MultiTokenPeriodEnforcer` carries a full config *per asset* (token, periodAmount, periodDuration,
startDate), and Rhinestone's policy notes "Every config can allow multiple tokens with its own limit
each." A single-denomination cap is only meaningful for assets that have a reliable price against that
denomination.

**Concrete failures, four, all different.**

*The cap silently tightens.* A user signs a 100-unit cap for an outcome quoted at 95. The settlement
asset softens 6% against the assets being moved before the last leg lands; true cost is now ~101.
Under "caps are enforced where value moves," the movement does not happen — so a user whose economics
never changed gets a failed or partial outcome because the *unit of account* moved, with no way to
tell whether they were protected or mispriced.

*The cap silently loosens.* The mirror case, and worse because it is invisible: the settlement asset
strengthens and the signed cap now authorises materially more real value than intended. Nothing
prevents it and nothing reports it. "Bounded loss" drifts with its unit.

*The execution-cost leg is unhedged.* Costs are incurred in assets the user never holds and converted
into the settlement asset; two prices move between signing and landing. Whether the user or the kernel
absorbs that is unstated, so the "cost" shown at quote time was never a commitment.

*An unpriceable asset makes the cap meaningless.* A route touches a thin asset. To check the cap the
kernel must value it, and the only mark available is manipulable. Either the outcome is refused — for
a reason section 1 says the user need not think about — or the cap bounds nothing real and the user
can be drained inside a cap that was technically respected.

### 3. Caps bound totals but not the size of one movement, not the rate, and not the epoch — and there is no size above which delegation stops being enough

Section 3 bounds value "per intent, per period, and in total". Three omissions, each with its own
failure, plus a fourth that nobody in the industry has solved and that this kernel is uniquely placed
to solve.

*No single-movement bound.* Coinbase's own accounting documentation states the consequence: "No
per-transaction cap exists beyond the period allowance" — one transaction can drain the entire
remaining allowance. Systems that have the bound ship it as a separate knob: ZeroDev `valueLimit`,
Rhinestone `valueLimitPerUse`, Biconomy `maxAmount` per call. A per-*intent* cap is not the same
thing, because one intent legitimately contains many movements.

*No rate bound.* The policy bounds quantity, never velocity. Fraud containment everywhere is
per-period counting, not per-transaction size: Stripe Radar exposes velocity as first-class attributes
across four families × six subjects × four windows
(`authorized_charges_per_card_number_{hourly,daily,weekly,all_time}`, `total_charges_per_customer_*`,
and so on). ZeroDev ships rate limiting in two distinct semantics — without reset, "one UserOp per
`interval`, up to `count` times"; with reset, "`count` UserOps within an `interval`," indefinitely —
and the choice between them is not inferable from a single number. ERC-7265 standardises the
protocol-side analogue: a circuit breaker that "triggers a temporary halt on protocol-wide token
outflows when a threshold is exceeded for a predefined metric," per asset, with the integrator
choosing to "delay settlement and temporarily custody outflows during the cooldown period, or revert
on attempted outflows."

*No period anchor, no carry-over rule.* Every shipped implementation makes the epoch an explicit field
rather than an implied calendar. Safe carries `resetTimeMin` and `resetBaseMin` and snaps with
`lastResetMin = currentMin - ((currentMin - lastResetMin) % resetTimeMin)`; Coinbase uses `start` +
`period` to "set a deterministic schedule infinitely into the future"; MetaMask computes
`currentPeriod = (block.timestamp - startDate)/periodDuration + 1`. Carry-over is stated, not
inferred: "any unused tokens are forfeited once the period ends." Safe also treats the non-renewing
case as a distinct mode (`resetTimeMin == 0` never auto-renews).

*No threshold above which a delegated capability is insufficient.* This is the survey's clearest
negative result. Threshold or quorum approval above a value exists **only** in custodial and
server-side engines — Turnkey's `effect`/`consensus`/`condition` with `rootQuorum{threshold, userIds}`,
"codifying who must sign"; Fireblocks' policy encoding `amount`, `amountCurrency`, `amountScope`
(`SINGLE_TX` | `TIMEFRAME`), `periodSec`, `amountAggregation`, `authorizationGroups` with threshold
`th`, and actions `ALLOW` / `BLOCK` / `2-TIER` — and in **none** of the on-chain modules surveyed.
And those server-side engines cannot do the other half: Turnkey policies "are evaluated per each
request/activity, not cumulatively," and Privy's model is per-call ALLOW/DENY with no cumulative
limit. The two halves of a real spending control live in two systems today and nobody ships both. A
kernel that owns cumulative state *and* the authorisation path is the only place they can meet.

**Concrete failures.** A user grants a trading application a generous monthly budget — the entire
point of delegated spending. The application's key is compromised or its strategy has a bug. Every
movement is inside policy, every counterparty is allowlisted, and the whole month leaves in one block.
Caps performed exactly as specified and the user is fully drained; revocation is forward-looking and
there is nothing left to protect. Separately, on a kernel spanning many clocks, two legs of one period
disagree about which epoch they fall in, so a cap is double-spent across a boundary or work stalls
there for no visible reason. And with carry-over unstated, an application idle for eleven months may
or may not be able to spend a year's budget at once — neither user nor developer can know which. The
deeper failure is the false choice: set the period cap low enough to survive compromise and the agent
cannot work; set it high enough to work and one compromise takes everything.

### 4. Failed work still costs money, execution cost is billed as a maximum and reconciled later, and the spread inside it is unstated

Two related things the interface currently lets a user get wrong.

*Failure is not free.* The `PostOpMode` enum says it in as many words — `opReverted`: "User op
reverted. Still has to pay for gas." The EIP repeats it: "UserOperation reverted. paymaster still has
to pay for gas." If the billing step itself reverts, the whole operation reverts (`AA50 postOp
reverted`) and the sponsor is still charged. Unused gas is not even fully refunded:
`UNUSED_GAS_PENALTY_PERCENT = 10` applies to unused `callGasLimit` and `paymasterPostOpGasLimit` above
a `PENALTY_GAS_THRESHOLD = 40000`. Charging after execution is itself a documented loss vector: a
malicious bundler can revoke the collection allowance before `postOp`, so collection fails while "the
bundler still gets paid for submitting the failed UserOperation" — the recommended mitigation is to
take payment during validation, not after. Alchemy states the liability plainly: "If the
post-execution ERC-20 transfer fails, the transaction reverts, but you remain liable for the gas
costs."

Section 3 says exhaustion "never becomes debt" and section 7 says the kernel "will not bill you for an
operator's mistake" — true and good, but neither tells the user the thing that is true everywhere: an
attempt that fails is still paid for. Section 6 defines `Refunded` as "your consideration was
returned," which a reader will take to mean a failed outcome is free.

*Cost is a maximum, then a reconciliation, with a spread.* `validatePaymasterUserOp` receives "The
maximum cost of this transaction (based on maximum gas and gas price from userOp)"; `postOp` receives
`actualGasCost`, "Actual cost of gas used so far." The reference implementation pre-charges
`requiredPreFund + refundPostopCost * maxFeePerGas`, then refunds the excess *or* charges an overdraft
if it fell short — and the refund leg itself costs roughly `REFUND_POSTOP_COST = 40000`. The rate used
to bill a user in a non-execution asset is a **cache, not a live price**: "The token price cannot be
queried in the validation code due to storage access restrictions of ERC-4337. The price is cached
inside the contract and is updated in the `postOp` stage if the change is >10%", bounded by a
`priceMaxAge`; production contracts revert `OraclePriceStale()` past a staleness threshold. Markup is
bounded but wide — the reference permits 100%–200% (`require(priceMarkup <= 2 * PRICE_DENOMINATOR)`),
published defaults sit at 110% within a 100%–120% band, and provider premiums are off-chain and
per-vendor: Biconomy returns a `premiumPercentage` with a documented example of 13, and Pimlico states
that "the owner takes a fee that is baked into the `exchangeRate` returned by the API." Secondary
surveys put production markups at roughly 5–15%.

The industry's fix is the one Part I should adopt: denominate the user's cap **in the asset the user
pays in**. Pimlico's mode 1 adds a 32-byte *token spend limit* to the paymaster data, because a cap in
execution units does not bound what the user pays once the rate moves.

**Concrete failure.** A user asks for a genuinely hard outcome with a modest cap. Three legs are
attempted; two fail for reasons that are nobody's fault. Budget is materially consumed and the user
received nothing, while Part I led them to believe a failed outcome is free. The developer has no
field to show the cost of failure, so the application either hides it — misreporting, against the
honest-outcomes guarantee — or invents its own accounting. And a caps model that does not reserve for
failed attempts under-reserves: the kernel either exceeds the signed cap or abandons in-flight work,
having promised it would do neither. Separately, the kernel quotes 5 units and actual cost is 2: bill
the maximum and it overcharges 150% on every intent, making the cost statement a fiction; bill the
actual with no reserve and it breaks its own promise that in-flight work completes from a reserve set
aside beforehand. Section 3 caps what an *application* may charge "as a rate and as an absolute
ceiling" but says nothing about the kernel's own spread on execution cost — the fee every user pays
on every intent.

### 5. An amount cap is not a terms cap, so an allowlisted counterparty is a blank cheque

Section 3's allowlist names counterparties and applications. The engines actually deployed treat a
bare address allowlist as insufficient and bound what happens *at* the address: selector-level
allowlists with wildcard resolution (ERC-6900's `AllowlistModule`, Rhinestone's
`ActionData{actionTarget, actionTargetSelector}`), calldata and argument predicates
(`AllowedCalldataEnforcer`, `ExactCalldataEnforcer`, ZeroDev's
`EQUAL/GREATER_THAN/LESS_THAN/GTE/LTE/NOT_EQUAL` conditions, Turnkey's `eth.tx.contract_call_args`),
and cumulative limits on a *decoded argument* rather than on transfer value (`UniActionPolicy`'s
`ParamRule{condition, offset, isLimited, ref, usage, limit}`). Several also bind the counterparty
exactly rather than by list — Biconomy's `require(recipient == recipientCalled)`.

At outcome level the missing bound is the **price**. An amount cap plus an allowlist authorises
execution at any rate whatsoever provided the total stays under the cap. This is the protection every
intent venue does provide: UniswapX orders are a Dutch decay between `startAmount` and `endAmount`,
the user's floor, non-executable after the deadline; CoW's settlement contract verifies "that you will
receive at least your limit execution price or better" *before* the relayer may pull any funds.
ERC-7683 is the cautionary note in the other direction: it removed `maxSpent`/`minReceived` because
"If the bound was not tight, orders could incorrectly appear unprofitable and not get filled."

Note also what an allowlist without per-entry caps does: Safe's `to` is unrestricted and Coinbase
enforces "No recipient restrictions", so the single riskiest allowlisted counterparty inherits the
entire budget.

**Concrete failure.** An intent is routed to an allowlisted counterparty at a rate 40% worse than
quoted. The absolute amount is within every cap in section 3, so nothing stops it. The user gets the
outcome they asked for at a price nobody bounded, and the difference is extracted by whoever routed
it. Section 5 says a route that "cannot satisfy the caps, the deadline, the grade or the recovery
requirement is not offered" — price is not in that list, so there is nothing to fail.

### 6. Reserves have no release condition and exhaustion is a silent stall

Section 3 implies a reserve — "work already in flight is completed from a reserve set aside before it
began" — and never says when a reserve is released, or that release may be slow, or what the user's
budget looks like while it is held.

The card networks treat the authorized amount and the captured amount as two separate numbers with an
enforced relationship. A hold sets `requires_capture` with `amount_capturable`; authorizations expire
on a per-network schedule surfaced as `capture_before` (card-not-present typically 7 days, card-present
2–5 days); if capture never happens the funds are released and the payment becomes `canceled`, not
charged; partial capture auto-releases the remainder. Visa's rule is normative about the harm of
over-reserving: the estimate "must reflect the anticipated / average transaction amount," and where it
exceeds the final amount "merchants must process partial reversals for the difference… as soon as the
final amount is known," with issuers required to drop holds promptly. The agent-payments layer has
converged on the same shape: x402's requirements object carries `maxAmountRequired` and
`maxTimeoutSeconds`, and the ecosystem explicitly splits `exact` from `upto` so a seller can "authorize
a maximum and settle only what was used."

The on-chain analogue of slow release is measurable. Across refunds go to a `refundAddress` with a
`refundOnOrigin` choice and are automatic — "The refund is executed on-chain, returning the escrowed
funds to the refundAddress" — but only after `fillDeadline` passes unfilled, and then: "Refunds are not
instant. After a deposit expires, the refund goes through the bundle settlement process," spanning
"several hours." The documentation instructs integrators: "Do not tell users to expect immediate
refunds."

Exhaustion, meanwhile, is invisible by default. `AA31 paymaster deposit too low` means the sponsor is
out of funds, and because bundlers must track cumulative gas per paymaster and reject operations that
would exceed its deposit, exhaustion manifests as operations never being included rather than as any
error. A production instance reads exactly this way: `AA31` with the reporter observing "nodes run good
no error or left behind, but no tx recorded." It is also sticky: deposit and stake are distinct (stake
is "never slashed"), bundlers throttle and ban by inclusion rate (`MIN_INCLUSION_RATE_DENOMINATOR = 10`,
`THROTTLING_SLACK = 10`, `BAN_SLACK = 50`), and `EREP-050` forbids an unstaked sponsor from returning
`context` at all — removing the reconciliation leg of gap 4 entirely. Enforcement is admitted to be
eventually consistent: alerts fire at "50%, 75%, 90%, and 100% of the sponsorship limit," but "there
may be brief periods where the actual spending slightly exceeds the limits you've set."

**Concrete failure.** A user's intent is not filled. Their reserve — deducted against the period cap
before work began — is escrowed for hours. During that window their policy says they have less budget
than they economically do, so a second, urgent intent is refused for insufficient budget while the
funds sit in a settlement queue. The developer has no field to explain this, and the promise that
"running out stops work" fires for a reason that is not running out. Worse, when a cost source is dry,
intents simply stop landing — no outcome, no `Unresolved`, no reason — which is the exact thing
section 6's honest-outcomes model exists to prevent. And if cap accounting is batched rather than
reserved synchronously, the kernel exceeds the cap the user signed.

### 7. Revocation is honest about the past but silent about the present: no inventory, no kill switch, no renouncement, and unspent reserved budget is stranded

"Revocation is forward-looking and says so" is correct and should stay. Its honesty depends on
something section 3 does not provide: a way to see what is already authorised at the moment of
revoking.

Every framework surveyed is forward-looking-only and none resets state or settles the remainder.
MetaMask's manager flips `disabledDelegations[hash]` and reverts with `CannotUseADisabledDelegation()`,
and "simply toggles the disabled boolean; it contains no caveat state reset logic." Coinbase's
`revoke()` disables use "indefinitely" with no refund path, so unspent allowance is stranded under the
permission hash. Counters survive disable and re-enable: re-enabling a SmartSession overwrites the
*config* while leaving `UsageLimitPolicy.used` and `alreadySpent` untouched. Permit2's concepts
documentation does not even describe an early-removal path — expiry is the intended mechanism.

The features that exist *because* forward-only revocation is insufficient alone:

- **Enumeration and live remaining budget.** Safe keeps queryable delegate and token lists explicitly
  "to prevent hidden allowances"; Coinbase's `getCurrentPeriod()` returns `PeriodSpend{start, end,
  spend}`; ERC-7715 defines `wallet_getGrantedExecutionPermissions`.
- **A kill switch distinct from per-grant revocation.** `NonceEnforcer.incrementNonce` "invalidates all
  previous delegations with the old nonce" in one call; Safe has `removeDelegate(delegate,
  removeAllowances)`.
- **Spender-side renouncement.** Coinbase ships `revokeAsSpender()` alongside `revoke()`.
- **Stable identity for otherwise-identical grants.** Coinbase's `salt` "differentiate[s] between
  permissions with the same parameters"; ERC-7710 has `Delegation.salt`; Rhinestone has `Session.salt`.
- **A scope whose only power is to *remove* authority.** `ApprovalRevocationEnforcer` / ERC-7715's
  `token-approval-revocation`, with per-flag fields down to `permit2Lockdown` and
  `permit2InvalidateNonces`.

**Concrete failures.** (i) A user cannot answer "what have I authorised, and how much is left this
period", so authority accumulates unexamined — the documented reason Safe treats enumeration as a
safety feature. (ii) During an incident the user revokes one capability at a time while a compromised
application spends the ones not yet reached. (iii) A compromised application cannot renounce its own
authority, so remediation blocks on the user being online. (iv) Two recurring arrangements with
identical terms collapse into one budget, and revoking one silently kills both. (v) A user revokes,
the interface confirms, and three in-flight intents then settle against their caps — correct per
specification and contrary to everything the word "revoked" implies. Section 3 promises the kernel
"never reports that it did" reach back; it never promises to report what remains.

### 8. Sponsorship has no bounds, and a sponsor's refusal is indistinguishable from the user's own

Section 3 lets a policy name "who pays execution costs, where a sponsor is covering them," and
section 5 exposes `grantSponsorship`/`revokeSponsorship`. Nothing says what a sponsor may bound.

Every production sponsor bounds several things at once. Alchemy's published rules cap "total spend,
spend per sender, and spend per transaction", cap "total count and count per sender", carry sender
allow/blocklists and start/end dates, and offer a webhook for synchronous per-operation approval with
`approveOnFailure` defaulting to `false`; field names include `maxSpendPerUoUsd` and
`maxSpendPerSender`. Pimlico's limit "the global amount of sponsorships, the amount of sponsorships per
user, and per user operation." Coinbase's expose "contract allowlists, per-user limits, and global
spend caps." ERC-7677's Security Considerations direct applications to proxy sponsorship through their
own backend and re-simulate before sponsoring.

The threat is documented, not hypothetical: named attack classes include "Drain the Paymaster's deposit
by spamming invalid operations" and "Trick the Paymaster into sponsoring high-cost or reverted calls,"
and an unguarded validation makes a sponsor "a free gas bank for attackers." Per-address caps are
explicitly insufficient because identities are free — the recommended defence is identity verification
against Sybil attacks. EIP-7702 adds a sponsor-griefing vector with no protocol remedy: an authorized
account can make relayers "spend gas without being reimbursed by either invalidating the
authorization... or by sweeping the relevant assets out of the account." And a sponsorship signature
pins the cost numbers together with `validUntil`/`validAfter`, so any re-quote invalidates the
sponsorship (`AA32` on-chain).

Rejections also need to be attributable. The typed errors a sponsor returns — "Policy's max spend per
spender exceeded.", "User operation cost exceeds specified spend limit", "policy hasn't started" /
"policy has ended" — occur at request time, not as on-chain reverts.

**Concrete failure.** A developer sponsors execution costs, as section 3 invites. One user, or one
script with a thousand fresh identities, submits work that fails. Under gap 4 every failure is still
billed. The sponsor's budget is gone in minutes; they had no per-user, per-period, per-operation or
rate bound to set because the interface offered none; and the kernel's guarantee that "a sponsor's
shortfall never converts into your liability" lands the entire loss on the developer. Then gap 6:
users stop settling and nobody can say why — and because the user cannot tell whose cap stopped them,
they will try again, and again.

---

## ADDITIONS

Minimum set. Each is chain-free and paste-ready. Six guarantees plus one amended list and two
corrections; each covers several of the gaps above.

### A. The policy list gains four bounds

Replace the existing list in section 3 with:

> A policy sets, in your settlement asset:
>
> - the most that may leave your accounts in one movement, per intent, per period, and in total;
> - how often value may leave at all;
> - when each period begins, and whether budget unused in a period is forfeited at its end or carried
>   into the next;
> - which counterparties and applications may be paid at all, and the most each may be paid;
> - the worst terms you will accept — the least you will receive, or the most you will give, per unit
>   of what you are buying;
> - a size above which a delegated capability is not sufficient authority and the identity itself must
>   authorize the movement;
> - the most any named application may charge you, as a rate and as an absolute ceiling;
> - who pays execution costs, and where a sponsor is covering them, the bounds that sponsor has set.
>
> A policy whose period has no stated beginning, or whose carry-over is unstated, is refused when it is
> written. It is never interpreted later.

*Covers gaps 3, 5, 8. Without the single-movement bound, one compromised action empties a period
instead of costing one tick. Without the rate bound, an entire period's budget can leave in one block
with every individual movement inside policy. Without a stated period beginning and carry-over rule,
two legs of one intent disagree about which period they fall in, and nobody can say whether an
application idle for a year may spend a year at once. Without the terms bound, an allowlisted
counterparty may transact at any price whatsoever so long as the total is under the cap. Without a
size above which delegation is insufficient, every delegated budget is a false choice between too
small to work and too large to survive a compromise. Without the sponsor's bounds being part of the
policy, a sponsor is drained by design and the user cannot tell whose cap stopped their work.*

### B. Authority is charged when it is created, not when it is used

> **Authority is charged when it is created.** Every authority the kernel creates on your behalf is
> charged against your caps at the moment it is created, for the most it could move — not when it is
> exercised, and not only where value finally moves. This holds for authority a counterparty exercises
> rather than you, for authority that lasts longer than the step that needed it, for authority created
> by signature rather than by movement, and for the cost of execution, which is reserved at its
> worst case before work begins.
>
> Authority that is used for less than it could have been, or that ends unused, is released back to
> your caps promptly, and the release is reported. The kernel never holds a reserve after the work it
> was set aside for has ended.

*Covers gaps 1, 4, 6. This is the single most important addition. Without it a cap is escapable
without ever being violated: authority granted inside the cap is drawn on outside it, and authority
created by signature never passes the point where caps are said to be enforced. Without the worst-case
reserve, the kernel either exceeds the cap the user signed or abandons work in flight, having promised
neither. Without prompt release, a user's budget is smaller than their economics for hours and an
urgent second intent is refused while their own funds sit in a settlement queue.*

### C. Authority never outlives its intent, and authority that cannot be bounded is refused

> **No authority survives its intent.** Authority the kernel creates may be exercisable by a named
> counterparty while the intent that created it is alive. It never lasts longer. Every authority the
> kernel creates carries a maximum amount and an expiry no later than the intent's own; the kernel
> creates no authority without both.
>
> Where an outcome could only be reached by authority the kernel cannot bound this way — authority
> without a ceiling, without an expiry, or exercisable after the intent ends — the outcome is refused
> before anything moves, and the reason given is the authority, not the price or the route.
>
> While authority is live it appears in your record of live authority, with its holder, its ceiling and
> its expiry. Authority the kernel intended to retire and could not is reported there as unretired,
> never as closed.

*Covers gaps 1, 7. Without this, the guarantee that there are no standing allowances is simply false
wherever a venue settles asynchronously or an asset offers no exact-amount authorization, and it is
false invisibly — because the current text tells the user there is "nothing to revoke, nothing to
audit." Two incidents in this exact product category turned a router's bug into direct user loss
through precisely such residuals, and in one of them only the wallets with unbounded authority lost
money. Without the refusal, the kernel quietly does the thing it promised never to do; without the
record, the user cannot discover it.*

### D. A cap is a quantity in a named unit, at a named time, with a band

> **A cap has a unit, a moment and a band.** Every cap is a quantity of your settlement asset. A quote
> states the reference used to value anything else against that asset, when that reference was taken,
> how long the quote is good for, and who absorbs movement in it. A longer validity window may cost
> more, and what it costs is part of the quote.
>
> Movement in the reference within the band the quote states is absorbed by whoever the quote says
> absorbs it. Movement outside the band voids the quote: the intent is refused before value moves, and
> is never repriced against a cap you signed under a different assumption.
>
> The kernel will not value an asset against your settlement asset on a reference it cannot obtain
> fresh, and will not move an asset it cannot value. Where value is returned to you, it is returned in
> the amount actually recoverable, and the quote that priced the outgoing movement does not govern it.

*Covers gap 2. Without a named reference and a moment, "the most that may leave, in your settlement
asset" is not a measurable quantity during exactly the conditions where a cap is the only protection
the user has. Without a band, a user whose economics never changed loses an outcome because the unit
of account moved — or, unreported and worse, their signed cap comes to authorize materially more real
value than they intended, so bounded loss drifts with its unit. Without the refusal to price the
unpriceable, a user can be drained inside a cap that was technically respected.*

### E. An attempt that fails still costs, and the cost of failure is reported

> **Work that fails still costs.** An attempt that does not reach the outcome can still consume budget.
> The kernel charges what attempts actually cost, inside the caps you signed, and reports it whether
> the outcome was reached or not. It never charges you for a cost caused by an operator's fault, and it
> never lets a shortfall become a debt.
>
> A returned consideration is not a refunded cost. Where an outcome was not reached and your
> consideration came back, the cost of attempting it is stated separately.

*Covers gap 4. Without it, every reader of Part I concludes that a failed outcome is free, because
`Refunded` says the consideration was returned. It is not free anywhere: an attempt that reverts is
still paid for, and even unused allowance is not fully recovered. A developer with no field for the
cost of failure either hides it — misreporting, against the honest-outcomes guarantee — or invents
private accounting; and a caps model that does not expect it under-reserves.*

### F. Budget, reserves and the ability to pay are readable state

> **What is live is readable.** At any moment you can read the authority outstanding against your
> identity, the budget reserved against it, the budget remaining in the current period and when that
> period ends, and for each reserve the condition that releases it and when that is expected.
>
> A reserve awaiting release is reported as reserved, never as spent and never as available. Where the
> kernel cannot source execution costs on a path you need, that is reported as a refusal with its
> reason. It is never presented as work that is merely slow.

*Covers gaps 6, 7. Without it, exhaustion of a cost source is indistinguishable from work in progress
— the failure mode observed in production, where clients look healthy and nothing settles — which is
the one thing the honest-outcomes model exists to prevent. Without a readable remaining budget and
inventory, a user cannot answer "what have I authorized and how much is left", which is the documented
reason a major smart-account implementation treats enumeration as a safety feature rather than a
convenience.*

### G. Revocation states what was live, reaches everything at once, and can be exercised by the holder

Extend the existing revocation property with:

> Revoking states what was still live when it took effect: the intents already authorized, and the
> budget already reserved against them. A single withdrawal that reaches every capability at once is
> always available. A capability's holder may renounce it without you, and the kernel never requires
> you to be present to stop something you have already granted.
>
> Budget reserved for work that a revocation stopped is released to your caps, not stranded. A
> capability re-issued under a name you have used before begins with a stated budget; the kernel never
> silently inherits or silently resets what an earlier capability of that name had spent.

*Covers gap 7. Without the statement of what was live, the user takes their next decision believing
spending has stopped when three intents are still settling. Without the wholesale withdrawal, incident
response costs one action per capability while the capabilities not yet reached keep spending. Without
holder renouncement, a compromised application cannot stop itself and remediation waits for the user
to come online. Without the re-issue rule, a budget either arrives already spent or silently doubles —
the observed behaviour of every framework surveyed, none of which resets or settles state on
revocation.*

### H. Two corrections to existing text

1. In section 3, delete "There is nothing to revoke, nothing to audit and no infinite-approval
   exposure, because the authority to move a specific amount exists only inside a specific intent and
   expires with it." Replace with: "Authority to move a specific amount exists only inside a specific
   intent and expires with it. What is live is on your record while it is live, and what cannot be
   bounded that way is refused rather than granted." *The deleted sentence is the only load-bearing
   falsehood in section 3: it tells the user that auditing is unnecessary, which is the precise
   condition under which an un-auditable residual does harm.*

2. In section 3, replace "Caps are enforced where value moves, not in the interface" with "Caps are
   enforced where authority is created and where value moves, not in the interface." *The existing
   phrasing is the escapable formulation; the rest of the paragraph's examples still hold verbatim.*

---

## REFUSE AND SAY SO

Sentences for section 7.

- **It will not create authority it cannot bound.** Authority without a ceiling, without an expiry, or
  exercisable after the intent that needed it has ended, is not created — the outcome is refused
  instead, however good the price.
- **It will not price a cap on a reference it cannot obtain fresh**, and will not move an asset it
  cannot value against your settlement asset. An outcome that can only be reached on a stale or
  unobtainable mark is refused, not attempted.
- **It will not reprice a void quote.** Where the reference moves outside the band the quote stated,
  the intent is refused. The kernel does not substitute a new price into a cap you signed under a
  different one.
- **It will not tell you that failed work was free**, and will not present a returned consideration as
  a returned cost.
- **It will not present an exhausted budget, or a cost it cannot source, as work in progress.**
- **It will not accept a policy it cannot enforce**, including a period with no stated beginning, a
  carry-over it must guess, or a bound a path cannot honour.
- **It will not hold a reserve after the work it was set aside for has ended.**

---

## DO NOT ADD

Rejected, with reasons.

**A streaming or continuous-payment primitive.** Tempting because subscriptions and per-second metering
are real demands, and both shipped designs are well documented. Both import machinery this design has
already ruled out. Sablier Flow models `uncovered debt` explicitly and lets a stream run into deficit —
a direct contradiction of "running out stops work; it never becomes debt." Superfluid avoids debt only
by requiring a collateral buffer (four hours of flow on most networks, flat minimums elsewhere) plus a
solvent → critical → insolvent lifecycle, a patrician period, and third-party liquidators who take the
buffer. Section 3's pre-funded reserve already picks the non-debt side of this trade; a recurring
*authority* with a period and an anchor (addition A) delivers subscriptions without importing
liquidation into Part I.

**Raising a cap mid-flight (incremental authorization).** The card networks have it — capped at ten
attempts, each bounded by the greater of 500 USD or 500% of the prior amount — and it is the obvious
answer to an under-estimated reserve. It contradicts the first standing guarantee directly: you cannot
lose more than the cap carried in the intent you signed. Note that even where it exists, increments do
*not* extend the validity window. The correct answer is a new intent.

**Settling above the authorized amount (overcapture).** Shipped, bounded by network and category —
restaurants +30%, car rental the greater of +15% or 75 USD — and it would smooth the quote-to-settlement
gap. It breaks bounded loss for a convenience. Reserve the headroom under addition B instead, and
release what is unused.

**Chargebacks or reversal of a settled outcome.** Users will expect it and the analogy invites it. The
settlement layer does not offer it: a custodial processor's own stablecoin product records dispute
support as "No", manual capture as "Not supported", and refunds as new transfers that are "always
returned as stablecoins to the customer's original wallet" — with the refunded asset not necessarily
the one paid in. Promising reversibility the settlement layer cannot deliver would violate honest
outcomes. Addition D's last sentence carries the honest version.

**Per-asset cap tables, per-action allowlists, calldata or argument predicates.** These are the richest
part of the shipped policy surface and the temptation is strongest here. They are all descriptions of
*routes*, and Part I names no routes by design; importing them would put venue structure in front of a
reader who is promised outcomes. The outcome-level equivalents already earn their place: the terms
bound in addition A, and the refusal to move an unvaluable asset in addition D.

**Mandated escrow per action.** The natural way to implement addition B, and the wrong thing to
*specify*. The cross-chain intent standard removed per-order escrow precisely because it
"unnecessarily required escrow-per-order, incompatible with fill-first protocols" — mandating it in
Part I would outlaw whole classes of route for an implementation detail. Specify that authority is
charged when created and released when unused; leave how to hold it below the line.

**Notifications and spend alerts as a guarantee.** Nothing surveyed implements a notification or
step-up confirmation above a threshold as an enforceable policy knob, and where alerting does exist it
coexists with admitted overspend — alerts at 50/75/90/100% of a limit alongside the concession that
"there may be brief periods where the actual spending slightly exceeds the limits you've set." An alert
is not enforcement. Addition A's threshold above which the identity itself must authorize is the
enforceable version; a notification surface belongs to applications.

**Letting an application ask for a cap increase.** The grant-time counter-offer exists in the standards
(`isAdjustmentAllowed`, `justification`), and it is genuinely useful — in the wallet, at grant time. A
call that lets an application request more budget mid-flight converts every compromised application
into a phishing surface aimed at the one control that protects the user. Policy changes come from the
identity holder.

---

## SOURCES

### Approvals and standing authority

- https://github.com/Uniswap/permit2 — "users must approve the `Permit2` contract through the specific
  token contract"; AllowanceTransfer = time-bound standing allowance, SignatureTransfer = single-use
  with no standing allowance; non-monotonic nonces; expiring approvals framed as the security feature.
- http://developers.uniswap.org/llms.mdx/docs/protocols/permit2/overview — the prerequisite approval is
  "commonly for the maximum `uint256` amount"; AllowanceTransfer holds "a standing, time-bound
  allowance inside the Permit2 contract"; unordered nonce bitmap; signature deadline.
- http://developers.uniswap.org/llms.mdx/docs/protocols/permit2/concepts/allowance-transfer and
  https://developers.uniswap.org/contracts/permit2/reference/allowance-transfer — `PermitDetails{token,
  amount(uint160), expiration, nonce}` and `PermitSingle{spender, sigDeadline}`; expiration is the
  "timestamp at which a spender's token allowances become invalid"; `expiration = 0` expires at
  `block.timestamp`; `type(uint160).max` means unlimited; allowance expiry is independent of the
  signature deadline; **no early-removal path documented** — expiry is the intended mechanism.
- https://eips.ethereum.org/EIPS/eip-2612 — permit motivation; `deadline` (settable to `uint(-1)` for
  non-expiring); nonce replay protection; front-running and relayer-censorship warnings; the EIP-20
  approval race condition (SWC-114) still applies.
- https://eco.com/support/en/articles/12005190-erc-2612-permit-explained-gasless-token-approvals-on-ethereum
  — WETH, MKR and original USDT do not implement permit and cannot, since it lives in the token's own
  immutable bytecode; documented fallback is to "route through Uniswap's Permit2 contract."
- https://eips.ethereum.org/EIPS/eip-3009 — `transferWithAuthorization` /
  `receiveWithAuthorization` grant no standing allowance; authorize an **exact** value, not a ceiling;
  `validAfter`/`validBefore` plus a random 32-byte nonce chosen specifically to allow unordered
  concurrent authorizations; `cancelAuthorization`; the receive- variant checks the caller is the payee
  to prevent front-running; requires explicit token-contract support and cannot be retrofitted.
- https://docs.cow.fi/cow-protocol/reference/contracts/core/vault-relayer — allowances centralised in
  one relayer "to protect user funds from malicious solvers"; three allowance mechanisms; one relayer
  approval survives protocol upgrades.
- https://docs.cow.fi/cow-protocol/tutorials/cow-swap/swap — "CoW Swap prompts you to give vault relayer
  an 'unlimited' allowance for the sell token"; settlement verifies "that you will receive at least
  your limit execution price or better" before the relayer may pull funds.
- https://github.com/Uniswap/UniswapX — reactors "pull input tokens from the swapper to the fillContract
  using permit2 `permitWitnessTransferFrom` with the order as witness"; Dutch decay
  `startAmount`/`endAmount`; orders non-executable after the deadline; exclusive filler and exclusivity
  override.
- https://eips.ethereum.org/EIPS/eip-7702 — delegation indicator `0xef0100 || address`, persistent by
  design, cleared only by delegating to the zero address; "A poorly implemented delegate can allow a
  malicious actor to take near complete control over a signer's EOA"; "There is no safe way to provide
  this interface"; "The transaction sender will pay for all authorization tuples, regardless of validity
  or duplication"; the authorization signs only (chain id, address, nonce) so it can express no spend
  cap; sponsor-griefing note about relayers spending gas "without being reimbursed."
- https://solana.com/docs/tokens/basics/approve-delegate — "A token account stores one current delegate
  and one delegated amount at a time"; "Approving a new delegate replaces the previous delegate and
  allowance on the account"; delegate may transfer or burn.
- https://solana.com/docs/tokens/basics/revoke-delegate — revocation "clears the token account's current
  delegate and resets the delegated amount to zero"; in the Token Extension Program the delegate may
  also revoke; no partial revocation.

### Realised losses through standing authority, in this product category

- https://li.fi/knowledge-hub/incident-report-16th-july — 16 July 2024, ~$11.6M, 153 wallets; "the
  vulnerability was limited to infinite approvals and did not affect finite approvals, which is the
  default setting within the LI.FI API, SDK, and widget."
- https://www.coindesk.com/tech/2024/01/17/socket-bungee-restart-operations-after-apparent-33m-exploit
  and https://www.theblock.co/post/272986/socket-says-bungee-protocol-exploited — Socket/Bungee, 16
  January 2024, ~$3.3M, ~700 victims; attackers targeted wallets with infinite approvals via an
  unvalidated parameter on a newly added route, enabling an injected `transferFrom`.
- https://revoke.cash/exploits — approval-vector register: BadgerDAO Dec 2021 $120.3M, Radiant Oct 2024
  $60M, bZx Nov 2021 $55M, LI.FI Mar 2022 $600k, Socket Jan 2024; "If that contract gets exploited,
  attackers can use your approval to steal everything you approved."

### Unit of account, quote validity, price movement

- https://docs.stripe.com/payments/currencies/localize-prices/fx-quotes-api — presentment vs settlement
  currency; `lock_duration` of `five_minutes`/`hour`/`day` with `lock_expires_at`; `duration_premium` as
  the priced cost of a lock, with the published fee table (0.07/0.10/0.20% and 0.12/0.15/0.30%); a 3.5%
  rate threshold invalidates a payment lock (`lock_status` → `expired`); typed errors
  `payment_intent_fx_quote_invalid` / `transfers_fx_quote_invalid` and the `fx_quote.expired` webhook;
  refunds and disputes converted back "at the current exchange rate, instead of at the previous rate";
  mid-market fallback for methods slower than the lock.
- https://docs.chain.link/data-feeds/selecting-data-feeds — market-price feeds vs exchange-rate feeds
  that "report the internal redemption rates for an asset"; "assets with low liquidity or volume can be
  volatile or difficult to price"; set "an explicit valuation ceiling for each stablecoin" with peg
  protection; freshness checks; circuit breakers to "pause or halt contract functionality."
- https://eco.com/support/en/articles/15426773-stablecoin-custody-execution-settlement-the-split —
  under stress "the exchange quote is the noisiest signal available" while issuer redemption at par is
  informative; a market-price oracle at $0.90 liquidates fully-backed positions and the liquidations
  push price lower; fix is pricing by redemption rate.

### Bounds on what a filler or solver may spend

- https://eips.ethereum.org/EIPS/eip-7683 — the earlier draft's `maxSpent`/`minReceived` (and
  `GaslessCrossChainOrder`/`OnchainCrossChainOrder`, `IOriginSettler.open`/`openFor`) were **removed**:
  they were loose bounds that "could misrepresent profitability" — "If the bound was not tight, orders
  could incorrectly appear unprofitable and not get filled" — and they "unnecessarily required
  escrow-per-order, incompatible with fill-first protocols."

### Who pays for failed work; maximum-cost vs actual-cost billing

- https://github.com/eth-infinitism/account-abstraction/blob/develop/contracts/interfaces/IPaymaster.sol
  — validation receives "The maximum cost of this transaction (based on maximum gas and gas price from
  userOp)"; `postOp` receives `actualGasCost`, "Actual cost of gas used so far";
  `PostOpMode.opReverted` = "User op reverted. Still has to pay for gas."
- https://eips.ethereum.org/EIPS/eip-4337 — "UserOperation reverted. paymaster still has to pay for
  gas"; prefund formula; deposit vs stake (stake "never slashed"); unused-gas penalty.
- https://github.com/eth-infinitism/account-abstraction/blob/develop/contracts/core/EntryPoint.sol —
  `UNUSED_GAS_PENALTY_PERCENT = 10`, `PENALTY_GAS_THRESHOLD = 40000`; `PostOpReverted`.
- https://github.com/eth-infinitism/account-abstraction/blob/v0.7.0/contracts/samples/TokenPaymaster.sol
  — pre-charge `requiredPreFund + refundPostopCost * maxFeePerGas`, then refund the excess or charge an
  overdraft; "The token price cannot be queried in the validation code due to storage access
  restrictions of ERC-4337. The price is cached inside the contract and is updated in the `postOp` stage
  if the change is >10%"; `priceMaxAge`; markup bounded 100%–200% via
  `require(priceMarkup <= 2 * PRICE_DENOMINATOR)`.
- https://github.com/pimlicolabs/erc20-paymaster/blob/main/src/base/BaseERC20Paymaster.sol —
  `PriceMarkupTooLow()` / `PriceMarkupTooHigh()`; `OraclePriceStale()` when
  `updatedAt < block.timestamp - stalenessThreshold`.
- https://github.com/pimlicolabs/erc20-paymaster/blob/main/src/ERC20PaymasterV07.sol — token-charge
  formula including a postOp gas allowance; **mode 1 carries a 32-byte token spend limit**, i.e. the
  user's cap denominated in the asset they actually pay in.
- https://hackmd.io/@POarKy9oSma2S2unII8EaQ/erc20paymaster — `REFUND_POSTOP_COST = 40000`; default
  markup 110% within a configurable 100%–120% band; 2.5% price-update threshold. (Secondary.)
- https://docs.pimlico.io/references/paymaster/erc20-paymaster/faqs — "the owner takes a fee that is
  baked into the `exchangeRate` returned by the API."
- https://account-abstraction-docs.biconomy.io/smartAccountsV2/paymaster/api/get-fee-quotes/ —
  `premiumPercentage` (documented example 13), `exchangeRate`, `maxGasFee`, `validUntil`.
- https://docs.openzeppelin.com/community-contracts/paymasters — pre-charge the maximum then refund;
  user bears price risk; an example tolerating 15 minutes of staleness; circuit breakers recommended for
  "extreme market conditions."
- https://github.com/eth-infinitism/account-abstraction/blob/v0.7.0/contracts/samples/VerifyingPaymaster.sol
  — the sponsor signs over the cost numbers plus `validUntil`/`validAfter`, so any re-quote invalidates
  the sponsorship; `AA32` is the on-chain expiry failure.
- https://osec.io/blog/2025-12-02-paymasters-evm/ — charging in `postOp` is a documented loss vector
  (allowance revoked before `postOp`; "the bundler still gets paid for submitting the failed
  UserOperation"); recommends taking payment during validation, not after execution.
- https://www.alchemy.com/docs/wallets/reference/gas-manager-faqs — "If the post-execution ERC-20
  transfer fails, the transaction reverts, but you remain liable for the gas costs"; alerts at
  50/75/90/100% of the limit, and "there may be brief periods where the actual spending slightly exceeds
  the limits you've set."
- https://eco.com/support/en/articles/15254040-what-is-a-paymaster-gas-sponsorship-explained-2026 —
  production markups of roughly 5–15% (hosted 5–20%) over actual cost; when a sponsor fails "the bundler
  eats the cost, which is why bundlers are conservative about which paymasters they accept." (Secondary.)

### Sponsor-side bounds and sponsorship abuse

- https://www.alchemy.com/docs/wallets/transactions/sponsor-gas/conditional-sponsorship-rules — caps on
  "total spend, spend per sender, and spend per transaction" and on "total count and count per sender";
  sender allow/blocklists; webhook approval with `approveOnFailure` defaulting to `false`; policy
  start/end dates; `maxSpendPerUoUsd`, `maxSpendPerSender`; ERC-20 policies add a price multiplier and a
  transfer mode (before or after execution).
- https://www.alchemy.com/support/how-to-fix-gas-manager-errors — typed request-time rejections
  ("Policy's max spend per spender exceeded.", "User operation cost exceeds specified spend limit",
  "policy hasn't started" / "policy has ended", allowlist/blocklist rejections) — not on-chain reverts.
- https://www.alchemy.com/support/best-practices-to-limit-gas-manager-spend — Sybil defence pushed to
  identity (social login, phone verification, captcha) because per-address caps cannot bound spend when
  addresses are free.
- https://docs.erc4337.io/paymasters/security-and-griefing.html — named attack classes: "Drain the
  Paymaster's deposit by spamming invalid operations", "Trick the Paymaster into sponsoring high-cost or
  reverted calls", validation-timing races; validation-phase opcode and gas restrictions.
- https://dev.to/soken_team/unlocking-security-risks-in-erc-4337-paymasters-why-most-are-vulnerable-today-4gim
  — an unguarded validation makes a paymaster "a free gas bank for attackers"; missing replay protection
  turns a signed authorisation into a reusable one. (Secondary.)
- https://docs.pimlico.io/guides/how-to/sponsorship-policies and
  https://docs.pimlico.io/references/platform/api/sponsorship-policies — policies limit "the global
  amount of sponsorships, the amount of sponsorships per user, and per user operation."
- https://docs.cdp.coinbase.com/paymaster/introduction/welcome — "Configure contract allowlists,
  per-user limits, and global spend caps."
- https://eips.ethereum.org/EIPS/eip-7677 — `policyId` in the sponsorship `context`; Security
  Considerations direct apps to proxy through their own backend and re-simulate before sponsoring.

### Exhaustion as a silent stall; sponsor reputation

- https://github.com/ethereum/ERCs/blob/master/ERCS/erc-7562.md — bundlers must track cumulative gas per
  paymaster and reject operations exceeding its deposit; `MIN_INCLUSION_RATE_DENOMINATOR = 10`,
  `THROTTLING_SLACK = 10`, `BAN_SLACK = 50`; `EREP-050` forbids an unstaked paymaster from returning
  `context`, removing the reconciliation leg entirely.
- https://docs.candide.dev/wallet/technical-reference/aa31-paymaster-deposit-too-low/ and
  https://docs.pimlico.io/infra/bundler/entrypoint-errors/aa31 — `AA31` means the sponsor is out of
  funds; exhaustion surfaces as non-inclusion rather than a diagnosable error.
- https://github.com/gensyn-ai/rl-swarm/issues/408 — production instance (2025-07-23): `AA31 paymaster
  deposit too low`, with the reporter observing "nodes run good no error or left behind, but no tx
  recorded"; closed without published root cause.

### Spending-limit modules and policy engines (the shipped knobs)

- https://raw.githubusercontent.com/safe-global/safe-modules/main/modules/allowances/contracts/AllowanceModule.sol
  and https://raw.githubusercontent.com/safe-global/safe-modules/main/modules/allowances/README.md —
  `setAllowance(delegate, token, amount, resetTimeMin, resetBaseMin)` with epoch snapping
  `lastResetMin = currentMin - ((currentMin - lastResetMin) % resetTimeMin)`; `resetTimeMin == 0` is a
  one-time non-renewing allowance needing explicit `resetAllowance`; `uint96` amount, `uint16`
  resetTimeMin (~45 days max), `uint16` nonce (65534 transfers); `removeDelegate(delegate,
  removeAllowances)` as a bulk kill switch; queryable delegate/token lists kept "to prevent hidden
  allowances"; `to` unrestricted — no recipient allowlist in the module.
- https://github.com/coinbase/spend-permissions/blob/main/docs/SpendPermissionAccounting.md and
  https://raw.githubusercontent.com/coinbase/spend-permissions/main/src/SpendPermissionManager.sol —
  `SpendPermission{account, spender, token, allowance(uint160), period(uint48), start, end, salt,
  extraData}`; fixed interval boundaries `[start + n*period, min(end, start+(n+1)*period)-1]` so usage
  resets per period with no roll-over; "No per-transaction cap exists beyond the period allowance"; "No
  recipient restrictions"; `salt` to "differentiate between permissions with the same parameters";
  `getCurrentPeriod()` → `PeriodSpend{start, end, spend}`; both `revoke()` and `revokeAsSpender()`, with
  no refund path for unspent allowance. README: deliberately bypasses the ERC-4337 EntryPoint, "avoiding
  the possibility of ERC-4337 Paymasters spending users' tokens on gas fees."
- https://raw.githubusercontent.com/erc7579/smartsessions/main/contracts/external/policies/ERC20SpendingLimitPolicy.sol
  — enforces `alreadySpent + approvedAmount <= spendingLimit` across `transfer`, `transferFrom`,
  `approve` and `increaseAllowance`; "Every config can allow multiple tokens with its own limit each."
- https://raw.githubusercontent.com/erc7579/smartsessions/main/contracts/external/policies/UniActionPolicy.sol
  — `valueLimitPerUse`; `ParamRule{condition, offset, isLimited, ref, usage, limit}` accumulating a
  decoded calldata argument across calls.
- https://raw.githubusercontent.com/erc7579/smartsessions/main/contracts/external/policies/UsageLimitPolicy.sol,
  .../ValueLimitPolicy.sol, .../TimeFramePolicy.sol — usage count with no time reset; lifetime
  cumulative value cap that never resets; `validAfter`/`validUntil` as two packed `uint48`.
- https://raw.githubusercontent.com/erc7579/smartsessions/main/contracts/ISmartSession.sol and
  .../SmartSession.sol — `enableERC1271Policies`/`disableERC1271Policies` and `erc7739Policies` gating
  signature authority separately from execution; partial revocation via `disableUserOpPolicies` /
  `disableActionPolicies`; on re-enable "the policy will be overwritten with the new configuration" with
  no reset of `used`/`alreadySpent`.
- https://raw.githubusercontent.com/alchemyplatform/modular-account/develop/src/modules/permissions/AllowlistModule.sol
  — wildcard-selector → wildcard-address → exact-pair resolution then revert; limits keyed
  `entityId → target → account`; limits applied to `approve` as well as `transfer`.
- https://raw.githubusercontent.com/alchemyplatform/modular-account/develop/src/modules/permissions/NativeTokenLimitModule.sol
  — "a total native token spend limit across User Operation gas and native transfers," decremented in
  both `preUserOpValidationHook` and `preExecutionHook`; sibling `PaymasterGuardModule`.
- https://docs.zerodev.app/sdk/permissions/policies/rate-limit — two distinct rate semantics: without
  reset, "one UserOp per `interval`, up to `count` times" (finite); with reset, "`count` UserOps within
  an `interval`," indefinite; optional `startAt`.
- https://docs.zerodev.app/sdk/permissions/policies/call , .../gas , .../signature — per-call
  `valueLimit` and argument conditions (`EQUAL/GREATER_THAN/LESS_THAN/GTE/LTE/NOT_EQUAL`); gas policy
  capping wei "in total across all UserOps" with `enforcePaymaster`/`allowedPaymaster`; signature policy
  bounding `allowedCallers`.
- https://docs.metamask.io/smart-accounts-kit/reference/delegation/caveats/ and
  https://github.com/MetaMask/delegation-framework/tree/main/src/enforcers — named enforcers:
  `LimitedCallsEnforcer`, `TimestampEnforcer`, `BlockNumberEnforcer`, `ValueLteEnforcer`,
  `AllowedMethodsEnforcer`, `AllowedCalldataEnforcer`, `ExactCalldataEnforcer`,
  `ExactExecutionEnforcer`, `ArgsEqualityCheckEnforcer`, `RedeemerEnforcer`, `NonceEnforcer`,
  `IdEnforcer`, `ApprovalRevocationEnforcer`, and the `ERC20BalanceChangeEnforcer` /
  `*MultiOperationIncreaseBalance` family asserting balance-delta outcomes rather than calldata shape.
- https://raw.githubusercontent.com/MetaMask/delegation-framework/main/src/enforcers/ERC20PeriodTransferEnforcer.sol
  — `currentPeriod = (block.timestamp - startDate)/periodDuration + 1`; "any unused tokens are forfeited
  once the period ends."
- https://raw.githubusercontent.com/MetaMask/delegation-framework/main/src/enforcers/MultiTokenPeriodEnforcer.sol
  — a 116-byte config per asset (token, periodAmount, periodDuration, startDate; `address(0)` = native)
  selected by index.
- https://raw.githubusercontent.com/MetaMask/delegation-framework/main/src/enforcers/ERC20StreamingEnforcer.sol
  and https://docs.metamask.io/smart-accounts-kit/reference/advanced-permissions/permissions/ —
  streaming allowance shape `initialAmount`, `amountPerSecond`, `maxAmount`, `startTime`; the
  `token-approval-revocation` permission with per-flag fields (`erc20Approve`,
  `erc721SetApprovalForAll`, `permit2Approve`, `permit2Lockdown`, `permit2InvalidateNonces`); optional
  `justification` on every permission.
- https://raw.githubusercontent.com/MetaMask/delegation-framework/main/src/enforcers/NativeTokenPaymentEnforcer.sol
  — payment to the grantor as an after-hook precondition of exercising authority.
- https://raw.githubusercontent.com/MetaMask/delegation-framework/main/src/enforcers/NonceEnforcer.sol —
  `incrementNonce` "invalidates all previous delegations with the old nonce": a one-call kill switch.
- https://raw.githubusercontent.com/MetaMask/delegation-framework/main/src/DelegationManager.sol —
  `disabledDelegations[hash]`, `CannotUseADisabledDelegation()`, and the explicit note that it "simply
  toggles the disabled boolean; it contains no caveat state reset logic."
- https://eips.ethereum.org/EIPS/eip-7715 — defines `wallet_requestExecutionPermissions`,
  `wallet_revokeExecutionPermission`, `wallet_getSupportedExecutionPermissions`,
  `wallet_getGrantedExecutionPermissions` (the older `wallet_grantPermissions` name is gone); permission
  types are explicitly not an exhaustive registry; `expiry` is an optional `ExpiryRule` — only
  `chainId`/`to`/`permission` are required; `isAdjustmentAllowed` lets a wallet counter-offer narrower
  terms.
- https://docs.metamask.io/smart-accounts-kit/reference/advanced-permissions/rules/ — the rule
  reference, including `Redeemer` and `Payee` rules.
- https://eips.ethereum.org/EIPS/eip-7710 — `redeemDelegations(...)` is all that is standardized, with
  caveat enforcement (spend limits, expiry) explicitly out of scope; `Delegation.authority` chains a
  delegation to its parent for sub-delegation with attenuation; hooks run leaf-to-root then reverse; apps
  "SHOULD verify permissions before attempting to execute actions by simulating the `redeemDelegations`
  call."
- https://eips.ethereum.org/EIPS/eip-6900 — separate `preUserOpValidationHook` /
  `preRuntimeValidationHook` / `preSignatureValidationHook` and `preExecutionHook` /
  `postExecutionHook`: whether budget is decremented at validation or at execution decides whether a
  dropped or reverted intent consumes it.
- https://eips.ethereum.org/EIPS/eip-5792 and https://eips.ethereum.org/EIPS/eip-7846 —
  `wallet_getCapabilities` names only `paymasterService` and `atomic` (`flow-control`, `sessionKeys`,
  `auxiliaryFunds` illustrative); there is **no** spending capability in either.
- https://docs.turnkey.com/concepts/policies/overview and .../language — `effect` / `consensus` /
  `condition`, `rootQuorum{threshold, userIds}`, "codifying who must sign",
  `eth.tx.contract_call_args`; and the limitation that policies "are evaluated per each
  request/activity, not cumulatively."
- https://docs.privy.io/controls/policies/overview — rules → conditions → ALLOW/DENY per RPC method
  (`field_source`, `field`, `operator`, `value`, `abi`), DENY wins, default deny, no cumulative limit.
- https://docs.rhinestone.dev/smart-wallet/smart-sessions/overview — `ActionData{actionTarget,
  actionTargetSelector}` as the unit of scope.
- https://raw.githubusercontent.com/bcnmy/scw-contracts/develop/contracts/smart-account/modules/SessionValidationModules/ERC20SessionValidationModule.sol
  — exact recipient binding (`require(recipient == recipientCalled)`), `maxAmount` per call, `maxUsage`.
- Negative result, established by absence across all of the above: **no** surveyed system implements a
  notification or step-up confirmation above a value threshold as a policy knob, and **none** of the
  on-chain modules implements threshold/quorum approval above a value at all.

### Holds, pre-authorization, and reserve release

- https://docs.stripe.com/payments/place-a-hold-on-a-payment-method — `capture_method: manual`,
  `requires_capture`, `amount_capturable`, `payment_intent.amount_capturable_updated`; per-network
  expiry surfaced as `capture_before` (card-not-present Visa 7d CIT / 5d MIT, others 7d; card-present
  Visa 5d, others 2d; JPY up to 30d); if capture never happens funds are released and the payment
  becomes `canceled`; partial capture auto-releases the remainder; `capture_method=automatic_delayed`
  with `capture_by` (`auth_expiry` | `end_of_day` | `target_delay`) as an expiry backstop.
- https://docs.stripe.com/payments/incremental-authorization.md?platform=web&ui=elements —
  `increment_authorization`, `request_incremental_authorization=if_available`,
  `incremental_authorization.status`; max 10 attempts; each increment capped at the greater of 500 USD or
  500% of the prior amount; **increments do not extend the validity window**.
- https://docs.stripe.com/payments/overcapture.md?platform=web&ui=elements — `request_overcapture`,
  `overcapture.status`, `maximum_amount_capturable`; category-specific ceilings (restaurants +30% US,
  taxis/salons +20%, car rental greater of +15% or 75 USD, lodging/cruise +15%; Visa EEA excluded).
- https://docs.stripe.com/payments/multicapture.md?platform=web&ui=elements — `request_multicapture`,
  `final_capture=false` keeps the remainder authorized; up to 50 non-final captures plus one final;
  final capture or window expiry releases the rest; incompatible with incremental authorization once
  partially captured.
- https://docs.stripe.com/payments/extended-authorization.md?platform=web&ui=elements —
  `request_extended_authorization` stretches a hold to ~30 days, with a 0.08% surcharge outside
  lodging/vehicle-rental/cruise categories; hold duration is a priced dimension.
- https://www.visa.com.bz/content/dam/VCOM/global/support-legal/documents/ai09108.pdf — Visa Business
  News AI09108, 12 July 2019: an estimate "must reflect the anticipated / average transaction amount"
  and be disclosed; where it exceeds the final amount "merchants must process partial reversals for the
  difference… as soon as the final amount is known"; the 15% authorization-to-clearing tolerance ceases
  to apply once an incremental authorization is used; issuers must drop holds promptly.
- https://docs.stripe.com/refunds — an uncaptured charge cannot be refunded (cancel the PaymentIntent
  instead); refunds only to the original payment method; refunds can sit `pending`, `requires_action`
  or `failed`.
- https://github.com/coinbase/x402/blob/main/specs/schemes/exact/scheme_exact_evm.md — the `exact` EVM
  scheme is EIP-3009 `transferWithAuthorization`; payload `{signature, authorization:{from,to,value,
  validAfter,validBefore,nonce}}`; "the Facilitator cannot modify the amount or destination."
- https://docs.cdp.coinbase.com/x402/core-concepts/how-it-works and
  https://docs.cdp.coinbase.com/x402/core-concepts/facilitator — `PaymentRequirements{scheme, network,
  maxAmountRequired, resource, payTo, asset, maxTimeoutSeconds, extra}`; sellers can "authorize a
  maximum and settle only what was used"; `exact`, `upto` and `batch-settlement` schemes.
- https://docs.across.to/introduction/refunds — refunds go to `refundAddress` (default depositor) with
  `refundOnOrigin` selecting the chain; automatic — "The refund is executed on-chain, returning the
  escrowed funds to the refundAddress" — but only after `fillDeadline` passes unfilled; "Refunds are not
  instant. After a deposit expires, the refund goes through the bundle settlement process," several
  hours across bundle intervals, challenge period and canonical bridge delays; "Do not tell users to
  expect immediate refunds."

### Recurring, streaming, and the debt-versus-buffer choice

- https://docs.cdp.coinbase.com/wallets/using-wallets/spend-permissions — SDK surface `periodInDays`,
  `getCurrentPeriod`, `revokeSpendPermission`; native and ERC-20 only.
- https://docs.superfluid.org/docs/protocol/advanced-topics/solvency/liquidations-and-toga and
  https://docs.superfluid.org/docs/concepts/overview/money-streaming — opening a stream requires a
  buffer deposit (4 hours of flow on most networks; flat minimums on mainnet, e.g. 0.042 ETHx, 69
  tokens for USDCx/DAIx); solvent → critical (balance hits zero, anyone may close) → insolvent (deposit
  consumed, deficit tracked); 30-minute patrician period; Sentinels close critical streams and take the
  buffer.
- https://docs.sablier.com/concepts/flow/overview and https://docs.sablier.com/concepts/flow/statuses —
  no upfront deposit; debt modelled explicitly (`rps`, `total debt = covered debt + uncovered debt`,
  snapshot and ongoing debt); statuses `STREAMING_SOLVENT`, `STREAMING_INSOLVENT`, `PAUSED_SOLVENT`,
  `PAUSED_INSOLVENT`, `VOIDED`, where voiding permanently forfeits uncovered debt.
- https://docs.sablier.com/concepts/lockup/overview — the opposite trade: full upfront lock, fixed
  start/end, cancelable with the unvested remainder returned.
- https://docs.stripe.com/billing/subscriptions/stablecoins.md?api-integration=setupintents — even a
  custodial processor stores a durable consent record: SetupIntent with `usage=off_session` and explicit
  `mandate_data[customer_acceptance]`, then charges with `default_payment_method` + `off_session=true`.

### Delegated budgets and mandates for agents

- https://developers.openai.com/commerce/specs/payment — the delegated-payment `allowance` object
  `{reason:"one_time", max_amount, currency, checkout_session_id, merchant_id, expires_at}` (RFC 3339),
  returned as a `vt_…` vault token, plus `risk_signals[{type,score,action}]`; the token is "single-use
  and constrained… restricted by the delegated payment's max amount and expiry."
- https://developers.openai.com/commerce/specs/checkout — checkout totals are mutable between session
  creation and completion (tax/fulfillment recalculated), which is why a ceiling rather than an exact
  amount is the right consent primitive; refunds appear in `order_updated` as `refunds[{type:
  store_credit | original_payment, amount}]`.
- https://ap2-protocol.org/ap2/payment_mandate/index.md — the open Payment Mandate
  (`vct: mandate.payment.open.1`) carries six constraint types: agent recurrence (`ON_DEMAND`…`ANNUALLY`
  with max occurrences), amount range (min/max + currency), **budget** (cumulative cap across uses),
  allowed payees, allowed payment instruments, and execution window `not_before`/`not_after`. The closed
  form carries `transaction_id`, `payee`, `payment_amount`, `payment_instrument`.
- https://ap2-protocol.org/ap2/checkout_mandate/index.md — the open Checkout Mandate constrains
  `allowed_merchants` and `line_items`, bound to a merchant-signed `checkout_jwt` via `checkout_hash`,
  with `iat`/`exp`, and carries **no** price ceiling: the split between "what may be bought" and "how
  much may be spent" is deliberate.
- https://ap2-protocol.org/ap2/agent_authorization/index.md and
  https://ap2-protocol.org/ap2/specification/index.md — human-present users sign closed mandates,
  human-not-present users sign open mandates the agent later binds to a transaction with an endorsed
  key; after use "the agent reduces the scope of the open mandate based on the receipt, often preventing
  future presentations entirely"; `exp` should be "the smallest value that will allow the Shopping Agent
  to complete the assigned task."
- https://developer.visa.com/capabilities/trusted-agent-protocol/trusted-agent-protocol-specifications
  and https://github.com/visa/trusted-agent-protocol — RFC 9421 Message Signatures with
  `Signature-Input`/`Signature` carrying `created`/`expires` (max 8-minute window), `keyid`, `alg`,
  `nonce`, and a purpose `tag` of `agent-browser-auth` vs `agent-payer-auth`; Agentic Consumer
  Recognition Object and Agentic Payment Container share the `nonce`; the container has an explicit
  HTTP-402 "browsing IOU" form with invoice ID and amount.
- https://developer.mastercard.com/mastercard-checkout-solutions/documentation/use-cases/agent-pay/
  (official, JS-gated) and
  https://eco.com/support/en/articles/15192001-what-is-mastercard-agent-pay-ai-agent-commerce-protocol-in-2026
  (secondary) — a network token bound to one named agent, one consent policy and one set of merchant
  scopes, validated network-side at every authorization (per-transaction max, monthly cap, allowed
  categories, expiry window, step-up rules). *Field list is from the secondary source; Mastercard's own
  pages did not render for automated fetch.*

### Refund and chargeback asymmetry on chain

- https://docs.stripe.com/payments/stablecoin-payments and
  https://docs.stripe.com/payments/accept-stablecoin-payments.md?payment-ui=direct-api — manual capture
  "Not supported", dispute support "No", per-transaction customer limit 10,000 USD; "refunds are always
  returned as stablecoins to the customer's original wallet", with `destination_details.crypto.reference`
  as an on-chain `transaction_hash`, and the refunded token contract may differ from the one paid in.

### Velocity limiting and circuit breakers

- https://docs.stripe.com/radar/rules/supported-attributes and https://docs.stripe.com/radar/rules —
  velocity as first-class rule attributes across four families × six subjects × four windows
  (`authorized_charges_per_card_number_{hourly,daily,weekly,all_time}`, `total_charges_per_customer_*`,
  `declined_charges_per_ip_address_*`, `blocked_transactions_per_payment_instrument_fingerprint_*`),
  plus `dispute_count_on_card_number_{all_time,yearly}` and cumulative amount attributes; limits of 200
  transaction rules / 100 account rules.
- https://developers.fireblocks.com/reference/configure-transaction-authorization-policy — custody
  policy encodes `amount`, `amountCurrency` (USD/EURO/NATIVE), `amountScope` (`SINGLE_TX` |
  `TIMEFRAME`), `periodSec`, `amountAggregation` (`operators`/`srcTransferPeers`/`dstTransferPeers` =
  `ACROSS_ALL_MATCHES`), `designatedSigners`, `authorizationGroups` with threshold `th`, and actions
  `ALLOW` / `BLOCK` / `2-TIER`.
- https://ethereum-magicians.org/t/eip-7265-circuit-breaker-standard/14909 (also discussed at
  https://ethereum-magicians.org/t/erc-7265-circuit-breaker/14909; **not** published at
  eips.ethereum.org) — `registerAsset`, `updateAssetParams`, `onTokenOutflow`, with `metricThreshold`
  and `minAmountToLimit`; "triggers a temporary halt on protocol-wide token outflows when a threshold is
  exceeded for a predefined metric"; the integrator chooses to "delay settlement and temporarily custody
  outflows during the cooldown period, or revert on attempted outflows"; deliberately agnostic to the
  underlying protocol's structure.

### Not verifiable within this session

- Visa Intelligent Commerce constraint fields — docs behind login
  (https://developer.visa.com/capabilities/visa-intelligent-commerce/docs-getting-started).
- Anchorage Digital policy-engine knobs — https://docs.anchorage.com returns a navigation shell only.
- A formal x402 specification page for the `upto` scheme — the scheme names appear in CDP prose; the
  canonical spec paths 404. Only the exact-EVM scheme file resolved.
- Pimlico's exact sponsorship-policy field names — only the prose description is citable.
- Visa's merchant best-practices PDF at usa.visa.com returns 403 to automated fetch; the AI09108
  bulletin above carries the same rules.
- The session's WebSearch budget (200/200) was exhausted partway through; later evidence was gathered by
  direct fetch of primary sources, so the corpus is spec- and source-heavy rather than survey-heavy.
