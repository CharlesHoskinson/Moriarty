# Tasks: WP08 72-row protocol coverage

## 1. Define the schema

- [x] Pin the 72-row roster and its SHA-256 in `design.md`.
- [ ] Define closed dispositions, capabilities, and review states.
- [ ] Add validation for uniqueness and required evidence fields.

## 2. Classify every row

- [ ] Link each row to a WP07 canonical pattern where applicable.
- [ ] Specify one bounded instance or outside-kernel manifest per row.
- [ ] Name every unsupported obligation and trust boundary.

## 3. Review coverage

- [ ] Assign an independent reviewer to every row.
- [ ] Resolve or preserve every disagreement.
- [ ] Run all thirteen legacy regression patterns.
- [x] Name MR-01 through MR-13 in `legacy-regressions.json`.

## 4. Apply the gate

- [ ] Verify exactly 72 unique reviewed rows.
- [ ] Report obligation and capability coverage distributions.
- [ ] Update the wiki and semantic scope journal.
- [ ] Run `uv run python scripts/validate_sprint_evidence.py --package WP08 --manifest openspec/work-packages.json`.
