Drafting the governance recommendation now; the reading is done.

# MIL/2 Governance: independent recommendation (adversarial controls and release plan)

**Status.** This is a read-only design review of a proposal that is only specified. I ran nothing: no compilation, proofs, tests or transactions. Guarded CLI `status --json` reports `SP01.6` blocked on stale inputs. That blocks campaign dispatch, not this review. Every edit below is a **recommendation**, not an adopted MIL/2 rule.

## 1. Verdict

**Not fit to freeze for governance. The vocabulary is fit, the transitions are not.** The eight rights with scope, budget, window, delegability and revocation (`concepts/intent-language/DESIGN-MIL2.md:191`) and the `policy(id)`/`allowance(authId)` cells (`DESIGN-MIL2.md:239-242`) are the right carriers. The design still lacks a stateful rule for all five attacks in my focus:

- **Revocation.** Revocation is an attribute with no event, state or precedence rule (`06-governance.md:22`).
- **Threshold.** `k_of_n` is a Boolean over propositions (`DESIGN-MIL2.md:143`). It does not authenticate distinct signers (`06-governance.md:29`).
- **Timelock.** `after(T)` is a validity guard, not a queue (`06-governance.md:28`).
- **Upgrade.** There is no pinned policy version or migration relation (`06-governance.md:25,31`), although UNI-015 already names the hostile case "upgrade changes beneficiary or resurrects consumed authority" (`openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md:157-166`).

`AuthorityFresh` and `HistoryLink` are named (`MIL2-PROPOSED-SEMANTICS.tex:133-137`), but nothing defines what "fresh" reads. The authority leaves are still `NOT_ENFORCED` opaque strings (`R6-governance.md:282-283`).

## 2. Five ranked design edits

### E1 (rank 1): Grant cell with a monotone epoch, and a native current-authority read. Closes: stale grant.

**Rule sketch.** Replace the bare `allowance(authId)` with
`grant(g) = {grantor, holder, right, scope, budgetUsed, reserved, status ∈ {active, suspended, revoked}, epoch: u64, parent: g? }`.

```
Revoke(g):   s(grant g).status ≠ revoked, caller holds authority over g
             ⟹ status' = revoked, epoch' = epoch+1          -- irreversible
Suspend/Resume(g): status active↔suspended, epoch' = epoch+1 -- reversible
AuthorityFresh(I,s): ∀ g used by I:
   authRead(grant g) at predecessorHead ∧ status = active
   ∧ epoch = I.signedEpoch(g) ∧ budgetUsed + reserved + debit ≤ budget
   ∧ ∀ ancestors a of g: status(a) = active          -- revoking a parent kills children
```

**Precedence.** An *accepted* stage stays accepted, and revocation never discharges debt, liabilities or residual duties (`MORIARTY-PRODUCT-CONTRACT.md:45`). An *unaccepted* signed completion dies at the next epoch.

**Counterexample.** An allocator is revoked at head h, and the grant moves from epoch 7 to epoch 8. A solver submits a correctly signed epoch-7 completion at h+1, within budget. Under the current text it passes, because the window and signature hold and "revocation" has no read. Under E1 it rejects on `epoch ≠ 8`.

**Placement.** The cell sort, the `status`/`epoch` fields, and the public-input slot for the authenticated grant read belong in **U0**. This changes the U0 boundary, for the reason under "U0 boundary change" below. The native read belongs in U2. The ancestor chain is capped at depth ≤ 2 in U2 and deepened at U5 for OWS delegation (`ROADMAP.md:26`).

**External practice (not a Moriarty rule).** W3C Bitstring Status List separates irreversible `revocation` from reversible `suspension` (`source-text/w3c-status-list.md:327`; https://www.w3.org/TR/vc-bitstring-status-list/).

### E2 (rank 2): Threshold approval as a consumed, digest-bound approval set, not a `k_of_n` Boolean. Closes: threshold replay.

**Rule sketch.**

```
signerSet(id) = {members: sorted keys, k, setEpoch}              -- a cell
Approval = sig_ledger(domain_tag ‖ actionDigest ‖ setId ‖ setEpoch ‖ nonce)
ThresholdOK(A, act):
   |A| ≥ k ∧ signer indices strictly increasing (distinctness)
   ∧ all members ∈ signerSet at setEpoch = current setEpoch
   ∧ all signed bytes = digest(act)
   ∧ replay(actionDigest ‖ nonce) unconsumed ⟹ consumed'
```

Rotating a signer set bumps `setEpoch`, so approvals collected under the old set die.

Authority must not be a Φ proposition. Treat it like `anchored`, which is a typing side condition bound to a ledger read (`DESIGN-MIL2.md:185`). A `k_of_n` over propositions may not discharge an authority requirement. `threshold(k,n)` custody (`DESIGN-MIL2.md:201`) must elaborate to `ThresholdOK`.

**Counterexamples.** Each of these is accepted by a Boolean `k_of_n`, and each is rejected under E2:

1. Three approvals for transfer T₁ are resubmitted for an identical-amount T₂.
2. One signer is counted twice.
3. Approvals are gathered before a compromised key was rotated out.

**Placement.** Signatures are checked by the ledger, not in-circuit (`DESIGN-MIL2.md:264`). **U0** reserves the approval-set field and the `signerSet` cell alongside the reserved stage arity (`DESIGN-MIL2.md:336`). The rule is admitted in a post-U2 `gov-threshold` profile, before the post-U4 n-party clearing, because it authorizes one action rather than clearing n intents.

### E3 (rank 3): A queued administrative action as an instance of the escrow transition relation, re-checked at execution. Closes: revoked queued action.

**Rule sketch.** Generalize escrow (`DESIGN-MIL2.md:199-212`) so the payload can be an **administrative effect** as well as funds.

```
Queue(act):   AuthorityFresh ∧ ThresholdOK? ⟹ q = queued,
              actionDigest fixed, eta = stageTime_ledger + delay_policy   -- not signer-supplied
Execute:      q = queued ∧ after(eta) ∧ AuthorityFresh at *execution* head
              (default revalidate-at-execution) ∧ digest(effects) = actionDigest ⟹ executed, tombstone
Cancel/Veto:  holder of cancel scope, q = queued ⟹ cancelled, tombstone
Expire:       after(eta + grace) ⟹ expired, tombstone
priority:     cancel > execute                                         -- REQUIRED, per escrow rule
```

A policy may declare `snapshot-at-queue` instead of the default, but it must say so, and the signing display must show it.

**Counterexample.** An upgrade is queued under grant epoch 7, the proposer is revoked (epoch 8), and anyone calls `Execute` after eta. With the snapshot semantics that most timelock designs apply implicitly, it executes. Under the default it rejects. A second case: `eta` supplied as a signer field set to the past rejects, because `eta` must derive from the ledger clock.

**Placement.** The administrative-effect kind and the ledger `stageTime` go in U0 (the effect sort already sits there, `DESIGN-MIL2.md:345`). The queue lifecycle goes in **U3** with the late-race and tombstone work (`ROADMAP.md:24`). Queue as a library goes in U6.

### E4 (rank 4): Pinned policy reference, and a preservation-or-consent migration relation. Closes: malicious upgrade.

**Rule sketch.** Every obligation, escrow, encumbrance and position records `policyRef = (policyId, version, digest, programId, verifierId)` at creation. Two administrative effects:

```
Amend(p, v→v+1):  amend right scoped to p ⟹ policy(p).head' = v+1   -- new instances only
Adopt(x, v+1):    ConsentRef of every party whose ClaimClass is adversely
                  affected in x ⟹ policyRef(x)' = v+1
Migrate(old→new): ∀ live x: Π_pres(x under old) = Π_pres(x under new), where
   Π_pres = {beneficiary, debtor, outstanding, principal/accrued split,
             consumed replay/tombstone set, cumulative gross/fees, recovery path}
   otherwise reject
```

A stage over x evaluates under `policyRef(x)`, never under `policy(p).head`.

**Counterexample.** A new verifier that is interface-compatible accepts the same calls but pays to `attacker`. The public statement's program/verifier identity (`MIL2-PROPOSED-SEMANTICS.tex:240`) *detects* the change, but nothing *forbids* it. Π_pres differs on `beneficiary`, so it rejects. That is UNI-015's hostile witness. A second case: migration clears `replay`, which resurrects consumed authority, and it rejects.

**Placement.** The `policyRef` field and its public input go in **U0**. **U2 ships "immutable deployment only"**: `Migrate` is always rejected, and that rejection is the hostile control. The ZR15 limitation already calls immutable deployments a valid initial implementation (`R6-governance.md:285`). Π_pres-checked migration goes in U5 with ZR15 (`ROADMAP.md:26`), extended from federation epochs to program policy.

### E5 (rank 5): Beneficiary as a consent-guarded field outside delegation. Closes: beneficiary change.

**Rule sketch.** Recipients and creditors are fixed signed fields (`MIL2-PROPOSED-SEMANTICS.tex:109`), never holes, unless the signer supplies a signed substitution predicate (`MORIARTY-PRODUCT-CONTRACT.md:41`).

```
Assign(o, c→c'):  ConsentRef(c) ∧ (ClaimClass requires debtor notice ⟹ disclose) ⟹
                  creditor' = c', outstanding' = outstanding, debtor unchanged
Delegation:       scope(child) ⊆ scope(parent) ∧ child.right ≠ amend-beneficiary
```

No `complete`, `amend` or `enforce` grant confers `Assign` unless the scope explicitly names it. The payout recipient is part of Π_pres (E4).

**Counterexample.** A solver holding `complete by any solver budget affine 11 A` (`DESIGN-MIL2.md:321`) fills the route so proceeds land in its own account. That is the negative case listed at `MIL2-PROPOSED-SEMANTICS.tex:127`. Under E5 the recipient is not a hole, so the fill fails. A second case: a policy amendment changes the protocol fee recipient on live positions without `Adopt`, and it rejects under E4 and E5.

**Placement.** The fixed-field rule goes in U0/U2. `Assign` goes in U3, with obligations and continuation.

## 3. Core versus library

**Core** (it changes what acceptance means, so it cannot be a library):

- the grant cell with status and epoch, plus the authenticated read (E1)
- the approval-set structure and distinctness/consumption rule, reserved at U0 (E2)
- the administrative-effect kind and ledger-derived `stageTime` (E3)
- `policyRef` pinning, the `Π_pres` definition and the rejection of `Migrate` (E4)
- beneficiary as a fixed field, and the scope-intersection rule for delegation (E5)

**Library:**

- the timelock queue as an escrow instance
- governors, proposals, quorum, snapshots and vote escrow (out of Core, as `06-governance.md:30` says)
- guardian role bundles
- treasury allocation rules

## 4. Smallest implementable slice (U2, single signer)

This is E1 only, with one delegated `complete` grant, no amendment and an immutable deployment.

- **Positive.** Grant `g` is active at epoch 3 with budget 11 A. A solver completes a transfer of 10 A to the fixed recipient, with fee ≤ 1 A. Readback shows `grant(g).budgetUsed = 10 A`, the replay id consumed, and complete effects matching the signed digest.
- **Hostile.** The same signed bytes and the same witness shape are submitted after an accepted `Revoke(g)` at an earlier head, which moved the grant to epoch 4. The amount, fee, recipient and signature are all valid.

The only difference is the authenticated grant read, so rejection shows the native authority check. It is not caused by a malformed envelope (`ROADMAP.md:40`). A control must also show that the hostile witness *would* be accepted with the grant read removed. Otherwise the pair shows nothing.

## 5. Explicit disagreements

1. **With `06-governance.md:29`.** It grades `k_of_n` as a partial threshold carrier. I recommend `k_of_n` be *ineligible* for authority, which means a type error.
2. **With the U5-only placement of UNI-015 (`R6-governance.md:285`).** `policyRef` must be in U0, and rejecting `Migrate` must be a U2 control. Waiting until U5 would freeze a digest that cannot pin policy.
3. **Timelocks.** I recommend reusing the escrow relation instead of adding a new Core construct. The category report leaves this open.
4. **Revocation precedence.** I recommend revalidating at execution by default, and allowing snapshot only when declared. The report leaves precedence unspecified (`06-governance.md:22`).

**U0 boundary change (stated as required).** U0 hash-binds the cell sort, the public-input schema and the canonical digest (`DESIGN-MIL2.md:344-345`). Adding `grant.status/epoch`, `policyRef`, `signerSet` and a reserved approval slot after freeze would change every digest. This is the same argument as owner decision 4 (`DESIGN-MIL2.md:336`). Φ₁ is untouched, and none of these edits needs variable products.

**External practice (not a Moriarty rule).** IBC ICS-004 keeps an explicit `upgradeSequence` and monotone per-direction sequence counters (`source-text/ibc-ics004.md:267-277`; https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md). That supports a monotone epoch and a consumed sequence rather than validity windows.

## 6. Residual assumptions

- Midnight can give an authenticated current read of a contract-state cell at the predecessor head, bound as a public input. Unverified.
- Signature checks are done by the ledger, not in-circuit (`DESIGN-MIL2.md:264`).
- There is no inclusion guarantee. A revocation and a spend racing in one block are ordered by the ledger, and nothing guarantees the revocation wins (`DESIGN-MIL2.md:219`).
- Honest signers, key independence and resistance to governance capture are trust premises that cannot be proved (`06-governance.md:34`).
- The pause/guardian design (G4) is outside my focus. Any pause should leave repay, refund and recover transitions available, but I have not specified that.
- None of this is implemented, proved or accepted on a ledger.