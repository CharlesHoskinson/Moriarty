# U0 numeric profile decision

Status: decided 2026-09-23 by the project owner for roadmap milestone U0. This file is the only source of policy values for `deliverables/u0-semantic-contract-2026-09-23/numeric-profile.json`; the profile may refine these decisions into per-primitive rows but may not change them.

## D1 — Canonical price orientation

Moriarty's canonical price orientation is **base-per-quote**, the existing `moriarty-financial-agreement-source/5` convention. A price `Price<A,B,S>` states how many units of the base asset one unit of the quote asset is worth, at the declared scale.

DeFiFormal and other quote-per-base sources are adapted only through an explicit, dimensioned conversion with directed rounding at the boundary. A reciprocal, rename or implicit reinterpretation is not a conversion.

## D2 — Rounding direction and beneficiary policy

The default policy is **protocol-favoring**:

- Any computed amount a party owes, or that increases or preserves a liability (interest accrual, repayment due, fees, collateral requirements, debit caps checked against a computed charge), rounds **up** (`ceil`).
- Any computed amount a party receives, or that reduces a liability in the payer's favour (payouts, withdrawals, conversions into a received amount, refunds, liquidation proceeds to the borrower), rounds **down** (`floor`).
- Exact results use no rounding (`none`) and must remain exact; a primitive that can lose precision may not use `none`.
- The rounding remainder (dust) accrues to the **protocol reserve**. It is never silently dropped, burned or credited to the prover or solver.

Per-primitive overrides are allowed only when declared explicitly in the profile with a rationale. Later libraries (for example ACTUS in U6) must declare their overrides the same way; none are declared in U0.

Where the current source/5 implementation lets a program author select `none`, `floor` or `ceil` independently of this policy, the profile records that primitive's required direction and marks the author-selectable behaviour as an open conformance gap. U0 does not change the implementation.

## D3 — Scope of the U0 primitive slice

The U0 numeric profile covers exactly the arithmetic primitives the current successor implementation (`experiments/moriarty-language/src/successor/`) actually implements. AMM, vault, ACTUS and other library primitives are U6 scope and are not listed.

## D4 — Units

Amounts are exact domain-qualified integers in each asset's smallest unit, with checked finite-width arithmetic. They never become field elements modulo the proof field. Each asset's decimals and domain are part of its identity.
