# Task2 original evidence archive

Repository observation: the byte-only pack and independent part audit both exited0.
No native, Python semantic, or model-checking test was rerun for this packaging.
Root retains source/evidence admission authority.

The archive contains515 unique regular members,783773384 uncompressed member
bytes, and744523649 compressed bytes. It preserves all446 indexed stage files,
29 attachments, author index/report, the receipt-audit script, the five original
root audit/validation files, all30 frozen source files, and two packaging/audit
scripts. Original52 nested source archives retain3650 source members. No original
receipt was reconstructed or overwritten. The manifest, this note and the new
packaging-validation receipt are outer metadata, not self-referential members.

`archive-manifest.json` lists every member's path, size and SHA256 and each of the
15 ordered part hashes. Parts000–013 are52428800 bytes each; part014 is10520449
bytes. Concatenation SHA256:

`c3d2d97103b714c60762853ba087e338aaa901c841e4669e8bccfad3b6868ae1`

The original new tar remains ignored at
`.superpowers/sdd/a5-task2-lossless-archive/original-evidence.tar.gz`.
The committed-sized parts are a lossless split of those bytes, not recompression.

## Read-only verification and recovery

From the repository root:

```sh
/home/charl/Moriarty/.venv/bin/python -B evidence/s02-candidate-a-completion/a5/corpus-task2/audit-archive.py
```

The auditor validates each part and their concatenation, traverses all original
outer members without filesystem extraction, checks the author index/frozen
source bindings, and independently traverses all52 original source tar files
against their actual input.json pins. Its retained terminal output is in
`archive-validation.json`. It needs only the parts and manifest, not ignored
source receipts or the scratch tar.

After verification, recover to a new destination using this command. It refuses
to overwrite an existing recovered archive. Extract only into a separate fresh
directory when replay requires files; do not extract over the working tree.

```sh
/home/charl/Moriarty/.venv/bin/python -B - <<'PY'
import json, shutil
from pathlib import Path
p = Path('evidence/s02-candidate-a-completion/a5/corpus-task2')
m = json.loads((p/'archive-manifest.json').read_text())
with Path('/tmp/a5-task2-recovered-original-evidence.tar.gz').open('xb') as out:
    for part in m['parts']:
        with (p/part['path']).open('rb') as source:
            shutil.copyfileobj(source, out, 1024*1024)
PY
```

## Shared runtime references, not duplicate archives

The315704660-byte shared runtime is already retained in
`evidence/s02-candidate-a-completion/a5/kernel-task1/runtime.tar.gz.part-00`
through`part-06`; its own `audit-parts.py` verifies SHA256
`f0687656d30c0d59ece8dfd027508a9228020506d29b97aea3abe7568186c31c`.

The original reused Python archive is member
`.superpowers/sdd/a4-checker-task1-receipts/python-environment.tar.gz` inside
`evidence/s02-candidate-a-completion/a4/checker-task1/original-receipts.tar.gz`.
Its SHA256 is
`7363d106d464f976af2a0e7f1f56e3c7bed237c7a3396a60d073369add068dac`.
The adjacent original Python manifest is retained there too. Neither shared
archive is copied into this new package. The separate original uv supplement
is included because it belongs to the author's29 indexed attachments.
