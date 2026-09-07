"""Root audit of original kernel RED/GREEN receipts. Not A5 completion."""
import hashlib
import json
from pathlib import Path
import tarfile

ROOT = Path.cwd()
BASE = ROOT/'.superpowers/sdd/a5-factoring-receipts'
def sha(raw): return hashlib.sha256(raw).hexdigest()
def data(path): return json.loads(path.read_bytes())
stages = {}
source = {}
for name, code in {'kernel-red-typecheck':0,'kernel-red-test':1,
                   'kernel-green-typecheck':0,'kernel-green-tests':0}.items():
    stage = BASE/name
    inp, result = data(stage/'input.json'), data(stage/'result.json')
    assert inp['beforeDispatchCommit'] == '900bb2051225b4a3d99bf422c3b2e5e386e3e7bc'
    assert result['exitCode'] == code and not result['timedOut']
    assert result['runtimeUnchanged'] and result['sourceAndToolsUnchanged']
    assert sha((stage/'source-and-runner.tar.gz').read_bytes()) == inp['sourceArchiveSha256']
    expected = {str(Path(e['path']).relative_to(ROOT)):e for e in inp['pins'] if Path(e['path']).is_relative_to(ROOT)}
    with tarfile.open(stage/'source-and-runner.tar.gz') as tar:
        members = {m.name:m for m in tar.getmembers() if m.isfile()}
        assert set(members) == set(expected) and len(members) == 38
        source[name] = {}
        for path, e in expected.items():
            raw = tar.extractfile(members[path]).read()
            assert sha(raw) == e['sha256']
            source[name][path] = raw
            if name.startswith('kernel-green'):
                assert sha(Path(e['path']).read_bytes()) == e['sha256']
    for e in inp['pins']:
        if not Path(e['path']).is_relative_to(ROOT):
            assert sha(Path(e['path']).read_bytes()) == e['sha256']
    for stream in ('stdout','stderr'):
        assert sha((stage/(stream+'.txt')).read_bytes()) == result[stream+'Sha256']
    stages[name] = result | {'inputSha256':sha((stage/'input.json').read_bytes()),
                            'resultSha256':sha((stage/'result.json').read_bytes()),
                            'sourceArchiveSha256':inp['sourceArchiveSha256']}
kernel = 'specs/quint/s02/factored_verification/candidate_a_joint_f.qnt'
for red in ('kernel-red-typecheck','kernel-red-test'):
    assert {p for p in source[red] if source[red][p] != source['kernel-green-tests'][p]} == {kernel}
    assert source[red][kernel] == source['kernel-green-tests'][kernel].replace(
        b'if (supplied != actual) EffectResultMismatchA else joint.extraction', b'joint.extraction', 1)
assert source['kernel-green-tests'] == source['kernel-green-typecheck']
assert 'QNT508' in (BASE/'kernel-red-test/stdout.txt').read_text()
assert '6 passing' in (BASE/'kernel-green-tests/stdout.txt').read_text()
print(json.dumps({'status':'accepted-local-kernel-task1-only','greenTests':6,
    'behavioralSourceFiles':38,'soleCorrection':'restore supplied-result mismatch rejection',
    'scope':'finite kernel tests; no derived lifecycle, pilot or full A5 verdict', 'stages':stages},indent=2))
