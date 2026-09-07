# Preview public-network settlement — 2026-09-07

**Verified capability: public Preview contract deployment.** The indexer returned
`SUCCESS` for transaction hash
`b38849c16cb1b628b2d6c88dcaa4ab5ec1d571ba1dbfd47547462f45a9798b7d`
in block755639. A separate node RPC read confirmed that block's canonical hash
and a later finalized height of755701 at03:27:23 UTC. See
[deployment-finality.json](deployment-finality.json) and the original
[indexer response](initial-contract-indexer.json). This is a historical
experiment observation, not a claim about current endpoint availability.

The broader deploy-and-call acceptance test **did not pass**. The first
`storeMessage("Moriarty Preview settlement test")` attempt was rejected by the
node with `Custom error: 170`. The official
[error table](https://docs.midnight.network/nodes/error-codes) identifies this as
`InvalidDustSpendProof`. Its documented regeneration remedy justified one fresh
attempt; that attempt failed locally with `could not balance dust`. No call
receipt exists. The CLI read back an empty message, and the combined verifier
exited1 because the latest indexed action was still `ContractDeploy`.

The underlying cause of the invalid DUST proof remains unknown. The proof server
was8.1.0, with no mock flag, and served the deployment and call requests.
The wallet's coin reservation after rejection is a candidate explanation for
the subsequent balance error, not an established diagnosis. The SDK exposes
`revert`/`revertTransaction`; no rollback or wallet-state deletion was attempted.
Preserve the same wallet and deployed contract for a bounded follow-up that
inspects failed-transaction reservations and proof inputs before any retry.

Completed observations:

- Wallet received5,000tNIGHT (5,000,000,000 smallest units); see
  [funding-observation.json](funding-observation.json). No faucet txID was captured.
- All three persisted child wallets restored; full sync then completed.
- DUST registration submission returned, positive DUST became available, and
  deployment completed. The registration txID was not separately retained.
- Contract address:
  `ddc676b1bfb36f29665caf17b4ae5016595af74c3dc4165e638b6af77ef40b4f`.
- [deploy.log](deploy.log) is the complete deployment console output.
  `call-cli-partial.log` retains the first rejection;
  `call-retry-cli-partial.log` retains that output plus the fresh-attempt failure.
  These are partial PTY captures, not complete independent runs.
- [deploy-call-verification.log](deploy-call-verification.log) records the failed
  combined gate. The prepared verifier has not passed its positive call path.

This test used the pinned scaffold and local proof server from the
[Docker package](../moriarty-midnight-network-2026-09-07/README.md), with the public
Preview RPC/indexer from the wallet receipt. The bounded public experiment
stopped after deployment and two failed call attempts. Native R3 was not rerun.
Neither this deployment nor the local application checks establish Moriarty PCD.

The user selected Preview after the official documentation/network review.
The public test now uses the dedicated unshielded address in
[wallet-public.json](wallet-public.json). Its SDK Bech32m checksum, network/type
decoding and encode/decode roundtrip passed. The observation helper independently
re-derives the address through the hello-world wallet code and requires equality
before using its state stream. Keys remain in the external private seed file
referenced by that public receipt, with mode600; no secret is in this package.

Use https://faucet.preview.midnight.network/ for this attempt. The preceding
[network review](../midnight-network-review-2026-09-07/README.md) observed its
health endpoint reporting SERVING, while the Nethermind Preview URL linked by
the documentation returned503. A normal human CAPTCHA is required. No CAPTCHA
bypass or token request was performed by the observer.

`observe-*.ndjson` preserves timestamped sub-wallet progress and unshielded
balance independently of overall `isSynced`. This directly addresses the prior
Preprod helper's invisible-funding issue. Observation lasts180seconds and sends
no transactions. It uses the installed, tested scaffold's wallet persistence
functions with external storage and an owner-only umask. Persistence and restore
results must be read from actual run receipts; a helper call alone is not proof
that all three child wallets were saved or restored.

From the main repository root:

```sh
timeout --signal=TERM --kill-after=30s 210s \
  .worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/.bin/tsx \
  experiments/moriarty-midnight-network/preview-observe.mjs
```

The existing R3 worktree supplies the installed dependencies and compiled
contract tooling; keep it while this runtime is in use. Preview wallet state
is outside the repository under
`/home/charl/.local/share/moriarty/test-wallets/preview-runtime-20260907`.
The original two Preprod seeds are preserved. Their addresses are not Preview
addresses and must not be pasted into the Preview faucet.

The funded wallet and finalized deployment now satisfy public transaction
submission. A successful contract call with exact message readback remains open.

For a subsequent call to the existing deployment, run the existing CLI from the
external Preview runtime directory, set `MIDNIGHT_WALLET_SEED_FILE` to the
Preview seed path in the receipt, and explicitly pass `--network preview`.
Set `MIDNIGHT_INDEXER_URL`, `MIDNIGHT_INDEXER_WS_URL`, `MIDNIGHT_NODE_URL` and
`MIDNIGHT_PROOF_SERVER_URL` to the receipt's matching configuration. Do not rely
on the scaffold's default `undeployed` network or a previous sticky network.

Read-only combined check, expected to fail until a call actually settles:

```sh
node experiments/moriarty-midnight-network/preview-verify.mjs \
  ddc676b1bfb36f29665caf17b4ae5016595af74c3dc4165e638b6af77ef40b4f
```

Deployment finality was checked by querying `system_chain`,
`chain_getFinalizedHead`, `chain_getHeader(finalizedHash)`,
`chain_getBlockHash(755639)` and `chain_getBlock(canonicalBlockHash)`. Acceptance
required Preview identity, indexer `SUCCESS`, equal node/indexer block hashes,
the matching node block height and height no greater than finalized height.
The retained indexer query is in `initial-contract-indexer.json`.
