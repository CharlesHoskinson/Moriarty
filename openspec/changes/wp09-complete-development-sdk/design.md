# Design: WP09 complete development SDK

## Context

Moriarty needs a continuous evidence path from financial intent to a signed
Midnight transaction. Every untrusted boundary requires a local verifier and a
machine-readable artifact.

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

### Integrations

Define replaceable adapters for browsers, Node.js, wallets, hardware and
enterprise custody, Runtime, oracle providers, identity providers, validator
and script registries, continuation stores, chain providers, indexers, events,
rollbacks, payouts, and explorers.

### Operations and conformance

Define API versions, capability negotiation, idempotency, retries, error
taxonomies, privacy-preserving telemetry, provenance, compatibility windows,
conformance vectors, cost baselines, release channels, deprecation, and
incident response.

## Trust Boundaries

The compiler, Compact compiler, Runtime, registry, oracle, continuation store,
indexer, wallet adapter, and LLM are not silently trusted. Each supplies bytes
or claims that a local verifier checks against user-approved intent and pinned
artifacts. Private values do not enter telemetry or diagnostics by default.

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
