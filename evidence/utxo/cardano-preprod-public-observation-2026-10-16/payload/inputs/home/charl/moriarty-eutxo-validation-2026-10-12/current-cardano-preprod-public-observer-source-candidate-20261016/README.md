# Cardano Preprod public observer — source candidate (source-only)

Status: **source-only candidate for fresh non-author audit. Not adopted, not authorized, and not executed here.** Authorship is under the indexed explicit user fallback for unavailable Opus; no provider identity, independent approval, or on-chain AUTH is claimed.

## What it would do, if root separately adopts it

`runner/observe.py` makes exactly five sequential unauthenticated HTTPS calls through the installed Scrapling `FetcherSession` (curl_cffi engine), one attempt each, no paging and no batch retry:

1. `GET /genesis`
2. `GET /tip`
3. `POST /address_utxos` with `{"_addresses":["addr_test1vzt2n53u2vlr8ggxz8xan6ywrw8m55umfwvrgmzh3wmcmkcfxunjc"],"_extended":true}` (a provider query, not a ledger mutation)
4. `GET /epoch_params?_epoch_no=<epoch_no from step 2>` (explicit; the all-epochs default is never used)
5. `GET /tip`

Host is fixed to `https://preprod.koios.rest/api/v1`. No faucet, submit, signing, deployment, other URLs, link following, JS/browser fetchers, auth, cookies, tokens, wallet keys, or secret reads.

## Key source-derived controls

- Installed Scrapling 0.4.15 defaults to `retries=3`, `follow_redirects="safe"`, `impersonate="chrome"`, and stealth headers with a Google referer. The candidate overrides all of them: `retries=1` (one attempt in the inspected `max(1, retries or 1)` / `range(max_retries)` loop; `0` is never used), `follow_redirects=False`, `max_redirects=0`, `verify=True`, `stealthy_headers=False`, `impersonate=None`, 15 s per-request timeout.
- Only status 200 with an exact response-URL match and `application/json` is accepted. Raw bytes are appended before validation; partial failures are preserved in `RESULT.json` and never accepted. Duplicate JSON keys and NaN/Infinity are rejected.
- Each body is checked at 2 MiB before persist/JSON. Honest limit: Scrapling buffers the whole body first, so this is a post-buffer bound, not a streaming cap. Controller-level five calls do not prove an absolute DNS/wire count inside curl_cffi, and no unsupported `trust_env`-style argument is used.
- Provider genesis is checked against the captured public Preprod genesis (magic `1`/`Testnet`, start 2022-06-01, epoch 432000, slot 1). UTxO `value` stays a raw decimal string (no ADA/lovelace conversion), `tx_index` is bounded by the SQL `smallint` source (0..32767), `cost_models` stays opaque, and an empty UTxO list is a valid provider-reported unfunded observation. Funding language is provider-report only, never a verified balance.
- Same before/after tip means provider bracket only; any advance or epoch cross is `AMBIGUOUS`, never an atomic/authenticated snapshot, and is never silently retried.

## Boundaries

First blocked boundary: the script exits before any network call if the output directory already exists or Scrapling is unavailable. Proposed run paths, containment, and adoption steps are in `EXECUTION-PROPOSAL.json`; frozen invariants are in `EXPECTATIONS.json`; consequential choices are in `DESIGN-POSITIONS.json`. Root must materialize the source fresh, obtain a fresh full non-author audit, and separately review one public read-only run before any observation can qualify.

All 20 UTxO families, ACTUS/DeFi/general obligations, recursion, private handoff, bounded multi-parent PCD, universal certificates, and real Preprod target acceptance remain open.
