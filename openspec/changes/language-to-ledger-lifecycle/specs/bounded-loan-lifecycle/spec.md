## ADDED Requirements

### Requirement: Complete source-driven lifecycle
One committed .mori agreement SHALL provide originate, accrue, repay and settle actions. A runnable demo SHALL drive each action through the public API or CLI with actual predecessor output. It SHALL not edit debt, allowance, work or history fields between steps.

#### Scenario: Principal100 lifecycle
- **WHEN** funded origination creates principal100, one rate1/10 period accrues10, and interest-first repayment30 succeeds
- **THEN** principal SHALL be80, accrued0 and outstanding80. A computed final settlement80 SHALL leave debt0 and Settled status.

### Requirement: Complete conservation observation
The independent oracle SHALL compare all balances, allowances, obligation fields, operation identities, ordinary state, ordered effects and work. The fixture SHALL start with borrower reserve10 and lender100, disburse100, then repay110 total. Final borrower/lender balances SHALL be0/110. Lender allowance remaining/spent SHALL be0/100; borrower allowance SHALL be0/110. Unrelated rows SHALL remain unchanged. Every stage SHALL debit the same carried work pool.

#### Scenario: Failed continuation and retry
- **WHEN** a continuation fails funding, a postcondition, period freshness or work checks
- **THEN** its complete predecessor state SHALL remain usable for a valid retry without lost liabilities or consumed success identities.

### Requirement: Repeatable developer example
The README SHALL include runnable check, format and lifecycle demo commands. The demo SHALL retain inputs and complete outputs, including a failed duplicate accrual and failed payment after settlement. Exact reduction counts SHALL be independently derived from the final source body, not copied from prior examples.

#### Scenario: Clean reproduction
- **WHEN** the documented command runs from the repository root on the reviewed tree
- **THEN** it SHALL reproduce the complete lifecycle and named negative outcomes without services or manual fixture mutation.
