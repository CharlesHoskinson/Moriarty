# Tasks: WP06 high-risk composition

## 1. Specify before implementation

- [ ] Freeze the accepted WP04 motion inputs.
- [ ] Define the conservation and authorization equations.
- [ ] Define token-policy and attestation capability manifests.

## 2. Test adversarial paths

- [ ] Add failing tests for incomplete merge and counterfeit claims.
- [ ] Add failing tests for stale, replayed, and equivocated attestations.
- [ ] Add deadline, partial-submission, refund, and substitution tests.

## 3. Implement the slice

- [ ] Implement the reference transition machine.
- [ ] Generate readable Compact and canonical artifacts.
- [ ] Run differential and deterministic-build checks.

## 4. Apply the gate

- [ ] Preserve all counterexamples and raw compiler outputs.
- [ ] Update the semantic scope ledger with the disposition.
- [ ] Route every scope change through a new WP04 motion record.
- [ ] Record previous and new scope digests in `wiki/research-journal.md`.
- [ ] Extend the WP01 certificate validator before WP07.
- [ ] Reject Core inclusion if any stop condition fires.
- [ ] Run `uv run python scripts/validate_sprint_evidence.py --package WP06 --manifest openspec/work-packages.json`.
