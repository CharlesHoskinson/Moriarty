# Tasks: WP09 complete development SDK

## 1. Freeze component boundaries

- [x] Create `sdk-component-inventory.json` for all 65 components.
- [x] Create `sdk-data-contracts.json` for shared wire contracts.
- [x] Generate `evidence/wp09/sdk-contract-index.json` from both files.
- [x] Define browser, Node.js, CLI, LSP, and service boundaries.

## 2. Specify the language toolchain

- [x] Specify syntax, parsing, formatting, linting, and diagnostics.
- [x] Specify type, visibility, capability, and bound checking.
- [x] Specify elaboration, normalization, serialization, source maps, and Compact generation.

## 3. Specify assurance tooling

- [x] Specify simulation, debugging, trace, and counterexample formats.
- [x] Specify static, symbolic, equivalence, and cost analysis.
- [x] Specify property, differential, fuzz, mutation, golden, and conformance interfaces.

## 4. Specify packages and builds

- [x] Specify package identity, registry, resolution, locks, and signatures.
- [x] Specify canonical linking, reproducible builds, manifests, certificates, and SBOMs.
- [x] Specify continuation integrity and availability interfaces separately.

## 5. Specify deployment safety

- [x] Specify user intents, state snapshots, plans, and estimates.
- [x] Specify artifact, state, intent, disclosure, capability, and transaction verifiers.
- [x] Specify malicious-plan and partial-signature conformance duties.
- [ ] Implement the minimum safety spine named in `design.md`.
- [ ] Reject an unapproved prover endpoint before witness release.
- [ ] Reject a proof-parameter digest, circuit, or network mismatch.

## 6. Specify integrations

- [x] Specify wallet, hardware, custody, and partial-signing adapters.
- [x] Specify Runtime, oracle, identity, registry, and continuation adapters.
- [x] Specify chain, indexer, event, rollback, payout, and explorer adapters.

## 7. Specify release operations

- [x] Specify the complete version and compatibility matrix contract.
- [x] Specify conformance, security, privacy, cost, and migration gates.
- [x] Specify support, telemetry, deprecation, rollback, and incident policies.

## 8. Apply the gate

- [x] Run `uv run pytest tests/test_openspec_work_packages.py -q`.
- [ ] Write `evidence/wp09/minimum-safety-spine-results.json`.
- [ ] Write `evidence/wp09/malicious-plan-results.json`.
- [ ] Verify every signing path passes local verification.
- [ ] Record missing components as package failure, not future polish.
- [ ] Update `wiki/research-journal.md` with the SDK scope version.
- [ ] Run `uv run python scripts/validate_sprint_evidence.py --package WP09 --manifest openspec/work-packages.json`.
