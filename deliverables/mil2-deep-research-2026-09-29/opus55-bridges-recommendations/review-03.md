# Recommendation: Bridges and cross-domain settlement, with an asynchronous lifecycle focus

*Independent Opus 5.5 review. Specified-only design work. I ran the guarded status check (`SP01.6 loan-swap-subset`, no pending transactions). Nothing here was implemented, proved, compiled or accepted by a ledger. Items marked **Rec.** are recommendations.*

## 1. Verdict

MIL/2 is **fit as a local substrate but unfit as an asynchronous bridge lifecycle.** The local parts are sound:
- a legitimate pending state (`DESIGN-MIL2.md:215`);
- per-witness terminal exclusivity with a tombstone (`:216`);
- foreign credit treated as imported evidence, not as a local effect (`:254`);
- a refund does not reset gross debit (`MIL2-PROPOSED-SEMANTICS.tex:189`).

The defect is structural. The ESC rule resolves a race "at the same authenticated head" (`.tex:192`), but a cross-domain race has no shared head. A signed `priority release | refund` (`DESIGN-MIL2.md:206`) is evaluated on the source ledger only, so it cannot stop a destination delivery that is already in flight. The status list `unknown, received, nonreceivedProved, final` (`.tex:202`) mixes up two things: what happened, and how final that knowledge is. There is also no cell that records partial delivery or who holds a fast-fill claim (`DESIGN-MIL2.md:239–242`). The category report correctly marks these areas partial or open (`07-bridges.md:18–20`).

## 2. Five ranked design edits (Rec.)

### Edit 1: Make the destination's one-shot receipt the single point that decides the race, and enforce the deadline on the destination
**Rule sketch.** Add two cells: `xfer(id)` on the source and `receipt(id)` on the destination. The receipt moves `absent → received(holder, amt) | timedOut` and writes each value only once.

- **Destination delivery.** A delivery stage on D requires all of:
  - `receipt(id)=absent`;
  - `stageTime(clk_D) < deadline_D`, where `deadline_D : Instant(clk_D)` is signed in the intent.
- **After the deadline**, D may only write `timedOut`.
- **Source refund** requires imported evidence of one of:
  - `receipt(id)=timedOut`; or
  - non-membership of `receipt(id)` at a D head `h` with `clk_D(h) ≥ deadline_D`.

Because the deadline is enforced on D, "absent after the deadline" can never change later, so no late receipt can appear.

**External practice (not a Moriarty rule).** IBC applies the same pattern:
- the destination refuses to receive a packet after its timeout (`source-text/ibc-v2-packet-handler.md:324–325`);
- the source times a packet out only with a non-membership proof of the receipt (`:402`; `ibc-ics004.md:1341,1425`);
- the receipt also blocks replay (`ibc-v2-packet-handler.md:229`).

Source: https://github.com/cosmos/ibc/blob/main/spec/IBC_V2/core/ics-004-packet-semantics/PACKET_HANDLER.md

**Counterexample.** Take the MIL/2 showcase pattern with a cross-domain release guard: `refund_when after(validity.end)`, `priority release` (`DESIGN-MIL2.md:309–311`). The relay is delayed, S refunds 100 after the deadline, and then D accepts the delivery. The same 100 is paid out on both chains. The S tombstone cannot see D.

**Placement.** U0 adds the cell sorts and the `timedOut` sentinel. §15 already assigns the cell vocabulary to U0 (`:345`), and unknown tags are rejected (`:285`), so this cannot be added after freeze. U3 carries the transition. Verifying the imported non-membership proof stays in U4 (`:350`).

### Edit 2: Authoring rule rejecting a deadline-only cross-domain refund
**Rule sketch.** If an escrow's release depends on evidence from another domain `d≠exec`, its `refund_when` must include `nonreceiptProved(id)` or `recoveredUnder(policy)` in its source set, and `priority` must be the new enum value `destination_receipt`. `after(deadline)` alone is rejected with `Reject(unsupported-async-refund)`. This puts into force the rule "a deadline only enables a signed policy branch" (`.tex:200`) and "timeout is not evidence of nonexecution" (`MORIARTY-PRODUCT-CONTRACT.md:47`).

**Counterexample.** Same as Edit 1. It is currently well-typed.

**Placement.** U0 adds the enum value and the Φ₀ atoms `nonreceiptProved(id)` and `received(id, seq)`. These are evidence predicates that §16.5 leaves open (`:359`); they must enter the frozen grammar or refunds cannot name them. U1 adds the check to the difference-logic authoring check. No change to Φ₁ is needed.

### Edit 3: Cumulative partial-delivery accounting in the transfer claim
**Rule sketch.**
- `xfer(id) = {committed: Qty<A_S>, rate: (n,m) literal, delivered: Qty<A_D>, closed: Bool, maxFills: u8}`.
- Each partial delivery on D writes `receipt(id, seq)` for `seq < maxFills` and checks:

  `(deliveredΣ + amt) × m ≤ committed × n`

  This is Φ₀ literal cross-multiplication; for u128 × literal, use the §4.5 limb rule.
- D ends the claim with a one-shot `closed(id, deliveredΣ)`, written either explicitly or when the deadline passes.
- The source refunds the remainder only against imported `closed`:

  `refund = committed − ceil(deliveredΣ × m / n)`

  Rounding: the source equivalent of the delivered amount rounds **up**, so the refund to the payer rounds **down** and custody can never go negative.
- Fees count within `committed`. Gross debit stays at `committed` (`.tex:189`).

**Counterexample.** Two 60-unit fills each pass a local `fill ≤ 100` check, so 120 is delivered. A second one: the source refunds 40 while a third partial fill is still in flight.

**Placement.** U3 (partial fill: `ROADMAP.md:24`; `DESIGN-MIL2.md:349`). The fields go in U0. A variable, oracle-priced rate would need Φ₁ and stays **deferred**. I do not change that boundary.

### Edit 4: A fast fill is a debt that takes over the receipt, not a new supply
**Rule sketch.**
- The filler's delivery on D writes `receipt(id)=received(holder=filler, amt)`. This is the same one-shot cell the canonical relay would write.
- The stage creates `Obligation{debtor: xfer(id), creditor: filler, principal: amt+fee, consent: owner's signed fast-fill policy}` (`DESIGN-MIL2.md:94–97`). Debt is kept separate from supply (`PRODUCT-CONTRACT.md:43`).
- If the canonical message arrives later, it finds the receipt held and pays the **receipt holder**. It never pays the owner a second time.
- The source discharges the obligation from custody only on imported `received(holder=filler)` at finality class `final`.
- The filler's bond is an `Encumbrance{against:{challengeObl(id)}, enforceable_by: challenge policy}`. A slash needs evidence of a wrong amount, recipient or asset during `window(clk_D)`.

**Counterexample.** The filler pays the owner 99 on D, then the relay mints 99 to the owner as well. D issues 198 against a 100 lock, while S reimburses only once. Or: the owner refunds on S after the fill because nothing marked the claim as taken.

**Placement.** U0: `ClaimClass` values and the right to create the obligation. U3: single-domain obligation and encumbrance semantics. U4: imported verification. U5: live solver competition (`ROADMAP.md:26`).

### Edit 5: Two status axes and bonded trusted recovery
**Rule sketch.** Split the status in `.tex:202` into two axes:
- **outcome:** `{unknown, received(seq), timedOut, closed}`;
- **finality class:** `{soft, final}`, a type index on `Obs`.

Release, reimbursement and tombstone require `final`. `soft` may only reserve under K1 (`.tex:229–236`).

Trusted recovery through `recover` (`DESIGN-MIL2.md:322`) is a separate terminal branch. It requires a k-of-n attestation with a ledger-checked signature; there is no in-circuit Ed25519 (`:264`). It **must** create a residual `Obligation` of the recovery authority, backed by an encumbered bond, to cover a valid delivery on D that is proved later. It also attempts a `closed-by-recovery` write on D when D is reachable.

**Counterexample.** Attestors sign "nonreceipt", the owner is refunded, and D then proves a delivery or a reorg brings it back. Today the loss belongs to no one, and the funds disappear through a local success result (`PRODUCT-CONTRACT.md:45`).

**Placement.** U0: the finality index and the recovery residual field. U3: the recovery-authority transition. U4/U5: the verifier and the federation epoch policy. The federated kernel stays optional; the direct path uses the same rule.

## 3. Core versus library (Rec.)

- **Core:**
  - the `xfer` and `receipt` cells and their states;
  - destination deadline enforcement;
  - the authoring rule for async refunds;
  - the cumulative delivery invariant and its rounding direction;
  - the receipt-holder and obligation rule;
  - the finality-class index;
  - the residual obligation for recovery;
  - replay by receipt.

  These are safety relations. A library cannot enforce them across programs.
- **Library:**
  - transfer forms (lock-mint, burn-mint, pool release);
  - fill pricing and fees;
  - bond sizing and challenge window length;
  - verifier adapters (light client, quorum, optimistic);
  - fill auctions and rebalancing.

## 4. Smallest slice and evidence pair (Rec., specified-only)

**Slice (U3 profile).** Put two program lineages, "S-side" and "D-side", on the single Midnight Preview domain. The receipt is then an *anchored* authenticated read, not imported evidence. This exercises the Edit 1–3 state machine without U4 verifiers, but it proves **nothing about cross-domain trust**. It must be labelled that way.

**Positive:**
1. The owner commits 100 A with `deadline_D`, `rate 1:1`, `maxFills 2`.
2. D-side delivers 60 (`seq 0`) before the deadline.
3. D-side writes `closed(id, 60)`.
4. S-side refunds 40 on the anchored `closed` read and writes the tombstone.

Expected result: gross debit 100, owner net outflow 60, D issues 60, and E1 holds on each side.

**Hostile (same witness shape, one field mutated):**
1. After step 2, submit an S-side refund of 100 that cites a D-side head from **before** the receipt, where `clk_D < deadline_D`.
2. Expected: reject, because non-receipt evidence requires `clk_D(h) ≥ deadline_D`.
3. Second control: a `seq 1` fill of 50, which fails `60+50 ≤ 100`.

Both controls are feasible positive-shaped witnesses. They are not malformed envelopes (`ROADMAP.md:40`).

## 5. Explicit disagreements

1. **Race policy.** `07-bridges.md:19` asks for "a race rule for a late valid receipt". I recommend making a late receipt *impossible*, by enforcing the deadline on D, rather than racing it. `priority` cannot decide a race across domains.
2. **Where fills are linearized.** `07-bridges.md:20` asks for "shared reservation accounting across competing fills". Competing fills should be decided by D's one-shot receipt. The source only reimburses whoever holds that receipt.
3. **Episodes.** The source and destination should not form one Episode. Episode links are ledger heads (`DESIGN-MIL2.md:46`), and no single ledger covers both chains. I recommend one Episode per domain, joined by `xferId` plus imported evidence. The 8-stage cap (`:281`) then applies per domain, and the signed `maxFills` must fit within it.
4. **Verifier typing at U0.** The report puts verifier/finality policy typing in U0 (`07-bridges.md:33`). I would freeze only the finality-class index and the evidence atoms at U0. Concrete verifier modes stay in U4 so U0 does not overcommit.

## 6. Residual assumptions

- The destination is programmable and enforces its own deadline. A non-programmable destination such as Bitcoin supports only bonded trusted recovery (Edit 5).
- D state supports non-membership proofs, and S can verify them from U4 onward.
- The Midnight ledger supports one-shot cell writes of the nullifier kind and ledger-checked attestation signatures. Neither has been verified here.
- Relay and actor liveness, and the definition of finality, are named premises. They are not proved.
- A signed literal conversion rate is enough for the first profile.
- ERC-7683 (https://eips.ethereum.org/EIPS/eip-7683) is a possible comparison for fill deadlines. It is **not** captured in the repository, and I did not rely on it.