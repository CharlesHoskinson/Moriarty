# DeFi category tabs — complete specification

The site's category section is a **tabbed explainer with one tab per DeFi
category**. Completeness is a hard requirement: every economic family is
present, every one of the 24 action targets appears in exactly one tab, and no
tab is a stub. A category with less repository evidence gets a shorter tab, not
a missing one.

**Eight tabs.** Seven economic families plus one cross-cutting tab that carries
the five orthogonal action targets. Coverage check: DA01–DA24, each exactly once.

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

DA12 (`write / exercise / expire contingent claim`) is tagged `F3;P` in the
source matrix and legitimately appears in both the Derivatives and Prediction
Markets tabs. Show it in both; mark it as shared so the count still reads 24.

---

## Tab structure (identical for all eight)

Every tab renders the same six blocks. This regularity is what makes the section
feel complete rather than assembled.

1. **What this category is** — the financial function, and the boundary that
   excludes neighbours.
2. **Facet profile** — where this family typically sits on the eight mandatory
   facets. This is the part that shows the model is a classification and not a
   list.
3. **Action targets** — each with its semantic requirement and its
   distinguishing test. The distinguishing test is the emphasized element.
4. **What goes wrong** — the concrete failure this category produces in
   practice, stated as a mechanism rather than an anecdote.
5. **What Moriarty does about it** — the specific language construct, type,
   policy or authority rule that addresses it. Real `.mori` where it exists.
6. **Reference sources** — the pinned standards and papers behind the targets.

---

## F1 — Exchange and price discovery

**Function.** Convert one asset into another and, in doing so, produce a price.
Includes constant-function market makers, order books, concentrated-liquidity
positions, aggregators and routers.

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

**Action targets.**

- **DA01 — swap exact input / exact output.**
  Requirement: asset-indexed exchange with fee and slippage accounting.
  *Distinguishing test:* rounding; reserve safety; net minimum.
- **DA02 — provide / remove liquidity.**
  Requirement: share mint/burn and reserve contributions.
  *Distinguishing test:* proportional entitlement, and the donation and
  zero-supply boundaries — the first depositor and the direct-transfer donation
  are where share maths breaks.
- **DA03 — open / adjust / close liquidity position.**
  Requirement: bounded position identity and range.
  *Distinguishing test:* fee allocation and finite tick/range traversal.

**What goes wrong.** Integer division decides who keeps the remainder. A swap
that rounds the wrong way, or a share calculation whose zero-supply case is
unguarded, transfers value silently and legally. Reserve safety is not implied
by the constant-product identity: an output that empties a reserve satisfies the
formula and destroys the pool.

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

**Reference sources.** AMM literature §§2.3.2, 3.1, 3.3, 4; Uniswap V4 core for
singleton accounting and hook-modified mechanism assumptions.

---

## F2 — Credit and collateralized debt

The largest tab: seven action targets. Lending markets, CDPs, debt-backed
stablecoins and flash loans.

**Function.** Create an obligation to repay, usually secured, and manage it
through accrual, partial performance, default and discharge.

**Boundary.** *Nominal debt is not a transfer.* This distinction is the reason
F2 is the family Moriarty is architecturally organized around. A credit system
that models debt as a balance rather than an obligation cannot express partial
repayment correctly.

**Facet profile.** Execution: on-chain, often multi-step · Settlement:
same-domain, but discharge and transfer are separate events · Custody:
collateral is encumbered, not surrendered · Legal dependence: none on-chain;
material for RWA-collateralized variants · Collateral and solvency: the defining
facet · Oracles: required — liquidation depends on external price ·
Authorization: borrower authority to draw, liquidator authority to seize, and
they are different mandates · Price discovery: consumed, not produced.

**Action targets.**

- **DA04 — supply / redeem lending claims.**
  Requirement: claim shares and liquidity-constrained withdrawal.
  *Distinguishing test:* insufficient pool liquidity must reject **without
  erasing the claim.** Failure to withdraw is not loss of entitlement.
- **DA05 — post / release collateral.**
  Requirement: encumbrance and debt-dependent release.
  *Distinguishing test:* release cannot violate the collateral rule.
- **DA06 — borrow / accrue / repay.**
  Requirement: nominal debt distinct from transfers; rate and time arithmetic.
  *Distinguishing test:* **partial repayment preserves principal and interest
  allocation.** This is the project's flagship case.
- **DA07 — liquidate / recognize default.**
  Requirement: authorized seizure, loss and residual debt allocation.
  *Distinguishing test:* partial liquidation and close factor; maturity before
  forfeiture.
- **DA08 — flash borrow / repay atomically.**
  Requirement: atomic multi-leg settlement with fee.
  *Distinguishing test:* every loan repaid within the same atomic transaction.
  Flash borrowing is a capability with legitimate uses, not a vulnerability.
- **DA09 — refinance / novate / capitalize.**
  Requirement: a workflow over old debt, new debt, collateral and signed
  liability authority.
  *Distinguishing test:* reject refinance without old-debt discharge;
  capitalization changes liability.
- **DA10 — issue / burn debt-backed stablecoin.**
  Requirement: issuance authority plus debt and collateral.
  *Distinguishing test:* burn amount and released collateral obey the specified
  debt rule.

**What goes wrong.** A repayment reduces a balance without discharging the right
obligation. Interest and principal are allocated by whichever subtraction the
code happened to perform first. A withdrawal that cannot be funded erases the
claim instead of rejecting. A refinance issues new debt without discharging old.

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

**Reference sources.** Lending literature §§3.2–3.4; Kotzer §§III-B, III-C, V-C;
Werner §3.2; Gogol §IV-A3; ERC-3156 for the flash callback and repayment
contract.

---

## F3 — Derivatives

**Function.** Payoffs that reference something else — a price, a rate, an event —
rather than conveying ownership of it. Perpetuals, futures, options, structured
payoffs.

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

**Action targets.**

- **DA11 — open / margin / fund / close derivative.**
  Requirement: position notional, margin and funding obligations.
  *Distinguishing test:* funding **signs**; insolvency; precise settlement
  convention. A funding payment whose direction inverts is a total loss of
  meaning, and sign errors are invisible in balance-based models.
- **DA12 — write / exercise / expire contingent claim.** *(shared with P)*
  Requirement: choice authority and exercise/payment dates.
  *Distinguishing test:* **expiry does not erase an already exercised payment
  duty.**

**What goes wrong.** Funding sign inversion. Settlement convention ambiguity —
whether a position settles at mark, index or last trade, and at which timestamp.
An expiry sweep that cancels obligations which were already validly exercised.
Insolvency that is recognized after the loss rather than as a state.

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

**Reference sources.** Werner §3.5; Gogol §IV-B3.

---

## F4 — Consensus-position claims

**Function.** Claims whose value derives from participation in consensus:
staking, liquid staking, restaking and shared security.

**Boundary.** Distinguish slashable allocation from ordinary leveraged lending
or a token-lock reward. Restaking commits capital to *additional* security
duties; that is a different exposure from lending the same capital.

**Facet profile.** Execution: two-phase — request then completion — with a
protocol-imposed delay · Settlement: delayed and sometimes queued · Custody:
delegated to an operator, with operator authority persisting · Legal dependence:
none · Collateral and solvency: slashing is loss allocation, not liquidation ·
Oracles: reward and penalty accounting is internal but externally observable ·
Authorization: delegation is a mandate with its own scope · Price discovery:
consumed.

**Action targets.**

- **DA14 — stake / account rewards / slash.**
  Requirement: share-rate or rebase accounting; loss allocation.
  *Distinguishing test:* **the same nominal token balance can have a changing
  entitlement.** Rebasing (updating quantities) and a fixed share balance with a
  changing conversion rate are different mechanisms with different integration
  consequences.
- **DA15 — request unstake / claim exit.**
  Requirement: pending exit identity, custody and delayed completion.
  *Distinguishing test:* **an exit request is not immediate token delivery.**

**What goes wrong.** Integrations treat a rebasing balance as a fixed claim, or
a fixed share balance as if quantity were entitlement. Exit requests are modelled
as instantaneous, so the pending state — during which the capital is still
slashable but no longer earning, and the claim is not yet transferable — has no
representation at all.

**What Moriarty does.** The pending state is an object with an identity, not a
gap between two states. This is the same machinery as DA18's asynchronous
redemption lifecycle: a request creates a durable identified obligation, and
partial or delayed completion consumes it without resetting it. Affine authority
applies — a partially completed exit consumes its authorization and the residual
cannot grow.

Slashing is loss allocation against claims, and the four mandatory proof claims
require that the resulting state be a compliant successor of a compliant history,
so loss cannot be applied retroactively to a history that did not carry it.

**Reference sources.** Gogol §IV-A1; pending-workflow analysis for exit
lifecycles. DA15 is one of the targets with an identified primary-lifecycle
source gap.

---

## F5 — Tokenized off-chain claims

**Function.** On-chain representations of claims that are ultimately enforced
off-chain: real-world assets, attested claims, custodial receipts.

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

**Action targets.**

- **DA16 — issue / redeem external claim.**
  Requirement: attested claim with custody and legal assumptions.
  *Distinguishing test:* **missing external settlement evidence cannot discharge
  the duty.** An on-chain burn is not proof that anyone off-chain paid.

**What goes wrong.** The on-chain leg completes and is treated as settlement. The
system has no way to represent "the token was burned and the wire has not
arrived", so it represents it as done. Valuation time, publication time,
invalidation and redemption liquidity are collapsed into one NAV number.

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

**Reference sources.** Gogol §§III-B, IV-A2; ERC-3643 and ERC-7943 for transfer
control; ERC-8330 for subject-linked NAV observation boundaries; Centrifuge
protocol source for cross-domain fund infrastructure.

---

## F6 — Delegated asset management

**Function.** Someone else allocates your capital. Vaults, yield strategies,
allocators, standardized accounting entry points.

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

**Action targets.**

- **DA17 — deposit / mint / withdraw / redeem vault shares.**
  Requirement: asset/share conversions with **method-specific** rounding.
  *Distinguishing test:* fees, rounding direction, initial donation and zero
  shares. Four conversion methods, and the correct rounding direction differs
  between them.
- **DA18 — request / fulfill / claim asynchronous redemption.**
  Requirement: Pending → Claimable → Claimed, with residual request amount.
  *Distinguishing test:* partial claim and changed exchange rate; **double claim
  rejects.**
- **DA19 — allocate / harvest / reinvest / unwind / rebalance.**
  Requirement: a bounded workflow of trades, debt, shares and fees.
  *Distinguishing test:* **losses and debt persist through unwind**; liquidity
  shortage.

**What goes wrong.** The donation attack: a direct transfer inflates assets
without minting shares, and the first depositor's rounding does the rest. Rounding
in the depositor's favour on one method and the vault's on another, applied
consistently, drains one side. An asynchronous claim processed twice because the
request had no residual amount. An unwind that reports the position closed while
its debt persists.

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

**Reference sources.** ERC-4626 methods and security considerations; ERC-7540
request lifecycle and `requestRedeem`; ERC-7575 for external share-token
topology; yield literature §§III-A Fig. 2, III-B, V-B; Morpho MetaMorpho source
for allocator→market dependencies.

---

## P — Prediction markets and event-contingent claims

**Function.** Claims indexed by the outcome of an event, resolved by evidence.

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

**Action targets.**

- **DA13 — split / merge / resolve event claims.**
  Requirement: outcome-indexed claims and resolution evidence.
  *Distinguishing test:* **no duplicate winning claim; invalid resolution
  rejects.**
- **DA12 — write / exercise / expire contingent claim.** *(shared with F3)*
  *Distinguishing test:* expiry does not erase an already exercised payment duty.

**What goes wrong.** Split and merge are treated as token operations rather than
as conservation-preserving transformations of a claim set, so a merge can
duplicate value. Resolution is accepted from a source whose authority was never
modelled. An invalid or contested resolution has no rejection path, so the first
answer wins.

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

**Reference sources.** DeFi report contingent-claim targets; Werner §3.5. DA13
carries an identified primary-lifecycle-source gap.

---

## ⊥ — Cross-cutting: what makes the other seven composable

Not a leftover tab. These five targets are the machinery every other category
depends on, and the reason the model is a classification rather than a taxonomy.

**Action targets.**

- **DA20 — observe price / time / external event.**
  Requirement: typed observations with provenance, freshness and domain.
  *Distinguishing test:* stale or unauthorized evidence rejects.
- **DA21 — authorize exact plan / refine outcome intent.**
  Requirement: gross debit, net receipt, recipients, calls, new liabilities.
  *Distinguishing test:* **a refund cannot restore gross capacity; fees count
  against the net goal.**
- **DA22 — change parameters / pause / migrate.**
  Requirement: bounded administrative action under a fixed claim policy.
  *Distinguishing test:* cannot downgrade claims or reset work; affects existing
  positions.
- **DA23 — send / receive / refund pending message.**
  Requirement: bounded pending commitments and finality evidence.
  *Distinguishing test:* delayed or duplicate delivery; explicit refund duty.
- **DA24 — sequence / parallel / interleave / synchronize / message.**
  Requirement: operator-specific authority, duties, conflicts and fan-in.
  *Distinguishing test:* **split partitions work and claims; join cannot
  duplicate resource.**

**The five composition operators.** DA24 names them: **sequence, parallel,
interleave, synchronize, message.** Each has its own authority rules, duty
propagation, conflict semantics and fan-in behavior.

**The measured result that motivates the whole project.** Over 1,830 eligible
protocol pairs, 1,645 compose cleanly and 185 fail. **182 of the 185 failures are
cross-category.** Within-category failure rate 2.10%; cross-category 10.79% — a
**5.14×** difference. Composition across financial categories is where DeFi
breaks, and it is exactly what a type system and an operational semantics can
police. (An earlier informal "sixty times" claim was checked and is wrong.)

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

---

## Completeness checks the implementation must satisfy

- All eight tabs render, with all six blocks populated. No empty states.
- Every action target DA01–DA24 appears; DA12 appears twice and is marked shared.
- Every action target shows its semantic requirement and its distinguishing test.
- The seven family identifiers and the eight facet names are never abbreviated
  away or reordered.
- The five composition operators are always listed as five.
- Facet profiles cover all eight facets in every tab, including the ones where
  the honest answer is "none" or "not applicable".
- Category identifiers are presented as classification labels, never as Moriarty
  source syntax.
