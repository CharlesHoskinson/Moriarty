---
id: midnight.repositories.inventory
type: component
title: Midnight and Compact repository inventory
status: active
updated_at: 2026-09-07T03:04:05.743582+00:00
sources:
  - SRC-0055
  - SRC-0056
  - SRC-0051
  - SRC-0052
  - SRC-0005
  - SRC-0006
  - SRC-0007
created: 2026-09-02
updated: 2026-09-07
tags:
  - moriarty
  - research
---

# Midnight and Compact repository inventory

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
