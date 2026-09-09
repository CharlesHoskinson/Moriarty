# DeFi category tabs

The site's category section is a tabbed explainer with one tab per DeFi
category. Every economic family is present and every action target from DA01 to
DA24 appears, so a category with less repository evidence gets a shorter tab
rather than a missing one. DA12 is tagged `F3;P` in the source matrix and is
shown in both the Derivatives and the Prediction Markets tabs, marked as
shared.

| Tab | Label | Action targets | Count |
|---|---|---|---|
| F1 | Exchange and price discovery | DA01, DA02, DA03 | 3 |
| F2 | Credit and collateralized debt | DA04–DA10 | 7 |
| F3 | Derivatives | DA11, DA12 | 2 |
| F4 | Consensus-position claims | DA14, DA15 | 2 |
| F5 | Tokenized off-chain claims | DA16 | 1 |
| F6 | Delegated asset management | DA17, DA18, DA19 | 3 |
| P  | Prediction markets | DA12 (shared), DA13 | 1 own + 1 shared |
| ⊥  | Cross-cutting | DA20–DA24 | 5 |

Every tab renders the same blocks in the same order: what the category is, the
facet profile, the action targets, what goes wrong, what Moriarty does about
it, and the reference sources.

---

## F1 — Exchange and price discovery

**Function.** Convert one asset into another and, in doing so, produce a price.
The family includes constant-function market makers, order books,
concentrated-liquidity positions, aggregators and routers.

No obligation outlives the transaction. A trader brings one asset and leaves
with another, and when the two transfers have settled nobody owes anybody
anything. The claim that does persist belongs to the liquidity provider, whose
share is a proportional claim on whatever the reserves are worth when the share
is burned, not on what they were worth when it was minted.

**Boundary.** The exchange function is separate from the mechanism that
implements it. Uniswap V4's singleton accounting and hooks change the mechanism
and leave the exchange function alone. Routing and aggregation are
execution-facet properties layered on top of F1.

**Facet profile.** Execution is atomic on-chain, or intent and solver mediated,
because a swap that half-completes is not a swap. Settlement is same-domain and
immediate, and custody is with the pool for the length of the swap. Legal
dependence is none, because the trader's claim is discharged inside the
transaction; a custodial dependence in a pooled asset belongs to the asset
under F5. Collateral and solvency reduce to the reserves, because a pool cannot
owe more than it holds. Oracles are none for a constant-function maker, whose
reserves are the price, and one is consumed by a venue that quotes from an
external index; the pool's own price is read elsewhere as an observation, so a
large trade that moves it is an attack. Authorization is per-transaction,
because `min_out` is the whole of what the trader authorized. Price discovery
is what this family produces.

**Action targets.**

- **DA01 — swap exact input / exact output.**
  Requirement: asset-indexed exchange with fee and slippage accounting.
  *Distinguishing test:* rounding; reserve safety; net minimum.

  The repository's example fixes the arithmetic. Reserves are 1,000,000 A and
  2,000,000 B, the fee is 997/1000, and the input is 10,000 A, so the effective
  input is 9,970,000. The numerator is 9,970,000 multiplied by 2,000,000, which
  is 19,940,000,000,000, and the denominator is 1,000,000 multiplied by 1000
  plus 9,970,000, which is 1,009,970,000. The quotient is 19,743 with a
  remainder of 162,290,000 numerator units, and the floor is 19,743 B. After
  the swap the reserves are 1,010,000 A and 1,980,257 B.

  The wrong implementation rounds the other way. A ceiling gives 19,744, and
  the pool pays one quantum it did not earn on every swap whose remainder is
  nonzero, so reserves drift below what the constant-product identity predicts
  and no single transaction looks wrong. Rounding down keeps the remainder in
  the reserve where it backs every provider share, and the policy says so:
  `unpaid output remains in reserve_b`.

  The formula never yields an output equal to the reserve for a finite input,
  but a different fee path, rounding or an unchecked subtraction can, and a
  pool whose B reserve reaches zero has a product of zero and no defined price
  afterwards. The guard `output_calculated < state.reserve_b` forbids the
  emptying trade by name.

  The net minimum is compared after the fee, because that is what the trader
  receives. A wrong implementation compares `min_out` against a gross figure,
  the trader receives less than the minimum they signed, and such fills look
  like ordinary slippage unless the fee accounting is visible, so the
  comparison basis is a declared field of the policy.

- **DA02 — provide / remove liquidity.**
  Requirement: share mint/burn and reserve contributions.
  *Distinguishing test:* proportional entitlement, and the donation and
  zero-supply boundaries where share maths breaks.

  Proportional entitlement is the ordinary case. A pool holding 100 assets
  against 80 shares receives 25 assets and issues 25 multiplied by 80 divided
  by 100, which is 20 shares under a floor policy. The new provider holds 20 of
  100 shares against 125 assets, and nobody's proportion has moved.

  The zero-supply boundary is where the formula has nothing to say, because the
  first deposit has no ratio to preserve when shares and assets are both zero,
  and assets multiplied by zero divided by zero is not a number. A plausible
  implementation special-cases this with a rule that was never written down,
  and that rule decides who can be robbed later by the donation attack, whose
  arithmetic is walked in the F6 tab. The reader would see a second depositor
  whose assets arrive and whose share count is zero.

- **DA03 — open / adjust / close liquidity position.**
  Requirement: bounded position identity and range.
  *Distinguishing test:* fee allocation and finite tick/range traversal.

  A concentrated position is liquidity that is live only while the price is
  inside a declared range. Fees from a trade belong to the positions that were
  in range when the trade crossed them, and a trade that moves the price across
  several ranges has to visit each in order. When the code credits fees to
  whichever positions are in range at the end of the trade, fee income fails to
  match the time a position spent in range.

  A trade that can cross an unbounded number of ranges is a loop with no
  declared bound, and the language has no such loop: the atomic profile has no
  loops or recursion, the surface language's compile-time loops elaborate to a
  finite Core, and a range traversal must declare its maximum before it can be
  written.

**What goes wrong.** A conversion between two quantities produces a remainder,
and somebody keeps it. The code decides who by the direction it rounds, and
when the direction favours the caller the pool leaks one quantum per operation,
the leak compounds with volume, and no rule fires. When the zero-supply case is
left to a special rule, the first depositor controls the ratio every later
depositor is measured against, and that is the lever the donation attack pulls.

**What Moriarty does.** Rounding is a declared artifact, and the `policy` block
names it in full:

```mori
policy swap_output targets write(swap, reserve_b), write(swap, trader_b), effect(swap, 1, amount) {
  unit AssetB_quantum;
  derivation "floor((amount_in*997)*reserve_b/(reserve_a*1000+amount_in*997))";
  rounding floor(swap, output_calculated);
  remainder "unpaid output remains in reserve_b";
  comparison "exact integer output and reserves";
  proof "constant_product_exact_input_floor_v1";
}
```

Reserve safety and the net minimum are named guards with named rejections:

```mori
guard arg.min_out <= output_calculated,      "minimum output not met";
guard output_calculated < state.reserve_b,   "output would empty reserve";
```

On the repository's instance the output is exactly 19,743 B, and requesting
19,744 produces a named slippage failure.

At the type level the reserves are `Amount<AssetA_quantum>` and
`Amount<AssetB_quantum>`. Addition requires identical units, multiplication
adds unit exponents while `floor_div` subtracts them, so an A quantity cannot
be added to a B quantity and the swap quotient is a B quantity because the
arithmetic made it one. `floor_div` is the only division in the language. It
rejects a zero denominator, it returns the mathematical floor, and every
operand and intermediate result must fit UInt128.

At the guard level every guard carries a message, and a false guard rejects the
whole action so that no working write, effect or authority consumption is
exposed. The swap reserves its own closure: `reserve swap for close` and the
guard `remaining > uint(1)` keep the last transition for the provider's
withdrawal.

At the policy level every `set` of an Amount and every Amount field in an
`emit` must be covered by exactly one policy target. A value that passed through
a `floor_div` carries that node as provenance, a policy that says `rounding
floor` must name the one `let` that did the dividing, and a policy that says
`rounding none` requires that no division touched the value. The `proof` string
names a mandatory future claim.

The language does not make the pool's price true, and a large trade that moves
it is still a valid swap. DA01 is implemented as a local fixed exact-input
example only, DA02 needs extension, and DA03 needs a pinned protocol fixture.

**Reference sources.** AMM literature §§2.3.2, 3.1, 3.3 and 4, and Uniswap V4
core for singleton accounting and hooks.

---

## F2 — Credit and collateralized debt

**Function.** Create an obligation to repay, secured by collateral or, in the
flash case, by atomic repayment, and manage it through accrual, partial
performance, default and discharge. Its members are lending markets,
collateralized debt positions, debt-backed stablecoins and flash loans.

The obligation is a promise about the future. A borrower owes a nominal
quantity to a lender, the quantity grows on a schedule, and the borrower can
perform against it in pieces. The lender's claim is worth something only if the
borrower pays or the seized collateral covers the debt.

**Boundary.** *Nominal debt is not a transfer,* and that distinction is why the
language is organized around F2. A transfer moves cash now. A debt is a number
about later. They are accounted for separately because a payment is both at
once: cash moves, and a named obligation is reduced by an amount that need not
equal the cash. An earlier retained composition model reduced a duty from 50 to
30 after the funding transfer had been removed from the action list, and it
left the payer holding 50 and the lender holding nothing. Debt moved without
the cash that was supposed to fund it, and the successor kernel exists to
refuse that.

**Facet profile.** Execution is on-chain and multi-step, because a loan is a
lifecycle and the state between its transactions is the debt; the flash loan is
the one-transaction case. Settlement is same-domain, but discharge and transfer
are separate events with separate identities, and custody is encumbrance rather
than surrender, since the collateral still belongs to the borrower. Legal
dependence is none on-chain, because the contract seizes without a court, and
it becomes material the moment the collateral is an F5 claim. Collateral and
solvency is the defining facet, and oracles are required, because a liquidation
asks whether the pledge is still worth the promise. Authorization is two
mandates, the borrower's to draw and the liquidator's to seize, and neither may
do the other's action. Price discovery is consumed.

**Action targets.**

- **DA04 — supply / redeem lending claims.**
  Requirement: claim shares and liquidity-constrained withdrawal.
  *Distinguishing test:* insufficient pool liquidity must reject without
  erasing the claim. Failure to withdraw is not loss of entitlement.

  A supplier's claim is a share of a pool that has lent out more cash than the
  withdrawal asks for, so the share is good and the cash is not there, and the
  only correct failure leaves the share exactly where it was. The wrong
  implementation processes the withdrawal as far as it can and records the rest
  as done, or burns the shares and transfers what is available, or marks the
  request consumed on the way to a revert that never fires. A supplier's shares
  then fall during a utilization spike, and the lender grows richer by the same
  amount without anyone having borrowed.

- **DA05 — post / release collateral.**
  Requirement: encumbrance and debt-dependent release.
  *Distinguishing test:* release cannot violate the collateral rule.

  To post collateral is to lock an asset the borrower still owns, and a release
  must check the collateral rule as it will stand once the release is done. An
  implementation that checks the ratio before subtracting the released amount
  lets the borrower walk out under-collateralized by exactly the release.
  Another implementation lets one pledge back several duties and releases it
  against the first duty that asks, and another turns a doubled accounting
  ratio into a doubled debt allowance. The liquidation then recovers less than
  the debt.

- **DA06 — borrow / accrue / repay.**
  Requirement: nominal debt distinct from transfers; rate and time arithmetic.
  *Distinguishing test:* partial repayment preserves principal and interest
  allocation.

  Take a debt of principal 100 and accrued interest 10. A payment of 7 under an
  interest-first rule leaves principal 100 and interest 3, and the debt is
  outstanding at 103. Under a principal-first rule the same payment leaves
  principal 93 and interest 10, and under a pro-rata rule the principal share
  is the floor of 7 multiplied by 100 divided by 110, which is 6, so principal
  becomes 94 and interest 9. An implementation that subtracts 7 from whichever
  field the code reaches first, or from a single combined balance, shows
  interest that vanished before it was paid or principal that fell when only
  interest was due.

  Simple interest of 100 at 10 per 100 for
  30 days on a 360-day basis is 5 before the selected final rounding, and the
  accrual applies once over that declared interval. The wrong implementation
  substitutes a block count for elapsed contractual time, applies the same
  interval twice, or crosses a rate kink using the wrong side's utilization.
  The repository's loan fixes one interval exactly: on a notional of
  5,000,000,000 micro-USD at 8 per 100 for 31 days on a 365-day basis, the
  interest is the floor of the product, which is 33,972,602, and the discarded
  54/73 of a micro-USD is written into the policy's `remainder` field rather
  than left to be discovered.

- **DA07 — liquidate / recognize default.**
  Requirement: authorized seizure, loss and residual debt allocation.
  *Distinguishing test:* partial liquidation and close factor; maturity before
  forfeiture.

  An authorized partial liquidation with outstanding debt of 100 and a close
  limit of one half repays 40 and leaves debt of 60, and the collateral taken
  and any bonus are computed separately from the debt reduced. One
  implementation repays 60 despite the limit, one seizes at a stale or wrongly
  denominated price, and one treats the shortfall between debt and recovered
  collateral as if it had disappeared. A fixed-term loan with no early
  liquidation ends with repayment plus interest or with forfeiture of the
  collateral, and a liquidation that fires before term is a seizure without
  authority. Borrowers are then liquidated harder than the rule permits, or
  collateral is taken from a loan not yet due.

- **DA08 — flash borrow / repay atomically.**
  Requirement: atomic multi-leg settlement with fee.
  *Distinguishing test:* every loan repaid within the same atomic transaction.
  Flash borrowing is a capability with legitimate uses, not a vulnerability.

  A flash loan of principal 100 with fee 1 is repaid at 101 inside the one
  admitted atomic operation, and every effect of the borrowing binds to that
  operation. One wrong implementation accepts 100 and forgets the fee, another
  accepts 101 in a later transaction within the same block, the
  block-versus-transaction confusion Kotzer §III-B commits. A third fails the
  final leg after earlier legs have moved tentative balances and does not
  unwind them. The pool is then briefly short and never made whole.

- **DA09 — refinance / novate / capitalize.**
  Requirement: a workflow over old debt, new debt, collateral and signed
  liability authority.
  *Distinguishing test:* reject refinance without old-debt discharge;
  capitalization changes liability.

  A refinance discharges an old obligation and creates a new one in a single
  workflow, and the collateral moves from backing the first to backing the
  second. The wrong implementation creates the new debt and leaves the old one
  outstanding, so the borrower owes twice, or discharges the old and fails to
  create the new, so the lender holds nothing. Rolling accrued interest into
  principal does not change what is owed today and does change what accrues
  tomorrow, so it is a new liability that needs the borrower's signed
  authority, and the code that capitalizes on a missed payment without asking
  changes the debt's growth rate without anyone having agreed.

- **DA10 — issue / burn debt-backed stablecoin.**
  Requirement: issuance authority plus debt and collateral.
  *Distinguishing test:* burn amount and released collateral obey the specified
  debt rule.

  A debt-backed stablecoin is a collateralized debt position whose debt is the
  coin. Issuing creates debt against collateral under an issuance authority,
  burning reduces the debt by the burned amount under the same rule, and the
  collateral released is what the rule permits after the reduction, not a
  proportional slice of what was posted. Collateral released in proportion to
  the burn while accrued fees remain unpaid is the error, and so are coins
  burned without reducing debt and debt reduced without a burn. The peg is an
  economic objective rather than a static equality, and a system that treats it
  as an equality cannot represent a coin worth less than its debt.

**What goes wrong.** The system holds one number where it needed two objects,
and the operation that should have touched one of them touches the number. A
payment is a transfer and an allocation, and when they are one balance the
allocation rule is whatever order the subtractions ran in. A withdrawal is a
request and a claim, and when they are one balance a request that cannot be
funded consumes the claim. A refinance is a discharge and a creation, and when
they are one balance the code can do half of it.

**What Moriarty does.** `Debt<T>` is a distinct type from `Amount<T>`. A
payment is a transfer plus an allocation against a named obligation, the
allocation policy is one of AccrualFirst, PrincipalFirst or ProRata, and the
conversion rounding is none, floor or ceil. The successor profile states the
invariant in the source:

```mori
action payInterest(payment: Debt<USD>) {
  requires payment > debt(0, USD);
  requires payment <= pre.interest;
  next.interest = pre.interest - payment;
  ensures post.principal == pre.principal;
}
```

In the funded profile a payment names its transfer, its allocation and the
obligation it discharges as separate identified objects:

```mori
emit Transfer { id: TransferId("T1"), from: Payer, to: Lender,
                settlementAsset: Cash, amount: cash };
emit Repay    { allocationId: AllocationId("Alloc1"), transferId: TransferId("T1"),
                obligationId: ObligationId("Due100"), payer: Payer, nominalAmount: nominal };
```

Episode closure cannot discharge remaining notional:

```mori
guard state.notional == const.expected_outstanding_notional,
      "episode closure cannot discharge remaining notional";
```

`Debt<USD>` and `Amount<USD>` cannot be added or compared, so a routine that
subtracts a cash amount from a debt does not type check, and an obligation has
an identity, the pair of instance and due identifier. The loan example creates
two dues, `lam01:period1:PR` for principal and `lam01:period1:IP` for interest,
and settles each by name.

Guards of the atomic profile check that the nominal amounts settled between a
debtor and a creditor equal the transfers between them in the same asset, and
the bounded K definition rejects each failure by code. A `Repay` may discharge
nominal debt only from a `Transfer` that executed earlier in the same
candidate, so a transfer from a prior step, a future step or nowhere rejects as
`TRANSFER_NOT_IN_STEP`. A transfer whose parties or asset do not match the
obligation rejects as `TRANSFER_MISMATCH`, a discharge larger than the
component it discharges rejects as `ALLOCATION_COMPONENT`, and a positive
nominal payment that converts to zero settlement rejects as `DUST`. The funding
check uses converted settlement rather than nominal quantity, and ProRata
checks that `nominal × principal` fits UInt128 before dividing by total debt. A
settled obligation keeps its identity, so it cannot be recreated under the same
name, and a refund transfer never restores the payer's allowance. The due100
pay30 example moves the payer from 100 to 70, the lender from 0 to 30 and the
principal from 100 to 70, while the allowance records 70 remaining with 30
spent.

The loan's interest policy names the proof
`loan_first_period_interest_floor_v1`, and its settlement policy names
`loan_first_period_settlement_exact_v1` with `rounding none`, so the settlement
arithmetic may not contain a division at all.

No part of the language knows the collateral's price, so liquidation is an
action the contract enables and does not execute, and eligibility and execution
are different facts. DA06 is implemented as a local bounded loan episode only.
DA08 needs a normative atomicity fixture before the block-versus-transaction
question is settled, DA04, DA05 and DA10 need extension, and DA07 and DA09 need
pinned protocol fixtures.

**Reference sources.** Lending literature §§3.2–3.4, whose §7 excludes fees and
close factors; Kotzer §§III-B, III-C, V-C and Table II; Werner §3.2; Gogol
§IV-A3; ERC-3156 for the flash callback.

---

## F3 — Derivatives

**Function.** Payoffs that reference something else, a price, a rate or an
event, without conveying ownership of it. The family includes perpetuals,
futures, options and structured payoffs.

A derivative is a pair of contingent obligations. Each side promises to pay the
other an amount that depends on something neither controls, and margin is what
makes the promise believable. A position can be closed
and a duty it created can still be owed.

**Boundary.** "Synthetic" is used narrowly here: a reference-dependent payoff
without direct asset ownership. Broader usage that folds in wrapped, staking
and monetary claims is not adopted.

**Facet profile.** Execution means continuous margin maintenance, because a
perpetual has no maturity and the margin is checked every time the reference
moves. Settlement runs on two clocks, periodic funding while the position is
open and terminal settlement once when it closes, at different prices. Custody
is margin posted and at risk, and legal dependence is none on-chain while the
reference is an on-chain observation, because the contract computes and
enforces the settlement. Collateral and solvency carry the core risk, because a
gap in the reference can move a position past its margin before any liquidation
fires. Oracles are mandatory and adversarial, because the reference is the
entire payoff, and authorization separates the position-holder mandate from the
liquidator mandate. Price discovery is consumed and then fed back, since a
funding rate is a price the derivative market produces about itself.

**Action targets.**

- **DA11 — open / margin / fund / close derivative.**
  Requirement: position notional, margin and funding obligations.
  *Distinguishing test:* funding signs; insolvency; precise settlement
  convention.

  Funding is a periodic payment between longs and shorts whose direction
  depends on which side of the index the mark sits. When the direction is
  inverted the payment runs the wrong way, and every balance still sums to what
  it summed to before. Conservation holds. A balance-based model sees nothing,
  because the only fact it tracks is that value moved, and value did. The
  reader would see a market in which the side that should be paid is paying,
  with no invariant that fires. A position whose loss exceeds its margin has to
  exist as a state with a rule for who absorbs the shortfall, and a wrong
  implementation discovers insolvency only when a withdrawal fails. The
  settlement convention, mark, index or last trade and at which timestamp,
  decides the payoff, and an implementation that leaves the choice to whichever
  price was loaded last has no payoff at all.

- **DA12 — write / exercise / expire contingent claim.** *(shared with P)*
  Requirement: choice authority and exercise/payment dates.
  *Distinguishing test:* expiry does not erase an already exercised payment
  duty.

  An option is exercised on one date and paid on another. Suppose the holder
  exercises on the last day, the payment falls due the next day, and the expiry
  sweep also runs the next day. Expiry in the wrong implementation deletes
  every obligation attached to the contract, including the payment duty that a
  valid exercise created, so the holder did everything the contract asked and
  receives nothing, and the contract reports itself cleanly closed. The holder
  decides whether to exercise and the writer does not, and a rewrite that looks
  algebraically natural can move that choice to the wrong party.

**What goes wrong.** Each failure is a case of the position and its duties
being one object. The sign of a funding payment is then a sign on a balance
change, which no balance checks, the settlement price is whichever price the
position happened to be holding, and expiry deletes the object with every duty
attached to it. Insolvency is not a state, because there is no separate duty to
be short against, so it is recognized after the loss.

**What Moriarty does.** A funding rate is a typed observation carrying the
record DA20 specifies, so it names the feed it was derived from. Bounds are
declared on the agreement itself, `lifetime` capping the number of transitions
and `horizon` capping absolute time, so a derivative cannot become an unbounded
subscription.

A duration and a timestamp are different types from each other and from an
amount, so an exercise date cannot be compared against a quantity, and units
are carried as vectors, so a rate multiplied by a time yields a quantity whose
unit the checker computed. The horizon is a guard on every action,
`guard obs.now < uint(2000000000), "horizon expired"` in the examples, and each
action consumes one unit of the declared lifetime. An exercised claim is a
`Debt`-typed obligation with an identity that survives the position which
created it, and the mandatory proof claims require a compliant successor of a
compliant history, so an expiry that arrives at a state without the duty is not
a valid transition from a state that had it.

None of this makes the reference honest, and a price bound or a time-weighted
observation reduces what a specified attacker can do without removing the
economic risk. DA11 and DA12 both wait on pinned protocol fixtures, because
neither source gives an exercise calendar precise enough to fix DA12.

**Reference sources.** Werner §3.5 for atomic against non-atomic exposure, and
Gogol §IV-B3 for the perpetual and option mechanisms.

---

## F4 — Consensus-position claims

**Function.** Claims whose value derives from participation in consensus,
whether staking, liquid staking, restaking or shared security.

The obligation runs in both directions. A staker delegates capital to an
operator who performs validator duties, and the protocol owes rewards for
correct performance and takes a slashing penalty for incorrect performance. The
staker's claim is worth the delegated stake plus rewards minus slashing, and
only after a delay, because an exit is requested and then completed. The
capital keeps its exposure through the exit queue.

**Boundary.** Distinguish slashable allocation from ordinary leveraged lending
or a token-lock reward. A token that pays a reward for being locked, with no
security duties, is a yield product with no slashing exposure. Restaking is the
opposite case: the same capital is pledged to a second set of duties, a fault
in either set can slash it, and the exposure is the union of the duties and
not the sum of the rewards.

**Facet profile.** Execution runs in two phases, request then completion,
because the delay is imposed by consensus rather than chosen by the contract,
and settlement is delayed, and queued where the protocol rate-limits exits.
Custody is delegated to an operator whose authority persists until the exit
completes, and legal dependence is none, because the protocol itself computes
and applies the reward and the penalty, with no custodian and no court.
Collateral and solvency is loss allocation rather than liquidation: nobody
sells the stake, the protocol takes a share of it and the loss is allocated
across the claims. Oracles are internal, because the protocol knows what it
rewarded and slashed, and externally observable, because a liquid staking token
reads those facts. Authorization is a delegation mandate with its own scope.
Price discovery is consumed.

**Action targets.**

- **DA14 — stake / account rewards / slash.**
  Requirement: share-rate or rebase accounting; loss allocation.
  *Distinguishing test:* the same nominal token balance can have a changing
  entitlement.

  A rebasing token changes every holder's balance as rewards arrive and targets
  a fixed one-to-one value against the underlying. A reward-bearing token keeps
  every balance fixed and changes the conversion rate, so the same balance is
  worth more underlying tomorrow than today. A dual token separates principal
  and reward into two balances. Gogol reports rebasing as the less compatible
  of these with other protocols, and the reason is the test. An implementation
  that treats a rebasing balance as a fixed claim misses every reward, and one
  that treats a fixed share balance as a quantity misses every change in the
  rate. A lending market's collateral value then never moves while the token it
  holds is being slashed.

- **DA15 — request unstake / claim exit.**
  Requirement: pending exit identity, custody and delayed completion.
  *Distinguishing test:* an exit request is not immediate token delivery.

  Between the request and the completion the stake is in a third state, still
  slashable, no longer earning and not yet transferable. The wrong
  implementation has no such state. It either delivers the tokens at request
  time and hopes the protocol agrees later, or records nothing until completion
  and cannot say what the staker is owed in between. The ledger then shows an
  exit slashed after the staker was shown as paid, or a staker with no
  representable claim for the length of the queue.

**What goes wrong.** Both failures come from reading a balance as an
entitlement. The balance is the wrong number in the accounting case, and in the
exit case there is no balance to read, because the capital is between two
accounts and the model has only accounts, so the pending state has no
representation at all.

**What Moriarty does.** The pending state is an object with an identity, the
same machinery as DA18's asynchronous redemption and DA23's pending message: a
request creates a durable identified obligation, and partial or delayed
completion consumes it without resetting it. Affine authority applies, so a
partially completed exit consumes its authorization and the residual cannot
grow.

At the type level a share of a staking claim and an amount of the underlying
are different units, and the conversion between them carries a declared
rounding rather than a cast. At the guard level the exit request is an
obligation record that a completion must find and consume by name. At the proof
level a slash is loss allocation against claims and a transition like any
other, so the claim it reduces must be present in the predecessor history.

The queue length, the honesty of a delegated operator and the peg Gogol lists
beside slashing as the risk of a liquid staking token are outside what the
language decides, enforces or proves. DA14 waits on a pinned protocol fixture
and DA15 on a primary lifecycle source.

**Reference sources.** Gogol §IV-A1 and §3.6 for the token designs, and the
repository's pending-workflow analysis for the exit lifecycle.

---

## F5 — Tokenized off-chain claims

**Function.** Represent on the chain a claim that is enforced off it, whether a
real-world asset, an attested claim or a custodial receipt.

What the issuer owes is delivery of something a custodian holds where the chain
cannot see. The token holder has a claim against the issuer and the issuer has
a claim against the custodian and the custodian's performance is a matter of
contract and law, and none of those facts is on the chain. That token is
evidence of a deposit rather than possession of one.

**Boundary.** A representation does not establish a claim's economic substance,
and the redemption right is a separate specification. Identity and compliance
controls (ERC-3643, ERC-7943) are technical transfer policy and never legal
conclusions.

**Facet profile.** Execution splits into on-chain action and off-chain
performance, because the burn is on-chain and the wire is not, and settlement
splits across domains with different finality, because a bank transfer can be
reversed after a block cannot. Custody is the defining facet: an off-chain
custodian holds the underlying. Legal dependence is maximal, because the
custodian's duty, the issuer's solvency and the redemption right are enforced
off-chain or not at all, and collateral and solvency is issuer solvency.
Oracles are attestations, and an attestation is a signed statement by a party
who could sign something false. Authorization is institutional, because the
right to issue comes from a legal arrangement and not from a key, and price
discovery is external, with a gap between valuation time and publication time
where a redemption can be mispriced.

**Action targets.**

- **DA16 — issue / redeem external claim.**
  Requirement: attested claim with custody and legal assumptions.
  *Distinguishing test:* missing external settlement evidence cannot discharge
  the duty. An on-chain burn is not proof that anyone off-chain paid.

  A redemption burns the token and creates a duty on the issuer to deliver the
  underlying, discharged when authenticated evidence of the off-chain delivery
  arrives and not before. The burn is the only event the contract can see, so
  the wrong implementation treats the burn as the settlement and has no state
  for a token that is gone and a wire that has not arrived. Another accepts a
  bridge message or a local proof as evidence of foreign finality, and another
  treats an unavailable witness as an invalid commitment, which is the reverse
  error. Redemptions are then reported complete while the holders wait, and the
  ledger holds no record of who is still owed.

**What goes wrong.** The on-chain leg completes and is treated as settlement,
and the valuation fails by a second route. A fund token has an accounting
value, a redemption value, a market value and a stressed liquidation value,
with valuation time, publication time and invalidation differing again, so a
claim can be accounted for correctly while being illiquid, temporarily
unredeemable or unfit as collateral. A system that carries one NAV number
carries whichever was easiest to publish, and every consumer reads it as the
one they needed.

**What Moriarty does.** Every oracle and external effect has a named capability
and an assurance boundary, recorded in the deployable manifest. A typed
observation carries the record DA20 specifies, and the residual risk is stated
next to it: the source can still lie. External contract calls are excluded from
V0 entirely, and later they require a capability manifest, an allowlist and an
effect summary, because external behavior cannot inherit Moriarty guarantees.

A valuation observation carries its purpose in its type, so an accounting value
cannot be passed where a redemption value was required, and the redemption duty
is an obligation record that only a guard holding the selected authenticated
settlement observation discharges. An assurance boundary beside the named
capability says what the proof does not cover.

The language cannot make the custodian hold the asset, cannot make the issuer
solvent and cannot make the attestor truthful. It can make each of those an
assumption with a name, and it can refuse to discharge a duty on the strength
of an assumption alone. DA16 waits on a primary lifecycle source.

**Reference sources.** Gogol §§III-B, IV-A2; ERC-3643 and ERC-7943 for transfer
control; ERC-8330 for a bounded NAV observation; Centrifuge protocol source as
the worked example.

---

## F6 — Delegated asset management

**Function.** Someone else allocates your capital. Vaults, yield strategies,
allocators and standardized accounting entry points are the members.

A depositor hands assets to a vault and receives shares. The vault owes the
depositor their proportional share of whatever it holds, at its accounting rate
on the day they redeem, and the manager holds a mandate to move the assets
within a policy the depositor agreed to. A share is a claim on the accounting,
the accounting is a claim on the assets, and each arrow can fail on its own.

**Boundary.** "Vault" is the most overloaded word in the vocabulary. It can
mean a debt position, a custody container, an investment vehicle, a strategy
adapter or a standardized accounting entry point. A share is a unit
participating in portfolio accounting or a priority-defined claim, and it is
not automatically legal equity.

**Facet profile.** Execution is a synchronous deposit and withdraw, or an
asynchronous request lifecycle, because a vault whose assets are lent out or
held off-chain cannot convert on demand, and settlement is immediate or queued
for the same reason. Custody is with the vault, then with whatever the strategy
delegated to, and two strategies that delegate to one position hold one
exposure under two labels. Legal dependence varies with the underlying.
Collateral and solvency is a stress question, because the accounting can say
the vault owes more than it could realize by selling. Oracles are the valuation
problem under another name, since somebody supplies the NAV. Authorization
is a mandate with a scope and limits, because the depositor agreed to a policy
and not to each trade under it. Price discovery is a NAV, which is a valuation
and not a realizable price.

**Action targets.**

- **DA17 — deposit / mint / withdraw / redeem vault shares.**
  Requirement: asset/share conversions with method-specific rounding.
  *Distinguishing test:* fees, rounding direction, initial donation and zero
  shares.

  ERC-4626 fixes the directions. `convertToShares` and
  `convertToAssets` are ideal, caller-independent conversions and both round
  down. A deposit fixes the assets coming in and computes the shares going out,
  a mint fixes the shares going out and computes the assets coming in, a
  withdraw fixes the assets going out and computes the shares burned, and a
  redeem fixes the shares burned and computes the assets going out. Each
  preview must quote the caller nothing better than its execution will deliver.

  Behind the directions there is one rule: round against the caller, down on
  whatever the caller receives and up on whatever the caller pays, so that each
  remainder stays in the vault where it backs every other holder.

  The wrong implementation applies floor everywhere, because floor is what
  integer division does. Deposit and redeem then come out right while mint and
  withdraw come out wrong. A mint that floors the assets required lets the
  caller pay up to one unit less than the shares are worth, and a withdraw that
  floors the shares burned lets the caller burn up to one share fewer than the
  assets are worth. Each leaks at most one unit per call, and a caller who can
  choose amounts chooses the ones whose remainder is largest and repeats. The
  reader would see a vault whose assets per share fall slowly with no loss in
  the strategy.

  The standard says a preview includes fees while an ideal conversion excludes
  them, so a fee-aware quote and an ideal ratio are different numbers with
  different contracts.

- **DA18 — request / fulfill / claim asynchronous redemption.**
  Requirement: Pending → Claimable → Claimed, with residual request amount.
  *Distinguishing test:* partial claim and changed exchange rate; double claim
  rejects.

  An asynchronous redemption is a request that an operator later fulfils and
  the holder later claims, at an exchange rate the standard allows to change
  between the request and the claim. A request for 100 shares that is fulfilled
  in part leaves a residual request for the rest, with the same owner, the same
  controller and its own entitlement. The error is a request record with no
  residual amount: a partial fulfilment then either closes the whole request or
  leaves it open in full, and a second claim against a request already paid has
  nothing to stop it. Another implementation answers the synchronous preview
  methods with a number, when the standard requires them to revert for an
  asynchronous flow because there is no same-transaction rate to quote. A
  holder is then paid twice, or a holder's unfilled remainder vanishes.

- **DA19 — allocate / harvest / reinvest / unwind / rebalance.**
  Requirement: a bounded workflow of trades, debt, shares and fees.
  *Distinguishing test:* losses and debt persist through unwind; liquidity
  shortage.

  A strategy withdraws a specified upstream claim, accounts for fees and
  slippage, then deposits the authorized amount downstream, and every leg is a
  trade, a debt or a share operation in some other family. Unwinding runs it
  backwards. When the code reports the unwind complete once the position is
  closed while the debt it borrowed against is still outstanding, or reports
  partial leg success as atomic success, or expands a recursive strategy past
  its declared bound, the vault reports itself flat and owes money. A repeated
  sequence of small roundings and fees can inflate a downstream ratio while
  every operation passes its local check, and a strategy that can lose
  falsifies an invariant that says the share price only rises.

**What goes wrong.** Walk the donation through the first deposit. A new vault
has zero assets and zero shares. The attacker deposits 1 asset and, under the
usual first-deposit rule, receives 1 share, so the vault holds 1 asset against
1 share. The attacker then transfers 100 assets directly to the vault's address
without calling deposit. No shares are minted, because no deposit happened, and
the vault now holds 101 assets against 1 share. A victim deposits 100 assets.
The shares owed are 100 multiplied by 1 divided by 101, and the floor of that
is 0. The victim receives no shares. The vault holds 201 assets against 1
share, and the attacker's one share redeems for all of it. The attacker paid
101 in and takes 201 out, and every operation obeyed the rounding rule the
standard prescribes. The rounding direction was correct, and the zero-share
outcome is what the direction produces once the ratio has been made large
enough. The defence is not a different rounding. A minimum-shares bound on the
depositor's intent makes a deposit that would mint zero reject before any asset
moves, and a first-deposit rule the attacker cannot control closes the other
door.

**What Moriarty does.** Request duties are persistent identified obligations,
so Pending, Claimable and Claimed are distinct states, a residual request
amount survives a partial claim, and a second claim against a consumed
allocation has nothing to consume. Nested exposure is visible because each
delegation is a named capability with its own assurance boundary.

The successor type requirements name `Shares<Vault>` as distinct from
`Amount<Asset>`, and the conversion between them is an operation with a
declared policy. A preview is a query, a limit is a query, and an execution is
an action, so a preview cannot be mistaken for an authorization and a nonzero
preview beside a zero limit is a consistent state in which the deposit rejects.
The zero-share outcome is caught by a guard on the intent rather than the
vault: a signed outcome intent binds the minimum net shares the depositor will
accept, and a deposit that would deliver fewer rejects under IntentRefinement
before any effect is accepted. Each of deposit, mint, withdraw and redeem is a
conversion with its own `rounding` line and remainder disposition in policy, so
floor cannot be applied to all of them by default, and the mandatory proof
claims bind the effects of an unwind to the debt it leaves, so a strategy
cannot report closure while an obligation record survives.

No proof here covers whether the strategy is profitable, whether the underlying
is liquid when a redemption arrives, or whether two wrappers over one position
are independently backed. DA17 and DA18 are defined by their standards without
an implementation, and DA19 waits on a pinned strategy fixture.

**Reference sources.** ERC-4626 for the conversions and the donation case;
ERC-7540 for the request lifecycle; ERC-7575 for a share token outside the
vault contract; yield literature §§III-A Fig. 2, III-B, V-B; Morpho MetaMorpho
source as the worked allocator.

---

## P — Prediction markets and event-contingent claims

**Function.** Event-contingent claims, indexed by outcome and resolved by
evidence.

The obligation is a promise to pay the holder of the winning outcome one unit
of collateral per claim once the event is known. Collateral is deposited
against the full set of outcomes, a complete set of outcome claims is worth
exactly the collateral behind it whatever happens, and a single outcome claim
is worth the collateral if that outcome occurs and nothing otherwise. The set
of outcomes has to be complete so that exactly one can win.

**Boundary.** Held separate from F3 because resolution is evidentiary and not
price-mechanical. The question is what happened and who says so, and never
what the number is. DA12 sits in both families.

**Facet profile.** Execution is mint, trade and resolve, and settlement is all
at once after resolution, because nothing is owed until the event is known and
everything is owed the moment it is. Custody is collateral held against the
full outcome set, because the collateral backs every outcome until one wins,
and a merge must always be able to return it. Legal dependence varies with what
the event is and where the holder lives. Collateral and solvency: a complete
set must always be fully collateralized. Oracles are the entire trust model,
because the resolution is the payoff and there is no price mechanism to fall
back on, and authorization, who may resolve and under what evidence, is the
critical mandate. Price discovery produces a probability, because the price of
an outcome claim in collateral units is the market's estimate that the outcome
occurs.

**Action targets.**

- **DA13 — split / merge / resolve event claims.**
  Requirement: outcome-indexed claims and resolution evidence.
  *Distinguishing test:* no duplicate winning claim; invalid resolution
  rejects.

  A split takes one unit of collateral and issues one claim on each outcome. A
  merge takes one claim on each outcome and returns one unit of collateral.
  Resolution declares one outcome the winner and makes its claims redeemable.
  Across all outcomes the claims outstanding never exceed the collateral held,
  and the wrong implementation breaks that rule by treating a split as a mint
  and a merge as a burn. A merge that accepts a partial set returns collateral
  for claims still outstanding, and a resolution that can be submitted twice,
  or that accepts a second answer after the first, pays two winners. When the
  resolution comes from a source whose authority was never modelled there is no
  rejection path, so the first answer wins, and the market pays out more
  collateral than it held.

- **DA12 — write / exercise / expire contingent claim.** *(shared with F3)*
  *Distinguishing test:* expiry does not erase an already exercised payment
  duty.

  In a prediction market the exercise is the redemption of a winning claim
  after resolution, and the payment is the collateral it returns. A holder who
  redeems on the last permitted day has created a payment duty, and a market
  that sweeps unredeemed claims at expiry must not sweep that duty with them.
  The wrong implementation and the reader's view are as in the F3 tab.

**What goes wrong.** The market has a token contract and a resolver and
nothing between them. The token contract knows balances and the resolver knows
an answer. Nobody holds the claim set as a set, so nothing can check that a
merge consumed a complete one, and a merge can duplicate value. Nobody holds
the resolver's authority as a typed fact, so nothing can refuse an answer that
arrived through the wrong door, and an invalid or contested resolution has no
rejection path.

**What Moriarty does.** Split and merge are the composition operators DA24
names, under one conservation constraint: split partitions work and claims,
join cannot duplicate resource. Resolution is a typed observation with a named
capability, so the resolver's authority is part of the agreement, and evidence
that does not meet the declared provenance, freshness and domain requirements
rejects. Conditional-token split and merge belongs to a later composition layer
over signed capabilities, so the repository's crosswalk marks its
prediction-market rows as conditional on a primitive that is specified without
being built.

At the type level an outcome claim is indexed by its outcome and its event. At
the guard level a merge is a join, and a join that arrives without every
partition it was split into has nothing to join. At the proof level Core's
conservation of value is what a split and a merge are checked against.

The language does not know what happened. A valid oracle signature does not
establish economic truth, a resolver with a valid capability can still report a
falsehood, and the residual risk is stated next to the capability. DA13 waits
on a primary lifecycle source.

**Reference sources.** DeFi report contingent-claim targets for split, merge
and resolve, and Werner §3.5, shared with F3.

---

## ⊥ — Cross-cutting: what makes the other families composable

Every family above consumes an observation, acts under an authorization, lives
under parameters that somebody can change, and at some point waits on a message
or hands work to another agreement. A lending market needs a price it did not
compute. A swap needs a signed minimum. A staking exit and a cross-chain
redemption are both a pending message with a refund duty. None of those is a
financial product, and each is where a financial product fails when it is
composed with another.

**Action targets.**

- **DA20 — observe price / time / external event.**
  Requirement: typed observations with provenance, freshness and domain.
  *Distinguishing test:* stale or unauthorized evidence rejects.

  An observation is a signed record with a source, a feed, a unit, a timestamp,
  a freshness window, a sequence number, bounds and a fallback. The wrong
  implementation reads a number from a contract and uses it, so a validly
  signed spot price that an attacker can move inside one transaction is treated
  as a guarantee of economic truth. The correct implementation exposes the
  assumption, and the attack can remain economically possible while being
  visible. The reader of the wrong version would see a liquidation at a price
  that existed for one block.

- **DA21 — authorize exact plan / refine outcome intent.**
  Requirement: gross debit, net receipt, recipients, calls, new liabilities.
  *Distinguishing test:* a refund cannot restore gross capacity; fees count
  against the net goal.

  A signed authorization is a budget of gross spending by asset across every
  recipient, a list of permitted recipients, a net goal, a fee cap, and a list
  of liabilities the principal will accept. Partial fills consume it. The wrong
  implementation nets: a refund arrives and the allowance goes back up, so a
  route that spends, refunds and spends again has spent the budget twice
  against one signature. Another lets two reordered fills each read the same
  residual record, and another compares final balances and misses an
  intermediate call or a new liability that left them unchanged. The principal's
  one signature then moves more than it authorized, with every balance ending
  where it started.

- **DA22 — change parameters / pause / migrate.**
  Requirement: bounded administrative action under a fixed claim policy.
  *Distinguishing test:* cannot downgrade claims or reset work; affects existing
  positions.

  Governance is a financial action, and a parameter change is a transition like
  any other, applied under the authority version that was bound when the
  affected positions were created. Take the repository's loan, whose 31 days at
  8 per 100 accrued 33,972,602 micro-USD of interest. A vote that changes the
  rate after that interval has accrued governs the next interval, and an
  implementation that recomputes the interest already accrued under the new
  rate has reinterpreted a signed obligation retroactively. Another treats
  temporarily borrowed voting power as durable consent. The borrower then holds
  a debt whose terms changed after it was signed.

- **DA23 — send / receive / refund pending message.**
  Requirement: bounded pending commitments and finality evidence.
  *Distinguishing test:* delayed or duplicate delivery; explicit refund duty.

  A message creates a bounded pending commitment and a refund duty, and it is
  discharged by authenticated evidence of delivery. An implementation that
  treats a local proof or a bridge message as proof of foreign finality, keeps
  no state for a message sent and not yet delivered, or delivers a duplicate
  because the second arrival found nothing to check against leaves a
  cross-chain swap with an internal batch that succeeded and a withdrawal that
  was rolled back, with the model reporting success.

- **DA24 — sequence / parallel / interleave / synchronize / message.**
  Requirement: operator-specific authority, duties, conflicts and fan-in.
  *Distinguishing test:* split partitions work and claims; join cannot
  duplicate resource.

  Two calls on shared reserves compose under a declared observation order and a
  declared conflict rule. An implementation that checks each call against the
  same pre-state accepts both and applies effects that were only individually
  valid. Another inspects an intermediate state through a callback the
  composition did not declare, and another splits work between two branches and
  lets each carry the whole budget, so the join receives more than was split.
  What remains is two transactions that each passed and a reserve that is
  negative.

**The composition operators.** DA24 names sequence, parallel, interleave,
synchronize and message, each with its own authority rules, duty propagation,
conflict semantics and fan-in behavior. The work budget is the resource each
operator partitions. Every action costs exactly one unit of remaining work, the
closure reserve is not ordinary work, and a candidate that would spend more
work than it holds rejects before any action runs.

**The measured result.** Of 1,830 eligible protocol pairs, 1,645 compose
cleanly. The denominators are 143 pairs within a category against 1,687 pairs
across categories, and the failures divide as 3 within and 182 across. Within a
category the failure rate is 2.10%. Across categories it is 10.79%, which is
5.14 times the within-category rate. An eligible pair is one the composition
algebra can evaluate under explicit eligibility rules, and a clean composition
is one the algebra accepts without a conflict between the two protocols'
obligations.

The reason the rate rises at the boundary is the reason for the model. Inside a
family the two protocols make the same assumptions about settlement timing,
custody, authority and where the price comes from, which is what defines the
family. Across a boundary the output of one protocol becomes the input of
another that assumed something different about it: a vault share is used as
collateral by a lender that read its accounting value as a market value, or a
pending redemption is counted as cash. Each of those is a facet value that one
side assumed and the other did not check, and a facet value is what a type
system can carry and an operational semantics can police, so the failures
concentrate where the typed distinctions are dropped. The repository records
high confidence for the counts and rates and medium confidence for the
taxonomic interpretation. An earlier informal claim of sixty times was checked
against the corpus and is wrong.

**What this tab does and does not establish.** The compiler can reject unit
errors, unknown effects and unbound authority. Core can define accounting,
valid transition order, residual-duty preservation and finite work. The proof
relation can bind those predicates to the concrete statement, and ledger
acceptance can consume the correct predecessor and residual authority exactly
once. None of those steps establishes market liquidity, oracle truth,
profitable strategy selection, honest governance, external custody, inclusion
fairness or future settlement. DA20 is implemented as simulated observations
only and DA21 as local restricted profiles only. DA24 is specified without
being built, while DA22 waits on a pinned policy fixture and DA23 on a primary
lifecycle source.

---

## Acceptance criteria

These are constraints on the build, and the site must satisfy them without
reporting that it does.

- Every tab renders and every block in every tab is populated. - Every action
  target from DA01 to DA24 appears with its semantic requirement and its
  distinguishing test, taught through the wrong implementation, the diverging
  case and the observable consequence, and DA12 appears in both its tabs marked
  shared. - Family identifiers and facet names are never abbreviated or
  reordered. - Every composition operator named by DA24 is listed. - Every
  facet is answered in every tab, and every "none" carries its reason. -
  Category identifiers are classification labels, never Moriarty source syntax.
  - Every worked figure is one the repository carries: the swap output is
  19,743, the loan interest is 33,972,602, and the composition denominators are
  143 and 1,687. - The composition result carries the repository's
  qualification: high confidence for the counts and rates, medium for the
  taxonomic interpretation. - Conditional-token split and merge is presented as
  architecture and never as a shipped feature. - The word "vault" is qualified
  wherever the page uses it. - The cross-cutting tab carries the cross-chain
  material from CONTENT-SPEC §7: per-layer atomicity, route-specific custody
  manifests and the bridge-rollback trace. - No tally is displayed to the
  reader, per [`VOICE.md`](VOICE.md).
