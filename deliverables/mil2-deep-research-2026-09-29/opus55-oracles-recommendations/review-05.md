# Oracles and observations: MIL/2 adversarial controls and release plan (independent recommendation)

**Status.** This is a read-only design review. The guarded status command reported `SP01.6 loan-swap-subset` with unresolved operational history and no pending transactions. Nothing below is implemented, proved, or accepted by a ledger. Every edit is labelled **Recommendation**.

## 1. Verdict

**Not fit to freeze as specified.** The type-level idea is right. The evidence class and domain are type indices, and source sets propagate through operators (`DESIGN-MIL2.md:176-185`). But four attack paths remain open at the carrier, admission and lowering boundaries:

- **Forged label.** Nothing defines where an observation's `ε@d` label comes from. The label must come from the verification path, not from the witness. `DESIGN-MIL2.md:185` states this as an obligation only. The U0 observation item is still five free strings with no value, unit, identity or sequence (`openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json:239-268`). R5 G5 records that authenticity is a host Boolean today (`R5-oracles.md:332-342`).
- **Stale or wrong-unit value.** `fresh` compares against a `stageTime` whose source and bound direction are not defined (`DESIGN-MIL2.md:144,187`).
- **Status replay.** No observation sequence, no consumption mark and no key epoch exist. The `replay(id)` cell (`DESIGN-MIL2.md:242`) is never tied to observations.
- **Short-circuit laundering.** The proposed semantics lets a skipped branch "not fail dynamically" (`MIL2-PROPOSED-SEMANTICS.tex:95`). Separately, the captured ZKIR v3 text shows that a guarded `PublicInput` whose guard is false yields `default(t)` (`source-text/zkir-v3-spec.md:585`). Unless lowering forbids it, a disabled branch can therefore feed a zero or default into shared arithmetic.

## 2. Five ranked design edits

### E1. The verifier assigns the observation label; the witness never does (highest)

**Recommendation.** Replace the U0 observation leaf with this carrier:

```
ObsRecord ::= { obsId = H(tag_obs ‖ feedId ‖ epoch ‖ seq),
                feedId, policyRef, epoch, seq, unitTag = (B,Q,s),
                value, observedAt: Instant(c), commit }
```

Resolve the label with this rule:

```
FeedDecl(I).feedId = f  ⇒  (unitTag, clock c, admissible ε) := FeedDecl(I)  -- from the signed template
Verify_ε(rec, readProof) = ok                                               -- native constraint or ledger rule
──────────────────────────────────────────────
Γ ⊢ rec.value : Price<B,Q,s> ! {obsId}   with   λ(obsId) = (ε, d_verified)
```

- The label is the **output** of the verifier that actually ran: anchored ledger read, ledger-checked signature, or (later) recursive import.
- The unit and scale come from the signed `FeedDecl`, never from the delivered bytes.
- `commit` binds `unitTag` and `feedId`, so a reciprocal or rescaled price fails `Verify` rather than the arithmetic.
- Delete the `checks.observationsAuthentic` host Boolean (`R5-oracles.md:332-336`).

**Counterexample closed.** The witness supplies feed Y's record, with value, scale and time identical to feed X, plus a witness bit claiming `anchored`. Under today's leaf this is indistinguishable. Under E1 it fails with `OBS_WRONG_FEED`, because the read proof resolves to Y's cell.

**Placement.** U0 (schema leaf, rule, enforcement-map row), U1 (certified authenticated-read primitive), U2 (native binding).

**Why change the U0 boundary.** A five-string leaf with no value cannot be a premise (R5 G1, `R5-oracles.md:294-303`).

### E2. Validate evidence at admission, independent of branch; lower observations without guards

**Recommendation.** Put `EvidenceValid` in the existing admission phase ("full Pre/Args/Obs snapshot validation" before reduction, `MIL2-PROPOSED-SEMANTICS.tex:51`), for every observation in the **statically derived** read set:

```
∀i ∈ DerivedReads(stage).obs:  Verify(i) ∧ ReplayOK(i) ∧ DeclFresh(i)   -- before any reduction
Lower: every observation public input is unguarded; And/Or lower to constrained
       boolean arithmetic over both operands (constrain_to_boolean, DESIGN-MIL2.md:265).
```

This amends `.tex:95`. A skipped branch still costs no *work*, but it cannot skip *evidence validation*. The first profile also forbids dynamic source-set narrowing (`.tex:83`).

`final(obs)` and `attested(obs,k,n)` move out of Φ (`DESIGN-MIL2.md:145`) into declaration-level side conditions, as `anchored` already did. Otherwise `attested(o,3,5) or true_ish` makes the evidence floor optional.

**Counterexample closed.**

```
guard = (collateral_ok_via_anchored) or (price_imported.value × 150 ≥ L)
```

Here a stale `price_imported` sits in the right operand. The left operand is true, so the right is never reduced, and the stale or unauthenticated record is accepted into the bound statement. A second variant: the lowered guarded read yields `default = 0`, and a later `min(…)` uses it.

**Placement.** U0 (admission-order rule), U1/U2 (lowering certificate, hostile witness).

### E3. Observation replay and status consumption

**Recommendation.**

```
Price-type obs:   seq > hw(feedId, consumerScope)        ; write hw' := seq   (cell observation(feedId) becomes R/W)
Event/status obs: (delivered | discharged | nonreceivedProved | attestation):
                  replay(obsId ‖ episodeId ‖ purpose) ∉ consumed  ; consumed' ∪= {…}
Key epoch:        epoch = currentEpoch(policyRef) at authenticated head ∧ ¬revoked(key, head)
Domain tag:       signed message includes (domain, policyRef, purpose)
```

Signed intent picks either `monotone` or `window-reuse` for price observations; the default is `monotone`. Status observations are always one-shot per purpose.

**Counterexamples closed.**

- A `delivered(B, ≥20 B)` attestation that released escrow E1 is replayed to release E2.
- An old in-window quote is re-submitted after a lower quote is published, to avoid liquidation.
- A pre-revocation signature is used after key rotation.

**External practice (not a Moriarty rule).** IBC ICS-004 orders delivery by per-channel sequence numbers and uses receipts to reject double receipt (https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md; captured at `source-text/ibc-ics004.md:275-318`). W3C Bitstring Status List makes `revocation` irreversible and `suspension` reversible. Its `ttl` "does not override or replace the validity period" (https://www.w3.org/TR/vc-bitstring-status-list/; `source-text/w3c-status-list.md:327,467`). VC Data Model 2.0 treats replay as a separate threat (https://www.w3.org/TR/vc-data-model-2.0/#replay-attack).

**Placement.** U0 (carrier fields and the `replay` purpose key), U2 (price high-water on Midnight), U3 (status one-shot for conditional settlement, matching the late-race rule at `ROADMAP.md:24,40`), U4/U5 (imported key epochs).

### E4. Role-directed time bounds for freshness

**Recommendation.** Freshness has a conservative direction, like rounding does. `stageTime` is not a witness; it is the ledger-enforced validity interval `[t_lo, t_hi]` of the transaction.

```
fresh(o, Δ):   o.observedAt ≤ t_lo   ∧   t_hi ≤_checked o.observedAt + Δ     (Reject on overflow)
before(T):     t_hi < T          after(T):  t_lo ≥ T
```

A freshness check uses the **latest** possible execution time. A future-date check uses the **earliest**. Clock identity comes from `FeedDecl.clock = intent.clock` (`DESIGN-MIL2.md:71-73`). `Δ` is a signed literal, and a mismatched clock is a type error. The optional heartbeat or publication-lag policy stays in `FeedDecl`.

**Counterexample closed.** A prover picks `stageTime` equal to `observedAt + 4m59s` while the transaction's validity window runs to +20m. The stage is accepted, and the quote is 20 minutes old when it executes.

**Placement.** U0 numeric profile (`ROADMAP.md:21` already requires per-primitive direction; extend it to time), U2 (bind `[t_lo, t_hi]` to the ledger rule).

### E5. The intent owns the evidence floor; threshold means distinct issuers

**Recommendation.** Add a signed `EvidencePolicy` clause that the acceptance relation evaluates itself, outside the program's guard. This closes R5 G3 (`R5-oracles.md:315-322`):

```
Stage ⇒ ∀ f ∈ I.requiredFeeds: ∃! o ∈ obs. o.feedId = f ∧ EvidenceValid(o) ∧ I.floor(f)(o.value)
attested(k,n): |{ issuer(sig) | sig ∈ sigs, sig over same (obsId, value, observedAt) }| ≥ k,
               issuers ⊆ policy.members(epoch), duplicates rejected
```

**Counterexample closed.** A proposer's program replaces `price × 150 ≥ L` with `true`. Or one issuer signs three times toward a 3-of-5 threshold. Or three signers sign three different values.

A fallback feed must be a signed alternative in `EvidencePolicy`, never an unsigned downgrade (`MORIARTY-CONSOLIDATED-DESIGN.md:97`).

**Placement.** U0 (clause in the `intent` judgment), U2 (enforcement), U4/U5 (threshold membership and equivocation).

## 3. Core versus library boundary

**Recommendation: core.**
- `ObsRecord` and label derivation (E1)
- Admission-order validation and unguarded observation lowering (E2)
- Sequence, high-water, status consumption, key epoch (E3)
- Role-directed time (E4)
- Intent evidence floor, and distinct-issuer threshold semantics (E5)
- Source-set typing with no dynamic narrowing
- Nonzero price as a `FeedDecl` formation premise, not a guard

These are core because a library cannot enforce any of them against a hostile proposer.

**Recommendation: library, after bounded collections (`DESIGN-MIL2.md:360`).**
- TWAP, median
- Deviation and circuit-breaker policies
- Pause authority
- Fallback selection among signed alternatives
- Feed-registry entries and manipulation mitigations

Market truth stays an external premise (`MORIARTY-PRODUCT-CONTRACT.md:47`).

**Φ₁ unchanged.** With a fixed signed collateral (150 A) and debt (50 B), the ratio check is `price.value × 150 ≥ 50 × r_lit`. That is literal-coefficient, so Φ₀ (`DESIGN-MIL2.md:160`). Cross-multiplication makes it exact with no rounding. Hole-sized collateral would need `qty × price`, which is Φ₁. It stays deferred and does not justify moving the boundary.

## 4. Smallest implementable slice and evidence pair

**Recommendation.**
- **U2 slice.** One signer, one Midnight-resident feed cell (anchored), one Φ₀ guard `price.value × 150 ≥ K`, `fresh(price, 5m)` under E4, and price high-water under E3.
- **Bound statement fields:** `obsId`, `feedId`, `unitTag`, `value`, `observedAt`, `seq`, `[t_lo, t_hi]`, intent digest, and complete effects (`.tex:240`).
- **No** `imported`, `attested`, collections or joins in this slice.

**Positive control.** The feed publishes seq 7 with a value meeting K, 2 minutes before `t_lo`. The high-water is 6. The stage is accepted, binds `obsId(7)`, writes high-water 7, and produces complete loan effects.

**Hostile control.** Byte-identical to the positive, except the read proof points at a second deployed feed cell. That cell has the same `unitTag`, the same value, the same `observedAt` and seq 7. Expected outcome: `OBS_WRONG_FEED` from the native relation.

The envelope is well formed and every arithmetic and freshness clause would pass. So the rejection isolates identity binding, which meets the requirement that hostile controls not be malformed envelopes (`ROADMAP.md:40`).

**Follow-on hostile controls** (one per edit):
- replaying seq 7 after it is consumed → `OBS_REPLAY`
- `t_hi` = `observedAt` + 6m → `OBS_STALE`
- the E2 short-circuit counterexample
- a reciprocal `unitTag`
- a guard replaced with `true`

## 5. Explicit disagreements

1. **With `DESIGN-MIL2.md:187`.** "`imported` remains a named trust premise until U4" admits a verified-by-nobody value before U4. **Recommendation:** before U4, an `imported` observation is admissible only if its signature is ledger-checked or verified in-circuit with a Poseidon-friendly scheme. There is no in-circuit Ed25519 (`DESIGN-MIL2.md:264`). Otherwise the stage rejects. A named premise must not become a host Boolean.
2. **With `.tex:95`.** Skip-without-failure is acceptable for work accounting, not for evidence (E2).
3. **With `.tex:83` and `05-oracles.md:17`.** Both leave room for dynamic source-set narrowing given a theorem. For the first profile, forbid it outright. The benefit is small and the attack surface is the whole of O2.
4. **With `DESIGN-MIL2.md:145`.** `final` and `attested` should not be Φ propositions, because disjunction can bypass them.
5. **With `05-oracles.md:19`.** The category review treats positive-value checks as policy. Nonzero belongs in `FeedDecl` formation, because division and inversion depend on it.

## 6. Residual assumptions

- Midnight exposes an authenticated validity interval that ZKIR can bind as a public input or check through a ledger rule. **Unverified.**
- A Midnight-resident feed contract can serve as an anchored source, with the ledger authenticating its current state. The slice presumes this; it is **not demonstrated**.
- A Poseidon-friendly signature scheme is certifiable at U1 at acceptable cost. **Unmeasured.**
- Feed issuers are honest about market values. This is outside proof, and no edit here addresses manipulation.
- The high-water write adds contention on `observation(feedId)`. Per-consumer scoping is assumed to be enough for the `|S| = 1` profile (`DESIGN-MIL2.md:336`).
- The §12 caps are unmeasured (`DESIGN-MIL2.md:270`).
- Obligation O2 (`.tex:248`) remains undischarged under every edit above.