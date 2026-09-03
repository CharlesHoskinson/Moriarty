# Tasks: WP05 normative assurance

## 1. Define comparison inputs

- [ ] Freeze the candidate Core semantics from WP04.
- [ ] Define shared transition and serialization vectors.
- [ ] Inventory existing Marlowe proof artifacts and licenses.
- [x] Freeze `assurance-scorecard.json` with weights, thresholds, ties, and uncertainty.

## 2. Prototype strategies

- [ ] Score all five strategies before selecting prototype candidates.
- [ ] Prototype the two highest viable strategies under all three obligations.
- [ ] Write toolchain pins and commands to `evidence/wp05/toolchain-lock.json`.

## 3. Measure assurance

- [ ] Record theorem coverage, assumptions, extraction, and build results.
- [ ] Write objective contributor evidence to `maintainer-evidence.json`.
- [ ] Test drift detection with one deliberate semantic mutation.

## 4. Apply the gate

- [ ] Select one normative strategy and derived-artifact policy.
- [ ] Publish proof and conformance release gates.
- [ ] Select library-only if maintainability fails.
- [ ] Record the decision in `wiki/research-journal.md`.
- [ ] Run `uv run python scripts/validate_sprint_evidence.py --package WP05 --manifest openspec/work-packages.json`.
