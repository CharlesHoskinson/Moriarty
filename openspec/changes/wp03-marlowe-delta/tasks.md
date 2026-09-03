# Tasks: WP03 Marlowe semantic delta

## 1. Freeze sources

- [x] Verify Marlowe repository locks.
- [x] Preserve the V1 documentation crawl receipts.
- [ ] Map every cited definition to a full commit and source line.

## 2. Build the matrix

- [ ] Write `evidence/wp03/v1-construct-inventory.json`.
- [ ] Write `evidence/wp03/marlowe-moriarty-delta.csv`.
- [ ] Add every construct, warning, error, and serialization boundary.
- [ ] Add theorem premises and implementation correspondence.
- [ ] Add Cardano and Moriarty realization impacts.
- [ ] Require a complete join from the delta to the V1 inventory.

## 3. Reproduce deltas

- [ ] Add a failing test for each newly discovered behavior difference.
- [ ] Run representative semantics traces across maintained implementations.
- [ ] Preserve raw outputs and deterministic seeds.
- [ ] Write `toolchain-lock.json`, `trace-vectors.json`, and `correspondence-results.json`.

## 4. Apply the gate

- [ ] Update `wiki/marlowe-baseline.md` and `wiki/formal-assurance.md`.
- [ ] Mark unsupported inheritance as new proof work.
- [ ] Block compatibility claims with unresolved correspondence gaps.
- [ ] Exclude rows without reproducible definitions from WP04.
- [ ] Run `uv run python scripts/validate_sprint_evidence.py --package WP03 --manifest openspec/work-packages.json`.
