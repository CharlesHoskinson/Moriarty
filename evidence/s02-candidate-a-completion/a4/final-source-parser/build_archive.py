"""Exclusive approved evidence packaging. No native execution or runtime reads."""
import sys
if sys.flags.optimize:
    raise RuntimeError('Packaging requires optimization disabled')
import hashlib
import io
import json
from pathlib import Path, PurePosixPath
import re
import tarfile

HERE = Path(__file__).resolve().parent
ROOT = Path('/home/charl/Moriarty/.worktrees/s01-audit-start')
S = '.superpowers/sdd/'
PROPOSAL = S+'a4-final-source-parser-package-proposal-20260906.md'
PROPOSAL_SHA = 'b1a3bc6facb99e14dc94ab08bf4f4c20d988b7b6682587e2e40f5d26d67fd763'
ADMISSION = S+'a4-final-source-parser-admission-20260906.json'

def need(ok, message):
    if not ok:
        raise ValueError(message)

def digest(data):
    return hashlib.sha256(data).hexdigest()

def original(name):
    path = PurePosixPath(name)
    need(name == str(path) and not path.is_absolute() and '..' not in path.parts, 'unsafe original path')
    p = ROOT/name
    need(p.is_file() and not p.is_symlink() and not any(q.is_symlink() for q in p.parents), 'nonregular original')
    return p.read_bytes()

def main():
    proposal = original(PROPOSAL)
    need(digest(proposal) == PROPOSAL_SHA, 'proposal changed')
    admission_bytes = original(ADMISSION)
    need(digest(admission_bytes) == '29a427818e9894ce6674e9f4f26dfe4527736fa6289bde6b5b3a5e6664caec71', 'admission changed')
    admission = json.loads(admission_bytes)
    expected = dict(admission['originals'])
    need(len(expected) == 263, 'base count')
    rows = re.findall(r'^\| `((?:S/|evidence/|scripts/)[^`]+)` \| `([0-9a-f]{64})` \|$', proposal.decode(), re.M)
    need(len(rows) == 26, 'additional count')
    for name, sha in rows:
        name = S+name[2:] if name.startswith('S/') else name
        need(name not in expected, 'overlapping additional member')
        expected[name] = {'sha256': sha}
    expected[PROPOSAL] = {'sha256': PROPOSAL_SHA}
    need(len(expected) == 290, 'final count')
    originals = {}
    for name, pin in expected.items():
        b = original(name)
        need(digest(b) == pin['sha256'] and ('bytes' not in pin or len(b) == pin['bytes']), 'original changed: '+name)
        originals[name] = b
    report = originals[S+'a4-final-source-parser-independent-intake-20260906.md'].decode()
    blocks = [json.loads(b) for b in re.findall(r'```json\n(.*?)\n```', report, re.S)]
    need(json.loads(blocks[2]['output'])['originals'] == admission['originals'], 'independent original map')
    archive = HERE/'original-evidence.tar.gz'
    need(not archive.exists() and not (HERE/'index.json').exists(), 'fresh archive/index required')
    with archive.open('xb') as target:
        with tarfile.open(fileobj=target, mode='w:gz', format=tarfile.PAX_FORMAT) as tar:
            for name, b in sorted(originals.items()):
                info = tarfile.TarInfo(name); info.size = len(b); info.mode = 0o644
                tar.addfile(info, io.BytesIO(b))
    members = [{'path': n, 'bytes': len(b), 'sha256': digest(b)} for n,b in sorted(originals.items())]
    support = []
    for name in ['README.md', 'build_archive.py', 'audit.py']:
        b = (HERE/name).read_bytes()
        support.append({'path': name, 'bytes': len(b), 'sha256': digest(b)})
    external = [{'path': S+'a5-factoring-receipts/tool-store/runtime.tar.gz', 'bytes': 315704660,
                 'sha256': 'f0687656d30c0d59ece8dfd027508a9228020506d29b97aea3abe7568186c31c'},
                {'path': S+'a4-checker-task1-receipts/python-environment.tar.gz', 'bytes': 66716078,
                 'sha256': '7363d106d464f976af2a0e7f1f56e3c7bed237c7a3396a60d073369add068dac'}]
    b = archive.read_bytes()
    index = {'schema': 'moriarty.a4-final-source-parser-preservation/v1',
             'archive': {'path': archive.name, 'bytes': len(b), 'sha256': digest(b)},
             'members': members, 'packageSupport': support, 'externalRuntimeArchives': external,
             'proposalSha256': PROPOSAL_SHA, 'originalDispatchHead': admission['actualDispatchHead'],
             'packagingHeadObserved': '4006244885c7cf34562fe54cbb76856142b0d244',
             'packagingHeadObservationToolChunk': '3ec86b',
             'scope': 'preserved parser evidence and external recorded bindings only',
             'selfAndLaterReceiptHashesExcluded': True, 'archivedCodeExecuted': False,
             'currentRuntimeOrExternalArchivesRehashed': False, 'nativePilotAccepted': False,
             'full78ExporterAccepted': False, 'finalA4Accepted': False, 'futureFreshParserCallsRequired': True}
    with (HERE/'index.json').open('x') as f:
        json.dump(index, f, sort_keys=True, indent=2); f.write('\n')
    # Recheck exactly these source bytes after capture; no live runtime reads.
    for name,b in originals.items():
        need(original(name) == b, 'original changed during packaging: '+name)
    print(json.dumps({'ok': True, 'members': len(members), 'originalBytes': sum(r['bytes'] for r in members),
                      'archive': index['archive'], 'externalRuntimeArchivesRehashed': False,
                      'nativeOrArchivedCodeExecuted': False}))

if __name__ == '__main__':
    main()
