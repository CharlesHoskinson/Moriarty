"""Package admitted retained17 small evidence once; no raw data replay/hash."""
import hashlib
import io
import json
from pathlib import Path
import tarfile


OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[3]
SDD = ROOT / '.superpowers/sdd'
RESUMPTION = SDD / 'a4-task6-resumption-20260906'
LIFECYCLE = SDD / 'a4-task6-lifecycle-controls-20260906'


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def main():
    if (RESUMPTION / 'fresh97').exists():
        raise RuntimeError('This package is strictly before fresh97; new scope requires review')
    paths = set()
    for folder in (RESUMPTION, LIFECYCLE):
        for path in folder.rglob('*'):
            if path.is_symlink():
                raise RuntimeError('Unexpected small-receipt symlink: ' + str(path))
            if path.is_file():
                paths.add(path)
    for name in ('a4-task6-recovery-20260906.json', 'a4-task6-recovery-20260906.md',
                 'a4-task6-resumption-plan-20260906.md', 'a4-task6-resumption-review-20260906.md',
                 'a4-task6-resumption-recorder-review-20260906.md',
                 'a4-task6-lifecycle-controls-review-20260906.md',
                 'a4-task6-admit17-review-20260906.md'):
        paths.add(SDD / name)
    original = SDD / 'a4-task6-receipts/green-tests'
    for name in ('input.json', 'source.tar.gz', 'stdout.txt', 'stderr.txt'):
        paths.add(original / name)
    for index in range(18):
        folder = original / 'basetemp' / ('test_semantic_rebound_triples_' + str(index))
        for name in ('argv.json', 'stdout.txt', 'stderr.txt', 'terminal.json'):
            path = folder / name
            if path.exists():
                paths.add(path)
    paths.add(SDD / 'a4-checker-task1-receipts/python-environment.json')
    paths.add(ROOT / 'evidence/s02-candidate-a-completion/a4/native-resources/runner.py')
    entries = []
    with tarfile.open(OUT / 'original-small-evidence.tar.gz', 'x:gz') as archive:
        for path in sorted(paths):
            if not path.is_file() or path.is_symlink():
                raise RuntimeError('Regular original file required: ' + str(path))
            if path.stat().st_size > 2_000_000:
                raise RuntimeError('Non-small input must stay externally referenced: ' + str(path))
            raw = path.read_bytes()
            name = str(path.relative_to(ROOT))
            member = tarfile.TarInfo(name)
            member.size = len(raw)
            member.mode = path.stat().st_mode & 0o777
            member.mtime = int(path.stat().st_mtime)
            archive.addfile(member, io.BytesIO(raw))
            entries.append({'path': name, 'originalAbsolutePath': str(path),
                            'bytes': len(raw), 'sha256': sha(raw)})
    index = {
        'kind': 'original small evidence archive for root-admitted retained17 only',
        'originalRepositoryRoot': str(ROOT), 'archive': 'original-small-evidence.tar.gz',
        'archiveSha256': sha((OUT / 'original-small-evidence.tar.gz').read_bytes()),
        'members': entries, 'memberCount': len(entries), 'memberBytes': sum(e['bytes'] for e in entries),
        'scope': {'retainedControlsAdmitted': 17, 'freshControlsPending': 97,
                  'preNativeAggregateRequired': 114, 'nativeFinalSuiteRequired': 115,
                  'originalParentExit': None, 'a4Complete': False},
        'externalEvidence': [
            {'path': str(original), 'kind': 'original generated data retained in place',
             'hashSource': str(RESUMPTION / 'admit17/original-artifacts.json'),
             'manifestSha256': '043f61247e1fdafeab98d74ba9487e578aa386227da2855b9df28b805720ba7b',
             'files': 318, 'bytes': 9795796111, 'directories': 88, 'symlinks': 16,
             'packagingRehashPerformed': False},
            {'path': str(SDD / 'a4-checker-task1-receipts/python-environment.tar.gz'),
             'kind': 'reused admitted Python environment archive; not duplicated',
             'sha256': '7363d106d464f976af2a0e7f1f56e3c7bed237c7a3396a60d073369add068dac',
             'packagingRehashPerformed': False}],
        'packageSupport': [{'path': name, 'sha256': sha((OUT / name).read_bytes())}
                           for name in ('README.md', 'build_archive.py', 'audit.py')],
        'limitations': ['Original parent terminal/after-pins are absent; its exit remains unknown.',
                       'Raw-data hashes are recovery-time observations; continuous historical stability is not proved.',
                       'Historical assertions are inferred from mandatory asserted file creation in a fresh directory, not recorded flags.',
                       'Archive audit checks original receipt bindings; it does not replay controls or rehash external raw/runtime archives.']}
    with (OUT / 'index.json').open('x') as stream:
        json.dump(index, stream, indent=2, sort_keys=True)
        stream.write('\n')
    print(json.dumps({'archiveSha256': index['archiveSha256'], 'members': len(entries),
                      'uncompressedBytes': index['memberBytes']}))


if __name__ == '__main__':
    main()
