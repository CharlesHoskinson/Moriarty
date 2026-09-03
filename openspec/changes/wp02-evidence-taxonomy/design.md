# Design: WP02 evidence and taxonomy

## Context

The pinned DeFiFormal corpus contains 72 protocols and 60 constructed protocol
models.
The reproduced labels support M2+M3 for human navigation and M5 internally.

## Inputs

- DeFiFormal commit `8ae0bbfaa3193078d1cabf6999db1382985b7f95`.
- The 72-protocol and 60-constructed-protocol-model rosters.
- Existing metric outputs and contradiction records.

## Outputs

- `evidence/wp02/jaccard-matrix.json`.
- `evidence/wp02/composition-matrix.json`.
- `evidence/wp02/clustering-metrics.json`.
- `evidence/wp02/residue-689.csv`.
- `evidence/wp02/rater-handbook.md` and blinded rating records.
- `evidence/wp02/evidence-manifest.json`.

## Decisions

Classify deployed products, not organizations. Use one economic family and
multiple facets. Keep infrastructure facets separate from product families.
Use exact set Jaccard. Use average, complete, and Ward linkage with pinned
library versions. Sort stable product identifiers before every tie. Preserve
full precision in JSON and round displays to two decimal places.

Human raters use pseudonyms in repository artifacts. A private custodian keeps
the identity map. The protocol records consent, withdrawal, retention, deletion,
access control, and incident handling before collection.

## Failure Handling

Missing rows or irreproducible metrics fail the package. A simulated rating may
test procedure mechanics, but it cannot support an agreement claim.

## Verification

Recompute from pinned JSON. Compare counts and hashes. Review every excluded or
ambiguous row. Preserve raw rater decisions before adjudication. Run `uv run
python scripts/validate_sprint_evidence.py --package WP02 --manifest
openspec/work-packages.json`.
