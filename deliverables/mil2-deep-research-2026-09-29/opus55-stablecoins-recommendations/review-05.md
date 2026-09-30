# Stablecoins and synthetic assets: adversarial controls and release plan (independent recommendation)

**Status.** This is a read-only design review. It is specified only: nothing was compiled, proved, tested or submitted to a ledger. Startup status was run: the SP01 dispatch is blocked by stale inputs, which does not block this review. Every recommendation below is labelled as one.

## 1. Verdict

**Repository observation.** MIL/2 has the right pieces for issuance:
- nominal `AssetId` (`concepts/intent-language/DESIGN-MIL2.md:53-61`)
- a separate `issue` right scoped to `(domain, asset)` (`:191-193`)
- general conservation (`:256`)
- evidence carried as a type index (`:175-187`)
- the supply cell (`:239`)
- E1 in the semantics paper, which makes a supply change conditional on the signed `issue` right (`MIL2-PROPOSED-SEMANTICS.tex:171-178`)

**Inference.** The category is not yet fit to freeze, for five adversarial reasons:
1. **Unbacked mint.** A holder of `issue` who stays within budget can still mint with no backing. No rule ties a positive supply delta to collateral, reserve custody or a consented liability.
2. **Burns can replenish the issue budget.** The budget is only "linear | affine" (`DESIGN-MIL2.md:191`). Nothing states that it counts only positive supply deltas. Mint-burn-mint cycles could therefore exceed the signed quota.
3. **Supply aliasing.** E1 is checked on per-`(d,a)` sums (`.tex:173-176`). A hidden mint in a batch can net against an authorised burn. Nothing requires each supply line to carry its own authority, or forbids duplicate cell keys.
4. **Stale or self-dated attestation.** `fresh` requires the same clock (`DESIGN-MIL2.md:187`). A reserve report's `asOf` date is on the attester's clock, not the ledger clock, so there is no sound freshness rule for it.
5. **Depeg.** No policy mode gates minting or par redemption on reserve coverage (category review `03-stablecoins.md:24`). Par redemption during a shortfall pays early redeemers first and leaves the loss with later holders.

The carriers are sound, but the acceptance relation is missing these five clauses. None needs Φ₁, a global rollback, or a change to the separation of debt from supply.

## 2. Five ranked design edits (recommendations)

### E1. Typed `mint`/`burn` effects with a quota that burns cannot refill

**Rule sketch:**
```
Mint(d,a,to,n):  g.kind=issue ∧ g.scope=(d,a,+) ∧ g.root=IssuerRoot(a) ∧ ¬revoked(g)
                 ∧ n ≤ g.remaining
  ⟹ balance(d,to,a)+=n ; supply(d,a)+=n ; g.remaining−=n        (checked u128)
Burn(d,a,from,n): g.scope=(d,a,−) ∧ debitAuth(from,a,n) ∧ n ≤ spendable(from,a)
  ⟹ balance(d,from,a)−=n ; supply(d,a)−=n                      (g.remaining unchanged)
```
- The `issue` scope gains a direction field `{+,−}`. This is not a ninth right.
- The budget is cumulative across the Episode, like gross debit (`.tex:189`).
- Refusals: `ISSUE_UNAUTHORISED`, `ISSUE_QUOTA`, `ISSUE_REVOKED`, `SUPPLY_RANGE`.

**External practice (not a Moriarty rule).** Circle's FiatToken gives each minter a `minterAllowance`, and "the `minterAllowance` will decrease by the amount of tokens minted". Burning is limited to minters, and `removeMinter` sets the allowance to 0 (https://github.com/circlefin/stablecoin-evm/blob/master/doc/tokendesign.md).

**Counterexamples:**
- Quota 100: mint 100, burn 100, mint 100. If a burn refills the budget, the signed cap is exceeded. It must reject at the third step.
- **Wrong issuer:** a grant for `asset:0x07` (symbol USDm) is used to mint `asset:0x09`, a different asset with the same symbol. It must reject because `g.scope` compares `AssetId`, not the symbol.

**Placement:**
- **U0 change:** the `issue` scope direction and "the budget counts only positive deltas" go into the right-kind enum, and the four refusal codes are reserved. **Why:** the enum and canonical encoding are hash-bound at U0 (`DESIGN-MIL2.md:344-345`), so adding budget semantics later would change the digest.
- Execution belongs to a named `issuance/1` profile at **U2**.
- S0 keeps "empty supply, reject any non-empty value" (`R3-stablecoins.md:321`, R22).

### E2. Backing bond: every positive supply delta has a declared backing co-effect

**Rule sketch.** The asset's `policy(a)` cell declares `backing ∈ {reserve(R, num/den), cdp(policy), none-forbidden}`. Acceptance adds:
```
BackingBound:  Δsupply(d,a) > 0 ⇒
   reserve:  Δbalance(d, Custody(a), R) × den ≥ Δsupply(d,a) × num          -- Φ₀ literal coeffs
   cdp:      ∃o new. consent(o) ∧ creation_o = Δsupply ∧ ∃x. lock(x) against {o} ∧ K1 holds
```
- This keeps debt separate from supply. The CDP branch creates an obligation under L1 (`.tex:180-187`) and a lock under K1 (`.tex:229-237`); supply records only token units.
- Rounding is role-directed: minted units round down, and required backing rounds up (numeric profile D2, cited at `R3-stablecoins.md` R11).

**Counterexamples:**
- **Unbacked mint:** a valid quota mints 100 with no custody credit. It passes E1 and the quota check, but must fail `BACKING_SHORTFALL`.
- **Double pledge:** the same 150 collateral is locked against two CDP mints. Each lock alone passes, and K1 must reject the sum.

**Placement:**
- Reserve backing: **U2**, in Φ₀.
- CDP health, which multiplies an oracle price by a variable collateral quantity: **U4**, in Φ₁ with the limb rule (`DESIGN-MIL2.md:160-171`).
- I do not recommend pulling Φ₁ forward.

### E3. Per-line supply authority and canonical cell uniqueness (supply aliasing, hostile witness)

**Rules:**
- **Canonical form:** effect lines are sorted, and duplicate `(d, acct, AssetId)` balance keys or duplicate `(d, AssetId)` supply keys are rejected.
- **Authority per line:** each non-zero supply line consumes its own authority (E1). Positive and negative supply lines are never netted before the authority check.
- **Netting:** a netting lane requires the separate sums of mints and of burns to be zero unless each line is authorised.
- **Witness binding:** `pre(supply)`, `pre(balance)` and `AssetId` must be authenticated ledger reads bound in the circuit (`.tex:240`). Every Φ bit is Boolean-constrained (`DESIGN-MIL2.md:265`).

**Counterexamples:**
- **Aliasing:** a batch of five 20-unit redemption burns also carries a hidden +100 mint to the solver. The net supply delta is 0, so E1 passes. Only per-line authority rejects it.
- **Split supply keys:** two supply lines for one `(d,a)` hide an overflow.
- **Hostile witness:** the witness gives `pre(supply)=0` when the ledger says 10⁹, so the cap check passes. This must reject because the value was not read from the ledger.

**Placement:**
- **U0 change:** duplicate-key rejection goes into the canonical encoding. **Why:** canonical form is U0 (`:344`), and aliasing depends on encoding.
- Per-line authority is a U0 judgment clause. It is enforced at U2.

### E4. Reserve attestation as a typed evidence record, fresh by ledger inclusion time

**Rule sketch:**
```
Obs<ReserveAtt, attested(Attester,k,n), d> with {subject=reserve(a), assurance, asOf, validUntil,
     attester, obligor, statusRef, scope}
Formation: attester ∉ Obligors(a)   (independence)
fresh_att(o,Δ): asOf ≤ includedAt(o) ∧ stageTime − includedAt(o) ≤ Δ ∧ stageTime ≤ validUntil
               ∧ ¬revoked(statusRef)        -- includedAt is the anchored Midnight instant
Cap:  post(supply(d,a)) × den ≤ o.value × num      -- attestation caps supply, not per mint
```
- An attestation's source set stays `attested`. It can never fill an `anchored` position (`DESIGN-MIL2.md:185`).
- It remains a named trust premise (TP03), and imported evidence is verified at U4 (`:187,350`).

**External practice (not a Moriarty rule).** The captured W3C VC `validFrom`/`validUntil` fields and the Status List revocation mechanism (`source-text/w3c-vc-data-model.md`, `source-text/w3c-status-list.md`) are comparable. The category requirements come from L27 (`R3-stablecoins.md` A-6).

**Counterexamples:**
- **Stale:** a report with `asOf = t−90d` is posted today. Self-dated freshness passes, but `validUntil` or the Δ bound on ledger inclusion must reject it.
- **Reuse:** one attestation of 1,000,000 funds ten mints of 1,000,000 each. The per-mint check passes, but the supply cap rejects the second mint.
- **Self-attestation:** attester equals obligor. It must reject at formation.

**Placement:**
- **U0:** reserve the `attested` class index. It already exists (`:177`), so there is no new U0 change.
- **U2:** record schema and freshness rule.
- **U4:** verification of imported and attested evidence.

### E5. Policy mode cell with solvency-gated par redemption (depeg)

**Rule sketch.** `policy(a).mode ∈ {live, mintPaused, redeemOnly, settled(p_fix, R_snap, S_snap)}`:
```
Mint requires pre(mode)=live.
ParRedeem requires pre(mode)∈{live,mintPaused,redeemOnly}
   ∧ post(balance(d,Custody(a),R)) × den ≥ post(supply(d,a)) × num      -- Φ₀
Mode transitions: enforce-scoped right, signed order (§6); settled is terminal (tombstone); Δsupply ≤ 0 after settled.
ProRataRedeem(settled): payout = ⌊bal × R_snap / S_snap⌋                  -- Φ₁
```
- Each holder claims separately, at their own stage. That avoids global enumeration, which the category review says a bounded stage cannot do (`03-stablecoins.md:24`).

**Counterexample.** Reserve 90, supply 100. Ungated par redemption lets the first 90 units drain the reserve, leaving the last 10 holders with nothing. The gate rejects `RESERVE_COVERAGE` and forces mode `redeemOnly` or `settled`.

**Placement:**
- **U2/U3:** the mode cell and gates.
- **U4:** pro-rata payout (Φ₁).
- **U6:** full shutdown and loss waterfall, including residual claimants and the backstop.

## 3. Core versus library (recommendation)

**Core:**
- the `mint`/`burn` constructors and the `issue` direction and quota semantics
- per-line supply authority
- supply-cell uniqueness and canonical duplicate rejection
- E1
- a generic `BackingBound` hook that reads `policy(a)`
- the attested-evidence record, with freshness by inclusion time and independence
- the policy-mode read and the rule that `settled` is terminal
- refusal codes

**Library (U6, from certified primitives):**
- CDP origination, health and liquidation
- the loss waterfall
- stability fee and savings rate
- peg-swap modules
- fiat request/confirm with settlement lag
- shutdown pro-rata payout
- a rebasing index adapter
- synthetic shared-debt pools and hedges
- cross-domain burn-mint and lock-mint forms (L9, as an Episode of two local stages)
- the reflexive `backs` graph lint

Blacklists and freezes are application policy. They are not language core.

## 4. Smallest implementable slice (recommendation)

**The `issuance/1` peg-swap mint**, at U2 after the S0 path works. Setup:
- one domain, one signer
- reserve asset R, already existing and anchored
- a stable asset S with `backing = reserve(R, 1/1)` and equal decimals
- issuer quota 1,000; supply cap 1,000; mode `live`

One stage:
1. Transfer n R from holder to `Custody(S)`.
2. `Mint(S, holder, n)`.

Guards: `post(supply) ≤ cap` and `BackingBound`.

**Positive control:** n = 100.
- Asset R: holder −100, custody +100, supply delta 0.
- Asset S: holder +100, supply delta +100.
- Quota goes from 1,000 to 900. Expected result: accepted.

**Hostile control:** the same valid signed intent and a well-formed envelope, but the witness routes 99 R to custody and 1 R back to the holder.
- E1 holds for R (the net is 0), and the quota and cap hold.
- Only `BackingBound` fails, so the expected result is `BACKING_SHORTFALL`.
- This is a semantic rejection, not a malformed envelope, as `ROADMAP.md:40` requires.

**Follow-up mutations:**
- wrong `AssetId` with the same symbol → `ISSUE_UNAUTHORISED`
- a 950 mint after the 100 mint → quota and cap reject
- a forged `pre(supply)` → authenticated-read rejection
- mint-burn-mint beyond the quota → `ISSUE_QUOTA`

**Evidence order:**
1. TypeScript/K differential (O1, O3, O4).
2. U1 certificates for checked u128 add and compare on supply.
3. U2 native verification with full effect readback.

No step is claimed here.

## 5. Explicit disagreements

1. **With `03-stablecoins.md:39`**, which asks for "a smallest valid CDP and redemption example": a CDP mint guarded by an oracle price multiplies price by collateral, which is Φ₁ (`DESIGN-MIL2.md:160`). Make the peg-swap reserve mint the first slice and leave CDP for U4.
2. **With `03-stablecoins.md:19`**, which says a ratio is expressible "if … price conversion [is] pinned": that is true only if the price is a signed literal. A live `Price` observation is a variable, so the guard is not Φ₀.
3. **With E1 as written** (`.tex:178`): one "signed issue right" for any supply change is not enough. It needs a direction, a quota that burns do not refill, and per-line consumption.
4. **With the current per-owner lock invariant** (`DESIGN-MIL2.md:102`): it does not bound supply. The reserve-coverage gate and backing bond are separate clauses, not consequences of K1.
5. **Against a ninth right or a system-wide pause primitive:** direction-scoped `issue` and `enforce`-gated policy modes are enough.

## 6. Residual assumptions

- **Midnight native tokens (not verified here).** I assume the token type is derived from the minting contract and a domain separator, so that `IssuerRoot(a)` can be read from the ledger. This must be pinned in the U0 target matrix before E1 is sound.
- **Trust premises that stay open:** attester honesty, off-chain reserve existence, legal recourse and oracle honesty (TP03; product contract `:47`). A proof never establishes them.
- **Caps:** effect-line and footprint caps (`DESIGN-MIL2.md:274-283`) must admit transfer + mint + policy + quota cells. I estimate about 8 footprint cells, which has not been measured.
- **External source:** the Circle FiatToken design was fetched live and not added to `source-text/`. It needs a pinned receipt before anyone relies on it.
- **Unproven obligations:** O1–O6 remain open. Nothing here claims implementation, proof or ledger acceptance.