# Lending and borrowing: compiler and native-feasibility recommendation (Opus 5.5, independent)

**Scope.** This is read-only design review of a specified-only proposal. I ran the guarded `status --json` (capability `SP01.6 loan-swap-subset`; implementation is blocked by stale inputs and unresolved history; no pending transactions). I did not build, test, prove or submit anything. Everything below is a **recommendation**. None of it is a claim of implementation, proof or ledger acceptance.

## 1. Verdict

**Fit for one narrow fixed-rate loan. Not yet fit to compile.** MIL/2 has the right carriers: an `Obligation` kept apart from supply, `AccrualFirst`, an encumbrance set, consent, and `enforce` (`concepts/intent-language/DESIGN-MIL2.md:93-102,191-193`). The product rule "debt is not token supply" is preserved (`docs/MORIARTY-PRODUCT-CONTRACT.md:99`). Four gaps stop a compiler from lowering even a bullet loan:

- **Division.** Φ₀ has no division at all; `term × lit` is its only product (`DESIGN-MIL2.md:136,160`). ZKIR v3 only divides by powers of two (`DivModPowerOfTwo`, n ≤ 248), and `div` assumes a non-zero argument (`source-text/zkir-v3-spec.md:572,769`). Rate accrual therefore has no lowering target.
- **Rounding direction.** The numeric profile still lets the author pick `floor` for liability-increasing accrual. It also assigns a "protocol-reserve" beneficiary that no source declares (`deliverables/u0-semantic-contract-2026-09-23/numeric-profile.json:58,69-73`).
- **Lock sum.** K1 needs a sum over an unbounded set of locks (`MIL2-PROPOSED-SEMANTICS.tex:229-236`). A circuit cannot compute that without an authenticated aggregate.
- **Price × quantity.** A threshold over live collateral multiplies two variables, which is Φ₁ (`DESIGN-MIL2.md:160`). The category report's "plausible in Φ₀" (`category-review/02-lending.md:20`) holds only when the collateral amount is a literal.

## 2. Five ranked design edits (recommendations)

### E1. Role-directed rounding plus a divide-by-literal term

**Rule sketch.** Add a term `divlit_ρ(x, n, d)`, where `n` and `d` are signed literals with `0 < d ≤ 2^64` and `n ≤ 2^64`, and `ρ` is a numeric role. The compiler lowers it to a witness `q` plus two constraints that use only literal-coefficient products:

- `ceil`: `q·d ≥ x·n` and `q·d < x·n + d`
- `floor`: `q·d ≤ x·n` and `x·n < q·d + d`

With `x ≤ 2^128`, every side is below 2^193. That is under the `LessThan` bound (`bits < FR_BITS`, `zkir-v3-spec.md:990`) and needs no limb gadget (`DESIGN-MIL2.md:169-171`).

Direction comes from the role and cannot be chosen by the author:

| Role | Direction |
|---|---|
| `liabilityIncrease` (accrual) | ceil |
| `dischargeCredit` | floor |
| `payoutToActor` (seizure, release) | floor |
| `requirementOnActor` (collateral required, amount due) | ceil |

A rounded liability produces no token remainder, so it needs no reserve posting. Remainder posting applies only to splits across several recipients, and those stay under E1 conservation.

**Counterexample.** P = 99 units at 1 % per period with floor accrues 0 every period, so the debtor borrows interest-free for ever. The author-selectable floor path exists today (`numeric-profile.json:73`).

**Placement.** U0 numeric profile (role table; `ROADMAP.md:21` requires a per-primitive direction and beneficiary). U1 quotient-witness certificate with adversarial-witness soundness (`ROADMAP.md:22`).

**Boundary change, stated exactly.** This adds one term to the U0 Φ₀ term set. It does not move Φ₁: the divisor is a signed literal, so there is no variable × variable product and no variable divisor. The authoring check still reduces to QF-LIA. The reason is that fixed-rate accrual cannot be expressed without it.

### E2. Obligation as an authenticated state machine with one L1 check per transition

**Rule sketch.** Cell `obligation(id)` holds `{principal, accrued, outstanding, status ∈ {Active, Matured, Defaulted, Settled}, lastPeriod: u64, termsDigest}`. The allowed transitions are:

| Transition | Required conditions |
|---|---|
| Originate | debtor consent over `termsDigest` |
| Accrue(k) | `k = lastPeriod + 1`, `after(t0 + k·Δ)`, amount = `divlit_ceil(principal, r_n, r_d)` |
| Repay(n) | AccrualFirst split (`DESIGN-MIL2.md:100`); a funded transfer in the same stage |
| Default | `after(maturity + grace)` and `outstanding > 0` |
| Seize | E3/E5 |
| Forgive | creditor authority |
| Settle | only when `outstanding = 0` |

Each accepted stage checks `u' = p' + a'` and the L1 roll-forward (`MIL2-PROPOSED-SEMANTICS.tex:180-187`). It also publishes `liabilityId`, `opening` and `closing` in the stage statement (`:240`).

**Counterexamples.**
- A stage leaves `obligation(id)` out of its writes, so the debt silently vanishes. Today every `liabilities.opening/closing` leaf is still marked `partial` because "there is no opening-liability array and no liabilityId" (`R2-lending.md:204`).
- Accrue(k) is applied twice in one period.

**Placement.** U0 stage-statement schema. U2 lowering (the existing lifecycle kernel is the source/Core baseline; K10 is covered at lifecycle scope only, `R2-lending.md:204`).

### E3. Split encumbrance into `custody` and `lien`; enforce K1 through an aggregate cell

**Rule sketch.**
- `custody`: collateral moves into program custody. Here K1 reduces to conservation, because locked coins cannot be spent.
- `lien`: needs a new cell `lockTotal(registry, owner, asset)`. Every reserve, commit, release, seize or cancel applies a checked delta to that cell, and the stage asserts `lockTotal' ≤ balance'`, where both values are authenticated reads of the same registry.
- Liens are admitted only inside a single declared registry.

**Counterexamples.**
- Two independent contracts each record a lien on the same 1.5 ETH. No circuit sees both, so `Σ locks ≤ balance` (`DESIGN-MIL2.md:102`) cannot be enforced.
- A prover omits a reserved lock from a witness-side sum. An aggregate cell kept by ledger induction closes this.

**Placement.** U0 cell vocabulary (`lockTotal`) and encumbrance-kind enum. `custody` in U2/U3. `lien` in U3 with the O3 two-solver traces (`MIL2-PROPOSED-SEMANTICS.tex:249`).

### E4. Price orientation, dimension typing, and a fixed-collateral health check

**Rule sketch.**
- Replace "base-per-quote" (`DESIGN-MIL2.md:69`) with an explicit `Price<Q per B, s>`.
- Typing rule: `value(q: Qty<B>, p: Price<Q per B, s>) : Qty<Q>·10^s` is Φ₀ only when `q` or `p` is a literal. Otherwise it is Φ₁.
- Health check for a loan whose collateral `C` is signed and fixed: `p.value × (C·10^4) ≥ outstanding × (10^s·R_bps)`. Both sides are literal-coefficient products (Φ₀ cross-multiplication, `DESIGN-MIL2.md:140`). The requirement rounds as `requirementOnActor`. `p` must be anchored and fresh (`:185-187`).
- Opening and maintenance thresholds are separate signed literals.

**Counterexample.** Read with the opposite orientation, a 200 % check passes a position that is actually at 50 %. The type system accepts both readings today.

**Placement.** U0 orientation (`ROADMAP.md:21` names canonical price orientation). The health check itself goes in a U3 lending profile.

### E5. Default and seizure transition with debt persistence and no oracle in the first slice

**Rule sketch.** Seize(enc, o) is allowed only when `status = Defaulted` and `enforce` is held under the consented policy (`DESIGN-MIL2.md:193`). It:

1. transfers the custody quantity `C` to the creditor;
2. discharges `c = min(outstanding, credit)`, where `credit` is a signed literal recovery value rounded as `dischargeCredit`;
3. leaves the residual `outstanding − c` in place;
4. writes a tombstone on `enc`.

Oracle-driven partial liquidation requires `seizeQty × p.value` (variable × variable). That belongs behind a named bounded-product profile, not the slice.

**Counterexamples.**
- A proof shows `seized(enc)` and sets the obligation to `Settled` while 200 remains unpaid. `seized` alone proves no amounts (`category-review/02-lending.md:21`).
- Two keepers seize the same encumbrance. The tombstone plus the signed ordering rule (`DESIGN-MIL2.md:193`) must reject the second.

**Placement.** Time-based default and full seizure: U3. Oracle partial liquidation: U4 (Φ₁) or a later bounded-product profile.

## 3. Core versus library (recommendation)

**Core**, so that one compiled relation enforces it for every program:
- the Obligation cell and L1;
- the transition judgments: creation needs consent; discharge needs a funded transfer, a seizure credit or authorized forgiveness;
- encumbrance kinds and `lockTotal`;
- `divlit` and the rounding-role table;
- price dimension typing;
- tombstones and seize ordering;
- the opening/closing liability fields in the public statement.

**Library**, as signed parameters and templates:
- bullet or amortizing schedules, period length and rate literals;
- grace and cure policy, recovery-credit policy;
- close factor and bonus;
- LTV and maintenance literals;
- pools and IRMs, e-mode and isolation, loss waterfalls, undercollateralized credit.

Libraries may only instantiate Core transitions. They add no acceptance clause.

## 4. Smallest implementable slice and evidence pair

**Slice L0 (U2 target).** One domain. Settlement asset A (6 decimals). Collateral B held in program custody (E3). A bullet loan with fixed simple interest on principal (declared as such; compounding is out of scope, `R2-lending.md:223`). Four transitions: Originate, Accrue(k) with ceil, partial Repay under AccrualFirst, and full Repay → Settle → release custody. No oracle and no default.

**Correspondence chain.** Source/5 → Core/4 → ExpressionPrepared → FinancialPrepared → StageCandidate → native (`MIL2-PROPOSED-SEMANTICS.tex:157-167`). One differential per arrow:
- TypeScript against K on the same fixture;
- K against ZKIR witness extraction, with the O3/O4 producer and witness-shape premises (`zkir-v3-spec.md:990-1000`);
- ZKIR against ledger effect readback.

**Enforcement locus.**
- `divlit` and L1: in-circuit constraints.
- `obligation(id)` opening: authenticated ledger read. Closing: ledger write.
- `lastPeriod`: ledger replay cell.
- Consent signature: checked by the ledger, not in-circuit (`DESIGN-MIL2.md:264`).
- No host-computed acceptance Boolean.

**Positive control.**
1. Start: P = 999,999,999, rate 50/10,000. Accrue(1) computes 49,999,999,950 / 10,000 → witness q = 5,000,000. Check: 5,000,000 × 10,000 ≥ 49,999,999,950, and 5,000,000 × 10,000 < 49,999,999,950 + 10,000.
2. Repay 30,000,000 backed by a funded 30,000,000 A transfer. Split: dA = 5,000,000, dP = 25,000,000.
3. Closing state: p = 974,999,999, a = 0, u = 974,999,999.

**Hostile control.** The same valid envelope, signatures, predecessor and funding, with the accrual witness q = 4,999,999 (the floor result). 4,999,999 × 10,000 = 49,999,990,000 < 49,999,999,950, so the native ceil constraint must reject. Because everything else is feasible, the rejection is semantic and not caused by a malformed envelope (`ROADMAP.md:40`).

A second mutation for the history key: the closing liability equals the opening liability while the transfer is still recorded (debt erased).

## 5. Explicit disagreements

1. **Category report, line 20.** Price-conditioned borrowing is Φ₀ only when collateral is a literal. Dynamic collateral × price is Φ₁ (`DESIGN-MIL2.md:160`).
2. **`numeric-profile.json:70`.** A `protocol-reserve` remainder beneficiary for accrual is ill-typed. Liability rounding produces no token remainder; its beneficiary follows from the role.
3. **`DESIGN-MIL2.md:102`.** The unscoped "per owner and asset" K1 cannot be enforced across unrelated contracts or unregistered balances. It must be scoped to an authenticated registry.
4. **The Φ₀/Φ₁ cut (`DESIGN-MIL2.md:160-163`) is too coarse for lending.** The cited cost is SMT authoring and verification cost. A single checked u128 × u64 product costs little in a circuit. A later option (**not** proposed for U0): allow bounded variable products in circuit guards but exclude them from the escrow authoring-check fragment, which fails closed as `unsupported`. I keep this deferred to respect decision 1.
5. **Agreement.** The flash-loan exclusion (`DESIGN-MIL2.md:332`) and the containment failure in the showcase footprint (`MIL2-PROPOSED-SEMANTICS.tex:227`) are both correct.

## 6. Residual assumptions

- **Custody.** I assume a Midnight contract can hold collateral B and read and write authenticated state cells through its public transcript. I did not re-verify this against the pinned ledger in this session.
- **Comparison width.** I assume `LessThan` at `bits < FR_BITS` (about 253 bits, `DESIGN-MIL2.md:169`) covers 193-bit comparisons without limbs. That still needs a U1 certificate.
- **Interest basis.** Simple interest on principal is an acceptable first product scope. Compound or index-based accrual stays U6.
- **Consent.** Checked by the ledger or wallet signature over the digest, not in-circuit.
- **Caps.** The Φ₀ caps (`DESIGN-MIL2.md:274-283`) are unmeasured. `divlit` adds two comparison nodes and one witness per use.
- **External practice.** I did not use any external protocol standard as a Moriarty rule here. Every rule above is a Moriarty design recommendation.