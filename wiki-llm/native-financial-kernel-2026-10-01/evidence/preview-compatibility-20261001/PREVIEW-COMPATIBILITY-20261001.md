# Preview compatibility inspection —2026-10-01

## Current observed blocker

Public read-only RPC observation at 2026-10-01T08:10:16.341146+00:00: Preview reports `midnight_ledgerVersion =8.1.2` at finalized block `0xc4e6dd2236a889abe08bfda8ffc4f93e245d930f1840c24a7094c60282386522`, height1103914. The public indexer returned the SAME block hash and protocolVersion1000300. RPC `state_getRuntimeVersion` at that exact hash returned specName midnight, specVersion1000300, transactionVersion3, systemVersion3 and stateVersion3. `midnight_apiVersions` returned [2]. These are separate namespaces: RPC API2 and Substrate transactionVersion3 are not ZKIR2/3 evidence.

The older September17 ledger8 observation is now independently refreshed. The current same-block observation still differs from the candidate's pinned ledger-v9. The candidate's native experiment remains local trusted genesis/undeployed; it does not establish Preview compatibility. No remote candidate transaction/deploy/proof/validation was requested.

## Query provenance and measured predicates

Endpoint `https://rpc.preview.midnight.network/` comes from cached official docs `raw/midnight-docs-2026-09-07/markdown/nodes/node-endpoints.md:136`. Official cached RPC documentation `concepts/network-architecture/rpc-networking.md:82-98` documents midnight_apiVersions and midnight_ledgerVersion(at optional block hash), and rpc_methods discovery. The installed project `ledger/local-tip.mjs:11,26` uses the fixed official Preview indexer and read-only block-offset query. Cached official indexer docs `api-reference/midnight-indexer.md:107-122` document block(offset height) and protocolVersion. Those cached captures already carry original Scrapling acquisition receipts. This task acquired no web article; structured public RPC/GraphQL requests used Python standard-library HTTP with20second per-request bounds and response size bounds.

Measured calls: rpc_methods discovery; chain_getFinalizedHead; midnight_ledgerVersion(finalized hash); chain_getHeader(same hash); state_getRuntimeVersion(same hash), only after discovery showed that method; midnight_apiVersions, only after discovery; indexer block(height) requesting only height/hash/timestamp/protocolVersion. Indexer-to-RPC hash equality was checked. No credential, wallet, authenticated endpoint, state import, contract address or private witness was accessed. Public consensus digest/log payloads were removed from the staged header response because they are unnecessary; the original raw response SHA remains, alongside the redacted response SHA. Other responses contain only public network metadata. RPC methods enumeration includes write method names but none were invoked.

Timestamped redacted request/response records and response hashes are in `preview-compatibility-20261001/`; SOURCE-HASHES.json names exact inspected source identities. MANIFEST.json pins staged evidence. These records are one endpoint observation, not cryptographic endpoint authenticity or a reproducible mapping of the live node executable to a repository commit.

## Source facts for this candidate

`native-ledger-consumer/Cargo.toml:7` pins package midnight-ledger-v9 from official midnight-ledger commit9a8777c4d035fc7f38ae286bcf5f8656668efd9f. Cached package `ledger/Cargo.toml:2` names midnight-ledger-v9. That source's `ledger/src/structure.rs:298,489,519-520` represents ProofVersioned V3 and maps V3 proof to ContractOperationVersion V4, with the V3 proof path requiring op.v3_vk(). Current generated `kernel-prototype/output/contract/index.js:2` requires Compact runtime0.20.0; pay.zkir header explicitly states IR major3/minor1. Those establish the candidate's format/API requirements, not what the live Preview node accepts.

Cached older midnight-ledger trees a01a1ea and9f9842e were inspected narrowly: their ledger manifest depends on older runtime3.1.x/transient2.x/zkir_v2, and the a01a1ea ProofVersioned enum has only V2. Their static version files identify9, so they cannot be relabeled live ledger8.1.2 sources. They do not establish the current node's supported format matrix. No similarly authenticated live8.1.2 build-to-source linkage or official current compatibility declaration was established here.

## Inference, missing evidence and next concrete predicate

Inference: the unchanged Preview ledger8.1.2 versus candidate ledger-v9 distinction remains a concrete blocker to claiming that this candidate is Preview ready. Do not erase it by substituting protocolVersion1000300, RPC API2, a package version, local compilation, or an undeployed native success.

Missing: exact deployed8.1.2 verifier/contract operation format support and its onchain runtime/ZKIR mapping; current officially attested Preview release-to-source linkage; actual permitted candidate compatibility validation; authenticated registry/custody/funding/time and native financial acceptance. The available RPC methods expose ledger version and state metadata, not a documented supported-ZKIR/compiler-version matrix. This inspection does not prove that Preview rejects the V3 candidate; it proves that acceptance is unestablished against a currently different reported ledger generation.

Next bounded evidence needed before a Preview claim: obtain an official8.1.2 deployed release/source compatibility statement or source linkage showing support for the candidate's V3 proof/V4 operation/runtime transcript formats, or record their absence. If unsupported, a separately reviewed compatible encoding/profile decision is needed; this report supplies no new compile or migration authority. Preserve the four-attempt Compact stop and continue the already reviewed local native preparation experiment only within its scope. No fifth compile, new profile, wallet changes, proof/key/SRS work, cargo/node candidate execution or transaction was performed.

Guarded startup status remains SP01.6 loan-swap-subset with unresolved operational history/current-accounting/stale-binding blockers; no pending transactions were returned. No other auditor report was read and no frozen handoff/caller/current resource bytes were changed.
