"""Root selection of reviewed ordinary operations; existing capture functions only."""
from pathlib import Path
import datetime
import hashlib
import json
import os
import re
import runpy

D = Path(__file__).resolve().parent
M = D / 'revision02-metadata-corrections'
A = D / 'actual-nineteen-01'

def load(path):
    return json.loads(Path(path).read_text())

def pin(path):
    path = Path(path)
    raw = path.read_bytes()
    return {'path': str(path), 'bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest()}

def put(path, value):
    with Path(path).open('xb') as stream:
        stream.write((json.dumps(value, indent=2) + '\n').encode())
        stream.flush()
        os.fsync(stream.fileno())

def match(row):
    assert pin(row['path']) == {k: row[k] for k in ['path', 'bytes', 'sha256']}, row['path']

proposal = M / 'afterimages/PROPOSED.json'
assert pin(proposal)['sha256'] == '1194c5c53f64e10a6320398857cd572b5ec62ac169908b58587123c5033cff00'
p = load(proposal)
assert len(p['operations']) == 19
for key in ['sourceFreeze', 'sourceClosures', 'flatStaticInputs']:
    match(p[key])
assert p['sourceClosures']['sha256'] == '37c34c7bba041981e9b9b963b96d31e97a499b5a05f0dcf77130f0b6e9bb3776'
conditions = load(M / 'ROOT-CAPTURE-SEMANTICS.json')
helper = conditions['captureHelper']
assert helper['sha256'] == 'ce0482fdc12e4146500fa4421f7e4333b14321bdaa207c123a09ee6cb34c9932'
match(helper)
h = runpy.run_path(helper['path'], run_name='root_existing_functions_only')
flat = load(p['flatStaticInputs']['path'])['files']
assert len(flat) == 621
closures = load(p['sourceClosures']['path'])
review = M / 'fresh-operations-only-review/actual-fresh-muse-outage-01/FINAL-RESPONSE.txt'
assert review.is_file() and '### Verdict: `APPROVE`' in review.read_text()
source_review = D.parent / 'fresh-nonauthor-review/actual-fresh-gemini-outage-01/FINAL-RESPONSE.txt'
assert pin(source_review)['sha256'] == '778d63ed1b0ba582652eb6f634bb8ff41b7c1b1bea92183a7f45a5fc3d828aae'
generated = {}
static = {}
for row in flat:
    key = str(Path(row['path']).resolve(strict=True))
    if key in static:
        assert (static[key]['bytes'], static[key]['sha256']) == (row['bytes'], row['sha256'])
    static[key] = row

def check():
    assert h['static_check'](flat)['all_match'], 'static input changed'
    for row in generated.values():
        match(row)

check()
assert not A.exists()
for name in ['build', 'tmp']:
    path = D / name
    assert not path.is_symlink() and (not path.exists() or (path.is_dir() and not any(path.iterdir())))
for op in p['operations']:
    assert all(not Path(path).exists() for path in op['outputs'])
for name in ['build', 'tmp']:
    (D / name).mkdir(mode=0o700, exist_ok=True)
A.mkdir(mode=0o700)
put(A / 'ROOT-SELECTION.json', {
    'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'glue': pin(__file__), 'proposal': pin(proposal), 'helper': helper,
    'sourceReview': pin(source_review), 'operationsReview': pin(review),
    'exactOperations': [op['id'] for op in p['operations']],
    'conditions': p['RootControls'], 'noPrivateExpectedReads': True,
    'oneAttemptNoRetry': True,
})
records = []
failure = None
full_names = None
try:
    for index, op in enumerate(p['operations']):
        check()
        allowed = dict(static)
        allowed.update({str(Path(v['path']).resolve(strict=True)): v for v in generated.values()})
        for path in op.get('requiredInputs', []):
            key = str(Path(path).resolve(strict=True))
            assert key in allowed, 'required input not static or previously generated: ' + path
            row = allowed[key]
            actual = pin(path)
            assert (actual['bytes'], actual['sha256']) == (row['bytes'], row['sha256'])
        if index >= 13:
            binary = str(D / 'build/02-Core-cfgtest')
            assert binary in generated and binary in op['argv']
        assert all(not Path(path).exists() for path in op['outputs'])
        phase = A / op['id']
        phase.mkdir(mode=0o700)
        receipt = h['capture'](op, phase)
        put(phase / 'RESULT.json', receipt)
        for stream in receipt['streams']:
            with Path(stream['path']).open('rb') as handle:
                os.fsync(handle.fileno())
        records.append(receipt)
        assert receipt['actual_exit'] == 0 and receipt['controller_failure'] is None, 'child failure; no retry'
        assert len(receipt['streams']) == 2 and all(row['full_stream_retained'] for row in receipt['streams'])
        details = {}
        for path in op['outputs']:
            assert Path(path).is_file(), 'missing output: ' + path
            generated[path] = pin(path)
        if index < 13:
            expected = closures
            for key in op['expectedDepMembership'].split('.'):
                expected = expected[key]
            depfile = next(path for path in op['outputs'] if path.endswith('.d'))
            deps = h['dep_inputs'](depfile, allowed)
            actual_set = {str(Path(row['path']).resolve(strict=True)) for row in deps}
            expected_set = {str(Path(row['path']).resolve(strict=True)) for row in expected['files']}
            assert len(expected['files']) == expected['count']
            assert actual_set == expected_set and len(actual_set) == expected['count'], 'exact dep-info set mismatch'
            details['exactDepInfo'] = {'count': len(actual_set), 'closure': op['expectedDepMembership'], 'files': deps}
        if '--list' in op['argv']:
            lines = (phase / 'stdout.bin').read_text().splitlines()
            assert all(line.endswith(': test') for line in lines if line.strip()), 'unexpected list line or benchmark'
            names = [line[:-6] for line in lines if line.strip()]
            assert names and len(names) == len(set(names)), 'empty or duplicate test list'
            if index == 13:
                assert set(op['expectation']['requiredSubset']) <= set(names)
                full_names = names
                put(A / 'FROZEN-FULL-CORE-TEST-LIST.json', {'names': names, 'count': len(names), 'stdout': pin(phase / 'stdout.bin')})
            else:
                e = op['expectation']
                assert len(names) == e['count'] == 4 and set(names) == set(e['exactNames'])
            details['testList'] = names
        if '--exact' in op['argv']:
            assert full_names is not None
            e = op['expectation']
            text = (phase / 'stdout.bin').read_text()
            assert re.search(r'(?m)^running 1 test\s*$', text)
            assert re.findall(r'(?m)^test (\S+) \.\.\. (\S+)$', text) == [(e['exactTest'], 'ok')]
            summary = re.search(r'(?m)^test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out;', text)
            assert summary and tuple(map(int, summary.groups())) == (1, 0, 0, 0, len(full_names) - 1)
            details['exactTest'] = {'name': e['exactTest'], 'passed': 1, 'failed': 0, 'ignored': 0, 'measured': 0, 'filtered_out': len(full_names) - 1}
        check()
        put(phase / 'ROOT-POSTCHECK.json', details)
        print(json.dumps({'id': op['id'], 'exit': receipt['actual_exit'], 'elapsed': receipt['elapsed_seconds'], 'postcheck': 'PASS'}), flush=True)
except BaseException as error:
    failure = type(error).__name__ + ': ' + str(error)
    print(failure, flush=True)
finally:
    existing = [pin(path) for op in p['operations'] for path in op['outputs'] if Path(path).is_file()]
    put(A / 'ROOT-RESULT.json', {
        'status': 'PASS' if failure is None and len(records) == 19 else 'STOPPED',
        'failure': failure, 'actualOperations': len(records), 'requiredOperations': 19,
        'records': records, 'createdOutputs': existing,
        'laterOperationsUNRUN': [op['id'] for op in p['operations'][len(records):]],
        'glue': pin(__file__), 'proposal': pin(proposal), 'noPrivateExpectedReads': True,
        'wholeCoreTestSuiteRun': False, 'externalRuntimeRuns': False, 'nativeOrTarget': False, 'noRetry': True,
    })
if failure:
    raise SystemExit(1)
