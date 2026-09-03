# Design: WP10 real proof and cost

## Context

E00 produced ZKIR accepted by a mock compiler. Required proving parameters are
not present. Product budgets must come from a real supported execution path.

## Inputs

- WP01 backend evidence and its explicit limitations.
- WP05 selected assurance strategy and proof obligations.
- WP06 high-risk composition disposition and artifacts.
- WP07 canonical application artifacts.
- WP09 manifests, verifiers, and measurement schema.
- Pinned proving parameters, compiler, Runtime, wallet, network, and protocol versions.

## Outputs

- `evidence/wp10/measurement-protocol.json` frozen before measurement.
- `evidence/wp10/library-baseline-lock.json` with audit and provenance status.
- `evidence/wp10/proof-cost-samples.ndjson` and `proof-cost-summary.json`.
- `evidence/wp10/testnet-authorization.json` when submission is authorized.
- `evidence/wp10/evidence-manifest.json`.

## Decisions

Report distributions, not one favorable run. Separate local resource cost from
network fees. Preserve failed proofs and rejected transactions.
WP10 owns selection, pinning, and audit qualification of the Compact-library
baseline. Every sample records prover endpoint identity, parameter digest,
circuit digest, and the independent verifier result.

## Failure Handling

Missing real parameters leaves the experiment blocked. Cost outside the
approved budget selects scope reduction, library-only, or stop.
No network submission occurs without a named authority and signed authorization
record. Mainnet submission is outside this package.

## Verification

Repeat runs across approved hardware profiles. Verify every proof independently.
Bind all measurements to exact artifact and toolchain digests.
The decision authority approves the measurement protocol before results. Its
digest is recorded in `wiki/research-journal.md`. Run `uv run python
scripts/validate_sprint_evidence.py --package WP10 --manifest
openspec/work-packages.json`.
