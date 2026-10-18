# Compiler correspondence draft

## Why
A financial source acceptance does not demonstrate generated spending-validator execution. Current evidence separates source physics from the narrow public atomic native baseline. This package specifies the missing compiler correspondence predicates. Those predicates can change a scoped compatibility decision after independent execution. They cannot close the global Midnight release decision.

Status: S2 written proposal. All implementation and execution tasks are specified-only.

## What Changes
- Define version routing and exhaustive source-derived Core lowering.
- Define complete lifecycle representation and actual generated spending tests.
- Require digest, runtime pin, work, and artifact failure controls.
- Map the 20 source families and T1–T18 to explicit dependency boundaries.

The revision03 T1 design votes remain scoped to their original candidate. Broader constructors, financial kernels, and lifecycle schema changes are separate amendments. Revision01 and revision02 CHANGES_REQUIRED remain preserved.

## Capabilities
### New Capabilities
- `eutxo-compiler`: source-to-generated-validator correspondence and compiler artifact admission.

### Modified Capabilities
None. Existing Midnight OpenSpecs retain their acceptance gates.

## Impact
Prospective changes affect `experiments/moriarty-eutxo/src/financial-profile-router.mjs`, `core-inventory.mjs`, `lower-financial.mjs`, `lifecycle-codec.mjs`, and `compile-financial.mjs`. New tests affect the experimental compiler test directory. These paths are proposals, not existing public APIs. Existing custody modules are moving Grok implementation inputs and are read-only for this draft.

Non-goals: implementation, commits, native execution, wallets, keys, submissions, production backend changes, new orchestration, private proof construction, or approval of this draft.

## Provenance
Base commit: `84f64b30df53ff0b8ccaf475f57324d3d7c6e594`.
Approved T1 candidate manifest SHA-256: `dcc62e21e9e351843bd04a3be537ebe260f8c1d0ec606cfcc5e15fd83ca7aff0`.
Independent source packet SHA-256: `9d71e736e3d3c159a83ff3bb21dbb3f5b2a9661dd90ed1e70414c2690b6552d6`.
Observed input hashes and coverage are in `/home/charl/moriarty-eutxo-validation-2026-10-12/sol-validation-drafts-20261012/01-compiler/`.
