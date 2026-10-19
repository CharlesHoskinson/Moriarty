### Requirement: EUC-012 Artifact publication and failure isolation
WHEN compilation succeeds, the compiler SHALL publish the generated Aiken, checked blueprint, UPLC bytes, Cardano credential, source-file digest, and dependency manifest from one isolated output directory. IF admission, Aiken check, Aiken build, required tests, or blueprint parsing fails, THEN the compiler SHALL return nonzero and leave no success blueprint. IF artifacts or dependencies drift after verification, THEN consumption SHALL refuse the stale manifest.

#### Scenario: Positive Publish a checked result
- **WHEN** check, build, and required named tests succeed on pinned files
- **THEN** the blueprint references exactly the compiled source manifest

#### Scenario: Negative Refuse stale success
- **WHEN** a prior plutus.json exists and the next build fails
- **THEN** the old blueprint cannot be returned as the new compilation result

