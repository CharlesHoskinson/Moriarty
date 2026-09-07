"""Read-only root admission of the original sharded-checker unit receipts."""
import hashlib
import json
from pathlib import Path
import tarfile

BASE = Path('.superpowers/sdd/a4-checker-sharded-receipts')
ENV = Path('.superpowers/sdd/a4-checker-task1-receipts/python-environment.json')
def sha(raw): return hashlib.sha256(raw).hexdigest()
def read(path): return json.loads(path.read_bytes())
assert sha(ENV.read_bytes()) == 'cd004057c4067bf7cc536d2cec5038add88d0852c1a7de5030e402ff2204f9c4'
env = read(ENV)
assert sha((ENV.parent/'python-environment.tar.gz').read_bytes()) == env['archiveSha256']
for e in env['sourceBytes']: assert sha(Path(e['path']).read_bytes()) == e['sha256']
stages, sources = {}, {}
for name, code in {'tests-first':2, 'red-import':0, 'red-tests':1,
                   'green-syntax':0, 'green-import':0, 'direct-help-green':0,
                   'green-tests':1, 'metadata-red':1, 'final-syntax':0, 'final-tests':0}.items():
    directory = BASE/name
    inp, result = read(directory/'input.json'), read(directory/'terminal.json')
    assert result['exitCode'] == code
    assert not result['sourceChanged'] and not result['environmentChanged']
    assert inp['baseCommit'] == '9d26d9ba1e5cd970ff7be599473901be0d1eb33f'
    assert inp['parentOrigArgv'][1:4] == ['-B', '-X', 'pycache_prefix='+inp['parentCachePrefix']]
    assert inp['argv'][1:3] == ['-B', '-X'] and inp['argv'][3].startswith('pycache_prefix=')
    assert not inp['absentSources']
    assert {e['path']:e['sha256'] for e in inp['sourceBytes']} == {e['path']:e['sha256'] for e in result['sourceAfter']}
    assert inp['environmentBefore'] == result['environmentAfter']
    assert inp['pythonBefore'] == result['pythonAfter']
    assert inp['pythonEnvironmentManifestSha256'] == sha(ENV.read_bytes())
    assert inp['pythonEnvironmentArchiveSha256'] == env['archiveSha256']
    assert sha((directory/'source.tar.gz').read_bytes()) == inp['sourceArchiveSha256']
    sources[name] = {}
    with tarfile.open(directory/'source.tar.gz') as tar:
        members = {m.name:m for m in tar.getmembers() if m.isfile()}
        assert len(members) == len(inp['sourceBytes']) == 42
        for e in inp['sourceBytes']:
            raw = tar.extractfile(members[e['archivePath']]).read()
            assert len(raw) == e['bytes'] and sha(raw) == e['sha256']
            sources[name][e['path']] = raw
            if name.startswith('final'): assert sha(Path(e['path']).read_bytes()) == e['sha256']
    for stream in ('stdout', 'stderr'):
        assert sha((directory/(stream+'.txt')).read_bytes()) == result[stream+'Sha256']
    stages[name] = {k:result[k] for k in ('exitCode', 'elapsedSeconds', 'stdoutSha256', 'stderrSha256')}
    stages[name]['sourceFiles'] = len(inp['sourceBytes'])
    stages[name]['inputSha256'] = sha((directory/'input.json').read_bytes())
    stages[name]['terminalSha256'] = sha((directory/'terminal.json').read_bytes())
module = str(Path('scripts/check_s02_candidate_a_integrated.py').resolve())
tests = str(Path('tests/test_s02_candidate_a_integrated.py').resolve())
guard = b"        require(exported_raw == raw_field, 'raw provenance field '+name)\n"
assert guard in sources['green-syntax'][module] and guard not in sources['red-tests'][module]
assert {p for p in sources['red-tests'] if sources['red-tests'][p] != sources['green-syntax'][p]} == {module}
assert sources['red-tests'][tests] == sources['green-syntax'][tests]
assert {p for p in sources['metadata-red'] if sources['metadata-red'][p] != sources['final-tests'][p]} == {module}
assert 'DID NOT RAISE Invalid' in (BASE/'red-tests/stdout.txt').read_text()
assert '1 failed, 76 passed' in (BASE/'green-tests/stdout.txt').read_text()
assert 'input pin linkage' in (BASE/'green-tests/stdout.txt').read_text()
assert '2 failed' in (BASE/'metadata-red/stdout.txt').read_text()
assert '79 passed' in (BASE/'final-tests/stdout.txt').read_text()
assert sha(Path(module).read_bytes()) == 'ce055ac1a611f6eecc74248da64962a5c899ebbf282d41f3ca243dde51365c66'
assert sha(Path(tests).read_bytes()) == 'f6e547364e3d7707237654bb135229a414857ac64a414aaaa2178ce06957db26'
print(json.dumps({'status':'accepted-local-sharded-checker-unit-only', 'tests':79,
    'checkerTests':37, 'utilityTests':42, 'sourceFiles':42,
    'sharedRuntimeFiles':len(env['sourceBytes']), 'stages':stages,
    'scope':'focused synthetic shard controls; no actual package acceptance or Task6'}, indent=2))
