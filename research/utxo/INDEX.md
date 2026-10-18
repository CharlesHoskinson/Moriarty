# Moriarty UTXO/eUTXO research collection

Completed ten research assignments on2026-10-12. Public material acquired with Scrapling0.4.15 and Git, with retrieval records and SHA256/commit pins. Artifacts live outside the canonical Moriarty vault.

## Start here

- [Moriarty backend assessment](ASSESSMENT.md): compatibility, requirements, counterevidence and current Midnight release qualifications.
- [Reference-app learning guide](REFERENCE-APPS.md):12 downloaded app/library/SDK repositories;3 focused code inspections and9 ancillary snapshots.
- [Developer-documentation reading route](DEVELOPER-LEARNING-GUIDE.md).
- [Proposed experiments](EXPERIMENTS.md):P0–P6, all specified-only.
- [Claims and evidence status](CLAIMS.md).

## Downloads and coverage

- **36 unique repository/commit snapshots** (39 downloaded copies including3 duplicate pins), covering standards, ledger implementations, formal models, language/compiler tools, private verifiers and apps.
- **29 separately downloaded PDFs**: original research, published/preprint counterparts, ledger specifications and reports; repository-embedded papers are additional.
- **311 official prose-site HTTP200 captures** from developer portal, Cardano Docs and Plutus Docs;3 URLs returned404. This includes19 client-redirect stubs and a homepage; HTTP success is not a count of substantive pages read.
- **969 current Plutus API HTTP200 captures**, covering latest public-library modules and indexes. Root identifies1.71.0.0, updated2026-09-30, source261766ad1d582bce8c86bad1af7a3b0e5355b21c.
- Both crawls exhausted their discovered in-scope link graphs. Portal source-route comparison also found an unlinked archived changelog, now downloaded; the external sidechain technical-specification PDF is retained separately.
- Every921 HTML target in the Plutus API's own root index downloaded successfully. Additional generated cross-references produced126 HTTP404s (including unhosted dependency/identifier/example targets); they are retained and listed separately. Scope exhaustion is not a claim that every cross-reference resolves.
- Standalone research inventory:122 successful HTTP captures, plus repository snapshots;15 failed HTTP acquisitions preserved. These overlap some doc pages and mirrors, so do not add all category counts as independent sources.

- [Official prose documentation index](cardano-developer-docs/INDEX.md) · [manifest](cardano-developer-docs/MANIFEST.json) · [failed URL dispositions](cardano-developer-docs/FAILURE-DISPOSITIONS.md).
- [Plutus API index](cardano-plutus-api/INDEX.md) · [manifest](cardano-plutus-api/MANIFEST.json).
- [API root-index coverage](cardano-plutus-api/ROOT-INDEX-COVERAGE.json) · [broken cross-references](cardano-plutus-api/FAILURES.md).
- [Papers/specifications PDF index](PAPERS.md).
- [Full research source inventory](SOURCE-INVENTORY.md) · [machine inventory](SOURCE-INVENTORY.json).
- [Ten researcher identities and report hashes](RESEARCH-RUN.json).
- [Collection verification](COLLECTION-VERIFICATION.json).

## Ten focused reports

- [01-foundations](01-foundations/REPORT.md) · [source metadata](01-foundations/sources.json)
- [02-standards](02-standards/REPORT.md) · [source metadata](02-standards/sources.json)
- [03-marlowe](03-marlowe/REPORT.md) · [source metadata](03-marlowe/sources.json)
- [04-toolchains](04-toolchains/REPORT.md) · [source metadata](04-toolchains/sources.json)
- [05-concurrency](05-concurrency/REPORT.md) · [source metadata](05-concurrency/sources.json)
- [06-reference-apps](06-reference-apps/REPORT.md) · [source metadata](06-reference-apps/sources.json)
- [07-other-utxo](07-other-utxo/REPORT.md) · [source metadata](07-other-utxo/sources.json)
- [08-private-proofs](08-private-proofs/REPORT.md) · [source metadata](08-private-proofs/sources.json)
- [09-authority-intents](09-authority-intents/REPORT.md) · [source metadata](09-authority-intents/sources.json)
- [10-docs-midnight](10-docs-midnight/REPORT.md) · [source metadata](10-docs-midnight/sources.json)

## Evidence boundary

Downloaded material was not executed. No code port, prover campaign, benchmark or ledger transaction was performed. Compatibility is a research inference; privacy parity, full compiler/ledger correspondence and production acceptance remain unestablished. The current Moriarty Preview gate and existing user changes are preserved. Raw bytes/receipts remain separate from assessed source statements and recommendations.

Documentation scope: current official portal /docs, official Cardano Docs, official Plutus /docs and latest public-library Haddock. Historical API versions, translations, unlinked external ecosystem/private pages, static assets and source-code HTML views are excluded. Repositories preserve source code separately. Source-route staging/deployment equality is not proven. Collection coverage is not substantive review of every page.

Existing graph query used a103755-node historical graph with an1800-token output budget; no graph rebuild or new extraction was performed. Research/model cost was not measured. Exact local baseline file hashes are in MORIARTY-EVIDENCE.json; no fresh network revalidation is claimed.
