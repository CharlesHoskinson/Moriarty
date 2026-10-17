# M-HASH independent ordinary script-hash check: contract (PROPOSED SOURCE, primary-reconciled revision)

Author: Opus 5.5 high, source only. Nothing here has been executed or reviewed. This is not a vote, an approval or an adoption. The revision replaces the initial proposal (`MHASH-CONTRACT.md` 7359 B `fedf74ff…`, `MHASH-API.json` 14378 B `c8d5bc86…`). That proposal is kept unchanged as history. The revision changes only source grounding: C3, the C1/C2/C6 citations and the API `sourceDerivation`/`missingPrimarySymbols`/`limits`. The semantics, algorithm, API, v2/v3 scope, bounds, the 19 test IDs, the frozen expected values, the stop conditions, purity and the non-claims are unchanged. The test source `test_independent_script_hash.py` (28091 B, `a1c00918…`) is unchanged and is not touched by this revision.

Packets. The original packet is `mhash-author-packet/MANIFEST.json` (checkoutHead `2b83a8c3…`). Ledger `d6eec237…` and CIPs `befcdb75…` are pinned in `SOURCES.json`. These are the frozen T00 runtime inputs. The supplements are in `mhash-primary-reconciliation-packet/` (originals in `mhash-primary-supplement/`), and root pinned them in `SUPPLEMENT-SOURCES.json`:
- plutus `3de6f245d086d5fee0e0c3324340193127e16257`: `SUPPLEMENT-Plutus-SerialisedScript.hs` (11405 B, `4405a526…`) and `SUPPLEMENT-Plutus-Common.hs` (4227 B, `5b48f1f5…`).
- cardano-base `5813d21182c91fa00f8e799a787ef1759d469b26`: `SUPPLEMENT-Cardano-Base-Blake2b.hs` (5538 B, `753ac378…`).

The supplements are source-grounding references only. They are **not** T00 runtime expectations. They are repository-HEAD bodies and **not** a claim that the ledger at `d6eec237`, Aiken v1.1.24 or any installed binary or lockfile used these commits.

Citation prefixes: `Lang:` Ledger-Plutus-Language.hs, `Hashes:` Ledger-Hashes.hs, `Keys:` Ledger-Keys-Internal.hs, `CIP:` CIP-0057.md, `SS:` SUPPLEMENT-Plutus-SerialisedScript.hs, `PC:` SUPPLEMENT-Plutus-Common.hs, `B2b:` SUPPLEMENT-Cardano-Base-Blake2b.hs. A bare `:N` means `Lang:N`.

Labels: **[S]** marks a source fact at the pinned commit. **[I]** marks an inference that is not shown by any inspected body. **[X]** marks a runtime cross-check (not executed; not proof). The 404 `Ledger-Core-Script.hs` acquisition is kept as a failed receipt and is **not** a source.

**C1. Identifier and algorithm.**
- [S] A Plutus script identifier is `ScriptHash (Hash.Hash ADDRHASH EraIndependentScript)` (Hashes:181-182). `ADDRHASH` is imported from Keys (Hashes:105), and `type ADDRHASH = Hash.Blake2b_224` with `Hash` = `Cardano.Crypto.Hash` (Keys:37, 131-132).
- [S] In cardano-base, `HashSize Blake2b_224 = 28` and `digest _ = blake2b_libsodium 28` (B2b:53, 56-59). `blake2b_libsodium size input` calls `c_crypto_generichash_blake2b` with output length `size` and with no key: `nullPtr`, `0 -- we used unkeyed hash` (B2b:148-162).
- [S] CIP-57 `hash` is "A blake2b-224 hash digest of the validator script, as found in addresses" (CIP:102).
- Output is exactly 28 bytes, rendered as 56 lowercase hex characters. Blake2b-256 (`HASH`, Hashes:125; B2b:61-64), truncation and any other width are wrong.
- [I] Three links are not shown by any inspected body:
  - `Hash.Blake2b_224` in the ledger resolves to the B2b type (the `Cardano.Crypto.Hash` re-export was not inspected).
  - libsodium with outlen 28 is native BLAKE2b-224. Digest length is part of the parameter block, so it is not truncated BLAKE2b-256. The libsodium body and the `Cardano.Crypto.Libsodium.C` binding were not inspected.
  - That primitive equals CPython `hashlib.blake2b(digest_size=28)` with no key, salt or person. This equivalence is neither formalised nor proven here.

**C2. Hash input and language tag.**
- [S] `hashPlutusScript` hashes `BS.singleton (plutusLanguageTag plutusScript) <> originalBytes plutusScript` with `hashWith id` (:199-205). `plutusLanguageTag` is the class method "Tag that will be used as a prefix to compute the `ScriptHash`" (:471-472).
- [S] `originalBytes` of the script is the raw `PlutusBinary` ShortByteString (:185, :251, :261-262).
- [S] Tags are PlutusV2 = `0x02` (:545) and PlutusV3 = `0x03` (:565). They are **not** the CBOR `encodeEnum` constructor index (`PlutusV1`=0 …, :273-278, :329-330), which is used only in the serialised envelope.
- [I] `hashWith id` applies `digest` to the concatenated bytes unchanged. `castHash` changes only the phantom index. `Cardano.Crypto.Hash.Class` was not inspected.
- [S] PC:36-39 exports a plutus-side `Hash.hashScript`/`plutusVersionTag`. Its body (`PlutusLedgerApi.Common.Hash`) was not acquired, and it is not relied on.
- Version scope (tool scope): CIP-57 `preamble.plutusVersion` strings `"v2"` (CIP vector, CIP:382) and `"v3"` (B9 preamble). `"v1"` (0x01, :525) and `"v4"` (0x04, :586) are source-known but have no vector in the packet, so they are rejected.
- Native scripts are out of scope. Their hash rule (`Cardano.Ledger.Core.Script` / `hashScript`, referenced at :199-200) was not acquired (404).

**C3. CBOR boundary (now grounded in primary source; dependency version unproven).** Rule (unchanged): the checker hashes `tag || bytes.fromhex(compiledCode)` exactly as given. It does not strip the outer CBOR byte-string header, does not add another CBOR wrap, and does not hash the `encodeListLen 2 <> encodeEnum lang <> encCBOR binaryScript` envelope (:227-229).

Derivation:
1. [S] The ledger hashes `tag || bs`, where `bs` is the `PlutusBinary` ShortByteString (:205, :262).
2. [S] `decodePlutusRunnable` hands that same `bs` to `PV2.deserialiseScript` / `PV3.deserialiseScript` (:546-547, :566-567), next to `hashPlutusScript p`. So the hashed bytes are exactly the bytes Plutus deserialises as the script.
3. [S] In plutus, `type SerialisedScript = ShortByteString` (SS:124). `serialiseUPLC = toShort . BSL.toStrict . serialise . SerialiseViaFlat . UPLC.UnrestrictedProgram` (SS:155-160): the `Codec.Serialise` CBOR encoding of a Flat-serialised program (Note, SS:126-137). `uncheckedDeserialiseUPLC` is its inverse (SS:164-166).
4. [S] `deserialiseScript` decodes `fromShort sScript` from offset 0 with `CBOR.deserialiseFromBytes (scriptCBORDecoder ll pv)` (SS:265-269). That decoder runs `decodeViaFlatWith` (SS:233-236). The function returns `UnsafeScriptForEvaluation sScript …` with `sScript` unmodified (SS:263).
5. [S] The ledger's `P.CBORDeserialiseError`/`P.RemainderError` (:117-118, :156-163) are the `ScriptDecodeError` constructors re-exported by PC:12 and defined at SS:55-63.

Therefore [S] no layer is removed or added between the bytes Plutus decodes as one CBOR-encoded Flat script and the bytes the ledger hashes. The `[enum, bytes]` envelope is `encCBOR` for transport. It is not `originalBytes`, and `PlutusBinary` `EncCBOR` is newtype-derived (:253).

[I] CIP-57 `compiledCode`, "The full compiled and cbor-encoded serialized flat script" (CIP:101), denotes exactly these `SerialisedScript` bytes. This identification rests on matching the prose to SS:155-160; the CIP does not name the symbol.

[I] Header form: the outer item is a definite-length, minimal-header CBOR major-2 byte string. This depends on two bodies that were not inspected:
- `Codec.Extras.SerialiseViaFlat` (`instance Serialise (SerialiseViaFlat a)`, `decodeViaFlatWith`);
- `Codec.Serialise.serialise` / cborg `encodeBytes`.

The header form affects only the C4 admissibility domain, never the hash rule. Fixtures corroborate it (CIP `58ad`, seven B9 `59 L16`).

[S] Ledger-vs-tool note: at this plutus commit, a `RemainderError` is raised only when `ll ∉ {PlutusV1, PlutusV2}` (SS:259-261). V1/V2 binaries with trailing bytes therefore deserialise, and the ledger would hash them including the trailing bytes. C4 rejects trailing bytes for every version. That is a **tool-domain restriction**: such inputs are refused, never hashed differently, and it is not a protocol claim.

[X] Cross-checks: the CIP-57 hello_world vector (v2, `compiledCode` `58ad…5d11`, hash `5e1e8fa8…0a02`, CIP:382, 425-426) and the B9 receipts are independently frozen **required cross-checks**. They are not authority to choose or refit the boundary. A passing vector does not prove the source chain, dependency correspondence or primitive equivalence.

[I] Dependency qualification: ledger `d6eec237` may build against a different plutus-ledger-api / cardano-crypto-class version than plutus `3de6f245` / cardano-base `5813d211`. The ledger's cabal/flake pins were not inspected.

**Stop rule (unchanged in force):** if `test_cip57_primary_vector` fails under C2+C3, M-HASH stays OPEN. Do not refit boundary, tag or width, and do not soften M-HASH. Investigate transcription, primitive, producer and version mismatch, then request the first item of `missingPrimarySymbols` in MHASH-API.json. Source-connection status: sufficient for the C3 hash rule only at the pinned HEAD commits. The remaining items are named precisely in the API. If a reviewer judges any listed [I] link to be required for C3 itself (not C4 admissibility), STOP and request that exact symbol instead of relying on the vector.

**C4. Admitted input domain (local tool bound, not a Cardano protocol limit).**
- `compiled_code_hex` must be a `str` of lowercase ASCII `[0-9a-f]`, even length, at most 65536 characters, checked before decoding.
- The decoded bytes must be exactly one minimal definite-length CBOR major-2 byte string with header `0x58 L` (24≤L≤255) or `0x59 L16` (256≤L).
- Total length must be `header+L`, with no trailing or missing bytes, and at most `MAX_COMPILED_CODE_BYTES = 32768` bytes.
  - That is about 2.5× the largest frozen fixture (13018 B).
  - It is unrelated to the package's 14336 B working target, `maxTxSize` or ExUnits.
  - It is stricter than the ledger for V1/V2 trailing bytes (C3).
- Only the `0x58`/`0x59` header forms are corroborated in the packet: the CIP vector `58ad`+173 B, and the seven B9 `59`+L16 headers with L+3 = SIZES `appliedBytes`. The producer-side encoder (`SerialiseViaFlat`/cborg) was not inspected. Every other form is rejected by construction: short form, `0x5a/0x5b`, indefinite, other major types, non-minimal.
- The payload is opaque. It is never flat-decoded, evaluated or executed.

**C5. Blueprint check.**
- `check_blueprint(raw: bytes)` accepts at most `MAX_BLUEPRINT_BYTES = 1048576`. Input must be strict UTF-8 JSON in which duplicate object keys are an error.
- `preamble.plutusVersion` is required and must be in C2 scope. CIP marks it optional (CIP:78), and without it the tag is undetermined.
- `validators` must be a list of 1…`MAX_VALIDATORS = 64` objects. Each object needs:
  - a unique non-empty `str` `title`;
  - a `compiledCode` admitted by C4;
  - a `hash` of 56 lowercase hex;
  - `parameters` that are absent or `[]`. An unapplied template is not a final script (CIP:92).
- **Every** entry is recomputed; there is no per-group shortcut.
- A malformed input raises `ScriptHashInputError`. A well-formed `hash` that differs from the recomputed value raises `ScriptHashMismatchError`. The two are distinct `ValueError` subclasses.
- The function returns `{title: recomputed_hash}`. A consistent swap of code and hash between titles cannot be detected from the blueprint alone. It is detected only by comparison with pinned expected values (C6).

**C6. Vectors and oracle status.**
- The primary vector is the CIP-57 hello_world example. Fixture vectors are all 16 B9 handlers, grouped into 7 scripts (c2 ×3, s2p ×3, vi/vs1/vs2/vs3/vs4 ×2).
- Within a group, `compiledCode` and `hash` are identical, and each group's `hash` equals the captured receipt `hash-*.txt`.
- The 7 receipts are **comparison values** from the same Aiken v1.1.24 run that produced the bytes. They are not independent mathematical or semantic oracles.
- The tests compute no BLAKE2b. Expected values are frozen literals only.

**Shared-primitive limitation (unchanged in force):**
- Every positive vector is Aiken-produced. Aiken's BLAKE2b and CBOR/Flat sources were not inspected. The future checker relies on CPython `hashlib.blake2b(digest_size=28)`.
- Agreement therefore shows CPython/Aiken BLAKE2b-224 agreement on these bytes under the C2/C3 framing.
- `Cardano.Crypto.Hash.Blake2b` is now inspected at cardano-base HEAD (C1). It shows the ledger-side parameterisation: unkeyed, outlen 28, via libsodium. Its equivalence to CPython is [I], not proven, and its version correspondence to ledger `d6eec237` is unproven.
- There is no ledger- or chain-produced vector. A defect common to both BLAKE2b implementations would not be detected.

**C7. Purity and effects.**
- The future `independent_script_hash.py` is a standalone, deterministic module. It imports only `hashlib`, `json`, `re` (plus `typing`/`__future__`).
- It performs no file, network, subprocess or environment access, and does not use `eval`/`exec`/`open`/`__import__`.
- It does not import or call Aiken, cardano-cli, Pallas, any SDK, `hash_chain4.py` or hosthash helpers.
- The test loader enforces an AST import allowlist before executing the module. That is a guard, not a sandbox; containment is the envelope in MHASH-API.json.

**C8. Non-claims, authority and stop.**
- This check only recomputes identifiers from frozen bytes. It does **not** validate:
  - UPLC/flat well-formedness or behaviour, or parameter-application semantics;
  - proofs/signatures/VK/matrix correspondence;
  - deployability, transaction size/fees/ExUnits, registration or ledger acceptance;
  - that any installed binary, lockfile or ledger build uses the supplemental commits.
- All 20 UTxO families, financial/ACTUS/DeFi obligations and PCD/recursion remain open, as do X1/PX04/M-EU, FX1, FV-01 and FV-02. Native authority remains ZERO.
- RED, GREEN and meaningfulness outcomes in MHASH-API.json are predictions. Source readiness flags remain false.
- Before any RED:
  - root must inspect the full effects;
  - two fresh, agreeing, independent Opus 5.5 design/test-source audits of this revision paired with the unchanged test are required. Authoring does not count as a vote.
- Any consequential new choice needs the existing two substantive agreeing reviews.
- Source audit precedes RED, and later result adoption is a separate decision.
