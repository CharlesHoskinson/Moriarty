# Proposed independent recovery financial examples

Two newly chosen economic examples are frozen here, without using proposed recovery runtime/test expectations or observed outputs. They are proposed normative outcomes, not adopted recovery semantics, executed tests, whole recovery wire goldens, or authority judgments.

| Example | Prior committed effects retained | Proposed new remedy | Closing residual |
|---|---|---|---|
| Custody refund | Buyer funded60; fee4 remains at treasury | Custody returns60 to buyer; explicitly discharge60 | Refund duty present0; unrelated audit duty4 BOND |
| Forward compensation | Supplier retains completed payment40; treasury retains fee3 | Separate reserve pays buyer12; explicitly discharge12 of remedy20 | Remedy8 PAY plus unrelated audit duty4 BOND |

Each directory contains newly authored `primitive-actions.proposed.mori`, exact start/scenario and false-event input bytes, a documentary ordinary-remedy-body scenario, and complete proposed financial frames/effects/accounting in `financial-expectations.proposed.json`. All amounts are atomic units, both exact assets and every preserved balance cell are present, and no balance is silently treated as dust or recoverable principal. The unrelated duty has a declared, uninvoked source identity.

The source illustrates existing Core/2 primitives and an ordinary separate remedy action. That separate action is not asserted to implement recovery of the pending continuation. Original pending-frame disposal/update, subsequent original events, status/identity/bytes, recovery overhead and committed-work convention remain named unknowns. No new language keyword, wire version, semantic identifier or invented rejection code is introduced.

For each example, the ordinary prefix is7 work units and the selected normal false guard is2 attempted units with committed7 unchanged. The separate transfer/discharge body is4 primitive units if accepted as an ordinary action. Their arithmetic sum13 is not a recovery lifetime meter or a new committed-counter rule. Recovery and failure-specific costs remain unspecified.

`DERIVATION.md`, `MISSING-PREMISES.json` and `SOURCE-PROVENANCE.json` explain the chosen policies, public-source exposure, phase/failure limits and historical exposures. Source provenance is pinned by exact bytes; checkout74a909 is parent-reported, not independently queried through Git. `PAYLOAD-INVENTORY.json` and `FREEZE.sha256` bind every payload. Product/build/parser/evaluator/compiler/CLI/native/provider/network execution remains unrun. Existing freezes, failures, counters, all14 staged gaps, every composition mode and all original roadmap exits remain unchanged.
