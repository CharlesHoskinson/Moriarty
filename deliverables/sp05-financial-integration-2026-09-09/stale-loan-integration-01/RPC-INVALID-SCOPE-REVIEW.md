# RPC 1010 scope review

Independent GPT-6 Astra review, September 10, 2026. Read-only source interpretation; no helper change, operational admission, network/service, private input, proof or transaction execution.

**Conclusion:** a future real, exact-byte-bound response through the pinned submission path can be reported as **trusted-node RPC pool rejection (code 1010)**, with the explicit assumption that the observed node uses the documented author-RPC error mapping. This is narrower than a verified statement about the deployed runtime implementation. It can supply the rejection component of the local failed-transaction/no-financial-mutation case when paired with independently checked canonical full before/after state. The source study alone discharges neither component.

## Source facts and locators

The two retained Rust files are at Polkadot SDK revision `660acefe66599a3e54363797007befcb01bd610b`. Their retrieval digests match their actual bytes.

- [RPC base registry](https://github.com/paritytech/polkadot-sdk/blob/660acefe66599a3e54363797007befcb01bd610b/substrate/client/rpc-api/src/error.rs#L21): `AUTHOR = 1000`.
- [Author error registry](https://github.com/paritytech/polkadot-sdk/blob/660acefe66599a3e54363797007befcb01bd610b/substrate/client/rpc-api/src/author/error.rs#L67): `POOL_INVALID_TX = BASE_ERROR + 10`; unknown validity is the next code, 1011, at line 69. Lines 107–121 map both custom and ordinary `PoolError::InvalidTransaction` variants to 1010. Lines 122–128 map `UnknownTransaction` to 1011. Thus 1010 is a pool-invalid classification, not generic inability to determine validity.
- Installed `@polkadot/rpc-provider/coder/index.js:16–20` constructs the actual `RpcError` from a JSON-RPC response's numeric code, message and data. `coder/error.js:24–35` defines that class and retains the numeric code independently of its formatted message.
- Installed `wallet-sdk-node-client/dist/effect/PolkadotNodeClient.js:78–94` catches the transaction-send rejection and creates its own `SubmissionError` with the same serialized transaction bytes and original cause. This route is distinct from `status.isInvalid`, mapped to `TransactionInvalidError` at lines 179–185.
- Installed `wallet-sdk-capabilities/dist/submission/submissionService.js:31,42` wraps the node-client error in one capabilities `SubmissionError`, then applies `Effect.runPromise`. Installed `wallet-sdk-facade/dist/index.js:318–326` rethrows it after wallet revert.
- `experiments/moriarty-midnight-financial/ledger/stale-loan-submission-service.test.mjs:7–17` connects the actual default service and actual Polkadot client to a controlled API that rejects with an actual `RpcError(1010)`. This proves transport/error-shape plumbing under a fixture, not actual node behavior. Corrected helper `stale-loan-rejection.mjs:36–38` currently retains the code and correctly remains `OUTCOME_UNKNOWN` under its frozen reviewed scope.

## Evidence required for the future scoped interpretation

1. Record an actual submission through the admitted production provider path to the identified local node, with retained public transaction bytes/hash and exact equality against the node-client error's `txData`. Preserve the real installed class chain: direct/Effect single failure → one capabilities wrapper → node-client `SubmissionError` → actual `RpcError` with numeric 1010. A hand-constructed error, name/message match, arbitrary nested search or synthetic fixture is not a node observation.
2. Retain the numeric response and its provenance to the submission outcome, node identity/genesis and source/version assumption. A public, bounded raw JSON-RPC error envelope can strengthen transport provenance if available. It is not necessary to publish private SDK error objects or arbitrary error text. Do not infer a decoded subtype from a generic message.
3. For the financial negative predicate, independently verify full canonical before/after contract snapshots, complete native balances including zero entries, and decoded fields. Sample and retain the terminal finalized barrier **after** the observed rejection; require a later canonical after anchor. Keep fee accounting and reservations separate. Code 1010 by itself proves no nonmutation predicate.
4. Review any classifier/label change on its own exact candidate, with a real connected 1010 fixture and controls for 1011, other codes, mismatched bytes, wrong phase, spoofed/extra-nested errors and transport failure. This source interpretation does not amend candidate02 or authorize its execution.

The source-revision-to-running-image relationship remains a disclosed trust boundary. The retrieval record says the node checkout's Cargo.lock selects this revision; this review verified the downloaded source/digests, not the Cargo.lock selection independently and not a reproducible image build. To claim **verified deployed error semantics**, rather than the explicitly scoped trusted-node interpretation above, obtain deployed build/source correspondence or equivalent concrete evidence for that running implementation. That stronger claim is not a newly invented gate for ordinary trusted-node observation.

Do not infer: the particular stale revision was the rejected predicate; execution reached a Compact circuit; transaction inclusion or rollback; a paid or zero fee; absence of every future or intervening inclusion; authenticated state proof; Preview acceptance; deployed bytecode identity. A rejected request may fail for another invalidity such as payment, validity or proof problems. Generic nonmutation evidence cannot be relabeled stale-revision validation without reason-specific evidence.

## Verified digests

All installed paths below are relative to `/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/`.

| Source | SHA-256 |
| --- | --- |
| `/tmp/moriarty-rpc-invalid-source-01/author-error.rs` | `94970a010c4a7dfae558bddd3fe41b0c7034fd016e2becc966a8544505eee185` |
| `/tmp/moriarty-rpc-invalid-source-01/error-base.rs` | `efe3f0cd97b218165abe59372485c0a9b8dbb7ccac3c83bebbc0c5b1c94a1f4c` |
| `@midnight-ntwrk/wallet-sdk-node-client/dist/effect/PolkadotNodeClient.js` | `42409878403cbdc21608c8218ec780170441e06a40775d4a6aa121a85f55b1c6` |
| `@midnight-ntwrk/wallet-sdk-capabilities/dist/submission/submissionService.js` | `08009ac2ae14ede209f098409f7b11f9340dd84031e91b735800dd3aad031ac1` |
| `@midnight-ntwrk/wallet-sdk-facade/dist/index.js` | `bb63f609499432f54b7db424d124d7dc80ad7a466af3c39cf7033f52b33f7562` |
| `@polkadot/rpc-provider/coder/error.js` | `ee59a7433530ef446a19ea0c1f2c0cc44d50293cb25ce25efc5010d6f9c1b711` |
| `@polkadot/rpc-provider/coder/index.js` | `d621b16d7a0d10bb16aba0e1a270c5495f366ab175593071b7de863463acad53` |
| Moriarty `ledger/stale-loan-submission-service.test.mjs` | `270360d70cdfbbc654f55eaa63fa74510a412222c7a7f6f108cd9bbf130a9e79` |
| Moriarty `ledger/stale-loan-rejection.mjs` | `2729cf7aeaa05de8c27e3da79bcbc2e278b4bc7016748d561e2b834352c3a196` |

Candidate02 remains `0561261a5616cdd6190c62d5df39dd8e6a550355e38e26c13510ec4f0e482c5b`. This review does not rewrite its approved UNKNOWN handling for 1010.
