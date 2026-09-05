# s02-model-comparison

Status: specified-only

No architecture has been selected. This package defines the acceptance contract
for the bounded S02 comparison. Its acceptance predicates are not passed results,
and its output is not execution evidence. The frozen S01 judgment remains
`candidate-unmechanized` and grants no signing authority.

This package does not establish all 277 ACTUS vectors, real ledger execution,
proof generation, Compact or production correspondence, production cost, or
human preference. It creates no model, run receipt, counterexample, witness,
validation result, or comparison decision.

## Dependencies

- The completed and independently reviewed S01 package, including its evidence
  manifest and architecture-neutral intent-safety judgment.
- The independently reviewed S02 bounded architecture comparison design.
- The exact closed registry at
  `evidence/s02-model-comparison/requirements.json`.

S01 completion is a precondition for S02 experiment execution. This contract is
planning, not evidence that execution occurred.

## Immutable inputs

- Prompt XML version `1.3`, SHA-256
  `86b80dd1cbd14d1e5759988be9f619355495fc10c4e2fb6b6d9162367670ddcd`.
- Semantic scope `0.0.0-e00.2`, SHA-256
  `9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`.
- The completed S01 evidence manifest and the frozen S01 judgment with proof
  status `candidate-unmechanized`.
- Frozen `moriarty/core.py` and `moriarty/swap.py` inputs.
- The pinned Compact swap specialization named in
  `experiments/moriarty-core-swap/artifact-manifest.json` for candidate D.

## Exact outputs

Future S02 execution must produce four distinct `.qnt` representations, separate
scenario-test modules, a requirement/operation/property coverage map, an
independent E00 correspondence report, negative-control traces, raw run receipts,
a pinned toolchain inventory, a comparison selection-or-stop decision, a
recomputed validation report, and an evidence manifest. No digest is assigned
to an output before that output exists.

## Acceptance predicates

The ten normative predicates S02-01 through S02-10 are specified in
`specs/architecture-comparison/spec.md`. They require completed checks and
decisive evidence while separately determining which candidates are eligible.
A demonstrated unsafe candidate can be rejected while an independently safe
candidate remains selectable. Missing, stale, simulation-only, or inconclusive
evidence blocks the package.
