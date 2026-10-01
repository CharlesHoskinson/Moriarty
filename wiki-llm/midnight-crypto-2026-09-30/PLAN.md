# Midnight Rust cryptography implementation plan

> Apply executing-plans and test-driven-development to this authorized plan. User AFK execution authority persists.

Goal: executable Rust verification and adversarial tests grounded in actual Moriarty fixtures and pinned official Midnight implementations.
Architecture: experiments/midnight-crypto owns codec/verify CLI and fixture tests; a separate optional native proof module uses the pinned ZK API. Existing authoring/core code stays the comparison oracle. Findings and gates are recorded in wiki-llm/midnight-crypto-2026-09-30.

- [x] Inspect original beta contract and required premises; obtain independent expected test contract.
- [x] Pull full pinned ledger9f9842eb and zk0ededef repositories, preserving older local checkouts.
- [x] Write real cross-language fixture exporter and independently expected economics.
- [x] Write signature/codec tests, observe missing behavior failure, implement canonical framing and exact official Rust verify APIs.
- [x] Test every bound leaf, wrong keys/signatures, authority/expiry/replay and freshly signed invalid candidates.
- [x] Exercise official Merkle state/witness update and serialization rejection controls.
- [x] Review current native proof API/resource interface; run a smallest real proof with negative controls within design limits.
- [x] Inspect/test actual ledger-local acceptance APIs where supported; record precise integration blockers.
- [ ] Freeze bytes and run both required fresh independent code/result reviews; repair substantive findings.
- [ ] Save measured results and updated remaining-item checklist; publish reviewed work with exact scope.
