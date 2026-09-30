# Lending and borrowing: adversarial controls and release plan (independent recommendation)

**Scope.** This is read-only design review of a proposal that exists only as a specification. I ran `status --json` first. It reported capability `SP01.6 loan-swap-subset`, implementation blocked by stale inputs and missing accounting, and no pending transactions. I claim no implementation, proof or ledger acceptance. Every numbered edit below is a **recommendation**.

## 1. Verdict on design fitness

**The design is fit as a vocabulary, but its safety is not closed.** MIL/2 names the right objects:
- an obligation kept separate from the encumbrance (`DESIGN-MIL2.md:93-102`);
- an `enforce` right bounded by debtor consent (`:193`);
- the roll-forward rule L1 with forgiveness as its own term (`MIL2-PROPOSED-SEMANTICS.tex:180-187`);
- the aggregate lock rule K1 (`.tex:229-237`).

Five adversarial paths are still open in the text as written:
1. **Double pledge or drain:** no transition is forced to read all locks.
2. **Selective oracle:** `fresh` limits the age of a price but does not require the current one.
3. **Unbound discharge or forgiveness:** debt can be reduced without a matching payment or an authorized write-down.
4. **Concurrent liquidators:** they are ordered by a "signed total order" that no permissionless keeper can supply.
5. **Price-conditioned liquidation is not actually in Φ₀.**

None of these needs a new layer. Each needs an exact rule. Items 1 and 4 touch the cell vocabulary that is frozen at U0 (`DESIGN-MIL2.md:345`), so they should land before the U0 digest is hash-bound.

## 2. Five ranked design edits

### E1 (highest). Authenticated per-owner lock total; every debit checks spendable balance

**Problem.**
- K1 sums over every encumbrance of an owner and asset (`.tex:232`). A stage, however, reads only its footprint cells.
- The transfer footprint (F1) lists balance, allowance, fee, replay and custody cells, but no lock cell (`.tex:211-216`).
- The cell vocabulary has `encumbrance(id)` but no aggregate cell (`DESIGN-MIL2.md:239-242`).

**Rule sketch.** Add a cell `lockTotal(d, owner, asset)` holding committed plus reserved locks. Then:
```
LockOp(x, δ):   lockTotal' = lockTotal ± δ   (checked, reject on underflow)
                encumbrance(x)' updated in the same stage
AnyDebit(p,a,n): reads/writes balance(d,p,a), lockTotal(d,p,a)
                 require balance − n ≥ lockTotal'        -- K1 as a local check
Seize(x,n):     lockTotal' = lockTotal − n, balance' = balance − n, n ≤ amount(x)
Release(x):     require ∀o ∈ x.against. discharged(o)     -- set semantics
```
F1's auxiliary cells must include `lockTotal` for the debited party.

**Counterexamples.**
- Alice holds 1.5 ETH and pledges it to loan L1. Stage A is a plain transfer of 1.5 ETH out. Its footprint is disjoint from `encumbrance(e1)`, so it is accepted and L1 is left unsecured.
- Two concurrent origination stages each lock 1.5 ETH against different encumbrances. Each checks only its own amount against the balance, so both are accepted.
- An encumbrance backs `{o1, o2}`. Repaying o1 releases all the collateral, leaving o2 unsecured.

**Placement.** The cell kind and footprint rule belong to **U0**. This changes a U0 boundary because the vocabulary is frozen there and adding a cell after the digest would change it. The executed K1 differential and native authenticated read of `lockTotal` belong to **U2**. The two-solver reservation traces (O3) belong to **U3**.

### E2. Price comparison as one certified product-compare primitive; seizure as a checked hole, with no division

**Problem.** The category review says a literal-ratio threshold over a price is "plausible in Φ₀" (`02-lending.md:20`). That is true only if the collateral quantity is a signed literal. Once collateral is read from state (after any partial seizure), `collQty × price` multiplies two variables, which is Φ₁ (`DESIGN-MIL2.md:160-161`).

**Rule sketch.** Add one Φ₀ atom and give seizure a checked upper bound:
```
covers(q_c : Qty<C>, p : Price<C,D,s>, q_d : Qty<D>, num/den literals, roles)
  ≡ q_c · p · den  ≥  q_d · num · 10^s      -- two-limb gadget (§4.5), no division
Seize bound (hole seize : Qty<C>):
  seize · p · 10_000 ≤ repay · (10_000 + bonusBps) · 10^s
```
Rounding roles:
- collateral value rounds down, against the borrower;
- debt rounds up;
- seizure is checked with `≤`, so any remainder stays with the debtor.

The escrow authoring check treats `covers` as an uninterpreted Boolean atom. This is sound for validity; an `unknown` result fails closed.

**Why this touches a deferred Φ₁ boundary.** Liquidation is the minimum lending exercise. The measured blow-up that justified deferring Φ₁ was for 128-bit *division* (`DESIGN-MIL2.md:163`). A product-then-compare avoids division entirely. Φ₁ share conversion and `mulDiv` stay deferred.

**Counterexamples.**
- After a 50% seizure, the second health check needs `collQty(state) × price`. Φ₀ as specified cannot express it, so an author "approximates" with the literal original collateral and overstates health.
- A seizure computed by host-side division is rounded up in the keeper's favour.

**Placement.** The atom goes in the numeric profile with rounding roles at **U0** (`ROADMAP.md:21`). The native certificate goes at **U1**. Liquidation use goes at **U3**.

### E3. Oracle binding: current round, conservative stage time, one observation per decision

**Rule sketch.**
```
EvidenceValid(obs) requires:
  obs.id = head(observation(feedId)) at the authenticated pre-state   -- current, not any recent round
  hi(stageTime) − obs.observedAt ≤ Δ_purpose                         -- conservative bound, not a witness
  obs.observedAt ≤ lo(stageTime)
Liquidate uses one obs id for both covers(…) and the seize bound;
  write replay(obs.id, enc.id)  -- one liquidation per (price, encumbrance)
Authoring check: openingRatio > maintenanceRatio (signed literals)
```

**Counterexamples.**
- Two reports within a 5-minute window read 1,990 and 2,010. The keeper picks 1,990 to trip `covers`. The design's `fresh` (`DESIGN-MIL2.md:144, 187`) accepts this.
- The prover picks `stageTime` equal to `observedAt`.
- Opening ratio equals maintenance ratio, so the borrower can be liquidated in the next stage at the same price.

**Placement.** Evidence fields and rejection codes at **U0**. Native binding of the feed head and time bound at **U2**. The one-liquidation-per-price replay mark at **U3**.

### E4. Discharge bound to payment; forgiveness only under creditor authority

**Rule sketch.**
```
discharge_o = Σ { n | transfer(d, asset_o, payer, creditorAcct(o), n) ∈ e, attributed(o) }
Forgive(o, n): requires amend-right scoped to obligation(o), holder = o.creditor,
               budget linear; recorded as a liability record, never a balance delta
Impair(o): status flag only; u'_o = u_o
Tombstone(o) only if u'_o = 0 ∧ no active x with o ∈ x.against
```
I recommend reusing `amend`, scoped to one obligation cell, rather than adding a ninth right. The rights enum is frozen at U0.

**Counterexamples.**
- A repayment stage transfers 3 USDC but records `discharge = 30`. L1 (`.tex:182-187`) balances, but only the payment reference binds the transfer.
- A liquidation recovers 800 against 1,000. The keeper's stage records `authorizedForgiveness = 200`, zeroing the debt without the creditor.
- A debt token is burned and the obligation is reduced. This conflates supply with debt (`MORIARTY-PRODUCT-CONTRACT.md:43`).

**Placement.** Right scoping and the liability record at **U0**. The executed L1 differential at **U2**. Residual debt after a partial liquidation at **U3**.

### E5. Liquidator concurrency via a ledger state-version precondition, not a signed order

**Problem.** `DESIGN-MIL2.md:193` requires a "signed total order on seizes of one encumbrance." Independent keepers share no signer.

**Rule sketch.**
```
Enforce stage binds pre-head h of obligation(o) and encumbrance(x);
ledger accepts at most one stage per (cell, h); a successor must re-evaluate covers(…) on post-state.
closeFactor applies to u_o at h; the signed order remains only for several seizes inside one intent.
```

**Counterexample.** Two keepers each repay 50% (the close factor) against the same pre-state. Both pass the health check on the pre-state, together seize more than `amount(x)`, and the second one liquidates a position the first already restored.

**Placement.** Version precondition in the U0 stage schema (the `history` key, `.tex:149`). Race traces at **U3**, alongside the late-success/refund race (`ROADMAP.md:24`).

## 3. Core versus library boundary

**Core (language and stage relation):**
- the `Obligation` and `Encumbrance` cells, `lockTotal` and K1;
- L1 with payment-bound discharge;
- forgiveness authority and the impaired flag;
- the `covers` primitive with rounding roles;
- the oracle head and time binding;
- the version precondition;
- release under set semantics;
- failure-phase residual duties (`MORIARTY-PRODUCT-CONTRACT.md:47`).

**Library (U6):**
- LTV and threshold parameters, bonus and close-factor policies, liquidation modes (fixed bonus or auction);
- interest models;
- isolation and e-mode sets;
- loss waterfalls and backstops;
- pooled lending with shares (which waits for Φ₁ `sharesFor`/`assetsFor`; external practice favours rounding toward the vault, per [ERC-4626](https://eips.ethereum.org/EIPS/eip-4626), captured at `source-text/erc4626.md:631-633`. That is external practice, not a Moriarty rule).

A library may only instantiate core rules. It cannot weaken K1, L1 or evidence binding.

## 4. Smallest implementable slice and evidence pair

**Slice (U2, no oracle, Φ₀ only).** One domain, one signer, all quantities literal.
- **Origination:** Alice's `ConsentRef`, a 1,000 USDC funded transfer, and a lock of 1.5 ETH with `lockTotal += 1.5`.
- **Full repayment:** a 1,000 USDC transfer to the creditor; discharge equals the transfer; `u' = 0`.
- **Release:** `discharged(o)` holds for every obligation, `lockTotal −= 1.5`, and both the obligation and the encumbrance are tombstoned.

Liquidation waits for the E2 certificate at U1 and is sequenced into U3.

**Positive control.** From the retained origination state, Alice repays exactly 1,000 USDC in a correctly formed, feasible stage. It must be accepted natively, and the readback must show the transfer, `u' = 0`, `lockTotal = 0` and both tombstones.

**Hostile control, from the same pre-state.** Alice submits a well-formed transfer of 1.5 ETH to Bob, signed and with valid fees. It must be rejected by the `balance − n ≥ lockTotal` constraint.

Record the failing constraint: a rejection caused by a malformed envelope does not count (`ROADMAP.md:40`). Companion hostile witnesses:
- `lockTotal` supplied lower than the authenticated value;
- a branch bit of 2;
- `discharge` of 1,000 with a transfer of 999;
- a u128 underflow that would wrap in the field;
- replay of the release.

**Stop rules (recommended):**
- **S1.** If `lockTotal` cannot be an authenticated native read or write, stop the collateral scope. Do not ship an unsecured loan.
- **S2.** Any hostile control accepted means stop, reproduce, and make no further submissions (`AGENTS.md:43`).
- **S3.** A positive control rejected for envelope reasons counts as no evidence.
- **S4.** If the `covers` certificate fails at U1, liquidation is out of U3. Fixed-term collateral without liquidation may ship only with a residue statement.
- **S5.** No liquidation reaches Preview before E3's head binding and E5's version precondition have native evidence.

## 5. Explicit disagreements

- **Φ₀ plausibility.** `02-lending.md:20` calls a price-threshold liquidation plausible in Φ₀. I disagree when collateral comes from state: it is Φ₁ unless E2 is adopted.
- **Signed order for keepers.** `DESIGN-MIL2.md:193`: a signed order among keepers is unimplementable. A ledger version precondition is the right mechanism for independent keepers.
- **F1 footprint.** `.tex:211-216` defeats K1 by omitting lock cells. This is the same class of defect the design repaired for receipts (`DESIGN-MIL2.md:245`).
- **`fresh`.** It admits selective replay of stale rounds (`DESIGN-MIL2.md:144`).
- **Broken links.** `02-lending.md` links R2 as `../../../u0-study-2026-09-28/…`, which resolves to the repository root. The file actually lives at `deliverables/u0-study-2026-09-28/defi-coverage/R2-lending.md`.

## 6. Residual assumptions

- Midnight exposes an authenticated per-transaction time interval `[lo, hi]` and allows state-version preconditions, so that a stale-head stage fails at the ledger. **Unverified against the pinned ledger.**
- A two-limb `covers` gadget fits the U1 cost caps. The caps are proposals, not measurements (`DESIGN-MIL2.md:270`).
- Oracle honesty, keeper arrival and liquidity are named assumptions, not proved properties. `fresh` bounds the age of an observation, not how quickly a keeper acts.
- Multi-domain collateral stays out of scope: there is no global rollback, and timeout is not evidence of nonexecution (`.tex:202`).
- The optional federated kernel plays no part in any control above.