Status is recorded: `SP01.6 loan-swap-subset`, operational history unresolved, no pending transactions. It does not block this read-only review. I've read the required sources and the IBC captures, and am writing the recommendation below.

# Bridges and cross-domain settlement: paired claim accounting (recommendation)

**Scope.** This is a read-only design recommendation on a specified-only proposal. Nothing below is implemented, proved or accepted by a ledger. Everything labelled *Recommendation* is my proposal, not a current Moriarty rule.

## 1. Verdict

The local legs are fit. The paired transfer is not specified.

MIL/2 gets the per-domain pieces right:
- one executing domain per stage (`DESIGN-MIL2.md:46`)
- nominal asset identity with `repr` (`:53-61`)
- domain-matched local effects, with a foreign credit only as imported evidence (`:254`)
- per-`(domain, asset)` conservation (`:256`, E1 at `MIL2-PROPOSED-SEMANTICS.tex:171-178`)

None of these connects a source debit to a destination credit. E1 on D holds for a mint that has no lock on S behind it. The only precondition is the signed `issue` right (`.tex:178`; `DESIGN-MIL2.md:193`). The category review reaches the same result (`07-bridges.md:14,17`).

Three things are missing from §§3–10:
- a transfer identity
- a transfer-form sort
- a rule that consumes each pair exactly once

The `replay(id)` cell (`DESIGN-MIL2.md:242`) has no key discipline. An observation's identity is its own (`.tex:67`), not the identity of the transfer it reports. So two distinct attestations of one lock would pass as two distinct replay IDs.

**Verdict:** the category needs five structural additions before a bridge can be claimed as expressible. None of them needs Φ₁, global rollback or a global ledger.

## 2. Five ranked design edits (Recommendation)

### E1 (highest). Unique transfer identity and a one-shot destination receipt

**Rule.** Add the sort `XferId` and three cells:

```
XferId := Poseidon(tag_xfer ‖ link ‖ srcDomain ‖ srcInstance ‖ seq_S ‖ intentDigest)
Cell   += xferCommit(S, id) | xferReceipt(D, link, id) | xferTerminal(S, id)
```

Here `link` is the signed link policy described in E3.

- **Source stage.** Writes `xferCommit(S, id) := (form, asset_S, n_S, recipient_D, asset_D, deadline_D)`. `seq_S` comes from a per-link counter at S, incremented in the same stage.
- **Destination stage.**
  ```
  pre(xferReceipt(D, link, id)) = ⊥   ∧   EvidenceValid(obs ⊨ xferCommit(S, id))
  ─────────────────────────────────────────────────────────────────────
  post(xferReceipt(D, link, id)) ∈ {Success, TimeoutSentinel}   (write-once)
  ```
- **Source terminal.** `xferTerminal(S, id)` is a tombstone with three possible values: `acked`, `refunded` or `trustedRecovered`. `refunded` needs evidence that D wrote `TimeoutSentinel` for `id`, or that `id` has no receipt at a D height past `deadline_D`. It never follows from S-clock expiry alone. The design already says this (`.tex:200-202`; `DESIGN-MIL2.md:219`).

**Counterexample.** A relay submits two threshold attestations with different observation IDs, both over the same lock. Today both satisfy `EvidenceValid` and each consumes a different `replay(obsId)`, so D mints twice. With the rule, the second attempt finds `xferReceipt(D, link, id) ≠ ⊥` and rejects.

A second case: the S stage refunds on its own clock while D, which is late, still mints. That leaves a double spend across domains. Keying the source terminal on a destination-authenticated `TimeoutSentinel` closes it.

**External practice.** IBC stores a commitment per sequence (`source-text/ibc-ics004.md:413-417`). It writes a receipt so that a packet cannot be received twice or timed out after it was received (`ibc-v2-packet-handler.md:229, 324-325`). It also evaluates the timeout on the receiving chain's clock (`:261`). This is comparative practice only, not a Moriarty rule.

**Placement.**
- **U0:** sort, cells and stage public-input fields.
- **U3:** local enforcement, using same-chain anchored reads.
- **U4:** imported verification.

**Why U0 changes.** The cell vocabulary and the public-input schema are U0 items (`DESIGN-MIL2.md:345`). Adding them after the digest is hash-bound would force a version migration.

### E2. Transfer form as a type index with a per-form effect obligation

**Rule.** Replace the untyped "declared link" (`DESIGN-MIL2.md:61`) with a closed enum:

```
Form ::= lockMint | burnMint | custodyRelease | burnRelease     -- burnRelease = return leg of lockMint
```

| Form | Required S effect (E1 at S) | Required D effect (E1 at D) |
|---|---|---|
| lockMint | owner −n, custody(link) +n, Δsupply_S = 0 | Δsupply_D(wA) = +m, recipient +m |
| burnMint | owner −n, Δsupply_S = −n | Δsupply_D = +m |
| custodyRelease | owner −n, custody +n | custody(link) −m, recipient +m, Δsupply_D = 0 |
| burnRelease (D→S) | Δsupply(wA) = −n at the wrapped side | custody(link) −m at the canonical side |

The form is determined by the `repr` of both endpoint assets under the link, not chosen by the signer. For example, `lockMint` requires `repr(asset_D) = wrapped(asset_S, link)`.

**Counterexample.** A signer labels as `lockMint` a transfer whose S "lock" moves funds to an account the signer controls. E1 at S still holds (−n, +n), but the custody cell is not `custody(link)`. Unbacked wrapped supply then appears at D.

A second case: a return leg is declared as `burnMint` into the canonical asset at S. That mints canonical supply, which only the canonical issuer may do.

**Placement.** Enum and repr coupling at U0. Per-form effect checking at U2 for the source leg alone, and at U3 for the pair.

### E3. Link policy with local backing counters and a mint cap. No global ledger.

**Rule.** Add a signed, versioned `Link ℓ` object:

```
{ asset_S, asset_D, form, verifierPolicy, finalityClass, cap_ℓ, conversion }
```

It has one counter cell per side:
- `locked_S(ℓ)` counts outbound commits minus released `burnRelease` amounts.
- `outstanding_D(ℓ)` counts minted amounts minus amounts burned for return.

Local invariants, each checkable within one domain:
- **(B-D)** `outstanding_D(ℓ) ≤ cap_ℓ`. Each increment consumes exactly one `xferReceipt`, and the increment equals that commit's converted `m`.
- **(B-S)** Each S release consumes exactly one D burn commitment, via E1 in the reverse direction.

The cross-domain backing theorem is stated as a conditional:

```
Injective(receipt consumption) ∧ Sound(verifierPolicy, finalityClass)
  ⇒ outstanding_D(ℓ) ≤ convert(locked_S(ℓ))
```

The injectivity half is **[obligation, local]**. The verifier half is a named premise.

`issue` for `asset_D` is bound to `ℓ` and can be exercised only by consuming a receipt. It is never a free signer right.

**Counterexample.** A holder of `issue(D, wA)` mints with no receipt. E1 holds, and the mint is unbacked. Under E3, `issue(D, wA)` with no consumed receipt is ill-typed.

**Verifier compromise.** The loss is bounded by `cap_ℓ`, a measurable exposure. That gives the category review's "exposure versus slashable security" (`07-bridges.md:22`) a concrete carrier.

**Debt separation.** An in-flight transfer is a `Claim` record held at `xferCommit`. It is neither supply nor a debtor `Obligation`, because no debtor consented (`DESIGN-MIL2.md:94-100`). Supply changes only at mint or burn.

**Placement.** Sort and cells at U0. B-D and B-S at U3 (loopback). Verifier premise at U4. Haircut and cap values belong to libraries.

### E4. The claim is a linear resource, so fast fill substitutes the claimant

**Rule.** `xferCommit(id).claimant` starts as `recipient_D`. A fast fill is a D-side stage with these effects:
- filler −m′
- recipient +m′
- write `claimant := filler`

It needs the recipient's signed consent, since the recipient is selling the claim. Final settlement of `id` pays whoever is `claimant` in `pre`, and consumes the receipt once. The footprint includes `receipt(id)` and `xferReceipt(D, link, id)`, so a fork partitions the claim (`DESIGN-MIL2.md:250`).

**Counterexample.** Today the filler pays 99 wA from its own balance, and later the standard mint also pays the recipient 100 wA. Both settlements consume different cells: the filler's balance and the replay entry of an observation. The recipient ends up paid twice and the filler's reimbursement is unbacked.

**Placement.**
- Claimant field: U0.
- Substitution transition: U3, where partial progress and persistent duty live (`ROADMAP.md:24`).
- Bond, slashing and dispute economics: U5 library.

### E5. Cross-representation conversion stays in Φ₀, with rounding against the minted side

**Rule.** Let `k = dS − dD ≥ 0`, taken from checked decimals. The admission guard is:

```
m × 10^k ≤ n   ∧   n < (m + 1) × 10^k        -- literal coefficients: Φ₀ (DESIGN-MIL2.md:136,140,160)
dust = n − m × 10^k  → custody(link), class retained-in-custody, counted in locked_S
```

The rounding direction is floor toward the minted side, so dust stays backed. If `k < 0`, the constraint is `m = n × 10^{−k}` exactly. The `u128` × literal product follows the limb rule when it exceeds the field (`:171`).

**Counterexample.** Rounding up mints 1 unit that has no backing on every transfer where `n mod 10^k ≠ 0`. Repeated transfers grow `outstanding_D − locked_S` without bound, while every single stage still satisfies E1.

**Placement.** U0 numeric profile, which already owns per-primitive rounding direction (`ROADMAP.md:21`). **No Φ₁ change.**

## 3. Core versus library

**Core (language plus acceptance relation):**
- `XferId`, the three xfer cells, and write-once receipts
- the `Form` enum and its repr coupling
- the `Link` sort and its counters
- `issue`-by-receipt only
- the `Claim` sort with a claimant field
- the conversion and rounding rule
- source-terminal exclusivity keyed to destination evidence

These must be in core because otherwise the footprint and linearity checks cannot see double consumption (`DESIGN-MIL2.md:245`).

**Library:**
- verifier adapters (light client, committee, optimistic)
- finality-class tables
- cap and haircut values
- fast-fill pricing, bonds and slashing
- rate limiters
- eligibility propagation policies (`07-bridges.md:21`)

## 4. Smallest implementable slice and evidence pair

**Slice (Recommendation, U3 profile).** A Midnight-to-Midnight loopback between two contract instances (S, D) on Preview.

- Evidence of the S commit is an **anchored** authenticated ledger read on the same chain. This avoids U4 imported verification entirely, and the proposed statement already distinguishes instances (`.tex:240`).
- Form: `lockMint`. Asset A has 8 decimals and wA has 6, so `k = 2`.
- Stages: S-lock, then D-mint. No fast fill and no refund in the first slice.

**Positive.**
1. Owner locks `n = 100_000_050` A. The effects are owner −n and custody(ℓ) +n. The stage writes `xferCommit(S, id)` and sets `locked_S` += n.
2. Destination mints `m = 1_000_000` wA to the recipient and writes `xferReceipt(D, ℓ, id) = Success`. It sets `outstanding_D` += m. The 50 units of dust stay retained in custody.
3. Readback checks every cell, supply and both counters.

**Hostile, one field changed, valid envelope.** Submit a second D stage that is otherwise identical: fresh nonce and replay ID, a valid signature, and a valid anchored read of the same `xferCommit(S, id)`. The rejection must come **natively** from the constraint `pre(xferReceipt(D, ℓ, id)) = ⊥`, not from a malformed envelope (`ROADMAP.md:40`). The pass condition is that `outstanding_D` stays unchanged on the ledger.

**Secondary hostile case.** `m = 1_000_001` with everything else valid must be rejected by E5.

These are future acceptance cases. None has been run.

## 5. Explicit disagreements

1. **With R7 L8 ("global claim ledger", `R7-bridges.md` via `07-bridges.md:17`).** I reject a global ledger. It contradicts one-domain stages and the no-global-rollback rule. Per-link local counters plus a named verifier premise give the same safety claim, conditional on the verifier, with local enforcement.
2. **With the category review's priority 1 (`07-bridges.md:33`).** It puts the whole paired relation in U0. Only the identity, sorts, cells and public-input fields need U0. Enforcement belongs to U3 (anchored loopback) and U4 (imported). Freezing verifier semantics at U0 would pull U4 forward.
3. **With `DESIGN-MIL2.md:193` and `.tex:178`.** "Supply change requires the signed `issue` right" is insufficient for wrapped assets, because a signer-held issue right is exactly the unbacked-mint path. Wrapped `issue` must be receipt-gated.
4. **With `DESIGN-MIL2.md:61`.** A `repr` "declared link" is metadata. It has to become a signed `Link` policy that carries form, cap and conversion, or `repr` gives no redemption or backing meaning (`07-bridges.md:15`).
5. **With keying replay on observation identity.** Replay for bridge messages must be keyed by `XferId`. Observation identity (`.tex:67`) is still right for provenance, but it is the wrong key for uniqueness.

## 6. Residual assumptions

- Imported verifier soundness and the source finality class remain premises. MIL/2 cannot prove them (`DESIGN-MIL2.md:187`; `07-bridges.md:22`). The cap bounds the loss; it does not prevent it.
- Midnight contract state can hold write-once receipt cells and per-link counters, and a same-chain instance can authenticate another instance's cell as an anchored read. This is unverified against the pinned ledger.
- Poseidon collision resistance for `XferId`. The per-link `seq_S` counter is serialised on S.
- Asset decimals metadata is authentic, since the conversion rule trusts it.
- The caps (8 stages, 16 effects, 32 footprint cells; `DESIGN-MIL2.md:274-283`) can hold lock → fast fill → mint → return → release. This is unmeasured.
- Reorg retraction after a D mint is not handled beyond the cap. It needs a finality-class rule for a later profile.
- ICS-020's escrow/voucher split and its refund-on-error/timeout behaviour (https://github.com/cosmos/ibc/blob/main/spec/app/ics-020-fungible-token-transfer/README.md) inform E2 and E4. I recalled that spec; I did not fetch or capture it in this review. Only ICS-004 and the IBC v2 packet handler were checked, against the captured text.