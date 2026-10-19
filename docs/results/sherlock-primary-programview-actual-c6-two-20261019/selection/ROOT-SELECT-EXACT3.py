from pathlib import Path
import datetime, hashlib, importlib.util, json, os, stat

R = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017')
P = R / 'primary-programview-to-ngprogram-port-20261018'
Q = R / 'primary-programview-actual-c6-capture-port-20261019'
E = Q / 'exact-three-prefrozen-oracle-20261019'

def load(p):
    return json.loads(Path(p).read_text())

def pin(p):
    p = Path(p)
    b = p.read_bytes()
    return {'path': str(p), 'bytes': len(b), 'sha256': hashlib.sha256(b).hexdigest()}

def put(p, x):
    p = Path(p)
    with p.open('xb') as f:
        f.write((json.dumps(x, indent=2) + '\n').encode())
        f.flush()
        os.fsync(f.fileno())
    fd = os.open(p.parent, os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)

def sync(p):
    with Path(p).open('rb') as f:
        os.fsync(f.fileno())

op = load(E / 'PROPOSED-OPERATIONS.json')
manifest = load(E / 'COMPARISON-MANIFEST.json')
old = load(P / 'STATIC-PINS.json')['pins']
new = load(E / 'STATIC-PINS.json')['pins']

def check():
    for d in old + new:
        assert pin(d['path']) == d, d['path']
    assert str(Path('/usr/bin/python3').resolve()) == '/usr/bin/python3.14'
    assert sorted(x.name for x in (P / 'source').iterdir()) == ['compare_interface_ports.py', 'program_view_to_ngprogram.py']
    assert pin(E / 'SOURCE-OP-FREEZE.json') == source_freeze_pin
    assert pin(E / 'ROOT-OPERATION-RECIPE.md') == recipe_pin

# Root binds the complete descriptors below before any child; they are also
# retained in ROOT-SELECTION so this executable glue cannot hide a source change.
source_freeze_pin = pin(E / 'SOURCE-OP-FREEZE.json')
recipe_pin = pin(E / 'ROOT-OPERATION-RECIPE.md')
assert source_freeze_pin['bytes'] == 2663 and source_freeze_pin['sha256'] == 'a2e28a9ce7ab1f5f472e46749055538d186fe972b888470806bc79783715c8f6'
assert recipe_pin['bytes'] == 3126 and recipe_pin['sha256'] == '41057b82b74137975c690488e1777d0bde1f2b338b92576b43f567d74fb10a07'
check()
spec = importlib.util.spec_from_file_location('root_existing_f03_launch', op['callback']['fullsource']['path'])
f03 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(f03)
check()
A = Q / 'actual'
A.mkdir(mode=0o700)
for phase in op['phases']:
    cwd = Path(phase['cwd'])
    cwd.mkdir(mode=0o700)
    (cwd / 'home').mkdir(mode=0o700)
    (cwd / 'tmp').mkdir(mode=0o700)

put(A / 'ROOT-SELECTION.json', {'review': pin(E / 'actual-fresh-opus-01/FINAL-RESPONSE.txt'), 'source_freeze': source_freeze_pin, 'recipe': recipe_pin, 'glue': pin(__file__), 'expectedBodiesOpenedByRoot': False, 'operations': pin(E / 'PROPOSED-OPERATIONS.json')})
results = []
actual_pins = []
for i, phase in enumerate(op['phases']):
    check()
    cwd = Path(phase['cwd'])
    assert stat.S_IMODE(A.stat().st_mode) == 0o700
    assert sorted(x.name for x in cwd.iterdir()) == ['home', 'tmp']
    for d in [cwd, cwd / 'home', cwd / 'tmp']:
        assert stat.S_IMODE(d.stat().st_mode) == 0o700
    assert not list((cwd / 'home').iterdir()) and not list((cwd / 'tmp').iterdir())
    assert set(phase['environment']) == {'HOME', 'TMPDIR', 'LANG', 'LC_ALL', 'SHERLOCK_K_ATTEMPT_TOKEN'}
    for product in phase['products']:
        assert not Path(product).exists()
    put(cwd / 'ROOT-SELECTION.json', phase)
    record = {'id': phase['id'], 'argv': phase['argv'], 'cwd': phase['cwd'], 'environment': phase['environment']}
    try:
        f03.launch(record, phase['argv'], dict(phase['environment']), phase['cwd'], phase['environment']['SHERLOCK_K_ATTEMPT_TOKEN'], 536870912)
    finally:
        put(cwd / 'RESULT.json', record)
    streams = [pin(cwd / 'stdout'), pin(cwd / 'stderr')]
    for d in streams:
        sync(d['path'])
    tree = [{'relative': str(p.relative_to(cwd)), 'kind': 'directory' if p.is_dir() else 'file', 'mode': stat.S_IMODE(p.stat().st_mode)} for p in sorted(cwd.rglob('*'))]
    put(cwd / 'ROOT-POSTCHILD-TREE.json', tree)
    assert record['outcome'] == 'exited' and not record['timed_out'] and not record['containment_breach'] and record['remaining'] == []
    if i < 2:
        assert record['returncode'] == 0
        actual = pin(phase['products'][0])
        sync(actual['path'])
        carrier = load(actual['path'])
        expected_input = manifest['cases'][i]['input']
        assert carrier['status'] == 'ported' and carrier['unhandled'] == [] and carrier['partial_counterpart'] is True
        assert set(carrier['ng_program']) == {'declarations', 'expressions', 'functions', 'entry'}
        assert carrier['program_pointer'] == '/canonicalProgramView'
        assert carrier['source_file'] == {'bytes': expected_input['bytes'], 'sha256': expected_input['sha256']}
        lines = (cwd / 'stdout').read_bytes().splitlines()
        assert len(lines) == 1
        status = json.loads(lines[0])
        assert status['output_bytes'] == actual['bytes'] and status['output_sha256'] == actual['sha256'] and status['unhandled_count'] == 0
        assert streams[1]['bytes'] == 0
        actual_pins.append(actual)
    else:
        comparison_path = Path(manifest['comparison_output'])
        if not comparison_path.exists():
            raise AssertionError('fail-closed comparison abort; raw receipts retained')
        comparison = load(comparison_path)
        assert (record['returncode'], comparison['status']) in [(0, 'PASS'), (1, 'FAIL')]
        consumer = load(manifest['actual_freeze_output'])
        assert [x['actual'] for x in consumer['actuals']] == actual_pins
        assert [x['input'] for x in consumer['actuals']] == [x['input'] for x in manifest['cases']]
        assert [x['case_id'] for x in consumer['actuals']] == [x['case_id'] for x in manifest['cases']]
        assert comparison['actual_freeze'] == pin(manifest['actual_freeze_output'])
        assert [x['expected_pin'] for x in comparison['results']] == [x['expected'] for x in manifest['cases']]
        assert [pin(x['path']) for x in actual_pins] == actual_pins
    post = {'streams': streams, 'products': [pin(p) for p in phase['products']], 'all53StaticPinsCurrent': True}
    check()
    put(cwd / 'ROOT-POSTCHECK.json', post)
    results.append({'id': phase['id'], 'exit': record['returncode'], 'products': post['products']})
    if i == 1:
        frozen = {'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'actuals': actual_pins, 'receipts': [pin(Path(p['cwd']) / 'RESULT.json') for p in op['phases'][:2]], 'streamsAndTrees': [pin(Path(p['cwd']) / name) for p in op['phases'][:2] for name in ['stdout', 'stderr', 'ROOT-POSTCHILD-TREE.json', 'ROOT-POSTCHECK.json']], 'expectedBodiesOpenedByRoot': False, 'source_freeze': source_freeze_pin}
        put(A / 'ROOT-ACTUALS-AND-RECEIPTS-FROZEN-BEFORE-COMPARISON.json', frozen)
result = {'status': comparison['status'], 'allThreeObserved': True, 'cases': results, 'provenance': op['scope_qualification'], 'historicalConsumerScopeLiteralPreserved': True, 'rootOpenedExpectedBodies': False, 'scope': 'Two actual C6 compile captures; complete NGProgram typed-equals independently prefrozen expectations. Global ArmRow IDs and SCC-min groups; no generic-instance coverage or K/runtime/physical-B inference.'}
put(A / 'ROOT-RESULT.json', result)
print(json.dumps(result))
