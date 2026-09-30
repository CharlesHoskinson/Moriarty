# Recommendation: one exact observation-conditioned stage, with public and private transcript bindings (Oracles and observations)

**Status:** This is a read-only recommendation, based on the design documents only. I did not compile, prove or test anything, and nothing here claims ledger acceptance. I ran the guarded `status` command. It reports `SP01.6 loan-swap-subset` as blocked on operational history, with no pending transactions. That block does not affect this review.

## 1. Verdict

**The category is not yet fit for a U0 freeze. The core is repairable, and the repair is small.** The vocabulary is sound: evidence carried as a type index, source sets that track which observations influenced a value, the `observedAt ≤ stageTime` rule, and `observation(feedId)` as a declared cell (`DESIGN-MIL2.md:176-187,241`). Three problems specific to compiling this to native circuits block the category's own core scenario:

1. **No rule ties an observation to the circuit's inputs.** In ZKIRv3, `PublicInput` values are "witnessed freely … constrained only to inhabit `type`" in-circuit (`source-text/zkir-v3-spec.md:606-608`). Only an `Impact` group puts values into the public-input vector π (`:589-602`). MIL/2 says anchored reads are "authenticated" (`DESIGN-MIL2.md:185`) but never says which ZKIR mechanism does this. Without that rule, any price the prover chooses satisfies the circuit.
2. **The lending scenario falls outside Φ₀.** The report's core transaction is "150 A pledged, ratio against feed A/B" (`05-oracles.md:7`). That needs `qty × obs.value`, a product of two variables. §4.4 excludes such products from Φ₀ and defers them to Φ₁ (`DESIGN-MIL2.md:160-161`). So the category's central example cannot be written in the U0 profile as it stands.
3. **The circuit has no scalar for `stageTime`.** `fresh` compares against a stage time that is not a circuit value (`DESIGN-MIL2.md:144,187`). Only a time bracket checked by the ledger can supply it.

On top of these, the stage schema's observation item is still five free strings with no value (`stage-relation.schema.json:239-268`), and R5 G3 is still open: nothing stops a proposer replacing an oracle guard with `true` (`R5-oracles.md:315-322`).

## 2. Five ranked design edits (recommendations)

### E1 — Transcript-binding rule for observations (U0 rule; U1 certificate; U2 slice)

**Rule sketch.** Each observation atom `i` read by any guard that is evaluated or statically included must carry exactly one binding locus:

```
Bind(i) ::= LedgerRead(slot)            -- ε = anchored@d
          | SigOpen(pk_ref, C_i)        -- ε = attested(Issuer,1,1)
```

- **Ledger read (anchored).** `LedgerRead` lowers to an `Impact` group whose ledger-VM read op names `(contract, cellPath)`. The results it returns (`value`, `observedAt`) are compared in-circuit, with `ConstrainEq`, to the registers the guard uses. The Impact's guard wire is either the constant 1 or the branch bit already forced to 0/1 by `ConstrainToBoolean` (`DESIGN-MIL2.md:265`).
- **Signed commitment (attested).** `SigOpen` verifies a signature in-circuit over `C_i = Poseidon(tag ‖ feedRef ‖ value ‖ observedAt ‖ seq)`. The issuer key `pk_ref` must be reachable from the intent digest.
- **Rejected:** a guard operand that comes only from `PrivateInput`, or only from a `PublicInput` outside any Impact. This is a compile-time rejection, code `OBS_UNBOUND`.

**Counterexample.** A lowering that reads `price` with `PublicInput` but never includes it in an Impact group. The circuit is satisfiable for any price, and the verifier accepts. This is exactly the "host Boolean" shape R5 G5 forbids (`R5-oracles.md:332-341`).

**Scope limit.** Anchored values are public by construction, because ledger reads are public. Private oracle values need `SigOpen` with `C_i` public and the opening private. The U0 profile should admit `attested` only once U1 measures a verifier that ZKIR can actually run. Ed25519 is unavailable (`DESIGN-MIL2.md:264`). A Jubjub Schnorr signature over a Poseidon challenge, using `EcMul`/`EcMulGenerator`/`TransientHash`, is a candidate I have not measured. `imported` stays at U4, unchanged.

### E2 — Replace the five-string observation record (U0 schema and formation rule)

```
ObsRecord = { obsId, feedRef:(domain, contract, cellPath) | (issuerRef, stream),
              class: anchored|attested, domain, clock,
              type: Price<B,Q,s> | Qty<A> | Instant(c),
              value, observedAt: Instant(clk), seq: u64,
              policyRef: digest, bind: Bind(i) }
```

**Formation rule:**

```
Π(feed f) = (T, ε@d, clk, D_max)   Γ ⊢ bind : Bind(i) matches ε
──────────────────────────────────────────────────────────────
Σ;Γ;phase;Π ⊢ observe i from f : Obs<T,ε,d> ! {i}
```

A value whose unit or orientation does not match `T` is rejected at lookup with `OBS_UNIT`, following the R5 kernel precedent (`R5-oracles.md:293-303`). A feed that does not match `Π` rejects with `OBS_FEED`.

**Counterexample.** Today a `{kind:"price", issuer:"x", …}` record is well-formed and carries no value. A stage can therefore list any observation it did not use, or use one it did not list.

**Placement.** This also records the `source/5` regression decision (G8): observation declarations return as a MIL/2 intent-layer construct, not through source/5.

### E3 — A Φ₀ observation-scaled comparison atom (U0; this changes the deferred Φ₁ boundary)

```
Γ ⊢ q : Qty<A> ! S₁        Γ ⊢ p : Obs<Price<B,A,s>,ε,d>.value ! {i}
Γ ⊢ r : Qty<B> ! S₂        width(p) ≤ 124,  k, m literals
─────────────────────────────────────────────────────────────────────
Γ ⊢ q ⊗ₒ p × k  ≥  r × m × 10^s : Prop ! S₁ ∪ S₂ ∪ {i}
```

- **It is rounding-free by construction.** It is an exact cross-multiplication with no division. The product needs at most 128+124 = 252 bits, which fits within `LessThan` without the limb rule (`DESIGN-MIL2.md:169-171`). Wider operands fall back to the declared two-limb gadget.
- **Why I am changing the Φ₁ boundary:**
  - Without this atom, the category's defining transaction is inexpressible until U4.
  - The atom is a *single* certified primitive, not general variable×variable products.
  - It stays out of the escrow authoring check's arithmetic. That check treats the atom as an opaque Boolean. This is sound for proving validity; when a proof needs the atom's arithmetic, the check returns `Reject(unsupported)` (semantics `:106`).
  - Where a division is truly needed (for example, "maximum borrowable"), the role fixes the rounding direction: floor against the party the value benefits. The author cannot choose it (R5 G6, `R5-oracles.md:343-353`).
- **Counterexample it closes.** Under Φ₀ alone, an author works around the gap by writing `q ≥ r × lit`, with a constant price hard-coded at signing time. That is a guard that ignores the oracle, and it type-checks.
- **Placement.** Atom in U0 Φ₀. Certificate at U1: valid-witness completeness plus rejection of an overflow witness.

### E4 — Freshness as a time bracket the ledger checks (U0 rule; U2 hostile control)

`stageTime` becomes a pair of public inputs `(tLo, tHi)` with `tLo < tHi`.

- **Ledger:** `tLo ≤ blockTime < tHi`.
- **Circuit:** `observedAt ≤ tLo ∧ tHi ≤ observedAt + D_max`. `D_max` comes from `Π`. The addition is checked, so overflow raises `Reject`.

Together these imply `observedAt ≤ blockTime < observedAt + D_max`, a half-open interval stated explicitly.

- **Distinct codes:** `OBS_FUTURE`, `OBS_STALE`, `OBS_MISSING`, `TIME_BRACKET`.
- **Counterexample:** with a single scalar `stageTime` supplied as a witness, the prover sets `stageTime = observedAt` and any stale quote passes.
- **Scope of `final(obs)`:** restrict it to `imported`. For an anchored read of the current state, inclusion itself serves as finality.

### E5 — Oracle clauses fixed by the signed policy, independent of the program's guard (U0 schema; U2 enforcement)

The template's evidence policy (semantics `:109`) carries `ObsPolicy{feedRef, class, D_max, orientation, ratio, roundingRole}`. `Stage` gains the conjunct

`PolicyGuards(I) = ⋀_{f∈Π} (bound(f) ∧ fresh(f) ∧ atom_f)`

The lowerer builds this conjunct **from the intent**, not from the program. It is bound through `intentDigest` as a public input, alongside the program's own `GuardsTrue`.

- **Counterexample (R5 G3):** a proposer's program replaces `fresh(price,5m) and collateralOK` with `true`. Today the stage still accepts, because the intent carries only caps and recipients.
- **Also required:** footprint derivation is done per operation over `PolicyGuards ∪ GuardsTrue` (R5 G4). A policy feed missing from the declared reads rejects with `FOOT_OBS`.

## 3. Core versus library

**Core (recommended):**

- `ObsRecord` and its formation rule
- `Bind` loci and the `OBS_UNBOUND` compile-time rejection
- source-set propagation, including through skipped branches (semantics `:79-83`)
- the `fresh` time bracket
- the `⊗ₒ` atom and role-directed rounding
- the independent `PolicyGuards`
- the distinct rejection codes
- the evidence classes: `anchored` (U0), `attested(·,1,1)` (gated on U1), `imported` (U4)

**Library:**

- deviation and circuit-breaker bounds, which are compositions of two `⊗ₒ` atoms
- pause and fallback policies. Fallback must be signed and must not weaken the evidence policy.
- liquidation recipes
- feed governance

**Later profile (not library, not yet):**

- bounded observation collections: quorum over distinct feeds, median, TWAP (`DESIGN-MIL2.md:360`)
- `attested(·,k,n)` with k>1, which needs its own cap on signature verifications, measured separately from the `k_of_n` width of 8

## 4. Smallest implementable slice and an evidence pair

**Slice (U2 scope, one domain, one signer):**

- A Midnight feed contract whose ledger cell holds `(priceNum: u124, observedAt, seq)`. It is written only under the feed's own authority.
- One borrower stage that:
  - reads the cell through an `Impact` (E1)
  - checks the time bracket with `D_max = 5m` (E4)
  - checks `150 A ⊗ₒ p × 100 ≥ 50 B × 150 × 10^s` (E3)
  - locks 150 A and opens a 50 B obligation with consent (debt kept separate from supply, L1)
  - binds `intentDigest`, `obsId` and `C_i`, plus the complete effects
- Anchored feeds only. There are no attested or imported feeds, no federated kernel, and no rollback.

**Positive control.** A feed write inside the window. The stage with the true cell value produces a native proof and is accepted, and readback of the effects matches the full lock and debt.

**Hostile control.** The same stage and bracket, but the prover supplies `p' = p×2`. The witness is produced honestly for `p'`, and the circuit is satisfiable, so the proof itself is valid. The ledger must reject because the result returned by the Impact read does not match the current cell. This satisfies the roadmap requirement that rejection not come from a malformed envelope alone (`ROADMAP.md:40`).

**Second hostile control.** The true `p`, but `observedAt = tLo − 6m`. This is rejected in-circuit with `OBS_STALE`, while a positive run with the same key is accepted.

## 5. Explicit disagreements

- **With `05-oracles.md:7,19`:** its core transaction and deviation rules implicitly assume multiplying a price by a quantity is in Φ₀. It is not (`DESIGN-MIL2.md:160`). Either E3 is adopted or the scenario moves to U4.
- **With `DESIGN-MIL2.md:144,187` and semantics `:85-98`:** a scalar `stageTime` cannot be enforced natively. Replace it with E4's bracket.
- **With `DESIGN-MIL2.md:241`:** `observation(feedId)` should be a state cell only for `anchored`. Attested observations are environment inputs with a separate footprint component. Otherwise footprint disjointness gives signed quotes a meaning they do not have.
- **With `DESIGN-MIL2.md:145` and the `k_of_n` cap of 8 at `:278`:** each signature verification is a nonlinear, chip-heavy node. It needs its own cap, measured at U1, and must not be counted as a `k_of_n` leaf.
- **With `DESIGN-MIL2.md:185` as worded:** "bound to an authenticated ledger read" should name the ZKIR mechanism: `Impact` plus `ConstrainEq`, with the guard constrained to 0/1. A private witness tag is not the only way this fails. A `PublicInput` outside an Impact fails the same way.

## 6. Residual assumptions

1. **Ledger re-checks reads (critical for E1).** At inclusion, the pinned Midnight ledger re-executes the Impact read ops against current state and rejects mismatched results. This needs confirming against the pinned ledger. The ZKIR spec only describes the prover-side self-check (`zkir-v3-spec.md:598`).
2. **The ledger can check a time bracket.** The pinned ledger exposes a check of `blockTime` against public bounds that ZKIR can reach. This is not in the ZKIR spec. If it is missing, E4 is blocked, not weakened.
3. **A signature scheme ZKIR can verify for `attested`.** It exists within cost, to be measured at U1. Until then `attested` stays out of the admitted profile.
4. **Hash agreement.** Poseidon and `transient_commit` agree as part of the trust base (`zkir-v3-spec.md:903-906`).
5. **Feed authority.** The feed contract's writer authority is itself a named trust premise. As in `05-oracles.md:22`, native proofs bind the statement about the price, not market truth.
6. **Width bound.** `LessThan` supports 252-bit operands under the pinned field (`DESIGN-MIL2.md:169`). Otherwise E3 needs the limb gadget.