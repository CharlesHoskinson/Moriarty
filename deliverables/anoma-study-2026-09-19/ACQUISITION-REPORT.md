# Anoma source acquisition — 19 September 2026

The bounded acquisition is complete. It contains 19 nonempty, shallow official repository checkouts at recorded commits, 4,943 tracked files and 410 documentation/PDF files. The repository set is frozen. `repo-manifest.json` records origins, commits, commit dates and license files; `file-inventory.json` records the hash and pinned GitHub URL of every acquired tracked file. These are source snapshots, not a build or a tested combined release.

## Finite documentation scope and result

Scrapling 0.4.15 Fetcher captured public pages from docs.anoma.net, specs.anoma.net and docs.juvix.org. Discovery used robots, sitemaps, seed pages and same-scope links. The current scope used the Anoma documentation root plus `/latest/` paths for specifications and Juvix. A separate explicit `/v1.0.0/` specification scope was necessary because current navigation links to that version. Other historical versions were excluded. Query URLs, static assets and unrelated hosts were excluded; PDFs are inventoried separately.

The current scope closed after six rounds and 1,135 requests (1,058 HTTP 200; 77 HTTP 404). The versioned scope closed after seven rounds and 338 requests (237 HTTP 200; 101 HTTP 404). The 4,000-discovery cap was not reached. Combined host counts are: docs.anoma.net 20 successful pages; docs.juvix.org 955 successes and 40 failures; specs.anoma.net 320 successes and 138 failures. Generated Juvix example/Stdlib source views are included in the finite closure. An HTTP 200 is acquisition evidence, not an assertion that every page contains a distinct substantive specification.

`docs/source-manifest.json` combines all 1,473 request records with original/final URL, status, capture paths, raw/text SHA256, retrieval time and line count. The per-scope link inventories and batch manifests preserve how each page was discovered. All captured raw/text hashes verified successfully. The 178 HTTP 404 records remain visible; link closure does not turn failed links into acquired content. There is no claim to have discovered hidden or unlinked pages, every official subdomain, all GitHub history, or every organization repository.

## Repository boundary

The selection includes the node and split core dependencies, ARM RISC0, Cairo ARM, EVM protocol adapter, specifications, Juvix compiler/docs/stdlib/Anoma stdlib/Lean backend, two SDKs, AnomaPay resource/forwarder and generic-call resource/forwarder. The separate official `generic-call-forwarder` repository is present at `b95b0cac0d219d1c27fd2fdd2b0f9d4435bb3476`; its contract is `contracts/src/GenericCallForwarder.sol`.

The selection excludes historical/experimental ARM alternatives and unrelated organization projects. It does not recursively vendor every package dependency or initialize a complete build environment. In particular, pinned dependency references remain more authoritative for compatibility than independent repository HEADs. The node's Cairo dependency reference differs from the independently acquired Cairo HEAD. No compiler, wallet, contract deployment or financial transaction was executed. Parent-owned graph output and independent language/security/architecture studies are separate analysis artifacts.

## Version and proof interpretation

The ARM README distinguishes development-mode tests from proof production, requires reproducible builds to associate circuit source with ELF/ImageID, and describes aggregation of compliance and logic proofs within a transaction. That description alone does not establish recursive historical compliance. Generic Call Forwarder's README identifies an older audited commit; an audit PDF in a checkout is not proof that its current HEAD was audited. Independent security review also identifies differences between current Rust, Solidity and versioned specifications; these must not be presented as one tested stack.

The market comparison under `/home/charl/research/moriarty-market-landscape-2026-09-19/intent-systems` gives the bounded architectural conclusion: Anoma is strong prior art for proof-checked intent/resource composition. This does not establish equivalence to Moriarty's desired private, staged, conditional multichain settlement, nor prove that no existing system has that conjunction. Acquisition volume is not reading coverage.

## License and PDF handling

`license-inventory.json` records exact license text hashes and package license declarations. The set includes MIT, Apache-2.0 and GPL-3.0 material. Some repositories lack a standalone license file; absence is not permission to reuse. This inventory does not resolve derivative-work or linking questions.

`pdf-inventory.json` records nine repository PDF locations representing four unique hashes. All four unique PDFs (211 physical pages) are rendered with the installed PixelRAG `pixelshot` CLI at 90 DPI. The existing 99-page security-study render was reused after exact source-hash matching; the other three PDFs were rendered separately. `pdf-processing.json` records every source alias, render-manifest hash and tile hash. The security agent visually inspected physical pages 4 and 6 of the historical Informal audit; this acquisition agent has not visually read any PDF pages. Rendering is not full reading or a current-HEAD audit certification. Paper captures outside repository paths remain under their owning study manifests.

## Reproduction and limits

`discover_docs.py`, `crawl_docs.py`, `crawl_docs_v1.py`, `clone.py` and `finalize_acquisition.py` preserve the process. The two crawls used three workers and a short per-request delay. Bootstrap attempts and failures remain under `discovery/`. Re-running network acquisition would obtain new mutable pages/HEADs; use the recorded hashes and commits to reproduce this evidence set. `acquisition-verification.json` reports closure, counts, hash checks and date checks. There are no future-dated repository commits in the selected set.

## Captured-document hyperlink graph

`graphify-out/docs-hyperlinks/graph.json` contains 3,311 URL nodes and 26,113 unique directed links; 1,473 nodes have capture/request records, with 13,189 links between captured URLs. `stats.json` records exact input hashes, degree counts and capture status. Linked but uncaptured destinations are labeled explicitly. This is a literal hyperlink graph built from captured inventories, not a semantic entailment graph or code dependency model. Repeated navigation links can dominate degree counts.
