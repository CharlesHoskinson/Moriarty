# Design: WP10 real proof and cost

## Context

E00 produced ZKIR accepted by a mock compiler. Required proving parameters are
not present. Product budgets must come from a real supported execution path.

## Inputs

- WP07 canonical application artifacts.
- WP09 manifests, verifiers, and measurement schema.
- Pinned proving parameters, compiler, Runtime, wallet, network, and protocol versions.

## Outputs

- Key-generation, proving, verification, and submission receipts.
- Constraint, key, proof, memory, latency, state, transaction, and fee distributions.
- Cold and warm measurements with hardware and provider descriptions.
- Comparable language and audited-library measurements.

## Decisions

Report distributions, not one favorable run. Separate local resource cost from
network fees. Preserve failed proofs and rejected transactions.

## Failure Handling

Missing real parameters leaves the experiment blocked. Cost outside the
approved budget selects scope reduction, library-only, or stop.

## Verification

Repeat runs across approved hardware profiles. Verify every proof independently.
Bind all measurements to exact artifact and toolchain digests.
