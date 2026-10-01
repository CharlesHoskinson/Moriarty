# Fixed beta fee-transfer kernel: actual local result

## Decision

The scoped source/runtime route works on isolated Compact 0.35.0, language 0.27, runtime 0.20 (ledger9 family). The actual existing public beta wallet signature is accepted against its exact production 1278-byte signing message. The impure entrypoint produces the full fixed Core5 numerical transition and two native unshielded escrow spend claims. This is generated source, ZKIR and public runtime evaluation, not a proof, ledger acceptance, wallet debit or Preview settlement. No host success flag grants acceptance.

## Exact artifacts and commands

`generate.mjs` consumes checked-in transfer-ecdsa-wallet program/scenario/signature through actual production Rust intent-build, intent-frame, intent-verify and beta verify-intent. `projection.json` preserves those outputs. Regenerating reproduces exactly the compiled source SHA256 `a3205e5bed3aabcbf06e7293be3384724498b7fbf1ae15e5c62a4c36486199db`.

Source SHA256: fb507bfb848a8859a55c020faf410e52f9e40c00fc3938a90ea3c763e1cf1d6c. Owner program SHA256: fabc99c8650814a483ea6af5172b59e5aad2c76fc64d8c26139b834696cc8035. Exact signing-message SHA256: e59e6a4d1d480387c206088663f6611d29dff98d1ac99dadfe2c582b27cc1a48.

Four permitted compiler attempts are exhausted. Attempts 1 and 2 retain actual errors (scalar zero cast; disclosure requirement). Attempts 3 and 4 compile successfully. Final command/exit/source identity in compile-4.json: compactc --skip-zk --feature-zkir-v3 fixed-transfer.compact output; exit0, 0.542 seconds. No fifth attempt, key generation, SRS or proof occurred. Compiler0.35 binary SHA256 is 5f7e2f0e2da785f1f88d231c8d78a4f98278ddf370deac9de805b2c0e267c910; upstream commit debb05f9414b9d1e176741c2be289bb32233f0fc. Original0.31 toolchain is untouched.

Actual runtime command: node test.mjs. Final result: 29 predicate checks pass and one explicitly observed durable race gap. runtime-final.txt and runtime-result.json retain results. Previous red compile/runtime receipts remain. Combined external compiler-probe plus kernel footprint is approximately107.4MB, below256MiB.

## Source predicates

Exact fixed message equality and source SHA256 hashing bind the accepted signature to actual production bytes. Configured public point X and Y match the canonical compressed SEC1 artifact key; nonidentity and parity are enforced. Native ECDSA verifies the source-computed SHA256. Signature r/s are nonzero, scalar decoding rejects out-of-range values, and full low-S compares both128-bit halves of the runtime little-endian32-byte scalar encoding. No Uint256 or truncated Field comparison is used. Boundary tests cross2^128 and the exact curve half-order.

Constructor initializes a single state copy with configured key, fixed asset and distinct recipient/fee mappings, nonce/head/revision, trusted round, balances and counters. pay requires unused nonce and initial head/revision, window0..10, sufficient work/allowance/balance and matching native escrow balance. Checked UInt128 additions enforce recipient/fee/counter overflow limits and total allowance/work bounds. It transfers1000+10; resulting owner8990, recipient1000, fee10, allowance remaining8990/spent1010, work remaining9/spent1, consumed nonce, revision1. Successor head is source-computed from predecessor, signed digest, source/program identity, mappings and resulting financial state.

Native effects have exactly two claimedUnshieldedSpends for fixed asset a1*32 to recipient02*32 amount1000 and fee03*32 amount10, output debit1010, no additional input/mint/shielded effect. Native effect maps have runtime ordering, not source effect ordering. Numeric effect fields separately retain the fixed Core sequence. The initial funding map is explicitly fixture custody state; the runtime does not settle the claims into real recipient wallets.

Tests reject changed frame, wrong point parity, identity, zero/out-of-range/highS signatures, incorrect mappings, funding/allowance/work/window failures, recipient/fee/counter overflow, and replay against returned post-state. Failure tests preserve the original native serialized snapshot. Two evaluations from the same predecessor both produce candidate transitions: runtime evaluation does not establish durable exclusive acceptance or chain replay protection.

## Remaining prerequisites

1. Constructor trust is material: source Owner/account-domain identity is mapped by this prototype to the signed key and fixture escrow/address/color, not authenticated by a registry or deployed owner onboarding. Addresses02/03 and color a1 are public fixture encodings, not existing wallet addresses or actual asset identity. Existing wallets are untouched.
2. This is source specialization for one exact signed program/frame. The generator checks source correspondence using production Rust/beta; there is no general on-chain source parser, compiler correctness proof or generic frame decoder. The fixed accepted frame and fixed financial constants must be independently reviewed together.
3. Initial h0 is a local trusted label-to-digest mapping, not an authenticated chain head/history root. Round is constructor-trusted, not chain time. Durable nonce/head conflict handling requires actual ledger verification/commit.
4. metadata marks pay pure:false/proof:true and emits pay.zkir; this describes a proof-bearing circuit API, not an existing proof. Hash/ECDSA/financial relation proving size, setup cost, generated verifier correctness and native acceptance are unmeasured. Pure helper evaluation is public host execution.
5. Preview ledger8 compatibility blocks using this ledger9-family contract directly. Preserve pinned existing Preview consumers; require independently evidenced compatible compiler/runtime/node contract pipeline or separately scoped migration. Public node/indexer version1000300 observations are not settlement.
6. Real custody funding, native asset/color, recipient addresses, DUST/round/transaction fees, balancing residual state, proof finalization, registration and dispatch require exact resource/debit identities and existing transaction gates. Off-chain beta signData verification remains distinct from native ledger transaction signatures.

Recommended continuation: independently review this exact fixed source/projection/generated ZKIR and constructor trust model, then identify a compatible chain proof/transaction route before any separate resource proposal. Do not close the whole signed-intent-to-settlement goal from these local results. Repository candidate remains unchanged.
