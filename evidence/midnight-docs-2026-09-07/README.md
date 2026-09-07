# Midnight documentation acquisition — 2026-09-07 UTC

This evidence package records a same-host acquisition of every documentation
Markdown route published by Midnight's live `llms.txt` index at retrieval time.
It is source acquisition for Moriarty R3, not evidence that any documented
component was installed, run, compatible with the pinned R3 backend, or accepted
by a ledger.

## Result

- Discovery: 1,289 unique `https://docs.midnight.network/*.md` routes from the
  live `llms.txt`, including narrative, API-reference, SDK, release-note and
  version pages.
- Acquisition: 1,289/1,289 returned HTTP 200; zero page failures and zero
  robots exclusions.
- Representations: the full response bytes are in `raw/.../markdown/`; exact
  UTF-8-decoded textual copies are in `raw/.../text/`; per-page JSON receipts
  record requested URL, final URL, UTC retrieval time, status, response metadata,
  byte count and SHA-256.
- Discovery receipts: live `llms.txt` returned 200. Live `robots.txt` and
  `sitemap.xml` each returned a Vercel 429 security checkpoint after bounded
  retries; those response bodies and receipts are preserved. The crawl used the
  pinned official documentation repository's `static/robots.txt` as an explicit
  fallback (`User-agent: *`, `Allow: /`), from repository commit
  `f1422dafa4241e55fa33541e17bf14d1e9b3a5f8`.

The discovery responses and receipts in this package are the latest capture
from the verification rerun at approximately 2026-09-07T02:17Z. The initial
run's discovery files were overwritten by the original crawler and were not
retained separately. Page bodies remained hash-identical across that rerun.
The crawler now treats every existing raw file as immutable and fails closed on
missing or mismatched cached artifacts; refetching requires a fresh dated raw
directory.

The machine-readable inventory is [coverage-manifest.json](coverage-manifest.json).
Raw source material and receipts are under
[`raw/midnight-docs-2026-09-07`](../../raw/midnight-docs-2026-09-07/).

## R3-relevant moving facts observed

These are source facts from the current published pages and must be cited with
their retrieval time when reused:

- [Compatibility matrix](../../raw/midnight-docs-2026-09-07/markdown/relnotes/support-matrix.md):
  Compact devtools 0.5.1, Compact compiler 0.31.1, Compact runtime 0.16.0,
  Compact JS 2.5.1, Platform JS 2.2.4, on-chain runtime 3.0.0, Wallet SDK
  1.2.0, Midnight.js/testkit-js 4.1.1, DApp Connector API 4.0.1 and proof
  server 8.1.0. It lists Preview node/indexer 1.0.1/4.3.5 and
  Preprod/Mainnet node/indexer 1.0.2/4.3.3-hotfix.
- [Latest stable release](../../raw/midnight-docs-2026-09-07/markdown/relnotes/overview.md)
  identifies Midnight Ledger 8.0 across Preview, Preprod and Mainnet.
- [Installation](../../raw/midnight-docs-2026-09-07/markdown/getting-started/installation.md)
  uses `midnightntwrk/proof-server:8.1.0` and links a Compact 0.2.13 VSIX.
  The latter is an editor extension artifact and should not be confused with
  the 0.31.1 compiler in the compatibility matrix.
- [Networks and environments](../../raw/midnight-docs-2026-09-07/markdown/guides/networks-and-environments.md)
  lists `undeployed`, `preview`, `preprod`, and `mainnet`; says `testnet-02` is
  retired; and gives current node, indexer, local proof-server and faucet URLs.
- [Funding a wallet](../../raw/midnight-docs-2026-09-07/markdown/guides/acquire-tokens.md)
  links the Preview and Preprod tNIGHT faucets, requires an unshielded address,
  then registers NIGHT for DUST generation. It documents 1,000 tNIGHT per
  Preprod request at retrieval time.
- [DUST architecture](../../raw/midnight-docs-2026-09-07/markdown/concepts/dust-architecture.md)
  describes the commitment/nullifier design, retroactive first-registration fee
  bootstrap, an initial cap of 5 DUST per NIGHT, and 10^15 Specks per DUST.

## Scope and limitations

The acquisition is restricted to the official documentation host and the URLs
published in its live index. Linked GitHub repositories, faucets, explorers,
blogs, forum pages, images, downloadable files and other external assets were
not crawled. No authentication, wallet, cookie, Docker action, CAPTCHA solution,
or browser challenge bypass was used.

Because the live sitemap was blocked, this package cannot independently compare
the live sitemap URL set with `llms.txt`. The live index itself contains the
generated API and release/version documentation requested for this acquisition.
HTML page routes hit the same Vercel security checkpoint and were not bypassed;
the package therefore preserves the site's complete published Markdown and text
representations, plus the checkpoint HTML responses, rather than claiming a
complete HTML mirror.

## Reproduction

```sh
/home/charl/Moriarty/.venv/bin/python \
  experiments/moriarty-native-ivc-r3/scripts/midnight_docs_acquire.py \
  --raw-dir raw/midnight-docs-2026-09-07 \
  --evidence-dir evidence/midnight-docs-2026-09-07 \
  --robots-fallback /home/charl/Moriarty/repos/midnightntwrk/midnight-docs/static/robots.txt \
  --workers 2 --delay 0.5
```

The script verifies response hashes and exact text copies before reusing an
existing capture. A cached rerun performs no network requests and never rewrites
raw responses or receipts. It has no page-count cap. Scrapling 0.4.15 performed
every acquisition request.
