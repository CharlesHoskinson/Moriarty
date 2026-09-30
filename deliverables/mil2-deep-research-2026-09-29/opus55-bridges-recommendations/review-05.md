# Recommendation: bridges and cross-domain settlement under MIL/2 (adversarial controls and release plan)

**Status.** This is a specified-only design recommendation. It claims no implementation, proof or ledger acceptance. I did not read any other reviewer's output. I ran the guarded `status`. It reported `SP01.6 loan-swap-subset`, unresolved operational history and no pending transactions. That block applies only to dispatch, so it does not affect this review.

## 1. Verdict

**Recommendation: the category is not yet fit for a bridge claim, but it can be repaired without changing the architecture.** MIL/2 has three correct foundations:
- effects are domain-qualified (`DESIGN-MIL2.md:254`);
- evidence is typed by class and domain (`:177–185`);
- there is no global rollback (`MIL2-PROPOSED-SEMANTICS.tex:59`).

Its escrow race rule is unsafe across domains. `priority` "decides" the late race (`DESIGN-MIL2.md:206,216`), and ESC selects one branch "at the same authenticated head" (`.tex:192`). A bridge has **two** heads, S and D, so no single head exists. A source-side `priority refund` taken after `after(deadline)` can race a destination issue that is still valid. The result is a double spend. The semantics admits that a deadline is not proof of nonexecution (`.tex:200,202`), but nothing forbids a refund guard that uses only the local clock. The category report records the other gaps: there is no paired transfer identity (`07-bridges.md:14,17`), no verifier or finality typing (`:16`), no reorg rule (`:22`) and no filler loss allocation (`:20`).

## 2. Five ranked design edits (recommendations)

### E1. A paired claim identity with a one-shot receipt on both sides. Target: double mint.

**Rule sketch.** Add the cell `claim(x)` and extend `replay` so that `receipt(x@D)` has the key
`x = H(srcDomain, srcInstance, dstDomain, dstInstance, form, seq)`.
The key excludes the verifier epoch and the relayer.

- **Source stage.** It writes `claim(x) := {form ∈ {lock-mint, burn-mint, custodial-release}, asset_S, q_S, asset_D, recipient, deadline : Instant(clk_D), policyDigest, status=pending}`.
- **Destination issue stage.** It requires `Obs<Claim, imported(P)@S>` whose identity is `x`, and `¬receipt(x@D)`. It writes `receipt(x@D)` and applies `Δsupply_D(asset_D) = q_D`, where `q_D·10^k ≤ q_S` for a declared decimal shift `k`.
- **Rounding.** The recipient's amount rounds down and the backing side is favoured. The remainder goes to a named `residual`.
- **Width.** The intermediate needs about 168 bits (u128 + 40 bits). It must use a declared wider width, under the limb rule (`DESIGN-MIL2.md:171`), never a silent `mul`.
- This stays in Φ₀ because it multiplies only by a literal (`:160`). **Φ₁ is not changed.**

**Counterexample (current MIL/2).** Suppose the replay key is the observation identity. Two relayers wrap the same lock in attestations signed in different epochs. D consumes both `replay(obs₁)` and `replay(obs₂)`, so it issues 2×99 against a single lock of 100. E1 on D (`.tex:171–176`) holds both times.

**Placement.**
- U0: the claim cell, key layout and public-input slot.
- U3: enforcement.
- U4: imported verification.

**Why U0 changes.** A public-input layout that is bound into the digest cannot be retrofitted. This is the same argument as Decision 4 (`DESIGN-MIL2.md:336`).

### E2. Bind every imported observation to a domain pair and an instance. Target: wrong-domain proof.

**Rule sketch.** Change `imported(Policy)` to `imported(P, src=(chainId, domainId, instance, epoch), dst=(chainId, domainId, instance))`.

Formation rule: a destination stage admits an observation only if `obs.dst = (self.chain, self.domain, self.instance)` **and** `P.digest = claim.policyDigest`. The chain identity includes a fork or genesis hash, not only a chain number. An observation with any unbound field is rejected with `Reject(EVIDENCE_DOMAIN)`, following the totality rule (`DESIGN-MIL2.md:124`).

**Counterexamples.**
- A lock that targeted D₂ is replayed to D.
- An attestation from a testnet or post-fork chain that shares a `chainId` is accepted.
- An attestation for a sibling contract instance on D is accepted.

The current index carries only `Policy@d` (`:177`), so the destination side is not typed.

**Placement.** U0 fixes the shape of the type index. U4 verifies it.

**External practice (not a Moriarty rule).** IBC v2 rejects a packet whose `destClient` does not match the counterparty's client ID ([PACKET_HANDLER.md](https://github.com/cosmos/ibc/blob/main/spec/IBC_V2/core/ics-004-packet-semantics/PACKET_HANDLER.md); captured copy at `source-text/ibc-v2-packet-handler.md:372,400`).

### E3. The destination enforces the deadline, and a refund needs proof of nonreceipt. Target: false timeout and late receipt.

**Rule sketch.**
1. D rejects `receive(x)` when `now(clk_D) ≥ claim.deadline`. It may instead write a `timeout(x@D)` tombstone.
2. S moves `claim(x): pending → refundable` only on `nonreceivedProved(x)`. That means one of:
   - an authenticated non-membership proof of `receipt(x@D)` at a D head whose D time is at or after the deadline; or
   - a D-written `timeout(x@D)`.
3. S moves `pending → released` only on membership evidence of `receipt(x@D)`.
4. For paired claims, the authoring check **rejects** `priority` values of `refund` and `signed_order`, and rejects any `refund_when` whose source set lacks D evidence. The destination's state decides the priority. A signer cannot choose it.
5. Because the deadline is typed `Instant(clk_D)`, an S-clock comparison is a formation error (`DESIGN-MIL2.md:115–117`).

**Counterexample.** Take the MIL/2 showcase pattern (`refund_when after(validity.end)`, `:310`) applied to a bridge. The relay stalls. S refunds at S-time T+1. Because the clocks drift, the relay then delivers at D-time T−1, and D issues. Both terminal transitions pass their local tombstones (`.tex:192–198`).

**Placement.** U0 adds the status enum `{pending, received, nonreceivedProved, released, refunded, disputed}` and the D-clock deadline. U1 adds the authoring rejection. U3 adds the race demonstration.

**External practice (not a Moriarty rule).** IBC evaluates the timeout on the receiving chain's clock (`PACKET_HANDLER.md`, `source-text/…:261,324`). Its timeout requires a non-membership proof of the receipt (`:402`) at a proof timestamp at or after the timeout (ICS-004, [README](https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md); `ibc-ics004.md:1376–1378`). The receipt exists specifically so that a received packet cannot also be timed out (`packet-handler.md:229`).

### E4. Typed finality class, a `disputed` state, and a named response to partition and reorg

**Rule sketch.**
- `P` declares `finality ∈ {final, withdrawable(window), optimistic(challengeWindow)}`.
- D may mint a *canonical-grade* representation only on `final`.
- On `withdrawable` or `optimistic` evidence, D issues a `repr = contingent(x)` asset. Its encumbrance cannot be transferred until an S finality observation upgrades it. It cannot be used as collateral, which is R7 L25/X12.
- Either side may enter `disputed` on authenticated conflicting evidence, such as a verifier equivocation or an S reorg that removes the lock. In `disputed`, no terminal transition is allowed. Only the loss rule in E5 may close the claim.
- **Partition:** if neither side has evidence, the status stays `pending`/`unknown` indefinitely. Silence never produces a terminal state.

**Counterexample.**
- A k-of-n verifier signs both "lock at S height h" and a reorged "no lock". D mints and S refunds, and each is locally valid.
- Separately, R7 identifies a wrapped claim that becomes collateral before finality as class F (X12).

**Placement.** U0 reserves the finality enum and the `disputed` state. This extends the four-state escrow set (`DESIGN-MIL2.md:203`), because a non-terminal state added after the digest freeze would break canonical encoding. U4 handles verification semantics and U5 handles the adapter.

### E5. Loss allocation is recorded as debt, never as supply. Target: loss allocation and fast fill.

**Rule sketch.**
- A fast fill creates `Obligation{debtor: claim(x) custody, creditor: filler, principal: q_fill, rank, consent: filler + owner}`, using `Obligation` (`DESIGN-MIL2.md:94–95`).
- The filler posts an `Encumbrance{against: {x}, enforceable_by: challenge policy}`.
- On `disputed`, the signed waterfall closes the claim in this order: filler bond, then bridge reserve, then a pro-rata haircut on `contingent(x)` holders, then a named residual claimant (R7 L29). Each step is an L1 obligation transition (`.tex:180–185`).
- A deficit **never** appears as a negative supply delta, which the product contract requires (`MORIARTY-PRODUCT-CONTRACT.md:43–45`).
- **Exposure cap:** the policy requires `Σ pending q over x ≤ slashable bond` as a lock-sum instance (K1, `.tex:229–236`). This is R7 X13.

**Counterexample.** The filler advances 99 on D. S later refunds after a reorg. If there is no obligation cell, the 99 either disappears, which erases debt, or is re-minted, which inflates supply.

**Placement.** U3 covers the obligation and encumbrance mechanics. U5/U6 cover the fast-fill library and bond economics.

## 3. Core versus library boundary (recommendation)

**Core (bound into the digest and enforced natively):**
- the `claim(x)` cell, key and one-shot receipt;
- the domain-pair and instance evidence index;
- clock-indexed D deadlines;
- the claim status enum including `disputed`;
- the `form` tag as a required enum field;
- the finality-class enum;
- the authoring rejection of source-clock refunds on paired claims;
- the rule that a deficit is an obligation and never supply.

**Library (versioned templates over the core):**
- the semantics of each of lock-mint, burn-mint and custodial-release;
- verifier adapters (light client, threshold, optimistic);
- loss waterfalls, haircuts, rate limits and exposure caps;
- fast-fill contracts;
- eligibility propagation (L26/X19).

The optional federated kernel stays outside acceptance (`.tex:59`). It may relay evidence. It never supplies an acceptance Boolean.

## 4. Smallest implementable slice (U3) and evidence pair

**Slice.** Two Midnight Preview contract instances, `S` and `D`, run a lock-mint paired claim with **anchored** evidence, which means ledger-authenticated reads. This removes the dependency on U4 imported verification. The lifecycle has three stages, within the 8-stage and fan-in-2 caps (`DESIGN-MIL2.md:281–282`):
1. lock on S;
2. issue on D, consuming the receipt;
3. release on S, using receipt-membership evidence.

**Labels.**
- This slice tests the paired-claim relation and the late-receipt race. **It does not demonstrate bridge security.** R7's P29 stop rule rejects counting a single-chain stand-in as a complete bridge.
- **Assumption:** the pinned target must support an authenticated read or ledger-linked commitment of another instance's state. If it does not, the slice is blocked. It must not be simulated inside one contract.

**Positive control.** Lock 100 A on S, with `x` and a D-clock deadline T. Before T, D issues 99 wA to the recipient and routes 1 A to the fee recipient. D writes `receipt(x@D)`, and E1 holds on D with `Δsupply = +99`. S releases on membership evidence. The effects are read back from the ledger.

**Hostile control.** Same bytes, a valid proof and the same fees. The only change is that, after `receipt(x@D)` exists, a stage on S tries the refund branch with `after(T)` true on the S clock and no nonreceipt evidence. Expected result: rejection by the native or ledger check with a named code such as `NONRECEIPT_UNPROVEN`. It must not be rejected for a malformed envelope (`ROADMAP.md:40`).

**Required next hostile cases:**
- replaying the D issue (second receipt);
- evidence for the wrong instance or a sibling domain;
- receipt after the deadline on D;
- partition with no terminal transition allowed;
- a solver fee exceeding the cap.

## 5. Explicit disagreements

1. **With the MIL/2 race rule.** `priority` cannot decide cross-domain races (`DESIGN-MIL2.md:216`, `.tex:192`). For paired claims the destination's state decides.
2. **With the category report's recovery path** (`07-bridges.md:7,19`). A signed trusted-recovery attestation should only move a claim to `disputed`. It should not refund, unless its signers are bonded for at least the claim value under E5. Otherwise a trusted refund is an unpriced double spend.
3. **With the report's U0 scope** (`07-bridges.md:33`). U0 should freeze only the identity, index, enum and public-input slots. Transfer-form semantics and verifier modes belong in the library and U4. Freezing them at U0 would lock in unmeasured trust models.
4. **With the escrow state set** (`DESIGN-MIL2.md:203`). It needs `disputed` and `contingent` states now. They are non-terminal, so the ESC safety argument is unchanged.

## 6. Residual assumptions

- Midnight exposes an authenticated read or commitment across contract instances, so that anchored evidence works within Preview. This is unverified.
- Non-membership proofs of D state can be verified on S within ZKIR caps, or through ledger induction before U4. This is unmeasured.
- For imported domains, verifier honesty, finality depth and availability remain named premises. MIL/2 cannot prove a foreign chain honest (`MORIARTY-PRODUCT-CONTRACT.md:47`).
- The claim that the u192 conversion width fits the field rests on reasoning about the limb rule. Its cost has not been measured.
- Liveness under partition is conditional (O5, `.tex:204–206,251`). Silence never yields refund safety.