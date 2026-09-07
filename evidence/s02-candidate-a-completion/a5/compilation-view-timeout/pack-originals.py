"""Archive frozen original bytes once, in bounded parts; no native execution."""
import hashlib
import json
import tarfile
import tempfile
from pathlib import Path

ROOT = Path.cwd()
PACKAGE = Path(__file__).resolve().parent
PART_BYTES = 16 * 1024 * 1024

def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

inputs = json.loads((PACKAGE / 'original-inputs.json').read_text())
assert len({row['path'] for row in inputs}) == len(inputs)
assert not (PACKAGE / 'archive-manifest.json').exists()
assert not list(PACKAGE.glob('original-receipts.tar.part-*'))
for row in inputs:
    source = ROOT / row['path']
    assert source.is_file() and not source.is_symlink()
    assert source.stat().st_size == row['bytes'] and sha(source) == row['sha256']
with tempfile.TemporaryFile() as combined:
    with tarfile.open(fileobj=combined, mode='w', format=tarfile.PAX_FORMAT) as archive:
        for row in inputs:
            archive.add(ROOT / row['path'], arcname=row['path'], recursive=False)
    archive_bytes = combined.tell()
    combined.seek(0)
    digest = hashlib.sha256()
    parts = []
    number = 0
    while data := combined.read(PART_BYTES):
        digest.update(data)
        name = f'original-receipts.tar.part-{number:03d}'
        with (PACKAGE / name).open('xb') as stream:
            stream.write(data)
        parts.append({'path': name, 'bytes': len(data), 'sha256': hashlib.sha256(data).hexdigest()})
        number += 1
for row in inputs:
    assert sha(ROOT / row['path']) == row['sha256']
manifest = {'schema': 'moriarty.a5-view-timeout-archive/v1',
            'archiveName': 'original-receipts.tar', 'archiveBytes': archive_bytes,
            'archiveSha256': digest.hexdigest(), 'partLimitBytes': PART_BYTES,
            'parts': parts, 'members': inputs,
            'scope': 'Lossless original source/receipt archive; environment archives remain pinned references'}
with (PACKAGE / 'archive-manifest.json').open('x') as stream:
    json.dump(manifest, stream, indent=2)
    stream.write('\n')
print(json.dumps({'ok': True, 'archiveSha256': digest.hexdigest(),
                  'archiveBytes': archive_bytes, 'parts': len(parts), 'members': len(inputs)}))
