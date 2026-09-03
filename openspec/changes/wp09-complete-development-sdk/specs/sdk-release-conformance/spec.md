# SDK release-and-conformance specification

## ADDED Requirements

### Requirement: explicit version matrix

Each SDK release SHALL publish supported source, Core, serializer, manifest,
certificate, Compact, ZKIR, ledger, Runtime, wallet, and API versions.

#### Scenario: an unknown critical version appears

- WHEN compatibility is not declared
- THEN the SDK fails closed and does not infer latest-version support.

### Requirement: conformance suite

The release SHALL include parser, formatter, type, elaboration, semantics,
serialization, backend, planner, verifier, adapter, migration, and cost vectors.

#### Scenario: one maintained implementation diverges

- WHEN it fails a normative vector
- THEN the release gate fails for that implementation.

### Requirement: security and privacy release gates

The release SHALL have no unresolved critical or high audit finding. It SHALL
test secret redaction, dependency integrity, malicious plans, and disclosure.

#### Scenario: telemetry contains a private value

- WHEN the privacy test detects the value
- THEN release fails and the captured data is handled as an incident.

### Requirement: compatibility and deprecation

The SDK SHALL publish directional compatibility, migration tools, support
windows, deprecation notices, and rollback procedures.

#### Scenario: a breaking serializer change is proposed

- WHEN no migration and equivalence evidence exists
- THEN the release cannot claim backward compatibility.

### Requirement: observability boundary

Observability SHALL use explicit consent, data minimization, stable event
schemas, redaction, retention, and local disable controls.

#### Scenario: consent is absent

- WHEN telemetry is optional
- THEN no telemetry leaves the client.

### Requirement: incident response

The SDK SHALL define vulnerability intake, severity, embargo, artifact
revocation, user notification, recovery, and post-incident evidence updates.

#### Scenario: a compiler release is compromised

- WHEN compromise is confirmed
- THEN affected artifact identities are revoked
- AND clients reject them under the published policy.
