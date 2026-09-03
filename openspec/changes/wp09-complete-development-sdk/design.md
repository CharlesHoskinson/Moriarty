# Design: WP09 complete development SDK

## Context

Moriarty needs a continuous evidence path from financial intent to a signed
Midnight transaction. Every untrusted boundary requires a local verifier and a
machine-readable artifact.

## Inputs

- WP01 backend feasibility artifacts and certificate limits.
- WP04 frozen semantic-scope digest.
- WP05 normative semantics and conformance strategy.
- WP07 shared language interfaces and bounded demonstrations.
- WP08 reviewed protocol coverage and legacy regressions.
- `sdk-component-inventory.json` and `sdk-data-contracts.json`.

## Outputs

- Complete contracts for 65 SDK components.
- Canonical contracts for 28 shared artifacts and wire messages.
- `evidence/wp09/sdk-contract-index.json`.
- A tested minimum safety spine for WP10 and WP11.
- Malicious-plan and unapproved-prover negative results.

## Dependencies

WP09 depends on WP01, WP04, WP05, WP07, and WP08. The normative dependency
record is `openspec/work-packages.json`.

## Architecture

```text
source and packages
  -> parser and typed checks
  -> elaborator and canonical Core
  -> analyzer, simulator, and resource certificate
  -> Compact generator and translation evidence
  -> compactc and ZKIR artifacts
  -> intent and transaction planner
  -> local artifact, state, intent, disclosure, and transaction verifiers
  -> wallet or custody signer
  -> chain, indexer, explorer, and event clients
```

The language server and CLI use the same compiler services. The simulator and
analyzer use the normative Core semantics. Package resolution happens before
canonical Core hashing. Runtime and index providers are replaceable and
untrusted.

The minimum safety spine contains the 17 components marked
`minimum-safety-spine` in `sdk-component-inventory.json`: backend validator;
manifest and certificate generators; intent builder; state verifier; coin
selector; transaction planner; artifact, disclosure, capability, and transaction
verifiers; proof-parameter verifier; prover client; wallet, Runtime, and chain
adapters; and deployment orchestrator.
It makes WP10 measurements and WP11 audits executable. It does not imply that
the complete SDK is implemented.

## Component Contracts

### Authoring and compiler

Define a canonical textual syntax, parser, formatter, linter, type checker,
visibility checker, capability checker, bound checker, elaborator, Core
normalizer, serializer, source-map generator, Compact generator, and diagnostic
schema.

### Assurance tools

Define a reference interpreter, deterministic simulator, time-travel debugger,
trace viewer, symbolic analyzer, counterexample reducer, property runner,
differential runner, fuzzer interface, equivalence checker, semantic-difference
tool, and cost-envelope viewer.

### Packages and builds

Define package identities, semantic versions, dependency locks, canonical
linking, registry resolution, signatures, reproducible builds, source and Core
hashes, capability manifests, translation certificates, deployment manifests,
and software bills of materials.

### Planning and verification

Define typed intents, state snapshots, transaction plans, fee and proof-cost
estimates, continuation requests, partial-signing sessions, and local
verification results. Verification must fail closed before any signing request.
Coin selection is an untrusted planner function. The verifier independently
recomputes value, fees, collateral, network, and change effects.

### Integrations

Define replaceable adapters for browsers, Node.js, wallets, hardware and
enterprise custody, Runtime, oracle providers, identity providers, validator
and script registries, continuation stores, chain providers, indexers, events,
rollbacks, payouts, and explorers.

Define local and remote prover adapters. A remote prover is untrusted. The user
must approve its endpoint and privacy policy. Raw private witnesses never reach
an unapproved endpoint. Proof parameters bind to circuit, network, proof system,
version, digest, source, and revocation state. The client verifies the proof.

### Operations and conformance

Define API versions, capability negotiation, idempotency, retries, error
taxonomies, privacy-preserving telemetry, provenance, compatibility windows,
conformance vectors, cost baselines, release channels, deprecation, and
incident response.

## Trust Boundaries

The compiler, Compact compiler, backend generator, prover, proof-parameter
provider, Runtime, registry, oracle, continuation store, indexer, wallet adapter,
and LLM are not silently trusted. A local verifier checks their bytes and claims
against user-approved intent and pinned artifacts. Private values do not enter
telemetry, diagnostics, or unapproved prover requests.

## Versioning

Every artifact identifies source-language, Core, serializer, manifest,
certificate, Compact language, Compact compiler, Runtime, ledger, and ZKIR
versions. Compatibility is explicit and directional. Unknown critical fields
fail closed.

## Failure Handling

All public APIs return typed results. Signing remains unavailable after any
identity, state, intent, disclosure, capability, cost, version, or transaction
mismatch. Retriable provider failures remain distinct from semantic rejection.

## Verification

The SDK specification is complete only when every named component has inputs,
outputs, stable identifiers, typed failures, trust assumptions, version rules,
security duties, conformance vectors, and an owning package boundary.
Run `uv run pytest tests/test_openspec_work_packages.py -q`. After the safety
spine exists, run `uv run python scripts/validate_sprint_evidence.py --package
WP09 --manifest openspec/work-packages.json`.
