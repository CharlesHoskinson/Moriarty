# Research publication

The original [research index](INDEX.md), [source inventory](SOURCE-INVENTORY.md), ten focused `REPORT.md` files and `sources.json` manifests, and official documentation indexes are readable here. Their bytes and historical evidence boundaries are preserved. A downloaded source is not an implementation approval.

Repository snapshots, downloaded papers, documentation captures, historical graph variants, and other selected public originals are stored in the [history archive](../../evidence/utxo/history/README.md). Its `paths.jsonl` maps each original path under the `research` or `validation` root to a SHA-256 object. `exclusions.jsonl` records withheld material and its reason. Strict public model-text extractions have their own manifest.

Some links in the preserved historical reports point to compressed originals. Restore the original research directory structure to make those links available locally. For a single original path, from the repository root:

```sh
python3 evidence/utxo/history/reconstruct.py --root research --path 01-foundations/REPORT.md --output /tmp/moriarty-research/01-foundations/REPORT.md
```

Choose other exact original paths from `paths.jsonl`; use the reconstruction README for SHA-based restoration and complete integrity verification. Original Git internals, dependency caches, runtime downloads, private keys, credentials, and raw private model events are withheld. Public source/license files and pinned provenance are retained where safe.

The history archive is being filtered and verified separately from these readable documents. [Current implementation evidence](../../evidence/utxo/current-implementation-417/REPORT.md) records the current candidate's limits.
