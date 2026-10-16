# CORE-REQUEST-WIRE-CONTRACT

Status: AUTHOR PROPOSAL for a fresh non-author Opus review in a separate context.

- Author: claude-opus-5-5, security source-contract role. Tools were off, so nothing was compiled, imported or run.
- This is not an independent audit, an implementation or an adoption.
- Operator, native, authority and publication flags are all false.
- The old broad plan and test remain rejected and are not copied. No frozen parent is rewritten.

## 1 Scope

In scope: three pure `host_codec` functions for the Rust `moriarty-k2-prove-request/1` DTO (k2 `input.rs`).

Out of scope, owned elsewhere and still open: `encode_data` (disjoint author), intent signing, ledger context, CLI/transaction assembly, statement/export parsing, VK/proof/crypto verification, and Core simulation or any financial evaluation.

Compile, prove and deploy stay permissionless: there is no action, program or filename allowlist, and no new native authority.

## 2 Source pins (verbatim from the supplied input index)

Prefixes:

- S = `/home/charl/moriarty-eutxo-validation-2026-10-12/current-rust-real-preprod-proof-binding-source-data-sol-20261016/source/experiments/moriarty-cardano-rust/`
- V = `/home/charl/moriarty-eutxo-validation-2026-10-12/`
- G = `/home/charl/Moriarty/.worktrees/utxo/evidence/utxo/current-rust-groth16-first-proof-2026-10-16/`

| path | bytes | sha256 |
|---|---|---|
| S`k2/src/input.rs` | 6262 | 646c1aad3cea935de2e084e2cb1518772e8bd0fc7e57eb6b3f4581edc67b5b42 |
| S`k2/src/relation.rs` | 14625 | e18ae46c5ec0650f0c5dbc3c8b39262eb432c7d2e3be7b3b14cc01fe285493d5 |
| S`k2/src/claim.rs` | 9280 | f54545d8b8286cad1c3549afab82a3b15020c0b498041d093ce8e9f3cb4541d3 |
| S`k2/src/export.rs` | 15374 | 8eb9268d81d2b50513f9f1e54d400aef08993ed1eee78ecc119c9398455dcb9c |
| S`r1cs/source/mori-r1cs/src/abi2.rs` | 19304 | 51fa106305481cb6c48c9eeda0d2b24ae5e5f7b9816daa48b2b474b56e98debb |
| S`r1cs/source/mori-r1cs/src/statement.rs` | 20068 | 956b85251f1f9a8ed03617a58fff6e9ee6c388f3147dcb14b227f8555cd36df5 |
| S`r1cs/source/mori-r1cs/src/adapter.rs` | 15142 | 24dcee35856ee8bf6ef3dcde8be39b56692af34e99fc5975bd1230dc07a91b1d |
| S`moriarty/crates/mori-core/src/ir.rs` | 12823 | 0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d |
| S`moriarty/crates/mori-core/src/runtime.rs` | 37400 | 7c736b2f817c3dc095df27d96744369c3357c758267f9838564c8f9e583ac27d |
| S`moriarty/crates/mori-core/src/json.rs` | 5391 | faef7db8115320cc483dce30d7b5d22bd01a780facae1c761a8c9203fbb66183 |
| V`current-cardano-fx1-host-codec-expectations-r2-candidate-20261016/independent_expectations.py` | 58184 | f9f1dcca617c03a07e1c647d204ce681b0e8fd98e10bea3f5380f6e5fffa0b63 |
| V`current-cardano-host-codec-r2-review-source-facts-sol-20261016/CLAIM-COMPARISON.json` | 30306 | 49fae8657830bc37250291c2db032fc7473c0fd804c50714f56a59dc271463cb |
| V`current-cardano-k11-r4-opus-semantic-source-repair-20261016/FINDINGS.json` | 2911 | dc47fd6ce54a918ee1e57c82ce9f99057d9f25a305f3bd0559a5acfa9cfe1929 |
| V`current-cardano-k11-r4a-semantic-source-candidate-20261016/SOURCE-MANIFEST.json` | 4151 | 825afc8791ee2dd8a8c2850958047265809555f2c34e7e01c6864d482236a308 |
| G`gold/K4H-GOLD.json` | 39353 | 65c90d3d13912b762b4977e08ea86212b2b83a16bb2b3ddcaa7a21c905075e46 |
| G`fixtures/installment-invoice.mori` | 1497 | 105137af8ac6a1f060ec7b634e3ec4aa913808e94bd1b7cffb736f8105d2cfc9 |
| G`fixtures/invoice-partial.request.json` | 3008 | aa1703e7b3aa2e0dd6288668cc871996fc1ee49ecc90861a3c4b40f02107f3cd |
| G`fixtures/conditional-reserve.mori` | 1415 | 395e647534160fde45325232e5f1650a8bd48ce1455a68b28af203bf8bdbe357 |

The following were not read: k2 `lib.rs` (`ADAPTER`), `hexutil.rs`, mori-r1cs `witness.rs`/`plan.rs`, and the reserve request.

## 3 API

### `build_prove_request`

Signature: `build_prove_request(action, transition, context, expected_source_sha256=None, expected_program_hash=None) -> dict`

- Returns a new `{format, selector, transition, context}`.
- The selector is the fixed constants plus `action`. A pin key is emitted only when its argument is not `None`.
- `transition` and `context` are deep-copied verbatim. Nothing is sorted, dropped, defaulted or coerced, and the presence or `null` of `vkDigest` is preserved.
- The whole result is then validated.
- Source text is never a parameter or a field. It stays a separate Rust driver input (`read_source`, at most 262144 bytes of UTF-8).

### `validate_prove_request`

Signature: `validate_prove_request(request) -> True`

- Structural check only.
- Never mutates or reorders the request.

### `validate_context_input`

Signature: `validate_context_input(context) -> dict`

- Returns a new dict containing the validated context fields as given, plus `authentication: CALLER_SUPPLIED_UNAUTHENTICATED`.
- This labels caller data; it is never ledger authentication.
- The result is not wire data: `ContextInput` refuses its extra key.

### Common rules

- Every refusal raises `ValueError`, never `TypeError`, `KeyError` or `AttributeError`.
- No cryptographic verification.
- No trust in caller-claimed authentication.
- No Core evaluation.

## 4 Wire schema

Every object is closed (`deny_unknown_fields`).

- **ProveRequest**:
  - `format` = `moriarty-k2-prove-request/1`
  - `selector`, `transition`, `context`
- **RelationSelector** (camelCase):
  - `format` = `moriarty-k2-relation-selector/1`
  - `language` = `moriarty/1`
  - `semantics` = `moriarty-core/1`
  - `action`: 1..1024 UTF-8 bytes
  - `adapter` = `SettlementHashV2`
  - `cap` = `NotBound`
  - `accounting` = `GrossDebitAndFees`
  - `frame` = `TouchedOnly`
  - optional `expectedSourceSha256` and `expectedProgramHash` (hex32)
- **TransitionInput**, tagged by `kind`:
  - Either `CoreReference{scenario}` or `Records{inputs, pre, post, effects}`.
  - There is no default kind, no `mode` and no `core_ir`.
  - CoreReference never carries pre/post/effects. Core `simulate` proposes them inside Rust; the host computes none.
- **Scenario**: `state` and `inputs` (map str -> Value), `balances` [Balance], `liabilities` [Liability]. All four are required: the struct derives Default but has no `#[serde(default)]`.
- **Snapshot**: `state`, `balances`, `liabilities`.
- **Balance**: `{account, asset, amount}`.
- **Liability**: `{name, asset, debtor, creditor, amount}`.
- **AssetId**: `{network, issuer, reference, scale}`, where scale is a u8 in 0..255.
- **Value**, tagged by `type` with content in `value`:
  - `{UInt, canonical decimal string}` or `{Bool, JSON bool}`.
  - UInt is non-empty ASCII digits, with no leading zero except `0`, and at most 2^128-1.
- **Effect**, tagged by `kind`:
  - `State{name, before:Value, after:Value}`
  - `Transfer` and `Fee`: `{asset, from, to, amount:UInt}`
  - `Liability{name, asset, debtor, creditor, before:UInt, after:UInt}`
  - `Emit{name, value:Value}`
  - There is no Discharge or Accrue kind.
- **ContextInput** (camelCase):
  - `claimantVkh` hex28, `networkId` u8, `settleHash` hex28
  - `threadId` and `consumedTxid` hex32
  - `consumedIx` u16, `seq` u64
  - `catalogueDigest` and `intentsDigest` hex32
  - optional `vkDigest` hex32

## 5 Admission: serde equivalence (S) vs proposed host rule (H)

| id | rule | basis | kind |
|---|---|---|---|
| A1 | unknown or snake_case keys refused | deny_unknown_fields, rename_all camelCase | S |
| A2 | Scenario 4 / Snapshot 3 fields required | no serde default | S |
| A3 | `vkDigest` and pins: omitted, null or valid | Option + default | S |
| A4 | hex: exactly 2N lowercase chars | decode_exact::<N>; hexutil unread | H |
| A5 | ints are Python int, not bool or float, in range | u8/u16/u64 | S |
| A6 | UInt canonical ASCII decimal <= 2^128-1 | ir.rs UInt deserializer | S |
| A7 | Value type/value tagging; Bool is a JSON bool | ir.rs Value | S |
| A8 | action 1..1024 UTF-8 bytes, any content | check_selector | driver |
| A9 | fixed selector and format strings | check_selector, parse_request | driver |
| A10 | scale 0..255; <=38 not checked | u8 vs runtime validate | S |
| A11 | networkId 0..255; no network naming | u8 caller premise | S |
| A12 | str must be UTF-8 encodable; empty allowed | Rust String | H/S |
| A13 | no Core runtime or financial rule | structural only | non-goal |
| A14 | verbatim deep copy, no sorting | R4-A03 order sensitivity | H |
| A15 | ValueError only | API | H |

A13 means the validator does not check non-empty identifiers, duplicate rows, row or effect order, types against the program, or Emit refusal. A wrong financial story can therefore be wire-valid, and the tests require that it is.

## 6 Byte-parser stage (not claimed)

`json.rs` `parse_limited` enforces:

- refusal of duplicate decoded keys and of trailing data;
- depth at most 32;
- at most 65,536 nodes;
- at most 1,048,576 bytes.

A dict validator cannot see duplicates after the dict is constructed. No test pretends otherwise, and no byte parser is implemented here. Python `json.loads` quirks belong to that future stage: last-wins duplicates, NaN, and big ints.

## 7 Fixtures and crosschecks

- `original_request()` transcribes the CoreReference invoice request in its original row order. Root must check `json.loads(exact bytes) == literal`.
- `invoice_records()` is the complete invoice claim:
  - Effects: Fee 2, Transfer 35, Liability invoice 90->55, State paid 0->35.
  - Unchanged: marker 7, opened true, reserve 14, spectator 41.
  - Balances: escrow 109->72, fees 0->2, seller 0->35.
- `reserve_records()` is derived from GOLD and `conditional-reserve.mori`. The original reserve request is unread, so coverage is narrow.
- Both programs remain positive obligations for the future pipeline.

Crosschecks:

- Fixtures are crosschecked by label lookup against the sha-pinned sibling `K4H-GOLD.json`, covering all 75 rows.
- 69 rows are looked up. 6 `accounting.*` rows are compiler-derived and absent from the wire, so they are left to real compiler coverage.
- Absent named duties map to `(false, 0)` (C09).
- These are fixture cross-consistency checks, not real compiler coverage.

GOLD caveats:

- `declaredVkDigest` is SYNTHETIC.
- r1cs/predicate/shape digests and counts are historical observations, not the current relation.
- `bodyLen` 378 and `statementLength` 721 are not a generic rule.

Derived row order:

- The canonical order in the Records fixtures is a NEW array.
- The original and derived scenario hashes both stay recorded (R4-A03).
- A K6 proof is never valid for the reordered array and is never relabelled.

## 8 Ordinary self-test proposal (no command run by the author)

Setup:

- Directory contains exactly this test, `host_codec.py` and `K4H-GOLD.json`.
- First audit `host_codec.py` import effects. It must use stdlib only, and at top level must perform no I/O, subprocess, network, randomness, environment reads, sys.path edits or writes.

Command: `python3 -I -B /abs/dir/test_core_request_wire.py`

- 10 s wall clock
- 512 MiB address space
- no network
- read-only directory

Before implementation, gold tests are expected to pass and codec tests to fail with `missing implementation`. No result is claimed.

## 9 Open (not closed here)

- Real Rust parse, compile and prove of these requests.
- Export check: 343-byte header plus a program-dependent body.
- Complete execution-gold comparison.
- General compiler/VK relation.
- 20 UTxO families.
- ACTUS 277/18/32.
- DS01-07.
- DeFi 72.
- DA01-24.
- Held-outs.
- Intents.
- Regressions.
- Recursion.
- Private handoff.
- Multi-parent.
- Universal.
- Real Preprod.
- `encode_data`.
- Ledger context.
- CLI assembly.

Positions on each consequential admission decision (A1-A15) are author FOR proposals for review, not adoptions.
