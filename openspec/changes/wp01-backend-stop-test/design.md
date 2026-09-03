# Design: WP01 backend stop test

## Context

The repository already contains the Core swap, generated Compact, four ZKIR
circuits, a certificate, a manifest, and differential tests. This package
freezes their contract and makes the stop rule repeatable.

## Inputs

- The repository commit recorded in the experiment manifest.
- The pinned Compact compiler and ZKIR mock compiler tuple.
- The Core swap source and deterministic trace seed.

## Outputs

- Generated Compact and ZKIR artifacts.
- A static-bound report and disclosure manifest.
- A translation-validation certificate.
- Raw results for at least 1,000 deterministic traces.
- A negative compilation result for an undeclared disclosure.

## Decisions

The Core stays finite. The backend rejects constructs without static bounds.
The client must verify artifact identity before signing. Generated code does not
inherit semantic guarantees without certificate validation.

## Failure Handling

Any divergence, undeclared disclosure, or unbounded witness path fires the stop
gate. The fallback is an audited Compact library or project termination.

## Verification

Run focused Core, bound, lowering, and certificate tests. Compile the positive
artifact. Confirm the negative artifact fails. Preserve exact commands and raw
results.
