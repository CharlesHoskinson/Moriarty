
## September 12 language delivery reconciliation

The following capabilities are implemented, tested and merged into main through `dc9d516eabb5757591e6ef73a7544639cbe55134`. Their status is S4 local implementation. They do not close full SP02, SP03, authenticated authority, K correspondence, proofs or Preview acceptance.

| Capability | Merged change | Current scoped evidence |
| --- | --- | --- |
| Computed funded repayment through simulation CLI | PR1, `e6dc9f68bdb30adaed5adea6838549e7b457dc28` | [Result](deliverables/expression-funded-repayment-2026-09-11/RESULT.md), [final audit](deliverables/expression-funded-repayment-2026-09-11/audit-01/review.json) |
| Source-defined repayment schemas | PR2, `ea40ab488d1d056c7192cc4dfd7ae250d44e3d49` | [Result](deliverables/source-defined-repayment-2026-09-11/RESULT.md), [final audit](deliverables/source-defined-repayment-2026-09-11/audit-02/review.json) |
| Multiple named actions with explicit selection | PR3, `96860344978e176236a6f152cd694433263c8d5f` | [Result](deliverables/multiple-named-actions-2026-09-11/RESULT.md), [final audit](deliverables/multiple-named-actions-2026-09-11/audit-01/review.json) |
| Typed financial PRE reads and computed remaining repayment | PR4, `98f6d59f11e163325256766a08d2d3f119d102ed` | [Result](deliverables/financial-state-reads-2026-09-12/RESULT.md), [final audit](deliverables/financial-state-reads-2026-09-12/audit-01/review.json) |

The user-approved [language-to-ledger plan](docs/superpowers/plans/2026-09-12-language-to-ledger.md) and [OpenSpec package](openspec/changes/language-to-ledger-lifecycle/README.md) now order financial postconditions, origination/accrual, a complete lifecycle, scoped K agreement and authenticated Docker/Preview execution. These new behaviors are S2, specified-only until their independent result evidence is recorded. Existing wider MC/SP acceptance and resource gates remain unchanged.
