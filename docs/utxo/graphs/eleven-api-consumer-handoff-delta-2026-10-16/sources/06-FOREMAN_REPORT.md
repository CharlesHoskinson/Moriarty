# Rust ProofExport to C2 artifact consumer: source facts

Collected exactly 15 small public source bodies or bounded excerpts. Every item
records its original full-file SHA-256 and exact inclusive line ranges. Capsule
members were selected from the tracked exact prefix and matched against its
copy manifest and output allowlist. No original archival paths were followed.

The existing K10 R2 builder reads `b9-applied.json` and `k8_fixture.ak`, then
writes script.hex, positive/mutation context UPLC files and manifest.json.
It does not consume Rust ProofExport JSON. Fixture VK/proof/statement/digest
literals match the named public Rust records by text comparison only.

`cardano.envV2` exports eight fields and no expiry. The builder supplies the
2030 expiry separately and hard-codes version/kind/family/predicate, claimant55
and outref22/index0; only VK and statement digests enter env_data as arguments.
The a2 settle context is fabricated. None is current Preview wallet or real T1
state. Historical R2 wording mentioning expiry does not establish proof binding.

The saved positive manifest records EnvV2 CBOR and token
`9fd5c53f6a1a03d20b0409f2ca3ad28bd02d5401564603fab5f81b289cc6d54e`.
The separate saved compiled-C2 receipt embeds exactly that positive context
and reports exit0/ACCEPT_UNIT. This task performed no native run. The expectation
source supplies frozen CBOR segments but explicitly does not assert an
independently computed token hex. Historical generated literals are available
for exact input reproduction; independent complete CBOR/token crypto-oracle
status is not established. No replacement oracle was computed.

The candidate wiring boundary is the real builder main at lines458-462 and
487-492, before its context file reaches the existing operator. This is a source
location for a later Opus contract/test author, not a new API or implementation.
The actual eleven-API candidate was neither read nor imported nor changed.

No tests, implementation imports, proof/key/signature/VK validation, native,
network, private, checkout/git edits or nested agents occurred. Preview first
and later Preprod acceptance remain distinct; external runner/byte parser,
circuit/VK correspondence, full20 UTxO, financial/ACTUS/DeFi and PCD obligations
remain open. See ARTIFACT-HANDOFF-FACTS.json for exact fields/paths and limits.
