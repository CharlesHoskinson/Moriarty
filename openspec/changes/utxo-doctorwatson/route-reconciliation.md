# Current Rust and fixed-verifier route reconciliation, proposal v1

Status: additive ordinary interface proposal; not adopted product acceptance. Original EUC-001/012/013 and their scenarios remain intact and open. Design authority comes from the user's instruction to build the UTXO adaptation; substantive security contracts retain their designated owner.

The current selected Cardano experiment consumes Rust `moriarty/1` / `moriarty-core/1` relations. The generated-Aiken clauses require a different artifact path. A local fixed C2 verifier observation therefore cannot be substituted for a generated spending validator. The following proposed clauses identify the ordinary consumer work required on the selected route. They do not dispose of the legacy route.

### RCU-001 Versioned artifact publication

WHEN the selected Rust relation compiler publishes an artifact, it SHALL publish an isolated complete manifest binding exact source, semantics, compiler, relation, statement interface and dependency identities; IF any publication step fails, THEN it SHALL return the original diagnostic and publish no success manifest.

Positive: two structurally contrasting supported programs produce complete isolated manifests through the same real public consumer. Negative: stale dependencies, unsupported staged profiles and partial publication yield explicit refusal without a successful artifact.

Lineage: EUC-001/002/012/016; EU-CON-001/003/018. Source preparation may proceed; the current compiler's synthesis calls are not ordinary reporting checks.

### RCU-002 Artifact consumption

WHEN a Cardano artifact builder consumes a Rust relation artifact, it SHALL consume that exact published manifest and preserve source/statement/consumer identities; IF a required identity or artifact is missing or differs, THEN it SHALL refuse before declaring a usable transaction artifact.

Positive: a real builder consumes the isolated output, with an independent expected artifact envelope. Negative: an edited dependency, wrong statement encoding or incomplete manifest is refused at the actual consumer. A renderer comparison is child evidence only.

Lineage: EUC-012/013/014; EU-CON-001/002/003. The applicable owner defines substantive relation/VK/verifier correspondence; this ordinary interface clause does not establish it.

### RCU-003 Complete boundary observations

WHEN source and Cardano boundary behavior are compared, the comparison SHALL retain the complete independently expected state, ordered effects, identifiers, duties, work and original refusal object; IF a required field or observation is absent, THEN correspondence SHALL remain unestablished.

Positive: complete invoice and contrasting reserve envelopes agree through their genuinely supported actions. Negative: differing effects, duties, identifiers, retained fields or failure diagnostics are reported with the first differing path. The original debt-specific F03 and generated-spend scenarios remain separate.

Lineage: EUC-006/008/013/014; EU-FIN-001/003/005/024; EU-CON-002/003. Independent semantic and security owners determine adequacy; no host-computed flag supplies a missing constraint.

### RCU-004 Explicit profile selection

WHEN the caller selects a source, semantics or target profile, the actual consumer SHALL preserve that selection in its manifest; IF the selected profile lacks a supported lowering, THEN it SHALL refuse explicitly rather than silently selecting another profile.

Positive: the supported Core/1 selector remains bound through its actual artifact consumer. Negative: staged Core/2, legacy /5-/6 and wrong-target inputs cannot inherit Core/1 or target acceptance.

Lineage: EUC-001/016; CS01/CS10; EU-CON-018. Preview, Preprod and Midnight remain distinct, separately qualified targets.

Claim issuance, consumption, setup trust, authenticated context, intention authorization, composition and settlement retain the existing designated-owner contracts. The audit must name their exact missing predicates and evidence; these four ordinary route proposals do not replace them.
