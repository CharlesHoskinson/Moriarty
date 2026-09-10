# Independent candidate-04 integration source review

Verdict: **PASS within the source-review scope**, with no blocking defect found. This is not wallet execution admission or financial acceptance.

Reviewer: fresh `gpt-6-astra`, agent `/root/sp05_final_integration_audit`, independent of candidate author `/root`.

Candidate: `../candidate-04.json`, SHA-256 `3cc0849a1703c9fc552e6701b41f8560d7904ad0af8a543a507601deffc941f3`. All 34 file bindings were verified before review and again at report creation; there were no mismatches.

The actual production composition was inspected across `integrate-local`, `run-local`, `providers`, `prepare-deployment`, `receipt`, `financial-comparison`, and `proven-assets`. The builder and launcher have a separate review scope.

The receipt decoder now requires exactly one signature per input and verifies matching indexes against each owner's key and segment data. The native-byte regression covers correct two-owner signatures and rejects reordered, missing, and extra signatures. The standalone comparator pins both compact-runtime metadata and its entry before importing; its inspection adapter returns only a non-executable source-test result. Both unchanged-version byte mutations are rejected.

The integration cleanup fallback covers driver failure before a cleanup receipt, including the real driver's provider ownership guard. It attempts provider cleanup, then wallet stop if provider cleanup cannot run or throws, and closes the loaded assets. The regressions verify cleanup without deployment and preserve incomplete containment.

The prepared deployment binds the exact native constructor transaction before allocation of asset caps and is consumed once. Its successful private-state update order agrees with the inspected pinned SDK `submitDeployTx`. Providers retain cumulative gross-asset and native DUST reservations, check semantic preservation through balancing/signing/finalization, and accept only exact issued bytes for submission. The observer and comparator connect native action/UTXO effects, canonical finality, decoded state and reserves, and check fixed financial expectations and replay restrictions.

Fresh checks: 154/154 ledger tests, 52/52 baseline tests, and 3/3 retained generated-runtime tests, all with zero failures or skips. Exact commands and output hashes are in `gpt6-review.json`; outputs are `gpt6-ledger-tests.txt`, `gpt6-baseline-tests.txt`, and `gpt6-compiled-tests.txt`. The generated-runtime checks execute real retained generated JavaScript and mutate its resulting state/payout; their ledger transport, UTXOs, fee and finality records are synthetic.

The production provider explicitly reports incomplete containment, so the driver cannot return PASS solely because four comparisons pass. The integration retains the result without financial, network or proof acceptance. Direct runtime pins do not attest transitive dependencies, and RPC/indexer trust remains external.

No compiler, full build, proof generation, wallet operation or ledger dispatch was performed. Mandatory PCD, financial settlement and execution admission remain outside this review. Startup status still reports unresolved operational history; review does not clear that stop. No other reviewer output was read and no production source was edited.
