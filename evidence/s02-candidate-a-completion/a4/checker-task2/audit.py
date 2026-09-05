"""Read-only Task2 receipt audit; run from the experimental worktree."""
import hashlib
import json
from pathlib import Path
import tarfile

BASE = Path('.superpowers/sdd/a4-checker-task2-receipts')
ENV = Path('.superpowers/sdd/a4-checker-task1-receipts/python-environment.json')
def sha(raw): return hashlib.sha256(raw).hexdigest()
def read(path): return json.loads(path.read_bytes())
assert sha(ENV.read_bytes()) == 'cd004057c4067bf7cc536d2cec5038add88d0852c1a7de5030e402ff2204f9c4'
env = read(ENV)
assert sha((ENV.parent/'python-environment.tar.gz').read_bytes()) == env['archiveSha256']
for e in env['sourceBytes']:
    assert sha(Path(e['path']).read_bytes()) == e['sha256']
stages, inputs = {}, {}
for name, code in {'tests-first':2, 'red-syntax':0, 'red-import':0, 'red-tests':1,
                   'green-syntax':0, 'green-import':0, 'green-tests':0}.items():
    directory = BASE/name
    inp, result = read(directory/'input.json'), read(directory/'terminal.json')
    inputs[name] = inp
    assert result['exitCode'] == code
    assert not result['sourceChanged'] and not result['environmentChanged']
    assert {e['path']:e['sha256'] for e in inp['sourceBytes']} == {e['path']:e['sha256'] for e in result['sourceAfter']}
    assert inp['environmentBefore'] == result['environmentAfter']
    assert inp['pythonEnvironmentManifestSha256'] == sha(ENV.read_bytes())
    assert inp['pythonEnvironmentArchiveSha256'] == env['archiveSha256']
    assert sha((directory/'source.tar.gz').read_bytes()) == inp['sourceArchiveSha256']
    with tarfile.open(directory/'source.tar.gz') as tar:
        members = {m.name:m for m in tar.getmembers() if m.isfile()}
        assert len(members) == len(inp['sourceBytes'])
        for e in inp['sourceBytes']:
            raw = tar.extractfile(members[e['archivePath']]).read()
            assert len(raw) == e['bytes'] and sha(raw) == e['sha256']
    for stream in ('stdout','stderr'):
        assert sha((directory/(stream+'.txt')).read_bytes()) == result[stream+'Sha256']
    stages[name] = {k:result[k] for k in ('exitCode','elapsedSeconds','stdoutSha256','stderrSha256')}
    stages[name]['inputSha256'] = sha((directory/'input.json').read_bytes())
    stages[name]['terminalSha256'] = sha((directory/'terminal.json').read_bytes())

def source(stage, suffix):
    e = next(e for e in inputs[stage]['sourceBytes'] if e['path'].endswith(suffix))
    with tarfile.open(BASE/stage/'source.tar.gz') as tar:
        return tar.extractfile(e['archivePath']).read()
red, green = source('red-tests','/scripts/a4_authority.py'), source('green-tests','/scripts/a4_authority.py')
fault = b"    if not coupling(s['authority']['context']): return False\n"
assert red.count(fault) == 1 and red.replace(fault,b'',1) == green
for suffix in ('/scripts/a4_carrier.py','/scripts/a4_agreement.py','/tests/test_s02_candidate_a_integrated.py'):
    assert source('red-tests',suffix) == source('green-tests',suffix)
for e in inputs['green-tests']['sourceBytes']:
    assert sha(Path(e['path']).read_bytes()) == e['sha256']
assert '1 failed, 6 passed' in (BASE/'red-tests/stdout.txt').read_text()
assert 'test_uncoupled_rejection_is_reachable' in (BASE/'red-tests/stdout.txt').read_text()
assert '7 passed' in (BASE/'green-tests/stdout.txt').read_text()
print(json.dumps({'status':'accepted-local-task2-only', 'greenTests':7,
    'sharedEnvironmentFiles':len(env['sourceBytes']), 'behavioralSourceFiles':len(inputs['green-tests']['sourceBytes']),
    'compilingRedOnlyBehaviorChange':'uncoupled rejection incorrectly disabled', 'stages':stages},indent=2))
