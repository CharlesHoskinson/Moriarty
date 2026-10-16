# PLUTUS-DATA-SOURCE-CONTRACT

This is a narrow primitive contract for `host_codec.encode_data(data) -> bytes` only. The companion test is `test_plutus_data_serialisation.py`.

Author: claude-opus-5-5, acting as a fresh source-contract author with tools off. Nothing was executed.

R2 targeted security correction: claude-opus-5-5 author, separate context, tools off, nothing executed. It addresses fresh non-author review F01-F11 (same provider; independence is context-only). Byte vectors, CBOR rules and financial scope are unchanged.

This document is an author proposal for a fresh non-author Opus review. It is not an approval and not an implementation audit.

Status of related work:
- `host_codec.py` is the GPT6.1Sol candidate (5740 B, `0865b2ab2cca350ce2e8f56872ba16e5b1002ea63de1462cac2090b85359fd46`). Root ran it against the 18 R1 tests in E1: 18 pass, 0.119 s. That is behavioural only, not a security approval or implementation audit. R2 removes one exception handler (see I2).
- The parent R2 package, including all 75 K4H gold rows, remains untouched and immutable.
- This slice does not complete host_codec, the DTO, ledger-context, intent, transaction or UTxO work.

Labels used below:
- SOURCE: a fact read in a pinned input.
- DESIGN: a host admission decision made by this contract.
- UNVERIFIED: a correspondence not established from the inputs read.

## 1. Pins (copied from the public input index)

- `/home/charl/moriarty-eutxo-validation-2026-10-12/cardano-aiken-serializer-callchain-primary-source-20261016/source/crates/uplc/src/lib.rs` `a0d5944dcb1fac58d8d8275ed46ceedb5e144f42861203000683f37832c00ed6`
- `/home/charl/moriarty-eutxo-validation-2026-10-12/cardano-aiken-serializer-callchain-primary-source-20261016/RESULT.json` `5c981ed7885a605902cf3a6b8679512616a2958d00e671f6c5cf566b9d283ffb`
- `/home/charl/moriarty-eutxo-validation-2026-10-12/cardano-aiken-serializer-dependency-primary-source-20261016/RESULT.json` `c75fde17ca6a347f391622ca33ea3510fceb569b9200dfd6833461803a8c8d21`
- `/home/charl/moriarty-eutxo-validation-2026-10-12/cardano-aiken-serializer-dependency-primary-source-20261016/source/pallas-primitives-0.35.1/src/plutus_data.rs` `adfebe4d9b53be5656bed1fce51e627fac0e3e1f14ee990dfe0af66bd26c924b`
- `/home/charl/moriarty-eutxo-validation-2026-10-12/cardano-aiken-serializer-dependency-primary-source-20261016/source/pallas-codec-0.35.1/src/utils.rs` `7019c76d3ba34edf028e7276a251248bd3bde2b5e759bd4f7847c394c6df798f`
- `/home/charl/moriarty-eutxo-validation-2026-10-12/cardano-aiken-serializer-dependency-primary-source-20261016/source/minicbor-0.25.1/src/encode/encoder.rs` `feb18599fdc8f33f1ba81080d2ee76920ea316f93a75e4b88532edc0d1f30921`
- `/home/charl/moriarty-eutxo-validation-2026-10-12/cardano-data-serialization-source-supplement-20261015/source/plutus/plutus-core/plutus-core/src/PlutusCore/Data.hs` `e4633b7b293b4078387b107eb36d1f16864b93b7a129a7a2531493791fb5628c`
- `/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-fx1-host-codec-expectations-r2-candidate-20261016/independent_expectations.py` `f9f1dcca617c03a07e1c647d204ce681b0e8fd98e10bea3f5380f6e5fffa0b63`
- `/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-fx1-host-codec-expectations-r2-candidate-20261016/IMPLEMENTATION-PLAN.md` `610226cc1c6780913853a8b7d3b515db41d3043d59b6b0c28b7df5cccf61396a`
- `/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-fx1-host-codec-expectations-r3-opus-source-contract-repair-20261016/PROCESS.json` `9f18a3131c7b1ee2973a5a7a9727bc99849b57aa46c2e5e7a2bc6a87d330b0cd`

Aiken commit: `bacbeb35fbc9f1db93cd2e1e93fb1d93053acb4a`.

Cargo.lock sha256: `ce94f9c0212dfe5a3ae8992de21a7fb58f594d04046a1bedc4eedb4dbc11e989`.

Crate archive SHA-256 values below are copied from dependency RESULT.json. That file records `checksumMatchesLockedVersion: true`; this is a metadata claim that was not re-verified here.
- minicbor-0.25.1: `c0452a60c1863c1f50b5f77cd295e8d2786849f35883f0b9e18e7e6e1b5691b0`
- pallas-codec-0.35.1: `afff0c114bc5ff0e5bf95b6b151a664da57c259ab266bcff6587faad91a064d2`
- pallas-primitives-0.35.1: `857f6908501bcb6f794f2c81674eb0f69414e66399739bbd808aa178c17c2001`

SerialiseData call chain (R2, read): runtime.rs `SerialiseData` calls lib.rs `plutus_data_to_bytes`. Supplement pins (paths relative to `/home/charl/moriarty-eutxo-validation-2026-10-12/`):
- `cardano-data-serialization-source-supplement-20261015/MANIFEST.json` `5983515d9592fde3192170ae95a37a5001072b8fed5b7a9b91adbc1692c9f129`
- `cardano-data-serialization-source-supplement-20261015/source/aiken/crates/uplc/src/machine/runtime.rs` `5624cc68cf4c3dedea5a063a2542ea047456a6b9e4d6ceda67c6fc0c17d93f48`
- `cardano-aiken-integer-boundary-primary-source-20261016/RESULT.json` `8bc84192af514afce8a53befa2682a3c216609a0d49a06519a7a63193712a480`
- `cardano-aiken-integer-boundary-primary-source-20261016/source/crates/uplc/src/machine/value.rs` `03d9cae02b77121dd1a7a514e2a763297dfbeb84c173f47eea7e5c2833a8f5fb`
- `cardano-aiken-serializer-dependency-primary-source-20261016/source/minicbor-0.25.1/src/data.rs` `3e7c313dd453df846d71c6a785910482306396f533da53674e31fef8390caa10`
- `cardano-aiken-serializer-dependency-primary-source-20261016/source/minicbor-0.25.1/src/lib.rs` `16c916a21139bafae13a5d0ad10b42088a4d183446434ff7200d8e36a2f39474`
- `cardano-serializer-minicbor-dispatch-source-data-20261016/source/minicbor-0.25.1/src/encode.rs` `9046b48bdaddf26c11f40d900a91a0cc73395bdc291a81d5195a7039d197010e`

Provenance (root retrieval records, not re-verified here): MANIFEST.json records runtime.rs at Aiken `bacbeb35fbc9f1db93cd2e1e93fb1d93053acb4a` (blob `b00ea98889c9defc2eb1fcdb44036c60d10ea552`), the value.rs and lib.rs commit. It records Data.hs at IntersectMBO/plutus `9e17e2404dc6988c908b1fea099dde202df73b6a` (blob `a994354fb3c87a55ed6d1b4b6b544c7cd94c1c60`), a separate repository, so no revision is equated. No retrieval record for encode.rs was supplied to this author.

## 2. Interface (DESIGN)

- I1. `host_codec.py` defines exactly one public callable, `encode_data`.
  - It takes exactly one positional-or-keyword parameter named `data`, with no default.
  - It returns an object of exact type `bytes`.
  - It must not define stubs or the future names `build_prove_request`, `validate_prove_request`, `validate_context_input`, `build_value_data`, `build_output_reference`, `build_env_data`, `build_c2_mint_data`, `parse_statement_header`, `statement_digest_hex` or `statement_public_inputs`.
  - Private helpers (leading underscore) and non-callable constants are allowed.
  - Scope (R2): this gate, the FUTURE_APIS absence check and the I3 no-import rule bind this encode_data increment only. A later host_codec.py adding the separately frozen DTO/statement APIs, and any stdlib imports they need, replaces only these phase gates, through a fresh audit of the actual combined candidate. Every encode_data byte, admission, bound and rejection row stays mandatory. No stub, mirrored module, mock, financial interpreter or public admission policy is admitted.
- I2 (R2). Every input-dependent rejection (malformed shape, non-exact type, bool, out-of-range value, cycle, depth, node, leaf, magnitude or output bound) raises `ValueError` or a private subclass and returns no partial bytes. Traversal is iterative, with no Python recursion proportional to input depth. Under E1, no admitted or rejected input is expected to raise `RecursionError`, `TypeError` or `MemoryError`. Process-level failures (host memory exhaustion, signals, interpreter faults) are environmental: they are not converted into `ValueError` and are not claimed impossible.
- I3. Imports and conversions:
  - No imports, except optional `from __future__` imports.
  - The names `eval`, `exec`, `compile`, `open`, `__import__`, `breakpoint` and `input` must not appear.
  - Integers are converted with `int.to_bytes`, never via decimal `str`, because CPython's max-str-digits limit applies.
  - The B3 magnitude is checked with `int.bit_length` before any `int.to_bytes` allocation.
  - Error messages are fixed text and never format input values.
- I4. The function is pure and deterministic: no I/O, no global state, no mutation of its input.

## 3. Admission (DESIGN)

- A1. A node is an exact `tuple` (`type(x) is tuple`) whose first element is an exact `str` naming one of the following forms, with exactly this arity:
  - `('int', v)`: `type(v) is int`. A `bool` is rejected.
  - `('bytes', v)`: `type(v) is bytes`. `bytearray`, `memoryview` and `str` are rejected.
  - `('list', xs)`: `type(xs) is list`, and every element is a node.
  - `('map', ps)`: `type(ps) is list`, and each element is an exact 2-tuple of nodes.
  - `('constr', i, fs)`: `type(i) is int` with `0 <= i <= 2^64-1`, and `type(fs) is list` of nodes.
  - Any other tag, a miscased or bytes tag, wrong arity, a dict, None, or an invalid nested element raises `ValueError`.
- A2. Map pairs:
  - Any key order is admitted and emitted exactly as given.
  - Duplicate keys are admitted, because no read source rejects them.
  - Aliased (shared) subnodes are admitted and encoded at every occurrence.

## 4. Bounds (DESIGN; not source behaviour)

Neither the Aiken reencode nor the Haskell `encodeData` limits depth, count or size. These bounds make malformed, cyclic and hostile input finite. Bounds are enforced incrementally (R2): B1 and B2 when a node is visited, B3 before converting or copying a leaf, and B4 by a running or precomputed output total before any byte beyond 1048576 is materialised. Peak allocation per call is B4 plus O(B2) bookkeeping, independent of aliased expansion such as 65535 aliases of a 65536-byte leaf (about 4.43e9 expanded bytes).

- B1. Depth.
  - A leaf (int or bytes) has depth 1; so does an empty container.
  - A container has depth 1 plus the maximum depth of its children. Map keys and values are both children.
  - Depth 128 is accepted; depth 129 is rejected. Cycles are therefore rejected.
- B2. Node visits are counted over the expanded tree, including the root and every alias occurrence.
  - Up to 65536 visits are accepted; 65537 is rejected.
  - The count must abort early, which bounds DAG aliasing bombs.
- B3. Leaf and magnitude limits:
  - A bytes leaf may be at most 65536 bytes long.
  - A bignum magnitude may be at most 65536 bytes long, so integers lie in `[-2^524288, 2^524288-1]`.
- B4. Total output must be at most 1048576 bytes. 1048577 bytes is rejected.

Rationale: four-piece-path Data (EnvV2, OutputReference, C2Mint) has depth of about 4 or less, far fewer than 100 nodes, and leaves of at most 488 bytes. The chosen values are design choices; ledger limits were not re-read for this contract.

## 5. Encoding rules (SOURCE)

- S1. Arrays (list and constr fields): `lib.rs reencode_plutus_array`.
  - An empty array is encoded definite, as `80`.
  - A nonempty array is encoded as `9f`, then the elements, then `ff`.
  - The Def/Indef origin of the input is ignored.
- S2. Maps: `lib.rs reencode_plutus_map` forces a definite `e.map(len)` head, with pairs in the given order. `Data.hs encodeData Map` uses `encodeMapLen` with pairs in order. Both sources agree.
- S3. Bytes: pallas `BoundedBytes` `Encode` and `Data.hs encodeBs`/`to64ByteChunks` agree.
  - Lengths up to 64 use definite `e.bytes`.
  - Longer values use `5f`, then definite 64-byte chunks with a final remainder, then `ff`.
  - 128 bytes gives exactly two chunks, with no empty chunk.
- S4. Integers: `Data.hs encodeInteger` uses a CBOR integer for `0 <= i <= 2^64-1` and for `-2^64 <= i < 0`. minicbor `u64`/`int`/`type_len` choose minimal heads:
  - values up to 23 are inline;
  - head byte 24 carries 1 byte, 25 carries 2, 26 carries 4, 27 carries 8;
  - negative values use major type 1 with `n = -1-i`.
- S5. Bignums:
  - `Data.hs` uses tag 2 with `encodeBs(integerToBytesBE i)`, or tag 3 with `encodeBs(integerToBytesBE(-1-i))`.
  - pallas `BigInt` writes tag PosBignum/NegBignum, then the `BoundedBytes` encoding (S3).
  - The payload is the minimal big-endian magnitude with no leading zero (DESIGN, see U1 and U2).
- S6. Constructors: the `Data.hs` Note [CBOR alternative tags] and `encodeData` give this mapping:
  - 0..6 use tag `121+i`;
  - 7..127 use tag `1280+(i-7)`;
  - 128..2^64-1 use tag 102, then `82`, then the uint `i`, then the fields per S1.
  - Aiken `reencode_plutus_constr` matches: the tag, then for 102 `e.array(2)` and the u64 `any_constructor`, then the fields array.
  - Tag heads come from `type_len(TAGGED, t)`: 121 is `d879`, 127 is `d87f`, 1280 is `d90500`, 1400 is `d90578`, and 102 is `d866`.
  - Bool and Option shapes:
    - False is `d87980` and True is `d87a80`.
    - True wrapped in constructor 1 is `d87a9fd87a80ff`.
    - None is `d87a80`; Some is `d8799f..ff`.

## 6. Disagreements and unverified correspondences (disclosed, not merged)

Disagreements between source profiles:
- X1. Constructor indices that are negative or at least 2^64.
  - `Data.hs` emits soft-failure bytes that will not deserialise.
  - pallas `any_constructor: u64` cannot represent them.
  - DESIGN: reject them (A1).
- X2. Aiken writes the stored tag, so a decoded tag 102 carrying an index below 128 re-encodes as 102.
  - The Python algebra carries only the index, so `encode_data` always emits the compact (Haskell) form.
  - A byte-exact round trip of such on-chain data is not provided and is out of scope.
- X3. The pallas generic `Encode` preserves Def/Indef. It is not the serialise_data path and is not followed.

Unverified correspondences:
- U1. Aiken's Integer to `BigInt` boundary is now SOURCE (R2). runtime.rs `IData` calls value.rs `to_pallas_bigint`. That yields a CBOR `Int` when `to_i128` and pallas `Int::try_from(i128)` succeed (data.rs range [-2^64, 2^64-1]); otherwise it yields `BigUInt(to_bytes_be(n))` or `BigNInt(to_bytes_be(n+1))`. This matches `Data.hs encodeInteger`. runtime.rs `ConstrData` rejects indices outside u64. num-bigint `to_bytes_be` minimality is unread, so S5 minimality stays DESIGN; the runtime.rs `IntegerToByteString` use is corroboration, not proof.
- U2. The body of `integerToBytesBE` (PlutusCore.Bitwise) is unread.
- U3. Now SOURCE (R2): minicbor lib.rs fixes SIGNED 0x20, BYTES 0x40, ARRAY 0x80, MAP 0xa0 and TAGGED 0xc0; data.rs maps PosBignum/NegBignum to tags 2/3; encode.rs dispatches `u64` to `e.u64` and `Int` to `e.int`. Still unread: minicbor-derive `#[cbor(transparent)]` for pallas `Int`, and `encode/write.rs`.
- U4. The serialise/cborg `Serialise [a]` instance is unread. Rule S1 rests on `lib.rs` and its cborg comment. cborg `encodeTag`/`encodeInteger` are also unread.
- U5. The `ast.rs` `Data::constr` and `Data::list` bodies were captured but not supplied.

Decode acceptance is out of scope:
- Decoders accept indefinite maps, definite nonempty arrays and arbitrary chunking.
- pallas has no 64-byte check; Haskell `decodeBoundedBytes` does.
- None of this is `encode_data` output.

## 7. Superseded vectors (R1)

The following parent R2 values are wrong:
- `d87a9fff` for constr 1 [] (Burn); the correct value is `d87a80`.
- `d87a9fd87a9fffff` for VBool True; the correct value is `d87a9fd87a80ff`.
- The parent plan rules of indefinite maps, tag `1280+index`, and no tag-102 path.

These parent vectors remain consistent and correct: `00`, `186d`, `420102`, `9f0102ff`, `d8799f186dff`, and the outref `d8799f5820..03ff`.

The parent files stay immutable. Their wrong vectors must not be used as `encode_data` gold.

## 8. Ordinary execution proposal (E1)

This proposal is not authority for native, SDK, curve, signature, proof or ledger operations.

1. Root performs an explicit import audit before any run.
   - The test imports only `ast`, `inspect`, `stat`, `sys`, `unittest`, `importlib.util` and `pathlib`.
   - `host_codec.py` imports nothing.
   - R2: this human full import/effect audit is mandatory. The in-test AST audit is best-effort, not a sandbox: `getattr`/`__builtins__` lookups and dunder attributes are not blocked, and `exec_module` re-reads the file after the audit. Optional hardening is not implemented.
2. Run `python3 -I -B /absolute/dir/test_plutus_data_serialisation.py`.
   - Use a dedicated directory holding only the two `.py` files. The runner may create that directory and copy the files in.
   - Wall-clock limit 10 s and address space 512 MiB.
   - No network and no secrets in the environment.
3. Test behaviour:
   - It sets `sys.dont_write_bytecode` and writes nothing.
   - It reads only the regular sibling `host_codec.py`. It rejects a missing or symlinked file before import and asserts the same resolved `__file__`.
   - Peak test data is a few MiB.
4. Observed: 18 RED (missing implementation), then 18 GREEN against the unchanged Sol candidate. R2 adds rows for behaviour that candidate already has by source trace, so no RED is expected for them. GREEN is not compiled-VM, ledger or security acceptance.

The implementation was written by GPT6.1Sol HIGH. The Muse Wonder HIGH audit is unavailable (the last 3 Muse attempts (one NAV job and two canaries) returned zero qualified responses); that is not approval and nothing substitutes for it. This author owns only the R2 security correction and is not the implementation auditor.

## 9. Non-claims and open items (N1)

The author ran no tests.

No claim is made that:
- the captured Aiken release ELF equals this source;
- the encoding is equivalent on-chain, in the ledger, or in a compiled validator. Compiled and ledger correspondence remain open.

Big integers are data, not cryptographic arithmetic. The 488-byte and 192-byte values are opaque public shapes; no VK or proof is validated. Owner-intent correctness and current genuine proof correctness are unchanged.

Still open:
- DTO, ledger-context, intent and transaction modules;
- the general compiler-VK relation;
- 20 UTxO families;
- ACTUS 277/18/32 and DS01-07;
- DeFi 72 and DA01-24;
- held-outs, intents and regressions;
- recursion, private handoff, multi-parent and universal;
- real Preprod.
