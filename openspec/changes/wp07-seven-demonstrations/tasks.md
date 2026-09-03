# Tasks: WP07 seven canonical demonstrations

## 1. Freeze shared interfaces

- [ ] Define the source, Core, manifest, certificate, and verifier contracts.
- [ ] Define common typed party, token, amount, time, and capability values.

## 2. Write tests first

- [ ] Add invariant and timeout tests for all seven slices.
- [ ] Add authorization and artifact-substitution tests for all seven slices.
- [ ] Add outside-assumption assertions for F4, F5, F6, and Prediction.

## 3. Build demonstrations

- [ ] Implement and compile the F1 through F6 slices.
- [ ] Implement and compile the Prediction slice.
- [ ] Preserve artifacts, commands, toolchains, and digests.
- [ ] Write `experiments/moriarty-family-slices/index.json`.

## 4. Evaluate language evidence

- [ ] Measure shared versus application-specific implementation paths.
- [ ] Mark custom-only slices as library evidence.
- [ ] Update `wiki/benchmarks.md` and `wiki/research-journal.md`.

## 5. Apply the gate

- [ ] Require all seven slices to pass.
- [ ] Record `six-family-evidence` if WP06 rejects Prediction.
- [ ] Name every unsupported full-protocol behavior.
- [ ] Run `uv run python scripts/validate_sprint_evidence.py --package WP07 --manifest openspec/work-packages.json`.
