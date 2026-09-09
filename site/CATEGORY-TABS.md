# DeFi category tabs — complete specification

The site's category section is a **tabbed explainer with one tab per DeFi
category**. Completeness is a hard requirement: every economic family is
present, every action target from DA01 to DA24 appears in exactly one tab, and
no tab is a stub. A category with less repository evidence gets a shorter tab,
not a missing one.

One tab holds each economic family. A final tab holds the cross-cutting targets,
which are the ones every other family depends on. The build must place every
action target from DA01 to DA24, and DA12 belongs to two families at once.

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

DA12, `write / exercise / expire contingent claim`, is tagged `F3;P` in the
source matrix and belongs in both the Derivatives and the Prediction Markets
tabs. Show it in both, and mark it as shared so it is not read as two separate
targets.

The section has to teach, not only assert. Each tab states the financial
function before the mechanism, explains why the family sits where it does on
each facet, and walks each distinguishing test through the case where a
plausible wrong implementation parts company with the correct one.

---

## Tab structure

Every tab renders the same blocks in the same order. The regularity is what
makes the section read as one instrument rather than a set of assembled pages.

1. **What this category is** — the financial function, and the boundary that
   excludes neighbours. This block names the obligation that exists, who owes
   it to whom, and what has to be true for the claim to be worth anything.
2. **Facet profile** — where this family typically sits on the mandatory
   facets. This is the part that shows the model is a classification and not a
   list. Each value carries its reason, and a value of "none" says why it is
   none rather than unexamined.
3. **Action targets** — each with its semantic requirement and its
   distinguishing test. The distinguishing test is the emphasized element, and
   it is taught: the plausible wrong implementation, the case where it
   diverges, and what a reader would see if the wrong version shipped.
4. **What goes wrong** — the concrete failure this category produces in
   practice, stated as a mechanism rather than an anecdote.
5. **What Moriarty does about it** — the specific language construct, type,
   policy or authority rule that addresses it. Real `.mori` where it exists.
   The block says what is forbidden at the type level, at the guard level and
   at the proof level, and it says what is not forbidden.
6. **Reference sources** — the pinned standards and papers behind the targets,
   with what each one contributes.

---

## F1 — Exchange and price discovery

**Function.** Convert one asset into another and, in doing so, produce a price.
Includes constant-function market makers, order books, concentrated-liquidity
positions, aggregators and routers.

The exchange is the family in which no obligation outlives the transaction. A
trader brings one asset and leaves with another, and when the two transfers
have settled nobody owes anybody anything. The claim that does persist belongs
to the liquidity provider. A provider hands reserves to the pool and receives a
share, and that share is a proportional claim on whatever the reserves are worth
when the share is burned, not on what they were worth when it was minted. For
the trader's momentary claim to be worth anything the reserves have to exist and
the output has to be computed from the reserves as they stand at that moment.
For the provider's claim to be worth anything the pool has to preserve the
proportion between shares and reserves through every swap and every other
provider's entry and exit.

**Boundary.** The exchange function is separate from the mechanism that
implements it. Uniswap V4's singleton accounting and hooks change the mechanism
and the callback assumptions; they do not change the exchange function. Routing
and aggregation are execution-facet properties layered on top of F1, not a
distinct family.

**Facet profile.** Execution: atomic on-chain, or intent/solver-mediated ·
Settlement: same-domain, immediate · Custody: pool-held during the swap ·
Legal dependence: none · Collateral and solvency: reserves are the backing ·
Oracles: usually none for CFMMs — the pool *is* the price; oracle-dependent for
some venues · Authorization: per-transaction, with slippage bounds ·
Price discovery: this is the family that produces it.

Execution is atomic because a swap that half-completes is not a swap, and the
atomic profile emits both transfers from one action so that a rejection exposes
neither. The intent and solver variant sits on the same facet because it
changes how the trade is authorized and routed, not what the trade is.
Settlement is immediate because both transfers land on one ledger inside the
transaction that computed them, and custody is with the pool because the input
is credited to the reserve before the output leaves it. Legal dependence is
genuinely none rather than unexamined: the trader's claim is discharged inside
the transaction and there is no counterparty left to pursue, and when a pool
holds an asset whose value depends on a custodian that dependence belongs to
the asset under F5. Collateral reduces to the reserves, because a pool cannot
owe more than it holds. Oracles are usually none because the pool computes its
price from its own reserves, which is also why the pool is a target: its price
is consumed elsewhere as an observation, and a large trade can move it.
Authorization is per-transaction because `min_out` is the whole of what the
trader authorized.

**Action targets.**

- **DA01 — swap exact input / exact output.**
  Requirement: asset-indexed exchange with fee and slippage accounting.
  *Distinguishing test:* rounding; reserve safety; net minimum.

  The repository's example fixes the arithmetic. Reserves are 1,000,000 A and
  2,000,000 B, the fee is 997/1000, and the input is 10,000 A. The effective
  input is 9,970,000. The numerator is 9,970,000 multiplied by 2,000,000, which
  is 19,940,000,000,000. The denominator is 1,000,000 multiplied by 1000 plus
  9,970,000, which is 1,009,970,000. The quotient is 19,743 with a remainder of
  162,290,000 numerator units, and the floor is 19,743 B. After the swap the
  reserves are 1,010,000 A and 1,980,257 B.

  The plausible wrong implementation rounds the other way. A ceiling gives
  19,744, and the pool pays one quantum it did not earn. The quantum is small
  and the loss is real, and it recurs on every swap whose remainder is nonzero,
  which is nearly all of them. A reader watching such a pool would see reserves
  drift below what the constant-product identity predicts, and no single
  transaction would look wrong. Rounding down is correct because the remainder
  then stays in the reserve where it backs every provider share, and the policy
  says so: `unpaid output remains in reserve_b`.

  Reserve safety is the second test. The formula never yields an output equal
  to the reserve for a finite input, but a different fee path, a different
  rounding or an unchecked subtraction can. A pool whose B reserve reaches zero
  has a product of zero, and every later price is undefined. In the atomic
  profile the next `floor_div` would reject a zero denominator rather than
  misprice, but the pool would still be dead. The guard `output_calculated <
  state.reserve_b` forbids the emptying trade by name.

  The net minimum is the third. The trader's `min_out` must be compared against
  the output after the fee, because that is what the trader receives. A wrong
  implementation compares it against a gross figure before the fee, and the
  trader receives less than the minimum they signed. Such fills look like
  ordinary slippage unless the fee accounting is visible, which is why the
  comparison basis is a declared field of the policy.

- **DA02 — provide / remove liquidity.**
  Requirement: share mint/burn and reserve contributions.
  *Distinguishing test:* proportional entitlement, and the donation and
  zero-supply boundaries — the first depositor and the direct-transfer donation
  are where share maths breaks.

  Proportional entitlement is the ordinary case. A pool holding 100 assets
  against 80 shares receives 25 assets and issues 25 multiplied by 80 divided
  by 100, which is 20 shares under a floor policy. The new provider holds 20 of
  100 shares against 125 assets, and nobody's proportion has moved.

  The zero-supply boundary is where the formula has nothing to say. The first
  deposit has no ratio to preserve, because shares and assets are both zero,
  and a share count of assets multiplied by zero divided by zero is not a
  number. A plausible implementation special-cases this with a rule that was
  never written down, and that rule decides who can be robbed later. The
  donation boundary is the attack that follows. The full walk through the
  arithmetic is in the F6 tab, because the case is the same for a pool share
  and a vault share and the F6 tab owns the vault arithmetic. What the reader
  would see is a second depositor whose assets arrive and whose share count is
  zero.

- **DA03 — open / adjust / close liquidity position.**
  Requirement: bounded position identity and range.
  *Distinguishing test:* fee allocation and finite tick/range traversal.

  A concentrated position is liquidity that is live only while the price is
  inside a declared range. Fees from a trade belong to the positions that were
  in range when the trade crossed them, and a trade that moves the price across
  several ranges has to visit each in order. A plausible wrong implementation
  credits fees to whichever positions are in range at the end of the trade,
  which pays positions that supplied nothing to the early part of the path and
  skips positions that did. The reader would see fee income that does not
  match the time a position spent in range.

  The traversal has to be finite. A trade that can cross an unbounded number
  of ranges is a loop with no declared bound, and the language has no such
  loop. The atomic profile has no loops or recursion, and the surface
  language's compile-time loops elaborate to a finite Core, so a range
  traversal must declare its maximum before it can be written. This target
  waits on a pinned protocol fixture.

**What goes wrong.** Integer division decides who keeps the remainder. A swap
that rounds the wrong way, or a share calculation whose zero-supply case is
unguarded, transfers value silently and legally. Reserve safety is not implied
by the constant-product identity: an output that empties a reserve satisfies the
formula and destroys the pool.

The mechanism runs from the division to the loss. A conversion between two
quantities produces a remainder, and somebody keeps it. The code decides who by
the direction it rounds and by whether the leftover stays in the reserve or
leaves with the output. When the direction favours the caller the pool leaks
one quantum per operation, and the leak compounds with volume rather than with
any single trade. When the zero-supply case is left to a special rule, the
first depositor controls the ratio every later depositor is measured against,
and that is the lever the donation attack pulls.

**What Moriarty does.** Rounding is a declared artifact, not a side effect of
`/`. The `policy` block names the unit, the derivation, the rounding direction,
the disposition of the remainder, the comparison basis and the proof obligation:

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

Reserve safety and the net minimum are named guards, and a failure is a named
rejection rather than a silent adjustment:

```mori
guard arg.min_out <= output_calculated,      "minimum output not met";
guard output_calculated < state.reserve_b,   "output would empty reserve";
```

Concrete instance in the repository: reserves 1,000,000 A and 2,000,000 B, a
997/1000 fee, a 10,000 A input, output exactly **19,743 B**. Requesting 19,744
produces a named slippage failure.

At the type level the two reserves are `Amount<AssetA_quantum>` and
`Amount<AssetB_quantum>`, and addition requires identical units, so an A
quantity cannot be added to a B quantity or compared with one. Multiplication
adds unit exponents and `floor_div` subtracts them, so the quotient of the swap
formula is a B quantity because the arithmetic made it one. `floor_div` is the
only division in the language. It rejects a zero denominator and returns the
mathematical floor, and every operand and intermediate result must fit UInt128
before any later division can hide an overflow.

At the guard level every guard carries a message, and a false guard rejects the
whole action so that no working write, effect or authority consumption is
exposed. The swap also reserves its own closure: `reserve swap for close` and
the guard `remaining > uint(1)` keep the last transition for the provider's
withdrawal, so the pool cannot spend its lifetime on swaps and leave the
provider unable to close.

At the policy level every `set` of an Amount and every Amount field in an
`emit` must be covered by exactly one policy target. A value that passed through
a `floor_div` carries that node as provenance, and a policy that says `rounding
floor` must name the one `let` that did the dividing, while a policy that says
`rounding none` requires that no division touched the value at all. Rounding
cannot be inherited by accident and cannot be claimed for a value that was never
rounded. The `proof` string names a mandatory future claim.

What this does not forbid matters as much. The language does not make the
pool's price true, and a large trade that moves it is still a valid swap. The
`derivation`, `remainder` and `comparison` strings are documentary, bound into
the program hash but not parsed, so they record intent rather than check it.
The policy establishes that the rounding is declared and provable, not that the
fee parameter is sensible or that the ledger will accept the transfer. DA01 is
implemented as a local fixed exact-input example only, DA02 needs extension,
and DA03 needs a pinned protocol fixture.

**Reference sources.** AMM literature §§2.3.2, 3.1, 3.3, 4; Uniswap V4 core for
singleton accounting and hook-modified mechanism assumptions.

The AMM survey supplies the action set, and a reader should note that its
formulas are stated over real numbers and need an explicit finite arithmetic
interpretation before they say anything about integers. Uniswap V4 core is the
mechanism source: singleton accounting and hooks alter the callback
assumptions a composition has to respect without altering what a swap is.

---

## F2 — Credit and collateralized debt

Lending markets, collateralized debt positions, debt-backed stablecoins and
flash loans. This is the largest tab, and it is the family the language is
organized around.

**Function.** Create an obligation to repay, usually secured, and manage it
through accrual, partial performance, default and discharge.

The obligation is a promise about the future. A borrower owes a nominal
quantity to a lender, the quantity grows on a schedule, and the borrower can
perform against it in pieces. The lender's claim is worth something only if the
borrower pays or the collateral can be seized and sold for at least the debt.
The first depends on the borrower. The second depends on an encumbrance the
borrower cannot undo, on a price that somebody outside the contract has to
supply, and on an authority to seize that is different from the authority to
borrow. Everything in this tab is one of those pieces.

**Boundary.** *Nominal debt is not a transfer.* This distinction is the reason
F2 is the family Moriarty is architecturally organized around. A credit system
that models debt as a balance rather than an obligation cannot express partial
repayment correctly.

A transfer moves cash now. A debt is a number about later. They have to be
accounted for separately because a payment is both at once: cash moves, and a
named obligation is reduced by an amount that is not necessarily the cash
amount, since the settlement asset can differ from the denomination and the
conversion can round. The repository has a concrete record of the wrong model.
An earlier retained composition model reduced a duty from 50 to 30 after the
funding transfer had been removed from the action list, and it left the payer
holding 50 and the lender holding nothing. Debt moved without the cash that was
supposed to fund it. That is what a balance-based model does when asked about a
partial payment, and the successor kernel exists to refuse it.

**Facet profile.** Execution: on-chain, often multi-step · Settlement:
same-domain, but discharge and transfer are separate events · Custody:
collateral is encumbered, not surrendered · Legal dependence: none on-chain;
material for RWA-collateralized variants · Collateral and solvency: the defining
facet · Oracles: required — liquidation depends on external price ·
Authorization: borrower authority to draw, liquidator authority to seize, and
they are different mandates · Price discovery: consumed, not produced.

Execution is multi-step because a loan is a lifecycle and the state between
its transactions is the debt. Discharge is a separate event from transfer
because the transfer that funds a repayment and the allocation that discharges
the debt are two objects with two identities. Custody is encumbrance rather
than surrender: the collateral still belongs to the borrower and is locked so
it can be seized on default. Legal dependence is none on-chain because the
seizure is executed by the contract without a court, and it becomes material
the moment the collateral is an F5 claim. Oracles are required because a
liquidation asks whether the pledge is still worth the promise, and the
contract cannot answer from its own state. Authorization splits because the
borrower may draw and the liquidator may seize, and neither may do the other's
action.

**Action targets.**

- **DA04 — supply / redeem lending claims.**
  Requirement: claim shares and liquidity-constrained withdrawal.
  *Distinguishing test:* insufficient pool liquidity must reject **without
  erasing the claim.** Failure to withdraw is not loss of entitlement.

  A supplier's claim is a share of a pool that has lent most of its cash out.
  The share is good and the cash is not there. A withdrawal that exceeds what
  the pool holds has to fail, and the only correct failure leaves the share
  exactly where it was. The plausible wrong implementation processes the
  withdrawal as far as it can and records the rest as done, or burns the shares
  and transfers what is available, or marks the request consumed on the way to
  a revert that never fires. The reader would see a supplier whose shares fell
  during a utilization spike, and a lender that grew richer by the same amount
  without anyone having borrowed.

- **DA05 — post / release collateral.**
  Requirement: encumbrance and debt-dependent release.
  *Distinguishing test:* release cannot violate the collateral rule.

  Posting collateral locks an asset the borrower still owns. Releasing it must
  check the collateral rule as it will stand after the release. The plausible
  wrong implementation checks the ratio against the balance before subtracting
  the released amount, and the borrower walks out under-collateralized by
  exactly the release. A second lets one pledge back several duties and
  releases it against the first duty that asks. A third converts a doubled
  accounting ratio directly into a doubled debt allowance, which is what a
  donation into a collateral vault produces when the valuation policy is not
  pinned. The reader would see a liquidation that recovers less than the debt.

- **DA06 — borrow / accrue / repay.**
  Requirement: nominal debt distinct from transfers; rate and time arithmetic.
  *Distinguishing test:* **partial repayment preserves principal and interest
  allocation.** This is the project's flagship case.

  Take a debt of principal 100 and accrued interest 10. A payment of 7 under an
  interest-first rule leaves principal 100 and interest 3, and the debt is
  outstanding at 103. Under a principal-first rule the same payment leaves
  principal 93 and interest 10. Under a pro-rata rule the principal share is
  the floor of 7 multiplied by 100 divided by 110, which is 6, so principal
  becomes 94 and interest 9. Each rule leaves a different debt behind the same
  payment. The plausible wrong implementation subtracts 7 from whichever field
  the code reaches first, or from a single combined balance, and the reader
  would see interest that vanished before it was paid or principal that fell
  when only interest was due.

  The time arithmetic is its own test. Simple interest of 100 at 10 per 100 for
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

  With outstanding debt of 100 and a close limit of one half, an authorized
  partial liquidation repays 40 and leaves debt of 60, and the collateral taken
  and any bonus are computed separately from the debt reduced. The wrong
  implementation repays 60 despite the limit, or seizes at a stale or wrongly
  denominated price, or treats the shortfall between debt and recovered
  collateral as if it had disappeared. Maturity before forfeiture is the other
  half: a fixed-term loan with no early liquidation ends with repayment plus
  interest or with forfeiture of the collateral, and a liquidation that fires
  before term is a seizure without authority. The reader would see borrowers
  liquidated harder than the rule permits, or collateral taken from a loan not
  yet due.

- **DA08 — flash borrow / repay atomically.**
  Requirement: atomic multi-leg settlement with fee.
  *Distinguishing test:* every loan repaid within the same atomic transaction.
  Flash borrowing is a capability with legitimate uses, not a vulnerability.

  A flash loan of principal 100 with fee 1 is repaid at 101 inside the one
  admitted atomic operation, and every effect of the borrowing binds to that
  operation. One wrong implementation accepts 100 and forgets the fee. Another
  accepts 101 in a later transaction within the same block, which is the
  block-versus-transaction confusion one of the surveyed papers commits on its
  own page. A third fails the final leg after earlier legs have moved tentative
  balances and does not unwind them. The reader would see a pool that is
  briefly short and never made whole.

- **DA09 — refinance / novate / capitalize.**
  Requirement: a workflow over old debt, new debt, collateral and signed
  liability authority.
  *Distinguishing test:* reject refinance without old-debt discharge;
  capitalization changes liability.

  A refinance discharges an old obligation and creates a new one in a single
  workflow, and the collateral moves from backing the first to backing the
  second. The wrong implementation creates the new debt and leaves the old one
  outstanding, so the borrower owes twice, or discharges the old and fails to
  create the new, so the lender holds nothing. Capitalization is subtler.
  Rolling accrued interest into principal does not change what is owed today
  and does change what accrues tomorrow, so it is a new liability that needs
  the borrower's signed authority, and the wrong implementation capitalizes
  silently on a missed payment. The reader would see a debt whose growth rate
  changed without anyone having agreed.

- **DA10 — issue / burn debt-backed stablecoin.**
  Requirement: issuance authority plus debt and collateral.
  *Distinguishing test:* burn amount and released collateral obey the specified
  debt rule.

  A debt-backed stablecoin is a collateralized debt position whose debt is the
  coin. Issuing creates debt against collateral under an issuance authority.
  Burning reduces the debt by the burned amount under the same rule, and the
  collateral released is what the rule permits after the reduction, not a
  proportional slice of what was posted. The wrong implementation releases
  collateral in proportion to the burn while accrued fees remain unpaid, or
  burns coins without reducing debt, or reduces debt without a burn. The peg is
  an economic objective and not a static equality, and a system that treats it
  as an equality cannot represent a coin worth less than its debt.

**What goes wrong.** A repayment reduces a balance without discharging the right
obligation. Interest and principal are allocated by whichever subtraction the
code happened to perform first. A withdrawal that cannot be funded erases the
claim instead of rejecting. A refinance issues new debt without discharging old.

The mechanism is the same in each case. The system holds one number where it
needed two objects, and the operation that should have touched one of them
touches the number. A payment is a transfer and an allocation, and when they
are one balance the allocation rule is whatever order the subtractions ran in.
A withdrawal is a request and a claim, and when they are one balance a request
that cannot be funded consumes the claim. A refinance is a discharge and a
creation, and when they are one balance the code can do half of it.

**What Moriarty does.** `Debt<T>` is a distinct type from `Amount<T>`. A payment
is a transfer *plus* an allocation against a named obligation, and the allocation
policy is explicit — AccrualFirst, PrincipalFirst or ProRata — with explicit
none/floor/ceil conversion rounding. The successor profile states the invariant
directly in the source:

```mori
action payInterest(payment: Debt<USD>) {
  requires payment > debt(0, USD);
  requires payment <= pre.interest;
  next.interest = pre.interest - payment;
  ensures post.principal == pre.principal;
}
```

`ensures post.principal == pre.principal` — paying interest may not touch
principal. In the funded profile a payment names its transfer, its allocation
and the obligation it discharges as separate identified objects:

```mori
emit Transfer { id: TransferId("T1"), from: Payer, to: Lender,
                settlementAsset: Cash, amount: cash };
emit Repay    { allocationId: AllocationId("Alloc1"), transferId: TransferId("T1"),
                obligationId: ObligationId("Due100"), payer: Payer, nominalAmount: nominal };
```

The bounded K definition executes exactly this and rejects the failure modes by
code and index: a positive nominal payment converting to zero settlement rejects
as `DUST`; converted settlement — not nominal quantity — is checked against the
preceding transfer's funding; ProRata checks that `nominal × principal` fits
UInt128 before dividing by total debt. Episode closure cannot discharge
remaining notional:

```mori
guard state.notional == const.expected_outstanding_notional,
      "episode closure cannot discharge remaining notional";
```

At the type level `Debt<USD>` and `Amount<USD>` cannot be added or compared,
so a routine that subtracts a cash amount from a debt does not type check. An
obligation has an identity, and in the atomic profile that identity is the pair
of instance and due identifier for the life of the agreement. The loan example
creates two dues, `lam01:period1:PR` for principal and `lam01:period1:IP` for
interest, and settles each by name.

At the guard level the atomic profile checks that the nominal amounts settled
between a debtor and a creditor equal the transfers between them in the same
asset, so an obligation record cannot assert settlement without matching
movement. The successor kernel adds the funded rule: a `Repay` may discharge
nominal debt only from a `Transfer` that executed earlier in the same
candidate, and a transfer from a prior step, a future step or nowhere rejects
as `TRANSFER_NOT_IN_STEP`. A transfer whose parties or asset do not match the
obligation rejects as `TRANSFER_MISMATCH`. A discharge larger than the
component it discharges rejects as `ALLOCATION_COMPONENT`. A settled obligation
keeps its identity and its record, so it cannot be recreated under the same
name, and a refund transfer never restores the payer's allowance. The due100
pay30 example runs: the payer moves from 100 to 70 and the lender from 0 to 30,
the principal moves from 100 to 70, and the allowance records 70 remaining and
30 spent.

At the proof level the loan's interest policy names
`loan_first_period_interest_floor_v1`, and its settlement policy names
`loan_first_period_settlement_exact_v1` with `rounding none`, so the settlement
arithmetic may not contain a division at all. The closure guard on `notional`
is the atomic profile's version of the successor's `ensures`: an episode may
end, and the remaining notional survives it.

What is not forbidden is stated as plainly. The language does not know the
collateral's price. Liquidation is an externally triggered action that the
contract enables and does not execute on its own, so eligibility and eventual
execution are different facts. The kernel accepts a third party as payer under
its own allowance and does not require the debtor to pay. Dust is refused rather
than resolved, so a conversion that rounds a positive nominal to zero settlement
is a rejection today and a policy decision later. DA06 is implemented as a local
bounded loan episode only, DA08 needs a normative atomicity fixture before the
block-versus-transaction question is settled, DA04, DA05 and DA10 need
extension, and DA07 and DA09 need pinned protocol fixtures.

**Reference sources.** Lending literature §§3.2–3.4; Kotzer §§III-B, III-C, V-C;
Werner §3.2; Gogol §IV-A3; ERC-3156 for the flash callback and repayment
contract.

The lending-pool survey's §3.4 supplies the transition set, and its §7
excludes fees and close factors, so a theorem about the model is not
conformance to a protocol. Kotzer's §III and Table II compare loan mechanisms,
including fixed-term loans with no early liquidation, §V-C is the source for
partial liquidation and the close factor, and §III-B is the page that uses
block and transaction interchangeably for flash repayment, which is why DA08
needs its own fixture. Gogol §IV-A3 covers the debt-backed stablecoin
lifecycle. ERC-3156 is the callback shape a flash loan has to satisfy and the
repayment contract it imposes.

---

## F3 — Derivatives

**Function.** Payoffs that reference something else — a price, a rate, an event —
rather than conveying ownership of it. Perpetuals, futures, options, structured
payoffs.

A derivative is a pair of contingent obligations. Each side promises to pay the
other an amount that depends on something neither controls, and margin is what
makes the promise believable. For the claim to be worth anything the margin has
to be adequate at the moment the reference moves against the holder, the
reference has to be honest, and the obligation created by an exercise or a
funding period has to survive until it is paid. The position and the duty are
different things. A position can be closed and a duty it created can still be
owed.

**Boundary.** "Synthetic" is used narrowly here: a reference-dependent payoff
without direct asset ownership. Broader historical usage that folds in wrapped,
staking and monetary claims is preserved as a source translation, not adopted.

**Facet profile.** Execution: continuous margin maintenance, not one-shot ·
Settlement: periodic funding plus terminal settlement, on different clocks ·
Custody: margin is posted and at risk · Legal dependence: none on-chain ·
Collateral and solvency: margin adequacy and insolvency handling are the core
risk · Oracles: mandatory and adversarial · Authorization: position-holder
mandate distinct from liquidator mandate · Price discovery: consumed; funding
rates feed back into it.

Execution is continuous because a perpetual has no maturity and the margin has
to be checked every time the reference moves. Settlement runs on two clocks
because funding is paid on a schedule while the position is open and terminal
settlement once when it closes, and the two use different prices. Legal
dependence is none on-chain because the contract computes and enforces the
settlement, and it is genuinely none only while the reference is an on-chain
observation. Collateral and solvency carry the core risk because a gap in the
reference can move a position past its margin before any liquidation fires.
Oracles are adversarial because the reference is the entire payoff. Price
discovery is consumed and then fed back, since a funding rate is a price the
derivative market produces about itself.

**Action targets.**

- **DA11 — open / margin / fund / close derivative.**
  Requirement: position notional, margin and funding obligations.
  *Distinguishing test:* funding **signs**; insolvency; precise settlement
  convention. A funding payment whose direction inverts is a total loss of
  meaning, and sign errors are invisible in balance-based models.

  Funding is a periodic payment between longs and shorts whose direction
  depends on which side of the index the mark sits. When the direction is
  inverted the payment runs the wrong way, and every balance still sums to what
  it summed to before. Conservation holds. A balance-based model sees nothing,
  because the only fact it tracks is that value moved from one account to
  another, and value did. The reader would see a market in which the side that
  should be paid is paying, with no invariant that fires. Insolvency is the
  second test: a position whose loss exceeds its margin has to exist as a state
  with a rule for who absorbs the shortfall, and a wrong implementation
  discovers insolvency only when a withdrawal fails. The settlement convention
  is the third: whether a position settles at mark, index or last trade, and
  at which timestamp, decides the payoff, and an implementation that leaves
  the choice to whichever price was loaded last has no defined payoff at all.

- **DA12 — write / exercise / expire contingent claim.** *(shared with P)*
  Requirement: choice authority and exercise/payment dates.
  *Distinguishing test:* **expiry does not erase an already exercised payment
  duty.**

  An option is exercised on one date and paid on another. Suppose the holder
  exercises on the last day, the payment falls due the next day, and the expiry
  sweep also runs the next day. A plausible wrong implementation expires the
  contract by deleting every obligation attached to it, including the payment
  duty that a valid exercise created. The holder did everything the contract
  asked and receives nothing, and the contract reports itself cleanly closed.
  Choice authority is the other half of the test: the holder decides whether
  to exercise and the writer does not, and a rewrite that looks algebraically
  natural can move that choice to the wrong party.

**What goes wrong.** Funding sign inversion. Settlement convention ambiguity —
whether a position settles at mark, index or last trade, and at which timestamp.
An expiry sweep that cancels obligations which were already validly exercised.
Insolvency that is recognized after the loss rather than as a state.

Each of these is a case of the position and its duties being one object. When
they are one object the sign of a funding payment is a sign on a balance
change, which no balance checks. The settlement price is whichever price the
position happened to be holding. Expiry deletes the object and takes the duties
with it. Insolvency is not a state because there is no separate duty to be
short against.

**What Moriarty does.** Time and duration are distinct types from amounts, so an
exercise date cannot be compared against a quantity. Obligations survive
independently of the position that created them — an exercised claim is a
`Debt`-typed duty that expiry cannot silently erase, because erasing it would
violate an `ensures` clause. Observations are typed with provenance, freshness
and domain (DA20), so a funding rate carries the feed, unit, timestamp and
sequence it was derived from.

Bounds are declared on the agreement itself — `lifetime` caps the number of
transitions and `horizon` caps absolute time — so a derivative cannot become an
unbounded subscription.

At the type level a duration and a timestamp are different types from each
other and from an amount, and the atomic profile carries units as vectors, so a
rate multiplied by a time yields a quantity whose unit the checker computed
rather than assumed. At the guard level the horizon is a guard on every action,
`guard obs.now < uint(2000000000), "horizon expired"` in the examples, and the
number of transitions is a declared lifetime that each action consumes. At the
proof level an exercised claim is an obligation with an identity, and the
mandatory claims require the successor state to be a compliant successor of a
compliant history, so an expiry that arrives at a state without the duty is not
a valid transition from a state that had it.

What is not forbidden: the language does not make the reference honest, and a
price bound or a time-weighted observation reduces what a specified attacker
can do without removing the economic risk. Margin adequacy under a gap is a
solvency predicate over selected prices and assumptions, not a theorem. DA11
and DA12 both wait on pinned protocol fixtures before the settlement
convention and the exercise calendar can be frozen.

**Reference sources.** Werner §3.5; Gogol §IV-B3.

Werner §3.5 is the reference for what a derivative protocol is and for the
distinction between atomic and non-atomic exposure that makes funding and
margin different from a swap. Gogol §IV-B3 is the source for the perpetual and
option mechanisms and their stakeholder risks. Neither gives an exercise
calendar precise enough to fix DA12, which is why the target carries a fixture
gap.

---

## F4 — Consensus-position claims

**Function.** Claims whose value derives from participation in consensus:
staking, liquid staking, restaking and shared security.

The obligation runs in both directions. A staker delegates capital to an
operator who performs validator duties, and the protocol owes rewards for
correct performance and takes a slashing penalty for incorrect performance. The
staker's claim is worth the delegated stake plus rewards minus slashing, and it
is worth that only after a delay, because an exit is requested and then
completed rather than executed. The operator's authority persists through the
delegation, and the capital keeps its exposure through the exit queue.

**Boundary.** Distinguish slashable allocation from ordinary leveraged lending
or a token-lock reward. Restaking commits capital to *additional* security
duties; that is a different exposure from lending the same capital.

Token locking without security duties is not consensus staking, and a token
that pays a reward for being locked is a yield product with no slashing
exposure. Restaking is the opposite case. The same capital is pledged to a
second set of duties, and a fault in either set can slash it, so the exposure
is the union of the duties and not the sum of the rewards.

**Facet profile.** Execution: two-phase — request then completion — with a
protocol-imposed delay · Settlement: delayed and sometimes queued · Custody:
delegated to an operator, with operator authority persisting · Legal dependence:
none · Collateral and solvency: slashing is loss allocation, not liquidation ·
Oracles: reward and penalty accounting is internal but externally observable ·
Authorization: delegation is a mandate with its own scope · Price discovery:
consumed.

Execution is two-phase because the protocol will not release stake on demand,
and the delay is imposed by consensus rather than chosen by the contract.
Custody is delegated because the operator performs the duties, and the
operator's authority persists until the exit completes. Legal dependence is
genuinely none because the reward and the penalty are computed and applied by
the protocol itself, with no custodian and no court. Collateral and solvency is
loss allocation rather than liquidation: nobody sells the stake to cover a
debt, the protocol takes a share of it and the loss is allocated across the
claims. Oracles are internal because the protocol knows what it rewarded and
slashed, and externally observable because a liquid staking token has to read
those facts to update its entitlement.

**Action targets.**

- **DA14 — stake / account rewards / slash.**
  Requirement: share-rate or rebase accounting; loss allocation.
  *Distinguishing test:* **the same nominal token balance can have a changing
  entitlement.** Rebasing (updating quantities) and a fixed share balance with a
  changing conversion rate are different mechanisms with different integration
  consequences.

  A rebasing token changes every holder's balance as rewards arrive and
  targets a fixed one-to-one value against the underlying. A reward-bearing
  token keeps every balance fixed and changes the conversion rate, so the same
  balance is worth more underlying tomorrow than today. A dual token separates
  principal and reward into two balances. The surveyed literature reports
  rebasing as the less compatible of these with other protocols, and the reason
  is the test. An integration that caches a balance, or reads a transfer delta
  as a change of entitlement, is correct for one mechanism and wrong for the
  other. The plausible wrong implementation treats a rebasing balance as a
  fixed claim and misses every reward, or treats a fixed share balance as a
  quantity and misses every change in the rate. The reader would see a lending
  market whose collateral value never moves while the token it holds is being
  slashed.

- **DA15 — request unstake / claim exit.**
  Requirement: pending exit identity, custody and delayed completion.
  *Distinguishing test:* **an exit request is not immediate token delivery.**

  Between the request and the completion the stake is in a third state. It is
  still slashable, it is no longer earning, and it is not yet transferable. A
  plausible wrong implementation has no such state, and it either delivers the
  tokens at request time and hopes the protocol agrees later, or records
  nothing until completion and cannot say what the staker is owed in between.
  The reader would see an exit that was slashed after the staker was shown as
  paid, or a staker with no representable claim for the length of the queue.

**What goes wrong.** Integrations treat a rebasing balance as a fixed claim, or
a fixed share balance as if quantity were entitlement. Exit requests are modelled
as instantaneous, so the pending state — during which the capital is still
slashable but no longer earning, and the claim is not yet transferable — has no
representation at all.

Both failures come from reading a balance as an entitlement. In the accounting
case the balance is the wrong number. In the exit case there is no balance to
read, because the capital is between two accounts and the model has only
accounts.

**What Moriarty does.** The pending state is an object with an identity, not a
gap between two states. This is the same machinery as DA18's asynchronous
redemption lifecycle: a request creates a durable identified obligation, and
partial or delayed completion consumes it without resetting it. Affine authority
applies — a partially completed exit consumes its authorization and the residual
cannot grow.

Slashing is loss allocation against claims, and the four mandatory proof claims
require that the resulting state be a compliant successor of a compliant history,
so loss cannot be applied retroactively to a history that did not carry it.

At the type level a share of a staking claim and an amount of the underlying
are different units, and the conversion between them is an operation with a
declared rounding rather than a cast, so a share balance cannot be spent as if
it were the underlying. At the guard level the exit request is an obligation
record that a completion must find, consume by name and not recreate, and a
completion that arrives without a matching request has nothing to complete. At
the proof level a slash applied to a claim is a transition, and the claim it
reduces must be present in the predecessor history.

What is not forbidden: the language does not decide the queue length, does not
know whether the operator will be slashed, and cannot make a delegated operator
honest. The surveyed sources identify depeg alongside slashing as a risk of
liquid staking tokens, and a peg is an economic objective the language does not
prove. DA14 waits on a pinned protocol fixture and DA15 on a primary lifecycle
source.

**Reference sources.** Gogol §IV-A1; pending-workflow analysis for exit
lifecycles. DA15 is one of the targets with an identified primary-lifecycle
source gap.

Gogol §IV-A1 and its §3.6 are the sources for the three token designs, for the
observation that rebasing is less compatible with other protocols, and for the
validator-admission distinctions. The pending-workflow analysis is the
repository's own treatment of request-then-complete lifecycles and is the
reason DA15 shares machinery with DA18 and DA23.

---

## F5 — Tokenized off-chain claims

**Function.** On-chain representations of claims that are ultimately enforced
off-chain: real-world assets, attested claims, custodial receipts.

The obligation is an issuer's promise to deliver something that a custodian
holds where the chain cannot see. The token holder has a claim against the
issuer, the issuer's claim is against the custodian, and the custodian's
performance is a matter of contract and law. For the claim to be worth anything
the custodian has to hold the asset, the issuer has to be solvent, the
attestation has to be true, and a court somewhere has to be willing to enforce
the redemption right. None of those facts is on the chain, and the token is
evidence of a deposit rather than possession of one.

**Boundary.** A representation does not establish a claim's economic substance.
A receipt is evidence of a deposit; the redemption right is a separate
specification. Identity and compliance controls (ERC-3643, ERC-7943) describe
technical transfer policy and do not constitute legal conclusions.

**Facet profile.** Execution: on-chain action, off-chain performance ·
Settlement: split across domains with different finality ·
Custody: an off-chain custodian holds the underlying — this is the defining
facet · **Legal dependence: maximal.** The claim is only as good as its
enforceability · Collateral and solvency: issuer solvency is the risk ·
Oracles: attestation is the oracle, and the attestor can lie ·
Authorization: issuance authority is institutional, not cryptographic ·
Price discovery: external, often with a valuation-time/publication-time gap.

Execution splits because the burn is on-chain and the wire is not, and
settlement splits across domains because a block is final on one schedule and
a bank transfer on another, and the second can be reversed after the first
cannot. Legal dependence is maximal because every other facet reduces to it:
the custodian's duty, the issuer's solvency and the redemption right are
enforced off-chain or not at all. Oracles are attestations, and an attestation
is a signed statement by a party who could sign something false. Authorization
is institutional because the right to issue comes from a legal arrangement and
not from a key. Price discovery is external and lagged, because a valuation is
computed at one time and published at another, and the gap is where a
redemption can be mispriced.

**Action targets.**

- **DA16 — issue / redeem external claim.**
  Requirement: attested claim with custody and legal assumptions.
  *Distinguishing test:* **missing external settlement evidence cannot discharge
  the duty.** An on-chain burn is not proof that anyone off-chain paid.

  A redemption burns the token and creates a duty on the issuer to deliver the
  underlying. The duty is discharged when authenticated evidence of the
  off-chain delivery arrives, and not before. The plausible wrong
  implementation treats the burn as the settlement, because the burn is the
  only event the contract can see, and it has no state for a token that is gone
  and a wire that has not arrived. A second accepts a bridge message or a local
  proof as evidence of foreign finality. A third treats a witness that is
  unavailable as a commitment that is invalid, which is the reverse error. The
  reader would see redemptions reported complete while the holders wait, and a
  ledger with no record of who is still owed.

**What goes wrong.** The on-chain leg completes and is treated as settlement. The
system has no way to represent "the token was burned and the wire has not
arrived", so it represents it as done. Valuation time, publication time,
invalidation and redemption liquidity are collapsed into one NAV number.

The valuation collapse has its own mechanism. A fund token has an accounting
value, a redemption value, a market value and a stressed liquidation value, and
they differ. A claim can be accounted for correctly and still be illiquid,
temporarily unredeemable or unfit as collateral. A system that carries one
number carries whichever was easiest to publish, and every consumer reads it as
the one they needed.

**What Moriarty does.** This is the family where the honest architecture matters
most, and Moriarty's answer is explicit rather than optimistic: every oracle and
external effect has a **named capability and an assurance boundary**, recorded in
the deployable manifest. A typed observation carries source, feed, unit,
timestamp, freshness, sequence, bounds and fallback — and the residual risk is
stated next to it: **the source can still lie.**

External contract calls are excluded from V0 entirely. Later they require a
capability manifest, an allowlist and an effect summary, with separate audit
evidence, on the stated principle that **external behavior cannot inherit
Moriarty guarantees.** A duty that depends on off-chain performance stays a duty
until evidence of that performance exists.

At the type level an observation is a typed record and not a number, and a
valuation observation carries its purpose, so an accounting value cannot be
passed where a redemption value was required. At the guard level the
redemption duty is an obligation record, and the action that discharges it must
present the selected authenticated settlement observation, so a burn without
evidence leaves the duty outstanding. At the proof level the capability that
supplied the evidence is named in the manifest, and the assurance boundary next
to it says what the proof does not cover.

What is not forbidden is the point of the family. The language cannot make the
custodian hold the asset, cannot make the issuer solvent and cannot make the
attestor truthful. It can make each of those an assumption with a name, and it
can refuse to discharge a duty on the strength of an assumption alone. DA16
waits on a primary lifecycle source.

**Reference sources.** Gogol §§III-B, IV-A2; ERC-3643 and ERC-7943 for transfer
control; ERC-8330 for subject-linked NAV observation boundaries; Centrifuge
protocol source for cross-domain fund infrastructure.

Gogol §III-B and §IV-A2 supply asset provenance and the custodial claim
mechanisms. ERC-3643 and ERC-7943 are the transfer-control standards, and a
reader goes to them for the technical policy an identity-gated token enforces,
not for any conclusion about what the token is legally worth. ERC-8330 is the
reference for an NAV observation linked to a subject and bounded in time. The
Centrifuge protocol source is the worked example of a fund whose accounting
crosses domains.

---

## F6 — Delegated asset management

**Function.** Someone else allocates your capital. Vaults, yield strategies,
allocators, standardized accounting entry points.

A depositor hands assets to a vault and receives shares. The vault owes the
depositor redemption of their proportional share of whatever the vault holds,
at the vault's accounting rate on the day they redeem, and the manager holds a
mandate to move the assets within a policy the depositor agreed to. For the
claim to be worth anything the accounting has to be honest, the rate has to be
computed the same way for everyone, the manager has to stay inside the mandate,
and the assets have to be recoverable when a redemption is asked for. A share
is a claim on the accounting, and the accounting is a claim on the assets, and
each arrow is a separate thing that can fail.

**Boundary.** "Vault" is the most overloaded word in the vocabulary — it can mean
a debt position, a custody container, an investment vehicle, a strategy adapter
or a standardized accounting entry point. Always qualify it. A share is a unit
participating in portfolio accounting or a priority-defined claim; it is not
automatically legal equity.

**Facet profile.** Execution: synchronous deposit/withdraw, or an asynchronous
request lifecycle · Settlement: immediate or queued · Custody: the vault holds;
the strategy may re-delegate, producing nested exposure · Legal dependence:
varies with the underlying · Collateral and solvency: the vault's obligations to
depositors can exceed realizable value under stress · Oracles: valuation is the
oracle problem in disguise · Authorization: the manager's mandate has scope and
limits, and the depositor authorized a *policy*, not each trade ·
Price discovery: NAV is a valuation, not a realizable price.

Execution is synchronous or a request lifecycle because a vault whose assets
are liquid can convert on demand and a vault whose assets are lent out or held
off-chain cannot. Custody is with the vault and then with whatever the strategy
delegated to, and two strategies that delegate to one underlying position hold
one exposure and two labels. Collateral and solvency is a stress question
because the accounting can say the vault owes more than it could realize by
selling. Oracles are the valuation problem under another name, since a NAV is
an observation of what the assets are worth and somebody has to supply it.
Authorization is a mandate with a scope because the depositor agreed to a
policy and not to each trade under it.

**Action targets.**

- **DA17 — deposit / mint / withdraw / redeem vault shares.**
  Requirement: asset/share conversions with **method-specific** rounding.
  *Distinguishing test:* fees, rounding direction, initial donation and zero
  shares. Each conversion method carries its own rounding direction, and the
  correct direction differs between them.

  The retained ERC-4626 text fixes the directions. `convertToShares` and
  `convertToAssets` are ideal, caller-independent conversions and both round
  down. A deposit fixes the assets coming in and computes the shares going out,
  and `previewDeposit` must return no more shares than the deposit will mint.
  A mint fixes the shares going out and computes the assets coming in, and
  `previewMint` must return no fewer assets than the mint will require. A
  withdraw fixes the assets going out and computes the shares burned, and
  `previewWithdraw` must return no fewer shares than the withdraw will burn. A
  redeem fixes the shares burned and computes the assets going out, and
  `previewRedeem` must return no more assets than the redeem will pay.

  The rule behind the directions is one rule: round against the caller. When
  the caller receives shares, round the shares down. When the caller pays
  assets for a fixed share count, round the assets up. When the caller pays
  shares for a fixed asset amount, round the shares up. When the caller
  receives assets, round the assets down. Each direction leaves the remainder
  in the vault, where it backs every other holder.

  The plausible wrong implementation applies floor everywhere, because floor
  is what integer division does. Deposit and redeem are then correct and mint
  and withdraw are wrong. A mint that floors the assets required lets the
  caller pay up to one unit less than the shares are worth, and a withdraw that
  floors the shares burned lets the caller keep up to one share's worth more
  than they took out. Each is a leak of at most one unit per call, and a caller
  who can choose amounts can choose the ones whose remainder is largest and
  repeat. The reader would see a vault whose assets per share fall slowly and
  steadily with no loss in the strategy.

  Fees are the last item, and the standard is explicit that a preview includes
  them while an ideal conversion excludes them, so a fee-aware quote and an
  ideal ratio are different numbers with different contracts.

- **DA18 — request / fulfill / claim asynchronous redemption.**
  Requirement: Pending → Claimable → Claimed, with residual request amount.
  *Distinguishing test:* partial claim and changed exchange rate; **double claim
  rejects.**

  An asynchronous redemption is a request that an operator later fulfils and
  the holder later claims, and the standard is explicit that the exchange rate
  can change between the request and the claim. A request for 100 shares that
  is fulfilled in part leaves a residual request for the rest, with the same
  owner, the same controller and its own entitlement. The plausible wrong
  implementation has a request with no residual amount, so a partial
  fulfilment either closes the whole request or leaves it open in full, and a
  second claim against a request already paid has nothing to stop it. A second
  wrong implementation answers the synchronous preview methods with a number,
  when the standard requires them to revert for an asynchronous flow, because
  there is no same-transaction rate to quote. The reader would see a holder
  paid twice, or a holder whose unfilled remainder vanished.

- **DA19 — allocate / harvest / reinvest / unwind / rebalance.**
  Requirement: a bounded workflow of trades, debt, shares and fees.
  *Distinguishing test:* **losses and debt persist through unwind**; liquidity
  shortage.

  A strategy withdraws a specified upstream claim, accounts for fees and
  slippage, and deposits the authorized amount downstream, and every leg is a
  trade, a debt or a share operation in some other family. Unwinding runs it
  backwards. The plausible wrong implementation reports the unwind complete
  when the position is closed while the debt it borrowed against is still
  outstanding, or reports partial leg success as atomic success, or expands a
  recursive strategy past its declared bound. A repeated sequence of small
  roundings and fees can inflate a downstream ratio while every single
  operation passes its local check, and an invariant that says the share price
  only rises is false the moment the strategy is allowed to lose. The reader
  would see a vault that reports itself flat and owes money.

**What goes wrong.** The donation attack: a direct transfer inflates assets
without minting shares, and the first depositor's rounding does the rest. Rounding
in the depositor's favour on one method and the vault's on another, applied
consistently, drains one side. An asynchronous claim processed twice because the
request had no residual amount. An unwind that reports the position closed while
its debt persists.

Walk the donation through the first deposit. A new vault has zero assets and
zero shares. The attacker deposits 1 asset and, under the usual first-deposit
rule, receives 1 share, so the vault holds 1 asset against 1 share. The
attacker then transfers 100 assets directly to the vault's address without
calling deposit. No shares are minted, because no deposit happened, and the
vault now holds 101 assets against 1 share. A victim deposits 100 assets. The
shares owed are 100 multiplied by 1 divided by 101, and the floor of that is 0.
The victim receives no shares. The vault holds 201 assets against 1 share, and
the attacker's one share redeems for all of it. The attacker paid 101 in and
takes 201 out, and every operation obeyed the rounding rule the standard
prescribes. The rounding direction was correct, and the zero-share outcome is
what the direction produces once the ratio has been made large enough. The
defence is not a different rounding. It is a minimum-shares bound on the
depositor's intent, so that a deposit that would mint zero rejects before any
asset moves, and a first-deposit rule the attacker cannot control.

**What Moriarty does.** Direct-transfer and rounding accounting are complete
rather than inferred — the `policy` block requires each conversion to state its
rounding direction and remainder disposition, so "method-specific rounding" is a
declared property that can be checked rather than a convention. Request duties
are persistent identified obligations, so Pending, Claimable and Claimed are
distinct states and a residual request amount survives a partial claim; a second
claim against a consumed allocation has nothing to consume.

Valuation roles are separated: conversion, preview, limit and execution semantics
are distinct, so a preview cannot be mistaken for an authorization. Nested
exposure is visible because each delegation is a named capability with its own
assurance boundary.

At the type level shares and assets are different units and the conversion
between them is an operation with a declared policy, and the successor type
requirements name `Shares<Vault>` as distinct from `Amount<Asset>`. A preview
is a query, a limit is a query, and an execution is an action, and a nonzero
preview beside a zero limit is a consistent state in which the deposit rejects.
At the guard level the zero-share outcome is caught by the intent rather than
by the vault: a signed outcome intent binds the minimum net shares the depositor
will accept, and a deposit that would deliver fewer rejects under
IntentRefinement before any effect is accepted. At the policy level each of
deposit, mint, withdraw and redeem is a conversion with its own `rounding`
line, so floor cannot be applied to all of them by default, and `rounding none`
on a path that divides is a static rejection. At the proof level the mandatory
claims bind the effects of an unwind to the debt it leaves, so a strategy cannot
report closure while an obligation record survives.

What is not forbidden: the language does not prove that the strategy is
profitable, that the underlying is liquid when a redemption arrives, or that
two wrappers over one position are independently backed. Computing an
accounting net exposure does not prove redemption liquidity. Cancellation of an
asynchronous request is not supplied by the standard and is not assumed. DA17
and DA18 are source-defined targets whose implementation is open, and DA19
waits on a pinned strategy fixture.

**Reference sources.** ERC-4626 methods and security considerations; ERC-7540
request lifecycle and `requestRedeem`; ERC-7575 for external share-token
topology; yield literature §§III-A Fig. 2, III-B, V-B; Morpho MetaMorpho source
for allocator→market dependencies.

ERC-4626 is the normative source for the conversion methods, their rounding
directions and the preview inequalities, and its security considerations are
where the donation case is discussed. ERC-7540 is the source for the request
lifecycle, for the rule that the synchronous previews must revert, and for the
statement that the rate can change between request and claim. ERC-7575 covers
the case where the share token lives outside the vault contract. The yield
survey describes the strategy phases and the liquidity and composition risks,
and it does not specify a complete redemption queue. The MetaMorpho source is
the worked example of an allocator whose dependencies on markets have to be
bounded and visible.

---

## P — Prediction markets and event-contingent claims

**Function.** Claims indexed by the outcome of an event, resolved by evidence.

The obligation is a promise to pay the holder of the winning outcome one unit
of collateral per claim once the event is known. Collateral is deposited
against the full set of outcomes, a complete set of outcome claims is worth
exactly the collateral behind it whatever happens, and a single outcome claim
is worth the collateral if that outcome occurs and nothing otherwise. For the
claim to be worth anything the collateral has to be there in full, the set of
outcomes has to be complete so that exactly one can win, and the resolver has
to report the truth.

**Boundary.** Held separate from F3 because resolution is evidentiary rather
than price-mechanical: the question is not "what is the number" but "what
happened, and who says so". DA12 sits in both families.

**Facet profile.** Execution: mint, trade, resolve · Settlement: after
resolution, all at once · Custody: collateral held against the full outcome set ·
Legal dependence: varies by jurisdiction and event type · Collateral and
solvency: a complete set must always be fully collateralized ·
**Oracles: the resolution source is the entire trust model** ·
Authorization: who may resolve, and under what evidence, is the critical mandate ·
Price discovery: produces probability estimates.

Settlement is all at once because nothing is owed until the event is known and
everything is owed the moment it is. Custody is against the full set because
the collateral backs every outcome until one wins, and a merge must always be
able to return it. Legal dependence varies because the enforceability of a
claim on an event depends on what the event is and where the holder lives.
Oracles are the whole trust model because the resolution is the payoff and
there is no price mechanism to fall back on. Price discovery produces a
probability, because the price of an outcome claim in collateral units is the
market's estimate that the outcome occurs.

**Action targets.**

- **DA13 — split / merge / resolve event claims.**
  Requirement: outcome-indexed claims and resolution evidence.
  *Distinguishing test:* **no duplicate winning claim; invalid resolution
  rejects.**

  A split takes one unit of collateral and issues one claim on each outcome. A
  merge takes one claim on each outcome and returns one unit of collateral.
  Resolution declares one outcome the winner and makes its claims redeemable.
  The conservation rule is that across all outcomes the claims outstanding
  never exceed the collateral held, and the plausible wrong implementation
  breaks it by treating split and merge as token mints and burns. A merge that
  accepts a partial set returns collateral for claims still outstanding. A
  resolution that can be submitted twice, or that accepts a second answer after
  the first, pays two winners. A resolution from a source whose authority was
  never modelled has no rejection path, so the first answer wins. The reader
  would see a market that paid out more collateral than it held.

- **DA12 — write / exercise / expire contingent claim.** *(shared with F3)*
  *Distinguishing test:* expiry does not erase an already exercised payment duty.

  In a prediction market the exercise is the redemption of a winning claim
  after resolution, and the payment is the collateral it returns. A holder who
  redeems on the last permitted day has created a payment duty, and a market
  that sweeps unredeemed claims at expiry must not sweep that duty with them.
  The wrong implementation and the reader's view are as in the F3 tab.

**What goes wrong.** Split and merge are treated as token operations rather than
as conservation-preserving transformations of a claim set, so a merge can
duplicate value. Resolution is accepted from a source whose authority was never
modelled. An invalid or contested resolution has no rejection path, so the first
answer wins.

The mechanism is that the market has a token contract and a resolver and
nothing between them. The token contract knows balances and the resolver knows
an answer. Nobody holds the claim set as a set, so nothing can check that a
merge consumed a complete one, and nobody holds the resolver's authority as a
typed fact, so nothing can refuse an answer that arrived through the wrong
door.

**What Moriarty does.** Split and merge are exactly the composition operators
DA24 names, and the constraint is stated as a conservation property: **split
partitions work and claims; join cannot duplicate resource.** That is checkable
against Core's value-conservation property rather than trusted to the token
contract.

Resolution is a typed observation with a named capability — the resolver's
authority is part of the agreement, and evidence that does not meet the declared
provenance, freshness and domain requirements rejects. The security position is
stated plainly: **a valid oracle signature does not establish economic truth.**
Conditional-token split/merge is identified as work belonging to a later
composition layer over signed capabilities, so the site should present it as
architecture rather than as a shipped feature.

At the type level an outcome claim is indexed by its outcome and its event, so
a claim on one outcome cannot be presented as a claim on another. At the guard
level a merge is a join, and a join that arrives without every partition it
was split into has nothing to join. At the proof level Core's conservation of
value is the property a split and a merge are checked against, and a resolution
is a transition that must find the resolver's capability in the manifest.

What is not forbidden: the language does not know what happened. A resolver
with a valid capability can still report a falsehood, and the residual risk is
stated next to the capability. The repository's crosswalk marks its
prediction-market rows as conditional on a specified split and merge
primitive, and that primitive is specified rather than built. DA13 waits on a
primary lifecycle source.

**Reference sources.** DeFi report contingent-claim targets; Werner §3.5. DA13
carries an identified primary-lifecycle-source gap.

The DeFi report's contingent-claim targets are the source of the split, merge
and resolve action set and of the conservation framing. Werner §3.5 is the
shared source with F3 for the contingent-claim mechanism. Neither supplies a
pinned resolution lifecycle, which is the gap.

---

## ⊥ — Cross-cutting: what makes the other families composable

Not a leftover tab. These targets are the machinery every other category
depends on, and they are the reason the model is a classification rather than a
list of products.

Every family above consumes an observation, acts under an authorization, lives
under parameters that somebody can change, and at some point waits on a message
or hands work to another agreement. A lending market needs a price it did not
compute. A swap needs a signed minimum. A vault needs a manager whose mandate
was granted before the trade. A staking exit and a cross-chain redemption are
both a pending message with a refund duty. None of those is a financial product,
and each is where a financial product fails when it is composed with another.

**Action targets.**

- **DA20 — observe price / time / external event.**
  Requirement: typed observations with provenance, freshness and domain.
  *Distinguishing test:* stale or unauthorized evidence rejects.

  An observation is a signed record with a source, a feed, a unit, a
  timestamp, a freshness window, a sequence number, bounds and a fallback. The
  plausible wrong implementation reads a number from a contract and uses it. A
  validly signed spot price that an attacker can move inside one transaction is
  then treated as a guarantee of economic truth, and the model has no place to
  say that the price is manipulable. The correct implementation exposes the
  assumption, and the attack can remain economically possible while being
  visible. The reader of the wrong version would see a liquidation at a price
  that existed for one block.

- **DA21 — authorize exact plan / refine outcome intent.**
  Requirement: gross debit, net receipt, recipients, calls, new liabilities.
  *Distinguishing test:* **a refund cannot restore gross capacity; fees count
  against the net goal.**

  A signed authorization is a budget of gross spending by asset across every
  recipient, a list of permitted recipients, a net goal, a fee cap, and a list
  of liabilities the principal will accept. Partial fills consume it. The
  plausible wrong implementation nets: a refund arrives and the allowance goes
  back up, so a route that spends, refunds and spends again has spent the
  budget twice against one signature. A second lets two reordered fills each
  read the same residual record. A third compares final balances and misses an
  intermediate call or a new liability that left them unchanged. The reader
  would see a principal whose one signature moved more than it authorized, with
  every balance ending where it started.

- **DA22 — change parameters / pause / migrate.**
  Requirement: bounded administrative action under a fixed claim policy.
  *Distinguishing test:* cannot downgrade claims or reset work; affects existing
  positions.

  A parameter change is a transition like any other, applied under the
  authority version that was bound when the affected positions were created.
  The plausible wrong implementation lets a new governance setting reinterpret
  a signed obligation retroactively, or treats temporarily borrowed voting
  power as durable consent, or assumes the administrative key cannot be
  compromised. The reader would see a debt whose terms changed after it was
  signed.

- **DA23 — send / receive / refund pending message.**
  Requirement: bounded pending commitments and finality evidence.
  *Distinguishing test:* delayed or duplicate delivery; explicit refund duty.

  A message creates a bounded pending commitment and a refund duty, and it is
  discharged by authenticated evidence of delivery. The plausible wrong
  implementation treats a local proof or a bridge message as proof of foreign
  finality, or has no state for a message sent and not yet delivered, or
  delivers a duplicate because the second arrival found nothing to check
  against. The reader would see a cross-chain swap whose internal batch
  succeeded and whose withdrawal was rolled back, with the model reporting
  success.

- **DA24 — sequence / parallel / interleave / synchronize / message.**
  Requirement: operator-specific authority, duties, conflicts and fan-in.
  *Distinguishing test:* **split partitions work and claims; join cannot
  duplicate resource.**

  Two calls on shared reserves compose under a declared observation order and
  a declared conflict rule. The plausible wrong implementation checks each call
  against the same pre-state, accepts both, and applies effects that were only
  individually valid. A second inspects an intermediate state through a
  callback the composition did not declare. A third splits work between two
  branches and lets each carry the whole budget, so the join receives more than
  was split. The reader would see two transactions that each passed and a
  reserve that is now negative.

**The composition operators.** DA24 names them: **sequence, parallel,
interleave, synchronize** and **message**. Each carries its own authority rules,
its own duty propagation, its own conflict semantics and its own behavior when
work fans back in.

The work budget is the concrete resource each operator has to partition. Every
action costs exactly one unit of remaining work, the closure reserve is not
ordinary work, and a candidate that would spend more work than it holds rejects
before any action runs.

**The measured result that motivates the work.** Of 1,830 eligible protocol
pairs, 1,645 compose cleanly. The rest fail, and almost all of those failures
cross a category boundary. Within a category the failure rate is 2.10%. Across
categories it is 10.79%, which is **5.14 times** as often.

An eligible pair is a pair of protocols from the corpus for which the
composition algebra can be evaluated, under explicit eligibility rules so that
a later run cannot reach the headline number by a different procedure. A clean
composition is one the algebra accepts without a conflict between the two
protocols' obligations. The denominators are 143 pairs within a category and
1,687 pairs across categories, and the failures divide as 3 within and 182
across.

The reason the rate rises at the boundary is the reason for the model. Inside a
family the two protocols make the same assumptions about settlement timing,
custody, authority and where the price comes from, because the family is
defined by those assumptions. Across a boundary the output of one protocol
becomes the input of another that assumed something different about it: a
vault share is used as collateral by a lender that read its accounting value as
a market value, or a staking claim is borrowed against by a market that read
its balance as a fixed entitlement, or a pending redemption is counted as cash.
Each of those is a facet value that one side assumed and the other did not
check. Facet values are what a type system can carry and an operational
semantics can police, so the measured result is evidence for the whole project
rather than a coincidence about a corpus: the failures concentrate where the
typed distinctions are dropped. The repository records high confidence for the
counts and rates and medium confidence for the taxonomic interpretation, and
the site should carry the same qualification.

Composition across financial categories is where this breaks, and composition is
what a type system and an operational semantics can police. An earlier informal
claim of sixty times was checked against the corpus and is wrong.

**Governance is a financial action.** DA22 is in this tab deliberately. A
parameter change affects existing positions; it is a bounded administrative
action under a fixed claim policy, and it cannot downgrade claims or reset
accrued work. Treating governance as outside the financial semantics is how
positions get changed retroactively.

**Messaging is where cross-chain lives.** DA23's pending commitments and finality
evidence are the same machinery as F4's exit requests and F6's asynchronous
redemptions: a durable identified obligation with an explicit refund duty and a
delivery that may be delayed or duplicated. The cross-chain material in
CONTENT-SPEC §7 belongs here — per-layer atomicity, route-specific custody
manifests, and the bridge-rollback trace that any adequate model must be able to
represent and reject.

**What this tab does and does not establish.** The compiler can reject unit
errors, unknown effects and unbound authority. Core and its executable
semantics can define accounting, valid transition order, residual-duty
preservation and finite work. The proof relation can bind those predicates to
the concrete statement, and ledger acceptance can consume the correct
predecessor and residual authority exactly once. None of those steps
establishes market liquidity, oracle truth, profitable strategy selection,
honest governance, external custody, inclusion fairness or future settlement.
DA20 is implemented as simulated observations only, DA21 as local restricted
profiles only, DA24 is specified and not built, and DA22 and DA23 wait on a
pinned policy fixture and a primary lifecycle source.

---

## Acceptance criteria

These are constraints on the build, not sentences for the page. The site must
satisfy them without ever reporting that it does.

- Every tab renders, and every block in every tab is populated. No empty states.
- Every action target from DA01 to DA24 appears. DA12 appears in two tabs and is
  marked shared.
- Every action target shows its semantic requirement and its distinguishing test.
- Every distinguishing test is taught: the plausible wrong implementation, the
  diverging case and the observable consequence are each present.
- Family identifiers and facet names are never abbreviated away or reordered.
- Every composition operator named by DA24 is listed.
- Every facet is answered in every tab, including where the honest answer is
  "none" or "not applicable", and every "none" carries its reason.
- Category identifiers appear as classification labels and never as Moriarty
  source syntax.
- Every worked figure is one the repository carries. The swap output is 19,743;
  the loan interest is 33,972,602; the composition denominators are 143 and
  1,687.
- No tally is displayed to the reader. Coverage is shown by the surface. See
  [`VOICE.md`](VOICE.md).
