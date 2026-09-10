# Midnight Preview public readiness — 2026-09-10

Observed at 16:11:50–16:11:52 UTC: the officially documented Preview HTTPS RPC answered both permitted public read-only calls with HTTP 200 and valid JSON-RPC hash results.

- Endpoint: https://rpc.preview.midnight.network/
- Reported genesis: `0x3c096de209e06a1a8c52be7bda109ee8c891a42288c0a9db5495f9616dd13796`
- Reported finalized head: `0xf66a390b75343fddbf20572b65d4fc6dd30c18be56322ea80b38842459ad4984`
- Exactly two RPC requests, zero retries; 15 seconds and 64 KiB maximum per request. Actual requests took 902 ms and 767 ms; both bodies were 102 bytes.

Current primary documentation, retrieved with installed Scrapling 0.4.15, confirms network ID `preview`, HTTPS/WSS node endpoints and `mn_addr_preview` / `mn_shield-addr_preview` / `mn_dust_preview` address prefixes:

- https://docs.midnight.network/nodes/node-endpoints.md — HTTP 200, SHA256 `2afdc6f181875709cbe7f91a912fc3bfcd2eec91546de70be8b2bc21f91e01d0`.
- https://docs.midnight.network/guides/networks-and-environments.md — HTTP 200, SHA256 `ffdd801f94326f644cd1964ab7e16b3ba030c318bbe5d0a8fdd17a49127dcad9`.

The robots.txt precheck returned HTTP 429; its response is retained, with no retry or bypass. Documentation GETs succeeded. No cookies or credentials were persisted, no new scraping pattern was needed, and no skill/package installation occurred.

These observations establish the availability of two RPC methods at the documented endpoint and the identity values that endpoint reported. They do not independently authenticate consensus, establish a block height, protocol version, indexer health, wallet funding/synchronization, financial settlement, or Moriarty execution. Existing historical source snapshots were consulted first; no matching historical genesis record was found in the bounded earlier Preview evidence search.

No wallet/private state/proof/transaction/local service was accessed or run. The mandatory end-to-end compiled Moriarty financial settlement gate on Midnight Preview remains open. See documentation-receipts.json, rpc-receipts.json and individual request/response files for exact public receipts.
