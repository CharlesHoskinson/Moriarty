"""Read-only root admission of original streaming reader receipts."""
import hashlib
import json
from pathlib import Path
import tarfile

BASE = Path('.superpowers/sdd/a4-stream-reader-task1-receipts')
ENV = Path('.superpowers/sdd/a4-checker-task1-receipts/python-environment.json')
def sha(raw): return hashlib.sha256(raw).hexdigest()
def read(path): return json.loads(path.read_bytes())
assert sha(ENV.read_bytes()) == 'cd004057c4067bf7cc536d2cec5038add88d0852c1a7de5030e402ff2204f9c4'
env = read(ENV)
assert sha((ENV.parent/'python-environment.tar.gz').read_bytes()) == env['archiveSha256']
for e in env['sourceBytes']: assert sha(Path(e['path']).read_bytes()) == e['sha256']
stages, sources = {}, {}
for name, code in {'tests-first':2,'red-import':0,'red-tests':1,
                   'green-syntax':0,'green-import':0,'green-tests':0}.items():
    directory = BASE/name
    inp, result = read(directory/'input.json'), read(directory/'terminal.json')
    assert result['exitCode'] == code
    assert not result['sourceChanged'] and not result['environmentChanged']
    assert inp['baseCommit'] == '22cdec6e561b4def3bd7c4a9bd723e2ad824d800'
    assert inp['argv'][1:3] == ['-B','-X'] and 'pycache_prefix=' in inp['argv'][3]
    assert {e['path']:e['sha256'] for e in inp['sourceBytes']} == {e['path']:e['sha256'] for e in result['sourceAfter']}
    assert inp['environmentBefore'] == result['environmentAfter']
    assert inp['pythonEnvironmentManifestSha256'] == sha(ENV.read_bytes())
    assert inp['pythonEnvironmentArchiveSha256'] == env['archiveSha256']
    assert sha((directory/'source.tar.gz').read_bytes()) == inp['sourceArchiveSha256']
    sources[name] = {}
    with tarfile.open(directory/'source.tar.gz') as tar:
        members = {m.name:m for m in tar.getmembers() if m.isfile()}
        assert len(members) == len(inp['sourceBytes'])
        for e in inp['sourceBytes']:
            raw = tar.extractfile(members[e['archivePath']]).read()
            assert len(raw) == e['bytes'] and sha(raw) == e['sha256']
            sources[name][e['path']] = raw
            if name.startswith('green'): assert sha(Path(e['path']).read_bytes()) == e['sha256']
    for stream in ('stdout','stderr'):
        assert sha((directory/(stream+'.txt')).read_bytes()) == result[stream+'Sha256']
    stages[name] = {k:result[k] for k in ('exitCode','elapsedSeconds','stdoutSha256','stderrSha256')}
    stages[name]['sourceFiles'] = len(inp['sourceBytes'])
    stages[name]['inputSha256'] = sha((directory/'input.json').read_bytes())
    stages[name]['terminalSha256'] = sha((directory/'terminal.json').read_bytes())
module = str(Path('scripts/a4_json_stream.py').resolve())
assert {p for p in sources['red-tests'] if sources['red-tests'][p] != sources['green-tests'][p]} == {module}
assert sources['green-tests'][module].replace(b"        if key in out: raise JsonStreamError('duplicate object key')\n",b'',1) == sources['red-tests'][module]
assert 'DID NOT RAISE' in (BASE/'red-tests/stdout.txt').read_text()
assert '29 passed' in (BASE/'green-tests/stdout.txt').read_text()
print(json.dumps({'status':'accepted-local-stream-reader-task1-only','tests':29,
    'sourceFiles':7,'sharedRuntimeFiles':len(env['sourceBytes']), 'stages':stages,
    'scope':'strict JSON transport only; no writer, authority or actual package acceptance'},indent=2))
