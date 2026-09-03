# Tasks: WP10 real proof and cost

## 1. Acquire authorized inputs

- [ ] Obtain supported proving parameters and record their provenance.
- [ ] Pin compiler, Runtime, wallet, network, and protocol versions.
- [ ] Define hardware and wallet-state profiles.
- [ ] Select and pin the eligible audited-library baseline.
- [ ] Write `evidence/wp10/library-baseline-lock.json`.

## 2. Define budgets before measurement

- [ ] Set percentile gates for latency, memory, proof size, state, transaction, and fees.
- [ ] Set the language-versus-library advantage threshold.
- [ ] Freeze sample counts and exclusion rules.
- [ ] Write and approve `evidence/wp10/measurement-protocol.json`.
- [ ] Record its digest in `wiki/research-journal.md` before results.

## 3. Run measurements

- [ ] Generate keys and proofs for every canonical application.
- [ ] Verify every proof independently.
- [ ] Create `evidence/wp10/testnet-authorization.json` before any submission.
- [ ] Submit testnet transactions only under the named authority.
- [ ] Run the same workloads with audited Compact libraries.

## 4. Reconcile evidence

- [ ] Preserve raw outputs and sample manifests.
- [ ] Publish distributions and failed controls.
- [ ] Update `wiki/benchmarks.md` and `wiki/research-journal.md`.

## 5. Apply the gate

- [ ] Reject mock-only proof claims.
- [ ] Select reduction, library-only, or stop after a failed budget.
- [ ] Run `uv run python scripts/validate_sprint_evidence.py --package WP10 --manifest openspec/work-packages.json`.
