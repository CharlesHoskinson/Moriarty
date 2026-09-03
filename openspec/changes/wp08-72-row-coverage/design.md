# Design: WP08 72-row protocol coverage

## Context

The 72 products contain bounded state-machine kernels and substantial
off-chain, legal, solver, bridge, oracle, custody, and governance behavior.

## Inputs

- WP02 roster at
  `evidence/defiformal-72-protocol-family-facet-crosswalk-2026-09-02.csv`.
- WP03 construct and assurance delta.
- WP04 semantic scope ledger.
- WP07 canonical application artifacts.

## Outputs

- One coverage row per stable protocol identifier.
- A bounded parameterization or outside-kernel manifest for every row.
- Unsupported behavior, trust, proof, and SDK requirements.
- Reviewer identity, decision, confidence, and evidence links.
- `evidence/wp08/legacy-regression-results.json` for MR-01 through MR-13.

## Decisions

Coverage is multi-valued. Use `bounded-instance`, `partial-kernel`,
`outside-kernel`, `library-preferred`, or `unsupported`. Never reduce these to a
misleading boolean.

## Failure Handling

Missing, duplicated, or unreviewed rows fail the package. An outside-kernel
result is acceptable when it names the boundary precisely.
Repository review records use pseudonyms. A custodian keeps the private identity
mapping and reviewer-independence evidence outside the public corpus.

## Verification

Join against the pinned 72-row roster. Validate closed values, unique IDs,
artifact links, reviewer state, and the thirteen legacy regressions.
The roster SHA-256 is
`21921f986edca676c2cf501e60c9463ef29a3e5175f86f9ffa9b193814d43e4d`.
Run `uv run python scripts/validate_sprint_evidence.py --package WP08 --manifest
openspec/work-packages.json`.
