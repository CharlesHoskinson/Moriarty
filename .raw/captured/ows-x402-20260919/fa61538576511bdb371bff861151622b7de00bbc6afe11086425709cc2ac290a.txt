# Agent Access Layer

> Optional access profiles for exposing OWS functionality to applications, agents, CLIs, and local services.

This document is part of the OWS architecture, but it is not a package-level API reference. Concrete SDK names, installer commands, transport details, and language-specific function signatures are non-normative and belong in reference implementation documentation.

## Purpose

The OWS core specification defines stored artifacts, signing semantics, policy evaluation, wallet lifecycle behavior, and chain identifiers. Implementations may expose those capabilities through different local access layers:

- in-process language bindings
- local subprocess execution
- local RPC or daemon interfaces
- CLI wrappers

Those surfaces MAY differ, but they MUST preserve the core OWS semantics.

## Required Access Capabilities

An access layer is conforming only if it preserves the following capabilities from the core spec:

| Capability | Requirement |
|---|---|
| Wallet selection | MUST identify the target wallet unambiguously by ID or implementation-defined stable alias |
| Chain selection | MUST resolve the request to a canonical chain identifier before signing |
| Credential handling | MUST distinguish owner credentials from API tokens without ambiguity |
| Policy enforcement | MUST evaluate applicable policies before any token-backed secret is decrypted |
| Error propagation | MUST surface core errors without rewriting a denial into a success or silent fallback |
| Secret handling | MUST NOT expose decrypted mnemonic or private key material to the caller unless an explicit export operation is invoked |

## Abstract Operations

An implementation MAY choose any surface syntax, but it SHOULD expose operations equivalent to the following abstract methods:

```typescript
createWallet(request) -> WalletDescriptor
importWallet(request) -> WalletDescriptor
listWallets(request?) -> WalletDescriptor[]
getWallet(request) -> WalletDescriptor
deleteWallet(request) -> DeleteResult
sign(request) -> SignResult
signAndSend(request) -> SignAndSendResult
signMessage(request) -> SignMessageResult
signTypedData(request) -> SignMessageResult
createPolicy(request) -> PolicyDescriptor
listPolicies(request?) -> PolicyDescriptor[]
getPolicy(request) -> PolicyDescriptor
deletePolicy(request) -> DeleteResult
createApiKey(request) -> ApiKeyCreationResult
listApiKeys(request?) -> ApiKeyDescriptor[]
revokeApiKey(request) -> DeleteResult
```

The abstract operations above define capability coverage only. They do not require any specific package name, function name, argument order, or transport encoding.

## Credential Semantics

Access layers MUST preserve the credential semantics defined by the policy engine:

- **Owner credential**: unlocks the wallet directly and bypasses policy checks.
- **API token**: resolves to an API key, enforces the token's wallet scope, and evaluates all attached policies before decryption.

If a surface accepts a single generic credential field, it MUST apply deterministic credential-type detection and MUST NOT guess in a way that could weaken policy enforcement.

## Access Profiles

### Profile A: In-Process Binding

The caller links directly against an OWS implementation in the same process.

Requirements:

- MUST preserve all core signing and policy semantics.
- MUST zeroize decrypted secret material as soon as the operation completes.
- SHOULD document that the application and signer share an address space.

### Profile B: Local Subprocess

The caller spawns an OWS child process per operation or per session.

Requirements:

- MUST provide authenticated request input to the child process.
- MUST ensure that policy evaluation happens before token-backed secrets are decrypted.
- SHOULD use structured request and response payloads.

### Profile C: Local Service

The caller communicates with a loopback-only daemon or local RPC endpoint.

Requirements:

- MUST bind only to local interfaces unless a stronger trust boundary is explicitly documented.
- MUST authenticate callers or operating-system principals before performing owner or token-backed actions.
- MUST map remote method names back to the core OWS operations without changing their semantics.

## Cross-Layer Consistency

If an implementation offers multiple access layers, all of them MUST agree on:

- wallet and API key lookup behavior
- policy evaluation order
- canonical error codes
- chain identifier normalization
- audit-log side effects

An implementation MUST NOT make the CLI stricter or weaker than the SDK for the same operation unless the difference is explicitly documented as a surface-specific validation rule.

## Non-Normative Examples

The following are explicitly outside the scope of this document:

- package names such as npm or PyPI artifacts
- shell installer commands
- generated client stubs
- framework-specific wrappers

Those details may be documented in `sdk-cli.md`, `sdk-node.md`, and `sdk-python.md`, but they do not define the standard.
