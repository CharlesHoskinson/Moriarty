# Local loan attempt: deployment rejected

The actual local `undeployed` run restored the existing wallet, finalized native deployment bytes and reached node submission. The node rejected transaction `09824832f0174603938ee7f51b368019ef1def3774adff746d2caa847f790864` with `Malformed(OutOfDustValidityWindow)` (RPC custom error 171). No financial stage completed. This is not Preview settlement or proof/financial acceptance.

[The result](attempt-result.json) binds retained public events, native bytes and reservations. One submission and 300000000000001 SPECK remain reserved; this attempt cannot retry. [Terminal containment](terminal-containment.json) independently records an empty launcher cgroup and all three containers stopped, 149.486 seconds after timer arming. The inner cleanup result remains incomplete in the original integration record.

[Native inspection](native-time-inspection.json) found DUST creation time 2026-09-08T12:55:36Z, while [node rejection](node-rejection.json) occurred at 2026-09-10T03:58:39Z. The transaction TTLs were current. Historical receipt/finality checks passed, but [preflight](live-chain-preflight.json) did not check latest-block age.

[Inspected source](../../../raw/sp05-error171-source-2026-09-10/receipt.json) shows the installed SDK asks the indexer for `block(offset: null)` and uses its timestamp when balancing DUST. Wallet synchronization checks event progress, not timestamp freshness. A stale indexer response after startup is the leading explanation; that response was not captured, so its exact origin remains an inference. The node log establishes the actual rejection. Release-tag source explains code 171 but is not proven identical to the container image's unavailable source commit.

Next repair: require a fresh indexed block matching the finalized node chain before balancing. Preserve existing seed, snapshots, Docker layers, contract state and failed native bytes. Any successor execution needs a new bounded allocation that retains these charges; no automatic resubmission or fresh-wallet workaround.

Grok's required resource review completed in 300.524 seconds with approval; its terminal result is retained in [the provider result](grok-resource-result.json). The independent source reviews and resource votes enabled this attempt, but cannot establish ledger acceptance.
