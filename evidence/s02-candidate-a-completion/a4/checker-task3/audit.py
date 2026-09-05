"""Read-only Task3 receipt audit; run from the experimental worktree."""
import hashlib
import json
from pathlib import Path
import tarfile

BASE = Path('.superpowers/sdd/a4-checker-task3-receipts')
ENV = Path('.superpowers/sdd/a4-checker-task1-receipts/python-environment.json')
def sha(raw): return hashlib.sha256(raw).hexdigest()
def read(path): return json.loads(path.read_bytes())
assert sha(ENV.read_bytes()) == 'cd004057c4067bf7cc536d2cec5038add88d0852c1a7de5030e402ff2204f9c4'
env = read(ENV)
assert sha((ENV.parent/'python-environment.tar.gz').read_bytes()) == env['archiveSha256']
for e in env['sourceBytes']:
    assert sha(Path(e['path']).read_bytes()) == e['sha256']
stages, inputs = {}, {}
for name, code in {'tests-first':2, 'red-import-retry':0, 'red-tests-retry':1,
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
red, green = source('red-tests-retry','/scripts/a4_cases.py'), source('green-tests','/scripts/a4_cases.py')
fault = b"ledger=ledger(loop,2) if loop=='installment' else ledger(loop,0)"
assert red.count(fault) == 1 and red.replace(fault,b'ledger=ledger(loop,0)',1) == green
for suffix in ('/scripts/a4_carrier.py','/scripts/a4_agreement.py','/tests/test_s02_candidate_a_integrated.py'):
    assert source('red-tests-retry',suffix) == source('green-tests',suffix)
for e in inputs['green-tests']['sourceBytes']:
    assert sha(Path(e['path']).read_bytes()) == e['sha256']
assert '3 failed, 10 passed' in (BASE/'red-tests-retry/stdout.txt').read_text()
assert 'test_independent_initial_coupling' in (BASE/'red-tests-retry/stdout.txt').read_text()
assert '13 passed' in (BASE/'green-tests/stdout.txt').read_text()
print(json.dumps({'status':'accepted-local-task3-only', 'greenTests':13,
    'sharedEnvironmentFiles':len(env['sourceBytes']), 'behavioralSourceFiles':len(inputs['green-tests']['sourceBytes']),
    'compilingRedOnlyBehaviorChange':'installment initial ledger incorrectly emptied', 'stages':stages},indent=2))
