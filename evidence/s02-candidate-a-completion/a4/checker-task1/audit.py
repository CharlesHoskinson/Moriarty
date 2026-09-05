"""Read-only original-byte audit; execute from the experimental worktree."""
import hashlib
import json
from pathlib import Path
import tarfile

ROOT = Path.cwd()
BASE = ROOT / '.superpowers/sdd/a4-checker-task1-receipts'
def digest(data):
    return hashlib.sha256(data).hexdigest()
def read(path):
    return json.loads(path.read_bytes())
def archive(path, entries):
    with tarfile.open(path) as tar:
        members = {m.name: m for m in tar.getmembers() if m.isfile()}
        assert len(members) == len(entries)
        for entry in entries:
            raw = tar.extractfile(members[entry['archivePath']]).read()
            assert len(raw) == entry['bytes'] and digest(raw) == entry['sha256']

environment = read(BASE / 'python-environment.json')
assert digest((BASE / 'python-environment.tar.gz').read_bytes()) == environment['archiveSha256']
archive(BASE / 'python-environment.tar.gz', environment['sourceBytes'])
for entry in environment['sourceBytes']:
    assert digest(Path(entry['path']).read_bytes()) == entry['sha256']
stages = {}
expected = {'tests-first': 2, 'tests-first-recorder-corrected': 2,
            'red-syntax': 0, 'red-import': 0, 'red-tests': 1,
            'green-syntax': 0, 'green-import': 0, 'green-tests': 0}
inputs = {}
for stage, code in expected.items():
    directory = BASE / stage
    inp, result = read(directory / 'input.json'), read(directory / 'terminal.json')
    inputs[stage] = inp
    assert result['exitCode'] == code
    assert digest((directory / 'source.tar.gz').read_bytes()) == inp['sourceArchiveSha256']
    archive(directory / 'source.tar.gz', inp['sourceBytes'])
    before = {e['path']: e['sha256'] for e in inp['sourceBytes']}
    after = {e['path']: e['sha256'] for e in result['sourceAfter']}
    assert before == after
    assert inp['environmentBefore'] == result['environmentAfter']
    assert not result['environmentChanged']
    if stage != 'tests-first':
        assert not result['sourceChanged']
    for stream in ('stdout', 'stderr'):
        assert digest((directory / (stream + '.txt')).read_bytes()) == result[stream + 'Sha256']
    stages[stage] = {k: result[k] for k in ('exitCode', 'elapsedSeconds', 'stdoutSha256', 'stderrSha256')}
    stages[stage]['inputSha256'] = digest((directory / 'input.json').read_bytes())
    stages[stage]['terminalSha256'] = digest((directory / 'terminal.json').read_bytes())

red = (BASE / 'red-tests/stdout.txt').read_text()
green = (BASE / 'green-tests/stdout.txt').read_text()
assert 'DID NOT RAISE' in red and '1 failed, 4 passed' in red
assert '5 passed' in green
def source(stage, suffix):
    inp = inputs[stage]
    entry = next(e for e in inp['sourceBytes'] if e['path'].endswith(suffix))
    with tarfile.open(BASE / stage / 'source.tar.gz') as tar:
        return tar.extractfile(entry['archivePath']).read()
red_carrier = source('red-tests', '/scripts/a4_carrier.py')
green_carrier = source('green-tests', '/scripts/a4_carrier.py')
assert red_carrier.replace(b'            pass\n', b"            raise Invalid('duplicate map key')\n", 1) == green_carrier
for entry in inputs['green-tests']['sourceBytes']:
    assert digest(Path(entry['path']).read_bytes()) == entry['sha256']
assert source('red-tests', '/scripts/a4_agreement.py') == source('green-tests', '/scripts/a4_agreement.py')
assert source('red-tests', '/tests/test_s02_candidate_a_integrated.py') == source('green-tests', '/tests/test_s02_candidate_a_integrated.py')
print(json.dumps({'status': 'accepted-local-task1-only', 'environmentFiles': len(environment['sourceBytes']),
    'stages': stages, 'sourceFilesPerBehavioralStage': len(inputs['green-tests']['sourceBytes']),
    'compilingRedOnlyChange': 'duplicate-map rejection disabled', 'greenTests': 5,
    'scope': 'native root byte/source review; not Council or full A4 acceptance'}, indent=2))
