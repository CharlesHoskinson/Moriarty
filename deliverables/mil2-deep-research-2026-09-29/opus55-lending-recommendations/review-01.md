# Lending and borrowing: debt and payment semantics (independent recommendation)

**Scope.** This is read-only design review. I claim no implementation, proof or ledger acceptance. Guarded `status --json` reported `SP01.6 loan-swap-subset`. Implementation is blocked by stale inputs, missing current accounting and unresolved operational history. There are no pending transactions. The captured ACTUS text (`source-text/actus-techspec.md:40-44`) is only a landing page. My ACTUS remarks below come from the linked official specification and are labelled as **external practice**.

## 1. Verdict on category design fitness

**The debt carrier is fit for a bounded repay slice. It is not fit to freeze for the debt lifecycle.** MIL/2 gets four structural choices right:
- debt is a separate `Obligation` record, not supply (`DESIGN-MIL2.md:94-95`; product contract `:43`);
- AccrualFirst is restored (`DESIGN-MIL2.md:100`);
- liabilities have their own roll-forward (L1), with forgiveness kept separate (`MIL2-PROPOSED-SEMANTICS.tex:180-187`);
- `enforce` is bounded by debtor-consented policy (`DESIGN-MIL2.md:193`).

Five gaps remain:
- `outstanding` is a free stored field.
- `ConsentRef` is an unauthenticated pointer, and it covers both origination and enforcement (`:95`, `:100`, `:193`).
- There is no obligation status, maturity or default transition. The predicate list has `discharged(o)` but no `defaulted` (`:148`). R2 H1 is still open (`R2-lending.md:458-461`).
- "Corresponding funded transfer" (`.tex:187`) names no payer, recipient, phase or asset binding.
- Nothing says who may authorise forgiveness.

**I disagree with the category review's "Covered" grade for origination** (`02-lending.md:17`). Without funded disbursement and authenticated one-shot consent, it is **partial**.

## 2. Five ranked design edits (recommendations)

### R1. An obligation state machine with derived `outstanding` and a discharge tombstone (highest priority)

**Rule sketch.**
- Obligation state: `{p, a, status ∈ {open, defaulted, discharged, written_off}, lastAccrual, termsDigest}`.
- `u` is defined as `p + a` and never stored independently. If kept on the wire, `TypedBounded` rejects any cell where `u ≠ p + a`.
- Allowed transitions:
  - `Originate: ∅ → open`
  - `Accrue: open|defaulted → same`
  - `Repay(n): open|defaulted → same | discharged`
  - `Default: open → defaulted`
  - `Forgive(f): open|defaulted → same | written_off`
- `discharged` and `written_off` are terminal. Each writes a one-shot tombstone, the same mechanism as the escrow tombstone (`.tex:192-197`).
- `discharged(o)` holds only if the most recent transition was `Repay` with `n = u`. `written_off` is a distinct state, so forgiveness is never reported as repayment (consolidated design `:54`).
- L1 (`.tex:180-184`) holds on every transition, including failure transitions with all deltas zero.

**Counterexample.** A stage writes `outstanding = 0` while leaving `p = 1000`, and a downstream guard `discharged(o)` releases collateral. Today, L1 constrains `u'` but nothing links `discharged` to the method of reaching zero.

**Placement.**
- **U0:** cell record, status enum, L1 as a `history`/`effect` judgment clause. This is consistent with F4's complaint that the schema admits liabilities without components (`UNIFIED-PROPOSAL.md:35`).
- **U2:** `Repay`.
- **U3:** `Default` and persistence under partial progress (`ROADMAP.md:24`).

### R2. Funded repayment is a paired effect in the same phase

**Rule sketch.** `Repay(o, n, payer)` requires all of the following:
- `0 < n ≤ u`;
- the effect set contains exactly one `transfer{domain = o.domain; asset = o.asset; from payer; to creditorSink(o); amount n}`, where `creditorSink` is fixed in `termsDigest` (the creditor account or program custody);
- `dA = min(n, a)` and `dP = n − dA` (`DESIGN-MIL2.md:100`);
- the discharge and its paired transfer sit in **the same Midnight phase**, so a failed fallible phase retains neither (product contract `:47`);
- a third-party payer needs its own spend authority but not debtor consent. Subrogation (the payer acquiring the claim) is **not** implied and needs a separate consented `assign` transition.

Overpayment is rejected, not refunded as residue, consistent with `.tex:187` and `DESIGN-MIL2.md:258`.

**Counterexamples.**
- Discharge 30 is paired with a 30-unit transfer to an account the debtor controls.
- The transfer uses the wrong asset.
- The transfer is in the fallible phase and fails, while the discharge was booked in the guaranteed phase. This is erased debt (`ROADMAP.md:40`).

**Placement.** U0 `effect` clause; U2 native enforcement; U3 phase-pairing hostile controls.

### R3. Origination consent as a typed, one-shot, digest-bound record, with funded disbursement

**Rule sketch.**
- `Consent ::= {debtor, creditor, asset, principalCap, termsDigest, accrualRule, roundingPolicy, maturity, grace, defaultPolicyDigest, enforcePolicyDigest, validity, nonce}`.
- Signature authentication follows `DESIGN-MIL2.md:263-264`: the digest is a public input, and the ledger checks the signature. It is **not** checked in-circuit.
- `Originate` requires, in the same stage:
  - a fresh `replay(nonce)`;
  - `creation = p₀ ≤ principalCap`, `a₀ = 0`;
  - a paired disbursement transfer of `p₀` from the creditor or reserve to the recipient named in the consent.
- Enforcement consent is a separate digest field, so `enforce` cannot be widened by reusing a borrow consent.

**Counterexamples.**
- A creditor replays Alice's earlier consent to create a second 1,000 debt.
- Phantom debt is created with no disbursement.
- A keeper cites a borrow consent that never named a liquidation bonus.

**Placement.**
- **U0:** record fields, replay cell, stage public-input binding (`.tex:240`).
- **U2:** at `|S| = 1` the debtor is the stage signer and the creditor side is program-custody reserve policy.
- **U3:** bilateral peer-to-peer origination uses a pre-signed creditor offer as an escrow template.

I do **not** change decision 4 (`DESIGN-MIL2.md:336`). Full two-signer origination waits for `|S| > 1`. This resolves R2's H2 without widening U0 (`R2-lending.md:464`).

### R4. Accrual as a checked, period-indexed transition with role-directed rounding, in Φ₀ only

**Rule sketch.** `Accrue(o, k)` requires:
- `k = lastAccrual + 1`;
- `after(periodEnd_k)` on the obligation's clock;
- status ∈ {open, defaulted}, with the rate chosen by `termsDigest`.

The interest `q` is a **witness**, checked with literal-coefficient cross-multiplication (legal in Φ₀, `DESIGN-MIL2.md:140,160`). Using ceiling rounding in the creditor's favour:

`q × den ≥ p × num` and `(q − 1) × den < p × num`

with the limb rule wherever `p × num` can exceed field width (`:169-171`). The rounding direction (ceiling to the creditor or floor to the debtor) is part of `Consent.roundingPolicy` and the numeric profile's beneficiary policy (`ROADMAP.md:21`). Ceiling accrual adds to a liability, as F10 notes (`UNIFIED-PROPOSAL.md:41`). The effect is `a' = a + q` and `lastAccrual' = k`. There is no compounding: index, variable-rate or compound accrual stays Φ₁/U6 (`DESIGN-MIL2.md:161`).

**Counterexamples.**
- The same period is accrued twice.
- Floor rounding on tiny principals produces zero interest, so splitting a loan makes it interest-free.
- Accrual continues after `discharged`.

**Placement.** U0: numeric-profile row for the accrual primitive and its rounding owner. U1: certificate. U3/U6: execution. The S0 slice excludes accrual (`UNIFIED-PROPOSAL.md:73`), and this recommendation keeps it excluded.

### R5. Default, maturity and forgiveness authority as explicit transitions that never change the amounts owed

**Rule sketch.**
- `Default(o)` requires:
  - `status = open`;
  - `after(maturity + grace)` on the anchored clock, **or** a missed scheduled payment `k` under `defaultPolicyDigest`;
  - `u > 0`.
- Its effect is `status' = defaulted` with `p' = p` and `a' = a`. It enables the `enforce` scope in the consented policy. Default is a status change, not a discharge.
- `Forgive(o, f)` requires the **creditor's** `amend` right scoped to `obligation(o)`. It must never be the debtor's or a keeper's right. It needs `0 < f ≤ u`, applies AccrualFirst or a declared component order, and moves to `written_off` only when `f = u`.
- Add the atom `defaulted(o)`. It is denotable from the status cell, so it satisfies the removal criterion at `DESIGN-MIL2.md:156`.

**Counterexample.** Seizure recovers 800 against 1,000. The stage marks `discharged`, erasing 200. Under R1 and R2 this is rejected because `n = 800 ≠ u`. The residual 200 persists as `defaulted`, and only a creditor-authorised `Forgive(200)` or a loss-allocation library can remove it.

**Placement.** U0 freezes the status enum, the `defaulted` atom and the `amend` sub-scope. **Changing a U0 boundary:** this folds forgiveness into `amend` as a scoped sub-right instead of adding a ninth right to the U0-frozen enum (`DESIGN-MIL2.md:191`, `:345`). I recommend it because retrofitting a right kind after the header freezes is costly, and a sub-scope keeps the enum stable. The transition belongs to U3.

**External practice (not a Moriarty rule).** The ACTUS technical specification (https://www.actusfrf.org/techspecs; PDF https://www.actusfrf.org/_files/ugd/3df5e2_11de48b7dffd47758c729f21e9d5219a.pdf) models contract performance as distinct states: performant, delayed, delinquent, default. Grace and delinquency periods and credit events move between them, and notional principal and accrued interest are tracked separately. This supports a status machine separate from the balances. I did not verify these details in the local capture.

## 3. Core versus library boundary (recommendation)

**Core (the frozen relation):**
- the obligation cell, status enum and tombstones;
- L1 on every transition and phase;
- AccrualFirst allocation;
- funded-pairing for `Originate` and `Repay`;
- the `Consent` record, replay and separate enforcement digest;
- `defaulted(o)`;
- the forgiveness authority scope;
- the rule that no transition other than `Repay` or `Forgive` reduces `u`;
- debt excluded from E1 (`.tex:171-178`).

**Library (U6):**
- amortisation and annuity schedules (ACTUS PAM/ANN-style);
- grace, cure and default-rate parameter values;
- penalty fees;
- liquidation economics (close factor, bonus, auction);
- rate models and indices;
- impairment and loss waterfalls;
- assignment and subrogation policies.

Libraries may only *instantiate* core transitions. They cannot add a way to reduce `u`.

## 4. Smallest implementable slice and evidence pair

This is S0 Program A, a funded repay with AccrualFirst (`UNIFIED-PROPOSAL.md:68-78`), with R1 and R2 applied. It has one signer, one domain, one asset and no in-stage accrual.

**Positive control.**
- Before: `o = {p 1000, a 10, open}`, so `u = 1010`.
- Alice signs `Repay(o, 30)`.
- Effect: `transfer{USDC; Alice → creditorSink(o); 30}`.
- After: `a' = 0`, `p' = 980`, `status open`.
- E1: Alice −30, sink +30, supply delta 0.
- L1: `980 = 1010 − 30`.
- The stage must be valid and feasible, and its acceptance must be native (`ROADMAP.md:40`).

**Hostile control.** The same bytes and signature, except the transfer recipient is changed to a second account Alice controls. The discharge stays 30.
- Without R2, E1 still holds and L1 still holds, so a relation checking only conservation and roll-forward *accepts* it. That shows the missing clause.
- The required result is native rejection at the `effect` judgment for recipient mismatch with `creditorSink`.
- The envelope must be well-formed, so the rejection comes from the semantic check itself.

**Second hostile control.** Principal-first allocation (`p' = 970, a' = 10`) must fail `ALLOCATION_COMPONENT` natively. The existing K and evaluator code has this rule (`UNIFIED-PROPOSAL.md:33`), but only at local scope.

## 5. Explicit disagreements

1. **Category review:** origination is partial, not covered (`02-lending.md:17`).
2. **Design §3.4:** `outstanding` should be derived, not a free field (`DESIGN-MIL2.md:94`).
3. **Design §6:** one `consent` reference for both borrowing and enforcement is unsafe. Split the digests (`:193`).
4. **PDF (C1) negative corpus:** add "retain discharge after its paired transfer's phase fails" and "consent replay" to the list at `.tex:127`.
5. I **disagree with treating "default" as library-only.** Its *non-erasure* is a core invariant, so its status must be core. Only its trigger parameters belong in a library.

## 6. Residual assumptions

- Ledger signature checking over the consent digest is available at U2. `DESIGN-MIL2.md:264` rules out in-circuit Ed25519.
- The phase layout lets a transfer and an obligation write be placed in the same phase atomically. This is unverified against the pinned ledger.
- `creditorSink` can be fixed at origination. Creditor assignment is deferred.
- The widths of `p × num` under literal rates fit the limb rule. Caps are unmeasured (`DESIGN-MIL2.md:270`).
- The anchored clock is trusted for maturity. `fresh` does not bound when an actor arrives (`:187`, `:219`).
- The ACTUS performance-state details come from the official specification, not the local capture.