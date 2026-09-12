# Moriarty DeFi Kernel: Interface Specification

**How to read this.** The document is layered, and each layer has an audience that never needs the one below it.

| Part | Audience | Contains |
|---|---|---|
| I — The interface | Account holders, application developers | The whole of what the kernel offers: identity, spending, receiving, orchestration, outcomes |
| II — The kernel | Protocol implementers | How Part I's guarantees are kept: intent binding, settlement, pricing, operators, error contract |
| III — The adapter contract | Chain integrators | Everything chain-specific: what a chain must supply to be routable, and what is currently routable |
| IV — Record | Maintainers | Decisions taken, and parameters still unfixed |

Part I names no chain, no venue and no signing scheme. That is deliberate and is the point of the design: an account holder states an outcome, and which chains and assets serve it is a routing decision. A reader who only wants to know what the kernel does for them can stop at the end of Part I.

**Status:** specified only. No adapter has been tested against deployed behaviour, no meter value has been benchmarked, and no attestation committee has been audited for independence. A number that neither a source nor a measurement fixes is not written into this document; Part IV names each one, what would settle it, and who owns it.

---

# Part I — The interface

This part is the whole of what an account holder and an application see. It names no chain, no venue and no signing scheme, because none of those is a property of what the kernel does. Chains and assets are routing details, resolved below the line in Part III.

## 1. What the kernel does

You state an outcome you want. The kernel reaches it, using whatever chains, venues, assets and counterparties the outcome requires, and reports what actually happened.

Five guarantees hold for every outcome, on every chain, whatever fails:

- **Bounded loss, on both sides.** A cap bounds what may leave your accounts. A floor bounds what must arrive. Both are carried in the intent you signed, both are enforced where value moves, and every authority the kernel creates is charged against your caps when it is created rather than when it is used. This holds against a failed step, a hostile counterparty, an operator fault, a chain halt, and any combination.
- **Honest outcomes.** The kernel never reports an outcome stronger than it achieved. A submitted transaction is not a settled one, an outcome nobody can yet prove is reported as unresolved rather than as success or failure, and waiting is never reported as failure. An outcome describes an intent; it is never a statement that your exposure has ended.
- **Authority is bounded and ends.** Every authority the kernel creates carries a ceiling and a moment it stops, no later than the intent that needed it, and it stops then without anyone acting. Everything live is enumerable at any time. Where an outcome could only be reached by authority the kernel cannot bound that way, **the outcome is refused rather than the authority granted** — and where an authority the kernel meant to retire could not be retired, it is reported as unretired rather than as closed.
- **One identity, outliving whatever authorizes it.** One identity spends and receives everywhere. You never hold a per-chain key, a per-chain gas balance or a per-chain approval, and changing what you authorize with never changes your accounts or where value already sent to you lands.
- **Nothing arriving unbidden spends your money.** Pricing, routing, converting and consolidating all cost something. The kernel performs none of them for value you did not ask for, so a stranger who sends you something cannot thereby choose what you spend.

### Assets and chains are routing details

You name assets symbolically. An intent names an asset, never a token address on a particular chain. Resolving that to a specific asset on a specific chain, and deciding whether reaching it needs a conversion, a transfer or nothing at all, is the router's decision and is not part of what you sign.

Where the same name exists on several chains, they are distinct assets to the router and one asset to you. The kernel never treats two same-named assets as interchangeable without an explicit conversion step it can evidence, and an inbound asset bearing your settlement asset's name is not your settlement asset until the kernel has established that it is.

You never hold a chain's gas token. Execution costs on every chain are sourced by the kernel and billed to you in your settlement asset, inside the caps you signed, reserved at their worst case before work begins.

### Grades

Exactly one property of the underlying mechanism reaches you, because it changes what an outcome is worth rather than how it was obtained. Every outcome and every receipt carries two marks:

| Mark | Values | Meaning |
|---|---|---|
| Strength | `Proven` or `Attested` | `Proven`: the settling ledger verified the fact itself. `Attested`: a named committee signed that the fact occurred, and the outcome is correct unless enough of them are wrong together |
| Finality | `Final` or `Contingent` | `Final`: no further condition can reverse it. `Contingent`: a dispute period, unlock or challenge window is still open, and the record names the condition and when it closes |

The router always selects the strongest path available for the outcome you asked for. You are told the grade of what you got. You are never told the mechanism that produced it, and you never choose between signing schemes.

An intent may require a minimum grade. An intent that demands `Proven` and can only be served `Attested` is refused before anything moves, with the reason given as the grade shortfall.

---

## 2. Identity

One identity. It holds accounts on every chain the kernel supports, derived from it, existing whether or not you have ever used that chain.

- You never create, import, back up or select a per-chain key.
- You never connect a wallet per chain or per application.
- Your receiving address on any supported chain already exists and is already yours.
- Adding a chain to the kernel gives you accounts on it. It requires nothing from you.

**Your identity outlives whatever authorizes it.** More than one means of authorization may stand for your identity at once. You may add one and retire one, and doing either changes nothing else: not the identity, not the accounts it holds, not where value already sent to you will land. An account that was ever yours stays yours and stays able to receive. The kernel never moves you onto a new set of accounts, and never asks a counterparty to be told a new one because something you authorize with changed.

Retiring the last remaining means of authorization is refused. The kernel will not hold an identity that one lost device ends. Restoring authorization through a party you nominated in advance takes effect only after a delay the identity states, and only if no standing means of authorization refuses it within that delay. The kernel tells you a restoration has been requested by every means it has, at the moment it is requested, and a restoration that completes never reaches backwards: it does not adopt work authorized before it, and it is reported as a restoration rather than as your own act.

Your means of authorization are enumerable, each with when it was added, when it was last used, and whether it is standing or retired. Retirement is not deletion; it stays readable so past authority can still be explained.

### Delegation

An identity delegates without surrendering. An application acts for you only through a capability naming what it may do and what it may spend, and the identity itself is never handed over.

**Every capability ends by itself.** A capability carries the moment it stops, and at that moment it stops without you, the application or the kernel doing anything. A capability that has ended cannot be extended or renewed; continuing requires a new grant.

**You can see everything that stands for you.** At any time you can list every capability that can act for your identity: what each may do, what it may spend, what is left of that, what it has already spent, and when it ends. That list is the authority the kernel enforces, not a record an application keeps, and no application can present a different one.

**Authority never widens when it is passed on.** Where a capability may be exercised by someone other than the party you granted it to, what they receive is narrower than what was granted and ends no later.

**Every outcome names the authority that produced it** — the capability it was authorized under and the means of authorization behind it — so you can always say which authority moved what.

**One action stops everything.** You can withdraw all authority from your identity in a single act, without naming what you are withdrawing it from. It takes effect at once, for every capability, on every network. While your identity stands down, no new capability can be created and no new intent is accepted under it; receiving is unaffected, and what you already hold stays yours. Standing down is forward-looking: it stops authority being used again, and the kernel never reports that it reached work already dispatched.

### Proving an account is yours

**You can prove an account is yours without giving anything away.** For any account your identity holds, you can obtain a proof that it is yours, addressed to a recipient you name and valid for a period you name. The proof carries no authority to move value, cannot be used by a recipient it does not name, and cannot be used after its period ends. You are shown the statement being proved before it is issued; nothing is ever proved on your behalf that you were not shown, and no application can obtain such a proof without you.

### Readiness, visibility and separation

**An account is either ready to receive or says what is missing.** Every account the kernel reports says whether value can reach it now. Where a network requires something before an account can hold or use value, meeting that requirement is the kernel's work and not yours to know, and the kernel does not describe an account as ready before it is. Where the requirement falls on whoever sends to the account, the kernel states it, in your settlement asset, before you hand the account out.

**The kernel makes no secrecy claim for your accounts.** What the kernel can compute, anyone who knows what it knows can compute. An account you disclose can be used to find the others, and activity under one identity can be connected by anyone watching. The kernel states this rather than letting one identity everywhere be mistaken for one identity nobody can follow.

**You can keep things apart.** One identity may hold separate compartments, each with its own accounts, its own spending policy and its own receiving policy. A capability reaches exactly one compartment and can never reach another. A compartment bounds authority, exposure and accounting; it does not bound observability, and the kernel never suggests otherwise.

**Where an outcome requires something about you, that is part of whether it is reachable.** Some outcomes are open only to a holder with a standing that some party verifies. The kernel treats such a standing exactly as it treats a cap, a deadline or a grade: a condition tested before anything moves. An outcome you are not eligible for is reported as unreachable, with the missing condition named, rather than quoted and then refused. A standing carries its own expiry and can be withdrawn by whoever granted it; the kernel checks it when it is used rather than when it was obtained.

---

## 3. Spending

One spending policy, written once, binding on every chain.

A policy sets, in your settlement asset:

- the most that may leave your accounts in one movement, per intent, per period, and in total;
- how often value may leave at all;
- when each period begins, and whether budget unused in a period is forfeited at its end or carried into the next;
- which counterparties and applications may be paid at all, and the most each may be paid;
- the worst terms you will accept — the least you will receive, or the most you will give, per unit of what you are buying;
- a size above which a delegated capability is not sufficient authority and the identity itself must authorize the movement;
- the most any named application may charge you, as a rate and as an absolute ceiling;
- who pays execution costs, and where a sponsor is covering them, the bounds that sponsor has set.

A policy whose period has no stated beginning, or whose carry-over is unstated, is refused when it is written. It is never interpreted later.

**Authority is charged when it is created.** Every authority the kernel creates on your behalf is charged against your caps at the moment it is created, for the most it could move — not when it is exercised, and not only where value finally moves. This holds for authority a counterparty exercises rather than you, for authority that lasts longer than the step that needed it, for authority created by signature rather than by movement, and for the cost of execution, which is reserved at its worst case before work begins. Authority used for less than it could have been, or that ends unused, is released back to your caps promptly, and the release is reported. The kernel never holds a reserve after the work it was set aside for has ended.

**No authority survives its intent.** Authority the kernel creates may be exercisable by a named counterparty while the intent that created it is alive. It never lasts longer. Every authority carries a maximum amount and an expiry no later than the intent's own, and the kernel creates none without both. Where an outcome could only be reached by authority the kernel cannot bound this way — without a ceiling, without an expiry, or exercisable after the intent ends — the outcome is refused before anything moves, and the reason given is the authority, not the price or the route. Authority to move a specific amount exists only inside a specific intent and expires with it. What is live is on your record while it is live, and what cannot be bounded that way is refused rather than granted.

**Caps are enforced where authority is created and where value moves, not in the interface.** An application that misreports a fee, a venue that charges more than quoted, and a bug in your own client all fail the same way: the movement does not happen. The cap is not a display convention.

**A cap has a unit, a moment and a band.** Every cap is a quantity of your settlement asset. A quote states the reference used to value anything else against that asset, when that reference was taken, how long the quote is good for, and who absorbs movement in it. A longer validity window may cost more, and what it costs is part of the quote. Movement within the band the quote states is absorbed by whoever the quote says absorbs it. Movement outside the band voids the quote: the intent is refused before value moves, and is never repriced against a cap you signed under a different assumption. The kernel will not value an asset on a reference it cannot obtain fresh, and will not move an asset it cannot value. Where value is returned to you, it is returned in the amount actually recoverable, and the quote that priced the outgoing movement does not govern it.

**Running out stops work; it never becomes debt.** When a budget is exhausted the kernel stops taking on new work. It does not fall back to charging you, and a sponsor's shortfall never converts into your liability. Work already in flight is completed from a reserve set aside before it began, so stopping never strands something half-done.

**Work that fails still costs.** An attempt that does not reach the outcome can still consume budget. The kernel charges what attempts actually cost, inside the caps you signed, and reports it whether the outcome was reached or not. It never charges you for a cost caused by an operator's fault. A returned consideration is not a returned cost: where an outcome was not reached and your consideration came back, the cost of attempting it is stated separately.

**What is live is readable.** At any moment you can read the authority outstanding against your identity, the budget reserved against it, the budget remaining in the current period and when that period ends, and for each reserve the condition that releases it and when that is expected. A reserve awaiting release is reported as reserved, never as spent and never as available. Where the kernel cannot source execution costs on a path you need, that is reported as a refusal with its reason, never as work that is merely slow.

**Revocation is forward-looking and says so.** Withdrawing an application's capability stops it being used again. It does not reach back into work already authorized, and the kernel never reports that it did. Revoking states what was still live when it took effect: the intents already authorized, and the budget already reserved against them. A single withdrawal reaching every capability at once is always available. A capability's holder may renounce it without you. Budget reserved for work a revocation stopped is released to your caps, not stranded, and a capability re-issued under a name you have used before begins with a stated budget — the kernel never silently inherits or resets what an earlier capability of that name had spent.

---

## 4. Receiving

One receiving policy, written once, binding everywhere. It has two halves: what you accept, and where accepted value lands.

You state what you accept — value you are expecting, value from counterparties you name, assets you name — and everything else is unsolicited. Accepted value is routed to your destination in your settlement asset without your involvement, on whatever chain it arrives, from whatever venue, so you never maintain a destination per chain. Unsolicited value is recorded where it sits, attributed to you, reported to you, and otherwise untouched: not priced, not converted, not moved, and never merged with your settlement destination or with value from another source, until you decide it.

**Nothing arriving unbidden spends your money.** The kernel prices, routes, converts and consolidates nothing for value you did not ask for, and nothing for value whose delivery would cost more than the value is worth. It reports the shortfall and leaves the decision with you.

**The kernel never runs an unrecognized asset's own logic inside your settlement path.** An asset whose transfer, conversion or redemption behaviour it cannot evaluate in advance is left as it arrived and reported as unroutable. It is never carried through a route on the strength of what it claims about itself.

**Nothing acting for you can change where your value lands.** Your receiving policy is set by your identity and by nothing acting for it. No capability, however broad, and no application, however trusted, can change where your value lands or what you accept. A capability may read what you have received, and may direct value it is owed to itself where the intent said so before you signed — and that is the whole of its reach over receiving.

### What you are owed

You can state what is owed to you before it arrives: the amount, the asset, the counterparty where you know it, your own reference, and the time by which it is due. That is an expectation, and it is what makes an arrival expected. Value matching an expectation is applied to it and routed. Value matching no expectation is unsolicited, whatever it is and whoever sent it.

A request you hand to a payer carries the asset, the amount, your reference and the deadline, and names no chain. Which network a payer uses to satisfy it is their routing decision, and you are told which one they used after the fact.

An expectation ends in exactly one of: met, met in part, met in excess, met late, or unmet at its deadline. Each is reported as itself. An amount short of what was due is never reported as met, an expectation is never quietly extended past its deadline, and value beyond what was due is reported as excess rather than absorbed. These are the states of an expectation and they do not widen the outcomes an intent can reach.

### Receipts

Every receipt carries the same two marks as every outcome. A receipt marked contingent names the condition that could still undo it and when that condition closes, and it is reported and not spendable until then. Value is never reported as received because a sender announced it, only because the kernel observed it, and never as final because time passed.

A receipt states the amount actually credited, measured after arrival; it never repeats the amount a sender said they sent. It names its asset by what issued it, never by the name the asset carries.

Receipts are records. Each has a stable identity, each is enumerable from any point you choose, and reporting the same receipt twice reports the same record — so an application that credits a payment twice is a bug the interface did not create. Your statement covers value received as well as value spent, and is exportable.

### Claims and holdings

An open claim states what is owed, where it sits until you act, what taking it requires, what taking it would cost you in your settlement asset, and whether it expires. A claim that cannot state its cost and its custody is not reported as a claim, because you could not decide about it.

You have four exits from a claim, and the interface offers all four: take it to your destination; take it once to a different destination without changing your policy; return it to the origin of the receipt; or abandon it. Abandonment is recorded, is permanent, and is stated to be permanent before you choose it. **A return is addressed to the origin of the receipt, never to an address you supply** — the kernel already knows where value came from, so it never asks you to retype or paste an address to send value back. A return is a new outbound movement under your policy and your caps, and is never presented as a reversal of what arrived.

What you hold is reported in three states that are never merged: **spendable**, **accruing** and **claimable**.

Accruing value is value growing in place that no transfer has delivered. It is reported with what it has accrued and it is not spendable. A mechanism that can revise it downward is reported as one.

Claimable value exists only if an action is taken, and the report names the action, its cost in your settlement asset, and the deadline after which the value ceases to be yours. The kernel does not let such a deadline pass without telling you it is closing, and does not spend your budget to take something worth less than it costs to take. Authority to realize a claim lives inside an intent with its own cap and expiry; there is no standing permission over your positions.

Positions that are locked or subject to a release condition are reported with that condition and are never shown as spendable balance. A better-than-quoted result belongs to whoever the intent says it belongs to; it is never assigned by default to whichever party touched it last.

---

## 5. Orchestration

You declare an outcome. You never describe a route.

**An intent bounds both sides.** A cap bounds what may leave your accounts. A floor bounds what must arrive. Every intent that acquires, disposes of or converts value carries both, and the kernel enforces the floor exactly as it enforces the cap: where value moves, not in the interface. A route that cannot hold the floor is not offered. A result below the floor is not a settlement, and is never reported as one. An intent with no floor is refused rather than served at whatever price the route returns.

An intent names what you want, the cap, the floor, the deadline, the minimum acceptable grade, and what should happen if it cannot be reached. It does not name chains, venues, sequence, gas, or signing. From that the router selects venues and counterparties, sequences whatever steps the outcome needs across however many chains, sources execution costs on each, and handles failure of any individual step. A route that cannot satisfy the caps, the floor, the deadline, the grade or the recovery requirement is not offered to you; the kernel refuses before value moves rather than discovering it midway.

**A floor is priced against a reference, and the reference is disclosed.** Every quote states the reference price it was assessed against, the deviation of the quote from that reference, and whether that reference is independent of the venues the route would use. Where the only available reference is the venue that would trade, the quote says so before you sign, because a price confirmed only by the party quoting it is not confirmed. An intent may require an independent reference, and one that requires it where none exists is refused before anything moves.

**Authority expires by construction.** Every intent carries a deadline, the deadline bounds the window in which any authority derived from it can be used, and the kernel refuses an intent whose authority would remain usable after its deadline has passed. There is no open-ended authorization, and no intent that can be executed later at a price its signer never saw.

**A quote is valid for a stated period, and time is a term.** A quote states when it expires, what the outcome is expected to take, and the longest it can take while still being served. The long figure is the one the kernel is held to. Where a route's speed depends on a shared resource that can be exhausted, the quote says so and states what the outcome becomes if it is. Signing against an expired quote is refused; re-quoting is an ordinary priced call.

### Preview

**Nothing is signed unseen, and this is the call that makes that true.** `preview(intent)` returns, in your settlement asset, the complete exposure of an intent before it is signed: the most that can leave, the least that must arrive, the reference price and the deviation from it, the deadline, every charge with its payer, every commitment with its release condition and delay, the grade that will be achieved, the expected and worst-case time to settle, and what you are left holding on each way the intent can fail. Signing is refused for an intent whose preview was not produced. The preview is the kernel's own statement, not the application's rendering of it, and an application cannot alter, suppress or relabel it.

A preview states what the kernel will enforce. It is not a prediction of the market: only the caps, the floor, the deadline and the grade are guaranteed.

### Intents that wait

**An intent may be immediate, or it may wait.** One record covers conditions, schedules and recurrence. A mandate states the condition or schedule under which it executes, the evidence class that establishes the condition and the staleness bound on that evidence, the number of executions it authorizes, the bound and floor for each, the total bound across all of them, and the moment the whole mandate expires.

A mandate is not standing authority. It authorizes a counted number of bounded executions and nothing else, it cannot be enlarged after signing, and it expires whether or not it executed. Revoking it stops every execution not yet dispatched.

- **A condition is established by evidence, never by assertion.** The mandate names the evidence class and freshness the condition requires, and a condition the kernel cannot observe to that standard is refused at signing rather than watched and missed.
- **The ability to execute is reserved before the mandate is accepted.** An authorization to act that cannot pay to act is not a protection.
- **A missed execution is reported as a missed execution.** A window that closes unexecuted is not carried forward, is not retried at a later price, and is never reported as a fill. Nothing is charged for an execution that did not occur.
- **Each execution is bounded on its own**, so a later execution cannot consume the whole mandate's allowance or settle at a price the earlier ones would have refused.

### Stopping and replacing

**Stopping is forward-looking, and the kernel states what is still live.** `cancel` stops future executions and future dispatch. It does not reach work already dispatched, and the kernel never reports that it did. It returns what it stopped, what remains executable, and the moment each remaining authorization can no longer be used. Until that moment the intent is reported as stopping rather than stopped, because an interface that says "cancelled" while value can still move is a false statement about your exposure.

An intent may be replaced rather than stopped. A replacement takes effect only once its predecessor can no longer be executed, so no window exists in which both are fillable.

### Progress and repetition

**Progress is reported, not guessed.** `status` reports position in the sequence: which steps are complete, which is in flight, which have not begun, what each is waiting for, and the earliest moment the state can change. An absence of news is never reported as failure, and a deadline passing is not evidence that work did not happen.

**Submitting the same intent twice causes one execution.** An intent is identified by what it authorizes, and a repeated submission returns the state of the first rather than starting a second. A retry re-presents authority already given; it never creates new authority.

**Where a dispatched step can be accelerated, the price of accelerating it is signed in advance or not offered at all.** An intent may carry an acceleration allowance inside its caps. Where none was signed, the step is reported as unresolved and the kernel waits; it never improves the offer with money the payer did not authorize.

### Risk that outlives the intent

**Where an outcome leaves a holding that someone else can close, that is stated before you sign and reported for as long as the holding exists.** The kernel reports the closure condition, the measurement by which it is judged, your current distance from it, any threshold at which the severity changes, and whether the closure can be triggered by circumstances other than your own holding. A holding whose closure condition the kernel cannot measure to the evidence standard the intent names is refused rather than entered.

The kernel reports this risk. It does not promise to prevent the closure, and it does not promise that you will be able to act before it happens.

### The application interface

An application works entirely in these terms. Calls are grouped by what they are for.

| Group | Calls | What it does |
|---|---|---|
| Identity | `identity`, `accountFor`, `delegate`, `revokeDelegation`, `delegations`, `proveControl`, `standDown`, `reinstate` | Read the identity and its accounts with their readiness; grant, withdraw and enumerate authority; prove an account is yours; stop everything and restore it |
| Spending | `setPolicy`, `policy`, `budget`, `grantSponsorship`, `revokeSponsorship` | Set and read the spending policy; read live authority, reserves and remaining budget; arrange for someone else to cover costs |
| Receiving | `setReceivingPolicy`, `receivingPolicy`, `expect`, `expectations`, `receipts`, `claims`, `claim` | Say what you accept and where it lands; state what you are owed; read what arrived; take, redirect once, return or abandon what is open |
| Planning | `capabilities`, `quote`, `preview`, `intentFor` | Ask whether an outcome is reachable and at what grade, price it, see the full exposure, and build the intent |
| Execution | `sign`, `execute`, `status`, `resume`, `cancel`, `replace` | Authorize, run, observe, recover from a client crash, stop future work, and supersede an intent |
| Reporting | `outcome`, `statement`, `positions` | The result and its grade, the complete exportable statement, and what you hold in its three states |

`capabilities(outcome)` answers whether the kernel can reach an outcome, at what grade, by when, and at what cost, without naming how — and where it cannot, names the missing condition. An application asks that, not whether a particular chain is supported.

`execute` runs to completion or to a reported outcome without further application involvement. Where the kernel needs authority it does not have, it stops and reports rather than proceeding with less.

`resume(intentId)` recovers a client that crashed. It reconciles what was already authorized and reports current state. It authorizes nothing new.

Applications do not sign payloads, choose venues, manage nonces, hold gas, parse venue responses, or handle per-chain retries. Those are not withheld as a convenience; an application that could do them could exceed the authority its user granted.

---

## 6. Outcomes

An intent is in exactly one state at any moment, and ends in exactly one terminal state.

| Outcome | Meaning |
|---|---|
| `Live` | Authority exists, is bounded, and nothing has been dispatched. The intent is waiting for its condition, its schedule or its next execution. It is cancellable, and it is not a result |
| `Settled` | You got what the intent asked for, at or above its floor. Carries its strength and finality marks |
| `PartiallySettled` | Part of the outcome was reached and is yours. The remainder was released; no authority for it survives |
| `Unresolved` | Work was dispatched and its result cannot yet be established. Your exposure is capped at what you signed |
| `Refunded` | The outcome was not reached and your consideration was returned |
| `Compensated` | The outcome was not reached and you were paid the compensation the intent specified |
| `Expired` | The intent's window closed without execution. No value moved and no authority survives. An ordinary ending, not a failure |
| `Superseded` | The intent was replaced. The successor is named, and the predecessor can no longer be executed |
| `OpenClaim` | Something is owed to you that could not be delivered to your receiving policy. The claim is stated, costed and holdable |

`Live` is the only state from which nothing has been dispatched, and the only one that is not a result.

**`Unresolved` is a real state, not an error, and waiting does not clear it.** It means authority was used and the result is genuinely unknown. It is left when the result is established, or when the recovery path you signed becomes safe to run — a specific condition, not a timer. An interface that turned this into "failed" after a delay would be guessing with your money.

**`Settled` means the financial result, never the transaction.** A transaction that succeeds while the payment inside it fails is not a settlement, and is never reported as one.

**A partial result is final as a partial result.** The filled part is yours and the rest is released. Nothing remains live to be filled later at a price you did not agree to.

**An outcome describes an intent, never a holding.** A holding that outlives the intent that created it is reported as a position, with its own release and closure conditions, for as long as it exists. A terminal outcome is never a statement that your exposure has ended.

**The statement is complete and exportable.** For every intent it records what was intended, what arrived and what left, in which asset, when, at what price against what reference, which charges fell to which payer, and the evidence class behind each figure. It is sufficient to reconstruct your history without consulting any chain.

---

## 7. What the kernel will not do

- **It will not promise atomicity across chains.** Multi-chain outcomes are sequences. A step can succeed while a later one fails, and you can end up holding part of the outcome. The kernel bounds what that costs you, states it before you sign, and pays the compensation you agreed. It does not claim a rollback it cannot perform.
- **It will not sign anything you cannot see.** There is no path, through any call or sequence of calls, that signs an opaque payload on your behalf. It will not prove anything about you that you were not shown, and a proof that an account is yours carries no authority to move value, names its recipient, and expires.
- **It will not create authority it cannot bound.** Authority without a ceiling, without an expiry, or exercisable after the intent that needed it has ended, is not created — the outcome is refused instead, however good the price. It will not issue an open-ended capability, will not extend or renew one that has ended, and will not let authority widen as it is passed on.
- **It will not execute at a price you did not bound.** An intent without a floor is refused. A cap on what leaves your account is not a price, and the kernel never treats it as one.
- **It will not price a cap on a reference it cannot obtain fresh**, will not move an asset it cannot value, and will not reprice a void quote against a cap you signed under a different assumption. Where the only available price comes from the venue that would trade, that is stated before you sign.
- **It will not charge you outside the caps you signed**, will not convert a sponsor's shortfall into your debt, will not bill you for an operator's mistake, and will not hold a reserve after the work it was set aside for has ended.
- **It will not tell you that failed work was free**, and will not present a returned consideration as a returned cost.
- **It will not present an exhausted budget, or a cost it cannot source, as work in progress.**
- **It will not accept a policy it cannot enforce**, including a period with no stated beginning or a carry-over it must guess.
- **It will not hold an identity that one lost device ends**, and will not let a restoration of authorization take effect without a delay in which you can refuse it.
- **It will not move you onto new accounts.** Changing what you authorize with never changes where value sent to you arrives.
- **It will not report an outcome without naming the authority that produced it.**
- **It will not route, convert, consolidate or price value you did not ask for**, and will not spend more to deliver something than the something is worth.
- **It will not run an unrecognized asset's own logic inside your settlement path**, and will not treat an inbound asset as your settlement asset because it carries that name.
- **It will not claim it can refuse an inbound transfer.** Networks let strangers push value at you and no one can decline it. What the kernel controls is what it touches: it can leave value where it is, keep it apart from everything else you hold, and tell you it is there. That is the whole of what is possible.
- **It will not let anything acting for you change where your value lands.**
- **It will not present what it knows about where value came from as a clearance.** Provenance is evidence of the same attested kind as any other imported fact, it changes as more is learned, and no report of it protects you from whoever issued the asset deciding otherwise later.
- **It will not let a claim deadline pass without telling you**, and will not take a claim on your behalf without authority that names its cost.
- **It will not carry a missed execution forward**, retry it later at a price you never saw, or charge you for it.
- **It will not report an intent as stopped while any authorization for it can still be used.**
- **It will not treat a preview as a promise about the market.** The preview binds the kernel to caps, floor, deadline and grade; a route that turns out worse than previewed but inside your bounds is not a breach.
- **It will not spend more to accelerate work than you authorized in advance.**
- **It will not prevent a third party from closing a position you hold**, and will not promise you a window in which to respond. Reporting the risk is the guarantee; preventing it is not.
- **It will not claim that the accounts it derives for you are unlinkable**, or that a compartment hides you from anyone watching a ledger. Compartments bound authority, not observability.
- **It will not present a standing it cannot show is still valid**, or quote an outcome you are not eligible to reach.
- **It will not treat two assets as the same because they share a name**, and will not show locked or accruing value as spendable.
- **It will not route through a path whose failure it cannot bound**, however good the price.
- **It will not leave you without a route to your own accounts.** Withdrawing support for a network removes a route the kernel offers, never your ability to reach what you hold.

---

# Part II — The kernel

Nothing in this part reaches an account holder or an application. It is how the guarantees in Part I are kept. The calls named here are internal: an application that could invoke them could exceed the authority its user granted.

---

## 8. Intent, settlement and session bounds

A session is bounded by two resources frozen when the instance is created: a transition allowance, consumed one unit per accepted transition and never reset, and a horizon beyond which no transition is admitted. Neither can be increased later. A session that cannot fit its worst-case transition count inside the allowance, or its last recovery action inside the horizon, is refused before any value moves.

A session has one anchored instance on Midnight and zero or more foreign legs.

```text
IntentTerms
  -> ResolvedOrder / Plan
  -> Quote
  -> proof-independent ClaimSpec manifest
  -> Intent
  -> intent signatures
  -> instantiated transition claims
  -> transition proof
```

Quote signatures and proof bytes never enter their own preimages.

The lifecycle is:

1. **Resolve.** `resolve(program, terms)` publishes the bounded relation, amounts, approved primitives, evidence policies, deadlines and the recovery policy. It publishes no wallet secret and no unrestricted signing authority.
2. **Quote.** A solver returns a `Plan` with typed legs and a dependency order, and a `Quote` binding `H(terms)` and `H(plan)`, itemizing every charge, reserving maximum metered work, and carrying a separate settlement-funding estimate.
3. **Sign.** The principal signs a selected bounded plan: the outcome, route, caps, evidence policy and materialization rules. The display reports every distinct asset, the maximum gross debit, net credit goals, all charges, deadlines, recovery exposure, the transition budget the session reserves, and the words "threshold-attested foreign results" wherever that is the evidence class.
4. **Admit.** One `Step` consumes the intent nonce, reserves consideration and authorized fees, creates the bounded `PendingIntent`, and fixes quote allocations. Nonce reservation, fund reservation and head update occur in that single transition. Only after finalized acceptance may an operator request a foreign signature.
5. **Dispatch.** Per leg: materialize the exact foreign payload, verify it, request a native signature bound to the decoded network, assemble, broadcast. A dropped acknowledgement reconciles the recorded submission and re-broadcasts the identical bytes. It never produces a second signature.
6. **Import.** Per leg: collect evidence under the pinned policy, verify it, and apply `ImportFrom` against the current head. One leg's import does not require another leg's result and does not declare the intent settled.
7. **Settle.** A final `Step` checks actual — not projected — gross debits, net credits, fees and liabilities, and moves the intent to a terminal state.

Continuations address the pending record by `pendingId` and are made unique by the leg and import nullifiers of section 7. They do not re-consume the intent nonce.

### Serialized progress

Predecessor fan-in is one, and an accepted transition binds exactly one predecessor: the current head. Two consequences are normative.

First, every head-advancing action is serialized per instance. The SDK keeps at most one head-advancing transaction in flight for an instance. A second concurrent attempt is refused as stale rather than queued behind an unknown outcome, and is retried against the new head. Legs proceed in parallel off-ledger; their imports land one at a time.

Second, the head a continuation advances is not known when the principal signs. A signed statement binds the exact predecessor, so an import prepared for a later head is not covered by the admitting signature. The interface therefore requires a continuation authority whose signed statement binds the pending record identity, the leg and the attempt instead of a predecessor digest, with freshness supplied by the pending record's deadline and the nullifiers. Until that authority is admitted, each import needs a fresh principal signature over the then-current head, and the specification says so rather than presenting an unattended import path that does not exist.

### Session budget

The admitting `Step` reserves a transition budget and refuses the session if the instance cannot pay it:

```text
required = 1 (admit)
         + sum over legs of (maximum authorized attempts per leg)
         + 1 (settle)
         + maximum recovery transitions
```

The settlement and recovery actions are declared under the language's closure reserve rule, so allowance exhaustion cannot leave a funded pending record with no representable transition. All deadlines, including `recoverAfter` and the last recovery action, fall strictly inside the instance horizon. A route whose recovery cannot complete inside the horizon, or inside the reserved allowance, is refused at quote verification, not discovered at recovery time.

Bounds are simultaneous. A representable pending state can have no representable successor. The coordinator program's declared observation fields, effects per action and record widths are sized for the maximum number of legs and imported facts the profile advertises, and a plan exceeding any of them is refused before admission.

### Leg lifecycle

Each leg advances through the states below. The right-hand column names the record that makes the transition idempotent across a crash at any point.

| Transition | Precondition | Record that makes it idempotent |
|---|---|---|
| Planned → Authorized | Verified payload, budget reserved, network binding decoded | `SignatureCertificate` keyed by `(pendingId, legId, attempt)`; a crash after signing re-reads it and never re-signs |
| Authorized → Submitted | Assembled bytes equal the certified preimage | `ForeignSubmission` keyed by the signed payload digest; a crash after signing and before acknowledgement re-broadcasts those exact bytes |
| Submitted → UnknownExecution | Acknowledgement lost, or the venue's status vocabulary returns a value the pinned extractor does not recognize | `ForeignSubmission`; the leg is not retried with new bytes while this state holds |
| Submitted or UnknownExecution → Imported | An accepted `ImportCertificate` under section 7 | Source-event nullifier, then leg-progress nullifier |
| Imported → Imported | A further fill of the same leg | Source-event nullifier per event, with cumulative quantity checked against the authorized quantity |
| Any → Closed | Settlement, recovery, or a proven non-execution | Terminal marking of the pending record, which admits no further import for that leg |

`UnknownExecution` is a first-class state, not an error. A leg in it has consumed authorization and may or may not have moved value. The only exits are an import, a proven non-execution, or the signed recovery path. Elapsed time is not one of them.

A partial fill is an imported fact with an executed quantity below the authorized quantity. The imported quantity is recorded as executed, the remainder is released, and no residual authority survives. A second economic fill requires a new authorization. The interface never converts an unfilled remainder into a standing instruction.

Re-broadcasting identical bytes is safe only where the venue itself rejects the duplicate. Each adapter names the venue field that gives that guarantee and the window over which it holds. An adapter that cannot name one does not get a dispatch path, because a crash between signing and acknowledgement would otherwise be indistinguishable from a second transfer.

### Flow conservation

For every exact asset, across all participants including counterparties and fee collectors:

```text
sum(all account deltas for that AssetId) = 0
```

Conservation per asset is an accounting check performed outside the transition relation, on ledger-visible flows. It is not evidence of cross-chain atomicity, and no equality is asserted between assets on different chains. Foreign network fees paid by a solver appear as their own flows and charges, and are never subtracted from the principal's promised credits.

---

## 9. Language dependencies

The gap labels below are the ones the kernel design uses. Each capability class in section 18 names the gaps it requires.

| Gap | What must change | What fails without it |
|---|---|---|
| L1 | The composition rule joining the expression core to the financial objects | The coordinator cannot express both the orchestration record and the financial effects in one program |
| L2 | `permittedCalls` opened with a domain-qualified callee, a signed per-call cap and a payload digest | A non-empty call permission is rejected outright, so no intent can authorize an off-ledger action; and callee with selector alone restricts neither arguments nor amounts |
| L3 | Domain-qualified asset identity `AssetId { domain, issuer, assetReference }` | A bare asset text cannot distinguish the same symbol on two chains, and caps silently aggregate across them |
| L4 | A pending state with bounded contents, a deadline, bounded recovery, and a continuation authority that binds the pending record instead of a predecessor digest | Pending is rejected, and every continuation needs a signature over a head that is unknown when the principal signs |
| L5 | A signed nominal liability cap | Debit caps do not bound new nominal obligations, so a leg can create debt no signed cap covers |
| L7 | A fee capability type | The fee effect exists and the authorization to charge it does not |
| L8 | Meter counters as a family that never nets against the existing counters | Orchestration work either goes unpriced or is netted against transition work that bounds a different thing |

L2 authorizes; it does not make Midnight execute or observe anything. Every resulting fact still enters by import.

Fan-in above one is **not** a dependency. The import sequence is designed for fan-in one: each import advances the single pending record, and the settling transition reads that record rather than joining several predecessors. A higher fan-in would raise import throughput and is not required for correctness.

Two items that earlier appeared on this list are not language changes and are not carried here. Foreign signing authority bound to a decoded payload is a property of the signing service and its adapters, outside the settlement contract, and is specified in section 6. A solver interface is an SDK publication, specified in sections 2 and 5. Neither alters the language, and listing them as language gaps hid where they must actually be enforced.

An SDK cannot simulate any admitted gap with local bookkeeping. Where a class's gap is unadmitted, the internal pipeline returns `E_LIFECYCLE_UNSUPPORTED` or `E_PROFILE_NOT_ADMITTED` and names the missing change, and the router declines every path in that class.

### State binding

The signed statement already binds the execution domain, the genesis digest, the instance, the program, the required claim root, the validity window and the exact predecessor, and a mismatch in any of them rejects. What the multichain mode adds is not a head binding but its complement: the admitting `Step` binds the exact head and reserves funds against concurrent intents, and every continuation binds the pending record identity, the leg and the attempt. A profile that leaves the continuation binding implicit is an authorization gap, and `E_STATE_BINDING_UNDEFINED` rejects it.

---

## 10. Records

Identity and reference: `ChainRef`, `AccountRef`, `InstanceRef`, `HeadRef`, `AssetId`, `Amount`.

Authority and policy: `BoundsProfile`, `CallPermission`, `ForeignAuthorization`, `EvidencePolicy`, `NetworkBinding`, `RecoveryPolicy`.

Economics: `FeeCapability`, `FeeBudget`, `ForeignFeeBudget`, `MeterProfile`, `CapacityReservation`, `SponsorGrant`, `ChargeLine`, `JobTicket`, `MeterReceipt`, `PriorityBid`.

Execution: `IntentTerms`, `Leg`, `Plan`, `Quote`, `PendingIntent`, `LegProgress`, `ClaimSpec`, `TransitionCore`, `ProofRequest`, `TransactionBundle`, `EffectRecord`, `SettlementRecord`.

Foreign: `AdapterManifest`, `ForeignPayload`, `SignatureRequest`, `SignatureCertificate`, `ForeignSubmission`, `ForeignFact`, `ImportCertificate`.

`LegProgress` is the per-leg crash-recovery record inside `PendingIntent`. It carries the leg identity, the current state, the current `attempt`, the digest of the certified payload, the recorded submission, the cumulative executed quantity per asset, and the nullifiers already consumed. `attempt` is part of the signed continuation and never increments on its own; section 7 states what an increment requires.

`ForeignSubmission` records one dispatch: the signed payload digest, the exact broadcast bytes, the venue's duplicate-rejection field and window, the acknowledgement if one arrived, and the venue identifiers needed to query the result. It is what `reconcileSubmission` reads, and it is why a lost acknowledgement never becomes a second transfer.

`ForeignFact` carries an outcome tag drawn from `Executed`, `PartiallyExecuted`, `NotExecuted` and `Indeterminate`, the executed quantity per asset, the venue's own success discriminant, and the outer transaction status as a separate field. The two are never merged, and `Indeterminate` is the required tag whenever the pinned extractor meets a status value it does not recognize.

The budget record is:

```text
FeeBudget {
  applicationCap,
  serviceCap,
  foreignCapsByLeg,     -- LegId -> ForeignFeeBudget
  priorityCapsByLeg,
  recoveryReserve,
  grossDebitCapsByAsset
}
```

Every total is a vector by domain-qualified asset. One scalar that adds a settlement asset, a native gas token and a stablecoin is not a total.

Typed foreign arguments are one record per registered primitive, never a generic blob. A record enters this surface when its adapter is admitted at the capability class that uses it, and not before; an argument type for an unadmitted route is a signature the SDK can construct but must never issue.

---

## 11. Internal pipeline

Grouped by stage. Each call returns `Verified<T>` or a typed refusal from section 10.

- **Profile and adapter**: `loadProfile`, `listAdapters`, `adapterManifest`.
- **Authority**: `createCapability`, `revokeCapability`, `deriveForeignAccount`, `grantSponsorship`, `revokeSponsorship`.
- **Pricing**: `quoteWork`, `reserveQuote`, `buyCapacity`, `estimateSettlementFunding`.
- **Planning**: `resolve`, `requestQuotes`, `verifyPlan`, `prepareIntent`, `createSigningRequest`, `signIntent`, `verifyIntent`.
- **Admission**: `prepareTransition`, `buildProofRequest`, `bundle`, `verifyBundle`, `submit`, `awaitFinality`.
- **Foreign**: `materializeForeign`, `verifyForeignPayload`, `createForeignSigningRequest`, `signForeign`, `assembleForeign`, `broadcastForeign`, `reconcileSubmission`.
- **Import**: `collectImport`, `verifyImport`, `applyImport`.
- **Recovery and exit**: `resume`, `settle`, `recover`, `claimFees`, `closeInstance`.

`resume(pendingId)` reconstructs the session from the anchored pending record and the local submission records, reconciles every leg whose recorded state is `Submitted` or `UnknownExecution`, and returns the reconciled record. It is the only supported entry after a crash, and it issues no signature and broadcasts nothing.

Primitive registration is not on this surface. Admitting a new foreign primitive changes what the interface can be made to do, so it is a governed deployment change to the profile and the adapter set, reflected here only by `listAdapters` and `adapterManifest`. A runtime call that registers a primitive is an unrestricted call by another name.

### What the surface refuses to compose

Section 13 forbids certain capabilities. The call surface is constrained so that no sequence of permitted calls reconstructs them.

| Constraint | Failure it prevents |
|---|---|
| `SignatureRequest` is constructed only by `createForeignSigningRequest` from a verified payload, and is not constructible by a caller | A caller assembling a request around an arbitrary digest is `signHash` |
| `signForeign` accepts only a verified payload whose network binding decoded, and refuses a payload it did not verify itself | Signing bytes whose network and decoded fields were never checked |
| `assembleForeign` recomputes the signature preimage from the payload and refuses a signature issued for a different preimage | Pairing a signature obtained for one payload with another |
| `broadcastForeign` accepts only assembled bytes whose digest equals the digest in the `SignatureCertificate`, and records `ForeignSubmission` before transmitting | Broadcasting unsigned or substituted bytes, and losing the record of what was sent |
| `submit` and `applyImport` accept only verified bundles and verified certificates | Skipping verification by calling the next stage directly |
| `deriveForeignAccount` derives only under the caller's authenticated principal and never accepts a caller-supplied path | Deriving, and then using, an account that belongs to another principal |
| `recover` requires `recoverAfter` to have passed, the recovery policy to be the signed one, and every leg to be in a state the policy covers | Recovering around a leg that is still executable |
| `closeInstance` refuses while any obligation, reservation or unresolved leg remains | Closing over residual liabilities |
| `revokeCapability` and `revokeSponsorship` bind future admissions only | Retroactive revocation of a dispatched obligation |

`estimateSettlementFunding` returns an estimate. Settlement funding is arranged outside the signed statement and is not visible to the transition relation, so under-funding is a liveness failure and never a validity result. The SDK never presents a funding offer as evidence.


### Public calls and what they drive

Part I's interface is the whole of what an application calls. Each of its calls drives a sequence of the internal calls above. The mapping is fixed: an application cannot reach an internal call except through the public one that drives it, and no public call expands to a sequence its caller could not have authorized.

| Public call | Internal sequence |
|---|---|
| `identity`, `accountFor` | `deriveForeignAccount` under the authenticated principal only; readiness from the adapter manifest |
| `delegate`, `revokeDelegation` | `createCapability`, `revokeCapability` |
| `delegations` | The capability set with its ceilings, consumption and expiry, read from admitted state rather than from an application's record |
| `proveControl` | A recipient-addressed, expiring control statement over a derived account. It is not a `SignatureRequest`, carries no payload authority, and does not reach `signForeign` |
| `standDown`, `reinstate` | Epoch invalidation across every capability at once, then its reversal. Neither reaches dispatched work |
| `setPolicy`, `policy` | Bounds recorded in `BoundsProfile` and `FeeCapability`; enforced at `verifyPlan`, `submit` and `settle`, and at every point authority is created |
| `budget` | Outstanding authority, `FeeBudget` reserves with their release conditions, and remaining period allowance |
| `grantSponsorship`, `revokeSponsorship` | The same calls |
| `setReceivingPolicy`, `receivingPolicy` | Acceptance rules and destination recorded in `IntentTerms`; applied at `settle` and by the residual-claim path |
| `expect`, `expectations` | Expectation records matched against accepted imported facts |
| `receipts` | Accepted `ForeignFact` and settlement records, keyed by source-event nullifier so redelivery is idempotent |
| `claims`, `claim` | `claimFees` and the pending record's residual obligations. A return or a one-off redirection is a new outbound intent under the holder's policy; abandonment is recorded and performs no movement |
| `capabilities` | `loadProfile`, `listAdapters`, `adapterManifest`, then the routing check of section 18 |
| `quote` | `resolve`, `requestQuotes`, `quoteWork`, `reserveQuote`, `buyCapacity`, `estimateSettlementFunding`, `verifyPlan` |
| `preview` | The verified plan, quote and recovery policy rendered as one exposure statement. `sign` refuses an intent with no produced preview |
| `intentFor` | `prepareIntent`, `createSigningRequest` |
| `sign` | `signIntent`, `verifyIntent` |
| `execute` | `prepareTransition`, `buildProofRequest`, `bundle`, `verifyBundle`, `submit`, `awaitFinality`; then per leg `materializeForeign`, `verifyForeignPayload`, `createForeignSigningRequest`, `signForeign`, `assembleForeign`, `broadcastForeign`, `collectImport`, `verifyImport`, `applyImport`; then `settle` |
| `status`, `outcome` | The pending record's state and `SettlementRecord`, mapped to the public outcomes by section 15 |
| `resume` | `resume`, `reconcileSubmission` |
| `cancel` | Epoch invalidation for future admissions; `recover` once `recoverAfter` has passed and the expiry rule of section 15 is satisfied. Reports `stopping` until the last dispatched authorization lapses |
| `replace` | A successor intent admitted only after the predecessor's authority can no longer be exercised |
| `statement` | The `ChargeLine` set, accepted `MeterReceipt`s and accepted facts, totalled per asset with the evidence class of each figure |
| `positions` | `LegProgress`, accepted imported facts and residual holdings, separated into spendable, accruing and claimable |

A mandate is admitted as a `PendingIntent` whose continuation authority carries a counted execution allowance; each execution consumes one unit of the session's transition budget, and the mandate is refused at admission if the budget cannot cover every authorized execution plus settlement and recovery.

`closeInstance` has no public call. An instance closes when its intent reaches a terminal outcome with no residual obligation.

---

## 12. Refused by construction

- No `signHash`, `sendRawTransaction` or unrestricted `call`, and no sequence of calls that reconstructs one. These bypass decoded permission and exposure checks.
- No caller-supplied derivation path, and no derivation under a principal other than the authenticated one.
- No caller-constructed signing request. A request exists only as the output of verifying a payload.
- No runtime registration of a foreign primitive.
- No arbitrary remote execution inside the language.
- No cross-chain atomicity promise.
- No chain label, metadata field or unverified chain identifier accepted as a network binding.
- No same-token alias across chains. Conversion or bridging is an explicit primitive with an inventory source and evidence.
- No automatically reusable partial-fill authority. A second economic fill requires new authorization.
- No re-signing on retry. A retry re-broadcasts the identical certified bytes, and a leg whose venue cannot reject the duplicate has no dispatch path.
- No unauthorized attempt increment, and no attempt increment on a timeout.
- No cancellation that erases dispatched work. Epoch invalidation prevents future admissions; it does not retroactively revoke an issued foreign signature, and nothing in the interface reports that it does.
- No import whose acceptance rests on an outer transaction status, and no fee amount reconstructed from an ambiguous aggregate. An adapter that cannot distinguish a component charge from a total, or an entry's outcome from its batch's, produces unavailable evidence.
- No sponsor-to-user fallback by surprise.
- No shared coordinator instance across sessions, and no attestor or policy rotation inside a live session.
- No session admitted without a reserved transition budget and a recovery deadline inside the instance horizon.
- No hardcoded resource prices, finality delays or auction latencies. All are versioned quote and policy inputs.
- No claim that priority buys finality or success.
- No unbounded nonce table or watcher.
- No release-time certificate route yet. Threshold imports are the first supported evidence class.
- No kernel guarantee for arbitrary foreign staking exits. The SDK exposes position and withdrawal conditions, not an unconditional liquid balance.

---

## 13. Cost model

**Five separately accounted planes.** Foreign execution consumes resources independently of both Midnight settlement and orchestration, so it is a plane of its own and never a line inside orchestration.

Every charge belongs to exactly one plane, decided by an ordered test:

1. A payment that is optional and buys only position in an ordering is a **priority** charge, whichever system collects it and in whichever asset.
2. A charge the agreement makes for its own service, carried inside the contract-call statement and counted by the signed gross-debit caps, is an **application** charge. Other in-statement debits are consideration, not charges.
3. A charge the Midnight ledger's resource mechanism deducts outside the contract-call statement is a **settlement** charge.
4. A charge a foreign chain or venue deducts under its own accounting is a **foreign execution** charge.
5. A charge Moriarty's meter ledger deducts for work it performed is an **orchestration** charge.

| Plane | Prices | Unit | Payer → recipient | Binding |
|---|---|---|---|---|
| Midnight settlement | Ledger acceptance | DUST | Transaction funder → Midnight's resource mechanism | Outside the contract-call statement. The circuit cannot see fee sufficiency, so an SDK funding offer is not circuit evidence. |
| Foreign execution | Execution resources, account activation, permission installation, per-signature charges, venue and bridge fees | Native atoms or the venue's fee asset | Named leg payer → foreign mechanism or venue | Signed `ForeignFeeBudget` with one line per charged resource; actual expenditure is imported evidence. |
| Orchestration services | Payload bytes, registered jobs, inventory commitment, observation exposure | Settlement-asset atoms | User, sponsor or operator → named providers | `Quote`, `JobTicket`, `SponsorGrant`, escrow, consumed receipts. |
| Application | The agreement's own service | Settlement-asset atoms, or the venue's collateral asset where only that asset can carry it | Principal → approved application | Revocable `FeeCapability`, signed intent, ordinary `Fee` effect, gross-debit caps. |
| Priority | Optional ordering preference | Venue-native amount or explicit rate | Opted-in payer → venue recipient or burn | Separate `PriorityBid` with its own cap, never inferred from an application fee. |

The test terminates for every payment and admits one answer, so the planes are exclusive. A charge line that resolves to two planes, or to none, is a defect: quote verification rejects the plan rather than booking the charge twice or silently.

A charge line carries one plane, one payer, one recipient and one domain-qualified asset. A priority charge is never netted against a net-credit goal, and it is charged even where the action it prioritized produced no fill.

### Commitments are not charges

Locked capital is not expenditure and has no plane. Foreign state deposits, staked or rented chain resources, venue collateral, operator minimum balances and local escrow are reported as a separate commitment vector by domain-qualified asset, each line carrying its release condition and the delay before release completes. A commitment is never presented as a cost, and a cost is never satisfied by a commitment.

The only priced part of a commitment is its financing. Where a third party supplies locked capital, it charges a disclosed financing fee on the orchestration plane; where the principal supplies it, nothing is charged and the lockup is disclosed in the signing display.

Commitment amounts are read from each network's live protocol configuration at quote time. They are never taken from documentation and never hardcoded, because the deployed value can differ from the documented one and can be set to zero on a variant deployment.

### Surplus is assigned, never captured

Every deviation in the payer's favour is assigned in the signed terms: a fill better than quoted, an unconsumed reservation, a refunded foreign resource. An unassigned surplus is a defect, and quote verification rejects a plan that produces one. No operator retains a surplus that the intent did not award it.

A refund is not whole. A network can charge a penalty on the refund of an unused reservation and can price a reservation above its nominal estimate by its own inflation ratio. The quote states the non-refundable part of every reservation as a charge, not as a contingency.

### What must have a payer

Work is performed only where a payer is named for it: quote solicitation and the verification of returned quotes, plan verification, payload materialization and verification, foreign signature jobs, submission, observation to the policy's finality depth, import verification, proof generation, cancellation, and recovery. A quote round is itself a priced job charged to its requester, so soliciting quotes has a cost and quote spam has a payer. Unpriced work is not performed, and an operator that performs it is not owed for it.

### Meters

Two dimensions, both of registered work, neither a claim about elapsed CPU time:

- **PU** — one byte of canonical encoded, authorized payload delivered. Counted once per designated delivery across envelope, selected quote, foreign payload and certificate bytes. Unsolicited quotes, redundant gossip and operator retries are excluded.
- **CU** — one registered work credit. A profile assigns fixed credits to a bounded job with an identifiable output.

A meter profile publishes, for every registered job, its CU credits, its PU accounting, and its price as an exact rational over the atomic unit of one named settlement asset. Published values are versioned and do not change for the life of a profile version. A profile carrying an unbenchmarked value is not admitted, and there is no default tariff that admission can fall back to.

### Venue request allowance

A venue's own request allowance is a charged resource that Moriarty cannot sell. It accrues to a specific address from that address's settled volume, is granted as a one-time endowment at address inception, is not transferable, and tightens further under venue congestion according to that address's prior share of activity.

The allowance is therefore metered and reserved per leg like any other resource, and is never purchased. A leg is admitted only where the acting address holds the allowance its dispatch, cancellation and observation actions consume. Exhaustion refuses admission; it never degrades into retry pressure against the venue.

### Foreign cost is a vector, not a field

A per-leg foreign budget carries one line per charged resource. A single native limit field is not a cap on a leg's cost: on Tron a contract call's execution resource is bounded by the caller's fee limit while its bandwidth, account activation, permission installation and per-signature charges are outside that bound entirely. A leg with any charged resource unbounded returns `E_FOREIGN_COST_UNBOUNDED`.

Exceeding a foreign bound is not free. A call that exhausts its limit still consumes the resources it used before reverting, so every reserve is sized to the reverted case and not to the successful one.

A venue can change its own fee schedule after a quote is issued, with no floor. The leg therefore binds a cap: an actual charge above the cap fails the leg predicate and is never resolved by enlarging the debit.

### Capacity is bought, not staked

Refundable stake cannot fund recurring operating costs without another revenue source, and a resource model that allots capacity as a share of total staked weight yields no deterministic amount from a fixed stake. Recurring orchestration capacity is therefore purchased: `reserveQuote` holds a priced allocation for a quote lifetime, `buyCapacity` buys a fixed reservation for a stated window. Security collateral is kept entirely separate from capacity purchase, and no collateral doubles as capacity.

Three capacities are distinct and are never conflated: purchased orchestration capacity, deterministic over its stated window; foreign chain resources, obtained at the chain's own rates by staking, renting or burning; and the venue request allowance, which cannot be obtained by paying for it.

### Bursty demand carries no unnamed subsidy

A capacity seller publishes the commitment window and the peak rate it serves within that window. It may not sell more than it serves; a seller that does breaches and owes the replacement cost of the work it failed to serve. Demand above sold capacity stops admission. No burst allowance is extended beyond sold capacity, because an unpriced burst allowance is a subsidy.

Two endowments granted by third parties are subsidies and are named as such: a venue's inception request buffer and a chain's free daily resource quota. Both are finite, neither renews on demand, and exhausting either silently shifts the cost to a resource burn. A profile declares any endowment a route relies on, and a route whose steady-state feasibility depends on one is not admitted.

### Exhaustion stops work

**When capacity and the signed monetary budget are exhausted, execution stops before any further work.** A priced fallback is used only where the payer authorized that fallback in the signed budget. A sponsor shortfall never becomes user debt, and an operator fault never becomes an unsigned user charge.

The stop is made safe by reserving completion before dispatch, not by exempting dispatched legs from the budget. `recoveryReserve` is held per leg and per domain-qualified asset, and covers:

1. the observation and import jobs that establish the leg's outcome, to the finality depth the pinned policy names;
2. the cancellation or recovery actions of the signed recovery path, sized to a foreign attempt that consumes resources and reverts;
3. the Midnight settlement funding for the import and settling transactions;
4. the prefunded compensation the recovery policy names;
5. the venue request allowance those actions consume.

The reserve is not spendable by another leg, another intent or another work class, and is released only when its leg reaches a terminal state. A leg whose reserve is not held is not dispatched, and `reserveQuote` refuses the plan.

The hard stop applies to the admission of new work. Work drawn from a reserve already held for a dispatched leg is not new work and is never refused for exhaustion. Exhaustion therefore cannot strand a dispatched foreign leg, because dispatch was conditional on the reserve that completes it.

The settlement component of the reserve cannot be enforced by the relation, because DUST is not bound into the contract-call statement and the circuit cannot see fee sufficiency. It is a funder condition verified before dispatch and re-verified before each continuation. Its failure is a liveness failure, is reported as one, and never stands as a settled financial result.

Sponsor escrow is required rather than optional, because the failure it prevents is observed in deployment: where a foreign contract's deployer absorbs part of a caller's execution cost, exhausting the deployer's allowance falls through silently to the caller's own balance. Committed sponsor funds are escrowed and cannot be withdrawn while any leg they cover is live.

### Charge reporting

The settlement record reports, per plane and per domain-qualified asset, the quoted cap, the amount actually charged, and the evidence class of that actual amount. Quoted and actual are never merged into one figure, and a quoted foreign fee is never reported as an incurred one. Every total is a vector by domain-qualified asset; one scalar that adds DUST, a native gas token and a stablecoin is not a total.

---

## 14. Operators and compensation

### Application fee

A proportional fee with an absolute ceiling, carried by a revocable `FeeCapability` and realized as an ordinary `Fee` effect inside the signed gross-debit caps:

```text
feeAtoms = floor(basis × rateTenthBps / 100,000)
```

The rate is an integer count of tenths of a basis point. The capability is granted by the principal and never by a session key, and no foreign signing key can grant, widen or renew one.

The basis is a named asset and a named amount in the signed terms. A fee whose basis asset or basis amount the principal did not sign is not charged. Where a venue can collect a fee only in its own collateral asset, the intent names that asset and the fee is charged in it; the fee is never reconstructed by converting a basis the user did not sign.

The effective cap on a route is the minimum of the rate in the capability, the profile's cap for that market class, and the venue's own enforced maximum where the venue enforces one. Each venue's enforced maximum is an adapter fact pinned against deployed behaviour and recorded in the `AdapterManifest`, not a constant of this document. A principal holds a bounded number of active capabilities, published in the profile.

Revocation and cap changes apply to future admissions only. They are never applied retroactively to an already dispatched obligation. Fees accrue and are claimed explicitly through `claimFees`; execution never blocks on a fee payout, and an accrual without backing returns `E_FEE_ACCRUAL_UNBACKED`.

A disclosed share of collected orchestration revenue is routed to the protocol, and a disclosed share of a primitive's base compute revenue is routed to that primitive's author where the author has opted in. Both shares are profile parameters, fixed for the life of a profile version and published before admission. Congestion charges, penalties and retries are excluded from the royalty basis.

### Roles

A role exists only where its work is separately priced and its misbehaviour has a consequence. Each role below is paid for a bounded output and loses something specific when it deviates.

| Role | Priced work | Most profitable deviation | What prevents it |
|---|---|---|---|
| Solver | Winning quote's fixed fee or disclosed spread; losing bids earn nothing | Win a round with a price it cannot honour, then fade or leave a committed leg unfilled | No completion fee without a proven fill; performance collateral covering the replacement cost of legs it holds; deselection on a published fill-failure rate; it bears its own foreign costs on a fade |
| Attestor | Fixed per-observation fee plus exposure premium, identical whichever outcome it reports | Sign a false fact that releases value to a colluding party | Fee independent of the reported outcome; bond covering the maximum a single import can release; objectively penalized equivocation; independence from the beneficiary; per-import exposure cap |
| Sponsor | Promotional sponsor earns zero; commercial sponsor charges a disclosed financing fee | Withdraw or exhaust the grant after dispatch, shifting cost to the user | Escrowed funds, non-withdrawable while a covered leg is live; shortfall fails closed before dispatch; no authority to enlarge user liabilities |
| Relayer | Fixed submission fee plus reimbursement bounded by imported actual cost | Submit twice, or claim costs it did not incur | One submission fee per leg-progress nullifier; reimbursement bounded by imported evidence; duplicate or unauthorized submissions unreimbursed and charged to service collateral |
| Prover | Fixed profile-priced fee for a verifying proof | Withhold or delay committed delivery to extract more | Payment only on a verifying proof; committed delivery windows priced at replacement cost; the job is reassignable |
| Foreign signer | Fixed signature-job fee plus quoted custody premium | Sign a payload other than the authorized one, or sign twice for a second economic effect | Authority bond covering the maximum exposure of the authorizations it holds; signature issued only against a verified network-bound payload; one recorded signature per leg attempt |
| Coordinator | Validation and payload tariffs | Over-report delivered bytes and job counts, or withhold dispatch | Receipts are per designated delivery and consumed once; the payer's counter-signature is a condition of claiming one; overbilling fails verification; unperformed jobs are unpaid and the session transfers at the pending record |

An issued foreign signature cannot be revoked. For the foreign signer the control is therefore the bond and the payload binding, never revocation.

A role posts an authority bond when a single deviation can profit it by more than its expected revenue from continued selection. Custody of a signing key and operation of a mutable data source always require one. Where no single deviation can exceed that revenue, exclusion from future selection is the enforcement and no bond is posted. Supplying an immutable artifact is not an operator role: it carries no liveness obligation and no custody, so its consequence is loss of future admission and royalty eligibility. An author who also operates an oracle or holds a key assumes that role's obligations and its bond.

### Collateral and adjudication

Collateral is sized by relation, not by a fixed figure:

- For each pool, the collateral forfeitable on a proven false import is at least that pool's aggregate unresolved exposure. Collateral may not simultaneously back another pool.
- Aggregate unresolved exposure per pool is capped, and admission stops automatically at the cap. The cap, the per-operator floor, the quorum size and the threshold are profile parameters published before admission; an unbenchmarked value blocks admission.
- The observation quorum is independent of the solver and of any party the observation benefits. Its threshold carries a published threat model naming what it does not protect against, including a common faulty data source.
- A bond exits no earlier than the published notice period after notice, and not before the last covered exposure closes. An unresolved dispute extends the lock.
- Penalties compensate verified losses and replacement costs first; only a residual is disposed of otherwise. Penalties are not burned, because a burn pays no victim.

This is collateral arithmetic, not a safety proof. A dishonest quorum can agree on one false claim without equivocating, and signatures alone do not distinguish that case. Adjudication of it uses a separately identified, deployment-bound quorum whose verdict is disclosed as **another trusted import**. Where that trust is unacceptable, the route does not launch before certificate verification exists.

---

## 15. Error contract and failure handling

Every refusal is typed, names the missing artifact, and never degrades into a best-effort attempt.

A refusal is a pre-dispatch result. No call returns a refusal after it has caused a foreign broadcast or an accepted Midnight transaction. From that point the only results are the lifecycle states below, and `reconcileSubmission` returns a state rather than a refusal. A refusal leaves no reservation consumed, no nonce spent, no signature issued and no payload in flight. Any code path that can emit a refusal after `submit` or `broadcastForeign` is a defect, and the crash-boundary evidence required by section 18 exists to find it.

Three properties hold of the code set. Every code names the exact artifact that is missing, wrong or unavailable. No two codes cover the same condition. Every code is emitted by at least one named call, so an unemitted code is a defect in the set rather than a reserve for future use.

### Codes

Profile, adapter and lifecycle:

| Code | Emitted by | Condition |
|---|---|---|
| `E_PROFILE_NOT_ADMITTED` | `loadProfile`, `prepareTransition` | The profile, or a language change it names, is not admitted for this deployment |
| `E_SCHEMA_NONCANONICAL` | every `verify` call, `collectImport` | The encoding is not the canonical form of its pinned version |
| `E_EFFECT_NOT_REGISTERED` | `loadProfile`, `prepareTransition` | The effect or primitive is not in the admitted set |
| `E_ASSET_UNSUPPORTED` | `resolve`, `quoteWork` | The asset's domain or issuer is not registered |
| `E_LIFECYCLE_UNSUPPORTED` | `resolve`, `prepareIntent` | The requested lifecycle requires an unadmitted language change, which the refusal names |
| `E_PROOF_ROUTE_UNSUPPORTED` | `buildProofRequest` | No admitted proof route produces the requested statement |
| `E_STATE_BINDING_UNDEFINED` | `loadProfile`, `prepareIntent` | The profile does not define which digest binds the head and which binds the pending id |

Authority, network binding and payload:

| Code | Emitted by | Condition |
|---|---|---|
| `E_AUTH_INVALID` | `createCapability`, `signIntent`, `signForeign` | The presented authority does not cover the requested action |
| `E_SIGNATURE_INVALID` | `verifyIntent`, `verifyImport`, `assembleForeign` | A signature does not verify over the canonical preimage |
| `E_SIGNING_DOMAIN_INVALID` | `createForeignSigningRequest`, `verifyForeignPayload` | The decoded signing domain is not the pinned domain of the selected network |
| `E_NETWORK_BINDING_MISSING` | `verifyForeignPayload`, `createForeignSigningRequest` | The payload's network identity is not decodable from the signature preimage |
| `E_NETWORK_BINDING_UNSOUND` | `verifyPlan`, `createForeignSigningRequest` | The binding decodes, and its class is not admitted for this leg's value class |
| `E_BINDING_WINDOW_UNSAFE` | `createForeignSigningRequest` | The signature remains executable at the venue past the leg deadline plus the recovery reserve |
| `E_KEY_NETWORK_REUSE` | `deriveForeignAccount`, `signForeign` | The derived key is already pinned to a different network of the same family |
| `E_CUSTODY_CONTRACT_REQUIRED` | `verifyPlan`, `createForeignSigningRequest` | A value-releasing leg on this venue has no pinned custody contract |
| `E_SIGNER_PAYLOAD_OPAQUE` | `createForeignSigningRequest` | The signing backend accepts only an opaque digest, so no decoded-payload check binds the signature at the signer |
| `E_PAYLOAD_MISMATCH` | `verifyForeignPayload`, `assembleForeign` | Materialized bytes differ from the authorized payload |
| `E_APPROVAL_UNBOUNDED` | `verifyForeignPayload` | The payload grants an allowance or authority without an amount bound and an expiry |
| `E_NONCE_INVALID` | `submit`, `assembleForeign` | The intent nonce is already consumed, or the foreign nonce lies outside the accepted range |
| `E_HEAD_STALE` | `submit` | The admitting step's bound head is no longer current |

Payment, capacity and metering:

| Code | Emitted by | Condition |
|---|---|---|
| `E_PAYER_NOT_AUTHORIZED` | `quoteWork`, `submit` | The named payer has not authorized this charge class |
| `E_SPONSOR_GRANT_INVALID` | `grantSponsorship`, `reserveQuote` | The grant does not name the sponsor, covered classes and cap |
| `E_SPONSOR_UNFUNDED` | `reserveQuote`, `submit` | Committed sponsor funds do not cover the reservation |
| `E_BUDGET_EXHAUSTED` | any metered call | Capacity and the signed monetary budget are spent, and no signed fallback applies |
| `E_CAPACITY_NOT_RESERVED` | `quoteWork`, `buyCapacity`, dispatch calls | No reservation covers the requested work |
| `E_QUOTE_INVALID` | `verifyPlan`, `submit` | The quote is expired, unbound to the terms, or not itemized |
| `E_METER_RECEIPT_INVALID` | `claimFees`, `settle` | The receipt does not verify against the pinned meter profile |
| `E_RECEIPT_REUSED` | `claimFees`, `applyImport` | The receipt or foreign event already discharged an obligation |
| `E_FEE_CAPABILITY_INVALID` | `prepareIntent`, `submit` | The fee capability is revoked, expired or out of scope |
| `E_FEE_CAP_EXCEEDED` | `prepareIntent`, `settle` | The computed fee exceeds the signed rate or ceiling |
| `E_FEE_ACCRUAL_UNBACKED` | `claimFees` | The claimed accrual is not backed by settled funds |
| `E_FOREIGN_COST_UNBOUNDED` | `verifyForeignPayload` | The payload carries no enforceable foreign fee limit |

Amounts and bounds:

| Code | Emitted by | Condition |
|---|---|---|
| `E_GROSS_CAP_EXCEEDED` | `verifyPlan`, `submit`, `settle` | A gross debit exceeds the signed per-asset cap |
| `E_NET_GOAL_UNMET` | `settle` | Actual net credits fall short of the signed goal |
| `E_LIABILITY_CAP_EXCEEDED` | `submit`, `settle`, `recover` | A nominal liability exceeds the signed cap |
| `E_AMOUNT_INVALID` | `resolve`, `verifyPlan` | An amount is negative, non-integral in its unit, or unqualified by asset |
| `E_CONSERVATION_UNPROVED` | `verifyPlan`, `settle` | Per-asset deltas do not sum to zero across all participants |
| `E_PLAN_OUTSIDE_INTENT` | `verifyPlan` | The plan enlarges the authorized relation |
| `E_PARTIAL_FILL_UNAUTHORIZED` | `materializeForeign`, `applyImport` | A second economic fill is attempted under authority already discharged by a partial fill |

Evidence and import:

| Code | Emitted by | Condition |
|---|---|---|
| `E_TRANSITION_ALLOWANCE_EXHAUSTED` | `applyImport`, `prepareTransition` | The session cannot represent a successor: admitting this transition would consume the reserved allowance needed to settle or recover |
| `E_NONEXECUTION_UNPROVEN` | `materializeForeign`, `recover` | An attempt increment or a recovery step was requested without an accepted non-execution import or a proven venue expiry |
| `E_IMPORT_REQUIRED` | `applyImport`, `settle` | A signature certificate is offered where an import certificate is required |
| `E_EVIDENCE_CLASS_MISMATCH` | `verifyImport` | The evidence class is not the one the intent names for this leg |
| `E_EVIDENCE_NOT_FINAL` | `verifyImport` | Finality or freshness does not satisfy the pinned policy |
| `E_EVIDENCE_CONFLICT` | `verifyImport` | Two admissible observations of the same fact disagree |
| `E_FACT_MISMATCH` | `verifyImport`, `applyImport` | Extracted account, recipients, amounts, fees or liabilities fail the leg predicate |
| `E_OBSERVATION_POLICY_MISMATCH` | `verifyImport` | The policy hash is not the one bound in the intent |
| `E_QUORUM_INVALID` | `verifyImport` | The key set, threshold or signer distinctness does not match the deployed policy |
| `E_EXTRACTION_SOURCE_UNAUTHENTICATED` | `collectImport`, `verifyImport` | The network identity of the source rests on an endpoint's claim rather than an authenticated network witness |
| `E_RESULT_INDETERMINATE` | `collectImport`, `verifyImport` | The venue's response schema does not let the adapter separate per-item failure from success |

Recovery and exit:

| Code | Emitted by | Condition |
|---|---|---|
| `E_DEADLINE_INFEASIBLE` | `verifyPlan`, `prepareIntent` | A leg deadline cannot be met under the pinned finality and freshness policy |
| `E_RECOVERY_UNSAFE` | `verifyPlan`, `recover` | The selected recovery path can release a reservation while a dispatched authorization is still executable |
| `E_CANCEL_NOT_PROVEN` | `recover`, `settle` | A cancellation is asserted without imported evidence |
| `E_UNKNOWN_EXECUTION_UNRESOLVED` | `settle`, `closeInstance` | A leg is in `UnknownExecution` and its fate is neither imported nor time-barred by venue rule |
| `E_TERMINAL_STATE_INVALID` | `settle`, `closeInstance` | The requested terminal state is not reachable from the current pending state |
| `E_EXIT_HAS_LIABILITIES` | `closeInstance` | Residual obligations remain |

### Lifecycle states

These are the kernel's internal per-intent states. They map to the public outcomes of Part I section 6: `Succeeded` reports as `Settled`; a partial import reports as `PartiallySettled`; `Submitted` and `UnknownExecution` both report as `Unresolved`; `Refunded`, `Compensated` and `ClosedWithClaims` report as themselves, the last as `OpenClaim`. The public set is never widened by adding an internal state to it.

Product states are distinguished and never collapsed: `Submitted`, `UnknownExecution`, `Imported`, `Succeeded`, `Refunded`, `Compensated`, `ClosedWithClaims`.

`Succeeded` requires the specific financial result, not a successful outer transaction. The canonical case is a withdrawal path that emits a failure event and returns without reverting: the batch call succeeds, the batch's own receipt reports no error, and the funds do not move. An adapter therefore predicates success on the venue's success event for that exact item, never on the enclosing transaction's status.

`UnknownExecution` is entered whenever a dispatch has occurred and its outcome is not yet imported, including a dropped acknowledgement. It is left by import, or by a signed recovery once the dispatched authorization has expired by venue rule. A timeout alone does not leave it.

### Failure handling

When one leg succeeds and another cannot be proven: keep the imported fact, do not mark net goals satisfied, stop new dispatch at the signed cutoff, run the signed bounded recovery after `recoverAfter`, release unused local escrow, pay specified prefunded compensation, and preserve residual obligations. Return `Compensated` or `ClosedWithClaims`. Never `Succeeded`.

A deadline is not proof of non-execution. Releasing a reservation on a timeout can enable double spending; holding it can strand funds. The timeout does not resolve this, and the signature's own expiry does. Recovery is safe only where the dispatched authorization has already expired by venue rule at the moment recovery runs, so that the released reservation cannot be spent twice by replaying work already authorized. This is the expiry-before-recovery rule, and it is enforced at signing time: `createForeignSigningRequest` refuses with `E_BINDING_WINDOW_UNSAFE` any payload whose venue-enforced expiry is later than `recoverAfter`.

A venue that offers no enforceable expiry on the authorization it accepts cannot satisfy that rule. Routes with irreversible user-funded foreign execution on such a venue are admitted only where the selected recovery policy has prefunded compensation that survives quorum and operator failure, or a demonstrated bounded refund path independent of the venue's cooperation. Otherwise quote verification rejects the route with `E_RECOVERY_UNSAFE`.

### Ranked failure modes

Critical: a false import; recovery releasing funds while a dispatched authorization remains executable; double use of one authorization before an anchored reservation; cross-network replay of an authorization whose network identity is not in its preimage; payload, asset or authority confusion; obligations escaping signed liability bounds.

High: a foreign leg succeeding while settlement or import cannot proceed; sponsor capacity disappearing or metering racing past reservations; a per-item venue failure inside a successful outer transaction being read as success; revocation applied retroactively to dispatched obligations; a signing backend authorizing bytes no party decoded.

Medium: quote spam, repeated proving, retry amplification and free cancellation exhausting capacity.

---

# Part III — The adapter contract

This part is the contract between the kernel and a chain integrator. Every chain-specific fact in the design lives here and nowhere above it. An application never reads this part, and an account holder is never shown its contents.

---

## 16. Adapter obligations

Authorization requires both Moriarty permission and native authority. The `CallPermission` answers *what may this intent cause*; the `ForeignAuthorization` answers *which native account may authorize that payload, under which current permission*. Both must hold. Holding a native key never enlarges the signed relation.

Moriarty's own call permission is coarse: it names a callee and a selector, and restricts neither argument values nor amounts. The economic restriction therefore comes from three other places at once — the signed gross-debit caps and permitted recipients for anything that touches Midnight value, the signed per-call cap that L2 adds, and the adapter's payload predicate for everything that happens only on the foreign side. A route whose foreign economic effect appears in none of the three is unbounded and is refused.

### Derivation

For chain-derived keys the SDK computes an injective path:

```text
"moriarty/1/"
  + hex(H(canonical(ChainRef)))
  + "/"
  + hex(principalId)
  + "/"
  + decimal(accountIndex)
  + "/"
  + decimal(keyEpoch)
```

A developer selects `accountIndex` and nothing else. Arbitrary derivation paths are not accepted, and the signing service verifies the same policy independently rather than trusting the caller's path.

Three properties are required of any derivation scheme the SDK drives. The scheme binds the requesting account, so that no principal can obtain another principal's derived key. The path is canonical and its component alphabets exclude the scheme's own separators, because signing services accept free-form path strings without validating or canonicalizing them, and a path that can be spelled two ways is two keys or, worse, one key for two principals. And `keyEpoch` is Moriarty's rotation index only; where a venue's request carries its own scheme or domain selector, that selector is a separate field of `ForeignAuthorization` and is never conflated with the rotation index.

Derivation separates control, not visibility. Derived addresses are computable by anyone who knows the account and the path, so the interface makes no privacy claim for them.

### Network binding

A free-standing chain label is not a network binding. A `NetworkBinding` holds only when both halves are present:

1. The signature preimage contains a value that differs between the networks the key can reach, decodable from what is signed rather than supplied alongside it.
2. A named verifier compares that value against its own configuration and refuses a mismatch.

The second half is the one that is usually missing. A chain identifier that a network's own transaction validation never consults binds nothing at that network, however prominently the payload carries it. Each admitted form therefore names its verifier, and each adapter manifest publishes the form, the verifier, the quantified strength and the validity window.

| Form | What is signed | Who refuses a mismatch | What must be published |
|---|---|---|---|
| `SignedNetworkDiscriminant` | A field whose value is fixed per network and covered by the signature | The venue's own acceptance path | The exact field and values, and whether the refusal is performed by a verifiable rule or by a trusted operator |
| `SignedVerifierIdentity` | The identity of the verifying instance or contract | That instance, comparing the identity against itself | That the identity cannot exist on another network the key reaches |
| `SignedChainState` | A reference to state that exists only on the intended network | The network's own validation of that reference | Bits of network-specific content the reference actually commits to, the block window, and the expiration ceiling |

Three narrowings follow, and each rejects a route that a looser reading would admit.

`SignedVerifierIdentity` holds only where the named identity is unavailable on the other network. Where account identifiers derive from a public key with no network salt, the same identifier exists on every network of that family, and naming it binds nothing. The form is admitted for identities under a network-exclusive naming authority and refused for key-derived or deterministic identifiers.

`SignedChainState` is a statistical binding, not an equality. Its published strength is the network-specific content the reference commits to, not the width of the field carrying it, and its window is the shorter of the reference window and the expiration ceiling. Value released against it is bounded by the capability class in section 18 rather than treated as if the network identity were certain.

`SignedNetworkDiscriminant` distinguishes a verifiable refusal from a promise. Where the only party that checks the discriminant is the venue's off-ledger operator or the signing wallet, and no inspected artifact performs that check, the form is admitted for observation and for routes whose failure is bounded by custody; the adapter manifest records that the enforcement is trusted rather than demonstrated, and where value moves, a custody contract performs the comparison at the point of movement. An adapter never relies on a venue library's own recovery helper: helpers that take the network as an input, or overwrite the network field before recovering, cannot detect a cross-network payload at all. The adapter performs the comparison itself, on the decoded preimage, and reports a mismatch as a network-binding failure rather than as a signer mismatch.

A payload whose network identity cannot be decoded from what is signed returns `E_NETWORK_BINDING_MISSING`, and no signature is requested for it. Three things resemble a binding and are not one. An absolute height, sequence number or nonce constrains when a payload is valid, not which network it is valid on, and where one network's height leads another's it does not even constrain that. A prefix or tag that stops signed data being parsed as a transaction separates formats, not networks. And a chain identifier carried in a typed-data domain that only a wallet is asked to check is unenforced wherever that wallet is not the SDK.

Where a venue offers two signing domains for the same action family, they are distinct bindings and are never interchanged. An adapter pins one domain per primitive and refuses a payload carrying the other.

### Native permission is venue-specific

Native permission systems are coarser than Moriarty's. A weighted threshold over a bitmap of transaction types restricts which kinds of transaction a key set may authorize; it restricts no callee, selector, argument value or debit amount. For contract calls the finest available granularity is a single bit covering the entire class, so any key set meeting the threshold may call any contract with any arguments and any attached value. Where the native system cannot express the required restriction, the adapter supplies its own checks and, where value is at stake, a restricted custody contract that performs them at the point value moves.

A signed per-transaction execution-fee ceiling is not a cost cap either. It bounds what the venue may charge for executing the call, in the venue's own resource accounting, and says nothing about the value the call moves. Gross-debit and liability caps are the only limits on that.

Delegated-call validation that checks action count, deposit, receiver and method does not decrement an allowance. Sponsorship therefore imposes its own signed budget and never relies on a native allowance to bound spending.

Where a venue's signed action carries no expiry, a signature remains usable until its replay window closes. Adapters publish that window, the interface counts it as live exposure until it closes, and no cancellation is reported as having removed it.

---

## 17. Evidence and import acceptance

Import acceptance is split, because the two halves have different strengths and conflating them is how an off-ledger fact gets presented as anchored.

The **transition relation** enforces the value-level conditions: the identities, the quantities, the predicates, the cumulative accounting and the uniqueness of each discharge. These hold by construction, and a violation rejects the transition.

The **deployment's authentication policy** establishes that the evidence is what it claims to be: the quorum's keys and threshold, the signature over the canonical import statement, and the freshness of the observation. This is an external check against a provider bound when the instance was created. It fails closed, but it is trust, not proof, and every surface that reports the resulting fact reports it as imported.

Two consequences are normative. The provider set and the authentication policy are fixed for the life of the instance, so rotating an attestor quorum means a new instance, and a live session cannot migrate to one while composition across instances is unadmitted. And the clock that decides freshness is itself an authenticated observation, not a host clock read.

`ImportFrom` is admitted only when all of the following hold.

1. The intent names this evidence class for this leg, and the policy hash matches the intent exactly and is one the deployment permits.
2. The deployed key set and threshold match the policy, signer indices are distinct, and the threshold signature verifies over the canonical import statement.
3. Source chain, payload hash, pending id, intent id, leg id and attempt all match the pending record.
4. The extractor version, the source schema version and the adapter version are the pinned ones.
5. Finality and freshness satisfy the named policy, evaluated against the venue's finality parameters **as observed**, not against a constant. Where those parameters are governed and can change, the policy binds the observed values and a change invalidates the pinned policy rather than silently widening the window. Where finality has more than one clock, every clock clears.
6. The extracted account, recipients, amounts, fees and liabilities satisfy the leg predicate.
7. The fact's outcome tag comes from the venue's own success discriminant. A successful outer transaction is recorded as a successful outer transaction and nothing more. Where a venue reports failure by emitting an event and returning without reverting, the extractor reads the event; where a batch entry can fail while the enclosing call succeeds, the extractor resolves the individual entry, and a batch whose per-entry outcome cannot be resolved produces unavailable evidence.
8. A status the pinned extractor does not recognize yields `Indeterminate`. An open-ended status vocabulary is never collapsed to failure, because treating an unknown status as non-execution is how recovery releases funds against a leg that executed.
9. Cumulative executed quantity for the leg, per asset, including this import and every earlier accepted one, does not exceed the authorized quantity. Quantities are taken from per-fill records identified individually; an aggregated view whose aggregation rule is a query parameter is not an accounting source.
10. Partial execution is recorded as partial execution, with its executed quantity, and never as success.
11. Neither nullifier below has been consumed.
12. The pending state still admits this import, or admits its signed late-evidence recovery path, and the remaining transition allowance still covers settlement and recovery after this import is applied.

Two nullifiers enforce uniqueness:

```text
source event:  H(sourceChain, transactionId, eventOrReceiptId, extractionPolicyHash)
leg progress:  H(instance, intentId, legId, attempt)
```

The first prevents one foreign event from discharging more than one obligation, and it is the guard that prevents double value. The second prevents duplicate progress **within one attempt**, and nothing more: a fresh attempt yields a fresh leg-progress nullifier. An attempt increment is therefore authorized, not incidental. It requires an accepted import carrying `NotExecuted` for the current attempt, or the venue's own expiry of the current attempt's payload proven under the same rules as any other imported fact, together with the continuation authority for the new attempt. A deadline alone never increments an attempt, and no operator increments one.

---

## 18. Routing and the capability registry

These are inputs to the router, not product tiers, and no account holder or application ever sees them. The router offers an outcome when a path exists that meets the intent's caps, deadline and minimum grade, and declines when none does. What reaches Part I is a grade or a refusal.

A chain becomes routable by satisfying the obligations in section 16; it does not become routable by being popular. Where a chain supports observation but not value movement, outcomes that only read from it are routable and outcomes that move value through it are not, and neither fact is phrased as a limitation of the chain.

Routability is classified, and the class is a property of what a chain can evidence, not a release schedule. The registry below records each class and what a chain must satisfy to enter it. No path that moves value is currently routable: the classes exist so that the router can decline precisely rather than wholesale, and so that a chain that becomes evidenceable becomes routable without any change above this part.

**Class A — settlement only.** Outcomes served entirely on the settling ledger: pricing, fee collection, sponsorship, capacity reservation and settlement. No foreign leg, no import, no foreign signing, and no job whose output is a foreign observation. Requires no unadmitted language change beyond the fee capability.

**Class B — observation.** Read-only imports of position, balance and status facts. Requires L4, L3 and a pinned observation policy with a published threat model. The class boundary is a rule about value, not about intent: in Class B no ledger effect's existence, amount, beneficiary or timing is a function of any extracted field. An imported fact is recorded and nothing else. It does not satisfy a leg predicate, advance a pending record toward settlement, validate a quote, trigger recovery, or release collateral. Charges for observation are priced on job identity and delivered bytes only, which keeps an observation fee independent of what is observed. A route in which any charge, release or deadline depends on extracted content is a Class C route regardless of how small the amount is.

**Class C — value movement.** Foreign signing and imports that release value. Requires L4, L2, L3, L5, L1 and L7, a binding class admitted for value, and every gate below.

### Binding classes

A free-standing chain label is not a network binding. Three classes are admitted, and the class determines the highest class a route can reach.

| Class | What the preimage carries | Enforced by | Admitted for |
|---|---|---|---|
| `SignedVerifierIdentity` | A typed domain separator carrying a network-specific chain identifier, checked by the verifying contract executing on the target network | The target chain's own execution | Value-releasing legs |
| `SignedChainState` | An identifier of a block that the adapter authenticates to the selected network | The target chain's block store and ancestry rules | Value-releasing legs, while no other network in the family shares the referenced block |
| `SignedNetworkDiscriminant` | A network or environment discriminant inside the signed structure, with no chain identifier bound to any ledger | The venue's own software, off-ledger and uncommitted | Observation; value only behind a pinned custody contract |

A payload carrying none of the three returns `E_NETWORK_BINDING_MISSING`. A payload whose class is not admitted for its leg's value class returns `E_NETWORK_BINDING_UNSOUND`.

A custody contract is required, not optional, wherever a venue's only check on network identity executes inside contract code, and wherever the class is `SignedNetworkDiscriminant` and value leaves the venue. On TRON and on NEAR a value-releasing leg without a pinned custody contract returns `E_CUSTODY_CONTRACT_REQUIRED`.

### Adapter admission status

| Adapter | Binding mechanism | Strength | Validity window | Fails when | Status |
|---|---|---|---|---|---|
| Midnight (anchored) | Head read-then-write under ledger induction | Ledger validity | The admitting step's head | Not applicable | Class A, pending Preview acceptance on the exact candidate |
| Hyperliquid L1 exchange action | `SignedNetworkDiscriminant`: a mainnet or testnet discriminant inside the signed action structure. The typed domain's chain identifier is the constant 1337 and binds nothing | One bit of environment separation, enforced by venue software and committed to no ledger | The action's own expiry field where set, otherwise the venue's nonce acceptance window | The venue's environments share a signature validator, or label semantics change without a version a client can pin | Class B. Class C only for venue-internal effects whose outcome is imported per item |
| Hyperliquid user-signed action and bridge withdrawal | `SignedNetworkDiscriminant`: an environment field inside the typed payload; the signature chain identifier is a constant reused across environments. Value leaving the venue crosses a bridge contract requiring two thirds of validator power and a dispute period measured in both seconds and blocks | One bit at the venue; a validator quorum and a dispute period at the bridge | Venue-side, the same window as above; bridge-side, the dispute period | The validator quorum is dishonest, or the importer reads the request rather than the finalization | Class C once its requirements are met, conditional on importing the finalization event for the exact withdrawal |
| TRON contract call, bare | `SignedChainState`: a two-byte block-number key and an eight-byte slice of the reference block identifier inside the signed raw data. The raw data carries no chain identifier | 64 bits of block-hash content | 65,536 blocks, bounded by a 24-hour expiration ceiling | Two TRON networks share the referenced block, which a fork or a network bootstrapped from another's history produces. The native permission system cannot narrow the authority: it expresses a weighted key threshold and a 256-bit bitmap indexed by transaction type, and restricts no callee, selector, argument or debit | Class B. Refused for value release |
| TRON contract call via custody contract | `SignedVerifierIdentity`: a typed domain separator whose chain identifier is the trailing four bytes of the genesis block identifier, read and checked inside the virtual machine by the verifying contract | 32 bits of network identity, enforced by the target chain's own execution | Set by the custody contract, bounded by the transaction expiration ceiling | The custody contract omits the domain check, or the deployment is not pinned in the adapter manifest | Class C once its requirements are met, conditional on a pinned custody contract and its conformance evidence |
| NEAR function call, outer transaction | `SignedChainState`: a block hash in the signed transaction, checked against the local block store with an ancestry test to the selected chain | 256 bits of block-hash content plus ancestry to the selected chain | The network's configured transaction validity period, in blocks | Two NEAR networks share the referenced block. Access-key permission restricts receiver and method names only, and restricts no argument or deposit | Class B. Class C conditional on a pinned custody contract bounding arguments and deposits |
| NEAR delegate action | None. The delegate record carries sender, receiver, actions, nonce, maximum block height and public key, and no network field; its preimage carries a message-type discriminant, not a chain tag. The outer transaction's block hash is never re-checked on the delegate application path | None. Maximum block height is compared to local height only and protects asymmetrically: a testnet-signed action is not expired on mainnet. The sole remaining obstacle is a nonce upper bound of local height multiplied by 1,000,000, an artifact of two networks' relative heights and of when the signing key's nonce was initialized | Not applicable | Always, for the purpose of network binding | Refused outright for any value-releasing leg |
| NEAR offchain authorization envelope | A signed envelope binding chain identifier, signer, resolution path, signing timestamp and payload, whose resolver panics on a chain-identifier mismatch. Resolution is a view function that its standard forbids on-chain callers from using and that gates no state change | Full chain-identifier separation off-ledger; none on-ledger | Open unless the resolver enforces a time to live | Used as authority for a state-changing call, which its standard prohibits | Admitted as authority for off-ledger requests only. Refused as authority for value movement |

Until a gate closes, the affected call returns `Unavailable` and names the exact missing adapter, policy or benchmark digest.

### Routes refused outright

These are refusals, not gates. No policy, quorum size, deadline or collateral level admits them, because the defect is in the authorization rather than in the evidence, and evidence cannot repair an authorization that never named its network.

- A NEAR delegate action as authority for any leg that releases value. Its record carries no network identity, and the protection that currently stops cross-network replay is a moving property of two networks' heights that nothing commits to.
- Any payload whose network identity lives in a metadata field outside the signature preimage, or in a claimed source chain supplied by the caller.
- Any value-releasing import whose network identity rests on an endpoint's routing configuration rather than an authenticated network witness.
- Reuse of one derived key across two networks of the same family. A TRON address and a NEAR implicit account identifier derive from the public key with no network salt, so the same key controls the same account on every network of its family. The signing service pins each derived key to the first network it signs for and refuses any later request naming a different one.
- Any irreversible value-releasing leg on a venue that enforces no expiry on the authorization it accepts, unless prefunded compensation or a venue-independent bounded refund path exists.
- Any route whose venue response schema cannot separate per-item failure from success, and any fee whose component cannot be distinguished from an aggregate.
- An opaque-digest-only signing backend as the sole authority for a value-releasing leg. Where a backend accepts only a fixed-size digest, no party at the signer verifies what was authorized, and the decoded-payload check exists only in the requesting service. Such a route is admitted only with the accepted risk recorded below and a custody contract bounding the worst outcome.

### Class C requirements

- [ ] Required language changes admitted for the advertised feature set, including the pending-bound continuation authority. Correctness does not depend on predecessor fan-in above one.
- [ ] Complete lifecycle semantics for reservation, signature issuance, dispatch, unknown execution, import, compensation, refund and termination.
- [ ] Crash boundaries tested by enumeration, naming at minimum the instants immediately before and after nonce reservation, signature issuance, broadcast, acknowledgement, import and payout.
- [ ] Nonce reservation, fund reservation and head update atomic; duplicate solvers and duplicate submissions cannot duplicate authorization.
- [ ] Foreign receipt consumption prevents reuse across intents, legs, instances and recovery branches.
- [ ] Gross debit, net credit and nominal liability caps cover the whole lifecycle including retries, fees and compensation.
- [ ] No normal or recovery action introduces a fifth effect or presents a foreign fact as anchored.
- [ ] Each adapter has retained deployed-behaviour evidence for encoding, network binding, permissions, fee limits, finality and per-item financial success.
- [ ] Each adapter's binding class recorded with its measured strength, its window, and the condition under which it fails, and the class admitted for the leg's value class.
- [ ] Expiry-before-recovery demonstrated per venue: the dispatched authorization's venue-enforced expiry is earlier than `recoverAfter`, shown against deployed behaviour rather than documentation.
- [ ] Key-to-network pinning enforced at the signing service, with a replay test proving a second network's request is refused for a key already pinned.
- [ ] Every value-releasing import carries an authenticated network witness, and an endpoint's routing claim is rejected as that witness.
- [ ] Revocation, legacy-format rejection and nonce cleanup have adversarial replay tests, with no bypass for records that fail to decode as the versioned format.
- [ ] Quorum manifest names each operator, its hosting provider and each data source it reads; admission stops when two members declare the same data source.
- [ ] Threshold parameters have an explicit threat model naming what they do not protect against, including a common faulty data source and an agreeing dishonest quorum.
- [ ] Meter profiles publish byte bounds, work limits, attempt limits, tariffs, rounding, quote lifetime and recovery budget. Missing benchmarked values block admission.
- [ ] Sponsor grants cannot be concurrently overspent; sponsor failure never silently bills the user.
- [ ] Compensation funded before irreversible foreign authority is issued, with explicit asset, amount, beneficiary and late-execution treatment.
- [ ] Launch exposure numerically capped per intent, instance, asset and quorum, with automatic admission stops at those limits.
- [ ] Verification-enabled Midnight Preview financial acceptance demonstrated on the exact candidate, with independent review.

### Accepted risks

These are not gates. Each names a residual risk the interface cannot make falsifiable, and the role that carries it. A deployment that will not accept one does not launch the routes that depend on it.

| Risk | Why no gate closes it | Owner |
|---|---|---|
| A quorum agrees on one false fact without equivocating | Agreement is indistinguishable from correctness to any checker that has only the quorum's signatures | Deployment operator who signs the quorum manifest |
| A venue changes the meaning of a signed environment label | The label is enforced by venue software, is committed to no ledger, and carries no version a client can pin | Deployment operator who admits the venue adapter |
| A signing backend that accepts only a fixed-size digest authorizes bytes it never decoded | The backend verifies nothing about the payload, so the decoded-payload check cannot be verified at the point of signing | Signing-service operator, under the authority bond |
| Adjudication of a false import is itself a trusted import | The verdict arrives as evidence of the same class as the claim it judges | Adjudication quorum operator |

---

# Part IV — Record

---

## 19. Decisions

Each row records a decision, the alternative it displaced, and why that alternative lost. A decision reversed once is not reopened without new cited evidence.

| Decision | Rejected alternative | Why it lost |
|---|---|---|
| A continuation authority binds the pending record identity, leg and attempt. Until it is admitted, each import needs a fresh principal signature | Continuations bind the pending id instead of the head, with no language change | Every accepted transition rejects unless the signed predecessor is exactly the current before-state hash, and that head is unknown when the principal signs. The unattended import path did not exist in the profile |
| Predecessor fan-in above one is not a dependency | Carrying fan-in as a required language change | Each import advances the single pending record and the settling transition reads that record. No join over several predecessors is needed, so correctness holds at fan-in one |
| The admitting step reserves a transition budget, and every deadline falls inside the instance horizon | Leaving instance lifetime unmentioned | The allowance is frozen at genesis and one unit is consumed per accepted action, so legs, retries, settlement and recovery can exhaust it and strand a funded pending record |
| A network binding requires both a per-network value inside the signed preimage and a named verifier that refuses a mismatch | Decodability from the preimage as the whole test | A chain identifier that a network's own validation never consults binds nothing there. Decodability without a checker is not a binding |
| Three binding forms, separated by who refuses a mismatch: `SignedNetworkDiscriminant`, `SignedVerifierIdentity`, `SignedChainState` | Two forms separated by codec | The two-form split conflated a typed-domain codec with a per-network value, and so admitted a venue whose signed domain chain identifier is constant across its networks |
| `SignedVerifierIdentity` is refused for key-derived, implicit or deterministic identifiers | Accepting a signed verifying-contract identity generally | Key-derived identifiers carry no network salt, so the same identity exists on every network of the family and naming it binds nothing |
| Hyperliquid is recorded as a discriminant enforced by venue software: Class B, and Class C only for per-item venue effects or an imported bridge finalization | Hyperliquid as the sole Class C once its requirements are met on the strength of a decodable signed domain | The signed domain chain identifier is a constant across environments, the only per-network value is a message field, and no inspected artifact performs the check |
| TRON bare contract calls reach Class B on the reference-block binding, and Class C only behind a pinned custody contract | Leaving TRON blocked for want of a chain identifier in the raw transaction | Reference-block validation authenticates the block to the network at a stated strength and window, and the typed-data domain supplies a real network identity inside the virtual machine. The blocking condition was answered |
| A NEAR delegate action is refused outright for any value-releasing leg | Gating it behind a domain-checking receiving contract | The record carries no network identity and its application path never re-checks the outer block hash. A receiving contract cannot repair the signature it receives, so there is no gate to close |
| The chain-bound offchain envelope is authority for off-ledger requests only | Treating it as the answer for NEAR value movement | Its own standard makes resolution a view convention and forbids gating state changes on it |
| A custody contract is required, not optional, for value release on TRON and NEAR | Custody as adapter discretion | On both, the only network-identity check that executes on the target chain lives in contract code, and native permission restricts no callee, selector, argument or debit |
| Expiry-before-recovery: recovery runs only once the dispatched authorization has expired by venue rule, enforced at signing time | No timeout resolves the safe-versus-stranding dilemma, leaving compensation as the only answer | The signature's own expiry resolves it where a ledger deadline cannot, converting an unresolvable tension into a per-venue falsifiable gate. Compensation remains for venues without enforceable expiry |
| Each derived key is pinned to the first network it signs for | Distinct derivation paths per chain as sufficient | Addresses and implicit identifiers collide across networks of one family, so a per-chain path does not stop one key signing for two networks |
| Import acceptance separates relation-enforced value conditions from externally authenticated evidence conditions | The relation enforces the whole checklist | Observation authenticity is a trusted external check against a provider bound at genesis, and the evidence digest is opaque to the profile. Claiming the relation verifies it presents an off-ledger fact as anchored |
| An unrecognized venue status yields an indeterminate outcome | Collapsing an unknown status to failure | Venue status vocabularies are open-world. Treating unknown as non-execution is how recovery releases funds against a leg that executed |
| Cumulative executed quantity is checked per leg and asset across attempts | A per-import predicate only | Two partial fills each inside the cap can together exceed the authorization |
| A refusal is a pre-dispatch result only; after dispatch the outputs are lifecycle states | Typed refusals available at any point | A refusal after dispatch is indistinguishable from a partial attempt, which the contract forbids. As a structural rule, any post-dispatch refusal path is a testable defect |
| Primitive registration leaves the call surface; admitting a primitive is a governed deployment change | A runtime registration call | Registering a foreign primitive at runtime is an unrestricted new call shape, which section 13 forbids |
| Five planes, with an ordered test assigning every charge to exactly one | The plane table with no assignment rule | Without a rule a venue-collected ordering payment books to both foreign execution and priority. The prior text asserted separation instead of deciding it |
| Commitments and surplus are accounted as vectors that are not planes | Booking locked capital as a cost and leaving positive deviation unassigned | Locked capital is refundable on a condition, so booking it as cost overstates cost and loses the release condition. Unassigned surplus is retained by whichever operator touches it last |
| Budget exhaustion stops the admission of new work, and a per-leg completion reserve is held before dispatch | A uniform hard stop, or exempting dispatched legs from the budget | A uniform stop strands a dispatched leg with no import and no recovery; exempting dispatched work makes the budget unbounded. Reserving completion before dispatch satisfies both |
| Recurring capacity is purchased, not staked | Refundable stake funding recurring capacity | Stake has no revenue source for operating costs, and a capacity model allotting a share of total staked weight yields no deterministic amount from a fixed stake |
| The application fee cap is the minimum of capability rate, profile cap for the market class, and the venue's enforced maximum | Stating fixed spot and derivative percentages as Moriarty's caps | Those figures are one venue's builder-fee ceiling, not a protocol parameter, and stating them omits venues with different or no ceiling |
| Every parameter no source or measurement fixes is named in section 14 and absent from the text | Publishing arbitrary figures as proposed launch values | A number in normative text is implemented as written. Tariffs, shares, quorum size and bond constants had neither source nor measurement |
| A bond is posted only where one deviation can exceed the expected revenue from continued selection; otherwise deselection enforces | Uniform bonding of every role, or uniform reliance on reputation | Gives a stated test for which roles bond, instead of bonding authorship and idleness alongside custody |
| The watcher role and the primitive author as an operator role are removed | Keeping both in the operator table | An availability fee pays an idle watcher in full, so its most profitable deviation is to do nothing. Authorship has no liveness obligation, custody or runtime work to price; the royalty remains as revenue routing |
| Penalties compensate verified losses first, and are not burned | Burning penalties, as inspected venues burn fees | A burn pays no victim |
| Class B is bounded by a rule about value: no ledger effect's existence, amount, beneficiary or timing is a function of any extracted field | No release of Midnight value conditioned on imported facts | The original is not falsifiable and the tier leaked, because metered observation fees, quote validation and recovery triggering all consume imported facts |
| Quorum independence becomes a manifest check, with the unfalsifiable residue recorded as an accepted risk | Independence as a gate | Independence is not falsifiable by the SDK; a declared shared data source is |

---

## 20. Open parameters

Each row names something this document does not fix, what would settle it, and the role that owns it. A parameter belongs here rather than in the text whenever no source fixes it and no measurement in hand fixes it. Rows are appended; none is resolved by restating it elsewhere.

| # | Parameter | What is unknown | What settles it | Owner |
|---|---|---|---|---|
| U01 | Hyperliquid nonce acceptance window | The range of nonces and timestamps the venue accepts for an L1 action, and the exact effect of the expiry field on already-signed actions | Read-only measurement against the deployed API, or a venue specification a client can pin | Adapter author |
| U02 | Bridge dispute period and block-duration parameters | Both are constructor arguments of the deployed bridge, not constants in its source | Read the deployed contract's public values at the pinned deployment | Adapter author |
| U03 | TRON deployed chain parameters | The maximum signature count and the deployed custody contract's domain values | Read the deployed chain parameters and the deployed contract | Adapter author |
| U04 | NEAR transaction validity period per network | The configured value on each target network, which bounds the recent-block binding's window | Read each network's genesis configuration | Adapter author |
| U05 | Shared block history across networks in one family | Whether any future network in a target family is bootstrapped from another's blocks, which voids a recent-block binding entirely | A pinned network registry recording each network's genesis identifier and its provenance, checked at adapter admission | Deployment operator |
| U06 | Maximum admissible signature validity window per venue | The window that makes the expiry-before-recovery rule satisfiable, which depends on the recovery reserve | Fix the recovery reserve, then derive the window per venue from measured finality | Deployment operator |
| U07 | Quorum size, threshold and per-operator bond | The values at which attestor equivocation and agreeing dishonesty are unprofitable | An exposure model priced against measured venue volumes, reviewed independently | Deployment operator |
| U08 | Benchmarked meter values | Work credits, tariffs and byte bounds are launch proposals, not measurements | A retained benchmark on the admitted candidate, named by digest | Coordinator |
| U09 | NEAR receiving contract for value release | Whether an audited contract exists that performs the chain-identifier check inside a state-changing call, or whether one must be written and admitted | An artifact review against deployed behaviour | Adapter author |
| U10 | Enforceable loss adjudication | Whether a verdict on a false import is enforceable against collateral outside the quorum that produced the claim | A collateral and adjudication design whose enforcement does not depend on the attesting set | Deployment operator |
| U11 | Per-item financial success schemas | For each venue, the exact response fields that distinguish per-item failure inside a successful outer transaction, and their stability across venue versions | Retained deployed-behaviour evidence per adapter version | Adapter author |
| U12 | Session transition allowance | The worst-case transition count for a multi-leg route, and therefore the allowance a session must reserve. Maximum attempts per leg and maximum recovery transitions are profile parameters | Measure a complete multi-leg lifecycle including retries, recovery and settlement on the admitted candidate | Coordinator |
| U13 | Anchored import latency | The latency of one import, and therefore the largest number of serial imports that fits inside a signed deadline and inside the instance horizon | Measure import admission on the admitted candidate | Coordinator |
| U14 | Continuation authority encoding | Whether a pending-bound continuation authority, and the pending and import relations consuming it, fit the profile's joint encoding and resource bounds | Encode the candidate relations and check them against admitted bounds | Coordinator |
| U15 | Coordinator program caps | The numeric caps on legs, imported facts and observation fields for one coordinator program | Derive from the joint-bounds encoding check | Coordinator |
| U16 | Venue enforcement of the network discriminant | Whether the venue operator in fact refuses a mismatched discriminant. No verification artifact exists in any pinned repository and the node distribution ships no source | Read-only cross-environment probe against the deployed venue, retained as evidence | Adapter author |
| U17 | Venue duplicate-rejection window | Each venue's duplicate-rejection field and the window over which re-broadcasting identical certified bytes is rejected. Without it an adapter has no dispatch path | Read-only measurement against the deployed venue | Adapter author |
| U18 | Value permitted against a chain-state binding | The quantity of network-specific content a `SignedChainState` reference must commit before value is released against it. The inspected reference commits 64 bits inside a 65,536-block window under a 24-hour ceiling, and no threshold is fixed | A policy decision bounded by measured strength and window per adapter | Deployment operator |
| U19 | Positive evidence of non-execution | Whether any target venue can produce positive evidence that an action did not execute. Where it cannot, an attempt can never be incremented and a leg in unknown execution has only the signed recovery path | Per-venue artifact review against deployed behaviour | Adapter author |
| U20 | Settlement asset and atomic unit | Which asset denominates orchestration prices, and its decimal precision | The deployment's choice of settlement asset | Deployment operator |
| U21 | Capacity window and peak rate | The window over which a purchased reservation is deterministic, and the peak rate a seller commits to serve within it | Measure the coordinator's sustained and peak throughput | Coordinator |
| U22 | Quote lifetime | How long a reserved quote allocation is held | Measure the time from quote issue to admission across the candidate route | Coordinator |
| U23 | Recovery budget replenishment window | The period over which a recovery allowance refreshes. It must derive from Moriarty's own accounting, never from a foreign network's daily quota | A policy decision expressed in the meter's own units | Deployment operator |
| U24 | Application fee cap per market class | The profile's own ceiling, distinct from each venue's enforced maximum, which is an adapter fact | A policy decision plus the pinned per-venue maxima | Deployment operator |
| U25 | Active capability bound | How many fee capabilities one principal holds at once | A policy decision; the nearest deployed comparator allows ten | Deployment operator |
| U26 | Protocol revenue share and split | The share of collected orchestration revenue routed to the protocol, and how it divides | A funding decision. Comparable venues route half of the comparable revenue | Deployment operator |
| U27 | Primitive author royalty rate | The share of a primitive's base compute revenue routed to its author | A policy decision | Deployment operator |
| U28 | Attestor exposure premium | The premium that makes observation commercially viable at the admitted exposure cap | Binding quotes solicited from candidate operators | Deployment operator |
| U29 | Aggregate unresolved exposure cap per pool | The maximum tolerable loss per pool. The bond's relation to pool exposure is specified and does not need the figure | A risk decision | Deployment operator |
| U30 | Bond exit notice and post-exposure lock | The notice period and the lock after the last covered exposure closes | A policy decision bounded below by the longest dispute resolution observed in the pilot | Deployment operator |
| U31 | Non-refundable portion of a foreign reservation | The refund penalty and pessimistic pricing ratio for each admitted network. Both are live network parameters that can change, and one is presently zero on the inspected configuration | Read each network's live protocol configuration at quote time | Adapter author |
| U32 | Venue request allowance headroom | The allowance a route consumes per intent, and the settled volume required to sustain it | Measure the request count of one complete lifecycle, including cancellation and observation, against the venue's published accrual rule | Adapter author |
| U33 | Restoration delay and veto window | The delay before a nominated party's restoration of authorization takes effect, and the window in which a standing means of authorization may refuse it | A policy decision bounded below by the time a holder needs to notice a notification and act | Deployment operator |
| U34 | Period anchor and carry-over defaults | The default beginning of a spending period and whether unused budget carries, where a policy does not state them. The specification refuses such a policy rather than guessing; whether a default should exist at all is undecided | A policy decision | Deployment operator |
| U35 | Reference band width per asset class | The deviation band within which quote movement is absorbed rather than voiding the quote, and who absorbs it | Measure realized reference volatility per asset class against quote lifetimes | Coordinator |
| U36 | Delegation ceiling above which the identity must authorize | The movement size above which a delegated capability is insufficient authority | A risk decision per deployment | Deployment operator |
| U37 | Mandate execution and observation limits | The maximum executions one mandate may authorize, and the staleness bound per evidence class for a condition | Derive from the session transition budget and measured observation latency | Coordinator |
| U38 | Dust threshold per asset and destination | The value below which delivering a receipt costs more than the receipt is worth | Measure delivery cost per asset and destination at quote time | Adapter author |
| U39 | Acceleration allowance bounds | The maximum a holder may pre-authorize for accelerating a stalled dispatched step | A policy decision bounded by the recovery reserve | Deployment operator |
| U40 | Contingency windows per receipt class | The period after which a contingent receipt becomes final, per evidence class and venue | Measure reorganization and dispute-window behaviour per adapter | Adapter author |
| U41 | Compartment limit per identity | The number of compartments one identity may hold, bounded by the profile's record widths | Derive from the joint-bounds encoding check | Coordinator |
| U42 | Claim expiry notice lead time | How far in advance the kernel must report a closing claim deadline | A policy decision bounded below by observed settlement latency | Deployment operator |

Four unknowns recorded above subsume separately raised items: U07 covers quorum size, threshold and per-operator bond together with the independence audit; U08 covers every meter tariff, work credit and byte bound; U02 covers the bridge's mutable finality parameters; and U11 covers the venue failure-code vocabulary, which is pinned per adapter version rather than standardized.

The residue is empirical rather than a missing interface field. No inspected source shows that a proposed adapter matches deployed behaviour, that a chosen threshold represents independent operators, or that the pending and import relations fit admitted resource bounds. Those are the Class C requirements in section 18, not open questions about the shape of the interface.

---
