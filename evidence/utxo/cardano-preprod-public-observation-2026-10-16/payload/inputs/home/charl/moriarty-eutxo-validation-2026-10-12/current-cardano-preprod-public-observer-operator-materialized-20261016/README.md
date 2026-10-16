# Cardano Preprod public observer — R2 source repair candidate (source-only)

Status: **source-only R2 repair candidate for a fresh full non-author audit. Not adopted, not authorized, and not executed here.** Authorship is Muse Wonder 1.0 HIGH acting as source repair author (not independent reviewer) under the indexed explicit user fallback for unavailable Opus; no provider identity, independent approval, or on-chain AUTH is claimed. R1 and the failed reviews are preserved; root materializes this as a new R2 sibling and obtains a fresh full independent review, not a patch-only review. Actual provider data is still **NONE**.

## What it would do, if root separately adopts it

`runner/observe.py` makes exactly five sequential unauthenticated HTTPS calls through the installed Scrapling `FetcherSession` (curl_cffi engine), one attempt each, no paging and no batch retry:

1. `GET /genesis`
2. `GET /tip`
3. `POST /address_utxos` with `{"_addresses":["addr_test1vzt2n53u2vlr8ggxz8xan6ywrw8m55umfwvrgmzh3wmcmkcfxunjc"],"_extended":true}` (a provider query, not a ledger mutation)
4. `GET /epoch_params?_epoch_no=<epoch_no from step 2>` (explicit; the all-epochs default is never used)
5. `GET /tip`

Host is fixed to `https://preprod.koios.rest/api/v1`. No faucet, submit, signing, deployment, other URLs, link following, JS/browser fetchers, explicit auth, tokens, wallet keys, secret reads, or key generation.

## Repairs relative to the fresh strict audit (F1–F6)

- **F1 cookies.** Every `get`/`post` now passes `discard_cookies=True`, which the indexed Scrapling `_merge_request_args` forwards (non-skip, non-None kwargs) to the curl_cffi `Session.request` parameter of the same name. Each call starts from the one fresh session without persistent cookie storage, so a cookie cannot be stored in the persistent jar or reused on later calls. A `Set-Cookie` may still arrive in the ephemeral response object; no claim of "no Set-Cookie received" is made.
- **F2 environment honesty.** The source cannot disable `trust_env` at the `FetcherSession` constructor (no such supported argument, and no invented argument is passed), and it passes no explicit proxy or auth. Freedom from proxy/CA environment effects is therefore **not** a code guarantee: root must launch with the exact clean environment dict in `EXECUTION-PROPOSAL.json` (not an inherited environment), containing no proxy, CA-bundle, or credential entries at all, with the actual observed interpreter path pinned by root (never `-I`, which would ignore the explicit `PYTHONPATH`).
- **F3 completeness.** The allowlisted `Content-Range` is passed into UTxO validation. Only an exactly-covering range (or `*/0` with zero rows) proves completeness; partial (`0-2/3` short), unknown-total (`0-2/*`, `*/*`), absent, malformed, or row-count-mismatched ranges are flagged bounded/incomplete or completeness-unknown, and exactly-1000 rows still carries a possible-truncation flag. An empty result without proven completeness is `provider_reports_empty_bounded_result_completeness_unknown`, never valid-unfunded; a proven-complete empty result remains a provider-only report, not ledger truth. No paging, retry, extra request, or Prefer header was added.
- **F4 partial preservation.** The STARTED `RESULT.json` is saved immediately and again after each ledger append, before the request; `transport_attempted` and `response_received` are recorded distinctly. `Abort` keeps its source-known message, and any other exception (including session-enter failure) finalizes `BLOCKED_PARTIAL_PRESERVED` with the exception type name only, preserving raw files and the ledger. Preservation is best effort: a physical write failure can prevent the final receipt, in which case the root outer captured terminal output is the authority — there is no universal preservation guarantee. Output is never overwritten; root passes the exact fresh `O`.
- **F5 finite JSON.** Strict parsing now rejects duplicate keys, literal `NaN`/`Infinity`, and numeric overflow such as `1e999` through a finite `parse_float` check, and serialization uses `allow_nan=False`. Raw actual bytes are retained; `cost_models` semantics stay opaque and no fee is ever calculated.
- **F6 bounded receipt.** `FSIZE` 8 MiB is per-file, not aggregate. Full raw bodies remain exact separate step files, while `RESULT.json` carries only whitelisted known-field summaries plus raw-file references (`cost_models` as type/shape unknown plus its raw reference, with no invented decoder), with a bounded compact fallback if even that summary exceeded the receipt guard. No raw observation is silently discarded.

## Key source-derived controls (unchanged from R1)

- Installed Scrapling defaults to `retries=3`, `follow_redirects="safe"`, `impersonate="chrome"`, and stealth headers with a Google referer. The candidate overrides all of them: `retries=1` (one attempt in the inspected `max(1, retries or 1)` / `range(max_retries)` loop; `0` is never used), `follow_redirects=False`, `max_redirects=0`, `verify=True`, `stealthy_headers=False`, `impersonate=None`, 15 s per-request timeout, logging disabled.
- Only status 200 with an exact response-URL match and `application/json` is accepted. Raw bytes are appended before validation; partial failures are preserved best-effort in `RESULT.json` and never accepted.
- Each body is checked at 2 MiB before persist/JSON. Honest limit: Scrapling buffers the whole body first, so this is a post-buffer bound, not a streaming cap, and there is no stream cap. Controller-level five calls do not prove an absolute DNS/wire count inside curl_cffi.
- Provider genesis is checked against the captured public Preprod genesis (magic `1`/`Testnet`, start 2022-06-01, epoch 432000, slot 1). UTxO `value` stays a raw decimal string (no ADA/lovelace conversion, no total balance), `tx_index` is bounded by the SQL `smallint` source (0..32767), and funding language is provider-report only, never a verified balance.
- Same before/after tip means provider bracket only; any advance or epoch cross is `AMBIGUOUS`, never an atomic/authenticated snapshot, and is never silently retried.

## Boundaries

First blocked boundary: adoption is blocked until a fresh full non-author review of the materialized R2 agrees; the script itself exits before any network call if the output directory already exists or Scrapling is unavailable. Proposed run paths, the exact clean environment, containment (outer 120 s + 10 s, CPU 15:20, address space 1 GiB, CORE 0, FSIZE 8 MiB per file), and adoption steps are in `EXECUTION-PROPOSAL.json`; frozen invariants are in `EXPECTATIONS.json`; author FOR positions are in `DESIGN-POSITIONS.json`. Two agreeing positions (author FOR plus a fresh non-author position) may adopt the SOURCE only; they are not two fresh full audits and grant no execution authority.

All 20 UTxO families, ACTUS/DeFi/general obligations, recursion, private handoff, bounded multi-parent PCD, universal certificates, and real Preprod target acceptance remain open.
