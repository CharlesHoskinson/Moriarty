# Orchestration: what is missing from Part I sections 5 and 6

Scope: the orchestration and outcome surface of the DeFi kernel interface design
(`2026-09-11-defi-kernel-sdk-interface-design.md`, Part I §§5-6). Findings are ranked by the
severity of the concrete failure the absence causes. Proposed Part I prose names no chain,
venue or protocol; all specifics are confined to GAPS and SOURCES.

A recurring pattern is worth stating first, because it changes what several of these gaps
cost to close. **Three of the largest gaps are exposure gaps, not mechanism gaps.** Part II
already carries a signed net goal and `E_NET_GOAL_UNMET` at `settle`; a signing display that
discloses lockups; `E_BINDING_WINDOW_UNSAFE`, which refuses authority that outlives its
deadline; `E_QUOTE_INVALID` for an expired quote; and open parameter U22 for quote lifetime.
None of that reaches the account holder. The interface therefore promises less than the
kernel already enforces, and a user reading Part I cannot tell that the price they receive is
bounded at all.

---

## GAPS

### G1. An intent bounds what leaves but not what arrives

**The gap.** §3 bounds "the most that may leave your accounts per intent, per period, and in
total." §5 says an intent names "what you want, what you will pay for it, the deadline, the
minimum acceptable grade, and what should happen if it cannot be reached." Nothing in Part I
is a bound on the quantity received. "What you want" is a goal, not a floor, and the four
standing guarantees bound loss only to "the cap carried in the intent you signed" — a cap on
outflow.

**The concrete failure.** A user signs an intent to convert 1 ETH to USDC with a cap of 1 ETH.
The cap holds perfectly: exactly 1 ETH leaves. They receive 1,500 USDC against a market of
2,000. Every guarantee in Part I is satisfied, the outcome reports `Settled` with a `Proven`
strength mark, and the user has lost 500 USDC. `Bounded loss` did not bound the loss, because
the loss did not happen on the outflow side.

**Evidence.** The inbound bound is the protection that actually exists in practice, enforced
as a revert: "Minimum output (`amountOutMinimum`)... If execution would return fewer output
tokens than this value, the transaction reverts", implemented as
`require(amounts[amounts.length - 1] >= amountOutMin, 'UniswapV2Router:
INSUFFICIENT_OUTPUT_AMOUNT')`. Intent systems carry the same field: a signed order specifies
"starting output amount, minimum output, a decay function, a claim deadline", and when a
quoter fades the order decays toward "the minimum the swapper signed for". The harm from
having no inbound bound is measured and enormous in count: 750,529 sandwich attacks yielding
$174.34M (Dec 2018-Aug 2021), and 3,016,971 sandwich attacks measured through Aug 2023 with a
mean profit of $78.72 and a median of $28.80 per attack — small per event, ubiquitous in
aggregate. Attackability scales directly with the tolerance the user permitted
(`s·δ_vy ≥ 2b`), and a wide tolerance is a direct transfer: "if your slippage is set to 25%
then you may receive 25% fewer tokens than the amount shown to you in the swap preview".
Crucially, no outflow cap detects any of this.

### G2. Every intent is immediate and one-shot

**The gap.** §5 describes one shape: declare an outcome, the router reaches it, `execute` runs
"to completion or to a reported outcome". There is no conditional intent, no schedule, no
recurrence, and no trigger. §6 reinforces this: "Every intent ends in exactly one of these."

**The concrete failure.** A user wants to buy a fixed amount weekly for ten weeks, or to exit a
position if its price falls below a level. The kernel cannot express either. The user's only
options are to sit at a screen and submit ten intents by hand, or to grant a standing
allowance to some external automation service — which is precisely the standing authority the
kernel exists to abolish. The absence of scheduled execution does not merely omit a feature;
it pushes the user back onto the exposure the design's third guarantee was written to remove.
Worse, a stop-loss the kernel cannot express is a stop-loss the user believes they have and
does not.

**Evidence.** Conditional and scheduled orders are a mature, standard product class:
long-lived conditional orders registered as `ConditionalOrderParams{handler, salt,
staticInput}` with a handler that generates discrete orders validated against on-chain
conditions; schedule orders parameterized as `partSellAmount`, `minPartLimit`, `t0`, `n`
parts, `t` interval, `span`; and maker-defined predicates that "determin[e] if the order is
allowed to be fulfilled" and must return 1, which is the documented way to build a stop-loss.
The bounding primitives for recurrence are standardized and explicitly *not* standing
approvals: `wallet_grantPermissions` requires a chain, a session account, a permission and
rules such as `expiry`, with the warning that applications "should only request the
permissions they need, with a reasonable expiration time"; a period-bounded transfer enforcer
fixes `periodAmount`/`periodDuration`/`startDate` where "the transferable amount resets at the
beginning of each period, and any unused tokens are forfeited once the period ends". The
default-permissive alternative is exactly what the design refuses: "By default, a Delegation
allows the delegate to make **any** onchain action so caveats are strongly recommended."

### G3. The outcome set cannot describe anything that is not a finished one-shot trade

**The gap.** Six outcomes, all terminal, one per intent. There is no state for an intent that
is authorized and waiting; none for an intent whose window closed without executing; none for
an intent replaced by another. `Unresolved` is explicitly for work that "was dispatched" and
so cannot cover work that never was. And an outcome describes an intent, while a position may
outlive it — §4 reports positions with release conditions, but §6 has no vocabulary linking a
terminal outcome to a holding that is still live.

**The concrete failure.** A user sets a conditional exit. The condition never occurs and the
deadline passes. The kernel must report one of six states: `Settled` is false,
`PartiallySettled` is false, `Unresolved` is false and alarming (it implies dispatched
authority and unknown exposure), `Refunded` implies consideration was taken and returned when
nothing ever moved, `Compensated` implies a failure that owes compensation, and `OpenClaim`
implies something is owed. Every available answer is a lie about the user's exposure. Under
the honest-outcomes guarantee this is not a cosmetic problem: the kernel would be reporting
an outcome it did not achieve. Separately, a user who cancels sees no state between
"requested" and "stopped", and a user whose settled intent left a leveraged holding sees
`Settled` `Final` on a position that can still be closed against them tomorrow.

**Evidence.** Every mature order vocabulary carries these states. FIX 4.4 `OrdStatus` has
fourteen values including `New`, `PartiallyFilled`, `Expired`, `PendingCancel`,
`PendingReplace` and `Suspended`, with a documented precedence rule for when an order is in
several states at once. Payment lifecycles do the same with `requires_action` and `processing`
as first-class non-terminal states, and note that cancellation "might fail due to a limited
and varying cancellation time window". Cross-chain routing interfaces converge on the same
shape: a terminal verdict plus a substatus, from `NOT_FOUND, INVALID, PENDING, DONE, FAILED`
with substatus `WAIT_DESTINATION_TRANSACTION, REFUND_IN_PROGRESS, COMPLETED, PARTIAL,
REFUNDED`; another exposes exactly `pending, filled, expired ("Fill deadline passed without a
fill (eligible for refund)"), refunded`; another six values including `needs_gas` and
`partial_success`. `expired` is a separate first-class state in all of them, and a batch
status vocabulary likewise separates pending from partial revert (100/200/400/500/600). On the
schedule side, an unfilled part "simply expires — it is not carried forward", which is a state
the current six cannot express at all.

### G4. Nothing renders what is about to be signed

**The gap.** §7 promises "It will not sign anything you cannot see." No call in §5 produces the
thing the user sees. `quote` prices an outcome and `intentFor` builds the intent; neither is
specified to return a human-readable statement of total exposure, and `sign` is not
conditioned on one having been produced. The promise is therefore unimplemented at the
interface: a compliant kernel could satisfy every stated call contract while showing the user
nothing.

**The concrete failure.** An application builds an intent whose terms are within every cap but
economically ruinous, or simply different from what its UI displayed. The user signs. Caps
hold, the movement happens, and the user's recourse is nil — they authorized it. The design's
own threat model requires this call: §5 says applications are denied capabilities because "an
application that could do them could exceed the authority its user granted", yet the
application remains the only party that renders the intent.

**Evidence.** Signature phishing is the dominant loss channel precisely because signing is
invisible and deferred: $295M across ~324,000 victims (2023), $494M across 332,000 (2024)
with `Permit` accounting for 56.7% of losses above $1M, and $83.85M across 106,106 victims
(2025), the largest single loss a $6.5M `Permit` signature. The mechanism is stated plainly:
"the act of signing is not recorded on chain", and a signature "can be configured to remain
valid for a specified duration" — deferred, uninvestigable theft. Revoking a token approval
does not help, because "the approval inside Permit2 still remains". Typed-data signing exists
for this reason: "signed messages are an opaque hex string displayed to the user with little
context"; without a descriptor "the signer falls back to blind signing". Preview is the
industry's answer — balance-change previews on by default, a Sign button *disabled* until
risks are dismissed, and vendors claiming $13.1B in prevented theft — while also being the
right size of promise, since offchain simulations are "not guaranteed" to match execution and
are spoofable through the gap between simulation and execution (one victim lost 143.45 ETH,
about $460,895, signing roughly 30 seconds after a state change).

### G5. A quote carries no reference price, so a manufactured price is indistinguishable from a good one

**The gap.** §5 has the router select venues and source costs. Nothing requires the quoted
price to be corroborated against anything outside the venue that produced it, and nothing
discloses to the user what it was checked against.

**The concrete failure.** The router selects the best available route for a thinly traded
asset. The only price reference is the venue it would trade on, whose price was moved
minutes earlier. Every cap holds, the floor from G1 holds if it was derived from the same
poisoned quote, and the user's loss is complete and fully authorized. The floor alone does
not fix this: a floor computed from a manipulated reference is a floor at the manipulated
price.

**Evidence.** Oracle and single-venue pricing failures are a large, recurring loss class:
$403.2M across 41 oracle manipulation attacks in 2022, and price-oracle attacks make up 15% of
181 incidents totaling $3.24B. A single adversary moved an oracle-fed price "over 13-fold
during a 30-minute span" and withdrew about $116M; a thin pair spiked roughly 50x supported
$15.6M of borrowing, after which the operator concluded "this TWAP feed was not
manipulation-resistant"; a flash-loan drain of the reserves feeding one price took about
$8.7M. The decisive case involves no bug at all: a legitimate print of about $1.30 against
only ~300k of book depth liquidated $89M in 24 hours, and the conclusion was that "the
protocol and price feed performed as designed". Guidance is unanimous that one venue is not a
reference — "the spot price on a decentralized exchange may be wildly incorrect during a
transaction"; reserve-ratio prices "could be trivially manipulated"; "[r]elying on only a
subset of all trading environments makes them vulnerable".

### G6. Cancellation tells the user nothing about what is still fillable

**The gap.** §7 refuses to "claim to have cancelled work already dispatched", and Part II
implements `cancel` as epoch invalidation plus recovery after `recoverAfter`. Both are
correct. But §5 gives `cancel` no return contract: the user learns that cancellation is
forward-looking and not what remains live, nor until when.

**The concrete failure.** A user cancels, sees the call succeed, and treats the position as
closed. A dispatched authorization is still executable at a venue and fills two minutes later
at a price the user no longer accepts. Nothing in the kernel misreported anything — and the
user still made a decision on the belief that nothing could execute. The honest-outcomes
guarantee is about not overstating outcomes; this is the adjacent obligation to state the
residual window.

**Evidence.** Operators document the race explicitly: "Cancellations are not immediate, and
your order may settle before the cancellation goes through", with on-chain invalidation as the
enforcing but gas-costly fallback — "this places trust in the API to cancel the order. If you
want to enforce the cancellation of an order, you can do so on-chain." Nonce-invalidation
cancellation is raceable by construction, and the filler side confirms it: a nonce check
failure means "The order has already been filled by another filler [or] The user cancelled
the order [or] The signature has expired." FIX carries `PendingCancel` and `PendingReplace`
as states for exactly this interval.

### G7. No surface for risk that outlives the intent

**The gap.** §4 reports positions "with their release condition", and `positions` returns what
you hold. A release condition answers "when can I move this". It does not answer "who else
can close this, on what measurement, and how close am I". Nothing in §§5-6 admits that an
outcome can leave a holding that a third party may liquidate.

**The concrete failure.** A user's intent settles into a leveraged holding. `Settled`,
`Proven`, `Final`. Weeks later the position is closed against them at a loss, triggered by a
measurement they were never shown and a threshold they were never told about. `Bounded loss`
is arguably intact (the cap bounded what left), and the user's actual loss is unbounded in
the sense that mattered. Compounding it, the severity is not linear: there are thresholds at
which the fraction of the position that can be closed jumps.

**Evidence.** The measurement and the cliff are both public and computable: liquidation occurs
when a health factor "falls below 1", and up to 50% of debt is liquidatable above a
0.95 factor while **up to 100%** is liquidatable at or below it — a discontinuity a risk
surface must disclose before it is crossed. Perpetual positions have a mechanical liquidation
price and, worse, a closure mode driven by *other* accounts: when an account's value goes
negative, profitable opposite-side traders are closed at the previous mark price, ranked by
profitability and leverage. The historical harm from users being unable to act is documented:
roughly 1,200 vaults liquidated with about $8.3M of collateral sold for 0 in a single episode,
where "[v]ault owners struggled with high gas prices and transactional delays when attempting
to add more collateral", oracles "were affected by the same delays as everyone else", and
governance then voted about 65% against compensating the victims. The modern equivalent is
sequencer downtime: "users will lose access to the standard read/write APIs, preventing them
from interacting with applications", distorting "liquidations or market operations that rely
on timely transactions".

### G8. Time and quote validity are not terms of the deal

**The gap.** An intent names a deadline, which bounds when the kernel may act. Nothing states
how long the outcome is expected to take, nothing states how long the quote is valid, and
nothing states that a route's speed may depend on a shared resource that can be exhausted.
Part II knows quote lifetime is unfixed (U22) and can refuse an expired quote
(`E_QUOTE_INVALID`); the user is told neither.

**The concrete failure.** A user accepts a quote for an outcome that normally completes in
seconds. A shared fast-path allowance is exhausted at the moment of dispatch, so the route
silently takes the slow path and settles in nine hours, inside the signed deadline and with
every guarantee intact. The user, believing the transfer failed, acts on that belief. Separately,
an application holds a quote for ten minutes, submits, and the intent is refused after the
user has already been shown a price — or worse, is served at a materially different one.

**Evidence.** The spread between fast and slow paths is enormous and conditional: a
confirmed-attestation path settles in roughly 8-20 seconds while hard finality runs about
15-19 minutes on some chains and 6-32 hours on others, and the fast allowance is "a **global
shared pool**" where, on exhaustion, "Fast Transfers are temporarily unavailable until the
allowance replenishes" with the documented workaround being to accept slow timing. Refund
paths are likewise slow and must be stated: an automatic refund requires no user action but
takes bundle settlement plus a challenge period plus a canonical bridge — "typically takes
several hours". On quote validity, operators are explicit: quotes carry `expiresAt` with the
rule "Check `expiresAt` before submitting — do not send expired quotes", and a request-for-quote
price carries `quoteExpiry`, "the unix timestamp (in seconds) when the quote will expire and
be rejected on-chain".

### G9. A claim is stated but never costed, so the user is told about value they cannot reach

**The gap.** §4 is strong: value that cannot be routed "is held and reported as an explicit
claim. It is never silently left on a chain for you to discover." §6 gives `OpenClaim`. But a
claim carries no statement of what taking it costs or whether taking it is worth doing, and
`claim` has no specified relationship to a new intent.

**The concrete failure.** A conversion's final step fails, so the user holds full value in the
wrong asset. The kernel honestly reports `OpenClaim`. The user now needs a second intent to
convert it, which needs a quote, which may cost more than the claim is worth. The interface
has handed them a research task. This is the single most common real cross-chain failure, and
it is the one users experience as "my funds are stuck".

**Evidence.** This state is named and documented: "A `PARTIAL` completion occurs when a
cross-chain transfer succeeds but the user receives a different token than requested" —
"If the final swap on the destination chain fails (e.g., due to slippage), the user receives
token B instead of token C." The prescribed remediation is exactly the costed quote the
interface lacks: detect the partial state, read the received token, quote a same-chain swap,
**check it is economic against gas**, then offer it to the user. Others leave the analogous
residue — a failed destination swap leaves a bridged stand-in asset on the destination, or
refunds a bridge token on the source — and a pooled-liquidity shortfall can leave the holder
with intermediary tokens needing a fallback swap plus a withdraw path "so users can recover".
Where completion requires a second, user-signed step, the cost is concrete: manual transfers
involve "covering gas fees on both the source and destination chains", and the right to finish
may not even belong to the user — "it may have been initiated through a third-party app or
smart contract that reserved the right to finalize it".

### G10. No progress, no repetition safety, no observability contract

**The gap.** `status` exists with no specified content; `resume` recovers a crashed client.
Nothing says what progress looks like, when state can next change, what happens if an
application submits the same intent twice, or how an application learns of a change without
holding a session open.

**The concrete failure.** An application's `execute` call times out at the network layer. It
does not know whether the intent was accepted. It retries. Without a defined identity for an
intent and a defined response to a duplicate, the safe choices are to retry and risk double
execution or to not retry and abandon a possibly-live intent. Meanwhile a user watching a
spinner for a nine-hour settlement is told nothing, and an application that gives up polling
reports failure on an intent that later settles.

**Evidence.** Idempotency is the standard answer: an idempotency key is prescribed "to prevent
the creation of duplicate" operations, and batch interfaces return an identifier with the
normative rule "Wallets MUST reject requests with duplicate `id`s". No cross-chain routing
interface surveyed documents an integrator-supplied idempotency key, and the consequence is
stated: a retried execute risks a double deposit, with correlation retrofitted onto a
transaction hash or quote identifier. On progress, the documented discipline is a bounded
poll ladder (10s ×6, 30s ×6, 60s ×12, then 120s, default max 30 minutes) with an explicit rule
that a timeout is not a failure — "do **not** assume failure — persist tx details and give the
user explorer links" — and another operator notes indexer latency of 1-15 seconds so
"[p]olling more frequently than every 10 seconds won't return faster results". None of the
surveyed routing interfaces offers webhooks, so every integrator reimplements this, and
long-tail finality outlives any in-session poll loop.

### G11. A dispatched leg that stalls has no named remedy

**The gap.** §5 says the router "handles failure of any individual step". Part II is stricter
and correct: no re-signing on retry, no attempt increment on a timeout, recovery only after
the dispatched authorization has expired by venue rule. The result is honest but, from the
user's side, featureless: a stalled leg offers nothing but waiting.

**The concrete failure.** A leg is dispatched and under-priced for current conditions. It will
not complete, and recovery cannot safely run until its authorization expires. The user waits
hours with `Unresolved`, holding an exposure they cannot influence, when a small increase in
what they offered would have completed it. If the kernel accepts an unsigned increase it
breaks bounded loss; if it never offers one, it is strictly worse than existing practice.

**Evidence.** Acceleration is standard and is done as a *signed* revision by the original
payer, not as an operator discretion: a depositor-signed speed-up revises the amount offered
with a depositor signature, and a separate on-chain fill status distinguishes unfilled from
requested-slow-fill from filled. Elsewhere the equivalent is an explicit add-gas or manual
execute step, where "[t]he prepaid gas to the Gas Service contract could be insufficient when
the destination chain is too busy" and the remedy is to "manually execute" or "[a]dd gas at
source chain", with unused prepaid gas refunded. Express execution degrades rather than
fails: "[i]f the fee is not paid, the transaction will revert to a regular non-Express"
transaction. Solver-side failure also lands on the user as delay rather than loss — an
unsettled order stays open until its validity time and is re-auctioned — while the penalty on
the failing party may be trivially small.

### G12. The statement is not specified to be sufficient for reconciliation

**The gap.** `statement` returns "the cost statement in one asset". Costs are one line of an
account's history. Nothing requires the statement to record what was intended, what arrived,
at what price against what reference, or the evidence class behind each figure — and nothing
requires it to be exportable or complete.

**The concrete failure.** A user with a year of activity must report disposals with dates,
units, proceeds and basis. The kernel's statement gives costs in one asset. The user
reconstructs the rest by hand from chains the design promised they would never need to know
about — the one place where "assets and chains are routing details" becomes a liability.

**Evidence.** The required fields are externally fixed and granular: a digital asset
identifier, asset name, units "to 18 decimal places", date acquired, date sold, gross proceeds
"reduced by transaction costs and fees", adjusted basis, and transfer-in units and date, for
sales "a broker has effected for customers after 2025". A statement that omits acquisition
dates, per-asset units or proceeds net of fees cannot satisfy that. Independently, the
opacity risk in intent systems is the recognized one: the danger is "signing an intent that
disappears... with no clarity on how or by whom the transaction was created", and the
prescribed mitigation is transparency through auditable execution data.

---

## ADDITIONS

Minimum additions to Part I. Six subsections in §5, three outcome rows plus two properties in
§6, and two call-group changes. Each is chain-free and paste-ready.

### To §5, immediately after "You declare an outcome. You never describe a route."

**An intent bounds both sides.** A cap bounds what may leave your accounts. A floor bounds
what must arrive. Every intent that acquires, disposes of or converts value carries both, and
the kernel enforces the floor exactly as it enforces the cap: where value moves, not in the
interface. A route that cannot hold the floor is not offered. A result below the floor is not
a settlement, and is never reported as one. An intent with no floor is refused rather than
served at whatever price the route returns.

Three properties follow.

**A floor is priced against a reference, and the reference is disclosed.** Every quote states
the reference price it was assessed against, the deviation of the quote from that reference,
and whether that reference is independent of the venues the route would use. Where the only
available reference is the venue that would trade, the quote says so before you sign, because
a price confirmed only by the party quoting it is not confirmed. An intent may require an
independent reference, and one that requires it where none exists is refused before anything
moves.

**Authority expires by construction.** Every intent carries a deadline, the deadline bounds
the window in which any authority derived from it can be used, and the kernel refuses an
intent whose authority would remain usable after its deadline has passed. There is no
open-ended authorization, and no intent that can be executed later at a price its signer
never saw.

**A quote is valid for a stated period, and time is a term.** A quote states when it expires,
what the outcome is expected to take, and the longest it can take while still being served.
The long figure is the one the kernel is held to. Where a route's speed depends on a shared
resource that can be exhausted, the quote says so and states what the outcome becomes if it
is. Signing against an expired quote is refused; re-quoting is an ordinary priced call.

*Failure without it:* the entire class of G1, G5 and G8. A cap of one unit of an asset is
satisfied exactly by handing over that unit and receiving three-quarters of its value; the
kernel reports `Settled` and the user's loss is fully authorized. A floor derived from a
single manipulated venue price is a floor at the manipulated price. A user who is not told
that an outcome can take nine hours instead of nine seconds treats a working transfer as a
lost one.

### To §5, as a new subsection: Preview

**Nothing is signed unseen, and this is the call that makes that true.**
`preview(intent)` returns, in your settlement asset, the complete exposure of an intent before
it is signed: the most that can leave, the least that must arrive, the reference price and the
deviation from it, the deadline, every charge with its payer, every commitment with its
release condition and release delay, the grade that will be achieved, the expected and
worst-case time to settle, and what you are left holding on each way the intent can fail.
Signing is refused for an intent whose preview was not produced. The preview is the kernel's
own statement, not the application's rendering of it, and an application cannot alter,
suppress or relabel it.

A preview states what the kernel will enforce. It is not a prediction of the market, and the
kernel never presents it as a guarantee of the result — only the caps, the floor, the
deadline and the grade are guaranteed.

*Failure without it:* §7's promise that nothing is signed unseen has no implementation. The
application remains the only party that renders the intent, which is the exposure that makes
signature-based theft the largest single loss channel in practice.

### To §5, as a new subsection: Conditional, scheduled and repeated intents

**An intent may be immediate, or it may wait.** One record covers conditions, schedules and
recurrence. A mandate states the condition or schedule under which it executes, the evidence
class that establishes the condition and the staleness bound on that evidence, the number of
executions it authorizes, the bound on each execution and the floor for each, the total bound
across all of them, and the moment the whole mandate expires.

A mandate is not standing authority. It authorizes a counted number of bounded executions and
nothing else, it cannot be enlarged after signing, and it expires whether or not it executed.
Revoking it stops every execution that has not yet been dispatched.

Four properties hold.

- **A condition is established by evidence, never by assertion.** The mandate names the
  evidence class and the freshness the condition requires, and a condition the kernel cannot
  observe to that standard is refused at signing rather than watched and missed.
- **The ability to execute is reserved before the mandate is accepted.** A mandate whose
  future executions are not funded and reserved is not accepted, because an authorization to
  act that cannot pay to act is not a protection.
- **A missed execution is reported as a missed execution.** A window that closes unexecuted
  is not carried forward, is not retried at a later price, and is never reported as a fill.
  Nothing is charged for an execution that did not occur.
- **Each execution is bounded on its own.** A mandate's per-execution cap and floor bind every
  execution separately, so a later execution cannot consume the whole mandate's allowance or
  settle at a price the earlier ones would have refused.

*Failure without it:* the user cannot express a limit, an exit condition, a schedule or a
recurring payment, and the only way to obtain those is to grant a standing allowance to
something outside the kernel — reintroducing exactly the exposure the third standing
guarantee removes. A stop-loss that cannot be expressed is one the user believes they have.

### To §5, as a new subsection: Stopping and replacing

**Stopping is forward-looking, and the kernel states what is still live.** `cancel` stops
future executions and future dispatch. It does not reach work already dispatched, and the
kernel never reports that it did. It returns what it stopped, what remains executable, and
the moment each remaining authorization can no longer be used. Until that moment the intent
is reported as stopping rather than stopped, because an interface that says "cancelled" while
value can still move is a false statement about your exposure.

An intent may be replaced rather than stopped. A replacement takes effect only once its
predecessor can no longer be executed, so no window exists in which both are fillable.

*Failure without it:* the user reads a successful `cancel` as closure and acts on it, then a
dispatched authorization fills minutes later at a price they had rejected. This race is
documented behaviour in every system that signs orders off-chain.

### To §5, as a new subsection: Progress and repetition

**Progress is reported, not guessed.** `status` reports position in the sequence: which steps
are complete, which is in flight, which have not begun, what each is waiting for, and the
earliest moment the state can change. An absence of news is never reported as failure, and a
deadline passing is not evidence that work did not happen.

**Submitting the same intent twice causes one execution.** An intent is identified by what it
authorizes, and a repeated submission of the same intent returns the state of the first rather
than starting a second. A retry re-presents authority already given; it never creates new
authority.

**Where a dispatched step can be accelerated, the price of accelerating it is signed in
advance or not offered at all.** An intent may carry an acceleration allowance inside its
caps, which the kernel may spend to complete a stalled step. Where no allowance was signed,
the step is reported as unresolved and the kernel waits; it never improves the offer with
money the payer did not authorize.

*Failure without it:* an application whose call times out cannot distinguish "not accepted"
from "accepted", and must choose between double execution and abandoning a live intent. A
stalled step has no remedy but waiting, which is strictly worse than prevailing practice,
where the payer signs the increase themselves.

### To §5, as a new subsection: Risk that outlives the intent

**Where an outcome leaves a holding that someone else can close, that is stated before you
sign and reported for as long as the holding exists.** The kernel reports the condition of the
closure, the measurement by which it is judged, your current distance from it, any threshold
at which the severity of the closure changes, and whether the closure can be triggered by
circumstances other than your own holding. A holding whose closure condition the kernel
cannot measure to the evidence standard the intent names is refused rather than entered.

The kernel reports this risk. It does not promise to prevent the closure, and it does not
promise that you will be able to act before it happens.

*Failure without it:* an intent reports `Settled` and `Final`, and the user is closed out
weeks later on a measurement they were never shown, at a severity that changed at a threshold
they were never told about.

### To §6: three added states

| Outcome | Meaning |
|---|---|
| `Live` | Authority exists, is bounded, and nothing has been dispatched. The intent is waiting for its condition, its schedule or its next execution. It is cancellable, and it is not a result |
| `Expired` | The intent's window closed without execution. No value moved and no authority survives. This is an ordinary ending, not a failure |
| `Superseded` | The intent was replaced. The successor is named, and the predecessor can no longer be executed |

### To §6, replacing "Every intent ends in exactly one of these"

An intent is in exactly one state at any moment, and ends in exactly one terminal state.
`Live` is the only state from which nothing has yet been dispatched, and it is the only state
that is not a result.

### To §6, as two added properties

**An outcome describes an intent, never a holding.** A holding that outlives the intent that
created it is reported as a position, with its own release and closure conditions, for as
long as it exists. A terminal outcome on an intent is never a statement that your exposure has
ended.

**A claim states what taking it costs.** An open claim carries the asset and amount held, what
reaching your receiving preference from there would cost, whether doing so is worth more than
it costs, and what happens if you leave it. A claim you cannot act on economically is stated
as such, so an honest report of held value is never a research task handed back to you.

**`statement` is complete and exportable.** For every intent it records what was intended,
what arrived and what left, in which asset, when, at what price against what reference, which
charges fell to which payer, and the evidence class behind each figure. It is sufficient to
reconstruct the account's history without consulting any chain, which is the condition for
the promise that chains are routing details to survive contact with a record-keeping
obligation.

### To §5's call table

| Group | Change |
|---|---|
| Planning | add `preview` — the signed exposure statement for an intent, required before `sign` |
| Execution | `cancel` returns what it stopped and what remains executable; add `replace` |
| Reporting | `claims` carries the cost of taking each claim; `statement` is complete and exportable |

*Total additions to the application interface: two calls (`preview`, `replace`), taking it
from 23 to 25.*

---

## REFUSE AND SAY SO

Sentences for §7, in its existing voice.

- **It will not execute at a price you did not bound.** An intent without a floor is refused.
  A cap on what leaves your account is not a price, and the kernel never treats it as one.
- **It will not price against a reference it cannot corroborate without telling you.** Where
  the only available price comes from the venue that would trade, that is stated before you
  sign, and an intent that demands an independent reference is refused where none exists.
- **It will not promise that you will be able to act in time.** It reports a risk condition,
  the measurement behind it and your distance from it. Under congestion, an outage or a fast
  market, no interface can guarantee you a window in which to respond, and the kernel does not
  pretend to be one.
- **It will not prevent a third party from closing a position you hold.** Where an outcome
  leaves such a holding, the closure condition is stated before you sign and reported while it
  lasts. Reporting it is the guarantee; preventing it is not.
- **It will not carry a missed execution forward.** A window that closes unexecuted is closed.
  The kernel does not retry it later at a price you never saw, and does not charge you for it.
- **It will not report an intent as stopped while any authorization for it can still be
  used.** Until the last dispatched authorization can no longer execute, the intent is
  reported as stopping, with the moment each one lapses.
- **It will not treat a preview as a promise about the market.** The preview binds the kernel
  to caps, floor, deadline and grade. It is not a prediction, and a route that turns out worse
  than previewed but inside your bounds is not a breach.
- **It will not spend more to accelerate work than you authorized in advance.** A stalled step
  waits, and is reported as unresolved, rather than being completed with money you did not
  commit.
- **It will not hold authority that outlives its deadline**, and will not accept an intent
  whose authorization remains usable after its window has closed.

---

## DO NOT ADD

- **A "protect me from extraction" guarantee, or any promise of a rebate share.** The honest
  bound is the floor: you get at least what you signed for. Systems that pay a share of
  recovered value are explicit that the share is set by the recovering party, that the best
  bid may not land, and that "MEV bots may still be able to extract value from your trades";
  independent measurement puts the benefit at 4-5 basis points, and only within some trade
  sizes. A guarantee the kernel cannot verify would violate honest outcomes.
- **A user-facing slippage percentage.** It is the wrong shape for this design: it is a route
  parameter, and Part I's whole premise is that the user states an outcome. A floor in the
  asset you asked for carries the same protection without asking the user to reason about
  execution. Measured harm scales directly with the tolerance users set, and defaults have
  been shown not to work.
- **Solver, filler or venue reputation, scores or leaderboards in Part I.** Naming
  counterparties is naming routes. If a counterparty's reliability matters, it belongs in the
  grade model or in the refusal to offer a route whose failure cannot be bounded — both of
  which already exist.
- **An all-or-nothing flag for multi-chain outcomes.** This is cross-chain atomicity in a
  parameter, and the design correctly refuses it. `atomicRequired` exists in single-chain
  batch interfaces precisely because the guarantee is available there; the kernel's answer for
  sequences remains bounds, disclosure and compensation.
- **Automatic remediation of a claim.** Converting a wrongly-delivered asset is a new economic
  decision at a new price. The kernel should cost it and offer it; performing it under the old
  intent's authority would be authority outliving its intent. State the cost, require a new
  intent.
- **A guaranteed trigger or an execution-price promise on a conditional intent.** Triggers
  depend on observation with latency and on the ability to act; feeds update on deviation or
  heartbeat, and outages remove the ability entirely. The kernel can promise bounded
  authority, reserved funding, an evidence standard, and honest reporting of a miss. It cannot
  promise the fill.
- **A portfolio manager, a rebalancer or an automatic de-risking engine.** These select routes
  and take positions on the user's behalf on a standing basis. Report the risk; let the user
  or their application decide. Anything else is discretionary authority in the kernel.
- **A webhook or notification transport in Part I.** The guarantee the application needs is
  that progress is observable and that the earliest next change is stated. How it is delivered
  is an SDK concern and would be the only transport detail in a part that names no signing
  scheme.
- **New outcome states for every failure shape** — a separate state for a stuck leg, a slow
  route, a failed trigger. `Unresolved` plus `status` covers these, and the value of the
  outcome set is that it is short. Three additions are the minimum that removes states the
  kernel would otherwise have to misreport.
- **A cancellation that refunds the cost of cancelling.** Cancellation and recovery are priced
  work with a named payer in the cost model; making them free creates the exhaustion vector
  Part II's medium-severity failure modes already identify.

---

## SOURCES

**Price protection and the inbound bound**
- https://developers.uniswap.org/llms.mdx/docs/get-started/concepts/traders/swaps — minimum output is the enforced protection; execution returning less reverts.
- https://github.com/Uniswap/v2-periphery/blob/master/contracts/UniswapV2Router02.sol — the floor as code: `INSUFFICIENT_OUTPUT_AMOUNT`.
- https://support.uniswap.org/hc/en-us/articles/8643879653261-What-is-Price-Slippage- — auto tolerance 0.5%-5%; a 25% tolerance means possibly receiving 25% less than previewed.
- https://github.com/Uniswap/interface/blob/da6d36f71c4d2fd665b0aae1a052a4ffda917b31/packages/uniswap/src/i18n/locales/source/en-US.json — "Receive at least"/"Spend at most"; high tolerance invites front-running.
- https://app.uniswap.org/whitepaper-uniswapx.pdf — a signed intent carries starting output, minimum output, decay and claim deadline; RFQ exclusivity is a free option needing a penalty system.

**Quantified MEV harm**
- https://arxiv.org/abs/2101.05511 — 750,529 sandwich attacks, $174.34M, within $540.54M extractable value.
- https://arxiv.org/abs/2102.03347 — 196,691 insertion attacks, ~$13.9M, mean $78.72 / median $28.80; a 0.5% default "does not work".
- https://arxiv.org/abs/2405.17944 — 3,016,971 sandwich attacks measured through Aug 2023.
- https://arxiv.org/abs/2202.03762 — attackability condition `s·δ_vy ≥ 2b`: profit scales with the tolerance the user set.
- https://arxiv.org/abs/2309.13648 — cost decomposition; slippage 11bps of 24bps on large stable pairs, ~140bps on a long-tail pair; 3 of 8,294 private swaps saw significant adversarial slippage.
- https://arxiv.org/abs/1902.05164 — front-running taxonomy (displacement, insertion, suppression).

**Deadline semantics**
- https://github.com/code-423n4/2022-12-backed-findings/issues/64 — the canonical unbounded-deadline finding: a stale order's minimum goes out of date and becomes sandwichable; 200,000+ pending transactions older than a month.
- https://github.com/code-423n4/2024-03-revert-lend-findings/blob/5fd5466a646d1870367733b8c2bf352af5f9298d/report.md — judged repeatedly: no proper deadline enables later malicious execution.
- https://eips.ethereum.org/EIPS/eip-2612 — a relaying party may withhold a signed authorization, so the owner must bound its validity.
- https://developers.uniswap.org/llms.mdx/docs/protocols/permit2/overview — single-use nonce-scoped transfer versus standing time-bound allowance.

**Order flow auctions: what is and is not protected**
- https://docs.mevblocker.io/concepts/order-flow-auction — 90% of the winning backrun bid to the user; auction scored on user rebate.
- https://raw.githubusercontent.com/cowprotocol/docs/b9cb59963ae4c0ba466692a9c447b0e24f1d9ab3/docs/mevblocker/builders/rules.md — limits: a lower bid can still land, hints leak, good behaviour is contractual.
- https://cow.fi/learn/understanding-mev-protection — the residual is conceded: bots may still extract value even with optimal slippage.
- https://docs.cow.fi/cow-protocol/concepts/benefits/mev-protection — uniform clearing prices make order within a batch irrelevant.
- https://docs.flashbots.net/flashbots-protect/quick-start — private routing is a trust model, not a proof.
- https://arxiv.org/abs/2405.00537 — measured benefit 4-5 basis points.
- https://arxiv.org/abs/2503.00738 — benefit confined to certain trade sizes, weakest on long-tail pairs.

**Solver and filler risk**
- https://docs.cow.fi/cow-protocol/reference/core/auctions/bonding-pools — slashable solver bonds.
- https://forum.cow.fi/t/1440.json and https://forum.cow.fi/t/2649.json — actual slashings (166,183 USDC; $76,783).
- https://docs.cow.fi/cow-protocol/reference/core/auctions/competition-rules — settlement deadline; penalty for failing to settle capped near $20 equivalent.
- https://raw.githubusercontent.com/cowprotocol/services/main/crates/orderbook/openapi.yml — an unsettled order stays open until its validity time and is re-auctioned: the cost to the user is delay.
- https://developers.uniswap.org/llms.mdx/docs/liquidity/uniswapx/filling/faq — fade cooldown formula; removal only near a 10% fade rate; on a fade the order decays to the signed minimum and the user's remedy is to retry.
- https://developers.uniswap.org/llms.mdx/docs/liquidity/uniswapx/concepts/auction-types — permissioned quoters, permissionless fillers, ~2-block exclusivity.
- https://arxiv.org/abs/2304.04981 — contingent-on-execution payment worsens execution probability and effective spreads.
- https://arxiv.org/abs/2503.05338 — unbounded upside for winning bids, limited downside for failures.
- https://www.globalfxc.org/docs/fx_global.pdf — last look defined as a final opportunity to reject against the quoted price.
- https://www.dfs.ny.gov/reports_and_publications/press_releases/pr1511181 — $150M fine; a client rejected 9 times out of 10 on competitive rates.
- https://www.paradigm.xyz/2023/06/intents — intent-architecture risk: signing an intent that disappears with no clarity on how or by whom it was executed; transparency and auditable execution data prescribed.

**Reference price**
- https://www.chainalysis.com/blog/oracle-manipulation-attacks-rising/ — $403.2M across 41 oracle manipulation attacks in 2022.
- https://arxiv.org/abs/2208.13035 — price-oracle attacks 15% of 181 incidents totaling $3.24B.
- https://www.sec.gov/news/press-release/2023-13 and https://www.cftc.gov/PressRoom/PressReleases/8647-23 — ~$116M; oracle-fed price jumped over 13-fold in 30 minutes. Cite as mechanism only: convictions vacated May 2025 (https://www.trmlabs.com/resources/blog/breaking-federal-judge-overturns-all-criminal-convictions-in-mango-markets-case-against-avraham-eisenberg).
- https://forum.inverse.finance/t/update-frontier-inv-price-feed/300 and https://rekt.news/inverse-finance-rekt/ — ~50x spike on a thin pair supported $15.6M of borrowing; the feed "was not manipulation-resistant".
- https://rekt.news/polter-finance-rekt/ — ~$8.7M from flash-loan-drained reserves feeding a price.
- https://decrypt.co/49657/oracle-exploit-sees-100-million-liquidated-on-compound and https://compound.substack.com/p/developer-community-call-recap-dai — $89M liquidated in 24 hours on a legitimate thin-book print; "performed as designed". The no-bug case.
- https://samczsun.com/so-you-want-to-use-a-price-oracle/, https://www.openzeppelin.com/news/secure-smart-contract-guidelines-the-dangers-of-price-oracles, https://chain.link/education-hub/market-manipulation-vs-oracle-exploits — unanimous guidance that a single venue is not a reference.

**Preview and blind signing**
- https://drops.scamsniffer.io/scam-sniffer-2024-web3-phishing-attacks-wallet-drainers-drain-494-million/ — $494M, 332,000 victims; `Permit` 56.7% of losses above $1M.
- https://drops.scamsniffer.io/scam-sniffer-2023-crypto-phishing-scams-drain-300-million-from-320000-users/ and https://drops.scamsniffer.io/scam-sniffer-2025-crypto-phishing-losses-fall-83-to-84-million/ — $295M/324,000 (2023); $83.85M/106,106 (2025), largest a $6.5M `Permit`.
- https://support.metamask.io/stay-safe/protect-yourself/wallet-and-hardware/signature-phishing/ — signing is not recorded on chain; a signature can stay valid for a configured duration.
- https://drops.scamsniffer.io/introducing-uniswap-permit2-authorization-management/ — revoking the token approval leaves the inner approval intact.
- https://eips.ethereum.org/EIPS/eip-712 and https://eips.ethereum.org/EIPS/eip-7730 — signed messages are opaque hex with little context; without a descriptor the signer blind-signs.
- https://support.rabby.io/en/articles/14124199-understanding-rabby-s-transaction-simulation — exact in/out preview; Sign disabled until risks are dismissed; gasless permissions flagged.
- https://support.metamask.io/manage-crypto/transactions/simulations/ and https://support.metamask.io/configure/wallet/security-alerts/ — balance-change previews on by default; offchain simulation not guaranteed; warnings do not block confirmation.
- https://blockaid.io/ — vendor-published $13.1B prevented, 5.9B transactions scanned (scale of the practice).
- https://drops.scamsniffer.io/transaction-simulation-spoofing-a-new-threat-in-web3/ — simulation spoofing via the simulation-to-execution gap; 143.45 ETH (~$460,895) lost ~30 seconds after a state change. Why preview must not be sold as a guarantee.
- https://developers.ledger.com/docs/clear-signing/overview — simulation "predicts an outcome but strips away context".

**Conditional, scheduled and recurring execution**
- https://docs.cow.fi/cow-protocol/reference/contracts/periphery/composable-cow — long-lived conditional orders as handler + salt + static input generating discrete validated orders.
- https://github.com/cowprotocol/composable-cow/blob/main/src/types/twap/libraries/TWAPOrderMathLib.sol — schedule math; a part not filled in its window expires and is not carried forward.
- https://docs.cow.fi/cow-protocol/reference/contracts/programmatic/twap — schedule parameters including span.
- https://github.com/cowprotocol/watch-tower — an off-chain watch-tower, run by anyone, is what makes a conditional order execute: liveness is an external dependency.
- https://github.com/1inch/limit-order-protocol/blob/master/description.md — predicates as the documented stop-loss mechanism; expiration and fill flags; cancellation routes.
- https://docs.chain.link/chainlink-automation/overview/automation-economics — below the minimum balance "the Automation Network will not perform onchain transactions": an unfunded trigger silently never fires.
- https://docs.chain.link/architecture-overview/architecture-decentralized-model — deviation-or-heartbeat updates mean a trigger reads a lagging price.
- https://docs.chain.link/data-feeds/l2-sequencer-feeds — sequencer downtime removes the ability to transact; grace-period pattern.
- https://hyperliquid.gitbook.io/hyperliquid-docs/trading/take-profit-and-stop-loss-orders-tp-sl — even venue-native triggers degrade: mark-price triggering, slippage tolerance, stop-limits resting unfilled in sharp moves.
- https://eips.ethereum.org/EIPS/eip-7715 — permission grants with expiry rules; request only what is needed with a reasonable expiration.
- https://eips.ethereum.org/EIPS/eip-7710 — delegations can be revoked, expire or be invalidated by state changes.
- https://github.com/MetaMask/delegation-framework — by default a delegation permits any action, so caveats are essential.
- https://github.com/MetaMask/delegation-framework/blob/main/src/enforcers/ERC20PeriodTransferEnforcer.sol — period amount/duration/start with forfeiture of the unused remainder: the shape of a bounded recurring authority.
- https://github.com/coinbase/spend-permissions — recurring spend permissions, revocable.
- https://eips.ethereum.org/EIPS/eip-4337 — validity window primitives; scoping is account logic, not protocol logic.

**Cancellation and replacement**
- https://docs.cow.fi/cow-protocol/tutorials/cow-swap/limit — "Cancellations are not immediate, and your order may settle before the cancellation goes through."
- https://learn.cow.fi/tutorial/cancel-on-chain-order — off-chain cancellation trusts the API; on-chain enforcement costs gas.
- https://github.com/Uniswap/permit2/blob/main/src/SignatureTransfer.sol — cancellation as nonce invalidation, raceable until it lands.
- https://developers.uniswap.org/docs/liquidity/uniswapx/filling/faq — a nonce failure means already filled, cancelled, or expired.

**Cross-chain failure, status vocabulary and stuck value**
- https://docs.li.fi/api-reference/check-the-status-of-a-cross-chain-transfer — status plus substatus, including `PARTIAL` and `REFUNDED` under `DONE`.
- https://docs.li.fi/agents/workflows/partial-completion — `PARTIAL` means full value in the wrong asset; prescribed remediation is a same-chain re-quote checked as economic against gas.
- https://docs.li.fi/faqs/troubleshooting — the bridge can succeed while the destination swap fails; refund behaviour depends on the tool used.
- https://docs.li.fi/agents/workflows/status-recovery — poll ladder 10s/30s/60s/120s to ~30 minutes, and a timeout must not be treated as failure.
- https://docs.li.fi/llms.txt — no webhook interface; polling is prescribed.
- https://docs.socket.tech/integrate/integration-guides/socket-api.md — `expiresAt` on quotes, "do not send expired quotes", quote id as the polling key, terminal `EXPIRED`/`REFUNDED`, refund address for some origins.
- https://docs.bungee.exchange/bungee-legacy/socket-api/guides/bungee-smart-contract-integration/ — thin destination liquidity leaves intermediary tokens; integrators need a fallback swap and a withdraw path. (Legacy URL, now redirecting.)
- https://docs.squidrouter.com/api-and-sdk-integration/api/get-route-status and https://docs.squidrouter.com/api-and-sdk-integration/key-concepts/track-status — `needs_gas`, `partial_success`, `refund`; a failed destination swap leaves a stand-in asset or a source-side refund.
- https://docs.axelar.dev/dev/general-message-passing/debug/transaction-recovery/ and https://docs.axelar.dev/dev/general-message-passing/debug/error-debugging/ — insufficient prepaid gas on a busy destination; manual execute or add gas; named error conditions.
- https://docs.axelar.dev/dev/axelarjs-sdk/tx-status-query-recovery — recovery is programmatic: query status, manual relay, execute, add gas.
- https://docs.axelar.dev/dev/general-message-passing/express — express degrades to the slow path rather than failing.
- https://docs.across.to/api-reference/deposit/status/get — exactly four states including `expired` ("Fill deadline passed without a fill (eligible for refund)") and `refunded`.
- https://docs.across.to/introduction/refunds — no-fill is a first-class refund path, automatic but taking several hours.
- https://docs.across.to/reference/api-reference — omitting a refund address downgrades recovery to a much slower manual process.
- https://docs.across.to/introduction/tracking-deposits — indexer latency 1-15s; polling faster than 10s gains nothing; use a sane timeout.
- https://docs.across.to/guides/concepts/intent-lifecycle — relayer fronts capital; exclusivity window then open filling.
- https://github.com/across-protocol/contracts/blob/master/contracts/interfaces/V3SpokePoolInterface.sol — depositor-signed speed-up revising the amount offered; on-chain fill status unfilled/requested-slow-fill/filled.
- https://developers.circle.com/cctp/references/technical-guide — the mint is a separate transaction someone must submit; hook execution is not in the core protocol.
- https://developers.circle.com/cctp/concepts/finality-and-block-confirmations — seconds versus 15-19 minutes versus 6-32 hours by chain.
- https://developers.circle.com/cctp/concepts/fast-transfer-allowance — a global shared pool; on exhaustion the fast path is unavailable and the workaround is slow timing.
- https://developers.circle.com/cctp/concepts/forwarding-service — who pays the destination transaction, and why sponsorship exists.
- https://wormhole.com/docs/products/token-transfers/wrapped-token-transfers/portal/faqs/ — a stuck transfer may be finishable only by the third party that reserved the right to finalize it.
- https://wormhole.com/docs/products/token-transfers/wrapped-token-transfers/guides/transfer-wrapped-assets/ — manual completion needs gas on both chains.

**Position risk**
- https://aave.com/help/borrowing/liquidations — health factor formula; liquidation below 1; continuous monitoring required; close factor up to 50% above 0.95 and up to 100% at or below it.
- https://aave.com/docs/concepts/liquidations — liquidation mechanics and bonus behaviour as the factor falls.
- https://hyperliquid.gitbook.io/hyperliquid-docs/trading/liquidations — mechanical liquidation price and maintenance margin.
- https://hyperliquid.gitbook.io/hyperliquid-docs/trading/auto-deleveraging — a position can be closed because of *other* accounts' insolvency, ranked by profitability and leverage.
- https://www.quadrigainitiative.com/casestudy/makerdaoabnormalliquidations.php and https://medium.com/@whiterabbit_hq/black-thursday-for-makerdao-8-32-million-was-liquidated-for-0-dai-36b83cac56b6 — ~1,200 vaults, ~$8.3M sold for zero; owners could not get collateral transactions through; compensation refused. The inability-to-act failure.

**Idempotency, progress and batching**
- https://docs.stripe.com/payments/paymentintents/lifecycle — non-terminal `requires_action` and `processing` as first-class states; cancellation may fail inside a varying window.
- https://docs.stripe.com/payments/payment-intents — idempotency keys to prevent duplicate operations.
- https://eips.ethereum.org/EIPS/eip-5792 — batch identifier with "Wallets MUST reject requests with duplicate `id`s"; atomic versus non-atomic execution; status classes including partial revert. Atomicity is offered only where it can be guaranteed.
- https://www.inforeachinc.com/fix-dictionary/fix_4_4_fields_ordstatus — fourteen order states including `Expired`, `PendingCancel`, `PendingReplace`, `Suspended`, with a precedence rule for simultaneous states.

**Statement and reconciliation**
- https://www.irs.gov/instructions/i1099da — required fields: asset identifier and name, units to 18 decimal places, date acquired, date sold, proceeds net of transaction costs and fees, adjusted basis, transfer-in units and date; applies to sales effected after 2025.
- https://www.irs.gov/forms-pubs/about-form-1099-da — the reporting form itself.

**Quote validity**
- https://docs.hashflow.com/hashflow/taker/getting-started-api-v3 — `quoteExpiry` as the timestamp after which a quote is rejected on-chain.
- https://help.1inch.com/en/articles/6796085-what-is-1inch-fusion-and-how-does-it-work — a decaying auction ends in fill, slippage breach or expiry; an expired swap must be resubmitted and costs nothing.
- https://raw.githubusercontent.com/1inch/limit-order-settlement/master/README.md — protection is mempool avoidance: the fill never becomes a public pending transaction.

**Provenance caveats.** One status enum is taken from an integration guide's prose rather than
an enforced schema; the intermediary-token guidance survives only at a legacy URL that now
redirects; and one speed-up detail is cited from contract source because it is absent from the
rendered documentation. The oracle-manipulation case is cited for its mechanism only, since
the criminal convictions in it were vacated in May 2025.
