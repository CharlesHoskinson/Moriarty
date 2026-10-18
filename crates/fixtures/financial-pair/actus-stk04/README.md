# STK04 independent semantic bridge

This fixture is consumed by the existing `financial_pair` integration target through public Rust Core/1 `parse`, `validate` and `simulate`. It is a finite local semantic case, not an all-field ACTUS fixture pass.

The exact unchanged source/opening scenario/call operands came from input custody manifest SHA256473ef396c4a43f38e53ff66b97d26ee4dfb8fc53c5b021bd6e018b318f17c5a1. The separate independent custodian froze the complete semantic two-call observations without reading the historical author-derived oracle/test, evaluator outputs or upstream reference result values. The independent packet is preserved under independent/, with its original manifests and byte hashes. No frozen payload was edited or generated from a product run.

The test consumes independent oracle.json directly. It checks both complete normalized pre/post snapshots; every typed Step.value, ordinal, specified statement member, snapshot, local delta and ordered effect prefix; both complete Execution.effects arrays and the combined eight-effect history. It creates the second scenario from the actual first poststate. Expected snapshots are comparison-only. The four independently specified ACTUS semantic fields are eventDate, eventType, signed payoff and currency, projected mechanically from the actual Transfer/Emit effects. The exact original actus-tests-stk.json remains retained with all members/fields; the independent test does not read or copy its result values.

Three present ACTUS result mappings remain explicitly UNASSERTED: notionalPrincipal, nominalInterestRate and accruedInterest. The source's zero-valued Core state slots are fully compared as Core state and do not establish these ACTUS mappings. This distinction is required by independent/reference-field-dispositions.json and UNASSERTED.json. No fixture/family is counted passing. All277/18/32/DS01–07 and parent requirements remain.

All numeric ExprId values, whole Program/Execution wire equality, exact CLI bytes and actual work/cap/CostReport remain unasserted. The test removes only the independently enumerated numeric ExprId fields from statement comparison, requiring their typed presence; all remaining members must match exactly. It selects no metered invocation. Equity delivery, general scheduling/EOM/calendars, dividends, accrual/day-count, target/proof/ledger/native fees and settlement are outside this case.

The unchanged ACTUS tests file is supplied by ACTUS Financial Research Foundation, https://github.com/actusfrf/actus-tests, revision f7a8064872b69db1f0beabac771c99dc3ce0c397, under CC BY-SA4.0; see ACTUS-LICENSE.md. Preserve attribution/share-alike requirements for this adaptation. The prior author-derived proposal remains historical and is not this independent freeze.

Candidate preparation executed no product/Cargo/SDK/VM/proof/wallet/network command. Any future result must identify this exact source and the unchanged independent freeze.
