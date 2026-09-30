# Governance recommendation: proposals, votes, and threshold authority

*Independent Opus 5.5 brief. This is read-only design review. Nothing here claims implementation, proof or ledger acceptance. Every numbered edit is a **recommendation**, not an existing Moriarty rule. The guarded CLI `status` was run; its blocked SP01 dispatch has nothing to do with this review.*

## 1. Verdict

**Fit for governance effects; not yet fit for governance authorization.** MIL/2 has the right building blocks:

- distinct `amend` and `issue` rights (`DESIGN-MIL2.md:191-193`);
- a `policy(id)` cell and a `replay(id)` cell (`:239-242`);
- same-clock time predicates (`:146`);
- anchored evidence tied to authenticated reads (`:185`);
- a stage relation that requires `AuthorityFresh` (`MIL2-PROPOSED-SEMANTICS.tex:132-137`).

It has no rule that turns a quorum into authority. Two defects matter most:

1. **`k_of_n` is a Φ proposition** (`DESIGN-MIL2.md:143`). Counting propositions is not counting distinct authorized keys. Anchoring had the same problem, and MIL/2 fixed it by making `anchored` a typing side condition rather than a proposition (`:185`). Authority needs the same fix.
2. **The design has no source for a voting outcome.** There is no snapshot, tally, quorum or proposal object. The category report calls this "open as a library" (`06-governance.md:30`). I agree the tally belongs in a library. But two core hooks are missing, and a library cannot provide them soundly: a distinct-signer threshold source, and a strictly-past snapshot read.

The single-signer boundary is **not** the obstacle the report implies (`06-governance.md:29`). Decision 4 limits how many *signed intents* one stage may bind (`DESIGN-MIL2.md:336`). A 3-of-5 treasury transfer is still one intent (`|S| = 1`) whose authority is a threshold. These are separate axes.

## 2. Five ranked design edits

### Edit 1 (highest): a typed authority source with distinct-key counting

```
AuthSource ::= key(pk) | threshold(k, setRef: policy(id), setVersion: u32)
AuthorityFresh(threshold(k,P,v)) ⇔
   read_auth(policy(P)).version = v  ∧  members = read_auth(policy(P)).keys
 ∧ |{ keyId(sig) | sig ∈ W, LedgerVerify(sig, intentDigest), keyId(sig) ∈ members }| ≥ k
 ∧ 1 ≤ k ≤ |members| ≤ 8
```

- Distinctness is by key identity, not by signature bytes.
- Signatures are checked by the ledger over the Poseidon digest, because ZKIR has no Ed25519 verifier (`DESIGN-MIL2.md:263-264`).
- Using `k_of_n` over authority facts becomes a **type error**.

**Counterexample:** `k_of_n(2, [signedBy(a), signedBy(a'), signedBy(b)])`, where `a` and `a'` are two malleated encodings of one key's signature. The Boolean reaches 2 while only one signer is present.

**Placement:** U0 reserves the `AuthSource` enum in the authority public-input schema. U0 admits only `key`. U1 measures the cost of k ledger signatures (`ROADMAP.md:22`). U2/U3 admit `threshold`.

**Why this touches a U0 boundary:** U0 freezes the authority enum and the public-input schema (`DESIGN-MIL2.md:345`). Adding a set-valued source later would change the digest and the statement. This is the same retrofit argument Decision 4 already accepted (`:336`).

### Edit 2: policy versioning, with in-flight positions pinned

```
amend(P, Δ) : pre(policy(P)).version = v ⇒ post(policy(P)).version = v+1
Intent binds (policyId, version) in its digest.
Stage for intent I with pinned (P, v) and current head v' > v:
   accept only if Δ(v→v') ∈ Preserving(I)  or  consent(I.debtor, v') ∈ W
```

Membership changes (Edit 1) are also `amend` operations on `policy(P)`.

**Counterexample:** A removed signer's approval, collected under v1, is replayed after the set has moved to v2. The version equality in Edit 1 rejects it. Separately, a rate-cap amendment that rewrites Alice's open loan without her consent fails `Preserving`.

**Placement:** The version field in the digest is U0. The preservation predicate is U3 for loans. ZR15/UNI-015 migration stays U5 (`ROADMAP.md:26`).

### Edit 3: snapshot-bound outcomes (library shape, core hook)

The core adds one read form: `checkpoint(cell, t)`, with the premise `t < proposal.createdAt ≤ stageTime` on one clock.

The rule is forward-maintained. Every transfer or delegation stage that changes voting weight also writes a bounded checkpoint cell. A stage never reads arbitrary ledger history.

The library outcome is an anchored observation with this shape:

```
Outcome { proposalDigest, snapshot t, for, against, abstain, eligible@t, num, den }
quorum:  for × den ≥ eligible@t × num      -- literal num/den ⇒ Φ₀ (DESIGN-MIL2.md:160)
pass:    for > against
```

Comparisons use cross-multiplication and never divide, so no rounding role arises. If a library does compute a threshold by division, it must round **up**, against the proposal. Products that exceed the field use the limb rule (`:171`).

**External practice, not a Moriarty rule:** ERC-5805 says `getPastVotes` "SHOULD revert" when the timepoint is at or after the current clock. It also recommends an ERC-6372 clock. See https://eips.ethereum.org/EIPS/eip-5805 and https://eips.ethereum.org/EIPS/eip-6372.

**Counterexample:** In a Beanstalk-style attack (`R6:291-292`, G11), votes are flash-borrowed in the same stage as execution. If the snapshot equals the current timepoint, the attack passes. The strict `<` rejects it. MIL/2 already excludes intra-stage flash traces (`DESIGN-MIL2.md:332`), but a borrow over two stages still needs the snapshot rule.

**Placement:** The `checkpoint` read and the strictly-past premise go in the U0 cell vocabulary as a reserved sort. The tally library is U6.

### Edit 4: one vote per voter via a linear receipt; vote escrow as an encumbrance

```
castVote(p, voter, w): consumes replay((p, voter)) once;
   w = checkpoint(power(voter), p.snapshot)
voteEscrow(owner, A, amt, unlock) := Encumbrance{ owner, asset A, amt,
   against: {govLock}, enforceable_by: none }
   -- balance unchanged, K1 holds
power = Σ_i  amt × c_i  over literal tiers c_i by remaining duration
   -- Φ₀; the continuous decay amt × (unlock − now) is Φ₁, deferred
```

Locking is not a supply change. E1 is untouched (`MIL2-PROPOSED-SEMANTICS.tex:173-178`). Debt stays separate from supply.

**Counterexample:** Without the receipt, a voter can vote, transfer tokens, and vote again from a second account. The snapshot blocks this. The receipt stops a re-cast by the same voter.

**Placement:** The receipt and the encumbrance use existing cells (`DESIGN-MIL2.md:240-242`), so this is library work in U6. Continuous decay stays behind Φ₁ and is left deferred; I do not recommend changing that boundary.

### Edit 5: separate voting-power delegation from right delegation; proposal queue

**Voting-power delegation** is single-hop and non-transitive: `delegate(voter) → delegatee`. Power moves at checkpoint time. ERC-5805 practice is also non-transitive, so no cycle can double-count.

**Right delegation** (MIL/2's "delegable") requires:
- `scope(child) ⊆ scope(parent)`;
- one shared affine counter (`DESIGN-MIL2.md:250`);
- depth ≤ 1 until U5;
- revocation of the parent read at the current head, which kills the child.

**The proposal is a tombstoned state machine** with the same pattern as the escrow rule (`MIL2-PROPOSED-SEMANTICS.tex:192-198`):

```
proposed → active → (defeated | succeeded)
succeeded → queued(eta = queuedAt + delay)
queued → (executed | canceled | expired)
execute: after(eta) ∧ ¬tombstone(p) ∧ effects ≡ decode(proposalDigest)
         -- an exact-effect analogue of C1
```

A signed rule decides the execute-versus-cancel race, like escrow `priority`.

**Counterexample:** A proposal passes to "set fee to 25", but the executor submits "set fee to 25 and change the recipient". The exact-effect equality rejects it (compare UNI-015's beneficiary-change hostile case, `R6:286`).

**Placement:** Proposal lifecycle and delegation go in the U6 library. Depth-bounded right delegation goes in U5, together with OWS delegation (`ROADMAP.md:26`).

## 3. Core versus library boundary

**Core:**
- `AuthSource` and the distinct-key counting rule;
- `policy(id)@version` with authenticated reads and `amend` version increments;
- intent pinning of the policy version;
- the `checkpoint` read with its strictly-past premise;
- the typing ban on authority facts inside `k_of_n`;
- replay receipts and tombstones.

A library cannot supply these because each one is an acceptance premise. Without them, a host could compute the result, which the stage relation forbids (`MIL2-PROPOSED-SEMANTICS.tex:130`).

**Library (U6):** tallies, quorum formulas, vote escrow tiers, voting delegation and the proposal/queue machine.

**Out of scope:** deliberation, social capture, off-chain votes. An off-chain vote may enter only as `imported(Policy)`, as a named trust premise (`DESIGN-MIL2.md:187`), consistent with `wiki/defiformal-taxonomy.md:185-192`.

The optional federated kernel's U5 threshold is a **service** policy (`ROADMAP.md:26`). It must never be the source of program authority.

## 4. Smallest implementable slice and evidence pair

**Slice:** One stage on one domain amends a single fee parameter under a 2-of-3 threshold.

- `policy(treasuryFee)@v1 = {keys: {K1, K2, K3}, fee: 30}` → `@v2 = {fee: 25}`.
- The intent digest binds `(policyId, v1)`.
- Φ₀ only; no transfers; replay is consumed.
- It depends on U1 measuring k-signature ledger checks.
- Its natural placement is as U2's "structurally contrasting program" (`ROADMAP.md:23`).

**Positive control:** Valid signatures from K1 and K3 over the digest. The authenticated read gives v1. Post-state is version 2 with fee 25, and replay is consumed once. Expected result: accept.

**Hostile control:** The same bytes, except the second signature is a re-encoded, valid K1 signature instead of K3. The envelope is well-formed, the signatures verify, and the digest and version are correct. Only the distinct-key count fails (1 < 2). Expected result: reject, with a proposed code `AUTH_THRESHOLD_DISTINCT`.

This control isolates the semantic check, as `ROADMAP.md:40` requires.

**Secondary hostile control:** Valid K1 and K2 signatures after K2 was removed in a v1→v1′ membership amendment. Expected result: reject on the version mismatch.

## 5. Explicit disagreements

1. **With `06-governance.md:29`:** Threshold approval should not wait for multi-signer stage arity. Tying them together would push treasury multisig past U4 (`DESIGN-MIL2.md:351`). They are separate axes (Edit 1).
2. **With `06-governance.md:30`:** "Out of scope for Core" is right for the tally but wrong for snapshot reads and distinct-signer counting. Those are core premises.
3. **With `DESIGN-MIL2.md:143`:** `k_of_n` should remain for evidence and guard propositions only, and be a type error over authority facts.
4. **With an implicit reading of `06-governance.md:30`:** Quorum needs no Φ₁. Literal-ratio quorum is Φ₀. Only continuous vote-escrow decay and share-weighted power need Φ₁.
5. **With `DESIGN-MIL2.md:193`:** `enforce` must not become a guardian pause power. A pause needs its own policy-cell transition, which is outside my assigned scope here.

## 6. Residual assumptions

- The Midnight ledger can verify k signatures over one digest in one transaction at acceptable cost. This is unmeasured and is a U1 item.
- Key independence and committee honesty cannot be proved. They remain trust premises.
- Forward-maintained checkpoints assume every weight-changing stage writes its checkpoint cell. This depends on derived-footprint containment (O6, `MIL2-PROPOSED-SEMANTICS.tex:252`).
- The signer-set cap of 8 reuses the `k_of_n` cap proposal (`DESIGN-MIL2.md:278`), which is not yet measured.
- Nothing here gives liveness: a queued action's deadline does not guarantee inclusion (`DESIGN-MIL2.md:219`).
- ERC-5805 and ERC-6372 are cited only as external practice. RFC 9591 (FROST, https://www.rfc-editor.org/rfc/rfc9591) would be an alternative that aggregates signatures off-ledger. It would hide signer distinctness inside one signature and need its own trust premise, so it is not recommended for the first slice.