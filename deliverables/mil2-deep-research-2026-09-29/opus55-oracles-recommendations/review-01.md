# Recommendation: Oracles and observations. Lens: observation identity, provenance, authenticated read and status

**Status:** this is a specified-only design review. I ran the guarded status command. It reported `SP01.6 loan-swap-subset` with unresolved operational history and no pending transactions. That does not block read-only review. I claim no implementation, proof or ledger acceptance.

## 1. Verdict

**Partial. It is not freezable for observations yet.** MIL/2 has the right pieces:
- type-indexed evidence (`DESIGN-MIL2.md:176-178`)
- source-set propagation (`:184`)
- `anchored` as a typing side condition bound to an authenticated read (`:185`)
- a same-clock `fresh` rule that rejects future timestamps (`:187`)

Four identity defects remain:

1. **The two documents disagree on what a source set contains.** The design keys it by class and domain, `S ⊆ {ε@d}` (`DESIGN-MIL2.md:178`). The proposed semantics keys it by observation *identity* (`MIL2-PROPOSED-SEMANTICS.tex:67,72`). Keying by class and domain cannot tell feed A/B from feed B/A, or round *r* from a replayed round *r−k*.
2. **"Where the value was read" and "who asserted it" are one index.** The showcase types a third-party feed as `! {anchored@midnight.preview}` (`DESIGN-MIL2.md:303-304`). An oracle's claim, once written to a Midnight cell, then passes as a ledger-native fact. That is laundering by typing, not by arithmetic.
3. **Nothing defines an observation's identity, unit, scale, sequence or status.** The U0 schema item is still five free strings with no value (`stage-relation.schema.json:239-268`). The current evaluator's authenticity check is a caller-supplied Boolean (`R5-oracles.md:215`).
4. **Time provenance is under-specified.** `fresh` requires one clock (`DESIGN-MIL2.md:144`). But an issuer's timestamp is on the issuer's clock, not the Midnight clock. `now` is resolved at signing (`:78`), so it cannot serve as the stage time.

## 2. Five ranked design edits (recommendations)

### E1. A canonical observation identity, with source sets keyed by identity

**Rule sketch (recommendation):**
```
ObsRec   ::= { feed: FeedId, assertor: Principal, keyEpoch: u32, round: u64,
               type: T, unit: (B,Q) | AssetId | Dimensionless, scale: u8,
               value: T, observedAt: Instant(c_a), clock: c_a, presence: {1} }
ObsId    = Poseidon(tag_obs ‖ canonical(ObsRec))

Π(feed) = {type T, unit U, scale s}   decl.feed = feed   rec.unit = U   rec.scale = s
───────────────────────────────────────────────────────────────────────
Σ;Γ;phase;Π ⊢ o.value : T⟨U,s⟩ ! {ObsId(o)}
```
- The source set L is a set of `ObsId`s. λ(i) returns (locus, assertor, domain), defined in E2.
- The design's line 178 should be changed to match the semantics at tex:67.
- `Price<B,Q,s>` formation takes its unit from the feed declaration signed in the template (tex:109). It never takes it from delivered bytes.

**Counterexample:**
- Two feeds, `A/B` at scale 6 and `B/A` at scale 6, are both `anchored@midnight`. Under `{ε@d}` both have source set `{anchored@midnight}`, so swapping them passes provenance.
- A round-*r−k* record with a fresher-looking value but the same class and domain also passes.
- Under E1, both are different `ObsId`s. The first fails with `WRONG_FEED`/`WRONG_UNIT`; the second is caught by E4's round rule.

**Placement:** U0, which owns the canonical encoding and schema (`DESIGN-MIL2.md:262,344-345`).

**U0 boundary change:** the U0 `observations[]` item gains these fields. U0 hash-binds the canonical form. Adding identity fields after the freeze changes every digest, so this cannot wait.

### E2. Separate the read locus from the assertor

**Rule sketch (recommendation):**
```
ε ::= (locus, assertor)
locus    ::= ledger(d) | imported(Policy) | signed
assertor ::= native(d)            -- ledger-maintained: balance, supply, clock, tombstone
           | issuer(IssuerId, KeySetPolicy)

position requires native@d : admits t iff ∀i∈L. λ(i) = (ledger(d), native(d))
position requires issuer-policy P@d :
   admits t iff ∀i∈L. locus(i)=ledger(d) ∧ assertor(i) ∈ P   (or signed, E3)
```
An `observation(feedId)` cell (`DESIGN-MIL2.md:241`) always has assertor `issuer(…)`, even though it is read from the ledger. Surface `anchored@d` becomes shorthand for `(ledger(d), native(d))`. A feed declaration must write `from feed F by issuer P`.

**Counterexample:** a lending intent requires anchored collateral balances and accepts only a named oracle's price. Suppose any account can post to a price cell, or the pinned oracle's key is replaced. Under the current single index, that cell is `anchored@midnight.preview` and satisfies the anchored position. The *assertor* has been laundered into a *ledger fact*. The product contract says external attestations remain named assumptions (`MORIARTY-PRODUCT-CONTRACT.md:47`). Only a separate assertor index enforces that at the type level.

**Placement:** U0, as a change to the type index. Enforcement follows in the first U2 slice that claims an observation. The R5 audit records that TP03 blocks U2 only when a program claims an observation (`R5-oracles.md:273-274`).

### E3. Bind every statically named observation in the native statement, with no host Boolean

**Rule sketch (recommendation):**
```
EvidenceValid(o) ⇔ for each i ∈ L_static(program):
   ledger(d):  circuit reads cell observation(feed_i) at predecessorHead via an
               authenticated ledger read; ObsId_i recomputed in-circuit from the read
               record; ObsId_i ∈ unguarded public inputs
   signed:     signature over ObsId_i verified by a ledger validity rule
               (not in-circuit: no Ed25519/SHA-512 in ZKIR v3, DESIGN-MIL2.md:264)
   imported:   U4
```
- Every identity in the **static** source set is bound, including identities used only in a skipped `And`/`Or` branch (tex:83,95).
- An explicit presence bit is constrained to 1.

**Decisive native fact (source fact).** In ZKIR v3 (`source-text/zkir-v3-spec.md:589-601`), a guarded-off `Impact` pushes **zeros** as public inputs. If observation bindings are lowered inside the guard that uses them, a skipped observation and a real zero-valued observation cannot be told apart in the public statement. Hence two requirements: unguarded binding, and a presence bit so that value 0 is not the same as "missing". A zero price is then rejected by an explicit positivity guard, not by an ambiguous public input.

**Counterexample:** under the atomic profile, the prover supplies a witness price and `checks.observationsAuthentic = true` (`R5-oracles.md:215`). Under E3, a witness price that differs from the ledger cell fails the recomputed `ObsId`.

**Placement:** U0 field-by-field enforcement map (`ROADMAP.md:21`). The anchored binding is in the U2 slice, the signed binding in U3, and the imported binding in U4 (`DESIGN-MIL2.md:350`).

### E4. Time provenance: the assertor's clock, an authenticated stage time, rounds

**Rule sketch (recommendation):**
```
stageTime : Instant(c_d) ! {native(d)}      -- authenticated ledger clock read, not `now`
Π declares clockMap(c_a → c_d, skew σ ≥ 0, unitRatio literal)

fresh(o, H) ≜  map(o.observedAt) ≤ stageTime            (FUTURE otherwise)
             ∧ stageTime − map(o.observedAt) + σ ≤ H     (STALE otherwise)
map overflow ⇒ Reject(CLOCK_RANGE); mismatched clock without clockMap ⇒ type error
monotone(feed) policy: o.round > lastRound(feed) read from replay(feed) cell  (optional)
```
- The skew direction follows the role, like role-directed rounding. Skew always *adds* to the age and is never subtracted from the future check. Uncertainty therefore resolves against the observation.
- The heartbeat is a separate feed-policy bound on how far apart rounds may be. I agree with the category report that it is not implied by `fresh` (`05-oracles.md:15`).

**Counterexample:** an issuer's Unix-seconds timestamp typed directly as `Instant(midnight.preview.clock)` satisfies the same-clock rule by fiat. The same laundering happens if the evaluator compares against `now` fixed at signing (`DESIGN-MIL2.md:78`): a quote 23 hours old inside a 24-hour validity window would pass `fresh(price, 5m)`, because `now` is the signing time.

**Placement:** the typing rule and rejection codes go in U0. Boundary, stale, future and skew negatives go in the U3 corpus. The authenticated clock read belongs in the U2 slice.

### E5. Issuer key status evaluated at stage time, with distinct rejection codes

**Rule sketch (recommendation):**
```
StatusOK(i) ⇔ read policy(KeySetPolicy_i) at predecessorHead:
     keyEpoch_i ∈ activeEpochs ∧ status(key_i) ∉ {revoked, suspended}
     ∧ (revokedAt(key_i) undefined ∨ observedAt_i < revokedAt(key_i) ∧ policy.allowPreRevocation)
Reject codes (distinct, fail closed): OBS_MISSING, OBS_WRONG_FEED, OBS_WRONG_UNIT,
  OBS_UNAUTHENTICATED, OBS_STALE, OBS_FUTURE, OBS_REVOKED, OBS_SUSPENDED, OBS_STATUS_UNKNOWN
```
- Status is itself a native anchored read and appears in L.
- An unreadable status is `OBS_STATUS_UNKNOWN`, never "active".

**External practice (not a Moriarty rule):** the W3C Bitstring Status List makes `revocation` irreversible and `suspension` reversible (https://www.w3.org/TR/vc-bitstring-status-list/, captured at `source-text/w3c-status-list.md:327`). This supports keeping them as separate codes.

**Counterexample:** a key is compromised and rotated at epoch *e+1*. A record correctly signed under epoch *e*, with an in-window `observedAt`, still passes signature and freshness checks. Only a status read at the predecessor head rejects it.

**Placement:** the vocabulary and codes go in U0. Enforcement is in U3 (TP03 blocks U3, `R5-oracles.md:275`). Federated epoch migration stays in U5 (`ROADMAP.md:26`).

## 3. Core versus library boundary (recommendation)

**Core:**
- `ObsRec`/`ObsId` and the formation rule
- the identity-keyed source set
- the locus/assertor index
- static-L native binding with a presence bit
- the authenticated `stageTime`, `clockMap` and `fresh`
- status reads
- the rejection codes
- `k_of_n`/`attested` counting **distinct assertor identities** over one shared `(feed, round, value)`

That last item must be core because distinctness can only be checked on core identities.

**Library** (after bounded collections, `DESIGN-MIL2.md:360`):
- median and TWAP
- deviation and circuit-breaker policies
- fallback chains, which must be signed alternatives and must not weaken policy
- feed adapters
- heartbeat presets
- the Atlas `Ex`/`Tp`/`Oa`/`At` taxonomy and bond laws (U6)

Oracle truth stays out of scope (`MORIARTY-PRODUCT-CONTRACT.md:47`).

## 4. Smallest implementable slice and evidence pair (recommendation)

**Slice (U2 scope, Φ₀ only).** A pinned issuer contract on `midnight.preview` writes `observation(feed "A/B")` holding `ObsRec` with a price at scale 6. The borrow stage:
- performs an authenticated read at the predecessor head and recomputes `ObsId`
- reads `stageTime` from the ledger
- checks `fresh(price, 5m)` and `price > 0`
- checks the collateral ratio using literal coefficients only: `price × 150·10^6 ≥ 50·10^6 · 10^6 · ratioLit`

The ratio check needs no Φ₁ and no rounding, and falls under the limb rule if it exceeds the field (`DESIGN-MIL2.md:169-171`). The stage binds `ObsId`, the presence bit and complete effects, including the debt record, which stays separate from supply.

**Positive control.** A valid issuer write at round *r*, with `observedAt = stageTime − 60 s` and a price that satisfies the ratio. Expected: the native proof verifies, and the public statement contains the exact `ObsId`.

**Hostile control (a valid envelope, per `ROADMAP.md:40`).** The same stage and witness shape, but the prover's witness value differs by one unit from the ledger cell and still satisfies the ratio. Expected: native rejection from the `ObsId` recomputation, not an envelope or schema error.

**Secondary hostiles:** a `B/A` feed cell, round *r−1*, a future `observedAt`, a suspended key, and the guard replaced by `true` with the policy unchanged.

## 5. Explicit disagreements

- **With `DESIGN-MIL2.md:178`:** source sets must hold identities, as the semantics has it at tex:67. Class and domain are not enough.
- **With `DESIGN-MIL2.md:303-304`:** typing a feed `anchored@midnight.preview` launders the assertor. It should read `by issuer P`.
- **With the kernel-style K1 environment model** (`R5-oracles.md:72-79`), partially. For `ledger(d)` observations, the observation must be an authenticated ledger cell, not a host-supplied environment. An environment-style input is acceptable only for `imported`, at U4.
- **With the category report's "partial" for feed identity** (`05-oracles.md:14`): I rate it **open**. There is no identity record at all.
- **With a U0 schema that binds `observations[]` as empty** (`R5-oracles.md:233`): that is acceptable for S0 only if U0 still freezes the item's *shape* (E1). Otherwise it is fine.

## 6. Residual assumptions

- A Midnight circuit can authenticate a read of another contract's cell at `predecessorHead`. This interface is unverified; U1 must measure it.
- The ledger exposes a clock of known granularity to circuits. Unverified.
- Signed observations need a ledger-side signature check, because ZKIR v3 has no in-circuit Ed25519 (`DESIGN-MIL2.md:264`).
- Poseidon domain tags for `tag_obs` are distinct from the intent-digest tag.
- Issuer honesty, market truth and manipulation resistance remain external trust premises (TP03, `R5-oracles.md:222`).
- Skew and heartbeat values are signed policy, not measurements.
- Φ₁ and the deferred features are unchanged.