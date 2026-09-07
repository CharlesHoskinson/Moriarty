# Preview public-network settlement — 2026-09-07

**Verified capability: public Preview deployment, contract call and exact state
readback.** The [combined receipt](settlement-2026-09-07T03-46-27-714Z.json) records
both transactions as SUCCESS, matches their indexer block hashes to node RPC,
and confirms both heights are at or below the finalized height of 755891.
The call settled in block **755889**, with transaction hash
`f79580fe0075dc26ae3f97f10557706f340cdf7a3b3118cd65a72b4fd870110a`.
Readback: `Moriarty Preview settlement test`.

**Original deployment observation.** The indexer returned
`SUCCESS` for transaction hash
`b38849c16cb1b628b2d6c88dcaa4ab5ec1d571ba1dbfd47547462f45a9798b7d`
in block755639. A separate node RPC read confirmed that block's canonical hash
and a later finalized height of755701 at03:27:23 UTC. See
[deployment-finality.json](deployment-finality.json) and the original
[indexer response](initial-contract-indexer.json). This is a historical
experiment observation, not a claim about current endpoint availability.

The initial deploy-and-call acceptance test **did not pass**. The first
`storeMessage("Moriarty Preview settlement test")` attempt was rejected by the
node with `Custom error: 170`. The official
[error table](https://docs.midnight.network/nodes/error-codes) identifies this as
`InvalidDustSpendProof`. Its documented regeneration remedy justified one fresh
attempt; that attempt failed locally with `could not balance dust`. At that point no call
receipt existed. The CLI read back an empty message, and the combined verifier
exited1 because the latest indexed action was still `ContractDeploy`.

**Recovery observation.** Read-only diagnostics found the DUST coin hidden by
an internal reservation even though the SDK pending list was empty. Processing
the grace period on a temporary copy exposed the coin; live timers were not
changed. Wallet and chain DUST parameters agreed. The
[inspected SDK sources](dust-source-inspection.json) show automatic revert on
submission failure, pending-list filtering through exposed ledger UTXOs, and
snapshot restoration with an empty SDK pending list. This explains why an empty
pending list alone was insufficient evidence of released DUST. The exact original
race was not captured; no upstream SDK bugfix is claimed.

Replayed only DUST in a separate private directory, retaining the same seed and
copies of the shielded/unshielded snapshots. The first scan reached its 240-second
cap and saved offset171818; one bounded continuation completed the remaining
history. The [recovered view](diagnose-2026-09-07T03-43-57-686Z.ndjson) exposed the
sequence1 coin. One instrumented submission then succeeded; see
[call output](call-2026-09-07T03-45-46-311Z.ndjson). The runner waits for fresh sync
before balancing and retains finalized transaction bytes privately before
submission. No proof-server, SDK version, key or deployed contract changed.
The cause of the original error170 is still unconfirmed; recovery plus a fresh
call succeeded, which does not prove that future proof rejection is impossible.

After finality passed, the recovered DUST snapshot was promoted to the original
runtime directory. The [promotion receipt](dust-state-promotion.json) names the
preserved original snapshot; no private state is in this repository.
A [fresh restore](diagnose-2026-09-07T03-47-02-821Z.ndjson) reported
all three children restored, a spendable sequence2 DUST coin, and no pending coin.
See the [bounded follow-up record](dust-followup-plan.md).

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
  combined gate. The subsequent combined receipt above records the passing call path.

This test used the pinned scaffold and local proof server from the
[Docker package](../moriarty-midnight-network-2026-09-07/README.md), with the public
Preview RPC/indexer from the wallet receipt. The bounded public experiment
initially stopped after deployment and two failed call attempts. The authorized
follow-up completed the call and finality gate. Native R3 was not rerun.
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
submission. The successful contract call, exact readback and combined finality gate also passed.

For a subsequent call to the existing deployment, run the existing CLI from the
external Preview runtime directory, set `MIDNIGHT_WALLET_SEED_FILE` to the
Preview seed path in the receipt, and explicitly pass `--network preview`.
Set `MIDNIGHT_INDEXER_URL`, `MIDNIGHT_INDEXER_WS_URL`, `MIDNIGHT_NODE_URL` and
`MIDNIGHT_PROOF_SERVER_URL` to the receipt's matching configuration. Do not rely
on the scaffold's default `undeployed` network or a previous sticky network.

Read-only combined check (passed on the recorded chain state):

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


To submit this same test message again when authorized, run the instrumented
helper with the existing runtime as the working directory. It requires
`--submit`; no automatic retry is performed. The installed R3 worktree remains
a runtime dependency. The separate `preview-diagnose.mjs` helper is read-only
with respect to chain transactions and saves wallet sync state privately.

```sh
cd /home/charl/.local/share/moriarty/test-wallets/preview-runtime-20260907
/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/.bin/tsx \
  /home/charl/Moriarty/experiments/moriarty-midnight-network/preview-call.mjs --submit
```
