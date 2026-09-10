# Finalized node-state reader — source-only check

`captureFinalizedFinancialState({contractAddress,rpc,loadedContract,deadlineMs})` captures public financial state at an explicit node-finalized block. It reads the finalized head/header, checks the canonical hash at that height, invokes `midnight_contractState(address, blockHash)` with the mandatory second argument, and checks canonicality again after decoding. It retains full serialized state hex/SHA256, address, block hash/height, native balances and every generated decoded state field.

The reader uses the existing `beforeDeadline` observation wrapper with one shared deadline capped at60seconds. It makes five read-only RPC calls and never retries or substitutes latest/indexer state. It caps the returned state at1MiB, uses the pinned protocol Compact-runtime ContractState decoder, requires exact native serialization roundtrip and the existing strict unshielded balance projection, and detaches generated public fields from their getter view. The loader and RPC lifecycle remain caller-owned. The caller must supply its verified current build loader; this reader does not independently authenticate loader identity. The real loader decodeState enforces its existing freshness check. A later head may advance; the result stays an as-of observation at its recorded anchor, not a claim that no intervening action occurred afterward.

## Source semantics and limitations

The initial `source-interface-finding.json` remains retained: indexer block offsets select actions IN a block, not state as of that block. The node RPC resolves this interface gap. Public node source `pallets/midnight/rpc/src/lib.rs:32–52` declares `(contract_address:String,at:Option<BlockHash>)`; lines285–318 call the runtime state API at that explicit hash and hex-encode the result. `node/src/openrpc.rs:149–165` documents the same arguments. The unspecified-at best-block default is never used.

No retained SP05 evidence was found for an actual invocation of this RPC. Public repository API source does not establish installed local-service support. Unsupported methods, missing/pruned state and provider errors fail without fallback. The node is trusted for finalized head, canonical mapping and state-at-address; these RPC checks do not authenticate a state proof. Syntactically valid address substitution by an untrusted caller or dishonest RPC remains outside this reader's identity authority; the caller supplies its already-bound contract address. The implementation has no indexer dependency.

This check uses controlled RPC responses containing the8150-byte actual retained initialized swap state, the real pinned native decoder and retained full generated build decoder. It does not start services, access private state, prove, compile, submit or observe live state. Before/after actual adverse-call comparison and ledger nonmutation are not established. The combined candidate includes the narrow RPC transport integration: exact bare64hex address plus explicit0x64hex block parameter, retaining its existing64KiB HTTP body limit (stricter than the reader's1MiB native cap).

## Reproduce

```sh
MORIARTY_SWAP_BUILD_RECEIPT=/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/swap-output/build/build-receipt.json node --test --test-reporter=tap experiments/moriarty-midnight-financial/ledger/finalized-financial-state.test.mjs
```

The environment input is required; tests never skip, install or rebuild. `red.tap` records missing-helper failures before implementation. `green-01.tap` passed16tests. `green-02.tap` passed17tests, adding explicit later-head/unchanged-anchor scope coverage; it also frees a deserialized native object on roundtrip failure. Negative controls cover malformed head/header/height, canonical mismatch before and after state query, missing/unknown/oversized/non-native/noncanonical state, invalid address, expired deadline, unsupported RPC, hanging RPC, absent decoded state and native shielded balances. The positive test compares the complete decoded record and both1M/2Mreserve balances; no expected fixture supplies those decoded fields to the reader.

Independent source/result review remains pending. No campaign allocation or prior source15 file was changed by this slice.

The review found deadline coercion before capping: Infinity/numeric strings could reach RPC. `deadline-red.tap` reproduces it; the original deadline now must be a future safe integer before Math.min. `green-03.tap` runs all reader and RPC transport cases with zero skipped. The original focused red selects only the new deadline control. Root's `rpc-red.tap`/`rpc-green.tap` retain its transport TDD and93integration/regression results.
