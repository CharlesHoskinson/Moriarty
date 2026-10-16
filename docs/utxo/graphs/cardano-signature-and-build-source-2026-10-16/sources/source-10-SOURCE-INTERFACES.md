# Locked signature-producer interfaces — source data

The exact `k256 0.13.4` and `ecdsa 0.16.9` distributions expose existing signing and verification methods. This packet locates their public inputs, outputs, outer message-hash steps and randomness branches. It does not recommend a signing procedure or establish correspondence with Aiken, cryptographic soundness, operator approval or ledger acceptance.

Two static Scrapling Fetcher invocations returned HTTP 200, without application retries. Each archive matched the existing public K2 `Cargo.lock` checksum **before** source selection. Archive bodies were handled only in memory. Twelve source/build files plus four applicable license files are retained. [Manifest](SOURCE-MANIFEST.json), [retrieval receipts](RETRIEVAL-RECEIPTS.json) and [read coverage](READ-COVERAGE.json) give exact byte pins and limits; no complete semantic or dependency-closure review is claimed.

| Distribution | Cargo SHA-256 | Archive-declared upstream commit |
| --- | --- | --- |
| [k256 0.13.4](https://static.crates.io/crates/k256/k256-0.13.4.crate), 93,562 bytes | `f6e3919bbaa2945715f0bb6d3934a173d1e9a59ac23767fbaaef277265a7411b` | `5ac8f5d77f11399ff48d87b0554935f6eddda342` |
| [ecdsa 0.16.9](https://static.crates.io/crates/ecdsa/ecdsa-0.16.9.crate), 31,406 bytes | `ee27f32b5c5292967d2d4a9d7f1e0b0aed2c15daded5a60300e4abb9d8020bca` | `abd0175f3ae2dc4b118fa914e94c1fc4b0182ba9` |

The commits come from each checksum-matching archive's `.cargo_vcs_info.json`, not a separate Git-object verification. Package metadata identifies RustCrypto `elliptic-curves/k256` and `signatures/ecdsa`, both `Apache-2.0 OR MIT`. The normalized Cargo manifests and original manifests are copied unchanged. No latest-release or binary-build parity claim is made.

## Schnorr key and byte interfaces

[k256 Schnorr signing.rs](source/k256-0.13.4/src/schnorr/signing.rs), lines 37–69, exposes:

| Method | Source input/output and delegation |
| --- | --- |
| `SigningKey::random(&mut impl CryptoRngCore) -> Self` | Calls `NonZeroScalar::random(rng)`. The caller supplies the RNG; this method does not select OS entropy in the inspected body. |
| `SigningKey::from_bytes(&[u8]) -> Result<Self>` | Parses big-endian scalar bytes through `NonZeroScalar::try_from`, then converts to the Schnorr key; parse failure maps to `signature::Error::new()`. The delegated parser body is not selected. |
| `to_bytes(&self) -> FieldBytes` | Serializes the stored secret scalar. |
| `verifying_key(&self) -> &VerifyingKey` | Returns the associated public key. |
| `as_nonzero_scalar(&self) -> &NonZeroScalar` | Returns a secret-scalar reference; it is not a public-key output. |

The conversion from `NonZeroScalar` (137–156) computes public-point y parity and conditionally negates the secret scalar before storing it. Thus the stored scalar serialization is not source-guaranteed to reproduce the caller's original scalar bytes unchanged. This is a source observation only. No actual key was read or generated.

[VerifyingKey](source/k256-0.13.4/src/schnorr/verifying.rs), 27–85, provides `to_bytes() -> FieldBytes` as the public point's x-coordinate, `from_bytes(&[u8]) -> Result<Self>` via decompaction, and `as_affine()`. Fixed-size byte conversion/decompaction delegates are outside the selected body closure. [schnorr.rs](source/k256-0.13.4/src/schnorr.rs), 81–159, declares a 64-byte `Signature`, serializes `r || s`, and parses the two components using field/scalar helpers. The direct parser uses fixed-width slicing/conversions; this packet does not establish arbitrary-input panic/error behavior for omitted conversion implementations. Existing Core's consumer checks the public key/signature lengths before these parsers, as recorded in the separate owner-intent packet.

## Schnorr message, prehash and auxiliary paths

These are call-path descriptions, not instructions for selecting a path. “Outer hash” below means message preprocessing, distinct from the tagged SHA-256 operations inside the signature calculation.

| Entry point in signing.rs | Actual chain and auxiliary input |
| --- | --- |
| `Signer<Signature>::try_sign(&self, msg: &[u8]) -> Result<Signature>` (225–229) | `Sha256::new_with_prefix(msg)` → `try_sign_digest` → finalized 32-byte digest → `sign_raw`; auxiliary array is `Default::default()` (zero bytes). |
| `DigestSigner<D, Signature>::try_sign_digest(D)` (176–183) | Requires a digest with fixed 32-byte output; finalizes it and passes bytes directly to `sign_raw`, with zero auxiliary bytes. |
| `PrehashSigner<Signature>::sign_prehash(&[u8])` (185–189) | Passes the supplied slice directly to `sign_raw`, with zero auxiliary bytes; this wrapper adds no outer SHA-256 or explicit 32-byte length check. |
| `sign_prehash_with_aux_rand(&[u8; 32], &[u8; 32])` (79–85) | Passes the supplied 32-byte digest and auxiliary array directly to `sign_raw`. |
| `sign_raw(&[u8], &[u8; 32]) -> Result<Signature>` (97–134) | Uses the supplied message bytes directly in nonce/challenge tagged-hash inputs; it does not first replace them with `SHA256(msg)`. It still performs the internal tagged SHA-256 operations. |
| `RandomizedSigner::try_sign_with_rng(rng, msg)` (206–210) | Adds the outer SHA-256 via `Sha256::new_with_prefix`, then the randomized digest path. |
| `RandomizedDigestSigner::try_sign_digest_with_rng(rng, digest)` (191–204) | Fills a 32-byte auxiliary array via `rng.fill_bytes`; finalizes the digest and calls `sign_raw`. |
| `RandomizedPrehashSigner::sign_prehash_with_rng(rng, prehash)` (212–223) | Fills a 32-byte auxiliary array via `rng.fill_bytes`; calls `sign_raw` on the supplied prehash slice. No outer hash here. |

`sign_raw` hashes the auxiliary bytes with `BIP0340/aux`, XORs with secret scalar bytes, uses `BIP0340/nonce` over that result/public x/message, and `BIP0340/challenge` over r/public x/message. It returns opaque signature errors on named scalar conversions. Under `debug_assertions` only, it also invokes the public key's `verify_prehash(msg, &sig)` before returning. [schnorr.rs](source/k256-0.13.4/src/schnorr.rs), 77–79 and 180–186, defines the tags and tagged-hash construction as SHA-256(tag) twice prefixed to a new SHA-256 state. These source operations are not an entropy or security theorem.

The matching [verifying chain](source/k256-0.13.4/src/schnorr/verifying.rs), 48–115, is:

```text
Verifier::verify(msg, sig)
  -> verify_digest(Sha256::new_with_prefix(msg), sig)
  -> verify_prehash(finalized_digest, sig)
  -> verify_raw(prehash, sig)
```

`PrehashVerifier::verify_prehash` forwards the slice without an outer hash. `verify_raw` forms the tagged challenge using supplied message bytes, computes a point expression and rejects identity, odd y or mismatched x. Therefore the message API adds an outer SHA-256 to whatever bytes it receives, including a digest byte slice if one is passed. Raw/prehash wrappers do not add that outer step. The source `Verifier` call used by Core is a message API; convenience `Signer::sign`/trait panic semantics reside in the unacquired `signature` crate, while `try_sign` bodies are selected here.

## ECDSA key, signer and verifier interfaces

[k256 ecdsa.rs](source/k256-0.13.4/src/ecdsa.rs), 162–179, aliases fixed-size `Signature`, DER `DerSignature`, `SigningKey<Secp256k1>` and `VerifyingKey<Secp256k1>` from `ecdsa`, and binds `DigestPrimitive::Digest = sha2::Sha256` under `sha256`.

[ecdsa signing.rs](source/ecdsa-0.16.9/src/signing.rs), 80–125, supplies `random(rng)`, `from_bytes(&FieldBytes<C>)`, `from_slice(&[u8])`, `to_bytes()` and feature-gated `verifying_key()`. Constructors delegate to `elliptic_curve::SecretKey` or `NonZeroScalar`. Those validation/RNG internals are not selected. `Drop` invokes scalar zeroization (380–389); effective memory/OS guarantees are not established here.

| ECDSA entry point | Source chain |
| --- | --- |
| `Signer<Signature<C>>::try_sign(msg)` (171–180) | `C::Digest::new_with_prefix(msg)` → `try_sign_digest` → finalized digest → `sign_prehash`. For this k256 alias the declared digest is SHA-256. |
| `DigestSigner::try_sign_digest(digest)` (136–146) | Finalizes digest, then calls `sign_prehash`. |
| `PrehashSigner::sign_prehash(prehash)` (152–165) | Calls `bits2field`; then `secret_scalar.try_sign_prehashed_rfc6979::<C::Digest>(&z, &[])`; no additional outer message hash. |
| `RandomizedPrehashSigner::sign_prehash_with_rng` (198–217) | `bits2field`, fills field-sized additional-data bytes with the caller RNG, then the same RFC6979 helper with those bytes. |
| Randomized digest/message forms (182–195, 219–229) | Digest form finalizes its digest; message form constructs `C::Digest::new_with_prefix(msg)` before the randomized chain. |

[hazmat.rs](source/ecdsa-0.16.9/src/hazmat.rs), 94–112, calls `rfc6979::generate_k` with secret scalar representation, curve order, digest-field bytes and additional data, then dispatches `try_sign_prehashed`. That RFC6979 implementation body is not acquired. `bits2field` (185–206) rejects slices shorter than half the field width, copies equal-width input, left-pads shorter allowed input and truncates longer input to the field width. This conversion is not an outer SHA-256. The generic signing primitive body (224–259) is selected; its curve/scalar operations remain dependencies.

[k256's SignPrimitive override](source/k256-0.13.4/src/ecdsa.rs), 181–198, calls generic `hazmat::sign_prehashed`, normalizes the result through `normalize_s().unwrap_or(sig)` and adjusts the recovery-id parity. Its VerifyPrimitive override (200–209) rejects high-S before generic `hazmat::verify_prehashed`. [ecdsa lib.rs](source/ecdsa-0.16.9/src/lib.rs), 314–328, defines `normalize_s()` to return `Some` with negated s when `is_high`, otherwise `None`; the scalar `is_high` implementation is not selected.

[ecdsa verifying.rs](source/ecdsa-0.16.9/src/verifying.rs), 91–133, exposes `from_sec1_bytes`, `from_encoded_point`, `from_affine`, `to_encoded_point(compress)` and feature-gated `to_sec1_bytes`. Message `Verifier::verify` (169–178) constructs `C::Digest::new_with_prefix`; digest verification dispatches to the affine point's `verify_digest` (145–155), which finalizes its digest in [hazmat.rs](source/ecdsa-0.16.9/src/hazmat.rs), 139–144, then calls `verify_prehashed`. Prehash verification (157–167) uses `bits2field` and the same primitive without adding an outer message hash.

Fixed-size ECDSA signatures serialize as big-endian `r || s`; the source documents 64 bytes for secp256k1. `from_slice` checks the required total length; `from_scalars` rejects out-of-range/zero components. DER parsing/encoding is a separate API ([lib.rs](source/ecdsa-0.16.9/src/lib.rs), 167–289). These facts do not establish which serialization a future operator has actually produced.

## Feature/version boundaries and missing bodies

[k256 original Cargo](source/k256-0.13.4/Cargo.toml.orig), 20–62, declares ecdsa-core package `ecdsa` range `0.16.8`, elliptic-curve `0.13.8`, sha2 `0.10`, signature `2`. `ecdsa` enables arithmetic, ecdsa-core signing/verifying and sha256; `schnorr` enables arithmetic, sha256 and signature; sha256 enables digest and sha2. Defaults additionally enable pkcs8/precomputed-tables/std. The earlier Core workspace requests k256 `=0.13.4` with `schnorr` and `ecdsa`; this packet has not run Cargo feature resolution.

[ecdsa original Cargo](source/ecdsa-0.16.9/Cargo.toml.orig), 18–48, declares elliptic-curve `0.13.6`, signature `2.0, <2.3`, digest `0.10.7`, optional rfc6979 `0.4`. Signing enables arithmetic/digest/hazmat/rfc6979; verifying enables arithmetic/digest/hazmat. Both packages declare edition 2021, rust-version 1.65 and dual Apache-2.0/MIT licensing; normalized manifests are retained alongside originals.

The existing K2 lock resolves k256 0.13.4, ecdsa 0.16.9, signature 2.2.0, elliptic-curve 0.13.8, sha2 0.10.9, digest 0.10.7, rfc6979 0.4.0 and rand_core 0.6.4. The same lock also has Core-side sha2 0.11.1/digest 0.11.3. Exact lock entries are metadata in the manifest, **not acquired implementation bodies**. Missing implementations include signature trait defaults/errors; SHA-2/digest; RFC6979/HMAC; elliptic-curve secret parsing, random-scalar generation and SEC1; k256 curve arithmetic; and platform RNG/entropy. No source closure theorem, entropy guarantee or Aiken-builtin equivalence follows from these selected methods.

The remaining experiment boundary is the separately reviewed fresh-key private caller and its exact byte/artifact handling, followed by observed consumer behavior. No such caller is authored here. No build, test, Cargo, CLI, curve import, key generation, RNG, signing, verification, proof, ledger query or transaction ran. Only static source acquisition, parsing and byte/hash checks occurred. Public source fixtures in copied files are package source data, not private operator material. No additional authority is created by this packet.
