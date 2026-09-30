# Governance recommendation: compiler and native feasibility of a single policy-amendment stage

**Scope and status.** This is a read-only design review of a specified-only proposal. I did not compile, prove, execute or submit anything, and nothing below claims implementation, proof or ledger acceptance. Every numbered edit is a **recommendation**, not an existing MIL/2 rule. `D` = `concepts/intent-language/DESIGN-MIL2.md`, `P` = `deliverables/mil2-deep-research-2026-09-29/MIL2-PROPOSED-SEMANTICS.tex`, `G` = `category-review/06-governance.md`. Status CLI: the campaign dispatch is blocked on stale SP01 inputs. That does not affect this review.

## 1. Verdict on category design fitness

**The category is partially fit and not yet compilable.** MIL/2 has the right pieces: `amend`, `issue` and `enforce` are separate rights (`D:191-193`), `policy(id)` is a cell (`D:239-242`), and there is a ledger-linked predecessor head (`D:46`). But none of these defines a transition. Nothing specifies:

- what an `amend` stage writes;
- which head it is checked against;
- how signatures authenticate it without an in-circuit Ed25519 verifier (`D:264`);
- which policy version an open position is evaluated under.

`G:25-31` reaches the same conclusion from the coverage side. From the native side, the problem is concrete. The U0 public-input schema (`D:345`) has no field that an amendment proof could bind. If the schema is frozen without one, every later governance feature needs a schema migration, which is the retrofit cost `D:336` describes for stage arity.

## 2. Five ranked design edits (recommendations)

### E1. A core `amend` stage kind over an authenticated `PolicyHead` (highest priority)

**Rule sketch.**
```
policy(id) holds PolicyHead h = Poseidon(tag_pol ‖ id ‖ ver ‖ paramsDigest ‖ authDigest ‖ pendingDigest ‖ prevHead)

AMEND(A):
  pre(policy(A.id)) = A.base                      -- authenticated ledger read, same mechanism as predecessorHead (D:46)
  digest(A) = Poseidon(tag_amend ‖ canonical(A))  -- public input; ledger checks signature over it (D:263)
  signerKey ∈ authSet(A.base) with right amend, scope ⊇ A.fields    -- membership proof against authDigest
  paramsDigest' = Poseidon(canonical(params(A.base) ⊕ A.diff))      -- recomputed in-circuit, equal to public input
  A.resultDigest = paramsDigest'                  -- the signer signs the result, not only the diff
  post(policy(A.id)) = H(A.base, A);  consume replay(A.nonce), allowance(amendAuth)
  footprint W = {policy(id), replay(n), allowance(a)};  E1 with Δsupply = 0;  L1 unchanged
```
The circuit, not the host, recomputes the new parameter digest. A host-computed acceptance Boolean therefore never stands in for the new-state check (`P:130`). An amend stage writes no balance, supply or obligation cells. This keeps footprint derivation (`D:249`) and conservation (`P:173-177`) trivial for this stage.

**Counterexample.** Without `A.base`, two amendments signed against the same head both apply (a lost update). An amendment signed before an intervening revocation also still applies. Without `resultDigest`, a host can compose the diff with a different base and change a parameter no one signed.

**Placement.** Reserve at U0 the stage-kind enum value `amend`, the public inputs `policyHead` and `policyHead'`, and the `PolicyHead` encoding. U1 measures the cost of a Poseidon hash over a capped parameter record (proposed cap: 16 fixed-width fields, to be measured) and the feasibility of a ledger-checked signature over a contract-supplied digest. Implementation belongs at U2, which is the single-stage path (`ROADMAP.md:23`).

**Why this changes U0.** The public-input schema is hash-bound at U0 (`D:345`). Adding fields later changes every verifier key.

### E2. Pin policy versions on positions; migration needs consent or a "future-only" declaration

**Rule sketch.** Add `policyPin: (id, ver, paramsDigest)` to `Obligation` and `Encumbrance` (`D:94-97`). The pinned version is written at origination. Parameter records are append-only: `policyVersion(id, v)` is an immutable cell. A stage that touches obligation `o` reads parameters from `o.policyPin` through an authenticated read, never from the current head. Moving `o` to version `v'` is a separate `MIGRATE(o, v')` effect. It requires a `ConsentRef` from each party whose signed fields change (debtor for terms, creditor for beneficiary). The only exception is an amendment declared `scope future-only`, which the compiler rejects if it writes any field read by a pinned position.

**Counterexample.** `G:25` gives the scenario: a DAO raises a rate cap and Alice's open loan silently accrues at the new rate. A second case is ZR15/UNI-015's hostile example, an upgrade that changes the beneficiary (`R6:289`). Pinning rejects both unless consent is given.

**Placement.** Reserve the `policyPin` field at U0 (same reason as E1). The native fixture belongs at **U3**, which already owns persistent duty and continuation (`ROADMAP.md:24`). Federation-epoch migration stays at U5 (`ROADMAP.md:26`).

### E3. Authority freshness means membership at the head the stage is applied against

**Rule sketch.** Grant, revoke and delegate are E1 amendments to `authDigest`. For every stage that uses a right, `AuthorityFresh` means: the signer key has a membership proof against `authDigest` of the authenticated **current** head. The signing time is irrelevant. Ledger order decides precedence: a revocation accepted earlier wins, and a stage already accepted stays accepted. There is no global rollback, and revocation does not erase debt (product contract, `:45,56`).

For threshold approval, one amendment digest carries k signatures. The circuit constrains strictly increasing signer indices into the authority set, which blocks one key being counted twice, and binds `k ≤ count`. The ledger verifies each signature.

This is **not** `|S| > 1` stage arity (`D:336`). One intent with k signers is a different axis from k intents in one stage.

**Counterexample.** This is the old signed call after a revocation (`G:22`). A signature that is valid but made before the revocation must still fail at the new head. For thresholds: key K submits its signature twice and reaches k = 2 without the index-ordering constraint.

**Placement.** Single-key membership at U2. Threshold membership is gated on U1 feasibility of verifying several ledger signatures. If that is unsupported, it stays **blocked**; a host check must not emulate it.

### E4. Timelock as a pending-state transition that reuses the escrow rule

**Rule sketch.** `QUEUE(A)` stores `(digest(A), eta = stageTime + policy.delay, authDigest@queue)` in `pendingDigest`. The delay comes from the policy, not from the amendment. `EXECUTE(A)` needs four things:

- an authenticated ledger `after(eta)` on the policy's clock;
- the pending entry present and not tombstoned;
- `authDigest` unchanged since queueing, or else a re-approval;
- the E1 premises.

`CANCEL(A)` requires `amend` scoped to `cancel`. Execute versus cancel at the same head is exactly the escrow rule (ESC, `P:193-198`) with signed `priority refund`, so cancel wins, plus a terminal tombstone.

**Counterexample.** `G:28` notes that `after(T)` alone is only a validity window. Its hostile case, a queued action whose approval was later revoked, is rejected by the unchanged-`authDigest` premise.

**Placement.** Library over core: it adds no new Φ₀ atom and no new right. It needs U3 race semantics (`ROADMAP.md:24`).

**External practice, not a Moriarty rule.** ERC-6372 makes a governance contract state which clock mode it uses (https://eips.ethereum.org/EIPS/eip-6372). This matches MIL/2's clock-qualified `Instant`.

### E5. Pause as a policy parameter, with remedy classes that cannot be paused (core invariant)

**Rule sketch.** A policy may declare `paused: ⊆ ActionClass` and `pauseUntil: Instant(c)`. A guardian is a holder of `amend` scoped to `{paused, pauseUntil}`, with a policy-declared maximum duration. Each value stage's `GuardsTrue` gains `class ∉ paused ∨ after(pauseUntil)`. **Core authoring rule:** repay, discharge, refund and recover can never be paused. The only exception is when the obligation's origination `ConsentRef` names that pause explicitly.

**Counterexample.** `G:27`: a guardian pauses repayment while accrual continues, and the borrower is then liquidated.

**Placement.** The invariant is reserved at U0 as a rule on `ActionClass`. The native path is at U3. Pause configuration is a library.

## 3. Core versus library boundary

- **Core:** the E1 transition and `PolicyHead` encoding; separation of value writes from amendment writes; E2 pinning and the migration-consent rule; E3 membership at the current head; the E5 non-pausable remedy classes.
- **Library:** timelock queue, veto and cancel (E4); guardian configurations; per-protocol parameter schemas; treasury as ordinary balance authority for a named principal; vote objects and snapshots. External practice for snapshots: ERC-5805 `getPastVotes(account, timepoint)` (https://eips.ethereum.org/EIPS/eip-5805). That requires Φ₁ share arithmetic and stays deferred.
- **Out of scope:** deliberation, choosing governors, resistance to social capture (`G:34`).
- **Freeze and clawback:** I recommend declaring these non-Core. An asset may carry them only through an issuance policy fixed at asset creation. They should not be added as a new right (`G:32`).

## 4. Smallest implementable slice and evidence pair

**Slice (recommendation; U2 after the U1 measurements).** One policy cell with one `u128` parameter (`maxFeeBps`) and an authority set of two keys. The E1 transition only: single signer, no timelock, no migration.

**Positive control.** Key K1 holds `amend` on P at head h0 and signs A = {base h0, maxFeeBps 5→3, resultDigest, nonce n1}. Expected outcome:
- native proof verifies;
- readback shows `policy(P) = H(h0, A)` and `replay(n1)` consumed;
- every balance and supply delta is zero.

**Hostile control (a well-formed envelope, not a malformed one; see `ROADMAP.md:40`).**
1. First, a valid amendment A2 from K2 removes K1, giving head h1.
2. K1 then correctly signs A3 = {base h1, maxFeeBps 3→30, fresh nonce}.

The signature, envelope and head are all valid; only the membership check against h1 can reject A3. The expected result is rejection with `AUTHORITY_NOT_MEMBER` and no state change.

A second mutation: take the positive control's valid proof inputs and alter `paramsDigest'` to encode 30. The proof must fail.

## 5. Explicit disagreements

1. **With `G:29`:** threshold approval is not limited by the U0 `|S| = 1` stage profile (`D:336`). k signatures on one digest is an authentication question for U1/U2, not post-U4 arity.
2. **With the placement in `G:25` and `G:31`:** application-policy preservation (ZR15/UNI-015) should not wait for U5 (`ROADMAP.md:26`). Only federation-epoch migration belongs at U5; position pinning belongs at U3.
3. **With adding rights:** I recommend no `pause`, `veto` or `freeze` right. Scoped `amend` is enough, and the eight-kind enum (`D:191`) stays frozen.
4. **With `D:191`'s budget model applied to `amend`:** a `linear | affine` value budget means nothing for amendments. Its budget should be a count of amendments per window.
5. **With `G:33`:** I would drop the general residual-claimant rule as a governance prerequisite. Treasury spending is ordinary balance authority once a principal exists.

## 6. Residual assumptions (unverified)

- The Midnight ledger can verify one or more signatures over a digest supplied by the contract and expose the signer key or index as a bound public input. `D:263` states this intent; U1 must measure it.
- The same authenticated contract-state read used for `predecessorHead` works for `policy(id)` and for the `policyVersion` cells.
- An authenticated ledger clock exists with a stated skew, so E4's `after(eta)` is meaningful.
- Poseidon over a 16-field parameter record fits the U1 cost budget. The cap is a proposal, not a measurement.
- Key independence, governor honesty and inclusion of queued or cancel transactions are external trust or liveness premises that no proof establishes (`P:200`; product contract `:47`).

**Boundary changes.** U0 gains the `amend` stage kind, the `policyHead` public inputs, the `policyPin` field and the non-pausable-class rule, because the public-input schema is hash-bound at U0. **Φ₁ is unchanged:** every guard above is Φ₀ (digest equality, set membership, `after`).