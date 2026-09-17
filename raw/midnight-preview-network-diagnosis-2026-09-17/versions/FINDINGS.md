# Preview version compatibility research — 2026-09-17

Inference: the reported installed SDK/compiler/prover combination matches the current officially tested Preview matrix. The network ledger reporting 8.1.2 while the client/prover reports 8.1.0 does not establish incompatibility, and does not explain the registration relay failure by itself.

## Exact published Preview matrix

Source fact: https://docs.midnight.network/relnotes/support-matrix, updated September 16, captured September 17 via Scrapling with robots allowed. The matrix describes latest tested versions and says earlier versions may work but are not guaranteed/supported.

| Component | Tested Preview version |
| --- | --- |
| Node (Midnight) | 1.0.2 |
| Compact devtools | 0.5.1 |
| Compact toolchain | 0.31.1 |
| Compact runtime | 0.16.0 |
| Compact JS | 2.5.1 |
| Platform JS | 2.2.4 |
| On-chain runtime | 3.0.0 |
| Wallet SDK | 1.2.0 |
| Midnight.js | 4.1.1 |
| testkit-js | 4.1.1 |
| DApp Connector API | 4.0.1 |
| Midnight Indexer | 4.3.5 |
| Proof server | 8.1.0 |

The matrix does not contain a separate ledger npm-package row. Do not represent this table as explicit independent support certification for every ledger-v8 client/server pairing. Preprod/Mainnet rows are otherwise identical but specify Indexer 4.3.3-hotfix.

Repository observation: `installed-lock-observation.json` records the hello-world package-lock hash and exact versions. Compact JS 2.5.1, compact-runtime 0.16.0, ledger-v8 8.1.0, Midnight.js contracts/protocol 4.1.1, onchain-runtime-v3 3.0.0, platform-js 2.2.4, wallet-sdk 1.2.0 match the reported stack. Wallet umbrella and facade versions are distinct: umbrella 1.2.0 resolves facade 4.1.0 and dust-wallet 4.2.0. This is not itself skew.

Repository observation: prior `raw/midnight-preview-setup-2026-09-17/node-version.json` reports 1.0.2-eb71e64e; `ledger-versions.json` reports =8.1.2 and API version 2. Those are historical same-day probes, not probes performed by this subtask.

## Relevant fixes and their limits

Source fact: https://docs.midnight.network/relnotes/ledger documents 8.1.2 hardening noncanonical/deserialization-invalid values, dust zero-divide and saturation fixes. It characterizes breaking compatibility as confined to environments containing maliciously formed transactions. This is consistent with the canonical GitHub ledger release https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-8.1.2 (web discovery inspected). No source found says ordinary 8.1.0-produced registration transactions are generally incompatible with 8.1.2 nodes.

Source fact: https://docs.midnight.network/relnotes/wallet says SDK 1.2.0 fixes the race where registration fee exceeds allow_fee_payment, resulting in BalanceCheckOverspend; it adds build-time fee estimation/failure and waitForGeneratedDust with estimateRegistration. Repository observation: installed facade source lines 273–284 contains the fail-fast fee check and lines 627 onward implements the wait helper; retained excerpts and full-source digest are adjacent. Inference: simply upgrading to the documented fix cannot explain/repair this case when that fix is already present. Whether the failing caller followed this method and its preconditions remains a separate source/trace question.

Source fact: https://docs.midnight.network/relnotes/midnight-js lists 4.1.1 with hardened indexer error handling and state emission for blockHeight/blockHash queries; it contains no stated fix for pending registration relay. No confirmed current relay outage or exact matching issue was established by this bounded research.

Source fact from web discovery only: https://github.com/midnightntwrk/midnight-node/releases/tag/node-1.0.2 contains historical timestamp/cache validation fixes (PR #1932, narrowed by #1964/#1965), including a historical Preview stall at block 128536. That is a different recorded symptom and is not evidence of today's cause. Structured GitHub release/issue acquisition was blocked by unauthenticated API rate limiting (HTTP 403 receipts preserved). Consequently this is not an exhaustive issue review and GitHub release prose was not admitted as a locally captured immutable source.

## Documentation discrepancies and coverage limits

Contradiction/documentation lag: current support matrix says node 1.0.2, while current node release-note page still labels 1.0.1 latest. Disposition: use the environment-specific compatibility matrix for the tested pairing; retain both source captures and the live observed node version. Do not downgrade based on the stale node page. The overview's generic Ledger 8.0 wording is less precise than component notes and live RPC.

Git references were captured with remote URLs, symref default branch and full hashes: node main 1efc605cf08ca36f8aee0fc447ea8ae917661067; node-1.0.2 tag 1aa27a31d64fd2a428d118ab4dbea4b60de9f5ec; ledger default branch ledger-8 HEAD 5410620fd43bbfe1c11d79499776ae228991f001; ledger-8.1.2 tag 36d7442172136758e33009f75ff4aa56616cff40; wallet main 03e2b783779a579d9cd331aa24bb09947ee11423. Git reference capture does not prove deployed binary identity, nor were repositories cloned/source-audited here.

Recommendation: keep the matched versions while diagnosing transaction acceptance versus propagation/inclusion from retained receipts. A version upgrade is not justified as the established remedy. SDK support is not proof that a particular transaction is valid, submitted, propagated, or finalized. No wallet changes, installations, submissions, or production edits were performed.
