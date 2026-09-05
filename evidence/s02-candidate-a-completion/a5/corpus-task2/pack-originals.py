"""Package retained originals only. Run from repository root; exclusive outputs."""
import hashlib
import json
import tarfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
DEST = Path(__file__).resolve().parent
RECEIPTS = ROOT / '.superpowers/sdd/a5-factoring-receipts'
INDEX = RECEIPTS / 'task2-terminal-index.json'
SCRATCH = ROOT / '.superpowers/sdd/a5-task2-lossless-archive'
LIMIT = 50 * 1024 * 1024


def digest(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def main():
    index = json.loads(INDEX.read_text())
    selected = {}
    for entry in [e for s in index['stages'] for e in s['files']] + index['attachments']:
        path = ROOT / entry['path']
        assert path.is_file() and not path.is_symlink()
        assert digest(path) == entry['sha256'] and path.stat().st_size == entry['bytes']
        selected[entry['path']] = path
    assert sum(len(s['files']) for s in index['stages']) == 446
    assert len(index['attachments']) == 29
    extras = [INDEX, ROOT / '.superpowers/sdd/a5-corpus-task2-report.md',
              ROOT / '.superpowers/sdd/a5-corpus-receipt-audit.py']
    extras += [DEST / n for n in ('audit.py', 'validation.json', 'source-validation.json',
               'red-validation.json', 'uv-validation.json', 'pack-originals.py', 'audit-archive.py')]
    for entry in index['frozenSources']['files']:
        path = ROOT / entry['path']
        assert digest(path) == entry['sha256']
        extras.append(path)
    for path in extras:
        assert path.is_file() and not path.is_symlink()
        selected[str(path.relative_to(ROOT))] = path
    members = [dict(path=name, bytes=path.stat().st_size, sha256=digest(path))
               for name, path in sorted(selected.items())]
    SCRATCH.mkdir(exist_ok=False)
    archive = SCRATCH / 'original-evidence.tar.gz'
    with tarfile.open(archive, 'x:gz') as out:
        for entry in members:
            path = selected[entry['path']]
            out.add(path, arcname=entry['path'], recursive=False)
            assert digest(path) == entry['sha256']
    parts = []
    with archive.open('rb') as source:
        while chunk := source.read(LIMIT):
            name = f'original-evidence.tar.gz.part-{len(parts):03d}'
            with (DEST / name).open('xb') as target:
                target.write(chunk)
            parts.append(dict(path=name, bytes=len(chunk), sha256=hashlib.sha256(chunk).hexdigest()))
    result = dict(schema='moriarty.a5-task2-lossless-archive/v1',
                  scope='Original-byte packaging only; no new semantic or native execution claim',
                  memberCount=len(members), memberBytes=sum(e['bytes'] for e in members),
                  archiveBytes=archive.stat().st_size, archiveSha256=digest(archive),
                  originalArchive=str(archive.relative_to(ROOT)), members=members, parts=parts,
                  sharedRuntimeReferences={
                      'runtimeParts': 'evidence/s02-candidate-a-completion/a5/kernel-task1/runtime.tar.gz.part-00..06',
                      'runtimeArchiveSha256': 'f0687656d30c0d59ece8dfd027508a9228020506d29b97aea3abe7568186c31c',
                      'pythonOriginal': '.superpowers/sdd/a4-checker-task1-receipts/python-environment.tar.gz',
                      'pythonArchiveSha256': '7363d106d464f976af2a0e7f1f56e3c7bed237c7a3396a60d073369add068dac',
                      'pythonDurableLocation': 'A4 checker Task1 original evidence archive; not duplicated'})
    with (DEST / 'archive-manifest.json').open('x') as out:
        json.dump(result, out, indent=2)
        out.write('\n')
    print(json.dumps({k: result[k] for k in ('memberCount', 'memberBytes', 'archiveBytes', 'archiveSha256', 'parts')}, indent=2))


if __name__ == '__main__':
    main()
