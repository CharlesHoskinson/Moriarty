# Moriarty Intent Language (MIL/2) — design

**Date:** 2026-09-28
**Status:** specified-only proposal. Supersedes [MIL/1](DESIGN.md), which nine reviewers unanimously found unfreezable. Nothing here is implemented and no milestone has accepted it.

**Basis.** Cut against the [nine-reviewer review](review/REVIEW-REPORT.md) rather than patched in place, because the repairs interact.

## 0. Claim-status convention

MIL/1's central failure was stating theorems it had not checked. Every formal claim in this document carries one of three tags, and **no untagged claim is load-bearing**:

- **[checked]** — verified against the files or by construction, in this document or the review.
- **[obligation]** — a property MIL/2 requires and must be proved before freeze; named, with its proof route.
- **[deferred]** — deliberately not claimed at this version.

## 1. What changed, and why

| MIL/1 defect | MIL/2 repair | § |
| --- | --- | --- |
| Escrow meta-rule rejects its own examples, forbids the waiting state, forbids the U3 late race | Escrow is a **transition relation** with a legitimate pending state; exclusivity is per-witness in-circuit; recovery viability is a separate named obligation | §7 |
| Monotone completion false and not a sentence | Replaced by **acceptance refinement** checked per filling; polarity demoted to an authoring lint with a variance table | §8 |
| Anchoring rule erased by every eliminator | Evidence is a **type index with a source-set effect**; `anchored` is a typing side condition bound in-circuit to an authenticated read, not a Φ proposition | §5 |
| Φ untyped, partial, and not the language the examples use | Formation judgments; total evaluation into `Value \| Reject`; atomic negation; `pre`/`post`; missing atoms added | §4 |
| Decidability claim wrong | **Φ₀ / Φ₁ split**: Φ₀ is solver-free, Φ₁ is reserved behind a version flag | §4.4 |
| Cross-multiplied `u128` exceeds the 255-bit field | `Qty` width declared; **limb rule** for products that do not fit; nonlinear node cap | §4.5 |
| Footprint cells omit receipts, encumbrances, positions, shares — linearity defeated | Complete cell vocabulary; footprints **derived and checked** against declaration; fork partitions the linear context | §9 |
| §12 fallback evaluates a subset of a signed relation | **Deleted.** A clause not evaluated is not in the relation | §11 |
| Episode ambiguous between ledger-linked and in-circuit | **Ledger-linked**, with named link fields; in-circuit parent verification stays U4 | §2, §14 |
| `unbounded_within`, asset identity, `Share<P>`, `Signed<A>`, encumbrance arity, L7 regression, residue-as-escape-hatch | All corrected | §3, §10 |

## 2. Layers

```
  Intent (MIL/2)   signed template: constraints, holes, escrow relations, residue
      │  completion σ (a solver fills holes)
      ▼
  Plan             template + σ. Checked, never trusted.
      │  acceptance relation
      ▼
  Episode          a ledger-linked sequence of stages, joined by authenticated commitments
      │  one proof per stage
      ▼
  Stage            one accepted transition on one domain
```

**An Episode is not one proof. [checked]** A stage awaiting evidence has no witness yet and ZKIR cannot pause. Each stage is proved separately and linked by ledger-authenticated commitments: `episodeId`, `predecessorHead`, `intentDigest`, `cumulativeGrossDebit`, `cumulativeFees`, `obligationRollForward`, `terminalTombstone`. This is ledger induction, which the architecture permits for a Midnight-resident lineage and which U2 may ship.

## 3. Types

### 3.1 Identity

```
AssetId   ::= opaque immutable ledger identifier          -- nominal, not structural
Asset     ::= { id: AssetId, domain: Domain, issuer: Issuer,
                repr: Repr, symbol: Symbol, decimals: u8 }   -- symbol/decimals are checked metadata
PoolId    ::= opaque identifier of a pool instance
ShareClass::= identifier within a pool
Instrument::= opaque identifier (underlying, expiry, strike, side-convention)
```

Two deployed tokens may share domain, issuer, symbol, representation and decimals; structural identity conflated them. **[checked]** Identity is nominal; the structured fields are checked metadata, and `repr` still distinguishes canonical, wrapped, shielded and synthetic with a declared link.

### 3.2 Quantities

```
Qty<A>              -- checked unsigned, width u128, in A's smallest unit
Delta<A>            -- checked signed balance/supply change
Position<I>         -- checked signed exposure in Instrument I
Price<B, Q, s>      -- base-per-quote, s ∈ 0..18
Share<P, C>         -- claim against pool instance P, class C
Instant(clk)        -- domain-qualified by clock
Duration(clk)       -- clock-indexed, so subtraction is well-typed
Window(clk)
```

`Signed<A>` is split: **`Delta<A>` for supply and balance changes, `Position<I>` for exposure. [checked]** MIL/1 used one sort for both and keyed positions by asset, so two instruments on one underlying were the same position.

Literal elaboration is defined: `11 A` elaborates to `11 · 10^A.decimals` smallest units, and `now` elaborates to `Instant(intent.clock)` resolved at signing. Both were undefined in MIL/1 and made an off-by-`10^decimals` intent well-typed. **[checked]**

### 3.3 Pool arithmetic

```
totalShares : PoolId × ShareClass → Share<P,C>
totalAssets : PoolId × AssetId    → Qty<A>
sharesFor   : Qty<A> × Share<P,C> × Qty<A> × Rounding → Share<P,C>   -- deposit
assetsFor   : Share<P,C> × Qty<A> × Share<P,C> × Rounding → Qty<A>   -- redeem
```

Both directions, with `totalAssets` present. MIL/1 had one `mulDiv` that typed redemption only. Zero-supply is a **defined branch**, not a comment: `sharesFor` with `totalShares = 0` is the bootstrap case and must name its rule (virtual offset or first-depositor convention) in the pool's declared policy. Remainder class `retained-in-pool`. **[checked]** These are Φ₁ terms (§4.4).

### 3.4 Obligations and encumbrances

```
Obligation  ::= { id, debtor, creditor, asset, principal, accrued, outstanding,
                  terms, rank: ClaimClass, consent: ConsentRef }
Encumbrance ::= { id, owner, asset, amount, against: {ObligationId},
                  priority: u8, enforceable_by: AuthorityRef }
```

`against` is a **set** with a priority, so one collateral position may back several obligations — MIL/1's single `ObligationId` contradicted its own category claim. Liability creation carries a `ConsentRef` to the debtor's signature. The **AccrualFirst component split is restored**: `dA = min(nominal, accrued)`, `dP = nominal − dA`, each part fitting its component — MIL/1 dropped U0 law L7. **[checked]**

Per owner and asset: `Σ active locks ≤ balance`, and `spendable = balance − Σ active locks`. **[obligation]** Prevents double pledging; checking each encumbrance alone does not.

## 4. Φ — the predicate language

### 4.1 Formation

Φ is a **typed** language. Every atom carries formation premises; these were absent in MIL/1, which made a NIGHT balance comparable to a DUST balance.

```
Γ ⊢ t₁ : Qty<A> ! S₁     Γ ⊢ t₂ : Qty<A> ! S₂        -- same asset
─────────────────────────────────────────────────
Γ ⊢ t₁ ≤ t₂ : Prop ! S₁ ∪ S₂

Γ ⊢ t₁ : Instant(c)   Γ ⊢ t₂ : Instant(c)            -- same clock
─────────────────────────────────────────────────
Γ ⊢ t₁ ≤ t₂ : Prop
```

`! S` is the **source set** (§5). Comparisons across assets, clocks or scales are type errors.

### 4.2 Totality

Evaluation is total into `Value | Reject(err)`. **[obligation]** Every partial operator names its rejection: division by zero, checked-subtraction underflow, `Instant + Duration` overflow, product exceeding its declared width, `k_of_n` with `k = 0` or `k > n`. A `Reject` is not a truth value; the acceptance relation fails closed on it, as the product contract requires of `unknown` and `timeout`.

### 4.3 Terms and propositions

```
term ::= lit | qty | price | instant | duration | hole
       | pre(cell) | post(cell)                       -- NEW: state at stage boundaries
       | balance(d,a,asset) | supply(d,asset) | outstanding(o) | allowance(auth)
       | escrowed(e) | locked(owner,asset)
       | totalShares(p,c) | totalAssets(p,asset)
       | obs.value | obs.observedAt
       | term + term | term − term | min | max
       | term × lit                                    -- Φ₀: literal coefficient only
       | sharesFor(…) | assetsFor(…) | mulDiv(…)       -- Φ₁ only

prop ::= term ≤ term | term < term | term = term
       | term × lit ≤ term × lit                       -- Φ₀ cross-multiplication
       | not atom                                      -- NEW: bounded atomic negation
       | x ∈ set | set ⊆ set
       | prop and prop | prop or prop | k_of_n(k,[prop…])
       | fresh(obs, Duration(c))                       -- same clock; observedAt ≤ stageTime
       | final(obs) | attested(obs,k,n)
       | after(Instant(c)) | before(…) | within(…)
       | delivered(asset, ≥ qty, to party, on domain)
       | discharged(o) | seized(enc) | consumed(receipt)
       | funded(e) | terminal(e) | exercised(opt) | challenged(e)
```

**Atomic negation is admitted.** MIL/1 banned negation, then used `not terminal` to derive its own workflow states and needed `not exercised` for options. On a finite width, atomic negation does not change decidability; it changes polarity, which is tractable. **[checked]**

`pre`/`post` are added because an AMM invariant, a share-price relation and a funding computation are all statements relating two state boundaries — none was expressible in MIL/1.

Removed as undenotable: `failed(route)`, `never`, `equity(…)`, `collateral_value` and bare `settle`/`return`. `Route` and `Program` are **not Φ sorts**; a hole of those types may be referenced only by the effect grammar, never inside a predicate. **[checked]**

### 4.4 Φ₀ and Φ₁ — **owner decision 1, taken**

- **Φ₀** is everything above except `sharesFor`, `assetsFor`, `mulDiv` and variable×variable products. **Literal-coefficient cross-multiplication stays in Φ₀**, so the U3 discriminator (`spent × 20 ≤ received × 11`) is Φ₀ and needs no solver. **[checked]**
- **Φ₁** adds pool arithmetic and variable products. Reserved behind a version flag; **not frozen at U0**.

Decision procedures: Φ₀ escrow patterns are quantifier-free integer difference logic, decided by negative-cycle detection in polynomial time; sums, `min`/`max` and `k_of_n` lift to QF-LIA, NP-complete but practical under the §12 caps. Φ₁ is QF-BV, which in practice means SMT — and the repository's own measurement makes that cut for us: one 128-bit division obligation cost bitvector `rlimit` **242,607,369** against **1,978** for unbounded `Int`, and at 256 bits the encoding **exceeded 600 seconds**. **[checked]**

*Reversal cost if you want Φ₁ at U0:* moderate. The grammar and caps already reserve it; adopting it adds an SMT dependency to the authoring trusted base and requires the §7 authoring check to fail closed on `unknown`.

### 4.5 Width and the limb rule

`Qty` is `u128`. The BLS12-381 scalar field is **255 bits** and a `u128 × u128` product needs **256**, so `mul` followed by `less_than` is silent modular coercion, which the numeric profile forbids; `less_than` stops at 253 bits regardless. **[checked]**

Therefore: a product that does not fit the field is a **declared two-limb gadget**, never a native `mul`. Nonlinear nodes are capped separately from comparison nodes (§12). This rule also applies to the U0 proposal's U3 discriminator, which specifies cross-multiplied UInt256 inequalities.

## 5. Evidence

```
Obs<T, ε, d>            -- evidence class ε, anchoring domain d
ε ::= anchored | imported(Policy) | attested(Issuer, k, n)
Γ ⊢ t : T ! S           -- S ⊆ {ε@d}, the source set of every observation that influenced t
```

Three repairs to MIL/1's central claim:

1. **Evidence and domain are type indices**, not record fields. `anchored` alone was a nullary tag for a domain-relative property, so an observation anchored in one domain satisfied `anchored(o)` in another after crossing an Episode edge. **[checked]**
2. **The source set propagates through every operator.** `o.value : T ! {ε@d}`, and `min(a.value, b.value) : T ! {εa@da, εb@db}`. MIL/1's eliminators erased the label, so `min(imported, anchored) ≤ limit and anchored(a)` was satisfiable with the imported value doing the work. **[checked — this was the review's laundering counterexample]**
3. **`anchored` is a typing side condition, not a proposition.** A position requiring anchored evidence rejects a term whose source set contains anything else. At proof time the circuit binds each anchored observation to an **authenticated ledger read** — a private witness tag is forgeable. Holes carry `! S` and may not be filled from a wider source set. **[obligation: non-laundering, proof route = source-set preservation over Φ evaluation, provable in K]**

`imported` remains a named trust premise until U4. `fresh` requires the same clock and `observedAt ≤ stageTime`, closing MIL/1's symmetric reading that let a feed pre-timestamp.

## 6. Authority

Eight rights: `initiate, complete, reconcile, recover, disclose, amend, issue, enforce`, each with scope, budget (`linear | affine`), window, delegability and revocation. `unbounded_within` is **deleted** — it contradicted bounded episode budgets and the design's own "bounded everything". **[checked]**

`issue` is scoped to `(domain, asset)`; `enforce` is bounded by a policy the debtor consented to at origination, referenced by `Obligation.consent`. Two `enforce` effects writing one cell require a **signed total order on seizes of one encumbrance id** — this is core, not a library, because footprint disjointness would otherwise reject them outright (owner decision 4's neighbour; see §15).

## 7. Escrow as a transition relation

MIL/1's meta-rule is deleted. An escrow is a small state machine.

```
escrow E {
  custody  signer | program | threshold(k,n)
  fund     Qty<A> from Party
  states   unfunded → pending → (released | refunded)      -- terminal states tombstoned
  release_when Φ₀        -- guard on the release transition
  refund_when  Φ₀        -- guard on the refund transition
  priority     release | refund | signed_order             -- late-race policy, REQUIRED
  deadline     Instant(c) | none
  on_release { effects }
  on_refund  { effects }
  residual   to Party
  footprint  { reads … writes … }
}
```

- **Pending is legitimate.** A funded escrow before its deadline with nothing delivered is a normal state, not a compile error. MIL/1's validity requirement forbade it.
- **Exclusivity is per-witness, in-circuit.** A constrained boolean branch bit selects one transition, and the ledger writes a one-shot terminal tombstone. Universal disjointness of the two guards is *not* required, which is what permits `ROADMAP.md:39`'s late-success/refund race — the `priority` field decides it. **[checked]**
- **`deadline none` is legal** (locked collateral discharged only by repayment), because exhaustiveness no longer depends on a deadline disjunct.
- **Authoring check:** `after(deadline) ⇒ release_when ∨ refund_when`, over Φ₀, decided by difference logic. `unknown` fails closed. With `deadline none` the check is vacuous and the exit obligation below carries the weight. **[checked]**
- **Recovery viability is a separate, named acceptance obligation**, not a propositional consequence. An escrow claiming a recovery guarantee must name its liveness assumptions — inclusion, actor arrival, witness availability — and the acceptance predicate rejects a workflow that claims the property without them. A deadline is a clock fact, never an exit and never a refund entitlement. **[obligation]**

## 8. Holes and refinement

```
hole route  : Route   where route.hops ≤ 3          -- effect-grammar only
hole amount : Qty<A> ! {anchored@d}  where amount ≤ 11 A
```

**Monotone completion is replaced by acceptance refinement. [checked that the old rule was false]**

> For a signed template `I` and a completion `σ`: `Accepted(I[σ]) ⊆ Accepted(I)`.

This is checked **per concrete filling** by the acceptance relation — the circuit verifies that `σ` respects every declared bound, that the completed guarantees hold, and that no fixed field changed. It is not a static quantification over all fillings, which is what made MIL/1's implication ill-formed (the right-hand side still contained the hole).

Polarity survives only as an **authoring-time lint** over a restricted fragment, with a declared operator variance table, and **signed factors excluded** — variance in `a × h ≤ b × c` depends on the sign of `a`, and `Delta`/`Position` are signed. The lint warns; it does not license. **[checked]**

## 9. Footprints and linearity

```
Cell ::= balance(d, acct, asset) | supply(d, asset) | obligation(id)
       | encumbrance(id) | escrow(id) | receipt(id) | position(acct, instrument)
       | shares(pool, class, acct) | allowance(authId) | observation(feedId)
       | replay(id) | policy(id) | pool(id)
```

MIL/1's six-cell vocabulary omitted receipts, encumbrances, positions, shares and allowances, so two Episode branches consuming the *same receipt* had disjoint declared footprints and were certified independent — the disjointness rule defeated the linearity it existed to support. **[checked]**

Two further repairs:

- **Footprints are derived and checked, not merely declared.** The compiler derives the read/write set from guards and effect expressions and requires `declared ⊇ derived`, as *[df]* `lean/DefiKernel/Typed/Transition.lean:80,121` does. A cell containing an unfilled hole yields `unknown`, which fails closed.
- **A fork partitions the linear context**, `Δ = Δ₁ ⊎ Δ₂`, and a join consumes each predecessor exactly once. Affine authority carries a shared non-duplicable budget across branches.

## 10. Effects, conservation, residue

Every effect line carries its domain, and a **local effect must match the executing domain**. A foreign credit is represented as imported evidence linked to a later accepted stage, never as an effect this stage applies — MIL/1 made a foreign mint describable as though it settled.

**Conservation, general:** for each `(domain, asset)`, `Σ balance deltas = declared supply delta`, with escrow custody and reserve accounts inside the quantified set. S0 specializes it with the supply delta fixed at zero, which is how `UNIFIED-PROPOSAL.md:90`'s E1 becomes an instance rather than a competing law. **[checked]**

**Residue** is an opaque, digested, unparsed string. It records an unexpressed wish or risk. It **never covers an effect**: an effect the relation cannot account for is rejected, not made residual. MIL/1's framing let a model delete an expressible clause, mention it in residue and still be accepted. **[checked]**

## 11. Canonical form, digest, lowering

- Canonical encoding: declared field order, fixed widths, sorted sets, normalized commutative connectives, explicit absent-versus-default, hex case fixed. MIL/1 deferred all of this to "the language version", and **U0 cannot hash-bind what is unspecified**. **[checked]**
- `digest = Poseidon(domain_tag ‖ canonical(intent))`, with the digest a public input and the wallet signing that digest, or the ledger checking a signature over it.
- **In-circuit signature verification is not claimed.** Authoritative ZKIR v3 has `ec_mul` on Curve25519 and Poseidon/SHA-256/Keccak, but **no SHA-512 and no Ed25519 verifier**; `and`/`or`/`xor`/`sha512` are crate extensions outside the 34 instructions. This corrects the U0 proposal's T6, which makes in-circuit authentication primary. **[checked]**
- **Lowering rule:** `assert` enforces non-zero in-circuit, so every Φ bit passes `constrain_to_boolean` before `assert`, or a witness of 2 satisfies a clause (ZR06). **[checked]**
- **The MIL/1 fallback is deleted.** Committing to the tree while evaluating only "effect-binding" clauses yields a *different relation*: freshness, anchoring, authority windows and hole-domain restrictions bind what may happen without being effect lines. A clause not evaluated is not in the relation, and no negative control repairs it. If a tree exceeds the caps, **reject it**. **[checked]**

## 12. Version header and caps

Frozen numbers, not prose. Initial values are **proposals to be measured, not measurements**:

| Cap | Initial | Note |
| --- | ---: | --- |
| Φ depth | 12 | |
| Φ total nodes | 64 | |
| Nonlinear nodes (Φ₁) | 0 at U0 | Φ₁ deferred |
| `min`/`max` nodes | 8 | each case-splits |
| `k_of_n` width | 8 | expands binomially |
| Effect lines | 16 | |
| Footprint cells | 32 | |
| Episode length | 8 stages | |
| Join fan-in | 2 | |
| `Qty` width | u128 | §4.5 limb rule above the field |

Unknown tags are rejected; the version header governs migration.

## 13. Surface syntax

```
intent AcquireB {
  version   moriarty-intent/2
  signer    owner = 0xab…
  clock     midnight.preview.clock
  validity  from now for 24h

  assets { A = asset:0x01 symbol NIGHT decimals 6
           B = asset:0x02 symbol DUST  decimals 6 }

  budget { gross_debit <= 11 A; fees <= 1 A; net >= 20 B to owner }

  hole route : Route where route.hops <= 3

  observe price : Price<B,A,6> ! {anchored@midnight.preview}
          from feed "dust/night" require fresh(price, 5m)

  escrow E {
    custody      program
    fund         11 A from owner
    release_when delivered(B, >= 20 B, to owner, on midnight.preview)
    refund_when  after(validity.end)
    priority     release
    deadline     validity.end + 1h
    on_release   { transfer { domain midnight.preview; asset A; from E; to counterparty; amount 11 A } }
    on_refund    { transfer { domain midnight.preview; asset A; from E; to owner; amount 11 A } }
    residual     to owner
    footprint    { reads balance(midnight.preview, owner, A), escrow(E)
                   writes escrow(E), balance(midnight.preview, owner, B) }
  }

  authority { initiate by owner
              complete by any solver budget affine 11 A
              recover  by owner window from validity.end + 1h for 30d }

  residue { "ordering and MEV exposure are not expressed" }
}
```

Every construct here is in the grammar. **[checked]** MIL/1's showcase used `failed`, bare `settle`/`return`, and a `release`/`refund` pair that its own rule rejected.

## 14. The other owner decisions

**Decision 2 — flash loans: restated as a product cut, not an entailment.** MIL/1 claimed atomicity excluded them; the reviewers showed ZKIR can unroll a bounded internal trace, so the exclusion is a choice. MIL/2 excludes intra-stage traces because they add a sub-language that interacts with footprints and linearity, and reserves a `trace` extension point in the version header. *Reversal cost: low now, high after U0 freezes the header.*

**Decision 3 — joins are ledger-linked through U3.** In-circuit verification of parent proofs is ZR09 and stays in U4. *Reversal cost: low to defer further, high to pull forward — it imports U4's recursion dependency.*

**Decision 4 — multi-signer stage arity is reserved now.** A stage binds a **set** of signed intents with one shared write set; the U0 profile admits `|S| = 1`. Reviewers were unanimous that the single-signer relation is load-bearing and cannot be retrofitted, and that MIL/1's proposed carrier for n-party clearing — a join over *disjoint* footprints — is exactly backwards, since every order writes the pool. Reserving the arity is a cardinality field plus quantification in the acceptance relation; admitting `|S| > 1` is later work. *Reversal cost: very high if not reserved now.*

**Decision 5 — concentrated liquidity is out of scope, declared.** A Q64.96 square-root price is not a `Price<_,_,s>` with `s ∈ 0..18`; tick boundaries are powers of 1.0001; crossing ticks is a bounded loop over `liquidityNet`, and Φ has no loop or array; and a position `(tickLower, tickUpper, liquidity)` is none of `Share`, `Position` or a supply-∈{0,1} NFT. It needs a bounded-iteration construct and a new sort — a future version, not a patch. *Reversal cost: high; it is a second language, not an extension.*

## 15. Milestone placement

| Work | Milestone |
| --- | --- |
| Canonical encoding, digest, domain tag, version header and caps, unknown-tag rejection | **U0** |
| Stage public-input schema; authority right-kind enum with `issue`/`enforce`; footprint sort and cell vocabulary; evidence type index; domain-qualified `Instant`; residue as opaque bytes; conservation as the general law | **U0** |
| Φ₀ grammar, formation and totality rules; the operator variance table | **U0** |
| Φ₀ escrow authoring check (difference logic, no solver); certificates for Φ₀ primitives; digest cost measurement | **U1** |
| One specialized two-asset escrow end to end, with hostile mutations of digest, amount, clock and asset | **U2** |
| Ledger-linked joins, the late-success/refund race, partial fill, recovery | **U3** |
| Φ₁, recursive certificates, imported-evidence verification, private handoff | **U4** |
| Multi-signer stage arity admitted; n-party clearing | **post-U4** |

## 16. Open, and honestly open

1. **n-party clearing** — arity reserved, mechanism unspecified.
2. **Concentrated liquidity** — out of scope by decision 5.
3. **Flash loans** — excluded by decision 2, extension point reserved.
4. **Φ₁ at U0** — deferred by decision 1.
5. **Evidence predicates** for recipient acceptance, document commitments and challenge records — named in the product's conditional-delivery contract, not yet in Φ.
6. **Bounded observation collections**, without which TWAP and aggregation cannot be libraries.
7. **The family→constructor map** — MIL/2 adds constructors and makes it more necessary; still owned by a provenance alias rather than a milestone.

## 17. Obligations before freeze

Every **[obligation]** above, collected: Φ totality into `Value | Reject`; source-set preservation (non-laundering); the encumbrance sum rule; acceptance refinement under completion; recovery viability as an acceptance predicate; and derived-footprint containment. Each needs a proof or an executed differential in K and TypeScript before the digest is hash-bound. **None is claimed proved here.** That is the discipline MIL/1 lacked.
