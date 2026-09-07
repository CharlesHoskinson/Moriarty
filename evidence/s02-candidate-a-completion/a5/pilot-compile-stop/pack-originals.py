"""Archive the stopped pilot's original bytes; no native invocation or repair."""
import hashlib
import json
from pathlib import Path
import tarfile

ROOT = Path.cwd()
HERE = ROOT / 'evidence/s02-candidate-a-completion/a5/pilot-compile-stop'
R = ROOT / '.superpowers/sdd/a5-factoring-receipts'
def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

index = json.loads((R/'task3-compile-gate-index.json').read_text())
indexed = list(index['attachments']) + index['pilotSources']
for stage in index['stages']:
    indexed.extend(stage['files'] + stage['bindingFiles'])
selected = {}
for entry in indexed:
    path = ROOT/entry['path']
    assert path.is_file() and not path.is_symlink()
    assert sha(path) == entry['sha256'] and path.stat().st_size == entry['bytes']
    selected[entry['path']] = path
extras = [R/'task3-compile-gate-index.json',ROOT/'.superpowers/sdd/a5-task3-compile-gate-report.md',
          ROOT/'.superpowers/sdd/a5-task3-independent-source-review.md']
extras.extend(HERE/name for name in ('audit.py','source-audit.py','validation.json','source-validation.json',
                                    'pack-originals.py'))
for path in extras:
    assert path.is_file() and not path.is_symlink()
    selected[str(path.relative_to(ROOT))] = path
entries = [dict(path=name,bytes=path.stat().st_size,sha256=sha(path)) for name,path in sorted(selected.items())]
archive_path = HERE/'original-receipts.tar.gz'
with tarfile.open(archive_path, 'x:gz') as archive:
    for entry in entries:
        archive.add(selected[entry['path']],arcname=entry['path'],recursive=False)
        assert sha(selected[entry['path']]) == entry['sha256']
with tarfile.open(archive_path) as archive:
    members = archive.getmembers()
    assert len(members) == len(entries)
    expected = {e['path']:e for e in entries}
    assert {m.name for m in members} == set(expected)
    for member in members:
        entry = expected[member.name]
        assert member.isfile() and member.size == entry['bytes']
        assert hashlib.file_digest(archive.extractfile(member),'sha256').hexdigest() == entry['sha256']
result = {'ok':True,'scope':'Original stopped pilot evidence only; no size baseline or checker pass',
          'archiveSha256':sha(archive_path),'archiveBytes':archive_path.stat().st_size,
          'memberCount':len(entries),'members':entries,
          'sharedRuntimeArchiveSha256':'f0687656d30c0d59ece8dfd027508a9228020506d29b97aea3abe7568186c31c',
          'sharedPythonArchiveSha256':'7363d106d464f976af2a0e7f1f56e3c7bed237c7a3396a60d073369add068dac'}
with (HERE/'archive-validation.json').open('x') as out:
    json.dump(result,out,indent=2)
    out.write('\n')
print(json.dumps({k:v for k,v in result.items() if k!='members'},indent=2))
