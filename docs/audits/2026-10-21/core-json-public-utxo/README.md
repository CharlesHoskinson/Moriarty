# Public UTXO Core JSON integration

The Core JSON reader now accepts caller-selected byte, value-node and depth budgets through `parse_budgeted`. Legacy `parse`, `parse_limited`, `deserialize_strict` and `export` remain separate APIs. The contract is in [core-json-budget-contract.md](../../../core-json-budget-contract.md).

The six source, contract and fixture files retain the exact hashes covered by the saved source-only review. Relative to the previous public branch, this adds the budget parser implementation as well as documentation and tests.

Local validation compiled the public worktree Core library afresh, then linked and ran both complete JSON integration-test targets against that library. Eleven coverage functions and three fixture functions passed; no tests were filtered. The fixture target retains 24 witnesses and 38 cases. The build used direct rustc with pinned cached dependencies; Cargo and the full workspace checks were not run.

[VALIDATION.json](VALIDATION.json) records the source hashes, commands, results, raw test streams and remaining checks. Independent review of these new execution results is pending. The prior source review does not certify the whole compiler, resource adequacy, financial behavior or proof soundness. No SDK, proof, wallet or ledger operation was selected, and no EARS requirement is closed by this component record.
