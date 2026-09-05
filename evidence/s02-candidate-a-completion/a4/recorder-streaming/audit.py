"""Root intake of the original RH001 RED/GREEN receipts; run from repo root."""
import hashlib
import json
from pathlib import Path
import re

ROOT = Path.cwd()
BASE = ROOT / '.superpowers/sdd/a4-producer-receipts'

def sha(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        while block := stream.read(1048576): h.update(block)
    return h.hexdigest()

def read(path): return json.loads(path.read_bytes())

index_path = ROOT / '.superpowers/sdd/a4-recorder-streaming-artifact-index.json'
report = ROOT / '.superpowers/sdd/a4-recorder-streaming-report.md'
assert sha(index_path) == '518173653bc363927f7c3380c87409ad6c273645143eff8feee8d0391f5bcef1'
assert sha(report) == '852e704b63f83b2cadd0fb850d50d941ff6d7da9b9dfcc6d8d504596471488d4'
index = read(index_path)
assert len(index['files']) == 492
for name, pin in index['files'].items():
    path = ROOT / name
    assert path.resolve().is_relative_to(ROOT) and not path.is_symlink()
    assert path.stat().st_size == pin['bytes'] and sha(path) == pin['sha256']

stages = {}
members = 0
runtime = None
for name, code in [('recorder-stream-red', 1), ('recorder-stream-green', 0)]:
    stage = BASE / name
    r = read(stage / 'receipt.json')
    assert r['exit_code'] == r['recorder_exit_code'] == code
    assert r['cwd'] == str(ROOT)
    assert r['sourceCommit'] == r['sourceCommitAfter'] == '8be3cef9a3be94d7dd0b54f9393dded1c5437883'
    assert r['beforeDispatchCommit'] == '386bf0ae10f767e051b414a7231be105cc4b0f71'
    assert r['source_stable'] and r['runtime_stable']
    assert r['sources_before'] == r['sources_after'] and len(r['sources_before']) == 120
    assert r['not_yet_created_before'] == r['not_yet_created_after']
    assert r['runtime_before'] == r['runtime_after']
    assert r['shared_runtime_before'] == r['shared_runtime_after']
    assert r['runtime_archive_sha256'] == 'f0687656d30c0d59ece8dfd027508a9228020506d29b97aea3abe7568186c31c'
    assert r['runtime_manifest_sha256'] == 'fa3e9838c66480557ed6d928a2d159ebc074d05f67f15ef08ab20388d9c49e0b'
    assert r['shared_runtime_before']['beforeDispatchCommit'] == '900bb2051225b4a3d99bf422c3b2e5e386e3e7bc'
    if runtime is None: runtime = r['runtime_before']
    assert runtime == r['runtime_before']
    assert r['elapsed_ns'] == r['ended_ns'] - r['started_ns'] > 0
    assert not r['python_bytecode_writes']
    assert r['python_cache_prefix'] == str(stage / 'python-cache')
    assert sha(stage / 'runtime-helper.py') == '816c3ad79dc3a67ca9a03729af56d74188c161a7546b9c43e821c222a4bce7f2'
    command = ['/home/charl/Moriarty/.venv/bin/python', '-m', 'pytest',
               'tests/test_s02_candidate_a_integrated_export.py', '-vv']
    if code: command += ['-k', 'recorder_artifact_hash']
    assert r['command'] == command
    for when in ('before', 'after'):
        closure = read(stage / when / 'closure.json')
        assert closure == {'sources': r['sources_' + when],
                           'not_yet_created': r['not_yet_created_' + when]}
        actual = {str(p.relative_to(stage / when / 'source'))
                  for p in (stage / when / 'source').rglob('*') if p.is_file()}
        assert actual == set(closure['sources'])
        for path, digest in closure['sources'].items():
            assert sha(stage / when / 'source' / path) == digest
            members += 1
            if name.endswith('green'): assert sha(ROOT / path) == digest
    for stream in ('stdout', 'stderr'):
        assert sha(stage / (stream + '.bin')) == r[stream + '_sha256']
    assert (stage / 'stderr.bin').stat().st_size == 0
    assert r['artifacts'] == {} and not list(stage.glob('*.itf.json'))
    stages[name] = {'exitCode': code, 'elapsedNanoseconds': r['elapsed_ns'],
                    'sourceFiles': 120, 'receiptSha256': sha(stage / 'receipt.json')}

red = read(BASE / 'recorder-stream-red/receipt.json')
green = read(BASE / 'recorder-stream-green/receipt.json')
recorder = 'scripts/record_s02_candidate_a_integrated.py'
test = 'tests/test_s02_candidate_a_integrated_export.py'
assert {p for p in red['sources_before'] if red['sources_before'][p] != green['sources_before'][p]} == {recorder}
original = (BASE / 'recorder-stream-red/before/source' / recorder).read_text()
corrected = original.replace('ROOT = Path(__file__).resolve().parents[1]\n',
    'ROOT = Path(__file__).resolve().parents[1]\nif __package__ in (None, ""):\n'
    '    sys.path.insert(0, str(ROOT))\nfrom scripts.a4_json_stream import hash_stream\n\n', 1)
corrected = corrected.replace('def artifact_hash(path):\n    return sha(path.read_bytes())',
    'def artifact_hash(path):\n    with path.open("rb") as stream:\n        return hash_stream(stream)[0]', 1)
assert corrected == (ROOT / recorder).read_text()
assert sha(ROOT / recorder) == 'dc30b764e2603dfe3c39ef6e2063a7082d116dc9d3eb9a012072f468beb8f8e9'
assert sha(ROOT / test) == 'e8cf739c2de53ce6dfe0a992acf25acf048e16221c69ebdc3998d41621d3b74c'
failure = (BASE / 'recorder-stream-red/stdout.bin').read_text()
assert 'AssertionError: whole native artifact read_bytes' in failure
assert '1 failed, 14 deselected' in failure
passing = (BASE / 'recorder-stream-green/stdout.bin').read_text()
nodes = re.findall(r'(?m)^(tests/\S+) PASSED ', passing)
assert len(nodes) == len(set(nodes)) == 15
assert '15 passed' in passing and 'deselected' not in passing
for path, digest in runtime['files'].items(): assert sha(Path(path)) == digest
for directory, names in runtime['treeMembers'].items():
    assert sorted({str(p.resolve()) for p in Path(directory).rglob('*') if p.is_file()}) == names
print(json.dumps({'ok': True, 'scope': 'local-RH001-only-no-native-export',
    'stages': stages, 'originalSourceMembersAudited': members,
    'originalArtifactFiles': len(index['files']), 'runtimePins': len(runtime['files']),
    'pythonTests': len(nodes), 'orderedPassingNodeIds': nodes,
    'recorderSha256': sha(ROOT / recorder), 'testSha256': sha(ROOT / test)}, indent=2))
