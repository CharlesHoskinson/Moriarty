"""Intake the stopped Task3 originals: four passes, one original frontend failure."""
import ast
import hashlib
import json
from pathlib import Path
import re
import sys
import tarfile

ROOT = Path.cwd()
R = ROOT / '.superpowers/sdd/a5-factoring-receipts'
MODEL = ROOT / 'specs/quint/s02/factored_verification'
HELPER = ROOT / 'scripts/run_s02_candidate_a_factoring_pilot.py'
BASE = '9ccbf0ed571e4055bc05962e5681fdf2fa75ad96'
BOOT = '900bb2051225b4a3d99bf422c3b2e5e386e3e7bc'
QUINT = '/home/charl/.npm-global/bin/quint'
def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()
def read(path): return json.loads(Path(path).read_bytes())
def archive_check(path, expected):
    with tarfile.open(path) as archive:
        members = archive.getmembers()
        assert len(members) == len(expected)
        assert {m.name for m in members} == set(expected)
        for member in members:
            assert member.isfile()
            assert hashlib.file_digest(archive.extractfile(member), 'sha256').hexdigest() == expected[member.name]
            assert sha(ROOT / member.name) == expected[member.name]
    return len(expected)

helper_tree = ast.parse(HELPER.read_text())
witnesses = next(ast.literal_eval(node.value) for node in helper_tree.body
                 if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id == 'WITNESSES' for t in node.targets))
assert len(witnesses) == 8
deriver = ast.parse((ROOT / 'scripts/derive_s02_candidate_a_factored.py').read_text())
originals = next(ast.literal_eval(node.value) for node in deriver.body
                if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id == 'PINNED_ORIGINALS' for t in node.targets))
sources = {f'specs/quint/s02/{name}.qnt' for name in originals}
sources.update(f'specs/quint/s02/{name}.qnt' for name in ('effects','consumption','observations','policies','authorization','execution'))
frozen = read(R / 'task2-frozen-source.json')
sources.update(e['path'] for e in frozen['files'])
pilot_names = ['candidate_a_funding_pilot.qnt', 'candidate_a_funding_pilot_f.qnt', 'candidate_a_funding_pilot_test.qnt']
sources.update(str((MODEL / name).relative_to(ROOT)) for name in pilot_names)
sources.update(('scripts/run_s02_candidate_a_factoring_pilot.py','scripts/derive_s02_candidate_a_factored.py'))
assert len(sources) == 69
bindings = ['run-task3.py','task3-dispatch.json','task3-test-first.json','task2-frozen-source.json']
expected_binding = {str((R/name).relative_to(ROOT)): sha(R/name) for name in bindings}
assert expected_binding[str((R/'run-task3.py').relative_to(ROOT))] == 'beacf086d818651fbfeb04051678d8771f1511932832cd2c239dc2318858360d'
stages = ['task3-pilot-typecheck','task3-pilot-prefix-test','sample-original-pilotSafety',
          'sample-factored-pilotSafety','compile-original-pilotSafety','compile-factored-pilotSafety']
results = {}
for name in stages:
    stage, binding = R / name, R / 'task3-command-bindings' / name
    if not (binding / 'outer-terminal.json').exists(): continue
    inp, result = read(stage / 'input.json'), read(stage / 'result.json')
    before, after, outer = (read(binding / x) for x in ('before.json','after.json','outer-terminal.json'))
    expected_exit = int(name == 'compile-original-pilotSafety')
    assert result['exitCode'] == outer['exitCode'] == after['exitCode'] == expected_exit
    assert not result['timedOut'] and result['sourceAndToolsUnchanged'] and result['runtimeUnchanged']
    assert outer['postchecksPassed'] is (expected_exit == 0)
    assert after['sourceDispatchUnchanged'] and after['exception'] is None
    assert before['actualTask3Base'] == after['actualTask3Base'] == BASE
    assert before['runtimeBootstrapBase'] == after['runtimeBootstrapBase'] == inp['beforeDispatchCommit'] == BOOT
    assert inp['sourceCommit'] == BASE and inp['cwd'] == str(ROOT) and inp['depth'] == 5
    assert before['stage'] == name and before['parentBytecodeDisabled']
    assert before['parentArgv'][1:4] == ['-B','-X','pycache_prefix='+before['parentCachePrefix']]
    assert before['parentArgv'][4:] == ['.superpowers/sdd/a5-factoring-receipts/run-task3.py',before['mode']]
    assert after['originalInputSha256'] == sha(stage/'input.json')
    assert after['originalResultSha256'] == sha(stage/'result.json')
    assert after['sourceAndDispatchArchiveSha256'] == sha(binding/'source-and-dispatch.tar.gz')
    assert {str(Path(p['path']).relative_to(ROOT)):p['sha256'] for p in before['pins']} == expected_binding
    archive_check(binding/'source-and-dispatch.tar.gz', {**expected_binding,
        str((binding/'before.json').relative_to(ROOT)):sha(binding/'before.json')})
    joined = ''.join(chunk['output'] for chunk in outer['chunks'])
    assert joined == (binding/'outer-tool-output.txt').read_text()
    assert outer['chunks'][-1]['exitCode'] == expected_exit and outer['chunks'][-1]['chunkId'] == outer['terminalChunk']
    assert read(stage/'result.json') == {k:v for k,v in json.loads(joined.splitlines()[0]).items() if k!='stage'}
    if expected_exit:
        assert joined.splitlines()[-1] == "AssertionError: ('compile-original-pilotSafety', 1)"
        assert '"task3Stage"' not in joined
    else:
        assert json.loads(joined.splitlines()[-1]) == {'task3Stage':name,'binding':str(binding),'terminal':0}
    pins = {p['path']:p['sha256'] for p in inp['pins']}
    assert len(pins) == len(inp['pins'])
    local = {str(Path(p).relative_to(ROOT)):digest for p,digest in pins.items() if Path(p).is_relative_to(ROOT)}
    expected_sources = sources | (set(expected_binding) if name.startswith('task3-') else set())
    assert set(local) == expected_sources
    assert sha(stage/'source-and-runner.tar.gz') == inp['sourceArchiveSha256']
    count = archive_check(stage/'source-and-runner.tar.gz',local)
    for path,digest in pins.items(): assert sha(path) == digest
    output = stage / ('input.qnt.json' if name.startswith('compile-') else 'stdout.txt')
    assert sha(output) == result['stdoutSha256'] and sha(stage/'stderr.txt') == result['stderrSha256']
    if not expected_exit: assert (stage/'stderr.txt').stat().st_size == 0
    assert re.search(r'Exit status: '+str(expected_exit)+r'\b',(stage/'resources.txt').read_text())
    wall = 900 if name.startswith('compile-') else 1200
    assert inp['limits'] == {'wallSeconds':wall,'jvmHeapMiB':4096,'nodeHeapMiB':4096}
    assert 0 < result['durationSeconds'] < wall
    test = str(MODEL/'candidate_a_funding_pilot_test.qnt')
    if name == 'task3-pilot-typecheck': expected = [QUINT,'typecheck',test]
    elif name == 'task3-pilot-prefix-test':
        expected = [QUINT,'test',test,'--backend=rust','--seed=42','--match','allPilotPrefixesTest']
        assert re.findall(r'ok (\w+) passed \d+ test\(s\)',output.read_text()) == ['allPilotPrefixesTest']
    else:
        mode,variant,_ = name.split('-')
        stem = 'candidate_a_funding_pilot' + ('_f' if variant=='factored' else '')
        entry = str(MODEL/(stem+'.qnt'))
        if mode == 'sample':
            expected = [QUINT,'run',entry,'--backend=rust','--seed=42','--max-samples=100',
                        '--max-steps=5','--invariant=pilotSafety','--witnesses',*witnesses,'--verbosity=1']
            rows = re.findall(r'(\w+) was witnessed in (\d+) trace\(s\) out of (\d+) explored',output.read_text())
            assert [r[0] for r in rows] == list(witnesses) and all(0<int(r[1])<=100 and r[2]=='100' for r in rows)
        else:
            expected = [QUINT,'compile',entry,'--main='+stem,'--target=json','--invariant=pilotSafety','--verbosity=0']
            assert expected_exit == 1 and output.stat().st_size == 0
            assert 'generated' not in result and 'measurementError' not in result
            errors = (stage/'stderr.txt').read_text()
            assert errors.count("Error [QNT404]: Type alias 'AuthorityKey' not found") == 7
            assert errors.endswith('error: name resolution failed\n')
            assert sha(stage/'stderr.txt') == '606a4bf274f5499111b50b3897cc06064a2c953d3ef9fdbcf57e567baccba842'
    assert inp['requestedArgv'] == expected and inp['argv'][2:] == expected[1:]
    if name.startswith('task3-'): assert before['dispatchArgv'] == expected
    else: assert before['dispatchArgv'] == [str(HELPER),mode,variant,'pilotSafety',BOOT]
    results[name] = {'exitCode':expected_exit,'sourceMembers':count,'durationSeconds':result['durationSeconds'],
                     'resultSha256':sha(stage/'result.json'),'bindingAfterSha256':sha(binding/'after.json'),
                     'generated':result.get('generated')}
assert list(results) == stages[:-1]
assert not (R/stages[-1]).exists()
for name in ('check-original-pilotSafety','check-factored-pilotSafety','compile-factored-neverPrepared','check-factored-neverPrepared'):
    assert not (R/name).exists()
index_path = R/'task3-compile-gate-index.json'
assert sha(index_path) == '7683111834647f3677a96921f71573b9952b763497c94a8b18d6e0b9c5e7df15'
assert sha(ROOT/'.superpowers/sdd/a5-task3-compile-gate-report.md') == '8f300f8b2653c5ec8ab03482e76befb59e6443ae2dfe874f87b45bd7456bd9f1'
index = read(index_path)
assert index['actualTask3Base'] == BASE and index['runtimeBootstrapBase'] == BOOT
assert [e['stage'] for e in index['stages']] == list(results)
assert index['unchangedTask2Sources'] == frozen
indexed_files = list(index['attachments']) + index['pilotSources']
for entry in index['stages']:
    name = entry['stage']
    assert entry['result'] == read(R/name/'result.json')
    assert entry['outerTerminal'] == read(R/'task3-command-bindings'/name/'outer-terminal.json')
    assert entry['expectedExitCode'] == results[name]['exitCode']
    assert entry['sourceArchiveMembers'] == results[name]['sourceMembers'] and entry['bindingArchiveMembers'] == 5
    resources = (R/name/'resources.txt').read_text()
    assert int(re.search(r'Maximum resident set size \(kbytes\): (\d+)',resources)[1]) == entry['maximumRssKiB']
    indexed_files.extend(entry['files'] + entry['bindingFiles'])
for entry in indexed_files + [index['diagnosticInstalledCliSource']]:
    path = ROOT/entry['path']
    assert path.is_file() and not path.is_symlink()
    assert sha(path) == entry['sha256'] and path.stat().st_size == entry['bytes']
assert sum(e['sourceMembers'] for e in results.values()) == 353
print(json.dumps({'ok':True,'scope':'Stopped original experiment intake: four passing finite checks and one compiler failure; no size baseline or checker verdict',
                  'terminalStages':len(results),'remainingStages':[s for s in stages if s not in results],
                  'nativeSourceMembers':353,'dispatchSourceMembers':25,'indexedFilesChecked':len(indexed_files)+1,
                  'stages':results},indent=2))
