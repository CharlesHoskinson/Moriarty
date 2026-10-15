# Reconstruct historical evidence

Each archive generation is self-contained: its `paths.jsonl` maps original root/path to SHA-256 and byte count; `objects.jsonl` maps each SHA-256 to gzip ranges in the numbered blob chunks. Identical bytes share an object. Chunk boundaries do not change reconstructed bytes.

Use Python 3 with its standard library. Run from any directory:

```sh
python3 evidence/utxo/history/reconstruct.py --verify-all
python3 evidence/utxo/history/reconstruct.py --root validation --path graphify-out/graph.json --output /tmp/moriarty-historical-graph.json
```

The historical canonical graph is 332,866,013 bytes, SHA-256 `e52c2768c5b357ea433a4d0b0f6280f416652780da56c66d712f36acc0bca934`. The output is deliberately outside this repository because the reconstructed file exceeds GitHub's ordinary file limit.

For a supplement, run the same commands against that supplement's own `reconstruct.py`. Its root/path namespace is local to its own manifests. Source restoration retains the original research/validation paths and adds independently pinned acquisition provenance; it does not alter the main archive's original exclusion ledger.

The reconstructor verifies SHA-256 and byte counts before atomically installing an output and refuses unsafe output paths. `--verify-all` checks all reachable objects, manifests and blob ranges, including orphan/trailing data. It does not establish source, native Cardano, security or proof-system acceptance.

Original private captures, signing material, dependency/runtime caches and other withheld bytes are not reconstructible from this publication. Their safe omission/provenance metadata is retained where available. Public text extracted from provider captures has its own source-log SHA and extraction rule; it is not the original raw provider stream.

The publication tools reproduce inspection and packing only when their original local source roots are available. Those roots include deliberately withheld material and are not prerequisites for verifying or reconstructing the published archive.
