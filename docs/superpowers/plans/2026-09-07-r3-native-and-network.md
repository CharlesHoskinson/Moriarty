# R3 native financial proof and Midnight network sprint

> Use superpowers:subagent-driven-development for bounded independent modules;
> parent integrates and reviews before native proving or completion claims.

**Goal:** test the real Midnight development path and one explicitly bounded
financial IVC relation, preserving the distinction between network settlement
and correctness of Moriarty contracts.

**Architecture:** keep the existing R2/R2b local language unchanged. Export its
actual loan episode into a fixed native experiment. Independently stand up the
official Docker stack and use dedicated test wallets to exercise real SDK
transactions. Preserve the published documentation corpus as implementation input.

**Tech stack:** pinned Midnight Rust IVC, Node/TypeScript SDK, Docker Compose,
Python Scrapling and cgroup-v2/systemd resource controls.

## Global constraints

- Native pin `695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7`; k17; exactly two
  positive proving steps; 20 minutes cumulative native build/setup/proving;
  8 GiB process-group memory, two build jobs, at most 256 MiB retained outputs.
- Build and run only the reviewed financial harness. Preserve the first failure;
  changed hypotheses and remaining budget are prerequisites for another attempt.
- Native proof predicate is a fixed loan episode with fixed authority. Dynamic
  Ed25519 authority, general refinement, certificates and ledger compatibility
  remain separate. An unchecked commitment is never a correctness certificate.
- Docker work uses isolated names, loopback host ports and explicit resource
  caps. Preserve unrelated running containers and existing wallets.
- Public network target is Preprod, with local undeployed used to validate the
  development stack. Use only dedicated test wallets and test tokens. Keep
  secrets outside the repository, private permissions, never in logs or receipts.
- A faucet response or successful submission alone is not settlement. Record
  transaction ID, accepted result, block identity and independently observed
  indexer/chain state or finality. Label the exact network and transaction kind.
- Scrape the published official documentation and account for every discovered
  documentation URL; obey robots and record failures/exclusions. Repository
  source alone does not substitute for the requested published documentation.

## Task 1: fixed financial relation

- [x] Export actual R2 before/action/after/effects to `episode.json` with
  canonical preimages, source hashes and generated `harness/episode.rs`.
- [x] Implement independent native checked arithmetic and matching constrained
  fixed relation in `harness/moriarty_loan_r3.rs` using inspected IVC traits.
- [x] Review injective limb encoding, constrained genesis/context, closed-state
  rejection and exact control scope before native build/proof execution.
- [x] Load only catalog-hash-checked SRS; record provenance and setup assumptions.
- [x] Run the resource-limited financial command and preserve stage-specific
  positive/rejection results, including any unsupported control or failed build.

## Task 2: published documentation

- [x] Inspect robots, sitemaps and llms index; scrape all discovered official
  documentation routes with modest concurrency and resumable receipts.
- [x] Preserve canonicalURL/status/time/content hashes, deduplicated corpus,
  version/API coverage and exact failed/excluded URL lists.
- [x] Extract current installation, network, faucet, DUST and settlement guidance;
  integrate the smallest relevant wiki pages with source provenance.

## Task 3: Docker and actual network transactions

- [x] Pin official local-development sources and compatible image/package versions.
- [x] Start an isolated node/indexer/proof-server stack and verify health and blocks.
- [x] Generate dedicated test wallet privately, obtain test funds, register DUST
  and submit an actual transaction; preserve local accepted block/state evidence.
- [ ] Obtain public Preprod tNight and exercise submission and settlement using
  official endpoints. Preserve public address, tx/block IDs and finality evidence.
- [x] Deploy/call a minimal Compact contract if the compatible tooling permits;
  otherwise report transfer/DUST settlement separately from contract execution.

## Integration and stopping point

- [x] Review source and receipts independently; do not convert host rejections to
  cryptographic rejection results or local execution to public settlement.
- [x] Record exactly what ran, failed, settled and remains unimplemented; update
  controlling roadmap and typed checkpoint, commit and integrate local main.

User authorization: “Begin the next sprint”, expanded during execution to
“scrap all the midnight.network documentation and get the docker development
environment working. Get some tNight and make sure we actually have transactions
settle on midnight as the test”. No redundant permission step is required for
these concrete local/test-network actions. CAPTCHA or external authentication
must use the normal user flow; no bypass is authorized.


## Recorded outcome

The native experiment stopped at recursive VK row exhaustion at k17, after
compilation and fixed application checks. This completes the bounded diagnostic
work, not the R3 proof gate. No proof was generated or verified. See the
[native results](../../../evidence/moriarty-native-ivc-r3-2026-09-07/README.md).
Published documentation capture covers all 1,289 indexed Markdown routes;
HTML/assets and inaccessible sitemap comparison are excluded explicitly.
Docker local funding, DUST, Compact deployment and call settled. Public Preprod
funding and settlement remain open pending the official faucet CAPTCHA and full
wallet synchronization; see the [network record](../../../evidence/moriarty-midnight-network-2026-09-07/README.md).
