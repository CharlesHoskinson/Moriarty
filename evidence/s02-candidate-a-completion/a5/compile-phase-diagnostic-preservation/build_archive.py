"""Exclusive lossless packaging; no native execution or large runtime reads."""
import sys
if sys.flags.optimize != 0:
    raise RuntimeError('Packaging requires Python optimization disabled')
import gzip
import hashlib
import io
import json
from pathlib import Path
import re
import stat
import tarfile

HERE = Path(__file__).resolve().parent
ROOT = Path('/home/charl/Moriarty/.worktrees/s01-audit-start')
OUT = '.superpowers/sdd/a5-compiler-phase-diagnostic-20260906'
PROPOSAL = '.superpowers/sdd/a5-phase-package-proposal-20260906.md'
PROPOSAL_SHA = '30521716a0f5a440edc878846811487918de582388cb16c37ffc3425ba598193'
INTAKE_SHA = 'f52aabfe60056dfce067edefb3cae9e6fd4863761d1fda2275927599ef4bcac6'

def need(ok, message):
    if not ok:
        raise RuntimeError(message)

def sha(data):
    return hashlib.sha256(data).hexdigest()

def original(name):
    p = ROOT / name
    before = p.lstat()
    need(stat.S_ISREG(before.st_mode) and before.st_size <= 4_000_000, 'non-small regular original: '+name)
    data = p.read_bytes()
    after = p.lstat()
    need((before.st_ino, before.st_size, before.st_mtime_ns) == (after.st_ino, after.st_size, after.st_mtime_ns), 'original changed: '+name)
    return data

def save(name, obj):
    with (HERE / name).open('x') as stream:
        json.dump(obj, stream, indent=2, sort_keys=True)
        stream.write('\n')

def main():
    need(not (HERE/'index.json').exists() and not (HERE/'original-small-evidence.tar.gz').exists(), 'exclusive outputs already exist')
    proposal = original(PROPOSAL)
    need(sha(proposal) == PROPOSAL_SHA, 'reviewed proposal changed')
    table = proposal.decode().split('## External original members\n')[1].split('## External large runtime bindings\n')[0]
    rows = re.findall(r'^\| `([^`]+)` \| (\d+) \| `([0-9a-f]{64})` \|$', table, re.M)
    need(len(rows) == 22 and len({r[0] for r in rows}) == 22, 'external table inventory')
    intake_bytes = original(OUT+'/intake.json')
    need(sha(intake_bytes) == INTAKE_SHA, 'original intake changed')
    intake = json.loads(intake_bytes)
    pins = {str(Path(r['path']).relative_to(ROOT)): r for r in intake['files']}
    need(len(pins) == len(intake['files']) == 255, 'original intake inventory')
    for name, size, digest in rows:
        need(name not in pins, 'duplicate external name')
        pins[name] = {'bytes': int(size), 'sha256': digest}
    additions = [OUT+'/intake.json', *[OUT+'/transport/intake/'+n for n in ['command.json','response-000.json','terminal.json']], PROPOSAL]
    for name in additions:
        need(name not in pins, 'duplicate supplement name')
        data = original(name)
        pins[name] = {'bytes': len(data), 'sha256': sha(data)}
    directories = sorted([str(Path(p).relative_to(ROOT)) for p in intake['directories']] + [OUT+'/transport/intake'])
    actual_files, actual_dirs = [], []
    for p in (ROOT/OUT).rglob('*'):
        mode = p.lstat().st_mode
        need(stat.S_ISREG(mode) or stat.S_ISDIR(mode), 'unexpected original entry')
        (actual_files if stat.S_ISREG(mode) else actual_dirs).append(str(p.relative_to(ROOT)))
    need(set(actual_files) == {n for n in pins if n.startswith(OUT+'/')} and len(actual_files) == 259, 'closed OUT file set changed')
    need(sorted(actual_dirs) == directories and len(directories) == 25, 'closed OUT directory set changed')
    need(len(pins) == 282, 'exact package regular members')
    members = []
    with (HERE/'original-small-evidence.tar.gz').open('xb') as raw:
        with gzip.GzipFile(filename='', mode='wb', fileobj=raw, mtime=0) as gz:
            with tarfile.open(fileobj=gz, mode='w') as tar:
                for name in directories:
                    entry = tarfile.TarInfo(name); entry.type = tarfile.DIRTYPE; entry.mode = 0o755
                    tar.addfile(entry)
                for name, pin in sorted(pins.items()):
                    data = original(name)
                    need(len(data) == pin['bytes'] and sha(data) == pin['sha256'], 'original pin mismatch: '+name)
                    entry = tarfile.TarInfo(name); entry.size = len(data); entry.mode = 0o644
                    tar.addfile(entry, io.BytesIO(data))
                    members.append({'path': name, 'kind': 'file', 'bytes': len(data), 'sha256': sha(data)})
    archive = (HERE/'original-small-evidence.tar.gz').read_bytes()
    support = []
    for name in ['README.md','build_archive.py','audit.py']:
        data = (HERE/name).read_bytes()
        support.append({'path': name, 'bytes': len(data), 'sha256': sha(data)})
    freeze = json.loads(original(OUT+'/freeze.json'))
    index = {'schema': 'moriarty.a5-phase-diagnostic-preservation/v1', 'members': members, 'directories': directories,
             'archive': {'path': 'original-small-evidence.tar.gz', 'bytes': len(archive), 'sha256': sha(archive)},
             'packageSupport': support, 'proposalSha256': PROPOSAL_SHA, 'intakeSha256': INTAKE_SHA,
             'externalRuntimeArchives': [r for r in freeze['runtime']['references'] if r['path'].endswith('.tar.gz')],
             'scope': 'preservation only; new incomplete observed compile interval; H1 unresolved',
             'largeRuntimeRehashed': False, 'nativeExecuted': False, 'compilerAcceptance': False}
    save('index.json', index)
    report = {'ok': True, 'regularMembers': len(members), 'directories': len(directories), 'originalBytes': sum(r['bytes'] for r in members),
              'archiveBytes': len(archive), 'archiveSha256': sha(archive), 'externalRuntimeRehashed': False}
    save('packaging-report.json', report)
    print(json.dumps(report, sort_keys=True))

if __name__ == '__main__':
    main()
