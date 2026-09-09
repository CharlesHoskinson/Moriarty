# H. Reusable data, evidence method and maintenance

## Package and schema

This package contains complete populated tables, not a sample standing in for absent attachments:

| File | Record unit | Count/scope |
|---|---|---|
| categories.json / .csv | Recommended root or leaf category |31: 8 roots, 23 subcategories |
| standards.json / .csv | Full normative/historical profile |47; historical 1400 explicitly distinguished |
| standard-dependency-stubs.json | Referenced supporting standard metadata |14; metadata inspection, not full recursive semantic audit |
| category-standard-mappings.json / .csv | Category→capability relevance |253; implementation_claim=false |
| cases.json / .csv | Scoped component or documented composition |24; source and financial scope retained |
| paper-crosswalk.json / .csv | Original category→recommended disposition |27 |
| relationships.json | Typed source, taxonomy and integration graph |226 nodes, 619 relationships; parallel types retained |
| graph.json / graph.html | Graphify navigation projection |605 directed pairs, 12 communities |
| source-manifest.json | Preserved evidence file and digest |Includes normative source, history and case captures; not an independent-source count |

[Schema](schema.json) defines required record fields and controlled values. Arrays/objects inside CSV cells use JSON encoding, preserving source lists, multiple functions, histories and structured evidence. CSV is UTF-8 with one header row. Unknown values remain null or explicit “not established”; they are never zero, false or assumed compliance.

Stable identifiers: `FIN-*` functions, `ERC-*`/`EIP-*` standards based on verified category, `CASE-*` examples, `MAP-*` crosswalk rows, `REL-*` typed evidence edges, and P1–P4 historical papers. Node IDs derive from the dossier path and entity identity. Revisions retain the same conceptual standard ID but bind claims to a `source_commit`; incompatible revisions need distinct relationship scopes. New relationships should receive new IDs without renumbering existing records.

A representative populated mapping is MAP-001: FIN-MON → ERC-20, `position_representation`, source commit `f4c23717b6a6fc48436bb6778dcecce29cc5345c`, confidence medium, analyst recommendation, **implementation_claim=false**. It means ERC-20 can represent a money-like unit; it says nothing about an issuer's backing or any named deployed contract. CASE-01 instead identifies historical Maker components and their actual source-scoped interface relation. Both records are necessary.

## Evidence method and dates

The mandatory four PDFs were read in full during the preceding library intake, including all 80 pages and the relevant figures/tables. Their immutable hashes, full analyses and visual inspection coverage remain linked from the paper crosswalk. This study first reconstructed those distinct units and classification rules, then tested a replacement architecture using current normative specifications and structurally diverse examples. The supplied secondary vault report served as a discovery lead, not evidence for its inaccessible references or absent data archive.

Research proceeded in three bounded passes: source/version reconstruction; targeted standards and implementation acquisition; counterexample classification and independent review. Semantic extraction used three bounded research workers covering asset/vault standards, execution/authorization standards, and24components. The root researcher produced the ontology and integrated the evidence. Token telemetry was unavailable; no fabricated cost or token count is reported. No new orchestrator, campaign or chain deployment was needed.

The official ERCs repository is pinned to `f4c23717b6a6fc48436bb6778dcecce29cc5345c` (6September 2026), and EIPs to `991d932f52a56477753cd9f62114b842cd77275c` (8September 2026), the latest inspected commits at or before the declared UTC cutoff. GitHub structured APIs acquired source and revision metadata. Public-document acquisition used Scrapling with available access controls respected. Access failures are preserved as limitations; no bypass or authenticated acquisition was used.

Primary bibliographic searches checked the papers' own arXiv/IACR records, author institutions and publisher leads. Backward/forward discovery used their cited protocol/security work, later arXiv records and the authors' evolving incident repository. This is targeted citation following, not a complete citation-index census. IACR's version endpoint was unavailable under the inspected access policy, and ACM's publisher page returned403. The report does not invent publication dates, absence of revisions or publisher-byte equivalence to fill those gaps.

**Date discipline:** original submission, manuscript revision, empirical window, standard creation, formal status change, material interface revision, code commit, claimed deployment/incident, and access/verification are different fields. Verification took place9September 2026UTC during the local8 September session. Normative and source-code claims use pre-cutoff commits. Undated web documentation is an access-time design statement: its precise pre-cutoff wording is not established unless a dated history/source corroborates it. Current market share, TVL, APY and active deployment states were not extrapolated from these pages.

Each standard has five separate evidence axes: specification status; implementation maturity/code availability; deployment evidence; economic adoption; security evidence. A reference implementation listed in a specification was not automatically compiled or bytecode-matched. An implementation README naming a deployment is documented deployment evidence, not an independent chain observation. Current parameter, legal-right and operational claims require appropriately dated primary records.

No protocol interactions, RPC state snapshots, reserve attestations, economic measurements, exploit reproductions or conformance suites were performed. The executed checks validate the research artifact: JSON/CSV integrity, required fields, references, source digests, graph endpoints, link resolution and independent source/semantic review. These are distinct from financial-system validation.

## Coverage and decision-rule testing

The 24 cases span all 8 roots; leaf assignments and coverage counts are exported in `category-coverage.json`. Some subcategories are represented only by the original papers, normative interfaces or a design definition, and are not independently exercised by a dedicated benchmark case. In particular, conditional crowdfunding, managed liquidity and default-backstop subcategories need additional dedicated cases before claiming leaf-wide implementation validation. The benchmark is a classification reference, not measured agreement across an exhaustive product population.

An independent six-description rule check was actually performed, with reviewed input hashes in `sources/assets/classifier-review.json`. It was not blinded: the reviewer could see similar examples in the category reference. It found ambiguities about ordinary vault-share issuance versus capital raising, solver reimbursement versus lending, pure CDP issuance versus inventory lending, and no-function capability outcomes. Those rules were tightened. No inter-rater agreement percentage is claimed. A fresh GPT-6 Astra review separately checks the substantive dossier and evidence; its exact scope and remaining limits are in [REVIEW](REVIEW.md).

## Required design-space coverage

| Area | Canonical destination and covered distinctions | Evidence/validation boundary |
|---|---|---|
| Money, payments, settlement | MON.1–3: reserve/peg/redemption/yield are separate; fiat cash dependencies; streams and escrow | Maker, Circle, Sablier, Across escrow; no current reserve attestation |
| Exchange | EXC.1–2: AMM/concentrated, CLOB, RFQ, auctions/batches, routing/solvers, hooks; MGT.3 for managed LP | UniswapV3/V4, CoW, Drift, Osmosis; hooks not new financial roots |
| Credit | CRE.1–3: pooled/isolated/peer-matched, underwriting/credit delegation or controlled accounts, fungible/NFT collateral, term/open/callable, fixed/variable, early-liquidating/non-liquidating, default and flash | Aave/Morpho/Goldfinch plus Kotzer for broader mechanisms; NFT/controlled-account and non-liquidating loans lack dedicated current benchmark deployments |
| Capital formation/treasury | CAP.1–2: primary subscriptions/auctions, bonding curves, crowdfunding; treasury allocation MGT.2; governance-right auctions EXC if they trade rights | Balancer primary-offering use; crowdfunding and vote-incentive markets defined but no separate benchmark; emissions alone not a category |
| Derivatives/event claims | DER.1–4: linear/perpetual/future, optional/structured, event resolution, rate/yield transformation; payoff and funding distinct from AMM/orderbook | Opyn, GMX, Drift, ConditionalTokens, Pendle; not a census of rate swaps |
| Management | MGT.1–3: single/multiple strategies, indexes, treasury, LP management, curated lending, leverage and nesting | MetaMorpho, Yearn, Centrifuge; index/managed-LP leaves need dedicated cases |
| Yield/rate markets | DER.4 when transforming future yield; CRE for debt; MGT for allocation; wrapper is representation | Pendle maturity collateral boundary; current5095/5115 adoption not assumed |
| Staking/shared security | SEC.1–3: validator/delegation, liquid receipt, restaking, liquid-restaking receipt, service allocation and penalties | Lido and EigenLayer; liquid restaking composes SEC.2+SEC.3, not ordinary leveraged farming |
| Risk transfer | RSK.1–3: contractual/discretionary cover, tranches, first loss, guarantees/backstops | Nexus and Goldfinch; dedicated backstop operation remains coverage work |
| External assets/hybrid finance | Underlying/trust facets across MON/CRE/CAP/MGT; cash/government debt/private credit/equity/commodity claims described by rights | Circle, Goldfinch, Centrifuge and token standards; no separate equity/commodity deployed case or legal-opinion claim |
| Cross-chain finance | Execution/dependency facets plus relevant EXC/MON/CRE/MGT functions; message, verification, token movement, liquidity, coordination, finality separate | Across and Centrifuge architecture; no generic cross-chain collateral or distributed-vault conformance proof |
| Supporting capabilities | Oracles, keepers, automation, governance, accounts, permissions, MEV services, privacy and agent operation | Standards atlas and paper layers; agent-controlled signatures/strategy are authority facets, no new function without an economic duty |

## Maintenance procedure

1. Add the new object at its correct granularity. Reuse a function ID if its inclusion test still fits; add a facet or mechanism before creating a new financial root.
2. Preserve original source bytes, retrieval metadata, source URL/version and content digest. Keep duplicate formats and work versions linked without counting them as independent corroboration.
3. For a standard update, compare actual changed clauses, status and dependencies. Record material revision dates separately from editorial commits. Re-evaluate affected adapters and cases instead of replacing history.
4. Bind implementation claims to code/deployment versions and exact evidence predicates. Add deployed-bytecode or execution evidence when available; do not overwrite “not tested” merely because code exists.
5. Add typed relationships and a counterexample to nearby categories. If classifiers disagree, refine an inclusion/exclusion rule and preserve the disagreement before proposing a new root.
6. Regenerate/validate tables and graph, verify local links and hashes, and obtain independent review of consequential semantic changes. Canonical Moriarty language adoption is a separately scoped decision.

Immutable paper history need not expire. Recheck evolving Draft/Review/LastCall standards before any integration decision and at each quarterly atlas refresh. Recheck deployment/admin/parameter and external-asset eligibility claims at the actual decision date. No fixed refresh interval makes a stale operational claim current.

## Gap ledger

| Gap or disputed boundary | Current disposition | Concrete next evidence |
|---|---|---|
| Complete financial classification | Eight roots recommended; multi-label components and zero-label capabilities allowed | Additional independent unfamiliar-case tests, especially uncovered leaves |
| Standardized universal lending/AMM/derivatives interface | Protocol-specific conventions dominate inspected examples; no universal proposal established by this search | Scoped registry/source search plus competing proposals and deployment comparison |
| Debt, principal and yield standards | Proposals exist; broad adoption not established here | Version-bound implementation/deployment inventory and conformance suites |
| Permissioned asset standards | Competing scopes:3643identity architecture,7943control interface,1400historical family | Actual instrument governing docs plus implementation-policy mapping |
| Current7683 resolver adoption | Old settler use is not current adoption; precise pinned redesign established | Resolver implementation, declared assumptions, version-matched integrations and execution evidence |
| Async fund composition | Architecture verified at source level; exact external fund→vault deployment binding missing | Fund terms, chain/address/code pin, actual request/claim trace |
| ERC7887 cancellation | Draft with internal inconsistencies; no interpretation silently selected | Corrected revision or implementation-specific documented deviation |
| ERC8330 valuation | Review proposal; intended behavior inspected, production adoption unknown | Provider/consumer implementation and stale/corrected-NAV tests |
| Historical paper revisions | Exact attachments retained; later records distinguished; inaccessible endpoints remain limited | Publisher/version bytes and exact equality/diff if accessible |
| External equity/commodity and controlled-credit cases | Covered as claim/mechanism facets; not dedicated empirical cases | Dated issuer terms, contract code and independent scope classification |
| Solvency, legal enforceability and safe composition | Not implied by tokens/interfaces or this taxonomy | Appropriate reserve/legal/execution evidence and model-relative tests |
| Moriarty implementation | Research recommendation only | Scoped adoption, actual syntax/semantics/adapter tests and separate Midnight settlement evidence |

Library collection: SRC-0105, [immutable source bundle](../../.raw/captured/5b1e5408d69f6ef6a282a7b9962829070e31f4502183cb1500b554bf6ce8eb17.json). Individual member hashes and metadata preserve independence/version scope; the bundle is an archival container, not a new corroborating author.

Viewer note: graph.html uses the Graphify interactive viewer and a pinned external vis-network script; it is not advertised as an offline self-contained viewer. The JSON and Markdown remain independently readable.
