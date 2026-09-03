---
id: research.contradictions
type: contradiction
title: Contradictions and documentation drift
status: active
updated_at: 2026-09-03T14:17:00Z
sources:
  - SRC-0002
  - SRC-0005
  - SRC-0006
  - SRC-0007
  - SRC-0009
  - SRC-0012
  - SRC-0015
  - SRC-0016
  - SRC-0017
  - SRC-0023
  - SRC-0024
  - SRC-0025
---

<!-- markdownlint-disable MD013 MD025 MD060 -->

# Contradictions and documentation drift

| Conflict | Evidence | Disposition |
|---|---|---|
| Marlowe live sitemap targets versus live site | The current sitemap points all 100 entries at dead `play.marlowe.iohk.io` hosts; the same paths work at `docs.marlowe-lang.org` and were acquired 100/100 | Documentation deployment drift; use the live-host crawl, preserve the sitemap as evidence |
| Marlowe abstract interval versus formal placeholder | Haskell V1 implements inclusive bounds; Isabelle `BlockchainTypes.thy` retains a TODO about endpoint treatment | Normative ambiguity; release blocker for V2 equivalence |
| Isabelle coverage versus Haskell behavior | Haskell comments identify refund-order differences and lack of Isabelle Merkleization | Do not claim full implementation correspondence |
| `marlowe-runtime-ng` name versus implementation | Repository has one initial commit and an empty README | S1 placeholder, not an implemented Runtime replacement |
| V2 proposal versus implementation status | April 2026 report proposes `WhenAll`, `EntryDeposit`, compression, iteration, and types; current active validator branch does not contain those language features | S2 design must not be described as S3+ implementation |
| Midnight `compact` API flag versus README | GitHub metadata says `archived=false`; README says archived/no longer maintained and development moved to LFDT Minokawa | Treat as S5 release mirror; use `LFDT-Minokawa/compact` for source authority |
| Current Midnight docs versus compiler development head | Docs describe language 0.26.0/compiler 0.34.0; active source builds compiler 0.34.100 with language 0.26.0 and runtime 0.19.100 | Record the complete version tuple; do not use “Compact 0.26” as a toolchain identifier |
| ZKIR generations | Some shipped/precompiled artifacts remain ZKIR 2 while the extracted `midnight-zkir` default branch is `zkir-v3` and Moriarty generated 3.0 | Require explicit IR major/minor and backend conformance; no implicit latest |
| Imported report citations | The user-supplied report contains internal `turn...` citation markers that cannot be resolved outside its original session | Treat the prose as design input only; replace claims with pinned local evidence |
| Koios transaction-detail capture | The first all-at-once POST is preserved as a 413 response in the top-level `.json`; seven subsequent bounded Scrapling POSTs returned all 137 rows | Use the chunk manifest as evidence; never parse the failed response as JSON |
| Taxonomy report access versus authorized repository | `SRC-0015` correctly reports that its online environment could not retrieve a public DeFiFormal repository, but `SRC-0016` is an authorized clean local checkout at commit `8ae0bbfaa3193078d1cabf6999db1382985b7f95` | Preserve the report's limitation as historical context; use the local pinned artifacts for the 72-row and 60-construction reconstruction without inferring public availability |
| DeFiFormal working-note totals versus current pinned artifacts | An earlier working note expected 47 partial, 13 inadmissible, 573 covered, and 686 residue; direct aggregation at `SRC-0016` yields 45 partial, 15 inadmissible, 570 covered, and 689 residue from the same 1,259 obligations | The reproduced current-commit totals control; earlier figures are superseded unless a different commit is produced |
| Marlowe baseline abbreviated hash | The first wiki draft expanded prefix `99f432d8` to the wrong full hash; the repository lock and checked-out `HEAD` agree on `99f432d8ef9dbd1b52b7fa089254de15913b490f` | Corrected in `marlowe-baseline.md`; all future claims use the full lock-record hash |
| First taxonomy recommendation versus updated run | SRC-0015 recommends M4+ after it could not inspect DeFiFormal; SRC-0017, after repository inspection, recommends M2+M3 for human-facing families and facets with M5 as the internal formal profile | SRC-0017 supersedes the top-level taxonomy decision; preserve M4+ only as a historical crosswalk and useful facet decomposition |
| Updated-run archive provenance versus Moriarty repository pin | SRC-0017 reports that its supplied ZIP had no recoverable commit; Moriarty has a separate clean checkout at `8ae0bbfaa3193078d1cabf6999db1382985b7f95` | Bind all reproduced results to SRC-0016 and the full commit; do not attribute that commit to the ZIP |
| Updated-run complete-linkage ARI at 12 clusters versus independent harness | SRC-0017 reports 0.36; the deterministic SciPy nearest-neighbour-chain reproduction gives 0.3547259508, which rounds to 0.35 at two decimal places | Preserve the raw value and pinned method; treat 0.36 as a minor reporting or implementation-version discrepancy that does not change the best-ARI conclusion |
| NEAR Intents atomic and automatic-refund overview versus route behavior | The overview states atomic execution and automatic refunds; Verifier documentation says external calls complete asynchronously, simulation excludes them, and deposit, withdrawal, storage, and indexer paths include detached, nonrefundable, or manual recovery | Define atomicity and recovery per layer and route; never lift Verifier batch atomicity to bridge fulfillment |
| NEAR Intents non-custodial description versus implementation boundaries | The overview says users maintain control; `intents.near` records contract-held internal balances, 1Click says it temporarily transfers assets to a trusted swapping agent, and confidential execution adds a treasury and PoA bridge | Publish a route-specific custody and authority manifest before approval |
| NEAR Intents narrative status versus OpenAPI | The quickstart lists `KNOWN_DEPOSIT_TX`, which is absent from the published swap status enum; the order enum contains `UNTRIGGERED`, which the narrative omits; fill and payout use independent states | SDKs must accept unknown statuses, preserve raw evidence, and reconcile service status with chain evidence |
| NEAR Intents guaranteed-delivery name versus transport semantics | The relay replays unacknowledged events, requires client deduplication, has a seven-day retention limit, and is described as live but not yet exercised by a solver | Treat it as bounded at-least-once delivery, not exactly-once or permanent delivery |
| NEAR Intents confidentiality label versus trust boundary | `basic` and `advanced` lack public normative leakage definitions; the embedded profile uses a private NEAR fork, small permissioned validator set, treasury, private relay, and PoA bridge | Model confidentiality as a named adapter profile with explicit disclosure and custody assumptions |
| Current ERC-7683 versus prior draft and OIF terminology | The 2026 resolver draft removes the prior standardized order/open/fill objects; current OIF code and prose still call `StandardOrder` and `MandateOutput` ERC-7683 | Pin the exact revision and treat OIF as a separate compatibility profile; never blend the two semantics |
