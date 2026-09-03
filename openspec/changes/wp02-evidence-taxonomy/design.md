# Design: WP02 evidence and taxonomy

## Context

The pinned DeFiFormal corpus contains 72 protocols and 60 constructed models.
The reproduced labels support M2+M3 for human navigation and M5 internally.

## Inputs

- DeFiFormal commit `8ae0bbfaa3193078d1cabf6999db1382985b7f95`.
- The 72-row and 60-construction rosters.
- Existing metric outputs and contradiction records.

## Outputs

- Full pairwise Jaccard and composition matrices.
- Hierarchical clustering, ARI, NMI, and deterministic tie rules.
- A 689-row residue classification with reviewer state.
- A rater handbook, blinded sample, and adjudication procedure.

## Decisions

Classify deployed products, not organizations. Use one economic family and
multiple facets. Keep infrastructure facets separate from product families.

## Failure Handling

Missing rows or irreproducible metrics fail the package. A simulated rating may
test procedure mechanics, but it cannot support an agreement claim.

## Verification

Recompute from pinned JSON. Compare counts and hashes. Review every excluded or
ambiguous row. Preserve raw rater decisions before adjudication.
