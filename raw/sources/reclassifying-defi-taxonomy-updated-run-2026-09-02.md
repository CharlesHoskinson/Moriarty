# Reclassifying DeFi — repository-verified updated run

Research date: 2026-09-02  
Received in the Moriarty research thread: 2026-09-03T01:22:22Z  
Source class: user-supplied research synthesis  
Repository baseline used by Moriarty: `CharlesHoskinson/defiformal@8ae0bbfaa3193078d1cabf6999db1382985b7f95`

## Status and decision

This memorandum records the decision-bearing claims from the user-supplied
updated run. It supersedes the repository-unreachable premise in SRC-0015. The
original message is more detailed than this durable memorandum. Moriarty must
cite the pinned repository or a reproduced local experiment, rather than this
summary, for claims that can be checked from code.

Do not preserve the twelve DeFiFormal labels as flat top-level classes. Use a
human-facing hierarchy of six economic families plus Prediction Markets over a
mandatory facet layer. Use a formal-behavior profile as the internal DSL and
verification taxonomy. Keep D01 through D12 only as legacy sampling and
benchmark labels.

The recommended families are:

- `F1`: Exchange and price discovery.
- `F2`: Credit and collateralized debt.
- `F3`: Derivatives, including perpetuals and options.
- `F4`: Consensus-position claims, including liquid staking and restaking.
- `F5`: Tokenized off-chain claims, including reserve-backed monetary claims,
  real-world assets, and custodial wrapped claims.
- `F6`: Delegated asset management, including vaults, risk curators, and
  on-chain capital allocators.
- `P`: Prediction markets and event-contingent claims.

Mandatory facets are execution architecture, settlement domain, custody and
asset control, legal or off-chain dependence, collateral and solvency model,
oracle dependence, authorization and mandate, and price-discovery mechanism.
Intents are primarily an execution facet. Bridges split by trust model and are
primarily a settlement facet or infrastructure dependency.

## Repository-derived results reported by the updated run

- The corpus contains 72 distinct protocol or product labels across 12 legacy
  categories. Sixty received construction specifications and obligation
  ledgers; 12 rank-six-or-lower rows in over-full categories did not.
- Leave-one-out nearest-neighbor label recovery is reported as 47/72 (65%) for
  1-NN and 50/72 (69%) for 3-NN. D12 has 0/5 1-NN recall.
- The best reported hierarchical-clustering agreement with the twelve labels
  is ARI 0.41. D12 is the only category whose best external overlap exceeds
  its internal cohesion.
- Of 1,830 composable pairs, 1,645 compose and 185 fail. Of the failures, 182
  are cross-category and three are within-category. The rate-adjusted effect is
  about fivefold, not the roughly sixtyfold impression created by raw counts.
- The sixty constructions contain 1,259 obligations: 570 covered and 689
  residue. Coverage is weakest for bridges and intents.
- The most strongly evidenced missing language abstractions are a party or
  authority sort, a bounded allocation mandate, a strategy level outside the
  value algebra, and conditional-token split and merge.

Moriarty independently reproduced the 72-row and 60-construction rosters, the
1,259/570/689 obligation totals, the 61/72 laws-and-warrants result, the 1,830
pair count, the 185 failures with 182/3 split, the four canonical collisions,
the 29/72 canonical compression result, and the 22/22 v3 self-test at the
pinned repository commit. The nearest-neighbor, Jaccard, ARI, and held-out
classification figures still require a preserved independent harness before
they become Moriarty experiment observations.

## Moriarty implications

The family taxonomy organizes packages, examples, user documentation, and the
72-row demonstration matrix. It must not determine the semantic constructors.
Moriarty Core is organized by bounded formal behavior: parties and authority,
assets and conservation, finite obligations and transitions, explicit waits
and timeouts, bounded mandates, typed attestations, and conditional settlement.

The initial family applications are:

- F1 constant-product or bounded-batch swap;
- F2 over-collateralized loan;
- F3 fully collateralized option or finite-horizon derivative;
- F4 liquid-staking accounting claim;
- F5 reserve-backed or tokenized external claim with explicit assumptions;
- F6 bounded allocation mandate; and
- P conditional-token market with split and merge.

Keep solver search and auctions, bridge relaying and remote finality, reserve
custody, legal recourse, register-of-record facts, oracle publication, validator
duties, and discretionary strategy selection outside Core. Compact circuits
and ZKIR can prove predicates over supplied statements; they cannot prove those
open-world statements are honest or available.

## Limits carried forward

The updated run's held-out classification is a simulated rule-set exercise, not
independent human inter-rater evidence. External market figures are moving
facts and need their own Scrapling receipts. The source archive used by the
updated run did not preserve a Git commit hash; Moriarty instead binds its
reproductions to the independently available clean checkout named above. No
Moriarty, Compact, ZKIR, or Midnight implementation was found inside
DeFiFormal; that architecture remains a separate synthesis and must not be
described as repository history.
