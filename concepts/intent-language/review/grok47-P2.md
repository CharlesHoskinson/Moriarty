I'll review MIL/1 as a language of financial intention: read the design, the category map, and the constraints they have to satisfy, then test expressiveness by writing programs rather than restating the prose.The design is an intent language with a fixed predicate grammar. I'll test that grammar against the hard cases and check the category map's verdicts against the same syntax, including the comparison systems it claims to sit beside.I checked the surface syntax and the category-map examples against the grammars in `DESIGN.md` §§3–10 by hand. I did not run a parser, an evaluator, or a solver. The text is specified-only, so nothing below is a demonstrated behavior.

## Verdict

The bones are the right ones for a permissionless intent language: outcome budgets, evidence class as a type, debt kept off the token-supply equation, one conservation law per `(domain, asset)`, and no predicate that can assert an anchored fact about a foreign domain. I would not replace that architecture.

I would not freeze it. The predicate grammar cannot say the pre/post facts the category map treats as already expressible, the one `mulDiv` signature types redemption and not minting, and the binary escrow rejects the collateral, option, and dispute programs the map exhibits. Several "Expressible" rows are those missing forms under a confident name. Of the five declared-open items, one is a real composition hole (n-party clearing), one is the missing state schema wearing a governance label, and three are decisions written as if they were unsolved. Concentrated liquidity is the case that still cannot be said even after the obvious grammar patches, and the open list does not mention it.

## Findings

### The showcase fails the rules that are supposed to make it checkable

`AcquireB` (`DESIGN.md:251-256`) sets `release` to `delivered(B, >= 20 B, to owner)` and `refund` to `after(validity.end) or failed(route)`, with `deadline` an hour later. `failed` is not a production of Φ (`DESIGN.md:133-142`). Ignoring that, delivery can still hold after `validity.end`, so `release ∧ refund` is satisfiable. The showcase is illegal under the disjointness rule at `DESIGN.md:187`.

That rule is also the wrong check. "Valid" for `release ∨ refund ∨ after(deadline)` cannot mean what it says. As a tautology it rejects every interesting escrow, including this one. As mere satisfiability, `after(deadline)` makes every escrow with a deadline pass, so exhaustiveness does no work. The derived workflow state at `DESIGN.md:185` uses `not`, and §4 has no negation. The nine states are not derivable in Φ.

The same split shows up wherever an example is richer than the grammar: `attested(reserves, 3, 5)` is used as a proposition (`CATEGORY-MAP.md:134`) but `attested` is only an `Evidence` former (`DESIGN.md:91`); `delivered` takes three arguments (`DESIGN.md:141`) and is given a domain and a `via imported(...)` clause (`CATEGORY-MAP.md:173-174`); `equity(p, mark)`, `collateral_value`, `never`, and `deadline none` (`CATEGORY-MAP.md:47-48, 57, 111`) are not terms.

### Programs

**(a) Concentrated-liquidity swap.** A user-level outcome is sayable and is the wrong object:

```
intent SwapCL {
  version moriarty-intent/1
  signer trader = 0x11
  clock ethereum.mainnet.clock
  validity from now for 10m
  assets {
    USDC = ethereum.mainnet / circle / USDC decimals 6
    WETH = ethereum.mainnet / wrapped-native / WETH decimals 18
  }
  budget { gross_debit <= 10000 USDC; fees <= 30 USDC; net >= 4 WETH to trader }
  hole venue : Program where venue ∈ { uni_v3_weth_usdc_500 }
  hole route : Route where route.domains ⊆ { ethereum.mainnet } and route.hops <= 1
  observe slot0 : Price<WETH, USDC, 18> from feed "uni-v3/slot0"
          require fresh(slot0, 12s) and anchored(slot0)
  escrow E {
    custody program
    fund 10000 USDC from trader
    release delivered(WETH, >= 4 WETH, to trader)
    refund after(validity.end)
    deadline validity.end + 5m
    on_release { settle E }
    on_refund { return E to trader }
    residual to trader
    footprint { reads balance(ethereum.mainnet, trader, USDC)
                writes escrow(E), balance(ethereum.mainnet, trader, WETH) }
  }
  residue { "tick range, sqrt price and the position key are not in the language" }
}
```

`Route`, `Program`, `settle`, and `return` are undefined, and `refund` overlaps `release` as in the showcase. The economically meaningful constraint does not parse. Inside one initialized tick the amount is `Δy = mulDiv(L, sqrtP_post − sqrtP_pre, 2^96, up)`. Φ has no `pre`/`post` (`DESIGN.md:126-131`). A √price in Q64.96 is not a `Price<_,_,s>` for `s ∈ 0..18`: tick boundaries are powers of 1.0001, not 18-decimal prices, and the frozen scale (`ROADMAP.md:21`) cannot hold them. Crossing ticks is a bounded loop over `liquidityNet`. There is no loop, no array, and `policy(Id)` is a footprint cell (`DESIGN.md:115`) with no term that reads it. A position `(tickLower, tickUpper, liquidity)` is not `Share<P>`, not `Signed<A>`, and not the supply-∈-{0,1} NFT of `CATEGORY-MAP.md:228`, because liquidity is a quantity living on the unique claim. The constant-product row (`CATEGORY-MAP.md:30`) does not carry this. *[df]* correctly lists concentrated liquidity as its own element (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:186`, E007), not as a parameter of `x·y ≥ k`. Adding `pre`/`post` would repair the constant-product row and still would not repair this one. It is absent from the five open items.

**(b) CDP mint plus liquidation.** The map's program (`CATEGORY-MAP.md:43-60`) is not in the language: `refund never`, `deadline none`, `collateral_value`, and `outstanding(loan) · (1 + bonus)`. The last of these uses `·` as a term. In §4, `·` occurs only inside a comparison. The bound is also dimensionally mixed if `seize` is collateral and `outstanding` is debt.

The closest reading that respects consent and `enforce` is: borrower signs an encumbrance and an obligation; a third party may seize under a policy fixed at origination; repayment discharges and unlocks. That program needs three exits (repay, seize, and a deadline only if one exists). §6 provides `release` and `refund`. Maker-style vaults have no deadline, so `after(deadline)` cannot be the exhaustiveness witness. `DESIGN.md:295` then says a liquidation is multi-stage by construction, while `CATEGORY-MAP.md:67` says liquidation is expressible. An atomic collateral receipt is one effect bundle (`transfer` collateral, `transfer` repayment, `oblig discharge`, `encumber seize`) and fits a single stage's effect list (`DESIGN.md:211-216`). A liquidation that must swap the collateral into the debt asset is the flash-loan problem. The map does not distinguish them, so the "Expressible" cell is true only for the capital-rich liquidator who takes the collateral as-is.

**(c) Perpetual funding and maintenance.** A signed position types (`Signed<ETH>`). The funding cashflow does not. Payment is `position · (mark − index) / index` every eight hours, and the sign flips. Checked subtraction is specified for unsigned quantities; a negative premium has no result type; there is no signed scalar beside `Signed<A>`, which is defined to be a position and never a balance or a rate (`DESIGN.md:72`). There is no surface syntax for a continuation or a period ratchet, so the carrier named at `CATEGORY-MAP.md:120` ("Episode of time-gated continuations") cannot be written down. `equity(p, mark) < maintenance` (`CATEGORY-MAP.md:111`) is not a Φ atom. Margin locked until a call is again a third escrow exit.

**(d) Burn-mint with a dispute window.** Two intents, linked by an obligation, is the right shape, and per-domain conservation (`CATEGORY-MAP.md:189`) should stay. The destination condition the map writes cannot be typed:

```
observe proof : Qty<MBTC> from feed "bridge/minted"
        evidence imported(light_client_v2)
escrow Source {
  fund 1 BTC from owner
  release delivered(MBTC, >= 1 MBTC, to owner)   -- no domain, no evidence class
  refund attested(challenge, 2, 3)               -- attested is not a prop
  deadline now + 6h
}
```

`delivered` is a bare proposition (`DESIGN.md:141`). The anchoring rule (`DESIGN.md:96`) therefore does not apply to it, and a local stage can treat a foreign mint as an anchored fact. That is the laundering the type system claims to prevent. Optimistic finality is a partition: after the window, exactly one of {finalized, challenged} holds. Φ can state that partition only by reading a single-valued status. It cannot read `policy(Id)`, and it cannot say `not challenged`. `CATEGORY-MAP.md:186` calls the dispute window expressible via `after` and `k_of_n`. Those two operators do not name the partition. This row is the policy-state open item, mis-filed as done.

**(e) ERC-4626 deposit, redeem, empty pool.** Redeem matches the one declared operator. `mulDiv : Qty<A> × Share<P> × Share<P> → Qty<A>` (`DESIGN.md:78`) is `totalAssets · userShares / totalShares`. Deposit is `assets · totalShares / totalAssets`, whose divisor is a `Qty` and whose result is a `Share`. No such operator exists, so the mint the signature was introduced to make typeable does not type. The virtual-offset bootstrap `mulDiv(assets, totalShares+1, totalAssets+1, down)` is the right empty-pool formula — one expression, no branch — and it needs that missing operator plus `pre`/`post`, because the totals are opening balances and the shares credited are a closing balance. `delivered` pays an `Asset`, and `Share<P>` is not one (`DESIGN.md:74, 141`), so the depositor's `net >= … to owner` budget cannot state receipt of shares. `totalShares` as a projection rather than a sum over holders is the right call (`DESIGN.md:79`); the missing piece is the mint direction and a coherence law that the issued token's `supply` equals `totalShares(P)` when the claim is also a token.

**(f) Three-party batch.** Each side is a legal outcome intent: Alice funds 10 A for at least 5 B, Bob funds 10 B for at least 8 A, Carol funds 4 A for at least 3 B. Uniform clearing is one price π against which all three minimums hold. No intent can constrain another intent's price. All three write the auction's balances, so the independence test (`DESIGN.md:117`) fails and a join over disjoint footprints (`CATEGORY-MAP.md:39`) does not apply. "A swap, an RFQ fill and a batch-auction clearing are the same intent" (`CATEGORY-MAP.md:26`) is false for the batch. An RFQ is that shape. A uniform clear is a stage that binds a bounded list of digests and checks each release formula against one post-state. N = 3 is inside any fan-in bound the roadmap already contemplates (`ROADMAP.md:26`).

**(g) Flash loan.** The lender's predicate is `post(balance(pool, A)) ≥ pre(balance(pool, A)) + fee` and no residual flash obligation. That is the same two-state atom as `x·y ≥ k`, and *[df]* already treats a transition as a net effect plus a guard, with authority applied to net debits rather than to an intermediate trace (`lean/DefiKernel/Core.lean:10, 49-55, 66-67`). I found no flash-loan development there and did not execute the kernel. Given multi-line effects (`DESIGN.md:211-216`), a plan whose lines borrow, swap, and repay is one stage if non-negativity and conservation are demanded of the net. `CATEGORY-MAP.md:72-75` calls this "exactly the intra-stage structure this design excludes." That follows only if each opaque `Program` call is its own stage. The design never chooses between "a stage is an effect list" and "a stage is one external call." Until it does, "Open" is an undecided semantics, not a gap in Φ. Arbitrary callbacks should stay excluded: a stage still needs a checked bound (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:66`).

### Verdicts that should move

| Row | Now | Should be |
|---|---|---|
| `x·y ≥ k` over pre/post (`CATEGORY-MAP.md:30`) | Expressible | Inexpressible until `pre`/`post` exist |
| LP / vault mint (`:32`, `:198`) | Expressible | Ill-typed; only the redeem direction matches `DESIGN.md:78` |
| Liquidation, unbonding-with-slash (`:67`, `:204`) | Expressible | Blocked by the two-exit escrow |
| Default (`:68`) | Expressible | No such atom in §4 |
| Pooled bad-debt socialization (`:71`) | Expressible | Needs a bounded fold over holders; a single creditor is fine |
| Periodic funding, margin (`:120-119`) | Expressible | No signed rate, no episode syntax, no `equity` |
| Dispute window, fast fill (`:186-187`) | Expressible | `delivered` has no evidence class; the partition is unreadable policy |
| Options, perps, ACTUS payoffs as library (`:123`; `ROADMAP.md:47`) | Library | Payoff formulas are library; contract status is the policy-state hole |
| Parameter change (`CATEGORY-MAP.md:162`) | Partly open, effects expressible | §9 has no policy-update effect |
| Surplus allocation, bundled with MEV (`:36`) | Open | A split of `post` output is a formula once `pre`/`post` exist; ordering residue can stay |
| RFQ, bundled with batch clearing (`:35`) | Partly open | One maker intent with a fixed minimum is expressible |

Fee tiers, k-of-n observations, structured asset identity, and "debt is not a supply delta" (`CATEGORY-MAP.md:224`, matching `docs/MORIARTY-PRODUCT-CONTRACT.md:43`) are carried by what is actually written.

### Where the three asset rules stop

Identity = domain, issuer, representation; one conservation equation per `(domain, asset)`; `issue` for units and consent for liabilities (`CATEGORY-MAP.md:231`). That covers hook-free on-ledger fungibles, distinct wrapped versus canonical assets, and obligations.

It fails in four places.

- **Transfer hooks.** Fee-on-transfer, blacklist, and pause are balance changes, or refused changes, that the effect list does not determine. The three rules assume Σ balance deltas equals the declared supply delta because every delta was written down. An external token with a hook makes that assumption false. The local AnomaPay review already recorded quantity-versus-custody failure for fee-on-transfer and rebasing (`deliverables/anoma-study-2026-09-19/PDF-REVIEW.md:11`). Identity has to carry a transfer-policy commitment, or the asset class has to be closed to hook-free tokens. Pause and blacklist are also the unreadable `policy(Id)` cell, so this is not fixed by a library.
- **Rebasing balances.** A holder-universal rebase is not a bounded effect list. The honest encoding is `Share<P>` with a stored index, which the share row allows. The "fungible issued" row (`CATEGORY-MAP.md:219`) does not say so, and a developer will classify stETH as a normal token whose balances move by themselves.
- **A traded claim on an off-chain obligor.** The real-world row (`CATEGORY-MAP.md:229`) uses `Asset{issuer: Policy}` plus `issue` and an attestation, and then says the obligor is "in the asset identity." Creating units is the `issue` rule. Creating a liability is the consent rule. A receivable that trades is both, and the row picks only the first, so the product-contract separation is lost for the asset class that most needs it. Consent's evidence class is also unspecified: an off-chain obligor cannot sign a Midnight stage, and `Authority` has no `imported`/`attested` bit (`DESIGN.md:154-156`).
- **Range claims and open collateral sets.** The twelve categories have no object for `(liquidity, tickLower, tickUpper)`. Multi-collateral with a fixed list is an explicit sum of encumbrances that each name the same `ObligationId` (the field is singular, the records need not be). An open list of collateral types needs a fold. Φ's additions are binary (`DESIGN.md:130`). Aave-shaped LTV is not the single-collateral formula at `CATEGORY-MAP.md:57`. NFT fractionalization does fit: one symbol per token id, supply in {−1,0,+1}, lock plus `issue` of shares, with royalties falling under the hook gap rather than a new kind.

### Holes

`DESIGN.md:201` says that for any filling σ, `Φ_guarantee[σ] ⟹ Φ_guarantee`. If the right-hand side still contains the hole, that is not a sentence. The property that can actually be checked is: a hole is an existential witness; a filling substitutes a closed value; the executions of the filled intent are a subset of the executions of the open intent; a hole is illegal in a position where a larger value admits a debit, a fee, a window, or a recipient the smaller value excluded.

Under that reading, monotonicity is a safety feature. A solver-chosen fee cap, an extendable window, and a recipient set the solver can grow are exactly the weakenings `docs/FOOTGUNS.md` §13 and the product contract forbid. "This fill was the best available route" is also inexpressible, because it quantifies over routes not taken. No honest circuit has those routes. That is a limitation only for a checked best-execution promise, which the language should keep refusing.

Monotonicity is not why batch matching fails. A uniform price, or "my order clears only if Alice's does," is a constraint across digests. It is not a polarity bug in one hole. Split routes are not forbidden either. They are unrepresentable: `route.hops <= 3` (`DESIGN.md:245`) describes a path, and `Route` has no constructors. `DESIGN.md:199` says everything that is not a hole is fixed, and §11 says the holes are the search space. An undefined hole type means there is no search space to check polarity against. Either `Route` is a closed grammar that can name a split, a tick limit, and a partial-fill bound, or the route hole is doing no work and the plan is an unconstrained episode checked only against the budget.

## Defects

These are wrong as specified, not merely unfinished.

1. **The exhaustiveness check has no reading under which both the prose and the §10 program survive** (`DESIGN.md:187` against `:251-256`).
2. **`mulDiv`'s type does not type minting** (`DESIGN.md:78`). The staking and vault rows that cite it as the carrier of deposit and of redeem are wrong in one direction.
3. **Effect-propositions bypass the anchoring rule** (`DESIGN.md:96` versus `:141`). Cross-domain `delivered` can appear where an anchored fact is required.
4. **The monotonicity entailment is ill-formed** (`DESIGN.md:201`), so the static check §7 and §11.3 treat as the safety argument cannot be implemented from the text.
5. **Liquidation is both "multi-stage by construction" and "Expressible"** (`DESIGN.md:295`, `CATEGORY-MAP.md:67`). A freeze cannot contain both.
6. **`Share` addition was fixed by dropping the holder index, and the mint operator was not added.** That is a type error in the repair itself, not an open economic question.

## Missing

**In MIL/1, because U0 hash-binds the grammar** (`DESIGN.md:303-310`): `pre`/`post`; negation of atoms; `attested(obs)` as a proposition; evidence class on `delivered`, `discharged`, and `seized`; the mint-direction operator returning `Share`; a finite exit list in place of `release`/`refund`; a defined `Window` form for the indefinite recovery the showcase already writes (`DESIGN.md:267`) and the architecture requires (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:70`); elapsed time as `Instant − Instant`; a signed scalar for rates; and closed grammars, or an explicit stub, for `Route`, `Program`, `Pool`, `Terms`, and `ClaimClass`. A `batch: admit | refuse` bit belongs on the canonical intent even if the clearing stage is later. Leaving it out and adding it after the digest is frozen is a second intent language.

**Library, once those exist:** fee tiers, ERC-4626 with a virtual offset, RFQ, insurance-fund balances, ACTUS and perp *payoff* formulas, surplus splits.

**Later milestone:** the multi-digest clearing stage (U3), episode syntax and joins (U3, where `DESIGN.md:306` already puts them), federated routing (U5). Full ACTUS calendars and day-count conventions are U6 only after policy state is readable. They are not a library over today's core.

**The policy schema is the open item that swallows the others.** A finite, signer-committed record — status enum, fee index, funding index, tick `liquidityNet`, blacklist commitment — readable in Φ and updatable only by `amend` or `enforce` inside the consented window, is the carrier for dispute partitions, option exercise, pause, compounding, and concentrated liquidity. It is not a governance extra. Without it, "library" is a label for programs that cannot be written.

## Disagreements

**n-party clearing** is genuinely open, and the text's candidate answer should be withdrawn. Disjoint join cannot clear a shared auction (`CATEGORY-MAP.md:39`). The construct is a bounded multi-intent stage. Anoma's prior art is candidate composition into one balanced transaction, kept distinct from a committed partial stage (`deliverables/anoma-study-2026-09-19/SYNTHESIS.md:17`). MIL is ahead of that prior art on residual duties (`SYNTHESIS.md:23`) and behind it on the settlement object: "a stage binds one signed intention" (`CATEGORY-MAP.md:39`) makes E011 (`UNIFIED-DEFI-ELEMENT-TABLE.md:204`) inexpressible by decree.

**Flash loans** should not sit on an open list. Decide the stage: net effect list, or one opaque call. The first makes the lender predicate expressible as soon as `pre`/`post` exist, and matches *[df]*'s element E040 as an abstract atomic scope (`UNIFIED-DEFI-ELEMENT-TABLE.md:218-227`) rather than as an EVM callback. The second also makes swap-liquidation inexpressible, and the lending row has to say so. Unbounded callbacks stay out either way.

**Liquidation latency** should be closed by the non-goal the design already states (`DESIGN.md:25`). A bound on how fast an enforcer must act is liveness. A timeout still does not prove non-execution (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:64`). What is expressible, and enough, is a price on delay: a bonus computed from a fresh observation, inside a signed `enforce` window.

**Competing slashes** are a one-line rule, not a research question. An episode order applies slashes sequentially. Two slashes in one stage allocate pro-rata on `pre(totalShares)`, with dust in `retained-in-pool`. Deciding that is compatible with everything else in §3.3. Leaving it "open" (`CATEGORY-MAP.md:206`) invites a second semantics for the same cells.

**Policy state over reachability** is the right open problem and is filed too narrowly, under governance (`CATEGORY-MAP.md:162, 249`). It is the reason concentrated liquidity, funding indices, optimistic bridges, option status, pause, and ACTUS lifecycle are not expressible. I agree that governance *process* (proposals, vote-escrow) is a library. I do not agree that governance *effects* are already in the language: there is no policy arm in `Effect` (`DESIGN.md:211-216`).

**Placement.** `DESIGN.md:310` says the U0 item is "the whole of it." The U0 slice this design has to land in still excludes division, prices, and escrow (`deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md:75-76`). Freeze the corrected intent grammar and the batch bit. Do not freeze `Route` holes, episode joins, or a scorecard that counts carriers the grammar does not contain.

**Marlowe and Daml already had the exit form this design narrowed away.** Marlowe's `When` takes a list of `Case`s (deposit, choice, notify) and a timeout continuation; `If` and `NotObs` are in the observation language. Daml puts consent on `signatory` and third-party action on `controller`, with one template and N choices. MIL's `enforce` right is that controller, and §6 then gives the controller nowhere to go that is not `release` or `refund`. That is the regression. MIL is ahead of both on evidence classes, domain-qualified clocks, checked finite-width arithmetic, and proof-carrying acceptance, and ahead of *[df]*'s expression language on refusing rationals (`lean/DefiKernel/Typed/Types.lean:28-32`, quote-per-base at `:40`). It is behind *[df]*'s `Expr` on `not`, `neg`, and `ite` (`lean/DefiKernel/Typed/Expr.lean:34-36, 92`). Take those three operators over checked integers. Leave the rationals. ERC-4337 signs calldata; MIL signing an outcome is the better intent. The x402 lesson in the product contract — payment is not delivery — is only half applied until `delivered` carries an evidence class.

## Top three changes

1. **Rewrite the §4 productions and the `mulDiv` signature before anything is hash-bound.** Add `pre(t)`, `post(t)`, and `not` restricted to atomic propositions. Add `attested(obs)` as a proposition. Give `delivered`, `discharged`, and `seized` an evidence argument so the anchoring rule covers them. Add `mulDiv` of type `Qty<A> × Share<P> × Qty<A> → Share<P>` with an explicit rounding role and `retained-in-pool`. Without this edit the constant-product row, the vault row, and the dispute row cannot become true.

2. **Replace `release` / `refund` / `deadline` in §6 with a finite list of named exits, each a Φ formula plus an effect block.** Require the formulas pairwise unsatisfiable, and require the disjunction satisfiable in the stage's finite model; delete the word "valid." One case may be a consented `enforce` (seize, slash, margin call). Rewrite `AcquireB` so that delivery and the timeout do not overlap. This is the edit that makes the CDP, the covered call, unbonding-with-slash, and the dispute window programs in the language they are claimed to inhabit.

3. **Replace the five open items and the scorecard with a decision record, and give holes a semantics that is a sentence.** Close latency as refused liveness. Close same-stage slashing as pre-state pro-rata. Decide the flash-loan stage explicitly and make the lending row match that decision. Keep n-party clearing open, delete the disjoint-join candidate, and reserve a batch bit on the canonical intent. Widen "policy state" so it is the named carrier for ticks, funding indices, pause, option status, and ACTUS status, and demote every Expressible row whose carrier is `equity`, an episode, a route, or an unreadable policy. State hole-filling as existential instantiation with admitted-set inclusion, and either define `Route` or remove route holes from the U0 syntax.
