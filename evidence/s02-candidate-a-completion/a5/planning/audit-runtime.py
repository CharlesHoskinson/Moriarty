"""Root byte admission for the one shared runtime snapshot, not model checking."""
import hashlib
import json
from pathlib import Path
import tarfile

ROOT = Path.cwd()
BASE = ROOT/'.superpowers/sdd/a5-factoring-receipts'
STORE = BASE/'tool-store'
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def data(path): return json.loads(path.read_bytes())
dispatch = data(BASE/'dispatch.json')
assert dispatch['beforeDispatchCommit'] == '900bb2051225b4a3d99bf422c3b2e5e386e3e7bc'
assert sha(STORE/'manifest.json') == dispatch['runtimeManifestSha256'] == 'fa3e9838c66480557ed6d928a2d159ebc074d05f67f15ef08ab20388d9c49e0b'
assert sha(STORE/'runtime.tar.gz') == dispatch['runtimeArchiveSha256'] == 'f0687656d30c0d59ece8dfd027508a9228020506d29b97aea3abe7568186c31c'
manifest = data(STORE/'manifest.json')
with tarfile.open(STORE/'runtime.tar.gz') as tar:
    members = tar.getmembers()
    assert all(m.isfile() for m in members)
    expected = {e['archivePath']:e for e in manifest['files']}
    assert len(members) == len(expected)
    assert {m.name for m in members} == set(expected)
    for m in members:
        raw = tar.extractfile(m).read()
        assert hashlib.sha256(raw).hexdigest() == expected[m.name]['sha256']
for e in manifest['files'] + manifest['pythonFiles']:
    assert sha(Path(e['path'])) == e['sha256']
for root, entries in manifest['treeMembers'].items():
    assert sorted({str(p.resolve()) for p in Path(root).rglob('*') if p.is_file()}) == entries
for kind in ('Manifest','Archive'):
    assert sha(Path(manifest['reusedPython'+kind])) == manifest['reusedPython'+kind+'Sha256']
for v in manifest['versions']:
    for stream in ('stdout','stderr'):
        assert sha(STORE/(v['name']+'.'+stream)) == v[stream+'Sha256']
    assert v['exitCode'] == (1 if v['name']=='rust' else 0)
stage = BASE/'bootstrap-command'
inp, result = data(stage/'input.json'), data(stage/'result.json')
assert result['exitCode'] == 0 and not result['timedOut'] and result['sourceUnchanged']
assert sha(stage/'source.tar.gz') == inp['sourceArchiveSha256']
with tarfile.open(stage/'source.tar.gz') as tar:
    byname = {m.name:m for m in tar.getmembers() if m.isfile()}
    for e in inp['pins']:
        relative = str(Path(e['path']).relative_to(ROOT))
        assert relative in byname
        assert hashlib.sha256(tar.extractfile(byname[relative]).read()).hexdigest() == e['sha256']
for stream in ('stdout','stderr'):
    assert sha(stage/(stream+'.txt')) == result[stream+'Sha256']
print(json.dumps({'status':'runtime-byte-admission-only','runtimeFiles':len(manifest['files']),
    'reusedPythonFiles':len(manifest['pythonFiles']), 'runtimeArchiveBytes':(STORE/'runtime.tar.gz').stat().st_size,
    'dispatch':dispatch, 'bootstrap':result,
    'scope':'original runtime bytes and command provenance; no Candidate A behavioral/checker verdict'},indent=2))
