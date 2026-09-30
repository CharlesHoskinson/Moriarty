# Bridges and cross-domain settlement: recommendation (Opus 5.5, compiler/native-feasibility lens)

*Specified-only design review. I did not compile, prove, test or send anything. Guarded status: `SP01.6 loan-swap-subset`, operational history unresolved, no pending transactions. This review does not depend on that dispatch.*

## 1. Verdict

**The category design is not fit to freeze. The repairs are local and can be implemented.** MIL/2 already has the right basic shape: one executing domain per stage (`DESIGN-MIL2.md:43,46`), domain-qualified effects with foreign credit carried only as imported evidence (`:254`), nominal assets with `repr` (`:53-61`), and a `replay(id)` cell (`:242`). Three things are missing:

- **A transfer relation.** No typed object pairs a source lock with a destination issue. E1 (`MIL2-PROPOSED-SEMANTICS.tex:171-177`) holds just as well for an unbacked mint.
- **A natively verifiable verifier mode.** `imported(Policy)` is only a named premise until U4 (`DESIGN-MIL2.md:187,350`). The design also correctly says there is no in-circuit Ed25519 or SHA-512 (`:264`). As written, a destination stage would need a host to check the foreign attestation, and a host-computed Boolean is forbidden (`MIL2-PROPOSED-SEMANTICS.tex:130`).
- **A nullifier definition for imported messages.**

From the compiler's point of view, the important fact is that ZKIR v3 has `ec_mul` and Poseidon (`DESIGN-MIL2.md:264`). **Recommendation:** a threshold of Schnorr signatures over a Poseidon digest on the embedded curve can be verified inside one domain-local stage without recursion. That is enough for a bounded bridge-in stage.

## 2. Five ranked design edits (recommendations)

### Edit 1: Transfer identity, a claim cell and a one-sided backing equation (most important)

**Rule sketch.** Add these carriers:

```
Cell += bridgeClaim(route)      route = (srcDomain, emitter, srcAsset, dstDomain, dstAsset, form)
form ::= lock-mint | burn-mint | custodial-release
XferId = Poseidon(tag_xfer ‖ srcDomain ‖ emitter ‖ seq ‖ dstDomain ‖ dstAsset
                  ‖ recipient ‖ amountDst ‖ timeout(Instant(dstClock)))
```

The `bridge_in` transition on D (for lock-mint or burn-mint):

```
pre(replay(n)) = unset,  n = ImportNullifier(msg)            (Edit 3)
EvidenceVerified(msg, policy)                                (Edit 2)
effects = { issue(D, dstAsset, recipient, amountDst) }
post(bridgeClaim(route)) = pre(bridgeClaim(route)) + amountDst ≤ policy.cap
post(replay(n)) = set
```

`bridge_out`, the redemption burn on D, decrements the claim and emits a message. The invariant to prove is **one-sided and conditional**:

`supply(D, dstAsset) ≤ bridgeClaim(route)`, with `bridgeClaim ≤ Σ attested source locks − Σ attested releases` **under the attester-honesty premise**.

Do not claim `debit(S) = credit(D)`, because MIL/2 cannot prove the foreign chain honest (`07-bridges.md:17,22`).

**Counterexample it closes.** Suppose a stage holding an `issue(D,W)` right mints 99 W to a recipient with no source lock. Today E1 balances (Δbalance = Δsupply = 99), so the stage is acceptable. Under the edit it is rejected: there is no verified message and no claim increment.

**Placement.** U0 adds `bridgeClaim` to the cell vocabulary and `XferId` to the stage public-input schema. U0 already owns both (`DESIGN-MIL2.md:345`). Doing this at U0 changes the U0 boundary on purpose. The reason is the same one MIL/2 gives for reserving multi-signer arity: a public-input field added after the digest is hash-bound costs a version migration (`:336`). Enforcement belongs to U3.

### Edit 2: Enumerated verifier mode, limited to modes the profile can verify natively

**Rule sketch.**

```
BridgePolicy = { route, mode, keysetCommit, epoch, finalityClass, maxAge: Duration(dstClock), cap }
mode ::= attestedSchnorr(k, n)      -- U3: k Poseidon-Schnorr verifications via ec_mul
       | ledgerSig(k, n)            -- only if U1 shows the ledger checks it (cf. :263)
       | lightClient | zkProof      -- reserved; U4 (recursion)
       | optimistic(challenge)      -- library over escrow + challenged(e)
```

- The policy digest is part of the signed intent.
- `keysetCommit` is an **authenticated ledger read** of `policy(id)`, never a witness. This follows the anchoring rule at `DESIGN-MIL2.md:185`.
- Each signature-validity bit passes `constrain_to_boolean` (`:265`), and `k_of_n` is counted in-circuit.
- `n ≤ 8` follows the `k_of_n` cap (`:278`).
- An `imported(Policy)` with an unadmitted mode is rejected at authoring. It is not labelled and passed through.

**Counterexample it closes.** With a secp256k1 or Ed25519 guardian set, the stage cannot verify the signatures in-circuit. The only way to "accept" is for the relayer to check them and supply `valid=1`. That is exactly the forbidden host Boolean.

**Placement.** U0 freezes the enum. U1 measures the cost of k embedded-curve verifications (see the R3 k17 exhaustion note, `AGENTS.md:79`). U3 admits `attestedSchnorr`, and U4 keeps `lightClient` and `zkProof`.

### Edit 3: A nullifier for imported messages that does not depend on the policy

**Rule sketch.**

`ImportNullifier = Poseidon(tag_import ‖ srcDomain ‖ emitter ‖ seq ‖ dstDomain)`

- It is computed **in-circuit from the attested message fields**.
- It is published as an **unguarded** public input.
- The ledger writes `replay(n)` once.
- The epoch, keyset and policy id are left out on purpose.

**Counterexamples it closes.**

1. If the nullifier includes `epoch`, the attesters can rotate keys and re-sign the old message. A new nullifier then permits a second mint.
2. ZKIR `Impact` under a false guard publishes `select(guard, x, 0)`, which is zeros (`zkir-v3-spec.md:589-601`). If the nullifier is published under a branch guard, a skipped branch publishes nullifier 0. Either replay checks collide on 0, or the check is silently skipped. The rule is therefore: **bridge identity inputs are never guarded, and the ledger rejects n = 0.**

IBC v2 uses the same external pattern: a receipt keyed by client and sequence "prevents replay attacks" (`ibc-v2-packet-handler.md:229,325`; https://github.com/cosmos/ibc/blob/main/spec/IBC_V2/core/ics-004-packet-semantics/PACKET_HANDLER.md). That is IBC practice, not a Moriarty rule.

**Placement.** U0 fixes the definition and domain tag. U2/U3 enforce it.

### Edit 4: Refund requires authenticated nonreceipt judged on the destination clock

**Rule sketch.**

- Source escrow `refund_when` needs `nonreceivedProved(xfer)`. This is an attested statement from the same policy's attesters saying `replay(n)` is unset on D at a D-clock time `t ≥ xfer.timeout`.
- Symmetrically, `bridge_in` requires `stageTime(D) < xfer.timeout` on the D clock.
- Exclusivity then holds under honest attesters, with no global rollback.
- The escrow `priority` field (`DESIGN-MIL2.md:206`) settles same-head races on S only.

**Counterexample it closes.** The source refunds at a *source-clock* deadline while a relayer later delivers the attestation to D. The result is a refund on S and a mint on D. IBC evaluates the timeout against "the **receiving chain** clock" and deletes the commitment only on a verified non-existence proof (`ibc-v2-packet-handler.md:261,378-396`). Again, that is external practice.

**Placement.** U3, which owns the late race (`DESIGN-MIL2.md:349`). The generic "explicit trusted recovery condition" should be narrowed to this one attested statement.

### Edit 5: Decimal conversion with role-directed rounding and a named dust claimant

**Rule sketch.**

```
amountDst = ⌊amountSrc / 10^(sd−dd)⌋          (sd ≥ dd)
dust      = amountSrc − amountDst · 10^(sd−dd)  → retained in source escrow, residual to owner
```

This is a Φ₀ literal coefficient, so the limb rule is not needed. Rounding is **down for the recipient and the protocol mint**. The claim cell carries destination units only.

**Counterexample it closes.** Bridging an 18-decimal source to 6 decimals and rounding half-up mints up to 0.5 destination unit per transfer without backing. Splitting a transfer into many small ones amplifies that.

**Placement.** U0 numeric profile, which already requires per-primitive rounding direction (`ROADMAP.md:21`).

## 3. Core versus library

**Core:**
- `XferId`, the transfer `form` and the `bridgeClaim` cell
- `ImportNullifier` and the no-guard rule for its public input
- the verifier-mode enum with its admission-by-profile rule
- the states `unknown / received / nonreceivedProved / final` (`MIL2-PROPOSED-SEMANTICS.tex:202`)
- destination-clock timeout
- the conversion and rounding rule
- derived bridge footprints: `supply`, `balance`, `replay`, `bridgeClaim` and `policy`. The showcase's missing-cell defect (`:218-227`) must not repeat here.

**Library** (built from Φ₀ guards, escrow and liabilities):
- lock-mint, burn-mint and custodial-release templates
- fast-fill reimbursement, as a liability plus a bonded escrow plus `challenged(e)`
- haircuts and exposure caps, as guards over `pre(bridgeClaim)`
- rate limits per epoch
- wrapped-eligibility checks, where the policy's issuer approval is core but jurisdiction rules are library

**Optional federated kernel:** relays bytes and runs attesters. Its signatures are one `attestedSchnorr` source and never an acceptance decision.

## 4. Smallest implementable slice and evidence pair

**Slice (recommendation; nothing here is implemented):** one destination-only `bridge_in` stage on Midnight Preview.

- A `policy(id)` cell commits to a test attester keyset with k=2, n=3.
- The stage verifies two signatures in-circuit over `Poseidon(msg)`, derives `ImportNullifier`, and checks the D-clock timeout.
- It then issues 99 W to the recipient under `issue(D,W)`, writes `replay(n)` and raises `bridgeClaim` by 99.
- Readback covers `supply(D,W)`, the recipient balance, `replay(n)` and `bridgeClaim`.

No source chain is needed: the source leg is the named trust premise.

**Positive control.** A fresh `seq` with a valid 2-of-3 attestation and signed intent is accepted with exactly those four readback deltas.

**Hostile control.** The same attestation bytes are resubmitted with a **fresh, valid proof** and a correct envelope. Expected outcome: rejected by the ledger because `replay(n)` is already set, with no supply change.

This pair isolates the semantic check, which ROADMAP asks for: a rejection caused only by a malformed envelope proves nothing (`ROADMAP.md:40`). A secondary hostile case is a valid attestation whose `dstDomain` names another domain. It must fail inside the circuit, because the nullifier and `XferId` preimage bind `dstDomain`.

## 5. Explicit disagreements

1. **Milestone placement.** `DESIGN-MIL2.md:350` places *all* imported-evidence verification in U4. I disagree: embedded-curve threshold attestation needs no recursion and should be U3-admissible once U1 has measured it. Light-client and zk-proof import stay in U4.
2. **Trust taxonomy.** R7 suggests a six-value `trust_domain` taxonomy. I would admit only modes the pinned profile verifies natively and **reject** the rest, rather than labelling them.
3. **Recovery.** I would not allow a generic "exact signed trusted-recovery condition" (`07-bridges.md:7`). Recovery must be the policy's own attested nonreceipt on the destination clock. An owner-selected oracle reopens the double-spend.
4. **Backing equation.** `07-bridges.md:17` frames the goal as proving `debit(S)=credit(D)`. That equality cannot be proved. The claim should be the one-sided inequality under a named premise.
5. **No seventh judgment.** The paired obligation fits into the existing `effect` key (claim, supply) and `history` key (nullifier, predecessor) (`MIL2-PROPOSED-SEMANTICS.tex:139-154`).

## 6. Residual assumptions

- The embedded-curve identity in `ec_mul` must be pinned. The doc says "Curve25519" (`DESIGN-MIL2.md:264`); I have not checked that against the actual chip. The cost of k ≤ 8 Schnorr verifications within the admitted circuit size is also unmeasured.
- A Midnight contract stage can read `policy(id)` and its own D clock as authenticated in-circuit values.
- At least k attesters are honest and wait for the declared source `finalityClass`. Reorg retraction after D credits is still open: the claim only bounds the loss to `cap`.
- The source emitter locks or burns correctly. The slice does not verify this; it is the named premise.
- The ledger enforces the one-shot `replay` write atomically with the stage's effects in the chosen phase layout.

None of these edits is implemented, proved or ledger-accepted.