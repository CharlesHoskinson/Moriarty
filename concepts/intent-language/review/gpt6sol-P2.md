# P2 review — MIL/1 expressiveness and category mapping

**Verdict.** MIL/1 has the right aim: a signed outcome policy checked against complete financial effects, with bounded stages and persistent duties. I would **not freeze this design as U0’s language contract**. Several “Expressible” verdicts rely on operations the stated types cannot form, and the escrow rule can authorize a refund on a deadline without establishing entitlement. U0 can freeze a coherent signed-intent core, but it should leave the broader category claims conditional until the missing state and composition rules are specified. This is a review of a specified proposal, not an implementation finding. [DESIGN.md:4](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/DESIGN.md:4), [ROADMAP.md:21](/home/charl/Moriarty/ROADMAP.md:21)

## Seven paper programs

These use the closest syntax in §10 and the category examples. Asset aliases and principals are assumed declared. Each `residue` names a requirement the shown program **does not enforce**; residue never affects validity. I did not run a parser or prover. [DESIGN.md:225](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/DESIGN.md:225), [DESIGN.md:223](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/DESIGN.md:223)

```moriarty
intent TickSwap {
  signer trader = 0x01
  clock midnight.preview.clock
  validity from now for 5m
  budget { gross_debit <= 10 X; fees <= 1 X; net >= 9 Y to trader }
  hole venue : Program where venue ∈ { CLPool }
  hole route : Route where route.hops <= 1
  residue { "tick range [-120,120] and crossed-tick liquidity transitions" }
}

intent CDPMint {
  signer borrower = 0x02
  clock midnight.preview.clock
  validity from now for 1h
  observe px : Price<USDm, ETH, 8> from feed "eth/usdm"
          require fresh(px, 5m) and anchored(px)
  escrow C {
    custody program
    fund 2 ETH from borrower
    release discharged(loan)
    refund after(validity.end)
    deadline validity.end
  }
  authority enforce liquidation by any
  residue { "1000 USDm mint, debt creation and liquidation transition" }
}

intent Perpetual {
  signer trader = 0x03
  clock midnight.preview.clock
  validity from now for 30d
  observe mark : Price<USDm, BTC, 8> from feed "btc/usdm"
          require fresh(mark, 5m) and anchored(mark)
  budget { gross_debit <= 500 USDm; fees <= 10 USDm }
  residue { "signed position, funding periods and maintenance-margin call" }
}

intent BurnMint {
  signer owner = 0x04
  clock midnight.preview.clock
  validity from now for 6h
  escrow Source {
    custody program
    fund 1 Xsrc from owner
    release delivered(Xdst, >= 1 Xdst, to owner)
    refund after(validity.end)
    deadline validity.end + 1h
  }
  residue { "source burn, destination mint, challenge and unknown outcome" }
}

intent VaultDeposit {
  signer alice = 0x05
  clock midnight.preview.clock
  validity from now for 1h
  escrow D {
    custody program
    fund 100 USDC from alice
    release delivered(Share<V>, >= 100 Share<V>, to alice)
    refund after(validity.end)
    deadline validity.end
  }
  residue { "zero-share bootstrap and share issuance" }
}
intent VaultRedeem {
  signer alice = 0x05
  clock midnight.preview.clock
  validity from now for 1h
  escrow R {
    custody program
    fund 10 Share<V> from alice
    release delivered(USDC, >= 10 USDC, to alice)
    refund after(validity.end)
    deadline validity.end
  }
}

intent BidA { signer a = 0x06
  budget { gross_debit <= 10 X; net >= 20 Y to a } }
intent BidB { signer b = 0x07
  budget { gross_debit <= 10 Y; net >= 5 X to b } }
intent BidC { signer c = 0x08
  budget { gross_debit <= 10 Y; net >= 5 X to c } }

intent Flash {
  signer borrower = 0x09
  budget { gross_debit <= 101 USDC; fees <= 1 USDC }
  escrow L { custody program; fund 100 USDC from pool }
  residue { "same-stage callback and repayment before atomic acceptance" }
}
```

The swap specifies the trader’s outcome but cannot bind the tick range, indexed tick state, or a bounded crossing calculation. Naming `CLPool` fixes a venue; it does not constrain an individual position’s range. The CDP fragment exposes a worse problem: its deadline refund would release collateral while `loan` may remain outstanding. It also lacks a specified effect linking the borrower’s consent, debt creation, mint, and a liquidator’s seize-and-discharge path. The perpetual has a `Signed` type but no position cell or transition, funding-period state, or callable maintenance action. These are missing mechanisms behind the corresponding category verdicts. [CATEGORY-MAP.md:30](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/CATEGORY-MAP.md:30), [CATEGORY-MAP.md:92](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/CATEGORY-MAP.md:92), [CATEGORY-MAP.md:119](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/CATEGORY-MAP.md:119)

Burn–mint needs **two accepted domain transitions** joined by authenticated evidence. A local deadline does not establish that a foreign mint did not happen, and MIL has no challenge-record or no-valid-challenge predicate for the proposed dispute window. The three bids balance economically, yet there is no transaction that authenticates all three signatures and checks each intention against one atomic effect set. A join over *disjoint* footprints does not solve clearing against shared inventory. Flash funding also needs the pool’s authority and a bounded borrow–callback–repay trace inside the atomic boundary. [DESIGN.md:35](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/DESIGN.md:35), [CATEGORY-MAP.md:39](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/CATEGORY-MAP.md:39), [CATEGORY-MAP.md:184](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/CATEGORY-MAP.md:184), [CATEGORY-MAP.md:186](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/CATEGORY-MAP.md:186)

## Defects and category-map corrections

1. **The vault claim is ill-typed.** `escrow fund` takes `Qty<A>` and `delivered` names an asset, while `Share<P>` is a separate type. More decisively, the specified `mulDiv` returns `Qty<A>` and divides by `Share<P>`: it can model assets returned on redemption, but not shares issued on deposit. Deposit needs a typed `Qty × totalShares ÷ totalAssets → Share` operation and a defined zero-supply branch. Calling deposit, redeem, and empty-pool bootstrap “Expressible” is wrong. [DESIGN.md:71](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/DESIGN.md:71), [DESIGN.md:79](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/DESIGN.md:79), [CATEGORY-MAP.md:198](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/CATEGORY-MAP.md:198)

2. **The escrow exit theorem is false.** `release ∨ refund ∨ after(deadline)` can hold because time passed while neither executable branch is enabled. It proves no closure path, and a deadline proves no cross-domain non-execution or refund entitlement. The §10 example also permits `delivered(...)` and `after(validity.end)` together, violating its own disjointness rule. `release delivered(...)` is circular if delivery is an effect of `on_release`. Specify branch preconditions, resulting effects, unknown outcomes, and exclusive terminal consumption separately. [DESIGN.md:187](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/DESIGN.md:187), [DESIGN.md:251](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/DESIGN.md:251), [docs/MORIARTY-PRODUCT-CONTRACT.md:47](/home/charl/Moriarty/docs/MORIARTY-PRODUCT-CONTRACT.md:47)

3. **Several claimed carriers do not carry their claims.** Φ has `balance(...)` but no `pre`/`post` projections, so it cannot state the map’s AMM transition invariant. `ClaimClass` is an order, not a loss-allocation effect or ADL rule. An encumbrance has one `against: ObligationId`, contradicting the assertion that one encumbrance against several obligations is expressible. The footprint cell set omits encumbrances, positions, and share totals, so its independence rule can admit conflicting operations unless actual reads and writes are derived and checked. [DESIGN.md:104](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/DESIGN.md:104), [DESIGN.md:114](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/DESIGN.md:114), [DESIGN.md:125](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/DESIGN.md:125), [CATEGORY-MAP.md:121](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/CATEGORY-MAP.md:121), [CATEGORY-MAP.md:206](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/CATEGORY-MAP.md:206)

4. **A remote effect needs a different judgment from a local effect.** Giving every effect a domain makes a foreign debit and credit *describable*, but a Midnight stage cannot thereby execute or authenticate both. Otherwise the effect relation can report a foreign mint as though it settled. Require local effects to match the executing domain; represent foreign results as imported evidence linked to a later accepted stage. [DESIGN.md:219](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/DESIGN.md:219), [docs/MORIARTY-CONSOLIDATED-DESIGN.md:72](/home/charl/Moriarty/docs/MORIARTY-CONSOLIDATED-DESIGN.md:72)

## What is missing, and where it belongs

**MIL/1 core:** pre/post and indexed state terms; actual-footprint derivation; typed share issuance and bootstrap; multi-intent atomic acceptance; explicit position and period-indexed obligation state; escrow branch/phase semantics; evidence predicates for signatures, recipient acceptance, documents, challenges, and unknown foreign outcomes. The last group is required by the product’s conditional-delivery contract, yet absent from the five-item open list. `fresh` also needs a same-clock rule or an explicit trusted cross-clock relation: an observation’s `observedAt` may be on another domain, while freshness is defined against stage time. [DESIGN.md:49](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/DESIGN.md:49), [DESIGN.md:138](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/DESIGN.md:138), [docs/MORIARTY-PRODUCT-CONTRACT.md:80](/home/charl/Moriarty/docs/MORIARTY-PRODUCT-CONTRACT.md:80)

**Libraries over that core:** tick-swap mathematics with a bounded crossing limit, collateral valuation and liquidation policy, funding-rate formula, auction price rule, vault virtual offsets, and fee schedules. Calling TWAP a library is justified only after bounded observation collections and aggregation exist. [CATEGORY-MAP.md:145](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/CATEGORY-MAP.md:145)

**Later milestone:** native costs and proofs for these broader constructors, live cross-domain adapters, and full category conformance. U0 must decide their semantic interfaces, not claim their behavior demonstrated. [ROADMAP.md:21](/home/charl/Moriarty/ROADMAP.md:21), [ROADMAP.md:27](/home/charl/Moriarty/ROADMAP.md:27)

The twelve-asset table is a useful inventory, **not a three-rule asset architecture**. Rebasing needs an authenticated share/index conversion or bounded holder updates; fee-on-transfer needs transfer-policy effects and recipient-net accounting; blacklists and pauses need changing policy state; multi-collateral claims need basket valuation and lien priority. NFT fractionalisation needs a token identifier plus a lock/share/redeem link: supply `{0,1}` alone cannot distinguish two NFTs of one collection. For off-chain claims, `issuer: Policy` does not identify the obligor or establish its consent; an attestation cannot create that liability. Per-asset conservation remains necessary but proves none of those relationships. [CATEGORY-MAP.md:216](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/CATEGORY-MAP.md:216), [CATEGORY-MAP.md:228](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/CATEGORY-MAP.md:228)

## Disagreements and solver freedom

Of the five declared-open items, **n-party clearing and general policy state are genuine MIL/1 gaps**. Flash loans are a design choice, not a consequence of atomicity: a bounded internal trace with final repayment can still be one atomic stage. Competing-slash ordering should be a signed deterministic priority or pro-rata policy; the hard missing piece is a shared encumbrance and effect rule. A liquidation *latency guarantee* is liveness and cannot follow from `fresh` or a deadline; MIL can specify when enforcement becomes permitted, while inclusion remains an assumption. [CATEGORY-MAP.md:249](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/CATEGORY-MAP.md:249)

I also disagree that polarity proves safe solver completion. Routes and programs have no numeric “increasing” order, and the stated implication compares a filled guarantee with an unfilled formula. Matching two signed offers needs intersection of both policies and complete-effect checking. A route hole is useful search space; unconstrained choice of a price, recipient, challenge result, or matching counterparty is not. Define completion as a relation between authenticated policies and a candidate episode, rather than a polarity test alone. [DESIGN.md:193](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/DESIGN.md:193), [DESIGN.md:201](/home/charl/Moriarty/deliverables/intent-language-design-2026-09-28/DESIGN.md:201)

A sharp comparison: *[df]* derives required reads from guards and effect expressions, then checks declared footprints in [Typed/Transition.lean:80](/home/charl/projects/defiformal/lean/DefiKernel/Typed/Transition.lean:80) and [Typed/Transition.lean:121](/home/charl/projects/defiformal/lean/DefiKernel/Typed/Transition.lean:121). MIL’s declared-only footprint is weaker. MIL’s intended signed gross limits, persistent debt, and cross-domain escrow address different financial obligations that *[df]*’s aggregate net-effect transition explicitly does not model. Anoma’s candidate composition is also a stronger starting point for three-party clearing; MIL’s proposed advantage is binding each participant’s financial limits and residual duties to eventual Midnight acceptance, which this design has yet to define. [Typed/Transition.lean:5](/home/charl/projects/defiformal/lean/DefiKernel/Typed/Transition.lean:5), [Anoma synthesis:17](/home/charl/Moriarty/deliverables/anoma-study-2026-09-19/SYNTHESIS.md:17)

## Top three edits before freeze

1. **Replace Φ’s state fragment** with typed `pre`/`post` indexed reads, checked finite collections, derived footprints, and both directions of share conversion including zero-supply behavior; correct the affected “Expressible” rows.
2. **Replace escrow exhaustiveness** with an executable branch-and-effect judgment that tracks pending/unknown foreign outcomes, challenge evidence, authorized refund, retained duties, and exclusive terminal consumption. Fix the §10 example against that rule.
3. **Specify multi-intent completion and core lifecycle cells** for positions, encumbrances, shares, and policy state. State that route matching and financial formulas can be libraries only when these underlying transitions and authenticated effects are present.