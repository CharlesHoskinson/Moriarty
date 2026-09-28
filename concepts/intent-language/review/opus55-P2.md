# MIL/1 review — P2, expressiveness and the category mapping

**Reviewer:** Claude Opus 5.5 · **Lens:** P2 · **Date:** 2026-09-28 · read-only, nothing executed.

## Verdict

The layering is right and the type additions (§3.1–3.6) are the correct additions. The expressiveness claim is not. I wrote seven programs; the failures cluster on one missing thing the design has **not** noticed: MIL/1 can state *bounds* on a signer's own outcome and nothing else. It has no term for a pool's asset side, no pre/post state, no user-defined atom, no collection, no guarded effect, no computed amount, and — because monotone completion (§7:201) admits only antitone holes — **no way to pin a solver-supplied value to an exact formula**. Every construct protects the signer; nothing protects the counterparty, and AMM, vault, auction and funding correctness all live on the counterparty's side. `CATEGORY-MAP.md` then converts that gap into a row of "Expressible" verdicts whose carriers do not carry. Before freeze I would add two-state terms, a `let`, a guarded effect, and an explicit statement that an equality-determined hole is legal — all four are U0-shaped, none touches ZKIRv3. Freeze the types; do not freeze the verdict table.

## Findings — seven programs

I use §10 syntax exactly. `-- ✗` marks a construct **not in the grammar**; that annotation is the finding.

### (d) Cross-domain burn-mint bridge with a dispute window — *works*

```
intent BridgeOut {
  version  moriarty-intent/1
  signer   owner = 0x99…
  clock    midnight.preview.clock
  validity from now for 6h
  assets {
    W = midnight.preview / policy BridgeMint / wrapped(canonical BTC @ bitcoin.mainnet) decimals 8
    N = bitcoin.mainnet  / native            / BTC                                      decimals 8
  }
  observe mintpf : Qty<N> from feed "bitcoin.spv"
          require imported(mintpf, policy = light_client_v2)
              and k_of_n(2, [ attested(mintpf, gA), attested(mintpf, gB), attested(mintpf, gC) ])
  escrow Out {                                       -- burn side; mint side is a second stage
    custody  program
    fund     1 W from owner
    release  delivered(N, >= 1 N, to owner)         -- ✗ no `on <domain>` argument in Φ:141
             and after(bitcoin.mainnet.clock@h0 + 2h)
    refund   after(validity.end)
    deadline validity.end + 1h
    on_release { supply { domain midnight.preview; asset W; delta −1 W; authority BridgeMint } }
    on_refund  { return Out to owner }
    residual   to owner
    footprint  { reads balance(midnight.preview, owner, W)
                 writes escrow(Out), supply(midnight.preview, W) }
  }
  residue { "a successful challenge inside the window has no effect line" }
}
```

The design's best category, and the type work earns it. Two gaps: `delivered` has arity 3 in `DESIGN.md:141` and arity 4 in `CATEGORY-MAP.md:174` — in the one category where domains are the point. And the dispute *itself* is inexpressible: a challenge is a third party's act, needing an atom MIL/1 cannot declare and a negation Φ does not have.

### (b) CDP mint with a liquidation path — *breaks the escrow rule*

```
  escrow Collateral {
    custody program; fund 1500 C from borrower
    release discharged(loan)
    refund  never            -- ✗ not in Φ
    deadline none            -- ✗ Instant has no `none` (§3.1:50)
  }
  authority enforce liquidation by any
    window from now indefinite                      -- ✗ `indefinite` (§10:267) not in Window (§3.1:52)
    policy { trigger  escrowed(Collateral) · px.value · 66
                   <= outstanding(loan) · 100 · 1_000_000      -- 66% LTV, cross-multiplied: OK
             requires fresh(px, 5m) and anchored(px)
             bound    seize <= outstanding(loan) · (1 + bonus) } -- ✗ `·` is not a term former (§4:126-131)
```

The trigger *is* writable — but only in cross-multiplied form, not as `CATEGORY-MAP.md:57` writes it (`mulDiv(collateral_value, ltv_max, 1)` type-errors three ways against `DESIGN.md:79`: `ltv_max` and `1` are not `Share<P>`, and there is no `collateral_value` term). The `bound` line is not writable at all: Φ has comparison over products but no product *term*, so the seize amount cannot be computed, only compared.

Worse, `DESIGN.md:187` requires `release or refund or after(deadline)` to be **valid**. With `refund never` and `deadline none` that reduces to requiring `discharged(loan)` to be valid, which it is not. **The design's headline compile-time check rejects the design's own headline lending program.** Open-term collateral has no deadline by nature; the rule as stated forces every escrow to be a timeout escrow.

### (c) Perpetual with funding and a maintenance call — *not expressible*

```
  position p : Signed<ETH> = +10 ETH
  observe mark : Price<ETH, USD, 6> from feed "eth/usd" require fresh(mark, 30s) and anchored(mark)
  observe indx : Price<ETH, USD, 6> from feed "eth/usd-index" require fresh(indx, 30s)

  continuation Funding every 8h {                      -- ✗ no Episode/continuation syntax exists
    effect transfer { domain midnight.preview; asset USDC; from ???; to ???   -- ✗ payer flips on sign
                      amount hole f : Qty<USDC> }
    guarantee f · 1_000_000 = 10 · (mark.value − indx.value) · r   -- ✗ equality on a hole: not antitone
  }
  authority enforce margin_call by any
    policy { trigger escrowed(Margin) · 1_000_000 · 100 <= 10 · mark.value · 5 }   -- this part works
```

Three independent blockers. **(i)** The Episode — the layer §2 introduces for sequencing, joins and continuations — has **no surface syntax anywhere in §10**, and half of `DESIGN.md:293`'s "closes" list rests on it. **(ii)** `Effect` (§9:212) has fixed `from`/`to` and no guard; `on_release`/`on_refund` are the only conditional effect positions in the language, and they are escrow-specific and binary. A payment whose direction depends on `mark − indx` has nowhere to live. **(iii)** the funding amount is *exactly determined*, and §7:201 requires guarantees antitone in every hole. An equality puts the hole in both polarities; the polarity check either rejects it or is vacuous.

### (e) ERC-4626 deposit including the empty-pool bootstrap — *not expressible*

```
  hole s : Share<V> where s <= 1_000_000
  guarantee s · totalAssets(V) <= 1000 U · totalShares(V)        -- ✗ no totalAssets term (§4:129)
        and (s + 1) · totalAssets(V) > 1000 U · totalShares(V)   -- floor exactness: ✗ not antitone
```

Φ projects `totalShares(P)` and nothing else about a pool. **The vault's asset side has no term, so `floor(a·S/Va)` — the one library rule the repository has already lowered to Core once — cannot be written.** `CATEGORY-MAP.md:198-200` rates deposit/redeem and empty-pool bootstrap both *Expressible*; neither is.

Note the shape of the failure. A one-sided guarantee `s ≥ …` is antitone and legal, and it is safe *for the depositor*. The clause that stops the solver minting **too many** shares protects the existing holders, who did not sign this intent. Monotone completion has nothing to say about them.

### (f) Batch auction clearing three participants — *refutes the design's own proposed answer*

```
intent BidAlice {
  budget { gross_debit <= 100 U; net >= 90 X to alice }
  hole fill : Qty<X> where fill <= 100 X
  hole px   : Price<X, U, 6> where px.value <= 1_111_111
  guarantee fill · px.value <= 100 U · 1_000_000 and fill >= 90 X
  residue { "nothing binds px to the same value in Bob's and Carol's intents" }
}
```

Each side is writable; uniform clearing is not, because **no construct relates a hole in one intent to a hole in another** and no stage binds more than one digest (§8:207, and `signer` is singular at §10:230). `CATEGORY-MAP.md:39` proposes "an Episode with a join over disjoint footprints". That cannot work: three bids all write `balance(d, pool, X)`, so by §3.6:119 they are the *maximally non-disjoint* case. The declared open item's own suggested resolution is provably the wrong shape.

### (a) Concentrated-liquidity swap with a tick range — *taker side degenerates, LP side absent*

```
  observe spot : Price<B, A, 6> from feed "dust/night" require fresh(spot, 30s) and anchored(spot)
  guarantee spot.value >= 980_000 and spot.value <= 1_020_000     -- price band, not a tick range
  escrow S { fund 1000 A from lp; release delivered(B, >= 990 B, to lp)
             refund after(validity.end); deadline validity.end + 5m }
  residue { "per-tick liquidity, tick crossing, and (x + L/√p_b)(y + L·√p_a) >= L² are not expressible" }
```

The taker's intent collapses to an ordinary net-outcome intent for *any* AMM — the outcome-first property working correctly. But an LP **range order** ("provide L between ticks a and b, accept being all-A below and all-B above") is a claim whose composition is a function of an external price, and `Share<P>` is a scalar claim with no such parameterisation. `CATEGORY-MAP.md:30` rates `x·y ≥ k` *Expressible*, carrier "the pool's own program states it". **MIL/1 defines no pool program language**, and the invariant is over pre- **and post-** state while Φ has no pre/post at all.

### (g) Flash loan — *misdiagnosed as open*

`CATEGORY-MAP.md:72-75` blames the atomic-stage decision. That is not the binding constraint. A flash loan needs `balance(d, pool, A)_post ≥ balance(d, pool, A)_pre + fee` — a **two-state predicate**, exactly what the AMM invariant row also needs. Adding `pre(term)` to Φ gives both, at U0 cost, without any intra-stage structure. Flash loans are blocked by a missing term, not by a milestone decision — and *[df]* settles flash liquidity as **lane obligations inside one atomic settlement**, with no intra-stage steps, which is the existence proof.

## Defects

1. **§6:187 exhaustiveness rejects open-term collateral.** `release or refund or after(deadline)` *valid* is only satisfiable when a finite deadline exists. Should be: every escrow has a finite deadline **and** the disjunction is valid, or the rule is reachability, not validity. As written, the check and `CATEGORY-MAP.md:44-50` cannot both stand.
2. **Φ has no negation and no user atom, and §6:185 uses both.** `eligible_for_release ≡ funded and release and not terminal` uses `not`, absent from `prop` (§4:133-143). `CATEGORY-MAP.md:109` writes `not exercised`; `:111` writes `equity(p, mark)`; `:57` writes `collateral_value`. None is a Φ term. The nine derived states cannot be derived in the language they are declared over.
3. **Two mulDiv signatures.** §3.3:79 types it `Qty<A> × Share<P> × Share<P> → Qty<A>`; §4:130 admits `mulDiv(term, term, term, rounding)` untyped. The examples use the untyped form. Pick one; the typed one cannot do price conversion and the untyped one reintroduces division. *[df]* already has the right signature — `mulDiv (mode : Rounding) (a b : Word w) (denominator : Nat)` (`lean/DefiKernel/Arithmetic/Rounding.lean` L12-21) with floor/ceil characterisation theorems and an explicit remainder law at L117/L129. §3.3:79 narrowed a general operator to a `Share`-only one and lost the whole price-conversion category with it.
4. **No computed amount anywhere.** `Effect.amount` (§9:212) is a literal or a hole; a hole must be antitone. Interest at a computed rate, funding, PnL, pro-rata redemption and uniform-price fills are all exactly-determined amounts. This is the single largest expressiveness defect and it is not on the open list.
5. **Undefined types used load-bearingly.** `Pool`, `Route` (with `.domains`/`.hops`), `Program`, `Terms`, `ClaimClass`, `Scope`, `RevocationPolicy`, `Principal`, `Policy`, `Symbol`, `FeedId` are referenced and never defined. `Route` is the central hole type in both examples.
6. **`guarantee` has no surface syntax.** §7:201 defines monotonicity over "the intent's guarantee clauses"; §10 has `budget` and `escrow.release` and no guarantee block. The static check has no object.
7. **Asset table, NFT row (`CATEGORY-MAP.md:228`) is wrong.** `Asset` (§3.2:60) has no token-id field and `balance(d, a, asset)` is the only ownership cell, so a 10 000-item collection is 10 000 unrelated `Asset` identities; "supply ∈ {0,1}" is prose with no slot to hold it. Likewise `:227`: `Signed<A>` is parameterised by *asset*, so two ETH perp venues share one type and "Σ long = Σ short per instrument" cannot be stated — `instrument` is not a type.
8. **Enforcement triggers are not required to be anchored.** §3.4:96 makes anchoring a typing rule only where a position *requires* anchored. Nothing marks `enforce` policy triggers as such a position, so a k-of-n attestation can seize collateral. Make it a typing rule.

## Missing

**Must be in MIL/1 (all U0-shaped):** `pre(term)`/two-state predicates; a `let` binding for named derived terms (this is what makes `equity`, `collateral_value`, `totalAssets` and funding writable, and it is Marlowe's `Let`); a guarded effect form; an explicit rule that an equality-determined hole is legal, with a separate soundness argument; Episode surface syntax; multi-digest stage binding, or an explicit written refusal.

**Library, once the above exist:** TWAP, fee tiers, insurance funds, vote-escrow.

**Later milestone:** a bounded collection type with a `sum` former. Without it, multi-collateral CDPs, TWAP (`CATEGORY-MAP.md:145` calls it *Library* — over what collection?), holder sets and position aggregation are all unwritable. `k_of_n(k, [prop, …])` is the language's only sequence.

**Asset taxonomy failures the three rules do not cover.** *Rebasing tokens*: a rebase writes every holder's balance with no transfer; the footprint is a finite cell set (§3.6:116) and stages are bounded, so the only sound encoding is balance = shares × index — but `balance` is a primitive Cell and `Asset` carries no index. `CATEGORY-MAP.md:223` claims LP, vault and liquid staking are "one type"; that is true of wstETH and false of stETH. *Fee-on-transfer*: `transfer` has one `amount` (§9:212), so debit ≠ credit is unstatable and the §9:221 conservation law is violated by a well-formed effect whose shortfall is imposed by the issuer, who is not a party. *Blacklist/pausable*: issuer state is not reachable from Φ, so §6's exhaustiveness is syntactic — a compile-time-exhaustive escrow can be runtime-stuck on both branches. *NFT fractionalisation*: shares work, redemption does not (a buyout is case (f)). *Off-chain obligor*: honest, but `default` as "a named predicate" (`:68`) ranges only over on-chain cells.

## Disagreements

**The five open items are the wrong five.** *Flash loans* is a decision presented as a question (§12:295 already made it) and is anyway misattributed — see (g). *Liquidation latency* is answered by §1:24's own non-goal: MIL does not model liveness. That is a closed question, not an open one. *Competing-slash ordering* is real but under-scoped: there is no concurrency semantics for enforcement at all, so competing liquidations against one encumbrance and racing escrow releases have the same status. *Policy state over reachability* is real and mis-filed under governance: it is the universal absence of user-defined state, which also blocks ticks, funding indices, exercise flags and fee growth. Only *n-party clearing* is genuinely open — and its proposed resolution is refuted above. In fairness it is hard everywhere: *[df]* clears pointwise per (lane, participant), never multilaterally, with branch arity exactly two.

**The three unnoticed items I would add:** (1) **exactness** — monotone completion admits only one-sided freedom, and market clearing is two-sided; (2) **counterparty protection has no signer** — every check in this design is the signer's, and pool/vault/auction correctness is not; (3) **asset-level transfer semantics** — the language assumes every asset is a plain conserved balance.

**Where MIL/1 reinvents something worse.** Marlowe's `When [Case action cont] timeout cont` is the guarded, timed, multi-party continuation that §2's Episode promises and §10 does not supply, and Marlowe proves the *semantic* property ("money always leaves the contract") that §6:187 approximates syntactically and gets wrong. The deeper divergence: Marlowe makes the **contract** the signed object and parties act into it; MIL/1 makes the **intent** the signed object, which is why one digest per stage is structural and why liquidation, clearing, disputes and vault fairness all land on the same rock. *[df]* `/home/charl/projects/defiformal` carries the state transition itself as a first-class typed object (`lean/DefiKernel/Typed/Transition.lean`, beside `Types.lean`, `Authority.lean` and `Acceptance.lean`), so a pre/post relation is the unit of meaning; MIL/1 discarded state to gain a bounded digest and is now paying for it in five categories. Where MIL/1 is genuinely better: the evidence class as a typing rule (§3.4:96), and time as a first-class domain-qualified type — *[df]* has only `now` as a guard variable (`lean/DefiKernel/Typed/Expr.lean` L85-86, L109), with no duration, deadline or expiry type at all.

**Residue is not MIL/1's invention, and MIL/1 took the weaker half.** It is a mandatory grammar production in *[df]*'s taxonomy (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md` L636, L691, L729-733): omitting the `residue:` line makes a molecule **ill-formed**, and recurring residue across three protocols is the promotion signal for a new element. MIL/1's real contribution is putting it inside the signed digest. But §9:223's "it never affects validity" discards exactly the enforcement *[df]* gets from ill-formedness — and *[df]* concedes its own grammar is "a convention, not a checkable artifact" (L1153), i.e. neither system has made it bite. Make residue's *presence* mandatory and its *content* free, rather than the reverse.

## Top three changes

1. **Add `pre(term)` and a `let` to Φ (§4:126-143), and state that an equality-determined hole is legal with its own soundness rule (§7:201).** These three edits, together, make the flash loan, the AMM pre/post invariant, ERC-4626 share math, funding and liquidation amounts writable. They are the cheapest high-value change in the document and all three are U0.
2. **Rewrite §6:187 as: every escrow declares a finite deadline, `release ∧ refund` unsatisfiable, and `release ∨ refund ∨ after(deadline)` valid — and state that this is syntactic reachability only.** Then fix `CATEGORY-MAP.md:44-50`, which the current rule rejects.
3. **Downgrade the wrong verdicts and say why, in the map itself:** AMM invariant (no pool program, no pre/post), LTV/health as written (three type errors at `:57`), vault deposit/redeem and empty-pool bootstrap (no `totalAssets`), periodic funding and ADL (no computed or guarded effect), NFT and position rows in the asset table. A verdict table whose carriers do not carry is worse than no table, because U6 will plan against it.
