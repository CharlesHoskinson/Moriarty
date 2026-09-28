# Moriarty Intent Language (MIL/1) — design

**Date:** 2026-09-28
**Status:** specified-only proposal. Nothing here is implemented, no milestone has accepted it, and no evidence closes any U predicate. It is a design intended to become controlling, not a record of behaviour.

**Scope.** A language of **signed financial intention** that expresses abstract, cross-chain **conditional settlement with programmable escrow**, is checkable by the Moriarty acceptance relation, compiles to pinned ZKIRv3, and is written and completed by AI solvers without weakening what the signer authorized.

**Why now.** The owner placed conditional settlement and programmable escrow in the intent language (`wiki/moriarty-architecture.md`, CLM-0978). The [architecture review](../../deliverables/u0-study-2026-09-28/ARCHITECTURE-COVERAGE-REPORT.md) found the U0 signed intent carries twelve fields — caps, sets and policies — with no condition, predicate, escrow or release construct, and found the architectural holes this design has to close. The signed-intent syntax task sits inside the U0 freeze, so the shape has to be decided before the contract is hash-bound.

**Companion.** [CATEGORY-MAP.md](CATEGORY-MAP.md) maps this design to each DeFi category and each asset category.

---

## 1. Design goals

1. **Outcome-first, route-free.** The signer states what must be true when settlement completes. Which domains, venues and asset paths achieve it is a solver's decision inside declared bounds.
2. **The intent is the specification.** A solver's plan is *checked against* the intent, never trusted. There is no reviewer step and no privileged solver.
3. **One predicate language.** The same total, bounded predicate language Φ serves intent conditions, judgment clauses, library preconditions and refinements. Two predicate surfaces would mean two evaluators, two certifications and two correspondence arguments.
4. **Trust is syntactic.** An imported fact cannot be written where an anchored one is required. The evidence class is part of the type, not a comment.
5. **Bounded everything.** Every stage terminates in checked bounds; every clause declares a cost; the canonical intent has a bounded encoding so its digest can be authenticated in-circuit.
6. **AI-completable without weakening.** Degrees of freedom are declared as typed holes. Filling a hole can only narrow the admissible set; this is a static property, not a review.
7. **Inexpressibility is reported, never silent.** Residue is a first-class field.

**Non-goals.** MIL is not a general programming language, not a solver, not a routing engine, and not a replacement for the Federated DeFi Kernel — which remains optional. It does not promise global cross-domain rollback, and it does not model liveness.

## 2. The four layers

```
  Intent (MIL)        what the signer authorizes and requires — signed, digested, in-circuit
      │  completion (holes filled by a solver; monotone)
      ▼
  Plan                a candidate: concrete route, venues, amounts, evidence
      │  checking (acceptance relation)
      ▼
  Episode             bounded DAG of stages, joins, compensation, cumulative budgets
      │  per-domain atomicity
      ▼
  Stage               one accepted transition on one domain — the canonical stage statement
```

A plan is never authoritative. The acceptance relation reads the intent and the produced episode and decides. The Episode is the composite unit the architecture review found missing above the stage: a join's two branch outcomes and an interleaving schedule belong to it, not to a stage field.

## 3. Types

### 3.1 Domains and clocks

```
Domain    ::= identifier                      -- midnight.preview, ethereum.mainnet
Clock     ::= Domain "." "clock"
Instant   ::= Clock "@" u64                   -- domain-qualified, never bare
Duration  ::= u64 unit                        -- 30s, 24h; unit-checked
Window    ::= "from" Instant ("for" Duration | "to" Instant)
```

Time is a core type, domain-qualified. A bare instant is a type error: "now" on one chain is not "now" on another. This closes the derivatives finding that the core enumeration contained no time or clock while expiry, continuations, accrual and recovery all depend on one.

### 3.2 Assets

```
Asset ::= { domain: Domain, issuer: Issuer, symbol: Symbol,
            repr: Repr, decimals: u8 }
Issuer ::= "native" | Principal | Policy
Repr   ::= "canonical" | "wrapped" of Asset | "shielded" of Asset | "synthetic" of Policy
```

Asset identity is structured, not a string. `wrapped(BTC@ethereum)` on Midnight and `canonical BTC@bitcoin` are distinct assets with a declared link — the bridge category depends on this, and so does the stablecoin finding that no on-chain observation separates two dollar tokens without an obligor.

### 3.3 Quantities, prices and the claim algebra

```
Qty<A: Asset>        -- checked unsigned integer in A's smallest unit
Signed<A>            -- checked signed; positions only, never balances
Price<B, Q, s>       -- base-per-quote, scale s ∈ 0..18
Share<P: Pool>       -- a claim against a pool
```

Arithmetic is the frozen numeric profile: checked add/sub, directed rounding by economic role, and the five remainder classes (`none`, `conserved-split`, `charged-increment`, `sub-unit-residual`, `protocol-reserve`). Two additions this design requires:

- **`mulDiv : Qty<A> × Share<P> × Share<P> → Qty<A>`** with an explicit rounding role, plus `Share` addition across holders and a `totalShares(P)` projection. Without these a vault, an LP position and a liquid-staking token are not typeable, which the staking review established. A sixth remainder class, **`retained-in-pool`**, covers the truncated unit that stays with the remaining holders.
- **Cross-multiplied comparison** as the primitive form of a ratio constraint: `a·d ≤ b·c` rather than `a/b ≤ c/d`, in a wider intermediate. Limits never require division.

### 3.4 Observations

```
Obs<T> ::= { feed: FeedId, issuer: Issuer, domain: Domain,
             observedAt: Instant, value: T, finality: Finality,
             evidence: Evidence }
Finality ::= "pending" | "probabilistic" of confirmations | "final"
Evidence ::= "anchored"                    -- authenticated by the executing domain
           | "imported" of Policy          -- a fact from elsewhere, with its trust policy
           | "attested" of (Issuer, k, n)  -- k-of-n attestation
```

An observation carries its **value**, its **observed-at** instant, and its **evidence class**. All three were missing: the canonical stage statement bound observations by issuer, domain, time and finality with no value at all, which left freshness a property with no operand and made a price-dependent program unwritable.

**The anchoring rule is a typing rule.** A predicate position that requires `anchored` rejects an `imported` or `attested` term at compile time. This implements "an imported fact may never be presented as an anchored one" as a type error rather than as prose in a deliverable.

### 3.5 Obligations, encumbrances and receipts

```
Obligation ::= { id, debtor, creditor, asset: Asset,
                 principal: Qty, accrued: Qty, outstanding: Qty,
                 terms: Terms, rank: ClaimClass }
Encumbrance ::= { id, owner, asset, amount: Qty, against: ObligationId,
                  enforceable_by: Authority }
Receipt ::= linear token of a completed effect
```

`outstanding = principal + accrued`, all components non-negative, with `opening + accrual − discharge = closing` as a bound roll-forward rather than two snapshots. `ClaimClass` gives loss allocation a seniority order. An **encumbrance** is a linear resource its owner may not spend and a named authority may seize — the shape collateral needs, which the architecture asserted nowhere.

### 3.6 Footprints

```
Cell ::= balance(Domain, Account, Asset) | supply(Domain, Asset)
       | obligation(Id) | escrow(Id) | replay(Id) | policy(Id)
Footprint ::= { reads: {Cell}, writes: {Cell} }
```

Every operation declares one. Two operations are **independent** iff `writes₁ ∩ (reads₂ ∪ writes₂) = ∅` and symmetrically. This is what disjoint-parallel admission, interference reasoning and call-site framing all need, and it is the one addition that cannot be retrofitted: independence is not a field added to a stage, it is something operations must carry. The existing per-constructor `frame` field is at the wrong granularity — 37 of 40 constructors read `unchanged`, and two transfers between disjoint accounts have identical frames.

## 4. Φ — the predicate language

One language, used everywhere. Total, decidable, bounded, with a declared cost per clause.

```
term ::= literal | qty | price | instant | duration
       | balance(d, a, asset) | outstanding(oblig) | allowance(auth)
       | supply(d, asset) | escrowed(e) | totalShares(p)
       | obs.value | obs.observedAt | hole
       | term + term | term − term | mulDiv(term, term, term, rounding)
       | min(term, term) | max(term, term)

prop ::= term ≤ term | term < term | term = term
       | term · term ≤ term · term            -- cross-multiplied ratio
       | x ∈ set | set ⊆ set
       | prop and prop | prop or prop
       | k_of_n(k, [prop, …])                 -- threshold
       | fresh(obs, Duration)                  -- observedAt within d of stage time
       | final(obs) | anchored(obs)
       | after(Instant) | before(Instant) | within(Window)
       | delivered(asset, ≥ qty, to party)
       | discharged(oblig) | seized(enc)
```

**Deliberately absent:** recursion, unbounded quantifiers, negation over unbounded domains, division, floating point, and any call to an external evaluator. Every clause is statically costed, which lets a solver budget proving before attempting a plan, and lets U1 certify the primitives a clause uses.

**Φ is also the judgment clause language.** The U0 plan proposes a small predicate language of linear integer arithmetic and set membership over relation paths for executable judgments. That is a subset of Φ. Adopting one language means the intent's release condition and the effect judgment are evaluated by the same certified evaluator, in TypeScript and in K, with one correspondence argument.

## 5. Authority

```
Right ::= initiate | complete | reconcile | recover | disclose | amend
        | issue | enforce
Authority ::= { right: Right, holder: Principal, scope: Scope,
                budget: Budget, window: Window,
                delegable: bool, revocation: RevocationPolicy }
Budget ::= linear of Qty | affine of Qty | unbounded_within(Scope)
```

Six rights came from the architecture; **`issue`** and **`enforce`** are the additions this design requires, and four independent reviewers reached that conclusion from four categories:

- **`issue(domain, asset)`** — the right to change supply. The product contract requires "authorized mint/burn supply changes" while the authority model had no slot one could occupy. Stablecoins, LP shares, liquid-staking tokens and any vault share depend on it.
- **`enforce(policy)`** — a third party's right to act against a defaulting counterparty, bounded by a policy the debtor consented to at origination. Without it, liquidation contradicts the rule that creating a liability requires consent from the party made liable.

Authority consumption is bound per-asset and per-right, with replay identity. Revocation cannot erase outstanding duties.

## 6. Escrow and conditional settlement

The central construct.

```
escrow E {
  custody   <who holds it: signer | program | threshold(k, n)>
  fund      Qty<A> from Party
  release   Φ                  -- when the escrow pays out
  refund    Φ                  -- when it returns
  deadline  Instant
  on_release { effects }
  on_refund  { effects }
  residual  to Party           -- where sub-unit remainders go
  footprint { reads … writes … }
}
```

**The nine workflow states are derived, not enumerated.** The architecture listed submitted-unfunded, funded-into-escrow, partially-fulfilled, waiting-for-evidence, eligible-for-release, in-flight, delivered, unresolved and recovering, and explicitly deferred their representation. Here each is a predicate over `(funded, evidence, release, refund, deadline, terminal)` — for example `eligible_for_release ≡ funded and release and not terminal`. Adding a tenth state is adding a predicate, not changing a type.

**Release and refund must be exhaustive and disjoint.** The compiler requires `release and refund` to be unsatisfiable, and `release or refund or after(deadline)` to be valid. An escrow with no reachable exit is rejected at compile time. This is the checkable core of the design's demand that a claimed recovery guarantee either establish a viable closure path or reject the workflow — an obligation that previously had no component.

**Cross-domain settlement without global rollback.** An escrow lives on one domain. A release condition that depends on another domain reads an `imported` or `attested` observation, and the anchoring rule forces the signer to state the trust policy. Compensation is a declared `on_refund` effect, not an inferred rollback. The design's rule that a local model must never claim global rollback is preserved by construction, because no predicate can assert an anchored fact about a foreign domain.

## 7. Holes and monotone completion

```
hole route  : Route where route.domains ⊆ {…} and route.hops ≤ 3
hole amount : Qty<A> where amount ≤ 11 A
hole venue  : Program where venue ∈ approved_set
```

A hole is a declared degree of freedom. **Everything not a hole is fixed.**

**Monotonicity is a static check.** The intent's guarantee clauses must be antitone in every hole: for any filling `σ`, `Φ_guarantee[σ] ⟹ Φ_guarantee`. The compiler checks polarity — a hole may not appear in a position where increasing it widens the admissible set of a guarantee. A solver therefore cannot widen a cap, add a recipient, extend a window or resurrect consumed authority by filling holes, and this is a property of the program rather than something a reviewer confirms.

## 8. Canonical form, digest and authentication

The intent has a canonical encoding: fields in declared order, integers in fixed width, no floats, sets sorted, depth and node count bounded by the language version. `digest = H(canonical(intent))`.

That digest is a public input of the stage circuit and is authenticated **in-circuit**, per the U0 target decision. Because conditions are inside the digest, escrow semantics are signed, not merely advertised — which is the whole point of putting conditional settlement in the intent language. The bound encoding is what makes the circuit cost measurable; U1 owns measuring it.

## 9. Effects, conservation and residue

```
Effect ::= transfer { domain, asset, from, to, amount }
         | fee      { domain, asset, from, to, amount }
         | supply   { domain, asset, delta: Signed, authority: issue }
         | oblig    { op: create|accrue|discharge|transfer|forgive, … }
         | encumber { op: lock|release|seize, … }
```

**Every effect line carries its domain.** The stage's domain becomes the *executing* domain rather than the only one. Without this a debit in one domain and a credit in another cannot be stated in one relation.

**Conservation, correctly general:** for each `(domain, asset)`, `Σ balance deltas = declared supply delta`. It is zero exactly when nothing is minted or burned. The U0 proposal's law E1 states the zero case unconditionally; freezing that form would make every issuance program violate an accepted law.

**Residue is a field.** `residue { "…" }` records what the signer knows the language cannot express. It never affects validity and is always reported. Without it, an effect the relation cannot record makes the instance *invalid* rather than *residual*, and every later review has to choose between silence and a false failure.

## 10. Surface syntax

```
intent AcquireB {
  version   moriarty-intent/1
  signer    owner = 0xab…
  clock     midnight.preview.clock
  validity  from now for 24h

  assets {
    A = midnight.preview / native / NIGHT  decimals 6
    B = midnight.preview / native / DUST   decimals 6
  }

  budget {
    gross_debit <= 11 A
    fees        <= 1 A
    net         >= 20 B to owner
  }

  hole route : Route where route.domains ⊆ { midnight.preview }
                       and route.hops <= 3

  observe price : Price<B, A, 6> from feed "dust/night"
          require fresh(price, 5m) and anchored(price)

  escrow E {
    custody  program
    fund     11 A from owner
    release  delivered(B, >= 20 B, to owner)
    refund   after(validity.end) or failed(route)
    deadline validity.end + 1h
    on_release { settle E }
    on_refund  { return E to owner; retain fees <= 0 A }
    residual   to owner
    footprint  { reads balance(midnight.preview, owner, A)
                 writes escrow(E), balance(midnight.preview, owner, B) }
  }

  authority {
    initiate by owner
    complete by any solver budget affine 11 A
    recover  by owner window from validity.end + 1h indefinite
  }

  residue { "ordering and MEV exposure are not expressed" }
}
```

Properties that make this AI-friendly are in §11; the syntax is deliberately keyword-led, block-structured, unit-explicit and free of significant whitespace, because those are the properties that make a language reliably writable by a model and diffable by a human.

## 11. What makes it AI-friendly

Ten concrete properties, each a design decision rather than a style preference:

1. **One predicate language.** A model learns Φ once and uses it for conditions, judgments, preconditions and refinements.
2. **Declared search space.** Holes make the degrees of freedom explicit. A solver never has to infer what it is allowed to change.
3. **Monotone completion, statically checked.** A solver cannot weaken a guarantee, so an incorrect or adversarial completion is rejected structurally rather than caught in review.
4. **Counterexample-carrying rejection.** A failed check returns the minimal unsatisfied clause with a witness. This is what lets a model iterate without a human in the loop.
5. **Distinct outcome vocabulary.** `satisfied`, `violated`, `unknown`, `unsupported`, `timeout`, `inconsistent` are different answers. Collapsing them is what makes agents loop.
6. **Declared cost per clause.** A solver can budget proving before it commits to a plan.
7. **Footprints give local reasoning.** An agent can compose two intents, or search two branches in parallel, by checking disjointness instead of simulating the world.
8. **Syntactic trust boundary.** Evidence classes make trust laundering a type error, which matters most precisely when a non-human is assembling the plan.
9. **Canonical form and digest.** Identical intents are identical bytes: dedupe, cache, replay-detect and compare across agents.
10. **Residue.** A model can say "I could not express this" in-band. Every other option — silent approximation, a dropped constraint, a plausible-looking near-miss — is worse, and this is the single most important property for machine authorship.

## 12. What this closes, and what it does not

**Closes, from the architecture review:** the region notion (§3.6); authority kinds including issuance and enforcement (§5); observation value, observed-at and evidence class (§3.4); domain on the effect line (§9); the general conservation law (§9); the composite unit above the stage (§2); the nine workflow states (§6); the claim algebra for shares (§3.3); a residue slot (§9); the recovery-viability obligation, as a compile-time exhaustiveness check (§6); the conjunction and threshold rules that had no operator (§4); and the carrier for conditional settlement itself.

**Does not close:** the family→constructor map — this design adds constructors and makes the map more necessary, not less; whether a compiler, ledger and proof server emit and accept ZKIRv3 end to end; the circuit cost of in-circuit authentication of a predicate tree, which U1 must measure; liveness of any kind; and the intra-stage question — this design deliberately keeps the stage atomic and pushes sequencing to the Episode, so a liquidation is multi-stage by construction.

**Known tension.** A richer intent is a larger digest and a larger circuit. If U1's measurement shows predicate authentication is too expensive, the fallback is a commitment to the condition tree with in-circuit evaluation of only the clauses that bind effects — and that fallback must be a recorded decision with a negative control, not a silent downgrade.

## 13. Placement in the roadmap

| Work | Milestone |
| --- | --- |
| Φ, types, footprints, authority kinds, canonical form — the freeze | **U0**, as the signed-intent syntax task, extended |
| Certificates for Φ's primitives and the digest cost measurement | **U1** |
| One signed intent end-to-end with escrow on a pinned target | **U2** |
| Conditional settlement, partial fill, recovery, joins | **U3** |
| Cross-domain episodes, imported evidence, compensation | **U4**, with **U5** for federated routing |
| Library families over the core | **U6** |

The U0 item is the whole of it. Everything else is enabled by deciding the shape now, and foreclosed by freezing the twelve-field intent instead.
