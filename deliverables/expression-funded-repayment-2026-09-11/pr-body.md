Financial expression source currently produces descriptors, while funded repayment uses a separate restricted source path. This adds an explicit `simulate --repayment-state` mode that computes and checks a payment, prepares the full ordered Transfer/Repay batch through the existing kernel, and publishes ordinary and financial results together only on success.

The same `.mori` example supports different snapshot arguments: a payment of 30 leaves principal 70; an interest-first payment of 7 against principal 100 and interest 10 leaves principal 100 and interest 3. Unit bindings, funding reuse, residual obligations and combined work accounting are checked. Existing pure simulation remains unchanged.

Validation: 698 language tests pass; TypeScript checking passes; root independent checks cover 19 API and 3 actual CLI cases. Fresh GPT-6 Astra medium approved the final 149-file candidate after 113 focused tests and 44 independent probes. Grok 4.6 authored the implementation (returned `grok-4.6-build`); the final audit and evidence are in `deliverables/expression-funded-repayment-2026-09-11/`.

This is local preparation only. It does not establish authenticated authority, K correspondence, proof-carrying acceptance or Preview settlement.
