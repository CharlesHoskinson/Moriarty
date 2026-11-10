---
id: midnight.repositories.inventory
type: component
title: Midnight and Compact repository inventory
status: active
updated_at: 2026-11-07T21:00:00+00:00
sources:
  - SRC-0055
  - SRC-0056
  - SRC-0051
  - SRC-0052
  - SRC-0005
  - SRC-0006
  - SRC-0007
created: 2026-09-02
updated: 2026-11-07
tags:
  - moriarty
  - research
---

# Midnight and Compact repository inventory

## Clone refresh, 2026-11-10

All 83 local clones under `repos/` (81 `midnightntwrk`, 2 `LFDT-Minokawa`) are
fast-forwarded to their upstream default branch, shallow and blobless. No clone
had local changes. 14 heads moved. The hardfork release tags are fetched as tags,
and every commit that `qa/sherlock/target-pins.json` cites resolves in the clones.

Preview, Preprod and Mainnet run the same stack. On 2026-11-10 at 04:20Z all
three RPC endpoints report node `2.1.101-913ec011`, spec 2001100 and ledger
`1.0.1 (crate-ledger-9.1.0.1, rev 3e69c5e3)`. Preprod was upgraded in place on
2026-11-09; Preview was relaunched as a new chain on 2026-11-07. The readings are
in `research/midnight-ledger9-20261107/api/rpc-status-20261110.json`.

| Repository | Ref | Commit | Role |
|---|---|---|---|
| `midnightntwrk/midnight-node` | tag `node-2.1.101` | `86774edf8cc76345a5c3a4221f3fe80c094b1c33` | Deployed node release: spec 2001100, `midnight-ledger-v9 =1.0.1` from crates.io |
| `midnightntwrk/midnight-node` | `main` | `52747762cc2576aec85fdb2c08292f9eee71e369` | Head (2026-11-09) |
| `midnightntwrk/midnight-ledger` | tag `ledger-9.1.0.1` | `a2b30128ca4b660b73a068e29b56b48ede8c9a74` | Ledger in use; tag `proof-server-9.0.1` is the same commit |
| `midnightntwrk/midnight-ledger` | branch `ledger-9` | `8319309b0e24f84dcb99899b7476177c109f30e5` | Ledger 9 line head (2026-11-09) |
| `midnightntwrk/midnight-ledger` | default `ledger-8` | `55124e8feb337596b62d290ce21c96d1f09762d0` | Still the default branch |
| `midnightntwrk/midnight-indexer` | tag `v4.4.101` | `0e6d8385ba573af970ec2e4313ce13ce97f5f608` | Indexer for all three networks |
| `midnightntwrk/midnight-indexer` | `main` | `7fa946bd7ee1e4f97685e2fcc4f71701c5a0aa73` | Head (2026-11-09) |
| `midnightntwrk/midnight-js` | tag `v5.0.0` | `39725c71dc5c7681b0a424efae01afcb23107b1c` | Midnight.js 5.0.0 |
| `midnightntwrk/midnight-zkir` | tag `zkir-3.2.0` | `446c7fecc6bd41dab99ebe89ab404e9d077d11a9` | Unchanged |
| `LFDT-Minokawa/compact` | tag `compactc-v0.36.0` | `b5d9e4dfc532fe47011c4f6296e254f4853eb469` | Unchanged toolchain |
| `LFDT-Minokawa/compact` | `main` | `1840c99f50a8af13beed6f24d409cc5129a45d9c` | Head (2026-11-09) |
| `midnightntwrk/midnight-docs` | `main` | `70e8378873e727accd6e85d4eb0311b1cb5c8da9` | Support matrix for the three networks |

The support matrix lists the same versions for all three networks: compactc
0.36.0, compact-runtime 0.21.0, compact-js 3.0.0, onchain-runtime-v4 4.0.0,
wallet-sdk 2.0.0, Midnight.js 5.0.0, indexer 4.4.101 and proof server 9.0.0.
The chain reports ledger crate 1.0.1, but the JS package `@midnightntwrk/ledger-v9`
stays at 1.0.0 because `midnight-js-protocol` 5.0.0 pins it exactly.

The local images are linux/arm64 builds: `midnightntwrk/midnight-node:2.1.101`,
`midnightntwrk/indexer-standalone:4.4.101` and `midnightntwrk/proof-server:9.0.1`.
Their digests are in `qa/sherlock/target-pins.json` under `localImages`.

## Current pins: ledger 9 refresh, 2026-11-07

> **Superseded on 2026-11-07.** Mainnet moved to ledger 9.1.0.1 on 2026-11-06
> (first block with runtime spec 2001100 is 3346139, at 20:51:24Z). Preview was
> reset and runs ledger 9. Preprod stayed on ledger 8.1.3 until 2026-11-09. The September pins
> below (`ledger-8` `a8ab82b`, `zkir-v3` `2ffe2d1`, Compact `11e7ec5`) are kept
> as history. They describe the ledger 8 stack and must not be used against
> Mainnet or Preview. Details: [[wiki/midnight-ledger-9-hardfork|Ledger 9
> network upgrade]].

All 84 public repositories (81 `midnightntwrk`, 3 `LFDT-Minokawa`) were
re-cloned shallow and blobless under `repos/<org>/<repo>`. Heads, dates and
outcomes are in `repos/clone-manifest.json`. The API snapshots are in
`research/midnight-ledger9-20261107/api/`.

| Repository | Ref | Commit | Role |
|---|---|---|---|
| `midnightntwrk/midnight-ledger` | tag `ledger-9.1.0.1` | `a2b30128ca4b660b73a068e29b56b48ede8c9a74` | Mainnet ledger (security patch, notes published after the fact) |
| `midnightntwrk/midnight-ledger` | tag `ledger-9.1.0.0` | `31c90b8bac8cc841bf94a741f0ec410511f85e32` | First full ledger-9 release, proof server 9.0.0 |
| `midnightntwrk/midnight-ledger` | branch `ledger-9` | `6304c7aec534299bed21870f8b0ea388f69e4244` | Ledger 9 line head (2026-11-06) |
| `midnightntwrk/midnight-ledger` | default `ledger-8` | `2843a0ae821dc11ada03a8785207157600399488` | Still the default branch; ledger 8.1.3 line (Preprod) |
| `midnightntwrk/midnight-zkir` | tag `zkir-3.2.0` | `446c7fecc6bd41dab99ebe89ab404e9d077d11a9` | ZKIR v3 used by ledger 9.1.0.x |
| `midnightntwrk/midnight-zkir` | default `zkir-v3` | `7c33a67ed770802969c7b2048a84cc6f9256308a` | ZKIR v3 head |
| `LFDT-Minokawa/compact` | tag `compactc-v0.36.0` | `b5d9e4dfc532fe47011c4f6296e254f4853eb469` | Toolchain 0.36.0, language 0.28.0, runtime 0.21.0 (ledger 9) |
| `LFDT-Minokawa/compact` | `main` | `ded731230264e6db7f8383a434d9ac51ba9efff8` | Active source |
| `midnightntwrk/midnight-node` | `release/node-2.1.101` | `806ff702f74908503c674035f478eb43d9539239` | Public branch only: spec 2001000 with ledger rc.6, **not** the deployed 2001100 build |
| `midnightntwrk/midnight-node` | `main` | `bfb833982462c115348b1d6fb21036d4d7778028` | Node head |
| `midnightntwrk/midnight-indexer` | `main` | `5af321fd7e970ecd9d0822fd3b31792534be0656` | Deployed 4.4.101 has no public tag |
| `midnightntwrk/midnight-zk` | `main` | `24a1eb5a59824651fd77cab815ba2d385fbca035` | ZK stack (circuits 7.3.0, zk-stdlib 2.4.0) |
| `midnightntwrk/midnight-js` | `main` (v5.0.0 released) | `39725c71dc5c7681b0a424efae01afcb23107b1c` | Midnight.js 5 |
| `midnightntwrk/midnight-wallet` | `main` (v2.0.0 released) | `e99e586f12dab5cf8f835c9eb717e7458b68d317` | Wallet SDK 2 |
| `midnightntwrk/midnight-docs` | `main` | `aaa068f535243af79f8ca70159602b62fc1a5a00` | Docs source (`docs/relnotes/ledger-9.mdx`) |

A `ledger-10` branch (`6f72b82f`) and the tag `ledger-10.1.0.0-alpha.1` already
exist. The next fork is in development.

## Historical pins (September 2026, ledger 8 era)

All 74 public default branches returned by the `midnightntwrk` organization API
were shallow, blob-filtered, cloned under `repos/midnightntwrk/`. The exact
heads and clone outcomes are in `repos/midnightntwrk/clone-manifest.json`; the
complete classification is in
`evidence/midnightntwrk-repositories-2026-09-02.tsv`.

The official `midnightntwrk/compact` repository is a release-artifact mirror.
Its GitHub API field says `archived=false`, but its current README says it is
archived and that development moved to `LFDT-Minokawa/compact`. All three
public LFDT Minokawa repositories were therefore also acquired. Active Compact
source is pinned at `11e7ec5abeecb99297c4faa74d30ef9adc7b51f3`.

Key backend pins are:

| Repository | Branch | Commit | Role |
|---|---|---|---|
| `midnightntwrk/midnight-zkir` | `zkir-v3` | `2ffe2d17bbb736aec36fb300aeaca679a10d2278` | ZKIR 3 compiler/model/key-generation implementation |
| `midnightntwrk/midnight-ledger` | `ledger-8` | `a8ab82ba2124c36f92795c683e70bd888bc1d1fb` | Ledger implementation and data model |
| `midnightntwrk/midnight-zk` | `main` | `695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7` | ZK stack |
| `midnightntwrk/midnight-sdk` | `main` | `f5404d29d9fb5b2c37e494e47734cbeb76bd080d` | Application SDK |
| `midnightntwrk/midnight-docs` | `main` | `f1422dafa4241e55fa33541e17bf14d1e9b3a5f8` | Current docs source |
| `midnightntwrk/example-zkloan` | `main` | `eff9030a753a45555d24343dd6dd935174b679bd` | Financial/privacy example |

Repository recency is not implementation evidence. `compact-js` and
`platform-js` are recently pushed template placeholders whose READMEs still say
“TODO - New Repo Owner.” Conversely, Compact development is active under a
different organization. Product status must be determined from code, releases,
deployment, and operational evidence together.

The active Compact repository contains the Scheme compiler, TypeScript runtime,
Agda specification, language docs, CLI, editor support, tests, and examples.
The compiler has two final code-generation paths: TypeScript and ZKIR. This is
the decisive reason Moriarty should initially generate Compact and validate the
translation instead of maintaining a second direct ZKIR backend.

The ZKIR repository is tightly coupled to a fixed ledger revision. Its workspace
at the pinned commit patches Midnight ledger crates to
`9a8777c4d035fc7f38ae286bcf5f8656668efd9f`. This coupling is evidence that
raw ZKIR is not currently a stable, blockchain-independent target contract.

## Published documentation snapshot — 2026-09-07

The [Midnight documentation corpus](../evidence/midnight-docs-2026-09-07/README.md)
adds the full published Markdown index to the earlier repository pins. It includes
API and version documentation, current installation instructions, the support
matrix, network endpoints, faucet and NIGHT-to-DUST registration guidance.
Consult its per-page timestamps before reusing moving version claims. Repository
heads above remain historical pins and are not updated to match documentation
silently. See journal CLM-0200 for acquisition scope and the discovery limitation.

Metadata: SRC-0051 task input, SRC-0052 primary published docs; source observation,
2026-09-07; S3 local acquisition, reproduced hashes; confidence high for captured
content, untested for component-level compatibility unless separate execution
evidence is linked. A published instruction is not a successful network test.


## Public network selection review

CLM-0203 and the [fresh documentation/network review](../evidence/midnight-network-review-2026-09-07/README.md)
replace the assumption that Preprod is the only public development target.
Preview fits early experiments; Preprod remains supported for final validation.
The complete indexed Markdown corpus was refreshed, and live endpoint/faucet
checks remain distinct from SDK compatibility and actual public settlement.
