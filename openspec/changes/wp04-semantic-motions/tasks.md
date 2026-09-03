# Tasks: WP04 semantic motions

## 1. Prepare motions

- [ ] Create `evidence/wp04/motions/index.json` and one record per motion.
- [ ] Attach WP03 source and theorem locators.
- [ ] Attach WP02 demand and residue evidence.

## 2. Specify semantics

- [ ] Write transition rules before implementation.
- [ ] Define typed warnings and hard errors.
- [ ] Define lifetime, state, transition, and witness bounds.
- [ ] Define public and private disclosure requirements.

## 3. Challenge motions

- [ ] Generate counterexamples for ordering and timeout races.
- [ ] Test conservation, authorization, replay, and overflow failures.
- [ ] Compare Core inclusion with surface and library alternatives.

## 4. Apply the gate

- [ ] Record exactly one disposition per motion.
- [ ] Reject ambiguous Core semantics.
- [ ] Update `wiki/research-journal.md` with the semantic version delta.
- [ ] Create a new immutable snapshot and update `evidence/semantic-scope/index.json`.
- [ ] Record previous and new snapshot digests in the journal.
- [ ] Freeze only the accepted motion set.
- [ ] Write `evidence/wp04/model-check-results.json`.
- [ ] Run `uv run python scripts/validate_sprint_evidence.py --package WP04 --manifest openspec/work-packages.json`.
