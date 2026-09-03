# Tasks: WP02 evidence and taxonomy

## 1. Freeze inputs

- [x] Verify the pinned DeFiFormal commit.
- [x] Reconcile the 72-row and 60-construction populations.

## 2. Reproduce metrics

- [x] Run `uv run pytest tests/test_defiformal_taxonomy_metrics.py`.
- [x] Run `uv run pytest tests/test_defiformal_taxonomy_crosswalk.py`.
- [ ] Preserve the complete Jaccard and composition matrices.
- [ ] Reproduce every hierarchical result with pinned library versions.

## 3. Classify residue

- [ ] Create the 689-row residue dataset under `evidence/`.
- [ ] Define closed family, facet, and disposition values.
- [ ] Review every orphan and low-confidence row.

## 4. Run genuine rating

- [ ] Publish the rater handbook before collecting ratings.
- [ ] Collect at least two independent human ratings.
- [ ] Calculate agreement before adjudication.

## 5. Apply the gate

- [ ] Update `wiki/defiformal-taxonomy.md` and its claim records.
- [ ] Fail the package for missing rows or simulated-only agreement.
