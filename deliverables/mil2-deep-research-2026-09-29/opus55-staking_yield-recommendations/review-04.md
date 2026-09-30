Still working: I've read the design, the category review, the semantics draft, ZKIR v3 and ERC-4626, and I'm writing up the recommendation now.

# Staking, restaking and yield: recommendation through the compiler and native-feasibility lens

**Status.** This is read-only design review of a specified-only proposal. Nothing here claims implementation, proof or ledger acceptance. `cli.py status` reports SP01.6 blocked on stale inputs. That blocks its dispatch, not this review.

## 1. Verdict on design fitness

The carriers are there, but a narrow share vault is **not yet compilable**. Three gaps block it:

- **Conversion sits in the wrong layer.** `sharesFor` and `assetsFor` are defined as Φ₁ *predicate* terms (`concepts/intent-language/DESIGN-MIL2.md:85-89`), and Φ₁ is deferred to U4 (`DESIGN-MIL2.md:350`). The semantics draft also leaves vault conversion out of the first profile (`MIL2-PROPOSED-SEMANTICS.tex:59`).
- **Rounding is a free parameter the author passes in** (`DESIGN-MIL2.md:85-86`). The U0 numeric profile, by contrast, must fix rounding direction and beneficiary for each primitive (`ROADMAP.md:21`).
- **Shares fall outside conservation.** E1 ranges over `balance` cells (`DESIGN-MIL2.md:256`; `.tex:171-176`). Shares live in separate `shares(pool,class,acct)` cells (`DESIGN-MIL2.md:241`), so no law governs share mint or burn. The category review's "K14 covered" (`08-staking-yield.md:34`) therefore overstates coverage for shares.

The deferral rests on a cost argument that I think answers the wrong question (see §5). Once conversion becomes a certified effect primitive that is checked for each witness, the smallest vault fits U1/U2 without admitting Φ₁ into intents.

## 2. Five ranked design edits (recommendations)

### E1. Make conversion a certified Core primitive with a witnessed quotient and remainder

**Rule sketch.** `mulDiv_ρ(x:u128, y:u128, d:u128) → u128 | Reject`, where ρ ∈ {floor, ceil}.

- Private witnesses: `q` and `r`.
- In-circuit constraints:
  - `0 < d`, checked with `LessThan(0,d,128)`, not `Inv`.
  - `x·y = q·d + r`, checked with a declared two-limb gadget (64-bit halves, carried partial products).
  - `r < d`, checked with `LessThan(r,d,128)`.
  - `ConstrainBits(q,128)` and `ConstrainBits(r,128)`.
- Ceiling: `q' = q + [r≠0]`. It rejects `ARITH_RANGE` if `q' ≥ 2^128`.
- Every result bit passes `constrain_to_boolean` before `Assert` (`DESIGN-MIL2.md:265`).

ZKIR v3 has no general integer division. It offers only `DivModPowerOfTwo`, `LessThan` and `ConstrainBits`, and `LessThan` needs `bits < FR_BITS` (`source-text/zkir-v3-spec.md:461,472,498-501`). This construction therefore follows the limb rule (`DESIGN-MIL2.md:169-171`) and uses only range checks.

**Counterexamples.**
- Take x=500, y=1999, d=1000. A hostile prover supplies q=998, r=1500. That satisfies `q·d + r = 999,500` but mints the depositor one share fewer. Only `r<d` rejects it.
- A single native `Mul` of two values near 2^128 wraps modulo the 255-bit field. That wrap is silent coercion.

**Placement.** The U0 numeric profile names the primitive, its rejections and its rounding roles. U1 produces its native certificate, which is a natural candidate for "the first exact arithmetic primitive" (`ROADMAP.md:22`). The rule enters a proposed `vault/1` profile.

### E2. Direct rounding by role, and remove the free `Rounding` argument

**Rule.** The operation fixes the rounding direction:

| Operation | Rounding |
|---|---|
| `deposit` (shares out) | floor |
| `mint` (assets in) | ceil |
| `withdraw` (shares burned) | ceil |
| `redeem` (assets out) | floor |

The pool always benefits, and the residue stays in the pool's managed assets. A fee is computed on its gross base and rounded up in the fee recipient's favour. Its residue is also pool-retained. The external practice is ERC-4626: `convertTo*` MUST round down (`source-text/erc4626.md:124-166`), and previews carry directional bounds (`erc4626.md:210-220, 293-303, 374-384`). The table above is a proposed Moriarty rule; it is not text from ERC-4626.

**Counterexample.** A pool holds A=10 and S=3. Redeeming 1 share with author-selected `ceil` pays 4. Three such redemptions request 12 from a pool of 10, and the loss falls on other holders.

**Placement.** U0 numeric profile, which `ROADMAP.md:21` already requires.

### E3. Authenticate the pool totals, separate from custody balance

**Rule.** Specify the `pool(P)` cell (`DESIGN-MIL2.md:242`) as the record `{managed: Qty<A>, supply[C]: Qty<Share(P,C)>, offsets (V_s,V_a), policyDigest}`.

- Conversion reads `managed` and `supply` **only** through the authenticated ledger read at the predecessor head.
- Invariant: `managed ≤ balance(d, custody(P), A)`.
- Only vault effects write `managed`. An unsolicited transfer raises the custody balance but not `managed`, so it cannot move the price.

**Counterexample.** The first depositor deposits 1 unit and receives 1 share, then donates 10⁶ units. If `totalAssets` is read from custody balance, the next deposit of 999,999 receives `floor(999,999·1/1,000,001) = 0` shares.

**Placement.** U0 fixes the cell layout and the public-input field. U2 enforces it.

### E4. Bring shares under E1 and scope minting through `issue`

**Rule.**
- `Share(P,C)` elaborates to a nominal derived AssetId, `shareAsset(P,C)`, and `shares(P,C,acct)` is its balance cell. Then E1 gives `Σ_acct Δshares = Δsupply[C]`.
- Mint and burn require `issue` scoped to `(d, shareAsset(P,C))`, held by the pool program (`DESIGN-MIL2.md:193`).
- A deposit that mints 0 shares, or a redeem that pays 0 assets, is rejected with `Reject(ZERO_OUT)`.
- Shares are supply, not debt. L1 does not apply to them, which keeps debt and supply separate.

**Counterexample.** Under today's text, a stage can credit `shares(P,C,alice)` without changing `totalShares`. No stated law rejects it, because E1 does not quantify over share cells.

**Placement.** This is a **U0 boundary change.** U0 freezes the cell vocabulary, asset identity and "conservation as the general law" (`DESIGN-MIL2.md:344-345`). Retrofitting a second conservation law after the stage public-input schema is hash-bound would change the digest. Reserving it now costs one derivation rule.

### E5. Define the vault operation as a complete-effect stage with a pre-state snapshot and one bootstrap rule

**Rule sketch (deposit).**
```
s = mulDiv_floor(a, pre.supply[C] + V_s, pre.managed + V_a)
s > 0
effects:
  transfer(d, A, dep → custody(P), a)
  managed += a
  shares(P,C,dep) += s
  supply[C] += s
```
- The derived footprint is {`balance(d,dep,A)`, `balance(d,custody,A)`, `pool(P)`, `shares(P,C,dep)`, `policy(P)`, `replay`}. That is 6 cells against the 32-cell cap (`DESIGN-MIL2.md:281`), and `declared ⊇ derived` must hold (`DESIGN-MIL2.md:249`).
- Every conversion reads `pre(pool)`, and `post = pre + deltas`.
- The intent binds only Φ₀ atoms: `shares_out ≥ s_min`, `gross_debit ≤ a_max` and `fees ≤ f_max`. The signer never evaluates Φ₁.
- **Bootstrap:** the core rule is virtual offsets with `V_a ≥ 1`. Then `d > 0` holds by construction, and the zero-supply branch (`DESIGN-MIL2.md:89`) becomes unreachable instead of a reject path.
- A first-depositor convention remains a library policy: a seeded stage the pool must accept first.

**Counterexample.** Suppose conversion reads `post(pool)` after crediting `managed`. The depositor's own assets then sit in the divisor, and they receive fewer shares.

**Placement.** U2 as one single-stage contrasting program (`ROADMAP.md:23`). The full ERC-4626-style family goes to U6.

## 3. Core versus library boundary

**Core (proposed):**
- `mulDiv_ρ` with its certificate
- the rounding-role table
- the `pool(P)` cell and its authenticated read
- `shareAsset` identity under E1 and `issue`
- the snapshot rule and `ZERO_OUT` rejection
- offset bootstrap

These are the pieces whose error would silently break conservation or native soundness.

**Library (proposed):**
- `max*` and `preview*` as advisory views, with preview ≤/≥ bounds proved as library lemmas
- fee models
- reward indices and emissions (K7)
- rebase adapters (K6)
- withdrawal queue and rate freeze, built on escrow (§7) and receipts (K12)
- restaking allocation and slash waterfall, built on encumbrances (K9/K10)
- strategy mandates (K16)
- principal/yield split (K15)

Queue, slash and validator events still need their own semantics, as the category review lists (`08-staking-yield.md:26-39`). None of them needs new *arithmetic* beyond E1.

## 4. Smallest implementable slice

**Scope:** one domain, one signer, one asset, one share class, `deposit` only. No fees. `V_s = V_a = 1`.

**Positive control.**
- Pre-state: `managed=999`, `supply=1998`.
- Deposit a=500, giving x=500, y=1999, d=1000. Then x·y = 999,500 = 999·1000 + 500, so q=999 and r=500.
- Intent: `shares_out ≥ 998`, `gross_debit ≤ 500 A`.
- Expected post-state: `managed=1499`, `supply=2997`, depositor shares +999, depositor A −500, custody A +500.
- Native verification should accept, and the read-back effects should match exactly.

**Hostile control (well-formed envelope, same intent).**
- Witness q=998, r=1500. It satisfies the product equation and the intent bound (998 ≥ 998).
- It must reject **only** at `LessThan(r,d)`.
- This follows `ROADMAP.md:40`: the rejection must come from the semantic check, not from a malformed envelope.

**Companion pair for the limb gadget.**
- Positive: x=y=d=2^128−1 gives q=2^128−1, r=0.
- Hostile: a lowering that emits native `Mul` must fail certificate review.

A further hostile control is a host-supplied `accepted=true` flag with a wrong q. It must reject, because no host Boolean counts toward acceptance (`.tex:130`).

## 5. Explicit disagreements

1. **With the Φ₁ deferral rationale** (`DESIGN-MIL2.md:163`, placement at `:350`). The rlimit 242,607,369 measurement prices *universal authoring validity* over bitvectors. Stage acceptance is checked for each concrete filling (`DESIGN-MIL2.md:232`), and checking one division needs roughly four range checks and a limb product. **Proposed change:** keep Φ₁ deferred for *predicates*, and move conversion into a U1-certified effect primitive. This does not change the Φ₁ boundary itself.
2. **With the category review's K4 placement** (`08-staking-yield.md:24`, which leaves the fix to "the library contract"). Rounding direction belongs in the core numeric profile, as `ROADMAP.md:21` requires.
3. **With the equal standing of first-depositor and virtual offset** (`DESIGN-MIL2.md:89`). Only the offset makes the divisor nonzero by construction, which removes a reject path from the certified primitive.
4. **With the category review's "K14 covered"** (`08-staking-yield.md:34`). Share supply is outside E1 as written.

## 6. Residual assumptions

- Circuit cost of the limb gadget, the `LessThan` checks and the 128-bit range checks is **unmeasured**. The §12 caps are proposals (`DESIGN-MIL2.md:270`).
- The pinned Midnight ledger is assumed to expose an authenticated read of a contract cell holding `managed` and `supply` at the predecessor head. This is unverified.
- The ZKIR v3 spec is a working draft (`zkir-v3-spec.md:2`). Statement soundness depends on producer and witness-shape premises (`.tex:240`).
- Virtual offsets reduce, but do not eliminate, rounding extraction. The safety bound on `V_s` has to be proved.
- ERC-4626 (`https://eips.ethereum.org/EIPS/eip-4626`, captured at `source-text/erc4626.md`) is comparative external practice, not a Moriarty rule.
- Restaking, queues and validator evidence stay open. This lens did not resolve them.