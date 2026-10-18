# Download and reconstruct

The archive payload is prepared for direct publication on the public `utxo` branch of [CharlesHoskinson/Moriarty](https://github.com/CharlesHoskinson/Moriarty/tree/utxo/evidence/utxo/history), with no Git LFS or private storage requirement.

```sh
git clone --branch utxo --single-branch https://github.com/CharlesHoskinson/Moriarty.git
cd Moriarty
git rev-parse HEAD
python3 evidence/utxo/history/reconstruct.py --root validation --path graphify-out/graph.json --output /tmp/utxo-graph.json
python3 evidence/utxo/history/reconstruct.py --verify-all
```

Record the checked-out full commit returned by `git rev-parse HEAD` for a stable publication reference. The manifest paths are relative to their named original `research` or `validation` root. Each archived object is identified by the SHA-256 of its uncompressed original bytes. This permits an independent check against retained source files without trusting pack boundaries.

For a public model-output extraction, use its `public_sha256` from `public-log-extractions.jsonl`:

```sh
python3 evidence/utxo/history/reconstruct.py --sha256 DIGEST --output /tmp/public-model-output.txt
```

Consult `exclusions.jsonl` for unavailable originals. Private source logs, keys and credentials cannot be downloaded. Dependency payloads and executable downloads must be reacquired from the retained pinned dependency/download manifests; they are not replaced by invented source or receipts.

The final publication file inventory is `PUBLICATION-FILES.json`. It pins every file in this package except itself. Verify its own digest against the independent publication receipt or the Git commit. For each supplement, use its own `reconstruct.py --verify-all`; its manifests and chunk ranges are independent of the main generation.
