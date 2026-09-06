# The Generation Theorem

## 1. What "generated" has to mean

A protocol is a state machine, so "lies in the closure of `P` under composition"
needs a reading. Three were on offer.

**(a) Dataflow-graph composition.** Worthless, and demonstrably so. A dataflow
graph records arity and wiring, nothing else. `collValue(amount, price) = amount
* price` and `cpProduct(r0, r1) = r0 * r1` have the identical graph; so does
every binary operation in the corpus. I ran the check that reading licenses: it
matched 29 of the corpus's 64 raw arithmetic sites to `collValue` alone, because
`collValue`'s body *is* the pattern `X * Y`. Under (a) a basis of one binary
operator generates everything. This is Post's `P = {⊤}` failure exactly.

**(b) Bisimilarity of transition systems.** The strongest reading, and right in
principle: there is a `P`-term whose denotation is bisimilar to the protocol's
transition relation. It is not one I can discharge — bisimulation between two
388-variable systems over unbounded integers is not a finite exact computation.
I take (b) as the target that (c) approximates from below.

**(c) Every definition is a `P`-term up to renaming.** This is checkable, and it
is non-degenerate **provided arithmetic is not free**. Naively, (c) is vacuous:
every Quint definition is by construction a finite term over Quint builtins, so
"reducible to primitives plus arithmetic" is true of everything. The whole
question is what "plus arithmetic" admits.

So I fix the reading by splitting the builtins.

> **Definition.** Let `Π` be the *plumbing*: boolean connectives, comparison,
> conditional, additive integer arithmetic (`+`, `-`, unary minus), and data
> movement (records, tuples, sets, maps, `get`/`put`, `fold`, `filter`,
> quantifiers, action combinators, assignment, `oneOf`). Let
> `Π_hard = {*, /, mod, pow}`.
>
> A definition `d` of protocol spec `S` in lane `ℓ` is **`P`-generated** iff
> every operator applied in `body(d)` lies in `Π ∪ B_ℓ ∪ G(S)` — where `B_ℓ` is
> the set of operators declared in `ℓ/common.qnt` and `G(S)` is the set of
> definitions of `S` already shown generated, under a well-founded order — and
> **no operator of `Π_hard` occurs in `body(d)` outside a `B_ℓ` call.**

The load is carried by the last clause. Additive arithmetic is inert: a ledger
is an additive monoid, and every one of the eight families presupposes that
carrier. **Multiplicative and divisive arithmetic is not inert — it is precisely
what the families are.** `a*b/d` is pro-rata (F1). `x * index / BASE` is index
accrual (F7). `c*p*ltv ≥ d*p*10⁴` is the health predicate (F5). If `*` and `/`
were free plumbing, F1, F5, F7 and F8 would all be plumbing, and the theorem
would say nothing. Banning `Π_hard` outside a basis call is what makes an
unbounded family of definitions capable of failing.

Two corollaries of the definition, both enforced in the check:

- **Non-substantive basis members cannot serve as templates.** A basis operator
  whose body is a single builtin applied to distinct bare metavariables is a
  *rebadged builtin*, not a mechanism. Thirteen lane primitives are of this
  form — `collValue(a,p) = a*p`, `cpProduct(r0,r1) = r0*r1`,
  `applyEscrowDeposit(e,a) = e+a`, and ten more. They are excluded from
  matching, and their existence is itself evidence.
- **Closed arithmetic is not a mechanism.** `100 * 100`, `BASE_BPS / 2`,
  `DELEGATE_COVER / 2` compute numbers from literals. Three such sites are
  excluded as constant folding.

What (c) lets through that (b) would not: **anything whose content is temporal
rather than algebraic.** A two-phase request/claim and an immediate claim have
the same terms and different transition systems. This blind spot is real and I
report where it bites (§6).

## 2. The theorem

> **Generation.** For every application `A` in the corpus with a Quint
> formalisation `S_A` in lane `ℓ`, every definition of `S_A` is `P`-generated in
> the sense of §1, where `B_ℓ` realises only families of `P`.
>
> **Non-degeneracy.** `Π_hard ∩ Π = ∅`, and no member of `B_ℓ` used as a
> template is a rebadged builtin.
>
> **Independence.** For each `F ∈ P`, some operator realising `F` is not a term
> over `(P \ {F}) ∪ Π`.
>
> **Refutation condition.** The theorem is refuted by exhibiting a DeFi
> application and a definition in its specification containing a `Π_hard`
> operator outside every `B_ℓ` call, where no operator realising a family of `P`
> has that content.

That last clause is what makes this a theorem rather than a slogan, and it is
satisfied. Nineteen such witnesses exist.

## 3. The check

All 57 specs parse under `quint 0.32.0`; IR extracted with `quint parse --out`.
Fifty-one are protocol specs, six are lane `common.qnt`. **820 definitions**
tested (the six commons are the basis and are not tested against themselves).

| | count |
|---|---|
| definitions tested | 820 |
| `P`-generated | **743** |
| not generated | **77** |
| — of which fail on their own body (root cause) | 37 |
| — of which fail only through a dependency | 40 |
| specs with zero violations | **27 / 51** |
| specs with residue | 24 / 51 |

Of the 40 transitive failures, 24 are the `step` action — the nondeterministic
driver that disjoins every other action, so it inherits any failure in the spec.
It is an artefact of the model, not a finding.

The root cause is small and fully enumerable: **46 maximal violating subterms**
(maximal = the outermost `Π_hard` node outside a basis call; nested products
inside one are part of the same violation). Forty-six sites is the whole
residue, and every one is listed below.

## 4. The residue, item by item

Each of the 46 is either a basis operator the spec wrote out by hand
(**INLINE** — the basis covers it, the spec is sloppy) or content no operator in
the lane basis and no family in `P` possesses (**MISSING** — the basis must
grow).

**INLINE: 27.** These re-fold. Eleven are `x * y / d` — the F1 kernel — written
raw: `babylon.slash` `(sats * SLASH_FRACTION_BPS)/BPS`, `crvusd.health`
`(debt * (WAD - LIQ_DISCOUNT))/WAD`, `eigenlayer.slash` ×3,
`eigenlayer.sharesToUnderlying`, `eigenlayer.underlyingToShares`,
`etherfi.rebase`, `usdd_psm.buyGem`, `usdd_psm.sellGem`, `liquity.redeem`. Eight
more are fixed-ratio scalings (`uniswap_v3` `amountIn/1000`, `panoptic`
`collat/2`, `kyber` `amountIn*2`, `fluid` `rate/2`, `beefy` `pool*shares`,
`apex` `baseIn*lpShares`, `huma` `seniorRedeemReq*fillS` and `juniorRedeemReq*fillJ`).
Eight are index arithmetic that `presentFromScaled` / `scaledFromPresent` /
`accrueIndex` already provide: `justlend` ×5 (`principal*globalIdx`,
`amount*INDEX_BASE`, `seizeUnderlying*INDEX_BASE`, `cTokens*rate`,
`((cash+borrows)-reserves)*INDEX_BASE`), `justlend.accrueBlock`
`(totalBorrows*rateBpsPerBlock)*blocks`, `maple.accrueLoan`, and `convex.pending`
`stakedU*(ix-paid)` — the reward-debt accumulator, which is F7 then F1.

**MISSING: 19.** These are the result.

| # | spec · definition | term | family that must be added |
|---|---|---|---|
| 1 | `L1/compound_v3.absorb` | `col * collPrice` | N2 mark-to-market |
| 2 | `L1/morpho_blue.liquidate` | `seizedCol * collPrice` | N2 |
| 3 | `L1/morpho_blue.liquidate` | `(seizedCol * collPrice) * 10000` | N2 |
| 4–6 | `L3/gmx.increaseLong`, `.increaseShort`, `.oiCovered` | `poolAmount * markPrice` | N2 |
| 7 | `L3/gmx.oiCovered` | `impactPool * markPrice` | N2 |
| 8–10 | `L6/polymarket.tradeYes` (×3) | `yesAmt * price` | N2 |
| 11 | `L2/liquity.redeem` | `(boldAmt * WAD) / price` | N2, inverse |
| 12 | `L5/derive.openOption` | `qty * MAINT_PER_SHORT` | N2, per-unit margin |
| 13 | `L1/uniswap_v2.mint` | `(reserve0 + a0) * (reserve1 + a1)` | N1 trading function |
| 14 | `L1/uniswap_v2.burn` | `(reserve0 - a0) * (reserve1 - a1)` | N1 |
| 15 | `L3/apex.swapOut` | `y * dx` | N1 |
| 16 | `L3/apex.addLiquidity` | `baseIn * reserveQuote` | N1 |
| 17–19 | `L1/curve.approxD` | `2 * min(x,y)`, `amp * 2`, `((ann*S) + prodTerm)/(ann+1)` | N1, StableSwap `D` |
| 20 | `L3/huma.depositSenior` | `MAX_SENIOR_RATIO * max(1, tranches.junior)` | N4 tranche subordination |

Three distinct families, and the diagnosis for each is different.

**N2 — valuation application, `(A, P) → V`.** Nine of the nineteen. `P` contains
F6, *valuation source*, signature `() → (P)`: it produces a price. **Nothing in
`P` consumes one.** Mark-to-market — the step from a quantity and a price to a
value — is the single most common operation in the corpus and it is not a
family. L1 and L3 noticed and added `collValue` and `positionValue` to their
commons; both are rebadged `*`, which is why the strict check rejects them. This
is a genuine missing primitive and it is not exotic.

**N1 — trading function.** Seven. Constant product and StableSwap `D` are the
rule by which a pool quotes a price. `P` has no such family. F8, *payoff*, is
`(P) → (A)`: it consumes a price. N1 *produces* one from reserves. `curve.approxD`
is the extreme case: a Newton iteration solving an implicit invariant, with no
counterpart anywhere in the eight.

**N4 — tranche subordination.** One, `huma.depositSenior`, enforcing
`senior ≤ k · junior`. L3's common already carries `totalTranche`, `applyLoss`,
`applyProfit`, `seniorFirstRedeem` — a complete waterfall family that `P` does
not contain.

## 5. The converse check, which is worse

Running generation forward asks whether the specs stay inside the basis. The
converse asks whether the *basis* stays inside `P`. I classified all **183**
distinct operators declared across the six `common.qnt` files:

| | count |
|---|---|
| realise a family of `F1..F8` | 101 |
| neutral kernel (`min`, `max`, `abs`, `WAD`, `RAY`, `BPS`, scale constants) | 8 |
| sum-type constructors | 17 |
| **outside `P` entirely** | **57** |

Fifty-seven lane primitives — 31% — belong to no family of `P`. They fall into
ten groups: **N1** swap (3), **N2** mark-to-market (3), **N3** utilization rate
curve (2), **N4** tranche (4), **N5** custody/escrow/wrap (12), **N6** once-only
delivery (2), **N7** limit order with partial fill (6), **N8** delegated
authority (6), **N9** the plain transfer/mint/burn ledger (14), **N10**
authorization and allowances (5).

N1, N2 and N4 were rediscovered independently by the forward check. The other
seven never appear in the forward residue for a mechanical reason: their content
is relational, not arithmetic, so the `Π_hard` test cannot see them — a custody
wrap and a delegation grant are both `put` into a map. This is reading (c)'s
blind spot, and it means 57 is a floor on the shortfall, not a ceiling.

## 6. Independence

For each family, is some operator realising it irreducible over the others plus
plumbing, with `Π_hard` banned?

| family | ops | verdict |
|---|---|---|
| F1 pro-rata | 26 | independent (21 irreducible; `mulDivDown` needs raw `*`,`/`) |
| F2 deferred claim | 19 | **dependent — no independent term content** |
| F3 rate limit | 17 | independent (8; `currentLimit` needs raw `*`) |
| F4 conservation | 15 | independent (1; `supplyConserved` via `sumMap`) |
| F5 health | 11 | independent (9; `isHealthy`, `collateralRatio`, `seizeCollateral`) |
| F6 valuation source | 2 | **dependent — no independent term content** |
| F7 index accrual | 17 | independent (11; `accrueIndex`, `accrueByIndex`) |
| F8 payoff | 7 | independent (3; `redeemPayout` needs raw `*`) |

F2 and F6 fail. Every F2 operator — `canRequest`, `applyRequest`, `canClaim`,
`applyClaim`, `enqueue`, `headReady` — is a record update and a comparison: pure
plumbing. F6's two operators post an exogenous number. **Neither has any term
content at all.** Under reading (c) they are not primitives and must be dropped.

I do not think they should be dropped, and this is the honest limit of my
reading. F2's content is that the claim is *deferred* — a temporal separation
between request and settlement, invisible in a term and visible in a transition
system. F2 is a family under (b) and not under (c). F6 is weaker still: "a price
arrives from outside" is a signature, not a mechanism.

## 7. Verdict

**`P` does not generate the corpus.**

Of 51 formalised protocols: **27 generated outright**, **14 more generated once
inlined basis calls are re-folded** — 41 of 51 — and **10 not generated at any
reading**: `compound_v3`, `curve`, `morpho_blue`, `uniswap_v2`, `liquity`,
`apex`, `gmx`, `huma`, `derive`, `polymarket`. Uniswap V2 and Curve are not
edge cases; they are the two largest AMMs in the corpus, and both fail on the
trading function.

The refutation condition is met, and the refuting objects are named: the 19
subterms of §4. The basis must gain at least **N1 trading function**, **N2
valuation application**, and **N4 tranche subordination**, and the converse
check says at least seven more — N3, N5, N6, N7, N8, N9, N10 — are needed for
`B_ℓ` itself to be a `P`-realisation.

Two scope facts must not be buried. The corpus names 72 applications and only
51 have specs; the theorem is untested on 21, and one of the untested is
**Steakhouse Financial**, whose mechanism is exactly N8, delegated allocation
authority. It is a $3.08B application in the corpus that `P` does not generate,
and formalising it is not needed to see that. Second, F2 and F6 fail
independence under the reading I chose, so `P` as stated is neither complete
nor irredundant.

This is a positive result. The eight families cover 101 of 183 lane primitives
and 41 of 51 protocols. Ten named families close the gap. **The basis is
`F1, F3, F4, F5, F7, F8` (six, independent) plus `N1..N10`, with F2 and F6
readmitted only under a transition-system reading** — sixteen to eighteen
families, not eight, and now with an exact list of what each must do.
