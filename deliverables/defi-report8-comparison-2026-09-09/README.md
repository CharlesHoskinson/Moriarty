# Report 8 compared with Moriarty

**Recommendation: retain Moriarty's existing taxonomy and formal-language plan; use this report to improve explanations and add two specific research cases.** The report is a financial ontology and standards survey, not an alternative executable language, type system or proof model. Its central architecture substantially agrees with the taxonomy already published in this repository.

Source: **SRC-0106**, [complete immutable Markdown](../../.raw/captured/02e57f7b7616b730a7be59cb7cdc63e3753b007c2d0806393d63a35a9a6c1045.md), 72,017 bytes, 8,837 whitespace-delimited words. SHA-256: `02e57f7b7616b730a7be59cb7cdc63e3753b007c2d0806393d63a35a9a6c1045`. Supplied title: *Modern DeFi Taxonomy and Ethereum Standards Atlas*. The report states a September 8, 2026 cutoff; author and originating citation map are unavailable. Inspection date: September 9, 2026. Read in full. External assertions remain secondary claims unless separately checked below.

[Interactive linked graph](graph.html) · [typed relationships](relationships.json) · [category aliases](category-crosswalk.json) · [independent comparison](INDEPENDENT-COMPARISON.md) · [final artifact audit](FINAL-REVIEW.md) · [remaining roadmap checklist](ROADMAP-REMAINING.md).

## Which model is superior, and for what?

| Dimension | Assessment | Evidence and consequence |
|---|---|---|
| Reader orientation | Report 8 is a useful, compact primer. | Its opening hierarchy, decision rules and action diagrams make the distinction between service, mechanism, claim and interface easy to teach. Reuse this explanatory approach in developer examples. |
| Core taxonomy | Substantial agreement, with sharper boundaries in our existing reference. | Both have eight financial roots and independent facets. Our [seven-step classifier](../modern-defi-taxonomy-2026-09-08/README.md) explicitly handles supporting components with no financial label, incidental transfers/share minting, and solver reimbursement. |
| Capital formation versus treasury | Keep Moriarty's boundary. | Report lines 35 and 86 place treasury deployment under FF.CAP, while its asset-management category also covers delegated allocation. Our [category reference](../modern-defi-taxonomy-2026-09-08/CATEGORIES.md) distinguishes primary funding from treasury portfolio management. A treasury buying bonds is managing capital; a firm issuing bonds to finance itself is raising capital. |
| Standards and provenance | Our existing package is the stronger portable research artifact. | It retains complete JSON/CSV, a schema, source hashes, revision pins and explicit implementation relationships. Report 8 has opaque citation markers and links to three sandbox attachments that were not supplied with this Markdown. Its sample object is illustrative, not a complete schema. |
| New research leads | Report 8 adds useful candidates. | ERC-8161 and ERC-8113 are absent from our 47-profile standards table. Version-specific cases such as Morpho Vault V2 and Lido V3 are leads to source-pin, not established deployments or automatic substitutes for older cases. |
| Financial execution and proofs | Moriarty addresses the necessary engineering problem; completion remains open. | The report has no BNF/EBNF, operational semantics, proof relation or ledger implementation. Moriarty has scoped executable artifacts and explicit acceptance obligations, but the full successor language, K semantics, native recursion and financial Preview acceptance are unfinished. A larger specification is not proof that its implementation is superior. |

The report's cleaner presentation should complement the existing model. Renaming FIN identifiers to FF identifiers or creating another taxonomy would add migration work without new financial behavior.

## Eight-root crosswalk

| Report | Existing Moriarty research ID | Disposition |
|---|---|---|
| FF.MNY | FIN-MON | Alias, with component-level payment/issuance tests retained. |
| FF.EXE | FIN-EXC | Alias; mechanism and settlement topology remain facets. |
| FF.CRD | FIN-CRE | Alias; debt issuance and inventory lending remain distinct subtypes. |
| FF.CAP | FIN-CAP; FIN-MGT.2 for treasury allocation | Conditional mapping; do not merge raising capital with investing it. |
| FF.DER | FIN-DER | Alias; payoff and underlying ownership remain distinguishable. |
| FF.AMG | FIN-MGT | Alias; a container/interface alone is not a management mandate. |
| FF.SEC | FIN-SEC | Alias; security duties distinguish staking from generic reward programs. |
| FF.RSK | FIN-RSK | Alias; loss trigger, priority and discretionary assessment remain explicit. |

These are analyst mappings between research vocabularies, not new Core constructors or accepted semantics. [Machine-readable crosswalk](category-crosswalk.json).

## Two additions worth developing

**Transferable pending rights.** Report 8 identifies ERC-8161 as a discovery lead. The [official specification](https://eips.ethereum.org/EIPS/eip-8161), inspected September 9, is labeled Final and permits transfer of an entire pending request balance under controller/operator authority. Claimable balances are excluded. Deposit and redeem transfer support are independently optional; operator permissions extend to moving pending rights. This is more specific than generic request transferability. Current status does not independently establish its status at the report's cutoff or implementation adoption.

For Moriarty, model request identity, controller, pending quantity, authority and fees separately from claimable rights. Specify a positive transfer and negative unauthorized/duplicate-use cases. A controller change must not recreate an entitlement or expand previously signed spending authority. This fits **SP08.3 and SP10.3**; it is a proposed bounded example, not an ERC backend commitment. The standard's endpoint contract alone does not establish all of Moriarty's global invariants.

**Investor-cohort fee accounting.** The [official ERC-8113 draft](https://eips.ethereum.org/EIPS/eip-8113), inspected September 9, describes per-series accounting to distinguish entry cohorts and high-water marks. It explicitly declares incompatibility with ERC-4626/7540: series-specific queries and redemption units differ. The report mentions the idea but does not surface that integration boundary clearly enough.

For Moriarty, specify a two-cohort example with different entry values, losses, recovery and net performance fees. Keep the cohorts distinct until a justified consolidation transition. This fits **SP03 accounting, SP08.2 and SP11 conformance**. Do not implement a global iterate-over-all-investors operation: the number of cohorts, effects and consolidation steps must be bounded. This is an accounting requirement to investigate, not adoption of the draft interface or a claim of proven fee fairness.

## Lessons already in our roadmap

The [September 9 refinement](../../openspec/ROADMAP-REFINEMENT-2026-09-09.md) already requires:

- Nominal asset/share units; fees and rounding; distinct accounting, redemption, market and stressed valuations.
- Separate conversion, preview, limits and authorization; pending/claimable rights and claim/cancel races.
- Versioned interface semantics, behavioral compatibility and complete effects.
- Explicit source of return, authority, dependencies and exit conditions.
- All original 277 ACTUS fixtures, 72 DeFi rows, TX01–TX12/VX01–VX06 and existing uncovered-leaf tasks.

ERC-7683's resolver redesign is also already documented in our [execution atlas](../modern-defi-taxonomy-2026-09-08/STANDARDS-EXECUTION.md). The [official current specification](https://eips.ethereum.org/EIPS/eip-7683) confirms that it standardizes a solver-facing representation and does not require one escrow or settlement contract. It supplies no automatic cross-chain atomicity or Moriarty proof guarantee.

Therefore this intake recommends bounded refinements inside existing sprints. It does not reopen the whole roadmap, add another backend, freeze semantics or admit a proving campaign.

## Evidence limits and comparison method

The report contains 26 descriptive benchmark rows. Several are related components of the same family; raw counts do not establish greater coverage than our 24-case benchmark. Its capital-formation and shared-security roots lack comparably direct standalone benchmark cases. Its yield-bearing-collateral boundary is discussed but lacks a complete worked collateral lifecycle. Our benchmark also has documented gaps; neither table establishes deployed conformance.

The scope is every named standard and paper in the supplied report, not recursive acquisition of every citation inside those sources. Unnamed basket-token and solvency-proof proposals are referenced only through opaque markers; their exact identities remain unresolved.

The three referenced attachments (`modern_defi_taxonomy_2026.json`, `defi_ethereum_standards_atlas_2026.csv`, `defi_category_standard_crosswalk_2026.csv`) were not part of the supplied input. They were not read, reconstructed as originals or counted as delivered evidence. Citation markers such as `turn...` are retained in the raw source but cannot resolve without the originating session's citation map.

The report, our existing taxonomy and the research prompt share the same four-paper foundation. Conceptual agreement is not independent corroboration of external factual claims. The independent comparison reviews these supplied bytes and repository artifacts; it is not a second empirical study.

Graph edges distinguish report assertions, existing repository relationships and analyst comparison. Confidence describes extraction or mapping, not protocol correctness. Native typed relationships are retained separately from the graph's navigation projection. Token telemetry was unavailable and is recorded as unmeasured.

Expanded source intake: all 40 explicitly named ERC/EIP pages were fetched from the official registry (40 HTTP 200 responses); HTML and extracted main text are retained with hashes in [the acquisition manifest](sources/standards/manifest.json). The four exact paper PDFs were hash-verified and all 80 pages re-extracted, reusing their existing source IDs and prior full-paper analyses. See [paper manifest](sources/papers/manifest.json) and [per-standard relevance](RELEVANCE.md). Thirty-eight standards already had profiles in our atlas; only ERC-8113 and ERC-8161 are new to that set. Dependency-only standards appear as explicitly uninspected graph stubs. The current captures do not silently replace the older pinned corpus or establish September 8 historical state.

No financial or proof test was executed for this report comparison. No new Midnight transaction was submitted. The next useful language demonstration remains **a partial repayment authored in `.mori`, elaborated to typed Core, executed consistently by K and the evaluator, with residual debt preserved**. The existing local repayment reference is only one part of that path.
