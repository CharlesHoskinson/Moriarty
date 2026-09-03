# SDK package-and-build specification

## ADDED Requirements

### Requirement: package identity and resolution

Packages SHALL use stable names, semantic versions, content hashes, signed
metadata, declared capabilities, and a complete dependency lock.

#### Scenario: registry bytes differ from the lock

- WHEN a resolved package has a different content hash
- THEN resolution fails before parsing its source.

### Requirement: canonical artifacts

The SDK SHALL define deterministic source bundles, linked Core, canonical Core
serialization, manifests, certificates, deployment manifests, and source maps.

#### Scenario: two clean builders use the same inputs

- WHEN supported environments build the package
- THEN every canonical artifact digest matches.

### Requirement: backend provenance

Build outputs SHALL identify the Compact language, compiler, runtime, ledger,
ZKIR, source package, Core, and generator versions.

#### Scenario: the compiler version is unsupported

- WHEN the version tuple is outside the manifest range
- THEN the build fails before producing a signable deployment manifest.

### Requirement: supply-chain evidence

The SDK SHALL emit an SBOM, package signature status, toolchain lock, build
receipt, and reproducibility result.

#### Scenario: a dependency signature is revoked

- WHEN policy requires a valid signature
- THEN verification fails and identifies the affected dependency.

### Requirement: continuation identity

Continuation packages SHALL use canonical linking and stable content hashes.
Integrity evidence SHALL remain separate from availability evidence.

#### Scenario: a continuation is unavailable

- WHEN its hash is valid but bytes cannot be retrieved
- THEN the SDK reports an availability failure, not an integrity failure.
