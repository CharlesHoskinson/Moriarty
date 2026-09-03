# Design: WP01 backend stop test

## Context

The repository already contains the Core swap, generated Compact, four ZKIR
circuits, a certificate, a manifest, and differential tests. This package
freezes their contract and makes the stop rule repeatable.

## Inputs

- Moriarty commit `006c4d91ed09c0a89261861b6e7203b3efa3e2df`.
- Compact commit `11e7ec5abeecb99297c4faa74d30ef9adc7b51f3`.
- ZKIR commit `2ffe2d17bbb736aec36fb300aeaca679a10d2278`.
- The compiler tuple in `experiments/moriarty-core-swap/toolchain-results.json`.
- The `seeded-boundary-prefix-plus-lexicographic-product-v2` trace algorithm.
- No random seed. The algorithm deterministically selects 1,000 unique traces.

## Outputs

- Generated Compact and ZKIR artifacts.
- A static-bound report and disclosure manifest.
- A translation-validation certificate.
- Raw results for at least 1,000 deterministic traces.
- A negative compilation result for an undeclared disclosure.
- `evidence/wp01/reproduction-receipt.json` from a clean pinned environment.

## Decisions

The Core stays finite. The generator rejects constructs without static bounds.
The client must verify artifact identity before signing. Generated code does not
inherit semantic guarantees without certificate validation. The third-party
compiler's disclosure rejection is separate from Moriarty's manifest-versus-
generated-artifact disclosure validation.

## Failure Handling

Any semantic, authorization, conservation, or boundedness failure selects stop.
A correspondence or Compact-language-generality failure selects the pinned,
audited Compact-library baseline. The selection is recorded without widening
Core.

## Verification

Run the commands in `experiments/moriarty-core-swap/README.md`. Validate the
package with `uv run python scripts/validate_sprint_evidence.py --package WP01
--manifest openspec/work-packages.json`. Compare every digest in the reproduction
receipt. Re-run after WP04 changes the semantic-scope digest.
