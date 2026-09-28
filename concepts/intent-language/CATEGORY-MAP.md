# MIL/1 mapped to the DeFi and asset categories

**Date:** 2026-09-28
**Status:** specified-only proposal. Companion to [DESIGN.md](DESIGN.md). Section references like §3.4 point there. Nothing is implemented.

**How to read the verdicts.** *Expressible* means the intent language can state the constraint and the acceptance relation can check it. *Library* means it is ordinary source over the core, needing no new core mechanism. *Open* means something still has no answer — stated plainly rather than absorbed.

---

## Part 1 — DeFi categories

### 1. AMMs and exchanges

```
intent Acquire {
  budget { gross_debit <= 11 A; fees <= 1 A; net >= 20 B to owner }
  hole route : Route where route.hops <= 3
  hole venue : Program where venue ∈ approved
  escrow E { fund 11 A from owner
             release delivered(B, >= 20 B, to owner)
             refund  after(validity.end) }
  residue { "ordering and MEV exposure are not expressed" }
}
```

The signer never names a pool. `venue` and `route` are holes; the guarantee is the net outcome. This is the outcome-first property doing the work the category needs: a swap, a multi-hop route, an RFQ fill and a batch-auction clearing are the *same intent* with different completions.

| Requirement | Verdict | Carrier |
| --- | --- | --- |
| Invariant `x·y ≥ k` over pre/post state | **Expressible** | Φ cross-multiplied comparison (§3.3, §4); the pool's own program states it, the effect judgment checks it |
| Slippage / price-impact limit | **Expressible** | `net >= …` plus cross-multiplied ratio, no division |
| LP share issue and redeem | **Expressible** | `Share<P>`, `mulDiv`, `issue` right (§3.3, §5) |
| Fee tiers | Library | literal or computed fee lines under `feeCap` |
| Multi-hop routing | **Expressible** | `hole route`, monotone completion (§7) |
| Order book, RFQ, batch auction | **Partly open** | one intent per participant is expressible; *clearing several participants in one stage* is not — see open item below |
| MEV, ordering, surplus allocation | **Open, and declared** | `residue`; no construct, deliberately |

**Closes:** the language previously could not state an inequality over pre- and post-state at all, which is why the one existing swap example pins its output to a fixture constant.
**Open:** n-party clearing. A stage binds one signed intention. Batch auctions and order-book crossing need several signers' intents cleared against each other in one atomic settlement. MIL expresses each side; the composite is an **Episode with a join over disjoint footprints**, and whether that is sufficient for a true batch clear — or whether a multi-signer stage is required — is the one open question this design does not settle for the category.

### 2. Lending and borrowing

```
escrow Collateral {
  custody program
  fund    1.5 ETH from borrower
  release discharged(loan)
  refund  never
  deadline none
}
obligation loan {
  debtor borrower; creditor pool; principal 1000 USDC
  terms  { accrual simple(rate, period); allocation AccrualFirst }
  rank   senior
}
authority enforce liquidation by any
  policy { trigger  mulDiv(collateral_value, ltv_max, 1) < outstanding(loan)
           requires fresh(price, 5m) and anchored(price)
           bound    seize <= outstanding(loan) · (1 + bonus) }
```

| Requirement | Verdict | Carrier |
| --- | --- | --- |
| Principal / accrued / outstanding, roll-forward | **Expressible** | `Obligation` (§3.5); already the strongest part of the evaluator |
| Collateral posting and withdrawal | **Expressible** | `Encumbrance` — a linear resource the owner cannot spend (§3.5) |
| LTV / health factor | **Expressible** | Φ cross-multiplied over an `anchored` price with `fresh` |
| Liquidation by a third party | **Expressible** | `enforce` right bounded by a policy the debtor consented to at origination (§5) |
| Default | **Expressible** | a named predicate, which the architecture never defined |
| Interest: simple | **Expressible** | evaluator's existing periodic ratchet |
| Interest: compounding, variable, index-based | Library + numeric | needs `mulDiv` and a rate-update action |
| Bad debt, backstop, seniority | **Expressible** | `ClaimClass` rank plus loss-allocation effects |
| Flash loans | **Open** | an intra-stage borrow-and-repay is exactly the intra-stage structure this design excludes |

**Closes:** collateral had no sort, liquidation had no authority, default had no definition, and the solvency predicate had nothing joining an observation to an obligation's admissibility.
**Open:** flash loans are the honest casualty of keeping the stage atomic and pushing sequencing to the Episode. They would require intra-stage steps.

### 3. Stablecoins and synthetic assets

```
authority issue by policy StableMint
  scope  (midnight.preview, USDm)
  policy { requires collateral_ratio_ok and fresh(price, 1m) and anchored(price)
           bound    supply(midnight.preview, USDm) + delta <= debt_ceiling }
effect supply { domain midnight.preview; asset USDm; delta +1000; authority StableMint }
```

| Requirement | Verdict | Carrier |
| --- | --- | --- |
| Mint / burn with authorization | **Expressible** | `issue` right + `supply` effect (§5, §9) |
| Per-asset conservation | **Expressible** | `Σ balance deltas = supply delta` (§9) — the general law |
| Debt ceiling / supply cap | **Expressible** | Φ over `supply(d, a)` |
| CDP issuance against collateral | **Expressible** | `Encumbrance` + `issue` + solvency Φ |
| Redemption at par | **Expressible** | escrow with `release = delivered(…)` and burn on release |
| Peg arbitrage, stability fee, savings rate | Library | fees and accrual over existing constructs |
| Distinguishing USDm from USDn | **Expressible** | structured `Asset` with `issuer` (§3.2) |
| Emergency shutdown / global settlement | **Partly open** | expressible as an `enforce`-gated terminal episode; the *policy state* that makes it reachable is the governance gap below |

**Closes:** the whole category. Supply changes were bound as a stage field that no layer was responsible for producing, and no authority could authorize them, though the product contract requires "authorized mint/burn supply changes".

### 4. Derivatives

```
intent CoveredCall {
  observe mark : Price<ETH, USD, 6> from feed "eth/usd"
  escrow Premium { fund 50 USDC from buyer; release after(expiry); refund never }
  escrow Underlying { fund 1 ETH from writer
    release  exercised and before(expiry)
    refund   after(expiry) and not exercised }
  position p : Signed<ETH> = −1 ETH        -- writer is short
  authority enforce margin_call by any
    policy { trigger equity(p, mark) < maintenance }
}
```

| Requirement | Verdict | Carrier |
| --- | --- | --- |
| Expiry, exercise window, settlement fixing | **Expressible** | `Instant`, `Window`, domain-qualified clocks (§3.1) |
| Signed position / direction | **Expressible** | `Signed<A>`, distinct from balances which stay unsigned (§3.3) |
| Margin, maintenance, margin call | **Expressible** | `Encumbrance` + `enforce` + Φ over a fresh anchored mark |
| Periodic funding, mark-to-market | **Expressible** | Episode of time-gated continuations, generalizing the evaluator's existing non-skip/non-replay period ratchet |
| Loss allocation, ADL, socialized loss | **Expressible** | `ClaimClass` ordering |
| Insurance fund | Library | a pool with `Share<P>` |
| Options, perps, futures, structured tranches | Library | all of the above composed |
| Latency bound on liquidation | **Open** | `fresh` bounds the *observation*; nothing bounds how fast an enforcement must occur |

**Closes:** no time type, no position state, no seniority — the three findings that made the category unbuildable. The signedness question is answered explicitly: balances unsigned, positions signed, which the architecture had left silent in both directions.

### 5. Oracles and observations

```
observe price : Price<B, A, 6> from feed "dust/night"
        require fresh(price, 5m) and anchored(price)
observe reserves : Qty<USD> from feed "proof-of-reserve"
        require attested(reserves, 3, 5) and final(reserves)
```

| Requirement | Verdict | Carrier |
| --- | --- | --- |
| Typed observed value with unit | **Expressible** | `Obs<T>` (§3.4) |
| Freshness / staleness / heartbeat | **Expressible** | `fresh(obs, d)` with `observedAt` as the operand |
| Issuer identity, feed identity | **Expressible** | `feed`, `issuer` fields |
| Finality and reorg assumptions | **Expressible** | `Finality`, `final(obs)` |
| Multi-source, k-of-n, fallback | **Expressible** | `k_of_n` in Φ; `attested(issuer, k, n)` |
| Anchored vs imported | **Expressible, as a type rule** | the anchoring rule (§3.4) |
| Aggregation, TWAP | Library | over a bounded window of observations |
| Circuit breaker / deviation | **Expressible** | Φ comparison between two observations |
| Oracle truth | **Out of scope, correctly** | issuers remain explicit trust assumptions |

**Closes:** the category's central gap — an observation with no value and no observed-at time. This is also what unblocks lending solvency, derivatives settlement and any price-conditioned escrow, which is why it is on the critical path for the owner's escrow decision rather than a category concern.

### 6. Governance

| Requirement | Verdict | Carrier |
| --- | --- | --- |
| Authority kinds | **Expressible** | eight rights, `amend` among them (§5) |
| Delegation of authority | **Expressible** | `delegable`, scoped |
| Amendment of an in-flight obligation | **Expressible** | `amend` right + consent from the party made liable |
| Protection of a non-governance program from a rule change | **Expressible** | **policy commitment**: an intent binds `policy(Id)`, and a continuation may only extend under the policy it was signed against |
| Quorum, multisig, threshold | **Expressible** | `k_of_n`, `custody threshold(k, n)` |
| Timelock | **Expressible** | `after(Instant)` on an `amend`-gated effect |
| Proposal lifecycle, voting, vote-escrow | Library | a pool, an obligation and time-gated stages |
| Parameter change, pause, upgrade | **Partly open** | expressible as `amend`/`enforce` effects on `policy(Id)`; the *general* notion of policy state over reachability remains thin |

**Closes:** the right-kind gap, and — more importantly for every other category — the in-flight protection. The policy commitment is the cheap half: one bound field, so a parameter change cannot silently alter an obligation already signed against a different policy.
**Open:** governance *process* is deliberately library work. The design's own documents conflict on whether governance is in or out of scope; this design takes the position that governance **effects** are in the language and governance **process** is a library, and that position should be written into the controlling documents to end the conflict.

### 7. Bridges and cross-domain settlement

```
escrow Source {
  custody program
  fund    1 BTC from owner                    -- on bitcoin
  release delivered(wrapped(BTC), >= 1, to owner, on midnight.preview)
          via imported(policy = light_client_v2)
  refund  after(deadline)
  deadline now + 6h
}
```

| Requirement | Verdict | Carrier |
| --- | --- | --- |
| Domain-qualified effects | **Expressible** | domain on every effect line (§9) |
| Canonical vs wrapped representation | **Expressible** | `Repr` in `Asset` (§3.2) |
| Lock-mint, burn-mint | **Expressible** | escrow + `issue` right, per domain |
| Imported facts with a trust policy | **Expressible, as a type rule** | `imported of Policy`; cannot be written where `anchored` is required |
| Dispute window, optimistic verification | **Expressible** | `after(Instant)` + `k_of_n` challenge predicates |
| Fast fill by a filler | **Expressible** | filler completes a hole; escrow releases on imported proof |
| Compensation | **Expressible** | `on_refund`, declared, not inferred |
| Cross-domain conservation | **Decided, not assumed** | stated per domain; a cross-domain pair is an *obligation* linking two escrows, never one conservation equation |
| Global atomicity | **Refused, by construction** | no predicate can assert an anchored fact about a foreign domain |

**Closes:** effect lines that could not carry a domain, wrapped and canonical assets sharing one identifier, and the anchored-versus-imported rule living only in a deliverable. The design's prohibition on claiming global rollback from a local model survives as a typing consequence rather than as advice.

### 8. Staking, restaking and yield

| Requirement | Verdict | Carrier |
| --- | --- | --- |
| Vault shares, deposit / mint / withdraw / redeem | **Expressible** | `Share<P>`, `mulDiv`, `issue` (§3.3) |
| Share price / exchange rate | **Expressible** | derived and *bound*, so the rate a conversion used is recorded |
| Empty-pool bootstrap, virtual offset | **Expressible** | a pool invariant in Φ; the donation attack becomes a stated precondition |
| Truncated unit on redemption | **Expressible** | `retained-in-pool`, the sixth remainder class (§3.3) |
| Reward accrual, emissions | **Expressible** | time-gated continuations + `issue` |
| Slashing | **Expressible** | `enforce` right + loss allocation by `ClaimClass` |
| Unbonding period | **Expressible** | escrow with `release after(unbond_end)` |
| Delegation to a validator | **Expressible** | `Encumbrance` + delegated authority |
| Restaking / shared security | **Partly open** | one encumbrance against several obligations is expressible; whether the *ordering* of competing slashes is decidable is open |

**Closes:** a library family the core could not type. `Shares` existed as a type with no `Mul` or `Div` rule, so the `mulDiv` shape the whole category runs on was not typeable and a total supply was not summable.

---

## Part 2 — Asset categories

The asset algebra (§3.2, §3.3, §3.5) is designed so that every asset category has a type, a conservation rule and a required authority. This table is the answer to "how do the components fit together" for assets specifically.

| Asset category | Type | Conservation rule | Authority to create | Notes |
| --- | --- | --- | --- | --- |
| **Native base units** | `Asset{issuer: native}` | Σ deltas = 0 in-domain | none — the domain issues | fees are ordinary transfers to a fee recipient |
| **Fungible issued tokens** | `Asset{issuer: Principal\|Policy}` | Σ deltas = supply delta | `issue(domain, asset)` | the general case; stablecoins are this plus a policy |
| **Shielded balances** | `Asset{repr: shielded of A}` | Σ deltas = supply delta, with disclosure declared | same as underlying | privacy is a disclosure policy, not a different asset economy |
| **Wrapped / bridged** | `Asset{repr: wrapped of A}` | per-domain; the cross-domain link is an **obligation**, not an equation | `issue` on the destination | canonical and wrapped are distinct assets with a declared link |
| **Synthetic** | `Asset{repr: synthetic of Policy}` | supply delta under a policy predicate | `issue` bounded by collateral Φ | no claim on a reserve unless an obligation says so |
| **Pool claims / shares** | `Share<P>` | `Σ shares = totalShares(P)`; value via `mulDiv` | `issue` scoped to the pool | LP tokens, vault shares, liquid staking — one type |
| **Obligations / debt** | `Obligation` | `opening + accrual − discharge = closing`, components ≥ 0 | consent from the party made liable | **debt is not token supply**; the product contract's separation is preserved in the type system |
| **Encumbrances / collateral** | `Encumbrance` | linear; locked ⊆ owner's balance | owner's `initiate`; released by discharge or `enforce` | the missing sort that made collateral unstatable |
| **Receipts / vouchers** | linear `Receipt` | consumed exactly once | produced by an effect | consumed receipts are linear resources, per the design |
| **Positions** | `Signed<A>` | Σ long = Σ short per instrument | `initiate` within margin | the only signed quantity; balances stay unsigned |
| **Unique assets (NFTs)** | `Asset` with supply ∈ {0,1} | supply delta ∈ {−1, 0, +1} | `issue` | a degenerate fungible, not a new kind |
| **Real-world / attested claims** | `Asset{issuer: Policy}` + `Obs` attestation | supply delta under attestation Φ | `issue` + `attested(k, n)` | the obligor is in the asset identity; enforceability is explicitly not claimed |

**The rule that makes the table coherent:** an asset's *identity* carries its domain, issuer and representation; an asset's *economy* is one conservation equation per `(domain, asset)`; and creating units always requires the `issue` right, while creating liabilities always requires consent from the party made liable. Those three sentences are the whole asset architecture, and none of the three was previously expressible.

---

## Part 3 — Scorecard

| Category | Before (architecture axis) | With MIL/1 |
| --- | --- | --- |
| AMMs / exchanges | library placement only; no invariant carrier | expressible except n-party clearing |
| Lending | obligation half only; no collateral, no liquidation | expressible except flash loans |
| Stablecoins | placed nowhere; no supply authority | expressible |
| Derivatives | placed, not architected; no time, no position | expressible except a latency bound |
| Oracles | well designed, no value or time | expressible |
| Governance | placed nowhere | effects expressible; process is library; scope conflict must be resolved in writing |
| Bridges | correctly placed in kernel/adapters | language side expressible; global atomicity refused by construction |
| Staking / yield | a family the core could not type | expressible except competing-slash ordering |
| **Asset categories** | asset identity was a bare string | twelve categories, three rules |

**Five things remain open and are not hidden:** n-party clearing, flash loans (excluded by the atomic-stage decision), a liquidation latency bound, competing-slash ordering, and the general form of policy state over reachability. Each is stated in the category section that owns it.

**The one prerequisite this design does not supply:** the family→constructor map. MIL adds constructors — `mulDiv`, `Share`, `Encumbrance`, `Signed`, `Instant`, `Obs`, footprints, two authority rights — which makes the map more necessary, not less. It should be produced for one rule per family, by the method already demonstrated once, and given a milestone owner that is not a provenance alias.
