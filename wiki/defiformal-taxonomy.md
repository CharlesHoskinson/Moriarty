---
id: defiformal.taxonomy.audit
type: comparison
title: DeFiFormal taxonomy audit and Moriarty mapping
status: active
updated_at: 2026-09-07T16:50:22.642457+00:00
sources:
  - SRC-0070
  - SRC-0071
  - SRC-0072
  - SRC-0015
  - SRC-0016
  - SRC-0017
  - SRC-0047
  - SRC-0048
  - SRC-0049
---

<!-- markdownlint-disable MD025 -->

# DeFiFormal taxonomy audit and Moriarty mapping

## Intents report reconciliation — 2026-09-06

SRC-0048 inspected the report's quoted `QSIGMA-VERDICT.md` and keyword-checker
sources at `8ae0bbfaa3193078d1cabf6999db1382985b7f95`, the existing design-study
pin. The verdict explicitly withdraws its four-primitive invariant; the checker
classifies declaration names and labels the output a keyword seed. This source
observation does not rerun the mathematics or certify a new basis. Preserve the
72-row target corpus and semantic counterexamples while selecting Core from
required behaviors. [CLM-0195](research-journal.md#clm-0195-intents-report-separates-authority-plans-and-receipts)
and the [reconciliation](../docs/research/2026-09-06-intents-report-integration.md)
state the resulting R2b authority/refinement obligations.

## Decision

Use M2+M3 for the human-facing taxonomy: six economic families plus Prediction
Markets over mandatory multi-label facets. Use M5, the formal-behavior profile,
inside the language, specification, and assurance system. Retain D01–D12 and
the earlier M4+ crosswalk only as legacy benchmark and migration artifacts.

**CLM-0110.** The flat twelve-area list mixes product functions, instruments,
mechanisms, infrastructure, asset provenance, and legal/custodial properties.
The repository-verified updated run replaces the earlier M4+ top-level decision
with M2+M3 for human-facing classification and M5 for internal formal behavior.
Source: SRC-0017, “Status and decision”; publication 2026-09-02; authority
descriptive research synthesis; scope design proposal; evidence recommendation;
partially reproduced; confidence medium; status S2.

The recommended primary families are:

- `F1`: exchange and price discovery;
- `F2`: credit and collateralized debt;
- `F3`: derivatives;
- `F4`: consensus-position claims;
- `F5`: tokenized off-chain claims;
- `F6`: delegated asset management; and
- `P`: prediction markets and event-contingent claims.

Required facets cover execution, settlement, custody, legal dependence,
collateral and solvency, oracles, authorization and mandate, and price
discovery. Intents move to the execution facet. Bridges move to settlement or
infrastructure and split by message-verified versus custodial trust. These
identifiers organize packages and evidence; they are not Moriarty Core syntax.

**CLM-0113.** A deterministic local harness independently reproduces 1-NN
label recovery of 47/72 and 3-NN recovery of 50/72 over the protocol element
sets. It also reproduces D12 internal mean Jaccard 0.16 and D12-to-D09 mean
Jaccard 0.27, so D12's external overlap exceeds its internal cohesion. The
best hierarchical-clustering ARI of 0.41 is also reproduced. Average linkage
gives 0.22 at seven clusters and 0.24 at twelve; complete linkage gives 0.35
and 0.3547259508; Ward gives 0.32 and 0.41. The updated run reports the second
complete-linkage value as 0.36, but the independently reproduced value rounds
to 0.35 at two decimal places. Source: SRC-0016, `corpus50/lanes/*.json`, and
`scripts/audit_defiformal_taxonomy_metrics.py`; commit date 2026-08-23;
authority primary repository plus derived experiment; scope experimental
taxonomy audit; evidence experiment observation; reproduction reproduced;
confidence high for reproduced measurements and medium for their taxonomic
interpretation; status S4.

**CLM-0114.** The composition result is independently reproduced at commit
`8ae0bbfaa3193078d1cabf6999db1382985b7f95`: 1,830 eligible pairs, 1,645 clean
compositions, and 185 failures, of which 182 are cross-category and three are
within-category. The exact eligible denominators are 143 within-category pairs
and 1,687 cross-category pairs. The failure rates are 2.0979% and 10.7884%, so
the cross-category rate is 5.1425 times the within-category rate—not sixty
times. Source: SRC-0016, `formal/v2/pairs.mjs` and
`scripts/audit_defiformal_composition_rates.mjs`; commit date 2026-08-23;
authority primary repository plus derived experiment; scope DeFiFormal algebra;
evidence experiment observation; reproduced 2026-09-03; confidence high for
counts and rates and medium for their taxonomic interpretation; status S4.

## Local corpus reconstruction

**CLM-0111.** The authorized clean DeFiFormal checkout at commit
`8ae0bbfaa3193078d1cabf6999db1382985b7f95` contains exactly 72 uniquely named
corpus rows across 12 legacy categories and exactly 60 uniquely named
construction specifications. Direct verdict aggregation produces 1,259 total
obligations: 570 covered and 689 residue; 45 constructions are `PARTIAL` and 15
are `INADMISSIBLE`, with none `COMPLETE`. Source: SRC-0016,
`corpus50/lanes/*.json`, `expansion/*/specs/*.json`, and
`expansion/*/verdicts.json`; commit date 2026-08-23; authority primary
repository artifacts; scope experimental DeFiFormal corpus; experiment
observation; reproduced; confidence high; status S4.

The 72-row counts are D01 5, D02 6, D03 6, D04 5, D05 7, D06 8, D07 7, D08
8, D09 5, D10 5, D11 5, and D12 5. Twelve rows have no construction spec:
Compound V3; crvUSD; Jupiter Perpetual Exchange; GMX V2 Perps; Yearn Finance;
Beefy; Steakhouse Financial in D06; Circle CCTP; Across; DFlow; 1inch; and CoW
Swap. `Steakhouse Financial` also appears as a different D12 product label,
which supports product-level classification rather than organization-level
exclusivity.

The report's public-repository failure is not an error about what its online
environment observed. It is a scope limitation that local authorized access
now resolves. The report's corpus-dependent conclusions remain unreproduced
unless they agree with the pinned local artifacts.

## Two 72-row crosswalks

`evidence/defiformal-72-protocol-m4plus-crosswalk-2026-09-02.csv` preserves the
first report's draft M4+ fields as historical design input.
`evidence/defiformal-72-protocol-family-facet-crosswalk-2026-09-02.csv` applies
the updated family/facet decision to all 72 rows. Primary-family counts are F1
13, F2 12, F3 12, F4 5, F5 13, F6 10, P 3, and infrastructure 4. The mapping
marks 36 rows as bounded-kernel candidates, 33 as outside Core, and three
prediction-market rows as conditional on a specified split/merge primitive.

Sixty construction labels match one-to-one: 56 by normalized exact name and
four by explicit documented alias; fuzzy matching is deliberately not used.

The preserved metric artifacts are
`evidence/defiformal-taxonomy-metrics-2026-09-02.json`,
`evidence/defiformal-taxonomy-nn-predictions-2026-09-02.csv`, and
`evidence/defiformal-composition-rates-2026-09-02.json`. The nearest-neighbour
tie rules and pair eligibility rules are explicit so later runs cannot obtain
the same headline number through a different procedure.

**CLM-0112.** Both crosswalks are migration aids, not normative truth about live
products. They inherit design recommendations from SRC-0015 or SRC-0017 and row
identities from SRC-0016. They do not prove that each product's current behavior
matches its assigned family, subtype, facet, or kernel flag. Source: both
crosswalks and the reconstruction JSON; build date 2026-09-03; authority derived
experimental artifact; scope Moriarty taxonomy design; evidence inference;
reproduced for row and construction counts, not reproduced for every live
product behavior; confidence medium; status S3.

## Moriarty consequence

Moriarty Core should encode reusable bounded behavior, not product labels. The
strongest DeFiFormal residue evidence supports a party or authority sort and a
bounded allocation mandate. Prediction markets motivate a separately reviewed
conditional-token split/merge primitive. Strategies require their own surface
or package level instead of being smuggled into the value algebra.

Product names and the F1–F6/P classes belong in typed libraries, templates,
package metadata, and the benchmark matrix. Solver search, bridge/DVN
operation, sequencers, reserve custody, legal enforcement, identity providers,
validator duties, oracle data acquisition, discretionary portfolio decisions,
and governance stay outside Core as explicit capabilities and assumptions.
Generated Compact/ZKIR can prove deterministic predicates and private
authorization facts; it cannot turn those open-world dependencies into
semantic guarantees.

This produces the canonical demonstration strategy for the 72 rows: one
bounded reference application per behaviorally distinct product pattern,
parameterized into the 72-row manifest. It does not require 72 bespoke language
features or 12 mutually exclusive kernel constructs.

The existing 13 legacy-category strawmen remain useful migration tests. Seven
family-level canonical applications—swap, loan, option, staking claim,
tokenized external claim, bounded mandate, and conditional-token market—become
the primary language demonstrations. Both sets remain non-executable syntax
sketches until the surface grammar, type checker, elaborator, and conformance
tests exist.


## CLM-0919: DeFi report requirements and bounded Midnight interpretation

SRC-0072 proposes typed open transitions, resources, authority, liabilities, effect footprints, assumptions and verified financial libraries (report lines 442-654). It permits unbounded numeric domains at line 664; Moriarty's bounded mandate supersedes that choice. Its twelve holdouts are candidate financial challenges, not additional chain-adapter requirements. Preserve all 72 historical rows and normalize modeled product/version scope without replacing the coverage denominator. Full ACTUS coverage remains 277 fixtures and all 32 taxonomy dispositions.

The [DeFi review](../deliverables/moriarty-report-plan-review-2026-09-07/defi.review.md) distinguishes report claims from retained source findings and missing attachments. Early adversarial semantics constrain a successor Core; complete conformance remains MC07. Model-to-contract fidelity and independent expected observations cannot be replaced by theorem-name certificates or primitive counts.

Metadata: SRC-0070/SRC-0072; observed 2026-09-07; secondary descriptive synthesis and planning recommendation; S2, not executed conformance; confidence high for report statements, unverified for its external citations.
