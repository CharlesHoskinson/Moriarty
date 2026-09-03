# Design: WP08 72-row protocol coverage

## Context

The 72 products contain bounded state-machine kernels and substantial
off-chain, legal, solver, bridge, oracle, custody, and governance behavior.

## Inputs

- WP02 pinned roster and taxonomy.
- WP03 construct and assurance delta.
- WP04 semantic scope ledger.
- WP07 canonical application artifacts.

## Outputs

- One coverage row per stable protocol identifier.
- A bounded parameterization or outside-kernel manifest for every row.
- Unsupported behavior, trust, proof, and SDK requirements.
- Reviewer identity, decision, confidence, and evidence links.

## Decisions

Coverage is multi-valued. Use `bounded-instance`, `partial-kernel`,
`outside-kernel`, `library-preferred`, or `unsupported`. Never reduce these to a
misleading boolean.

## Failure Handling

Missing, duplicated, or unreviewed rows fail the package. An outside-kernel
result is acceptable when it names the boundary precisely.

## Verification

Join against the pinned 72-row roster. Validate closed values, unique IDs,
artifact links, reviewer state, and the thirteen legacy regressions.
