# Preview read-only RPC source increment

Worktree `/home/charl/Moriarty/.worktrees/sp05-preview-owner`, base68c6d9a. Three files in `/tmp/moriarty-preview-rpc-source.json`: new `ledger/financial-rpc.mjs`, new `ledger/preview-rpc.test.mjs`, existing `ledger/integrate-local.mjs`. Four frozen owner candidate source hashes verified unchanged. No publication, resources or operational admission.

Moved existing `createLocalRpc` implementation into one private shared request core with separate exported local/Preview factories. Integration imports/reexports the same local API. Its loopback HTTP endpoint policy and read method/parameter validation remain intact. Preview factory accepts only the exact configured HTTPS origin `https://rpc.preview.midnight.network` with optional trailing slash, rejecting credentials, ports, other hosts, path/query/fragment and noncanonical spelling. No fake localhost forwarding.

Both factories preserve four read methods: finalized head, exact header, canonical block hash (including genesis), and explicit block-bound midnight_contractState. Requests use POST, redirect:error, increasing request IDs, a shared absolute deadline, abort cleanup and64KiB streamed response limit. JSON-RPC errors reject RPC_RESPONSE with no retry or fallback. They do not establish unsupported method classification. Added post-await checks immediately after fetch and each body read, retaining the previous final deadline check to prevent late success when an overdue timer cannot run.

Endpoint shape is sourced from `experiments/moriarty-midnight-network/preview-verify.mjs:16`. Its current availability, TLS/DNS deployment identity, genesis/protocol and financial readback are unobserved by this source task. Explicit target validation is not proof of chain identity. Caller integration must supply per-request capped deadlines and separately check canonical state semantics through the existing reader/observer.

Test-first RED: new factory absent (10 failures). GREEN:104pass, zero failure/skips. Controlled fetch only; no HTTP/network/services/private inputs/proofs/transactions. Command from repository root:

```
node --experimental-test-module-mocks --test experiments/moriarty-midnight-financial/ledger/preview-rpc.test.mjs experiments/moriarty-midnight-financial/ledger/finalized-state-rpc.test.mjs experiments/moriarty-midnight-financial/ledger/integrate-local.test.mjs experiments/moriarty-midnight-financial/ledger/initialized-swap-integration.test.mjs
```

Raw logs `/tmp/moriarty-preview-rpc-red.tap` and `-green.tap`; `git diff --check` passed. Controls cover target, write/malformed params, invalid deadline, HTTP/JSON/envelope errors, oversized body, late fetch/body, pending transport timeout and unchanged local integration tests. Source ready for independent review; no Preview acceptance.
