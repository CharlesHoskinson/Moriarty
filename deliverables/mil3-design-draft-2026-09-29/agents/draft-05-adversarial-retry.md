I have what I need: MIL/2, the proposed-semantics LaTeX, all eight syntheses with targeted reads of the underlying reviews, the U0 exit gate and numeric profile, and the pinned ZKIR v3 capture. The draft follows.

---

# MIL/3 candidate: independent red-team proposal and integrated draft

**Author role:** a fresh, independent Claude Opus 5.5 drafting agent. **Date:** 2026-09-29. **Baseline:** worktree `mil2-primary-research-20260929` at `983a4bb4`.

**Status:** this is a proposal from one agent. It is not an integrated specification, and nobody has voted on it or adopted it. Every rule below is **specified-only**. I ran no tests, proofs, compilers or Preview transactions, and I did not read any other MIL/3 agent's output.

`status --json` reports `SP01.6 loan-swap-subset` blocked on stale inputs and missing operational history. This read-only drafting task does not touch that capability.

Tags follow MIL/2 §0 (`concepts/intent-language/DESIGN-MIL2.md:8-14`):
- **[checked]** means I verified it against a file or by hand arithmetic shown here.
- **[obligation]** means it must be proved or measured before freeze.
- **[deferred]** means it is deliberately not claimed.
- **[comparative]** marks an external practice. It is not Moriarty evidence.

---

## 1. Design thesis and scope

### 1.1 Thesis

MIL/2 folds three different kinds of arithmetic into one Φ₀/Φ₁ axis. The eight category syntheses all push on that axis from different directions. MIL/3 should split it into three strata with separate trust, width and milestone rules:

| Stratum | Who writes it | Where it is evaluated | Arithmetic | Milestone |
|---|---|---|---|---|
| **Φ₀**: signed intent predicates | The signer | Stage acceptance (`GuardsTrue`, `SignedBound`) | Linear, literal coefficients only. Unchanged from MIL/2 §4.4 | U0 grammar |
| **Κ**: certified transition kernel | The program or profile author, from a **closed, versioned list** | `ProgramValid_P`, the new stage conjunct | Named nonlinear verify-relations with explicit width preconditions and role-fixed rounding | U0 names them, U1 certifies each one, U2 uses them |
| **Φ₁**: general nonlinear predicates | The signer | Stage acceptance | Variable × variable, author-chosen | Stays at U4, as in MIL/2 §15 |

This split answers the first common tension (certified transition arithmetic before general Φ₁). A pool invariant, a vault conversion or a payoff is a property of **one concrete witness transition** of a named program. It is not an author-quantified predicate. So it needs a *certificate for a fixed relation*, not an SMT-backed authoring language. The AMM, staking, lending, stablecoin and oracle syntheses all ask for this; §5 gives the anchors.

The other three tensions have the same shape: a quantity that two parties both want, and that MIL/2 leaves unallocated.
- **Quote vs surplus:** output above the trader's floor.
- **Bridge timeout vs unknown receipt:** the outcome of an in-flight message.
- **Vault totals vs donation:** custody that exceeds the accounted total.

MIL/3's general rule is **no unallocated value**. Every such gap must be named by a cell and assigned by a signed clause or a program rule. Otherwise the stage rejects.

### 1.2 Scope

**In scope:** the three-stratum split, a numeric width discipline grounded in the pinned ZKIR v3 text, a stage relation with `ProgramValid` and `SurplusAlloc`, a cross-domain transfer claim state machine, vault accounting, a rejection taxonomy, an eight-profile conformance matrix, proof obligations, migration from MIL/2, and falsification experiments.

**Out of scope, [deferred]:** Φ₁ admission; n-party shared-write clearing (MIL/2 decision 4, arity still reserved); concentrated liquidity (decision 5); flash-loan traces (decision 2); in-circuit parent verification (U4); foreign light-client verification (U4); governance voting libraries; perps and portfolio margin; restaking correlated-loss budgets; rebasing stablecoins; system-wide shutdown pro-rata. For each of these I only reserve tags where the unknown-tag rule would make a later addition a breaking change.

**Core vs library.** *Core* means the stage relation, typing, the Κ list and its certificates, cell vocabulary, conservation, the lock aggregate, surplus allocation, the transfer-claim state machine, vault accounting invariants and rejection codes. *Library/profile policy* means fee tiers, bootstrap constants, recognition schedules, queue parameters, reward curves, liquidation bands, voting procedures, verifier-committee membership and attestor bonds. A library can narrow core acceptance. It can never widen it.

**Not a process gate.** The conformance matrix in §4 describes specification conformance of language profiles. It adds no compile, prove or deploy prerequisite for developers (`plugins/moriarty-dev/skills/develop/SKILL.md`, "Permissionless product boundary").

---

## 2. Red-team of the four tensions

### T1. Certified transition arithmetic before general Φ₁

**Claim under test.** The AMM synthesis says "a narrow certified operation at U1 … with the U0 profile reserving its syntax, widths and public inputs" (`opus55-amm-recommendations/SYNTHESIS.md:16`). Staking, lending, stablecoins and oracles make parallel claims (`opus55-staking_yield-recommendations/SYNTHESIS.md:251`, `opus55-lending-recommendations/SYNTHESIS.md:150`, `opus55-stablecoins-recommendations/SYNTHESIS.md:212`, `opus55-oracles-recommendations/SYNTHESIS.md:191`).

**Counterexample 1: the width premise is miscited. [checked]**

MIL/2 §4.5 and the review report say `less_than` "stops at 253 bits" (`DESIGN-MIL2.md:169`, `concepts/intent-language/review/REVIEW-REPORT.md:96`, via `wiki/contradictions.md:116`). The pinned ZKIR v3 capture says something else:
- `LessThan { bits }` requires only `bits < FR_BITS` with `FR_BITS = 255` (`deliverables/mil2-deep-research-2026-09-29/source-text/zkir-v3-spec.md:80,497-498`).
- The **circuit** enforces only `⟦a⟧,⟦b⟧ < 2^e` with `e = max(n + n mod 2, 4)`.
- The exact bound `< 2ⁿ` is a **producer obligation O4**, not a circuit fact (`zkir-v3-spec.md:862`).
- `Mul` is field multiplication `⊗` (`:859`).
- `DivModPowerOfTwo`/`ReconstituteField` need `bits ≤ 248` (`:989-991`).

Two consequences follow:
1. For odd `n`, a `LessThan` alone admits an operand up to `2^{n+1}`. Both reviews assume `LessThan` range-checks its operands: review-02's "`le253`" and review-05's "under the 253-bit limit" (`opus55-amm-recommendations/review-02.md:19-26`, `review-05.md:60`). It does not.
2. The actual ceiling (254) is looser than the one MIL/2 records (253). The recorded figure is therefore unreliable in both directions until U1 measures the pinned tuple.

MIL/3 rule N1 (§3.2) therefore requires an explicit `ConstrainBits` on every operand and even `n`.

**Counterexample 2: the two proposed CPMM rules are not equivalent. [checked by hand]**

The AMM synthesis asks for a proof that review-02's witnessed rule and review-05's inequality-plus-tightness "characterize the same output" (`SYNTHESIS.md:29`). They do not, because they charge the fee differently. Take the fixture from `review-05.md:195`: `x = 1000`, `y = 2200`, fee `30/10000`, `dx = 11`, decimals 0.

*Review-02* (`review-02.md:43-47`) uses φ = ⌈11·30/10000⌉ = 1, so e = 10, and requires `(x+e)·y' ≥ x·y`:
- `1010·y' ≥ 2,200,000`, so `y' ≥ 2179`, so **dy ≤ 21**.
- Check: 1010·2178 = 2,199,780 < 2.2·10⁶, and 1010·2179 = 2,200,790.

*Review-05* (`review-05.md:52`) requires `(x'·φd − dx·φn)·y' ≥ x·y·φd`:
- x'·φd − dx·φn = 10,110,000 − 330 = 10,109,670.
- The target is 22,000,000,000.
- 10,109,670·2177 = 22,008,751,590 ≥ target, and 10,109,670·2176 = 21,998,641,920 < target, so **dy ≤ 23**.

Review-02 rounds the fee up to one whole unit *before* the invariant, which transfers ~0.967 A of value to LPs. Review-05 applies the exact rational fee inside the invariant, as Uniswap v2 does **[comparative]**: `balance0Adjusted·balance1Adjusted ≥ reserve0·reserve1·1000²` (https://github.com/Uniswap/v2-core/blob/master/contracts/UniswapV2Pair.sol).

So choosing between the rules is an **economic decision about who owns fee-rounding value**, not an implementation detail. The "prove they are equivalent" experiment would fail. It should be replaced by the decision D3 in §7.

**Counterexample 3: an escape hatch through a growing Κ list.** If any profile author can add a Κ op, Κ becomes Φ₁ without the U4 gate.

*Rule:* the Κ list is closed inside the version header. Adding an op is a version increment. Each op must carry a U1 certificate before any accepted stage uses it. Signed intents can never call Κ ops directly. They can only read post-state cells a Κ op wrote.

**Recommendation.** Adopt the three-stratum split. **Dissent preserved:** review-01 favours a general two-limb u128 path over narrow widths (`opus55-amm-recommendations/SYNTHESIS.md:26`). Two-limb Κ ops stay a legal later version, and D2 records the evidence needed to choose.

### T2. Quote exactness versus surplus

**Claim under test.** The AMM synthesis resolves this by fixing "pool execution to the exact quote" and then allocating surplus in a signed clause (`SYNTHESIS.md:27`). It frames exactness as protection against "LP value leakage".

**Counterexample: tightness protects the owner, not the LPs. [checked by construction]**
- The fee-aware inequality `INV` alone already prevents LP loss. Any `dy` satisfying `INV` leaves `k` non-decreasing.
- Without `TIGHT`, a solver colluding with LPs can deliver `dy = 21` from a pool whose maximum valid output is 23. It pays the owner 21 ≥ floor 20 and leaves 2 B in the pool as a donation to LPs it controls.
- `surplus to owner` cannot see this, because surplus measured against *delivered* output is 1, not 3.

So a surplus clause without pool tightness is **unenforceable**, and pool tightness without a surplus clause is **unallocated**. Both are required, and each is useless without the other. Trace H2 in §5 shows MIL/2 plus review-02 accepting this skim.

**A second counterexample: tightness is not best execution.** `surplus to owner` plus `TIGHT` still lets a solver pick a worse pool among permitted venues, or sandwich the pool's pre-state (`opus55-amm-recommendations/review-03.md:162`). MIL/3 does not claim best execution. It claims **conditional exactness**: given the authenticated pre-state and chosen venue, all value above the floor goes where the signed clause says. Ordering and MEV exposure stay a declared residue item, and venue restriction or an anchored reference-price guard (Φ₀ with literal coefficients) is optional signed policy.

**Recommendation.** `TIGHT` is mandatory for every Κ swap op. `surplus` is a mandatory clause with **no canonical default**. **Dissent:** review-03 wants `split n/d` in core (`review-03.md:25-34`). I keep it in core because it is Φ₀-expressible, but D4 records whether the canonical form should forbid defaulting.

### T3. Bridge timeout versus unknown foreign receipt

**Claim under test.** MIL/2's escrow `priority` and the LaTeX ESC rule resolve races "at the same authenticated head" (`MIL2-PROPOSED-SEMANTICS.tex:192-199`). A cross-domain transfer has no shared head (`opus55-bridges-recommendations/review-03.md:13`).

**Counterexample 1: a false refund from a source clock. [checked against the product contract]**
- The escrow is `refund_when after(deadline_src)`, with release conditioned on a destination credit.
- The destination receives the message at its local time `< deadline_D` but after `deadline_src` on the source clock, or the relay is merely slow.
- The source refunds and the destination credits, so the owner is paid twice from one lock.
- MIL/2 permits this pattern, and it violates "Timeout is not evidence of nonexecution" (`docs/MORIARTY-PRODUCT-CONTRACT.md:47`).

**Counterexample 2: attested nonreceipt followed by a late receipt.** Attestors sign nonreceipt, the source refunds, and a later reorg or delayed relay produces a receipt. Under an attested policy this is not preventable locally. The loss currently "belongs to no one" (`opus55-bridges-recommendations/review-03.md:89`).

**Comparative practice [comparative].** IBC makes the *destination* decide: `timeoutHeight`/`timeoutTimestamp` are destination-chain values after which "the packet will no longer be processed". Receipts are sentinel values the receiving chain writes (https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md; capture `source-text/ibc-ics004.md:319-335`).

**Recommendation.**
1. The destination owns a one-shot receipt cell with states `absent → received(seq) | timedOut`.
2. `timedOut` can be written only when the destination's authenticated clock is at least `deadline_D`, and a receipt after `timedOut` rejects.
3. A source refund needs evidence of `timedOut`, and its **exclusivity guarantee is scoped to the evidence class**:
   - anchored, when the destination is a Midnight contract on the same ledger;
   - imported(verified), which is [deferred] to U4;
   - attested, which requires declaring `exclusivity premise(attestors)` and creates a contingent loss obligation.
4. If no evidence arrives, the transfer stays **pending indefinitely** unless a named recovery policy with named liveness assumptions applies.

**Dissent preserved:** optimistic timeout with a challenge window, from review-02/05's disputed state (`SYNTHESIS.md:68`). This is admissible as a *library policy* over the attested mode. It cannot upgrade exclusivity.

### T4. Vault account totals, rounding and donation

**Claim under test.** MIL/2 §3.3 lets the author supply `Rounding`, leaves `totalAssets` undefined (raw custody or accounted?), allows a "first-depositor convention", and sends remainders to `retained-in-pool` (`DESIGN-MIL2.md:85-89`).

**Counterexample 1: donation inflation. [checked]** This is the attack in `opus55-staking_yield-recommendations/review-05.md:42-57`. With `totalAssets` read from raw custody and no offset:
1. The attacker deposits 1 unit and receives 1 share.
2. The attacker donates `10⁶` units directly to custody.
3. A victim deposits `10⁶` and receives `⌊10⁶·1/(10⁶+1)⌋ = 0` shares.

Every step satisfies E1 conservation. Conservation is necessary but not sufficient.

**Counterexample 2: accounted totals move the attack to recognition.** If yield must be "recognized" into the accounted total, whoever triggers recognition controls a price jump. A permissionless `recognize` re-opens the donation attack. A privileged one creates a front-runnable event for anyone who can predict it. Core can make recognition *explicit and authorized*, using the existing `reconcile` right (`DESIGN-MIL2.md:191`). Smoothing it, for example by vesting, is library policy.

**Counterexample 3: the virtual offset is itself a remainder sink. [checked by algebra]** With a virtual offset `(vS, vA)`, a fraction `vS/(S+vS)` of every recognized gain accrues to virtual shares that nobody can redeem. That value is locked in custody forever. The U0 numeric profile's default remainder beneficiary is `protocol-reserve`, and that reserve mechanism is `absent` (`deliverables/u0-semantic-contract-2026-09-23/numeric-profile.json`, `defaultPolicy`, `reserveMechanism.status`). MIL/2's `retained-in-pool` therefore contradicts the recorded U0 default, and the offset's leakage has no recorded beneficiary.

**Recommendation.**
- The rounding direction is fixed by the operation, following the four ERC-4626 directions **[comparative]** (https://eips.ethereum.org/EIPS/eip-4626; capture `source-text/erc4626.md:631-636`).
- A `managed` accounted-total cell is required, with `managed ≤ custody`.
- Unrecognized custody surplus is a named quantity.
- `recognize` is a `reconcile`-authorized transition.
- The bootstrap policy is mandatory and immutable (virtual offset or dead shares; the δ constant is library policy).
- The remainder beneficiary is declared per profile. This needs a U0 numeric-profile amendment (D8).

---

## 3. Normative draft: load-bearing sections

### 3.0 Proposed MIL/3 document outline

```
0  Status and claim tags
1  Layers: Intent / Plan / Episode / Stage (MIL/2 §2 unchanged)
2  Strata: Φ₀, Κ, Φ₁ (reserved)                                  ← NEW (§3.1 below)
3  Types and identity (MIL/2 §3; Rounding parameter removed)
4  Numeric profile: widths, N1–N4, role-fixed rounding, remainder  ← REWRITTEN (§3.2)
5  Φ₀: formation, totality, stratified pre/post                   ← AMENDED (§3.3)
6  Κ: the closed kernel list and certificate schema                ← NEW (§3.4)
7  Evidence: identity-keyed source sets, verifier-assigned labels  ← AMENDED (from oracles synthesis)
8  Authority: eight rights, grant epoch, head freshness             ← AMENDED
9  Stage acceptance relation with ProgramValid and SurplusAlloc     ← REWRITTEN (§3.5)
10 Escrow (same-head) and TransferClaim (cross-domain)              ← SPLIT (§3.6)
11 Pools and vaults: accounted totals, recognition, bootstrap      ← NEW (§3.7)
12 Obligations, encumbrances, lock aggregate                        ← AMENDED
13 Footprints: alias-closed derivation, custody expansion          ← AMENDED
14 Effects, conservation, supply-as-effect, residue                 ← AMENDED
15 Canonical form, digest, lowering (N1 lowering added)
16 Rejection taxonomy                                               ← NEW (§3.8)
17 Profiles and conformance matrix                                  ← NEW (§4)
18 Version header, caps, Κ list
19 Migration from MIL/2                                             ← NEW (§8)
20 Obligations before freeze (O1–O6 retained; O7–O16 added)
21 Deferred register
```

### 3.1 Strata

**Definition S (strata).** Let `Φ₀` be the MIL/2 §4.3 grammar minus `sharesFor`, `assetsFor` and `mulDiv`. Let `Κ = {κ₁,…,κ_m}` be the finite list of kernel operations named in the version header (§3.4). Let `Φ₁` be reserved.

**Rule S1 (intent confinement).** A signed intent `I` may contain only Φ₀ predicates. A Κ-op name in `I` is rejected with `ADM_STRATUM`. `I` may read any cell `c` through `pre(c)` or `post(c)`, including cells a Κ op writes.

**Rule S2 (program confinement).** A program transition relation `T_P` (the "pool rule", "vault rule" and so on) may use Φ₀ and the Κ ops its profile lists. Using an unlisted Κ op rejects with `ADM_KERNEL_UNLISTED`.

**Rule S3 (no stratum promotion).** No Φ₀ term's value may be *defined* by a Κ op inside the intent. Κ affects the intent only through committed post-state. **[obligation O7]** Prove that S1–S3 imply that the signer's observable trace `π_I` (tex:109) is determined by Φ₀ semantics plus `post` cell values, with no Κ internals visible or required.

### 3.2 Numeric profile: width discipline

**Symbols.**
- `F = FR_BITS = 255`, the scalar field size from the pinned ZKIR v3 spec (`source-text/zkir-v3-spec.md:79-80`).
- `bits(v)` is the declared upper bit bound of value `v`.
- `CB(v, k)` is a `ConstrainBits` asserting `v < 2^k`.
- `LT(a, b, n)` is a ZKIR `LessThan`.

**Rule N1 (explicit ranges).** Every operand of every `LT(a,b,n)` emitted for a Φ₀ or Κ comparison must be preceded by `CB(a,n)` and `CB(b,n)`, with `n` even and `n ≤ N_max`. The candidate `N_max` is 252, a margin below the pinned `n < 255` bound. The measured value is **[obligation O8]**, a U1 target-pin measurement.

*Rationale.* The circuit bound is `2^{n + n mod 2}` (`zkir-v3-spec.md:862`), so exactness depends on the producer. *Rejection:* `NUM_RANGE_UNPROVED` at lowering.

**Rule N2 (products).** A Κ op may emit a field `Mul(a,b)` only if `bits(a) + bits(b) ≤ N_max`, where each `bits(·)` is established by a preceding `CB`. Otherwise the product must be a declared two-limb gadget (MIL/2 §4.5 limb rule, retained). *Rejection:* `NUM_PRODUCT_WIDTH`.

**Rule N3 (Qty stays u128).** `Qty<A>` remains u128. A Κ op with a narrower operand width `w < 128` must `CB` its inputs at `w`. A legitimate value above `w` rejects `NUM_PROFILE_RANGE`. This is fail-closed and incomplete by design.

**Rule N4 (role-fixed rounding).** Each Κ op fixes the rounding direction of each output from the output's *role*, never from an author parameter:
- the amount a party **receives** rounds **down**;
- the amount a party **owes or pays** rounds **up**;
- the remainder goes to the beneficiary declared in the profile (§3.7, D8).

This matches the U0 `defaultPolicy` (`obligation: ceil, receipt: floor`, `numeric-profile.json`). The `Rounding` parameter in MIL/2 §3.3 is **deleted**.

**Note on U0 `UInt256`.** The recorded U0 widths include `UInt256` (`numeric-profile.json`, `units.widths`), which cannot be one element of a 255-bit field. Under N2, any `UInt256` operand is necessarily multi-limb. **[obligation O9]** Reconcile the U0 width list with N1–N3.

### 3.3 Φ₀ amendments

- **Stratified `pre`/`post`.** A guard that selects a branch may read only `pre(·)` and authenticated observations. `post(·)` may appear only in postconditions (`ensures`) evaluated after `FinancialPrepared`. This matches the existing Core rule that `ReadPre(post,…)` is legal only in `Ensure` (tex:83). *Rejection:* `TYP_POST_IN_GUARD`.
- **Reserved atoms, admitted by evidence class:** `received(id, seq)`, `timedOut(id)`, `recoveredUnder(policyDigest)`. They enter the U0 grammar now, because unknown tags reject (`DESIGN-MIL2.md:285`). A foreign-domain instance of these atoms is admissible only with evidence class `anchored` until U4 imported verification exists. Otherwise it rejects `EVID_CLASS_UNAVAILABLE`.
- **Surplus terms** (§3.5) are Φ₀: `surplus = delivered − floor` (checked subtraction), and `split n/d` uses literal coefficients.

### 3.4 Κ: kernel list and certificate schema

Each Κ op is a **verify-relation**. The witness supplies the outputs, and the circuit checks a relation. No op computes a quotient by field inversion.

**Certificate schema** (all fields required before any U2 use):

```
κ ::= { name, version,
        inputs  : [(sym, width)],        -- each CB'd per N1
        outputs : [(sym, width, role)],  -- role ∈ {receive, pay, state}
        pre     : Φ₀ over inputs         -- e.g. x > 0, dy < y
        rel     : conjunction of LT / Mul / Add atoms satisfying N1–N2
        tight   : optional Φ₀ over outputs  -- maximality/minimality
        rejects : [code]                 -- every partial case named
        cert    : { completeness: valid witness ⇒ satisfiable,
                    soundness:    satisfiable ⇒ rel holds over ℤ,
                    cost:         measured constraint count @ pinned tuple } }
```

**Candidate Κ list for MIL/3 profile 1** (names reserved at U0; each [obligation] for its U1 certificate):

| Κ op | Relation (over ℤ, after N1 ranges) | Widths | Used by |
|---|---|---|---|
| `cp_swap_in/1` | `INV(dy) ≜ (x'·φd − dx·φn)·y' ≥ x·y·φd`, `TIGHT ≜ ¬INV(dy+1)`, with `x' = x+dx`, `y' = y−dy`, `0<dy<y`, `φn<φd` | x, y, x' < 2¹¹²; φd ≤ 2¹⁴; max product < 2²³⁸ | AMM |
| `cp_swap_out/1` | same `INV`, `TIGHT ≜ ¬INV_{dx−1}` (minimal input) | same | AMM |
| `divmod_floor/1` | `q·d + r = n ∧ 0 ≤ r < d ∧ d > 0` | n < 2²⁴⁰, d < 2¹²⁰ | vault, derivatives |
| `divmod_ceil/1` | `q·d − r' = n ∧ 0 ≤ r' < d ∧ d > 0` | same | vault, lending accrual |
| `vault_conv/1` | four ERC-4626-direction conversions via `divmod_*` (§3.7) | M, S, a, s < 2¹²⁰ | staking/yield |
| `pos_part_scaled/1` | `p = ⌊q·max(0, P−K)/10^s⌋` via `divmod_floor`; `P, K : Price<B,Q,s>` same `s` | q < 2¹¹², P, K < 2¹²⁰ | derivatives |
| `price_cmp/1` | `q·P.m ≥ θ·D·10^s` for collateral health | **reserved, not admitted in profile 1** | lending, stablecoins |

`cp_swap_in/1` uses review-05's exact-rational fee form, not review-02's rounded-fee form. That choice is D3, and the value at stake is counterexample 2 in T1.

*Width arithmetic [checked]:* `(x'·φd − dx·φn) < 2^{112+14} = 2^{126}`, and times `y' < 2^{112}` gives `< 2^{238}`. `x·y·φd < 2^{238}`. Both are ≤ 252.

### 3.5 Stage acceptance relation (MIL/3)

**Symbols.**
- `I` is the signed intent and `σ` its completion.
- `P` is the selected program, with nominal id `pid` and version `v`, read at the authenticated head `h`.
- `s, s'` are the pre- and post-state, `o` the admitted observations, and `e` the complete effect list.
- `cells(P)` is P's declared custody and state cells.
- `T_P(s|cells(P), e, s'|cells(P))` is P's transition relation.

```
Stage(I,σ,P,h,s,o,e,s') ≜
    Admit(I)                  -- canonical form, version, caps, Κ list, unknown tags
  ∧ Typed(I,σ)                -- formation incl. source sets, stratification
  ∧ SignedBound(I,σ)          -- Fill(I,σ) (tex:109): holes, fixed bytes, domain
  ∧ HeadFresh(h,P,s)          -- s is the authenticated state at h; P@v current at h
  ∧ EvidenceValid(o,h)        -- every named observation admitted (not only evaluated)
  ∧ GuardsTrue(I,σ,s,o)       -- Φ₀ over pre/obs only
  ∧ ProgramValid_P(s,e,s')    -- NEW: T_P holds, incl. every Κ rel and tight clause
  ∧ EffectExact(e,s,s')       -- s' = apply(e,s) on every written cell; no other writes
  ∧ Conserve(e)               -- E1 (tex:171-176) with supply derived from mint/burn
  ∧ SurplusAlloc(I,σ,e)       -- NEW: see below
  ∧ AuthorityFresh(I,h)       -- grant epoch, budget, window, replay
  ∧ LiabilityRoll(s,e,s')     -- L1 (tex:180-187)
  ∧ LocksSafe(s')             -- K1 via authenticated aggregate cell, every debit
  ∧ FootprintSound(I,P,e)     -- alias-closed derived ⊆ declared
  ∧ Ensures(I,σ,s,s')         -- Φ₀ postconditions over pre/post
  ∧ HistoryLink(h,I)          -- predecessor, tombstones, roll-forward
  ∧ FailurePolicy(I,phase)    -- ledger phase layout (PRODUCT-CONTRACT:47)
```

Acceptance is the conjunction. The clause order fixes only **which rejection code is reported**. Acceptance itself is order-independent. **[obligation O10]** Prove that reporting order does not change the accept/reject decision, including under `And` short-circuiting (tex:95).

**`SurplusAlloc` (signed clause, Φ₀).**
- For each signed output floor `net ≥ N B to r`, let `D` be the total `B` credited by `e` out of the program's output leg, before any split.
- `surplus_B = D − N`. If `D < N`, the checked subtraction rejects with `INT_FLOOR`.
- The intent's clause `surplus ∈ {to owner | to completer c | split n/d}` requires one of:
  - *to owner:* `credit(r,B) = D` and `credit(c,B) = 0` for every completer `c`;
  - *to completer:* `credit(r,B) = N`;
  - *split n/d:* `credit(c,B)·d ≤ surplus_B·n` and `credit(r,B) = D − credit(c,B)`, with rounding favouring the owner.
- Any other credit of `B` from the output leg rejects with `INT_SURPLUS_MISALLOC`.

`SurplusAlloc` is meaningful only because `ProgramValid` with `TIGHT` pins `D` to the program's maximum valid output. For RFQ or maker offers, `TIGHT` is replaced by the maker's signed price, and `D` is the maker's committed output.

**Fees (D5).** `fees(stage) = Σ explicit fee transfers + Σ_hops ⌈dx_i·φn_i/φd_i⌉`. Embedded pool fees count toward the signed fee cap (`review-05.md:83-93`). Without this, a 10 % pool satisfies `fees ≤ 1 A`.

### 3.6 Escrow (same head) and TransferClaim (cross-domain)

**Escrow** keeps MIL/2 §7 and the tex ESC rule unchanged for **same-head** races: one domain, both guards evaluated at the same authenticated head.

**Rule X0 (no cross-domain priority).** If a release guard's source set contains an observation from domain `d ≠ exec`, then `priority ∈ {release, refund, signed_order}` is ill-formed (`XDOM_PRIORITY`), and the escrow must be expressed as a `TransferClaim`.

**TransferClaim.**

```
TransferClaim τ = { id, src, dst, asset_src, asset_dst, form ∈ {lockMint, burnMint, release},
                    ratio (num, den), amount, owner, recipient, policy: VerifierPolicyDigest,
                    deadline_D : Instant(clk_dst), nonce }
Source cell   srcState(τ) ∈ {locked, refunded, settled}
Dest cell     dstState(τ) ∈ {absent, received(seq, amt), timedOut}   -- one-shot
```

Transitions (each one is an accepted stage on its own domain):

```
(X1) dst: absent → received(seq,amt)   requires  clk_dst(h) < deadline_D
                                        ∧ verified source lock/burn for τ.id
                                        ∧ amt ≤ remaining(τ)           (partial allowed)
(X2) dst: absent → timedOut            requires  clk_dst(h) ≥ deadline_D
(X3) dst: timedOut → *                 REJECT    XDOM_LATE_RECEIPT
(X4) src: locked → refunded            requires  ev ⊨ timedOut(τ.id) with class(ev) = κ_ev
(X5) src: locked → settled             requires  ev ⊨ received(τ.id, …) covering amount
```

**Exclusivity theorem shape [obligation O11].** If `class(ev) ∈ {anchored, imported-verified}` and the verifier policy is sound, then `refunded(τ)` and `received(τ,·)` are never both accepted.

If `class(ev) = attested(k,n)`, MIL/3 claims **nothing** beyond the premise. The intent must declare `exclusivity premise(attestors: policyDigest)`, and X4 must also create an obligation `Obligation{debtor: attestorBond, creditor: loss-bearer, contingent on received(τ)}`. The bond sizing and loss-bearer are **library** (D6).

**Rule X6 (unknown stays pending).** No transition fires on the absence of evidence. A workflow claiming a recovery guarantee must name its liveness premises (inclusion, relay, actor arrival, witness availability), per MIL/2 §7 and tex:205. Otherwise it rejects `ESC_RECOVERY_UNPREMISED`.

**Rule X7 (source-clock refund forbidden).** `refund_when after(t)` with `t : Instant(clk_src)` on a claim whose release depends on `dst` is rejected at authoring with `XDOM_TIMEOUT_NOT_EVIDENCE`.

### 3.7 Pools and vaults

**Cells.**
- `custody(V,a)` is a balance cell.
- `managed(V)` is the accounted total, `Qty<a>`.
- `supplyShares(V)` is `S`.
- `shares(V,acct)` is a holder's shares.
- `bootstrap(V)` is immutable: `virtualOffset(vS ≥ 1, vA ≥ 1)` or `deadShares(m ≥ 1)`.
- `remBeneficiary(V)` is immutable (D8).

**Invariants [obligation O12].**
- V1: `managed(V) ≤ custody(V,a)`.
- V2: `Σ_acct shares(V,acct) = S`.
- V3: `S = 0 ⇒ managed(V) = 0`, except for dead shares.

**Conversions** (`vault_conv/1`, with `Ŝ = S + vS` and `M̂ = managed + vA`):

| Operation | Given | Computes | Rounding | Role |
|---|---|---|---|---|
| deposit | a | s = ⌊a·Ŝ / M̂⌋ | down | receive |
| mint | s | a = ⌈s·M̂ / Ŝ⌉ | up | pay |
| withdraw | a | s = ⌈a·Ŝ / M̂⌉ | up | pay |
| redeem | s | a = ⌊s·M̂ / Ŝ⌋ | down | receive |

- `deposit` with `s = 0` or `redeem` with `a = 0` rejects `VAULT_ZERO_OUT`.
- Every conversion reads `managed(V)`, **never** `custody`. Reading custody is a footprint violation: `custody` is not in the conversion's derived read set, so a witness supplying it fails `HeadFresh`/`EffectExact` with `STATE_READ_MISMATCH`.

**Recognition.** `recognize(V, Δ)` requires `Δ ≤ custody − managed` and the `reconcile` right scoped to `V`, and it sets `managed' = managed + Δ`. Loss is the mirror operation: `writeDown(V, Δ)` is **mandatory** in any stage that debits custody other than by redeem or withdraw. Otherwise V1 can break (`VAULT_UNRECOGNIZED_LOSS`).

**No-free-lunch [obligation O13].** For every reachable state, `redeem(deposit(a)) ≤ a` and `mint(s)` followed by `redeem(s)` returns `≤ mint(s)`. Proof route: monotonicity of floor and ceil over positive `Ŝ, M̂`, then exhaustive check at small widths.

**Library, not core:** the δ constant (review-05's `δ ≥ 3` is a candidate, `review-05.md:48`), recognition vesting, fees, rewards, withdrawal queues, slashing schedules.

### 3.8 Rejection taxonomy

Every rejection is fail-closed. A `Reject` is never a truth value (MIL/2 §4.2). The existing Core codes `TYPE_MISMATCH`, `WORK_EXHAUSTED`, `INPUT_SPAN` and `ARITH_RANGE` are **retained** as their existing meanings (`experiments/moriarty-language/spec/successor/semantic-contract.md`, tex:95). The new codes are proposals.

| Family | Code | Trigger | Stage clause |
|---|---|---|---|
| ADM | `ADM_VERSION`, `ADM_UNKNOWN_TAG`, `ADM_CAP_<name>` | Header, tag or cap violation | Admit |
| | `ADM_STRATUM` | Κ op named in a signed intent | Admit |
| | `ADM_KERNEL_UNLISTED` | Program uses a Κ op not in its profile | Admit |
| | `ADM_NONCANONICAL` | Encoding not canonical | Admit |
| TYP | `TYPE_MISMATCH` (existing) | Asset, clock, scale or sort mismatch | Typed |
| | `TYP_POST_IN_GUARD` | `post` in a branch guard | Typed |
| | `TYP_SOURCE_SET` | Required-anchored position gets a wider source set | Typed |
| EVAL | `ARITH_RANGE` (existing) | Width overflow | GuardsTrue/Ensures |
| | `EVAL_UNDERFLOW`, `EVAL_DIV_ZERO`, `EVAL_KOFN`, `EVAL_CLOCK` | MIL/2 §4.2 partials | any Φ₀ |
| | `WORK_EXHAUSTED` (existing) | Metering | any |
| NUM | `NUM_RANGE_UNPROVED` | N1: missing `CB` or odd `n` at lowering | lowering |
| | `NUM_PRODUCT_WIDTH` | N2 | lowering |
| | `NUM_PROFILE_RANGE` | N3: value exceeds Κ width | ProgramValid |
| PROG | `PROG_STALE_VERSION` | P@v not current at h | HeadFresh |
| | `PROG_INVARIANT` | Κ `rel` false | ProgramValid |
| | `PROG_QUOTE_NOT_TIGHT` | Κ `tight` false | ProgramValid |
| | `PROG_PRE` | Κ `pre` false (for example `dy ≥ y`) | ProgramValid |
| STATE | `STATE_READ_MISMATCH` | Witness pre-value ≠ authenticated cell | HeadFresh |
| | `STATE_EXTRA_WRITE` | Write not in `e` | EffectExact |
| EVID | `EVID_MISSING`, `EVID_STALE`, `EVID_STATUS`, `EVID_REPLAY` | Admission failures | EvidenceValid |
| | `EVID_CLASS_UNAVAILABLE` | Foreign atom needs a verifier mode not yet admitted | EvidenceValid |
| | `EVID_LABEL_FORGED` | Witness-chosen class | EvidenceValid |
| INT | `INT_FLOOR`, `INT_GROSS`, `INT_FEE`, `INT_RECIPIENT` | Signed budget violations | SignedBound/Ensures |
| | `INT_SURPLUS_MISALLOC` | §3.5 | SurplusAlloc |
| | `INT_FILL` | Hole bound or fixed-byte change | SignedBound |
| AUTH | `AUTH_RIGHT_MISSING`, `AUTH_EPOCH_STALE`, `AUTH_BUDGET`, `AUTH_WINDOW`, `AUTH_REPLAY` | | AuthorityFresh |
| CONS | `CONS_E1` | Conservation failure | Conserve |
| | `CONS_UNBACKED_MINT` | Positive supply without issue right or backing co-effect | Conserve |
| LIAB | `LIAB_L1`, `LIAB_UNFUNDED_DISCHARGE`, `LIAB_NO_CONSENT` | | LiabilityRoll |
| LOCK | `LOCK_AGGREGATE` | K1 breach on any debit | LocksSafe |
| FOOT | `FOOT_UNDECLARED`, `FOOT_ALIAS_UNRESOLVED`, `FOOT_LINEAR_DUP` | | FootprintSound |
| HIST | `HIST_PREDECESSOR`, `HIST_TOMBSTONE`, `HIST_ROLLFWD` | | HistoryLink |
| ESC/XDOM | `ESC_DOUBLE_TERMINAL` | | HistoryLink |
| | `ESC_RECOVERY_UNPREMISED` | | Admit |
| | `XDOM_PRIORITY` | X0 | Admit |
| | `XDOM_TIMEOUT_NOT_EVIDENCE` | X7 | Admit |
| | `XDOM_LATE_RECEIPT` | X3 | ProgramValid (dst) |
| | `XDOM_EARLY_TIMEOUT` | X2 with `clk_dst < deadline_D` | ProgramValid (dst) |
| VAULT | `VAULT_ZERO_OUT`, `VAULT_UNRECOGNIZED_LOSS`, `VAULT_BOOTSTRAP` | | ProgramValid |
| FAIL | `FAIL_PHASE_UNDECLARED` | Failure branch not in signed policy | FailurePolicy |

**[obligation O14]** Every code is reachable by some well-formed hostile witness, and no well-formed valid witness reaches any code. This is a differential in TypeScript and K, using one hostile control per code.

---

## 4. Eight-profile conformance matrix

**Conformance levels** (specification conformance, not a developer gate):
- **C0** specified;
- **C1** TypeScript/K differential on valid and hostile cases;
- **C2** U1 native certificate for every Κ op used;
- **C3** U2 accepted stage with ledger readback;
- **C4** U3 multi-stage lifecycle.

**Every row is currently C0 at most. [checked: nothing was run]**

| Profile | First slice (bounded) | Core constructs required | Κ ops | New/confirmed cells | Evidence | Earliest level possible | Mandatory hostile controls (envelope otherwise valid) | Deferred |
|---|---|---|---|---|---|---|---|---|
| **amm/cp1** | 1 pool, 1 hop, exact-in, fixed fee, 1 signer, no LP mint/burn | ProgramValid, SurplusAlloc, embedded-fee accounting, alias-closed footprint | `cp_swap_in/1` | `pool(id)` with version; reserve = custody balance cells | pool pre-state anchored at h | C3 (U2) | +1-unit over-delivery (`PROG_INVARIANT`); under-delivery skim (`PROG_QUOTE_NOT_TIGHT`); surplus to completer (`INT_SURPLUS_MISALLOC`); stale version; hidden LP fee; missing recipient cell | multi-hop, LP mint/burn, weighted, CL, n-party |
| **lend/loan1** | Fixed-rate, 1 debtor/creditor, originate, partial repay, discharge; **no oracle** | L1, consent, AccrualFirst, funded discharge, lock aggregate | `divmod_ceil/1` (accrual) | `obligation(id)`, `lockTotal(owner,asset)` | none | C3 | Unfunded discharge; forgiveness disguised as repay; double pledge; debit bypassing `lockTotal` | health factor (`price_cmp` reserved), liquidation, variable rates |
| **stable/cdp1** | 1 issuer, 1 collateral, fixed debt ceiling, **literal-price** collateral policy | Mint/burn effects, issue right, backing co-effect, lock aggregate | none (literal price is Φ₀) | `supply`, `debtCeiling`, `obligation`, `lockTotal` | none | C3 | Positive supply without backing (`CONS_UNBACKED_MINT`); ceiling race at stale head | oracle price CDP, redemption queue, shutdown, rebase |
| **deriv/call1** | Fully collateralized cash-settled European call, 1 fixing | Nominal instrument, write-once fixing, one-shot exercise, persistent settlement duty | `pos_part_scaled/1` | `instrument(id)`, `fixing(id)`, `exercise(id)` | anchored price round | C3 | Wrong round; double exercise; sign error; expiry erasing unpaid duty | margin, funding, perps, ADL |
| **oracle/obs1** | 1 anchored typed price, 1 signed threshold | Identity-keyed source sets, admission of all named observations, verifier-assigned labels | none | `observation(feedId)` with round/status | anchored | C3 | Forged class label; wrong unit; stale round; malformed obs in unused `or` branch | median/TWAP, imported, attested diversity |
| **gov/amend1** | 1 policy amendment leaving an existing obligation under its prior policy | PolicyHead, grant epoch, head freshness, policy pinning | none | `policy(id)` with version, `grant(id)` with epoch | anchored ledger reads | C3 | Revoke-before-execute; replayed approval; beneficiary change; paused repayment path | voting library, timelock queue, `suspend` right (D11) |
| **bridge/xfer1** | Midnight→Midnight, two contracts, one lock and one receipt/timeout | TransferClaim X1–X7, per-domain conservation, replay by claim id | `divmod_floor/1` for decimal ratio | `srcState`, `dstState` | anchored only | C4 (U3) | Source-clock refund (`XDOM_TIMEOUT_NOT_EVIDENCE`); early timeout; late receipt; replayed receipt; amount ratio drift | foreign verifier (U4), attested mode with bond, fast fill, reorg |
| **vault/v1** | 1 asset, 1 class, virtual offset, deposit + redeem only | Accounted `managed`, V1–V3, recognize with `reconcile`, role-fixed rounding | `vault_conv/1`, `divmod_*` | `managed`, `supplyShares`, `shares`, `bootstrap`, `remBeneficiary` | none | C3 | Custody-as-total read; forged quotient; zero-out deposit; unauthorized recognize; unrecognized loss | rewards, queues, restaking, strategies |

Notes:
- `bridge/xfer1` can at most reach **C4 under a same-ledger premise**. Calling it a "bridge" beyond that is [deferred] to U4.
- `stable/cdp1` with a literal price is a narrower claim than a CDP, per the stablecoin synthesis (`SYNTHESIS.md:221`).
- Two profiles are natural candidates for U2's required "structurally contrasting program" (`ROADMAP.md:23`): `vault/v1` (suggested in `opus55-staking_yield-recommendations/review-05.md:139`) and `lend/loan1`.

---

## 5. Transaction traces (expected results, not executed)

Common setup: domain `midnight.preview`, assets A and B at decimals 0 (fixture only), pool `P@v7` of profile `amm/cp1` with `x = 1000 A`, `y = 2200 B`, `φn/φd = 30/10000`. The intent is `AcquireB′`: `gross ≤ 11 A`, `fees ≤ 1 A`, `net ≥ 20 B to owner`, `surplus to owner`, `venues ⊆ {P}`, `hops = 1`.

**V1: valid AMM swap. Expected: ACCEPT.**
- Effects: `A owner→custody(P) 11`, `B custody(P)→owner 23`.
- `pre(pool) = (1000, 2200, v7)` at h, authenticated.
- `cp_swap_in/1`: INV(23) holds (22,008,751,590 ≥ 22,000,000,000), and TIGHT holds because INV(24) fails (21,998,641,920 < 22·10⁹).
- `fees = 0 + ⌈330/10000⌉ = 1 ≤ 1`; `gross = 11`; `D = 23`, `surplus = 3`, and all 23 go to the owner.
- `post(pool) = (1011, 2177, v8)`, and E1 holds per asset.

**H1: surplus skim, well-formed. Expected: REJECT `INT_SURPLUS_MISALLOC`.** Same as V1, but the effects credit `owner 20` and `solver 3`. Every other clause holds.

**H2: LP-collusion under-delivery, well-formed. Expected: REJECT `PROG_QUOTE_NOT_TIGHT`.**
- Pool pays `21`, and all 21 go to the owner.
- INV(21) holds, net 21 ≥ 20, and the surplus relative to D=21 is correctly allocated.
- TIGHT fails because INV(22) holds.
- *Under MIL/2 plus review-02's rule (no TIGHT), H2 is ACCEPTED.* That is the gap MIL/3 closes. Under review-02's rounded-fee rule, 21 is actually the *maximum*, so H2 is indistinguishable from V1 (see T1, counterexample 2).

**H3: over-delivery. Expected: REJECT `PROG_INVARIANT`.** The pool pays 24 and INV(24) fails.

**H4: stale head. Expected: REJECT `PROG_STALE_VERSION` / `STATE_READ_MISMATCH`.** The witness uses `(1000, 2200, v7)` after an accepted stage has moved the pool to `v8`.

**V2: same-ledger transfer timeout and refund. Expected: ACCEPT (two stages).**
1. `dst` stage at `clk_dst = deadline_D + 5`: `absent → timedOut` (X2).
2. `src` stage reads anchored `dstState = timedOut` and applies `locked → refunded` (X4), refunding the owner.

**H5: source-clock refund. Expected: REJECT at authoring, `XDOM_TIMEOUT_NOT_EVIDENCE`.** The intent's refund guard is `after(deadline_src)`. It is rejected even though the envelope is otherwise valid.

**H6: late receipt. Expected: REJECT `XDOM_LATE_RECEIPT`.** After V2, a relayer submits a valid source-lock proof to `dst`.

**H7: early timeout. Expected: REJECT `XDOM_EARLY_TIMEOUT`.** `dst` stage at `clk_dst = deadline_D − 1`.

**V3: vault donation made harmless. Expected: ACCEPT.** `vault/v1` with `virtualOffset(vS=1000, vA=1)`.
1. The attacker deposits 1 at `S=0, managed=0`: `s = ⌊1·1000/1⌋ = 1000`.
2. The attacker transfers `10⁶ A` into `custody` with no transition, so `managed` stays 1.
3. The victim deposits `10⁶`: `s = ⌊10⁶·2000/2⌋ = 10⁹`.

The donation is invisible to the conversion.

**H8: custody-as-total. Expected: REJECT `STATE_READ_MISMATCH`.** A prover witnesses `managed = 1,000,001`, which is the custody value, not the authenticated cell.

**H9: permissionless recognition. Expected: REJECT `AUTH_RIGHT_MISSING`.** A stage `recognize(V, 10⁶)` is signed by a party without `reconcile` on V.

---

## 6. Conflicts with MIL/2 and the reviews

I read "the five reviews" as the five per-lens Opus reviews behind each category synthesis, together with the syntheses themselves.

| # | Conflict | Sources | MIL/3 position |
|---|---|---|---|
| C1 | Φ₁ (pool arithmetic) at U4 vs certified arithmetic at U1 | `DESIGN-MIL2.md:89,158-165,350`; AMM/staking/lending/stablecoin/oracle syntheses | Split Κ from Φ₁. Φ₁ stays U4 |
| C2 | "less_than stops at 253" vs pinned `bits < 255` and a circuit bound of `2^{n+n mod 2}` | `DESIGN-MIL2.md:169`; `REVIEW-REPORT.md:96`; `zkir-v3-spec.md:497-498,862` | N1: explicit `CB`, even `n`, `N_max` measured |
| C3 | Review-02 and review-05 CPMM rules claimed equivalent | `opus55-amm-recommendations/SYNTHESIS.md:29` | False (21 vs 23). Decision D3 |
| C4 | Tightness framed as LP protection | same, `:27` | Tightness protects the owner. INV protects LPs |
| C5 | Same-head `priority` used for cross-domain races | `DESIGN-MIL2.md:206`; tex:192 | X0 forbids it. TransferClaim instead |
| C6 | Bridge review-03 makes `nonreceiptProved` a U0 Φ₀ atom | `opus55-bridges-recommendations/review-03.md:46` | Tag reserved at U0. Foreign instance inadmissible until U4 |
| C7 | Author-supplied `Rounding` | `DESIGN-MIL2.md:85-86` vs staking synthesis:241 | Deleted (N4) |
| C8 | `retained-in-pool` vs U0 default `protocol-reserve` (reserve absent) | `DESIGN-MIL2.md:89`; `numeric-profile.json` | Per-profile beneficiary, D8. Needs a U0 numeric-profile edit |
| C9 | `totalAssets` undefined; first-depositor allowed | `DESIGN-MIL2.md:83-89` | `managed` cell; bootstrap mandatory |
| C10 | E1 treated as sufficient | tex:171-176 | Necessary, not sufficient (V3/H8) |
| C11 | Showcase footprint omits recipient balance writes | tex:218-227; `DESIGN-MIL2.md:316-317` | FootprintSound alias-closed. Showcase must be rewritten |
| C12 | Signed total order on seizes vs ledger-version serialization vs origination priority | `DESIGN-MIL2.md:193`; lending synthesis:160; staking synthesis:254 | **Unresolved**, D10 |
| C13 | `UInt256` in U0 widths vs 255-bit field | `numeric-profile.json` | N2 limb rule applies. O9 |
| C14 | Pause as a ninth `suspend` right vs a policy parameter | governance synthesis:128 | **Unresolved**, D11. Tag reserved |
| C15 | Unbacked-mint bucket vs mandatory backing co-effect | stablecoin synthesis:220 | `CONS_UNBACKED_MINT` unless an explicit, native-visible unbacked mode. D12 |
| C16 | MIL/2 residue as the MEV answer vs Φ₀-enforceable surplus | `DESIGN-MIL2.md:324`; `review-03.md:155` | Surplus is core. Ordering exposure stays residue |
| C17 | Review-02 u126 vs review-05 u112 with φd ≤ 2¹⁴ | AMM synthesis:26 | Recommend u112 (D2); u126 as dissent |
| C18 | `fresh(price)` treated as sufficient | `DESIGN-MIL2.md:187`; lending synthesis:150 | Fixing and current-round selection belong in the profile. `fresh` alone does not choose a round |

---

## 7. Unresolved decisions, with recommendations and the evidence each needs

| ID | Decision | Recommendation (proposal-only) | Alternatives | Evidence to decide |
|---|---|---|---|---|
| D1 | Adopt the Κ stratum | Adopt | Wait for Φ₁ (MIL/2); widen Φ₀ with a narrow `price_cmp` | U1 certificate cost for `cp_swap_in/1`, and proof of S1–S3 (O7) |
| D2 | Κ width profile | u112 reserves, φd ≤ 2¹⁴ | u126 (review-02); two-limb u128 (review-01) | Measured constraint counts for each; survey of Midnight asset supplies vs 2¹¹² |
| D3 | CPMM fee placement | Exact rational fee inside INV (review-05, Uniswap-v2 form **[comparative]**) | Rounded fee first (review-02) | Economic owner decision on who owns fee-rounding value; per-trade loss bound at realistic decimals |
| D4 | Surplus default | No default; clause required | Default `to owner` | Wallet-UX review; digest canonical-form impact |
| D5 | Embedded fees under `fees ≤` | Count them | Separate `embedded_fees ≤` clause | Owner decision (review-05's stop rule, `:105`) |
| D6 | Attested bridge mode | Admit only with a declared premise and a contingent loss obligation | Forbid until U4 | Named loss-bearer; bond sizing is library |
| D7 | Recognition authority | `reconcile` right | New right; permissionless with vesting | Front-running analysis of recognition events |
| D8 | Remainder beneficiary | Per-profile, immutable, declared | Global `protocol-reserve` (currently absent) | U0 numeric-profile amendment; reserve mechanism design |
| D9 | Bootstrap | Virtual offset required, δ in library | Dead shares | Attack-cost bound for the chosen δ; the offset's value-leak rate (T4, counterexample 3) |
| D10 | Seize ordering | Ledger-version serialization for permissionless keepers, with signed priority as policy | Signed total order (MIL/2) | Double-seize and concurrent-keeper traces |
| D11 | Pause | Policy parameter with unpausable remedies; reserve the `suspend` tag | Ninth right | Proof that recovery and debt duties survive pause |
| D12 | Unbacked mint | Explicit unbacked mode visible in the public statement | Forbid | Issuer-class taxonomy |
| D13 | `price_cmp/1` admission | Reserved, not in profile 1 | Admit at U1 | Width and cost; a health-comparison hostile corpus |

### Obligations

MIL/2 O1–O6 are **all retained unchanged** (tex:247-252: totality, non-laundering, lock invariant, C1 refinement, recovery, footprints). New obligations:
- **O7:** stratum confinement.
- **O8:** measured `N_max` at the pinned tuple.
- **O9:** U0 width-list reconciliation.
- **O10:** reporting-order independence.
- **O11:** TransferClaim exclusivity per evidence class.
- **O12:** vault invariants V1–V3 preserved by every transition.
- **O13:** no-free-lunch.
- **O14:** rejection-code reachability and valid-witness non-reachability.
- **O15:** `TIGHT` plus `SurplusAlloc` implies that the owner's credit equals the program maximum minus the signed completer share.
- **O16:** every Κ op has soundness over ℤ, not just over the field.

### U0 obligations preserved, unchanged

All U0 obligations remain as recorded in `deliverables/u0-semantic-contract-2026-09-23/EXIT-GATE.md:7-14`:
- embeddings: 1 present, 17 partial, 66 absent;
- 6 judgments with incomplete realization;
- numeric profile: 6 open gaps, reserve absent;
- K reconciliation: 0 covered, 5 partial, 18 not covered;
- target pins: 10 unresolved;
- enforcement map: 0 of 84 enforced;
- trust premises: 6 open;
- backend requirements: 24 specified-only.

**U1/U2 native gates** stay exactly as in `ROADMAP.md:22-23`. MIL/3 adds Κ certificates to U1's "every primitive needed by the initial slice". It weakens no gate.

---

## 8. Migration from MIL/2

MIL/2 was never frozen or hash-bound (`DESIGN-MIL2.md:4,365`), so no deployed MIL/2 digest exists. Migration is a document change. There is no dual-acceptance period.

| MIL/2 construct | MIL/3 | Mechanical? |
|---|---|---|
| `version moriarty-intent/2` | `moriarty-intent/3`. The /2 header rejects with `ADM_VERSION` | yes |
| `sharesFor/assetsFor(…, Rounding)` | Removed from terms. Replaced by `vault_conv/1` in program transitions | no, needs a program |
| `mulDiv` (Φ₁) | Remains reserved Φ₁; Κ `divmod_*` for programs | no |
| Escrow with cross-domain release | `TransferClaim` | no |
| `priority` on cross-domain escrow | `XDOM_PRIORITY` | — |
| Residue "ordering and MEV" | Kept, plus a mandatory `surplus` clause | yes, the author must choose |
| `budget { fees <= … }` | Semantics widened to embedded fees (D5) | yes |
| Showcase footprint | Adds recipient balance cells, or `escrow(E)` defined as expanding to custody cells | yes |
| `hole amount : Qty ! {anchored@d}` | Unchanged; source sets become identity-keyed | yes |
| §12 caps | Add `Κ ops per stage` (proposed 4) and `Κ products per op` (proposed 4) | yes |
| §15 milestones | Κ names U0, certificates U1, first Κ stage U2; Φ₁ unchanged U4 | — |

The canonical encoding changes, so every MIL/2 test-vector digest is invalidated.

---

## 9. Falsification experiments

Each experiment names what would refute the corresponding recommendation.

1. **F-width:** Build a ZKIR fragment with `LessThan(a,b,253)` and no `ConstrainBits`, then look for a satisfying witness with `a ≥ 2²⁵³`. If none exists at the pinned tuple, N1's odd-`n` concern is refuted and the rule can relax.
2. **F-equiv:** Evaluate both CPMM rules on the V1 fixture. If both give 23, counterexample 2 in T1 is wrong. (My hand arithmetic says 21 vs 23.)
3. **F-skim:** Encode MIL/2 + tex `Stage` and H2 in K or TypeScript. If H2 is rejected without TIGHT, C4 is wrong.
4. **F-TIGHT cost:** Measure the extra constraints of `¬INV(dy+1)`. If they exceed the U1 budget, the alternative is a signed `dy = Q` equality from witnessed divmod (review-02/04), with an equivalence proof.
5. **F-vault:** Exhaustively check V1–V3 and O13 at 8-bit widths over all four operations plus recognize/writeDown. Any counterexample refutes §3.7.
6. **F-offset leak:** Compute the fraction of recognized yield captured by virtual shares for `vS ∈ {1, 10³, 10⁶}` against realistic `S`. If it is material, D9 must prefer dead shares.
7. **F-bridge:** Model X1–X7 in Quint or TLA+ with an adversarial relay and clock skew, and check exclusivity for anchored evidence. Expect a counterexample in attested mode, which confirms the premise scoping.
8. **F-lock:** Search for any debit path in the effect grammar that does not touch `lockTotal`. Finding one refutes `LOCK_AGGREGATE` completeness.
9. **F-codes (O14):** Produce one well-formed hostile witness per code. Any unreachable code is dead or mis-specified.

---

## 10. Repository anchors and primary URLs

**Repository** (worktree root, baseline `983a4bb4`):
- `concepts/intent-language/DESIGN-MIL2.md:8-14, 83-89, 100-102, 124, 158-171, 187, 191-193, 199-219, 239-250, 256, 285, 289-325, 332-338, 345-351, 365`
- `deliverables/mil2-deep-research-2026-09-29/MIL2-PROPOSED-SEMANTICS.tex:83, 95, 106, 109-127, 131-138, 171-187, 192-205, 209-237, 247-252`
- The eight syntheses in `deliverables/mil2-deep-research-2026-09-29/opus55-*/SYNTHESIS.md`, as cited inline, plus `opus55-amm-recommendations/review-02.md:17-60`, `review-03.md:11-34,155,162`, `review-05.md:44-60,83-105,195-205`; `opus55-bridges-recommendations/review-03.md:13,39-46,82-89,127`; `opus55-staking_yield-recommendations/review-01.md:71-83`, `review-05.md:10,42-57,139`
- Category reviews: `deliverables/mil2-deep-research-2026-09-29/category-review/01-…08-*.md`. I consulted these through the syntheses and did not re-audit them line by line.
- `deliverables/mil2-deep-research-2026-09-29/source-text/zkir-v3-spec.md:79-80, 296, 472, 497-498, 583, 855-866, 984-996`
- `deliverables/mil2-deep-research-2026-09-29/source-text/erc4626.md:631-636`; `source-text/ibc-ics004.md:308-335`
- `deliverables/u0-semantic-contract-2026-09-23/EXIT-GATE.md:3-21`; `numeric-profile.json` (`defaultPolicy`, `reserveMechanism`, `units.widths`)
- `ROADMAP.md:15, 21-28, 34, 38`; `docs/MORIARTY-PRODUCT-CONTRACT.md:47, 68`
- `concepts/intent-language/review/REVIEW-REPORT.md:96`
- `plugins/moriarty-dev/skills/develop/SKILL.md` (permissionless boundary)

**Primary external sources** (all [comparative]; captures listed in `deliverables/mil2-deep-research-2026-09-29/sources.json`):
- ZKIR v3 spec, pinned `47793c8`: https://raw.githubusercontent.com/midnightntwrk/midnight-zkir/47793c8ab042aa5a91d1a4672c6b82de6bdf9dd8/zkir-spec/docs/zkir-v3-spec.md. This is Midnight's own target spec; the Moriarty evidence gap is that it has not been *re-verified* against the pinned tuple.
- ERC-4626: https://eips.ethereum.org/EIPS/eip-4626
- IBC ICS-004: https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md; IBC v2 packet handler: https://github.com/cosmos/ibc/blob/main/spec/IBC_V2/core/ics-004-packet-semantics/PACKET_HANDLER.md
- Uniswap v2 pair: https://github.com/Uniswap/v2-core/blob/master/contracts/UniswapV2Pair.sol. This is cited through the AMM synthesis and is not in `sources.json`, so it has **no retained capture**.
- SMT-LIB logics: https://smt-lib.org/logics-all.shtml
- Nomos: https://arxiv.org/abs/1902.06056

---

**Summary of the candidate.**
- **Three strata:** Φ₀ for signers, a closed certified Κ list for programs, and Φ₁ still deferred to U4.
- **No unallocated value:** tight quote plus signed surplus; destination-decided transfer outcome with exclusivity scoped to the evidence class; accounted vault totals with authorized recognition and a declared remainder beneficiary.
- **Width discipline:** explicit `ConstrainBits` per the pinned ZKIR text, not the "253" figure.

Three findings refute parts of the prior material:
1. The two CPMM rules are not equivalent: they give 21 vs 23 B on the shared fixture.
2. Tightness protects the owner, not the LPs.
3. MIL/2's 253-bit `less_than` premise does not match the pinned spec.

Everything above is proposal-only, and none of it has been executed or verified natively.