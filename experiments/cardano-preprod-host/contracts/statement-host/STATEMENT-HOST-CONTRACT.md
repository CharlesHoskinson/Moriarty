# Statement host contract: three ABI v2 statement APIs for host_codec.py

Status: PROPOSED source contract and independent test source only. Author: claude-opus-5-5, HIGH cryptographic/security source-contract and independent-test author, tools off. The author executed, imported, compiled and hashed nothing; RED and GREEN are unobserved; no implementation is supplied. Nothing here grants native, operator, SDK, VM, key, signing, proving, network, ledger or publication authority. Clauses SH-01 to SH-22 are stated in full, with AUTHOR_FOR positions, in DESIGN-POSITIONS.json. This document explains them, and the JSON text governs on any wording difference.

## 1. Scope and placement (SH-01, SH-18)

The increment appends exactly three public functions to the existing `host_codec.py`:

- `parse_statement_header(statement) -> dict`
- `statement_digest_hex(statement) -> str`
- `statement_public_inputs(statement) -> tuple[str, str]`

Rules:

- The current file (16134 bytes, SHA-256 `8e15bc783afc7c8c96268ea1ade0f881e631d9397a59d9642d4b2a1bbc62a838`) is an exact, unchanged prefix. `encode_data`, `build_prove_request`, `validate_prove_request` and `validate_context_input` are untouched.
- New code, including one module-level `import hashlib`, is appended after the final prefix byte. The appended import is permitted for this increment only after a fresh full import and effect audit. Private helpers use a leading underscore. The three public names are plain `def` functions.
- The separate four Data factory functions are a different increment. The two appends may land in either order, because the tests pin only the 16134-byte prefix.
- Out of scope: statement body generation, compiler or application-IR substitutes, proof generation or verification, claim authentication, signature handling, VK or descriptor checks, Rust or Aiken invocation, and any network or ledger action. The rejected old IMPLEMENTATION-PLAN is used only as an API-name index; its wider plan, CBOR rules and 49-method test are not adopted.

## 2. Primary sources

All inputs come from the frozen public index. Line ranges are those recorded in HOST-CONSUMPTION-COVERAGE.json; the symbol names are authoritative.

| Id | Path (abbreviated) | Bytes | SHA-256 | Role |
| --- | --- | --- | --- | --- |
| claim | `k2/src/claim.rs` | 9280 | `f54545d8b8286cad1c3549afab82a3b15020c0b498041d093ce8e9f3cb4541d3` | `MAX_STATEMENT_BYTES` (15-16), `offsets` (18-32), `digest_halves` (46-53), `parse_statement` (137-181) |
| abi2 | `mori-r1cs/src/abi2.rs` | 19304 | `51fa106305481cb6c48c9eeda0d2b24ae5e5f7b9816daa48b2b474b56e98debb` | `STMT_TAG`, `FAMILY_V2`, `HEADER_LEN` (33-37), `ContextV2::bytes`, `statement_bytes` (280-294), `public_inputs` (297-305) |
| export | `k2/src/export.rs` | 15374 | `8eb9268d81d2b50513f9f1e54d400aef08993ed1eee78ecc119c9398455dcb9c` | `PREMISES`, `ContextExport`, `PublicInputsExport` (140-145), `check_export` |
| core | `pvc2/core.ak` | 19555 | `2f4fbf3b5a9c6f09afaa0bf9342a36e0d547ee993e82b854682d05c148f60cfa` | `stmt_tag`, `family_v2`, `statement`, `body` |
| fixture | `run/receipts/invoice-partial.proof.json` | 5800 | `930198631617f1f931aa93e836b32ffa8acadedadafe3b3d90081ff629e9ba5b` | genuine public test vector (section 7) |
| host | `host_codec.py` | 16134 | `8e15bc783afc7c8c96268ea1ade0f881e631d9397a59d9642d4b2a1bbc62a838` | conventions and preserved prefix |
| oldPlan | `IMPLEMENTATION-PLAN.md` | 37457 | `610226cc1c6780913853a8b7d3b515db41d3043d59b6b0c28b7df5cccf61396a` | REJECTED; API names only |

## 3. Source profile

### 3.1 Constants (SH-02)

- Tag, bytes [0,16): ASCII `MRG16-PVC-STMT-2`, hex `4d524731362d5056432d53544d542d32`.
- Family, bytes [16,48): ASCII `MRG16-PVC-FAM2-G16-BLS-ARK04-HL2`, hex `4d524731362d5056432d46414d322d4731362d424c532d41524b30342d484c32`.
- Both are fixed-size Rust byte-string literals (`&[u8; 16]` and `&[u8; 32]`) with no padding or terminator, compared by exact equality in `parse_statement`. `core.ak` uses the same strings, and the fixture `cardano.envV2.familyId` is the same family hex.
- `HEADER_LEN = 343`; `MAX_STATEMENT_BYTES = 262144`.

### 3.2 Fixed header (SH-03)

| Returned key | Offset | Width | Encoding | Rust source |
| --- | --- | --- | --- | --- |
| (tag, validated, not returned) | 0 | 16 | literal | `STMT_TAG` |
| (family, validated, not returned) | 16 | 32 | literal | `FAMILY_V2` |
| predicateDigest | 48 | 32 | raw bytes as lowercase hex | `offsets::PREDICATE` |
| shapeDigest | 80 | 32 | raw bytes as lowercase hex | `offsets::SHAPE` |
| vkDigest | 112 | 32 | raw bytes as lowercase hex | `offsets::VK_DIGEST` |
| claimantVkh | 144 | 28 | raw bytes as lowercase hex | `offsets::CLAIMANT` |
| networkId | 172 | 1 | u8 | `offsets::NETWORK` |
| settleHash | 173 | 28 | raw bytes as lowercase hex | `offsets::SETTLE` |
| threadId | 201 | 32 | raw bytes as lowercase hex | `offsets::THREAD` |
| consumedTxid | 233 | 32 | raw bytes as lowercase hex | `offsets::TXID` |
| consumedIx | 265 | 2 | u16 big-endian | `offsets::IX` |
| seq | 267 | 8 | u64 big-endian | `offsets::SEQ` |
| catalogueDigest | 275 | 32 | raw bytes as lowercase hex | `offsets::CATALOGUE` |
| intentsDigest | 307 | 32 | raw bytes as lowercase hex | `offsets::INTENTS` |
| bodyLen | 339 | 4 | u32 big-endian | `offsets::BODY_LEN` |
| body | 343 | bodyLen | opaque bytes | `s[HEADER_LEN..]` |

The table is contiguous and ends at 343. In `bind_v2`, bytes 112..339 are free context witness bytes, and bytes 0..112 plus 339..343 are circuit constants. The host draws no consequence from that split.

### 3.3 Framing and admission (SH-04)

`parse_statement` accepts exactly the byte strings `s` with all of the following:

- 343 <= len(s) <= 262144;
- s[0:16] equals the tag;
- s[16:48] equals the family;
- 343 + u32be(s[339:343]) equals len(s).

Its final re-encoding through `abi2::statement_bytes` rebuilds the same bytes from fields it just read, and the body-length recheck in `statement_bytes` is already implied. So for such input it refuses nothing further; its purpose is to detect drift between the `claim.rs` offsets and the compiler encoder. The host admission set over exact `bytes` is the same set. Consequences:

- a 343-byte header with bodyLen 0 is admitted;
- exactly 262144 bytes with consistent bodyLen is admitted;
- 262145 bytes is refused even when consistently framed;
- no bodyLen above 261801 can ever be admitted.

### 3.4 Digest and public inputs (SH-11, SH-13)

- Digest: unkeyed BLAKE2b with 32-byte output over the whole statement (`mori_r1cs::blake2b::blake2b256(&statement)` in `claim_from_values` and `check_export`; `abi2::public_inputs`). Python form: `hashlib.blake2b(statement, digest_size=32)` with no key, salt or person.
- Halves: `x1 = int_be(d[0:16])` and `x2 = int_be(d[16:32])` (`digest_halves`), exported as canonical decimal strings (`PublicInputsExport`). Values lie in [0, 2**128). The host applies no field reduction or curve arithmetic.

### 3.5 Aiken consumer agreement

`core.ak` `statement` concatenates the same fields in the same order with `bytes.u8`, `bytes.u16`, `bytes.u64` and `bytes.u32(bytes.len(body_bytes))`, so on-chain bodyLen is the actual body length by construction. `core.ak` imposes no 262144 cap: the host cap gives parity with the Rust driver parse limit and is not an on-chain rule.

## 4. API contract

### 4.1 Common admission (SH-05, SH-06, SH-12)

Each of the three functions performs these steps in order and raises `ValueError` at the first failure:

1. A1 type: `type(statement) is bytes`. Nothing else about the argument is touched first: no `len`, indexing, comparison, buffer export or method call.
2. A2 bounds: `343 <= len(statement) <= 262144`.
3. A3 tag: `statement[0:16]` equals the tag literal.
4. A4 family: `statement[16:48]` equals the family literal.
5. A5 framing: `int.from_bytes(statement[339:343], 'big') + 343 == len(statement)`.

`statement_digest_hex` and `statement_public_inputs` run A1 to A5 before hashing and hash nothing on refusal. They accept and refuse exactly the same inputs as `parse_statement_header`.

### 4.2 parse_statement_header(statement) -> dict (SH-07)

Returns a new exact `dict` with exactly 14 keys:

- `predicateDigest`, `shapeDigest`, `vkDigest`, `threadId`, `consumedTxid`, `catalogueDigest`, `intentsDigest`: exact `str` of 64 lowercase hex characters;
- `claimantVkh`, `settleHash`: exact `str` of 56 lowercase hex characters;
- `networkId` (0..255), `consumedIx` (0..65535), `seq` (0..2**64-1), `bodyLen`: exact `int`, never `bool`;
- `body`: exact `bytes` equal to `statement[343:]`.

No other key, authentication label or derived value is added. The tag and family are validated but not returned.

### 4.3 statement_digest_hex(statement) -> str (SH-11)

After admission, returns exact `str` of 64 lowercase hex characters: the 32-byte unkeyed BLAKE2b of the entire statement.

### 4.4 statement_public_inputs(statement) -> tuple[str, str] (SH-13)

After admission, returns an exact `tuple` of exactly two exact `str`: canonical ASCII decimal (no sign, no leading zero except for 0) of the big-endian integers of digest bytes [0,16) and [16,32). For the same input, the halves correspond exactly to the digest returned by 4.3.

### 4.5 Errors (SH-06)

Ordinary refusals raise exactly `ValueError`, never a subclass and never a leaked `TypeError`, `IndexError`, `AttributeError`, `OverflowError` or `UnicodeError`. Message text is not frozen. Environmental failures (MemoryError, KeyboardInterrupt and similar) propagate unconverted and no behaviour under them is claimed.

### 4.6 Purity (SH-14)

The functions do no I/O and use no environment, clock, randomness, global mutable state, logging or caching. Each parse call returns a new dict. The input is never mutated, which exact `bytes` guarantees anyway.

## 5. Host design choices beyond source

| Id | Clause | Choice | Why it is a design rather than Rust law |
| --- | --- | --- | --- |
| D1 | SH-05 | Exact builtin `bytes` only. `bytearray`, `memoryview`, `str`, bytes subclasses and other types are refused, with no copy. | Rust borrows an immutable `&[u8]`. Python mutable buffers could change between validation and hashing, and subclasses carry caller hooks. |
| D2 | SH-06 | Fixed check order, bounds before any slicing or hashing, exact `ValueError`. | Rust returns `DriverError` variants; the class and order are host conventions. |
| D3 | SH-12 | Digest and public-input functions validate before hashing. | `abi2::public_inputs` hashes any slice; only `check_export` parses first. |
| D4 | SH-07 | Closed camelCase dict, lowercase hex, exact `int`, body as `bytes`. | Rust returns a `StatementView` struct with byte arrays and omits the body. |
| D5 | SH-13 | Public inputs as canonical decimal strings. | Rust uses `u128` and field elements internally; only the export uses decimal strings. |

## 6. Non-claims (SH-08, SH-09, SH-10, SH-20)

- The body is opaque: only its length is checked. No slot, width, Bool byte, layout, accounting or program meaning is decoded or checked.
- A header-valid statement is not thereby a valid statement of any compiled relation. No predicate, shape, VK, descriptor bodyLen or program pin is compared (`check_statement_against` has no host analogue in this increment).
- Context fields are caller-declared premises (export.rs `PREMISES[1]`), reported as found and never authenticated.
- Reconstructing the fixed header from parsed fields is a test-side parity check against genuine bytes. It is not the Rust compiler re-encoding.
- This is source-inspected parity only. VK identity, raw-source binding, regression coverage, native roundtrip, compiled Aiken correspondence and Preprod all remain open.

## 7. Genuine public fixture (SH-15, SH-16)

Root stages the exact retained bytes as sibling `K6-PUBLIC-PROOF.json` (5800 bytes, SHA-256 `930198631617f1f931aa93e836b32ffa8acadedadafe3b3d90081ff629e9ba5b`). The author's manual, unexecuted reading of its `statementHex` header:

| Field | Value |
| --- | --- |
| tag / family | the SH-02 literals |
| predicateDigest | `0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd` |
| shapeDigest | `36310727c784c8a8b1204a9cb2de5eb60193afd5517179c16ebd1ae3a92e2969` |
| vkDigest | `d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd` |
| claimantVkh | `55` repeated 28 times |
| networkId | 0 |
| settleHash | `a2` repeated 28 times |
| threadId | `7a7fafdd81303746cd8af0fc48add098ab6468e5c510042d41be69f33abcf29a` |
| consumedTxid | `22` repeated 32 times |
| consumedIx | 0 |
| seq | 1 |
| catalogueDigest | `37de4837bc1417d65df0c40b4829d796b04d97fd037bb128fc0814abb5646e18` |
| intentsDigest | `b8e2f734456ad3d939e91441330cb0635707e09d70af45edead18923108fc308` |
| bodyLen | `0000017a` = 378; total 343 + 378 = 721 bytes |

Published digest `c9ef8ea00a7f7b71fc1e049bd956dc8a068e635e31306aca88210711282384cc`; x1 `268418678883876601707409168117995592842`; x2 `8714689549842776844379904284721120460`.

The fixture context is caller-declared, not Preprod ledger state. The setup is single-party and forgeable by its owner. Compiler-to-matrix and VK correspondence is not proved. Parsing or hashing the fixture verifies no proof, signature or ledger truth, and `proofHex` is never parsed. Edited copies used by the tests are structural boundary test data with no financial interpretation.

## 8. Independent tests (SH-17, SH-18, SH-19)

### 8.1 Harness

- Stdlib only: hashlib, importlib.util, json, os, stat, sys, types, unittest.
- It reads only the sibling fixture and the sibling `host_codec.py`, each verified as a regular non-symlink file and read with O_NOFOLLOW.
- It loads the host by importlib file-location loading, with `sys.dont_write_bytecode` set. It never uses string eval or exec, subprocess, network, writes, RNG, SDK or VM.
- SHA-256 is used only for the fixture pin and the host prefix pin. The tests never compute BLAKE2b; every expected digest and half is a fixture literal.
- API lookup goes through the module dict, before any call. A missing file or name fails with an assertion containing `missing implementation`. There are no skips, xfails or fallbacks.

### 8.2 Methods (19)

1. `test_public_fixture_pin_and_literal_consistency` (no host API)
2. `test_host_prefix_preserved_byte_for_byte` (no host API)
3. `test_three_statement_apis_are_present_plain_functions`
4. `test_parse_genuine_fixture_header_fields`
5. `test_parsed_fields_reconstruct_exact_fixed_header`
6. `test_integer_fields_are_unsigned_big_endian`
7. `test_digest_matches_published_fixture_literal`
8. `test_public_inputs_are_published_big_endian_digest_halves`
9. `test_wrong_tag_refused_by_all_three_apis`
10. `test_wrong_family_refused_by_all_three_apis`
11. `test_truncated_and_overlong_framing_refused`
12. `test_body_length_field_mismatch_refused`
13. `test_structural_length_boundaries` (343, 262144 admitted; 262145 refused)
14. `test_far_oversized_inputs_refused`
15. `test_non_bytes_inputs_refused`
16. `test_bytes_subclass_refused_without_invoking_hooks`
17. `test_body_bytes_are_opaque_to_header_parsing`
18. `test_context_and_relation_fields_reported_not_checked`
19. `test_results_deterministic_and_detached`

### 8.3 RED expectation (unobserved)

Against the existing 16134-byte host, methods 1 and 2 pass and methods 3 to 19 fail with assertion text containing `missing implementation`. Any other observation stops the step for review.

### 8.4 Later run envelope

Run `python3 -I -B test_statement_host.py` in a dedicated directory containing only `test_statement_host.py`, `host_codec.py` and `K6-PUBLIC-PROOF.json`, with directory mode 555 and file modes 444. Limits: timeout 10 s, address space 512 MiB, cpu 10 s, core 0, minimal environment, no network, and a post-run directory check. The largest constructed input is about 4.2 MiB.

## 9. Future phase and audit routing (SH-21)

The digest and public-input derivation is cryptographic, so under current routing Opus writes the implementation. One fresh non-author Opus actual combined audit, with same-provider independence disclosed, then reviews these SH clauses together with the separate four Data factory clauses in a single review, without duplicated full auditors. The Sol Data factory portion separately requires the Muse audit; a missing Muse audit is not approval and cannot be replaced.

## 10. Open obligations (SH-22)

All authority stays zero. Still open: all 20 UTxO families, current Rust acceptance, compiler and VK matrix correspondence, authenticated ledger context, the catalogue 42 provenance gap, ACTUS 277/18/32 and DS01-07, DeFi 72 with DA01-24, held-out intent regressions, KQuint, recursion, private handoff, multi-parent, universal and Preprod.
