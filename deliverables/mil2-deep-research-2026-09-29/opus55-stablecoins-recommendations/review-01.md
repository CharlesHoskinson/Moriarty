# Issuance and supply review of MIL/2: stablecoins and synthetic assets (Opus 5.5 recommendation brief)

**Status:** This is read-only design review. Nothing here has been implemented, proved, compiled or accepted on a ledger. The `status --json` command reports that dispatch is blocked at `sp01-loan-report`. That does not block this review.

## 1. Verdict

**Not yet fit for the issuance part of the category.** MIL/2 has the right pieces: a nominal `AssetId`, an `issue` right scoped to `(domain, asset)`, a `supply(d,asset)` cell, and the general conservation law (`concepts/intent-language/DESIGN-MIL2.md:53-61,193,239,256`). Debt is kept separate from supply (`docs/MORIARTY-PRODUCT-CONTRACT.md:43`; `MIL2-PROPOSED-SEMANTICS.tex:180-187`). Four defects still make supply accounting unsound or impossible to state:

- **Conservation is vacuous.** E1 equates balance deltas with a *declared* supply delta (`DESIGN-MIL2.md:256`; `MIL2-PROPOSED-SEMANTICS.tex:171-178`). No effect constructor produces a supply delta: the only effect form shown is `transfer` (`DESIGN-MIL2.md:313-314`). If the supply delta is a free field, E1 holds for any unbalanced effect set whose declaration matches it.
- **One `issue` right covers mint, burn and confiscation.** There is no rule that says whose balance a burn may debit.
- **Nothing links issuance to debt.** L1 (the liability roll-forward) permits `authorizedForgiveness` (`MIL2-PROPOSED-SEMANTICS.tex:182-183`). Once debt is forgiven, circulating tokens have no debtor, and no state records that they are unbacked.
- **Collateral-ratio guards cannot be typed.** Formation allows comparisons only between quantities of the same asset (`DESIGN-MIL2.md:111-120`), so `collateral ≥ 1.5 × debt` across two assets is a type error unless a conversion term exists.

## 2. Five ranked design edits (all are recommendations)

### E-1: Derive the supply delta from typed `mint` and `burn` effects

**Rule sketch:**
```
mint{domain d; asset a; to p; amount q:Qty<a>; under auth}
burn{domain d; asset a; from p; amount q:Qty<a>; under auth}
Δsupply_derived(d,a) := Σ mint.q − Σ burn.q        -- checked i129, then u128 on apply
transfer contributes 0 net to Δsupply_derived
Conserve(d,a):  Σ_p Δbalance(d,p,a) = Δsupply_derived(d,a)
                ∧ declared.supplyChanges(d,a) = Δsupply_derived(d,a)
supply'(d,a) = supply(d,a) + Δsupply_derived   -- SUPPLY_UNDERFLOW / ARITH_RANGE reject
foot(mint) : W ⊇ {supply(d,a), balance(d,p,a), allowance(auth)}
```
A stage that has no `mint` or `burn` line gets `Δsupply_derived = 0` automatically. This makes the netting and transfer-only rule (category report row "Netting cannot hide issuance", `03-stablecoins.md:29`) a consequence of the rule, not a separate lane rule.

**Counterexample to the current design:** A stage declares `supplyChanges(USDm) = +100`, credits `R` with +100, and has no mint line and no `issue` consumption. Derived-footprint containment never sees a write to `supply(d, USDm)`, because no effect writes it. So `AuthorityFresh` has nothing to require `issue` against, and E1 holds.

**Placement:** U0 freezes the constructor tags, the derivation, the refusal codes (`SUPPLY_UNAUTHORIZED`, `SUPPLY_SCOPE_MISMATCH`, `SUPPLY_CONSERVATION`, `SUPPLY_UNDERFLOW`, `SUPPLY_FOREIGN_DOMAIN`) and the footprint rule. This is a U0 change because the public-input schema, the cell vocabulary and "conservation as the general law" are already U0 items (`DESIGN-MIL2.md:345`). Adding them later means changing the frozen digest. The S0 slice stays at empty-with-rejection (`R3-stablecoins.md:321`). U2 admits the first non-empty supply change.

### E-2: Give `issue` a quantity budget with separate mint and burn authority

**Rule sketch:**
```
issue ::= { scope:(d,a), mintBudget: linear Qty<a>, burnScope: self | consentedEnc(policy),
            window, delegable, revocable }
Mint:  Σ_{lines under auth} q ≤ remaining(auth);  allowance'(auth) = remaining − Σq
Burn(from p):  (p = auth.holder ∧ burnScope ∋ self)
             ∨ (∃enc. owner(enc)=p ∧ enforce-right(enc) ∧ policy consented by p)
             ∧ spendable(p,a) ≥ q              -- K1 lock safety still applies
```
**Counterexample:** Under the current rules, a holder of `issue(preview, USDm)` can emit `burn{from victim; 100}`. The supply and balance deltas agree, E1 holds, and scope matches. The result is confiscation with no consent. A burn also must not consume locked balance: if `victim` has 100 USDm and all of it is encumbered, burning 100 breaks K1 (`MIL2-PROPOSED-SEMANTICS.tex:229-236`).

**External practice (not a Moriarty rule):** Circle's FiatToken gives each minter a `minterAllowance` that decreases on mint, and `burn` debits only the minter's own balance (https://github.com/circlefin/stablecoin-evm/blob/master/doc/tokendesign.md).

**Placement:** U0 fixes the shape of the right. `DESIGN-MIL2.md:191` already gives each right a budget, so the only additions are `burnScope` and the direction split. U2 enforces them.

### E-3: Add an issuance-policy cell and a debt-correspondence invariant with an explicit unbacked bucket

**Rule sketch:** Add a new cell, `issuance(policyId) = { asset, issuedNet:Qty, unbacked:Qty, ceiling:Qty }`. A policy of class `debt-backed` has these transitions:
```
MintAgainstDebt(o,q):   mint(a→debtor(o), q) ∧ creation_o = q ∧ consent(o)
                        ∧ ∃enc∋o created/extended this stage ∧ issuedNet' = issuedNet + q
RepayBurn(o,n):         AccrualFirst dA=min(n,a_o), dP=n−dA;
                        burn(a, payer, dP) ∧ issuedNet' = issuedNet − dP;
                        dA is a funded transfer to the policy's surplus account (not burned)
WriteOff(o,x):          authorizedForgiveness_o = x (principal part)
                        ∧ unbacked' = unbacked + x      -- tokens still circulate
K2 (per policy):        issuedNet = Σ_{o∈P} principal_o + unbacked
```
Debt stays separate from supply: K2 is a correspondence between two separate ledgers, not a merger of them. With this rule, a write-off can no longer leave circulating tokens silently unbacked.

**Counterexample:** A CDP mints 100 USDm (L1 creation = 100). Governance then forgives the 100. L1 holds, E1 holds and K1 holds, yet 100 USDm circulate with no debtor, and no state reveals the shortfall.

**External practice (not a Moriarty rule):** Maker's Vat keeps unbacked debt in `sin`/`vice` and maintains `debt = vice + Σ dai` (https://docs.makerdao.com/smart-contract-modules/core-module/vat-detailed-documentation).

**Placement:**
- U0 adds the `issuance(policyId)` cell kind to the §9 vocabulary, for the same digest reason as E-1.
- U3 enforces K2 across partial repayment and persistent duty (`ROADMAP.md:24`).
- The liquidation waterfall and backstop that consume `unbacked` belong to the U6 library.

### E-4: Check ceilings at the authenticated head, and add a typed cross-asset conversion for ratio guards

**Rule sketch:**
```
CapCheck: post(supply(d,a)) ≤ policy.cap ∧ post(issuance(P).issuedNet) ≤ P.ceiling,
          where pre(·) is the authenticated value at the head the ledger applies against;
          a proof against any other head rejects HEAD_STALE.
Φ₀ term:  convert(q:Qty<C>, lit:Price<U,C,s>, role) : Qty<U>
          role ∈ {collateralValue ↦ floor, debtValue ↦ ceil}   -- numeric profile D2
          fits-width or two-limb (DESIGN-MIL2.md:167-171)
```
**Counterexample:** The cap is 1000 and supply is 900. Two stages each prove `900 + 100 ≤ 1000` against the same pre-state. If the ledger does not serialize writes to the supply cell, supply ends at 1100. Separately, `locked(o,ETH) × 100 ≥ outstanding(o) × 150` is ill-typed under `DESIGN-MIL2.md:111-113`. Writers will then reach for Φ₁ `mulDiv`, which is deferred (`:161`).

**Placement:**
- U0 adds the role→rounding rows to the numeric profile (`ROADMAP.md:21` already requires per-primitive rounding direction).
- `convert` with a *literal* price stays in Φ₀, because it multiplies by a literal. This does not move a Φ₁ boundary.
- A `convert` that takes a live observed price is variable × variable, so it remains Φ₁/U4.

### E-5: Gate foreign-backed mints on consuming a linear receipt, and never refund on timeout

**Rule sketch:**
```
MintForeign(a_wrapped, q): consumed(receipt r) this stage
      ∧ r : Obs<Lock|Burn{srcDomain, srcAsset, q_src, beneficiary}, ε, d_src>
      ∧ q = q_src (fixed 1:1 or signed conversion with directed rounding)
      ∧ repr(a_wrapped) links srcAsset with form ∈ {lock-mint, burn-mint}
      ∧ ε ⊆ policy.accepted (imported ⇒ named trust premise until U4)
Source side: burn/lock stays in state `unknown` until destination evidence
             of received / nonreceivedProved; no refund on deadline alone.
```
**Counterexample:** A single lock receipt is presented to two mint stages. Each stage is locally valid and double supply results. Or a timeout triggers a source-side refund while the destination mint later finalizes, which creates the same double supply. `MIL2-PROPOSED-SEMANTICS.tex:202` already forbids reading absence as failure. **External practice:** IBC timeouts require proof about the destination chain (captured in `source-text/ibc-ics004.md`).

**Placement:**
- U3 covers receipt linearity and the pending/unknown states.
- U4 covers verified imported evidence (`DESIGN-MIL2.md:350`).
- Fiat request/confirm is the same shape with an `attested` receipt, and belongs to the U6 library.

## 3. Core versus library boundary (recommendation)

**Core (frozen at U0, enforced from U2 or U3):**
- `mint` and `burn` constructors, and the derived supply delta.
- E1 over derived deltas.
- The `issue` shape with its mint budget and burn scope.
- The `issuance(policyId)` cell with `issuedNet`, `unbacked` and `ceiling`, and the K2 invariant.
- Refusal codes, and the rounding roles for `convert`.
- Receipt linearity for foreign mints.

These go in core because each one either writes a cell that a hostile witness could otherwise reach through footprint gaps, or decides whether E1 or K2 can be stated at all.

**Library (U6):**
- CDP open, adjust and close; liquidation auctions and bonus rounding; the loss waterfall that consumes `unbacked`.
- Stability fees and savings rates, peg or PSM modules, and the fiat request/confirm workflow.
- Synthetic shared debt pools and emergency shutdown.

The shutdown library needs a core *completeness* primitive over `issuance(P)` (see D-3 below). Rebasing is outside this lens. My view is that it cannot be a library over explicit per-account deltas.

## 4. Smallest implementable slice (recommendation)

**Slice:** A single-signer mint under an allowance, using only Φ₀, with no collateral and no oracle, on `midnight.preview`.
- The issuer `I` holds `issue{scope (preview, USDm), mintBudget linear 1000, burnScope self}`.
- The stage mints to recipient `R`.
- The public statement binds: the intent digest, the authenticated pre-values of `supply`, `allowance(auth)` and `balance(R)`, the single mint line, the derived supply delta, and the post-values.

**Positive control:** pre `supply = 0`, `allowance = 1000`, `balance(R) = 0`. The stage has one line, `mint 100 USDm to R`. Expected post-state: supply 100, `balance(R) = 100`, allowance 900. Σ Δbalance = 100 = Δsupply_derived, and the declared change equals the derived one.

**Hostile control:** Keep the same signed digest, envelope and mint line, and give the witness an extra `balance(R)` credit of +50 (total Δbalance 150). Expected result: `SUPPLY_CONSERVATION`, enforced by a native constraint rather than a host flag. This hostile case is valid at the envelope level, so the rejection exercises E1 itself. That meets `ROADMAP.md:40`.

**Follow-up hostile controls:**
- a mint to `USDm'`, same symbol but a different `AssetId` → `SUPPLY_SCOPE_MISMATCH`
- a mint of 1001 → budget exceeded
- a forged pre-value of `allowance` → authenticated-read failure
- `burn{from R}` signed by `I` → `SUPPLY_UNAUTHORIZED`

**Placement:** U2, as the first non-empty supply profile. The debt-backed version (E-3 with a literal-price `convert`) is the next slice, at U3.

## 5. Explicit disagreements

- **D-1: E1 as currently written.** The category report treats "`Σ balance deltas = declared supply delta`" as a usable carrier that repairs R4 (`03-stablecoins.md:17`). I disagree: while the supply delta is declared rather than derived, E1 does not constrain issuance (E-1).
- **D-2: One `issue` right.** `DESIGN-MIL2.md:193` scopes `issue` only by `(domain, asset)`. Burning another holder's balance is an `enforce`-class act and needs debtor consent (E-2).
- **D-3: Where bad debt lives.** The report puts bad-debt recording under liquidation, which is library work (`03-stablecoins.md:22`). I put the `unbacked` bucket in core, because without it L1 forgiveness breaks K2 silently. Only the *allocation* of losses belongs in the library.
- **D-4: Old R10 ("L7 bonding").** R10 asks for a mint that is structurally bonded to redemption or liquidation capacity (`R3-stablecoins.md:311` context). I would not make that a core well-formedness rule. K2 plus the policy cell gives the checkable part; the capacity itself is a protocol property.
- **D-5: Milestone for supply constructors.** I agree issuance stays out of S0. I disagree that the constructors can wait for U6 (`R3-stablecoins.md` R23). Their tags and cells are digest-bound at U0.

## 6. Residual assumptions

- I have not verified how Midnight's native contract-controlled token issuance maps onto a `supply(d,a)` cell and its authenticated read. This needs a U0 target-matrix pin.
- The ledger must serialize writes to the `supply` and `issuance` cells, or check the head at application. E-4 depends on this.
- `u128` is assumed wide enough for supply and `issuedNet`. Overflow rejects; it never wraps.
- K2 is per policy and per domain. There is no global cross-domain supply invariant, and no global rollback.
- Imported and attested receipts remain named trust premises until U4. An attestation does not prove that off-chain reserves exist.
- The external URLs cited in E-2 and E-3 (Circle, Maker) come from my prior knowledge and were not fetched in this session; the IBC reference is the captured copy in `source-text/`. All three are comparative practice, not Moriarty rules.
- None of the six MIL/2 obligations (`DESIGN-MIL2.md:365`) is discharged. K1 and K2 are proposed invariants, not proved ones.