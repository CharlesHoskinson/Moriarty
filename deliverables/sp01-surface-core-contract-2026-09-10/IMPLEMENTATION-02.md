# Boolean contract revision implementation plan

Goal: implement approved short-circuit And/Or in the proposed expression contract /1, with unchanged exhaustive admission and finite work.

Design authority: scoped-design-approval-01.json binds the two independent votes on source-candidate-01.json. Fresh result review is still required.

- [x] Preserve the original bytes of each modified file named in the published 101-file expression candidate; map all old pins to unchanged or archived paths.
- [x] Revise semantic-contract.md, static-semantics.md and expression-signatures.json together: separate generic strict contexts from And/Or, zero-cost administrative contraction, original paths/spans, one entry charge and conservative bounds.
- [x] Give changed expression cases revision-specific identities; preserve the strict-And rejection under its historical identity. Add canonical specified Boolean cases with exact constructor syntax, work, node paths, synthetic/source spans, admission failures and complete-action cases.
- [x] Update the structural checker version and metadata checks; run existing and focused structural negative controls. No interpreter, parser execution, K or proof run.
- [x] Verify all historical pins, inspect the complete patch, and bind the new source bytes plus checks in source-candidate-02.json for fresh independent audits. No commits before root review.
