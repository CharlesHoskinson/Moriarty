# W-D1/W-D2 recommendation from current primary sources

**Status:** research recommendation; no wire freeze, wallet interoperability result, verifier certificate or ledger acceptance result. W-D1 and W-D2 remain open.

## Verified interface facts

The pinned [Midnight DApp connector specification](https://github.com/midnightntwrk/midnight-dapp-connector-api/blob/612db2b62dbd78c079da62e57e7b8585a204b858/docs/api/_media/SPECIFICATION.md#L371) requires `signData` to prepend `midnight_signed_message:<data_size>:` to the decoded byte payload. It describes Schnorr BIP340 and ECDSA signatures, with different key encodings, and treats an omitted scheme as Schnorr. It does not specify a maximum payload size. Its [encoding options](https://github.com/midnightntwrk/midnight-dapp-connector-api/blob/612db2b62dbd78c079da62e57e7b8585a204b858/docs/api/_media/SPECIFICATION.md#L254) decode hex or base64 before signing.

The [ledger WASM API](https://docs.midnight.network/api-reference/ledger/functions/verifySignature) exposes `verifySignature` for arbitrary bytes. The [pinned implementation](https://github.com/midnightntwrk/midnight-ledger/blob/9f9842ebed66cdff0f54d3fb09efc6a7cd077ed8/onchain-runtime-wasm/src/primitives.rs#L93) deserializes Schnorr keys and signatures. The ledger [Schnorr wrapper](https://github.com/midnightntwrk/midnight-ledger/blob/9f9842ebed66cdff0f54d3fb09efc6a7cd077ed8/base-crypto/src/schnorr.rs#L142) calls k256 verification; pinned k256 0.13.4 [SHA-256 hashes the supplied message](https://github.com/RustCrypto/elliptic-curves/blob/5ac8f5d77f11399ff48d87b0554935f6eddda342/k256/src/schnorr/verifying.rs#L111) before raw BIP340 verification. The [BIP340 specification](https://github.com/bitcoin/bips/blob/3a10b5b5f0a7586df8928d580a3009744ebb2079/bip-0340.mediawiki#messages-of-arbitrary-size) does not itself require that extra application prehash.

These facts establish a host API and a possible preprocessing rule. They do not establish that a current wallet returns a compatible signature or that a consensus contract can enforce this arbitrary-message check as part of Moriarty acceptance.

## Recommended candidate for review

Use one S0 Schnorr profile. Let `C` be the eventual canonical `/3` authorization bytes; let `D = SHA256(C)` be exactly 32 bytes. For a connector `signData` call with hex-encoded `D`, define `M = ASCII("midnight_signed_message:32:") || D`. The **candidate** verification predicate is raw `BIP340.Verify(key32, SHA256(M), signature64)`, equivalent to the inspected k256 host wrapper when called with `M`. Reject explicit ECDSA or unknown schemes in this profile; normalize an omitted scheme to Schnorr only where the connector version requires it.

This candidate needs an actual wallet signature vector and host-verifier result before adoption. The proof public statement and ledger consumer must bind the same `D`, signer key, selected program, predecessor, replay key and complete effect commitment. The proof must constrain `D` as the digest of the exact signed authorization bytes. A caller supplied authentication Boolean is insufficient. If no consensus enforced host hook exists, retain an external premise or choose and certify an in-circuit verifier.

For `/3`, recommend a closed typed binary encoding with a literal domain header, numeric tags, fixed field order, fixed-width UInt128 atomic amounts, length-prefixed identifiers, explicit absent/empty values and canonical ordering of set-valued collections. Preserve ordered effect lines. Reject unknown tags, duplicate or unordered sets, omitted required leaves, trailing bytes and out-of-cap fields. Source/6, Core/5 and signed `/3` identities remain independent version axes; `/4` migration requires a separate authenticated relation. The complete tag table, finite caps and codec are still to be written and reviewed.

## Required experiments before a decision vote

1. Obtain a wallet `signData` signature over the candidate 32-byte `D`. Independently verify against `M`, `SHA256(M)`, `D` and `SHA256(D)` to identify the actual preprocessing. Check decoded 32-byte length against 64 hex characters and reject wrong or duplicated prefixes.
2. Hold the signature fixed while changing one signed recipient byte, fee cap, proof-public digest, signer key, predecessor, replay value or effect commitment. Every substitution must reject with no accepted effects.
3. Run codec round trips and independent noncanonical controls: unknown tag, omitted empty, duplicate collection item, alternate order, trailing bytes, cap+1 and wrong scheme/key length.
4. Demonstrate an acceptance consumer that fails when the host signature check is bypassed, or retain authentication as a named external premise. Pin the exact wallet, ledger and k256 versions used in that experiment.

No experiment in this note has yet run. This recommendation does not close W-D1 or W-D2.

## 2026-09-30 experiment update

The paragraph above records the state when this research note was first written. A separate [provisional `/3` codec experiment](../../experiments/moriarty-language/formal/mil4/wire/RESULT.md) now implements closed S0 tags and executes finite byte tests. Its Python reference builder was added after the first codec audit. Neither the codec nor its tests establish signature, effect-commitment, proof or ledger correspondence. W-D1 and W-D2 remain open.

The codec uses 16-byte unsigned fields but admits S0 nominal values only through `2^127−1`; balances and allowance counters in the semantic S0 prototype use checked UInt128. This is narrower than the full-UInt128 atomic-amount recommendation above. No proof-target feasibility result or final W-D4 decision justifies that difference yet. A normative wire decision must either adopt the narrower nominal rule with its exact source/Core boundary or widen the codec and re-review every bound and hostile vector. The Source/6 settlement scale cap is 18 while the provisional wire scale cap is 38; this also requires an explicit cross-layer rule before closure.

A local [pinned ledger primitive experiment](../../experiments/moriarty-language/formal/mil4/signature/RESULT.md) shows that a synthetic signature over the connector-style prefixed digest verifies on the exact supplied message and rejects several changed messages. It does not use a connector wallet or establish a consensus acceptance hook.
