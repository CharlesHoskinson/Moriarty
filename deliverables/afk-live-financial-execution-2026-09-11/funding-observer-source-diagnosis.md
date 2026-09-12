# Nonpersisting existing-wallet funding observer: source diagnosis

Read-only source inspection, 2026-09-11. No wallet/seed/snapshot contents were read, no SDK module was executed, and no network, proof, transaction, service or wallet connection was started. The proposed observer is not implemented or validated by this note.

## Feasible narrow consumer

The installed SDK supports restoring existing child-wallet serialized states into memory, starting indexer synchronization, observing funding, and stopping without persistence. Prefer direct existing `DustWallet` and `UnshieldedWallet` child APIs for this narrow observation; they avoid the facade default submission/proving service initialization and do not need the shielded wallet for unshielded/DUST funding. Use strict original identity/snapshot validation from the existing Preview bootstrap; do not invoke its financial launch function.

SDK root: `/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/`.

| API / code | Exact behavior relevant to observer |
| --- | --- |
| `wallet-sdk-dust-wallet/dist/DustWallet.d.ts:44,60,86,96` | `DustWallet(configuration).restore(serializedState)` returns child wallet; `start(secretKey): Promise<void>`; `waitForSyncedState(allowedGap?: bigint): Promise<DustWalletState>`; inherited `stop(): Promise<void>`. Restore is not from-seed creation. |
| `wallet-sdk-unshielded-wallet/dist/UnshieldedWallet.d.ts:44,60,69` | `UnshieldedWallet(configuration).restore(serializedState)`; `start(): Promise<void>`; `waitForSyncedState(allowedGap?: bigint): Promise<UnshieldedWalletState>`. Public key comes from restored state and must match independently derived existing identity. |
| `wallet-sdk-dust-wallet/dist/DustWallet.js:34,55,81,91,158` | `state.availableCoins` and `state.pendingCoins`; `balance(time: Date)`; restore deserializes then initializes runtime; start only dispatches background sync; synced wait filters progress. |
| `wallet-sdk-dust-wallet/dist/v1/CoinsAndBalances.js` | Available coins exclude pending nonce entries. Each `availableCoins` row has `generatedNow`; getter defaults time to native `state.syncTime`. `balance(time)` instead forwards raw ledger `walletBalance(time)` without explicitly applying the SDK pending filter. Sum available `generatedNow` for an observed unreserved-DUST projection and label its evaluation time; do not substitute total balance. |
| `wallet-sdk-unshielded-wallet/dist/UnshieldedWallet.js:28–38` | `balances` uses `getAvailableBalances`, plus `availableCoins`/`pendingCoins`. Retain only aggregate selected token amounts/counts; do not publish UTXO detail or raw snapshot state. |
| `wallet-sdk-runtime/dist/WalletBuilder.js:105` | `stop()` closes the runtime Effect scope. No snapshot persistence appears in this stop implementation. |
| `wallet-sdk-dust-wallet/dist/v1/RunningV1Variant.js:61` | Start forks sync into the child scope and updates an in-memory subscription reference. Sync may call configured transaction-history storage and retries with exponential delay; use `NoOpTransactionHistoryStorage` and an external absolute runtime deadline. |
| `wallet-sdk-facade/dist/index.js:179,309,655,663` | Facade init also initializes default submission/proving/pending services; start starts three children and pending service; synced wait combines children; stop closes children/submission/pending scope. No direct serialization/persistence in these start/stop methods. Direct children are a smaller observer dependency. |

Existing repository pattern: `experiments/moriarty-midnight-financial/ledger/preview-bootstrap.mjs:76–89` reads strict existing envelopes, derives the original seed roles, restores each child without fallback, checks restored network/public key/address against existing identity, then starts/syncs. Reuse its logic narrowly. Its surrounding `launchPreviewFinancialCase` creates directories, reads role/password material, executes financial work and persists snapshots; never use it merely to observe funding. `hello-world/src/wallet.ts:createWallet` falls back to fresh sync if restore fails, and `check-balance.ts` calls `getOrCreateWallet` and persistence; neither is suitable unchanged.

## Necessary boundaries and caveats

1. Restoration requires privately reading existing snapshot contents and deriving the existing DUST key from the existing seed in the eventual admitted observer process. This is reuse, not generation of a new identity. This diagnosis did not perform those reads. Do not read role secrets or contract-store password when the narrow observer does not need them. Never log error objects, private SDK state, keys or raw websocket responses.
2. Use strict owner-only files/directories, no symlinks/fallback, original address/key/network match, no pending persistence marker, exact snapshot bytes/digests before and after, and exclusive wallet usage. Derivation must use the same account/role/index as production (`HDWallet.fromSeed(...).selectAccount(0).selectRoles(...).deriveKeysAt(0)`), then clear temporary key material where supported.
3. **Restoration loses an important pending-state signal:** dust `v1/Serialization.js:63–70` calls `CoreWallet.restore(..., [], ...)`, resetting `pendingDust` to empty. Therefore restored pendingCoins empty does not prove no earlier unresolved spend. Existing consumed allocation records and public transaction finality/persistence lineage must independently exclude unresolved transactions before calling funds unreserved. Do not turn this into a wallet reset or spend permission.
4. `waitForSyncedState(0n)` filters progress, not a fresh canonical timestamp. Check `progress.isConnected` and `isStrictlyComplete`, current network/genesis/protocol and an independently bounded freshness anchor. Reject future/stale native sync time. Funding is a timestamped observation, not a reservation or guarantee that a later transaction will balance/prove/finalize.
5. A wallet balance threshold cannot establish exact future fee/proof feasibility. Record available generated DUST and target need separately. `estimateDustGeneration` uses hypothetical coins; do not label its result currently spendable funding. No registration, transfer, balancing, signing, proving, finalizing, submission or persistence call belongs in the observer.
6. Synchronization mutates in-memory wallet state and may issue indexer history queries/subscriptions and retry. It need not mutate wallet files or chain state. Use NoOp history storage; no LevelDB provider. Finite outer containment is still needed because Promise.race does not cancel background work, and SDK internal retry count is not an independent hard bound.
7. Always stop every created child, including partial initialization and late completion; missing stop or exceeded deadline yields UNKNOWN, not a ready observation. A stopped read-only observer must leave original seed/snapshots/markers unchanged. A bounded output receipt may be written to a separate evidence destination by its caller, never inside original wallet state.
8. Pin the actual SDK closure used by the new caller. These direct entry hashes are observations, not a transitive dependency attestation. Controlled-source tests should enforce no forbidden APIs, no fallback, no writes to original files and cleanup after errors before any admitted live observation.

## Observed source hashes

| Installed source relative to SDK root | SHA-256 |
| --- | --- |
| wallet-sdk-dust-wallet/dist/DustWallet.js | `6122952c8d140f4421edd2ad41eb648706c5c4341eb91e79bfb78a0461938a02` |
| wallet-sdk-dust-wallet/dist/DustWallet.d.ts | `c0bad03a4c609fdf6e67006ce9c9f84dc4d7b15a2bb5b37c3291aae47c4a151a` |
| wallet-sdk-dust-wallet/dist/v1/CoinsAndBalances.js | `b2e6e462c0709cc3f3945e1e0c16f6fcbf45ce8e58c70b22e2e11b300373fc37` |
| wallet-sdk-dust-wallet/dist/v1/Serialization.js | `41236f39928c2ba7ebe0470d7fc1de8b9724399c356bd70648597e599ddb4090` |
| wallet-sdk-dust-wallet/dist/v1/RunningV1Variant.js | `a676c5bef4ec5df1b55cdf6c05d22268f7f4df424a662eb36dcd983d899c13f6` |
| wallet-sdk-unshielded-wallet/dist/UnshieldedWallet.js | `703c24afc04f7e6907a3fe581a429985512fa514ca58b377580200c687d828f0` |
| wallet-sdk-unshielded-wallet/dist/UnshieldedWallet.d.ts | `84c8941c96abef2b24185c23ebefc8a9a4c34e952fde17e2dea1614fc34e6cae` |
| wallet-sdk-facade/dist/index.js | `bb63f609499432f54b7db424d124d7dc80ad7a466af3c39cf7033f52b33f7562` |
| wallet-sdk-runtime/dist/WalletBuilder.js | `32d1d45b8194c29df1fff2370da7f3d4570c763ed2ba29116d118b62e68594ee` |
