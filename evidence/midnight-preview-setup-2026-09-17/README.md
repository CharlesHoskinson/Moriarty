# Preview setup on WSL — 2026-09-17

## Current result

DUST registration succeeded through HTTP and finalized in canonical block 908091. Transaction identifier: `0081d838d9fb4c9924d99c01e2b32da6638e801ce44fc44ea7e00b989c0edbb6df`. A fresh read-only wallet restore confirmed all three components synchronized, 5,000 tNIGHT retained, positive growing DUST and zero unregistered NIGHT coins. See `dust-registration-finality.json` and `wallet-ready-after-registration.json`. The successful registration helper exited zero.

The earlier WebSocket attempts and expired-transaction rejection below remain historical failures. HTTP exposed node error 182 on the stale transaction; its TTL had expired. A fresh SDK recipe submitted through HTTP succeeded. The underlying WebSocket failure remains separately unresolved; do not attribute it to version incompatibility. Official documentation confirms the installed stack matches the tested Preview matrix.

No contract deployment or Moriarty financial transaction is established by this wallet setup.


## Observed setup

Docker Engine 29.1.3 and Docker Compose were installed from Ubuntu packages.
The [Preview Compose file](../../experiments/moriarty-midnight-network/compose.preview.yml)
runs proof server 8.1.0 at its previously tested SHA-256 image digest, bound to
`127.0.0.1:16300`, limited to 4 CPUs, 8 GiB RAM and 256 processes. Its container
health check and HTTP `/health` passed. HTTP `/version` returned plain text
`8.1.0`; the initial JSON-only probe recorded a parsing failure, not a server
failure. This is an environment observation, not a transaction proof test.

The existing hello-world scaffold compiled with Compact 0.31.1. The installed
Midnight.js 4.1.1, wallet SDK 1.2.0 and runtime dependencies match all 26 file
digests in the financial launch runtime manifest. Public RPC identified itself
as `Midnight Preview`, version `1.0.2-eb71e64e`, with 13 peers and no node sync
in progress. The v4 indexer returned block 907063 during the first probe.

## New wallet

The user explicitly selected a new test wallet. Its
[public receipt](wallet-public.json) records this address:

```text
mn_addr_preview1urkqx6s2c9ff3cdvcm2kcyrxjyq83u6r9aq6vfmtc7av9nkvmt2szhaurc
```

Fund it using <https://faucet.preview.midnight.network/>. The faucet page was
reachable. Complete its normal human interaction in the browser.

The seed was created exclusively outside Git at
`/home/charl/.local/share/moriarty/test-wallets/public-preview-20260917.seed`,
mode 0600, with a separate mode-0700 runtime directory. The live wallet helper
re-derived and checked the address before observing the chain. The historical
September 7 address and receipts remain unchanged; their private files are
absent from this WSL installation. Do not use this new identity as a substitute
for owner keys on historical contracts.

The user's faucet transaction was verified SUCCESS in canonical finalized
block 907092; see [faucet finality](faucet-finality.json). The observer reported
5,000,000,000 smallest NIGHT units (5,000 tNIGHT), and preserved all three
child snapshots with mode 0600. The continuation completed full sync and
stopped with exit 0 at 16:03:36 UTC.

One authorized DUST-registration submission was attempted after full sync.
It returned a submission error (exit 1). Its identifier is
`0074fcd5910e16809368733e60c97aa6e49724747c8231369bcd30f2d754487b69`,
transaction hash
`680156b4b58058f4160d0b8da187e6f17880d04c1e61c90281a690026607c089`.
Finalized bytes and pre-registration snapshots are retained privately. The
estimated DUST fee was 300000000000001 smallest units. The indexer had no
matching transaction and the later public mempool check was empty. A fresh
read-only wallet restore confirmed full sync, unchanged NIGHT, one unregistered
NIGHT coin and zero DUST. Therefore transaction readiness is **not established**.
The initial failure log lacks the nested cause; later logging preserves only
sanitized cause messages. No automatic retry was performed.

DUST registration/generation and a concrete admitted transaction remain
requirements for subsequent contract submission. Funding alone does not establish
financial settlement or mandatory PCD acceptance.

## Reproduce

From `/home/charl/Moriarty`:

```bash
source ~/.config/moriarty/preview.sh
sudo docker compose -f experiments/moriarty-midnight-network/compose.preview.yml up -d --wait
node experiments/moriarty-midnight-network/preview-readiness.mjs
timeout --signal=TERM --kill-after=30s 210s \
  experiments/moriarty-midnight-network/hello-world/node_modules/.bin/tsx \
  experiments/moriarty-midnight-network/preview-observe.mjs
```

The observer performs no chain submission and persists sync state privately.
Read its `stopped` record and subsequent restore result before claiming sync
and persistence. The readiness command exits 1 while any setup check fails;
it reports file presence separately from identity, funding and sync.

Stop this proof server with:

```bash
sudo docker compose -f experiments/moriarty-midnight-network/compose.preview.yml stop
```

The financial launcher's `PINNED_NM` now accepts an absolute
`MORIARTY_MIDNIGHT_NODE_MODULES` override, retaining the historical default.
`preview.sh` selects this checkout's scaffold dependencies. All provider
version/digest checks remain enforced. Actual provider imports and the
26-file launch-runtime check passed; all 24 existing provider tests passed
after their ledger import was routed through the same configured directory.
No fabricated worktree or historical state was created.

## Source scope

The wiki index and existing network evidence were inspected before intake.
[Raw captures and API receipts](../../raw/midnight-preview-setup-2026-09-17/)
include retrieval times, canonical URLs, status and content digests for the
three initial Scrapling captures. The official proof-server guide redirected
to `/guides/run-proof-server`; the old wallet tutorial redirected to the
tutorial index and does not substantiate wallet-specific setup. The official
developer-experience blog explicitly shows proof-server 8.1.0, but is
explanatory material rather than a complete compatibility specification.
Historical upstream version matrices may lag the observed live Preview node.

These are scoped setup observations. Canonical wiki intake remains separate;
no new accepted research claims or product acceptance are asserted here.
