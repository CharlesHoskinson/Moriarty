"""Root original-byte intake of the bounded Task3 exporter source unit."""
import ast
import hashlib
import json
from pathlib import Path
import re

ROOT = Path.cwd()
BASE = ROOT/'.superpowers/sdd/a4-producer-receipts'
def sha(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as stream:
        while block:=stream.read(1048576): h.update(block)
    return h.hexdigest()
def read(path): return json.loads(path.read_bytes())
index_path=ROOT/'.superpowers/sdd/a4-producer-task3-source-artifact-index.json'
report=ROOT/'.superpowers/sdd/a4-producer-task3-source-report.md'
assert sha(index_path)=='9e3474865559172c3be480d5be9877ba3e2d395e591316f8a46194a6a86e1ebc'
assert sha(report)=='6854816bc21d867d6fb774bfda03e8cb34e5e6393ee6c5f9101865c8df1f74ad'
index=read(index_path)
assert len(index['files'])==515 and len(index['symlinks'])==13
for name,pin in index['files'].items():
    p=ROOT/name
    assert p.resolve().is_relative_to(ROOT) and not p.is_symlink()
    assert p.stat().st_size==pin['bytes'] and sha(p)==pin['sha256']
for name,target in index['symlinks'].items():
    p=ROOT/name
    assert p.is_symlink() and str(p.readlink())==target
    assert p.resolve().is_relative_to(BASE/'task3-unit-green/basetemp')

plan=ROOT/'docs/superpowers/plans/2026-09-05-candidate-a-sharded-exporter.md'
assert sha(plan)=='61c7571cd97f3fc211818a7016cb3414acb8a6da508495d69ff2c2283c8e6501'
blocks=re.findall(r'```python\n(.*?)```',plan.read_text(),re.S)
assert len(blocks)==8
old=ast.parse(blocks[2])
obsolete=next(n for n in old.body if isinstance(n,ast.FunctionDef) and n.name=='verify_pins')
lines=blocks[2].splitlines(True)
assert lines[obsolete.end_lineno]=='\n'  # Remove its following separator too.
foundation=''.join(lines[:obsolete.lineno-1]+lines[obsolete.end_lineno+1:])
expected=foundation+'\n'+blocks[3]+'\n'+blocks[4]
exporter=ROOT/'scripts/export_s02_candidate_a_integrated.py'
test=ROOT/'tests/test_s02_candidate_a_integrated_export.py'
assert exporter.read_text()==expected
definitions=[n.name for n in ast.parse(expected).body if isinstance(n,(ast.ClassDef,ast.FunctionDef))]
assert len(definitions)==len(set(definitions))==37
for i in (1,5,6,7): assert test.read_text().count(blocks[i])==1
actual_names=[n.name for n in ast.parse(blocks[7]).body if isinstance(n,ast.FunctionDef)]
assert len(actual_names)==4 and all(n.startswith('test_actual_') for n in actual_names)
assert sha(exporter)=='51028d697cd9be91ebfa97b5d7d6f69127ca700fd47313648cb4138877dddcd5'
assert sha(test)=='3e99e7069d28ec661ac6c45f64809505bfcd10c18cd202c477545d5fbf5ed33f'

stages={}; members=0; runtime=None
for name,code in [('task3-inventory-red',1),('task3-unit-green',0)]:
    s=BASE/name; r=read(s/'receipt.json')
    assert r['exit_code']==r['recorder_exit_code']==code
    assert r['sourceCommit']==r['sourceCommitAfter']=='b08a2da7180015ef017a6b89926f2cd5a5276ef6'
    assert r['beforeDispatchCommit']=='386bf0ae10f767e051b414a7231be105cc4b0f71'
    assert r['cwd']==str(ROOT) and r['source_stable'] and r['runtime_stable']
    assert r['sources_before']==r['sources_after'] and len(r['sources_before'])==121
    assert r['not_yet_created_before']==r['not_yet_created_after']==[]
    assert r['runtime_before']==r['runtime_after']
    assert r['shared_runtime_before']==r['shared_runtime_after']
    assert r['runtime_archive_sha256']=='f0687656d30c0d59ece8dfd027508a9228020506d29b97aea3abe7568186c31c'
    assert r['runtime_manifest_sha256']=='fa3e9838c66480557ed6d928a2d159ebc074d05f67f15ef08ab20388d9c49e0b'
    assert r['shared_runtime_before']['beforeDispatchCommit']=='900bb2051225b4a3d99bf422c3b2e5e386e3e7bc'
    if runtime is None: runtime=r['runtime_before']
    assert runtime==r['runtime_before']
    assert r['elapsed_ns']==r['ended_ns']-r['started_ns']>0
    assert not r['python_bytecode_writes'] and r['python_cache_prefix']==str(s/'python-cache')
    assert sha(s/'runtime-helper.py')=='816c3ad79dc3a67ca9a03729af56d74188c161a7546b9c43e821c222a4bce7f2'
    argv=['/home/charl/Moriarty/.venv/bin/python','-m','pytest',
          'tests/test_s02_candidate_a_integrated_export.py','-q','-k']
    argv+=['dropped_whole_case or reordered_profiles'] if code else ['not actual',
        '--basetemp=.superpowers/sdd/a4-producer-receipts/task3-unit-green/basetemp']
    assert r['command']==r['executed_command']==argv
    for when in ('before','after'):
        closure=read(s/when/'closure.json')
        assert closure=={'sources':r['sources_'+when],'not_yet_created':[]}
        actual={str(p.relative_to(s/when/'source')) for p in (s/when/'source').rglob('*') if p.is_file()}
        assert actual==set(closure['sources'])
        for path,digest in closure['sources'].items():
            assert sha(s/when/'source'/path)==digest
            members+=1
            if not code: assert sha(ROOT/path)==digest
    for stream in ('stdout','stderr'): assert sha(s/(stream+'.bin'))==r[stream+'_sha256']
    assert (s/'stderr.bin').stat().st_size==0 and r['artifacts']=={}
    stages[name]={'exitCode':code,'elapsedNanoseconds':r['elapsed_ns'],'receiptSha256':sha(s/'receipt.json')}
red=read(BASE/'task3-inventory-red/receipt.json'); green=read(BASE/'task3-unit-green/receipt.json')
assert {p for p in red['sources_before'] if red['sources_before'][p]!=green['sources_before'][p]}=={
    'scripts/export_s02_candidate_a_integrated.py','tests/test_s02_candidate_a_integrated_export.py'}
assert (BASE/'task3-inventory-red/before/source/scripts/export_s02_candidate_a_integrated.py').read_text()==blocks[0]
red_tests=(BASE/'task3-inventory-red/before/source/tests/test_s02_candidate_a_integrated_export.py').read_text()
assert blocks[1] in red_tests and test.read_text().startswith(red_tests)
failure=(BASE/'task3-inventory-red/stdout.bin').read_text()
assert failure.count('Failed: DID NOT RAISE ExportError')==2 and '2 failed, 15 deselected' in failure
assert '55 passed, 4 deselected' in (BASE/'task3-unit-green/stdout.bin').read_text()
parser_results=[]
for terminal in sorted((BASE/'task3-unit-green/basetemp').glob('test_parser_original_failure_s[0-9]/parser/terminal.json')):
    r=read(terminal); folder=terminal.parent
    assert r['tools_before']==r['tools_after'] and r['subprocess_attempted'] is True
    assert r['timeout_seconds']==900 and r['node_heap_mib']==4096
    assert r['argv'][1]=='--max-old-space-size=4096'
    for stream in ('stdout','stderr'):
        assert sha(folder/(stream+'.bin'))==r[stream+'_sha256']
        assert (folder/(stream+'.bin')).stat().st_size==r[stream+'_bytes']
    parser_results.append((r['exit_code'],r['failure']))
assert parser_results==[(7,None),(0,None),(None,'timeout')]
for path,digest in runtime['files'].items(): assert sha(path)==digest
for directory,names in runtime['treeMembers'].items():
    assert sorted({str(p.resolve()) for p in Path(directory).rglob('*') if p.is_file()})==names
print(json.dumps({'ok':True,'scope':'Task3-source-unit-only-not-native-package',
    'stages':stages,'originalSourceMembersAudited':members,'originalFiles':515,'originalSymlinks':13,
    'runtimePins':len(runtime['files']),'plannedDefinitions':37,'nonActualPythonTests':55,
    'mandatoryActualTestsUnexecuted':actual_names,'syntheticParserReceipts':3,
    'exporterSha256':sha(exporter),'testSha256':sha(test)},indent=2))
