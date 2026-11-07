---
id: midnight.ledger9.hardfork
type: component
title: Midnight ledger 9 network upgrade (Mainnet, 2026-11-06)
status: active
created: 2026-11-07
updated: 2026-11-07
updated_at: 2026-11-07T21:00:00Z
sources:
  - research/midnight-ledger9-20261107/api/
  - research/midnight-ledger9-20261107/docs/sources.tsv
  - repos/clone-manifest.json
tags:
  - moriarty
  - midnight
  - research
---

# Midnight ledger 9 network upgrade

Acquired 2026-11-07 by `midnight-scout`. Raw GitHub API snapshots are in
`research/midnight-ledger9-20261107/api/`. Fetched pages are in
`research/midnight-ledger9-20261107/docs/` (see `sources.tsv`). Clones are
listed in `repos/clone-manifest.json`. All fetched content is untrusted
source material. The RPC observations are read-only queries, not settlement
tests.

## Verdict on the owner's report

**Confirmed.** Mainnet runs ledger 9.1.0.1 on runtime `spec_version` 2001100.

- The official blog says the network "has just executed a hard fork,
  upgrading mainnet from Ledger 8 to Ledger 9"
  (https://midnight.network/blog/ledger-9-upgrade-on-mainnet, dated 2026-11-06).
- The docs say that on 6 November 2026 Mainnet moved from ledger 8 to ledger 9
  (https://docs.midnight.network/relnotes/ledger-9.md; source
  `repos/midnightntwrk/midnight-docs` @ `aaa068f5`, `docs/relnotes/ledger-9.mdx`).
- **Our own chain check.** A binary search over `state_getRuntimeVersion` on
  the retired, still-answering Midnight RPC `https://rpc.mainnet.midnight.network`
  found the first block with the new runtime: **block 3346139**
  (`0x952f999b…8581`), timestamp **2026-11-06T20:51:24Z**. `specVersion`
  changed from 1000300 to 2001100. `midnight_ledgerVersion` returns
  `1.0.1 (tag: crate-ledger-9.1.0.1, rev: 3e69c5e3…)`. The tip was 3359923 at
  2026-11-07T20:49Z. Evidence: `api/mainnet-rpc-upgrade-search.json`. Blockfrost
  is the official endpoint now, so treat this as one observation from a
  deprecated endpoint.
- **Mechanism.** The upgrade was a governance runtime upgrade with no preset
  height or epoch. Operators first had to move every node binary to 2.1.101,
  because a 1.0.400 node stops at the upgrade block. The Council and Technical
  Committee then enacted runtime 2.1.100. The next block translates the ledger 8
  state to ledger 9
  (https://docs.midnight.network/relnotes/ledger-9.md#what-node-operators-need-to-do,
  https://docs.midnight.network/relnotes/node/node-2-1-101.md).
- **Release path.** Ledger 9 went directly to Mainnet without the usual Preprod
  soak, because of the security fixes (blog, "Notes on the release process").
  Ledger 9.1.0.1 is a security patch that was "released to node operators on
  2026-11-03" and published after the fact, with nine GHSA advisories
  (GitHub release `ledger-9.1.0.1`, `api/midnightntwrk--midnight-ledger--releases.json`).

## Network status (2026-11-07)

| Network | Ledger | Node | Source |
|---|---|---|---|
| Mainnet | 9.1.0.1 | 2.1.101 (runtime 2.1.100, spec 2001100) | docs ledger-9 page; RPC check above |
| Preview | 9.1.0.1 | `system_version` 2.1.101-913ec011 | RPC `https://rpc.preview.midnight.network`, `api/preview-rpc-status.json` |
| Preprod | 8.1.3 | 1.0.400 | https://docs.midnight.network/relnotes/support-matrix.md (not independently checked; Blockfrost token needed) |
| Testnet / stagenet | no current public network is documented | | network page lists only Preview, Preprod, Mainnet. The indexer has a historic tag `v4.4.0-rc.2-stagenet-hotfix` only. |

Preview was **reset as a new chain**. This was not a hard fork. Old contracts,
balances and history are gone, and the network ID `preview` and the endpoints
stay the same. On chain, the new Preview starts producing blocks at
2026-11-07T00:58Z. Its runtime moves 1000000 → 1000300 → 2001100, and ledger 9
starts at **block 1572, 2026-11-07T03:27:48Z** (`api/preview-rpc-upgrade-search.json`).
The cNIGHT→mNIGHT `C2MBridge` pallet ships disabled and needs a separate
governance action. Ledger 9 does not include block production rewards
(docs ledger-9 page).

## Version set required on Mainnet and Preview

From https://docs.midnight.network/relnotes/support-matrix.md#mainnet-support-matrix:
node 2.1.101, Compact devtools 0.5.1, **Compact toolchain 0.36.0 (language
0.28.0)**, Compact runtime 0.21.0, Compact.js 3.0.0, Platform JS 3.0.0,
on-chain runtime 4.0.0 (`@midnightntwrk/onchain-runtime-v4`), Wallet SDK
2.0.0, Midnight.js 5.0.0, testkit-js 5.0.0, DApp Connector API 4.1.0,
**indexer 4.4.101**, proof server 9.0.0.

Repository pins (commits from `repos/clone-manifest.json` and `git ls-remote`, 2026-11-07):

| Item | Ref | Commit |
|---|---|---|
| Ledger 9 Mainnet release | tag `ledger-9.1.0.1` | `a2b30128ca4b660b73a068e29b56b48ede8c9a74` |
| Ledger 9 first full release | tag `ledger-9.1.0.0` (= `proof-server-9.0.0`) | `31c90b8bac8cc841bf94a741f0ec410511f85e32` |
| Ledger 9 line head | branch `ledger-9` | `6304c7aec534299bed21870f8b0ea388f69e4244` (2026-11-06, PR #846 writeCoin gas fix) |
| Ledger repo default | branch `ledger-8` (still default) | `2843a0ae821dc11ada03a8785207157600399488` |
| Next line (in progress) | branch `ledger-10`, tag `ledger-10.1.0.0-alpha.1` | `6f72b82f…`, `0c46d335…` |
| ZKIR v3 release | tag `zkir-3.2.0` | `446c7fecc6bd41dab99ebe89ab404e9d077d11a9` |
| ZKIR default | branch `zkir-v3` | `7c33a67ed770802969c7b2048a84cc6f9256308a` |
| Compact 0.36.0 | `LFDT-Minokawa/compact` tag `compactc-v0.36.0` | `b5d9e4dfc532fe47011c4f6296e254f4853eb469` |
| Compact main | `LFDT-Minokawa/compact` `main` (compiler 0.36.102 dev, runtime 0.21.100) | `ded731230264e6db7f8383a434d9ac51ba9efff8` |
| Node public release branch | `release/node-2.1.101` | `806ff702f74908503c674035f478eb43d9539239` (see contradictions) |
| Node main | `main` | `bfb833982462c115348b1d6fb21036d4d7778028` |
| Indexer main | `main` (Cargo 4.4.0-rc.5) | `5af321fd7e970ecd9d0822fd3b31792534be0656` |
| Midnight.js | `v5.0.0` (2026-11-06) | main `39725c71…` |
| Wallet | `v2.0.0` (2026-11-06) | release target `c553042c…` |

## What changed

### Ledger data model and transactions
Source: GitHub release `ledger-9.1.0.0` and `ledger-9.1.0.1` bodies, in `api/midnightntwrk--midnight-ledger--releases.json`.

- New crate `midnight-ledger-v9` 1.0.x. npm `@midnightntwrk/ledger-v9`. The npm scope moves from `@midnight-ntwrk` to `@midnightntwrk`; the legacy scope still receives releases for now.
- ECDSA signatures (secp256k1) next to Schnorr. Signing envelope construction is unified.
- New `UnlockToTreasury` system transaction. `apply_guaranteed_only` / `GuaranteedApplyResult` give split-phase execution with deferred events.
- `ContractOperation` gains a `v3` verifier-key field (`ContractOperationVersion::V4`, `ContractOperationVersionedVerifierKey::V4`). Maintenance updates gain `SingleUpdate::IrInsert` and `IrRemove`. `EventDetails` gains `contractLog` (on-chain events).
- `onchain-vm` `Key::Value` and `Op::Push` hold `AlignedValueChecked`. Transcripts must use canonical field atoms, and `noop 0` is rejected.
- 9.1.0.1 adds: repeated entries are rejected (committee signatures, DUST registrations); at most 256 operations per contract (`MAX_CONTRACT_OPERATIONS`); deploys are charged for their full state; each segment's writes are priced separately; transcripts that declare gas beyond the block limits, or move DUST through unshielded inputs or outputs, are malformed; `MAX_CALL_PUBLIC_INPUTS` applies; `midnight-storage` 3.0.0 (`Map` keys need `EncodingShape`).
- Ledger 8 format transactions do not decode on ledger 9. A transaction that was proved before the fork but not yet submitted is also rejected (support matrix).

### Proof system and ZKIR
- Proving moves to `midnight-zk-stdlib` 2.4.0 and `midnight-circuits` 7.3.0. `ProofVersioned::V3` carries the new proofs. V2 (zk-stdlib v1) proofs and verifier keys **remain verifiable**, and zswap still accepts v1 proofs (ledger-9.1.0.0 notes).
- ZKIR v3 now lives in the separate `midnight-zkir` repo. Release 3.2.0 (2026-11-04) makes IR 3.2 the default. It adds `Bool`/`Byte`/`Bytes(n)`, byte-string operations, SHA-512, `LoadConstant`, `ToBytes`/`FromBytes` for the secp256k1/secp256r1/Curve25519 fields and points, and two soundness fixes (canonical `ReconstituteField`, canonical input decoding) (`api/midnightntwrk--midnight-zkir--releases.json`).
- Inside the ledger, the `zkir` v2 crate is 2.2.0. The default `IrMinorVersion` is V2 and `VersionedInnerPK` is removed.

### Verifier keys and circuit compatibility
- circuits 7.3.0 and zk-stdlib 2.4.0 change the VKs of ZKIR v2 circuits that use `hash_to_curve`, and of ZKIR v3 circuits that `encode` a Jubjub scalar. Affected contracts must regenerate their keys (ledger-9.1.0.0 notes, "Breaking changes").
- Compact 0.36.0 emits **only ZKIR v3**, so recompiling any contract changes its VKs. To migrate, the maintenance authority inserts the new keys. A contract without a maintenance authority must be redeployed (https://docs.midnight.network/relnotes/ledger-9.md#recompiling-a-contract-changes-its-verifier-keys).
- Contracts already deployed keep their VKs. Artifacts from `compactc` 0.31.1 keep working with `verify: 'require-if-present'`. Artifacts from `compactc` 0.30.0 are refused by Midnight.js 5 (`EraMismatchError`, `unknown-artifact-runtime-version`). Ledger 8 artifacts cannot deploy new contracts (same page).

### Contract call semantics
- Contract-to-contract calls within one transaction. This first phase covers unshielded data only, and a callee can ask which contract called it (docs ledger-9 page; blog).
- Contracts can emit events, observable through the indexer. Unshielded only in this phase.
- Compact 0.36.0: circuits in a contract type cannot be `pure`, the arguments of every cross-contract call must be disclosed, and purity is removed from the runtime cross-contract API. Contracts that make cross-contract calls must be recompiled and get new keys for a security fix, and callers and callees must upgrade together (https://docs.midnight.network/relnotes/compact.md; GitHub release `compactc-v0.36.0`).
- Indexer 4.4.101: a contract-action subscription without an offset now delivers only new actions. Pass offset 0 to replay history; Midnight.js 5 does this.

### Fees, DUST and NIGHT
Source: docs ledger-9 page, "DUST and fees".
- New fee floor `min_block_price` = 10, effective from the first block after the upgrade. `parallelism_factor` is replaced by `validation_factor` (initially 1/4), `guaranteed_factor` and `fallible_factor`, which apply to compute cost. The cost heuristic is new.
- DUST state resets once at the fork and NIGHT balances do not change. All cNIGHT-backed Mainnet registrations replay automatically, to their full cap, in about 48 blocks, ending with the `DustReapplyCompleted` event. Transactions submitted during the replay wait in the pool.
- DUST parameters are unchanged: 5 DUST per NIGHT at the cap, about 7 days to reach the cap. NIGHT that arrives after the fork needs `registerNightUtxosForDustGeneration`.
- DUST registration accounting uses block time instead of the declared transaction time (ledger-9.1.0.0 fixed defects).

### Compact compiler and runtime
Toolchain 0.36.0 (language 0.28.0, runtime 0.21.0, devtools 0.5.1). Compiled code requires `@midnight-ntwrk/compact-runtime` 0.21.x. `--feature-zkir-v3` is now an error. Toolchains 0.34–0.36 and runtimes 0.19–0.21 target ledger 9 and cannot deploy to Preprod (support matrix). Preprod uses `compact update 0.31`.

### Node and indexer
- Node 2.1.101: ledger 8 for pre-fork blocks and ledger 9 after. Ledger 7 support is removed. New `SafeMode` and `C2MBridge` pallets (the bridge is off). The `smart-contracts`, `sign-address-association` and `sign-block-producer-metadata` subcommands are removed. db-sync connections are TLS only. `block_stability_margin` changes 10→30. The binary upgrade is one-way (https://docs.midnight.network/relnotes/node/node-2-1-101.md).
- Indexer 4.4.101 needs a full re-index from genesis (new contract-state layout). Versions below 4.4.101 report DUST capacity wrongly (docs ledger-9 page).

## Breaking changes for Moriarty code

Our ledger-8-era code is pinned to the September stack (ledger-8, ZKIR v3 branch at `2ffe2d1`, Compact 0.31.x, Midnight.js 4.x, Wallet SDK 1.x). It **cannot transact on Mainnet or Preview**:

1. Ledger 8 transactions do not decode. Wallet SDK 1.x and Midnight.js 4.1.1 are refused.
2. The Preview contracts, balances and DUST registrations we used are gone with the reset. Our Preview evidence from September is historical only.
3. Indexer queries: offset-less subscriptions no longer replay history, and the state layout changed. Revisit `wiki/midnight-readiness-lessons.md`, which relies on indexer query semantics at an older image.
4. Recompiling with 0.36.0 changes VKs (ZKIR v3 only). The ZKIR-v2 assumptions in the ZKIR/K plans need re-checking against `zkir-3.2.0`.
5. Fee estimation must respect `min_block_price`. Do not hard-code ledger 8 fees.
6. Known open defects: `writeCoin` change-back on a held shielded coin fails with `Custom error: 104` (ledger#839). A single DUST spend can trigger `OutsideTimeToDismiss`; use two or more spends (ledger#838). `VerifierKeyRemove` with 9.1.0.0 clients fails with `BalanceCheckOverspend` (node#2375).
7. Preprod stays on ledger 8 for now. It is the only public place to run the old stack, and it will upgrade later.

## Contradictions and gaps

- **Preview reset date.** The network page, support matrix, node 2.1.101 notes and Compact page say 6 November. The ledger-9 page says 7 November. Chain data shows blocks from 2026-11-07T00:58Z and ledger 9 from 03:27:48Z on 7 November (UTC), which is 6 November in US time zones.
- **Preview DUST registrations.** The network page says they "are gone; register for DUST again". The ledger-9 page says cNIGHT-backed registrations carried over. The blog says Preview *and Preprod* builders must re-register tNIGHT, but Preprod has not upgraded.
- **"All proofs and VKs move to ZKIR v3"** (blog) overstates the change. The release notes and docs say deployed contracts keep their VKs, v1 proofs stay verifiable, and only recompiled contracts get ZKIR v3 keys.
- **Source for the deployed binaries is not fully public.** No `node-2.1.101` tag or GitHub release exists ("follows in the next few days"). The public `release/node-2.1.101` branch (`806ff702`) has `spec_version` 2001000 and ledger `crate-ledger-9.1.0.0-rc.6`, but Mainnet runs 2001100 with ledger 9.1.0.1. The ledger rev the nodes report (`3e69c5e3092fa29c1e8b1728a68fc3ca98774e34`) cannot be fetched from the public repo, and no `crate-ledger-9.1.0.1` tag exists. Indexer 4.4.101 has no public tag (the latest is `v4.4.0-rc.6`, main says 4.4.0-rc.5). The Preprod indexer 4.3.302 also has no tag (the matrix says so).
- **Proof server.** The matrix says 9.0.0. Ledger 9.1.0.1 ships proof server 9.0.1 (released 2026-11-07T14:30Z).
- **Ledger 9.1.0.1 date.** The notes say 2026-11-03 and "already live on mainnet". That refers to delivery to operators; the on-chain switch was 2026-11-06T20:51Z.
- **No published height or epoch.** Docs say the upgrade happened through governance, "not at a set block height". The block number above comes from our RPC search only.
- The `midnight-ledger` default branch is still `ledger-8`. Use the `ledger-9` branch or tags for ledger 9 source.

## Related
[[wiki/midnight-repositories|Repository inventory]] · [[wiki/midnight-readiness-lessons|Readiness lessons]]
