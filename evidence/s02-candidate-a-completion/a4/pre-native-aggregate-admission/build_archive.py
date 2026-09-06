"""Exclusive small-original archive; no external large-tree/runtime rehash."""
import hashlib
import io
import json
from pathlib import Path
import sys
import tarfile

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[3]
SDD = ROOT / '.superpowers/sdd'
R = SDD / 'a4-task6-resumption-20260906'
F = R / 'fresh97'
PRIOR = OUT.parent / 'pre-native-recovery-admission'


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def main():
    require(sys.flags.optimize == 0, 'Python optimization must be disabled')
    adoption = R / 'root-admission114.json'
    review = SDD / 'a4-fresh97-independent-intake-20260906.md'
    proposal = SDD / 'a4-fresh97-package-proposal-20260906.md'
    require(sha(adoption.read_bytes()) == '2b112e8273b7f2a1b12adf9cc1404dc7c3da5fb933e14a972fc12f35519aba08', 'root adoption pin')
    require(sha(review.read_bytes()) == '650058f9d22b21946f68e0b50034d5db4efc840e23c7f871e9ae9b36f85197e0', 'independent intake pin')
    require(sha(proposal.read_bytes()) == 'd013b17962cf8227925ebbe797b6a951b75dc31e65d19c797df069b2c959fffa', 'proposal pin')
    manifest = json.loads((F / 'fresh-artifacts.json').read_text())
    selected = [e for e in manifest['entries'] if e['kind'] == 'file' and e['bytes'] <= 2_000_000]
    require(len(selected) == 100, 'exact100 small-original selection')
    paths = {p for p in F.iterdir() if p.is_file()}
    require(len(paths) == 21, 'exact21 fresh top-level originals; review any new additions')
    paths.update(Path(e['path']) for e in selected)
    paths.update(R / n for n in ('recorder.py', 'lifecycle_controls.py', 'partition.json',
                 'reviewed-amendment.md', 'receipt-supplement.md', 'support.json',
                 'root-dispatch-fresh97.json', 'fresh97-outer-progress.ndjson',
                 'root-admission17.json', 'root-dispatch-admit17.json'))
    paths.update(PRIOR / n for n in ('original-small-evidence.tar.gz', 'index.json',
                 'audit-report.json', 'audit-tool-receipt.json', 'README.md', 'audit.py', 'build_archive.py'))
    paths.update((adoption, review, proposal))
    recorded = {e['path']: e for e in selected}
    entries = []
    with tarfile.open(OUT / 'original-small-evidence.tar.gz', 'x:gz') as archive:
        for path in sorted(paths):
            require(path.is_file() and not path.is_symlink(), 'regular original file required: ' + str(path))
            require(path.stat().st_size <= 2_000_000, 'large input must remain external: ' + str(path))
            raw = path.read_bytes()
            if str(path) in recorded:
                expected = recorded[str(path)]
                require(len(raw) == expected['bytes'] and sha(raw) == expected['sha256'],
                        'selected small artifact changed: ' + str(path))
            name = str(path.relative_to(ROOT))
            member = tarfile.TarInfo(name)
            member.size = len(raw)
            member.mode = path.stat().st_mode & 0o777
            member.mtime = int(path.stat().st_mtime)
            archive.addfile(member, io.BytesIO(raw))
            entries.append({'path': name, 'originalAbsolutePath': str(path),
                            'bytes': len(raw), 'sha256': sha(raw)})
    index = {'kind': 'root-admitted amended preliminary114 small evidence',
             'originalRepositoryRoot': str(ROOT), 'archive': 'original-small-evidence.tar.gz',
             'archiveSha256': sha((OUT / 'original-small-evidence.tar.gz').read_bytes()),
             'members': entries, 'memberCount': len(entries), 'memberBytes': sum(e['bytes'] for e in entries),
             'selectedSmallArtifactPaths': [str(Path(e['path']).relative_to(ROOT)) for e in selected],
             'smallThresholdBytes': 2_000_000,
             'scope': {'preliminaryAdmitted': 114, 'retained': 17, 'fresh': 97,
                       'nativeFinalRequired': 115, 'originalParentExit': None, 'a4Complete': False},
             'externalEvidence': [
                 {'kind': 'fresh generated original tree', 'path': str(F / 'basetemp'),
                  'manifestPath': str(F / 'fresh-artifacts.json'),
                  'manifestSha256': '5148c33e29abe44526f76825d31611f6938c46f3fa7986617412eaab72eed43c',
                  'files': 274, 'bytes': 6237424864, 'directories': 76, 'symlinks': 20,
                  'largeFiles': 174, 'largeBytes': 6204874085, 'largeRehashPerformed': False},
                 {'kind': 'retained original tree', 'path': str(SDD / 'a4-task6-receipts/green-tests'),
                  'manifestPath': str(R / 'admit17/original-artifacts.json'),
                  'manifestSha256': '043f61247e1fdafeab98d74ba9487e578aa386227da2855b9df28b805720ba7b',
                  'files': 318, 'bytes': 9795796111, 'directories': 88, 'symlinks': 16,
                  'largeRehashPerformed': False},
                 {'kind': 'reused3329-file Python runtime archive',
                  'path': str(SDD / 'a4-checker-task1-receipts/python-environment.tar.gz'),
                  'sha256': '7363d106d464f976af2a0e7f1f56e3c7bed237c7a3396a60d073369add068dac',
                  'largeRehashPerformed': False}],
             'packageSupport': [{'path': n, 'sha256': sha((OUT / n).read_bytes())}
                                for n in ('README.md', 'build_archive.py', 'audit.py', 'retained_checks.py')],
             'limitations': json.loads(adoption.read_text())['limitations'],
             'stillRequired': json.loads(adoption.read_text())['stillRequired']}
    with (OUT / 'index.json').open('x') as stream:
        json.dump(index, stream, indent=2, sort_keys=True)
        stream.write('\n')
    print(json.dumps({'archiveSha256': index['archiveSha256'], 'members': len(entries),
                      'memberBytes': index['memberBytes'], 'selectedSmallOriginals': len(selected)}))


if __name__ == '__main__':
    main()
