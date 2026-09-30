# Governance recommendation: authority and policy state (Opus 5.5, independent)

Scope: this is a read-only design review. Nothing here is implemented, proved or accepted by a ledger. Every edit below is a **recommendation**. Startup was followed: the develop skill was loaded and `status --json` ran. The blocked SP01 dispatch does not affect this review.

## 1. Verdict on category design fitness

**The design partly fits, and the gaps are structural.** MIL/2 names the right kinds of authority: eight rights, each with scope, budget, window, delegability and revocation (`concepts/intent-language/DESIGN-MIL2.md:191`). It has `allowance(authId)` and `policy(id)` cells (`DESIGN-MIL2.md:241-242`) and debtor `ConsentRef` (`DESIGN-MIL2.md:94-100`). But none of these has a **transition relation**:

- No transition grants, delegates, suspends or revokes a right.
- Nothing reads *current* revocation status at acceptance.
- No object is pinned to a policy version.

The authority judgment still means only "consumed/remaining/replay stay inside the signed budget" (`deliverables/u0-semantic-contract-2026-09-23/judgments.json:86-92`). Its leaves are opaque strings and `NOT_ENFORCED` (`deliverables/u0-study-2026-09-28/defi-coverage/R6-governance.md:280-281`).

The product contract requires "current revocation" as an owner-authorization condition (`docs/MORIARTY-PRODUCT-CONTRACT.md:56`). It also requires "per-contract policy commits to exact claim and verifier semantics" (`MORIARTY-PRODUCT-CONTRACT.md:62`). MIL/2 cannot state either today. The category review reaches the same conclusion (`category-review/06-governance.md:22,25-26`).

MIL/2 fixed the escrow defect by turning a meta-rule into a transition relation. Authority needs the same repair.

## 2. Five ranked design edits

### Edit 1 (highest): a ledger-resident grant cell with a closed status machine

Replace `allowance(authId)` with `grant(id)`:

```
Grant ::= { id, grantor, holder, right: RightKind, scope, budget: linear|affine Qty,
            consumed, reserved, window: Window(clk), depth: u8, parent: GrantId?,
            policyRef, status: active | suspended | revoked | exhausted }

AuthorityFresh(stage, g) :=
  authRead(grant(g), predecessorHead) ∧ status(g) = active
  ∧ stageTime ∈ window(g) ∧ right(g) ⊒ required(effect)
  ∧ consumed(g) + reserved(g) + n ≤ budget(g)
  ∧ ∀ a ∈ ancestors(g): authRead(grant(a)) ∧ status(a) = active      -- bounded by depth cap
post: consumed'(g) = consumed(g) + n;  consumed' = budget ⇒ status' = exhausted
```

- Transitions: `revoked` and `exhausted` are terminal and tombstoned. `suspended ↔ active` is reversible.
- Every consuming stage writes `grant(g)`. A revoke is also a stage that writes `grant(g)`. Footprint write-conflict therefore serializes a revoke against a consume on the same cell. The ledger's existing head order resolves the race, so no new ordering mechanism is needed.
- **Counterexample:** category G2 (`06-governance.md:22`). An allocator is removed, but an old signed call is still circulating. The signature, digest and budget all check, and the stage is accepted. Only an authenticated read of current status rejects it. A private witness bit saying "not revoked" is forgeable, for the same reason that `anchored` needed a ledger read (`DESIGN-MIL2.md:185`).
- **External practice (not a Moriarty rule):** W3C Bitstring Status List v1.0 separates `revocation` ("not reversible") from `suspension` ("reversible") (https://www.w3.org/TR/vc-bitstring-status-list/, captured in `source-text/w3c-status-list.md:327,338-343`). MIL/2's single word "revocation" should split the same way.
- **Placement:**
  - **U0:** the cell sort, the status enum, the grant record fields and the stage public-input binding of `(grantId, grantHead)`.
  - **U2:** enforcement for depth 0.
  - **Why this changes a U0 boundary:** the authority leaves are frozen U0 schema. Adding a typed status later would change the public input, the same "very high reversal cost if not reserved now" argument that decision 4 uses (`DESIGN-MIL2.md:336`).

### Edit 2: revocation is non-retroactive and cannot strand in-flight duties

This is the core of in-flight protection. Revoking or suspending a grant only stops **future** use of it.

```
revoke(g) accepted at head h:
  (R1) every stage accepted at head < h keeps its effects, liabilities and fees
  (R2) ∀ o created under g: o.terms, o.ConsentRef, o.policyRef unchanged; debt persists
  (R3) reserved(g) backing a pending escrow/continuation is not released by revoke;
       it is released only by that escrow's terminal transition
  (R4) exempt(right) = {recover, reconcile, discharge-side complete}: a grantor's
       revoke/suspend cannot remove a right that a counterparty holds under a signed
       recovery policy; such a right ends only by its own signed termination rule
  (R5) suspend(scope) may block initiate/issue/new-liability effects only;
       repayment, refund and recovery of existing positions stay reachable
```

- R4 follows `docs/MORIARTY-CONSOLIDATED-DESIGN.md:70`: "Revocation cannot erase outstanding duties", and recovery grants "declare their own signed termination rule".
- R5 turns a guardian pause into a *bounded* suppression. This addresses G4/G16 without a global pause power.
- **Counterexample 1 (stranding):** escrow E is funded, with `refund_when after(validity.end)`. The owner's counterparty holds `recover`. The grantor revokes `recover` before the deadline, so no terminal transition is authorized and funds are trapped.
- **Counterexample 2 (pause):** a pause over "all lending actions" blocks `repay`. Accrual continues and the borrower is liquidated because they could not repay. R5 makes this unrepresentable.
- **Placement:**
  - **U0:** R1–R5 as rule text in the `authority` and `failure` judgments.
  - **U3:** the hostile traces, because the late-race and recovery work is already U3's (`ROADMAP.md:24`).

### Edit 3: a policy-version commitment pinned on every persistent object

```
PolicyRef ::= (policyId, version: u32, digest)          -- digest over canonical policy bytes
Intent, Obligation, Encumbrance, Escrow, Grant each carry policyRef, fixed at creation.
policy(id) cell ::= { version, digest, minDelay, amendable: FieldSet, successor? }

Eval(stage acting on object x) uses x.policyRef, never head(policy(id)),
unless a Migrate(x, v→v') stage carries
  ConsentRef(holder(x), digest_v, digest_v') ∨ HolderMonotone(x, v, v')
Migrate preserves: beneficiary, outstanding, consumed authority, replay marks,
                   cumulative fees, recovery rights                 -- UNI-015
```

- `HolderMonotone` is a per-field check against a declared direction table: a cap may only rise and a rate may only fall for the holder. It is checked per concrete migration, as acceptance refinement is (`DESIGN-MIL2.md:230-232`). It is not proved statically.
- **Counterexample:** a lending DAO raises the rate after Alice borrows. If `Eval` reads the current `policy(id)`, the next accrual stage applies the new rate with no consent. Separately, an interface-compatible upgrade changes the beneficiary. That is UNI-015's hostile witness (`openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md:164-166`), and here it is rejected by construction.
- **Placement:**
  - **U0:** the `policyRef` field in the intent and stage public input. The public statement already lists "semantic and numeric version; program/Core identity" (`MIL2-PROPOSED-SEMANTICS.tex:240`) but has no per-object policy.
  - **U2:** pinned-only. Immutable deployment is a valid first implementation (`R6-governance.md:284`).
  - **U5/U6:** consented migration, left where the roadmap already puts it (`ROADMAP.md:26`).

### Edit 4: a typed `amend` effect with a delay that cannot shrink itself

```
amend(policyId, v→v+1, Δ): requires grant with right=amend, scope ∋ policyId
  Δ ⊆ amendable(v);  activation(v+1) ≥ acceptedAt(queueStage(digest(Δ))) + minDelay(v)
  minDelay(v+1) takes effect only for amendments queued after activation(v+1)
  a queued digest is consumed once (replay cell); a revoked queuer grant ⇒ queue entry void
```

- **Counterexample:** one amendment sets `minDelay := 0` and executes in the same stage, which bypasses its own timelock (G6/G17). A second case is a queued action whose approver's grant was later revoked (`06-governance.md:46`). The void rule rejects it.
- **Placement:**
  - **U0:** the `amend` effect shape.
  - **U3:** the first fixture, because a queue→execute pair is a two-stage ledger-linked episode.
  - The proposal, veto and cancel lifecycle is a library.

### Edit 5: delegation is attenuating, and the budget is shared rather than copied

```
delegate(g → g'): status(g)=active ∧ depth(g) > 0 ∧ depth(g') < depth(g)
  ∧ scope(g') ⊆ scope(g) ∧ window(g') ⊆ window(g) ∧ right(g') ⊑ right(g)
  ∧ budget(g') is a reservation against g: reserved'(g) = reserved(g) + budget(g')
```

- Revoking g invalidates g' through the ancestor read in Edit 1. That read is bounded by the depth cap, which I recommend adding to the caps table at `DESIGN-MIL2.md:272-283`.
- **Counterexample:** g allows 100 A. g' is minted with its own 100 A counter, so the two can spend 200 A in total. This duplicates an affine resource, which a fork partition forbids (`DESIGN-MIL2.md:250`). A second case: g' has a window extending past g's expiry.
- **Placement:**
  - **U0:** the `parent` and `depth` fields.
  - **U2:** depth fixed at 0, matching S0's `delegation=none`.
  - **U3:** depth ≤ 1.
  - **U5:** OWS delegation (`ROADMAP.md:26`).

## 3. Core versus library boundary (recommendation)

| Layer | Contents |
| --- | --- |
| **Core** (in the acceptance relation) | `grant(id)` and its status machine; `AuthorityFresh` with authenticated current reads; R1–R5 non-retroactivity and the pause-exempt set; `policyRef` pinning; the `amend`/`Migrate` effects and their preservation clauses; the delay-monotonicity rule; delegation attenuation. These guard objective financial state and debt, so a library cannot opt out of them. |
| **Library** | Proposal, vote, snapshot, quorum, vote-escrow, the veto/cancel UI lifecycle, guardian committee selection, treasury residual-claimant rules. Each compiles to core `amend`/`suspend`/`grant` stages. |
| **Application policy, not a core right** | Issuer freeze and clawback. Declare these in the asset's `policyRef` so holders take on the risk when they acquire the asset. `enforce` stays debtor-consented collateral only (`DESIGN-MIL2.md:193`). |
| **Out of scope** | Honesty of human governors and capture resistance: named trust premises (`wiki/defiformal-taxonomy.md:185-192`). |

## 4. Smallest implementable slice

This is a single signer, delegation depth 0, one affine `complete` grant, a pinned immutable `policyRef`, and only the `active → revoked | exhausted` transitions. The stage public input binds `(grantId, grantHeadBefore, grantHeadAfter, policyRef)`, and the circuit binds the grant read to authenticated ledger state.

- **Positive:** the owner grants solver S `complete`, affine 11 A, over window W. S submits a stage at head h₀ that consumes 11 A. `AuthorityFresh` holds, and `grant(g)` becomes `exhausted` with the tombstone written. The effects match the signed intent.
- **Hostile:** the owner's revoke stage is accepted at h₁. S then submits a *fully valid* proof built against h₀, with a correct signature, digest, budget and envelope. It must be rejected because the authenticated read at the current head shows `revoked`, or because the predecessor head is stale.
  - This control must fail on the status or head binding. A malformed envelope would not count (`ROADMAP.md:40`).
  - A companion mutation flips the witness status bit to `active`. It must also reject, which shows the bit is not trusted.

## 5. Explicit disagreements

1. **With `DESIGN-MIL2.md:191` treating revocation as a single attribute.** Revocation has to be a transition with a terminal tombstone, and it has to be distinct from reversible suspension. As an attribute it cannot be checked at acceptance.
2. **With leaving all migration to U5.** UNI-015 is scoped there to federation epochs (`R6-governance.md:284`). An *application's* mutable `policy(id)` cell at U2, with no pinning, would breach `MORIARTY-PRODUCT-CONTRACT.md:62`. Pinning belongs in U0/U2; only consented migration waits.
3. **With using `k_of_n` (`DESIGN-MIL2.md:143`) as threshold authority.** It is a Boolean over propositions, not distinct authenticated signers. There is no in-circuit Ed25519 (`DESIGN-MIL2.md:264`), and U0 admits one signer (`DESIGN-MIL2.md:336`). Until multi-signer arity is admitted after U4, committee authority should be a ledger-checked signature set or a named trust premise. It should not be written as a Φ clause.
4. **With the legacy map's "expressible" claims** for timelock and policy binding (`06-governance.md:40`). I agree with the downgrade. Edits 3 and 4 are the missing rules.

## 6. Residual assumptions

- A Midnight contract can make an authenticated read of `grant(id)` and `policy(id)` at the acceptance head, and write-conflicts on one cell are totally ordered. This is not verified for the pinned target.
- A single domain clock supplies `acceptedAt` for the timelock. Cross-domain timelocks are out of scope, and there is no global rollback.
- Signature checking is a ledger rule, not a circuit rule.
- Public grant status may leak usage patterns. The W3C spec recommends random indexes for herd privacy (external practice), and private grants depend on U4.
- The outcome of an off-chain vote enters only as an authenticated signature or imported evidence under a named trust premise.
- The depth cap and the `HolderMonotone` direction table are proposals and have not been measured.