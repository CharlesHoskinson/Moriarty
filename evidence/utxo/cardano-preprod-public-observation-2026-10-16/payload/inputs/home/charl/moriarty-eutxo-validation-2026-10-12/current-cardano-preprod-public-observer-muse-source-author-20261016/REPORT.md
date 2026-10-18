# Verdict

**Source-only candidate complete in `files[]` (exactly 5 files); ready for root fresh materialization and a fresh full non-author audit. Not adopted, not authorized, not executed, and not a provider observation or ledger proof.**

The candidate is a minimal standalone one-shot script plus frozen expectations, an execution proposal, a README, and design positions. It would, only if separately adopted, make one batch of at most five sequential unauthenticated HTTPS calls to the fixed host `https://preprod.koios.rest/api/v1` in the order genesis, tip, `POST /address_utxos` (a query, not a mutation), `epoch_params?_epoch_no=<step-2 epoch>`, tip, for the exact public address in `EXPECTATIONS.json`.

# Findings

- Scrapling 0.4.15 defaults are unsafe for this contract: `retries=3`, `follow_redirects="safe"`, `impersonate="chrome"`, and stealth headers adding a Google referer (`static.py:73-84,663-675`). The loop is `max(1, retries or 1)` over `range(max_retries)` (`static.py:231-232,248`), so the candidate pins literal `retries=1` at session and request level (one attempt under both readings, never `0`), plus `follow_redirects=False`, `max_redirects=0`, `verify=True`, `stealthy_headers=False`, and `impersonate=None`.
- The 2 MiB bound is honestly post-buffer: `Response.body` is already-materialized bytes (`custom.py:83-86`), so no streaming/download cap or absolute DNS/wire count inside curl_cffi is claimed, and no unsupported `trust_env`-style argument is used.
- Provider semantics stay provider semantics: genesis is checked against the documented string contract and the captured static Preprod genesis (magic 1/Testnet, 2022-06-01, 432000/1; Preview magic 2 distinguishes it), `value` remains a raw decimal string with no ADA/lovelace conversion, `tx_index` uses the SQL `smallint` bound 0..32767 (`address_utxos.sql:4`), `cost_models` remains opaque, empty UTxO lists are valid provider-reported unfunded, exactly-1000 rows is bounded-set-only, and a changed tip is AMBIGUOUS rather than an authenticated snapshot.

# Scope, limits, and first blocked boundary

Read-only public mirror only; no shell/code execution, network query, keys, signing, proof, or nested agents were used. Unread inputs and unresolved library/environment assumptions (notably curl_cffi environment-proxy behaviour and the unindexed logger implementation) are listed in `coverage` and `unresolvedEvidence`. **First blocked boundary:** the script exits before any network call if the output path already exists or Scrapling is unavailable; before any qualifying use, root fresh materialization and the fresh full non-author audit must complete.

# Independence

This is an independent Muse Wonder 1.0 HIGH source-authorship context under the indexed explicit user fallback for unavailable Opus. It invents no provider identity, approval, or authority, performs no independent approval or execution, and leaves all 20 UTxO families, ACTUS/DeFi/general obligations, recursion, private handoff, bounded multi-parent PCD, universal certificates, and real Preprod acceptance open.

