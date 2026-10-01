# Midnight Rust cryptography experiment design

User authority: pull official Midnight cryptography and use Rust to test signatures and remaining acceptance items. This follows the published authoring beta; existing wallets, state, original tests and release remain unchanged.

## Decision and alternatives

Choose a portable Rust conformance/verifier harness plus fixtures exported through the actual beta/Source6/Core5 path. It uses official Midnight signature/hash/state primitives and a separate smallest real native proof test. Directly replacing the beta with platform binaries would change its distribution contract; a production WASM/in-circuit authorizer needs a settled codec, authenticated authority and compiler correspondence. Those are later integration work. This experiment makes no such promotion.

Use an explicit experimental message domain, sorted canonical JSON subset and byte-length frame. Export the complete Source6 AST, lowered intent/state/effects and prepared candidate, original source digest and selected agreement/action/asset scale. Test both a pure owner-intent statement and a stronger complete candidate statement separately; the latter is a local fixture statement, not required solver-independent intent. Ephemeral test keys remain memory-only. No real wallet/ledger signing or transaction submission.

Signature validity, local key authority, freshness/replay, snapshot membership, native proof predicate and actual ledger acceptance are separate results. A signature over a supplied snapshot authenticates that assertion, not ledger provenance. Beta outputs remain PreparedUnqualified with original premises/bindings. The stronger envelope is not the production moriarty-intent/3 codec.

## Components and acceptance

- Pinned official ledger base-crypto for Schnorr BIP340, ECDSA, SHA256, exact serialization and malformed encoding rejection.
- Canonical JSON excludes numbers (exact quantities remain decimal strings), duplicates, trailing data and excessive depth/size. Canonical object keys sort; arrays retain order; strings use scalar UTF8. Cross-language golden frames/digests and every-leaf mutations establish tested byte binding.
- Source fixtures: fee/zero-fee transfer; repayment below/across accrued and full repayment. Core independently determines complete effects/post. Freshly signed invalid candidates remain CoreRejected.
- Separate local authority map: wrong key/domain/signer/reference, absent/revoked/expired entry, stale head, used nonce and validity boundaries tested. This registry is a fixture assumption.
- Official Rust Merkle tree: valid membership, wrong leaf/sibling/direction/root, update invalidates old path; serialization roundtrip. Locally chosen root is not an authenticated ledger head.
- Native proof: the smallest real pinned PLONK/KZG relation with mutated public input/proof negatives. Disposable unsafe test SRS is explicitly unsuitable for deployment. No MockProver success is reported as a real proof.
- Ledger/atomic acceptance: inspect actual native interfaces and test a disposable in-memory official fixture if available within limits. Full Moriarty financial proving/ledger integration remains open until that actual path is exercised.

## Resource bounds and stop

One bounded local test task; two Cargo build jobs, one native proof at a time, k<=6 for the primitive proof, no recursive campaign, no network submissions, no proving-server deployment, no downloads of large production SRS. Initial envelope <=65536 bytes, canonical output <=65536, depth32/nodes4096; transport <=65536. Build/test allowance60 minutes and <=8GiB new task artifacts, keeping at least10GiB free disk; stop at the first concrete unsupported prerequisite or repeated defect and change the approach before retry. This user-authorized local cryptography experiment does not reopen exhausted R3 proving reservations. Further expensive financial native work needs its own reviewed exact proposal and existing gates.

Independent preimplementation expected outcomes: audits/test-contract.md. Final source/result review requires fresh Astra and exact Grok4.6 high on frozen bytes. Preserve actual identities and all failed runs. Save results/checklist in wiki-llm and publish reviewed work.
