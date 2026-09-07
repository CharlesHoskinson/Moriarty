"""Root read-only audit of Task0/Task1 original producer receipts."""
import hashlib
import json
from pathlib import Path

ROOT = Path.cwd()
BASE = ROOT/'.superpowers/sdd/a4-producer-receipts'
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def read(path): return json.loads(path.read_bytes())
expected = {'recorder-smoke':0,'inventory-red':1,'inventory-green':0,
    'observer-scaffold-types':0,'observer-red':1,'observer-types':1,'observer-green':1,
    'observer-types-2':0,'observer-green-2':0}
stages, shared_runtime = {}, None
for name, code in expected.items():
    stage = BASE/name
    r = read(stage/'receipt.json')
    assert r['exit_code'] == r['recorder_exit_code'] == code
    assert r['source_stable'] and r['runtime_stable']
    assert r['beforeDispatchCommit'] == '386bf0ae10f767e051b414a7231be105cc4b0f71'
    assert r['sources_before'] == r['sources_after']
    assert r['not_yet_created_before'] == r['not_yet_created_after']
    assert r['runtime_before'] == r['runtime_after']
    assert r['shared_runtime_before'] == r['shared_runtime_after']
    if shared_runtime is None: shared_runtime = r['runtime_before']
    assert shared_runtime == r['runtime_before']
    assert r['elapsed_ns'] == r['ended_ns'] - r['started_ns'] and r['elapsed_ns'] > 0
    assert r['artifacts'] == {} and not r['python_bytecode_writes']
    assert sha(stage/'runtime-helper.py') == '816c3ad79dc3a67ca9a03729af56d74188c161a7546b9c43e821c222a4bce7f2'
    for when in ('before','after'):
        closure = read(stage/when/'closure.json')
        assert closure['sources'] == r['sources_'+when]
        assert closure['not_yet_created'] == r['not_yet_created_'+when]
        for path, pin in closure['sources'].items():
            assert sha(stage/when/'source'/path) == pin
    for stream in ('stdout','stderr'):
        assert sha(stage/(stream+'.bin')) == r[stream+'_sha256']
    stages[name] = {k:r[k] for k in ('exit_code','elapsed_ns','sourceCommit','sourceCommitAfter','stdout_sha256','stderr_sha256')}
    stages[name]['sourceFiles'] = len(r['sources_before'])
    stages[name]['receiptSha256'] = sha(stage/'receipt.json')
for path, pin in shared_runtime['files'].items(): assert sha(Path(path)) == pin
for root, names in shared_runtime['treeMembers'].items():
    assert sorted({str(p.resolve()) for p in Path(root).rglob('*') if p.is_file()}) == names
green = read(BASE/'observer-green-2/receipt.json')
for path, pin in green['sources_before'].items(): assert sha(ROOT/path) == pin
assert '2 failed' in (BASE/'inventory-red/stdout.bin').read_text()
assert '2 passed' in (BASE/'inventory-green/stdout.bin').read_text()
assert 'QNT508' in (BASE/'observer-red/stdout.bin').read_text()
assert 'actualDepositObservationTest' in (BASE/'observer-red/stdout.bin').read_text()
assert 'QNT404' in (BASE/'observer-types/stderr.bin').read_text()
assert (BASE/'observer-types/stderr.bin').read_bytes() == (BASE/'observer-green/stderr.bin').read_bytes()
assert '4 passing' in (BASE/'observer-green-2/stdout.bin').read_text()
old = (BASE/'observer-types/before/source/specs/quint/s02/candidate_a_integrated_observer.qnt').read_text()
new = (ROOT/'specs/quint/s02/candidate_a_integrated_observer.qnt').read_text()
assert old.replace('ExtractionObservedA4(EffectsExtractedA(_)) => true',
    'ExtractionObservedA4(extraction) => match extraction {\n          | EffectsExtractedA(_) => true | _ => false }') == new
print(json.dumps({'status':'accepted-local-producer-task1-only','inventoryTests':2,
    'observerTests':4,'runtimePins':len(shared_runtime['files']),'sourceFiles':len(green['sources_before']),
    'scope':'typed computation observer and fixed inventory; no case lowering, native exports or A4 acceptance',
    'stages':stages},indent=2))
