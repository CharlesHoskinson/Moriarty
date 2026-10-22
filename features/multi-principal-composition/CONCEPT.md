# Multi-principal intent composition: concept notes

2026-10-22. Specified only. Tags: **[checked]**, **[obligation]**, **[deferred]** (MIL/2 §0), **[prop]** (proposed here, unchecked). These notes extend MIL/2. They do not replace any MIL/2 decision.

## 1. The problem

A Moriarty intent today is the signed statement of **one** owner over the effects of one stage. The acceptance relation quantifies over a single signer. The signed profiles refuse a program in which another actor pays or owes (`INTENT_ACTOR_SCOPE`, `docs/language` semantics). **[checked]**

Regulated assets are never governed by one party. A tokenized deposit carries:
- the holder's wishes;
- the issuer's eligibility rules;
- the terms its regulator approved;
- the rules of whatever venue it touches;
- the rules of the holder's jurisdiction.

These parties author their rules independently, at different times, and for different lengths of time. The issuer's rules outlive any one holder's intent. An exocovenant must enforce all of them at once on a chain that knows none of them. That is the "sum of the intentions" the owner describes.

Three naive designs fail:
- **Merging into one program written by one author.** The author's signature would then stand in for every other principal's authority. The issuer's rules would be only as trustworthy as whoever merged them.
- **Disjoint-footprint joins (MIL/1's carrier).** Every governing principal constrains the *same* cells, so disjointness rejects the very case it was meant for. MIL/2 decision 4 already records this for n-party clearing. **[checked]**
- **Role precedence ("the regulator overrides the user").** It lets any party that holds a role remove a protection another party relied on, without that party's agreement. A user who said "never sell" would be sold by whoever claims the regulator role.

## 2. Two different multi-principal problems

| | Layered governance (this feature) | n-party clearing (MIL/2 open item 1) |
|---|---|---|
| What is shared | One governed object (asset, account, contract) | A pool or batch that several owners trade through |
| Who signs | Owner plus the principals whose rules attach to the object | Each trading owner |
| Acceptance | Every component touching the footprint accepts | Every owner's intent accepts, *and* a matching or pricing rule holds |
| Extra semantics | Scope, consent exemptions, exit safety | Matching, pricing, fairness |
| Placement | Requested ahead of U4 (owner decision) | Post-U4, unchanged |

Both need the reserved stage arity `|S| > 1` (MIL/2 decision 4). Only layered governance is required here. Clearing reuses the same quantified acceptance relation later, with a different combiner. **[prop]**

## 3. The model

### 3.1 Components

A **governed object** `g` has a cell footprint `cells(g)` (MIL/2 §9 cell vocabulary). A **component** is a signed intent authored by one principal for `g`:

```
component k = ⟨ principal P_k : DID,
                role_k        : declared label, no precedence,
                scope_k       ⊆ cells(g) × effect-kinds,
                Φ_k           : constraint (Φ₀ guard and outcome predicates),
                G_k           : grants (rights over cells P_k controls),
                ev_k          : evidence policy (issuers, k-of-n, freshness),
                aud_k         : disclosure audience for k's evidence,
                d_k           : canonical digest, signed by P_k ⟩
```

Each principal's intent is **split into a constraint part and a grant part.** **[prop]**
- A **constraint** restricts which effects are acceptable. Constraints from different principals are *combined by intersection*.
- A **grant** gives a named holder a right (MIL/2 §6: `initiate, complete, reconcile, recover, disclose, amend, issue, enforce`) over cells. A grant is valid only over cells that the grantor's own authority covers. Grants are *combined by scoped union*.

This split is the core idea. Restricting is something anyone with a legitimate scope may do. Empowering is something only an owner of the authority may do.

### 3.2 The composed covenant

```
C = ⊗{k ∈ K} = ⟨ g, K, Order, Exit ⟩
```

`Order` is a signed total order or priority for rights that contend on one cell. It generalizes MIL/2's signed order on seizes of one encumbrance (§6) and the escrow `priority` field (§7). `Exit` is the set of exit routes the composition must keep open.

### 3.3 Acceptance

A stage `σ` proposes effects `e` by actor `a` (an owner, a holder of a grant, or a mandated agent) with evidence `ev`. The composed acceptance relation adds five conjuncts to MIL/2's per-intent acceptance: **[prop]**

```
Accept_⊗(C, s, σ) iff
 (A) Authorized   ∀ cell x written or debited by e:
                    a holds an unconsumed, unrevoked right covering x at this position,
                    and that right was granted by x's owner or by a grant within it;
 (B) Constrained  ∀ k with footprint(e) ∩ scope_k ≠ ∅ and not Exempt_k(e):
                    Accept(I_k, s|scope_k, e, ev|aud_k);
 (C) Duties       every live obligation and encumbrance in s is preserved or discharged by an explicit effect;
 (D) Ordered      if e contends with another pending right on a cell, Order selects e;
 (E) Exit-safe    e is in Exit, or after e some exit route remains enabled,
                    except while a consented, bounded encumbrance is active.
```

The rest of acceptance is unchanged: refinement under completion (MIL/2 §8), derived-footprint containment (§9) and conservation (§10) apply to the whole stage.

### 3.4 Exemptions come only from the constrained party's own consent

`Exempt_k(e)` holds only when both of these are true:
- `e` is an enforcement effect (`enforce`: freeze, cease, seize, release, award-release) exercised under a **consent capability** that `P_k` itself signed;
- `e` lies within that capability's scope, budget and window.

So Bob's "never sell" does not block the seize he agreed to at onboarding, and it does block everything else. No role grants an exemption. **[prop]**

### 3.5 Agents and mandates

An agent is an actor, never a component author on its principal's behalf. A **mandate** is a grant from principal `P` to an agent DID. It is outcome-form: what the agent may achieve, within affine budgets, an expiry and sub-delegation that can only narrow (attenuation). The agent proposes a stage, and acceptance checks `(A)` against the mandate *and* `(B)` against every component, including `P`'s own. A corrupt agent can therefore only choose among stages that the full composition already accepts. Moriarty's existing rule that a solver is checked, never trusted, carries over unchanged. **[prop]**

## 4. Laws to prove

All of these are **[obligation]**. None is claimed.

1. **Every principal is honored.** For every component `k` and every accepted stage `σ` touching `scope_k`, either `Accept(I_k, …)` holds or `σ` is an enforcement effect under a consent signed by `P_k`.
2. **Grant locality.** If an accepted stage writes cell `x`, a chain of grants rooted in `x`'s owner authorizes the actor. A regulator, issuer or venue cannot spend a holder's funds without the holder's prior consent.
3. **Monotonicity.** Adding a component never enlarges the accepted set, except through exemptions that the added component's own constrained parties signed. Removing a component requires that component's own principal, through `Amend`/`Revoke`.
4. **Algebra.** `⊗` is commutative, associative and idempotent on the multiset of components. `⊤` (no constraints, no grants) is its unit. On constraint-only components, `Acc(I ⊗ J) = Acc(I) ∩ Acc(J)`.
5. **Exit safety.** At every reachable state of an admitted covenant, `Exit ∩ Acc(C) ≠ ∅`, except during a consented encumbrance with its own bounded duration. Composition cannot build a trap.
6. **Refinement under completion lifts.** For a template with holes, `Accepted(C[σ]) ⊆ Accepted(C)`, checked per filling, extending MIL/2 §8 to the composite.

## 5. Conflict semantics

| Conflict | When detected | Outcome |
|---|---|---|
| **Static incompatibility.** The components jointly accept no effect leading to any terminal or exit state. | At `Compose`, by a decidable check over Φ₀ that extends MIL/2's escrow authoring check `after(deadline) ⇒ release ∨ refund` from one intent to the composite. An undecided check fails closed. | No covenant is created; funds are never placed under an unsatisfiable covenant. What a failure reveals is owner decision 2 (§7). |
| **Dynamic contention.** Two rights act on one cell at the same position (spend vs freeze, seize vs seize). | At acceptance. | `Order`. The default is that an encumbrance settled earlier precedes a later spend. |
| **Amendment drift.** An issuer or regulator changes its rules mid-life. | At `Amend`. | Pinned policy: existing obligations and encumbrances keep their digest, and new actions use the amended digest only for components whose principals endorsed it. A holder who refuses the amendment keeps an exit route. |
| **Evidence disagreement.** Attesters or oracles disagree. | Inside the component's own evidence policy. | `k-of-n`, freshness and issuer allowlists apply. `Unknown` fails closed for spends, and exits follow law 5. |

## 6. Identity, evidence and the skein

- **Principals are DIDs** resolved at the signing position. Today's raw keys and strings become DID-controlled keys. Roles are declared labels and carry no power.
- **Credential predicates are typed evidence.** A new evidence class `presented(φ, issuers, schema)` is satisfiable only by a selective-disclosure presentation bound to this stage's context. For example: "resident not in US", "KYC by an approved attester", "net worth ≥ X". MIL/2's non-laundering rule (§5, source-set propagation) extends unchanged, so a component's evidence policy can't be met by evidence from issuers it did not name. **[prop]**
- **Audience-scoped evaluation.** `ev|aud_k` passes component `k` only the disclosures its audience may see. The issuer's KYC clause learns "holder ∈ KYC(approved attesters)", not the attributes.
- **Skein emission.** Every accepted stage of a bound covenant emits a skein entry. The entry is part of the canonical stage statement: actor, mandate if any, components evaluated, exemptions used and effects. A stage that does not emit its entry is not accepted. Moriarty does not interpret the skein; Passport holds it. This is what makes a later audience-scoped disclosure ("tax for this period", "Travel Rule for this transfer") complete with respect to the covenant's own history. **[prop]**

## 7. Privacy of components

In an exocovenant, the principals' rules are **private from the enforcing operators**: Fenrir's nodes learn only accept or reject. Some rules are also private from the *other principals*. A regulator's negotiated terms, or an issuer's risk rules, need not be visible to the holder.

This creates a tension the design must resolve (owner decision 2):
- A useful `CONFLICT` result tells principals which components clash, and possibly a witness. That leaks policy content.
- A bare "incompatible" bit is safe but hard to debug.
- A middle option is to report conflicts only to the principals whose components are involved, and only as the identity of the clashing component, never as a witness.

Evaluation privacy is not part of this feature. The language must, however, produce a canonical statement in which each component appears only as a commitment to its digest plus its acceptance bit. Otherwise the proof system cannot hide components from one another. **[prop]**

## 8. Interaction with existing decisions

| MIL/2 element | Effect of this feature |
|---|---|
| Decision 4 (reserved arity) | Uses it. The cardinality field becomes the component set `K`, and quantification enters the acceptance relation as already planned. |
| §6 Authority, `enforce` consented at origination | Generalizes `Obligation.consent` to consent capabilities held by a designated party. Seize ordering becomes a case of `Order`. |
| §7 Escrow priority | A case of `Order`. Escrow becomes a component-level construct that composes. |
| §8 Acceptance refinement | Lifted to the composite (law 6). |
| §9 Footprints | Scopes are subsets of the cell vocabulary. Derived-footprint containment is checked per component and for the stage. |
| §14 Decision 3 (ledger-linked joins through U3) | Unchanged. Composition happens within a stage, not across joins. |
| Signed intent/1 `delegation: "none"` | Replaced by mandates (§3.5) in a new profile. The old profile stays valid. |

## 9. Smallest useful profile

To make the Fenrir v1 path reachable, admit a restricted profile first: **[prop]**
- at most five components, each with an explicit role;
- constraints in Φ₀ only;
- enforcement grants only through consent capabilities;
- one `Order` per encumbrance;
- static conflict check by difference logic;
- one mandate level, with no sub-delegation.

This is enough for the Monument Bank example: Bob, Monument, the Bank of England's negotiated terms, the venue allowlist, and the India 1% withholding as a required effect.

## 10. Open questions

1. **Withholding and other required effects.** These are positive obligations ("must pay 1%"), not guards. Model them as duties that the composition requires the stage to discharge, under `(C)`, rather than as constraints. **[prop]**
2. **Scope authority.** Who may claim a scope? An issuer's scope over its own asset is clear. A jurisdiction's scope over "actors with credential `jur`" needs a rule for who may author the jurisdiction component.
3. **Re-composition when a holder changes.** When a governed asset moves to a new holder, the holder component is replaced and the others persist. Define this as an explicit transition.
4. **Cost.** Proving `(B)` evaluates every touched component, so proof size grows with `|K|`. Measure this before setting the profile cap.
5. **n-party clearing** remains as MIL/2 left it.

## 11. Example: Monument Bank deposits to Ethereum collateral

```
covenant monument_gbp_bob {
  object   asset MGBP held by did:bob

  component holder by did:bob {
    constrain never(effect.kind = sell)
    constrain recipient ∈ regulated_addresses
    grant initiate, complete to mandate agent_bob
  }
  component issuer by did:monument {
    constrain holder presented(kyc, issuers = {did:sblocks, did:monument})
    grant enforce{freeze, release} to did:monument_compliance  -- via Bob's onboarding consent
  }
  component regulator by did:monument for did:boe_terms {
    constrain effect.kind ∈ {lend_as_collateral, transfer_regulated, redeem}
  }
  component venue by did:fenrir_venue_policy {
    constrain foreign.call.target ∈ allow_eth_venues ∧ foreign.asset_out ∈ allow_assets
  }
  component jurisdiction by did:jur_in {
    when holder presented(resident = IN)
    require duty withhold(1%, effect.gain) to did:in_tax_agent
  }
  order    encumbrance-first
  exit     redeem_to_monument, refund_to_holder
}

mandate agent_bob for did:agent_bob { outcome yield ≥ 10% p.a.; budget affine 250k MGBP; expiry 90d }
```

The syntax is illustrative, not grammar. If Bob's agent proposes lending MGBP as collateral for USDC on an allowed venue, every component accepts it. If it proposes a sale or a non-allowlisted pool, the holder, regulator or venue component rejects it, and Fenrir never signs. If Monument's compliance team freezes under Bob's consent, the holder component is exempt and the freeze applies within its bounded duration.
