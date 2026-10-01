# Midnight Rust cryptography experiments

Executable local tests using pinned official Midnight Rust libraries. This is an experimental verifier and conformance harness. It does not qualify Moriarty transactions for ledger acceptance.

## Run

Requires Rust/Cargo and Node 24. The first Cargo build fetches Git dependencies and registry crates. `Cargo.lock` pins the complete resolution. From the repository root:

```sh
node --experimental-strip-types experiments/midnight-crypto/export-fixtures.mjs
CARGO_BUILD_JOBS=2 cargo test --locked --manifest-path experiments/midnight-crypto/Cargo.toml --features state,native-proof,ledger -- --nocapture
```

Set `CARGO_TARGET_DIR` to an external build directory if needed. Without optional features, the signature, authority and CLI tests run. The exported fixtures come from the actual beta → Source/6 → Core/5 caller. The exporter independently checks complete ordered effects and post-state for fee/zero-fee transfer, interest-only repayment, partial repayment and full repayment.

## Readonly verifier

```sh
cargo run --locked --manifest-path experiments/midnight-crypto/Cargo.toml -- frame < statement.json
cargo run --locked --manifest-path experiments/midnight-crypto/Cargo.toml -- verify < request.json
```

A verify request has exactly these fields:

```json
{"scheme":"schnorr_bip340","statement":{"amount":"1000"},"public_key_hex":"<32-byte raw key as hex>","signature_hex":"<64-byte raw signature as hex>","wallet_prefix":false}
```

For `ecdsa_secp256k1_sha256`, use a 33-byte compressed raw key and a 64-byte raw signature. Set `wallet_prefix` to true only when the signature covers the wallet-prefixed frame. Ephemeral signing exists only in tests. Exit codes: 0 valid/frame, 1 invalid signature, 2 malformed request. A valid result leaves authority, snapshot membership and transition validity null and ledger acceptance false.

## Experimental byte codec

`moriarty-midnight-auth-experiment/1\0` UTF-8 bytes, then a four-byte big-endian canonical payload length, then canonical JSON. The statement must be an object. Object keys sort by UTF-8 byte order, arrays retain order, strings contain Unicode scalar values. Null, absence and empty collections differ. JSON numbers, decoded duplicate keys, lone surrogates and trailing content reject. Exact financial quantities are strings; this generic codec does not validate their financial meaning or reject numeric spelling aliases inside strings. The exporter/Core supplies the financial schema checks.

Input and canonical payload limits: 65,536 bytes, depth 32 and 4,096 values. The complete frame also includes the tag and length header. CLI transport is limited to 65,536 bytes. Optional wallet framing is `midnight_signed_message:<frame byte length>:` followed by the complete frame. Cross-language vectors include astral/BMP key ordering, escapes and UTF-8 length.

The pure owner-intent statement and the stronger complete candidate claim are distinct signed messages. The complete claim commits local snapshot, effects and post-state assertions; it is not mandatory solver-independent intent. Neither is the production `moriarty-intent/3` codec.

## Source families and proof scope

- [Official ledger at 9f9842eb](https://github.com/midnightntwrk/midnight-ledger/tree/9f9842ebed66cdff0f54d3fb09efc6a7cd077ed8): native BIP340/ECDSA/hash, local Merkle state and privileged ledger system-transaction tests.
- [Official ZK at 0ededef0](https://github.com/midnightntwrk/midnight-zk/tree/0ededef0e605701fc5139ebdcf011b11f3d86ba7): separate PLONK/KZG smoke, proofs 0.8.0 / curves 0.3.0. Ledger dependencies resolve proofs 0.7.3 / curves 0.2.1. This is not one integrated released or Preview stack.

The native proof checks only field equality `10000 = 8990 + 1010`, with three public inputs and final pairing verification, k=6 and an ephemeral `unsafe_setup` test SRS. It has no UInt128 range constraints, complete financial semantics, signature circuit, compiler correspondence, recursion or production SRS. The test profile disables upstream midnight-proofs debug assertions: debug builds otherwise panic when an invalid witness reaches the prover's linearization assertion. Using release assertion behavior allows a generated invalid-witness proof to reach and fail the real verifier. No MockProver is used.

The authority map and Merkle root are local fixture assumptions. Actual ledger tests exercise privileged reserve/reward distribution, replay, overdraw and atomic failure on in-memory state; they do not exercise a user financial transaction. [Measured results and remaining gates](../../wiki-llm/midnight-crypto-2026-09-30/RESULT.md) preserve the distinctions and failed runs. Upstream source/license capture and pin receipts are linked there.
