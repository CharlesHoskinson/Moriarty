Adds financial postconditions to `.mori` agreements. Source profile `/4` lets `ensures` inspect resulting debt, balances and allowances after one private kernel preparation. A failed condition publishes no ordinary state, financial state or effects. Existing source `/1`–`/3` and Core `/1`–`/2` behavior is preserved.

The implementation adds six `post_*` reads, integrated Core `/3`, CLI support, an executable repayment example, and updated README syntax and semantics. The example consumes exactly82 work units, retains closure reserve, and verifies complete repayment. This PR also records the ordered six-stage OpenSpec plan and reconciles the four previously merged capabilities at their implemented scope.

Validation:835 package tests, TypeScript checking,17 independent financial cases,25 Core cases,261 legacy comparisons, private-continuation and diagnostic-ownership regressions, and four actual README commands. A fresh Astra medium audit approved the exact candidate after61 additional adversarial assertions. Grok4.6 high authored the implementation and repairs; returned identity was `grok-4.6-build`.

Audit: `deliverables/language-to-ledger-2026-09-12/postconditions/audit-postconditions-01.md`. Candidate digest: `3aec887bfd4fa8485c886d2bd74c1f8afc7e5895ef3283551410accccdb31dc0`.

This establishes local source execution. Origination/accrual, the full lifecycle, K agreement, authenticated ledger integration and Docker/Preview acceptance remain subsequent stages. Existing proof and ledger gates remain open.
