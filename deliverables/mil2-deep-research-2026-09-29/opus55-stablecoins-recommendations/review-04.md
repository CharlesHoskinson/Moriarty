# Stablecoins and synthetic assets: MIL/2 design recommendation (compiler and native feasibility)

**Status.** This is read-only design review. I compiled, proved and executed nothing, and no ledger has accepted anything described here. `status --json` reports implementation of `loan-swap-subset` as blocked. This review does not depend on that dispatch. Every item below marked **Recommendation** is a proposal, not an existing Moriarty rule.

## 1. Verdict

**The design is not yet fit for this category. It is repairable without a new language.** MIL/2 has the right components: a nominal `AssetId`, the `issue` right, a `supply(d,asset)` cell, general conservation E1, obligations kept separate from supply, the lock invariant K1, and evidence classes (`DESIGN-MIL2.md:50-61,126-137,189-193,252-256`; `MIL2-PROPOSED-SEMANTICS.tex:171-187`). Four things block a native collateralized-issuance stage:

- **No effect constructor.** No mint or burn effect form exists and no refusal code exists (`03-stablecoins.md:17`). The one existing schema field, `supplyChanges[]`, is keyed per account, which disagrees with E1's per-(domain, asset) key. It is also `NOT_ENFORCED` (`R3-stablecoins.md:299,322`).
- **`issue` has scope but no quantity.** It is a role scoped to `(domain, asset)` with a generic linear or affine budget. It carries no issuance quantity or recipient semantics (`DESIGN-MIL2.md:191-193`).
- **Collateral health is variable×variable.** The price observation times the collateral quantity is a product of two variables, which is Φ₁ and deferred to U4 (`DESIGN-MIL2.md:160-161,350`). Without it a CDP (collateralized debt position) cannot state an oracle-priced health guard.
- **Reads are not tied to native binding.** The design never says how `pre(supply)`, `locked` or `outstanding` are bound natively. "Authenticated ledger read" is an obligation with no lowering rule (`DESIGN-MIL2.md:185`; `.tex:240`).

## 2. Five ranked design edits (Recommendations)

### E1. Typed `mint` and `burn` effects with a derived supply footprint

**Rule sketch:**
```
mint{ domain d; asset a; to p; amount n : Qty<a>; right r }
burn{ domain d; asset a; from p; amount n : Qty<a>; right r }
Foot(mint) = R=W={ supply(d,a), balance(d,p,a), allowance(r) }
Eff(mint) : Δbalance(d,p,a)=+n, Δsupply(d,a)=+n      (burn: −n, −n)
Stage rejects SUPPLY_UNDECLARED if Δsupply(d,a) ≠ 0 with no mint/burn line,
          SUPPLY_CONSERVATION if Σ_p Δbalance(d,p,a) ≠ Σ mint − Σ burn,
          ARITH_RANGE if post(supply) ∉ u128.
```
Burning another holder's balance requires `enforce` under consented policy. `issue` alone never authorizes it.

- **Schema change.** Replace the per-account `supplyChanges[]` with `(domain, asset, signed delta)`. The per-account data already exists in the balance lines.
- **Counterexample.** Under the current per-account shape, one stage can emit `+100` to Alice and `−100` to a fabricated "reserve" account with no custody cell. The net supply is 0, so the S0 "Σ = 0" law (`UNIFIED-PROPOSAL.md:90`) accepts it. The rule above makes this unexpressible. Every balance line must name a cell in the derived footprint, and supply is keyed per asset.
- **Placement.** U0: constructor enum, cell and refusal codes in the stage schema. U2: enforcement. S0 keeps "empty-with-rejection" (`UNIFIED-PROPOSAL.md:74`).

### E2. `issue` as a linear quantity budget held in an authenticated cell

**Rule sketch:**
```
IssueRight r = { domain d, asset a, recipients ⊆ Party | any,
                 budget: linear Qty<a>, window, revocation }
AuthorityFresh ∧ pre(allowance(r)) ≥ n ∧ post(allowance(r)) = pre − n
               ∧ ¬revoked(r)@head ∧ p ∈ r.recipients
```
Burns do not replenish the budget unless a signed policy says so.

- **Signature binding.** The design says in-circuit Ed25519 is unavailable (`DESIGN-MIL2.md:264`). The circuit therefore binds the intent digest and `r` as public inputs, and the ledger checks the signature. The budget itself is a ledger cell, not a witness.
- **Counterexample.** With an *affine* or role-only right, two stages at different heads each mint 400 against a "500" budget. Each check passes locally, and 800 are issued.
- **External practice.** Circle's FiatToken uses a per-minter `minterAllowance` that `mint` decrements (https://github.com/circlefin/stablecoin-evm/blob/master/doc/tokendesign.md). That is evidence of a practice, not a Moriarty rule.
- **Placement.** U0: right-kind field and budget semantics. U2: enforcement.

### E3. Lower every Φ state read to a ledger-bound public-transcript read, and add named aggregate cells

**Rule sketch (lowering):**
```
⟦pre(c)⟧ = PublicInput under a guarded read of cell c at predecessorHead
⟦post(c)⟧ = Impact declaration of c's new value
PrivateInput never supplies a value of sort supply/balance/locked/outstanding/allowance/aggregate.
```
ZKIR states the public-input vector is "the only part of Σ the verifier sees" and uses guarded transcript reads with `Impact` declarations (`source-text/zkir-v3-spec.md:156,281`). "Anchored" should therefore mean *this* binding (`DESIGN-MIL2.md:185`).

- **New cells.** Add `aggregate(id)` to the cell vocabulary (`DESIGN-MIL2.md:239-242`) for per-collateral-class debt and global ceilings. A cap check is `post(aggregate(k)) ≤ cap_lit` with `aggregate(k)` in W. The ledger's head-conflict rule then serializes concurrent mints, so no reservation protocol is needed.
- **Readback.** For U2 evidence, add readback of the post-state cells from the ledger to compare against the declared post-state (`ROADMAP.md:23`). Readback is evidence and never an acceptance input.
- **Counterexample.** The prover feeds `pre(supply)=0` through `PrivateInput` while the ledger holds 999,950. The cap guard passes and the supply ends above the cap.
- **Placement.** U0: the lowering rule in the enforcement map. U1: certification of read-binding soundness. U2: executed.

### E4. One width-bounded, exact priced comparison node; this moves the deferred Φ₁ boundary

**Rule sketch (new node, "Φ₀+P"):**
```
Γ ⊢ q : Qty<C>!S₁   Γ ⊢ π : Price<U,C,s>!S₂ (S₂ ⊆ {anchored@d})   num,den : u16 literals
───────────────────────────────────────────────────────────────────
Γ ⊢ covers(q, π, debt, num/den) : Prop ! S₁∪S₂∪S₃
⟦covers⟧ ≡ q · π.mantissa · den  ≥  debt · num · 10^s
```
- **Width.** With mantissa at most u64 and s at most 18, the left side is at most 128+64+16 = 208 bits and the right side at most 128+16+60 = 204 bits. Both stay under `less_than`'s 253-bit limit (`DESIGN-MIL2.md:169`), so there is no limb gadget and no modular wrap.
- **No rounding.** The comparison has no division, so no rounding direction is needed.
- **Limits.** The comparison is checked per filling in the circuit, and no escrow exhaustiveness check involves it, so no SMT enters the authoring trusted base.
- **Cap.** At most one `covers` node per stage.
- **Why the boundary changes.** The category's core operation cannot be expressed without it, and this node avoids both reasons for deferring Φ₁ at `:163`: bitvector division cost and solver dependence. General Φ₁ (`mulDiv`, shares) stays at U4. U0's cap of "0 nonlinear nodes" stays; a version header adds the node at U2.
- **Counterexample (division-based conversion).** Take collateral 3, price 0.3 (s=1, mantissa 3), debt 1, ratio 1/1. `ceil(3·3/10)=1 ≥ 1` accepts an undercollateralized mint, which is the mistake if D2's "collateral charge → ceil" (`numeric-profile.json:231`) is misapplied. The exact form gives `3·3·1 = 9 ≥ 1·1·10 = 10`, which is false, so it rejects.
- **External practice.** Maker `vat.frob` compares `art·rate ≤ ink·spot` without dividing (https://github.com/makerdao/dss/blob/master/src/vat.sol).

### E5. CDP `draw` and `repayBurn` as a Core template over E1 to E4, with debt separate from supply

**`draw(o, enc, n)` in one stage:**
```
consent(o.debtor) ∧ enc.against ∋ o ∧ K1 over post(locked)
∧ L1: u'_o = u_o + n (creation)          -- obligation cell
∧ mint{to debtor; n}                     -- supply cell (E1)
∧ covers(post(locked(debtor,C)), π, post(outstanding(o)), num/den)
∧ post(aggregate(ilk)) ≤ line ∧ post(supply(d,U)) ≤ cap
```
**`repayBurn(o, n)`:** a `burn` of n from the debtor, an AccrualFirst split `dA=min(n,a_o)`, `dP=n−dA` (`.tex:186-187`), and lock release only if `post(outstanding)=0` or `covers` still holds.

- **Counterexample.** A single "supply-backed debt" cell that treats burn as discharge. With accrued interest of 5, burning 100 discharges 100 of principal and silently forgives the accrual. Keeping separate cells with L1 exposes this.
- **Placement.** U2 slice as a checked template. Liquidation, auctions and loss waterfalls go to U6.

## 3. Core versus library boundary (Recommendation)

**Core (language and native basis):**
- the `mint`/`burn` constructors
- the issue-right budget
- the `supply` and `aggregate` cells
- E1, L1 and K1
- the E3 read lowering
- `covers`
- refusal codes: `SUPPLY_UNDECLARED`, `SUPPLY_CONSERVATION`, `ISSUE_UNAUTHORIZED`, `ISSUE_BUDGET`, `CAP_EXCEEDED`, `HEALTH_FAIL`, `EVIDENCE_CLASS`, `STALE_OBS`, `CONSENT_MISSING`
- the signed seize order already declared core (`DESIGN-MIL2.md:193`)

**Library (U6):**
- CDP templates, including liquidation and bad-debt waterfall
- the peg stability module and redemption at par
- the fiat request/confirm lifecycle over escrow
- stability-fee and savings-rate modules
- synthetic pools and hedges

**Deferred core:** emergency shutdown, rebasing and cross-domain supply. Each needs a core primitive MIL/2 lacks: authenticated completeness over all claims, a global balance index, or a cross-domain form rule. None of these can be a library over the current core.

## 4. Smallest implementable slice and evidence pair

**Slice S-ISS-1** (Recommendation; U2, one signer, one domain, Φ₀+P): the `draw` from E5 on a fresh obligation.

**Setup.** USDm and C both have 6 decimals. The price is `Price<USDm,C,6>` with mantissa 1,000,000, anchored and fresh within 5 minutes. The ratio is 3/2. The issue budget is 500 USDm, the cap is 1,000,000 USDm, and the pre-state supply is 0.

**Positive control.** The owner locks 150 C against `o1` and mints 100 USDm.
- Check: `150e6·1e6·2 = 3.0e20 ≥ 100e6·3·1e6 = 3.0e20`. This is the exact boundary and it accepts.
- Expected post-state:
  - `supply(d,USDm)` = 100e6
  - `balance(owner,USDm)` = +100e6
  - `outstanding(o1)` = 100e6, with principal 100e6 and accrued 0
  - `locked(owner,C)` = 150e6
  - `allowance(r)` = 400e6
- Readback: every one of these cells must match.

**Hostile control.** The same bytes with `n = 100_000001`, which keeps the witness feasible. The comparison becomes `3.0e20 ≥ 3.00000003e20`, which is false, so the stage rejects `HEALTH_FAIL` with no effects published.

**Second hostile control (E3).** A feasible witness whose `pre(supply)` comes from `PrivateInput`. It must fail natively, not only in the host evaluator. A malformed-envelope rejection does not count as evidence for this control (`ROADMAP.md:40`).

## 5. Explicit disagreements

1. **With `03-stablecoins.md:42` and the placement at `ROADMAP.md:27`.** Issuance *primitives* should not wait for U6. They belong in the U0 schema and a U2 slice. Only the protocol families belong in U6. This settles the R23 split (`R3-stablecoins.md:320`).
2. **With `DESIGN-MIL2.md:160-161,276`.** Deferring every variable product to U4 blocks the category for no feasibility reason. E4 admits one exact, bounded product comparison at U2. General Φ₁ remains deferred.
3. **With `DESIGN-MIL2.md:191-193`.** Scoping `issue` by `(domain, asset)` is not enough. It needs a linear quantity budget held in an authenticated cell, plus a recipient scope.
4. **With `UNIFIED-PROPOSAL.md:90`, and the per-account `supplyChanges` shape.** Adopt the per-(domain, asset) delta. I agree with the category report's correction that "Σ supply = 0" is S0-only.
5. **Agreement, stated for completeness.** Imported or attested reserve evidence cannot anchor an issuance guard (`DESIGN-MIL2.md:187`). Fiat-reserve issuance stays a named trust premise until U4.

## 6. Residual assumptions (unverified)

- **Ledger-bound reads.** Midnight's ledger checks public transcript reads against authenticated contract state at the bound head, and it rejects stale heads. I read only the circuit side (`zkir-v3-spec.md:246-281`); the ledger behavior is assumed.
- **Price mantissa width.** The design does not state it; E4 assumes u64.
- **Supply ownership.** The supply cell is contract-held state. Whether Midnight native token minting can be read back and bound this way is unverified.
- **Signature binding.** The ledger verifies the signature over the intent digest, so no in-circuit signature is needed.
- **Oracle honesty.** "Anchored" establishes only authenticated provenance, not a correct price. Oracle honesty stays a trust premise (TP03, `R3-stablecoins.md:302`).
- **Unproved obligations.** K1, non-laundering and footprint containment (`DESIGN-MIL2.md:365`; O1–O6 at `.tex:247-252`) are unproved. The slice depends on them.