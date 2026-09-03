# Change: WP09 complete development SDK

## Why

A safe language is unusable without a complete development system. A thin
TypeScript wrapper would leave source intent, generated circuits, Runtime plans,
wallet requests, and indexed state disconnected.

## What Changes

- Specify the authoring, type, elaboration, and analysis toolchain.
- Specify canonical builds, packages, manifests, certificates, and provenance.
- Specify simulation, debugging, testing, and semantic-difference tools.
- Specify transaction planning and local verification before signing.
- Specify wallet, custody, oracle, identity, registry, chain, indexer, and explorer adapters.
- Specify API versioning, conformance, release, support, and incident behavior.
- Implement the minimum safety spine used by WP10 and WP11.

## Capabilities

### New Capabilities

- `sdk-language-toolchain`: Define authoring, checking, elaboration, code generation,
  diagnostics, LSP, and CLI contracts.
- `sdk-assurance-tools`: Define simulation, analysis, debugging, property,
  differential, fuzzing, equivalence, and cost tools.
- `sdk-package-build`: Define packages, locks, canonical serialization,
  reproducible builds, signing, manifests, certificates, and SBOMs.
- `sdk-planning-verification`: Define intents, transaction planning, state checks,
  disclosure checks, and malicious-plan rejection.
- `sdk-integrations`: Define wallet, custody, Runtime, oracle, identity, registry,
  continuation, chain, indexer, event, and explorer interfaces.
- `sdk-release-conformance`: Define versioning, compatibility, telemetry, test
  vectors, release gates, support, and incident response.

### Modified Capabilities

None.

## Impact

This package specifies all 65 SDK components and 28 shared wire contracts. It
implements only the minimum safety spine required by WP10 and WP11. Other
components remain `specified-only` until their owning sprint implements them.
This package does not release the SDK.
