# Midnight network choice review — 2026-09-07

**Preprod is a supported public testnet, but Preview is the documented fit for
Moriarty's current experimental stage. Use local Docker for routine development,
Preview for early shared-network tests, and Preprod for final mainnet validation.**
This is a recommendation based on the published task-to-network mapping. It does
not establish that switching networks will fix faucet acceptance or wallet sync.

## What was read and acquired

A fresh Scrapling crawl acquired all 1,289 official indexed Markdown
pages with HTTP200 and zero page failures; all 1,289 page hashes match the earlier
same-day capture. The corpus search found 72 network-related pages. See
[verification](review-verification.json); `coverage-manifest.json` records each URL, HTTP result, capture time and
hash. The whole corpus is searched for network, faucet and testnet references.
Detailed reading covers all three getting-started guides, networks/environments,
funding, wallet development, deployment/operations, release endpoints and
compatibility, and relevant troubleshooting. Most indexed pages are generated
API references: acquisition/search of all pages is distinct from a line-by-line
technical review of every API entry.

The [focused primary-source capture](../../raw/midnight-network-guidance-2026-09-07/receipts.json)
refreshes the decision-critical pages independently. The seven Markdown guides
are byte-identical to the earlier same-day corpus. Live HTML/robots/sitemap or
marketing-page security checkpoints were retained where encountered; there was
no CAPTCHA or access-control bypass. This is a Markdown/text documentation
corpus, not a complete site asset mirror. Fresh copies supersede no old bytes.

## Getting-started interpretation

| Task | Documented environment | Our use |
| --- | --- | --- |
| Compile, iterate, test and run CI | `undeployed` | Keep the working Docker stack and local contract settlement test. |
| Early experiments on shared public infrastructure | `preview` | Preferred next public integration target for the experimental DSL. |
| Final testing before mainnet | `preprod` | Retain as the release-validation target; it is also a valid public deployment route now. |
| Production | `mainnet` | Outside this test-token task. |

The [quickstart](https://docs.midnight.network/getting-started/quickstart)
defaults to a local node, indexer and proof server with genesis funding. Both
public testnets are explicit options. The [network guide](https://docs.midnight.network/guides/networks-and-environments)
assigns Preview to early development and Preprod to final validation, and retires
`testnet-02`. The [deployment guide](https://docs.midnight.network/guides/deploy-and-operate)
also supports promotion directly from local to Preprod. Thus Preprod was not an
invalid network choice; presenting it as our only development testnet skipped
an important distinction in the current guidance.

## Live endpoint and faucet observations

Observed around 02:59–03:00 UTC. These are point-in-time read-only checks, not a
service-availability guarantee and not proof that a faucet paid tokens.

- Preview RPC returned `Midnight Preview`, version `1.0.1-5edf8ddd`, 13 peers and
  `isSyncing: false`; its indexer served block 755420.
- Preprod RPC returned `Midnight Preprod`, version `1.0.2-eb71e64e`, 12 peers and
  `isSyncing: false`; its indexer served block 2439557.
- These node versions match the network-specific versions in the captured
  compatibility matrix. A node's `isSyncing: false` says nothing about whether
  our separately initialized wallet has finished its history scan.

See [RPC/indexer receipts](../../raw/midnight-network-guidance-2026-09-07/endpoint-receipts.json).

| Faucet endpoint | Observed result | Interpretation |
| --- | --- | --- |
| `midnight-tmnight-preprod.nethermind.dev` | Page HTTP 200; `/api/health` reports `ok`, faucet wallet `ok` | This is the URL linked by the current quickstart/funding/network docs. |
| `midnight-tmnight-preview.nethermind.dev` | HTTP 503, `no healthy upstream` | The docs-linked Preview faucet was unavailable in this check. |
| `faucet.preprod.midnight.network` | Page HTTP 200; `/api/health` reports `SERVING` | This is the endpoint previously given to the user. Page/config identifies Preprod. |
| `faucet.preview.midnight.network` | Page HTTP 200; `/api/health` reports `SERVING` | A live Preview alternative; token delivery remains untested. |

See [faucet health receipts](../../raw/midnight-network-guidance-2026-09-07/faucet-health-receipts.json).
The two Preprod frontends use different APIs: the docs-linked UI submits to
`/api/request-tokens` with address/CAPTCHA fields; the previously supplied UI
uses `/api/drips` with a CAPTCHA header. Their published JavaScript and hashes
are retained in [asset receipts](../../raw/midnight-network-guidance-2026-09-07/faucet-asset-receipts.json).
Do not treat the URLs or API contracts as interchangeable. No token POST was
made during this review. The developer hub also links Preprod in its published
web-search representation; direct Scrapling capture of that marketing page hit
a Vercel checkpoint, so it is not treated as a successful scraped text source.

## What the guides say about our failure modes

The [funding guide](https://docs.midnight.network/guides/acquire-tokens) requires
an **unshielded** Bech32m address for the target network, pasted without leading
or trailing whitespace. NIGHT addresses begin `mn_addr_`; shielded addresses
and DUST addresses are different types. Preprod addresses cannot fund Preview.
Changing the prefix by hand invalidates the checksum; derive through the SDK.

Both previously supplied Preprod addresses passed the installed SDK's network,
type and checksum decoding; the replacement also passed encode/decode roundtrip.
That does not prove server acceptance. The reported invalid-address error has
not been reproduced with a valid human CAPTCHA and its exact backend response.
Generating another address did not establish or fix the cause. Keep both keys
and any funds safe; stop replacing wallets speculatively.

The documented sequence is: receive tNIGHT, observe the unshielded balance,
finish wallet synchronization, register NIGHT for DUST, observe generated DUST,
then submit the contract deployment/call and check accepted finalized blocks.
The funding guide explicitly permits observing confirmed balance while overall
wallet sync is still running; registration requires sync to finish. It warns
that restarting without retained state repeats the initial scan. The deployment
guide says the first Preprod scan takes substantially longer than local.

Our public-wallet helper waits for `isSynced` before printing any balance and
times out after 600 seconds. That makes funding progress invisible and the
observed timeout is not evidence of a broken public chain. Its `nativeToken()`
lookup was checked against `unshieldedToken()` in the installed SDK: both return
the same unshielded zero asset ID, so that alias is not the demonstrated cause.

## Revised operational plan

1. Keep local Docker as the default development and regression environment.
   Keep the native R3 row-limit decision separate; this research authorizes no
   extra proof run or change to k.
2. Prefer Preview for the next fresh public integration experiment. Verify all
   node/indexer/network-ID/address settings together, and use the matrix for
   that network. Preserve Preprod as final validation and as a usable route if
   the existing dedicated Preprod wallet is funded successfully.
3. Before another faucet attempt, use the matching documented faucet and record
   its exact UI/backend outcome. For the existing Preprod wallet, first try the
   docs-linked Nethermind Preprod UI. Do not create another address as a guess.
4. Fix wallet observation before a long sync: print unshielded funding progress
   independently of overall sync, report sub-wallet progress, and retain state
   privately across restarts. Keep a bounded wall-time budget and distinguish
   disconnected, connected, synchronizing, funded, DUST-ready and settled states.
5. Complete a real public contract deployment/call with transaction, block,
   indexer status and canonical finalized-chain evidence. Faucet health, valid
   address encoding and RPC connectivity do not satisfy that acceptance test.

The documentation review itself switched no network, generated no wallet, and
sent no transaction. The user subsequently selected Preview for an actual public
attempt; that execution follows as a separate test. Preprod abandonment remains
unconfirmed because its live RPC/indexer responded during this review.
