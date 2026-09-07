"""Exclusive approved failed-pilot preservation; no runtime/archive execution."""
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
PROPOSAL = S+'a4-pilot014-failure-package-proposal-20260906.md'
PROPOSAL_SHA = 'f099390b42f717d24e7660e1c14b9bfc4a73834bdaf25b4e3f3d7e20da29d543'
ORIGINAL_INDEX = S+'a4-pilot014-failure-independent-index-20260906.json'

def need(ok, message):
    if not ok:
        raise ValueError(message)

def sha(b):
    return hashlib.sha256(b).hexdigest()

def original(name):
    p = PurePosixPath(name)
    need(name == str(p) and not p.is_absolute() and '..' not in p.parts, 'safe original path')
    path = ROOT/name
    need(path.is_file() and not path.is_symlink() and not any(p.is_symlink() for p in path.parents), 'regular original')
    return path.read_bytes()

def main():
    proposal = original(PROPOSAL); need(sha(proposal) == PROPOSAL_SHA, 'proposal changed')
    b = original(ORIGINAL_INDEX)
    need(sha(b) == 'e0f7cfda8a2ef9bd9cf913485d24d1928f9b211f795b4a6c40af4e99da8b4599', 'original index changed')
    expected = dict(json.loads(b)['originals']); need(len(expected) == 303, '303 originals')
    extras = re.findall(r'^\| `((?:S/|evidence/|scripts/)[^`]+)` \| `([0-9a-f]{64})` \|$',proposal.decode(),re.M)
    need(len(extras) == 21, '21 additions')
    for n,h in extras:
        n = S+n[2:] if n.startswith('S/') else n
        need(n not in expected, 'additional overlap'); expected[n] = {'sha256':h}
    expected[PROPOSAL] = {'sha256':PROPOSAL_SHA}; need(len(expected) == 325, '325 members')
    data = {}
    for n,pin in expected.items():
        b = original(n)
        need(sha(b) == pin['sha256'] and ('bytes' not in pin or len(b) == pin['bytes']), 'original changed: '+n)
        data[n] = b
    archive = HERE/'original-evidence.tar.gz'
    need(not archive.exists() and not (HERE/'index.json').exists(), 'fresh archive/index')
    with archive.open('xb') as f:
        with tarfile.open(fileobj=f,mode='w:gz',format=tarfile.PAX_FORMAT) as tar:
            for n,b in sorted(data.items()):
                info = tarfile.TarInfo(n); info.size = len(b); info.mode = 0o644
                tar.addfile(info,io.BytesIO(b))
    support = []
    for n in ['README.md','build_archive.py','audit.py']:
        b = (HERE/n).read_bytes(); support.append({'path':n,'bytes':len(b),'sha256':sha(b)})
    external = [{'path':S+'a5-factoring-receipts/tool-store/runtime.tar.gz','bytes':315704660,
                 'sha256':'f0687656d30c0d59ece8dfd027508a9228020506d29b97aea3abe7568186c31c'},
                {'path':S+'a4-checker-task1-receipts/python-environment.tar.gz','bytes':66716078,
                 'sha256':'7363d106d464f976af2a0e7f1f56e3c7bed237c7a3396a60d073369add068dac'}]
    b = archive.read_bytes()
    index = {'schema':'moriarty.a4-pilot014-failure-preservation/v1',
             'archive':{'path':archive.name,'bytes':len(b),'sha256':sha(b)},
             'members':[{'path':n,'bytes':len(b),'sha256':sha(b)}for n,b in sorted(data.items())],
             'packageSupport':support,'proposalSha256':PROPOSAL_SHA,'externalRuntimeArchives':external,
             'externalParserPackage':{'archive':{'path':'evidence/s02-candidate-a-completion/a4/final-source-parser/original-evidence.tar.gz',
                'bytes':5434195,'sha256':'7233f85c03bfacf80d83d50a657a10f05a23d4863c157ecc81a4c37b13ee8119'},
                'index':{'path':'evidence/s02-candidate-a-completion/a4/final-source-parser/index.json','bytes':78831,
                'sha256':'19348e1089004e998e096b3c59dff2cbc7888a98a5b31fd7ae89db5dc6a00932'}},
             'originalDispatchHead':'08e426c7163880b9312f1f1f029a4dde9d0e7593',
             'packagingHeadObserved':'4006244885c7cf34562fe54cbb76856142b0d244','packagingHeadObservationToolChunk':'f8db22',
             'scope':'failed-pilot preservation and recorded external dependencies only',
             'selfAndLaterReceiptHashesExcluded':True,'archivedCodeExecuted':False,'externalArchivesOrRuntimeRehashed':False,
             'pilotAccepted':False,'case044Authorized':False,'full78Authorized':False,'structuralIntakePerformed':False,
             'canonicalStageConsumed':True,'internalPhase':'unknown','explicitNodeHeapFlag':False}
    with (HERE/'index.json').open('x') as f:json.dump(index,f,sort_keys=True,indent=2);f.write('\n')
    for n,b in data.items():need(original(n) == b, 'changed during packaging: '+n)
    print(json.dumps({'ok':True,'members':len(data),'originalBytes':sum(len(b)for b in data.values()),
                      'archive':index['archive'],'externalArchivesOrRuntimeRehashed':False,'nativeOrArchivedCodeExecuted':False}))

if __name__ == '__main__':
    main()
