# Staking, restaking and yield: recommendation from the adversarial-controls and release lens

**Status.** This is a read-only design recommendation. Nothing here is implemented, proved or accepted by a ledger. `cli.py status` reports SP01.6 as blocked by operational-history evidence gaps. That does not affect this review.

## 1. Verdict

**Recommendation: the category is fit as a substrate but not fit for release.** MIL/2 has the right carriers: pool/class share identity, both conversion directions, a named zero-supply branch, multi-obligation encumbrances, and a signed seize order (`DESIGN-MIL2.md:83-102`, `:193`). What it lacks is the rules that stop an adversary extracting value. Five gaps matter most:

- **Rounding is a free argument.** `sharesFor`/`assetsFor` take `Rounding` as a parameter (`:85-86`), so the author, not the operation, picks the direction.
- **Donations can move the price.** "First-depositor convention" is allowed (`:89`), and the design does not say whether `totalAssets` means the raw custody balance or an internally accounted figure. If it is the raw balance, anyone can move it by sending tokens in.
- **There are no reward-index or checkpoint cells** in the frozen U0 vocabulary (`:239-242`).
- **Duplicate slash evidence is not prevented.** The signed seize order (`:193`) makes competing seizes run one at a time, but the same evidence can still be applied twice.
- **Queue and strategy are absent.** There is no queue rate-freeze point and no strategy mandate (`08-staking-yield.md:32`, `:36`).

A deeper problem is that §4.4 places conversion in Φ₁ next to variable products, on the basis of an SMT *authoring* cost (`:163`). But in a stage transition, conversion is *evaluation* of a checked division witness. It is not *decision* of a quantified predicate. Grouping the two pushes every vault out to U4 (`:350`) for a cost that does not apply to it.

## 2. Five ranked design edits

### E1 (highest). A certified divmod primitive, with the rounding direction fixed by the operation

**Rule (recommendation):**
- Remove `Rounding` from the user-visible signature. Conversion becomes `convert(op, x, num, den)` with `op ∈ {deposit, mint, withdraw, redeem, accrue, fee}`.
- A fixed table in the numeric profile gives each operation its direction, and the pool always benefits:
  - deposit and redeem round **down**;
  - mint and withdraw round **up**;
  - accrue rounds **down**, with dust staying in the pool;
  - fee rounds **down** on the fee recipient's side.
- The witness `(q, r)` must satisfy `q·den + r = x·num` computed in two limbs (§4.5, `:171`), and `0 ≤ r < den`.
- Rounding up is `q + [r > 0]`.
- `den = 0` gives `Reject(DIV_ZERO)`. Any deposit or mint that yields `q = 0` gives `Reject(ZERO_OUT)`.
- **Stage invariant for every user operation (pps monotone):** `post(A)·pre(S) ≥ pre(A)·post(S)`, checked with the same limb gadget.

**Counterexample (current design):** take `A = 1,000`, `S = 3,000`. A redeem signed with `Rounding = up` pays 1 asset per share. Repeating it 1,000 times drains the pool while burning only 1,000 of the 3,000 shares. The type system accepts this today.

**External practice:** ERC-4626 requires previews to round in the vault's favour (`source-text/erc4626.md:136, 214, 297, 378, 466`; https://eips.ethereum.org/EIPS/eip-4626). That is Ethereum practice, not a Moriarty rule.

**Placement:**
- **U0:** the direction table and the operation enum go into the numeric profile. **This changes the U0 boundary.** U0's exit already requires "per-primitive rounding direction and beneficiary policy" (`ROADMAP.md:21`), and unknown tags are rejected after freeze (`DESIGN-MIL2.md:285`).
- **U1:** the divmod certificate, meaning the first exact arithmetic primitive (`ROADMAP.md:22`).
- **Profile:** `vault-conv/1` as an effect-layer primitive. Φ₁-in-predicates stays at U4.

### E2. Accounted assets and a mandatory virtual offset (inflation and donation)

**Rule (recommendation):**
- `totalAssets(p)` reads a new accounted cell, `pool(p).managed`. Only typed pool effects write it: deposit, redeem, accrue, slash, harvest.
- Custody invariant: `balance(d, custody_p, a) ≥ managed`.
- An unrequested transfer into custody becomes `surplus`. Only a declared `harvest` transition, under mandate (E5), can move it into `managed`.
- Bootstrap must use a virtual offset `(vA = 1, vS = 10^δ)` with δ ≥ 3. Otherwise it must use dead shares minted to an unspendable holder, with a declared minimum. A bare first-depositor rule is rejected.
- Signed deposit intents carry a `min_shares_out` bound. This is Φ₀, with a literal or hole bound.

**Counterexample:**
1. The attacker deposits 1 unit and receives 1 share.
2. They donate 10⁶ units straight to custody.
3. A victim deposits 2·10⁶ − 1 and receives ⌊(2·10⁶−1)/(10⁶+1)⌋ = 1 share.
4. The attacker redeems 1 of the 2 shares and takes about 5·10⁵ units from the victim.

Every step satisfies E1 conservation (`MIL2-PROPOSED-SEMANTICS.tex:171-176`). The fix works in two ways. Accounted assets make the donation invisible to the conversion. The offset makes the attack cost roughly 10^δ times its gain.

**Placement:** reserve the `pool.managed`/`surplus` fields and the bootstrap enum in the U0 cell vocabulary. Enforce them in the U2 slice. The full library comes at U6.

### E3. Reward index with checkpoint-before-change (duplicate rewards)

**Rule (recommendation):**
- Add two cells: `index(p, c)` and `checkpoint(p, c, acct)`.
- `accrue(p, c, R)` needs one of two authorities:
  - `issue` on the reward asset, for emissions (`DESIGN-MIL2.md:193`);
  - a funded transfer into `reserve_p` that sits inside the E1 set, for yield.
- The accrue update is `index' = index + convert(accrue, R, SCALE, S)`. The index is monotone.
- **Settlement is mandatory.** Any effect that writes `shares(p, c, acct)` must settle in the same stage, first:
  - `owed = convert(accrue, shares, index − ckpt, SCALE)`;
  - then `ckpt := index`.
- A newly created checkpoint cell starts at the current index, **not** at 0.
- Reserve invariant: Σ claimed ≤ Σ funded.
- Derived footprints (`:249`) must contain the checkpoint of every account whose shares are written.

**Counterexample A:** Alice claims, then transfers her shares to a fresh account B whose checkpoint defaults to 0. B claims the whole index history again.

**Counterexample B:** a deposit and an accrue land in one stage, and the deposit is applied first. The depositor captures rewards earned before they held shares.

**Placement:** the cells are **U0**, because the frozen vocabulary is at `:345`. This is the same boundary change as in E1, and the reason is that tags cannot be added after freeze without migration. The rule comes at U3/U6, since streaming needs persistent continuations.

### E4. Replay-keyed, budgeted slashing, with requests staying slashable while queued (double slash and stale queue)

**Rule (recommendation):**
- `slash(ev, op, svc, f)` consumes `replay(slash‖digest(ev))`, with the digest taken over the canonical evidence.
- The seizure is limited to `alloc(op, svc)`, and it reduces both `alloc` and the encumbrance.
- Across services, Σ seize ≤ Σ alloc ≤ stake, using the K1 lock invariant (`MIL2-PROPOSED-SEMANTICS.tex:229-236`).
- Loss reduces `pool.managed`, so every share class that is still slashable bears it pro rata.
- A queued withdrawal request `w` keeps its shares locked and slashable until `finalize`. That is allowed only when `after(req_t + Δ_slash)`, where `Δ_slash ≥` the declared evidence latency.
- The payout is frozen at finalization: `min(convert(redeem, s_w, A_req, S_req), convert(redeem, s_w, A_fin, S_fin))`. Holders who exit take losses but not later gains. This follows the R8 anchor `lido.qnt:71-138` (see R8:377).
- Queue order is FIFO by request identifier. A partial batch finalizes a prefix.
- A foreign verdict is `imported` evidence: a named trust premise until U4 (`DESIGN-MIL2.md:187`).

**Counterexample (double slash):** one offence is submitted as two evidence envelopes. The signed seize order (`:193`) simply puts them in sequence and both apply.

**Counterexample (stale queue):** an operator who sees a slash coming requests a withdrawal at the pre-slash rate and claims before the slash lands. The loss moves to the holders who stayed.

**Placement:** replay keys and `alloc` cells at U0. Queue and finalize at U3, because they are a continuation plus a race (`ROADMAP.md:24`). AVS allocation belongs to U6 libraries, and an external verdict to U4/U5.

### E5. Strategy mandate as a digest-bound policy with timelocked amendment (strategy drift)

**Rule (recommendation):**
- `policy(m)` stores the mandate: venues, per-venue cap in basis points, loss rule, fee basis, and high-water mark (HWM).
- Deposit intents sign `digest(m)`.
- Each strategy stage checks per-venue caps as `post(alloc_v) × 10000 ≤ post(managed) × cap_v`. **This is Φ₀**: cross-multiplication by a literal (`:160`).
- Writing `policy(m)` needs `amend`, with an effective time of at least now + max queue duration, so depositors can leave first.
- Stages under the old digest are rejected once the new one takes effect.
- The performance fee mints shares, using the `issue` right on share supply, and only on accounted gain above the HWM.
- Realized loss lowers `managed`. Strategy borrowing is an `Obligation` and never a supply change (`MORIARTY-PRODUCT-CONTRACT.md:43`).

**Counterexample:** a manager amends the venue list in the middle of an episode, then harvests an imported "gain" that exists only on paper. A fee is minted on it, the later loss falls on depositors, and today nothing binds the policy cell's value to the intent.

**Placement:** the digest binding and the `amend` timelock at U2/U3. Mandate libraries at U6.

## 3. Core versus library (recommendation)

**Core:**
- the divmod primitive and the direction table (E1);
- share-supply conservation, Σ_acct shares = totalShares, as the share-level analogue of E1;
- the `pool.managed`/`surplus` split;
- index and checkpoint cells, with settle-before-write as a derived-footprint rule;
- replay-keyed consumption for evidence;
- the K1 lock invariant;
- `amend` timelock semantics;
- the rate-freeze min() on async redemption.

**Library:**
- the ERC-4626-shaped interface and its preview bounds;
- fee schedules;
- emission curves;
- LST queue policy parameters;
- AVS allocation and adjudication;
- principal/yield (PT/YT) splits;
- rebase-to-index adapters;
- strategy templates.

## 4. Smallest implementable slice

**Recommendation.** `vault-conv/1` on one Midnight domain: one asset, one class, a virtual offset with δ = 3 (vA = 1, vS = 1000), deposit and redeem only, with no rewards, fees or queue. It needs only the E1 certificate and the E2 cells, and could serve as U2's "structurally contrasting program" (`ROADMAP.md:23`).

**Positive control (a feasible valid witness):**
1. From `A = S = 0`, deposit 1,000 units.
2. `q = ⌊1000·(0+1000)/(0+1)⌋ = 1,000,000` shares, with `r = 0`.
3. Then redeem 1,000,000 shares: `⌊10⁶·1001 / 1,001,000⌋ = 1000`, with `r = 0`.
4. This returns exactly 1,000. E1 holds, pps is monotone, and the readback matches.

**Hostile control (a well-formed envelope, rejected on semantics):**
1. From the post-deposit state, redeem 1 share while claiming `q = 1`.
2. This needs `r = 1001 − 1,001,000 < 0`. A prover can supply the field-wrapped `r = p − 999,999`.
3. The range check `r < den` must reject it.

**Second hostile control:** the same stage with direction `up` on redeem. It must fail the operation-to-direction table, not an envelope check. This meets the roadmap's rule that a malformed envelope does not count as a semantic rejection (`ROADMAP.md:40`).

## 5. Explicit disagreements

1. **With §4.4 (`:160-163`).** Checking a conversion witness in a stage is linear once the product is split into limbs, and it needs no SMT. The 242M-rlimit figure measures authoring-time decision. Placing conversion in Φ₁ delays every vault to U4 without cause.
2. **With §3.3 (`:85-89`).** A free `Rounding` argument and a bare first-depositor rule should not be admitted. Both are extraction vectors.
3. **With §6 (`:193`).** Ordering is not idempotence. Seizes also need evidence-keyed replay.
4. **With the category review's K5/K7 framing (`08-staking-yield.md:25, 27`)** and R8 K6. The index and checkpoint belong in core, because duplicate-reward safety is a footprint property. A rebase with no per-holder line (K6) should *not* be core. Express it through the index plus a library adapter, so that every balance change stays an explicit effect under E1.
5. **With the cell vocabulary (`:239-242`)** as a frozen U0 set. It is missing `index`, `checkpoint`, `pool.managed` and `alloc`.

## 6. Residual assumptions

- A two-limb divmod plus range check fits within ZKIRv3 cost caps. This is unmeasured (`:270`).
- δ = 3 is a proposal. Its safety margin depends on asset decimals.
- `Δ_slash` depends on an evidence-latency bound that holds only under named liveness premises (`:219`).
- An external slash verdict or yield report stays `imported` until U4.
- Ledger phase failure can retain fees (`MORIARTY-PRODUCT-CONTRACT.md:47`), so settle-before-write must hold in the guaranteed phase.
- ERC-4626 and Lido behaviour are external practice used for comparison.
- None of E1–E5 has an executed differential, a K rule or a native certificate.