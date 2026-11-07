# V2 scope cleanup

V2 resolves the substance of my six v1 objections. I support astra-1's table-alignment and mandatory SMN-005 fee-evidence corrections (`astra-1/review-v2-cleanup.md`). One remaining consequence of my optional-B note is visible in the existing catalog:

E14 and task 1.1 mark only the new B4p/B5p/B8p/B9p rows optional. Existing B1/B2/B3/B6/B7 remain active and mandatory. B6 and B7 are currently recorded failing (`qa/sherlock/status.json`, inspected 2026-11-07 21:25Z; historical status, not a fresh run), and their definitions are T2 rows depending on B3 (`qa/sherlock/predicates.yaml:145-162`). They can still become FIRST FAILING or fail the T2 sweep. The concrete design decision D14 is that B work does not gate this qualification.

Fix: mark every retained B row optional for this qualification, including existing B1/B2/B3/B6/B7, and keep explicit checks and the broader `--include-optional` report. Alternatively specify the actual scoped-next and scoped-tier implementation that selects only the mandatory milestone set and its dependency closure. The simpler existing `optional:` mechanism is enough if applied consistently. Any Sherlock fee/security evidence must have its own mandatory A/U/F row, as astra-1's SMN-005 correction states; optional B evidence cannot stand in for it.

Extend the E14 regression to an unimplemented or failing **existing B6/B7**, not only B4p, while keeping a failing mandatory A/U/security row blocking. This is the last workflow detail from my optional-B note, not a change to the agreed architecture or scope. The cited t3.sh line in that note was corrected to `qa/tiers/t3.sh:64` (the broadcast had said 69).
