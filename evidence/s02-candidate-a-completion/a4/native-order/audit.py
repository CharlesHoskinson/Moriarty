"""Read-only root intake of the native-variable-order correction receipts."""
import hashlib
import json
from pathlib import Path
import tarfile

BASE = Path('.superpowers/sdd/a4-native-order-receipts')
ENV = Path('.superpowers/sdd/a4-checker-task1-receipts/python-environment.json')
def sha(raw): return hashlib.sha256(raw).hexdigest()
def read(path): return json.loads(path.read_bytes())
def file_sha(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as stream:
        while data := stream.read(1048576): h.update(data)
    return h.hexdigest()
assert file_sha(ENV) == 'cd004057c4067bf7cc536d2cec5038add88d0852c1a7de5030e402ff2204f9c4'
env = read(ENV)
assert file_sha(ENV.parent/'python-environment.tar.gz') == env['archiveSha256']
for e in env['sourceBytes']: assert file_sha(e['path']) == e['sha256']
stages, sources = {}, {}
for name, code in {'red-import':0, 'red-tests':1, 'green-syntax':0, 'green-tests':0}.items():
    folder = BASE/name
    inp, result = read(folder/'input.json'), read(folder/'terminal.json')
    assert result['exitCode'] == code
    assert not result['sourceChanged'] and not result['environmentChanged']
    assert inp['baseCommit'].startswith('4465863') and inp['sourceCommit'] == inp['baseCommit']
    assert inp['parentOrigArgv'][1:4] == ['-B', '-X', 'pycache_prefix='+inp['parentCachePrefix']]
    assert inp['argv'][1:3] == ['-B', '-X'] and inp['argv'][3].startswith('pycache_prefix=')
    assert not inp['absentSources']
    assert {e['path']:e['sha256'] for e in inp['sourceBytes']} == {e['path']:e['sha256'] for e in result['sourceAfter']}
    assert inp['environmentBefore'] == result['environmentAfter']
    assert inp['pythonBefore'] == result['pythonAfter']
    assert inp['pythonEnvironmentManifestSha256'] == file_sha(ENV)
    assert inp['pythonEnvironmentArchiveSha256'] == env['archiveSha256']
    assert file_sha(folder/'source.tar.gz') == inp['sourceArchiveSha256']
    sources[name] = {}
    with tarfile.open(folder/'source.tar.gz') as tar:
        members = {m.name:m for m in tar.getmembers() if m.isfile()}
        assert len(members) == len(inp['sourceBytes'])
        for e in inp['sourceBytes']:
            raw = tar.extractfile(members[e['archivePath']]).read()
            assert len(raw) == e['bytes'] and sha(raw) == e['sha256']
            sources[name][e['path']] = raw
            if name.startswith('green'): assert file_sha(e['path']) == e['sha256']
    for stream in ('stdout', 'stderr'):
        assert file_sha(folder/(stream+'.txt')) == result[stream+'Sha256']
    assert file_sha(folder/'artifacts.json') == result['artifactManifestSha256']
    artifacts = read(folder/'artifacts.json')['entries']
    assert len(artifacts) == result['artifactEntries']
    for e in artifacts:
        path = Path(e['path'])
        if e['kind'] == 'symlink': assert path.is_symlink() and str(path.readlink()) == e['target']
        elif e['kind'] == 'directory': assert path.is_dir() and not path.is_symlink()
        else:
            assert e['kind'] == 'file' and not path.is_symlink()
            assert path.stat().st_size == e['bytes'] and file_sha(path) == e['sha256']
    stages[name] = {k:result[k] for k in ('exitCode', 'elapsedSeconds', 'stdoutSha256', 'stderrSha256')}
    stages[name].update(sourceFiles=len(inp['sourceBytes']), artifactEntries=len(artifacts),
                        inputSha256=file_sha(folder/'input.json'), terminalSha256=file_sha(folder/'terminal.json'))
module = str(Path('scripts/check_s02_candidate_a_integrated.py').resolve())
tests = str(Path('tests/test_s02_candidate_a_integrated.py').resolve())
assert {p for p in sources['red-tests'] if sources['red-tests'][p] != sources['green-tests'][p]} == {module}
assert sources['red-tests'][tests] == sources['green-tests'][tests]
assert sources['red-tests'][module].replace(
    b"VARS=('authorityState','latestEvent','caseIndex','cursor')",
    b"VARS=('authorityState','caseIndex','cursor','latestEvent')", 1) == sources['green-tests'][module]
assert '2 failed' in (BASE/'red-tests/stdout.txt').read_text()
assert 'exact raw vars and order' in (BASE/'red-tests/stdout.txt').read_text()
assert '79 passed' in (BASE/'green-tests/stdout.txt').read_text()
probe = Path('.superpowers/sdd/a4-producer-receipts/task2-literal-probe-run/probe.itf.json')
assert file_sha(probe) == '7b14b09c806313f38548a689f1c909033d6fbb7182350ef82c70c4932744aaf4'
assert read(probe)['vars'] == ['authorityState', 'caseIndex', 'cursor', 'latestEvent']
print(json.dumps({'status':'accepted-local-native-order-correction-only', 'tests':79,
    'checkerSha256':file_sha(module), 'testSha256':file_sha(tests),
    'sharedRuntimeFiles':len(env['sourceBytes']), 'stages':stages,
    'scope':'transport compatibility; no Task6, native Candidate A export, or package acceptance'}, indent=2))
