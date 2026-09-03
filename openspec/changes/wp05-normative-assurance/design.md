# Design: WP05 normative assurance

## Context

Marlowe has Isabelle, Agda, Haskell, Plutus, and TypeScript semantics with
uneven correspondence. Moriarty must choose authority before expanding Core.

## Inputs

- WP03 theorem and implementation delta.
- WP04 accepted candidate semantics.
- Available maintainers, toolchains, extraction paths, and CI constraints.

## Outputs

- `evidence/wp05/assurance-results.json` scored under `assurance-scorecard.json`.
- Executable prototypes for all three frozen high-risk obligations.
- A normative artifact and derived-artifact policy.
- `evidence/wp05/maintainer-evidence.json` and `toolchain-lock.json`.
- `evidence/wp05/prototype-results.json` and `evidence-manifest.json`.

## Decisions

The selected strategy must support executable semantics, maintainable proofs,
and backend correspondence. Independent implementations may remain only with a
defined conformance role.
The two highest viable strategies prototype all three obligations. Strategy
scores, uncertainty penalties, viability thresholds, ties, and maintainer
criteria are frozen in `assurance-scorecard.json` before prototype results.

## Failure Handling

If no strategy has credible maintainers and integration, select audited Compact
libraries. Do not create a new prover formalization for prestige.

## Verification

Build and run each prototype in a pinned environment. Record axioms, admitted
lemmas, extraction gaps, build time, artifact hashes, and maintainer evidence.
Run `uv run python scripts/validate_sprint_evidence.py --package WP05 --manifest
openspec/work-packages.json`.
