# Midnight Rust experiment results — 2026-09-30

Classification: repository and experiment observations. Local primitive tests pass; full financial/native/Preview acceptance remains open. Independent final reviews are recorded separately in DELIVERY.md when available.

## Measured execution

[Harness and commands](../../experiments/midnight-crypto/README.md). [Final raw test run](evidence/tests-frozen.txt), [fixture export](evidence/export-final.txt), [source receipt](../../raw/sources/midnight-crypto-2026-09-30/receipt.json), [feature resolution](evidence/cargo-features.txt), [preimplementation independent contract](audits/test-contract.md).

| Item | Observed result | Limit |
| --- | --- | --- |
| Rust suite | 12 integration tests pass: signatures 6, CLI 2, authority 1, state 1, proof 1, ledger 1 | Local harness, not whole product acceptance |
| Signatures | Actual Midnight Schnorr BIP340 and ECDSA secp256k1/SHA256 native signing and verification; exact encoding and negative controls | Ephemeral test keys; no wallet interoperability trial |
| Financial fixture byte binding | 5 actual caller fixtures; 2,506 changed-leaf signature refusals across both schemes | One mutation per exported leaf; not a security proof or all financial families |
| Invalid signed candidates | 25 Core rejections; 50 freshly valid signature checks across both schemes | Expiry, consumed replay, effects, work and gross-cap cases; Core observations are exported and pinned |
| Fixture economics | Complete ordered effects and complete post-state independently asserted for fee/zero-fee transfer and interest-only/partial/full AccrualFirst repayment | Existing local S0 slice only |
| Codec/CLI | Node/Rust frame and SHA256 golden vectors match, including astral/BMP keys, Unicode and escapes; actual binary verifies both schemes with/without wallet prefix | Generic experimental JSON subset, not production `/3` financial schema |
| Authority | Absent/wrong/revoked/expired mapping, stale head, used nonce, future/inverted window and attacker key refuse local eligibility despite valid signature | Local stipulated Schnorr authority map, no authenticated ledger registry |
| State | Official Rust Merkle path/root, leaf/sibling/direction mutations, state update, untouched leaf and serialization roundtrip | Locally selected root; no authenticated ledger head |
| Native proof | Real 1,244-byte PLONK/KZG proof verifies through final pairing; changed public inputs, proof bytes/truncation and invalid-witness proof reject | k=6, ephemeral unsafe test SRS, one field equality, no ranges/financial correspondence/PCD |
| Ledger | Official in-memory ledger accepts local privileged reserve/reward distribution; replay, mixed-batch atomic error, reward overdraw and reserve overdraw reject without changing original state | System transactions, not compiled Moriarty user financial settlement; no Preview/finality |

Machine-readable leaf locations and independent effect/post expectations are in the [exported fixtures](../../experiments/midnight-crypto/fixtures/moriarty.json). [Selected upstream source files and licenses](../../raw/sources/midnight-crypto-2026-09-30/collection.json), lockfile and result bytes are committed by the candidate manifest. Keys and private witnesses are not archived.

## Failure evidence and version boundary

Original failed runs remain in evidence/. The positive codec first failed before implementation. The exporter initially used incorrect expected rejection-code names, then used inspected actual Core codes. Upstream API shape failures involved Merkle leaf representation and PLONK Constraints selectors. The SRS test exposed exact-size automatic keygen; explicit k=6 keygen fixed it. The invalid-witness prover hit an upstream debug assertion; the test profile now uses upstream release assertion behavior to observe real verifier rejection. These failures are retained, not described as successful executions.

Ledger9f9842eb and ZK0ededef0 are separate official source pins. Ledger-family crates use registry proofs0.7.3/curves0.2.1; the newest ZK smoke uses git proofs0.8.0/curves0.3.0. Cargo.lock and feature resolution preserve both. No released-stack or Preview compatibility conclusion follows. The connector prefix/encoding conventions were inspected in the [official specification](https://github.com/midnightntwrk/midnight-dapp-connector-api/blob/main/docs/api/_media/SPECIFICATION.md); a live wallet test was not performed.

Builds used two Cargo jobs and external artifacts. Measured target directory: 1.8GiB; free disk: 25GiB at the result capture, within the 8GiB/10GiB resource limits. No broad upstream workspace tests, production SRS download, recursive campaign, real wallet use or network transaction occurred.

## Remaining-item checklist

- [x] Pull pinned official cryptography sources and install/use required Rust dependencies.
- [x] Test both native signature schemes on actual Moriarty fixture bytes and adversarial encodings.
- [x] Keep valid signatures separate from key authority, freshness, replay and financial validity.
- [x] Test official local Merkle state, real bounded proof verification and local ledger failure semantics.
- [ ] Required fresh Astra and exact Grok4.6 high frozen-candidate reviews; completion receipt is separate.
- [ ] Production `moriarty-intent/3` schema/codec, deployment-compatible verifier and complete key/algorithm commitments.
- [ ] Authenticated owner/key registry and key lifecycle; current authority map is stipulated.
- [ ] Snapshot-to-head provenance and authenticated predecessor/successor relationship.
- [ ] Native signature/financial circuit correspondence, UInt128 ranges, mandatory properties/intent/transition/history proofs and recursive PCD.
- [ ] Actual Moriarty financial ledger caller with atomic head/replay/work/allowance compare-and-consume.
- [ ] Preview financial settlement/finality and complete ACTUS/eight-area DeFi conformance.

The production beta still returns PreparedUnqualified. Its four required premises and four unverified bindings remain unchanged. Passing these experiments does not remove them.
