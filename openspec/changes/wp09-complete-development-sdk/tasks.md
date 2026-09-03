# Tasks: WP09 complete development SDK

## 1. Freeze component boundaries

- [ ] Create a package ownership map for every named SDK component.
- [ ] Define shared identifiers, result types, diagnostics, and version fields.
- [ ] Define browser, Node.js, CLI, LSP, and service boundaries.

## 2. Specify the language toolchain

- [ ] Specify syntax, parsing, formatting, linting, and diagnostics.
- [ ] Specify type, visibility, capability, and bound checking.
- [ ] Specify elaboration, normalization, serialization, source maps, and Compact generation.

## 3. Specify assurance tooling

- [ ] Specify simulation, debugging, trace, and counterexample formats.
- [ ] Specify static, symbolic, equivalence, and cost analysis.
- [ ] Specify property, differential, fuzz, mutation, golden, and conformance interfaces.

## 4. Specify packages and builds

- [ ] Specify package identity, registry, resolution, locks, and signatures.
- [ ] Specify canonical linking, reproducible builds, manifests, certificates, and SBOMs.
- [ ] Specify continuation integrity and availability interfaces separately.

## 5. Specify deployment safety

- [ ] Specify user intents, state snapshots, plans, and estimates.
- [ ] Specify artifact, state, intent, disclosure, capability, and transaction verifiers.
- [ ] Specify malicious-plan and partial-signature test suites.

## 6. Specify integrations

- [ ] Specify wallet, hardware, custody, and partial-signing adapters.
- [ ] Specify Runtime, oracle, identity, registry, and continuation adapters.
- [ ] Specify chain, indexer, event, rollback, payout, and explorer adapters.

## 7. Specify release operations

- [ ] Publish the complete version and compatibility matrix.
- [ ] Publish conformance, security, privacy, cost, and migration gates.
- [ ] Publish support, telemetry, deprecation, rollback, and incident policies.

## 8. Apply the gate

- [ ] Verify every component has inputs, outputs, failures, trust, versions, and tests.
- [ ] Verify every signing path passes local verification.
- [ ] Record missing components as package failure, not future polish.
- [ ] Update `wiki/research-journal.md` with the SDK scope version.
