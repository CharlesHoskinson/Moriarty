# Conformance tasks

All tasks are specified-only and unchecked. Commands below require future implementation.

## CON-T001 Frozen inputs and drift refusal

- [ ] Freeze positive and negative fixtures for `EU-CON-001`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/freeze-inputs.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/freeze-inputs.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/freeze-inputs.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/freeze-inputs.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.

## CON-T002 Complete layered correspondence

- [ ] Freeze positive and negative fixtures for `EU-CON-002`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/full-envelope.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/full-envelope.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/full-envelope.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/full-envelope.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.

## CON-T003 Exact failure identity

- [ ] Freeze positive and negative fixtures for `EU-CON-003`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/failure-layers.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/failure-layers.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/failure-layers.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/failure-layers.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.

## CON-T004 Twenty-family completeness

- [ ] Freeze positive and negative fixtures for `EU-CON-004`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/inventory.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/inventory.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/inventory.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/inventory.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.

## CON-T005 Independent expected values

- [ ] Freeze positive and negative fixtures for `EU-CON-005`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/oracle-independence.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/oracle-independence.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/oracle-independence.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/oracle-independence.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.

## CON-T006 Deterministic differential replay

- [ ] Freeze positive and negative fixtures for `EU-CON-006`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/differential.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/differential.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/differential.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/differential.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.

## CON-T007 Bounded adversarial generation

- [ ] Freeze positive and negative fixtures for `EU-CON-007`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/properties.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/properties.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/properties.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/properties.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.

## CON-T008 Semantics-preserving metamorphic controls

- [ ] Freeze positive and negative fixtures for `EU-CON-008`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/metamorphic.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/metamorphic.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/metamorphic.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/metamorphic.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.

## CON-T009 Native control discrimination

- [ ] Freeze positive and negative fixtures for `EU-CON-009`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/native-discrimination.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/native-discrimination.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/native-discrimination.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/native-discrimination.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.

## CON-T010 Scoped review gates

- [ ] Freeze positive and negative fixtures for `EU-CON-010`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/review-gates.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/review-gates.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/review-gates.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/review-gates.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.

## CON-T011 Fail-closed certificate and private source

- [ ] Freeze positive and negative fixtures for `EU-CON-011`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/private-refusal.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/private-refusal.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/private-refusal.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/private-refusal.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.

## CON-T012 Measured proof capability seam

- [ ] Freeze positive and negative fixtures for `EU-CON-012`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/proof-seam.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/proof-seam.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/proof-seam.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/proof-seam.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.

## CON-T013 Whole private transition binding

- [ ] Freeze positive and negative fixtures for `EU-CON-013`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/proof-binding.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/proof-binding.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/proof-binding.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/proof-binding.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.

## CON-T014 Proof malformed-input controls

- [ ] Freeze positive and negative fixtures for `EU-CON-014`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/proof-negatives.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/proof-negatives.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/proof-negatives.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/proof-negatives.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.

## CON-T015 Ledger-anchored PCD distinction

- [ ] Freeze positive and negative fixtures for `EU-CON-015`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/pcd-gates.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/pcd-gates.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/pcd-gates.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/pcd-gates.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.

## CON-T016 Feasibility disposition and engineering duty

- [ ] Freeze positive and negative fixtures for `EU-CON-016`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/feasibility.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/feasibility.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/feasibility.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/feasibility.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.

## CON-T017 Assessment and graph provenance

- [ ] Freeze positive and negative fixtures for `EU-CON-017`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/assessment-graph.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/assessment-graph.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/assessment-graph.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/assessment-graph.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.

## CON-T018 Versioned feature amendment

- [ ] Freeze positive and negative fixtures for `EU-CON-018`.
- [ ] Add `experiments/moriarty-eutxo/tests/conformance/amendments.test.mjs` before behavioral implementation.
- [ ] Implement the required check in `experiments/moriarty-eutxo/src/conformance/amendments.mjs`.
- [ ] Write `experiments/moriarty-eutxo/artifacts/conformance/amendments.json` with inputs, full outcomes, and reached failure layers.
- [ ] Run `node --test experiments/moriarty-eutxo/tests/conformance/amendments.test.mjs`.
- [ ] Require exit zero and exact positive and negative outcomes from both named scenarios.
