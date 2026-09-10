# Explicit Preview financial stage driver

Source-only slice in `/home/charl/Moriarty/.worktrees/sp05-preview-owner`: modified `ledger/run-local.mjs`, new `ledger/run-preview.test.mjs`. Source hashes in `/tmp/moriarty-preview-driver-source.json`. All four owner files and three RPC files were independently rehashed and remain byte-identical to their frozen candidates. No publication, live operation or admission.

Private shared sequence is selected only by explicit `runLocalFinancialCase(options)` and `runPreviewFinancialCase(options)` wrappers. Local network guards, endpoint restrictions, recovery/continuation behavior, result schema and default observer arguments remain intact. Preview requires both requested and SDK/provider network to be preview, exact configured Preview node/indexer/indexerWS targets, existing local HTTP proof endpoint policy, matching payer and stable full provider binding before every SDK action. It refuses local deployment recovery and initialized loan/swap shortcuts.

Both targets use the existing guarded provider.execute calls, exact deploy/initialize/accrue/settle or deploy/initialize/swap/close arguments and hints, observe then mandatory PASS comparison before the next action, existing reservation/public-ID projection, and owned provider cleanup. Preview adds network:preview to observer arguments so the reviewed receipt selector can be connected explicitly by the future integration. Its result schema is moriarty.preview-financial-run/1; the local durable writer is deliberately not extended here. Scope remains uncertified I2, not mandatory PCD acceptance.

Tests first failed on the missing Preview export. Focused GREEN69/69; composed regression164/164, no skips. Existing local exact argument assertions remain green, and new Preview tests compare the actual sequence arguments against the existing local path. New controls reject wrong target/SDK network/payer/endpoints, changed binding, local history shortcuts, failed observation/comparison or guarded execution, and incomplete cleanup. Synthetic roles and inert SDK/provider callbacks exercise callable production sequencing; they do not produce actual proofs or chain effects.

Command from repository root:

```
node --experimental-test-module-mocks --test experiments/moriarty-midnight-financial/ledger/run-preview.test.mjs experiments/moriarty-midnight-financial/ledger/run-local.test.mjs experiments/moriarty-midnight-financial/ledger/initialized-swap-driver.test.mjs experiments/moriarty-midnight-financial/ledger/integrate-local.test.mjs experiments/moriarty-midnight-financial/ledger/initialized-swap-integration.test.mjs experiments/moriarty-midnight-financial/ledger/integration-negative.test.mjs
```

Raw RED/GREEN/regression and baseline logs `/tmp/moriarty-preview-driver-*.tap`; git diff --check passed. Future integration must still bind actual Preview genesis/protocol/build/roles/identity, real financial observer/comparator, provider reservations, durable results and separately admitted execution. Configured hostnames do not establish current availability or chain identity. No private inputs, wallet, network, services, compiler, prover or transaction dispatch used.
