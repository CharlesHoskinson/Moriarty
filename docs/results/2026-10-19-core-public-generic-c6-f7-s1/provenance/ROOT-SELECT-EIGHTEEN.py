from pathlib import Path
import datetime, hashlib, json, os, re, runpy

D = Path(__file__).resolve().parent
R = D.parent.parent

def load(p):
    return json.loads(Path(p).read_text())

def pin(p):
    p = Path(p)
    b = p.read_bytes()
    return {'path': str(p), 'bytes': len(b), 'sha256': hashlib.sha256(b).hexdigest()}

def put(p, v):
    p = Path(p)
    with p.open('xb') as f:
        f.write((json.dumps(v, indent=2) + '\n').encode())
        f.flush()
        os.fsync(f.fileno())

proposal = D / 'PROPOSED.json'
assert pin(proposal)['sha256'] == '00e343860280f41e74e61789e8e53a9ce4c013eaf9b765e722a71aa23d9ded08'
p = load(proposal)
helper = Path(p['existingCapture']['existing_file']['path'])
assert pin(helper) == p['existingCapture']['existing_file']
h = runpy.run_path(str(helper), run_name='root_existing_capture_functions_only')
flat = load(p['flatInputs']['path'])['files']
assert pin(p['flatInputs']['path']) == p['flatInputs'] and len(flat) == 718
membership = load(p['depMembership']['path'])
assert pin(p['depMembership']['path']) == p['depMembership']
generated = []

def check():
    assert h['static_check'](flat)['all_match']
    for d in generated:
        assert pin(d['path']) == d

check()
A = Path(p['existingCapture']['fresh_actual_dir'])
assert not A.exists()
for name in ['build', 'tmp']:
    d = D / name
    assert not d.is_symlink() and (not d.exists() or (d.is_dir() and not list(d.iterdir())))
    d.mkdir(mode=0o700, exist_ok=True)
for op in p['operations']:
    assert all(not Path(v).exists() for v in op['outputs'])
capture_parent = D / 'actual-capture-output-01'
assert not capture_parent.is_symlink() and (not capture_parent.exists() or (capture_parent.is_dir() and not list(capture_parent.iterdir())))
capture_parent.mkdir(mode=0o700, exist_ok=True)
A.mkdir(mode=0o700)
put(A / 'ROOT-SELECTION.json', {'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'proposal': pin(proposal), 'glue': pin(__file__), 'helper': pin(helper), 'review': pin(D.parent / 'fresh-affected-nonauthor-review/actual-fresh-opus-01/FINAL-RESPONSE.txt'), 'sourceAndOperationsApprovedByFreshReview': True, 'conditions': p['RootCompositionRequiredConditionsOC1toOC7'], 'noPrivateExpectedReads': True})
allowed = {str(Path(d['path']).resolve(strict=True)): d for d in flat}
compiles = {0: 'Core', 1: 'publicConsumer', 5: 'affected035Consumer', 12: 'newOwnerRegressions'}
records = []
failure = None
try:
    for index, op in enumerate(p['operations']):
        check()
        assert all(not Path(v).exists() for v in op['outputs'])
        if index == 3:
            assert not list(capture_parent.iterdir())
        phase = A / op['id']
        phase.mkdir(mode=0o700)
        receipt = h['capture'](op, phase)
        put(phase / 'RESULT.json', receipt)
        # Receipt and full streams are durable before any acceptance checks.
        for stream in receipt['streams']:
            with Path(stream['path']).open('rb') as f:
                os.fsync(f.fileno())
        records.append(receipt)
        assert receipt['actual_exit'] == 0 and receipt['controller_failure'] is None, 'child failed; no retry'
        assert len(receipt['streams']) == 2 and all(s['full_stream_retained'] for s in receipt['streams'])
        details = {}
        for path in op['outputs']:
            assert Path(path).is_file(), 'declared output missing: ' + path
            generated.append(pin(path))
        if index in compiles:
            expected = membership[compiles[index]]
            depfile = next(v for v in op['outputs'] if v.endswith('.d'))
            deps = h['dep_inputs'](depfile, allowed)
            actual_set = {str(Path(d['path']).resolve(strict=True)) for d in deps}
            expected_set = {str(Path(d['path']).resolve(strict=True)) for d in expected['files']}
            assert len(expected['files']) == expected['count']
            assert actual_set == expected_set and len(actual_set) == expected['count'], 'exact dep-info membership mismatch'
            details['exactDepInfo'] = {'count': len(actual_set), 'expected': compiles[index], 'files': deps}
        if '--list' in op['argv']:
            lines = (phase / 'stdout.bin').read_text().splitlines()
            assert all(line.endswith(': test') for line in lines if line.strip()), 'unexpected benchmark/list line'
            names = [line[:-6] for line in lines if line.strip()]
            e = op.get('expectedMembership', op.get('expected'))
            exact = e['exactSet'] if 'exactSet' in e else e['exactUniqueTestNames']
            count = e['count'] if 'count' in e else e['testCount']
            assert len(names) == len(set(names)) == count and set(names) == set(exact)
            details['exactTestList'] = names
        if '--exact' in op['argv']:
            e = op['expectedMembership']
            text = (phase / 'stdout.bin').read_text()
            assert re.search(r'(?m)^running 1 test\s*$', text)
            assert re.findall(r'(?m)^test (\S+) \.\.\. (\S+)$', text) == [(e['exactTest'], 'ok')]
            m = re.search(r'(?m)^test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out;', text)
            assert m and tuple(map(int, m.groups())) == tuple(e[k] for k in ['passed', 'failed', 'ignored', 'measured', 'filtered_out'])
            details['exactTestEnvelope'] = e
        check()
        put(phase / 'ROOT-POSTCHECK.json', details)
        print(json.dumps({'id': op['id'], 'exit': receipt['actual_exit'], 'elapsed': receipt['elapsed_seconds'], 'postcheck': 'PASS'}), flush=True)
        if index == 4:
            ten = [Path(v) for caseop in p['operations'][3:5] for v in caseop['outputs']]
            assert len(ten) == 10 and all(v.is_file() for v in ten)
            assert {v for v in capture_parent.rglob('*') if v.is_file()} == set(ten)
            for v in ten:
                with v.open('rb') as f:
                    os.fsync(f.fileno())
            put(A / 'ROOT-COMPLETE-ACTUAL-CAPTURES-FROZEN-BEFORE-INDEPENDENT-COMPARISON.json', {'files': [pin(v) for v in ten], 'newUnionActual': True, 'expectedBodiesOpened': False, 'proposal': pin(proposal)})
except BaseException as error:
    failure = type(error).__name__ + ': ' + str(error)
    print(failure, flush=True)
finally:
    existing = [pin(v) for op in p['operations'] for v in op['outputs'] if Path(v).is_file()]
    result = {'status': 'PASS' if failure is None and len(records) == 18 else 'STOPPED', 'failure': failure, 'actualOperations': len(records), 'all18Required': 18, 'records': records, 'createdOutputs': existing, 'laterOperationsUNRUN': [op['id'] for op in p['operations'][len(records):]], 'source': p['source'], 'noPrivateExpectedReads': True, 'glue': pin(__file__), 'noRetry': True}
    put(A / 'ROOT-RESULT.json', result)
if failure:
    raise SystemExit(1)
