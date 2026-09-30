# Oracles and observations: recommendation on imported and attested trust policy

*Independent Opus 5.5 recommendation. This is read-only, specified-only design work. I ran the guarded `status` command, which reported SP01.6 with unresolved operational history and no pending transactions. I did not run tests, proofs, compilation or transactions, and I claim no implementation, proof or ledger acceptance.*

## 1. Verdict on category design fitness

**The design is not yet fit to freeze for trust policy. It is fit for its provenance core.** MIL/2 correctly makes evidence class and domain type indices and propagates source sets (`concepts/intent-language/DESIGN-MIL2.md:176-185`). The proposed semantics correctly keeps observation *identity* and a policy reference (`MIL2-PROPOSED-SEMANTICS.tex:67,83`).

The trust layer itself has five gaps:

- **`Policy` is only a name.** In `ε ::= anchored | imported(Policy) | attested(Issuer,k,n)` (`DESIGN-MIL2.md:177`), `Policy` has no content, digest, key-epoch model, revocation model or reuse scope.
- **`final(obs)` and `attested(obs,k,n)` are Φ propositions** (`DESIGN-MIL2.md:145`). A program can therefore choose, weaken or disjoin them away. This recreates R5's G3 bypass (`u0-study-2026-09-28/defi-coverage/R5-oracles.md:315-322`).
- **"`imported` remains a named trust premise until U4"** (`DESIGN-MIL2.md:187`). If nothing verifies an imported observation, the premise can only be discharged by a caller-supplied `observationsAuthentic` Boolean. That is exactly the host Boolean R5 G5 rejects (`R5-oracles.md:332-341`), and the brief forbids it.
- **No signature path is identified for `attested`.** No Ed25519/SHA-512 verification is claimed in-circuit (`DESIGN-MIL2.md:264`), and none of the attested path is assigned an enforcement locus.
- **The U0 carrier is five free strings** (`openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json:239-268`). It has no value, policy digest, key epoch, sequence or reuse scope. `finality` is a string that a relayer can simply assert.

## 2. Five ranked design edits (recommendations)

### E1 — The evidence policy becomes a digest-bound signed object, and source labels carry its digest

**Rule sketch.**
```
TrustPolicy ::= { mode: anchored | registry(RegistryId) | foreign(DomainId, VerifierId),
                  statement: StmtType,            -- feedId, value sort Price<B,Q,s>, unit, clock
                  issuers: { setCommitment, k, n, epochRule },
                  maxAge: Duration(c), maxFutureSkew: 0,
                  finality: FinRule, revocation: RevRule, reuse: ReuseMode }
policyDigest = Poseidon(tag_policy ‖ canonical(TrustPolicy))
Obs<T, π, d>      -- π = policyDigest replaces ε
L ⊆ { (obsId, π, d) }
Position requires π*:  admit t iff ∀(i,π,d)∈L. π ⊑_signed π*
```
The ordering `⊑_signed` is **only** reflexivity plus refinements that the template explicitly signs. It has no built-in lattice: two `k-of-n` sets over different issuers are incomparable. Π (`MIL2-PROPOSED-SEMANTICS.tex:44-46`) carries the policies and is part of `SignedBound`.

**Counterexample (current text).** An intent signs `imported("dust-feed")`. After signing, the solver resolves `"dust-feed"` to a different issuer set, or to a 1-of-1 policy with a key it controls. The source set still reads `imported(dust-feed)@d`, so typing passes.

**Placement.** U0. The evidence type index and canonical encoding are already U0 (`DESIGN-MIL2.md:344-345`). This refines that U0 item; it does not move a boundary.

### E2 — Attestation is typed as `Said<I,T>`, verified only at a named enforcement locus, and `imported` is inadmissible until U4

**Rule sketch.**
```
registry(R) observation:  Obs<Said<IssuerSet,T>, π, midnight>
  -- the ledger-authenticated read of registry cell observation(feedId)
  -- binds (seq, value, observedAt, keyEpoch, π)
elim:  Γ ⊢ o : Obs<Said<S,T>,π,d>   π ∈ Π_signed
       ─────────────────────────────────────────────
       Γ ⊢ o.value : T ! {(o.id,π,d)}
Profile U0–U3:  mode ∈ {anchored, registry}; foreign(...) ⇒ Reject(EVIDENCE_MODE_UNSUPPORTED)
```
A registry is a Midnight contract. Its write transition authorizes the issuer quorum under ledger validity rules, so no in-circuit Ed25519 is needed. The consumer then gets an **anchored read of "S said v"**, not an anchored *truth* of `v`. Remove `attested(obs,k,n)` and `final(obs)` as Φ atoms. They become admission premises of π and are checked under `EvidenceValid` (`MIL2-PROPOSED-SEMANTICS.tex:134`).

**Counterexample.** Under today's grammar, `release_when attested(o,2,3) or spent×20 ≤ received×11` lets the prover satisfy the right-hand disjunct. The oracle requirement is optional even though the template "has" it. Under the current `k_of_n` (`DESIGN-MIL2.md:143`), two signatures by the *same* issuer on different messages can also count as 2-of-3.

**Placement.** U0 for the type and profile rule. U1/U2 must certify the registry read path. Foreign verification stays at U4 (`DESIGN-MIL2.md:349-350`). **This changes a U0 statement.** I recommend that U0–U3 *reject* `imported`, not carry it as a premise, because an unverified premise can only be discharged by a host Boolean.

### E3 — Issuer key epochs, rotation and irreversible revocation are checked at the acceptance head

**Rule sketch.**
```
KeyOK(o, stageHead) ≜
    epochActive(o.keyEpoch, o.observedAt)                     -- signed under an epoch valid then
  ∧ ¬compromised_{≤stageHead}(o.keyEpoch)                      -- compromise is retroactive
  ∧ (retired_{≤stageHead}(o.keyEpoch) ⇒ o.observedAt < retireAt)
  ∧ ¬suspended_{stageHead}(policy)                             -- suspension is reversible
RevRule ::= { registryCell, compromise: irreversible, retire: grace, suspension: reversible }
Rejects: OBS_KEY_EPOCH, OBS_ISSUER_REVOKED, OBS_POLICY_SUSPENDED
```
The revocation state is an authenticated ledger read in the footprint as `policy(id)`, a cell MIL/2 already lists (`DESIGN-MIL2.md:242`). The product contract already requires "current revocation" for owner authorization (`docs/MORIARTY-PRODUCT-CONTRACT.md:56`). This applies the same rule to issuers.

**Counterexample.** Suppose only `epochActive(keyEpoch, observedAt)` is checked. An attacker who steals a key that was rotated out at time T signs a price with `observedAt = T − 1s`. The signer chooses `observedAt`, so backdating is free. If the stage is within `maxAge`, it accepts. Separately, if `maxAge` is lax, a retired key with no grace bound keeps its signing power indefinitely. That is why compromise must invalidate retroactively and retirement needs a grace bound.

**External practice (not a Moriarty rule).** W3C Bitstring Status List v1.0 makes `revocation` "not reversible" and `suspension` "reversible" (https://www.w3.org/TR/vc-bitstring-status-list/; captured `source-text/w3c-status-list.md:327`).

**Placement.** U0 for the fields and codes. U2/U3 for enforcement on registries. U5 for federation epoch migration, which the roadmap already requires (`ROADMAP.md:26`).

### E4 — Freshest-value selection, statement domain separation, and linear or affine reuse

**Rule sketch.**
```
Signed statement preimage = (tag_obs, chainId, registry/feedId, seq, value, unit, observedAt, keyEpoch, π)
ReuseMode ::= read-head      -- must equal the current registry head seq (affine read, reusable)
            | read-window(Δseq)
            | consume         -- linear: writes replay(obsId); used for delivery/receipt evidence
FreshHead(o) ≜ o.seq = head(observation(feedId)) ∧ observedAt ≤ stageTime ≤ observedAt+maxAge
```
**Counterexample.**

- **Selection.** `fresh(price,5m)` (`DESIGN-MIL2.md:303-304`) admits any validly signed price from the last five minutes. A borrower picks the most favorable of several fresh quotes and draws more than the current price allows.
- **Cross-domain reuse.** A proof-of-reserves attestation signed for chain A lacks a chain tag. It satisfies an episode on chain B, or the same `delivered` receipt releases two escrows.

**Placement.** U0 must decide whether `observation(feedId)` (`DESIGN-MIL2.md:241`) is a read-only environment cell or a ledger cell with a high-water mark (R5 G4). I recommend a **ledger cell with a head sequence**. `consume` joins U3's receipt linearity.

### E5 — Finality is a typed per-domain outcome with a frozen state, and never a guard atom

**Rule sketch.**
```
FinRule  ::= local | depth(k) | checkpoint(lightClientId) | attestedFinal(π')
FinState ::= unknown | included(h) | final(h, FinRule) | frozen(misbehaviour)
admit o under π  requires  FinState(o) = final(_, π.finality)
frozen ⇒ Reject(OBS_SOURCE_FROZEN); no retroactive rollback of accepted stages;
         the policy names a remedy stage (dispute/compensation) as a residual duty
```
This refines the four outcome states in `MIL2-PROPOSED-SEMANTICS.tex:202`. It keeps "no global cross-domain rollback". A reorg or misbehaviour after acceptance produces an explicit later remedy stage, not an undo.

**Counterexample.** Today `finality` is a free string (`stage-relation.schema.json:239-268`). A relayer writes `"final"` for a block that later reorgs, and the local stage has already released collateral. With `final(obs)` as a Φ atom, a local anchored observation also "is final" vacuously on another domain after crossing an episode edge.

**External practice (not a Moriarty rule).** IBC freezes a client on misbehaviour and requires proof of the frozen state (https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md; captured `ibc-ics004.md:784-795`). IBC v2 checks timeouts against an authenticated time source (https://github.com/cosmos/ibc/blob/main/spec/IBC_V2/core/ics-004-packet-semantics/PACKET_HANDLER.md; captured `ibc-v2-packet-handler.md:253`).

**Placement.** U0 for the vocabulary and `local` only. Foreign `depth`, `checkpoint` and `attestedFinal` verification stay at U4. Nothing moves into Φ₁.

## 3. Core versus library boundary (recommendation)

**Core (U0 semantics):**
- `TrustPolicy`, its canonical digest, and the signed refinement order `⊑_signed`
- `Said<S,T>` and its elimination rule; source labels carrying `(obsId, π, d)`
- The `KeyOK` and revocation semantics
- `FinState` and `FinRule`
- `ReuseMode`, including the `consume` linearity
- The observation public-statement fields (extending `MIL2-PROPOSED-SEMANTICS.tex:240`)
- Distinct rejection codes
- The rule that a **signed fallback** is an ordered list of policies. Its trigger must be positive authenticated evidence, such as an anchored registry head showing the primary is stale by X. A missing primary is never the trigger, because a withheld primary would downgrade evidence (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:97`).

**Library (U6):**
- Concrete registry contracts and feed adapters
- Deviation and circuit-breaker templates
- Proof-of-reserves templates
- Median, TWAP and distinct-feed quorum. These wait on bounded collections (`DESIGN-MIL2.md:360`).
- Pause-authority patterns

**Federated kernel (U5, optional):** it may *operate* registries and collect attestations. Its statements enter only as `registry` or `foreign` evidence under a signed π, never as an acceptance Boolean (`MIL2-PROPOSED-SEMANTICS.tex:59`).

## 4. Smallest implementable slice and evidence pair

**Slice (U2-scoped, single stage, single signer).** The pieces are:

- **Registry cell:** one Midnight registry holds `observation("dust/night") = {seq, value: Price<B,A,6>, observedAt, keyEpoch, π}`. Issuer writes are authorized under ledger rules (a residual assumption, see §6).
- **Signed intent:** mode `registry`, `maxAge 5m`, `reuse read-head`, `RevRule` pointing at the `policy(id)` cell, and `finality local`.
- **Consumer stage:** reads the cell and `policy(id)` through authenticated ledger reads. It checks `FreshHead`, `KeyOK`, the unit and the collateral inequality as a Φ₀ literal-coefficient cross-multiplication under the §4.5 limb rule. It publishes `(feedId, seq, value, observedAt, keyEpoch, π)` in the public statement.

The slice needs no in-circuit signature verification, no foreign finality and no Φ₁.

**Positive control.** The head is `seq = 7`, `observedAt = stageTime − 2m`, epoch `e1` is active and unrevoked, and 150 A satisfies the lender ratio at that price. Borrowing 50 B is accepted, with the complete effects and the new debt recorded separately from supply.

**Hostile control.** The witness is byte-identical except that it substitutes the registry's previous entry `seq = 6`. That entry is validly written by `e1`, sits within five minutes (`stageTime − 4m`), and has a more favorable price. It must reject with `OBS_NOT_HEAD` from the native verifier or ledger check, not from envelope malformation. Before the rejection counts as evidence, the positive run must be shown feasible under the same keys (`ROADMAP.md:40`). A second hostile control marks `e1` as `compromised` before the stage and expects `OBS_ISSUER_REVOKED`.

## 5. Explicit disagreements

1. **With `DESIGN-MIL2.md:187`:** "imported remains a named trust premise until U4" should instead be "imported is rejected until U4" for acceptance. A premise nobody verifies is a host Boolean.
2. **With `DESIGN-MIL2.md:145`:** `final` and `attested` do not belong in Φ as propositions. The category report (`category-review/05-oracles.md:16,20`) keeps them as vocabulary to be defined further. I would move them out of the guard language into π admission.
3. **With `05-oracles.md:15`:** I add the freshest-value selection attack. `fresh` with the right operands is still exploitable without a head or sequence rule.
4. **With `DESIGN-MIL2.md:177`:** `attested(Issuer,k,n)` as a class conflates issuer identity with key material. Rotation needs key epochs *inside* π, not a static issuer name.
5. **U0 boundary change:** the U0 `observations[]` schema must gain `obsId, value, unit/scale, observedAt, seq, keyEpoch, policyDigest, finState, reuse`. This is justified because U0 cannot hash-bind unspecified fields (`DESIGN-MIL2.md:262`). R5 G8's `source/5` regression needs a recorded restoration decision at U0 (`R5-oracles.md:364-374`). Φ₁ deferral is unchanged.

## 6. Residual assumptions

- The pinned Midnight ledger can authorize registry writes with an issuer quorum under ledger validity rules, at acceptable cost. This is unverified and belongs to U1.
- A registry's issuers and a threshold signature attest to *what was said*, never to market truth (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:99`). Manipulation remains an application-policy risk.
- The operator of the revocation cell is itself a trust root. Its update authority must be named in π.
- The on-ledger `stageTime` source is authenticated and shares π's clock.
- `maxAge` and head-sequence liveness depend on issuer arrival. Stale-feed refusal is safety, not liveness.
- Foreign finality rules (`depth`, `checkpoint`) have U4-deferred verifiers. Until then, cross-domain evidence cannot guard acceptance.